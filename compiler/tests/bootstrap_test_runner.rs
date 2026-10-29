//! The bootstrap's own test runner (the P35 prerequisite): the shared
//! contract's `Bootstrap.Test` module compiles, and real programs run it
//! through `cinch::driver`, so the bootstrap can run its own checks
//! instead of relying on a Rust assertion beside it.
//!
//! Two programs are built and RUN. The first declares a hard-coded suite
//! of five checks (three passing, two failing with the distinct codes 41
//! and 42), writes the rendered report to `bootstrap-test-report.txt`
//! through `Effect.Fs`, and returns the failure count as its exit code.
//! The second declares an all-passing suite and returns 0. The test
//! asserts each exit code exactly, reads the report file back, and
//! asserts its contents: the two failing names, the summary line with
//! the right counts, and a `failed 0` summary for the passing program.
//!
//! The program writes to a relative path, so the test runs it with its
//! project directory as the working directory and the report lands
//! beside it, which is also how `tooling/run-bootstrap-ladder.sh` runs it.

mod common;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const TEST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Test.cinch");
const RUN_TEST: &str = include_str!("../../bootstrap/programs/Run.Test.cinch");
const RUN_TEST_PASS: &str = include_str!("../../bootstrap/programs/Run.TestPass.cinch");

/// The report file the programs write into their project.
const REPORT_NAME: &str = "bootstrap-test-report.txt";

/// Assemble a project from the contract modules plus `src` as `Main`,
/// build it, run it with its project directory as the working directory,
/// and return its exit code and the report file's text.
///
/// The programs write a relative path (`bootstrap-test-report.txt`), so
/// running with `current_dir(dir)` is what puts the report where
/// `read_output` reads it.
fn run_program(tag: &str, src: &str) -> (i32, String) {
    let dir = common::project(
        tag,
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Test.cinch", TEST_SRC),
            ("Main.cinch", src),
        ],
    );
    let exe = common::build(&dir, "Main.cinch", "program")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = std::process::Command::new(&exe)
        .current_dir(&dir)
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap_or_else(|e| panic!("{tag}: run {}: {e}", exe.display()));
    let code = out.status.code().unwrap_or(-1);
    let report = common::read_output(&dir, REPORT_NAME);
    common::cleanup(&dir);
    (code, report)
}

/// The mixed suite (three pass, two fail): the exit code is the failure
/// count (exactly 2), and the report names both failing checks and the
/// counts. The passing checks are not named in the report.
#[test]
fn the_runner_reports_failures_and_encodes_the_failure_count() {
    let (code, report) = run_program("test-run", RUN_TEST);

    assert_eq!(code, 2, "mixed suite: the exit code is the failure count");

    assert!(report.contains("away-by-one"), "report must name the first failing check:\n{report}");
    assert!(
        report.contains("different-names"),
        "report must name the second failing check:\n{report}"
    );
    assert!(
        report.contains("passed 3, failed 2"),
        "report must carry the summary line with the right counts:\n{report}"
    );
    assert!(
        !report.contains("one-plus-one"),
        "the report names failures only, not passing checks:\n{report}"
    );

    assert_eq!(
        report,
        "bootstrap.test:1:1: check failed: away-by-one\n\
         bootstrap.test:1:1: check failed: different-names\n\
         passed 3, failed 2\n",
        "the report is one diagnostic line per failure, then the summary line"
    );
}

/// The all-passing suite: exit 0 and a `failed 0` summary.
#[test]
fn an_all_passing_suite_exits_zero_with_a_zero_failure_summary() {
    let (code, report) = run_program("test-pass", RUN_TEST_PASS);

    assert_eq!(code, 0, "all-passing suite: exit 0");
    assert!(
        report.contains("failed 0"),
        "all-passing suite: the summary reports zero failures:\n{report}"
    );
    assert_eq!(report, "passed 2, failed 0\n", "all-passing suite: report mismatch");
}
