//! Rung 6, end to end: the ladder compiles an example with itself.
//!
//! `examples/compiler/fib.cinch` goes source -> parsed -> checked -> LLVM IR
//! text -> native binary through a pipeline written in Cinch
//! (`Bootstrap.Pipeline`), and the binary is run. Nothing in this test
//! compiles the example with the Rust compiler: the pipeline program does all
//! of it and clang is the only foreign tool involved.
//!
//! Two things about the harness are deliberate. The program's own project is
//! built FIRST and the example source is written into the directory after
//! that, because every `.cinch` beside the entry belongs to the project and
//! `fib.cinch` declares `module Main` too. And the pipeline reads its four
//! argument words from the runtime's input source, so the test drives the
//! built executable itself with a piped stdin, the same shape
//! `bootstrap_driver.rs` and `bootstrap_test_runner.rs` use.

mod common;

use std::io::Write;
use std::path::{Path, PathBuf};

const AST: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const PARSE: &str = include_str!("../../bootstrap/Bootstrap.Parse.cinch");
const CHECK: &str = include_str!("../../bootstrap/Bootstrap.Check.cinch");
const IR: &str = include_str!("../../bootstrap/Bootstrap.Ir.cinch");
const DRIVER: &str = include_str!("../../bootstrap/Bootstrap.Driver.cinch");
const PIPELINE: &str = include_str!("../../bootstrap/Bootstrap.Pipeline.cinch");
const RUN: &str = include_str!("../../bootstrap/programs/Run.Pipeline.cinch");
const FIB: &str = include_str!("../../examples/compiler/fib.cinch");

/// The ladder as one project's sources.
fn ladder() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Bootstrap.Ast.cinch", AST),
        ("Bootstrap.Report.cinch", REPORT),
        ("Bootstrap.Parse.cinch", PARSE),
        ("Bootstrap.Check.cinch", CHECK),
        ("Bootstrap.Ir.cinch", IR),
        ("Bootstrap.Driver.cinch", DRIVER),
        ("Bootstrap.Pipeline.cinch", PIPELINE),
        ("Main.cinch", RUN),
    ]
}

/// Run a built program in its project with `words` fed to its input source,
/// one word per line, and return its exit code.
fn run_built(exe: &Path, dir: &Path, words: &[String]) -> (Option<i32>, String) {
    let input = words.iter().map(|w| format!("{w}\n")).collect::<String>();
    let mut child = std::process::Command::new(exe)
        .current_dir(dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn the pipeline");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("feed the argument words");
    let out = child.wait_with_output().expect("wait for the pipeline");
    (out.status.code(), String::from_utf8_lossy(&out.stderr).to_string())
}

/// Build the pipeline program and hand back its project and executable.
fn pipeline_project(tag: &str) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &ladder());
    let exe = common::build(&dir, "Main.cinch", "pipeline")
        .unwrap_or_else(|e| panic!("building the pipeline program failed: {e}"));
    (dir, exe)
}

#[test]
fn the_ladder_compiles_the_fib_example_with_itself() {
    let (dir, exe) = pipeline_project("pipeline_fib");
    // Written after the build: a `.cinch` beside the entry would join the
    // program's own project, and this one declares `module Main` too.
    std::fs::write(dir.join("fib.cinch"), FIB).expect("write the example");
    let words: Vec<String> = ["fib.cinch", "fib.ll", "fib.bin", "pipeline-report.txt"]
        .iter()
        .map(|s| dir.join(s).display().to_string())
        .collect();

    let (code, stderr) = run_built(&exe, &dir, &words);
    assert_eq!(
        code,
        Some(0),
        "the pipeline did not reach the end (0 ok, 1 bad words, 2 read, 3 parse, 4 check, 5 emit, 6 write, then the driver's own codes); stderr: {stderr}"
    );

    // The IR module is real IR text, and the report says what clang did.
    let ll = common::read_output(&dir, "fib.ll");
    assert!(ll.contains("define"), "the emitted module has no definition:\n{ll}");
    assert!(ll.contains("ret i32"), "the emitted module has no i32 return:\n{ll}");
    let report = common::read_output(&dir, "pipeline-report.txt");
    assert!(report.contains("clang-exit: 0"), "clang did not link: {report}");
    assert!(
        report.contains("artifact-exit: 0"),
        "the artifact did not answer 0 (fib 10 == 55 means main is 0): {report}"
    );

    // And the binary the pipeline produced runs on its own.
    let ran = std::process::Command::new(dir.join("fib.bin"))
        .status()
        .expect("run the artifact the pipeline produced");
    assert_eq!(ran.code(), Some(0), "the produced binary answered {ran:?}");

    common::cleanup(&dir);
}

#[test]
fn a_refused_source_stops_with_its_diagnostics_written() {
    let (dir, exe) = pipeline_project("pipeline_refuse");
    // A well-formed program whose body is not in the fragment: the checker
    // or the emitter must refuse it by name rather than emit nonsense.
    std::fs::write(
        dir.join("broken.cinch"),
        "module Main where\n  main :: Int\n  main = missing-name\n",
    )
    .expect("write the refused source");
    let words: Vec<String> = ["broken.cinch", "broken.ll", "broken.bin", "broken-report.txt"]
        .iter()
        .map(|s| dir.join(s).display().to_string())
        .collect();

    let (code, stderr) = run_built(&exe, &dir, &words);
    assert_eq!(
        code,
        Some(4),
        "an unbound name must be refused by the checker (4), not carried further; stderr: {stderr}"
    );
    let refusal = common::read_output(&dir, "broken.ll.diagnostics");
    assert!(
        !refusal.trim().is_empty(),
        "the refusal wrote no diagnostics:\n{refusal}"
    );
    assert!(
        !dir.join("broken.ll").exists(),
        "a refused source must not leave an IR module behind"
    );
    assert!(
        !dir.join("broken.bin").exists(),
        "a refused source must not leave an executable behind"
    );

    common::cleanup(&dir);
}
