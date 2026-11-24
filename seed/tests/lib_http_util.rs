//! Integration and unit verification for:
//! - `Effect.Http` TLS 1.3 HTTPS client (`http-request-tls`, `http-request-tls-max`, `http-get-tls`)
//! - `Effect.Http.Server` TCP listener serving (`http-serve-listener-n`, `http-listen-and-serve-with`),
//!   TLS 1.3 HTTPS server (`http-serve-tls`, `http-serve-tls-with`, `http-serve-tls-listener-n`),
//!   and TLS-aware `HttpClient` (`http-client-with-tls`, `http-client-do-tls`, `http-client-get-tls`,
//!   `http-client-post-tls`)
//! - `Effect.Http.Util` (`dump-request`, `dump-response`, `chunked-encode`, `chunked-decode`,
//!   `strip-hop-by-hop-headers`, and HTTP/HTTPS `ReverseProxy`)
//! - `Effect.Smtp` TLS 1.3 implicit SMTPS (`smtp-dial-tls`, `smtp-send-mail-tls`) and RFC 3207
//!   `STARTTLS` (`smtp-starttls`, `smtp-send-mail-starttls`, `smtp-tls-*`)
mod common;

use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpListener;

/// All source-distributed prelude modules transitively required by `Effect.Http`,
/// `Effect.Http.Server`, `Effect.Http.Util`, `Effect.Smtp`, and `Effect.Tls`.
const DEPS: &[(&str, &str)] = &[
    ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
    ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
    ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
    ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
    ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
    ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
    ("Data.CookieJar.cinch", include_str!("../prelude/Data/CookieJar.cinch")),
    ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
    ("Effect.Net.cinch", include_str!("../prelude/Effect/Net.cinch")),
    ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
    ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
    ("Text.Base64.cinch", include_str!("../prelude/Text/Base64.cinch")),
    ("Text.Base64.Encoding.cinch", include_str!("../prelude/Text/Base64/Encoding.cinch")),
    ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
    ("Text.Cookie.cinch", include_str!("../prelude/Text/Cookie.cinch")),
    ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
    ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
    ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
    ("Text.Pem.cinch", include_str!("../prelude/Text/Pem.cinch")),
    ("Text.Punycode.cinch", include_str!("../prelude/Text/Punycode.cinch")),
    ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
    ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
    ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
    ("Text.TextProto.cinch", include_str!("../prelude/Text/TextProto.cinch")),
    ("Text.Uri.cinch", include_str!("../prelude/Text/Uri.cinch")),
    ("Text.Url.cinch", include_str!("../prelude/Text/Url.cinch")),
    ("Text.Url.Codec.cinch", include_str!("../prelude/Text/Url/Codec.cinch")),
    ("Text.Url.Error.cinch", include_str!("../prelude/Text/Url/Error.cinch")),
    ("Text.Url.Ip.cinch", include_str!("../prelude/Text/Url/Ip.cinch")),
    ("Text.Url.Model.cinch", include_str!("../prelude/Text/Url/Model.cinch")),
    ("Text.Url.Parser.cinch", include_str!("../prelude/Text/Url/Parser.cinch")),
    ("Text.Url.Query.cinch", include_str!("../prelude/Text/Url/Query.cinch")),
    ("Text.Url.Resolve.cinch", include_str!("../prelude/Text/Url/Resolve.cinch")),
    ("Text.Url.Scan.cinch", include_str!("../prelude/Text/Url/Scan.cinch")),
    ("Text.Url.Userinfo.cinch", include_str!("../prelude/Text/Url/Userinfo.cinch")),
    ("Text.Http.cinch", include_str!("../prelude/Text/Http.cinch")),
    ("Text.Smtp.cinch", include_str!("../prelude/Text/Smtp.cinch")),
    ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
    ("Crypto.Hmac.cinch", include_str!("../prelude/Crypto/Hmac.cinch")),
    ("Crypto.Kdf.cinch", include_str!("../prelude/Crypto/Kdf.cinch")),
    ("Crypto.Cipher.cinch", include_str!("../prelude/Crypto/Cipher.cinch")),
    ("Crypto.Aes.cinch", include_str!("../prelude/Crypto/Aes.cinch")),
    ("Crypto.ChaCha20Poly1305.cinch", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch")),
    ("Crypto.P256.cinch", include_str!("../prelude/Crypto/P256.cinch")),
    ("Crypto.Ecdsa.cinch", include_str!("../prelude/Crypto/Ecdsa.cinch")),
    ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
    ("Crypto.Ed25519.cinch", include_str!("../prelude/Crypto/Ed25519.cinch")),
    ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
    ("Crypto.Rsa.cinch", include_str!("../prelude/Crypto/Rsa.cinch")),
    ("Crypto.Pkix.cinch", include_str!("../prelude/Crypto/Pkix.cinch")),
    ("Crypto.X509.cinch", include_str!("../prelude/Crypto/X509.cinch")),
    ("Crypto.Tls.cinch", include_str!("../prelude/Crypto/Tls.cinch")),
    ("Effect.Tls.cinch", include_str!("../prelude/Effect/Tls.cinch")),
    ("Effect.Http.cinch", include_str!("../prelude/Effect/Http.cinch")),
    ("Effect.Http.Server.cinch", include_str!("../prelude/Effect/Http/Server.cinch")),
    ("Effect.Http.Util.cinch", include_str!("../prelude/Effect/Http/Util.cinch")),
    ("Effect.Smtp.cinch", include_str!("../prelude/Effect/Smtp.cinch")),
];

fn run_cinch(tag: &str, main_src: &str) -> i32 {
    let mut files: Vec<(&str, &str)> = DEPS.to_vec();
    files.push(("Main.cinch", main_src));
    let dir = common::project(tag, &files);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    code
}

fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn ecdsa_fixture_pems() -> (String, String, String) {
    let f: Value = serde_json::from_str(include_str!("fixtures/tls-vectors.json")).unwrap();
    let ec = &f["certs"][1];
    (
        ec["root_pem"].as_str().unwrap().to_string(),
        ec["chain_pem"].as_str().unwrap().to_string(),
        ec["key_pem"].as_str().unwrap().to_string(),
    )
}

/// Canonical documentation checks on `Effect.Http.Util`, `Effect.Http.Server`, and `Effect.Smtp`.
#[test]
fn http_util_and_tls_modules_have_canonical_documentation() {
    common::assert_documented(
        "Effect.Http.Util",
        include_str!("../prelude/Effect/Http/Util.cinch"),
    );
    common::assert_documented(
        "Effect.Http.Server",
        include_str!("../prelude/Effect/Http/Server.cinch"),
    );
    common::assert_documented(
        "Effect.Smtp",
        include_str!("../prelude/Effect/Smtp.cinch"),
    );
}

/// Pure `Effect.Http.Util` verification: `dump-request`, `dump-response`, `chunked-encode`,
/// `chunked-decode`, and `strip-hop-by-hop-headers`.
#[test]
fn http_util_pure_dump_chunked_and_hop_by_hop() {
    let main = r#"-- | Pure Effect.Http.Util wire helper checks.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http.Util
  import Text.Http

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Check whether `hay` contains substring `needle`.
  contains-sub :: String -> String -> Bool
  contains-sub hay needle =
    let hb = string-to-bytes hay in
    let nb = string-to-bytes needle in
    let nn = bytes-len nb in
    if nn == 0 then True
    else if nn > bytes-len hb then False
    else scan-sub hb nb 0 nn

  -- | Scan `hb` for `nb` starting at offset `i`.
  scan-sub :: Bytes -> Bytes -> Nat -> Nat -> Bool
  scan-sub hb nb i nn =
    if i + nn > bytes-len hb then False
    else
      let out = bytes-new nn in
      let u = bytes-copy-range out hb 0 i nn in
      if eq-i64 (bytes-compare out nb) 0 then True
      else scan-sub hb nb (i + 1) nn

  -- | Run all pure wire helper assertions.
  main :: Int
  main =
    let req0 = req-with-header (req-with-host (http-new-request method-post "/submit?x=1" (string-to-bytes "hello")) "example.com") "X-Test" "yes" in
    let dr-with = text (dump-request req0 True) in
    let dr-no = text (dump-request req0 False) in
    let ok-dr = (contains-sub dr-with "POST /submit?x=1 HTTP/1.1\r\n"
      && contains-sub dr-with "Host: example.com\r\n"
      && contains-sub dr-with "Content-Length: 5\r\n"
      && contains-sub dr-with "\r\n\r\nhello"
      && contains-sub dr-no "POST /submit?x=1 HTTP/1.1\r\n"
      && not (contains-sub dr-no "hello")) in
    let resp0 = resp-with-header (http-response-text status-created "made-it") "X-Id" "99" in
    let dp-with = text (dump-response resp0 True) in
    let dp-no = text (dump-response resp0 False) in
    let ok-dp = (contains-sub dp-with "HTTP/1.1 201 Created\r\n"
      && contains-sub dp-with "X-Id: 99\r\n"
      && contains-sub dp-with "\r\n\r\nmade-it"
      && contains-sub dp-no "HTTP/1.1 201 Created\r\n"
      && not (contains-sub dp-no "made-it")) in
    let payload = string-to-bytes "Hello, chunked HTTP/1.1 world!" in
    let enc = chunked-encode payload 7 in
    let ok-c1 = (case chunked-decode enc of
      Ok got -> eq-i64 (bytes-compare got payload) 0
      Err _ -> False) in
    let ok-c2 = (case chunked-decode (chunked-encode (bytes-new 0) 4) of
      Ok got -> bytes-len got == 0
      Err _ -> False) in
    let ok-c3 = (case chunked-decode (string-to-bytes "5;ext=1\r\nhello\r\n0\r\nX-Trail: ok\r\n\r\n") of
      Ok got -> text got == "hello"
      Err _ -> False) in
    let ok-cbad1 = (case chunked-decode (string-to-bytes "ZZ\r\nhello\r\n0\r\n\r\n") of
      Err (HttpProtoBadChunk _) -> True
      _ -> False) in
    let ok-cbad2 = (case chunked-decode (string-to-bytes "5\nhello\r\n0\r\n\r\n") of
      Err (HttpProtoBadChunk _) -> True
      _ -> False) in
    let ok-cbad3 = (case chunked-decode (string-to-bytes "5\r\nhelloXX0\r\n\r\n") of
      Err (HttpProtoBadChunk _) -> True
      _ -> False) in
    let ok-cbad4 = (case chunked-decode (string-to-bytes "5\r\nhel") of
      Err HttpProtoIncomplete -> True
      _ -> False) in
    let ok-cbad5 = (case chunked-decode (string-to-bytes "0\r\n\r\nextra") of
      Err (HttpProtoBadChunk _) -> True
      _ -> False) in
    let hdrs = [
      ("Connection", "keep-alive, X-Custom-Hop"),
      ("connection", "Foo-Hop"),
      ("Keep-Alive", "timeout=5"),
      ("Proxy-Authenticate", "Basic"),
      ("Proxy-Authorization", "secret"),
      ("Proxy-Connection", "keep-alive"),
      ("TE", "trailers"),
      ("Trailers", "X-T"),
      ("Transfer-Encoding", "chunked"),
      ("Upgrade", "h2c"),
      ("x-custom-hop", "drop-me"),
      ("FOO-HOP", "drop-me-too"),
      ("X-Keep", "stay"),
      ("Content-Type", "application/json")
    ] in
    let stripped = strip-hop-by-hop-headers hdrs in
    let ok-hop = (case stripped of
      Cons (k1, v1) (Cons (k2, v2) Nil) -> k1 == "X-Keep" && v1 == "stay" && k2 == "Content-Type" && v2 == "application/json"
      _ -> False) in
    if not ok-dr then 1
    else if not ok-dp then 2
    else if not (ok-c1 && ok-c2 && ok-c3) then 3
    else if not (ok-cbad1 && ok-cbad2 && ok-cbad3 && ok-cbad4 && ok-cbad5) then 4
    else if not ok-hop then 5
    else 200
"#;
    let code = run_cinch("http_util_pure", main);
    assert_eq!(code, 200, "http_util_pure failed with code {code}");
}

/// Live loopback TCP tests for standalone Cinch `http-serve-listener-n` / `http-listen-and-serve-with`,
/// `http-get "https://..."` returning `Err HttpNoTls`, and full 3-hop `ReverseProxy` (Cinch client ->
/// Cinch `http-serve-listener-n` with `reverse-proxy-handler` -> Rust loopback backend) plus 502 error handling.
#[test]
fn http_listener_and_reverse_proxy_live_loopback() {
    let backend = TcpListener::bind("127.0.0.1:0").expect("bind backend");
    let back_port = backend.local_addr().unwrap().port();

    std::thread::spawn(move || {
        if let Ok((mut s, _)) = backend.accept() {
            let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(10)));
            let mut raw = Vec::new();
            let mut buf = [0u8; 4096];
            let mut head_end = None;
            let mut content_len = 0usize;
            loop {
                match s.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        raw.extend_from_slice(&buf[..n]);
                        if head_end.is_none() {
                            if let Some(pos) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                                head_end = Some(pos + 4);
                                let head_str = String::from_utf8_lossy(&raw[..pos]);
                                for line in head_str.lines().skip(1) {
                                    if let Some((k, v)) = line.split_once(':') {
                                        if k.eq_ignore_ascii_case("content-length") {
                                            content_len = v.trim().parse().unwrap_or(0);
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(he) = head_end {
                            if raw.len() >= he + content_len {
                                break;
                            }
                        }
                    }
                }
            }
            let req_str = String::from_utf8_lossy(&raw);
            let mut lines = req_str.split("\r\n");
            let req_line = lines.next().unwrap_or("");
            let req_path = req_line.split_whitespace().nth(1).unwrap_or("");
            let mut host = String::new();
            let mut xff = String::new();
            let mut xfh = String::new();
            let mut xfp = String::new();
            let mut xrw = String::new();
            let mut hop_leaked = false;
            for line in lines {
                if line.is_empty() {
                    break;
                }
                if let Some((k, v)) = line.split_once(':') {
                    let val = v.trim();
                    if k.eq_ignore_ascii_case("host") {
                        host = val.to_string();
                    } else if k.eq_ignore_ascii_case("x-forwarded-for") {
                        xff = val.to_string();
                    } else if k.eq_ignore_ascii_case("x-forwarded-host") {
                        xfh = val.to_string();
                    } else if k.eq_ignore_ascii_case("x-forwarded-proto") {
                        xfp = val.to_string();
                    } else if k.eq_ignore_ascii_case("x-rewritten") {
                        xrw = val.to_string();
                    } else if k.eq_ignore_ascii_case("x-drop-req")
                        || k.eq_ignore_ascii_case("keep-alive")
                    {
                        hop_leaked = true;
                    }
                }
            }
            let body_slice = head_end.map(|he| &raw[he..]).unwrap_or(&[]);
            let body_text = String::from_utf8_lossy(body_slice);
            let resp_body = format!(
                "{req_path}|host={host}|xff={xff}|xfh={xfh}|xfp={xfp}|xrw={xrw}|hop={}|body={body_text}",
                if hop_leaked { "leaked" } else { "gone" }
            );
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nConnection: X-Drop-Resp, close\r\nX-Drop-Resp: secret-hop\r\nKeep-Alive: timeout=9\r\nX-Backend: alive\r\nContent-Length: {}\r\n\r\n{resp_body}",
                resp_body.len()
            );
            let _ = s.write_all(resp.as_bytes());
            let _ = s.flush();
            let _ = s.shutdown(std::net::Shutdown::Both);
        }
    });

    let main = format!(
        r#"-- | Live TCP listener and ReverseProxy verification.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Concurrent
  import Effect.Http
  import Effect.Http.Server
  import Effect.Http.Util
  import Effect.Net
  import Text.Http
  import Text.Show (nat-to-string)

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Check whether `hay` contains `needle`.
  contains-sub :: String -> String -> Bool
  contains-sub hay needle =
    let hb = string-to-bytes hay in
    let nb = string-to-bytes needle in
    let nn = bytes-len nb in
    if nn == 0 then True
    else if nn > bytes-len hb then False
    else scan-sub hb nb 0 nn

  -- | Scan `hb` for `nb` from `i`.
  scan-sub :: Bytes -> Bytes -> Nat -> Nat -> Bool
  scan-sub hb nb i nn =
    if i + nn > bytes-len hb then False
    else
      let out = bytes-new nn in
      let u = bytes-copy-range out hb 0 i nn in
      if eq-i64 (bytes-compare out nb) 0 then True
      else scan-sub hb nb (i + 1) nn

  -- | Parse the port number from "<host>:<port>".
  parse-addr-port :: String -> Nat
  parse-addr-port s =
    let b = string-to-bytes s in
    let n = bytes-len b in
    parse-port-after-colon b n n

  -- | Locate last colon and parse trailing decimal digits.
  parse-port-after-colon :: Bytes -> Nat -> Nat -> Nat
  parse-port-after-colon b idx n =
    if idx == 0 then 0
    else
      let prev = idx - 1 in
      if prev < bytes-len b && bytes-get b prev == 58 then parse-dec-from b idx n 0
      else parse-port-after-colon b prev n

  -- | Parse decimal digits in [i, n).
  parse-dec-from :: Bytes -> Nat -> Nat -> Nat -> Nat
  parse-dec-from b i n acc =
    if i >= n || i >= bytes-len b then acc
    else
      let c = bytes-get b i in
      if c >= 48 && c <= 57 then parse-dec-from b (i + 1) n (acc * 10 + (if 48 <= c then c - 48 else 0))
      else acc

  -- | Serve 2 requests over `http-serve-listener-n` and fetch them with `http-get`.
  run-listener-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-listener-scope listener port = withScope (\sc -> do
    srv <- spawn sc (\() ->
      let h = http-handler-pure (\req ->
        let p = req-path req in
        let q = case req-query-param req "n" of Some v -> v; None -> "" in
        http-response-text status-ok (string-concat-all ["tcp:", p, ":", q])) in
      let sres = http-serve-listener-n http-server-config-default listener 2 h in
      let u1 = net-listener-close listener in
      case sres of
        Ok 2 -> Ok 1
        _ -> Err 13)
    cli <- spawn sc (\() ->
      let u1 = string-concat-all ["http://127.0.0.1:", nat-to-string port, "/alpha?n=1"] in
      let u2 = string-concat-all ["http://127.0.0.1:", nat-to-string port, "/beta?n=2"] in
      case http-get u1 of
        Err _ -> Err 14
        Ok r1 ->
          if not (http-status r1 == 200 && text (http-body r1) == "tcp:/alpha:1") then Err 15
          else case http-get u2 of
            Err _ -> Err 16
            Ok r2 ->
              if http-status r2 == 200 && text (http-body r2) == "tcp:/beta:2" then Ok 1
              else Err 17)
    joinAllOrCancel sc srv cli)

  -- | Run a real Cinch ReverseProxy TCP listener forwarding to the Rust loopback backend.
  run-proxy-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-proxy-scope proxy-ln proxy-port = withScope (\sc -> do
    t-proxy <- spawn sc (\() ->
      let p0 = reverse-proxy-single-host "http" "127.0.0.1" {back_port} "/v1" in
      let p1 = reverse-proxy-with-client-ip p0 "203.0.113.9" in
      let p2 = reverse-proxy-with-rewrite p1 (\q -> req-with-header q "X-Rewritten" "true") in
      let p3 = reverse-proxy-with-modify-response p2 (\resp -> resp-with-header resp "X-Proxied" "cinch") in
      let h = reverse-proxy-handler p3 in
      let sres = http-serve-listener-n http-server-config-default proxy-ln 1 h in
      let u1 = net-listener-close proxy-ln in
      case sres of
        Ok 1 -> Ok 1
        _ -> Err 22)
    t-cli <- spawn sc (\() ->
      let req = MkHttpRequest "POST" "127.0.0.1" proxy-port "/items/7?q=ok" [
        ("Host", "public.example"),
        ("X-Forwarded-For", "198.51.100.1"),
        ("Connection", "X-Drop-Req"),
        ("X-Drop-Req", "must-strip"),
        ("Keep-Alive", "timeout=5")
      ] (Some (string-to-bytes "ping-body")) in
      case http-request req of
        Err _ -> Err 23
        Ok r ->
          let bt = text (http-body r) in
          let ok-body = (contains-sub bt "/v1/items/7?q=ok"
            && contains-sub bt "|host=127.0.0.1:{back_port}"
            && contains-sub bt "|xff=198.51.100.1, 203.0.113.9"
            && contains-sub bt "|xfh=public.example"
            && contains-sub bt "|xfp=http"
            && contains-sub bt "|xrw=true"
            && contains-sub bt "|hop=gone"
            && contains-sub bt "|body=ping-body") in
          let ok-hdrs = (
            (case http-header r "X-Proxied" of Some v -> v == "cinch"; _ -> False)
            && (case http-header r "X-Backend" of Some v -> v == "alive"; _ -> False)
            && (case http-header r "X-Drop-Resp" of None -> True; _ -> False)
            && (case http-header r "Keep-Alive" of None -> True; _ -> False)) in
          if http-status r == 200 && ok-body && ok-hdrs then Ok 1
          else Err 24)
    joinAllOrCancel sc t-proxy t-cli)

  -- | Run TCP listener, ReverseProxy, and 502 error handling checks.
  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in
    let ok-no-tls = (case http-get "https://example.com/" of
      Err HttpNoTls -> True
      _ -> False) in
    if not ok-no-tls then 10
    else case net-listen "127.0.0.1" 0 of
      Err _ -> 11
      Ok ln1 -> case net-listener-addr ln1 of
        Err _ -> let u1 = net-listener-close ln1 in 12
        Ok a1 -> case run-listener-scope ln1 (parse-addr-port a1) of
          Err e -> e
          Ok (1, 1) -> case net-listen "127.0.0.1" 0 of
            Err _ -> 20
            Ok ln2 -> case net-listener-addr ln2 of
              Err _ -> let u2 = net-listener-close ln2 in 21
              Ok a2 -> case run-proxy-scope ln2 (parse-addr-port a2) of
                Err e -> e
                Ok (1, 1) ->
                  let bad-proxy = reverse-proxy-single-host "http" "127.0.0.1" 1 "" in
                  let bad-resp = reverse-proxy-forward bad-proxy (http-new-request method-get "/fail" (bytes-new 0)) in
                  let custom-proxy = reverse-proxy-with-error-handler bad-proxy (\msg -> http-response-text status-service-unavailable (string-concat "custom:" msg)) in
                  let custom-resp = reverse-proxy-forward custom-proxy (http-new-request method-get "/fail" (bytes-new 0)) in
                  if not (resp-status bad-resp == 502) then 26
                  else if not (resp-status custom-resp == 503 && contains-sub (text (resp-body custom-resp)) "custom:") then 27
                  else 200
                _ -> 25
          _ -> 18
"#
    );

    let code = run_cinch("http_listener_proxy", &main);
    assert_eq!(code, 200, "http_listener_proxy failed with code {code}");
}

/// Real TLS 1.3 HTTPS server (`http-serve-tls-listener-n`), `Effect.Http` (`http-get-tls`),
/// `HttpClient` (`http-client-with-tls`, `http-client-get-tls`, `http-client-post-tls`, cookies &
/// 302 redirect), and `ReverseProxy` over `https://` (`reverse-proxy-with-tls` + `reverse-proxy-forward-tls`).
#[test]
fn https_tls13_server_client_and_reverse_proxy_live_loopback() {
    let (ec_root, ec_chain, ec_key) = ecdsa_fixture_pems();

    let main = format!(
        r#"-- | Live TLS 1.3 HTTPS server, client, cookies/redirects, and HTTPS ReverseProxy checks.
module Main where
  import Crypto.Tls
  import Crypto.X509
  import Data.Bool
  import Data.Bytes
  import Data.CookieJar
  import Data.Int
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Concurrent
  import Effect.Http
  import Effect.Http.Server
  import Effect.Http.Util
  import Effect.Net
  import Effect.Tls
  import Text.Cookie
  import Text.Http
  import Text.Show (nat-to-string)

  -- | Fixture ECDSA P-256 root CA PEM.
  ec-root :: Bytes
  ec-root = string-to-bytes {ec_root}

  -- | Fixture ECDSA P-256 certificate chain PEM.
  ec-chain :: Bytes
  ec-chain = string-to-bytes {ec_chain}

  -- | Fixture ECDSA P-256 private key PEM.
  ec-key :: Bytes
  ec-key = string-to-bytes {ec_key}

  -- | Verification timestamp inside fixture validity window.
  valid-now :: I64
  valid-now = 1700003600

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Load a root CA CertPool from PEM bytes.
  pool-from-pem :: Bytes -> CertPool
  pool-from-pem pem = case cert-pool-append-pem cert-pool-new pem of (p, _) -> p

  -- | Build client TlsConfig verifying against the ECDSA test CA for "ecdsa.test".
  client-tls-cfg :: Unit -> TlsConfig
  client-tls-cfg u =
    tls-config-with-now valid-now
      (tls-config-with-roots (pool-from-pem ec-root)
        (tls-config-with-server-name "ecdsa.test" tls-config-default))

  -- | Build server TlsConfig from the ECDSA certificate chain and private key.
  server-tls-cfg :: Unit -> Option TlsConfig
  server-tls-cfg u = case tls-load-x509-key-pair ec-chain ec-key of
    Err _ -> None
    Ok cert -> Some (tls-config-with-certificate cert tls-config-default)

  -- | Parse the port number from "<host>:<port>".
  parse-addr-port :: String -> Nat
  parse-addr-port s =
    let b = string-to-bytes s in
    let n = bytes-len b in
    parse-port-after-colon b n n

  -- | Locate last colon and parse trailing decimal digits.
  parse-port-after-colon :: Bytes -> Nat -> Nat -> Nat
  parse-port-after-colon b idx n =
    if idx == 0 then 0
    else
      let prev = idx - 1 in
      if prev < bytes-len b && bytes-get b prev == 58 then parse-dec-from b idx n 0
      else parse-port-after-colon b prev n

  -- | Parse decimal digits in [i, n).
  parse-dec-from :: Bytes -> Nat -> Nat -> Nat -> Nat
  parse-dec-from b i n acc =
    if i >= n || i >= bytes-len b then acc
    else
      let c = bytes-get b i in
      if c >= 48 && c <= 57 then parse-dec-from b (i + 1) n (acc * 10 + (if 48 <= c then c - 48 else 0))
      else acc

  -- | Run HTTPS server and clients concurrently in a dedicated scope.
  run-https-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-https-scope listener port = withScope (\sc -> do
    t-srv <- spawn sc (\() -> case server-tls-cfg () of
      None -> let u1 = net-listener-close listener in Err 30
      Some srv-tls ->
        let h = http-handler-pure (\req ->
          let p = req-path req in
          if p == "/hello" then http-response-text status-ok "tls-hello-ok"
          else if p == "/login" then
            let c = cookie-new "sid" "tls42" in
            resp-add-cookie (http-response-redirect status-found "/me") c
          else if p == "/me" then
            let sid = case req-cookie req "sid" of Some c -> cookie-value c; None -> "" in
            http-response-text status-ok (string-concat "me-tls:" sid)
          else if p == "/echo" then
            http-response-text status-ok (string-concat "post:" (text (req-body req)))
          else if p == "/api/proxy-check" then
            http-response-text status-ok (string-concat "https-proxied:" (text (req-body req)))
          else http-response-error status-not-found "missing") in
        let sres = http-serve-tls-listener-n http-server-config-default srv-tls listener 5 h in
        let u1 = net-listener-close listener in
        case sres of
          Ok 5 -> Ok 1
          _ -> Err 33)
    t-cli <- spawn sc (\() ->
      let cli-tls = client-tls-cfg () in
      let u1 = string-concat-all ["https://127.0.0.1:", nat-to-string port, "/hello"] in
      case http-get-tls cli-tls u1 of
        Err _ -> Err 34
        Ok r1 ->
          if not (http-status r1 == 200 && text (http-body r1) == "tls-hello-ok") then Err 35
          else
            let jar0 = cookie-jar-new None in
            let c0 = http-client-with-tls (http-client-with-time 1700003600 (http-client-with-jar jar0 http-client-default)) cli-tls in
            let u2 = string-concat-all ["https://127.0.0.1:", nat-to-string port, "/login"] in
            case http-client-get-tls c0 u2 of
              Err _ -> Err 36
              Ok (r2, c1) ->
                if not (resp-status r2 == 200 && text (resp-body r2) == "me-tls:tls42") then Err 37
                else
                  let u3 = string-concat-all ["https://127.0.0.1:", nat-to-string port, "/echo"] in
                  case http-client-post-tls c1 u3 "text/plain" (string-to-bytes "tls-post-body") of
                    Err _ -> Err 38
                    Ok (r3, _) ->
                      if not (resp-status r3 == 200 && text (resp-body r3) == "post:tls-post-body") then Err 39
                      else
                        let skip-tls = tls-config-with-insecure-skip-verify True tls-config-default in
                        let rp = reverse-proxy-with-tls (reverse-proxy-single-host "https" "127.0.0.1" port "/api") skip-tls in
                        let r4 = reverse-proxy-forward-tls rp (http-new-request method-post "/proxy-check" (string-to-bytes "secret")) in
                        if resp-status r4 == 200 && text (resp-body r4) == "https-proxied:secret" then Ok 1
                        else Err 40)
    joinAllOrCancel sc t-srv t-cli)

  -- | Entry point.
  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in
    case net-listen "127.0.0.1" 0 of
      Err _ -> 31
      Ok listener -> case net-listener-addr listener of
        Err _ -> let u1 = net-listener-close listener in 32
        Ok addr -> case run-https-scope listener (parse-addr-port addr) of
          Ok (1, 1) -> 200
          Err e -> e
          _ -> 41
"#,
        ec_root = lit(&ec_root),
        ec_chain = lit(&ec_chain),
        ec_key = lit(&ec_key),
    );

    let code = run_cinch("https_tls13_e2e", &main);
    assert_eq!(code, 200, "https_tls13_e2e failed with code {code}");
}

/// Real TLS 1.3 SMTP verification:
/// - Implicit SMTPS (`smtp-send-mail-tls` + `smtp-tls-auth` with `PLAIN` auth over non-localhost `127.0.0.2`)
/// - RFC 3207 `STARTTLS` upgrade (`smtp-dial` + `smtp-starttls` resetting greeting state -> encrypted `EHLO`
///   -> `smtp-tls-extension` / `smtp-tls-auth-offered` / `smtp-tls-auth` / `smtp-tls-noop` / `smtp-tls-reset`
///   -> `smtp-send-mail-starttls`)
#[test]
fn smtp_tls_and_starttls_live_loopback() {
    let (ec_root, ec_chain, ec_key) = ecdsa_fixture_pems();

    let main = format!(
        r#"-- | Live TLS 1.3 SMTPS and STARTTLS verification.
module Main where
  import Crypto.Tls
  import Crypto.X509
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Concurrent
  import Effect.Net
  import Effect.Smtp
  import Effect.Tls
  import Text.Smtp

  -- | Fixture ECDSA P-256 root CA PEM.
  ec-root :: Bytes
  ec-root = string-to-bytes {ec_root}

  -- | Fixture ECDSA P-256 certificate chain PEM.
  ec-chain :: Bytes
  ec-chain = string-to-bytes {ec_chain}

  -- | Fixture ECDSA P-256 private key PEM.
  ec-key :: Bytes
  ec-key = string-to-bytes {ec_key}

  -- | Verification timestamp inside fixture validity window.
  valid-now :: I64
  valid-now = 1700003600

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Load a root CA CertPool from PEM bytes.
  pool-from-pem :: Bytes -> CertPool
  pool-from-pem pem = case cert-pool-append-pem cert-pool-new pem of (p, _) -> p

  -- | Build client TlsConfig trusting the ECDSA test CA for "ecdsa.test".
  client-tls-cfg :: Unit -> TlsConfig
  client-tls-cfg u =
    tls-config-with-now valid-now
      (tls-config-with-roots (pool-from-pem ec-root)
        (tls-config-with-server-name "ecdsa.test" tls-config-default))

  -- | Build server TlsConfig from the ECDSA certificate chain and private key.
  server-tls-cfg :: Unit -> Option TlsConfig
  server-tls-cfg u = case tls-load-x509-key-pair ec-chain ec-key of
    Err _ -> None
    Ok cert -> Some (tls-config-with-certificate cert tls-config-default)

  -- | Check whether `hay` contains `needle`.
  contains-sub :: String -> String -> Bool
  contains-sub hay needle =
    let hb = string-to-bytes hay in
    let nb = string-to-bytes needle in
    let nn = bytes-len nb in
    if nn == 0 then True
    else if nn > bytes-len hb then False
    else scan-sub hb nb 0 nn

  -- | Scan `hb` for `nb` from `i`.
  scan-sub :: Bytes -> Bytes -> Nat -> Nat -> Bool
  scan-sub hb nb i nn =
    if i + nn > bytes-len hb then False
    else
      let out = bytes-new nn in
      let u = bytes-copy-range out hb 0 i nn in
      if eq-i64 (bytes-compare out nb) 0 then True
      else scan-sub hb nb (i + 1) nn

  -- | Parse port from "<host>:<port>".
  parse-addr-port :: String -> Nat
  parse-addr-port s =
    let b = string-to-bytes s in
    let n = bytes-len b in
    parse-port-after-colon b n n

  -- | Locate last colon and parse trailing decimal digits.
  parse-port-after-colon :: Bytes -> Nat -> Nat -> Nat
  parse-port-after-colon b idx n =
    if idx == 0 then 0
    else
      let prev = idx - 1 in
      if prev < bytes-len b && bytes-get b prev == 58 then parse-dec-from b idx n 0
      else parse-port-after-colon b prev n

  -- | Parse decimal digits in [i, n).
  parse-dec-from :: Bytes -> Nat -> Nat -> Nat -> Nat
  parse-dec-from b i n acc =
    if i >= n || i >= bytes-len b then acc
    else
      let c = bytes-get b i in
      if c >= 48 && c <= 57 then parse-dec-from b (i + 1) n (acc * 10 + (if 48 <= c then c - 48 else 0))
      else acc

  -- | Send a CRLF-terminated line over TLS.
  tls-send-line :: forall r. using (io :: Io r) => TlsConn r -> String ->{{IO r}} Result TlsIoError (TlsConn r)
  tls-send-line conn s = case tls-write conn (string-to-bytes (string-concat s "\r\n")) of
    Err e -> Err e
    Ok (_, c2) -> Ok c2

  -- | Read until CRLF over TLS.
  tls-read-cmd :: forall r. using (io :: Io r) => TlsConn r -> Bytes ->{{IO r}} Result TlsIoError (String, TlsConn r)
  tls-read-cmd conn acc =
    if contains-sub (text acc) "\r\n" then Ok (text acc, conn)
    else case tls-read conn 4096 of
      Err e -> Err e
      Ok (chunk, c2) ->
        if bytes-len chunk == 0 then Ok (text acc, c2)
        else tls-read-cmd c2 (bytes-concat [acc, chunk])

  -- | Read until end-of-DATA marker over TLS.
  tls-expect-data :: forall r. using (io :: Io r) => TlsConn r -> Bytes ->{{IO r}} Result TlsIoError (String, TlsConn r)
  tls-expect-data conn acc =
    if contains-sub (text acc) "\r\n.\r\n" then Ok (text acc, conn)
    else case tls-read conn 4096 of
      Err e -> Err e
      Ok (chunk, c2) ->
        if bytes-len chunk == 0 then Ok (text acc, c2)
        else tls-expect-data c2 (bytes-concat [acc, chunk])

  -- | Run a minimal TLS SMTP server transcript after handshake.
  run-tls-smtp-session :: forall r. using (io :: Io r) => TlsConn r -> Bool ->{{IO r}} Result Int Int
  run-tls-smtp-session conn0 send-banner =
    let step0 = if send-banner then tls-send-line conn0 "220 mail.ecdsa.test ESMTP" else Ok conn0 in
    case step0 of
      Err _ -> Err 50
      Ok c1 -> case tls-read-cmd c1 (bytes-new 0) of
        Err _ -> Err 51
        Ok (ehlo-cmd, c2) ->
          if not (contains-sub ehlo-cmd "EHLO ") then Err 52
          else case tls-send-line c2 "250-mail.ecdsa.test\r\n250 AUTH PLAIN" of
            Err _ -> Err 53
            Ok c3 -> case tls-read-cmd c3 (bytes-new 0) of
              Err _ -> Err 54
              Ok (auth-cmd, c4) ->
                if not (contains-sub auth-cmd "AUTH PLAIN ") then Err 55
                else case tls-send-line c4 "235 2.7.0 Authentication successful" of
                  Err _ -> Err 56
                  Ok c5 -> case tls-read-cmd c5 (bytes-new 0) of
                    Err _ -> Err 57
                    Ok (mail-cmd, c6) ->
                      if not (contains-sub mail-cmd "MAIL FROM:<a@test>") then Err 58
                      else case tls-send-line c6 "250 OK" of
                        Err _ -> Err 59
                        Ok c7 -> case tls-read-cmd c7 (bytes-new 0) of
                          Err _ -> Err 60
                          Ok (rcpt-cmd, c8) ->
                            if not (contains-sub rcpt-cmd "RCPT TO:<b@test>") then Err 61
                            else case tls-send-line c8 "250 OK" of
                              Err _ -> Err 62
                              Ok c9 -> case tls-read-cmd c9 (bytes-new 0) of
                                Err _ -> Err 63
                                Ok (data-cmd, c10) ->
                                  if not (contains-sub data-cmd "DATA") then Err 64
                                  else case tls-send-line c10 "354 End data" of
                                    Err _ -> Err 65
                                    Ok c11 -> case tls-expect-data c11 (bytes-new 0) of
                                      Err _ -> Err 66
                                      Ok (body-wire, c12) ->
                                        if not (contains-sub body-wire "Hello TLS SMTP") then Err 67
                                        else case tls-send-line c12 "250 Queued" of
                                          Err _ -> Err 68
                                          Ok c13 -> case tls-read-cmd c13 (bytes-new 0) of
                                            Err _ -> Err 69
                                            Ok (quit-cmd, c14) ->
                                              if not (contains-sub quit-cmd "QUIT") then Err 70
                                              else case tls-send-line c14 "221 Bye" of
                                                Err _ -> Err 71
                                                Ok c15 ->
                                                  let u1 = tls-close c15 in
                                                  Ok 1

  -- | Implicit SMTPS scope over non-localhost 127.0.0.2 (proving PLAIN auth passes tls=True).
  run-smtps-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-smtps-scope ln port = withScope (\sc -> do
    t-srv <- spawn sc (\() -> case server-tls-cfg () of
      None -> let u1 = net-listener-close ln in Err 40
      Some srv-tls -> case net-accept ln of
        Err _ -> let u1 = net-listener-close ln in Err 43
        Ok sock ->
          let u1 = net-listener-close ln in
          case tls-server sock srv-tls of
            Err _ -> let u2 = net-close sock in Err 44
            Ok conn -> run-tls-smtp-session conn True)
    t-cli <- spawn sc (\() ->
      let cli-tls = client-tls-cfg () in
      let auth = SmtpPlainAuth "" "alice" "s3cr3t" "127.0.0.2" in
      case smtp-send-mail-tls "127.0.0.2" port cli-tls (Some auth) "a@test" ["b@test"] (string-to-bytes "Hello TLS SMTP") of
        Ok _ -> Ok 1
        Err _ -> Err 45)
    joinAllOrCancel sc t-srv t-cli)

  -- | RFC 3207 STARTTLS upgrade scope over non-localhost 127.0.0.2.
  run-starttls-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-starttls-scope ln port = withScope (\sc -> do
    t-srv <- spawn sc (\() -> case server-tls-cfg () of
      None -> let u1 = net-listener-close ln in Err 80
      Some srv-tls -> case net-accept ln of
        Err _ -> let u1 = net-listener-close ln in Err 82
        Ok sock ->
          let u0 = net-listener-close ln in
          let u1 = net-write sock (string-to-bytes "220 mail.ecdsa.test ESMTP\r\n") in
          case net-read sock 4096 of
            Err _ -> let u2 = net-close sock in Err 83
            Ok ehlo1 ->
              if not (contains-sub (text ehlo1) "EHLO ") then let u2 = net-close sock in Err 84
              else
                let u2 = net-write sock (string-to-bytes "250-mail.ecdsa.test\r\n250 STARTTLS\r\n") in
                case net-read sock 4096 of
                  Err _ -> let u3 = net-close sock in Err 85
                  Ok st ->
                    if not (contains-sub (text st) "STARTTLS") then let u3 = net-close sock in Err 86
                    else
                      let u3 = net-write sock (string-to-bytes "220 Ready to start TLS\r\n") in
                      case tls-server sock srv-tls of
                        Err _ -> let u4 = net-close sock in Err 87
                        Ok conn -> run-tls-smtp-session conn False)
    t-cli <- spawn sc (\() ->
      let cli-tls = client-tls-cfg () in
      let auth = SmtpPlainAuth "" "alice" "s3cr3t" "127.0.0.2" in
      case smtp-send-mail-starttls "127.0.0.2" port cli-tls (Some auth) "a@test" ["b@test"] (string-to-bytes "Hello TLS SMTP") of
        Ok _ -> Ok 1
        Err _ -> Err 88)
    joinAllOrCancel sc t-srv t-cli)

  -- | Negative check: STARTTLS refused when server does not advertise STARTTLS.
  run-no-starttls-scope :: forall r. using (io :: Io r) => Listener -> Nat ->{{Async r, IO r, Cancel r, Diverge}} Result Int (Int, Int)
  run-no-starttls-scope ln port = withScope (\sc -> do
    t-srv <- spawn sc (\() -> case net-accept ln of
      Err _ -> let u1 = net-listener-close ln in Err 90
      Ok sock ->
        let u0 = net-listener-close ln in
        let u1 = net-write sock (string-to-bytes "220 plain.test ESMTP\r\n") in
        let u2 = net-read sock 4096 in
        let u3 = net-write sock (string-to-bytes "250 plain.test\r\n") in
        let u4 = net-read sock 4096 in
        let u5 = net-close sock in
        Ok 1)
    t-cli <- spawn sc (\() ->
      let cli-tls = client-tls-cfg () in
      case smtp-dial "127.0.0.1" port of
        Err _ -> Err 91
        Ok c -> case smtp-starttls c cli-tls of
          Err (SmtpProtocolError _) ->
            let u1 = smtp-close c in
            Ok 1
          _ ->
            let u1 = smtp-close c in
            Err 92)
    joinAllOrCancel sc t-srv t-cli)

  -- | Run all three TLS SMTP integration tests.
  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in
    case net-listen "127.0.0.2" 0 of
      Err _ -> 41
      Ok ln1 -> case net-listener-addr ln1 of
        Err _ -> let u1 = net-listener-close ln1 in 42
        Ok a1 -> case run-smtps-scope ln1 (parse-addr-port a1) of
          Err e -> e
          Ok (1, 1) -> case net-listen "127.0.0.2" 0 of
            Err _ -> 80
            Ok ln2 -> case net-listener-addr ln2 of
              Err _ -> let u2 = net-listener-close ln2 in 81
              Ok a2 -> case run-starttls-scope ln2 (parse-addr-port a2) of
                Err e -> e
                Ok (1, 1) -> case net-listen "127.0.0.1" 0 of
                  Err _ -> 93
                  Ok ln3 -> case net-listener-addr ln3 of
                    Err _ -> let u3 = net-listener-close ln3 in 94
                    Ok a3 -> case run-no-starttls-scope ln3 (parse-addr-port a3) of
                      Ok (1, 1) -> 200
                      Err e -> e
                      _ -> 95
                _ -> 89
          _ -> 46
"#,
        ec_root = lit(&ec_root),
        ec_chain = lit(&ec_chain),
        ec_key = lit(&ec_key),
    );

    let code = run_cinch("smtp_tls_starttls", &main);
    assert_eq!(code, 200, "smtp_tls_starttls failed with code {code}");
}
