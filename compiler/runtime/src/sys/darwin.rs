//! The Darwin backend: `libSystem` symbols, declared here.
//!
//! Darwin's syscall numbers are not a stable ABI. They are generated per
//! release and Apple documents the C library as the interface, so a raw
//! `syscall` instruction on this target is a bet on one OS build. Go
//! reaches Darwin the same way, and so does this module: the entry
//! points are `libSystem` symbols declared below, which is the only
//! route available here (design/base-as-a-package.md section 5).
//!
//! The declarations are ours and the error number is still data: every
//! call answers `sys`'s `(result, errno)` pair, with the number read
//! from `__error()`, this platform's per-thread `errno` slot, right
//! after the call that set it.
//!
//! This module is the one the project actually runs on: the development
//! machine is macOS arm64, so the test in [`super::tests`] exercises
//! these bindings (and [`super::getpid`] checks one against `std`).

use std::ffi::c_void;
use std::ptr::null_mut;

/// `pthread_key_t` on Darwin is `unsigned long`.
pub(crate) type Key = usize;

/// `PATH_MAX` on Darwin: the longest name a path may be, terminator
/// excluded.
pub(crate) const PATH_MAX: usize = 1024;

/// `O_RDONLY`.
const O_RDONLY: i32 = 0;
/// `O_CLOEXEC`: the descriptor must not survive an `exec`.
const O_CLOEXEC: i32 = 0x100_0000;

/// `PROT_READ`.
const PROT_READ: i32 = 1;
/// `PROT_WRITE`.
const PROT_WRITE: i32 = 2;
/// `MAP_PRIVATE`.
const MAP_PRIVATE: i32 = 2;
/// `MAP_ANON`.
const MAP_ANON: i32 = 0x1000;

// ---------------------------------------------------------------------------
// The declarations
// ---------------------------------------------------------------------------
//
// Everything the runtime asks of the OS on this target, in one block:
// the descriptor calls, the process calls, the memory calls, the signal
// and thread calls, the terminal calls, and the environment. All of
// them are `libSystem`.

mod c {
    use std::ffi::c_void;

    use super::Key;

    unsafe extern "C" {
        pub(super) fn write(fd: i32, buf: *const u8, n: usize) -> isize;
        pub(super) fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
        pub(super) fn close(fd: i32) -> i32;
        // Variadic, and it must be declared that way: Apple's arm64 ABI
        // passes variadic arguments on the stack, so a fixed-argument
        // declaration would put the mode (and `ioctl`'s argument) in a
        // register the callee never reads.
        pub(super) fn open(path: *const u8, flags: i32, ...) -> i32;
        pub(super) fn _exit(code: i32) -> !;
        pub(super) fn getpid() -> i32;
        pub(super) fn kill(pid: i32, sig: i32) -> i32;
        pub(super) fn pause() -> i32;
        pub(super) fn mmap(
            addr: *mut c_void,
            len: usize,
            prot: i32,
            flags: i32,
            fd: i32,
            off: i64,
        ) -> *mut c_void;
        pub(super) fn munmap(addr: *mut c_void, len: usize) -> i32;
        pub(super) fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
        pub(super) fn ioctl(fd: i32, request: u64, ...) -> i32;
        pub(super) fn getpagesize() -> i32;
        pub(super) fn sigaction(sig: i32, act: *const c_void, old: *mut c_void) -> i32;
        pub(super) fn sigaltstack(ss: *const c_void, old: *mut c_void) -> i32;
        pub(super) fn pthread_self() -> usize;
        pub(super) fn pthread_key_create(
            key: *mut Key,
            dtor: Option<unsafe extern "C" fn(*mut c_void)>,
        ) -> i32;
        pub(super) fn pthread_getspecific(key: Key) -> *mut c_void;
        pub(super) fn pthread_setspecific(key: Key, value: *const c_void) -> i32;
        pub(super) fn pthread_get_stackaddr_np(thread: usize) -> *mut c_void;
        pub(super) fn pthread_get_stacksize_np(thread: usize) -> usize;
        pub(super) fn posix_openpt(flags: i32) -> i32;
        pub(super) fn grantpt(fd: i32) -> i32;
        pub(super) fn unlockpt(fd: i32) -> i32;
        pub(super) fn ptsname_r(fd: i32, buf: *mut u8, len: usize) -> i32;
        pub(super) fn getenv(name: *const u8) -> *const u8;
        pub(super) fn atexit(f: extern "C" fn()) -> i32;
        // This platform's per-thread `errno` slot (`__errno_location`
        // elsewhere).
        pub(super) fn __error() -> *mut i32;
    }
}

/// The error number of a call that answered `last`: 0 when the answer is
/// a value, and `__error()`'s number when it is negative. Every call
/// declared above answers a count, a descriptor or an address on
/// success, so a negative answer is a failure on all of them (and a
/// successful `mmap` on this target is always below `2^47`).
fn errno_of(last: i64) -> i64 {
    if last < 0 {
        i64::from(unsafe { *c::__error() })
    } else {
        0
    }
}

/// `(result, errno)` for a call that has just returned `last`.
fn pair(last: i64) -> (i64, i64) {
    (last, errno_of(last))
}

// ---------------------------------------------------------------------------
// Handles and bytes
// ---------------------------------------------------------------------------

pub(crate) fn write(fd: i64, buf: &[u8]) -> (i64, i64) {
    let r = unsafe { c::write(fd as i32, buf.as_ptr(), buf.len()) };
    pair(r as i64)
}

pub(crate) fn read(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    let r = unsafe { c::read(fd as i32, buf.as_mut_ptr(), buf.len()) };
    pair(r as i64)
}

pub(crate) fn close(fd: i64) -> (i64, i64) {
    pair(i64::from(unsafe { c::close(fd as i32) }))
}

pub(crate) fn open(name: &[u8], flags: i32) -> (i64, i64) {
    // The mode is read only with `O_CREAT`, which no caller here sets.
    pair(i64::from(unsafe { c::open(name.as_ptr(), flags, 0i32) }))
}

pub(crate) fn open_read(name: &[u8]) -> (i64, i64) {
    open(name, O_RDONLY | O_CLOEXEC)
}

pub(crate) fn getenv(name: &[u8]) -> *const u8 {
    unsafe { c::getenv(name.as_ptr()) }
}

// ---------------------------------------------------------------------------
// This process
// ---------------------------------------------------------------------------

pub(crate) fn exit(code: i32) -> ! {
    unsafe { c::_exit(code) }
}

pub(crate) fn getpid() -> i64 {
    i64::from(unsafe { c::getpid() })
}

pub(crate) fn kill(pid: i64, sig: i64) -> (i64, i64) {
    pair(i64::from(unsafe { c::kill(pid as i32, sig as i32) }))
}

/// `pause` answers `-1` with `EINTR` once a signal arrives, and never a
/// success value: the same contract as the Linux `rt_sigsuspend` route.
pub(crate) fn pause() -> (i64, i64) {
    pair(i64::from(unsafe { c::pause() }))
}

pub(crate) fn atexit(f: extern "C" fn()) -> (i64, i64) {
    pair(i64::from(unsafe { c::atexit(f) }))
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

pub(crate) fn mmap_anon(size: usize) -> (i64, i64) {
    let p = unsafe {
        c::mmap(
            null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON,
            -1,
            0,
        )
    };
    // `MAP_FAILED` is `-1`, so the answer needs no separate check.
    pair(p as i64)
}

pub(crate) fn munmap(addr: i64, size: usize) -> (i64, i64) {
    pair(i64::from(unsafe { c::munmap(addr as *mut c_void, size) }))
}

pub(crate) fn mprotect(addr: i64, size: usize, prot: i32) -> (i64, i64) {
    pair(i64::from(unsafe {
        c::mprotect(addr as *mut c_void, size, prot)
    }))
}

pub(crate) fn getpagesize() -> usize {
    (unsafe { c::getpagesize() }).max(1) as usize
}

// ---------------------------------------------------------------------------
// Signals and threads
// ---------------------------------------------------------------------------

pub(crate) unsafe fn sigaction(sig: i32, act: *const c_void, old: *mut c_void) -> (i64, i64) {
    pair(i64::from(unsafe { c::sigaction(sig, act, old) }))
}

pub(crate) unsafe fn sigaltstack(ss: *const c_void, old: *mut c_void) -> (i64, i64) {
    pair(i64::from(unsafe { c::sigaltstack(ss, old) }))
}

/// `pthread_get_stackaddr_np` answers the HIGH end of the stack and
/// `pthread_get_stacksize_np` its size, so the low end is the
/// difference. Both are Darwin extensions, which is why this is a
/// per-target function rather than a caller's arithmetic.
pub(crate) fn this_thread_stack() -> Option<(usize, usize)> {
    let thread = unsafe { c::pthread_self() };
    let top = unsafe { c::pthread_get_stackaddr_np(thread) } as usize;
    let size = unsafe { c::pthread_get_stacksize_np(thread) };
    top.checked_sub(size).map(|low| (low, size))
}

pub(crate) fn thread_key_create() -> Option<Key> {
    let mut key: Key = 0;
    if unsafe { c::pthread_key_create(&mut key, None) } == 0 {
        Some(key)
    } else {
        None
    }
}

pub(crate) unsafe fn pthread_getspecific(key: Key) -> *mut c_void {
    unsafe { c::pthread_getspecific(key) }
}

pub(crate) unsafe fn pthread_setspecific(key: Key, value: *const c_void) -> (i64, i64) {
    pair(i64::from(unsafe { c::pthread_setspecific(key, value) }))
}

// ---------------------------------------------------------------------------
// Terminals
// ---------------------------------------------------------------------------

pub(crate) unsafe fn posix_openpt(flags: i32) -> (i64, i64) {
    pair(i64::from(unsafe { c::posix_openpt(flags) }))
}

pub(crate) unsafe fn grantpt(fd: i64) -> (i64, i64) {
    pair(i64::from(unsafe { c::grantpt(fd as i32) }))
}

pub(crate) unsafe fn unlockpt(fd: i64) -> (i64, i64) {
    pair(i64::from(unsafe { c::unlockpt(fd as i32) }))
}

/// Darwin's `ptsname_r` answers `-1` with `errno` set (glibc answers
/// the error number), and a positive answer is taken as the number too,
/// so both shapes land in the same pair.
pub(crate) fn ptsname(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    let r = unsafe { c::ptsname_r(fd as i32, buf.as_mut_ptr(), buf.len()) };
    if r == 0 {
        (0, 0)
    } else if r > 0 {
        (-1, i64::from(r))
    } else {
        (-1, errno_of(i64::from(r)))
    }
}

pub(crate) fn ioctl(fd: i64, request: u64, arg: *const u8) -> (i64, i64) {
    pair(i64::from(unsafe { c::ioctl(fd as i32, request, arg) }))
}
