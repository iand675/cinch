//! Imported constructors and class members need their checked kinds,
//! result indices, defining identities and exact public spellings.
mod common;

use cinch::interface::{Iface, SCHEMA_VERSION};

#[test]
fn partial_arrow_former_survives_a_checked_instance_interface() {
    let source = r#"module ArrowMapping where
  signature Mapper (f :: Type -> Type) where
    map-item :: forall a b. (a -> b) -> f a -> f b
  implicit instance MapperArrow r :: Mapper ((->) r) where
    map-item f g x = f (g x)
module Main where
  import ArrowMapping
  main :: Int
  main = 0
"#;
    let dir = common::project("arrow_instance", &[("Main.cinch", source)]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("emit the arrow instance contract");
    cinch::interface::verify_chain(&interfaces).unwrap();
    let owner = interfaces.iter().find(|iface| iface.module == "ArrowMapping").unwrap();
    let artifact = Iface::from_json(&owner.to_json()).unwrap();
    let instance = artifact.exports.instances.iter().find(|item| item.id == "ArrowMapping.MapperArrow").unwrap();
    assert_eq!(instance.class, "ArrowMapping.Mapper");
    assert_eq!(instance.target, "ArrowMapping.Mapper ((->) r)");
    let resolved = instance.resolved.as_ref().expect("checked arrow former in the instance head");
    assert_eq!(resolved.scheme, "forall (r :: Type). ArrowMapping.Mapper ((->) r)");
    assert_eq!(resolved.nominals.iter().map(|nominal| nominal.identity.as_str()).collect::<Vec<_>>(),
        ["ArrowMapping.Mapper"]);
    common::cleanup(&dir);
}

#[test]
fn ordering_instances_publish_definition_owned_headers() {
    let source = "module Main where\n  import Data.Ordering\n  main :: Int\n  main = 0\n";
    let dir = common::project("ordering_instance_headers", &[("Main.cinch", source)]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("load the checked Ordering headers");
    cinch::interface::verify_chain(&interfaces).unwrap();
    let owner = interfaces.iter().find(|iface| iface.module == "Data.Ordering").unwrap();
    for name in ["Monoid", "Semigroup"] {
        let id = format!("Data.Ordering.{name}_Ordering");
        let instance = owner.exports.instances.iter().find(|item| item.id == id).unwrap();
        let class = format!("Algebra.{name}.{name}");
        let head = format!("{class} Data.Ordering.Ordering");
        assert_eq!(instance.class, class, "the Ordering instance keeps its defining class");
        assert_eq!(instance.target, head);
        assert_eq!(instance.resolved.as_ref().unwrap().scheme, head);
    }
    common::cleanup(&dir);
}

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
    assert_eq!(mapper.methods[0].scheme, mapper.methods[0].resolved.as_ref().unwrap().scheme);
    let mut mismatched = contract.clone();
    mismatched.exports.signatures.iter_mut().find(|s| s.identity == "Contract.Mapper").unwrap()
        .methods[0].scheme.push(' ');
    let error = Iface::from_json(&mismatched.to_json()).unwrap_err();
    assert!(error.to_string().contains("class method spelling disagrees with its checked scheme"));
    let lift = contract.exports.values.iter().find(|v| v.name == "lift").unwrap();
    assert!(lift.scheme.contains("f :: Type -> Type"));
    assert_eq!(lift.scheme, lift.resolved.as_ref().unwrap().scheme);
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
    let dir = common::project("opaque_kinds", &[("Main.cinch", source)]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("emit the opaque and facade contracts");
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
    common::cleanup(&dir);
    for body in [
        "bad :: Mark Nat -> Nat\n  bad x = 0",
        "bad :: Wrap Nat -> Nat\n  bad x = 0",
    ] {
        let wrong = format!("module Main where\n  opaque type Mark (n :: Nat) :: Type\n  opaque type Wrap (f :: Type -> Type) :: Type\n  {body}\n");
        let dir = common::project("bad_opaque_kind", &[("Main.cinch", &wrong)]);
        assert!(common::check(&dir, "Main.cinch").is_err(), "opaque parameter kind was erased: {body}");
        common::cleanup(&dir);
    }
}
