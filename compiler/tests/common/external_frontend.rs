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

// Inventory headers only to choose source inputs. Program parsing, checking,
// diagnostics and code generation remain the selected compiler's work.
fn reachable_sources(files: &[PathBuf], entry: &Path) -> Vec<PathBuf> {
    let headers = files.iter().map(|path| {
        std::fs::read_to_string(path).ok().and_then(|text| cinch::driver::scan_file(&text, &path.display().to_string()).ok())
    }).collect::<Vec<_>>();
    let mut selected = BTreeSet::from([entry.to_path_buf()]);
    loop {
        let mut additional = BTreeSet::new();
        for (index, path) in files.iter().enumerate().filter(|(_, path)| selected.contains(*path)) {
            let Some(header) = &headers[index] else { continue };
            for (_, imports) in &header.modules {
                for (wanted, _) in imports {
                    // Include every actual provider of a requested module;
                    // duplicate-provider diagnostics belong to the compiler.
                    for (candidate, metadata) in files.iter().zip(&headers) {
                        if metadata.as_ref().is_some_and(|header| header.modules.iter().any(|(name, _)| name == wanted)) {
                            additional.insert(candidate.clone());
                        }
                    }
                }
            }
            let _ = path;
        }
        let before = selected.len(); selected.extend(additional);
        if selected.len() == before { return selected.into_iter().collect(); }
    }
}

fn next_invocation_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    format!("{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::SeqCst))
}

fn record_invocation(id: &str, invocation: Json, exit_code: Option<i32>) -> Result<(), String> {
    let Some(directory) = std::env::var_os("CINCH_TEST_INVOCATIONS").filter(|value| !value.is_empty()) else {
        return Ok(());
    };
    let directory = PathBuf::from(directory);
    let record = Json::obj(vec![
        ("invocation", invocation),
        ("test", std::thread::current().name().map(|name| Json::Str(name.into())).unwrap_or(Json::Null)),
        ("compiler_exit_code", exit_code.map(|code| Json::Num(f64::from(code))).unwrap_or(Json::Null)),
    ]);
    std::fs::create_dir_all(&directory).map_err(|error| format!("create invocation receipt directory: {error}"))?;
    std::fs::write(directory.join(format!("{id}.json")), record.to_text())
        .map_err(|error| format!("write selected compiler receipt: {error}"))
}

pub fn run_cli(frontend: &Frontend, entry: &Path, words: &[&str]) -> Result<i32, String> {
    let compiler = frontend.compiler.canonicalize().map_err(|e| format!("selected CLI: {e}"))?;
    let interfaces = frontend.interfaces.canonicalize().map_err(|e| format!("selected interfaces: {e}"))?;
    validate_interfaces(&interfaces)?;
    let configured_path = |name| -> Result<PathBuf, String> {
        let path = std::env::var_os(name).filter(|path| !path.is_empty()).ok_or_else(|| format!("selected CLI requires {name}"))?;
        PathBuf::from(path).canonicalize().map_err(|e| format!("{name}: {e}"))
    };
    let objects = configured_path("CINCH_TEST_OBJECTS")?;
    let runtime = configured_path("CINCH_TEST_RUNTIME_ARCHIVE")?;
    let entry = entry.canonicalize().map_err(|e| format!("selected CLI entry: {e}"))?;
    let prefix = entry.with_file_name("cli-run");
    let mut args = vec!["--interfaces".into(), interfaces.display().to_string(),
        "--objects".into(), objects.display().to_string(), "run".into(), runtime.display().to_string(),
        prefix.display().to_string(), entry.display().to_string()];
    args.extend(words.iter().map(|word| (*word).to_owned()));
    let source = std::fs::read_to_string(&entry).map_err(|e| format!("read selected CLI entry: {e}"))?;
    let file = entry.file_name().ok_or("selected CLI entry needs a filename")?;
    let input = Json::obj(vec![("path", Json::Str(file.to_string_lossy().into_owned())),
        ("hash", Json::Str(cinch::interface::content_hash(&source)))]);
    let receipt = Json::obj(vec![
        ("compiler", Json::Str(compiler.display().to_string())),
        ("interfaces", Json::Str(interfaces.display().to_string())),
        ("entry", Json::Str(entry.display().to_string())),
        ("mode", Json::Str("RunCli".into())),
        ("arguments", Json::Arr(args.iter().cloned().map(Json::Str).collect())),
        ("inputs", Json::Arr(vec![input])),
    ]);
    let status = std::process::Command::new(&compiler).args(&args).status()
        .map_err(|e| format!("invoke selected Cinch CLI: {e}"))?;
    record_invocation(&next_invocation_id(), receipt, status.code())?;
    status.code().ok_or_else(|| "selected Cinch CLI terminated without an exit code".into())
}

pub fn drive(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    let objects = if mode == Mode::EmitObject {
        std::env::var_os("CINCH_TEST_OBJECTS").filter(|p| !p.is_empty()).map(PathBuf::from)
    } else {
        None
    };
    drive_with_objects(frontend, mode, dir, entry, out, objects)
}

pub fn drive_with_objects(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, objects: Option<PathBuf>) -> Result<i32, String> {
    let compiler = frontend.compiler.canonicalize().map_err(|e| format!("selected compiler (no Native fallback): {e}"))?;
    let interfaces = frontend.interfaces.canonicalize().map_err(|e| format!("selected interfaces: {e}"))?;
    validate_interfaces(&interfaces)?;
    let dir = dir.canonicalize().map_err(|e| e.to_string())?;
    let entry = dir.join(entry).canonicalize().map_err(|e| format!("external test entry: {e}"))?;
    if !entry.starts_with(&dir) { return Err("external test entry leaves its project".into()) }
    let mut files = Vec::new(); sources(&dir, &dir, &mut files)?; files.sort();
    if !files.contains(&entry) { return Err("external test entry is not a project Cinch source".into()) }
    let files = reachable_sources(&files, &entry);
    let invocation_id = next_invocation_id();
    let work = dir.join(".cinch-self-tests").join(&invocation_id);
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
    let mut ir = work.join("output.ll");
    let requested_output = out.is_some();
    let exe = out.map(|p| if p.is_absolute() { p } else { std::env::current_dir().unwrap_or_else(|_| dir.clone()).join(p) }).unwrap_or_else(|| if mode == Mode::EmitObject { entry.with_extension("o") } else { work.join("program") });
    let report = work.join("link-report.txt");
    let flag = match mode { Mode::Check => "--check", Mode::EmitIr => "--emit-ir", Mode::EmitObject => "--emit-object", Mode::Build | Mode::Run => "--compile-only" };
    let mut args = vec![flag.into(), "--interfaces".into(), interfaces.display().to_string()];
    if mode == Mode::EmitObject {
        if objects.is_none() { return Err("external object emission requires explicit CINCH_TEST_OBJECTS; refusing a Native fallback".into()); }
        ir = exe.with_extension("ll");
        if exe == ir || exe == exe.with_extension("abi.json") { return Err("object output collides with its companions".into()); }
        std::fs::create_dir_all(dir.join("build")).map_err(|e| e.to_string())?;
    }
    if matches!(mode, Mode::EmitObject | Mode::EmitIr) {
        if let Some(configuration) = objects {
            let configuration = configuration.canonicalize().map_err(|e| format!("selected library object configuration: {e}"))?;
            args.extend(["--objects".into(), configuration.display().to_string()]);
        }
    }
    args.extend([source.display().to_string(), ir.display().to_string()]);
    match mode {
        Mode::EmitObject => args.extend([exe.display().to_string(), exe.with_extension("abi.json").display().to_string(), format!("{}/", dir.join("build").display())]),
        Mode::Build | Mode::Run => args.extend([exe.display().to_string(), report.display().to_string()]),
        Mode::Check | Mode::EmitIr => (),
    }
    let receipt = Json::obj(vec![("compiler", Json::Str(compiler.display().to_string())), ("interfaces", Json::Str(interfaces.display().to_string())), ("entry", Json::Str(entry.display().to_string())), ("mode", Json::Str(format!("{mode:?}"))), ("arguments", Json::Arr(args.iter().cloned().map(Json::Str).collect())), ("inputs", Json::Arr(inputs))]);
    std::fs::write(work.join("invocation.json"), receipt.to_text()).map_err(|e| e.to_string())?;
    let output = std::process::Command::new(&compiler).args(&args).current_dir(&dir).output().map_err(|e| format!("invoke selected Cinch compiler (no Native fallback): {e}"))?;
    std::fs::write(work.join("compiler.stdout"), &output.stdout).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler.stderr"), &output.stderr).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler-status.txt"), format!("{:?}\n", output.status.code())).map_err(|e| e.to_string())?;
    // The suite auditor keeps this receipt when the fixture removes its project.
    record_invocation(&invocation_id, receipt, output.status.code())?;
    if !output.status.success() {
        let diagnostic = std::fs::read_to_string(ir.with_extension("ll.diagnostics")).unwrap_or_else(|_| String::from_utf8_lossy(&output.stderr).into_owned());
        return Err(format!("selected Cinch compiler exited {:?}: {diagnostic}\ninvocation: {}", output.status.code(), work.display()));
    }
    match mode {
        Mode::Check => Ok(0),
        Mode::Build => if exe.is_file() { Ok(0) } else { Err("selected compiler succeeded without an executable".into()) },
        // Match the Native driver: programs inherit the test caller's working directory.
        Mode::Run => std::process::Command::new(&exe).status().map_err(|e| e.to_string())?.code().ok_or_else(|| "external test artifact terminated without an exit code".into()),
        Mode::EmitIr => if ir.is_file() {
            if requested_output { std::fs::copy(&ir, &exe).map_err(|e| e.to_string())?; }
            else { print!("{}", std::fs::read_to_string(&ir).map_err(|e| e.to_string())?); }
            Ok(0)
        } else { Err("selected compiler succeeded without LLVM output".into()) },
        Mode::EmitObject => if exe.is_file() && ir.is_file() && exe.with_extension("abi.json").is_file() { Ok(0) }
            else { Err("selected compiler succeeded without object companions".into()) },
    }
}
