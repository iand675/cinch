//! AMD ROCm HIP GPU compute backend for Cinch's data-parallel arrays
//! (equivalent to Futhark's `hip` backend).
//!
//! Uses `dlopen`/`dlsym` via [`crate::sys`] to reach the HIP Runtime API
//! (`libamdhip64.so`) and hipRTC (`libhiprtc.so`) with zero external Cargo
//! dependencies and zero extra linker flags.
//!
//! On hosts without AMD ROCm / HIP drivers, `HipState::open` cleanly returns
//! an informative error so `cinch_device_open(DEVICE_KIND_HIP_REQUIRED)`
//! surfaces `DEV_ERR_NO_DEVICE` at the defined error boundary.

use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut};
use std::sync::OnceLock;

use crate::metal::{DeviceLaunchPlan, DeviceTelemetry, SyncPoint};
use crate::par_arrays;

type HipError = i32;
type HipModule = *mut c_void;
type HipFunction = *mut c_void;
type HipStream = *mut c_void;
type HiprtcResult = i32;
type HiprtcProgram = *mut c_void;

const HIP_SUCCESS: HipError = 0;
const HIPRTC_SUCCESS: HiprtcResult = 0;
const HIP_MEMCPY_HOST_TO_DEVICE: i32 = 1;
const HIP_MEMCPY_DEVICE_TO_HOST: i32 = 2;

struct HipApi {
    get_device_count: unsafe extern "C" fn(*mut i32) -> HipError,
    set_device: unsafe extern "C" fn(i32) -> HipError,
    device_get_name: unsafe extern "C" fn(*mut i8, i32, i32) -> HipError,
    malloc: unsafe extern "C" fn(*mut *mut c_void, usize) -> HipError,
    free: unsafe extern "C" fn(*mut c_void) -> HipError,
    memcpy: unsafe extern "C" fn(*mut c_void, *const c_void, usize, i32) -> HipError,
    device_synchronize: unsafe extern "C" fn() -> HipError,
    module_load_data: unsafe extern "C" fn(*mut HipModule, *const c_void) -> HipError,
    module_unload: unsafe extern "C" fn(HipModule) -> HipError,
    module_get_function: unsafe extern "C" fn(*mut HipFunction, HipModule, *const i8) -> HipError,
    module_launch_kernel: unsafe extern "C" fn(
        HipFunction,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        HipStream,
        *mut *mut c_void,
        *mut *mut c_void,
    ) -> HipError,
    hiprtc_create_program: unsafe extern "C" fn(
        *mut HiprtcProgram,
        *const i8,
        *const i8,
        i32,
        *const *const i8,
        *const *const i8,
    ) -> HiprtcResult,
    hiprtc_compile_program:
        unsafe extern "C" fn(HiprtcProgram, i32, *const *const i8) -> HiprtcResult,
    hiprtc_get_code_size: unsafe extern "C" fn(HiprtcProgram, *mut usize) -> HiprtcResult,
    hiprtc_get_code: unsafe extern "C" fn(HiprtcProgram, *mut i8) -> HiprtcResult,
    hiprtc_get_program_log_size: unsafe extern "C" fn(HiprtcProgram, *mut usize) -> HiprtcResult,
    hiprtc_get_program_log: unsafe extern "C" fn(HiprtcProgram, *mut i8) -> HiprtcResult,
    hiprtc_destroy_program: unsafe extern "C" fn(*mut HiprtcProgram) -> HiprtcResult,
}

unsafe impl Send for HipApi {}
unsafe impl Sync for HipApi {}

static HIP_API: OnceLock<Option<HipApi>> = OnceLock::new();

fn api() -> Option<&'static HipApi> {
    HIP_API
        .get_or_init(|| unsafe {
            let mut hip_lib = null_mut();
            for p in &[
                b"libamdhip64.so.6".as_slice(),
                b"libamdhip64.so.5",
                b"libamdhip64.so",
                b"/opt/rocm/lib/libamdhip64.so",
            ] {
                hip_lib = crate::sys::dlopen(p);
                if !hip_lib.is_null() {
                    break;
                }
            }
            let mut rtc_lib = null_mut();
            for p in &[
                b"libhiprtc.so.6".as_slice(),
                b"libhiprtc.so.5",
                b"libhiprtc.so",
                b"/opt/rocm/lib/libhiprtc.so",
            ] {
                rtc_lib = crate::sys::dlopen(p);
                if !rtc_lib.is_null() {
                    break;
                }
            }
            if hip_lib.is_null() || rtc_lib.is_null() {
                return None;
            }
            macro_rules! load {
                ($lib:expr, $sym:literal) => {{
                    let p = crate::sys::dlsym($lib, $sym);
                    if p.is_null() {
                        return None;
                    }
                    std::mem::transmute(p)
                }};
            }
            Some(HipApi {
                get_device_count: load!(hip_lib, b"hipGetDeviceCount"),
                set_device: load!(hip_lib, b"hipSetDevice"),
                device_get_name: load!(hip_lib, b"hipDeviceGetName"),
                malloc: load!(hip_lib, b"hipMalloc"),
                free: load!(hip_lib, b"hipFree"),
                memcpy: load!(hip_lib, b"hipMemcpy"),
                device_synchronize: load!(hip_lib, b"hipDeviceSynchronize"),
                module_load_data: load!(hip_lib, b"hipModuleLoadData"),
                module_unload: load!(hip_lib, b"hipModuleUnload"),
                module_get_function: load!(hip_lib, b"hipModuleGetFunction"),
                module_launch_kernel: load!(hip_lib, b"hipModuleLaunchKernel"),
                hiprtc_create_program: load!(rtc_lib, b"hiprtcCreateProgram"),
                hiprtc_compile_program: load!(rtc_lib, b"hiprtcCompileProgram"),
                hiprtc_get_code_size: load!(rtc_lib, b"hiprtcGetCodeSize"),
                hiprtc_get_code: load!(rtc_lib, b"hiprtcGetCode"),
                hiprtc_get_program_log_size: load!(rtc_lib, b"hiprtcGetProgramLogSize"),
                hiprtc_get_program_log: load!(rtc_lib, b"hiprtcGetProgramLog"),
                hiprtc_destroy_program: load!(rtc_lib, b"hiprtcDestroyProgram"),
            })
        })
        .as_ref()
}

pub fn is_available() -> bool {
    api().is_some() && HipState::open().is_ok()
}

// ---------------------------------------------------------------------------
// Built-in Production AMD ROCm HIP C++ Kernels (wavefront-portable)
// ---------------------------------------------------------------------------

pub const BUILTIN_HIP_SOURCE: &str = r#"
#include <hip/hip_runtime.h>

extern "C" {

// 1. Fused Dot Product (Wavefront-portable LDS Relaxed & Exact-Tree 1024-element blocks)

__global__ void dot_f32_relaxed(
    const float* __restrict__ a,
    const float* __restrict__ b,
    float* __restrict__ partials,
    unsigned int n
) {
    __shared__ float s[256];
    unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    unsigned int gsize = hipGridDim_x * hipBlockDim_x;
    unsigned int tid = hipThreadIdx_x;
    float acc = 0.0f;
    for (unsigned int i = gid; i < n; i += gsize) {
        acc += a[i] * b[i];
    }
    s[tid] = acc;
    __syncthreads();
    for (unsigned int stride = 128u; stride > 0u; stride >>= 1) {
        if (tid < stride) s[tid] += s[tid + stride];
        __syncthreads();
    }
    if (tid == 0u) partials[hipBlockIdx_x] = s[0];
}

__global__ void dot_f32_exact_block(
    const float* __restrict__ a,
    const float* __restrict__ b,
    float* __restrict__ block_out,
    unsigned int n
) {
    __shared__ float s[1024];
    unsigned int tid = hipThreadIdx_x;
    unsigned int base = hipBlockIdx_x * 1024u;
    for (unsigned int k = 0; k < 4u; ++k) {
        unsigned int idx = tid + k * 256u;
        unsigned int g = base + idx;
        s[idx] = (g < n) ? (a[g] * b[g]) : 0.0f;
    }
    __syncthreads();
    unsigned int len = (base < n) ? ((n - base < 1024u) ? (n - base) : 1024u) : 0u;
    if (len == 1024u) {
        for (unsigned int stride = 1u; stride < 1024u; stride <<= 1) {
            for (unsigned int k = 0; k < 4u; ++k) {
                unsigned int idx = tid + k * 256u;
                if ((idx & ((stride << 1) - 1u)) == 0u) s[idx] = s[idx] + s[idx + stride];
            }
            __syncthreads();
        }
        if (tid == 0u) block_out[hipBlockIdx_x] = s[0];
    } else if (tid == 0u) {
        float vals[12];
        unsigned int sizes[12];
        unsigned int sp = 0u;
        for (unsigned int i = 0; i < len; ++i) {
            float cur = s[i];
            unsigned int sz = 1u;
            while (sp > 0u && sizes[sp - 1] == sz) {
                cur = vals[sp - 1] + cur;
                sz <<= 1;
                sp -= 1u;
            }
            vals[sp] = cur;
            sizes[sp] = sz;
            sp += 1u;
        }
        float acc = 0.0f;
        if (sp > 0u) {
            acc = vals[sp - 1];
            for (int j = (int)sp - 2; j >= 0; --j) acc = vals[j] + acc;
        }
        block_out[hipBlockIdx_x] = acc;
    }
}

// 2. Prefix Scan

__global__ void scan_i32_block(
    const int* __restrict__ in_arr, int* __restrict__ out_arr,
    int* __restrict__ block_sums, unsigned int n, unsigned int inclusive
) {
    __shared__ int s[256];
    unsigned int tid = hipThreadIdx_x;
    unsigned int gid = hipBlockIdx_x * 256u + tid;
    int val = (gid < n) ? in_arr[gid] : 0;
    s[tid] = val;
    __syncthreads();
    for (unsigned int offset = 1u; offset < 256u; offset <<= 1) {
        int add = (tid >= offset) ? s[tid - offset] : 0;
        __syncthreads();
        s[tid] += add;
        __syncthreads();
    }
    if (tid == 255u) block_sums[hipBlockIdx_x] = s[255];
    if (gid < n) out_arr[gid] = (inclusive != 0u) ? s[tid] : (s[tid] - val);
}

__global__ void scan_i32_add_offsets(
    int* __restrict__ out_arr, const int* __restrict__ block_offsets, unsigned int n
) {
    unsigned int gid = hipBlockIdx_x * 256u + hipThreadIdx_x;
    if (gid < n) out_arr[gid] += block_offsets[hipBlockIdx_x];
}

// 3. Stable Compaction

__global__ void filter_i32_gt_flags(
    const int* __restrict__ in_arr, int* __restrict__ flags, unsigned int n, int threshold
) {
    unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (gid < n) flags[gid] = (in_arr[gid] > threshold) ? 1 : 0;
}

__global__ void filter_i32_gt_scatter(
    const int* __restrict__ in_arr, const int* __restrict__ flags,
    const int* __restrict__ positions, int* __restrict__ out_arr, unsigned int n
) {
    unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (gid < n && flags[gid] != 0) out_arr[positions[gid]] = in_arr[gid];
}

// 4. Batched Histogram (LDS privatized, adaptive many-small vs few-large)

__global__ void hist_u8_zero(unsigned int* out_hist, unsigned int total) {
    unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (gid < total) out_hist[gid] = 0u;
}

__global__ void hist_u8_many_small_frames(
    const unsigned char* __restrict__ pixels, unsigned int* __restrict__ out_hist,
    unsigned int frames, unsigned int frame_pixels, unsigned int num_bins
) {
    unsigned int frame = hipBlockIdx_x;
    unsigned int tid = hipThreadIdx_x;
    if (frame >= frames) return;
    __shared__ unsigned int local_bins[256];
    local_bins[tid] = 0u;
    __syncthreads();
    unsigned int base = frame * frame_pixels;
    for (unsigned int i = tid; i < frame_pixels; i += 256u) {
        unsigned int b = (unsigned int)pixels[base + i];
        if (b < num_bins) atomicAdd(&local_bins[b], 1u);
    }
    __syncthreads();
    if (tid < num_bins) out_hist[frame * num_bins + tid] = local_bins[tid];
}

__global__ void hist_u8_few_large_frames(
    const unsigned char* __restrict__ pixels, unsigned int* __restrict__ out_hist,
    unsigned int frames, unsigned int frame_pixels, unsigned int num_bins,
    unsigned int tiles_per_frame
) {
    unsigned int frame = hipBlockIdx_x / tiles_per_frame;
    unsigned int tile = hipBlockIdx_x % tiles_per_frame;
    unsigned int tid = hipThreadIdx_x;
    if (frame >= frames) return;
    __shared__ unsigned int local_bins[256];
    local_bins[tid] = 0u;
    __syncthreads();
    unsigned int base = frame * frame_pixels;
    unsigned int stride = tiles_per_frame * 256u;
    for (unsigned int i = tile * 256u + tid; i < frame_pixels; i += stride) {
        unsigned int b = (unsigned int)pixels[base + i];
        if (b < num_bins) atomicAdd(&local_bins[b], 1u);
    }
    __syncthreads();
    if (tid < num_bins) {
        unsigned int c = local_bins[tid];
        if (c > 0u) atomicAdd(&out_hist[frame * num_bins + tid], c);
    }
}

// 5. Batched 2D 3x3 Stencil (U8 & F32) with 18x18 LDS halo tile

__device__ __forceinline__ int resolve_coord(int idx, int dim, unsigned int policy) {
    if (idx >= 0 && idx < dim) return idx;
    if (policy == 0u) return (idx < 0) ? 0 : (dim - 1);
    if (policy == 1u) return -1;
    if (policy == 2u) {
        if (dim <= 1) return 0;
        int period = 2 * dim - 2;
        int m = ((idx % period) + period) % period;
        return (m >= dim) ? (period - m) : m;
    }
    return ((idx % dim) + dim) % dim;
}

__global__ void stencil2d_u8_tiled(
    const unsigned char* __restrict__ src, unsigned char* __restrict__ dst,
    unsigned int frames, unsigned int height, unsigned int width,
    int divisor, unsigned int edge_policy,
    int w0, int w1, int w2, int w3, int w4, int w5, int w6, int w7, int w8
) {
    unsigned int gx = hipBlockIdx_x * 16u + hipThreadIdx_x;
    unsigned int gy = hipBlockIdx_y * 16u + hipThreadIdx_y;
    unsigned int frame = hipBlockIdx_z;
    if (frame >= frames) return;
    int h = (int)height, w = (int)width;
    unsigned int fbase = frame * height * width;
    __shared__ unsigned char tile[18][18];
    int tile_x0 = (int)hipBlockIdx_x * 16 - 1;
    int tile_y0 = (int)hipBlockIdx_y * 16 - 1;
    unsigned int flat_tid = hipThreadIdx_y * 16u + hipThreadIdx_x;
    for (unsigned int k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u), tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0 : src[fbase + (unsigned int)(ry * w + rx)];
    }
    __syncthreads();
    if (gx >= width || gy >= height) return;
    unsigned int ly = hipThreadIdx_y + 1u, lx = hipThreadIdx_x + 1u;
    int acc = w0 * (int)tile[ly - 1][lx - 1] + w1 * (int)tile[ly - 1][lx] + w2 * (int)tile[ly - 1][lx + 1]
            + w3 * (int)tile[ly][lx - 1]     + w4 * (int)tile[ly][lx]     + w5 * (int)tile[ly][lx + 1]
            + w6 * (int)tile[ly + 1][lx - 1] + w7 * (int)tile[ly + 1][lx] + w8 * (int)tile[ly + 1][lx + 1];
    int q = acc / ((divisor == 0) ? 1 : divisor);
    q = (q < 0) ? 0 : ((q > 255) ? 255 : q);
    dst[fbase + gy * width + gx] = (unsigned char)q;
}

__global__ void stencil2d_f32_tiled(
    const float* __restrict__ src, float* __restrict__ dst,
    unsigned int frames, unsigned int height, unsigned int width,
    unsigned int edge_policy,
    float w0, float w1, float w2, float w3, float w4, float w5, float w6, float w7, float w8
) {
    unsigned int gx = hipBlockIdx_x * 16u + hipThreadIdx_x;
    unsigned int gy = hipBlockIdx_y * 16u + hipThreadIdx_y;
    unsigned int frame = hipBlockIdx_z;
    if (frame >= frames) return;
    int h = (int)height, w = (int)width;
    unsigned int fbase = frame * height * width;
    __shared__ float tile[18][18];
    int tile_x0 = (int)hipBlockIdx_x * 16 - 1;
    int tile_y0 = (int)hipBlockIdx_y * 16 - 1;
    unsigned int flat_tid = hipThreadIdx_y * 16u + hipThreadIdx_x;
    for (unsigned int k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u), tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0.0f : src[fbase + (unsigned int)(ry * w + rx)];
    }
    __syncthreads();
    if (gx >= width || gy >= height) return;
    unsigned int ly = hipThreadIdx_y + 1u, lx = hipThreadIdx_x + 1u;
    dst[fbase + gy * width + gx] =
        w0 * tile[ly - 1][lx - 1] + w1 * tile[ly - 1][lx] + w2 * tile[ly - 1][lx + 1]
      + w3 * tile[ly][lx - 1]     + w4 * tile[ly][lx]     + w5 * tile[ly][lx + 1]
      + w6 * tile[ly + 1][lx - 1] + w7 * tile[ly + 1][lx] + w8 * tile[ly + 1][lx + 1];
}

// 6. Adaptive Row Reduction

__global__ void row_reduce_f32_outer(
    const float* __restrict__ a, float* __restrict__ out, unsigned int rows, unsigned int cols
) {
    unsigned int row = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (row >= rows) return;
    float acc = 0.0f;
    unsigned int base = row * cols;
    for (unsigned int j = 0; j < cols; ++j) acc += a[base + j];
    out[row] = acc;
}

__global__ void row_reduce_f32_inner(
    const float* __restrict__ a, float* __restrict__ out, unsigned int rows, unsigned int cols
) {
    __shared__ float s[256];
    unsigned int row = hipBlockIdx_x;
    unsigned int tid = hipThreadIdx_x;
    if (row >= rows) return;
    float acc = 0.0f;
    unsigned int base = row * cols;
    for (unsigned int j = tid; j < cols; j += 256u) acc += a[base + j];
    s[tid] = acc;
    __syncthreads();
    for (unsigned int stride = 128u; stride > 0u; stride >>= 1) {
        if (tid < stride) s[tid] += s[tid + stride];
        __syncthreads();
    }
    if (tid == 0u) out[row] = s[0];
}

// 7. CSR SpMV (scalar, workgroup-32, exact-tree with empty rows)

__global__ void spmv_csr_f32_scalar(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    unsigned int row = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (row >= nrows) return;
    unsigned int s = row_ptr[row], e = row_ptr[row + 1];
    float acc = 0.0f;
    for (unsigned int k = s; k < e; ++k) acc += vals[k] * x[cols[k]];
    y[row] = acc;
}

__global__ void spmv_csr_f32_wg(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    __shared__ float s[256];
    unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    unsigned int tid = hipThreadIdx_x;
    unsigned int lane = tid & 31u;
    unsigned int row = gid >> 5;
    float acc = 0.0f;
    if (row < nrows) {
        unsigned int st = row_ptr[row], en = row_ptr[row + 1];
        for (unsigned int k = st + lane; k < en; k += 32u) acc += vals[k] * x[cols[k]];
    }
    s[tid] = acc;
    __syncthreads();
    for (unsigned int stride = 16u; stride > 0u; stride >>= 1) {
        if (lane < stride) s[tid] += s[tid + stride];
        __syncthreads();
    }
    if (lane == 0u && row < nrows) y[row] = s[tid];
}

__global__ void spmv_csr_f32_exact(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    unsigned int row = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x;
    if (row >= nrows) return;
    unsigned int s = row_ptr[row], e = row_ptr[row + 1];
    if (e <= s) { y[row] = 0.0f; return; }
    float st_val[32];
    unsigned int st_sz[32];
    unsigned int sp = 0u;
    for (unsigned int k = s; k < e; ++k) {
        float cur = vals[k] * x[cols[k]];
        unsigned int sz = 1u;
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
    for (int j = (int)sp - 2; j >= 0; --j) acc = st_val[j] + acc;
    y[row] = acc;
}

} // extern "C"
"#;

// ---------------------------------------------------------------------------
// HIP State & Resident Buffer
// ---------------------------------------------------------------------------

pub struct HipBuffer {
    pub ptr: *mut c_void,
    pub bytes: usize,
}

unsafe impl Send for HipBuffer {}
unsafe impl Sync for HipBuffer {}

impl Drop for HipBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            if let Some(a) = api() {
                unsafe { (a.free)(self.ptr) };
            }
        }
    }
}

pub struct HipState {
    pub device_name: String,
    module: HipModule,
    funcs: HashMap<String, HipFunction>,
}

unsafe impl Send for HipState {}
unsafe impl Sync for HipState {}

impl Drop for HipState {
    fn drop(&mut self) {
        if let Some(a) = api() {
            if !self.module.is_null() {
                unsafe { (a.module_unload)(self.module) };
            }
        }
    }
}

impl HipState {
    pub fn open() -> Result<Self, String> {
        let a = api().ok_or_else(|| "AMD ROCm HIP runtime (libamdhip64 + libhiprtc) not available on this host".to_string())?;
        let mut count = 0i32;
        let rc = unsafe { (a.get_device_count)(&mut count) };
        if rc != HIP_SUCCESS || count <= 0 {
            return Err(format!("no ROCm HIP GPU devices found (rc={rc}, count={count})"));
        }
        let rc = unsafe { (a.set_device)(0) };
        if rc != HIP_SUCCESS {
            return Err(format!("hipSetDevice(0) failed ({rc})"));
        }
        let mut name_buf = [0i8; 256];
        let _ = unsafe { (a.device_get_name)(name_buf.as_mut_ptr(), 256, 0) };
        let device_name = unsafe { CStr::from_ptr(name_buf.as_ptr()) }
            .to_string_lossy()
            .trim()
            .to_string();

        let mut prog: HiprtcProgram = null_mut();
        let src_c = std::ffi::CString::new(BUILTIN_HIP_SOURCE).unwrap();
        let name_c = std::ffi::CString::new("cinch_builtin.hip").unwrap();
        let rc = unsafe {
            (a.hiprtc_create_program)(&mut prog, src_c.as_ptr(), name_c.as_ptr(), 0, null(), null())
        };
        if rc != HIPRTC_SUCCESS {
            return Err(format!("hiprtcCreateProgram failed ({rc})"));
        }
        let comp_rc = unsafe { (a.hiprtc_compile_program)(prog, 0, null()) };
        if comp_rc != HIPRTC_SUCCESS {
            let mut log_sz = 0usize;
            unsafe { (a.hiprtc_get_program_log_size)(prog, &mut log_sz) };
            let mut log = vec![0u8; log_sz.max(1)];
            unsafe {
                (a.hiprtc_get_program_log)(prog, log.as_mut_ptr().cast());
                (a.hiprtc_destroy_program)(&mut prog);
            }
            return Err(format!(
                "hiprtcCompileProgram failed ({comp_rc}): {}",
                String::from_utf8_lossy(&log)
            ));
        }
        let mut code_sz = 0usize;
        unsafe { (a.hiprtc_get_code_size)(prog, &mut code_sz) };
        let mut code = vec![0u8; code_sz.max(1)];
        unsafe {
            (a.hiprtc_get_code)(prog, code.as_mut_ptr().cast());
            (a.hiprtc_destroy_program)(&mut prog);
        }
        let mut module: HipModule = null_mut();
        let rc = unsafe { (a.module_load_data)(&mut module, code.as_ptr().cast()) };
        if rc != HIP_SUCCESS || module.is_null() {
            return Err(format!("hipModuleLoadData failed ({rc})"));
        }
        Ok(Self {
            device_name,
            module,
            funcs: HashMap::new(),
        })
    }

    fn func(&mut self, name: &str) -> Result<HipFunction, String> {
        if let Some(&f) = self.funcs.get(name) {
            return Ok(f);
        }
        let a = api().unwrap();
        let cname = std::ffi::CString::new(name).unwrap();
        let mut f: HipFunction = null_mut();
        let rc = unsafe { (a.module_get_function)(&mut f, self.module, cname.as_ptr()) };
        if rc != HIP_SUCCESS || f.is_null() {
            return Err(format!("hipModuleGetFunction('{name}') failed ({rc})"));
        }
        self.funcs.insert(name.to_string(), f);
        Ok(f)
    }

    pub fn alloc_buf(&self, bytes: usize) -> Result<HipBuffer, String> {
        let a = api().unwrap();
        let mut ptr: *mut c_void = null_mut();
        let rc = unsafe { (a.malloc)(&mut ptr, bytes.max(4)) };
        if rc != HIP_SUCCESS {
            return Err(format!("hipMalloc({bytes}) failed ({rc})"));
        }
        Ok(HipBuffer { ptr, bytes })
    }

    pub fn upload(&self, src: *const u8, bytes: usize) -> Result<HipBuffer, String> {
        let a = api().unwrap();
        let buf = self.alloc_buf(bytes)?;
        if bytes > 0 && !src.is_null() {
            let rc = unsafe { (a.memcpy)(buf.ptr, src.cast(), bytes, HIP_MEMCPY_HOST_TO_DEVICE) };
            if rc != HIP_SUCCESS {
                return Err(format!("hipMemcpy(HtoD) failed ({rc})"));
            }
        }
        Ok(buf)
    }

    pub fn download(&self, buf: &HipBuffer, dst: *mut u8, bytes: usize) -> Result<(), String> {
        let a = api().unwrap();
        if bytes > 0 && !dst.is_null() {
            let rc = unsafe { (a.memcpy)(dst.cast(), buf.ptr, bytes, HIP_MEMCPY_DEVICE_TO_HOST) };
            if rc != HIP_SUCCESS {
                return Err(format!("hipMemcpy(DtoH) failed ({rc})"));
            }
        }
        Ok(())
    }

    fn launch(
        &self,
        f: HipFunction,
        grid: [u32; 3],
        block: [u32; 3],
        params: &mut [*mut c_void],
    ) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe {
            (a.module_launch_kernel)(
                f,
                grid[0],
                grid[1],
                grid[2],
                block[0],
                block[1],
                block[2],
                0,
                null_mut(),
                params.as_mut_ptr(),
                null_mut(),
            )
        };
        if rc != HIP_SUCCESS {
            return Err(format!("hipModuleLaunchKernel failed ({rc})"));
        }
        Ok(())
    }

    fn sync(&self) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe { (a.device_synchronize)() };
        if rc != HIP_SUCCESS {
            return Err(format!("hipDeviceSynchronize failed ({rc})"));
        }
        Ok(())
    }

    pub fn dot_f32(
        &mut self,
        a_buf: &HipBuffer,
        b_buf: &HipBuffer,
        n: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<f32, String> {
        if n == 0 {
            return Ok(0.0);
        }
        let mut n_u32 = n as u32;
        let mut pa = a_buf.ptr;
        let mut pb = b_buf.ptr;
        if exact {
            let nblocks = (n + 1023) / 1024;
            let partial = self.alloc_buf(nblocks * 4)?;
            telemetry.note_alloc(nblocks * 4);
            let mut pp = partial.ptr;
            let f = self.func("dot_f32_exact_block")?;
            let mut args = [
                (&mut pa as *mut *mut c_void).cast(),
                (&mut pb as *mut *mut c_void).cast(),
                (&mut pp as *mut *mut c_void).cast(),
                (&mut n_u32 as *mut u32).cast(),
            ];
            self.launch(f, [nblocks as u32, 1, 1], [256, 1, 1], &mut args)?;
            self.sync()?;
            let mut h = vec![0.0f32; nblocks];
            self.download(&partial, h.as_mut_ptr().cast(), nblocks * 4)?;
            telemetry.note_free(nblocks * 4);
            telemetry.kernel_launches += 1;
            telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-exact-tree".to_string(),
                reason: "hip-device-sync".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-exact-tree".to_string(),
                schedule: format!("hip-exact-tree-1024(blocks={nblocks})"),
                grid: [nblocks * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 1024 * 4,
                numerical_mode: "f32-add-exact-tree".to_string(),
            };
            Ok(par_arrays::f32_exact_tree_sum(&h))
        } else {
            let ngroups = ((n + 255) / 256).clamp(1, 1024);
            let partial = self.alloc_buf(ngroups * 4)?;
            telemetry.note_alloc(ngroups * 4);
            let mut pp = partial.ptr;
            let f = self.func("dot_f32_relaxed")?;
            let mut args = [
                (&mut pa as *mut *mut c_void).cast(),
                (&mut pb as *mut *mut c_void).cast(),
                (&mut pp as *mut *mut c_void).cast(),
                (&mut n_u32 as *mut u32).cast(),
            ];
            self.launch(f, [ngroups as u32, 1, 1], [256, 1, 1], &mut args)?;
            self.sync()?;
            let mut h = vec![0.0f32; ngroups];
            self.download(&partial, h.as_mut_ptr().cast(), ngroups * 4)?;
            telemetry.note_free(ngroups * 4);
            telemetry.kernel_launches += 1;
            telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-relaxed".to_string(),
                reason: "hip-device-sync".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-relaxed".to_string(),
                schedule: format!("hip-lds-reduction(groups={ngroups})"),
                grid: [ngroups * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 256 * 4,
                numerical_mode: "f32-add-relaxed".to_string(),
            };
            Ok(h.into_iter().fold(0.0f32, |a, b| a + b))
        }
    }

    pub fn scan_i32(
        &mut self,
        in_buf: &HipBuffer,
        n: usize,
        inclusive: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<HipBuffer, String> {
        let out_buf = self.alloc_buf(n * 4)?;
        telemetry.note_alloc(n * 4);
        if n == 0 {
            return Ok(out_buf);
        }
        let nblocks = (n + 255) / 256;
        let block_sums = self.alloc_buf(nblocks * 4)?;
        telemetry.note_alloc(nblocks * 4);
        let mut pin = in_buf.ptr;
        let mut pout = out_buf.ptr;
        let mut pbs = block_sums.ptr;
        let mut n_u32 = n as u32;
        let mut inc_u32 = u32::from(inclusive);
        let f1 = self.func("scan_i32_block")?;
        let mut a1 = [
            (&mut pin as *mut *mut c_void).cast(),
            (&mut pout as *mut *mut c_void).cast(),
            (&mut pbs as *mut *mut c_void).cast(),
            (&mut n_u32 as *mut u32).cast(),
            (&mut inc_u32 as *mut u32).cast(),
        ];
        self.launch(f1, [nblocks as u32, 1, 1], [256, 1, 1], &mut a1)?;
        let mut launches = 1usize;
        if nblocks > 1 {
            self.sync()?;
            let mut h = vec![0i32; nblocks];
            self.download(&block_sums, h.as_mut_ptr().cast(), nblocks * 4)?;
            let mut acc = 0i32;
            for v in &mut h {
                let x = *v;
                *v = acc;
                acc = acc.wrapping_add(x);
            }
            let offs = self.upload(h.as_ptr().cast(), nblocks * 4)?;
            let mut poffs = offs.ptr;
            let f2 = self.func("scan_i32_add_offsets")?;
            let mut a2 = [
                (&mut pout as *mut *mut c_void).cast(),
                (&mut poffs as *mut *mut c_void).cast(),
                (&mut n_u32 as *mut u32).cast(),
            ];
            self.launch(f2, [nblocks as u32, 1, 1], [256, 1, 1], &mut a2)?;
            launches += 1;
        }
        self.sync()?;
        telemetry.note_free(nblocks * 4);
        telemetry.kernel_launches += launches;
        Ok(out_buf)
    }

    pub fn filter_i32_gt(
        &mut self,
        in_buf: &HipBuffer,
        n: usize,
        threshold: i32,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<(HipBuffer, usize), String> {
        if n == 0 {
            return Ok((self.alloc_buf(0)?, 0));
        }
        let flags = self.alloc_buf(n * 4)?;
        telemetry.note_alloc(n * 4);
        let nblocks = (n + 255) / 256;
        let mut pin = in_buf.ptr;
        let mut pflags = flags.ptr;
        let mut n_u32 = n as u32;
        let mut thr = threshold;
        let kf = self.func("filter_i32_gt_flags")?;
        let mut af = [
            (&mut pin as *mut *mut c_void).cast(),
            (&mut pflags as *mut *mut c_void).cast(),
            (&mut n_u32 as *mut u32).cast(),
            (&mut thr as *mut i32).cast(),
        ];
        self.launch(kf, [nblocks as u32, 1, 1], [256, 1, 1], &mut af)?;
        let pos = self.scan_i32(&flags, n, false, telemetry)?;
        let mut last_f = 0i32;
        let mut last_p = 0i32;
        let a = api().unwrap();
        unsafe {
            (a.memcpy)(
                (&mut last_f as *mut i32).cast(),
                flags.ptr.cast::<u8>().add((n - 1) * 4).cast(),
                4,
                HIP_MEMCPY_DEVICE_TO_HOST,
            );
            (a.memcpy)(
                (&mut last_p as *mut i32).cast(),
                pos.ptr.cast::<u8>().add((n - 1) * 4).cast(),
                4,
                HIP_MEMCPY_DEVICE_TO_HOST,
            );
        }
        let k = (last_p + last_f).max(0) as usize;
        let out_buf = self.alloc_buf(k * 4)?;
        telemetry.note_alloc(k * 4);
        if k > 0 {
            let mut ppos = pos.ptr;
            let mut pout = out_buf.ptr;
            let ks = self.func("filter_i32_gt_scatter")?;
            let mut as_ = [
                (&mut pin as *mut *mut c_void).cast(),
                (&mut pflags as *mut *mut c_void).cast(),
                (&mut ppos as *mut *mut c_void).cast(),
                (&mut pout as *mut *mut c_void).cast(),
                (&mut n_u32 as *mut u32).cast(),
            ];
            self.launch(ks, [nblocks as u32, 1, 1], [256, 1, 1], &mut as_)?;
            self.sync()?;
        }
        telemetry.note_free(flags.bytes);
        telemetry.note_free(pos.bytes);
        telemetry.kernel_launches += 2;
        Ok((out_buf, k))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stencil_histogram_u8(
        &mut self,
        in_buf: &HipBuffer,
        f: usize,
        h: usize,
        w: usize,
        mut ws: [i32; 9],
        mut divisor: i32,
        mut edge_policy: u32,
        nsteps: usize,
        bins: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<HipBuffer, String> {
        let frame_pixels = h * w;
        let total = f * frame_pixels;
        let out_hist = self.alloc_buf(f * bins * 4)?;
        telemetry.note_alloc(f * bins * 4);
        if f == 0 || h == 0 || w == 0 || bins == 0 {
            return Ok(out_hist);
        }
        let ping = if nsteps >= 1 {
            let b = self.alloc_buf(total)?;
            telemetry.note_alloc(total);
            Some(b)
        } else {
            None
        };
        let pong = if nsteps >= 2 {
            let b = self.alloc_buf(total)?;
            telemetry.note_alloc(total);
            Some(b)
        } else {
            None
        };
        let mut launches = 0usize;
        let mut cur_src = in_buf.ptr;
        let mut f_u32 = f as u32;
        let mut h_u32 = h as u32;
        let mut w_u32 = w as u32;
        if nsteps > 0 {
            let kst = self.func("stencil2d_u8_tiled")?;
            let gx = ((w + 15) / 16) as u32;
            let gy = ((h + 15) / 16) as u32;
            for step in 0..nsteps {
                let mut dst = if step % 2 == 0 {
                    ping.as_ref().unwrap().ptr
                } else {
                    pong.as_ref().unwrap().ptr
                };
                let mut args = [
                    (&mut cur_src as *mut *mut c_void).cast(),
                    (&mut dst as *mut *mut c_void).cast(),
                    (&mut f_u32 as *mut u32).cast(),
                    (&mut h_u32 as *mut u32).cast(),
                    (&mut w_u32 as *mut u32).cast(),
                    (&mut divisor as *mut i32).cast(),
                    (&mut edge_policy as *mut u32).cast(),
                    (&mut ws[0] as *mut i32).cast(),
                    (&mut ws[1] as *mut i32).cast(),
                    (&mut ws[2] as *mut i32).cast(),
                    (&mut ws[3] as *mut i32).cast(),
                    (&mut ws[4] as *mut i32).cast(),
                    (&mut ws[5] as *mut i32).cast(),
                    (&mut ws[6] as *mut i32).cast(),
                    (&mut ws[7] as *mut i32).cast(),
                    (&mut ws[8] as *mut i32).cast(),
                ];
                self.launch(kst, [gx, gy, f_u32], [16, 16, 1], &mut args)?;
                cur_src = dst;
                launches += 1;
            }
        }
        let many_small = f >= 16 || frame_pixels <= 16_384;
        let mut fp_u32 = frame_pixels as u32;
        let mut bins_u32 = bins as u32;
        let mut tiles_per_frame = if many_small {
            1u32
        } else {
            ((frame_pixels / (256 * 32)).clamp(1, 256)) as u32
        };
        let hist_groups = f * (tiles_per_frame as usize);
        let mut pout = out_hist.ptr;
        if many_small {
            let kh = self.func("hist_u8_many_small_frames")?;
            let mut args = [
                (&mut cur_src as *mut *mut c_void).cast(),
                (&mut pout as *mut *mut c_void).cast(),
                (&mut f_u32 as *mut u32).cast(),
                (&mut fp_u32 as *mut u32).cast(),
                (&mut bins_u32 as *mut u32).cast(),
            ];
            self.launch(kh, [hist_groups as u32, 1, 1], [256, 1, 1], &mut args)?;
            launches += 1;
        } else {
            let mut total_bins = (f * bins) as u32;
            let kz = self.func("hist_u8_zero")?;
            let mut az = [(&mut pout as *mut *mut c_void).cast(), (&mut total_bins as *mut u32).cast()];
            self.launch(kz, [((f * bins + 255) / 256) as u32, 1, 1], [256, 1, 1], &mut az)?;
            let kh = self.func("hist_u8_few_large_frames")?;
            let mut ah = [
                (&mut cur_src as *mut *mut c_void).cast(),
                (&mut pout as *mut *mut c_void).cast(),
                (&mut f_u32 as *mut u32).cast(),
                (&mut fp_u32 as *mut u32).cast(),
                (&mut bins_u32 as *mut u32).cast(),
                (&mut tiles_per_frame as *mut u32).cast(),
            ];
            self.launch(kh, [hist_groups as u32, 1, 1], [256, 1, 1], &mut ah)?;
            launches += 2;
        }
        self.sync()?;
        if let Some(ref b) = ping {
            telemetry.note_free(b.bytes);
        }
        if let Some(ref b) = pong {
            telemetry.note_free(b.bytes);
        }
        telemetry.kernel_launches += launches;
        telemetry.sync_points.push(SyncPoint {
            op: "stencil-histogram-pipeline".to_string(),
            reason: "hip-device-sync".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil({nsteps})-then-histogram-u8"),
            schedule: if many_small {
                format!("many-small-frames(hip,frames={f},h={h},w={w},hist_groups={hist_groups})")
            } else {
                format!("few-large-frames(hip,frames={f},h={h},w={w},tiles_per_frame={tiles_per_frame})")
            },
            grid: [hist_groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
        };
        Ok(out_hist)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stencil2d_f32(
        &mut self,
        in_buf: &HipBuffer,
        f: usize,
        h: usize,
        w: usize,
        mut ws: [f32; 9],
        mut edge_policy: u32,
        nsteps: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<HipBuffer, String> {
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
        let kst = self.func("stencil2d_f32_tiled")?;
        let gx = ((w + 15) / 16) as u32;
        let gy = ((h + 15) / 16) as u32;
        let mut f_u32 = f as u32;
        let mut h_u32 = h as u32;
        let mut w_u32 = w as u32;
        let mut cur_src = in_buf.ptr;
        let mut final_is_ping = true;
        for step in 0..nsteps {
            let mut dst = if step % 2 == 0 {
                final_is_ping = true;
                ping.ptr
            } else {
                final_is_ping = false;
                pong.as_ref().unwrap().ptr
            };
            let mut args = [
                (&mut cur_src as *mut *mut c_void).cast(),
                (&mut dst as *mut *mut c_void).cast(),
                (&mut f_u32 as *mut u32).cast(),
                (&mut h_u32 as *mut u32).cast(),
                (&mut w_u32 as *mut u32).cast(),
                (&mut edge_policy as *mut u32).cast(),
                (&mut ws[0] as *mut f32).cast(),
                (&mut ws[1] as *mut f32).cast(),
                (&mut ws[2] as *mut f32).cast(),
                (&mut ws[3] as *mut f32).cast(),
                (&mut ws[4] as *mut f32).cast(),
                (&mut ws[5] as *mut f32).cast(),
                (&mut ws[6] as *mut f32).cast(),
                (&mut ws[7] as *mut f32).cast(),
                (&mut ws[8] as *mut f32).cast(),
            ];
            self.launch(kst, [gx, gy, f_u32], [16, 16, 1], &mut args)?;
            cur_src = dst;
        }
        self.sync()?;
        let out = if final_is_ping {
            if let Some(ref b) = pong {
                telemetry.note_free(b.bytes);
            }
            ping
        } else {
            telemetry.note_free(ping.bytes);
            pong.unwrap()
        };
        telemetry.kernel_launches += nsteps;
        Ok(out)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spmv_csr_f32(
        &mut self,
        vals: &HipBuffer,
        cols: &HipBuffer,
        row_ptr: &HipBuffer,
        x: &HipBuffer,
        r: usize,
        nz: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<HipBuffer, String> {
        let out_y = self.alloc_buf(r * 4)?;
        telemetry.note_alloc(r * 4);
        if r == 0 {
            return Ok(out_y);
        }
        let avg = nz / r;
        let (kname, groups) = if exact {
            ("spmv_csr_f32_exact", (r + 255) / 256)
        } else if avg >= 16 {
            ("spmv_csr_f32_wg", (r * 32 + 255) / 256)
        } else {
            ("spmv_csr_f32_scalar", (r + 255) / 256)
        };
        let f = self.func(kname)?;
        let mut pv = vals.ptr;
        let mut pc = cols.ptr;
        let mut pr = row_ptr.ptr;
        let mut px = x.ptr;
        let mut py = out_y.ptr;
        let mut r_u32 = r as u32;
        let mut args = [
            (&mut pv as *mut *mut c_void).cast(),
            (&mut pc as *mut *mut c_void).cast(),
            (&mut pr as *mut *mut c_void).cast(),
            (&mut px as *mut *mut c_void).cast(),
            (&mut py as *mut *mut c_void).cast(),
            (&mut r_u32 as *mut u32).cast(),
        ];
        self.launch(f, [groups as u32, 1, 1], [256, 1, 1], &mut args)?;
        self.sync()?;
        telemetry.kernel_launches += 1;
        Ok(out_y)
    }

    pub fn row_reduce_f32(
        &mut self,
        in_buf: &HipBuffer,
        r: usize,
        c: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<HipBuffer, String> {
        let out = self.alloc_buf(r * 4)?;
        telemetry.note_alloc(r * 4);
        if r == 0 {
            return Ok(out);
        }
        let outer = r >= 128 && c < 2048;
        let (kname, groups) = if outer {
            ("row_reduce_f32_outer", (r + 255) / 256)
        } else {
            ("row_reduce_f32_inner", r)
        };
        let f = self.func(kname)?;
        let mut pa = in_buf.ptr;
        let mut po = out.ptr;
        let mut r_u32 = r as u32;
        let mut c_u32 = c as u32;
        let mut args = [
            (&mut pa as *mut *mut c_void).cast(),
            (&mut po as *mut *mut c_void).cast(),
            (&mut r_u32 as *mut u32).cast(),
            (&mut c_u32 as *mut u32).cast(),
        ];
        self.launch(f, [groups as u32, 1, 1], [256, 1, 1], &mut args)?;
        self.sync()?;
        telemetry.kernel_launches += 1;
        Ok(out)
    }
}
