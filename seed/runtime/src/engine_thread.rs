//! Share-nothing pinned OS worker threads, page/DMA-aligned `mmap` slabs,
//! and per-thread allocation & atomic-RC telemetry counters.
//!
//! Unlike [`crate::sched`] (which multiplexes M:N cooperative fibers over a
//! shared work-stealing pool), this module provides dedicated 1:1 OS threads
//! intended for share-nothing engine shards:
//!
//! * Linux workers pin themselves to `core_id % cpu_count` via the raw
//!   `sched_setaffinity` syscall.
//! * Darwin workers request high-throughput performance-core scheduling via
//!   `pthread_set_qos_class_self_np` resolved through [`crate::sys::dlopen`] /
//!   [`crate::sys::dlsym`], keeping every foreign OS declaration inside
//!   `src/sys/`.
//! * DMA slabs come straight from [`crate::sys::mmap_anon`] /
//!   [`crate::sys::munmap`] (zeroed, page-aligned `>= 4096`, power-of-two,
//!   with best-effort `MADV_HUGEPAGE` on Linux for slabs `>= 2 MiB`).
//! * Per-thread telemetry counters expose cumulative heap allocations
//!   ([`crate::heap`]) and atomic RC operations ([`crate::rc`]) with zero
//!   extra instructions on the non-atomic single-threaded RC fast path.

use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Mutex, OnceLock};

/// Minimum DMA alignment in bytes (4 KiB).
const MIN_DMA_ALIGN: usize = 4096;

/// Threshold for best-effort `MADV_HUGEPAGE` hint on Linux (2 MiB).
#[cfg(target_os = "linux")]
const HUGEPAGE_THRESHOLD: usize = 2 << 20;

/// Global handle table for spawned pinned OS worker threads.
static WORKERS: OnceLock<Mutex<HashMap<i64, std::thread::JoinHandle<i64>>>> = OnceLock::new();
static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);

fn workers() -> &'static Mutex<HashMap<i64, std::thread::JoinHandle<i64>>> {
    WORKERS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Pin the calling OS thread to logical CPU `core_id` (best-effort on
/// platforms without hard CPU affinity).
fn configure_worker_thread(core_id: usize) {
    #[cfg(target_os = "linux")]
    {
        pin_linux(core_id);
    }
    #[cfg(target_os = "macos")]
    {
        let _ = core_id;
        set_darwin_high_throughput_qos();
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        let _ = core_id;
    }
}

/// Linux: pin the calling thread (`pid = 0`) to `core_id` using the raw
/// `sched_setaffinity` syscall (zero libc dependency, no foreign declaration
/// outside `src/sys/`).
#[cfg(target_os = "linux")]
fn pin_linux(core_id: usize) {
    // Support up to 1024 logical CPUs (16 * 64 bits = 128 bytes).
    if core_id >= 1024 {
        return;
    }
    let mut mask = [0u64; 16];
    mask[core_id / 64] = 1u64 << (core_id % 64);
    let len = std::mem::size_of_val(&mask) as i64;
    let ptr = mask.as_ptr() as i64;

    #[cfg(target_arch = "x86_64")]
    unsafe {
        const SYS_SCHED_SETAFFINITY: i64 = 203;
        let _ret: i64;
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_SCHED_SETAFFINITY => _ret,
            in("rdi") 0i64,
            in("rsi") len,
            in("rdx") ptr,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        const SYS_SCHED_SETAFFINITY: i64 = 122;
        let _ret: i64;
        core::arch::asm!(
            "svc 0",
            inlateout("x0") 0i64 => _ret,
            in("x1") len,
            in("x2") ptr,
            in("x8") SYS_SCHED_SETAFFINITY,
            options(nostack, preserves_flags)
        );
    }
}

/// Linux: best-effort `madvise(addr, len, MADV_HUGEPAGE)` for large DMA slabs.
#[cfg(target_os = "linux")]
fn advise_hugepage_linux(addr: i64, len: usize) {
    const MADV_HUGEPAGE: i64 = 14;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        const SYS_MADVISE: i64 = 28;
        let _ret: i64;
        core::arch::asm!(
            "syscall",
            inlateout("rax") SYS_MADVISE => _ret,
            in("rdi") addr,
            in("rsi") len as i64,
            in("rdx") MADV_HUGEPAGE,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        const SYS_MADVISE: i64 = 233;
        let _ret: i64;
        core::arch::asm!(
            "svc 0",
            inlateout("x0") addr => _ret,
            in("x1") len as i64,
            in("x2") MADV_HUGEPAGE,
            in("x8") SYS_MADVISE,
            options(nostack, preserves_flags)
        );
    }
}

/// Darwin: set high-throughput QoS (`QOS_CLASS_USER_INITIATED`) on the calling
/// thread via `libSystem.B.dylib`, resolved once through `crate::sys`.
#[cfg(target_os = "macos")]
fn set_darwin_high_throughput_qos() {
    type SetQosFn = unsafe extern "C" fn(u32, i32) -> i32;
    /// `QOS_CLASS_USER_INITIATED` (`0x19` in `<sys/qos.h>`): high-throughput
    /// P-core execution class on Apple Silicon and Intel macOS.
    const QOS_CLASS_USER_INITIATED: u32 = 0x19;

    static FN_PTR: OnceLock<Option<SetQosFn>> = OnceLock::new();
    let f = *FN_PTR.get_or_init(|| {
        let lib = crate::sys::dlopen(b"/usr/lib/libSystem.B.dylib");
        if lib.is_null() {
            return None;
        }
        let sym = crate::sys::dlsym(lib, b"pthread_set_qos_class_self_np");
        if sym.is_null() {
            return None;
        }
        Some(unsafe { std::mem::transmute::<*mut std::ffi::c_void, SetQosFn>(sym) })
    });
    if let Some(set_qos) = f {
        unsafe {
            let _ = set_qos(QOS_CLASS_USER_INITIATED, 0);
        }
    }
}

/// Return the number of available logical CPUs (`>= 1`).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_cpu_count() -> i64 {
    #[cfg(target_os = "unknown")]
    {
        1
    }
    #[cfg(not(target_os = "unknown"))]
    {
        std::thread::available_parallelism()
            .ok()
            .map(usize::from)
            .unwrap_or(1)
            .max(1) as i64
    }
}

/// Spawn a dedicated OS worker thread (not a `sched.rs` fiber), pin it to
/// `core_id % cpu_count` on Linux (or high-throughput QoS on Darwin), invoke
/// the Cinch closure `fn_closure` with `arg`, and return an opaque handle
/// (`> 0`) or `-1` on error.
///
/// # Safety
///
/// `fn_closure` must be a well-formed Cinch closure record word
/// (`{ code; arity; nfilled; saved[0..] }` preceded by its 64-bit block header
/// at offset `-8`) whose apply stub accepts one `i64` argument.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_engine_spawn_pinned(
    core_id: i64,
    fn_closure: i64,
    arg: i64,
) -> i64 {
    if fn_closure == 0 || (fn_closure & 1) != 0 {
        return -1;
    }
    #[cfg(target_os = "unknown")]
    {
        let _ = (core_id, arg);
        return -1;
    }
    #[cfg(not(target_os = "unknown"))]
    {
        let cpus = cinch_engine_cpu_count().max(1);
        let pinned_core = core_id.rem_euclid(cpus) as usize;

        // If reclamation is active, mark the closure graph atomic before the
        // worker OS thread starts so any captured blocks shared with the
        // spawner stay data-race-free. Static/immortal closures (`rc == 0`)
        // are untouched and incur zero atomic RC operations on the worker.
        if crate::rc::reclaiming() {
            unsafe {
                crate::rc::cinch_mark_mt(fn_closure);
            }
        }

        let spawned = std::thread::Builder::new().spawn(move || {
            configure_worker_thread(pinned_core);
            // Reset per-thread counters right before entering user code so
            // OS/std thread bootstrap overhead is not charged to the worker.
            crate::heap::reset_thread_alloc_count();
            crate::rc::reset_thread_atomic_rc_count();
            unsafe { crate::task::apply_closure(fn_closure, arg) }
        });

        let Ok(join_handle) = spawned else {
            return -1;
        };

        let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
        if handle <= 0 {
            let _ = join_handle.join();
            return -1;
        }
        let mut table = workers().lock().unwrap_or_else(|e| e.into_inner());
        table.insert(handle, join_handle);
        handle
    }
}

/// Join the pinned OS worker thread identified by `handle`, release its handle
/// entry, and return the worker's `i64` result (or `-1` if `handle` is invalid
/// or the worker panicked).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_join(handle: i64) -> i64 {
    if handle <= 0 {
        return -1;
    }
    let join_handle = {
        let mut table = workers().lock().unwrap_or_else(|e| e.into_inner());
        table.remove(&handle)
    };
    let Some(join_handle) = join_handle else {
        return -1;
    };
    join_handle.join().unwrap_or(-1)
}

/// Allocate zeroed, page/DMA-aligned memory via anonymous `mmap`.
///
/// `size_bytes` must be `> 0`. `align_bytes` must be `>= 4096` and a power of
/// two (passing `0` defaults to `4096`). Returns the raw aligned address as
/// `i64`, or `0` on invalid arguments or allocation failure.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_dma_alloc(size_bytes: i64, align_bytes: i64) -> i64 {
    if size_bytes <= 0 || align_bytes < 0 {
        return 0;
    }
    let req_align = if align_bytes == 0 {
        MIN_DMA_ALIGN
    } else {
        align_bytes as usize
    };
    if req_align < MIN_DMA_ALIGN || !req_align.is_power_of_two() {
        return 0;
    }
    let page_size = crate::sys::getpagesize().max(MIN_DMA_ALIGN);
    let Some(mapped_size) = (size_bytes as usize).checked_next_multiple_of(page_size) else {
        return 0;
    };

    let addr = if req_align <= page_size {
        let (ptr, _) = crate::sys::mmap_anon(mapped_size);
        if ptr <= 0 {
            return 0;
        }
        ptr
    } else {
        // Over-aligned request (`req_align > page_size`): over-reserve and trim
        // page-aligned leading/trailing margins so `cinch_engine_dma_free` can
        // unmap `[aligned, aligned + mapped_size)` directly with zero metadata.
        let Some(total) = mapped_size.checked_add(req_align) else {
            return 0;
        };
        let (base, _) = crate::sys::mmap_anon(total);
        if base <= 0 {
            return 0;
        }
        let base_u = base as usize;
        let mask = req_align - 1;
        let aligned_u = (base_u + mask) & !mask;
        let prefix = aligned_u - base_u;
        let end_u = base_u + total;
        let used_end = aligned_u + mapped_size;
        let suffix = end_u - used_end;
        if prefix > 0 {
            let _ = crate::sys::munmap(base, prefix);
        }
        if suffix > 0 {
            let _ = crate::sys::munmap(used_end as i64, suffix);
        }
        aligned_u as i64
    };

    #[cfg(target_os = "linux")]
    if mapped_size >= HUGEPAGE_THRESHOLD {
        advise_hugepage_linux(addr, mapped_size);
    }

    addr
}

/// Unmap a DMA slab previously allocated with [`cinch_engine_dma_alloc`].
/// Returns `0` on success or `-1` on invalid arguments / `munmap` failure.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_dma_free(ptr: i64, size_bytes: i64) -> i64 {
    if ptr <= 0 || size_bytes <= 0 {
        return -1;
    }
    let page_size = crate::sys::getpagesize().max(MIN_DMA_ALIGN);
    if !(ptr as usize).is_multiple_of(page_size) {
        return -1;
    }
    let Some(mapped_size) = (size_bytes as usize).checked_next_multiple_of(page_size) else {
        return -1;
    };
    let (res, _) = crate::sys::munmap(ptr, mapped_size);
    if res == 0 { 0 } else { -1 }
}

/// Return the calling OS thread's cumulative heap allocation count.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_thread_alloc_count() -> i64 {
    crate::heap::thread_alloc_count() as i64
}

/// Return the calling OS thread's cumulative atomic RC operation count.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_thread_atomic_rc_count() -> i64 {
    crate::rc::thread_atomic_rc_count() as i64
}

/// Reset the calling OS thread's heap allocation and atomic RC counters to `0`.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_engine_thread_counters_reset() -> i64 {
    crate::heap::reset_thread_alloc_count();
    crate::rc::reset_thread_atomic_rc_count();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{self, test_shapes::CONS_T, allocate_described, KIND_CLOSURE, KIND_CTOR};

    /// Static immortal closure block matching the Cinch closure ABI:
    /// header at offset `-8`, followed by `{ code, arity, nfilled }`.
    #[repr(C, align(8))]
    struct StaticClosure {
        header: u64,
        code: i64,
        arity: i64,
        nfilled: i64,
    }

    impl StaticClosure {
        fn new(stub: unsafe extern "C" fn(i64, i64, *mut i64) -> i64) -> Self {
            Self {
                header: block::static_header(KIND_CLOSURE, 0),
                code: stub as usize as i64,
                arity: 1,
                nfilled: 0,
            }
        }

        fn word(&self) -> i64 {
            (&self.code as *const i64) as usize as i64
        }
    }

    unsafe extern "C" fn square_plus_one(_self: i64, argc: i64, argv: *mut i64) -> i64 {
        assert_eq!(argc, 1);
        let x = unsafe { *argv };
        x * x + 1
    }

    unsafe extern "C" fn no_alloc_loop_worker(_self: i64, argc: i64, argv: *mut i64) -> i64 {
        assert_eq!(argc, 1);
        let iters = unsafe { *argv };
        cinch_engine_thread_counters_reset();
        let mut acc: i64 = 0;
        for i in 0..iters {
            acc = acc.wrapping_add(i.wrapping_mul(3) ^ 0x5a5a);
        }
        let allocs = cinch_engine_thread_alloc_count();
        let atomic_rcs = cinch_engine_thread_atomic_rc_count();
        if allocs != 0 || atomic_rcs != 0 {
            return -1000 - allocs - atomic_rcs;
        }
        acc
    }

    #[test]
    fn cpu_count_is_at_least_one() {
        assert!(cinch_engine_cpu_count() >= 1);
    }

    #[test]
    fn spawn_and_join_multiple_pinned_workers() {
        let closure = StaticClosure::new(square_plus_one);
        let w = closure.word();
        let mut handles = Vec::new();
        for core in 0..4i64 {
            let h = unsafe { cinch_engine_spawn_pinned(core, w, core + 10) };
            assert!(h > 0, "expected valid handle, got {h}");
            handles.push((core, h));
        }
        for (core, h) in handles {
            let got = cinch_engine_join(h);
            let expected = (core + 10) * (core + 10) + 1;
            assert_eq!(got, expected);
        }
        // Joining an invalid or already-joined handle returns -1.
        assert_eq!(cinch_engine_join(0), -1);
        assert_eq!(cinch_engine_join(-5), -1);
        assert_eq!(cinch_engine_join(999_999_999), -1);
    }

    #[test]
    fn spawn_rejects_null_and_immediate_closures() {
        assert_eq!(unsafe { cinch_engine_spawn_pinned(0, 0, 1) }, -1);
        assert_eq!(unsafe { cinch_engine_spawn_pinned(0, 3, 1) }, -1);
    }

    #[test]
    fn dma_alloc_respects_4k_and_overaligned_and_frees_cleanly() {
        // Default 4 KiB alignment, zeroed memory.
        let p4k = cinch_engine_dma_alloc(4096, 4096);
        assert_ne!(p4k, 0);
        assert_eq!(p4k as usize % 4096, 0);
        let slice = unsafe { std::slice::from_raw_parts_mut(p4k as usize as *mut u8, 4096) };
        assert!(slice.iter().all(|&b| b == 0), "DMA slab must be zeroed");
        slice[0] = 0xAA;
        slice[4095] = 0x55;
        assert_eq!(cinch_engine_dma_free(p4k, 4096), 0);

        // Over-aligned 64 KiB DMA slab.
        let p64k = cinch_engine_dma_alloc(8192, 65536);
        assert_ne!(p64k, 0);
        assert_eq!(p64k as usize % 65536, 0);
        assert_eq!(cinch_engine_dma_free(p64k, 8192), 0);

        // Invalid sizes / alignments return 0 / -1.
        assert_eq!(cinch_engine_dma_alloc(0, 4096), 0);
        assert_eq!(cinch_engine_dma_alloc(-16, 4096), 0);
        assert_eq!(cinch_engine_dma_alloc(4096, 2048), 0);
        assert_eq!(cinch_engine_dma_alloc(4096, 6000), 0);
        assert_eq!(cinch_engine_dma_free(0, 4096), -1);
        assert_eq!(cinch_engine_dma_free(123, 4096), -1);
    }

    #[test]
    fn pinned_worker_observes_zero_allocs_and_zero_atomic_rc_in_pure_loop() {
        let closure = StaticClosure::new(no_alloc_loop_worker);
        let h = unsafe { cinch_engine_spawn_pinned(0, closure.word(), 10_000) };
        assert!(h > 0);
        let res = cinch_engine_join(h);
        let mut expected: i64 = 0;
        for i in 0..10_000i64 {
            expected = expected.wrapping_add(i.wrapping_mul(3) ^ 0x5a5a);
        }
        assert_eq!(res, expected, "worker observed unexpected alloc/atomic-rc: {res}");
    }

    #[test]
    fn thread_counters_track_local_allocs_and_atomic_rc_only_on_mt_path() {
        cinch_engine_thread_counters_reset();
        assert_eq!(cinch_engine_thread_alloc_count(), 0);
        assert_eq!(cinch_engine_thread_atomic_rc_count(), 0);

        // Allocate a local constructor block and dup/drop it single-threaded:
        // alloc count increases, atomic RC count MUST stay 0!
        let p = unsafe { allocate_described(16, KIND_CTOR, CONS_T, false) }.cast::<i64>();
        unsafe {
            p.add(0).write(1); // immediate
            p.add(1).write(0); // null
        }
        let word = p as usize as i64;
        assert!(cinch_engine_thread_alloc_count() >= 1);
        unsafe {
            crate::rc::cinch_dup(word);
            crate::rc::cinch_drop(word);
        }
        assert_eq!(
            cinch_engine_thread_atomic_rc_count(),
            0,
            "single-threaded RC fast path must record 0 atomic RC ops"
        );

        // Now mark it atomic and exercise atomic dup + atomic drops:
        unsafe {
            crate::rc::cinch_mark_mt(word);
            crate::rc::cinch_dup(word);
            crate::rc::cinch_drop(word);
            crate::rc::cinch_drop(word);
        }
        assert_eq!(
            cinch_engine_thread_atomic_rc_count(),
            3,
            "expected 1 atomic dup + 2 atomic drops"
        );

        // Reset clears both counters.
        assert_eq!(cinch_engine_thread_counters_reset(), 0);
        assert_eq!(cinch_engine_thread_alloc_count(), 0);
        assert_eq!(cinch_engine_thread_atomic_rc_count(), 0);
    }
}
