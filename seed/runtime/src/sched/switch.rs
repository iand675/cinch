//! The context switch: the one piece of the scheduler that is not Rust.
//!
//! A context is a stack pointer. Everything else a suspended computation
//! needs is on the stack that pointer names: [`switch`] pushes the
//! registers its caller expects to survive a call, stores the resulting
//! stack pointer through `save`, adopts the stack pointer `to`, pops the
//! same registers from it and returns. The return lands wherever the
//! context `to` last called `switch`, or, for a context that has never
//! run, in [`start`], which [`prepare`] arranged.
//!
//! Only the registers the platform's C calling convention makes the
//! callee responsible for are saved. `switch` is an ordinary `extern
//! "C"` call to its caller, so the compiler has already spilled every
//! other register it cares about:
//!
//! * aarch64 (AAPCS64, the same on Darwin and Linux): `x19` to `x28`,
//!   the frame pointer `x29`, the link register `x30`, the low halves
//!   `d8` to `d15` of the vector registers, and `sp`. `x18` is the
//!   platform register: reserved on Darwin, never touched here.
//! * x86_64 (System V): `rbx`, `rbp`, `r12` to `r15`, `rsp`, and the
//!   control bits of `mxcsr` and of the x87 control word, which the ABI
//!   also makes callee-saved.
//!
//! The floating-point control register of aarch64 (`fpcr`) is not
//! switched: generated code never changes it, and a foreign function
//! that did would have to restore it before returning anyway.
//!
//! These are naked functions rather than a `global_asm!` block so that
//! the symbol names follow the target's own mangling (Darwin prefixes an
//! underscore, Linux does not) with no per-platform spelling here.

/// The bytes [`switch`] pushes, and [`prepare`] lays out, on a stack.
#[cfg(target_arch = "aarch64")]
const FRAME_BYTES: usize = 160;
#[cfg(target_arch = "x86_64")]
const FRAME_BYTES: usize = 64;

/// Suspend the calling context into `*save` and resume the context `to`.
/// Returns when some context switches back to the value stored in
/// `*save`, possibly on another OS thread.
///
/// # Safety
///
/// `to` is a stack pointer a previous `switch` stored or [`prepare`]
/// returned, not resumed since; `save` is writable and stays valid until
/// the stored context is resumed. The caller holds no reference derived
/// from a thread-local across the call (the module docs of
/// [`crate::sched`]).
#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
pub(super) unsafe extern "C" fn switch(save: *mut usize, to: usize) {
    core::arch::naked_asm!(
        "sub sp, sp, #160",
        "stp x19, x20, [sp, #0]",
        "stp x21, x22, [sp, #16]",
        "stp x23, x24, [sp, #32]",
        "stp x25, x26, [sp, #48]",
        "stp x27, x28, [sp, #64]",
        "stp x29, x30, [sp, #80]",
        "stp d8, d9, [sp, #96]",
        "stp d10, d11, [sp, #112]",
        "stp d12, d13, [sp, #128]",
        "stp d14, d15, [sp, #144]",
        "mov x9, sp",
        "str x9, [x0]",
        "mov sp, x1",
        "ldp x19, x20, [sp, #0]",
        "ldp x21, x22, [sp, #16]",
        "ldp x23, x24, [sp, #32]",
        "ldp x25, x26, [sp, #48]",
        "ldp x27, x28, [sp, #64]",
        "ldp x29, x30, [sp, #80]",
        "ldp d8, d9, [sp, #96]",
        "ldp d10, d11, [sp, #112]",
        "ldp d12, d13, [sp, #128]",
        "ldp d14, d15, [sp, #144]",
        "add sp, sp, #160",
        "ret",
    )
}

/// Where a context that has never run begins: [`prepare`] left the entry
/// function in `x20` and its argument in `x19`. The frame pointer is
/// cleared so a frame walk ends here.
#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
unsafe extern "C" fn start() {
    core::arch::naked_asm!(
        "mov x0, x19",
        "mov x29, #0",
        "blr x20",
        "brk #1",
    )
}

/// See the aarch64 definition: the same contract, for System V x86_64.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
pub(super) unsafe extern "C" fn switch(save: *mut usize, to: usize) {
    core::arch::naked_asm!(
        "push rbp",
        "push rbx",
        "push r12",
        "push r13",
        "push r14",
        "push r15",
        "sub rsp, 8",
        "stmxcsr [rsp]",
        "fnstcw [rsp + 4]",
        "mov [rdi], rsp",
        "mov rsp, rsi",
        "ldmxcsr [rsp]",
        "fldcw [rsp + 4]",
        "add rsp, 8",
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "ret",
    )
}

/// Where a context that has never run begins: [`prepare`] left the entry
/// function in `r12` and its argument in `rbx`. The stack pointer is a
/// multiple of 16 here, as the ABI requires before a `call`.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "C" fn start() {
    core::arch::naked_asm!(
        "mov rdi, rbx",
        "xor ebp, ebp",
        "call r12",
        "ud2",
    )
}

/// The default `mxcsr` (every exception masked, round to nearest) and
/// x87 control word (the same, 64-bit precision), packed as [`switch`]
/// stores them.
#[cfg(target_arch = "x86_64")]
const FP_CONTROL: usize = 0x1F80 | (0x037F << 32);

/// Lay out a context on the stack whose highest address is `top` (a
/// multiple of 16) that calls `entry(arg)` when first resumed, and
/// answer its stack pointer. `entry` never returns: a context ends by
/// switching away for the last time.
///
/// # Safety
///
/// At least [`FRAME_BYTES`] plus 16 writable bytes lie below `top`.
pub(super) unsafe fn prepare(top: usize, entry: extern "C" fn(usize) -> !, arg: usize) -> usize {
    // Sixteen spare bytes above the frame keep the first real frame off
    // the mapping's last word.
    let sp = top - 16 - FRAME_BYTES;
    let frame = sp as *mut usize;
    // A recycled stack holds an earlier context's bytes: every slot the
    // first `switch` pops is written.
    for i in 0..FRAME_BYTES / 8 {
        unsafe { frame.add(i).write(0) };
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        frame.add(0).write(arg); // x19
        frame.add(1).write(entry as usize); // x20
        frame.add(11).write(start as unsafe extern "C" fn() as usize); // x30
    }
    #[cfg(target_arch = "x86_64")]
    unsafe {
        frame.add(0).write(FP_CONTROL);
        frame.add(4).write(entry as usize); // r12
        frame.add(5).write(arg); // rbx
        frame.add(7).write(start as unsafe extern "C" fn() as usize); // return address
    }
    sp
}
