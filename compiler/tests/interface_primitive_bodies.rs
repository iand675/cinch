//! Actual checked primitive values retain their own executable contracts.
mod common;

use cinch::interface::{Iface, verify_chain};
use common::source_frontend::check_source_interfaces;
use cinch::total::{Expr, Op, Sort};
#[test]
fn plain_primitive_values_survive_without_an_ordinary_caller() {
    let interfaces = check_source_interfaces("module Main where\n  main :: Int\n  main = 0\n", "primitive-roots.cinch").unwrap();
    let nat = interfaces.iter().find(|interface| interface.module == "Data.Nat").unwrap();
    for (name, operation, result) in [("nat-add", Op::NatAdd, Sort::Nat), ("nat-mul", Op::NatMul, Sort::Nat), ("nat-cmp", Op::NatCmp, Sort::I64)] {
        let value = nat.exports.values.iter().find(|value| value.name == name).unwrap();
        let graph = value.total_body.as_ref().expect("the checked ABI primitive is a real defining value");
        let definition = graph.get(&value.symbol).unwrap();
        assert_eq!(definition.parameters, [Sort::Nat, Sort::Nat]); assert_eq!(definition.result, result);
        assert_eq!(definition.body, Expr::Primitive(operation, vec![Expr::Local(0), Expr::Local(1)]));
        assert_eq!(graph.definitions().count(), 1, "no unrelated bodies enter the root closure");
    }
    for name in ["nat-sub", "nat-div", "nat-rem", "nat-from-i64", "nat-to-i64"] {
        assert!(nat.exports.values.iter().find(|value| value.name == name).unwrap().total_body.is_none(), "partial/refined {name} cannot claim an unrestricted plain body");
    }
    for interface in &interfaces { assert_eq!(Iface::from_json(&interface.to_json()).unwrap().to_json(), interface.to_json()); }
    verify_chain(&interfaces).unwrap();
}
#[test]
fn primitive_identity_and_actual_scheme_survive_aliases_without_name_axioms() {
    let source = r#"module Owner where
  foreign import ccall "cinch.runtime.natAdd" actual :: Nat -> Nat -> Nat
  foreign import ccall "cinch.prim.wrappingAddI64" wrapping :: I64 -> I64 -> I64
  foreign import ccall "custom.opaque_add" foreign-fn :: I64 -> I64 ->{Diverge} I64
  nat-add :: Nat -> Nat -> Nat
  nat-add x y = y
module Facade where
  import Owner (actual as renamed)
  export renamed
module Main where
  import Facade
  main :: Int
  main = 0
"#;
    let interfaces = check_source_interfaces(source, "primitive-aliases.cinch").unwrap();
    let owner = interfaces.iter().find(|interface| interface.module == "Owner").unwrap();
    let actual = owner.exports.values.iter().find(|value| value.name == "actual").unwrap();
    let graph = actual.total_body.as_ref().unwrap();
    assert_eq!(graph.get("Owner.actual").unwrap().body, Expr::Primitive(Op::NatAdd, vec![Expr::Local(0), Expr::Local(1)]));
    let facade = interfaces.iter().find(|interface| interface.module == "Facade").unwrap();
    let renamed = facade.exports.values.iter().find(|value| value.name == "renamed").unwrap();
    assert_eq!(renamed.symbol, "Owner.actual"); assert_eq!(renamed.total_body, actual.total_body);
    for name in ["foreign-fn", "wrapping"] { assert!(owner.exports.values.iter().find(|value| value.name == name).unwrap().total_body.is_none()); }
    let ordinary = owner.exports.values.iter().find(|value| value.name == "nat-add").unwrap();
    assert_eq!(ordinary.total_body.as_ref().unwrap().get("Owner.nat-add").unwrap().body, Expr::Local(1));
    verify_chain(&interfaces).unwrap();
}
