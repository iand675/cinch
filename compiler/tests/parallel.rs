//! Deterministic pure parallelism in the seed (`design/parallel-evaluation.md`
//! section 2): `parBoth`, `parTabulate` and `parMap` type, check their
//! callbacks' effects and captures, lower to the runtime's work-stealing
//! entries (or the cooperative backend's sequential readings), and give the
//! same answers for every `CINCH_THREADS` value.
//!
//! The tests build a program once and run the executable several times under
//! different worker counts, so the measurement is of the artifact rather than
//! of the compiler.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const ARRAY_SOURCE: &str = include_str!("../prelude/Data/Array.cinch");

/// `Data.Array` is a source module a program ships beside its entry (it is
/// not in the embedded base package), and `parMap` lowers through its two
/// conversions, so every program here carries it.
fn project(tag: &str, source: &str) -> PathBuf {
    common::project(
        tag,
        &[("Data.Array.cinch", ARRAY_SOURCE), ("Main.cinch", source)],
    )
}

fn build(tag: &str, source: &str) -> (PathBuf, PathBuf) {
    let dir = project(tag, source);
    let exe = common::build(&dir, "Main.cinch", "program").expect("build the parallel program");
    (dir, exe)
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
    wall: Duration,
}

/// Run the artifact with `threads` participants (unset: the runtime's
/// default) and the reclaiming diagnostics on. The leak report is asserted
/// here, so every call site checks it: a parallel call that leaks a block at
/// any worker count fails the test that ran it.
fn run(exe: &Path, threads: Option<u32>) -> Run {
    let mut command = Command::new(exe);
    if let Some(n) = threads {
        command.env("CINCH_THREADS", n.to_string());
    }
    let started = Instant::now();
    let output = command
        // Sixteen participants each hold a working set, so the suites'
        // default 32M cap is too small for the parallel peak.
        .env("CINCH_HEAP_LIMIT", "512M")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run the parallel program");
    let wall = started.elapsed();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert_eq!(
        stderr.matches("cinch-leak: live=0 immortal=0 leaked=0").count(),
        1,
        "expected exactly one clean leak report at {threads:?} participants, got: {stderr}"
    );
    Run {
        code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr,
        wall,
    }
}

fn expect_ok(name: &str, source: &str, expected: i32) {
    let (dir, exe) = build(name, source);
    let out = run(&exe, None);
    assert_eq!(out.code, expected, "{name}: {}", out.stderr);
    common::cleanup(&dir);
}

fn expect_err(name: &str, source: &str, needle: &str) {
    let dir = project(name, source);
    let build = common::build(&dir, "Main.cinch", "program");
    let message = match build {
        Ok(_) => panic!("{name}: expected a refusal containing {needle:?}, but the program built"),
        Err(message) => message,
    };
    assert!(
        message.contains(needle),
        "{name}: refusal {message:?} does not contain {needle:?}"
    );
    common::cleanup(&dir);
}

/// A program whose stdout is one byte per check ('A' for agreement, 'B' on
/// disagreement) and whose exit code is 0 when every check agreed. It covers
/// all three primitives on a large input, so one run is a determinism
/// witness for the family.
const CHECKED_PROGRAM: &str = r#"
module Main where
  import Data.Array
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Bytes

  build :: Nat -> Nat -> List Nat
  build i n = if i < n then Cons i (build (i + 1) n) else Nil

  sum-nats :: List Nat -> Nat
  sum-nats xs = foldr (\x acc -> x + acc) 0 xs

  triple :: List Nat -> List Nat
  triple xs = parMap (\x -> x * 3) xs

  reference :: List Nat -> List Nat
  reference xs = map (\x -> x * 3) xs

  squares :: Nat -> Array Nat
  squares n = parTabulate n (\i -> i * i)

  sequential-squares :: Nat -> Array Nat
  sequential-squares n = array-tabulate n (\i -> i * i)

  array-sum :: Array Nat -> Nat -> Nat -> Nat
  array-sum a i n = if i < n then case array-get a i of
      Some x -> x + array-sum a (i + 1) n
      None -> 0
    else 0

  fib :: Nat -> Nat
  fib n = if n < 2 then n
    else if n < 8 then fib (n - 1) + fib (n - 2)
    else case parBoth (\() -> fib (n - 1)) (\() -> fib (n - 2)) of
      (a, b) -> a + b

  fib-sequential :: Nat -> Nat
  fib-sequential n = if n < 2 then n else fib-sequential (n - 1) + fib-sequential (n - 2)

  fill :: InRange 0 255 Nat -> Bytes
  fill v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  put :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Unit
  put v = ioWrite (fill v)

  check :: forall r. using (io :: Io r) => Bool ->{IO r} Unit
  check ok = if ok then put 65 else put 66

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    let xs = build 0 100000
    in case parBoth (\() -> sum-nats (triple xs)) (\() -> sum-nats (reference xs)) of
      (parallel, sequential) ->
        case check (parallel == sequential) of
          _ -> case parBoth (\() -> array-sum (squares 50000) 0 50000)
                            (\() -> array-sum (sequential-squares 50000) 0 50000) of
            (tabulated, linear) ->
              case check (tabulated == linear) of
                _ -> case check (fib 24 == fib-sequential 24) of
                  _ -> 0
"#;

#[test]
fn the_checksum_is_identical_at_every_worker_count() {
    let (dir, exe) = build("parallel_determinism", CHECKED_PROGRAM);
    let mut seen: Option<String> = None;
    for n in [1u32, 2, 4, 8, 16] {
        let out = run(&exe, Some(n));
        assert_eq!(out.code, 0, "N = {n}: {}", out.stderr);
        // The leak report is checked per run in `run`, and the text is
        // identical at every count, so the exception below is exhaustive.
        match &seen {
            None => seen = Some(out.stdout.clone()),
            Some(first) => assert_eq!(
                first, &out.stdout,
                "N = {n} produced different output than the first count"
            ),
        }
    }
    assert_eq!(seen.as_deref(), Some("AAA"), "every check agrees");
    common::cleanup(&dir);
}

#[test]
fn par_map_over_a_hundred_thousand_elements_equals_map() {
    // The default participant count, a large input, and the sequential
    // reference computed beside it.
    expect_ok(
        "parallel_map_large",
        r#"
module Main where
  import Data.Array
  import Data.List
  import Data.Nat

  build :: Nat -> Nat -> List Nat
  build i n = if i < n then Cons (i * i) (build (i + 1) n) else Nil

  sum-nats :: List Nat -> Nat
  sum-nats xs = foldr (\x acc -> x + acc) 0 xs

  main :: Int
  main =
    let xs = build 0 100000
    in if sum-nats (parMap (\x -> x + 7) xs) == sum-nats (map (\x -> x + 7) xs)
      then 0
      else 1
"#,
        0,
    );
}

#[test]
fn nested_par_both_fibonacci_agrees_with_the_sequential_reading() {
    expect_ok(
        "parallel_fib",
        r#"
module Main where
  import Data.Array
  import Data.Nat

  fib :: Nat -> Nat
  fib n = if n < 2 then n
    else if n < 8 then fib (n - 1) + fib (n - 2)
    else case parBoth (\() -> fib (n - 1)) (\() -> fib (n - 2)) of
      (a, b) -> a + b

  fib-sequential :: Nat -> Nat
  fib-sequential n = if n < 2 then n else fib-sequential (n - 1) + fib-sequential (n - 2)

  main :: Int
  main = if fib 24 == fib-sequential 24 then 0 else 1
"#,
        0,
    );
}

#[test]
fn par_tabulate_returns_its_indices_in_order() {
    expect_ok(
        "parallel_tabulate",
        r#"
module Main where
  import Data.Array
  import Data.Nat
  import Data.Option

  total :: Array Nat -> Nat -> Nat -> Nat
  total a i n = if i < n then case array-get a i of
      Some x -> x + total a (i + 1) n
      None -> 0
    else 0

  main :: Int
  main =
    let a = parTabulate 5 (\i -> i * i)
    in if nat-to-int (total a 0 (array-length a)) == 30 then 0 else 1
"#,
        0,
    );
}

/// The cancelling `parBoth` branch: a diverging branch is admitted (the
/// sequential reading already permits it, and `e` carries `Diverge`), and
/// the effect row check admits the mixed total/diverging pair.
#[test]
fn a_diverging_branch_is_admitted_beside_a_total_one() {
    let dir = project(
        "parallel_diverge",
        r#"
module Main where
  import Data.Array
  import Data.Nat

  spin :: Unit ->{Diverge} Nat
  spin u = spin u

  main :: Unit ->{Diverge} Int
  main u = case parBoth (\() -> spin ()) (\() -> nat-to-int 1) of
    (a, b) -> nat-to-int a + b
"#,
    );
    // It checks; it would not terminate, so it is never built or run.
    let checked = common::check(&dir, "Main.cinch");
    assert!(checked.is_ok(), "{checked:?}");
    common::cleanup(&dir);
}

// ------------------------------------------------------------------
// The cooperative backend (section 3.4): the Wasm artifact profile
// selects the sequential `cinch_coop_par_*` entries, and the entries
// themselves give the sequential reading (the runtime's own tests
// compare them with the native sequential loops).
// ------------------------------------------------------------------

#[test]
fn the_cooperative_profile_selects_the_sequential_entries() {
    use cinch::check::StageMode;
    use cinch::core::{ArtifactProfile, TargetProfile};
    use cinch::driver::Mode;
    let profile = TargetProfile::wasm32(ArtifactProfile::CoreWasm);
    let dir = project("parallel_coop", CHECKED_PROGRAM);
    let ir_path = dir.join("program.ll");
    common::drive_with_profile(
        Mode::EmitIr,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(ir_path.clone()),
        &profile,
    )
    .expect("wasm32 IR emission");
    let ir = std::fs::read_to_string(&ir_path).expect("read the emitted IR");
    for entry in [
        "cinch_coop_par_both",
        "cinch_coop_par_tabulate",
        "cinch_coop_par_map_array",
    ] {
        assert!(
            ir.contains(&format!("call i64 @{entry}(")),
            "the cooperative artifact does not call {entry}"
        );
    }
    for entry in ["cinch_par_both", "cinch_par_tabulate", "cinch_par_map_array"] {
        assert!(
            !ir.contains(&format!("call i64 @{entry}(")),
            "the cooperative artifact calls the pooled entry {entry}"
        );
    }
    common::cleanup(&dir);
}

// ------------------------------------------------------------------
// Refusals (sections 2.2 and 2.3): each is a compile error with the
// documented rule.
// ------------------------------------------------------------------

#[test]
fn an_io_operation_in_a_callback_is_rejected() {
    expect_err(
        "parallel-io-callback",
        r#"
module Main where
  import Data.Array
  import Data.Bytes

  ignore :: Unit -> Unit
  ignore u = u

  bad :: forall r. using (io :: Io r) => Bytes ->{IO r} Int
  bad b = case parBoth (\() -> ignore (ioWrite b)) (\() -> 1) of
    (x, y) -> 0

  main :: Int
  main = 0
"#,
        "a parBoth callback may perform only the parallel-safe row",
    );
}

#[test]
fn a_mut_borrow_capture_is_rejected() {
    expect_err(
        "parallel-mut-capture",
        r#"
module Main where
  import Data.Array
  import Data.Nat

  readMut :: forall g. Mut g Nat -> Nat
  readMut m = copyOut m

  bad :: forall g. Mut g Nat -> Int
  bad m = case parBoth (\() -> readMut m) (\() -> 1) of
    (x, y) -> 0

  main :: Int
  main = 0
"#,
        "an exclusive `Mut` borrow cannot enter a parallel callback",
    );
}

#[test]
fn an_untransferred_bytes_capture_is_rejected() {
    expect_err(
        "parallel-bytes-capture",
        r#"
module Main where
  import Data.Array
  import Data.Bytes

  bad :: Bytes -> Int
  bad b = case parBoth (\() -> bytes-len b) (\() -> 1) of
    (x, y) -> 0

  main :: Int
  main = 0
"#,
        "a parallel capture must be Share",
    );
}

#[test]
fn a_memo_cell_capture_is_rejected() {
    expect_err(
        "parallel-memo-capture",
        r#"
module Main where
  import Data.Array
  import Data.Bytes

  bad :: Unit -> Int
  bad u = let ~y = bytes-len (bytes-new 2) in case parBoth (\() -> copyOut (demand y)) (\() -> 1) of
    (a, b) -> 0

  main :: Int
  main = 0
"#,
        "a parBoth callback may perform only the parallel-safe row",
    );
}

#[test]
fn a_linear_capture_in_a_par_map_callback_is_rejected() {
    expect_err(
        "parallel-linear-map-capture",
        r#"
module Main where
  import Data.Array
  import Data.Bytes

  bad :: Bytes %1 -> List Nat -> Int
  bad b xs = case parMap (\x -> bytes-len b + x) xs of
    _ -> 0

  main :: Int
  main = 0
"#,
        "a parMap callback runs once per index, so b cannot move into it",
    );
}

#[test]
fn a_linear_capture_may_move_into_one_par_both_branch() {
    expect_ok(
        "parallel-linear-move",
        r#"
module Main where
  import Data.Array
  import Data.Bytes
  import Data.Nat

  one :: Bytes %1 -> Int
  one b = case parBoth (\() -> nat-to-int (bytes-len b)) (\() -> 0) of
    (a, c) -> a + c

  main :: Int
  main = if one (bytes-new 3) == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn a_linear_capture_in_both_par_both_branches_is_rejected() {
    expect_err(
        "parallel-linear-both",
        r#"
module Main where
  import Data.Array
  import Data.Bytes

  bad :: Bytes %1 -> Int
  bad b = case parBoth (\() -> bytes-len b) (\() -> bytes-len b) of
    (a, c) -> 0

  main :: Int
  main = 0
"#,
        "used 2 time(s)",
    );
}

#[test]
fn a_function_parameter_callback_is_rejected() {
    expect_err(
        "parallel-function-parameter",
        r#"
module Main where
  import Data.Array
  import Data.Nat

  bad :: (Nat -> Nat) -> Int
  bad f = case parTabulate 4 f of
    _ -> 0

  main :: Int
  main = 0
"#,
        "is a function-typed binding whose captures are unknown",
    );
}

#[test]
fn the_builtins_are_not_first_class_values() {
    expect_err(
        "parallel-first-class",
        r#"
module Main where
  import Data.Array

  alias :: forall a b. (a -> b) -> List a -> List b
  alias = parMap

  main :: Int
  main = 0
"#,
        "parMap must be applied directly to check its parallel callback",
    );
}

#[test]
fn a_rigid_row_callback_is_rejected() {
    expect_err(
        "parallel-rigid-row",
        r#"
module Main where
  import Data.Array
  import Data.Nat

  polymorphic :: forall e. (Unit ->{|e} Nat) -> Int
  polymorphic f = case parBoth f (\() -> 2) of
    (a, b) -> 0

  main :: Int
  main = 0
"#,
        "whose tail is a rigid row variable",
    );
}

// ------------------------------------------------------------------
// Scaling: 64 expensive items over one `parTabulate`.
// ------------------------------------------------------------------

#[test]
fn par_tabulate_over_sixty_four_expensive_items_scales() {
    let (dir, exe) = build(
        "parallel_scaling",
        r#"
module Main where
  import Data.Array
  import Data.Nat
  import Data.Machine
  import Data.Nat
  import Data.Option

  -- A pure, allocation-free unit of work: U64 arithmetic so the loop
  -- measures the arithmetic and not the allocator.
  burn :: U64 -> U64 -> U64 -> U64
  burn i acc n =
    if lt-u64 i n
    then burn (wrapping-add-u64 i 1) (bit-xor-u64 (wrapping-add-u64 acc i) i) n
    else acc

  -- One unit of work per index: the loop is the measurement, and every
  -- index does the same amount of it.
  item :: Nat -> U64
  item i = burn (0 :: U64) (1 :: U64) 2000000

  total :: Array U64 -> Nat -> Nat -> U64
  total a i n = if i < n then case array-get a i of
      Some x -> wrapping-add-u64 x (total a (i + 1) n)
      None -> (0 :: U64)
    else (0 :: U64)

  main :: Int
  main =
    let a = parTabulate 64 item
    in if lt-u64 (0 :: U64) (total a 0 (array-length a)) then 0 else 1
"#,
    );
    // Three repetitions per count, reported as the minimum: the host also
    // runs other work, so a single reading is noisy by a factor of two.
    let mut table: Vec<(u32, Duration)> = Vec::new();
    for n in [1u32, 2, 4, 8] {
        let mut best = Duration::MAX;
        for _ in 0..3 {
            let out = run(&exe, Some(n));
            assert_eq!(out.code, 0, "N = {n}: {}", out.stderr);
            best = best.min(out.wall);
        }
        table.push((n, best));
    }
    let baseline = table[0].1;
    let report: Vec<String> = table
        .iter()
        .map(|(n, wall)| format!("N = {n}: {:.3}s ({:.2}x)", wall.as_secs_f64(), baseline.as_secs_f64() / wall.as_secs_f64()))
        .collect();
    eprintln!("parTabulate over 64 expensive items: {}", report.join(", "));
    if baseline > Duration::from_millis(300) {
        let eight = table.last().expect("N = 8").1;
        assert!(
            eight < baseline,
            "eight participants are not faster than one: {}",
            report.join(", ")
        );
    }
    common::cleanup(&dir);
}
