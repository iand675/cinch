//! Native single-place variables. Each box owns its mutex and FIFO queues;
//! a suspended call holds the box alive and parks through `crate::park`.
//!
//! Values are marked shared *before* entering the box, even if the box is
//! currently private. Consequently the generic mark walk need not inspect
//! its changing Rust-owned fields. The box's Rust destructor releases the
//! slot and queued values; the generic block scan sees no counted slots.
//! Reads duplicate under the mutex, takes transfer ownership, and queued
//! handoffs publish their result before waking the recipient. No lock is
//! held across a park. Cancellation and handoff serialize under that same
//! mutex, with an already completed handoff taking precedence.

#[cfg(not(target_family = "wasm"))]
use crate::park;
use crate::{abi, block, rc};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard};

// ABI with compiler/src/mvar_codegen.rs.
const TAKE: i64 = 0;
const PUT: i64 = 1;
const READ: i64 = 2;
const SWAP: i64 = 3;
const IS_EMPTY: i64 = 4;
const BLOCK: i64 = 0;
const TRY: i64 = 1;
const CANCEL: i64 = 2;

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// One owned ABI word. A separate Option, never a sentinel word, records
/// emptiness: zero is a valid raw scalar (and Unit) payload.
struct Value {
    word: i64,
    format: i64,
}

impl Value {
    unsafe fn owned(word: i64, format: i64) -> Self {
        if format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_mark_mt(word) };
        }
        Self { word, format }
    }

    fn duplicate(&self) -> Self {
        if self.format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_dup(self.word) };
        }
        Self {
            word: self.word,
            format: self.format,
        }
    }

    fn into_word(self) -> i64 {
        let word = self.word;
        std::mem::forget(self);
        word
    }

    fn unit() -> Self {
        Self {
            word: 0,
            format: block::SCAN_RAW as i64,
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_drop(self.word) };
        }
    }
}

struct Waiter {
    #[cfg(not(target_family = "wasm"))]
    parker: park::Parker,
    result: Mutex<Option<Value>>,
}

impl Waiter {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            #[cfg(not(target_family = "wasm"))]
            parker: park::current(),
            result: Mutex::new(None),
        })
    }

    fn complete(&self, value: Value) {
        *lock(&self.result) = Some(value);
        #[cfg(not(target_family = "wasm"))]
        self.parker.unpark();
    }
}

#[derive(Default)]
struct State {
    value: Option<Value>,
    takers: VecDeque<(Arc<Waiter>, Option<Value>)>,
    readers: VecDeque<Arc<Waiter>>,
    writers: VecDeque<(Arc<Waiter>, Value)>,
}

impl State {
    // Invariant: full => no queued takers/readers; empty => no writers.
    // Swaps join the taker queue and install their replacement atomically.
    fn fill(&mut self, mut value: Value) {
        for r in self.readers.drain(..) {
            r.complete(value.duplicate());
        }
        while let Some((t, replacement)) = self.takers.pop_front() {
            t.complete(value);
            match replacement {
                None => return,
                Some(next) => value = next,
            }
        }
        self.value = Some(value);
    }

    fn take(&mut self) -> Option<Value> {
        let value = self.value.take()?;
        if let Some((w, next)) = self.writers.pop_front() {
            self.value = Some(next);
            w.complete(Value::unit());
        }
        Some(value)
    }

    fn withdraw(&mut self, waiter: &Arc<Waiter>) {
        self.takers.retain(|(w, _)| !Arc::ptr_eq(w, waiter));
        self.readers.retain(|w| !Arc::ptr_eq(w, waiter));
        self.writers.retain(|(w, _)| !Arc::ptr_eq(w, waiter));
    }
}

pub(crate) struct MVar {
    format: i64,
    state: Mutex<State>,
}

unsafe fn cell<'a>(word: i64) -> &'a MVar {
    if word == 0
        || word & 7 != 0
        || block::kind(unsafe { block::read_header(word as usize as *const u8) })
            != block::KIND_MVAR
    {
        abi::panic_exit(b"compiler bug: corrupt MVar");
    }
    unsafe { &*(word as usize as *const MVar) }
}

/// Allocate an empty/full box. A full box takes ownership of `value`.
///
/// # Safety
/// `value` is an owned ABI word with scan code `format` when `full != 0`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_mvar_new(value: i64, format: i64, full: i64) -> i64 {
    if !(block::SCAN_RAW as i64..=block::SCAN_TOBJ as i64).contains(&format) {
        abi::panic_exit(b"compiler bug: invalid MVar value format");
    }
    let value = if full != 0 {
        Some(unsafe { Value::owned(value, format) })
    } else {
        None
    };
    block::allocate_value(
        block::KIND_MVAR,
        block::RC_FRESH,
        MVar {
            format,
            state: Mutex::new(State {
                value,
                ..State::default()
            }),
        },
    ) as usize as i64
}

/// Borrow the box for the entire call, including parks. PUT/SWAP consume
/// `value` on every path. Success writes an owned result to `out` and
/// returns 1; TRY failure or cancellation returns 0 without writing `out`.
/// IS_EMPTY instead returns the snapshot boolean and writes no result.
///
/// # Safety
/// `word` is a live MVar box; PUT/SWAP's `value` is owned with its format;
/// `out` points at writable word storage that stays live across a park.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_mvar_access(
    word: i64,
    value: i64,
    op: i64,
    mode: i64,
    out: *mut i64,
) -> i64 {
    if !(TAKE..=IS_EMPTY).contains(&op) || !(BLOCK..=CANCEL).contains(&mode) {
        abi::panic_exit(b"compiler bug: invalid MVar operation");
    }
    let m = unsafe { cell(word) };
    // Mark before locking: an incoming graph can itself reach this MVar.
    let mut input = if op == PUT || op == SWAP {
        Some(unsafe { Value::owned(value, m.format) })
    } else {
        None
    };
    let mut state = lock(&m.state);
    let result = match op {
        IS_EMPTY => return i64::from(state.value.is_none()),
        TAKE => state.take(),
        READ => state.value.as_ref().map(Value::duplicate),
        SWAP if state.value.is_some() => state.value.replace(input.take().unwrap()),
        PUT if state.value.is_none() => {
            state.fill(input.take().unwrap());
            Some(Value::unit())
        }
        _ => None,
    };
    if let Some(value) = result {
        unsafe { out.write(value.into_word()) };
        return 1;
    }
    if mode == TRY || (mode == CANCEL && cancelled()) {
        return 0;
    }

    let waiter = Waiter::new();
    match op {
        TAKE | SWAP => state.takers.push_back((Arc::clone(&waiter), input.take())),
        READ => state.readers.push_back(Arc::clone(&waiter)),
        PUT => state
            .writers
            .push_back((Arc::clone(&waiter), input.take().unwrap())),
        _ => unreachable!(),
    }
    loop {
        // Completion and withdrawal both hold the state mutex, so a
        // successful handoff can never be mistaken for cancellation.
        if let Some(value) = lock(&waiter.result).take() {
            unsafe { out.write(value.into_word()) };
            return 1;
        }
        if mode == CANCEL && cancelled() {
            state.withdraw(&waiter);
            return 0;
        }
        drop(state);
        suspend(&waiter, mode);
        state = lock(&m.state);
    }
}

fn cancelled() -> bool {
    #[cfg(not(target_family = "wasm"))]
    {
        crate::task::current_task_canceled()
    }
    #[cfg(target_family = "wasm")]
    {
        crate::coop::current_task_canceled()
    }
}

#[cfg(not(target_family = "wasm"))]
fn suspend(_waiter: &Waiter, _mode: i64) {
    park::park();
}

#[cfg(target_family = "wasm")]
fn suspend(waiter: &Waiter, mode: i64) {
    // A cooperative task runs to completion, so a blocked operation can
    // only progress by running queued work.
    crate::coop::io_sleep(0);
    if lock(&waiter.result).is_none() && !(mode == CANCEL && cancelled()) {
        abi::panic_exit(b"MVar blocked: no runnable task can complete the operation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::{self, Receiver};
    use std::time::{Duration, Instant};

    fn raw(word: i64) -> Value {
        Value {
            word,
            format: block::SCAN_RAW as i64,
        }
    }
    fn result(w: &Waiter) -> Option<i64> {
        lock(&w.result).take().map(Value::into_word)
    }

    #[test]
    fn fifo_takers_and_broadcast_readers_receive_the_promised_value() {
        let mut s = State::default();
        let (t1, t2, r1, r2) = (Waiter::new(), Waiter::new(), Waiter::new(), Waiter::new());
        s.takers.push_back((t1.clone(), None));
        s.readers.push_back(r1.clone());
        s.takers.push_back((t2.clone(), None));
        s.readers.push_back(r2.clone());
        s.fill(raw(10));
        assert_eq!(result(&t1), Some(10));
        assert_eq!(result(&r1), Some(10));
        assert_eq!(result(&r2), Some(10));
        assert_eq!(result(&t2), None);
        assert!(s.value.is_none());
        s.fill(raw(20));
        assert_eq!(result(&t2), Some(20));
        assert!(s.value.is_none());
    }

    #[test]
    fn swaps_and_writers_handoff_without_exposing_reserved_space() {
        let mut s = State::default();
        let (a, b, c) = (Waiter::new(), Waiter::new(), Waiter::new());
        s.takers.push_back((a.clone(), Some(raw(20))));
        s.takers.push_back((b.clone(), Some(raw(30))));
        s.takers.push_back((c.clone(), None));
        s.fill(raw(10));
        assert_eq!(
            (result(&a), result(&b), result(&c)),
            (Some(10), Some(20), Some(30))
        );
        assert!(s.value.is_none());
        s.fill(raw(40));
        let (w1, w2) = (Waiter::new(), Waiter::new());
        s.writers.push_back((w1.clone(), raw(50)));
        s.writers.push_back((w2.clone(), raw(60)));
        assert_eq!(s.take().map(Value::into_word), Some(40));
        assert_eq!(result(&w1), Some(0));
        assert_eq!(result(&w2), None);
        assert_eq!(s.take().map(Value::into_word), Some(50));
        assert_eq!(result(&w2), Some(0));
        assert_eq!(s.take().map(Value::into_word), Some(60));
    }

    #[test]
    fn withdrawing_middle_waiters_preserves_the_survivors_order() {
        let mut s = State::default();
        let (a, b, c, reader) = (Waiter::new(), Waiter::new(), Waiter::new(), Waiter::new());
        s.takers.push_back((a.clone(), None));
        s.takers.push_back((b.clone(), Some(raw(999))));
        s.takers.push_back((c.clone(), None));
        s.readers.push_back(reader.clone());
        s.withdraw(&b);
        s.withdraw(&reader);
        s.fill(raw(10));
        s.fill(raw(20));
        assert_eq!(
            (result(&a), result(&b), result(&c), result(&reader)),
            (Some(10), None, Some(20), None)
        );
        s.fill(raw(30));
        s.writers.push_back((a.clone(), raw(40)));
        s.writers.push_back((b.clone(), raw(999)));
        s.writers.push_back((c.clone(), raw(50)));
        s.withdraw(&b);
        assert_eq!(s.take().map(Value::into_word), Some(30));
        assert_eq!(s.take().map(Value::into_word), Some(40));
        assert_eq!(s.take().map(Value::into_word), Some(50));
    }

    unsafe fn access(m: i64, value: i64, op: i64, mode: i64) -> (i64, i64) {
        let mut out = -999;
        let status = unsafe { cinch_mvar_access(m, value, op, mode, &mut out) };
        (status, out)
    }

    fn worker(m: i64, value: i64, op: i64) -> (std::thread::JoinHandle<()>, Receiver<(i64, i64)>) {
        let (send, recv) = mpsc::channel();
        let thread = std::thread::spawn(move || {
            send.send(unsafe { access(m, value, op, BLOCK) }).unwrap();
        });
        (thread, recv)
    }

    fn registered(m: i64, takers: usize, readers: usize, writers: usize) {
        let start = Instant::now();
        loop {
            let s = lock(&unsafe { cell(m) }.state);
            if (s.takers.len(), s.readers.len(), s.writers.len()) == (takers, readers, writers) {
                return;
            }
            drop(s);
            assert!(
                start.elapsed() < Duration::from_secs(10),
                "MVar request failed to register"
            );
            std::thread::yield_now();
        }
    }

    fn receive(worker: (std::thread::JoinHandle<()>, Receiver<(i64, i64)>)) -> (i64, i64) {
        let out = worker
            .1
            .recv_timeout(Duration::from_secs(10))
            .expect("MVar worker did not wake");
        worker.0.join().unwrap();
        out
    }

    #[test]
    fn real_parked_calls_are_fifo_and_cannot_be_barged_by_try_calls() {
        unsafe {
            let m = cinch_mvar_new(0, block::SCAN_RAW as i64, 0);
            let t1 = worker(m, 0, TAKE);
            registered(m, 1, 0, 0);
            let r1 = worker(m, 0, READ);
            registered(m, 1, 1, 0);
            let t2 = worker(m, 0, TAKE);
            registered(m, 2, 1, 0);
            let r2 = worker(m, 0, READ);
            registered(m, 2, 2, 0);
            assert_eq!(access(m, 10, PUT, TRY).0, 1);
            assert_eq!(access(m, 0, TAKE, TRY).0, 0);
            assert_eq!(receive(t1), (1, 10));
            assert_eq!(receive(r1), (1, 10));
            assert_eq!(receive(r2), (1, 10));
            assert_eq!(access(m, 20, PUT, BLOCK).0, 1);
            assert_eq!(receive(t2), (1, 20));
            access(m, 30, PUT, BLOCK);
            let w1 = worker(m, 40, PUT);
            registered(m, 0, 0, 1);
            let w2 = worker(m, 50, PUT);
            registered(m, 0, 0, 2);
            assert_eq!(access(m, 0, TAKE, TRY), (1, 30));
            assert_eq!(access(m, 999, PUT, TRY).0, 0);
            assert_eq!(receive(w1), (1, 0));
            assert_eq!(access(m, 0, TAKE, TRY), (1, 40));
            assert_eq!(receive(w2), (1, 0));
            assert_eq!(access(m, 0, TAKE, TRY), (1, 50));
            rc::cinch_drop(m);
        }
    }

    #[test]
    fn raw_zero_is_a_value_and_failed_calls_do_not_write_the_output() {
        unsafe {
            let m = cinch_mvar_new(0, block::SCAN_RAW as i64, 1);
            assert_eq!(access(m, 0, READ, TRY), (1, 0));
            assert_eq!(access(m, 0, TAKE, TRY), (1, 0));
            assert_eq!(access(m, 0, TAKE, TRY), (0, -999));
            assert_eq!(access(m, 0, READ, TRY), (0, -999));
            assert_eq!(access(m, 0, IS_EMPTY, TRY).0, 1);
            rc::cinch_drop(m);
        }
    }

    #[test]
    fn values_are_published_and_released_by_the_native_box() {
        unsafe {
            rc::cinch_reclaim_enable();
            let value = abi::cinch_bytes_new(4);
            let m = cinch_mvar_new(value, block::SCAN_OBJ as i64, 1);
            let count = |w: i64| block::rc(block::read_header(w as usize as *const u8));
            assert_eq!(count(value), -1);
            let (_, read) = access(m, 0, READ, TRY);
            assert_eq!(count(value), -2);
            rc::cinch_drop(m);
            assert_eq!(count(read), -1);
            rc::cinch_drop(read);
        }
    }
}
