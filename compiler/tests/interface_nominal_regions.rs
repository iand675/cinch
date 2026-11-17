//! Explicit declaration-owned Region roles survive nominal contracts and source deletion.
mod common;

use cinch::interface::Iface;
use std::collections::BTreeMap;
use std::path::Path;

/// Check genuine source and retain the exact interfaces produced by the selected compiler.
fn producer() -> (std::path::PathBuf, Vec<Iface>) {
    let dir = common::project("nominal_region_producer", &[
        ("Producer.cinch", include_str!("fixtures/nominal-region-contracts.cinch")),
    ]);
    let interfaces = common::check_interfaces(&dir, "Producer.cinch")
        .expect("check explicit nominal Region declarations");
    (dir, interfaces)
}

/// Require public and private contracts to retain the same defining parameter telescope.
#[test]
fn explicit_nominal_region_headers_and_resolved_reexports_agree() {
    let (dir, interfaces) = producer();
    cinch::interface::verify_chain(&interfaces).unwrap();
    let owner = interfaces.iter().find(|iface| iface.module == "RegionOwner").unwrap();
    let facade = interfaces.iter().find(|iface| iface.module == "RegionFacade").unwrap();
    for (name, alias, function) in [
        ("Carrier", "Handle", "run"), ("Phantom", "EmptyHandle", "keep-phantom"),
        ("Envelope", "Envelope", "keep-envelope"), ("Observed", "Observed", "keep-observed"),
        ("Mark", "Mark", "keep-mark"),
    ] {
        let header = owner.exports.types.iter().find(|row| row.name == name).unwrap();
        let exported = facade.exports.types.iter().find(|row| row.name == alias).unwrap();
        assert_eq!(header.parameters[0].kind, "Region", "{name}");
        assert_eq!(exported.identity, header.identity);
        assert_eq!(exported.parameters, header.parameters);
        let value = owner.exports.values.iter().find(|row| row.name == function).unwrap();
        let resolved = value.resolved.as_ref().expect(function);
        let nominal = resolved.nominals.iter().find(|row| row.identity == header.identity).unwrap();
        assert_eq!(nominal.parameters, header.parameters, "{function}");
        assert!(resolved.scheme.contains("r :: Region"), "{}", resolved.scheme);
        let exported_name = if function == "run" { "invoke" } else { function };
        assert_eq!(facade.exports.values.iter().find(|row| row.name == exported_name).unwrap().resolved, value.resolved);
        assert_eq!(Iface::from_json(&owner.to_json()).unwrap(), *owner);
    }
    let phantom = owner.exports.types.iter().find(|row| row.name == "Phantom").unwrap();
    assert!(phantom.parameters[0].implicit, "erasure and implicitness are independent roles");
    assert!(owner.exports.values.iter().find(|row| row.name == "open-run").unwrap().resolved.is_none(),
        "a nominal role must not grant open-row transport authority");
    let legacy = owner.exports.types.iter().find(|row| row.name == "Legacy").unwrap();
    assert_eq!(legacy.parameters[0].kind, "Type", "unannotated legacy headers are never restamped");
    let legacy_use = owner.exports.values.iter().find(|row| row.name == "legacy-use").unwrap().resolved.as_ref().unwrap();
    assert!(legacy_use.scheme.contains("r :: Region") && legacy_use.scheme.contains("Mut r"), "{}", legacy_use.scheme);
    assert_eq!(legacy_use.nominals.iter().find(|row| row.identity == "RegionOwner.Legacy").unwrap().parameters,
        legacy.parameters, "the function's latent identity cannot rewrite its defining nominal header");
    for (name, binder) in [("keep-uppercase", "R :: Region"), ("keep-io", "r :: Region")] {
        let resolved = owner.exports.values.iter().find(|row| row.name == name).unwrap().resolved.as_ref().expect(name);
        assert!(resolved.scheme.contains(binder), "{}", resolved.scheme);
    }
    common::cleanup(&dir);
}

/// Sibling quantifiers and separate declarations may reuse a spelling without sharing its role.
#[test]
fn nominal_region_kind_notes_stay_with_their_lexical_binders() {
    let source = r#"-- | Exercise distinct kind-note scopes that deliberately reuse one spelling.
module Main where
  -- | This carrier supplies checked Region authority to its own argument slot.
  data Carrier (r :: Region) a where {}
  -- | A pure selector infers its identity role from the defining carrier alone.
  region-first :: forall r a. Carrier r a -> Carrier r a
  region-first value = value
  -- | A later declaration's same-spelling ordinary type has a separate telescope.
  type-second :: forall (r :: Type). r -> r
  type-second value = value
  -- | Reversing declaration order preserves the same lexical separation.
  region-third :: forall (r :: Region) a. Carrier r a -> Carrier r a
  region-third value = value
  -- | Region then Type siblings keep their locally quantified names separate.
  region-then-type :: (forall (r :: Region). Carrier r Nat -> Carrier r Nat) -> (forall (r :: Type). r -> r) -> Unit
  region-then-type first second = ()
  -- | Type then Region siblings also retain independent quantifier roles.
  type-then-region :: (forall (r :: Type). r -> r) -> (forall (r :: Region). Carrier r Nat -> Carrier r Nat) -> Unit
  type-then-region first second = ()
  -- | Keep the scope check independent from runtime mutation.
  main :: Int
  main = 0
"#;
    let dir = common::project("nominal_region_lexical_scopes", &[("Main.cinch", source)]);
    let interfaces = common::check_interfaces(&dir, "Main.cinch").expect("independent binders do not share kind notes");
    let main = interfaces.iter().find(|row| row.module == "Main").unwrap();
    for (name, expected) in [("region-first", "r :: Region"), ("type-second", "r :: Type"), ("region-third", "r :: Region")] {
        let value = main.exports.values.iter().find(|row| row.name == name).unwrap().resolved.as_ref().unwrap();
        assert!(value.scheme.contains(expected), "{name}: {}", value.scheme);
    }
    common::cleanup(&dir);
    for body in [
        "bad :: forall (r :: Region). Carrier r Nat -> (forall (r :: Type). r -> r) -> Unit\n  bad carrier callback = ()",
        "bad :: forall (r :: Type). r -> (forall (r :: Region). Carrier r Nat -> Carrier r Nat) -> Unit\n  bad value callback = ()",
    ] {
        let input = format!("-- | An inner binder may not shadow an enclosing telescope.\nmodule Main where\n  -- | Retain a defining Region slot for the refusal.\n  data Carrier (r :: Region) a where {{}}\n  -- | Refuse the existing nested-shadow boundary.\n  {body}\n");
        let dir = common::project("nominal_region_shadow_refusal", &[("Main.cinch", &input)]);
        let error = common::check(&dir, "Main.cinch").expect_err("nested forall shadowing remains refused");
        assert!(error.contains("shadows a type variable already in scope"), "{error}");
        common::cleanup(&dir);
    }
}

/// Reject incompatible explicit source roles before any interface is published.
#[test]
fn explicit_nominal_region_parameters_refuse_type_index_and_owner_confusion() {
    for (name, declaration) in [
        ("region as a field type", "data Bad (r :: Region) where\n    BadValue :: r -> Bad r"),
        ("Type as a mutation identity", "data Bad (r :: Type) where\n    BadValue :: (Unit ->{Mut r} Unit) -> Bad r"),
        ("concrete type as a region", "data Handle (r :: Region) where {}\n  bad :: Handle Nat -> Nat\n  bad value = 0"),
        ("explicit Type as a region", "data Handle (r :: Region) where {}\n  bad :: forall (a :: Type). Handle a -> Nat\n  bad value = 0"),
        ("checked role cannot also be a value type", "data Handle (r :: Region) where {}\n  bad :: forall r. Handle r -> r -> r\n  bad handle value = value"),
        ("region as an explicit Type argument", "data Plain (a :: Type) where {}\n  data Bad (r :: Region) where\n    BadValue :: Plain r -> Bad r"),
        ("region as an index", "data Count (n :: Nat) where {}\n  data Bad (r :: Region) where\n    BadValue :: Count r -> Bad r"),
        ("region as a higher kind", "data Higher (f :: Type -> Type) where {}\n  data Bad (r :: Region) where\n    BadValue :: Higher r -> Bad r"),
        ("duplicate role binder", "data Bad (r :: Region) (r :: Type) where {}"),
    ] {
        let documented = declaration.lines().map(|line| {
            let line = line.strip_prefix("  ").unwrap_or(line);
            if line.contains(" :: ") || line.trim_start().starts_with("data ") {
                let indentation = &line[..line.len() - line.trim_start().len()];
                format!("{indentation}-- | Exercise a declaration-owned role refusal.\n{line}")
            } else { line.to_owned() }
        }).collect::<Vec<_>>().join("\n");
        let documented = documented.lines().map(|line| format!("  {line}")).collect::<Vec<_>>().join("\n");
        let source = format!("-- | Refuse an inconsistent nominal role.\nmodule Main where\n{documented}\n");
        let dir = common::project("nominal_region_refusal", &[("Main.cinch", &source)]);
        let error = common::check(&dir, "Main.cinch").expect_err(name);
        assert!(error.contains("kind") || error.contains("region") || error.contains("identity") || error.contains("duplicate"),
            "{name}: {error}");
        common::cleanup(&dir);
    }
}

/// Write complete dependency hashes without changing any defining nominal contract.
fn manifest(directory: &Path, interfaces: &[Iface]) {
    let mut rows = Vec::new();
    std::fs::create_dir_all(directory.join("interfaces")).unwrap();
    for interface in interfaces.iter().filter(|row| row.module != "Main") {
        let path = directory.join("interfaces").join(format!("{}.cinchif", interface.module));
        std::fs::write(&path, interface.to_json()).unwrap();
        rows.push(serde_json::json!([interface.module, interface.iface_hash(), path]));
    }
    std::fs::write(directory.join("interfaces.json"), serde_json::json!({"schema":1,"interfaces":rows}).to_string()).unwrap();
}

/// Recheck genuine source-absent metadata, including the historical inconsistent-header refusal.
#[test]
fn compiled_native_checker_reads_source_absent_nominal_region_roles() {
    let (source, interfaces) = producer();
    let consumer = common::project("nominal_region_native_consumer", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let executable = common::build(&consumer, "Main.cinch", "nominal-region-checker")
        .expect("build actual Native interface checker");
    manifest(&consumer, &interfaces);
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    assert!(!consumer.join("RegionOwner.cinch").exists());
    for (name, body, status) in [
        ("closed mutation through alias", "safe :: forall (s :: Region) a. Handle s a ->{Mut s} a\n  safe value = invoke value", 0),
        ("qualified reexport retains owner", "safe :: forall (s :: Region) a. Qualified.Handle s a ->{Mut s} a\n  safe value = Qualified.invoke value", 0),
        ("pure alias retains role", "safe :: forall (s :: Region) a. Handle s a -> Handle s a\n  safe value = value", 0),
        ("abstract explicit role", "safe :: forall (s :: Region) a. Mark s a -> Mark s a\n  safe value = keep-mark value", 0),
        ("different identities", "bad :: forall (s :: Region) (t :: Region) a. Handle s a ->{Mut t} a\n  bad value = invoke value", 4),
        ("ordinary type is not a region", "bad :: Handle Nat Nat -> Nat\n  bad value = 0", 4),
        ("legacy Type header remains authoritative", "bad :: forall (s :: Region). Legacy s ->{Mut s} Unit\n  bad value = legacy-use value", 4),
    ] {
        let import = if body.contains("Qualified.") { "import RegionFacade as Qualified" } else { "import RegionFacade" };
        let source = format!("-- | Recheck the defining nominal role after source deletion.\nmodule Main where\n  {import}\n  -- | Exercise one source-absent role boundary.\n  {body}\n  -- | Keep checking separate from runtime mutation.\n  main :: Int\n  main = 0\n");
        std::fs::write(consumer.join("source.cinch"), source).unwrap();
        let output = std::process::Command::new(&executable).current_dir(&consumer).output().unwrap();
        let report = common::read_output(&consumer, "imported-report.txt");
        assert_eq!(output.status.code(), Some(status), "{name}: {report}\n{}", String::from_utf8_lossy(&output.stderr));
        if status == 0 { assert!(report.is_empty(), "{name}: {report}"); }
    }
    // A changed defining header gets fresh dependency hashes, not new authority.
    let mut changed = interfaces.clone();
    let owner = changed.iter_mut().find(|row| row.module == "RegionOwner").unwrap();
    owner.exports.types.iter_mut().find(|row| row.name == "Carrier").unwrap().parameters[0].kind = "Type".into();
    for _ in 0..3 {
        let hashes: BTreeMap<_, _> = changed.iter().map(|row| (row.module.clone(), row.iface_hash())).collect();
        for interface in &mut changed {
            for dependency in &mut interface.imports { dependency.iface_hash = hashes[&dependency.module].clone(); }
        }
    }
    cinch::interface::verify_chain(&changed).expect("forged test keeps the complete hash closure coherent");
    manifest(&consumer, &changed);
    std::fs::write(consumer.join("source.cinch"), "-- | A mismatched header must be rejected.\nmodule Main where\n  import RegionFacade\n  -- | Retain the claimed identity without executing it.\n  safe :: forall (s :: Region) a. Handle s a -> Handle s a\n  safe value = value\n").unwrap();
    let output = std::process::Command::new(&executable).current_dir(&consumer).output().unwrap();
    assert_eq!(output.status.code(), Some(90), "the loader must refuse contradictory defining nominal contracts before elaboration");
    common::cleanup(&source);
    common::cleanup(&consumer);
}
