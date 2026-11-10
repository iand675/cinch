//! The syscall layer: the one place in this crate that names the
//! operating system.
//!
//! # The rule
//!
//! This directory is the only one allowed to declare a foreign
//! function. `sys/tests.rs` walks `compiler/runtime/src` and fails when
//! any other file does, so the whole surface the runtime borrows from
//! the OS is auditable here (design/base-as-a-package.md section 5). A
//! `#[unsafe(no_mangle)] pub extern "C" fn` DEFINITION, which is what
//! every `cinch_*` export is, is not a declaration and stays where it
//! lives; only a declaration of somebody else's symbol has to move.
//!
//! # The interface
//!
//! Words and bytes only: `i64` descriptors, counts and addresses,
//! `&[u8]` names and buffers, `i32` flags and signal numbers. No
//! `std::io`, no `io::Error`, no C types, so a failure is data the
//! caller maps onto the named vocabulary of [`crate::platform`].
//!
//! Every call that can fail answers `(result, errno)`:
//!
//! * `result` is the call's own value on success (a byte count, a
//!   descriptor, an address) and `-1` on failure. One convention for
//!   both targets, so a caller never has to know whether the kernel
//!   answered `-errno` (the Linux `syscall` convention) or `-1` with
//!   `errno` set (the C library's).
//! * `errno` is the platform's positive error number (`ENOENT` 2,
//!   `EACCES` 13, `EISDIR` 21, `EINVAL` 22, ...) and 0 when the call
//!   succeeded. The numbers stay the platform's own: [`crate::platform`]
//!   maps the six POSIX-fixed ones onto the named codes and everything
//!   else onto `IO_ERROR`.
//!
//! A call with no failure mode answers a plain value instead: a
//! `getpagesize` or `getpid` number, or the `Option` of
//! `this_thread_stack` and `thread_key_create`.
//!
//! # The two backends
//!
//! * `linux` issues the `syscall` instruction with numbers pinned per
//!   architecture in its own table (x86_64 and aarch64), because those
//!   numbers are a stable ABI and pinning them is what will let a static
//!   binary drop the C library.
//! * `darwin` calls `libSystem` symbols declared in that module,
//!   because Darwin's syscall numbers are not a stable ABI and move
//!   between releases. Go makes the same choice for the same reason.
//!
//! # What is still the C library, on purpose
//!
//! Linux has no syscall for the pthread ABI, for the signal-disposition
//! structs, for the page size, for `getenv`/`atexit`, or for the
//! pseudo-terminal allocation (`grantpt` issues its own control calls).
//! Those stay C library calls, declared in the marked section of
//! `linux`, so the "one auditable place" property holds even where a
//! syscall route does not exist yet. Darwin declares them beside the
//! rest because every call there is a `libSystem` symbol anyway.
//!
//! The pointer arguments `sigaction`, `sigaltstack`, `ioctl` and
//! `posix_openpt` take are opaque to this module: the struct layouts
//! are data, not entry points, and the caller that owns the layout
//! ([`crate::stack`] for the signal structs, [`crate::pty`] for the
//! window size) keeps it. When the stack trap becomes its own seam
//! (design section 5.1) those layouts move in here too.
//!
//! # What is still `std`
//!
//! Allocation, threads, mutexes, subprocesses and name resolution stay
//! `std`, which links the C library regardless. "No libc at all"
//! (`#![no_std]`, our allocator, our `_start`) is a later rung of the
//! same design, and this module is the seam it will grow from.

#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "macos")]
pub(crate) mod darwin;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!(
    "the syscall layer has backends for linux and darwin only; \
     another target needs its own module here (design/base-as-a-package.md section 5)"
);

#[cfg(target_os = "linux")]
use linux as back;
#[cfg(target_os = "macos")]
use darwin as back;

#[cfg(test)]
mod tests;

use std::ffi::c_void;

/// `EINTR`: 4 on both targets, fixed by POSIX. A caller that loops on a
/// short read or write compares the `errno` of a `-1` result with it.
pub(crate) const EINTR: i64 = 4;

/// `EINVAL`: 22 on both targets, fixed by POSIX. `open` and
/// `open_read` answer it for a name this interface cannot spell.
pub(crate) const EINVAL: i64 = 22;

/// A thread-specific key, as wide as the platform's `pthread_key_t`:
/// `unsigned int` on Linux, `unsigned long` on Darwin.
pub(crate) type Key = back::Key;

// ---------------------------------------------------------------------------
// Handles and bytes
// ---------------------------------------------------------------------------

/// Write up to `buf.len()` bytes to `fd`: the count written, or `-1`
/// and the error number. One kernel call, no retry: a caller that wants
/// the whole buffer written loops on the count (and on `EINTR`).
pub(crate) fn write(fd: i64, buf: &[u8]) -> (i64, i64) {
    back::write(fd, buf)
}

/// Read up to `buf.len()` bytes from `fd`: the count read (0 at the end
/// of the stream), or `-1` and the error number. One kernel call, no
/// retry, for the same reason as `write`.
pub(crate) fn read(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    back::read(fd, buf)
}

/// Close `fd`: 0, or `-1` and the error number.
pub(crate) fn close(fd: i64) -> (i64, i64) {
    back::close(fd)
}

/// Open `path` with `flags` (the platform's own flag bits, which the
/// caller names): a descriptor, or `-1` and the error number. The name
/// is bytes without a NUL; a name longer than `PATH_MAX` or carrying a
/// NUL of its own is `EINVAL`.
pub(crate) fn open(path: &[u8], flags: i32) -> (i64, i64) {
    with_name(path, |name| back::open(name, flags)).unwrap_or_else(|e| (-1, e))
}

/// Open `path` read only, close on exec: a descriptor, or `-1` and the
/// error number. The one open every file read starts with, so the flags
/// stay on this side of the seam.
pub(crate) fn open_read(path: &[u8]) -> (i64, i64) {
    with_name(path, back::open_read).unwrap_or_else(|e| (-1, e))
}

/// The environment entry `name` names (bytes without a NUL), or `None`
/// when it is unset. The answer borrows the environment block, which
/// lives as long as the process, and costs no allocation: this is what
/// the allocator itself reads its settings through.
pub(crate) fn getenv(name: &[u8]) -> Option<&'static [u8]> {
    let p = with_name(name, back::getenv).ok()?;
    if p.is_null() {
        return None;
    }
    Some(unsafe { std::ffi::CStr::from_ptr(p.cast()) }.to_bytes())
}

// ---------------------------------------------------------------------------
// This process
// ---------------------------------------------------------------------------

/// Leave the process at once with `code`: `_exit(2)` semantics, so no
/// `atexit` handler and no buffered output runs. This is the only exit
/// a signal handler or an allocator may take, and it is what the
/// `Exhausted` fault path leaves through.
pub(crate) fn exit(code: i32) -> ! {
    back::exit(code)
}

/// This process's own id. The one call here with no failure mode.
///
/// No runtime path needs it yet (the language side signals through
/// `cinch_signal_send` and reads a child's id from `cinch_pipe_pid`),
/// so it is `allow(dead_code)` and the layer's test is its only caller.
/// It stays because the process-control seam of the design (section
/// 5.1) is where it lands, and a pinned number with a test is cheaper
/// than the second binding it would take to add it later.
#[allow(dead_code)]
pub(crate) fn getpid() -> i64 {
    back::getpid()
}

/// Send signal `sig` to process `pid` (the platform's signal numbers):
/// 0, or `-1` and the error number.
pub(crate) fn kill(pid: i64, sig: i64) -> (i64, i64) {
    back::kill(pid, sig)
}

/// Sleep until a signal arrives: `-1` and `EINTR` on the ordinary
/// path, and never a success value. Used to park a thread that has
/// lost an exit race, so returning at all only ever means another
/// signal arrived.
pub(crate) fn pause() -> (i64, i64) {
    back::pause()
}

/// Register `f` to run when the process exits normally: 0, or `-1` and
/// the error number. Not a kernel entry point (the C library keeps the
/// list), but the allocator's `CINCH_ALLOC_STATS` line needs it before
/// any allocation can happen.
pub(crate) fn atexit(f: extern "C" fn()) -> (i64, i64) {
    back::atexit(f)
}

// ---------------------------------------------------------------------------
// Memory
// ---------------------------------------------------------------------------

/// `size` bytes of read/write private anonymous memory: the address, or
/// `-1` and the error number. Anonymous memory is what the stack guard
/// and the alternate signal stack are made of.
pub(crate) fn mmap_anon(size: usize) -> (i64, i64) {
    back::mmap_anon(size)
}

/// Reserve `size` bytes of private anonymous address space with no
/// access, committing nothing: the address, or `-1` and the error
/// number. [`mprotect`] makes parts of it usable. The small-object
/// allocator ([`crate::small`]) carves its pages from one such range.
pub(crate) fn mmap_reserve(size: usize) -> (i64, i64) {
    back::mmap_reserve(size)
}

/// Release a mapping `mmap_anon` made: 0, or `-1` and the error
/// number.
pub(crate) fn munmap(addr: i64, size: usize) -> (i64, i64) {
    back::munmap(addr, size)
}

/// Set the access protection of `size` bytes at `addr` (the platform's
/// `PROT_*` bits, which the caller names): 0, or `-1` and the error
/// number.
pub(crate) fn mprotect(addr: i64, size: usize, prot: i32) -> (i64, i64) {
    back::mprotect(addr, size, prot)
}

/// A word that identifies the calling thread among the live threads, read
/// from the thread register in one instruction: the thread's TSD base
/// (`TPIDRRO_EL0`, low bits masked) on Darwin arm64, the thread pointer
/// (`TPIDR_EL0`, or the TCB self pointer at `fs:0`) on Linux. A thread
/// that has exited may have its word reused by a later thread. Targets
/// without such a register answer [`NO_THREAD_WORD`], which no thread
/// matches. Not a system call: it is here because the registers are the
/// platform's ABI.
#[inline(always)]
pub(crate) fn thread_word() -> usize {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        let w: usize;
        // SAFETY: reading the read-only thread register has no effects.
        unsafe { std::arch::asm!("mrs {}, tpidrro_el0", out(reg) w, options(nomem, nostack, preserves_flags)) };
        w & !7
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        let w: usize;
        // SAFETY: as above.
        unsafe { std::arch::asm!("mrs {}, tpidr_el0", out(reg) w, options(nomem, nostack, preserves_flags)) };
        w
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        let w: usize;
        // SAFETY: `fs:0` is the TCB's pointer to itself on x86_64 Linux,
        // readable by its thread at any time.
        unsafe { std::arch::asm!("mov {}, fs:0", out(reg) w, options(nostack, preserves_flags, readonly)) };
        w
    }
    #[cfg(not(any(
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "linux", any(target_arch = "aarch64", target_arch = "x86_64"))
    )))]
    {
        NO_THREAD_WORD
    }
}

/// What [`thread_word`] answers where no thread register is read; no
/// thread's word is ever this value (a thread register holds an aligned
/// address).
pub(crate) const NO_THREAD_WORD: usize = 1;

/// The system page size in bytes. Cannot fail: the platform answers a
/// fixed power of two for the life of the process.
pub(crate) fn getpagesize() -> usize {
    back::getpagesize()
}

// ---------------------------------------------------------------------------
// Signals and threads
// ---------------------------------------------------------------------------

/// Replace the disposition of `sig`. `act` points at the target's
/// `struct sigaction` and `old`, when not null, receives the disposition
/// that was there: 0, or `-1` and the error number.
///
/// # Safety
///
/// Both pointers must be null or point at a live `struct sigaction`
/// with this target's layout ([`crate::stack`] owns it).
pub(crate) unsafe fn sigaction(sig: i32, act: *const c_void, old: *mut c_void) -> (i64, i64) {
    unsafe { back::sigaction(sig, act, old) }
}

/// Install or read the calling thread's alternate signal stack. `ss`
/// points at the target's `stack_t`, and `old`, when not null, receives
/// the one that was there: 0, or `-1` and the error number.
///
/// # Safety
///
/// Both pointers must be null or point at a live `stack_t` with this
/// target's layout ([`crate::stack`] owns it).
pub(crate) unsafe fn sigaltstack(ss: *const c_void, old: *mut c_void) -> (i64, i64) {
    unsafe { back::sigaltstack(ss, old) }
}

/// The calling thread's stack as `(low address, size)`, or `None` when
/// the platform cannot describe it.
pub(crate) fn this_thread_stack() -> Option<(usize, usize)> {
    back::this_thread_stack()
}

/// Create a thread-specific key with no destructor, or `None` when the
/// platform has no slot left. The signal handler reads its zone through
/// this key, and `pthread_getspecific` neither allocates nor takes a
/// lock, which is what makes the read safe inside the handler.
pub(crate) fn thread_key_create() -> Option<Key> {
    back::thread_key_create()
}

/// The value this thread stored under `key`, or null.
///
/// # Safety
///
/// The value is returned as it was stored, so the caller owns what it
/// points at (and a thread that never stored anything answers null).
pub(crate) unsafe fn pthread_getspecific(key: Key) -> *mut c_void {
    unsafe { back::pthread_getspecific(key) }
}

/// Store `value` under `key` for the calling thread: 0, or `-1` and the
/// error number.
///
/// # Safety
///
/// `value` must stay valid while any thread can read it through `key`.
pub(crate) unsafe fn pthread_setspecific(key: Key, value: *const c_void) -> (i64, i64) {
    unsafe { back::pthread_setspecific(key, value) }
}

// ---------------------------------------------------------------------------
// Terminals
// ---------------------------------------------------------------------------

/// Allocate the master end of a pseudo-terminal pair with `flags`: a
/// descriptor, or `-1` and the error number.
///
/// # Safety
///
/// A C library routine with no argument the caller owns beyond the
/// flags; declared `unsafe` because it is a foreign call.
pub(crate) unsafe fn posix_openpt(flags: i32) -> (i64, i64) {
    unsafe { back::posix_openpt(flags) }
}

/// Give the pair's slave to this user with a terminal's mode: 0, or `-1`
/// and the error number.
///
/// # Safety
///
/// `fd` must be a master `posix_openpt` returned.
pub(crate) unsafe fn grantpt(fd: i64) -> (i64, i64) {
    unsafe { back::grantpt(fd) }
}

/// Clear the slave's lock, so its path can be opened: 0, or `-1` and the
/// error number.
///
/// # Safety
///
/// `fd` must be a master `posix_openpt` returned.
pub(crate) unsafe fn unlockpt(fd: i64) -> (i64, i64) {
    unsafe { back::unlockpt(fd) }
}

/// Write the slave's device path for `fd` into `buf` (the answer is a
/// NUL-terminated name inside `buf`): 0, or `-1` and the error number
/// (`ERANGE` when `buf` is too short).
pub(crate) fn ptsname(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    back::ptsname(fd, buf)
}

/// The terminal control call: 0, or `-1` and the error number.
///
/// # Safety
///
/// `arg` must be null or point at the argument `request` expects
/// ([`crate::pty`] owns the window-size layout).
pub(crate) fn ioctl(fd: i64, request: u64, arg: *const u8) -> (i64, i64) {
    back::ioctl(fd, request, arg)
}

// ---------------------------------------------------------------------------
// Names
// ---------------------------------------------------------------------------

/// Run `f` with `bytes` as a NUL-terminated name, or answer `EINVAL`
/// when the name cannot be one: longer than `PATH_MAX`, or carrying a
/// NUL of its own (which the kernel would read as the end of the name,
/// silently opening a different file).
fn with_name<R>(bytes: &[u8], f: impl FnOnce(&[u8]) -> R) -> Result<R, i64> {
    let mut name = [0u8; back::PATH_MAX + 1];
    if bytes.len() > back::PATH_MAX || bytes.contains(&0) {
        return Err(EINVAL);
    }
    name[..bytes.len()].copy_from_slice(bytes);
    Ok(f(&name[..bytes.len() + 1]))
}
