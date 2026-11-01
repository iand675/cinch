//! Definition-site numeric conditions must survive separate compilation.
mod common;
use cinch::interface::Iface;

#[test]
fn numeric_conditions_keep_checked_argument_positions_and_reexports() {
    let dir = common::project("interface_preconditions", &[
        ("A.cinch", r#"module A where
  data Token where
    Token :: Token
  pred :: using (n > 0) => (n :: Nat) -> Nat
  pred actual = actual - 1
  difference :: using (left >= right) => (left :: Nat) -> (right :: Nat) -> Nat
  difference a b = a - b
  keep :: using (n > 0) => (n :: Nat) -> Token -> Token
  keep n value = value
  signature Gate a where
    gate :: a -> a
  keep-with :: forall a. using (witness :: Gate a, n > 0) => (n :: Nat) -> a -> a
  keep-with n value = value
  scalar :: using (n > 0) => (n :: I64) -> I64
  scalar n = n
  reflected :: using (n <= bytes-len bytes) => (bytes :: Bytes) -> (n :: Nat) -> Nat
  reflected bytes n = n
"#),
        ("Facade.cinch", "module Facade where\n  import A (pred as previous, difference, keep)\n  export previous, difference, keep\n"),
        ("Untrusted.cinch", "module Untrusted where\n  bytes-len :: Nat -> Nat\n  bytes-len n = n\n  reflected :: using (bytes-len n > 0) => (n :: Nat) -> Nat\n  reflected n = n\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  import Untrusted\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit checked numeric contracts");
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let a = read("A");
    let facade = read("Facade");
    let pred = a.exports.values.iter().find(|v| v.name == "pred").unwrap();
    let resolved = pred.resolved.as_ref().expect("numeric predicate must be portable");
    assert!(resolved.scheme.contains("n > 0") && resolved.scheme.contains("n :: Nat"), "{}", resolved.scheme);
    assert!(!resolved.scheme.contains('$'), "argument positions must be bound");
    assert_eq!(pred.arity, 1, "proof condition is erased");
    assert_eq!(facade.exports.values.iter().find(|v| v.name == "previous").unwrap().resolved, pred.resolved);
    let keep = a.exports.values.iter().find(|v| v.name == "keep").unwrap().resolved.as_ref().unwrap();
    assert!(keep.scheme.contains("A.Token"));
    assert_eq!(keep.nominals.iter().map(|n| n.identity.as_str()).collect::<Vec<_>>(), ["A.Token"]);
    let difference = a.exports.values.iter().find(|v| v.name == "difference").unwrap().resolved.as_ref().unwrap();
    assert!(difference.scheme.contains("left >= right"), "{}", difference.scheme);
    assert!(a.exports.values.iter().find(|v| v.name == "scalar").unwrap().resolved.is_none(), "machine predicates still need checked carrier support");
    let reflected = a.exports.values.iter().find(|v| v.name == "reflected").unwrap().resolved.as_ref().expect("actual byte-length primitive closes its checked bound");
    assert!(reflected.scheme.contains("n <= (b.bytesLen bytes)") && reflected.scheme.contains("bytes :: Bytes") && reflected.scheme.contains("n :: Nat"), "{}", reflected.scheme);
    let untrusted = read("Untrusted");
    assert!(untrusted.exports.values.iter().find(|v| v.name == "reflected").unwrap().resolved.is_none(), "ordinary same-named function cannot supply the primitive contract");
    let mixed = a.exports.values.iter().find(|v| v.name == "keep-with").unwrap();
    let mixed_contract = mixed.resolved.as_ref().expect("dictionary and proof contexts survive together");
    assert_eq!(mixed.arity, 3, "one dictionary plus two computational arguments, no proof argument");
    assert!(mixed_contract.scheme.contains("witness :: A.Gate a") && mixed_contract.scheme.contains("n > 0"));
    assert_eq!(Iface::from_json(&a.to_json()).unwrap(), a);
    let mut changed = a.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "pred").unwrap().resolved.as_mut().unwrap().scheme = resolved.scheme.replace("> 0", "> 1");
    assert_ne!(changed.iface_hash(), a.iface_hash(), "predicate changes invalidate consumers");
    common::cleanup(&dir);
}
