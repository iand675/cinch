//! `parMap` inside the compiler's own source set.
//!
//! `parMap` lowers through `Data.Array`'s `array-from-list` and
//! `array-to-list` (`Bootstrap.Infer.par-map-array-call`), and `Data.Array` is
//! not in the embedded base package. `tooling/bootstrap.py`'s
//! `collect_compiler_sources` therefore bundles it under `COMPILER_PRELUDE`
//! beside the `Text` toolkit. Without that, a `parMap` anywhere in the
//! compiler fails at resolve with "missing source module or checked
//! interface: Data.Array", which is the gap this test holds shut.
//!
//! The source set is read from disk at run time, the way
//! `bootstrap_ladder.rs` reads the ladder, so this cannot hold a stale copy of
//! a module another stream is editing. The probe is a `parMap` over a
//! compiler-shaped value: `Bootstrap.Export`'s `List (String, Json)` row list,
//! which is the exact type the survey named.

mod common;

use std::path::{Path, PathBuf};

fn bootstrap_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../bootstrap")
}

fn prelude_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("prelude")
}

/// Every top-level `bootstrap/*.cinch` module plus the entry, as the bundle
/// `collect_compiler_sources` assembles. `Data.Array` is included, because
/// that is the change under test; the negative control below removes it.
fn compiler_sources(entry: &str, with_array: bool) -> Vec<(String, String)> {
    let dir = bootstrap_dir();
    let mut out: Vec<(String, String)> = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
        let path = entry.expect("directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("cinch") {
            continue;
        }
        let name = path.file_name().expect("file name").to_string_lossy().to_string();
        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
        out.push((name, src));
    }
    for (name, src) in common::TOOLKIT {
        out.push(((*name).to_string(), (*src).to_string()));
    }
    if with_array {
        let path = prelude_dir().join("Data/Array.cinch");
        out.push((
            "Data.Array.cinch".to_string(),
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display())),
        ));
    }
    // The entry imports every bundled module under a qualified alias, so the
    // probe module's exports stay out of the entry's own scope. This is the
    // shape `collect_compiler_sources` produces.
    let header = "module Main where\n";
    let mut modules: Vec<String> = out
        .iter()
        .map(|(name, _)| name.trim_end_matches(".cinch").to_string())
        .collect();
    modules.sort();
    let imports = modules
        .iter()
        .enumerate()
        .map(|(i, module)| format!("  import {module} as CompilerSource{i}\n"))
        .collect::<String>();
    out.push((
        "Main.cinch".to_string(),
        entry.replacen(header, &format!("{header}{imports}"), 1),
    ));
    out
}

/// The probe: a `parMap` over `List (String, Json)` in a module of the
/// compiler, with `Data.Array` imported so the lowering's two conversions
/// resolve by their bare names, as `Bootstrap.Infer` calls them.
const PROBE: &str = r#"
  probe :: List (String, Json) -> List (String, Json)
  probe rows = parMap (\row -> row) rows
"#;

fn with_probe(sources: Vec<(String, String)>) -> Vec<(String, String)> {
    sources
        .into_iter()
        .map(|(name, src)| {
            if name != "Bootstrap.Manifest.cinch" {
                return (name, src);
            }
            // `Bootstrap.Manifest` is a leaf of the ladder's data flow, so a
            // probe here cannot perturb another module's checking.
            let src = src.replacen("  import Data.Bool\n", "  import Data.Array\n  import Data.Bool\n", 1);
            let src = src.replacen(
                "  export manifest-field, manifest-items\n",
                &format!("{PROBE}\n  export manifest-field, manifest-items, probe\n"),
                1,
            );
            assert!(src.contains("probe rows"), "the probe did not land in Bootstrap.Manifest");
            assert!(src.contains("import Data.Array"), "the Data.Array import did not land");
            (name, src)
        })
        .collect()
}

fn check_project(tag: &str, sources: &[(String, String)]) -> Result<(), String> {
    let refs: Vec<(&str, &str)> = sources
        .iter()
        .map(|(name, src)| (name.as_str(), src.as_str()))
        .collect();
    let dir = common::project(tag, &refs);
    let outcome = common::check(&dir, "Main.cinch");
    common::cleanup(&dir);
    outcome
}

/// The acceptance case: a `parMap` over a compiler-shaped type compiles
/// inside the compiler's own source set.
#[test]
fn par_map_over_a_compiler_row_list_compiles_in_the_compiler_source_set() {
    let entry = std::fs::read_to_string(bootstrap_dir().join("programs/Run.Pipeline.cinch"))
        .expect("read the compiler entry");
    let sources = with_probe(compiler_sources(&entry, true));
    let modules = sources.len();
    check_project("par-map-compiler-sources", &sources).unwrap_or_else(|error| {
        panic!("a parMap inside the compiler's own source set was refused ({modules} modules): {error}")
    });
}

/// The negative control: without `Data.Array` in the set the same probe fails,
/// and it fails on the missing module rather than on anything else. This is
/// what makes the test above a witness rather than a coincidence.
///
/// The refusal is the resolver's, because the probe's own `import Data.Array`
/// is the first thing that misses. With the import removed the failure moves
/// to the lowering itself ("the prelude's Data.Array.array-from-list is
/// unavailable (parMap lowers through it)"), which is the same gate one layer
/// further in; that message is recorded in the lane's report rather than
/// asserted here, because keeping the import is what makes this control differ
/// from the positive case in exactly one respect.
#[test]
fn the_same_par_map_without_data_array_names_the_missing_module() {
    let entry = std::fs::read_to_string(bootstrap_dir().join("programs/Run.Pipeline.cinch"))
        .expect("read the compiler entry");
    let sources = with_probe(compiler_sources(&entry, false));
    let message = match check_project("par-map-no-array", &sources) {
        Ok(()) => panic!("the probe compiled without Data.Array in the source set"),
        Err(message) => message,
    };
    assert!(
        message.contains("missing source module or checked interface: Data.Array"),
        "the refusal does not name the missing module: {message:?}"
    );
}
