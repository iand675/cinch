//! The base package manifest: the module list and the resolution ORDER
//! live in `compiler/prelude/cinch-base.cinchpkg`, not in the driver's
//! code (design/base-library.md, "Packages and the manifest"). These
//! tests pin the manifest against the embedded source table, so drift
//! is caught rather than trusted, and one of them proves the family
//! still checks end to end with the order coming from the data.

use cinch::driver::{
    base_package, check_sources, embedded_source, prelude_module_names, prelude_sources, scan_file,
    BASE_PACKAGE, EMBEDDED_SOURCES,
};
use std::collections::BTreeSet;

/// The generated WIT adapter vocabulary is appended to the facade
/// source and has no `.cinch` file of its own.
const GENERATED: &str = "Prelude.Generated";

#[test]
fn manifest_parses_and_names_match_the_embedded_sources() {
    let pkg = base_package();
    assert_eq!(pkg.name, "cinch-base");
    assert!(!pkg.version.is_empty(), "the manifest names its version");

    let listed: BTreeSet<&str> = pkg.modules.iter().map(|m| m.path.as_str()).collect();
    let embedded: BTreeSet<&str> = EMBEDDED_SOURCES.iter().map(|(p, _)| *p).collect();
    assert_eq!(
        listed, embedded,
        "every manifest path needs an embedded source and every source needs a manifest entry"
    );
    assert_eq!(
        pkg.modules.len(),
        EMBEDDED_SOURCES.len(),
        "and exactly once each: a duplicate path would make the source table ambiguous"
    );
}

#[test]
fn the_order_is_a_permutation_of_the_modules() {
    let pkg = base_package();
    let names: BTreeSet<&str> = pkg.modules.iter().map(|m| m.name.as_str()).collect();
    let ordered: Vec<&str> = pkg.order.iter().map(|n| n.as_str()).collect();
    assert_eq!(
        ordered.len(),
        names.len(),
        "the order must name every module exactly once"
    );
    let ordered_set: BTreeSet<&str> = ordered.iter().copied().collect();
    assert_eq!(ordered_set, names, "and name nothing else");
}

#[test]
fn exactly_one_facade_splits_the_family() {
    let pkg = base_package();
    let facades: Vec<&str> = pkg
        .modules
        .iter()
        .filter(|m| m.facade)
        .map(|m| m.name.as_str())
        .collect();
    assert_eq!(facades, vec!["Prelude"], "one facade, named Prelude");
    let at = pkg.facade_index();
    assert!(
        at > 0 && at < pkg.order.len() - 1,
        "the facade sits between the early homes and the late ones, got index {at} of {}",
        pkg.order.len()
    );
    assert_eq!(pkg.order[at], "Prelude", "and its index names it");
}

#[test]
fn the_driver_reads_the_manifest() {
    let pkg = base_package();
    assert_eq!(
        prelude_module_names(),
        pkg.order,
        "the driver's module list is the manifest's order, not a copy"
    );
    let sources = prelude_sources();
    assert_eq!(sources.len(), pkg.order.len());
    for (i, (path, src)) in sources.iter().enumerate() {
        let entry = embedded_source(path).expect("a manifest path has embedded bytes");
        assert_eq!(*src, entry.1, "entry {i} ({path}) is the manifest's own source");
    }
    // The manifest lists the facade too; its source is the facade file.
    assert!(pkg.order.iter().any(|n| n == "Prelude"));
}

#[test]
fn every_import_has_a_home_or_is_generated() {
    // Scanned from the SOURCES, not from a copy in the manifest: the
    // manifest deliberately does not record imports (see
    // `PackageModule`), so this checks that the package's module list is
    // complete against what the sources actually reach for.
    let pkg = base_package();
    let names: BTreeSet<&str> = pkg.modules.iter().map(|m| m.name.as_str()).collect();
    let mut missing: Vec<String> = Vec::new();
    for m in &pkg.modules {
        let (path, src) = embedded_source(&m.path).expect("a manifest path has embedded bytes");
        let scanned = scan_file(src, path).expect("the family lexes and lays out");
        let declared: Vec<&(String, Vec<(String, bool)>)> = scanned
            .modules
            .iter()
            .filter(|(name, _)| name == &m.name)
            .collect();
        assert_eq!(
            declared.len(),
            1,
            "{} must declare exactly one module, found {}",
            m.path,
            declared.len()
        );
        for (import, _stage) in &declared[0].1 {
            if import != GENERATED && !names.contains(import.as_str()) {
                missing.push(format!("{} imports {}", m.name, import));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "imports with no home in the package:\n{}",
        missing.join("\n")
    );
}

#[test]
fn the_family_still_checks_with_the_manifest_order() {
    // A home that the facade does NOT re-export, reached with an
    // explicit import: its surface only exists if the family was
    // collected in a workable order.
    // Each class name costs an explicit import of its own home: the
    // facade re-exports no class home, and an imported name is not
    // re-exported by the importing module, so `<>` needs
    // Algebra.Semigroup and `mempty` needs Algebra.Monoid.
    let src = "module Main where\n\
               \x20 import Algebra.Semigroup\n\
               \x20 import Algebra.Monoid\n\
               \n\
               \x20 main :: Int\n\
               \x20 main = case mempty <> (mempty :: Sum) of\n\
               \x20   Sum n -> nat-to-int n\n";
    match check_sources(src, "<manifest-order>") {
        Ok(_) => {}
        Err(e) => panic!("the manifest-ordered family must still check: {e}"),
    }
    assert!(
        BASE_PACKAGE.contains("\"schema\": 1"),
        "the embedded manifest is the canonical JSON document itself"
    );
}
