//! Actual versioned measure contracts are consumed and rechecked by compiled Cinch.
mod common;
use cinch::interface::Iface;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
fn fixture(changed: bool) -> Vec<Iface> {
    let source = r#"module A where
  import Algebra.Arithmetic (InRange)
  export width, cap, same-argument, previous
  hidden :: Nat -> InRange 1 4 Nat
  hidden n = if n < 128 then 1 else 4
  width :: Nat -> InRange 1 4 Nat
  width n = hidden n
  cap :: using (hidden n <= 4) => (n :: Nat) -> Nat
  cap actual = actual
  same-argument :: using (width a <= 2) => (a :: Nat) -> (b :: Nat) -> Nat
  same-argument a b = b
  previous :: using (n > 0) => (n :: Nat) -> Nat
  previous actual = actual - 1
module Facade where
  import A (width as measure, cap, same-argument, previous)
  export measure, cap, same-argument, previous
module Main where
  main :: Int
  main = 0
"#;
    let source = if changed { source.replace("InRange 1 4 Nat", "InRange 5 7 Nat").replace("then 1 else 4", "then 5 else 7") } else { source.to_string() };
    let (_, interfaces) = cinch::driver::check_sources_ifaces(&source, "actual-measure-contracts.cinch").unwrap();
    let a = interfaces.iter().find(|i| i.module == "A").unwrap();
    assert!(a.exports.values.iter().all(|v| v.name != "hidden"), "private measure has no source export");
    let cap = a.exports.values.iter().find(|v| v.name == "cap").unwrap().resolved.as_ref().unwrap();
    assert_eq!(cap.measures[0].identity, "A.hidden");
    interfaces
}
fn refresh_hashes(interfaces: &mut [Iface]) {
    for _ in 0..=interfaces.len() {
        let hashes: std::collections::BTreeMap<_, _> = interfaces.iter().map(|i| (i.module.clone(), i.iface_hash())).collect();
        let mut changed = false;
        for iface in interfaces.iter_mut() {
            for reference in &mut iface.imports {
                let current = &hashes[&reference.module];
                if reference.iface_hash != *current { reference.iface_hash = current.clone(); changed = true; }
            }
        }
        if !changed { return }
    }
    panic!("fixture interface hashes did not converge");
}
fn alter_width_contract(interfaces: &mut [Iface], wrong_scheme: bool) {
    use cinch::total::{CheckedGraph, Expr, Sort};
    for iface in interfaces.iter_mut() {
        for value in &mut iface.exports.values {
            if value.symbol == "A.width" {
                if wrong_scheme {
                    let resolved = value.resolved.as_mut().unwrap();
                    resolved.scheme = "(Int -> Int)".into(); resolved.nominals.clear();
                } else if let Some(graph) = &value.total_body {
                    let mut definitions: Vec<_> = graph.definitions().cloned().collect();
                    definitions.iter_mut().find(|d| d.identity == "A.hidden").unwrap().body = Expr::Number { sort: Sort::Nat, decimal: "2".into() };
                    value.total_body = Some(CheckedGraph::check(definitions).unwrap().closure(["A.width"]).unwrap());
                }
            }
            if let Some(resolved) = &mut value.resolved {
                for measure in &mut resolved.measures {
                    if measure.identity == "A.width" {
                        if wrong_scheme { measure.resolved.scheme = "(Int -> Int)".into(); measure.resolved.nominals.clear(); }
                        else {
                            let mut definitions: Vec<_> = measure.total_body.definitions().cloned().collect();
                            definitions.iter_mut().find(|d| d.identity == "A.hidden").unwrap().body = Expr::Number { sort: Sort::Nat, decimal: "2".into() };
                            measure.total_body = CheckedGraph::check(definitions).unwrap().closure(["A.width"]).unwrap();
                        }
                    }
                }
            }
        }
    }
    refresh_hashes(interfaces);
}
fn write_interfaces(dir: &std::path::Path, tag: &str, interfaces: &[Iface]) {
    let folder = dir.join(tag); std::fs::create_dir_all(&folder).unwrap();
    let mut rows = Vec::new();
    for iface in interfaces.iter().filter(|i| i.module != "Main") {
        let path = folder.join(format!("{}.cinchif", iface.module));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
}
#[test]
fn compiled_cinch_checks_actual_imported_measures_and_changed_bodies() {
    let dir = common::project("measure_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.MeasureContracts.cinch")),
    ]);

    let exe = common::build(&dir, "Main.cinch", "measure-contracts").expect("build actual measure consumer");
    for changed in [false, true] {
        write_interfaces(&dir, if changed { "changed" } else { "original" }, &fixture(changed));
        let cases = [
            ("private measure", "answer :: Nat\n  answer = cap 7", if changed { 4 } else { 0 }),
            ("ordinary retained call", "answer :: Nat\n  answer = previous (measure 7)", 0),
            ("same actual argument", "safe :: Nat -> Nat -> Nat\n  safe n m = if measure n <= 2 then same-argument n m else 0", 0),
            ("wrong actual argument", "bad :: Nat -> Nat -> Nat\n  bad n m = if measure m <= 2 then same-argument n m else 0", 4),
            ("checked local body retains its true precondition", "fake :: Nat -> Nat\n  fake n = 1\n  bad :: using (fake n <= 4) => (n :: Nat) -> Nat\n  bad n = n", 0),
            ("local body cannot establish a false result", "fake :: Nat -> Nat\n  fake n = 1\n  bad :: Nat -> refined (out > 4) Nat\n  bad n = fake n", 4),
            ("local body keeps its actual argument", "fake :: Nat -> Nat\n  fake n = n\n  bad :: (n :: Nat) -> (m :: Nat) -> refined (out == fake n) Nat\n  bad n m = fake m", 4),
            ("recursive helper supplies no numeric authority", "fake :: Nat -> Nat\n  fake n = fake n\n  bad :: Nat -> refined (out > 4) Nat\n  bad n = fake n", 4),
            ("private source name remains private", "bad :: Nat\n  bad = A.hidden 7", 4),
            ("real encoding header", "safe :: Bytes -> Nat -> Nat\n  safe buffer cp = if bytes-len buffer >= 4 then utf8-encode-into buffer 0 cp else 0", 0),
        ];
        for (name, body, wanted) in cases {
            if changed && name == "wrong actual argument" { continue; }
            let input = format!("module Main where\n  import Facade (measure, cap, same-argument, previous)\n  import Data.Bytes (bytes-len)\n  import Data.String (utf8-encode-into)\n  {body}\n  main :: Int\n  main = 0\n");
            std::fs::write(dir.join("source.cinch"), &input).unwrap();
            let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
            let report = common::read_output(&dir, "imported-report.txt");
            assert_eq!(output.status.code(), Some(wanted), "{name}, changed={changed}: {input}\n{report}\n{}", String::from_utf8_lossy(&output.stderr));
        }
    }
    for (tag, wrong_scheme, wanted) in [("wrong-full-scheme", true, 4), ("conflicting-private-dependency", false, 90)] {
        let mut interfaces = fixture(false);
        alter_width_contract(&mut interfaces, wrong_scheme);
        for iface in &interfaces { Iface::from_json(&iface.to_json()).expect("individually structurally valid adversarial payload"); }
        write_interfaces(&dir, tag, &interfaces);
        std::fs::write(dir.join("source.cinch"), "module Main where\n  import Facade (measure, cap, previous)\n  answer :: Nat\n  answer = previous (measure (cap 7))\n  main :: Int\n  main = 0\n").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(wanted), "{tag}: {}", common::read_output(&dir, "imported-report.txt"));
    }
    common::cleanup(&dir);
}
