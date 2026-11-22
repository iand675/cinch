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
