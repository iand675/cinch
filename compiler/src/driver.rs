//! Command driver: source file in, diagnostics or native binary out.

use crate::parser::{parse_module, FixityTable, ParsedModule};
use crate::{check, codegen, layout, lexer, Error, Result};
use std::path::Path;

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

/// Lex, lay out and parse one module, with `imported` fixities visible.
pub fn parse_source(src: &str, file: &str, imported: &FixityTable) -> Result<ParsedModule> {
    let ctx = |e: Error| Error::plain(format!("{file}: {e}"));
    let tokens = lexer::lex(src).map_err(ctx)?;
    let tokens = layout::layout(tokens).map_err(ctx)?;
    parse_module(&tokens, imported).map_err(ctx)
}

/// Scan a source's import declarations without full parsing: imports do not
/// need fixity resolution, and the module graph must be known before the
/// merge of imported fixity tables (design note, section 3).
pub fn scan_imports(src: &str, file: &str) -> Result<Vec<String>> {
    let ctx = |e: Error| Error::plain(format!("{file}: {e}"));
    let tokens = lexer::lex(src).map_err(ctx)?;
    let tokens = layout::layout(tokens).map_err(ctx)?;
    let mut imports = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if matches!(tokens[i].tok, lexer::Tok::Import) {
            i += 1;
            let mut last = None;
            while let Some(t) = tokens.get(i) {
                match &t.tok {
                    lexer::Tok::ConId(name) => {
                        last = Some(name.clone());
                        i += 1;
                        if matches!(tokens.get(i).map(|t| &t.tok), Some(lexer::Tok::Dot)) {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }
            if let Some(name) = last {
                if !imports.contains(&name) {
                    imports.push(name);
                }
            }
        } else {
            i += 1;
        }
    }
    Ok(imports)
}

/// A resolved multi-file project: modules in dependency order (imports
/// first), entry last, with each module's source and imports.
struct Project {
    order: Vec<String>,
    sources: std::collections::BTreeMap<String, String>,
    imports: std::collections::BTreeMap<String, Vec<String>>,
    entry: String,
}

/// Load the entry file and every module it imports, transitively. Bootstrap
/// rule: `Foo.lang` declares `module Foo`, and imports resolve to
/// `<entry dir>/Foo.lang`.
fn load_project(entry_path: &Path) -> Result<Project> {
    let dir = entry_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default();
    let mut sources: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    let mut imports: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    let mut queue: Vec<std::path::PathBuf> = vec![entry_path.to_path_buf()];
    let mut entry_name = String::new();
    while let Some(path) = queue.pop() {
        let src = std::fs::read_to_string(&path)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        let imports_here = scan_imports(&src, &path.display().to_string())?;
        // The module name is the file stem for the bootstrap rule.
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        if entry_name.is_empty() {
            entry_name = name.clone();
        }
        if let Some(prev) = sources.insert(name.clone(), src) {
            if prev != sources[&name] {
                return Err(Error::plain(format!("module {name} loaded twice with different sources")));
            }
            continue;
        }
        imports.insert(name.clone(), imports_here.clone());
        for imp in imports_here {
            if imp == "Prelude" {
                continue;
            }
            let p = dir.join(format!("{imp}.lang"));
            if !p.exists() {
                return Err(Error::plain(format!(
                    "module {name} imports {imp}, but {} does not exist",
                    p.display()
                )));
            }
            queue.push(p);
        }
    }
    // Dependency order; the entry goes last because its dependencies precede
    // it and a cycle is an error.
    let graph: std::collections::BTreeMap<String, Vec<crate::interface::IfaceRef>> = imports
        .iter()
        .map(|(m, imps)| {
            (
                m.clone(),
                imps
                    .iter()
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
    order.retain(|m| sources.contains_key(m));
    if let Some(pos) = order.iter().position(|m| m == &entry_name) {
        let e = order.remove(pos);
        order.push(e);
    }
    Ok(Project { order, sources, imports, entry: entry_name })
}

/// Write the per-module interfaces and the artifact manifest (design note,
/// sections 2 and 5; core specification, section 11).
fn write_artifacts(
    out_dir: &Path,
    entry: &str,
    ifaces: &[crate::interface::Iface],
    manifest_path: &Path,
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
    text.push_str("  }\n}\n");
    std::fs::write(manifest_path, text)
        .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
    Ok(())
}

/// Check the prelude plus `user_src`, returning the elaborated program.
pub fn check_sources(user_src: &str, user_file: &str) -> Result<crate::core::Program> {
    let empty = FixityTable::new();
    let prelude = parse_source(PRELUDE_SOURCE, "Prelude.lang", &empty)?;
    let user = parse_source(user_src, user_file, &prelude.fixities)?;
    check::check_program(&prelude.module, &user.module)
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
    let runtime = ensure_runtime()?;
    let mut cmd = std::process::Command::new(clang());
    cmd.arg(ir_path).arg(&runtime).arg("-o").arg(out);
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
/// exit code in `Ok`.
pub fn driver(mode: Mode, path: &Path, out: Option<&Path>) -> Result<i32> {
    // Multi-file project: load imports, parse in dependency order so each
    // module sees its imports' fixities, then check the whole set and write
    // per-module interfaces (design note, compiled interface metadata).
    let project = load_project(path)?;
    let empty = FixityTable::new();
    let prelude = parse_source(PRELUDE_SOURCE, "Prelude.lang", &empty)?;
    let mut parsed_fixities: std::collections::HashMap<String, FixityTable> =
        std::collections::HashMap::new();
    let mut modules: Vec<crate::ast::Module> = Vec::new();
    for name in &project.order {
        let src = &project.sources[name];
        let mut imported = prelude.fixities.clone();
        for dep in &project.imports[name] {
            if dep == "Prelude" {
                continue;
            }
            if let Some(t) = parsed_fixities.get(dep) {
                imported = imported.merge(t);
            }
        }
        let parsed = parse_source(src, &format!("{name}.lang"), &imported)?;
        // Imported modules follow the file-stem rule so `import Lib` finds
        // `Lib.lang`; the entry file is free to declare any module name.
        if *name != project.entry && parsed.module.name.text != *name {
            return Err(Error::plain(format!(
                "{}.lang declares module {}, but the bootstrap rule requires the file stem",
                name, parsed.module.name.text
            )));
        }
        parsed_fixities.insert(name.clone(), parsed.fixities);
        modules.push(parsed.module);
    }
    if let Some(pos) = modules.iter().position(|m| m.name.text == project.entry) {
        let e = modules.remove(pos);
        modules.push(e);
    }
    let hashes: std::collections::HashMap<String, String> = project
        .sources
        .iter()
        .map(|(n, s)| (n.clone(), crate::interface::content_hash(s)))
        .collect();
    let (program, ifaces) = check::check_program_multi(&prelude.module, &modules, &hashes)?;
    // Metadata boundary checks (design note, section 3).
    crate::interface::verify_chain(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    crate::interface::check_coherence(&ifaces).map_err(|e| Error::plain(e.to_string()))?;
    let dir = path.parent().map(|p| p.to_path_buf()).unwrap_or_default();
    let out_dir = dir.join("build");
    let manifest = out_dir.join(format!("{}.manifest.json", project.entry));
    write_artifacts(&out_dir, &project.entry, &ifaces, &manifest)?;
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
    let program = check_sources(src, "<test>")?;
    let ir = codegen::emit(&program)?;
    let tmp = scratch_dir("test");
    std::fs::create_dir_all(&tmp).map_err(|e| Error::plain(format!("{}: {e}", tmp.display())))?;
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
