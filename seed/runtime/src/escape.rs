//! Arena-delimited $O(1)$ prompt escape (`Effect.Arena`).
//!
//! [`cinch_arena_escape_run`] enters a fresh bump arena (with an optional byte
//! budget), saves a lightweight [`EscapeFrame`] on the current stack, and runs
//! a pure region-scoped computation. Inside the extent:
//!
//! * Normal completion leaves the arena and invokes `on_ok(result)`.
//! * [`cinch_arena_escape_abort`] (or exceeding the arena's byte budget inside
//!   [`crate::arena`]) jumps straight back to [`cinch_escape_call`] in $O(1)$
//!   via naked assembly on `x86_64` (System V) and `aarch64` (AAPCS64), tears
//!   down the arena (freeing all chunks, `served` heap children, and any
//!   `escape_owned` heap allocations skipped by the jump), restores the
//!   fiber's effect evidence stack, and invokes `on_err(payload)` or
//!   `on_exhausted(0)`.
//!
//! On `wasm32` or other architectures without naked-asm support, an arena
//! escape falls back cleanly to cooperative unwinding via [`crate::effects`].
//!
//! Safety relies on static compiler invariants checked in both `seed/` and
//! `compiler/`: return and error types are `Copy`/scalar (no arena pointer
//! escapes region `r`), no `%1` linear binder is captured across the jump, and
//! no `Async`/`Spawn` runs inside the jump extent.

use crate::arena::{self, Arena};
use crate::effects::{self, EvEntry};

pub(crate) const STATUS_OK: i64 = 0;
pub(crate) const STATUS_ABORT: i64 = 1;
pub(crate) const STATUS_EXHAUSTED: i64 = 2;

type ApplyStub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

#[inline]
unsafe fn apply1(clo: i64, arg: i64) -> i64 {
    let code = unsafe { *(clo as usize as *const i64) };
    let stub: ApplyStub = unsafe { std::mem::transmute(code as usize) };
    let mut argv = [arg];
    unsafe { stub(clo, 1, argv.as_mut_ptr()) }
}

/// Saved prompt state for $O(1)$ stack escape.
///
/// Field offsets are pinned for the naked assembly in [`cinch_escape_call`]
/// and [`cinch_escape_jump`] and verified by compile-time assertions below.
#[cfg(target_arch = "x86_64")]
#[repr(C, align(16))]
pub struct EscapeFrame {
    pub rbx: u64,        // +0
    pub rbp: u64,        // +8
    pub r12: u64,        // +16
    pub r13: u64,        // +24
    pub r14: u64,        // +32
    pub r15: u64,        // +40
    pub rsp: u64,        // +48
    pub rip: u64,        // +56
    pub mxcsr: u32,      // +64
    pub fcw: u32,        // +68
    pub status: i64,     // +72
    pub payload: i64,    // +80
    pub arena: *mut Arena,
    pub saved_ev_head: *mut EvEntry,
    pub saved_yielding: u8,
}

#[cfg(target_arch = "x86_64")]
const _: () = {
    assert!(core::mem::offset_of!(EscapeFrame, rbx) == 0);
    assert!(core::mem::offset_of!(EscapeFrame, rbp) == 8);
    assert!(core::mem::offset_of!(EscapeFrame, r12) == 16);
    assert!(core::mem::offset_of!(EscapeFrame, r13) == 24);
    assert!(core::mem::offset_of!(EscapeFrame, r14) == 32);
    assert!(core::mem::offset_of!(EscapeFrame, r15) == 40);
    assert!(core::mem::offset_of!(EscapeFrame, rsp) == 48);
    assert!(core::mem::offset_of!(EscapeFrame, rip) == 56);
    assert!(core::mem::offset_of!(EscapeFrame, mxcsr) == 64);
    assert!(core::mem::offset_of!(EscapeFrame, fcw) == 68);
    assert!(core::mem::offset_of!(EscapeFrame, status) == 72);
    assert!(core::mem::offset_of!(EscapeFrame, payload) == 80);
};

#[cfg(target_arch = "aarch64")]
#[repr(C, align(16))]
pub struct EscapeFrame {
    pub x19_x28: [u64; 10], // +0..80
    pub x29: u64,           // +80
    pub x30: u64,           // +88
    pub sp: u64,            // +96
    pub d8_d15: [u64; 8],   // +104..168
    pub status: i64,        // +168
    pub payload: i64,       // +176
    pub arena: *mut Arena,
    pub saved_ev_head: *mut EvEntry,
    pub saved_yielding: u8,
}

#[cfg(target_arch = "aarch64")]
const _: () = {
    assert!(core::mem::offset_of!(EscapeFrame, x29) == 80);
    assert!(core::mem::offset_of!(EscapeFrame, x30) == 88);
    assert!(core::mem::offset_of!(EscapeFrame, sp) == 96);
    assert!(core::mem::offset_of!(EscapeFrame, d8_d15) == 104);
    assert!(core::mem::offset_of!(EscapeFrame, status) == 168);
    assert!(core::mem::offset_of!(EscapeFrame, payload) == 176);
};

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[repr(C)]
pub struct EscapeFrame {
    pub status: i64,
    pub payload: i64,
    pub arena: *mut Arena,
    pub saved_ev_head: *mut EvEntry,
    pub saved_yielding: u8,
}

impl EscapeFrame {
    fn zeroed() -> Self {
        // SAFETY: all fields are integers or raw pointers where 0 is valid.
        unsafe { core::mem::zeroed() }
    }
}

/// Save callee-saved state into `frame`, invoke `body(frame, ctx)`, and return
/// `STATUS_OK` on normal return or the jump `status` on [`cinch_escape_jump`].
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
pub unsafe extern "C" fn cinch_escape_call(
    frame: *mut EscapeFrame,
    body: unsafe extern "C" fn(*mut EscapeFrame, i64) -> i64,
    ctx: i64,
) -> i64 {
    core::arch::naked_asm!(
        "mov [rdi + 0], rbx",
        "mov [rdi + 8], rbp",
        "mov [rdi + 16], r12",
        "mov [rdi + 24], r13",
        "mov [rdi + 32], r14",
        "mov [rdi + 40], r15",
        "mov [rdi + 48], rsp",
        "mov rax, [rsp]",
        "mov [rdi + 56], rax",
        "stmxcsr [rdi + 64]",
        "fnstcw [rdi + 68]",
        "sub rsp, 8",
        "mov rbx, rdi",
        "mov rax, rsi",
        "mov rsi, rdx",
        "call rax",
        "mov qword ptr [rbx + 72], 0",
        "mov [rbx + 80], rax",
        "xor eax, eax",
        "add rsp, 8",
        "mov rbx, [rbx + 0]",
        "ret",
    )
}

/// Restore callee-saved state from `frame` and return `status` directly to
/// [`cinch_escape_call`]'s caller in $O(1)$.
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
pub unsafe extern "C" fn cinch_escape_jump(
    frame: *mut EscapeFrame,
    status: i64,
    payload: i64,
) -> ! {
    core::arch::naked_asm!(
        "mov [rdi + 72], rsi",
        "mov [rdi + 80], rdx",
        "mov rbx, [rdi + 0]",
        "mov rbp, [rdi + 8]",
        "mov r12, [rdi + 16]",
        "mov r13, [rdi + 24]",
        "mov r14, [rdi + 32]",
        "mov r15, [rdi + 40]",
        "ldmxcsr [rdi + 64]",
        "fldcw [rdi + 68]",
        "mov rsp, [rdi + 48]",
        "mov rcx, [rdi + 56]",
        "add rsp, 8",
        "mov rax, rsi",
        "jmp rcx",
    )
}

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
pub unsafe extern "C" fn cinch_escape_call(
    frame: *mut EscapeFrame,
    body: unsafe extern "C" fn(*mut EscapeFrame, i64) -> i64,
    ctx: i64,
) -> i64 {
    core::arch::naked_asm!(
        "stp x19, x20, [x0, #0]",
        "stp x21, x22, [x0, #16]",
        "stp x23, x24, [x0, #32]",
        "stp x25, x26, [x0, #48]",
        "stp x27, x28, [x0, #64]",
        "stp x29, x30, [x0, #80]",
        "mov x9, sp",
        "str x9, [x0, #96]",
        "stp d8, d9, [x0, #104]",
        "stp d10, d11, [x0, #120]",
        "stp d12, d13, [x0, #136]",
        "stp d14, d15, [x0, #152]",
        "mov x19, x0",
        "mov x9, x1",
        "mov x1, x2",
        "blr x9",
        "str xzr, [x19, #168]",
        "str x0, [x19, #176]",
        "mov x0, #0",
        "ldp x29, x30, [x19, #80]",
        "ldr x19, [x19, #0]",
        "ret",
    )
}

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
pub unsafe extern "C" fn cinch_escape_jump(
    frame: *mut EscapeFrame,
    status: i64,
    payload: i64,
) -> ! {
    core::arch::naked_asm!(
        "str x1, [x0, #168]",
        "str x2, [x0, #176]",
        "ldp x19, x20, [x0, #0]",
        "ldp x21, x22, [x0, #16]",
        "ldp x23, x24, [x0, #32]",
        "ldp x25, x26, [x0, #48]",
        "ldp x27, x28, [x0, #64]",
        "ldp x29, x30, [x0, #80]",
        "ldr x9, [x0, #96]",
        "mov sp, x9",
        "ldp d8, d9, [x0, #104]",
        "ldp d10, d11, [x0, #120]",
        "ldp d12, d13, [x0, #136]",
        "ldp d14, d15, [x0, #152]",
        "mov x0, x1",
        "ret",
    )
}

/// Cooperative fallback for targets without naked-asm (e.g. `wasm32`).
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub unsafe extern "C" fn cinch_escape_call(
    frame: *mut EscapeFrame,
    body: unsafe extern "C" fn(*mut EscapeFrame, i64) -> i64,
    ctx: i64,
) -> i64 {
    let r = unsafe { body(frame, ctx) };
    let status = unsafe { (*frame).status };
    if status == STATUS_OK {
        unsafe { (*frame).payload = r };
    }
    status
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub unsafe extern "C" fn cinch_escape_jump(
    frame: *mut EscapeFrame,
    status: i64,
    payload: i64,
) -> ! {
    unsafe {
        (*frame).status = status;
        (*frame).payload = payload;
    }
    crate::abi::panic_exit(b"arena escape jump is unavailable on this target without cooperative check")
}

/// Trigger an $O(1)$ budget-exhaustion escape to `frame` from inside
/// [`crate::arena`].
pub(crate) unsafe fn jump_budget_exhausted(frame: *mut EscapeFrame) -> ! {
    unsafe { cinch_escape_jump(frame, STATUS_EXHAUSTED, 0) }
}

unsafe extern "C" fn run_body_trampoline(frame: *mut EscapeFrame, body_clo: i64) -> i64 {
    let handle = frame as usize as i64;
    unsafe { apply1(body_clo, handle) }
}

/// Enter an escape-capable bump arena (with optional byte cap `budget`; `0` =
/// unlimited), invoke `body(token_handle)`, and dispatch the outcome through
/// `on_ok(result)`, `on_err(err_word)`, or `on_exhausted(0)`.
///
/// All four closure arguments are borrowed (the caller releases them after
/// `cinch_arena_escape_run` returns).
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_arena_escape_run(
    budget: i64,
    body: i64,
    on_ok: i64,
    on_err: i64,
    on_exhausted: i64,
) -> i64 {
    if budget < 0 {
        crate::abi::panic_exit(b"compiler bug: cinch_arena_escape_run called with a negative budget");
    }
    let mut frame = EscapeFrame::zeroed();
    let (ev_head, yielding) = effects::snapshot_for_escape();
    frame.saved_ev_head = ev_head;
    frame.saved_yielding = yielding;
    let initial = if budget > 0 { (budget as usize).min(64 << 10) } else { 64 << 10 };
    let arena_ptr = unsafe { arena::new_escape(initial, budget as usize, &raw mut frame) };
    frame.arena = arena_ptr;

    let status = unsafe { cinch_escape_call(&raw mut frame, run_body_trampoline, body) };
    let payload = frame.payload;

    if status == STATUS_OK {
        unsafe { arena::leave_escape_to(arena_ptr, false) };
        unsafe { apply1(on_ok, payload) }
    } else {
        effects::restore_after_escape(frame.saved_ev_head, frame.saved_yielding);
        unsafe { arena::leave_escape_to(arena_ptr, true) };
        if status == STATUS_EXHAUSTED {
            unsafe { apply1(on_exhausted, 0) }
        } else {
            unsafe { apply1(on_err, payload) }
        }
    }
}

/// Escape in $O(1)$ from an active arena prompt `handle` with error word
/// `err_word`. Never returns on `x86_64` / `aarch64`.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_arena_escape_abort(handle: i64, err_word: i64) -> i64 {
    if handle == 0 {
        crate::abi::panic_exit(b"compiler bug: cinch_arena_escape_abort called with null handle");
    }
    let frame = handle as usize as *mut EscapeFrame;
    unsafe { cinch_escape_jump(frame, STATUS_ABORT, err_word) }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "C" fn deep_abort(frame: *mut EscapeFrame, depth: i64) -> i64 {
        if depth <= 0 {
            unsafe { cinch_escape_jump(frame, STATUS_ABORT, 42) }
        }
        // Allocate a stack array to ensure callee-saved registers & frame
        // pointers are non-trivial across recursion.
        let mut buf = [depth; 16];
        let r = unsafe { deep_abort(frame, depth - 1) };
        buf[0].wrapping_add(r)
    }

    unsafe extern "C" fn normal_return(_frame: *mut EscapeFrame, x: i64) -> i64 {
        x * 2
    }

    #[test]
    fn escape_call_handles_normal_and_deep_abort() {
        let mut f1 = EscapeFrame::zeroed();
        let s1 = unsafe { cinch_escape_call(&raw mut f1, normal_return, 21) };
        assert_eq!(s1, STATUS_OK);
        assert_eq!(f1.payload, 42);

        let mut f2 = EscapeFrame::zeroed();
        let s2 = unsafe { cinch_escape_call(&raw mut f2, deep_abort, 200) };
        assert_eq!(s2, STATUS_ABORT);
        assert_eq!(f2.payload, 42);
    }

    unsafe extern "C" fn blow_budget(frame: *mut EscapeFrame, _ctx: i64) -> i64 {
        use crate::block::test_shapes::{field_of, CONS_T};
        use crate::block::allocate_ctor;
        let desc = CONS_T;
        let mut acc: i64 = 0;
        for i in 0..10_000i64 {
            let p = unsafe { allocate_ctor(16, field_of(desc), false) }.cast::<i64>();
            unsafe {
                p.write((i << 1) | 1);
                p.add(1).write(0);
            }
            acc = acc.wrapping_add(p as usize as i64);
        }
        let _ = frame;
        acc
    }

    #[test]
    fn budgeted_arena_escapes_cleanly_on_exhaustion_with_zero_leaks() {
        let _heavy = crate::heap::TEST_HEAVY.lock().unwrap_or_else(|e| e.into_inner());
        crate::block::install_test_layouts();
        let (_, _, chunks0, chunk_bytes0) = arena::stats();
        let mut frame = EscapeFrame::zeroed();
        let arena_ptr = unsafe { arena::new_escape(4096, 4096, &raw mut frame) };
        frame.arena = arena_ptr;
        let status = unsafe { cinch_escape_call(&raw mut frame, blow_budget, 0) };
        assert_eq!(status, STATUS_EXHAUSTED);
        unsafe { arena::leave_escape_to(arena_ptr, true) };
        let (_, _, chunks1, chunk_bytes1) = arena::stats();
        assert_eq!(chunks1, chunks0);
        assert_eq!(chunk_bytes1, chunk_bytes0);
    }
}
