//! Typed JSON descriptors, record metadata, conversion errors and bounded ownership.
mod common;

use cinch::driver::Mode;

/// Build a source-distributed JSON consumer and check ordinary and poisoned reclamation.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Text.Json.Binary32",
            include_str!("../prelude/Text/Json/Binary32.cinch"),
        ),
        (
            "Text.Base64.Encoding",
            include_str!("../prelude/Text/Base64/Encoding.cinch"),
        ),
        (
            "Text.Json.Codec",
            include_str!("../prelude/Text/Json/Codec.cinch"),
        ),
        (
            "Text.Json.Value",
            include_str!("../prelude/Text/Json/Value.cinch"),
        ),
        (
            "Text.Json.Dynamic",
            include_str!("../prelude/Text/Json/Dynamic.cinch"),
        ),
        (
            "Text.Json.Legacy",
            include_str!("../prelude/Text/Json/Legacy.cinch"),
        ),
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
        ("Data.Json", include_str!("../prelude/Data/Json.cinch")),
        ("Text.Codec", include_str!("../prelude/Text/Codec.cinch")),
        ("Data.Array", include_str!("../prelude/Data/Array.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Data.Unicode.Kernel",
            include_str!("../prelude/Data/Unicode/Kernel.cinch"),
        ),
        (
            "Data.Unicode.Tables",
            include_str!("../prelude/Data/Unicode/Tables.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .unwrap();
    for debug in [false, true] {
        let result = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(0),
            "debug={debug}: stdout={} stderr={}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let diagnostic = String::from_utf8_lossy(&result.stderr);
        assert!(diagnostic.contains("live=0") && diagnostic.contains("leaked=0"));
    }
}

/// Primitive, nullable sequence, fixed array and application record updates match independent observations.
#[test]
fn json_typed_updates_preserve_partial_values_and_original_error_metadata() {
    run(include_str!("fixtures/json-codec-typed.cinch"));
}

/// Shortest formatting, finite range edges and nonfinite refusals match independent binary payloads.
#[test]
fn json_float64_formatting_and_updates_match_independent_observations() {
    run(include_str!("fixtures/json-codec-float.cinch"));
}

/// Exact midpoint rounding, finite range errors and shortest formatting match independent binary32 observations.
#[test]
fn json_float32_rounding_and_formatting_match_independent_observations() {
    run(include_str!("fixtures/json-codec-float32.cinch"));
}

/// Mutable source and output aliases, quoted fields and retained range errors survive repeated conversions.
#[test]
fn json_float32_owners_and_composed_updates_are_bounded() {
    run(include_str!("fixtures/json-codec-float32-control.cinch"));
}

/// Raw scalar/byte spellings preserve saturated and infinite outcomes, numeric metadata and strict encoding.
#[test]
fn json_raw_number_outcomes_match_independent_observations() {
    run(include_str!("fixtures/json-codec-raw-number.cinch"));
}

/// Long-significand cancellation and the actual safe-shift rounding primitive match independent integer models.
#[test]
fn json_raw_number_cancellation_and_rounding_kernel_are_exact() {
    run(include_str!("fixtures/json-codec-raw-number-edge.cinch"));
    run(include_str!("fixtures/json-codec-raw-number-kernel.cinch"));
}

/// Original raw/error bytes, checked numeric policies and composed field causes retain independent owners.
#[test]
fn json_raw_number_owners_and_error_capabilities_are_bounded() {
    run(include_str!("fixtures/json-codec-raw-number-control.cinch"));
}

/// Dynamic shapes, partial numeric errors, exact-number selection and both HTML encodings match independent observations.
#[test]
fn json_dynamic_updates_and_encodings_match_independent_observations() {
    run(include_str!("fixtures/json-codec-dynamic.cinch"));
    run(include_str!("fixtures/json-codec-dynamic-edge.cinch"));
}

/// Deep/flat traversal, byte ownership, retained errors and explicit established-family capabilities stay bounded.
#[test]
fn json_dynamic_owners_and_legacy_bridges_are_bounded() {
    run(include_str!("fixtures/json-codec-dynamic-control.cinch"));
}

/// Independently selected method directions and semantic token/value exports retain bounded owners.
#[test]
fn json_method_categories_and_semantic_owners_are_bounded() {
    run(include_str!("fixtures/json-codec-method-owner-control.cinch"));
}

/// Method priority, returned causes, advanced values and first-error continuation remain observable.
#[test]
fn json_method_priority_errors_and_continuation_match_observations() {
    run(include_str!("fixtures/json-codec-method-protocol.cinch"));
}

/// Every successful addressability context preserves exact output and method invocation counts.
#[test]
fn json_method_addressability_outputs_and_callback_counts_match_observations() {
    run(include_str!("fixtures/json-codec-addressability.cinch"));
}

/// Every vector mismatch lane and Unicode boundary follows independent exact-first name matching.
#[test]
fn json_record_name_matching_matches_independent_observations() {
    run(include_str!("fixtures/json-codec-field.cinch"));
}

/// Unicode/exact names, embedding dominance, custom errors and large records retain bounded ownership.
#[test]
fn json_record_policies_callbacks_and_large_values_are_bounded() {
    run(include_str!("fixtures/json-codec-codec-control.cinch"));
}

/// Every signed/unsigned machine width preserves independent null/type/range outcomes.
#[test]
fn json_integer_widths_match_independent_observations() {
    run(include_str!("fixtures/json-codec-integer.cinch"));
}

/// Binary slices and fixed byte arrays preserve independent text and partial numeric updates.
#[test]
fn json_binary_values_match_independent_observations() {
    run(include_str!("fixtures/json-codec-binary.cinch"));
}

/// Malformed binary prefixes preserve prior aliases, exact progress and large bounded pipelines.
#[test]
fn json_binary_partial_updates_aliases_and_policies_are_bounded() {
    run(include_str!("fixtures/json-codec-binary-control.cinch"));
}

/// The actual production SWAR primitives agree with a scalar model for every byte in every lane.
#[test]
fn json_word_kernels_match_every_lane_byte() {
    let source = include_str!("../prelude/Text/Json/Codec.cinch");
    let mut consumer = String::from("-- | Independent JSON word primitive validation.\nmodule Main where\n  import Data.Machine\n  import Data.Bytes.Word\n");
    for name in ["decimal-eight", "binary-linebreak-eight"] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        consumer.push_str("  -- | Exact production word primitive.\n");
        consumer.push_str(&rest[..end]);
        consumer.push('\n');
    }
    consumer.push_str(include_str!(
        "fixtures/json-codec-word-kernel-body.cinch.inc"
    ));
    run(&consumer);
}

/// Typed, custom and JSON-valued map keys match independent active-reference updates and failures.
#[test]
fn json_map_keys_updates_and_invalid_values_match_independent_observations() {
    run(include_str!("fixtures/json-codec-map.cinch"));
}

/// Raw byte-name normalization, lexical ordering and original custom causes match independent outputs.
#[test]
fn json_map_raw_names_match_independent_observations() {
    run(include_str!("fixtures/json-codec-map-encode.cinch"));
}

/// Persistent maps, source-backed key views, retained callbacks and every large lookup remain bounded.
#[test]
fn json_map_owners_callbacks_and_large_values_are_bounded() {
    run(include_str!("fixtures/json-codec-map-control.cinch"));
}

/// Every public/private declaration and constructor owns canonical standard-library documentation.
#[test]
fn json_codec_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Json.Dynamic",
            include_str!("../prelude/Text/Json/Dynamic.cinch"),
        ),
        (
            "Text.Json.Legacy",
            include_str!("../prelude/Text/Json/Legacy.cinch"),
        ),
        (
            "Text.Json.Codec",
            include_str!("../prelude/Text/Json/Codec.cinch"),
        ),
        (
            "Text.Json.Binary32",
            include_str!("../prelude/Text/Json/Binary32.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("{}: missing documentation", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}

/// Compile the actual private vector classifier for little- and big-endian foreign objects.
#[test]
fn rust_json_field_kernel_compiles_for_wasm_arm_and_big_endian_targets() {
    let source = include_str!("../prelude/Text/Json/Codec.cinch");
    let start = source.find("  prim folded-ascii16 ::").unwrap();
    let remaining = &source[start..];
    let end = remaining.find("\n  -- |").unwrap_or(remaining.len());
    let mut kernels = String::from("-- | Isolated JSON vector classification.\nmodule Main where\n  import Data.Machine\n  -- | Actual complete-interval load-only classifier.\n");
    kernels.push_str(&remaining[..end]);
    for name in ["key-ascii16", "key-ascii-byte"] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str("\n  -- | Actual complete-interval key-text classifier.\n");
        kernels.push_str(&rest[..end]);
    }

    let binary32 = include_str!("../prelude/Text/Json/Binary32.cinch");
    for name in ["binary32-bits", "binary32-value"] {
        let start = binary32.find(&format!("  prim {name} ::")).unwrap();
        let rest = &binary32[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str("\n  -- | Actual binary32 representation primitive.\n");
        kernels.push_str(&rest[..end]);
    }

    kernels.push_str("\n  -- | Retain the source owner through its checked private load.\n  keep :: Bytes -> U8 -> U8\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep the complete classifier reachable without claiming foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 16 in let a = keep bytes (folded-ascii16 (bytes-data bytes) (bytes-data bytes) 0) in let b = keep bytes (folded-ascii16 (bytes-data bytes) (bytes-data bytes) 1) in let ascii = keep bytes (key-ascii16 (bytes-data bytes)) in let fringe = keep bytes (key-ascii-byte (bytes-data bytes)) in let bits = wrapping-i64-to-u32 (bytes-data bytes) in if eq-u8 a b && eq-u8 ascii 1 && eq-u8 fringe 1 && eq-u32 (binary32-bits (binary32-value bits)) bits then 0 else 1\n");
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
        let input = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&input, "json-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<16 x i8>") && ir.contains("load <16 x i8>"));
        assert!(ir.contains("bitcast i32") && ir.contains("bitcast float"));
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, &ir).unwrap();
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

/// Ordinary bool fields retain every quoted policy, partial-prefix and formatting observation.
#[test]
fn json_quoted_bool_errors_and_prefixes_match_independent_observations() {
    run(include_str!("fixtures/json-codec-error-bool.cinch"));
}

/// Addressable methods preserve callback counts and the original reporting-disabled descriptor refusal.
#[test]
fn json_quoted_pointer_methods_and_retained_refusal_match_observations() {
    run(include_str!("fixtures/json-codec-error-pointer.cinch"));
}

/// Quoted strings retain scalar policy, exact escaped output and semantic context.
#[test]
fn json_quoted_string_errors_and_prefixes_match_independent_observations() {
    run(include_str!("fixtures/json-codec-error-string.cinch"));
}

/// Quoted integral fields retain all policy and formatting combinations.
#[test]
fn json_quoted_integer_outputs_match_independent_observations() {
    run(include_str!("fixtures/json-codec-error-integer.cinch"));
}

/// Public encoding outcomes retain independent bytes and directly inspectable error families.
#[test]
fn json_encoding_outcome_owners_and_error_families_are_bounded() {
    run(include_str!("fixtures/json-codec-error-owner-control.cinch"));
}

/// JSON and text methods retain independent receiver identity and exact original semantic context.
#[test]
fn json_method_error_families_prefixes_and_context_match_observations() {
    run(include_str!("fixtures/json-codec-error-methods.cinch"));
}

/// Named reads retain optional type metadata, root records, partial values and callback input bytes.
#[test]
fn json_update_error_identity_records_and_continuation_match_observations() {
    run(include_str!("fixtures/json-codec-error-updates.cinch"));
}
