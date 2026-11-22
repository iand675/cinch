//! Bounded-stack immutable string construction and explicit base-source epochs.
use cinch::driver::Mode;
use serde_json::Value;

/// The exact implementation under test is an embedded base module, not an ordinary source-distributed extension.
const STRING: &str = include_str!("../prelude/Data/String.cinch");
/// Concrete consumers use independent byte digests and opaque runtime construction counts.
const HELPERS: &str = include_str!("fixtures/string-construction-consumer.cinchpart");

/// Read fixed independent observations without requiring their generating SDK during consumer tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/string-construction-vectors.json")).unwrap()
}

/// Preserve valid Unicode scalars and controls through accepted source literal escapes.
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

/// Render actual public construction calls with expected bytes supplied only by the independent fixture.
fn checks(group: &str, rows: &[Value]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            let expected = format!(
                "{} {} {} {}",
                r["bytes"],
                r["scalars"],
                literal(r["hex"].as_str().unwrap()),
                literal(r["sha256"].as_str().unwrap())
            );
            match group {
                "repeats" => format!(
                    "repeat-case {} {} {expected}",
                    literal(r["text"].as_str().unwrap()),
                    r["count"]
                ),
                "concats" => format!(
                    "concat-case [{}] {} {expected}",
                    r["pieces"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| literal(v.as_str().unwrap()))
                        .collect::<Vec<_>>()
                        .join(","),
                    r["cycles"]
                ),
                other => panic!("unknown fixture group {other}"),
            }
        })
        .collect()
}

/// Compile a selected source epoch with a real CLI-style project, then check both ownership modes.
fn run(checks: &[String], opt_level: u8, override_source: bool) {
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
    let mut source = HELPERS.replace(
        "module Main where",
        "module Main where\n  import Effect.Process",
    );
    let mut names = Vec::new();
    // Poison quarantine retains freed allocations until process exit. Keep
    // each independent poisoned observation in its own process, including the
    // 100-iteration owner control. Ordinary mode still runs the entire corpus
    // together under the smaller live-heap limit.
    let batch_width = 1;
    for (index, batch) in checks.chunks(batch_width).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent construction fixture batch {index}.\n  checks{index} :: Unit -> Bool\n  checks{index} unit = all [\n"));
        source.push_str(
            &batch
                .iter()
                .map(|s| format!("    ({s})"))
                .collect::<Vec<_>>()
                .join(",\n"),
        );
        source.push_str("\n    ]\n");
        names.push(format!("checks{index} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (index, name) in names.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{index}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select all ordinary observations or one bounded poisoned batch.\n  selected :: String -> Bool\n  selected batch = if batch == \"all\" then {} else {dispatch}\n  -- | Observe every construction with separate live-space and quarantine bounds.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n",names.join(" && ")));
    let selected_source = if override_source {
        // A comment discriminator prevents an identical fresh embedded copy from satisfying the override receipt accidentally.
        let selected = format!("-- Explicit constructor source-override receipt v1.\n{STRING}");
        std::fs::write(directory.path().join("Data.String.cinch"), &selected).unwrap();
        selected
    } else {
        STRING.to_owned()
    };
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, source).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
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
                    opt_level: Some(opt_level),
                    ..Default::default()
                },
            )
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    let interface = cinch::interface::read_package(&directory.path().join("build/Main.cpk")).unwrap()
        .into_iter().map(|(iface, _)| iface).find(|iface| iface.module == "Data.String").unwrap();
    assert_eq!(interface.module, "Data.String");
    assert_eq!(
        interface.source_hash,
        cinch::interface::content_hash(&selected_source)
    );
    if override_source {
        assert_ne!(
            interface.source_hash,
            cinch::interface::content_hash(STRING)
        );
    }
    for debug in [false, true] {
        let batches = if debug {
            (0..names.len()).map(|index| index.to_string()).collect()
        } else {
            vec!["all".to_owned()]
        };
        for batch in batches {
            let output = std::process::Command::new(&exe)
                .arg(&batch)
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1")
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "batch={batch}, debug={debug}, opt={opt_level}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
        }
    }
}

/// Unoptimized consumers cover complete independent Unicode, control, empty and deep-traversal boundaries.
#[test]
fn rust_string_construction_source_override_matches_independent_o0_results() {
    let f = fixture();
    let mut all = checks("repeats", f["repeats"].as_array().unwrap());
    all.extend(checks("concats", f["concats"].as_array().unwrap()));
    all.push("owners-case ()".into());
    run(&all, 0, true);
}

/// Optimized temporary owners and deep constructions preserve exact UTF-8 and conversion alias isolation.
#[test]
fn rust_string_construction_source_override_preserves_optimized_owners() {
    let f = fixture();
    let repeats: Vec<_> = f["repeats"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["count"] == 65536 || r["count"] == 65537)
        .cloned()
        .collect();
    let concats: Vec<_> = f["concats"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["cycles"] == 65536 || r["cycles"] == 16384)
        .cloned()
        .collect();
    let mut all = checks("repeats", &repeats);
    all.extend(checks("concats", &concats));
    all.push("owners-case ()".into());
    run(&all, 2, true);
}

/// The freshly compiled embedded family itself supplies the repaired public constructors without an override.
#[test]
fn rust_string_construction_fresh_embedded_epoch_passes_o0_deep_cases() {
    let f = fixture();
    let repeat = f["repeats"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["text"] == "x" && r["count"] == 65536)
        .unwrap()
        .clone();
    let concat = f["concats"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["pieces"] == serde_json::json!(["x"]) && r["cycles"] == 65536)
        .unwrap()
        .clone();
    let mut all = checks("repeats", &[repeat]);
    all.extend(checks("concats", &[concat]));
    run(&all, 0, false);
}

/// Every changed or added construction declaration has canonical module-owned documentation.
#[test]
fn rust_string_construction_changed_declarations_have_canonical_docs() {
    let parsed =
        cinch::cinchdoc::parse_source(STRING, Some("Data.String"), "Data.String", None, false)
            .unwrap();
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for name in [
        "string-concat-all",
        "concat-size",
        "concat-copy",
        "string-repeat",
        "repeat-double",
    ] {
        let declaration = declarations.iter().find(|d| d.name == name).unwrap();
        let doc = declaration.doc.as_ref().unwrap();
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}
