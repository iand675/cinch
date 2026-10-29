//! The ladder as one project: every artifact in `bootstrap/` type-checks when
//! assembled with the toolkit the package does not embed, and they can all be
//! imported together (no ladder module collides with another).
//!
//! The rung tests exercise behaviour; this one holds the property that keeps
//! them composable, and it is deliberately directory-driven: the ladder grows
//! by adding files, so a hand-listed inventory would go stale the moment a
//! stream landed one. `bootstrap/programs/` is excluded because every program
//! there declares `module Main`, and a project holds one entry.

mod common;

use std::path::{Path, PathBuf};

fn bootstrap_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../bootstrap")
}

/// The top-level `bootstrap/*.cinch` modules as (file name, source, module
/// name), read at run time so the test cannot hold a stale copy of a file a
/// stream is editing.
fn ladder_modules() -> Vec<(String, String, String)> {
    let dir = bootstrap_dir();
    let mut out: Vec<(String, String, String)> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path: PathBuf = entry.expect("directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("cinch") {
            continue;
        }
        let name = path.file_name().expect("file name").to_string_lossy().to_string();
        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
        let module = src
            .lines()
            .find_map(|l| l.strip_prefix("module "))
            .and_then(|rest| rest.split(char::is_whitespace).next())
            .unwrap_or_else(|| panic!("{name} declares no module"))
            .to_string();
        out.push((name, src, module));
    }
    out.sort();
    out
}

#[test]
fn every_ladder_module_checks_together() {
    let modules = ladder_modules();
    assert!(
        modules.len() >= 2,
        "the ladder has lost its shared contract modules: {:?}",
        modules.iter().map(|(n, _, _)| n).collect::<Vec<_>>()
    );

    // One entry importing every ladder module: a single run then proves they
    // type-check AND that importing them side by side resolves, which a
    // per-module check would not prove.
    let mut entry = String::from("module Main where\n");
    for (_, _, module) in &modules {
        entry.push_str("  import ");
        entry.push_str(module);
        entry.push('\n');
    }
    entry.push_str("\n  main :: Int\n  main = 0\n");

    let refs: Vec<(&str, &str)> = modules
        .iter()
        .map(|(name, src, _)| (name.as_str(), src.as_str()))
        .collect();
    let dir = common::project("ladder", &refs);
    std::fs::write(dir.join("Main.cinch"), &entry).expect("write the entry");
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 0,
        "the ladder does not check as one project ({} modules)",
        modules.len()
    );
}
