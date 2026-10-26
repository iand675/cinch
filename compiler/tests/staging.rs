//! Staging and metaprogramming (P9): typed quotations and splices,
//! `quoteDecls`, hygienic macros, `generate do` with compile-time
//! capabilities, stage scheduling, and multi-module source files
//! (modules and compilation specification sections 7-10).

use safelang::driver::{driver_with, run_source_with, Mode, StageMode, StagingCx};
use std::path::PathBuf;

fn run(src: &str) -> Result<i32, String> {
    run_source_with(src, &mut StagingCx::default()).map_err(|e| e.to_string())
}

fn run_err(src: &str) -> String {
    run(src).expect_err("expected a compile-time failure")
}

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "safelang_staging_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write file");
    }
    dir
}

// ---------------------------------------------------------------- quotations

/// A quotation with a typed splice evaluating correctly at runtime: the
/// `stage` definition composes `Code` values at compile time, and the
/// emitted declaration runs in the program.
#[test]
fn quote_with_typed_splice_evaluates_at_runtime() {
    let code = run(
        "module Main where\n\
         \x20 stage addOne :: Code Int -> Code Int\n\
         \x20 stage addOne c = [| let x = $(c) in x + 1 |]\n\
         \x20 generate do\n\
         \x20   emit (quoteDecls where\n\
         \x20     answer :: Int\n\
         \x20     answer = $(addOne [| 21 |]))\n\
         \x20 main :: Int\n\
         \x20 main = answer\n",
    )
    .expect("quote and splice compile and run");
    assert_eq!(code, 22);
}

/// `quoteDecls` with a splice produces a checked declaration: the
/// generated `base` is spliced code, and `answer` references the generated
/// `base` by its chosen name.
#[test]
fn quote_decls_splice_produces_checked_declaration() {
    let code = run(
        "module Main where\n\
         \x20 stage mkBase :: Code Int -> Code Int\n\
         \x20 stage mkBase c = [| $(c) + 1 |]\n\
         \x20 generate do\n\
         \x20   emit (quoteDecls where\n\
         \x20     base :: Int\n\
         \x20     base = $(mkBase [| 20 |])\n\
         \x20     answer :: Int\n\
         \x20     answer = base + 22)\n\
         \x20 main :: Int\n\
         \x20 main = answer\n",
    )
    .expect("quoteDecls produces checked declarations");
    assert_eq!(code, 43);
}

// -------------------------------------------------------------------- hygiene

/// A macro's generated binder does not capture the caller's name of the
/// same spelling: the template binds `x`, the caller's spliced `x` keeps
/// its own binding (41), so the result is 1 + 41.
#[test]
fn macro_binder_does_not_capture_caller_name() {
    let code = run(
        "module Main where\n\
         \x20 x :: Int\n\
         \x20 x = 41\n\
         \x20 macro addVia :: Syntax -> Declarations\n\
         \x20 macro addVia arg = quoteDecls where\n\
         \x20   result :: Int\n\
         \x20   result = let x = 1 in x + $(arg)\n\
         \x20 generate do\n\
         \x20   emit (addVia!(x))\n\
         \x20 main :: Int\n\
         \x20 main = result\n",
    )
    .expect("hygienic macro compiles and runs");
    assert_eq!(code, 42);
}

/// The other direction: a macro helper resolves where the macro was
/// defined, and the caller's same-spelled binding keeps its own meaning.
/// The macro lives in `Macros` (helper = 1); the caller defines its own
/// `helper` (100) and splices it as the argument: 100 + 1. (Exit codes
/// truncate to eight bits, so the values stay small.)
#[test]
fn macro_helper_resolves_at_definition_site() {
    let code = run(
        "module Macros where\n\
         \x20 helper :: Int\n\
         \x20 helper = 1\n\
         \x20 macro useHelper :: Syntax -> Declarations\n\
         \x20 macro useHelper arg = quoteDecls where\n\
         \x20   result :: Int\n\
         \x20   result = $(arg) + helper\n\
         \n\
         module Main where\n\
         \x20 import stage Macros\n\
         \x20 helper :: Int\n\
         \x20 helper = 100\n\
         \x20 generate do\n\
         \x20   emit (useHelper!(helper))\n\
         \x20 main :: Int\n\
         \x20 main = result\n",
    )
    .expect("definition-site helper resolution compiles and runs");
    assert_eq!(code, 101);
}

// -------------------------------------------------------------- provenance

/// Expansion provenance survives into error spans: a generated body that
/// fails its ordinary check reports the template location and the macro
/// call site.
#[test]
fn expansion_provenance_in_error_span() {
    let err = run_err(
        "module Macros where\n\
         \x20 wrong :: Bool\n\
         \x20 wrong = True\n\
         \x20 macro bad :: Syntax -> Declarations\n\
         \x20 macro bad arg = quoteDecls where\n\
         \x20   broken :: Int\n\
         \x20   broken = wrong\n\
         \n\
         module Main where\n\
         \x20 import stage Macros\n\
         \x20 generate do\n\
         \x20   emit (bad!())\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert!(err.contains("in expansion of"), "{err}");
    assert!(err.contains("bad"), "{err}");
    assert!(err.contains("Bool"), "{err}");
}

// ------------------------------------------------- generate do capabilities

/// `generate do` with the file capability reads an input and writes a
/// value the program consumes: the captured input is lifted into the
/// generated declaration and returned as the exit code.
#[test]
fn generate_file_capability_feeds_the_program() {
    let dir = write_project(&[
        ("input.txt", "42\n"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 generate do\n\
             \x20   n <- snapshot \"inputs\" (\\io ->\n\
             \x20     withReadFiles io (\\fs -> strNat (readFile fs \"input.txt\")))\n\
             \x20   emit (quoteDecls where\n\
             \x20     answer :: Nat\n\
             \x20     answer = $(quoteNat n))\n\
             \x20 main :: Int\n\
             \x20 main = int-from-i64 (nat-to-i64 answer)\n",
        ),
    ]);
    let code = driver_with(Mode::Run, StageMode::Refresh, &dir.join("Main.lang"), None)
        .expect("file capability run");
    assert_eq!(code, 42);
    // The build identity records the captured input (modules
    // specification section 10).
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");
    assert!(manifest.contains("\"staging\""), "{manifest}");
    assert!(manifest.contains("input.txt"), "{manifest}");
    assert!(manifest.contains("refresh"), "{manifest}");
}

/// Replay uses the captured input: after a refresh build, the file is
/// removed and the replay build still produces the recorded value; a
/// replay with no captured input is rejected.
#[test]
fn stage_replay_uses_captured_inputs() {
    let dir = write_project(&[
        ("input.txt", "42\n"),
        (
            "Main.lang",
            "module Main where\n\
             \x20 generate do\n\
             \x20   n <- snapshot \"inputs\" (\\io ->\n\
             \x20     withReadFiles io (\\fs -> strNat (readFile fs \"input.txt\")))\n\
             \x20   emit (quoteDecls where\n\
             \x20     answer :: Nat\n\
             \x20     answer = $(quoteNat n))\n\
             \x20 main :: Int\n\
             \x20 main = int-from-i64 (nat-to-i64 answer)\n",
        ),
    ]);
    driver_with(Mode::Check, StageMode::Refresh, &dir.join("Main.lang"), None).expect("refresh");
    std::fs::remove_file(dir.join("input.txt")).expect("remove input");
    let code = driver_with(Mode::Run, StageMode::Replay, &dir.join("Main.lang"), None)
        .expect("replay from the snapshot");
    assert_eq!(code, 42);
    // Without any snapshot, replay names the missing input.
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 generate do\n\
         \x20   n <- snapshot \"inputs\" (\\io ->\n\
         \x20     withReadFiles io (\\fs -> strNat (readFile fs \"input.txt\")))\n\
         \x20   emit (quoteDecls where\n\
         \x20     answer :: Nat\n\
         \x20     answer = $(quoteNat n))\n\
         \x20 main :: Int\n\
         \x20 main = int-from-i64 (nat-to-i64 answer)\n",
    )]);
    let err = driver_with(Mode::Check, StageMode::Replay, &dir.join("Main.lang"), None)
        .expect_err("no captured input");
    let err = err.to_string();
    assert!(err.contains("no captured input"), "{err}");
    assert!(err.contains("input.txt"), "{err}");
}

/// The subprocess capability runs a tool and the program consumes the
/// captured output's length.
#[test]
fn generate_subprocess_capability_runs_a_tool() {
    let dir = write_project(&[(
        "Main.lang",
        "module Main where\n\
         \x20 generate do\n\
         \x20   out <- snapshot \"tools\" (\\io ->\n\
         \x20     withProcess io (\\p -> run p \"/bin/echo\" [\"hi\"]))\n\
         \x20   emit (quoteDecls where\n\
         \x20     answer :: Nat\n\
         \x20     answer = $(quoteNat (strLen out)))\n\
         \x20 main :: Int\n\
         \x20 main = int-from-i64 (nat-to-i64 answer)\n",
    )]);
    let code = driver_with(Mode::Run, StageMode::Refresh, &dir.join("Main.lang"), None)
        .expect("subprocess capability run");
    // `/bin/echo hi` writes `hi\n`.
    assert_eq!(code, 3);
}

/// A secret value must not be embedded in generated source: converting it
/// to text is rejected and the message stays redacted.
#[test]
fn secret_cannot_be_embedded() {
    std::env::set_var("SAFELANG_TEST_SECRET_42", "999999");
    let err = run_err(
        "module Main where\n\
         \x20 generate do\n\
         \x20   s <- secret \"SAFELANG_TEST_SECRET_42\"\n\
         \x20   emit (quoteDecls where\n\
         \x20     answer :: Nat\n\
         \x20     answer = $(quoteNat (strNat s)))\n\
         \x20 main :: Int\n\
         \x20 main = int-from-i64 (nat-to-i64 answer)\n",
    );
    assert!(
        err.contains("Secret") || err.contains("secret"),
        "{err}"
    );
    assert!(!err.contains("999999"), "the secret leaked into the message: {err}");
}

// ------------------------------------------------------------- scheduling

/// A cyclic stage dependency is rejected with the cycle shown: compiling
/// `f` needs the generated `n`, and executing the generator needs `f`.
#[test]
fn stage_dependency_cycle_shows_the_cycle() {
    let err = run_err(
        "module Main where\n\
         \x20 stage f :: Nat\n\
         \x20 stage f = n\n\
         \x20 generate do\n\
         \x20   let x = f\n\
         \x20   emit (quoteDecls where\n\
         \x20     n :: Nat\n\
         \x20     n = 1)\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    );
    assert!(err.contains("stage dependency cycle"), "{err}");
    assert!(err.contains("->"), "{err}");
    assert!(err.contains("`f`"), "{err}");
    assert!(err.contains("generate block"), "{err}");
}

/// Generated declarations are checked under ordinary rules: a generated
/// body of the wrong type is rejected, not silently admitted.
#[test]
fn generated_declaration_fails_ordinary_checks() {
    let err = run_err(
        "module Main where\n\
         \x20 generate do\n\
         \x20   emit (quoteDecls where\n\
         \x20     broken :: Int\n\
         \x20     broken = True)\n\
         \x20 main :: Int\n\
         \x20 main = broken\n",
    );
    assert!(err.contains("Bool"), "{err}");
}

// -------------------------------------------------------- multi-module files

/// A multi-module source file compiles and runs with cross-module
/// references.
#[test]
fn multi_module_source_file_compiles_and_runs() {
    let code = run(
        "module Lib where\n\
         \x20 double :: Int -> Int\n\
         \x20 double x = x + x\n\
         \n\
         module Main where\n\
         \x20 import Lib\n\
         \x20 main :: Int\n\
         \x20 main = if Lib.double 21 == 42 then 0 else 1\n",
    )
    .expect("multi-module source compiles and runs");
    assert_eq!(code, 0);
}

/// A macro in one module of a multi-module file expands in another, and
/// generated declarations land in the generator's own module.
#[test]
fn multi_module_staging_crosses_modules() {
    let code = run(
        "module Gen where\n\
         \x20 stage answerCode :: Code Int\n\
         \x20 stage answerCode = [| 42 |]\n\
         \n\
         module Main where\n\
         \x20 import stage Gen\n\
         \x20 generate do\n\
         \x20   emit (quoteDecls where\n\
         \x20     answer :: Int\n\
         \x20     answer = $(answerCode))\n\
         \x20 main :: Int\n\
         \x20 main = answer\n",
    )
    .expect("cross-module staging compiles and runs");
    assert_eq!(code, 42);
}

// -------------------------------------- schema introspection (A6; section 11)

/// Write a schema introspection tool script (the subprocess the
/// `Stage.withDatabase` delegation runs) and return its path.
fn schema_tool(name: &str, doc: &str) -> String {
    let dir =
        std::env::temp_dir().join(format!("safelang_schema_{}_{}", std::process::id(), name));
    std::fs::create_dir_all(&dir).expect("schema tool dir");
    let path = dir.join("introspect.sh");
    std::fs::write(&path, format!("#!/bin/sh\ncat <<'JSON'\n{doc}\nJSON\n")).expect("write tool");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }
    path.to_string_lossy().into_owned()
}

/// The generator and program the schema tests share: `CodecGenerator.derive`
/// turns the captured snapshot into the row record and its codec pair,
/// and `main` round-trips a row through them.
const A6_PROGRAM: &str = r#"module Main where
  generate do
    snapshot <- Stage.snapshot "billing-schema" (\io ->
      Stage.withDatabase io (secret "SL_TEST_A6_DSN") ReadSchema
        (\db -> introspectConsistently db))
    emit (CodecGenerator.derive snapshot)
  main :: Int
  main =
    let row = BillingEvents { id = 42, active = True } in
    case decodeBillingEvents (encodeBillingEvents row) of
      Some r -> if r.id == 42 then if r.active then 0 else 1 else 2
      None -> 3
"#;

/// The introspection tool, the consistency check and the codec generator
/// run end to end: the derived row record and codec pair compile as
/// ordinary declarations and round-trip a row at runtime.
#[test]
fn generate_with_database_introspection_derives_codecs() {
    std::env::set_var("SL_TEST_A6_DSN", "postgres://test");
    let tool = schema_tool(
        "derive",
        r#"{"dialect": "postgresql", "collation": "en_US.UTF-8", "snapshot": "0/16B3748", "tables": [{"name": "billing_events", "columns": [{"name": "id", "type": "INTEGER", "nullable": false}, {"name": "active", "type": "BOOLEAN", "nullable": false}]}]}"#,
    );
    let mut scx = StagingCx::default();
    scx.schema_tool = Some(tool);
    let code = run_source_with(A6_PROGRAM, &mut scx).expect("introspection derives codecs");
    assert_eq!(code, 0);
}

/// `introspectConsistently` is the sketch's consistency check: a document
/// whose table names collide is rejected (modules specification
/// section 11).
#[test]
fn introspect_consistently_rejects_an_inconsistent_document() {
    std::env::set_var("SL_TEST_A6_DSN", "postgres://test");
    let tool = schema_tool(
        "dup",
        r#"{"tables": [{"name": "t", "columns": []}, {"name": "t", "columns": []}]}"#,
    );
    let mut scx = StagingCx::default();
    scx.schema_tool = Some(tool);
    let err = run_source_with(A6_PROGRAM, &mut scx).expect_err("duplicate table rejected");
    let err = err.to_string();
    assert!(err.contains("inconsistent schema"), "{err}");
    assert!(err.contains("`t` appears twice"), "{err}");
}

/// The interpreted schema vocabulary is named in the error: a column type
/// outside INTEGER, REAL, BOOLEAN and TEXT is rejected, and the codec
/// generator's coverage gap is explicit.
#[test]
fn schema_column_types_and_codec_coverage_are_explicit() {
    std::env::set_var("SL_TEST_A6_DSN", "postgres://test");
    let tool = schema_tool(
        "bogus",
        r#"{"tables": [{"name": "t", "columns": [{"name": "c", "type": "JSONB"}]}]}"#,
    );
    let mut scx = StagingCx::default();
    scx.schema_tool = Some(tool.clone());
    let err = run_source_with(A6_PROGRAM, &mut scx).expect_err("unknown column type");
    let err = err.to_string();
    assert!(err.contains("unsupported column type `JSONB`"), "{err}");
    // TEXT is interpreted but not yet encoded (decision D65).
    let tool = schema_tool(
        "text",
        r#"{"tables": [{"name": "billing_events", "columns": [{"name": "id", "type": "INTEGER"}, {"name": "active", "type": "BOOLEAN"}, {"name": "note", "type": "TEXT"}]}]}"#,
    );
    let mut scx = StagingCx::default();
    scx.schema_tool = Some(tool);
    let err = run_source_with(A6_PROGRAM, &mut scx).expect_err("TEXT not encoded");
    let err = err.to_string();
    assert!(err.contains("the codec generator's column encoders cover INTEGER and BOOLEAN"), "{err}");
}

/// Without a configured introspection tool, `Stage.withDatabase` names
/// the delegation rather than inventing a network stack (decision D65).
#[test]
fn with_database_needs_a_configured_introspection_tool() {
    std::env::set_var("SL_TEST_A6_DSN", "postgres://test");
    let mut scx = StagingCx::default();
    let err = run_source_with(A6_PROGRAM, &mut scx).expect_err("no tool configured");
    let err = err.to_string();
    assert!(err.contains("no schema introspection tool is configured"), "{err}");
}

/// Replay uses the captured schema: the tool on disk now reports
/// `active` as INTEGER, but the captured document says BOOLEAN, and the
/// program (which reads `active` as a `Bool`) compiles and runs from the
/// capture.
#[test]
fn stage_replay_uses_the_captured_schema() {
    use safelang::check::StageInput;
    std::env::set_var("SL_TEST_A6_DSN", "postgres://test");
    let tool = schema_tool(
        "replay",
        r#"{"tables": [{"name": "billing_events", "columns": [{"name": "id", "type": "INTEGER"}, {"name": "active", "type": "INTEGER"}]}]}"#,
    );
    let captured = r#"{"snapshot": "0/16B3748", "tables": [{"name": "billing_events", "columns": [{"name": "id", "type": "INTEGER", "nullable": false}, {"name": "active", "type": "BOOLEAN", "nullable": false}]}]}"#;
    let key = format!("{tool}  env:SL_TEST_A6_DSN");
    let mut scx = StagingCx::default();
    scx.mode = StageMode::Replay;
    scx.schema_tool = Some(tool);
    scx.prior_inputs = vec![StageInput {
        snapshot: "billing-schema".to_string(),
        kind: "subprocess".to_string(),
        name: key,
        hash: safelang::interface::content_hash(captured),
        captured: captured.to_string(),
        live: false,
    }];
    let code = run_source_with(A6_PROGRAM, &mut scx).expect("replay from the captured schema");
    assert_eq!(code, 0);
}

/// The build identity records the introspection tool and the credential's
/// environment name, never the credential (modules specification
/// section 10).
#[test]
fn schema_build_identity_records_the_tool_not_the_credential() {
    std::env::set_var("SL_TEST_A6_DSN", "postgres://supersecret");
    let tool = schema_tool(
        "identity",
        r#"{"tables": [{"name": "billing_events", "columns": [{"name": "id", "type": "INTEGER"}, {"name": "active", "type": "BOOLEAN"}]}]}"#,
    );
    std::env::set_var("SAFELANG_SCHEMA_TOOL", &tool);
    let dir = write_project(&[("Main.lang", A6_PROGRAM)]);
    let code = driver_with(Mode::Run, StageMode::Refresh, &dir.join("Main.lang"), None)
        .expect("schema introspection run");
    assert_eq!(code, 0);
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");
    assert!(manifest.contains("env:SL_TEST_A6_DSN"), "{manifest}");
    assert!(!manifest.contains("supersecret"), "the credential leaked: {manifest}");
}
