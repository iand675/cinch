//! Read the exact sources used by the bootstrap, including generated host and ABI modules.
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

pub fn compiler_sources(root: &Path) -> BTreeMap<String, String> {
    let output = Command::new("python3")
        .current_dir(root.join("tooling"))
        .args(["-B", "-c", r#"
import json
import bootstrap

sources = bootstrap.collect_compiler_sources()
print(json.dumps({name: data.decode("utf-8") for name, data in sources.items()}, ensure_ascii=False))
"#])
        .output()
        .expect("collect bootstrap compiler sources");
    assert!(output.status.success(), "bootstrap source inventory failed: {}", String::from_utf8_lossy(&output.stderr));
    let text = String::from_utf8(output.stdout).expect("UTF-8 compiler inventory");
    let cinch::lsp::Json::Obj(files) = cinch::lsp::parse_json(&text).expect("compiler source inventory") else {
        panic!("expected compiler source inventory by filename");
    };
    files.into_iter().map(|(name, value)| match value {
        cinch::lsp::Json::Str(source) => (name, source),
        _ => panic!("expected compiler source text for {name}"),
    }).collect()
}
