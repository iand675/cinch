//! Runtime-fed Cinch source becomes typed Core and shared-runtime LLVM.
//! Bignums and lexical aliases run with reclamation and leak checks enabled.
mod common;
use std::process::Command;

#[test]
fn typed_runtime_numbers_preserve_exact_values_and_reclaim_owned_words() {
    let dir = common::project("runtime_ir", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeIr.cinch")),
    ]);
    let compiler = common::build(&dir, "Main.cinch", "runtime-ir").expect("build the Cinch runtime emitter");
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    assert!(runtime.is_file(), "the Native-built emitter must have built its shared runtime");
    let cases = [
        ("huge", "  main :: Int\n  main = if 340282366920938463463374607431768211456 + 7 == 340282366920938463463374607431768211463 then 37 else 1\n", 37),
        ("alias", "  main :: Int\n  main = let x = 9223372036854775808 in let y = x * x in if y - x * x == 0 then 39 else 1\n", 39),
        ("call", "  identity :: Int -> Int\n  identity x = x\n  main :: Int\n  main = if identity 340282366920938463463374607431768211456 == 340282366920938463463374607431768211456 then 41 else 1\n", 41),
        ("branch", "  pick :: Bool -> Int -> Int -> Int\n  pick flag x y = if flag then x else y\n  main :: Int\n  main = if pick False 9223372036854775808 18446744073709551616 == 18446744073709551616 then 43 else 1\n", 43),
        ("recursive", "  sum :: Int -> Int\n  sum n = if n <= 0 then 9223372036854775808 else n + sum (n - 1)\n  main :: Int\n  main = if sum 20 == 9223372036854776018 then 47 else 1\n", 47),
        ("negative", "  main :: Int\n  main = let x = -9223372036854775809 in if -x == 9223372036854775809 then 53 else 1\n", 53),
        ("natural", "  previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else n - 1\n  main :: Int\n  main = if previous 340282366920938463463374607431768211456 == 340282366920938463463374607431768211455 then 59 else 1\n", 59),
        ("wildcard", "  pick :: Int -> Int -> Int\n  pick _ x = x\n  main :: Int\n  main = if pick 18446744073709551616 9223372036854775808 == 9223372036854775808 then 61 else 1\n", 61),
    ];
    for (name, body, expected) in cases {
        std::fs::write(dir.join("input.source"), format!("module Main where\n{body}")).unwrap();
        let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{name}: {} {}", common::read_output(&dir, "output.diagnostics"), String::from_utf8_lossy(&output.stderr));
        let exe = dir.join(format!("emitted-{name}"));
        let output = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .arg(dir.join("output.ll")).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe).output().unwrap();
        assert!(output.status.success(), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        let output = Command::new(&exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        let stderr = String::from_utf8_lossy(&output.stderr);
        let leak = stderr.lines().find(|line| line.starts_with("cinch-leak:")).expect("runtime leak report");
        assert!(leak.split_whitespace().any(|word| word == "leaked=0"), "{name}: {stderr}");
        println!("{name}: exit {expected}; {leak}");
    }
    // The typed frontend accepts this; unavailable lowering must refuse it
    // explicitly and clear the preceding successful IR artifact.
    std::fs::write(dir.join("input.source"), "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = identity 1\n").unwrap();
    let output = Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(5), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(common::read_output(&dir, "output.diagnostics").contains("polymorphic"));
    assert!(common::read_output(&dir, "output.ll").is_empty());
    common::cleanup(&dir);
}
