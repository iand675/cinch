//! NVIDIA CUDA GPU compute backend for Cinch's data-parallel arrays
//! (equivalent to Futhark's `cuda` backend).
//!
//! Uses `dlopen`/`dlsym` via [`crate::sys`] to reach the CUDA Driver API
//! (`libcuda.so.1` / `libcuda.so`) and NVRTC (`libnvrtc.so.12` / `libnvrtc.so`)
//! with zero external Cargo dependencies and zero extra linker flags.
//!
//! On hosts without an NVIDIA GPU or CUDA driver, `CudaState::open` cleanly
//! returns an informative error so `cinch_device_open(DEVICE_KIND_CUDA_REQUIRED)`
//! surfaces `DEV_ERR_NO_DEVICE` at the defined error boundary.

use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut};
use std::sync::OnceLock;

use crate::metal::{DeviceLaunchPlan, DeviceTelemetry, SyncPoint};
use crate::par_arrays;

type CuResult = i32;
type CuDevice = i32;
type CuContext = *mut c_void;
type CuModule = *mut c_void;
type CuFunction = *mut c_void;
type CuStream = *mut c_void;
type CuDevicePtr = u64;
type NvrtcResult = i32;
type NvrtcProgram = *mut c_void;

const CUDA_SUCCESS: CuResult = 0;
const NVRTC_SUCCESS: NvrtcResult = 0;

struct CudaApi {
    cu_init: unsafe extern "C" fn(u32) -> CuResult,
    cu_device_get: unsafe extern "C" fn(*mut CuDevice, i32) -> CuResult,
    cu_device_get_name: unsafe extern "C" fn(*mut i8, i32, CuDevice) -> CuResult,
    cu_ctx_create: unsafe extern "C" fn(*mut CuContext, u32, CuDevice) -> CuResult,
    cu_ctx_destroy: unsafe extern "C" fn(CuContext) -> CuResult,
    cu_ctx_synchronize: unsafe extern "C" fn() -> CuResult,
    cu_mem_alloc: unsafe extern "C" fn(*mut CuDevicePtr, usize) -> CuResult,
    cu_mem_free: unsafe extern "C" fn(CuDevicePtr) -> CuResult,
    cu_memcpy_htod: unsafe extern "C" fn(CuDevicePtr, *const c_void, usize) -> CuResult,
    cu_memcpy_dtoh: unsafe extern "C" fn(*mut c_void, CuDevicePtr, usize) -> CuResult,
    cu_module_load_data: unsafe extern "C" fn(*mut CuModule, *const c_void) -> CuResult,
    cu_module_unload: unsafe extern "C" fn(CuModule) -> CuResult,
    cu_module_get_function: unsafe extern "C" fn(*mut CuFunction, CuModule, *const i8) -> CuResult,
    cu_launch_kernel: unsafe extern "C" fn(
        CuFunction,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        CuStream,
        *mut *mut c_void,
        *mut *mut c_void,
    ) -> CuResult,
    nvrtc_create_program: unsafe extern "C" fn(
        *mut NvrtcProgram,
        *const i8,
        *const i8,
        i32,
        *const *const i8,
        *const *const i8,
    ) -> NvrtcResult,
    nvrtc_compile_program: unsafe extern "C" fn(NvrtcProgram, i32, *const *const i8) -> NvrtcResult,
    nvrtc_get_ptx_size: unsafe extern "C" fn(NvrtcProgram, *mut usize) -> NvrtcResult,
    nvrtc_get_ptx: unsafe extern "C" fn(NvrtcProgram, *mut i8) -> NvrtcResult,
    nvrtc_get_program_log_size: unsafe extern "C" fn(NvrtcProgram, *mut usize) -> NvrtcResult,
    nvrtc_get_program_log: unsafe extern "C" fn(NvrtcProgram, *mut i8) -> NvrtcResult,
    nvrtc_destroy_program: unsafe extern "C" fn(*mut NvrtcProgram) -> NvrtcResult,
}

unsafe impl Send for CudaApi {}
unsafe impl Sync for CudaApi {}

static CUDA_API: OnceLock<Option<CudaApi>> = OnceLock::new();

fn api() -> Option<&'static CudaApi> {
    CUDA_API
        .get_or_init(|| unsafe {
            let mut cuda_lib = null_mut();
            for p in &[
                b"libcuda.so.1".as_slice(),
                b"libcuda.so",
                b"/usr/lib/x86_64-linux-gnu/libcuda.so.1",
                b"/usr/lib/aarch64-linux-gnu/libcuda.so.1",
                b"/usr/local/cuda/lib64/libcuda.so",
            ] {
                cuda_lib = crate::sys::dlopen(p);
                if !cuda_lib.is_null() {
                    break;
                }
            }
            let mut nvrtc_lib = null_mut();
            for p in &[
                b"libnvrtc.so.12".as_slice(),
                b"libnvrtc.so.11.2",
                b"libnvrtc.so",
                b"/usr/local/cuda/lib64/libnvrtc.so",
            ] {
                nvrtc_lib = crate::sys::dlopen(p);
                if !nvrtc_lib.is_null() {
                    break;
                }
            }
            if cuda_lib.is_null() || nvrtc_lib.is_null() {
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
            Some(CudaApi {
                cu_init: load!(cuda_lib, b"cuInit"),
                cu_device_get: load!(cuda_lib, b"cuDeviceGet"),
                cu_device_get_name: load!(cuda_lib, b"cuDeviceGetName"),
                cu_ctx_create: load!(cuda_lib, b"cuCtxCreate_v2"),
                cu_ctx_destroy: load!(cuda_lib, b"cuCtxDestroy_v2"),
                cu_ctx_synchronize: load!(cuda_lib, b"cuCtxSynchronize"),
                cu_mem_alloc: load!(cuda_lib, b"cuMemAlloc_v2"),
                cu_mem_free: load!(cuda_lib, b"cuMemFree_v2"),
                cu_memcpy_htod: load!(cuda_lib, b"cuMemcpyHtoD_v2"),
                cu_memcpy_dtoh: load!(cuda_lib, b"cuMemcpyDtoH_v2"),
                cu_module_load_data: load!(cuda_lib, b"cuModuleLoadData"),
                cu_module_unload: load!(cuda_lib, b"cuModuleUnload"),
                cu_module_get_function: load!(cuda_lib, b"cuModuleGetFunction"),
                cu_launch_kernel: load!(cuda_lib, b"cuLaunchKernel"),
                nvrtc_create_program: load!(nvrtc_lib, b"nvrtcCreateProgram"),
                nvrtc_compile_program: load!(nvrtc_lib, b"nvrtcCompileProgram"),
                nvrtc_get_ptx_size: load!(nvrtc_lib, b"nvrtcGetPTXSize"),
                nvrtc_get_ptx: load!(nvrtc_lib, b"nvrtcGetPTX"),
                nvrtc_get_program_log_size: load!(nvrtc_lib, b"nvrtcGetProgramLogSize"),
                nvrtc_get_program_log: load!(nvrtc_lib, b"nvrtcGetProgramLog"),
                nvrtc_destroy_program: load!(nvrtc_lib, b"nvrtcDestroyProgram"),
            })
        })
        .as_ref()
}

pub fn is_available() -> bool {
    api().is_some() && CudaState::open().is_ok()
}

// ---------------------------------------------------------------------------
// Built-in Production CUDA C++ Kernels (all 7 Cinch kernel families)
// ---------------------------------------------------------------------------

pub const BUILTIN_CUDA_SOURCE: &str = r#"
extern "C" {

// 1. Fused Dot Product (Warp-Shuffle Relaxed & Exact-Tree 1024-element blocks)

__device__ __forceinline__ float warp_sum(float v) {
    for (int offset = 16; offset > 0; offset >>= 1) {
        v += __shfl_down_sync(0xFFFFFFFFu, v, offset);
    }
    return v;
}

__global__ void dot_f32_relaxed(
    const float* __restrict__ a,
    const float* __restrict__ b,
    float* __restrict__ partials,
    unsigned int n
) {
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int gsize = gridDim.x * blockDim.x;
    float acc = 0.0f;
    for (unsigned int i = gid; i < n; i += gsize) {
        acc += a[i] * b[i];
    }
    acc = warp_sum(acc);
    __shared__ float shared[8];
    unsigned int lane = threadIdx.x & 31u;
    unsigned int wid = threadIdx.x >> 5;
    if (lane == 0u) shared[wid] = acc;
    __syncthreads();
    if (wid == 0u) {
        float v = (lane < 8u) ? shared[lane] : 0.0f;
        v = warp_sum(v);
        if (lane == 0u) partials[blockIdx.x] = v;
    }
}

__global__ void dot_f32_exact_block(
    const float* __restrict__ a,
    const float* __restrict__ b,
    float* __restrict__ block_out,
    unsigned int n
) {
    __shared__ float s[1024];
    unsigned int tid = threadIdx.x;
    unsigned int base = blockIdx.x * 1024u;
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
                if ((idx & ((stride << 1) - 1u)) == 0u) {
                    s[idx] = s[idx] + s[idx + stride];
                }
            }
            __syncthreads();
        }
        if (tid == 0u) block_out[blockIdx.x] = s[0];
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
        block_out[blockIdx.x] = acc;
    }
}

// 2. Prefix Scan (Inclusive & Exclusive)

__global__ void scan_i32_block(
    const int* __restrict__ in_arr,
    int* __restrict__ out_arr,
    int* __restrict__ block_sums,
    unsigned int n,
    unsigned int inclusive
) {
    __shared__ int s[256];
    unsigned int tid = threadIdx.x;
    unsigned int gid = blockIdx.x * 256u + tid;
    int val = (gid < n) ? in_arr[gid] : 0;
    s[tid] = val;
    __syncthreads();
    for (unsigned int offset = 1u; offset < 256u; offset <<= 1) {
        int add = (tid >= offset) ? s[tid - offset] : 0;
        __syncthreads();
        s[tid] += add;
        __syncthreads();
    }
    if (tid == 255u) block_sums[blockIdx.x] = s[255];
    if (gid < n) out_arr[gid] = (inclusive != 0u) ? s[tid] : (s[tid] - val);
}

__global__ void scan_i32_add_offsets(
    int* __restrict__ out_arr,
    const int* __restrict__ block_offsets,
    unsigned int n
) {
    unsigned int gid = blockIdx.x * 256u + threadIdx.x;
    if (gid < n) out_arr[gid] += block_offsets[blockIdx.x];
}

// 3. Stable Compaction (Filter i32 > threshold)

__global__ void filter_i32_gt_flags(
    const int* __restrict__ in_arr,
    int* __restrict__ flags,
    unsigned int n,
    int threshold
) {
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid < n) flags[gid] = (in_arr[gid] > threshold) ? 1 : 0;
}

__global__ void filter_i32_gt_scatter(
    const int* __restrict__ in_arr,
    const int* __restrict__ flags,
    const int* __restrict__ positions,
    int* __restrict__ out_arr,
    unsigned int n
) {
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid < n && flags[gid] != 0) out_arr[positions[gid]] = in_arr[gid];
}

// 4. Batched Histogram (U8 -> U32) with adaptive many-small vs few-large schedules

__global__ void hist_u8_zero(unsigned int* out_hist, unsigned int total) {
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    if (gid < total) out_hist[gid] = 0u;
}

__global__ void hist_u8_many_small_frames(
    const unsigned char* __restrict__ pixels,
    unsigned int* __restrict__ out_hist,
    unsigned int frames,
    unsigned int frame_pixels,
    unsigned int num_bins
) {
    unsigned int frame = blockIdx.x;
    unsigned int tid = threadIdx.x;
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
    const unsigned char* __restrict__ pixels,
    unsigned int* __restrict__ out_hist,
    unsigned int frames,
    unsigned int frame_pixels,
    unsigned int num_bins,
    unsigned int tiles_per_frame
) {
    unsigned int frame = blockIdx.x / tiles_per_frame;
    unsigned int tile = blockIdx.x % tiles_per_frame;
    unsigned int tid = threadIdx.x;
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

// 5. Batched 2D 3x3 Stencil (U8 & F32) with 18x18 shared halo tile

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
    const unsigned char* __restrict__ src,
    unsigned char* __restrict__ dst,
    unsigned int frames, unsigned int height, unsigned int width,
    int divisor, unsigned int edge_policy,
    int w0, int w1, int w2, int w3, int w4, int w5, int w6, int w7, int w8
) {
    unsigned int gx = blockIdx.x * 16u + threadIdx.x;
    unsigned int gy = blockIdx.y * 16u + threadIdx.y;
    unsigned int frame = blockIdx.z;
    if (frame >= frames) return;
    int h = (int)height, w = (int)width;
    unsigned int fbase = frame * height * width;
    __shared__ unsigned char tile[18][18];
    int tile_x0 = (int)blockIdx.x * 16 - 1;
    int tile_y0 = (int)blockIdx.y * 16 - 1;
    unsigned int flat_tid = threadIdx.y * 16u + threadIdx.x;
    for (unsigned int k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u), tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0 : src[fbase + (unsigned int)(ry * w + rx)];
    }
    __syncthreads();
    if (gx >= width || gy >= height) return;
    unsigned int ly = threadIdx.y + 1u, lx = threadIdx.x + 1u;
    int acc = w0 * (int)tile[ly - 1][lx - 1] + w1 * (int)tile[ly - 1][lx] + w2 * (int)tile[ly - 1][lx + 1]
            + w3 * (int)tile[ly][lx - 1]     + w4 * (int)tile[ly][lx]     + w5 * (int)tile[ly][lx + 1]
            + w6 * (int)tile[ly + 1][lx - 1] + w7 * (int)tile[ly + 1][lx] + w8 * (int)tile[ly + 1][lx + 1];
    int q = acc / ((divisor == 0) ? 1 : divisor);
    q = (q < 0) ? 0 : ((q > 255) ? 255 : q);
    dst[fbase + gy * width + gx] = (unsigned char)q;
}

__global__ void stencil2d_f32_tiled(
    const float* __restrict__ src,
    float* __restrict__ dst,
    unsigned int frames, unsigned int height, unsigned int width,
    unsigned int edge_policy,
    float w0, float w1, float w2, float w3, float w4, float w5, float w6, float w7, float w8
) {
    unsigned int gx = blockIdx.x * 16u + threadIdx.x;
    unsigned int gy = blockIdx.y * 16u + threadIdx.y;
    unsigned int frame = blockIdx.z;
    if (frame >= frames) return;
    int h = (int)height, w = (int)width;
    unsigned int fbase = frame * height * width;
    __shared__ float tile[18][18];
    int tile_x0 = (int)blockIdx.x * 16 - 1;
    int tile_y0 = (int)blockIdx.y * 16 - 1;
    unsigned int flat_tid = threadIdx.y * 16u + threadIdx.x;
    for (unsigned int k = flat_tid; k < 18u * 18u; k += 256u) {
        int ty = (int)(k / 18u), tx = (int)(k % 18u);
        int ry = resolve_coord(tile_y0 + ty, h, edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0.0f : src[fbase + (unsigned int)(ry * w + rx)];
    }
    __syncthreads();
    if (gx >= width || gy >= height) return;
    unsigned int ly = threadIdx.y + 1u, lx = threadIdx.x + 1u;
    dst[fbase + gy * width + gx] =
        w0 * tile[ly - 1][lx - 1] + w1 * tile[ly - 1][lx] + w2 * tile[ly - 1][lx + 1]
      + w3 * tile[ly][lx - 1]     + w4 * tile[ly][lx]     + w5 * tile[ly][lx + 1]
      + w6 * tile[ly + 1][lx - 1] + w7 * tile[ly + 1][lx] + w8 * tile[ly + 1][lx + 1];
}

// 6. Adaptive Row Reduction

__global__ void row_reduce_f32_outer(
    const float* __restrict__ a, float* __restrict__ out, unsigned int rows, unsigned int cols
) {
    unsigned int row = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= rows) return;
    float acc = 0.0f;
    unsigned int base = row * cols;
    for (unsigned int j = 0; j < cols; ++j) acc += a[base + j];
    out[row] = acc;
}

__global__ void row_reduce_f32_inner(
    const float* __restrict__ a, float* __restrict__ out, unsigned int rows, unsigned int cols
) {
    unsigned int row = blockIdx.x;
    if (row >= rows) return;
    float acc = 0.0f;
    unsigned int base = row * cols;
    for (unsigned int j = threadIdx.x; j < cols; j += 256u) acc += a[base + j];
    acc = warp_sum(acc);
    __shared__ float shared[8];
    unsigned int lane = threadIdx.x & 31u, wid = threadIdx.x >> 5;
    if (lane == 0u) shared[wid] = acc;
    __syncthreads();
    if (wid == 0u) {
        float v = (lane < 8u) ? shared[lane] : 0.0f;
        v = warp_sum(v);
        if (lane == 0u) out[row] = v;
    }
}

// 7. CSR SpMV (scalar, warp-per-row, exact-tree with empty rows)

__global__ void spmv_csr_f32_scalar(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    unsigned int row = blockIdx.x * blockDim.x + threadIdx.x;
    if (row >= nrows) return;
    unsigned int s = row_ptr[row], e = row_ptr[row + 1];
    float acc = 0.0f;
    for (unsigned int k = s; k < e; ++k) acc += vals[k] * x[cols[k]];
    y[row] = acc;
}

__global__ void spmv_csr_f32_warp(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x;
    unsigned int lane = threadIdx.x & 31u;
    unsigned int row = gid >> 5;
    if (row >= nrows) return;
    unsigned int s = row_ptr[row], e = row_ptr[row + 1];
    float acc = 0.0f;
    for (unsigned int k = s + lane; k < e; k += 32u) acc += vals[k] * x[cols[k]];
    acc = warp_sum(acc);
    if (lane == 0u) y[row] = acc;
}

__global__ void spmv_csr_f32_exact(
    const float* __restrict__ vals, const unsigned int* __restrict__ cols,
    const unsigned int* __restrict__ row_ptr, const float* __restrict__ x,
    float* __restrict__ y, unsigned int nrows
) {
    unsigned int row = blockIdx.x * blockDim.x + threadIdx.x;
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
// CUDA State & Resident Buffer
// ---------------------------------------------------------------------------

pub struct CudaBuffer {
    pub ptr: CuDevicePtr,
    pub bytes: usize,
}

impl Drop for CudaBuffer {
    fn drop(&mut self) {
        if self.ptr != 0 {
            if let Some(a) = api() {
                unsafe { (a.cu_mem_free)(self.ptr) };
            }
        }
    }
}

pub struct CudaState {
    pub device_name: String,
    ctx: CuContext,
    module: CuModule,
    funcs: HashMap<String, CuFunction>,
}

unsafe impl Send for CudaState {}
unsafe impl Sync for CudaState {}

impl Drop for CudaState {
    fn drop(&mut self) {
        if let Some(a) = api() {
            if !self.module.is_null() {
                unsafe { (a.cu_module_unload)(self.module) };
            }
            if !self.ctx.is_null() {
                unsafe { (a.cu_ctx_destroy)(self.ctx) };
            }
        }
    }
}

impl CudaState {
    pub fn open() -> Result<Self, String> {
        let a = api().ok_or_else(|| "NVIDIA CUDA driver (libcuda + libnvrtc) not available on this host".to_string())?;
        let rc = unsafe { (a.cu_init)(0) };
        if rc != CUDA_SUCCESS {
            return Err(format!("cuInit failed ({rc})"));
        }
        let mut dev: CuDevice = 0;
        let rc = unsafe { (a.cu_device_get)(&mut dev, 0) };
        if rc != CUDA_SUCCESS {
            return Err(format!("cuDeviceGet(0) failed ({rc})"));
        }
        let mut name_buf = [0i8; 256];
        let _ = unsafe { (a.cu_device_get_name)(name_buf.as_mut_ptr(), 256, dev) };
        let device_name = unsafe { CStr::from_ptr(name_buf.as_ptr()) }
            .to_string_lossy()
            .trim()
            .to_string();
        let mut ctx: CuContext = null_mut();
        let rc = unsafe { (a.cu_ctx_create)(&mut ctx, 0, dev) };
        if rc != CUDA_SUCCESS || ctx.is_null() {
            return Err(format!("cuCtxCreate_v2 failed ({rc})"));
        }

        // Compile built-in CUDA kernels via NVRTC -> PTX -> cuModuleLoadData
        let mut prog: NvrtcProgram = null_mut();
        let src_c = std::ffi::CString::new(BUILTIN_CUDA_SOURCE).unwrap();
        let name_c = std::ffi::CString::new("cinch_builtin.cu").unwrap();
        let rc = unsafe {
            (a.nvrtc_create_program)(&mut prog, src_c.as_ptr(), name_c.as_ptr(), 0, null(), null())
        };
        if rc != NVRTC_SUCCESS {
            unsafe { (a.cu_ctx_destroy)(ctx) };
            return Err(format!("nvrtcCreateProgram failed ({rc})"));
        }
        let comp_rc = unsafe { (a.nvrtc_compile_program)(prog, 0, null()) };
        if comp_rc != NVRTC_SUCCESS {
            let mut log_sz = 0usize;
            unsafe { (a.nvrtc_get_program_log_size)(prog, &mut log_sz) };
            let mut log = vec![0u8; log_sz.max(1)];
            unsafe {
                (a.nvrtc_get_program_log)(prog, log.as_mut_ptr().cast());
                (a.nvrtc_destroy_program)(&mut prog);
                (a.cu_ctx_destroy)(ctx);
            }
            return Err(format!(
                "nvrtcCompileProgram failed ({comp_rc}): {}",
                String::from_utf8_lossy(&log)
            ));
        }
        let mut ptx_sz = 0usize;
        unsafe { (a.nvrtc_get_ptx_size)(prog, &mut ptx_sz) };
        let mut ptx = vec![0u8; ptx_sz.max(1)];
        unsafe {
            (a.nvrtc_get_ptx)(prog, ptx.as_mut_ptr().cast());
            (a.nvrtc_destroy_program)(&mut prog);
        }
        let mut module: CuModule = null_mut();
        let rc = unsafe { (a.cu_module_load_data)(&mut module, ptx.as_ptr().cast()) };
        if rc != CUDA_SUCCESS || module.is_null() {
            unsafe { (a.cu_ctx_destroy)(ctx) };
            return Err(format!("cuModuleLoadData failed ({rc})"));
        }

        Ok(Self {
            device_name,
            ctx,
            module,
            funcs: HashMap::new(),
        })
    }

    fn func(&mut self, name: &str) -> Result<CuFunction, String> {
        if let Some(&f) = self.funcs.get(name) {
            return Ok(f);
        }
        let a = api().unwrap();
        let cname = std::ffi::CString::new(name).unwrap();
        let mut f: CuFunction = null_mut();
        let rc = unsafe { (a.cu_module_get_function)(&mut f, self.module, cname.as_ptr()) };
        if rc != CUDA_SUCCESS || f.is_null() {
            return Err(format!("cuModuleGetFunction('{name}') failed ({rc})"));
        }
        self.funcs.insert(name.to_string(), f);
        Ok(f)
    }

    pub fn alloc_buf(&self, bytes: usize) -> Result<CudaBuffer, String> {
        let a = api().unwrap();
        let mut ptr: CuDevicePtr = 0;
        let rc = unsafe { (a.cu_mem_alloc)(&mut ptr, bytes.max(4)) };
        if rc != CUDA_SUCCESS {
            return Err(format!("cuMemAlloc_v2({bytes}) failed ({rc})"));
        }
        Ok(CudaBuffer { ptr, bytes })
    }

    pub fn upload(&self, src: *const u8, bytes: usize) -> Result<CudaBuffer, String> {
        let a = api().unwrap();
        let buf = self.alloc_buf(bytes)?;
        if bytes > 0 && !src.is_null() {
            let rc = unsafe { (a.cu_memcpy_htod)(buf.ptr, src.cast(), bytes) };
            if rc != CUDA_SUCCESS {
                return Err(format!("cuMemcpyHtoD_v2 failed ({rc})"));
            }
        }
        Ok(buf)
    }

    pub fn download(&self, buf: &CudaBuffer, dst: *mut u8, bytes: usize) -> Result<(), String> {
        let a = api().unwrap();
        if bytes > 0 && !dst.is_null() {
            let rc = unsafe { (a.cu_memcpy_dtoh)(dst.cast(), buf.ptr, bytes) };
            if rc != CUDA_SUCCESS {
                return Err(format!("cuMemcpyDtoH_v2 failed ({rc})"));
            }
        }
        Ok(())
    }

    fn launch(
        &self,
        f: CuFunction,
        grid: [u32; 3],
        block: [u32; 3],
        params: &mut [*mut c_void],
    ) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe {
            (a.cu_launch_kernel)(
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
        if rc != CUDA_SUCCESS {
            return Err(format!("cuLaunchKernel failed ({rc})"));
        }
        Ok(())
    }

    fn sync(&self) -> Result<(), String> {
        let a = api().unwrap();
        let rc = unsafe { (a.cu_ctx_synchronize)() };
        if rc != CUDA_SUCCESS {
            return Err(format!("cuCtxSynchronize failed ({rc})"));
        }
        Ok(())
    }

    pub fn dot_f32(
        &mut self,
        a_buf: &CudaBuffer,
        b_buf: &CudaBuffer,
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
                (&mut pa as *mut CuDevicePtr).cast(),
                (&mut pb as *mut CuDevicePtr).cast(),
                (&mut pp as *mut CuDevicePtr).cast(),
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
                reason: "cuda-ctx-sync".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-exact-tree".to_string(),
                schedule: format!("cuda-exact-tree-1024(blocks={nblocks})"),
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
                (&mut pa as *mut CuDevicePtr).cast(),
                (&mut pb as *mut CuDevicePtr).cast(),
                (&mut pp as *mut CuDevicePtr).cast(),
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
                reason: "cuda-ctx-sync".to_string(),
            });
            telemetry.last_plan = DeviceLaunchPlan {
                op: "dot-f32-relaxed".to_string(),
                schedule: format!("cuda-warp-shuffle(groups={ngroups})"),
                grid: [ngroups * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 32,
                numerical_mode: "f32-add-relaxed".to_string(),
            };
            Ok(h.into_iter().fold(0.0f32, |a, b| a + b))
        }
    }

    pub fn scan_i32(
        &mut self,
        in_buf: &CudaBuffer,
        n: usize,
        inclusive: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<CudaBuffer, String> {
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
            (&mut pin as *mut CuDevicePtr).cast(),
            (&mut pout as *mut CuDevicePtr).cast(),
            (&mut pbs as *mut CuDevicePtr).cast(),
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
                (&mut pout as *mut CuDevicePtr).cast(),
                (&mut poffs as *mut CuDevicePtr).cast(),
                (&mut n_u32 as *mut u32).cast(),
            ];
            self.launch(f2, [nblocks as u32, 1, 1], [256, 1, 1], &mut a2)?;
            launches += 1;
        }
        self.sync()?;
        telemetry.note_free(nblocks * 4);
        telemetry.kernel_launches += launches;
        telemetry.sync_points.push(SyncPoint {
            op: "scan-i32".to_string(),
            reason: "cuda-ctx-sync".to_string(),
        });
        Ok(out_buf)
    }

    pub fn filter_i32_gt(
        &mut self,
        in_buf: &CudaBuffer,
        n: usize,
        threshold: i32,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<(CudaBuffer, usize), String> {
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
            (&mut pin as *mut CuDevicePtr).cast(),
            (&mut pflags as *mut CuDevicePtr).cast(),
            (&mut n_u32 as *mut u32).cast(),
            (&mut thr as *mut i32).cast(),
        ];
        self.launch(kf, [nblocks as u32, 1, 1], [256, 1, 1], &mut af)?;
        let pos = self.scan_i32(&flags, n, false, telemetry)?;
        let mut last_f = 0i32;
        let mut last_p = 0i32;
        let a = api().unwrap();
        unsafe {
            (a.cu_memcpy_dtoh)((&mut last_f as *mut i32).cast(), flags.ptr + ((n - 1) * 4) as u64, 4);
            (a.cu_memcpy_dtoh)((&mut last_p as *mut i32).cast(), pos.ptr + ((n - 1) * 4) as u64, 4);
        }
        let k = (last_p + last_f).max(0) as usize;
        let out_buf = self.alloc_buf(k * 4)?;
        telemetry.note_alloc(k * 4);
        if k > 0 {
            let mut ppos = pos.ptr;
            let mut pout = out_buf.ptr;
            let ks = self.func("filter_i32_gt_scatter")?;
            let mut as_ = [
                (&mut pin as *mut CuDevicePtr).cast(),
                (&mut pflags as *mut CuDevicePtr).cast(),
                (&mut ppos as *mut CuDevicePtr).cast(),
                (&mut pout as *mut CuDevicePtr).cast(),
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
        in_buf: &CudaBuffer,
        f: usize,
        h: usize,
        w: usize,
        mut ws: [i32; 9],
        mut divisor: i32,
        mut edge_policy: u32,
        nsteps: usize,
        bins: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<CudaBuffer, String> {
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
                    (&mut cur_src as *mut CuDevicePtr).cast(),
                    (&mut dst as *mut CuDevicePtr).cast(),
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
                (&mut cur_src as *mut CuDevicePtr).cast(),
                (&mut pout as *mut CuDevicePtr).cast(),
                (&mut f_u32 as *mut u32).cast(),
                (&mut fp_u32 as *mut u32).cast(),
                (&mut bins_u32 as *mut u32).cast(),
            ];
            self.launch(kh, [hist_groups as u32, 1, 1], [256, 1, 1], &mut args)?;
            launches += 1;
        } else {
            let mut total_bins = (f * bins) as u32;
            let kz = self.func("hist_u8_zero")?;
            let mut az = [(&mut pout as *mut CuDevicePtr).cast(), (&mut total_bins as *mut u32).cast()];
            self.launch(kz, [((f * bins + 255) / 256) as u32, 1, 1], [256, 1, 1], &mut az)?;
            let kh = self.func("hist_u8_few_large_frames")?;
            let mut ah = [
                (&mut cur_src as *mut CuDevicePtr).cast(),
                (&mut pout as *mut CuDevicePtr).cast(),
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
            reason: "cuda-ctx-sync".to_string(),
        });
        telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil({nsteps})-then-histogram-u8"),
            schedule: if many_small {
                format!("many-small-frames(cuda,frames={f},h={h},w={w},hist_groups={hist_groups})")
            } else {
                format!("few-large-frames(cuda,frames={f},h={h},w={w},tiles_per_frame={tiles_per_frame})")
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
        in_buf: &CudaBuffer,
        f: usize,
        h: usize,
        w: usize,
        mut ws: [f32; 9],
        mut edge_policy: u32,
        nsteps: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<CudaBuffer, String> {
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
                (&mut cur_src as *mut CuDevicePtr).cast(),
                (&mut dst as *mut CuDevicePtr).cast(),
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
        vals: &CudaBuffer,
        cols: &CudaBuffer,
        row_ptr: &CudaBuffer,
        x: &CudaBuffer,
        r: usize,
        nz: usize,
        exact: bool,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<CudaBuffer, String> {
        let out_y = self.alloc_buf(r * 4)?;
        telemetry.note_alloc(r * 4);
        if r == 0 {
            return Ok(out_y);
        }
        let avg = nz / r;
        let (kname, groups) = if exact {
            ("spmv_csr_f32_exact", (r + 255) / 256)
        } else if avg >= 16 {
            ("spmv_csr_f32_warp", (r * 32 + 255) / 256)
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
            (&mut pv as *mut CuDevicePtr).cast(),
            (&mut pc as *mut CuDevicePtr).cast(),
            (&mut pr as *mut CuDevicePtr).cast(),
            (&mut px as *mut CuDevicePtr).cast(),
            (&mut py as *mut CuDevicePtr).cast(),
            (&mut r_u32 as *mut u32).cast(),
        ];
        self.launch(f, [groups as u32, 1, 1], [256, 1, 1], &mut args)?;
        self.sync()?;
        telemetry.kernel_launches += 1;
        Ok(out_y)
    }

    pub fn row_reduce_f32(
        &mut self,
        in_buf: &CudaBuffer,
        r: usize,
        c: usize,
        telemetry: &mut DeviceTelemetry,
    ) -> Result<CudaBuffer, String> {
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
            (&mut pa as *mut CuDevicePtr).cast(),
            (&mut po as *mut CuDevicePtr).cast(),
            (&mut r_u32 as *mut u32).cast(),
            (&mut c_u32 as *mut u32).cast(),
        ];
        self.launch(f, [groups as u32, 1, 1], [256, 1, 1], &mut args)?;
        self.sync()?;
        telemetry.kernel_launches += 1;
        Ok(out)
    }
}
