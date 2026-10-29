//! The ladder's differential oracle: `Bootstrap.Eval` interprets the
//! fragment, and `bootstrap/programs/Run.Eval.cinch` drives it as a whole
//! program, so the IR rung's emitted binary can later be checked against a
//! second implementation rather than against itself.
//!
//! The rung's program builds the AST of `examples/compiler/fib.cinch` the
//! way `bootstrap_contract.rs` builds its small tree, evaluates it with
//! `Bootstrap.Eval`, and checks that `fib 10` is 55 and that `main` is 0.
//! It then asserts each named refusal (an `*Unmodeled` node, an unmodeled
//! declaration, an operator outside the fragment, applying a non-function, a
//! case with no matching alternative, an unbound variable, and an arity that
//! disagrees with the declaration) and exercises the fragment's remaining
//! shapes (a lambda
//! with a parameter, a constructor pattern with an argument pattern, a
//! nullary constructor, a wildcard alternative, recursion between two
//! top-level functions, string and character values, and `==` over a
//! comparison's Bool result).
//!
//! Secondary acceptance: `Bootstrap.Parse` exists, so the program also
//! parses the same source bytes (`examples/compiler/fib.cinch`'s text) and
//! evaluates the tree the parser produced, asserting the parsed program has
//! no unmodeled nodes and that `fib 10` is 55 and `main` is 0 in it too.
//! The interpreter's answers are therefore checked against a second source
//! of the same program, not only against the hand-built tree.
//!
//! The program accumulates its verdict in its exit code (0 when every check
//! passed, a distinct number per failed check) and writes its evidence to
//! `bootstrap-eval-report.txt`, which the test reads back. The program
//! writes a relative path, so the test runs it with its project directory
//! as the working directory (the way `bootstrap_test_runner.rs` runs its
//! programs, and the way `tooling/run-bootstrap-ladder.sh` runs this one).

mod common;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const EVAL_SRC: &str = include_str!("../../bootstrap/Bootstrap.Eval.cinch");
const PARSE_SRC: &str = include_str!("../../bootstrap/Bootstrap.Parse.cinch");
const RUN_EVAL: &str = include_str!("../../bootstrap/programs/Run.Eval.cinch");

/// The report file the program writes into its project.
const REPORT_NAME: &str = "bootstrap-eval-report.txt";

/// The evidence the program's report must carry, exactly: one line per
/// evaluated expression and the verdict.
const EXPECTED_REPORT: &str = "fib 10 = 55\n\
                               main = 0\n\
                               lambda = 42\n\
                               constructor-case = 7\n\
                               nullary-constructor-case = 0\n\
                               mutual-recursion = 1\n\
                               string = cinch\n\
                               char = z\n\
                               bool-compare = 1\n\
                               wildcard = 42\n\
                               parsed-fib 10 = 55\n\
                               parsed-main = 0\n\
                               unmodeled-declaration = <refused>\n\
                               verdict = 0\n";

/// Assemble a project from the contract and the rung, build it, run it with
/// its project directory as the working directory, and return its exit code
/// and the report file's text.
fn run_program(tag: &str) -> (i32, String) {
    let dir = common::project(
        tag,
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Eval.cinch", EVAL_SRC),
            ("Bootstrap.Parse.cinch", PARSE_SRC),
            ("Main.cinch", RUN_EVAL),
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

/// The acceptance: the interpreter's `fib 10` is 55 and its `main` is 0, the
/// named refusals are refused by name, and the fragment's remaining shapes
/// evaluate. The exit code is the verdict (0) and the report is the
/// evidence.
#[test]
fn the_oracle_evaluates_fib_and_refuses_by_name() {
    let (code, report) = run_program("eval");

    assert_eq!(
        code, 0,
        "the oracle's verdict must be 0; a nonzero code names the first failing check:\n{report}"
    );

    assert!(
        report.contains("fib 10 = 55"),
        "the interpreter must evaluate `fib 10` to 55:\n{report}"
    );
    assert!(
        report.contains("main = 0"),
        "the interpreter must evaluate `main` to 0:\n{report}"
    );
    assert_eq!(
        report, EXPECTED_REPORT,
        "the evidence must be exactly the evaluated expressions and the verdict"
    );
}
