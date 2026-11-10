//! P22 acceptance: the language server driven over in-memory stdio.
//!
//! The server runs its stdio loop against pipe-backed in-memory streams
//! (`Content-Length` framed JSON-RPC, exactly as a client would speak
//! it): initialize, didOpen/didChange diagnostics with correct spans,
//! hover, definition across files, completion, semantic tokens, the
//! incremental recheck report over the `.cinchif` chain, and the
//! no-absolute-paths rule.

use cinch::lsp::{frame, parse_json, Json, LspServer, TOKEN_TYPES};
use std::collections::VecDeque;
use std::io::{BufReader, Read, Write};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

// ----------------------------------------------------- in-memory pipes

struct PipeState {
    bytes: VecDeque<u8>,
    closed: bool,
}

#[derive(Clone)]
struct PipeWriter {
    state: Arc<(Mutex<PipeState>, Condvar)>,
}

struct PipeReader {
    state: Arc<(Mutex<PipeState>, Condvar)>,
}

fn memory_pipe() -> (PipeReader, PipeWriter) {
    let state = Arc::new((
        Mutex::new(PipeState {
            bytes: VecDeque::new(),
            closed: false,
        }),
        Condvar::new(),
    ));
    (
        PipeReader {
            state: state.clone(),
        },
        PipeWriter { state },
    )
}

impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let (m, cv) = &*self.state;
        let mut g = m.lock().unwrap();
        g.bytes.extend(buf);
        cv.notify_all();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        let (m, cv) = &*self.state;
        let mut g = m.lock().unwrap();
        g.closed = true;
        cv.notify_all();
    }
}

impl Read for PipeReader {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let (m, cv) = &*self.state;
        let mut g = m.lock().unwrap();
        loop {
            if !g.bytes.is_empty() {
                let n = out.len().min(g.bytes.len());
                for b in out.iter_mut().take(n) {
                    *b = g.bytes.pop_front().expect("non-empty");
                }
                return Ok(n);
            }
            if g.closed {
                return Ok(0);
            }
            g = cv.wait(g).unwrap();
        }
    }
}

// ------------------------------------------------------------- client

/// A JSON-RPC client over the in-memory pipes.
struct Client {
    to_server: PipeWriter,
    from_server: BufReader<PipeReader>,
    next_id: i64,
    /// Notifications received while waiting for responses.
    notes: Vec<Json>,
    /// Every message the server sent, in order.
    log: Vec<Json>,
}

impl Client {
    fn send(&mut self, body: &str) {
        self.to_server.write_all(&frame(body)).expect("write");
        self.to_server.flush().expect("flush");
    }

    fn recv(&mut self) -> Json {
        let mut len = None;
        loop {
            let mut line = String::new();
            let n = std::io::BufRead::read_line(&mut self.from_server, &mut line).expect("read line");
            assert!(n > 0, "the server closed its stream");
            let line = line.trim_end_matches(['\r', '\n']);
            if line.is_empty() {
                break;
            }
            if let Some(rest) = line.strip_prefix("Content-Length:") {
                len = Some(rest.trim().parse::<usize>().expect("content length"));
            }
        }
        let mut buf = vec![0u8; len.expect("framed message")];
        self.from_server.read_exact(&mut buf).expect("read body");
        let msg = parse_json(std::str::from_utf8(&buf).expect("utf8")).expect("json");
        self.log.push(msg.clone());
        msg
    }

    /// Send a request and wait for its response (stashing any
    /// notifications that arrive first).
    fn request(&mut self, method: &str, params: Json) -> Json {
        self.next_id += 1;
        let id = self.next_id;
        let body = Json::obj(vec![
            ("jsonrpc", Json::Str("2.0".to_string())),
            ("id", Json::Num(id as f64)),
            ("method", Json::Str(method.to_string())),
            ("params", params),
        ])
        .to_text();
        self.send(&body);
        loop {
            let msg = self.recv();
            if msg.get("id").and_then(|i| i.as_num()) == Some(id as f64) {
                return msg;
            }
            self.notes.push(msg);
        }
    }

    /// Send a notification.
    fn notify(&mut self, method: &str, params: Json) {
        let body = Json::obj(vec![
            ("jsonrpc", Json::Str("2.0".to_string())),
            ("method", Json::Str(method.to_string())),
            ("params", params),
        ])
        .to_text();
        self.send(&body);
    }

    /// Read until a notification matches `pred` (the rest are stashed).
    fn wait_note(&mut self, pred: impl Fn(&Json) -> bool) -> Json {
        if let Some(i) = self.notes.iter().position(&pred) {
            return self.notes.remove(i);
        }
        loop {
            let msg = self.recv();
            if pred(&msg) {
                return msg;
            }
            self.notes.push(msg);
        }
    }
}

fn start(root: &str) -> (Client, thread::JoinHandle<LspServer>) {
    let (client_in_reader, client_in_writer) = memory_pipe();
    let (client_out_reader, client_out_writer) = memory_pipe();
    let server = LspServer::new();
    let handle = thread::spawn(move || {
        let mut server = server;
        server
            .run(BufReader::new(client_in_reader), client_out_writer)
            .expect("server run");
        server
    });
    let mut client = Client {
        to_server: client_in_writer,
        from_server: BufReader::new(client_out_reader),
        next_id: 0,
        notes: Vec::new(),
        log: Vec::new(),
    };
    let init = client.request(
        "initialize",
        Json::obj(vec![
            ("processId", Json::Null),
            ("rootUri", Json::Str(root.to_string())),
        ]),
    );
    assert!(init.get("result").is_some(), "{init:?}");
    client.notify("initialized", Json::Null);
    (client, handle)
}

fn shutdown(client: &mut Client, handle: thread::JoinHandle<LspServer>) -> LspServer {
    client.request("shutdown", Json::Null);
    client.notify("exit", Json::Null);
    handle.join().expect("server thread")
}

fn did_open(client: &mut Client, uri: &str, text: &str) {
    client.notify(
        "textDocument/didOpen",
        Json::obj(vec![(
            "textDocument",
            Json::obj(vec![
                ("uri", Json::Str(uri.to_string())),
                ("languageId", Json::Str("cinch".to_string())),
                ("version", Json::Num(1.0)),
                ("text", Json::Str(text.to_string())),
            ]),
        )]),
    );
}

fn did_change(client: &mut Client, uri: &str, text: &str) {
    client.notify(
        "textDocument/didChange",
        Json::obj(vec![
            ("textDocument", Json::obj(vec![("uri", Json::Str(uri.to_string()))])),
            (
                "contentChanges",
                Json::Arr(vec![Json::obj(vec![("text", Json::Str(text.to_string()))])]),
            ),
        ]),
    );
}

fn position(line: u32, character: u32) -> Json {
    Json::obj(vec![
        ("line", Json::Num(line as f64)),
        ("character", Json::Num(character as f64)),
    ])
}

fn doc(uri: &str) -> Json {
    Json::obj(vec![("uri", Json::Str(uri.to_string()))])
}

/// The diagnostics of one published notification, by URI.
fn published_uri(msg: &Json) -> String {
    msg.get("params")
        .and_then(|p| p.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string()
}

fn published_diagnostics(msg: &Json) -> &[Json] {
    msg.get("params")
        .and_then(|p| p.get("diagnostics"))
        .and_then(|d| d.as_arr())
        .unwrap_or(&[])
}

const A_SRC: &str = "\nmodule A where\n  double :: Int -> Int\n  double n = n + n\n";
const B_SRC: &str =
    "\nmodule B where\n  import A\n  f :: Int -> Int\n  f x = double x\n  g :: Int\n  g = double 21\n";
const B_BAD: &str =
    "\nmodule B where\n  import A\n  f :: Int -> Int\n  f x = True\n  g :: Int\n  g = double 21\n";

#[test]
fn open_and_change_produce_diagnostics_with_correct_spans() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    // Read the initial publishes (no errors anywhere).
    let note = client.wait_note(|m| published_uri(m).ends_with("A.cinch"));
    assert_eq!(note.get("method").and_then(Json::as_str), Some("textDocument/publishDiagnostics"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    did_open(&mut client, "file:///project/B.cinch", B_SRC);
    let note = client.wait_note(|m| published_uri(m).ends_with("B.cinch"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    // A change introduces a type error at `True` (line 5, col 9 of the
    // source: 1-based), reported on B with that span.
    did_change(&mut client, "file:///project/B.cinch", B_BAD);
    let note = client.wait_note(|m| {
        published_uri(m).ends_with("B.cinch") && !published_diagnostics(m).is_empty()
    });
    let diags = published_diagnostics(&note);
    assert_eq!(diags.len(), 1, "{diags:?}");
    let d = &diags[0];
    assert_eq!(d.get("severity").and_then(|s| s.as_num()), Some(1.0), "{d:?}");
    let msg = d.get("message").and_then(|m| m.as_str()).unwrap_or("");
    assert!(msg.contains("type mismatch"), "{msg}");
    let start = d
        .get("range")
        .and_then(|r| r.get("start"))
        .cloned()
        .unwrap_or(Json::Null);
    // 0-based line 4, character 8: the `True` token.
    assert_eq!(start.get("line").and_then(|l| l.as_num()), Some(4.0), "{start:?}");
    assert_eq!(
        start.get("character").and_then(|c| c.as_num()),
        Some(8.0),
        "{start:?}"
    );
    let _ = shutdown(&mut client, handle);
}

#[test]
fn hover_shows_a_scheme() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    did_open(&mut client, "file:///project/B.cinch", B_SRC);
    // `double` in `g = double 21` (1-based 7:7; 0-based 6:6).
    let resp = client.request(
        "textDocument/hover",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 6)),
        ]),
    );
    let value = resp
        .get("result")
        .and_then(|r| r.get("contents"))
        .and_then(|c| c.get("value"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    assert!(value.contains("double"), "{value}");
    assert!(value.contains("Int -> Int"), "{value}");
    let _ = shutdown(&mut client, handle);
}

#[test]
fn definition_crosses_files() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    did_open(&mut client, "file:///project/B.cinch", B_SRC);
    let resp = client.request(
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 6)),
        ]),
    );
    let uri = resp
        .get("result")
        .and_then(|r| r.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("");
    assert_eq!(uri, "file:///project/A.cinch", "{resp:?}");
    let start = resp
        .get("result")
        .and_then(|r| r.get("range"))
        .and_then(|r| r.get("start"))
        .cloned()
        .unwrap_or(Json::Null);
    // A's `double :: Int -> Int` name (1-based 3:3).
    assert_eq!(start.get("line").and_then(|l| l.as_num()), Some(2.0), "{start:?}");
    assert_eq!(
        start.get("character").and_then(|c| c.as_num()),
        Some(2.0),
        "{start:?}"
    );
    let _ = shutdown(&mut client, handle);
}

#[test]
fn completions_offer_imported_and_prelude_names() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    did_open(&mut client, "file:///project/B.cinch", B_SRC);
    // Inside `g = double 21`, completing `d` (0-based 6:8).
    let resp = client.request(
        "textDocument/completion",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 8)),
        ]),
    );
    let items = resp
        .get("result")
        .and_then(|r| r.get("items"))
        .and_then(|i| i.as_arr())
        .unwrap_or(&[]);
    let labels: Vec<&str> = items
        .iter()
        .filter_map(|i| i.get("label").and_then(|l| l.as_str()))
        .collect();
    assert!(labels.contains(&"double"), "{labels:?}");
    // And a prelude name completes.
    let resp = client.request(
        "textDocument/completion",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 2)),
        ]),
    );
    let items = resp
        .get("result")
        .and_then(|r| r.get("items"))
        .and_then(|i| i.as_arr())
        .unwrap_or(&[]);
    let labels: Vec<&str> = items
        .iter()
        .filter_map(|i| i.get("label").and_then(|l| l.as_str()))
        .collect();
    assert!(labels.contains(&"not"), "{labels:?}");
    let _ = shutdown(&mut client, handle);
}

#[test]
fn semantic_tokens_are_delta_encoded() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    let resp = client.request(
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/A.cinch"))]),
    );
    let data: Vec<f64> = resp
        .get("result")
        .and_then(|r| r.get("data"))
        .and_then(|d| d.as_arr())
        .unwrap_or(&[])
        .iter()
        .filter_map(|n| n.as_num())
        .collect();
    assert!(!data.is_empty(), "{resp:?}");
    // Decode the deltas to absolute positions.
    let mut tokens: Vec<(f64, f64, f64, f64)> = Vec::new();
    let (mut line, mut col) = (0.0, 0.0);
    for c in data.chunks(5) {
        line += c[0];
        col = if c[0] == 0.0 { col + c[1] } else { c[1] };
        tokens.push((line, col, c[2], c[3]));
    }
    let keyword = TOKEN_TYPES.iter().position(|t| *t == "keyword").expect("legend") as f64;
    let function = TOKEN_TYPES.iter().position(|t| *t == "function").expect("legend") as f64;
    // `module` (line 2, col 1 of the source: 0-based 1:0), length 6.
    assert!(
        tokens.contains(&(1.0, 0.0, 6.0, keyword)),
        "{tokens:?}"
    );
    // The `double` definition (0-based 2:2), length 6, as a function.
    assert!(
        tokens.contains(&(2.0, 2.0, 6.0, function)),
        "{tokens:?}"
    );
    let _ = shutdown(&mut client, handle);
}

#[test]
fn comments_are_reported_as_semantic_tokens() {
    let (mut client, handle) = start("file:///project");
    // One line comment and a block comment that spans two lines.
    let src = "\nmodule C where\n  -- a note\n  {- block\n     second -}\n  c :: Int\n  c = 1\n";
    did_open(&mut client, "file:///project/C.cinch", src);
    let resp = client.request(
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/C.cinch"))]),
    );
    let data: Vec<f64> = resp
        .get("result")
        .and_then(|r| r.get("data"))
        .and_then(|d| d.as_arr())
        .unwrap_or(&[])
        .iter()
        .filter_map(|n| n.as_num())
        .collect();
    assert!(!data.is_empty(), "{resp:?}");
    let mut tokens: Vec<(f64, f64, f64, f64)> = Vec::new();
    let (mut line, mut col) = (0.0, 0.0);
    for c in data.chunks(5) {
        line += c[0];
        col = if c[0] == 0.0 { col + c[1] } else { c[1] };
        tokens.push((line, col, c[2], c[3]));
    }
    let comment = TOKEN_TYPES.iter().position(|t| *t == "comment").expect("legend") as f64;
    // `-- a note` (0-based line 2, column 2, 9 characters).
    assert!(tokens.contains(&(2.0, 2.0, 9.0, comment)), "{tokens:?}");
    // The block comment reports one token per line: `{- block`, then the
    // remainder of the line that closes it (the whole line is comment text).
    assert!(tokens.contains(&(3.0, 2.0, 8.0, comment)), "{tokens:?}");
    assert!(tokens.contains(&(4.0, 0.0, 14.0, comment)), "{tokens:?}");
    let _ = shutdown(&mut client, handle);
}

#[test]
fn a_change_rechecks_the_file_and_its_dependents_only() {
    let (mut client, handle) = start("file:///project");
    // A is independent of D: B imports A, C imports B, D imports
    // nothing.
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    did_open(
        &mut client,
        "file:///project/B.cinch",
        "\nmodule B where\n  import A\n  f :: Int -> Int\n  f x = double x\n",
    );
    did_open(
        &mut client,
        "file:///project/C.cinch",
        "\nmodule C where\n  import B\n  h :: Int -> Int\n  h x = f x\n",
    );
    did_open(
        &mut client,
        "file:///project/D.cinch",
        "\nmodule D where\n  k :: Int\n  k = 1\n",
    );
    did_change(
        &mut client,
        "file:///project/A.cinch",
        "\nmodule A where\n  double :: Int -> Int\n  double n = n + n + 0\n",
    );
    // Drain the publishes, then read the recheck report off the server.
    let note = client.wait_note(|m| published_uri(m).ends_with("D.cinch"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    let server = shutdown(&mut client, handle);
    let report = server.reports().last().expect("a report");
    // The change rechecked A and its dependents B and C via the
    // `.cinchif` chain, and kept D's cached interface: not the whole
    // project.
    assert_eq!(report.rechecked, vec!["A", "B", "C"], "{report:?}");
    assert_eq!(report.kept, vec!["D"], "{report:?}");
    assert!(report.chain_ok, "{report:?}");
}

#[test]
fn no_output_carries_an_absolute_path() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.cinch", A_SRC);
    did_open(&mut client, "file:///project/B.cinch", B_BAD);
    client.request(
        "textDocument/hover",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 6)),
        ]),
    );
    client.request(
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 6)),
        ]),
    );
    client.request(
        "textDocument/completion",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.cinch")),
            ("position", position(6, 8)),
        ]),
    );
    client.request(
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/B.cinch"))]),
    );
    let _server = shutdown(&mut client, handle);
    // Every message the server sent: strings outside `uri` fields carry
    // no absolute path at all, and every document URI stays under the
    // root URI the client itself supplied (the workspace-relative keys
    // the analysis produces are the only path form in the text).
    assert!(!client.log.is_empty());
    for msg in &client.log {
        walk_strings(msg, &[]);
    }
}

/// Assert every string outside `uri` fields is free of absolute paths.
fn walk_strings(v: &Json, key_path: &[String]) {
    match v {
        Json::Str(s) => {
            let in_uri = key_path.last().map(|k| k == "uri").unwrap_or(false);
            if in_uri {
                assert!(
                    s.starts_with("file:///project/"),
                    "a URI escaped the workspace root: {s}"
                );
            } else {
                for bad in ["/project/", "/Users/", "/home/", "/tmp/", "/var/", "/private/"] {
                    assert!(!s.contains(bad), "absolute path {bad:?} in output: {s:?}");
                }
                assert!(!s.starts_with('/'), "absolute path in output: {s:?}");
            }
        }
        Json::Arr(items) => {
            for item in items {
                walk_strings(item, key_path);
            }
        }
        Json::Obj(m) => {
            for (k, item) in m {
                let mut path = key_path.to_vec();
                path.push(k.clone());
                walk_strings(item, &path);
            }
        }
        _ => {}
    }
}

/// `cinch lsp` runs the server: the subcommand's real process speaks
/// the handshake over its stdio in the harness's framing shape
/// (initialize, initialized, shutdown, exit) and exits zero.
#[test]
fn the_lsp_subcommand_runs_the_server_over_stdio() {
    use cinch::lsp::read_frame;
    use std::process::{Command, Stdio};

    let mut child = Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn cinch lsp");
    let request = |id: f64, method: &str| {
        Json::obj(vec![
            ("jsonrpc", Json::Str("2.0".to_string())),
            ("id", Json::Num(id)),
            ("method", Json::Str(method.to_string())),
            ("params", Json::Null),
        ])
        .to_text()
    };
    let notify = |method: &str| {
        Json::obj(vec![
            ("jsonrpc", Json::Str("2.0".to_string())),
            ("method", Json::Str(method.to_string())),
            ("params", Json::Null),
        ])
        .to_text()
    };
    {
        let stdin = child.stdin.as_mut().expect("child stdin");
        let init = Json::obj(vec![
            ("jsonrpc", Json::Str("2.0".to_string())),
            ("id", Json::Num(1.0)),
            ("method", Json::Str("initialize".to_string())),
            ("params", Json::obj(vec![
                ("processId", Json::Null),
                ("rootUri", Json::Str("file:///project".to_string())),
            ])),
        ])
        .to_text();
        for body in [
            init,
            notify("initialized"),
            request(2.0, "shutdown"),
            notify("exit"),
        ] {
            stdin.write_all(&frame(&body)).expect("write to cinch lsp");
        }
        stdin.flush().expect("flush cinch lsp stdin");
    }
    drop(child.stdin.take());
    let mut reader = BufReader::new(child.stdout.take().expect("child stdout"));
    let mut msgs = Vec::new();
    while let Some(body) = read_frame(&mut reader).expect("frame from cinch lsp") {
        msgs.push(parse_json(&body).expect("json from cinch lsp"));
    }
    let status = child.wait().expect("wait for cinch lsp");
    assert!(status.success(), "cinch lsp exits zero: {status:?}");
    let response = |id: f64| {
        msgs.iter()
            .find(|m| m.get("id").and_then(|i| i.as_num()) == Some(id))
            .unwrap_or_else(|| panic!("no response to request {id}: {msgs:?}"))
    };
    assert!(response(1.0).get("result").is_some(), "{msgs:?}");
    assert!(response(2.0).get("result").is_some(), "{msgs:?}");
}

// ------------------------------------------------------- core documents
//
// The Core path (design note section 8.4) answers from the document text
// alone: these tests drive `LspServer::handle` with the wire bodies, as a
// client would, and inspect the published diagnostics and responses.

const CORE_SRC: &str = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.first :: Int -> Int
  @span tests/core/lsp.cinch:1:1
  @params v0 :: Int @owned
  @stage(checked)
  body (local v0)
end
unit Main.second :: Int
  @span tests/core/lsp.cinch:5:1
  @stage(checked)
  body (app (global Main.first) (int 1))
end
";

const CORE_FOLDS: &str = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.alts :: Int
  @span tests/core/lsp.cinch:1:1
  @stage(checked)
  body (case (int 0)
    ((int 1)
      (int 2))
    ((int 3)
      (int 4)))
end
unit Main.lammy :: Int
  @span tests/core/lsp.cinch:10:1
  @stage(checked)
  body (lam (v0)
    (int 1))
end
";

/// The 0-based position of `needle` in `text`.
fn core_at(text: &str, needle: &str) -> (u32, u32) {
    let byte = text.find(needle).expect("the needle is in the text");
    let before = &text[..byte];
    let line = before.matches('\n').count() as u32;
    let col = before
        .rsplit('\n')
        .next()
        .unwrap_or("")
        .chars()
        .map(|c| c.len_utf16() as u32)
        .sum();
    (line, col)
}

fn request_raw(server: &mut LspServer, id: f64, method: &str, params: Json) -> Json {
    let body = Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("id", Json::Num(id)),
        ("method", Json::Str(method.to_string())),
        ("params", params),
    ])
    .to_text();
    let out = server.handle(&body);
    assert_eq!(out.len(), 1, "{method} answers once: {out:?}");
    parse_json(&out[0]).expect("the response is JSON")
}

fn call(server: &mut LspServer, method: &str, params: Json) -> Json {
    request_raw(server, 2.0, method, params)
}

fn start_core() -> (LspServer, Json) {
    let mut server = LspServer::new();
    let init = request_raw(
        &mut server,
        1.0,
        "initialize",
        Json::obj(vec![("rootUri", Json::Str("file:///project".to_string()))]),
    );
    (server, init)
}

/// Open a document and return the notifications the server published.
fn open_core(server: &mut LspServer, uri: &str, text: &str) -> Vec<Json> {
    let body = Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("method", Json::Str("textDocument/didOpen".to_string())),
        (
            "params",
            Json::obj(vec![(
                "textDocument",
                Json::obj(vec![
                    ("uri", Json::Str(uri.to_string())),
                    ("languageId", Json::Str("cinch-core".to_string())),
                    ("version", Json::Num(1.0)),
                    ("text", Json::Str(text.to_string())),
                ]),
            )]),
        ),
    ])
    .to_text();
    server
        .handle(&body)
        .iter()
        .map(|out| parse_json(out).expect("the notification is JSON"))
        .collect()
}

fn change_core(server: &mut LspServer, uri: &str, text: &str) -> Vec<Json> {
    let body = Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("method", Json::Str("textDocument/didChange".to_string())),
        (
            "params",
            Json::obj(vec![
                ("textDocument", Json::obj(vec![("uri", Json::Str(uri.to_string()))])),
                (
                    "contentChanges",
                    Json::Arr(vec![Json::obj(vec![("text", Json::Str(text.to_string()))])]),
                ),
            ]),
        ),
    ])
    .to_text();
    server
        .handle(&body)
        .iter()
        .map(|out| parse_json(out).expect("the notification is JSON"))
        .collect()
}

fn hover_at(server: &mut LspServer, uri: &str, line: u32, col: u32) -> Json {
    call(
        server,
        "textDocument/hover",
        Json::obj(vec![
            ("textDocument", doc(uri)),
            ("position", position(line, col)),
        ]),
    )
}

fn hover_value(resp: &Json) -> String {
    resp.get("result")
        .and_then(|result| result.get("contents"))
        .and_then(|contents| contents.get("value"))
        .and_then(Json::as_str)
        .unwrap_or_else(|| panic!("no hover value: {resp:?}"))
        .to_string()
}

fn core_position(line: u32, col: u32) -> Json {
    position(line, col)
}

#[test]
fn initialize_advertises_the_capabilities_the_server_answers() {
    let (_, init) = start_core();
    let caps = init
        .get("result")
        .and_then(|result| result.get("capabilities"))
        .expect("capabilities");
    for provider in [
        "hoverProvider",
        "definitionProvider",
        "documentSymbolProvider",
        "foldingRangeProvider",
        "workspaceSymbolProvider",
    ] {
        assert!(
            matches!(caps.get(provider), Some(Json::Bool(true))),
            "{provider} is advertised: {caps:?}"
        );
    }
    assert!(
        matches!(caps.get("semanticTokensProvider"), Some(Json::Obj(_))),
        "semantic tokens stay advertised"
    );
    let legend = caps
        .get("semanticTokensProvider")
        .and_then(|provider| provider.get("legend"))
        .and_then(|legend| legend.get("tokenTypes"))
        .and_then(Json::as_arr)
        .expect("a token legend");
    let legend: Vec<&str> = legend.iter().filter_map(Json::as_str).collect();
    assert!(legend.contains(&"namespace"), "{legend:?}");
    assert!(legend.contains(&"property"), "{legend:?}");
}

#[test]
fn a_clean_core_document_publishes_no_diagnostics() {
    let (mut server, _) = start_core();
    let notes = open_core(&mut server, "file:///project/main.core", CORE_SRC);
    assert_eq!(notes.len(), 1, "{notes:?}");
    assert!(
        published_diagnostics(&notes[0]).is_empty(),
        "{:?}",
        notes[0]
    );
    // A Core document never reaches the analysis.
    assert!(server.reports().is_empty(), "a Core document never rechecks");
    // And closing it clears whatever it published.
    let body = Json::obj(vec![
        ("jsonrpc", Json::Str("2.0".to_string())),
        ("method", Json::Str("textDocument/didClose".to_string())),
        (
            "params",
            Json::obj(vec![("textDocument", doc("file:///project/main.core"))]),
        ),
    ])
    .to_text();
    let out = server.handle(&body);
    assert_eq!(out.len(), 1);
    let note = parse_json(&out[0]).expect("JSON");
    assert!(published_diagnostics(&note).is_empty());
}

#[test]
fn an_unbound_local_is_one_located_diagnostic() {
    let (mut server, _) = start_core();
    let clean = open_core(&mut server, "file:///project/broken.core", CORE_SRC);
    assert!(published_diagnostics(&clean[0]).is_empty());
    let text = CORE_SRC.replace("body (local v0)", "body (local $v9)");
    let notes = change_core(&mut server, "file:///project/broken.core", &text);
    let diags = published_diagnostics(&notes[0]);
    assert_eq!(diags.len(), 1, "{diags:?}");
    let diag = &diags[0];
    assert_eq!(
        diag.get("code").and_then(Json::as_str),
        Some("core/unbound-local"),
        "{diag:?}"
    );
    assert_eq!(diag.get("severity").and_then(Json::as_num), Some(1.0));
    let (line, col) = core_at(&text, "(local $v9)");
    let start = diag
        .get("range")
        .and_then(|range| range.get("start"))
        .expect("a range start");
    assert_eq!(start.get("line").and_then(Json::as_num), Some(line as f64));
    assert_eq!(
        start.get("character").and_then(Json::as_num),
        Some(col as f64)
    );
    assert!(server.reports().is_empty());
}

#[test]
fn core_semantic_tokens_cover_a_unit_name() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/main.core", CORE_SRC);
    let resp = call(
        &mut server,
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/main.core"))]),
    );
    let data = resp
        .get("result")
        .and_then(|result| result.get("data"))
        .and_then(Json::as_arr)
        .expect("a token payload");
    assert!(!data.is_empty(), "{resp:?}");
    let nums: Vec<u32> = data
        .iter()
        .map(|value| value.as_num().expect("a number") as u32)
        .collect();
    assert_eq!(nums.len() % 5, 0, "{nums:?}");
    let mut line = 0u32;
    let mut col = 0u32;
    let mut tokens = Vec::new();
    for chunk in nums.chunks(5) {
        if chunk[0] > 0 {
            line += chunk[0];
            col = chunk[1];
        } else {
            col += chunk[1];
        }
        tokens.push((line, col, chunk[2], chunk[3]));
    }
    let (want_line, want_col) = core_at(CORE_SRC, "Main.first ::");
    let namespace = TOKEN_TYPES
        .iter()
        .position(|kind| *kind == "namespace")
        .expect("namespace is in the legend") as u32;
    assert!(
        tokens.contains(&(want_line, want_col, "Main.first".len() as u32, namespace)),
        "{tokens:?}"
    );
    let property = TOKEN_TYPES
        .iter()
        .position(|kind| *kind == "property")
        .expect("property is in the legend") as u32;
    let (span_line, span_col) = core_at(CORE_SRC, "span tests/core");
    assert!(
        tokens.contains(&(span_line, span_col, "span".len() as u32, property)),
        "{tokens:?}"
    );
}

#[test]
fn core_hover_shows_a_unit_a_reference_and_a_local() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/main.core", CORE_SRC);
    let (line, col) = core_at(CORE_SRC, "Main.first ::");
    let value = hover_value(&hover_at(&mut server, "file:///project/main.core", line, col));
    assert!(value.contains("type: `Int -> Int`"), "{value}");
    assert!(value.contains("stage: `checked`"), "{value}");
    assert!(value.contains("tests/core/lsp.cinch:1:1"), "{value}");
    // The `(global Main.first)` reference reports the defining unit's type.
    let (line, col) = core_at(CORE_SRC, "Main.first) (int 1)");
    let value = hover_value(&hover_at(&mut server, "file:///project/main.core", line, col));
    assert!(value.contains("`Main.first :: Int -> Int`"), "{value}");
    // The local Reference reports the binder `@params` printed.
    let (line, col) = core_at(CORE_SRC, "(local v0)");
    let value = hover_value(&hover_at(
        &mut server,
        "file:///project/main.core",
        line,
        col + 8,
    ));
    assert!(value.contains("v0 :: Int @owned"), "{value}");
}

#[test]
fn core_definition_jumps_only_inside_the_file() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/main.core", CORE_SRC);
    let (line, col) = core_at(CORE_SRC, "Main.first) (int 1)");
    let resp = call(
        &mut server,
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/main.core")),
            ("position", core_position(line, col)),
        ]),
    );
    let result = resp.get("result").expect("a location: {resp:?}");
    assert_eq!(
        result.get("uri").and_then(Json::as_str),
        Some("file:///project/main.core")
    );
    let (want_line, want_col) = core_at(CORE_SRC, "Main.first ::");
    let start = result
        .get("range")
        .and_then(|range| range.get("start"))
        .expect("a range start");
    assert_eq!(
        start.get("line").and_then(Json::as_num),
        Some(want_line as f64)
    );
    assert_eq!(
        start.get("character").and_then(Json::as_num),
        Some(want_col as f64)
    );
    // An imported name reports no location, never a wrong one.
    let imported = CORE_SRC.replace("(global Main.first)", "(global Other.thing)");
    open_core(&mut server, "file:///project/imported.core", &imported);
    let (line, col) = core_at(&imported, "Other.thing");
    let resp = call(
        &mut server,
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/imported.core")),
            ("position", core_position(line, col)),
        ]),
    );
    assert!(matches!(resp.get("result"), Some(Json::Null)), "{resp:?}");
}

#[test]
fn document_symbols_list_units_in_order() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/main.core", CORE_SRC);
    let resp = call(
        &mut server,
        "textDocument/documentSymbol",
        Json::obj(vec![("textDocument", doc("file:///project/main.core"))]),
    );
    let items = resp
        .get("result")
        .and_then(Json::as_arr)
        .expect("a symbol list");
    let names: Vec<&str> = items
        .iter()
        .filter_map(|item| item.get("name").and_then(Json::as_str))
        .collect();
    assert_eq!(names, ["Main.first", "Main.second"]);
    let children = items[1]
        .get("children")
        .and_then(Json::as_arr)
        .expect("the body's forms");
    assert!(
        children
            .iter()
            .any(|child| child.get("name").and_then(Json::as_str) == Some("app")),
        "{children:?}"
    );
    // A surface document has no document symbols from this server.
    let resp = call(
        &mut server,
        "textDocument/documentSymbol",
        Json::obj(vec![("textDocument", doc("file:///project/A.cinch"))]),
    );
    assert_eq!(resp.get("result").and_then(Json::as_arr), Some(&[][..]));
}

#[test]
fn folding_covers_units_lam_and_case_alternatives() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/folds.core", CORE_FOLDS);
    let resp = call(
        &mut server,
        "textDocument/foldingRange",
        Json::obj(vec![("textDocument", doc("file:///project/folds.core"))]),
    );
    let ranges = resp.get("result").and_then(Json::as_arr).expect("ranges");
    let pairs: Vec<(u32, u32)> = ranges
        .iter()
        .map(|range| {
            (
                range.get("startLine").and_then(Json::as_num).expect("start") as u32,
                range.get("endLine").and_then(Json::as_num).expect("end") as u32,
            )
        })
        .collect();
    // The first unit's block spans its `unit` line to its `end`.
    let (unit_line, _) = core_at(CORE_FOLDS, "unit Main.alts");
    assert!(
        pairs.iter().any(|(start, _)| *start == unit_line),
        "{pairs:?}"
    );
    // A case alternative that wraps folds from its pattern line.
    let (alt_line, _) = core_at(CORE_FOLDS, "((int 1)");
    assert!(pairs.iter().any(|(start, _)| *start == alt_line), "{pairs:?}");
    // And a `lam` folds from its own line.
    let (lam_line, _) = core_at(CORE_FOLDS, "(lam (v0)");
    assert!(pairs.iter().any(|(start, _)| *start == lam_line), "{pairs:?}");
}

#[test]
fn workspace_symbols_cover_every_open_core_document() {
    let (mut server, _) = start_core();
    open_core(&mut server, "file:///project/main.core", CORE_SRC);
    open_core(&mut server, "file:///project/folds.core", CORE_FOLDS);
    let resp = call(
        &mut server,
        "workspace/symbol",
        Json::obj(vec![("query", Json::Str(String::new()))]),
    );
    let items = resp.get("result").and_then(Json::as_arr).expect("symbols");
    let names: Vec<&str> = items
        .iter()
        .filter_map(|item| item.get("name").and_then(Json::as_str))
        .collect();
    assert_eq!(
        names,
        [
            "folds.core: Main.alts",
            "folds.core: Main.lammy",
            "main.core: Main.first",
            "main.core: Main.second"
        ],
        "every unit of every open Core document, by file"
    );
    let query = call(
        &mut server,
        "workspace/symbol",
        Json::obj(vec![("query", Json::Str("second".to_string()))]),
    );
    let items = query.get("result").and_then(Json::as_arr).expect("symbols");
    assert_eq!(items.len(), 1, "{items:?}");
}

#[test]
fn a_cinch_document_still_gets_its_analysis_diagnostics() {
    let (mut server, _) = start_core();
    let notes = open_core(&mut server, "file:///project/B.cinch", B_BAD);
    // The analysis publishes for every open surface document.
    assert!(!notes.is_empty(), "the analysis answered");
    let note = notes
        .iter()
        .find(|note| published_uri(note).ends_with("B.cinch"))
        .expect("a B.cinch publish");
    let diags = published_diagnostics(note);
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert!(
        diags[0]
            .get("message")
            .and_then(Json::as_str)
            .unwrap_or("")
            .contains("type mismatch"),
        "{:?}",
        diags[0]
    );
    assert!(!server.reports().is_empty(), "the surface path rechecks");
}

#[test]
fn a_missing_span_warns_and_a_bad_header_is_a_parse_error() {
    let (mut server, _) = start_core();
    // A record without `@span` is warned about, not refused: a dump carries
    // no source position.
    let unspanned = CORE_SRC.replace("  @span tests/core/lsp.cinch:5:1\n", "");
    let notes = open_core(&mut server, "file:///project/unspanned.core", &unspanned);
    let diags = published_diagnostics(&notes[0]);
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert_eq!(diags[0].get("severity").and_then(Json::as_num), Some(2.0));
    assert_eq!(
        diags[0].get("code").and_then(Json::as_str),
        Some("core/span-missing")
    );
    // A file the reader cannot start is a parse finding, not a verifier one.
    let notes = open_core(&mut server, "file:///project/bad.core", "core 2\nmodule Main\n");
    let diags = published_diagnostics(&notes[0]);
    assert_eq!(diags.len(), 1, "{diags:?}");
    assert_eq!(
        diags[0].get("code").and_then(Json::as_str),
        Some("core/parse")
    );
    assert_eq!(diags[0].get("severity").and_then(Json::as_num), Some(1.0));
}
