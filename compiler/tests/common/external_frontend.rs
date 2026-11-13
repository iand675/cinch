//! Invoke the selected Cinch executable; no Rust frontend is used on this path.
use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package, BASE_PACKAGE};
use cinch::lsp::{parse_json, Json};
use std::collections::BTreeSet;
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
    pub flags: Vec<String>,
    pub program_args: Vec<String>,
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
    // The profile the selected compiler builds for this target. A triple it
    // does not support has no profile at all, and the refusal is the CLI's
    // own: the triple is passed through and the compiler names its set.
    let host = implemented_profile(Some(profile.triple.as_str()))?;
    // Destructured whole, so a new profile field fails to compile here
    // until the selector decides how the selected compiler meets it.
    let TargetProfile {
        name, triple, os, arch, libc, os_version, word_size, endian, vector_shape, features,
        fuel: _, measure: _, allow_eval, artifact, required, jpl, rc_debug, reclaim,
    } = profile;
    // Fuel and measure are upper bounds on driving. The selected compiler
    // performs no driving, so it meets every bound.
    //
    // `name` is compared only when the request names a FOREIGN triple. A
    // `native()` request is the host itself under a different label
    // (`native64`, the store key host artifacts are filed under, while the
    // derived profile is named by its triple), so for the host's own triple
    // the name carries no information the triple has not already fixed.
    // Renaming `native()` instead would invalidate every recorded manifest
    // and cached artifact that keyed on `native64`.
    let foreign_triple = *triple != TargetProfile::native().triple;
    let mut lacking = Vec::new();
    for (field, same) in [
        ("name", !foreign_triple || *name == host.name),
        ("triple", *triple == host.triple),
        ("os", *os == host.os),
        ("arch", *arch == host.arch),
        ("libc", *libc == host.libc),
        ("os_version", *os_version == host.os_version),
        ("word_size", *word_size == host.word_size),
        ("endian", *endian == host.endian),
        ("vector_shape", *vector_shape == host.vector_shape),
        ("features", *features == host.features),
        ("allow_eval", *allow_eval == host.allow_eval),
        ("artifact", *artifact == host.artifact),
        ("required", *required == host.required),
        ("jpl", *jpl == host.jpl),
        ("reclaim", *reclaim == host.reclaim),
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
    if host.triple != TargetProfile::native().triple {
        flags.push("--target".to_owned());
        flags.push(host.triple.clone());
    }
    if *rc_debug {
        flags.push("--rc-debug".to_owned());
    }
    Ok(Request { flags, program_args: Vec::new() })
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
        ("explicit object roots", !object_roots.is_empty()),
        ("emitted module sets", !emit_modules.is_empty()),
        ("package objects", !package_objects.is_empty()),
        ("--rebuild", *rebuild_store),
        ("resolved native words", !native_words.is_empty()),
        ("--print-link-plan", *print_link_plan),
        ("--allow-raw-link", !allow_raw_link.is_empty()),
        ("--runtime-archive", runtime_archive.is_some()),
        ("--opt-level", opt_level.is_some()),
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
    if !lacking.is_empty() {
        return Err(format!("external frontend lacks {}", lacking.join(", ")));
    }
    // The program's words reach only a program the compile runs.
    let program_args = if mode == Mode::Run { program_args.clone() } else { Vec::new() };
    Ok(Request { flags: target_flags, program_args })
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
    drive_profile(frontend, mode, dir, entry, out, &Request::default())
}

pub fn drive_with_objects(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, objects: Option<PathBuf>) -> Result<i32, String> {
    drive_request(frontend, mode, dir, entry, out, objects, &Request::default())
}

/// [`drive`] with the profile flags and program words of `request`.
pub fn drive_profile(frontend: &Frontend, mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, request: &Request) -> Result<i32, String> {
    let objects = if matches!(mode, Mode::EmitObject | Mode::Check) {
        std::env::var_os("CINCH_TEST_OBJECTS").filter(|p| !p.is_empty()).map(PathBuf::from)
    } else {
        None
    };
    drive_request(frontend, mode, dir, entry, out, objects, request)
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
    // The entry stem also names the build manifest emitted by the frontend.
    let source = work.join(entry.file_name().ok_or("external test entry needs a filename")?);
    std::fs::write(&source, bundle).map_err(|e| e.to_string())?;
    let mut ir = work.join("output.ll");
    let requested_output = out.is_some();
    let exe = out.map(|p| if p.is_absolute() { p } else { std::env::current_dir().unwrap_or_else(|_| dir.clone()).join(p) }).unwrap_or_else(|| if mode == Mode::EmitObject { entry.with_extension("o") } else { work.join("program") });
    let report = work.join("link-report.txt");
    // This CLI has no check mode that writes the artifacts a Native check
    // writes, so a check runs its interface-and-manifest emitting mode and
    // the selector publishes what it wrote into the project's `build/`.
    let flag = match mode { Mode::Check => "--emit-ir-with-interfaces", Mode::EmitIr => "--emit-ir", Mode::EmitObject => "--emit-object", Mode::Build | Mode::Run => "--compile-only" };
    let mut args = vec![flag.to_owned()];
    args.extend(request.flags.iter().cloned());
    args.extend(["--interfaces".into(), interfaces.display().to_string()]);
    if mode == Mode::Check && objects.is_none() {
        return Err("external check requires CINCH_TEST_OBJECTS: this CLI writes a check's interfaces and build manifest only in its interface mode".into());
    }
    if mode == Mode::EmitObject {
        if objects.is_none() { return Err("external object emission requires explicit CINCH_TEST_OBJECTS; refusing a Native fallback".into()); }
        ir = exe.with_extension("ll");
        if exe == ir || exe == exe.with_extension("abi.json") { return Err("object output collides with its companions".into()); }
        std::fs::create_dir_all(dir.join("build")).map_err(|e| e.to_string())?;
    }
    if matches!(mode, Mode::EmitObject | Mode::EmitIr | Mode::Check) {
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
        Mode::Check => { publish_check_artifacts(frontend, &dir, &entry, &ir)?; Ok(0) }
        Mode::Build => if exe.is_file() { Ok(0) } else { Err("selected compiler succeeded without an executable".into()) },
        // Match the Native driver: programs inherit the test caller's working directory.
        Mode::Run => std::process::Command::new(&exe).args(&request.program_args).status().map_err(|e| e.to_string())?.code().ok_or_else(|| "external test artifact terminated without an exit code".into()),
        Mode::EmitIr => if ir.is_file() {
            if requested_output { std::fs::copy(&ir, &exe).map_err(|e| e.to_string())?; }
            else { print!("{}", std::fs::read_to_string(&ir).map_err(|e| e.to_string())?); }
            Ok(0)
        } else { Err("selected compiler succeeded without LLVM output".into()) },
        Mode::EmitObject => if exe.is_file() && ir.is_file() && exe.with_extension("abi.json").is_file() { Ok(0) }
            else { Err("selected compiler succeeded without object companions".into()) },
    }
}

/// Publish a check's artifacts into the project's `build/` directory: the
/// base interfaces the selected manifest names, the project's own
/// interfaces, and the build manifest. That is the layout a Native check
/// leaves, which the selectors read back.
fn publish_check_artifacts(frontend: &Frontend, dir: &Path, entry: &Path, ir: &Path) -> Result<(), String> {
    let build = dir.join("build");
    std::fs::create_dir_all(&build).map_err(|error| format!("{}: {error}", build.display()))?;
    let text = std::fs::read_to_string(&frontend.interfaces)
        .map_err(|error| format!("{}: {error}", frontend.interfaces.display()))?;
    let manifest = parse_json(&text)?;
    let rows = manifest
        .get("interfaces")
        .and_then(Json::as_arr)
        .ok_or("selected interface manifest names no interfaces")?;
    for row in rows {
        let fields = row.as_arr().ok_or("selected interface manifest row is not a list")?;
        let (Some(name), Some(path)) = (
            fields.first().and_then(Json::as_str),
            fields.get(2).and_then(Json::as_str),
        ) else {
            return Err("selected interface manifest row is not (name, hash, path)".into());
        };
        let target = build.join(format!("{name}.cinchif"));
        std::fs::copy(path, &target).map_err(|error| format!("{}: {error}", target.display()))?;
    }
    let work = ir.parent().ok_or("check output needs a directory")?;
    let named = ir.file_name().and_then(|name| name.to_str()).ok_or("check output needs a filename")?;
    let prefix = format!("{named}.");
    for artifact in std::fs::read_dir(work).map_err(|error| format!("{}: {error}", work.display()))? {
        let path = artifact.map_err(|error| error.to_string())?.path();
        let file = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
        let Some(module) = file.strip_prefix(&prefix).and_then(|rest| rest.strip_suffix(".cinchif")) else { continue };
        let target = build.join(format!("{module}.cinchif"));
        std::fs::copy(&path, &target).map_err(|error| format!("{}: {error}", target.display()))?;
    }
    let stem = entry.file_stem().and_then(|name| name.to_str()).ok_or("check entry needs a name")?;
    let written = PathBuf::from(format!("{}.manifest.json", ir.display()));
    let target = build.join(format!("{stem}.manifest.json"));
    std::fs::copy(&written, &target).map_err(|error| format!("{}: {error}", target.display()))?;
    Ok(())
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    /// A `native()` request is the host under its own label. The derived
    /// profile names itself by triple and `native()` names itself `native64`,
    /// so comparing the name would refuse the host its own target; the
    /// triple already fixes which profile this is.
    #[test]
    fn a_native_request_is_accepted_for_the_hosts_own_triple() {
        let request = TargetProfile { reclaim: true, ..TargetProfile::native() };
        assert_eq!(request.name, "native64");
        assert_ne!(request.triple, request.name, "the labels must differ for this test to mean anything");
        let accepted = profile_request(Mode::Build, StageMode::Refresh, &request);
        assert!(accepted.is_ok(), "native request refused: {}", accepted.unwrap_err());
    }

    /// A foreign triple still has to match its own derived profile exactly,
    /// including the name, so the exemption above cannot hide a real
    /// disagreement about which target was asked for. Built from the real
    /// derived profile with only `name` perturbed, so the only field that can
    /// be reported is the one under test.
    #[test]
    fn a_foreign_triple_must_match_its_own_profile_including_the_name() {
        let triple = "aarch64-unknown-linux-gnu";
        let derived = cinch::driver::target_profile(&DriverOptions {
            target: Some(triple.to_string()),
            ..DriverOptions::default()
        })
        .expect("the foreign triple is one the compiler supports");
        let request = TargetProfile { reclaim: true, ..derived.clone() };
        assert_eq!(request.name, triple, "a derived profile is named by its triple");
        assert!(profile_request(Mode::Build, StageMode::Refresh, &request).is_ok(),
            "the derived profile itself must be accepted");

        let renamed = TargetProfile { name: "native64".to_string(), ..request };
        let refused = profile_request(Mode::Build, StageMode::Refresh, &renamed);
        assert!(refused.is_err(), "a foreign triple was accepted under the host's name");
        let message = refused.unwrap_err();
        assert!(message.contains("name"), "the refusal must name the field: {message}");
        assert!(!message.contains("os, libc") && !message.contains("triple"),
            "only the name should disagree: {message}");
    }
}
