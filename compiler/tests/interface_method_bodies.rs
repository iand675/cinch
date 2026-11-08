//! Actual selected implementations travel without publishing private names.
use cinch::interface::{Iface, verify_chain};
use cinch::total::{CheckedGraph, Definition, Expr, Sort};

const OWNER: &str = r#"module Owner where
  export Probe, observe
  signature Probe a where
    observe :: a -> a
    hidden :: a -> a
    loop :: a -> a
    generic :: forall b. b -> b
  implicit instance AtNat :: Probe Nat where
    observe n = private-helper n
    hidden n = n
    loop n = loop n
    generic x = x
  private-helper :: Nat -> Nat
  private-helper n = nat-add n 1
"#;
fn interfaces(source: &str) -> Vec<Iface> { cinch::driver::check_sources_ifaces(source, "method-bodies.cinch").unwrap().1 }
fn owner(interfaces: &[Iface]) -> &Iface { interfaces.iter().find(|interface| interface.module == "Owner").unwrap() }
fn reject(mut interface: Iface, change: impl FnOnce(&mut cinch::interface::InstanceMethodBody)) {
    change(&mut interface.exports.instances[0].method_bodies[0]);
    assert!(Iface::from_json(&interface.to_json()).is_err());
}
#[test]
fn standard_numeric_methods_export_their_own_exact_closed_roots() {
    let interfaces = interfaces("module Main where\n  main :: Int\n  main = 0\n");
    for identity in ["Data.Nat.Ord_Nat.<", "Data.Nat.Ord_Nat.<=", "Data.Nat.Ord_Nat.>", "Data.Nat.Ord_Nat.>=", "Data.Nat.Add_Nat.+"] {
        let matching = interfaces.iter().flat_map(|interface| &interface.exports.instances)
            .flat_map(|instance| &instance.method_bodies).filter(|body| body.implementation == identity).collect::<Vec<_>>();
        assert_eq!(matching.len(), 1, "actual selected implementation {identity}");
        let body = matching[0];
        assert!(body.total_body.get(identity).is_some());
        assert_eq!(body.total_body.closure([identity]).unwrap(), body.total_body);
        assert!(body.resolved.nominals.is_empty() && body.resolved.measures.is_empty());
    }
    for interface in &interfaces { let canonical = interface.to_json(); assert_eq!(Iface::from_json(&canonical).unwrap().to_json(), canonical); }
    verify_chain(&interfaces).expect("all actual method dependency closures");
    let nat = interfaces.iter().find(|interface| interface.module == "Data.Nat").unwrap();
    assert!(nat.exports.instances.iter().find(|instance| instance.id == "Data.Nat.Sub_Nat").unwrap().method_bodies.is_empty(), "partial subtraction never claims an unrestricted total body");
}

#[test]
fn keyed_ordering_evidence_is_exported_from_the_actual_body() {
    let interfaces = interfaces(r#"
module Owner where
  import Relation.Ord (Ord)
  import Data.Ordering (Ordering, GT)
  export constant
  constant :: forall a b. using (ord :: Ord b) => (a -> b) -> a -> a -> Ordering
  constant key x y = GT
"#);
    let ordinal = interfaces.iter().find(|interface| interface.module == "Relation.Ord").unwrap();
    let helper = ordinal.exports.values.iter().find(|value| value.name == "comparing").unwrap();
    let body = helper.keyed_ordering.expect("derive the two actual method slots");
    assert_eq!((body.less_slot, body.greater_slot), (0, 2));
    assert!(owner(&interfaces).exports.values[0].keyed_ordering.is_none());
    assert_eq!(Iface::from_json(&ordinal.to_json()).unwrap().to_json(), ordinal.to_json());
    verify_chain(&interfaces).expect("the defining method slots belong to the checked closure");

    let mut forged = interfaces.clone();
    let ordinal = forged.iter_mut().find(|interface| interface.module == "Relation.Ord").unwrap();
    let helper = ordinal.exports.values.iter_mut().find(|value| value.name == "comparing").unwrap();
    helper.keyed_ordering.as_mut().unwrap().less_slot = body.greater_slot;
    assert!(Iface::from_json(&ordinal.to_json()).is_err());
}

#[test]
fn standard_monoid_default_transports_the_checked_owner_clauses() {
    let interfaces = interfaces("module Main where\n  import Algebra.Monoid\n  main :: Int\n  main = 0\n");
    let monoid = interfaces.iter().find(|interface| interface.module == "Algebra.Monoid").unwrap();
    let class = monoid.exports.signatures.iter()
        .find(|class| class.identity == "Algebra.Monoid.Monoid").unwrap();
    let method = class.methods.iter().find(|method| method.name == "mconcat").unwrap();
    let body = method.default.as_ref().expect("the original mconcat body").to_json();
    let identity = |name: &str| {
        let encoded = name.chars().map(|c| format!("{}_", u32::from(c))).collect::<String>();
        format!("i.{encoded}")
    };
    assert!(body.contains("\"ECase\""));
    assert!(body.contains(&identity("Algebra.Monoid.Monoid.mempty")));
    assert!(body.contains(&identity("Algebra.Semigroup.sconcat")));
    assert!(body.contains(&identity("Data.Core.None")));
    assert!(body.contains(&identity("Data.Core.Some")));
    assert_eq!(Iface::from_json(&monoid.to_json()).unwrap().to_json(), monoid.to_json());
    verify_chain(&interfaces).unwrap();
}
#[test]
fn private_calls_and_slots_remain_associated_without_leaking_source_names() {
    let interfaces = interfaces(OWNER); let original = owner(&interfaces);
    let instance = &original.exports.instances[0];
    assert_eq!(instance.method_bodies.iter().map(|body| body.slot).collect::<Vec<_>>(), [0, 1]);
    assert!(instance.method_bodies[0].total_body.get("Owner.private-helper").is_some());
    assert!(instance.method_bodies[0].total_body.get("Owner.AtNat.hidden").is_none());
    assert!(!original.exports.values.iter().any(|value| value.name.contains("private-helper") || value.symbol.contains("AtNat")));
    let class = original.exports.signatures.iter().find(|signature| signature.identity == "Owner.Probe").unwrap();
    assert!(class.methods.iter().find(|method| method.name == "hidden").unwrap().exports.is_empty());
    assert!(class.methods.iter().find(|method| method.name == "observe").unwrap().exports.contains(&"observe".to_string()));
    verify_chain(&interfaces).unwrap();
    assert!(cinch::driver::check_sources(&format!("{OWNER}\nmodule Main where\n  import Owner\n  bad :: Nat\n  bad = Owner.private-helper 1\n  main :: Int\n  main = 0\n"), "private-method.cinch").is_err());
    let changed = self::interfaces(&OWNER.replace("nat-add n 1", "nat-add n 2"));
    assert_ne!(original.iface_hash(), owner(&changed).iface_hash());
}
#[test]
fn malformed_slot_root_declaration_and_dependency_payloads_are_refused() {
    let interfaces = interfaces(OWNER); let original = owner(&interfaces).clone();
    reject(original.clone(), |body| body.slot = 99);
    reject(original.clone(), |body| body.implementation = "Owner.AtNat.hidden".into());
    reject(original.clone(), |body| body.resolved.scheme = "Int -> Nat".into());
    reject(original.clone(), |body| {
        let mut definitions = body.total_body.definitions().cloned().collect::<Vec<_>>();
        definitions.push(Definition { identity: "Owner.unrelated".into(), parameters: vec![], result: Sort::Nat, body: Expr::Number { sort: Sort::Nat, decimal: "0".into() } });
        body.total_body = CheckedGraph::check(definitions).unwrap();
    });
    let mut duplicate = original.clone(); let copied = duplicate.exports.instances[0].method_bodies[0].clone(); duplicate.exports.instances[0].method_bodies.push(copied);
    assert!(Iface::from_json(&duplicate.to_json()).is_err());
    let mut missing_factory = original; missing_factory.exports.instances[0].resolved = None;
    assert!(Iface::from_json(&missing_factory.to_json()).is_err());
}
#[test]
fn an_absent_optional_method_payload_retains_the_exact_opaque_encoding() {
    let mut interface = owner(&interfaces(OWNER)).clone();
    interface.exports.instances[0].method_bodies.clear();
    let encoded = interface.to_json(); assert!(!encoded.contains("method_bodies"));
    let decoded = Iface::from_json(&encoded).unwrap(); assert_eq!(decoded.to_json(), encoded);
    assert!(decoded.exports.instances[0].method_bodies.is_empty());
}
#[test]
fn foreign_method_dependencies_do_not_escape_the_checked_import_closure() {
    let mut interfaces = interfaces(OWNER);
    let interface = interfaces.iter_mut().find(|interface| interface.module == "Owner").unwrap();
    let body = &mut interface.exports.instances[0].method_bodies[0];
    body.total_body = CheckedGraph::check(vec![
        Definition { identity: body.implementation.clone(), parameters: vec![Sort::Nat], result: Sort::Nat, body: Expr::Call { target: "Outside.helper".into(), arguments: vec![Expr::Local(0)] } },
        Definition { identity: "Outside.helper".into(), parameters: vec![Sort::Nat], result: Sort::Nat, body: Expr::Local(0) },
    ]).unwrap();
    Iface::from_json(&interface.to_json()).expect("structurally closed typed graph before dependency-set validation");
    assert!(verify_chain(&interfaces).unwrap_err().0.contains("outside the checked module closure"));
}
#[test]
fn conflicting_method_dependencies_are_refused_even_when_each_graph_checks() {
    let mut interfaces = interfaces(OWNER);
    let interface = interfaces.iter_mut().find(|interface| interface.module == "Owner").unwrap();
    let body = &mut interface.exports.instances[0].method_bodies[1];
    body.total_body = CheckedGraph::check(vec![
        Definition { identity: body.implementation.clone(), parameters: vec![Sort::Nat], result: Sort::Nat, body: Expr::Call { target: "Owner.private-helper".into(), arguments: vec![Expr::Local(0)] } },
        Definition { identity: "Owner.private-helper".into(), parameters: vec![Sort::Nat], result: Sort::Nat, body: Expr::Number { sort: Sort::Nat, decimal: "9".into() } },
    ]).unwrap();
    Iface::from_json(&interface.to_json()).unwrap();
    assert!(verify_chain(&interfaces).unwrap_err().0.contains("conflicting actual total body dependency"));
}

#[test]
fn a_nested_module_cannot_lend_its_actual_method_to_another_dictionary() {
    let mut interfaces = interfaces(OWNER);
    let nested = self::interfaces("module Owner.Sub where\n  marker :: Nat\n  marker = 0\n").into_iter().find(|interface| interface.module == "Owner.Sub").unwrap();
    let nested_hash = nested.iface_hash(); interfaces.push(nested);
    let original = interfaces.iter().find(|interface| interface.module == "Owner").unwrap().clone();
    for replace_dictionary in [false, true] {
        let mut altered = original.clone();
        altered.imports.push(cinch::interface::IfaceRef { module: "Owner.Sub".into(), iface_hash: nested_hash.clone() });
        if replace_dictionary { altered.exports.instances[0].id = "Owner.Sub.AtNat".into(); }
        else {
            let body = &mut altered.exports.instances[0].method_bodies[0];
            body.implementation = "Owner.Sub.AtNat.observe".into();
            body.total_body = CheckedGraph::check(vec![Definition { identity: body.implementation.clone(), parameters: vec![Sort::Nat], result: Sort::Nat, body: Expr::Local(0) }]).unwrap();
            altered.exports.instances[0].methods[0] = "Owner.Sub.AtNat.observe".into();
        }
        Iface::from_json(&altered.to_json()).expect("well-formed rows still require defining-module ownership");
        *interfaces.iter_mut().find(|interface| interface.module == "Owner").unwrap() = altered;
        assert!(verify_chain(&interfaces).unwrap_err().0.contains("not owned by its declaring dictionary module"));
    }
}
