//! Deterministic pure parallelism: the work-stealing pool behind
//! `parBoth`, `parTabulate` and `parMap` (design/parallel-evaluation.md
//! sections 2.6 and 3).
//!
//! # Meaning
//!
//! Each entry computes its defining equation, `(f (), g ())`,
//! `array-tabulate n f` or `map f xs`, and only that. The pool decides
//! which participant evaluates which callback invocation; every result is
//! written into its own slot by exactly one participant, so the value
//! returned never depends on the worker count or the schedule. With one
//! participant ([`budget`] is 1) the entries are plain loops: no thread
//! is created, no participant is registered and no block is marked.
//!
//! # Participants
//!
//! The pool has `N` participants: `N - 1` worker threads and whichever
//! threads call an entry (the program's thread, a task thread, a worker
//! running a job). `N` is [`cinch_par_set_workers`]'s argument when the
//! program set one, else `CINCH_THREADS`, else the available parallelism.
//! Workers are started at the first call that could publish work, run on
//! guarded stacks of the program thread's size (`CINCH_STACK_SIZE`), and
//! allocate from their own small-object heaps like any thread. A smaller
//! `N` set later retires the surplus workers (they wait, never exit); a
//! larger one starts or reactivates workers.
//!
//! Every participant owns a fixed-capacity Chase-Lev deque, registered on
//! the thread's first parallel call and handed to a later thread when
//! this one exits (it is empty then, because every entry joins what it
//! published before it returns). The owner pushes and pops at the
//! bottom; thieves steal from the top.
//!
//! # Publication
//!
//! Work is published only when it can be taken at once: when more
//! participants are idle than jobs are queued ([`wants_work`]). Otherwise
//! `parBoth` runs `f ()` then `g ()` inline, and a range runs its next
//! index inline. A range splits off its upper half only when, in
//! addition, the splitting participant's deque is empty (lazy binary
//! splitting). A full deque also runs inline.
//!
//! Before a job becomes stealable, the publishing thread marks the
//! callback closure (and `parMap`'s input array) with
//! [`crate::rc::cinch_mark_mt`] while every unmarked block it reaches is
//! still reachable from this thread only; the deque's release store of
//! `bottom` and the thief's acquire load publish the marks with the job.
//! A call that never publishes marks nothing. Results cross back unmarked:
//! the participant that ran a stolen job writes its result words, then
//! completes the job's latch with a release, and the joiner's acquire
//! observation of the latch orders every count operation the participant
//! made before the joiner's own.
//!
//! # Joins
//!
//! A join first tries to pop its own job back and run it inline. If the
//! job was stolen, the joiner helps: it steals from the thief first, then
//! from anyone, rechecking the latch between jobs, and counts as idle
//! meanwhile, so busy participants publish to it. It sleeps only when
//! nothing is stealable, or without helping at all when less than half of
//! its stack remains, because a helped job runs on top of the waiting
//! frame. Idle workers spin briefly and then sleep in the same sleeper
//! set; a publication wakes one sleeper whenever any sleeps.
//!
//! # Ownership
//!
//! Every entry borrows its closures and its input array; the caller drops
//! them after the call. A closure application consumes the closure (the
//! apply stub ABI, `compiler/src/codegen.rs` `stub_def`), so a reclaiming
//! program's closure gets one reference per invocation, and a counted
//! `parMap` element one per invocation as its owned argument. Result
//! words are owned by the caller.
//!
//! # Failure
//!
//! Nothing here catches a fault: a fault, a compiler-bug trap or an
//! `Exhausted` exit in any participant ends the process from that thread,
//! as in a task, and a stolen job's stack overflow is the stack
//! `Exhausted` exit 73 because every worker runs inside
//! [`crate::stack::guarded`].

use std::cell::{Cell, UnsafeCell};
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release, SeqCst};
use std::sync::atomic::{fence, AtomicBool, AtomicIsize, AtomicPtr, AtomicU64, AtomicU8, AtomicUsize};
use std::sync::{Condvar, Mutex, MutexGuard};

use crate::{arrays, rc, task::apply_closure};

/// The Unit word a `parBoth` branch is applied to.
const UNIT_WORD: i64 = 0;

/// The largest participant budget: `CINCH_THREADS` and
/// [`cinch_par_set_workers`] are clamped to it.
pub const MAX_PARTICIPANTS: usize = 256;

/// Jobs one deque holds; a power of two. A participant whose deque is
/// full runs the work inline. Publication needs an idle participant per
/// queued job, so a deque rarely holds more than the budget.
const DEQUE_CAP: usize = 1024;
const DEQUE_MASK: usize = DEQUE_CAP - 1;

/// The most splits one range frame makes: each halves what remains.
const MAX_SPLITS: usize = 64;

/// Search rounds an idle participant makes before it sleeps.
const SPIN_ROUNDS: u32 = 128;
/// `spin_loop` hints per early search round.
const SPIN_HINTS: u32 = 64;

// ---------------------------------------------------------------------------
// The budget
// ---------------------------------------------------------------------------

/// The participant budget `N`; 0 until first resolved.
static BUDGET: AtomicUsize = AtomicUsize::new(0);

/// The participant budget: [`cinch_par_set_workers`]'s, else
/// `CINCH_THREADS`, else the available parallelism (design section 3.1).
pub fn budget() -> usize {
    match BUDGET.load(Relaxed) {
        0 => {
            let n = default_budget();
            match BUDGET.compare_exchange(0, n, Relaxed, Relaxed) {
                Ok(_) => n,
                Err(set) => set,
            }
        }
        n => n,
    }
}

/// `CINCH_THREADS` when it is a positive decimal count, else the
/// available parallelism, clamped to `1..=MAX_PARTICIPANTS`.
fn default_budget() -> usize {
    let from_env = crate::heap::env_bytes(c"CINCH_THREADS").and_then(|v| {
        let s = std::str::from_utf8(v).ok()?;
        s.parse::<usize>().ok().filter(|&n| n > 0)
    });
    from_env
        .or_else(|| std::thread::available_parallelism().ok().map(usize::from))
        .unwrap_or(1)
        .clamp(1, MAX_PARTICIPANTS)
}

// ---------------------------------------------------------------------------
// Deques
// ---------------------------------------------------------------------------

/// A cache-line aligned atomic index, so the owner's `bottom` and the
/// thieves' `top` do not share a line.
#[repr(align(128))]
struct Index(AtomicIsize);

/// A fixed-capacity Chase-Lev work-stealing deque (Le, Pop, Cohen and
/// Zappa Nardelli, PPoPP 2013, with the C11 orderings of that paper).
/// Only the owning thread pushes and pops; any thread steals.
///
/// Invariant: `top <= bottom + 1`, and the jobs are the slots
/// `top..bottom`, each the address of a [`JobHeader`] whose owner is
/// waiting for it. `bottom - top <= DEQUE_CAP`, so a slot a thief may read
/// at `top` is not overwritten while `top` stays put.
struct Deque {
    top: Index,
    bottom: Index,
    slots: [AtomicPtr<JobHeader>; DEQUE_CAP],
}

/// A steal attempt's outcome.
enum Steal {
    Empty,
    /// Another thief or the owner won the race for the top job.
    Lost,
    Got(*const JobHeader),
}

impl Deque {
    fn new() -> Deque {
        Deque {
            top: Index(AtomicIsize::new(0)),
            bottom: Index(AtomicIsize::new(0)),
            slots: [const { AtomicPtr::new(null_mut()) }; DEQUE_CAP],
        }
    }

    /// Whether the owner's deque holds no job (owner only).
    fn is_empty(&self) -> bool {
        self.bottom.0.load(Relaxed) <= self.top.0.load(Relaxed)
    }

    /// Whether a push would fit (owner only). `top` only grows, so the
    /// answer stays true until the owner pushes.
    fn has_room(&self) -> bool {
        self.bottom.0.load(Relaxed) - self.top.0.load(Acquire) < DEQUE_CAP as isize
    }

    /// Push `job` at the bottom (owner only, after [`Deque::has_room`]).
    /// The release store of `bottom` publishes the job's contents, and
    /// every mark made before it, to the thief whose acquire load sees it.
    fn push(&self, job: *const JobHeader) {
        let b = self.bottom.0.load(Relaxed);
        self.slots[b as usize & DEQUE_MASK].store(job.cast_mut(), Relaxed);
        self.bottom.0.store(b + 1, Release);
    }

    /// Pop the bottom job (owner only).
    fn pop(&self) -> Option<*const JobHeader> {
        let b = self.bottom.0.load(Relaxed) - 1;
        self.bottom.0.store(b, Relaxed);
        fence(SeqCst);
        let t = self.top.0.load(Relaxed);
        if t > b {
            self.bottom.0.store(b + 1, Relaxed);
            return None;
        }
        let job = self.slots[b as usize & DEQUE_MASK].load(Relaxed).cast_const();
        if t < b {
            return Some(job);
        }
        // The last job: race the thieves for it through `top`.
        let won = self.top.0.compare_exchange(t, t + 1, SeqCst, Relaxed).is_ok();
        self.bottom.0.store(b + 1, Relaxed);
        won.then_some(job)
    }

    /// Steal the top job (any thread).
    fn steal(&self) -> Steal {
        let t = self.top.0.load(Acquire);
        fence(SeqCst);
        let b = self.bottom.0.load(Acquire);
        if t >= b {
            return Steal::Empty;
        }
        let job = self.slots[t as usize & DEQUE_MASK].load(Relaxed).cast_const();
        if self.top.0.compare_exchange(t, t + 1, SeqCst, Relaxed).is_err() {
            return Steal::Lost;
        }
        Steal::Got(job)
    }
}

// ---------------------------------------------------------------------------
// Participants
// ---------------------------------------------------------------------------

/// One participant: a deque and the means to put its thread to sleep.
/// Participants are never freed: a thread that exits releases its
/// participant (empty) for the next thread to adopt, so there are never
/// more than the peak number of threads that called an entry at once.
pub(crate) struct Participant {
    deque: Deque,
    /// The registry link, written once before the participant is listed.
    next: AtomicPtr<Participant>,
    /// Whether a live thread owns this participant.
    owned: AtomicBool,
    /// Set by [`Participant::signal`], consumed by [`Participant::sleep`].
    signaled: Mutex<bool>,
    wake: Condvar,
}

impl Participant {
    /// Wake this participant's thread, or make its next sleep return at
    /// once. A stale signal only costs one extra loop, because every
    /// sleep rechecks why it slept.
    fn signal(&self) {
        *lock(&self.signaled) = true;
        self.wake.notify_one();
    }

    /// Sleep until signaled.
    fn sleep(&self) {
        let mut s = lock(&self.signaled);
        while !*s {
            s = self.wake.wait(s).unwrap_or_else(|e| e.into_inner());
        }
        *s = false;
    }
}

fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Every participant ever made, newest first.
static REGISTRY: AtomicPtr<Participant> = AtomicPtr::new(null_mut());
/// How many participants [`REGISTRY`] lists.
static REGISTERED: AtomicUsize = AtomicUsize::new(0);

/// A free participant, or a new one; owned by the caller either way.
///
/// The registry owns every participant for the rest of the process:
/// thieves walk the list and may read any deque at any time without a
/// lock, so a participant is reused by the next thread rather than freed,
/// as [`crate::small`] reuses its heaps.
fn adopt() -> &'static Participant {
    let mut p = REGISTRY.load(Acquire);
    while let Some(r) = unsafe { p.as_ref() } {
        if !r.owned.load(Relaxed) && r.owned.compare_exchange(false, true, Acquire, Relaxed).is_ok() {
            return r;
        }
        p = r.next.load(Relaxed);
    }
    let new = Box::into_raw(Box::new(Participant {
        deque: Deque::new(),
        next: AtomicPtr::new(null_mut()),
        owned: AtomicBool::new(true),
        signaled: Mutex::new(false),
        wake: Condvar::new(),
    }));
    let mut head = REGISTRY.load(Relaxed);
    loop {
        unsafe { (*new).next.store(head, Relaxed) };
        match REGISTRY.compare_exchange_weak(head, new, Release, Relaxed) {
            Ok(_) => break,
            Err(now) => head = now,
        }
    }
    REGISTERED.fetch_add(1, Relaxed);
    unsafe { &*new }
}

/// The calling thread's registration.
struct Local {
    me: Cell<*const Participant>,
    /// The lowest stack address at which this thread may still help:
    /// half of its stack remains above it. `usize::MAX` when the stack
    /// cannot be described, so such a thread never helps.
    help_floor: Cell<usize>,
    /// Victim selection state (xorshift).
    rng: Cell<u32>,
}

impl Drop for Local {
    fn drop(&mut self) {
        // Every entry joined what it published, so the deque is empty.
        if let Some(p) = unsafe { self.me.get().as_ref() } {
            p.owned.store(false, Release);
        }
    }
}

thread_local! {
    static LOCAL: Local = const {
        Local { me: Cell::new(std::ptr::null()), help_floor: Cell::new(usize::MAX), rng: Cell::new(0) }
    };
}

/// The calling thread's participant, registered on first use; `None`
/// once the thread is past its thread-local teardown, where the caller
/// runs sequentially.
fn participant() -> Option<&'static Participant> {
    LOCAL
        .try_with(|l| {
            if let Some(p) = unsafe { l.me.get().as_ref() } {
                return p;
            }
            let p = adopt();
            l.me.set(p);
            if let Some((low, size)) = crate::sys::this_thread_stack() {
                l.help_floor.set(low.saturating_add(size / 2));
            }
            l.rng.set((p as *const Participant as usize as u32) | 1);
            p
        })
        .ok()
}

/// Whether more than half of the calling thread's stack remains, so a
/// stolen job may run on top of the current frame.
#[inline(never)]
fn may_help() -> bool {
    let here = 0u8;
    let sp = &raw const here as usize;
    LOCAL.try_with(|l| sp >= l.help_floor.get()).unwrap_or(false)
}

/// The next victim-selection number for this thread.
fn next_random() -> u32 {
    LOCAL
        .try_with(|l| {
            let mut x = l.rng.get();
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            l.rng.set(x);
            x
        })
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// The pool
// ---------------------------------------------------------------------------

struct Pool {
    /// Worker threads started; worker `i` is active while `i + 1 <
    /// budget()`.
    started: AtomicUsize,
    spawn: Mutex<()>,
    /// Participants looking for work: active workers between jobs and
    /// joiners that may help. Publication compares it with `queued`.
    idle: AtomicUsize,
    /// Jobs published and not yet taken. Incremented before the push and
    /// decremented after the take, so it never undercounts the deques.
    queued: AtomicUsize,
    /// How many participants [`Pool::sleepers`] lists.
    sleeping: AtomicUsize,
    /// Participants asleep that a publication should wake. The lock also
    /// guards retirement waits.
    sleepers: Mutex<Vec<&'static Participant>>,
    retired: Condvar,
    published: AtomicU64,
    stolen: AtomicU64,
    helped: AtomicU64,
    slept: AtomicU64,
}

static POOL: Pool = Pool {
    started: AtomicUsize::new(0),
    spawn: Mutex::new(()),
    idle: AtomicUsize::new(0),
    queued: AtomicUsize::new(0),
    sleeping: AtomicUsize::new(0),
    sleepers: Mutex::new(Vec::new()),
    retired: Condvar::new(),
    published: AtomicU64::new(0),
    stolen: AtomicU64::new(0),
    helped: AtomicU64::new(0),
    slept: AtomicU64::new(0),
};

/// Process-wide pool counters, for tests and diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stats {
    /// Worker threads started.
    pub workers: usize,
    /// Participants looking for work right now.
    pub idle: usize,
    /// Jobs made stealable.
    pub published: u64,
    /// Jobs taken by a participant other than their publisher.
    pub stolen: u64,
    /// Stolen jobs a joiner ran while waiting for its own.
    pub helped: u64,
    /// Times a participant went to sleep.
    pub slept: u64,
}

/// The pool's counters now.
pub fn stats() -> Stats {
    Stats {
        workers: POOL.started.load(Acquire),
        idle: POOL.idle.load(Relaxed),
        published: POOL.published.load(Relaxed),
        stolen: POOL.stolen.load(Relaxed),
        helped: POOL.helped.load(Relaxed),
        slept: POOL.slept.load(Relaxed),
    }
}

/// Whether a published job would be taken at once: more participants
/// look for work than jobs wait.
#[inline]
fn wants_work() -> bool {
    POOL.idle.load(Relaxed) > POOL.queued.load(Relaxed)
}

/// The calling thread's participant when the budget allows parallel work,
/// with the workers started; `None` means run sequentially.
fn parallel_participant() -> Option<&'static Participant> {
    let n = budget();
    if n <= 1 {
        return None;
    }
    if POOL.started.load(Acquire) < n - 1 {
        start_workers(n - 1);
    }
    participant()
}

/// Start workers until `want` exist.
#[cold]
fn start_workers(want: usize) {
    let _g = lock(&POOL.spawn);
    let stack = crate::platform::program_stack_bytes();
    while POOL.started.load(Relaxed) < want {
        let index = POOL.started.load(Relaxed);
        // Counted idle from now: the worker starts by looking for work.
        POOL.idle.fetch_add(1, SeqCst);
        std::thread::Builder::new()
            .name("cinch-par".into())
            .stack_size(stack)
            .spawn(move || crate::stack::guarded(|| worker(index)))
            .unwrap_or_else(|e| crate::stack::spawn_refused(&e));
        POOL.started.store(index + 1, Release);
    }
}

/// A worker thread's life: look for work, run it, sleep when there is
/// none; wait while retired.
fn worker(index: usize) {
    let Some(me) = participant() else {
        return;
    };
    // The starter counted this worker idle.
    let mut counted = true;
    loop {
        if index + 1 >= budget() {
            if counted {
                POOL.idle.fetch_sub(1, SeqCst);
                counted = false;
            }
            // A publication may have woken this worker for its job.
            if POOL.queued.load(SeqCst) > 0 {
                wake_one();
            }
            let mut g = lock(&POOL.sleepers);
            while index + 1 >= budget() {
                g = POOL.retired.wait(g).unwrap_or_else(|e| e.into_inner());
            }
            continue;
        }
        if !counted {
            POOL.idle.fetch_add(1, SeqCst);
            counted = true;
        }
        if let Some(job) = search(me, index) {
            POOL.idle.fetch_sub(1, SeqCst);
            unsafe { run(me, job) };
            POOL.idle.fetch_add(1, SeqCst);
            continue;
        }
        if enter_sleepers(me) {
            POOL.slept.fetch_add(1, Relaxed);
            me.sleep();
            leave_sleepers(me);
        }
    }
}

/// An idle worker's search: steal, spinning then yielding between
/// attempts, for a bounded number of rounds.
fn search(me: &'static Participant, index: usize) -> Option<*const JobHeader> {
    for round in 0..SPIN_ROUNDS {
        if let Some(job) = steal_any(me, std::ptr::null()) {
            return Some(job);
        }
        if index + 1 >= budget() {
            return None;
        }
        if round < SPIN_ROUNDS / 2 {
            for _ in 0..SPIN_HINTS {
                std::hint::spin_loop();
            }
        } else {
            std::thread::yield_now();
        }
    }
    None
}

/// List `me` as asleep, unless a job is queued. The sequentially
/// consistent increment of `sleeping` and load of `queued` here, against
/// the publisher's increment of `queued` and load of `sleeping` in
/// [`publish`] and [`wake_one`], guarantee that either this sleeper sees
/// the job or the publisher sees the sleeper.
fn enter_sleepers(me: &'static Participant) -> bool {
    let mut list = lock(&POOL.sleepers);
    POOL.sleeping.fetch_add(1, SeqCst);
    if POOL.queued.load(SeqCst) > 0 {
        POOL.sleeping.fetch_sub(1, SeqCst);
        return false;
    }
    list.push(me);
    true
}

/// Take `me` off the sleeper list after a wake. Answers whether a
/// publication took it off instead (the wake was meant for a job).
fn leave_sleepers(me: &'static Participant) -> bool {
    let mut list = lock(&POOL.sleepers);
    match list.iter().position(|p| std::ptr::eq(*p, me)) {
        Some(i) => {
            list.swap_remove(i);
            POOL.sleeping.fetch_sub(1, SeqCst);
            false
        }
        None => true,
    }
}

/// Wake one sleeping participant, if any sleeps.
fn wake_one() {
    if POOL.sleeping.load(SeqCst) == 0 {
        return;
    }
    let mut list = lock(&POOL.sleepers);
    if let Some(p) = list.pop() {
        POOL.sleeping.fetch_sub(1, SeqCst);
        drop(list);
        p.signal();
    }
}

/// Make `job` stealable from the caller's deque, which has room, and wake
/// a sleeper. Marking, when needed, happened before.
fn publish(me: &'static Participant, job: &JobHeader) {
    POOL.queued.fetch_add(1, SeqCst);
    me.deque.push(job);
    POOL.published.fetch_add(1, Relaxed);
    wake_one();
}

/// Take back the caller's own job `job`, which it published last among
/// those still unjoined. Fails when a thief took it, and then the deque
/// is empty: thieves take jobs oldest first, so every job pushed before
/// it was taken too.
fn pop_own(me: &'static Participant, job: &JobHeader) -> bool {
    match me.deque.pop() {
        None => false,
        Some(p) if std::ptr::eq(p, job) => {
            POOL.queued.fetch_sub(1, SeqCst);
            true
        }
        Some(_) => crate::abi::panic_exit(b"compiler bug: parallel jobs joined out of order"),
    }
}

/// Steal one job from `p`, retrying a lost race a few times.
fn steal_from(p: &Participant) -> Option<*const JobHeader> {
    for _ in 0..4 {
        match p.deque.steal() {
            Steal::Empty => return None,
            Steal::Lost => continue,
            Steal::Got(job) => {
                POOL.queued.fetch_sub(1, SeqCst);
                POOL.stolen.fetch_add(1, Relaxed);
                return Some(job);
            }
        }
    }
    None
}

/// Steal a job from `hint` (the thief of the job the caller awaits), then
/// from every other participant, starting at a random one.
fn steal_any(me: &'static Participant, hint: *const Participant) -> Option<*const JobHeader> {
    if POOL.queued.load(Relaxed) == 0 {
        return None;
    }
    if let Some(h) = unsafe { hint.as_ref() } {
        if !std::ptr::eq(h, me) {
            if let Some(job) = steal_from(h) {
                return Some(job);
            }
        }
    }
    let count = REGISTERED.load(Relaxed).max(1);
    let skip = next_random() as usize % count;
    let head = REGISTRY.load(Acquire);
    let mut p = head;
    for _ in 0..skip {
        match unsafe { p.as_ref() } {
            Some(r) => p = r.next.load(Relaxed),
            None => break,
        }
    }
    // From the random start to the end, then from the head to it.
    for (from, until) in [(p, null_mut()), (head, p)] {
        let mut q = from;
        while q != until {
            let Some(r) = (unsafe { q.as_ref() }) else { break };
            if !std::ptr::eq(r, me) {
                if let Some(job) = steal_from(r) {
                    return Some(job);
                }
            }
            q = r.next.load(Relaxed);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Jobs and latches
// ---------------------------------------------------------------------------

const PENDING: u8 = 0;
/// The owner sleeps on the latch: completion signals it.
const SLEEPING: u8 = 1;
const DONE: u8 = 2;

/// The common head of every job record. A job lives in its owner's stack
/// frame, which does not return before the latch says [`DONE`]; after
/// completing the latch, the participant that ran the job never touches
/// it again.
#[repr(C)]
struct JobHeader {
    exec: unsafe fn(*const JobHeader, &'static Participant),
    latch: AtomicU8,
    /// The publishing participant, which waits for the latch. Static.
    owner: *const Participant,
    /// The participant running the job after a steal, for leapfrogging.
    thief: AtomicPtr<Participant>,
}

impl JobHeader {
    fn new(exec: unsafe fn(*const JobHeader, &'static Participant), owner: &'static Participant) -> JobHeader {
        JobHeader { exec, latch: AtomicU8::new(PENDING), owner, thief: AtomicPtr::new(null_mut()) }
    }

    fn done(&self) -> bool {
        self.latch.load(Acquire) == DONE
    }
}

/// Run a stolen job on the calling thread.
///
/// # Safety
///
/// `job` was taken from a deque, so its owner waits for it.
unsafe fn run(me: &'static Participant, job: *const JobHeader) {
    unsafe {
        (*job).thief.store(me as *const _ as *mut _, Relaxed);
        ((*job).exec)(job, me);
    }
}

/// Complete a stolen job's latch: everything the job wrote happens before
/// the owner's observation of [`DONE`].
///
/// # Safety
///
/// `job` is a stolen job whose results are written; it is not touched
/// again.
unsafe fn complete(job: *const JobHeader) {
    let owner = unsafe { (*job).owner };
    if unsafe { (*job).latch.swap(DONE, AcqRel) } == SLEEPING {
        unsafe { (*owner).signal() };
    }
}

/// Wait for the caller's stolen job `job`, helping meanwhile (module
/// docs, Joins).
fn wait_for(me: &'static Participant, job: &JobHeader) {
    if job.done() {
        return;
    }
    let help = may_help();
    if help {
        POOL.idle.fetch_add(1, SeqCst);
    }
    while !job.done() {
        if help {
            if let Some(other) = steal_any(me, job.thief.load(Relaxed)) {
                POOL.idle.fetch_sub(1, SeqCst);
                POOL.helped.fetch_add(1, Relaxed);
                unsafe { run(me, other) };
                POOL.idle.fetch_add(1, SeqCst);
                continue;
            }
        }
        let mut busy = false;
        for _ in 0..SPIN_ROUNDS * SPIN_HINTS {
            if job.done() || help && POOL.queued.load(Relaxed) > 0 {
                busy = true;
                break;
            }
            std::hint::spin_loop();
        }
        if busy {
            continue;
        }
        if help && !enter_sleepers(me) {
            continue;
        }
        // Completion signals this participant from now on (or already
        // did, when the latch says SLEEPING from an earlier sleep).
        let _ = job.latch.compare_exchange(PENDING, SLEEPING, AcqRel, Acquire);
        if !job.done() {
            POOL.slept.fetch_add(1, Relaxed);
            me.sleep();
        }
        if help && leave_sleepers(me) && job.done() && POOL.queued.load(SeqCst) > 0 {
            // A publication woke this joiner for a job it will not run.
            wake_one();
        }
    }
    if help {
        POOL.idle.fetch_sub(1, SeqCst);
    }
}

// ---------------------------------------------------------------------------
// Callbacks
// ---------------------------------------------------------------------------

/// Apply the borrowed closure `f` to the owned word `arg`. The apply
/// stub consumes a closure reference, so a reclaiming program's closure
/// gets one first.
///
/// # Safety
///
/// `f` is a live one-argument closure word the caller keeps alive.
#[inline]
unsafe fn call(f: i64, arg: i64) -> i64 {
    if rc::reclaiming() {
        unsafe { rc::cinch_dup(f) };
    }
    unsafe { apply_closure(f, arg) }
}

/// `parBoth`'s published branch.
#[repr(C)]
struct BothJob {
    header: JobHeader,
    closure: i64,
    result: UnsafeCell<i64>,
}

unsafe fn exec_both(job: *const JobHeader, _me: &'static Participant) {
    let both = job.cast::<BothJob>();
    unsafe {
        let r = call((*both).closure, UNIT_WORD);
        (*both).result.get().write(r);
        complete(job);
    }
}

/// One `parTabulate` or `parMap` call: the closure, the input array (0
/// for `parTabulate`) and its format, and the result array.
struct RangeCtx {
    f: i64,
    input: i64,
    input_format: i64,
    out: i64,
    /// Set once the first split marked `f` and `input`. The first split
    /// happens on the calling thread before any job of this call is
    /// stealable, and a thief reads the flag after acquiring its job, so
    /// every later split sees it set and touches no header.
    marked: AtomicBool,
}

impl RangeCtx {
    /// Invocation `i`: its result word goes to slot `i` of the result
    /// array, which only this invocation writes.
    ///
    /// # Safety
    ///
    /// `i` is below the result array's length.
    unsafe fn invoke(&self, i: usize) {
        let arg = if self.input == 0 {
            crate::abi::cinch_nat_from_i64(i as i64)
        } else {
            let x = unsafe { arrays::read(self.input, i as i64) };
            if rc::reclaiming() && arrays::counted(self.input_format) {
                unsafe { rc::cinch_dup(x) };
            }
            x
        };
        let r = unsafe { call(self.f, arg) };
        unsafe { arrays::write(self.out, i as i64, r) };
    }

    /// Promote what a stolen range job reaches, the closure and the input
    /// array, before the first split is published.
    fn mark(&self) {
        if rc::reclaiming() && !self.marked.load(Relaxed) {
            unsafe {
                rc::cinch_mark_mt(self.f);
                rc::mark_mt_array(self.input);
            }
            self.marked.store(true, Relaxed);
        }
    }
}

/// A published upper part `lo..hi` of a range.
#[repr(C)]
struct RangeJob {
    header: JobHeader,
    ctx: *const RangeCtx,
    lo: usize,
    hi: usize,
}

unsafe fn exec_range(job: *const JobHeader, me: &'static Participant) {
    let range = job.cast::<RangeJob>();
    unsafe {
        run_range(me, &*(*range).ctx, (*range).lo, (*range).hi);
        complete(job);
    }
}

/// Invoke `ctx` at every index of `lo..hi`, splitting the upper half off
/// as a job whenever some participant is idle and the caller's deque is
/// empty (lazy binary splitting), and join the splits before returning.
///
/// # Safety
///
/// `hi` is at most the result array's length, and no other participant
/// runs any index of `lo..hi`.
unsafe fn run_range(me: &'static Participant, ctx: &RangeCtx, lo: usize, hi: usize) {
    let mut splits: [MaybeUninit<RangeJob>; MAX_SPLITS] = [const { MaybeUninit::uninit() }; MAX_SPLITS];
    let mut made = 0;
    let (mut i, mut end) = (lo, hi);
    while i < end {
        if end - i >= 2 && made < MAX_SPLITS && me.deque.is_empty() && wants_work() && me.deque.has_room() {
            let mid = i + (end - i) / 2;
            ctx.mark();
            let job = splits[made].write(RangeJob {
                header: JobHeader::new(exec_range, me),
                ctx,
                lo: mid,
                hi: end,
            });
            publish(me, &job.header);
            made += 1;
            end = mid;
            continue;
        }
        unsafe { ctx.invoke(i) };
        i += 1;
    }
    // Join in reverse: the last split is the deque's bottom job.
    while made > 0 {
        made -= 1;
        let job = unsafe { splits[made].assume_init_ref() };
        if pop_own(me, &job.header) {
            unsafe { run_range(me, ctx, job.lo, job.hi) };
        } else {
            wait_for(me, &job.header);
        }
    }
}

// ---------------------------------------------------------------------------
// Sequential readings (one participant, and the cooperative backend)
// ---------------------------------------------------------------------------

/// `(f (), g ())` in order.
///
/// # Safety
///
/// As for [`cinch_par_both`].
pub(crate) unsafe fn both_sequential(f: i64, g: i64, out: i64) {
    let out = out as usize as *mut i64;
    unsafe {
        out.write(call(f, UNIT_WORD));
        out.add(1).write(call(g, UNIT_WORD));
    }
}

/// `array-tabulate n f`, index by index.
///
/// # Safety
///
/// As for [`cinch_par_tabulate`].
pub(crate) unsafe fn tabulate_sequential(n: i64, f: i64, class: i64) -> i64 {
    let ctx = unsafe { tabulate_ctx(n, f, class) };
    for i in 0..n as usize {
        unsafe { ctx.invoke(i) };
    }
    ctx.out
}

/// `map f` over the array `arr`, element by element.
///
/// # Safety
///
/// As for [`cinch_par_map_array`].
pub(crate) unsafe fn map_sequential(f: i64, arr: i64, class: i64) -> i64 {
    let ctx = unsafe { map_ctx(f, arr, class) };
    for i in 0..unsafe { arrays::cinch_array_len(arr) } as usize {
        unsafe { ctx.invoke(i) };
    }
    ctx.out
}

unsafe fn tabulate_ctx(n: i64, f: i64, class: i64) -> RangeCtx {
    // `n` comes through `cinch_nat_to_len`, so it is never negative.
    if n < 0 {
        crate::abi::raise(crate::error::Error::CorruptValue);
    }
    RangeCtx { f, input: 0, input_format: 0, out: unsafe { arrays::array(n, class) }, marked: AtomicBool::new(false) }
}

unsafe fn map_ctx(f: i64, arr: i64, class: i64) -> RangeCtx {
    let n = unsafe { arrays::cinch_array_len(arr) };
    let input_format = unsafe { arrays::format(arr) };
    RangeCtx { f, input: arr, input_format, out: unsafe { arrays::array(n, class) }, marked: AtomicBool::new(false) }
}

// ---------------------------------------------------------------------------
// Entry points (runtime ABI 8)
// ---------------------------------------------------------------------------

/// `parBoth f g`: `out[0]` is `f ()` and `out[1]` is `g ()`, both owned
/// result words; returns 0. `g` is published when a participant is idle,
/// and `f ()` runs on the calling thread.
///
/// # Safety
///
/// `f` and `g` are live closure words the caller keeps alive across the
/// call (borrowed) whose stubs take the Unit word; `out` points at two
/// writable words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_par_both(f: i64, g: i64, out: i64) -> i64 {
    if let Some(me) = parallel_participant() {
        if wants_work() && me.deque.has_room() {
            if rc::reclaiming() {
                unsafe { rc::cinch_mark_mt(g) };
            }
            let job = BothJob { header: JobHeader::new(exec_both, me), closure: g, result: UnsafeCell::new(0) };
            publish(me, &job.header);
            let a = unsafe { call(f, UNIT_WORD) };
            let b = if pop_own(me, &job.header) {
                unsafe { call(g, UNIT_WORD) }
            } else {
                wait_for(me, &job.header);
                unsafe { job.result.get().read() }
            };
            let out = out as usize as *mut i64;
            unsafe {
                out.write(a);
                out.add(1).write(b);
            }
            return 0;
        }
    }
    unsafe { both_sequential(f, g, out) };
    0
}

/// `parTabulate n f`: a fresh array of `n` elements of format `class`
/// (the array constructors' element format), element `i` the word `f i`
/// returns for the `Nat` word of `i`, stored as `cinch_array_from_list`
/// stores a list's words.
///
/// # Safety
///
/// `n` is a length from `cinch_nat_to_len`; `f` is a live closure word
/// the caller keeps alive across the call (borrowed) whose stub takes a
/// `Nat` word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_par_tabulate(n: i64, f: i64, class: i64) -> i64 {
    if n >= 2 {
        if let Some(me) = parallel_participant() {
            let ctx = unsafe { tabulate_ctx(n, f, class) };
            unsafe { run_range(me, &ctx, 0, n as usize) };
            return ctx.out;
        }
    }
    unsafe { tabulate_sequential(n, f, class) }
}

/// `parMap` over an array: a fresh array of format `class` whose element
/// `i` is `f` applied to element `i` of `arr` (an owned word: a counted
/// element is dupped for the call). `parMap f xs` elaborates to
/// `array-to-list (parMapArray f (array-from-list xs))`.
///
/// # Safety
///
/// `f` and `arr` are live words the caller keeps alive across the call
/// (borrowed); `f`'s stub takes one element word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_par_map_array(f: i64, arr: i64, class: i64) -> i64 {
    if unsafe { arrays::cinch_array_len(arr) } >= 2 {
        if let Some(me) = parallel_participant() {
            let ctx = unsafe { map_ctx(f, arr, class) };
            let n = unsafe { arrays::cinch_array_len(arr) } as usize;
            unsafe { run_range(me, &ctx, 0, n) };
            return ctx.out;
        }
    }
    unsafe { map_sequential(f, arr, class) }
}

/// `parSetWorkers n`: set the participant budget to `n` (a count from
/// `cinch_nat_to_len`), clamped to [`MAX_PARTICIPANTS`]; 0 restores the
/// default (`CINCH_THREADS`, else the available parallelism). Workers
/// past the new budget retire when they next look for work; a larger
/// budget starts or reactivates workers at the next parallel call. No
/// result depends on it. Returns 0.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_par_set_workers(n: i64) -> i64 {
    if n < 0 {
        crate::abi::raise(crate::error::Error::CorruptValue);
    }
    let n = if n == 0 { default_budget() } else { (n as usize).min(MAX_PARTICIPANTS) };
    BUDGET.store(n, Relaxed);
    // Wake every sleeper and retired worker to re-read the budget.
    let mut list = lock(&POOL.sleepers);
    let woken: Vec<&'static Participant> = list.drain(..).collect();
    POOL.sleeping.fetch_sub(woken.len(), SeqCst);
    POOL.retired.notify_all();
    drop(list);
    for p in woken {
        p.signal();
    }
    0
}
