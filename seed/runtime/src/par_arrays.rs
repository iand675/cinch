//! Data-parallel packed array operations: scalar reference evaluator and
//! multicore CPU execution engine with explicit numerical contracts, dynamic
//! nested-parallelism schedule selection, buffer reuse, and launch telemetry.
//!
//! Both execution engines read and write the packed `KIND_ARRAY` buffers of
//! [`crate::arrays`] directly without boxing or intermediate `List`
//! materialization. Setting `CINCH_ARRAY_EXEC=reference` (or calling
//! [`cinch_array_set_exec_mode`]) selects the scalar reference evaluator so
//! tests can verify agreement against the multicore CPU engine and Metal GPU.

use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, OnceLock};

use crate::{abi, arrays, par, rc, task};

// ---------------------------------------------------------------------------
// Constants: packed formats, reduction contracts, edge policies
// ---------------------------------------------------------------------------

pub const FMT_U8: i64 = 0x101;
pub const FMT_I32: i64 = 0x184;
pub const FMT_U32: i64 = 0x104;
pub const FMT_F32: i64 = 0x104;
pub const FMT_I64: i64 = 0x188;
pub const FMT_U64: i64 = 0x108;
pub const FMT_F64: i64 = 0x108;

pub const REDUCE_ADD_WRAP: i64 = 0;
pub const REDUCE_MUL_WRAP: i64 = 1;
pub const REDUCE_MIN: i64 = 2;
pub const REDUCE_MAX: i64 = 3;
pub const REDUCE_BIT_AND: i64 = 4;
pub const REDUCE_BIT_OR: i64 = 5;
pub const REDUCE_BIT_XOR: i64 = 6;
pub const REDUCE_F32_ADD_EXACT_TREE: i64 = 10;
pub const REDUCE_F32_ADD_RELAXED: i64 = 11;
pub const REDUCE_F64_ADD_EXACT_TREE: i64 = 12;
pub const REDUCE_F64_ADD_RELAXED: i64 = 13;
pub const REDUCE_F32_MIN: i64 = 14;
pub const REDUCE_F32_MAX: i64 = 15;
pub const REDUCE_F64_MIN: i64 = 16;
pub const REDUCE_F64_MAX: i64 = 17;

pub const EDGE_CLAMP: i64 = 0;
pub const EDGE_ZERO: i64 = 1;
pub const EDGE_MIRROR: i64 = 2;
pub const EDGE_WRAP: i64 = 3;

pub const BINOP_ADD: i64 = 0;
pub const BINOP_SUB: i64 = 1;
pub const BINOP_MUL: i64 = 2;
pub const BINOP_DIV: i64 = 3;

pub const UNARY_ID: i64 = 0;
pub const UNARY_ADD_CONST: i64 = 1;
pub const UNARY_MUL_CONST: i64 = 2;
pub const UNARY_SQUARE: i64 = 3;
pub const UNARY_ABS: i64 = 4;
pub const UNARY_NEG: i64 = 5;

// ---------------------------------------------------------------------------
// Execution mode & telemetry
// ---------------------------------------------------------------------------

/// -1 = unset (check env), 0 = multicore CPU, 1 = scalar reference.
static EXEC_MODE: AtomicI64 = AtomicI64::new(-1);

pub fn is_reference_mode() -> bool {
    let m = EXEC_MODE.load(Ordering::Relaxed);
    if m >= 0 {
        return m == 1;
    }
    let from_env = crate::heap::env_bytes(c"CINCH_ARRAY_EXEC")
        .is_some_and(|v| v == b"reference" || v == b"scalar");
    EXEC_MODE.store(i64::from(from_env), Ordering::Relaxed);
    from_env
}

#[unsafe(no_mangle)]
pub extern "C" fn cinch_array_set_exec_mode(mode: i64) -> i64 {
    EXEC_MODE.store(if mode == 1 { 1 } else { 0 }, Ordering::Relaxed);
    0
}

#[derive(Clone, Debug)]
pub struct LaunchReport {
    pub backend: &'static str,
    pub op: String,
    pub schedule: String,
    pub numerical_mode: &'static str,
    pub kernel_launches: usize,
    pub peak_temp_bytes: usize,
    pub reused_in_place: bool,
    pub threads: usize,
}

impl Default for LaunchReport {
    fn default() -> Self {
        Self {
            backend: "cpu",
            op: String::new(),
            schedule: "scalar".to_string(),
            numerical_mode: "exact",
            kernel_launches: 0,
            peak_temp_bytes: 0,
            reused_in_place: false,
            threads: 1,
        }
    }
}

impl LaunchReport {
    pub fn to_json(&self) -> String {
        format!(
            concat!(
                "{{\"backend\":\"{}\",\"op\":\"{}\",\"schedule\":\"{}\",",
                "\"numerical_mode\":\"{}\",\"kernel_launches\":{},",
                "\"peak_temp_bytes\":{},\"reused_in_place\":{},\"threads\":{}}}"
            ),
            self.backend,
            self.op,
            self.schedule,
            self.numerical_mode,
            self.kernel_launches,
            self.peak_temp_bytes,
            self.reused_in_place,
            self.threads,
        )
    }
}

static LAST_REPORT: OnceLock<Mutex<LaunchReport>> = OnceLock::new();

pub fn record_report(report: LaunchReport) {
    let lock = LAST_REPORT.get_or_init(|| Mutex::new(LaunchReport::default()));
    if let Ok(mut g) = lock.lock() {
        *g = report;
    }
}

pub fn last_report() -> LaunchReport {
    let lock = LAST_REPORT.get_or_init(|| Mutex::new(LaunchReport::default()));
    lock.lock().map(|g| g.clone()).unwrap_or_default()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_last_report_bytes() -> i64 {
    let json = last_report().to_json();
    let bytes = json.as_bytes();
    unsafe { abi::cinch_bytes_from_data(bytes.as_ptr(), bytes.len() as i64) }
}

pub fn reduce_mode_name(op: i64) -> &'static str {
    match op {
        REDUCE_ADD_WRAP => "wrapping-add",
        REDUCE_MUL_WRAP => "wrapping-mul",
        REDUCE_MIN => "min",
        REDUCE_MAX => "max",
        REDUCE_BIT_AND => "bit-and",
        REDUCE_BIT_OR => "bit-or",
        REDUCE_BIT_XOR => "bit-xor",
        REDUCE_F32_ADD_EXACT_TREE => "f32-add-exact-tree",
        REDUCE_F32_ADD_RELAXED => "f32-add-relaxed",
        REDUCE_F64_ADD_EXACT_TREE => "f64-add-exact-tree",
        REDUCE_F64_ADD_RELAXED => "f64-add-relaxed",
        REDUCE_F32_MIN => "f32-min",
        REDUCE_F32_MAX => "f32-max",
        REDUCE_F64_MIN => "f64-min",
        REDUCE_F64_MAX => "f64-max",
        _ => "custom",
    }
}

// ---------------------------------------------------------------------------
// Low-latency persistent parallel-for pool (zero OS thread spawn per kernel)
// ---------------------------------------------------------------------------

struct JobFn {
    func: fn(usize, usize, usize, usize), // (tid, num_threads, env_ptr, total)
    env: usize,
    total: usize,
    active_threads: usize,
}

struct PoolState {
    generation: u64,
    job: Option<JobFn>,
}

struct FastPool {
    max_workers: usize,
    state: Mutex<PoolState>,
    wake: Condvar,
    done_count: AtomicUsize,
    done_lock: Mutex<()>,
    done_cv: Condvar,
}

struct PoolHandle(&'static FastPool);

static POOL_HANDLE: OnceLock<PoolHandle> = OnceLock::new();

fn pool() -> &'static FastPool {
    POOL_HANDLE
        .get_or_init(|| {
            let hw = std::thread::available_parallelism()
                .map(usize::from)
                .unwrap_or(1)
                .clamp(1, 64);
            let pool: &'static FastPool = Box::leak(Box::new(FastPool {
                max_workers: hw,
                state: Mutex::new(PoolState {
                    generation: 0,
                    job: None,
                }),
                wake: Condvar::new(),
                done_count: AtomicUsize::new(0),
                done_lock: Mutex::new(()),
                done_cv: Condvar::new(),
            }));
            for tid in 1..hw {
                let _ = std::thread::Builder::new()
                    .name(format!("cinch-arr-{tid}"))
                    .spawn(move || worker_loop(pool, tid));
            }
            PoolHandle(pool)
        })
        .0
}

fn worker_loop(p: &'static FastPool, tid: usize) {
    let mut seen_gen = 0u64;
    loop {
        let job = {
            let mut st = p.state.lock().unwrap_or_else(|e| e.into_inner());
            while st.generation == seen_gen {
                st = p.wake.wait(st).unwrap_or_else(|e| e.into_inner());
            }
            seen_gen = st.generation;
            st.job.as_ref().map(|j| (j.func, j.env, j.total, j.active_threads))
        };
        if let Some((func, env, total, active_threads)) = job {
            if tid < active_threads {
                func(tid, active_threads, env, total);
                let prev = p.done_count.fetch_add(1, Ordering::AcqRel);
                if prev + 1 == active_threads - 1 {
                    let _g = p.done_lock.lock().unwrap_or_else(|e| e.into_inner());
                    p.done_cv.notify_one();
                }
            }
        }
    }
}

/// Active thread budget for array kernels (respects `parWorkers` / `CINCH_THREADS`,
/// and returns 1 in scalar reference mode).
pub fn active_threads(work_items: usize, min_per_thread: usize) -> usize {
    if is_reference_mode() || work_items < min_per_thread * 2 {
        return 1;
    }
    let budget = par::budget().clamp(1, pool().max_workers);
    let needed = (work_items + min_per_thread - 1) / min_per_thread;
    budget.min(needed).max(1)
}

/// Serialize concurrent top-level dispatches so nested or multi-caller usage
/// never races on the shared pool slot.
static DISPATCH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Run `f(tid, num_threads, start, end)` across `num_threads` threads partitioning `0..total`.
pub fn parallel_chunks<F>(total: usize, min_per_thread: usize, f: F) -> usize
where
    F: Fn(usize, usize, usize, usize) + Sync,
{
    let nthreads = active_threads(total, min_per_thread);
    if nthreads <= 1 {
        f(0, 1, 0, total);
        return 1;
    }
    let _dispatch = DISPATCH_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let p = pool();
    fn trampoline<F: Fn(usize, usize, usize, usize) + Sync>(
        tid: usize,
        nt: usize,
        env: usize,
        total: usize,
    ) {
        let f = unsafe { &*(env as *const F) };
        let chunk = (total + nt - 1) / nt;
        let start = (tid * chunk).min(total);
        let end = (start + chunk).min(total);
        f(tid, nt, start, end);
    }
    p.done_count.store(0, Ordering::Release);
    {
        let mut st = p.state.lock().unwrap_or_else(|e| e.into_inner());
        st.job = Some(JobFn {
            func: trampoline::<F>,
            env: (&f as *const F) as usize,
            total,
            active_threads: nthreads,
        });
        st.generation += 1;
        p.wake.notify_all();
    }
    // Main thread executes tid = 0
    trampoline::<F>(0, nthreads, (&f as *const F) as usize, total);
    // Spin briefly before parking on condvar
    let mut spins = 0u32;
    while p.done_count.load(Ordering::Acquire) < nthreads - 1 {
        if spins < 256 {
            std::hint::spin_loop();
            spins += 1;
        } else {
            let g = p.done_lock.lock().unwrap_or_else(|e| e.into_inner());
            if p.done_count.load(Ordering::Acquire) < nthreads - 1 {
                let _g = p.done_cv.wait(g);
            }
        }
    }
    nthreads
}

// ---------------------------------------------------------------------------
// Packed slice views over Cinch `KIND_ARRAY` blocks
// ---------------------------------------------------------------------------

#[inline]
pub(crate) unsafe fn slice_u8<'a>(a: i64) -> &'a [u8] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a), n) }
}

#[inline]
pub(crate) unsafe fn slice_u8_mut<'a>(a: i64) -> &'a mut [u8] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(arrays::data_ptr(a), n) }
}

#[inline]
pub(crate) unsafe fn slice_i32<'a>(a: i64) -> &'a [i32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a).cast::<i32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_i32_mut<'a>(a: i64) -> &'a mut [i32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(arrays::data_ptr(a).cast::<i32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_u32<'a>(a: i64) -> &'a [u32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a).cast::<u32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_u32_mut<'a>(a: i64) -> &'a mut [u32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(arrays::data_ptr(a).cast::<u32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_f32<'a>(a: i64) -> &'a [f32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a).cast::<f32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_f32_mut<'a>(a: i64) -> &'a mut [f32] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(arrays::data_ptr(a).cast::<f32>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_f64<'a>(a: i64) -> &'a [f64] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a).cast::<f64>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_f64_mut<'a>(a: i64) -> &'a mut [f64] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &mut [];
    }
    unsafe { std::slice::from_raw_parts_mut(arrays::data_ptr(a).cast::<f64>(), n) }
}

#[inline]
pub(crate) unsafe fn slice_i64<'a>(a: i64) -> &'a [i64] {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    if n == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(arrays::data_ptr(a).cast::<i64>(), n) }
}

// ---------------------------------------------------------------------------
// Deterministic binary tree float reduction (exact across ref / CPU / Metal)
// ---------------------------------------------------------------------------

/// Specified deterministic binary tree sum over `F32`:
/// recursively halves `[lo..hi)` at `mid = lo + floor(len/2)` (or equivalently
/// power-of-2 left-complete tree: largest power of two `< len`), ensuring
/// identical rounding tree across scalar reference, multicore CPU, and GPU!
#[inline]
pub fn f32_exact_tree_sum(xs: &[f32]) -> f32 {
    let n = xs.len();
    if n == 0 {
        return 0.0f32;
    }
    if n == 1 {
        return xs[0];
    }
    if n == 2 {
        return xs[0] + xs[1];
    }
    // Split at largest power of 2 strictly less than n so power-of-2 subtrees
    // match GPU threadgroup pairwise binary tree reduction bit-for-bit!
    let split = (n - 1).next_power_of_two() >> 1;
    let split = if split == 0 { n / 2 } else { split };
    f32_exact_tree_sum(&xs[..split]) + f32_exact_tree_sum(&xs[split..])
}

#[inline]
pub fn f64_exact_tree_sum(xs: &[f64]) -> f64 {
    let n = xs.len();
    if n == 0 {
        return 0.0f64;
    }
    if n == 1 {
        return xs[0];
    }
    if n == 2 {
        return xs[0] + xs[1];
    }
    let split = (n - 1).next_power_of_two() >> 1;
    let split = if split == 0 { n / 2 } else { split };
    f64_exact_tree_sum(&xs[..split]) + f64_exact_tree_sum(&xs[split..])
}

/// Exact pairwise tree sum over a virtual producer `f(i)` for `i in lo..hi`.
pub fn f32_exact_tree_prod(lo: usize, hi: usize, f: &impl Fn(usize) -> f32) -> f32 {
    let n = hi - lo;
    if n == 0 {
        return 0.0f32;
    }
    if n == 1 {
        return f(lo);
    }
    if n == 2 {
        return f(lo) + f(lo + 1);
    }
    let split = (n - 1).next_power_of_two() >> 1;
    let split = if split == 0 { n / 2 } else { split };
    f32_exact_tree_prod(lo, lo + split, f) + f32_exact_tree_prod(lo + split, hi, f)
}

pub fn f64_exact_tree_prod(lo: usize, hi: usize, f: &impl Fn(usize) -> f64) -> f64 {
    let n = hi - lo;
    if n == 0 {
        return 0.0f64;
    }
    if n == 1 {
        return f(lo);
    }
    if n == 2 {
        return f(lo) + f(lo + 1);
    }
    let split = (n - 1).next_power_of_two() >> 1;
    let split = if split == 0 { n / 2 } else { split };
    f64_exact_tree_prod(lo, lo + split, f) + f64_exact_tree_prod(lo + split, hi, f)
}

/// Parallel exact-tree reduction over `0..n` using 1024-element power-of-2 leaves
/// so the binary tree shape is 100% invariant to thread count!
fn parallel_f32_exact_tree<F: Fn(usize) -> f32 + Sync>(n: usize, f: F) -> (f32, usize) {
    if n == 0 {
        return (0.0, 1);
    }
    if is_reference_mode() || n <= 8192 {
        return (f32_exact_tree_prod(0, n, &f), 1);
    }
    const BLOCK: usize = 1024;
    let num_blocks = (n + BLOCK - 1) / BLOCK;
    let mut partials = vec![0.0f32; num_blocks];
    let p_ptr = partials.as_mut_ptr() as usize;
    let threads = parallel_chunks(num_blocks, 4, |_, _, b_lo, b_hi| {
        let out = unsafe { std::slice::from_raw_parts_mut(p_ptr as *mut f32, num_blocks) };
        for b in b_lo..b_hi {
            let lo = b * BLOCK;
            let hi = (lo + BLOCK).min(n);
            out[b] = f32_exact_tree_prod(lo, hi, &f);
        }
    });
    // Reduce block results with the exact same power-of-2 tree structure.
    // Because BLOCK = 1024 is a power of 2, splitting at the largest power of 2 < num_blocks
    // preserves the exact same binary tree as `f32_exact_tree_prod(0, n, &f)`!
    (f32_exact_tree_sum(&partials), threads)
}

fn parallel_f64_exact_tree<F: Fn(usize) -> f64 + Sync>(n: usize, f: F) -> (f64, usize) {
    if n == 0 {
        return (0.0, 1);
    }
    if is_reference_mode() || n <= 8192 {
        return (f64_exact_tree_prod(0, n, &f), 1);
    }
    const BLOCK: usize = 1024;
    let num_blocks = (n + BLOCK - 1) / BLOCK;
    let mut partials = vec![0.0f64; num_blocks];
    let p_ptr = partials.as_mut_ptr() as usize;
    let threads = parallel_chunks(num_blocks, 4, |_, _, b_lo, b_hi| {
        let out = unsafe { std::slice::from_raw_parts_mut(p_ptr as *mut f64, num_blocks) };
        for b in b_lo..b_hi {
            let lo = b * BLOCK;
            let hi = (lo + BLOCK).min(n);
            out[b] = f64_exact_tree_prod(lo, hi, &f);
        }
    });
    (f64_exact_tree_sum(&partials), threads)
}

// ---------------------------------------------------------------------------
// Direct packed tabulation, map, zip-with, ordered foldl (closure-based)
// ---------------------------------------------------------------------------

#[inline]
unsafe fn invoke_closure(f: i64, arg: i64) -> i64 {
    if rc::reclaiming() {
        unsafe { rc::cinch_dup(f) };
    }
    unsafe { task::apply_closure(f, arg) }
}

/// Direct packed tabulation: allocates ONE packed `Array` and populates it
/// directly (no intermediate `List`!), running across threads when `n` is large.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_tabulate_packed(n: i64, f: i64, format: i64) -> i64 {
    if n < 0 {
        abi::panic_exit(b"cinch_array_tabulate_packed: negative length");
    }
    let out = if format & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(n, format) }
    } else {
        unsafe { arrays::array(n, format) }
    };
    let len = n as usize;
    let mode_ref = is_reference_mode();
    let threads = if mode_ref || len < 4096 {
        for i in 0..len {
            let idx = abi::cinch_nat_from_i64(i as i64);
            let v = unsafe { invoke_closure(f, idx) };
            unsafe { arrays::write(out, i as i64, v) };
        }
        1
    } else {
        if rc::reclaiming() {
            unsafe { rc::cinch_mark_mt(f) };
        }
        parallel_chunks(len, 2048, |_, _, lo, hi| {
            for i in lo..hi {
                let idx = abi::cinch_nat_from_i64(i as i64);
                let v = unsafe { invoke_closure(f, idx) };
                unsafe { arrays::write(out, i as i64, v) };
            }
        })
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "tabulate-packed".to_string(),
        schedule: format!("1d-parallel(n={len},threads={threads})"),
        numerical_mode: "exact",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Direct packed map over an `Array` using a Cinch closure.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_map_packed(f: i64, a: i64, out_format: i64) -> i64 {
    let n = unsafe { arrays::cinch_array_len(a) };
    let out = if out_format & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(n, out_format) }
    } else {
        unsafe { arrays::array(n, out_format) }
    };
    let len = n as usize;
    let in_fmt = unsafe { arrays::format(a) };
    let in_counted = arrays::counted(in_fmt);
    let mode_ref = is_reference_mode();
    let threads = if mode_ref || len < 4096 {
        for i in 0..len {
            let x = unsafe { arrays::read(a, i as i64) };
            if in_counted {
                unsafe { rc::cinch_dup(x) };
            }
            let y = unsafe { invoke_closure(f, x) };
            unsafe { arrays::write(out, i as i64, y) };
        }
        1
    } else {
        if rc::reclaiming() {
            unsafe {
                rc::cinch_mark_mt(f);
                rc::cinch_mark_mt(a);
            }
        }
        parallel_chunks(len, 2048, |_, _, lo, hi| {
            for i in lo..hi {
                let x = unsafe { arrays::read(a, i as i64) };
                if in_counted {
                    unsafe { rc::cinch_dup(x) };
                }
                let y = unsafe { invoke_closure(f, x) };
                unsafe { arrays::write(out, i as i64, y) };
            }
        })
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "map-packed".to_string(),
        schedule: format!("1d-parallel(n={len},threads={threads})"),
        numerical_mode: "exact",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Direct packed `zip-with` over two equal-length arrays (returns 0 on length mismatch).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_zip_with_packed(
    f: i64,
    a: i64,
    b: i64,
    out_format: i64,
) -> i64 {
    let na = unsafe { arrays::cinch_array_len(a) };
    let nb = unsafe { arrays::cinch_array_len(b) };
    if na != nb {
        return 0;
    }
    let out = if out_format & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(na, out_format) }
    } else {
        unsafe { arrays::array(na, out_format) }
    };
    let len = na as usize;
    let a_counted = arrays::counted(unsafe { arrays::format(a) });
    let b_counted = arrays::counted(unsafe { arrays::format(b) });
    let mode_ref = is_reference_mode();
    let threads = if mode_ref || len < 4096 {
        for i in 0..len {
            let x = unsafe { arrays::read(a, i as i64) };
            let y = unsafe { arrays::read(b, i as i64) };
            if a_counted {
                unsafe { rc::cinch_dup(x) };
            }
            if b_counted {
                unsafe { rc::cinch_dup(y) };
            }
            let partial = unsafe { invoke_closure(f, x) };
            let res = unsafe { task::apply_closure(partial, y) };
            unsafe { arrays::write(out, i as i64, res) };
        }
        1
    } else {
        if rc::reclaiming() {
            unsafe {
                rc::cinch_mark_mt(f);
                rc::cinch_mark_mt(a);
                rc::cinch_mark_mt(b);
            }
        }
        parallel_chunks(len, 2048, |_, _, lo, hi| {
            for i in lo..hi {
                let x = unsafe { arrays::read(a, i as i64) };
                let y = unsafe { arrays::read(b, i as i64) };
                if a_counted {
                    unsafe { rc::cinch_dup(x) };
                }
                if b_counted {
                    unsafe { rc::cinch_dup(y) };
                }
                let partial = unsafe { invoke_closure(f, x) };
                let res = unsafe { task::apply_closure(partial, y) };
                unsafe { arrays::write(out, i as i64, res) };
            }
        })
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "zip-with-packed".to_string(),
        schedule: format!("1d-parallel(n={len},threads={threads})"),
        numerical_mode: "exact",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Strict left-to-right ordered fold (distinct from reassociative `reduce`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_foldl_packed(f: i64, init: i64, a: i64) -> i64 {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    let in_counted = arrays::counted(unsafe { arrays::format(a) });
    let mut acc = init;
    for i in 0..n {
        let x = unsafe { arrays::read(a, i as i64) };
        if in_counted {
            unsafe { rc::cinch_dup(x) };
        }
        let partial = unsafe { invoke_closure(f, acc) };
        acc = unsafe { task::apply_closure(partial, x) };
    }
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "foldl-ordered".to_string(),
        schedule: format!("sequential-ordered(n={n})"),
        numerical_mode: "ordered-left-fold",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads: 1,
    });
    acc
}

// ---------------------------------------------------------------------------
// Scalar reductions with explicit numerical contracts
// ---------------------------------------------------------------------------

#[inline]
fn i32_identity(op: i64) -> i32 {
    match op {
        REDUCE_ADD_WRAP | REDUCE_BIT_OR | REDUCE_BIT_XOR => 0,
        REDUCE_MUL_WRAP => 1,
        REDUCE_MIN => i32::MAX,
        REDUCE_MAX => i32::MIN,
        REDUCE_BIT_AND => -1,
        _ => 0,
    }
}

#[inline]
fn i32_combine(op: i64, a: i32, b: i32) -> i32 {
    match op {
        REDUCE_ADD_WRAP => a.wrapping_add(b),
        REDUCE_MUL_WRAP => a.wrapping_mul(b),
        REDUCE_MIN => a.min(b),
        REDUCE_MAX => a.max(b),
        REDUCE_BIT_AND => a & b,
        REDUCE_BIT_OR => a | b,
        REDUCE_BIT_XOR => a ^ b,
        _ => a.wrapping_add(b),
    }
}

#[inline]
fn u32_identity(op: i64) -> u32 {
    match op {
        REDUCE_ADD_WRAP | REDUCE_BIT_OR | REDUCE_BIT_XOR | REDUCE_MAX => 0,
        REDUCE_MUL_WRAP => 1,
        REDUCE_MIN | REDUCE_BIT_AND => u32::MAX,
        _ => 0,
    }
}

#[inline]
fn u32_combine(op: i64, a: u32, b: u32) -> u32 {
    match op {
        REDUCE_ADD_WRAP => a.wrapping_add(b),
        REDUCE_MUL_WRAP => a.wrapping_mul(b),
        REDUCE_MIN => a.min(b),
        REDUCE_MAX => a.max(b),
        REDUCE_BIT_AND => a & b,
        REDUCE_BIT_OR => a | b,
        REDUCE_BIT_XOR => a ^ b,
        _ => a.wrapping_add(b),
    }
}

#[inline]
fn i64_identity(op: i64) -> i64 {
    match op {
        REDUCE_ADD_WRAP | REDUCE_BIT_OR | REDUCE_BIT_XOR => 0,
        REDUCE_MUL_WRAP => 1,
        REDUCE_MIN => i64::MAX,
        REDUCE_MAX => i64::MIN,
        REDUCE_BIT_AND => -1,
        _ => 0,
    }
}

#[inline]
fn i64_combine(op: i64, a: i64, b: i64) -> i64 {
    match op {
        REDUCE_ADD_WRAP => a.wrapping_add(b),
        REDUCE_MUL_WRAP => a.wrapping_mul(b),
        REDUCE_MIN => a.min(b),
        REDUCE_MAX => a.max(b),
        REDUCE_BIT_AND => a & b,
        REDUCE_BIT_OR => a | b,
        REDUCE_BIT_XOR => a ^ b,
        _ => a.wrapping_add(b),
    }
}

fn reduce_f32_slice(xs: &[f32], op: i64) -> (f32, usize) {
    let n = xs.len();
    if n == 0 {
        let id = match op {
            REDUCE_F32_MIN => f32::INFINITY,
            REDUCE_F32_MAX => f32::NEG_INFINITY,
            _ => 0.0f32,
        };
        return (id, 1);
    }
    if op == REDUCE_F32_ADD_EXACT_TREE {
        return parallel_f32_exact_tree(n, |i| xs[i]);
    }
    if is_reference_mode() || n < 16384 {
        let val = match op {
            REDUCE_F32_ADD_RELAXED => {
                let mut a0 = 0.0f32;
                let mut a1 = 0.0f32;
                let mut a2 = 0.0f32;
                let mut a3 = 0.0f32;
                let mut i = 0;
                while i + 4 <= n {
                    a0 += xs[i];
                    a1 += xs[i + 1];
                    a2 += xs[i + 2];
                    a3 += xs[i + 3];
                    i += 4;
                }
                let mut s = (a0 + a1) + (a2 + a3);
                while i < n {
                    s += xs[i];
                    i += 1;
                }
                s
            }
            REDUCE_F32_MIN => xs.iter().copied().fold(f32::INFINITY, f32::min),
            REDUCE_F32_MAX => xs.iter().copied().fold(f32::NEG_INFINITY, f32::max),
            _ => f32_exact_tree_sum(xs),
        };
        return (val, 1);
    }
    let mut partials = [0.0f32; 64];
    let p_ptr = partials.as_mut_ptr() as usize;
    let threads = parallel_chunks(n, 8192, |tid, _, lo, hi| {
        let chunk = &xs[lo..hi];
        let v = match op {
            REDUCE_F32_ADD_RELAXED => {
                let mut a0 = 0.0f32;
                let mut a1 = 0.0f32;
                let mut a2 = 0.0f32;
                let mut a3 = 0.0f32;
                let mut i = 0;
                while i + 4 <= chunk.len() {
                    a0 += chunk[i];
                    a1 += chunk[i + 1];
                    a2 += chunk[i + 2];
                    a3 += chunk[i + 3];
                    i += 4;
                }
                let mut s = (a0 + a1) + (a2 + a3);
                while i < chunk.len() {
                    s += chunk[i];
                    i += 1;
                }
                s
            }
            REDUCE_F32_MIN => chunk.iter().copied().fold(f32::INFINITY, f32::min),
            REDUCE_F32_MAX => chunk.iter().copied().fold(f32::NEG_INFINITY, f32::max),
            _ => f32_exact_tree_sum(chunk),
        };
        unsafe { *(p_ptr as *mut f32).add(tid) = v };
    });
    let res = match op {
        REDUCE_F32_MIN => partials[..threads].iter().copied().fold(f32::INFINITY, f32::min),
        REDUCE_F32_MAX => partials[..threads].iter().copied().fold(f32::NEG_INFINITY, f32::max),
        _ => partials[..threads].iter().copied().sum(),
    };
    (res, threads)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_reduce_i32(a: i64, op: i64) -> i64 {
    let xs = unsafe { slice_i32(a) };
    let n = xs.len();
    let mode_ref = is_reference_mode();
    let (res, threads) = if mode_ref || n < 16384 {
        let mut acc = i32_identity(op);
        for &x in xs {
            acc = i32_combine(op, acc, x);
        }
        (acc, 1)
    } else {
        let mut partials = [i32_identity(op); 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut acc = i32_identity(op);
            for &x in &xs[lo..hi] {
                acc = i32_combine(op, acc, x);
            }
            unsafe { *(p_ptr as *mut i32).add(tid) = acc };
        });
        let mut acc = i32_identity(op);
        for &p in &partials[..t] {
            acc = i32_combine(op, acc, p);
        }
        (acc, t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "reduce-i32".to_string(),
        schedule: format!("1d-reduce(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: 1,
        peak_temp_bytes: threads * 4,
        reused_in_place: false,
        threads,
    });
    i64::from(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_reduce_u32(a: i64, op: i64) -> i64 {
    let xs = unsafe { slice_u32(a) };
    let n = xs.len();
    let mode_ref = is_reference_mode();
    let (res, threads) = if mode_ref || n < 16384 {
        let mut acc = u32_identity(op);
        for &x in xs {
            acc = u32_combine(op, acc, x);
        }
        (acc, 1)
    } else {
        let mut partials = [u32_identity(op); 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut acc = u32_identity(op);
            for &x in &xs[lo..hi] {
                acc = u32_combine(op, acc, x);
            }
            unsafe { *(p_ptr as *mut u32).add(tid) = acc };
        });
        let mut acc = u32_identity(op);
        for &p in &partials[..t] {
            acc = u32_combine(op, acc, p);
        }
        (acc, t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "reduce-u32".to_string(),
        schedule: format!("1d-reduce(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: 1,
        peak_temp_bytes: threads * 4,
        reused_in_place: false,
        threads,
    });
    i64::from(res)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_reduce_i64(a: i64, op: i64) -> i64 {
    let xs = unsafe { slice_i64(a) };
    let n = xs.len();
    let mode_ref = is_reference_mode();
    let (res, threads) = if mode_ref || n < 16384 {
        let mut acc = i64_identity(op);
        for &x in xs {
            acc = i64_combine(op, acc, x);
        }
        (acc, 1)
    } else {
        let mut partials = [i64_identity(op); 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut acc = i64_identity(op);
            for &x in &xs[lo..hi] {
                acc = i64_combine(op, acc, x);
            }
            unsafe { *(p_ptr as *mut i64).add(tid) = acc };
        });
        let mut acc = i64_identity(op);
        for &p in &partials[..t] {
            acc = i64_combine(op, acc, p);
        }
        (acc, t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "reduce-i64".to_string(),
        schedule: format!("1d-reduce(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: 1,
        peak_temp_bytes: threads * 8,
        reused_in_place: false,
        threads,
    });
    res
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_reduce_f32(a: i64, op: i64) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let (res, threads) = reduce_f32_slice(xs, op);
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "reduce-f32".to_string(),
        schedule: format!("1d-reduce(n={},threads={threads})", xs.len()),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: 1,
        peak_temp_bytes: threads * 4,
        reused_in_place: false,
        threads,
    });
    i64::from(res.to_bits())
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_reduce_f64(a: i64, op: i64) -> i64 {
    let xs = unsafe { slice_f64(a) };
    let n = xs.len();
    let mode_ref = is_reference_mode();
    let (res, threads) = if n == 0 {
        let id = match op {
            REDUCE_F64_MIN => f64::INFINITY,
            REDUCE_F64_MAX => f64::NEG_INFINITY,
            _ => 0.0f64,
        };
        (id, 1)
    } else if op == REDUCE_F64_ADD_EXACT_TREE {
        parallel_f64_exact_tree(n, |i| xs[i])
    } else if mode_ref || n < 16384 {
        let v = match op {
            REDUCE_F64_MIN => xs.iter().copied().fold(f64::INFINITY, f64::min),
            REDUCE_F64_MAX => xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
            _ => {
                let mut a0 = 0.0f64;
                let mut a1 = 0.0f64;
                let mut a2 = 0.0f64;
                let mut a3 = 0.0f64;
                let mut i = 0;
                while i + 4 <= n {
                    a0 += xs[i];
                    a1 += xs[i + 1];
                    a2 += xs[i + 2];
                    a3 += xs[i + 3];
                    i += 4;
                }
                let mut s = (a0 + a1) + (a2 + a3);
                while i < n {
                    s += xs[i];
                    i += 1;
                }
                s
            }
        };
        (v, 1)
    } else {
        let mut partials = [0.0f64; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let chunk = &xs[lo..hi];
            let v = match op {
                REDUCE_F64_MIN => chunk.iter().copied().fold(f64::INFINITY, f64::min),
                REDUCE_F64_MAX => chunk.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                _ => {
                    let mut a0 = 0.0f64;
                    let mut a1 = 0.0f64;
                    let mut a2 = 0.0f64;
                    let mut a3 = 0.0f64;
                    let mut i = 0;
                    while i + 4 <= chunk.len() {
                        a0 += chunk[i];
                        a1 += chunk[i + 1];
                        a2 += chunk[i + 2];
                        a3 += chunk[i + 3];
                        i += 4;
                    }
                    let mut s = (a0 + a1) + (a2 + a3);
                    while i < chunk.len() {
                        s += chunk[i];
                        i += 1;
                    }
                    s
                }
            };
            unsafe { *(p_ptr as *mut f64).add(tid) = v };
        });
        let v = match op {
            REDUCE_F64_MIN => partials[..t].iter().copied().fold(f64::INFINITY, f64::min),
            REDUCE_F64_MAX => partials[..t].iter().copied().fold(f64::NEG_INFINITY, f64::max),
            _ => partials[..t].iter().copied().sum(),
        };
        (v, t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "reduce-f64".to_string(),
        schedule: format!("1d-reduce(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: 1,
        peak_temp_bytes: threads * 8,
        reused_in_place: false,
        threads,
    });
    res.to_bits() as i64
}

// ---------------------------------------------------------------------------
// Fused producer-consumer kernels (zero intermediate allocation!)
// ---------------------------------------------------------------------------

/// Fused dot product over packed `Array F32`: allocates ZERO product vector.
/// `float_mode`: 0 = `F32AddExactTree`, 1 = `F32AddRelaxed`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_dot_f32(a: i64, b: i64, float_mode: i64) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let ys = unsafe { slice_f32(b) };
    let n = xs.len().min(ys.len());
    let mode_ref = is_reference_mode();
    let exact = float_mode == 0 || float_mode == REDUCE_F32_ADD_EXACT_TREE;
    let (res, threads) = if n == 0 {
        (0.0f32, 1)
    } else if exact {
        parallel_f32_exact_tree(n, |i| xs[i] * ys[i])
    } else if mode_ref || n < 16384 {
        let mut a0 = 0.0f32;
        let mut a1 = 0.0f32;
        let mut a2 = 0.0f32;
        let mut a3 = 0.0f32;
        let mut a4 = 0.0f32;
        let mut a5 = 0.0f32;
        let mut a6 = 0.0f32;
        let mut a7 = 0.0f32;
        let mut i = 0;
        while i + 8 <= n {
            a0 += xs[i] * ys[i];
            a1 += xs[i + 1] * ys[i + 1];
            a2 += xs[i + 2] * ys[i + 2];
            a3 += xs[i + 3] * ys[i + 3];
            a4 += xs[i + 4] * ys[i + 4];
            a5 += xs[i + 5] * ys[i + 5];
            a6 += xs[i + 6] * ys[i + 6];
            a7 += xs[i + 7] * ys[i + 7];
            i += 8;
        }
        let mut s = ((a0 + a1) + (a2 + a3)) + ((a4 + a5) + (a6 + a7));
        while i < n {
            s += xs[i] * ys[i];
            i += 1;
        }
        (s, 1)
    } else {
        let mut partials = [0.0f32; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut a0 = 0.0f32;
            let mut a1 = 0.0f32;
            let mut a2 = 0.0f32;
            let mut a3 = 0.0f32;
            let mut a4 = 0.0f32;
            let mut a5 = 0.0f32;
            let mut a6 = 0.0f32;
            let mut a7 = 0.0f32;
            let mut i = lo;
            while i + 8 <= hi {
                a0 += xs[i] * ys[i];
                a1 += xs[i + 1] * ys[i + 1];
                a2 += xs[i + 2] * ys[i + 2];
                a3 += xs[i + 3] * ys[i + 3];
                a4 += xs[i + 4] * ys[i + 4];
                a5 += xs[i + 5] * ys[i + 5];
                a6 += xs[i + 6] * ys[i + 6];
                a7 += xs[i + 7] * ys[i + 7];
                i += 8;
            }
            let mut s = ((a0 + a1) + (a2 + a3)) + ((a4 + a5) + (a6 + a7));
            while i < hi {
                s += xs[i] * ys[i];
                i += 1;
            }
            unsafe { *(p_ptr as *mut f32).add(tid) = s };
        });
        (partials[..t].iter().copied().sum(), t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "fused-dot-f32".to_string(),
        schedule: format!("fused-zip-reduce(n={n},threads={threads})"),
        numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" },
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    i64::from(res.to_bits())
}

/// Fused dot product over packed `Array F64`: allocates ZERO product vector.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_dot_f64(a: i64, b: i64, float_mode: i64) -> i64 {
    let xs = unsafe { slice_f64(a) };
    let ys = unsafe { slice_f64(b) };
    let n = xs.len().min(ys.len());
    let mode_ref = is_reference_mode();
    let exact = float_mode == 0 || float_mode == REDUCE_F64_ADD_EXACT_TREE;
    let (res, threads) = if n == 0 {
        (0.0f64, 1)
    } else if exact {
        parallel_f64_exact_tree(n, |i| xs[i] * ys[i])
    } else if mode_ref || n < 16384 {
        let mut a0 = 0.0f64;
        let mut a1 = 0.0f64;
        let mut a2 = 0.0f64;
        let mut a3 = 0.0f64;
        let mut i = 0;
        while i + 4 <= n {
            a0 += xs[i] * ys[i];
            a1 += xs[i + 1] * ys[i + 1];
            a2 += xs[i + 2] * ys[i + 2];
            a3 += xs[i + 3] * ys[i + 3];
            i += 4;
        }
        let mut s = (a0 + a1) + (a2 + a3);
        while i < n {
            s += xs[i] * ys[i];
            i += 1;
        }
        (s, 1)
    } else {
        let mut partials = [0.0f64; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut a0 = 0.0f64;
            let mut a1 = 0.0f64;
            let mut a2 = 0.0f64;
            let mut a3 = 0.0f64;
            let mut i = lo;
            while i + 4 <= hi {
                a0 += xs[i] * ys[i];
                a1 += xs[i + 1] * ys[i + 1];
                a2 += xs[i + 2] * ys[i + 2];
                a3 += xs[i + 3] * ys[i + 3];
                i += 4;
            }
            let mut s = (a0 + a1) + (a2 + a3);
            while i < hi {
                s += xs[i] * ys[i];
                i += 1;
            }
            unsafe { *(p_ptr as *mut f64).add(tid) = s };
        });
        (partials[..t].iter().copied().sum(), t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "fused-dot-f64".to_string(),
        schedule: format!("fused-zip-reduce(n={n},threads={threads})"),
        numerical_mode: if exact { "f64-add-exact-tree" } else { "f64-add-relaxed" },
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    res.to_bits() as i64
}

/// Fused wrapping `I32` dot product: allocates ZERO product vector.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_dot_i32(a: i64, b: i64) -> i64 {
    let xs = unsafe { slice_i32(a) };
    let ys = unsafe { slice_i32(b) };
    let n = xs.len().min(ys.len());
    let mode_ref = is_reference_mode();
    let (res, threads) = if mode_ref || n < 16384 {
        let mut s = 0i32;
        for i in 0..n {
            s = s.wrapping_add(xs[i].wrapping_mul(ys[i]));
        }
        (s, 1)
    } else {
        let mut partials = [0i32; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut s = 0i32;
            for i in lo..hi {
                s = s.wrapping_add(xs[i].wrapping_mul(ys[i]));
            }
            unsafe { *(p_ptr as *mut i32).add(tid) = s };
        });
        let mut total = 0i32;
        for &p in &partials[..t] {
            total = total.wrapping_add(p);
        }
        (total, t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "fused-dot-i32".to_string(),
        schedule: format!("fused-zip-reduce(n={n},threads={threads})"),
        numerical_mode: "wrapping-add",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    i64::from(res)
}

/// Strided fused dot product over two 1D views (handles aliased, sliced, or
/// transposed views with zero temporary copy).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_strided_dot_f32(
    a: i64,
    off_a: i64,
    str_a: i64,
    b: i64,
    off_b: i64,
    str_b: i64,
    len: i64,
    float_mode: i64,
) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let ys = unsafe { slice_f32(b) };
    let n = len.max(0) as usize;
    let oa = off_a.max(0) as usize;
    let ob = off_b.max(0) as usize;
    let sa = str_a.max(0) as usize;
    let sb = str_b.max(0) as usize;
    if n == 0 {
        return i64::from(0.0f32.to_bits());
    }
    if oa + (n - 1) * sa >= xs.len() || ob + (n - 1) * sb >= ys.len() {
        abi::panic_exit(b"cinch_array_strided_dot_f32: view bounds exceeded");
    }
    let exact = float_mode == 0 || float_mode == REDUCE_F32_ADD_EXACT_TREE;
    let (res, threads) = if exact {
        parallel_f32_exact_tree(n, |i| xs[oa + i * sa] * ys[ob + i * sb])
    } else if is_reference_mode() || n < 16384 {
        let mut s = 0.0f32;
        for i in 0..n {
            s += xs[oa + i * sa] * ys[ob + i * sb];
        }
        (s, 1)
    } else {
        let mut partials = [0.0f32; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let mut s = 0.0f32;
            for i in lo..hi {
                s += xs[oa + i * sa] * ys[ob + i * sb];
            }
            unsafe { *(p_ptr as *mut f32).add(tid) = s };
        });
        (partials[..t].iter().copied().sum(), t)
    };
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "fused-strided-dot-f32".to_string(),
        schedule: format!("strided-zip-reduce(n={n},sa={sa},sb={sb},threads={threads})"),
        numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" },
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    i64::from(res.to_bits())
}

/// Fast packed vector binary op (`add`/`sub`/`mul`/`div`) on `Array F32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_zip_binop_f32(a: i64, b: i64, binop: i64) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let ys = unsafe { slice_f32(b) };
    if xs.len() != ys.len() {
        return 0;
    }
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(n, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, n) };
        match binop {
            BINOP_ADD => {
                for i in lo..hi {
                    d[i] = xs[i] + ys[i];
                }
            }
            BINOP_SUB => {
                for i in lo..hi {
                    d[i] = xs[i] - ys[i];
                }
            }
            BINOP_MUL => {
                for i in lo..hi {
                    d[i] = xs[i] * ys[i];
                }
            }
            BINOP_DIV => {
                for i in lo..hi {
                    d[i] = xs[i] / ys[i];
                }
            }
            _ => {
                for i in lo..hi {
                    d[i] = xs[i] + ys[i];
                }
            }
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "zip-binop-f32".to_string(),
        schedule: format!("1d-parallel(n={n},threads={threads})"),
        numerical_mode: "ieee754-f32",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Fast packed vector binary op (`add`/`sub`/`mul`) on `Array I32` (wrapping).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_zip_binop_i32(a: i64, b: i64, binop: i64) -> i64 {
    let xs = unsafe { slice_i32(a) };
    let ys = unsafe { slice_i32(b) };
    if xs.len() != ys.len() {
        return 0;
    }
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_I32) };
    let dst = unsafe { slice_i32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(n, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut i32, n) };
        match binop {
            BINOP_ADD => {
                for i in lo..hi {
                    d[i] = xs[i].wrapping_add(ys[i]);
                }
            }
            BINOP_SUB => {
                for i in lo..hi {
                    d[i] = xs[i].wrapping_sub(ys[i]);
                }
            }
            BINOP_MUL => {
                for i in lo..hi {
                    d[i] = xs[i].wrapping_mul(ys[i]);
                }
            }
            _ => {
                for i in lo..hi {
                    d[i] = xs[i].wrapping_add(ys[i]);
                }
            }
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "zip-binop-i32".to_string(),
        schedule: format!("1d-parallel(n={n},threads={threads})"),
        numerical_mode: "wrapping-i32",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Fast packed unary map on `Array F32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_map_unary_f32(a: i64, map_op: i64, scalar_bits: i64) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let c = f32::from_bits(scalar_bits as u32);
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(n, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, n) };
        match map_op {
            UNARY_ADD_CONST => {
                for i in lo..hi {
                    d[i] = xs[i] + c;
                }
            }
            UNARY_MUL_CONST => {
                for i in lo..hi {
                    d[i] = xs[i] * c;
                }
            }
            UNARY_SQUARE => {
                for i in lo..hi {
                    d[i] = xs[i] * xs[i];
                }
            }
            UNARY_ABS => {
                for i in lo..hi {
                    d[i] = xs[i].abs();
                }
            }
            UNARY_NEG => {
                for i in lo..hi {
                    d[i] = -xs[i];
                }
            }
            _ => {
                for i in lo..hi {
                    d[i] = xs[i];
                }
            }
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "map-unary-f32".to_string(),
        schedule: format!("1d-parallel(n={n},threads={threads})"),
        numerical_mode: "ieee754-f32",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Fused map + reduce over `Array F32` (zero intermediate array!).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_fused_map_reduce_f32(
    a: i64,
    map_op: i64,
    scalar_bits: i64,
    reduce_op: i64,
) -> i64 {
    let xs = unsafe { slice_f32(a) };
    let c = f32::from_bits(scalar_bits as u32);
    let n = xs.len();
    let apply = move |x: f32| -> f32 {
        match map_op {
            UNARY_ADD_CONST => x + c,
            UNARY_MUL_CONST => x * c,
            UNARY_SQUARE => x * x,
            UNARY_ABS => x.abs(),
            UNARY_NEG => -x,
            _ => x,
        }
    };
    let (res, threads) = if n == 0 {
        let id = match reduce_op {
            REDUCE_F32_MIN => f32::INFINITY,
            REDUCE_F32_MAX => f32::NEG_INFINITY,
            _ => 0.0f32,
        };
        (id, 1)
    } else if reduce_op == REDUCE_F32_ADD_EXACT_TREE {
        parallel_f32_exact_tree(n, |i| apply(xs[i]))
    } else if is_reference_mode() || n < 16384 {
        let v = match reduce_op {
            REDUCE_F32_MIN => xs.iter().copied().map(apply).fold(f32::INFINITY, f32::min),
            REDUCE_F32_MAX => xs.iter().copied().map(apply).fold(f32::NEG_INFINITY, f32::max),
            _ => xs.iter().copied().map(apply).sum(),
        };
        (v, 1)
    } else {
        let mut partials = [0.0f32; 64];
        let p_ptr = partials.as_mut_ptr() as usize;
        let t = parallel_chunks(n, 8192, |tid, _, lo, hi| {
            let v = match reduce_op {
                REDUCE_F32_MIN => xs[lo..hi].iter().copied().map(apply).fold(f32::INFINITY, f32::min),
                REDUCE_F32_MAX => xs[lo..hi].iter().copied().map(apply).fold(f32::NEG_INFINITY, f32::max),
                _ => xs[lo..hi].iter().copied().map(apply).sum(),
            };
            unsafe { *(p_ptr as *mut f32).add(tid) = v };
        });
        let v = match reduce_op {
            REDUCE_F32_MIN => partials[..t].iter().copied().fold(f32::INFINITY, f32::min),
            REDUCE_F32_MAX => partials[..t].iter().copied().fold(f32::NEG_INFINITY, f32::max),
            _ => partials[..t].iter().copied().sum(),
        };
        (v, t)
    };
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "fused-map-reduce-f32".to_string(),
        schedule: format!("fused-map-reduce(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(reduce_op),
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    i64::from(res.to_bits())
}

// ---------------------------------------------------------------------------
// Nested parallelism: Row reductions (m x n -> m) with dynamic schedule choice
// ---------------------------------------------------------------------------

/// Row reduction on packed `F32` matrix (`rows x cols -> rows`).
/// Dynamically chooses between:
/// - `OuterParallel`: many rows (`rows >= 4 * threads` or small `cols`), parallel across rows;
/// - `InnerParallel`: few large rows (`rows < 4 * threads && cols >= 16384`), parallel within each row.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_row_reduce_f32(
    a: i64,
    rows: i64,
    cols: i64,
    reduce_op: i64,
) -> i64 {
    let r = rows.max(0) as usize;
    let c = cols.max(0) as usize;
    let xs = unsafe { slice_f32(a) };
    if r.checked_mul(c) != Some(xs.len()) {
        return 0;
    }
    let out = unsafe { arrays::alloc_uninit_packed(r as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let mode_ref = is_reference_mode();
    let budget = if mode_ref { 1 } else { par::budget().clamp(1, pool().max_workers) };

    let (schedule, threads) = if r == 0 {
        ("empty-rows".to_string(), 1)
    } else if c == 0 {
        let id = match reduce_op {
            REDUCE_F32_MIN => f32::INFINITY,
            REDUCE_F32_MAX => f32::NEG_INFINITY,
            _ => 0.0f32,
        };
        dst.fill(id);
        ("empty-cols-identity".to_string(), 1)
    } else if mode_ref || budget == 1 {
        for row in 0..r {
            let row_slice = &xs[row * c..(row + 1) * c];
            dst[row] = match reduce_op {
                REDUCE_F32_ADD_EXACT_TREE => f32_exact_tree_sum(row_slice),
                REDUCE_F32_MIN => row_slice.iter().copied().fold(f32::INFINITY, f32::min),
                REDUCE_F32_MAX => row_slice.iter().copied().fold(f32::NEG_INFINITY, f32::max),
                _ => row_slice.iter().copied().sum(),
            };
        }
        (format!("scalar(rows={r},cols={c})"), 1)
    } else if r >= budget * 2 || c < 8192 {
        // Outer-parallel: many rows -> parallelize across rows, tight serial reduction per row
        let d_ptr = dst.as_mut_ptr() as usize;
        let t = parallel_chunks(r, 1, |_, _, r_lo, r_hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, r) };
            for row in r_lo..r_hi {
                let row_slice = &xs[row * c..(row + 1) * c];
                d[row] = match reduce_op {
                    REDUCE_F32_ADD_EXACT_TREE => f32_exact_tree_sum(row_slice),
                    REDUCE_F32_MIN => row_slice.iter().copied().fold(f32::INFINITY, f32::min),
                    REDUCE_F32_MAX => row_slice.iter().copied().fold(f32::NEG_INFINITY, f32::max),
                    _ => {
                        let mut a0 = 0.0f32;
                        let mut a1 = 0.0f32;
                        let mut a2 = 0.0f32;
                        let mut a3 = 0.0f32;
                        let mut j = 0;
                        while j + 4 <= c {
                            a0 += row_slice[j];
                            a1 += row_slice[j + 1];
                            a2 += row_slice[j + 2];
                            a3 += row_slice[j + 3];
                            j += 4;
                        }
                        let mut s = (a0 + a1) + (a2 + a3);
                        while j < c {
                            s += row_slice[j];
                            j += 1;
                        }
                        s
                    }
                };
            }
        });
        (format!("outer-parallel(rows={r},cols={c},threads={t})"), t)
    } else {
        // Inner-parallel: few very long rows -> parallelize within each row!
        let mut max_t = 1;
        for row in 0..r {
            let row_slice = &xs[row * c..(row + 1) * c];
            let (val, t) = reduce_f32_slice(row_slice, reduce_op);
            dst[row] = val;
            max_t = max_t.max(t);
        }
        (format!("inner-parallel(rows={r},cols={c},threads={max_t})"), max_t)
    };

    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "row-reduce-f32".to_string(),
        schedule,
        numerical_mode: reduce_mode_name(reduce_op),
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Row reduction on packed `I32` matrix (`rows x cols -> rows`) with dynamic schedule selection.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_row_reduce_i32(
    a: i64,
    rows: i64,
    cols: i64,
    reduce_op: i64,
) -> i64 {
    let r = rows.max(0) as usize;
    let c = cols.max(0) as usize;
    let xs = unsafe { slice_i32(a) };
    if r.checked_mul(c) != Some(xs.len()) {
        return 0;
    }
    let out = unsafe { arrays::alloc_uninit_packed(r as i64, FMT_I32) };
    let dst = unsafe { slice_i32_mut(out) };
    let mode_ref = is_reference_mode();
    let budget = if mode_ref { 1 } else { par::budget().clamp(1, pool().max_workers) };

    let (schedule, threads) = if r == 0 {
        ("empty-rows".to_string(), 1)
    } else if c == 0 {
        dst.fill(i32_identity(reduce_op));
        ("empty-cols-identity".to_string(), 1)
    } else if mode_ref || budget == 1 {
        for row in 0..r {
            let mut acc = i32_identity(reduce_op);
            for &x in &xs[row * c..(row + 1) * c] {
                acc = i32_combine(reduce_op, acc, x);
            }
            dst[row] = acc;
        }
        (format!("scalar(rows={r},cols={c})"), 1)
    } else if r >= budget * 2 || c < 8192 {
        let d_ptr = dst.as_mut_ptr() as usize;
        let t = parallel_chunks(r, 1, |_, _, r_lo, r_hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut i32, r) };
            for row in r_lo..r_hi {
                let mut acc = i32_identity(reduce_op);
                for &x in &xs[row * c..(row + 1) * c] {
                    acc = i32_combine(reduce_op, acc, x);
                }
                d[row] = acc;
            }
        });
        (format!("outer-parallel(rows={r},cols={c},threads={t})"), t)
    } else {
        let mut max_t = 1;
        for row in 0..r {
            let row_slice = &xs[row * c..(row + 1) * c];
            let mut partials = [i32_identity(reduce_op); 64];
            let p_ptr = partials.as_mut_ptr() as usize;
            let t = parallel_chunks(c, 8192, |tid, _, lo, hi| {
                let mut acc = i32_identity(reduce_op);
                for &x in &row_slice[lo..hi] {
                    acc = i32_combine(reduce_op, acc, x);
                }
                unsafe { *(p_ptr as *mut i32).add(tid) = acc };
            });
            let mut acc = i32_identity(reduce_op);
            for &p in &partials[..t] {
                acc = i32_combine(reduce_op, acc, p);
            }
            dst[row] = acc;
            max_t = max_t.max(t);
        }
        (format!("inner-parallel(rows={r},cols={c},threads={max_t})"), max_t)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "row-reduce-i32".to_string(),
        schedule,
        numerical_mode: reduce_mode_name(reduce_op),
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// Prefix Scan (inclusive & exclusive, two-pass parallel block scan)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scan_i32(a: i64, op: i64, inclusive: i64) -> i64 {
    let xs = unsafe { slice_i32(a) };
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_I32) };
    let dst = unsafe { slice_i32_mut(out) };
    let inc = inclusive != 0;
    let mode_ref = is_reference_mode();
    let (threads, temp_bytes) = if n == 0 {
        (1, 0)
    } else if mode_ref || n < 32768 {
        let mut acc = i32_identity(op);
        for i in 0..n {
            if inc {
                acc = i32_combine(op, acc, xs[i]);
                dst[i] = acc;
            } else {
                dst[i] = acc;
                acc = i32_combine(op, acc, xs[i]);
            }
        }
        (1, 0)
    } else {
        let nt = active_threads(n, 16384);
        let mut block_sums = vec![i32_identity(op); nt];
        let bs_ptr = block_sums.as_mut_ptr() as usize;
        // Pass 1: local block reductions
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let mut acc = i32_identity(op);
            for &x in &xs[lo..hi] {
                acc = i32_combine(op, acc, x);
            }
            unsafe { *(bs_ptr as *mut i32).add(tid) = acc };
        });
        // Exclusive prefix scan over block sums
        let mut offsets = vec![i32_identity(op); nt];
        let mut running = i32_identity(op);
        for t in 0..nt {
            offsets[t] = running;
            running = i32_combine(op, running, block_sums[t]);
        }
        // Pass 2: local prefix scan with block offset
        let d_ptr = dst.as_mut_ptr() as usize;
        let off_ptr = offsets.as_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut i32, n) };
            let mut acc = unsafe { *(off_ptr as *const i32).add(tid) };
            if inc {
                for i in lo..hi {
                    acc = i32_combine(op, acc, xs[i]);
                    d[i] = acc;
                }
            } else {
                for i in lo..hi {
                    d[i] = acc;
                    acc = i32_combine(op, acc, xs[i]);
                }
            }
        });
        (nt, nt * 8)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: if inc { "scan-inclusive-i32" } else { "scan-exclusive-i32" }.to_string(),
        schedule: format!("two-pass-block-scan(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: if threads > 1 { 2 } else { 1 },
        peak_temp_bytes: temp_bytes,
        reused_in_place: false,
        threads,
    });
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scan_u32(a: i64, op: i64, inclusive: i64) -> i64 {
    let xs = unsafe { slice_u32(a) };
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_U32) };
    let dst = unsafe { slice_u32_mut(out) };
    let inc = inclusive != 0;
    let mode_ref = is_reference_mode();
    let (threads, temp_bytes) = if n == 0 {
        (1, 0)
    } else if mode_ref || n < 32768 {
        let mut acc = u32_identity(op);
        for i in 0..n {
            if inc {
                acc = u32_combine(op, acc, xs[i]);
                dst[i] = acc;
            } else {
                dst[i] = acc;
                acc = u32_combine(op, acc, xs[i]);
            }
        }
        (1, 0)
    } else {
        let nt = active_threads(n, 16384);
        let mut block_sums = vec![u32_identity(op); nt];
        let bs_ptr = block_sums.as_mut_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let mut acc = u32_identity(op);
            for &x in &xs[lo..hi] {
                acc = u32_combine(op, acc, x);
            }
            unsafe { *(bs_ptr as *mut u32).add(tid) = acc };
        });
        let mut offsets = vec![u32_identity(op); nt];
        let mut running = u32_identity(op);
        for t in 0..nt {
            offsets[t] = running;
            running = u32_combine(op, running, block_sums[t]);
        }
        let d_ptr = dst.as_mut_ptr() as usize;
        let off_ptr = offsets.as_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut u32, n) };
            let mut acc = unsafe { *(off_ptr as *const u32).add(tid) };
            if inc {
                for i in lo..hi {
                    acc = u32_combine(op, acc, xs[i]);
                    d[i] = acc;
                }
            } else {
                for i in lo..hi {
                    d[i] = acc;
                    acc = u32_combine(op, acc, xs[i]);
                }
            }
        });
        (nt, nt * 8)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: if inc { "scan-inclusive-u32" } else { "scan-exclusive-u32" }.to_string(),
        schedule: format!("two-pass-block-scan(n={n},threads={threads})"),
        numerical_mode: reduce_mode_name(op),
        kernel_launches: if threads > 1 { 2 } else { 1 },
        peak_temp_bytes: temp_bytes,
        reused_in_place: false,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// Stable Compaction (Filter)
// ---------------------------------------------------------------------------

/// Stable parallel compaction of packed `I32` elements strictly greater than `threshold`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_filter_i32_gt(a: i64, threshold: i64) -> i64 {
    let xs = unsafe { slice_i32(a) };
    let thr = threshold as i32;
    let n = xs.len();
    let mode_ref = is_reference_mode();
    if mode_ref || n < 32768 {
        let mut kept = Vec::with_capacity(n / 2);
        for &x in xs {
            if x > thr {
                kept.push(x);
            }
        }
        let out = unsafe { arrays::alloc_uninit_packed(kept.len() as i64, FMT_I32) };
        if !kept.is_empty() {
            unsafe { slice_i32_mut(out) }.copy_from_slice(&kept);
        }
        record_report(LaunchReport {
            backend: if mode_ref { "reference" } else { "cpu" },
            op: "filter-stable-i32".to_string(),
            schedule: format!("sequential-compact(n={n},k={})", kept.len()),
            numerical_mode: "exact-stable",
            kernel_launches: 1,
            peak_temp_bytes: kept.len() * 4,
            reused_in_place: false,
            threads: 1,
        });
        return out;
    }
    // Two-pass parallel stable compaction (count per block -> prefix sum -> scatter)
    let nt = active_threads(n, 16384);
    let mut counts = vec![0usize; nt];
    let c_ptr = counts.as_mut_ptr() as usize;
    parallel_chunks(n, 16384, |tid, _, lo, hi| {
        let mut cnt = 0usize;
        for &x in &xs[lo..hi] {
            cnt += usize::from(x > thr);
        }
        unsafe { *(c_ptr as *mut usize).add(tid) = cnt };
    });
    let mut offsets = vec![0usize; nt];
    let mut total = 0usize;
    for t in 0..nt {
        offsets[t] = total;
        total += counts[t];
    }
    let out = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_I32) };
    if total > 0 {
        let dst = unsafe { slice_i32_mut(out) };
        let d_ptr = dst.as_mut_ptr() as usize;
        let off_ptr = offsets.as_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut i32, total) };
            let mut pos = unsafe { *(off_ptr as *const usize).add(tid) };
            for &x in &xs[lo..hi] {
                if x > thr {
                    d[pos] = x;
                    pos += 1;
                }
            }
        });
    }
    record_report(LaunchReport {
        backend: "cpu",
        op: "filter-stable-i32".to_string(),
        schedule: format!("two-pass-compact(n={n},k={total},threads={nt})"),
        numerical_mode: "exact-stable",
        kernel_launches: 2,
        peak_temp_bytes: nt * 16,
        reused_in_place: false,
        threads: nt,
    });
    out
}

/// Stable compaction with an arbitrary Cinch predicate closure `a -> Bool`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_filter_packed(pred: i64, a: i64) -> i64 {
    let n = unsafe { arrays::cinch_array_len(a) } as usize;
    let fmt = unsafe { arrays::format(a) };
    let in_counted = arrays::counted(fmt);
    let mode_ref = is_reference_mode();
    let mut flags = vec![0u8; n];
    let f_ptr = flags.as_mut_ptr() as usize;
    let threads = if mode_ref || n < 4096 {
        for i in 0..n {
            let x = unsafe { arrays::read(a, i as i64) };
            if in_counted {
                unsafe { rc::cinch_dup(x) };
            }
            let keep = unsafe { invoke_closure(pred, x) };
            flags[i] = u8::from(keep != 0);
        }
        1
    } else {
        if rc::reclaiming() {
            unsafe {
                rc::cinch_mark_mt(pred);
                rc::cinch_mark_mt(a);
            }
        }
        parallel_chunks(n, 2048, |_, _, lo, hi| {
            let fl = unsafe { std::slice::from_raw_parts_mut(f_ptr as *mut u8, n) };
            for i in lo..hi {
                let x = unsafe { arrays::read(a, i as i64) };
                if in_counted {
                    unsafe { rc::cinch_dup(x) };
                }
                let keep = unsafe { invoke_closure(pred, x) };
                fl[i] = u8::from(keep != 0);
            }
        })
    };
    let mut k = 0usize;
    for &fl in &flags {
        k += usize::from(fl);
    }
    let out = if fmt & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(k as i64, fmt) }
    } else {
        unsafe { arrays::array(k as i64, fmt) }
    };
    let mut pos = 0i64;
    for (i, &fl) in flags.iter().enumerate() {
        if fl != 0 {
            let x = unsafe { arrays::read(a, i as i64) };
            if in_counted {
                unsafe { rc::cinch_dup(x) };
            }
            unsafe { arrays::write(out, pos, x) };
            pos += 1;
        }
    }
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "filter-stable-closure".to_string(),
        schedule: format!("flags-compact(n={n},k={k},threads={threads})"),
        numerical_mode: "exact-stable",
        kernel_launches: 2,
        peak_temp_bytes: n,
        reused_in_place: false,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// Gather & Scatter (unique in-place + explicit collision combining)
// ---------------------------------------------------------------------------

#[inline]
pub(crate) fn resolve_edge(idx: i64, len: usize, policy: i64) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let n = len as i64;
    if idx >= 0 && idx < n {
        return Some(idx as usize);
    }
    match policy {
        EDGE_CLAMP => Some(idx.clamp(0, n - 1) as usize),
        EDGE_ZERO => None,
        EDGE_MIRROR => {
            if n == 1 {
                return Some(0);
            }
            let period = 2 * n - 2;
            let mut m = idx.rem_euclid(period);
            if m >= n {
                m = period - m;
            }
            Some(m as usize)
        }
        EDGE_WRAP => Some(idx.rem_euclid(n) as usize),
        _ => None,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_gather_f32(data: i64, indices: i64, edge_policy: i64) -> i64 {
    let src = unsafe { slice_f32(data) };
    let idx = unsafe { slice_i32(indices) };
    let m = idx.len();
    let out = unsafe { arrays::alloc_uninit_packed(m as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(m, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, m) };
        for i in lo..hi {
            d[i] = match resolve_edge(i64::from(idx[i]), src.len(), edge_policy) {
                Some(j) => src[j],
                None => 0.0,
            };
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "gather-f32".to_string(),
        schedule: format!("1d-gather(m={m},threads={threads})"),
        numerical_mode: "exact",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_gather_i32(data: i64, indices: i64, edge_policy: i64) -> i64 {
    let src = unsafe { slice_i32(data) };
    let idx = unsafe { slice_i32(indices) };
    let m = idx.len();
    let out = unsafe { arrays::alloc_uninit_packed(m as i64, FMT_I32) };
    let dst = unsafe { slice_i32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(m, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut i32, m) };
        for i in lo..hi {
            d[i] = match resolve_edge(i64::from(idx[i]), src.len(), edge_policy) {
                Some(j) => src[j],
                None => 0,
            };
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "gather-i32".to_string(),
        schedule: format!("1d-gather(m={m},threads={threads})"),
        numerical_mode: "exact",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Unique-destination scatter replacement on packed `U32`.
/// Reuses `dest` in place when uniquely owned (`RC == 1`), otherwise copies.
/// Out-of-bounds indices are ignored (Futhark `scatter` semantics).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scatter_unique_u32(
    dest: i64,
    indices: i64,
    values: i64,
) -> i64 {
    let idx = unsafe { slice_u32(indices) };
    let vals = unsafe { slice_u32(values) };
    let m = idx.len().min(vals.len());
    let n = unsafe { arrays::cinch_array_len(dest) } as usize;
    let reused = unsafe { arrays::is_exclusive(dest) };
    let out = if reused {
        unsafe { rc::cinch_dup(dest) };
        dest
    } else {
        let fresh = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_U32) };
        if n > 0 {
            unsafe { slice_u32_mut(fresh) }.copy_from_slice(unsafe { slice_u32(dest) });
        }
        fresh
    };
    let dst = unsafe { slice_u32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = if is_reference_mode() || m < 16384 {
        for i in 0..m {
            let j = idx[i] as usize;
            if j < n {
                dst[j] = vals[i];
            }
        }
        1
    } else {
        parallel_chunks(m, 8192, |_, _, lo, hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut u32, n) };
            for i in lo..hi {
                let j = idx[i] as usize;
                if j < n {
                    // Unique-destination contract: disjoint slots.
                    d[j] = vals[i];
                }
            }
        })
    };
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "scatter-unique-u32".to_string(),
        schedule: format!("1d-scatter-unique(n={n},m={m},threads={threads})"),
        numerical_mode: "unique-replace",
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: reused,
        threads,
    });
    out
}

/// Combining scatter on packed `I32` with explicit associative combining op for
/// colliding indices! Race-free via thread-local privatization when `n` is small
/// or sequential deterministic combine, reusing `dest` in place when `RC == 1`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scatter_combine_i32(
    dest: i64,
    indices: i64,
    values: i64,
    combine_op: i64,
) -> i64 {
    let idx = unsafe { slice_i32(indices) };
    let vals = unsafe { slice_i32(values) };
    let m = idx.len().min(vals.len());
    let n = unsafe { arrays::cinch_array_len(dest) } as usize;
    let reused = unsafe { arrays::is_exclusive(dest) };
    let out = if reused {
        unsafe { rc::cinch_dup(dest) };
        dest
    } else {
        let fresh = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_I32) };
        if n > 0 {
            unsafe { slice_i32_mut(fresh) }.copy_from_slice(unsafe { slice_i32(dest) });
        }
        fresh
    };
    let dst = unsafe { slice_i32_mut(out) };
    let mode_ref = is_reference_mode();
    let (threads, temp_bytes) = if mode_ref || m < 16384 || n == 0 || n > 65536 {
        for i in 0..m {
            let j = idx[i];
            if j >= 0 && (j as usize) < n {
                let ju = j as usize;
                dst[ju] = i32_combine(combine_op, dst[ju], vals[i]);
            }
        }
        (1, 0)
    } else {
        let nt = active_threads(m, 8192);
        let id = i32_identity(combine_op);
        let mut priv_buf = vec![id; nt * n];
        let p_ptr = priv_buf.as_mut_ptr() as usize;
        parallel_chunks(m, 8192, |tid, _, lo, hi| {
            let local = unsafe { std::slice::from_raw_parts_mut((p_ptr as *mut i32).add(tid * n), n) };
            for i in lo..hi {
                let j = idx[i];
                if j >= 0 && (j as usize) < n {
                    let ju = j as usize;
                    local[ju] = i32_combine(combine_op, local[ju], vals[i]);
                }
            }
        });
        for t in 0..nt {
            let local = &priv_buf[t * n..(t + 1) * n];
            for j in 0..n {
                dst[j] = i32_combine(combine_op, dst[j], local[j]);
            }
        }
        (nt, nt * n * 4)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "scatter-combine-i32".to_string(),
        schedule: format!("privatized-scatter-combine(n={n},m={m},threads={threads})"),
        numerical_mode: reduce_mode_name(combine_op),
        kernel_launches: 1,
        peak_temp_bytes: temp_bytes,
        reused_in_place: reused,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// Histogram (single & batched frames x height x width)
// ---------------------------------------------------------------------------

/// Histogram over packed `U8` pixels -> packed `Array U32` of length `num_bins`.
/// Uses thread-local privatized bin arrays so hot-bin contention never stalls.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_histogram_u8(data: i64, num_bins: i64) -> i64 {
    let bins = num_bins.max(0) as usize;
    let src = unsafe { slice_u8(data) };
    let n = src.len();
    let out = unsafe { arrays::array(bins as i64, FMT_U32) };
    if bins == 0 {
        return out;
    }
    let dst = unsafe { slice_u32_mut(out) };
    let mode_ref = is_reference_mode();
    let (threads, temp_bytes) = if mode_ref || n < 32768 {
        for &px in src {
            let b = px as usize;
            if b < bins {
                dst[b] = dst[b].wrapping_add(1);
            }
        }
        (1, 0)
    } else {
        let nt = active_threads(n, 16384);
        let mut priv_bins = vec![0u32; nt * bins];
        let p_ptr = priv_bins.as_mut_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let local = unsafe { std::slice::from_raw_parts_mut((p_ptr as *mut u32).add(tid * bins), bins) };
            for &px in &src[lo..hi] {
                let b = px as usize;
                if b < bins {
                    local[b] = local[b].wrapping_add(1);
                }
            }
        });
        for t in 0..nt {
            let local = &priv_bins[t * bins..(t + 1) * bins];
            for b in 0..bins {
                dst[b] = dst[b].wrapping_add(local[b]);
            }
        }
        (nt, nt * bins * 4)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "histogram-u8".to_string(),
        schedule: format!("privatized-histogram(n={n},bins={bins},threads={threads})"),
        numerical_mode: "wrapping-add",
        kernel_launches: 1,
        peak_temp_bytes: temp_bytes,
        reused_in_place: false,
        threads,
    });
    out
}

/// Histogram over packed `U32` keys -> packed `Array U32` of length `num_bins`
/// (building block for radix sort & general histograms).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_histogram_u32(keys: i64, num_bins: i64) -> i64 {
    let bins = num_bins.max(0) as usize;
    let src = unsafe { slice_u32(keys) };
    let n = src.len();
    let out = unsafe { arrays::array(bins as i64, FMT_U32) };
    if bins == 0 {
        return out;
    }
    let dst = unsafe { slice_u32_mut(out) };
    let mode_ref = is_reference_mode();
    let (threads, temp_bytes) = if mode_ref || n < 32768 || bins > 65536 {
        for &k in src {
            let b = k as usize;
            if b < bins {
                dst[b] = dst[b].wrapping_add(1);
            }
        }
        (1, 0)
    } else {
        let nt = active_threads(n, 16384);
        let mut priv_bins = vec![0u32; nt * bins];
        let p_ptr = priv_bins.as_mut_ptr() as usize;
        parallel_chunks(n, 16384, |tid, _, lo, hi| {
            let local = unsafe { std::slice::from_raw_parts_mut((p_ptr as *mut u32).add(tid * bins), bins) };
            for &k in &src[lo..hi] {
                let b = k as usize;
                if b < bins {
                    local[b] = local[b].wrapping_add(1);
                }
            }
        });
        for t in 0..nt {
            let local = &priv_bins[t * bins..(t + 1) * bins];
            for b in 0..bins {
                dst[b] = dst[b].wrapping_add(local[b]);
            }
        }
        (nt, nt * bins * 4)
    };
    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "histogram-u32".to_string(),
        schedule: format!("privatized-histogram(n={n},bins={bins},threads={threads})"),
        numerical_mode: "wrapping-add",
        kernel_launches: 1,
        peak_temp_bytes: temp_bytes,
        reused_in_place: false,
        threads,
    });
    out
}

/// Batched histogram over `frames x height x width` packed `U8` images ->
/// packed `Array U32` of length `frames * num_bins`.
/// Dynamically switches schedule between `many-small-frames` (outer parallel across frames)
/// and `few-large-frames` (inner privatized parallel within each frame)!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_histogram_batch_u8(
    data: i64,
    frames: i64,
    height: i64,
    width: i64,
    num_bins: i64,
) -> i64 {
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let bins = num_bins.max(0) as usize;
    let src = unsafe { slice_u8(data) };
    let frame_pixels = h.saturating_mul(w);
    if f.checked_mul(frame_pixels) != Some(src.len()) {
        return 0;
    }
    let out = unsafe { arrays::array((f * bins) as i64, FMT_U32) };
    if f == 0 || bins == 0 {
        return out;
    }
    let dst = unsafe { slice_u32_mut(out) };
    let mode_ref = is_reference_mode();
    let budget = if mode_ref { 1 } else { par::budget().clamp(1, pool().max_workers) };

    let (schedule, threads, temp_bytes) = if mode_ref || budget == 1 {
        for fi in 0..f {
            let fslice = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
            let fhist = &mut dst[fi * bins..(fi + 1) * bins];
            for &px in fslice {
                let b = px as usize;
                if b < bins {
                    fhist[b] = fhist[b].wrapping_add(1);
                }
            }
        }
        (format!("scalar(frames={f},pixels={frame_pixels})"), 1, 0)
    } else if f >= budget * 2 || frame_pixels < 32768 {
        // Many small frames: parallelize across frames, zero extra temp buffers!
        let d_ptr = dst.as_mut_ptr() as usize;
        let t = parallel_chunks(f, 1, |_, _, f_lo, f_hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut u32, f * bins) };
            for fi in f_lo..f_hi {
                let fslice = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
                let fhist = &mut d[fi * bins..(fi + 1) * bins];
                for &px in fslice {
                    let b = px as usize;
                    if b < bins {
                        fhist[b] = fhist[b].wrapping_add(1);
                    }
                }
            }
        });
        (format!("many-small-frames(frames={f},frame_pixels={frame_pixels},threads={t})"), t, 0)
    } else {
        // Few large frames: inner privatized histogram within each frame!
        let nt = active_threads(frame_pixels, 16384);
        let mut priv_bins = vec![0u32; nt * bins];
        let p_ptr = priv_bins.as_mut_ptr() as usize;
        for fi in 0..f {
            priv_bins.fill(0);
            let fslice = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
            parallel_chunks(frame_pixels, 16384, |tid, _, lo, hi| {
                let local = unsafe { std::slice::from_raw_parts_mut((p_ptr as *mut u32).add(tid * bins), bins) };
                for &px in &fslice[lo..hi] {
                    let b = px as usize;
                    if b < bins {
                        local[b] = local[b].wrapping_add(1);
                    }
                }
            });
            let fhist = &mut dst[fi * bins..(fi + 1) * bins];
            for t in 0..nt {
                let local = &priv_bins[t * bins..(t + 1) * bins];
                for b in 0..bins {
                    fhist[b] = fhist[b].wrapping_add(local[b]);
                }
            }
        }
        (
            format!("few-large-frames(frames={f},frame_pixels={frame_pixels},threads={nt})"),
            nt,
            nt * bins * 4,
        )
    };

    record_report(LaunchReport {
        backend: if mode_ref { "reference" } else { "cpu" },
        op: "histogram-batch-u8".to_string(),
        schedule,
        numerical_mode: "wrapping-add",
        kernel_launches: 1,
        peak_temp_bytes: temp_bytes,
        reused_in_place: false,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// 2D Stencil (single, batched frames x height x width, & ping-pong iterated!)
// ---------------------------------------------------------------------------

#[inline]
fn stencil_sample_u8(frame: &[u8], h: usize, w: usize, y: i64, x: i64, policy: i64) -> i32 {
    let Some(ry) = resolve_edge(y, h, policy) else {
        return 0;
    };
    let Some(rx) = resolve_edge(x, w, policy) else {
        return 0;
    };
    i32::from(frame[ry * w + rx])
}

#[inline]
fn stencil_sample_f32(frame: &[f32], h: usize, w: usize, y: i64, x: i64, policy: i64) -> f32 {
    let Some(ry) = resolve_edge(y, h, policy) else {
        return 0.0;
    };
    let Some(rx) = resolve_edge(x, w, policy) else {
        return 0.0;
    };
    frame[ry * w + rx]
}

fn run_stencil_step_u8(
    src: &[u8],
    dst: &mut [u8],
    f: usize,
    h: usize,
    w: usize,
    weights: &[i32; 9],
    divisor: i32,
    policy: i64,
) -> (String, usize) {
    let frame_pixels = h * w;
    let div = if divisor == 0 { 1 } else { divisor };
    let mode_ref = is_reference_mode();
    let budget = if mode_ref { 1 } else { par::budget().clamp(1, pool().max_workers) };

    let apply_row = |frame_in: &[u8], row_out: &mut [u8], y: usize| {
        let yi = y as i64;
        if y > 0 && y + 1 < h && w >= 3 {
            // Fast interior path: no edge policy branches for x in 1..w-1!
            let r0 = &frame_in[(y - 1) * w..y * w];
            let r1 = &frame_in[y * w..(y + 1) * w];
            let r2 = &frame_in[(y + 1) * w..(y + 2) * w];
            // Left border x = 0
            {
                let mut acc = 0i32;
                for dy in -1..=1i64 {
                    for dx in -1..=1i64 {
                        acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                            * stencil_sample_u8(frame_in, h, w, yi + dy, dx, policy);
                    }
                }
                row_out[0] = (acc / div).clamp(0, 255) as u8;
            }
            for x in 1..w - 1 {
                let acc = weights[0] * i32::from(r0[x - 1])
                    + weights[1] * i32::from(r0[x])
                    + weights[2] * i32::from(r0[x + 1])
                    + weights[3] * i32::from(r1[x - 1])
                    + weights[4] * i32::from(r1[x])
                    + weights[5] * i32::from(r1[x + 1])
                    + weights[6] * i32::from(r2[x - 1])
                    + weights[7] * i32::from(r2[x])
                    + weights[8] * i32::from(r2[x + 1]);
                row_out[x] = (acc / div).clamp(0, 255) as u8;
            }
            // Right border x = w - 1
            {
                let xi = (w - 1) as i64;
                let mut acc = 0i32;
                for dy in -1..=1i64 {
                    for dx in -1..=1i64 {
                        acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                            * stencil_sample_u8(frame_in, h, w, yi + dy, xi + dx, policy);
                    }
                }
                row_out[w - 1] = (acc / div).clamp(0, 255) as u8;
            }
        } else {
            for x in 0..w {
                let xi = x as i64;
                let mut acc = 0i32;
                for dy in -1..=1i64 {
                    for dx in -1..=1i64 {
                        acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                            * stencil_sample_u8(frame_in, h, w, yi + dy, xi + dx, policy);
                    }
                }
                row_out[x] = (acc / div).clamp(0, 255) as u8;
            }
        }
    };

    if mode_ref || budget == 1 {
        for fi in 0..f {
            let fin = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
            let fout = &mut dst[fi * frame_pixels..(fi + 1) * frame_pixels];
            for y in 0..h {
                apply_row(fin, &mut fout[y * w..(y + 1) * w], y);
            }
        }
        (format!("scalar(frames={f},h={h},w={w})"), 1)
    } else if f >= budget * 2 {
        // Many small frames: parallelize across frames
        let d_ptr = dst.as_mut_ptr() as usize;
        let t = parallel_chunks(f, 1, |_, _, f_lo, f_hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut u8, f * frame_pixels) };
            for fi in f_lo..f_hi {
                let fin = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
                let fout = &mut d[fi * frame_pixels..(fi + 1) * frame_pixels];
                for y in 0..h {
                    apply_row(fin, &mut fout[y * w..(y + 1) * w], y);
                }
            }
        });
        (format!("many-small-frames(frames={f},h={h},w={w},threads={t})"), t)
    } else {
        // Few large frames: parallelize across (frame, row) pairs!
        let total_rows = f * h;
        let d_ptr = dst.as_mut_ptr() as usize;
        let t = parallel_chunks(total_rows, 8, |_, _, r_lo, r_hi| {
            let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut u8, f * frame_pixels) };
            for fr in r_lo..r_hi {
                let fi = fr / h;
                let y = fr % h;
                let fin = &src[fi * frame_pixels..(fi + 1) * frame_pixels];
                let row_out = &mut d[fi * frame_pixels + y * w..fi * frame_pixels + (y + 1) * w];
                apply_row(fin, row_out, y);
            }
        });
        (format!("few-large-frames(frames={f},h={h},w={w},threads={t})"), t)
    }
}

/// 2D 3x3 stencil over `frames x height x width` packed `U8` batch, supporting
/// `steps` repeated iterations with ping-pong 2-buffer storage reuse!
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_stencil2d_u8(
    data: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights_arr: i64,
    divisor: i64,
    edge_policy: i64,
    steps: i64,
) -> i64 {
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let total = f.saturating_mul(h).saturating_mul(w);
    let src_in = unsafe { slice_u8(data) };
    if src_in.len() != total {
        return 0;
    }
    let w_slice = unsafe { slice_i32(weights_arr) };
    if w_slice.len() != 9 {
        return 0;
    }
    let mut weights = [0i32; 9];
    weights.copy_from_slice(w_slice);
    let nsteps = steps.max(0) as usize;
    if nsteps == 0 {
        let out = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_U8) };
        if total > 0 {
            unsafe { slice_u8_mut(out) }.copy_from_slice(src_in);
        }
        return out;
    }
    if total == 0 {
        return unsafe { arrays::alloc_uninit_packed(0, FMT_U8) };
    }

    // Ping-pong storage planning:
    // - 1 step: 1 output buffer, 0 temporary buffers.
    // - k >= 2 steps: at most 2 buffers (ping & pong) total! If `data` is
    //   uniquely owned (`RC == 1`), reuse `data` as one of the two ping-pong
    //   buffers (peak temp = 1 buffer instead of k buffers)!
    let buf_a = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_U8) };
    let (mut schedule, mut threads) = run_stencil_step_u8(
        src_in,
        unsafe { slice_u8_mut(buf_a) },
        f,
        h,
        w,
        &weights,
        divisor as i32,
        edge_policy,
    );
    if nsteps == 1 {
        record_report(LaunchReport {
            backend: if is_reference_mode() { "reference" } else { "cpu" },
            op: "stencil2d-u8".to_string(),
            schedule,
            numerical_mode: "integer-clamped-u8",
            kernel_launches: 1,
            peak_temp_bytes: 0,
            reused_in_place: false,
            threads,
        });
        return buf_a;
    }

    let buf_b = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_U8) };
    let mut cur_src = buf_a;
    let mut cur_dst = buf_b;
    for _ in 1..nsteps {
        let (s, t) = run_stencil_step_u8(
            unsafe { slice_u8(cur_src) },
            unsafe { slice_u8_mut(cur_dst) },
            f,
            h,
            w,
            &weights,
            divisor as i32,
            edge_policy,
        );
        schedule = s;
        threads = t;
        std::mem::swap(&mut cur_src, &mut cur_dst);
    }
    // `cur_src` holds the final output; drop the spare ping-pong buffer.
    unsafe { rc::cinch_drop(cur_dst) };
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: format!("stencil2d-u8-pingpong(steps={nsteps})"),
        schedule,
        numerical_mode: "integer-clamped-u8",
        kernel_launches: nsteps,
        peak_temp_bytes: total,
        reused_in_place: false,
        threads,
    });
    cur_src
}

/// 2D 3x3 stencil over `frames x height x width` packed `F32` batch with ping-pong reuse.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_stencil2d_f32(
    data: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights_arr: i64,
    edge_policy: i64,
    steps: i64,
) -> i64 {
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let total = f.saturating_mul(h).saturating_mul(w);
    let src_in = unsafe { slice_f32(data) };
    if src_in.len() != total {
        return 0;
    }
    let w_slice = unsafe { slice_f32(weights_arr) };
    if w_slice.len() != 9 {
        return 0;
    }
    let mut weights = [0.0f32; 9];
    weights.copy_from_slice(w_slice);
    let nsteps = steps.max(0) as usize;
    if nsteps == 0 || total == 0 {
        let out = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_F32) };
        if total > 0 {
            unsafe { slice_f32_mut(out) }.copy_from_slice(src_in);
        }
        return out;
    }

    let step_f32 = |src: &[f32], dst: &mut [f32]| -> (String, usize) {
        let fp = h * w;
        let mode_ref = is_reference_mode();
        let budget = if mode_ref { 1 } else { par::budget().clamp(1, pool().max_workers) };
        let apply_row = |fin: &[f32], row_out: &mut [f32], y: usize| {
            let yi = y as i64;
            if y > 0 && y + 1 < h && w >= 3 {
                let r0 = &fin[(y - 1) * w..y * w];
                let r1 = &fin[y * w..(y + 1) * w];
                let r2 = &fin[(y + 1) * w..(y + 2) * w];
                {
                    let mut acc = 0.0f32;
                    for dy in -1..=1i64 {
                        for dx in -1..=1i64 {
                            acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                                * stencil_sample_f32(fin, h, w, yi + dy, dx, edge_policy);
                        }
                    }
                    row_out[0] = acc;
                }
                for x in 1..w - 1 {
                    row_out[x] = weights[0] * r0[x - 1]
                        + weights[1] * r0[x]
                        + weights[2] * r0[x + 1]
                        + weights[3] * r1[x - 1]
                        + weights[4] * r1[x]
                        + weights[5] * r1[x + 1]
                        + weights[6] * r2[x - 1]
                        + weights[7] * r2[x]
                        + weights[8] * r2[x + 1];
                }
                {
                    let xi = (w - 1) as i64;
                    let mut acc = 0.0f32;
                    for dy in -1..=1i64 {
                        for dx in -1..=1i64 {
                            acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                                * stencil_sample_f32(fin, h, w, yi + dy, xi + dx, edge_policy);
                        }
                    }
                    row_out[w - 1] = acc;
                }
            } else {
                for x in 0..w {
                    let xi = x as i64;
                    let mut acc = 0.0f32;
                    for dy in -1..=1i64 {
                        for dx in -1..=1i64 {
                            acc += weights[((dy + 1) * 3 + (dx + 1)) as usize]
                                * stencil_sample_f32(fin, h, w, yi + dy, xi + dx, edge_policy);
                        }
                    }
                    row_out[x] = acc;
                }
            }
        };
        if mode_ref || budget == 1 {
            for fi in 0..f {
                let fin = &src[fi * fp..(fi + 1) * fp];
                let fout = &mut dst[fi * fp..(fi + 1) * fp];
                for y in 0..h {
                    apply_row(fin, &mut fout[y * w..(y + 1) * w], y);
                }
            }
            (format!("scalar(frames={f},h={h},w={w})"), 1)
        } else if f >= budget * 2 {
            let d_ptr = dst.as_mut_ptr() as usize;
            let t = parallel_chunks(f, 1, |_, _, f_lo, f_hi| {
                let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, f * fp) };
                for fi in f_lo..f_hi {
                    let fin = &src[fi * fp..(fi + 1) * fp];
                    let fout = &mut d[fi * fp..(fi + 1) * fp];
                    for y in 0..h {
                        apply_row(fin, &mut fout[y * w..(y + 1) * w], y);
                    }
                }
            });
            (format!("many-small-frames(frames={f},h={h},w={w},threads={t})"), t)
        } else {
            let total_rows = f * h;
            let d_ptr = dst.as_mut_ptr() as usize;
            let t = parallel_chunks(total_rows, 8, |_, _, r_lo, r_hi| {
                let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, f * fp) };
                for fr in r_lo..r_hi {
                    let fi = fr / h;
                    let y = fr % h;
                    let fin = &src[fi * fp..(fi + 1) * fp];
                    let row_out = &mut d[fi * fp + y * w..fi * fp + (y + 1) * w];
                    apply_row(fin, row_out, y);
                }
            });
            (format!("few-large-frames(frames={f},h={h},w={w},threads={t})"), t)
        }
    };

    let buf_a = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_F32) };
    let (mut schedule, mut threads) = step_f32(src_in, unsafe { slice_f32_mut(buf_a) });
    if nsteps == 1 {
        record_report(LaunchReport {
            backend: if is_reference_mode() { "reference" } else { "cpu" },
            op: "stencil2d-f32".to_string(),
            schedule,
            numerical_mode: "ieee754-f32",
            kernel_launches: 1,
            peak_temp_bytes: 0,
            reused_in_place: false,
            threads,
        });
        return buf_a;
    }
    let buf_b = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_F32) };
    let mut cur_src = buf_a;
    let mut cur_dst = buf_b;
    for _ in 1..nsteps {
        let (s, t) = step_f32(unsafe { slice_f32(cur_src) }, unsafe { slice_f32_mut(cur_dst) });
        schedule = s;
        threads = t;
        std::mem::swap(&mut cur_src, &mut cur_dst);
    }
    unsafe { rc::cinch_drop(cur_dst) };
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: format!("stencil2d-f32-pingpong(steps={nsteps})"),
        schedule,
        numerical_mode: "ieee754-f32",
        kernel_launches: nsteps,
        peak_temp_bytes: total * 4,
        reused_in_place: false,
        threads,
    });
    cur_src
}

// ---------------------------------------------------------------------------
// Segmented Reduction & CSR SpMV (handles empty rows!)
// ---------------------------------------------------------------------------

/// Segmented reduction over packed `F32` values and CSR `row_offsets` (`Array U32` of length `rows + 1`).
/// Empty rows (`offsets[r] == offsets[r+1]`) yield the exact reducer identity.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_segmented_reduce_f32(
    values: i64,
    row_offsets: i64,
    reduce_op: i64,
) -> i64 {
    let vals = unsafe { slice_f32(values) };
    let offs = unsafe { slice_u32(row_offsets) };
    if offs.is_empty() {
        return unsafe { arrays::alloc_uninit_packed(0, FMT_F32) };
    }
    let nrows = offs.len() - 1;
    let out = unsafe { arrays::alloc_uninit_packed(nrows as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let id = match reduce_op {
        REDUCE_F32_MIN => f32::INFINITY,
        REDUCE_F32_MAX => f32::NEG_INFINITY,
        _ => 0.0f32,
    };
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(nrows, 256, |_, _, r_lo, r_hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, nrows) };
        for r in r_lo..r_hi {
            let s = (offs[r] as usize).min(vals.len());
            let e = (offs[r + 1] as usize).min(vals.len());
            if e <= s {
                d[r] = id;
            } else {
                let seg = &vals[s..e];
                d[r] = match reduce_op {
                    REDUCE_F32_ADD_EXACT_TREE => f32_exact_tree_sum(seg),
                    REDUCE_F32_MIN => seg.iter().copied().fold(f32::INFINITY, f32::min),
                    REDUCE_F32_MAX => seg.iter().copied().fold(f32::NEG_INFINITY, f32::max),
                    _ => seg.iter().copied().sum(),
                };
            }
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "segmented-reduce-f32".to_string(),
        schedule: format!("csr-row-parallel(rows={nrows},nnz={},threads={threads})", vals.len()),
        numerical_mode: reduce_mode_name(reduce_op),
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

/// Fused CSR SpMV: `y[r] = reduce_{k in row_offsets[r]..row_offsets[r+1]} values[k] * x[col_indices[k]]`.
/// Handles empty rows (`row_offsets[r] == row_offsets[r+1]`) by writing `0.0`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_spmv_csr_f32(
    values: i64,
    col_indices: i64,
    row_offsets: i64,
    x: i64,
    float_mode: i64,
) -> i64 {
    let vals = unsafe { slice_f32(values) };
    let cols = unsafe { slice_u32(col_indices) };
    let offs = unsafe { slice_u32(row_offsets) };
    let xv = unsafe { slice_f32(x) };
    if offs.is_empty() || vals.len() != cols.len() {
        return unsafe { arrays::alloc_uninit_packed(0, FMT_F32) };
    }
    let nrows = offs.len() - 1;
    let out = unsafe { arrays::alloc_uninit_packed(nrows as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let exact = float_mode == 0 || float_mode == REDUCE_F32_ADD_EXACT_TREE;
    let d_ptr = dst.as_mut_ptr() as usize;
    let threads = parallel_chunks(nrows, 256, |_, _, r_lo, r_hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, nrows) };
        for r in r_lo..r_hi {
            let s = (offs[r] as usize).min(vals.len());
            let e = (offs[r + 1] as usize).min(vals.len());
            if e <= s {
                d[r] = 0.0;
            } else if exact {
                d[r] = f32_exact_tree_prod(s, e, &|k| {
                    let c = cols[k] as usize;
                    vals[k] * if c < xv.len() { xv[c] } else { 0.0 }
                });
            } else {
                let mut acc = 0.0f32;
                for k in s..e {
                    let c = cols[k] as usize;
                    acc += vals[k] * if c < xv.len() { xv[c] } else { 0.0 };
                }
                d[r] = acc;
            }
        }
    });
    record_report(LaunchReport {
        backend: if is_reference_mode() { "reference" } else { "cpu" },
        op: "spmv-csr-f32".to_string(),
        schedule: format!("csr-fused-spmv(rows={nrows},nnz={},threads={threads})", vals.len()),
        numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" },
        kernel_launches: 1,
        peak_temp_bytes: 0,
        reused_in_place: false,
        threads,
    });
    out
}

// ---------------------------------------------------------------------------
// Strided view packing (materialize contiguous packed array only when needed)
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_pack_strided_1d(
    data: i64,
    offset: i64,
    len: i64,
    stride: i64,
) -> i64 {
    let f = unsafe { arrays::format(data) };
    let n = len.max(0) as usize;
    let off = offset.max(0) as usize;
    let st = stride.max(0) as usize;
    let src_len = unsafe { arrays::cinch_array_len(data) } as usize;
    if n > 0 && off + (n - 1) * st >= src_len {
        return 0;
    }
    // Zero-copy fast path when already contiguous and full-buffer!
    if off == 0 && st == 1 && n == src_len {
        unsafe { rc::cinch_dup(data) };
        return data;
    }
    let out = if f & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(n as i64, f) }
    } else {
        unsafe { arrays::array(n as i64, f) }
    };
    let is_counted = arrays::counted(f);
    for i in 0..n {
        let v = unsafe { arrays::read(data, (off + i * st) as i64) };
        if is_counted {
            unsafe { rc::cinch_dup(v) };
        }
        unsafe { arrays::write(out, i as i64, v) };
    }
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_pack_strided_2d(
    data: i64,
    offset: i64,
    rows: i64,
    cols: i64,
    stride_r: i64,
    stride_c: i64,
) -> i64 {
    let f = unsafe { arrays::format(data) };
    let r = rows.max(0) as usize;
    let c = cols.max(0) as usize;
    let off = offset.max(0) as usize;
    let sr = stride_r.max(0) as usize;
    let sc = stride_c.max(0) as usize;
    let src_len = unsafe { arrays::cinch_array_len(data) } as usize;
    let total = r.saturating_mul(c);
    if r > 0 && c > 0 && off + (r - 1) * sr + (c - 1) * sc >= src_len {
        return 0;
    }
    if off == 0 && sc == 1 && sr == c && total == src_len {
        unsafe { rc::cinch_dup(data) };
        return data;
    }
    let out = if f & 0x100 != 0 {
        unsafe { arrays::alloc_uninit_packed(total as i64, f) }
    } else {
        unsafe { arrays::array(total as i64, f) }
    };
    let is_counted = arrays::counted(f);
    for ri in 0..r {
        for ci in 0..c {
            let v = unsafe { arrays::read(data, (off + ri * sr + ci * sc) as i64) };
            if is_counted {
                unsafe { rc::cinch_dup(v) };
            }
            unsafe { arrays::write(out, (ri * c + ci) as i64, v) };
        }
    }
    out
}

/// Allocate a packed Cinch `Array` directly from a contiguous raw host buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_from_raw(src: *const u8, n: i64, format: i64) -> i64 {
    let w = arrays::elem_width(format);
    if w == 0 || n < 0 {
        return 0;
    }
    let out = unsafe { arrays::alloc_uninit_packed(n, format) };
    let bytes = (n as usize).saturating_mul(w);
    if bytes > 0 && !src.is_null() {
        unsafe {
            std::ptr::copy_nonoverlapping(src, arrays::data_ptr(out), bytes);
        }
    }
    out
}

/// Copy raw bytes out of a packed Cinch `Array` into a caller-provided host buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_copy_out_raw(a: i64, dst: *mut u8, max_bytes: usize) -> i64 {
    if a == 0 || dst.is_null() {
        return -1;
    }
    let f = unsafe { arrays::format(a) };
    let w = arrays::elem_width(f);
    let n = unsafe { arrays::cinch_array_len(a) }.max(0) as usize;
    let bytes = n.saturating_mul(w);
    if bytes > max_bytes {
        return -2;
    }
    if bytes > 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(arrays::data_ptr(a), dst, bytes);
        }
    }
    bytes as i64
}

// ---------------------------------------------------------------------------
// Automatic Differentiation (AD) parallel array helpers
// ---------------------------------------------------------------------------

/// Fill a packed `Array F32` of length `n` with `val` (adjoint of `reduce-f32`, zero tangent).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_fill_f32(n: i64, val_bits: i64) -> i64 {
    let len = n.max(0) as usize;
    let v = f32::from_bits(val_bits as u32);
    let out = unsafe { arrays::alloc_uninit_packed(len as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    parallel_chunks(len, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, len) };
        for i in lo..hi {
            d[i] = v;
        }
    });
    out
}

/// Fill a packed `Array F64` of length `n` with `val` (adjoint of `reduce-f64`, zero tangent).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_fill_f64(n: i64, val_bits: i64) -> i64 {
    let len = n.max(0) as usize;
    let v = f64::from_bits(val_bits as u64);
    let out = unsafe { arrays::alloc_uninit_packed(len as i64, FMT_F64) };
    let dst = unsafe { slice_f64_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    parallel_chunks(len, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f64, len) };
        for i in lo..hi {
            d[i] = v;
        }
    });
    out
}

/// Parallel elementwise binary op on two packed `Array F64` buffers (returns 0 on length mismatch).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_zip_binop_f64(a: i64, b: i64, binop: i64) -> i64 {
    let xs = unsafe { slice_f64(a) };
    let ys = unsafe { slice_f64(b) };
    if xs.len() != ys.len() {
        return 0;
    }
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_F64) };
    let dst = unsafe { slice_f64_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    parallel_chunks(n, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f64, n) };
        match binop {
            BINOP_ADD => for i in lo..hi { d[i] = xs[i] + ys[i]; },
            BINOP_SUB => for i in lo..hi { d[i] = xs[i] - ys[i]; },
            BINOP_MUL => for i in lo..hi { d[i] = xs[i] * ys[i]; },
            BINOP_DIV => for i in lo..hi { d[i] = xs[i] / ys[i]; },
            _ => for i in lo..hi { d[i] = xs[i] + ys[i]; },
        }
    });
    out
}

/// Parallel scalar multiplication `c * a` on a packed `Array F64`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scale_f64(a: i64, scalar_bits: i64) -> i64 {
    let xs = unsafe { slice_f64(a) };
    let c = f64::from_bits(scalar_bits as u64);
    let n = xs.len();
    let out = unsafe { arrays::alloc_uninit_packed(n as i64, FMT_F64) };
    let dst = unsafe { slice_f64_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    parallel_chunks(n, 8192, |_, _, lo, hi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f64, n) };
        for i in lo..hi {
            d[i] = c * xs[i];
        }
    });
    out
}

/// Adjoint of `arrayRowReduceF32`: broadcast `rows` cotangents across `cols` elements each.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_row_broadcast_f32(v: i64, rows: i64, cols: i64) -> i64 {
    let xs = unsafe { slice_f32(v) };
    let r = rows.max(0) as usize;
    let c = cols.max(0) as usize;
    let total = r.saturating_mul(c);
    let out = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_F32) };
    if total == 0 || xs.len() < r {
        if total > 0 {
            for slot in unsafe { slice_f32_mut(out) }.iter_mut() {
                *slot = 0.0;
            }
        }
        return out;
    }
    let dst = unsafe { slice_f32_mut(out) };
    let d_ptr = dst.as_mut_ptr() as usize;
    parallel_chunks(r, 64, |_, _, rlo, rhi| {
        let d = unsafe { std::slice::from_raw_parts_mut(d_ptr as *mut f32, total) };
        for row in rlo..rhi {
            let val = xs[row];
            let base = row * c;
            for col in 0..c {
                d[base + col] = val;
            }
        }
    });
    out
}

/// Adjoint of `arrayGatherF32`: scatter-add `vals` into a zero-initialized `Array F32` of length `n`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_scatter_add_f32(
    n: i64,
    indices: i64,
    vals: i64,
    policy: i64,
) -> i64 {
    let len = n.max(0) as usize;
    let idx = unsafe { slice_i32(indices) };
    let vs = unsafe { slice_f32(vals) };
    let out = unsafe { arrays::alloc_uninit_packed(len as i64, FMT_F32) };
    let dst = unsafe { slice_f32_mut(out) };
    for slot in dst.iter_mut() {
        *slot = 0.0;
    }
    if len == 0 {
        return out;
    }
    let m = idx.len().min(vs.len());
    for i in 0..m {
        if let Some(pos) = resolve_edge(i64::from(idx[i]), len, policy) {
            dst[pos] += vs[i];
        }
    }
    out
}

/// Exact mathematical transpose/adjoint of `cinch_array_stencil2d_f32` for all 4 edge policies
/// and arbitrary step counts `steps`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_stencil2d_adjoint_f32(
    dy_arr: i64,
    frames: i64,
    height: i64,
    width: i64,
    weights_arr: i64,
    edge_policy: i64,
    steps: i64,
) -> i64 {
    let f = frames.max(0) as usize;
    let h = height.max(0) as usize;
    let w = width.max(0) as usize;
    let total = f.saturating_mul(h).saturating_mul(w);
    let dy_in = unsafe { slice_f32(dy_arr) };
    if dy_in.len() != total {
        return 0;
    }
    let w_slice = unsafe { slice_f32(weights_arr) };
    if w_slice.len() != 9 {
        return 0;
    }
    let mut weights = [0.0f32; 9];
    weights.copy_from_slice(w_slice);
    let nsteps = steps.max(0) as usize;
    let out = unsafe { arrays::alloc_uninit_packed(total as i64, FMT_F32) };
    if nsteps == 0 || total == 0 {
        if total > 0 {
            unsafe { slice_f32_mut(out) }.copy_from_slice(dy_in);
        }
        return out;
    }
    let fp = h * w;
    let mut cur = dy_in.to_vec();
    let mut next = vec![0.0f32; total];
    for _ in 0..nsteps {
        for slot in next.iter_mut() {
            *slot = 0.0;
        }
        let n_ptr = next.as_mut_ptr() as usize;
        let c_ptr = cur.as_ptr() as usize;
        parallel_chunks(f, 1, |_, _, flo, fhi| {
            let src_all = unsafe { std::slice::from_raw_parts(c_ptr as *const f32, total) };
            let dst_all = unsafe { std::slice::from_raw_parts_mut(n_ptr as *mut f32, total) };
            for fr in flo..fhi {
                let base = fr * fp;
                let src = &src_all[base..base + fp];
                let dst = &mut dst_all[base..base + fp];
                for y in 0..h {
                    let yi = y as i64;
                    for x in 0..w {
                        let xi = x as i64;
                        let g = src[y * w + x];
                        for dy in -1..=1i64 {
                            let Some(ry) = resolve_edge(yi + dy, h, edge_policy) else { continue };
                            for dx in -1..=1i64 {
                                let Some(rx) = resolve_edge(xi + dx, w, edge_policy) else { continue };
                                let wt = weights[((dy + 1) * 3 + (dx + 1)) as usize];
                                dst[ry * w + rx] += wt * g;
                            }
                        }
                    }
                }
            }
        });
        std::mem::swap(&mut cur, &mut next);
    }
    unsafe { slice_f32_mut(out) }.copy_from_slice(&cur);
    out
}

// ---------------------------------------------------------------------------
// Unit tests: Reference Evaluator vs Multicore CPU agreement across all ops
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn make_f32(vals: &[f32]) -> i64 {
        let a = unsafe { arrays::alloc_uninit_packed(vals.len() as i64, FMT_F32) };
        if !vals.is_empty() {
            unsafe { slice_f32_mut(a) }.copy_from_slice(vals);
        }
        a
    }

    unsafe fn make_i32(vals: &[i32]) -> i64 {
        let a = unsafe { arrays::alloc_uninit_packed(vals.len() as i64, FMT_I32) };
        if !vals.is_empty() {
            unsafe { slice_i32_mut(a) }.copy_from_slice(vals);
        }
        a
    }

    unsafe fn make_u32(vals: &[u32]) -> i64 {
        let a = unsafe { arrays::alloc_uninit_packed(vals.len() as i64, FMT_U32) };
        if !vals.is_empty() {
            unsafe { slice_u32_mut(a) }.copy_from_slice(vals);
        }
        a
    }

    unsafe fn make_u8(vals: &[u8]) -> i64 {
        let a = unsafe { arrays::alloc_uninit_packed(vals.len() as i64, FMT_U8) };
        if !vals.is_empty() {
            unsafe { slice_u8_mut(a) }.copy_from_slice(vals);
        }
        a
    }

    #[test]
    fn dot_product_exact_tree_matches_reference_and_cpu_bit_for_bit() {
        let n = 65_537;
        let xs: Vec<f32> = (0..n).map(|i| ((i % 97) as f32 - 48.0) * 0.125).collect();
        let ys: Vec<f32> = (0..n).map(|i| ((i % 53) as f32 - 26.0) * 0.25).collect();
        unsafe {
            let a = make_f32(&xs);
            let b = make_f32(&ys);
            cinch_array_set_exec_mode(1);
            let ref_bits = cinch_array_dot_f32(a, b, 0);
            cinch_array_set_exec_mode(0);
            let cpu_bits = cinch_array_dot_f32(a, b, 0);
            assert_eq!(ref_bits, cpu_bits, "exact binary tree dot product must agree bit-for-bit");
            let relaxed = f32::from_bits(cinch_array_dot_f32(a, b, 1) as u32);
            let exact = f32::from_bits(cpu_bits as u32);
            assert!((relaxed - exact).abs() < 1e-2, "relaxed {relaxed} vs exact {exact}");
            rc::cinch_drop(a);
            rc::cinch_drop(b);
        }
    }

    #[test]
    fn scan_filter_histogram_and_row_reductions_agree_between_reference_and_cpu() {
        let n = 50_000;
        let data_i32: Vec<i32> = (0..n).map(|i| ((i as i32 * 37) % 100) - 50).collect();
        let data_u8: Vec<u8> = (0..n).map(|i| if i % 3 == 0 { 42 } else { (i % 256) as u8 }).collect();
        unsafe {
            let a = make_i32(&data_i32);
            let pixels = make_u8(&data_u8);

            // Exclusive & inclusive scan
            for inc in [0, 1] {
                cinch_array_set_exec_mode(1);
                let ref_s = cinch_array_scan_i32(a, REDUCE_ADD_WRAP, inc);
                cinch_array_set_exec_mode(0);
                let cpu_s = cinch_array_scan_i32(a, REDUCE_ADD_WRAP, inc);
                assert_eq!(slice_i32(ref_s), slice_i32(cpu_s));
                rc::cinch_drop(ref_s);
                rc::cinch_drop(cpu_s);
            }

            // Stable compaction
            cinch_array_set_exec_mode(1);
            let ref_f = cinch_array_filter_i32_gt(a, 10);
            cinch_array_set_exec_mode(0);
            let cpu_f = cinch_array_filter_i32_gt(a, 10);
            assert_eq!(slice_i32(ref_f), slice_i32(cpu_f));
            assert!(slice_i32(cpu_f).len() <= n);
            rc::cinch_drop(ref_f);
            rc::cinch_drop(cpu_f);

            // Histogram with hot bin 42
            cinch_array_set_exec_mode(1);
            let ref_h = cinch_array_histogram_u8(pixels, 256);
            cinch_array_set_exec_mode(0);
            let cpu_h = cinch_array_histogram_u8(pixels, 256);
            assert_eq!(slice_u32(ref_h), slice_u32(cpu_h));
            assert_eq!(slice_u32(cpu_h).iter().map(|&x| x as usize).sum::<usize>(), n);
            rc::cinch_drop(ref_h);
            rc::cinch_drop(cpu_h);

            rc::cinch_drop(a);
            rc::cinch_drop(pixels);
        }
    }

    #[test]
    fn row_reduce_switches_schedule_for_many_short_vs_few_long_rows() {
        unsafe {
            let many_short = make_f32(&vec![1.0f32; 256 * 128]);
            cinch_array_set_exec_mode(0);
            let out1 = cinch_array_row_reduce_f32(many_short, 256, 128, REDUCE_F32_ADD_EXACT_TREE);
            let rep1 = last_report();
            assert!(rep1.schedule.contains("outer-parallel"), "expected outer-parallel: {}", rep1.schedule);
            assert!(slice_f32(out1).iter().all(|&v| v == 128.0));
            rc::cinch_drop(out1);
            rc::cinch_drop(many_short);

            let few_long = make_f32(&vec![1.0f32; 2 * 65536]);
            let out2 = cinch_array_row_reduce_f32(few_long, 2, 65536, REDUCE_F32_ADD_EXACT_TREE);
            let rep2 = last_report();
            assert!(rep2.schedule.contains("inner-parallel"), "expected inner-parallel: {}", rep2.schedule);
            assert_eq!(slice_f32(out2), &[65536.0, 65536.0]);
            rc::cinch_drop(out2);
            rc::cinch_drop(few_long);
        }
    }

    #[test]
    fn stencil_pingpong_and_spmv_with_empty_rows_agree_with_reference() {
        unsafe {
            let weights = make_i32(&[1, 1, 1, 1, 1, 1, 1, 1, 1]);
            let imgs: Vec<u8> = (0..(8 * 32 * 32)).map(|i| (i * 13 % 256) as u8).collect();
            let a = make_u8(&imgs);
            cinch_array_set_exec_mode(1);
            let ref_st = cinch_array_stencil2d_u8(a, 8, 32, 32, weights, 9, EDGE_CLAMP, 4);
            cinch_array_set_exec_mode(0);
            let cpu_st = cinch_array_stencil2d_u8(a, 8, 32, 32, weights, 9, EDGE_CLAMP, 4);
            assert_eq!(slice_u8(ref_st), slice_u8(cpu_st));
            let rep = last_report();
            assert_eq!(rep.kernel_launches, 4);
            assert!(rep.peak_temp_bytes <= 8 * 32 * 32);
            rc::cinch_drop(ref_st);
            rc::cinch_drop(cpu_st);
            rc::cinch_drop(a);
            rc::cinch_drop(weights);

            // SpMV with empty rows (rows 1 and 3 are empty!)
            let vals = make_f32(&[2.0, 3.0, 4.0, 5.0]);
            let cols = make_u32(&[0, 2, 1, 2]);
            let row_ptr = make_u32(&[0, 2, 2, 4, 4]);
            let x = make_f32(&[10.0, 20.0, 30.0]);
            let y = cinch_array_spmv_csr_f32(vals, cols, row_ptr, x, 0);
            assert_eq!(slice_f32(y), &[110.0, 0.0, 230.0, 0.0]);
            rc::cinch_drop(y);
            rc::cinch_drop(vals);
            rc::cinch_drop(cols);
            rc::cinch_drop(row_ptr);
            rc::cinch_drop(x);
        }
    }

    #[test]
    fn empty_axes_scatter_collisions_and_aliased_strided_views() {
        unsafe {
            // Empty axes
            let empty_f32 = make_f32(&[]);
            let r_empty = cinch_array_row_reduce_f32(empty_f32, 4, 0, REDUCE_F32_ADD_EXACT_TREE);
            assert_eq!(slice_f32(r_empty), &[0.0, 0.0, 0.0, 0.0]);
            rc::cinch_drop(r_empty);
            rc::cinch_drop(empty_f32);

            // Scatter combine collisions (all 1000 updates hit bin 2!)
            let dest = make_i32(&[10, 20, 30, 40]);
            let idx = make_i32(&vec![2i32; 1000]);
            let vals = make_i32(&vec![3i32; 1000]);
            let out = cinch_array_scatter_combine_i32(dest, idx, vals, REDUCE_ADD_WRAP);
            assert_eq!(slice_i32(out), &[10, 20, 3030, 40]);
            assert!(last_report().reused_in_place, "unique dest should be reused in place");
            rc::cinch_drop(out);
            rc::cinch_drop(dest);
            rc::cinch_drop(idx);
            rc::cinch_drop(vals);

            // Aliased strided view dot product (dot of even elements with odd elements of same buffer)
            let buf = make_f32(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
            let dot_bits = cinch_array_strided_dot_f32(buf, 0, 2, buf, 1, 2, 3, 0);
            let dot = f32::from_bits(dot_bits as u32);
            assert_eq!(dot, 1.0 * 2.0 + 3.0 * 4.0 + 5.0 * 6.0);
            rc::cinch_drop(buf);
        }
    }
}
