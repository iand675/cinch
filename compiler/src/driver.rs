//! Command driver: source file in, diagnostics or native binary out.

use crate::parser::{parse_modules, FixityTable, ParsedModules};
use crate::{check, codegen, layout, lexer, Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
pub const PRELUDE_FACADE: &str = include_str!("../prelude/Prelude.lang");

/// The base library's scheme homes (the prelude split), embedded so
/// every compilation sees them. The list is in dependency order
/// (imports first) for fixity threading and interface order; the
/// facade above checks last (it re-exports the family) and its source
/// parses first (its fixity table is the base table).
/// The base library's scheme homes whose surfaces the `Prelude` facade
/// re-exports (the prelude split), in collection order: each module's
/// eager references (signature types, signature constraint classes)
/// resolve against modules listed before it. The facade collects after
/// these (its `export` items read their surfaces) and before
/// [`PRELUDE_LATE`].
pub const PRELUDE_EARLY: &[(&str, &str)] = &[
    ("Relation/Eq.lang", include_str!("../prelude/Relation/Eq.lang")),
    ("Relation/Ord.lang", include_str!("../prelude/Relation/Ord.lang")),
    ("Algebra/Arithmetic.lang", include_str!("../prelude/Algebra/Arithmetic.lang")),
    ("Algebra/Bits.lang", include_str!("../prelude/Algebra/Bits.lang")),
    ("Function/Combinators.lang", include_str!("../prelude/Function/Combinators.lang")),
    ("Data/Nat.lang", include_str!("../prelude/Data/Nat.lang")),
    ("Data/Int.lang", include_str!("../prelude/Data/Int.lang")),
    ("Data/Option.lang", include_str!("../prelude/Data/Option.lang")),
    ("Data/Result.lang", include_str!("../prelude/Data/Result.lang")),
    ("Data/List.lang", include_str!("../prelude/Data/List.lang")),
    // The enumeration class behind the range literals collects after
    // `Data.List` (its `List` type); its instances live with their
    // types in `Data.Nat`/`Data.Int`.
    ("Data/Enum.lang", include_str!("../prelude/Data/Enum.lang")),
    ("Data/Ordering.lang", include_str!("../prelude/Data/Ordering.lang")),
    ("Data/Char.lang", include_str!("../prelude/Data/Char.lang")),
    ("Data/Bool.lang", include_str!("../prelude/Data/Bool.lang")),
    ("Data/Machine.lang", include_str!("../prelude/Data/Machine.lang")),
    ("Data/Bytes.lang", include_str!("../prelude/Data/Bytes.lang")),
    ("Data/Map.lang", include_str!("../prelude/Data/Map.lang")),
    ("Data/String.lang", include_str!("../prelude/Data/String.lang")),
    ("Data/Json.lang", include_str!("../prelude/Data/Json.lang")),
    ("Data/Producer.lang", include_str!("../prelude/Data/Producer.lang")),
    ("Data/Target.lang", include_str!("../prelude/Data/Target.lang")),
    ("Effect/Sequencing.lang", include_str!("../prelude/Effect/Sequencing.lang")),
    ("Effect/Concurrent.lang", include_str!("../prelude/Effect/Concurrent.lang")),
    ("Effect/Io.lang", include_str!("../prelude/Effect/Io.lang")),
    ("Data/FromList.lang", include_str!("../prelude/Data/FromList.lang")),
    ("Data/FromMap.lang", include_str!("../prelude/Data/FromMap.lang")),
    ("Text/Show.lang", include_str!("../prelude/Text/Show.lang")),
    // `Data.Sym` collects after `Text.Show` (its `Show` instance) and
    // `Data.String` (its name conversions); the facade collects after
    // the whole family.
    ("Data/Sym.lang", include_str!("../prelude/Data/Sym.lang")),
];

/// The rest of the base library family (the class modules and the
/// newer families), collected after the facade: their eager references
/// are old-surface names, which the facade's surface provides.
pub const PRELUDE_LATE: &[(&str, &str)] = &[
    ("Algebra/Semigroup.lang", include_str!("../prelude/Algebra/Semigroup.lang")),
    ("Algebra/Group.lang", include_str!("../prelude/Algebra/Group.lang")),
    ("Algebra/Semiring.lang", include_str!("../prelude/Algebra/Semiring.lang")),
    ("Algebra/Monoid.lang", include_str!("../prelude/Algebra/Monoid.lang")),
    ("Category/Functor.lang", include_str!("../prelude/Category/Functor.lang")),
    ("Category/Applicative.lang", include_str!("../prelude/Category/Applicative.lang")),
    ("Category/Monad.lang", include_str!("../prelude/Category/Monad.lang")),
    ("Category/Alternative.lang", include_str!("../prelude/Category/Alternative.lang")),
    ("Category/Foldable.lang", include_str!("../prelude/Category/Foldable.lang")),
    ("Category/Traversable.lang", include_str!("../prelude/Category/Traversable.lang")),
    ("Data/Identity.lang", include_str!("../prelude/Data/Identity.lang")),
    ("Data/Const.lang", include_str!("../prelude/Data/Const.lang")),
    ("Data/Compose.lang", include_str!("../prelude/Data/Compose.lang")),
    ("Data/NonEmpty.lang", include_str!("../prelude/Data/NonEmpty.lang")),
    ("Data/Sorted.lang", include_str!("../prelude/Data/Sorted.lang")),
    ("Data/NonZero.lang", include_str!("../prelude/Data/NonZero.lang")),
    ("Data/Ascii.lang", include_str!("../prelude/Data/Ascii.lang")),
    ("Data/Set.lang", include_str!("../prelude/Data/Set.lang")),
    ("Data/Hash.lang", include_str!("../prelude/Data/Hash.lang")),
    ("Text/Codec.lang", include_str!("../prelude/Text/Codec.lang")),
    ("Data/HashMap.lang", include_str!("../prelude/Data/HashMap.lang")),
];

/// Every embedded prelude module name (the family's scheme names and
/// the facade): imports of these resolve to the embedded sources,
/// never to a file beside the entry unless an explicit source module
/// shadows the copy.
pub fn prelude_module_names() -> Vec<String> {
    let mut names: Vec<String> = PRELUDE_EARLY
        .iter()
        .chain(PRELUDE_LATE.iter())
        .map(|(path, _)| path.trim_end_matches(".lang").replace('/', "."))
        .collect();
    names.push("Prelude".to_string());
    names
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
    let facade_parsed = parse_source(&facade_src, "Prelude.lang", &empty, &HashMap::new())?;
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
    for (path, src) in PRELUDE_EARLY {
        parse_into(src, path, &mut early)?;
    }
    for (path, src) in PRELUDE_LATE {
        parse_into(src, path, &mut late)?;
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
/// `X.lang` does not exist (decision D42).
fn index_dir_modules(dir: &Path) -> Result<HashMap<String, Vec<PathBuf>>> {
    let mut index: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(index),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("lang") {
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
/// loaded file, then to `X.lang`, then to a unique `module X` beside the
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
                let direct = dir.join(format!("{imp}.lang"));
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

fn json_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
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

/// Write the per-module interfaces, the artifact manifest, and the stage
/// snapshot (design note sections 2 and 5; modules specification section
/// 10: the build identity includes generator versions and input hashes;
/// supercompilation and JIT specification sections 5, 8 and 9: the
/// specialization configuration and trust level; platform proofs and SIMD
/// specification sections 1 and 8: the target conditions and which
/// lowering was checked).
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
) -> Result<()> {
    std::fs::create_dir_all(out_dir)
        .map_err(|e| Error::plain(format!("{}: {e}", out_dir.display())))?;
    for iface in ifaces {
        let p = out_dir.join(format!("{}.sliface", iface.module));
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
    let mut text = String::new();
    text.push_str("{\n  \"entry\": ");
    text.push_str(&format!("{:?},\n", entry));
    text.push_str("  \"runtime_abi\": 1,\n  \"modules\": [\n");
    for (i, (m, h)) in modules.iter().enumerate() {
        text.push_str(&format!(
            "    {{\"module\": {:?}, \"iface_hash\": {:?}}}",
            m, h
        ));
        text.push_str(if i + 1 < modules.len() { ",\n" } else { "\n" });
    }
    text.push_str("  ],\n  \"assumptions\": {\n");
    text.push_str(&format!("    \"primitives\": {:?},\n", prims));
    text.push_str(&format!("    \"foreign\": {:?},\n", foreign));
    text.push_str(&format!("    \"declared\": {:?}\n", declared));
    text.push_str("  },\n");
    // `prim` declarations (primitive operations and inline target IR
    // specification section 8): per prim the selected body's family and
    // condition, its tier, its `uses` list and its validation result.
    // Selection is manifest-only: it never enters the contract
    // projection (section 6).
    text.push_str("  \"prims\": [\n");
    for (i, p) in program.prims.iter().enumerate() {
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
        text.push_str(&format!(
            "    {{\"name\": \"{}\", \"module\": \"{}\", \"tier\": \"{}\", \"family\": \"{}\", \"condition\": {}, \"why\": \"{}\", \"uses\": {:?}, \"immarg\": {:?}, \"validation\": {:?}, \"trust\": {:?}, \"identity\": \"{}\"}}",
            p.name,
            p.module,
            match p.tier {
                crate::core::PrimTier::Justified => "justified",
                crate::core::PrimTier::Assumed => "assumed",
            },
            body.family,
            match &body.when {
                Some(c) => format!("\"{c}\""),
                None => "null".to_string(),
            },
            p.selected_why,
            uses,
            body.immarg,
            validations,
            trust,
            p.identity
        ));
        text.push_str(if i + 1 < program.prims.len() { ",\n" } else { "\n" });
    }
    text.push_str("  ],\n");
    // The class law check rows (section 5): what the sample-wise check
    // decided, honestly (`checked` over its samples or `not checked` at
    // the law machinery's boundary).
    text.push_str("  \"laws\": [\n");
    for (i, l) in program.laws.iter().enumerate() {
        text.push_str(&format!(
            "    {{\"instance\": \"{}\", \"class\": \"{}\", \"law\": \"{}\", \"status\": \"{}\"}}",
            l.instance, l.class, l.law, l.status
        ));
        text.push_str(if i + 1 < program.laws.len() { ",\n" } else { "\n" });
    }
    text.push_str("  ],\n");
    // The target profile and the conditions the target rules discharged
    // (platform proofs and SIMD specification sections 1-3; modules and
    // compilation specification section 13: target conditions are
    // preserved in the manifest).
    text.push_str("  \"target\": {\n");
    text.push_str(&format!("    \"profile\": \"{}\",\n", profile.name));
    text.push_str(&format!("    \"word_size\": {},\n", profile.word_size));
    text.push_str(&format!("    \"endian\": \"{}\",\n", profile.endian));
    text.push_str(&format!(
        "    \"vector_shape\": \"{}\",\n",
        match profile.vector_shape {
            crate::core::VectorShape::Fixed(w) => format!("fixed({w})"),
            crate::core::VectorShape::Scalable => "scalable".to_string(),
        }
    ));
    text.push_str(&format!("    \"features\": {:?},\n", profile.features));
    text.push_str(&format!(
        "    \"admission\": \"{}\",\n",
        "the deployment contract must satisfy the recorded feature assumptions"
    ));
    text.push_str("    \"conditions\": [\n");
    let conds: Vec<String> = program
        .rewrites
        .iter()
        .filter(|r| r.target)
        .map(|r| {
            let fired = opt
                .sites
                .iter()
                .any(|s| s.kind == format!("rewrite:{}", r.name) && s.stop.is_none());
            format!(
                "      {{\"rule\": \"{}\", \"condition\": \"{}\", \"discharged\": {}}}",
                r.name,
                match &r.cond {
                    crate::core::RequireCond::CapabilityInScope { class, .. } => {
                        format!("capability {class} valid in the firing region's validity scope")
                    }
                    crate::core::RequireCond::Always => "always".to_string(),
                    crate::core::RequireCond::Never => "never".to_string(),
                    crate::core::RequireCond::Unknown => {
                        "decided at the match site".to_string()
                    }
                },
                fired
            )
        })
        .collect();
    text.push_str(&conds.join(",\n"));
    if !conds.is_empty() {
        text.push('\n');
    }
    text.push_str("    ]\n  },\n");
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
        if r.target
            && opt
                .sites
                .iter()
                .any(|s| s.kind == format!("rewrite:{}", r.name) && s.stop.is_none())
        {
            lowerings.push(format!("rewrite:{}", r.name));
        }
    }
    text.push_str("  \"wasm\": {\n");
    text.push_str(&format!(
        "    \"artifact\": \"{}\",\n",
        profile.artifact.spelling()
    ));
    let canon = match wit.first() {
        Some(a) => a.abi.spelling(),
        None => "none",
    };
    text.push_str(&format!("    \"canonical_abi\": \"{canon}\",\n"));
    text.push_str(&format!("    \"required\": {:?},\n", required));
    text.push_str(&format!("    \"available\": {:?},\n", profile.features));
    text.push_str(&format!(
        "    \"selected\": {{\"profile\": {:?}, \"features\": {:?}}},\n",
        profile.name, profile.features
    ));
    text.push_str(&format!("    \"lowerings\": {:?},\n", lowerings));
    text.push_str("    \"adapters\": [\n");
    let mut adapter_rows: Vec<String> = Vec::new();
    for a in wit {
        for b in &a.bindings {
            let mut row = format!(
                "{{\"package\": {:?}, \"module\": {:?}, \"name\": {:?}, \"symbol\": {:?}, \"export\": {}, \"core_params\": {:?}, \"core_results\": {:?}",
                a.package,
                a.module,
                b.name,
                b.symbol,
                b.export,
                b.call.core_params.iter().map(|t| t.spelling()).collect::<Vec<_>>(),
                b.call.core_results.iter().map(|t| t.spelling()).collect::<Vec<_>>()
            );
            if let Some(ret) = &b.call.ret {
                row.push_str(&format!(
                    ", \"ret_area\": {{\"size\": {}, \"align\": {}, \"cases\": {}}}",
                    ret.size,
                    ret.align,
                    ret.cases.len()
                ));
            }
            if let Some(g) = &b.call.glue_gap {
                row.push_str(&format!(", \"glue_gap\": {g:?}"));
            }
            row.push('}');
            adapter_rows.push(row);
        }
    }
    text.push_str(&adapter_rows.join(",\n"));
    if !adapter_rows.is_empty() {
        text.push('\n');
    }
    text.push_str("    ]\n  },\n");
    // Specialization configuration and report (supercompilation and JIT
    // specification sections 5 and 8).
    text.push_str("  \"specialization\": {\n");
    text.push_str("    \"ran\": true,\n");
    text.push_str(&format!("    \"fuel\": {},\n", profile.fuel));
    text.push_str(&format!("    \"measure\": {},\n", profile.measure));
    text.push_str("    \"whistle\": \"repetition-or-measure\",\n");
    text.push_str(&format!("    \"runtime_codegen\": {},\n", false));
    text.push_str("    \"sites\": [\n");
    for (i, s) in opt.sites.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"unit\": {:?}, \"kind\": {:?}, \"stop\": {}}}",
            s.fn_id,
            s.kind,
            match &s.stop {
                Some(stop) => format!("{stop:?}"),
                None => "null".to_string(),
            }
        ));
        text.push_str(if i + 1 < opt.sites.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ],\n");
    text.push_str(&format!("    \"fused\": {:?},\n", opt.fused));
    text.push_str(&format!("    \"discarded\": {:?}\n", opt.discarded));
    text.push_str("  },\n");
    // Tail-call status of the recursive call-graph groups (surface
    // grammar local amendments section 9; decision D104): "marked" when
    // the group's self and mutual tail calls are jumps or `musttail`
    // calls (constant stack), "fallback (<reason>)" when one is an
    // ordinary call (the reason names the call).
    let tail_groups = crate::core::tail_report(program);
    text.push_str("  \"tail_calls\": {\n    \"groups\": [\n");
    for (i, g) in tail_groups.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"members\": {:?}, \"mark\": {:?}}}",
            g.members, g.mark
        ));
        text.push_str(if i + 1 < tail_groups.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ]\n  },\n");
    // The collection literal fold boundary (surface grammar local
    // amendments section 10; decision D110): each literal's site, its
    // construction's kind, and whether the compile-time fold made it a
    // constant value or it keeps its runtime `from-list`/`from-map` call.
    text.push_str("  \"literal_folds\": {\n    \"sites\": [\n");
    for (i, f) in report.literal_folds.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"unit\": {:?}, \"site\": {:?}, \"kind\": {:?}, \"folded\": {}}}",
            f.unit, f.site, f.kind, f.folded
        ));
        text.push_str(if i + 1 < report.literal_folds.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ]\n  },\n");
    // The `import data` bindings' provenance (surface grammar local
    // amendments section 11): the converter identity and the file's
    // content hash per binding. The import's contract covers the
    // conversion, not the file bytes; the manifest records hashes, never
    // contents.
    text.push_str("  \"data_imports\": {\n    \"bindings\": [\n");
    for (i, d) in report.data_imports.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"module\": {:?}, \"name\": {:?}, \"file\": {:?}, \"converter\": {:?}, \"hash\": {:?}}}",
            d.module, d.name, d.file, d.converter, d.hash
        ));
        text.push_str(if i + 1 < report.data_imports.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ]\n  },\n");
    // P20's safety assertion summary (attribute annotations
    // specification section 10): per definition, the assertion, its
    // verification result, and for `bounded(x)` which proof method
    // established the bound (decision D113); plus the `jpl` profile's
    // whole-program summary (decision D114). Definition names, never
    // paths.
    text.push_str("  \"safety\": {\n    \"assertions\": [\n");
    for (i, a) in report.safety.assertions.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"definition\": {:?}, \"attribute\": {:?}, \"result\": {:?}}}",
            a.definition, a.attribute, a.result
        ));
        text.push_str(
            if i + 1 < report.safety.assertions.len() { ",\n" } else { "\n" },
        );
    }
    text.push_str("    ],\n");
    match &report.safety.jpl {
        Some(j) => text.push_str(&format!(
            "    \"jpl\": {{\"modules\": {:?}, \"definitions\": {}, \"assertions\": {}}}\n",
            j.modules, j.definitions, j.assertions
        )),
        None => text.push_str("    \"jpl\": null\n"),
    }
    text.push_str("  },\n");
    // The verified temporal contracts (core specification section 10):
    // per contract, the definition it names, the kind and formula, the
    // verification result (which argument discharged it), and the stated
    // assumptions (a liveness claim relies on the runtime policies it
    // names). Definition names, never paths.
    text.push_str("  \"temporal\": {\n    \"contracts\": [\n");
    for (i, t) in report.temporal.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"definition\": {:?}, \"contract\": {:?}, \"kind\": {:?}, \"formula\": {:?}, \"result\": {:?}, \"assumptions\": {:?}}}",
            t.definition, t.contract, t.kind, t.formula, t.result, t.assumptions
        ));
        text.push_str(if i + 1 < report.temporal.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ]\n  },\n");
    // Trust level (supercompilation and JIT specification section 9:
    // full recheck is the default) and the checked-versus-trusted
    // lowering split (platform proofs and SIMD specification section 8).
    text.push_str("  \"trust\": {\n");
    text.push_str("    \"level\": 2,\n");
    text.push_str("    \"name\": \"full-recheck\",\n");
    text.push_str(&format!("    \"rechecked\": {:?},\n", opt.rechecked));
    text.push_str(&format!(
        "    \"trusted\": {:?}\n",
        vec![
            "type preservation by construction",
            "effect-order preservation by construction",
        ]
    ));
    text.push_str("  },\n");
    text.push_str("  \"lowering\": {\n");
    text.push_str(&format!(
        "    \"checked\": {:?},\n",
        vec![
            "core structure and scoping",
            "usage of %1/%aff parameters",
            "memo and demand sharing",
            "decision-point weights",
            "contract identity",
        ]
    ));
    text.push_str(&format!(
        "    \"trusted\": {:?}\n",
        vec![
            "LLVM code generation",
            "the runtime and its instruction model",
        ]
    ));
    text.push_str("  },\n");
    // Staging build identity (modules specification section 10): the mode,
    // the compiler version, generated declarations, and every captured
    // input with its hash. Secrets record their name only.
    text.push_str("  \"staging\": {\n");
    text.push_str(&format!("    \"mode\": \"{}\",\n", match report.mode {
        StageMode::Refresh => "refresh",
        StageMode::Replay => "replay",
        StageMode::Live => "live",
    }));
    text.push_str(&format!("    \"compiler\": \"{}\",\n", env!("CARGO_PKG_VERSION")));
    text.push_str("    \"generated\": {");
    let mut gen: Vec<(&String, &Vec<String>)> =
        report.generated.iter().map(|(m, names)| (m, names)).collect();
    gen.sort();
    for (i, (m, names)) in gen.iter().enumerate() {
        let list: Vec<String> = names.iter().map(|n| format!("{:?}", n)).collect();
        text.push_str(&format!(
            "{}{:?}: [{}]",
            if i > 0 { ", " } else { "" },
            m,
            list.join(", ")
        ));
    }
    text.push_str("},\n    \"inputs\": [\n");
    for (i, input) in report.inputs.iter().enumerate() {
        text.push_str(&format!(
            "      {{\"kind\": \"{}\", \"snapshot\": \"{}\", \"name\": \"{}\", \"hash\": \"{}\", \"live\": {}}}",
            input.kind,
            json_escape(&input.snapshot),
            json_escape(&input.name),
            input.hash,
            input.live
        ));
        text.push_str(if i + 1 < report.inputs.len() { ",\n" } else { "\n" });
    }
    text.push_str("    ]\n  }\n}\n");
    std::fs::write(manifest_path, text)
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
        let mut out = String::new();
        out.push_str("{\"selected\": ");
        crate::interface::json_str(&mut out, selected);
        out.push_str(", \"available\": ");
        crate::interface::json_list(&mut out, available);
        out.push_str(", \"variants\": [");
        for (i, v) in self.variants.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str("{\"name\": ");
            crate::interface::json_str(&mut out, &v.name);
            out.push_str(", \"required\": ");
            crate::interface::json_list(&mut out, &v.required);
            out.push_str(", \"lowerings\": ");
            crate::interface::json_list(&mut out, &v.lowerings);
            out.push_str(", \"ir_hash\": ");
            crate::interface::json_str(&mut out, &v.ir_hash);
            out.push('}');
        }
        out.push_str("]}");
        out
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
        let (program, _opt) = crate::core::optimize(&program, profile)?;
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
    let user = parse_source(user_src, user_file, &prelude.base, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), user_file.to_string());
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
/// `.sliface` compatibility evidence and contract projection that the
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
    std::env::var("SAFELANG_CLANG").unwrap_or_else(|_| "clang".to_string())
}

/// Build the Rust runtime staticlib if needed and return its path. The
/// runtime is Rust (design decision D5); generated code links against it.
fn ensure_runtime() -> Result<std::path::PathBuf> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let runtime = manifest_dir.join("runtime");
    let debug = runtime.join("target/debug/libslruntime.a");
    let release = runtime.join("target/release/libslruntime.a");
    // Rebuild when any runtime source is newer than the staticlib (a
    // stale library silently drops the newer `sl_*` entry points).
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
        return Err(Error::plain("building compiler/runtime (slruntime) failed"));
    }
    if debug.exists() {
        Ok(debug)
    } else {
        Err(Error::plain("libslruntime.a not found after building the runtime"))
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
    let status = cmd
        .status()
        .map_err(|e| Error::plain(format!("failed to run {}: {e}", clang())))?;
    if !status.success() {
        return Err(Error::plain(format!(
            "{} failed to link {}",
            clang(),
            ir_path.display()
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
    std::env::temp_dir().join(format!("safelang-{tag}-{pid}-{n}"))
}

/// Compile `path` according to `mode`. `out` names the output file for
/// `EmitIr` and `Build`; `Run` executes the produced binary and returns its
/// exit code in `Ok`. The compile-time I/O mode comes from
/// `SAFELANG_STAGE_MODE` (`refresh`, `replay`, or `live`; default
/// `refresh`).
pub fn driver(mode: Mode, path: &Path, out: Option<&Path>) -> Result<i32> {
    let stage_mode = match std::env::var("SAFELANG_STAGE_MODE").as_deref() {
        Ok("replay") => StageMode::Replay,
        Ok("live") => StageMode::Live,
        _ => StageMode::Refresh,
    };
    driver_with(mode, stage_mode, path, out)
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
#[allow(clippy::too_many_arguments)]
pub fn driver_with_profile(
    mode: Mode,
    stage_mode: StageMode,
    path: &Path,
    out: Option<&Path>,
    profile: &crate::core::TargetProfile,
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
        schema_tool: std::env::var("SAFELANG_SCHEMA_TOOL").ok(),
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
    let (program, opt) = crate::core::optimize(&program, profile)?;
    write_artifacts(&out_dir, &entry_stem, &ifaces, &manifest_path, &report, &program, &opt, profile, &adapters)?;
    if mode == Mode::Check {
        return Ok(0);
    }
    // Artifact admission (platform proofs and SIMD specification
    // section 3): `check` is not admission. Producing or running the
    // artifact enforces the assumed deployment baseline and the world's
    // implementation obligations.
    admit_artifact(profile, &program, &adapters)?;
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
            let status = std::process::Command::new(&exe)
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
    let (program, _opt) = crate::core::optimize(program, profile)?;
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
    let (program, _opt) = crate::core::optimize(program, profile)?;
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
