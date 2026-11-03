//! Kernel capabilities and closed builtin rows retain region identities.
mod common;
use cinch::interface::{Iface, SCHEMA_VERSION};

#[test]
fn resolved_contracts_keep_region_binders_and_closed_kernel_effects() {
    let dir = common::project("region_contracts", &[
        ("A.cinch", r#"module A where
  export identity-io, write, implicit-region, callback, row-polymorphic, signal
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
"#),
        ("Main.cinch", "module Main where\n  import A\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").expect("emit region/effect contracts");
    let a = Iface::from_json(&std::fs::read_to_string(dir.join("build/A.cinchif")).unwrap()).unwrap();
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
    assert!(Iface::from_json(&a.to_json().replace(&format!("\"schema\":{SCHEMA_VERSION}"), "\"schema\":5")).is_err());
    common::cleanup(&dir);
}
