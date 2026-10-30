//! Stack exhaustion as the defined `Exhausted` outcome
//! (`design/no-faults.md` sections 4 and 7).
//!
//! Every thread that runs Cinch code (the program's thread from
//! [`crate::platform::cinch_run_main`] and every task thread of
//! [`crate::task`]) runs it inside [`guarded`]. A thread's stack can only
//! run out at its low end, so `guarded` makes that end a known, reserved
//! fault zone and installs a handler that recognises a fault inside it:
//!
//! * The bottom [`GUARD_BYTES`] of the thread's usable stack are made
//!   inaccessible (`mprotect(PROT_NONE)`), on top of whatever guard the
//!   C library put below the stack. The zone the handler accepts is
//!   that band plus [`GUARD_BYTES`] below it, which covers the C
//!   library's own guard page on every page size in use (4 KiB and
//!   64 KiB on Linux, 16 KiB on Apple arm64). The band is our own rather
//!   than the C library's guard because the band's address is exact on
//!   every platform, while the C library's guard size and whether a
//!   reported stack range includes it vary.
//! * A frame can step over a guard only if it is larger than the guard
//!   and is not probed. Generated code carries `"probe-stack"="inline-asm"`
//!   on every unit (`compiler/src/codegen.rs`, `Registry::attr_group`),
//!   so a large frame touches each 4 KiB page in order; the Rust code of
//!   this runtime is probed by rustc's default on x86_64 and aarch64. The
//!   64 KiB band then only has to outlast an unprobed C-library frame.
//! * `SIGSEGV` and `SIGBUS` (a store to a `PROT_NONE` page is `SIGBUS`
//!   on Apple platforms and `SIGSEGV` on Linux) run [`on_fault`] on a
//!   per-thread alternate signal stack, since the faulting stack has no
//!   room left. A fault address inside the calling thread's zone is the
//!   `Exhausted` exit: `cinch: exhausted: stack` on stderr and exit code
//!   [`crate::abi::EXIT_EXHAUSTED`], through `_exit` (the only exit a
//!   signal handler may take, so `atexit` handlers such as the
//!   `CINCH_ALLOC_STATS` line do not run). Any other fault is not stack
//!   exhaustion: the handler reinstates the disposition it replaced and
//!   returns, so the faulting instruction faults again and the process
//!   ends exactly as it would have without this module.
//!
//! The zone is found through a thread-specific key, not a Rust
//! `thread_local!`: `pthread_getspecific` neither allocates nor takes a
//! lock on either platform, so the handler may call it on a thread that
//! never ran `guarded` (it answers null there). The zone itself lives in
//! `guarded`'s frame, which outlives every fault the body can take.
//!
//! The signal layouts (`struct sigaction`, `stack_t`, the `si_addr`
//! offset of `siginfo_t`) and constants are fixed by each platform's
//! ABI and spelled per platform below, the same way [`crate::pty`] spells
//! its own: the Linux values hold for glibc and musl on x86_64 and
//! aarch64, the others are Darwin's.

use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Once;

/// The inaccessible band at the low end of every guarded stack, and the
/// extent below it the handler also attributes to the stack.
pub const GUARD_BYTES: usize = 64 << 10;

/// The smallest stack a guarded thread is given: the band plus room for
/// the runtime's own frames above it. `CINCH_STACK_SIZE` below this is
/// raised to it.
pub const MIN_STACK_BYTES: usize = 4 * GUARD_BYTES;

/// The alternate signal stack each guarded thread gets when it has none.
/// The handler itself needs a few hundred bytes; the rest is the
/// kernel's signal frame, which on aarch64 Linux grows with the vector
/// state (SVE/SME) and on Darwin is bounded by `SIGSTKSZ` (128 KiB).
const ALT_STACK_BYTES: usize = 128 << 10;

const PROT_NONE: i32 = 0;
const PROT_READ: i32 = 1;
const PROT_WRITE: i32 = 2;
const MAP_PRIVATE: i32 = 2;
#[cfg(target_os = "linux")]
const MAP_ANON: i32 = 0x20;
#[cfg(not(target_os = "linux"))]
const MAP_ANON: i32 = 0x1000;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const SIGSEGV: i32 = 11;
#[cfg(target_os = "linux")]
const SIGBUS: i32 = 7;
#[cfg(not(target_os = "linux"))]
const SIGBUS: i32 = 10;

#[cfg(target_os = "linux")]
const SA_ONSTACK: i32 = 0x0800_0000;
#[cfg(not(target_os = "linux"))]
const SA_ONSTACK: i32 = 0x0001;
#[cfg(target_os = "linux")]
const SA_SIGINFO: i32 = 4;
#[cfg(not(target_os = "linux"))]
const SA_SIGINFO: i32 = 0x0040;
#[cfg(target_os = "linux")]
const SS_DISABLE: i32 = 2;
#[cfg(not(target_os = "linux"))]
const SS_DISABLE: i32 = 4;

/// Offset of `si_addr` in `siginfo_t`: after three `int`s, aligned to
/// 8, on Linux; after `si_signo`, `si_errno`, `si_code`, `si_pid`,
/// `si_uid` and `si_status` on Darwin.
#[cfg(target_os = "linux")]
const SI_ADDR_OFFSET: usize = 16;
#[cfg(not(target_os = "linux"))]
const SI_ADDR_OFFSET: usize = 24;

/// `struct sigaction`: the handler, a 1024-bit mask, the flags and the
/// restorer on Linux; the handler, a 32-bit mask and the flags on Darwin.
#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Clone, Copy)]
struct SigAction {
    handler: usize,
    mask: [u64; 16],
    flags: i32,
    restorer: usize,
}
#[cfg(not(target_os = "linux"))]
#[repr(C)]
#[derive(Clone, Copy)]
struct SigAction {
    handler: usize,
    mask: u32,
    flags: i32,
}

#[cfg(target_os = "linux")]
const NO_ACTION: SigAction = SigAction { handler: 0, mask: [0; 16], flags: 0, restorer: 0 };
#[cfg(not(target_os = "linux"))]
const NO_ACTION: SigAction = SigAction { handler: 0, mask: 0, flags: 0 };

/// `stack_t`: the field order differs between the two platforms.
#[cfg(target_os = "linux")]
#[repr(C)]
struct StackT {
    sp: *mut c_void,
    flags: i32,
    size: usize,
}
#[cfg(not(target_os = "linux"))]
#[repr(C)]
struct StackT {
    sp: *mut c_void,
    size: usize,
    flags: i32,
}

impl StackT {
    fn new(sp: *mut c_void, size: usize, flags: i32) -> StackT {
        StackT { sp, size, flags }
    }
}

/// `pthread_key_t`: `unsigned int` on Linux, `unsigned long` on Darwin.
#[cfg(target_os = "linux")]
type Key = u32;
#[cfg(not(target_os = "linux"))]
type Key = usize;

unsafe extern "C" {
    fn sigaction(sig: i32, act: *const SigAction, old: *mut SigAction) -> i32;
    fn sigaltstack(ss: *const StackT, old: *mut StackT) -> i32;
    fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> i32;
    fn mprotect(addr: *mut c_void, len: usize, prot: i32) -> i32;
    fn getpagesize() -> i32;
    fn pthread_self() -> usize;
    fn pthread_key_create(key: *mut Key, dtor: Option<unsafe extern "C" fn(*mut c_void)>) -> i32;
    fn pthread_getspecific(key: Key) -> *mut c_void;
    fn pthread_setspecific(key: Key, value: *const c_void) -> i32;
}

#[cfg(not(target_os = "linux"))]
unsafe extern "C" {
    fn pthread_get_stackaddr_np(thread: usize) -> *mut c_void;
    fn pthread_get_stacksize_np(thread: usize) -> usize;
}

/// Storage for a `pthread_attr_t` (56 bytes on glibc x86_64 and musl, 64
/// on glibc aarch64), read only through the calls below.
#[cfg(target_os = "linux")]
#[repr(C, align(16))]
struct Attr([u8; 128]);

#[cfg(target_os = "linux")]
unsafe extern "C" {
    fn pthread_getattr_np(thread: usize, attr: *mut Attr) -> i32;
    fn pthread_attr_getstack(attr: *const Attr, addr: *mut *mut c_void, size: *mut usize) -> i32;
    fn pthread_attr_destroy(attr: *mut Attr) -> i32;
}

/// The addresses a fault on this thread is stack exhaustion at:
/// `lo..hi`.
struct Zone {
    lo: usize,
    hi: usize,
}

/// The thread-specific key holding the current thread's `*const Zone`.
static KEY: AtomicUsize = AtomicUsize::new(0);
static INSTALL: Once = Once::new();
/// The dispositions [`on_fault`] replaced, reinstated for a fault that
/// is not stack exhaustion. Written once inside [`INSTALL`], before the
/// handler that reads them is installed.
static mut PREV_SEGV: SigAction = NO_ACTION;
static mut PREV_BUS: SigAction = NO_ACTION;

/// Create the key and install the handler for both signals, once per
/// process.
fn install() -> Key {
    INSTALL.call_once(|| {
        let mut key: Key = 0;
        if unsafe { pthread_key_create(&mut key, None) } != 0 {
            // Without a key no thread can be guarded: the system is out
            // of thread-specific slots, a thread resource.
            crate::abi::exhausted(crate::abi::Resource::Threads);
        }
        KEY.store(key as usize, Relaxed);
        let mut act = NO_ACTION;
        act.handler = on_fault as extern "C" fn(i32, *mut c_void, *mut c_void) as usize;
        act.flags = SA_SIGINFO | SA_ONSTACK;
        unsafe {
            sigaction(SIGSEGV, &act, &raw mut PREV_SEGV);
            sigaction(SIGBUS, &act, &raw mut PREV_BUS);
        }
    });
    KEY.load(Relaxed) as Key
}

/// The handler for `SIGSEGV` and `SIGBUS` (module docs).
extern "C" fn on_fault(sig: i32, info: *mut c_void, _ctx: *mut c_void) {
    let addr = unsafe { info.cast::<u8>().add(SI_ADDR_OFFSET).cast::<usize>().read() };
    let zone = unsafe { pthread_getspecific(KEY.load(Relaxed) as Key) }.cast::<Zone>();
    if !zone.is_null() {
        let zone = unsafe { &*zone };
        if (zone.lo..zone.hi).contains(&addr) {
            crate::abi::exhausted_in_signal(crate::abi::Resource::Stack);
        }
    }
    let prev = if sig == SIGBUS { &raw const PREV_BUS } else { &raw const PREV_SEGV };
    unsafe { sigaction(sig, prev, null_mut()) };
}

/// The lowest usable address of the calling thread's stack.
#[cfg(not(target_os = "linux"))]
fn stack_low() -> Option<usize> {
    let t = unsafe { pthread_self() };
    let top = unsafe { pthread_get_stackaddr_np(t) } as usize;
    let size = unsafe { pthread_get_stacksize_np(t) };
    top.checked_sub(size)
}

/// The lowest usable address of the calling thread's stack (glibc and
/// musl report the range above their guard).
#[cfg(target_os = "linux")]
fn stack_low() -> Option<usize> {
    let mut attr = Attr([0; 128]);
    if unsafe { pthread_getattr_np(pthread_self(), &mut attr) } != 0 {
        return None;
    }
    let mut addr = null_mut();
    let mut size = 0;
    let ok = unsafe { pthread_attr_getstack(&attr, &mut addr, &mut size) } == 0;
    unsafe { pthread_attr_destroy(&mut attr) };
    ok.then_some(addr as usize)
}

/// Run `body` with stack exhaustion on the calling thread turned into
/// the `Exhausted` exit (module docs). Call it at the base of a thread,
/// before any Cinch code runs on it.
pub fn guarded<R>(body: impl FnOnce() -> R) -> R {
    let key = install();
    let page = (unsafe { getpagesize() }).max(1) as usize;
    let Some(low) = stack_low() else {
        // A thread whose stack the C library cannot describe cannot be
        // guarded; this is a runtime invariant on both platforms.
        crate::abi::panic_exit(b"compiler bug: the thread's stack range is unknown");
    };
    let lo = low.next_multiple_of(page);
    // Protect the band only when it lies well below the frames already
    // live on this thread (the base of a thread, by contract).
    let here = &raw const key as usize;
    let band = here.checked_sub(lo).is_some_and(|room| room >= 2 * GUARD_BYTES)
        && unsafe { mprotect(lo as *mut c_void, GUARD_BYTES, PROT_NONE) } == 0;
    let zone = Zone {
        lo: lo.saturating_sub(GUARD_BYTES),
        hi: if band { lo + GUARD_BYTES } else { lo },
    };
    let _armed = Armed {
        key,
        band: band.then_some(lo),
        alt: alt_stack(),
    };
    unsafe { pthread_setspecific(key, (&raw const zone).cast()) };
    body()
}

/// Undoes [`guarded`]'s setup when the body returns: the key forgets the
/// zone before it goes out of scope, the alternate stack is released,
/// and the band is made writable again, so a C library that caches
/// thread stacks hands an ordinary stack to its next thread.
struct Armed {
    key: Key,
    band: Option<usize>,
    alt: Option<*mut c_void>,
}

impl Drop for Armed {
    fn drop(&mut self) {
        unsafe { pthread_setspecific(self.key, null()) };
        if let Some(sp) = self.alt {
            let off = StackT::new(null_mut(), ALT_STACK_BYTES, SS_DISABLE);
            unsafe {
                sigaltstack(&off, null_mut());
                munmap(sp, ALT_STACK_BYTES);
            }
        }
        if let Some(lo) = self.band {
            unsafe { mprotect(lo as *mut c_void, GUARD_BYTES, PROT_READ | PROT_WRITE) };
        }
    }
}

/// Give the calling thread an alternate signal stack unless it already
/// has one (Rust's own runtime installs one on its threads when it owns
/// `main`); answers the mapping this call made.
fn alt_stack() -> Option<*mut c_void> {
    let mut cur = StackT::new(null_mut(), 0, 0);
    if unsafe { sigaltstack(null(), &mut cur) } == 0 && cur.flags & SS_DISABLE == 0 {
        return None;
    }
    let sp = unsafe {
        mmap(null_mut(), ALT_STACK_BYTES, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0)
    };
    // Without a signal stack an overflow could not be reported: the
    // thread has no memory for the one place stack exhaustion is caught.
    if sp == MAP_FAILED {
        crate::abi::exhausted(crate::abi::Resource::Stack);
    }
    let ss = StackT::new(sp, ALT_STACK_BYTES, 0);
    if unsafe { sigaltstack(&ss, null_mut()) } != 0 {
        unsafe { munmap(sp, ALT_STACK_BYTES) };
        crate::abi::exhausted(crate::abi::Resource::Stack);
    }
    Some(sp)
}

/// `ENOMEM`: 12 on both platforms, fixed by the ABI.
const ENOMEM: i32 = 12;

/// The OS refused a thread for Cinch code (the program's own or a
/// task's): the `Exhausted` exit, naming the stack when the refusal was
/// for memory (the reservation itself) and threads otherwise (a thread or
/// process limit). It runs before the refused thread's code could start.
pub fn spawn_refused(e: &std::io::Error) -> ! {
    if e.raw_os_error() == Some(ENOMEM) {
        crate::abi::exhausted(crate::abi::Resource::Stack)
    }
    crate::abi::exhausted(crate::abi::Resource::Threads)
}
