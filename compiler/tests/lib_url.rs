//! Byte-preserving URL escaping, independent fixture observations and copied ownership.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Name shallow long-input groups so poisoned quarantine can be bounded per process.
fn group_name(index: usize) -> String {
    let suffix = if index < 26 {
        ((b'a' + index as u8) as char).to_string()
    } else {
        format!("a{}", (b'a' + (index - 26) as u8) as char)
    };
    format!("long-batch-{suffix}")
}

/// Render the same independent fixture stream and bounded groups as the standalone validator.
fn codec_consumer() -> (String, Vec<String>) {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/url-vectors.json")).unwrap();
    let mut encoded = String::new();
    for row in fixture["encoded"].as_array().unwrap() {
        for value in [
            row["mode"].to_string(),
            row["input"].as_str().unwrap().into(),
            row["output"].as_str().unwrap().into(),
        ] {
            encoded.push_str(&value);
            encoded.push('\0');
        }
    }
    let mut decoded = String::new();
    for row in fixture["decoded"].as_array().unwrap() {
        for value in [
            row["mode"].to_string(),
            row["input"].as_str().unwrap().into(),
            row["output"].as_str().unwrap().into(),
            row["error"].as_str().unwrap().into(),
            row["offset"].to_string(),
        ] {
            decoded.push_str(&value);
            decoded.push('\0');
        }
    }
    let checks = fixture["long"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "long-case {} {} {} {} {} {}",
                row["mode"],
                row["size"],
                row["pattern"],
                row["input_hash"],
                row["output_size"],
                row["output_hash"]
            )
        })
        .collect::<Vec<_>>();
    let mut declarations = Vec::new();
    let mut calls = Vec::new();
    let mut selection = Vec::new();
    let mut groups = vec!["small".to_string()];
    for (index, batch) in checks.chunks(16).enumerate() {
        let name = group_name(index);
        declarations.push(format!("  -- | Check independent bounded large-input observations.\n  {name} :: Unit -> Bool\n  {name} unit = all [{}]", batch.join(",\n    ")));
        calls.push(format!("{name} ()"));
        selection.push(format!("    else if name == \"{name}\" then {name} ()"));
        groups.push(name);
    }
    let literal = |s: &str| {
        serde_json::to_string(s)
            .unwrap()
            .replace("\\u0000", "\\u{0}")
    };
    (
        include_str!("fixtures/url-codec-consumer.cinch.in")
            .replace("__ENCODED__", &literal(&encoded))
            .replace("__DECODED__", &literal(&decoded))
            .replace("__LONG_DECLS__", &declarations.join("\n\n"))
            .replace("__LONG_CALLS__", &calls.join(", "))
            .replace("__LONG_SELECT__", &selection.join("\n")),
        groups,
    )
}

/// Render all public capability groups without placing deep fixture lists in constant evaluation.
fn url_consumer() -> (String, Vec<String>) {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/url-vectors.json")).unwrap();
    let literal = |text: &str| {
        serde_json::to_string(text)
            .unwrap()
            .replace("\\u0000", "\\u{0}")
    };
    let mut declarations = Vec::new();
    let mut selection =
        vec!["    else if name == \"ownership\" then ownership-cases ()".to_string()];
    let mut calls = vec!["ownership-cases".to_string()];
    let mut groups = vec!["ownership".to_string()];
    for (section, runner) in [
        ("parsing", "parsing-from"),
        ("queries", "query-from"),
        ("resolution", "resolution-from"),
        ("manual", "manual-from"),
        ("joining", "joining-from"),
    ] {
        for (index, rows) in fixture[section].as_array().unwrap().chunks(64).enumerate() {
            let mut fields = Vec::<String>::new();
            for row in rows {
                let string = |key: &str| row[key].as_str().unwrap().to_string();
                let values = |key: &str| {
                    row[key]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|value| value.as_str().unwrap().to_string())
                        .collect::<Vec<_>>()
                };
                match section {
                    "manual" => fields.extend(
                        row.as_array()
                            .unwrap()
                            .iter()
                            .map(|value| value.as_str().unwrap().to_string()),
                    ),
                    "parsing" => {
                        fields.extend([
                            string("input"),
                            if row["request"].as_bool().unwrap() {
                                "1".into()
                            } else {
                                "0".into()
                            },
                            string("error"),
                            string("context"),
                        ]);
                        fields.extend(values("fields"));
                    }
                    "resolution" => {
                        fields.extend([
                            string("base"),
                            string("reference"),
                            string("error"),
                            string("context"),
                        ]);
                        fields.extend(values("fields"));
                    }
                    "queries" => fields.extend([
                        string("input"),
                        string("output"),
                        string("error"),
                        row["size"].to_string(),
                    ]),
                    _ => {
                        let elements = values("elements");
                        fields.extend([string("base"), elements.len().to_string()]);
                        fields.extend(elements);
                        fields.extend([string("output"), string("error")]);
                        fields.extend(values("fields"));
                    }
                }
            }
            let suffix = if index < 26 {
                ((b'a' + index as u8) as char).to_string()
            } else {
                format!("a{}", (b'a' + (index - 26) as u8) as char)
            };
            let name = format!("{section}-batch-{suffix}");
            let stream = fields
                .iter()
                .map(|value| format!("{value}\0"))
                .collect::<String>();
            declarations.push(format!("  -- | Check independent bounded capability observations.\n  {name} :: Unit -> Nat\n  {name} unit = {runner} (string-to-bytes {}) (u32-to-nat (runtime-size 0))",literal(&stream)));
            selection.push(format!("    else if name == \"{name}\" then {name} ()"));
            calls.push(name.clone());
            groups.push(name);
        }
    }
    for (index, row) in fixture["query_long"].as_array().unwrap().iter().enumerate() {
        let suffix = if index < 26 {
            ((b'a' + index as u8) as char).to_string()
        } else {
            format!("a{}", (b'a' + (index - 26) as u8) as char)
        };
        let name = format!("stress-batch-{suffix}");
        let mut args = [
            "count",
            "mode",
            "input_size",
            "input_hash",
            "output_size",
            "output_hash",
            "size",
        ]
        .iter()
        .map(|key| row[*key].to_string())
        .collect::<Vec<_>>();
        args.push(literal(row["error"].as_str().unwrap()));
        declarations.push(format!("  -- | Check independent query stress boundaries.\n  {name} :: Unit -> Nat\n  {name} unit = if query-stress-case {} then 0 else 1",args.join(" ")));
        selection.push(format!("    else if name == \"{name}\" then {name} ()"));
        calls.push(name.clone());
        groups.push(name);
    }
    let mut blocks = Vec::new();
    for (block, batch) in calls.chunks(8).enumerate() {
        let name = format!("check-block-{}", (b'a' + block as u8) as char);
        let mut body = format!(
            "  -- | Check a shallow capability batch.\n  {name} :: Unit -> Nat\n  {name} unit =\n"
        );
        for (index, call) in batch.iter().enumerate() {
            body.push_str(&format!(
                "    let result = {call} () in if result /= 0 then {} * 1000000 + result else\n",
                block * 8 + index + 1
            ));
        }
        body.push_str("    0");
        declarations.push(body);
        blocks.push(name);
    }
    let mut body="  -- | Check all shallow independent groups.\n  all-groups :: Unit -> Nat\n  all-groups unit =\n".to_string();
    for name in blocks {
        body.push_str(&format!(
            "    let result = {name} () in if result /= 0 then result else\n"
        ));
    }
    body.push_str("    0");
    declarations.push(body);
    (
        include_str!("fixtures/url-consumer.cinch.in")
            .replace("__GROUP_DECLS__", &declarations.join("\n\n"))
            .replace("__GROUP_SELECT__", &selection.join("\n")),
        groups,
    )
}

/// Build source namespaces once, then verify ordinary memory and all bounded poisoned groups.
fn run(source: &str, groups: &[String]) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Text.Url.Codec",
            include_str!("../prelude/Text/Url/Codec.cinch"),
        ),
        (
            "Text.Url.Error",
            include_str!("../prelude/Text/Url/Error.cinch"),
        ),
        ("Text.Url", include_str!("../prelude/Text/Url.cinch")),
        (
            "Text.Url.Scan",
            include_str!("../prelude/Text/Url/Scan.cinch"),
        ),
        ("Text.Url.Ip", include_str!("../prelude/Text/Url/Ip.cinch")),
        (
            "Text.Url.Query",
            include_str!("../prelude/Text/Url/Query.cinch"),
        ),
        (
            "Text.Url.Userinfo",
            include_str!("../prelude/Text/Url/Userinfo.cinch"),
        ),
        (
            "Text.Url.Model",
            include_str!("../prelude/Text/Url/Model.cinch"),
        ),
        (
            "Text.Url.Parser",
            include_str!("../prelude/Text/Url/Parser.cinch"),
        ),
        (
            "Text.Url.Resolve",
            include_str!("../prelude/Text/Url/Resolve.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
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
    for (debug, group) in
        std::iter::once((false, "all")).chain(groups.iter().map(|group| (true, group.as_str())))
    {
        let output = std::process::Command::new(&executable)
            .arg(group)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "debug={debug}, group={group}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Every byte/context, malformed escape, Unicode boundary and large run matches independent observations.
#[test]
fn url_component_corpus_and_owned_buffers_match_independent_results() {
    let (source, groups) = codec_consumer();
    run(&source, &groups);
}

/// Parsing, manual fields, query ordering, limits and reference/join behavior match independent results.
#[test]
fn url_full_public_capabilities_and_ownership_match_independent_results() {
    let (source, groups) = url_consumer();
    run(&source, &groups);
}

/// Every module-owned declaration carries canonical documentation without borrowed language comparisons.
#[test]
fn url_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Url", include_str!("../prelude/Text/Url.cinch")),
        (
            "Text.Url.Codec",
            include_str!("../prelude/Text/Url/Codec.cinch"),
        ),
        (
            "Text.Url.Error",
            include_str!("../prelude/Text/Url/Error.cinch"),
        ),
        (
            "Text.Url.Scan",
            include_str!("../prelude/Text/Url/Scan.cinch"),
        ),
        ("Text.Url.Ip", include_str!("../prelude/Text/Url/Ip.cinch")),
        (
            "Text.Url.Query",
            include_str!("../prelude/Text/Url/Query.cinch"),
        ),
        (
            "Text.Url.Userinfo",
            include_str!("../prelude/Text/Url/Userinfo.cinch"),
        ),
        (
            "Text.Url.Model",
            include_str!("../prelude/Text/Url/Model.cinch"),
        ),
        (
            "Text.Url.Parser",
            include_str!("../prelude/Text/Url/Parser.cinch"),
        ),
        (
            "Text.Url.Resolve",
            include_str!("../prelude/Text/Url/Resolve.cinch"),
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

/// The actual SIMD, SWAR, exact-load and packed-name arithmetic remains legal on foreign objects.
/// This checks the Rust emitter explicitly; selected host frontend tests never fall back.
#[test]
fn url_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let codec = include_str!("../prelude/Text/Url/Codec.cinch");
    let scan = include_str!("../prelude/Text/Url/Scan.cinch");
    let mut kernels = String::from("-- | Isolated actual URL kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n");
    for (source, names) in [
        (
            codec,
            vec![
                "load-eight",
                "load-one",
                "classify-one",
                "classify-eight",
                "decode-flags-eight",
            ],
        ),
        (scan, vec!["mask-eight", "population"]),
    ] {
        for name in names {
            let start = source.find(&format!("  prim {name} ::")).unwrap();
            let rest = &source[start..];
            let end = rest.find("\n  -- |").unwrap_or(rest.len());
            kernels
                .push_str("\n  -- | Actual retained URL classification or exact-load primitive.\n");
            kernels.push_str(&rest[..end]);
        }
    }
    kernels.push_str("\n  -- | Hold backing storage across a private exact load.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep every byte-policy kernel reachable without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 8 in let address = bytes-data bytes in let word = keep bytes (load-eight address) in let byte = keep bytes (load-one address) in let single = classify-one byte byte in let encoded = classify-eight word byte in let decoded = decode-flags-eight word byte in let mask = mask-eight word single decoded in let count = population mask in if eq-u64 count encoded then 0 else 1\n");
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
                    cinch::driver::check_sources_profile(&input, "url-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<8 x i8>"));
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
