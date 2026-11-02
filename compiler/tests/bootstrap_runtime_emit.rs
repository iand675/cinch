//! The real compiler CLI exposes checked typed IR without linking or running.
mod common;
use std::process::Command;

#[test]
fn compiler_emit_mode_uses_the_typed_runtime_backend_and_preserves_refusals() {
    let dir = common::project("runtime_emit", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Ir.cinch", include_str!("../../bootstrap/Bootstrap.Ir.cinch")),
        ("Bootstrap.Driver.cinch", include_str!("../../bootstrap/Bootstrap.Driver.cinch")),
        ("Bootstrap.Pipeline.cinch", include_str!("../../bootstrap/Bootstrap.Pipeline.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Pipeline.cinch")),
    ]);
    let compiler = common::build(&dir, "Main.cinch", "compiler").expect("build the actual CLI compiler");
    let invoke = |args: &[&str]| Command::new(&compiler).args(args).current_dir(&dir).output().unwrap();
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = if identity 340282366920938463463374607431768211456 + 3 == 340282366920938463463374607431768211459 then 67 else 1\n").unwrap();
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    for args in [
        vec!["--emit-ir", "input.source", "output.ll"],
        vec!["--emit-ir", "--interfaces", "interfaces.json", "input.source", "output.ll"],
        vec!["--interfaces", "interfaces.json", "--emit-ir", "input.source", "output.ll"],
    ] {
        let output = invoke(&args);
        assert_eq!(output.status.code(), Some(0), "{} {}", common::read_output(&dir, "output.ll.diagnostics"), String::from_utf8_lossy(&output.stderr));
        assert!(common::read_output(&dir, "output.ll").contains("@cinch_int_add"));
        assert!(common::read_output(&dir, "output.ll.diagnostics").is_empty());
    }
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(dir.join("artifact")).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let output = Command::new(dir.join("artifact")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").output().unwrap();
    assert_eq!(output.status.code(), Some(67), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"));
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = identity 1\n").unwrap();
    assert_eq!(invoke(&["--check", "input.source", "checked"]).status.code(), Some(0));
    assert_eq!(invoke(&["--emit-ir", "input.source", "output.ll"]).status.code(), Some(5));
    assert!(common::read_output(&dir, "output.ll").is_empty(), "failed emission retained stale IR");
    assert!(common::read_output(&dir, "output.ll.diagnostics").contains("polymorphic"));
    assert_eq!(invoke(&["--check", "--emit-ir", "input.source", "output.ll"]).status.code(), Some(1));
    common::cleanup(&dir);
}
