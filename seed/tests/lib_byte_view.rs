//! Owner-retaining byte spans, stable compiled patterns and exact substring observations.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// Actual source modules are copied into isolated consumers rather than resolved from a repository cwd.
const VIEW: &str = include_str!("../prelude/Data/Bytes/View.cinch");
/// Concrete assertions use checked runtime recipes and independent query results.
const HELPERS: &str = include_str!("fixtures/byte-view-consumer.cinchpart");

/// Read independently generated exact byte and malformed-scalar observations.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/byte-view-vectors.json")).unwrap()
}

/// Render one typed complete public query pipeline.
fn expression(row: &Value) -> String {
    let fields = [
        "mode",
        "size",
        "start",
        "end",
        "needle_mode",
        "needle_size",
        "needle_source",
        "first",
        "last",
        "count",
        "prefix",
        "suffix",
        "compare",
        "equal",
    ];
    format!(
        "view-case {}",
        fields
            .iter()
            .map(|name| match &row[name] {
                Value::Bool(true) => "True".into(),
                Value::Bool(false) => "False".into(),
                value => format!("({value})"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    )
}

/// Dispatch bounded checked-RC process groups without accumulating unrelated quarantine.
fn consumer(checks: &[String], width: usize) -> String {
    let mut source = HELPERS.to_owned();
    if checks
        .iter()
        .any(|check| check.starts_with("endpoint-case "))
    {
        source = source.replace(
            "  import Data.String\n",
            "  import Data.String\n  import Text.Hex\n",
        );
        source.push_str(include_str!(
            "fixtures/byte-view-endpoint-consumer.cinchpart"
        ));
    }
    let mut names = Vec::new();
    for (i, batch) in checks.chunks(width).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent byte-view fixture group {i}.\n  checks{i} :: Unit -> Bool\n  checks{i} unit = all [\n{}\n    ]\n",
            batch.iter().map(|check| format!("    ({check})")).collect::<Vec<_>>().join(",\n")));
        names.push(format!("checks{i} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (i, name) in names.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{i}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select a bounded independent ownership group.\n  selected :: String -> Bool\n  selected batch = {dispatch}\n  -- | Observe complete public byte queries.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n"));
    source
}

/// Compile one exact source closure with the chosen frontend and verify ordinary and poisoned reclamation.
fn run(checks: Vec<String>, width: usize, opt_level: Option<u8>) {
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
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, consumer(&checks, width)).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if let Some(level) = opt_level {
        let (entry, output, profile) = (input.clone(), exe.clone(), profile.clone());
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
        for batch in 0..checks.len().div_ceil(width) {
            let output = std::process::Command::new(&exe)
                .arg(batch.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_LEAK_CHECK", "1")
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "batch={batch}, debug={debug}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
        }
    }
}

/// Short complete-word fringes, independent span ends and overlapping cuts match every reference answer.
#[test]
fn byte_view_queries_match_independent_short_span_observations() {
    let f = fixture();
    run(
        f["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["size"].as_u64().unwrap() <= 129)
            .map(expression)
            .collect(),
        32,
        None,
    );
}

/// Long malformed and repeated-prefix inputs retain linear fallback semantics and bounded live ownership.
#[test]
fn byte_view_queries_match_independent_adversarial_long_observations() {
    let f = fixture();
    run(
        f["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["size"].as_u64().unwrap() > 129)
            .map(expression)
            .collect(),
        1,
        None,
    );
}

/// Endpoint agreement followed by a late interior mismatch must preserve complete linear fallback behavior.
#[test]
fn byte_view_endpoint_candidates_preserve_late_mismatch_fallbacks() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/byte-view-endpoint-vectors.json")).unwrap();
    run(
        fixture["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "endpoint-case {} {} {} {} {} {}",
                    r["size"],
                    r["width"],
                    r["start"],
                    r["end"],
                    r["raw_sha256"],
                    r["needle_sha256"]
                )
            })
            .collect(),
        1,
        None,
    );
}

/// Independently projected nested view children, atomic refusals and stable pattern snapshots survive release.
#[test]
fn byte_views_retain_alias_owners_and_refuse_invalid_writes_atomically() {
    run(vec!["owners-case ()".into(); 10], 1, None);
}

/// Optimized ownership traversal preserves temporary cut children and private pattern storage.
#[test]
fn rust_byte_views_o2_preserve_nested_alias_and_compiled_pattern_owners() {
    run(vec!["owners-case ()".into(); 10], 1, Some(2));
}

/// Actual private packed loads, endian-normalized failure entries and bounded loops compile portably.
#[test]
fn byte_view_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let start = VIEW.find("  -- | Keep one backing owner").unwrap();
    let end = VIEW
        .find("  -- | Convert a private absolute match sentinel")
        .unwrap();
    let count_start = VIEW
        .find("  -- | Count nonoverlapping complete matches with SWAR")
        .unwrap();
    let count_end = VIEW
        .find("  -- | Count nonoverlapping compiled matches;")
        .unwrap();
    let compare_start = VIEW
        .find("  -- | Find the first unequal private byte")
        .unwrap();
    let compare_end = VIEW
        .find("  -- | Compare raw view bytes lexicographically")
        .unwrap();
    let kernels = format!("-- | Isolated production byte-view kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Bool\n  import Data.Machine\n  import Data.Nat\n{}\n{}\n{}\n{}",
        &VIEW[start..end], &VIEW[count_start..count_end], &VIEW[compare_start..compare_end],
        include_str!("fixtures/byte-view-kernel-entry.cinchpart"));
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
        let source = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(
                    &source,
                    "byte-view-kernels.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("%x = load i64, ptr %a, align 1"));
        assert!(ir.contains("%sums = mul i64 %lanes, 72340172838076673"));
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, ir).unwrap();
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        if !output.status.success() {
            let retained = directory.keep();
            panic!(
                "{triple}: {}\nretained portability inputs: {}",
                String::from_utf8_lossy(&output.stderr),
                retained.display()
            );
        }
    }
}

/// All implementation declarations have canonical owned documentation.
#[test]
fn byte_view_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        VIEW,
        Some("Data.Bytes.View"),
        "Data.Bytes.View",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.doc.is_some() && parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
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
