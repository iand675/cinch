//! Portable slash-path operations and checked shell patterns against independent fixtures.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Shared runtime consumers keep the CLI validator and selected frontend tests equivalent.
const HELPERS: &str = include_str!("fixtures/path-consumer-helpers.cinchpart");

/// Render exact scalars using only escape forms accepted by the source language.
fn literal(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{{{:x}}}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Use explicit Boolean constructors instead of numeric coercions.
fn flag(value: &Value) -> &'static str {
    if value.as_bool().unwrap() {
        "True"
    } else {
        "False"
    }
}

/// Read fixed independent observations without requiring their SDK during consumer tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/path-vectors.json")).unwrap()
}

/// Keep fixture source batches bounded while executing every public observation at runtime.
fn consumer(checks: &[String], extra: &str) -> String {
    let mut source = format!("{HELPERS}\n{extra}\n");
    let mut names = Vec::new();
    for (index, batch) in checks.chunks(192).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent runtime fixture batch {index}.\n  checks{index} :: Unit -> Bool\n  checks{index} unit = all [\n"));
        source.push_str(
            &batch
                .iter()
                .map(|check| format!("    ({check})"))
                .collect::<Vec<_>>()
                .join(",\n"),
        );
        source.push_str("\n    ]\n");
        names.push(format!("checks{index} ()"));
    }
    source.push_str(&format!("\n  -- | Observe the complete executable fixture result.\n  main :: Int\n  main = if {} then 0 else 1\n", names.join(" && ")));
    source
}

/// Compile the selected frontend, with an explicit Rust-only optimized ownership case.
fn run(checks: Vec<String>, extra: &str, optimized_rust: bool) {
    run_source(&consumer(&checks, extra), optimized_rust);
}

fn run_source(source: &str, optimized_rust: bool) {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Text.FilePath.cinch"),
        include_str!("../prelude/Text/FilePath.cinch"),
    )
    .unwrap();
    std::fs::write(
        directory.path().join("Text.Path.cinch"),
        include_str!("../prelude/Text/Path.cinch"),
    )
    .unwrap();
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, source).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized_rust {
        let (input, output, profile) = (input.clone(), exe.clone(), profile.clone());
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &input,
                    Some(&output),
                    &profile,
                    &cinch::driver::DriverOptions {
                        opt_level: Some(2),
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
    } else {
        common::drive_with_profile(
            Mode::Build,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            Some(exe.clone()),
            &profile,
        )
        .unwrap();
    }
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Cleaning and every lexical reader match independent results at Unicode and SIMD boundaries.
#[test]
fn slash_path_lexical_operations_match_independent_results() {
    let f = fixture();
    let checks = f["lexical"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let mut args = ["input", "clean", "base", "dir", "ext"]
                .map(|key| literal(row[key].as_str().unwrap()))
                .to_vec();
            args.push(flag(&row["absolute"]).into());
            args.push(literal(row["split_dir"].as_str().unwrap()));
            args.push(literal(row["split_base"].as_str().unwrap()));
            format!("lexical-case {}", args.join(" "))
        })
        .collect();
    run(checks, "", false);
}

/// Joins ignore empty elements, preserve relative prefixes and clean once after concatenation.
#[test]
fn slash_path_joins_match_independent_empty_root_and_parent_cases() {
    let f = fixture();
    let checks = f["joins"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let args = row["elements"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| literal(value.as_str().unwrap()))
                .collect::<Vec<_>>()
                .join(",");
            format!(
                "join-case [{args}] {}",
                literal(row["output"].as_str().unwrap())
            )
        })
        .collect();
    run(checks, "", false);
}

/// Compiled and convenience globs match independent Unicode, class, escape and malformed-suffix cases.
#[test]
fn slash_path_patterns_match_independent_results_and_error_precedence() {
    let f = fixture();
    let checks = f["globs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "glob-case {} {} {} {}",
                literal(row["pattern"].as_str().unwrap()),
                literal(row["name"].as_str().unwrap()),
                flag(&row["matched"]),
                flag(&row["error"])
            )
        })
        .collect();
    run(checks, "", false);
}

/// Temporary source owners and captured compiled literals survive optimized raw scans and word comparison.
#[test]
fn rust_slash_path_optimized_borrowed_owners_survive_repeated_calls() {
    let extra = include_str!("fixtures/path-owner-consumer.cinchpart");
    run(vec!["owner-case ()".into()], extra, true);
}

/// Actual 16-byte SIMD slash-path scanning and glob paths remain legal for 32-bit and non-x86 object targets.
#[test]
fn slash_path_scan_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..Default::default()
        })
        .unwrap_or_else(|_| {
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        std::fs::write(
            project.join("Text.Path.cinch"),
            include_str!("../prelude/Text/Path.cinch"),
        )
        .unwrap();
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public slash-path scanning and glob paths for foreign targets.\nmodule Main where\n  import Text.Path\n  import Data.Bytes\n  import Data.Nat\n  import Data.Option\n  import Data.Result\n  import Data.String\n\n  -- | Exercise public 16-byte SIMD forward, backward and metacharacter scans across targets.\n  main :: Int\n  main =\n    let raw = case string-from-bytes (bytes-new 32) of Some s -> s; None -> \"/a/b/c.txt\" in\n    let cleaned = path-clean raw in\n    let base = path-base cleaned in\n    let ext = path-ext base in\n    case path-match \"*.txt\" ext of\n      Ok matched -> if matched || bytes-len (string-to-bytes cleaned) == 32 then 0 else 1\n      Err err -> 1\n",
        )
        .unwrap();
        let entry = project.join("Main.cinch");
        let path = project.join(format!("{arch}.ll"));
        let ir_path = path.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::EmitIr,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&ir_path),
                    &profile,
                    &cinch::driver::DriverOptions::default(),
                )
                .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(
            ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>") && ir.contains("load i128")
        );
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Every new declaration has canonical module-owned documentation without reference-language prose.
#[test]
fn slash_path_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Path.cinch");
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Text.Path"), "Text.Path", None, false).unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

#[test]
fn posix_file_paths_preserve_relative_and_search_list_boundaries() {
    let source = include_str!("fixtures/filepath-consumer.cinch");
    run_source(source, false);
    run_source(source, true);
}
