//! P22 acceptance: the language server driven over in-memory stdio.
//!
//! The server runs its stdio loop against pipe-backed in-memory streams
//! (`Content-Length` framed JSON-RPC, exactly as a client would speak
//! it): initialize, didOpen/didChange diagnostics with correct spans,
//! hover, definition across files, completion, semantic tokens, the
//! incremental recheck report over the `.sliface` chain, and the
//! no-absolute-paths rule.

use safelang::lsp::{frame, parse_json, Json, LspServer, TOKEN_TYPES};
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
                ("languageId", Json::Str("safelang".to_string())),
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
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    // Read the initial publishes (no errors anywhere).
    let note = client.wait_note(|m| published_uri(m).ends_with("A.lang"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    did_open(&mut client, "file:///project/B.lang", B_SRC);
    let note = client.wait_note(|m| published_uri(m).ends_with("B.lang"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    // A change introduces a type error at `True` (line 5, col 9 of the
    // source: 1-based), reported on B with that span.
    did_change(&mut client, "file:///project/B.lang", B_BAD);
    let note = client.wait_note(|m| {
        published_uri(m).ends_with("B.lang") && !published_diagnostics(m).is_empty()
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
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    did_open(&mut client, "file:///project/B.lang", B_SRC);
    // `double` in `g = double 21` (1-based 7:7; 0-based 6:6).
    let resp = client.request(
        "textDocument/hover",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
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
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    did_open(&mut client, "file:///project/B.lang", B_SRC);
    let resp = client.request(
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
            ("position", position(6, 6)),
        ]),
    );
    let uri = resp
        .get("result")
        .and_then(|r| r.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("");
    assert_eq!(uri, "file:///project/A.lang", "{resp:?}");
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
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    did_open(&mut client, "file:///project/B.lang", B_SRC);
    // Inside `g = double 21`, completing `d` (0-based 6:8).
    let resp = client.request(
        "textDocument/completion",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
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
            ("textDocument", doc("file:///project/B.lang")),
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
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    let resp = client.request(
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/A.lang"))]),
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
fn a_change_rechecks_the_file_and_its_dependents_only() {
    let (mut client, handle) = start("file:///project");
    // A is independent of D: B imports A, C imports B, D imports
    // nothing.
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    did_open(
        &mut client,
        "file:///project/B.lang",
        "\nmodule B where\n  import A\n  f :: Int -> Int\n  f x = double x\n",
    );
    did_open(
        &mut client,
        "file:///project/C.lang",
        "\nmodule C where\n  import B\n  h :: Int -> Int\n  h x = f x\n",
    );
    did_open(
        &mut client,
        "file:///project/D.lang",
        "\nmodule D where\n  k :: Int\n  k = 1\n",
    );
    did_change(
        &mut client,
        "file:///project/A.lang",
        "\nmodule A where\n  double :: Int -> Int\n  double n = n + n + 0\n",
    );
    // Drain the publishes, then read the recheck report off the server.
    let note = client.wait_note(|m| published_uri(m).ends_with("D.lang"));
    assert!(published_diagnostics(&note).is_empty(), "{note:?}");
    let server = shutdown(&mut client, handle);
    let report = server.reports().last().expect("a report");
    // The change rechecked A and its dependents B and C via the
    // `.sliface` chain, and kept D's cached interface: not the whole
    // project.
    assert_eq!(report.rechecked, vec!["A", "B", "C"], "{report:?}");
    assert_eq!(report.kept, vec!["D"], "{report:?}");
    assert!(report.chain_ok, "{report:?}");
}

#[test]
fn no_output_carries_an_absolute_path() {
    let (mut client, handle) = start("file:///project");
    did_open(&mut client, "file:///project/A.lang", A_SRC);
    did_open(&mut client, "file:///project/B.lang", B_BAD);
    client.request(
        "textDocument/hover",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
            ("position", position(6, 6)),
        ]),
    );
    client.request(
        "textDocument/definition",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
            ("position", position(6, 6)),
        ]),
    );
    client.request(
        "textDocument/completion",
        Json::obj(vec![
            ("textDocument", doc("file:///project/B.lang")),
            ("position", position(6, 8)),
        ]),
    );
    client.request(
        "textDocument/semanticTokens/full",
        Json::obj(vec![("textDocument", doc("file:///project/B.lang"))]),
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

/// `safelang lsp` runs the server: the subcommand's real process speaks
/// the handshake over its stdio in the harness's framing shape
/// (initialize, initialized, shutdown, exit) and exits zero.
#[test]
fn the_lsp_subcommand_runs_the_server_over_stdio() {
    use safelang::lsp::read_frame;
    use std::process::{Command, Stdio};

    let mut child = Command::new(env!("CARGO_BIN_EXE_safelang"))
        .arg("lsp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn safelang lsp");
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
            stdin.write_all(&frame(&body)).expect("write to safelang lsp");
        }
        stdin.flush().expect("flush safelang lsp stdin");
    }
    drop(child.stdin.take());
    let mut reader = BufReader::new(child.stdout.take().expect("child stdout"));
    let mut msgs = Vec::new();
    while let Some(body) = read_frame(&mut reader).expect("frame from safelang lsp") {
        msgs.push(parse_json(&body).expect("json from safelang lsp"));
    }
    let status = child.wait().expect("wait for safelang lsp");
    assert!(status.success(), "safelang lsp exits zero: {status:?}");
    let response = |id: f64| {
        msgs.iter()
            .find(|m| m.get("id").and_then(|i| i.as_num()) == Some(id))
            .unwrap_or_else(|| panic!("no response to request {id}: {msgs:?}"))
    };
    assert!(response(1.0).get("result").is_some(), "{msgs:?}");
    assert!(response(2.0).get("result").is_some(), "{msgs:?}");
}
