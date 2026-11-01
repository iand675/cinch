//! The Cinch loader consumes the seed's real contracts, preserves their
//! metadata, checks the dependency closure and parses selected schemes.
mod common;

use cinch::interface::{Iface, IfaceRef, ValueExport, ResolvedScheme, NominalContract};
use std::path::Path;

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
    out.push('"');
    out
}

fn array(items: impl IntoIterator<Item = String>) -> String {
    format!("[{}]", items.into_iter().collect::<Vec<_>>().join(","))
}

fn row(dir: &Path, file: &str, iface: &Iface) -> Vec<String> {
    std::fs::write(dir.join(file), iface.to_json()).unwrap();
    vec![iface.module.clone(), iface.iface_hash(), dir.join(file).display().to_string()]
}

fn strings(row: &[String]) -> String { array(row.iter().map(|s| quote(s))) }

#[test]
fn the_loader_checks_emitted_interfaces_and_dependency_identities() {
    let source = common::project("load_source", &[
        ("Dep.cinch", "module Dep where\n  twice :: Int -> Int\n  twice x = x + x\n"),
        ("Lib.cinch", "module Lib where\n  import Dep\n  export answer, identity, bounded, Positive, (<+>), Mapper, reveal, secret-api, kernel-write\n  kernel-write :: forall r. using (io :: Io r) => Bytes ->{IO r} Unit\n  kernel-write bytes = ioWrite bytes\n  answer :: Int\n  answer = Dep.twice 21\n  identity :: forall a. a -> a\n  identity x = x\n  bounded :: using (0 <= x, x < y) => (x :: I64) -> (y :: I64) -> I64\n  bounded x y = x\n  type Positive = refine (x :: I64) with 0 < x\n  infixr 6 <+>\n  (<+>) :: Int -> Int -> Int\n  (<+>) a b = a - b\n  signature Mapper (f :: Type -> Type) where\n    map-item :: forall a b. (a -> b) -> f a -> f b\n  signature Hidden a where\n    reveal :: a -> a\n    private-method :: a -> a\n  signature Secret a where\n    locked :: a -> a\n  secret-api :: forall a. using (secret :: Secret a) => a -> a\n  secret-api x = x\n  instance Secret_Int :: Secret Int where\n    locked x = x\n"),
        ("Facade.cinch", "module Facade where\n  import Lib (secret-api as use, reveal as show-one)\n  export use, show-one\n"),
        ("Main.cinch", "module Main where\n  import Lib\n  import Facade\n  main :: Int\n  main = answer\n"),
    ]);
    common::check(&source, "Main.cinch").expect("compile the contract fixture");
    let dir = common::project("load", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Load.cinch")),
    ]);
    let mut ifaces = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            ifaces.push(Iface::from_json(&std::fs::read_to_string(path).unwrap()).unwrap());
        }
    }
    ifaces.sort_by(|a, b| a.module.cmp(&b.module));
    let full: Vec<_> = ifaces.iter().map(|i| row(&dir, &format!("{}.cinchif", i.module), i)).collect();
    let lib = ifaces.iter().find(|i| i.module == "Lib").unwrap();
    let original = row(&dir, "original.cinchif", lib);
    let mut cases = vec![(original.clone(), String::new())];
    let nat = ifaces.iter().find(|i| i.module == "Data.Nat").unwrap();
    cases.push((row(&dir, "nat-proof-contracts.cinchif", nat), String::new()));
    let mut erased_proof = nat.clone();
    let unrestricted = lib.exports.instances.iter().find(|i| i.id == "Lib.Secret_Int").unwrap().resolved.clone();
    erased_proof.exports.instances.iter_mut().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap().resolved = unrestricted;
    cases.push((row(&dir, "erased-method-proof.cinchif", &erased_proof), "malformed interface contract envelope".into()));
    let empty_module = Iface::new("", "");
    cases.push((row(&dir, "empty-module.cinchif", &empty_module), "malformed interface contract envelope".into()));
    let mut duplicate_value = lib.clone();
    duplicate_value.exports.values.push(duplicate_value.exports.values[0].clone());
    cases.push((row(&dir, "duplicate-value.cinchif", &duplicate_value), "malformed interface contract envelope".into()));
    let mut bad_slot = lib.clone();
    bad_slot.exports.signatures[0].methods[0].slot = 7;
    cases.push((row(&dir, "bad-method-slot.cinchif", &bad_slot), "malformed interface contract envelope".into()));
    let mut bad_precedence = lib.clone();
    bad_precedence.exports.fixities[0].prec = 10;
    cases.push((row(&dir, "bad-precedence.cinchif", &bad_precedence), "malformed interface contract envelope".into()));
    let mut bad_hash = original.clone();
    bad_hash[1] = "0".into();
    cases.push((bad_hash, "interface contract identity mismatch".into()));
    let mut bad_name = original.clone();
    bad_name[0] = "Wrong".into();
    cases.push((bad_name, "interface module name mismatch".into()));
    for (name, error, changed) in [
        ("schema", "unsupported interface schema", lib.to_json().replace(&format!("\"schema\":{}", lib.schema), "\"schema\":99")),
        ("abi", "interface runtime ABI mismatch", lib.to_json().replace(&format!("\"runtime_abi\":{}", lib.runtime_abi), "\"runtime_abi\":99")),
        ("shape", "malformed interface contract envelope", lib.to_json().replace("\"assoc\":\"right\"", "\"assoc\":\"up\"")),
        ("json", "interface is not valid JSON", "not JSON".into()),
    ] {
        let path = dir.join(format!("bad-{name}.cinchif"));
        std::fs::write(&path, changed).unwrap();
        cases.push((vec!["Lib".into(), lib.iface_hash(), path.display().to_string()], error.into()));
    }
    // Valid JSON with a malformed required table, rather than broken JSON.
    let path = dir.join("bad-table.cinchif");
    let malformed = lib.to_json().replace("\"imports\":[", "\"imports_bad\":[");
    std::fs::write(&path, malformed).unwrap();
    cases.push((vec!["Lib".into(), lib.iface_hash(), path.display().to_string()], "malformed interface contract envelope".into()));
    let mut stale = lib.clone();
    stale.imports.iter_mut().find(|r| r.module == "Dep").unwrap().iface_hash = "0".into();
    let stale_row = row(&dir, "stale.cinchif", &stale);
    // Keep exactly the intended bad edge. Facade and Main refer to the
    // original Lib hash and would otherwise introduce an earlier failure.
    let stale_set: Vec<_> = full.iter().filter(|r| r[0] != "Facade" && r[0] != "Main")
        .map(|r| if r[0] == "Lib" { stale_row.clone() } else { r.clone() }).collect();
    let missing: Vec<_> = full.iter().filter(|r| r[0] != "Dep").cloned().collect();
    let mut duplicate = full.clone();
    duplicate.push(original.clone());
    let mut cycle_a = Iface::new("CycleA", "");
    let mut cycle_b = Iface::new("CycleB", "");
    cycle_a.imports.push(IfaceRef { module: "CycleB".into(), iface_hash: "0".into() });
    cycle_b.imports.push(IfaceRef { module: "CycleA".into(), iface_hash: "0".into() });
    let cycle = vec![row(&dir, "cycle-a.cinchif", &cycle_a), row(&dir, "cycle-b.cinchif", &cycle_b)];
    let nominal_value = |symbol: &str, identity: &str, constructors: Vec<String>| ValueExport {
        name: symbol.into(), symbol: format!("Origin.{symbol}"), scheme: "Hidden".into(), arity: 0,
        resolved: Some(ResolvedScheme { scheme: identity.into(), nominals: vec![NominalContract { refinement: None,
            identity: identity.into(), kind: "data".into(), parameters: vec![], constructors: Some(constructors),
        }] }),
    };
    let mut outside = Iface::new("Outside", "");
    outside.exports.values.push(nominal_value("value", "Missing.Token", vec![]));
    let outside = vec![row(&dir, "outside.cinchif", &outside)];
    let mut conflict = Iface::new("Origin", "");
    conflict.exports.values.push(nominal_value("first", "Origin.Hidden", vec![]));
    conflict.exports.values.push(nominal_value("second", "Origin.Hidden", vec!["Origin.Ctor".into()]));
    let conflict = vec![row(&dir, "conflict.cinchif", &conflict)];
    let facade = ifaces.iter().find(|i| i.module == "Facade").unwrap();
    let mut overridden = facade.clone();
    overridden.exports.signatures.iter_mut().find(|s| s.identity == "Lib.Hidden").unwrap()
        .methods[0].scheme.push_str(" changed");
    let overridden_row = row(&dir, "overridden-class.cinchif", &overridden);
    let override_set: Vec<_> = full.iter().filter(|r| r[0] != "Main").map(|r|
        if r[0] == "Facade" { overridden_row.clone() } else { r.clone() }).collect();
    let mut wrong_parameters = facade.clone();
    wrong_parameters.exports.values.iter_mut().find(|v| v.name == "use").unwrap()
        .resolved.as_mut().unwrap().nominals.iter_mut().find(|n| n.identity == "Lib.Secret").unwrap()
        .parameters[0].kind = "Type -> Type".into();
    let wrong_parameters_row = row(&dir, "wrong-class-parameters.cinchif", &wrong_parameters);
    let wrong_parameters_set: Vec<_> = full.iter().filter(|r| r[0] != "Main").map(|r|
        if r[0] == "Facade" { wrong_parameters_row.clone() } else { r.clone() }).collect();
    let sets = [
        (full, ""),
        (missing, "Lib: missing interface dependency: Dep"),
        (stale_set, "Lib: stale interface dependency: Dep"),
        (duplicate, "duplicate interface module: Lib"),
        (cycle, "cyclic interface dependencies involving CycleA"),
        (outside, "Outside: nominal identity is outside the checked dependency closure: Missing.Token"),
        (conflict, "Origin: conflicting defining nominal contracts: Origin.Hidden"),
        (override_set, "conflicting defining signature contract: Lib.Hidden"),
        (wrong_parameters_set, "Facade: signature identity has no matching defining contract: Lib.Secret"),
    ];
    let case_json = array(cases.iter().map(|(r, expected)| { let mut r = r.clone(); r.push(expected.clone()); strings(&r) }));
    let set_json = array(sets.iter().map(|(rows, expected)| array([array(rows.iter().map(|r| strings(r))), quote(expected)])));
    std::fs::write(dir.join("load-cases.json"), format!("{{\"cases\":{case_json},\"sets\":{set_json}}}")).unwrap();
    let exe = common::build(&dir, "Main.cinch", "loader").expect("build the Cinch loader runner");
    let out = std::process::Command::new(exe).current_dir(&dir).output().expect("run loader");
    assert_eq!(out.status.code(), Some(0), "loader case failed; project: {}\n{}\n{}", dir.display(), String::from_utf8_lossy(&out.stderr), common::read_output(&dir, "load-detail.txt"));
    assert_eq!(common::read_output(&dir, "load-report.txt"), "0");
    common::cleanup(&source);
    common::cleanup(&dir);
}
