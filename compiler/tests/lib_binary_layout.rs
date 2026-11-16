//! Typed binary descriptors, exact arithmetic bits and explicit stream progress.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Concrete consumers are shared with the standalone two-frontend corpus runner.
const HELPERS: &str = include_str!("fixtures/binary-layout-consumers.cinchpart");
/// Ownership, stream and application-record contracts use concrete public codecs.
const CONTRACTS: &str = include_str!("fixtures/binary-layout-contracts.cinchpart");
/// The pinned SDK is needed only for independently regenerating these observations.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/binary-layout-vectors.json")).unwrap()
}
/// Select the concrete arithmetic layout used by each independently encoded carrier.
fn kind(row: &Value) -> usize {
    [
        "bool", "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64", "c64", "c128",
    ]
    .iter()
    .position(|name| Some(*name) == row["kind"].as_str())
    .unwrap()
}
/// Keep target-native order distinct from either explicit wire-order value.
fn order(row: &Value) -> usize {
    ["little", "big", "native"]
        .iter()
        .position(|name| Some(*name) == row["order"].as_str())
        .unwrap()
}
/// Format exact raw bytes as source literals.
fn quoted(value: &Value) -> String {
    format!("\"{}\"", value.as_str().unwrap())
}
/// Compile a complete runtime consumer, then check bounded normal and poisoned ownership runs.
fn run(checks: Vec<String>, contracts: bool, optimized: bool) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Text.Binary.Layout",
            include_str!("../prelude/Text/Binary/Layout.cinch"),
        ),
        (
            "Effect.Binary",
            include_str!("../prelude/Effect/Binary.cinch"),
        ),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Data.Complex",
            include_str!("../prelude/Data/Complex.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let mut source = HELPERS.to_owned();
    if contracts {
        source.push_str(CONTRACTS);
    }
    let mut names = Vec::new();
    for (index, batch) in checks.chunks(96).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent runtime fixture batch {index}.\n  checks{index} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool\n  checks{index} unit = all [\n"));
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
    source.push_str(&format!("\n  -- | Observe the complete public fixture result.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in if {} then 0 else 1\n", names.join(" && ")));
    let input = directory.path().join("Main.cinch");
    std::fs::write(&input, source).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized {
        let (input, exe, profile) = (input.clone(), exe.clone(), profile.clone());
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &input,
                    Some(&exe),
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
/// Every concrete arithmetic field preserves its exact signed, floating and complex bit pattern.
#[test]
fn binary_scalar_layouts_match_independent_bytes_and_offset_operations() {
    let f = fixture();
    run(
        f["scalars"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "scalar-case {} {} {} {} {} {}",
                    kind(r),
                    order(r),
                    r["bits"],
                    r["imaginary"],
                    r["size"],
                    quoted(&r["hex"])
                )
            })
            .collect(),
        false,
        false,
    );
}
/// Complete SIMD blocks and scalar fringes cover every concrete homogeneous carrier.
#[test]
fn binary_arrays_match_independent_bytes_and_full_output_digests() {
    let f = fixture();
    run(
        f["arrays"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "array-case {} {} {} {} {} {}",
                    kind(r),
                    order(r),
                    r["count"],
                    r["size"],
                    quoted(&r["hex"]),
                    quoted(&r["sha256"])
                )
            })
            .collect(),
        false,
        false,
    );
}
/// Products, application records, blank padding and nested arrays compose into actual codecs.
#[test]
fn binary_record_views_match_independent_layouts() {
    let f = fixture();
    run(
        f["records"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                let mut args = vec![
                    order(r).to_string(),
                    if r["active"].as_bool().unwrap() {
                        "True".into()
                    } else {
                        "False".into()
                    },
                ];
                for name in [
                    "code",
                    "count_bits",
                    "single_bits",
                    "double_bits",
                    "real_bits",
                    "imaginary_bits",
                ] {
                    args.push(r[name].to_string());
                }
                args.push(format!(
                    "[{}]",
                    r["samples"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                ));
                args.push(quoted(&r["hex"]));
                format!("record-case {}", args.join(" "))
            })
            .collect(),
        true,
        false,
    );
}
/// Full-width signed and unsigned variable integers match independent shortest encodings.
#[test]
fn binary_varints_match_independent_append_put_and_decode_results() {
    let f = fixture();
    run(
        f["varints"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "varint-case {} {} {}",
                    r["bits"],
                    quoted(&r["unsigned"]),
                    quoted(&r["signed"])
                )
            })
            .collect(),
        true,
        false,
    );
}
/// Ten continuation bytes distinguish buffer incompleteness from bounded stream overflow.
#[test]
fn binary_varint_prefix_errors_preserve_exact_progress_and_remaining_input() {
    let f = fixture();
    run(
        f["prefixes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                let tag = match r["kind"].as_str().unwrap() {
                    "" => 0,
                    "incomplete" => 1,
                    "overflow" => 2,
                    other => panic!("unexpected {other}"),
                };
                let stream = match r["stream_kind"].as_str().unwrap() {
                    "" => 0,
                    "eof" => 1,
                    "truncated" | "overflow" => 2,
                    other => panic!("unexpected {other}"),
                };
                format!(
                    "prefix-case {} {tag} {} {} {stream} {} {} {}",
                    quoted(&r["input"]),
                    r["count"],
                    r["value"],
                    r["stream_count"],
                    r["stream_value"],
                    quoted(&r["remaining"])
                )
            })
            .collect(),
        true,
        false,
    );
}
/// Invalid nested values fail atomically; copied byte fields isolate aliases and zero layouts remain valid.
#[test]
fn binary_buffer_alias_bounds_padding_and_validation_contracts() {
    run(vec!["contracts-case ()".into()], true, false);
}
/// Streams retain advanced continuations, read ownership, short-write progress and mutation-safe retries.
#[test]
fn binary_stream_eof_alias_partial_write_and_failure_contracts() {
    run(
        vec!["streams-case ()".into(), "stream-stress-case ()".into()],
        true,
        false,
    );
}
/// Optimized packed-array and borrowed stream callbacks retain every owner across repeated pipelines.
#[test]
fn rust_binary_layout_optimized_ownership_contracts() {
    run(vec!["stress-case ()".into(),"contracts-case ()".into(),"streams-case ()".into(),"array-case 9 1 1024 4096 \"\" \"38d93503d71c29ba89b60c412d26445631a0b74a988c1063df82528318190183\"".into()],true,true);
}
/// Actual packed integer lanes and float bitcasts remain legal for portable and big-endian objects.
#[test]
fn binary_layout_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let source = include_str!("../prelude/Text/Binary/Layout.cinch");
    let mut kernels = String::from("-- | Isolated packed binary-layout kernels.\nmodule Main where\n  import Data.Machine\n  import Data.Bool\n  import Data.Bytes\n  import Data.Nat\n");
    for (begin, end) in [
        (
            "  -- | Select target endianness at compilation",
            "  -- | Resolve explicit or target-native",
        ),
        (
            "  -- | Retain a complete byte owner",
            "  -- | Reverse one complete unsigned sixteen",
        ),
        (
            "  -- | Observe all binary32",
            "  -- | One bit-preserving binary32",
        ),
        (
            "  -- | Pack four low byte lanes",
            "  -- | Resolve the packed little-endian",
        ),
    ] {
        let start = source.find(begin).unwrap();
        let finish = source[start..].find(end).unwrap() + start;
        kernels.push_str(&source[start..finish]);
    }
    kernels.push_str(include_str!(
        "fixtures/binary-layout-kernel-entry.cinchpart"
    ));
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
        let input = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(
                    &input,
                    "binary-layout-kernels.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<4 x i32>"));
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
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Every declaration has canonical module-owned documentation.
#[test]
fn binary_layout_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Binary.Layout",
            include_str!("../prelude/Text/Binary/Layout.cinch"),
        ),
        (
            "Effect.Binary",
            include_str!("../prelude/Effect/Binary.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
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
}
