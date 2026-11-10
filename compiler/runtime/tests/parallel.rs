//! Deterministic parallel evaluation (design/parallel-evaluation.md,
//! phase 1): the pool of `compiler/runtime/src/par.rs` through its
//! `extern "C"` surface, with callbacks as stand-in closure records over
//! the closure apply-stub ABI (each stub consumes its closure, as
//! `compiler/src/codegen.rs` `stub_def` does).
//!
//! Everything that depends on the process (the participant budget read
//! from `CINCH_THREADS`, `CINCH_RC_DEBUG`, `CINCH_LEAK_CHECK`, a pool that
//! has never started, an exit code) runs in a child process: this test
//! binary re-run with only the `scenario` test selected and
//! `CINCH_PAR_SCENARIO` naming what it runs. A child reports on lines
//! `par: key=value ...`.

use std::collections::{HashMap, HashSet};
use std::hint::black_box;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
use std::sync::{Mutex, Once, OnceLock};
use std::thread::ThreadId;
use std::time::{Duration, Instant};

use cinchruntime::abi::{
    cinch_alloc, cinch_alloc_ctor, cinch_alloc_shaped, cinch_layouts_register, cinch_panic, cinch_scope_close,
    cinch_scope_open, cinch_task_join_all_or_cancel, cinch_task_spawn,
};
use cinchruntime::arrays::{cinch_array_from_list, cinch_array_len};
use cinchruntime::block::{
    ctor_id, layout_entry, read_header, scan_slot, static_header, KIND_CLOSURE, SCAN_OBJ, SCAN_RAW, SCAN_TOBJ,
};
use cinchruntime::coop::{cinch_coop_par_both, cinch_coop_par_map_array, cinch_coop_par_tabulate};
use cinchruntime::par::{cinch_par_both, cinch_par_map_array, cinch_par_set_workers, cinch_par_tabulate, stats};
use cinchruntime::rc::{cinch_drop, cinch_dup, cinch_exit_check, cinch_reclaim_enable};

const SCENARIO: &str = "CINCH_PAR_SCENARIO";

// ---------------------------------------------------------------------------
// Layouts and blocks
// ---------------------------------------------------------------------------

const R: u64 = SCAN_RAW;
const O: u64 = SCAN_OBJ;
const T: u64 = SCAN_TOBJ;

/// Closure shapes: `{ code; arity; filled; saved... }`.
const CLO_LIST: u64 = 1; // saved: the shared list
const CLO_NODE: u64 = 2; // saved: depth and path (raw), the shared list
const CLO_NONE: u64 = 3; // nothing saved
/// Constructors: one group per type, the tag in the low bits.
const NIL: u64 = ctor_id(1, 0);
const CONS: u64 = ctor_id(1, 1); // { TObj head; Obj tail }
const LEAF: u64 = ctor_id(2, 0); // { TObj value; Obj shared cell }
const BOXED: u64 = ctor_id(2, 1); // { TObj value }
const OK: u64 = ctor_id(3, 1); // the prelude `Ok` tag, { TObj payload }
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
    t[CLO_LIST as usize] = entry(&[R, R, R, O]);
    t[CLO_NODE as usize] = entry(&[R, R, R, R, O]);
    t[CLO_NONE as usize] = entry(&[R, R, R]);
    t[NIL as usize] = entry(&[]);
    t[CONS as usize] = entry(&[T, O]);
    t[LEAF as usize] = entry(&[T, O]);
    t[BOXED as usize] = entry(&[T]);
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

fn ctor(desc: u64, fields: &[i64]) -> i64 {
    let p = unsafe { cinch_alloc_ctor((fields.len() * 8) as i64, desc as i64) }.cast::<i64>();
    for (i, f) in fields.iter().enumerate() {
        unsafe { p.add(i).write(*f) };
    }
    p as usize as i64
}

/// A heap closure of `shape` owning `saved`.
fn closure(shape: u64, code: Stub, saved: &[i64]) -> i64 {
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

/// An immortal closure with nothing saved, like a global function's
/// static `.clo` block: counting it is a no-op.
fn static_closure(code: Stub) -> i64 {
    let b = Box::new([static_header(KIND_CLOSURE, CLO_NONE as u16) as i64, code as usize as i64, 1, 0]);
    Box::into_raw(b) as usize as i64 + 8
}

fn saved(clo: i64, k: usize) -> i64 {
    field(clo, 3 + k)
}

fn count_of(word: i64) -> i64 {
    cinchruntime::block::rc(unsafe { read_header(word as usize as *const u8) })
}

fn live() -> u64 {
    cinchruntime::heap::live_blocks()
}

// ---------------------------------------------------------------------------
// Workloads
// ---------------------------------------------------------------------------

const LIST_LEN: u64 = 1000;
const TAB_LEN: i64 = 10_000;
const MAP_LEN: u64 = 10_000;
const RUNS: usize = 20;
const TREE_DEPTH: u64 = 20;
const MASK40: u64 = (1 << 40) - 1;

/// The list `[0, 1, ..., n - 1]` (heads as immediates).
fn build_list(n: u64) -> i64 {
    let mut xs = ctor(NIL, &[]);
    for v in (0..n).rev() {
        xs = ctor(CONS, &[imm(v * 7 + 3), xs]);
    }
    xs
}

/// Read the whole shared list for index `i`: a fresh leaf holding a
/// weighted sum and a counted reference to cell `i mod LIST_LEN` of the
/// list, so results share structure with the capture.
fn leaf_for(xs: i64, i: u64) -> i64 {
    let (mut sum, mut cell, mut nth) = (0u64, xs, 0i64);
    for k in 0..LIST_LEN {
        sum = sum.wrapping_add(unimm(field(cell, 0)).wrapping_mul(i + 1 + k));
        if k == i % LIST_LEN {
            nth = cell;
        }
        cell = field(cell, 1);
    }
    unsafe { cinch_dup(nth) };
    ctor(LEAF, &[imm(sum & MASK40), nth])
}

/// `\i -> leaf-for xs i` over a tabulate index (a `Nat` word).
unsafe extern "C" fn tab_stub(clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    let i = unimm(unsafe { argv.read() });
    let r = leaf_for(saved(clo, 0), i);
    unsafe { cinch_drop(clo) };
    r
}

/// `\b -> leaf-for xs (value b)` over an owned boxed element.
unsafe extern "C" fn map_stub(clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    let x = unsafe { argv.read() };
    let r = leaf_for(saved(clo, 0), unimm(field(x, 0)));
    unsafe {
        cinch_drop(x);
        cinch_drop(clo);
    }
    r
}

/// An order-sensitive digest of an array of leaves.
fn digest(arr: i64) -> u64 {
    let n = unsafe { cinch_array_len(arr) } as usize;
    let mut d = 0u64;
    for i in 0..n {
        let leaf = field(arr, 2 + i);
        let v = unimm(field(leaf, 0)) ^ unimm(field(field(leaf, 1), 0)) << 41;
        d = d.wrapping_mul(1_000_003).wrapping_add(v);
    }
    d
}

fn mix(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51_afd7_ed55_8ccd);
    x ^= x >> 33;
    x
}

/// One node of a `parBoth` tree: the sum of `2^depth` leaves, each child
/// a closure over the shared list, as `sum-tree` would build them.
fn node(depth: u64, path: u64, list: i64) -> u64 {
    if depth == 0 {
        return (mix(path) & 0xF_FFFF) + unimm(field(list, 0));
    }
    let f = node_closure(depth - 1, path * 2, list);
    let g = node_closure(depth - 1, path * 2 + 1, list);
    let mut out = [0i64; 2];
    unsafe {
        cinch_par_both(f, g, out.as_mut_ptr() as usize as i64);
        cinch_drop(f);
        cinch_drop(g);
    }
    unimm(out[0]) + unimm(out[1])
}

fn node_closure(depth: u64, path: u64, list: i64) -> i64 {
    unsafe { cinch_dup(list) };
    closure(CLO_NODE, node_stub, &[(depth << 32 | path) as i64, list])
}

unsafe extern "C" fn node_stub(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let tag = saved(clo, 0) as u64;
    let r = imm(node(tag >> 32, tag & 0xFFFF_FFFF, saved(clo, 1)));
    unsafe { cinch_drop(clo) };
    r
}

/// The `parMap` input: boxed values `0, 5, 10, ...` in an array.
fn build_input() -> i64 {
    let mut xs = ctor(NIL, &[]);
    for v in (0..MAP_LEN).rev() {
        xs = ctor(CONS, &[ctor(BOXED, &[imm(v * 5)]), xs]);
    }
    let arr = unsafe { cinch_array_from_list(xs, 1) };
    unsafe { cinch_drop(xs) };
    arr
}

/// The determinism and reclamation workload, at the budget the
/// environment sets: 20 tabulates and 20 maps reading a shared 1,000-cell
/// list, a depth-20 `parBoth` tree, the cooperative entries, then every
/// block dropped.
fn workloads() {
    setup();
    let base = live();
    let list = build_list(LIST_LEN);
    unsafe { cinch_dup(list) };
    let f_tab = closure(CLO_LIST, tab_stub, &[list]);
    unsafe { cinch_dup(list) };
    let f_map = closure(CLO_LIST, map_stub, &[list]);
    let input = build_input();
    let (mut tab, mut map) = (None, None);
    for _ in 0..RUNS {
        let t = unsafe { cinch_par_tabulate(TAB_LEN, f_tab, 1) };
        let d = digest(t);
        assert!(tab.is_none_or(|p| p == d), "tabulate differs between runs");
        tab = Some(d);
        unsafe { cinch_drop(t) };
        let m = unsafe { cinch_par_map_array(f_map, input, 1) };
        let d = digest(m);
        assert!(map.is_none_or(|p| p == d), "map differs between runs");
        map = Some(d);
        unsafe { cinch_drop(m) };
    }
    let tree = node(TREE_DEPTH, 0, list);

    // The cooperative backend's sequential loops agree.
    let t = unsafe { cinch_coop_par_tabulate(TAB_LEN, f_tab, 1) };
    assert_eq!(Some(digest(t)), tab, "cooperative tabulate");
    unsafe { cinch_drop(t) };
    let m = unsafe { cinch_coop_par_map_array(f_map, input, 1) };
    assert_eq!(Some(digest(m)), map, "cooperative map");
    unsafe { cinch_drop(m) };
    let (f, g) = (node_closure(10, 0, list), node_closure(10, 1, list));
    let mut out = [0i64; 2];
    unsafe { cinch_coop_par_both(f, g, out.as_mut_ptr() as usize as i64) };
    assert_eq!(unimm(out[0]) + unimm(out[1]), node(11, 0, list), "cooperative parBoth");
    unsafe {
        cinch_drop(f);
        cinch_drop(g);
    }

    let marked = count_of(f_tab) < 0 || count_of(list) < 0 || count_of(input) < 0;
    unsafe {
        cinch_drop(f_tab);
        cinch_drop(f_map);
        cinch_drop(input);
        cinch_drop(list);
    }
    let leaked = live() as i64 - base as i64;
    let s = stats();
    println!(
        "par: tab={} map={} tree={tree} leaked={leaked} marked={} workers={} published={} stolen={} helped={}",
        tab.unwrap(),
        map.unwrap(),
        u8::from(marked),
        s.workers,
        s.published,
        s.stolen,
        s.helped
    );
    println!("par: exit_check={}", cinch_exit_check(0));
}

// ---------------------------------------------------------------------------
// Scenario helpers
// ---------------------------------------------------------------------------

static CALLER: OnceLock<ThreadId> = OnceLock::new();
static STARTED_ELSEWHERE: AtomicBool = AtomicBool::new(false);

/// Wait up to `limit` for `flag`; false on timeout.
fn wait_flag(flag: &AtomicBool, limit: Duration) -> bool {
    let start = Instant::now();
    while !flag.load(SeqCst) {
        if start.elapsed() > limit {
            return false;
        }
        std::hint::spin_loop();
    }
    true
}

fn on_caller() -> bool {
    CALLER.get() == Some(&std::thread::current().id())
}

/// `f` of a `parBoth` whose `g` must start on another participant: it
/// waits for that, and a timeout ends the child with exit 3.
unsafe extern "C" fn wait_for_g(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    if !wait_flag(&STARTED_ELSEWHERE, Duration::from_secs(20)) {
        std::process::exit(3);
    }
    imm(1)
}

fn note_started() {
    if !on_caller() {
        STARTED_ELSEWHERE.store(true, SeqCst);
    }
}

#[inline(never)]
fn deep(n: u64) -> u64 {
    let frame = [n; 32];
    black_box(&frame);
    deep(n + 1).wrapping_add(frame[n as usize % 32])
}

unsafe extern "C" fn overflow_g(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    note_started();
    imm(deep(0) & 1)
}

unsafe extern "C" fn exhaust_g(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    note_started();
    let raw = 13; // KIND_RAW
    unsafe { cinch_alloc(1 << 30, raw) };
    imm(0)
}

/// `parBoth (\_ -> wait) g` where `g` must run on a worker.
fn both_with_stolen(g: Stub) {
    setup();
    CALLER.set(std::thread::current().id()).unwrap();
    let (f, g) = (static_closure(wait_for_g), static_closure(g));
    let mut out = [0i64; 2];
    unsafe { cinch_par_both(f, g, out.as_mut_ptr() as usize as i64) };
    println!("par: returned=1");
}

unsafe extern "C" fn panic_at_37(_clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    let i = unimm(unsafe { argv.read() });
    if i == 37 {
        let msg = b"branch 37 failed";
        unsafe { cinch_panic(msg.as_ptr(), msg.len()) };
    }
    imm(i)
}

static ROUND_G_ELSEWHERE: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn round_f(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    imm(u64::from(wait_flag(&ROUND_G_ELSEWHERE, Duration::from_secs(5))))
}

unsafe extern "C" fn round_g(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    if !on_caller() {
        ROUND_G_ELSEWHERE.store(true, SeqCst);
    }
    imm(0)
}

/// Rounds in which every worker has gone to sleep before the caller
/// publishes `g` and then blocks until a participant starts it: a lost
/// wakeup, or a publication withheld from idle workers, would leave the
/// job queued while the caller waits.
fn wake() {
    setup();
    CALLER.set(std::thread::current().id()).unwrap();
    let (f, g) = (static_closure(round_f), static_closure(round_g));
    let mut missed = 0;
    for _ in 0..200 {
        std::thread::sleep(Duration::from_millis(3));
        ROUND_G_ELSEWHERE.store(false, SeqCst);
        let mut out = [0i64; 2];
        unsafe { cinch_par_both(f, g, out.as_mut_ptr() as usize as i64) };
        if unimm(out[0]) == 0 {
            missed += 1;
        }
    }
    println!("par: missed={missed} slept={}", stats().slept);
}

static G2_THREAD: OnceLock<ThreadId> = OnceLock::new();
static G2_STARTED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" fn help_g(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    note_started();
    // Wait until the caller, its `f` done, waits for this job and counts
    // as idle; then fork, so the published half can go to the caller.
    let start = Instant::now();
    while stats().idle == 0 {
        if start.elapsed() > Duration::from_secs(20) {
            std::process::exit(4);
        }
        std::hint::spin_loop();
    }
    let (g1, g2) = (static_closure(help_g1), static_closure(help_g2));
    let mut out = [0i64; 2];
    unsafe { cinch_par_both(g1, g2, out.as_mut_ptr() as usize as i64) };
    imm(1)
}

unsafe extern "C" fn help_g1(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    if !wait_flag(&G2_STARTED, Duration::from_secs(20)) {
        std::process::exit(5);
    }
    imm(1)
}

unsafe extern "C" fn help_g2(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let _ = G2_THREAD.set(std::thread::current().id());
    G2_STARTED.store(true, SeqCst);
    imm(1)
}

unsafe extern "C" fn quick_f(_clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    if !wait_flag(&STARTED_ELSEWHERE, Duration::from_secs(20)) {
        std::process::exit(3);
    }
    imm(1)
}

/// A joiner whose job was stolen runs the thief's published work.
fn help() {
    setup();
    CALLER.set(std::thread::current().id()).unwrap();
    let (f, g) = (static_closure(quick_f), static_closure(help_g));
    let mut out = [0i64; 2];
    unsafe { cinch_par_both(f, g, out.as_mut_ptr() as usize as i64) };
    let on_caller = G2_THREAD.get() == CALLER.get();
    println!("par: g2_on_caller={} helped={}", u8::from(on_caller), stats().helped);
}

static SPREAD_THREADS: Mutex<Option<HashSet<ThreadId>>> = Mutex::new(None);

unsafe extern "C" fn busy_20ms(_clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(20) {
        std::hint::spin_loop();
    }
    SPREAD_THREADS.lock().unwrap().get_or_insert_with(HashSet::new).insert(std::thread::current().id());
    unsafe { argv.read() }
}

/// Sixteen 20 ms items over four participants.
fn spread() {
    setup();
    let f = static_closure(busy_20ms);
    let start = Instant::now();
    let arr = unsafe { cinch_par_tabulate(16, f, 0) };
    let ms = start.elapsed().as_millis();
    let ok = (0..16).all(|i| field(arr, 2 + i) == imm(i as u64));
    unsafe { cinch_drop(arr) };
    let threads = SPREAD_THREADS.lock().unwrap().as_ref().map_or(0, HashSet::len);
    println!("par: threads={threads} ms={ms} in_order={}", u8::from(ok));
}

unsafe extern "C" fn task_body(clo: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let list = saved(clo, 0);
    unsafe { cinch_dup(list) };
    let f = closure(CLO_LIST, tab_stub, &[list]);
    let t = unsafe { cinch_par_tabulate(2_000, f, 1) };
    let d = digest(t) & MASK40;
    unsafe {
        cinch_drop(t);
        cinch_drop(f);
        cinch_drop(clo);
    }
    ctor(OK, &[imm(d)])
}

/// A task body calls `parTabulate`; its thread participates and the call
/// returns the sequential reading.
fn task() {
    setup();
    let base = live();
    let list = build_list(LIST_LEN);
    unsafe { cinch_dup(list) };
    let body = closure(CLO_LIST, task_body, &[list]);
    let scope = cinch_scope_open();
    let handle = unsafe { cinch_task_spawn(scope, body) };
    let mut out = [0i64; 1];
    let rc = unsafe {
        cinch_task_join_all_or_cancel(scope, &handle as *const i64 as usize as i64, 1, out.as_mut_ptr() as usize as i64)
    };
    assert_eq!(rc, 0, "the task succeeded");
    let got = unimm(field(out[0], 0));
    unsafe {
        cinch_drop(out[0]);
        cinch_scope_close(scope);
    }
    unsafe { cinch_dup(list) };
    let f = closure(CLO_LIST, tab_stub, &[list]);
    let t = unsafe { cinch_coop_par_tabulate(2_000, f, 1) };
    let want = digest(t) & MASK40;
    unsafe {
        cinch_drop(t);
        cinch_drop(f);
        cinch_drop(list);
    }
    println!("par: same={} leaked={}", u8::from(got == want), live() as i64 - base as i64);
}

/// Budgets set by the program override `CINCH_THREADS`, grow and shrink
/// the pool, and never change a result.
fn budgets() {
    setup();
    let list = build_list(LIST_LEN);
    unsafe { cinch_dup(list) };
    let f = closure(CLO_LIST, tab_stub, &[list]);
    let mut seen = Vec::new();
    for n in [1, 4, 2, 0, 8, 1] {
        if n != 1 || !seen.is_empty() {
            cinch_par_set_workers(n);
        }
        let t = unsafe { cinch_par_tabulate(TAB_LEN, f, 1) };
        seen.push(format!("{n}:{}:{}", digest(t), stats().workers));
        unsafe { cinch_drop(t) };
    }
    unsafe {
        cinch_drop(f);
        cinch_drop(list);
    }
    println!("par: runs={}", seen.join(","));
}

unsafe extern "C" fn cheap(_clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    imm(unimm(unsafe { argv.read() }) * 3 + 1)
}

/// [`cheap`] as the stub of a heap closure: it consumes the closure.
unsafe extern "C" fn cheap_owned(clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    unsafe { cinch_drop(clo) };
    imm(unimm(unsafe { argv.read() }) * 3 + 1)
}

unsafe extern "C" fn expensive(_clo: i64, _argc: i64, argv: *mut i64) -> i64 {
    let mut x = unimm(unsafe { argv.read() }) | 1;
    for _ in 0..20_000_000u32 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
    }
    imm(black_box(x) & MASK40)
}

/// Best of five wall times of `parTabulate` over 1e6 cheap and 64
/// expensive items.
fn bench() {
    setup();
    let best = |n: i64, f: i64| {
        (0..5)
            .map(|_| {
                let start = Instant::now();
                let arr = unsafe { cinch_par_tabulate(n, f, 0) };
                let t = start.elapsed();
                black_box(field(arr, 2 + (n as usize - 1)));
                unsafe { cinch_drop(arr) };
                t
            })
            .min()
            .unwrap()
    };
    let cheap_t = best(1_000_000, static_closure(cheap));
    // A heap closure: marked at the first split, then counted atomically
    // by every invocation on every participant.
    let mortal = closure(CLO_NONE, cheap_owned, &[]);
    let mortal_t = best(1_000_000, mortal);
    unsafe { cinch_drop(mortal) };
    let expensive_t = best(64, static_closure(expensive));
    println!(
        "par: cheap_us={} mortal_us={} expensive_us={} workers={}",
        cheap_t.as_micros(),
        mortal_t.as_micros(),
        expensive_t.as_micros(),
        stats().workers
    );
}

// ---------------------------------------------------------------------------
// The child dispatcher and the parent side
// ---------------------------------------------------------------------------

/// Runs the scenario a parent selected; does nothing in a normal run.
#[test]
fn scenario() {
    let Ok(name) = std::env::var(SCENARIO) else {
        return;
    };
    match name.as_str() {
        "workloads" => workloads(),
        "overflow" => both_with_stolen(overflow_g),
        "exhaust" => both_with_stolen(exhaust_g),
        "panic" => {
            setup();
            unsafe { cinch_par_tabulate(64, static_closure(panic_at_37), 0) };
            println!("par: returned=1");
        }
        "wake" => wake(),
        "help" => help(),
        "spread" => spread(),
        "task" => task(),
        "budgets" => budgets(),
        "bench" => bench(),
        other => panic!("unknown scenario {other}"),
    }
}

/// Children run one at a time, so timing-sensitive ones see the machine.
static CHILDREN: Mutex<()> = Mutex::new(());

fn child(name: &str, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(std::env::current_exe().unwrap());
    cmd.args(["scenario", "--exact", "--nocapture", "--test-threads=1"]).env(SCENARIO, name);
    for var in ["CINCH_THREADS", "CINCH_RC_DEBUG", "CINCH_LEAK_CHECK", "CINCH_HEAP_LIMIT", "CINCH_STACK_SIZE"] {
        cmd.env_remove(var);
    }
    cmd.envs(env.iter().copied());
    cmd.output().unwrap()
}

/// The `par:` lines of a child's stdout as one map.
fn report(out: &Output) -> HashMap<String, String> {
    let stdout = String::from_utf8_lossy(&out.stdout);
    stdout
        .lines()
        .filter_map(|l| l.split_once("par: ").map(|(_, rest)| rest))
        .flat_map(|l| l.split(' '))
        .filter_map(|kv| kv.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn results_are_identical_at_every_worker_count_and_nothing_leaks() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let mut digests: Option<(String, String, String)> = None;
    for n in ["1", "2", "4", "8", "16"] {
        for debug in ["0", "1"] {
            let out = child("workloads", &[("CINCH_THREADS", n), ("CINCH_RC_DEBUG", debug), ("CINCH_LEAK_CHECK", "1")]);
            let r = report(&out);
            let err = stderr(&out);
            assert!(out.status.success(), "N={n} debug={debug}: {:?}\n{err}", out.status);
            assert_eq!(r["leaked"], "0", "N={n} debug={debug}: live blocks left behind");
            assert!(err.contains("leaked=0"), "N={n} debug={debug}: leak check: {err}");
            assert_eq!(r["exit_check"], "0", "N={n} debug={debug}");
            let d = (r["tab"].clone(), r["map"].clone(), r["tree"].clone());
            match &digests {
                None => digests = Some(d),
                Some(first) => assert_eq!(first, &d, "N={n} debug={debug}: results differ from N=1"),
            }
            if n == "1" {
                // One participant: plain loops, no thread, no marking.
                assert_eq!((r["workers"].as_str(), r["marked"].as_str(), r["published"].as_str()), ("0", "0", "0"));
            } else {
                assert_eq!(r["workers"], (n.parse::<usize>().unwrap() - 1).to_string(), "N={n}");
                assert_eq!(r["marked"], "1", "N={n}: publication marks the shared closure");
                assert_ne!(r["published"], "0", "N={n}");
            }
        }
    }
}

#[test]
fn a_stolen_job_that_overflows_its_stack_is_the_exhausted_exit() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("overflow", &[("CINCH_THREADS", "2"), ("CINCH_STACK_SIZE", "1M")]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(73), "{err}");
    assert!(err.contains("cinch: exhausted: stack"), "{err}");
}

#[test]
fn a_branch_that_exhausts_the_heap_or_faults_ends_the_process() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("exhaust", &[("CINCH_THREADS", "2"), ("CINCH_HEAP_LIMIT", "64M")]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(73), "{err}");
    assert!(err.contains("cinch: exhausted: heap"), "{err}");
    assert!(!report(&out).contains_key("returned"));

    let out = child("panic", &[("CINCH_THREADS", "4")]);
    let err = stderr(&out);
    assert_eq!(out.status.code(), Some(71), "{err}");
    assert!(err.contains("branch 37 failed"), "{err}");
    assert!(!report(&out).contains_key("returned"));
}

#[test]
fn sleeping_workers_wake_for_every_publication() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("wake", &[("CINCH_THREADS", "4")]);
    let r = report(&out);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(r["missed"], "0", "a published job waited beside sleeping workers");
    assert_ne!(r["slept"], "0", "the workers slept between rounds");
}

#[test]
fn a_joiner_whose_job_was_stolen_helps_the_thief() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("help", &[("CINCH_THREADS", "2")]);
    let r = report(&out);
    assert!(out.status.success(), "{:?} {}", out.status, stderr(&out));
    assert_eq!(r["g2_on_caller"], "1", "the thief's published half ran on the waiting caller");
    assert_ne!(r["helped"], "0");
}

#[test]
fn expensive_items_spread_over_every_participant() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("spread", &[("CINCH_THREADS", "4")]);
    let r = report(&out);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(r["in_order"], "1");
    assert_eq!(r["threads"], "4", "every participant ran items");
    let ms: u64 = r["ms"].parse().unwrap();
    // 320 ms of work over four participants.
    assert!(ms < 200, "sixteen 20 ms items took {ms} ms on four participants");
}

#[test]
fn a_parallel_call_from_a_task_thread_participates_and_returns() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("task", &[("CINCH_THREADS", "4"), ("CINCH_LEAK_CHECK", "1")]);
    let r = report(&out);
    assert!(out.status.success(), "{}", stderr(&out));
    assert_eq!(r["same"], "1");
    assert_eq!(r["leaked"], "0");
}

#[test]
fn the_program_budget_overrides_the_environment_and_never_changes_results() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let out = child("budgets", &[("CINCH_THREADS", "1")]);
    let r = report(&out);
    assert!(out.status.success(), "{}", stderr(&out));
    let runs: Vec<(String, String, String)> = r["runs"]
        .split(',')
        .map(|s| {
            let mut it = s.split(':').map(String::from);
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    assert!(runs.iter().all(|(_, d, _)| *d == runs[0].1), "results differ: {runs:?}");
    // Workers started so far: none at the environment's budget of 1, then
    // 3, kept (retired) at 2 and at the default of 1, then 7.
    let workers: Vec<&str> = runs.iter().map(|(_, _, w)| w.as_str()).collect();
    assert_eq!(workers, ["0", "3", "3", "3", "7", "7"], "{runs:?}");
}

/// The microbenchmark: `cargo test --release --test parallel -- --ignored
/// --nocapture parallel_tabulate_benchmark`.
#[test]
#[ignore]
fn parallel_tabulate_benchmark() {
    let _one = CHILDREN.lock().unwrap_or_else(|e| e.into_inner());
    let mut base: Option<(f64, f64, f64)> = None;
    println!("| N | 1e6 cheap, static closure (ms) | speedup | 1e6 cheap, heap closure (ms) | speedup | 64 expensive (ms) | speedup |");
    println!("| ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for n in ["1", "2", "4", "8"] {
        let out = child("bench", &[("CINCH_THREADS", n)]);
        assert!(out.status.success(), "{}", stderr(&out));
        let r = report(&out);
        let ms = |k: &str| r[k].parse::<f64>().unwrap() / 1000.0;
        let (cheap, mortal, exp) = (ms("cheap_us"), ms("mortal_us"), ms("expensive_us"));
        let (c0, m0, e0) = *base.get_or_insert((cheap, mortal, exp));
        println!(
            "| {n} | {cheap:.1} | {:.2} | {mortal:.1} | {:.2} | {exp:.1} | {:.2} |",
            c0 / cheap,
            m0 / mortal,
            e0 / exp
        );
    }
}
