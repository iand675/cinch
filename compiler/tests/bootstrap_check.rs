//! Rung 3: the fragment checker (`bootstrap/Bootstrap.Check.cinch`).
//!
//! The rung's artifact is a real program, `bootstrap/programs/Run.Check.cinch`,
//! shipped here as the entry module of a project holding the toolkit, the two
//! shared contract modules and the checker. It hand-builds the AST of
//! `examples/compiler/fib.cinch` and then, for each case, compares the checker's
//! rendered diagnostics against the text pinned in the program itself, so the
//! program's exit code is the 1-based number of the first case that does not
//! match (0 when the whole suite agrees), and it writes the same rendering to
//! `bootstrap-check-report.txt` in its working directory.
//!
//! This test asserts both artefacts: the exit code, and the report file byte for
//! byte, so the exact diagnostic text of every refusal is pinned here rather
//! than asserted in a message a reader has to reconstruct. The cases are the
//! accepted `fib` program, an unbound variable, a type mismatch, an arity
//! mismatch against the signature, a duplicated signature, a function with no
//! signature, an unmodeled expression, declaration, type and pattern, class, instance and
//! fixity declarations, an operator and an expression and a type and a pattern
//! form outside the fragment, an unknown constructor, a constructor at the wrong
//! arity, a constructor whose result type is not its declaration's, an
//! unreachable clause, an accepted recursive corpus (a data type, case
//! matching, mutual recursion) and the accepted literal surface.

mod common;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const CHECK_SRC: &str = include_str!("../../bootstrap/Bootstrap.Check.cinch");
const RUN: &str = include_str!("../../bootstrap/programs/Run.Check.cinch");

/// The report file the rung's program writes, pinned byte for byte. Each case
/// contributes its label, a colon and a newline, then the checker's own
/// rendering of that program (empty for an accepted one).
const EXPECTED_REPORT: &str = r#"fib:
unbound:
bootstrap.check:5:14: unbound variable: m
type:
bootstrap.check:5:11: type mismatch: expected Int, found Bool
arity:
bootstrap.check:5:7: arity mismatch: fib: signature arity 1, clause arity 2
dup-sig:
bootstrap.check:4:3: duplicate signature for fib
no-sig:
bootstrap.check:5:3: no signature for fib
bootstrap.check:8:13: unbound variable: fib
unmodeled-expr:
bootstrap.check:5:11: unmodeled expression: let x = 1 in x
unmodeled-decl:
bootstrap.check:4:3: unmodeled declaration: type Alias = Int
unmodeled-type:
bootstrap.check:4:8: unmodeled type: (a, b) -> Int
unmodeled-pattern:
bootstrap.check:10:5: unmodeled pattern: Pz | Ps q
class:
bootstrap.check:4:3: class declarations are not supported in this rung
instance:
bootstrap.check:4:3: instance declarations are not supported in this rung
fixity:
bootstrap.check:4:3: fixity declarations are not supported in this rung
operator:
bootstrap.check:5:11: unsupported operator: &&
expression:
bootstrap.check:5:11: unsupported expression: tuple
type-form:
bootstrap.check:4:8: unsupported type: forall
pattern:
bootstrap.check:10:5: unsupported pattern: list literal
unknown-ctor:
bootstrap.check:10:5: unknown constructor: Qz
ctor-arity:
bootstrap.check:11:5: arity mismatch: constructor Ps: declared arity 1, used arity 2
ctor-return:
bootstrap.check:4:10: constructor Pz must return Peano
unreachable-clause:
bootstrap.check:6:7: unreachable clause: an earlier clause already matches these patterns
fragment:
literals:
"#;

/// The report path the program writes and this test reads (both run with the
/// same working directory, so the name is all either needs to know).
const REPORT_FILE: &str = "bootstrap-check-report.txt";

#[test]
fn the_fragment_checker_accepts_fib_and_names_every_refusal() {
    let path = std::path::Path::new(REPORT_FILE);
    let _ = std::fs::remove_file(path);
    let dir = common::project(
        "check",
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("Bootstrap.Check.cinch", CHECK_SRC),
            ("Main.cinch", RUN),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    let report = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!("the rung's program wrote no {REPORT_FILE}: {e} (exit code {code})")
    });
    let _ = std::fs::remove_file(path);
    assert_eq!(
        code, 0,
        "rung 3 driver exited {code}: that is the 1-based number of the first case whose \
         diagnostics differ from the text pinned for it in bootstrap/programs/Run.Check.cinch"
    );
    assert_eq!(report, EXPECTED_REPORT, "the rung's report");
}
