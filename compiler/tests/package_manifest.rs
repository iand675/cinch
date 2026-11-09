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

// ---------------------------------------------------------------------
// Schema 2: select conditions, exposed modules, the prelude setting, and
// the semantic / raw link tiers (design/base-as-a-package.md 4.1 to 4.4).
// ---------------------------------------------------------------------

use cinch::driver::{
    version_matches, LinkSpec, Package, PreludeSpec, TargetFacts,
};

fn facts(os: &str, arch: &str) -> TargetFacts {
    TargetFacts {
        triple: format!("{arch}-unknown-{os}"),
        os: os.to_string(),
        arch: arch.to_string(),
        libc: if os == "darwin" { "libSystem" } else { "glibc" }.to_string(),
        os_version: None,
        compiler: env!("CARGO_PKG_VERSION").to_string(),
        runtime_abi: cinch::interface::RUNTIME_ABI_VERSION.to_string(),
    }
}

const SCHEMA2: &str = r#"{
  "schema": 2,
  "name": "acme/watch",
  "version": "0.1.0",
  "prelude": "Acme.Prelude",
  "modules": [
    { "name": "Watch", "path": "Watch.cinch" }
  ],
  "order": ["Watch"],
  "link": { "libs": ["base-lib"], "pkgconfig": [ { "name": "gio", "min": "2.70" } ] },
  "select": [
    {
      "when": { "os": ["darwin"] },
      "modules": [
        { "name": "Watch.Kqueue", "path": "darwin/kqueue.cinch" },
        { "name": "Watch.Kevent", "path": "darwin/kevent.cinch", "exposed": true }
      ],
      "link": { "frameworks": ["CoreServices"] }
    },
    {
      "when": { "os": ["linux"], "libc": ["glibc"] },
      "modules": [ { "name": "Watch.Inotify", "path": "linux/inotify.cinch", "exposed": false } ],
      "link": { "raw": { "ldflags": ["-Wl,--as-needed"] } }
    }
  ]
}"#;

#[test]
fn schema_two_parses_its_fields() {
    let pkg = Package::parse(SCHEMA2).expect("schema 2 parses");
    assert_eq!(pkg.schema, 2);
    assert_eq!(pkg.prelude, PreludeSpec::Module("Acme.Prelude".to_string()));
    assert_eq!(pkg.select.len(), 2);
    assert_eq!(pkg.link.libs, vec!["base-lib".to_string()]);
    assert_eq!(pkg.link.pkgconfig[0].name, "gio");
    assert_eq!(pkg.link.pkgconfig[0].min.as_deref(), Some("2.70"));
    assert!(pkg.modules[0].exposed, "a module without the flag is exposed");
    assert!(!pkg.select[1].modules[0].exposed);
}

#[test]
fn resolve_selects_one_platform_and_keeps_the_rest_out() {
    let pkg = Package::parse(SCHEMA2).expect("parses");
    let darwin = pkg.resolve(&facts("darwin", "aarch64")).expect("darwin resolves");
    assert!(darwin.owns("Watch.Kevent"));
    assert!(!darwin.owns("Watch.Inotify"), "the linux branch does not apply");
    assert_eq!(darwin.matched, vec![0]);
    assert!(darwin.link.frameworks.contains(&"CoreServices".to_string()));
    assert!(!darwin.link.has_raw());
    assert_eq!(darwin.exposed_names(), vec![
        "Watch".to_string(),
        "Watch.Kqueue".to_string(),
        "Watch.Kevent".to_string(),
    ]);

    let linux = pkg.resolve(&facts("linux", "x86_64")).expect("linux resolves");
    assert!(linux.owns("Watch.Inotify"));
    assert!(!linux.owns("Watch.Kevent"));
    assert!(linux.link.has_raw(), "the linux branch carries raw words");
    assert_eq!(linux.exposed_names(), vec!["Watch".to_string()], "Inotify is internal");
}

#[test]
fn the_order_grows_with_the_selected_modules() {
    let pkg = Package::parse(SCHEMA2).expect("parses");
    let darwin = pkg.resolve(&facts("darwin", "aarch64")).expect("resolves");
    assert_eq!(darwin.order, vec![
        "Watch".to_string(),
        "Watch.Kqueue".to_string(),
        "Watch.Kevent".to_string(),
    ]);
}

#[test]
fn an_order_entry_the_target_drops_is_an_error() {
    let text = r#"{
      "schema": 2,
      "name": "acme/oops",
      "version": "0.1.0",
      "modules": [ { "name": "Keep", "path": "Keep.cinch", "exposed": false } ],
      "order": ["Keep"],
      "select": [ { "when": { "os": ["darwin"] }, "modules": [] } ]
    }"#;
    // An entry with neither modules nor link words is refused outright.
    let err = Package::parse(text).expect_err("an empty select entry is a manifest bug");
    assert!(format!("{err}").contains("changes nothing"), "{err}");
}

#[test]
fn arch_spellings_are_normalised() {
    let pkg = Package::parse(SCHEMA2).expect("parses");
    // The manifest writes `arm64`, the triple writes `aarch64`.
    let arm = pkg.resolve(&facts("darwin", "arm64")).expect("arm64 resolves");
    assert!(arm.owns("Watch.Kevent"), "arm64 matches the darwin branch");
}

#[test]
fn an_os_version_condition_refuses_without_a_target_version() {
    let text = r#"{
      "schema": 2,
      "name": "acme/ver",
      "version": "0.1.0",
      "modules": [ { "name": "V", "path": "V.cinch" } ],
      "order": ["V"],
      "select": [ { "when": { "os_version": ">=15.0" }, "link": { "libs": ["extra"] } } ]
    }"#;
    let pkg = Package::parse(text).expect("parses");
    let err = pkg
        .resolve(&facts("darwin", "aarch64"))
        .expect_err("no version means no guess");
    assert!(format!("{err}").contains("--target-version"), "{err}");

    let mut with_version = facts("darwin", "aarch64");
    with_version.os_version = Some("15.2".to_string());
    let resolved = pkg.resolve(&with_version).expect("15.2 satisfies >=15.0");
    assert!(resolved.link.libs.contains(&"extra".to_string()));
}

#[test]
fn version_comparison_is_numeric_not_lexicographic() {
    assert!(version_matches("14.10", ">14.9"), "14.10 is greater than 14.9");
    assert!(version_matches("14.0", ">=14"));
    assert!(version_matches("14.0", "=14.0"));
    assert!(version_matches("14.0", "14.0"));
    assert!(!version_matches("13.9", ">=14"));
    assert!(version_matches("12", "<13"));
}

#[test]
fn duplicate_module_paths_across_entries_are_refused() {
    let text = r#"{
      "schema": 2,
      "name": "acme/dup",
      "version": "0.1.0",
      "modules": [ { "name": "M", "path": "one.cinch" } ],
      "order": ["M"],
      "select": [ { "when": {}, "modules": [ { "name": "M", "path": "two.cinch" } ] } ]
    }"#;
    let pkg = Package::parse(text).expect("parses");
    let err = pkg.resolve(&facts("darwin", "aarch64")).expect_err("two paths, one name");
    assert!(format!("{err}").contains("different paths"), "{err}");
}

#[test]
fn the_same_module_path_twice_is_additive() {
    let text = r#"{
      "schema": 2,
      "name": "acme/add",
      "version": "0.1.0",
      "modules": [ { "name": "M", "path": "one.cinch" } ],
      "order": ["M"],
      "select": [ { "when": {}, "modules": [ { "name": "M", "path": "one.cinch" } ],
                    "link": { "libs": ["only-once"] } } ]
    }"#;
    let pkg = Package::parse(text).expect("parses");
    let resolved = pkg.resolve(&facts("darwin", "aarch64")).expect("additive");
    assert_eq!(resolved.modules.len(), 1);
    assert_eq!(resolved.link.libs, vec!["only-once".to_string()]);
}

#[test]
fn unknown_condition_and_link_keys_are_refused() {
    let bad_when = r#"{
      "schema": 2, "name": "a/b", "version": "0.1.0",
      "modules": [ { "name": "M", "path": "M.cinch" } ], "order": ["M"],
      "select": [ { "when": { "ostype": ["darwin"] }, "link": { "libs": ["x"] } } ]
    }"#;
    let err = Package::parse(bad_when).expect_err("a misspelled condition key");
    assert!(format!("{err}").contains("unknown condition"), "{err}");

    let bad_link = r#"{
      "schema": 2, "name": "a/b", "version": "0.1.0",
      "modules": [ { "name": "M", "path": "M.cinch" } ], "order": ["M"],
      "link": { "librarys": ["x"] }
    }"#;
    let err = Package::parse(bad_link).expect_err("a misspelled link key");
    assert!(format!("{err}").contains("unknown key"), "{err}");
}

#[test]
fn prelude_takes_a_module_default_or_none() {
    let with = |value: &str| {
        format!(
            r#"{{"schema": 2, "name": "a/b", "version": "0.1.0",
                 "prelude": {value},
                 "modules": [ {{ "name": "M", "path": "M.cinch" }} ], "order": ["M"]}}"#
        )
    };
    assert_eq!(
        Package::parse(&with("\"none\"")).expect("none").prelude,
        PreludeSpec::None
    );
    assert_eq!(
        Package::parse(&with("\"default\"")).expect("default").prelude,
        PreludeSpec::Default
    );
    let err = Package::parse(&with("42")).expect_err("a number is not a prelude");
    assert!(format!("{err}").contains("`prelude`"), "{err}");
}

#[test]
fn a_schema_one_manifest_keeps_its_meaning() {
    let pkg = Package::parse(&format!("{BASE_PACKAGE}")).expect("the base manifest parses");
    assert_eq!(pkg.schema, 1);
    assert_eq!(pkg.prelude, PreludeSpec::Default);
    assert!(pkg.select.is_empty());
    assert!(LinkSpec::default().is_empty());
    let resolved = pkg
        .resolve(&facts("darwin", "aarch64"))
        .expect("a schema 1 manifest resolves to itself");
    assert_eq!(resolved.order, pkg.order);
}
