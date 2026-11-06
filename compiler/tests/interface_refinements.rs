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
    assert_eq!(original.resolved, renamed.resolved);
    let mut changed = provider.clone();
    changed.exports.values.iter_mut().find(|value| value.name == "wrap").unwrap().scheme = original.scheme.replace("False", "True");
    assert_ne!(provider.iface_hash(), changed.iface_hash());
    common::cleanup(&dir);
}
