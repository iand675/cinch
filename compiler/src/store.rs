//! The package artifact store: a per-user, content-keyed cache of a
//! package set's COMPILED artifacts, so a warm compiler loads the base
//! library instead of re-parsing and re-checking its sources.
//!
//! Layout, one entry per (package, version, key):
//!
//! ```text
//! <root>/<package>/<version>/<key>/<Module>.cinchif   the interface
//! <root>/<package>/<version>/<key>/<Module>.o         an object file
//! <root>/<package>/<version>/<key>/<Module>.ir        the module's IR form
//! <root>/<package>/<version>/<key>/store.json         the entry manifest
//! ```
//!
//! The root is `CINCH_STORE` when that is set. Otherwise it is a per-user
//! cache directory: `$HOME/Library/Caches/cinch/store` on macOS,
//! `$XDG_CACHE_HOME/cinch/store` when `XDG_CACHE_HOME` is set elsewhere,
//! and `$HOME/.cache/cinch/store` as the remaining fallback (with no
//! `HOME` the fallback is relative to the current directory). One store
//! serves every project on the machine.
//!
//! `store.json` records the key, the package and toolchain identities,
//! every module's contract hash, and which artifact kinds the entry holds
//! per module. Interfaces are stored as their canonical artifact; objects
//! and IR are stored as opaque bytes with their content hash recorded, so
//! a truncated body file is caught exactly like a truncated interface.
//!
//! The two paths have deliberately different policies.
//!
//! The compile path WRITES: it files the artifact set it just produced
//! under its key, or leaves an entry alone when that entry is intact and
//! exactly what this compile produced. It never fails a build over an
//! entry's contents, and it never lets an entry decide whether the build
//! is correct. An entry that is not this compile's set (a cold cache, an
//! aborted write, a truncated artifact, an identity this compiler no
//! longer agrees with, a different kind set) is rewritten from the
//! artifacts in hand. A store that can fail ordinary builds for reasons
//! unrelated to the build in hand is worse than no store, and the base
//! library is edited constantly.
//!
//! The consuming path LOADS ([`Store::load`]) and reports every
//! disagreement as a hard error naming the reason, because a caller
//! about to rely on the artifacts must not receive a set that is not the
//! one the entry describes:
//!
//! * an entry manifest that cannot be read or parsed,
//! * a recorded key, or a recorded store schema, package, version,
//!   interface schema, runtime ABI, language or compiler identity, that
//!   is not the requested or current one,
//! * a module artifact that is absent, unreadable, or not parseable as
//!   its kind claims,
//! * a module artifact that does not hash to what the entry records for
//!   it (the recorded contract hash against the artifact's OWN
//!   recomputation, never against what the current sources would
//!   produce: a source change moves the key, so it is a miss, not a
//!   fault),
//! * interfaces that do not form a consistent "checked against" chain.
//!
//! Nothing is recomputed on the way in, so a caller either gets the
//! artifacts the key names or a named reason. `--rebuild` (see
//! [`crate::driver::DriverOptions`]) rewrites the entry instead of
//! keeping it, which is the escape for a genuinely corrupt entry.
//!
//! A write stages the whole artifact set in a directory of its own and
//! swaps it into place with one rename, so a reader sees the previous
//! set, the new set, or nothing, and never a mixture. That matters
//! because two builds of the compiler render the same sources
//! differently while `CARGO_PKG_VERSION` stays put, so two processes can
//! legitimately hold different artifacts for one key; interleaving them
//! would leave an entry that reads as inconsistent forever.
//!
//! What this stage reaches, and what it does not: the compile path
//! writes entries and repairs them, and [`Store::load`] is a real,
//! tested loader. No compile path calls the loader yet, because a warm
//! compile cannot skip the base library without its bodies: the checker
//! installs the family's checked state (schemes, instances, fixities)
//! and codegen needs the family's functions, and an interface carries
//! rendered contracts, not that state. So the remaining stages are a
//! per-module body producer (core IR first, then objects), a checker and
//! codegen entry point that take a loaded family instead of checking and
//! emitting it, and a key input that distinguishes compiler builds, since
//! the interface schema and `CARGO_PKG_VERSION` do not move during
//! development.

use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::TargetProfile;
use crate::driver::Package;
use crate::interface::{
    self, Iface, JsonValue, WitAdapter, content_hash, content_hash_bytes, parse_json,
};

/// The store's own layout version. A change to the layout or to
/// `store.json`'s shape bumps this, which invalidates every entry
/// through [`StoreError::IdentityMismatch`] instead of relying on
/// readers to tolerate two shapes.
pub const STORE_SCHEMA: u32 = 2;

/// The entry manifest's file name inside an entry directory.
pub const ENTRY_MANIFEST: &str = "store.json";

/// One artifact kind an entry can hold for a module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// The `.cinchif` contract projection.
    Interface,
    /// A per-module object file (the code a link consumes).
    Object,
    /// The module's IR form (what cross-module inlining and rewrite
    /// rules need beyond an interface).
    Ir,
}

impl Kind {
    /// The spelling recorded in `store.json`.
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Interface => "interface",
            Kind::Object => "object",
            Kind::Ir => "ir",
        }
    }

    /// Read back a recorded spelling.
    pub fn parse(s: &str) -> Option<Kind> {
        match s {
            "interface" => Some(Kind::Interface),
            "object" => Some(Kind::Object),
            "ir" => Some(Kind::Ir),
            _ => None,
        }
    }

    /// The artifact's file name in an entry directory. Module names carry
    /// dots (`Data.Bool`), so the kind's extension is appended instead of
    /// replacing a suffix.
    pub fn file_name(self, module: &str) -> String {
        let ext = match self {
            Kind::Interface => "cinchif",
            Kind::Object => "o",
            Kind::Ir => "ir",
        };
        format!("{module}.{ext}")
    }

    /// The artifact's path in an entry directory.
    pub fn path_in(self, dir: &Path, module: &str) -> PathBuf {
        dir.join(self.file_name(module))
    }
}

/// One module's non-interface artifact bytes, when a producer has them.
/// Interfaces travel as parsed [`Iface`] values (they are read back, not
/// copied), so a body is never `Kind::Interface`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyArtifact {
    pub module: String,
    pub kind: Kind,
    pub bytes: Vec<u8>,
}

/// What a write did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// No entry existed for the key; this compile wrote one.
    Wrote(String),
    /// The entry existed and is exactly this compile's artifact set;
    /// nothing changed.
    Verified(String),
    /// The entry existed but was not this compile's artifact set (a
    /// truncated artifact, an aborted write, an identity this compiler
    /// no longer agrees with, a different kind set, or an explicit
    /// `--rebuild`), so it was rewritten from this compile's artifacts.
    /// A repair is never an error: the artifacts written are the ones
    /// this compile just produced, and nothing was consumed.
    Repaired(String),
    /// The compile's interfaces are not the embedded package's, so there
    /// is nothing to key (a source module shadows the package's copy of
    /// the same name, see [`Store::write`]).
    Skipped(String),
}

impl Outcome {
    /// The key the outcome is about.
    pub fn key(&self) -> &str {
        match self {
            Outcome::Wrote(k)
            | Outcome::Verified(k)
            | Outcome::Repaired(k)
            | Outcome::Skipped(k) => k,
        }
    }
}

/// A store fault. Every mismatch is a hard error: a store that repairs
/// itself silently is worse than no store, because a stale artifact then
/// looks like a fresh one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    /// No entry for this package and key. A cold cache, not a fault: the
    /// caller compiles and [`Store::write`]s the result.
    Missing { path: PathBuf },
    /// The entry manifest is not readable as one.
    Corrupt { path: PathBuf, reason: String },
    /// The entry records a different key than the one requested.
    KeyMismatch { recorded: String, requested: String },
    /// A recorded identity (store schema, package, version, interface
    /// schema, runtime ABI, language, compiler) is not the current one.
    IdentityMismatch {
        field: &'static str,
        recorded: String,
        current: String,
    },
    /// A module artifact is absent, unreadable, or not parseable as its
    /// kind claims.
    ArtifactUnreadable {
        module: String,
        kind: Kind,
        path: PathBuf,
        reason: String,
    },
    /// An artifact does not hash to what the entry records, so it is not
    /// the artifact the entry is about.
    HashMismatch {
        module: String,
        kind: Kind,
        recorded: String,
        recomputed: String,
    },
    /// The loaded interfaces do not form a consistent "checked against"
    /// chain.
    ChainBroken { reason: String },
    /// The package cannot be resolved to its module sources, so it cannot
    /// be keyed or written.
    Package { reason: String },
    /// A filesystem operation failed.
    Io {
        path: PathBuf,
        op: &'static str,
        detail: String,
    },
}

impl StoreError {
    /// Whether a cold cache is the whole story (the caller may compile
    /// and write, rather than report a fault).
    pub fn is_missing(&self) -> bool {
        matches!(self, StoreError::Missing { .. })
    }

    /// Whether `--rebuild` repairs this fault by recomputing the entry.
    pub fn rebuild_repairs(&self) -> bool {
        matches!(
            self,
            StoreError::Corrupt { .. }
                | StoreError::KeyMismatch { .. }
                | StoreError::IdentityMismatch { .. }
                | StoreError::ArtifactUnreadable { .. }
                | StoreError::HashMismatch { .. }
                | StoreError::ChainBroken { .. }
        )
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "package store: ")?;
        match self {
            StoreError::Missing { path } => {
                write!(
                    f,
                    "no entry at {} (a cold cache: compile to write one)",
                    path.display()
                )?;
            }
            StoreError::Corrupt { path, reason } => {
                write!(f, "{} is not a readable entry manifest: {reason}", path.display())?;
            }
            StoreError::KeyMismatch { recorded, requested } => {
                write!(f, "the entry records key {recorded}, not the requested {requested}")?;
            }
            StoreError::IdentityMismatch {
                field,
                recorded,
                current,
            } => {
                write!(
                    f,
                    "the entry's {field} identity is {recorded}, this compiler's is {current}"
                )?;
            }
            StoreError::ArtifactUnreadable {
                module,
                kind,
                path,
                reason,
            } => {
                write!(
                    f,
                    "module {module}: the {} artifact at {} is unreadable: {reason}",
                    kind.as_str(),
                    path.display()
                )?;
            }
            StoreError::HashMismatch {
                module,
                kind,
                recorded,
                recomputed,
            } => {
                write!(
                    f,
                    "module {module}: the {} artifact records hash {recorded} but recomputes to {recomputed}",
                    kind.as_str()
                )?;
            }
            StoreError::ChainBroken { reason } => {
                write!(f, "the entry's interfaces do not form a chain: {reason}")?;
            }
            StoreError::Package { reason } => write!(f, "{reason}")?,
            StoreError::Io { path, op, detail } => {
                write!(f, "cannot {op} {}: {detail}", path.display())?;
            }
        }
        if self.rebuild_repairs() {
            write!(f, " (rerun with --rebuild to recompute the entry)")?;
        }
        Ok(())
    }
}

impl std::error::Error for StoreError {}

// ------------------------------------------------------------------ the key

/// The package key: a content hash over this exact input list.
///
/// 1. the package name,
/// 2. the package version,
/// 3. the manifest's bytes (`Package::manifest_hash`, recorded by
///    `Package::parse`),
/// 4. every manifest module in manifest order: its name, its path, and
///    whether it is the facade,
/// 5. every module's source bytes, in the same order (`content_hash` of
///    each source),
/// 6. the interface schema version,
/// 7. the runtime ABI version,
/// 8. the language version,
/// 9. the compiler version,
/// 10. the target profile's identity: name, triple, word size, endian,
///     vector shape, features, fuel, measure, `eval` admission, artifact
///     shape, required capabilities, and the `jpl` flag,
/// 11. the generated WIT adapters' structure, because adapters rewrite
///     the facade source (they append the generated vocabulary), so a
///     compile of the same package with different adapters is a
///     different compilation.
///
/// Every one of these is pinned by `tests/package_store.rs`: an input
/// missing from this list is the failure mode that makes a store worse
/// than no store.
///
/// The key resolves the package's embedded sources, so it fails for a
/// package whose sources are not embedded; [`package_key_for`] takes the
/// resolved list instead.
pub fn package_key(
    pkg: &Package,
    profile: &TargetProfile,
    adapters: &[WitAdapter],
) -> Result<String, StoreError> {
    let sources = pkg
        .ordered_sources()
        .map_err(|e| StoreError::Package {
            reason: e.to_string(),
        })?;
    let sources: Vec<(&str, &str)> = sources;
    Ok(package_key_for(pkg, &sources, profile, adapters))
}

/// [`package_key`] over an explicit `(path, source)` list in manifest
/// order: the form a package whose sources live in the store's own
/// consumer (or a test pinning the source-bytes input) keys with. The
/// caller is responsible for the list being the manifest's order and
/// contents; [`package_key`] resolves it for an embedded package.
pub fn package_key_for(
    pkg: &Package,
    sources: &[(&str, &str)],
    profile: &TargetProfile,
    adapters: &[WitAdapter],
) -> String {
    // Tabs separate fields and newlines separate records; no input here
    // is a hash or a rendered structure that can contain either (module
    // names, paths and imports are single-line identifiers, and the
    // adapter rendering is a one-line `Debug`), so the encoding is
    // unambiguous.
    let mut text = String::new();
    fn line(t: &mut String, k: &str, v: &str) {
        t.push_str(k);
        t.push('\t');
        t.push_str(v);
        t.push('\n');
    }
    line(&mut text, "package", &pkg.name);
    line(&mut text, "version", &pkg.version);
    line(&mut text, "manifest", &pkg.manifest_hash);
    line(&mut text, "iface_schema", &interface::SCHEMA_VERSION.to_string());
    line(&mut text, "runtime_abi", &interface::RUNTIME_ABI_VERSION.to_string());
    line(&mut text, "language", interface::LANGUAGE_VERSION);
    line(&mut text, "compiler", env!("CARGO_PKG_VERSION"));
    line(&mut text, "profile", &profile_identity(profile));
    line(&mut text, "adapters", &format!("{adapters:?}"));
    for name in &pkg.order {
        match pkg.modules.iter().find(|m| &m.name == name) {
            Some(m) => line(
                &mut text,
                "module",
                &format!(
                    "{}\t{}\t{}",
                    m.name,
                    m.path,
                    if m.facade { "facade" } else { "module" }
                ),
            ),
            // An `order` entry with no module is a broken manifest; the
            // key records the name alone so two such manifests differ.
            None => line(&mut text, "module", name),
        }
    }
    for (path, src) in sources {
        line(&mut text, "source", &format!("{path}\t{}", content_hash(src)));
    }
    content_hash(&text)
}

/// The target profile's identity, one field per input (see
/// [`package_key`]'s list). The enums are rendered by their `Debug`
/// spelling; the compiler version in the key keeps that honest across
/// renames.
fn profile_identity(p: &TargetProfile) -> String {
    format!(
        "{}\t{}\t{}\t{}\t{:?}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        p.name,
        p.triple,
        p.word_size,
        p.endian,
        p.vector_shape,
        p.features.join(","),
        p.fuel,
        p.measure,
        p.allow_eval,
        p.artifact.spelling(),
        p.required.join(","),
        p.jpl
    )
}

// ----------------------------------------------------------------- the store

/// A package store rooted at one directory.
#[derive(Clone, Debug)]
pub struct Store {
    root: PathBuf,
}

impl Store {
    /// The per-user store: `CINCH_STORE` when set, else the platform
    /// cache directory (see the module docs for the default).
    pub fn user() -> Store {
        Store { root: store_root() }
    }

    /// A store rooted at `root`, for a caller that chooses its own
    /// location (tests, and any tool that keeps a project-local store).
    pub fn at(root: impl Into<PathBuf>) -> Store {
        Store { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// `<root>/<package>/<version>`: every key of one package version.
    pub fn package_dir(&self, pkg: &Package) -> PathBuf {
        self.root.join(&pkg.name).join(&pkg.version)
    }

    /// `<root>/<package>/<version>/<key>`: one entry.
    pub fn entry_dir(&self, pkg: &Package, key: &str) -> PathBuf {
        self.package_dir(pkg).join(key)
    }

    /// Create the store root. The compile path treats a root that cannot
    /// be created as "no store": the store is a cache, never a compile
    /// dependency, and a read-only cache directory must not break a
    /// build. Every other failure stays a hard error.
    pub fn ensure_root(&self) -> Result<(), StoreError> {
        std::fs::create_dir_all(&self.root).map_err(|e| StoreError::Io {
            path: self.root.clone(),
            op: "create the store root",
            detail: e.to_string(),
        })
    }

    /// Write the package's interface artifact set: what the compile path
    /// calls with the interfaces it just produced. The write never fails
    /// over the entry's contents (see the module docs): an entry that is
    /// not this compile's set is rewritten.
    pub fn write(
        &self,
        pkg: &Package,
        profile: &TargetProfile,
        adapters: &[WitAdapter],
        ifaces: &[Iface],
        rebuild: bool,
    ) -> Result<Outcome, StoreError> {
        self.write_with_bodies(pkg, profile, adapters, ifaces, &[], rebuild)
    }

    /// [`Store::write`] with the modules' object and IR artifacts. No
    /// producer of per-module bodies exists yet (codegen is whole
    /// program and the incremental session's object cache is keyed by
    /// declaration, not by package), so today's compile path passes
    /// none; the layout, the entry manifest and the loader all carry
    /// them, and the test drives this form with real payloads.
    ///
    /// `rebuild` forces the rewrite: without it an entry that is intact
    /// and exactly this compile's artifact set is left alone.
    pub fn write_with_bodies(
        &self,
        pkg: &Package,
        profile: &TargetProfile,
        adapters: &[WitAdapter],
        ifaces: &[Iface],
        bodies: &[BodyArtifact],
        rebuild: bool,
    ) -> Result<Outcome, StoreError> {
        let sources = pkg.ordered_sources().map_err(|e| StoreError::Package {
            reason: e.to_string(),
        })?;
        let key = package_key_for(pkg, &sources, profile, adapters);
        // Only the package's own modules go in the entry, in the
        // manifest's check order. A module whose interface does not carry
        // the embedded source's hash is a source module shadowing the
        // package's copy (the compiler accepts that), so this compile's
        // interfaces are not the package's and the entry would not be a
        // function of the key; nothing is written.
        let mut selected: Vec<&Iface> = Vec::new();
        for (name, (_path, src)) in pkg.order.iter().zip(sources.iter()) {
            let Some(iface) = ifaces.iter().find(|i| &i.module == name) else {
                return Ok(Outcome::Skipped(key));
            };
            if iface.source_hash != content_hash(src) {
                return Ok(Outcome::Skipped(key));
            }
            selected.push(iface);
        }
        let expected = expected_records(&selected, bodies)?;
        let dir = self.entry_dir(pkg, &key);
        let manifest_path = dir.join(ENTRY_MANIFEST);
        // The compile path WRITES the store; it never trusts an entry to
        // decide whether to fail. An entry is kept only when it is both
        // intact and exactly the set this compile produced, and anything
        // else (a cold cache, an aborted write, a truncated artifact, a
        // recorded identity the current compiler no longer agrees with)
        // is repaired by writing the artifacts this compile just
        // produced. A store that could fail an ordinary build is worse
        // than no store: the prelude is edited constantly, so the entry
        // under one key goes stale for reasons that have nothing to do
        // with the build in hand.
        //
        // The hard errors a genuinely corrupt entry produces belong to
        // [`Store::load`], the consuming path, where the reason is
        // reported to whoever is about to rely on the artifacts.
        if !rebuild {
            if let Ok(loaded) = self.load(pkg, &key) {
                if self.agrees_with_compile(&loaded, &selected, &expected) {
                    return Ok(Outcome::Verified(key));
                }
            }
        }
        let prior_entry = manifest_path.exists();
        // Write this compile's set into a directory of this writer's own
        // and swap it in with a single rename. Two writers of one key
        // (two builds of the compiler render the same sources
        // differently, and `CARGO_PKG_VERSION` does not move between
        // them) must never interleave their artifacts: a directory that
        // holds half of one build's set and half of another's reads as an
        // inconsistent entry forever, which is exactly the state that
        // made a store worse than no store.
        let parent = self.package_dir(pkg);
        std::fs::create_dir_all(&parent).map_err(|e| StoreError::Io {
            path: parent.clone(),
            op: "create the package directory",
            detail: e.to_string(),
        })?;
        let staged = parent.join(format!(
            ".{key}.{}.{}.staged",
            std::process::id(),
            next_sequence()
        ));
        let _ = std::fs::remove_dir_all(&staged);
        std::fs::create_dir_all(&staged).map_err(|e| StoreError::Io {
            path: staged.clone(),
            op: "create the staged entry directory",
            detail: e.to_string(),
        })?;
        for iface in &selected {
            write_file(
                &Kind::Interface.path_in(&staged, &iface.module),
                iface.to_json().as_bytes(),
            )?;
        }
        for body in bodies {
            write_file(&body.kind.path_in(&staged, &body.module), &body.bytes)?;
        }
        let text = entry_manifest(pkg, &key, profile, &expected);
        write_file(&staged.join(ENTRY_MANIFEST), text.as_bytes())?;
        if dir.exists() {
            std::fs::remove_dir_all(&dir).map_err(|e| StoreError::Io {
                path: dir.clone(),
                op: "clear the entry directory",
                detail: e.to_string(),
            })?;
        }
        std::fs::rename(&staged, &dir).map_err(|e| {
            let _ = std::fs::remove_dir_all(&staged);
            StoreError::Io {
                path: dir.clone(),
                op: "install the entry directory",
                detail: e.to_string(),
            }
        })?;
        Ok(if prior_entry {
            Outcome::Repaired(key)
        } else {
            Outcome::Wrote(key)
        })
    }

    /// Load one entry: the interfaces for every module, and the paths of
    /// the object and IR artifacts it holds. Every disagreement is a
    /// [`StoreError`]; nothing is recomputed here, so a caller either
    /// gets the artifacts the key names or a named reason.
    ///
    /// No compile path calls this yet: today's compile re-parses and
    /// re-checks the base library every time. A warm compile cannot call
    /// it until the entry also holds the family's bodies and until the
    /// checker and codegen can take a loaded family instead of checking
    /// and emitting one (see the module docs). The loader is here so that
    /// stage is a wiring change, and it is tested against real interface
    /// artifacts.
    pub fn load(&self, pkg: &Package, key: &str) -> Result<PackageArtifacts, StoreError> {
        let dir = self.entry_dir(pkg, key);
        let manifest_path = dir.join(ENTRY_MANIFEST);
        if !manifest_path.exists() {
            return Err(StoreError::Missing { path: dir });
        }
        let text = match std::fs::read_to_string(&manifest_path) {
            Ok(text) => text,
            // A concurrent `--rebuild` may have cleared the directory
            // between the check above and this read; that is a cold
            // cache, not a fault.
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(StoreError::Missing { path: dir });
            }
            Err(e) => {
                return Err(StoreError::Corrupt {
                    path: manifest_path.clone(),
                    reason: format!("cannot be read: {e}"),
                });
            }
        };
        let recorded = parse_entry_manifest(&text, &manifest_path)?;
        if recorded.store_schema != STORE_SCHEMA as u64 {
            return Err(StoreError::IdentityMismatch {
                field: "store schema",
                recorded: recorded.store_schema.to_string(),
                current: STORE_SCHEMA.to_string(),
            });
        }
        if recorded.package != pkg.name {
            return Err(StoreError::IdentityMismatch {
                field: "package",
                recorded: recorded.package,
                current: pkg.name.clone(),
            });
        }
        if recorded.version != pkg.version {
            return Err(StoreError::IdentityMismatch {
                field: "version",
                recorded: recorded.version,
                current: pkg.version.clone(),
            });
        }
        if recorded.key != key {
            return Err(StoreError::KeyMismatch {
                recorded: recorded.key,
                requested: key.to_string(),
            });
        }
        check_identity("interface schema", recorded.iface_schema, interface::SCHEMA_VERSION as u64)?;
        check_identity("runtime ABI", recorded.runtime_abi, interface::RUNTIME_ABI_VERSION as u64)?;
        if recorded.language != interface::LANGUAGE_VERSION {
            return Err(StoreError::IdentityMismatch {
                field: "language",
                recorded: recorded.language,
                current: interface::LANGUAGE_VERSION.to_string(),
            });
        }
        if recorded.compiler != env!("CARGO_PKG_VERSION") {
            return Err(StoreError::IdentityMismatch {
                field: "compiler",
                recorded: recorded.compiler,
                current: env!("CARGO_PKG_VERSION").to_string(),
            });
        }
        let mut ifaces = Vec::new();
        let mut objects = Vec::new();
        let mut ir = Vec::new();
        let mut kinds = BTreeMap::new();
        for m in &recorded.modules {
            kinds.insert(m.module.clone(), m.kinds.clone());
            for kind in &m.kinds {
                match kind {
                    Kind::Interface => {
                        let path = Kind::Interface.path_in(&dir, &m.module);
                        let text = std::fs::read_to_string(&path).map_err(|e| {
                            StoreError::ArtifactUnreadable {
                                module: m.module.clone(),
                                kind: Kind::Interface,
                                path: path.clone(),
                                reason: e.to_string(),
                            }
                        })?;
                        let iface = Iface::from_json(&text).map_err(|e| {
                            StoreError::ArtifactUnreadable {
                                module: m.module.clone(),
                                kind: Kind::Interface,
                                path,
                                reason: e.to_string(),
                            }
                        })?;
                        if iface.module != m.module {
                            return Err(StoreError::IdentityMismatch {
                                field: "module",
                                recorded: m.module.clone(),
                                current: iface.module,
                            });
                        }
                        if iface.language != interface::LANGUAGE_VERSION {
                            return Err(StoreError::IdentityMismatch {
                                field: "language",
                                recorded: iface.language,
                                current: interface::LANGUAGE_VERSION.to_string(),
                            });
                        }
                        if iface.runtime_abi != interface::RUNTIME_ABI_VERSION {
                            return Err(StoreError::IdentityMismatch {
                                field: "runtime ABI",
                                recorded: iface.runtime_abi.to_string(),
                                current: interface::RUNTIME_ABI_VERSION.to_string(),
                            });
                        }
                        if iface.compiler != env!("CARGO_PKG_VERSION") {
                            return Err(StoreError::IdentityMismatch {
                                field: "compiler",
                                recorded: iface.compiler,
                                current: env!("CARGO_PKG_VERSION").to_string(),
                            });
                        }
                        let recomputed = iface.iface_hash();
                        if recomputed != m.iface_hash {
                            return Err(StoreError::HashMismatch {
                                module: m.module.clone(),
                                kind: Kind::Interface,
                                recorded: m.iface_hash.clone(),
                                recomputed,
                            });
                        }
                        ifaces.push(iface);
                    }
                    Kind::Object | Kind::Ir => {
                        let path = kind.path_in(&dir, &m.module);
                        let bytes = std::fs::read(&path).map_err(|e| {
                            StoreError::ArtifactUnreadable {
                                module: m.module.clone(),
                                kind: *kind,
                                path: path.clone(),
                                reason: e.to_string(),
                            }
                        })?;
                        let Some((_, recorded_hash)) =
                            m.bodies.iter().find(|(k, _)| k == kind)
                        else {
                            return Err(StoreError::Corrupt {
                                path: manifest_path.clone(),
                                reason: format!(
                                    "module {} records a {} artifact with no hash",
                                    m.module,
                                    kind.as_str()
                                ),
                            });
                        };
                        let recomputed = content_hash_bytes(&bytes);
                        if &recomputed != recorded_hash {
                            return Err(StoreError::HashMismatch {
                                module: m.module.clone(),
                                kind: *kind,
                                recorded: recorded_hash.clone(),
                                recomputed,
                            });
                        }
                        match kind {
                            Kind::Object => objects.push((m.module.clone(), path)),
                            _ => ir.push((m.module.clone(), path)),
                        }
                    }
                }
            }
        }
        // The set is closed over its imports (design note section 2.2):
        // a loaded set that is missing an import, or that found two
        // artifacts for one module identity, is not the set the key
        // names.
        interface::verify_chain(&ifaces).map_err(|e| StoreError::ChainBroken {
            reason: e.to_string(),
        })?;
        Ok(PackageArtifacts {
            key: key.to_string(),
            package: recorded.package,
            version: recorded.version,
            dir,
            ifaces,
            objects,
            ir,
            kinds,
        })
    }

    /// [`Store::load`] for the key `pkg`, `profile` and `adapters`
    /// produce: the call a compile path makes when it wants to load the
    /// package instead of checking it.
    pub fn load_for(
        &self,
        pkg: &Package,
        profile: &TargetProfile,
        adapters: &[WitAdapter],
    ) -> Result<PackageArtifacts, StoreError> {
        let key = package_key(pkg, profile, adapters)?;
        self.load(pkg, &key)
    }

    /// Check an existing entry against the artifact set a compile just
    /// produced. The entry has already verified itself in
    /// [`Store::load`]; this is the additional comparison that makes the
    /// entry the same compilation, so a recompile that renders contracts
    /// differently under an unchanged key refreshes the entry instead of
    /// leaving artifacts behind that no longer match the compiler.
    fn agrees_with_compile(
        &self,
        loaded: &PackageArtifacts,
        selected: &[&Iface],
        expected: &[ModuleRecord],
    ) -> bool {
        // The entry has to be about exactly the modules this compile
        // produced, not merely about ones that agree where they overlap.
        // The recorded order is the manifest's check order, not this
        // map's sorted one, so membership is what is compared.
        if loaded.kinds.len() != selected.len() {
            return false;
        }
        for iface in selected {
            if !loaded.kinds.contains_key(&iface.module) {
                return false;
            }
        }
        for record in expected {
            let want: Vec<&str> = record.kinds.iter().map(|k| k.as_str()).collect();
            let recorded: Vec<&str> = loaded
                .kinds
                .get(&record.module)
                .map(|ks| ks.iter().map(|k| k.as_str()).collect())
                .unwrap_or_default();
            if want != recorded {
                return false;
            }
        }
        for iface in selected {
            let Some(prior) = loaded.iface(&iface.module) else {
                return false;
            };
            if prior.iface_hash() != iface.iface_hash() || prior.source_hash != iface.source_hash {
                return false;
            }
        }
        true
    }
}

/// A loaded entry: the interfaces, and the paths of the body artifacts
/// the entry holds.
#[derive(Clone, Debug)]
pub struct PackageArtifacts {
    /// The key this entry is filed under.
    pub key: String,
    pub package: String,
    pub version: String,
    /// The entry directory.
    pub dir: PathBuf,
    /// Every module's interface, in the entry manifest's order (the
    /// package's check order).
    pub ifaces: Vec<Iface>,
    /// `(module, path)` for every object artifact in the entry.
    pub objects: Vec<(String, PathBuf)>,
    /// `(module, path)` for every IR artifact in the entry.
    pub ir: Vec<(String, PathBuf)>,
    /// The artifact kinds recorded per module.
    pub kinds: BTreeMap<String, Vec<Kind>>,
}

impl PackageArtifacts {
    /// One module's interface.
    pub fn iface(&self, module: &str) -> Option<&Iface> {
        self.ifaces.iter().find(|i| i.module == module)
    }

    /// One module's object artifact path.
    pub fn object(&self, module: &str) -> Option<&Path> {
        self.objects
            .iter()
            .find(|(m, _)| m == module)
            .map(|(_, p)| p.as_path())
    }

    /// One module's IR artifact path.
    pub fn ir_path(&self, module: &str) -> Option<&Path> {
        self.ir.iter().find(|(m, _)| m == module).map(|(_, p)| p.as_path())
    }
}

/// One module's record in the entry manifest: its contract identity and
/// the artifact kinds the entry holds for it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct ModuleRecord {
    module: String,
    iface_hash: String,
    kinds: Vec<Kind>,
    /// The content hash of each body artifact, so a truncated object or
    /// IR file is caught exactly like a truncated interface.
    bodies: Vec<(Kind, String)>,
}

/// The entry manifest as read back.
#[derive(Clone, Debug)]
struct Entry {
    store_schema: u64,
    package: String,
    version: String,
    key: String,
    compiler: String,
    iface_schema: u64,
    runtime_abi: u64,
    language: String,
    modules: Vec<ModuleRecord>,
}

/// The records a write will put in the entry manifest: the interfaces in
/// manifest order, each with the body artifacts offered for it.
fn expected_records(
    selected: &[&Iface],
    bodies: &[BodyArtifact],
) -> Result<Vec<ModuleRecord>, StoreError> {
    let mut records = Vec::new();
    let mut consumed: Vec<&str> = Vec::new();
    for iface in selected {
        let mut kinds = vec![Kind::Interface];
        let mut hashes = Vec::new();
        for body in bodies.iter().filter(|b| b.module == iface.module) {
            consumed.push(&body.module);
            if body.kind == Kind::Interface {
                return Err(StoreError::Package {
                    reason: format!(
                        "an interface artifact for module {} was offered as a body",
                        body.module
                    ),
                });
            }
            kinds.push(body.kind);
            hashes.push((body.kind, content_hash_bytes(&body.bytes)));
        }
        kinds.sort();
        kinds.dedup();
        hashes.sort();
        records.push(ModuleRecord {
            module: iface.module.clone(),
            iface_hash: iface.iface_hash(),
            kinds,
            bodies: hashes,
        });
    }
    // A body artifact belongs beside the interface of its module; one
    // for a module the compile did not produce an interface for has no
    // place in the entry, and dropping it silently would cache a body
    // nothing can find.
    for body in bodies {
        if !consumed.iter().any(|m| *m == body.module) {
            return Err(StoreError::Package {
                reason: format!(
                    "a {} artifact is offered for module {}, which this compile has no interface for",
                    body.kind.as_str(),
                    body.module
                ),
            });
        }
    }
    Ok(records)
}

/// The entry manifest's canonical text (the canonical JSON form every
/// compiler artifact uses), with no timestamps, so two writes of one key
/// are byte-identical.
fn entry_manifest(
    pkg: &Package,
    key: &str,
    profile: &TargetProfile,
    modules: &[ModuleRecord],
) -> String {
    let mut union: Vec<Kind> = modules.iter().flat_map(|m| m.kinds.iter().copied()).collect();
    union.sort();
    union.dedup();
    JsonValue::obj([
        ("store_schema", STORE_SCHEMA.into()),
        ("package", (&pkg.name).into()),
        ("version", (&pkg.version).into()),
        ("key", key.into()),
        ("compiler", env!("CARGO_PKG_VERSION").into()),
        ("iface_schema", interface::SCHEMA_VERSION.into()),
        ("runtime_abi", interface::RUNTIME_ABI_VERSION.into()),
        ("language", interface::LANGUAGE_VERSION.into()),
        ("profile", (&profile.name).into()),
        ("kinds", kind_list(&union)),
        (
            "modules",
            JsonValue::arr(modules, |m| {
                JsonValue::obj([
                    ("module", (&m.module).into()),
                    ("iface_hash", (&m.iface_hash).into()),
                    ("kinds", kind_list(&m.kinds)),
                    (
                        "bodies",
                        JsonValue::arr(&m.bodies, |(k, h)| {
                            JsonValue::obj([("kind", k.as_str().into()), ("hash", h.into())])
                        }),
                    ),
                ])
            }),
        ),
    ])
    .to_canonical_string()
}

fn kind_list(kinds: &[Kind]) -> JsonValue {
    JsonValue::arr(kinds, |k| k.as_str().into())
}

fn check_identity(field: &'static str, recorded: u64, current: u64) -> Result<(), StoreError> {
    if recorded == current {
        return Ok(());
    }
    Err(StoreError::IdentityMismatch {
        field,
        recorded: recorded.to_string(),
        current: current.to_string(),
    })
}

/// Parse an entry manifest. A shape the reader does not recognize is
/// [`StoreError::Corrupt`] with the field named: a half-written or
/// hand-edited manifest must not read as an empty entry.
fn parse_entry_manifest(text: &str, path: &Path) -> Result<Entry, StoreError> {
    let corrupt = |reason: String| StoreError::Corrupt {
        path: path.to_path_buf(),
        reason,
    };
    let doc = parse_json(text).map_err(|e| corrupt(e.to_string()))?;
    let obj = doc.as_object().map_err(|e| corrupt(e.to_string()))?;
    let jstr = |k: &str| -> Result<String, StoreError> {
        obj.get(k)
            .ok_or_else(|| corrupt(format!("missing field `{k}`")))?
            .as_str()
            .map(|s| s.to_string())
            .map_err(|e| corrupt(format!("field `{k}`: {e}")))
    };
    let jnum = |k: &str| -> Result<u64, StoreError> {
        obj.get(k)
            .ok_or_else(|| corrupt(format!("missing field `{k}`")))?
            .as_num()
            .map_err(|e| corrupt(format!("field `{k}`: {e}")))
    };
    let store_schema = jnum("store_schema")?;
    let package = jstr("package")?;
    let version = jstr("version")?;
    let key = jstr("key")?;
    let compiler = jstr("compiler")?;
    let iface_schema = jnum("iface_schema")?;
    let runtime_abi = jnum("runtime_abi")?;
    let language = jstr("language")?;
    let list = obj
        .get("modules")
        .ok_or_else(|| corrupt("missing field `modules`".to_string()))?
        .as_array()
        .map_err(|e| corrupt(format!("field `modules`: {e}")))?;
    let mut modules = Vec::new();
    for (i, item) in list.iter().enumerate() {
        let o = item
            .as_object()
            .map_err(|e| corrupt(format!("modules[{i}]: {e}")))?;
        let field = |k: &str| -> Result<&JsonValue, StoreError> {
            o.get(k)
                .ok_or_else(|| corrupt(format!("modules[{i}]: missing field `{k}`")))
        };
        let module = field("module")?
            .as_str()
            .map_err(|e| corrupt(format!("modules[{i}]: {e}")))?
            .to_string();
        let iface_hash = field("iface_hash")?
            .as_str()
            .map_err(|e| corrupt(format!("modules[{i}]: {e}")))?
            .to_string();
        let mut kinds = Vec::new();
        for k in field("kinds")?
            .as_array()
            .map_err(|e| corrupt(format!("modules[{i}].kinds: {e}")))?
        {
            let name = k
                .as_str()
                .map_err(|e| corrupt(format!("modules[{i}].kinds: {e}")))?;
            let kind = Kind::parse(name).ok_or_else(|| {
                corrupt(format!("modules[{i}]: unknown artifact kind `{name}`"))
            })?;
            kinds.push(kind);
        }
        let mut bodies = Vec::new();
        if let Some(JsonValue::Arr(items)) = o.get("bodies") {
            for b in items {
                let bo = b
                    .as_object()
                    .map_err(|e| corrupt(format!("modules[{i}].bodies: {e}")))?;
                let name = bo
                    .get("kind")
                    .ok_or_else(|| corrupt(format!("modules[{i}].bodies: missing `kind`")))?
                    .as_str()
                    .map_err(|e| corrupt(format!("modules[{i}].bodies: {e}")))?;
                let kind = Kind::parse(name).ok_or_else(|| {
                    corrupt(format!("modules[{i}]: unknown artifact kind `{name}`"))
                })?;
                let hash = bo
                    .get("hash")
                    .ok_or_else(|| corrupt(format!("modules[{i}].bodies: missing `hash`")))?
                    .as_str()
                    .map_err(|e| corrupt(format!("modules[{i}].bodies: {e}")))?
                    .to_string();
                bodies.push((kind, hash));
            }
        }
        modules.push(ModuleRecord {
            module,
            iface_hash,
            kinds,
            bodies,
        });
    }
    Ok(Entry {
        store_schema,
        package,
        version,
        key,
        compiler,
        iface_schema,
        runtime_abi,
        language,
        modules,
    })
}

/// A process-wide counter for staged entry directory names, so two
/// writers in one process never share one.
fn next_sequence() -> u64 {
    static SEQ: AtomicU64 = AtomicU64::new(0);
    SEQ.fetch_add(1, Ordering::Relaxed)
}

/// Write one file of a staged entry. The staged directory belongs to this
/// writer alone and is swapped into place by the caller, so the swap is
/// what makes the entry appear and no per-file dance is needed.
fn write_file(path: &Path, bytes: &[u8]) -> Result<(), StoreError> {
    std::fs::write(path, bytes).map_err(|e| StoreError::Io {
        path: path.to_path_buf(),
        op: "write",
        detail: e.to_string(),
    })
}

/// The per-user store root: `CINCH_STORE` when set, else the platform's
/// cache directory (see the module docs).
pub fn store_root() -> PathBuf {
    if let Some(v) = std::env::var_os("CINCH_STORE") {
        return PathBuf::from(v);
    }
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_default();
    if cfg!(target_os = "macos") {
        return home.join("Library/Caches/cinch/store");
    }
    if let Some(x) = std::env::var_os("XDG_CACHE_HOME") {
        return PathBuf::from(x).join("cinch/store");
    }
    home.join(".cache/cinch/store")
}
