//! The Cinch loader consumes Rust-produced contracts, preserves their
//! metadata, checks the dependency closure and parses selected schemes.
mod common;

use cinch::interface::{Iface, IfaceRef, ValueExport, ResolvedScheme, NominalContract};
use cinch::total::{CheckedGraph, Definition, Expr, Sort};
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
    common::check_library_fixture_native(&source, "Main.cinch").expect("compile the contract fixture");
    let dir = common::project("load", &[
        ("Bootstrap.Ast.cinch", include_str!("../../compiler/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../compiler/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../compiler/Bootstrap.Parse.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../compiler/Bootstrap.Load.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.Load.cinch")),
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
    let mut no_body = nat.clone();
    let subtraction = no_body.exports.instances.iter_mut().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap();
    let method = &mut subtraction.method_contracts[0];
    method.checked.body = None;
    let implementation = format!("\"implementation\":{{\"identity\":{}", quote(&method.checked.implementation));
    let display = format!("{{\"name\":{},\"scheme\":{},\"slot\":{}}}",
                          quote(&method.name), quote(&method.scheme), method.slot);
    let no_body_json = no_body.to_json();
    assert!(Iface::from_json(&no_body_json).is_ok(), "omit an unavailable body");
    cases.push((row(&dir, "no-guarded-body.cinchif", &no_body), String::new()));

    no_body.exports.instances.iter_mut().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap().method_contracts.clear();
    let empty = no_body.to_json();
    let contract = "\"id\":\"Data.Nat.Sub_Nat\",\"implicit\":true,\"method_contracts\":[]";
    let (prefix, _) = contract.rsplit_once("[]").unwrap();
    let mut malformed = vec![
        ("empty-method-bodies", empty.replace(contract, &format!("{contract},\"method_bodies\":[]")), &empty),
        ("display-only-method", empty.replace(contract, &format!("{prefix}[{display}]")), &empty),
        ("null-guarded-body", no_body_json.replace(&implementation, &implementation.replace("{", "{\"body\":null,")), &no_body_json),
    ];
    let null_checked = display.replacen('{', "{\"checked\":null,", 1);
    malformed.push(("null-checked-method", empty.replace(contract, &format!("{prefix}[{null_checked}]")), &empty));
    for (name, invalid, original_json) in malformed {
        assert_ne!(&invalid, original_json, "{name} fixture must be mutated");
        assert!(Iface::from_json(&invalid).is_err(), "Rust accepted {name}");
        let path = dir.join(format!("{name}.cinchif"));
        std::fs::write(&path, invalid).unwrap();
        cases.push((vec![nat.module.clone(), nat.iface_hash(), path.display().to_string()],
                    "malformed interface contract envelope".into()));
    }
    let mut erased_proof = nat.clone();
    let unrestricted = lib.exports.instances.iter().find(|i| i.id == "Lib.Secret_Int").unwrap().resolved.clone();
    erased_proof.exports.instances.iter_mut().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap().resolved = unrestricted;
    cases.push((row(&dir, "erased-method-proof.cinchif", &erased_proof), "malformed interface contract envelope".into()));
    let empty_module = Iface::new("", "");
    cases.push((row(&dir, "empty-module.cinchif", &empty_module), "malformed interface contract envelope".into()));
    let mut duplicate_value = lib.clone();
    duplicate_value.exports.values.push(duplicate_value.exports.values[0].clone());
    cases.push((row(&dir, "duplicate-value.cinchif", &duplicate_value), "malformed interface contract envelope".into()));
    // Check the raw table order as well as the producer's sorted output.
    let mut unordered: serde_json::Value = serde_json::from_str(&lib.to_json()).unwrap();
    unordered["exports"]["values"].as_array_mut().unwrap().reverse();
    let path = dir.join("unordered-values.cinchif");
    std::fs::write(&path, unordered.to_string()).unwrap();
    unordered["compiler"] = "".into();
    unordered["source_hash"] = "".into();
    unordered["units"] = serde_json::json!([]);
    let hash = cinch::interface::content_hash_bytes(unordered.to_string().as_bytes());
    cases.push((vec![lib.module.clone(), hash, path.display().to_string()], String::new()));

    let values = unordered["exports"]["values"].as_array_mut().unwrap();
    assert!(values.len() > 2);
    values.push(values[0].clone());
    let path = dir.join("unordered-duplicate-value.cinchif");
    std::fs::write(&path, unordered.to_string()).unwrap();
    let hash = cinch::interface::content_hash_bytes(unordered.to_string().as_bytes());
    cases.push((vec![lib.module.clone(), hash, path.display().to_string()],
                "malformed interface contract envelope".into()));
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
    let old_schema = lib.to_json().replace(&format!("\"schema\":{}", lib.schema), "\"schema\":9");
    assert!(Iface::from_json(&old_schema).unwrap_err().to_string().contains("interface schema 9 is not supported"));
    for (name, error, changed) in [
        ("previous-schema", "unsupported interface schema", old_schema),
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
    let nominal_value = |symbol: &str, identity: &str, constructors: Vec<String>| ValueExport { total_body: None, keyed_ordering: None,
        name: symbol.into(), symbol: format!("Origin.{symbol}"), scheme: "Hidden".into(), arity: 0,
        resolved: Some(ResolvedScheme { measures: vec![], scheme: identity.into(), nominals: vec![NominalContract { usage: None, refinement: None,
            identity: identity.into(), kind: "data".into(), parameters: vec![], constructors: Some(constructors),
        }] }),
    };
    let mut outside = Iface::new("Outside", "");
    outside.exports.values.push(nominal_value("value", "Missing.Token", vec![]));
    let outside = vec![row(&dir, "outside.cinchif", &outside)];

    // The root can mention a nominal through either arm of a reversed diamond,
    // but a nominal in an unrelated interface is still outside its closure.
    let left_type = Iface::new("LeftType", "");
    let right_type = Iface::new("RightType", "");
    let mut left = Iface::new("Left", "");
    left.imports.push(IfaceRef { module: left_type.module.clone(), iface_hash: left_type.iface_hash() });
    let mut right = Iface::new("Right", "");
    right.imports = vec![
        IfaceRef { module: left_type.module.clone(), iface_hash: left_type.iface_hash() },
        IfaceRef { module: right_type.module.clone(), iface_hash: right_type.iface_hash() },
    ];
    let mut root = Iface::new("Origin", "");
    root.imports = vec![
        IfaceRef { module: left.module.clone(), iface_hash: left.iface_hash() },
        IfaceRef { module: right.module.clone(), iface_hash: right.iface_hash() },
    ];
    root.exports.values = vec![
        nominal_value("first", "LeftType.Token", vec![]),
        nominal_value("second", "RightType.Token", vec![]),
    ];
    let diamond = [&root, &left, &right, &left_type, &right_type].into_iter()
        .map(|i| row(&dir, &format!("diamond-{}.cinchif", i.module), i))
        .collect::<Vec<_>>();
    right.imports.pop();
    root.imports[1].iface_hash = right.iface_hash();
    let disconnected = vec![
        row(&dir, "disconnected-origin.cinchif", &root),
        row(&dir, "disconnected-right.cinchif", &right),
        diamond[1].clone(), diamond[3].clone(), diamond[4].clone(),
    ];

    let mut conflict = Iface::new("Origin", "");
    conflict.exports.values.push(nominal_value("first", "Origin.Hidden", vec![]));
    conflict.exports.values.push(nominal_value("second", "Origin.Hidden", vec!["Origin.Ctor".into()]));
    let conflict = vec![row(&dir, "conflict.cinchif", &conflict)];
    let facade = ifaces.iter().find(|i| i.module == "Facade").unwrap();
    let mut overridden = facade.clone();
    let method = &mut overridden.exports.signatures.iter_mut()
        .find(|s| s.identity == "Lib.Hidden").unwrap().methods[0];
    let changed = method.scheme.replace("(a -> a)", "(a -> Int)");
    assert_ne!(changed, method.scheme);
    method.scheme = changed.clone();
    method.resolved.as_mut().unwrap().scheme = changed;
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
    let mut old_abi = lib.clone();
    old_abi.runtime_abi = 5;
    let old_abi_row = row(&dir, "old-abi.cinchif", &old_abi);
    cases.push((old_abi_row.clone(), "interface runtime ABI mismatch".into()));
    let mixed_abi_set: Vec<_> = full.iter().map(|r|
        if r[0] == "Lib" { old_abi_row.clone() } else { r.clone() }).collect();
    // The first client can admit a private body from a dependency. Seeing
    // that exact body earlier must not grant it to an unrelated client.
    let definitions = |module: &str, value: &str| vec![
        Definition {
            identity: "BodyOwner.private".into(), parameters: Vec::new(), result: Sort::Int,
            body: Expr::Number { sort: Sort::Int, decimal: value.into() },
        },
        Definition {
            identity: format!("{module}.read"), parameters: Vec::new(), result: Sort::Int,
            body: Expr::Call { target: "BodyOwner.private".into(), arguments: Vec::new() },
        },
    ];
    let reader = |module: &str, dependency: Option<&Iface>, literal: &str| {
        let mut iface = Iface::new(module, "");
        if let Some(dependency) = dependency {
            iface.imports.push(IfaceRef { module: dependency.module.clone(), iface_hash: dependency.iface_hash() });
        }
        iface.exports.values.push(ValueExport {
            name: "read".into(), symbol: format!("{module}.read"), scheme: "Int".into(), arity: 0,
            resolved: Some(ResolvedScheme { scheme: "Int".into(), nominals: Vec::new(), measures: Vec::new() }),
            total_body: Some(CheckedGraph::check(definitions(module, literal)).unwrap()),
            keyed_ordering: None,
        });
        iface
    };
    let body_owner = Iface::new("BodyOwner", "");
    let first = reader("FirstReader", Some(&body_owner), "0");
    let both = reader("SecondReader", Some(&body_owner), "0");
    let no_body_import = reader("SecondReader", None, "0");
    let changed_definition = reader("SecondReader", Some(&body_owner), "1");
    let body_owner_row = row(&dir, "body-owner.cinchif", &body_owner);
    let first_reader_row = row(&dir, "first-reader.cinchif", &first);
    let body_set = |file: &str, iface: &Iface| vec![
        body_owner_row.clone(), first_reader_row.clone(), row(&dir, file, iface),
    ];

    let mut unrelated = first.clone();
    let mut extra = definitions("FirstReader", "0");
    extra.push(Definition {
        identity: "FirstReader.unrelated".into(), parameters: Vec::new(), result: Sort::Int,
        body: Expr::Number { sort: Sort::Int, decimal: "9".into() },
    });
    unrelated.exports.values[0].total_body = Some(CheckedGraph::check(extra).unwrap());
    cases.push((row(&dir, "unrelated-body.cinchif", &unrelated), "malformed interface contract envelope".into()));

    let sets = [
        (full, ""),
        (mixed_abi_set, "Lib: interface runtime ABI mismatch"),
        (missing, "Lib: missing interface dependency: Dep"),
        (stale_set, "Lib: stale interface dependency: Dep"),
        (duplicate, "duplicate interface module: Lib"),
        (cycle, "cyclic interface dependencies involving CycleA"),
        (outside, "Outside: nominal identity is outside the checked dependency closure: Missing.Token"),
        (diamond, ""),
        (disconnected, "Origin: nominal identity is outside the checked dependency closure: RightType.Token"),
        (conflict, "Origin: conflicting defining nominal contracts: Origin.Hidden"),
        (override_set, "conflicting defining signature contract: Lib.Hidden"),
        (wrong_parameters_set, "Facade: signature identity has no matching defining contract: Lib.Secret"),
        (body_set("second-reader.cinchif", &both), ""),
        (body_set("missing-body-edge.cinchif", &no_body_import), "SecondReader: total body dependency is outside the checked module closure: BodyOwner.private"),
        (body_set("conflicting-shared-body.cinchif", &changed_definition), "SecondReader: conflicting actual total body dependency: BodyOwner.private"),
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


#[test]
fn body_owners_keep_longest_dot_boundary_prefixes_for_utf8_identities() {
    let names = ["A", "A.B", "A.B.C", "AB", "A.BC", "Unicode.λ", "Unicode.λ.Nested", "Unicode.λNested", "Ö", "Ö.深", "Ö.深.Nested"];
    let interfaces = names.iter().map(|name| {
        let iface = Iface::new(*name, "owner-prefix-fixture");
        format!("({}, {}, string-to-bytes {})", quote(name), quote(&iface.iface_hash()), quote(&iface.to_json()))
    }).collect::<Vec<_>>().join(", ");
    let cases = [
        ("A.B.C.value", "A.B.C"), ("A.B.value", "A.B"), ("AB.value", "AB"),
        ("A.BC.value", "A.BC"), ("A.Bogus.value", "A"), ("A.B", "A"),
        ("A.B.", "A.B"), ("A", ""), ("Missing.value", ""),
        ("Unicode.λ.Nested.value", "Unicode.λ.Nested"),
        ("Unicode.λNested.value", "Unicode.λNested"),
        ("Unicode.λ.Nest.value", "Unicode.λ"),
        ("Ö.深.Nested.value", "Ö.深.Nested"), ("Ö.深x.value", "Ö"),
    ];
    let checks = cases.iter().map(|(identity, owner)| format!(
        "same (body-node-owner index (string-concat {} suffix)) {} && same (body-node-owner reversed (string-concat {} suffix)) {}",
        quote(identity), quote(owner), quote(identity), quote(owner),
    )).collect::<Vec<_>>().join(" &&\n          ");
    let program = format!(r#"module Main where
  import Bootstrap.Load
  import Data.Path (path-from-string)
  import Effect.Fs (fs-read)
  same :: String -> String -> Bool
  same a b = string-compare a b == 0
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in case fs-read (path-from-string "nonce.txt") of
    Err error -> 90
    Ok input -> case load-interface-set [{interfaces}] of
      Err error -> 91
      Ok loaded -> let index = index-interfaces loaded in
        let reversed = index-interfaces (reverse loaded) in
        let suffix = string-repeat (bytes-len input) "λ" in
        if {checks} then 0 else 1
"#);
    // Expose the private lookup and the index that feeds it only in this
    // test's owned module copy. Production exports and admission paths
    // retain their existing boundary.
    let loader = include_str!("../../compiler/Bootstrap.Load.cinch").replacen(
        "  export ImportedValue",
        "  export body-node-owner\n  export index-interfaces\n  export ImportedValue",
        1,
    );
    let dir = common::project("owner_prefixes", &[
        ("Bootstrap.Ast.cinch", include_str!("../../compiler/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../compiler/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../compiler/Bootstrap.Parse.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Bootstrap.Load.cinch", &loader), ("Main.cinch", &program),
    ]);
    let exe = common::build(&dir, "Main.cinch", "owner-prefixes").expect("build owner-prefix checks");
    for size in [0, 1, 17, 129, 257] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "owner prefixes at {size}: {}", String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}

#[test]
fn the_body_loader_prunes_stale_importers_and_keeps_dependency_order() {
    let root = Iface::new("Root", "");
    let mut stale = Iface::new("Stale", "");
    stale.imports.push(IfaceRef { module: root.module.clone(), iface_hash: "0000000000000000".into() });
    let mut middle = Iface::new("Middle", "");
    middle.imports.push(IfaceRef { module: stale.module.clone(), iface_hash: stale.iface_hash() });
    let mut leaf = Iface::new("Leaf", "");
    leaf.imports.push(IfaceRef { module: middle.module.clone(), iface_hash: middle.iface_hash() });
    let independent = Iface::new("Independent", "");
    let mut child = Iface::new("Child", "");
    child.imports.push(IfaceRef { module: root.module.clone(), iface_hash: root.iface_hash() });
    let documents = |rows: &[&Iface]| -> String {
        rows.iter().map(|iface|
            format!("({}, {}, {})", quote(&iface.module), quote(&iface.iface_hash()), quote(&iface.to_json()))
        ).collect::<Vec<_>>().join(", ")
    };
    let stale = documents(&[&leaf, &middle, &independent, &root, &stale]);
    let ordered = documents(&[&child, &root, &independent]);
    let main = format!(r#"module Main where
  import Bootstrap.Interface (iface-parse)
  import Bootstrap.Load (load-checked-body-interface-set-from-documents, interface-set-members, loaded-module)
  import Data.Json (Json)
  documents :: List (String, String, String) -> Option (List (String, String, Json))
  documents rows = case rows of
    Nil -> Some Nil
    Cons (name, hash, source) rest -> case (iface-parse (string-to-bytes source), documents rest) of
      (Some doc, Some more) -> Some (Cons (name, hash, doc) more)
      other -> None
  same-modules :: List String -> List String -> Bool
  same-modules actual expected = case (actual, expected) of
    (Nil, Nil) -> True
    (Cons a rest, Cons b more) -> string-compare a b == 0 && same-modules rest more
    other -> False
  admits :: List (String, String, String) -> List String -> Bool
  admits rows expected = case documents rows of
    None -> False
    Some docs -> case load-checked-body-interface-set-from-documents docs of
      Err error -> False
      Ok checked -> same-modules (map loaded-module (interface-set-members checked)) expected
  main :: Int
  main = if admits [{stale}] ["Independent", "Root"] && admits [{ordered}] ["Root", "Child", "Independent"] then 0 else 1
"#);
    let dir = common::project("stale_body_closure", &[
        ("Bootstrap.Ast.cinch", include_str!("../../compiler/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../compiler/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../compiler/Bootstrap.Parse.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../compiler/Bootstrap.Load.cinch")),
        ("Main.cinch", &main),
    ]);
    let exe = common::build(&dir, "Main.cinch", "stale-body-closure").expect("build body closure checks");
    let output = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "body closure check failed in {}", dir.display());
    common::cleanup(&dir);
}
