//! Effect.Benchmark consumer semantics (P34): the measured interval
//! around an `IO r` action, value transparency, the ordering between a
//! busy and a trivial action (never an absolute duration), and the
//! table renderer's per-label rows and totals.
//!
//! Every program is the real module source (`Effect/Benchmark.cinch`)
//! beside a `Main`, a real multi-file project compiled and run through
//! the same driver the CLI uses (`Text/Scan.cinch` is written beside it
//! too, because it is the toolkit module the table checks read with and
//! it is not part of the embedded prelude). The busy and trivial
//! intervals are compared only to each other, so the checks do not
//! depend on machine speed: a busy action (a bignum counter accumulated
//! many times) must measure a strictly positive span that exceeds the
//! trivial action's.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};

const BENCHMARK: &str = include_str!("../prelude/Effect/Benchmark.cinch");
const SCAN: &str = include_str!("../prelude/Text/Scan.cinch");

/// A temp project: the module under test and the scan toolkit beside the
/// entry, so `import Effect.Benchmark` / `import Text.Scan` resolve to
/// these sources and every other import falls back to the embedded
/// prelude.
fn write_project(name: &str, main: &str) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "cinch_bench_{}_{}_{}",
        name,
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Effect.Benchmark.cinch"), BENCHMARK).expect("write Effect.Benchmark");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN).expect("write Text.Scan");
    std::fs::write(dir.join("main.cinch"), main).expect("write main");
    dir
}

/// Run a driver invocation on a thread with a large stack (the checker
/// recurses over a program's elaborated code).
fn drive(mode: Mode, entry: &Path, out: Option<&Path>) -> i32 {
    let entry = entry.to_path_buf();
    let out = out.map(|p| p.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &entry, out.as_deref()))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .unwrap_or_else(|e| panic!("compilation failed: {e}"))
}

/// Compile and run a `Main`; the exit code is the bitmask `main` sums.
fn run_main(name: &str, main: &str) -> i32 {
    let dir = write_project(name, main);
    let code = drive(Mode::Run, &dir.join("main.cinch"), None);
    let _ = std::fs::remove_dir_all(&dir);
    code
}

fn expect_exit(name: &str, main: &str, expected: i32) {
    let code = run_main(name, main);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

/// Run a `Main` whose action writes to stdout and assert the captured
/// output (the `run_program_io` shape, through a real built executable).
fn expect_output(name: &str, main: &str, expected_code: i32, expected_out: &str) {
    let dir = write_project(name, main);
    let exe = dir.join("program");
    let code = drive(Mode::Build, &dir.join("main.cinch"), Some(&exe));
    assert_eq!(code, 0, "{name}: build failed");
    let out = std::process::Command::new(&exe)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("{name}: spawn exe: {e}"))
        .wait_with_output()
        .unwrap_or_else(|e| panic!("{name}: wait exe: {e}"));
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(out.status.code().unwrap_or(1), expected_code, "{name}: exit");
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert_eq!(text, expected_out, "{name}: expected output {expected_out:?}, got {text:?}");
}

#[test]
fn measure_returns_the_value_and_orders_a_busy_action_above_a_trivial_one() {
    // Checks: 1 the trivial action's value is returned unchanged; 2 the
    // busy action's value is returned unchanged; 4 the trivial elapsed
    // is non-negative; 8 the busy elapsed is strictly positive; 16 the
    // busy elapsed exceeds the trivial elapsed.
    expect_exit(
        "measure-order",
        r#"module Main where

  import Data.Nat
  import Effect.Benchmark

  spin :: Nat -> Nat -> Nat
  spin n acc = if n == 0 then acc else spin (n - 1) (acc + 1)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let t = benchmark-measure "trivial" (\() -> 7) in
    case t of
      (tr, tv) ->
        let b = benchmark-measure "busy" (\() -> spin 500000 0) in
        case b of
          (br, bv) ->
            let c1 = if tv == 7 then 0 else 1 in
            let c2 = if bv == 500000 then 0 else 2 in
            let c3 = if benchmark-elapsed-ns tr >= 0 then 0 else 4 in
            let c4 = if benchmark-elapsed-ns br > 0 then 0 else 8 in
            let c5 = if benchmark-elapsed-ns br > benchmark-elapsed-ns tr then 0 else 16 in
            c1 + c2 + c3 + c4 + c5
"#,
        0,
    );
}

#[test]
fn table_has_a_row_per_label_and_a_total_that_is_the_sum_of_the_rows() {
    // Checks: 1 the trivial label is present; 2 the busy label is
    // present; 4 the total row is present; 8 the table has one header
    // line, one row per report and one total line; 16 the total row
    // carries the sum of the rows' elapsed values.
    expect_exit(
        "table-rows",
        r#"module Main where

  import Data.List
  import Data.Nat
  import Effect.Benchmark
  import Text.Scan

  spin :: Nat -> Nat -> Nat
  spin n acc = if n == 0 then acc else spin (n - 1) (acc + 1)

  measured :: forall r a. using (io :: Io r) => String -> (Unit ->{IO r} a) ->{IO r} BenchmarkReport
  measured l w = case benchmark-measure l w of
    (r, _) -> r

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let tr = measured "trivial" (\() -> ()) in
    let br = measured "busy" (\() -> spin 20000 0) in
    let table = benchmark-table (Cons tr (Cons br Nil)) in
    let total = benchmark-elapsed-ns tr + benchmark-elapsed-ns br in
    let c1 = if string-contains "trivial" table then 0 else 1 in
    let c2 = if string-contains "busy" table then 0 else 2 in
    let c3 = if string-contains "total" table then 0 else 4 in
    let c4 = if length (string-lines table) == 4 then 0 else 8 in
    let c5 = if string-contains (nat-to-string total) table then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
        0,
    );
}

#[test]
fn realtime_runs_the_action_and_orders_the_busy_span_above_the_trivial_one() {
    // The `benchmark-realtime` path: it runs its action (the busy
    // action's write is observable in the captured output) and returns
    // a `Duration`. The check orders the busy duration above the
    // trivial one; it never pins an absolute time.
    expect_output(
        "realtime-order",
        r#"module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Effect.Benchmark
  import Effect.Clock

  spin :: Nat -> Nat -> Nat
  spin n acc = if n == 0 then acc else spin (n - 1) (acc + 1)

  -- One byte whose value depends on the counter, so the accumulation
  -- cannot be discarded: the write consumes its result.
  one-byte :: InRange 0 255 Nat -> Bytes
  one-byte v = bytes-from-list (Cons v Nil)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let triv = benchmark-realtime (\() -> ()) in
    let busy = benchmark-realtime (\() ->
      let v = spin 500000 0 in ioWrite (one-byte (65 + nat-rem v 10))) in
    if duration-millis busy > duration-millis triv then 0 else 1
"#,
        0,
        "A",
    );
}
