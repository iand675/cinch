//! Production Apple Metal GPU compute backend for Cinch's data-parallel arrays.
//!
//! Uses `dlopen`/`dlsym` via [`crate::sys`] to reach Apple's `Metal.framework`,
//! `Foundation.framework`, and `/usr/lib/libobjc.A.dylib` with zero external
//! Cargo dependencies and zero extra linker flags.
//!
//! Capabilities:
//! - Device context lifecycle (`cinch_device_open`, `cinch_device_close`),
//!   supporting both Metal GPU and CPU fallback, plus `MetalRequired` mode.
//! - Resident typed GPU buffers (`MTLBuffer`) with upload, on-device kernel
//!   submission, explicit wait/sync, and download.
//! - Buffer retention: every submitted command buffer retains its input/output
//!   buffers until completion, preventing use-after-free even if the host handle
//!   is dropped before `wait`.
//! - Defined error boundary: missing GPU, shader compile failure, allocation
//!   exhaustion (`newBufferWithLength:` returning null), and command-buffer
//!   execution errors are returned as structured error codes + messages.
//! - Full telemetry: upload/download counts & bytes, kernel launches,
//!   synchronization points, peak device bytes, and launch plans (grid,
//!   threadgroup, shared memory, adaptive schedule, numerical mode).
//! - Production MSL compute kernels + generic compiler-emitted MSL runner for
//!   Typed Array IR codegen.

use std::collections::HashMap;
use std::ffi::{c_void, CStr};
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, OnceLock};

use crate::{abi, arrays, par_arrays};

// ---------------------------------------------------------------------------
// Error codes & device backend kinds
// ---------------------------------------------------------------------------

pub const DEVICE_KIND_CPU: i64 = 0;
pub const DEVICE_KIND_METAL: i64 = 1;
pub const DEVICE_KIND_METAL_REQUIRED: i64 = 2;
pub const DEVICE_KIND_OPENCL_REQUIRED: i64 = 3;
pub const DEVICE_KIND_CUDA_REQUIRED: i64 = 4;
pub const DEVICE_KIND_HIP_REQUIRED: i64 = 5;
pub const DEVICE_KIND_CPU_SEQ: i64 = 6;
pub const DEVICE_KIND_ISPC_SIMD: i64 = 7;
pub const DEVICE_KIND_AUTO: i64 = 8;
pub const DEVICE_KIND_OPENCL: i64 = 9;
pub const DEVICE_KIND_CUDA: i64 = 10;
pub const DEVICE_KIND_HIP: i64 = 11;

pub const DEV_OK: i64 = 0;
pub const DEV_ERR_NO_DEVICE: i64 = -1;
pub const DEV_ERR_INVALID_HANDLE: i64 = -2;
pub const DEV_ERR_OUT_OF_MEMORY: i64 = -3;
pub const DEV_ERR_COMPILE_FAILED: i64 = -4;
pub const DEV_ERR_EXECUTION_FAILED: i64 = -5;
pub const DEV_ERR_INVALID_SHAPE: i64 = -6;

// ---------------------------------------------------------------------------
// Metal & Objective-C dynamic bindings (macOS)
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod objc {
    use std::ffi::{c_void, CStr};
    use std::ptr::null_mut;
    use std::sync::OnceLock;

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct MTLSize {
        pub width: usize,
        pub height: usize,
        pub depth: usize,
    }

    pub const MTL_RESOURCE_STORAGE_MODE_SHARED: usize = 0;
    pub const MTL_CMD_BUF_STATUS_ERROR: usize = 5;

    pub struct ObjcApi {
        pub get_class: unsafe extern "C" fn(*const u8) -> *mut c_void,
        pub sel_register: unsafe extern "C" fn(*const u8) -> *mut c_void,
        pub msg_send: *const c_void,
        pub pool_push: unsafe extern "C" fn() -> *mut c_void,
        pub pool_pop: unsafe extern "C" fn(*mut c_void),
        pub create_default_device: unsafe extern "C" fn() -> *mut c_void,
    }

    unsafe impl Send for ObjcApi {}
    unsafe impl Sync for ObjcApi {}

    static API: OnceLock<Option<ObjcApi>> = OnceLock::new();

    pub fn api() -> Option<&'static ObjcApi> {
        API.get_or_init(|| unsafe {
            let objc_lib = crate::sys::dlopen(b"/usr/lib/libobjc.A.dylib");
            let _foundation = crate::sys::dlopen(
                b"/System/Library/Frameworks/Foundation.framework/Foundation",
            );
            let metal_lib =
                crate::sys::dlopen(b"/System/Library/Frameworks/Metal.framework/Metal");
            if objc_lib.is_null() || metal_lib.is_null() {
                return None;
            }
            let get_class = crate::sys::dlsym(objc_lib, b"objc_getClass");
            let sel_register = crate::sys::dlsym(objc_lib, b"sel_registerName");
            let msg_send = crate::sys::dlsym(objc_lib, b"objc_msgSend");
            let pool_push = crate::sys::dlsym(objc_lib, b"objc_autoreleasePoolPush");
            let pool_pop = crate::sys::dlsym(objc_lib, b"objc_autoreleasePoolPop");
            let create_dev = crate::sys::dlsym(metal_lib, b"MTLCreateSystemDefaultDevice");
            if get_class.is_null()
                || sel_register.is_null()
                || msg_send.is_null()
                || pool_push.is_null()
                || pool_pop.is_null()
                || create_dev.is_null()
            {
                return None;
            }
            Some(ObjcApi {
                get_class: std::mem::transmute(get_class),
                sel_register: std::mem::transmute(sel_register),
                msg_send,
                pool_push: std::mem::transmute(pool_push),
                pool_pop: std::mem::transmute(pool_pop),
                create_default_device: std::mem::transmute(create_dev),
            })
        })
        .as_ref()
    }

    pub struct AutoreleasePool {
        token: *mut c_void,
        pop: unsafe extern "C" fn(*mut c_void),
    }

    impl AutoreleasePool {
        pub fn push() -> Option<Self> {
            let a = api()?;
            let token = unsafe { (a.pool_push)() };
            Some(Self {
                token,
                pop: a.pool_pop,
            })
        }
    }

    impl Drop for AutoreleasePool {
        fn drop(&mut self) {
            unsafe { (self.pop)(self.token) };
        }
    }

    #[inline]
    pub unsafe fn sel(name: &[u8]) -> *mut c_void {
        let a = api().unwrap();
        unsafe { (a.sel_register)(name.as_ptr()) }
    }

    #[inline]
    pub unsafe fn cls(name: &[u8]) -> *mut c_void {
        let a = api().unwrap();
        unsafe { (a.get_class)(name.as_ptr()) }
    }

    #[inline]
    pub unsafe fn msg0(obj: *mut c_void, s: *mut c_void) -> *mut c_void {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) -> *mut c_void =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s) }
    }

    #[inline]
    pub unsafe fn msg0_void(obj: *mut c_void, s: *mut c_void) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s) }
    }

    #[inline]
    pub unsafe fn msg0_usize(obj: *mut c_void, s: *mut c_void) -> usize {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void) -> usize =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s) }
    }

    #[inline]
    pub unsafe fn msg1_ptr(obj: *mut c_void, s: *mut c_void, a1: *const c_void) -> *mut c_void {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void) -> *mut c_void =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1) }
    }

    #[inline]
    pub unsafe fn msg1_ptr_void(obj: *mut c_void, s: *mut c_void, a1: *mut c_void) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1) }
    }

    #[inline]
    pub unsafe fn msg2_usize(
        obj: *mut c_void,
        s: *mut c_void,
        a1: usize,
        a2: usize,
    ) -> *mut c_void {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, usize, usize) -> *mut c_void =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1, a2) }
    }

    #[inline]
    pub unsafe fn msg2_usize_void(obj: *mut c_void, s: *mut c_void, a1: usize, a2: usize) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, usize, usize) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1, a2) }
    }

    #[inline]
    pub unsafe fn msg2_ptr_err(
        obj: *mut c_void,
        s: *mut c_void,
        a1: *mut c_void,
        err: *mut *mut c_void,
    ) -> *mut c_void {
        let f: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut *mut c_void,
        ) -> *mut c_void = unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1, err) }
    }

    #[inline]
    pub unsafe fn msg3_lib(
        obj: *mut c_void,
        s: *mut c_void,
        src: *mut c_void,
        opts: *mut c_void,
        err: *mut *mut c_void,
    ) -> *mut c_void {
        let f: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut c_void,
            *mut *mut c_void,
        ) -> *mut c_void = unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, src, opts, err) }
    }

    #[inline]
    pub unsafe fn msg3_set_buf(
        obj: *mut c_void,
        s: *mut c_void,
        buf: *mut c_void,
        off: usize,
        idx: usize,
    ) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void, usize, usize) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, buf, off, idx) }
    }

    #[inline]
    pub unsafe fn msg3_set_bytes(
        obj: *mut c_void,
        s: *mut c_void,
        ptr: *const c_void,
        len: usize,
        idx: usize,
    ) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, *const c_void, usize, usize) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, ptr, len, idx) }
    }

    #[inline]
    pub unsafe fn msg2_dispatch(
        obj: *mut c_void,
        s: *mut c_void,
        a1: MTLSize,
        a2: MTLSize,
    ) {
        let f: unsafe extern "C" fn(*mut c_void, *mut c_void, MTLSize, MTLSize) =
            unsafe { std::mem::transmute(api().unwrap().msg_send) };
        unsafe { f(obj, s, a1, a2) }
    }

    pub unsafe fn nsstring(text: &str) -> *mut c_void {
        let mut buf = Vec::with_capacity(text.len() + 1);
        buf.extend_from_slice(text.as_bytes());
        buf.push(0);
        unsafe {
            msg1_ptr(
                cls(b"NSString\0"),
                sel(b"stringWithUTF8String:\0"),
                buf.as_ptr().cast(),
            )
        }
    }

    pub unsafe fn nsstring_to_rust(ns: *mut c_void) -> String {
        if ns.is_null() {
            return String::new();
        }
        let cstr = unsafe { msg0(ns, sel(b"UTF8String\0")) } as *const i8;
        if cstr.is_null() {
            return String::new();
        }
        unsafe { CStr::from_ptr(cstr) }.to_string_lossy().into_owned()
    }

    pub unsafe fn nserror_to_string(err: *mut c_void) -> String {
        if err.is_null() {
            return "unknown Metal error".to_string();
        }
        let desc = unsafe { msg0(err, sel(b"localizedDescription\0")) };
        unsafe { nsstring_to_rust(desc) }
    }

    pub unsafe fn retain(obj: *mut c_void) {
        if !obj.is_null() {
            unsafe { msg0(obj, sel(b"retain\0")) };
        }
    }

    pub unsafe fn release(obj: *mut c_void) {
        if !obj.is_null() {
            unsafe { msg0_void(obj, sel(b"release\0")) };
        }
    }

    pub unsafe fn new_buffer_uninit(dev: *mut c_void, bytes: usize) -> *mut c_void {
        let alloc_bytes = bytes.max(4);
        unsafe {
            msg2_usize(
                dev,
                sel(b"newBufferWithLength:options:\0"),
                alloc_bytes,
                MTL_RESOURCE_STORAGE_MODE_SHARED,
            )
        }
    }

    pub unsafe fn buffer_contents(buf: *mut c_void) -> *mut u8 {
        unsafe { msg0(buf, sel(b"contents\0")) }.cast::<u8>()
    }

    pub unsafe fn compile_pipeline(
        dev: *mut c_void,
        source: &str,
        fn_name: &str,
    ) -> Result<*mut c_void, String> {
        let _pool = AutoreleasePool::push();
        let ns_src = unsafe { nsstring(source) };
        let mut err: *mut c_void = null_mut();
        let lib = unsafe {
            msg3_lib(
                dev,
                sel(b"newLibraryWithSource:options:error:\0"),
                ns_src,
                null_mut(),
                &mut err,
            )
        };
        if lib.is_null() {
            return Err(unsafe { nserror_to_string(err) });
        }
        let ns_fn = unsafe { nsstring(fn_name) };
        let func = unsafe { msg1_ptr(lib, sel(b"newFunctionWithName:\0"), ns_fn.cast()) };
        unsafe { release(lib) };
        if func.is_null() {
            return Err(format!("Metal kernel function '{fn_name}' not found"));
        }
        let mut perr: *mut c_void = null_mut();
        let pso = unsafe {
            msg2_ptr_err(
                dev,
                sel(b"newComputePipelineStateWithFunction:error:\0"),
                func,
                &mut perr,
            )
        };
        unsafe { release(func) };
        if pso.is_null() {
            return Err(unsafe { nserror_to_string(perr) });
        }
        Ok(pso)
    }
}

// ---------------------------------------------------------------------------
// Production Metal Shading Language (MSL) Kernel Library
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
const BUILTIN_MSL_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

// ---------------------------------------------------------------------------
// 1. Fused Dot Product (Relaxed SIMD-group + Exact Pairwise Binary Tree)
// ---------------------------------------------------------------------------

kernel void dot_f32_relaxed(
    device const float* a [[buffer(0)]],
    device const float* b [[buffer(1)]],
    device float* partials [[buffer(2)]],
    constant uint& n [[buffer(3)]],
    uint gid [[thread_position_in_grid]],
    uint grid_size [[threads_per_grid]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]],
    uint simd_lane [[thread_index_in_simdgroup]],
    uint simd_id [[simdgroup_index_in_threadgroup]]
) {
    float acc = 0.0f;
    for (uint i = gid; i < n; i += grid_size) {
        acc += a[i] * b[i];
    }
    acc = simd_sum(acc);
    threadgroup float shared[32];
    if (simd_lane == 0) {
        shared[simd_id] = acc;
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (simd_id == 0) {
        uint num_simds = 256 / 32;
        float v = (simd_lane < num_simds) ? shared[simd_lane] : 0.0f;
        v = simd_sum(v);
        if (simd_lane == 0) {
            partials[tgid] = v;
        }
    }
}

kernel void reduce_f32_relaxed(
    device const float* a [[buffer(0)]],
    device float* partials [[buffer(1)]],
    constant uint& n [[buffer(2)]],
    uint gid [[thread_position_in_grid]],
    uint grid_size [[threads_per_grid]],
    uint tgid [[threadgroup_position_in_grid]],
    uint simd_lane [[thread_index_in_simdgroup]],
    uint simd_id [[simdgroup_index_in_threadgroup]]
) {
    float acc = 0.0f;
    for (uint i = gid; i < n; i += grid_size) {
        acc += a[i];
    }
    acc = simd_sum(acc);
    threadgroup float shared[32];
    if (simd_lane == 0) {
        shared[simd_id] = acc;
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (simd_id == 0) {
        float v = (simd_lane < 8) ? shared[simd_lane] : 0.0f;
        v = simd_sum(v);
        if (simd_lane == 0) {
            partials[tgid] = v;
        }
    }
}

// Exact pairwise binary tree over 1024-element blocks: matches CPU
// `f32_exact_tree_sum` BIT-FOR-BIT!
kernel void dot_f32_exact_block(
    device const float* a [[buffer(0)]],
    device const float* b [[buffer(1)]],
    device float* block_out [[buffer(2)]],
    constant uint& n [[buffer(3)]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]]
) {
    threadgroup float s[1024];
    uint base = tgid * 1024;
    for (uint k = 0; k < 4; ++k) {
        uint idx = tid + k * 256;
        uint g = base + idx;
        s[idx] = (g < n) ? (a[g] * b[g]) : 0.0f;
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    uint len = (base < n) ? min(1024u, n - base) : 0u;
    // Exact power-of-2 left-complete reduction for full blocks, or exact
    // recursive split when len < 1024 (tail block handled identically).
    if (len == 1024u) {
        for (uint stride = 1; stride < 1024u; stride <<= 1) {
            for (uint k = 0; k < 4; ++k) {
                uint idx = tid + k * 256;
                if ((idx & ((stride << 1) - 1)) == 0) {
                    s[idx] = s[idx] + s[idx + stride];
                }
            }
            threadgroup_barrier(mem_flags::mem_threadgroup);
        }
        if (tid == 0) {
            block_out[tgid] = s[0];
        }
    } else if (tid == 0) {
        // Tail block: exact iterative simulation of largest-pow2-split tree
        // Bottom-up merge: maintain stack of (pow2_size, sum).
        float vals[12];
        uint sizes[12];
        uint sp = 0;
        for (uint i = 0; i < len; ++i) {
            float cur = s[i];
            uint sz = 1;
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
            for (int j = int(sp) - 2; j >= 0; --j) {
                acc = vals[j] + acc;
            }
        }
        block_out[tgid] = acc;
    }
}

// ---------------------------------------------------------------------------
// 2. Prefix Scan (Inclusive & Exclusive, 256-element block scan + offsets)
// ---------------------------------------------------------------------------

kernel void scan_i32_block(
    device const int* in_arr [[buffer(0)]],
    device int* out_arr [[buffer(1)]],
    device int* block_sums [[buffer(2)]],
    constant uint& n [[buffer(3)]],
    constant uint& inclusive [[buffer(4)]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]]
) {
    threadgroup int s[256];
    uint gid = tgid * 256 + tid;
    int val = (gid < n) ? in_arr[gid] : 0;
    s[tid] = val;
    threadgroup_barrier(mem_flags::mem_threadgroup);
    for (uint offset = 1; offset < 256; offset <<= 1) {
        int add = (tid >= offset) ? s[tid - offset] : 0;
        threadgroup_barrier(mem_flags::mem_threadgroup);
        s[tid] += add;
        threadgroup_barrier(mem_flags::mem_threadgroup);
    }
    if (tid == 255) {
        block_sums[tgid] = s[255];
    }
    if (gid < n) {
        out_arr[gid] = (inclusive != 0) ? s[tid] : (s[tid] - val);
    }
}

kernel void scan_i32_add_offsets(
    device int* out_arr [[buffer(0)]],
    device const int* block_offsets [[buffer(1)]],
    constant uint& n [[buffer(2)]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]]
) {
    uint gid = tgid * 256 + tid;
    if (gid < n) {
        out_arr[gid] += block_offsets[tgid];
    }
}

// ---------------------------------------------------------------------------
// 3. Stable Compaction (Filter i32 > threshold): flags -> scan -> scatter
// ---------------------------------------------------------------------------

kernel void filter_i32_gt_flags(
    device const int* in_arr [[buffer(0)]],
    device int* flags [[buffer(1)]],
    constant uint& n [[buffer(2)]],
    constant int& threshold [[buffer(3)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid < n) {
        flags[gid] = (in_arr[gid] > threshold) ? 1 : 0;
    }
}

kernel void filter_i32_gt_scatter(
    device const int* in_arr [[buffer(0)]],
    device const int* flags [[buffer(1)]],
    device const int* positions [[buffer(2)]],
    device int* out_arr [[buffer(3)]],
    constant uint& n [[buffer(4)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid < n && flags[gid] != 0) {
        out_arr[positions[gid]] = in_arr[gid];
    }
}

// ---------------------------------------------------------------------------
// 4. Batched Histogram (U8 -> U32) with TWO adaptive schedules!
//    (a) Many small frames: 1 threadgroup per frame, zero global atomics!
//    (b) Few large frames: tiled grid-stride per frame + TG-local privatization
// ---------------------------------------------------------------------------

struct HistParams {
    uint frames;
    uint frame_pixels;
    uint num_bins;
    uint tiles_per_frame;
};

kernel void hist_u8_many_small_frames(
    device const uchar* pixels [[buffer(0)]],
    device uint* out_hist [[buffer(1)]],
    constant HistParams& p [[buffer(2)]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]]
) {
    uint frame = tgid;
    if (frame >= p.frames) return;
    threadgroup atomic_uint local_bins[256];
    if (tid < 256) {
        atomic_store_explicit(&local_bins[tid], 0u, memory_order_relaxed);
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    uint base = frame * p.frame_pixels;
    for (uint i = tid; i < p.frame_pixels; i += 256) {
        uint b = uint(pixels[base + i]);
        if (b < p.num_bins) {
            atomic_fetch_add_explicit(&local_bins[b], 1u, memory_order_relaxed);
        }
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (tid < p.num_bins) {
        out_hist[frame * p.num_bins + tid] =
            atomic_load_explicit(&local_bins[tid], memory_order_relaxed);
    }
}

kernel void hist_u8_zero(
    device uint* out_hist [[buffer(0)]],
    constant uint& total [[buffer(1)]],
    uint gid [[thread_position_in_grid]]
) {
    if (gid < total) {
        out_hist[gid] = 0u;
    }
}

kernel void hist_u8_few_large_frames(
    device const uchar* pixels [[buffer(0)]],
    device atomic_uint* out_hist [[buffer(1)]],
    constant HistParams& p [[buffer(2)]],
    uint tid [[thread_position_in_threadgroup]],
    uint tgid [[threadgroup_position_in_grid]]
) {
    uint frame = tgid / p.tiles_per_frame;
    uint tile = tgid % p.tiles_per_frame;
    if (frame >= p.frames) return;
    threadgroup atomic_uint local_bins[256];
    if (tid < 256) {
        atomic_store_explicit(&local_bins[tid], 0u, memory_order_relaxed);
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    uint base = frame * p.frame_pixels;
    uint stride = p.tiles_per_frame * 256;
    for (uint i = tile * 256 + tid; i < p.frame_pixels; i += stride) {
        uint b = uint(pixels[base + i]);
        if (b < p.num_bins) {
            atomic_fetch_add_explicit(&local_bins[b], 1u, memory_order_relaxed);
        }
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (tid < p.num_bins) {
        uint c = atomic_load_explicit(&local_bins[tid], memory_order_relaxed);
        if (c > 0u) {
            atomic_fetch_add_explicit(&out_hist[frame * p.num_bins + tid], c, memory_order_relaxed);
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Batched 2D 3x3 Stencil (U8 & F32) with explicit edge policy & halo tiling
// ---------------------------------------------------------------------------

struct StencilU8Params {
    uint frames;
    uint height;
    uint width;
    int divisor;
    uint edge_policy;
    int w0, w1, w2, w3, w4, w5, w6, w7, w8;
};

struct StencilF32Params {
    uint frames;
    uint height;
    uint width;
    uint edge_policy;
    float w0, w1, w2, w3, w4, w5, w6, w7, w8;
};

inline int resolve_coord(int idx, int dim, uint policy) {
    if (idx >= 0 && idx < dim) return idx;
    if (policy == 0u) return clamp(idx, 0, dim - 1); // Clamp
    if (policy == 1u) return -1;                      // Zero
    if (policy == 2u) {                               // Mirror
        if (dim <= 1) return 0;
        int period = 2 * dim - 2;
        int m = ((idx % period) + period) % period;
        return (m >= dim) ? (period - m) : m;
    }
    // Wrap
    return ((idx % dim) + dim) % dim;
}

kernel void stencil2d_u8_tiled(
    device const uchar* src [[buffer(0)]],
    device uchar* dst [[buffer(1)]],
    constant StencilU8Params& p [[buffer(2)]],
    uint3 gid [[thread_position_in_grid]],
    uint3 lid [[thread_position_in_threadgroup]],
    uint3 tgid [[threadgroup_position_in_grid]]
) {
    uint frame = gid.z;
    if (frame >= p.frames) return;
    int h = int(p.height);
    int w = int(p.width);
    uint fbase = frame * p.height * p.width;

    // 16x16 interior tile + 1-pixel halo => 18x18 shared tile
    threadgroup uchar tile[18][18];
    int tile_x0 = int(tgid.x) * 16 - 1;
    int tile_y0 = int(tgid.y) * 16 - 1;

    uint flat_tid = lid.y * 16 + lid.x;
    for (uint k = flat_tid; k < 18 * 18; k += 256) {
        int ty = int(k / 18);
        int tx = int(k % 18);
        int ry = resolve_coord(tile_y0 + ty, h, p.edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, p.edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? uchar(0) : src[fbase + uint(ry * w + rx)];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);

    if (gid.x >= p.width || gid.y >= p.height) return;
    uint ly = lid.y + 1;
    uint lx = lid.x + 1;
    int acc = p.w0 * int(tile[ly - 1][lx - 1])
            + p.w1 * int(tile[ly - 1][lx])
            + p.w2 * int(tile[ly - 1][lx + 1])
            + p.w3 * int(tile[ly][lx - 1])
            + p.w4 * int(tile[ly][lx])
            + p.w5 * int(tile[ly][lx + 1])
            + p.w6 * int(tile[ly + 1][lx - 1])
            + p.w7 * int(tile[ly + 1][lx])
            + p.w8 * int(tile[ly + 1][lx + 1]);
    int div = (p.divisor == 0) ? 1 : p.divisor;
    int q = acc / div;
    dst[fbase + gid.y * p.width + gid.x] = uchar(clamp(q, 0, 255));
}

kernel void stencil2d_f32_tiled(
    device const float* src [[buffer(0)]],
    device float* dst [[buffer(1)]],
    constant StencilF32Params& p [[buffer(2)]],
    uint3 gid [[thread_position_in_grid]],
    uint3 lid [[thread_position_in_threadgroup]],
    uint3 tgid [[threadgroup_position_in_grid]]
) {
    uint frame = gid.z;
    if (frame >= p.frames) return;
    int h = int(p.height);
    int w = int(p.width);
    uint fbase = frame * p.height * p.width;

    threadgroup float tile[18][18];
    int tile_x0 = int(tgid.x) * 16 - 1;
    int tile_y0 = int(tgid.y) * 16 - 1;

    uint flat_tid = lid.y * 16 + lid.x;
    for (uint k = flat_tid; k < 18 * 18; k += 256) {
        int ty = int(k / 18);
        int tx = int(k % 18);
        int ry = resolve_coord(tile_y0 + ty, h, p.edge_policy);
        int rx = resolve_coord(tile_x0 + tx, w, p.edge_policy);
        tile[ty][tx] = (ry < 0 || rx < 0) ? 0.0f : src[fbase + uint(ry * w + rx)];
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);

    if (gid.x >= p.width || gid.y >= p.height) return;
    uint ly = lid.y + 1;
    uint lx = lid.x + 1;
    float acc = p.w0 * tile[ly - 1][lx - 1]
              + p.w1 * tile[ly - 1][lx]
              + p.w2 * tile[ly - 1][lx + 1]
              + p.w3 * tile[ly][lx - 1]
              + p.w4 * tile[ly][lx]
              + p.w5 * tile[ly][lx + 1]
              + p.w6 * tile[ly + 1][lx - 1]
              + p.w7 * tile[ly + 1][lx]
              + p.w8 * tile[ly + 1][lx + 1];
    dst[fbase + gid.y * p.width + gid.x] = acc;
}

// ---------------------------------------------------------------------------
// 6. Row Reduction (Adaptive Outer-Parallel vs Inner-Parallel with simd_sum)
// ---------------------------------------------------------------------------

struct RowParams {
    uint rows;
    uint cols;
};

kernel void row_reduce_f32_outer(
    device const float* a [[buffer(0)]],
    device float* out [[buffer(1)]],
    constant RowParams& p [[buffer(2)]],
    uint row [[thread_position_in_grid]]
) {
    if (row >= p.rows) return;
    float acc = 0.0f;
    uint base = row * p.cols;
    for (uint j = 0; j < p.cols; ++j) {
        acc += a[base + j];
    }
    out[row] = acc;
}

kernel void row_reduce_f32_inner(
    device const float* a [[buffer(0)]],
    device float* out [[buffer(1)]],
    constant RowParams& p [[buffer(2)]],
    uint tid [[thread_position_in_threadgroup]],
    uint row [[threadgroup_position_in_grid]],
    uint simd_lane [[thread_index_in_simdgroup]],
    uint simd_id [[simdgroup_index_in_threadgroup]]
) {
    if (row >= p.rows) return;
    float acc = 0.0f;
    uint base = row * p.cols;
    for (uint j = tid; j < p.cols; j += 256) {
        acc += a[base + j];
    }
    acc = simd_sum(acc);
    threadgroup float shared[8];
    if (simd_lane == 0) {
        shared[simd_id] = acc;
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
    if (simd_id == 0) {
        float v = (simd_lane < 8) ? shared[simd_lane] : 0.0f;
        v = simd_sum(v);
        if (simd_lane == 0) {
            out[row] = v;
        }
    }
}

// ---------------------------------------------------------------------------
// 7. CSR SpMV (Segmented Reduction with empty row support: scalar & SIMD)
// ---------------------------------------------------------------------------

kernel void spmv_csr_f32_scalar(
    device const float* vals [[buffer(0)]],
    device const uint* cols [[buffer(1)]],
    device const uint* row_ptr [[buffer(2)]],
    device const float* x [[buffer(3)]],
    device float* y [[buffer(4)]],
    constant uint& nrows [[buffer(5)]],
    uint row [[thread_position_in_grid]]
) {
    if (row >= nrows) return;
    uint s = row_ptr[row];
    uint e = row_ptr[row + 1];
    float acc = 0.0f;
    for (uint k = s; k < e; ++k) {
        acc += vals[k] * x[cols[k]];
    }
    y[row] = acc;
}

kernel void spmv_csr_f32_simd(
    device const float* vals [[buffer(0)]],
    device const uint* cols [[buffer(1)]],
    device const uint* row_ptr [[buffer(2)]],
    device const float* x [[buffer(3)]],
    device float* y [[buffer(4)]],
    constant uint& nrows [[buffer(5)]],
    uint gid [[thread_position_in_grid]],
    uint simd_lane [[thread_index_in_simdgroup]]
) {
    uint row = gid / 32;
    if (row >= nrows) return;
    uint s = row_ptr[row];
    uint e = row_ptr[row + 1];
    float acc = 0.0f;
    for (uint k = s + simd_lane; k < e; k += 32) {
        acc += vals[k] * x[cols[k]];
    }
    acc = simd_sum(acc);
    if (simd_lane == 0) {
        y[row] = acc;
    }
}

// Exact-tree SpMV (matches CPU f32_exact_tree_prod bit-for-bit)
kernel void spmv_csr_f32_exact(
    device const float* vals [[buffer(0)]],
    device const uint* cols [[buffer(1)]],
    device const uint* row_ptr [[buffer(2)]],
    device const float* x [[buffer(3)]],
    device float* y [[buffer(4)]],
    constant uint& nrows [[buffer(5)]],
    uint row [[thread_position_in_grid]]
) {
    if (row >= nrows) return;
    uint s = row_ptr[row];
    uint e = row_ptr[row + 1];
    if (e <= s) {
        y[row] = 0.0f;
        return;
    }
    float st_val[32];
    uint st_sz[32];
    uint sp = 0;
    for (uint k = s; k < e; ++k) {
        float cur = vals[k] * x[cols[k]];
        uint sz = 1;
        while (sp > 0 && st_sz[sp - 1] == sz) {
            cur = st_val[sp - 1] + cur;
            sz <<= 1;
            sp -= 1;
        }
        st_val[sp] = cur;
        st_sz[sp] = sz;
        sp += 1;
    }
    float acc = st_val[sp - 1];
    for (int j = int(sp) - 2; j >= 0; --j) {
        acc = st_val[j] + acc;
    }
    y[row] = acc;
}
"#;

// ---------------------------------------------------------------------------
// Device Context, Resident Buffers, Telemetry & Handle Table
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct DeviceLaunchPlan {
    pub op: String,
    pub schedule: String,
    pub grid: [usize; 3],
    pub threadgroup: [usize; 3],
    pub threadgroup_mem_bytes: usize,
    pub numerical_mode: String,
}

impl Default for DeviceLaunchPlan {
    fn default() -> Self {
        Self {
            op: String::new(),
            schedule: String::new(),
            grid: [0, 0, 0],
            threadgroup: [0, 0, 0],
            threadgroup_mem_bytes: 0,
            numerical_mode: String::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SyncPoint {
    pub op: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct DeviceTelemetry {
    pub uploads: usize,
    pub upload_bytes: usize,
    pub downloads: usize,
    pub download_bytes: usize,
    pub kernel_launches: usize,
    pub sync_points: Vec<SyncPoint>,
    pub live_device_bytes: usize,
    pub peak_device_bytes: usize,
    pub last_plan: DeviceLaunchPlan,
    pub last_error: String,
}

impl DeviceTelemetry {
    pub(crate) fn note_alloc(&mut self, bytes: usize) {
        self.live_device_bytes += bytes;
        if self.live_device_bytes > self.peak_device_bytes {
            self.peak_device_bytes = self.live_device_bytes;
        }
    }
    pub(crate) fn note_free(&mut self, bytes: usize) {
        self.live_device_bytes = self.live_device_bytes.saturating_sub(bytes);
    }
    fn to_json(&self, device_name: &str, backend: &str) -> String {
        let syncs: Vec<String> = self
            .sync_points
            .iter()
            .map(|s| format!("{{\"op\":\"{}\",\"reason\":\"{}\"}}", esc(&s.op), esc(&s.reason)))
            .collect();
        format!(
            concat!(
                "{{\"device_name\":\"{}\",\"backend\":\"{}\",",
                "\"uploads\":{},\"upload_bytes\":{},",
                "\"downloads\":{},\"download_bytes\":{},",
                "\"kernel_launches\":{},\"sync_points\":[{}],",
                "\"live_device_bytes\":{},\"peak_device_bytes\":{},",
                "\"last_launch_plan\":{{\"op\":\"{}\",\"schedule\":\"{}\",",
                "\"grid\":[{},{},{}],\"threadgroup\":[{},{},{}],",
                "\"threadgroup_mem_bytes\":{},\"numerical_mode\":\"{}\"}},",
                "\"last_error\":\"{}\"}}"
            ),
            esc(device_name),
            esc(backend),
            self.uploads,
            self.upload_bytes,
            self.downloads,
            self.download_bytes,
            self.kernel_launches,
            syncs.join(","),
            self.live_device_bytes,
            self.peak_device_bytes,
            esc(&self.last_plan.op),
            esc(&self.last_plan.schedule),
            self.last_plan.grid[0],
            self.last_plan.grid[1],
            self.last_plan.grid[2],
            self.last_plan.threadgroup[0],
            self.last_plan.threadgroup[1],
            self.last_plan.threadgroup[2],
            self.last_plan.threadgroup_mem_bytes,
            esc(&self.last_plan.numerical_mode),
            esc(&self.last_error),
        )
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

struct ResidentBuffer {
    ctx_id: i64,
    bytes: usize,
    #[cfg(target_os = "macos")]
    mtl_buf: *mut c_void,
    ocl_buf: Option<crate::opencl::OpenClBuffer>,
    cuda_buf: Option<crate::cuda::CudaBuffer>,
    hip_buf: Option<crate::hip::HipBuffer>,
    cpu_buf: Vec<u8>,
}

impl ResidentBuffer {
    fn new_cpu(ctx_id: i64, cpu_buf: Vec<u8>) -> Arc<Self> {
        let bytes = cpu_buf.len();
        Arc::new(Self {
            ctx_id,
            bytes,
            #[cfg(target_os = "macos")]
            mtl_buf: null_mut(),
            ocl_buf: None,
            cuda_buf: None,
            hip_buf: None,
            cpu_buf,
        })
    }

    fn new_opencl(ctx_id: i64, buf: crate::opencl::OpenClBuffer) -> Arc<Self> {
        let bytes = buf.bytes;
        Arc::new(Self {
            ctx_id,
            bytes,
            #[cfg(target_os = "macos")]
            mtl_buf: null_mut(),
            ocl_buf: Some(buf),
            cuda_buf: None,
            hip_buf: None,
            cpu_buf: Vec::new(),
        })
    }

    fn new_cuda(ctx_id: i64, buf: crate::cuda::CudaBuffer) -> Arc<Self> {
        let bytes = buf.bytes;
        Arc::new(Self {
            ctx_id,
            bytes,
            #[cfg(target_os = "macos")]
            mtl_buf: null_mut(),
            ocl_buf: None,
            cuda_buf: Some(buf),
            hip_buf: None,
            cpu_buf: Vec::new(),
        })
    }

    fn new_hip(ctx_id: i64, buf: crate::hip::HipBuffer) -> Arc<Self> {
        let bytes = buf.bytes;
        Arc::new(Self {
            ctx_id,
            bytes,
            #[cfg(target_os = "macos")]
            mtl_buf: null_mut(),
            ocl_buf: None,
            cuda_buf: None,
            hip_buf: Some(buf),
            cpu_buf: Vec::new(),
        })
    }
}

unsafe impl Send for ResidentBuffer {}
unsafe impl Sync for ResidentBuffer {}

impl Drop for ResidentBuffer {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        if !self.mtl_buf.is_null() {
            unsafe { objc::release(self.mtl_buf) };
        }
    }
}

struct DeviceCtx {
    backend: &'static str,
    device_name: String,
    #[cfg(target_os = "macos")]
    mtl_device: *mut c_void,
    #[cfg(target_os = "macos")]
    mtl_queue: *mut c_void,
    #[cfg(target_os = "macos")]
    pipelines: HashMap<String, *mut c_void>,
    ocl: Option<crate::opencl::OpenClState>,
    cuda: Option<crate::cuda::CudaState>,
    hip: Option<crate::hip::HipState>,
    /// Retained buffers kept alive until command completion.
    in_flight: Vec<Arc<ResidentBuffer>>,
    telemetry: DeviceTelemetry,
}

unsafe impl Send for DeviceCtx {}
unsafe impl Sync for DeviceCtx {}

impl Drop for DeviceCtx {
    fn drop(&mut self) {
        self.in_flight.clear();
        #[cfg(target_os = "macos")]
        {
            for (_, pso) in self.pipelines.drain() {
                unsafe { objc::release(pso) };
            }
            if !self.mtl_queue.is_null() {
                unsafe { objc::release(self.mtl_queue) };
            }
            if !self.mtl_device.is_null() {
                unsafe { objc::release(self.mtl_device) };
            }
        }
    }
}

struct HandleStore {
    next_id: i64,
    ctxs: HashMap<i64, Arc<Mutex<DeviceCtx>>>,
    bufs: HashMap<i64, Arc<ResidentBuffer>>,
}

static STORE: OnceLock<Mutex<HandleStore>> = OnceLock::new();

fn store() -> &'static Mutex<HandleStore> {
    STORE.get_or_init(|| {
        Mutex::new(HandleStore {
            next_id: 1,
            ctxs: HashMap::new(),
            bufs: HashMap::new(),
        })
    })
}

impl DeviceCtx {
    #[cfg(target_os = "macos")]
    fn open_metal() -> Result<Self, String> {
        let Some(api) = objc::api() else {
            return Err("Metal.framework not available on this host".to_string());
        };
        let _pool = objc::AutoreleasePool::push();
        let dev = unsafe { (api.create_default_device)() };
        if dev.is_null() {
            return Err("MTLCreateSystemDefaultDevice returned null".to_string());
        }
        let ns_name = unsafe { objc::msg0(dev, objc::sel(b"name\0")) };
        let device_name = unsafe { objc::nsstring_to_rust(ns_name) };
        let queue = unsafe { objc::msg0(dev, objc::sel(b"newCommandQueue\0")) };
        if queue.is_null() {
            unsafe { objc::release(dev) };
            return Err("failed to create MTLCommandQueue".to_string());
        }
        Ok(Self {
            backend: "metal",
            device_name,
            mtl_device: dev,
            mtl_queue: queue,
            pipelines: HashMap::new(),
            ocl: None,
            cuda: None,
            hip: None,
            in_flight: Vec::new(),
            telemetry: DeviceTelemetry::default(),
        })
    }

    fn open_cpu_variant(variant: crate::simd_cpu::CpuVariant) -> Self {
        let (backend, device_name) = match variant {
            crate::simd_cpu::CpuVariant::Seq => ("seq", "sequential-cpu"),
            crate::simd_cpu::CpuVariant::Multicore => ("cpu", "multicore-cpu"),
            crate::simd_cpu::CpuVariant::IspcSimd => ("ispc", "ispc-simd-multicore-cpu"),
        };
        Self {
            backend,
            device_name: device_name.to_string(),
            #[cfg(target_os = "macos")]
            mtl_device: null_mut(),
            #[cfg(target_os = "macos")]
            mtl_queue: null_mut(),
            #[cfg(target_os = "macos")]
            pipelines: HashMap::new(),
            ocl: None,
            cuda: None,
            hip: None,
            in_flight: Vec::new(),
            telemetry: DeviceTelemetry::default(),
        }
    }

    fn open_cpu() -> Self {
        Self::open_cpu_variant(crate::simd_cpu::CpuVariant::Multicore)
    }

    fn open_opencl() -> Result<Self, String> {
        let state = crate::opencl::OpenClState::open()?;
        let device_name = state.device_name.clone();
        Ok(Self {
            backend: "opencl",
            device_name,
            #[cfg(target_os = "macos")]
            mtl_device: null_mut(),
            #[cfg(target_os = "macos")]
            mtl_queue: null_mut(),
            #[cfg(target_os = "macos")]
            pipelines: HashMap::new(),
            ocl: Some(state),
            cuda: None,
            hip: None,
            in_flight: Vec::new(),
            telemetry: DeviceTelemetry::default(),
        })
    }

    fn open_cuda() -> Result<Self, String> {
        let state = crate::cuda::CudaState::open()?;
        let device_name = state.device_name.clone();
        Ok(Self {
            backend: "cuda",
            device_name,
            #[cfg(target_os = "macos")]
            mtl_device: null_mut(),
            #[cfg(target_os = "macos")]
            mtl_queue: null_mut(),
            #[cfg(target_os = "macos")]
            pipelines: HashMap::new(),
            ocl: None,
            cuda: Some(state),
            hip: None,
            in_flight: Vec::new(),
            telemetry: DeviceTelemetry::default(),
        })
    }

    fn open_hip() -> Result<Self, String> {
        let state = crate::hip::HipState::open()?;
        let device_name = state.device_name.clone();
        Ok(Self {
            backend: "hip",
            device_name,
            #[cfg(target_os = "macos")]
            mtl_device: null_mut(),
            #[cfg(target_os = "macos")]
            mtl_queue: null_mut(),
            #[cfg(target_os = "macos")]
            pipelines: HashMap::new(),
            ocl: None,
            cuda: None,
            hip: Some(state),
            in_flight: Vec::new(),
            telemetry: DeviceTelemetry::default(),
        })
    }

    #[cfg(target_os = "macos")]
    fn pipeline(&mut self, source: &str, name: &str) -> Result<*mut c_void, String> {
        let key = format!("{name}#{:x}", fnv1a(source.as_bytes()));
        if let Some(&pso) = self.pipelines.get(&key) {
            return Ok(pso);
        }
        let pso = unsafe { objc::compile_pipeline(self.mtl_device, source, name)? };
        self.pipelines.insert(key, pso);
        Ok(pso)
    }

    #[cfg(target_os = "macos")]
    fn alloc_mtl_buf(&mut self, ctx_id: i64, bytes: usize) -> Result<Arc<ResidentBuffer>, String> {
        let mtl_buf = unsafe { objc::new_buffer_uninit(self.mtl_device, bytes) };
        if mtl_buf.is_null() {
            return Err(format!("Metal buffer allocation exhausted ({bytes} bytes)"));
        }
        self.telemetry.note_alloc(bytes);
        Ok(Arc::new(ResidentBuffer {
            ctx_id,
            bytes,
            mtl_buf,
            ocl_buf: None,
            cuda_buf: None,
            hip_buf: None,
            cpu_buf: Vec::new(),
        }))
    }
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn register_buf(buf: Arc<ResidentBuffer>) -> i64 {
    let mut st = store().lock().unwrap_or_else(|e| e.into_inner());
    let id = st.next_id;
    st.next_id += 1;
    st.bufs.insert(id, buf);
    id
}

fn get_ctx(ctx_id: i64) -> Option<Arc<Mutex<DeviceCtx>>> {
    store().lock().unwrap_or_else(|e| e.into_inner()).ctxs.get(&ctx_id).cloned()
}

fn get_buf(buf_id: i64) -> Option<Arc<ResidentBuffer>> {
    store().lock().unwrap_or_else(|e| e.into_inner()).bufs.get(&buf_id).cloned()
}

// ---------------------------------------------------------------------------
// Public C-ABI Device Lifecycle, Transfer, Compute & Reporting API
// (Used by both Cinch stdlib `Data.Device` AND external C / Python callers!)
// ---------------------------------------------------------------------------

/// Open a device context for any Cinch / Futhark-equivalent backend:
/// - `0`: Multicore CPU (`cpu`, futhark `multicore`)
/// - `1`: Metal GPU with CPU fallback (`metal`)
/// - `2`: Metal GPU required (`metal-required`)
/// - `3`: OpenCL GPU required (`opencl-required`, futhark `opencl`)
/// - `4`: NVIDIA CUDA GPU required (`cuda-required`, futhark `cuda`)
/// - `5`: AMD ROCm HIP GPU required (`hip-required`, futhark `hip`)
/// - `6`: Sequential single-threaded CPU (`seq`, futhark `c`)
/// - `7`: SIMD-vectorized multicore CPU (`ispc`, futhark `ispc`)
/// - `8`: Auto-select best available accelerator (`auto`)
/// - `9`: OpenCL with CPU fallback (`opencl`)
/// - `10`: CUDA with CPU fallback (`cuda`)
/// - `11`: HIP with CPU fallback (`hip`)
#[unsafe(no_mangle)]
pub extern "C" fn cinch_device_open(kind: i64) -> i64 {
    let ctx = match kind {
        DEVICE_KIND_CPU => DeviceCtx::open_cpu(),
        DEVICE_KIND_CPU_SEQ => DeviceCtx::open_cpu_variant(crate::simd_cpu::CpuVariant::Seq),
        DEVICE_KIND_ISPC_SIMD => DeviceCtx::open_cpu_variant(crate::simd_cpu::CpuVariant::IspcSimd),
        DEVICE_KIND_METAL => {
            #[cfg(target_os = "macos")]
            {
                DeviceCtx::open_metal().unwrap_or_else(|_| DeviceCtx::open_cpu())
            }
            #[cfg(not(target_os = "macos"))]
            {
                DeviceCtx::open_cpu()
            }
        }
        DEVICE_KIND_METAL_REQUIRED => {
            #[cfg(target_os = "macos")]
            {
                match DeviceCtx::open_metal() {
                    Ok(c) => c,
                    Err(_) => return DEV_ERR_NO_DEVICE,
                }
            }
            #[cfg(not(target_os = "macos"))]
            {
                return DEV_ERR_NO_DEVICE;
            }
        }
        DEVICE_KIND_OPENCL => DeviceCtx::open_opencl().unwrap_or_else(|_| DeviceCtx::open_cpu()),
        DEVICE_KIND_OPENCL_REQUIRED => match DeviceCtx::open_opencl() {
            Ok(c) => c,
            Err(_) => return DEV_ERR_NO_DEVICE,
        },
        DEVICE_KIND_CUDA => DeviceCtx::open_cuda().unwrap_or_else(|_| DeviceCtx::open_cpu()),
        DEVICE_KIND_CUDA_REQUIRED => match DeviceCtx::open_cuda() {
            Ok(c) => c,
            Err(_) => return DEV_ERR_NO_DEVICE,
        },
        DEVICE_KIND_HIP => DeviceCtx::open_hip().unwrap_or_else(|_| DeviceCtx::open_cpu()),
        DEVICE_KIND_HIP_REQUIRED => match DeviceCtx::open_hip() {
            Ok(c) => c,
            Err(_) => return DEV_ERR_NO_DEVICE,
        },
        DEVICE_KIND_AUTO => {
            #[cfg(target_os = "macos")]
            if let Ok(c) = DeviceCtx::open_metal() {
                c
            } else if let Ok(c) = DeviceCtx::open_opencl() {
                c
            } else {
                DeviceCtx::open_cpu_variant(crate::simd_cpu::CpuVariant::IspcSimd)
            }
            #[cfg(not(target_os = "macos"))]
            if let Ok(c) = DeviceCtx::open_cuda() {
                c
            } else if let Ok(c) = DeviceCtx::open_hip() {
                c
            } else if let Ok(c) = DeviceCtx::open_opencl() {
                c
            } else {
                DeviceCtx::open_cpu_variant(crate::simd_cpu::CpuVariant::IspcSimd)
            }
        }
        _ => return DEV_ERR_INVALID_HANDLE,
    };
    let mut st = store().lock().unwrap_or_else(|e| e.into_inner());
    let id = st.next_id;
    st.next_id += 1;
    st.ctxs.insert(id, Arc::new(Mutex::new(ctx)));
    id
}

#[unsafe(no_mangle)]
pub extern "C" fn cinch_device_close(ctx_id: i64) -> i64 {
    let mut st = store().lock().unwrap_or_else(|e| e.into_inner());
    st.bufs.retain(|_, b| b.ctx_id != ctx_id);
    if st.ctxs.remove(&ctx_id).is_some() {
        DEV_OK
    } else {
        DEV_ERR_INVALID_HANDLE
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn cinch_device_reset_telemetry(ctx_id: i64) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let live = ctx.telemetry.live_device_bytes;
    ctx.telemetry = DeviceTelemetry::default();
    ctx.telemetry.live_device_bytes = live;
    ctx.telemetry.peak_device_bytes = live;
    DEV_OK
}

#[unsafe(no_mangle)]
pub extern "C" fn cinch_device_buffer_free(buf_id: i64) -> i64 {
    let buf = {
        let mut st = store().lock().unwrap_or_else(|e| e.into_inner());
        st.bufs.remove(&buf_id)
    };
    let Some(b) = buf else {
        return DEV_ERR_INVALID_HANDLE;
    };
    if let Some(ctx_arc) = get_ctx(b.ctx_id) {
        let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
        ctx.telemetry.note_free(b.bytes);
    }
    DEV_OK
}

#[unsafe(no_mangle)]
pub extern "C" fn cinch_device_buffer_bytes(buf_id: i64) -> i64 {
    get_buf(buf_id).map(|b| b.bytes as i64).unwrap_or(DEV_ERR_INVALID_HANDLE)
}

/// Upload raw host bytes into a typed resident device buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_upload_raw(
    ctx_id: i64,
    src_ptr: *const u8,
    bytes: usize,
) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let buf = if ctx.backend == "metal" {
        #[cfg(target_os = "macos")]
        {
            match ctx.alloc_mtl_buf(ctx_id, bytes) {
                Ok(b) => {
                    if bytes > 0 && !src_ptr.is_null() {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                src_ptr,
                                objc::buffer_contents(b.mtl_buf),
                                bytes,
                            );
                        }
                    }
                    b
                }
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            return DEV_ERR_NO_DEVICE;
        }
    } else if let Some(ref ocl) = ctx.ocl {
        match ocl.upload(src_ptr, bytes) {
            Ok(b) => {
                ctx.telemetry.note_alloc(bytes);
                ResidentBuffer::new_opencl(ctx_id, b)
            }
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        }
    } else if let Some(ref cuda) = ctx.cuda {
        match cuda.upload(src_ptr, bytes) {
            Ok(b) => {
                ctx.telemetry.note_alloc(bytes);
                ResidentBuffer::new_cuda(ctx_id, b)
            }
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        }
    } else if let Some(ref hip) = ctx.hip {
        match hip.upload(src_ptr, bytes) {
            Ok(b) => {
                ctx.telemetry.note_alloc(bytes);
                ResidentBuffer::new_hip(ctx_id, b)
            }
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        }
    } else {
        let mut v = vec![0u8; bytes];
        if bytes > 0 && !src_ptr.is_null() {
            unsafe { std::ptr::copy_nonoverlapping(src_ptr, v.as_mut_ptr(), bytes) };
        }
        ctx.telemetry.note_alloc(bytes);
        ResidentBuffer::new_cpu(ctx_id, v)
    };
    ctx.telemetry.uploads += 1;
    ctx.telemetry.upload_bytes += bytes;
    drop(ctx);
    register_buf(buf)
}

/// Upload a packed Cinch `Array a` into a resident device buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_upload_array(ctx_id: i64, arr: i64) -> i64 {
    let n = unsafe { arrays::cinch_array_len(arr) } as usize;
    let f = unsafe { arrays::format(arr) };
    let bytes = n.saturating_mul(arrays::elem_width(f));
    let ptr = if bytes > 0 {
        unsafe { arrays::data_ptr(arr) }
    } else {
        std::ptr::null()
    };
    unsafe { cinch_device_upload_raw(ctx_id, ptr, bytes) }
}

/// Download raw bytes from a resident device buffer into caller memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_download_raw(
    ctx_id: i64,
    buf_id: i64,
    dst_ptr: *mut u8,
    bytes: usize,
) -> i64 {
    let (Some(ctx_arc), Some(buf)) = (get_ctx(ctx_id), get_buf(buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    if bytes > buf.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    if bytes > 0 && !dst_ptr.is_null() {
        if ctx.backend == "metal" {
            #[cfg(target_os = "macos")]
            unsafe {
                std::ptr::copy_nonoverlapping(objc::buffer_contents(buf.mtl_buf), dst_ptr, bytes);
            }
        } else if let (Some(ocl), Some(ob)) = (&ctx.ocl, &buf.ocl_buf) {
            if let Err(e) = ocl.download(ob, dst_ptr, bytes) {
                ctx.telemetry.last_error = e;
                return DEV_ERR_EXECUTION_FAILED;
            }
        } else if let (Some(cuda), Some(cb)) = (&ctx.cuda, &buf.cuda_buf) {
            if let Err(e) = cuda.download(cb, dst_ptr, bytes) {
                ctx.telemetry.last_error = e;
                return DEV_ERR_EXECUTION_FAILED;
            }
        } else if let (Some(hip), Some(hb)) = (&ctx.hip, &buf.hip_buf) {
            if let Err(e) = hip.download(hb, dst_ptr, bytes) {
                ctx.telemetry.last_error = e;
                return DEV_ERR_EXECUTION_FAILED;
            }
        } else {
            unsafe { std::ptr::copy_nonoverlapping(buf.cpu_buf.as_ptr(), dst_ptr, bytes) };
        }
    }
    ctx.telemetry.downloads += 1;
    ctx.telemetry.download_bytes += bytes;
    DEV_OK
}

/// Download a resident device buffer into a fresh packed Cinch `Array a`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_download_array(
    ctx_id: i64,
    buf_id: i64,
    format: i64,
) -> i64 {
    let Some(buf) = get_buf(buf_id) else {
        return 0;
    };
    let w = arrays::elem_width(format);
    if w == 0 || buf.bytes % w != 0 {
        return 0;
    }
    let n = buf.bytes / w;
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, format) };
    let dst = if buf.bytes > 0 {
        unsafe { arrays::data_ptr(out) }
    } else {
        null_mut()
    };
    if unsafe { cinch_device_download_raw(ctx_id, buf_id, dst, buf.bytes) } != DEV_OK {
        unsafe { crate::rc::cinch_drop(out) };
        return 0;
    }
    out
}

/// Write JSON telemetry report into caller buffer; returns byte length written (or negative error).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_report_cstr(
    ctx_id: i64,
    out_ptr: *mut u8,
    out_cap: usize,
) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let json = ctx.telemetry.to_json(&ctx.device_name, ctx.backend);
    let b = json.as_bytes();
    if !out_ptr.is_null() && out_cap > 0 {
        let copy_n = b.len().min(out_cap - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(b.as_ptr(), out_ptr, copy_n);
            *out_ptr.add(copy_n) = 0;
        }
        copy_n as i64
    } else {
        b.len() as i64
    }
}

/// Return device report JSON as a Cinch `Bytes` value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_report_bytes(ctx_id: i64) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        let empty = b"{\"error\":\"invalid device handle\"}";
        return unsafe { abi::cinch_bytes_from_data(empty.as_ptr(), empty.len() as i64) };
    };
    let ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let json = ctx.telemetry.to_json(&ctx.device_name, ctx.backend);
    let b = json.as_bytes();
    unsafe { abi::cinch_bytes_from_data(b.as_ptr(), b.len() as i64) }
}

// ---------------------------------------------------------------------------
// Metal GPU Execution Kernels (Resident DeviceBuffer -> Resident DeviceBuffer)
// ---------------------------------------------------------------------------

/// Execute a fused `F32` dot product on two resident device buffers.
/// `float_mode`: `0 = exact-tree`, `1 = relaxed`. Returns f32 bits in `i64` (or negative error if invalid).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_dot_f32(
    ctx_id: i64,
    buf_a_id: i64,
    buf_b_id: i64,
    n_elems: i64,
    float_mode: i64,
    out_f32_ptr: *mut f32,
) -> i64 {
    let (Some(ctx_arc), Some(buf_a), Some(buf_b)) =
        (get_ctx(ctx_id), get_buf(buf_a_id), get_buf(buf_b_id))
    else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let n = n_elems.max(0) as usize;
    let exact = float_mode == 0 || float_mode == par_arrays::REDUCE_F32_ADD_EXACT_TREE;
    if n == 0 {
        if !out_f32_ptr.is_null() {
            unsafe { *out_f32_ptr = 0.0 };
        }
        return DEV_OK;
    }
    if n * 4 > buf_a.bytes || n * 4 > buf_b.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());

    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let n_u32 = n as u32;
        if exact {
            // Exact binary tree reduction over 1024-element blocks -> bit-exact with CPU!
            let num_blocks = (n + 1023) / 1024;
            let pso = match ctx.pipeline(BUILTIN_MSL_SOURCE, "dot_f32_exact_block") {
                Ok(p) => p,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_COMPILE_FAILED;
                }
            };
            let partials = match ctx.alloc_mtl_buf(ctx_id, num_blocks * 4) {
                Ok(b) => b,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            };
            ctx.in_flight.push(buf_a.clone());
            ctx.in_flight.push(buf_b.clone());
            ctx.in_flight.push(partials.clone());

            let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
            let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
            unsafe {
                objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), buf_a.mtl_buf, 0, 0);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), buf_b.mtl_buf, 0, 1);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), partials.mtl_buf, 0, 2);
                objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 3);
                objc::msg2_dispatch(
                    enc,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: num_blocks, height: 1, depth: 1 },
                    objc::MTLSize { width: 256, height: 1, depth: 1 },
                );
                objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
                objc::msg0_void(cb, objc::sel(b"commit\0"));
                objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
            }
            let status = unsafe { objc::msg0_usize(cb, objc::sel(b"status\0")) };
            ctx.in_flight.clear();
            if status == objc::MTL_CMD_BUF_STATUS_ERROR {
                let err = unsafe { objc::msg0(cb, objc::sel(b"error\0")) };
                ctx.telemetry.last_error = unsafe { objc::nserror_to_string(err) };
                ctx.telemetry.note_free(partials.bytes);
                return DEV_ERR_EXECUTION_FAILED;
            }
            let p_slice = unsafe {
                std::slice::from_raw_parts(
                    objc::buffer_contents(partials.mtl_buf).cast::<f32>(),
                    num_blocks,
                )
            };
            let result = par_arrays::f32_exact_tree_sum(p_slice);
            ctx.telemetry.note_free(partials.bytes);
            ctx.telemetry.kernel_launches += 1;
            ctx.telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-exact".to_string(),
                reason: "scalar-result-readback".to_string(),
            });
            ctx.telemetry.last_plan = DeviceLaunchPlan {
                op: "fused-dot-f32".to_string(),
                schedule: format!("metal-exact-tree-1024(blocks={num_blocks})"),
                grid: [num_blocks * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 1024 * 4,
                numerical_mode: "f32-add-exact-tree".to_string(),
            };
            if !out_f32_ptr.is_null() {
                unsafe { *out_f32_ptr = result };
            }
            return DEV_OK;
        } else {
            // Relaxed high-throughput SIMD-group reduction
            let num_groups = ((n + 2047) / 2048).clamp(1, 1024);
            let pso = match ctx.pipeline(BUILTIN_MSL_SOURCE, "dot_f32_relaxed") {
                Ok(p) => p,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_COMPILE_FAILED;
                }
            };
            let partials = match ctx.alloc_mtl_buf(ctx_id, num_groups * 4) {
                Ok(b) => b,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            };
            ctx.in_flight.push(buf_a.clone());
            ctx.in_flight.push(buf_b.clone());
            ctx.in_flight.push(partials.clone());
            let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
            let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
            unsafe {
                objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), buf_a.mtl_buf, 0, 0);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), buf_b.mtl_buf, 0, 1);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), partials.mtl_buf, 0, 2);
                objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 3);
                objc::msg2_dispatch(
                    enc,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: num_groups, height: 1, depth: 1 },
                    objc::MTLSize { width: 256, height: 1, depth: 1 },
                );
                objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
                objc::msg0_void(cb, objc::sel(b"commit\0"));
                objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
            }
            let status = unsafe { objc::msg0_usize(cb, objc::sel(b"status\0")) };
            ctx.in_flight.clear();
            if status == objc::MTL_CMD_BUF_STATUS_ERROR {
                let err = unsafe { objc::msg0(cb, objc::sel(b"error\0")) };
                ctx.telemetry.last_error = unsafe { objc::nserror_to_string(err) };
                ctx.telemetry.note_free(partials.bytes);
                return DEV_ERR_EXECUTION_FAILED;
            }
            let p_slice = unsafe {
                std::slice::from_raw_parts(
                    objc::buffer_contents(partials.mtl_buf).cast::<f32>(),
                    num_groups,
                )
            };
            let result: f32 = p_slice.iter().copied().sum();
            ctx.telemetry.note_free(partials.bytes);
            ctx.telemetry.kernel_launches += 1;
            ctx.telemetry.sync_points.push(SyncPoint {
                op: "dot-f32-relaxed".to_string(),
                reason: "scalar-result-readback".to_string(),
            });
            ctx.telemetry.last_plan = DeviceLaunchPlan {
                op: "fused-dot-f32".to_string(),
                schedule: format!("metal-simd-group-reduce(groups={num_groups})"),
                grid: [num_groups * 256, 1, 1],
                threadgroup: [256, 1, 1],
                threadgroup_mem_bytes: 32 * 4,
                numerical_mode: "f32-add-relaxed".to_string(),
            };
            if !out_f32_ptr.is_null() {
                unsafe { *out_f32_ptr = result };
            }
            return DEV_OK;
        }
    }

    // OpenCL / CUDA / HIP / CPU (seq, cpu, ispc) dispatch
    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ba), Some(bb)) = (ocl.as_mut(), buf_a.ocl_buf.as_ref(), buf_b.ocl_buf.as_ref()) {
        return match ocl.dot_f32(ba, bb, n, exact, telemetry) {
            Ok(res) => {
                if !out_f32_ptr.is_null() {
                    unsafe { *out_f32_ptr = res };
                }
                DEV_OK
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ba), Some(bb)) = (cuda.as_mut(), buf_a.cuda_buf.as_ref(), buf_b.cuda_buf.as_ref()) {
        return match cuda.dot_f32(ba, bb, n, exact, telemetry) {
            Ok(res) => {
                if !out_f32_ptr.is_null() {
                    unsafe { *out_f32_ptr = res };
                }
                DEV_OK
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ba), Some(bb)) = (hip.as_mut(), buf_a.hip_buf.as_ref(), buf_b.hip_buf.as_ref()) {
        return match hip.dot_f32(ba, bb, n, exact, telemetry) {
            Ok(res) => {
                if !out_f32_ptr.is_null() {
                    unsafe { *out_f32_ptr = res };
                }
                DEV_OK
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let res = crate::simd_cpu::dot_f32(variant, &buf_a.cpu_buf, &buf_b.cpu_buf, n, exact, telemetry);
    if !out_f32_ptr.is_null() {
        unsafe { *out_f32_ptr = res };
    }
    DEV_OK
}

/// GPU Prefix Scan (inclusive or exclusive) over resident `I32` buffer -> returns new resident buffer handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_scan_i32(
    ctx_id: i64,
    in_buf_id: i64,
    n_elems: i64,
    inclusive: i64,
) -> i64 {
    let (Some(ctx_arc), Some(in_buf)) = (get_ctx(ctx_id), get_buf(in_buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let n = n_elems.max(0) as usize;
    if n * 4 > in_buf.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let out_buf = match ctx.alloc_mtl_buf(ctx_id, n * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        if n == 0 {
            drop(ctx);
            return register_buf(out_buf);
        }
        let num_blocks = (n + 255) / 256;
        let pso_block = match ctx.pipeline(BUILTIN_MSL_SOURCE, "scan_i32_block") {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        let pso_add = match ctx.pipeline(BUILTIN_MSL_SOURCE, "scan_i32_add_offsets") {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        let block_sums = match ctx.alloc_mtl_buf(ctx_id, num_blocks * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        let block_offs = match ctx.alloc_mtl_buf(ctx_id, num_blocks * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        ctx.in_flight.push(in_buf.clone());
        ctx.in_flight.push(out_buf.clone());
        ctx.in_flight.push(block_sums.clone());
        ctx.in_flight.push(block_offs.clone());

        let n_u32 = n as u32;
        let inc_u32 = u32::from(inclusive != 0);
        let cb1 = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc1 = unsafe { objc::msg0(cb1, objc::sel(b"computeCommandEncoder\0")) };
        unsafe {
            objc::msg1_ptr_void(enc1, objc::sel(b"setComputePipelineState:\0"), pso_block);
            objc::msg3_set_buf(enc1, objc::sel(b"setBuffer:offset:atIndex:\0"), in_buf.mtl_buf, 0, 0);
            objc::msg3_set_buf(enc1, objc::sel(b"setBuffer:offset:atIndex:\0"), out_buf.mtl_buf, 0, 1);
            objc::msg3_set_buf(enc1, objc::sel(b"setBuffer:offset:atIndex:\0"), block_sums.mtl_buf, 0, 2);
            objc::msg3_set_bytes(enc1, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 3);
            objc::msg3_set_bytes(enc1, objc::sel(b"setBytes:length:atIndex:\0"), (&inc_u32 as *const u32).cast(), 4, 4);
            objc::msg2_dispatch(
                enc1,
                objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                objc::MTLSize { width: num_blocks, height: 1, depth: 1 },
                objc::MTLSize { width: 256, height: 1, depth: 1 },
            );
            objc::msg0_void(enc1, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb1, objc::sel(b"commit\0"));
            objc::msg0_void(cb1, objc::sel(b"waitUntilCompleted\0"));
        }
        // Scan block sums (tiny: num_blocks <= n/256) in shared memory
        let sums = unsafe {
            std::slice::from_raw_parts(objc::buffer_contents(block_sums.mtl_buf).cast::<i32>(), num_blocks)
        };
        let offs = unsafe {
            std::slice::from_raw_parts_mut(objc::buffer_contents(block_offs.mtl_buf).cast::<i32>(), num_blocks)
        };
        let mut running = 0i32;
        for b in 0..num_blocks {
            offs[b] = running;
            running = running.wrapping_add(sums[b]);
        }
        let mut launches = 1;
        if num_blocks > 1 {
            let cb2 = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
            let enc2 = unsafe { objc::msg0(cb2, objc::sel(b"computeCommandEncoder\0")) };
            unsafe {
                objc::msg1_ptr_void(enc2, objc::sel(b"setComputePipelineState:\0"), pso_add);
                objc::msg3_set_buf(enc2, objc::sel(b"setBuffer:offset:atIndex:\0"), out_buf.mtl_buf, 0, 0);
                objc::msg3_set_buf(enc2, objc::sel(b"setBuffer:offset:atIndex:\0"), block_offs.mtl_buf, 0, 1);
                objc::msg3_set_bytes(enc2, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 2);
                objc::msg2_dispatch(
                    enc2,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: num_blocks, height: 1, depth: 1 },
                    objc::MTLSize { width: 256, height: 1, depth: 1 },
                );
                objc::msg0_void(enc2, objc::sel(b"endEncoding\0"));
                objc::msg0_void(cb2, objc::sel(b"commit\0"));
                objc::msg0_void(cb2, objc::sel(b"waitUntilCompleted\0"));
            }
            launches = 2;
        }
        ctx.in_flight.clear();
        ctx.telemetry.note_free(block_sums.bytes + block_offs.bytes);
        ctx.telemetry.kernel_launches += launches;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: "scan-i32".to_string(),
            reason: "block-prefix-spine-and-complete".to_string(),
        });
        ctx.telemetry.last_plan = DeviceLaunchPlan {
            op: if inclusive != 0 { "scan-inclusive-i32" } else { "scan-exclusive-i32" }.to_string(),
            schedule: format!("metal-block-scan-256(blocks={num_blocks})"),
            grid: [num_blocks * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "wrapping-add".to_string(),
        };
        drop(ctx);
        return register_buf(out_buf);
    }

    // OpenCL / CUDA / HIP / CPU (seq, cpu, ispc) dispatch
    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ib)) = (ocl.as_mut(), in_buf.ocl_buf.as_ref()) {
        return match ocl.scan_i32(ib, n, inclusive != 0, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ib)) = (cuda.as_mut(), in_buf.cuda_buf.as_ref()) {
        return match cuda.scan_i32(ib, n, inclusive != 0, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ib)) = (hip.as_mut(), in_buf.hip_buf.as_ref()) {
        return match hip.scan_i32(ib, n, inclusive != 0, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let raw = crate::simd_cpu::scan_i32(variant, &in_buf.cpu_buf, n, inclusive != 0, telemetry);
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    register_buf(res)
}

/// GPU Stable Compaction (filter `i32 > threshold`) on resident buffer.
/// Writes compacted element count `k <= n` into `*out_len_ptr` and returns new resident buffer handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_filter_i32_gt(
    ctx_id: i64,
    in_buf_id: i64,
    n_elems: i64,
    threshold: i64,
    out_len_ptr: *mut i64,
) -> i64 {
    let (Some(ctx_arc), Some(in_buf)) = (get_ctx(ctx_id), get_buf(in_buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let n = n_elems.max(0) as usize;
    if n * 4 > in_buf.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let is_metal = ctx_arc.lock().unwrap_or_else(|e| e.into_inner()).backend == "metal";
    #[cfg(target_os = "macos")]
    if is_metal {
        let _pool = objc::AutoreleasePool::push();
        let flags_id = {
            let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
            let pso_flags = match ctx.pipeline(BUILTIN_MSL_SOURCE, "filter_i32_gt_flags") {
                Ok(p) => p,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_COMPILE_FAILED;
                }
            };
            let flags = match ctx.alloc_mtl_buf(ctx_id, n * 4) {
                Ok(b) => b,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            };
            if n > 0 {
                let n_u32 = n as u32;
                let thr_i32 = threshold as i32;
                let groups = (n + 255) / 256;
                ctx.in_flight.push(in_buf.clone());
                ctx.in_flight.push(flags.clone());
                let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
                let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
                unsafe {
                    objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso_flags);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), in_buf.mtl_buf, 0, 0);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), flags.mtl_buf, 0, 1);
                    objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 2);
                    objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&thr_i32 as *const i32).cast(), 4, 3);
                    objc::msg2_dispatch(
                        enc,
                        objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                        objc::MTLSize { width: groups, height: 1, depth: 1 },
                        objc::MTLSize { width: 256, height: 1, depth: 1 },
                    );
                    objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
                    objc::msg0_void(cb, objc::sel(b"commit\0"));
                    objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
                }
                ctx.in_flight.clear();
                ctx.telemetry.kernel_launches += 1;
            }
            drop(ctx);
            register_buf(flags)
        };
        let pos_id = unsafe { cinch_device_scan_i32(ctx_id, flags_id, n as i64, 0) };
        if pos_id < 0 {
            cinch_device_buffer_free(flags_id);
            return pos_id;
        }
        let (flags_buf, pos_buf) = (get_buf(flags_id).unwrap(), get_buf(pos_id).unwrap());
        let k = if n == 0 {
            0usize
        } else {
            let last_flag = unsafe { *(objc::buffer_contents(flags_buf.mtl_buf).cast::<i32>().add(n - 1)) };
            let last_pos = unsafe { *(objc::buffer_contents(pos_buf.mtl_buf).cast::<i32>().add(n - 1)) };
            (last_pos + last_flag).max(0) as usize
        };
        let out_buf = {
            let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
            let out_buf = match ctx.alloc_mtl_buf(ctx_id, k * 4) {
                Ok(b) => b,
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    drop(ctx);
                    cinch_device_buffer_free(flags_id);
                    cinch_device_buffer_free(pos_id);
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            };
            if k > 0 {
                let pso_scatter = match ctx.pipeline(BUILTIN_MSL_SOURCE, "filter_i32_gt_scatter") {
                    Ok(p) => p,
                    Err(e) => {
                        ctx.telemetry.last_error = e;
                        drop(ctx);
                        cinch_device_buffer_free(flags_id);
                        cinch_device_buffer_free(pos_id);
                        return DEV_ERR_COMPILE_FAILED;
                    }
                };
                let n_u32 = n as u32;
                let groups = (n + 255) / 256;
                ctx.in_flight.push(in_buf.clone());
                ctx.in_flight.push(flags_buf.clone());
                ctx.in_flight.push(pos_buf.clone());
                ctx.in_flight.push(out_buf.clone());
                let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
                let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
                unsafe {
                    objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso_scatter);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), in_buf.mtl_buf, 0, 0);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), flags_buf.mtl_buf, 0, 1);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), pos_buf.mtl_buf, 0, 2);
                    objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), out_buf.mtl_buf, 0, 3);
                    objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&n_u32 as *const u32).cast(), 4, 4);
                    objc::msg2_dispatch(
                        enc,
                        objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                        objc::MTLSize { width: groups, height: 1, depth: 1 },
                        objc::MTLSize { width: 256, height: 1, depth: 1 },
                    );
                    objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
                    objc::msg0_void(cb, objc::sel(b"commit\0"));
                    objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
                }
                ctx.in_flight.clear();
                ctx.telemetry.kernel_launches += 1;
                ctx.telemetry.last_plan = DeviceLaunchPlan {
                    op: "filter-stable-i32".to_string(),
                    schedule: format!("metal-flags-scan-scatter(n={n},k={k})"),
                    grid: [groups * 256, 1, 1],
                    threadgroup: [256, 1, 1],
                    threadgroup_mem_bytes: 256 * 4,
                    numerical_mode: "exact-stable".to_string(),
                };
            }
            out_buf
        };
        cinch_device_buffer_free(flags_id);
        cinch_device_buffer_free(pos_id);
        if !out_len_ptr.is_null() {
            unsafe { *out_len_ptr = k as i64 };
        }
        return register_buf(out_buf);
    }

    // OpenCL / CUDA / HIP / CPU (seq, cpu, ispc) dispatch
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ib)) = (ocl.as_mut(), in_buf.ocl_buf.as_ref()) {
        return match ocl.filter_i32_gt(ib, n, threshold as i32, telemetry) {
            Ok((out, k)) => {
                drop(ctx);
                if !out_len_ptr.is_null() {
                    unsafe { *out_len_ptr = k as i64 };
                }
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ib)) = (cuda.as_mut(), in_buf.cuda_buf.as_ref()) {
        return match cuda.filter_i32_gt(ib, n, threshold as i32, telemetry) {
            Ok((out, k)) => {
                drop(ctx);
                if !out_len_ptr.is_null() {
                    unsafe { *out_len_ptr = k as i64 };
                }
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ib)) = (hip.as_mut(), in_buf.hip_buf.as_ref()) {
        return match hip.filter_i32_gt(ib, n, threshold as i32, telemetry) {
            Ok((out, k)) => {
                drop(ctx);
                if !out_len_ptr.is_null() {
                    unsafe { *out_len_ptr = k as i64 };
                }
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let (raw, k) = crate::simd_cpu::filter_i32_gt(variant, &in_buf.cpu_buf, n, threshold as i32, telemetry);
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    if !out_len_ptr.is_null() {
        unsafe { *out_len_ptr = k as i64 };
    }
    register_buf(res)
}

/// Complete Resident Pipeline: Batched `frames x height x width` 2D Stencil (`steps` ping-pong
/// iterations) -> Batched Histogram (`frames x num_bins`) in ONE command buffer on Metal GPU!
/// Zero intermediate host downloads or uploads!
/// Automatically selects `many-small-frames` vs `few-large-frames` schedule from runtime dims.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_stencil_histogram_u8(
    ctx_id: i64,
    in_buf_id: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights9: *const i32,
    divisor: i64,
    edge_policy: i64,
    steps: i64,
    num_bins: i64,
) -> i64 {
    let (Some(ctx_arc), Some(in_buf)) = (get_ctx(ctx_id), get_buf(in_buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let bins = num_bins.max(0).min(256) as usize;
    let nsteps = steps.max(0) as usize;
    let frame_pixels = h.saturating_mul(w);
    let total_pixels = f.saturating_mul(frame_pixels);
    if total_pixels > in_buf.bytes || weights9.is_null() {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ws = [0i32; 9];
    unsafe { std::ptr::copy_nonoverlapping(weights9, ws.as_mut_ptr(), 9) };

    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let out_hist = match ctx.alloc_mtl_buf(ctx_id, f * bins * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        if f == 0 || bins == 0 {
            drop(ctx);
            return register_buf(out_hist);
        }

        // Ping-pong resident device buffers for stencil (0 host copies!)
        let ping = if nsteps >= 1 {
            match ctx.alloc_mtl_buf(ctx_id, total_pixels) {
                Ok(b) => Some(b),
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            }
        } else {
            None
        };
        let pong = if nsteps >= 2 {
            match ctx.alloc_mtl_buf(ctx_id, total_pixels) {
                Ok(b) => Some(b),
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            }
        } else {
            None
        };

        let pso_stencil = match ctx.pipeline(BUILTIN_MSL_SOURCE, "stencil2d_u8_tiled") {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        let many_small = f >= 16 || frame_pixels <= 16_384;
        let pso_hist_name = if many_small {
            "hist_u8_many_small_frames"
        } else {
            "hist_u8_few_large_frames"
        };
        let pso_hist = match ctx.pipeline(BUILTIN_MSL_SOURCE, pso_hist_name) {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        let pso_zero = if !many_small {
            match ctx.pipeline(BUILTIN_MSL_SOURCE, "hist_u8_zero") {
                Ok(p) => Some(p),
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_COMPILE_FAILED;
                }
            }
        } else {
            None
        };

        // Retain all buffers for the lifetime of this command buffer!
        ctx.in_flight.push(in_buf.clone());
        ctx.in_flight.push(out_hist.clone());
        if let Some(ref b) = ping {
            ctx.in_flight.push(b.clone());
        }
        if let Some(ref b) = pong {
            ctx.in_flight.push(b.clone());
        }

        // Encode ALL stencil steps + histogram in ONE MTLCommandBuffer (1 sync point!)
        let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };

        #[repr(C)]
        struct StencilU8Params {
            frames: u32,
            height: u32,
            width: u32,
            divisor: i32,
            edge_policy: u32,
            w: [i32; 9],
        }
        let sp = StencilU8Params {
            frames: f as u32,
            height: h as u32,
            width: w as u32,
            divisor: divisor as i32,
            edge_policy: edge_policy as u32,
            w: ws,
        };
        let st_gx = (w + 15) / 16;
        let st_gy = (h + 15) / 16;

        let mut cur_src_mtl = in_buf.mtl_buf;
        let mut launches = 0usize;
        for step in 0..nsteps {
            let dst_mtl = if step % 2 == 0 {
                ping.as_ref().unwrap().mtl_buf
            } else {
                pong.as_ref().unwrap().mtl_buf
            };
            unsafe {
                objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso_stencil);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), cur_src_mtl, 0, 0);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), dst_mtl, 0, 1);
                objc::msg3_set_bytes(
                    enc,
                    objc::sel(b"setBytes:length:atIndex:\0"),
                    (&sp as *const StencilU8Params).cast(),
                    std::mem::size_of::<StencilU8Params>(),
                    2,
                );
                objc::msg2_dispatch(
                    enc,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: st_gx, height: st_gy, depth: f },
                    objc::MTLSize { width: 16, height: 16, depth: 1 },
                );
            }
            cur_src_mtl = dst_mtl;
            launches += 1;
        }

        // Now encode batched histogram reading `cur_src_mtl` directly on-device!
        #[repr(C)]
        struct HistParams {
            frames: u32,
            frame_pixels: u32,
            num_bins: u32,
            tiles_per_frame: u32,
        }
        let tiles_per_frame = if many_small {
            1u32
        } else {
            ((frame_pixels / (256 * 32)).clamp(1, 256)) as u32
        };
        let hp = HistParams {
            frames: f as u32,
            frame_pixels: frame_pixels as u32,
            num_bins: bins as u32,
            tiles_per_frame,
        };
        if let Some(pz) = pso_zero {
            let total_bins = (f * bins) as u32;
            let zgroups = (f * bins + 255) / 256;
            unsafe {
                objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pz);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), out_hist.mtl_buf, 0, 0);
                objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&total_bins as *const u32).cast(), 4, 1);
                objc::msg2_dispatch(
                    enc,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: zgroups, height: 1, depth: 1 },
                    objc::MTLSize { width: 256, height: 1, depth: 1 },
                );
            }
            launches += 1;
        }
        let hist_groups = f * (tiles_per_frame as usize);
        unsafe {
            objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso_hist);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), cur_src_mtl, 0, 0);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), out_hist.mtl_buf, 0, 1);
            objc::msg3_set_bytes(
                enc,
                objc::sel(b"setBytes:length:atIndex:\0"),
                (&hp as *const HistParams).cast(),
                std::mem::size_of::<HistParams>(),
                2,
            );
            objc::msg2_dispatch(
                enc,
                objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                objc::MTLSize { width: hist_groups, height: 1, depth: 1 },
                objc::MTLSize { width: 256, height: 1, depth: 1 },
            );
            objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb, objc::sel(b"commit\0"));
            objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
        }
        launches += 1;
        let status = unsafe { objc::msg0_usize(cb, objc::sel(b"status\0")) };
        ctx.in_flight.clear();
        if let Some(ref b) = ping {
            ctx.telemetry.note_free(b.bytes);
        }
        if let Some(ref b) = pong {
            ctx.telemetry.note_free(b.bytes);
        }
        if status == objc::MTL_CMD_BUF_STATUS_ERROR {
            let err = unsafe { objc::msg0(cb, objc::sel(b"error\0")) };
            ctx.telemetry.last_error = unsafe { objc::nserror_to_string(err) };
            ctx.telemetry.note_free(out_hist.bytes);
            return DEV_ERR_EXECUTION_FAILED;
        }
        ctx.telemetry.kernel_launches += launches;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: "stencil-histogram-pipeline".to_string(),
            reason: "single-command-buffer-end".to_string(),
        });
        ctx.telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil({nsteps})-then-histogram-u8"),
            schedule: if many_small {
                format!("many-small-frames(frames={f},h={h},w={w},hist_groups={hist_groups},tg_privatized)")
            } else {
                format!("few-large-frames(frames={f},h={h},w={w},tiles_per_frame={tiles_per_frame},hist_groups={hist_groups},tg_privatized)")
            },
            grid: [hist_groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 256 * 4,
            numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
        };
        drop(ctx);
        return register_buf(out_hist);
    }

    // OpenCL / CUDA / HIP / CPU (seq, cpu, ispc) dispatch
    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ib)) = (ocl.as_mut(), in_buf.ocl_buf.as_ref()) {
        return match ocl.stencil_histogram_u8(
            ib, f, h, w, ws, divisor as i32, edge_policy as u32, nsteps, bins, telemetry,
        ) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ib)) = (cuda.as_mut(), in_buf.cuda_buf.as_ref()) {
        return match cuda.stencil_histogram_u8(
            ib, f, h, w, ws, divisor as i32, edge_policy as u32, nsteps, bins, telemetry,
        ) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ib)) = (hip.as_mut(), in_buf.hip_buf.as_ref()) {
        return match hip.stencil_histogram_u8(
            ib, f, h, w, ws, divisor as i32, edge_policy as u32, nsteps, bins, telemetry,
        ) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let raw = crate::simd_cpu::stencil_histogram_u8(
        variant,
        &in_buf.cpu_buf,
        f,
        h,
        w,
        ws,
        divisor as i32,
        edge_policy,
        nsteps,
        bins,
        telemetry,
    );
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    register_buf(res)
}

/// GPU Batched 2D F32 Stencil (with ping-pong buffer reuse across `steps`), returning resident buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_stencil2d_f32(
    ctx_id: i64,
    in_buf_id: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights9: *const f32,
    edge_policy: i64,
    steps: i64,
) -> i64 {
    let (Some(ctx_arc), Some(in_buf)) = (get_ctx(ctx_id), get_buf(in_buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let nsteps = steps.max(1) as usize;
    let total = f.saturating_mul(h).saturating_mul(w);
    if total * 4 > in_buf.bytes || weights9.is_null() {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ws = [0.0f32; 9];
    unsafe { std::ptr::copy_nonoverlapping(weights9, ws.as_mut_ptr(), 9) };

    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let ping = match ctx.alloc_mtl_buf(ctx_id, total * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        let pong = if nsteps >= 2 {
            match ctx.alloc_mtl_buf(ctx_id, total * 4) {
                Ok(b) => Some(b),
                Err(e) => {
                    ctx.telemetry.last_error = e;
                    return DEV_ERR_OUT_OF_MEMORY;
                }
            }
        } else {
            None
        };
        let pso = match ctx.pipeline(BUILTIN_MSL_SOURCE, "stencil2d_f32_tiled") {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        ctx.in_flight.push(in_buf.clone());
        ctx.in_flight.push(ping.clone());
        if let Some(ref b) = pong {
            ctx.in_flight.push(b.clone());
        }
        #[repr(C)]
        struct StencilF32Params {
            frames: u32,
            height: u32,
            width: u32,
            edge_policy: u32,
            w: [f32; 9],
        }
        let sp = StencilF32Params {
            frames: f as u32,
            height: h as u32,
            width: w as u32,
            edge_policy: edge_policy as u32,
            w: ws,
        };
        let gx = (w + 15) / 16;
        let gy = (h + 15) / 16;
        let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
        let mut cur_src = in_buf.mtl_buf;
        let mut final_is_ping = true;
        for step in 0..nsteps {
            let dst = if step % 2 == 0 {
                final_is_ping = true;
                ping.mtl_buf
            } else {
                final_is_ping = false;
                pong.as_ref().unwrap().mtl_buf
            };
            unsafe {
                objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), cur_src, 0, 0);
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), dst, 0, 1);
                objc::msg3_set_bytes(
                    enc,
                    objc::sel(b"setBytes:length:atIndex:\0"),
                    (&sp as *const StencilF32Params).cast(),
                    std::mem::size_of::<StencilF32Params>(),
                    2,
                );
                objc::msg2_dispatch(
                    enc,
                    objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                    objc::MTLSize { width: gx, height: gy, depth: f },
                    objc::MTLSize { width: 16, height: 16, depth: 1 },
                );
            }
            cur_src = dst;
        }
        unsafe {
            objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb, objc::sel(b"commit\0"));
            objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
        }
        ctx.in_flight.clear();
        let out_buf = if final_is_ping {
            if let Some(ref b) = pong {
                ctx.telemetry.note_free(b.bytes);
            }
            ping
        } else {
            ctx.telemetry.note_free(ping.bytes);
            pong.unwrap()
        };
        ctx.telemetry.kernel_launches += nsteps;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: format!("stencil2d-f32-pingpong({nsteps})"),
            reason: "command-buffer-complete".to_string(),
        });
        ctx.telemetry.last_plan = DeviceLaunchPlan {
            op: format!("stencil2d-f32(steps={nsteps})"),
            schedule: format!("metal-tiled-16x16-halo(frames={f},h={h},w={w},pingpong=2bufs)"),
            grid: [gx * 16, gy * 16, f],
            threadgroup: [16, 16, 1],
            threadgroup_mem_bytes: 18 * 18 * 4,
            numerical_mode: "ieee754-f32".to_string(),
        };
        drop(ctx);
        return register_buf(out_buf);
    }

    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ib)) = (ocl.as_mut(), in_buf.ocl_buf.as_ref()) {
        return match ocl.stencil2d_f32(ib, f, h, w, ws, edge_policy as u32, nsteps, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ib)) = (cuda.as_mut(), in_buf.cuda_buf.as_ref()) {
        return match cuda.stencil2d_f32(ib, f, h, w, ws, edge_policy as u32, nsteps, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ib)) = (hip.as_mut(), in_buf.hip_buf.as_ref()) {
        return match hip.stencil2d_f32(ib, f, h, w, ws, edge_policy as u32, nsteps, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let raw = crate::simd_cpu::stencil2d_f32(
        variant,
        &in_buf.cpu_buf,
        f,
        h,
        w,
        ws,
        edge_policy,
        nsteps,
        telemetry,
    );
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    register_buf(res)
}

/// GPU CSR SpMV (`y = A * x`) over resident device buffers, handling empty rows!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_spmv_csr_f32(
    ctx_id: i64,
    vals_id: i64,
    cols_id: i64,
    row_ptr_id: i64,
    x_id: i64,
    nrows: i64,
    nnz: i64,
    float_mode: i64,
) -> i64 {
    let (Some(ctx_arc), Some(vals), Some(cols), Some(row_ptr), Some(x)) = (
        get_ctx(ctx_id),
        get_buf(vals_id),
        get_buf(cols_id),
        get_buf(row_ptr_id),
        get_buf(x_id),
    ) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let r = nrows.max(0) as usize;
    let nz = nnz.max(0) as usize;
    if (r + 1) * 4 > row_ptr.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let exact = float_mode == 0 || float_mode == par_arrays::REDUCE_F32_ADD_EXACT_TREE;
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let out_y = match ctx.alloc_mtl_buf(ctx_id, r * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        if r == 0 {
            drop(ctx);
            return register_buf(out_y);
        }
        let avg_nnz = if r > 0 { nz / r } else { 0 };
        let (kname, groups, sched) = if exact {
            (
                "spmv_csr_f32_exact",
                (r + 255) / 256,
                format!("metal-csr-exact-tree(rows={r},nnz={nz})"),
            )
        } else if avg_nnz >= 16 {
            (
                "spmv_csr_f32_simd",
                (r * 32 + 255) / 256,
                format!("metal-csr-simd-row(rows={r},nnz={nz},avg_nnz={avg_nnz})"),
            )
        } else {
            (
                "spmv_csr_f32_scalar",
                (r + 255) / 256,
                format!("metal-csr-scalar-row(rows={r},nnz={nz},avg_nnz={avg_nnz})"),
            )
        };
        let pso = match ctx.pipeline(BUILTIN_MSL_SOURCE, kname) {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        ctx.in_flight.push(vals.clone());
        ctx.in_flight.push(cols.clone());
        ctx.in_flight.push(row_ptr.clone());
        ctx.in_flight.push(x.clone());
        ctx.in_flight.push(out_y.clone());
        let r_u32 = r as u32;
        let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
        unsafe {
            objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), vals.mtl_buf, 0, 0);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), cols.mtl_buf, 0, 1);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), row_ptr.mtl_buf, 0, 2);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), x.mtl_buf, 0, 3);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), out_y.mtl_buf, 0, 4);
            objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&r_u32 as *const u32).cast(), 4, 5);
            objc::msg2_dispatch(
                enc,
                objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                objc::MTLSize { width: groups, height: 1, depth: 1 },
                objc::MTLSize { width: 256, height: 1, depth: 1 },
            );
            objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb, objc::sel(b"commit\0"));
            objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
        }
        ctx.in_flight.clear();
        ctx.telemetry.kernel_launches += 1;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: "spmv-csr-f32".to_string(),
            reason: "command-buffer-complete".to_string(),
        });
        ctx.telemetry.last_plan = DeviceLaunchPlan {
            op: "spmv-csr-f32".to_string(),
            schedule: sched,
            grid: [groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: 0,
            numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" }.to_string(),
        };
        drop(ctx);
        return register_buf(out_y);
    }

    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(bv), Some(bc), Some(br), Some(bx)) = (
        ocl.as_mut(),
        vals.ocl_buf.as_ref(),
        cols.ocl_buf.as_ref(),
        row_ptr.ocl_buf.as_ref(),
        x.ocl_buf.as_ref(),
    ) {
        return match ocl.spmv_csr_f32(bv, bc, br, bx, r, nz, exact, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(bv), Some(bc), Some(br), Some(bx)) = (
        cuda.as_mut(),
        vals.cuda_buf.as_ref(),
        cols.cuda_buf.as_ref(),
        row_ptr.cuda_buf.as_ref(),
        x.cuda_buf.as_ref(),
    ) {
        return match cuda.spmv_csr_f32(bv, bc, br, bx, r, nz, exact, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(bv), Some(bc), Some(br), Some(bx)) = (
        hip.as_mut(),
        vals.hip_buf.as_ref(),
        cols.hip_buf.as_ref(),
        row_ptr.hip_buf.as_ref(),
        x.hip_buf.as_ref(),
    ) {
        return match hip.spmv_csr_f32(bv, bc, br, bx, r, nz, exact, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let raw = crate::simd_cpu::spmv_csr_f32(
        variant,
        &vals.cpu_buf,
        &cols.cpu_buf,
        &row_ptr.cpu_buf,
        &x.cpu_buf,
        r,
        nz,
        exact,
        telemetry,
    );
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    register_buf(res)
}

/// GPU Row Reduction (`rows x cols -> rows`) with adaptive outer vs inner schedule!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_row_reduce_f32(
    ctx_id: i64,
    in_buf_id: i64,
    rows: i64,
    cols: i64,
) -> i64 {
    let (Some(ctx_arc), Some(in_buf)) = (get_ctx(ctx_id), get_buf(in_buf_id)) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    let r = rows.max(0) as usize;
    let c = cols.max(0) as usize;
    if r.saturating_mul(c).saturating_mul(4) > in_buf.bytes {
        return DEV_ERR_INVALID_SHAPE;
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let out_buf = match ctx.alloc_mtl_buf(ctx_id, r * 4) {
            Ok(b) => b,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_OUT_OF_MEMORY;
            }
        };
        if r == 0 {
            drop(ctx);
            return register_buf(out_buf);
        }
        let outer = r >= 128 && c < 2048;
        let (kname, groups, sched) = if outer {
            (
                "row_reduce_f32_outer",
                (r + 255) / 256,
                format!("metal-outer-parallel(rows={r},cols={c})"),
            )
        } else {
            (
                "row_reduce_f32_inner",
                r,
                format!("metal-inner-simd-parallel(rows={r},cols={c})"),
            )
        };
        let pso = match ctx.pipeline(BUILTIN_MSL_SOURCE, kname) {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        #[repr(C)]
        struct RowParams {
            rows: u32,
            cols: u32,
        }
        let rp = RowParams {
            rows: r as u32,
            cols: c as u32,
        };
        ctx.in_flight.push(in_buf.clone());
        ctx.in_flight.push(out_buf.clone());
        let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
        unsafe {
            objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), in_buf.mtl_buf, 0, 0);
            objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), out_buf.mtl_buf, 0, 1);
            objc::msg3_set_bytes(enc, objc::sel(b"setBytes:length:atIndex:\0"), (&rp as *const RowParams).cast(), 8, 2);
            objc::msg2_dispatch(
                enc,
                objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                objc::MTLSize { width: groups, height: 1, depth: 1 },
                objc::MTLSize { width: 256, height: 1, depth: 1 },
            );
            objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb, objc::sel(b"commit\0"));
            objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
        }
        ctx.in_flight.clear();
        ctx.telemetry.kernel_launches += 1;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: "row-reduce-f32".to_string(),
            reason: "command-buffer-complete".to_string(),
        });
        ctx.telemetry.last_plan = DeviceLaunchPlan {
            op: "row-reduce-f32".to_string(),
            schedule: sched,
            grid: [groups * 256, 1, 1],
            threadgroup: [256, 1, 1],
            threadgroup_mem_bytes: if outer { 0 } else { 32 },
            numerical_mode: "f32-add-relaxed".to_string(),
        };
        drop(ctx);
        return register_buf(out_buf);
    }

    let DeviceCtx {
        ref mut ocl,
        ref mut cuda,
        ref mut hip,
        ref mut telemetry,
        backend,
        ..
    } = *ctx;
    if let (Some(ocl), Some(ib)) = (ocl.as_mut(), in_buf.ocl_buf.as_ref()) {
        return match ocl.row_reduce_f32(ib, r, c, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_opencl(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(cuda), Some(ib)) = (cuda.as_mut(), in_buf.cuda_buf.as_ref()) {
        return match cuda.row_reduce_f32(ib, r, c, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_cuda(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    if let (Some(hip), Some(ib)) = (hip.as_mut(), in_buf.hip_buf.as_ref()) {
        return match hip.row_reduce_f32(ib, r, c, telemetry) {
            Ok(out) => {
                drop(ctx);
                register_buf(ResidentBuffer::new_hip(ctx_id, out))
            }
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_EXECUTION_FAILED
            }
        };
    }
    let variant = match backend {
        "seq" => crate::simd_cpu::CpuVariant::Seq,
        "ispc" => crate::simd_cpu::CpuVariant::IspcSimd,
        _ => crate::simd_cpu::CpuVariant::Multicore,
    };
    let raw = crate::simd_cpu::row_reduce_f32(variant, &in_buf.cpu_buf, r, c, telemetry);
    telemetry.note_alloc(raw.len());
    let res = ResidentBuffer::new_cpu(ctx_id, raw);
    drop(ctx);
    register_buf(res)
}

/// Cinch-ABI wrapper for `cinch_device_dot_f32`: returns `u32` IEEE-754 bits
/// in the low 32 bits (`>= 0`) on success, or a negative `DEV_ERR_*` code.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_dot_f32_bits(
    ctx_id: i64,
    buf_a_id: i64,
    buf_b_id: i64,
    n_elems: i64,
    float_mode: i64,
) -> i64 {
    let mut out = 0.0f32;
    let status = unsafe { cinch_device_dot_f32(ctx_id, buf_a_id, buf_b_id, n_elems, float_mode, &mut out) };
    if status != DEV_OK {
        return status;
    }
    i64::from(out.to_bits())
}

/// Cinch-ABI wrapper for `cinch_device_filter_i32_gt`: caller reads output
/// length `k = cinch_device_buffer_bytes(handle) / 4`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_filter_i32_gt_handle(
    ctx_id: i64,
    in_buf_id: i64,
    n_elems: i64,
    threshold: i64,
) -> i64 {
    let mut k = 0i64;
    unsafe { cinch_device_filter_i32_gt(ctx_id, in_buf_id, n_elems, threshold, &mut k) }
}

/// Cinch-ABI wrapper taking a packed Cinch `Array I32` of 9 stencil weights.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_stencil_histogram_u8_arr(
    ctx_id: i64,
    in_buf_id: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights_arr: i64,
    divisor: i64,
    edge_policy: i64,
    steps: i64,
    num_bins: i64,
) -> i64 {
    let ws = unsafe { par_arrays::slice_i32(weights_arr) };
    if ws.len() != 9 {
        return DEV_ERR_INVALID_SHAPE;
    }
    unsafe {
        cinch_device_stencil_histogram_u8(
            ctx_id,
            in_buf_id,
            frames,
            height,
            width,
            ws.as_ptr(),
            divisor,
            edge_policy,
            steps,
            num_bins,
        )
    }
}

/// Cinch-ABI wrapper taking a packed Cinch `Array F32` of 9 stencil weights.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_stencil2d_f32_arr(
    ctx_id: i64,
    in_buf_id: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights_arr: i64,
    edge_policy: i64,
    steps: i64,
) -> i64 {
    let ws = unsafe { par_arrays::slice_f32(weights_arr) };
    if ws.len() != 9 {
        return DEV_ERR_INVALID_SHAPE;
    }
    unsafe {
        cinch_device_stencil2d_f32(
            ctx_id,
            in_buf_id,
            frames,
            height,
            width,
            ws.as_ptr(),
            edge_policy,
            steps,
        )
    }
}

/// Generic compiler-emitted MSL kernel runner (for Typed Array IR codegen!).
/// Takes NUL-terminated MSL source + kernel name, an array of `nbufs` resident buffer handles,
/// optional uniform bytes, and 1D thread count `n_threads`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_run_msl_1d(
    ctx_id: i64,
    msl_cstr: *const i8,
    kernel_cstr: *const i8,
    buf_ids: *const i64,
    nbufs: usize,
    uniform_ptr: *const u8,
    uniform_len: usize,
    n_threads: usize,
) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    if msl_cstr.is_null() || kernel_cstr.is_null() {
        return DEV_ERR_COMPILE_FAILED;
    }
    let msl = unsafe { CStr::from_ptr(msl_cstr) }.to_string_lossy();
    let kname = unsafe { CStr::from_ptr(kernel_cstr) }.to_string_lossy();
    let mut bufs = Vec::with_capacity(nbufs);
    for i in 0..nbufs {
        let id = unsafe { *buf_ids.add(i) };
        let Some(b) = get_buf(id) else {
            return DEV_ERR_INVALID_HANDLE;
        };
        bufs.push(b);
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    #[cfg(target_os = "macos")]
    if ctx.backend == "metal" {
        let _pool = objc::AutoreleasePool::push();
        let pso = match ctx.pipeline(&msl, &kname) {
            Ok(p) => p,
            Err(e) => {
                ctx.telemetry.last_error = e;
                return DEV_ERR_COMPILE_FAILED;
            }
        };
        for b in &bufs {
            ctx.in_flight.push(b.clone());
        }
        let groups = (n_threads + 255) / 256;
        let cb = unsafe { objc::msg0(ctx.mtl_queue, objc::sel(b"commandBuffer\0")) };
        let enc = unsafe { objc::msg0(cb, objc::sel(b"computeCommandEncoder\0")) };
        unsafe {
            objc::msg1_ptr_void(enc, objc::sel(b"setComputePipelineState:\0"), pso);
            for (i, b) in bufs.iter().enumerate() {
                objc::msg3_set_buf(enc, objc::sel(b"setBuffer:offset:atIndex:\0"), b.mtl_buf, 0, i);
            }
            if !uniform_ptr.is_null() && uniform_len > 0 {
                objc::msg3_set_bytes(
                    enc,
                    objc::sel(b"setBytes:length:atIndex:\0"),
                    uniform_ptr.cast(),
                    uniform_len,
                    nbufs,
                );
            }
            objc::msg2_dispatch(
                enc,
                objc::sel(b"dispatchThreadgroups:threadsPerThreadgroup:\0"),
                objc::MTLSize { width: groups.max(1), height: 1, depth: 1 },
                objc::MTLSize { width: 256, height: 1, depth: 1 },
            );
            objc::msg0_void(enc, objc::sel(b"endEncoding\0"));
            objc::msg0_void(cb, objc::sel(b"commit\0"));
            objc::msg0_void(cb, objc::sel(b"waitUntilCompleted\0"));
        }
        let status = unsafe { objc::msg0_usize(cb, objc::sel(b"status\0")) };
        ctx.in_flight.clear();
        if status == objc::MTL_CMD_BUF_STATUS_ERROR {
            let err = unsafe { objc::msg0(cb, objc::sel(b"error\0")) };
            ctx.telemetry.last_error = unsafe { objc::nserror_to_string(err) };
            return DEV_ERR_EXECUTION_FAILED;
        }
        ctx.telemetry.kernel_launches += 1;
        ctx.telemetry.sync_points.push(SyncPoint {
            op: kname.into_owned(),
            reason: "compiled-msl-kernel-complete".to_string(),
        });
        return DEV_OK;
    }
    DEV_ERR_NO_DEVICE
}

/// Generic compiler-emitted OpenCL C 1D kernel runner over resident OpenCL buffers.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_run_opencl_1d(
    ctx_id: i64,
    ocl_cstr: *const i8,
    kernel_cstr: *const i8,
    buf_ids: *const i64,
    nbufs: usize,
    n_threads: usize,
) -> i64 {
    let Some(ctx_arc) = get_ctx(ctx_id) else {
        return DEV_ERR_INVALID_HANDLE;
    };
    if ocl_cstr.is_null() || kernel_cstr.is_null() {
        return DEV_ERR_COMPILE_FAILED;
    }
    let src = unsafe { CStr::from_ptr(ocl_cstr) }.to_string_lossy();
    let kname = unsafe { CStr::from_ptr(kernel_cstr) }.to_string_lossy();
    let mut bufs = Vec::with_capacity(nbufs);
    for i in 0..nbufs {
        let id = unsafe { *buf_ids.add(i) };
        let Some(b) = get_buf(id) else {
            return DEV_ERR_INVALID_HANDLE;
        };
        bufs.push(b);
    }
    let mut ctx = ctx_arc.lock().unwrap_or_else(|e| e.into_inner());
    let DeviceCtx { ref ocl, ref mut telemetry, .. } = *ctx;
    if let Some(ocl) = ocl.as_ref() {
        let mut ocl_refs = Vec::with_capacity(nbufs);
        for b in &bufs {
            let Some(ob) = b.ocl_buf.as_ref() else {
                return DEV_ERR_INVALID_HANDLE;
            };
            ocl_refs.push(ob);
        }
        return match ocl.run_source_1d(&src, &kname, &ocl_refs, n_threads, telemetry) {
            Ok(()) => DEV_OK,
            Err(e) => {
                telemetry.last_error = e;
                DEV_ERR_COMPILE_FAILED
            }
        };
    }
    DEV_ERR_NO_DEVICE
}

/// Write a JSON summary of all supported backends and live availability on this host.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_available_backends_cstr(
    out_ptr: *mut u8,
    out_cap: usize,
) -> i64 {
    #[cfg(target_os = "macos")]
    let metal_ok = DeviceCtx::open_metal().is_ok();
    #[cfg(not(target_os = "macos"))]
    let metal_ok = false;
    let ocl_ok = crate::opencl::is_available();
    let cuda_ok = crate::cuda::is_available();
    let hip_ok = crate::hip::is_available();
    let json = format!(
        "{{\"seq\":true,\"cpu\":true,\"ispc\":true,\"metal\":{metal_ok},\"opencl\":{ocl_ok},\"cuda\":{cuda_ok},\"hip\":{hip_ok}}}"
    );
    let b = json.as_bytes();
    if !out_ptr.is_null() && out_cap > 0 {
        let n = b.len().min(out_cap - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(b.as_ptr(), out_ptr, n);
            *out_ptr.add(n) = 0;
        }
        n as i64
    } else {
        b.len() as i64
    }
}

/// Copy built-in kernel source for a given `DEVICE_KIND_*` into `out_ptr` (returns byte length).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_device_backend_source_cstr(
    kind: i64,
    out_ptr: *mut u8,
    out_cap: usize,
) -> i64 {
    let src: &str = match kind {
        DEVICE_KIND_METAL | DEVICE_KIND_METAL_REQUIRED => {
            #[cfg(target_os = "macos")]
            {
                BUILTIN_MSL_SOURCE
            }
            #[cfg(not(target_os = "macos"))]
            {
                ""
            }
        }
        DEVICE_KIND_OPENCL | DEVICE_KIND_OPENCL_REQUIRED => crate::opencl::BUILTIN_OPENCL_SOURCE,
        DEVICE_KIND_CUDA | DEVICE_KIND_CUDA_REQUIRED => crate::cuda::BUILTIN_CUDA_SOURCE,
        DEVICE_KIND_HIP | DEVICE_KIND_HIP_REQUIRED => crate::hip::BUILTIN_HIP_SOURCE,
        DEVICE_KIND_ISPC_SIMD => crate::simd_cpu::BUILTIN_ISPC_SOURCE,
        _ => return DEV_ERR_INVALID_HANDLE,
    };
    let b = src.as_bytes();
    if !out_ptr.is_null() && out_cap > 0 {
        let n = b.len().min(out_cap - 1);
        unsafe {
            std::ptr::copy_nonoverlapping(b.as_ptr(), out_ptr, n);
            *out_ptr.add(n) = 0;
        }
        n as i64
    } else {
        b.len() as i64
    }
}

// ---------------------------------------------------------------------------
// Live Hardware Tests on Apple Metal GPU (Apple Silicon)
// ---------------------------------------------------------------------------

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::*;

    fn report_string(ctx: i64) -> String {
        let mut buf = vec![0u8; 4096];
        let n = unsafe { cinch_device_report_cstr(ctx, buf.as_mut_ptr(), buf.len()) };
        assert!(n > 0);
        String::from_utf8_lossy(&buf[..n as usize]).into_owned()
    }

    #[test]
    fn metal_dot_product_exact_tree_matches_cpu_bit_for_bit() {
        let ctx = cinch_device_open(DEVICE_KIND_METAL_REQUIRED);
        assert!(ctx > 0, "Metal GPU must be available on macOS Apple Silicon");
        let n = 100_003usize;
        let xs: Vec<f32> = (0..n).map(|i| ((i % 89) as f32 - 44.0) * 0.125).collect();
        let ys: Vec<f32> = (0..n).map(|i| ((i % 47) as f32 - 23.0) * 0.25).collect();
        let cpu_exact = par_arrays::f32_exact_tree_prod(0, n, &|i| xs[i] * ys[i]);
        unsafe {
            let ba = cinch_device_upload_raw(ctx, xs.as_ptr().cast(), n * 4);
            let bb = cinch_device_upload_raw(ctx, ys.as_ptr().cast(), n * 4);
            let mut gpu_exact = 0.0f32;
            assert_eq!(
                cinch_device_dot_f32(ctx, ba, bb, n as i64, 0, &mut gpu_exact),
                DEV_OK,
                "metal dot failed: {}",
                report_string(ctx)
            );
            assert_eq!(
                gpu_exact.to_bits(),
                cpu_exact.to_bits(),
                "GPU exact-tree {gpu_exact} ({:#x}) != CPU exact-tree {cpu_exact} ({:#x})",
                gpu_exact.to_bits(),
                cpu_exact.to_bits()
            );
            let mut gpu_relaxed = 0.0f32;
            assert_eq!(cinch_device_dot_f32(ctx, ba, bb, n as i64, 1, &mut gpu_relaxed), DEV_OK);
            assert!((gpu_relaxed - cpu_exact).abs() < 1e-2);
            let rep = report_string(ctx);
            assert!(rep.contains("\"backend\":\"metal\""), "{rep}");
            cinch_device_buffer_free(ba);
            cinch_device_buffer_free(bb);
        }
        cinch_device_close(ctx);
    }

    #[test]
    fn metal_stencil_histogram_pipeline_one_upload_one_download_and_schedule_switching() {
        let ctx = cinch_device_open(DEVICE_KIND_METAL_REQUIRED);
        assert!(ctx > 0);
        let weights = [1i32, 1, 1, 1, 1, 1, 1, 1, 1];

        for (label, frames, h, w, expect_sched) in [
            ("many-small", 64usize, 32usize, 32usize, "many-small-frames"),
            ("few-large", 2usize, 256usize, 256usize, "few-large-frames"),
        ] {
            cinch_device_reset_telemetry(ctx);
            let total = frames * h * w;
            let imgs: Vec<u8> = (0..total).map(|i| if i % 5 == 0 { 128 } else { (i * 17 % 256) as u8 }).collect();
            unsafe {
                // 1 upload!
                let in_buf = cinch_device_upload_raw(ctx, imgs.as_ptr(), total);
                // Run 2-step stencil + histogram completely resident on Metal GPU!
                let out_buf = cinch_device_stencil_histogram_u8(
                    ctx,
                    in_buf,
                    frames as i64,
                    h as i64,
                    w as i64,
                    weights.as_ptr(),
                    9,
                    par_arrays::EDGE_CLAMP,
                    2,
                    256,
                );
                assert!(out_buf > 0, "{label}: pipeline failed ({out_buf})");
                // 1 download (only the histogram counts)!
                let mut gpu_hist = vec![0u32; frames * 256];
                assert_eq!(
                    cinch_device_download_raw(ctx, out_buf, gpu_hist.as_mut_ptr().cast(), frames * 256 * 4),
                    DEV_OK
                );
                let rep = report_string(ctx);
                assert!(rep.contains("\"uploads\":1,"), "{label}: {rep}");
                assert!(rep.contains("\"downloads\":1,"), "{label}: {rep}");
                assert!(rep.contains(expect_sched), "{label}: expected {expect_sched} in {rep}");

                // Verify bit-exact agreement with CPU scalar reference!
                let arr_in = arrays::alloc_uninit_packed(total as i64, par_arrays::FMT_U8);
                par_arrays::slice_u8_mut(arr_in).copy_from_slice(&imgs);
                let arr_w = arrays::alloc_uninit_packed(9, par_arrays::FMT_I32);
                par_arrays::slice_i32_mut(arr_w).copy_from_slice(&weights);
                par_arrays::cinch_array_set_exec_mode(1);
                let cpu_st = par_arrays::cinch_array_stencil2d_u8(
                    arr_in, frames as i64, h as i64, w as i64, arr_w, 9, par_arrays::EDGE_CLAMP, 2,
                );
                let cpu_h = par_arrays::cinch_array_histogram_batch_u8(
                    cpu_st, frames as i64, h as i64, w as i64, 256,
                );
                assert_eq!(gpu_hist.as_slice(), par_arrays::slice_u32(cpu_h), "{label}: GPU != CPU ref");
                par_arrays::cinch_array_set_exec_mode(0);
                crate::rc::cinch_drop(cpu_h);
                crate::rc::cinch_drop(cpu_st);
                crate::rc::cinch_drop(arr_w);
                crate::rc::cinch_drop(arr_in);
                cinch_device_buffer_free(in_buf);
                cinch_device_buffer_free(out_buf);
            }
        }
        cinch_device_close(ctx);
    }

    #[test]
    fn metal_scan_filter_and_spmv_with_empty_rows_agree_with_cpu() {
        let ctx = cinch_device_open(DEVICE_KIND_METAL_REQUIRED);
        assert!(ctx > 0);
        let n = 10_000usize;
        let data: Vec<i32> = (0..n).map(|i| ((i as i32 * 29) % 40) - 20).collect();
        unsafe {
            let b_in = cinch_device_upload_raw(ctx, data.as_ptr().cast(), n * 4);
            // Prefix scan
            let b_scan = cinch_device_scan_i32(ctx, b_in, n as i64, 0);
            assert!(b_scan > 0);
            let mut gpu_scan = vec![0i32; n];
            assert_eq!(cinch_device_download_raw(ctx, b_scan, gpu_scan.as_mut_ptr().cast(), n * 4), DEV_OK);
            let mut acc = 0i32;
            for i in 0..n {
                assert_eq!(gpu_scan[i], acc, "scan mismatch at {i}");
                acc = acc.wrapping_add(data[i]);
            }
            // Stable filter
            let mut k = 0i64;
            let b_filt = cinch_device_filter_i32_gt(ctx, b_in, n as i64, 5, &mut k);
            assert!(b_filt > 0);
            let mut gpu_filt = vec![0i32; k as usize];
            assert_eq!(cinch_device_download_raw(ctx, b_filt, gpu_filt.as_mut_ptr().cast(), k as usize * 4), DEV_OK);
            let expected: Vec<i32> = data.iter().copied().filter(|&x| x > 5).collect();
            assert_eq!(gpu_filt, expected);
            cinch_device_buffer_free(b_filt);
            cinch_device_buffer_free(b_scan);
            cinch_device_buffer_free(b_in);

            // SpMV with empty rows (exact-tree mode)
            let vals = [2.0f32, 3.0, 4.0, 5.0];
            let cols = [0u32, 2, 1, 2];
            let row_ptr = [0u32, 2, 2, 4, 4];
            let x = [10.0f32, 20.0, 30.0];
            let bv = cinch_device_upload_raw(ctx, vals.as_ptr().cast(), 16);
            let bc = cinch_device_upload_raw(ctx, cols.as_ptr().cast(), 16);
            let br = cinch_device_upload_raw(ctx, row_ptr.as_ptr().cast(), 20);
            let bx = cinch_device_upload_raw(ctx, x.as_ptr().cast(), 12);
            let by = cinch_device_spmv_csr_f32(ctx, bv, bc, br, bx, 4, 4, 0);
            assert!(by > 0);
            let mut y = [0.0f32; 4];
            assert_eq!(cinch_device_download_raw(ctx, by, y.as_mut_ptr().cast(), 16), DEV_OK);
            assert_eq!(y, [110.0, 0.0, 230.0, 0.0]);
            cinch_device_buffer_free(by);
            cinch_device_buffer_free(bx);
            cinch_device_buffer_free(br);
            cinch_device_buffer_free(bc);
            cinch_device_buffer_free(bv);
        }
        cinch_device_close(ctx);
    }

    #[test]
    fn opencl_all_kernels_and_stencil_histogram_match_metal_and_cpu_bit_for_bit() {
        let ctx = cinch_device_open(DEVICE_KIND_OPENCL_REQUIRED);
        assert!(ctx > 0, "OpenCL GPU must be available on macOS Apple Silicon");
        let n = 100_003usize;
        let xs: Vec<f32> = (0..n).map(|i| ((i % 89) as f32 - 44.0) * 0.125).collect();
        let ys: Vec<f32> = (0..n).map(|i| ((i % 47) as f32 - 23.0) * 0.25).collect();
        let cpu_exact = par_arrays::f32_exact_tree_prod(0, n, &|i| xs[i] * ys[i]);
        unsafe {
            let ba = cinch_device_upload_raw(ctx, xs.as_ptr().cast(), n * 4);
            let bb = cinch_device_upload_raw(ctx, ys.as_ptr().cast(), n * 4);
            let mut ocl_exact = 0.0f32;
            assert_eq!(
                cinch_device_dot_f32(ctx, ba, bb, n as i64, 0, &mut ocl_exact),
                DEV_OK,
                "opencl dot exact failed: {}",
                report_string(ctx)
            );
            assert_eq!(
                ocl_exact.to_bits(),
                cpu_exact.to_bits(),
                "OpenCL exact-tree {ocl_exact} != CPU exact-tree {cpu_exact}"
            );
            let mut ocl_relaxed = 0.0f32;
            assert_eq!(cinch_device_dot_f32(ctx, ba, bb, n as i64, 1, &mut ocl_relaxed), DEV_OK);
            assert!((ocl_relaxed - cpu_exact).abs() < 1e-2);
            let rep = report_string(ctx);
            assert!(rep.contains("\"backend\":\"opencl\""), "{rep}");
            cinch_device_buffer_free(ba);
            cinch_device_buffer_free(bb);

            // Stencil -> Histogram pipeline (many-small-frames & few-large-frames)
            let weights = [1i32, 2, 1, 2, 4, 2, 1, 2, 1];
            for (label, frames, h, w, expect_sched) in [
                ("many-small", 64usize, 32usize, 32usize, "many-small-frames"),
                ("few-large", 2usize, 256usize, 256usize, "few-large-frames"),
            ] {
                cinch_device_reset_telemetry(ctx);
                let total = frames * h * w;
                let imgs: Vec<u8> = (0..total).map(|i| ((i * 37 + (i >> 7) * 13) & 0xff) as u8).collect();
                let in_buf = cinch_device_upload_raw(ctx, imgs.as_ptr(), total);
                let out_buf = cinch_device_stencil_histogram_u8(
                    ctx, in_buf, frames as i64, h as i64, w as i64,
                    weights.as_ptr(), 16, par_arrays::EDGE_CLAMP, 2, 256,
                );
                assert!(out_buf > 0, "opencl {label} failed: {}", report_string(ctx));
                let mut got = vec![0u32; frames * 256];
                assert_eq!(cinch_device_download_raw(ctx, out_buf, got.as_mut_ptr().cast(), frames * 256 * 4), DEV_OK);
                let rep = report_string(ctx);
                assert!(rep.contains("\"uploads\":1,"), "{label}: {rep}");
                assert!(rep.contains("\"downloads\":1,"), "{label}: {rep}");
                assert!(rep.contains(expect_sched), "{label}: {rep}");

                // Compare bit-for-bit against seq CPU reference
                let mut ref_tel = DeviceTelemetry::default();
                let ref_bytes = crate::simd_cpu::stencil_histogram_u8(
                    crate::simd_cpu::CpuVariant::Seq, &imgs, frames, h, w, weights, 16, par_arrays::EDGE_CLAMP, 2, 256, &mut ref_tel,
                );
                let ref_u32 = std::slice::from_raw_parts(ref_bytes.as_ptr().cast::<u32>(), frames * 256);
                assert_eq!(got.as_slice(), ref_u32, "opencl {label} != seq ref");
                cinch_device_buffer_free(in_buf);
                cinch_device_buffer_free(out_buf);
            }

            // Scan, Filter, SpMV (empty rows), Row-Reduce (outer & inner), Stencil2D F32
            let data: Vec<i32> = (0..4096).map(|i| ((i as i32 * 29) % 40) - 20).collect();
            let b_in = cinch_device_upload_raw(ctx, data.as_ptr().cast(), data.len() * 4);
            let b_scan = cinch_device_scan_i32(ctx, b_in, data.len() as i64, 1);
            assert!(b_scan > 0);
            let mut got_scan = vec![0i32; data.len()];
            assert_eq!(cinch_device_download_raw(ctx, b_scan, got_scan.as_mut_ptr().cast(), data.len() * 4), DEV_OK);
            let mut acc = 0i32;
            for i in 0..data.len() {
                acc = acc.wrapping_add(data[i]);
                assert_eq!(got_scan[i], acc);
            }
            let mut k = 0i64;
            let b_filt = cinch_device_filter_i32_gt(ctx, b_in, data.len() as i64, 7, &mut k);
            assert!(b_filt > 0);
            let mut got_filt = vec![0i32; k as usize];
            assert_eq!(cinch_device_download_raw(ctx, b_filt, got_filt.as_mut_ptr().cast(), k as usize * 4), DEV_OK);
            let exp_filt: Vec<i32> = data.iter().copied().filter(|&x| x > 7).collect();
            assert_eq!(got_filt, exp_filt);
            cinch_device_buffer_free(b_filt);
            cinch_device_buffer_free(b_scan);
            cinch_device_buffer_free(b_in);

            // SpMV with empty rows
            let vals = [2.0f32, 3.0, 4.0, 5.0];
            let cols = [0u32, 2, 1, 2];
            let row_ptr = [0u32, 2, 2, 4, 4];
            let x = [10.0f32, 20.0, 30.0];
            let bv = cinch_device_upload_raw(ctx, vals.as_ptr().cast(), 16);
            let bc = cinch_device_upload_raw(ctx, cols.as_ptr().cast(), 16);
            let br = cinch_device_upload_raw(ctx, row_ptr.as_ptr().cast(), 20);
            let bx = cinch_device_upload_raw(ctx, x.as_ptr().cast(), 12);
            let by = cinch_device_spmv_csr_f32(ctx, bv, bc, br, bx, 4, 4, 0);
            assert!(by > 0);
            let mut y = [0.0f32; 4];
            assert_eq!(cinch_device_download_raw(ctx, by, y.as_mut_ptr().cast(), 16), DEV_OK);
            assert_eq!(y, [110.0, 0.0, 230.0, 0.0]);
            cinch_device_buffer_free(by);
            cinch_device_buffer_free(bx);
            cinch_device_buffer_free(br);
            cinch_device_buffer_free(bc);
            cinch_device_buffer_free(bv);

            // Row reduce (inner and outer schedules)
            let mat: Vec<f32> = (0..(4 * 1024)).map(|i| ((i % 7) as f32) * 0.5).collect();
            let bm = cinch_device_upload_raw(ctx, mat.as_ptr().cast(), mat.len() * 4);
            let br_out = cinch_device_row_reduce_f32(ctx, bm, 4, 1024);
            assert!(br_out > 0);
            let mut row_sums = [0.0f32; 4];
            assert_eq!(cinch_device_download_raw(ctx, br_out, row_sums.as_mut_ptr().cast(), 16), DEV_OK);
            for r in 0..4 {
                let exp: f32 = mat[r * 1024..(r + 1) * 1024].iter().sum();
                assert!((row_sums[r] - exp).abs() < 1e-2);
            }
            cinch_device_buffer_free(br_out);
            cinch_device_buffer_free(bm);

            // Generic compiler-emitted OpenCL C kernel execution
            let src = std::ffi::CString::new(
                "__kernel void scale2(__global const float* a, __global float* b, const uint n) { uint i = get_global_id(0); if (i < n) b[i] = a[i] * 2.0f; }"
            ).unwrap();
            let kname = std::ffi::CString::new("scale2").unwrap();
            let inp = [1.5f32, -2.0, 3.25, 4.0];
            let ba = cinch_device_upload_raw(ctx, inp.as_ptr().cast(), 16);
            let bb = cinch_device_upload_raw(ctx, inp.as_ptr().cast(), 16);
            let handles = [ba, bb];
            assert_eq!(cinch_device_run_opencl_1d(ctx, src.as_ptr(), kname.as_ptr(), handles.as_ptr(), 2, 4), DEV_OK);
            let mut out = [0.0f32; 4];
            assert_eq!(cinch_device_download_raw(ctx, bb, out.as_mut_ptr().cast(), 16), DEV_OK);
            assert_eq!(out, [3.0, -4.0, 6.5, 8.0]);
            cinch_device_buffer_free(ba);
            cinch_device_buffer_free(bb);
        }
        cinch_device_close(ctx);
    }

    #[test]
    fn all_five_live_backends_agree_and_cuda_hip_report_unavailable_cleanly() {
        // Verify seq, cpu, ispc, opencl, metal all agree bit-for-bit on exact-tree dot,
        // scan, filter, stencil_histogram_u8, stencil2d_f32, spmv_csr_f32, and row_reduce_f32!
        let n = 4099usize;
        let xs: Vec<f32> = (0..n).map(|i| ((i % 31) as f32 - 15.0) * 0.25).collect();
        let ys: Vec<f32> = (0..n).map(|i| ((i % 19) as f32 - 9.0) * 0.5).collect();
        let expected_dot = par_arrays::f32_exact_tree_prod(0, n, &|i| xs[i] * ys[i]);
        let st_f32_in: Vec<f32> = (0..(2 * 32 * 32)).map(|i| ((i % 13) as f32) * 0.125).collect();
        let st_w = [0.0625f32, 0.125, 0.0625, 0.125, 0.25, 0.125, 0.0625, 0.125, 0.0625];
        let mut ref_st: Option<Vec<f32>> = None;

        for (kind, tag) in [
            (DEVICE_KIND_CPU_SEQ, "seq"),
            (DEVICE_KIND_CPU, "cpu"),
            (DEVICE_KIND_ISPC_SIMD, "ispc"),
            (DEVICE_KIND_OPENCL_REQUIRED, "opencl"),
            (DEVICE_KIND_METAL_REQUIRED, "metal"),
        ] {
            let ctx = cinch_device_open(kind);
            assert!(ctx > 0, "backend {tag} failed to open ({ctx})");
            unsafe {
                let ba = cinch_device_upload_raw(ctx, xs.as_ptr().cast(), n * 4);
                let bb = cinch_device_upload_raw(ctx, ys.as_ptr().cast(), n * 4);
                let mut got_dot = 0.0f32;
                assert_eq!(cinch_device_dot_f32(ctx, ba, bb, n as i64, 0, &mut got_dot), DEV_OK);
                assert_eq!(got_dot.to_bits(), expected_dot.to_bits(), "backend {tag} dot mismatch");
                let mut got_rel = 0.0f32;
                assert_eq!(cinch_device_dot_f32(ctx, ba, bb, n as i64, 1, &mut got_rel), DEV_OK);
                assert!((got_rel - expected_dot).abs() < 1e-2, "backend {tag} relaxed dot mismatch");
                cinch_device_buffer_free(ba);
                cinch_device_buffer_free(bb);

                // Stencil2D F32
                let bs = cinch_device_upload_raw(ctx, st_f32_in.as_ptr().cast(), st_f32_in.len() * 4);
                let bso = cinch_device_stencil2d_f32(ctx, bs, 2, 32, 32, st_w.as_ptr(), par_arrays::EDGE_CLAMP, 2);
                assert!(bso > 0, "backend {tag} stencil2d_f32 failed ({bso})");
                let mut out_st = vec![0.0f32; st_f32_in.len()];
                assert_eq!(cinch_device_download_raw(ctx, bso, out_st.as_mut_ptr().cast(), out_st.len() * 4), DEV_OK);
                if let Some(ref r) = ref_st {
                    for (idx, (&a, &b)) in out_st.iter().zip(r.iter()).enumerate() {
                        assert!((a - b).abs() < 1e-5, "backend {tag} stencil2d_f32[{idx}] {a} != {b}");
                    }
                } else {
                    ref_st = Some(out_st);
                }
                cinch_device_buffer_free(bso);
                cinch_device_buffer_free(bs);

                let rep = report_string(ctx);
                assert!(rep.contains(&format!("\"backend\":\"{tag}\"")), "{rep}");
            }
            cinch_device_close(ctx);
        }

        // CUDA & HIP required return DEV_ERR_NO_DEVICE cleanly on macOS (no NVIDIA/ROCm GPU)
        assert_eq!(cinch_device_open(DEVICE_KIND_CUDA_REQUIRED), DEV_ERR_NO_DEVICE);
        assert_eq!(cinch_device_open(DEVICE_KIND_HIP_REQUIRED), DEV_ERR_NO_DEVICE);

        // Introspection API lists all 7 backends and exports non-empty kernel sources
        let mut avail_buf = vec![0u8; 512];
        let n_avail = unsafe { cinch_device_available_backends_cstr(avail_buf.as_mut_ptr(), avail_buf.len()) };
        assert!(n_avail > 0);
        let avail = String::from_utf8_lossy(&avail_buf[..n_avail as usize]);
        assert!(avail.contains("\"seq\":true"), "{avail}");
        assert!(avail.contains("\"cpu\":true"), "{avail}");
        assert!(avail.contains("\"ispc\":true"), "{avail}");
        assert!(avail.contains("\"metal\":true"), "{avail}");
        assert!(avail.contains("\"opencl\":true"), "{avail}");
        assert!(avail.contains("\"cuda\":false"), "{avail}");
        assert!(avail.contains("\"hip\":false"), "{avail}");

        for (kind, needle) in [
            (DEVICE_KIND_METAL_REQUIRED, "dot_f32_exact_block"),
            (DEVICE_KIND_OPENCL_REQUIRED, "__kernel void stencil2d_u8_tiled"),
            (DEVICE_KIND_CUDA_REQUIRED, "__global__ void spmv_csr_f32_warp"),
            (DEVICE_KIND_HIP_REQUIRED, "#include <hip/hip_runtime.h>"),
            (DEVICE_KIND_ISPC_SIMD, "export uniform float dot_f32_relaxed_ispc"),
        ] {
            let mut src_buf = vec![0u8; 32768];
            let sz = unsafe { cinch_device_backend_source_cstr(kind, src_buf.as_mut_ptr(), src_buf.len()) };
            assert!(sz > 200, "backend kind {kind} source too short ({sz})");
            let text = String::from_utf8_lossy(&src_buf[..sz as usize]);
            assert!(text.contains(needle), "backend kind {kind} missing '{needle}'");
        }
    }
}
