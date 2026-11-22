//! Class contracts retain definition-site names and dictionary layout.
//! Facade visibility is independent of the private contract closure.
mod common;

use cinch::interface::{verify_chain, Iface, SCHEMA_VERSION};

#[test]
fn class_contexts_methods_and_private_contracts_keep_their_definitions() {
    let dir = common::project("class_contracts", &[
        ("A.cinch", r#"module A where
  import Category.Applicative (Applicative)
  export choose, Mapper, Consumer, Exists, HideAny
  data Hidden where
    Secret :: Hidden
  data Exists where
    HideAny :: forall t. t -> Exists
  signature Gate a where
    z-first :: a -> Hidden
    a-last :: forall b. using (eq :: Eq b) => (a -> b) -> a -> b
  signature Mapper (f :: Type -> Type) where
    map-item :: forall a b. (a -> b) -> f a -> f b
  signature Consumer a where
    transform :: forall f b. using (app :: Applicative f) => (a -> f b) -> a -> f b
  choose :: forall a. using (gate :: Gate a) => a -> a
  choose x = x
  instance Gate_Int :: Gate Int where
    z-first x = Secret
    a-last f x = f x
  implicit instance Gate_List a :: using (eq :: Eq a) => Gate (List a) where
    z-first x = Secret
    a-last f x = f x
"#),
        ("Left.cinch", "module Left where\n  signature Choice a where\n    left-item :: a -> a\n"),
        ("Right.cinch", "module Right where\n  signature Choice a where\n    right-item :: a -> a\n"),
        ("Facade.cinch", "module Facade where\n  import A (choose as use)\n  export use\n"),
        ("Main.cinch", r#"module Main where
  import Left as L
  import Right as R
  import Facade
  pair :: forall a. using (left :: L.Choice a, right :: R.Choice a) => a -> a
  pair x = x
  main :: Int
  main = 0
"#),
    ]);
    common::check(&dir, "Main.cinch").expect("emit class contracts");
    let read = |module: &str| Iface::from_json(&std::fs::read_to_string(
        dir.join(format!("build/{module}.cinchif"))).unwrap()).unwrap();
    let a = read("A");
    let facade = read("Facade");
    let main = read("Main");
    let choose = a.exports.values.iter().find(|v| v.name == "choose").unwrap().resolved.as_ref().unwrap();
    assert!(choose.scheme.contains("gate :: A.Gate a"), "{}", choose.scheme);
    let gate = choose.nominals.iter().find(|n| n.identity == "A.Gate").unwrap();
    assert_eq!(gate.kind, "signature");
    assert!(gate.constructors.is_none());
    assert_eq!(gate.parameters[0].kind, "Type");
    assert_eq!(facade.exports.values.iter().find(|v| v.name == "use").unwrap().resolved.as_ref(), Some(choose));
    let gate = facade.exports.signatures.iter().find(|s| s.identity == "A.Gate").unwrap();
    assert!(gate.exports.is_empty());
    assert_eq!(gate.methods.iter().map(|m| (m.name.as_str(), m.slot)).collect::<Vec<_>>(),
        [("z-first", 0), ("a-last", 1)], "dictionary order must survive canonical serialization");
    assert!(gate.methods.iter().all(|m| m.exports.is_empty()));
    assert!(facade.exports.types.iter().all(|t| t.identity != "A.Hidden"));
    let first = gate.methods[0].resolved.as_ref().unwrap();
    assert!(first.scheme.contains("dict :: A.Gate a"), "{}", first.scheme);
    assert!(first.scheme.contains("A.Hidden"), "{}", first.scheme);
    assert_eq!(first.nominals.iter().find(|n| n.identity == "A.Hidden").unwrap()
        .constructors.as_deref(), Some(["A.Secret".to_string()].as_slice()));
    let last = gate.methods[1].resolved.as_ref().unwrap();
    assert_eq!(last.scheme.matches("forall ").count(), 1, "inherited forall must be hoisted: {}", last.scheme);
    assert!(last.scheme.contains("eq :: Relation.Eq.Eq b"), "{}", last.scheme);
    assert!(facade.exports.signatures.iter().any(|s| s.identity == "Relation.Eq.Eq" && s.exports.is_empty()));
    let mapper = a.exports.signatures.iter().find(|s| s.identity == "A.Mapper").unwrap();
    assert!(mapper.methods[0].resolved.as_ref().unwrap().scheme.contains("(f :: Type -> Type)"));
    let consumer = a.exports.signatures.iter().find(|s| s.identity == "A.Consumer").unwrap();
    let transform = &consumer.methods[0].resolved.as_ref().unwrap().scheme;
    assert!(consumer.methods[0].scheme.contains("f :: Type -> Type"));
    assert!(transform.contains("(f :: Type -> Type)"), "method-local higher kind: {transform}");
    assert!(transform.contains("app :: Category.Applicative.Applicative f"), "{transform}");
    let exists = a.exports.types.iter().find(|t| t.identity == "A.Exists").unwrap();
    assert!(exists.ctors[0].resolved.is_none(), "an existential is not a universal constructor scheme");
    let pair = main.exports.values.iter().find(|v| v.name == "pair").unwrap().resolved.as_ref().unwrap();
    assert!(pair.scheme.contains("left :: Left.Choice a"), "{}", pair.scheme);
    assert!(pair.scheme.contains("right :: Right.Choice a"), "{}", pair.scheme);
    assert!(pair.nominals.iter().any(|n| n.identity == "Left.Choice"));
    assert!(pair.nominals.iter().any(|n| n.identity == "Right.Choice"));
    let explicit = a.exports.instances.iter().find(|i| i.id == "A.Gate_Int").unwrap();
    assert!(!explicit.implicit);
    assert_eq!(explicit.resolved.as_ref().unwrap().scheme, "A.Gate Int");
    let generic = a.exports.instances.iter().find(|i| i.id == "A.Gate_List").unwrap();
    assert!(generic.implicit);
    let generic = generic.resolved.as_ref().unwrap();
    assert!(generic.scheme.contains("eq :: Relation.Eq.Eq a"), "{}", generic.scheme);
    assert!(generic.scheme.contains("A.Gate (Data.Core.List a)"), "{}", generic.scheme);
    let nat = read("Data.Nat");
    assert!(nat.exports.instances.iter().find(|i| i.id == "Data.Nat.Add_Nat").unwrap().resolved.is_some());
    let subtraction = nat.exports.instances.iter().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap();
    assert!(subtraction.resolved.is_none(), "conditional methods cannot become unconstrained dictionaries");
    assert_eq!(subtraction.method_contracts.len(), 1);
    assert_eq!(subtraction.method_contracts[0].slot, 0);
    assert!(subtraction.method_contracts[0].scheme.contains("arg1 <= arg0"), "{}", subtraction.method_contracts[0].scheme);
    assert!(!subtraction.method_contracts[0].scheme.contains('$'));
    let boolean = read("Data.Bool");
    assert!(boolean.exports.instances.iter().find(|i| i.id == "Data.Bool.Codec_Bool").unwrap().resolved.is_some(),
        "dependency-ordered emission builds the defining Codec contract before Bool's instance");
    assert!(boolean.imports.iter().any(|r| r.module == "Text.Codec" && r.iface_hash == read("Text.Codec").iface_hash()));

    let all: Vec<Iface> = std::fs::read_dir(dir.join("build")).unwrap().filter_map(|entry| {
        let path = entry.unwrap().path();
        (path.extension().is_some_and(|s| s == "cinchif"))
            .then(|| Iface::from_json(&std::fs::read_to_string(path).unwrap()).unwrap())
    }).collect();
    verify_chain(&all).expect("facade aliases keep the defining contract");
    let mut changed = facade.clone();
    changed.exports.signatures.iter_mut().find(|s| s.identity == "A.Gate").unwrap()
        .methods[0].resolved.as_mut().unwrap().scheme.push_str(" altered");
    assert_ne!(changed.iface_hash(), facade.iface_hash());
    let mut changed_set: Vec<_> = all.iter().filter(|f| f.module != "Main" && f.module != "Facade").cloned().collect();
    changed_set.push(changed);
    assert!(verify_chain(&changed_set).unwrap_err().0.contains("conflicting defining signature contract"));
    let mut malformed = facade.clone();
    malformed.exports.signatures[0].methods[0].slot = 7;
    assert!(Iface::from_json(&malformed.to_json()).is_err());
    let without_slot = facade.to_json().replacen("\"slot\":0", "\"unchecked_slot\":0", 1);
    assert!(Iface::from_json(&without_slot).is_err());
    let mut without_resolved = facade.to_json();
    let start = without_resolved.find("\"methods\":[").unwrap();
    let start = start + without_resolved[start..].find("\"resolved\":").unwrap();
    without_resolved.replace_range(start..start + "\"resolved\"".len(), "\"unchecked\"");
    assert!(Iface::from_json(&without_resolved).is_err());
    let mut erased_precondition = nat.clone();
    erased_precondition.exports.instances.iter_mut().find(|i| i.id == "Data.Nat.Sub_Nat").unwrap().resolved = explicit.resolved.clone();
    assert!(Iface::from_json(&erased_precondition.to_json()).is_err());
    assert!(Iface::from_json(&facade.to_json().replace(
        &format!("\"schema\":{SCHEMA_VERSION}"), "\"schema\":4")).is_err());
    common::cleanup(&dir);
}
