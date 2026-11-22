//! Checked anonymous predicates retain computational argument identity.
mod common;
use cinch::interface::Iface;
const DEFINITION: &str = "module A where\n  import Algebra.Arithmetic (NonZero)\n  export bounded, under, identity, positive, require-positive, below-zero, argument-only, collide, by-positive, fixed\n  bounded :: (limit :: Nat) -> refined (out <= limit) Nat\n  bounded limit = (0 :: Nat)\n  under :: (limit :: Nat) -> refined (limit >= out) Nat\n  under limit = (0 :: Nat)\n  identity :: (x :: Nat) -> refined (out == x) Nat\n  identity x = x\n  positive :: refined NonZero Nat\n  positive = (1 :: Nat)\n  require-positive :: refined (n > 0) Nat -> Nat\n  require-positive n = n\n  below-zero :: refined (n < 0) Int\n  below-zero = (-1 :: Int)\n  argument-only :: (limit :: Nat) -> refined (limit >= 0) Nat\n  argument-only limit = (0 :: Nat)\n  collide :: (cinch-result :: Nat) -> refined (out == cinch-result) Nat\n  collide cinch-result = cinch-result\n  by-positive :: (d :: NonZero Nat) -> refined (r < d) Nat\n  by-positive d = (0 :: Nat)\n  fixed :: refined (n >= 0) I64\n  fixed = (1 :: I64)\n";
const FACADE: &str = "module Facade where\n  import A (bounded as bound, under, identity, positive as pos, require-positive as need, below-zero, argument-only, collide, by-positive, fixed)\n  export bound, under, identity, pos, need, below-zero, argument-only, collide, by-positive, fixed\n";
fn producer() -> std::path::PathBuf {
    let dir = common::project("anonymous_contract_producer", &[
        ("A.cinch", DEFINITION), ("Facade.cinch", FACADE),
        ("Main.cinch", "module Main where\n  import Facade\n  import Data.Bytes\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("check real anonymous refinement definitions");
    dir
}

#[test]
fn checked_anonymous_contracts_survive_facades() {
    let dir = producer();
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A"); let facade = read("Facade");
    let get = |iface: &Iface, name: &str| iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone();
    for (original, alias) in [("bounded", "bound"), ("under", "under"), ("identity", "identity"), ("positive", "pos"), ("require-positive", "need"), ("below-zero", "below-zero"), ("argument-only", "argument-only"), ("collide", "collide"), ("by-positive", "by-positive")] {
        let defining = get(&a, original).expect(original);
        assert_eq!(get(&facade, alias), Some(defining.clone()), "{original}: defining scheme changed through facade");
        assert!(!defining.scheme.contains('$'), "internal binder leaked: {}", defining.scheme);
    }
    let bounded = get(&facade, "bound").unwrap();
    assert!(bounded.scheme.contains("cinch-result <= limit"), "{}", bounded.scheme);
    assert!(get(&facade, "under").unwrap().scheme.contains("limit >= cinch-result"));
    assert!(get(&facade, "pos").unwrap().scheme.contains("NonZero cinch-result"));
    assert!(get(&facade, "collide").unwrap().scheme.contains("cinch-result-r == cinch-result"));
    let argument_only = get(&facade, "argument-only").unwrap();
    assert!(argument_only.scheme.contains("limit >= 0") && !argument_only.scheme.contains("cinch-result"));
    let fixed = get(&a, "fixed").expect("checked machine-value comparisons retain their actual carrier");
    assert!(fixed.scheme.contains("I64") && fixed.scheme.contains(">= 0"), "{}", fixed.scheme);
    let natural = read("Data.Nat");
    let remainder = get(&natural, "nat-rem").expect("real checked remainder result closes its refined argument");
    assert!(remainder.scheme.contains("refined") && remainder.scheme.contains("Algebra.Arithmetic.NonZero"), "{}", remainder.scheme);
    assert!(remainder.nominals.iter().any(|row| row.identity == "Algebra.Arithmetic.NonZero" && row.refinement.is_some()));
    let bytes = read("Data.Bytes");
    let allocated = get(&bytes, "bytes-new").expect("checked byte-length primitive closes the allocation promise");
    assert_eq!(allocated.scheme, "((n :: Nat) -> (refined ((b.bytesLen cinch-result) == n) Bytes))");
    assert!(allocated.nominals.is_empty(), "kernel observation must not invent a nominal type");
    let mut changed = facade.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "bound").unwrap().resolved.as_mut().unwrap().scheme = bounded.scheme.replace(" <= ", " < ");
    assert_ne!(changed.iface_hash(), facade.iface_hash());
    assert_eq!(Iface::from_json(&facade.to_json()).unwrap(), facade);
    common::cleanup(&dir);
}
