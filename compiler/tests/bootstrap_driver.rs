//! Rung 5 of the bootstrap ladder: `Bootstrap.Driver` drives clang.
//!
//! The rung's program (`bootstrap/programs/Run.Driver.cinch`, shipped as
//! `Main.cinch`) is compiled as a real project through the harness, then run
//! with real command-line arguments: the words name a
//! hand-written LLVM IR module, the executable clang must produce, and a
//! report file. The program reads the IR module through `Effect.Fs`, hands it
//! to a spawned `clang` on the child's stdin (`Effect.Process`), runs the
//! executable clang produced, and writes both exit codes into the report.
//!
//! The assertions are the rung's acceptance: (a) the program's own exit code
//! is 0 (clang linked the module), (b) the executable clang produced exists in
//! the project, and (c) running that executable yields exactly the value the
//! IR's `main` returns.
//!
//! The harness builds the program and invokes it directly so its argv,
//! rather than the seed compiler's argv, is the surface under test.

mod common;

use std::path::Path;
use std::process::{Command, Stdio};

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const DRIVER_SRC: &str = include_str!("../../bootstrap/Bootstrap.Driver.cinch");
const RUN_SRC: &str = include_str!("../../bootstrap/programs/Run.Driver.cinch");

/// The fixed, hand-written LLVM IR module the rung links: a bare `main` that
/// returns 42, so the linked executable's exit code is the number the IR
/// states and nothing else.
const IR: &str = r#"define i32 @main() {
entry:
  ret i32 42
}
"#;

/// The value the IR's `main` returns, and so the linked executable's exit code.
const IR_EXIT: i32 = 42;

/// How a program's exit code is reported when it is killed rather than exiting.
const KILLED: i32 = -1;

/// Real command-line arguments, with no stdin channel to fall back to.
fn run_with_args(exe: &Path, args: &[&str]) -> i32 {
    Command::new(exe)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()))
        .status
        .code()
        .unwrap_or(KILLED)
}

/// The rung's project: the shared contract, the rung artifact, and the
/// runnable program as the entry.
fn driver_project(tag: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Driver.cinch", DRIVER_SRC),
            ("Main.cinch", RUN_SRC),
        ],
    )
}

#[test]
fn driver_links_a_fixed_ir_module_and_reports_both_exit_codes() {
    let dir = driver_project("driver");

    let ir_path = dir.join("input with spaces.ll");
    std::fs::write(&ir_path, IR).expect("write the IR module");
    let exe_path = dir.join("prog with spaces");
    let report_path = dir.join("report.txt");

    let program = common::build(&dir, "Main.cinch", "driver-program")
        .unwrap_or_else(|e| panic!("build the driver program: {e}"));

    let ir_word = ir_path.to_string_lossy();
    let exe_word = exe_path.to_string_lossy();
    let report_word = report_path.to_string_lossy();
    let code = run_with_args(
        &program,
        &[
            "--input",
            &ir_word,
            "--output",
            &exe_word,
            "--report",
            &report_word,
        ],
    );

    // (a) the program's own exit code reports that clang linked the module.
    assert_eq!(
        code, 0,
        "driver program exit (0 = linked; each nonzero code names the step that refused)"
    );

    // (b) the executable clang produced exists in the project.
    let meta =
        std::fs::metadata(&exe_path).unwrap_or_else(|e| panic!("{}: {e}", exe_path.display()));
    assert!(meta.is_file(), "{} is not a file", exe_path.display());

    // The report names the child's exit code and the artifact's.
    let report = common::read_output(&dir, "report.txt");
    assert_eq!(
        report,
        format!("clang-exit: 0\nartifact-exit: {IR_EXIT}\n"),
        "the report file names both exit codes"
    );

    // (c) running the executable yields exactly the value the IR returns.
    let artifact = Command::new(&exe_path)
        .status()
        .unwrap_or_else(|e| panic!("run {}: {e}", exe_path.display()));
    assert_eq!(
        artifact.code(),
        Some(IR_EXIT),
        "the linked executable exits with the value the IR's main returns"
    );

    common::cleanup(&dir);
}

#[test]
fn driver_refuses_incomplete_arguments_with_its_named_exit_code() {
    let dir = driver_project("driver-args");
    let program = common::build(&dir, "Main.cinch", "driver-program")
        .unwrap_or_else(|e| panic!("build the driver program: {e}"));

    // No `--output`: the argument reader refuses with exit 1 (`exit-bad-args`).
    let code = run_with_args(&program, &["--input", "input.ll"]);
    assert_eq!(code, 1, "an incomplete argument list is exit 1");

    common::cleanup(&dir);
}
