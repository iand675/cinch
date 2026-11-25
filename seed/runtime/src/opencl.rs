//! Production OpenCL 1.2 GPU/Accelerator compute backend for Cinch's
//! data-parallel arrays (equivalent to Futhark's `opencl` backend).
//!
//! Uses `dlopen`/`dlsym` via [`crate::sys`] to reach:
//! - macOS: `/System/Library/Frameworks/OpenCL.framework/OpenCL`
//! - Linux: `libOpenCL.so.1` or `libOpenCL.so`
//! with zero external Cargo dependencies and zero extra linker flags.
//!
//! Supports all 7 Cinch resident compute operations (`dot_f32` exact-tree &
//! relaxed, `scan_i32`, `filter_i32_gt`, batched `stencil_histogram_u8` with
//! 2-buffer ping-pong and adaptive `many-small-frames` vs `few-large-frames`
//! schedules, `stencil2d_f32`, `spmv_csr_f32` with empty rows, and adaptive
//! `row_reduce_f32`), plus a generic OpenCL kernel runner for compiler-emitted
//! OpenCL C kernels.

use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut};
use std::sync::OnceLock;

use crate::metal::{DeviceLaunchPlan, DeviceTelemetry, SyncPoint};
use crate::par_arrays;

// ---------------------------------------------------------------------------
// OpenCL 1.2 C ABI types & constants
// ---------------------------------------------------------------------------

type ClInt = i32;
type ClUint = u32;
type ClBool = u32;
type ClBitfield = u64;
type ClPlatformId = *mut c_void;
type ClDeviceId = *mut c_void;
type ClContext = *mut c_void;
type ClCommandQueue = *mut c_void;
type ClMem = *mut c_void;
type ClProgram = *mut c_void;
type ClKernel = *mut c_void;
type ClEvent = *mut c_void;

const CL_SUCCESS: ClInt = 0;
const CL_TRUE: ClBool = 1;
const CL_DEVICE_TYPE_GPU: ClBitfield = 1 << 2;
const CL_DEVICE_TYPE_ALL: ClBitfield = 0xFFFF_FFFF;
const CL_DEVICE_NAME: ClUint = 0x102B;
const CL_MEM_READ_WRITE: ClBitfield = 1 << 0;
const CL_PROGRAM_BUILD_LOG: ClUint = 0x1183;

struct OpenClApi {
    get_platform_ids: unsafe extern "C" fn(ClUint, *mut ClPlatformId, *mut ClUint) -> ClInt,
    get_device_ids:
        unsafe extern "C" fn(ClPlatformId, ClBitfield, ClUint, *mut ClDeviceId, *mut ClUint) -> ClInt,
    get_device_info:
        unsafe extern "C" fn(ClDeviceId, ClUint, usize, *mut c_void, *mut usize) -> ClInt,
    create_context: unsafe extern "C" fn(
        *const isize,
        ClUint,
        *const ClDeviceId,
        *const c_void,
        *mut c_void,
        *mut ClInt,
    ) -> ClContext,
    create_command_queue:
        unsafe extern "C" fn(ClContext, ClDeviceId, ClBitfield, *mut ClInt) -> ClCommandQueue,
    create_buffer:
        unsafe extern "C" fn(ClContext, ClBitfield, usize, *mut c_void, *mut ClInt) -> ClMem,
    enqueue_write_buffer: unsafe extern "C" fn(
        ClCommandQueue,
        ClMem,
        ClBool,
        usize,
        usize,
        *const c_void,
        ClUint,
        *const ClEvent,
        *mut ClEvent,
    ) -> ClInt,
    enqueue_read_buffer: unsafe extern "C" fn(
        ClCommandQueue,
        ClMem,
        ClBool,
        usize,
        usize,
        *mut c_void,
        ClUint,
        *const ClEvent,
        *mut ClEvent,
    ) -> ClInt,
    release_mem_object: unsafe extern "C" fn(ClMem) -> ClInt,
    create_program_with_source: unsafe extern "C" fn(
        ClContext,
        ClUint,
        *const *const i8,
        *const usize,
        *mut ClInt,
    ) -> ClProgram,
    build_program: unsafe extern "C" fn(
        ClProgram,
        ClUint,
        *const ClDeviceId,
        *const i8,
        *const c_void,
        *mut c_void,
    ) -> ClInt,
    get_program_build_info: unsafe extern "C" fn(
        ClProgram,
        ClDeviceId,
        ClUint,
        usize,
        *mut c_void,
        *mut usize,
    ) -> ClInt,
    create_kernel: unsafe extern "C" fn(ClProgram, *const i8, *mut ClInt) -> ClKernel,
    set_kernel_arg: unsafe extern "C" fn(ClKernel, ClUint, usize, *const c_void) -> ClInt,
    enqueue_nd_range_kernel: unsafe extern "C" fn(
        ClCommandQueue,
        ClKernel,
        ClUint,
        *const usize,
        *const usize,
        *const usize,
        ClUint,
        *const ClEvent,
        *mut ClEvent,
    ) -> ClInt,
    finish: unsafe extern "C" fn(ClCommandQueue) -> ClInt,
    release_kernel: unsafe extern "C" fn(ClKernel) -> ClInt,
    release_program: unsafe extern "C" fn(ClProgram) -> ClInt,
    release_command_queue: unsafe extern "C" fn(ClCommandQueue) -> ClInt,
    release_context: unsafe extern "C" fn(ClContext) -> ClInt,
}

unsafe impl Send for OpenClApi {}
unsafe impl Sync for OpenClApi {}

static OCL_API: OnceLock<Option<OpenClApi>> = OnceLock::new();

fn api() -> Option<&'static OpenClApi> {
    OCL_API
        .get_or_init(|| unsafe {
            let candidates: &[&[u8]] = &[
                b"/System/Library/Frameworks/OpenCL.framework/OpenCL",
                b"libOpenCL.so.1",
                b"libOpenCL.so",
                b"/usr/lib/x86_64-linux-gnu/libOpenCL.so.1",
                b"/usr/lib/aarch64-linux-gnu/libOpenCL.so.1",
            ];
            let mut lib = null_mut();
            for path in candidates {
                lib = crate::sys::dlopen(path);
                if !lib.is_null() {
                    break;
                }
            }
            if lib.is_null() {
                return None;
            }
            macro_rules! load {
                ($name:literal) => {{
                    let p = crate::sys::dlsym(lib, $name);
                    if p.is_null() {
                        return None;
                    }
                    std::mem::transmute(p)
                }};
            }
            Some(OpenClApi {
                get_platform_ids: load!(b"clGetPlatformIDs"),
                get_device_ids: load!(b"clGetDeviceIDs"),
                get_device_info: load!(b"clGetDeviceInfo"),
                create_context: load!(b"clCreateContext"),
                create_command_queue: load!(b"clCreateCommandQueue"),
                create_buffer: load!(b"clCreateBuffer"),
                enqueue_write_buffer: load!(b"clEnqueueWriteBuffer"),
                enqueue_read_buffer: load!(b"clEnqueueReadBuffer"),
                release_mem_object: load!(b"clReleaseMemObject"),
                create_program_with_source: load!(b"clCreateProgramWithSource"),
                build_program: load!(b"clBuildProgram"),
                get_program_build_info: load!(b"clGetProgramBuildInfo"),
                create_kernel: load!(b"clCreateKernel"),
                set_kernel_arg: load!(b"clSetKernelArg"),
                enqueue_nd_range_kernel: load!(b"clEnqueueNDRangeKernel"),
                finish: load!(b"clFinish"),
                release_kernel: load!(b"clReleaseKernel"),
                release_program: load!(b"clReleaseProgram"),
                release_command_queue: load!(b"clReleaseCommandQueue"),
                release_context: load!(b"clReleaseContext"),
            })
        })
        .as_ref()
}

pub fn is_available() -> bool {
    OpenClState::open().is_ok()
}

// ---------------------------------------------------------------------------
// Built-in Production OpenCL C 1.2 Kernels (all 7 Cinch kernel families)
// ---------------------------------------------------------------------------

pub const BUILTIN_OPENCL_SOURCE: &str = r#"
// 1. Fused Dot Product (Relaxed & Exact-Tree matching CPU f32_exact_tree_prod bit-for-bit)

__kernel void dot_f32_relaxed(
    __global const float* a,
    __global const float* b,
    __global float* partials,
    const uint n
) {
    __local float s[256];
    uint gid = get_global_id(0);
    uint gsize = get_global_size(0);
    uint tid = get_local_id(0);
    uint wid = get_group_id(0);
    float acc = 0.0f;
    for (uint i = gid; i < n; i += gsize) {
        acc += a[i] * b[i];
    }
    s[tid] = acc;
    barrier(CLK_LOCAL_MEM_FENCE);
    for (uint stride = 128; stride > 0; stride >>= 1) {
        if (tid < stride) {
            s[tid] += s[tid + stride];
        }
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    if (tid == 0) {
        partials[wid] = s[0];
    }
}

__kernel void dot_f32_exact_block(
    __global const float* a,
    __global const float* b,
    __global float* block_out,
    const uint n
) {
    __local float s[1024];
    uint tid = get_local_id(0);
    uint wid = get_group_id(0);
    uint base = wid * 1024u;
    for (uint k = 0; k < 4u; ++k) {
        uint idx = tid + k * 256u;
        uint g = base + idx;
        s[idx] = (g < n) ? (a[g] * b[g]) : 0.0f;
    }
    barrier(CLK_LOCAL_MEM_FENCE);
    uint len = (base < n) ? min(1024u, n - base) : 0u;
    if (len == 1024u) {
        for (uint stride = 1u; stride < 1024u; stride <<= 1) {
            for (uint k = 0; k < 4u; ++k) {
                uint idx = tid + k * 256u;
                if ((idx & ((stride << 1) - 1u)) == 0u) {
                    s[idx] = s[idx] + s[idx + stride];
                }
            }
            barrier(CLK_LOCAL_MEM_FENCE);
        }
        if (tid == 0) {
            block_out[wid] = s[0];
        }
    } else if (tid == 0) {
        float vals[12];
        uint sizes[12];
        uint sp = 0;
        for (uint i = 0; i < len; ++i) {
            float cur = s[i];
            uint sz = 1u;
            while (sp > 0 && sizes[sp - 1] == sz) {
                cur = vals[sp - 1] + cur;
                sz <<= 1;
                sp -= 1;
            }
            vals[sp] = cur;
            sizes[sp] = sz;
            sp += 1;
        }
        float acc = 0.0f;
        if (sp > 0) {
            acc = vals[sp - 1];
            for (int j = (int)sp - 2; j >= 0; --j) {
                acc = vals[j] + acc;
            }
        }
        block_out[wid] = acc;
    }
}

// 2. Prefix Scan (Inclusive & Exclusive, 256-element block scan + offsets)

__kernel void scan_i32_block(
    __global const int* in_arr,
    __global int* out_arr,
    __global int* block_sums,
    const uint n,
    const uint inclusive
) {
    __local int s[256];
    uint tid = get_local_id(0);
    uint wid = get_group_id(0);
    uint gid = wid * 256u + tid;
    int val = (gid < n) ? in_arr[gid] : 0;
    s[tid] = val;
    barrier(CLK_LOCAL_MEM_FENCE);
    for (uint offset = 1u; offset < 256u; offset <<= 1) {
        int add = (tid >= offset) ? s[tid - offset] : 0;
        barrier(CLK_LOCAL_MEM_FENCE);
        s[tid] += add;
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    if (tid == 255u) {
        block_sums[wid] = s[255];
    }
    if (gid < n) {
        out_arr[gid] = (inclusive != 0u) ? s[tid] : (s[tid] - val);
    }
}

__kernel void scan_i32_add_offsets(
    __global int* out_arr,
    __global const int* block_offsets,
    const uint n
) {
    uint tid = get_local_id(0);
    uint wid = get_group_id(0);
    uint gid = wid * 256u + tid;
    if (gid < n) {
        out_arr[gid] += block_offsets[wid];
    }
}

// 3. Stable Compaction (Filter i32 > threshold): flags -> scan -> scatter

__kernel void filter_i32_gt_flags(
    __global const int* in_arr,
    __global int* flags,
    const uint n,
    const int threshold
) {
    uint gid = get_global_id(0);
    if (gid < n) {
        flags[gid] = (in_arr[gid] > threshold) ? 1 : 0;
    }
}

__kernel void filter_i32_gt_scatter(
    __global const int* in_arr,
    __global const int* flags,
    __global const int* positions,
    __global int* out_arr,
    const uint n
) {
    uint gid = get_global_id(0);
    if (gid < n && flags[gid] != 0) {
        out_arr[positions[gid]] = in_arr[gid];
    }
}

// 4. Batched Histogram (U8 -> U32) with TWO adaptive schedules

__kernel void hist_u8_zero(
    __global uint* out_hist,
    const uint total
) {
    uint gid = get_global_id(0);
    if (gid < total) {
        out_hist[gid] = 0u;
    }
}

__kernel void hist_u8_many_small_frames(
    __global const uchar* pixels,
    __global uint* out_hist,
    const uint frames,
    const uint frame_pixels,
    const uint num_bins
) {
    uint frame = get_group_id(0);
    uint tid = get_local_id(0);
    if (frame >= frames) return;
    __local uint local_bins[256];
    local_bins[tid] = 0u;
    barrier(CLK_LOCAL_MEM_FENCE);
    uint base = frame * frame_pixels;
    for (uint i = tid; i < frame_pixels; i += 256u) {
        uint b = (uint)pixels[base + i];
        if (b < num_bins) {
            atomic_inc(&local_bins[b]);
        }
    }
    barrier(CLK_LOCAL_MEM_FENCE);
    if (tid < num_bins) {
        out_hist[frame * num_bins + tid] = local_bins[tid];
    }
}

__kernel void hist_u8_few_large_frames(
    __global const uchar* pixels,
    __global uint* out_hist,
    const uint frames,
    const uint frame_pixels,
    const uint num_bins,
    const uint tiles_per_frame
) {
    uint wid = get_group_id(0);
    uint tid = get_local_id(0);
    uint frame = wid / tiles_per_frame;
    uint tile = wid % tiles_per_frame;
    if (frame >= frames) return;
    __local uint local_bins[256];
    local_bins[tid] = 0u;
    barrier(CLK_LOCAL_MEM_FENCE);
    uint base = frame * frame_pixels;
    uint stride = tiles_per_frame * 256u;
    for (uint i = tile * 256u + tid; i < frame_pixels; i += stride) {
        uint b = (uint)pixels[base + i];
        if (b < num_bins) {
            atomic_inc(&local_bins[b]);
        }
    }
    barrier(CLK_LOCAL_MEM_FENCE);
    if (tid < num_bins) {
        uint c = local_bins[tid];
        if (c > 0u) {
            atomic_add(&out_hist[frame * num_bins + tid], c);
        }
    }
}

// 5. Batched 2D 3x3 Stencil (U8 & F32) with explicit edge policy & 18x18 halo tile

inline int resolve_coord(int idx, int dim, uint policy) {
    if (idx >= 0 && idx < dim) return idx;
    if (policy == 0u) return clamp(idx, 0, dim - 1);
    if (policy == 1u) return -1;
    if (policy == 2u) {
        if (dim <= 1) return 0;
        int period = 2 * dim - 2;
        int m = ((idx % period) + period) % period;
        return (m >= dim) ? (period - m) : m;
    }
    return ((idx % dim) + dim) % dim;
}

__kernel void stencil2d_u8_tiled(
    __global const uchar* src,
    __global uchar* dst,
    const uint frames,
    const uint height,
    const uint width,
    const int divisor,
    const uint edge_policy,
    const int w0, const int w1, const int w2,
    const int w3, const int w4, const int w5,
    const int w6, const int w7, const int w8
) {
    uint gx = get_global_id(0);
    uint gy = get_global_id(1);
    uint frame = get_global_id(2);
    if (frame >= frames) return;
    int h = (int)height;
    int w = (int)width;
    uint fbase = frame * height * width;

    __local uchar tile[18][18];
    int tile_x0 = (int)get_group_id(0) * 16 - 1;
    int tile_y0 = (int)get_group_id(1) * 16 - 1;
    uint lx0 = get_local_id(0);
    uint ly0 = get_local_id(1);
    uint flat_tid = ly0 * 16u + lx0;

    for (uint k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u);
        int tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? (uchar)0 : src[fbase + (uint)(ry * w + rx)];
    }
    barrier(CLK_LOCAL_MEM_FENCE);

    if (gx >= width || gy >= height) return;
    uint ly = ly0 + 1u;
    uint lx = lx0 + 1u;
    int acc = w0 * (int)tile[ly - 1][lx - 1]
            + w1 * (int)tile[ly - 1][lx]
            + w2 * (int)tile[ly - 1][lx + 1]
            + w3 * (int)tile[ly][lx - 1]
            + w4 * (int)tile[ly][lx]
            + w5 * (int)tile[ly][lx + 1]
            + w6 * (int)tile[ly + 1][lx - 1]
            + w7 * (int)tile[ly + 1][lx]
            + w8 * (int)tile[ly + 1][lx + 1];
    int div = (divisor == 0) ? 1 : divisor;
    int q = acc / div;
    dst[fbase + gy * width + gx] = (uchar)clamp(q, 0, 255);
}

__kernel void stencil2d_f32_tiled(
    __global const float* src,
    __global float* dst,
    const uint frames,
    const uint height,
    const uint width,
    const uint edge_policy,
    const float w0, const float w1, const float w2,
    const float w3, const float w4, const float w5,
    const float w6, const float w7, const float w8
) {
    uint gx = get_global_id(0);
    uint gy = get_global_id(1);
    uint frame = get_global_id(2);
    if (frame >= frames) return;
    int h = (int)height;
    int w = (int)width;
    uint fbase = frame * height * width;

    __local float tile[18][18];
    int tile_x0 = (int)get_group_id(0) * 16 - 1;
    int tile_y0 = (int)get_group_id(1) * 16 - 1;
    uint lx0 = get_local_id(0);
    uint ly0 = get_local_id(1);
    uint flat_tid = ly0 * 16u + lx0;

    for (uint k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u);
        int tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0.0f : src[fbase + (uint)(ry * w + rx)];
    }
    barrier(CLK_LOCAL_MEM_FENCE);

    if (gx >= width || gy >= height) return;
    uint ly = ly0 + 1u;
    uint lx = lx0 + 1u;
    float acc = w0 * tile[ly - 1][lx - 1]
              + w1 * tile[ly - 1][lx]
              + w2 * tile[ly - 1][lx + 1]
              + w3 * tile[ly][lx - 1]
              + w4 * tile[ly][lx]
              + w5 * tile[ly][lx + 1]
              + w6 * tile[ly + 1][lx - 1]
              + w7 * tile[ly + 1][lx]
              + w8 * tile[ly + 1][lx + 1];
    dst[fbase + gy * width + gx] = acc;
}

// 6. Row Reduction (Adaptive Outer-Parallel vs Inner-Parallel)

__kernel void row_reduce_f32_outer(
    __global const float* a,
    __global float* out,
    const uint rows,
    const uint cols
) {
    uint row = get_global_id(0);
    if (row >= rows) return;
    float acc = 0.0f;
    uint base = row * cols;
    for (uint j = 0; j < cols; ++j) {
        acc += a[base + j];
    }
    out[row] = acc;
}

__kernel void row_reduce_f32_inner(
    __global const float* a,
    __global float* out,
    const uint rows,
    const uint cols
) {
    __local float s[256];
    uint row = get_group_id(0);
    uint tid = get_local_id(0);
    if (row >= rows) return;
    float acc = 0.0f;
    uint base = row * cols;
    for (uint j = tid; j < cols; j += 256u) {
        acc += a[base + j];
    }
    s[tid] = acc;
    barrier(CLK_LOCAL_MEM_FENCE);
    for (uint stride = 128u; stride > 0u; stride >>= 1) {
        if (tid < stride) {
            s[tid] += s[tid + stride];
        }
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    if (tid == 0u) {
        out[row] = s[0];
    }
}

// 7. CSR SpMV (Segmented Reduction with empty rows: scalar, workgroup, exact-tree)

__kernel void spmv_csr_f32_scalar(
    __global const float* vals,
    __global const uint* cols,
    __global const uint* row_ptr,
    __global const float* x,
    __global float* y,
    const uint nrows
) {
    uint row = get_global_id(0);
    if (row >= nrows) return;
    uint s = row_ptr[row];
    uint e = row_ptr[row + 1];
    float acc = 0.0f;
    for (uint k = s; k < e; ++k) {
        acc += vals[k] * x[cols[k]];
    }
    y[row] = acc;
}

__kernel void spmv_csr_f32_wg(
    __global const float* vals,
    __global const uint* cols,
    __global const uint* row_ptr,
    __global const float* x,
    __global float* y,
    const uint nrows
) {
    __local float s[256];
    uint gid = get_global_id(0);
    uint tid = get_local_id(0);
    uint lane = tid & 31u;
    uint row = gid >> 5;
    float acc = 0.0f;
    if (row < nrows) {
        uint st = row_ptr[row];
        uint en = row_ptr[row + 1];
        for (uint k = st + lane; k < en; k += 32u) {
            acc += vals[k] * x[cols[k]];
        }
    }
    s[tid] = acc;
    barrier(CLK_LOCAL_MEM_FENCE);
    for (uint stride = 16u; stride > 0u; stride >>= 1) {
        if (lane < stride) {
            s[tid] += s[tid + stride];
        }
        barrier(CLK_LOCAL_MEM_FENCE);
    }
    if (lane == 0u && row < nrows) {
        y[row] = s[tid];
    }
}

__kernel void spmv_csr_f32_exact(
    __global const float* vals,
    __global const uint* cols,
    __global const uint* row_ptr,
    __global const float* x,
    __global float* y,
    const uint nrows
) {
    uint row = get_global_id(0);
    if (row >= nrows) return;
    uint s = row_ptr[row];
    uint e = row_ptr[row + 1];
    if (e <= s) {
        y[row] = 0.0f;
        return;
    }
    float st_val[32];
    uint st_sz[32];
    uint sp = 0u;
    for (uint k = s; k < e; ++k) {
        float cur = vals[k] * x[cols[k]];
        uint sz = 1u;
        while (sp > 0u && st_sz[sp - 1] == sz) {
            cur = st_val[sp - 1] + cur;
            sz <<= 1;
            sp -= 1u;
        }
        st_val[sp] = cur;
        st_sz[sp] = sz;
        sp += 1u;
    }
    float acc = st_val[sp - 1];
    for (int j = (int)sp - 2; j >= 0; --j) {
        acc = st_val[j] + acc;
    }
    y[row] = acc;
}
"#;

// ---------------------------------------------------------------------------
// OpenCL Context, Buffer, Program & Kernel Execution
// ---------------------------------------------------------------------------

pub struct OpenClBuffer {
    pub mem: ClMem,
    pub bytes: usize,
}

unsafe impl Send for OpenClBuffer {}
unsafe impl Sync for OpenClBuffer {}

impl Drop for OpenClBuffer {
    fn drop(&mut self) {
        if !self.mem.is_null() {
            if let Some(a) = api() {
                unsafe { (a.release_mem_object)(self.mem) };
            }
        }
    }
}

pub struct OpenClState {
    pub device_name: String,
    device: ClDeviceId,
    context: ClContext,
    queue: ClCommandQueue,
    program: ClProgram,
    kernels: HashMap<String, ClKernel>,
}

unsafe impl Send for OpenClState {}
unsafe impl Sync for OpenClState {}

impl Drop for OpenClState {
    fn drop(&mut self) {
        if let Some(a) = api() {
            for (_, k) in self.kernels.drain() {
                unsafe { (a.release_kernel)(k) };
            }
            if !self.program.is_null() {
                unsafe { (a.release_program)(self.program) };
            }
            if !self.queue.is_null() {
                unsafe { (a.release_command_queue)(self.queue) };
            }
            if !self.context.is_null() {
                unsafe { (a.release_context)(self.context) };
            }
        }
    }
}

impl OpenClState {
    pub fn open() -> Result<Self, String> {
        let a = api().ok_or_else(|| "OpenCL runtime library not found on this host".to_string())?;
        let mut num_platforms: ClUint = 0;
        let rc = unsafe { (a.get_platform_ids)(0, null_mut(), &mut num_platforms) };
        if rc != CL_SUCCESS || num_platforms == 0 {
            return Err(format!("no OpenCL platforms found (clGetPlatformIDs={rc})"));
        }
        let mut platforms = vec![null_mut(); num_platforms as usize];
        let rc = unsafe { (a.get_platform_ids)(num_platforms, platforms.as_mut_ptr(), null_mut()) };
        if rc != CL_SUCCESS {
            return Err(format!("clGetPlatformIDs failed ({rc})"));
        }

        // Prefer GPU device across all platforms; fall back to any OpenCL device.
        let mut chosen_dev: ClDeviceId = null_mut();
        for &dtype in &[CL_DEVICE_TYPE_GPU, CL_DEVICE_TYPE_ALL] {
            for &plat in &platforms {
                let mut ndev: ClUint = 0;
                if unsafe { (a.get_device_ids)(plat, dtype, 0, null_mut(), &mut ndev) }
                    == CL_SUCCESS
                    && ndev > 0
                {
                    let mut devs = vec![null_mut(); ndev as usize];
                    if unsafe { (a.get_device_ids)(plat, dtype, ndev, devs.as_mut_ptr(), null_mut()) }
                        == CL_SUCCESS
                    {
                        chosen_dev = devs[0];
                        break;
                    }
                }
            }
            if !chosen_dev.is_null() {
                break;
            }
        }
        if chosen_dev.is_null() {
            return Err("no OpenCL compute devices available".to_string());
        }

        let mut name_buf = [0u8; 256];
        let mut name_len = 0usize;
        let _ = unsafe {
            (a.get_device_info)(
                chosen_dev,
                CL_DEVICE_NAME,
                name_buf.len(),
                name_buf.as_mut_ptr().cast(),
                &mut name_len,
            )
        };
        let device_name = if name_len > 0 {
            unsafe { CStr::from_ptr(name_buf.as_ptr().cast()) }
                .to_string_lossy()
                .trim()
                .to_string()
        } else {
            "OpenCL Device".to_string()
        };

        let mut err: ClInt = 0;
        let context = unsafe { (a.create_context)(null(), 1, &chosen_dev, null(), null_mut(), &mut err) };
        if context.is_null() || err != CL_SUCCESS {
            return Err(format!("clCreateContext failed ({err})"));
        }
        let queue = unsafe { (a.create_command_queue)(context, chosen_dev, 0, &mut err) };
        if queue.is_null() || err != CL_SUCCESS {
            unsafe { (a.release_context)(context) };
            return Err(format!("clCreateCommandQueue failed ({err})"));
        }

        let src_ptr = BUILTIN_OPENCL_SOURCE.as_ptr().cast::<i8>();
        let src_len = BUILTIN_OPENCL_SOURCE.len();
        let program = unsafe { (a.create_program_with_source)(context, 1, &src_ptr, &src_len, &mut err) };
        if program.is_null() || err != CL_SUCCESS {
            unsafe {
                (a.release_command_queue)(queue);
                (a.release_context)(context);
            }
            return Err(format!("clCreateProgramWithSource failed ({err})"));
        }

        let build_rc = unsafe { (a.build_program)(program, 1, &chosen_dev, null(), null(), null_mut()) };
        if build_rc != CL_SUCCESS {
            let mut log_len = 0usize;
            unsafe {
                (a.get_program_build_info)(program, chosen_dev, CL_PROGRAM_BUILD_LOG, 0, null_mut(), &mut log_len);
            }
            let mut log = vec![0u8; log_len.max(1)];
            unsafe {
                (a.get_program_build_info)(
                    program,
                    chosen_dev,
                    CL_PROGRAM_BUILD_LOG,
                    log.len(),
                    log.as_mut_ptr().cast(),
                    null_mut(),
                );
                (a.release_program)(program);
                (a.release_command_queue)(queue);
                (a.release_context)(context);
            }
            let msg = String::from_utf8_lossy(&log).trim().to_string();
            return Err(format!("clBuildProgram failed ({build_rc}): {msg}"));
        }

        Ok(Self {
            device_name,
            device: chosen_dev,
            context,
            queue,
            program,
            kernels: HashMap::new(),
        })
    }

    fn kernel(&mut self, name: &str) -> Result<ClKernel, String> {
        if let Some(&k) = self.kernels.get(name) {
            return Ok(k);
        }
        let a = api().ok_or_else(|| "OpenCL unavailable".to_string())?;
        let mut cstr = Vec::with_capacity(name.len() + 1);
        cstr.extend_from_slice(name.as_bytes());
        cstr.push(0);
        let mut err: ClInt = 0;
        let k = unsafe { (a.create_kernel)(self.program, cstr.as_ptr().cast(), &mut err) };
        if k.is_null() || err != CL_SUCCESS {
            return Err(format!("clCreateKernel('{name}') failed ({err})"));
        }
        self.kernels.insert(name.to_string(), k);
        Ok(k)
    }

    pub fn alloc_buf(&self, bytes: usize) -> Result<OpenClBuffer, String> {
        let a = api().ok_or_else(|| "OpenCL unavailable".to_string())?;
        let alloc_bytes = bytes.max(4);
        let mut err: ClInt = 0;
        let mem = unsafe {
            (a.create_buffer)(self.context, CL_MEM_READ_WRITE, alloc_bytes, null_mut(), &mut err)
        };
        if mem.is_null() || err != CL_SUCCESS {
            return Err(format!("clCreateBuffer({alloc_bytes} bytes) failed ({err})"));
        }
        Ok(OpenClBuffer { mem, bytes })
    }

    pub fn upload(&self, src: *const u8, bytes: usize) -> Result<OpenClBuffer, String> {
        let a = api().ok_or_else(|| "OpenCL unavailable".to_string())?;
        let buf = self.alloc_buf(bytes)?;
        if bytes > 0 && !src.is_null() {
            let rc = unsafe {
                (a.enqueue_write_buffer)(
                    self.queue,
                    buf.mem,
                    CL_TRUE,
                    0,
                    bytes,
                    src.cast(),
                    0,
                    null(),
                    null_mut(),
                )
            };
            if rc != CL_SUCCESS {
                return Err(format!("clEnqueueWriteBuffer failed ({rc})"));
            }
        }
        Ok(buf)
    }

    pub fn download(&self, buf: &OpenClBuffer, dst: *mut u8, bytes: usize) -> Result<(), String> {
        let a = api().ok_or_else(|| "OpenCL unavailable".to_string())?;
        if bytes > 0 && !dst.is_null() {
            let rc = unsafe {
                (a.enqueue_read_buffer)(
                    self.queue,
                    buf.mem,
                    CL_TRUE,
                    0,
                    bytes,
                    dst.cast(),
                    0,
                    null(),
                    null_mut(),
                )
            };
            if rc != CL_SUCCESS {
                return Err(format!("clEnqueueReadBuffer failed ({rc})"));
            }
        }
        Ok(())
    }

    fn set_arg_mem(&self, k: ClKernel, idx: u32, mem: ClMem) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe {
            (a.set_kernel_arg)(k, idx, std::mem::size_of::<ClMem>(), (&mem as *const ClMem).cast())
        };
        if rc != CL_SUCCESS {
            return Err(format!("clSetKernelArg(mem, {idx}) failed ({rc})"));
        }
        Ok(())
    }

    fn set_arg_val<T>(&self, k: ClKernel, idx: u32, val: &T) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe { (a.set_kernel_arg)(k, idx, std::mem::size_of::<T>(), (val as *const T).cast()) };
        if rc != CL_SUCCESS {
            return Err(format!("clSetKernelArg(val, {idx}) failed ({rc})"));
        }
        Ok(())
    }

    fn enqueue_1d(&self, k: ClKernel, global: usize, local: usize) -> Result<(), String> {
        let a = api().unwrap();
        let g = [global];
        let l = [local];
        let rc = unsafe {
            (a.enqueue_nd_range_kernel)(self.queue, k, 1, null(), g.as_ptr(), l.as_ptr(), 0, null(), null_mut())
        };
        if rc != CL_SUCCESS {
            return Err(format!("clEnqueueNDRangeKernel(1D, g={global}, l={local}) failed ({rc})"));
        }
        Ok(())
    }

    fn enqueue_3d(&self, k: ClKernel, global: [usize; 3], local: [usize; 3]) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe {
            (a.enqueue_nd_range_kernel)(
                self.queue,
                k,
                3,
                null(),
                global.as_ptr(),
                local.as_ptr(),
                0,
                null(),
                null_mut(),
            )
        };
        if rc != CL_SUCCESS {
            return Err(format!("clEnqueueNDRangeKernel(3D, g={global:?}, l={local:?}) failed ({rc})"));
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe { (a.finish)(self.queue) };
        if rc != CL_SUCCESS {
            return Err(format!("clFinish failed ({rc})"));
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // 1. Dot product (Exact-Tree & Relaxed)
    // -----------------------------------------------------------------------
    pub fn dot_f32(
        &mut self,
        a_buf: &OpenClBuffer,
        b_buf: &OpenClBuffer,
        n: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<f32, String> {
        if n == 0 {
            return Ok(0.0);
        }
        let n_u32 = n as u32;
        if exact {
            let nblocks = (n + 1023) / 1024;
            let partial_buf = self.alloc_buf(nblocks * 4)?;
            telemetry.note_alloc(nblocks * 4);
            let k = self.kernel("dot_f32_exact_block")?;
            self.set_arg_mem(k, 0, a_buf.mem)?;
            self.set_arg_mem(k, 1, b_buf.mem)?;
            self.set_arg_mem(k, 2, partial_buf.mem)?;
            self.set_arg_val(k, 3, &n_u32)?;
            self.enqueue_1d(k, nblocks * 256, 256)?;
            let mut partials = vec![0.0f32; nblocks];
            self.download(&partial_buf, partials.as_mut_ptr().cast(), nblocks * 4)?;
            telemetry.note_free(nblocks * 4);
            telemetry.kernel_launches += 1;
            telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-exact-tree".to_string(),
                reason: "opencl-block-partials-readback".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-exact-tree".to_string(),
                schedule: format!("opencl-exact-tree-1024(blocks={nblocks})"),
                grid: [nblocks * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 1024 * 4,
                numerical_mode: "f32-add-exact-tree".to_string(),
            };
            Ok(par_arrays::f32_exact_tree_sum(&partials))
        } else {
            let ngroups = ((n + 255) / 256).clamp(1, 1024);
            let partial_buf = self.alloc_buf(ngroups * 4)?;
            telemetry.note_alloc(ngroups * 4);
            let k = self.kernel("dot_f32_relaxed")?;
            self.set_arg_mem(k, 0, a_buf.mem)?;
            self.set_arg_mem(k, 1, b_buf.mem)?;
            self.set_arg_mem(k, 2, partial_buf.mem)?;
            self.set_arg_val(k, 3, &n_u32)?;
            self.enqueue_1d(k, ngroups * 256, 256)?;
            let mut partials = vec![0.0f32; ngroups];
            self.download(&partial_buf, partials.as_mut_ptr().cast(), ngroups * 4)?;
            telemetry.note_free(ngroups * 4);
            telemetry.kernel_launches += 1;
            telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-relaxed".to_string(),
                reason: "opencl-relaxed-partials-readback".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-relaxed".to_string(),
                schedule: format!("opencl-grid-stride(groups={ngroups})"),
                grid: [ngroups * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 256 * 4,
                numerical_mode: "f32-add-relaxed".to_string(),
            };
            Ok(partials.into_iter().fold(0.0f32, |a, b| a + b))
        }
    }

    // -----------------------------------------------------------------------
    // 2. Prefix scan (inclusive & exclusive)
    // -----------------------------------------------------------------------
    pub fn scan_i32(
        &mut self,
        in_buf: &OpenClBuffer,
        n: usize,
        inclusive: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<OpenClBuffer, String> {
        let out_buf = self.alloc_buf(n * 4)?;
        telemetry.note_alloc(n * 4);
        if n == 0 {
            return Ok(out_buf);
        }
        let nblocks = (n + 255) / 256;
        let block_sums = self.alloc_buf(nblocks * 4)?;
        telemetry.note_alloc(nblocks * 4);
        let n_u32 = n as u32;
        let inc_u32 = if inclusive { 1u32 } else { 0u32 };

        let k1 = self.kernel("scan_i32_block")?;
        self.set_arg_mem(k1, 0, in_buf.mem)?;
        self.set_arg_mem(k1, 1, out_buf.mem)?;
        self.set_arg_mem(k1, 2, block_sums.mem)?;
        self.set_arg_val(k1, 3, &n_u32)?;
        self.set_arg_val(k1, 4, &inc_u32)?;
        self.enqueue_1d(k1, nblocks * 256, 256)?;

        let mut launches = 1usize;
        if nblocks > 1 {
            let mut h_sums = vec![0i32; nblocks];
            self.download(&block_sums, h_sums.as_mut_ptr().cast(), nblocks * 4)?;
            let mut acc = 0i32;
            for v in &mut h_sums {
                let x = *v;
                *v = acc;
                acc = acc.wrapping_add(x);
            }
            let offsets = self.upload(h_sums.as_ptr().cast(), nblocks * 4)?;
            let k2 = self.kernel("scan_i32_add_offsets")?;
            self.set_arg_mem(k2, 0, out_buf.mem)?;
            self.set_arg_mem(k2, 1, offsets.mem)?;
            self.set_arg_val(k2, 2, &n_u32)?;
            self.enqueue_1d(k2, nblocks * 256, 256)?;
            launches += 1;
        }
        self.finish()?;
        telemetry.note_free(nblocks * 4);
        telemetry.kernel_launches += launches;
        telemetry.sync_points.push(SyncPoint {
            op: "scan-i32".to_string(),
            reason: "opencl-queue-finish".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: if inclusive { "scan-inclusive-i32" } else { "scan-exclusive-i32" }.to_string(),
            schedule: format!("opencl-2pass-scan(blocks={nblocks})"),
            grid: [nblocks * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "wrapping-i32".to_string(),
        };
        Ok(out_buf)
    }

    // -----------------------------------------------------------------------
    // 3. Stable Compaction (filter i32 > threshold)
    // -----------------------------------------------------------------------
    pub fn filter_i32_gt(
        &mut self,
        in_buf: &OpenClBuffer,
        n: usize,
        threshold: i32,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<(OpenClBuffer, usize), String> {
        if n == 0 {
            let out = self.alloc_buf(0)?;
            return Ok((out, 0));
        }
        let flags = self.alloc_buf(n * 4)?;
        telemetry.note_alloc(n * 4);
        let nblocks = (n + 255) / 256;
        let n_u32 = n as u32;

        let kf = self.kernel("filter_i32_gt_flags")?;
        self.set_arg_mem(kf, 0, in_buf.mem)?;
        self.set_arg_mem(kf, 1, flags.mem)?;
        self.set_arg_val(kf, 2, &n_u32)?;
        self.set_arg_val(kf, 3, &threshold)?;
        self.enqueue_1d(kf, nblocks * 256, 256)?;

        let positions = self.scan_i32(&flags, n, false, telemetry)?;
        let mut last_flag = 0i32;
        let mut last_pos = 0i32;
        let a = api().unwrap();
        unsafe {
            (a.enqueue_read_buffer)(
                self.queue,
                flags.mem,
                CL_TRUE,
                (n - 1) * 4,
                4,
                (&mut last_flag as *mut i32).cast(),
                0,
                null(),
                null_mut(),
            );
            (a.enqueue_read_buffer)(
                self.queue,
                positions.mem,
                CL_TRUE,
                (n - 1) * 4,
                4,
                (&mut last_pos as *mut i32).cast(),
                0,
                null(),
                null_mut(),
            );
        }
        let k = (last_pos + last_flag).max(0) as usize;
        let out_buf = self.alloc_buf(k * 4)?;
        telemetry.note_alloc(k * 4);
        if k > 0 {
            let ks = self.kernel("filter_i32_gt_scatter")?;
            self.set_arg_mem(ks, 0, in_buf.mem)?;
            self.set_arg_mem(ks, 1, flags.mem)?;
            self.set_arg_mem(ks, 2, positions.mem)?;
            self.set_arg_mem(ks, 3, out_buf.mem)?;
            self.set_arg_val(ks, 4, &n_u32)?;
            self.enqueue_1d(ks, nblocks * 256, 256)?;
            self.finish()?;
        }
        telemetry.note_free(flags.bytes);
        telemetry.note_free(positions.bytes);
        telemetry.kernel_launches += 2;
        telemetry.sync_points.push(SyncPoint {
            op: "filter-i32-gt".to_string(),
            reason: "opencl-compact-complete".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: "filter-i32-gt".to_string(),
            schedule: format!("opencl-flag-scan-scatter(n={n},k={k})"),
            grid: [nblocks * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "stable-compaction".to_string(),
        };
        Ok((out_buf, k))
    }

    // -----------------------------------------------------------------------
    // 4. Batched 3x3 U8 Stencil (ping-pong) -> Per-Frame Histogram
    // -----------------------------------------------------------------------
    #[allow(clippy::too_many_arguments)]
    pub fn stencil_histogram_u8(
        &mut self,
        in_buf: &OpenClBuffer,
        f: usize,
        h: usize,
        w: usize,
        ws: [i32; 9],
        divisor: i32,
        edge_policy: u32,
        nsteps: usize,
        bins: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<OpenClBuffer, String> {
        let frame_pixels = h * w;
        let total_pixels = f * frame_pixels;
        let out_hist = self.alloc_buf(f * bins * 4)?;
        telemetry.note_alloc(f * bins * 4);
        if f == 0 || h == 0 || w == 0 || bins == 0 {
            return Ok(out_hist);
        }

        let ping = if nsteps >= 1 {
            let b = self.alloc_buf(total_pixels)?;
            telemetry.note_alloc(total_pixels);
            Some(b)
        } else {
            None
        };
        let pong = if nsteps >= 2 {
            let b = self.alloc_buf(total_pixels)?;
            telemetry.note_alloc(total_pixels);
            Some(b)
        } else {
            None
        };

        let mut launches = 0usize;
        let mut cur_src = in_buf.mem;
        if nsteps > 0 {
            let kst = self.kernel("stencil2d_u8_tiled")?;
            let gx = ((w + 15) / 16) * 16;
            let gy = ((h + 15) / 16) * 16;
            let f_u32 = f as u32;
            let h_u32 = h as u32;
            let w_u32 = w as u32;
            for step in 0..nsteps {
                let dst = if step % 2 == 0 {
                    ping.as_ref().unwrap().mem
                } else {
                    pong.as_ref().unwrap().mem
                };
                self.set_arg_mem(kst, 0, cur_src)?;
                self.set_arg_mem(kst, 1, dst)?;
                self.set_arg_val(kst, 2, &f_u32)?;
                self.set_arg_val(kst, 3, &h_u32)?;
                self.set_arg_val(kst, 4, &w_u32)?;
                self.set_arg_val(kst, 5, &divisor)?;
                self.set_arg_val(kst, 6, &edge_policy)?;
                for (i, wv) in ws.iter().enumerate() {
                    self.set_arg_val(kst, 7 + i as u32, wv)?;
                }
                self.enqueue_3d(kst, [gx, gy, f], [16, 16, 1])?;
                cur_src = dst;
                launches += 1;
            }
        }

        let many_small = f >= 16 || frame_pixels <= 16_384;
        let f_u32 = f as u32;
        let fp_u32 = frame_pixels as u32;
        let bins_u32 = bins as u32;
        let tiles_per_frame = if many_small {
            1u32
        } else {
            ((frame_pixels / (256 * 32)).clamp(1, 256)) as u32
        };
        let hist_groups = f * (tiles_per_frame as usize);

        if many_small {
            let kh = self.kernel("hist_u8_many_small_frames")?;
            self.set_arg_mem(kh, 0, cur_src)?;
            self.set_arg_mem(kh, 1, out_hist.mem)?;
            self.set_arg_val(kh, 2, &f_u32)?;
            self.set_arg_val(kh, 3, &fp_u32)?;
            self.set_arg_val(kh, 4, &bins_u32)?;
            self.enqueue_1d(kh, hist_groups * 256, 256)?;
            launches += 1;
        } else {
            let total_bins = (f * bins) as u32;
            let zgroups = (f * bins + 255) / 256;
            let kz = self.kernel("hist_u8_zero")?;
            self.set_arg_mem(kz, 0, out_hist.mem)?;
            self.set_arg_val(kz, 1, &total_bins)?;
            self.enqueue_1d(kz, zgroups * 256, 256)?;
            launches += 1;

            let kh = self.kernel("hist_u8_few_large_frames")?;
            self.set_arg_mem(kh, 0, cur_src)?;
            self.set_arg_mem(kh, 1, out_hist.mem)?;
            self.set_arg_val(kh, 2, &f_u32)?;
            self.set_arg_val(kh, 3, &fp_u32)?;
            self.set_arg_val(kh, 4, &bins_u32)?;
            self.set_arg_val(kh, 5, &tiles_per_frame)?;
            self.enqueue_1d(kh, hist_groups * 256, 256)?;
            launches += 1;
        }

        self.finish()?;
        if let Some(ref b) = ping {
            telemetry.note_free(b.bytes);
        }
        if let Some(ref b) = pong {
            telemetry.note_free(b.bytes);
        }
        telemetry.kernel_launches += launches;
        telemetry.sync_points.push(SyncPoint {
            op: "stencil-histogram-pipeline".to_string(),
            reason: "opencl-queue-finish".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil({nsteps})-then-histogram-u8"),
            schedule: if many_small {
                format!("many-small-frames(opencl,frames={f},h={h},w={w},hist_groups={hist_groups},wg_privatized)")
            } else {
                format!("few-large-frames(opencl,frames={f},h={h},w={w},tiles_per_frame={tiles_per_frame},hist_groups={hist_groups},wg_privatized)")
            },
            grid: [hist_groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
        };
        Ok(out_hist)
    }

    // -----------------------------------------------------------------------
    // 5. Batched 2D F32 Stencil (ping-pong)
    // -----------------------------------------------------------------------
    #[allow(clippy::too_many_arguments)]
    pub fn stencil2d_f32(
        &mut self,
        in_buf: &OpenClBuffer,
        f: usize,
        h: usize,
        w: usize,
        ws: [f32; 9],
        edge_policy: u32,
        nsteps: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<OpenClBuffer, String> {
        let total = f * h * w;
        let ping = self.alloc_buf(total * 4)?;
        telemetry.note_alloc(total * 4);
        let pong = if nsteps >= 2 {
            let b = self.alloc_buf(total * 4)?;
            telemetry.note_alloc(total * 4);
            Some(b)
        } else {
            None
        };
        let kst = self.kernel("stencil2d_f32_tiled")?;
        let gx = ((w + 15) / 16) * 16;
        let gy = ((h + 15) / 16) * 16;
        let f_u32 = f as u32;
        let h_u32 = h as u32;
        let w_u32 = w as u32;

        let mut cur_src = in_buf.mem;
        let mut final_is_ping = true;
        for step in 0..nsteps {
            let dst = if step % 2 == 0 {
                final_is_ping = true;
                ping.mem
            } else {
                final_is_ping = false;
                pong.as_ref().unwrap().mem
            };
            self.set_arg_mem(kst, 0, cur_src)?;
            self.set_arg_mem(kst, 1, dst)?;
            self.set_arg_val(kst, 2, &f_u32)?;
            self.set_arg_val(kst, 3, &h_u32)?;
            self.set_arg_val(kst, 4, &w_u32)?;
            self.set_arg_val(kst, 5, &edge_policy)?;
            for (i, wv) in ws.iter().enumerate() {
                self.set_arg_val(kst, 6 + i as u32, wv)?;
            }
            self.enqueue_3d(kst, [gx, gy, f], [16, 16, 1])?;
            cur_src = dst;
        }
        self.finish()?;
        let out_buf = if final_is_ping {
            if let Some(ref b) = pong {
                telemetry.note_free(b.bytes);
            }
            ping
        } else {
            telemetry.note_free(ping.bytes);
            pong.unwrap()
        };
        telemetry.kernel_launches += nsteps;
        telemetry.sync_points.push(SyncPoint {
            op: format!("stencil2d-f32-pingpong({nsteps})"),
            reason: "opencl-queue-finish".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil2d-f32(steps={nsteps})"),
            schedule: format!("opencl-tiled-16x16-halo(frames={f},h={h},w={w},pingpong=2bufs)"),
            grid: [gx, gy, f],
            threadgroup: [16, 16, 1],
            threadgroup_mem_bytes: 18 * 18 * 4,
            numerical_mode: "ieee754-f32".to_string(),
        };
        Ok(out_buf)
    }

    // -----------------------------------------------------------------------
    // 6. CSR SpMV (with empty row support & exact-tree / relaxed modes)
    // -----------------------------------------------------------------------
    #[allow(clippy::too_many_arguments)]
    pub fn spmv_csr_f32(
        &mut self,
        vals: &OpenClBuffer,
        cols: &OpenClBuffer,
        row_ptr: &OpenClBuffer,
        x: &OpenClBuffer,
        r: usize,
        nz: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<OpenClBuffer, String> {
        let out_y = self.alloc_buf(r * 4)?;
        telemetry.note_alloc(r * 4);
        if r == 0 {
            return Ok(out_y);
        }
        let avg_nnz = nz / r;
        let (kname, groups, sched) = if exact {
            (
                "spmv_csr_f32_exact",
                (r + 255) / 256,
                format!("opencl-csr-exact-tree(rows={r},nnz={nz})"),
            )
        } else if avg_nnz >= 16 {
            (
                "spmv_csr_f32_wg",
                (r * 32 + 255) / 256,
                format!("opencl-csr-wg-row(rows={r},nnz={nz},avg_nnz={avg_nnz})"),
            )
        } else {
            (
                "spmv_csr_f32_scalar",
                (r + 255) / 256,
                format!("opencl-csr-scalar-row(rows={r},nnz={nz},avg_nnz={avg_nnz})"),
            )
        };
        let k = self.kernel(kname)?;
        let r_u32 = r as u32;
        self.set_arg_mem(k, 0, vals.mem)?;
        self.set_arg_mem(k, 1, cols.mem)?;
        self.set_arg_mem(k, 2, row_ptr.mem)?;
        self.set_arg_mem(k, 3, x.mem)?;
        self.set_arg_mem(k, 4, out_y.mem)?;
        self.set_arg_val(k, 5, &r_u32)?;
        self.enqueue_1d(k, groups * 256, 256)?;
        self.finish()?;
        telemetry.kernel_launches += 1;
        telemetry.sync_points.push(SyncPoint {
            op: "spmv-csr-f32".to_string(),
            reason: "opencl-queue-finish".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: "spmv-csr-f32".to_string(),
            schedule: sched,
            grid: [groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: if !exact && avg_nnz >= 16 { 256 * 4 } else { 0 },
            numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" }.to_string(),
        };
        Ok(out_y)
    }

    // -----------------------------------------------------------------------
    // 7. Adaptive Row Reduction (outer vs inner schedule)
    // -----------------------------------------------------------------------
    pub fn row_reduce_f32(
        &mut self,
        in_buf: &OpenClBuffer,
        r: usize,
        c: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<OpenClBuffer, String> {
        let out_buf = self.alloc_buf(r * 4)?;
        telemetry.note_alloc(r * 4);
        if r == 0 {
            return Ok(out_buf);
        }
        let outer = r >= 128 && c < 2048;
        let (kname, groups, sched) = if outer {
            (
                "row_reduce_f32_outer",
                (r + 255) / 256,
                format!("opencl-outer-parallel(rows={r},cols={c})"),
            )
        } else {
            (
                "row_reduce_f32_inner",
                r,
                format!("opencl-inner-wg-parallel(rows={r},cols={c})"),
            )
        };
        let k = self.kernel(kname)?;
        let r_u32 = r as u32;
        let c_u32 = c as u32;
        self.set_arg_mem(k, 0, in_buf.mem)?;
        self.set_arg_mem(k, 1, out_buf.mem)?;
        self.set_arg_val(k, 2, &r_u32)?;
        self.set_arg_val(k, 3, &c_u32)?;
        self.enqueue_1d(k, groups * 256, 256)?;
        self.finish()?;
        telemetry.kernel_launches += 1;
        telemetry.sync_points.push(SyncPoint {
            op: "row-reduce-f32".to_string(),
            reason: "opencl-queue-finish".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: "row-reduce-f32".to_string(),
            schedule: sched,
            grid: [groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: if outer { 0 } else { 256 * 4 },
            numerical_mode: "f32-add-relaxed".to_string(),
        };
        Ok(out_buf)
    }

    /// Run a compiler-emitted OpenCL C 1D kernel over resident OpenCL buffers.
    pub fn run_source_1d(
        &self,
        source: &str,
        kernel_name: &str,
        bufs: &[&OpenClBuffer],
        n_threads: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<(), String> {
        let a = api().ok_or_else(|| "OpenCL unavailable".to_string())?;
        let mut err: ClInt = 0;
        let ptr = source.as_ptr().cast::<i8>();
        let len = source.len();
        let prog = unsafe { (a.create_program_with_source)(self.context, 1, &ptr, &len, &mut err) };
        if prog.is_null() || err != CL_SUCCESS {
            return Err(format!("clCreateProgramWithSource failed ({err})"));
        }
        let brc = unsafe { (a.build_program)(prog, 1, &self.device, null(), null(), null_mut()) };
        if brc != CL_SUCCESS {
            unsafe { (a.release_program)(prog) };
            return Err(format!("clBuildProgram failed ({brc})"));
        }
        let mut kc = Vec::with_capacity(kernel_name.len() + 1);
        kc.extend_from_slice(kernel_name.as_bytes());
        kc.push(0);
        let k = unsafe { (a.create_kernel)(prog, kc.as_ptr().cast(), &mut err) };
        if k.is_null() || err != CL_SUCCESS {
            unsafe { (a.release_program)(prog) };
            return Err(format!("clCreateKernel('{kernel_name}') failed ({err})"));
        }
        for (i, b) in bufs.iter().enumerate() {
            let r = self.set_arg_mem(k, i as u32, b.mem);
            if let Err(e) = r {
                unsafe {
                    (a.release_kernel)(k);
                    (a.release_program)(prog);
                }
                return Err(e);
            }
        }
        let n_u32 = n_threads as u32;
        let _ = self.set_arg_val(k, bufs.len() as u32, &n_u32);
        let groups = ((n_threads + 255) / 256).max(1);
        let rc = self.enqueue_1d(k, groups * 256, 256).and_then(|_| self.finish());
        unsafe {
            (a.release_kernel)(k);
            (a.release_program)(prog);
        }
        rc?;
        telemetry.kernel_launches += 1;
        telemetry.sync_points.push(SyncPoint {
            op: kernel_name.to_string(),
            reason: "compiled-opencl-kernel-complete".to_string(),
        });
        Ok(())
    }
}
