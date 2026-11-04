//! Imported constructors and class members need their checked kinds,
//! result indices, defining identities and exact public spellings.
mod common;

use cinch::interface::{Iface, SCHEMA_VERSION};

#[test]
fn interfaces_preserve_indexed_constructors_and_method_visibility() {
    let dir = common::project("interface_kinds", &[
        ("Contract.cinch", r#"module Contract where
  export Vec, VNil, VCons, Empty, Handle, Mapper, reveal, lift
  data Vec (n :: Nat) a where
    VNil :: Vec 0 a
    VCons :: forall (k :: Nat). a -> Vec k a -> Vec (k + 1) a
  data Empty {a :: Type} where {}
  opaque type Handle a :: Type
  lift :: forall (f :: Type -> Type) a. f a -> f a
  lift x = x
  signature Mapper (f :: Type -> Type) where
    map-item :: forall a b. (a -> b) -> f a -> f b
  signature Hidden a where
    reveal :: a -> a
    private-method :: a -> a
"#),
        ("Facade.cinch", "module Facade where\n  import Contract (Vec as Sequence, VNil as EmptySeq, VCons as Push, Mapper as Transform, reveal as expose)\n  export Sequence, EmptySeq, Push, Transform, expose\n"),
        ("Host.cinch", "module Host where\n  export Handle, read-handle\n  opaque type Handle :: Type\n  foreign import ccall \"h_read\" read-handle :: Borrowed Handle ->{Foreign Host, IO} I64\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  import Host\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("check metadata fixture");
    let read = |name: &str| Iface::from_json(&std::fs::read_to_string(dir.join(format!("build/{name}.cinchif"))).unwrap()).unwrap();
    let contract = read("Contract");
    let facade = read("Facade");
    let host = read("Host");
    let foreign = host.exports.values.iter().find(|v| v.name == "read-handle").expect("public foreign binding is an imported value");
    assert_eq!(foreign.symbol, "Host.read-handle");
    assert_eq!(foreign.arity, 1);
    assert!(foreign.scheme.contains("Borrowed"));
    assert_eq!(contract.schema, SCHEMA_VERSION);
    let vec = contract.exports.types.iter().find(|t| t.name == "Vec").unwrap();
    assert_eq!(vec.identity, "Contract.Vec");
    assert_eq!(vec.parameters.iter().map(|p| (p.name.as_str(), p.kind.as_str(), p.implicit)).collect::<Vec<_>>(),
        vec![("n", "Nat", false), ("a", "Type", false)]);
    let nil = vec.ctors.iter().find(|c| c.name == "VNil").unwrap();
    assert_eq!(nil.symbol, "Contract.VNil");
    assert!(nil.scheme.contains("Vec 0 a"), "{}", nil.scheme);
    let cons = vec.ctors.iter().find(|c| c.name == "VCons").unwrap();
    assert!(cons.scheme.contains("(k :: Nat)"), "{}", cons.scheme);
    assert!(cons.scheme.contains("k + 1"), "{}", cons.scheme);
    let empty = contract.exports.types.iter().find(|t| t.name == "Empty").unwrap();
    assert_eq!(empty.params, ["a"]);
    assert_eq!((&*empty.parameters[0].kind, empty.parameters[0].implicit), ("Type", true));
    assert_eq!(contract.exports.types.iter().find(|t| t.name == "Handle").unwrap().kind, "opaque");
    let mapper = contract.exports.signatures.iter().find(|s| s.identity == "Contract.Mapper").unwrap();
    assert_eq!(mapper.parameters[0].kind, "Type -> Type");
    assert!(mapper.methods[0].scheme.contains("f :: Type -> Type"), "{}", mapper.methods[0].scheme);
    assert!(contract.exports.values.iter().find(|v| v.name == "lift").unwrap().scheme.contains("f :: Type -> Type"));
    assert_eq!(mapper.exports, ["Mapper"]);
    assert!(mapper.methods[0].exports.is_empty(), "publishing a class does not publish its private method names");
    let hidden = contract.exports.signatures.iter().find(|s| s.identity == "Contract.Hidden").unwrap();
    assert!(hidden.exports.is_empty(), "a public method does not publish its private class name");
    assert_eq!(hidden.methods.iter().find(|m| m.name == "reveal").unwrap().exports, ["reveal"]);
    assert!(hidden.methods.iter().find(|m| m.name == "private-method").unwrap().exports.is_empty());
    let sequence = facade.exports.types.iter().find(|t| t.name == "Sequence").unwrap();
    assert_eq!(sequence.identity, vec.identity);
    assert_eq!(sequence.ctors.iter().find(|c| c.name == "Push").unwrap().symbol, "Contract.VCons");
    let hidden = facade.exports.signatures.iter().find(|s| s.identity == "Contract.Hidden").unwrap();
    assert!(hidden.exports.is_empty());
    assert_eq!(hidden.methods.iter().find(|m| m.name == "reveal").unwrap().exports, ["expose"]);
    assert_eq!(facade.exports.signatures.iter().find(|s| s.identity == "Contract.Mapper").unwrap().exports, ["Transform"]);
    let mut changed = contract.clone();
    changed.exports.types.iter_mut().find(|t| t.name == "Vec").unwrap().parameters[0].kind = "Int".into();
    assert_ne!(contract.iface_hash(), changed.iface_hash());
    let mut changed = contract.clone();
    changed.exports.types.iter_mut().find(|t| t.name == "Vec").unwrap().ctors[0].scheme.push_str(" changed");
    assert_ne!(contract.iface_hash(), changed.iface_hash());
    let old = contract.to_json().replace(&format!("\"schema\":{SCHEMA_VERSION}"), "\"schema\":2");
    assert!(Iface::from_json(&old).is_err(), "schema 2 cannot supply these contracts");
    common::cleanup(&dir);
}

#[test]
fn opaque_parameter_kinds_survive_checked_interfaces_and_facades() {
    let source = r#"module A where
  export Mark, Wrap, keep-mark, keep-wrap
  opaque type Mark (n :: Nat) :: Type
  opaque type Wrap (f :: Type -> Type) :: Type
  keep-mark :: forall (n :: Nat). Mark n -> Mark n
  keep-mark x = x
  keep-wrap :: forall (f :: Type -> Type). Wrap f -> Wrap f
  keep-wrap x = x
module Facade where
  import A (Mark as M, Wrap as W, keep-mark, keep-wrap)
  export M, W, keep-mark, keep-wrap
module Main where
  import Facade
  main :: Int
  main = 0
"#;
    let (_, interfaces) = cinch::driver::check_sources_ifaces(source, "opaque-kinds.cinch").unwrap();
    let a = interfaces.iter().find(|i| i.module == "A").unwrap();
    let facade = interfaces.iter().find(|i| i.module == "Facade").unwrap();
    for (name, alias, kind) in [("Mark", "M", "Nat"), ("Wrap", "W", "Type -> Type")] {
        let original = a.exports.types.iter().find(|t| t.name == name).unwrap();
        let renamed = facade.exports.types.iter().find(|t| t.name == alias).unwrap();
        assert_eq!(original.kind, "opaque");
        assert!(original.ctors.is_empty() && original.fields.is_empty());
        assert_eq!(original.parameters[0].kind, kind);
        assert_eq!(renamed.identity, original.identity);
        assert_eq!(renamed.parameters, original.parameters);
        assert_eq!(Iface::from_json(&facade.to_json()).unwrap(), *facade);
    }
    for name in ["keep-mark", "keep-wrap"] {
        let original = a.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().unwrap();
        let renamed = facade.exports.values.iter().find(|v| v.name == name).unwrap().resolved.as_ref().unwrap();
        assert_eq!(renamed, original);
        assert!(original.nominals.iter().any(|n| n.kind == "opaque" && n.constructors.is_none()), "opaque kind lost its definition-owned abstract contract");
    }
    for body in [
        "bad :: Mark Nat -> Nat\n  bad x = 0",
        "bad :: Wrap Nat -> Nat\n  bad x = 0",
    ] {
        let wrong = format!("module Main where\n  opaque type Mark (n :: Nat) :: Type\n  opaque type Wrap (f :: Type -> Type) :: Type\n  {body}\n");
        assert!(cinch::driver::check_sources(&wrong, "bad-opaque-kind.cinch").is_err(), "opaque parameter kind was erased: {body}");
    }
}
