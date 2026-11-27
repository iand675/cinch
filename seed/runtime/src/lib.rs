//! Array acceleration runtime support for Cinch.
//!
//! All core Cinch runtime subsystems (memory allocator, refcounting, arenas,
//! bignums, strings, bytes, symbols, text/crypto, non-accel arrays/tensors/
//! columnar, slots, sync primitives, stack guard, fiber scheduler, tasks,
//! cooperative runtime, work-stealing `par`, platform OS/FS/process,
//! networking, PTY, runtime evaluator, and hot evolution) are implemented
//! directly in Cinch under `seed/runtime/cinch/Runtime.*.cinch` and merged
//! into `libcinchruntime.a` as `cinch_runtime.o`.
//!
//! This Rust crate provides ONLY:
//! - Array acceleration primitives: parallel CPU array ops (`par_arrays`),
//!   SIMD CPU (`simd_cpu`), Metal (`metal`), OpenCL (`opencl`), CUDA (`cuda`),
//!   HIP (`hip`), and WebGPU (`webgpu`).
//! - The `#[global_allocator]` wiring so Rust heap allocations inside those
//!   accelerator backends share the Cinch runtime's heap cap and accounting.
//! - Minimal thin FFI shims importing Cinch runtime symbols and `dlopen`/`dlsym`.

use std::alloc::{GlobalAlloc, Layout};
#[cfg(test)]
use std::alloc::System;

#[cfg(not(test))]
unsafe extern "C" {
    fn cinch_rt_global_alloc(size: usize, align: usize, zeroed: u8) -> *mut u8;
    fn cinch_rt_global_dealloc(ptr: *mut u8, size: usize, align: usize);
    fn cinch_rt_global_realloc(
        ptr: *mut u8,
        old_size: usize,
        align: usize,
        new_size: usize,
    ) -> *mut u8;
}

struct CinchAllocator;

#[global_allocator]
static ALLOCATOR: CinchAllocator = CinchAllocator;

#[cfg(not(test))]
unsafe impl GlobalAlloc for CinchAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { cinch_rt_global_alloc(layout.size(), layout.align(), 0) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { cinch_rt_global_alloc(layout.size(), layout.align(), 1) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { cinch_rt_global_dealloc(ptr, layout.size(), layout.align()) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { cinch_rt_global_realloc(ptr, layout.size(), layout.align(), new_size) }
    }
}

#[cfg(test)]
unsafe impl GlobalAlloc for CinchAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Thin FFI imports of Cinch-implemented core runtime primitives used by
/// the array acceleration modules below.
pub(crate) mod abi {
    unsafe extern "C" {
        pub(crate) fn cinch_panic(ptr: *const u8, len: usize) -> !;
        pub(crate) fn cinch_alloc(size: i64, kind: i64) -> i64;
        pub(crate) fn cinch_alloc_raw(size: i64, kind: i64) -> *mut u8;
        pub(crate) fn cinch_bytes_from_data(ptr: *const u8, len: i64) -> i64;
        pub(crate) fn cinch_nat_from_i64(v: i64) -> i64;
    }

    #[cold]
    #[inline(never)]
    pub(crate) fn panic_exit(msg: &[u8]) -> ! {
        unsafe { cinch_panic(msg.as_ptr(), msg.len()) }
    }
}

pub(crate) mod rc {
    unsafe extern "C" {
        pub(crate) fn cinch_dup(word: i64);
        pub(crate) fn cinch_drop(word: i64);
        pub(crate) fn cinch_mark_mt(word: i64);
        fn cinch_rt_reclaiming() -> u8;
    }

    #[inline]
    pub(crate) fn reclaiming() -> bool {
        unsafe { cinch_rt_reclaiming() != 0 }
    }
}

pub(crate) mod par {
    unsafe extern "C" {
        fn cinch_rt_par_budget() -> usize;
    }

    #[inline]
    pub(crate) fn budget() -> usize {
        unsafe { cinch_rt_par_budget() }
    }
}

pub(crate) mod heap {
    pub(crate) fn env_bytes(name: &std::ffi::CStr) -> Option<Vec<u8>> {
        let key = name.to_str().ok()?;
        let val = std::env::var_os(key)?;
        Some(val.as_encoded_bytes().to_vec())
    }
}

pub(crate) mod task {
    type ApplyStub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

    pub(crate) unsafe fn apply_closure(clo: i64, mut arg: i64) -> i64 {
        let code = unsafe { *(clo as usize as *const i64) };
        let stub: ApplyStub = unsafe { std::mem::transmute(code as usize) };
        unsafe { stub(clo, 1, &mut arg) }
    }
}

pub(crate) mod arrays {
    use crate::abi;

    const KIND_ARRAY: i64 = 14;
    const KIND_SHIFT: u32 = 35;
    const KIND_MASK: u64 = 0x1F;
    const RC_MASK: u64 = (1u64 << 35) - 1;
    const RC_FRESH: i64 = 1;

    unsafe extern "C" {
        pub(crate) fn cinch_array_len(a: i64) -> i64;
    }

    fn corrupt() -> ! {
        abi::panic_exit(b"compiler bug: corrupt array value")
    }

    #[inline]
    unsafe fn payload(word: i64) -> *mut i64 {
        if word == 0 || word & 7 != 0 {
            corrupt();
        }
        let p = word as usize as *mut u8;
        let hdr = unsafe { p.sub(8).cast::<u64>().read() };
        if ((hdr >> KIND_SHIFT) & KIND_MASK) as i64 != KIND_ARRAY {
            corrupt();
        }
        p.cast()
    }

    #[inline]
    pub(crate) unsafe fn is_exclusive(word: i64) -> bool {
        if word == 0 || word & 7 != 0 {
            return false;
        }
        let p = word as usize as *const u8;
        let hdr = unsafe { p.sub(8).cast::<u64>().read() };
        let rc = (((hdr & RC_MASK) as i64) << 29) >> 29;
        rc == RC_FRESH
    }

    #[inline]
    pub(crate) fn elem_width(format: i64) -> usize {
        if format <= 2 {
            8
        } else {
            (format & 0x7f) as usize
        }
    }

    #[inline]
    pub(crate) fn counted(format: i64) -> bool {
        format == 1 || format == 2
    }

    #[inline]
    pub(crate) unsafe fn format(a: i64) -> i64 {
        unsafe { payload(a).read() }
    }

    #[inline]
    pub(crate) unsafe fn data_ptr(a: i64) -> *mut u8 {
        unsafe { payload(a).add(2).cast::<u8>() }
    }

    pub(crate) unsafe fn array(len: i64, format: i64) -> i64 {
        let w = elem_width(format);
        if len < 0
            || (!(0..=2).contains(&format)
                && !(format & 0x100 != 0 && matches!(w, 1 | 2 | 4 | 8)))
        {
            corrupt();
        }
        let bytes = (len as usize)
            .checked_mul(w)
            .and_then(|n| n.checked_add(16))
            .unwrap_or_else(|| abi::panic_exit(b"array allocation overflow"));
        let word = unsafe { abi::cinch_alloc(bytes as i64, KIND_ARRAY) };
        let p = word as usize as *mut i64;
        unsafe {
            p.write(format);
            p.add(1).write(len);
        }
        word
    }

    pub(crate) unsafe fn alloc_uninit_packed(len: i64, format: i64) -> i64 {
        let w = elem_width(format);
        if len < 0 || format & 0x100 == 0 || !matches!(w, 1 | 2 | 4 | 8) {
            corrupt();
        }
        let bytes = (len as usize)
            .checked_mul(w)
            .and_then(|n| n.checked_add(16))
            .unwrap_or_else(|| abi::panic_exit(b"array allocation overflow"));
        let p = unsafe { abi::cinch_alloc_raw(bytes as i64, KIND_ARRAY) }.cast::<i64>();
        unsafe {
            p.write(format);
            p.add(1).write(len);
        }
        p as usize as i64
    }

    pub(crate) unsafe fn read(a: i64, i: i64) -> i64 {
        let f = unsafe { format(a) };
        let w = elem_width(f);
        let p = unsafe { payload(a).add(2).cast::<u8>().add(i as usize * w) };
        let v = match w {
            1 => unsafe { p.read() as u64 },
            2 => unsafe { p.cast::<u16>().read_unaligned() as u64 },
            4 => unsafe { p.cast::<u32>().read_unaligned() as u64 },
            8 => unsafe { p.cast::<u64>().read_unaligned() },
            _ => corrupt(),
        };
        if f > 2 && f & 0x80 != 0 {
            let shift = 64 - w * 8;
            ((v << shift) as i64) >> shift
        } else {
            v as i64
        }
    }

    pub(crate) unsafe fn write(a: i64, i: i64, value: i64) {
        let f = unsafe { format(a) };
        let w = elem_width(f);
        let p = unsafe { payload(a).add(2).cast::<u8>().add(i as usize * w) };
        match w {
            1 => unsafe { p.write(value as u8) },
            2 => unsafe { p.cast::<u16>().write_unaligned(value as u16) },
            4 => unsafe { p.cast::<u32>().write_unaligned(value as u32) },
            8 => unsafe { p.cast::<i64>().write_unaligned(value) },
            _ => corrupt(),
        }
    }
}

pub(crate) mod sys {
    use std::ffi::c_void;

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    unsafe extern "C" {
        #[link_name = "dlopen"]
        fn c_dlopen(path: *const u8, mode: i32) -> *mut c_void;
        #[link_name = "dlsym"]
        fn c_dlsym(handle: *mut c_void, symbol: *const u8) -> *mut c_void;
    }

    #[cfg(target_os = "macos")]
    const RTLD_FLAGS: i32 = 0x1 | 0x8; // RTLD_LAZY | RTLD_GLOBAL

    #[cfg(target_os = "linux")]
    const RTLD_FLAGS: i32 = 0x1; // RTLD_LAZY

    pub(crate) fn dlopen(path: &[u8]) -> *mut c_void {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if path.len() > 4096 || path.contains(&0) {
                return std::ptr::null_mut();
            }
            let mut buf = [0u8; 4097];
            buf[..path.len()].copy_from_slice(path);
            unsafe { c_dlopen(buf.as_ptr(), RTLD_FLAGS) }
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            let _ = path;
            std::ptr::null_mut()
        }
    }

    pub(crate) fn dlsym(handle: *mut c_void, sym: &[u8]) -> *mut c_void {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if sym.len() > 4096 || sym.contains(&0) {
                return std::ptr::null_mut();
            }
            let mut buf = [0u8; 4097];
            buf[..sym.len()].copy_from_slice(sym);
            unsafe { c_dlsym(handle, buf.as_ptr()) }
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            let _ = (handle, sym);
            std::ptr::null_mut()
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) mod wasm {
        pub(crate) mod webgpu_host {
            #[link(wasm_import_module = "cinch_webgpu")]
            extern "C" {
                pub(crate) fn cinch_webgpu_available() -> i32;
                pub(crate) fn cinch_webgpu_open() -> i32;
                pub(crate) fn cinch_webgpu_close(ctx: i32);
                pub(crate) fn cinch_webgpu_alloc(ctx: i32, bytes: u32) -> i32;
                pub(crate) fn cinch_webgpu_upload(ctx: i32, ptr: *const u8, bytes: u32) -> i32;
                pub(crate) fn cinch_webgpu_download(ctx: i32, buf: i32, dst: *mut u8, bytes: u32) -> i32;
                pub(crate) fn cinch_webgpu_free(ctx: i32, buf: i32);
                pub(crate) fn cinch_webgpu_dot_f32(
                    ctx: i32,
                    a: i32,
                    b: i32,
                    n: u32,
                    exact: i32,
                    out_val: *mut f32,
                ) -> i32;
                pub(crate) fn cinch_webgpu_scan_i32(ctx: i32, in_buf: i32, n: u32, inclusive: i32) -> i32;
                pub(crate) fn cinch_webgpu_filter_i32_gt(
                    ctx: i32,
                    in_buf: i32,
                    n: u32,
                    threshold: i32,
                    out_len: *mut u32,
                ) -> i32;
                pub(crate) fn cinch_webgpu_stencil_hist_u8(
                    ctx: i32,
                    in_buf: i32,
                    f: u32,
                    h: u32,
                    w: u32,
                    ws_ptr: *const i32,
                    divisor: i32,
                    edge_policy: u32,
                    nsteps: u32,
                    bins: u32,
                ) -> i32;
                pub(crate) fn cinch_webgpu_stencil2d_f32(
                    ctx: i32,
                    in_buf: i32,
                    f: u32,
                    h: u32,
                    w: u32,
                    ws_ptr: *const f32,
                    edge_policy: u32,
                    nsteps: u32,
                ) -> i32;
                pub(crate) fn cinch_webgpu_spmv_csr_f32(
                    ctx: i32,
                    vals: i32,
                    cols: i32,
                    row_ptr: i32,
                    x: i32,
                    r: u32,
                    nz: u32,
                    exact: i32,
                ) -> i32;
                pub(crate) fn cinch_webgpu_row_reduce_f32(ctx: i32, in_buf: i32, r: u32, c: u32) -> i32;
            }
        }
    }
}

pub mod par_arrays;
pub mod metal;
pub mod opencl;
pub mod cuda;
pub mod hip;
pub mod webgpu;
pub mod simd_cpu;
