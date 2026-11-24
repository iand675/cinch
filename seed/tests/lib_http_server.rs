//! HTTP/1.1 protocol framing, Go 1.22 ServeMux routing, Effect.Http.Server over Effect.Net,
//! cookie-jar client integration, and Effect.HttpTest ResponseRecorder verified against Go 1.24
//! reference vectors (`tooling/http_server_vectors.go`) and live loopback TCP roundtrips.
mod common;

use serde_json::Value;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;

/// All source-distributed modules required by Text.Http, Effect.Http.Server, Effect.Tls, and Effect.HttpTest.
fn http_modules<'a>(main: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
        ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
        ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
        ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
        ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
        ("Text.Base64.cinch", include_str!("../prelude/Text/Base64.cinch")),
        ("Text.Base64.Encoding.cinch", include_str!("../prelude/Text/Base64/Encoding.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
        ("Text.Pem.cinch", include_str!("../prelude/Text/Pem.cinch")),
        ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
        ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
        ("Text.Uri.cinch", include_str!("../prelude/Text/Uri.cinch")),
        ("Text.Punycode.cinch", include_str!("../prelude/Text/Punycode.cinch")),
        ("Text.Cookie.cinch", include_str!("../prelude/Text/Cookie.cinch")),
        ("Text.TextProto.cinch", include_str!("../prelude/Text/TextProto.cinch")),
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
        ("Data.CookieJar.cinch", include_str!("../prelude/Data/CookieJar.cinch")),
        ("Effect.Net.cinch", include_str!("../prelude/Effect/Net.cinch")),
        ("Effect.Http.cinch", include_str!("../prelude/Effect/Http.cinch")),
        ("Text.Http.cinch", include_str!("../prelude/Text/Http.cinch")),
        ("Effect.Http.Server.cinch", include_str!("../prelude/Effect/Http/Server.cinch")),
        ("Effect.HttpTest.cinch", include_str!("../prelude/Effect/HttpTest.cinch")),
        ("Main.cinch", main),
    ]
}

/// Escape a Rust string into a Cinch string literal.
fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                out.push_str(&format!("\\u{{{:x}}}", c as u32))
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Format a Cinch List String literal.
fn str_list(items: &[String]) -> String {
    format!(
        "[{}]",
        items.iter().map(|s| lit(s)).collect::<Vec<_>>().join(", ")
    )
}

/// Format a Cinch List (String, String) literal sorted by key.
fn kv_list(pairs: &[(String, String)]) -> String {
    format!(
        "[{}]",
        pairs
            .iter()
            .map(|(k, v)| format!("({}, {})", lit(k), lit(v)))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// Format a Cinch Int literal.
fn cinch_int(v: i64) -> String {
    if v < 0 {
        format!("(0 - {})", -v)
    } else {
        v.to_string()
    }
}

/// Every declaration in Text.Http, Effect.Http.Server, and Effect.HttpTest carries canonical documentation.
#[test]
fn http_server_modules_have_canonical_documentation() {
    common::assert_documented("Text.Http", include_str!("../prelude/Text/Http.cinch"));
    common::assert_documented(
        "Effect.Http.Server",
        include_str!("../prelude/Effect/Http/Server.cinch"),
    );
    common::assert_documented(
        "Effect.HttpTest",
        include_str!("../prelude/Effect/HttpTest.cinch"),
    );
}

/// Verify all Go 1.24 reference vectors: StatusText, CanonicalHeaderKey, ServeMux registration &
/// conflicts, ServeMux routing/precedence/path-values/405-Allow/redirects, HTTP/1.1 wire request
/// and response parsing + chunked encoding, ResponseRecorder, and httptest-new-request.
#[test]
fn http_server_matches_go_reference_vectors() {
    let f: Value = serde_json::from_str(include_str!("fixtures/http-server-vectors.json")).unwrap();
    let mut rows: Vec<String> = Vec::new();

    // 1. Status texts (68 vectors)
    for sv in f["status_texts"].as_array().unwrap() {
        let code = sv["code"].as_u64().unwrap();
        let text = sv["text"].as_str().unwrap();
        rows.push(format!("check-status {code} {}", lit(text)));
    }

    // 2. Canonical header keys (10 vectors)
    for cv in f["canonical_headers"].as_array().unwrap() {
        rows.push(format!(
            "check-canon {} {}",
            lit(cv["in"].as_str().unwrap()),
            lit(cv["out"].as_str().unwrap())
        ));
    }

    // 3. Pattern registration & conflict detection (36 vectors)
    for pr in f["pattern_register"].as_array().unwrap() {
        let pats: Vec<String> = pr["patterns"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let valid = if pr["valid"].as_bool().unwrap() { "True" } else { "False" };
        rows.push(format!("check-reg {} {valid}", str_list(&pats)));
    }

    // 4. ServeMux routing, precedence, path values, 405 Allow, redirects (36 vectors across 3 scenarios)
    for sc in f["mux_scenarios"].as_array().unwrap() {
        let routes: Vec<String> = sc["routes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let queries = sc["queries"].as_array().unwrap();
        let expected = sc["expected"].as_array().unwrap();
        for (q, e) in queries.iter().zip(expected.iter()) {
            let mut pvs: Vec<(String, String)> = e["path_values"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
                .collect();
            pvs.sort();
            rows.push(format!(
                "check-mux {} {} {} {} {} {} {} {} {}",
                str_list(&routes),
                lit(q["method"].as_str().unwrap()),
                lit(q["host"].as_str().unwrap()),
                lit(q["target"].as_str().unwrap()),
                e["status"].as_u64().unwrap(),
                lit(e["handler"].as_str().unwrap()),
                lit(e["location"].as_str().unwrap()),
                lit(e["allow"].as_str().unwrap()),
                kv_list(&pvs)
            ));
        }
    }

    // 5. Wire HTTP/1.1 request parsing + query/form params (8 vectors)
    for wr in f["wire_requests"].as_array().unwrap() {
        let valid = wr["valid"].as_bool().unwrap();
        if !valid {
            rows.push(format!("check-wire-req-bad {}", lit(wr["raw"].as_str().unwrap())));
        } else {
            let mut hdrs: Vec<(String, String)> = Vec::new();
            if let Some(arr) = wr["headers"].as_array() {
                for item in arr {
                    let a = item.as_array().unwrap();
                    hdrs.push((
                        a[0].as_str().unwrap().to_string(),
                        a[1].as_str().unwrap().to_string(),
                    ));
                }
            }
            let mut qp: Vec<(String, String)> = wr["query_params"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
                .collect();
            qp.sort();
            let mut fp: Vec<(String, String)> = wr["form_params"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
                .collect();
            fp.sort();
            let clz = if wr["close"].as_bool().unwrap() { "True" } else { "False" };
            rows.push(format!(
                "check-wire-req {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                lit(wr["raw"].as_str().unwrap()),
                lit(wr["method"].as_str().unwrap()),
                lit(wr["target"].as_str().unwrap()),
                lit(wr["path"].as_str().unwrap()),
                lit(wr["raw_query"].as_str().unwrap()),
                lit(wr["host"].as_str().unwrap()),
                wr["proto_major"].as_u64().unwrap(),
                wr["proto_minor"].as_u64().unwrap(),
                kv_list(&hdrs),
                lit(wr["body_hex"].as_str().unwrap()),
                clz,
                cinch_int(wr["content_length"].as_i64().unwrap()),
                kv_list(&qp),
                kv_list(&fp)
            ));
        }
    }

    // 6. Wire HTTP/1.1 response parsing (7 vectors)
    for wr in f["wire_responses"].as_array().unwrap() {
        let valid = wr["valid"].as_bool().unwrap();
        if !valid {
            rows.push(format!(
                "check-wire-resp-bad {} {}",
                lit(wr["req_method"].as_str().unwrap()),
                lit(wr["raw"].as_str().unwrap())
            ));
        } else {
            let mut hdrs: Vec<(String, String)> = Vec::new();
            if let Some(arr) = wr["headers"].as_array() {
                for item in arr {
                    let a = item.as_array().unwrap();
                    hdrs.push((
                        a[0].as_str().unwrap().to_string(),
                        a[1].as_str().unwrap().to_string(),
                    ));
                }
            }
            rows.push(format!(
                "check-wire-resp {} {} {} {} {} {} {}",
                lit(wr["req_method"].as_str().unwrap()),
                lit(wr["raw"].as_str().unwrap()),
                wr["status"].as_u64().unwrap(),
                wr["proto_major"].as_u64().unwrap(),
                wr["proto_minor"].as_u64().unwrap(),
                kv_list(&hdrs),
                lit(wr["body_hex"].as_str().unwrap())
            ));
        }
    }

    // 7. ResponseRecorder scenarios (4 vectors)
    for rv in f["recorders"].as_array().unwrap() {
        let mut hdrs: Vec<(String, String)> = Vec::new();
        if let Some(arr) = rv["headers"].as_array() {
            for item in arr {
                let a = item.as_array().unwrap();
                hdrs.push((
                    a[0].as_str().unwrap().to_string(),
                    a[1].as_str().unwrap().to_string(),
                ));
            }
        }
        let fl = if rv["flushed"].as_bool().unwrap() { "True" } else { "False" };
        rows.push(format!(
            "check-recorder {} {} {} {} {} {}",
            lit(rv["name"].as_str().unwrap()),
            rv["status"].as_u64().unwrap(),
            lit(rv["body"].as_str().unwrap()),
            lit(rv["content_type"].as_str().unwrap()),
            kv_list(&hdrs),
            fl
        ));
    }

    // 8. httptest-new-request scenarios (3 vectors)
    for nr in f["new_requests"].as_array().unwrap() {
        rows.push(format!(
            "check-new-req {} {} {} {} {} {} {} {} {} {} {}",
            lit(nr["method"].as_str().unwrap()),
            lit(nr["target"].as_str().unwrap()),
            lit(nr["body"].as_str().unwrap()),
            lit(nr["got_method"].as_str().unwrap()),
            lit(nr["got_host"].as_str().unwrap()),
            lit(nr["got_path"].as_str().unwrap()),
            lit(nr["got_query"].as_str().unwrap()),
            lit(nr["got_uri"].as_str().unwrap()),
            nr["proto_major"].as_u64().unwrap(),
            nr["proto_minor"].as_u64().unwrap(),
            lit(nr["remote_addr"].as_str().unwrap())
        ));
    }

    // 9. Extra wire roundtrip & chunked codec checks (3 vectors)
    rows.push("check-chunk-roundtrip".to_string());
    rows.push("check-wire-roundtrip".to_string());
    rows.push("check-conflicting-cl".to_string());

    println!("Total Go reference + protocol vectors: {}", rows.len());
    assert!(rows.len() < 200, "exit code names failing row; got {}", rows.len());

    let main = format!(
        "{VECTOR_HELPERS}\n  -- | Run all reference vector checks.\n  main :: Unit ->{{IO r}} Int\n  main u = with (io = newIO ()) in first-fail [{}] 0\n",
        rows.join(",\n    ")
    );

    let dir = common::project("http_vectors", &http_modules(&main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 200,
        "reference vector row {code} failed (row: {})",
        rows.get(code as usize).map(String::as_str).unwrap_or("?")
    );
}

/// Live loopback HTTP/1.1 server over Effect.Net: keep-alive multi-request connection,
/// ServeMux routing + path params, chunked request body, 405 Allow, 301 trailing-slash redirect,
/// HEAD without body, and 413/400 error enforcement.
#[test]
fn http_server_live_loopback_tcp_roundtrip() {
    // Peer 1: Real TCP HTTP/1.1 client exercising 5 requests on a single keep-alive connection.
    let listener1 = TcpListener::bind("127.0.0.1:0").expect("bind loopback 1");
    let port1 = listener1.local_addr().unwrap().port();
    let (tx1, rx1) = mpsc::channel::<Vec<String>>();

    std::thread::spawn(move || {
        let mut results = Vec::new();
        if let Ok((mut s, _)) = listener1.accept() {
            let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(10)));
            let requests: &[&[u8]] = &[
                // 1. GET with path param + query param (keep-alive)
                b"GET /items/42?lang=cinch HTTP/1.1\r\nHost: example.com\r\n\r\n",
                // 2. POST with chunked transfer encoding
                b"POST /echo HTTP/1.1\r\nHost: example.com\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nHello\r\n7\r\n, Cinch\r\n0\r\n\r\n",
                // 3. Disallowed method -> 405 with Allow header
                b"PUT /items/42 HTTP/1.1\r\nHost: example.com\r\n\r\n",
                // 4. Subtree without trailing slash -> 301 redirect
                b"GET /docs HTTP/1.1\r\nHost: example.com\r\n\r\n",
                // 5. HEAD with Connection: close -> 200 with Content-Length and empty wire body
                b"HEAD /items/42 HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
            ];
            for (idx, req) in requests.iter().enumerate() {
                s.write_all(req).expect("write req");
                s.flush().expect("flush req");
                let resp = read_one_http_response(&mut s, idx == 4);
                results.push(resp);
            }
        }
        let _ = tx1.send(results);
    });

    // Peer 2: Oversized body -> 413 Request Entity Too Large
    let listener2 = TcpListener::bind("127.0.0.1:0").expect("bind loopback 2");
    let port2 = listener2.local_addr().unwrap().port();
    let (tx2, rx2) = mpsc::channel::<String>();

    std::thread::spawn(move || {
        let mut out = String::new();
        if let Ok((mut s, _)) = listener2.accept() {
            let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(10)));
            let req = b"POST /echo HTTP/1.1\r\nHost: example.com\r\nContent-Length: 100\r\n\r\n";
            let _ = s.write_all(req);
            let _ = s.flush();
            let mut buf = Vec::new();
            let _ = s.read_to_end(&mut buf);
            out = String::from_utf8_lossy(&buf).into_owned();
        }
        let _ = tx2.send(out);
    });

    // Peer 3: Malformed request line -> 400 Bad Request
    let listener3 = TcpListener::bind("127.0.0.1:0").expect("bind loopback 3");
    let port3 = listener3.local_addr().unwrap().port();
    let (tx3, rx3) = mpsc::channel::<String>();

    std::thread::spawn(move || {
        let mut out = String::new();
        if let Ok((mut s, _)) = listener3.accept() {
            let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(10)));
            let _ = s.write_all(b"GARBAGE\r\n\r\n");
            let _ = s.flush();
            let mut buf = Vec::new();
            let _ = s.read_to_end(&mut buf);
            out = String::from_utf8_lossy(&buf).into_owned();
        }
        let _ = tx3.send(out);
    });

    let main = format!(
        r#"-- | Live loopback HTTP/1.1 server tests over Effect.Net.
module Main where
  import Data.Bytes
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http.Server
  import Effect.HttpTest
  import Text.Http

  -- | Build the test ServeMux handler.
  build-handler :: forall r. using (io :: Io r) => Unit -> HttpHandler r
  build-handler u =
    let m0 = mux-empty in
    let h-get-item = http-handler-pure (\req ->
      let id = case req-path-value req "id" of Some v -> v; None -> "" in
      let lang = case req-query-param req "lang" of Some v -> v; None -> "" in
      http-response-text status-ok (string-concat-all ["item=", id, "&lang=", lang])) in
    let h-del-item = http-handler-pure (\req -> http-response-text status-no-content "") in
    let h-echo = http-handler-pure (\req ->
      let body-txt = case string-from-bytes (req-body req) of Some s -> s; None -> "" in
      http-response-text status-ok (string-concat "echo:" body-txt)) in
    let h-docs = http-handler-pure (\req -> http-response-text status-ok "docs-root") in
    case mux-handle "GET /items/{{id}}" h-get-item m0 of
      Err _ -> http-not-found-handler
      Ok m1 -> case mux-handle "DELETE /items/{{id}}" h-del-item m1 of
        Err _ -> http-not-found-handler
        Ok m2 -> case mux-handle "POST /echo" h-echo m2 of
          Err _ -> http-not-found-handler
          Ok m3 -> case mux-handle "GET /docs/" h-docs m3 of
            Err _ -> http-not-found-handler
            Ok m4 -> http-mux-handler m4

  -- | Run all three loopback server sessions.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    let h = build-handler () in
    let c1 = (case httptest-serve-loopback "127.0.0.1" {port1} h of
      Ok n -> if n == 5 then 0 else 10
      Err _ -> 11) in
    let small-cfg = http-server-config-with-limits 4096 16 http-server-config-default in
    let c2 = (case httptest-serve-loopback-with small-cfg "127.0.0.1" {port2} h of
      Err (ServerProtoError (HttpProtoTooLarge _)) -> 0
      _ -> 20) in
    let c3 = (case httptest-serve-loopback "127.0.0.1" {port3} h of
      Err (ServerProtoError (HttpProtoBadRequestLine _)) -> 0
      _ -> 30) in
    let r-wire = httptest-serve-wire h (httptest-new-request "GET" "/items/7?lang=ok" (bytes-new 0)) in
    let c4 = (case r-wire of
      Ok resp ->
        let txt = case string-from-bytes (resp-body resp) of Some s -> s; None -> "" in
        if resp-status resp == 200 && txt == "item=7&lang=ok" then 0 else 40
      Err _ -> 41) in
    if c1 + c2 + c3 + c4 == 0 then 200 else c1 + c2 + c3 + c4
"#
    );

    let dir = common::project("http_live_server", &http_modules(&main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "live loopback server test failed with code {code}");

    let responses = rx1
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("peer 1 responses");
    assert_eq!(responses.len(), 5, "expected 5 keep-alive responses");
    assert!(responses[0].starts_with("HTTP/1.1 200 OK\r\n"), "resp 0: {}", responses[0]);
    assert!(responses[0].ends_with("\r\n\r\nitem=42&lang=cinch"), "resp 0 body: {}", responses[0]);
    assert!(responses[1].starts_with("HTTP/1.1 200 OK\r\n"), "resp 1: {}", responses[1]);
    assert!(responses[1].ends_with("\r\n\r\necho:Hello, Cinch"), "resp 1 body: {}", responses[1]);
    assert!(
        responses[2].starts_with("HTTP/1.1 405 Method Not Allowed\r\n"),
        "resp 2: {}",
        responses[2]
    );
    assert!(
        responses[2].contains("Allow: DELETE, GET, HEAD\r\n"),
        "resp 2 allow: {}",
        responses[2]
    );
    assert!(
        responses[3].starts_with("HTTP/1.1 301 Moved Permanently\r\n"),
        "resp 3: {}",
        responses[3]
    );
    assert!(
        responses[3].contains("Location: /docs/\r\n"),
        "resp 3 location: {}",
        responses[3]
    );
    assert!(responses[4].starts_with("HTTP/1.1 200 OK\r\n"), "resp 4: {}", responses[4]);
    assert!(
        responses[4].contains("Connection: close\r\n"),
        "resp 4 close: {}",
        responses[4]
    );
    assert!(
        responses[4].ends_with("\r\n\r\n"),
        "HEAD response must have empty wire body: {:?}",
        responses[4]
    );

    let r413 = rx2.recv_timeout(std::time::Duration::from_secs(10)).expect("peer 2");
    assert!(
        r413.starts_with("HTTP/1.1 413 Request Entity Too Large\r\n"),
        "expected 413, got: {r413:?}"
    );

    let r400 = rx3.recv_timeout(std::time::Duration::from_secs(10)).expect("peer 3");
    assert!(
        r400.starts_with("HTTP/1.1 400 Bad Request\r\n"),
        "expected 400, got: {r400:?}"
    );
}

/// Read a single HTTP/1.1 response from a persistent TCP stream (respecting Content-Length and HEAD).
fn read_one_http_response(stream: &mut std::net::TcpStream, is_head: bool) -> String {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];
    let head_end;
    loop {
        match stream.read(&mut byte) {
            Ok(0) | Err(_) => return String::from_utf8_lossy(&buf).into_owned(),
            Ok(_) => {
                buf.push(byte[0]);
                if buf.len() >= 4 && &buf[buf.len() - 4..] == b"\r\n\r\n" {
                    head_end = buf.len();
                    break;
                }
            }
        }
    }
    if is_head {
        return String::from_utf8_lossy(&buf).into_owned();
    }
    let head = String::from_utf8_lossy(&buf[..head_end]);
    let mut cl: usize = 0;
    for line in head.lines() {
        if let Some((k, v)) = line.split_once(':') {
            if k.eq_ignore_ascii_case("content-length") {
                cl = v.trim().parse().unwrap_or(0);
            }
        }
    }
    let mut body = vec![0u8; cl];
    if cl > 0 {
        stream.read_exact(&mut body).expect("read exact body");
    }
    buf.extend_from_slice(&body);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Test HttpClient with Data.CookieJar & Redirects over live loopback TCP (and Cinch server-to-client
/// roundtrip via loopback bridge!).
#[test]
fn http_client_cookie_jar_and_redirects_loopback() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind cookie test server");
    let port = listener.local_addr().unwrap().port();

    // Multi-connection loopback HTTP server handling /login -> 302 + Set-Cookie -> /me,
    // POST /update -> 303 -> /done, and /loop -> 302 -> /loop.
    std::thread::spawn(move || {
        for _ in 0..8 {
            let Ok((mut s, _)) = listener.accept() else { break };
            let _ = s.set_read_timeout(Some(std::time::Duration::from_secs(5)));
            let mut req_buf = vec![0u8; 4096];
            let n = s.read(&mut req_buf).unwrap_or(0);
            let req = String::from_utf8_lossy(&req_buf[..n]);
            let first_line = req.lines().next().unwrap_or("");
            let mut parts = first_line.split_whitespace();
            let method = parts.next().unwrap_or("");
            let path = parts.next().unwrap_or("/");
            let mut cookie = String::new();
            for line in req.lines().skip(1) {
                if let Some((k, v)) = line.split_once(':') {
                    if k.eq_ignore_ascii_case("cookie") {
                        cookie = v.trim().to_string();
                    }
                }
            }
            let resp = match (method, path) {
                ("GET", "/login") => {
                    "HTTP/1.1 302 Found\r\nSet-Cookie: sid=abc789; Path=/; HttpOnly\r\nLocation: /me\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
                }
                ("GET", "/me") => {
                    let body = format!("me:{cookie}");
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
                }
                ("POST", "/update") => {
                    "HTTP/1.1 303 See Other\r\nSet-Cookie: updated=yes; Path=/\r\nLocation: /done\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
                }
                ("GET", "/done") => {
                    let body = format!("done:{cookie}");
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len())
                }
                ("GET", "/loop") => {
                    "HTTP/1.1 302 Found\r\nLocation: /loop\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
                }
                _ => {
                    "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string()
                }
            };
            let _ = s.write_all(resp.as_bytes());
            let _ = s.flush();
            let _ = s.shutdown(std::net::Shutdown::Both);
        }
    });

    // Also run a Cinch-server <-> Cinch-client live loopback TCP bridge!
    let srv_listener = TcpListener::bind("127.0.0.1:0").expect("bind bridge srv");
    let srv_port = srv_listener.local_addr().unwrap().port();
    let cli_listener = TcpListener::bind("127.0.0.1:0").expect("bind bridge cli");
    let cli_port = cli_listener.local_addr().unwrap().port();

    std::thread::spawn(move || {
        if let (Ok((mut s_sock, _)), Ok((mut c_sock, _))) =
            (srv_listener.accept(), cli_listener.accept())
        {
            let mut s_read = s_sock.try_clone().unwrap();
            let mut c_write = c_sock.try_clone().unwrap();
            let t1 = std::thread::spawn(move || {
                let _ = std::io::copy(&mut c_sock, &mut s_sock);
                let _ = s_sock.shutdown(std::net::Shutdown::Write);
            });
            let _ = std::io::copy(&mut s_read, &mut c_write);
            let _ = c_write.shutdown(std::net::Shutdown::Write);
            let _ = t1.join();
        }
    });

    let main = format!(
        r#"-- | CookieJar + Redirect client test and live Cinch-to-Cinch loopback roundtrip.
module Main where
  import Data.Bytes
  import Data.CookieJar
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Concurrent
  import Effect.Http.Server
  import Effect.HttpTest
  import Text.Http

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Verify CookieJar persistence across redirects and subsequent requests, plus redirect limit.
  check-cookie-client :: forall r. using (io :: Io r) => Unit ->{{IO r}} Int
  check-cookie-client u =
    let jar0 = cookie-jar-new None in
    let c0 = http-client-with-time 1700000000 (http-client-with-jar jar0 http-client-default) in
    case http-client-get c0 "http://127.0.0.1:{port}/login" of
      Err _ -> 1
      Ok (r1, c1) ->
        if not (resp-status r1 == 200 && text (resp-body r1) == "me:sid=abc789") then 2
        else case http-client-post c1 "http://127.0.0.1:{port}/update" "text/plain" (string-to-bytes "payload") of
          Err _ -> 3
          Ok (r2, c2) ->
            if not (resp-status r2 == 200 && text (resp-body r2) == "done:sid=abc789; updated=yes") then 4
            else
              let jar-ok = (case http-client-jar c2 of
                Some j -> jar-size j == 2
                None -> False) in
              if not jar-ok then 5
              else
                let c3 = http-client-with-max-redirects 2 c2 in
                case http-client-get c3 "http://127.0.0.1:{port}/loop" of
                  Err (ClientTooManyRedirects 2) -> 0
                  _ -> 6

  -- | Live Cinch server and Cinch client talking over OS loopback TCP sockets concurrently.
  check-cinch-to-cinch :: forall r. using (io :: Io r) => Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  check-cinch-to-cinch u =
    let res = withScope (\sc -> do
      s-task <- spawn sc (\() ->
        let h = http-handler-pure (\req ->
          let q = case req-query-param req "who" of Some v -> v; None -> "" in
          http-response-text status-ok (string-concat "hello-loopback:" q)) in
        case httptest-serve-loopback "127.0.0.1" {srv_port} h of
          Ok 1 -> Ok 1
          _ -> Err 10)
      c-task <- spawn sc (\() -> case http-client-get http-client-default "http://127.0.0.1:{cli_port}/hi?who=cinch" of
        Ok (resp, _) -> if resp-status resp == 200 && text (resp-body resp) == "hello-loopback:cinch" then Ok 1 else Err 11
        Err _ -> Err 12)
      joinAllOrCancel sc s-task c-task) in
    case res of
      Ok (1, 1) -> 0
      Err code -> code
      _ -> 13

  -- | Run both integration checks.
  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in
    let a = check-cookie-client () in
    let b = check-cinch-to-cinch () in
    if a + b == 0 then 200 else a + b
"#
    );

    let dir = common::project("http_cookie_client", &http_modules(&main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "cookie-jar client / Cinch-to-Cinch loopback test failed with code {code}");
}

/// Exercise Effect.HttpTest in-memory helpers (httptest-record, httptest-record-mux,
/// httptest-record-writer), pipelined http-serve-bytes, and Text.Http cookie helpers.
#[test]
fn http_test_recorders_cookies_and_pipelining() {
    let main = r#"-- | In-memory Effect.HttpTest helpers, pipelining, and HTTP cookie helpers.
module Main where
  import Data.Bytes
  import Data.List
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http.Server
  import Effect.HttpTest
  import Text.Cookie
  import Text.Http

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Test httptest-record-mux with path values, 405, and Set-Cookie/Cookie helpers.
  check-record-mux :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-record-mux u =
    let h = http-handler-pure (\req ->
      let id = case req-path-value req "id" of Some v -> v; None -> "" in
      let sid = case req-cookie req "sid" of Some c -> cookie-value c; None -> "" in
      let out-cookie = cookie-new "seen" (string-concat-all [id, ":", sid]) in
      resp-add-cookie (http-response-text status-ok (string-concat-all [id, "|", sid])) out-cookie) in
    case mux-handle "GET /users/{id}" h mux-empty of
      Err _ -> 1
      Ok mux ->
        let req0 = httptest-new-request "GET" "/users/alice" (bytes-new 0) in
        let req1 = req-add-cookie req0 (cookie-new "sid" "tok42") in
        let rec = httptest-record-mux mux req1 in
        let res = recorder-result rec in
        let ok1 = resp-status res == 200 && recorder-body-string rec == "alice|tok42" in
        let ok2 = (case resp-cookies res of
          Cons c Nil -> cookie-name c == "seen" && cookie-value c == "alice:tok42"
          _ -> False) in
        let rec405 = httptest-record-mux mux (httptest-new-request "DELETE" "/users/alice" (bytes-new 0)) in
        let res405 = recorder-result rec405 in
        let ok3 = resp-status res405 == 405 && (case resp-header res405 "Allow" of Some v -> v == "GET, HEAD"; None -> False) in
        if ok1 && ok2 && ok3 then 0 else 2

  -- | Test pure httptest-record-writer (no IO effects).
  check-record-writer :: Int
  check-record-writer =
    let req = httptest-new-request "POST" "/note" (string-to-bytes "hello") in
    let rec = httptest-record-writer (\q w ->
      let w1 = recorder-set-header w "X-Mode" "pure" in
      let w2 = recorder-write-header w1 status-accepted in
      let w3 = recorder-write w2 (req-body q) in
      recorder-flush w3) req in
    let res = recorder-result rec in
    let ok = (resp-status res == 202
      && recorder-body-string rec == "hello"
      && recorder-flushed rec
      && (case resp-header res "X-Mode" of Some v -> v == "pure"; None -> False)) in
    if ok then 0 else 3

  -- | Test pipelined multi-request http-serve-bytes with keep-alive and Connection: close.
  check-pipelined-bytes :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-pipelined-bytes u =
    let h = http-handler-pure (\req -> http-response-text status-ok (req-path req)) in
    let wire = string-to-bytes "GET /one HTTP/1.1\r\nHost: x\r\n\r\nGET /two HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n" in
    case http-serve-bytes http-server-config-default wire h of
      Err _ -> 4
      Ok (out, count) ->
        if not (count == 2) then 5
        else case http-parse-response out of
          Err _ -> 6
          Ok (r1, used1) ->
            let rem = bytes-new (if used1 <= bytes-len out then bytes-len out - used1 else 0) in
            let cp = if used1 + bytes-len rem <= bytes-len out then bytes-copy-range rem out 0 used1 (bytes-len rem) else () in
            case http-parse-response rem of
              Err _ -> 7
              Ok (r2, _) ->
                if text (resp-body r1) == "/one" && text (resp-body r2) == "/two" then 0 else 8

  -- | Run all in-memory checks.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let a = check-record-mux () in
    let b = check-record-writer in
    let c = check-pipelined-bytes () in
    if a + b + c == 0 then 200 else a + b + c
"#;

    let dir = common::project("http_recorders", &http_modules(main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "recorders/cookies/pipelining test failed with code {code}");
}

/// Cinch helpers compiled into Main for Go reference vector verification.
const VECTOR_HELPERS: &str = r#"-- | Reference vector consumer for Text.Http, Effect.Http.Server, and Effect.HttpTest.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http.Server
  import Effect.HttpTest
  import Text.Http

  -- | Convert bytes to lowercase hex string.
  to-hex :: Bytes -> String
  to-hex b = to-hex-loop b 0 Nil

  -- | Accumulate two hex chars per byte.
  to-hex-loop :: Bytes -> Nat -> List String -> String
  to-hex-loop b i acc =
    if i >= bytes-len b then string-concat-all (reverse acc)
    else
      let v = bytes-get b i in
      let hi = hex-nib (nat-div v 16) in
      let lo = hex-nib (nat-rem v 16) in
      to-hex-loop b (i + 1) (Cons lo (Cons hi acc))

  -- | Single lowercase hex nibble.
  hex-nib :: Nat -> String
  hex-nib d =
    let c = if d < 10 then 48 + d else 87 + d in
    let out = bytes-new 1 in
    let w = if c < 256 then bytes-set out 0 c else () in
    case string-from-bytes out of Some s -> s; None -> ""

  -- | Check one status-text row.
  check-status :: Nat -> String -> Bool
  check-status code want = http-status-text code == want

  -- | Check one canonical-header-key row.
  check-canon :: String -> String -> Bool
  check-canon inp want = http-canonical-header-key inp == want

  -- | Register a list of patterns in sequence into a ServeMux.
  register-all :: List String -> ServeMux String -> Result HttpProtoError (ServeMux String)
  register-all pats mux = case pats of
    Nil -> Ok mux
    Cons p rest -> case mux-handle p p mux of
      Err e -> Err e
      Ok next -> register-all rest next

  -- | Check one pattern registration/conflict row.
  check-reg :: List String -> Bool -> Bool
  check-reg pats want-ok = case register-all pats mux-empty of
    Ok _ -> want-ok
    Err _ -> not want-ok

  -- | Sort (String, String) pairs by key, preserving value order for equal keys.
  sort-pairs :: List (String, String) -> List (String, String)
  sort-pairs xs = sort-pairs-acc xs Nil

  -- | Insertion sort for string pairs.
  sort-pairs-acc :: List (String, String) -> List (String, String) -> List (String, String)
  sort-pairs-acc xs acc = case xs of
    Nil -> acc
    Cons p rest -> sort-pairs-acc rest (insert-pair p acc)

  -- | Insert one pair in lexicographical key order (stable for equal keys).
  insert-pair :: (String, String) -> List (String, String) -> List (String, String)
  insert-pair p ys = case ys of
    Nil -> Cons p Nil
    Cons q rest -> case (p, q) of
      ((pk, pv), (qk, qv)) ->
        let cmp = bytes-compare (string-to-bytes pk) (string-to-bytes qk) in
        if lt-i64 cmp 0 then Cons p ys
        else Cons q (insert-pair p rest)

  -- | Structural equality for lists of (String, String) pairs.
  pairs-eq :: List (String, String) -> List (String, String) -> Bool
  pairs-eq a b = case a of
    Nil -> case b of Nil -> True; Cons _ _ -> False
    Cons pa ra -> case b of
      Nil -> False
      Cons pb rb -> case (pa, pb) of
        ((ak, av), (bk, bv)) -> ak == bk && av == bv && pairs-eq ra rb

  -- | Drop pairs with empty values (matching Go r.PathValue(name) != "" vector projection).
  nonempty-pairs :: List (String, String) -> List (String, String)
  nonempty-pairs ps = case ps of
    Nil -> Nil
    Cons p rest -> case p of
      (_, v) -> if v == "" then nonempty-pairs rest else Cons p (nonempty-pairs rest)

  -- | Check one ServeMux routing query row.
  check-mux :: List String -> String -> String -> String -> Nat -> String -> String -> String -> List (String, String) -> Bool
  check-mux routes method host raw-uri want-status want-handler want-loc want-allow want-pvs =
    case register-all routes mux-empty of
      Err _ -> False
      Ok mux -> case mux-match mux method host raw-uri of
        MuxMatched h pat params ->
          want-status == 200 && h == want-handler && pat == want-handler && want-loc == "" && want-allow == "" && pairs-eq (sort-pairs (nonempty-pairs params)) want-pvs
        MuxRedirect loc code ->
          want-status == code && want-handler == "" && loc == want-loc && want-allow == ""
        MuxMethodNotAllowed methods ->
          want-status == 405 && want-handler == "" && want-loc == "" && mux-allow-header methods == want-allow
        MuxNotFound ->
          want-status == 404 && want-handler == "" && want-loc == "" && want-allow == ""

  -- | Check every expected key/value exists in a lookup function.
  check-all-params :: (String -> Option String) -> List (String, String) -> Bool
  check-all-params lookup expected = case expected of
    Nil -> True
    Cons pair rest -> case pair of
      (k, v) -> case lookup k of
        Some got -> got == v && check-all-params lookup rest
        None -> False

  -- | Check a valid wire HTTP/1.1 request vector.
  check-wire-req :: String -> String -> String -> String -> String -> String -> Nat -> Nat -> List (String, String) -> String -> Bool -> Int -> List (String, String) -> List (String, String) -> Bool
  check-wire-req raw m uri path rq host maj min hdrs body-hex clz cl qp fp =
    case http-parse-request (string-to-bytes raw) of
      Err _ -> False
      Ok (req, used) ->
        used == bytes-len (string-to-bytes raw)
        && req-method req == m
        && req-request-uri req == uri
        && req-path req == path
        && req-raw-query req == rq
        && req-host req == host
        && req-proto-major req == maj
        && req-proto-minor req == min
        && pairs-eq (sort-pairs (http-header-to-pairs (req-headers req))) hdrs
        && to-hex (req-body req) == body-hex
        && req-close req == clz
        && req-content-length req == cl
        && check-all-params (\k -> req-query-param req k) qp
        && check-all-params (\k -> req-form-value req k) fp

  -- | Check an invalid wire HTTP/1.1 request vector.
  check-wire-req-bad :: String -> Bool
  check-wire-req-bad raw = case http-parse-request (string-to-bytes raw) of
    Ok _ -> False
    Err e -> not (http-proto-error-message e == "")

  -- | Check a valid wire HTTP/1.1 response vector.
  check-wire-resp :: String -> String -> Nat -> Nat -> Nat -> List (String, String) -> String -> Bool
  check-wire-resp req-m raw status maj min hdrs body-hex =
    case http-parse-response-for req-m (string-to-bytes raw) of
      Err _ -> False
      Ok (resp, used) ->
        used == bytes-len (string-to-bytes raw)
        && resp-status resp == status
        && resp-proto-major resp == maj
        && resp-proto-minor resp == min
        && pairs-eq (sort-pairs (http-header-to-pairs (resp-headers resp))) hdrs
        && to-hex (resp-body resp) == body-hex

  -- | Check an invalid wire HTTP/1.1 response vector.
  check-wire-resp-bad :: String -> String -> Bool
  check-wire-resp-bad req-m raw = case http-parse-response-for req-m (string-to-bytes raw) of
    Ok _ -> False
    Err e -> not (http-proto-error-message e == "")

  -- | Run the named ResponseRecorder scenario matching Go httptest.
  run-recorder-scenario :: String -> ResponseRecorder
  run-recorder-scenario name =
    if name == "empty-default" then
      httptest-new-recorder
    else if name == "implicit-200-and-snapshot" then
      let r0 = recorder-set-header httptest-new-recorder "X-Before" "1" in
      let r1 = recorder-write-string r0 "hello world" in
      let r2 = recorder-set-header r1 "X-After" "2" in
      recorder-write-header r2 status-not-found
    else if name == "created-json-flushed" then
      let r0 = recorder-set-header httptest-new-recorder "Content-Type" "application/json" in
      let r1 = recorder-write-header r0 status-created in
      let r2 = recorder-write-string r1 "{\"ok\":true}" in
      recorder-flush r2
    else
      let r0 = recorder-add-header httptest-new-recorder "X-Multi" "a" in
      let r1 = recorder-add-header r0 "X-Multi" "b" in
      recorder-flush r1

  -- | Check one ResponseRecorder reference row.
  check-recorder :: String -> Nat -> String -> String -> List (String, String) -> Bool -> Bool
  check-recorder name want-status want-body want-ct want-hdrs want-flushed =
    let rec = run-recorder-scenario name in
    let res = recorder-result rec in
    let got-ct = case resp-header res "Content-Type" of Some v -> v; None -> "" in
    resp-status res == want-status
    && recorder-body-string rec == want-body
    && got-ct == want-ct
    && pairs-eq (sort-pairs (http-header-to-pairs (resp-headers res))) want-hdrs
    && recorder-flushed rec == want-flushed

  -- | Check one httptest-new-request reference row.
  check-new-req :: String -> String -> String -> String -> String -> String -> String -> String -> Nat -> Nat -> String -> Bool
  check-new-req m raw-uri body want-m want-host want-path want-q want-uri want-maj want-min want-ra =
    let req = httptest-new-request m raw-uri (string-to-bytes body) in
    req-method req == want-m
    && req-host req == want-host
    && req-path req == want-path
    && req-raw-query req == want-q
    && req-request-uri req == want-uri
    && req-proto-major req == want-maj
    && req-proto-minor req == want-min
    && req-remote-addr req == want-ra

  -- | Check chunked encode/decode roundtrip including trailer parsing.
  check-chunk-roundtrip :: Bool
  check-chunk-roundtrip =
    let payload = string-to-bytes "The quick brown fox jumps over the lazy dog!" in
    let encoded = http-chunk-encode payload 7 in
    let ok1 = (case http-chunk-decode encoded 0 of
      Ok (decoded, _, end-pos) -> end-pos == bytes-len encoded && eq-i64 (bytes-compare decoded payload) 0
      Err _ -> False) in
    let with-trailer = string-to-bytes "4\r\nData\r\n0\r\nX-Checksum: abcd\r\n\r\n" in
    let ok2 = (case http-chunk-decode with-trailer 0 of
      Ok (decoded, tr, end-pos) ->
        end-pos == bytes-len with-trailer
        && (case string-from-bytes decoded of Some s -> s == "Data"; None -> False)
        && (case http-header-get tr "x-checksum" of Some v -> v == "abcd"; None -> False)
      Err _ -> False) in
    ok1 && ok2

  -- | Check HTTP/1.1 request and response render + parse roundtrips (Content-Length & chunked).
  check-wire-roundtrip :: Bool
  check-wire-roundtrip =
    let req0 = req-with-header (req-with-host (http-new-request "POST" "/api/v1?a=1" (string-to-bytes "body-data")) "api.test") "X-Test" "yes" in
    let r1 = http-render-request req0 in
    let ok-req = (case http-parse-request r1 of
      Ok (p, _) -> req-method p == "POST" && req-host p == "api.test" && req-path p == "/api/v1" && eq-i64 (bytes-compare (req-body p) (string-to-bytes "body-data")) 0
      Err _ -> False) in
    let r2 = http-render-request-chunked req0 4 in
    let ok-req-chunk = (case http-parse-request r2 of
      Ok (p, _) -> req-content-length p == (0 - 1) && eq-i64 (bytes-compare (req-body p) (string-to-bytes "body-data")) 0
      Err _ -> False) in
    let resp0 = resp-with-header (http-response-text status-created "created-ok") "X-Id" "99" in
    let w1 = http-render-response resp0 in
    let ok-resp = (case http-parse-response w1 of
      Ok (p, _) -> resp-status p == 201 && eq-i64 (bytes-compare (resp-body p) (string-to-bytes "created-ok")) 0
      Err _ -> False) in
    let w2 = http-render-response-chunked resp0 3 in
    let ok-resp-chunk = (case http-parse-response w2 of
      Ok (p, _) -> resp-status p == 201 && eq-i64 (bytes-compare (resp-body p) (string-to-bytes "created-ok")) 0
      Err _ -> False) in
    ok-req && ok-req-chunk && ok-resp && ok-resp-chunk

  -- | Check conflicting Content-Length headers are rejected cleanly.
  check-conflicting-cl :: Bool
  check-conflicting-cl =
    let raw = string-to-bytes "POST /x HTTP/1.1\r\nHost: a\r\nContent-Length: 4\r\nContent-Length: 5\r\n\r\n12345" in
    case http-parse-request raw of
      Err HttpProtoConflictingContentLength -> True
      _ -> False

  -- | Return the index of the first failing row, or 200 when all pass.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i
"#;
