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
pub fn project(tag: &str, files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_boot_{}_{}_{}", tag, std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    for (name, src) in TOOLKIT.iter().chain(files.iter()) {
        std::fs::write(dir.join(name), src).unwrap_or_else(|e| panic!("write {name}: {e}"));
    }
    dir
}

/// The driver call on a thread with a large stack: the checker recurses
/// over the elaborated program.
pub fn drive(mode: Mode, dir: &Path, entry: &str, out: Option<PathBuf>) -> Result<i32, String> {
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
    let path = dir.join(entry);
    let handle = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Build, &path, Some(&exe)))
        .expect("spawn driver thread");
    handle
        .join()
        .expect("driver thread panicked")
        .map_err(|e| e.to_string())?;
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
