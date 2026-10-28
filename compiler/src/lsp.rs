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

use crate::analysis::{Analysis, Completion, Diag, SemToken, Severity, SymKind, UpdateReport};
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
pub const TOKEN_TYPES: [&str; 12] = [
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
                let key = self.key_of(&uri);
                self.analysis.set_document(&key, &text);
                let report = self.analysis.update();
                self.reports.push(report);
                out.extend(self.publish_all());
            }
            "textDocument/didChange" => {
                let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
                let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let key = self.key_of(&uri);
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
                self.analysis.set_document(&key, &text);
                let report = self.analysis.update();
                self.reports.push(report);
                out.extend(self.publish_all());
            }
            "textDocument/didClose" => {
                let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
                let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
                let key = self.key_of(&uri);
                self.analysis.remove_document(&key);
                out.push(diagnostics_notification(&uri, &[]));
            }
            "textDocument/hover" => {
                let (uri, line, col) = doc_pos(&params);
                let key = self.key_of(&uri);
                let value = self.analysis.hover(&key, line, col);
                let result = match value {
                    Some(v) => Json::obj(vec![(
                        "contents",
                        Json::obj(vec![
                            ("kind", Json::Str("markdown".to_string())),
                            ("value", Json::Str(v)),
                        ]),
                    )]),
                    None => Json::Null,
                };
                out.push(result_response(&id, result));
            }
            "textDocument/definition" => {
                let (uri, line, col) = doc_pos(&params);
                let key = self.key_of(&uri);
                let result = match self.analysis.definition(&key, line, col) {
                    Some((file, l, c)) => Json::obj(vec![(
                        "uri",
                        Json::Str(self.uri_of(&file)),
                    ), ("range", range_json(l, c, l, c + 1))]),
                    None => Json::Null,
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
                let key = self.key_of(uri);
                let data = semantic_data(&self.analysis.semantic_tokens(&key));
                out.push(result_response(
                    &id,
                    Json::obj(vec![("data", Json::Arr(data))]),
                ));
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
    let td = params.get("textDocument").cloned().unwrap_or(Json::Null);
    let uri = td.get("uri").and_then(|u| u.as_str()).unwrap_or("").to_string();
    let pos = params.get("position").cloned().unwrap_or(Json::Null);
    let line = pos.get("line").and_then(|l| l.as_num()).unwrap_or(0.0) as u32 + 1;
    let col = pos
        .get("character")
        .and_then(|c| c.as_num())
        .unwrap_or(0.0) as u32
        + 1;
    (uri, line, col)
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
