//! WebGPU (WGSL + Wasm host bridge) GPU compute backend for Cinch's
//! data-parallel arrays.
//!
//! Designed primarily for `wasm32` targets running in browsers, Deno, Node.js,
//! or Wasm runtimes that expose the WebGPU API (`navigator.gpu` / `GPUDevice`).
//!
//! - On `wasm32`, the runtime imports the `cinch_webgpu` host module provided
//!   by Cinch's emitted `<out>.webgpu.mjs` bridge (zero Rust dependencies).
//!   GPU-resident buffer allocations, uploads (`queue.writeBuffer`), and
//!   compute dispatches (`queue.submit`) execute synchronously across the Wasm
//!   boundary, while readback operations integrate cleanly with WebAssembly
//!   JSPI (`WebAssembly.Suspending`) or async host orchestration.
//! - On native hosts (`not(target_arch = "wasm32")`), `WebGpuState::open`
//!   probes for native WebGPU C libraries (`libwgpu_native` / `libwebgpu_dawn`)
//!   via [`crate::sys::dlopen`] and cleanly reports unavailability when no
//!   native WebGPU C driver is present so `DEVICE_KIND_WEBGPU` falls back to
//!   multicore CPU and `DEVICE_KIND_WEBGPU_REQUIRED` surfaces
//!   `DEV_ERR_NO_DEVICE` at the defined error boundary.
//! - [`BUILTIN_WGSL_SOURCE`] and [`BUILTIN_WASM_WEBGPU_JS_BRIDGE`] are always
//!   available on every target for inspection, embedding, and AOT Wasm
//!   emission.

use crate::metal::DeviceTelemetry;
#[cfg(target_arch = "wasm32")]
use crate::metal::{DeviceLaunchPlan, SyncPoint};

// ---------------------------------------------------------------------------
// Built-in Production WebGPU WGSL Compute Shaders
// ---------------------------------------------------------------------------

/// Complete, spec-compliant WebGPU Shading Language (WGSL 1.0) compute shader
/// library implementing all 7 resident Cinch data-parallel device operations.
///
/// Notes on WGSL semantics:
/// - Core WGSL has no `f64` type; all floating-point shaders operate on `f32`
///   and the Cinch compiler diagnoses `f64` array ops at compile time under
///   `--array-target=webgpu-required` / `wasm-webgpu-required`.
/// - Core WGSL has no 8-bit storage buffer element type; `u8` image batches
///   are packed 4 pixels per `u32` word (little-endian) and written race-free
///   via `atomicOr` into zero-cleared `array<atomic<u32>>` output buffers.
pub const BUILTIN_WGSL_SOURCE: &str = include_str!("webgpu_builtin.wgsl");

// ---------------------------------------------------------------------------
// Built-in ES Module Wasm + WebGPU JS Host Bridge
// ---------------------------------------------------------------------------

/// Self-contained ES module implementing the `cinch_webgpu` Wasm import module
/// on top of standard browser / Deno / Node.js WebGPU (`navigator.gpu`).
///
/// Emitted automatically alongside `<out>.wasm` when compiling freestanding
/// Wasm with `--array-target=webgpu` or `--array-target=webgpu-required`.
pub const BUILTIN_WASM_WEBGPU_JS_BRIDGE: &str = include_str!("webgpu_bridge.mjs");

/// Materialize the standalone ES module JS bridge with the full WGSL source
/// embedded directly into `CINCH_WGSL_SOURCE`.
pub fn format_wasm_webgpu_js_bridge(extra_wgsl: Option<&str>) -> String {
    let mut wgsl = BUILTIN_WGSL_SOURCE.to_string();
    if let Some(extra) = extra_wgsl {
        if !extra.trim().is_empty() {
            wgsl.push_str("\n// --- Program-specific specialized WGSL kernels ---\n");
            wgsl.push_str(extra);
        }
    }
    let escaped = wgsl.replace('\\', "\\\\").replace('`', "\\`").replace("${", "\\${");
    BUILTIN_WASM_WEBGPU_JS_BRIDGE.replace("__CINCH_WGSL_PLACEHOLDER__", &escaped)
}

// ---------------------------------------------------------------------------
// Wasm32 Host FFI (`cinch_webgpu` import module, declared in `crate::sys::wasm`)
// ---------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
use crate::sys::wasm::webgpu_host::*;

// ---------------------------------------------------------------------------
// WebGpuBuffer & WebGpuState
// ---------------------------------------------------------------------------

pub struct WebGpuBuffer {
    pub ctx_handle: i32,
    pub handle: i32,
    pub bytes: usize,
}

unsafe impl Send for WebGpuBuffer {}
unsafe impl Sync for WebGpuBuffer {}

impl Drop for WebGpuBuffer {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            if self.handle > 0 {
                unsafe {
                    cinch_webgpu_free(self.ctx_handle, self.handle);
                }
            }
        }
    }
}

pub struct WebGpuState {
    pub device_name: String,
    pub ctx_handle: i32,
}

unsafe impl Send for WebGpuState {}
unsafe impl Sync for WebGpuState {}

impl Drop for WebGpuState {
    fn drop(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            if self.ctx_handle > 0 {
                unsafe {
                    cinch_webgpu_close(self.ctx_handle);
                }
            }
        }
    }
}

pub fn is_available() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe { cinch_webgpu_available() != 0 }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

impl WebGpuState {
    pub fn open() -> Result<Self, String> {
        #[cfg(target_arch = "wasm32")]
        {
            if unsafe { cinch_webgpu_available() } == 0 {
                return Err("WebGPU host bridge reported navigator.gpu unavailable".to_string());
            }
            let h = unsafe { cinch_webgpu_open() };
            if h <= 0 {
                return Err(format!("cinch_webgpu_open failed ({h})"));
            }
            Ok(Self {
                device_name: "WebGPU (Wasm host bridge)".to_string(),
                ctx_handle: h,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Probe for optional native wgpu-native / Dawn shared libraries;
            // native execution is not wired yet without the Wasm host bridge.
            let _has_native_lib = [
                b"libwgpu_native.dylib".as_slice(),
                b"libwgpu_native.so",
                b"libwebgpu_dawn.dylib",
                b"libwebgpu_dawn.so",
            ]
            .iter()
            .any(|lib| !crate::sys::dlopen(lib).is_null());
            Err("WebGPU runtime executes via the Wasm `cinch_webgpu` host bridge (browser/Deno/Node WebGPU); use Metal/OpenCL/CUDA/HIP on native targets or compile to Wasm with `--array-target=webgpu`".to_string())
        }
    }

    pub fn alloc_buf(&self, bytes: usize) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let h = unsafe { cinch_webgpu_alloc(self.ctx_handle, bytes as u32) };
            if h <= 0 {
                return Err(format!("cinch_webgpu_alloc({bytes}) failed ({h})"));
            }
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle: h,
                bytes,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = bytes;
            Err("WebGPU buffer allocation requires wasm32 host bridge".to_string())
        }
    }

    pub fn upload(&self, src: *const u8, bytes: usize) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let h = unsafe { cinch_webgpu_upload(self.ctx_handle, src, bytes as u32) };
            if h <= 0 {
                return Err(format!("cinch_webgpu_upload({bytes}) failed ({h})"));
            }
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle: h,
                bytes,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (src, bytes);
            Err("WebGPU upload requires wasm32 host bridge".to_string())
        }
    }

    pub fn download(&self, buf: &WebGpuBuffer, dst: *mut u8, bytes: usize) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            if bytes > 0 && !dst.is_null() {
                let rc = unsafe { cinch_webgpu_download(self.ctx_handle, buf.handle, dst, bytes as u32) };
                if rc != 0 {
                    return Err(format!("cinch_webgpu_download failed ({rc})"));
                }
            }
            Ok(())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (buf, dst, bytes);
            Err("WebGPU download requires wasm32 host bridge".to_string())
        }
    }

    pub fn dot_f32(
        &mut self,
        a_buf: &WebGpuBuffer,
        b_buf: &WebGpuBuffer,
        n: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<f32, String> {
        if n == 0 {
            return Ok(0.0);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let mut out = 0.0f32;
            let rc = unsafe {
                cinch_webgpu_dot_f32(
                    self.ctx_handle,
                    a_buf.handle,
                    b_buf.handle,
                    n as u32,
                    i32::from(exact),
                    &mut out,
                )
            };
            if rc != 0 {
                return Err(format!("cinch_webgpu_dot_f32 failed ({rc})"));
            }
            let groups = if exact {
                (n + 1023) / 1024
            } else {
                ((n + 255) / 256).clamp(1, 1024)
            };
            telemetry.kernel_launches += 1;
            telemetry.sync_points.push(SyncPoint {
                op: if exact {
                    "dot-f32-exact-tree".to_string()
                } else {
                    "dot-f32-relaxed".to_string()
                },
                reason: "webgpu-map-async-readback".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: if exact {
                    "dot-f32-exact-tree".to_string()
                } else {
                    "dot-f32-relaxed".to_string()
                },
                schedule: if exact {
                    format!("webgpu-wgsl-exact-tree-1024(workgroups={groups})")
                } else {
                    format!("webgpu-wgsl-workgroup-reduction(workgroups={groups})")
                },
                grid: [groups * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: if exact { 1024 * 4 } else { 256 * 4 },
                numerical_mode: if exact {
                    "f32-add-exact-tree".to_string()
                } else {
                    "f32-add-relaxed".to_string()
                },
            };
            Ok(out)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (a_buf, b_buf, exact, telemetry);
            Err("WebGPU dot_f32 requires wasm32 host bridge".to_string())
        }
    }

    pub fn scan_i32(
        &mut self,
        in_buf: &WebGpuBuffer,
        n: usize,
        inclusive: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let h = unsafe {
                cinch_webgpu_scan_i32(self.ctx_handle, in_buf.handle, n as u32, i32::from(inclusive))
            };
            if h <= 0 {
                return Err(format!("cinch_webgpu_scan_i32 failed ({h})"));
            }
            let nblocks = (n + 255) / 256;
            telemetry.note_alloc(n * 4);
            telemetry.kernel_launches += if nblocks > 1 { 2 } else { 1 };
            telemetry.last_plan = DeviceLaunchPlan {
                op: if inclusive { "scan-i32-inclusive" } else { "scan-i32-exclusive" }.to_string(),
                schedule: format!("webgpu-wgsl-hillis-steele(workgroups={nblocks})"),
                grid: [nblocks.max(1) * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 256 * 4,
                numerical_mode: "i32-wrapping-add".to_string(),
            };
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle: h,
                bytes: n * 4,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (in_buf, n, inclusive, telemetry);
            Err("WebGPU scan_i32 requires wasm32 host bridge".to_string())
        }
    }

    pub fn filter_i32_gt(
        &mut self,
        in_buf: &WebGpuBuffer,
        n: usize,
        threshold: i32,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<(WebGpuBuffer, usize), String> {
        #[cfg(target_arch = "wasm32")]
        {
            let mut out_len = 0u32;
            let h = unsafe {
                cinch_webgpu_filter_i32_gt(
                    self.ctx_handle,
                    in_buf.handle,
                    n as u32,
                    threshold,
                    &mut out_len,
                )
            };
            if h <= 0 {
                return Err(format!("cinch_webgpu_filter_i32_gt failed ({h})"));
            }
            let k = out_len as usize;
            telemetry.note_alloc(k * 4);
            telemetry.kernel_launches += 3;
            telemetry.last_plan = DeviceLaunchPlan {
                op: "filter-i32-gt".to_string(),
                schedule: format!("webgpu-wgsl-flag-scan-scatter(n={n},k={k})"),
                grid: [((n + 255) / 256).max(1) * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 256 * 4,
                numerical_mode: "stable-compaction".to_string(),
            };
            Ok((
                WebGpuBuffer {
                    ctx_handle: self.ctx_handle,
                    handle: h,
                    bytes: k * 4,
                },
                k,
            ))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (in_buf, n, threshold, telemetry);
            Err("WebGPU filter_i32_gt requires wasm32 host bridge".to_string())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stencil_histogram_u8(
        &mut self,
        in_buf: &WebGpuBuffer,
        f: usize,
        h: usize,
        w: usize,
        ws: [i32; 9],
        divisor: i32,
        edge_policy: u32,
        nsteps: usize,
        bins: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let handle = unsafe {
                cinch_webgpu_stencil_hist_u8(
                    self.ctx_handle,
                    in_buf.handle,
                    f as u32,
                    h as u32,
                    w as u32,
                    ws.as_ptr(),
                    divisor,
                    edge_policy,
                    nsteps as u32,
                    bins as u32,
                )
            };
            if handle <= 0 {
                return Err(format!("cinch_webgpu_stencil_hist_u8 failed ({handle})"));
            }
            let frame_pixels = h * w;
            let many_small = f >= 16 || frame_pixels <= 16_384;
            let tiles_per_frame = if many_small {
                1usize
            } else {
                (frame_pixels / (256 * 32)).clamp(1, 256)
            };
            let hist_groups = f * tiles_per_frame;
            telemetry.note_alloc(f * bins * 4);
            telemetry.kernel_launches += nsteps + 1;
            telemetry.last_plan = DeviceLaunchPlan {
                op: format!("stencil({nsteps})-then-histogram-u8"),
                schedule: if many_small {
                    format!("many-small-frames(webgpu,frames={f},h={h},w={w},hist_groups={hist_groups})")
                } else {
                    format!("few-large-frames(webgpu,frames={f},h={h},w={w},tiles_per_frame={tiles_per_frame})")
                },
                grid: [hist_groups.max(1) * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 256 * 4,
                numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
            };
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle,
                bytes: f * bins * 4,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (in_buf, f, h, w, ws, divisor, edge_policy, nsteps, bins, telemetry);
            Err("WebGPU stencil_histogram_u8 requires wasm32 host bridge".to_string())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stencil2d_f32(
        &mut self,
        in_buf: &WebGpuBuffer,
        f: usize,
        h: usize,
        w: usize,
        ws: [f32; 9],
        edge_policy: u32,
        nsteps: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let handle = unsafe {
                cinch_webgpu_stencil2d_f32(
                    self.ctx_handle,
                    in_buf.handle,
                    f as u32,
                    h as u32,
                    w as u32,
                    ws.as_ptr(),
                    edge_policy,
                    nsteps as u32,
                )
            };
            if handle <= 0 {
                return Err(format!("cinch_webgpu_stencil2d_f32 failed ({handle})"));
            }
            let total = f * h * w;
            telemetry.note_alloc(total * 4);
            telemetry.kernel_launches += nsteps;
            telemetry.last_plan = DeviceLaunchPlan {
                op: format!("stencil2d-f32({nsteps})"),
                schedule: format!("webgpu-wgsl-2d-ping-pong(frames={f},h={h},w={w})"),
                grid: [((w + 15) / 16) * 16, ((h + 15) / 16) * 16, f.max(1)],
                threadgroup: [16, 16, 1],
                threadgroup_mem_bytes: 0,
                numerical_mode: "f32-stencil-2d".to_string(),
            };
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle,
                bytes: total * 4,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (in_buf, f, h, w, ws, edge_policy, nsteps, telemetry);
            Err("WebGPU stencil2d_f32 requires wasm32 host bridge".to_string())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spmv_csr_f32(
        &mut self,
        vals: &WebGpuBuffer,
        cols: &WebGpuBuffer,
        row_ptr: &WebGpuBuffer,
        x: &WebGpuBuffer,
        r: usize,
        nz: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let handle = unsafe {
                cinch_webgpu_spmv_csr_f32(
                    self.ctx_handle,
                    vals.handle,
                    cols.handle,
                    row_ptr.handle,
                    x.handle,
                    r as u32,
                    nz as u32,
                    i32::from(exact),
                )
            };
            if handle <= 0 {
                return Err(format!("cinch_webgpu_spmv_csr_f32 failed ({handle})"));
            }
            let groups = (r + 255) / 256;
            telemetry.note_alloc(r * 4);
            telemetry.kernel_launches += 1;
            telemetry.last_plan = DeviceLaunchPlan {
                op: "spmv-csr-f32".to_string(),
                schedule: format!("webgpu-wgsl-csr-scalar(rows={r},nz={nz})"),
                grid: [groups.max(1) * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 0,
                numerical_mode: if exact {
                    "f32-add-exact-tree".to_string()
                } else {
                    "f32-add-relaxed".to_string()
                },
            };
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle,
                bytes: r * 4,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (vals, cols, row_ptr, x, r, nz, exact, telemetry);
            Err("WebGPU spmv_csr_f32 requires wasm32 host bridge".to_string())
        }
    }

    pub fn row_reduce_f32(
        &mut self,
        in_buf: &WebGpuBuffer,
        r: usize,
        c: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<WebGpuBuffer, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let handle = unsafe {
                cinch_webgpu_row_reduce_f32(self.ctx_handle, in_buf.handle, r as u32, c as u32)
            };
            if handle <= 0 {
                return Err(format!("cinch_webgpu_row_reduce_f32 failed ({handle})"));
            }
            let outer = r >= 128 && c < 2048;
            let groups = if outer { (r + 255) / 256 } else { r };
            telemetry.note_alloc(r * 4);
            telemetry.kernel_launches += 1;
            telemetry.last_plan = DeviceLaunchPlan {
                op: "row-reduce-f32".to_string(),
                schedule: if outer {
                    format!("outer-parallel-rows(webgpu,rows={r},cols={c})")
                } else {
                    format!("inner-parallel-workgroup(webgpu,rows={r},cols={c})")
                },
                grid: [groups.max(1) * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: if outer { 0 } else { 256 * 4 },
                numerical_mode: "f32-add-relaxed".to_string(),
            };
            Ok(WebGpuBuffer {
                ctx_handle: self.ctx_handle,
                handle,
                bytes: r * 4,
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (in_buf, r, c, telemetry);
            Err("WebGPU row_reduce_f32 requires wasm32 host bridge".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wgsl_shaders_cover_all_compute_entrypoints() {
        for entry in [
            "fn dot_f32_relaxed",
            "fn dot_f32_exact_block",
            "fn scan_i32_block",
            "fn scan_i32_add_offsets",
            "fn filter_i32_gt_flags",
            "fn filter_i32_gt_scatter",
            "fn stencil2d_u8_step",
            "fn hist_u8_many_small_frames",
            "fn hist_u8_few_large_frames",
            "fn stencil2d_f32_step",
            "fn spmv_csr_f32_scalar",
            "fn row_reduce_f32_outer",
            "fn row_reduce_f32_inner",
        ] {
            assert!(
                BUILTIN_WGSL_SOURCE.contains(entry),
                "missing WGSL entry point {entry}"
            );
        }
        assert!(BUILTIN_WGSL_SOURCE.contains("@compute @workgroup_size(256, 1, 1)"));
        assert!(BUILTIN_WGSL_SOURCE.contains("@compute @workgroup_size(16, 16, 1)"));
        assert!(BUILTIN_WGSL_SOURCE.contains("workgroupBarrier()"));
        assert!(BUILTIN_WGSL_SOURCE.contains("atomicAdd("));
        assert!(BUILTIN_WGSL_SOURCE.contains("atomicOr("));
        // Core WGSL must never emit unsupported f64.
        assert!(!BUILTIN_WGSL_SOURCE.contains("f64"));
    }

    #[test]
    fn wasm_webgpu_js_bridge_embeds_wgsl_and_exports_host_api() {
        let js = format_wasm_webgpu_js_bridge(Some("// extra kernel"));
        assert!(js.contains("export class CinchWebGpuHost"));
        assert!(js.contains("export function createCinchWebGpuImports"));
        assert!(js.contains("export async function loadCinchWebGpuWasm"));
        assert!(js.contains("navigator?.gpu"));
        assert!(js.contains("cinch_webgpu_dot_f32"));
        assert!(js.contains("cinch_webgpu_stencil_hist_u8"));
        assert!(js.contains("fn dot_f32_relaxed"));
        assert!(js.contains("// extra kernel"));
        assert!(!js.contains("__CINCH_WGSL_PLACEHOLDER__"));
    }
}
