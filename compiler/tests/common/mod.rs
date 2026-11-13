//! The harness the bootstrap ladder's tests share: the project a rung is
//! exercised in, and the toolkit sources every project ships.
//!
//! The compiler does not embed the P30/P31 toolkit: `Text.Lex`,
//! `Text.Scan`, `Text.Parse`, `Text.Json` and `Text.Builder` are on disk and absent from
//! `compiler/prelude/cinch-base.cinchpkg`, so a project ships them as
//! sources beside its entry. That is the rule
//! `design/stdlib-and-bootstrap.md` states and the pattern
//! `compiler/tests/parsers.rs` and `json.rs` already use. Everything else
//! the bootstrap modules import (`Data.*`, `Text.Show`, `Text.Codec`,
//! `Effect.Io`/`Process`/`Fs`/`Clock`) is in the embedded family and
//! resolves on its own.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver, DriverOptions, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub mod external_frontend;
pub mod source_frontend;

/// The toolkit a bootstrap project ships. Order follows the modules'
/// own imports: `Text.Parse` imports `Text.Lex` and `Text.Scan`; the
/// renderers emit through `Text.Builder`, because a right-nested
/// `string-concat` over a long sequence recopies its suffix at every step.
pub const TOOLKIT: &[(&str, &str)] = &[
    (
        "Text.Lex.cinch",
        include_str!("../../prelude/Text/Lex.cinch"),
    ),
    (
        "Text.Scan.cinch",
        include_str!("../../prelude/Text/Scan.cinch"),
    ),
    (
        "Text.Parse.cinch",
        include_str!("../../prelude/Text/Parse.cinch"),
    ),
    (
        "Text.Json.cinch",
        include_str!("../../prelude/Text/Json.cinch"),
    ),
    (
        "Text.Builder.cinch",
        include_str!("../../prelude/Text/Builder.cinch"),
    ),
];

/// A fresh project directory holding the toolkit and `files`. The caller
/// owns the directory's lifetime (`cleanup`), because a rung's evidence is
/// usually a file its program wrote there.
pub fn assert_source_root(compiled: &Path, active: &Path) {
    let compiled_root = compiled.canonicalize().expect("compiled package root");
    let active_root = active.canonicalize().expect("active package root");
    assert_eq!(compiled_root, active_root,
        "cached test paths refer to another checkout; use a separate Cargo target directory for this source snapshot");
}

fn project_directory(tag: &str) -> PathBuf {
    // A Cargo cache shared by copied checkouts can retain env! paths from
    // the previous checkout. Refuse that mixture before compiling a rung.
    let active_root = std::env::var_os("CINCH_TEST_SOURCE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("active package root"));
    assert_source_root(Path::new(env!("CARGO_MANIFEST_DIR")), &active_root);
    prune_stale_projects();
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_boot_{}_{}_{}", tag, std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    dir
}

/// Remove project directories a previous run left behind.
///
/// Every test that compiles a rung writes a whole compiler source tree into
/// the system temp directory, so one `cargo test` invocation for the ladder
/// leaves gigabytes behind and a full suite leaves hundreds of gigabytes.
/// A run shorter than the cutoff keeps its own directories, and every
/// sibling that outlives the cutoff is dead because no test run lasts that
/// long.
fn prune_stale_projects() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        const CUTOFF: std::time::Duration = std::time::Duration::from_secs(2 * 60 * 60);
        let root = std::env::temp_dir();
        let Ok(entries) = std::fs::read_dir(&root) else { return };
        let now = std::time::SystemTime::now();
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !(name.starts_with("cinch_boot_") || name.starts_with("cinch-run-")) {
                continue;
            }
            let Ok(meta) = entry.metadata() else { continue };
            if !meta.is_dir() {
                continue;
            }
            let stale = meta
                .modified()
                .ok()
                .and_then(|at| now.duration_since(at).ok())
                .is_some_and(|age| age > CUTOFF);
            if stale {
                let _ = std::fs::remove_dir_all(entry.path());
            }
        }
    });
}

pub fn project(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = project_directory(tag);
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.RuntimeIr.cinch" | "Bootstrap.Pipeline.cinch" | "Bootstrap.Object.cinch")) {
        std::fs::write(dir.join("Bootstrap.Object.cinch"), include_str!("../../../bootstrap/Bootstrap.Object.cinch"))
            .expect("checked runtime object dependency");
    }
    std::fs::write(dir.join("Bootstrap.RuntimeAbi.cinch"), include_str!("../../../bootstrap/Bootstrap.RuntimeAbi.cinch"))
        .expect("exact runtime ABI dependency");
    if files.iter().any(|(name, _)| *name == "Bootstrap.Pipeline.cinch") {
        std::fs::write(dir.join("Bootstrap.RuntimeIr.cinch"), include_str!("../../../bootstrap/Bootstrap.RuntimeIr.cinch"))
            .expect("typed pipeline emitter dependency");
        for (name, source) in [
            ("Bootstrap.Export.cinch", include_str!("../../../bootstrap/Bootstrap.Export.cinch")),
            ("Bootstrap.ExportType.cinch", include_str!("../../../bootstrap/Bootstrap.ExportType.cinch")),
            ("Bootstrap.ExportNominal.cinch", include_str!("../../../bootstrap/Bootstrap.ExportNominal.cinch")),
            ("Bootstrap.Stage.cinch", include_str!("../../../bootstrap/Bootstrap.Stage.cinch")),
            ("Bootstrap.StageEval.cinch", include_str!("../../../bootstrap/Bootstrap.StageEval.cinch")),
        ] {
            std::fs::write(dir.join(name), source).expect("checked pipeline interface dependency");
        }
    }
    std::fs::write(dir.join("Bootstrap.Index.cinch"), include_str!("../../../bootstrap/Bootstrap.Index.cinch"))
        .expect("shared index syntax dependency");
    std::fs::write(dir.join("Bootstrap.CoreBody.cinch"), include_str!("../../../bootstrap/Bootstrap.CoreBody.cinch"))
        .expect("dense body codec dependency");
    std::fs::write(dir.join("Bootstrap.Total.cinch"), include_str!("../../../bootstrap/Bootstrap.Total.cinch"))
        .expect("checked total-body dependency");
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Core.cinch" | "Bootstrap.Global.cinch" | "Bootstrap.Typing.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Dictionary.cinch" | "Bootstrap.Instance.cinch")) {
        for (name, source) in [
            ("Bootstrap.Load.cinch", include_str!("../../../bootstrap/Bootstrap.Load.cinch")),
            ("Bootstrap.Interface.cinch", include_str!("../../../bootstrap/Bootstrap.Interface.cinch")),
            ("Bootstrap.Parse.cinch", include_str!("../../../bootstrap/Bootstrap.Parse.cinch")),
        ] { std::fs::write(dir.join(name), source).expect("checked method body dependency"); }
        std::fs::write(dir.join("Bootstrap.ExportType.cinch"), include_str!("../../../bootstrap/Bootstrap.ExportType.cinch")).expect("checked Core identity export dependency");
        std::fs::write(dir.join("Bootstrap.TotalSemantics.cinch"), include_str!("../../../bootstrap/Bootstrap.TotalSemantics.cinch")).expect("checked body arithmetic dependency");
        std::fs::write(dir.join("Bootstrap.BodyContract.cinch"), include_str!("../../../bootstrap/Bootstrap.BodyContract.cinch"))
            .expect("checked body/declaration association dependency");
        std::fs::write(dir.join("Bootstrap.Guarded.cinch"), include_str!("../../../bootstrap/Bootstrap.Guarded.cinch"))
            .expect("checked guarded-body dependency");
    }
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Core.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Dictionary.cinch" | "Bootstrap.TotalSemantics.cinch")) {
        std::fs::write(dir.join("Bootstrap.Proof.cinch"), include_str!("../../../bootstrap/Bootstrap.Proof.cinch"))
            .expect("shared proof checker dependency");
    }
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Type.cinch" | "Bootstrap.Core.cinch" | "Bootstrap.Dictionary.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Load.cinch" | "Bootstrap.Contracts.cinch" | "Bootstrap.Nominal.cinch")) {
        for (name, source) in [
            ("Bootstrap.Nominal.cinch", include_str!("../../../bootstrap/Bootstrap.Nominal.cinch")),
            ("Bootstrap.Foreign.cinch", include_str!("../../../bootstrap/Bootstrap.Foreign.cinch")),
            ("Bootstrap.Builtin.cinch", include_str!("../../../bootstrap/Bootstrap.Builtin.cinch")),
            ("Bootstrap.Ast.cinch", include_str!("../../../bootstrap/Bootstrap.Ast.cinch")),
            ("Bootstrap.Report.cinch", include_str!("../../../bootstrap/Bootstrap.Report.cinch")),
        ] {
            std::fs::write(dir.join(name), source).expect("nominal contract dependency");
        }
    }
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch")) {
        std::fs::write(dir.join("Bootstrap.Parse.cinch"), include_str!("../../../bootstrap/Bootstrap.Parse.cinch")).expect("checked source signature parser dependency");
        std::fs::write(dir.join("Bootstrap.Contracts.cinch"), include_str!("../../../bootstrap/Bootstrap.Contracts.cinch")).expect("checked source reference dependency");
        std::fs::write(dir.join("Bootstrap.Load.cinch"), include_str!("../../../bootstrap/Bootstrap.Load.cinch")).expect("checked source interface dependency");
        std::fs::write(dir.join("Bootstrap.Interface.cinch"), include_str!("../../../bootstrap/Bootstrap.Interface.cinch")).expect("checked source contract identity dependency");
        std::fs::write(dir.join("Bootstrap.Kinds.cinch"), include_str!("../../../bootstrap/Bootstrap.Kinds.cinch")).expect("independent kind checking dependency");
        std::fs::write(dir.join("Bootstrap.Typing.cinch"), include_str!("../../../bootstrap/Bootstrap.Typing.cinch")).expect("independent Core typing dependency");
        std::fs::write(dir.join("Bootstrap.Global.cinch"), include_str!("../../../bootstrap/Bootstrap.Global.cinch")).expect("global reference checking dependency");
        std::fs::write(dir.join("Bootstrap.Instance.cinch"), include_str!("../../../bootstrap/Bootstrap.Instance.cinch")).expect("typed source instance dependency");
        std::fs::write(dir.join("Bootstrap.Builtin.cinch"), include_str!("../../../bootstrap/Bootstrap.Builtin.cinch")).expect("kernel contract dependency");
        std::fs::write(dir.join("Bootstrap.Temporal.cinch"), include_str!("../../../bootstrap/Bootstrap.Temporal.cinch")).expect("temporal contract vocabulary dependency");
        std::fs::write(dir.join("Bootstrap.TemporalCheck.cinch"), include_str!("../../../bootstrap/Bootstrap.TemporalCheck.cinch")).expect("temporal contract decision dependency");
    }
    if files.iter().any(|(name, _)| *name == "Bootstrap.Check.cinch") {
        for (name, src) in [
            ("Bootstrap.Type.cinch", include_str!("../../../bootstrap/Bootstrap.Type.cinch")),
            ("Bootstrap.Core.cinch", include_str!("../../../bootstrap/Bootstrap.Core.cinch")),
            ("Bootstrap.Dictionary.cinch", include_str!("../../../bootstrap/Bootstrap.Dictionary.cinch")),
            ("Bootstrap.Infer.cinch", include_str!("../../../bootstrap/Bootstrap.Infer.cinch")),
            ("Bootstrap.Parse.cinch", include_str!("../../../bootstrap/Bootstrap.Parse.cinch")),
            ("Bootstrap.CoreBody.cinch", include_str!("../../../bootstrap/Bootstrap.CoreBody.cinch")),
        ] {
            std::fs::write(dir.join(name), src).expect("checker dependency");
        }
    }
    for (name, src) in TOOLKIT.iter().chain(files.iter()) {
        std::fs::write(dir.join(name), src).unwrap_or_else(|e| panic!("write {name}: {e}"));
    }
    dir
}

/// The driver call on a thread with a large stack: the checker recurses
/// over the elaborated program.
pub fn drive(mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    if let Some(frontend) = external_frontend::configured()? {
        return external_frontend::drive(&frontend, mode, dir, entry, out);
    }
    drive_native(mode, dir, entry, out)
}

fn drive_native(mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    let path = dir.join(entry);
    on_driver_thread(move || driver(mode, &path, out.as_deref()))
}

fn on_driver_thread(compile: impl FnOnce() -> cinch::Result<i32> + Send + 'static) -> Result<i32, String> {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(compile)
        .expect("spawn driver thread")
        .join()
        .expect("driver thread panicked")
        .map_err(|e| e.to_string())
}

/// [`drive`] under an explicit compile-time I/O mode, for the native
/// target profile.
pub fn drive_with(mode: Mode, stage: StageMode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
    drive_with_profile(mode, stage, dir, entry, out, &TargetProfile::native())
}

/// [`drive`] under an explicit compile-time I/O mode and target profile.
/// The selected compiler receives the profile as flags, or the selector
/// names the profile fields it lacks.
pub fn drive_with_profile(mode: Mode, stage: StageMode, dir: &Path, entry: &str, out: Option<PathBuf>, profile: &TargetProfile) -> Result<i32, String> {
    if let Some(frontend) = external_frontend::configured()? {
        let request = external_frontend::profile_request(mode, stage, profile)?;
        return external_frontend::drive_profile(&frontend, mode, dir, entry, out, &request);
    }
    let (path, profile) = (dir.join(entry), profile.clone());
    on_driver_thread(move || cinch::driver::driver_with_profile(mode, stage, &path, out.as_deref(), &profile))
}

/// [`drive`] with explicit driver options (target triple, object roots,
/// program words). The selected compiler receives what it supports, or the
/// selector names the options it lacks.
pub fn drive_opts(mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>, opts: &DriverOptions) -> Result<i32, String> {
    if let Some(frontend) = external_frontend::configured()? {
        let request = external_frontend::options_request(mode, opts)?;
        return external_frontend::drive_profile(&frontend, mode, dir, entry, out, &request);
    }
    let (path, opts) = (dir.join(entry), opts.clone());
    on_driver_thread(move || cinch::driver::driver_opts(mode, &path, out.as_deref(), &opts))
}

/// Compile and run `entry`; the exit code is the program's.
pub fn run(dir: &Path, entry: &str) -> i32 {
    match drive(Mode::Run, dir, entry, None) {
        Ok(code) => code,
        Err(e) => panic!("{entry}: compilation failed: {e}"),
    }
}

/// Type-check `entry` only.
pub fn check(dir: &Path, entry: &str) -> Result<(), String> {
    drive(Mode::Check, dir, entry, None).map(|_| ())
}

/// Read checked contracts from the compiler that checked the project.
pub fn check_interfaces(dir: &Path, entry: &str) -> Result<Vec<cinch::interface::Iface>, String> {
    check(dir, entry)?;
    build_interfaces(dir)
}

/// [`check_interfaces`] under an explicit target profile.
pub fn check_interfaces_profile(dir: &Path, entry: &str, profile: &TargetProfile) -> Result<Vec<cinch::interface::Iface>, String> {
    drive_with_profile(Mode::Check, StageMode::Refresh, dir, entry, None, profile)?;
    build_interfaces(dir)
}

/// Every `build/*.cinchif` the last check of the project wrote.
fn build_interfaces(dir: &Path) -> Result<Vec<cinch::interface::Iface>, String> {
    let build = dir.join("build");
    let mut interfaces = Vec::new();
    for artifact in std::fs::read_dir(&build).map_err(|error| format!("{}: {error}", build.display()))? {
        let path = artifact.map_err(|error| error.to_string())?.path();
        if path.extension().is_some_and(|extension| extension == "cinchif") {
            let text = std::fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            let interface = cinch::interface::Iface::from_json(&text)
                .map_err(|error| format!("{}: {error}", path.display()))?;
            interfaces.push(interface);
        }
    }
    Ok(interfaces)
}

/// Produce Native library contracts used as input to a Cinch consumer test.
/// This is an explicit fixture producer, independent of the selected frontend;
/// a selected compiler's failure never reaches this path as a fallback.
pub fn check_library_fixture_native(dir: &Path, entry: &str) -> Result<(), String> {
    drive_native(Mode::Check, dir, entry, None)?;
    let artifacts = std::fs::read_dir(dir.join("build")).map_err(|e| e.to_string())?;
    let mut count = 0;
    for artifact in artifacts {
        let path = artifact.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|extension| extension == "cinchif") {
            let module = path.file_stem().and_then(|name| name.to_str()).unwrap_or("");
            if module == "Bootstrap" || module.starts_with("Bootstrap.") {
                return Err(format!("Native library fixture contains compiler implementation module {module}"));
            }
            count += 1;
        }
    }
    if count == 0 { return Err("Native library fixture produced no checked interfaces".into()); }
    Ok(())
}

/// Build `entry` into an executable named `out` inside the project.
pub fn build(dir: &Path, entry: &str, out: &str) -> Result<PathBuf, String> {
    let exe = dir.join(out);
    let code = drive(Mode::Build, dir, entry, Some(exe.clone()))?;
    if code != 0 { return Err(format!("building {entry} returned {code}")); }
    Ok(dir.join(out))
}

/// Reuse only an explicitly supplied, byte-identical Native-built Cinch
/// refinement-checker fixture. The final selected frontend always builds it.
pub fn build_refinement_checker(dir: &Path, out: &str) -> Result<PathBuf, String> {
    let Some(path) = std::env::var_os("CINCH_TEST_REFINEMENT_EMITTER") else { return build(dir, "Main.cinch", out); };
    if external_frontend::configured()?.is_some() { return Err("a cached Native checker cannot replace the selected Cinch frontend".into()); }
    let path = PathBuf::from(path).canonicalize().map_err(|e| e.to_string())?;
    let sources = |root: &Path| -> Result<std::collections::BTreeMap<std::ffi::OsString, Vec<u8>>, String> {
        let mut files = std::collections::BTreeMap::new();
        for item in std::fs::read_dir(root).map_err(|e| e.to_string())? {
            let file = item.map_err(|e| e.to_string())?.path();
            // Runtime-fed source.cinch is written after fixture compilation.
            if file.is_file() && file.extension().is_some_and(|e| e == "cinch") && file.file_name().unwrap() != "source.cinch" {
                files.insert(file.file_name().unwrap().to_owned(), std::fs::read(file).map_err(|e| e.to_string())?);
            }
        }
        Ok(files)
    };
    let entry = std::fs::read(dir.join("Main.cinch")).map_err(|e| e.to_string())?;
    if entry.as_slice() != include_bytes!("../../../bootstrap/programs/Run.SourceRefinements.cinch").as_slice() {
        return Err("refinement fixture has another compiler entry".into());
    }
    if sources(dir)? != sources(path.parent().ok_or("cached checker has no source directory")?)? {
        return Err("cached refinement checker source inputs changed".into());
    }
    Ok(path)
}

/// A file the program wrote in its project, as text.
pub fn read_output(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

pub fn cleanup(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}
