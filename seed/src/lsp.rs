//! P22: the language server over the shared analysis layer.
//!
//! One incremental checker, two front ends: this server drives
//! [`Analysis`] exactly as the REPL does. The transport is LSP over
//! stdio (JSON-RPC 2.0 with `Content-Length` framing), hand-rolled
//! because the workspace carries no dependencies; the shapes follow
//! the `lsp-server`/`lsp-types` conventions. Every file name the
//! analysis produces is workspace-relative, so no output carries an
//! absolute path (the standing manifest rule); documents are addressed
//! by their client-supplied URIs, rooted at the workspace root URI.
//!
//! One server serves both languages, and the document's extension routes it:
//! a `.core` document takes the [`crate::coretext`] path, whose checks are
//! structural and computed from the text alone (design note section 8.4), and
//! every other document takes the [`Analysis`] path exactly as before. The
//! Core path never feeds a file to the checker, and the requests the Core path
//! answers (hover, definition, symbols, folding, semantic tokens) answer with
//! an empty result for a surface document rather than guessing.

use crate::analysis::{Analysis, Completion, Diag, SemToken, Severity, SymKind, UpdateReport};
use crate::coretext::{self, Form, Item, Pos, Rng, Syntax};
use std::collections::BTreeMap;

// --------------------------------------------------------------- JSON

/// A minimal JSON value (the protocol's wire format).
#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(BTreeMap<String, Json>),
}

impl Json {
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(m) => m.get(key),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_num(&self) -> Option<f64> {
        match self {
            Json::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_arr(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(a) => Some(a),
            _ => None,
        }
    }

    /// Object construction from `(key, value)` pairs.
    pub fn obj(pairs: Vec<(&str, Json)>) -> Json {
        let mut m = BTreeMap::new();
        for (k, v) in pairs {
            m.insert(k.to_string(), v);
        }
        Json::Obj(m)
    }

    /// The serialized form.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Num(n) => {
                if n.fract() == 0.0 && n.abs() < 9e15 {
                    out.push_str(&format!("{}", *n as i64));
                } else {
                    out.push_str(&format!("{n}"));
                }
            }
            Json::Str(s) => write_str(s, out),
            Json::Arr(items) => {
                out.push('[');
                for (i, v) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    v.write(out);
                }
                out.push(']');
            }
            Json::Obj(m) => {
                out.push('{');
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    write_str(k, out);
                    out.push(':');
                    v.write(out);
                }
                out.push('}');
            }
        }
    }
}

fn write_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Parse a JSON document.
pub fn parse_json(text: &str) -> Result<Json, String> {
    let bytes: Vec<char> = text.chars().collect();
    let mut p = JsonParser { cs: &bytes, i: 0 };
    p.ws();
    let v = p.value()?;
    p.ws();
    if p.i != p.cs.len() {
        return Err(format!("trailing JSON at {}", p.i));
    }
    Ok(v)
}

struct JsonParser<'a> {
    cs: &'a [char],
    i: usize,
}

impl<'a> JsonParser<'a> {
    fn ws(&mut self) {
        while matches!(self.cs.get(self.i), Some(' ') | Some('\t') | Some('\n') | Some('\r')) {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.cs.get(self.i).copied()
    }

    fn value(&mut self) -> Result<Json, String> {
        match self.peek() {
            None => Err("unexpected end of JSON".to_string()),
            Some('n') => self.word("null", Json::Null),
            Some('t') => self.word("true", Json::Bool(true)),
            Some('f') => self.word("false", Json::Bool(false)),
            Some('"') => Ok(Json::Str(self.string()?)),
            Some('[') => self.array(),
            Some('{') => self.object(),
            Some(c) if c == '-' || c.is_ascii_digit() => self.number(),
            Some(c) => Err(format!("unexpected JSON character {c:?}")),
        }
    }

    fn word(&mut self, w: &str, v: Json) -> Result<Json, String> {
        for c in w.chars() {
            if self.peek() != Some(c) {
                return Err(format!("expected JSON {w}"));
            }
            self.i += 1;
        }
        Ok(v)
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.i;
        if self.peek() == Some('-') {
            self.i += 1;
        }
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-')
        {
            self.i += 1;
        }
        let text: String = self.cs[start..self.i].iter().collect();
        text.parse::<f64>()
            .map(Json::Num)
            .map_err(|_| format!("bad JSON number {text}"))
    }

    fn string(&mut self) -> Result<String, String> {
        if self.peek() != Some('"') {
            return Err("expected a JSON string".to_string());
        }
        self.i += 1;
        let mut out = String::new();
        loop {
            match self.peek() {
                None => return Err("unterminated JSON string".to_string()),
                Some('"') => {
                    self.i += 1;
                    return Ok(out);
                }
                Some('\\') => {
                    self.i += 1;
                    match self.peek() {
                        Some('n') => out.push('\n'),
                        Some('r') => out.push('\r'),
                        Some('t') => out.push('\t'),
                        Some('"') => out.push('"'),
                        Some('\\') => out.push('\\'),
                        Some('/') => out.push('/'),
                        Some('u') => {
                            self.i += 1;
                            let hex: String = self.cs[self.i..(self.i + 4).min(self.cs.len())]
                                .iter()
                                .collect();
                            self.i += hex.chars().count();
                            let n = u32::from_str_radix(&hex, 16)
                                .map_err(|_| format!("bad JSON escape \\u{hex}"))?;
                            out.push(char::from_u32(n).unwrap_or('\u{fffd}'));
                            continue;
                        }
                        other => return Err(format!("bad JSON escape {other:?}")),
                    }
                    self.i += 1;
                }
                Some(c) => {
                    out.push(c);
                    self.i += 1;
                }
            }
        }
    }

    fn array(&mut self) -> Result<Json, String> {
        self.i += 1;
        let mut items = Vec::new();
        self.ws();
        if self.peek() == Some(']') {
            self.i += 1;
            return Ok(Json::Arr(items));
        }
        loop {
            self.ws();
            items.push(self.value()?);
            self.ws();
            match self.peek() {
                Some(',') => self.i += 1,
                Some(']') => {
                    self.i += 1;
                    return Ok(Json::Arr(items));
                }
                other => return Err(format!("expected `,` or `]`, found {other:?}")),
            }
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.i += 1;
        let mut m = BTreeMap::new();
        self.ws();
        if self.peek() == Some('}') {
            self.i += 1;
            return Ok(Json::Obj(m));
        }
        loop {
            self.ws();
            let k = self.string()?;
            self.ws();
            if self.peek() != Some(':') {
                return Err("expected `:` in a JSON object".to_string());
            }
            self.i += 1;
            self.ws();
            let v = self.value()?;
            m.insert(k, v);
            self.ws();
            match self.peek() {
                Some(',') => self.i += 1,
                Some('}') => {
                    self.i += 1;
                    return Ok(Json::Obj(m));
                }
                other => return Err(format!("expected `,` or `}}`, found {other:?}")),
            }
        }
    }
}

// ------------------------------------------------------------- framing

/// One framed message body on the wire.
pub fn frame(body: &str) -> Vec<u8> {
    format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes()
}

/// Read one framed message body (`None` at clean end of stream).
pub fn read_frame(r: &mut impl std::io::BufRead) -> Result<Option<String>, String> {
    let mut len: Option<usize> = None;
    loop {
        let mut line = String::new();
        let n = r
            .read_line(&mut line)
            .map_err(|e| format!("read error: {e}"))?;
        if n == 0 {
            return Ok(None);
        }
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            len = Some(
                rest.trim()
                    .parse()
                    .map_err(|_| "bad Content-Length".to_string())?,
            );
        }
    }
    let Some(len) = len else {
        return Err("a framed message needs Content-Length".to_string());
    };
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)
        .map_err(|e| format!("read error: {e}"))?;
    String::from_utf8(buf)
        .map(Some)
        .map_err(|_| "message body is not UTF-8".to_string())
}

// --------------------------------------------------------------- server

/// The semantic token legend, in wire order.
pub const TOKEN_TYPES: [&str; 13] = [
    "keyword",
    "variable",
    "function",
    "type",
    "class",
    "enumMember",
    "method",
    "property",
    "namespace",
    "number",
    "string",
    "operator",
    // Comments are lexical trivia, so grammars that keep them out of the
    // syntax tree leave editors with nothing to highlight; reporting them as
    // tokens is what lets an editor colour them.
    "comment",
];

/// The language server (see the module docs).
pub struct LspServer {
    analysis: Analysis,
    /// The workspace root URI the client supplied (never rendered).
    root_uri: String,
    /// Its path form, for URI to workspace-relative key mapping.
    root_path: String,
    /// Every recheck report so far (the incremental evidence).
    reports: Vec<UpdateReport>,
    /// Open Core documents, by workspace-relative key. They never reach the
    /// analysis: a Core file records what a check produced, so re-checking one
    /// would be circular.
    core: BTreeMap<String, String>,
    shutdown: bool,
    exit: bool,
}

impl Default for LspServer {
    fn default() -> LspServer {
        LspServer::new()
    }
}

impl LspServer {
    /// A server whose workspace root comes from `initialize`.
    pub fn new() -> LspServer {
        LspServer {
            analysis: Analysis::new(std::path::Path::new("")),
            root_uri: String::new(),
            root_path: String::new(),
            reports: Vec::new(),
            core: BTreeMap::new(),
            shutdown: false,
            exit: false,
        }
    }

    /// The recheck reports, in order (which modules each update
    /// rechecked and which cached `.cinchif` entries it kept).
    pub fn reports(&self) -> &[UpdateReport] {
        &self.reports
    }

    /// Has the client sent `exit`?
    pub fn exited(&self) -> bool {
        self.exit
    }

    /// Drive the server over streams (the stdio transport).
    pub fn run(
        &mut self,
        mut r: impl std::io::BufRead,
        mut w: impl std::io::Write,
    ) -> Result<(), String> {
        while !self.exit {
            match read_frame(&mut r)? {
                None => break,
                Some(body) => {
                    for out in self.handle(&body) {
                        w.write_all(&frame(&out))
                            .map_err(|e| format!("write error: {e}"))?;
                        w.flush().map_err(|e| format!("write error: {e}"))?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Handle one incoming message body; returns the outgoing bodies
    /// (responses and notifications) in order.
    pub fn handle(&mut self, msg: &str) -> Vec<String> {
        let Ok(req) = parse_json(msg) else {
            return vec![error_response(&Json::Null, -32700, "parse error")];
        };
        let id = req.get("id").cloned();
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let params = req.get("params").cloned().unwrap_or(Json::Null);
        let mut out: Vec<String> = Vec::new();
        match method {
            "initialize" => {
                let root = params
                    .get("rootUri")
                    .and_then(|u| u.as_str())
                    .unwrap_or("")
                    .to_string();
                self.root_uri = root.clone();
                self.root_path = uri_path(&root);
                self.analysis = Analysis::new(std::path::Path::new(&self.root_path));
                out.push(result_response(&id, initialize_result()));
            }
            "initialized" | "workspace/didChangeConfiguration" => {}
            "shutdown" => {
                self.shutdown = true;
                out.push(result_response(&id, Json::Null));
            }
            "exit" => {
                self.exit = true;
            }
            "textDocument/didOpen" => {
                let doc = params.get("textDocument").cloned().unwrap_or(Json::Null);
                let uri = doc.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let text = doc.get("text").and_then(|t| t.as_str()).unwrap_or("").to_string();
                if self.is_core(&uri) {
                    let key = self.key_of(&uri);
                    self.core.insert(key.clone(), text);
                    out.push(self.publish_core(&uri, &key));
                } else {
                    let key = self.key_of(&uri);
                    self.analysis.set_document(&key, &text);
                    let report = self.analysis.update();
                    self.reports.push(report);
                    out.extend(self.publish_all());
                }
            }
            "textDocument/didChange" => {
                let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
                let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
                // Full sync: the last content change carries the whole
                // document.
                let text = params
                    .get("contentChanges")
                    .and_then(|c| c.as_arr())
                    .and_then(|c| c.last())
                    .and_then(|c| c.get("text"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                if self.is_core(&uri) {
                    let key = self.key_of(&uri);
                    self.core.insert(key.clone(), text);
                    out.push(self.publish_core(&uri, &key));
                } else {
                    let key = self.key_of(&uri);
                    self.analysis.set_document(&key, &text);
                    let report = self.analysis.update();
                    self.reports.push(report);
                    out.extend(self.publish_all());
                }
            }
            "textDocument/didClose" => {
                let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
                let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
                if self.is_core(&uri) {
                    let key = self.key_of(&uri);
                    self.core.remove(&key);
                } else {
                    let key = self.key_of(&uri);
                    self.analysis.remove_document(&key);
                }
                out.push(diagnostics_notification(&uri, &[]));
            }
            "textDocument/hover" => {
                let (uri, line, col) = raw_doc_pos(&params);
                let result = if self.is_core(&uri) {
                    match self.core_text(&uri).and_then(|text| coretext::hover(text, line, col)) {
                        Some(hover) => Json::obj(vec![
                            (
                                "contents",
                                Json::obj(vec![
                                    ("kind", Json::Str("markdown".to_string())),
                                    ("value", Json::Str(hover.markdown)),
                                ]),
                            ),
                            ("range", rng_json(&hover.range)),
                        ]),
                        None => Json::Null,
                    }
                } else {
                    let key = self.key_of(&uri);
                    match self.analysis.hover(&key, line + 1, col + 1) {
                        Some(v) => Json::obj(vec![(
                            "contents",
                            Json::obj(vec![
                                ("kind", Json::Str("markdown".to_string())),
                                ("value", Json::Str(v)),
                            ]),
                        )]),
                        None => Json::Null,
                    }
                };
                out.push(result_response(&id, result));
            }
            "textDocument/definition" => {
                let (uri, line, col) = raw_doc_pos(&params);
                let result = if self.is_core(&uri) {
                    match self
                        .core_text(&uri)
                        .and_then(|text| coretext::definition(text, line, col))
                    {
                        Some((sl, sc, el, ec)) => Json::obj(vec![
                            ("uri", Json::Str(uri.clone())),
                            ("range", rng_json(&Rng::new((sl, sc), (el, ec)))),
                        ]),
                        None => Json::Null,
                    }
                } else {
                    let key = self.key_of(&uri);
                    match self.analysis.definition(&key, line + 1, col + 1) {
                        Some((file, l, c)) => Json::obj(vec![
                            ("uri", Json::Str(self.uri_of(&file))),
                            ("range", range_json(l, c, l, c + 1)),
                        ]),
                        None => Json::Null,
                    }
                };
                out.push(result_response(&id, result));
            }
            "textDocument/completion" => {
                let (uri, line, col) = doc_pos(&params);
                let key = self.key_of(&uri);
                let items: Vec<Json> = self
                    .analysis
                    .completions(&key, line, col)
                    .iter()
                    .map(completion_json)
                    .collect();
                out.push(result_response(
                    &id,
                    Json::obj(vec![
                        ("isIncomplete", Json::Bool(false)),
                        ("items", Json::Arr(items)),
                    ]),
                ));
            }
            "textDocument/semanticTokens/full" => {
                let uri = params
                    .get("textDocument")
                    .and_then(|t| t.get("uri"))
                    .and_then(|u| u.as_str())
                    .unwrap_or("");
                let data = if self.is_core(uri) {
                    let tokens = match self.core_text(uri) {
                        Some(text) => core_semantic_tokens(&coretext::syntax(text)),
                        None => Vec::new(),
                    };
                    semantic_data(&tokens)
                } else {
                    let key = self.key_of(uri);
                    semantic_data(&self.analysis.semantic_tokens(&key))
                };
                out.push(result_response(
                    &id,
                    Json::obj(vec![("data", Json::Arr(data))]),
                ));
            }
            "textDocument/documentSymbol" => {
                let uri = params
                    .get("textDocument")
                    .and_then(|t| t.get("uri"))
                    .and_then(|u| u.as_str())
                    .unwrap_or("");
                let items = match self.core_text(uri) {
                    Some(text) => core_symbols(&coretext::syntax(text)),
                    // Document symbols are a Core capability; a surface
                    // document keeps whatever the client derives itself.
                    None => Vec::new(),
                };
                out.push(result_response(&id, Json::Arr(items)));
            }
            "textDocument/foldingRange" => {
                let uri = params
                    .get("textDocument")
                    .and_then(|t| t.get("uri"))
                    .and_then(|u| u.as_str())
                    .unwrap_or("");
                let ranges = match self.core_text(uri) {
                    Some(text) => core_folding(&coretext::syntax(text)),
                    None => Vec::new(),
                };
                out.push(result_response(&id, Json::Arr(ranges)));
            }
            "workspace/symbol" => {
                let query = params
                    .get("query")
                    .and_then(|q| q.as_str())
                    .unwrap_or("")
                    .to_lowercase();
                let mut items = Vec::new();
                for (key, text) in &self.core {
                    let name = std::path::Path::new(key)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| key.clone());
                    for unit in &coretext::syntax(text).units {
                        if !query.is_empty()
                            && !unit.name.to_lowercase().contains(&query)
                            && !name.to_lowercase().contains(&query)
                        {
                            continue;
                        }
                        items.push(Json::obj(vec![
                            ("name", Json::Str(format!("{name}: {}", unit.name))),
                            ("kind", Json::Num(12.0)),
                            (
                                "location",
                                Json::obj(vec![
                                    ("uri", Json::Str(self.uri_of(key))),
                                    ("range", rng_json(&unit.name_range)),
                                ]),
                            ),
                        ]));
                    }
                }
                out.push(result_response(&id, Json::Arr(items)));
            }
            "" if id.is_some() => {
                out.push(error_response(&id.unwrap_or(Json::Null), -32600, "invalid request"));
            }
            "" => {}
            other => {
                if id.is_some() {
                    out.push(error_response(
                        &id.unwrap_or(Json::Null),
                        -32601,
                        &format!("method not found: {other}"),
                    ));
                }
            }
        }
        out
    }

    /// The workspace-relative key of a document URI.
    fn key_of(&self, uri: &str) -> String {
        let path = uri_path(uri);
        path.strip_prefix(&self.root_path)
            .map(|k| k.trim_start_matches('/').to_string())
            .unwrap_or_else(|| {
                std::path::Path::new(&path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or(path)
            })
    }

    /// The document URI of a workspace-relative key (always rooted at
    /// the workspace root URI; no other absolute path appears).
    fn uri_of(&self, key: &str) -> String {
        format!(
            "{}/{}",
            self.root_uri.trim_end_matches('/'),
            key.trim_start_matches('/')
        )
    }

    /// Whether a document URI names a Core file. The extension decides: a
    /// client's language id may be spelled differently, and routing a file to
    /// the wrong checks would be worse than not answering.
    fn is_core(&self, uri: &str) -> bool {
        uri_path(uri).ends_with(".core")
    }

    /// The text of an open Core document.
    fn core_text(&self, uri: &str) -> Option<&str> {
        self.core.get(&self.key_of(uri)).map(String::as_str)
    }

    /// The diagnostics notification for one open Core document.
    fn publish_core(&self, uri: &str, key: &str) -> String {
        let text = self.core.get(key).map(String::as_str).unwrap_or("");
        core_diagnostics_notification(uri, text)
    }

    /// Publish diagnostics for every open document.
    fn publish_all(&self) -> Vec<String> {
        let mut out = Vec::new();
        for key in self.analysis.files() {
            if key.starts_with('<') {
                continue;
            }
            let diags = self.analysis.file_diagnostics(&key);
            out.push(diagnostics_notification(&self.uri_of(&key), &diags));
        }
        out
    }
}

// --------------------------------------------------------------- shapes

fn initialize_result() -> Json {
    let legend = Json::obj(vec![
        (
            "tokenTypes",
            Json::Arr(TOKEN_TYPES.iter().map(|t| Json::Str(t.to_string())).collect()),
        ),
        ("tokenModifiers", Json::Arr(Vec::new())),
    ]);
    Json::obj(vec![
        (
            "capabilities",
            Json::obj(vec![
                ("textDocumentSync", Json::Num(1.0)),
                ("hoverProvider", Json::Bool(true)),
                ("definitionProvider", Json::Bool(true)),
                ("documentSymbolProvider", Json::Bool(true)),
                ("foldingRangeProvider", Json::Bool(true)),
                ("workspaceSymbolProvider", Json::Bool(true)),
                (
                    "completionProvider",
                    Json::obj(vec![
                        ("resolveProvider", Json::Bool(false)),
                        (
                            "triggerCharacters",
                            Json::Arr(vec![Json::Str(".".to_string())]),
                        ),
                    ]),
                ),
                (
                    "semanticTokensProvider",
                    Json::obj(vec![("legend", legend), ("full", Json::Bool(true))]),
                ),
            ]),
        ),
        (
            "serverInfo",
            Json::obj(vec![
                ("name", Json::Str("cinch".to_string())),
                ("version", Json::Str(env!("CARGO_PKG_VERSION").to_string())),
            ]),
        ),
    ])
}

fn result_response(id: &Option<Json>, result: Json) -> String {
    Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        (
            "id",
            id.clone().unwrap_or(Json::Null),
        ),
        ("result", result),
    ])
    .to_text()
}

fn error_response(id: &Json, code: i64, message: &str) -> String {
    Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("id", id.clone()),
        (
            "error",
            Json::obj(vec![
                ("code", Json::Num(code as f64)),
                ("message", Json::Str(message.to_string())),
            ]),
        ),
    ])
    .to_text()
}

/// The `textDocument/publishDiagnostics` notification.
fn diagnostics_notification(uri: &str, diags: &[Diag]) -> String {
    let items: Vec<Json> = diags
        .iter()
        .map(|d| {
            Json::obj(vec![
                (
                    "range",
                    range_json(d.line, d.col, d.line, d.col + 1),
                ),
                (
                    "severity",
                    Json::Num(
                        match d.severity {
                            Severity::Error => 1,
                            Severity::Warning => 2,
                            Severity::Information => 3,
                        } as f64,
                    ),
                ),
                ("message", Json::Str(d.message.clone())),
                ("source", Json::Str("cinch".to_string())),
            ])
        })
        .collect();
    Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("method", Json::Str("textDocument/publishDiagnostics".to_string())),
        (
            "params",
            Json::obj(vec![
                ("uri", Json::Str(uri.to_string())),
                ("diagnostics", Json::Arr(items)),
            ]),
        ),
    ])
    .to_text()
}

/// A range in LSP coordinates (the analysis is 1-based; the wire is
/// 0-based).
fn range_json(sl: u32, sc: u32, el: u32, ec: u32) -> Json {
    let pos = |l: u32, c: u32| {
        Json::obj(vec![
            ("line", Json::Num(l.saturating_sub(1) as f64)),
            ("character", Json::Num(c.saturating_sub(1) as f64)),
        ])
    };
    Json::obj(vec![
        ("start", pos(sl, sc)),
        ("end", pos(el, ec)),
    ])
}

fn doc_pos(params: &Json) -> (String, u32, u32) {
    let (uri, line, col) = raw_doc_pos(params);
    (uri, line + 1, col + 1)
}

/// A document URI and the request position as the client sent it (0-based
/// line and character, which is what the Core path needs).
fn raw_doc_pos(params: &Json) -> (String, u32, u32) {
    let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
    let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
    let pos = params.get("position").cloned().unwrap_or(Json::Null);
    let line = pos.get("line").and_then(|l| l.as_num()).unwrap_or(0.0) as u32;
    let col = pos.get("character").and_then(|c| c.as_num()).unwrap_or(0.0) as u32;
    (uri, line, col)
}

/// A range whose endpoints are already 0-based (the Core path works in the
/// wire's own coordinates).
fn rng_json(range: &Rng) -> Json {
    let pos = |p: Pos| {
        Json::obj(vec![
            ("line", Json::Num(p.0 as f64)),
            ("character", Json::Num(p.1 as f64)),
        ])
    };
    Json::obj(vec![
        ("start", pos(range.start)),
        ("end", pos(range.end)),
    ])
}

fn completion_json(c: &Completion) -> Json {
    let kind = match c.kind {
        SymKind::Value => 3,
        SymKind::Type => 25,
        SymKind::Ctor => 4,
        SymKind::Field => 5,
        SymKind::Class => 7,
        SymKind::Method => 2,
        SymKind::Law => 3,
        SymKind::Instance => 3,
        SymKind::Effect => 25,
        SymKind::EffectOp => 3,
    };
    Json::obj(vec![
        ("label", Json::Str(c.label.clone())),
        ("kind", Json::Num(kind as f64)),
        ("detail", Json::Str(c.detail.clone())),
    ])
}

/// The semantic tokens payload: `[deltaLine, deltaStart, length,
/// tokenType, tokenModifiers]` per token.
fn semantic_data(tokens: &[SemToken]) -> Vec<Json> {
    let mut data = Vec::new();
    let mut prev_line = 0u32;
    let mut prev_col = 0u32;
    for t in tokens {
        let line = t.line.saturating_sub(1);
        let col = t.col.saturating_sub(1);
        let delta_line = line - prev_line;
        let delta_start = if delta_line == 0 {
            col.saturating_sub(prev_col)
        } else {
            col
        };
        let ty = TOKEN_TYPES
            .iter()
            .position(|k| *k == t.kind)
            .unwrap_or(0);
        for n in [delta_line, delta_start, t.len, ty as u32, 0] {
            data.push(Json::Num(n as f64));
        }
        prev_line = line;
        prev_col = col;
    }
    data
}

/// The path part of a `file:` URI (percent decoding included).
fn uri_path(uri: &str) -> String {
    let rest = uri.strip_prefix("file://").unwrap_or(uri);
    let rest = rest.split(['?', '#']).next().unwrap_or(rest);
    let mut out = String::new();
    let mut cs = rest.chars();
    while let Some(c) = cs.next() {
        if c == '%' {
            let hex: String = cs.by_ref().take(2).collect();
            if let Ok(b) = u8::from_str_radix(&hex, 16) {
                out.push(b as char);
                continue;
            }
            out.push('%');
            out.push_str(&hex);
        } else {
            out.push(c);
        }
    }
    out
}

// -------------------------------------------------------- core documents

/// One Core finding in the coordinates the wire uses.
struct CoreDiag {
    severity: u8,
    code: &'static str,
    message: String,
    range: Rng,
}

/// The `textDocument/publishDiagnostics` notification for a Core document:
/// the reader's and verifier's findings at their own ranges, plus a warning
/// for every unit the file does not locate with `@span`. A Core file is never
/// type-checked; these are structural checks only.
fn core_diagnostics_notification(uri: &str, text: &str) -> String {
    let mut diags: Vec<CoreDiag> = Vec::new();
    if let Err(errors) = coretext::verify(text) {
        for error in errors {
            diags.push(CoreDiag {
                severity: 1,
                code: error.code,
                message: error.message,
                range: Rng::new(error.start, error.end),
            });
        }
    }
    for unit in &coretext::syntax(text).units {
        if unit.annotation("span").is_none() {
            diags.push(CoreDiag {
                severity: 2,
                code: coretext::code::SPAN_MISSING,
                message: format!("unit `{}` has no `@span` annotation", unit.name),
                range: unit.name_range,
            });
        }
    }
    diags.sort_by_key(|diag| (diag.range.start, diag.range.end));
    let items: Vec<Json> = diags
        .iter()
        .map(|diag| {
            Json::obj(vec![
                ("range", rng_json(&diag.range)),
                ("severity", Json::Num(diag.severity as f64)),
                ("code", Json::Str(diag.code.to_string())),
                ("source", Json::Str("cinch".to_string())),
                ("message", Json::Str(diag.message.clone())),
            ])
        })
        .collect();
    Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("method", Json::Str("textDocument/publishDiagnostics".to_string())),
        (
            "params",
            Json::obj(vec![
                ("uri", Json::Str(uri.to_string())),
                ("diagnostics", Json::Arr(items)),
            ]),
        ),
    ])
    .to_text()
}

/// The semantic tokens a Core document reports. Candidates are sorted and a
/// token that starts inside its predecessor is dropped: the payload may not
/// overlap, and an atom that is both a form's keyword and a literal
/// (`(unit)`) is one token.
fn core_semantic_tokens(syn: &Syntax) -> Vec<SemToken> {
    let mut candidates: Vec<(Pos, u32, &'static str)> = Vec::new();
    {
        let mut push = |range: &Rng, kind: &'static str| {
            if range.start.0 != range.end.0 || range.end.1 <= range.start.1 {
                return;
            }
            candidates.push((range.start, range.end.1 - range.start.1, kind));
        };
        for line in &syn.header {
            let word = line.kind.word();
            let start = line.range.start;
            push(&Rng::new(start, (start.0, start.1 + word.len() as u32)), "keyword");
            if matches!(
                line.kind,
                coretext::HeaderKind::Module | coretext::HeaderKind::Global
            ) && !line.value.is_empty()
            {
                let value_start = (start.0, start.1 + word.len() as u32 + 1);
                let len: u32 = line.value.chars().map(|c| c.len_utf16() as u32).sum();
                push(&Rng::new(value_start, (start.0, value_start.1 + len)), "namespace");
            }
        }
        for unit in &syn.units {
            push(&unit.name_range, "namespace");
            if !unit.ty.is_empty() && unit.ty != "-" {
                push(&unit.ty_range, "type");
            }
            for ann in &unit.anns {
                push(&ann.name_range, "property");
            }
            for param in &unit.params {
                push(&param.name_range, "variable");
            }
        }
        for form in syn.forms() {
            if !form.head.is_empty() {
                push(&form.head_range, "keyword");
            }
            for mark in &form.marks {
                push(&mark.name_range, "property");
            }
        }
        for reference in &syn.refs {
            push(&reference.range, reference.kind.token());
        }
        for literal in &syn.literals {
            push(&literal.range, literal.kind.token());
        }
        for class in &syn.classes {
            push(&class.range, "type");
        }
        for projection in &syn.projections {
            if let Some(range) = &projection.slot_range {
                push(range, "number");
            }
        }
    }
    candidates.sort_by_key(|candidate| (candidate.0, candidate.1));
    let mut tokens = Vec::new();
    let mut last_end: Option<Pos> = None;
    for (start, len, kind) in candidates {
        if last_end.is_some_and(|end| start < end) {
            continue;
        }
        last_end = Some((start.0, start.1 + len));
        tokens.push(SemToken { line: start.0 + 1, col: start.1 + 1, len, kind });
    }
    tokens
}

/// A Core document's units, with the body's forms nested by depth.
fn core_symbols(syn: &Syntax) -> Vec<Json> {
    let mut out = Vec::new();
    for unit in &syn.units {
        let mut children = Vec::new();
        if let Some(body) = &unit.body {
            // The body is the unit's first nested symbol, so the tree shows
            // the depth the forms print at.
            children.push(form_symbol(body));
        }
        out.push(Json::obj(vec![
            ("name", Json::Str(unit.name.clone())),
            (
                "detail",
                Json::Str(if unit.ty.is_empty() { "-".to_string() } else { unit.ty.clone() }),
            ),
            ("kind", Json::Num(12.0)),
            ("range", rng_json(&unit.block)),
            ("selectionRange", rng_json(&unit.name_range)),
            ("children", Json::Arr(children)),
        ]));
    }
    out
}

fn form_symbol(form: &Form) -> Json {
    let mut nested = Vec::new();
    form_symbols(form, &mut nested);
    let name = if form.head.is_empty() { "alternative" } else { form.head.as_str() };
    Json::obj(vec![
        ("name", Json::Str(name.to_string())),
        ("kind", Json::Num(13.0)),
        ("range", rng_json(&form.range)),
        ("selectionRange", rng_json(&form.head_range)),
        ("children", Json::Arr(nested)),
    ])
}

fn form_symbols(form: &Form, out: &mut Vec<Json>) {
    for child in &form.children {
        let Item::Form(child) = child else { continue };
        out.push(form_symbol(child));
    }
}

/// A Core document's folding ranges: a unit's block, every `lam`, and every
/// `case` alternative.
fn core_folding(syn: &Syntax) -> Vec<Json> {
    let mut out = Vec::new();
    for unit in &syn.units {
        folding(&mut out, &unit.block, "region");
        if let Some(body) = &unit.body {
            fold_forms(body, &mut out);
        }
    }
    out
}

fn fold_forms(form: &Form, out: &mut Vec<Json>) {
    if form.head == "lam" {
        folding(out, &form.range, "region");
    }
    for child in &form.children {
        let Item::Form(child) = child else { continue };
        // A `case` alternative prints as `((pattern) body)`: no head of its
        // own, and children from the pattern on.
        if child.head.is_empty() && !child.children.is_empty() {
            folding(out, &child.range, "region");
        }
        fold_forms(child, out);
    }
}

fn folding(out: &mut Vec<Json>, range: &Rng, kind: &str) {
    if range.end.0 <= range.start.0 {
        return;
    }
    out.push(Json::obj(vec![
        ("startLine", Json::Num(range.start.0 as f64)),
        ("endLine", Json::Num(range.end.0 as f64)),
        ("kind", Json::Str(kind.to_string())),
    ]));
}
