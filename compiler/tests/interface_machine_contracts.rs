//! Checked machine-value comparisons keep their carrier across defining interfaces.
mod common;
use cinch::interface::Iface;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"'); out
}
fn producer() -> std::path::PathBuf {
    let dir = common::project("machine_contract_producer", &[
        ("A.cinch", r#"module A where
  import Data.Nat (nat-to-i64, nat-from-i64)
  import Data.Int (int-to-i64)
  export small, widen, small-int, fixed, fixed-unsigned, opaque-machine-add
  small :: using (n <= 9223372036854775807) => (n :: Nat) -> refined (out == n) I64
  small n = nat-to-i64 n
  widen :: using (n >= 0) => (n :: I64) -> refined (out == n) Nat
  widen n = nat-from-i64 n
  small-int :: using (-9223372036854775808 <= n, n <= 9223372036854775807) => (n :: Int) -> refined (out == n) I64
  small-int n = int-to-i64 n
  fixed :: refined (n >= 0) I64
  fixed = (1 :: I64)
  fixed-unsigned :: refined (n >= 0) U64
  fixed-unsigned = (0 :: U64)
  opaque-machine-add :: using (n + 1 > n) => (n :: I64) -> I64
  opaque-machine-add n = n
"#),
        ("Facade.cinch", "module Facade where\n  import A (small as narrow, widen, small-int as narrow-int, fixed, fixed-unsigned)\n  export narrow, widen, narrow-int, fixed, fixed-unsigned\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit real checked conversion and comparison contracts");
    dir
}
#[test]
fn checked_machine_comparisons_export_but_machine_arithmetic_stays_opaque() {
    let dir = producer();
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A"); let facade = read("Facade");
    for (name, alias) in [("small", "narrow"), ("widen", "widen"), ("small-int", "narrow-int"), ("fixed", "fixed"), ("fixed-unsigned", "fixed-unsigned")] {
        let value = a.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().expect(name);
        let exported = facade.exports.values.iter().find(|v| v.name == alias).unwrap().resolved.as_ref().expect(alias);
        assert_eq!(exported, value, "{name}: facade must keep defining carrier and predicates");
    }
    assert!(a.exports.values.iter().find(|v| v.name == "opaque-machine-add").unwrap().resolved.is_none(), "machine + may wrap and must not be published as exact index arithmetic");
    for (module, name) in [("Data.Nat", "nat-to-i64"), ("Data.Nat", "nat-from-i64"), ("Data.Int", "int-to-i64")] {
        let iface = read(module);
        let value = iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().expect(name);
        assert!(value.scheme.contains("using") && value.scheme.contains("refined") && value.scheme.contains("I64"), "{module}.{name}: {}", value.scheme);
        assert_eq!(Iface::from_json(&iface.to_json()).unwrap(), iface);
    }
    common::cleanup(&dir);
}
#[test]
fn compiled_cinch_consumer_rechecks_conversion_bounds_and_result_carriers() {
    let source = producer();
    let dir = common::project("machine_contract_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let mut rows = Vec::new();
    for entry in std::fs::read_dir(source.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" {
                rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
            }
        }
    }
    rows.sort(); std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "machine-contracts").expect("build actual Cinch interface consumer");
    for (name, body, status) in [
        ("checked Nat narrowing", "safe :: Nat -> I64\n  safe n = if n <= 9223372036854775807 then narrow n else 0", 0),
        ("narrowing needs upper bound", "bad :: Nat -> I64\n  bad n = narrow n", 4),
        ("bound belongs to actual argument", "bad :: Nat -> Nat -> I64\n  bad n m = if n <= 9223372036854775807 then narrow m else 0", 4),
        ("checked result keeps actual Nat", "safe :: using (n <= 9223372036854775807) => (n :: Nat) -> refined (out == n) I64\n  safe n = narrow n", 0),
        ("checked conversion round trip", "safe :: using (n <= 9223372036854775807) => (n :: Nat) -> Nat\n  safe n = let small = narrow n in widen small", 0),
        ("signed narrowing checks both bounds", "safe :: Int -> I64\n  safe n = if -9223372036854775808 <= n && n <= 9223372036854775807 then narrow-int n else 0", 0),
        ("signed narrowing needs lower bound", "bad :: Int -> I64\n  bad n = if n <= 9223372036854775807 then narrow-int n else 0", 4),
        ("declared signed result", "safe :: refined (out >= 0) I64\n  safe = fixed", 0),
        ("declared unsigned result", "safe :: refined (out >= 0) U64\n  safe = fixed-unsigned", 0),
        ("scoped machine comparison", "safe :: I64 -> Nat\n  safe n = if n >= 0 then widen n else 0", 0),
        ("wrong machine identity", "bad :: I64 -> I64 -> Nat\n  bad n m = if n >= 0 then widen m else 0", 4),
        ("local ordering is opaque", "bad :: using (ordering :: Ord I64) => I64 -> Nat\n  bad n = if n >= 0 then widen n else 0", 4),
        ("result cannot change carrier", "bad :: refined (out >= 0) Nat\n  bad = fixed", 4),
    ] {
        let input = format!("module Main where\n  import Facade (narrow, widen, narrow-int, fixed, fixed-unsigned)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &input).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "imported-report.txt");
        assert_eq!(result.status.code(), Some(status), "{name}; project {}\n{input}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        println!("checked machine contract case passed: {name}");
    }
    common::cleanup(&source); common::cleanup(&dir);
}
