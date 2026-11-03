//! Invoke the selected Cinch executable; no Rust frontend is used on this path.
use cinch::driver::{Mode, Package, BASE_PACKAGE};
use cinch::lsp::{parse_json, Json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Frontend {
    pub compiler: PathBuf,
    pub interfaces: PathBuf,
}

pub fn configured() -> Result<Option<Frontend>, String> {
    match (std::env::var_os("CINCH_TEST_COMPILER"), std::env::var_os("CINCH_TEST_INTERFACES")) {
        (None, None) => Ok(None),
        (Some(compiler), Some(interfaces)) if !compiler.is_empty() && !interfaces.is_empty() =>
            Ok(Some(Frontend { compiler: PathBuf::from(compiler), interfaces: PathBuf::from(interfaces) })),
        _ => Err("external Cinch tests require both CINCH_TEST_COMPILER and CINCH_TEST_INTERFACES; refusing a Native fallback".into()),
    }
}

fn validate_interfaces(path: &Path) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("read external interface manifest: {e}"))?;
    let doc = parse_json(&text).map_err(|e| format!("external interface manifest: {e}"))?;
    if doc.get("schema").and_then(Json::as_num) != Some(1.0) {
        return Err("external interface manifest must use schema 1".into());
    }
    let allowed = Package::parse(BASE_PACKAGE).map_err(|e| e.to_string())?.modules.into_iter().map(|m| m.name).collect::<BTreeSet<_>>();
    let rows = doc.get("interfaces").and_then(Json::as_arr).ok_or("external manifest needs interfaces")?;
    let mut seen = BTreeSet::new();
    for row in rows {
        let fields = row.as_arr().ok_or("invalid external interface row")?;
        if fields.len() != 3 || fields.iter().any(|x| x.as_str().is_none_or(str::is_empty)) {
            return Err("external interface row must contain module, hash and path strings".into());
        }
        let name = fields[0].as_str().unwrap();
        if !allowed.contains(name) || !seen.insert(name.to_owned()) {
            return Err(format!("external tests require distinct checked base interfaces, not {name}"));
        }
    }
    Ok(())
}

fn sources(root: &Path, dir: &Path, found: &mut Vec<PathBuf>) -> Result<(), String> {
    for item in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let path = item.map_err(|e| e.to_string())?.path();
        let name = path.file_name().and_then(|n| n.to_str()).ok_or("non-UTF-8 source path")?;
        if name.starts_with('.') || name == "build" || name == "target" { continue }
        let resolved = path.canonicalize().map_err(|e| e.to_string())?;
        if !resolved.starts_with(root) { return Err(format!("source path leaves the test project: {}", path.display())) }
        // Project source files are regular owned files; reject links to avoid
        // cycles and moving dependencies in a recorded test invocation.
        if std::fs::symlink_metadata(&path).map_err(|e| e.to_string())?.file_type().is_symlink() {
            return Err(format!("linked source path in external test project: {}", path.display()));
        }
        if path.is_dir() { sources(root, &path, found)? }
        else if path.extension().is_some_and(|e| e == "cinch") { found.push(path) }
    }
    Ok(())
}

pub fn drive(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    let compiler = frontend.compiler.canonicalize().map_err(|e| format!("selected compiler (no Native fallback): {e}"))?;
    let interfaces = frontend.interfaces.canonicalize().map_err(|e| format!("selected interfaces: {e}"))?;
    validate_interfaces(&interfaces)?;
    if matches!(mode, Mode::EmitIr | Mode::EmitObject) { return Err(format!("external frontend {mode:?} mode is not implemented; refusing a Native fallback")) }
    let dir = dir.canonicalize().map_err(|e| e.to_string())?;
    let entry = dir.join(entry).canonicalize().map_err(|e| format!("external test entry: {e}"))?;
    if !entry.starts_with(&dir) { return Err("external test entry leaves its project".into()) }
    let mut files = Vec::new(); sources(&dir, &dir, &mut files)?; files.sort();
    if !files.contains(&entry) { return Err("external test entry is not a project Cinch source".into()) }
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let work = dir.join(".cinch-self-tests").join(format!("{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::SeqCst)));
    std::fs::create_dir_all(&work).map_err(|e| e.to_string())?;
    let mut bundle = String::new();
    let mut inputs = Vec::new();
    for file in files {
        let relative = file.strip_prefix(&dir).unwrap().to_string_lossy().into_owned();
        let text = std::fs::read_to_string(&file).map_err(|e| e.to_string())?;
        // Escape labels so a newline in a filename cannot become source text.
        bundle.push_str(&format!("-- source: {}\n{}\n\n", relative.escape_default(), text));
        inputs.push(Json::obj(vec![("path", Json::Str(relative)), ("hash", Json::Str(cinch::interface::content_hash(&text)))]));
    }
    let source = work.join("source.cinch");
    std::fs::write(&source, bundle).map_err(|e| e.to_string())?;
    let ir = work.join("output.ll");
    let exe = out.map(|p| if p.is_absolute() { p } else { std::env::current_dir().unwrap_or_else(|_| dir.clone()).join(p) }).unwrap_or_else(|| work.join("program"));
    let report = work.join("link-report.txt");
    let mut args = vec![if mode == Mode::Check { "--check".to_string() } else { "--compile-only".to_string() }, "--interfaces".into(), interfaces.display().to_string(), source.display().to_string(), ir.display().to_string()];
    if mode != Mode::Check { args.extend([exe.display().to_string(), report.display().to_string()]); }
    let receipt = Json::obj(vec![("compiler", Json::Str(compiler.display().to_string())), ("interfaces", Json::Str(interfaces.display().to_string())), ("entry", Json::Str(entry.display().to_string())), ("mode", Json::Str(format!("{mode:?}"))), ("arguments", Json::Arr(args.iter().cloned().map(Json::Str).collect())), ("inputs", Json::Arr(inputs))]);
    std::fs::write(work.join("invocation.json"), receipt.to_text()).map_err(|e| e.to_string())?;
    let output = std::process::Command::new(&compiler).args(&args).current_dir(&dir).output().map_err(|e| format!("invoke selected Cinch compiler (no Native fallback): {e}"))?;
    std::fs::write(work.join("compiler.stdout"), &output.stdout).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler.stderr"), &output.stderr).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler-status.txt"), format!("{:?}\n", output.status.code())).map_err(|e| e.to_string())?;
    if !output.status.success() {
        let diagnostic = std::fs::read_to_string(ir.with_extension("ll.diagnostics")).unwrap_or_else(|_| String::from_utf8_lossy(&output.stderr).into_owned());
        return Err(format!("selected Cinch compiler exited {:?}: {diagnostic}\ninvocation: {}", output.status.code(), work.display()));
    }
    match mode {
        Mode::Check => Ok(0),
        Mode::Build => if exe.is_file() { Ok(0) } else { Err("selected compiler succeeded without an executable".into()) },
        Mode::Run => std::process::Command::new(&exe).current_dir(&dir).status().map_err(|e| e.to_string())?.code().ok_or_else(|| "external test artifact terminated without an exit code".into()),
        Mode::EmitIr | Mode::EmitObject => unreachable!(),
    }
}
