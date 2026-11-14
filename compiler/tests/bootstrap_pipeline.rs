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
//! `fib.cinch` declares `module Main` too. The built program is invoked
//! with real command-line arguments and no stdin, including a source path
//! containing spaces and a newline.

mod common;

use std::path::{Path, PathBuf};

const AST: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const PARSE: &str = include_str!("../../bootstrap/Bootstrap.Parse.cinch");
const RESOLVE: &str = include_str!("../../bootstrap/Bootstrap.Resolve.cinch");
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
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Report.cinch", REPORT),
        ("Bootstrap.Parse.cinch", PARSE),
        ("Bootstrap.Expand.cinch", include_str!("../../bootstrap/Bootstrap.Expand.cinch")),
        ("Bootstrap.Resolve.cinch", RESOLVE),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Check.cinch", CHECK),
        ("Bootstrap.Ir.cinch", IR),
        ("Bootstrap.Driver.cinch", DRIVER),
        ("Bootstrap.Pipeline.cinch", PIPELINE),
        ("Main.cinch", RUN),
    ]
}

/// Run the program with real OS arguments; stdin is unavailable.
fn run_built(exe: &Path, dir: &Path, words: &[String]) -> (Option<i32>, String) {
    let out = std::process::Command::new(exe)
        .current_dir(dir)
        .args(words)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("spawn the pipeline");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
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
    assert_eq!(run_built(&exe, &dir, &[]).0, Some(1), "missing arguments");
    // Written after the build: a `.cinch` beside the entry would join the
    // program's own project, and this one declares `module Main` too.
    std::fs::write(dir.join("fib with\na newline.cinch"), FIB).expect("write the example");
    let words: Vec<String> = [
        "fib with\na newline.cinch",
        "fib.ll",
        "fib.bin",
        "pipeline-report.txt",
    ]
    .iter()
    .map(|s| dir.join(s).display().to_string())
    .collect();

    let mut extra = words.clone();
    extra.push("extra".into());
    assert_eq!(run_built(&exe, &dir, &extra).0, Some(1), "extra arguments");
    assert!(
        !dir.join("fib.ll").exists(),
        "bad arguments must stop before emission"
    );

    let (code, stderr) = run_built(&exe, &dir, &words);
    assert_eq!(
        code,
        Some(0),
        "the pipeline did not reach the end (0 ok, 1 bad words, 2 read, 3 parse, 4 check, 5 emit, 6 write, then the driver's own codes); stderr: {stderr}"
    );

    // The IR module is real IR text, and the report says what clang did.
    let ll = common::read_output(&dir, "fib.ll");
    assert!(
        ll.contains("define"),
        "the emitted module has no definition:\n{ll}"
    );
    assert!(
        ll.contains("ret i32"),
        "the emitted module has no i32 return:\n{ll}"
    );
    let report = common::read_output(&dir, "pipeline-report.txt");
    assert!(
        report.contains("clang-exit: 0"),
        "clang did not link: {report}"
    );
    assert!(
        report.contains("artifact-exit: 0"),
        "the artifact did not answer 0 (fib 10 == 55 means main is 0): {report}"
    );

    // And the binary the pipeline produced runs on its own.
    let ran = std::process::Command::new(dir.join("fib.bin"))
        .status()
        .expect("run the artifact the pipeline produced");
    assert_eq!(ran.code(), Some(0), "the produced binary answered {ran:?}");

    // Reuse the compiled pipeline: rebuilding the Cinch frontend for each
    // input adds substantial CPU work without testing another build path.
    let source = "module A where\n  export answer, truth, (<+>)\n  answer :: Int\n  answer = 42\n  truth :: Bool\n  truth = True\n  infixr 6 <+>\n  (<+>) :: Int -> Int -> Int\n  (<+>) a b = a - b\nmodule Facade where\n  import A (answer, truth, (<+>) as (<*>))\n  export answer, truth, (<*>)\nmodule Main where\n  import Facade\n  main :: Int\n  main = if truth then answer + (10 <*> 3 <*> 2) - 51 else 1\n";
    std::fs::write(dir.join("modules.cinch"), source).expect("write scoped module source");
    let words: Vec<String> = ["modules.cinch", "modules.ll", "modules.bin", "modules-report.txt"]
        .iter().map(|s| dir.join(s).display().to_string()).collect();
    let (code, stderr) = run_built(&exe, &dir, &words);
    assert_eq!(code, Some(0), "module globals, Boolean values and inherited operator fixities: {stderr}");
    let report = common::read_output(&dir, "modules-report.txt");
    assert!(report.contains("clang-exit: 0") && report.contains("artifact-exit: 0"), "{report}");
    assert_eq!(std::process::Command::new(dir.join("modules.bin")).status().unwrap().code(), Some(0));

    frontend_modes(&dir, &exe);
    erased_reflexivity(&dir, &exe);
    staged_sources(&dir, &exe);
    interface_output(&dir, &exe);
    refused_source(&dir, &exe);
    checked_interfaces(&dir, &exe);
    common::cleanup(&dir);
}

fn erased_reflexivity(dir: &Path, exe: &Path) {
    let input = dir.join("refl.cinch");
    let ir = dir.join("refl.ll");
    let args = |mode: &str| vec![mode.into(), input.display().to_string(), ir.display().to_string()];
    let source = |goal: &str, body: &str| format!(
        "module Main where\n  consume :: {goal} -> Int\n  consume evidence = 0\n  main :: Int\n  main = {body}\n"
    );
    std::fs::write(&input, source("Eq (2 + 3) 5", "consume refl")).unwrap();
    let (code, stderr) = run_built(exe, dir, &args("--emit-ir"));
    assert_eq!(code, Some(0), "{stderr}\n{}", common::read_output(dir, "refl.ll.diagnostics"));
    assert!(!std::fs::read_to_string(&ir).unwrap().is_empty());

    let compare = "module Main where\n  consume :: (5 == 5) -> Int\n  consume evidence = 0\n  forward :: (5 == 5) -> Int\n  forward evidence = consume evidence\n  main :: Int\n  main = 0\n";
    std::fs::write(&input, compare).unwrap();
    let (code, stderr) = run_built(exe, dir, &args("--check"));
    assert_eq!(code, Some(0), "{stderr}\n{}", common::read_output(dir, "refl.ll.diagnostics"));

    for (goal, body, reason) in [
        ("Eq (2 + 3) 6", "consume refl", "refl requires definitionally equal operands"),
        ("(5 == 5)", "consume refl", "refl requires definitionally equal operands"),
        ("Eq 5 5", "let refl = 7 in consume refl", "erased argument needs"),
    ] {
        std::fs::write(&input, source(goal, body)).unwrap();
        assert_eq!(run_built(exe, dir, &args("--check")).0, Some(4));
        let diagnostic = common::read_output(dir, "refl.ll.diagnostics");
        assert!(diagnostic.contains(reason), "{diagnostic}");
    }

    let named = "module Main where\n  proof arithmetic :: Eq (2 + 3) 5\n  proof arithmetic = (refl)\n  consume :: Eq (2 + 3) 5 -> Int\n  consume evidence = 42\n  main :: Int\n  main = if consume arithmetic == 42 then 0 else 1\n";
    std::fs::write(&input, named).unwrap();
    let (code, stderr) = run_built(exe, dir, &args("--emit-ir"));
    assert_eq!(code, Some(0), "{stderr}\n{}", common::read_output(dir, "refl.ll.diagnostics"));
    let artifact = dir.join("refl.bin");
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let mut clang = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()));
    clang.arg(&ir).arg(runtime).arg("-o").arg(&artifact);
    if cfg!(target_os = "linux") { clang.args(["-ldl", "-lpthread", "-lm"]); }
    let linked = clang.output().expect("link the program with erased evidence");
    assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
    let output = std::process::Command::new(&artifact).env("CINCH_HEAP_LIMIT", "32M")
        .env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stderr).contains("live=0 immortal=0 leaked=0"));

    for (invalid, reason) in [
        (named.replacen("Eq (2 + 3) 5", "Eq (2 + 3) 6", 1), "obligation disproved"),
        (named.replace("proof arithmetic = (refl)", "proof arithmetic = arithmetic"), "requires kernel refl"),
        (named.replace("  proof arithmetic :: Eq (2 + 3) 5\n", ""), "no proof signature"),
        (named.replace("proof arithmetic = (refl)", "proof arithmetic = refl\n  proof arithmetic = refl"), "exactly one argument-free proof equation"),
    ] {
        std::fs::write(&input, invalid).unwrap();
        assert_eq!(run_built(exe, dir, &args("--check")).0, Some(4));
        let diagnostic = common::read_output(dir, "refl.ll.diagnostics");
        assert!(diagnostic.contains(reason), "{diagnostic}");
    }
}

fn staged_sources(dir: &Path, exe: &Path) {
    let source = dir.join("staged.cinch");
    let prefix = dir.join("staged.ll");
    let run = |mode: &str, text: &str| {
        std::fs::write(&source, text).unwrap();
        let args = vec![mode.into(), source.display().to_string(), prefix.display().to_string()];
        let (exit, stderr) = run_built(exe, dir, &args);
        let diagnostic = std::fs::read_to_string(dir.join("staged.ll.diagnostics")).unwrap_or_default();
        (exit, format!("{diagnostic}{stderr}"))
    };
    let generated = "module Main where\n  stage bump :: Nat -> Nat\n  stage bump n = n + 2\n  stage count :: Nat\n  stage count = bump 40\n  generate do\n    emit (quoteDecls where\n      result :: Nat\n      result = $(quoteNat count))\n  main :: Int\n  main = if result == 42 && result / 2 == 21 then 0 else 1\n";
    let (exit, diagnostic) = run("--check", generated);
    assert_eq!(exit, Some(0), "stage-checked generation: {diagnostic}");
    let (exit, diagnostic) = run("--emit-ir", generated);
    assert_eq!(exit, Some(0), "emit the checked generated declaration through typed IR: {diagnostic}");
    let runtime = Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let artifact = dir.join("staged.bin");
    let mut clang = std::process::Command::new("clang");
    clang.arg(&prefix).arg(runtime).arg("-o").arg(&artifact);
    if cfg!(target_os = "linux") { clang.args(["-ldl", "-lpthread", "-lm"]); }
    let linked = clang.output().expect("link the staged program");
    assert!(linked.status.success(), "{}", String::from_utf8_lossy(&linked.stderr));
    let ran = std::process::Command::new(&artifact).env("CINCH_LEAK_CHECK", "1")
        .output().expect("run the staged program");
    let report = String::from_utf8_lossy(&ran.stderr);
    assert_eq!(ran.status.code(), Some(0), "execute the checked generated declaration: {report}");
    assert!(report.contains("live=0 immortal=0 leaked=0"), "{report}");

    let guarded = "module Main where\n  divide :: Nat -> Nat -> Nat\n  divide value divisor = if divisor == 0 then 0 else value / divisor\n  main :: Int\n  main = 0\n";
    let (exit, diagnostic) = run("--check", guarded);
    assert_eq!(exit, Some(0), "nonzero divisor from the actual branch: {diagnostic}");
    let unchecked = guarded.replace("if divisor == 0 then 0 else ", "");
    let (exit, diagnostic) = run("--check", &unchecked);
    assert_eq!(exit, Some(4), "unguarded divisor: {diagnostic}");
    assert!(diagnostic.contains("division needs checked NonZero evidence"), "{diagnostic}");

    let shadowed = generated.replacen("  stage bump ::", "  stage quoteNat :: Nat -> Nat\n  stage quoteNat n = n\n  stage bump ::", 1);
    let (exit, diagnostic) = run("--check", &shadowed);
    assert_eq!(exit, Some(4), "a stage function returned a number in a code splice: {diagnostic}");
    assert!(diagnostic.contains("type mismatch"), "{diagnostic}");

    let leaked = "module Main where\n  stage count :: Int\n  stage count = 42\n  main :: Int\n  main = count\n";
    let (exit, diagnostic) = run("--check", leaked);
    assert_eq!(exit, Some(4), "stage-only binding escaped: {diagnostic}");
    assert!(diagnostic.contains("name is not in scope: count"), "{diagnostic}");

    let runtime_capture = "module Main where\n  runtime-value :: Nat\n  runtime-value = 42\n  stage count :: Nat\n  stage count = runtime-value\n  main :: Int\n  main = 0\n";
    let (exit, diagnostic) = run("--check", runtime_capture);
    assert_eq!(exit, Some(4), "runtime-only binding used by stage code: {diagnostic}");
    assert!(diagnostic.contains("name is not in scope: runtime-value"), "{diagnostic}");

    let unavailable = "module Main where\n  foreign import ccall \"cinch.runtime.arrayReplicate\" copies :: forall a. Nat -> a -> Array a\n  foreign import ccall \"cinch.runtime.arrayLen\" size :: forall a. Array a -> Nat\n  stage count :: Nat\n  stage count = size (copies 2 (1 :: Int))\n  generate do\n    emit (quoteDecls where\n      result :: Nat\n      result = $(quoteNat count))\n  main :: Int\n  main = 0\n";
    let (exit, diagnostic) = run("--check", unavailable);
    assert_eq!(exit, Some(4), "compile-time array use: {diagnostic}");
    assert!(diagnostic.contains("arrayReplicate") && diagnostic.contains("not available at compile time"), "{diagnostic}");
}

fn interface_output(dir: &Path, exe: &Path) {
    let source = dir.join("interface-source.cinch");
    std::fs::write(&source, "module A where\n  export Token, wrap\n  data Token where\n    Hidden :: Nat -> Token\n  wrap :: Nat -> Token\n  wrap n = Hidden n\nmodule Main where\n  import A\n  main :: Int\n  main = 0\n").unwrap();
    let prefix = dir.join("interfaces-only");
    let args = vec!["--emit-interfaces".into(), source.display().to_string(), prefix.display().to_string()];
    let (code, stderr) = run_built(exe, dir, &args);
    assert_eq!(code, Some(0), "{stderr}; {}", std::fs::read_to_string(prefix.with_extension("diagnostics")).unwrap_or_default());
    let interfaces: Vec<_> = ["A", "Main"].into_iter().map(|name| {
        let bytes = std::fs::read_to_string(dir.join(format!("interfaces-only.{name}.cinchif"))).unwrap();
        let iface = cinch::interface::Iface::from_json(&bytes).unwrap();
        assert_eq!(iface.to_json(), bytes);
        iface
    }).collect();
    cinch::interface::verify_chain(&interfaces).unwrap();
    let token = &interfaces[0].exports.types[0];
    assert_eq!(token.identity, "A.Token");
    assert!(token.ctors.is_empty());
    assert!(!prefix.exists(), "interface-only mode emitted IR");
    assert_eq!(std::fs::read_to_string(dir.join("interfaces-only.diagnostics")).unwrap(), "");

    // Refinement definitions are supported now: require the actual checked
    // template and private closure instead of the former blanket refusal.
    std::fs::write(&source, "module Main where\n  export Positive, one\n  type Positive = refine (n :: Nat) with n > 0\n  one :: Positive\n  one = 1\n  main :: Int\n  main = 0\n").unwrap();
    let prefix = dir.join("refinement-interface");
    let args = vec!["--emit-interfaces".into(), source.display().to_string(), prefix.display().to_string()];
    let (code, stderr) = run_built(exe, dir, &args);
    assert_eq!(code, Some(0), "{stderr}; {}", common::read_output(dir, "refinement-interface.diagnostics"));
    let bytes = common::read_output(dir, "refinement-interface.Main.cinchif");
    let interface = cinch::interface::Iface::from_json(&bytes).expect("independent refinement interface readback");
    assert_eq!(interface.to_json(), bytes, "canonical refinement interface");
    let positive = interface.exports.types.iter().find(|row| row.name == "Positive").unwrap();
    assert_eq!((positive.identity.as_str(), positive.kind.as_str()), ("Main.Positive", "refinement"));
    let template = positive.refinement.as_ref().unwrap();
    assert_eq!((template.binder.as_str(), template.carrier.as_str(), template.predicate.as_str()), ("n", "Nat", "n > 0"));
    let one = interface.exports.values.iter().find(|row| row.name == "one").unwrap().resolved.as_ref().unwrap();
    assert!(one.nominals.iter().any(|row| row.identity == "Main.Positive" && row.refinement.as_ref() == Some(template)), "value lost its defining refinement contract");
    assert_eq!(common::read_output(dir, "refinement-interface.diagnostics"), "");
    assert!(!prefix.exists(), "interface-only refinement output emitted IR");

    // Unmodeled external assumptions still require an explicit refusal and
    // must never leave a checked interface behind.
    std::fs::write(&source, "module Main where\n  foreign import ccall \"external_identity\" external-identity :: Nat -> Nat\n  main :: Int\n  main = 0\n").unwrap();
    let args = vec!["--emit-interfaces".into(), source.display().to_string(), dir.join("unsupported-interface").display().to_string()];
    assert_eq!(run_built(exe, dir, &args).0, Some(4));
    assert!(common::read_output(dir, "unsupported-interface.diagnostics").contains("foreign import needs a function type, closed explicit rows and accounted result effects"));
    assert!(!dir.join("unsupported-interface.Main.cinchif").exists(), "unsupported contract silently produced an interface");
    // Typed FFI is supported, with explicit effects and a representable ABI.
    // The pure unaccounted declaration above must still refuse and leave no
    // interface; a valid foreign declaration must retain its actual contract.
    std::fs::write(&source, "module Main where\n  foreign import ccall \"external_identity\" external-identity :: I64 ->{Foreign Test} I64\n  main :: Int\n  main = 0\n").unwrap();
    let checked = dir.join("checked-foreign-interface");
    let args = vec!["--emit-interfaces".into(), source.display().to_string(), checked.display().to_string()];
    let (code, stderr) = run_built(exe, dir, &args);
    assert_eq!(code, Some(0), "{stderr}; {}", common::read_output(dir, "checked-foreign-interface.diagnostics"));
    let bytes = common::read_output(dir, "checked-foreign-interface.Main.cinchif");
    let iface = cinch::interface::Iface::from_json(&bytes).expect("checked typed foreign interface");
    assert_eq!(iface.to_json(), bytes, "canonical typed foreign interface");
    assert_eq!(iface.exports.foreign.len(), 1);
    let foreign = &iface.exports.foreign[0];
    assert_eq!((foreign.symbol.as_str(), foreign.external.as_str(), foreign.kind.as_str()), ("external-identity", "external_identity", "import"));
    assert_eq!(foreign.effects, "Foreign Test");
    assert_eq!(foreign.params.len(), 1);
    assert_eq!((foreign.params[0].ty.as_str(), foreign.result.ty.as_str()), ("I64", "I64"));
    assert_eq!(iface.assumptions.foreign, vec!["external_identity"]);
    assert_eq!(common::read_output(dir, "checked-foreign-interface.diagnostics"), "");
    assert!(!checked.exists(), "interface-only FFI output emitted IR");
    std::fs::write(&source, "module Main where\n  foreign import ccall \"external_identity\" external-identity :: Nat -> Nat\n  main :: Int\n  main = 0\n").unwrap();

    let args = vec!["--emit-ir-with-interfaces".into(), source.display().to_string(), dir.join("missing-object").display().to_string()];
    assert_eq!(run_built(exe, dir, &args).0, Some(1), "combined IR/interface output requires the checked object configuration");
}

fn frontend_modes(dir: &Path, exe: &Path) {
    let source = dir.join("mode-source.cinch");
    let valid = "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = 0\n";
    std::fs::write(&source, valid).unwrap();
    let args = vec!["--check".into(), source.display().to_string(), dir.join("checked").display().to_string()];
    assert_eq!(run_built(exe, dir, &args).0, Some(0), "check accepts typed code before lowering");
    assert_eq!(common::read_output(dir, "checked.diagnostics"), "");
    assert!(!dir.join("checked").exists(), "check emitted an IR module");
    std::fs::write(&source, "module Main where\n  main :: Int\n  main = missing\n").unwrap();
    assert_eq!(run_built(exe, dir, &args).0, Some(4));
    assert!(common::read_output(dir, "checked.diagnostics").contains("missing"));
    std::fs::write(&source, valid).unwrap();
    assert_eq!(run_built(exe, dir, &args).0, Some(0));
    assert_eq!(common::read_output(dir, "checked.diagnostics"), "", "successful check retained stale diagnostics");
    let build_args = ["--compile-only".to_string(), source.display().to_string(), dir.join("only.ll").display().to_string(), dir.join("only.bin").display().to_string(), dir.join("only.report").display().to_string()];
    assert_eq!(run_built(exe, dir, &build_args).0, Some(5), "compile retains the typed emitter boundary");
    assert!(!dir.join("only.bin").exists());
    for bad in [vec!["--check", "--compile-only", "source", "output"], vec!["--check", "source", "output", "extra"], vec!["--interfaces", "a", "--interfaces", "b", "source", "ir", "exe", "report"]] {
        assert_eq!(run_built(exe, dir, &bad.into_iter().map(String::from).collect::<Vec<_>>()).0, Some(1), "invalid mode arguments");
    }
    std::fs::write(&source, "module Main where\n  main :: Int\n  main = 37\n").unwrap();
    assert_eq!(run_built(exe, dir, &build_args).0, Some(0));
    assert_eq!(common::read_output(dir, "only.report"), "clang-exit: 0\n");
    assert_eq!(std::process::Command::new(dir.join("only.bin")).status().unwrap().code(), Some(37), "compile-only produced a wrong artifact");
    #[cfg(unix)]
    compile_only_does_not_execute(dir, exe, &build_args);
}

#[cfg(unix)]
fn compile_only_does_not_execute(dir: &Path, exe: &Path, args: &[String]) {
    use std::os::unix::fs::PermissionsExt;
    let tools = dir.join("fake-tools");
    std::fs::create_dir(&tools).unwrap();
    let clang = tools.join("clang");
    std::fs::write(&clang, r#"#!/bin/sh
cat >/dev/null
printf called > clang-was-called
if [ -n "$CINCH_TEST_CLANG_EXIT" ]; then exit "$CINCH_TEST_CLANG_EXIT"; fi
while [ "$#" -gt 0 ]; do
  if [ "$1" = "-o" ]; then shift; output="$1"; fi
  shift
done
cat > "$output" <<'PROGRAM'
#!/bin/sh
printf executed > artifact-was-run
exit 37
PROGRAM
chmod +x "$output"
"#).unwrap();
    std::fs::set_permissions(&clang, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut search = vec![tools];
    search.extend(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()));
    let path = std::env::join_paths(search).unwrap();
    let run = |words: &[String], failure: Option<&str>| {
        let mut cmd = std::process::Command::new(exe);
        cmd.current_dir(dir).args(words).env("PATH", &path).env_remove("CINCH_TEST_CLANG_EXIT");
        if let Some(code) = failure { cmd.env("CINCH_TEST_CLANG_EXIT", code); }
        cmd.output().unwrap()
    };
    std::fs::remove_file(dir.join("only.bin")).unwrap();
    assert_eq!(run(args, None).status.code(), Some(0));
    assert!(dir.join("clang-was-called").exists());
    assert!(!dir.join("artifact-was-run").exists(), "compile-only executed its output");
    assert_eq!(common::read_output(dir, "only.report"), "clang-exit: 0\n");
    assert_eq!(run(&args[1..], None).status.code(), Some(0), "legacy link-and-run failed");
    assert!(dir.join("artifact-was-run").exists(), "execution spy was not reached by legacy mode");
    assert_eq!(common::read_output(dir, "only.report"), "clang-exit: 0\nartifact-exit: 37\n");
    std::fs::remove_file(dir.join("artifact-was-run")).unwrap();
    std::fs::remove_file(dir.join("only.bin")).unwrap();
    assert_eq!(run(args, Some("19")).status.code(), Some(4), "link failure was hidden");
    assert_eq!(common::read_output(dir, "only.report"), "clang-exit: 19\n");
    assert!(!dir.join("only.bin").exists());
    assert!(!dir.join("artifact-was-run").exists());
}

fn checked_interfaces(dir: &Path, exe: &Path) {
    use cinch::interface::Iface;
    let source = common::project("pipeline_contracts", &[
        ("Lib.cinch", "module Lib where\n  export answer, increment, (<+>)\n  answer :: Int\n  answer = 42\n  increment :: Int -> Int\n  increment x = x + 1\n  private-value :: Int\n  private-value = 9\n  infixr 6 <+>\n  (<+>) :: Int -> Int -> Int\n  (<+>) x y = x - y\n"),
        ("Facade.cinch", "module Facade where\n  import Lib (answer as result, increment, (<+>) as (<*>))\n  export result, increment, (<*>)\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = result\n"),
    ]);
    common::check_library_fixture_native(&source, "Main.cinch").expect("produce real checked contracts");
    let mut rows = Vec::new();
    for entry in std::fs::read_dir(source.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" {
                rows.push(format!("[{},{},{}]", json_string(&iface.module),
                    json_string(&iface.iface_hash()), json_string(&path.display().to_string())));
            }
        }
    }
    rows.sort();
    let manifest = dir.join("interfaces.json");
    std::fs::write(&manifest, format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    for (name, input, expected, message) in [
        ("unused-contracts", "module Main where\n  main :: Int\n  main = 0\n", 0, ""),
        ("imported-value", "module Main where\n  import Facade (result)\n  main :: Int\n  main = result\n", 5, "shared uniform-word ABI"),
        ("imported-call", "module Main where\n  import Facade (increment)\n  main :: Int\n  main = increment 1\n", 5, "shared uniform-word ABI"),
        ("imported-type", "module Main where\n  import Facade (increment)\n  main :: Bool\n  main = increment 1\n", 4, "type mismatch"),
        ("imported-private", "module Main where\n  import Lib (private-value)\n  main :: Int\n  main = private-value\n", 4, "private-value"),
        ("imported-owner", "module Main where\n  import Facade\n  infixr 6 <*>\n  main :: Int\n  main = 0\n", 3, "only the module declaring operator"),
        ("imported-operator", "module Main where\n  import Facade\n  main :: Int\n  main = 10 <*> 3 <*> 2\n", 5, "shared uniform-word ABI"),
        ("instance-contract", "module Main where\n  import Text.Codec (JsonCodec)\n  data Wrapped where\n    Wrapped :: Json -> Wrapped\n  implicit instance JsonCodec_Wrapped :: JsonCodec Wrapped where\n    encode-json (Wrapped j) = j\n    decode-json j = Ok (Wrapped j)\n  main :: Int\n  main = 0\n", 5, "an instance declaration is not in the fragment"),
        ("instance-dictionary-selection", "module Main where\n  import Text.Codec (JsonCodec)\n  data Wrapped where\n    Wrapped :: Json -> Wrapped\n  implicit instance JsonCodec_Wrapped :: JsonCodec Wrapped where\n    encode-json (Wrapped j) = j\n    decode-json j = Ok (Wrapped j)\n  main :: Int\n  main = with (codec = JsonCodec_Wrapped) in 0\n", 5, "an instance declaration is not in the fragment"),
        ("instance-dictionary-not-int", "module Main where\n  import Text.Codec (JsonCodec)\n  data Wrapped where\n    Wrapped :: Json -> Wrapped\n  implicit instance JsonCodec_Wrapped :: JsonCodec Wrapped where\n    encode-json (Wrapped j) = j\n    decode-json j = Ok (Wrapped j)\n  main :: Int\n  main = JsonCodec_Wrapped\n", 4, "type mismatch: expected Int"),
        ("instance-method-hidden", "module Main where\n  import Text.Codec (JsonCodec)\n  data Wrapped where\n    Wrapped :: Json -> Wrapped\n  implicit instance JsonCodec_Wrapped :: JsonCodec Wrapped where\n    encode-json (Wrapped j) = j\n    decode-json j = Ok (Wrapped j)\n  main :: Int\n  main = encode-json (Wrapped JNull)\n", 4, "name is not in scope: encode-json"),
    ] {
        std::fs::write(dir.join(format!("{name}.cinch")), input).unwrap();
        let mut args = vec!["--interfaces".into(), manifest.display().to_string()];
        args.extend(["cinch", "ll", "bin", "report.txt"].iter()
            .map(|ext| dir.join(format!("{name}.{ext}")).display().to_string()));
        let (code, stderr) = run_built(exe, dir, &args);
        assert_eq!(code, Some(expected), "{name}: {stderr}; project {}", dir.display());
        if expected != 0 {
            let diagnostic = common::read_output(dir, &format!("{name}.ll.diagnostics"));
            assert!(diagnostic.contains(message), "{name}: {diagnostic}");
            assert!(std::fs::read(dir.join(format!("{name}.ll"))).unwrap().is_empty(), "refused source retained usable IR");
            assert!(!dir.join(format!("{name}.bin")).exists());
        }
    }
    for before in [true, false] {
        let mut args = if before { vec!["--check".into(), "--interfaces".into(), manifest.display().to_string()] } else { vec!["--interfaces".into(), manifest.display().to_string(), "--check".into()] };
        args.extend([dir.join("imported-call.cinch").display().to_string(), dir.join("imported-check").display().to_string()]);
        assert_eq!(run_built(exe, dir, &args).0, Some(0), "checked imported call must stop before its emitter refusal");
        assert_eq!(common::read_output(dir, "imported-check.diagnostics"), "");
    }
    checked_source_library_replacement(dir, exe, &manifest, &source.join("build"));
    std::fs::write(&manifest, "{\"schema\":99,\"interfaces\":[]}").unwrap();
    let args = ["--interfaces", "interfaces.json", "unused-contracts.cinch", "invalid.ll", "invalid.bin", "invalid.report"]
        .iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(run_built(exe, dir, &args).0, Some(7));
    assert!(common::read_output(dir, "invalid.ll.diagnostics").contains("unsupported interface manifest schema"));
    assert!(std::fs::read(dir.join("invalid.ll")).unwrap().is_empty());
    common::cleanup(&source);
}

fn checked_source_library_replacement(dir: &Path, exe: &Path, manifest: &Path, library: &Path) {
    use cinch::interface::Iface;
    let original = include_str!("../prelude/Data/Core.cinch");
    let main = "module Main where\n  import Data.Core\n  main :: Int\n  main = case Some (42 :: Int) of Some value -> value; None -> 1\n";
    let edited = original.replace("  data List a where", "  -- Checked from source during replacement.\n  data List a where");
    let invoke = |mode: &str, input: &str, name: &str| {
        let path = dir.join(format!("{name}.cinch"));
        std::fs::write(&path, input).unwrap();
        let args = [
            mode.to_string(), "--interfaces".into(), manifest.display().to_string(),
            path.display().to_string(), dir.join(name).display().to_string(),
        ];
        let output = std::process::Command::new(exe).current_dir(dir).args(args).env("PATH", "").output().unwrap();
        let diagnostics = common::read_output(dir, &format!("{name}.diagnostics"));
        (output.status.code(), diagnostics)
    };
    let exact = format!("{original}\n{main}");
    assert_eq!(invoke("--check", &exact, "core-source-check"), (Some(0), String::new()));
    let changed_source = format!("{edited}\n{main}");
    assert_eq!(invoke("--emit-interfaces", &changed_source, "core-source-emit"), (Some(0), String::new()));
    let load = |path: &Path| Iface::from_json(&std::fs::read_to_string(path).unwrap()).unwrap();
    let previous = load(&library.join("Data.Core.cinchif"));
    let replacement = load(&dir.join("core-source-emit.Data.Core.cinchif"));
    let native_consumer = load(&library.join("Data.Nat.cinchif"));
    let source_consumer = load(&dir.join("core-source-emit.Main.cinchif"));
    assert_ne!(replacement.source_hash, previous.source_hash, "the replacement must come from the supplied source");
    assert_eq!(replacement.iface_hash(), previous.iface_hash(), "retain the exact checked compatibility contract");
    assert!(replacement.imports.is_empty(), "Data.Core must not gain a Prelude back edge");
    for consumer in [native_consumer, source_consumer] {
        assert!(consumer.imports.iter().any(|reference| reference.module == "Data.Core" && reference.iface_hash == replacement.iface_hash()));
    }
    // No retained module consumes Data.Set, so a changed public interface can
    // be emitted and compared without invalidating an unrelated consumer.
    let implicit_core = "module Data.Set where\n  sentinel :: List Int -> Option Int\n  sentinel xs = case xs of Nil -> None; Cons x rest -> Some x\n";
    let explicit_core = implicit_core.replace("module Data.Set where", "module Data.Set where\n  import Data.Core");
    let (status, diagnostics) = invoke("--emit-interfaces", &explicit_core, "early-explicit-core");
    assert_eq!(status, Some(0), "{diagnostics}");
    let (status, diagnostics) = invoke("--emit-interfaces", implicit_core, "early-implicit-core");
    assert_eq!(status, Some(0), "{diagnostics}");
    let implicit = load(&dir.join("early-implicit-core.Data.Set.cinchif"));
    let explicit = load(&dir.join("early-explicit-core.Data.Set.cinchif"));
    assert_eq!(implicit.iface_hash(), explicit.iface_hash());
    assert_ne!(implicit.source_hash, explicit.source_hash);
    assert_eq!(implicit.imports.len(), 1);
    assert_eq!(implicit.imports[0].module, "Data.Core");
    assert_eq!(implicit.imports[0].iface_hash, previous.iface_hash());

    let qualified_without_import = implicit_core.replace("List Int", "Data.Core.List Int");
    let (status, diagnostics) = invoke("--check", &qualified_without_import, "early-qualified-core");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("unknown type constructor or unbound type variable: Data.Core.List"), "{diagnostics}");

    let copy_array = r#"module CopyExample where
  from-shared :: forall r. Ref r (Array Int) -> Array Int
  from-shared value = copyOut value
"#;
    let (status, diagnostics) = invoke("--check", copy_array, "copy-immutable-array");
    assert_eq!(status, Some(0), "{diagnostics}");
    let affine_elements = copy_array.replace("Array Int", "Array (Thunk {} Int)");
    let (status, diagnostics) = invoke("--check", &affine_elements, "refuse-affine-array-copy");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("copyOut requires structural Copy evidence"), "{diagnostics}");

    let class_law = "module LawExample where\n  signature Restore a where\n    restore :: a -> a\n    law identity :: (x :: a) -> Eq (restore x) x\n";
    let (status, diagnostics) = invoke("--emit-interfaces", class_law, "unchecked-class-law");
    assert_eq!(status, Some(0), "{diagnostics}");
    assert!(load(&dir.join("unchecked-class-law.LawExample.cinchif")).laws.is_empty());
    let different = class_law.replace("Eq (restore x) x", "Eq (restore x) True");
    let (status, diagnostics) = invoke("--emit-interfaces", &different, "wrong-law-carrier");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("type mismatch"), "{diagnostics}");
    assert!(!dir.join("wrong-law-carrier.LawExample.cinchif").exists());
    let unresolved = class_law.replace("restore x", "not-in-law-scope x");
    let (status, diagnostics) = invoke("--check", &unresolved, "unresolved-class-law");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("name is not in scope: not-in-law-scope"), "{diagnostics}");

    let function_former = r#"module ArrowExample where
  signature Mapping (f :: Type -> Type) where
    project :: forall a b. (a -> b) -> f a -> f b
  implicit instance Mapping_Function r :: Mapping ((->) r) where
    project f g x = f (g x)
  direct :: ((->) Int) Int
  direct x = x
  use-project :: Int -> Int
  use-project = project (\n -> n + 1) direct
"#;
    let (status, diagnostics) = invoke("--check", function_former, "structural-function-instance");
    assert_eq!(status, Some(0), "{diagnostics}");
    let too_many = function_former.replace("((->) Int) Int", "(->) Int Int Int");
    let (status, diagnostics) = invoke("--check", &too_many, "overapplied-function-constructor");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("type constructor is applied to too many arguments"), "{diagnostics}");

    let boolean_law = "module LawExample where\n  signature Restore a where\n    restore :: a -> a\n    law involution :: (x :: a) -> Eq (restore (restore x)) x\n  implicit instance Restore_Bool :: Restore Bool where\n    restore x = if x then False else True\n";
    let (status, diagnostics) = invoke("--emit-interfaces", boolean_law, "sampled-boolean-law");
    assert_eq!(status, Some(0), "{diagnostics}");
    let sampled = load(&dir.join("sampled-boolean-law.LawExample.cinchif"));
    assert_eq!(sampled.laws.len(), 1);
    assert_eq!(sampled.laws[0].instance, "LawExample.Restore_Bool");
    assert_eq!(sampled.laws[0].status, "checked (2 samples)");
    let unlawful = boolean_law.replace("if x then False else True", "False");
    let (status, diagnostics) = invoke("--emit-interfaces", &unlawful, "refused-boolean-law");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("instance `Restore_Bool` breaks class law `LawExample.Restore.involution` on a Boolean sample"), "{diagnostics}");
    assert!(!dir.join("refused-boolean-law.LawExample.cinchif").exists());
    let finite_law = r#"module EnumLaw where
  data Comparison where
    Less :: Comparison
    Same :: Comparison
    Greater :: Comparison
  signature Combine a where
    combine :: a -> a -> a
    law associate :: (x :: a) -> (y :: a) -> (z :: a) -> Eq (combine (combine x y) z) (combine x (combine y z))
  signature UnitOf a where
    unit-of :: a
    law left-unit :: (x :: a) -> Eq (combine unit-of x) x
  implicit instance CombineComparison :: Combine Comparison where
    combine x y = case x of Less -> Less; Same -> y; Greater -> Greater
  implicit instance UnitComparison :: UnitOf Comparison where
    unit-of = Same
"#;
    let (status, diagnostics) = invoke("--emit-interfaces", finite_law, "sampled-finite-law");
    assert_eq!(status, Some(0), "{diagnostics}");
    let sampled = load(&dir.join("sampled-finite-law.EnumLaw.cinchif"));
    let law = |name: &str| sampled.laws.iter().find(|item| item.law == name).unwrap().status.as_str();
    assert_eq!(law("associate"), "checked (27 samples)");
    assert_eq!(law("left-unit"), "checked (3 samples)");
    let wrong_unit = finite_law.replace("unit-of = Same", "unit-of = Greater");
    let (status, diagnostics) = invoke("--emit-interfaces", &wrong_unit, "refused-finite-law");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("instance `UnitComparison` breaks class law `EnumLaw.UnitOf.left-unit` on a finite constructor sample"), "{diagnostics}");
    assert!(!dir.join("refused-finite-law.EnumLaw.cinchif").exists());
    let function_law = format!("{class_law}  implicit instance Restore_Function :: Restore (Int -> Int) where\n    restore f = f\n");
    let (status, diagnostics) = invoke("--emit-interfaces", &function_law, "unsampled-function-law");
    assert_eq!(status, Some(0), "{diagnostics}");
    let unsampled = load(&dir.join("unsampled-function-law.LawExample.cinchif"));
    assert_eq!(unsampled.laws.len(), 1);
    assert_eq!(unsampled.laws[0].instance, "LawExample.Restore_Function");
    assert_eq!(unsampled.laws[0].status, "not checked (no samples for the law binder `x :: (Int -> Int)`)");

    let incompatible = edited.replace("Option, None, Some", "Option, None, Some, Missing")
        .replace("    None :: Option a", "    None :: Option a\n    Missing :: Option a");
    let (status, diagnostics) = invoke("--emit-interfaces", &format!("{incompatible}\n{main}"), "stale-core-source");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("stale interface dependency: Data.Core"), "{diagnostics}");
    assert!(!dir.join("stale-core-source.Data.Core.cinchif").exists());
    assert!(!dir.join("stale-core-source.Main.cinchif").exists());
    let (status, diagnostics) = invoke("--check", "module Lib where\n  answer :: Int\n  answer = 42\n", "ordinary-collision");
    assert_eq!(status, Some(4), "{diagnostics}");
    assert!(diagnostics.contains("source module conflicts with a loaded interface"), "{diagnostics}");
}

fn json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            ch if ch <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn refused_source(dir: &Path, exe: &Path) {
    // A well-formed program whose body is not in the fragment: the checker
    // or the emitter must refuse it by name rather than emit nonsense.
    std::fs::write(
        dir.join("broken.cinch"),
        "module Main where\n  main :: Int\n  main = missing-name\n",
    )
    .expect("write the refused source");
    let words: Vec<String> = [
        "broken.cinch",
        "broken.ll",
        "broken.bin",
        "broken-report.txt",
    ]
    .iter()
    .map(|s| dir.join(s).display().to_string())
    .collect();

    // Refusal truncates the requested output, invalidating an earlier build.
    std::fs::write(dir.join("broken.ll"), "stale IR from an earlier successful build").unwrap();
    let (code, stderr) = run_built(exe, dir, &words);
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
        std::fs::read(dir.join("broken.ll")).unwrap().is_empty(),
        "a refused source must clear earlier IR"
    );
    assert!(
        !dir.join("broken.bin").exists(),
        "a refused source must not leave an executable behind"
    );

    let cases = [
        ("fixity-owner", "module A where\n  infixr 6 <+>\n  (<+>) :: Int -> Int -> Int\n  (<+>) a b = a - b\nmodule Main where\n  import A\n  infixr 6 <+>\n  main :: Int\n  main = 0\n", 3, "only the module declaring operator"),
        ("function-value", "module Main where\n  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = let f = identity in f 3\n", 5, "closure lowering"),
        ("builtin-capability", "module Main where\n  main :: Unit ->{IO r} Int\n  main unit = with (io = Runtime.newIO ()) in let writer = ioWrite in 0\n", 5, "shared uniform-word ABI"),
        ("capability-scope", "module Main where\n  main :: Int\n  main = with (io = io) in 0\n", 4, "name is not in scope: io"),
        ("foreign-contract", "module Main where\n  foreign import ccall \"external_identity\" ffi-id :: Int -> Int\n  main :: Int\n  main = ffi-id 1\n", 4, "foreign import needs a function type, closed explicit rows and accounted result effects"),
        ("intrinsic-lowering", "module Main where\n  foreign import ccall \"cinch.runtime.textSha256\" digest :: Bytes -> Bytes\n  main :: Int\n  main = 0\n", 5, "foreign import"),
        ("intrinsic-module-alias", "module Hash where\n  export hash\n  foreign import ccall \"cinch.runtime.textSha256\" hash :: Bytes -> Bytes\nmodule Main where\n  import Hash (hash as digest)\n  probe :: Bytes -> Bytes\n  probe input = digest input\n  main :: Int\n  main = 0\n", 5, "foreign import"),
        ("effect-declaration-lowering", "module Main where\n  effect Event where\n    poke :: Int ->{Event} Int\n  main :: Int\n  main = 0\n", 5, "is not in the fragment"),
        ("effect-call-lowering", "module Main where\n  effect Event where\n    poke :: Int ->{Event} Int\n  invoke :: Int ->{Event} Int\n  invoke x = poke x\n  main :: Int\n  main = 0\n", 5, "is not in the fragment"),
        ("handler-lowering", "module Main where\n  effect Event where\n    poke :: Int ->{Event} Int\n  main :: Int\n  main = handle poke 1 with (handler Event where\n    return x -> x\n    poke n resume k -> k n)\n", 5, "the emitter cannot lower this handle"),
        ("handler-effect-erasure", "module Main where\n  effect Event where\n    poke :: Int ->{Event} Int\n  effect Other where\n    outside :: Unit ->{Other} Int\n  main :: Int\n  main = handle poke 1 with (handler Event where\n    return x -> x\n    poke n resume k -> outside ())\n", 4, "performed effects exceed the declared row"),
        ("decision-contract", "module Main where\n  main :: Int\n  main = case decide (1 > 0) of\n    Yes evidence -> 0\n    No contrary -> 1\n", 5, "the emitter cannot lower this case"),
        ("polymorphic-lowering", "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = if identity True then identity 0 else 1\n", 5, "a forall the fragment does not lower"),
        ("polymorphic-body", "module Main where\n  wrong :: forall a. a -> a\n  wrong x = 1\n  main :: Int\n  main = 0\n", 4, "type mismatch"),
        ("named-pi-lowering", "module Main where\n  keep :: (n :: Int) -> Int\n  keep actual = actual\n  main :: Int\n  main = keep 0\n", 5, "a type without a fragment representation"),
        ("guarded-nat-lowering", "module Main where\n  pred :: Nat -> Nat\n  pred n = if n > 0 then n - 1 else 0\n  main :: Int\n  main = 0\n", 5, "a type without a fragment representation"),
        ("refinement-result-lowering", "module Main where\n  one :: refined (r > 0) Nat\n  one = 1\n  main :: Int\n  main = 0\n", 5, "a refined result the fragment does not lower"),
        ("refinement-ignored-invalid", "module Main where\n  main :: Int\n  main = let ignored = (0 :: refined (r > 0) Int) in 0\n", 4, "refinement predicate is not established"),
        ("conditional-call-lowering", "module Main where\n  pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  answer :: Nat\n  answer = pred 1\n  main :: Int\n  main = 0\n", 5, "a using constraint the fragment does not lower"),
        ("conditional-call-ignored", "module Main where\n  pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  main :: Int\n  main = let ignored = pred 0 in 0\n", 4, "function precondition is not established"),
        ("indexed-pi-lowering", "module Main where\n  data At (n :: Nat) where\n    Mark :: At n\n  make :: (n :: Nat) -> At n\n  make actual = Mark\n  main :: Int\n  main = 0\n", 5, "typed constructor lowering"),
        ("missing-record-field", "module Main where\n  record Person where\n    age :: Int\n  salary :: Person -> Int\n  salary p = p.salary\n  main :: Int\n  main = 0\n", 4, "unknown record field salary"),
        ("fixed-index-mismatch", "module Main where\n  data At (n :: Nat) where\n    One :: At 1\n  wrong :: At 2\n  wrong = One\n  main :: Int\n  main = 0\n", 4, "type mismatch: expected 2, found 1 (index mismatch)"),
        ("constructor-lowering", "module Main where\n  data Box a where\n    Box :: a -> Box a\n  unwrap :: forall a. Box a -> a\n  unwrap (Box x) = x\n  main :: Int\n  main = unwrap (Box 0)\n", 5, "typed constructor lowering"),
    ];
    for (name, source, expected, message) in cases {
        std::fs::write(dir.join(format!("{name}.cinch")), source).unwrap();
        let words: Vec<String> = ["cinch", "ll", "bin", "report.txt"].iter()
            .map(|extension| dir.join(format!("{name}.{extension}")).display().to_string()).collect();
        let (code, stderr) = run_built(exe, dir, &words);
        assert_eq!(code, Some(expected), "{name}: {stderr}");
        let diagnostic = common::read_output(dir, &format!("{name}.ll.diagnostics"));
        assert!(diagnostic.contains(message), "{name}: {diagnostic}");
        assert!(std::fs::read(dir.join(format!("{name}.ll"))).unwrap().is_empty(), "refused source retained usable IR");
        assert!(!dir.join(format!("{name}.bin")).exists());
    }
}

#[test]
fn diagnostic_origins_retain_actual_bundle_labels_and_local_lines() {
    let dir = common::project("diagnostic_origins", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.DiagnosticOrigins.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "diagnostic-origins").expect("build actual Cinch diagnostic renderer");
    let mut bundle = String::from("-- source: Earlier.cinch\nmodule Earlier where\n");
    bundle.push_str(&format!("-- {}\n", "é".repeat(50)).repeat(20_000));
    bundle.push_str("-- source: Étape.cinch\nmodule Étape where\n  broken = x");
    std::fs::write(dir.join("large-bundle"), bundle).unwrap();
    std::fs::write(dir.join("invalid-utf8"), b"-- source: NoLabel.cinch\nmodule Main where\n\xff").unwrap();
    let output = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "diagnostic origin bitmask; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    common::cleanup(&dir);
}
