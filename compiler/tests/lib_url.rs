//! Byte-preserving URL escaping, independent fixture observations and copied ownership.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Name shallow long-input groups so poisoned quarantine can be bounded per process.
fn group_name(index: usize) -> String {
    let suffix = if index < 26 { ((b'a' + index as u8) as char).to_string() }
        else { format!("a{}", (b'a' + (index - 26) as u8) as char) };
    format!("long-batch-{suffix}")
}

/// Render the same independent fixture stream and bounded groups as the standalone validator.
fn codec_consumer() -> (String, Vec<String>) {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/url-vectors.json")).unwrap();
    let mut encoded = String::new();
    for row in fixture["encoded"].as_array().unwrap() {
        for value in [row["mode"].to_string(), row["input"].as_str().unwrap().into(), row["output"].as_str().unwrap().into()] {
            encoded.push_str(&value); encoded.push('\0');
        }
    }
    let mut decoded = String::new();
    for row in fixture["decoded"].as_array().unwrap() {
        for value in [row["mode"].to_string(), row["input"].as_str().unwrap().into(), row["output"].as_str().unwrap().into(), row["error"].as_str().unwrap().into(), row["offset"].to_string()] {
            decoded.push_str(&value); decoded.push('\0');
        }
    }
    let checks = fixture["long"].as_array().unwrap().iter().map(|row| format!(
        "long-case {} {} {} {} {} {}", row["mode"], row["size"], row["pattern"], row["input_hash"], row["output_size"], row["output_hash"])).collect::<Vec<_>>();
    let mut declarations = Vec::new(); let mut calls = Vec::new(); let mut selection = Vec::new();
    let mut groups = vec!["small".to_string()];
    for (index, batch) in checks.chunks(16).enumerate() {
        let name = group_name(index);
        declarations.push(format!("  -- | Check independent bounded large-input observations.\n  {name} :: Unit -> Bool\n  {name} unit = all [{}]", batch.join(",\n    ")));
        calls.push(format!("{name} ()"));
        selection.push(format!("    else if name == \"{name}\" then {name} ()"));
        groups.push(name);
    }
    let literal = |s: &str| serde_json::to_string(s).unwrap().replace("\\u0000", "\\u{0}");
    (include_str!("fixtures/url-codec-consumer.cinch.in")
        .replace("__ENCODED__", &literal(&encoded)).replace("__DECODED__", &literal(&decoded))
        .replace("__LONG_DECLS__", &declarations.join("\n\n"))
        .replace("__LONG_CALLS__", &calls.join(", "))
        .replace("__LONG_SELECT__", &selection.join("\n")), groups)
}

/// Build source namespaces once, then verify ordinary memory and all bounded poisoned groups.
fn run(source: &str, groups: &[String]) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Url.Codec", include_str!("../prelude/Text/Url/Codec.cinch")),
        ("Text.Url.Error", include_str!("../prelude/Text/Url/Error.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] { std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap(); }
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile { reclaim: true, rc_debug: true, ..cinch::core::TargetProfile::native() };
    common::drive_with_profile(Mode::Build, cinch::check::StageMode::Refresh, directory.path(), "Main.cinch", Some(executable.clone()), &profile).unwrap();
    for (debug, group) in std::iter::once((false, "all")).chain(groups.iter().map(|group| (true, group.as_str()))) {
        let output = std::process::Command::new(&executable).arg(group)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" }).env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M").env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" }).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}, group={group}: {}\n{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Every byte/context, malformed escape, Unicode boundary and large run matches independent observations.
#[test]
fn url_component_corpus_and_owned_buffers_match_independent_results() {
    let (source, groups) = codec_consumer(); run(&source, &groups);
}

/// Every module-owned declaration carries canonical documentation without borrowed language comparisons.
#[test]
fn url_declarations_have_canonical_documentation() {
    for (name, source) in [("Text.Url.Codec", include_str!("../prelude/Text/Url/Codec.cinch")), ("Text.Url.Error", include_str!("../prelude/Text/Url/Error.cinch"))] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some()); assert!(parsed.diagnostics.is_empty()); assert!(parsed.orphans.is_empty());
        let mut declarations = Vec::new(); cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration.doc.as_ref().unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
            assert!(!doc.legacy); let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}
