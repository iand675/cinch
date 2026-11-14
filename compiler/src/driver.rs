//! Command driver: source file in, diagnostics or native binary out.

use crate::parser::{parse_modules, FixityTable, ParsedModules};
use crate::{check, codegen, layout, lexer, Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// P29: Blaze-style incremental recompilation (the REAPER item graph,
/// the INCR persistent session, and the LINK object cache). Declared
/// here, as a child of the driver, so the session reaches the driver's
/// internals (`load_project`, `write_artifacts`) through `super::`
/// without widening their visibility.
#[path = "incremental.rs"]
pub mod incremental;

/// The query graph (design/compilation-speed.md section 3): demand-driven
/// queries with recorded dependencies, early cutoff by fingerprint,
/// negative lookups as dependencies, and interface-first pipelining.
#[path = "query.rs"]
pub mod query;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Check,
    EmitIr,
    EmitObject,
    Build,
    Run,
}

/// The prelude facade: the curated re-export surface (the prelude
/// split's implicit base layer). Its fixity table is the base table
/// every module parses against.
pub const PRELUDE_FACADE: &str = include_str!("../prelude/Prelude.cinch");

/// The base package's manifest: module names, source paths, imports,
/// the facade flag and the check ORDER (design/base-library.md,
/// "Packages and the manifest"). Resolution order is data, so no order
/// is written into this code; [`EMBEDDED_SOURCES`] carries the bytes
/// (`include_str!`, so the family stays embedded and hash-stable per
/// D161-D174) keyed by the manifest's paths, and is deliberately
/// UNORDERED.
pub const BASE_PACKAGE: &str = include_str!("../prelude/cinch-base.cinchpkg");

/// Every embedded prelude source, keyed by its manifest path. The
/// manifest's `order` is the truth about the order; this table only
/// answers "what are the bytes of this path".
pub const EMBEDDED_SOURCES: &[(&str, &str)] = &[
    // The facade is listed here too, so the manifest's paths and this
    // table agree name for name; `parse_prelude` parses the facade from
    // `PRELUDE_FACADE` (with the generated vocabulary appended) rather
    // than through this entry.
    ("Prelude.cinch", PRELUDE_FACADE),
    (
        "Data/Core.cinch",
        include_str!("../prelude/Data/Core.cinch"),
    ),
    (
        "Relation/Eq.cinch",
        include_str!("../prelude/Relation/Eq.cinch"),
    ),
    (
        "Relation/Ord.cinch",
        include_str!("../prelude/Relation/Ord.cinch"),
    ),
    (
        "Algebra/Arithmetic.cinch",
        include_str!("../prelude/Algebra/Arithmetic.cinch"),
    ),
    (
        "Algebra/Bits.cinch",
        include_str!("../prelude/Algebra/Bits.cinch"),
    ),
    (
        "Function/Combinators.cinch",
        include_str!("../prelude/Function/Combinators.cinch"),
    ),
    ("Data/Nat.cinch", include_str!("../prelude/Data/Nat.cinch")),
    ("Data/Int.cinch", include_str!("../prelude/Data/Int.cinch")),
    (
        "Data/Option.cinch",
        include_str!("../prelude/Data/Option.cinch"),
    ),
    (
        "Data/Result.cinch",
        include_str!("../prelude/Data/Result.cinch"),
    ),
    (
        "Data/List.cinch",
        include_str!("../prelude/Data/List.cinch"),
    ),
    // The enumeration class behind the range literals collects after
    // `Data.List` (its `List` type); its instances live with their
    // types in `Data.Nat`/`Data.Int`.
    (
        "Data/Enum.cinch",
        include_str!("../prelude/Data/Enum.cinch"),
    ),
    (
        "Data/Ordering.cinch",
        include_str!("../prelude/Data/Ordering.cinch"),
    ),
    (
        "Data/Char.cinch",
        include_str!("../prelude/Data/Char.cinch"),
    ),
    (
        "Data/Bool.cinch",
        include_str!("../prelude/Data/Bool.cinch"),
    ),
    (
        "Data/Machine.cinch",
        include_str!("../prelude/Data/Machine.cinch"),
    ),
    (
        "Data/Bytes.cinch",
        include_str!("../prelude/Data/Bytes.cinch"),
    ),
    ("Data/Map.cinch", include_str!("../prelude/Data/Map.cinch")),
    (
        "Data/String.cinch",
        include_str!("../prelude/Data/String.cinch"),
    ),
    (
        "Data/Json.cinch",
        include_str!("../prelude/Data/Json.cinch"),
    ),
    (
        "Data/Producer.cinch",
        include_str!("../prelude/Data/Producer.cinch"),
    ),
    (
        "Data/Target.cinch",
        include_str!("../prelude/Data/Target.cinch"),
    ),
    (
        "Data/Path.cinch",
        include_str!("../prelude/Data/Path.cinch"),
    ),
    (
        "Effect/Sequencing.cinch",
        include_str!("../prelude/Effect/Sequencing.cinch"),
    ),
    (
        "Effect/Concurrent.cinch",
        include_str!("../prelude/Effect/Concurrent.cinch"),
    ),
    (
        "Effect/Clock.cinch",
        include_str!("../prelude/Effect/Clock.cinch"),
    ),
    (
        "Prim/Sys/Darwin.cinch",
        include_str!("../prelude/Prim/Sys/Darwin.cinch"),
    ),
    (
        "Prim/Sys/Linux.cinch",
        include_str!("../prelude/Prim/Sys/Linux.cinch"),
    ),
    (
        "Prim/Sys/Portable.cinch",
        include_str!("../prelude/Prim/Sys/Portable.cinch"),
    ),
    (
        "Prim/Sys.cinch",
        include_str!("../prelude/Prim/Sys.cinch"),
    ),
    (
        "Effect/Fs.cinch",
        include_str!("../prelude/Effect/Fs.cinch"),
    ),
    (
        "Effect/Process.cinch",
        include_str!("../prelude/Effect/Process.cinch"),
    ),
    (
        "Effect/Io.cinch",
        include_str!("../prelude/Effect/Io.cinch"),
    ),
    (
        "Data/FromList.cinch",
        include_str!("../prelude/Data/FromList.cinch"),
    ),
    (
        "Data/FromMap.cinch",
        include_str!("../prelude/Data/FromMap.cinch"),
    ),
    (
        "Text/Show.cinch",
        include_str!("../prelude/Text/Show.cinch"),
    ),
    // `Data.Sym` collects after `Text.Show` (its `Show` instance) and
    // `Data.String` (its name conversions); the facade collects after
    // the whole family.
    ("Data/Sym.cinch", include_str!("../prelude/Data/Sym.cinch")),
    // The class modules and the newer families continue here; the
    // manifest's order, not this table's, decides where each is checked.
    (
        "Algebra/Semigroup.cinch",
        include_str!("../prelude/Algebra/Semigroup.cinch"),
    ),
    (
        "Algebra/Group.cinch",
        include_str!("../prelude/Algebra/Group.cinch"),
    ),
    (
        "Algebra/Semiring.cinch",
        include_str!("../prelude/Algebra/Semiring.cinch"),
    ),
    (
        "Algebra/Monoid.cinch",
        include_str!("../prelude/Algebra/Monoid.cinch"),
    ),
    (
        "Category/Functor.cinch",
        include_str!("../prelude/Category/Functor.cinch"),
    ),
    (
        "Category/Applicative.cinch",
        include_str!("../prelude/Category/Applicative.cinch"),
    ),
    (
        "Category/Monad.cinch",
        include_str!("../prelude/Category/Monad.cinch"),
    ),
    (
        "Category/Alternative.cinch",
        include_str!("../prelude/Category/Alternative.cinch"),
    ),
    (
        "Category/Foldable.cinch",
        include_str!("../prelude/Category/Foldable.cinch"),
    ),
    (
        "Category/Traversable.cinch",
        include_str!("../prelude/Category/Traversable.cinch"),
    ),
    (
        "Data/Identity.cinch",
        include_str!("../prelude/Data/Identity.cinch"),
    ),
    (
        "Data/Const.cinch",
        include_str!("../prelude/Data/Const.cinch"),
    ),
    (
        "Data/Compose.cinch",
        include_str!("../prelude/Data/Compose.cinch"),
    ),
    (
        "Data/NonEmpty.cinch",
        include_str!("../prelude/Data/NonEmpty.cinch"),
    ),
    (
        "Data/Sorted.cinch",
        include_str!("../prelude/Data/Sorted.cinch"),
    ),
    (
        "Data/NonZero.cinch",
        include_str!("../prelude/Data/NonZero.cinch"),
    ),
    (
        "Data/Ascii.cinch",
        include_str!("../prelude/Data/Ascii.cinch"),
    ),
    ("Data/Set.cinch", include_str!("../prelude/Data/Set.cinch")),
    (
        "Data/Hash.cinch",
        include_str!("../prelude/Data/Hash.cinch"),
    ),
    (
        "Text/Codec.cinch",
        include_str!("../prelude/Text/Codec.cinch"),
    ),
    (
        "Data/HashMap.cinch",
        include_str!("../prelude/Data/HashMap.cinch"),
    ),
    (
        "Effect/Stm.cinch",
        include_str!("../prelude/Effect/Stm.cinch"),
    ),
    (
        "Effect/Stm/TMVar.cinch",
        include_str!("../prelude/Effect/Stm/TMVar.cinch"),
    ),
    (
        "Effect/Stm/TQueue.cinch",
        include_str!("../prelude/Effect/Stm/TQueue.cinch"),
    ),
    (
        "Effect/Stm/TBQueue.cinch",
        include_str!("../prelude/Effect/Stm/TBQueue.cinch"),
    ),
    (
        "Effect/Stm/TSem.cinch",
        include_str!("../prelude/Effect/Stm/TSem.cinch"),
    ),
];

/// One module of a package manifest: the name, the source path, and
/// whether it is the re-export facade. The module's IMPORTS are not
/// recorded here on purpose: the compiler reads them from the source
/// (`scan_file`), and a copy in the manifest is a second truth that
/// drifts the moment an import is added.
#[derive(Debug, Clone)]
pub struct PackageModule {
    pub name: String,
    pub path: String,
    pub facade: bool,
    /// Whether another package may import this module
    /// (design/base-as-a-package.md section 4.2). A module without the
    /// flag is compiled as part of its package and is invisible to
    /// consumers; a manifest that predates the field means every module it
    /// lists is exposed.
    pub exposed: bool,
}

/// Every embedded family module's `(name, source hash)`, so a produced
/// interface records the bytes it came from. Without this the family's
/// interfaces carry an empty `source_hash`, which leaves a stored
/// artifact with no witness for which sources produced it.
pub fn family_source_hashes() -> Vec<(String, String)> {
    base_package()
        .modules
        .iter()
        .filter_map(|m| {
            embedded_source(&m.path)
                .map(|(_, src)| (m.name.clone(), crate::interface::content_hash(src)))
        })
        .collect()
}

/// A parsed package manifest (`.cinchpkg`): the module list and the
/// check ORDER a package ships as data, instead of the compiler
/// holding either in code (design/base-library.md, "Packages and the
/// manifest").
#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    /// The manifest's schema number: 1 for a fixed module list, 2 when
    /// the manifest carries `select` conditions, a `prelude` setting, or
    /// a `link` block.
    pub schema: u64,
    pub modules: Vec<PackageModule>,
    pub order: Vec<String>,
    /// The package's prelude setting (design/base-as-a-package.md
    /// section 3): the compiler's default, no prelude, or a named module
    /// injected as an implicit import into every module of this package.
    pub prelude: PreludeSpec,
    /// Conditional entries: modules and link words that apply only when a
    /// condition on the target holds (sections 4.1 and 4.2).
    pub select: Vec<SelectEntry>,
    /// Link requirements that apply on every target.
    pub link: LinkSpec,
    /// The packages whose raw link words this project consents to
    /// (design/base-as-a-package.md section 4.3). Only a root project's
    /// list is consulted, so a transitive dependency cannot smuggle raw
    /// words in behind a package the consumer trusted.
    pub raw_link: Vec<String>,
    /// Machine-specific link words: project configuration, never package
    /// metadata. Applied last, excluded from the lock, and recorded in
    /// the build so a reproducibility check can refuse the artifact.
    pub local_link: LinkSpec,
    /// The manifest bytes' content hash, recorded by [`Package::parse`].
    /// The package store keys on it (see `crate::store::package_key`), so
    /// an edit to the manifest that changes the module set, the paths or
    /// the order moves the key even when every source is unchanged.
    pub manifest_hash: String,
}

impl Package {
    /// Parse a manifest: canonical JSON (design/stdlib-and-bootstrap.md),
    /// read with the parser the `.cinchif` reader uses. The `Num`
    /// variant is unsigned, so every field is a string, a boolean or an
    /// array.
    pub fn parse(text: &str) -> Result<Package> {
        use crate::interface::{parse_json, JsonValue};
        let wrap = |e: crate::interface::ParseError| Error::plain(format!("package manifest: {e}"));
        let doc = parse_json(text).map_err(wrap)?;
        let obj = doc.as_object().map_err(wrap)?;
        let field = |k: &str| -> Result<&JsonValue> {
            obj.get(k)
                .ok_or_else(|| Error::plain(format!("package manifest: missing `{k}`")))
        };
        let name = str_of(field("name")?, "name")?;
        let version = str_of(field("version")?, "version")?;
        let mut modules: Vec<PackageModule> = Vec::new();
        for m in field("modules")?.as_array().map_err(wrap)? {
            modules.push(parse_module(m)?);
        }
        let mut order: Vec<String> = Vec::new();
        for n in field("order")?.as_array().map_err(wrap)? {
            order.push(str_of(n, "an order entry")?);
        }
        let schema = match obj.get("schema") {
            None => 1,
            Some(v) => v.as_num().unwrap_or(1),
        };
        let prelude = match obj.get("prelude") {
            None => PreludeSpec::Default,
            Some(v) => match v.as_str() {
                Ok("none") => PreludeSpec::None,
                Ok("default") => PreludeSpec::Default,
                Ok(name) if !name.is_empty() => PreludeSpec::Module(name.to_string()),
                _ => {
                    return Err(Error::plain(
                        "package manifest: `prelude` takes a module name, `default`, or `none`",
                    ))
                }
            },
        };
        let link = match obj.get("link") {
            None => LinkSpec::default(),
            Some(v) => LinkSpec::parse(v)?,
        };
        let mut raw_link: Vec<String> = Vec::new();
        if let Some(v) = obj.get("raw_link") {
            for item in v.as_array().map_err(wrap)? {
                raw_link.push(str_of(item, "a raw_link entry")?);
            }
        }
        let local_link = match obj.get("local_link") {
            None => LinkSpec::default(),
            Some(v) => LinkSpec::parse(v)?,
        };
        let mut select: Vec<SelectEntry> = Vec::new();
        match obj.get("select") {
            None => {}
            Some(v) => {
                for (i, entry) in v.as_array().map_err(wrap)?.iter().enumerate() {
                    let o = entry.as_object().map_err(wrap)?;
                    let when = match o.get("when") {
                        None => When::default(),
                        Some(w) => When::parse(w)?,
                    };
                    let mut mods: Vec<PackageModule> = Vec::new();
                    if let Some(ms) = o.get("modules") {
                        for m in ms.as_array().map_err(wrap)? {
                            mods.push(parse_module(m)?);
                        }
                    }
                    let link = match o.get("link") {
                        None => LinkSpec::default(),
                        Some(v) => LinkSpec::parse(v)?,
                    };
                    if mods.is_empty() && link.is_empty() {
                        return Err(Error::plain(format!(
                            "package manifest: `select` entry {i} changes nothing (no modules, no link words)"
                        )));
                    }
                    select.push(SelectEntry { when, modules: mods, link });
                }
            }
        }
        Ok(Package {
            name,
            version,
            schema,
            modules,
            order,
            prelude,
            select,
            link,
            raw_link,
            local_link,
            manifest_hash: crate::interface::content_hash(text),
        })
    }

    /// The facade's position in the check order: the split between the
    /// early homes (checked before it) and the late ones.
    pub fn facade_index(&self) -> usize {
        self.order
            .iter()
            .position(|n| self.modules.iter().any(|m| m.facade && &m.name == n))
            .unwrap_or(self.order.len())
    }

    /// `(path, source)` for every embedded module, in manifest order.
    pub fn ordered_sources(&self) -> Result<Vec<(&'static str, &'static str)>> {
        self.order
            .iter()
            .map(|name| {
                let m = self
                    .modules
                    .iter()
                    .find(|m| &m.name == name)
                    .ok_or_else(|| {
                        Error::plain(format!(
                            "package manifest: `{name}` is in `order` but has no module entry"
                        ))
                    })?;
                embedded_source(&m.path).ok_or_else(|| {
                    Error::plain(format!(
                        "package manifest: no embedded source for `{}`",
                        m.path
                    ))
                })
            })
            .collect()
    }

    /// Every module name, in check order.
    pub fn module_names(&self) -> Vec<String> {
        self.order.clone()
    }
}

/// A package's prelude setting (design/base-as-a-package.md section 3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreludeSpec {
    /// No `prelude` field: the compiler's built-in prelude, which is the
    /// embedded facade today and `cinch/base`'s facade after the cut.
    Default,
    /// `"prelude": "none"`: nothing is injected. The preregistered type
    /// names remain in scope, because a signature needs them.
    None,
    /// `"prelude": "Some.Module"`: that module is imported implicitly at
    /// the top of every module in this package.
    Module(String),
}

/// A condition on the target (section 4.1). Inside one `When`, keys are
/// ANDed; a list value is ORed; a missing key matches anything.
#[derive(Debug, Clone, Default)]
pub struct When {
    pub os: Vec<String>,
    pub arch: Vec<String>,
    pub os_version: Option<String>,
    pub libc: Vec<String>,
    pub compiler: Option<String>,
    pub runtime_abi: Option<String>,
    pub target: Vec<String>,
}

/// The facts a condition is evaluated against: the compile TARGET, never
/// the build host.
#[derive(Debug, Clone)]
pub struct TargetFacts {
    pub triple: String,
    pub os: String,
    pub arch: String,
    pub libc: String,
    /// `None` when the caller stated no platform version. A condition on
    /// `os_version` then refuses instead of guessing, because a
    /// host-derived version makes the lock machine-specific.
    pub os_version: Option<String>,
    pub compiler: String,
    pub runtime_abi: String,
}

impl TargetFacts {
    /// The facts of a target profile, with the compiler's own version and
    /// runtime ABI.
    pub fn of(profile: &crate::core::TargetProfile) -> TargetFacts {
        TargetFacts {
            triple: profile.triple.clone(),
            os: profile.os.clone(),
            arch: profile.arch.clone(),
            libc: profile.libc.clone(),
            os_version: profile.os_version.clone(),
            compiler: env!("CARGO_PKG_VERSION").to_string(),
            runtime_abi: crate::interface::RUNTIME_ABI_VERSION.to_string(),
        }
    }
}

/// One `select` entry: the modules and link words that apply when
/// `when` holds.
#[derive(Debug, Clone)]
pub struct SelectEntry {
    pub when: When,
    pub modules: Vec<PackageModule>,
    pub link: LinkSpec,
}

/// One `pkgconfig` requirement: the module name and an optional minimum
/// version. Resolution runs `pkg-config` once at install time and the
/// resolved words are recorded in the lock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PkgConfigReq {
    pub name: String,
    pub min: Option<String>,
}

/// What a package needs from the final link, in semantic fields
/// (section 4.3) plus the raw escape hatch.
#[derive(Debug, Clone, Default)]
pub struct LinkSpec {
    pub libs: Vec<String>,
    pub frameworks: Vec<String>,
    pub rpaths: Vec<String>,
    pub libdirs: Vec<String>,
    pub dll_dirs: Vec<String>,
    pub soname: Option<String>,
    pub whole_archive: Vec<String>,
    pub import_libs: Vec<String>,
    pub system_libs: Vec<String>,
    pub pkgconfig: Vec<PkgConfigReq>,
    /// The escape hatch: words recorded verbatim, refused unless the
    /// consuming root consents, and reported by `cinch pkg verify`.
    pub raw_ldflags: Vec<String>,
    pub raw_cflags: Vec<String>,
}

/// A package resolved for one target: the module list the compiler
/// builds, the link requirements that apply, and the prelude setting.
#[derive(Debug, Clone)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
    pub modules: Vec<PackageModule>,
    pub order: Vec<String>,
    pub link: LinkSpec,
    pub prelude: PreludeSpec,
    /// The indices of the `select` entries that applied, so a report can
    /// name what the target chose.
    pub matched: Vec<usize>,
}

impl LinkSpec {
    /// Parse a `link` object. Unknown keys are refused: a typo in a link
    /// field would otherwise silently drop a requirement.
    pub fn parse(v: &crate::interface::JsonValue) -> Result<LinkSpec> {
        let wrap = |e: crate::interface::ParseError| Error::plain(format!("package manifest: link: {e}"));
        let obj = v.as_object().map_err(wrap)?;
        let mut none = LinkSpec::default();
        let mut strings = |key: &str| -> Result<Vec<String>> {
            match obj.get(key) {
                None => Ok(Vec::new()),
                Some(v) => {
                    let mut out = Vec::new();
                    for item in v.as_array().map_err(wrap)? {
                        out.push(str_of(item, key)?);
                    }
                    Ok(out)
                }
            }
        };
        none.libs = strings("libs")?;
        none.frameworks = strings("frameworks")?;
        none.rpaths = strings("rpaths")?;
        none.libdirs = strings("libdirs")?;
        none.dll_dirs = strings("dll_dirs")?;
        none.whole_archive = strings("whole_archive")?;
        none.import_libs = strings("import_libs")?;
        none.system_libs = strings("system_libs")?;
        none.soname = match obj.get("soname") {
            None => None,
            Some(v) => Some(str_of(v, "soname")?),
        };
        if let Some(pc) = obj.get("pkgconfig") {
            for entry in pc.as_array().map_err(wrap)? {
                let o = entry.as_object().map_err(wrap)?;
                let name = match o.get("name") {
                    Some(v) => str_of(v, "a pkgconfig entry needs `name`")?,
                    None => {
                        return Err(Error::plain(
                            "package manifest: a pkgconfig entry needs `name`",
                        ))
                    }
                };
                let min = match o.get("min") {
                    None => None,
                    Some(v) => Some(str_of(v, "pkgconfig min")?),
                };
                none.pkgconfig.push(PkgConfigReq { name, min });
            }
        }
        if let Some(raw) = obj.get("raw") {
            let raw = raw.as_object().map_err(wrap)?;
            for (key, slot) in [
                ("ldflags", &mut none.raw_ldflags),
                ("cflags", &mut none.raw_cflags),
            ] {
                if let Some(v) = raw.get(key) {
                    for item in v.as_array().map_err(wrap)? {
                        slot.push(str_of(item, key)?);
                    }
                }
            }
            for key in raw.keys() {
                if key != "ldflags" && key != "cflags" {
                    return Err(Error::plain(format!(
                        "package manifest: link.raw: unknown key `{key}` (allowed: ldflags, cflags)"
                    )));
                }
            }
        }
        for key in obj.keys() {
            if !matches!(
                key.as_str(),
                "libs"
                    | "frameworks"
                    | "rpaths"
                    | "libdirs"
                    | "dll_dirs"
                    | "soname"
                    | "whole_archive"
                    | "import_libs"
                    | "system_libs"
                    | "pkgconfig"
                    | "raw"
            ) {
                return Err(Error::plain(format!(
                    "package manifest: link: unknown key `{key}` (allowed: libs, frameworks, rpaths, \
                     libdirs, dll_dirs, soname, whole_archive, import_libs, system_libs, pkgconfig, raw)"
                )));
            }
        }
        Ok(none)
    }

    /// Whether this contribution is empty in every tier.
    pub fn is_empty(&self) -> bool {
        self.libs.is_empty()
            && self.frameworks.is_empty()
            && self.rpaths.is_empty()
            && self.libdirs.is_empty()
            && self.dll_dirs.is_empty()
            && self.soname.is_none()
            && self.whole_archive.is_empty()
            && self.import_libs.is_empty()
            && self.system_libs.is_empty()
            && self.pkgconfig.is_empty()
            && self.raw_ldflags.is_empty()
            && self.raw_cflags.is_empty()
    }

    /// Union another contribution into this one, keeping order and
    /// dropping exact duplicates so a base entry plus a version override
    /// does not link the same library twice.
    pub fn merge(&mut self, other: &LinkSpec) {
        fn add(dst: &mut Vec<String>, src: &[String]) {
            for s in src {
                if !dst.contains(s) {
                    dst.push(s.clone());
                }
            }
        }
        add(&mut self.libs, &other.libs);
        add(&mut self.frameworks, &other.frameworks);
        add(&mut self.rpaths, &other.rpaths);
        add(&mut self.libdirs, &other.libdirs);
        add(&mut self.dll_dirs, &other.dll_dirs);
        add(&mut self.whole_archive, &other.whole_archive);
        add(&mut self.import_libs, &other.import_libs);
        add(&mut self.system_libs, &other.system_libs);
        add(&mut self.raw_ldflags, &other.raw_ldflags);
        add(&mut self.raw_cflags, &other.raw_cflags);
        for pc in &other.pkgconfig {
            if !self.pkgconfig.contains(pc) {
                self.pkgconfig.push(pc.clone());
            }
        }
        if self.soname.is_none() {
            self.soname = other.soname.clone();
        }
    }

    /// Whether any contribution is the raw escape hatch.
    pub fn has_raw(&self) -> bool {
        !self.raw_ldflags.is_empty() || !self.raw_cflags.is_empty()
    }
}

impl When {
    /// Parse a `when` object. An unknown key is an error: a misspelled
    /// condition would otherwise match everything and silently select a
    /// platform branch.
    pub fn parse(v: &crate::interface::JsonValue) -> Result<When> {
        let wrap = |e: crate::interface::ParseError| Error::plain(format!("package manifest: when: {e}"));
        let obj = v.as_object().map_err(wrap)?;
        let mut when = When::default();
        for (key, value) in obj {
            let mut list = |value: &crate::interface::JsonValue| -> Result<Vec<String>> {
                match value.as_str() {
                    Ok(s) => Ok(vec![s.to_string()]),
                    Err(_) => {
                        let mut out = Vec::new();
                        for item in value.as_array().map_err(wrap)? {
                            out.push(str_of(item, key)?);
                        }
                        Ok(out)
                    }
                }
            };
            match key.as_str() {
                "os" => when.os = list(value)?,
                "arch" => when.arch = list(value)?,
                "libc" => when.libc = list(value)?,
                "target" => when.target = list(value)?,
                "os_version" => when.os_version = Some(str_of(value, "os_version")?),
                "compiler" => when.compiler = Some(str_of(value, "compiler")?),
                "runtime_abi" => when.runtime_abi = Some(str_of(value, "runtime_abi")?),
                other => {
                    return Err(Error::plain(format!(
                        "package manifest: when: unknown condition `{other}` (allowed: os, arch, \
                         os_version, libc, compiler, runtime_abi, target)"
                    )))
                }
            }
        }
        Ok(when)
    }

    /// Whether this condition holds for `facts`. `os_version` refuses when
    /// the target states no version rather than guessing.
    pub fn matches(&self, facts: &TargetFacts, pkg: &str) -> Result<bool> {
        fn any_of(list: &[String], value: &str, normalise: fn(&str) -> String) -> bool {
            list.is_empty() || list.iter().any(|v| normalise(v) == normalise(value))
        }
        if !any_of(&self.os, &facts.os, normalise_os) {
            return Ok(false);
        }
        if !any_of(&self.arch, &facts.arch, normalise_arch) {
            return Ok(false);
        }
        if !any_of(&self.libc, &facts.libc, |s| s.to_ascii_lowercase()) {
            return Ok(false);
        }
        if !self.target.is_empty()
            && !self
                .target
                .iter()
                .any(|t| normalise_arch(t) == normalise_arch(&facts.triple))
        {
            return Ok(false);
        }
        if let Some(want) = &self.runtime_abi {
            if want != &facts.runtime_abi {
                return Ok(false);
            }
        }
        if let Some(want) = &self.compiler {
            if !version_matches(&facts.compiler, want) {
                return Ok(false);
            }
        }
        if let Some(want) = &self.os_version {
            let Some(have) = facts.os_version.as_deref() else {
                return Err(Error::plain(format!(
                    "{pkg}: the manifest selects on `os_version {want}`, and the target states no \
                     platform version; pass --target-version (for example --target-version 14.0)"
                )));
            };
            if !version_matches(have, want) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// The condition as its author wrote it, for a refusal that has to say
    /// which condition would make a module available: one space-separated
    /// conjunction, a list in parentheses, `always` when it selects
    /// unconditionally.
    pub fn describe(&self) -> String {
        let list = |values: &[String]| -> Option<String> {
            match values {
                [] => None,
                [one] => Some(one.clone()),
                many => Some(format!("({})", many.join(", "))),
            }
        };
        let mut parts: Vec<String> = Vec::new();
        for (key, value) in [
            ("os", list(&self.os)),
            ("arch", list(&self.arch)),
            ("libc", list(&self.libc)),
            ("target", list(&self.target)),
        ] {
            if let Some(value) = value {
                parts.push(format!("{key} {value}"));
            }
        }
        for (key, value) in [
            ("os_version", &self.os_version),
            ("compiler", &self.compiler),
            ("runtime_abi", &self.runtime_abi),
        ] {
            if let Some(value) = value {
                parts.push(format!("{key} {value}"));
            }
        }
        if parts.is_empty() {
            "always".to_string()
        } else {
            parts.join(" ")
        }
    }
}

/// `darwin` is the manifest's spelling; a caller may write `macos`.
fn normalise_os(s: &str) -> String {
    match s.to_ascii_lowercase().as_str() {
        "macos" | "osx" | "darwin" => "darwin".to_string(),
        other => other.to_string(),
    }
}

/// A triple writes `aarch64`, a manifest may write `arm64`.
fn normalise_arch(s: &str) -> String {
    match s.to_ascii_lowercase().as_str() {
        "arm64" | "aarch64" => "aarch64".to_string(),
        "amd64" | "x86_64" | "x64" => "x86_64".to_string(),
        other => other.to_string(),
    }
}

/// Compare a dotted version against a constraint such as `>=14.0`,
/// `=13.4`, `<12`, or a bare `14.0` (exact).
pub fn version_matches(have: &str, want: &str) -> bool {
    let want = want.trim();
    let (op, rest) = if let Some(r) = want.strip_prefix(">=") {
        (">=", r)
    } else if let Some(r) = want.strip_prefix("<=") {
        ("<=", r)
    } else if let Some(r) = want.strip_prefix("==") {
        ("=", r)
    } else if let Some(r) = want.strip_prefix('>') {
        (">", r)
    } else if let Some(r) = want.strip_prefix('<') {
        ("<", r)
    } else if let Some(r) = want.strip_prefix('=') {
        ("=", r)
    } else {
        ("=", want)
    };
    let order = compare_versions(have, rest.trim());
    match op {
        ">=" => order != std::cmp::Ordering::Less,
        ">" => order == std::cmp::Ordering::Greater,
        "<=" => order != std::cmp::Ordering::Greater,
        "<" => order == std::cmp::Ordering::Less,
        _ => order == std::cmp::Ordering::Equal,
    }
}

/// Numeric segment comparison of two dotted versions, so `14.10` is
/// greater than `14.9` (a string compare gets that wrong).
fn compare_versions(a: &str, b: &str) -> std::cmp::Ordering {
    let mut left = a.split(['.', '-', '+']);
    let mut right = b.split(['.', '-', '+']);
    loop {
        match (left.next(), right.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(r)) => {
                return if r.parse::<u64>().unwrap_or(0) == 0 {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            }
            (Some(l), None) => {
                return if l.parse::<u64>().unwrap_or(0) == 0 {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Greater
                }
            }
            (Some(l), Some(r)) => {
                let (Ok(l), Ok(r)) = (l.parse::<u64>(), r.parse::<u64>()) else {
                    return l.cmp(r);
                };
                match l.cmp(&r) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
        }
    }
}

/// One module entry of a manifest, shared by the unconditional list and
/// the `select` entries.
fn parse_module(m: &crate::interface::JsonValue) -> Result<PackageModule> {
    let wrap = |e: crate::interface::ParseError| Error::plain(format!("package manifest: {e}"));
    let o = m.as_object().map_err(wrap)?;
    let name = match o.get("name") {
        Some(v) => str_of(v, "a module entry needs `name`")?,
        None => return Err(Error::plain("package manifest: a module entry needs `name`")),
    };
    let path = match o.get("path") {
        Some(v) => str_of(v, "a module entry needs `path`")?,
        None => return Err(Error::plain("package manifest: a module entry needs `path`")),
    };
    // `embedded` is recorded for a reader's benefit; the source table is
    // what decides whether a path has bytes.
    let facade = matches!(o.get("facade"), Some(crate::interface::JsonValue::Bool(true)));
    let exposed = match o.get("exposed") {
        None => true,
        Some(v) => v
            .as_bool()
            .map_err(|_| Error::plain("package manifest: `exposed` takes true or false"))?,
    };
    for key in o.keys() {
        if !matches!(key.as_str(), "name" | "path" | "embedded" | "facade" | "exposed") {
            return Err(Error::plain(format!(
                "package manifest: unknown module key `{key}` (allowed: name, path, embedded, facade, exposed)"
            )));
        }
    }
    Ok(PackageModule {
        name,
        path,
        facade,
        exposed,
    })
}

impl Package {
    /// Resolve this package for one target: the module list the compiler
    /// builds, the link requirements that apply, and the prelude setting
    /// (design/base-as-a-package.md sections 4.1 to 4.4).
    pub fn resolve(&self, facts: &TargetFacts) -> Result<ResolvedPackage> {
        let mut modules = self.modules.clone();
        let mut link = self.link.clone();
        let mut matched: Vec<usize> = Vec::new();
        for (i, entry) in self.select.iter().enumerate() {
            if !entry.when.matches(facts, &self.name)? {
                continue;
            }
            matched.push(i);
            for m in &entry.modules {
                match modules.iter().find(|existing| existing.name == m.name) {
                    None => modules.push(m.clone()),
                    Some(existing) if existing.path == m.path => {
                        // The same module named twice with one path is
                        // additive, so a base set plus an override works.
                    }
                    Some(existing) => {
                        return Err(Error::plain(format!(
                            "{}: module `{}` is selected twice with different paths (`{}` and `{}`) \
                             for target {}",
                            self.name, m.name, existing.path, m.path, facts.triple
                        )))
                    }
                }
            }
            link.merge(&entry.link);
        }
        // The check order: the manifest's own order, filtered to what the
        // target resolved, with any newly selected module appended in
        // manifest order. An order entry that the target did not resolve
        // is a manifest error, because it means the two lists disagree.
        let mut order: Vec<String> = Vec::new();
        for name in &self.order {
            match modules.iter().find(|m| &m.name == name) {
                Some(_) => order.push(name.clone()),
                None => {
                    return Err(Error::plain(format!(
                        "{}: `order` names `{name}`, which the target {} does not resolve",
                        self.name, facts.triple
                    )))
                }
            }
        }
        for m in &modules {
            if !order.contains(&m.name) {
                order.push(m.name.clone());
            }
        }
        Ok(ResolvedPackage {
            name: self.name.clone(),
            version: self.version.clone(),
            modules,
            order,
            link,
            prelude: self.prelude.clone(),
            matched,
        })
    }
}

impl ResolvedPackage {
    /// The module names another package may import.
    pub fn exposed_names(&self) -> Vec<String> {
        self.modules
            .iter()
            .filter(|m| m.exposed)
            .map(|m| m.name.clone())
            .collect()
    }

    /// Whether a module is part of the resolution at all.
    pub fn owns(&self, module: &str) -> bool {
        self.modules.iter().any(|m| m.name == module)
    }
}

/// Which modules one project resolves for each profile of a build
/// (design/base-as-a-package.md, "Host and target are different
/// questions"): availability is a per-profile fact, and one build computes
/// both answers from the same manifests. A module no manifest declares (a
/// source file beside the entry, a prelude family member) is available for
/// both profiles, because no condition governs it.
#[derive(Clone, Debug, Default)]
pub struct ModuleAvailability {
    host: std::collections::BTreeSet<String>,
    target: std::collections::BTreeSet<String>,
    /// Every module a manifest declares, with the package that declares
    /// it and the `select` conditions that name it. A module two
    /// conditions select keeps both, so the refusal can show the one that
    /// would apply.
    declared: std::collections::BTreeMap<String, (String, Vec<String>)>,
    /// Why the host resolution could not be computed. Absent normally;
    /// when present, the host side is not evidence for a refusal, and the
    /// artifact manifest records the reason instead of pretending.
    pub host_unresolved: Option<String>,
    /// Whether the host resolution used the target's stated platform
    /// version, because the compiler cannot ask the host for its own. The
    /// artifact manifest records it: a module selected only by that
    /// version is then a fact about the target, not about the host.
    pub host_os_version_from_target: bool,
}

impl ModuleAvailability {
    /// The availability facts of a compile that resolves no package and
    /// reads no host: the incremental session and the single-source check
    /// entries. Nothing is declared, so no module's availability is a
    /// separate fact, and the artifact manifest says so rather than
    /// claiming a resolution that never ran.
    pub fn untracked(reason: &str) -> ModuleAvailability {
        ModuleAvailability {
            host_unresolved: Some(reason.to_string()),
            ..ModuleAvailability::default()
        }
    }
    /// Build the availability facts from the two resolutions of a
    /// project's manifests. `declared` is every manifest the project
    /// contains, for the packages a resolution may leave a module out of.
    fn from_resolutions(
        host: &[(PathBuf, Package, ResolvedPackage)],
        target: &[(PathBuf, Package, ResolvedPackage)],
        declared: &[(PathBuf, Package)],
    ) -> ModuleAvailability {
        let mut facts = ModuleAvailability::default();
        for (_, _, package) in target {
            for m in &package.modules {
                facts.target.insert(m.name.clone());
            }
        }
        for (_, _, package) in host {
            for m in &package.modules {
                facts.host.insert(m.name.clone());
            }
        }
        for (_, package) in declared {
            for m in &package.modules {
                facts
                    .declared
                    .entry(m.name.clone())
                    .or_insert_with(|| (package.name.clone(), Vec::new()));
            }
            for entry in &package.select {
                for m in &entry.modules {
                    let (_, conditions) = facts
                        .declared
                        .entry(m.name.clone())
                        .or_insert_with(|| (package.name.clone(), Vec::new()));
                    let condition = entry.when.describe();
                    if !conditions.contains(&condition) {
                        conditions.push(condition);
                    }
                }
            }
        }
        facts
    }

    /// Every module this project resolves for `context`'s profile.
    pub fn modules(&self, context: crate::core::BuildContext) -> &std::collections::BTreeSet<String> {
        match context {
            crate::core::BuildContext::Emitted => &self.target,
            crate::core::BuildContext::CompileTime => &self.host,
        }
    }

    /// Is this module part of the resolution for `context`'s profile?
    /// `None` when the host resolution could not be computed, which is the
    /// one answer the compiler does not have and does not invent.
    pub fn available_for(&self, context: crate::core::BuildContext, module: &str) -> Option<bool> {
        if context == crate::core::BuildContext::CompileTime && self.host_unresolved.is_some() {
            return None;
        }
        Some(self.modules(context).contains(module))
    }

    /// The package whose manifest declares this module, if one does.
    pub fn owner(&self, module: &str) -> Option<&str> {
        self.declared.get(module).map(|(package, _)| package.as_str())
    }

    /// The `select` conditions that name this module, in manifest order.
    /// Empty when its manifest lists it unconditionally.
    pub fn conditions(&self, module: &str) -> Vec<String> {
        self.declared
            .get(module)
            .map(|(_, conditions)| conditions.clone())
            .unwrap_or_default()
    }

    /// Every module a manifest in this project declares, sorted, for the
    /// artifact manifest's availability rows.
    pub fn declared_modules(&self) -> Vec<String> {
        self.declared.keys().cloned().collect()
    }

    /// One module's availability for one profile, spelled for the artifact
    /// manifest. `undecided` is the honest answer when the compiler has no
    /// host resolution to read.
    pub fn availability_spelling(
        &self,
        context: crate::core::BuildContext,
        module: &str,
    ) -> &'static str {
        match self.available_for(context, module) {
            Some(true) => "available",
            Some(false) => "unavailable",
            None => "undecided",
        }
    }

    /// The condition sentence a refusal quotes, phrased so it reads after
    /// the package name: "selects it `when os darwin`".
    fn condition_account(&self, module: &str) -> String {
        let conditions = self.conditions(module);
        match conditions.as_slice() {
            [] => "lists it unconditionally, so the resolution left it out for another reason"
                .to_string(),
            [one] => format!("selects it `when {one}`"),
            many => format!(
                "selects it `when {}`",
                many.iter()
                    .map(|c| format!("({c})"))
                    .collect::<Vec<_>>()
                    .join(" or `when ")
            ),
        }
    }
}

fn str_of(v: &crate::interface::JsonValue, what: &str) -> Result<String> {
    v.as_str()
        .map(|s| s.to_string())
        .map_err(|e| Error::plain(format!("package manifest: {what}: {e}")))
}

/// The modules this target never selects: every import that names them
/// carries a `when` condition that does not hold for `profile` (modules
/// specification section 11), and no import names them unconditionally.
/// The target's view of each importer already drops those imports
/// (`check_decls::target_view`); this is the same decision read from the
/// imported side, for the one consumer that roots modules rather than
/// reaching them by import, a library object.
fn unselected_modules<'a>(
    modules: impl Iterator<Item = &'a crate::ast::Module>,
    profile: &crate::core::TargetProfile,
) -> Result<std::collections::HashSet<String>> {
    let mut held: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut dropped: std::collections::HashSet<String> = std::collections::HashSet::new();
    for m in modules {
        for d in &m.decls {
            if let crate::ast::Decl::Import(i) = d {
                let name = i.path.parts().join(".");
                if i.when.is_empty() || crate::core::WhenCond::lower(&i.when)?.holds(profile) {
                    held.insert(name);
                } else {
                    dropped.insert(name);
                }
            }
        }
    }
    Ok(dropped.into_iter().filter(|name| !held.contains(name)).collect())
}

/// The embedded `(path, source)` entry for a manifest path (the
/// `include_str!` table).
pub fn embedded_source(path: &str) -> Option<(&'static str, &'static str)> {
    EMBEDDED_SOURCES.iter().copied().find(|(p, _)| *p == path)
}

/// The base package, parsed once from its embedded manifest.
pub static BASE: std::sync::LazyLock<Package> = std::sync::LazyLock::new(|| {
    Package::parse(BASE_PACKAGE)
        .unwrap_or_else(|e| panic!("the embedded base package manifest must parse: {e}"))
});

/// The base package (see [`BASE`]).
pub fn base_package() -> &'static Package {
    &BASE
}

/// The embedded family's sources in manifest order. Infallible at the
/// call sites: the manifest is embedded, and `tests/package_manifest.rs`
/// pins it against the source table.
pub fn prelude_sources() -> Vec<(&'static str, &'static str)> {
    base_package()
        .ordered_sources()
        .unwrap_or_else(|e| panic!("the embedded base package manifest is inconsistent: {e}"))
}

/// Every embedded prelude module name (the family's scheme names and
/// the facade), in manifest order: imports of these resolve to the
/// embedded sources, never to a file beside the entry unless an
/// explicit source module shadows the copy.
pub fn prelude_module_names() -> Vec<String> {
    base_package().module_names()
}

/// The parsed prelude family: the scheme homes in dependency order
/// with the `Prelude` facade last, every module's fixity table, and
/// the facade's fixity table (the base table all parsing uses).
pub struct PreludeSources {
    pub modules: Vec<crate::ast::Module>,
    pub fixities: HashMap<String, FixityTable>,
    pub exported_fixities: HashMap<String, FixityTable>,
    pub base: FixityTable,
}

/// Parse the embedded family with the WIT adapters' generated
/// vocabulary appended to the facade's source file (the vocabulary
/// declares inside the facade and joins its export list).
pub fn parse_prelude(adapters: &[crate::interface::WitAdapter]) -> Result<PreludeSources> {
    let facade_src = prelude_source_with(adapters);
    let mut headers = Vec::new();
    let facade_at = base_package().facade_index();
    for (i, (path, source)) in prelude_sources().into_iter().enumerate() {
        let source = if i == facade_at {
            facade_src.as_str()
        } else {
            source
        };
        let toks = layout::layout(lexer::lex(source)?)?;
        headers.extend(
            crate::parser::fixity_headers(&toks)
                .map_err(|e| Error::plain(format!("{path}: {e}")))?,
        );
    }
    let mut exported_fixities = crate::parser::export_fixity_headers(&headers, &HashMap::new())?;
    let base = exported_fixities
        .get("Prelude")
        .cloned()
        .unwrap_or_default();
    let facade_parsed = parse_source(&facade_src, "Prelude.cinch", &base, &exported_fixities)?;
    let mut fixities = facade_parsed.fixities.clone();
    let mut early: Vec<crate::ast::Module> = Vec::new();
    let mut late: Vec<crate::ast::Module> = Vec::new();
    let mut tables = exported_fixities.clone();
    let mut parse_into = |src: &str, path: &str, out: &mut Vec<crate::ast::Module>| -> Result<()> {
        let parsed = parse_source(src, path, &base, &tables)?;
        for (m, f) in &parsed.fixities {
            fixities.insert(m.clone(), f.clone());
        }
        for (m, f) in &parsed.exported_fixities {
            tables.insert(m.clone(), f.clone());
            exported_fixities.insert(m.clone(), f.clone());
        }
        out.extend(parsed.modules);
        Ok(())
    };
    // The manifest's order decides everything. Every module listed
    // before the facade is an early home (checked before it, so the
    // facade's `export` items can read its surface); the facade itself
    // is parsed above, because ITS fixity table is the base table every
    // module parses against; every module after the facade is a late
    // home (checked after it, resolving old-surface names through it).
    for (i, (path, src)) in prelude_sources().into_iter().enumerate() {
        if i == facade_at {
            continue;
        }
        let out = if i < facade_at { &mut early } else { &mut late };
        parse_into(src, path, out)?;
    }
    // Check order: the facade's export targets first (the facade reads
    // their surfaces), the facade (the implicit base layer the late
    // homes resolve old-surface names through), then the late homes.
    let mut modules = early;
    modules.extend(
        facade_parsed
            .modules
            .into_iter()
            .filter(|m| m.name.text == "Prelude"),
    );
    modules.append(&mut late);
    Ok(PreludeSources {
        modules,
        fixities,
        exported_fixities,
        base,
    })
}

pub use crate::check::{StageInput, StageMode, StageReport, StagingCx};

/// Lex, lay out and parse one source file: any number of `module` blocks
/// (the staging slice supersedes the one-module-per-file rule). `base` is
/// the fixity table every module sees (the prelude); `tables` maps imported
/// module names to their tables.
pub fn parse_source(
    src: &str,
    file: &str,
    base: &FixityTable,
    tables: &HashMap<String, FixityTable>,
) -> Result<ParsedModules> {
    let ctx = |e: Error| Error::plain(format!("{file}: {e}"));
    let tokens = lexer::lex(src).map_err(ctx)?;
    let tokens = layout::layout(tokens).map_err(ctx)?;
    parse_modules(&tokens, base, tables).map_err(ctx)
}

/// A source file's module headers and their imports, without full parsing.
pub struct ScannedFile {
    /// `(module name, imports)` in declaration order; an import is
    /// `(imported name, stage)`.
    pub modules: Vec<(String, Vec<(String, bool)>)>,
}

/// Scan a source's `module` and `import` declarations: the module graph
/// must be known before parsing (imports need their modules' fixities).
/// Conditions are read against the build host's target, which is what an
/// editor asking about this source alone sees.
pub fn scan_file(src: &str, file: &str) -> Result<ScannedFile> {
    scan_file_for(src, file, &crate::core::TargetProfile::native())
}

/// [`scan_file`] for one compile target: an import whose `when` condition
/// does not hold in the profile is not part of the graph, so it loads no
/// module, contributes no dependency and cannot affect the check order
/// (modules specification section 11). The condition is read here, before
/// parsing, because the file set to load is what the graph decides; the
/// parser reads the same clause into the import's abstract syntax, and the
/// checker prunes from that syntax so names and interfaces agree.
pub fn scan_file_for(
    src: &str,
    file: &str,
    profile: &crate::core::TargetProfile,
) -> Result<ScannedFile> {
    let ctx = |e: Error| Error::plain(format!("{file}: {e}"));
    let tokens = lexer::lex(src).map_err(ctx)?;
    let tokens = layout::layout(tokens).map_err(ctx)?;
    let mut modules: Vec<(String, Vec<(String, bool)>)> = Vec::new();
    let mut i = 0;
    // The module name is `con_id { "." con_id }`; an import contributes to
    // the module declaration it follows.
    let mut path_at = |i: &mut usize| -> Option<String> {
        let mut segments = Vec::new();
        while let Some(t) = tokens.get(*i) {
            match &t.tok {
                lexer::Tok::ConId(s) => {
                    segments.push(s.clone());
                    *i += 1;
                    if matches!(tokens.get(*i).map(|t| &t.tok), Some(lexer::Tok::Dot)) {
                        *i += 1;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        if segments.is_empty() {
            None
        } else {
            Some(segments.join("."))
        }
    };
    while i < tokens.len() {
        match &tokens[i].tok {
            lexer::Tok::Module => {
                i += 1;
                // `export module X` names a module to re-export; it does
                // not declare one (surface grammar local amendments
                // section 11).
                let exported = i >= 2 && matches!(tokens[i - 2].tok, lexer::Tok::Export);
                if exported {
                    let _ = path_at(&mut i);
                    continue;
                }
                let name = path_at(&mut i).unwrap_or_default();
                modules.push((name, Vec::new()));
            }
            lexer::Tok::Import => {
                i += 1;
                let stage = matches!(tokens.get(i).map(|t| &t.tok), Some(lexer::Tok::Stage));
                if stage {
                    i += 1;
                }
                let Some(name) = path_at(&mut i) else { continue };
                // The import's `when` clauses run to the end of the
                // declaration. A clause the scan cannot read is treated as
                // live: the parser reports it in full when it reaches the
                // declaration, and the checker prunes from that syntax.
                let mut clauses: Vec<crate::ast::Term> = Vec::new();
                let mut j = i;
                while j < tokens.len() {
                    match &tokens[j].tok {
                        lexer::Tok::Semicolon | lexer::Tok::Eof => break,
                        lexer::Tok::VarId(w) if w == "when" => {
                            match crate::parser::parse_when_clause(&tokens, j) {
                                Some((term, end)) => {
                                    clauses.push(term);
                                    j = end;
                                }
                                None => {
                                    clauses.clear();
                                    break;
                                }
                            }
                        }
                        _ => j += 1,
                    }
                }
                i = j;
                let live = match crate::core::WhenCond::lower(&clauses) {
                    Ok(cond) => cond.holds(profile),
                    Err(_) => true,
                };
                if live {
                    if let Some((_, imps)) = modules.last_mut() {
                        if !imps.iter().any(|(n, _)| *n == name) {
                            imps.push((name, stage));
                        }
                    }
                }
            }
            _ => i += 1,
        }
    }
    Ok(ScannedFile { modules })
}

/// A resolved multi-file project: modules in dependency order (imports
/// first), with each module's source file and imports.
struct Project {
    /// Module name to the file declaring it (a file may declare several).
    module_file: HashMap<String, PathBuf>,
    /// Every resolved package the project contains: the root project's
    /// own `.cinchpkg` and each vendored dependency, resolved for the
    /// target (design/base-as-a-package.md sections 4.1 to 4.4).
    packages: Vec<(PathBuf, Package, ResolvedPackage)>,
    /// Module to the package that declares it. A module outside every
    /// manifest has no owner, which keeps single-file projects working.
    owner: HashMap<String, String>,
    /// Modules another package may import (section 4.2).
    exposed: HashMap<String, bool>,
    /// Per-package preludes: a module whose package names one sees that
    /// module injected instead of the default facade (section 3).
    module_preludes: HashMap<String, String>,
    /// File to source text.
    sources: HashMap<String, String>,
    /// Module to its import names.
    imports: HashMap<String, Vec<String>>,
    /// The entry file's modules, in declaration order.
    entry_modules: Vec<String>,
    /// Module names in dependency order.
    order: Vec<String>,
    /// What each profile of the build resolves for this project
    /// (design/base-as-a-package.md, "Host and target are different
    /// questions"). The compilation uses the target's answer; the host's
    /// is the fact a compile-time reachability refusal quotes.
    availability: ModuleAvailability,
    /// Imports that resolve for the host and not for the target, with the
    /// module that wrote them. The load drops them from the target's
    /// module graph, so which refusal they deserve is decided once the
    /// build knows which modules are compile-time code.
    host_only: Vec<HostOnlyImport>,
}

/// An `import M` that names a module the host resolution contains and the
/// target's does not (design/base-as-a-package.md, "Host and target are
/// different questions"). It has no place in the target's module graph,
/// and the context that reached it decides what the refusal says: code that
/// runs during the build on the host, or code emitted for the target.
struct HostOnlyImport {
    /// The module that wrote the import.
    importer: String,
    /// The module it named.
    module: String,
    /// `import stage M`: compile-time visibility, wherever it is written.
    stage: bool,
}

/// Every `module X` declaration beside `dir`, for import resolution when
/// `X.cinch` does not exist (decision D42).
fn index_dir_modules(dir: &Path) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let entries = match std::fs::read_dir(&current) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let skip = matches!(
                    path.file_name().and_then(|n| n.to_str()),
                    Some(".git" | "target" | "build" | "node_modules")
                );
                if !skip {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("cinch") {
                continue;
            }
            let Ok(src) = std::fs::read_to_string(&path) else {
                continue;
            };
            let Ok(scanned) = scan_file(&src, &path.display().to_string()) else {
                continue;
            };
            for (m, _) in &scanned.modules {
                index.entry(m.clone()).or_default().push(path.clone());
            }
        }
    }
    Ok(index)
}

/// One contribution to the final link, tagged with where it came from so
/// a report can name it (design/base-as-a-package.md section 4.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkEntry {
    /// The package that asked for it, or `project-local` for an override.
    pub origin: String,
    pub tier: LinkTier,
    /// The words this contribution adds, in order.
    pub words: Vec<String>,
}

/// Which tier of section 4.3 a contribution belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkTier {
    /// A semantic field: validated, ordered, and reported per package.
    Semantic,
    /// The escape hatch: recorded verbatim, refused without consent.
    Raw,
    /// Machine-local configuration: applied last, never locked.
    Local,
}

/// The final link's arguments, built from the resolved package closure
/// plus the root project's consent list and local overrides.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LinkPlan {
    pub entries: Vec<LinkEntry>,
    /// Whether any local override took part, which the build records so a
    /// reproducibility check can refuse the artifact.
    pub local: bool,
}

impl LinkPlan {
    /// Build the plan for a closure. Raw words are refused unless the
    /// root project consented to that package by name, and the words that
    /// change what the binary *is* are refused even then: a package that
    /// needs those needs a different mechanism.
    pub fn build(
        packages: &[(PathBuf, ResolvedPackage)],
        consent: &[String],
        local: &LinkSpec,
        lock_native: &std::collections::BTreeMap<String, Vec<String>>,
        facts: &TargetFacts,
    ) -> Result<LinkPlan> {
        let mut plan = LinkPlan::default();
        for (_, package) in packages {
            // A field that belongs to one platform's linker is a manifest
            // bug on another: `-framework` is ld64's and `-soname` is
            // GNU ld's, and silently dropping either would link without
            // what the package asked for.
            if !package.link.frameworks.is_empty() && facts.os != "darwin" {
                return Err(Error::plain(format!(
                    "{}: asks for the framework(s) {}, and frameworks belong to the Darwin \
                     linker; select that entry on `os: [\"darwin\"]`",
                    package.name,
                    package.link.frameworks.join(", ")
                )));
            }
            if package.link.soname.is_some() && facts.os != "linux" {
                return Err(Error::plain(format!(
                    "{}: asks for soname {}, which belongs to the GNU linker; select that \
                     entry on `os: [\"linux\"]`",
                    package.name,
                    package.link.soname.as_deref().unwrap_or("")
                )));
            }
            // A package that asked for a system library through
            // pkg-config gets its words from the lock, resolved once at
            // install time, so a build never re-reads the ambient system
            // (section 4.4). A missing record is a refusal, not a silent
            // link without the library.
            if !package.link.pkgconfig.is_empty() {
                match lock_native.get(&package.name) {
                    Some(words) => plan.entries.push(LinkEntry {
                        origin: package.name.clone(),
                        tier: LinkTier::Semantic,
                        words: words.clone(),
                    }),
                    None => {
                        return Err(Error::plain(format!(
                            "{} needs native libraries recorded in .cinchlock ({}), and this \
                             project has no record for it; run `cinch pkg install`",
                            package.name,
                            package
                                .link
                                .pkgconfig
                                .iter()
                                .map(|p| p.name.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )))
                    }
                }
            }
            let mut semantic = package.link.words_for(LinkTier::Semantic, &facts.os);
            semantic.retain(|w| !w.is_empty());
            if !semantic.is_empty() {
                plan.entries.push(LinkEntry {
                    origin: package.name.clone(),
                    tier: LinkTier::Semantic,
                    words: semantic,
                });
            }
            if package.link.has_raw() {
                if let Some(bad) = package.link.forbidden_raw().first() {
                    return Err(Error::plain(format!(
                        "{}: raw link word `{bad}` is refused even with consent; \
                         it changes what the produced binary is, not how a library is found",
                        package.name
                    )));
                }
                if !consent.iter().any(|c| c == &package.name) {
                    return Err(Error::plain(format!(
                        "{}: the manifest asks for raw link words ({}); this project has not \
                         consented. Add \"raw_link\": [\"{}\"] to the root .cinchpkg, or pass \
                         --allow-raw-link {}",
                        package.name,
                        package.link.raw_ldflags.join(" "),
                        package.name,
                        package.name
                    )));
                }
                let mut words = package.link.words_for(LinkTier::Raw, &facts.os);
                words.retain(|w| !w.is_empty());
                if !words.is_empty() {
                    plan.entries.push(LinkEntry {
                        origin: package.name.clone(),
                        tier: LinkTier::Raw,
                        words,
                    });
                }
            }
        }
        let local_words = local.words_for(LinkTier::Local, &facts.os);
        if !local_words.is_empty() {
            plan.local = true;
            plan.entries.push(LinkEntry {
                origin: "project-local".to_string(),
                tier: LinkTier::Local,
                words: local_words,
            });
        }
        Ok(plan)
    }

    /// The words the linker receives, in dependency order.
    pub fn words(&self) -> Vec<String> {
        self.entries.iter().flat_map(|e| e.words.clone()).collect()
    }

    /// One line per contribution, for `--print-link-plan` and the build
    /// log: the origin, the tier, and the words.
    pub fn describe(&self) -> String {
        let mut out = String::new();
        if self.entries.is_empty() {
            out.push_str("link plan: no package asked for anything\n");
            return out;
        }
        for entry in &self.entries {
            let tier = match entry.tier {
                LinkTier::Semantic => "semantic",
                LinkTier::Raw => "raw",
                LinkTier::Local => "local",
            };
            out.push_str(&format!(
                "link plan: {:<12} {:<24} {}\n",
                tier,
                entry.origin,
                entry.words.join(" ")
            ));
        }
        out
    }

    /// The packages that contributed raw words, for the build record.
    pub fn raw_origins(&self) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.tier == LinkTier::Raw)
            .map(|e| e.origin.clone())
            .collect()
    }
}

impl LinkSpec {
    /// The linker words this spec contributes at one tier. `Semantic`
    /// turns the fields into clang arguments; `Local` passes raw words
    /// through; `Raw` is the escape hatch, whose consent gate lives in
    /// [`LinkPlan::build`]. `os` is the compile target's operating
    /// system, because `-framework` and `-soname` belong to one linker
    /// each.
    pub fn words_for(&self, tier: LinkTier, os: &str) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        match tier {
            LinkTier::Semantic => {
                for dir in &self.libdirs {
                    words.push(format!("-L{dir}"));
                }
                for path in &self.rpaths {
                    words.push(format!("-Wl,-rpath,{path}"));
                }
                if let Some(soname) = &self.soname {
                    if os == "linux" {
                        words.push(format!("-Wl,-soname,{soname}"));
                    }
                }
                for lib in self.libs.iter().chain(&self.system_libs).chain(&self.import_libs) {
                    words.push(format!("-l{lib}"));
                }
                if os == "darwin" {
                    for framework in &self.frameworks {
                        words.push("-framework".to_string());
                        words.push(framework.clone());
                    }
                }
                for lib in &self.whole_archive {
                    words.push("-Wl,-whole-archive".to_string());
                    words.push(format!("-l{lib}"));
                    words.push("-Wl,-no-whole-archive".to_string());
                }
            }
            LinkTier::Raw => {
                words.extend(self.raw_ldflags.iter().cloned());
            }
            LinkTier::Local => {
                for dir in &self.libdirs {
                    words.push(format!("-L{dir}"));
                }
                words.extend(self.raw_ldflags.iter().cloned());
            }
        }
        words
    }

    /// Raw words that are refused even under consent, because they change
    /// what the produced binary is rather than how a library is found.
    pub fn forbidden_raw(&self) -> Vec<String> {
        self.raw_ldflags
            .iter()
            .filter(|word| {
                let w = word.as_str();
                w.starts_with("-Wl,--script")
                    || w.starts_with("--script")
                    || w == "-nostdlib"
                    || w == "-nostartfiles"
                    || w.starts_with("-Wl,-e")
                    || w == "-Xlinker"
                    || w.starts_with('@')
            })
            .cloned()
            .collect()
    }
}

/// The native words each locked package recorded at install time
/// (`.cinchlock`'s `native.words`), so a build links exactly what the
/// install resolved instead of re-reading the ambient system.
fn lock_native_words(
    project_dir: &Path,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut out = std::collections::BTreeMap::new();
    let text = match std::fs::read_to_string(project_dir.join(".cinchlock")) {
        Ok(text) => text,
        Err(_) => return out,
    };
    let Ok(lock) = serde_json::from_str::<serde_json::Value>(&text) else {
        return out;
    };
    let Some(rows) = lock.get("packages").and_then(|p| p.as_array()) else {
        return out;
    };
    for row in rows {
        let Some(name) = row.get("name").and_then(|n| n.as_str()) else {
            continue;
        };
        let Some(words) = row
            .get("native")
            .and_then(|n| n.get("words"))
            .and_then(|w| w.as_array())
        else {
            continue;
        };
        out.insert(
            name.to_string(),
            words
                .iter()
                .filter_map(|w| w.as_str().map(str::to_string))
                .collect(),
        );
    }
    out
}

/// Every package manifest a project contains: the project's own
/// `.cinchpkg`, then each vendored dependency's. A manifest that sits
/// under `vendor/` is a dependency; anything else is the project.
fn discover_manifests(dir: &Path) -> Result<Vec<(PathBuf, String)>> {
    let mut found: Vec<(PathBuf, String)> = Vec::new();
    let root = dir.join(".cinchpkg");
    if root.is_file() {
        let text = std::fs::read_to_string(&root)
            .map_err(|e| Error::plain(format!("{}: {e}", root.display())))?;
        found.push((dir.to_path_buf(), text));
    }
    let vendor = dir.join("vendor");
    if vendor.is_dir() {
        let mut stack = vec![vendor];
        while let Some(current) = stack.pop() {
            let entries = match std::fs::read_dir(&current) {
                Ok(e) => e,
                Err(_) => continue,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.file_name().and_then(|n| n.to_str()) == Some(".cinchpkg") {
                    let text = std::fs::read_to_string(&path)
                        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
                    let package_dir = path
                        .parent()
                        .map(|p| p.to_path_buf())
                        .unwrap_or_else(|| current.clone());
                    found.push((package_dir, text));
                }
            }
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

/// Every manifest a project contains, parsed once. Both profiles of the
/// build resolve these, and a refusal reads the declarations they carry,
/// so the discovery and the parse happen here rather than per profile.
fn project_manifests(dir: &Path) -> Result<Vec<(PathBuf, Package)>> {
    let mut out: Vec<(PathBuf, Package)> = Vec::new();
    for (package_dir, text) in discover_manifests(dir)? {
        let pkg = Package::parse(&text).map_err(|e| {
            Error::plain(format!("{}: {e}", package_dir.join(".cinchpkg").display()))
        })?;
        out.push((package_dir, pkg));
    }
    Ok(out)
}

/// Resolve every package in a project for one profile, and refuse two
/// packages that claim one module name (the duplicate-module rule the
/// registry also enforces).
fn resolve_project_packages(
    manifests: &[(PathBuf, Package)],
    facts: &TargetFacts,
) -> Result<Vec<(PathBuf, Package, ResolvedPackage)>> {
    let mut resolved: Vec<(PathBuf, Package, ResolvedPackage)> = Vec::new();
    for (package_dir, pkg) in manifests {
        let package = pkg.resolve(facts).map_err(|e| {
            Error::plain(format!("{}: {e}", package_dir.join(".cinchpkg").display()))
        })?;
        for m in &package.modules {
            if let Some((other_dir, _, other)) = resolved
                .iter()
                .find(|(_, _, r)| r.owns(&m.name))
            {
                return Err(Error::plain(format!(
                    "module {} is declared by both package {} ({}) and package {} ({})",
                    m.name,
                    other.name,
                    other_dir.display(),
                    package.name,
                    package_dir.display()
                )));
            }
        }
        resolved.push((package_dir.clone(), pkg.clone(), package));
    }
    Ok(resolved)
}

/// The facts the host side of a build resolves against: the machine's own
/// os, arch and libc. The compiler cannot ask a host for its platform
/// version, so a manifest condition on `os_version` uses the target's
/// stated version when the host states none, and the artifact manifest
/// records that substitution, because a module selected only by that
/// version is then a fact about the target rather than about the host.
fn host_facts(build: &crate::core::BuildProfiles) -> (TargetFacts, bool) {
    let mut facts = TargetFacts::of(&build.host);
    let mut borrowed = false;
    if facts.os_version.is_none() {
        if let Some(version) = &build.target.os_version {
            facts.os_version = Some(version.clone());
            borrowed = true;
        }
    }
    (facts, borrowed)
}

/// Load the entry file and every module it imports, transitively. A source
/// file may declare any number of modules (the staging slice supersedes the
/// one-module-per-file rule); `import X` resolves to a module of an already
/// loaded file, then to `X.cinch`, then to a unique `module X` beside the
/// entry (decision D42).
fn load_project(entry_path: &Path) -> Result<Project> {
    load_project_for(entry_path, &crate::core::BuildProfiles::native())
}

/// [`load_project`] for one build: the target's profile decides which
/// modules the artifact compiles (a package's `select` conditions and
/// every import's `when` clause resolve against it), and the host's
/// profile decides which modules the build *could* compile, so a module
/// available for one profile and not the other is a fact the build knows
/// rather than a failure the resolver invents (design/base-as-a-package.md,
/// "Host and target are different questions"; modules specification
/// section 11).
fn load_project_for(entry_path: &Path, build: &crate::core::BuildProfiles) -> Result<Project> {
    // Emitted code and its modules are the target: the compilation this
    // project describes is the target's.
    let profile = crate::core::condition_context(build, crate::core::BuildContext::Emitted);
    let target_facts = TargetFacts::of(profile);
    let dir = entry_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();
    let mut sources: HashMap<String, String> = HashMap::new();
    let mut module_file: HashMap<String, PathBuf> = HashMap::new();
    let mut imports: HashMap<String, Vec<String>> = HashMap::new();
    let mut entry_modules: Vec<String> = Vec::new();
    let mut host_only: Vec<HostOnlyImport> = Vec::new();
    let mut queue: Vec<PathBuf> = vec![entry_path.to_path_buf()];
    let dir_index = index_dir_modules(&dir)?;
    // Packages: the root project's manifest plus every vendored
    // dependency, resolved for this target, and resolved once more for
    // the host so availability is a per-profile fact rather than a guess
    // (design/base-as-a-package.md sections 2, 4.1 and 4.2). A project
    // with no manifest resolves none, which keeps single-file programs
    // and the test fixtures unchanged.
    let manifests = project_manifests(&dir)?;
    let packages = resolve_project_packages(&manifests, &target_facts)?;
    let (host, host_os_version_from_target) = host_facts(build);
    let (host_packages, host_unresolved) = match resolve_project_packages(&manifests, &host) {
        Ok(packages) => (packages, None),
        // The host's resolution is a fact used for diagnostics, never the
        // compilation, so its failure cannot fail a build: the build
        // records that it had no host evidence instead.
        Err(e) => (Vec::new(), Some(e.to_string())),
    };
    let mut availability = ModuleAvailability::from_resolutions(&host_packages, &packages, &manifests);
    availability.host_unresolved = host_unresolved;
    availability.host_os_version_from_target = host_os_version_from_target;
    let mut owner: HashMap<String, String> = HashMap::new();
    let mut exposed: HashMap<String, bool> = HashMap::new();
    let mut module_preludes: HashMap<String, String> = HashMap::new();
    for (package_dir, _declared, package) in &packages {
        for m in &package.modules {
            owner.insert(m.name.clone(), package.name.clone());
            exposed.insert(m.name.clone(), m.exposed);
        }
        if matches!(package.prelude, PreludeSpec::None) {
            // `prelude: none` is a decision per module, not the absence of
            // one: recording it keeps the interface from claiming the
            // default facade was imported when it was not.
            for m in &package.modules {
                module_preludes.insert(m.name.clone(), String::new());
            }
        }
        if let PreludeSpec::Module(prelude) = &package.prelude {
            // The injected module has to exist somewhere in the closure:
            // a package naming a prelude nobody declares would otherwise
            // fail much later as an unknown name in every module of the
            // package.
            let declared_here = package.modules.iter().any(|m| &m.name == prelude);
            if !declared_here {
                let elsewhere = packages
                    .iter()
                    .any(|(_, _, other)| other.owns(prelude));
                if !elsewhere {
                    return Err(Error::plain(format!(
                        "{}: the manifest names prelude `{prelude}`, and no package in this \
                         project declares that module",
                        package.name
                    )));
                }
            }
            for m in &package.modules {
                module_preludes.insert(m.name.clone(), prelude.clone());
            }
            // The injected module has to be part of the compilation: a
            // prelude the target resolves but nothing imports would
            // otherwise never load.
            if let Some(m) = package.modules.iter().find(|m| &m.name == prelude) {
                let path = package_dir.join(&m.path);
                if path.exists() {
                    queue.push(path);
                }
            }
        }
    }
    while let Some(path) = queue.pop() {
        let key = path.display().to_string();
        if sources.contains_key(&key) {
            continue;
        }
        let src = std::fs::read_to_string(&path)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        let scanned = scan_file_for(&src, &key, profile)?;
        if scanned.modules.is_empty() {
            return Err(Error::plain(format!("{key}: no `module` declaration")));
        }
        if path == entry_path {
            entry_modules = scanned.modules.iter().map(|(m, _)| m.clone()).collect();
        }
        for (m, imps) in &scanned.modules {
            if m.is_empty() {
                return Err(Error::plain(format!("{key}: a module needs a name")));
            }
            if let Some(prev) = module_file.get(m) {
                if prev != &path {
                    return Err(Error::plain(format!(
                        "module {m} is declared in both {} and {key}",
                        prev.display()
                    )));
                }
            }
            module_file.insert(m.clone(), path.clone());
            imports.insert(m.clone(), imps.iter().map(|(n, _)| n.clone()).collect());
        }
        sources.insert(key, src);
        let prelude_names = prelude_module_names();
        for (owner_module, imps) in &scanned.modules {
            for (imp, stage) in imps {
                // A module another package owns is importable only when
                // that package exposed it (section 4.2): an internal
                // module is compiled as part of its package and is not
                // part of the surface.
                if let (Some(declarer), Some(package_is_exposed)) =
                    (owner.get(imp), exposed.get(imp))
                {
                    let importer_package = owner.get(owner_module);
                    let crosses = importer_package != Some(declarer);
                    if crosses && !*package_is_exposed {
                        return Err(Error::plain(format!(
                            "module {imp} is declared by package {declarer} and is not exposed; \
                             the package's internal module cannot be imported from {owner_module}"
                        )));
                    }
                }
                if module_file.contains_key(imp) {
                    continue;
                }
                // Availability is a per-profile fact (design/base-as-a-package.md,
                // "Host and target are different questions"): a module a
                // manifest declares but this profile's resolution leaves
                // out is not importable here, whatever file the directory
                // happens to hold for it.
                if let Some(declarer) = availability.owner(imp) {
                    let emittable = availability
                        .modules(crate::core::BuildContext::Emitted)
                        .contains(imp);
                    if !emittable {
                        let host_answer = availability
                            .available_for(crate::core::BuildContext::CompileTime, imp);
                        if host_answer == Some(true) {
                            // It resolves for the host only. The target's
                            // module graph has no place for it, so the edge
                            // is recorded and the context that reached it
                            // decides the refusal.
                            host_only.push(HostOnlyImport {
                                importer: owner_module.clone(),
                                module: imp.clone(),
                                stage: *stage,
                            });
                            continue;
                        }
                        let host_note = match host_answer {
                            Some(false) => format!(
                                " nor for the build host {} ({})",
                                build.host.triple, build.host.os
                            ),
                            _ => String::new(),
                        };
                        return Err(Error::plain(format!(
                            "module {imp} is not available for target {} ({}){host_note}: \
                             package {declarer} {}; either build for a target the condition \
                             selects, or widen the package's conditions so {imp} resolves for {}",
                            build.target.triple,
                            build.target.os,
                            availability.condition_account(imp),
                            build.target.triple
                        )));
                    }
                }
                // A source module beside the entry shadows the embedded
                // prelude copy of the same name (the implicit base layer
                // loses to explicit source), so the disk is searched
                // first; only an unresolved family name falls back to
                // the embedded sources.
                let direct = dir.join(format!("{imp}.cinch"));
                if direct.exists() {
                    queue.push(direct);
                    continue;
                }
                let candidates = dir_index.get(imp).cloned().unwrap_or_default();
                let not_loaded: Vec<PathBuf> = candidates
                    .into_iter()
                    .filter(|p| !sources.contains_key(&p.display().to_string()))
                    .collect();
                match not_loaded.len() {
                    1 => queue.push(not_loaded.into_iter().next().expect("one")),
                    0 => {
                        if prelude_names.iter().any(|n| n == imp) {
                            continue;
                        }
                        return Err(Error::plain(format!(
                            "module {imp} is imported, but {} does not exist and no other source file beside the entry declares `module {imp}`",
                            direct.display()
                        )))
                    }
                    _ => {
                        return Err(Error::plain(format!(
                            "module {imp} is imported, but several source files declare it ({}); imports resolve to a unique module",
                            not_loaded
                                .iter()
                                .map(|p| p.display().to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )))
                    }
                }
            }
        }
    }
    // The injected prelude is a real dependency (design/base-as-a-package.md
    // section 3): it belongs in the module's import set, so the check
    // order and the interface closure see it. Without the edge a module
    // would name things it never imported and the coherence check would
    // reject the result.
    for (module, prelude) in &module_preludes {
        if module == prelude {
            continue;
        }
        if let Some(list) = imports.get_mut(module) {
            if !list.iter().any(|i| i == prelude) {
                list.push(prelude.clone());
            }
        }
    }
    // Keep dependency order within a file too: its first module may
    // import one declared later in the same file. Entry membership is
    // recorded separately and must not undo the graph's ordering.
    let graph: std::collections::BTreeMap<String, Vec<crate::interface::IfaceRef>> = imports
        .iter()
        .map(|(m, imps)| {
            (
                m.clone(),
                imps.iter()
                    .filter(|i| *i != m && module_file.contains_key(*i))
                    .map(|i| crate::interface::IfaceRef {
                        module: i.clone(),
                        iface_hash: String::new(),
                    })
                    .collect(),
            )
        })
        .collect();
    let mut order =
        crate::interface::topo_order(&graph).map_err(|e| Error::plain(e.to_string()))?;
    order.retain(|m| module_file.contains_key(m));
    Ok(Project {
        module_file,
        sources,
        imports,
        entry_modules,
        order,
        packages,
        owner,
        exposed,
        module_preludes,
        availability,
        host_only,
    })
}

/// The modules whose definitions run during the build: a `stage` or
/// `macro` declaration is compile-time code, and so is every module such a
/// module imports (design/base-as-a-package.md, "Host and target are
/// different questions"). A conditional definition counts where its
/// condition holds for the target, because that is the program the build
/// actually compiles; the `when` clause is read against the emitted
/// context, the same decision the scanner and the checker make.
fn compile_time_roots(
    modules: &[crate::ast::Module],
    imports: &HashMap<String, Vec<String>>,
    build: &crate::core::BuildProfiles,
) -> Vec<String> {
    let profile = crate::core::condition_context(build, crate::core::BuildContext::Emitted);
    // A `stage` or `macro` definition, wherever conditions wrap it. A
    // conditional definition counts where its condition holds for the
    // target, because that is the program this build compiles.
    fn staged(d: &crate::ast::Decl, profile: &crate::core::TargetProfile) -> bool {
        match d {
            crate::ast::Decl::Conditional { when, inner } => {
                crate::core::WhenCond::lower(when)
                    .map(|c| c.holds(profile))
                    .unwrap_or(false)
                    && staged(inner, profile)
            }
            crate::ast::Decl::Attributed { inner, .. }
            | crate::ast::Decl::NameSpliced { inner, .. } => staged(inner, profile),
            crate::ast::Decl::ValueSig { modifier: Some(m), .. }
            | crate::ast::Decl::Equation { modifier: Some(m), .. } => {
                matches!(m.text.as_str(), "stage" | "macro")
            }
            _ => false,
        }
    }
    let mut roots: Vec<String> = Vec::new();
    for m in modules {
        let name = m.name.text.clone();
        if m.decls.iter().any(|d| staged(d, profile)) {
            roots.push(name);
            continue;
        }
        // `import stage M` puts the importing module in the compile-time
        // path, because only a staged definition can see such a name.
        if let Some(imported) = imports.get(&name) {
            if m.imports.iter().any(|i| {
                i.stage && imported.iter().any(|n| *n == i.path.parts().join("."))
            }) {
                roots.push(name);
            }
        }
    }
    roots
}

/// Refuse a module the compile-time path reaches that the build cannot
/// compile (design/base-as-a-package.md, "Host and target are different
/// questions"). Two shapes exist, and the refusal says which: a module the
/// host resolves and the target does not (the build writes the target's
/// module set, so there is nothing to compile), and a module the target
/// resolves and the host does not (compile-time code runs on the host).
///
/// Ordinary code reaching a host-only module is the same fact without the
/// compile-time context, and it is refused as unavailable for the target,
/// naming the condition that would select it.
fn refuse_unavailable_modules(
    project: &Project,
    modules: &[crate::ast::Module],
    build: &crate::core::BuildProfiles,
) -> Result<()> {
    let availability = &project.availability;
    if availability.host_unresolved.is_some() {
        // The host resolution could not be computed, so claiming a module
        // is unavailable for the host would be a refusal on no evidence.
        // The manifest records the reason instead.
        return Ok(());
    }
    let roots = compile_time_roots(modules, &project.imports, build);
    // The compile-time closure: every module a staged module reaches
    // through the target's import graph, with the staged module that
    // reaches it.
    let mut closure: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut queue: std::collections::VecDeque<(String, String)> = std::collections::VecDeque::new();
    for root in &roots {
        if closure.insert(root.clone(), root.clone()).is_none() {
            queue.push_back((root.clone(), root.clone()));
        }
    }
    // A host-only import written with `import stage` is compile-time code
    // wherever it lives, so its importer joins the closure too.
    for edge in &project.host_only {
        if edge.stage && closure.insert(edge.importer.clone(), edge.importer.clone()).is_none() {
            queue.push_back((edge.importer.clone(), edge.importer.clone()));
        }
    }
    while let Some((module, root)) = queue.pop_front() {
        for next in project.imports.get(&module).into_iter().flatten() {
            if closure.contains_key(next) {
                continue;
            }
            closure.insert(next.clone(), root.clone());
            queue.push_back((next.clone(), root.clone()));
        }
    }
    let emittable = |module: &str| -> bool {
        availability
            .modules(crate::core::BuildContext::Emitted)
            .contains(module)
    };
    let host_answer = |module: &str| -> Option<bool> {
        availability.available_for(crate::core::BuildContext::CompileTime, module)
    };
    // A module reached only from ordinary code that the target cannot
    // compile: refused as unavailable for the target.
    for edge in &project.host_only {
        if edge.stage || closure.contains_key(&edge.importer) {
            continue;
        }
        return Err(Error::plain(format!(
            "module {} is not available for target {} ({}): package {} {}; the build host \
             {} ({}) resolves it, so the import works for a host build and not for this \
             target. Either build for a target the condition selects, or widen the \
             package's conditions so {} resolves for {}. Ordinary code is compiled for the \
             target, so it cannot import a module the target's resolution leaves out \
             (design/base-as-a-package.md section 4.2)",
            edge.module,
            build.target.triple,
            build.target.os,
            availability.owner(&edge.module).unwrap_or("(no manifest)"),
            availability.condition_account(&edge.module),
            build.host.triple,
            build.host.os,
            edge.module,
            build.target.triple
        )));
    }
    // The compile-time path: a module it reaches must resolve for the
    // host, because that is where the code runs, and must resolve for the
    // target, because the build compiles the target's module set.
    let compile_time_refusal = |module: &str, reached_by: &str| -> String {
        let owner = availability.owner(module).unwrap_or("(no manifest)");
        let condition = availability.condition_account(module);
        if host_answer(module) == Some(true) && !emittable(module) {
            format!(
                "module {module} is reached from compile-time code ({reached_by}) and is not \
                 available for both profiles of this build: it resolves for the host {} ({}) \
                 and not for the target {} ({}). Package {owner} {condition}, and the build \
                 compiles the target's module set, so compile-time code can only reach a \
                 module that resolves for both. Fix it one of two ways: move the code that \
                 needs {module} out of the build-time path, or widen {owner}'s conditions so \
                 {module} also resolves for the target {} \
                 (design/base-as-a-package.md, host and target are different questions)",
                build.host.triple,
                build.host.os,
                build.target.triple,
                build.target.os,
                build.target.triple
            )
        } else {
            format!(
                "module {module} is reached from compile-time code ({reached_by}) and is not \
                 available for both profiles of this build: it resolves for the target {} ({}) \
                 and not for the host {} ({}). Compile-time code runs during the build, on the \
                 host, so a module it reaches must be host-available. Package {owner} \
                 {condition}. Fix it one of two ways: move the code that needs {module} out of \
                 the build-time path, or widen {owner}'s conditions so {module} also resolves \
                 for the host {} (design/base-as-a-package.md, host and target are different \
                 questions)",
                build.target.triple,
                build.target.os,
                build.host.triple,
                build.host.os,
                build.host.triple
            )
        }
    };
    for edge in &project.host_only {
        if !(edge.stage || closure.contains_key(&edge.importer)) {
            continue;
        }
        let root = closure.get(&edge.importer).cloned().unwrap_or_default();
        let reached_by = if edge.stage {
            format!("{} imports it `stage`", edge.importer)
        } else if root != edge.importer {
            format!(
                "{} reaches it through the module graph from the compile-time code in {}",
                edge.importer, root
            )
        } else {
            edge.importer.clone()
        };
        return Err(Error::plain(compile_time_refusal(&edge.module, &reached_by)));
    }
    for (module, root) in &closure {
        if host_answer(module) == Some(true) {
            continue;
        }
        if !emittable(module) {
            continue;
        }
        let reached_by = if root == module {
            format!("{root} holds the definition that uses it")
        } else {
            format!("{root}'s compile-time code reaches it through the module graph")
        };
        return Err(Error::plain(compile_time_refusal(module, &reached_by)));
    }
    Ok(())
}

/// One escaped snapshot line: `kind \t snapshot \t name \t hash \t live \t
/// captured`, with the captured content escaped for one line.
fn escape_field(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out
}

fn unescape_field(s: &str) -> String {
    let mut out = String::new();
    let mut cs = s.chars();
    while let Some(c) = cs.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match cs.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some(other) => out.push(other),
            None => {}
        }
    }
    out
}

/// The captured inputs of a previous build (replay mode), from the
/// snapshot file beside the manifest.
fn read_stage_snap(path: &Path) -> Vec<StageInput> {
    let mut inputs = Vec::new();
    let Ok(text) = std::fs::read_to_string(path) else {
        return inputs;
    };
    for line in text.lines() {
        let parts: Vec<&str> = line.splitn(6, '\t').collect();
        if parts.len() < 5 {
            continue;
        }
        inputs.push(StageInput {
            kind: parts[0].to_string(),
            snapshot: parts[1].to_string(),
            name: unescape_field(parts[2]),
            hash: parts[3].to_string(),
            live: parts[4] == "live",
            captured: parts.get(5).map(|s| unescape_field(s)).unwrap_or_default(),
        });
    }
    inputs
}

/// Write the per-module interfaces, the artifact manifest, the stage
/// snapshot, and the package store entry (design note sections 2 and 5;
/// modules specification section 10: the build identity includes
/// generator versions and input hashes; supercompilation and JIT
/// specification sections 5, 8 and 9: the specialization configuration
/// and trust level; platform proofs and SIMD specification sections 1
/// and 8: the target conditions and which lowering was checked).
#[allow(clippy::too_many_arguments)]
fn write_artifacts(
    out_dir: &Path,
    entry: &str,
    ifaces: &[crate::interface::Iface],
    manifest_path: &Path,
    report: &StageReport,
    program: &crate::core::Program,
    opt: &crate::core::OptReport,
    profile: &crate::core::TargetProfile,
    build: &crate::core::BuildProfiles,
    availability: &ModuleAvailability,
    wit: &[crate::interface::WitAdapter],
    opts: &DriverOptions,
    link_plan: &LinkPlan,
    write_optimisations: bool,
) -> Result<()> {
    std::fs::create_dir_all(out_dir)
        .map_err(|e| Error::plain(format!("{}: {e}", out_dir.display())))?;
    // The optimisation artifact (`crate::ifaceopt`): the unfoldable bodies
    // of this module's small units, keyed by the interface identity written
    // below. The class pass and the encoding are only paid for when this
    // compile is a library producer (or the caller asked), so a check of a
    // user program does not run them.
    let unfoldable = if write_optimisations {
        Some(crate::simplify::unfoldable_names(program))
    } else {
        None
    };
    for iface in ifaces {
        let p = out_dir.join(format!("{}.cinchif", iface.module));
        std::fs::write(&p, iface.to_json())
            .map_err(|e| Error::plain(format!("{}: {e}", p.display())))?;
        let opt_path = out_dir.join(format!("{}.cinchopt", iface.module));
        match &unfoldable {
            None => {
                let _ = std::fs::remove_file(&opt_path);
            }
            Some(class) => {
                let optimisation = crate::ifaceopt::for_module_in(iface, program, class);
                if optimisation.bodies.is_empty() {
                    let _ = std::fs::remove_file(&opt_path);
                } else {
                    std::fs::write(&opt_path, optimisation.to_json()).map_err(|e| {
                        Error::plain(format!("{}: {e}", opt_path.display()))
                    })?;
                }
            }
        }
    }
    let mut modules: Vec<(&str, String)> = ifaces
        .iter()
        .map(|i| (i.module.as_str(), i.iface_hash()))
        .collect();
    modules.sort();
    let mut prims: Vec<String> = Vec::new();
    let mut foreign: Vec<String> = Vec::new();
    let mut declared: Vec<String> = Vec::new();
    for i in ifaces {
        prims.extend(i.assumptions.primitives.iter().cloned());
        foreign.extend(i.assumptions.foreign.iter().cloned());
        declared.extend(i.assumptions.declared.iter().cloned());
    }
    // A rewrite's justification identities are declared assumptions when
    // no instance discharges them (decision D55).
    for r in &program.rewrites {
        for a in &r.assumptions {
            declared.push(a.clone());
        }
    }
    prims.sort();
    prims.dedup();
    foreign.sort();
    foreign.dedup();
    declared.sort();
    declared.dedup();
    use crate::interface::JsonValue as J;
    // `prim` declarations (primitive operations and inline target IR
    // specification section 8): per prim the selected body's family and
    // condition, its tier, its `uses` list and its validation result.
    // Selection is manifest-only: it never enters the contract
    // projection (section 6).
    let prims_rows = J::arr(&program.prims, |p| {
        let body = &p.bodies[p.selected];
        let uses: Vec<String> = body
            .uses
            .iter()
            .map(|u| format!("{} :: {}", u.symbol, u.ty))
            .collect();
        let validations: Vec<String> = p
            .bodies
            .iter()
            .map(|b| format!("{}: {}", b.family, b.validation))
            .collect();
        // The manifest trust rows (primitive operations and inline
        // target IR specification section 12): each `unsafe` body's
        // declared row is trusted, not verified, with its per-body
        // assumption identity.
        let trust: Vec<String> = p
            .bodies
            .iter()
            .filter(|b| b.trusted)
            .map(|b| {
                format!(
                    "{}: declared under `unsafe`, trusted not verified ({})",
                    b.family, b.trust_id
                )
            })
            .collect();
        J::obj([
            ("name", (&p.name).into()),
            ("module", (&p.module).into()),
            (
                "tier",
                match p.tier {
                    crate::core::PrimTier::Justified => "justified",
                    crate::core::PrimTier::Assumed => "assumed",
                }
                .into(),
            ),
            ("family", (&body.family).into()),
            ("condition", body.when.as_ref().into()),
            ("why", (&p.selected_why).into()),
            ("uses", uses.into()),
            ("immarg", (&body.immarg).into()),
            ("validation", validations.into()),
            ("trust", trust.into()),
            ("identity", (&p.identity).into()),
        ])
    });
    // The class law check rows (section 5): what the sample-wise check
    // decided, honestly (`checked` over its samples or `not checked` at
    // the law machinery's boundary).
    let law_rows = J::arr(&program.laws, |l| {
        J::obj([
            ("instance", (&l.instance).into()),
            ("class", (&l.class).into()),
            ("law", (&l.law).into()),
            ("status", (&l.status).into()),
        ])
    });
    let fired = |name: &str| {
        opt.sites
            .iter()
            .any(|s| s.kind == format!("rewrite:{name}") && s.stop.is_none())
    };
    // The target profile and the conditions the target rules discharged
    // (platform proofs and SIMD specification sections 1-3; modules and
    // compilation specification section 13: target conditions are
    // preserved in the manifest).
    let conditions = J::arr(program.rewrites.iter().filter(|r| r.target), |r| {
        let condition = match &r.cond {
            crate::core::RequireCond::CapabilityInScope { class, .. } => {
                format!("capability {class} valid in the firing region's validity scope")
            }
            crate::core::RequireCond::Always => "always".to_string(),
            crate::core::RequireCond::Never => "never".to_string(),
            crate::core::RequireCond::Unknown => "decided at the match site".to_string(),
        };
        J::obj([
            ("rule", (&r.name).into()),
            ("condition", condition.into()),
            ("discharged", fired(&r.name).into()),
        ])
    });
    let target = J::obj([
        ("profile", (&profile.name).into()),
        // The target the artifact was built for, recorded because a
        // package manifest's conditions resolve against it
        // (design/base-as-a-package.md section 4.1) and a consumer has to
        // be able to read which target an artifact answers for.
        ("triple", profile.triple.as_str().into()),
        ("os", profile.os.as_str().into()),
        ("arch", profile.arch.as_str().into()),
        ("libc", profile.libc.as_str().into()),
        (
            "os_version",
            profile
                .os_version
                .as_deref()
                .map(J::from)
                .unwrap_or(J::Null),
        ),
        ("word_size", profile.word_size.into()),
        ("endian", (&profile.endian).into()),
        (
            "vector_shape",
            match profile.vector_shape {
                crate::core::VectorShape::Fixed(w) => format!("fixed({w})"),
                crate::core::VectorShape::Scalable => "scalable".to_string(),
            }
            .into(),
        ),
        ("features", (&profile.features).into()),
        (
            "admission",
            "the deployment contract must satisfy the recorded feature assumptions".into(),
        ),
        ("conditions", conditions),
    ]);
    // The build's two profiles (design/base-as-a-package.md, "Host and
    // target are different questions"). The host is where compile-time
    // code runs and the target is what the artifact answers for, so a
    // reader has to be able to tell a cross build from a native one and to
    // see which modules the split separated. The availability rows record
    // each declared module against both profiles, because that is the fact
    // a compile-time reachability refusal quotes.
    let module_rows = availability.declared_modules();
    let host_record = J::obj([
        ("profile", (&build.host.name).into()),
        ("triple", build.host.triple.as_str().into()),
        ("os", build.host.os.as_str().into()),
        ("arch", build.host.arch.as_str().into()),
        ("libc", build.host.libc.as_str().into()),
        (
            "os_version",
            build.host.os_version.as_deref().map(J::from).unwrap_or(J::Null),
        ),
        (
            "os_version_from",
            if availability.host_os_version_from_target {
                "target: the compiler cannot ask the host for its platform version"
            } else if build.host.os_version.is_some() {
                "the host profile"
            } else {
                "none stated"
            }
            .into(),
        ),
        (
            "resolution",
            match &availability.host_unresolved {
                None => "resolved".into(),
                Some(reason) => J::obj([
                    ("state", "unresolved".into()),
                    ("reason", reason.into()),
                    (
                        "consequence",
                        "no module is reported unavailable for the host on this evidence: \
                         its availability for the host is `undecided`, and a refusal needs \
                         evidence"
                            .into(),
                    ),
                ]),
            },
        ),
    ]);
    let profiles = J::obj([
        ("cross", build.cross().into()),
        ("host", host_record),
        (
            "target",
            build.target.triple.as_str().into(),
        ),
        (
            "condition_context",
            "emitted code and its modules resolve against the target; compile-time code \
             (staged units, macros, and the modules they import) resolves against the host. A \
             `prim` body is emitted code and resolves against the target wherever it is reached"
                .into(),
        ),
        (
            "interface_cache",
            "keyed by the profile that selected the entry: an interface compiled for the \
             target is keyed by the target profile (crate::store::package_key_for folds \
             profile_identity(target) into the key), and the host resolution writes no \
             interface and no cache entry, because it compiles nothing"
                .into(),
        ),
        (
            "modules",
            J::arr(&module_rows, |module| {
                J::obj([
                    ("module", module.as_str().into()),
                    (
                        "host",
                        availability
                            .availability_spelling(crate::core::BuildContext::CompileTime, module)
                            .into(),
                    ),
                    (
                        "target",
                        availability
                            .availability_spelling(crate::core::BuildContext::Emitted, module)
                            .into(),
                    ),
                ])
            }),
        ),
    ]);
    // The Wasm and components record (WebAssembly and WASI
    // specification sections 2, 4 and 8): the artifact shape, the
    // required and available capabilities, the loader's selected feature
    // profile, the lowerings the build chose, and each generated
    // component adapter's canonical table.
    let mut required: Vec<String> = profile.required.clone();
    if !wit.is_empty() && profile.artifact != crate::core::ArtifactProfile::Native {
        // A component adapter requires the component model on a Wasm
        // artifact (the requirement is recorded even when the profile's
        // capabilities satisfy it).
        required.push("wasm.component-model".to_string());
    }
    let mut lowerings: Vec<String> = vec![format!(
        "tasks:{}",
        if profile.artifact == crate::core::ArtifactProfile::Native {
            "os-thread"
        } else {
            "cooperative"
        }
    )];
    for r in &program.rewrites {
        if r.target && fired(&r.name) {
            lowerings.push(format!("rewrite:{}", r.name));
        }
    }
    let canon = match wit.first() {
        Some(a) => a.abi.spelling(),
        None => "none",
    };
    let mut adapter_rows: Vec<J> = Vec::new();
    for a in wit {
        for b in &a.bindings {
            let mut row = J::obj([
                ("package", (&a.package).into()),
                ("module", (&a.module).into()),
                ("name", (&b.name).into()),
                ("symbol", (&b.symbol).into()),
                ("export", b.export.into()),
                (
                    "core_params",
                    J::arr(&b.call.core_params, |t| t.spelling().into()),
                ),
                (
                    "core_results",
                    J::arr(&b.call.core_results, |t| t.spelling().into()),
                ),
            ]);
            let J::Obj(fields) = &mut row else {
                unreachable!()
            };
            if let Some(ret) = &b.call.ret {
                fields.insert(
                    "ret_area".to_string(),
                    J::obj([
                        ("size", ret.size.into()),
                        ("align", ret.align.into()),
                        ("cases", (ret.cases.len() as u64).into()),
                    ]),
                );
            }
            if let Some(g) = &b.call.glue_gap {
                fields.insert("glue_gap".to_string(), g.into());
            }
            adapter_rows.push(row);
        }
    }
    let wasm = J::obj([
        ("artifact", profile.artifact.spelling().into()),
        ("canonical_abi", canon.into()),
        ("required", required.into()),
        ("available", (&profile.features).into()),
        (
            "selected",
            J::obj([
                ("profile", (&profile.name).into()),
                ("features", (&profile.features).into()),
            ]),
        ),
        ("lowerings", lowerings.into()),
        ("adapters", J::Arr(adapter_rows)),
    ]);
    // Specialization configuration and report (supercompilation and JIT
    // specification sections 5 and 8).
    let specialization = J::obj([
        ("ran", true.into()),
        ("fuel", profile.fuel.into()),
        ("measure", profile.measure.into()),
        ("whistle", "repetition-or-measure".into()),
        ("runtime_codegen", false.into()),
        (
            "sites",
            J::arr(&opt.sites, |s| {
                J::obj([
                    ("unit", (&s.fn_id).into()),
                    ("kind", (&s.kind).into()),
                    ("stop", s.stop.as_ref().into()),
                ])
            }),
        ),
        ("fused", (&opt.fused).into()),
        ("discarded", (&opt.discarded).into()),
        // Option C's representation copies, one per reached class vector.
        ("representation_versions", (&opt.versions).into()),
        ("representation_unrecorded", (opt.unrecorded as u64).into()),
    ]);
    // The Core simplifier's budget use (design/compilation-speed.md
    // section 10): the cap it was given, the passes it ran, and the tick
    // count per rule, so a change in the pass's work is visible in the
    // artifact rather than inferred from IR size.
    let simplifier = J::obj([
        ("max_iterations", (opt.simplify.max_iterations as u64).into()),
        ("iterations", (opt.simplify.iterations as u64).into()),
        ("ticks", (opt.simplify.ticks as u64).into()),
        ("case_merge", (opt.simplify.case_merge as u64).into()),
        ("known_constructor", (opt.simplify.known_ctor as u64).into()),
        ("known_constructor_computed", (opt.simplify.known_ctor_once as u64).into()),
        ("projection", (opt.simplify.projection as u64).into()),
        ("join_scope", (opt.simplify.join_scope as u64).into()),
        ("direct_dict_call", (opt.simplify.dict_call as u64).into()),
        ("saturate_partial", (opt.simplify.saturate as u64).into()),
        ("case_of_case", (opt.simplify.case_of_case as u64).into()),
        ("joins", (opt.simplify.joins as u64).into()),
        ("duplicated", (opt.simplify.duplicated as u64).into()),
        ("inlined", (opt.simplify.inlined as u64).into()),
        ("inlined_nodes", (opt.simplify.inlined_nodes as u64).into()),
        ("pruned", (opt.simplify.pruned as u64).into()),
        ("pruned_nodes", (opt.simplify.pruned_nodes as u64).into()),
    ]);
    // The inlining decision's own verdict (crate::simplify, benefit model):
    // which model ran, the sites it inlined, and the sites it refused per
    // reason. A site lands in exactly one bucket, so the counters say what
    // the rule did rather than leaving it to be inferred from IR size.
    let b = &opt.simplify.benefit;
    let inlining = J::obj([
        ("model", opt.simplify.inline_model.into()),
        ("inlined", (b.inlined as u64).into()),
        ("refused", (b.refused() as u64).into()),
        ("refused_large", (b.refused_large as u64).into()),
        ("refused_size", (b.refused_size as u64).into()),
        ("refused_recursive", (b.refused_recursive as u64).into()),
        ("refused_value", (b.refused_value as u64).into()),
        ("refused_scope", (b.refused_scope as u64).into()),
        ("refused_fold", (b.refused_fold as u64).into()),
        ("refused_chain", (b.refused_chain as u64).into()),
        ("refused_budget", (b.refused_budget as u64).into()),
    ]);
    // Tail-call status of the recursive call-graph groups (surface
    // grammar local amendments section 9; decision D104): "marked" when
    // the group's self and mutual tail calls are jumps or `musttail`
    // calls (constant stack), "fallback (<reason>)" when one is an
    // ordinary call (the reason names the call).
    let tail_calls = J::obj([(
        "groups",
        J::arr(&crate::core::tail_report(program), |g| {
            J::obj([("members", (&g.members).into()), ("mark", (&g.mark).into())])
        }),
    )]);
    // The collection literal fold boundary (surface grammar local
    // amendments section 10; decision D110): each literal's site, its
    // construction's kind, and whether the compile-time fold made it a
    // constant value or it keeps its runtime `from-list`/`from-map` call.
    let literal_folds = J::obj([(
        "sites",
        J::arr(&report.literal_folds, |f| {
            J::obj([
                ("unit", (&f.unit).into()),
                ("site", (&f.site).into()),
                ("kind", (&f.kind).into()),
                ("folded", f.folded.into()),
            ])
        }),
    )]);
    // The `import data` bindings' provenance (surface grammar local
    // amendments section 11): the converter identity and the file's
    // content hash per binding. The import's contract covers the
    // conversion, not the file bytes; the manifest records hashes, never
    // contents.
    let data_imports = J::obj([(
        "bindings",
        J::arr(&report.data_imports, |d| {
            J::obj([
                ("module", (&d.module).into()),
                ("name", (&d.name).into()),
                ("file", (&d.file).into()),
                ("converter", (&d.converter).into()),
                ("hash", (&d.hash).into()),
            ])
        }),
    )]);
    // P20's safety assertion summary (attribute annotations
    // specification section 10): per definition, the assertion, its
    // verification result, and for `bounded(x)` which proof method
    // established the bound (decision D113); plus the `jpl` profile's
    // whole-program summary (decision D114). Definition names, never
    // paths.
    let safety = J::obj([
        (
            "assertions",
            J::arr(&report.safety.assertions, |a| {
                J::obj([
                    ("definition", (&a.definition).into()),
                    ("attribute", (&a.attribute).into()),
                    ("result", (&a.result).into()),
                ])
            }),
        ),
        (
            "jpl",
            match &report.safety.jpl {
                Some(j) => J::obj([
                    ("modules", (&j.modules).into()),
                    ("definitions", j.definitions.into()),
                    ("assertions", j.assertions.into()),
                ]),
                None => J::Null,
            },
        ),
    ]);
    // The verified temporal contracts (core specification section 10):
    // per contract, the definition it names, the kind and formula, the
    // verification result (which argument discharged it), and the stated
    // assumptions (a liveness claim relies on the runtime policies it
    // names). Definition names, never paths.
    let temporal = J::obj([(
        "contracts",
        J::arr(&report.temporal, |t| {
            J::obj([
                ("definition", (&t.definition).into()),
                ("contract", (&t.contract).into()),
                ("kind", (&t.kind).into()),
                ("formula", (&t.formula).into()),
                ("result", (&t.result).into()),
                ("assumptions", (&t.assumptions).into()),
            ])
        }),
    )]);
    // Trust level (supercompilation and JIT specification section 9:
    // full recheck is the default) and the checked-versus-trusted
    // lowering split (platform proofs and SIMD specification section 8).
    let trust = J::obj([
        ("level", 2u64.into()),
        ("name", "full-recheck".into()),
        ("rechecked", (&opt.rechecked).into()),
        (
            "trusted",
            vec![
                "type preservation by construction",
                "effect-order preservation by construction",
            ]
            .into(),
        ),
    ]);
    let lowering = J::obj([
        (
            "checked",
            vec![
                "core structure and scoping",
                "usage of %1/%aff parameters",
                "memo and demand sharing",
                "decision-point weights",
                "contract identity",
            ]
            .into(),
        ),
        (
            "trusted",
            vec![
                "LLVM code generation",
                "the runtime and its instruction model",
            ]
            .into(),
        ),
    ]);
    // Staging build identity (modules specification section 10): the mode,
    // the compiler version, generated declarations, and every captured
    // input with its hash. Secrets record their name only.
    let staging = J::obj([
        (
            "mode",
            match report.mode {
                StageMode::Refresh => "refresh",
                StageMode::Replay => "replay",
                StageMode::Live => "live",
            }
            .into(),
        ),
        ("compiler", env!("CARGO_PKG_VERSION").into()),
        (
            "generated",
            J::Obj(
                report
                    .generated
                    .iter()
                    .map(|(m, names)| (m.clone(), names.into()))
                    .collect(),
            ),
        ),
        (
            "inputs",
            J::arr(&report.inputs, |input| {
                J::obj([
                    ("kind", (&input.kind).into()),
                    ("snapshot", (&input.snapshot).into()),
                    ("name", (&input.name).into()),
                    ("hash", (&input.hash).into()),
                    ("live", input.live.into()),
                ])
            }),
        ),
    ]);
    let doc = J::obj([
        ("entry", entry.into()),
        ("runtime_abi", crate::interface::RUNTIME_ABI_VERSION.into()),
        (
            "modules",
            J::arr(&modules, |(m, h)| {
                J::obj([("module", (*m).into()), ("iface_hash", h.into())])
            }),
        ),
        (
            "assumptions",
            J::obj([
                ("primitives", prims.into()),
                ("foreign", foreign.into()),
                ("declared", declared.into()),
            ]),
        ),
        ("prims", prims_rows),
        ("laws", law_rows),
        ("target", target),
        ("profiles", profiles),
        ("wasm", wasm),
        ("specialization", specialization),
        ("simplifier", simplifier),
        ("inlining", inlining),
        ("tail_calls", tail_calls),
        ("literal_folds", literal_folds),
        ("data_imports", data_imports),
        ("safety", safety),
        ("temporal", temporal),
        ("trust", trust),
        ("lowering", lowering),
        ("staging", staging),
        (
            "link",
            J::obj([
                ("local", link_plan.local.into()),
                (
                    "raw",
                    J::arr(&link_plan.raw_origins(), |origin| origin.as_str().into()),
                ),
                ("words", J::arr(&link_plan.words(), |word| word.as_str().into())),
            ]),
        ),
    ]);
    std::fs::write(manifest_path, doc.to_canonical_string())
        .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
    // The snapshot file carries the captured input contents for replay.
    let snap_path = out_dir.join(format!("{entry}.stage.snap"));
    let mut snap = String::new();
    for input in &report.inputs {
        snap.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            input.kind,
            input.snapshot,
            escape_field(&input.name),
            input.hash,
            if input.live { "live" } else { "recorded" },
            escape_field(&input.captured)
        ));
    }
    std::fs::write(&snap_path, snap)
        .map_err(|e| Error::plain(format!("{}: {e}", snap_path.display())))?;
    // The package artifact store: this compile's interfaces for the
    // embedded base package, cached per user and keyed on the compile's
    // inputs (see `crate::store`). The compile path only writes here, and
    // never fails a build over an entry: a root that cannot be created,
    // or a store that cannot be written (a read-only cache directory, a
    // full disk), degrades to compiling without a store, and an entry
    // that is not this compile's artifact set is rewritten rather than
    // trusted. The hard errors belong to the consuming path
    // (`Store::load`), where a caller is about to rely on the artifacts.
    // A package that cannot be resolved to its sources is the one fault
    // worth reporting: the embedded manifest is pinned against the
    // embedded sources, so it cannot happen, and silently ignoring it
    // would hide a real inconsistency.
    let store = crate::store::Store::user();
    if store.ensure_root().is_ok() {
        if let Err(e) = store.write(base_package(), profile, wit, ifaces, opts.rebuild_store) {
            if matches!(&e, crate::store::StoreError::Package { .. }) {
                return Err(Error::plain(e.to_string()));
            }
        }
    }
    Ok(())
}

/// One variant of a packaged distribution (WebAssembly and WASI
/// specification section 8): a separate module with its own required
/// capabilities and selected lowerings, chosen by the loader before
/// instantiation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantPackage {
    pub name: String,
    /// Capabilities this variant's artifact requires.
    pub required: Vec<String>,
    /// The lowerings the variant selected (fired target rules and the
    /// task ABI), recorded beside the requirements.
    pub lowerings: Vec<String>,
    /// The artifact identity: the hash of its emitted module text.
    pub ir_hash: String,
}

/// A packaged distribution: the baseline and feature-specific modules of
/// one program (WebAssembly and WASI specification section 8).
#[derive(Clone, Debug, Default)]
pub struct Distribution {
    /// In packaging order: the most preferred variant first.
    pub variants: Vec<VariantPackage>,
}

impl Distribution {
    /// The loader's choice before instantiation: the first variant (in
    /// packaging order) whose requirements the available capabilities
    /// satisfy. A distribution with no satisfiable variant is refused,
    /// never silently downgraded.
    pub fn select<'a>(&'a self, available: &[String]) -> Result<&'a VariantPackage> {
        self.variants
            .iter()
            .find(|v| v.required.iter().all(|r| available.iter().any(|a| a == r)))
            .ok_or_else(|| {
                let needed: Vec<&str> =
                    self.variants.iter().flat_map(|v| v.required.iter().map(|s| s.as_str())).collect();
                Error::plain(format!(
                    "no packaged variant is satisfiable: the loader's capabilities cover none of {needed:?}"
                ))
            })
    }

    /// The loader record (section 8: the loader records the selected
    /// feature profile): the variant table and the selection, as JSON
    /// for the artifact manifest.
    pub fn loader_record(&self, selected: &str, available: &[String]) -> String {
        use crate::interface::JsonValue as J;
        J::obj([
            ("selected", selected.into()),
            ("available", available.into()),
            (
                "variants",
                J::arr(&self.variants, |v| {
                    J::obj([
                        ("name", (&v.name).into()),
                        ("required", (&v.required).into()),
                        ("lowerings", (&v.lowerings).into()),
                        ("ir_hash", (&v.ir_hash).into()),
                    ])
                }),
            ),
        ])
        .to_canonical_string()
    }
}

/// Package one program as the named variants (WebAssembly and WASI
/// specification section 8: separate baseline and feature-specific
/// modules selected before instantiation). Each variant compiles under
/// its own target profile; the artifact identity is the emitted module's
/// content hash. Packaging is deterministic: the same sources and
/// profiles produce the same identities, so a reload reproduces the
/// loader's choice.
pub fn package_variants(
    user_src: &str,
    user_file: &str,
    variants: &[(&str, crate::core::TargetProfile)],
) -> Result<Distribution> {
    let mut out = Vec::new();
    for (name, profile) in variants {
        let program = check_sources_profile(user_src, user_file, profile)?;
        // The variant's residual program: the same specialization path
        // the build takes (supercompilation and JIT specification
        // section 5: the driving bounds are profile parameters).
        let (program, _opt) = crate::specialize::optimize(&program, profile)?;
        let program = crate::rc::insert(program, profile)?;
        let program = stack_alloc(program, profile, &DriverOptions::default());
        let ir = crate::codegen::emit_for(profile, &program)?;
        let mut required: Vec<String> = profile.required.clone();
        if profile.artifact != crate::core::ArtifactProfile::Native
            && wit_for_source(std::path::Path::new(user_file))?.len() > 0
        {
            // As in the manifest: a component adapter requires the
            // component model on a Wasm artifact.
            required.push("wasm.component-model".to_string());
        }
        let mut lowerings: Vec<String> = Vec::new();
        lowerings.push(format!(
            "tasks:{}",
            if profile.artifact == crate::core::ArtifactProfile::Native {
                "os-thread"
            } else {
                "cooperative"
            }
        ));
        out.push(VariantPackage {
            name: name.to_string(),
            required,
            lowerings,
            ir_hash: crate::interface::content_hash(&ir),
        });
    }
    Ok(Distribution { variants: out })
}

/// Artifact admission (platform proofs and SIMD specification section
/// 3): an assumed deployment baseline is enforced when the artifact is
/// produced, and a generated component adapter requires the component
/// model on a Wasm artifact profile (the native lowering is the FFI
/// shim).
pub fn admit_artifact(
    profile: &crate::core::TargetProfile,
    program: &crate::core::Program,
    adapters: &[crate::interface::WitAdapter],
) -> Result<()> {
    for a in adapters {
        for b in &a.bindings {
            if b.export && !program.fns.iter().any(|f| f.id.0 == b.impl_unit) {
                return Err(Error::plain(format!(
                    "the world exports {} but the program defines no implementation `{}` (WebAssembly and WASI specification section 9)",
                    b.symbol, b.impl_unit
                )));
            }
        }
    }
    let mut required: Vec<String> = profile.required.clone();
    if !adapters.is_empty()
        && profile.artifact != crate::core::ArtifactProfile::Native
        && !profile.features.iter().any(|f| f == "wasm.component-model")
        && !required.iter().any(|f| f == "wasm.component-model")
    {
        required.push("wasm.component-model".to_string());
    }
    for f in &required {
        if !profile.features.iter().any(|x| x == f) {
            return Err(Error::plain(format!("missing required capability: {f}")));
        }
    }
    Ok(())
}

/// The generated WIT adapters of the `.wit` files beside a source file
/// (WebAssembly and WASI specification section 10: the toolchain imports
/// and exports WIT). Files parse in sorted order; two packages claiming
/// one vocabulary name are diagnosed when the vocabulary is installed.
pub fn wit_for_source(file: &Path) -> Result<Vec<crate::interface::WitAdapter>> {
    let dir = match file.parent() {
        Some(d) if !d.as_os_str().is_empty() => d.to_path_buf(),
        _ => return Ok(Vec::new()),
    };
    let mut wits: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for e in entries.flatten() {
            let p = e.path();
            if p.extension().map(|x| x == "wit").unwrap_or(false) {
                wits.push(p);
            }
        }
    }
    wits.sort();
    let mut out = Vec::new();
    for w in wits {
        let text = std::fs::read_to_string(&w)
            .map_err(|e| Error::plain(format!("{}: {e}", w.display())))?;
        let pkg = crate::parser::parse_wit(&text)
            .map_err(|e| Error::plain(format!("{}: {e}", w.display())))?;
        out.push(crate::interface::build_wit_adapter(
            &pkg,
            crate::interface::CanonicalAbi::Wasi03,
        )?);
    }
    Ok(out)
}

/// The facade source with the adapters' generated vocabulary appended
/// inside the prelude module and named in the facade's export list
/// (the WIT adapter vocabulary is the prelude's worked vocabulary,
/// like the sketch types of earlier chapters; the surface rule names
/// every one of its declarations).
fn prelude_source_with(adapters: &[crate::interface::WitAdapter]) -> String {
    let mut src = PRELUDE_FACADE.to_string();
    for a in adapters {
        src.push('\n');
        src.push_str(&a.vocab);
    }
    if adapters.is_empty() {
        return src;
    }
    let mut names: Vec<String> = Vec::new();
    for a in adapters {
        for n in &a.vocab_names {
            if !names.contains(n) {
                names.push(n.clone());
            }
        }
    }
    src.push_str(&format!("\n  export {}\n", names.join(", ")));
    src
}

/// Check the prelude plus `user_src`, returning the elaborated program.
pub fn check_sources(user_src: &str, user_file: &str) -> Result<crate::core::Program> {
    check_sources_with(user_src, user_file, &mut StagingCx::default())
}

/// The embedded prelude family and one user source parsed against it:
/// the family's modules in check order (the facade last), the user
/// source's modules, and the user modules' staging facts.
fn parse_family_and_user(
    adapters: &[crate::interface::WitAdapter],
    user_src: &str,
    user_file: &str,
    scx: &mut StagingCx,
) -> Result<(
    PreludeSources,
    crate::parser::ParsedModules,
    HashMap<String, String>,
)> {
    let prelude = parse_prelude(adapters)?;
    // The user source parses against the facade's base table plus every
    // embedded family module's own fixity table: an operator's fixity is
    // part of the module that declares it (the interface fact of
    // `design/compiled-interface-metadata.md` section 2.1, rule 3), so
    // `import Algebra.Semigroup` supplies `<>`'s fixity exactly as it
    // supplies the name, and no consumer redeclares it.
    let user = parse_source(
        user_src,
        user_file,
        &prelude.base,
        &prelude.exported_fixities,
    )?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(
            m.name.text.clone(),
            crate::interface::content_hash(user_src),
        );
        files.insert(m.name.text.clone(), user_file.to_string());
    }
    // The embedded family witnesses its own sources as well.
    for (name, hash) in family_source_hashes() {
        let file = format!("prelude/{name}.cinch");
        hashes.entry(name.clone()).or_insert(hash);
        files.entry(name).or_insert(file);
    }
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    Ok((prelude, user, hashes))
}

/// [`check_sources`] under an explicit target profile (the `Usize`
/// width, `eval` admission and feature assumptions key on the target:
/// primitive types specification section 2, supercompilation and JIT
/// specification section 7).
pub fn check_sources_profile(
    user_src: &str,
    user_file: &str,
    profile: &crate::core::TargetProfile,
) -> Result<crate::core::Program> {
    let adapters = wit_for_source(Path::new(user_file))?;
    let mut scx = StagingCx::default();
    let (prelude, user, hashes) = parse_family_and_user(&adapters, user_src, user_file, &mut scx)?;
    let (program, _ifaces, _report, _) = check::check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &adapters,
        profile,
    )?;
    Ok(program)
}

/// [`check_sources`] with an explicit staging context (mode, work
/// directory, captured inputs) for compile-time I/O.
pub fn check_sources_with(
    user_src: &str,
    user_file: &str,
    scx: &mut StagingCx,
) -> Result<crate::core::Program> {
    let adapters = wit_for_source(Path::new(user_file))?;
    let (prelude, user, hashes) = parse_family_and_user(&adapters, user_src, user_file, scx)?;
    let (program, _ifaces, _report, _) = check::check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        scx,
        &adapters,
        &crate::core::TargetProfile::native(),
    )?;
    Ok(program)
}

/// [`check_sources`] that also returns the per-module interfaces (the
/// `.cinchif` compatibility evidence and contract projection that the
/// "checked against" chain reads; runtime evolution specification
/// section 11).
pub fn check_sources_ifaces(
    user_src: &str,
    user_file: &str,
) -> Result<(crate::core::Program, Vec<crate::interface::Iface>)> {
    let adapters = wit_for_source(Path::new(user_file))?;
    let mut scx = StagingCx::default();
    let (prelude, user, hashes) = parse_family_and_user(&adapters, user_src, user_file, &mut scx)?;
    let (program, ifaces, _report, _) = check::check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &adapters,
        &crate::core::TargetProfile::native(),
    )?;
    Ok((program, ifaces))
}

/// [`check_sources_ifaces`] for one compile target: the interfaces record
/// the target's view of each module, so a conditional import that the
/// profile does not select is absent from the import list rather than
/// recorded as a dependency (modules specification section 11).
pub fn check_sources_profile_ifaces(
    user_src: &str,
    user_file: &str,
    profile: &crate::core::TargetProfile,
) -> Result<(crate::core::Program, Vec<crate::interface::Iface>)> {
    let adapters = wit_for_source(Path::new(user_file))?;
    let mut scx = StagingCx::default();
    let (prelude, user, hashes) = parse_family_and_user(&adapters, user_src, user_file, &mut scx)?;
    let (program, ifaces, _report, _) = check::check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &adapters,
        profile,
    )?;
    Ok((program, ifaces))
}

fn clang() -> String {
    std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string())
}

/// The advisory lock guarding the runtime staticlib: held exclusively
/// while the archive is rebuilt, shared while a link reads it. Cargo
/// replaces `libcinchruntime.a` in place, so without it two `cinch`
/// processes (parallel test binaries, sibling builds) race a rebuild
/// against a link and clang reads a truncated archive, failing with no
/// diagnostic of its own.
fn runtime_lock(shared: bool) -> Option<std::fs::File> {
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let dir = runtime.join("target");
    std::fs::create_dir_all(&dir).ok()?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(dir.join(".cinch-runtime.lock"))
        .ok()?;
    let held = if shared {
        file.lock_shared()
    } else {
        file.lock()
    };
    // A filesystem without advisory locking still builds and links, just
    // without the cross-process guarantee.
    held.ok()?;
    Some(file)
}

#[cfg(feature = "jit")]
pub(crate) fn ensure_runtime_dylib() -> Result<std::path::PathBuf> {
    let archive = ensure_runtime()?;
    let dir = archive
        .parent()
        .ok_or_else(|| Error::plain("runtime archive has no parent directory"))?;
    let dylib = dir.join(format!(
        "libcinchruntime.{}",
        std::env::consts::DLL_EXTENSION
    ));
    if dylib.exists() {
        return Ok(dylib);
    }
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let status = std::process::Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(runtime.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(runtime.join("target"))
        .status()
        .map_err(|e| Error::plain(format!("failed to build the runtime shared library: {e}")))?;
    if !status.success() || !dylib.exists() {
        return Err(Error::plain(
            "libcinchruntime shared library was not produced",
        ));
    }
    Ok(dylib)
}

fn ensure_runtime() -> Result<std::path::PathBuf> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let runtime = manifest_dir.join("runtime");
    let debug = runtime.join("target/debug/libcinchruntime.a");
    let release = runtime.join("target/release/libcinchruntime.a");
    // The rebuild holds the lock; a concurrent link waits rather than
    // reading a half-written archive.
    let _guard = runtime_lock(false);
    // Rebuild when any runtime source is newer than the staticlib (a
    // stale library silently drops the newer `cinch_*` entry points).
    let mut newest_src = std::time::SystemTime::UNIX_EPOCH;
    let mut stack = vec![runtime.join("src"), runtime.join("Cargo.toml")];
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&p) {
                for e in entries.flatten() {
                    stack.push(e.path());
                }
            }
        } else if let Ok(m) = std::fs::metadata(&p).and_then(|m| m.modified()) {
            newest_src = newest_src.max(m);
        }
    }
    let fresh = |p: &std::path::Path| -> bool {
        std::fs::metadata(p)
            .and_then(|m| m.modified())
            .map(|m| m >= newest_src)
            .unwrap_or(false)
    };
    if fresh(&release) {
        return Ok(release);
    }
    if fresh(&debug) {
        return Ok(debug);
    }
    let status = std::process::Command::new("cargo")
        .args(["build", "--manifest-path"])
        .arg(runtime.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(runtime.join("target"))
        .status()
        .map_err(|e| Error::plain(format!("failed to build the runtime crate: {e}")))?;
    if !status.success() {
        return Err(Error::plain(
            "building compiler/runtime (cinchruntime) failed",
        ));
    }
    if debug.exists() {
        Ok(debug)
    } else {
        Err(Error::plain(
            "libcinchruntime.a not found after building the runtime",
        ))
    }
}

fn link(ir_path: &Path, out: &Path) -> Result<()> {
    link_with(ir_path, out, &[], &LinkPlan::default())
}

/// How a link reaches a runnable artifact, which is decided by what the
/// target provides underneath the program.
#[derive(Clone, Copy, Default)]
enum LinkFlavor<'a> {
    /// The host's own target: clang's default triple, the runtime archive
    /// built here, and the platform's system libraries.
    #[default]
    Host,
    /// Another target with a C library: clang's driver links it, so it
    /// needs the triple and the runtime archive built for that target.
    Cross(&'a str),
    /// A freestanding target (no C library, no start files). clang's
    /// driver has nothing to find there: it would ask for `crt1.o` and
    /// `-lc`, and `wasm32-unknown-unknown` has neither. clang therefore
    /// only assembles the module and the target's own linker links it.
    Freestanding(&'a str),
}

/// Where a link's two target-dependent inputs come from: the cinch
/// runtime archive, and the triple the assembler and linker build for.
/// The host's own target needs neither spelled out (the archive is built
/// here, and clang's default is the host), and a foreign target needs
/// both. `opt_level` is the `--opt-level` clang optimizes the program at.
#[derive(Clone, Copy, Default)]
struct LinkTarget<'a> {
    /// Which of the three links this is.
    flavor: LinkFlavor<'a>,
    /// `--runtime-archive`: the archive built for the target.
    archive: Option<&'a Path>,
    /// `-O<n>` for clang; 0 is clang's own default and is not spelled.
    opt_level: u8,
}

/// Does this target have a C library and start files under it? A target
/// without either cannot use clang's driver to link, because the driver's
/// last step is to find the C runtime, and the honest report of that
/// absence is a different link rather than an error about `crt1.o`.
fn is_freestanding(profile: &crate::core::TargetProfile) -> bool {
    profile.arch == "wasm32" && profile.libc == "none"
}

/// The link inputs one compile needs: a foreign target hands clang its
/// triple and takes its runtime archive from `--runtime-archive`, and the
/// host's own target keeps the archive built here and clang's default.
fn link_target<'a>(
    profile: &'a crate::core::TargetProfile,
    opts: &'a DriverOptions,
) -> LinkTarget<'a> {
    let opt_level = opts.opt_level.unwrap_or(0);
    let flavor = if is_freestanding(profile) {
        LinkFlavor::Freestanding(profile.triple.as_str())
    } else if is_foreign(profile) {
        LinkFlavor::Cross(profile.triple.as_str())
    } else {
        LinkFlavor::Host
    };
    LinkTarget {
        flavor,
        archive: opts.runtime_archive.as_deref(),
        opt_level,
    }
}

/// [`link`] for an explicit target: the form `build` uses when
/// `--target` names something other than the host, and `emit-object`
/// uses to assemble the target's own objects.
fn link_for(
    ir_path: &Path,
    out: &Path,
    objects: &[&str],
    plan: &LinkPlan,
    target: &LinkTarget<'_>,
) -> Result<()> {
    if let LinkFlavor::Freestanding(triple) = target.flavor {
        return link_freestanding(ir_path, out, objects, plan, triple, target);
    }
    let mut cmd = std::process::Command::new(clang());
    cmd.arg(ir_path);
    if target.opt_level > 0 {
        cmd.arg(format!("-O{}", target.opt_level));
    }
    for o in objects {
        cmd.arg(o);
    }
    // The archive and, for the host, the shared lock that keeps a rebuild
    // from truncating it under this link. A cross link reads an archive
    // the caller produced, so it takes no lock.
    let _guard;
    match target.flavor {
        LinkFlavor::Host => {
            cmd.arg(ensure_runtime()?);
            _guard = runtime_lock(true);
        }
        LinkFlavor::Cross(triple) => {
            cmd.arg(cross_runtime(triple, target.archive)?);
            cmd.arg("-target").arg(triple);
            _guard = None;
        }
        // Returned above: clang's driver is not what links this one.
        LinkFlavor::Freestanding(_) => unreachable!("the freestanding link returns before this"),
    }
    cmd.arg("-o").arg(out);
    for word in plan.words() {
        cmd.arg(word);
    }
    // Rust staticlibs need the usual system libraries on Linux.
    if cfg!(target_os = "linux") {
        cmd.args(["-ldl", "-lpthread", "-lm"]);
    }
    let output = cmd
        .output()
        .map_err(|e| Error::plain(format!("failed to run {}: {e}", clang())))?;
    if !output.status.success() {
        // clang's own diagnostic is the whole story of a link failure
        // (an undefined symbol, a missing object); without it the error
        // reads as a truncated sentence.
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        // A cross link has one more requirement the caller has to
        // supply: an assembler and linker for the target. Naming it here
        // is what turns clang's "unable to create target" or its
        // truncated ELF complaint into an instruction.
        let hint = match target.flavor {
            LinkFlavor::Cross(triple) => format!(
                "\ncross linking for `{triple}` also needs a linker that produces that \
                 target's objects; CINCH_CLANG names a clang that has one (this run used `{}`)",
                clang()
            ),
            _ => String::new(),
        };
        return Err(Error::plain(format!(
            "{} failed to link {}{}{hint}",
            clang(),
            ir_path.display(),
            if detail.is_empty() {
                String::new()
            } else {
                format!(": {detail}")
            }
        )));
    }
    Ok(())
}

/// The clang major version a freestanding link requires: the emitted IR
/// carries `range(...)` attributes on the units whose result is known to
/// be a small integer, and LLVM 20 is the first release that parses them.
/// An older clang reports `expected type` on the first one, which reads
/// like corrupt IR rather than an old assembler.
pub const FREESTANDING_CLANG_MAJOR: u32 = 20;

/// The clang a freestanding link assembles with, and the version it
/// reported. `CINCH_CLANG` still names the compiler; this only reads the
/// version back so a refusal can name the compiler it found.
pub fn freestanding_clang() -> Result<(String, Option<u32>)> {
    let clang = clang();
    let output = std::process::Command::new(&clang)
        .arg("-dM")
        .arg("-E")
        .arg("-x")
        .arg("c")
        .arg(if cfg!(windows) { "NUL" } else { "/dev/null" })
        .output()
        .map_err(|e| Error::plain(format!(
            "target wasm32-unknown-unknown: assembling needs clang, and `{clang}` did not run: {e}; \
             CINCH_CLANG names the clang to use"
        )))?;
    let text = String::from_utf8_lossy(&output.stdout);
    let major = text
        .lines()
        .find_map(|line| line.strip_prefix("#define __clang_major__ "))
        .and_then(|value| value.trim().parse().ok());
    Ok((clang, major))
}

/// Link a freestanding target: clang assembles the module, and the
/// target's own linker links the object with the runtime archive.
///
/// The split is forced by what the target has. clang's driver finishes
/// every link by finding the C runtime (`crt1.o`, `-lc`), and a
/// freestanding target has none, so asking the driver to link reports a
/// missing C library that no flag can supply. Assembling is a different
/// question: the module is self-contained, and clang is the only tool
/// that reads it.
///
/// `wasm-ld` is found the way clang's driver finds it: beside clang when
/// the toolchain ships one (`-print-prog-name` answers a path), and
/// otherwise on `PATH`. A missing linker is refused by name with the
/// three ways to supply one, because the alternative is clang's
/// `posix_spawn failed` with no hint of what was missing.
fn link_freestanding(
    ir_path: &Path,
    out: &Path,
    objects: &[&str],
    plan: &LinkPlan,
    triple: &str,
    target: &LinkTarget<'_>,
) -> Result<()> {
    let (clang, major) = freestanding_clang()?;
    if let Some(major) = major {
        if major < FREESTANDING_CLANG_MAJOR {
            return Err(Error::plain(format!(
                "target `{triple}`: `{clang}` reports clang {major}, and assembling the emitted \
                 module needs clang {FREESTANDING_CLANG_MAJOR} or newer, because the IR carries \
                 `range(...)` attributes that older LLVM rejects as `expected type`; \
                 CINCH_CLANG names a newer clang (Apple clang 21 and LLVM 20 both read them)"
            )));
        }
    }
    let archive = cross_runtime(triple, target.archive)?;
    // The object clang assembles: beside the output, so a build directory
    // keeps the pair and a scratch directory takes both away.
    let object = out.with_extension("o");
    let mut assemble = std::process::Command::new(&clang);
    assemble.arg(ir_path);
    if target.opt_level > 0 {
        assemble.arg(format!("-O{}", target.opt_level));
    }
    assemble
        .arg("-target")
        .arg(triple)
        .arg("-c")
        .arg("-o")
        .arg(&object);
    let output = assemble
        .output()
        .map_err(|e| Error::plain(format!("failed to run {clang}: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::plain(format!(
            "{clang} failed to assemble {} for `{triple}`: {}",
            ir_path.display(),
            stderr.trim()
        )));
    }
    let linker = freestanding_linker(&clang)?;
    let mut link = std::process::Command::new(&linker);
    // A freestanding module has no `_start`, so the linker must not look
    // for one, and nothing else exports the program's entry points, so
    // the module exports them all.
    link.arg("--no-entry").arg("--export-all");
    link.arg(&object);
    for o in objects {
        link.arg(o);
    }
    link.arg(&archive);
    link.arg("-o").arg(out);
    for word in plan.words() {
        link.arg(word);
    }
    let output = link
        .output()
        .map_err(|e| Error::plain(format!("failed to run {}: {e}", linker.display())))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::plain(format!(
            "{} failed to link {} for `{triple}`: {}",
            linker.display(),
            ir_path.display(),
            stderr.trim()
        )));
    }
    // The object is an intermediate of the link, and the IR beside it is
    // the build's own record; a `build` keeps the IR and drops the object.
    let _ = std::fs::remove_file(&object);
    Ok(())
}

/// The JavaScript a freestanding `run` executes the module with: load the
/// bytes, instantiate with no imports satisfied, call the module's `main`,
/// and exit with what it returned.
///
/// The module's imports are its host boundary. A freestanding target
/// answers every operating-system call itself (`ENOSYS`), so a program
/// that runs at all imports nothing, and one that does import something is
/// asking for a host the target does not promise. Naming the import is the
/// honest answer there: a stub that returned a plausible value would run
/// the program against a machine that does not exist.
const WASM_HOST_JS: &str = r#"import fs from 'node:fs';
const module = await WebAssembly.compile(fs.readFileSync(process.argv[2]));
const imports = {};
for (const want of WebAssembly.Module.imports(module)) {
  if (want.kind !== 'function') {
    console.error(`cinch: the module imports ${want.module}.${want.name} (${want.kind}), which a freestanding target does not provide`);
    process.exit(70);
  }
  imports[want.module] ??= {};
  imports[want.module][want.name] = (...args) => {
    console.error(`cinch: the module called ${want.module}.${want.name}(${args.join(', ')}), which a freestanding target does not provide`);
    process.exit(70);
  };
}
const instance = await WebAssembly.instantiate(module, imports);
const main = instance.exports.main;
if (typeof main !== 'function') {
  console.error('cinch: the module exports no main');
  process.exit(70);
}
process.exit(main());
"#;

/// Execute a freestanding Wasm module under Node, which is the host that
/// runs one on this machine. The exit status is the module's `main`
/// result, so a program's exit code means here what it means natively.
fn run_wasm_module(module: &Path, profile: &crate::core::TargetProfile) -> Result<i32> {
    let node = std::env::var("CINCH_NODE").unwrap_or_else(|_| "node".to_string());
    let host = module.with_extension("host.mjs");
    std::fs::write(&host, WASM_HOST_JS)
        .map_err(|e| Error::plain(format!("{}: {e}", host.display())))?;
    let output = std::process::Command::new(&node)
        .arg(&host)
        .arg(module)
        .output()
        .map_err(|e| Error::plain(format!(
            "target `{}`: `run` executes a freestanding module under Node, and `{node}` did not \
             run: {e}; CINCH_NODE names the Node executable",
            profile.triple
        )))?;
    // Node's own diagnostics (a trap, a refused import) are the reason the
    // program did not produce a status, so they are the message.
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.trim().is_empty() {
        eprint!("{stderr}");
    }
    match output.status.code() {
        Some(code) => Ok(code),
        None => Err(Error::plain(format!(
            "target `{}`: `{node}` did not report an exit status for {}",
            profile.triple,
            module.display()
        ))),
    }
}

/// The linker a freestanding link uses, from `CINCH_WASM_LD` or the same
/// search clang's driver performs: the toolchain's own `wasm-ld` when
/// `-print-prog-name` answers a path, else `PATH`.
pub fn freestanding_linker(clang: &str) -> Result<PathBuf> {
    if let Some(named) = std::env::var_os("CINCH_WASM_LD").filter(|p| !p.is_empty()) {
        let named = PathBuf::from(named);
        if named.is_file() {
            return Ok(named);
        }
        return Err(Error::plain(format!(
            "CINCH_WASM_LD names {}, and that is not a file",
            named.display()
        )));
    }
    // A toolchain that ships the linker beside clang answers with a path.
    // One that does not answers the bare name, which is clang's way of
    // saying it expects the linker on `PATH`.
    if let Ok(output) = std::process::Command::new(clang)
        .arg("-print-prog-name=wasm-ld")
        .output()
    {
        if output.status.success() {
            let answer = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !answer.is_empty() && answer != "wasm-ld" && Path::new(&answer).is_file() {
                return Ok(PathBuf::from(answer));
            }
        }
    }
    if let Some(found) = search_path("wasm-ld") {
        return Ok(found);
    }
    Err(Error::plain(format!(
        "target wasm32-unknown-unknown: `{clang}` assembles the module, but no `wasm-ld` links \
         it; install LLVM's binary utilities so `wasm-ld` is on PATH (nixpkgs `llvmPackages.bintools` \
         provides it), or set CINCH_WASM_LD to a `wasm-ld`"
    )))
}

/// The first `name` on `PATH` that is a file, which is the search every
/// tool that execs by bare name performs.
fn search_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

/// The runtime archive a freestanding target links, built here when
/// `--runtime-archive` does not name one.
///
/// The host archive is built on demand the same way (`ensure_runtime`),
/// and a freestanding build is a build of this same tree: the runtime
/// crate compiles for the target without a C toolchain under it, so the
/// compiler can produce the archive it is about to link rather than
/// telling the caller to run cargo first. A target whose Rust support is
/// not installed fails that build, and the flag remains the way to
/// supply an archive from elsewhere.
fn ensure_freestanding_runtime(triple: &str) -> Result<PathBuf> {
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime");
    let target_dir = runtime.join("target");
    let archive = target_dir.join(triple).join("release/libcinchruntime.a");
    let _guard = runtime_lock(false);
    // The same freshness rule the host archive uses: a stale archive
    // silently drops the newer `cinch_*` entry points.
    let mut newest_src = std::time::SystemTime::UNIX_EPOCH;
    let mut stack = vec![runtime.join("src"), runtime.join("Cargo.toml")];
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&p) {
                for e in entries.flatten() {
                    stack.push(e.path());
                }
            }
        } else if let Ok(m) = std::fs::metadata(&p).and_then(|m| m.modified()) {
            newest_src = newest_src.max(m);
        }
    }
    let fresh = std::fs::metadata(&archive)
        .and_then(|m| m.modified())
        .map(|m| m >= newest_src)
        .unwrap_or(false);
    if fresh {
        return Ok(archive);
    }
    let status = std::process::Command::new("cargo")
        .args(["build", "--release", "--manifest-path"])
        .arg(runtime.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(&target_dir)
        .arg("--target")
        .arg(triple)
        .status()
        .map_err(|e| Error::plain(format!(
            "target `{triple}`: building the runtime archive needs cargo, and it did not run: {e}"
        )))?;
    if !status.success() || !archive.exists() {
        return Err(Error::plain(format!(
            "target `{triple}`: building compiler/runtime for `{triple}` failed; install that \
             Rust target (rustup target add {triple}) or pass --runtime-archive \
             <path>/libcinchruntime.a built for `{triple}`"
        )));
    }
    Ok(archive)
}

/// The runtime archive a cross link needs, from `--runtime-archive`: the
/// archive is read, required to be a static library, and its first
/// object is required to belong to the target's architecture. The host's
/// archive is the natural mistake here, and linking it would surface as
/// a pile of undefined `cinch_*` symbols instead of the one fact that
/// explains them.
fn cross_runtime(triple: &str, archive: Option<&Path>) -> Result<PathBuf> {
    // A freestanding target's archive is one this tree can build, so a
    // build without the flag produces it rather than refusing.
    let built;
    let path = match archive {
        Some(path) => path,
        None if triple_components(triple).is_some_and(|(arch, _, libc)| arch == "wasm32" && libc == "none") => {
            built = ensure_freestanding_runtime(triple)?;
            &built
        }
        None => {
            return Err(Error::plain(format!(
                "target `{triple}`: a cross build links the compiler runtime built for that target, \
                 and none was named; pass --runtime-archive <path>/libcinchruntime.a built for \
                 `{triple}` (cargo build --manifest-path compiler/runtime/Cargo.toml \
                 --target {triple} produces one)"
            )));
        }
    };
    let bytes = std::fs::read(path).map_err(|e| {
        Error::plain(format!(
            "--runtime-archive {}: {e}; a cross build for `{triple}` needs the runtime \
             archive built for that target",
            path.display()
        ))
    })?;
    if !bytes.starts_with(b"!<arch>\n") && !bytes.starts_with(b"!<thin>\n") {
        return Err(Error::plain(format!(
            "--runtime-archive {}: not a static library archive; a cross build for `{triple}` \
             needs libcinchruntime.a built for that target",
            path.display()
        )));
    }
    let expected = triple_components(triple).map(|(arch, _, _)| arch);
    if let (Some(expected), Some(found)) = (expected, first_object_arch(&bytes)) {
        if found != expected {
            return Err(Error::plain(format!(
                "--runtime-archive {}: its objects are {found}, not {expected}; a cross build \
                 for `{triple}` needs the runtime archive built for that target (cargo build \
                 --manifest-path compiler/runtime/Cargo.toml --target {triple})",
                path.display()
            )));
        }
    }
    Ok(path.to_path_buf())
}

/// The architecture of a static archive's first object, spelled as the
/// triple table spells it. `None` when the bytes are not an archive of
/// objects this check understands, which the linker reports instead of
/// this guess.
fn first_object_arch(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(b"!<thin>\n") {
        // A thin archive holds references, not objects: nothing here to
        // read, and clang's own diagnostic covers it.
        return None;
    }
    if !bytes.starts_with(b"!<arch>\n") {
        return None;
    }
    let mut pos = 8;
    while pos + 60 <= bytes.len() {
        let header = bytes.get(pos..pos + 60)?;
        if &header[58..60] != b"`\n" {
            return None;
        }
        let name = std::str::from_utf8(&header[0..16]).ok()?.trim_end();
        let size: usize = std::str::from_utf8(&header[48..58])
            .ok()?
            .trim()
            .parse()
            .ok()?;
        let start = pos + 60;
        let end = start.checked_add(size)?;
        let member = bytes.get(start..end)?;
        // The two tables ar puts first: the symbol index and the long
        // name table.
        let table = name == "/" || name == "//" || name == "/SYM64/" || name.starts_with("__.SYMDEF");
        if !table {
            return object_arch(member);
        }
        pos = end + (size & 1);
    }
    None
}

/// One object's architecture from its header: an ELF64 little-endian
/// machine word or a 64-bit little-endian Mach-O CPU type.
fn object_arch(object: &[u8]) -> Option<&'static str> {
    // A Wasm module: the magic, the version, and nothing else that varies
    // by target. `wasm32` is the only architecture the format has.
    if object.starts_with(b"\0asm") {
        return Some("wasm32");
    }
    if object.starts_with(b"\x7fELF") && object.len() >= 20 {
        // Class 2 is 64-bit and data 1 is little-endian; the four
        // supported targets are all of them.
        if object[4] != 2 || object[5] != 1 {
            return None;
        }
        return match u16::from_le_bytes([object[18], object[19]]) {
            62 => Some("x86_64"),
            183 => Some("aarch64"),
            _ => None,
        };
    }
    // MH_MAGIC_64 read on a little-endian host.
    if object.len() >= 8 && object.starts_with(b"\xcf\xfa\xed\xfe") {
        return match u32::from_le_bytes([object[4], object[5], object[6], object[7]]) {
            0x0100_0007 => Some("x86_64"),
            0x0100_000c => Some("aarch64"),
            _ => None,
        };
    }
    None
}

/// [`link`] with extra objects (the C fixtures of the foreign binding
/// tests; foreign function interface specification section 7: the linker
/// checks the compatible symbols).
fn link_with(ir_path: &Path, out: &Path, objects: &[&str], plan: &LinkPlan) -> Result<()> {
    link_for(ir_path, out, objects, plan, &LinkTarget::default())
}

/// Create a fresh directory for one compiled test or run.
fn scratch_dir(tag: &str) -> Result<std::path::PathBuf> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let root = std::env::temp_dir();
    std::fs::create_dir_all(&root).map_err(|e| Error::plain(format!("{}: {e}", root.display())))?;
    let pid = std::process::id();
    loop {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = root.join(format!("cinch-{tag}-{pid}-{n}"));
        match std::fs::create_dir(&path) {
            Ok(()) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(Error::plain(format!("{}: {e}", path.display()))),
        }
    }
}

/// The flags a compile takes that are neither a mode nor a profile.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DriverOptions {
    /// Concrete representation variants demanded by a library consumer.
    /// Used only by `EmitObject`; an empty list preserves ordinary roots.
    pub object_roots: Vec<crate::specialize::ObjectRoot>,
    /// `EmitObject` only: modules whose units this object defines. Every
    /// other module is an external dependency and is emitted as declarations,
    /// so separately produced package objects do not duplicate one another.
    /// Nonempty sets also select the installed package execution interface.
    /// Empty preserves the standalone whole-closure object.
    pub emit_modules: Vec<String>,
    /// `Build`/`Run`: separately produced dependency objects to link. Their
    /// recorded modules are excluded from emitted IR and resolved from the
    /// object at link time.
    pub package_objects: Vec<PathBuf>,
    /// Objects this build calls into but does not link (a package object's
    /// already-built dependencies): only their published lent conventions
    /// are read (`package_lent_modes`), and the object records the ones it
    /// used so its final link can check them.
    pub lent_objects: Vec<PathBuf>,
    /// `--rebuild`: rewrite the package-store entry for this compile's
    /// key even when the entry is intact and matches it, instead of
    /// leaving it alone. The compile path never fails over an entry's
    /// contents (it repairs anything that is not this compile's set, see
    /// `crate::store`), so this is the explicit way to force a rewrite,
    /// and the escape for an entry a consumer reported as corrupt.
    pub rebuild_store: bool,
    /// Native words the caller has already resolved for a package it is
    /// installing: merged over the lock's `native` records, so a package
    /// whose `pkgconfig` entries were just resolved can be built before
    /// its lock row exists. Only `cinch pkg` sets it.
    pub native_words: std::collections::BTreeMap<String, Vec<String>>,
    /// `--print-link-plan`: report every contribution to the final link
    /// with the package that asked for it (design/base-as-a-package.md
    /// section 4.3).
    pub print_link_plan: bool,
    /// `--allow-raw-link NAME`: consent to one package's raw link words
    /// for this build, the flag form of the root manifest's `raw_link`.
    pub allow_raw_link: Vec<String>,
    /// `--target <triple>`: the target to compile for. One of
    /// [`SUPPORTED_TARGETS`]; an unrecognised triple is refused with the
    /// list. `check` and `emit-ir` work for any of them on any host, a
    /// cross `build` needs `runtime_archive`, and `run` stays on the
    /// host's own target.
    pub target: Option<String>,
    /// `--runtime-archive <path>`: the compiler runtime archive
    /// (`libcinchruntime.a`) built for a foreign target. A cross build
    /// cannot link the host's archive, so the target's is supplied here;
    /// the archive is checked to be a static archive of the target's
    /// objects before the link starts.
    pub runtime_archive: Option<PathBuf>,
    /// `--opt-level <0..3>`: the optimization level clang assembles the
    /// emitted IR at. `None` takes the mode's default: 0 for `build` and
    /// `run` (fast compiles, as the test suites need), 2 for `emit-object`
    /// (a library object is assembled once and linked many times).
    pub opt_level: Option<u8>,
    /// `--target-version <version>`: the target platform version a
    /// package's `os_version` conditions are evaluated against. It is an
    /// explicit parameter rather than the host's version, because a
    /// host-derived value makes the lock machine-specific.
    pub target_version: Option<String>,
    /// `--dump-core=<stage>`: after the pipeline point that stage names,
    /// write the Core dump beside the artifact (`build/<stem>.core`) and
    /// list it in the artifact manifest. It never changes what the
    /// pipeline does otherwise (design/core-output.md section 2).
    pub dump_core: Option<crate::coretext::Stage>,
    /// `core emit`: write the dump here instead of `build/<stem>.core`.
    pub core_out: Option<PathBuf>,
    /// `core emit`: stop once the dump is written. The command's only job
    /// is the dump, so it emits neither IR nor a binary.
    pub core_only: bool,
    /// `--max-simplifier-iterations <n>`: the Core simplifier's pass cap
    /// (default 4, zero disables the simplifier; design/compilation-speed.md
    /// section 10).
    pub max_simplifier_iterations: Option<u32>,
    /// `--no-direct-dict-calls`: leave a static dictionary projection as a
    /// projection plus `__apply` instead of a direct call
    /// (design/compilation-speed.md section 10).
    pub no_direct_dict_calls: bool,
    /// `--known-ctor-computed`: fold a decision on a computed-argument
    /// constructor application. Off by default (the rule's documentation
    /// carries the measurement that keeps it off).
    pub known_ctor_computed: bool,
    /// `--no-projection`: keep a record field projection of a constructor
    /// application instead of replacing it by the field.
    pub no_projection: bool,
    /// `--no-saturate-partial`: keep a one-shot partial application as a
    /// closure and an application (the measurement's other arm).
    pub no_saturate_partial: bool,
    /// `--no-inline`: leave every call in place, so a measurement can be
    /// taken against the same program with the inliner active
    /// (design/stack-allocation.md section 7).
    pub no_inline: bool,
    /// `--inline-model <threshold|benefit>`: which site test the inliner
    /// uses (`crate::simplify::InlineModel`). Default `benefit`.
    pub inline_model: Option<String>,
    /// `--interface-optimisations <manifest>`: an interface manifest whose
    /// modules' `.cinchopt` unfoldings answer this program's cross-object
    /// call sites. The units those artifacts describe are not emitted by
    /// this compilation, so the caller links the object they describe.
    pub interface_optimisations: Option<PathBuf>,
    /// `--emit-interface-optimisations`: write the `.cinchopt` artifacts
    /// beside this compile's `.cinchif`s. An object build writes them by
    /// default, because it is a library producer and its consumer may read
    /// them; every other mode writes them only when asked, so a check does
    /// not pay for an artifact it has no consumer for.
    pub emit_interface_optimisations: bool,
    /// `--case-of-case`: run the case-of-case rule (off by default; the join
    /// node, its introduction and its printing stay on either way).
    pub case_of_case: bool,
    /// `--no-stack-alloc`: keep every allocation on the heap. The emitted
    /// IR is then the one the compiler produced before the stack
    /// allocation pass existed (design/stack-allocation.md).
    pub no_stack_alloc: bool,
    /// `--stack-frame-limit <bytes>`: the most frame bytes one unit may
    /// reserve (default 16 KiB); the rest of its allocations stay on the
    /// heap.
    pub stack_frame_limit: Option<u64>,
    /// The words the CLI forwards to the program `Mode::Run` executes:
    /// every argument after the entry file (`cinch run f.cinch a b`, or
    /// everything after `--` verbatim). They land on the built program's
    /// own command line, so the program reads them with
    /// `Effect.Process`'s `process-args` (`word 0` of that capture is
    /// the program itself, not one of these). Empty for a library
    /// caller, which is every test: only the CLI has words to forward.
    pub program_args: Vec<String>,
}

impl DriverOptions {
    /// The stack allocation knobs these flags select
    /// (design/stack-allocation.md).
    pub fn stack(&self) -> crate::stack::Options {
        crate::stack::Options {
            enabled: !self.no_stack_alloc,
            frame_limit: self.stack_frame_limit.unwrap_or(crate::stack::DEFAULT_FRAME_LIMIT),
            // The measurement switch for the rule's two extra transfers
            // (`CINCH_STACK_WIDEN`); unset is the shipped rule.
            widen: crate::stack::Widen::from_env(),
        }
    }

    /// The simplifier budget these flags select: the pass cap, whether
    /// static dictionary projections become direct calls, and the defaults
    /// of the rules the flags do not reach (`case-of-case` and the node
    /// count below which it duplicates rather than shares a body).
    pub fn simplifier(&self) -> crate::simplify::SimplifyOptions {
        crate::simplify::SimplifyOptions {
            max_iterations: self.max_simplifier_iterations.unwrap_or(4),
            direct_dict_calls: !self.no_direct_dict_calls,
            known_constructor_computed: self.known_ctor_computed,
            projection: !self.no_projection,
            saturate_partial: !self.no_saturate_partial,
            inline: !self.no_inline,
            inline_model: match self.inline_model.as_deref() {
                Some("threshold") => crate::simplify::InlineModel::Threshold,
                Some("benefit") | None => crate::simplify::InlineModel::Benefit,
                Some(_) => crate::simplify::InlineModel::Benefit,
            },
            case_of_case: self.case_of_case,
            ..crate::simplify::SimplifyOptions::default()
        }
    }
}

/// The compile-time I/O mode from `CINCH_STAGE_MODE` (`refresh`,
/// `replay`, or `live`; default `refresh`).
fn stage_mode_from_env() -> StageMode {
    match std::env::var("CINCH_STAGE_MODE").as_deref() {
        Ok("replay") => StageMode::Replay,
        Ok("live") => StageMode::Live,
        _ => StageMode::Refresh,
    }
}

/// Compile `path` according to `mode`. `out` names the output file for
/// `EmitIr` and `Build`; `Run` executes the produced binary and returns its
/// exit code in `Ok`. The compile-time I/O mode comes from
/// `CINCH_STAGE_MODE` (`refresh`, `replay`, or `live`; default
/// `refresh`).
pub fn driver(mode: Mode, path: &Path, out: Option<&Path>) -> Result<i32> {
    driver_opts(mode, path, out, &DriverOptions::default())
}

/// [`driver`] with explicit options: the entry point the `cinch` CLI
/// uses for its flags. `Run` executes the produced binary and returns
/// its exit code in `Ok`, handing the program `opts.program_args` as
/// its command line.
pub fn driver_opts(
    mode: Mode,
    path: &Path,
    out: Option<&Path>,
    opts: &DriverOptions,
) -> Result<i32> {
    let profile = target_profile(opts)?;
    driver_with_profile_opts(mode, stage_mode_from_env(), path, out, &profile, opts)
}

/// Apply the stack allocation pass (design/stack-allocation.md) to a
/// program the count pass just finished. `opts` carries `--no-stack-alloc`
/// and `--stack-frame-limit`; the test entry points take the defaults.
fn stack_alloc(
    mut program: crate::core::Program,
    profile: &crate::core::TargetProfile,
    opts: &DriverOptions,
) -> crate::core::Program {
    crate::stack::allocate(&mut program, profile, &opts.stack());
    program
}

/// The Core dump one compile asked for (`--dump-core=<stage>`, or `cinch
/// core emit`): what to print, where, and the facts the printer needs that
/// the program itself does not carry (unit signatures, the module set, the
/// source identity).
struct CoreDump<'a> {
    stage: Option<crate::coretext::Stage>,
    out: Option<PathBuf>,
    profile: &'a crate::core::TargetProfile,
    modules: std::collections::BTreeSet<String>,
    dir: PathBuf,
    stem: String,
    opt_level: u8,
}

impl<'a> CoreDump<'a> {
    fn new(
        opts: &DriverOptions,
        profile: &'a crate::core::TargetProfile,
        ifaces: &[crate::interface::Iface],
        entry_modules: &[String],
        path: &Path,
        out_dir: &Path,
        stem: &str,
    ) -> CoreDump<'a> {
        use sha2::{Digest, Sha256};
        let signatures = crate::coretext::Signatures::from_ifaces(ifaces);
        let modules: std::collections::BTreeSet<String> = entry_modules.iter().cloned().collect();
        // The source identity is the checked sources of the dumped modules,
        // not a file list: sorted, so a directory walk cannot reorder it.
        let mut identity = String::new();
        for iface in ifaces {
            if modules.contains(&iface.module) {
                identity.push_str(&iface.module);
                identity.push('\0');
                identity.push_str(&iface.source_hash);
                identity.push('\n');
            }
        }
        let digest = format!("{:x}", Sha256::digest(identity.as_bytes()));
        let _ = path;
        CoreDump {
            stage: opts.dump_core,
            out: opts.core_out.clone(),
            profile,
            modules,
            dir: out_dir.to_path_buf(),
            stem: stem.to_string(),
            opt_level: opts.opt_level.unwrap_or(0),
        }
    }

    fn stage(&self) -> Option<crate::coretext::Stage> {
        self.stage
    }

    fn wants(&self, stage: crate::coretext::Stage) -> bool {
        self.stage == Some(stage)
    }

    /// The file name for one stage: the last stage keeps the plain name,
    /// every earlier one appends its name (`Main.checked.core`).
    fn file_name(stem: &str, stage: crate::coretext::Stage) -> String {
        if stage == crate::coretext::Stage::Lowered {
            format!("{stem}.core")
        } else {
            format!("{stem}.{}.core", stage.name())
        }
    }

    fn text(
        &self,
        program: &crate::core::Program,
        stage: crate::coretext::Stage,
        rewrites: &std::collections::BTreeMap<String, Vec<String>>,
        ifaces: &[crate::interface::Iface],
        ticks: Option<usize>,
    ) -> String {
        crate::coretext::dump_with_ticks(
            program,
            stage,
            self.profile,
            ifaces,
            &self.modules,
            rewrites,
            self.opt_level,
            ticks,
        )
    }

    fn write(
        &self,
        program: &crate::core::Program,
        stage: crate::coretext::Stage,
        ifaces: &[crate::interface::Iface],
        rows: &mut Vec<CoreRow>,
    ) -> Result<()> {
        let text = self.text(program, stage, &std::collections::BTreeMap::new(), ifaces, None);
        self.save(&text, stage, rows)
    }

    /// The lowered dump: specialization recorded which rewrite rule fired in
    /// which unit, and that is the one fact the program does not carry.
    fn write_lowered(
        &self,
        program: &crate::core::Program,
        opt: &crate::core::OptReport,
        ifaces: &[crate::interface::Iface],
        rows: &mut Vec<CoreRow>,
    ) -> Result<()> {
        let mut rewrites: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();
        for site in &opt.sites {
            if let Some(rule) = site.kind.strip_prefix("rewrite:") {
                rewrites.entry(site.fn_id.clone()).or_default().push(rule.to_string());
            }
        }
        // The simplifier's tick count rides in the header, so the dump says
        // how much rewriting produced the bodies beside it.
        let text = self.text(
            program,
            crate::coretext::Stage::Lowered,
            &rewrites,
            ifaces,
            Some(opt.simplify.ticks),
        );
        self.save(&text, crate::coretext::Stage::Lowered, rows)
    }

    fn save(
        &self,
        text: &str,
        stage: crate::coretext::Stage,
        rows: &mut Vec<CoreRow>,
    ) -> Result<()> {
        use sha2::{Digest, Sha256};
        let path = match &self.out {
            Some(path) => path.clone(),
            None => self.dir.join(Self::file_name(&self.stem, stage)),
        };
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| Error::plain(format!("{}: {e}", parent.display())))?;
            }
        }
        std::fs::write(&path, text)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        rows.push(CoreRow {
            stage: stage.name().to_string(),
            target: self.profile.triple.clone(),
            digest: format!("{:x}", Sha256::digest(text.as_bytes())),
            path: path.display().to_string(),
        });
        Ok(())
    }

    /// Add the dumped files to the artifact manifest (design note section
    /// 5). The manifest is already written by the time the lowered stage
    /// exists, so the rows are merged into it here; everything else in the
    /// document is untouched.
    fn record(
        &self,
        manifest_path: &Path,
        rows: &[CoreRow],
        stack: Option<&crate::stack::Report>,
    ) -> Result<()> {
        if rows.is_empty() && stack.is_none() {
            return Ok(());
        }
        use crate::interface::{parse_json, JsonValue as J};
        let text = std::fs::read_to_string(manifest_path)
            .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
        let mut doc = parse_json(&text)
            .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
        let J::Obj(fields) = &mut doc else {
            return Err(Error::plain(format!(
                "{}: the manifest is not a JSON object",
                manifest_path.display()
            )));
        };
        if !rows.is_empty() {
            fields.insert(
                "core".to_string(),
                J::arr(rows, |row| {
                    J::obj([
                        ("stage", row.stage.as_str().into()),
                        ("target", row.target.as_str().into()),
                        ("digest", row.digest.as_str().into()),
                        ("path", row.path.as_str().into()),
                    ])
                }),
            );
        }
        if let Some(stack) = stack {
            // Frame allocation (design/stack-allocation.md): what the pass
            // put in the frames, and why the rest stayed on the heap.
            let mut refused: Vec<(&str, &u64)> =
                stack.refused.iter().map(|(why, n)| (*why, n)).collect();
            refused.sort();
            fields.insert(
                "stack".to_string(),
                J::obj([
                    ("enabled", stack.enabled.into()),
                    ("units", stack.units.into()),
                    ("slots", stack.slots.into()),
                    ("bytes", stack.bytes.into()),
                    ("candidates", stack.candidates.into()),
                    (
                        "refused",
                        J::Obj(
                            refused
                                .into_iter()
                                .map(|(why, n)| (why.to_string(), (*n).into()))
                                .collect(),
                        ),
                    ),
                ]),
            );
        }
        std::fs::write(manifest_path, doc.to_canonical_string())
            .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
        Ok(())
    }
}

/// One Core file the build wrote, as the artifact manifest records it.
#[derive(Clone, Debug)]
struct CoreRow {
    stage: String,
    target: String,
    digest: String,
    path: String,
}

/// `cinch core ...`: the Core text tools (design/core-output.md section 2).
///
/// `emit` runs the compile pipeline through the dump (no IR, no link),
/// `verify` parses and re-checks a Core file, and `diff` reports the first
/// structural divergence between two files. The return value is the process
/// exit code, so `main` can hand it straight to `exit`.
pub fn core_cli(args: &[String]) -> i32 {
    match args.first().map(String::as_str) {
        Some("emit") => core_emit(&args[1..]),
        Some("verify") => core_verify(&args[1..]),
        Some("diff") => core_diff(&args[1..]),
        _ => {
            eprintln!("{}", core_usage());
            2
        }
    }
}

fn core_usage() -> &'static str {
    "\
cinch core: the annotated Core dump (design/core-output.md)

usage:
  cinch core emit <file.cinch> [--stage=<name>] [--target <triple>] [-o <out.core>]
                     [--max-simplifier-iterations <n>] [--no-direct-dict-calls]
                     [--case-of-case] [--no-inline] [--inline-model <threshold|benefit>] [--no-stack-alloc]
                     [--stack-frame-limit <bytes>]
  cinch core verify <file.core>
  cinch core diff <a.core> <b.core>

stages: checked, solved, elaborated (aliases), owned, lowered (the default)
"
}

fn core_emit(args: &[String]) -> i32 {
    let mut file: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut stage = crate::coretext::Stage::Lowered;
    let mut opts = DriverOptions::default();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-o" => {
                i += 1;
                match args.get(i) {
                    Some(path) => out = Some(path),
                    None => {
                        eprintln!("cinch core emit: -o needs a path");
                        return 2;
                    }
                }
            }
            "--target" => {
                i += 1;
                match args.get(i) {
                    Some(triple) => opts.target = Some(triple.clone()),
                    None => {
                        eprintln!("cinch core emit: --target needs a triple such as aarch64-apple-darwin");
                        return 2;
                    }
                }
            }
            "--target-version" => {
                i += 1;
                match args.get(i) {
                    Some(version) => opts.target_version = Some(version.clone()),
                    None => {
                        eprintln!("cinch core emit: --target-version needs a version such as 14.0");
                        return 2;
                    }
                }
            }
            "--max-simplifier-iterations" => {
                i += 1;
                match args.get(i).and_then(|n| n.parse::<u32>().ok()) {
                    Some(n) => opts.max_simplifier_iterations = Some(n),
                    None => {
                        eprintln!("cinch core emit: --max-simplifier-iterations needs a non-negative count");
                        return 2;
                    }
                }
            }
            "--no-direct-dict-calls" => opts.no_direct_dict_calls = true,
            "--known-ctor-computed" => opts.known_ctor_computed = true,
            "--no-projection" => opts.no_projection = true,
            "--no-saturate-partial" => opts.no_saturate_partial = true,
            "--no-inline" => opts.no_inline = true,
            "--inline-model" => {
                i += 1;
                match args.get(i) {
                    Some(v) if v == "threshold" || v == "benefit" => opts.inline_model = Some(v.clone()),
                    _ => {
                        eprintln!("cinch: --inline-model needs `threshold` or `benefit`");
                        return 2;
                    }
                }
            }
            "--case-of-case" => opts.case_of_case = true,
            "--no-stack-alloc" => opts.no_stack_alloc = true,
            "--stack-frame-limit" => {
                i += 1;
                match args.get(i).and_then(|n| n.parse::<u64>().ok()) {
                    Some(n) => opts.stack_frame_limit = Some(n),
                    None => {
                        eprintln!("cinch core emit: --stack-frame-limit needs a byte count");
                        return 2;
                    }
                }
            }
            other if other.starts_with("--stage=") => {
                let name = &other["--stage=".len()..];
                match crate::coretext::Stage::parse(name) {
                    Some(parsed) => stage = parsed,
                    None => {
                        eprintln!(
                            "cinch core emit: unknown stage `{name}`: one of {}",
                            crate::coretext::Stage::spellings().join(", ")
                        );
                        return 2;
                    }
                }
            }
            other if other.starts_with("--") => {
                eprintln!("cinch core emit: unknown flag {other}");
                return 2;
            }
            other => {
                if file.is_some() {
                    eprintln!("cinch core emit: takes one source file");
                    return 2;
                }
                file = Some(other);
            }
        }
        i += 1;
    }
    let Some(file) = file else {
        eprintln!("cinch core emit: needs a source file\n{}", core_usage());
        return 2;
    };
    opts.dump_core = Some(stage);
    opts.core_out = out.map(PathBuf::from);
    opts.core_only = true;
    // `EmitIr` is the cheapest mode that runs specialization and reference
    // counting; with `core_only` the compile stops at the dump, so no IR and
    // no binary are produced.
    match driver_opts(Mode::EmitIr, Path::new(file), None, &opts) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("cinch: {error}");
            1
        }
    }
}

fn core_verify(args: &[String]) -> i32 {
    let Some(file) = args.iter().find(|arg| !arg.starts_with("--")) else {
        eprintln!("cinch core verify: needs a `.core` file\n{}", core_usage());
        return 2;
    };
    let text = match std::fs::read_to_string(file) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("cinch core verify: {file}: {error}");
            return 1;
        }
    };
    match crate::coretext::verify(&text) {
        Ok(report) => {
            println!("{report}");
            0
        }
        Err(errors) => {
            for error in &errors {
                eprintln!("{file}: {error}");
            }
            eprintln!(
                "cinch core verify: {} error{}",
                errors.len(),
                if errors.len() == 1 { "" } else { "s" }
            );
            1
        }
    }
}

fn core_diff(args: &[String]) -> i32 {
    let files: Vec<&String> = args.iter().filter(|arg| !arg.starts_with("--")).collect();
    let (Some(left), Some(right)) = (files.first(), files.get(1)) else {
        eprintln!("cinch core diff: needs two `.core` files\n{}", core_usage());
        return 2;
    };
    let mut texts = Vec::new();
    for file in [left, right] {
        match std::fs::read_to_string(file) {
            Ok(text) => texts.push(text),
            Err(error) => {
                eprintln!("cinch core diff: {file}: {error}");
                return 1;
            }
        }
    }
    match crate::coretext::diff(&texts[0], &texts[1]) {
        None => 0,
        Some(divergence) => {
            println!("{divergence}");
            1
        }
    }
}

/// The triples this compiler emits for. `check` and `emit-ir` work for
/// every one of them on any host, because the module header, type
/// widths, and `when arch` body selection read the target rather than
/// the host. `build` additionally needs the runtime archive named by
/// `--runtime-archive` and a linker that produces the target's objects;
/// `run` executes only a program built for the host's own target.
pub const SUPPORTED_TARGETS: &[&str] = &[
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "wasm32-unknown-unknown",
];

/// The components one supported triple names: architecture, operating
/// system, and C library. `None` is a triple this compiler does not emit
/// for, and the caller names the supported set in its message.
fn triple_components(triple: &str) -> Option<(&'static str, &'static str, &'static str)> {
    Some(match triple {
        "aarch64-apple-darwin" => ("aarch64", "darwin", "libSystem"),
        "x86_64-apple-darwin" => ("x86_64", "darwin", "libSystem"),
        "x86_64-unknown-linux-gnu" => ("x86_64", "linux", "glibc"),
        "aarch64-unknown-linux-gnu" => ("aarch64", "linux", "glibc"),
        "wasm32-unknown-unknown" => ("wasm32", "unknown", "none"),
        _ => return None,
    })
}

/// The profile of one supported triple. The operating system,
/// architecture, C library, word size, and byte order come from the
/// triple, never from the build host, so a cross compile cannot inherit
/// the laptop's conditions. The remaining fields are the bootstrap's
/// target-independent defaults (`TargetProfile::native`), and the name is
/// the triple itself, so the store key and the artifact manifest identify
/// the same target wherever the compile runs.
fn profile_of_triple(triple: &str) -> Result<crate::core::TargetProfile> {
    let Some((arch, os, libc)) = triple_components(triple) else {
        return Err(Error::plain(format!(
            "unknown target `{triple}`: cross compilation supports exactly these triples: {}",
            SUPPORTED_TARGETS.join(", ")
        )));
    };
    if arch == "wasm32" {
        // The freestanding Wasm target: its own constructor already sets
        // the 32-bit word, the Wasm triple and the artifact shape.
        let mut profile = crate::core::TargetProfile::wasm32(crate::core::ArtifactProfile::CoreWasm);
        profile.name = triple.to_string();
        return Ok(profile);
    }
    let mut profile = crate::core::TargetProfile::native();
    profile.name = triple.to_string();
    profile.triple = triple.to_string();
    profile.os = os.to_string();
    profile.arch = arch.to_string();
    profile.libc = libc.to_string();
    // Every native triple in the table is 64-bit and little-endian; a
    // 32-bit or big-endian one joins with its own values here.
    profile.word_size = 64;
    profile.endian = "little".to_string();
    Ok(profile)
}

/// Does this profile name a target other than the one this compiler
/// runs on? A cross compile needs that target's runtime archive, and the
/// program it produces executes only there.
fn is_foreign(profile: &crate::core::TargetProfile) -> bool {
    profile.triple != crate::core::TargetProfile::native().triple
}

/// The target profile a caller asked for: the host's own target, or the
/// profile of one triple from [`SUPPORTED_TARGETS`], with an optional
/// explicit platform version.
pub fn target_profile(opts: &DriverOptions) -> Result<crate::core::TargetProfile> {
    let mut profile = match &opts.target {
        Some(requested) => profile_of_triple(requested)?,
        None => crate::core::TargetProfile::native(),
    };
    if let Some(version) = &opts.target_version {
        if version.trim().is_empty() {
            return Err(Error::plain("--target-version needs a version such as 14.0"));
        }
        profile.os_version = Some(version.trim().to_string());
        profile.name = format!("{}-os{}", profile.name, profile.os_version.as_deref().unwrap_or("-"));
    }
    Ok(profile)
}

/// [`driver`] with an explicit compile-time I/O mode (modules
/// specification section 10).
pub fn driver_with(
    mode: Mode,
    stage_mode: StageMode,
    path: &Path,
    out: Option<&Path>,
) -> Result<i32> {
    driver_with_profile(
        mode,
        stage_mode,
        path,
        out,
        &crate::core::TargetProfile::native(),
    )
}

/// [`driver_with`] with an explicit target profile (the driving bounds
/// and feature assumptions of supercompilation section 5 and platform
/// section 1).
pub fn driver_with_profile(
    mode: Mode,
    stage_mode: StageMode,
    path: &Path,
    out: Option<&Path>,
    profile: &crate::core::TargetProfile,
) -> Result<i32> {
    driver_with_profile_opts(
        mode,
        stage_mode,
        path,
        out,
        profile,
        &DriverOptions::default(),
    )
}

pub(crate) fn package_execution_path(object: &Path) -> PathBuf {
    object.with_file_name("build").join("package.cinchif")
}

/// The lent conventions the package objects publish: each unit row with a
/// `lent` vector (its worker only borrows those parameters) and the width
/// of the worker that takes them (`lent_width`). A row without them is a
/// unit whose symbol is owned, which is every unit of an older object.
fn package_lent_modes(objects: &[PathBuf]) -> Result<std::collections::BTreeMap<String, (Vec<bool>, usize)>> {
    let mut out = std::collections::BTreeMap::new();
    for object in objects {
        let abi = read_package_execution(object)?;
        for unit in abi.get("units").and_then(serde_json::Value::as_array).into_iter().flatten() {
            let (Some(symbol), Some(lent), Some(width)) = (
                unit.get("symbol").and_then(serde_json::Value::as_str),
                unit.get("lent").and_then(serde_json::Value::as_array),
                unit.get("lent_width").and_then(serde_json::Value::as_u64),
            ) else {
                continue;
            };
            let modes: Option<Vec<bool>> = lent.iter().map(serde_json::Value::as_bool).collect();
            let Some(modes) = modes else {
                return Err(Error::plain(format!("{}: malformed lent modes for {symbol}", object.display())));
            };
            out.insert(symbol.to_string(), (modes, width as usize));
        }
    }
    Ok(out)
}

/// Refuse a link whose objects disagree about a lent convention: every
/// `lent_imports` row an object recorded (what its calls into another
/// object assumed) must be exactly what the linked objects publish.
/// Modes follow from bodies, so a dependency rebuilt from changed source
/// can move them while its contract stays put; this is the check that
/// turns that into "rebuild the dependent" rather than a count imbalance.
fn check_lent_imports(
    objects: &[PathBuf],
    published: &std::collections::BTreeMap<String, (Vec<bool>, usize)>,
) -> Result<()> {
    for object in objects {
        let abi = read_package_execution(object)?;
        for row in abi.get("lent_imports").and_then(serde_json::Value::as_array).into_iter().flatten() {
            let symbol = row.get("symbol").and_then(serde_json::Value::as_str).unwrap_or("");
            let modes: Option<Vec<bool>> = row
                .get("lent")
                .and_then(serde_json::Value::as_array)
                .and_then(|m| m.iter().map(serde_json::Value::as_bool).collect());
            let width = row.get("lent_width").and_then(serde_json::Value::as_u64).map(|w| w as usize);
            let agrees = match (modes, width, published.get(symbol)) {
                (Some(m), Some(w), Some((pm, pw))) => &m == pm && w == *pw,
                _ => false,
            };
            if !agrees {
                return Err(Error::plain(format!(
                    "{}: compiled against a lent convention for {symbol} that the linked objects do not publish; rebuild it against the current dependency objects",
                    object.display()
                )));
            }
        }
    }
    Ok(())
}

pub(crate) fn read_package_execution(path: &Path) -> Result<serde_json::Value> {
    let abi_path = package_execution_path(path);
    let bytes = std::fs::read(&abi_path)
        .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?;
    let abi: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?;
    let object = path.file_name().and_then(|name| name.to_str());
    if abi.get("schema").and_then(serde_json::Value::as_u64) != Some(1)
        || abi.get("kind").and_then(serde_json::Value::as_str) != Some("package-execution")
        || object.is_none()
        || abi.get("object").and_then(serde_json::Value::as_str) != object
    {
        return Err(Error::plain(format!(
            "{}: invalid package execution interface for {}",
            abi_path.display(),
            path.display()
        )));
    }
    let runtime_abi = abi.get("runtime_abi").and_then(serde_json::Value::as_u64);
    let expected_runtime_abi = crate::interface::RUNTIME_ABI_VERSION;
    if runtime_abi != Some(u64::from(expected_runtime_abi)) {
        let found = runtime_abi.map_or_else(|| "missing".into(), |version| version.to_string());
        return Err(Error::plain(format!(
            "{}: package object runtime ABI {found} does not match this compiler's {expected_runtime_abi}",
            abi_path.display()
        )));
    }
    Ok(abi)
}

fn package_object_modules(path: &Path) -> Result<Vec<String>> {
    let abi_path = package_execution_path(path);
    let abi = read_package_execution(path)?;
    let expected_runtime_abi = crate::interface::RUNTIME_ABI_VERSION;
    let object_bytes =
        std::fs::read(path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let expected = abi
        .get("object_content_hash")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            Error::plain(format!(
                "{}: package object ABI has no object_content_hash",
                abi_path.display()
            ))
        })?;
    let actual = crate::interface::content_hash_bytes(&object_bytes);
    if expected != actual {
        return Err(Error::plain(format!(
            "{}: object content hash does not match the package execution interface",
            path.display()
        )));
    }
    if let Some(interfaces) = abi.get("interfaces").and_then(serde_json::Value::as_array) {
        let build = path.parent().map(|parent| parent.join("build"));
        for interface in interfaces {
            let Some(module) = interface.get("module").and_then(serde_json::Value::as_str) else {
                continue;
            };
            let Some(expected_hash) = interface
                .get("iface_hash")
                .and_then(serde_json::Value::as_str)
            else {
                continue;
            };
            let Some(path) = build
                .as_ref()
                .map(|build| build.join(format!("{module}.cinchif")))
            else {
                continue;
            };
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let iface = crate::interface::Iface::from_json(&text)
                .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
            if iface.runtime_abi != expected_runtime_abi {
                return Err(Error::plain(format!(
                    "{}: package interface runtime ABI {} does not match this compiler's {expected_runtime_abi}",
                    path.display(),
                    iface.runtime_abi
                )));
            }
            if iface.iface_hash() != expected_hash {
                return Err(Error::plain(format!(
                    "{}: interface hash does not match the package execution interface",
                    path.display()
                )));
            }
        }
    }
    let modules = abi
        .get("modules")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            Error::plain(format!(
                "{}: package object ABI has no modules array; rebuild it with this compiler",
                path.display()
            ))
        })?;
    modules
        .iter()
        .map(|module| {
            module.as_str().map(str::to_string).ok_or_else(|| {
                Error::plain(format!("{}: module entry is not a string", path.display()))
            })
        })
        .collect()
}

fn object_paths(objects: &[PathBuf]) -> Vec<String> {
    objects
        .iter()
        .map(|path| path.display().to_string())
        .collect()
}

/// [`driver_with_profile`] with explicit options: the whole compile,
/// with the flags that are neither a mode nor a profile.
#[allow(clippy::too_many_arguments)]
pub fn driver_with_profile_opts(
    mode: Mode,
    stage_mode: StageMode,
    path: &Path,
    out: Option<&Path>,
    profile: &crate::core::TargetProfile,
    opts: &DriverOptions,
) -> Result<i32> {
    if mode != Mode::EmitObject && !opts.object_roots.is_empty() {
        return Err(Error::plain(
            "explicit object roots require emit-object mode",
        ));
    }
    if mode != Mode::EmitObject && !opts.package_objects.is_empty() && mode == Mode::EmitIr {
        return Err(Error::plain("package objects require build or run mode"));
    }
    // `run` builds a binary and executes it here, so a program for
    // another target has nothing to run on: refuse before any work
    // instead of failing later inside the linker or, worse, executing a
    // foreign binary the host cannot start. A freestanding Wasm module is
    // the exception, and only because the host can carry it: Node loads
    // the module and calls its `main`, which is the entry a freestanding
    // module has instead of a process.
    if mode == Mode::Run && is_foreign(profile) && !is_freestanding(profile) {
        let host = crate::core::TargetProfile::native();
        return Err(Error::plain(format!(
            "target `{}`: `run` executes the program on this host, whose target is `{}`, and \
             a binary built for `{}` does not run here; use `cinch build --target {} \
             --runtime-archive <path>` and copy the artifact to the target",
            profile.triple, host.triple, profile.triple, profile.triple
        )));
    }
    // A Wasm module has no command line and no process, so `run` refuses
    // the words it could not deliver rather than dropping them.
    if mode == Mode::Run && is_freestanding(profile) && !opts.program_args.is_empty() {
        return Err(Error::plain(format!(
            "target `{}`: a freestanding Wasm module has no command line, so `run` cannot pass \
             {} program argument{}; pass them to the host that loads the module instead",
            profile.triple,
            opts.program_args.len(),
            if opts.program_args.len() == 1 { "" } else { "s" }
        )));
    }
    let mut emit_modules = opts.emit_modules.clone();
    // One build, two profiles (design/base-as-a-package.md, "Host and
    // target are different questions"): the host is this machine, and the
    // target is that machine unless the caller named a triple. Nothing
    // derives one from the other.
    let build = crate::core::BuildProfiles::of_target(profile);
    let profile = &build.target;
    let project = load_project_for(path, &build)?;
    // The final link is built from the closure, not from a string: every
    // contribution is tagged with the package that asked for it, raw
    // words need the root project's consent, and machine-local overrides
    // are applied last and recorded (section 4.3).
    let root_dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let root_manifest = project
        .packages
        .iter()
        .find(|(dir, _, _)| dir == &root_dir)
        .map(|(_, declared, _)| declared);
    let mut consent: Vec<String> = root_manifest
        .map(|declared| declared.raw_link.clone())
        .unwrap_or_default();
    for name in &opts.allow_raw_link {
        if !consent.contains(name) {
            consent.push(name.clone());
        }
    }
    let local_link = root_manifest
        .map(|declared| declared.local_link.clone())
        .unwrap_or_default();
    let resolved_packages: Vec<(PathBuf, ResolvedPackage)> = project
        .packages
        .iter()
        .map(|(dir, _, resolved)| (dir.clone(), resolved.clone()))
        .collect();
    let link_plan = LinkPlan::build(
        &resolved_packages,
        &consent,
        &local_link,
        &{
            let mut words = lock_native_words(&root_dir);
            for (name, resolved) in &opts.native_words {
                words.insert(name.clone(), resolved.clone());
            }
            words
        },
        &TargetFacts::of(profile),
    )?;
    if opts.print_link_plan {
        eprint!("{}", link_plan.describe());
    }
    if !opts.package_objects.is_empty() {
        let mut excluded = Vec::new();
        for object in &opts.package_objects {
            for module in package_object_modules(object)? {
                if !excluded.contains(&module) {
                    excluded.push(module);
                }
            }
        }
        emit_modules = project
            .order
            .iter()
            .filter(|module| !excluded.contains(module))
            .cloned()
            .collect();
    }
    let adapters = wit_for_source(path)?;
    let prelude = parse_prelude(&adapters)?;
    let prelude_fixities = prelude.base.clone();
    let mut module_fixities: HashMap<String, FixityTable> = prelude.fixities.clone();
    let mut exported_fixities = prelude.exported_fixities.clone();
    let mut parsed_by_file: HashMap<String, ParsedModules> = HashMap::new();
    let mut modules: Vec<crate::ast::Module> = Vec::new();
    // Each file parses once, at the position of its last module in
    // dependency order: every module of the file has its dependencies'
    // fixity tables by then.
    for name in &project.order {
        let file = project.module_file[name].clone();
        let key = file.display().to_string();
        if !parsed_by_file.contains_key(&key) {
            let src = &project.sources[&key];
            // A package that sets `prelude: none` parses its modules
            // without the prelude's operator table, so `1 + 2` is refused
            // instead of resolving through a fixity the module never
            // imported (design/base-as-a-package.md section 3).
            let no_prelude = project
                .module_file
                .iter()
                .filter(|(_, owner)| owner.display().to_string() == key)
                .all(|(module, _)| {
                    project
                        .module_preludes
                        .get(module)
                        .is_some_and(|prelude| prelude.is_empty())
                })
                && project
                    .module_file
                    .iter()
                    .any(|(_, owner)| owner.display().to_string() == key);
            let empty = FixityTable::default();
            let base = if no_prelude { &empty } else { &prelude_fixities };
            let parsed = parse_source(src, &key, base, &exported_fixities)?;
            for (m, f) in &parsed.fixities {
                module_fixities.insert(m.clone(), f.clone());
            }
            for (m, f) in &parsed.exported_fixities {
                exported_fixities.insert(m.clone(), f.clone());
            }
            parsed_by_file.insert(key, parsed);
        }
        let parsed = &parsed_by_file[&file.display().to_string()];
        let m = parsed
            .modules
            .iter()
            .find(|m| m.name.text == *name)
            .cloned()
            .ok_or_else(|| Error::plain(format!("{file:?}: no module {name}")))?;
        modules.push(m);
    }
    // Compile-time reachability (design/base-as-a-package.md, "Host and
    // target are different questions"): a module the build-time path
    // reaches has to resolve for the host, and the build compiles the
    // target's module set, so it has to resolve for the target too.
    // Ordinary code that reaches a module only the host resolves is
    // refused as unavailable for the target. Both decisions need the
    // parsed declarations, which is why they run here rather than in the
    // loader.
    refuse_unavailable_modules(&project, &modules, &build)?;
    // A library object roots every module's exported values (below), so
    // it must not root a module the target never selects: a platform
    // backend `Prim.Sys` imports only `when os darwin` is not part of a
    // Linux object, and rooting it linked Darwin's `__error` errno accessor
    // into one, which no Linux libc defines.
    let unselected = unselected_modules(prelude.modules.iter().chain(modules.iter()), profile)?;
    let mut hashes: HashMap<String, String> = HashMap::new();
    let mut files: HashMap<String, String> = HashMap::new();
    for name in &project.order {
        let key = project.module_file[name].display().to_string();
        hashes.insert(
            name.clone(),
            crate::interface::content_hash(&project.sources[&key]),
        );
        files.insert(name.clone(), key);
    }
    // The embedded family is hashed too, so its interfaces witness their
    // sources. A source module beside the entry shadows a family copy of
    // the same name and keeps its own hash (the insert above).
    for (name, hash) in family_source_hashes() {
        let file = format!("prelude/{name}.cinch");
        hashes.entry(name.clone()).or_insert(hash);
        files.entry(name).or_insert(file);
    }
    let dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let entry_stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let out_dir = dir.join("build");
    let manifest_path = out_dir.join(format!("{entry_stem}.manifest.json"));
    let prior_inputs = read_stage_snap(&out_dir.join(format!("{entry_stem}.stage.snap")));
    let scx = StagingCx {
        fixities: module_fixities.clone(),
        files,
        prior_inputs,
        mode: stage_mode,
        workdir: dir.clone(),
        entry_modules: project.entry_modules.clone(),
        schema_tool: std::env::var("CINCH_SCHEMA_TOOL").ok(),
    };
    let (program, ifaces, report, _diags) = check::check_program_multi_full_with_preludes(
        &prelude.modules,
        &modules,
        &hashes,
        &scx,
        &adapters,
        profile,
        &project.module_preludes,
    )?;
    // Metadata boundary checks (design note, section 3).
    crate::interface::verify_chain(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    crate::interface::check_coherence(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    // Interface optimisations (design/inline-unfolding-policy.md section
    // 12, mechanism A): the unfoldings a library artifact carries join the
    // inliner as foreign bodies, and the units they describe are external
    // to this compilation. The checked program still holds those bodies
    // (the embedded family is checked first), so they are removed here,
    // which is exactly the separate-compilation shape the artifact exists
    // for. A unit of a module this compilation defines keeps its own body:
    // a source module shadows the external one.
    let mut program = program;
    let (mut program, foreign) = if let Some(manifest) = &opts.interface_optimisations {
        let table = crate::ifaceopt::load_manifest(manifest)?;
        if table.bodies.is_empty() || mode == Mode::EmitObject {
            (program, table.bodies)
        } else {
            // Specialization runs first, with the library's own bodies still
            // present: a dictionary projection becomes a direct call to the
            // method unit, and the method's definition is what the artifact
            // describes. Only then do the external units leave this program.
            let mut no_simplify = opts.simplifier();
            no_simplify.max_iterations = 0;
            let (specialized, _) =
                crate::specialize::optimize_with(&program, profile, &no_simplify)?;
            let mut program = specialized;
            let local: std::collections::BTreeSet<&str> =
                ifaces.iter().map(|i| i.module.as_str()).collect();
            let module_of = |unit: &str| -> Option<String> {
                let mut best: Option<&str> = None;
                for m in table.modules.iter() {
                    if unit.starts_with(m.as_str())
                        && unit.as_bytes().get(m.len()) == Some(&b'.')
                        && best.map_or(true, |b: &str| m.len() > b.len())
                    {
                        best = Some(m);
                    }
                }
                best.map(str::to_string)
            };
            program.fns.retain(|f| match module_of(&f.id.0) {
                Some(m) => local.contains(m.as_str()),
                None => true,
            });
            program.instances.retain(|i| match module_of(&i.id.0) {
                Some(m) => local.contains(m.as_str()),
                None => true,
            });
            (program, table.bodies)
        }
    } else {
        (program, Vec::new())
    };
    // Core dumps (design/core-output.md section 2): the checked program
    // prints here, and the ownership stage prints the count pass applied to
    // that same program, which is why the two are target independent.
    let dump = CoreDump::new(opts, profile, &ifaces, &project.entry_modules, path, &out_dir, &entry_stem);
    let mut core_rows: Vec<CoreRow> = Vec::new();
    if dump.wants(crate::coretext::Stage::Checked)
        || (mode == Mode::Check && dump.stage().is_some())
    {
        dump.write(&program, crate::coretext::Stage::Checked, &ifaces, &mut core_rows)?;
    }
    let owned_program = if dump.wants(crate::coretext::Stage::Owned) && mode != Mode::Check {
        Some(crate::rc::insert(program.clone(), profile)?)
    } else {
        None
    };
    if let Some(owned) = &owned_program {
        dump.write(owned, crate::coretext::Stage::Owned, &ifaces, &mut core_rows)?;
    }
    // Specialization (supercompilation and JIT specification sections
    // 2-5): every residual program is rechecked before installation, and
    // the report records where specialization stopped.
    let (mut program, mut opt, method_families) = if mode == Mode::EmitObject {
        crate::specialize::optimize_object_with_foreign(
            &program,
            profile,
            &opts.object_roots,
            &foreign,
            &opts.simplifier(),
        )?
    } else if !foreign.is_empty() {
        // The split above already specialized and removed the external
        // units, so this step is the simplifier only.
        let mut opt = crate::core::OptReport::default();
        crate::simplify::simplify_with_foreign(
            &mut program,
            &foreign,
            &opts.simplifier(),
            &mut opt.simplify,
        );
        (program, opt, Vec::new())
    } else {
        let (program, opt) =
            crate::specialize::optimize_with_foreign(&program, profile, &foreign, &opts.simplifier())?;
        (program, opt, Vec::new())
    };
    // Dead-unit elimination (design/stack-allocation.md section 7.4): after
    // inlining, whose copies leave a callee with no call site, and before
    // reference counting, so the removal is not paid for. `check` prunes
    // nothing: it emits no artifact.
    // `CINCH_NO_PRUNE=1` takes the measurement the other way: the same
    // program with every unit the sweep would remove still in it.
    if mode != Mode::Check && std::env::var_os("CINCH_NO_PRUNE").is_none() {
        let mut roots: Vec<String> = if mode == Mode::EmitObject {
            emit_modules.clone()
        } else {
            Vec::new()
        };
        // An object build also names its entry points, and a Wasm world's
        // exported bindings are called from the component side.
        for root in &opts.object_roots {
            if let crate::specialize::ObjectRoot::Unit { symbol, .. }
            | crate::specialize::ObjectRoot::Dictionary { symbol, .. } = root
            {
                roots.push(symbol.clone());
            }
        }
        for a in &adapters {
            for b in &a.bindings {
                if b.export {
                    roots.push(b.impl_unit.clone());
                }
            }
        }
        // Every unit an exported interface names. A consumer resolves a
        // unit through the identity its interface publishes, so a package
        // object that declares the value must define it even when no root
        // demands it in this build; `prune` drops names no unit carries.
        // A whole-program artifact has no consumer to resolve against, and
        // keeping its interface names would only add unreachable units.
        if mode == Mode::EmitObject {
            for iface in ifaces.iter().filter(|iface| !unselected.contains(&iface.module)) {
                for value in &iface.exports.values {
                    roots.push(value.symbol.clone());
                    if let Some(graph) = &value.total_body {
                        for definition in graph.definitions() {
                            roots.push(definition.identity.clone());
                        }
                    }
                }
                for signature in &iface.exports.signatures {
                    for method in &signature.methods {
                        roots.extend(method.exports.iter().cloned());
                    }
                }
                for instance in &iface.exports.instances {
                    roots.push(instance.id.clone());
                    for body in &instance.method_bodies {
                        roots.push(body.implementation.clone());
                    }
                }
                for unit in &iface.units {
                    roots.push(unit.name.clone());
                }
            }
        }
        let pruned = if mode == Mode::EmitObject {
            crate::simplify::prune_excluding(&mut program, &roots, &unselected)
        } else {
            crate::simplify::prune(&mut program, &roots)
        };
        opt.simplify.pruned = pruned.removed;
        opt.simplify.pruned_nodes = pruned.removed_nodes;
    }
    write_artifacts(
        &out_dir,
        &entry_stem,
        &ifaces,
        &manifest_path,
        &report,
        &program,
        &opt,
        profile,
        &build,
        &project.availability,
        &adapters,
        opts,
        &link_plan,
        opts.emit_interface_optimisations || mode == Mode::EmitObject,
    )?;
    if mode == Mode::Check {
        // A check reaches no later stage, so `--dump-core=owned` and
        // `--dump-core=lowered` dump the checked program and say so in
        // `@stage(checked)`: the dump never claims a stage the mode did not
        // reach.
        dump.record(&manifest_path, &core_rows, None)?;
        return Ok(0);
    }
    // Artifact admission (platform proofs and SIMD specification
    // section 3): `check` is not admission. Producing or running the
    // artifact enforces the assumed deployment baseline and the world's
    // implementation obligations.
    admit_artifact(profile, &program, &adapters)?;
    // Reference counting goes in after specialization (design/reclamation.md
    // section 4): driving duplicates and discards subterms, which would
    // unbalance inserted count operations.
    // An executable emitted whole calls its units only from itself, so
    // parameters may be inferred borrowed; an object, a module subset or
    // a link against package objects keeps every parameter owned.
    // An object, a module subset or a link against package objects is open:
    // its units infer modes too, and each one that lends keeps an owned
    // public symbol over a lent worker, while the units the linked objects
    // define are called with the modes those objects published
    // (`crate::rc::insert_open`, `package_lent_modes`).
    let closed = mode != Mode::EmitObject && emit_modules.is_empty() && opts.package_objects.is_empty();
    let program = if closed {
        crate::rc::insert_closed(program, profile)?
    } else {
        let sources: Vec<PathBuf> = opts.package_objects.iter().chain(&opts.lent_objects).cloned().collect();
        let imported = package_lent_modes(&sources)?;
        check_lent_imports(&opts.package_objects, &imported)?;
        let boundary = crate::rc::Boundary { modules: &emit_modules, imported: &imported, closed: false };
        crate::rc::insert_open(program, profile, &boundary)?
    };
    // Coalescing comes after counting and after the reuse insertion inside
    // it, which reads the dups where counting put them (Lean's order:
    // `expandResetReuse` then `coalesceRC`, `CoalesceRC.lean`). It moves
    // each value's dups to its first use in a run and its drops after its
    // last, which shortens the live ranges the backend carries.
    let mut program = program;
    crate::coalesce::coalesce(&mut program, profile);
    // Frame allocation comes after counting, where every allocation and
    // every reference is explicit (design/stack-allocation.md): the pass
    // turns a block whose references all die in its activation into a
    // 16-byte-aligned stack slot and elides its count operations.
    let program = stack_alloc(program, profile, opts);
    // The lowered stage is the exact code-generation input, so it prints
    // here, after specialization, reference counting and frame allocation.
    if dump.wants(crate::coretext::Stage::Lowered) && mode != Mode::Check {
        dump.write_lowered(&program, &opt, &ifaces, &mut core_rows)?;
    }
    if opts.core_only {
        dump.record(&manifest_path, &core_rows, Some(&program.stack))?;
        return Ok(0);
    }
    dump.record(&manifest_path, &core_rows, Some(&program.stack))?;
    if mode == Mode::EmitObject {
        return write_object(
            profile,
            &program,
            &ifaces,
            &method_families,
            path,
            out,
            &emit_modules,
            opts.opt_level.unwrap_or(2),
            &out_dir,
        );
    }
    let ir = if emit_modules.is_empty() {
        codegen::emit_for(profile, &program)?
    } else {
        codegen::emit_for_modules(profile, &program, &emit_modules)?
    };
    match mode {
        Mode::Check | Mode::EmitObject => unreachable!(),
        Mode::EmitIr => {
            match out {
                Some(out) => {
                    std::fs::write(out, &ir)
                        .map_err(|e| Error::plain(format!("{}: {e}", out.display())))?;
                }
                None => print!("{ir}"),
            }
            Ok(0)
        }
        Mode::Build => {
            let out = out
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| path.with_extension(""));
            let ir_path = out.with_extension("ll");
            std::fs::write(&ir_path, &ir)
                .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
            let objects = object_paths(&opts.package_objects);
            let refs: Vec<&str> = objects.iter().map(String::as_str).collect();
            link_for(&ir_path, &out, &refs, &link_plan, &link_target(profile, opts))?;
            Ok(0)
        }
        Mode::Run => {
            let tmp = scratch_dir("run")?;
            let ir_path = tmp.join("program.ll");
            // A freestanding target's artifact is a module, not an
            // executable: the `.wasm` suffix is what says so, and what a
            // host loads.
            let exe = tmp.join(if is_freestanding(profile) { "program.wasm" } else { "program" });
            std::fs::write(&ir_path, ir)
                .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
            let objects = object_paths(&opts.package_objects);
            let refs: Vec<&str> = objects.iter().map(String::as_str).collect();
            if is_freestanding(profile) {
                link_for(&ir_path, &exe, &refs, &link_plan, &link_target(profile, opts))?;
                let code = run_wasm_module(&exe, profile)?;
                let _ = std::fs::remove_dir_all(&tmp);
                return Ok(code);
            }
            link_for(&ir_path, &exe, &refs, &link_plan, &LinkTarget { opt_level: opts.opt_level.unwrap_or(0), ..LinkTarget::default() })?;
            // The program's own command line (the CLI's forwarded
            // words): the entry glue captures them, so the built
            // program reads them with `process-args`.
            let status = std::process::Command::new(&exe)
                .args(&opts.program_args)
                .status()
                .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
            let _ = std::fs::remove_dir_all(&tmp);
            Ok(status.code().unwrap_or(1))
        }
    }
}

/// Emit a closed library dependency object. Its physical manifest records the
/// complete checked interface closure; source exports still come only from
/// those interfaces. The final executable supplies apply and one layout table.
fn write_object(
    profile: &crate::core::TargetProfile,
    program: &crate::core::Program,
    ifaces: &[crate::interface::Iface],
    families: &[crate::specialize::MethodFamily],
    path: &Path,
    out: Option<&Path>,
    emit_modules: &[String],
    opt_level: u8,
    artifacts: &Path,
) -> Result<i32> {
    use crate::interface::{parse_json, JsonValue as J};
    let object = codegen::emit_object_for_modules(profile, program, emit_modules)?;
    let out = out
        .map(Path::to_path_buf)
        .unwrap_or_else(|| path.with_extension("o"));
    let ir_path = out.with_extension("ll");
    let abi_path = if emit_modules.is_empty() {
        out.with_extension("abi.json")
    } else {
        package_execution_path(&out)
    };
    if out == ir_path || out == abi_path {
        return Err(Error::plain(
            "emit-object output must differ from its LLVM and ABI companions",
        ));
    }
    // A failed rebuild must not leave an old manifest advertising new code.
    if abi_path.exists() {
        std::fs::remove_file(&abi_path)
            .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?;
    }
    std::fs::write(&ir_path, &object.ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    let mut assemble = std::process::Command::new(clang());
    // Library objects are assembled once and linked into every consumer,
    // so they are optimized by default (`--opt-level`, 2 unless given):
    // unoptimized objects keep every local in a stack slot and every call
    // out of line.
    assemble.arg(format!("-O{opt_level}")).arg("-c").arg(&ir_path).arg("-o").arg(&out);
    // An object for a foreign target is assembled for that target: the
    // triple decides which instruction set the body's `asm` is, and the
    // emitted module says so in its own header.
    if is_foreign(profile) {
        assemble.arg("-target").arg(&profile.triple);
    }
    let output = assemble
        .output()
        .map_err(|e| Error::plain(format!("compile library object: {e}")))?;
    if !output.status.success() {
        return Err(Error::plain(format!(
            "compile library object failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let mut abi = parse_json(&object.abi).map_err(|e| Error::plain(e.to_string()))?;
    let J::Obj(ref mut fields) = abi else {
        unreachable!()
    };
    if !emit_modules.is_empty() {
        fields.insert("kind".into(), "package-execution".into());
        fields.insert("modules".into(), J::arr(emit_modules, |name| name.as_str().into()));
        let name = out.file_name().and_then(|name| name.to_str())
            .ok_or_else(|| Error::plain("package object needs a UTF-8 file name"))?;
        fields.insert("object".into(), name.into());
        let directory = abi_path.parent().expect("package execution interface directory");
        std::fs::create_dir_all(directory)
            .map_err(|e| Error::plain(format!("{}: {e}", directory.display())))?;
    }
    let interfaces = ifaces
        .iter()
        .map(|iface| {
            // The optimisation artifact, when the module has one, is a
            // sibling of its contract. The package records its content hash
            // so a consumer can pin the unfoldings it inlines with the same
            // rigor as the object they came from.
            let opt_path = artifacts.join(format!("{}.cinchopt", iface.module));
            let optimisation = match std::fs::read_to_string(&opt_path) {
                Ok(text) => {
                    let artifact = crate::ifaceopt::Optimisation::from_json(&text)
                        .map_err(|e| Error::plain(format!("{}: {e}", opt_path.display())))?;
                    J::obj([
                        ("schema", crate::ifaceopt::SCHEMA.into()),
                        ("iface_hash", artifact.iface_hash.as_str().into()),
                        ("content_hash", artifact.content_hash().as_str().into()),
                        ("bodies", (artifact.bodies.len() as u64).into()),
                    ])
                }
                Err(_) => J::Null,
            };
            Ok(J::obj([
                ("module", iface.module.as_str().into()),
                ("iface_hash", iface.iface_hash().into()),
                (
                    "interface",
                    parse_json(&iface.to_json()).map_err(|e| Error::plain(e.to_string()))?,
                ),
                ("optimisation", optimisation),
            ]))
        })
        .collect::<Result<Vec<_>>>()?;
    fields.insert("interfaces".into(), J::Arr(interfaces));
    fields.insert(
        "method_families".into(),
        J::Arr(
            families
                .iter()
                .map(|family| {
                    J::obj([
                        ("class", family.class.as_str().into()),
                        ("declared_methods", (family.declared_methods as u64).into()),
                        (
                            "slots",
                            J::Arr(
                                family
                                    .slots
                                    .iter()
                                    .map(|slot| {
                                        J::obj([
                                            ("logical", (slot.logical as u64).into()),
                                            ("physical", (slot.physical as u64).into()),
                                            ("own", crate::repr::letters(&slot.own).into()),
                                        ])
                                    })
                                    .collect(),
                            ),
                        ),
                    ])
                })
                .collect(),
        ),
    );
    let bytes = std::fs::read(&out).map_err(|e| Error::plain(format!("{}: {e}", out.display())))?;
    fields.insert(
        "object_content_hash".into(),
        crate::interface::content_hash_bytes(&bytes).into(),
    );
    std::fs::write(&abi_path, abi.to_canonical_string())
        .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?;
    Ok(0)
}

/// Compile and run a source string; used by tests. Returns the exit code.
pub fn run_source(src: &str) -> Result<i32> {
    run_source_with(src, &mut StagingCx::default())
}

/// [`run_source`] with an explicit staging context.
pub fn run_source_with(src: &str, scx: &mut StagingCx) -> Result<i32> {
    let program = check_sources_with(src, "<test>", scx)?;
    run_program(&program, &crate::core::TargetProfile::native())
}

/// Compile and run a source string under an explicit target profile
/// (the driving bounds are profile parameters; a zero fuel runs nothing
/// and is the unfused baseline).
pub fn run_source_profile(src: &str, profile: &crate::core::TargetProfile) -> Result<i32> {
    let program = check_sources_with(src, "<test>", &mut StagingCx::default())?;
    run_program(&program, profile)
}

/// Specialize, compile and run a checked program.
pub fn run_program(
    program: &crate::core::Program,
    profile: &crate::core::TargetProfile,
) -> Result<i32> {
    run_program_objects(program, profile, &[])
}

/// [`run_program`] linked with extra objects (a `foreign export` program
/// may also leave `main` to the C side; foreign function interface
/// specification section 1).
pub fn run_program_objects(
    program: &crate::core::Program,
    profile: &crate::core::TargetProfile,
    objects: &[&str],
) -> Result<i32> {
    let (program, _opt) = crate::specialize::optimize(program, profile)?;
    let program = crate::rc::insert_closed(program, profile)?;
    let program = stack_alloc(program, profile, &DriverOptions::default());
    let ir = codegen::emit_for(profile, &program)?;
    let tmp = scratch_dir("test")?;
    let ir_path = tmp.join("program.ll");
    // The artifact a target produces is the target's: an executable for a
    // native one, a module for a freestanding one.
    let exe = tmp.join(if is_freestanding(profile) { "program.wasm" } else { "program" });
    std::fs::write(&ir_path, ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    // A profile that is not the host's links the way `build` does for that
    // target, so a test that runs a cross profile executes it where the
    // host can carry it and is refused where it cannot.
    let opts = DriverOptions::default();
    let target = link_target(profile, &opts);
    if is_freestanding(profile) || is_foreign(profile) {
        link_for(&ir_path, &exe, objects, &LinkPlan::default(), &target)?;
        let code = if is_freestanding(profile) {
            run_wasm_module(&exe, profile)?
        } else {
            return Err(Error::plain(format!(
                "target `{}`: a program built for it does not run on this host, whose target is `{}`",
                profile.triple,
                crate::core::TargetProfile::native().triple
            )));
        };
        let _ = std::fs::remove_dir_all(&tmp);
        return Ok(code);
    }
    link_with(&ir_path, &exe, objects, &LinkPlan::default())?;
    let status = std::process::Command::new(&exe)
        .status()
        .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
    let _ = std::fs::remove_dir_all(&tmp);
    Ok(status.code().unwrap_or(1))
}

/// Compile a source file to LLVM IR and execute its `main` through the
/// embedded ORC backend. The runtime is loaded as a shared library so JIT'd
/// code resolves the same `cinch_*` boundary as a native build.
#[cfg(feature = "jit")]
pub fn jit_file(path: &Path) -> Result<i32> {
    let tmp =
        tempfile::tempdir().map_err(|e| Error::plain(format!("jit scratch directory: {e}")))?;
    let ir_path = tmp.path().join("program.ll");
    driver_opts(
        Mode::EmitIr,
        path,
        Some(&ir_path),
        &DriverOptions::default(),
    )?;
    let ir = std::fs::read_to_string(&ir_path)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    let runtime = ensure_runtime_dylib()?;
    crate::jit::run_main(&ir, &runtime)
}

/// Load one installed package object into an embedded LLJIT engine and call
/// one of its zero-arity units.
///
/// The engine loads the runtime shared library and registers the layout
/// prefix the object's physical manifest records before the lookup, so a
/// unit that allocates, copies, or drops values runs against the same
/// runtime boundary as a native build. The symbol must name a unit the
/// manifest defines with arity zero; a call through a mismatched physical
/// signature is refused rather than made.
#[cfg(feature = "jit")]
pub fn jit_package_symbol(object: &Path, dependencies: &[PathBuf], symbol: &str) -> Result<i64> {
    let runtime = ensure_runtime_dylib()?;
    let mut engine = crate::jit::JitEngine::new(Some(&runtime))?;
    for dependency in dependencies {
        engine.add_package_object_dependency(dependency)?;
    }
    let package = engine.add_package_object(object)?;
    let arity = package.units.get(symbol).copied().ok_or_else(|| {
        Error::plain(format!(
            "{}: the object's ABI manifest defines no unit {symbol:?} ({} units recorded)",
            package.object.display(),
            package.units.len()
        ))
    })?;
    if arity != 0 {
        return Err(Error::plain(format!(
            "{}: unit {symbol:?} has arity {arity}; this entry point calls zero-arity units only",
            package.object.display()
        )));
    }
    engine.call_i64(symbol)
}

/// Compile and run a source string linked with extra objects; used by
/// the foreign binding tests.
pub fn run_source_objects(src: &str, objects: &[&str]) -> Result<i32> {
    let program = check_sources_with(src, "<test>", &mut StagingCx::default())?;
    run_program_objects(&program, &crate::core::TargetProfile::native(), objects)
}

/// Check and compile a source string into the executable `out`, linked
/// with extra objects, without running it: for a test that runs the
/// program under an environment of its own (the runtime diagnostics).
pub fn build_source_objects(src: &str, objects: &[&str], out: &Path) -> Result<()> {
    let profile = crate::core::TargetProfile::native();
    let program = check_sources_with(src, "<test>", &mut StagingCx::default())?;
    let (program, _opt) = crate::specialize::optimize(&program, &profile)?;
    let program = crate::rc::insert_closed(program, &profile)?;
    let program = stack_alloc(program, &profile, &DriverOptions::default());
    let ir = codegen::emit_for(&profile, &program)?;
    let ir_path = out.with_extension("ll");
    std::fs::write(&ir_path, &ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    link_with(&ir_path, out, objects, &LinkPlan::default())
}

/// Compile and run a source string with the runtime's input source fed
/// from `input` and its output sink captured; used by the runtime I/O
/// and concurrency tests (concurrency specification, sections 4 and 9).
pub fn run_source_io(src: &str, input: &[u8]) -> Result<(i32, String)> {
    let program = check_sources_with(src, "<test>", &mut StagingCx::default())?;
    run_program_io(&program, &crate::core::TargetProfile::native(), input)
}

/// [`run_source_io`] at the program level.
pub fn run_program_io(
    program: &crate::core::Program,
    profile: &crate::core::TargetProfile,
    input: &[u8],
) -> Result<(i32, String)> {
    let (program, _opt) = crate::specialize::optimize(program, profile)?;
    let program = crate::rc::insert_closed(program, profile)?;
    let program = stack_alloc(program, profile, &DriverOptions::default());
    let ir = codegen::emit_for(profile, &program)?;
    let tmp = scratch_dir("test")?;
    let ir_path = tmp.join("program.ll");
    let exe = tmp.join("program");
    std::fs::write(&ir_path, ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    link_with(&ir_path, &exe, &[], &LinkPlan::default())?;
    use std::io::Write;
    let mut child = std::process::Command::new(&exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
    child
        .stdin
        .take()
        .ok_or_else(|| Error::plain("child stdin unavailable"))?
        .write_all(input)
        .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
    let out = child
        .wait_with_output()
        .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
    let _ = std::fs::remove_dir_all(&tmp);
    Ok((
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    ))
}
