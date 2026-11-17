//! Stack exhaustion as the defined `Exhausted` outcome
//! (`design/no-faults.md` sections 4 and 7).
//!
//! Every thread that runs Cinch code (the program's thread from
//! [`crate::platform::cinch_run_main`], every parallel worker of
//! [`crate::par`] and every scheduler worker of [`crate::sched`]) runs
//! it inside [`guarded`]. A thread's stack can only run out at its low
//! end, so `guarded` makes that end a known, reserved fault zone and
//! installs a handler that recognises a fault inside it:
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
//! # Fiber stacks
//!
//! A task body does not run on a thread's stack: it is a fiber of
//! [`crate::sched`] on a stack of its own, a mapping whose low
//! [`GUARD_BYTES`] are never made accessible. That band, exactly, is the
//! fiber's zone. The key is per thread and a fiber moves between
//! threads, so the scheduler hands the zone over at every switch
//! ([`swap_zone`]): the fiber's zone while the thread runs the fiber,
//! the thread's own again afterwards. The handler still runs on the
//! thread's alternate signal stack, which is why every thread the
//! scheduler runs fibers on is a guarded one.
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

use crate::sys::Key;

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

/// The protection bits of both platforms' `mprotect`.
const PROT_NONE: i32 = 0;
pub(crate) const PROT_READ: i32 = 1;
pub(crate) const PROT_WRITE: i32 = 2;

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

// The calls themselves are [`crate::sys`]'s: `sigaction` and
// `sigaltstack` take this module's `SigAction` and `StackT` as opaque
// pointers, and the thread, mapping and page-size calls are its own
// functions, so this module names no foreign symbol. The two struct
// layouts stay here because they are data, not entry points, and they
// move into `sys/` when the stack trap becomes its own seam
// (design/base-as-a-package.md section 5.1).

/// The addresses a fault on this thread is stack exhaustion at:
/// `lo..hi`. An empty range (`lo == hi`) is a zone no address is in.
pub(crate) struct Zone {
    pub(crate) lo: usize,
    pub(crate) hi: usize,
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
        let Some(key) = crate::sys::thread_key_create() else {
            // Without a key no thread can be guarded: the system is out
            // of thread-specific slots, a thread resource.
            crate::abi::exhausted(crate::abi::Resource::Threads);
        };
        KEY.store(key as usize, Relaxed);
        let mut act = NO_ACTION;
        act.handler = on_fault as extern "C" fn(i32, *mut c_void, *mut c_void) as usize;
        act.flags = SA_SIGINFO | SA_ONSTACK;
        unsafe {
            crate::sys::sigaction(
                SIGSEGV,
                (&act as *const SigAction).cast(),
                (&raw mut PREV_SEGV).cast(),
            );
            crate::sys::sigaction(
                SIGBUS,
                (&act as *const SigAction).cast(),
                (&raw mut PREV_BUS).cast(),
            );
        }
    });
    KEY.load(Relaxed) as Key
}

/// The handler for `SIGSEGV` and `SIGBUS` (module docs).
extern "C" fn on_fault(sig: i32, info: *mut c_void, _ctx: *mut c_void) {
    let addr = unsafe { info.cast::<u8>().add(SI_ADDR_OFFSET).cast::<usize>().read() };
    let zone = unsafe { crate::sys::pthread_getspecific(KEY.load(Relaxed) as Key) }.cast::<Zone>();
    if !zone.is_null() {
        let zone = unsafe { &*zone };
        if (zone.lo..zone.hi).contains(&addr) {
            crate::abi::exhausted_in_signal(crate::abi::Resource::Stack);
        }
    }
    let prev = if sig == SIGBUS { &raw const PREV_BUS } else { &raw const PREV_SEGV };
    unsafe { crate::sys::sigaction(sig, prev.cast(), null_mut()) };
}

/// The lowest usable address of the calling thread's stack (both
/// platforms answer through their own call in [`crate::sys`]; glibc and
/// musl report the range above their guard, and Darwin's
/// `pthread_get_stackaddr_np` answers the high end).
fn stack_low() -> Option<usize> {
    crate::sys::this_thread_stack().map(|(low, _)| low)
}

/// Run `body` with stack exhaustion on the calling thread turned into
/// the `Exhausted` exit (module docs). Call it at the base of a thread,
/// before any Cinch code runs on it.
pub fn guarded<R>(body: impl FnOnce() -> R) -> R {
    let key = install();
    let page = crate::sys::getpagesize();
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
        && crate::sys::mprotect(lo as i64, GUARD_BYTES, PROT_NONE).0 == 0;
    let zone = Zone {
        lo: lo.saturating_sub(GUARD_BYTES),
        hi: if band { lo + GUARD_BYTES } else { lo },
    };
    let _armed = Armed {
        band: band.then_some(lo),
        alt: alt_stack(),
    };
    unsafe { swap_zone(&raw const zone) };
    body()
}

/// Undoes [`guarded`]'s setup when the body returns: the key forgets the
/// zone before it goes out of scope, the alternate stack is released,
/// and the band is made writable again, so a C library that caches
/// thread stacks hands an ordinary stack to its next thread.
struct Armed {
    band: Option<usize>,
    alt: Option<*mut c_void>,
}

impl Drop for Armed {
    fn drop(&mut self) {
        unsafe { swap_zone(null()) };
        if let Some(sp) = self.alt {
            let off = StackT::new(null_mut(), ALT_STACK_BYTES, SS_DISABLE);
            unsafe {
                crate::sys::sigaltstack((&off as *const StackT).cast(), null_mut());
            }
            crate::sys::munmap(sp as i64, ALT_STACK_BYTES);
        }
        if let Some(lo) = self.band {
            crate::sys::mprotect(lo as i64, GUARD_BYTES, PROT_READ | PROT_WRITE);
        }
    }
}

/// Give the calling thread an alternate signal stack unless it already
/// has one (Rust's own runtime installs one on its threads when it owns
/// `main`); answers the mapping this call made.
fn alt_stack() -> Option<*mut c_void> {
    let mut cur = StackT::new(null_mut(), 0, 0);
    if unsafe { crate::sys::sigaltstack(null(), (&mut cur as *mut StackT).cast()) }.0 == 0
        && cur.flags & SS_DISABLE == 0
    {
        return None;
    }
    let (sp, _) = crate::sys::mmap_anon(ALT_STACK_BYTES);
    // Without a signal stack an overflow could not be reported: the
    // thread has no memory for the one place stack exhaustion is caught.
    if sp < 0 {
        crate::abi::exhausted(crate::abi::Resource::Stack);
    }
    let sp = sp as *mut c_void;
    let ss = StackT::new(sp, ALT_STACK_BYTES, 0);
    if unsafe { crate::sys::sigaltstack((&ss as *const StackT).cast(), null_mut()) }.0 != 0 {
        crate::sys::munmap(sp as i64, ALT_STACK_BYTES);
        crate::abi::exhausted(crate::abi::Resource::Stack);
    }
    Some(sp)
}

/// Make `zone` the calling thread's fault zone and answer the one it
/// replaces (null for a thread that had none). [`guarded`] installs a
/// thread's own zone with it, and the scheduler hands a fiber's zone
/// over with it at every switch (module docs, "Fiber stacks").
///
/// # Safety
///
/// `zone` is null or stays valid until another call replaces it on this
/// thread.
pub(crate) unsafe fn swap_zone(zone: *const Zone) -> *const Zone {
    let key = install();
    unsafe {
        let previous = crate::sys::pthread_getspecific(key).cast::<Zone>().cast_const();
        crate::sys::pthread_setspecific(key, zone.cast());
        previous
    }
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
