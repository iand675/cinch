//! Complete byte splitting, persistent Producer traversal and explicit shared pull cursors.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// The actual source module supplies concrete state transitions and cursor storage.
const SPLIT: &str = include_str!("../prelude/Data/Bytes/Split.cinch");
/// Independent fingerprints consume every returned byte and fragment boundary.
const HELPERS: &str = include_str!("fixtures/byte-split-consumer.cinchpart");

/// Select independent bounded groups without raising checked-RC quarantine limits.
fn consumer(checks: &[String], width: usize) -> String {
    let mut source = HELPERS.to_owned();
    let mut dispatch = "False".to_owned();
    for (index, group) in checks.chunks(width).enumerate().rev() {
        source.push_str(&format!("\n  -- | Observe complete independent split group {index}.\n  group{index} :: Unit -> Bool\n  group{index} unit = all [\n{}\n    ]\n", group.iter().map(|value| format!("    ({value})")).collect::<Vec<_>>().join(",\n")));
        dispatch = format!("if index == \"{index}\" then group{index} () else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select a bounded complete output and lifecycle observation.\n  selected :: String -> Bool\n  selected index = {dispatch}\n  -- | Run the selected independent split group with runtime deep-input sizes.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index (Cons size Nil)) -> if index == \"deep\" then (if deep-case (decimal size) then 0 else 1) else 2\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n"));
    source
}

/// Compile the isolated source closure and verify ordinary and poisoned owner reclamation.
fn run_with_source(
    checks: Vec<String>,
    width: usize,
    optimized: bool,
    depth: Option<usize>,
    split_source: &str,
) {
    let directory = tempfile::tempdir().unwrap();
    for (name, text) in [
        ("Data.Bytes.Split", split_source),
        (
            "Data.Bytes.View",
            include_str!("../prelude/Data/Bytes/View.cinch"),
        ),
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), text).unwrap();
    }
    let entry = directory.path().join("Main.cinch");
    let exe = directory.path().join("consumer");
    std::fs::write(&entry, consumer(&checks, width)).unwrap();
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized {
        let output = exe.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &entry,
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
        for index in 0..checks.len().div_ceil(width) {
            let mut command = std::process::Command::new(&exe);
            if let Some(count) = depth {
                command.args(["deep".to_owned(), count.to_string()]);
            } else {
                command.arg(index.to_string());
            }
            let output = command
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "index={index}, debug={debug}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
        }
    }
}

/// Preserve the maintained public source for ordinary matrix, owner and deep-size observations.
fn run_with_depth(checks: Vec<String>, width: usize, optimized: bool, depth: Option<usize>) {
    run_with_source(checks, width, optimized, depth, SPLIT);
}

/// Check ordinary reference and ownership groups without a runtime deep-size argument.
fn run(checks: Vec<String>, width: usize, optimized: bool) {
    run_with_depth(checks, width, optimized, None);
}

/// Every actual output byte and boundary agrees with the independent pinned reference fixtures.
#[test]
fn split_lists_producers_and_cursors_match_independent_complete_outputs() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-split-vectors.json")).unwrap();
    let checks = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "split-case {} {} {} {} {} {} {} {} {}",
                row["input"],
                row["separator"],
                row["start"],
                row["end"],
                row["limit"].as_i64().unwrap().max(-1) as i128 + 1,
                if row["after"].as_bool().unwrap() {
                    "True"
                } else {
                    "False"
                },
                if row["lines"].as_bool().unwrap() {
                    "True"
                } else {
                    "False"
                },
                row["hash"],
                row["count"]
            )
        })
        .collect();
    run(checks, 16, false);
}

/// Live alias mutations, stable compiled patterns, shared cursors and replayable Producers remain distinct.
#[test]
fn split_owners_mutations_and_pull_lifecycle_are_preserved() {
    let checks: Vec<String> = [
        "lifecycle-case",
        "mutation-case",
        "multibyte-mutation-case",
        "aliased-separator-case",
        "yielded-separator-case",
        "empty-mutation-case",
        "lines-mutation-case",
        "temporary-owner-case",
        "counter-boundary-case",
    ]
    .into_iter()
    .map(|name| format!("{name} ()"))
    .collect();
    run(checks.clone(), 1, false);
    run(checks, 1, true);
}

/// Deep fragment lists and complete producer folds fit the unchanged compiled-program stack limit.
#[test]
fn split_deep_construction_uses_bounded_stack_at_o0_and_o2() {
    run_with_depth(vec!["True".into()], 1, false, Some(16384));
    run_with_depth(vec!["True".into()], 1, true, Some(16384));
}

/// An explicit test-only export observes private field boundaries without exposing mutable storage publicly.
#[test]
fn split_private_progress_preserves_width_sentinel_and_guard_invariants() {
    let source = format!(
        "{SPLIT}{}",
        include_str!("fixtures/byte-split-private-progress.cinchpart")
    );
    run_with_source(
        vec!["byte-split-test-progress ()".into()],
        1,
        false,
        None,
        &source,
    );
    run_with_source(
        vec!["byte-split-test-progress ()".into()],
        1,
        true,
        None,
        &source,
    );
}

/// Every declaration, including private cursor and state operations, has owned canonical documentation.
#[test]
fn split_declarations_have_canonical_owned_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        SPLIT,
        Some("Data.Bytes.Split"),
        "Data.Bytes.Split",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    assert!(declarations.len() >= 40);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing docs: {}", declaration.name));
        assert!(!doc.legacy && !doc.markdown.trim().is_empty());
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
    assert!(!SPLIT.contains("unsafe ") && !SPLIT.contains("bytes-data"));
}

/// Inspect only a named complete definition so unrelated runtime header loads cannot satisfy coverage.
fn split_definition<'a>(ir: &'a str, symbol: &str) -> &'a str {
    let marker = format!("@\"{symbol}\"(");
    let mut offset = 0;
    for line in ir.split_inclusive('\n') {
        if line.starts_with("define ") && line.contains(&marker) {
            let end = offset + ir[offset..].find("\n}").unwrap() + 2;
            return &ir[offset..end];
        }
        offset += line.len();
    }
    panic!("missing actual function definition {symbol}");
}

/// Follow emitted direct calls while allowing the compiler to inline an intermediate adapter.
fn split_call_path(ir: &str, start: &str, wanted: &str) -> Option<Vec<String>> {
    let mut pending = vec![vec![start.to_owned()]];
    let mut seen = std::collections::BTreeSet::new();
    while let Some(path) = pending.pop() {
        let symbol = path.last().unwrap();
        if symbol == wanted {
            return Some(path);
        }
        if !seen.insert(symbol.clone()) {
            continue;
        }
        for line in split_definition(ir, symbol)
            .lines()
            .filter(|line| line.contains("call "))
        {
            for suffix in line.split("@\"").skip(1) {
                let Some((callee, after)) = suffix.split_once('"') else {
                    continue;
                };
                if !after.starts_with('(') {
                    continue;
                }
                if callee.starts_with("Data.Bytes.View.")
                    || callee.starts_with("Data.Bytes.Split.")
                    || callee.starts_with("Main.")
                {
                    let mut next = path.clone();
                    next.push(callee.to_owned());
                    pending.push(next);
                }
            }
        }
    }
    None
}

/// Complete public split pipelines retain their actual bounded packed and scalar searches in foreign objects.
#[test]
fn split_full_public_pipelines_compile_for_wasm_arm_and_big_endian() {
    let source = [
        include_str!("../prelude/Data/Bytes/Word.cinch"),
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        include_str!("../prelude/Data/Bytes/View.cinch"),
        SPLIT,
        include_str!("fixtures/byte-split-portable.cinch"),
    ]
    .join("\n");
    // Retain all target IR even when a subsequent graph assertion or object build fails.
    let directory = tempfile::tempdir().unwrap().keep();
    let mut artifacts = Vec::new();
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
        let text = source.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&text, "byte-split-full.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        let file = directory.join(format!("{arch}.ll"));
        std::fs::write(&file, &ir).unwrap();
        artifacts.push((triple, file, ir));
    }
    for (triple, file, ir) in artifacts {
        for callee in [
            "Data.Bytes.Split.split-step",
            "Data.Bytes.View.index-forward-blocks",
            "Data.Bytes.View.index-forward-tail",
            "Data.Bytes.View.load-word.llvm",
            "Data.Bytes.View.load-byte.llvm",
            "Data.Bytes.View.matching-lanes.llvm",
        ] {
            assert!(
                split_call_path(&ir, "Main.main", callee).is_some(),
                "missing complete entrypoint split call path to {callee}; retained {}",
                file.display()
            );
        }
        for (start, end) in [
            ("index-forward-blocks", "load-word.llvm"),
            ("index-forward-blocks", "matching-lanes.llvm"),
            ("index-forward-tail", "load-byte.llvm"),
        ] {
            assert!(
                split_call_path(
                    &ir,
                    &format!("Data.Bytes.View.{start}"),
                    &format!("Data.Bytes.View.{end}")
                )
                .is_some(),
                "missing real worker path {start} -> {end}; retained {}",
                file.display()
            );
        }
        assert!(split_definition(&ir, "Data.Bytes.View.load-word.llvm")
            .contains("load i64, ptr %at, align 1"));
        assert!(split_definition(&ir, "Data.Bytes.View.load-byte.llvm")
            .contains("load i8, ptr %at, align 1"));
        assert!(
            split_definition(&ir, "Data.Bytes.View.matching-lanes.llvm")
                .contains("9187201950435737471")
                && split_definition(&ir, "Data.Bytes.View.matching-lanes.llvm")
                    .contains("-9187201950435737472")
        );
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&file)
        .arg("-o")
        .arg(file.with_extension("o"))
        .output()
        .unwrap();
        if !output.status.success() {
            panic!(
                "{triple}: {}\nretained full-adapter inputs: {}",
                String::from_utf8_lossy(&output.stderr),
                directory.display()
            );
        }
    }
}
