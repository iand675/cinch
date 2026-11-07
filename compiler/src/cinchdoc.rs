//! Documentation comments, the canonical `.cinchdoc` artifact, symbol lookup,
//! and the documentation test runner.
//!
//! The parser is source-order and indentation-aware, not a compiler AST pass.
//! It agrees with the contract in
//! `specifications/documentation-comments-specification.md` and is the
//! implementation behind `cinch doc`.

use crate::highlight;
use regex::Regex;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::LazyLock;
use std::thread;
use std::time::{Duration, Instant};

pub const SCHEMA: i32 = 1;

const RESERVED: &[&str] = &[
    "module", "import", "export", "data", "record", "codata", "signature", "instance", "foreign",
    "where", "let", "letrec", "in", "case", "of", "if", "then", "else", "do", "type", "opaque",
    "forall", "sigma", "using", "as", "infix", "infixl", "infixr", "stage", "macro", "generate",
    "rewrite", "upgrade", "law", "contract", "prim", "implicit",
];

fn known_tag(name: &str) -> bool {
    matches!(
        name,
        "since"
            | "see"
            | "deprecated"
            | "experimental"
            | "param"
            | "arg"
            | "typeparam"
            | "field"
            | "returns"
            | "safety"
            | "errors"
            | "law"
            | "effect"
    )
}

static RE_LINE_COMMENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*--\s?(.*)$").unwrap());
static RE_LEADING_KIND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^--\s*\|").unwrap());
static RE_TRAILING_KIND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^--\s*\^").unwrap());
static RE_LEADING_MARKER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^--\s*\|\s?").unwrap());
static RE_TRAILING_MARKER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^--\s*\^\s?").unwrap());
static RE_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*@([A-Za-z][\w-]*)(?:\s+(.*))?$").unwrap());
static RE_FENCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(`{3,})(.*)$").unwrap());
static RE_FENCE_CLOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(`{3,})\s*$").unwrap());
static RE_BLANK_RUN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n{3,}").unwrap());
static RE_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\(symbol:([^)\s]+)\)").unwrap());
static RE_PLAIN_LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\([^)]+\)").unwrap());
static RE_CODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`([^`]*)`").unwrap());
static RE_EMPHASIS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[*_]{1,2}([^*_\n]+)[*_]{1,2}").unwrap());
static RE_HTML: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"</?[A-Za-z][^>]*>").unwrap());
static RE_WS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static RE_TEST_SECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^---\s*(input|stdin|args|expect|error|exit)\s*$").unwrap()
});
static RE_INT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-?\d+$").unwrap());
static RE_BINDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\(([A-Za-z_][\w'-]*)\s*::\s*").unwrap());
static RE_INLINE_CODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`+[^`\n]*`+").unwrap());
static RE_MODULE_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*module\s+([A-Za-z][\w.']*)").unwrap());
static RE_FOREIGN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*foreign\s+import\s+\w+\s+"[^"]*"\s+([A-Za-z_][\w'-]*)\s*::"#).unwrap()
});
static RE_INSTANCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?:implicit\s+)?instance\s+([A-Za-z_][\w'-]*)").unwrap());
static RE_FIXITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(infixl|infixr|infix)\s+\d+\s+(.+)$").unwrap());
static RE_FIXITY_SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s*,\s*").unwrap());
static RE_CONTAINER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(opaque\s+)?(data|record|codata|signature|type)\s+([A-Za-z_][\w'-]*)")
        .unwrap()
});
static RE_OTHER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(stage|macro|generate|rewrite|upgrade|law|contract|prim)\s+([A-Za-z_][\w'-]*)")
        .unwrap()
});
static RE_SIGNATURE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*([A-Za-z_][\w'-]*|\([^)]+\))\s*::").unwrap());
static RE_EQUATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*([A-Za-z_][\w'-]*)\s*(?:[^=]*)=.*$").unwrap());
static RE_WHERE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\bwhere\b").unwrap());
static RE_HASH_LINE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*#\[").unwrap());
static RE_TRIM_DASHES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[-= ]+$").unwrap());

#[derive(Clone, Debug)]
struct CommentBlock {
    kind: &'static str,
    start: usize,
    end: usize,
    text: String,
    marker: &'static str,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub text: String,
    pub target: String,
    pub resolved: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct MathSpan {
    pub tex: String,
    pub display: bool,
    pub line: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestCase {
    pub name: String,
    pub mode: String,
    pub input: String,
    pub stdin: Option<String>,
    pub args: Vec<String>,
    pub expect: Option<String>,
    pub error: Option<String>,
    pub exit: i32,
    pub line: usize,
    pub fence: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Doc {
    pub summary: String,
    pub markdown: String,
    pub tags: BTreeMap<String, Value>,
    pub links: Vec<Link>,
    pub tests: Vec<TestCase>,
    pub math: Vec<MathSpan>,
    pub legacy: bool,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct Decl {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub line: usize,
    pub indent: usize,
    pub signature: String,
    pub doc: Option<Doc>,
    pub children: Vec<Decl>,
}

#[derive(Clone, Debug, Serialize)]
struct Parameter {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    doc: String,
}

#[derive(Clone, Debug)]
pub struct ModuleDoc {
    pub module: String,
    pub source: String,
    pub source_sha256: String,
    pub doc: Option<Doc>,
    pub declarations: Vec<Decl>,
    pub orphans: Vec<Value>,
    pub diagnostics: Vec<Value>,
}

impl Decl {
    pub fn parameters(&self) -> Vec<Parameter> {
        let mut found: Vec<Parameter> = Vec::new();
        let mut at: HashMap<String, usize> = HashMap::new();
        if let Some(doc) = &self.doc {
            if let Some(items) = doc.tags.get("params").and_then(Value::as_array) {
                for item in items {
                    let name = item.get("name").and_then(Value::as_str).unwrap_or("").to_string();
                    let text = item.get("text").and_then(Value::as_str).unwrap_or("").to_string();
                    if let Some(index) = at.get(&name) {
                        found[*index].doc = text;
                    } else {
                        at.insert(name.clone(), found.len());
                        found.push(Parameter { name, ty: String::new(), doc: text });
                    }
                }
            }
        }
        for (name, ty) in named_parameters(&self.signature) {
            if let Some(index) = at.get(&name) {
                found[*index].ty = ty;
            } else {
                at.insert(name.clone(), found.len());
                found.push(Parameter { name, ty, doc: String::new() });
            }
        }
        found
    }

    fn to_value(&self) -> Value {
        let type_parameters = self
            .doc
            .as_ref()
            .and_then(|doc| doc.tags.get("typeParams"))
            .cloned()
            .unwrap_or_else(|| json!([]));
        let fields = self
            .doc
            .as_ref()
            .and_then(|doc| doc.tags.get("fields"))
            .cloned()
            .unwrap_or_else(|| json!([]));
        json!({
            "id": self.id,
            "kind": self.kind,
            "name": self.name,
            "signature": self.signature,
            "doc": self.doc.as_ref().map(|doc| serde_json::to_value(doc).unwrap()).unwrap_or(Value::Null),
            "parameters": self.parameters(),
            "typeParameters": type_parameters,
            "fields": fields,
            "children": self.children.iter().map(Decl::to_value).collect::<Vec<_>>(),
            "source": {"line": self.line},
        })
    }
}

impl ModuleDoc {
    pub fn to_value(&self) -> Value {
        json!({
            "schema": SCHEMA,
            "module": self.module,
            "source": self.source,
            "sourceSha256": self.source_sha256,
            "doc": self.doc.as_ref().map(|doc| serde_json::to_value(doc).unwrap()).unwrap_or(Value::Null),
            "declarations": self.declarations.iter().map(Decl::to_value).collect::<Vec<_>>(),
            "orphans": self.orphans,
            "diagnostics": self.diagnostics,
        })
    }
}

fn named_parameters(signature: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for capture in RE_BINDER.captures_iter(signature) {
        let name = capture.get(1).unwrap().as_str().to_string();
        let start = capture.get(0).unwrap().end();
        let bytes = signature.as_bytes();
        let mut depth = 1usize;
        let mut index = start;
        while index < bytes.len() && depth > 0 {
            match bytes[index] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            index += 1;
        }
        if depth == 0 {
            found.push((name, signature[start..index - 1].trim().to_string()));
        }
    }
    found
}

fn strip_line_comment(line: &str) -> String {
    RE_LINE_COMMENT
        .captures(line)
        .and_then(|capture| capture.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| line.to_string())
}

fn line_comment_kind(line: &str) -> Option<&'static str> {
    let stripped = line.trim_start();
    if !stripped.starts_with("--") {
        return None;
    }
    if RE_LEADING_KIND.is_match(stripped) {
        return Some("leading");
    }
    if RE_TRAILING_KIND.is_match(stripped) {
        return Some("trailing");
    }
    Some("legacy")
}

fn strip_leading_marker(line: &str, kind: &str) -> String {
    let stripped = line.trim_start();
    match kind {
        "leading" => RE_LEADING_MARKER.replace(stripped, "").to_string(),
        "trailing" => RE_TRAILING_MARKER.replace(stripped, "").to_string(),
        _ => strip_line_comment(stripped),
    }
}

fn block_comment(lines: &[&str], start: usize) -> Option<(String, usize, &'static str)> {
    let first = lines.get(start)?;
    let stripped = first.trim_start();
    let marker = if stripped.starts_with("{-|") {
        "|"
    } else if stripped.starts_with("{-^") {
        "^"
    } else {
        return None;
    };
    let opener = if marker == "|" { "{-|" } else { "{-^" };
    let at = first.find(opener)?;
    let rest = &first[at + 3..];
    let mut chunks = Vec::new();
    if let Some(index) = rest.find("-}") {
        chunks.push(rest[..index].to_string());
        return Some((chunks.join("\n"), start, marker));
    }
    chunks.push(rest.to_string());
    for (offset, line) in lines.iter().enumerate().skip(start + 1) {
        if let Some(index) = line.find("-}") {
            chunks.push(line[..index].to_string());
            return Some((chunks.join("\n"), offset, marker));
        }
        chunks.push((*line).to_string());
    }
    Some((chunks.join("\n"), lines.len().saturating_sub(1), marker))
}

fn comment_lines(lines: &[&str]) -> HashSet<usize> {
    let mut marked = HashSet::new();
    let mut index = 0;
    while index < lines.len() {
        let stripped = lines[index].trim_start();
        if stripped.starts_with("--") {
            marked.insert(index);
            index += 1;
            continue;
        }
        if stripped.starts_with("{-") {
            let mut end = index;
            let mut depth = 0i32;
            let mut cursor = index;
            'outer: while cursor < lines.len() {
                let bytes = lines[cursor].as_bytes();
                let mut pos = 0;
                while pos < bytes.len() {
                    if bytes.get(pos) == Some(&b'{') && bytes.get(pos + 1) == Some(&b'-') {
                        depth += 1;
                        pos += 2;
                    } else if bytes.get(pos) == Some(&b'-') && bytes.get(pos + 1) == Some(&b'}') {
                        depth -= 1;
                        pos += 2;
                        if depth == 0 {
                            end = cursor;
                            break 'outer;
                        }
                    } else {
                        pos += 1;
                    }
                }
                cursor += 1;
            }
            for line in index..=end.min(lines.len().saturating_sub(1)) {
                marked.insert(line);
            }
            index = end + 1;
            continue;
        }
        index += 1;
    }
    marked
}

fn scan_blocks(lines: &[&str], legacy: bool) -> Vec<CommentBlock> {
    let mut blocks = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let stripped = lines[index].trim_start();
        if stripped.starts_with("{-|") || stripped.starts_with("{-^") {
            if let Some((text, end, marker)) = block_comment(lines, index) {
                blocks.push(CommentBlock {
                    kind: if marker == "|" { "leading" } else { "trailing" },
                    start: index,
                    end,
                    text,
                    marker,
                });
                index = end + 1;
                continue;
            }
        }
        if let Some(kind) = line_comment_kind(lines[index]) {
            let start = index;
            let mut parts = vec![strip_leading_marker(lines[index], kind)];
            index += 1;
            while index < lines.len() {
                match line_comment_kind(lines[index]) {
                    Some("leading") | Some("trailing") | None => break,
                    Some(_) => {
                        parts.push(strip_line_comment(lines[index]));
                        index += 1;
                    }
                }
            }
            blocks.push(CommentBlock {
                kind,
                start,
                end: index - 1,
                text: parts.join("\n"),
                marker: kind,
            });
            continue;
        }
        index += 1;
    }
    if !legacy {
        blocks.retain(|block| block.kind != "legacy");
    }
    blocks
}

fn compact(text: &str) -> String {
    text.lines()
        .filter(|line| !RE_TRIM_DASHES.is_match(line.trim()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn plain(text: &str) -> String {
    let text = RE_PLAIN_LINK.replace_all(text, "$1");
    let text = RE_CODE.replace_all(&text, "$1");
    let text = RE_EMPHASIS.replace_all(&text, "$1");
    let text = RE_HTML.replace_all(&text, "");
    RE_WS.replace_all(&text, " ").trim().to_string()
}

fn parse_test(info: &str, body: &[String], line: usize) -> Result<Option<TestCase>, String> {
    let mut name = "test".to_string();
    for word in info.split_whitespace() {
        if let Some(rest) = word.strip_prefix("name=") {
            name = rest.trim_matches('"').trim_matches('\'').to_string();
        }
    }
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for key in ["input", "stdin", "args", "expect", "error", "exit"] {
        sections.insert(key.to_string(), Vec::new());
    }
    let mut current = "input".to_string();
    for raw in body {
        if let Some(capture) = RE_TEST_SECTION.captures(raw.trim()) {
            current = capture.get(1).unwrap().as_str().to_string();
            continue;
        }
        sections.get_mut(&current).unwrap().push(raw.clone());
    }
    let mut mode = "run";
    if sections["expect"].is_empty() && sections["error"].is_empty() {
        let mut prompt_input = Vec::new();
        let mut prompt_expected = Vec::new();
        let mut in_prompt = false;
        for raw in body {
            if let Some(rest) = raw.strip_prefix('>') {
                in_prompt = true;
                prompt_input.push(rest.trim_start().to_string());
            } else if in_prompt {
                prompt_expected.push(raw.clone());
            }
        }
        if !prompt_input.is_empty() {
            sections.insert("input".into(), prompt_input);
            sections.insert("expect".into(), prompt_expected);
            mode = "transcript";
        }
    }
    let input = sections["input"].join("\n").trim_matches('\n').to_string();
    let expect = sections["expect"].join("\n").trim_matches('\n').to_string();
    let error = sections["error"].join("\n").trim_matches('\n').to_string();
    if input.is_empty() && expect.is_empty() && error.is_empty() {
        return Ok(None);
    }
    let args = if sections["args"].is_empty() {
        Vec::new()
    } else {
        shlex::split(&sections["args"].join("\n").trim()).unwrap_or_default()
    };
    let stdin = if sections["stdin"].is_empty() {
        None
    } else {
        Some(sections["stdin"].join("\n").trim_matches('\n').to_string())
    };
    let exit_text = sections["exit"].join("\n").trim().to_string();
    if !exit_text.is_empty() && !RE_INT.is_match(&exit_text) {
        return Err(format!(
            "line {line}: --- exit expects an integer, got {exit_text:?}"
        ));
    }
    Ok(Some(TestCase {
        name,
        mode: if error.is_empty() { mode } else { "error" }.to_string(),
        input,
        stdin,
        args,
        expect: if expect.is_empty() { None } else { Some(expect) },
        error: if error.is_empty() { None } else { Some(error) },
        exit: if exit_text.is_empty() {
            0
        } else {
            exit_text.parse().map_err(|_| format!("line {line}: invalid --- exit"))?
        },
        line,
        fence: info.to_string(),
    }))
}

fn extract_tests(markdown: &str, base_line: usize) -> Result<Vec<TestCase>, String> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut tests = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        let Some(open) = RE_FENCE.captures(lines[index]) else {
            index += 1;
            continue;
        };
        let fence_len = open.get(1).unwrap().as_str().len();
        let info = open.get(2).map(|m| m.as_str().trim()).unwrap_or("");
        let mut body = Vec::new();
        let mut end = index + 1;
        while end < lines.len() {
            if let Some(close) = RE_FENCE_CLOSE.captures(lines[end]) {
                if close.get(1).unwrap().as_str().len() >= fence_len {
                    break;
                }
            }
            body.push(lines[end].to_string());
            end += 1;
        }
        let words: Vec<&str> = info.split_whitespace().collect();
        let is_test = fence_len == 3
            && !words.is_empty()
            && ((words[0] == "cinch" && words[1..].contains(&"test"))
                || words[0].starts_with("cinch-test"));
        if is_test {
            if let Some(test) = parse_test(info, &body, base_line + index)? {
                tests.push(test);
            }
        }
        index = end + 1;
    }
    Ok(tests)
}

fn mask_fenced_code(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut out = Vec::with_capacity(lines.len());
    let mut in_fence = false;
    let mut fence_len = 0;
    for line in lines {
        if let Some(open) = RE_FENCE.captures(line) {
            if in_fence && open.get(2).map(|m| m.as_str().trim()).unwrap_or("").is_empty() {
                if open.get(1).unwrap().as_str().len() >= fence_len {
                    in_fence = false;
                }
                out.push(String::new());
                continue;
            }
            if !in_fence {
                in_fence = true;
                fence_len = open.get(1).unwrap().as_str().len();
                out.push(String::new());
                continue;
            }
        }
        out.push(if in_fence { String::new() } else { line.to_string() });
    }
    out.join("\n")
}

fn extract_math(markdown: &str, base_line: usize) -> Vec<MathSpan> {
    let masked = mask_fenced_code(markdown);
    let masked = RE_INLINE_CODE
        .replace_all(&masked, |capture: &regex::Captures<'_>| {
            " ".repeat(capture.get(0).unwrap().as_str().len())
        })
        .to_string();
    let mut math = Vec::new();
    for capture in Regex::new(r"(?s)\$\$(.+?)\$\$").unwrap().captures_iter(&masked) {
        let whole = capture.get(0).unwrap();
        math.push(MathSpan {
            tex: capture.get(1).unwrap().as_str().trim().to_string(),
            display: true,
            line: base_line + masked[..whole.start()].matches('\n').count(),
        });
    }
    let without_display = Regex::new(r"(?s)\$\$(.+?)\$\$")
        .unwrap()
        .replace_all(&masked, |capture: &regex::Captures<'_>| {
            " ".repeat(capture.get(0).unwrap().as_str().len())
        })
        .to_string();
    let bytes = without_display.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'$' || (index > 0 && bytes[index - 1] == b'\\') {
            index += 1;
            continue;
        }
        if bytes.get(index + 1) == Some(&b'$') {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len() && bytes[index] != b'$' && bytes[index] != b'\n' {
            if bytes[index] == b'\\' {
                index += 2;
            } else {
                index += 1;
            }
        }
        if index < bytes.len() && bytes[index] == b'$' {
            let tex = without_display[start + 1..index].trim();
            if !tex.is_empty() {
                math.push(MathSpan {
                    tex: tex.to_string(),
                    display: false,
                    line: base_line + without_display[..start].matches('\n').count(),
                });
            }
            index += 1;
        }
    }
    math.sort_by_key(|item| item.line);
    math
}

fn parse_doc(text: &str, line: usize, legacy: bool) -> Result<Option<Doc>, String> {
    let mut raw = text.trim_matches('\n').to_string();
    if legacy {
        raw = compact(&raw);
    }
    if raw.trim().is_empty() {
        return Ok(None);
    }
    let mut tags: BTreeMap<String, Value> = BTreeMap::new();
    let mut body = Vec::new();
    let mut in_fence = false;
    let mut fence_len = 0;
    for raw_line in raw.lines() {
        if in_fence {
            if let Some(fence) = RE_FENCE.captures(raw_line) {
                let len = fence.get(1).unwrap().as_str().len();
                let rest = fence.get(2).map(|m| m.as_str()).unwrap_or("");
                if rest.trim().is_empty() && len >= fence_len {
                    in_fence = false;
                }
            }
            body.push(raw_line.to_string());
            continue;
        }
        if let Some(fence) = RE_FENCE.captures(raw_line) {
            in_fence = true;
            fence_len = fence.get(1).unwrap().as_str().len();
            body.push(raw_line.to_string());
            continue;
        }
        let Some(capture) = RE_TAG.captures(raw_line) else {
            body.push(raw_line.to_string());
            continue;
        };
        let name = capture.get(1).unwrap().as_str();
        if !known_tag(name) {
            body.push(raw_line.to_string());
            continue;
        }
        let value = capture.get(2).map(|m| m.as_str()).unwrap_or("").trim().to_string();
        match name {
            "param" | "arg" | "typeparam" | "field" => {
                let mut pieces = value.splitn(2, char::is_whitespace);
                let item = json!({
                    "name": pieces.next().unwrap_or(""),
                    "text": pieces.next().unwrap_or("").trim_start(),
                });
                let key = if name == "typeparam" {
                    "typeParams"
                } else if name == "field" {
                    "fields"
                } else {
                    "params"
                };
                tags.entry(key.to_string())
                    .or_insert_with(|| json!([]))
                    .as_array_mut()
                    .unwrap()
                    .push(item);
            }
            "see" => {
                let items = Regex::new(r"[,\s]+").unwrap();
                let list = tags.entry("see".to_string()).or_insert_with(|| json!([]));
                for piece in items.split(&value).filter(|piece| !piece.is_empty()) {
                    list.as_array_mut().unwrap().push(json!(piece));
                }
            }
            "law" | "effect" => {
                let key = format!("{name}s");
                tags.entry(key).or_insert_with(|| json!([])).as_array_mut().unwrap().push(json!(value));
            }
            _ => {
                tags.insert(name.to_string(), if value.is_empty() { json!(true) } else { json!(value) });
            }
        }
    }
    let markdown = RE_BLANK_RUN
        .replace_all(body.join("\n").trim_matches('\n'), "\n\n")
        .to_string();
    if markdown.trim().is_empty() {
        return Ok(None);
    }
    let summary = markdown
        .split("\n\n")
        .find(|paragraph| !paragraph.trim().is_empty())
        .map(plain)
        .unwrap_or_default();
    let links = RE_LINK
        .captures_iter(&markdown)
        .map(|capture| Link {
            text: capture.get(1).unwrap().as_str().to_string(),
            target: capture.get(2).unwrap().as_str().to_string(),
            resolved: true,
        })
        .collect();
    let tests = extract_tests(&markdown, line)?;
    let math = extract_math(&markdown, line);
    Ok(Some(Doc { summary, markdown, tags, links, tests, math, legacy, line }))
}

fn module_name(lines: &[&str], comment_lines: &HashSet<usize>) -> (String, Option<usize>) {
    for (index, line) in lines.iter().enumerate() {
        if comment_lines.contains(&index) {
            continue;
        }
        if let Some(capture) = RE_MODULE_NAME.captures(line) {
            return (capture.get(1).unwrap().as_str().to_string(), Some(index));
        }
    }
    (String::new(), None)
}

fn top_indent(lines: &[&str], module_line: Option<usize>, comment_lines: &HashSet<usize>) -> usize {
    let start = module_line.map(|line| line + 1).unwrap_or(0);
    for (index, line) in lines.iter().enumerate().skip(start) {
        if comment_lines.contains(&index) || line.trim().is_empty() {
            continue;
        }
        let stripped = line.trim();
        if stripped.starts_with("import ")
            || stripped.starts_with("export ")
            || stripped.starts_with('{')
            || stripped.starts_with('}')
        {
            continue;
        }
        return line.len() - line.trim_start_matches(' ').len();
    }
    0
}

#[derive(Clone, Debug)]
struct InterfaceEntry {
    id: String,
    signature: String,
}

fn json_str(value: &Value, key: &str) -> String {
    value.get(key).and_then(Value::as_str).unwrap_or("").to_string()
}

fn interface_index(iface: Option<&Value>) -> HashMap<Vec<String>, InterfaceEntry> {
    let mut index = HashMap::new();
    let Some(iface) = iface else { return index };
    let module = json_str(iface, "module");
    let Some(exports) = iface.get("exports").and_then(Value::as_object) else {
        return index;
    };
    if let Some(values) = exports.get("values").and_then(Value::as_array) {
        for value in values {
            let name = json_str(value, "name");
            let id = value.get("symbol").and_then(Value::as_str).map(str::to_string)
                .unwrap_or_else(|| format!("{module}.{name}"));
            index.insert(
                vec!["value".to_string(), name],
                InterfaceEntry { id, signature: json_str(value, "scheme") },
            );
        }
    }
    if let Some(types) = exports.get("types").and_then(Value::as_array) {
        for ty in types {
            let name = json_str(ty, "name");
            let id = ty.get("identity").and_then(Value::as_str).map(str::to_string)
                .unwrap_or_else(|| format!("{module}.{name}"));
            let params = ty
                .get("params")
                .and_then(Value::as_array)
                .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" "))
                .unwrap_or_default();
            let kind = json_str(ty, "kind");
            index.insert(
                vec!["type".to_string(), name.clone()],
                InterfaceEntry {
                    id,
                    signature: format!("{kind} {name} {params}").trim().to_string(),
                },
            );
            if let Some(ctors) = ty.get("ctors").and_then(Value::as_array) {
                for ctor in ctors {
                    let ctor_name = json_str(ctor, "name");
                    let ctor_id = ctor.get("symbol").and_then(Value::as_str).map(str::to_string)
                        .unwrap_or_else(|| format!("{module}.{name}.{ctor_name}"));
                    index.insert(
                        vec!["constructor".to_string(), name.clone(), ctor_name],
                        InterfaceEntry { id: ctor_id, signature: json_str(ctor, "scheme") },
                    );
                }
            }
            if let Some(fields) = ty.get("fields").and_then(Value::as_array) {
                for field in fields {
                    let field_name = field.get("name").and_then(Value::as_str)
                        .map(str::to_string)
                        .unwrap_or_else(|| field.as_str().unwrap_or("").to_string());
                    index.insert(
                        vec!["field".to_string(), name.clone(), field_name.clone()],
                        InterfaceEntry {
                            id: format!("{module}.{name}.{field_name}"),
                            signature: field.get("ty").and_then(Value::as_str).unwrap_or("").to_string(),
                        },
                    );
                }
            }
        }
    }
    if let Some(signatures) = exports.get("signatures").and_then(Value::as_array) {
        for sig in signatures {
            let name = json_str(sig, "name");
            let id = sig.get("identity").and_then(Value::as_str).map(str::to_string)
                .unwrap_or_else(|| format!("{module}.{name}"));
            let params = sig
                .get("params")
                .and_then(Value::as_array)
                .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(" "))
                .unwrap_or_default();
            if let Some(methods) = sig.get("methods").and_then(Value::as_array) {
                for method in methods {
                    let method_name = json_str(method, "name");
                    index.insert(
                        vec!["method".to_string(), name.clone(), method_name.clone()],
                        InterfaceEntry {
                            id: format!("{id}.{method_name}"),
                            signature: json_str(method, "scheme"),
                        },
                    );
                }
            }
            index.insert(
                vec!["signature".to_string(), name.clone()],
                InterfaceEntry {
                    id,
                    signature: format!("signature {name} {params}").trim().to_string(),
                },
            );
        }
    }
    if let Some(instances) = exports.get("instances").and_then(Value::as_array) {
        for instance in instances {
            let full = json_str(instance, "id");
            let name = full.rsplit('.').next().unwrap_or("").to_string();
            index.insert(
                vec!["instance".to_string(), name.clone()],
                InterfaceEntry {
                    id: if full.is_empty() { format!("{module}.{name}") } else { full },
                    signature: format!(
                        "{name} :: {} {}",
                        json_str(instance, "class"),
                        json_str(instance, "target")
                    )
                    .trim()
                    .to_string(),
                },
            );
        }
    }
    if let Some(foreign) = exports.get("foreign").and_then(Value::as_array) {
        for item in foreign {
            let name = json_str(item, "name");
            let id = item.get("symbol").and_then(Value::as_str).map(str::to_string)
                .unwrap_or_else(|| format!("{module}.{name}"));
            index.insert(
                vec!["foreign".to_string(), name],
                InterfaceEntry { id, signature: json_str(item, "scheme") },
            );
        }
    }
    index
}

struct FlatNode {
    decl: Decl,
    parent: Option<usize>,
}

fn add_decl(
    nodes: &mut Vec<FlatNode>,
    by_line: &mut HashMap<usize, Vec<usize>>,
    containers: &[usize],
    module: &str,
    interface: &HashMap<Vec<String>, InterfaceEntry>,
    kind: &str,
    name: &str,
    line_index: usize,
    indent: usize,
    signature: &str,
    keys: &[&str],
) -> usize {
    let real_keys = if keys.is_empty() { vec![name] } else { keys.to_vec() };
    let lookup_kind = if matches!(kind, "data" | "record" | "codata") { "type" } else { kind };
    let mut key = vec![lookup_kind.to_string()];
    key.extend(real_keys.iter().map(|item| item.to_string()));
    let found = interface.get(&key);
    let parent = containers
        .last()
        .copied()
        .filter(|index| indent > nodes[*index].decl.indent);
    let id = found
        .map(|entry| entry.id.clone())
        .unwrap_or_else(|| match parent {
            Some(index) => format!("{module}.{}.{}", nodes[index].decl.name, name),
            None => format!("{module}.{name}"),
        });
    let final_signature = found
        .filter(|entry| !entry.signature.is_empty())
        .map(|entry| entry.signature.clone())
        .unwrap_or_else(|| signature.to_string());
    let node_index = nodes.len();
    nodes.push(FlatNode {
        decl: Decl {
            id,
            kind: kind.to_string(),
            name: name.to_string(),
            line: line_index + 1,
            indent,
            signature: final_signature,
            doc: None,
            children: Vec::new(),
        },
        parent,
    });
    by_line.entry(line_index).or_default().push(node_index);
    node_index
}

fn scan_declarations(
    lines: &[&str],
    comments: &HashSet<usize>,
    module: &str,
    top: usize,
    interface: &HashMap<Vec<String>, InterfaceEntry>,
) -> (Vec<FlatNode>, HashMap<usize, Vec<usize>>) {
    let mut nodes: Vec<FlatNode> = Vec::new();
    let mut by_line = HashMap::new();
    let mut containers: Vec<usize> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        if comments.contains(&index) || line.trim().is_empty() {
            continue;
        }
        if RE_HASH_LINE.is_match(line) {
            continue;
        }
        let indent = line.len() - line.trim_start_matches(' ').len();
        while let Some(last) = containers.last().copied() {
            if indent <= nodes[last].decl.indent {
                containers.pop();
            } else {
                break;
            }
        }
        if RE_MODULE_NAME.is_match(line) {
            continue;
        }
        if let Some(capture) = RE_FOREIGN.captures(line) {
            add_decl(
                &mut nodes, &mut by_line, &containers, module, interface,
                "foreign", capture.get(1).unwrap().as_str(), index, indent, line.trim(), &[],
            );
            continue;
        }
        if let Some(capture) = RE_INSTANCE.captures(line) {
            let decl = add_decl(
                &mut nodes, &mut by_line, &containers, module, interface,
                "instance", capture.get(1).unwrap().as_str(), index, indent, line.trim(), &[],
            );
            if RE_WHERE.is_match(line) {
                containers.push(decl);
            }
            continue;
        }
        if let Some(capture) = RE_FIXITY.captures(line) {
            for op in RE_FIXITY_SPLIT.split(capture.get(2).unwrap().as_str().trim()) {
                if !op.is_empty() {
                    add_decl(
                        &mut nodes, &mut by_line, &containers, module, interface,
                        "fixity", op, index, indent, line.trim(), &[],
                    );
                }
            }
            continue;
        }
        if let Some(capture) = RE_CONTAINER.captures(line) {
            let keyword = capture.get(2).unwrap().as_str();
            let name = capture.get(3).unwrap().as_str();
            let kind = if keyword == "signature" { "signature" } else { keyword };
            let decl = add_decl(
                &mut nodes, &mut by_line, &containers, module, interface,
                kind, name, index, indent, line.trim(), &[],
            );
            if matches!(keyword, "data" | "record" | "codata" | "signature") && RE_WHERE.is_match(line) {
                containers.push(decl);
            } else if keyword == "record" {
                containers.push(decl);
            }
            continue;
        }
        if let Some(capture) = RE_OTHER.captures(line) {
            add_decl(
                &mut nodes, &mut by_line, &containers, module, interface,
                capture.get(1).unwrap().as_str(), capture.get(2).unwrap().as_str(),
                index, indent, line.trim(), &[],
            );
            continue;
        }
        if let Some(capture) = RE_SIGNATURE.captures(line) {
            let raw_name = capture.get(1).unwrap().as_str();
            let name = raw_name.trim_start_matches('(').trim_end_matches(')');
            let parent = containers
                .last()
                .copied()
                .filter(|index| indent > nodes[*index].decl.indent);
            if let Some(parent) = parent {
                let parent_kind = nodes[parent].decl.kind.as_str();
                if parent_kind == "signature" {
                    let parent_name = nodes[parent].decl.name.clone();
                    add_decl(
                        &mut nodes, &mut by_line, &containers, module, interface,
                        "method", name, index, indent, line.trim(), &[&parent_name, name],
                    );
                    continue;
                }
                if matches!(parent_kind, "data" | "record" | "codata") {
                    let kind = if parent_kind == "record" { "field" } else { "constructor" };
                    let parent_name = nodes[parent].decl.name.clone();
                    add_decl(
                        &mut nodes, &mut by_line, &containers, module, interface,
                        kind, name, index, indent, line.trim(), &[&parent_name, name],
                    );
                    continue;
                }
            }
            if indent == top {
                add_decl(
                    &mut nodes, &mut by_line, &containers, module, interface,
                    "value", name, index, indent, line.trim(), &[],
                );
            }
            continue;
        }
        if let Some(capture) = RE_EQUATION.captures(line) {
            if indent == top {
                let name = capture.get(1).unwrap().as_str();
                if !RESERVED.contains(&name)
                    && !nodes.iter().any(|node| {
                        node.parent.is_none() && node.decl.kind == "value" && node.decl.name == name
                    })
                {
                    add_decl(
                        &mut nodes, &mut by_line, &containers, module, interface,
                        "value", name, index, indent, line.trim(), &[],
                    );
                }
            }
        }
    }
    (nodes, by_line)
}

fn next_significant(lines: &[&str], comments: &HashSet<usize>, start: usize) -> Option<usize> {
    let mut index = start;
    while index < lines.len() {
        if comments.contains(&index) || lines[index].trim().is_empty() || RE_HASH_LINE.is_match(lines[index]) {
            index += 1;
        } else {
            return Some(index);
        }
    }
    None
}

fn previous_significant(lines: &[&str], comments: &HashSet<usize>, start: usize) -> Option<usize> {
    let mut index = start as isize;
    while index >= 0 {
        let at = index as usize;
        if comments.contains(&at) || lines[at].trim().is_empty() || RE_HASH_LINE.is_match(lines[at]) {
            index -= 1;
        } else {
            return Some(at);
        }
    }
    None
}

fn duplicate(decl: &mut FlatNode, doc: Doc, diagnostics: &mut Vec<Value>) {
    if let Some(existing) = &decl.decl.doc {
        if doc.legacy && !existing.legacy {
            return;
        }
        if !doc.legacy && existing.legacy {
            decl.decl.doc = Some(doc);
            return;
        }
        if !doc.legacy && !existing.legacy {
            diagnostics.push(json!({
                "code": "duplicate-doc",
                "message": format!("{} already has a documentation block", decl.decl.id),
                "line": doc.line,
            }));
        }
    }
    decl.decl.doc = Some(doc);
}

/// Parse one source file.
pub fn parse_source(
    text: &str,
    module: Option<&str>,
    source: &str,
    iface: Option<&Value>,
    legacy: bool,
) -> Result<ModuleDoc, String> {
    let lines: Vec<&str> = text.lines().collect();
    let comments = comment_lines(&lines);
    let (detected, module_line) = module_name(&lines, &comments);
    let module_name = module
        .map(str::to_string)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| {
            if !detected.is_empty() {
                detected
            } else {
                Path::new(source)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("source")
                    .to_string()
            }
        });
    let top = top_indent(&lines, module_line, &comments);
    let interface = interface_index(iface);
    let (mut nodes, by_line) = scan_declarations(&lines, &comments, &module_name, top, &interface);
    let mut diagnostics = Vec::new();
    let mut module_doc: Option<Doc> = None;

    for block in scan_blocks(&lines, legacy) {
        let doc = match parse_doc(&block.text, block.start + 1, block.kind == "legacy")? {
            Some(doc) => doc,
            None => continue,
        };
        if block.kind == "leading" || block.kind == "legacy" {
            let target = next_significant(&lines, &comments, block.end + 1);
            if target == module_line {
                if module_doc.is_none()
                    || (!doc.legacy && module_doc.as_ref().is_some_and(|existing| existing.legacy))
                {
                    module_doc = Some(doc);
                } else if !doc.legacy && module_doc.as_ref().is_some_and(|existing| !existing.legacy) {
                    diagnostics.push(json!({
                        "code": "duplicate-doc",
                        "message": format!("{module_name} already has a module documentation block"),
                        "line": doc.line,
                    }));
                }
                continue;
            }
            let Some(target) = target else {
                if block.kind == "leading" {
                    diagnostics.push(json!({"code": "orphan-doc", "message": doc.summary, "line": doc.line}));
                }
                continue;
            };
            let Some(targets) = by_line.get(&target) else {
                // The orphan list is rebuilt below, so this pass only
                // attaches docs that found a declaration.
                continue;
            };
            for &target in targets {
                duplicate(&mut nodes[target], doc.clone(), &mut diagnostics);
            }
        } else {
            let target = block
                .start
                .checked_sub(1)
                .and_then(|start| previous_significant(&lines, &comments, start));
            let Some(target) = target else {
                continue;
            };
            let candidates: Vec<usize> = by_line.keys().copied().filter(|line| *line <= target).collect();
            let Some(best) = candidates.into_iter().max() else {
                continue;
            };
            for &target in &by_line[&best] {
                duplicate(&mut nodes[target], doc.clone(), &mut diagnostics);
            }
        }
    }
    // The orphan list is separate from diagnostics in the schema. Re-scan
    // leading blocks that found no declaration.
    let mut orphans = Vec::new();
    for block in scan_blocks(&lines, legacy) {
        let Ok(Some(doc)) = parse_doc(&block.text, block.start + 1, block.kind == "legacy") else {
            continue;
        };
        if block.kind == "leading" || block.kind == "legacy" {
            let target = next_significant(&lines, &comments, block.end + 1);
            if target != module_line && target.and_then(|line| by_line.get(&line)).is_none() {
                if block.kind == "leading" {
                    orphans.push(json!({"kind": "leading", "line": doc.line, "text": doc.summary}));
                }
                continue;
            }
        }
        if block.kind == "trailing" {
            let target = block
                .start
                .checked_sub(1)
                .and_then(|start| previous_significant(&lines, &comments, start));
            let has_target = target.is_some_and(|line| by_line.keys().any(|key| *key <= line));
            if !has_target {
                orphans.push(json!({"kind": "trailing", "line": doc.line, "text": doc.summary}));
            }
        }
    }

    let known_ids: HashSet<String> = nodes.iter().map(|node| node.decl.id.clone()).collect();
    let known_names: HashSet<String> = nodes.iter().map(|node| node.decl.name.clone()).collect();
    for node in &mut nodes {
        if let Some(doc) = &mut node.decl.doc {
            for link in &mut doc.links {
                link.resolved = known_ids.contains(&link.target) || known_names.contains(&link.target);
                if !link.resolved {
                    diagnostics.push(json!({
                        "code": "unresolved-symbol",
                        "message": format!("{} links to unknown symbol {}", node.decl.id, link.target),
                        "line": doc.line,
                    }));
                }
            }
        }
    }
    if let Some(doc) = &mut module_doc {
        for link in &mut doc.links {
            link.resolved = known_ids.contains(&link.target) || known_names.contains(&link.target);
            if !link.resolved {
                diagnostics.push(json!({
                    "code": "unresolved-symbol",
                    "message": format!("{module_name} links to unknown symbol {}", link.target),
                    "line": doc.line,
                }));
            }
        }
    }

    let mut declarations = Vec::new();
    build_forest(&nodes, None, &mut declarations);
    Ok(ModuleDoc {
        module: module_name,
        source: source.to_string(),
        source_sha256: format!("{:x}", Sha256::digest(text.as_bytes())),
        doc: module_doc,
        declarations,
        orphans,
        diagnostics,
    })
}

fn build_forest(nodes: &[FlatNode], parent: Option<usize>, out: &mut Vec<Decl>) {
    for (index, node) in nodes.iter().enumerate() {
        if node.parent == parent {
            let mut decl = node.decl.clone();
            build_forest(nodes, Some(index), &mut decl.children);
            out.push(decl);
        }
    }
}

/// Flatten a declaration forest in source order.
pub fn flatten<'a>(declarations: &'a [Decl], out: &mut Vec<&'a Decl>) {
    for decl in declarations {
        out.push(decl);
        flatten(&decl.children, out);
    }
}

fn tag_text(value: &Value) -> String {
    if value.as_bool() == Some(true) {
        String::new()
    } else {
        value.as_str().map(str::to_string).unwrap_or_else(|| value.to_string())
    }
}

fn tag_lines(doc: &Doc) -> Vec<String> {
    let mut lines = Vec::new();
    for (key, label) in [("params", "Parameters"), ("typeParams", "Type parameters"), ("fields", "Fields")] {
        if let Some(items) = doc.tags.get(key).and_then(Value::as_array) {
            lines.push(format!("**{label}**"));
            lines.push(String::new());
            for item in items {
                lines.push(format!(
                    "- `{}`: {}",
                    item.get("name").and_then(Value::as_str).unwrap_or(""),
                    item.get("text").and_then(Value::as_str).unwrap_or("")
                ));
            }
            lines.push(String::new());
        }
    }
    if let Some(value) = doc.tags.get("returns") {
        lines.push(format!("**Returns:** {}", tag_text(value)));
        lines.push(String::new());
    }
    for (key, label) in [("safety", "Safety"), ("errors", "Errors")] {
        if let Some(value) = doc.tags.get(key) {
            lines.push(format!("**{label}:** {}", tag_text(value)));
            lines.push(String::new());
        }
    }
    for (key, label) in [("laws", "Laws"), ("effects", "Effects")] {
        if let Some(items) = doc.tags.get(key).and_then(Value::as_array) {
            lines.push(format!("**{label}**"));
            lines.push(String::new());
            for item in items {
                lines.push(format!("- {}", item.as_str().unwrap_or("")));
            }
            lines.push(String::new());
        }
    }
    let mut meta = Vec::new();
    if let Some(value) = doc.tags.get("since") {
        meta.push(format!("Since {}", tag_text(value)));
    }
    if let Some(value) = doc.tags.get("deprecated") {
        meta.push(format!("Deprecated: {}", tag_text(value)));
    }
    if let Some(value) = doc.tags.get("experimental") {
        meta.push(format!("Experimental: {}", tag_text(value)));
    }
    if let Some(items) = doc.tags.get("see").and_then(Value::as_array) {
        meta.push(format!(
            "See also: {}",
            items.iter().map(|item| format!("`{}`", item.as_str().unwrap_or(""))).collect::<Vec<_>>().join(", ")
        ));
    }
    if !meta.is_empty() {
        lines.push(meta.join(" · "));
        lines.push(String::new());
    }
    lines
}

fn doc_lines(doc: &Doc, plain: bool) -> Vec<String> {
    let markdown = if plain { doc.markdown.clone() } else { highlight::markdown(&doc.markdown) };
    let mut lines = vec![markdown, String::new()];
    lines.extend(tag_lines(doc));
    lines
}

fn render_decl(decl: &Decl, level: usize, plain: bool, out: &mut Vec<String>) {
    let heading = "#".repeat(level);
    out.push(format!("{heading} `{}`", decl.name));
    out.push(String::new());
    out.push(format!("`{}`", decl.kind));
    out.push(String::new());
    if !decl.signature.is_empty() {
        if plain {
            out.push("```cinch".into());
            out.push(decl.signature.clone());
            out.push("```".into());
        } else {
            out.push(highlight::html(&decl.signature));
        }
        out.push(String::new());
    }
    if let Some(doc) = &decl.doc {
        out.extend(doc_lines(doc, plain));
    }
    for child in &decl.children {
        render_decl(child, level + 1, plain, out);
    }
}

/// Render one module as Markdown. Cinch code blocks are tree-sitter
/// highlighted unless `plain` is set.
pub fn render_module(module: &ModuleDoc, plain: bool) -> String {
    let mut lines = vec![format!("# {}", module.module), String::new()];
    if let Some(doc) = &module.doc {
        lines.extend(doc_lines(doc, plain));
    }
    if !module.declarations.is_empty() {
        lines.push("## Declarations".into());
        lines.push(String::new());
        for decl in &module.declarations {
            render_decl(decl, 2, plain, &mut lines);
        }
    }
    if !module.orphans.is_empty() {
        lines.push("## Unattached documentation".into());
        lines.push(String::new());
        for orphan in &module.orphans {
            lines.push(format!(
                "- Line {}: {}",
                orphan.get("line").and_then(Value::as_u64).unwrap_or(0),
                orphan.get("text").and_then(Value::as_str).unwrap_or("")
            ));
        }
        lines.push(String::new());
    }
    lines.join("\n").trim_end().to_string() + "\n"
}

/// Recursively collect `.cinch` files from a file or directory.
pub fn iter_sources(root: &Path) -> Vec<PathBuf> {
    if root.is_file() {
        return vec![root.to_path_buf()];
    }
    let mut paths = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let Ok(entries) = fs::read_dir(&directory) else { continue };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("cinch") {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths
}

/// Build the symbol lookup index for a source tree.
pub fn build_index(root: &Path, legacy: bool) -> Result<Value, String> {
    let mut symbols = Vec::new();
    for path in iter_sources(root) {
        let text = fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        let parsed = parse_source(&text, None, &path.to_string_lossy(), None, legacy)?;
        let mut decls = Vec::new();
        flatten(&parsed.declarations, &mut decls);
        for decl in decls {
            symbols.push(json!({
                "id": decl.id,
                "module": parsed.module,
                "kind": decl.kind,
                "name": decl.name,
                "summary": decl.doc.as_ref().map(|doc| doc.summary.clone()).unwrap_or_default(),
                "source": parsed.source,
                "line": decl.line,
            }));
        }
    }
    symbols.sort_by(|a, b| a["id"].as_str().cmp(&b["id"].as_str()));
    Ok(json!({"schema": SCHEMA, "symbols": symbols}))
}

/// Canonical JSON: sorted object keys, two-space indent, UTF-8.
pub fn canonical_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap() + "\n"
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
}

fn wrap_program(source: &str) -> String {
    if Regex::new(r"(?m)^\s*module\s+[A-Za-z_][\w.]*").unwrap().is_match(source) {
        return source.trim_end_matches('\n').to_string() + "\n";
    }
    let body = source
        .lines()
        .map(|line| if line.trim().is_empty() { String::new() } else { format!("  {line}") })
        .collect::<Vec<_>>()
        .join("\n");
    format!("module Main where\n{body}\n")
}

fn iter_docs<'a>(module: &'a ModuleDoc) -> Vec<(String, &'a Doc)> {
    let mut out = Vec::new();
    if let Some(doc) = &module.doc {
        out.push((module.module.clone(), doc));
    }
    let mut decls = Vec::new();
    flatten(&module.declarations, &mut decls);
    for decl in decls {
        if let Some(doc) = &decl.doc {
            out.push((decl.id.clone(), doc));
        }
    }
    out
}

fn run_child(mut command: Command, timeout: Duration) -> Result<(i32, String, String, bool), String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| error.to_string())?;
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");
    let stdout_thread = thread::spawn(move || {
        let mut output = Vec::new();
        let _ = stdout.read_to_end(&mut output);
        output
    });
    let stderr_thread = thread::spawn(move || {
        let mut output = Vec::new();
        let _ = stderr.read_to_end(&mut output);
        output
    });
    let start = Instant::now();
    let mut timed_out = false;
    let status = loop {
        match child.try_wait().map_err(|error| error.to_string())? {
            Some(status) => break status,
            None if start.elapsed() > timeout => {
                timed_out = true;
                let _ = child.kill();
                break child.wait().map_err(|error| error.to_string())?;
            }
            None => thread::sleep(Duration::from_millis(10)),
        }
    };
    let stdout = stdout_thread.join().unwrap_or_default();
    let stderr = stderr_thread.join().unwrap_or_default();
    Ok((
        status.code().unwrap_or(-1),
        String::from_utf8_lossy(&stdout).into_owned(),
        String::from_utf8_lossy(&stderr).into_owned(),
        timed_out,
    ))
}

/// Compile and run every documentation test under the given roots.
pub fn run_doc_tests(
    roots: &[PathBuf],
    legacy: bool,
    compiler: &Path,
    pattern: Option<&str>,
    timeout: u64,
) -> Vec<Value> {
    let mut results = Vec::new();
    let Ok(workspace) = tempfile::tempdir() else {
        return vec![json!({"case": "<temporary directory>", "passed": false, "detail": "could not create a temporary directory"})];
    };
    for root in roots {
        let mut modules = Vec::new();
        for path in iter_sources(root) {
            let Ok(text) = fs::read_to_string(&path) else {
                results.push(json!({"case": path.to_string_lossy(), "passed": false, "detail": "could not read source"}));
                continue;
            };
            match parse_source(&text, None, &path.to_string_lossy(), None, legacy) {
                Ok(module) => modules.push(module),
                Err(error) => results.push(json!({"case": path.to_string_lossy(), "passed": false, "detail": error})),
            }
        }
        for module in &modules {
            for (owner, doc) in iter_docs(module) {
                for test in &doc.tests {
                    let case = format!("{owner}.{}", test.name);
                    if pattern.is_some_and(|pattern| !case.contains(pattern)) {
                        continue;
                    }
                    if test.mode == "transcript" {
                        results.push(json!({"case": case, "passed": false, "detail": "transcript tests have no executable runner"}));
                        println!("FAIL {case}");
                        continue;
                    }
                    if test.input.is_empty() {
                        results.push(json!({"case": case, "passed": false, "detail": "test has no --- input section"}));
                        println!("FAIL {case}");
                        continue;
                    }
                    let directory = workspace.path().join(format!("{:04}", results.len()));
                    let _ = fs::create_dir_all(&directory);
                    let program = directory.join("Main.cinch");
                    let _ = fs::write(&program, wrap_program(&test.input));
                    let stdin_path = directory.join("stdin.txt");
                    if let Some(stdin) = &test.stdin {
                        let _ = fs::write(&stdin_path, stdin);
                    }
                    let mut command = Command::new(compiler);
                    command.arg("run").arg("Main.cinch").current_dir(&directory);
                    if let Some(stdin) = &test.stdin {
                        if let Ok(file) = fs::File::open(&stdin_path) {
                            command.stdin(Stdio::from(file));
                        }
                    } else {
                        command.stdin(Stdio::null());
                    }
                    if !test.args.is_empty() {
                        command.arg("--");
                        command.args(&test.args);
                    }
                    let (code, stdout, stderr, timed_out) = match run_child(command, Duration::from_secs(timeout)) {
                        Ok(result) => result,
                        Err(error) => {
                            results.push(json!({"case": case, "passed": false, "detail": error}));
                            println!("FAIL {case}");
                            println!("{error}");
                            continue;
                        }
                    };
                    let (passed, detail) = if timed_out {
                        (false, format!("timed out after {timeout}s"))
                    } else if let Some(error) = &test.error {
                        let combined = format!("{stderr}{stdout}");
                        (code != 0 && combined.contains(error), format!("exit={code}, stdout={stdout:?}, stderr={stderr:?}"))
                    } else {
                        let mut passed = code == test.exit;
                        if passed {
                            if let Some(expect) = &test.expect {
                                passed = stdout.trim_end_matches('\n') == expect.trim_end_matches('\n');
                            }
                        }
                        (passed, format!("exit={code} (expected {}), stdout={stdout:?}, stderr={stderr:?}", test.exit))
                    };
                    results.push(json!({"case": case, "passed": passed, "detail": detail}));
                    println!("{} {case}", if passed { "PASS" } else { "FAIL" });
                    if !passed {
                        println!("{detail}");
                    }
                }
            }
        }
    }
    results
}

fn option_value(args: &[String], index: &mut usize, flag: &str) -> Result<String, String> {
    *index += 1;
    args.get(*index).cloned().ok_or_else(|| format!("{flag} needs a value"))
}

fn parse_extract(args: &[String]) -> Result<(), String> {
    let mut source = None;
    let mut iface = None;
    let mut module = None;
    let mut legacy = false;
    let mut out = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--iface" => iface = Some(PathBuf::from(option_value(args, &mut index, "--iface")?)),
            "--module" => module = Some(option_value(args, &mut index, "--module")?),
            "--legacy" => legacy = true,
            "--out" | "-o" => out = Some(PathBuf::from(option_value(args, &mut index, "--out")?)),
            other if source.is_none() => source = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let source = source.ok_or("extract needs a source path")?;
    let text = fs::read_to_string(&source).map_err(|error| format!("{}: {error}", source.display()))?;
    let iface = match iface {
        Some(path) => Some(read_json(&path)?),
        None => None,
    };
    let parsed = parse_source(&text, module.as_deref(), &source.to_string_lossy(), iface.as_ref(), legacy)?;
    let payload = canonical_json(&parsed.to_value());
    if let Some(path) = out {
        fs::write(&path, payload).map_err(|error| format!("{}: {error}", path.display()))?;
    } else {
        print!("{payload}");
    }
    if !parsed.orphans.is_empty() {
        return Err("source contains unattached documentation".into());
    }
    Ok(())
}

fn parse_render(args: &[String]) -> Result<(), String> {
    let mut source = None;
    let mut iface = None;
    let mut module = None;
    let mut legacy = false;
    let mut plain = false;
    let mut out = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--iface" => iface = Some(PathBuf::from(option_value(args, &mut index, "--iface")?)),
            "--module" => module = Some(option_value(args, &mut index, "--module")?),
            "--legacy" => legacy = true,
            "--plain" => plain = true,
            "--out" | "-o" => out = Some(PathBuf::from(option_value(args, &mut index, "--out")?)),
            other if source.is_none() => source = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let source = source.ok_or("render needs a source path")?;
    let text = fs::read_to_string(&source).map_err(|error| format!("{}: {error}", source.display()))?;
    let iface = match iface {
        Some(path) => Some(read_json(&path)?),
        None => None,
    };
    let parsed = parse_source(&text, module.as_deref(), &source.to_string_lossy(), iface.as_ref(), legacy)?;
    let rendered = render_module(&parsed, plain);
    if let Some(path) = out {
        fs::write(&path, rendered).map_err(|error| format!("{}: {error}", path.display()))?;
    } else {
        print!("{rendered}");
    }
    Ok(())
}

fn parse_index(args: &[String]) -> Result<(), String> {
    let mut root = None;
    let mut legacy = false;
    let mut out = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--legacy" => legacy = true,
            "--out" | "-o" => out = Some(PathBuf::from(option_value(args, &mut index, "--out")?)),
            other if root.is_none() => root = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let root = root.ok_or("index needs a directory or source path")?;
    let payload = canonical_json(&build_index(&root, legacy)?);
    if let Some(path) = out {
        fs::write(&path, payload).map_err(|error| format!("{}: {error}", path.display()))?;
    } else {
        print!("{payload}");
    }
    Ok(())
}

fn parse_lookup(args: &[String]) -> Result<(), String> {
    let mut symbol = None;
    let mut root = PathBuf::from("compiler/prelude");
    let mut legacy = false;
    let mut as_json = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--dir" => root = PathBuf::from(option_value(args, &mut index, "--dir")?),
            "--legacy" => legacy = true,
            "--json" => as_json = true,
            other if symbol.is_none() => symbol = Some(other.to_string()),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let symbol = symbol.ok_or("lookup needs a symbol")?;
    let data = build_index(&root, legacy)?;
    let symbols = data.get("symbols").and_then(Value::as_array).cloned().unwrap_or_default();
    let found = symbols
        .iter()
        .find(|item| item.get("id").and_then(Value::as_str) == Some(&symbol))
        .or_else(|| {
            symbols
                .iter()
                .find(|item| item.get("name").and_then(Value::as_str) == Some(&symbol))
        })
        .ok_or_else(|| format!("no documentation for {symbol}"))?;
    if as_json {
        print!("{}", canonical_json(found));
    } else {
        println!(
            "{} ({})\n{}:{}\n\n{}",
            found.get("id").and_then(Value::as_str).unwrap_or(""),
            found.get("kind").and_then(Value::as_str).unwrap_or(""),
            found.get("source").and_then(Value::as_str).unwrap_or(""),
            found.get("line").and_then(Value::as_u64).unwrap_or(0),
            found.get("summary").and_then(Value::as_str).unwrap_or("(no summary)"),
        );
    }
    Ok(())
}

fn parse_test_command(args: &[String]) -> Result<(), String> {
    let mut roots = Vec::new();
    let mut compiler = PathBuf::from("cinch");
    let mut legacy = false;
    let mut pattern = None;
    let mut timeout = 90u64;
    let mut report = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--compiler" => compiler = PathBuf::from(option_value(args, &mut index, "--compiler")?),
            "--legacy" => legacy = true,
            "--match" => pattern = Some(option_value(args, &mut index, "--match")?),
            "--timeout" => timeout = option_value(args, &mut index, "--timeout")?.parse().map_err(|_| "--timeout expects seconds")?,
            "--report" => report = Some(PathBuf::from(option_value(args, &mut index, "--report")?)),
            other => roots.push(PathBuf::from(other)),
        }
        index += 1;
    }
    if roots.is_empty() {
        return Err("test needs at least one source or directory".into());
    }
    if !compiler.exists() && compiler.components().count() > 1 {
        return Err(format!("compiler not found: {}", compiler.display()));
    }
    let compiler = fs::canonicalize(&compiler).unwrap_or(compiler);
    let results = run_doc_tests(&roots, legacy, &compiler, pattern.as_deref(), timeout);
    if results.is_empty() {
        return Err("no documentation tests matched".into());
    }
    if let Some(path) = report {
        let compiler_sha = fs::read(&compiler)
            .map(|bytes| format!("{:x}", Sha256::digest(&bytes)))
            .unwrap_or_default();
        let payload = json!({"compilerSha256": compiler_sha, "tests": results});
        fs::write(&path, canonical_json(&payload)).map_err(|error| format!("{}: {error}", path.display()))?;
    }
    let failed = results.iter().filter(|item| item.get("passed").and_then(Value::as_bool) != Some(true)).count();
    println!("{}/{} documentation tests passed", results.len() - failed, results.len());
    if failed > 0 {
        return Err(String::new());
    }
    Ok(())
}

fn parse_markdown(args: &[String]) -> Result<(), String> {
    let mut source = None;
    let mut out = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--out" | "-o" => out = Some(PathBuf::from(option_value(args, &mut index, "--out")?)),
            other if source.is_none() => source = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let text = if let Some(path) = source {
        fs::read_to_string(&path).map_err(|error| format!("{}: {error}", path.display()))?
    } else {
        let mut text = String::new();
        std::io::stdin()
            .read_to_string(&mut text)
            .map_err(|error| error.to_string())?;
        text
    };
    let transformed = highlight::markdown(&text);
    if let Some(path) = out {
        fs::write(&path, transformed).map_err(|error| format!("{}: {error}", path.display()))?;
    } else {
        print!("{transformed}");
    }
    Ok(())
}

fn parse_highlight(args: &[String]) -> Result<(), String> {
    let mut source = None;
    let mut out = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--out" | "-o" => out = Some(PathBuf::from(option_value(args, &mut index, "--out")?)),
            other if source.is_none() => source = Some(PathBuf::from(other)),
            other => return Err(format!("unknown argument {other}")),
        }
        index += 1;
    }
    let source = source.ok_or("highlight needs a source path")?;
    let text = fs::read_to_string(&source).map_err(|error| format!("{}: {error}", source.display()))?;
    let html = highlight::html(&text);
    if let Some(path) = out {
        fs::write(&path, html).map_err(|error| format!("{}: {error}", path.display()))?;
    } else {
        print!("{html}");
    }
    Ok(())
}

/// Entry point for `cinch doc ...`.
pub fn cli(args: &[String]) -> i32 {
    let Some(command) = args.first().map(String::as_str) else {
        eprintln!("cinch doc: expected extract, render, lookup, index, test, or highlight");
        return 2;
    };
    let result = match command {
        "extract" => parse_extract(&args[1..]),
        "render" => parse_render(&args[1..]),
        "lookup" => parse_lookup(&args[1..]),
        "index" => parse_index(&args[1..]),
        "test" => parse_test_command(&args[1..]),
        "markdown" => parse_markdown(&args[1..]),
        "highlight" => parse_highlight(&args[1..]),
        other => Err(format!("unknown doc command {other}")),
    };
    match result {
        Ok(()) => 0,
        Err(message) => {
            if !message.is_empty() {
                eprintln!("cinch doc: {message}");
            }
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANONICAL: &str = r#"-- | The demo module.
--
-- See [the option](symbol:Demo.Option).
module Demo where

  -- | The option type.
  data Option a where
    -- | No value.
    None :: Option a
    Some :: a -> Option a
    -- ^ A present value.

  -- | Map a function.
  --
  -- ```cinch
  -- map f (Some 1)
  -- ```
  --
  -- The bound is $O(1)$ and the identity is:
  --
  -- $$map id x = x$$
  --
  -- ```cinch test name=map-some
  -- --- input
  -- map (\x -> x + 1) (Some 1)
  -- --- expect
  -- Some 2
  -- ```
  --
  -- @since 0.1.0
  -- @see Demo.Option
  -- @param f the transformation
  -- @param o the option
  -- @typeparam a the input element
  -- @returns the mapped option
  map :: forall a b. (f :: a -> b) -> (o :: Option a) -> Option b

  -- | Add two numbers.
  --
  -- @param x the first number
  -- @arg y the second number
  add :: Int -> Int -> Int

  -- | A nested binder.
  nested :: forall a b c. (g :: a -> (b -> c)) -> a -> b -> c
"#;

    #[test]
    fn parses_canonical_docs() {
        let module = parse_source(CANONICAL, Some("Demo"), "Demo.cinch", None, false).unwrap();
        assert_eq!(module.doc.as_ref().unwrap().summary, "The demo module.");
        assert_eq!(module.diagnostics.len(), 0);
        assert_eq!(module.orphans.len(), 0);
        let map = module.declarations.iter().find(|decl| decl.name == "map").unwrap();
        let doc = map.doc.as_ref().unwrap();
        assert_eq!(doc.tests.len(), 1);
        assert_eq!(doc.tests[0].name, "map-some");
        assert_eq!(doc.math.len(), 2);
        let params = map.parameters();
        assert_eq!(params[0].name, "f");
        assert_eq!(params[0].ty, "a -> b");
        assert_eq!(params[0].doc, "the transformation");
        assert_eq!(params[1].name, "o");
        assert_eq!(params[1].ty, "Option a");
        assert_eq!(doc.tags["typeParams"][0]["name"], "a");
        let nested = module.declarations.iter().find(|decl| decl.name == "nested").unwrap();
        assert_eq!(nested.parameters()[0].ty, "a -> (b -> c)");
    }

    #[test]
    fn nested_fence_is_literal() {
        let source = r#"-- | Nested fences.
--
-- ````markdown
-- ```cinch test name=literal
-- --- input
-- main = 0
-- ```
-- @since not a tag
-- ````
--
-- ```cinch test name=real
-- --- input
-- main = 0
-- --- exit
-- 0
-- ```
module Demo where
"#;
        let module = parse_source(source, Some("Demo"), "Demo.cinch", None, false).unwrap();
        let doc = module.doc.as_ref().unwrap();
        assert_eq!(doc.tests.len(), 1);
        assert_eq!(doc.tests[0].name, "real");
        assert!(doc.tags.get("since").is_none());
    }

    #[test]
    fn renders_named_parameter_sections() {
        let module = parse_source(CANONICAL, Some("Demo"), "Demo.cinch", None, false).unwrap();
        let rendered = render_module(&module, true);
        assert!(rendered.contains("**Parameters**"));
        assert!(rendered.contains("`f`: the transformation"));
        assert!(rendered.contains("**Type parameters**"));
    }

    #[test]
    fn parses_test_sections_and_protects_tags() {
        let source = r#"-- | A test case.
--
-- ```cinch test name=main
-- --- input
-- main
-- --- args
-- --verbose input.txt
-- --- stdin
-- hello
-- --- expect
-- hello
-- --- exit
-- 0
-- ```
--
-- ```cinch test name=tag-text
-- --- input
-- value
-- --- expect
-- @since is example text
-- ```
module Demo where
"#;
        let module = parse_source(source, Some("Demo"), "Demo.cinch", None, false).unwrap();
        let doc = module.doc.as_ref().unwrap();
        assert_eq!(doc.tests.len(), 2);
        let main = &doc.tests[0];
        assert_eq!(main.args, ["--verbose", "input.txt"]);
        assert_eq!(main.stdin.as_deref(), Some("hello"));
        assert_eq!(main.expect.as_deref(), Some("hello"));
        assert_eq!(main.exit, 0);
        assert!(doc.tags.get("since").is_none());
    }

    #[test]
    fn reports_orphans_and_duplicate_docs() {
        let orphan = parse_source(
            "-- | unattached\nimport Demo\nmodule Demo where\n",
            Some("Demo"),
            "Demo.cinch",
            None,
            false,
        )
        .unwrap();
        assert_eq!(orphan.orphans.len(), 1);
        assert_eq!(orphan.orphans[0]["kind"], "leading");

        let duplicate = parse_source(
            "-- | first\n-- | second\nvalue :: Int\nmodule Demo where\n",
            Some("Demo"),
            "Demo.cinch",
            None,
            false,
        )
        .unwrap();
        assert!(duplicate.diagnostics.iter().any(|item| item["code"] == "duplicate-doc"));
    }

    #[test]
    fn uses_checked_interface_identity() {
        let iface = json!({
            "module": "Demo",
            "exports": {
                "values": [{"name": "value", "symbol": "Demo.value", "scheme": "Int"}],
            }
        });
        let module = parse_source(
            "-- | value doc\nvalue :: Int\nmodule Demo where\n",
            Some("Demo"),
            "Demo.cinch",
            Some(&iface),
            false,
        )
        .unwrap();
        let value = module.declarations.iter().find(|decl| decl.name == "value").unwrap();
        assert_eq!(value.id, "Demo.value");
        assert_eq!(value.signature, "Int");
    }

    #[test]
    fn cli_extract_writes_canonical_json() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("Demo.cinch");
        let out = temp.path().join("Demo.cinchdoc");
        fs::write(&source, "-- | Demo.\nvalue :: Int\nmodule Demo where\n").unwrap();
        let status = cli(&[
            "extract".into(),
            source.to_string_lossy().into_owned(),
            "--legacy".into(),
            "--out".into(),
            out.to_string_lossy().into_owned(),
        ]);
        assert_eq!(status, 0);
        let value: Value = serde_json::from_str(&fs::read_to_string(&out).unwrap()).unwrap();
        assert_eq!(value["module"], "Demo");
        assert_eq!(value["declarations"][0]["id"], "Demo.value");
    }

    #[test]
    fn render_uses_tree_sitter_highlighting() {
        let module = parse_source(CANONICAL, Some("Demo"), "Demo.cinch", None, false).unwrap();
        let rendered = render_module(&module, false);
        assert!(rendered.contains("class=\"cinch-highlight\""), "{rendered}");
        assert!(rendered.contains("tok-keyword"), "{rendered}");
    }

    #[cfg(unix)]
    #[test]
    fn runs_a_documentation_test_with_a_compiler() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let compiler = temp.path().join("fake-cinch");
        fs::write(&compiler, "#!/bin/sh\nprintf ok\n").unwrap();
        let mut permissions = fs::metadata(&compiler).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&compiler, permissions).unwrap();
        let source = temp.path().join("Demo.cinch");
        fs::write(
            &source,
            r#"-- | A test.
--
-- ```cinch test name=ok
-- --- input
-- main = 0
-- --- expect
-- ok
-- --- exit
-- 0
-- ```
module Demo where
"#,
        )
        .unwrap();
        let results = run_doc_tests(&[source], false, &compiler, None, 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["passed"], true, "{results:?}");
    }
}
