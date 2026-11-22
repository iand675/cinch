//! Occurrence identities come from source lookup, including private types
//! and two qualified types with the same final spelling.
mod common;
use cinch::interface::{Iface, ResolvedScheme};

#[test]
fn resolved_schemes_preserve_definition_sites_and_private_contracts() {
    let dir = common::project("interface_nominals", &[
        ("A.cinch", "module A where\n  export Token, Mk, keep, secret-api, Box, Boxed\n  data Token where\n    Mk :: Token\n  data HiddenToken where\n    Hide :: Int -> HiddenToken\n  data Box a where\n    Boxed :: a -> Box a\n  keep :: Token -> Token\n  keep x = x\n  secret-api :: HiddenToken -> HiddenToken\n  secret-api x = x\n"),
        ("B.cinch", "module B where\n  data Token where\n    Mk :: Token\n"),
        ("Facade.cinch", "module Facade where\n  import A (Token as Label, Mk as Make, keep as use, secret-api as opaque-use, Box as Container, Boxed as Pack)\n  export Label, Make, use, opaque-use, Container, Pack\n"),
        ("Main.cinch", "module Main where\n  import A\n  import B as Other\n  import Facade (use as call)\n  pair :: A.Token -> Other.Token -> (A.Token, Other.Token)\n  pair x y = (x, y)\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("check nominal fixture");
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A");
    let facade = read("Facade");
    let main = read("Main");
    let get = |iface: &Iface, name: &str| -> ResolvedScheme {
        iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone().expect("resolved pure scheme")
    };
    assert_eq!(get(&facade, "use"), get(&a, "keep"));
    let both = get(&main, "pair");
    assert_eq!(both.nominals.iter().map(|n| n.identity.as_str()).collect::<Vec<_>>(), ["A.Token", "B.Token"]);
    assert!(both.scheme.contains("A.Token") && both.scheme.contains("B.Token"), "{}", both.scheme);
    let hidden = get(&facade, "opaque-use");
    assert_eq!(hidden.nominals[0].identity, "A.HiddenToken");
    assert_eq!(hidden.nominals[0].constructors.as_deref(), Some(["A.Hide".to_string()].as_slice()));
    assert!(facade.exports.types.iter().all(|t| t.identity != "A.HiddenToken"));
    let container = facade.exports.types.iter().find(|t| t.name == "Container").unwrap();
    let pack = &container.ctors[0];
    assert_eq!(pack.symbol, "A.Boxed");
    let resolved = pack.resolved.as_ref().expect("resolved constructor");
    assert!(resolved.scheme.contains("forall (a :: Type)."), "{}", resolved.scheme);
    assert!(resolved.scheme.contains("A.Box"));
    assert_eq!(resolved.nominals[0].constructors.as_deref(), Some(["A.Boxed".to_string()].as_slice()));
    let mut changed = facade.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "use").unwrap().resolved.as_mut().unwrap().nominals[0].identity = "B.Token".into();
    assert_ne!(changed.iface_hash(), facade.iface_hash());
    assert_eq!(read("Facade").to_json(), facade.to_json());
    let mut malformed = facade.clone();
    let resolved = malformed.exports.values.iter_mut().find(|v| v.name == "use").unwrap().resolved.as_mut().unwrap();
    resolved.nominals.push(resolved.nominals[0].clone());
    assert!(Iface::from_json(&malformed.to_json()).is_err(), "duplicate nominal contracts are malformed");
    let mut malformed = facade.clone();
    malformed.exports.values.iter_mut().find(|v| v.name == "use").unwrap().resolved.as_mut().unwrap().nominals[0].constructors = None;
    assert!(Iface::from_json(&malformed.to_json()).is_err(), "data needs its complete constructor identities");
    assert!(Iface::from_json(&facade.to_json().replace(
        &format!("\"schema\":{}", cinch::interface::SCHEMA_VERSION), "\"schema\":4")).is_err());
    common::cleanup(&dir);
}
