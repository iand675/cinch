//! The freestanding backend: a module with no operating system under it.
//!
//! A bare `wasm32-unknown-unknown` module has linear memory, its own
//! function table, and nothing else. There is no filesystem, no process
//! table, no signals, no threads and no environment, so the calls this
//! backend answers fall into two groups.
//!
//! **Answered.** `getenv` reports every name unset, which is what a
//! module with no environment has; the allocator's settings therefore
//! take their defaults. `getpagesize` reports the WebAssembly page,
//! which is fixed by the specification.
//!
//! **Refused.** Everything that needs an operating system answers the
//! `ENOSYS` convention rather than pretending: a program that reaches
//! one learns that the target has no such call, which is the honest
//! answer and the one the language's named errors can carry. The exit
//! path traps, because a freestanding module has no process to leave;
//! the generated `main` returns its status to the host instead.
//!
//! What is *not* here is as deliberate as what is: no `open`, no `mmap`,
//! no `pthread`, no `sigaction`. A freestanding target that needs any of
//! them needs a host to provide them, which is what the WASI backend
//! would be.

use std::ffi::c_void;

/// A thread-local storage key. No thread has storage here.
pub(crate) type Key = u32;

/// `PATH_MAX` on Linux, reused as the longest name this layer accepts:
/// a freestanding module opens nothing, so the bound is only the shape
/// the shared caller checks against.
pub(crate) const PATH_MAX: usize = 4096;

/// `ENOSYS`: the call is not implemented on this target.
const ENOSYS: i64 = 38;

/// `ENOMEM`: linear memory could not grow.
const ENOMEM: i64 = 12;

/// The WebAssembly page size, fixed by the specification.
const WASM_PAGE: usize = 65536;

fn unsupported() -> (i64, i64) {
    (-1, ENOSYS)
}

pub(super) fn write(_fd: i64, _buf: &[u8]) -> (i64, i64) {
    unsupported()
}

pub(super) fn read(_fd: i64, _buf: &mut [u8]) -> (i64, i64) {
    unsupported()
}

pub(super) fn close(_fd: i64) -> (i64, i64) {
    unsupported()
}

pub(super) fn open(_path: &[u8], _flags: i32) -> (i64, i64) {
    unsupported()
}

pub(super) fn open_read(_name: &[u8]) -> (i64, i64) {
    unsupported()
}

/// A module has no environment, so no name is set.
pub(super) fn getenv(_name: &[u8]) -> *const u8 {
    core::ptr::null()
}

/// There is no process to leave. The generated `main` returns its status
/// to the host, so no correct program reaches this; one that does has a
/// runtime invariant broken, and a trap is the only honest answer.
pub(super) fn exit(_code: i32) -> ! {
    core::arch::wasm32::unreachable()
}

pub(super) fn getpid() -> i64 {
    1
}

pub(super) fn kill(_pid: i64, _sig: i64) -> (i64, i64) {
    unsupported()
}

pub(super) fn pause() -> (i64, i64) {
    unsupported()
}

pub(super) fn atexit(_f: extern "C" fn()) -> (i64, i64) {
    unsupported()
}

/// Where the next `memory.grow` region starts, so a reservation and the
/// commit that follows it stay in one contiguous run.
static NEXT_REGION: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// The address `memory.grow` would return for the next page.
///
/// WebAssembly's linear memory is the whole address space: `memory.grow`
/// extends it by whole 64 KiB pages and reports the first new page. That
/// is what this target has in place of `mmap`, so a mapping here is a
/// growth of the linear memory, and an address is an offset into it.
fn memory_size_bytes() -> usize {
    core::arch::wasm32::memory_size(0) * WASM_PAGE
}

/// Reserve `size` bytes by growing linear memory, zero-filled by the
/// specification, and report the first byte's address. Growth is
/// contiguous by construction, so the caller's reserve-then-commit split
/// needs nothing further from this call: the pages are already there and
/// already writable.
pub(super) fn mmap_reserve(size: usize) -> (i64, i64) {
    let pages = size.div_ceil(WASM_PAGE);
    let base = core::arch::wasm32::memory_grow(0, pages);
    if base == usize::MAX {
        return (-1, ENOMEM);
    }
    let addr = base * WASM_PAGE;
    NEXT_REGION.store(addr + pages * WASM_PAGE, std::sync::atomic::Ordering::Release);
    (addr as i64, 0)
}

/// A mapping of `size` bytes at the end of linear memory: a growth, which
/// is the only way to get writable memory here.
pub(super) fn mmap_anon(size: usize) -> (i64, i64) {
    mmap_reserve(size)
}

/// Nothing to release: linear memory only grows, and a page handed back
/// is reused by the caller's own allocator rather than the host.
pub(super) fn munmap(_addr: i64, _size: usize) -> (i64, i64) {
    (0, 0)
}

/// Every page of linear memory is writable once grown, so the commit a
/// reservation's caller performs is already done. Reporting success is
/// the honest answer: the memory is there and usable.
pub(super) fn mprotect(_addr: i64, _size: usize, _prot: i32) -> (i64, i64) {
    (0, 0)
}

pub(super) fn getpagesize() -> usize {
    WASM_PAGE
}

pub(super) fn ioctl(_fd: i64, _request: u64, _arg: *const u8) -> (i64, i64) {
    unsupported()
}

pub(super) unsafe fn sigaction(
    _sig: i32,
    _act: *const c_void,
    _old: *mut c_void,
) -> (i64, i64) {
    unsupported()
}

pub(super) unsafe fn sigaltstack(_ss: *const c_void, _old: *mut c_void) -> (i64, i64) {
    unsupported()
}

pub(super) fn this_thread_stack() -> Option<(usize, usize)> {
    None
}

pub(super) fn thread_key_create() -> Option<Key> {
    None
}

pub(super) unsafe fn pthread_getspecific(_key: Key) -> *mut c_void {
    std::ptr::null_mut()
}

pub(super) unsafe fn pthread_setspecific(_key: Key, _value: *const c_void) -> (i64, i64) {
    unsupported()
}

pub(super) fn posix_openpt(_flags: i32) -> (i64, i64) {
    unsupported()
}

pub(super) fn grantpt(_fd: i64) -> (i64, i64) {
    unsupported()
}

pub(super) fn unlockpt(_fd: i64) -> (i64, i64) {
    unsupported()
}

pub(super) fn ptsname(_fd: i64, _buf: &mut [u8]) -> (i64, i64) {
    unsupported()
}
