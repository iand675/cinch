//! Invoke the selected Cinch executable; no Rust frontend is used on this path.
use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{base_package, DriverOptions, Mode};
use cinch::lsp::{parse_json, Json};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct Frontend {
    pub compiler: PathBuf,
    pub interfaces: PathBuf,
}

/// What one compile asks of the selected compiler beyond its mode: the
/// command-line words that select a target profile, and the words a `Run`
/// hands the built program.
#[derive(Clone, Debug, Default)]
pub struct Request {
    pub diagnostics_only: bool,
    pub discard_interfaces: bool,
    pub flags: Vec<String>,
    pub program_args: Vec<String>,
    source_profile: Option<TargetProfile>,
    link: Option<DriverOptions>,
    unit_roots: Vec<(String, String)>,
}

/// The profile the selected compiler implements for one `--target` value:
/// the triple's own profile when one is named, else the build host's native
/// target. Reference counting is on because the selected compiler's runtime
/// emission counts every block.
///
/// Derived through `cinch::driver::target_profile`, never listed by hand,
/// because a triple-built profile names the TRIPLE (`aarch64-apple-darwin`)
/// while `TargetProfile::native()` names itself `native64`. Comparing a
/// triple-built profile against `native()` therefore refused even the host's
/// own triple on the `name` field alone, which is what made every
/// target-profile probe report "external frontend lacks target profile".
fn implemented_profile(triple: Option<&str>) -> Result<TargetProfile, String> {
    let profile = cinch::driver::target_profile(&DriverOptions {
        target: triple.map(str::to_string),
        ..DriverOptions::default()
    })
    .map_err(|error| error.to_string())?;
    Ok(TargetProfile { reclaim: true, ..profile })
}

/// The request for a compile under `stage` and `profile`, or the options the
/// selected compiler cannot honour. Never a Native fallback.
pub fn profile_request(mode: Mode, stage: StageMode, profile: &TargetProfile) -> Result<Request, String> {
    // The selected compiler performs no compile-time I/O, so it has no
    // snapshots to replay and no live provenance to record.
    if stage != StageMode::Refresh {
        return Err(format!("external frontend lacks compile-time I/O stage mode {stage:?}"));
    }
    let backend = implemented_profile(Some(profile.triple.as_str()));
    let has_backend = backend.is_ok();
    let host = match backend {
        Ok(target) => target,
        Err(_) if mode == Mode::Check => implemented_profile(None)?,
        Err(message) => return Err(message),
    };
    // Destructured whole, so a new profile field fails to compile here
    // until the selector decides how the selected compiler meets it.
    let TargetProfile {
        name, triple, os, arch, libc, os_version, word_size, endian, vector_shape, features,
        fuel: _, measure: _, allow_eval, artifact, required, jpl, rc_debug, reclaim, no_cpr: _,
    } = profile;
    // Fuel and measure are upper bounds on driving. The selected compiler
    // performs no driving, so it meets every bound. Product results
    // (`no_cpr`) are a code generation choice with no observable behaviour,
    // so the selected compiler meets either setting.
    //
    // `name` is compared only when the request names a FOREIGN triple. A
    // `native()` request is the host itself under a different label
    // (`native64`, the store key host artifacts are filed under, while the
    // derived profile is named by its triple), so for the host's own triple
    // the name carries no information the triple has not already fixed.
    // Renaming `native()` instead would invalidate every recorded manifest
    // and cached artifact that keyed on `native64`.
    let foreign_triple = *triple != TargetProfile::native().triple;
    let core_wasm = host.artifact == cinch::core::ArtifactProfile::CoreWasm
        && *name == TargetProfile::wasm32(host.artifact).name;
    let same_name = !foreign_triple || *name == host.name || core_wasm;
    let same_target = has_backend && *triple == host.triple && *os == host.os
        && (*arch == host.arch || arch == "portable") && *libc == host.libc
        && *word_size == host.word_size && *endian == host.endian;
    let same_platform = same_name && same_target;
    let check_platform = mode == Mode::Check && !same_platform;
    let mut lacking = Vec::new();
    for (field, same) in [
        ("name", same_name || check_platform),
        ("platform", same_target || check_platform),
        ("os_version", *os_version == host.os_version),
        ("vector_shape", *vector_shape == host.vector_shape),
        ("allow_eval", *allow_eval == host.allow_eval),
        ("artifact", *artifact == host.artifact),
        ("jpl", *jpl == host.jpl),
        // `reclaim` is absent on purpose: the selected compiler counts
        // references by default and `--no-reclaim` selects the other arm, so
        // both values of this field are ones it implements. The flag below
        // is how the request states which arm it wants.
    ] {
        if !same {
            lacking.push(field);
        }
    }
    if *rc_debug && mode == Mode::EmitObject {
        lacking.push("rc_debug (library objects)");
    }
    if !lacking.is_empty() {
        return Err(format!("external frontend lacks target profile `{name}`: it differs from the profile it implements for `{}` in {}", host.triple, lacking.join(", ")));
    }
    // The CLI names the target it was asked for; without the flag it builds
    // the host's own target, which is the same profile.
    let mut flags = Vec::new();
    if check_platform {
        flags.extend([
            "--check-platform".to_owned(), name.clone(), triple.clone(), os.clone(),
            arch.clone(), libc.clone(), word_size.to_string(), endian.clone(),
        ]);
    } else if host.triple != TargetProfile::native().triple {
        flags.push("--target".to_owned());
        flags.push(host.triple.clone());
    }
    if arch == "portable" {
        flags.push("--portable-isa".to_owned());
    }
    if check_platform || *features != host.features {
        flags.push("--clear-features".to_owned());
        for feature in features {
            flags.extend(["--target-feature".to_owned(), feature.clone()]);
        }
    }
    for capability in required {
        flags.extend(["--require-capability".to_owned(), capability.clone()]);
    }
    if *rc_debug {
        flags.push("--rc-debug".to_owned());
    }
    // The selected compiler counts references unless a word says otherwise,
    // so a request that does not reclaim is the one that needs a flag.
    if !*reclaim {
        flags.push("--no-reclaim".to_owned());
    }
    let link = (foreign_triple && matches!(mode, Mode::Build | Mode::Run)).then(|| DriverOptions {
        target: Some(host.triple.clone()), ..Default::default()
    });
    Ok(Request { flags, link, source_profile: Some(profile.clone()), ..Request::default() })
}

pub fn profile_options_request(mode: Mode, stage: StageMode, profile: &TargetProfile, opts: &DriverOptions) -> Result<Request, String> {
    let mut request = profile_request(mode, stage, profile)?;
    let options = options_request(mode, opts)?;
    if opts.target.as_ref().is_some_and(|target| target != &profile.triple) {
        return Err("driver target disagrees with the explicit target profile".into());
    }
    if let Some(mut link) = options.link {
        link.target = Some(profile.triple.clone());
        request.link = Some(link);
    }
    if opts.no_reclaim && profile.reclaim {
        request.flags.push("--no-reclaim".into());
    }
    request.unit_roots = options.unit_roots;
    request.program_args = options.program_args;
    Ok(request)
}

/// The request for a compile under driver `opts`, or the options the
/// selected compiler cannot honour. Never a Native fallback.
pub fn options_request(mode: Mode, opts: &DriverOptions) -> Result<Request, String> {
    let DriverOptions {
        object_roots, emit_modules, package_objects, rebuild_store, native_words, print_link_plan,
        allow_raw_link, target, runtime_archive, opt_level, target_version, dump_core, core_out,
        core_only, program_args, max_simplifier_iterations, no_direct_dict_calls, ..
    } = opts;
    let mut lacking = Vec::new();
    // `--target` is a flag the selected compiler accepts for every triple it
    // supports, and it builds that triple's own profile. A triple it does
    // not support is the compiler's own refusal, named in its words, so the
    // selector asks for it rather than inventing a message.
    let mut target_flags = Vec::new();
    if let Some(triple) = target {
        match implemented_profile(Some(triple.as_str())) {
            Ok(profile) => {
                if profile.triple != TargetProfile::native().triple {
                    target_flags.push("--target".to_owned());
                    target_flags.push(triple.clone());
                }
            }
            Err(message) => lacking.push(message),
        }
    }
    for (option, requested) in [
        ("explicit object roots outside object emission", mode != Mode::EmitObject && !object_roots.is_empty()),
        ("emitted module sets", !emit_modules.is_empty()),
        ("package objects", !package_objects.is_empty()),
        ("--rebuild", *rebuild_store),
        ("resolved native words", !native_words.is_empty()),
        ("--print-link-plan", *print_link_plan),
        ("--allow-raw-link", !allow_raw_link.is_empty()),
        ("--runtime-archive", runtime_archive.is_some() && !matches!(mode, Mode::Build | Mode::Run)),
        ("--opt-level", opt_level.is_some() && !matches!(mode, Mode::Build | Mode::Run)),
        ("--target-version", target_version.is_some()),
        ("--dump-core", dump_core.is_some()),
        ("core emit output", core_out.is_some()),
        ("core emit mode", *core_only),
        // The simplifier budget and the direct-dictionary switch change what
        // the Native driver emits; a selected compiler that cannot honour them
        // must say so rather than silently keep its own defaults.
        ("--max-simplifier-iterations", max_simplifier_iterations.is_some()),
        ("--no-direct-dict-calls", *no_direct_dict_calls),
    ] {
        if requested {
            lacking.push(option.to_owned());
        }
    }
    let mut unit_roots = Vec::new();
    for root in object_roots {
        match root {
            cinch::specialize::ObjectRoot::Unit { symbol, classes } =>
                unit_roots.push((symbol.clone(), cinch::repr::letters(classes))),
            cinch::specialize::ObjectRoot::Dictionary { .. } => lacking.push("explicit dictionary roots".into()),
            cinch::specialize::ObjectRoot::Method { .. } => lacking.push("explicit method roots".into()),
        }
    }
    if !lacking.is_empty() {
        return Err(format!("external frontend lacks {}", lacking.join(", ")));
    }
    // The program's words reach only a program the compile runs.
    let program_args = if mode == Mode::Run { program_args.clone() } else { Vec::new() };
    let link = (matches!(mode, Mode::Build | Mode::Run)
        && (!target_flags.is_empty() || runtime_archive.is_some() || opt_level.is_some())).then(|| opts.clone());
    if opts.no_reclaim { target_flags.push("--no-reclaim".into()); }
    Ok(Request { flags: target_flags, unit_roots, program_args, link, ..Request::default() })
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
    let bytes = std::fs::read(path).map_err(|e| format!("read external interfaces: {e}"))?;
    if bytes.starts_with(b"CINCHPK4") {
        let names = packed_interface_names(&bytes).ok_or("invalid external packed interface directory")?;
        return validate_base_module_names(names);
    }
    let text = std::str::from_utf8(&bytes).map_err(|e| format!("external interface manifest: {e}"))?;
    let doc = parse_json(text).map_err(|e| format!("external interface manifest: {e}"))?;
    if doc.get("schema").and_then(Json::as_num) != Some(1.0) {
        return Err("external interface manifest must use schema 1".into());
    }
    let rows = doc.get("interfaces").and_then(Json::as_arr).ok_or("external manifest needs interfaces")?;
    let mut names = Vec::with_capacity(rows.len());
    for row in rows {
        let fields = row.as_arr().ok_or("invalid external interface row")?;
        if fields.len() != 3 || fields.iter().any(|x| x.as_str().is_none_or(str::is_empty)) {
            return Err("external interface row must contain module, hash and path strings".into());
        }
        names.push(fields[0].as_str().unwrap());
    }
    validate_base_module_names(names)
}

// The test wrapper only restricts which modules may be supplied. The selected
// compiler validates the package and checks each contract's identity.
fn packed_interface_names(bytes: &[u8]) -> Option<Vec<&str>> {
    fn word(bytes: &[u8], at: usize) -> Option<usize> {
        Some(u32::from_le_bytes(bytes.get(at..at.checked_add(4)?)?.try_into().ok()?) as usize)
    }
    let section = |at| {
        let start = word(bytes, at)?;
        let length = word(bytes, at + 4)?;
        bytes.get(start..start.checked_add(length)?)
    };
    let strings = section(16)?;
    let text_start = word(strings, 0)?.checked_mul(8)?.checked_add(4)?;
    let text = strings.get(text_start..)?;
    let modules = section(24)?;
    let rows = modules.chunks_exact(36);
    if !rows.remainder().is_empty() {
        return None;
    }
    rows.map(|row| {
        let start = word(row, 0)?;
        let length = word(row, 4)?;
        std::str::from_utf8(text.get(start..start.checked_add(length)?)?).ok()
    }).collect()
}

fn validate_base_module_names<'a>(names: impl IntoIterator<Item = &'a str>) -> Result<(), String> {
    static BASE_MODULES: std::sync::LazyLock<BTreeSet<&str>> = std::sync::LazyLock::new(|| {
        base_package().modules.iter().map(|module| module.name.as_str()).collect()
    });
    let mut seen = BTreeSet::new();
    for name in names {
        if !BASE_MODULES.contains(name) || !seen.insert(name) {
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
fn reachable_sources(files: &[PathBuf], entry: &Path, profile: &TargetProfile) -> Vec<PathBuf> {
    // The caller has verified the entry is present. With no sibling sources,
    // a header scan cannot add any input to the selected compiler's bundle.
    if files.len() == 1 {
        return files.to_vec();
    }
    let headers = files.iter().map(|path| {
        std::fs::read_to_string(path).ok().and_then(|text| cinch::driver::scan_file_for(&text, &path.display().to_string(), profile).ok())
    }).collect::<Vec<_>>();
    let mut providers = HashMap::<&str, Vec<usize>>::new();
    for (index, header) in headers.iter().enumerate() {
        if let Some(header) = header {
            for (name, _) in &header.modules {
                providers.entry(name).or_default().push(index);
            }
        }
    }

    // The caller supplies sorted paths and has validated the entry.
    let start = files.binary_search_by(|path| path.as_path().cmp(entry)).expect("project entry");
    let mut selected = BTreeSet::from([start]);
    let mut pending = vec![start];
    while let Some(index) = pending.pop() {
        let Some(header) = &headers[index] else { continue };
        for (_, imports) in &header.modules {
            for (wanted, _) in imports {
                // Keep every provider so the compiler can diagnose duplicates.
                if let Some(matches) = providers.get(wanted.as_str()) {
                    for &provider in matches {
                        if selected.insert(provider) {
                            pending.push(provider);
                        }
                    }
                }
            }
        }
    }
    selected.into_iter().map(|index| files[index].clone()).collect()
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

/// Check the entire project when the caller consumes diagnostics alone.
pub fn check_diagnostics(frontend: &Frontend, dir: &Path, entry: &str) -> Result<(), String> {
    let request = Request { diagnostics_only: true, ..Request::default() };
    drive_profile(frontend, Mode::Check, dir, entry, None, &request).map(|_| ())
}

pub fn drive(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    drive_profile(frontend, mode, dir, entry, out, &Request::default())
}

pub fn drive_with_objects(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, objects: Option<PathBuf>) -> Result<i32, String> {
    drive_request(frontend, mode, dir, entry, out, objects, &Request::default())
}

/// [`drive`] with the profile flags and program words of `request`.
pub fn drive_profile(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, request: &Request) -> Result<i32, String> {
    if let Some(options) = &request.link {
        let profile = cinch::driver::target_profile(options).map_err(|e| e.to_string())?;
        cinch::driver::check_execution_target(mode, &profile, &request.program_args).map_err(|e| e.to_string())?;
    }
    if let Some((interfaces, configuration)) = target_inputs(request, frontend)? {
        let target = Frontend { compiler: frontend.compiler.clone(), interfaces };
        return drive_request(&target, mode, dir, entry, out, Some(configuration), request);
    }
    let objects = if mode == Mode::EmitObject || request.link.is_some() {
        std::env::var_os("CINCH_TEST_OBJECTS").filter(|p| !p.is_empty()).map(PathBuf::from)
    } else {
        None
    };
    drive_request(frontend, mode, dir, entry, out, objects, request)
}

fn target_inputs(request: &Request, frontend: &Frontend) -> Result<Option<(PathBuf, PathBuf)>, String> {
    let target = request.flags.windows(2).find(|pair| pair[0] == "--target");
    let uncounted = request.flags.iter().any(|arg| arg == "--no-reclaim");
    if target.is_none() && !uncounted { return Ok(None); }
    let Some(helper) = std::env::var_os("CINCH_TEST_TARGET_INPUTS") else { return Ok(None) };
    if std::env::var_os("CINCH_TEST_COMPILER").as_deref() != Some(frontend.compiler.as_os_str())
        || std::env::var_os("CINCH_TEST_INTERFACES").as_deref() != Some(frontend.interfaces.as_os_str())
    {
        return Ok(None);
    }
    let mut command = std::process::Command::new(helper);
    command.args(["--test-target-inputs", target.map_or("native", |pair| pair[1].as_str())]);
    if uncounted { command.arg("--no-reclaim"); }
    let output = command.output().map_err(|err| format!("prepare target inputs: {err}"))?;
    if !output.status.success() {
        return Err(format!("prepare target inputs: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let json = parse_json(std::str::from_utf8(&output.stdout).map_err(|err| err.to_string())?)?;
    let path = |key| json.get(key).and_then(Json::as_str).map(PathBuf::from)
        .ok_or_else(|| format!("missing target library {key}"));
    Ok(Some((path("interfaces")?, path("objects")?)))
}

fn drive_request(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, objects: Option<PathBuf>, request: &Request) -> Result<i32, String> {
    let compiler = frontend.compiler.canonicalize().map_err(|e| format!("selected compiler (no Native fallback): {e}"))?;
    let interfaces = frontend.interfaces.canonicalize().map_err(|e| format!("selected interfaces: {e}"))?;
    validate_interfaces(&interfaces)?;
    let dir = dir.canonicalize().map_err(|e| e.to_string())?;
    let entry = dir.join(entry).canonicalize().map_err(|e| format!("external test entry: {e}"))?;
    if !entry.starts_with(&dir) { return Err("external test entry leaves its project".into()) }
    let mut files = Vec::new(); sources(&dir, &dir, &mut files)?; files.sort();
    if !files.contains(&entry) { return Err("external test entry is not a project Cinch source".into()) }
    let triple = request.flags.windows(2).find(|words| words[0] == "--target").map(|words| words[1].as_str());
    let profile = match &request.source_profile {
        Some(profile) => profile.clone(),
        None => implemented_profile(triple)?,
    };
    let files = reachable_sources(&files, &entry, &profile);
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
    // The entry stem also names the build manifest emitted by the frontend.
    let source = work.join(entry.file_name().ok_or("external test entry needs a filename")?);
    std::fs::write(&source, bundle).map_err(|e| e.to_string())?;
    let mut ir = work.join("output.ll");
    let requested_output = out.is_some();
    let exe = out.map(|p| if p.is_absolute() { p } else { std::env::current_dir().unwrap_or_else(|_| dir.clone()).join(p) }).unwrap_or_else(|| if mode == Mode::EmitObject { entry.with_extension("o") } else { work.join("program") });
    let report = work.join("link-report.txt");
    let flag = match mode { Mode::Check => "--check", Mode::EmitIr => "--emit-ir", Mode::EmitObject => "--emit-object", Mode::Build | Mode::Run => "--compile-only" };
    let flag = if mode == Mode::Check && request.diagnostics_only && std::env::var_os("CINCH_TEST_CHECK_TYPES").as_deref() == Some(std::ffi::OsStr::new("1")) {
        "--check-types"
    } else if request.link.is_some() {
        "--emit-ir"
    } else {
        flag
    };
    let mut args = vec![flag.to_owned()];
    args.extend(request.flags.iter().cloned());
    args.extend(["--interfaces".into(), interfaces.display().to_string()]);
    if mode == Mode::EmitObject {
        if objects.is_none() { return Err("external object emission requires explicit CINCH_TEST_OBJECTS; refusing a Native fallback".into()); }
        ir = exe.with_extension("ll");
        if exe == ir || exe == exe.with_extension("cpk") { return Err("object output collides with its companions".into()); }
    }
    if mode != Mode::Check || request.link.is_some() {
        if let Some(configuration) = &objects {
            let configuration = configuration.canonicalize().map_err(|e| format!("selected library object configuration: {e}"))?;
            args.extend(["--objects".into(), configuration.display().to_string()]);
        }
    }
    if !request.unit_roots.is_empty() {
        let path = work.join("object-roots.json");
        let rows = request.unit_roots.iter().map(|(symbol, classes)| Json::obj(vec![
            ("symbol", Json::Str(symbol.clone())), ("classes", Json::Str(classes.clone())),
        ])).collect();
        let roots = Json::obj(vec![("schema", Json::Num(1.0)), ("units", Json::Arr(rows))]);
        std::fs::write(&path, roots.to_text()).map_err(|e| e.to_string())?;
        args.extend(["--object-roots".to_owned(), path.display().to_string()]);
    }
    args.extend([source.display().to_string(), ir.display().to_string()]);
    match mode {
        Mode::EmitObject => args.extend([exe.display().to_string(), exe.with_extension("cpk").display().to_string()]),
        Mode::Build | Mode::Run if request.link.is_some() => {}
        Mode::Build | Mode::Run => args.extend([exe.display().to_string(), report.display().to_string()]),
        Mode::Check | Mode::EmitIr => (),
    }
    let receipt = Json::obj(vec![("compiler", Json::Str(compiler.display().to_string())), ("interfaces", Json::Str(interfaces.display().to_string())), ("entry", Json::Str(entry.display().to_string())), ("mode", Json::Str(format!("{mode:?}"))), ("arguments", Json::Arr(args.iter().cloned().map(Json::Str).collect())), ("inputs", Json::Arr(inputs))]);
    std::fs::write(work.join("invocation.json"), receipt.to_text()).map_err(|e| e.to_string())?;
    let output = std::process::Command::new(&compiler).args(&args).current_dir(&dir)
        .env("CINCH_TEST_DISCARD_INTERFACES", if request.discard_interfaces { "1" } else { "0" })
        .output().map_err(|e| format!("invoke selected Cinch compiler (no Native fallback): {e}"))?;
    std::fs::write(work.join("compiler.stdout"), &output.stdout).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler.stderr"), &output.stderr).map_err(|e| e.to_string())?;
    std::fs::write(work.join("compiler-status.txt"), format!("{:?}\n", output.status.code())).map_err(|e| e.to_string())?;
    // The suite auditor keeps this receipt when the fixture removes its project.
    record_invocation(&invocation_id, receipt, output.status.code())?;
    if !output.status.success() {
        let diagnostic = std::fs::read_to_string(ir.with_extension("ll.diagnostics")).unwrap_or_else(|_| String::from_utf8_lossy(&output.stderr).into_owned());
        return Err(format!("selected Cinch compiler exited {:?}: {diagnostic}\ninvocation: {}", output.status.code(), work.display()));
    }
    if let Some(options) = &request.link {
        let config = objects.as_ref().ok_or("linking selected IR requires its library object configuration")?;
        let json = parse_json(&std::fs::read_to_string(config).map_err(|e| e.to_string())?)?;
        let library = json.get("object").and_then(Json::as_str).ok_or("library object configuration needs an object path")?;
        let mut options = options.clone();
        let profile = cinch::driver::target_profile(&options).map_err(|e| e.to_string())?;
        if options.runtime_archive.is_none() && profile.triple == TargetProfile::native().triple {
            options.runtime_archive = Some(std::env::var_os("CINCH_TEST_RUNTIME_ARCHIVE").map(PathBuf::from)
                .ok_or("linking selected native IR requires its pinned runtime archive")?);
        }
        cinch::driver::link_emitted_ir(&ir, &exe, library, &profile, &options).map_err(|e| e.to_string())?;
        if mode == Mode::Run && profile.arch == "wasm32" {
            return cinch::driver::run_wasm_module(&exe, &profile).map_err(|e| e.to_string());
        }
    }
    match mode {
        Mode::Check => Ok(0),
        Mode::Build => if exe.is_file() {
            // Match the development driver's inspectable LLVM companion so
            // consumers can verify the actual selected compiler's kernels.
            std::fs::copy(&ir, exe.with_extension("ll")).map_err(|e| e.to_string())?;
            Ok(0)
        } else { Err("selected compiler succeeded without an executable".into()) },
        // Match the Native driver: programs inherit the test caller's working directory.
        Mode::Run => std::process::Command::new(&exe).args(&request.program_args).status().map_err(|e| e.to_string())?.code().ok_or_else(|| "external test artifact terminated without an exit code".into()),
        Mode::EmitIr => if ir.is_file() {
            if requested_output { std::fs::copy(&ir, &exe).map_err(|e| e.to_string())?; }
            else { print!("{}", std::fs::read_to_string(&ir).map_err(|e| e.to_string())?); }
            Ok(0)
        } else { Err("selected compiler succeeded without LLVM output".into()) },
        Mode::EmitObject => if exe.is_file() && ir.is_file() && exe.with_extension("cpk").is_file() { Ok(0) }
            else { Err("selected compiler succeeded without object companions".into()) },
    }
}
