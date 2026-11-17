//! The green-thread scheduler (design/green-threads.md): tasks as fibers
//! of `compiler/runtime/src/sched.rs`, through the runtime's `extern "C"`
//! surface and the park seam, with task bodies as stand-in closure
//! records over the closure apply-stub ABI (each stub consumes its
//! closure, as `compiler/src/codegen.rs` `stub_def` does).
//!
//! The worker budget is read once per process (`CINCH_THREADS`), so
//! every scenario runs in a child process: this test binary re-run with
//! only the `scenario` test selected and `CINCH_SCHED_SCENARIO` naming
//! what it runs. A child reports on lines `sched: key=value ...`. A
//! scheduler bug is usually a hang, so every child carries a watchdog
//! that ends it with a message and a failing exit code.
//!
//! A scenario named `...@main` runs its body as a program's `main`, on a
//! thread `cinch_run_main` registered with the scheduler: with
//! `CINCH_THREADS=1` that thread is the only one that runs tasks. The
//! same scenario without the suffix runs on the test's own thread, which
//! the scheduler does not own, so every task runs on a worker thread.
//!
//! Every scenario needs fibers, so on a target without them (or under
//! `--cfg cinch_thread_tasks`) the tests here pass without running one.

use std::collections::HashMap;
use std::hint::black_box;
use std::process::{Command, Output};
use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, AtomicUsize};
use std::sync::{Condvar, Mutex, Once};
use std::thread::ThreadId;
use std::time::{Duration, Instant};

use cinchruntime::abi::{
    cinch_alloc_ctor, cinch_alloc_shaped, cinch_io_sleep, cinch_layouts_register, cinch_scope_close,
    cinch_scope_open, cinch_task_join_all_or_cancel, cinch_task_poll_cancel, cinch_task_spawn,
};
use cinchruntime::block::{ctor_id, layout_entry, scan_slot, KIND_CLOSURE, SCAN_RAW, SCAN_TOBJ};
use cinchruntime::park::{self, Parker};
use cinchruntime::platform::cinch_run_main;
use cinchruntime::rc::{cinch_drop, cinch_exit_check, cinch_reclaim_enable};
use cinchruntime::sched::{self, cinch_sched_blocking_enter, cinch_sched_blocking_exit};
use cinchruntime::stm::{cinch_stm_state, cinch_stm_wait_begin, cinch_stm_wait_park, STATE_SLEEPERS_OFFSET};

const SCENARIO: &str = "CINCH_SCHED_SCENARIO";

/// How long a child may run before its watchdog ends it. Every scenario
/// finishes in well under a second on an idle machine.
const WATCHDOG_SECS: u64 = 120;

// ---------------------------------------------------------------------------
// Layouts and blocks
// ---------------------------------------------------------------------------

const R: u64 = SCAN_RAW;
const T: u64 = SCAN_TOBJ;

/// Closure shapes: `{ code; arity; filled; saved... }`.
const CLO_NONE: u64 = 1; // nothing saved
const CLO_TWO: u64 = 2; // saved: two raw words
/// The prelude `Ok`: tag 1 of its group, `{ TObj payload }`.
const OK: u64 = ctor_id(3, 1);
const TABLE_LEN: usize = 256;

const fn entry(codes: &[u64]) -> u64 {
    let mut map = 0;
    let mut i = 0;
    while i < codes.len() {
        map |= scan_slot(codes[i], i as u64);
        i += 1;
    }
    layout_entry(codes.len() as u64, map)
}

static TABLE: [u64; TABLE_LEN] = {
    let mut t = [0u64; TABLE_LEN];
    t[CLO_NONE as usize] = entry(&[R, R, R]);
    t[CLO_TWO as usize] = entry(&[R, R, R, R, R]);
    t[OK as usize] = entry(&[T]);
    t
};

/// Register the layouts and count references, as a reclaiming program's
/// global constructor does.
fn setup() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe {
        cinch_layouts_register(TABLE.as_ptr(), TABLE_LEN as i64);
        cinch_reclaim_enable();
    });
}

type Stub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

fn imm(v: u64) -> i64 {
    ((v << 1) | 1) as i64
}

fn unimm(w: i64) -> u64 {
    (w as u64) >> 1
}

fn field(block: i64, k: usize) -> i64 {
    unsafe { (block as usize as *const i64).add(k).read() }
}

/// A heap closure owning `saved` (raw words).
fn closure(code: Stub, saved: &[i64]) -> i64 {
    let shape = if saved.is_empty() { CLO_NONE } else { CLO_TWO };
    assert!(saved.is_empty() || saved.len() == 2);
    let size = ((3 + saved.len()) * 8) as i64;
    let p = unsafe { cinch_alloc_shaped(size, i64::from(KIND_CLOSURE), shape as i64) }.cast::<i64>();
    unsafe {
        p.write(code as usize as i64);
        p.add(1).write(saved.len() as i64 + 1);
        p.add(2).write(saved.len() as i64);
        for (i, w) in saved.iter().enumerate() {
            p.add(3 + i).write(*w);
        }
    }
    p as usize as i64
}

fn saved(clo: i64, k: usize) -> i64 {
    field(clo, 3 + k)
}

/// A task's successful outcome carrying `v`, after releasing its closure.
fn ok(clo: i64, v: u64) -> i64 {
    unsafe { cinch_drop(clo) };
    let p = unsafe { cinch_alloc_ctor(8, OK as i64) }.cast::<i64>();
    unsafe { p.write(imm(v)) };
    p as usize as i64
}

fn live() -> u64 {
    cinchruntime::heap::live_blocks()
}

fn spawn(scope: i64, code: Stub, saved: &[i64]) -> i64 {
    unsafe { cinch_task_spawn(scope, closure(code, saved)) }
}

/// Join `handles` (at least two) and answer their payloads.
fn join(scope: i64, handles: &[i64]) -> Vec<u64> {
    assert!(handles.len() >= 2, "one handle joins to its own `Ok` block, not to a payload");
    let mut out = vec![0i64; handles.len()];
    let status = unsafe {
        cinch_task_join_all_or_cancel(
            scope,
            handles.as_ptr() as usize as i64,
            handles.len() as i64,
            out.as_mut_ptr() as usize as i64,
        )
    };
    assert!(status >= 0, "every member succeeded");
    out.into_iter().map(unimm).collect()
}

/// The thread running the caller. Never inlined: a task body asks again
/// after a park, and by then it may be on another thread.
#[inline(never)]
fn thread_id() -> ThreadId {
    std::thread::current().id()
}

/// Wait for `done`, failing loudly if it never holds.
fn eventually(what: &str, done: impl Fn() -> bool) {
    let limit = Instant::now() + Duration::from_secs(30);
    while !done() {
        assert!(Instant::now() < limit, "timed out waiting until {what}");
        std::thread::yield_now();
    }
}

// ---------------------------------------------------------------------------
// One thread: a task is suspended in the middle of its body and resumed
// ---------------------------------------------------------------------------

/// 0, then 1 once A waits, 2 once B has woken it, 3 once A has resumed.
static STAGE: AtomicUsize = AtomicUsize::new(0);
static RELEASED: AtomicBool = AtomicBool::new(false);
static A_PARKER: Mutex<Option<Parker>> = Mutex::new(None);
static SEEN: Mutex<Vec<ThreadId>> = Mutex::new(Vec::new());

fn note() {
    SEEN.lock().unwrap().push(thread_id());
}

/// Task A: wait, in the middle of the body, for a wake only B gives.
unsafe extern "C" fn waits_for_b(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    note();
    *A_PARKER.lock().unwrap() = Some(park::current());
    assert_eq!(STAGE.swap(1, SeqCst), 0);
    while !RELEASED.load(SeqCst) {
        park::park();
    }
    // B ran from start to end while this frame was suspended.
    assert_eq!(STAGE.swap(3, SeqCst), 2);
    note();
    ok(clo, 11)
}

/// Task B: started after A, on the thread A was running on, while A is
/// suspended. It wakes A and runs on to its own end.
unsafe extern "C" fn wakes_a(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    note();
    assert_eq!(STAGE.load(SeqCst), 1, "A has begun and is waiting, not finished");
    RELEASED.store(true, SeqCst);
    A_PARKER.lock().unwrap().take().expect("A recorded its context").unpark();
    // The wake only queued A: with one thread it cannot run before this
    // body gives the thread up.
    assert_eq!(STAGE.swap(2, SeqCst), 1);
    ok(clo, 22)
}

fn single() {
    setup();
    let base = live();
    let caller = thread_id();
    let scope = cinch_scope_open();
    let a = spawn(scope, waits_for_b, &[]);
    let b = spawn(scope, wakes_a, &[]);
    let results = join(scope, &[a, b]);
    unsafe { cinch_scope_close(scope) };
    assert_eq!(results, [11, 22]);
    assert_eq!(STAGE.load(SeqCst), 3);
    let seen = SEEN.lock().unwrap().clone();
    // A before its wait, B, A after its wait: one OS thread throughout.
    assert_eq!(seen.len(), 3);
    let stats = sched::stats();
    println!(
        "sched: one_thread={} on_caller={} threads={} switches={} leaked={}",
        u8::from(seen.iter().all(|t| *t == seen[0])),
        u8::from(seen[0] == caller),
        stats.threads_started,
        stats.switches,
        live() as i64 - base as i64,
    );
}

// ---------------------------------------------------------------------------
// Ten thousand tasks in one scope
// ---------------------------------------------------------------------------

const MANY: usize = 10_000;

unsafe extern "C" fn index_body(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let i = unimm(saved(clo, 0));
    ok(clo, i * 3 + 1)
}

fn many() {
    setup();
    let base = live();
    let scope = cinch_scope_open();
    let handles: Vec<i64> = (0..MANY).map(|i| spawn(scope, index_body, &[imm(i as u64), 0])).collect();
    let results = join(scope, &handles);
    unsafe { cinch_scope_close(scope) };
    let in_order = results.iter().enumerate().all(|(i, r)| *r == i as u64 * 3 + 1);
    let stats = sched::stats();
    println!(
        "sched: in_order={} spawned={} threads={} workers={} stacks={} queued={} leaked={} exit_check={}",
        u8::from(in_order),
        stats.spawned,
        stats.threads_started,
        stats.workers,
        stats.stacks_mapped,
        stats.queued,
        live() as i64 - base as i64,
        cinch_exit_check(0),
    );
}

// ---------------------------------------------------------------------------
// Park and unpark under contention
// ---------------------------------------------------------------------------

const PAIRS: usize = 64;
const ROUNDS: usize = 2_000;

/// Two tasks pass a turn back and forth: the side whose turn it is
/// advances the counter and wakes the other, which waits for it parked.
struct Pair {
    turn: AtomicUsize,
    parkers: [Mutex<Option<Parker>>; 2],
}

static TABLES: [Pair; PAIRS] =
    [const { Pair { turn: AtomicUsize::new(0), parkers: [Mutex::new(None), Mutex::new(None)] } }; PAIRS];
/// Players that found themselves on another thread after a park.
static MIGRATED: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn player(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let pair = &TABLES[unimm(saved(clo, 0)) as usize];
    let side = unimm(saved(clo, 1)) as usize;
    // Recorded before the first look at the turn: the other side either
    // finds this context when it wakes, or moved before this side looked.
    *pair.parkers[side].lock().unwrap() = Some(park::current());
    let first = thread_id();
    let mut moved = false;
    for round in 0..ROUNDS {
        let mine = 2 * round + side;
        while pair.turn.load(SeqCst) != mine {
            park::park();
        }
        moved |= thread_id() != first;
        pair.turn.store(mine + 1, SeqCst);
        if let Some(other) = pair.parkers[1 - side].lock().unwrap().as_ref() {
            other.unpark();
        }
    }
    if moved {
        MIGRATED.fetch_add(1, SeqCst);
    }
    ok(clo, side as u64)
}

fn pingpong() {
    setup();
    let base = live();
    let scope = cinch_scope_open();
    let handles: Vec<i64> = (0..2 * PAIRS)
        .map(|i| spawn(scope, player, &[imm((i / 2) as u64), imm((i % 2) as u64)]))
        .collect();
    let results = join(scope, &handles);
    unsafe { cinch_scope_close(scope) };
    let complete = TABLES.iter().all(|pair| pair.turn.load(SeqCst) == 2 * ROUNDS)
        && results.iter().enumerate().all(|(i, side)| *side == (i % 2) as u64);
    let stats = sched::stats();
    println!(
        "sched: complete={} migrated={} threads={} switches={} leaked={}",
        u8::from(complete),
        MIGRATED.load(SeqCst),
        stats.threads_started,
        stats.switches,
        live() as i64 - base as i64,
    );
}

// ---------------------------------------------------------------------------
// A task that overflows its stack
// ---------------------------------------------------------------------------

/// Non-tail recursion with no end: each call keeps its frame.
#[inline(never)]
fn descend(n: u64) -> u64 {
    let pad = [n; 32];
    if black_box(n) == u64::MAX {
        return 0;
    }
    descend(n + 1) + black_box(pad)[7]
}

unsafe extern "C" fn overflow_body(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let depth = descend(0);
    ok(clo, depth)
}

unsafe extern "C" fn quick_body(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    ok(clo, 7)
}

fn overflow() {
    setup();
    let scope = cinch_scope_open();
    let deep = spawn(scope, overflow_body, &[]);
    let quick = spawn(scope, quick_body, &[]);
    join(scope, &[deep, quick]);
    println!("sched: returned=1");
}

// ---------------------------------------------------------------------------
// Cancellation resumes a task parked in a cancellable wait
// ---------------------------------------------------------------------------

/// What the cancellable wait answered: -1 until it returns.
static WAIT_ENDED: AtomicI64 = AtomicI64::new(-1);

fn stm_sleepers() -> u64 {
    let state = cinch_stm_state(0) as usize + STATE_SLEEPERS_OFFSET;
    unsafe { &*(state as *const AtomicU64) }.load(SeqCst)
}

/// A transaction's `retry`, as the prelude makes it: register, then
/// suspend until a commit or a cancellation request.
unsafe extern "C" fn waits_cancellably(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    cinch_stm_wait_begin(1 << 9);
    STAGE.store(1, SeqCst);
    let ended = cinch_stm_wait_park(1);
    WAIT_ENDED.store(ended, SeqCst);
    ok(clo, ended as u64)
}

fn cancel() {
    setup();
    let base = live();
    let scope = cinch_scope_open();
    spawn(scope, waits_cancellably, &[]);
    // The body has registered its wait and has been switched out: it is
    // parked, and no commit will ever wake it.
    eventually("the task is parked in its wait", || {
        if sched::stats().switches == 0 {
            // On the program's thread nothing runs a task until that
            // thread gives it the chance.
            cinch_io_sleep(0);
        }
        STAGE.load(SeqCst) == 1 && sched::stats().switches >= 1
    });
    assert_eq!(WAIT_ENDED.load(SeqCst), -1, "the wait has not returned");
    assert_eq!(stm_sleepers(), 1);
    // The close requests cancellation of the unjoined child and waits
    // for it: the request must reach the parked body.
    unsafe { cinch_scope_close(scope) };
    println!(
        "sched: ended={} sleepers={} switches={} leaked={}",
        WAIT_ENDED.load(SeqCst),
        stm_sleepers(),
        sched::stats().switches,
        live() as i64 - base as i64,
    );
}

// ---------------------------------------------------------------------------
// Blocking regions
// ---------------------------------------------------------------------------

/// How many bodies are blocked on the gate, and whether it is open.
static GATE: Mutex<(usize, bool)> = Mutex::new((0, false));
static GATE_MOVED: Condvar = Condvar::new();

/// Block the calling OS thread until the gate opens.
fn wait_at_gate() {
    let mut gate = GATE.lock().unwrap();
    gate.0 += 1;
    GATE_MOVED.notify_all();
    while !gate.1 {
        gate = GATE_MOVED.wait(gate).unwrap();
    }
}

/// A task that blocks its thread inside a blocking region, through the
/// entry the runtime's own blocking calls use.
unsafe extern "C" fn blocks_in_region(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    sched::blocking(wait_at_gate);
    ok(clo, 1)
}

/// The same through the two externs generated code brackets a `Blocking`
/// foreign call with.
unsafe extern "C" fn blocks_between_externs(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    cinch_sched_blocking_enter();
    wait_at_gate();
    cinch_sched_blocking_exit();
    ok(clo, 2)
}

unsafe extern "C" fn opens_gate(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    GATE.lock().unwrap().1 = true;
    GATE_MOVED.notify_all();
    ok(clo, 3)
}

/// With a budget of two, two tasks block their workers until a third
/// task runs: it only runs if the regions made room for a third worker.
fn blocking() {
    setup();
    let scope = cinch_scope_open();
    let a = spawn(scope, blocks_in_region, &[]);
    let b = spawn(scope, blocks_between_externs, &[]);
    // Both workers of the budget are blocked before the third task
    // exists.
    {
        let mut gate = GATE.lock().unwrap();
        while gate.0 < 2 {
            gate = GATE_MOVED.wait(gate).unwrap();
        }
    }
    let at_gate = sched::stats();
    let c = spawn(scope, opens_gate, &[]);
    let results = join(scope, &[a, b, c]);
    unsafe { cinch_scope_close(scope) };
    assert_eq!(results, [1, 2, 3]);
    // The worker that stood in leaves once the regions have ended.
    eventually("the surplus worker has retired", || sched::stats().workers <= 2);
    let stats = sched::stats();
    println!(
        "sched: blocked_at_gate={} threads={} workers={} blocked={}",
        at_gate.blocked, stats.threads_started, stats.workers, stats.blocked
    );
}

/// The program's thread is the only worker and blocks in a region until
/// a task runs.
fn blocking_main() {
    setup();
    let scope = cinch_scope_open();
    let c = spawn(scope, opens_gate, &[]);
    let d = spawn(scope, quick_body, &[]);
    cinch_sched_blocking_enter();
    wait_at_gate();
    cinch_sched_blocking_exit();
    let results = join(scope, &[c, d]);
    unsafe { cinch_scope_close(scope) };
    assert_eq!(results, [3, 7]);
    let stats = sched::stats();
    println!("sched: threads={} blocked={}", stats.threads_started, stats.blocked);
}

// ---------------------------------------------------------------------------
// Timed parks
// ---------------------------------------------------------------------------

const SLEEP_MS: u64 = 50;
/// Long enough that only a cancellation ends it within the test.
const LONG_SLEEP_MS: i64 = 600_000;

static SLEPT_MS: AtomicU64 = AtomicU64::new(0);
static OTHER_RAN: AtomicBool = AtomicBool::new(false);
static OTHER_RAN_MEANWHILE: AtomicBool = AtomicBool::new(false);
static LONG_BEGAN: AtomicBool = AtomicBool::new(false);
static LONG_MS: AtomicU64 = AtomicU64::new(u64::MAX);
static LONG_CANCELED: AtomicI64 = AtomicI64::new(-1);
static QUICK_RAN: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn sleeps(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let begun = Instant::now();
    cinch_io_sleep(SLEEP_MS as i64);
    SLEPT_MS.store(begun.elapsed().as_millis() as u64, SeqCst);
    OTHER_RAN_MEANWHILE.store(OTHER_RAN.load(SeqCst), SeqCst);
    ok(clo, 1)
}

unsafe extern "C" fn runs_meanwhile(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    OTHER_RAN.store(true, SeqCst);
    ok(clo, 2)
}

unsafe extern "C" fn sleeps_until_canceled(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let begun = Instant::now();
    LONG_BEGAN.store(true, SeqCst);
    cinch_io_sleep(LONG_SLEEP_MS);
    LONG_MS.store(begun.elapsed().as_millis() as u64, SeqCst);
    LONG_CANCELED.store(cinch_task_poll_cancel(), SeqCst);
    ok(clo, 3)
}

unsafe extern "C" fn sets_quick(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    QUICK_RAN.store(true, SeqCst);
    ok(clo, 4)
}

fn sleep() {
    setup();
    let scope = cinch_scope_open();
    let sleeper = spawn(scope, sleeps, &[]);
    let other = spawn(scope, runs_meanwhile, &[]);
    spawn(scope, sleeps_until_canceled, &[]);
    let results = join(scope, &[sleeper, other]);
    assert_eq!(results, [1, 2]);
    eventually("the long sleeper is asleep", || {
        cinch_io_sleep(0);
        LONG_BEGAN.load(SeqCst) && sched::stats().timers == 1
    });
    // The caller's own sleep is a wait like any other: a task spawned
    // before it has run by the time it returns.
    spawn(scope, sets_quick, &[]);
    let begun = Instant::now();
    cinch_io_sleep(SLEEP_MS as i64);
    let caller_ms = begun.elapsed().as_millis();
    let quick_ran = QUICK_RAN.load(SeqCst);
    // The close cancels the long sleeper, which must return at once.
    unsafe { cinch_scope_close(scope) };
    let stats = sched::stats();
    println!(
        "sched: slept_ms={} other_meanwhile={} caller_ms={} quick_ran={} long_ms={} long_canceled={} timers={} threads={}",
        SLEPT_MS.load(SeqCst),
        u8::from(OTHER_RAN_MEANWHILE.load(SeqCst)),
        caller_ms,
        u8::from(quick_ran),
        LONG_MS.load(SeqCst),
        LONG_CANCELED.load(SeqCst),
        stats.timers,
        stats.threads_started,
    );
}

// ---------------------------------------------------------------------------
// What a task costs
// ---------------------------------------------------------------------------

/// Spawn one task and join it, `rounds` times: the time per round.
fn spawn_join_one(rounds: usize) -> Duration {
    let scope = cinch_scope_open();
    let begun = Instant::now();
    for _ in 0..rounds {
        let handle = spawn(scope, quick_body, &[]);
        let mut out = [0i64; 1];
        let status = unsafe {
            cinch_task_join_all_or_cancel(
                scope,
                &handle as *const i64 as usize as i64,
                1,
                out.as_mut_ptr() as usize as i64,
            )
        };
        assert_eq!(status, 0);
        unsafe { cinch_drop(out[0]) };
    }
    let each = begun.elapsed() / rounds as u32;
    unsafe { cinch_scope_close(scope) };
    each
}

/// Spawn `count` tasks into one scope, then join them all: the time per
/// task.
fn spawn_join_batch(count: usize) -> Duration {
    let scope = cinch_scope_open();
    let begun = Instant::now();
    let handles: Vec<i64> = (0..count).map(|_| spawn(scope, quick_body, &[])).collect();
    black_box(join(scope, &handles));
    let each = begun.elapsed() / count as u32;
    unsafe { cinch_scope_close(scope) };
    each
}

/// The numbers of design/green-threads.md: the best of five passes of
/// each shape. Runs on both backends.
fn bench() {
    setup();
    let best = |pass: &dyn Fn() -> Duration| (0..5).map(|_| pass()).min().unwrap();
    let one = best(&|| spawn_join_one(2_000));
    let batch = best(&|| spawn_join_batch(2_000));
    let stats = sched::stats();
    println!(
        "sched: fibers={} one_ns={} batch_ns={} threads={} stacks={}",
        u8::from(sched::FIBERS),
        one.as_nanos(),
        batch.as_nanos(),
        stats.threads_started,
        stats.stacks_mapped,
    );
}

/// How many tasks [`suspended`] holds suspended, when the parent names a
/// count.
const SUSPENDED: &str = "CINCH_SCHED_SUSPENDED";
static WAITING_TASKS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn waits_until_canceled(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    cinch_stm_wait_begin(1 << 11);
    WAITING_TASKS.fetch_add(1, SeqCst);
    let ended = cinch_stm_wait_park(1);
    ok(clo, ended as u64)
}

/// The resident memory of this process, in KiB, as the system reports
/// it.
fn resident_kib() -> u64 {
    let out = Command::new("ps").args(["-o", "rss=", "-p", &std::process::id().to_string()]).output().unwrap();
    String::from_utf8_lossy(&out.stdout).trim().parse().unwrap_or(0)
}

/// What it costs to hold many tasks suspended at once: every task waits
/// until the scope's close cancels it.
fn suspended() {
    setup();
    let count: usize = std::env::var(SUSPENDED).ok().and_then(|v| v.parse().ok()).unwrap_or(10_000);
    let before = resident_kib();
    let scope = cinch_scope_open();
    let begun = Instant::now();
    for _ in 0..count {
        spawn(scope, waits_until_canceled, &[]);
    }
    eventually("every task is waiting", || {
        cinch_io_sleep(0);
        WAITING_TASKS.load(SeqCst) == count && sched::stats().queued == 0
    });
    let started_ms = begun.elapsed().as_millis();
    let held = resident_kib();
    let stats = sched::stats();
    let begun = Instant::now();
    unsafe { cinch_scope_close(scope) };
    println!(
        "sched: fibers={} count={count} started_ms={started_ms} resident_kib={} stacks={} threads={} closed_ms={}",
        u8::from(sched::FIBERS),
        held.saturating_sub(before),
        stats.stacks_mapped,
        stats.threads_started,
        begun.elapsed().as_millis(),
    );
}

// ---------------------------------------------------------------------------
// The child dispatcher and the parent side
// ---------------------------------------------------------------------------

/// The scenario the child was asked for, as a program's `main`.
extern "C" fn scenario_main() -> i64 {
    run_scenario(&std::env::var(SCENARIO).unwrap());
    0
}

fn run_scenario(name: &str) {
    match name.trim_end_matches("@main") {
        "single" => single(),
        "many" => many(),
        "pingpong" => pingpong(),
        "overflow" => overflow(),
        "cancel" => cancel(),
        "blocking" => blocking(),
        "blocking-main" => blocking_main(),
        "sleep" => sleep(),
        "bench" => bench(),
        "suspended" => suspended(),
        other => panic!("unknown scenario {other}"),
    }
}

/// Runs the scenario a parent selected; does nothing in a normal run.
#[test]
fn scenario() {
    let Ok(name) = std::env::var(SCENARIO) else {
        return;
    };
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(WATCHDOG_SECS));
        eprintln!("sched: the scenario did not finish in {WATCHDOG_SECS}s");
        std::process::exit(99);
    });
    if name.ends_with("@main") {
        assert_eq!(cinch_run_main(scenario_main), 0);
    } else {
        run_scenario(&name);
    }
}

/// Children run one at a time, so the timed ones see the machine.
static CHILDREN: Mutex<()> = Mutex::new(());

fn child(name: &str, threads: &str, env: &[(&str, &str)]) -> Output {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.args(["scenario", "--exact", "--nocapture", "--test-threads=1"]).env(SCENARIO, name);
    for var in ["CINCH_RC_DEBUG", "CINCH_LEAK_CHECK", "CINCH_HEAP_LIMIT", "CINCH_STACK_SIZE"] {
        cmd.env_remove(var);
    }
    cmd.env("CINCH_THREADS", threads);
    cmd.envs(env.iter().copied());
    cmd.output().unwrap()
}

/// The `sched:` lines of a successful child's stdout as one map.
fn report(name: &str, threads: &str, env: &[(&str, &str)]) -> HashMap<String, String> {
    let out = child(name, threads, env);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success(), "{name} with {threads} threads: {:?}\n{stderr}", out.status);
    let stdout = String::from_utf8_lossy(&out.stdout);
    let fields: HashMap<String, String> = stdout
        .lines()
        .filter_map(|l| l.split_once("sched: ").map(|(_, rest)| rest))
        .flat_map(|l| l.split(' '))
        .filter_map(|kv| kv.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    assert!(!fields.is_empty(), "{name} with {threads} threads reported nothing:\n{stdout}\n{stderr}");
    fields
}

fn number(r: &HashMap<String, String>, key: &str) -> i64 {
    r[key].parse().unwrap_or_else(|_| panic!("{key}={} is not a number", r[key]))
}

#[test]
fn one_thread_suspends_a_task_mid_body_and_resumes_it() {
    if !sched::FIBERS {
        return;
    }
    // The program's thread is the only thread that runs tasks: no worker
    // is ever started.
    let r = report("single@main", "1", &[]);
    assert_eq!(r["one_thread"], "1");
    assert_eq!(r["on_caller"], "1", "the tasks ran on the program's thread");
    assert_eq!(r["threads"], "0", "a budget of one starts no worker");
    assert_eq!(r["leaked"], "0");
    // A began, parked, B ran, A resumed: at least three switches.
    assert!(number(&r, "switches") >= 3, "{r:?}");
    // From a thread the scheduler does not own, the one worker of the
    // budget does the same.
    let r = report("single", "1", &[]);
    assert_eq!(r["one_thread"], "1");
    assert_eq!(r["on_caller"], "0");
    assert_eq!(r["threads"], "1");
    assert_eq!(r["leaked"], "0");
}

#[test]
fn ten_thousand_tasks_in_one_scope_join_and_leak_nothing() {
    if !sched::FIBERS {
        return;
    }
    for (name, workers) in [("many", 4), ("many@main", 3)] {
        let r = report(name, "4", &[("CINCH_LEAK_CHECK", "1")]);
        assert_eq!(r["in_order"], "1", "{name}");
        assert_eq!(number(&r, "spawned"), MANY as i64, "{name}");
        assert_eq!(r["leaked"], "0", "{name}");
        assert_eq!(r["exit_check"], "0", "{name}");
        assert_eq!(r["queued"], "0", "{name}");
        assert!(number(&r, "threads") <= workers, "{name}: {r:?}");
        // No task ever waits, so the stacks in use never exceed the
        // threads running tasks: ten thousand tasks, a handful of stacks.
        assert!(number(&r, "stacks") <= 4, "{name}: {r:?}");
    }
    // One thread runs all ten thousand, on one stack.
    let r = report("many@main", "1", &[("CINCH_LEAK_CHECK", "1")]);
    assert_eq!(r["in_order"], "1");
    assert_eq!(r["leaked"], "0");
    assert_eq!(r["threads"], "0");
    assert_eq!(r["stacks"], "1");
}

#[test]
fn parked_tasks_are_woken_across_workers_without_a_lost_wake() {
    if !sched::FIBERS {
        return;
    }
    for threads in ["1", "2", "4", "8"] {
        for name in ["pingpong", "pingpong@main"] {
            let r = report(name, threads, &[]);
            assert_eq!(r["complete"], "1", "{name} with {threads} threads");
            assert_eq!(r["leaked"], "0", "{name} with {threads} threads");
            // Every turn but a pair's first few is a park and a wake.
            assert!(number(&r, "switches") > (PAIRS * ROUNDS) as i64, "{name} with {threads} threads: {r:?}");
        }
    }
    // With several workers a woken task is taken by whichever is free.
    let r = report("pingpong", "4", &[]);
    assert!(number(&r, "migrated") > 0, "no task ever resumed on another thread: {r:?}");
}

#[test]
fn a_task_that_recurses_without_bound_is_the_exhausted_exit() {
    if !sched::FIBERS {
        return;
    }
    for (name, threads) in [("overflow", "2"), ("overflow@main", "1")] {
        let out = child(name, threads, &[]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(73), "{name}: {stderr}");
        assert!(stderr.contains("cinch: exhausted: stack"), "{name}: {stderr}");
        assert!(!String::from_utf8_lossy(&out.stdout).contains("returned=1"), "{name}");
    }
}

#[test]
fn a_cancellation_request_resumes_a_task_parked_in_a_cancellable_wait() {
    if !sched::FIBERS {
        return;
    }
    for (name, threads) in [("cancel@main", "1"), ("cancel", "4")] {
        let r = report(name, threads, &[]);
        assert_eq!(r["ended"], "1", "{name}: the wait reports the cancellation");
        assert_eq!(r["sleepers"], "0", "{name}: the wait left the registry");
        assert!(number(&r, "switches") >= 2, "{name}: {r:?}");
        assert_eq!(r["leaked"], "0", "{name}");
    }
}

#[test]
fn a_blocking_region_makes_room_for_the_task_it_waits_for() {
    if !sched::FIBERS {
        return;
    }
    let r = report("blocking", "2", &[]);
    assert_eq!(r["blocked_at_gate"], "2");
    assert_eq!(r["threads"], "3", "two workers of the budget and one that stood in");
    assert_eq!(r["workers"], "2");
    assert_eq!(r["blocked"], "0");
    // The program's thread is the whole budget and it is the one blocked.
    let r = report("blocking-main@main", "1", &[]);
    assert_eq!(r["threads"], "1");
    assert_eq!(r["blocked"], "0");
}

#[test]
fn a_sleeping_task_holds_no_thread_and_wakes_on_time_or_on_cancellation() {
    if !sched::FIBERS {
        return;
    }
    for (name, threads) in [("sleep@main", "1"), ("sleep", "4")] {
        let r = report(name, threads, &[]);
        assert!(number(&r, "slept_ms") >= SLEEP_MS as i64, "{name}: woke early: {r:?}");
        assert!(number(&r, "slept_ms") < 30_000, "{name}: {r:?}");
        assert_eq!(r["other_meanwhile"], "1", "{name}: another task ran during the sleep");
        assert!(number(&r, "caller_ms") >= SLEEP_MS as i64, "{name}: {r:?}");
        assert_eq!(r["quick_ran"], "1", "{name}: a task ran during the caller's sleep");
        assert!(number(&r, "long_ms") < LONG_SLEEP_MS / 2, "{name}: the cancellation did not end the sleep: {r:?}");
        assert_eq!(r["long_canceled"], "1", "{name}");
        assert_eq!(r["timers"], "0", "{name}: the cancelled sleep left its timer behind");
    }
    let r = report("sleep@main", "1", &[]);
    assert_eq!(r["threads"], "0");
}

/// The cost of tasks on this machine, for design/green-threads.md: a
/// spawn and a join, and ten thousand tasks held suspended at once. Not
/// a test of anything: run it with `--release -- --ignored --nocapture`,
/// and with `--cfg cinch_thread_tasks` for the cost of one OS thread per
/// task.
#[test]
#[ignore]
fn what_tasks_cost() {
    for (name, threads) in [("bench", "4"), ("bench@main", "4"), ("bench@main", "1")] {
        let r = report(name, threads, &[]);
        println!(
            "{name} with {threads} threads: fibers={} one spawn+join {} ns, in a batch of 2000 {} ns per task, {} worker threads started, {} stacks mapped",
            r["fibers"], r["one_ns"], r["batch_ns"], r["threads"], r["stacks"]
        );
    }
    let r = report("suspended", "4", &[(SUSPENDED, "10000")]);
    println!(
        "suspended: fibers={} {} tasks suspended after {} ms, {} KiB resident for them, {} stacks mapped, cancelled and joined in {} ms",
        r["fibers"], r["count"], r["started_ms"], r["resident_kib"], r["stacks"], r["closed_ms"]
    );
}
