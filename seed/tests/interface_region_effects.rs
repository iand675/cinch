//! Kernel capabilities and closed builtin rows retain region identities.
mod common;
use cinch::interface::{Iface, SCHEMA_VERSION};

#[test]
fn resolved_contracts_keep_region_binders_and_closed_kernel_effects() {
    let dir = common::project("region_contracts", &[
        ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
        ("A.cinch", r#"module A where
  import Data.Mutable.Var
  export identity-io, write, implicit-region, callback, row-polymorphic, signal, read-cell, mixed-cell, open-cell
  effect Notice where
    signal :: Nat ->{Notice} Nat
  identity-io :: forall r. Io r -> Io r
  identity-io value = value
  write :: forall r. using (io :: Io r) => Bytes ->{IO r} Unit
  write bytes = ioWrite bytes
  implicit-region :: Unit ->{IO r} Unit
  implicit-region unit = with (io = newIO ()) in ioWrite []
  callback :: forall r. using (io :: Io r) => (Unit ->{IO r} Unit) ->{IO r} Unit
  callback f = f ()
  row-polymorphic :: forall e. (Unit ->{|e} Unit) ->{|e} Unit
  row-polymorphic f = f ()
  -- | Preserve the real mutable cell owner in a closed mutation contract.
  read-cell :: forall r. Var r Nat ->{Mut r} Nat
  read-cell cell = var-read cell
  -- | Mutation and I/O can refer to the same explicit runtime identity.
  mixed-cell :: forall r. Var r Nat ->{IO r, Mut r} Nat
  mixed-cell cell = var-read cell
  -- | An open callback row remains outside the closed transport protocol.
  open-cell :: forall r e. Var r Nat -> (Nat ->{|e} Nat) ->{Mut r | e} Nat
  open-cell cell callback = callback (var-read cell)
"#),
        ("Main.cinch", "module Main where\n  import A\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit region/effect contracts");
    let a = common::project_interface(&dir, "Main.cinch", "A");
    assert_eq!(a.schema, SCHEMA_VERSION);
    let signal = a.exports.values.iter().find(|v| v.name == "signal").expect("public operation must not disappear");
    assert_eq!(signal.symbol, "A.signal");
    assert_eq!(signal.arity, 1);
    assert!(signal.scheme.contains("Notice"));
    assert!(signal.resolved.is_none());
    assert!(signal.total_body.is_none());
    assert!(!a.exports.types.iter().any(|t| t.name == "Notice"), "private effect label gained a public name");
    for name in ["identity-io", "write", "implicit-region", "callback"] {
        let value = a.exports.values.iter().find(|v| v.name == name).unwrap();
        let resolved = value.resolved.as_ref().unwrap_or_else(|| panic!("missing {name}: {}", value.scheme));
        assert!(resolved.scheme.contains("r :: Region"), "{name}: {}", resolved.scheme);
        assert!(!resolved.scheme.contains("<builtin>."), "kernel identities must not invent module dependencies");
        assert!(resolved.nominals.iter().all(|n| !n.identity.starts_with("<builtin>.")));
        if name == "write" || name == "callback" {
            assert!(resolved.scheme.contains("io :: Io r"), "{name}: {}", resolved.scheme);
        }
        if name != "identity-io" {
            assert!(resolved.scheme.contains("->{IO r}"), "{name}: {}", resolved.scheme);
        }
    }
    let implicit = a.exports.values.iter().find(|v| v.name == "implicit-region").unwrap().resolved.as_ref().unwrap();
    assert!(implicit.scheme.contains("{r :: Region}"), "inferred binder stays explicit in the contract");
    assert!(a.exports.values.iter().find(|v| v.name == "row-polymorphic").unwrap().resolved.is_none(),
        "an open row must not be published as closed");
    for name in ["read-cell", "mixed-cell"] {
        let value = a.exports.values.iter().find(|v| v.name == name).unwrap();
        let resolved = value.resolved.as_ref().unwrap_or_else(|| panic!("missing closed mutation contract: {name}"));
        assert!(resolved.scheme.contains("Mut r"), "{}", resolved.scheme);
        assert!(resolved.scheme.contains("r :: Region"), "{}", resolved.scheme);
        assert!(resolved.nominals.iter().any(|n| n.identity == "Data.Mutable.Var.Var"));
        if name == "mixed-cell" { assert!(resolved.scheme.contains("IO r")); }
    }
    assert!(a.exports.values.iter().find(|v| v.name == "open-cell").unwrap().resolved.is_none(),
        "an open mutation row must not be restamped as closed");
    assert!(Iface::from_json(&a.to_json().replace(&format!("\"schema\":{SCHEMA_VERSION}"), "\"schema\":5")).is_err());
    common::cleanup(&dir);
}

#[test]
fn scope_and_task_contracts_preserve_kernel_and_source_type_owners() {
    let source = r#"module Scoped where
  inspect :: forall r s. Scope r s -> Int
  inspect token = 7
  pending :: forall s e a. Unit -> Task s e a
  pending unit = pending unit
module Shadow where
  data Scope where
    ScopeValue :: Scope
  identity :: Shadow.Scope -> Shadow.Scope
  identity value = value
  -- | A source-owned task is distinct from the kernel task carrier.
  data Task where
    -- | A normal source constructor cannot mint a kernel task.
    TaskValue :: Task
  -- | Preserve the qualified source task identity.
  task-identity :: Shadow.Task -> Shadow.Task
  task-identity value = value
module Main where
  import Scoped as Scopes
  import Shadow as Local
  main :: Int
  main = 0
"#;
    let dir = common::project("scope_task_interface", &[("Main.cinch", source)]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("emit scope and task contracts");
    let scoped = interfaces.iter().find(|i| i.module == "Scoped").unwrap();
    let readback = Iface::from_json(&scoped.to_json()).expect("real schema8 kernel-contract readback");
    assert_eq!(readback.to_json(), scoped.to_json());
    for (name, kernel) in [("inspect", "Scope"), ("pending", "Task")] {
        let value = readback.exports.values.iter().find(|v| v.name == name).unwrap();
        let resolved = value.resolved.as_ref().unwrap_or_else(|| panic!("missing {name}: {}", value.scheme));
        assert!(resolved.scheme.contains(kernel) && resolved.scheme.contains(":: Region"));
        assert!(resolved.nominals.is_empty(), "kernel carriers must not invent module-owned nominal contracts");
        assert!(!resolved.scheme.contains("<builtin>."));
    }
    let shadow = interfaces.iter().find(|i| i.module == "Shadow").unwrap();
    let identity = shadow.exports.values.iter().find(|v| v.name == "identity").unwrap().resolved.as_ref().unwrap();
    assert!(identity.scheme.contains("Shadow.Scope"));
    assert!(identity.nominals.iter().any(|n| n.identity == "Shadow.Scope"));
    let task = shadow.exports.values.iter().find(|v| v.name == "task-identity").unwrap().resolved.as_ref().unwrap();
    assert!(task.scheme.contains("Shadow.Task"));
    assert!(task.nominals.iter().any(|n| n.identity == "Shadow.Task"));
    cinch::interface::verify_chain(&interfaces).expect("actual source owners and dependencies");
    common::cleanup(&dir);
}

/// A declaration-owned effect sharing a kernel spelling must not acquire kernel transport authority.
#[test]
fn owned_mutation_spelling_is_not_exported_as_a_kernel_effect() {
    let dir = common::project("owned_mutation_contract", &[
        ("A.cinch", r#"-- | A declaration-owned operation is distinct from kernel mutation.
module A where
  stage family :: Declarations
  stage family = quoteDecls where {
    -- | This lexical mutation spelling belongs to this declaration group.
    effect Mut r where { invoke :: Unit ->{Mut r} Nat };
    -- | Preserve the owned effect rather than claiming kernel mutation.
    run :: forall r. Unit ->{Mut r} Nat;
    run unit = invoke unit }
  generate do
    emit family
"#),
        ("Main.cinch", "-- | Check a declaration-owned effect without executing it.\nmodule Main where\n  import A\n  -- | The consumer keeps the checked import reachable.\n  main :: Int\n  main = 0\n"),
    ]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("owned effect source is valid");
    let interface = interfaces.iter().find(|i| i.module == "A").unwrap();
    let run = interface.exports.values.iter().find(|value| value.name == "run").unwrap();
    assert!(run.resolved.is_none(), "an owned effect was mistaken for kernel Mut: {:?}", run.resolved);
    common::cleanup(&dir);
}

#[test]
fn a_one_shot_callback_keeps_its_nominal_and_region_contract() {
    let dir = common::project("clock_callback_contract", &[
        ("ClockWitness.cinch", r#"module ClockWitness where
  data Delay where
    Delay :: Nat -> Delay
  data Outcome a where
    Finished :: a -> Outcome a
  wait :: forall a r. using (io :: Io r) => Delay -> (Unit ->{IO r, Cancel r} a) %1 ->{Async r, IO r, Cancel r, Diverge} Outcome a
  wait delay work = Finished (work ())
"#),
        ("Main.cinch", "module Main where\n  import ClockWitness\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit callback contract");
    let text = common::project_interface(&dir, "Main.cinch", "ClockWitness").to_json();
    let iface = Iface::from_json(&text).unwrap();
    let wait = iface.exports.values.iter().find(|v| v.name == "wait").unwrap();
    let contract = wait.resolved.as_ref().expect("portable one-shot callback");
    assert!(contract.scheme.contains("%1 ->"), "{}", contract.scheme);
    for name in ["ClockWitness.Delay", "ClockWitness.Outcome"] {
        assert!(contract.nominals.iter().any(|ty| ty.identity == name), "missing {name}");
    }
    common::cleanup(&dir);
}
