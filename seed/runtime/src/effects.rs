//! Cross-call algebraic effect evidence stack and cooperative bubbling state.
//!
//! Each `handle` expression for a tail-resumptive and/or abortive effect pushes
//! a stack-allocated [`EvEntry`] onto the current fiber's linked evidence chain
//! (`ev_head`) via [`cinch_ev_push`] and pops it on exit via [`cinch_ev_pop`].
//!
//! * Tail-resumptive operations (`abort_mask` bit 0) look up the innermost
//!   matching [`EvEntry`] by canonical `label_hash`, invoke the handler's
//!   clause directly with an immortal identity continuation, and return the
//!   resumed value without unwinding.
//! * Abortive operations (`abort_mask` bit 1) record the target `marker`,
//!   operation `slot`, and owned payload `arg` in [`EffectState`] via
//!   [`cinch_ev_abort`], setting `yielding = 1`. Every intermediate effectful
//!   frame checks [`cinch_ev_is_yielding`], runs its exact Perceus `Drop`s, and
//!   returns cooperatively until the matching `handle` catches `marker` via
//!   [`cinch_ev_is_caught`].
//!
//! [`EffectState`] is fiber-local: `sched/fiber.rs` exchanges it on every fiber
//! switch alongside `CURRENT_TASK`, `arena::CURRENT`, and `eval::TLS`, so
//! handlers migrate safely across OS worker threads and work unchanged on
//! `wasm32`.

use std::cell::Cell;

/// Stack-allocated handler evidence node linked into [`EffectState::ev_head`].
///
/// Generated code reserves 6 `i64` words (`48` bytes) on the stack frame with
/// `alloca [6 x i64]` and passes the pointer to [`cinch_ev_push`] /
/// [`cinch_ev_pop`]. Zero heap allocation per `handle`.
#[repr(C)]
pub struct EvEntry {
    pub prev: *mut EvEntry,
    pub label_hash: u64,
    pub marker: u64,
    pub vtable: i64,
    /// Bitmask of abortive operation slots (`bit s == 1` means operation slot `s`
    /// never calls `resume k` and aborts cooperatively to this prompt).
    pub abort_mask: u64,
    pub _reserved: u64,
}

const _: () = assert!(std::mem::size_of::<EvEntry>() == 48);

/// Per-fiber algebraic effect evidence and cooperative bubbling state.
#[derive(Clone, Copy)]
pub struct EffectState {
    pub ev_head: *mut EvEntry,
    pub yielding: u8,
    pub yield_marker: u64,
    pub yield_slot: u64,
    pub yield_arg: i64,
    pub next_marker: u64,
}

impl EffectState {
    pub const IDLE: EffectState = EffectState {
        ev_head: std::ptr::null_mut(),
        yielding: 0,
        yield_marker: 0,
        yield_slot: 0,
        yield_arg: 0,
        next_marker: 1,
    };
}

thread_local! {
    static EFFECTS: Cell<EffectState> = const { Cell::new(EffectState::IDLE) };
}

/// Exchange the calling thread's [`EffectState`] with `next`, returning the
/// previous state. Marked `#[inline(never)]` per the scheduler's fiber
/// migration rule (`sched.rs`).
#[inline(never)]
pub(crate) fn swap_state(next: EffectState) -> EffectState {
    EFFECTS.with(|c| c.replace(next))
}

/// Snapshot the current evidence head and yielding flag for arena $O(1)$
/// escape prompts (`escape.rs`).
#[inline(never)]
pub(crate) fn snapshot_for_escape() -> (*mut EvEntry, u8) {
    EFFECTS.with(|c| {
        let s = c.get();
        (s.ev_head, s.yielding)
    })
}

/// Restore the evidence head and yielding flag on an $O(1)$ arena escape jump.
#[inline(never)]
pub(crate) fn restore_after_escape(ev_head: *mut EvEntry, yielding: u8) {
    EFFECTS.with(|c| {
        let mut s = c.get();
        s.ev_head = ev_head;
        s.yielding = yielding;
        if yielding == 0 {
            s.yield_marker = 0;
            s.yield_slot = 0;
            s.yield_arg = 0;
        }
        c.set(s);
    });
}

/// Push a stack-allocated [`EvEntry`] onto the fiber's evidence stack and
/// return its unique nonzero `marker`.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_push(
    entry: *mut EvEntry,
    label_hash: i64,
    vtable: i64,
    abort_mask: i64,
) -> i64 {
    EFFECTS.with(|c| {
        let mut s = c.get();
        let marker = s.next_marker;
        s.next_marker = marker.wrapping_add(1).max(1);
        unsafe {
            entry.write(EvEntry {
                prev: s.ev_head,
                label_hash: label_hash as u64,
                marker,
                vtable,
                abort_mask: abort_mask as u64,
                _reserved: 0,
            });
        }
        s.ev_head = entry;
        c.set(s);
        marker as i64
    })
}

/// Pop `entry` from the top of the fiber's evidence stack.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_pop(entry: *mut EvEntry) -> i64 {
    EFFECTS.with(|c| {
        let mut s = c.get();
        if s.ev_head != entry {
            crate::abi::panic_exit(b"compiler bug: effect evidence frame popped out of order");
        }
        s.ev_head = unsafe { (*entry).prev };
        c.set(s);
    });
    0
}

/// Look up the innermost [`EvEntry`] matching `label_hash`.
///
/// Well-typed programs always have an enclosing handler on the evidence stack;
/// reaching the end without a match is a compiler bug trap.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_lookup(label_hash: i64) -> *mut EvEntry {
    let want = label_hash as u64;
    EFFECTS.with(|c| {
        let mut cur = c.get().ev_head;
        while !cur.is_null() {
            let entry = unsafe { &*cur };
            if entry.label_hash == want {
                return cur;
            }
            cur = entry.prev;
        }
        crate::abi::panic_exit(b"compiler bug: unhandled effect operation reached evidence lookup");
    })
}

/// Read the handler vtable word from `entry`.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_entry_vtable(entry: *mut EvEntry) -> i64 {
    unsafe { (*entry).vtable }
}

/// Read slot `slot` (0 = return, 1.. = operations) from `entry`'s handler
/// tuple block and retain (+1 RC) the closure word for the caller.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_vtable_slot(entry: *mut EvEntry, slot: i64) -> i64 {
    let vtable = unsafe { (*entry).vtable };
    if vtable == 0 || slot < 0 {
        crate::abi::panic_exit(b"compiler bug: invalid handler vtable slot lookup");
    }
    let clo = unsafe { (vtable as usize as *const i64).add(slot as usize).read() };
    unsafe { crate::rc::cinch_dup(clo) };
    clo
}

/// Read the unique prompt marker from `entry`.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_entry_marker(entry: *mut EvEntry) -> i64 {
    unsafe { (*entry).marker as i64 }
}

/// Return `1` if operation `slot` on `entry` is abortive, `0` if tail-resumptive.
#[inline(never)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_entry_slot_aborts(entry: *mut EvEntry, slot: i64) -> i64 {
    let mask = unsafe { (*entry).abort_mask };
    if slot >= 0 && (slot as u32) < 64 && ((mask >> (slot as u32)) & 1) != 0 {
        1
    } else {
        0
    }
}

/// Begin cooperative abort bubbling toward `marker` with operation `slot` and
/// owned payload word `arg`. Returns `0` (ignored by callers when yielding).
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_abort(marker: i64, slot: i64, arg: i64) -> i64 {
    EFFECTS.with(|c| {
        let mut s = c.get();
        s.yielding = 1;
        s.yield_marker = marker as u64;
        s.yield_slot = slot as u64;
        s.yield_arg = arg;
        c.set(s);
    });
    0
}

/// Return `1` when an abort is currently bubbling up the call stack, else `0`.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_is_yielding() -> i64 {
    EFFECTS.with(|c| c.get().yielding as i64)
}

/// Return `1` iff an abort is bubbling specifically to `marker`, else `0`.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_is_caught(marker: i64) -> i64 {
    EFFECTS.with(|c| {
        let s = c.get();
        i64::from(s.yielding != 0 && s.yield_marker == marker as u64)
    })
}

/// Read the caught abort's operation slot (1-based).
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_yield_slot() -> i64 {
    EFFECTS.with(|c| c.get().yield_slot as i64)
}

/// Consume the caught abort: clears `yielding` and returns the owned payload
/// word transferred from the performing call site.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_take_yield_arg() -> i64 {
    EFFECTS.with(|c| {
        let mut s = c.get();
        let arg = s.yield_arg;
        s.yielding = 0;
        s.yield_marker = 0;
        s.yield_slot = 0;
        s.yield_arg = 0;
        c.set(s);
        arg
    })
}

/// Immortal identity continuation stub used by cross-call tail-resumptive and
/// abortive handlers. Follows Cinch's closure apply-stub ABI:
/// `extern "C" fn(self_ptr: i64, nargs: i64, args: *const i64) -> i64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_ev_id_apply(_self: i64, nargs: i64, args: *const i64) -> i64 {
    if nargs < 1 || args.is_null() {
        return 0;
    }
    unsafe { *args }
}

/// Layout of an immortal 1-arg static closure block for [`cinch_ev_id_apply`]:
/// header word (`RC_IMMORTAL`, `KIND_CLOSURE`, layout `0`) followed by
/// `[stub_ptr, arity=1, nenv=0]`.
#[repr(C, align(8))]
struct IdClosureBlock {
    header: u64,
    stub: usize,
    arity: i64,
    nenv: i64,
}

static mut ID_CLOSURE: IdClosureBlock = IdClosureBlock {
    header: crate::block::header(crate::block::RC_IMMORTAL, crate::block::KIND_CLOSURE, 0),
    stub: 0,
    arity: 1,
    nenv: 0,
};

/// Return the immortal 1-argument identity closure word `(\x -> x)`.
/// Safe for `cinch_dup`/`cinch_drop` (immortal RC = 0) and `__apply`.
#[inline(never)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_ev_id_closure() -> i64 {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        ID_CLOSURE.stub = cinch_ev_id_apply as *const () as usize;
    });
    let payload = unsafe { (&raw const ID_CLOSURE.stub).cast::<u8>() };
    payload as usize as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_push_lookup_abort_catch_and_pop() {
        let mut e1 = std::mem::MaybeUninit::<EvEntry>::uninit();
        let mut e2 = std::mem::MaybeUninit::<EvEntry>::uninit();
        let m1 = unsafe { cinch_ev_push(e1.as_mut_ptr(), 111, 0x1000, 0b10) };
        let m2 = unsafe { cinch_ev_push(e2.as_mut_ptr(), 222, 0x2000, 0) };
        assert_ne!(m1, m2);
        let found1 = cinch_ev_lookup(111);
        let found2 = cinch_ev_lookup(222);
        assert_eq!(unsafe { cinch_ev_entry_vtable(found1) }, 0x1000);
        assert_eq!(unsafe { cinch_ev_entry_slot_aborts(found1, 1) }, 1);
        assert_eq!(unsafe { cinch_ev_entry_vtable(found2) }, 0x2000);
        assert_eq!(unsafe { cinch_ev_entry_slot_aborts(found2, 1) }, 0);
        assert_eq!(cinch_ev_is_yielding(), 0);
        cinch_ev_abort(m1, 1, 42);
        assert_eq!(cinch_ev_is_yielding(), 1);
        assert_eq!(cinch_ev_is_caught(m2), 0);
        assert_eq!(cinch_ev_is_caught(m1), 1);
        assert_eq!(cinch_ev_yield_slot(), 1);
        assert_eq!(cinch_ev_take_yield_arg(), 42);
        assert_eq!(cinch_ev_is_yielding(), 0);
        unsafe { cinch_ev_pop(e2.as_mut_ptr()) };
        unsafe { cinch_ev_pop(e1.as_mut_ptr()) };
        assert_eq!(snapshot_for_escape().0, std::ptr::null_mut());
    }

    #[test]
    fn id_closure_is_immortal_and_returns_first_arg() {
        let clo = cinch_ev_id_closure();
        assert_ne!(clo, 0);
        let h = unsafe { crate::block::read_header(clo as usize as *const u8) };
        assert_eq!(crate::block::rc(h), crate::block::RC_IMMORTAL);
        let args = [99i64];
        let stub_ptr = unsafe { *(clo as usize as *const usize) };
        let f: unsafe extern "C" fn(i64, i64, *const i64) -> i64 = unsafe { std::mem::transmute(stub_ptr) };
        assert_eq!(unsafe { f(clo, 1, args.as_ptr()) }, 99);
    }
}
