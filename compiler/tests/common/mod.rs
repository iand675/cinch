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

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub mod external_frontend;

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

pub fn project(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    // A Cargo cache shared by copied checkouts can retain env! paths from
    // the previous checkout. Refuse that mixture before compiling a rung.
    let active_root = std::env::var_os("CINCH_TEST_SOURCE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().expect("active package root"));
    assert_source_root(Path::new(env!("CARGO_MANIFEST_DIR")), &active_root);
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_boot_{}_{}_{}", tag, std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.RuntimeIr.cinch" | "Bootstrap.Pipeline.cinch" | "Bootstrap.Object.cinch")) {
        std::fs::write(dir.join("Bootstrap.Object.cinch"), include_str!("../../../bootstrap/Bootstrap.Object.cinch"))
            .expect("checked runtime object dependency");
    }
    std::fs::write(dir.join("Bootstrap.RuntimeAbi.cinch"), include_str!("../../../bootstrap/Bootstrap.RuntimeAbi.cinch"))
        .expect("exact runtime ABI dependency");
    if files.iter().any(|(name, _)| *name == "Bootstrap.Pipeline.cinch") {
        std::fs::write(dir.join("Bootstrap.RuntimeIr.cinch"), include_str!("../../../bootstrap/Bootstrap.RuntimeIr.cinch"))
            .expect("typed pipeline emitter dependency");
    }
    std::fs::write(dir.join("Bootstrap.Index.cinch"), include_str!("../../../bootstrap/Bootstrap.Index.cinch"))
        .expect("shared index syntax dependency");
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Core.cinch" | "Bootstrap.Global.cinch" | "Bootstrap.Typing.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Dictionary.cinch" | "Bootstrap.Instance.cinch")) {
        std::fs::write(dir.join("Bootstrap.BodyContract.cinch"), include_str!("../../../bootstrap/Bootstrap.BodyContract.cinch"))
            .expect("checked body/declaration association dependency");
        std::fs::write(dir.join("Bootstrap.Guarded.cinch"), include_str!("../../../bootstrap/Bootstrap.Guarded.cinch"))
            .expect("checked guarded-body dependency");
    }
    std::fs::write(dir.join("Bootstrap.Total.cinch"), include_str!("../../../bootstrap/Bootstrap.Total.cinch"))
        .expect("checked total-body dependency");
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Core.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Dictionary.cinch")) {
        std::fs::write(dir.join("Bootstrap.Proof.cinch"), include_str!("../../../bootstrap/Bootstrap.Proof.cinch"))
            .expect("shared proof checker dependency");
    }
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Core.cinch" | "Bootstrap.Dictionary.cinch" | "Bootstrap.Infer.cinch" | "Bootstrap.Check.cinch" | "Bootstrap.Load.cinch" | "Bootstrap.Contracts.cinch")) {
        for (name, source) in [
            ("Bootstrap.Nominal.cinch", include_str!("../../../bootstrap/Bootstrap.Nominal.cinch")),
            ("Bootstrap.Ast.cinch", include_str!("../../../bootstrap/Bootstrap.Ast.cinch")),
            ("Bootstrap.Report.cinch", include_str!("../../../bootstrap/Bootstrap.Report.cinch")),
        ] {
            std::fs::write(dir.join(name), source).expect("nominal contract dependency");
        }
    }
    if files.iter().any(|(name, _)| matches!(*name, "Bootstrap.Check.cinch" | "Bootstrap.Infer.cinch")) {
        std::fs::write(dir.join("Bootstrap.Kinds.cinch"), include_str!("../../../bootstrap/Bootstrap.Kinds.cinch")).expect("independent kind checking dependency");
        std::fs::write(dir.join("Bootstrap.Typing.cinch"), include_str!("../../../bootstrap/Bootstrap.Typing.cinch")).expect("independent Core typing dependency");
        std::fs::write(dir.join("Bootstrap.Global.cinch"), include_str!("../../../bootstrap/Bootstrap.Global.cinch")).expect("global reference checking dependency");
        std::fs::write(dir.join("Bootstrap.Instance.cinch"), include_str!("../../../bootstrap/Bootstrap.Instance.cinch")).expect("typed source instance dependency");
        std::fs::write(dir.join("Bootstrap.Builtin.cinch"), include_str!("../../../bootstrap/Bootstrap.Builtin.cinch")).expect("kernel contract dependency");
    }
    if files.iter().any(|(name, _)| *name == "Bootstrap.Check.cinch") {
        for (name, src) in [
            ("Bootstrap.Type.cinch", include_str!("../../../bootstrap/Bootstrap.Type.cinch")),
            ("Bootstrap.Core.cinch", include_str!("../../../bootstrap/Bootstrap.Core.cinch")),
            ("Bootstrap.Dictionary.cinch", include_str!("../../../bootstrap/Bootstrap.Dictionary.cinch")),
            ("Bootstrap.Infer.cinch", include_str!("../../../bootstrap/Bootstrap.Infer.cinch")),
            ("Bootstrap.Parse.cinch", include_str!("../../../bootstrap/Bootstrap.Parse.cinch")),
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
    let path = dir.join(entry);
    let handle = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &path, out.as_deref()))
        .expect("spawn driver thread");
    handle
        .join()
        .expect("driver thread panicked")
        .map_err(|e| e.to_string())
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

/// Build `entry` into an executable named `out` inside the project.
pub fn build(dir: &Path, entry: &str, out: &str) -> Result<PathBuf, String> {
    let exe = dir.join(out);
    let code = drive(Mode::Build, dir, entry, Some(exe.clone()))?;
    if code != 0 { return Err(format!("building {entry} returned {code}")); }
    Ok(dir.join(out))
}

/// A file the program wrote in its project, as text.
pub fn read_output(dir: &Path, name: &str) -> String {
    let path = dir.join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

pub fn cleanup(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}
