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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Check,
    EmitIr,
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
    ("Data/Core.cinch", include_str!("../prelude/Data/Core.cinch")),
    ("Relation/Eq.cinch", include_str!("../prelude/Relation/Eq.cinch")),
    ("Relation/Ord.cinch", include_str!("../prelude/Relation/Ord.cinch")),
    ("Algebra/Arithmetic.cinch", include_str!("../prelude/Algebra/Arithmetic.cinch")),
    ("Algebra/Bits.cinch", include_str!("../prelude/Algebra/Bits.cinch")),
    ("Function/Combinators.cinch", include_str!("../prelude/Function/Combinators.cinch")),
    ("Data/Nat.cinch", include_str!("../prelude/Data/Nat.cinch")),
    ("Data/Int.cinch", include_str!("../prelude/Data/Int.cinch")),
    ("Data/Option.cinch", include_str!("../prelude/Data/Option.cinch")),
    ("Data/Result.cinch", include_str!("../prelude/Data/Result.cinch")),
    ("Data/List.cinch", include_str!("../prelude/Data/List.cinch")),
    // The enumeration class behind the range literals collects after
    // `Data.List` (its `List` type); its instances live with their
    // types in `Data.Nat`/`Data.Int`.
    ("Data/Enum.cinch", include_str!("../prelude/Data/Enum.cinch")),
    ("Data/Ordering.cinch", include_str!("../prelude/Data/Ordering.cinch")),
    ("Data/Char.cinch", include_str!("../prelude/Data/Char.cinch")),
    ("Data/Bool.cinch", include_str!("../prelude/Data/Bool.cinch")),
    ("Data/Machine.cinch", include_str!("../prelude/Data/Machine.cinch")),
    ("Data/Bytes.cinch", include_str!("../prelude/Data/Bytes.cinch")),
    ("Data/Map.cinch", include_str!("../prelude/Data/Map.cinch")),
    ("Data/String.cinch", include_str!("../prelude/Data/String.cinch")),
    ("Data/Json.cinch", include_str!("../prelude/Data/Json.cinch")),
    ("Data/Producer.cinch", include_str!("../prelude/Data/Producer.cinch")),
    ("Data/Target.cinch", include_str!("../prelude/Data/Target.cinch")),
    ("Data/Path.cinch", include_str!("../prelude/Data/Path.cinch")),
    ("Effect/Sequencing.cinch", include_str!("../prelude/Effect/Sequencing.cinch")),
    ("Effect/Concurrent.cinch", include_str!("../prelude/Effect/Concurrent.cinch")),
    ("Effect/Clock.cinch", include_str!("../prelude/Effect/Clock.cinch")),
    ("Effect/Fs.cinch", include_str!("../prelude/Effect/Fs.cinch")),
    ("Effect/Process.cinch", include_str!("../prelude/Effect/Process.cinch")),
    ("Effect/Io.cinch", include_str!("../prelude/Effect/Io.cinch")),
    ("Data/FromList.cinch", include_str!("../prelude/Data/FromList.cinch")),
    ("Data/FromMap.cinch", include_str!("../prelude/Data/FromMap.cinch")),
    ("Text/Show.cinch", include_str!("../prelude/Text/Show.cinch")),
    // `Data.Sym` collects after `Text.Show` (its `Show` instance) and
    // `Data.String` (its name conversions); the facade collects after
    // the whole family.
    ("Data/Sym.cinch", include_str!("../prelude/Data/Sym.cinch")),
    // The class modules and the newer families continue here; the
    // manifest's order, not this table's, decides where each is checked.
    ("Algebra/Semigroup.cinch", include_str!("../prelude/Algebra/Semigroup.cinch")),
    ("Algebra/Group.cinch", include_str!("../prelude/Algebra/Group.cinch")),
    ("Algebra/Semiring.cinch", include_str!("../prelude/Algebra/Semiring.cinch")),
    ("Algebra/Monoid.cinch", include_str!("../prelude/Algebra/Monoid.cinch")),
    ("Category/Functor.cinch", include_str!("../prelude/Category/Functor.cinch")),
    ("Category/Applicative.cinch", include_str!("../prelude/Category/Applicative.cinch")),
    ("Category/Monad.cinch", include_str!("../prelude/Category/Monad.cinch")),
    ("Category/Alternative.cinch", include_str!("../prelude/Category/Alternative.cinch")),
    ("Category/Foldable.cinch", include_str!("../prelude/Category/Foldable.cinch")),
    ("Category/Traversable.cinch", include_str!("../prelude/Category/Traversable.cinch")),
    ("Data/Identity.cinch", include_str!("../prelude/Data/Identity.cinch")),
    ("Data/Const.cinch", include_str!("../prelude/Data/Const.cinch")),
    ("Data/Compose.cinch", include_str!("../prelude/Data/Compose.cinch")),
    ("Data/NonEmpty.cinch", include_str!("../prelude/Data/NonEmpty.cinch")),
    ("Data/Sorted.cinch", include_str!("../prelude/Data/Sorted.cinch")),
    ("Data/NonZero.cinch", include_str!("../prelude/Data/NonZero.cinch")),
    ("Data/Ascii.cinch", include_str!("../prelude/Data/Ascii.cinch")),
    ("Data/Set.cinch", include_str!("../prelude/Data/Set.cinch")),
    ("Data/Hash.cinch", include_str!("../prelude/Data/Hash.cinch")),
    ("Text/Codec.cinch", include_str!("../prelude/Text/Codec.cinch")),
    ("Data/HashMap.cinch", include_str!("../prelude/Data/HashMap.cinch")),
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
    pub modules: Vec<PackageModule>,
    pub order: Vec<String>,
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
        use crate::interface::{JsonValue, parse_json};
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
            let o = m.as_object().map_err(wrap)?;
            let name = match o.get("name").map(|v| v.as_str()) {
                Some(Ok(s)) => s.to_string(),
                _ => return Err(Error::plain("package manifest: a module entry needs `name`")),
            };
            let path = match o.get("path").map(|v| v.as_str()) {
                Some(Ok(s)) => s.to_string(),
                _ => return Err(Error::plain("package manifest: a module entry needs `path`")),
            };
            // `embedded` is recorded for a reader's benefit; the source
            // table is what decides whether a path has bytes.
            let facade = matches!(o.get("facade"), Some(JsonValue::Bool(true)));
            modules.push(PackageModule {
                name,
                path,
                facade,
            });
        }
        let mut order: Vec<String> = Vec::new();
        for n in field("order")?.as_array().map_err(wrap)? {
            order.push(str_of(n, "an order entry")?);
        }
        Ok(Package {
            name,
            version,
            modules,
            order,
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
                let m = self.modules.iter().find(|m| &m.name == name).ok_or_else(|| {
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

fn str_of(v: &crate::interface::JsonValue, what: &str) -> Result<String> {
    v.as_str()
        .map(|s| s.to_string())
        .map_err(|e| Error::plain(format!("package manifest: {what}: {e}")))
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
    pub base: FixityTable,
}

/// Parse the embedded family with the WIT adapters' generated
/// vocabulary appended to the facade's source file (the vocabulary
/// declares inside the facade and joins its export list).
pub fn parse_prelude(adapters: &[crate::interface::WitAdapter]) -> Result<PreludeSources> {
    let empty = FixityTable::new();
    let facade_src = prelude_source_with(adapters);
    let facade_parsed = parse_source(&facade_src, "Prelude.cinch", &empty, &HashMap::new())?;
    let base = facade_parsed
        .fixities
        .get("Prelude")
        .cloned()
        .unwrap_or_default();
    let mut fixities = facade_parsed.fixities.clone();
    let mut early: Vec<crate::ast::Module> = Vec::new();
    let mut late: Vec<crate::ast::Module> = Vec::new();
    let mut tables: HashMap<String, FixityTable> = HashMap::new();
    let mut parse_into =
        |src: &str, path: &str, out: &mut Vec<crate::ast::Module>| -> Result<()> {
            let parsed = parse_source(src, path, &base, &tables)?;
            for (m, f) in &parsed.fixities {
                fixities.insert(m.clone(), f.clone());
                tables.insert(m.clone(), f.clone());
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
    let facade_at = base_package().facade_index();
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
pub fn scan_file(src: &str, file: &str) -> Result<ScannedFile> {
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
                if let Some(name) = path_at(&mut i) {
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
/// first), the entry file's modules last, with each module's source file
/// and imports.
struct Project {
    /// Module name to the file declaring it (a file may declare several).
    module_file: HashMap<String, PathBuf>,
    /// File to source text.
    sources: HashMap<String, String>,
    /// Module to its import names.
    imports: HashMap<String, Vec<String>>,
    /// The entry file's modules, in declaration order.
    entry_modules: Vec<String>,
    /// Module names in dependency order.
    order: Vec<String>,
}

/// Every `module X` declaration beside `dir`, for import resolution when
/// `X.cinch` does not exist (decision D42).
fn index_dir_modules(dir: &Path) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(index),
    };
    for entry in entries.flatten() {
        let path = entry.path();
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
    Ok(index)
}

/// Load the entry file and every module it imports, transitively. A source
/// file may declare any number of modules (the staging slice supersedes the
/// one-module-per-file rule); `import X` resolves to a module of an already
/// loaded file, then to `X.cinch`, then to a unique `module X` beside the
/// entry (decision D42).
fn load_project(entry_path: &Path) -> Result<Project> {
    let dir = entry_path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let mut sources: HashMap<String, String> = HashMap::new();
    let mut module_file: HashMap<String, PathBuf> = HashMap::new();
    let mut imports: HashMap<String, Vec<String>> = HashMap::new();
    let mut entry_modules: Vec<String> = Vec::new();
    let mut queue: Vec<PathBuf> = vec![entry_path.to_path_buf()];
    let dir_index = index_dir_modules(&dir)?;
    while let Some(path) = queue.pop() {
        let key = path.display().to_string();
        if sources.contains_key(&key) {
            continue;
        }
        let src = std::fs::read_to_string(&path)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        let scanned = scan_file(&src, &key)?;
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
        for (_, imps) in &scanned.modules {
            for (imp, _stage) in imps {
                if module_file.contains_key(imp) {
                    continue;
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
    // Dependency order; the entry file's modules go last, in declaration
    // order, because their dependencies precede them and a cycle is an
    // error.
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
    let mut order = crate::interface::topo_order(&graph)
        .map_err(|e| Error::plain(e.to_string()))?;
    order.retain(|m| module_file.contains_key(m));
    let entry: Vec<String> = entry_modules.clone();
    order.retain(|m| !entry.contains(m));
    for m in &entry {
        order.push(m.clone());
    }
    Ok(Project {
        module_file,
        sources,
        imports,
        entry_modules,
        order,
    })
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
    wit: &[crate::interface::WitAdapter],
    opts: &DriverOptions,
) -> Result<()> {
    std::fs::create_dir_all(out_dir)
        .map_err(|e| Error::plain(format!("{}: {e}", out_dir.display())))?;
    for iface in ifaces {
        let p = out_dir.join(format!("{}.cinchif", iface.module));
        std::fs::write(&p, iface.to_json())
            .map_err(|e| Error::plain(format!("{}: {e}", p.display())))?;
    }
    let mut modules: Vec<(&str, String)> =
        ifaces.iter().map(|i| (i.module.as_str(), i.iface_hash())).collect();
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
            .map(|(n, t)| format!("{n} :: {t}"))
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
                ("core_params", J::arr(&b.call.core_params, |t| t.spelling().into())),
                ("core_results", J::arr(&b.call.core_results, |t| t.spelling().into())),
            ]);
            let J::Obj(fields) = &mut row else { unreachable!() };
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
            vec!["LLVM code generation", "the runtime and its instruction model"].into(),
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
            J::Obj(report.generated.iter().map(|(m, names)| (m.clone(), names.into())).collect()),
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
        ("runtime_abi", 1u64.into()),
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
        ("wasm", wasm),
        ("specialization", specialization),
        ("tail_calls", tail_calls),
        ("literal_folds", literal_folds),
        ("data_imports", data_imports),
        ("safety", safety),
        ("temporal", temporal),
        ("trust", trust),
        ("lowering", lowering),
        ("staging", staging),
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
        let pkg = crate::parser::parse_wit(&text).map_err(|e| {
            Error::plain(format!("{}: {e}", w.display()))
        })?;
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
) -> Result<(PreludeSources, crate::parser::ParsedModules, HashMap<String, String>)> {
    let prelude = parse_prelude(adapters)?;
    // The user source parses against the facade's base table plus every
    // embedded family module's own fixity table: an operator's fixity is
    // part of the module that declares it (the interface fact of
    // `design/compiled-interface-metadata.md` section 2.1, rule 3), so
    // `import Algebra.Semigroup` supplies `<>`'s fixity exactly as it
    // supplies the name, and no consumer redeclares it.
    let user = parse_source(user_src, user_file, &prelude.base, &prelude.fixities)?;
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
    let held = if shared { file.lock_shared() } else { file.lock() };
    // A filesystem without advisory locking still builds and links, just
    // without the cross-process guarantee.
    held.ok()?;
    Some(file)
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
        .status()
        .map_err(|e| Error::plain(format!("failed to build the runtime crate: {e}")))?;
    if !status.success() {
        return Err(Error::plain("building compiler/runtime (cinchruntime) failed"));
    }
    if debug.exists() {
        Ok(debug)
    } else {
        Err(Error::plain("libcinchruntime.a not found after building the runtime"))
    }
}

fn link(ir_path: &Path, out: &Path) -> Result<()> {
    link_with(ir_path, out, &[])
}

/// [`link`] with extra objects (the C fixtures of the foreign binding
/// tests; foreign function interface specification section 7: the linker
/// checks the compatible symbols).
fn link_with(ir_path: &Path, out: &Path, objects: &[&str]) -> Result<()> {
    let runtime = ensure_runtime()?;
    // Shared, so a concurrent rebuild of the archive waits for this link
    // instead of truncating the file under clang.
    let _guard = runtime_lock(true);
    let mut cmd = std::process::Command::new(clang());
    cmd.arg(ir_path);
    for o in objects {
        cmd.arg(o);
    }
    cmd.arg(&runtime).arg("-o").arg(out);
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
        return Err(Error::plain(format!(
            "{} failed to link {}{}",
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

/// Unique scratch directory name for compiled test and run artifacts.
fn scratch_dir(tag: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("cinch-{tag}-{pid}-{n}"))
}

/// The flags a compile takes that are neither a mode nor a profile.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DriverOptions {
    /// `--rebuild`: rewrite the package-store entry for this compile's
    /// key even when the entry is intact and matches it, instead of
    /// leaving it alone. The compile path never fails over an entry's
    /// contents (it repairs anything that is not this compile's set, see
    /// `crate::store`), so this is the explicit way to force a rewrite,
    /// and the escape for an entry a consumer reported as corrupt.
    pub rebuild_store: bool,
    /// The words the CLI forwards to the program `Mode::Run` executes:
    /// every argument after the entry file (`cinch run f.cinch a b`, or
    /// everything after `--` verbatim). They land on the built program's
    /// own command line, so the program reads them with
    /// `Effect.Process`'s `process-args` (`word 0` of that capture is
    /// the program itself, not one of these). Empty for a library
    /// caller, which is every test: only the CLI has words to forward.
    pub program_args: Vec<String>,
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
    driver_with_profile_opts(
        mode,
        stage_mode_from_env(),
        path,
        out,
        &crate::core::TargetProfile::native(),
        opts,
    )
}

/// [`driver`] with an explicit compile-time I/O mode (modules
/// specification section 10).
pub fn driver_with(
    mode: Mode,
    stage_mode: StageMode,
    path: &Path,
    out: Option<&Path>,
) -> Result<i32> {
    driver_with_profile(mode, stage_mode, path, out, &crate::core::TargetProfile::native())
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
    let project = load_project(path)?;
    let adapters = wit_for_source(path)?;
    let prelude = parse_prelude(&adapters)?;
    let prelude_fixities = prelude.base.clone();
    let mut module_fixities: HashMap<String, FixityTable> = prelude.fixities.clone();
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
            let parsed = parse_source(src, &key, &prelude_fixities, &module_fixities)?;
            for (m, f) in &parsed.fixities {
                module_fixities.insert(m.clone(), f.clone());
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
    let mut hashes: HashMap<String, String> = HashMap::new();
    let mut files: HashMap<String, String> = HashMap::new();
    for name in &project.order {
        let key = project.module_file[name].display().to_string();
        hashes.insert(name.clone(), crate::interface::content_hash(&project.sources[&key]));
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
    let (program, ifaces, report, _diags) = check::check_program_multi_full(
        &prelude.modules,
        &modules,
        &hashes,
        &scx,
        &adapters,
        profile,
    )?;
    // Metadata boundary checks (design note, section 3).
    crate::interface::verify_chain(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    crate::interface::check_coherence(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    // Specialization (supercompilation and JIT specification sections
    // 2-5): every residual program is rechecked before installation, and
    // the report records where specialization stopped.
    let (program, opt) = crate::specialize::optimize(&program, profile)?;
    write_artifacts(&out_dir, &entry_stem, &ifaces, &manifest_path, &report, &program, &opt, profile, &adapters, opts)?;
    if mode == Mode::Check {
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
    let program = crate::rc::insert(program, profile)?;
    let ir = codegen::emit_for(profile, &program)?;
    match mode {
        Mode::Check => unreachable!(),
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
            link(&ir_path, &out)?;
            Ok(0)
        }
        Mode::Run => {
            let tmp = scratch_dir("run");
            std::fs::create_dir_all(&tmp)
                .map_err(|e| Error::plain(format!("{}: {e}", tmp.display())))?;
            let ir_path = tmp.join("program.ll");
            let exe = tmp.join("program");
            std::fs::write(&ir_path, &ir)
                .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
            link(&ir_path, &exe)?;
            // The program's own command line (the CLI's forwarded
            // words): the entry glue captures them, so the built
            // program reads them with `process-args`.
            let status = std::process::Command::new(&exe)
                .args(&opts.program_args)
                .status()
                .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
            Ok(status.code().unwrap_or(1))
        }
    }
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
pub fn run_program(program: &crate::core::Program, profile: &crate::core::TargetProfile) -> Result<i32> {
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
    let program = crate::rc::insert(program, profile)?;
    let ir = codegen::emit_for(profile, &program)?;
    let tmp = scratch_dir("test");
    std::fs::create_dir_all(&tmp).map_err(|e| Error::plain(format!("{}: {e}", tmp.display())))?;
    let ir_path = tmp.join("program.ll");
    let exe = tmp.join("program");
    std::fs::write(&ir_path, &ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    link_with(&ir_path, &exe, objects)?;
    let status = std::process::Command::new(&exe)
        .status()
        .map_err(|e| Error::plain(format!("{}: {e}", exe.display())))?;
    Ok(status.code().unwrap_or(1))
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
    let program = crate::rc::insert(program, &profile)?;
    let ir = codegen::emit_for(&profile, &program)?;
    let ir_path = out.with_extension("ll");
    std::fs::write(&ir_path, &ir).map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    link_with(&ir_path, out, objects)
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
    let program = crate::rc::insert(program, profile)?;
    let ir = codegen::emit_for(profile, &program)?;
    let tmp = scratch_dir("test");
    std::fs::create_dir_all(&tmp).map_err(|e| Error::plain(format!("{}: {e}", tmp.display())))?;
    let ir_path = tmp.join("program.ll");
    let exe = tmp.join("program");
    std::fs::write(&ir_path, &ir)
        .map_err(|e| Error::plain(format!("{}: {e}", ir_path.display())))?;
    link_with(&ir_path, &exe, &[])?;
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
    Ok((
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    ))
}
