//! Conditional implementations retain callable proof contracts, never a general dictionary.
use cinch::driver::{check_sources, check_sources_ifaces};
use cinch::interface::Iface;

#[test]
fn checked_conditional_declarations_follow_actual_dictionary_slots() {
    let (_, interfaces) = check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "conditional-declarations.cinch").unwrap();
    let nat = interfaces.iter().find(|iface| iface.module == "Data.Nat").unwrap();
    let sub = nat.exports.instances.iter().find(|instance| instance.id == "Data.Nat.Sub_Nat").unwrap();
    assert!(sub.resolved.is_none(), "a conditional implementation must not become an unrestricted dictionary factory");
    assert_eq!(sub.method_contracts.len(), 1);
    let method = &sub.method_contracts[0];
    let checked = method.checked.as_ref().expect("checked closed conditional declaration");
    assert_eq!(sub.methods[method.slot as usize], checked.implementation);
    assert!(checked.head.scheme.contains("Algebra.Arithmetic.Sub Nat"), "{}", checked.head.scheme);
    assert!(checked.resolved.scheme.contains("y <= x"), "{}", checked.resolved.scheme);
    assert!(checked.resolved.scheme.contains("x :: Nat") && checked.resolved.scheme.contains("y :: Nat"), "{}", checked.resolved.scheme);
    assert_eq!(Iface::from_json(&nat.to_json()).unwrap().to_json(), nat.to_json());
    let mut changed = nat.clone();
    let contract = changed.exports.instances.iter_mut().find(|instance| instance.id == sub.id).unwrap();
    contract.method_contracts[0].checked.as_mut().unwrap().resolved.scheme.push(' ');
    assert_ne!(changed.iface_hash(), nat.iface_hash(), "checked implementation contracts enter dependency identity");
    let mut forged = nat.clone();
    let contract = forged.exports.instances.iter_mut().find(|instance| instance.id == sub.id).unwrap();
    contract.method_contracts[0].checked.as_mut().unwrap().implementation = "Data.Nat.Add_Nat.+".into();
    assert!(Iface::from_json(&forged.to_json()).is_err(), "actual slot association cannot be replaced by a same-carrier method");
    let mut opaque = nat.clone();
    opaque.exports.instances.iter_mut().find(|instance| instance.id == sub.id).unwrap().method_contracts[0].checked = None;
    assert!(Iface::from_json(&opaque.to_json()).is_ok(), "older display-only contracts remain opaque and readable");
    assert!(!opaque.to_json().contains("\"checked\":null"), "missing optional authority keeps the old display-record encoding");
    for (body, accepted) in [
        ("previous :: Nat -> Nat\n  previous n = if n == 0 then 0 else n - 1", true),
        ("bad :: Nat -> Nat\n  bad n = n - 1", false),
    ] {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        assert_eq!(check_sources(&source, "conditional-precondition.cinch").is_ok(), accepted);
    }
}
