//! Versioned interface bodies retain actual defining identity and dependencies.
use cinch::interface::{Iface, SCHEMA_VERSION};
use cinch::total::{CheckedGraph, Definition, Expr, Sort};

#[test]
fn exported_bodies_are_closed_versioned_contracts_for_the_defining_value() {
    let source = r#"module A where
  import Algebra.Arithmetic (InRange)
  export width, loop
  width :: Nat -> InRange 1 4 Nat
  width cp = if cp < 128 then 1 else 4
  loop :: Nat -> Nat
  loop cp = loop cp
module Facade where
  import A (width as measure)
  export measure
module Main where
  import Facade
  main :: Int
  main = 0
"#;
    let (_, interfaces) = cinch::driver::check_sources_ifaces(source, "body-interfaces.cinch").unwrap();
    let a = interfaces.iter().find(|i| i.module == "A").unwrap();
    assert_eq!(a.schema, SCHEMA_VERSION);
    let width = a.exports.values.iter().find(|v| v.name == "width").unwrap();
    let body = width.total_body.as_ref().expect("actual closed width body is a contract");
    assert_eq!(body.get("A.width").unwrap().parameters, [Sort::Nat]);
    assert_eq!(body.get("A.width").unwrap().result, Sort::Nat);
    assert!(body.get("Main.main").is_none(), "unrelated compiler/user bodies do not enter the contract");
    assert!(a.exports.values.iter().find(|v| v.name == "loop").unwrap().total_body.is_none(), "a pure-looking recursive scheme supplies no totality authority");
    let facade = interfaces.iter().find(|i| i.module == "Facade").unwrap();
    let measure = facade.exports.values.iter().find(|v| v.name == "measure").unwrap();
    assert_eq!(measure.symbol, "A.width");
    assert_eq!(measure.total_body.as_ref(), Some(body));
    assert_eq!(Iface::from_json(&a.to_json()).unwrap(), *a);
    let (_, changed) = cinch::driver::check_sources_ifaces(&source.replace("then 1 else 4", "then 2 else 4"), "body-interfaces.cinch").unwrap();
    let changed = changed.iter().find(|i| i.module == "A").unwrap();
    assert_ne!(a.iface_hash(), changed.iface_hash(), "actual body changes invalidate semantic consumers despite identical value schemes");
    let native = cinch::driver::check_sources("module Main where\n  main :: Int\n  main = 0\n", "real-measure-export.cinch").unwrap();
    assert!(native.total_bodies.get("Data.String.utf8-encode-width").is_some());
    let actual = interfaces.iter().find(|i| i.module == "Data.String").unwrap().exports.values.iter().find(|v| v.name == "utf8-encode-width").unwrap();
    assert!(actual.total_body.is_some(), "the actual remaining header's measure carries checked semantics");
    let into = interfaces.iter().find(|i| i.module == "Data.String").unwrap().exports.values.iter().find(|v| v.name == "utf8-encode-into").unwrap();
    let resolved = into.resolved.as_ref().expect("actual remaining dependent measure header is portable");
    assert_eq!(resolved.measures.len(), 1);
    assert_eq!(resolved.measures[0].identity, "Data.String.utf8-encode-width");
    assert_eq!(&resolved.measures[0].total_body, actual.total_body.as_ref().unwrap());
    assert!(resolved.measures[0].resolved.measures.is_empty());
    let string = interfaces.iter().find(|i| i.module == "Data.String").unwrap();
    let decoded = Iface::from_json(&string.to_json()).unwrap();
    assert_eq!(decoded.to_json(), string.to_json(), "canonical interface roundtrip");
    assert_eq!(decoded.exports.values.iter().find(|v| v.name == "utf8-encode-into").unwrap().resolved.as_ref(), Some(resolved));


    let mut missing = a.clone();
    missing.schema = SCHEMA_VERSION - 1;
    assert!(Iface::from_json(&missing.to_json()).is_err(), "old readers must not ignore new semantic fields");
    let mut unrelated = a.clone();
    let extra = Definition { identity: "unrelated".into(), parameters: vec![], result: Sort::Nat, body: Expr::Number { sort: Sort::Nat, decimal: "0".into() } };
    let mut definitions = body.definitions().cloned().collect::<Vec<_>>(); definitions.push(extra);
    unrelated.exports.values.iter_mut().find(|v| v.name == "width").unwrap().total_body = Some(CheckedGraph::check(definitions).unwrap());
    assert!(Iface::from_json(&unrelated.to_json()).is_err(), "closed root payloads may not smuggle unrelated definitions");
    let mut wrong_root = a.clone();
    wrong_root.exports.values.iter_mut().find(|v| v.name == "width").unwrap().symbol = "A.different".into();
    assert!(Iface::from_json(&wrong_root.to_json()).is_err());
    assert!(Iface::from_json(&a.to_json().replace("\"NatCmp\"", "\"AssumeClassSlot\"")).is_err());
}
