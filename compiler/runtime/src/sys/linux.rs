//! The Linux backend: the `syscall` instruction, with the numbers
//! pinned per architecture in this module's own table.
//!
//! Linux syscall numbers are a stable ABI: a number a kernel accepted in
//! 2010 is the same number today, and a raw `syscall` needs no C library
//! at all, which is what will let a static Cinch binary drop glibc
//! (design/base-as-a-package.md section 5).
//!
//! # Coverage, and what is untested
//!
//! The table carries `x86_64` and `aarch64`. NEITHER COLUMN HAS EVER
//! RUN: this project's development machine is macOS arm64, so this
//! module is written from the kernel's tables and checked only by
//! `cargo check` against the Linux target. Treat every number here as
//! unverified until a Linux machine runs `cargo test` in this crate.
//!
//! # The two sections
//!
//! The first section issues raw syscalls and converts the kernel's
//! negative-`errno` convention into `sys`'s `(result, errno)` pair. The
//! second declares C library functions for the calls Linux has no
//! nameable syscall for (the pthread ABI, the signal-disposition
//! structs, the page size, `getenv`/`atexit`, and the pseudo-terminal
//! allocation, whose `grantpt` issues its own control calls). Both are
//! here, in `sys/`, so that this directory stays the one place a foreign
//! symbol is named.

use std::ffi::c_void;
use std::ptr::null_mut;

use super::EINVAL;

/// `pthread_key_t` on Linux is `unsigned int`.
pub(crate) type Key = u32;

/// `PATH_MAX` on Linux: the longest name a path may be, terminator
/// excluded.
pub(crate) const PATH_MAX: usize = 4096;

/// `AT_FDCWD`: the "relative to the working directory" descriptor
/// `openat` takes.
const AT_FDCWD: i64 = -100;

/// `O_RDONLY`.
const O_RDONLY: i32 = 0;
/// `O_CLOEXEC`: the descriptor must not survive an `exec`.
const O_CLOEXEC: i32 = 0o2000000;

/// `PROT_NONE`.
const PROT_NONE: i32 = 0;
/// `PROT_READ`.
const PROT_READ: i32 = 1;
/// `PROT_WRITE`.
const PROT_WRITE: i32 = 2;
/// `MAP_PRIVATE`.
const MAP_PRIVATE: i32 = 2;
/// `MAP_ANONYMOUS` (the same value on every Linux architecture).
const MAP_ANONYMOUS: i32 = 0x20;
/// `MAP_NORESERVE` (the same value on x86_64 and aarch64).
const MAP_NORESERVE: i32 = 0x4000;

// ---------------------------------------------------------------------------
// The pinned numbers
// ---------------------------------------------------------------------------

/// The syscall numbers, from the x86_64 table (`arch/x86/entry/syscalls`).
#[cfg(target_arch = "x86_64")]
mod nr {
    pub(super) const READ: i64 = 0;
    pub(super) const WRITE: i64 = 1;
    pub(super) const CLOSE: i64 = 3;
    pub(super) const MMAP: i64 = 9;
    pub(super) const MPROTECT: i64 = 10;
    pub(super) const MUNMAP: i64 = 11;
    pub(super) const IOCTL: i64 = 16;
    pub(super) const GETPID: i64 = 39;
    pub(super) const KILL: i64 = 62;
    pub(super) const RT_SIGSUSPEND: i64 = 130;
    pub(super) const EXIT_GROUP: i64 = 231;
    pub(super) const OPENAT: i64 = 257;
}

/// The syscall numbers, from the generic table Linux uses on aarch64
/// (`include/uapi/asm-generic/unistd.h`).
#[cfg(target_arch = "aarch64")]
mod nr {
    pub(super) const IOCTL: i64 = 29;
    pub(super) const OPENAT: i64 = 56;
    pub(super) const CLOSE: i64 = 57;
    pub(super) const READ: i64 = 63;
    pub(super) const WRITE: i64 = 64;
    pub(super) const EXIT_GROUP: i64 = 94;
    pub(super) const KILL: i64 = 129;
    pub(super) const RT_SIGSUSPEND: i64 = 133;
    pub(super) const GETPID: i64 = 172;
    pub(super) const MUNMAP: i64 = 215;
    pub(super) const MMAP: i64 = 222;
    pub(super) const MPROTECT: i64 = 226;
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!(
    "the Linux syscall numbers are pinned for x86_64 and aarch64 only; \
     another architecture needs its own column in this table"
);

// ---------------------------------------------------------------------------
// The instruction
// ---------------------------------------------------------------------------

/// The `syscall` instruction: number in `rax`, arguments in `rdi`,
/// `rsi`, `rdx`, `r10`, `r8`, `r9`, result in `rax`. `rcx` and `r11`
/// are the instruction's own scratch registers and are declared
/// clobbered.
///
/// # Safety
///
/// The number and the arguments must be a call this process may make,
/// with pointer arguments pointing at memory the kernel may read or
/// write as that call requires.
#[cfg(target_arch = "x86_64")]
unsafe fn syscall(n: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            in("r10") a4,
            in("r8") a5,
            in("r9") a6,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// The `syscall` instruction: number in `x8`, arguments in `x0` to `x5`,
/// result in `x0`.
///
/// # Safety
///
/// The number and the arguments must be a call this process may make,
/// with pointer arguments pointing at memory the kernel may read or
/// write as that call requires.
#[cfg(target_arch = "aarch64")]
unsafe fn syscall(n: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64, a6: i64) -> i64 {
    let ret: i64;
    unsafe {
        core::arch::asm!(
            "svc 0",
            inlateout("x0") a1 => ret,
            in("x1") a2,
            in("x2") a3,
            in("x3") a4,
            in("x4") a5,
            in("x5") a6,
            in("x8") n,
            options(nostack, preserves_flags)
        );
    }
    ret
}

/// The kernel's convention, made `sys`'s: a result in `-4095..=-1` is
/// `-errno`, so it becomes `(-1, errno)`; anything else is the value
/// itself with no error. No address or count this interface returns can
/// land in that band (`0xffff_ffff_ffff_f001` and up is not a canonical
/// user address on either architecture).
fn split(raw: i64) -> (i64, i64) {
    if (-4095..=-1).contains(&raw) {
        (-1, -raw)
    } else {
        (raw, 0)
    }
}

// ---------------------------------------------------------------------------
// Raw syscalls
// ---------------------------------------------------------------------------

pub(crate) fn write(fd: i64, buf: &[u8]) -> (i64, i64) {
    let raw = unsafe { syscall(nr::WRITE, fd, buf.as_ptr() as i64, buf.len() as i64, 0, 0, 0) };
    split(raw)
}

pub(crate) fn read(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    let raw = unsafe { syscall(nr::READ, fd, buf.as_mut_ptr() as i64, buf.len() as i64, 0, 0, 0) };
    split(raw)
}

pub(crate) fn close(fd: i64) -> (i64, i64) {
    split(unsafe { syscall(nr::CLOSE, fd, 0, 0, 0, 0, 0) })
}

/// `openat` relative to the working directory: the syscall that carries
/// both a name and the flags, so one function covers every open.
pub(crate) fn open(name: &[u8], flags: i32) -> (i64, i64) {
    let raw = unsafe {
        syscall(
            nr::OPENAT,
            AT_FDCWD,
            name.as_ptr() as i64,
            i64::from(flags),
            0,
            0,
            0,
        )
    };
    split(raw)
}

pub(crate) fn open_read(name: &[u8]) -> (i64, i64) {
    open(name, O_RDONLY | O_CLOEXEC)
}

pub(crate) fn exit(code: i32) -> ! {
    unsafe { syscall(nr::EXIT_GROUP, i64::from(code), 0, 0, 0, 0, 0) };
    // `exit_group` ends every thread of the process and does not return.
    unsafe { core::hint::unreachable_unchecked() }
}

pub(crate) fn getpid() -> i64 {
    unsafe { syscall(nr::GETPID, 0, 0, 0, 0, 0, 0) }
}

pub(crate) fn kill(pid: i64, sig: i64) -> (i64, i64) {
    split(unsafe { syscall(nr::KILL, pid, sig, 0, 0, 0, 0) })
}

/// `rt_sigsuspend` over an all-zero mask. Linux has no `pause` syscall;
/// a suspend on the empty mask is the same wait, with the one difference
/// that a signal the thread had blocked is deliverable while it waits.
/// That is the wait's whole purpose here (the process is already
/// leaving), and the caller loops anyway.
pub(crate) fn pause() -> (i64, i64) {
    let mask = [0u64; 1];
    let raw = unsafe { syscall(nr::RT_SIGSUSPEND, mask.as_ptr() as i64, 8, 0, 0, 0, 0) };
    split(raw)
}

pub(crate) fn mmap_anon(size: usize) -> (i64, i64) {
    let raw = unsafe {
        syscall(
            nr::MMAP,
            0,
            size as i64,
            i64::from(PROT_READ | PROT_WRITE),
            i64::from(MAP_PRIVATE | MAP_ANONYMOUS),
            -1,
            0,
        )
    };
    split(raw)
}

pub(crate) fn mmap_reserve(size: usize) -> (i64, i64) {
    let raw = unsafe {
        syscall(
            nr::MMAP,
            0,
            size as i64,
            i64::from(PROT_NONE),
            i64::from(MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE),
            -1,
            0,
        )
    };
    split(raw)
}

pub(crate) fn munmap(addr: i64, size: usize) -> (i64, i64) {
    split(unsafe { syscall(nr::MUNMAP, addr, size as i64, 0, 0, 0, 0) })
}

pub(crate) fn mprotect(addr: i64, size: usize, prot: i32) -> (i64, i64) {
    let raw = unsafe { syscall(nr::MPROTECT, addr, size as i64, i64::from(prot), 0, 0, 0) };
    split(raw)
}

pub(crate) fn ioctl(fd: i64, request: u64, arg: *const u8) -> (i64, i64) {
    let raw = unsafe { syscall(nr::IOCTL, fd, request as i64, arg as i64, 0, 0, 0) };
    split(raw)
}

// ---------------------------------------------------------------------------
// C library, not a syscall
// ---------------------------------------------------------------------------
//
// Everything below is a C library function on Linux: the pthread ABI is
// the library's (the kernel has `clone`, `futex` and thread-local slots,
// not `pthread_create`), a signal disposition crosses as the library's
// `struct sigaction`, the page size comes from the auxiliary vector the
// library already parsed, `getenv` reads the library's `environ`, and
// `grantpt` runs its own control calls. Declared here so this directory
// stays the one place that names a foreign symbol.

/// The declarations themselves.
mod c {
    use std::ffi::c_void;

    use super::Key;

    unsafe extern "C" {
        pub(super) fn sigaction(sig: i32, act: *const c_void, old: *mut c_void) -> i32;
        pub(super) fn sigaltstack(ss: *const c_void, old: *mut c_void) -> i32;
        pub(super) fn pthread_self() -> usize;
        pub(super) fn pthread_key_create(
            key: *mut Key,
            dtor: Option<unsafe extern "C" fn(*mut c_void)>,
        ) -> i32;
        pub(super) fn pthread_getspecific(key: Key) -> *mut c_void;
        pub(super) fn pthread_setspecific(key: Key, value: *const c_void) -> i32;
        pub(super) fn pthread_getattr_np(thread: usize, attr: *mut Attr) -> i32;
        pub(super) fn pthread_attr_getstack(
            attr: *const Attr,
            addr: *mut *mut c_void,
            size: *mut usize,
        ) -> i32;
        pub(super) fn pthread_attr_destroy(attr: *mut Attr) -> i32;
        pub(super) fn getpagesize() -> i32;
        pub(super) fn posix_openpt(flags: i32) -> i32;
        pub(super) fn grantpt(fd: i32) -> i32;
        pub(super) fn unlockpt(fd: i32) -> i32;
        pub(super) fn ptsname_r(fd: i32, buf: *mut u8, len: usize) -> i32;
        pub(super) fn getenv(name: *const u8) -> *const u8;
        pub(super) fn atexit(f: extern "C" fn()) -> i32;
    }

    /// Storage for a `pthread_attr_t`: 56 bytes on glibc x86_64 and
    /// musl, 64 on glibc aarch64. Read only through the calls above.
    #[repr(C, align(16))]
    pub(super) struct Attr(pub(super) [u8; 128]);
}

pub(crate) unsafe fn sigaction(sig: i32, act: *const c_void, old: *mut c_void) -> (i64, i64) {
    split(i64::from(unsafe { c::sigaction(sig, act, old) }))
}

pub(crate) unsafe fn sigaltstack(ss: *const c_void, old: *mut c_void) -> (i64, i64) {
    split(i64::from(unsafe { c::sigaltstack(ss, old) }))
}

pub(crate) fn this_thread_stack() -> Option<(usize, usize)> {
    let mut attr = c::Attr([0; 128]);
    if unsafe { c::pthread_getattr_np(c::pthread_self(), &mut attr) } != 0 {
        return None;
    }
    let mut addr = null_mut();
    let mut size = 0usize;
    let ok = unsafe { c::pthread_attr_getstack(&attr, &mut addr, &mut size) } == 0;
    unsafe { c::pthread_attr_destroy(&mut attr) };
    ok.then_some((addr as usize, size))
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
    split(i64::from(unsafe { c::pthread_setspecific(key, value) }))
}

pub(crate) fn getpagesize() -> usize {
    (unsafe { c::getpagesize() }).max(1) as usize
}

pub(crate) unsafe fn posix_openpt(flags: i32) -> (i64, i64) {
    split(i64::from(unsafe { c::posix_openpt(flags) }))
}

pub(crate) unsafe fn grantpt(fd: i64) -> (i64, i64) {
    split(i64::from(unsafe { c::grantpt(fd as i32) }))
}

pub(crate) unsafe fn unlockpt(fd: i64) -> (i64, i64) {
    split(i64::from(unsafe { c::unlockpt(fd as i32) }))
}

/// glibc's `ptsname_r` answers the error number itself rather than
/// `-1` with `errno` set, so a positive answer is the error.
pub(crate) fn ptsname(fd: i64, buf: &mut [u8]) -> (i64, i64) {
    let r = unsafe { c::ptsname_r(fd as i32, buf.as_mut_ptr(), buf.len()) };
    if r == 0 {
        (0, 0)
    } else if r > 0 {
        (-1, i64::from(r))
    } else {
        // No `errno` of our own to read on Linux (this module issues raw
        // syscalls and keeps no errno slot), so a `-1` answer can only
        // be reported as the invalid argument it would be.
        (-1, EINVAL)
    }
}

pub(crate) fn getenv(name: &[u8]) -> *const u8 {
    unsafe { c::getenv(name.as_ptr()) }
}

pub(crate) fn atexit(f: extern "C" fn()) -> (i64, i64) {
    split(i64::from(unsafe { c::atexit(f) }))
}
