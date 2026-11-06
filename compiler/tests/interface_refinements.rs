//! The contracts stage 2 will import must retain the source's proof facts.
mod common;

use cinch::interface::{Iface, SCHEMA_VERSION};

#[test]
fn interfaces_publish_preconditions_and_named_refinement_definitions() {
    let dir = common::project("interface_refinements", &[
        ("Contract.cinch", r#"module Contract where
  export bounded, Positive
  type Positive = refine (x :: I64) with 0 < x
  bounded :: using (0 <= x, x < y) => (x :: I64) -> (y :: I64) -> I64
  bounded x y = x
"#),
        ("Main.cinch", "module Main where\n  import Contract\n  main :: Int\n  main = 0\n"),
    ]);
    common::build(&dir, "Main.cinch", "program").expect("build contract project");
    let path = dir.join("build/Contract.cinchif");
    let iface = Iface::from_json(&std::fs::read_to_string(&path).expect("interface")).expect("read interface");
    assert_eq!(iface.schema, SCHEMA_VERSION);
    let value = iface.exports.values.iter().find(|v| v.name == "bounded").expect("bounded export");
    assert!(value.scheme.contains("using ("), "{}", value.scheme);
    assert!(value.scheme.contains("0 <= arg0"), "{}", value.scheme);
    assert!(value.scheme.contains("arg0 < arg1"), "{}", value.scheme);
    assert!(!value.scheme.contains('$'), "{}", value.scheme);
    let ty = iface.exports.types.iter().find(|t| t.name == "Positive").expect("Positive export");
    assert_eq!(ty.kind, "refinement");
    let refinement = ty.refinement.as_ref().expect("refinement definition");
    assert_eq!(refinement.binder, "x");
    assert_eq!(refinement.carrier, "I64");
    assert_eq!(refinement.predicate, "0 < x");

    let mut changed = iface.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "bounded").unwrap().scheme =
        value.scheme.replace("arg0 < arg1", "arg0 <= arg1");
    assert_ne!(iface.iface_hash(), changed.iface_hash(), "preconditions affect the contract identity");
    let mut changed = iface.clone();
    changed.exports.types.iter_mut().find(|t| t.name == "Positive").unwrap()
        .refinement.as_mut().unwrap().predicate = "0 <= x".into();
    assert_ne!(iface.iface_hash(), changed.iface_hash(), "named predicates affect the contract identity");
    let previous = iface.to_json().replace(&format!("\"schema\":{SCHEMA_VERSION}"), "\"schema\":1");
    assert!(Iface::from_json(&previous).is_err(), "old incomplete contracts must be rebuilt");
    common::cleanup(&dir);
}


#[test]
fn logical_literal_proof_contracts_round_trip_without_becoming_boolean_values() {
    let dir = common::project("logical_literal_contracts", &[
        ("Evidence.cinch", r#"module Evidence where
  export wrap, reject
  wrap :: (h :: False) -> Dec False
  wrap h = Yes h
  reject :: (h :: Not False) -> Dec False
  reject h = No h
"#),
        ("Facade.cinch", "module Facade where\n  import Evidence (wrap as evidence, reject as refute)\n  export evidence, refute\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = case decide False of Yes _ -> 1; No h -> case refute h of No _ -> 0; Yes _ -> 2\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit logical literal proof contracts");
    let read = |name| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let provider = read("Evidence");
    let facade = read("Facade");
    for iface in [&provider, &facade] {
        assert_eq!(Iface::from_json(&iface.to_json()).unwrap(), *iface);
        assert!(iface.exports.values.iter().all(|value| value.scheme.contains("False")));
    }
    let original = provider.exports.values.iter().find(|value| value.name == "wrap").unwrap();
    let renamed = facade.exports.values.iter().find(|value| value.name == "evidence").unwrap();
    let contract = original.resolved.as_ref().expect("provider proof contract must be portable");
    assert!(contract.scheme.contains("False") && contract.scheme.contains("Dec"));
    assert!(contract.nominals.is_empty(), "kernel Dec must not invent a module owner");
    assert_eq!(Some(contract), renamed.resolved.as_ref());
    for iface in [&provider, &facade] {
        assert!(iface.exports.values.iter().all(|value| value.resolved.is_some()),
            "every published literal proof contract must have resolved metadata");
    }
    let mut changed = provider.clone();
    changed.exports.values.iter_mut().find(|value| value.name == "wrap").unwrap().scheme = original.scheme.replace("False", "True");
    assert_ne!(provider.iface_hash(), changed.iface_hash());
    common::cleanup(&dir);
}

#[test]
fn portable_proof_arrows_keep_kinds_effects_erasure_and_type_owners() {
    let source = r#"module Evidence where
  generic :: forall (p :: Prop). (h :: p) -> Dec p
  generic h = Yes h
  refute :: forall (p :: Prop). (h :: Not p) -> Dec p
  refute h = No h
  literal :: False -> Int
  literal h = 0
  compound :: (True && Not False) -> Int
  compound h = 0
  effectful :: forall {r}. using (io :: Io r) => False ->{IO r} Int
  effectful h = 0
module Shadow where
  data Dec where
    ShadowDec :: Dec
  keep :: Shadow.Dec -> Shadow.Dec
  keep value = value
module Main where
  main :: Int
  main = 0
"#;
    let (program, interfaces) = cinch::driver::check_sources_ifaces(source, "portable-proof-contracts.cinch").unwrap();
    cinch::interface::verify_chain(&interfaces).expect("checked proof contract dependency chain");
    let evidence = interfaces.iter().find(|iface| iface.module == "Evidence").unwrap();
    assert_eq!(Iface::from_json(&evidence.to_json()).unwrap(), *evidence);
    for name in ["generic", "refute", "literal", "compound", "effectful"] {
        let value = evidence.exports.values.iter().find(|v| v.name == name).unwrap();
        let resolved = value.resolved.as_ref().unwrap_or_else(|| panic!("missing {name}: {}", value.scheme));
        assert!(resolved.nominals.is_empty(), "kernel proof syntax must not invent module owners");
        assert!(value.total_body.is_none(), "an erased proof arrow is not a plain numeric function");
        if name == "generic" || name == "refute" {
            assert!(resolved.scheme.contains("p :: Prop") && resolved.scheme.contains("Dec"), "{}", resolved.scheme);
        }
        if name != "effectful" {
            let body = if name == "generic" { "Yes h" } else if name == "refute" { "No h" } else { "0" };
            let readback = format!("module Main where\n  probe :: {}\n  probe h = {body}\n  main :: Int\n  main = 0\n", resolved.scheme);
            cinch::driver::check_sources(&readback, "proof-contract-readback.cinch")
                .unwrap_or_else(|error| panic!("unreadable {name}: {}: {error:?}", resolved.scheme));
        }
        if name == "effectful" {
            assert!(resolved.scheme.contains("r :: Region") && resolved.scheme.contains("io :: Io r")
                && resolved.scheme.contains("->{IO r}"), "{}", resolved.scheme);
        }
        // Interface arity counts source arrows and dictionaries. Actual
        // checked Core omits the erased proof argument's runtime slot.
        let runtime = program.fns.iter().find(|f| f.id.0 == value.symbol).unwrap();
        let dictionaries = if name == "effectful" { 1 } else { 0 };
        assert_eq!(value.arity, 1 + dictionaries);
        assert_eq!(runtime.params.len(), dictionaries as usize,
            "erased proof arguments must not become runtime parameters");
    }
    let shadow = interfaces.iter().find(|iface| iface.module == "Shadow").unwrap();
    let keep = shadow.exports.values.iter().find(|v| v.name == "keep").unwrap().resolved.as_ref().unwrap();
    assert!(keep.scheme.contains("Shadow.Dec"));
    assert_eq!(keep.nominals.iter().map(|n| n.identity.as_str()).collect::<Vec<_>>(), ["Shadow.Dec"]);
}

#[test]
fn literal_preconditions_keep_checked_truth_and_facade_contract_identity() {
    let dir = common::project("literal_precondition_contracts", &[
        ("Conditions.cinch", "module Conditions where\n  export always, never\n  always :: using (True) => Int\n  always = 7\n  never :: using (False) => Int\n  never = 7\n"),
        ("Facade.cinch", "module Facade where\n  import Conditions (always as answer, never as blocked)\n  export answer, blocked\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = answer\n"),
    ]);
    let exe = common::build(&dir, "Main.cinch", "literal-precondition-program").expect("build literal context project");
    let result = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    assert_eq!(result.status.code(), Some(7), "{}", String::from_utf8_lossy(&result.stderr));
    let read = |name| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let provider = read("Conditions"); let facade = read("Facade");
    for (name, alias, predicate) in [("always", "answer", "True"), ("never", "blocked", "False")] {
        let original = provider.exports.values.iter().find(|v| v.name==name).unwrap();
        let renamed = facade.exports.values.iter().find(|v| v.name==alias).unwrap();
        let resolved = original.resolved.as_ref().expect("literal precondition metadata");
        assert!(resolved.scheme.contains("using (") && resolved.scheme.contains(predicate), "{}", resolved.scheme);
        assert_eq!(renamed.resolved.as_ref(), Some(resolved));
        assert_eq!(Iface::from_json(&facade.to_json()).unwrap(), facade);
        let mut changed = provider.clone();
        changed.exports.values.iter_mut().find(|v| v.name==name).unwrap().resolved.as_mut().unwrap().scheme.push_str(" changed");
        assert_ne!(provider.iface_hash(), changed.iface_hash());
    }
    std::fs::write(dir.join("Main.cinch"), "module Main where\n  import Facade\n  main :: Int\n  main = blocked\n").unwrap();
    assert!(common::check(&dir, "Main.cinch").is_err(), "an imported False precondition must not disappear");
    common::cleanup(&dir);
}
