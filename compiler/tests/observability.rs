//! End-to-end tests for observability (P32): the `Log r`/`Trace r`
//! capabilities in the `Io r` mold (spans as lexical scopes, the trace
//! context carried by the runtime identity `r`), structured `Json`
//! records with the OTLP/JSON export over `Text.Json`, W3C
//! `traceparent` propagation, and the DX: `log`/`metric`/`span` as
//! ordinary effect operations with lovely ergonomics.
//!
//! Every program is a real `Main` compiled and run through the same
//! driver the CLI uses. The observability modules beside it are the
//! checked in `compiler/prelude/Effect/*.cinch` and
//! `compiler/prelude/Text/Otlp.cinch` sources (the driver does not
//! embed them yet: the P30 note's shape; when Main folds them into the
//! embedded prelude and the facade, an explicit source module still
//! shadows the copy). The canonical printer beside them is the P31
//! `Text.Json` toolkit, copied in the same way.
//!
//! THE DX SAMPLE (reported, the row 3 acceptance bar): the program in
//! `dx_program_reads_naturally_with_levels_and_fields` is the
//! ergonomics sample verbatim. Levels are bare values (`log info ...`,
//! `log debug ...`), fields are one `fields [...]` list of pairs, and
//! a span scope opens with `span name fields` and closes with
//! `end-span name` (or runs to the tail of its computation with no
//! close at all). The handler clauses are one line each over the
//! record builders.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};

const LOG_SRC: &str = include_str!("../prelude/Effect/Log.cinch");
const TRACE_SRC: &str = include_str!("../prelude/Effect/Trace.cinch");
const OTLP_SRC: &str = include_str!("../prelude/Text/Otlp.cinch");
const LEX_SRC: &str = include_str!("../prelude/Text/Lex.cinch");
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");
const PARSE_SRC: &str = include_str!("../prelude/Text/Parse.cinch");
const JSON_SRC: &str = include_str!("../prelude/Text/Json.cinch");
const BUILDER_SRC: &str = include_str!("../prelude/Text/Builder.cinch");

// ----------------------------------------------------------------
// Shared harness
// ----------------------------------------------------------------

/// A self-contained project: the observability and toolkit sources
/// beside a `Main`.
fn write_project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "cinch_obs_{}_{}_{}",
        name,
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    let modules: [(&str, &str); 8] = [
        ("Effect.Log.cinch", LOG_SRC),
        ("Effect.Trace.cinch", TRACE_SRC),
        ("Text.Otlp.cinch", OTLP_SRC),
        ("Text.Lex.cinch", LEX_SRC),
        ("Text.Scan.cinch", SCAN_SRC),
        ("Text.Parse.cinch", PARSE_SRC),
        ("Text.Json.cinch", JSON_SRC),
        ("Text.Builder.cinch", BUILDER_SRC),
    ];
    for (file, src) in modules {
        std::fs::write(dir.join(file), src).unwrap_or_else(|e| panic!("write {file}: {e}"));
    }
    for (name, src) in files {
        std::fs::write(dir.join(name), src).unwrap_or_else(|e| panic!("write {name}: {e}"));
    }
    dir
}

/// Run a driver invocation on a thread with a large stack (the
/// checker recurses over a program's elaborated code; the toolkit
/// programs are bigger than a default test thread's stack).
fn drive(mode: Mode, entry: &Path, out: Option<&Path>) -> Result<i32, String> {
    let entry = entry.to_path_buf();
    let out = out.map(|p| p.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &entry, out.as_deref()))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .map_err(|e| e.to_string())
}

/// Compile and run a `Main`; the exit code is the program's
/// `main` result (0 on success, the failing case number on a named
/// failure).
fn run_main(name: &str, main: &str) -> i32 {
    let dir = write_project(name, &[("main.cinch", main)]);
    let code = drive(Mode::Run, &dir.join("main.cinch"), None)
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"));
    let _ = std::fs::remove_dir_all(&dir);
    code
}

fn expect_exit(name: &str, main: &str, expected: i32) {
    let code = run_main(name, main);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

/// Compile a `Main` to an executable and return it (kept alive in its
/// project directory until `remove_dir`).
fn build_exe(name: &str, main: &str) -> (PathBuf, PathBuf) {
    let dir = write_project(name, &[("main.cinch", main)]);
    let exe = dir.join("program");
    let code = drive(Mode::Build, &dir.join("main.cinch"), Some(&exe))
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"));
    assert_eq!(code, 0, "{name}: build failed");
    (dir, exe)
}

/// Run the executable with the runtime's input source fed from
/// `input` and its output sink captured (`run_program_io`'s shape).
fn run_exe(exe: &Path, input: &[u8]) -> (i32, String) {
    use std::io::Write;
    let mut child = std::process::Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()));
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(input)
        .expect("feed input");
    let out = child.wait_with_output().expect("wait for child");
    (
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// A program expected to be refused at check time; the error text.
fn check_error(name: &str, main: &str) -> String {
    let dir = write_project(name, &[("main.cinch", main)]);
    let res = drive(Mode::Check, &dir.join("main.cinch"), None);
    let _ = std::fs::remove_dir_all(&dir);
    match res {
        Ok(_) => panic!("{name}: expected a check error, but the program checked"),
        Err(e) => e,
    }
}

fn expect_err(name: &str, main: &str, needle: &str) {
    let msg = check_error(name, main);
    assert!(
        msg.contains(needle),
        "{name}: error {msg:?} does not contain {needle:?}"
    );
}

// ----------------------------------------------------------------
// The collecting handler shape (every program shares it)
// ----------------------------------------------------------------

/// The two collecting handlers over the record builders: log and
/// metric records consed in order, span scopes wrapped by
/// `span-scope`, scope closes consing their marker.
const COLLECTORS: &str = r#"        let collect-logs = handler Log r where
                return x -> Nil
                log lvl msg fs resume k -> Cons (log-record lvl msg fs) (k ())
                metric kind name value fs resume k -> Cons (metric-record kind name value fs) (k ()) in
        let collect-spans = handler Trace r where
                return x -> x
                span name fs resume k -> span-scope name fs (k ())
                end-span name resume k -> Cons (end-marker name) (k ()) in"#;

// ----------------------------------------------------------------
// 1. Structured records: span scopes, logs and metrics under a
//    collecting handler
// ----------------------------------------------------------------

#[test]
fn collecting_handler_nests_span_scopes_by_lexical_scope() {
    // `span` opens a lexical scope and `end-span` closes it, so the
    // collected records nest exactly as the sources nest: the
    // `db.query` scope holds the metric, `response.2xx` lands in
    // `http.request` but outside `db.query`, and the pre-span log
    // stays at the top level. Fields ride every record.
    let main = format!(
        r#"module Main where
  import Effect.Log (log, metric, info, counter, no-fields, fields, num, str, arr, log-record, metric-record)
  import Effect.Trace (span, end-span, end-marker, span-scope)
  import Text.Json (canonical-json)
  import Data.Json
  import Data.String

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
{COLLECTORS}
    case handle
        (handle
          (do
            log info "server.started" (fields [("port", num 8080)])
            span "http.request" (fields [("method", str "GET")])
            log info "auth.ok" no-fields
            span "db.query" (fields [("table", str "users")])
            metric counter "db.rows" 42 no-fields
            end-span "db.query"
            log info "response.2xx" no-fields
            end-span "http.request")
          with collect-logs)
        with collect-spans of
      records -> case ioWrite (string-to-bytes (string-concat (canonical-json (arr records)) "\n")) of
        w -> 0
"#
    );
    let (dir, exe) = build_exe("records", &main);
    let (code, out) = run_exe(&exe, b"");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "records: expected exit 0, got {code}");
    assert_eq!(
        out,
        concat!(
            r#"[{"fields":{"port":8080},"level":"info","message":"server.started"},"#,
            r#"{"children":[{"fields":{},"level":"info","message":"auth.ok"},"#,
            r#"{"children":[{"fields":{},"metric":"counter","name":"db.rows","value":42}],"#,
            r#""fields":{"table":"users"},"name":"db.query"},"#,
            r#"{"fields":{},"level":"info","message":"response.2xx"}],"#,
            r#""fields":{"method":"GET"},"name":"http.request"}]"#,
            "\n"
        ),
        "records: collected structure mismatch"
    );
}

#[test]
fn scope_brackets_close_sibling_scopes() {
    // Sibling scopes at one level: `request.a` brackets its body and
    // `request.b` runs to the tail (no close needed). `db` nests in
    // `request.b` and closes before `b.end`.
    let main = format!(
        r#"module Main where
  import Effect.Log (log, metric, info, counter, no-fields, fields, num, str, arr, log-record, metric-record)
  import Effect.Trace (span, end-span, end-marker, span-scope)
  import Text.Json (canonical-json)
  import Data.Json
  import Data.String

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
{COLLECTORS}
    case handle
        (handle
          (do
            span "request.a" (fields [("id", num 1)])
            log info "a.start" no-fields
            end-span "request.a"
            span "request.b" (fields [("id", num 2)])
            log info "b.start" no-fields
            span "db" no-fields
            log info "b.query" no-fields
            end-span "db"
            log info "b.end" no-fields)
          with collect-logs)
        with collect-spans of
      records -> case ioWrite (string-to-bytes (string-concat (canonical-json (arr records)) "\n")) of
        w -> 0
"#
    );
    let (dir, exe) = build_exe("siblings", &main);
    let (code, out) = run_exe(&exe, b"");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "siblings: expected exit 0, got {code}");
    assert_eq!(
        out,
        concat!(
            r#"[{"children":[{"fields":{},"level":"info","message":"a.start"}],"#,
            r#""fields":{"id":1},"name":"request.a"},"#,
            r#"{"children":[{"fields":{},"level":"info","message":"b.start"},"#,
            r#"{"children":[{"fields":{},"level":"info","message":"b.query"}],"#,
            r#""fields":{},"name":"db"},"#,
            r#"{"fields":{},"level":"info","message":"b.end"}],"#,
            r#""fields":{"id":2},"name":"request.b"}]"#,
            "\n"
        ),
        "siblings: collected structure mismatch"
    );
}

// ----------------------------------------------------------------
// 2. The OTLP/JSON export envelope over Text.Json
// ----------------------------------------------------------------

#[test]
fn otlp_json_envelope_exports_logs_metrics_and_spans() {
    // The handler's records exported as the OTLP/JSON envelope
    // (protobuf-JSON mapping, JSON only): resourceLogs carries the
    // log records with severity, body and attributes; resourceMetrics
    // carries the counter/gauge/histogram shapes; resourceSpans
    // carries the span with its assigned id and parent link. The
    // envelope is canonical-printable and round trips through the
    // `Text.Json` parser (the in-program check, exit 5 on drift).
    let main = format!(
        r#"module Main where
  import Effect.Log (log, metric, info, counter, gauge, histogram, no-fields, fields, num, str, arr, log-record, metric-record)
  import Effect.Trace (span, end-span, end-marker, span-scope, TraceContext)
  import Text.Json (canonical-json, parse-json)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Text.Otlp (parse-traceparent, otlp-export)
  import Data.Json
  import Data.Option
  import Data.String

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    case parse-traceparent "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01" of
      None -> 1
      Some ctx ->
{COLLECTORS}
        case handle
            (handle
              (do
                log info "boot" no-fields
                span "work" (fields [("unit", str "x")])
                metric counter "jobs" 3 no-fields
                metric gauge "queue" 7 no-fields
                metric histogram "job-ms" 12 no-fields
                end-span "work")
              with collect-logs)
            with collect-spans of
          records ->
            let env = otlp-export ctx (fields [("service.name", str "checkout")]) records in
            let text = canonical-json env in
            case parse-json "<otlp>" (string-to-bytes text) of
              Failed b -> 4
              Parsed v soft -> case ioWrite (string-to-bytes (string-concat text "\n")) of
                w -> if string-compare (canonical-json v) text == 0 then 0 else 5
"#
    );
    let (dir, exe) = build_exe("otlp", &main);
    let (code, out) = run_exe(&exe, b"");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "otlp: expected exit 0 (round trip), got {code}");
    // The shape pin: the envelope's canonical text.
    assert!(
        out.starts_with(r#"{"resourceLogs":[{"resource":{"attributes":[{"key":"service.name","value":{"stringValue":"checkout"}}]},"#),
        "otlp: resourceLogs head mismatch: {out}"
    );
    assert!(
        out.contains(r#""logRecords":[{"attributes":[],"body":{"stringValue":"boot"},"severityNumber":9,"severityText":"INFO","spanId":"00f067aa0ba902b7","traceId":"4bf92f3577b34da6a3ce929d0e0e4736"}]"#),
        "otlp: logRecords mismatch: {out}"
    );
    assert!(
        out.contains(r#""sum":{"aggregationTemporality":2,"dataPoints":[{"asInt":"3","attributes":[]}],"isMonotonic":true}"#),
        "otlp: counter sum mismatch: {out}"
    );
    assert!(
        out.contains(r#""gauge":{"dataPoints":[{"asInt":"7","attributes":[]}]}"#),
        "otlp: gauge mismatch: {out}"
    );
    assert!(
        out.contains(r#""histogram":{"aggregationTemporality":2,"dataPoints":[{"attributes":[],"bucketCounts":["1"],"count":"1","explicitBounds":[],"sum":12}]}"#),
        "otlp: histogram mismatch: {out}"
    );
    assert!(
        out.contains(r#""name":"work","parentSpanId":"00f067aa0ba902b7","spanId":"0000000000000001","traceId":"4bf92f3577b34da6a3ce929d0e0e4736"}"#),
        "otlp: span shape mismatch: {out}"
    );
    assert!(
        out.ends_with("]}\n"),
        "otlp: envelope tail mismatch: {out}"
    );
}

// ----------------------------------------------------------------
// 3. W3C traceparent: parse/format round trip and the header grammar
// ----------------------------------------------------------------

const W3C_MAIN: &str = r#"module Main where
  import Effect.Trace (TraceContext)
  import Text.Otlp (parse-traceparent, format-traceparent)
  import Data.Option
  import Data.String

  -- Parse and format round trip exactly: the header is canonical
  -- lowercase, so `format-traceparent (parse-traceparent h)` is `h`.
  round-trips :: String -> Bool
  round-trips h = case parse-traceparent h of
    None -> False
    Some c -> string-compare (format-traceparent c) h == 0

  refuses :: String -> Bool
  refuses h = case parse-traceparent h of
    None -> True
    Some _ -> False

  -- A later version is parsed through its first 55 characters and
  -- normalized; parsing the formatted header is stable.
  future-stable :: String -> String -> Bool
  future-stable h norm = case parse-traceparent h of
    None -> False
    Some c -> case parse-traceparent (format-traceparent c) of
      None -> False
      Some c2 -> string-compare (format-traceparent c2) norm == 0

  main :: Int
  main =
    if not (round-trips "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01") then 1
    else if not (round-trips "00-0af7651916cd43dd8448eb211c80319c-b7ad6b7169203331-00") then 2
    else if not (round-trips "00-11111111111111111111111111111111-2222222222222222-ff") then 3
    else if not (refuses "00-4BF92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01") then 4
    else if not (refuses "ff-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01") then 5
    else if not (refuses "00-00000000000000000000000000000000-00f067aa0ba902b7-01") then 6
    else if not (refuses "00-4bf92f3577b34da6a3ce929d0e0e4736-0000000000000000-01") then 7
    else if not (refuses "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7") then 8
    else if not (refuses "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01-00") then 9
    else if not (refuses "00+4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01") then 10
    else if not (refuses "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-0g") then 11
    else if not (refuses "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b701") then 12
    else if future-stable "01-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01-ignored"
      "01-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01" then 0
    else 13
"#;

#[test]
fn w3c_traceparent_round_trips_and_refuses_ill_formed_headers() {
    // The header grammar (version-traceid-spanid-flags): lowercase
    // hexadecimal, `ff` version forbidden, zero ids forbidden, version
    // `00` exactly four fields, later versions parsed through 55
    // characters and normalized. Parse and format round trip.
    expect_exit("w3c", W3C_MAIN, 0);
}

// ----------------------------------------------------------------
// 4. The trace context propagates across the boundary, carried by the
//    runtime identity `r`
// ----------------------------------------------------------------

#[test]
fn trace_context_propagates_across_run_boundaries_in_one_trace() {
    // The context is parsed at the W3C ingress boundary and carried by
    // the runtime identity `r` (`TraceContext r` beside `Io r` in
    // `emit`'s signature): two separately handled runs (the boundary
    // between ingress and commit) export into ONE trace. The pinned
    // envelope shows the parsed trace id on every signal and both root
    // spans parented at the parsed span id.
    let main = format!(
        r#"module Main where
  import Effect.Log (log, info, no-fields, fields, num, str, arr, log-record, metric-record)
  import Effect.Trace (span, end-span, end-marker, span-scope, TraceContext)
  import Text.Json (canonical-json)
  import Text.Otlp (parse-traceparent, otlp-export)
  import Algebra.Semigroup
  import Data.Json
  import Data.Option
  import Data.String

  infixr 6 <>

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    case parse-traceparent "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01" of
      None -> 1
      Some ctx ->
{COLLECTORS}
        case handle
            (handle
              (do
                log info "ingress.received" no-fields
                span "upstream" no-fields
                log info "validated" no-fields
                end-span "upstream")
              with collect-logs)
            with collect-spans of
          up ->
            case handle
                (handle
                  (do
                    span "downstream" no-fields
                    log info "committed" no-fields
                    end-span "downstream")
                  with collect-logs)
                with collect-spans of
              down -> emit ctx up down

  -- The trace context travels with the runtime identity: `ctx` is
  -- indexed by the same `r` as the emitting `Io r` capability.
  emit :: forall r. using (io :: Io r) => TraceContext r -> List Json -> List Json ->{{IO r}} Int
  emit ctx up down =
    let env = otlp-export ctx (fields [("service.name", str "checkout")]) (up <> down) in
    case ioWrite (string-to-bytes (string-concat (canonical-json env) "\n")) of
      w -> 0
"#
    );
    let (dir, exe) = build_exe("propagated", &main);
    let (code, out) = run_exe(&exe, b"");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "propagated: expected exit 0, got {code}");
    // One trace id on every signal (three logs, two spans); both root
    // spans parented at the ingressed span id (the context crossing
    // the run boundary).
    assert_eq!(
        out.matches(r#""traceId":"4bf92f3577b34da6a3ce929d0e0e4736""#).count(),
        5,
        "propagated: expected the trace id on all five signals: {out}"
    );
    assert!(
        out.contains(r#""name":"upstream","parentSpanId":"00f067aa0ba902b7","spanId":"0000000000000001""#),
        "propagated: upstream root not parented at the ingressed span: {out}"
    );
    assert!(
        out.contains(r#""name":"downstream","parentSpanId":"00f067aa0ba902b7","spanId":"0000000000000002""#),
        "propagated: downstream root not parented at the ingressed span: {out}"
    );
}

// ----------------------------------------------------------------
// 5. The DX: log/metric/span as ordinary effect operations
// ----------------------------------------------------------------

#[test]
fn dx_program_reads_naturally_with_levels_and_fields() {
    // The DX sample (the row 3 acceptance bar), verbatim. Levels are
    // bare values, fields are one pair list, spans open and close as
    // lexical scopes (and may run to the tail with no close). This is
    // the whole program; the collecting handlers above it are one-line
    // clauses over the record builders.
    let main = format!(
        r#"module Main where
  import Effect.Log (log, metric, info, debug, warn, counter, gauge, histogram, no-fields, fields, num, str, arr, log-record, metric-record)
  import Effect.Trace (span, end-span, end-marker, span-scope)
  import Text.Json (canonical-json)
  import Data.Json
  import Data.String

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
{COLLECTORS}
    case handle
        (handle
          (do
            log info "server.started" (fields [("port", num 8080)])
            span "http.request" (fields [("method", str "GET")])
            log debug "auth.cache-hit" (fields [("user", str "u-42")])
            span "db.query" (fields [("table", str "users")])
            metric counter "db.rows" 42 no-fields
            metric gauge "db.pool" 7 no-fields
            metric histogram "db.latency-ms" 12 no-fields
            end-span "db.query"
            log warn "slow.client" (fields [("wait-ms", num 240)]))
          with collect-logs)
        with collect-spans of
      records -> case ioWrite (string-to-bytes (string-concat (canonical-json (arr records)) "\n")) of
        w -> 0
"#
    );
    let (dir, exe) = build_exe("dx", &main);
    let (code, out) = run_exe(&exe, b"");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "dx: expected exit 0, got {code}");
    assert_eq!(
        out,
        concat!(
            r#"[{"fields":{"port":8080},"level":"info","message":"server.started"},"#,
            r#"{"children":[{"fields":{"user":"u-42"},"level":"debug","message":"auth.cache-hit"},"#,
            r#"{"children":[{"fields":{},"metric":"counter","name":"db.rows","value":42},"#,
            r#"{"fields":{},"metric":"gauge","name":"db.pool","value":7},"#,
            r#"{"fields":{},"metric":"histogram","name":"db.latency-ms","value":12}],"#,
            r#""fields":{"table":"users"},"name":"db.query"},"#,
            r#"{"fields":{"wait-ms":240},"level":"warn","message":"slow.client"}],"#,
            r#""fields":{"method":"GET"},"name":"http.request"}]"#,
            "\n"
        ),
        "dx: collected structure mismatch"
    );
}

// ----------------------------------------------------------------
// 6. Handler interpretation: unhandled, the operations are just their
//    effect rows (refused at the boundary)
// ----------------------------------------------------------------

#[test]
fn unhandled_operations_are_refused_at_the_boundary() {
    // Without a handler the `Log r`/`Trace r` rows surface honestly:
    // the pure entry refuses them and a signature that omits the row
    // does not match.
    expect_err(
        "unhandled-main",
        r#"module Main where
  import Effect.Log (log, info, no-fields)
  import Data.Json

  main :: Int
  main = case log info "x" no-fields of
    u -> 0
"#,
        "must be pure",
    );
    expect_err(
        "unhidden-effect",
        r#"module Main where
  import Effect.Trace (span)
  import Data.Json (Json, JObjEnd)

  poke :: Unit -> Unit
  poke u = case span "x" JObjEnd of
    w -> ()

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
    expect_err(
        "unhandled-at-entry",
        r#"module Main where
  import Effect.Log (metric, counter, no-fields)
  import Data.Json

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in case metric counter "x" 1 no-fields of
    w -> 0
"#,
        "effect rows do not match",
    );
    // With the handlers the very same operations run: the collecting
    // tests above are that half of the story.
}
