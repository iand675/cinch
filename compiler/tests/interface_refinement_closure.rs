//! Real definition-site closure for private refinements in exported values.
mod common;
use cinch::interface::Iface;

#[test]
fn private_refinement_contracts_survive_value_only_facades() {
    let dir = common::project("private_refinement_closure", &[
        ("A.cinch", "module A where\n  export bounded, widen, positive\n  type Window (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi\n  type Positive = refine (n :: Nat) with n > 0\n  bounded :: Window 0 10 Nat\n  bounded = 1\n  widen :: Window 2 8 Nat -> Window 0 10 Nat\n  widen n = n\n  positive :: Positive\n  positive = 1\n"),
        ("Facade.cinch", "module Facade where\n  import A (bounded as one, widen as wide, positive as pos)\n  export one, wide, pos\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit actual private refinement contracts");
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A");
    let facade = read("Facade");
    assert_eq!(a.schema, 7);
    assert!(a.exports.types.is_empty() && facade.exports.types.is_empty(), "private contract closure must not export source names");
    let get = |iface: &Iface, name: &str| iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone().expect("resolved defining refinement");
    let bounded = get(&a, "bounded");
    assert_eq!(get(&facade, "one"), bounded);
    assert_eq!(get(&facade, "wide"), get(&a, "widen"));
    assert_eq!(bounded.nominals.len(), 1);
    let window = &bounded.nominals[0];
    assert_eq!(window.identity, "A.Window");
    assert_eq!(window.kind, "refinement");
    assert!(window.constructors.is_none());
    assert_eq!(window.parameters.iter().map(|p| (p.name.as_str(), p.kind.as_str())).collect::<Vec<_>>(), [("lo", "Int"), ("hi", "Int"), ("a", "Type")]);
    let template = window.refinement.as_ref().unwrap();
    assert_eq!(template.binder, "x");
    assert_eq!(template.carrier, "a");
    assert!(template.predicate.contains("lo <= x") && template.predicate.contains("x <= hi"));
    let positive = get(&facade, "pos");
    assert_eq!(positive.nominals[0].identity, "A.Positive");
    assert_eq!(positive.nominals[0].refinement.as_ref().unwrap().carrier, "Nat");
    assert_eq!(Iface::from_json(&facade.to_json()).unwrap(), facade);
    let mut changed = facade.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "one").unwrap().resolved.as_mut().unwrap().nominals[0].refinement.as_mut().unwrap().predicate = "lo < x && x < hi".into();
    assert_ne!(changed.iface_hash(), facade.iface_hash(), "private predicate changes invalidate consumers");
    for mutation in 0..4 {
        let mut malformed = facade.clone();
        let row = &mut malformed.exports.values.iter_mut().find(|v| v.name == "one").unwrap().resolved.as_mut().unwrap().nominals[0];
        match mutation {
            0 => row.refinement = None,
            1 => row.constructors = Some(vec![]),
            2 => row.refinement.as_mut().unwrap().binder = "lo".into(),
            _ => row.kind = "opaque".into(),
        }
        assert!(Iface::from_json(&malformed.to_json()).is_err(), "malformed defining template {mutation}");
    }
    common::cleanup(&dir);
}
