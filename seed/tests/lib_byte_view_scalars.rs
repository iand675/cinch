//! Checked immutable scalar observations on retained mutable byte spans.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// The observed range stays opaque; consumers use no private backing or address accessors.
const VIEW: &str = include_str!("../prelude/Data/Bytes/View.cinch");
/// Concrete complete fingerprints observe every relative scalar field and status.
const HELPERS: &str = include_str!("fixtures/byte-view-scalar-consumer.cinchpart");

/// Render independent process groups with a fixed bounded poison quarantine.
fn consumer(checks: &[String], width: usize) -> String {
    let mut source = HELPERS.to_owned();
    let mut dispatch = "False".to_owned();
    for (i, group) in checks.chunks(width).enumerate().rev() {
        source.push_str(&format!("\n  -- | Observe independent scalar group {i}.\n  group{i} :: Unit -> Bool\n  group{i} unit = all [\n{}\n    ]\n", group.iter().map(|value| format!("    ({value})")).collect::<Vec<_>>().join(",\n")));
        dispatch = format!("if index == \"{i}\" then group{i} () else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one complete independent scalar group.\n  selected :: String -> Bool\n  selected index = {dispatch}\n  -- | Observe bounded scalar and ownership results.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons index Nil) -> if selected index then 0 else 1\n    other -> 2\n"));
    source
}

/// Compile the exact source closure and check both ordinary and poisoned backing reclamation.
fn run(checks: Vec<String>, width: usize, optimized: bool) {
    let directory = tempfile::tempdir().unwrap();
    for (name, text) in [
        ("Data.Bytes.View", VIEW),
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
            let output = std::process::Command::new(&exe)
                .arg(index.to_string())
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

/// Every legal subspan and selected relative coordinate agrees with independent bounded-slice decoding.
#[test]
fn byte_view_scalar_observations_match_independent_relative_fingerprints() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-view-scalar-vectors.json")).unwrap();
    let checks = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| format!("scalar-case {} {}", row["input"], row["hash"]))
        .collect();
    run(checks, 8, false);
}

/// Temporary backing, truncated scalar boundaries, alias mutations and huge positions remain safe.
#[test]
fn byte_view_scalar_observations_preserve_temporary_owner_and_boundary_semantics() {
    run(vec!["owners-case ()".into(); 10], 1, false);
}

/// Optimized scalar observations preserve the same retained-owner and mutation guarantees.
#[test]
fn rust_byte_view_scalar_observations_o2_preserve_retained_owners() {
    run(vec!["owners-case ()".into(); 10], 1, true);
}

/// The added observation has canonical owned documentation and preserves private storage boundaries.
#[test]
fn byte_view_scalar_observation_has_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        VIEW,
        Some("Data.Bytes.View"),
        "Data.Bytes.View",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    let declaration = declarations
        .iter()
        .find(|d| d.name == "byte-view-utf8-at")
        .unwrap();
    let doc = declaration.doc.as_ref().unwrap();
    assert!(!doc.legacy);
    let text = doc.markdown.to_lowercase();
    assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
}
