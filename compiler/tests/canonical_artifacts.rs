//! The compiler's JSON artifacts are in the one canonical form `Text.Json`
//! prints (sorted keys, no insignificant whitespace, the fixed escape
//! table), so a Cinch program reproduces their bytes exactly: the
//! bootstrap ladder's `canonical-bytes (parse-json artifact) == artifact`.
//!
//! A real project is checked through the driver, then a compiled Cinch
//! program reads the artifacts back through `Effect.Fs` and compares each
//! file's bytes against `canonical-bytes` of its parse. The Rust writer and
//! the Cinch printer are pinned against each other on real artifacts, not
//! on hand-written samples.
//!
//! The Cinch side reads a bounded sample: the user modules' interfaces, one
//! prelude interface with instances and foreign rows, and the package
//! store's entry manifest. The build manifest (`Main.manifest.json`, about
//! 230 KB of prelude rows) is left to the Rust-side checks below because a
//! compiled program's footprint is its total allocation today (nothing is
//! reclaimed), and that parse would cost gigabytes.

mod common;

use cinch::interface::Iface;
use std::path::{Path, PathBuf};

const LIB: &str = "module Lib where\n\
  \x20 export Token, Tok, describe\n\
  \x20 data Token where\n\
  \x20   Tok :: I64 -> Token\n\
  \x20 describe :: Token -> I64\n\
  \x20 describe t = case t of\n\
  \x20   Tok n -> n\n";

const MAIN: &str = "module Main where\n\
  \x20 import Lib\n\
  \x20 main :: Int\n\
  \x20 main = if Lib.describe (Lib.Tok 7) == 7 then 0 else 1\n";

/// Reads each listed artifact and exits 0 when every one is canonical,
/// else the 1-based index of the first that is not.
const LAW: &str = r#"module Main where
  import Text.Json (parse-json, canonical-bytes)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Data.Bytes
  import Data.List
  import Data.Path
  import Data.Result
  import Effect.Fs

  artifacts :: List String
  artifacts = __ARTIFACTS__

  is-canonical :: forall r. using (io :: Io r) => String ->{IO r} Bool
  is-canonical p = case fs-read (path-from-string p) of
    Err e -> False
    Ok b -> case parse-json p b of
      Failed x -> False
      Parsed v soft -> canonical-bytes v == b

  first-mismatch :: forall r. using (io :: Io r) => List String -> Int ->{IO r} Int
  first-mismatch ps i = case ps of
    Nil -> 0
    Cons p rest -> case is-canonical p of
      True -> first-mismatch rest (i + 1)
      False -> i

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in first-mismatch artifacts 1
"#;

/// A path as a Cinch string literal.
fn lit(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn files_named(dir: &Path, name: &str, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).expect("read dir") {
        let p = e.expect("entry").path();
        if p.is_dir() {
            files_named(&p, name, out);
        } else if p.file_name().is_some_and(|n| n == name) {
            out.push(p);
        }
    }
}

#[test]
fn written_artifacts_are_canonical_json_for_the_cinch_printer() {
    let dir = common::project("canonical-src", &[("Lib.cinch", LIB), ("Main.cinch", MAIN)]);
    // The package store writes beside the build, not into the user's
    // cache. This binary holds one test, so the variable races nothing.
    let store = dir.join("store");
    std::env::set_var("CINCH_STORE", &store);
    common::check(&dir, "Main.cinch").expect("the project checks");
    let build = dir.join("build");

    // Rust side, every interface the build wrote: one line, a fixed point
    // of the reader and writer.
    let mut ifaces = 0;
    for e in std::fs::read_dir(&build).expect("build dir") {
        let p = e.expect("entry").path();
        if p.extension().is_some_and(|x| x == "cinchif") {
            let text = std::fs::read_to_string(&p).expect("read interface");
            assert!(!text.contains('\n'), "{} is one line", p.display());
            let back = Iface::from_json(&text).expect("the interface parses");
            assert_eq!(back.to_json(), text, "{} is a fixed point", p.display());
            ifaces += 1;
        }
    }
    assert!(ifaces > 2, "the build wrote the prelude's interfaces too");
    let manifest = std::fs::read_to_string(build.join("Main.manifest.json")).expect("manifest");
    assert!(
        manifest.starts_with("{\"assumptions\":{") && !manifest.contains('\n'),
        "the build manifest is compact with sorted keys"
    );

    // Cinch side, a bounded sample (see the module docs).
    let mut sample = vec![
        build.join("Lib.cinchif"),
        build.join("Main.cinchif"),
        build.join("Algebra.Monoid.cinchif"),
    ];
    let mut entries = Vec::new();
    files_named(&store, "store.json", &mut entries);
    assert_eq!(entries.len(), 1, "one store entry: {entries:?}");
    sample.extend(entries);
    for p in &sample {
        assert!(p.is_file(), "missing artifact {}", p.display());
    }
    let lib = std::fs::read_to_string(&sample[0]).expect("Lib interface");
    // A control the checker must refuse: a canonical document plus a
    // trailing newline. Listed last, so the exit code is its index exactly
    // when every real artifact before it passed.
    let control = dir.join("control.json");
    std::fs::write(&control, format!("{lib}\n")).expect("control");
    sample.push(control);

    let list = sample
        .iter()
        .rev()
        .fold("Nil".to_string(), |acc, p| format!("(Cons {} {acc})", lit(p)));
    let law_dir = common::project("canonical-law", &[("Main.cinch", &LAW.replace("__ARTIFACTS__", &list))]);
    let code = common::run(&law_dir, "Main.cinch");
    assert_eq!(
        code as usize,
        sample.len(),
        "exit {code}: 0 means the control passed, a smaller index names the first non-canonical artifact in {sample:?}"
    );
    common::cleanup(&law_dir);
    common::cleanup(&dir);
}
