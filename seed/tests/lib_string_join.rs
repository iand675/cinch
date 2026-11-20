//! Exact separator-aware immutable String construction across explicit embedded-source epochs.
use cinch::driver::Mode;
use serde_json::Value;

/// The module owns private immutable UTF-8 backing and is embedded in the frontend.
const STRING: &str = include_str!("../prelude/Data/String.cinch");
/// Runtime inputs and byte observations are shared with the independently checked constructor fixtures.
const HELPERS: &str = include_str!("fixtures/string-construction-consumer.cinchpart");
/// Concrete join, clone and mutable-input boundary observations.
const EXTRA: &str = include_str!("fixtures/string-join-consumer.cinchpart");

/// Read fixed independent observations without a generating toolchain at consumer-test time.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/string-join-vectors.json")).unwrap()
}

/// Preserve valid scalar values and control escapes in source literals.
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

/// Render exact UTF-8/scalar identities without reconstructing expected joined text at runtime.
fn checks(rows: &[Value]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            format!(
                "join-case [{}] {} {} {} {} {} {}",
                r["pattern"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| literal(s.as_str().unwrap()))
                    .collect::<Vec<_>>()
                    .join(","),
                r["cycles"],
                literal(r["separator"].as_str().unwrap()),
                r["bytes"],
                r["scalars"],
                literal(r["hex"].as_str().unwrap()),
                literal(r["sha256"].as_str().unwrap())
            )
        })
        .collect()
}

/// Compile an exact source epoch and run bounded process batches in both reclamation modes.
fn run(checks: &[String], optimization: u8, override_source: bool, repetitions: usize) {
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
        source.push_str(&format!("\n  -- | Observe independent immutable join batch {i}.\n  checks{i} :: Unit -> Bool\n  checks{i} unit = all [\n{}\n    ]\n", batch.iter().map(|value| format!("    ({value})")).collect::<Vec<_>>().join(",\n")));
        names.push(format!("checks{i} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (i, name) in names.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{i}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one bounded poison-quarantine batch.\n  selected :: String -> Bool\n  selected batch = {dispatch}\n  -- | Observe one complete public construction batch.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n"));
    let selected = if override_source {
        format!("-- Explicit String join/clone source-override receipt v1.\n{STRING}")
    } else {
        STRING.to_owned()
    };
    if override_source {
        std::fs::write(directory.path().join("Data.String.cinch"), &selected).unwrap();
    }
    let entry = directory.path().join("Main.cinch");
    std::fs::write(&entry, source).unwrap();
    let exe = directory.path().join("consumer");
    let (input, output) = (entry.clone(), exe.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            cinch::driver::driver_with_profile_opts(
                Mode::Build,
                cinch::check::StageMode::Refresh,
                &input,
                Some(&output),
                &cinch::core::TargetProfile {
                    reclaim: true,
                    rc_debug: true,
                    ..cinch::core::TargetProfile::native()
                },
                &cinch::driver::DriverOptions {
                    opt_level: Some(optimization),
                    ..Default::default()
                },
            )
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap();
    let interface: Value = serde_json::from_slice(
        &std::fs::read(directory.path().join("build/Data.String.cinchif")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        interface["source_hash"],
        cinch::interface::content_hash(&selected)
    );
    for debug in [false, true] {
        for batch in 0..names.len() {
            for _ in 0..repetitions {
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
}

/// Every empty/Unicode/deep-list separator case agrees with independent exact bytes at O0.
#[test]
fn rust_string_join_source_override_matches_independent_o0_outputs() {
    run(&checks(fixture()["joins"].as_array().unwrap()), 0, true, 1);
}

/// Repeated optimized pipelines retain immutable backing owners and isolate mutable boundary copies.
#[test]
fn rust_string_join_optimized_source_override_preserves_owners() {
    run(&["join-owners-case ()".into()], 2, true, 10);
}

/// A freshly built frontend uses the current embedded String implementation without local shadows.
#[test]
fn rust_string_join_fresh_embedded_epoch_passes_o0_deep_lists() {
    let f = fixture();
    let rows: Vec<_> = f["joins"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["cycles"] == 16384)
        .cloned()
        .collect();
    run(&checks(&rows), 0, false, 1);
}

/// Every added public and private declaration has canonical owned documentation.
#[test]
fn string_join_added_declarations_have_canonical_documentation() {
    let parsed =
        cinch::cinchdoc::parse_source(STRING, Some("Data.String"), "Data.String", None, false)
            .unwrap();
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for name in ["string-clone", "join-size", "join-copy", "string-join"] {
        let declaration = declarations.iter().find(|d| d.name == name).unwrap();
        let doc = declaration.doc.as_ref().unwrap();
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}
