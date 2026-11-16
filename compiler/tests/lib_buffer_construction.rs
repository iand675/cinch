//! Bounded scalar packing and typed byte-list construction across explicit base-source epochs.
use cinch::driver::Mode;
use serde_json::Value;

/// Every embedded implementation changed by this checkpoint is overridden and receipted independently.
const SOURCES: [(&str, &str); 2] = [
    ("Data.String", include_str!("../prelude/Data/String.cinch")),
    ("Data.Bytes", include_str!("../prelude/Data/Bytes.cinch")),
];
/// Shared consumers avoid reconstructing expected output with the constructor being tested.
const HELPERS: &str = include_str!("fixtures/string-construction-consumer.cinchpart");
/// Concrete pack/natural-byte/machine-byte/FromList calls and independent mutable-result checks.
const EXTRA: &str = include_str!("fixtures/buffer-construction-consumer.cinchpart");

/// Read fixed independent observations without requiring the generating SDK during consumer tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/buffer-construction-vectors.json")).unwrap()
}
/// Encode exact valid Unicode and controls with accepted source literal escapes.
fn literal(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
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
/// Render actual public constructor calls with independently supplied byte identities.
fn checks(group: &str, rows: &[Value]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            if group == "packed" {
                format!(
                    "packed-case {} {} {} {} {} {}",
                    literal(r["text"].as_str().unwrap()),
                    r["count"],
                    r["bytes"],
                    r["scalars"],
                    literal(r["hex"].as_str().unwrap()),
                    literal(r["sha256"].as_str().unwrap())
                )
            } else {
                format!(
                    "buffer-case {} {} {} {} {}",
                    r["mode"],
                    r["count"],
                    r["bytes"],
                    literal(r["hex"].as_str().unwrap()),
                    literal(r["sha256"].as_str().unwrap())
                )
            }
        })
        .collect()
}
/// Compile an exact selected source epoch, verify both interfaces, then check normal and poisoned ownership runs.
fn run(checks: &[String], opt_level: u8, overrides: bool) {
    let directory = tempfile::tempdir().unwrap();
    for (name, module) in [
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), module).unwrap();
    }
    let mut source = format!(
        "{}\n{EXTRA}\n",
        HELPERS.replace(
            "  import Data.String\n",
            "  import Data.String\n  import Effect.Process\n"
        )
    );
    let mut names = Vec::new();
    for (i, batch) in checks.chunks(8).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent typed-constructor fixture batch {i}.\n  checks{i} :: Unit -> Bool\n  checks{i} unit = all [\n"));
        source.push_str(
            &batch
                .iter()
                .map(|c| format!("    ({c})"))
                .collect::<Vec<_>>()
                .join(",\n"),
        );
        source.push_str("\n    ]\n");
        names.push(format!("checks{i} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (i, name) in names.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{i}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one bounded poison-quarantine fixture batch.\n  selected :: String -> Bool\n  selected batch = {dispatch}\n  -- | Observe one independent process batch.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n"));
    let mut selected = Vec::new();
    for (name, text) in SOURCES {
        let chosen = if overrides {
            format!("-- Explicit buffer-constructor source-override receipt v1.\n{text}")
        } else {
            text.to_owned()
        };
        if overrides {
            std::fs::write(directory.path().join(format!("{name}.cinch")), &chosen).unwrap();
        }
        selected.push((name, chosen));
    }
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, source).unwrap();
    let exe = directory.path().join("consumer");
    let (entry, output) = (input.clone(), exe.clone());
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
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
                    opt_level: Some(opt_level),
                    ..Default::default()
                },
            )
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    for (name, text) in selected {
        let interface: Value = serde_json::from_slice(
            &std::fs::read(directory.path().join(format!("build/{name}.cinchif"))).unwrap(),
        )
        .unwrap();
        assert_eq!(interface["module"], name);
        assert_eq!(
            interface["source_hash"],
            cinch::interface::content_hash(&text)
        );
    }
    for debug in [false, true] {
        for batch in 0..names.len() {
            let result = std::process::Command::new(&exe)
                .arg(batch.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1")
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .output()
                .unwrap();
            assert_eq!(
                result.status.code(),
                Some(0),
                "opt={opt_level}, debug={debug}: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
        }
    }
}
/// Unoptimized implementations cover every scalar width and exact typed raw-byte list boundary.
#[test]
fn rust_typed_constructors_source_override_match_independent_o0_results() {
    let f = fixture();
    let mut all = checks("packed", f["packed"].as_array().unwrap());
    all.extend(checks("buffers", f["buffers"].as_array().unwrap()));
    all.push("buffer-owners-case ()".into());
    run(&all, 0, true);
}
/// Optimized temporary owners and conversion aliases remain safe at deep-list boundaries.
#[test]
fn rust_typed_constructors_optimized_source_override_preserves_owners() {
    let f = fixture();
    let packed: Vec<_> = f["packed"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["count"] == 65536 || r["count"] == 65537)
        .cloned()
        .collect();
    let buffers: Vec<_> = f["buffers"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["count"] == 65536 || r["count"] == 65537)
        .cloned()
        .collect();
    let mut all = checks("packed", &packed);
    all.extend(checks("buffers", &buffers));
    all.push("buffer-owners-case ()".into());
    run(&all, 2, true);
}
/// Freshly compiled embedded modules themselves provide the repaired implementations without source shadows.
#[test]
fn rust_typed_constructors_fresh_embedded_epoch_passes_o0_deep_cases() {
    let f = fixture();
    let packed = f["packed"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["text"] == "é" && r["count"] == 65536)
        .unwrap()
        .clone();
    let buffer = f["buffers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["mode"] == 2 && r["count"] == 65536)
        .unwrap()
        .clone();
    let mut all = checks("packed", &[packed]);
    all.extend(checks("buffers", &[buffer]));
    run(&all, 0, false);
}
/// Every added or changed constructor declaration has canonical module-owned documentation.
#[test]
fn rust_typed_constructor_changed_declarations_have_canonical_documentation() {
    for (name, text) in SOURCES {
        let parsed = cinch::cinchdoc::parse_source(text, Some(name), name, None, false).unwrap();
        let mut ds = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut ds);
        let wanted: Vec<&str> = if name == "Data.String" {
            vec!["pack", "pack-size", "pack-copy"]
        } else {
            vec![
                "bytes-from-list",
                "byte-list-size",
                "bytes-fill-into",
                "bytes-from-u8s",
                "u8s-fill-into",
            ]
        };
        for label in wanted {
            let decl = ds.iter().find(|d| d.name == label).unwrap();
            let doc = decl.doc.as_ref().unwrap();
            assert!(!doc.legacy);
            let t = doc.markdown.to_lowercase();
            assert!(!t.contains("golang") && !t.contains("go's") && !t.contains("go "));
        }
    }
}
