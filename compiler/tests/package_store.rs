//! The package artifact store: the key's input list, the on-disk entry,
//! the hard errors a mismatched or corrupt artifact produces, and the
//! `--rebuild` escape that recomputes and rewrites an entry.

use cinch::core::{ArtifactProfile, TargetProfile, VectorShape};
use cinch::driver::{base_package, check_sources_ifaces, Package, PackageModule};
use cinch::interface::{CanonicalAbi, Iface, WitAdapter, content_hash};
use cinch::store::{
    BodyArtifact, ENTRY_MANIFEST, Kind, Outcome, Store, StoreError, package_key, package_key_for,
    store_root,
};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};

/// The embedded family's interfaces, checked once for every case here (a
/// prelude check is the expensive step, and each case loads the same
/// artifact set).
static BASE_IFACES: LazyLock<Vec<Iface>> = LazyLock::new(|| {
    let (_, ifaces) = check_sources_ifaces(
        "module Main where\n  main :: Int\n  main = 0\n",
        "<package_store test>",
    )
    .expect("the embedded prelude checks");
    base_package()
        .module_names()
        .iter()
        .map(|name| {
            ifaces
                .iter()
                .find(|i| &i.module == name)
                .unwrap_or_else(|| panic!("the compile produced no interface for {name}"))
                .clone()
        })
        .collect()
});

fn scratch(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("cinch_store_{}_{tag}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch directory");
    dir
}

/// Every file in a directory, by name, so two writes can be compared
/// byte for byte.
fn entry_files(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<(String, Vec<u8>)> = std::fs::read_dir(dir)
        .expect("entry directory")
        .map(|e| {
            let e = e.expect("entry");
            (
                e.file_name().to_string_lossy().to_string(),
                std::fs::read(e.path()).expect("read entry file"),
            )
        })
        .collect();
    files.sort();
    files
}

/// The canonical form of an interface, which is what its artifact holds:
/// `to_json` writes sorted lists, while the checker builds some of them
/// in check order, so the artifact's own round trip is to this form
/// (`iface_hash` covers exactly this rendering).
fn canonical(iface: &Iface) -> Iface {
    Iface::from_json(&iface.to_json()).expect("an interface's own artifact parses")
}

fn canonical_set(ifaces: &[Iface]) -> Vec<Iface> {
    ifaces.iter().map(canonical).collect()
}

// ------------------------------------------------------------- the key

const SAMPLE_MANIFEST: &str = "{\"name\": \"cinch-sample\", \"version\": \"1.4.0\"}";

fn sample_package() -> Package {
    Package {
        name: "cinch-sample".to_string(),
        version: "1.4.0".to_string(),
        modules: vec![
            PackageModule {
                name: "Alpha".to_string(),
                path: "Alpha.cinch".to_string(),
                facade: false,
            },
            PackageModule {
                name: "Beta".to_string(),
                path: "Beta.cinch".to_string(),
                facade: false,
            },
            PackageModule {
                name: "Facade".to_string(),
                path: "Facade.cinch".to_string(),
                facade: true,
            },
        ],
        order: vec!["Alpha".to_string(), "Beta".to_string(), "Facade".to_string()],
        manifest_hash: content_hash(SAMPLE_MANIFEST),
    }
}

fn sample_sources() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Alpha.cinch", "module Alpha where\n"),
        ("Beta.cinch", "module Beta where\n  import Alpha\n"),
        (
            "Facade.cinch",
            "module Facade where\n  import Alpha\n  import Beta\n",
        ),
    ]
}

/// An adapter with the shape `build_wit_adapter` produces, so the
/// adapters' place in the key is pinned without needing a `.wit` file.
fn sample_adapter() -> WitAdapter {
    WitAdapter {
        package: "example:codec@0.1.0".to_string(),
        abi: CanonicalAbi::Wasi02,
        module: "Raw".to_string(),
        types: Vec::new(),
        bindings: Vec::new(),
        vocab: "type Marker = Int\n".to_string(),
        vocab_names: vec!["Marker".to_string()],
        boundary_error_payload: "ForeignFailure".to_string(),
        streams: false,
    }
}

/// Every input the key's documented list names, one at a time: an input
/// missing from the list is the failure mode that makes a store worse
/// than no store, so each one has to move the key here.
#[test]
fn the_package_key_is_stable_and_covers_its_inputs() {
    let pkg = sample_package();
    let srcs = sample_sources();
    let profile = TargetProfile::native();
    let base = package_key_for(&pkg, &srcs, &profile, &[]);
    assert_eq!(
        base,
        package_key_for(&pkg, &srcs, &profile, &[]),
        "the key is byte-stable across two computations in one process"
    );
    let embedded = package_key(base_package(), &profile, &[]).expect("the base package resolves");
    assert_eq!(
        embedded,
        package_key(base_package(), &profile, &[]).expect("the base package resolves"),
        "the embedded package's key is byte-stable too"
    );
    assert_ne!(embedded, base, "two packages key differently");

    let mut variants: Vec<(&str, String)> = Vec::new();

    let mut p = sample_package();
    p.name = "cinch-other".to_string();
    variants.push(("the package name", package_key_for(&p, &srcs, &profile, &[])));

    let mut p = sample_package();
    p.version = "1.4.1".to_string();
    variants.push(("the package version", package_key_for(&p, &srcs, &profile, &[])));

    let mut p = sample_package();
    p.manifest_hash = content_hash("{\"name\": \"cinch-sample\", \"version\": \"1.4.0\", \"x\": 1}");
    variants.push((
        "the manifest bytes",
        package_key_for(&p, &srcs, &profile, &[]),
    ));

    let mut p = sample_package();
    p.order = vec!["Beta".to_string(), "Alpha".to_string(), "Facade".to_string()];
    variants.push((
        "the manifest order",
        package_key_for(&p, &srcs, &profile, &[]),
    ));

    let mut p = sample_package();
    p.modules[0].path = "Elsewhere/Alpha.cinch".to_string();
    variants.push(("a module's path", package_key_for(&p, &srcs, &profile, &[])));

    let mut p = sample_package();
    p.modules[2].facade = false;
    variants.push((
        "the facade flag",
        package_key_for(&p, &srcs, &profile, &[]),
    ));

    let mut p = sample_package();
    p.modules.push(PackageModule {
        name: "Gamma".to_string(),
        path: "Gamma.cinch".to_string(),
        facade: false,
    });
    p.order.push("Gamma".to_string());
    variants.push(("the module set", package_key_for(&p, &srcs, &profile, &[])));

    let mut s = sample_sources();
    s[1].1 = "module Beta where\n  import Alpha\n  -- a body edit\n";
    variants.push((
        "a module's source bytes",
        package_key_for(&pkg, &s, &profile, &[]),
    ));

    let mut s = sample_sources();
    s[0].0 = "Prelude/Alpha.cinch";
    variants.push((
        "a source path",
        package_key_for(&pkg, &s, &profile, &[]),
    ));

    let mut p = TargetProfile::native();
    p.name = "custom64".to_string();
    variants.push(("the profile name", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.triple = "aarch64-apple-darwin".to_string();
    variants.push(("the target triple", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.word_size = 32;
    variants.push(("the word size", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.endian = "big".to_string();
    variants.push(("the endianness", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.vector_shape = VectorShape::Scalable;
    variants.push(("the vector shape", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.features.push("simd128".to_string());
    variants.push(("the feature assumptions", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.fuel = 3;
    variants.push(("the driving fuel", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.measure = 3;
    variants.push(("the whistle's measure bound", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.allow_eval = false;
    variants.push(("the eval admission", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.artifact = ArtifactProfile::CoreWasm;
    variants.push(("the artifact shape", package_key_for(&pkg, &srcs, &p, &[])));

    let mut p = TargetProfile::native();
    p.required.push("wasm.simd128".to_string());
    variants.push((
        "the required capabilities",
        package_key_for(&pkg, &srcs, &p, &[]),
    ));

    let mut p = TargetProfile::native();
    p.jpl = true;
    variants.push(("the jpl flag", package_key_for(&pkg, &srcs, &p, &[])));

    variants.push((
        "the generated WIT adapters",
        package_key_for(&pkg, &srcs, &profile, &[sample_adapter()]),
    ));

    for (what, key) in variants {
        assert_ne!(base, key, "the key must move when {what} changes");
    }
}

// ---------------------------------------------------------------- writing

#[test]
fn writing_twice_is_idempotent_and_the_entry_reads_back() {
    let store = Store::at(scratch("idempotent"));
    let pkg = base_package();
    let profile = TargetProfile::native();
    let ifaces: &[Iface] = &BASE_IFACES;

    let first = store
        .write(pkg, &profile, &[], ifaces, false)
        .expect("a cold store write");
    let Outcome::Wrote(key) = first else {
        panic!("the cold write writes: {first:?}");
    };
    let dir = store.entry_dir(pkg, &key);
    assert_eq!(
        dir,
        store
            .root()
            .join(&pkg.name)
            .join(&pkg.version)
            .join(&key),
        "the layout is <root>/<package>/<version>/<key>"
    );

    // (b) the artifact set round-trips: every stored interface parses
    // back to the in-memory one, and its contract hash recomputes to
    // what the entry records.
    let manifest = std::fs::read_to_string(dir.join(ENTRY_MANIFEST)).expect("the entry manifest");
    assert!(
        manifest.contains(&format!("\"key\": {key:?}")),
        "the manifest records the key: {manifest}"
    );
    for iface in ifaces {
        let text = std::fs::read_to_string(Kind::Interface.path_in(&dir, &iface.module))
            .expect("the stored interface");
        let back = Iface::from_json(&text).expect("the stored interface parses");
        assert_eq!(back, canonical(iface), "{} round-trips", iface.module);
        assert_eq!(back.iface_hash(), iface.iface_hash());
        assert!(
            manifest.contains(&format!("\"iface_hash\": {:?}", iface.iface_hash())),
            "the manifest records {}'s hash",
            iface.module
        );
    }
    assert!(
        manifest.contains("\"kinds\": [\"interface\"]"),
        "the manifest records the artifact kinds present: {manifest}"
    );

    // Writing again verifies and leaves every byte alone.
    let before = entry_files(&dir);
    let second = store
        .write(pkg, &profile, &[], ifaces, false)
        .expect("the second write verifies");
    assert_eq!(second, Outcome::Verified(key.clone()));
    assert_eq!(before, entry_files(&dir), "a verified write changes nothing");

    // A second call reads the same artifacts back, the way a second
    // process would.
    let loaded = store.load(pkg, &key).expect("the loader reads the entry");
    assert_eq!(loaded.ifaces, canonical_set(ifaces));
    assert_eq!(loaded.key, key);
    assert_eq!(loaded.package, pkg.name);
    assert_eq!(loaded.version, pkg.version);
    assert!(loaded.objects.is_empty() && loaded.ir.is_empty());
    for iface in ifaces {
        assert_eq!(loaded.kinds[&iface.module], vec![Kind::Interface]);
    }
    let again = store
        .load_for(pkg, &profile, &[])
        .expect("the key is recomputed the same way");
    assert_eq!(again.ifaces, canonical_set(ifaces));
}

/// The compile's interfaces are not always the embedded package's: a
/// source module may shadow a family copy. That compile's artifacts are
/// not a function of the key, so nothing is cached.
#[test]
fn interfaces_that_are_not_the_packages_are_not_cached() {
    let store = Store::at(scratch("shadow"));
    let pkg = base_package();
    let mut ifaces = BASE_IFACES.clone();
    ifaces[0].source_hash = content_hash("module Data.Core where\n");
    let outcome = store
        .write(pkg, &TargetProfile::native(), &[], &ifaces, false)
        .expect("a shadowed family is not an error");
    let Outcome::Skipped(key) = outcome else {
        panic!("a shadowed family is skipped: {outcome:?}");
    };
    assert!(!store.entry_dir(pkg, &key).join(ENTRY_MANIFEST).exists());
}

// --------------------------------------------------------------- staleness

/// (c) and (d): a corrupt artifact is a hard error naming the reason when
/// a consumer asks for the artifact set, the compile path repairs such an
/// entry instead of failing the build, and `--rebuild` forces the
/// rewrite.
#[test]
fn a_corrupt_artifact_is_a_hard_error_to_load_and_a_repair_to_write() {
    let store = Store::at(scratch("corrupt"));
    let pkg = base_package();
    let profile = TargetProfile::native();
    let ifaces: &[Iface] = &BASE_IFACES;
    let Outcome::Wrote(key) = store.write(pkg, &profile, &[], ifaces, false).unwrap() else {
        panic!("the cold write writes");
    };
    let dir = store.entry_dir(pkg, &key);
    let manifest_path = dir.join(ENTRY_MANIFEST);
    let manifest = std::fs::read_to_string(&manifest_path).expect("the entry manifest");

    // A truncated interface: the consumer gets a hard error naming the
    // module and the escape.
    let victim = &ifaces[3].module;
    let path = Kind::Interface.path_in(&dir, victim);
    let text = std::fs::read_to_string(&path).expect("the stored interface");
    std::fs::write(&path, &text[..text.len() / 2]).expect("truncate");
    let err = store.load(pkg, &key).expect_err("a truncated interface is a hard error");
    let msg = err.to_string();
    assert!(msg.contains("package store"), "{msg}");
    assert!(msg.contains(victim.as_str()), "the reason names the module: {msg}");
    assert!(msg.contains("--rebuild"), "the reason names the escape: {msg}");

    // The compile path does not fail a build over it: it writes the
    // artifacts it just produced, and the entry loads afterwards.
    assert_eq!(
        store.write(pkg, &profile, &[], ifaces, false).unwrap(),
        Outcome::Repaired(key.clone())
    );
    assert_eq!(
        store.load(pkg, &key).expect("the repaired entry loads").ifaces,
        canonical_set(ifaces)
    );

    // A mismatched recorded key.
    let other = format!("{key}0");
    std::fs::write(&manifest_path, manifest.replace(&format!("{key:?}"), &format!("{other:?}")))
        .expect("tamper the key");
    let err = store.load(pkg, &key).expect_err("a mismatched key is a hard error");
    assert!(matches!(err, StoreError::KeyMismatch { .. }), "{err}");
    assert!(err.to_string().contains("records key"), "{err}");
    assert_eq!(
        store.write(pkg, &profile, &[], ifaces, false).unwrap(),
        Outcome::Repaired(key.clone())
    );

    // A recorded hash that does not recompute against the artifact it
    // describes.
    std::fs::write(
        &manifest_path,
        manifest.replace(
            &format!("\"iface_hash\": {:?}", ifaces[0].iface_hash()),
            "\"iface_hash\": \"0\"",
        ),
    )
    .expect("tamper the hash");
    let err = store.load(pkg, &key).expect_err("a recorded hash must recompute");
    assert!(matches!(err, StoreError::HashMismatch { .. }), "{err}");
    assert!(err.to_string().contains(&ifaces[0].module), "{err}");
    assert_eq!(
        store.write(pkg, &profile, &[], ifaces, false).unwrap(),
        Outcome::Repaired(key.clone())
    );

    // A truncated manifest.
    std::fs::write(&manifest_path, "{\"store_schema\": 1, \"package\":").expect("truncate");
    let err = store.load(pkg, &key).expect_err("a truncated manifest is a hard error");
    assert!(matches!(err, StoreError::Corrupt { .. }), "{err}");

    // (d) `--rebuild` rewrites the entry even though the shortcut below
    // would have accepted it, and the entry then verifies.
    let rebuilt = store
        .write(pkg, &profile, &[], ifaces, true)
        .expect("--rebuild recomputes the entry");
    assert_eq!(rebuilt, Outcome::Repaired(key.clone()));
    let loaded = store.load(pkg, &key).expect("the rebuilt entry loads");
    assert_eq!(loaded.ifaces, canonical_set(ifaces));
    assert_eq!(
        store.write(pkg, &profile, &[], ifaces, false).unwrap(),
        Outcome::Verified(key)
    );
}

/// The failure that a store must never cause: an entry written by an
/// earlier build of the compiler whose contract rendering has since
/// changed. The key is unchanged (the sources are), the entry is
/// internally consistent, and what the current compile produces for those
/// sources differs. That is a refresh, never a fatal error, or a store
/// makes ordinary builds fail for reasons unrelated to the build.
#[test]
fn an_entry_from_another_build_is_refreshed_not_fatal() {
    let store = Store::at(scratch("refresh"));
    let pkg = base_package();
    let profile = TargetProfile::native();
    let ifaces: &[Iface] = &BASE_IFACES;

    // An older build's set for the same sources: one module's contract
    // differs (a claim the current checker no longer records), so its
    // recorded contract hash is not the one this compile computes.
    let mut older: Vec<Iface> = BASE_IFACES.clone();
    older[2]
        .assumptions
        .declared
        .push("an older build's claim".to_string());
    assert_ne!(older[2].iface_hash(), ifaces[2].iface_hash());
    assert_eq!(older[2].source_hash, ifaces[2].source_hash);
    let Outcome::Wrote(key) = store.write(pkg, &profile, &[], &older, false).unwrap() else {
        panic!("the cold write writes");
    };

    // The current build must not fail: it refreshes the entry.
    assert_eq!(
        store.write(pkg, &profile, &[], ifaces, false).expect("a refresh, not an error"),
        Outcome::Repaired(key.clone())
    );
    let loaded = store.load(pkg, &key).expect("the refreshed entry loads");
    assert_eq!(loaded.ifaces, canonical_set(ifaces));
}

/// A cold cache is not a fault: the loader says so by name, and nothing
/// is invented for it.
#[test]
fn a_missing_entry_is_reported_as_missing() {
    let store = Store::at(scratch("missing"));
    let err = store
        .load(base_package(), "no-such-key")
        .expect_err("a cold cache is reported, not invented");
    assert!(err.is_missing(), "{err}");
    assert!(!err.rebuild_repairs(), "{err}");
    let err = store
        .load_for(base_package(), &TargetProfile::native(), &[])
        .expect_err("a cold cache is reported, not invented");
    assert!(err.is_missing(), "{err}");
}

// ------------------------------------------------------------ bodies

/// The layout, the entry manifest and the loader carry the body kinds
/// (`object`, `ir`) the store exists for, even though today's compile
/// path has no per-module body producer (codegen is whole program).
#[test]
fn object_and_ir_artifacts_ride_in_the_entry() {
    let store = Store::at(scratch("bodies"));
    let pkg = base_package();
    let profile = TargetProfile::native();
    let ifaces: &[Iface] = &BASE_IFACES;
    let module = ifaces[1].module.clone();
    let bodies = vec![
        BodyArtifact {
            module: module.clone(),
            kind: Kind::Object,
            bytes: b"an object file".to_vec(),
        },
        BodyArtifact {
            module: module.clone(),
            kind: Kind::Ir,
            bytes: b"module Data.Core\n".to_vec(),
        },
    ];
    let outcome = store
        .write_with_bodies(pkg, &profile, &[], ifaces, &bodies, false)
        .expect("a write with bodies");
    let Outcome::Wrote(key) = outcome else {
        panic!("the cold write writes: {outcome:?}");
    };

    let loaded = store.load(pkg, &key).expect("the entry loads");
    assert_eq!(loaded.kinds[&module], vec![Kind::Interface, Kind::Object, Kind::Ir]);
    assert_eq!(
        std::fs::read(loaded.object(&module).expect("an object path")).unwrap(),
        b"an object file"
    );
    assert_eq!(
        std::fs::read(loaded.ir_path(&module).expect("an ir path")).unwrap(),
        b"module Data.Core\n"
    );
    let manifest =
        std::fs::read_to_string(store.entry_dir(pkg, &key).join(ENTRY_MANIFEST)).unwrap();
    assert!(manifest.contains("\"object\""), "{manifest}");
    assert!(manifest.contains("\"ir\""), "{manifest}");

    // A body file that no longer hashes to its record is a hard error to
    // load, exactly like a truncated interface; the compile path repairs
    // it from the artifacts it holds.
    let object = loaded.object(&module).expect("an object path").to_path_buf();
    std::fs::write(&object, b"truncated").expect("truncate the object");
    let err = store.load(pkg, &key).expect_err("a stale body is a hard error");
    assert!(matches!(err, StoreError::HashMismatch { .. }), "{err}");
    assert!(err.to_string().contains("object"), "{err}");
    assert_eq!(
        store
            .write_with_bodies(pkg, &profile, &[], ifaces, &bodies, false)
            .expect("the write repairs the stale body"),
        Outcome::Repaired(key.clone())
    );
    let loaded = store.load(pkg, &key).expect("the repaired entry loads");
    assert_eq!(
        std::fs::read(loaded.object(&module).expect("an object path")).unwrap(),
        b"an object file"
    );

    // An entry that holds bodies is not the entry an interfaces-only
    // compile asked for: the write refreshes it, and the bodies go.
    assert_eq!(
        store
            .write(pkg, &profile, &[], ifaces, false)
            .expect("a refresh, not an error"),
        Outcome::Repaired(key.clone())
    );
    let loaded = store.load(pkg, &key).expect("the refreshed entry loads");
    assert!(loaded.objects.is_empty() && loaded.ir.is_empty());
    assert_eq!(loaded.kinds[&module], vec![Kind::Interface]);

    // `--rebuild` forces a rewrite even when the shortcut would keep the
    // entry, and the bodies come back byte for byte.
    assert_eq!(
        store
            .write_with_bodies(pkg, &profile, &[], ifaces, &bodies, true)
            .expect("--rebuild with bodies"),
        Outcome::Repaired(key.clone())
    );
    let loaded = store.load(pkg, &key).expect("the rewritten entry loads");
    assert_eq!(loaded.kinds[&module], vec![Kind::Interface, Kind::Object, Kind::Ir]);
    assert_eq!(
        std::fs::read(loaded.object(&module).expect("an object path")).unwrap(),
        b"an object file"
    );
    assert_eq!(
        std::fs::read(loaded.ir_path(&module).expect("an ir path")).unwrap(),
        b"module Data.Core\n"
    );
}

// ------------------------------------------------------------- the root

#[test]
fn the_root_follows_cinch_store_and_has_a_documented_default() {
    let dir = scratch("root-env");
    // Only this test reads the process environment's store root, so
    // setting and clearing it here cannot disturb another case.
    std::env::set_var("CINCH_STORE", &dir);
    assert_eq!(store_root(), dir, "CINCH_STORE overrides the location");
    assert_eq!(Store::user().root(), dir);
    std::env::remove_var("CINCH_STORE");
    let fallback = store_root();
    let spelled = fallback.to_string_lossy().to_string();
    assert!(spelled.contains("cinch/store"), "the default is {spelled}");

    let store = Store::at(&dir);
    let entry = store.entry_dir(base_package(), "a-key");
    assert_eq!(
        entry,
        dir.join("cinch-base")
            .join(&base_package().version)
            .join("a-key")
    );
    assert_eq!(store.package_dir(base_package()), dir.join("cinch-base").join(&base_package().version));
}

// ----------------------------------------------------------------- the CLI

/// The CLI never fails a build over a store entry, and `--rebuild` is
/// wired end to end: a truncated artifact is repaired by the next check,
/// and `--rebuild` also succeeds.
#[test]
fn the_cli_repairs_a_corrupt_entry_and_accepts_rebuild() {
    let dir = scratch("cli");
    std::fs::write(
        dir.join("Main.cinch"),
        "module Main where\n  main :: Int\n  main = 0\n",
    )
    .expect("write the project");
    let store_dir = dir.join("store");
    let run = |args: &[&str]| {
        std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
            .args(args)
            .current_dir(&dir)
            .env("CINCH_STORE", &store_dir)
            .env("CINCH_STAGE_MODE", "refresh")
            .output()
            .expect("run the cinch binary")
    };

    let out = run(&["check", "Main.cinch"]);
    assert!(
        out.status.success(),
        "the first check compiles: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let pkg_dir = store_dir.join("cinch-base").join(&base_package().version);
    let keys: Vec<PathBuf> = std::fs::read_dir(&pkg_dir)
        .expect("the compile filed an entry")
        .map(|e| e.expect("entry").path())
        .collect();
    assert_eq!(keys.len(), 1, "one key per package version and input set");
    let victim = std::fs::read_dir(&keys[0])
        .expect("the entry directory")
        .map(|e| e.expect("artifact").path())
        .find(|p| p.to_string_lossy().ends_with(".cinchif"))
        .expect("an interface artifact");
    let text = std::fs::read_to_string(&victim).expect("the stored interface");
    std::fs::write(&victim, &text[..text.len() / 2]).expect("truncate it");

    // The compile path repairs the entry; it must not fail the build.
    let out = run(&["check", "Main.cinch"]);
    assert!(
        out.status.success(),
        "a corrupt entry must not fail a build: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(&victim).expect("the repaired interface"),
        text,
        "the next check rewrites the corrupted artifact"
    );

    let out = run(&["check", "Main.cinch", "--rebuild"]);
    assert!(
        out.status.success(),
        "--rebuild rewrites the entry: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let out = run(&["check", "Main.cinch"]);
    assert!(
        out.status.success(),
        "the entry verifies afterwards: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
