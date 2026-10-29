//! Rung 4 of the bootstrap ladder: LLVM IR text for the fragment.
//!
//! The Cinch driver program (`bootstrap/programs/Run.Ir.cinch`) builds
//! three fragments' trees, emits each one's LLVM IR text with
//! `Bootstrap.Ir` and writes the text into its project. This test ships
//! that program as a real project beside the shared contract, runs it
//! through the driver, and then treats the emitted text as IR: it
//! asserts the text's shape (a `define` per function, an `i32` `ret`),
//! assembles each module with `clang` and runs the produced binary,
//! asserting the exit code equals the fragment's `main` meaning.
//!
//! The three fragments, and the number their `main` must produce:
//!   * `fib`: `main = if fib 10 == 55 then 0 else 1`, so 0;
//!   * `arith`: `main = let x = 6 in let y = 7 in x * y`, so 42;
//!   * `pick`: `pick a b = if a < b then b else a`, `main = pick 7 3`, so 7.
//!
//! The last two exist so the assertion is not a coincidence of zero.
//! A fourth fragment (an unmodeled `main` body) is not emitted: the
//! program writes the emitter's refusal for it and this test asserts the
//! refusal is a `bootstrap.ir:` diagnostic that names the unmodeled node,
//! so the honest boundary is observed rather than assumed.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const IR_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ir.cinch");
const RUN_SRC: &str = include_str!("../../bootstrap/programs/Run.Ir.cinch");

/// A Cinch string literal for a path: the driver program's output paths
/// are rewritten to absolute locations inside the project (the compiled
/// program runs with the test process's working directory, so a relative
/// path would land outside the project).
fn quoted(path: &Path) -> String {
    format!("{:?}", path.to_string_lossy())
}

/// Assemble one emitted IR module with clang; returns the executable's
/// path. `CINCH_CLANG` selects the clang, as the compiler's own tests do.
fn assemble(dir: &Path, ll: &str) -> PathBuf {
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let exe = dir.join(ll.trim_end_matches(".ll"));
    let out = Command::new(&clang)
        .arg(dir.join(ll))
        .arg("-o")
        .arg(&exe)
        .output()
        .expect("run clang");
    assert!(
        out.status.success(),
        "{ll}: clang rejected the emitted IR:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    exe
}

#[test]
fn the_emitted_ir_assembles_and_runs_to_the_fragments_meaning() {
    let dir = common::project(
        "ir",
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Ir.cinch", IR_SRC),
        ],
    );
    // The driver's output paths become absolute locations in the project.
    let entry = RUN_SRC
        .replace("\"program.ll\"", &quoted(&dir.join("program.ll")))
        .replace("\"program-arith.ll\"", &quoted(&dir.join("program-arith.ll")))
        .replace("\"program-pick.ll\"", &quoted(&dir.join("program-pick.ll")))
        .replace(
            "\"program-ir.diagnostics\"",
            &quoted(&dir.join("program-ir.diagnostics")),
        )
        .replace(
            "\"program-ir.refusals\"",
            &quoted(&dir.join("program-ir.refusals")),
        );
    std::fs::write(dir.join("Main.cinch"), &entry).expect("write entry project");

    let code = common::run(&dir, "Main.cinch");
    let diagnostics = common::read_output(&dir, "program-ir.diagnostics");
    assert_eq!(
        code, 0,
        "driver failure bits (1 fib write, 2 arith write, 4 pick write, 8 refusals write, \
         16 diagnostics write, 32/64/128 a lowerable fragment was refused); \
         diagnostics: {diagnostics:?}"
    );

    // The emitter draws no refusal for a fragment it can lower.
    assert_eq!(
        diagnostics, "",
        "a lowerable fragment must draw no refusal"
    );

    // The honest boundary: the unmodeled fragment is refused by name.
    let refusals = common::read_output(&dir, "program-ir.refusals");
    assert!(
        refusals.contains("bootstrap.ir:"),
        "the refusal must be a rung diagnostic: {refusals:?}"
    );
    assert!(
        refusals.contains("unmodeled"),
        "the refusal must name the unmodeled node it cannot lower: {refusals:?}"
    );

    // The emitted text has the shape claimed: one `define` per function,
    // an i64 helper and an i32 `main`, a direct recursive call, a branch
    // and a slot.
    let fib_ir = common::read_output(&dir, "program.ll");
    assert_eq!(
        fib_ir.matches("define ").count(),
        2,
        "one `define` per function (fib and main):\n{fib_ir}"
    );
    assert!(
        fib_ir.contains("define i64 @cinch_fib(i64 %"),
        "the fragment's function is a `define` with an i64 parameter:\n{fib_ir}"
    );
    assert!(
        fib_ir.contains("define i32 @main() {"),
        "`main` is a `define` returning i32 (the fragment emitter's standalone process entry):\n{fib_ir}"
    );
    assert!(
        fib_ir.contains("ret i32 "),
        "`main` returns an i32:\n{fib_ir}"
    );
    assert!(
        fib_ir.contains("call i64 @cinch_fib(i64 "),
        "recursion is a direct call:\n{fib_ir}"
    );
    assert!(
        fib_ir.contains("br i1 ") && fib_ir.contains("alloca i64"),
        "`if` is a branch over an alloca slot:\n{fib_ir}"
    );

    // Each module is real IR: clang assembles it and the binary's exit
    // code is the fragment's `main` meaning.
    for (module, expected) in [
        ("program.ll", 0),
        ("program-arith.ll", 42),
        ("program-pick.ll", 7),
    ] {
        let exe = assemble(&dir, module);
        let status = Command::new(&exe)
            .status()
            .unwrap_or_else(|e| panic!("run {}: {e}", exe.display()));
        assert_eq!(
            status.code(),
            Some(expected),
            "{module}: the fragment's `main` must exit {expected}"
        );
    }

    common::cleanup(&dir);
}

#[test]
fn a_two_parameter_function_defines_one_symbol_and_its_call_carries_both_arguments() {
    // The `pick` fragment: a `define` and an `i32` entry, the call
    // carrying both `i64` arguments.
    let dir = common::project(
        "ir_pick",
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Ir.cinch", IR_SRC),
        ],
    );
    let entry = RUN_SRC
        .replace("\"program.ll\"", &quoted(&dir.join("program.ll")))
        .replace("\"program-arith.ll\"", &quoted(&dir.join("program-arith.ll")))
        .replace("\"program-pick.ll\"", &quoted(&dir.join("program-pick.ll")))
        .replace(
            "\"program-ir.diagnostics\"",
            &quoted(&dir.join("program-ir.diagnostics")),
        )
        .replace(
            "\"program-ir.refusals\"",
            &quoted(&dir.join("program-ir.refusals")),
        );
    std::fs::write(dir.join("Main.cinch"), &entry).expect("write entry project");

    let code = common::run(&dir, "Main.cinch");
    assert_eq!(code, 0, "driver failed on the pick fragment");

    let pick_ir = common::read_output(&dir, "program-pick.ll");
    assert_eq!(
        pick_ir.matches("define ").count(),
        2,
        "one `define` per function (pick and main):\n{pick_ir}"
    );
    assert!(
        pick_ir.contains("define i64 @cinch_pick(i64 %"),
        "the two-parameter function takes i64 parameters:\n{pick_ir}"
    );
    assert!(
        pick_ir.contains("call i64 @cinch_pick(i64 7, i64 3)"),
        "the call carries both i64 arguments:\n{pick_ir}"
    );

    common::cleanup(&dir);
}
