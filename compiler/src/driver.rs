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

/// The prelude is compiled alongside every user module and imported
/// implicitly.
pub const PRELUDE_SOURCE: &str = include_str!("../prelude/Prelude.lang");

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
        for (_, imps) in &scanned.modules {
            for (imp, _stage) in imps {
                if imp == "Prelude" || module_file.contains_key(imp) {
                    continue;
                }
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

/// Check the prelude plus `user_src`, returning the elaborated program.
pub fn check_sources(user_src: &str, user_file: &str) -> Result<crate::core::Program> {
    check_sources_with(user_src, user_file, &mut StagingCx::default())
}

/// [`check_sources`] with an explicit staging context (mode, work
/// directory, captured inputs) for compile-time I/O.
pub fn check_sources_with(
    user_src: &str,
    user_file: &str,
    scx: &mut StagingCx,
) -> Result<crate::core::Program> {
    let empty = FixityTable::new();
    let prelude = parse_source(PRELUDE_SOURCE, "Prelude.lang", &empty, &HashMap::new())?;
    let prelude_fixities = prelude
        .fixities
        .get("Prelude")
        .cloned()
        .unwrap_or_default();
    let user = parse_source(user_src, user_file, &prelude_fixities, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), user_file.to_string());
    }
    scx.fixities = user.fixities.clone();
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (program, _ifaces, _report) =
        check::check_program_multi(&prelude.modules[0], &user.modules, &hashes, scx)?;
    Ok(program)
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
    if release.exists() {
        return Ok(release);
    }
    if debug.exists() {
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
    let empty = FixityTable::new();
    let prelude = parse_source(PRELUDE_SOURCE, "Prelude.lang", &empty, &HashMap::new())?;
    let prelude_fixities = prelude
        .fixities
        .get("Prelude")
        .cloned()
        .unwrap_or_default();
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
    let (program, ifaces, report) =
        check::check_program_multi(&prelude.modules[0], &modules, &hashes, &scx)?;
    // Metadata boundary checks (design note, section 3).
    crate::interface::verify_chain(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    crate::interface::check_coherence(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    // Specialization (supercompilation and JIT specification sections
    // 2-5): every residual program is rechecked before installation, and
    // the report records where specialization stopped.
    let (program, opt) = crate::core::optimize(&program, profile)?;
    write_artifacts(&out_dir, &entry_stem, &ifaces, &manifest_path, &report, &program, &opt, profile)?;
    if mode == Mode::Check {
        return Ok(0);
    }
    let ir = codegen::emit(&program)?;
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
    let ir = codegen::emit(&program)?;
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
