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
    interface_output(&dir, &exe);
    refused_source(&dir, &exe);
    checked_interfaces(&dir, &exe);
    common::cleanup(&dir);
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
    assert!(common::read_output(dir, "unsupported-interface.diagnostics").contains("foreign declarations are not supported without a checked runtime intrinsic contract"));
    assert!(!dir.join("unsupported-interface.Main.cinchif").exists(), "unsupported contract silently produced an interface");
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
    std::fs::write(&manifest, "{\"schema\":99,\"interfaces\":[]}").unwrap();
    let args = ["--interfaces", "interfaces.json", "unused-contracts.cinch", "invalid.ll", "invalid.bin", "invalid.report"]
        .iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(run_built(exe, dir, &args).0, Some(7));
    assert!(common::read_output(dir, "invalid.ll.diagnostics").contains("unsupported interface manifest schema"));
    assert!(std::fs::read(dir.join("invalid.ll")).unwrap().is_empty());
    common::cleanup(&source);
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
        ("foreign-contract", "module Main where\n  foreign import ccall \"external_identity\" ffi-id :: Int -> Int\n  main :: Int\n  main = ffi-id 1\n", 4, "foreign declarations are not supported"),
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
