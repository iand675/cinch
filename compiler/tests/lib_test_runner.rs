//! The Minitest-shaped test runner (`compiler/prelude/Effect/Test.cinch`):
//! the case/check/report vocabulary, the assertion set, the suite run
//! and the rendered summary, exercised through the real compiler and a
//! real program.
//!
//! Every test compiles a project: the runner module's source written
//! beside the entry (the driver embeds the prelude family only, so an
//! explicit source module is what a consumer ships until integration
//! folds `Effect.Test` into the family), plus a `Main` that imports it
//! with `import Effect.Test` and runs a suite. The projects are driven
//! like modules.rs/observability.rs.
//!
//! What the programs pin, and where:
//!   vocabulary:  each assertion's `Pass`/`Fail` value, its label and
//!                its rendered message, including both values for
//!                `assert-eq`/`refute-eq` and multi-byte text (the
//!                assertions are VALUES, so an unequal `assert-eq` is a
//!                failure and never a fault);
//!   observation: a case's assertion count is the prefix up to and
//!                including the first failure, the failure reported is
//!                the FIRST one, and an empty body passes with zero
//!                assertions;
//!   aggregation: `run-suite (xs <> ys)` adds the counts and
//!                concatenates the failures in run order, and the empty
//!                suite renders the totals line alone;
//!   report:      the suite runs over I/O, writes one line per case and
//!                renders Minitest's summary (asserted against the
//!                program's captured stdout);
//!   refusals:    a suite without the `Io r` capability in scope, an
//!                `assert-eq` with no `Eq` instance, and an effectful
//!                case body are all refused at check time.
//!
//! The programs that need no I/O take the repo's bitmask convention
//! (check k contributes bit 2^(k-1), `main` sums them), so a nonzero
//! exit code names the failed check.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The module under test, written into every project beside the entry.
const TEST_SRC: &str = include_str!("../prelude/Effect/Test.cinch");

/// A fresh project directory: the runner module plus the `Main`.
fn write_project(name: &str, main: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "cinch_testrunner_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Effect.Test.cinch"), TEST_SRC).expect("write runner module");
    std::fs::write(dir.join("Main.cinch"), main).expect("write Main");
    dir
}

/// A driver invocation on a thread with a large stack (the checker
/// recurses over elaborated code; the suite programs are bigger than a
/// default test thread's stack).
fn drive(mode: Mode, entry: &Path, out: Option<&Path>) -> Result<i32, String> {
    let entry = entry.to_path_buf();
    let out = out.map(|p| p.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &entry, out.as_deref()))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .map_err(|e| e.to_string())
}

/// Compile and run a `Main`; the exit code is the program's `main`
/// result (0 when every check passed, otherwise the bitmask).
fn run_main(name: &str, main: &str) -> i32 {
    let dir = write_project(name, main);
    let code = drive(Mode::Run, &dir.join("Main.cinch"), None)
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"));
    let _ = std::fs::remove_dir_all(&dir);
    code
}

fn expect_exit(name: &str, main: &str, expected: i32) {
    let code = run_main(name, main);
    assert_eq!(code, expected, "{name}: check bitmask {code}");
}

/// Compile a `Main` to an executable, run it, and capture its exit code
/// and stdout.
fn build_and_run(name: &str, main: &str) -> (i32, String) {
    use std::io::Write;
    let dir = write_project(name, main);
    let exe = dir.join("program");
    let code = drive(Mode::Build, &dir.join("Main.cinch"), Some(&exe))
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"));
    assert_eq!(code, 0, "{name}: build failed");
    let mut child = std::process::Command::new(&exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()));
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(b"")
        .expect("feed input");
    let out = child.wait_with_output().expect("wait for child");
    let _ = std::fs::remove_dir_all(&dir);
    (
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// A program expected to be refused at check time; the error text.
fn check_error(name: &str, main: &str) -> String {
    let dir = write_project(name, main);
    let res = drive(Mode::Check, &dir.join("Main.cinch"), None);
    let _ = std::fs::remove_dir_all(&dir);
    match res {
        Ok(_) => panic!("{name}: expected a check error, but the program checked"),
        Err(e) => e,
    }
}

fn expect_err(name: &str, main: &str, needle: &str) {
    let msg = check_error(name, main);
    assert!(
        msg.contains(needle),
        "{name}: error {msg:?} does not contain {needle:?}"
    );
}

// ----------------------------------------------------------------
// 1. The report: counts, the first failing assertion, the rendering
// ----------------------------------------------------------------

/// Two passing cases and one failing case. The failing case's body
/// holds a third check after the failing one, which the runner must
/// not count (the prefix law) and must not report.
#[test]
fn suite_reports_counts_and_names_the_first_failure() {
    let main = r#"module Main where
  import Effect.Test
  import Data.Bool
  import Data.List
  import Data.Option
  import Data.String

  adds :: TestCase
  adds = TestCase "arithmetic.adds" (\u -> [assert-eq "add" (2 :: Int) 2, assert "positive" (2 + 2 == 4)])

  negates :: TestCase
  negates = TestCase "arithmetic.negates" (\u -> [refute "one-is-not-two" (1 == 2)])

  borrows :: TestCase
  borrows = TestCase "arithmetic.borrows" (\u -> [assert "sanity" True, assert-eq "borrow" (3 :: Int) 4, assert "never-counted" True])

  suite :: List TestCase
  suite = [adds, negates, borrows]

  first-failure :: Report -> Option CaseFailure
  first-failure report = case report-failures report of
    Nil -> None
    Cons f rest -> Some f

  names-right :: Report -> Bool
  names-right report = case first-failure report of
    None -> False
    Some f -> failure-case f == "arithmetic.borrows" && failure-assertion f == "borrow" && failure-message f == "expected 3, got 4"

  checks :: Report -> Int
  checks report =
    let c1 = if report-runs report == 3 then 0 else 1 in
    let c2 = if report-assertions report == 5 then 0 else 2 in
    let c3 = if length (report-failures report) == 1 then 0 else 4 in
    let c4 = if report-ok report == False then 0 else 8 in
    let c5 = if names-right report then 0 else 16 in
    c1 + c2 + c3 + c4 + c5

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case run-suite suite of
      report -> case ioWrite (string-to-bytes (render-report report)) of
        w -> checks report
"#;
    let (code, out) = build_and_run("suite-report", main);
    assert_eq!(code, 0, "suite-report: check bitmask {code}");
    assert_eq!(
        out,
        concat!(
            "ok arithmetic.adds\n",
            "ok arithmetic.negates\n",
            "FAIL arithmetic.borrows\n",
            "1) arithmetic.borrows: borrow\n",
            "  expected 3, got 4\n",
            "\n",
            "3 cases, 5 assertions, 1 failures\n"
        ),
        "suite-report: rendered report mismatch"
    );
}

// ----------------------------------------------------------------
// 2. The assertion vocabulary: values, labels, rendered messages
// ----------------------------------------------------------------

/// Every assertion form and both polarities, plus the renderings: the
/// message names both values (Int and String carriers, including
/// multi-byte text) and the label survives on both verdicts. An
/// unequal `assert-eq` is a `Fail` value here, so the program runs to
/// completion instead of faulting.
#[test]
fn assertion_vocabulary_reports_instead_of_faulting() {
    let main = r#"module Main where
  import Effect.Test
  import Data.Bool

  is-ok :: Check -> Bool
  is-ok c = case c of
    Pass l -> True
    Fail l m -> False

  is-fail :: Check -> Bool
  is-fail c = case c of
    Pass l -> False
    Fail l m -> True

  label-of :: Check -> String
  label-of c = case c of
    Pass l -> l
    Fail l m -> l

  message-of :: Check -> String
  message-of c = case c of
    Pass l -> ""
    Fail l m -> m

  c1 :: Int
  c1 = if is-ok (assert "holds" True) && is-fail (assert "fails" False) && message-of (assert "fails" False) == "expected true, got false" then 0 else 1

  c2 :: Int
  c2 = if is-ok (refute "stays-false" False) && is-fail (refute "is-true" True) && message-of (refute "is-true" True) == "expected false, got true" then 0 else 2

  c3 :: Int
  c3 = if is-ok (assert-eq "equal" (3 :: Int) 3) && is-fail (assert-eq "unequal" (3 :: Int) 4) && message-of (assert-eq "unequal" (3 :: Int) 4) == "expected 3, got 4" then 0 else 4

  c4 :: Int
  c4 = if is-ok (refute-eq "differ" (3 :: Int) 4) && is-fail (refute-eq "same" (3 :: Int) 3) && message-of (refute-eq "same" (3 :: Int) 3) == "expected different values, got 3 and 3" then 0 else 8

  c5 :: Int
  c5 = if message-of (assert-eq "strings" "a" "b") == "expected a, got b" then 0 else 16

  c6 :: Int
  c6 = if label-of (assert-eq "borrow" (3 :: Int) 4) == "borrow" && label-of (assert "held" True) == "held" then 0 else 32

  c7 :: Int
  c7 = if is-fail (assert-eq "utf8" "naïve" "naive") && message-of (assert-eq "utf8" "naïve" "naive") == "expected naïve, got naive" then 0 else 64

  c8 :: Int
  c8 = if is-ok (assert-eq "utf8-equal" "é" "é") then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#;
    expect_exit("assertion-vocabulary", main, 0);
}

// ----------------------------------------------------------------
// 3. The observation law: prefix counts, first failure, empty body
// ----------------------------------------------------------------

/// The failing case's body: two passes, the failure, then a check the
/// runner must not count (3 assertions, the failing one last in the
/// prefix). Beside it, an empty body (a vacuous pass with zero
/// assertions) and an all-passing body, and the outcome ORDER of the
/// run.
#[test]
fn assertion_count_is_the_prefix_through_the_first_failure() {
    let main = r#"module Main where
  import Effect.Test
  import Data.Bool
  import Data.List
  import Data.Option

  early :: TestCase
  early = TestCase "early" (\u -> [assert "first" True, assert "second" True, assert-eq "third" (1 :: Int) 2, assert "fourth" True])

  empty :: TestCase
  empty = TestCase "empty" (\u -> Nil)

  all-pass :: TestCase
  all-pass = TestCase "all-pass" (\u -> [assert "p1" True, assert "p2" True, assert "p3" True, assert "p4" True, assert "p5" True])

  suite :: List TestCase
  suite = [early, empty, all-pass]

  outcome-at :: List CaseOutcome -> Int -> Option CaseOutcome
  outcome-at os i = case os of
    Nil -> None
    Cons o rest -> if i == 0 then Some o else outcome-at rest (i - 1)

  at :: Report -> Int -> Option CaseOutcome
  at report i = outcome-at (report-outcomes report) i

  name-at :: Report -> Int -> String
  name-at report i = case at report i of
    None -> ""
    Some o -> outcome-name o

  ok-at :: Report -> Int -> Bool
  ok-at report i = case at report i of
    None -> False
    Some o -> outcome-ok o

  assertions-at :: Report -> Int -> Int
  assertions-at report i = case at report i of
    None -> -1
    Some o -> outcome-assertions o

  first-failure :: Report -> Option CaseFailure
  first-failure report = case report-failures report of
    Nil -> None
    Cons f rest -> Some f

  first-right :: Report -> Bool
  first-right report = case first-failure report of
    None -> False
    Some f -> failure-case f == "early" && failure-assertion f == "third" && failure-message f == "expected 1, got 2"

  checks :: Report -> Int
  checks report =
    let c1 = if report-runs report == 3 then 0 else 1 in
    let c2 = if report-assertions report == 8 then 0 else 2 in
    let c3 = if length (report-failures report) == 1 then 0 else 4 in
    let c4 = if first-right report then 0 else 8 in
    let c5 = if assertions-at report 0 == 3 then 0 else 16 in
    let c6 = if ok-at report 1 && assertions-at report 1 == 0 then 0 else 32 in
    let c7 = if ok-at report 2 && assertions-at report 2 == 5 then 0 else 64 in
    let c8 = if name-at report 0 == "early" && name-at report 1 == "empty" && name-at report 2 == "all-pass" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case run-suite suite of
      report -> checks report
"#;
    expect_exit("observation-law", main, 0);
}

// ----------------------------------------------------------------
// 4. Aggregation: the suite is the list's catamorphism
// ----------------------------------------------------------------

/// Counts add and failures concatenate in run order across `xs <> ys`,
/// the empty suite is an empty report, and the rendered totals line is
/// the Minitest shape for the empty, the all-passing and the failing
/// reports.
#[test]
fn suite_appends_counts_and_failures_in_run_order() {
    let main = r#"module Main where
  import Effect.Test
  import Algebra.Semigroup
  import Data.Bool
  import Data.List

  pass :: String -> TestCase
  pass name = TestCase name (\u -> [assert "a" True, assert "b" True])

  fail-once :: String -> String -> String -> TestCase
  fail-once name label msg = TestCase name (\u -> [assert "before" True, Fail label msg])

  xs :: List TestCase
  xs = [pass "xs-pass", fail-once "xs-fail" "bx" "bx-msg"]

  ys :: List TestCase
  ys = [pass "ys-pass", fail-once "ys-fail" "by" "by-msg"]

  whole :: List TestCase
  whole = xs <> ys

  failure-pairs :: List CaseFailure -> List (String, String)
  failure-pairs fs = case fs of
    Nil -> Nil
    Cons f rest -> Cons (failure-case f, failure-message f) (failure-pairs rest)

  failures-of :: Report -> List (String, String)
  failures-of report = failure-pairs (report-failures report)

  order-right :: List (String, String) -> Bool
  order-right fs = case fs of
    Nil -> False
    Cons f rest -> case rest of
      Nil -> False
      Cons g tail -> case f of
        (name1, msg1) -> case g of
          (name2, msg2) -> name1 == "xs-fail" && msg1 == "bx-msg" && name2 == "ys-fail" && msg2 == "by-msg" && length tail == 0

  expected-render :: String
  expected-render = "1) xs-fail: bx\n  bx-msg\n2) ys-fail: by\n  by-msg\n\n4 cases, 8 assertions, 2 failures\n"

  checks :: Report -> Report -> Report -> Report -> Report -> Int
  checks whole-r xs-r ys-r empty-r ok-r =
    let c1 = if report-runs whole-r == report-runs xs-r + report-runs ys-r then 0 else 1 in
    let c2 = if report-assertions whole-r == report-assertions xs-r + report-assertions ys-r then 0 else 2 in
    let c3 = if report-runs whole-r == 4 && report-assertions whole-r == 8 then 0 else 4 in
    let c4 = if order-right (failures-of whole-r) then 0 else 8 in
    let c5 = if report-runs empty-r == 0 && report-assertions empty-r == 0 && length (report-failures empty-r) == 0 && report-ok empty-r then 0 else 16 in
    let c6 = if render-report empty-r == "0 cases, 0 assertions, 0 failures\n" then 0 else 32 in
    let c7 = if render-report whole-r == expected-render then 0 else 64 in
    let c8 = if render-report ok-r == "1 cases, 2 assertions, 0 failures\n" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case run-suite whole of
      whole-r -> case run-suite xs of
        xs-r -> case run-suite ys of
          ys-r -> case run-suite Nil of
            empty-r -> case run-suite [pass "solo"] of
              ok-r -> checks whole-r xs-r ys-r empty-r ok-r
"#;
    expect_exit("aggregation-law", main, 0);
}

// ----------------------------------------------------------------
// 5. Refusals
// ----------------------------------------------------------------

/// The reporting row is exactly `{IO r}`: a suite run without the
/// capability in scope is refused, never silently ambient.
#[test]
fn running_a_suite_without_the_capability_is_refused() {
    expect_err(
        "no-capability",
        r#"module Main where
  import Effect.Test
  import Data.List

  main :: Unit ->{IO r} Int
  main u = case run-suite Nil of
    report -> 0
"#,
        "capability in scope",
    );
}

/// `assert-eq` states the constraints it uses: a carrier with no `Eq`
/// instance is refused at the call, naming the class and the carrier
/// the want could not solve (the parenthesized spelling, so a broken
/// family module's own `no instance for Eq ...` error cannot satisfy
/// this pin).
#[test]
fn assert_eq_without_eq_evidence_is_refused() {
    expect_err(
        "no-eq",
        r#"module Main where
  import Effect.Test

  identity-check :: Check
  identity-check = assert-eq "identity" (\n -> n) (\n -> n)

  main :: Int
  main = 0
"#,
        "no instance for Eq (",
    );
}

/// A case body is pure: a body that performs the runtime capability
/// contradicts the suite's declared row and is refused.
#[test]
fn an_effectful_case_body_is_refused() {
    expect_err(
        "effectful-body",
        r#"module Main where
  import Effect.Test
  import Data.List
  import Data.String

  noisy :: TestCase
  noisy = TestCase "noisy" (\u -> case ioWrite (string-to-bytes "x\n") of
    w -> Nil)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case run-suite [noisy] of
      report -> 0
"#,
        "effect rows do not match",
    );
}
