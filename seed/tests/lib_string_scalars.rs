//! Sealed UTF-8 String observations, packed counts and unchanged raw-prefix behavior.
use cinch::driver::Mode;
use serde_json::Value;

/// The source epoch owns private immutable backing and adds no base dependency.
const STRING: &str = include_str!("../prelude/Data/String.cinch");
/// Concrete byte/digest observations and opaque runtime construction counts.
const HELPERS: &str = include_str!("fixtures/string-construction-consumer.cinchpart");
/// Endpoint projections, all-scalar chunks, raw-prefix controls and owner pipelines.
const EXTRA: &str = include_str!("fixtures/string-scalar-consumer.cinchpart");

/// Read independent fixed observations without the generating SDK during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/string-scalar-vectors.json")).unwrap()
}

/// Preserve valid scalar/control source literals without encoding expected outputs through the implementation.
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

/// Represent absent endpoints by the first code beyond the scalar range.
fn endpoint(row: &Value, name: &str) -> i64 {
    let value = row[name].as_i64().unwrap();
    if value < 0 {
        1114112
    } else {
        value
    }
}

/// Render actual complete public calls with independently supplied endpoint and wire identities.
fn checks(group: &str, rows: &[Value]) -> Vec<String> {
    rows.iter()
        .map(|r| {
            if group == "raw" {
                return format!(
                    "raw-length-case {} {} {} {}",
                    r["hex"], r["start"], r["initial"], r["result"]
                );
            }
            let prefix = if group == "aggregates" {
                format!("aggregate-case {} {}", r["low"], r["high"])
            } else {
                format!(
                    "scalar-case {} {}",
                    literal(r["text"].as_str().unwrap()),
                    r["count"]
                )
            };
            format!(
                "{prefix} {} {} {} {} {} {} {}",
                r["bytes"],
                r["scalars"],
                endpoint(r, "first"),
                r["firstWidth"],
                endpoint(r, "last"),
                r["lastWidth"],
                r["sha256"]
            )
        })
        .collect()
}

/// Keep ordinary complete runs and one-observation poison processes in one checked consumer.
fn consumer(checks: &[String]) -> String {
    let mut source = format!(
        "{}\n{EXTRA}\n",
        HELPERS.replace(
            "  import Data.String\n",
            "  import Data.String\n  import Data.Char\n  import Effect.Process\n"
        )
    );
    let mut groups = Vec::new();
    for (group, batch) in checks.chunks(64).enumerate() {
        source.push_str(&format!("\n  -- | Observe complete independent scalar batch {group}.\n  checks{group} :: Unit -> Bool\n  checks{group} unit = all [\n{}\n    ]\n",batch.iter().map(|value|format!("    ({value})")).collect::<Vec<_>>().join(",\n")));
        groups.push(format!("checks{group} ()"));
        let mut dispatch = "False".to_owned();
        for (index, expression) in batch.iter().enumerate().rev() {
            dispatch = format!("if item == \"{index}\" then ({expression}) else {dispatch}");
        }
        source.push_str(&format!("\n  -- | Select one independently poisoned observation in batch {group}.\n  selected{group} :: String -> Bool\n  selected{group} item = {dispatch}\n"));
    }
    let mut dispatch = "False".to_owned();
    for group in (0..groups.len()).rev() {
        dispatch = format!("if group == \"{group}\" then selected{group} item else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one observation without a deeply nested global branch chain.\n  selected :: String -> String -> Bool\n  selected group item = {dispatch}\n  -- | Observe all ordinary assertions or one poisoned assertion.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons group (Cons item Nil)) -> if (if group == \"all\" then {} else selected group item) then 0 else 1\n    other -> 2\n",groups.join(" && ")));
    source
}

/// Compile the exact selected source, verify its interface identity and run both fixed heap profiles.
fn run(checks: &[String], optimization: u8, overrides: bool, repetitions: usize) {
    let directory = tempfile::tempdir().unwrap();
    for (name, text) in [
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), text).unwrap();
    }
    let selected = if overrides {
        format!("-- Explicit sealed String scalar source-override receipt v1.\n{STRING}")
    } else {
        STRING.to_owned()
    };
    if overrides {
        std::fs::write(directory.path().join("Data.String.cinch"), &selected).unwrap();
    }
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, consumer(checks)).unwrap();
    let exe = directory.path().join("consumer");
    let (entry, output) = (input.clone(), exe.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            cinch::driver::driver_with_profile_opts(
                Mode::Build,
                cinch::check::StageMode::Refresh,
                &entry,
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
    let interface = cinch::interface::read_package(&directory.path().join("build/Main.cpk")).unwrap()
        .into_iter().map(|(iface, _)| iface).find(|iface| iface.module == "Data.String").unwrap();
    assert_eq!(
        interface.source_hash,
        cinch::interface::content_hash(&selected)
    );
    for debug in [false, true] {
        let cases: Vec<_> = if debug {
            (0..checks.len()).map(Some).collect()
        } else {
            vec![None]
        };
        for case in cases {
            for _ in 0..repetitions {
                let args = case.map_or_else(
                    || vec!["all".to_owned(), "0".to_owned()],
                    |i| vec![(i / 64).to_string(), (i % 64).to_string()],
                );
                let result = std::process::Command::new(&exe)
                    .args(args)
                    .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                    .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                    .env("CINCH_STACK_SIZE", "8M")
                    .env("CINCH_LEAK_CHECK", "1")
                    .output()
                    .unwrap();
                assert_eq!(
                    result.status.code(),
                    Some(0),
                    "debug={debug} case={case:?}: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
                assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
            }
        }
    }
}

/// All scalar widths, alignments, complete scalar-range wires and raw stop-at-error controls pass at O0.
#[test]
fn rust_sealed_string_observations_match_independent_o0_reference() {
    let f = fixture();
    let mut all = Vec::new();
    for group in ["texts", "edges", "aggregates", "raw"] {
        all.extend(checks(group, f[group].as_array().unwrap()));
    }
    run(&all, 0, true, 1);
}

/// Independently projected nested observations retain temporary backing owners under optimized checked RC.
#[test]
fn rust_sealed_string_optimized_temporary_owners() {
    run(&["scalar-owners-case ()".into()], 2, true, 10);
}

/// The complete all-scalar wire traverses the freshly embedded implementation without a local source shadow.
#[test]
fn rust_sealed_string_fresh_embedded_all_scalar_epoch() {
    let f = fixture();
    let rows: Vec<_> = f["aggregates"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["low"] == 0 && r["high"] == 1114111)
        .cloned()
        .collect();
    run(&checks("aggregates", &rows), 0, false, 1);
}

/// Every added or touched declaration uses canonical documentation owned by this module.
#[test]
fn sealed_string_scalar_declarations_have_canonical_documentation() {
    let parsed =
        cinch::cinchdoc::parse_source(STRING, Some("Data.String"), "Data.String", None, false)
            .unwrap();
    let mut ds = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut ds);
    for name in [
        "string-length",
        "string-utf8-first",
        "string-utf8-last",
        "string-last-scalar-from",
        "string-count-add",
        "string-count-wide",
        "string-count-more",
        "string-count-sixteen",
        "string-count-eight",
        "string-count-one",
        "string-count-from",
        "string-count-tail",
    ] {
        let d = ds.iter().find(|d| d.name == name).unwrap();
        let doc = d.doc.as_ref().unwrap();
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Actual SIMD/SWAR kernels compile for portable and big-endian object profiles.
#[test]
fn sealed_string_count_kernels_compile_for_wasm_arm_and_big_endian() {
    let start = STRING
        .find("  -- | Add bounded private machine counters")
        .unwrap();
    let source=format!("-- | Actual sealed String scalar-count production kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Bool\n  import Data.Machine\n  import Data.Nat\n{}\n  -- | Keep the private backing owner through the complete raw traversal.\n  held :: forall a. Bytes -> a -> a\n  held raw value = case bytes-len raw of _ -> value\n  -- | Root packed blocks and scalar fringes in actual object code.\n  main :: Int\n  main = let raw = bytes-from-u8s [97,0,195,169,231,149,140,240,159,153,130,97,97,97,97,97,97] in let count = held raw (string-count-from (bytes-data raw) 0 (nat-to-i64 (bytes-len raw)) 0) in if eq-i64 count 11 then 0 else 1\n",&STRING[start..]);
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
            let mut p = cinch::core::TargetProfile::native();
            p.triple = triple.into();
            p.arch = arch.into();
            p.endian = endian.into();
            p
        });
        let input = source.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let p = cinch::driver::check_sources_profile(
                    &input,
                    "string-count-kernels.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &p)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("load <16 x i8>"));
        assert!(ir.contains("bitcast <16 x i8> %flags to <2 x i64>"));
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, ir).unwrap();
        let result = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            result.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}
