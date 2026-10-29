//! `Effect.Http` consumer semantics (P37-P40, the last item of the Ruby
//! networking sweep in design/stdlib-parity-ruby.md: Net::HTTP).
//!
//! Each test compiles a real multi-module project (the module under test,
//! `Effect/Http.cinch`, beside its dependencies `Effect/Net.cinch`,
//! `Text/Uri.cinch` and `Data/IpAddr.cinch`, plus a `Main`) in a fresh
//! temp directory and builds and runs it with the same `driver` the CLI
//! uses; the program's exit code and captured stdout are the assertion
//! surface, never source text.
//!
//! The origin is a loopback `TcpListener` this file stands up; a thread
//! writes a canned raw HTTP response (one write per piece, a flush
//! between pieces) and closes its side, so the client reaches the end of
//! the stream. The cases cover every framing the module names: a
//! `Content-Length` 200 whose body the program prints back, a 404 returned
//! as a RESPONSE (not an error), a chunked body split across several
//! writes, a response with no framing read to end of stream, a header
//! appearing twice, a header matched case-insensitively, a malformed
//! status line, a body past the bound, and `https` refused by name.

use cinch::driver::{driver, Mode};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The modules under test and their dependencies, written beside the entry.
const NET: &str = include_str!("../prelude/Effect/Net.cinch");
const URI: &str = include_str!("../prelude/Text/Uri.cinch");
const IPADDR: &str = include_str!("../prelude/Data/IpAddr.cinch");
const HTTP: &str = include_str!("../prelude/Effect/Http.cinch");

/// A fresh project directory: the four modules plus the `Main`.
fn write_project(name: &str, main: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "cinch_http_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Net.cinch", NET),
        ("Uri.cinch", URI),
        ("IpAddr.cinch", IPADDR),
        ("Http.cinch", HTTP),
        ("Main.cinch", main),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    dir
}

/// A driver invocation on a thread with a large stack (the checker
/// recurses over elaborated code).
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

/// Compile the `Main` to an executable, run it, and capture its exit code
/// and stdout.
fn build_and_run(name: &str, main: &str) -> (i32, String) {
    let dir = write_project(name, main);
    let exe = dir.join("program");
    let code = drive(Mode::Build, &dir.join("Main.cinch"), Some(&exe))
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"));
    assert_eq!(code, 0, "{name}: build failed");
    let out = std::process::Command::new(&exe)
        .stdout(std::process::Stdio::piped())
        .output()
        .unwrap_or_else(|e| panic!("run {}: {e}", exe.display()));
    let _ = std::fs::remove_dir_all(&dir);
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// A loopback listener whose peer thread writes each piece of a canned raw
/// response (a flush between pieces, so the wire splits where the pieces
/// do) and then closes its side. The port answers.
fn canned_peer(pieces: Vec<Vec<u8>>) -> u16 {
    use std::io::{Read, Write};
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().expect("local addr").port();
    std::thread::spawn(move || {
        if let Ok((mut s, _)) = listener.accept() {
            // Drain the client's request first: closing a socket with
            // unread received data makes the OS send RST, which the
            // client would see as a socket error instead of end of
            // stream. A short timeout keeps the drain from hanging.
            let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(300)));
            let mut buf = [0u8; 4096];
            let _ = s.read(&mut buf);
            for (i, piece) in pieces.iter().enumerate() {
                let _ = s.write_all(piece);
                let _ = s.flush();
                if i + 1 < pieces.len() {
                    // Shape the wire so the reader takes more than one
                    // `net-read`; the client accumulates either way.
                    std::thread::sleep(std::time::Duration::from_millis(15));
                }
            }
            // A clean FIN for the client's end of stream, then drain the
            // rest of the request so the final close is orderly.
            let _ = s.shutdown(std::net::Shutdown::Write);
            loop {
                match s.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
            }
        }
    });
    port
}

fn one(resp: &[u8]) -> Vec<Vec<u8>> {
    vec![resp.to_vec()]
}

fn contains(hay: &str, needle: &str) -> bool {
    hay.contains(needle)
}

#[test]
fn http_client_framing_headers_and_refusals() {
    // The 200 with Content-Length: the body is printed back verbatim.
    let p200 = canned_peer(one(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello"));
    // The 404: a response, never an error.
    let p404 = canned_peer(one(b"HTTP/1.1 404 Not Found\r\nContent-Length: 3\r\n\r\nno!"));
    // Chunked, split across three writes.
    let pchunk = canned_peer(vec![
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nhe".to_vec(),
        b"llo\r\n3\r\nabc\r\n0\r\n".to_vec(),
        b"\r\n".to_vec(),
    ]);
    // No framing: read to end of stream.
    let peof = canned_peer(one(b"HTTP/1.1 200 OK\r\n\r\nrawbody"));
    // A header appearing twice.
    let pdup = canned_peer(one(
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nX-Dup: a\r\nX-Dup: b\r\n\r\n",
    ));
    // A header whose name the caller asks for in the other case.
    let pci = canned_peer(one(
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n",
    ));
    // A malformed status line.
    let pbad = canned_peer(one(b"BOGUS LINE\r\n\r\n"));
    // A body past the (small) bound.
    let mut big = b"HTTP/1.1 200 OK\r\nContent-Length: 200\r\n\r\n".to_vec();
    big.extend(std::iter::repeat(b'x').take(200));
    let pbig = canned_peer(one(&big));
    // An origin for the `http-get` convenience.
    let pget = canned_peer(one(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nping"));

    let src = r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http

  -- Print `s` and a newline (the observation the Rust side asserts on).
  emit :: forall r. using (io :: Io r) => String ->{IO r} Unit
  emit s = case ioWrite (string-to-bytes s) of
    _ -> case ioWrite (string-to-bytes "\n") of
      _ -> ()

  body-text :: HttpResponse -> Option String
  body-text r = string-from-bytes (http-body r)

  -- A 200 whose body the program prints back.
  check-ok :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-ok u = case http-request (MkHttpRequest "GET" "127.0.0.1" __P200__ "/" Nil None) of
    Err e -> case emit "200:err" of
      _ -> 1
    Ok r -> case http-status r of
      200 -> case body-text r of
        Some t -> case emit (string-concat "200:" t) of
          _ -> case t == "hello" of
            True -> 0
            False -> 2
        None -> 3
      _ -> 4

  -- A 404 is a RESPONSE, not an error: the status is data.
  check-notfound :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-notfound u = case http-request (MkHttpRequest "GET" "127.0.0.1" __P404__ "/missing" Nil None) of
    Err e -> case emit "404:err" of
      _ -> 1
    Ok r -> case http-status r of
      404 -> case body-text r of
        Some t -> case emit (string-concat "404:" t) of
          _ -> case t == "no!" of
            True -> 0
            False -> 2
        None -> 3
      _ -> 4

  -- A chunked body reassembled across several reads.
  check-chunked :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-chunked u = case http-request (MkHttpRequest "GET" "127.0.0.1" __PCHUNK__ "/" Nil None) of
    Err e -> case emit "chunked:err" of
      _ -> 1
    Ok r -> case http-status r of
      200 -> case body-text r of
        Some t -> case emit (string-concat "chunked:" t) of
          _ -> case t == "helloabc" of
            True -> 0
            False -> 2
        None -> 3
      _ -> 4

  -- No framing: read to end of stream.
  check-eof :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-eof u = case http-request (MkHttpRequest "GET" "127.0.0.1" __PEOF__ "/" Nil None) of
    Err e -> case emit "eof:err" of
      _ -> 1
    Ok r -> case body-text r of
      Some t -> case emit (string-concat "eof:" t) of
        _ -> case t == "rawbody" of
          True -> 0
          False -> 2
      None -> 3

  -- A header appearing twice: every value is kept, in arrival order.
  check-dup :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-dup u = case http-request (MkHttpRequest "GET" "127.0.0.1" __PDUP__ "/" Nil None) of
    Err e -> case emit "dup:err" of
      _ -> 1
    Ok r -> case http-header-all r "x-dup" of
      Cons a (Cons b Nil) -> case emit (string-concat "dup:" (string-concat a (string-concat "," b))) of
        _ -> case a == "a" of
          True -> case b == "b" of
            True -> 0
            False -> 2
          False -> 3
      _ -> 4

  -- A header name matched case-insensitively (the reply's `Content-Type`
  -- found through `content-type`).
  check-ci :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-ci u = case http-request (MkHttpRequest "GET" "127.0.0.1" __PCI__ "/" Nil None) of
    Err e -> case emit "ci:err" of
      _ -> 1
    Ok r -> case http-header r "content-type" of
      Some v -> case emit (string-concat "ci:" v) of
        _ -> case v == "text/plain" of
          True -> 0
          False -> 2
      None -> 3

  -- A malformed status line is HttpBadStatusLine.
  check-bad :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-bad u = case http-request (MkHttpRequest "GET" "127.0.0.1" __PBAD__ "/" Nil None) of
    Err e -> case e of
      HttpBadStatusLine -> case emit "bad-status" of
        _ -> 0
      _ -> case emit "bad-status:wrong" of
        _ -> 1
    Ok r -> case emit "bad-status:ok" of
      _ -> 2

  -- A body past the explicit bound is HttpBodyTooLarge.
  check-big :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-big u = case http-request-max 32 (MkHttpRequest "GET" "127.0.0.1" __PBIG__ "/" Nil None) of
    Err e -> case e of
      HttpBodyTooLarge -> case emit "too-large" of
        _ -> 0
      _ -> case emit "too-large:wrong" of
        _ -> 1
    Ok r -> case emit "too-large:ok" of
      _ -> 2

  -- https is refused by name (no TLS, so no plaintext downgrade).
  check-tls :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-tls u = case http-get "https://example.com/" of
    Err e -> case e of
      HttpNoTls -> case emit "no-tls" of
        _ -> 0
      _ -> case emit "no-tls:wrong" of
        _ -> 1
    Ok r -> case emit "no-tls:ok" of
      _ -> 2

  -- The http-get convenience over a real origin (URL parsed by Text.Uri).
  check-get :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-get u = case http-get __URL__ of
    Err e -> case emit "get:err" of
      _ -> 1
    Ok r -> case http-status r of
      200 -> case body-text r of
        Some t -> case emit (string-concat "get:" t) of
          _ -> case t == "ping" of
            True -> 0
            False -> 2
        None -> 3
      _ -> 4

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = check-ok () in
    let c2 = check-notfound () in
    let c3 = check-chunked () in
    let c4 = check-eof () in
    let c5 = check-dup () in
    let c6 = check-ci () in
    let c7 = check-bad () in
    let c8 = check-big () in
    let c9 = check-tls () in
    let c10 = check-get () in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8 + c9 + c10
"#
    .replace("__P200__", &p200.to_string())
    .replace("__P404__", &p404.to_string())
    .replace("__PCHUNK__", &pchunk.to_string())
    .replace("__PEOF__", &peof.to_string())
    .replace("__PDUP__", &pdup.to_string())
    .replace("__PCI__", &pci.to_string())
    .replace("__PBAD__", &pbad.to_string())
    .replace("__PBIG__", &pbig.to_string())
    .replace("__URL__", &format!("\"http://127.0.0.1:{pget}/\""));

    let (code, out) = build_and_run("http-cases", &src);
    assert_eq!(code, 0, "http-cases: check bitmask {code}; stdout:\n{out}");
    for token in [
        "200:hello",
        "404:no!",
        "chunked:helloabc",
        "eof:rawbody",
        "dup:a,b",
        "ci:text/plain",
        "bad-status",
        "too-large",
        "no-tls",
        "get:ping",
    ] {
        assert!(
            contains(&out, token),
            "http-cases: expected {token:?} in stdout:\n{out}"
        );
    }
}
