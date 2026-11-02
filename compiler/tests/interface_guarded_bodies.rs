//! Guarded body transport follows checked code, not a method spelling.
use cinch::driver::check_sources_ifaces;
use cinch::interface::{GuardedNatExpr, Iface};

fn provider(body: &str) -> Iface {
    let source = format!("module A where\n  signature Operation a where\n    apply-op :: a -> a -> a\n  implicit instance Operation_Nat :: Operation Nat where\n    apply-op :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat\n    apply-op x y = {body}\nmodule Main where\n  import A\n  main :: Int\n  main = 0\n");
    check_sources_ifaces(&source, "guarded-body-provider.cinch").unwrap().1.into_iter().find(|iface| iface.module == "A").unwrap()
}
#[test]
fn guarded_bodies_retain_actual_code_and_formal_scope() {
    let (_, interfaces) = check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "guarded-base.cinch").unwrap();
    let nat = interfaces.iter().find(|iface| iface.module == "Data.Nat").unwrap();
    let method = &nat.exports.instances.iter().find(|instance| instance.id == "Data.Nat.Sub_Nat").unwrap().method_contracts[0];
    let body = method.checked.as_ref().unwrap().body.as_ref().expect("actual Nat subtraction body");
    assert_eq!(body.parameters, 2);
    assert_eq!(body.root, GuardedNatExpr::Subtract(Box::new(GuardedNatExpr::Argument(0)), Box::new(GuardedNatExpr::Argument(1))));
    let original = provider("nat-sub x y");
    let changed = provider("x");
    let contract = |iface: &Iface| iface.exports.instances.iter().find(|instance| instance.id == "A.Operation_Nat").unwrap().method_contracts[0].checked.as_ref().unwrap().clone();
    let first = contract(&original); let second = contract(&changed);
    assert_eq!(first.implementation, second.implementation);
    assert_eq!(first.resolved, second.resolved);
    assert_ne!(first.body, second.body, "same class/slot/scheme cannot supply a body equation");
    assert_eq!(second.body.unwrap().root, GuardedNatExpr::Argument(0));
    assert_ne!(original.iface_hash(), changed.iface_hash(), "actual guarded body changes invalidate dependent interfaces");
    assert_eq!(Iface::from_json(&original.to_json()).unwrap().to_json(), original.to_json());
    let mut forged = original.clone();
    forged.exports.instances.iter_mut().find(|instance| instance.id == "A.Operation_Nat").unwrap().method_contracts[0].checked.as_mut().unwrap().body.as_mut().unwrap().root = GuardedNatExpr::Argument(2);
    assert!(Iface::from_json(&forged.to_json()).is_err(), "a body cannot read outside its actual formal scope");
    let mut too_deep = original.clone();
    let root = &mut too_deep.exports.instances.iter_mut().find(|instance| instance.id == "A.Operation_Nat").unwrap().method_contracts[0].checked.as_mut().unwrap().body.as_mut().unwrap().root;
    for _ in 0..65 { *root = GuardedNatExpr::Subtract(Box::new(GuardedNatExpr::Argument(0)), Box::new(root.clone())); }
    assert!(Iface::from_json(&too_deep.to_json()).is_err(), "body transport has the same bounded depth as its checked producer");
}
