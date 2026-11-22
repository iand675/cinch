//! Checked machine-value comparisons keep their carrier across defining interfaces.
mod common;
use cinch::interface::Iface;

/// Source obligations distinguish a scalar's mathematical reading from its runtime carrier.
fn character_cases() -> Vec<(&'static str, &'static str, i32)> {
    vec![
        ("character conversion keeps actual identity", "safe :: (c :: Char) -> refined (out == c) U32\n  safe c = codepoint c", 0),
        ("character type supplies scalar bounds", "safe :: Char -> Char\n  safe c = scalar-bound c", 0),
        ("character identity belongs to the actual input", "bad :: (c :: Char) -> (d :: Char) -> refined (out == c) U32\n  bad c d = codepoint d", 4),
        ("scalar bounds cannot strengthen", "need :: using (c < 1114111) => (c :: Char) -> Char\n  need c = c\n  bad :: Char -> Char\n  bad c = need c", 4),
        ("converted scalar cannot exceed its bound", "bad :: Char -> refined (out > 1114111) U32\n  bad c = codepoint c", 4),
        ("character remains a distinct runtime carrier", "bad :: Char -> U32\n  bad c = c", 4),
        ("character has no numeric literal constructor", "bad :: Char\n  bad = 955", 4),
        ("character arithmetic is not a runtime integer operation", "bad :: Char -> Char\n  bad c = c + c", 4),
        ("floating proofs remain opaque", "bad :: (x :: F64) -> refined (out == x) U32\n  bad x = 0", 4),
    ]
}

/// Write a coherent interface set, optionally retaining the older display-only character contract encoding.
fn character_manifest(source: &std::path::Path, consumer: &std::path::Path, display_only: bool) {
    let mut interfaces = std::collections::BTreeMap::new();
    for entry in std::fs::read_dir(source.join("build")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let interface = Iface::from_json(&std::fs::read_to_string(path).unwrap()).unwrap();
            if interface.module != "Main" { interfaces.insert(interface.module.clone(), interface); }
        }
    }
    if display_only {
        for module in ["A", "Facade"] {
            let value = interfaces.get_mut(module).unwrap().exports.values.iter_mut()
                .find(|value| value.name == "codepoint").unwrap();
            assert!(value.total_body.is_none() && value.keyed_ordering.is_none(),
                "the display-only compatibility fixture has no executable body contract");
            value.resolved = None;
        }
        let defining_hash = interfaces["A"].iface_hash();
        for imported in &mut interfaces.get_mut("Facade").unwrap().imports {
            if imported.module == "A" { imported.iface_hash = defining_hash.clone(); }
        }
    }
    let directory = consumer.join(if display_only { "display-interfaces" } else { "resolved-interfaces" });
    std::fs::create_dir_all(&directory).unwrap();
    let mut rows = Vec::new();
    for (name, interface) in interfaces {
        let path = directory.join(format!("{name}.cinchif"));
        std::fs::write(&path, interface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(&name), quote(&interface.iface_hash()), quote(&path.display().to_string())));
    }
    std::fs::write(consumer.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
}
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
fn producer_project() -> std::path::PathBuf {
    common::project("machine_contract_producer", &[
        ("A.cinch", r#"module A where
  import Data.Nat (nat-to-i64, nat-from-i64)
  import Data.Int (int-to-i64)
  import Data.Char (char-to-u32)
  export small, widen, small-int, fixed, fixed-unsigned, opaque-machine-add
  export codepoint, scalar-bound, opaque-character-arithmetic
  -- | Keep the exact source character identity and its scalar bounds on the converted value.
  codepoint :: (c :: Char) -> refined (0 <= out && out <= 1114111 && out == c) U32
  codepoint c = char-to-u32 c
  -- | Require only bounds already guaranteed by every Unicode scalar value.
  scalar-bound :: using (0 <= c, c <= 1114111) => (c :: Char) -> Char
  scalar-bound c = c
  -- | Logical scalar readings do not authorize exporting runtime character arithmetic.
  opaque-character-arithmetic :: using (c + 1 > c) => (c :: Char) -> Char
  opaque-character-arithmetic c = c
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
        ("Facade.cinch", "module Facade where\n  import A (small as narrow, widen, small-int as narrow-int, fixed, fixed-unsigned, codepoint, scalar-bound)\n  export narrow, widen, narrow-int, fixed, fixed-unsigned, codepoint, scalar-bound\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ])
}
#[test]
fn checked_machine_comparisons_export_but_machine_arithmetic_stays_opaque() {
    let dir = producer_project();
    common::check(&dir, "Main.cinch").expect("emit the selected compiler's machine contracts");
    let read = |name: &str| common::project_interface(&dir, "Main.cinch", name);
    let a = read("A"); let facade = read("Facade");
    for (name, alias) in [("small", "narrow"), ("widen", "widen"), ("small-int", "narrow-int"), ("fixed", "fixed"), ("fixed-unsigned", "fixed-unsigned"), ("codepoint", "codepoint"), ("scalar-bound", "scalar-bound")] {
        let value = a.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().expect(name);
        let exported = facade.exports.values.iter().find(|v| v.name == alias).unwrap().resolved.as_ref().expect(alias);
        assert_eq!(exported, value, "{name}: facade must keep defining carrier and predicates");
    }
    assert!(a.exports.values.iter().find(|v| v.name == "opaque-machine-add").unwrap().resolved.is_none(), "machine + may wrap and must not be published as exact index arithmetic");
    assert!(a.exports.values.iter().find(|v| v.name == "opaque-character-arithmetic").unwrap().resolved.is_none(), "a character's scalar reading must not turn its arithmetic into a portable computation");
    for (module, name) in [("Data.Nat", "nat-to-i64"), ("Data.Nat", "nat-from-i64"), ("Data.Int", "int-to-i64")] {
        let iface = read(module);
        let value = iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().expect(name);
        assert!(value.scheme.contains("using") && value.scheme.contains("refined") && value.scheme.contains("I64"), "{module}.{name}: {}", value.scheme);
        assert_eq!(Iface::from_json(&iface.to_json()).unwrap(), iface);
    }
    let character = a.exports.values.iter().find(|v| v.name == "codepoint").unwrap().resolved.as_ref().unwrap();
    assert!(character.scheme.contains("Char") && character.scheme.contains("U32")
        && character.scheme.contains("==") && character.scheme.contains("1114111"), "{}", character.scheme);
    common::cleanup(&dir);
}
#[test]
fn compiled_cinch_consumer_rechecks_conversion_bounds_and_result_carriers() {
    let source = producer_project();
    common::check_library_fixture_native(&source, "Main.cinch")
        .expect("emit Native machine contracts for the Cinch consumer");
    let dir = common::project("machine_contract_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../compiler/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../compiler/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../compiler/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../compiler/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../compiler/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.ImportedPreconditions.cinch")),
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
    for display_only in [false, true] {
        character_manifest(&source, &dir, display_only);
        for (name, body, status) in character_cases() {
            let indented = format!("  {body}");
            let documented = indented.lines().map(|line| {
                if line.contains(" :: ") {
                    let indentation = &line[..line.len() - line.trim_start().len()];
                    format!("{indentation}-- | Check this character proof obligation without changing runtime carriers.\n{line}")
                } else { line.to_string() }
            }).collect::<Vec<_>>().join("\n");
            let input = format!("-- | Recheck a character contract through a coherent defining interface set.\nmodule Main where\n  import Facade (codepoint, scalar-bound)\n{documented}\n  -- | Observe that the complete source and its retained Core evidence check.\n  main :: Int\n  main = 0\n");
            std::fs::write(dir.join("source.cinch"), &input).unwrap();
            let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
            let report = common::read_output(&dir, "imported-report.txt");
            assert_eq!(result.status.code(), Some(status), "{name}, display_only={display_only}; project {}\n{input}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
            if status == 0 { assert!(report.is_empty(), "{name}: {report}"); }
            println!("checked character contract case passed: {name}, display_only={display_only}");
        }
    }
    common::cleanup(&source); common::cleanup(&dir);
}

/// Imported scalar conversion preserves literal values, Unicode boundaries and inverse rejection behavior.
#[test]
fn imported_character_conversion_runs_with_exact_scalar_results() {
    let dir = common::project("character_contract_runtime", &[
        ("Main.cinch", include_str!("../../compiler/programs/Run.CharConversion.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "character-contracts").expect("build actual character conversions");
    let result = std::process::Command::new(exe).env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(result.status.code(), Some(0), "{}", String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
