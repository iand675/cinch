//! Rung 2 of the bootstrap ladder: canonical manifests.
//!
//! The rung is `bootstrap/Bootstrap.Manifest.cinch` driven by
//! `bootstrap/programs/Run.Manifest.cinch`, a real program built and run
//! through `cinch::driver`. The program reads a directory of interface
//! artifacts plus `compiler/prelude/cinch-base.cinchpkg`, prints one line per
//! manifest, and exits 0 when every check holds; the test asserts the observed
//! lines against values read from those files and the package facts
//! hand-listed below.
//!
//! What the program answers, all as Cinch code over the parsed document:
//!
//! - `driver-round-trip`: 0 when the canonical form of the artifact's bytes
//!   equals those bytes. This is the rung's primary claim: a real artifact's
//!   bytes ARE its canonical form.
//! - `canonical-round-trip`: the same code applied to the canonical form,
//!   i.e. canonical output is a fixed point of parse then print.
//! - `reorder-same`, `added-key-differs`: the program reorders every object's
//!   entries, and adds a top-level key, in the parsed value and compares the
//!   canonical bytes. The first must not change them, the second must.
//! - `reorder-visible`: the printed (non-canonical) forms differ after the
//!   reorder, so the invariance check is not vacuous.
//! - `hash-stable`: the SHA-256 of the canonical bytes is unchanged by the
//!   reorder and changed by the addition.
//! - package facts: module count, facade module, the facade's 0-based index
//!   in `order`, and whether `order` is a permutation of the module list.
//!
//! Every artifact gets every column, in its own process: one claim each, and
//! the simplest memory bound while the runtime does not yet free memory (the
//! harness caps a spawned program's heap on a run's TOTAL allocation,
//! `compiler/.cargo/config.toml`, `CINCH_HEAP_LIMIT`). Every manifest fits
//! the cap on its own; the last that did not (`Data.Machine.cinchif`, 7.4 GB)
//! fits at 690 MB since small `Nat`/`Int` immediates. No line ever reports a
//! narrowed claim.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;

const MANIFEST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Manifest.cinch");
const RUN_SRC: &str = include_str!("../../bootstrap/programs/Run.Manifest.cinch");
const PACKAGE_SRC: &str = include_str!("../../compiler/prelude/cinch-base.cinchpkg");

const ARTIFACT_DIR_PLACEHOLDER: &str = "__MANIFEST_ARTIFACT_DIR__";
const PACKAGE_PATH_PLACEHOLDER: &str = "__MANIFEST_PACKAGE_PATH__";

/// The package facts hand-listed from `compiler/prelude/cinch-base.cinchpkg`:
/// 55 entries in `modules` and in `order`, `Prelude` is the facade and sits
/// at index 33 of `order`, and `order` is a permutation of the module names.
const PACKAGE_MODULES: &str = "55";
const PACKAGE_FACADE: &str = "Prelude";
const PACKAGE_FACADE_INDEX: &str = "33";

/// A module with a data type, an instance and a foreign import, so the
/// artifacts carry more than an empty export table.
const LIB_SRC: &str = "module Lib where\n\
  \x20 export Token, Tok, describe, peek\n\
  \x20 data Token where\n\
  \x20   Tok :: I64 -> Token\n\
  \x20 describe :: Token -> I64\n\
  \x20 describe t = case t of\n\
  \x20   Tok n -> n\n\
  \x20 peek :: Token -> Token\n\
  \x20 peek t = case t of\n\
  \x20   Tok n -> Tok n\n\
  \x20 foreign import ccall \"cinch.prim.eqI64\" primEqI64 :: I64 -> I64 -> Bool\n\
  \x20 implicit instance Eq_Token :: Eq Token where\n\
  \x20   (==) a b = case a of\n\
  \x20     Tok x -> case b of\n\
  \x20       Tok y -> primEqI64 x y\n\
  \x20   (/=) a b = case a of\n\
  \x20     Tok x -> case b of\n\
  \x20       Tok y -> not (primEqI64 x y)\n";

const APP_SRC: &str = "module Main where\n\
  \x20 import Lib\n\
  \x20 main :: Int\n\
  \x20 main = if Lib.describe (Lib.Tok 7) == 7 then 0 else 1\n";

/// A whitespace-delimited field of one report line (`key=value`).
fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    line.split_whitespace()
        .find_map(|t| t.strip_prefix(&prefix))
}

fn expect_field(line: &str, key: &str, want: &str, what: &str) {
    assert_eq!(
        field(line, key),
        Some(want),
        "{what}: expected {key}={want} in {line:?}"
    );
}

/// Every `build/*.cinchif` a compile produced, sorted by name.
fn read_artifacts(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let build = dir.join("build");
    let mut out: Vec<(String, Vec<u8>)> = std::fs::read_dir(&build)
        .unwrap_or_else(|e| panic!("read {}: {e}", build.display()))
        .map(|entry| entry.expect("build entry").path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("cinchif"))
        .map(|p| {
            let name = p
                .file_name()
                .expect("artifact name")
                .to_string_lossy()
                .to_string();
            let bytes = std::fs::read(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
            (name, bytes)
        })
        .collect();
    out.sort();
    out
}

/// Run the rung program once over the whole artifact directory; the exit code
/// names the first failed claim, and the `CINCH_ALLOC_STATS=1` line on stderr
/// reports the run's total allocation (the runtime frees nothing, so that is
/// the number a heap cap acts on).
fn run_program(exe: &Path) -> (i32, String, String) {
    let out = Command::new(exe)
        .env("CINCH_ALLOC_STATS", "1")
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()));
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Empty a directory so exactly one manifest is in it.
fn clear_dir(dir: &Path) {
    for entry in std::fs::read_dir(dir).expect("read manifests directory") {
        let path = entry.expect("manifests entry").path();
        std::fs::remove_file(&path).unwrap_or_else(|e| panic!("clear {}: {e}", path.display()));
    }
}

/// The report line for one named manifest in a run's output.
fn report_line<'a>(stdout: &'a str, prefix: &str, name: &str) -> Option<&'a str> {
    stdout
        .lines()
        .find(|line| line.starts_with(prefix) && line.split_whitespace().nth(1) == Some(name))
}

/// The rung's full column set, asserted for one manifest's report line.
fn expect_full_columns(line: &str, what: &str) {
    assert_eq!(
        field(line, "parse"),
        Some("yes"),
        "{what} must parse: {line:?}"
    );
    // The rung's primary claim: the artifact's bytes ARE its canonical form,
    // so canonicalizing it is a byte-identical round trip.
    expect_field(line, "driver-round-trip", "0", what);
    // And the canonical form is a fixed point, order-free in a way the
    // document's own modifications demonstrate is not vacuous.
    expect_field(line, "canonical-round-trip", "0", what);
    expect_field(line, "reorder-same", "yes", what);
    expect_field(line, "added-key-differs", "yes", what);
    expect_field(line, "reorder-visible", "yes", what);
    expect_field(line, "hash-stable", "yes", what);
}

/// The package facts, answered from the real `cinch-base.cinchpkg` in one
/// run's output, plus the program's own sample package.
fn expect_package_facts(stdout: &str) {
    let package = report_line(stdout, "package ", "cinch-base.cinchpkg")
        .unwrap_or_else(|| panic!("no cinch-base.cinchpkg line in:\n{stdout}"));
    expect_field(package, "parse", "yes", "cinch-base.cinchpkg");
    expect_field(package, "modules", PACKAGE_MODULES, "cinch-base.cinchpkg");
    expect_field(package, "facade", PACKAGE_FACADE, "cinch-base.cinchpkg");
    expect_field(
        package,
        "facade-index",
        PACKAGE_FACADE_INDEX,
        "cinch-base.cinchpkg",
    );
    expect_field(package, "order-permutation", "yes", "cinch-base.cinchpkg");

    let sample = report_line(stdout, "package ", "sample-package.cinchpkg")
        .unwrap_or_else(|| panic!("no sample package line in:\n{stdout}"));
    expect_field(sample, "parse", "yes", "sample package");
    expect_field(sample, "modules", "2", "sample package");
    expect_field(sample, "facade", "Prelude", "sample package");
    expect_field(sample, "facade-index", "1", "sample package");
    expect_field(sample, "order-permutation", "yes", "sample package");
}

/// One rung project: a two-module compile whose artifacts the test reads, and
/// the rung's own project with the run program built and an empty
/// `manifests/` directory for one batch at a time.
fn fixture(tag: &str) -> (Vec<(String, Vec<u8>)>, PathBuf, PathBuf, PathBuf, PathBuf) {
    let src_dir = common::project(
        &format!("manifest_src_{tag}"),
        &[("Lib.cinch", LIB_SRC), ("Main.cinch", APP_SRC)],
    );
    common::check(&src_dir, "Main.cinch").expect("the manifest project must check");
    let artifacts = read_artifacts(&src_dir);
    assert!(
        artifacts.len() > 2,
        "the compile produced {} artifacts; expected the project's modules plus the prelude family",
        artifacts.len()
    );

    assert!(
        RUN_SRC.contains(ARTIFACT_DIR_PLACEHOLDER) && RUN_SRC.contains(PACKAGE_PATH_PLACEHOLDER),
        "both input placeholders must be present in Run.Manifest.cinch"
    );
    let run_dir = common::project(
        &format!("manifest_run_{tag}"),
        &[
            ("Bootstrap.Manifest.cinch", MANIFEST_SRC),
            ("cinch-base.cinchpkg", PACKAGE_SRC),
        ],
    );
    let manifests = run_dir.join("manifests");
    std::fs::create_dir_all(&manifests).expect("manifests directory");
    let artifact_dir = manifests
        .to_str()
        .expect("artifact directory path")
        .to_string();
    let package_path = run_dir
        .join("cinch-base.cinchpkg")
        .to_str()
        .expect("package path")
        .to_string();
    let entry = RUN_SRC
        .replace(ARTIFACT_DIR_PLACEHOLDER, &format!("\"{artifact_dir}\""))
        .replace(PACKAGE_PATH_PLACEHOLDER, &format!("\"{package_path}\""));
    std::fs::write(run_dir.join("Main.cinch"), entry).expect("write the run program");

    let exe: PathBuf = common::build(&run_dir, "Main.cinch", "manifest_probe")
        .expect("the run program must build");
    (artifacts, manifests, exe, run_dir, src_dir)
}

fn exit_message(code: i32, alloc: &str, stdout: &str) -> String {
    format!(
        "the rung program must pass every check; bit {code} (1 unparseable, 2 not a fixed \
         point, 3 reorder changed canonical bytes, 4 added key did not, 5 package facts, \
         6 hash not stable, 7 unreadable, 9 bytes are not canonical, 73 heap limit; the \
         runtime frees nothing, so the cap acts on this run's TOTAL allocation)\n\
         allocation: {alloc}\nfull output:\n{stdout}"
    )
}

#[test]
fn manifest_round_trips_and_answers_package_facts() {
    let (artifacts, manifests, exe, run_dir, src_dir) = fixture("main");

    let mut package_checked = false;
    for (name, bytes) in &artifacts {
        clear_dir(&manifests);
        std::fs::write(manifests.join(name), bytes).expect("copy artifact");
        let (code, stdout, alloc) = run_program(&exe);

        assert_eq!(code, 0, "{name}: {}", exit_message(code, &alloc, &stdout));
        assert!(
            !stdout.contains("skipped"),
            "{name}: no manifest may be reported with a narrowed claim:\n{stdout}"
        );
        let line = report_line(&stdout, "artifact ", name)
            .unwrap_or_else(|| panic!("{name}: no artifact line in:\n{stdout}"));
        expect_full_columns(line, name);

        // The program's own sample: a canonical small document, so the 0 side
        // of the byte-identity check has a subject whose canonical form is
        // known by inspection, independent of the compiler's writer.
        let sample = report_line(&stdout, "artifact ", "sample-artifact.cinchif")
            .unwrap_or_else(|| panic!("{name}: no sample artifact line in:\n{stdout}"));
        expect_full_columns(sample, "sample artifact");

        if !package_checked {
            expect_package_facts(&stdout);
            package_checked = true;
        }
    }
    assert!(package_checked, "at least one manifest must have run");

    common::cleanup(&run_dir);
    common::cleanup(&src_dir);
}
