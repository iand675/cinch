//! Fresh raw-byte construction with exact output identities and independent mutable ownership.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// Source-only operations preserve the existing embedded buffer representation.
const BUILD: &str = include_str!("../prelude/Data/Bytes/Build.cinch");
/// Concrete consumers use opaque runtime counts and independent expected bytes.
const HELPERS: &str = include_str!("fixtures/byte-build-consumer.cinchpart");

/// Read reference observations without requiring the external generating toolchain during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/byte-build-vectors.json")).unwrap()
}

/// Render complete independent repetitions and separator placement observations.
fn checks(group: &str, rows: &[Value]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            if group == "repeats" {
                format!(
                    "repeat-case {} {} {} {} {}",
                    r["input"], r["count"], r["bytes"], r["hex"], r["sha256"]
                )
            } else {
                format!(
                    "join-case {} {} {} {} {} {}",
                    r["pattern"], r["cycles"], r["separator"], r["bytes"], r["hex"], r["sha256"]
                )
            }
        })
        .collect()
}

/// Compile actual source operations and check both ordinary and poisoned backing reclamation.
fn run(checks: &[String], opt_level: Option<u8>) {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Data.Bytes.Build.cinch"), BUILD).unwrap();
    for (name, contents) in [
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
    }
    let mut source = HELPERS.to_owned();
    let mut batches = Vec::new();
    for (i, batch) in checks.chunks(8).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent owned-byte construction batch {i}.\n  checks{i} :: Unit -> Bool\n  checks{i} unit = all [\n{}\n    ]\n", batch.iter().map(|c| format!("    ({c})")).collect::<Vec<_>>().join(",\n")));
        batches.push(format!("checks{i} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (i, name) in batches.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{i}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one bounded poison-quarantine batch.\n  selected :: String -> Bool\n  selected batch = {dispatch}\n  -- | Observe one public construction batch.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n"));
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, source).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if let Some(level) = opt_level {
        let (entry, output) = (input.clone(), exe.clone());
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
                        opt_level: Some(level),
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
        for batch in 0..batches.len() {
            let output = std::process::Command::new(&exe)
                .arg(batch.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1")
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
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
}

/// Raw bytes, empty pieces and complete-prefix fringes match every independently generated output.
#[test]
fn owned_byte_construction_matches_independent_raw_outputs() {
    let f = fixture();
    let mut rows = checks("repeats", f["repeats"].as_array().unwrap());
    rows.extend(checks("joins", f["joins"].as_array().unwrap()));
    rows.push("alias-case ()".into());
    run(&rows, None);
}

/// Deep piece lists use bounded stack even without optimization.
#[test]
fn rust_owned_byte_construction_o0_has_bounded_stack() {
    let f = fixture();
    let rows: Vec<_> = f["joins"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["cycles"] == 16384)
        .cloned()
        .collect();
    run(&checks("joins", &rows), Some(0));
}

/// Repeated pipelines hold source owners and never retain aliases to returned mutable buffers.
#[test]
fn rust_owned_byte_construction_o2_releases_temporary_owners() {
    run(&["owners-case ()".into()], Some(2));
}

/// Every private and public implementation declaration has canonical owned documentation.
#[test]
fn owned_byte_construction_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        BUILD,
        Some("Data.Bytes.Build"),
        "Data.Bytes.Build",
        None,
        false,
    )
    .unwrap();
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
