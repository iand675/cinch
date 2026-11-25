//! Comprehensive verification suite for `Text.Rpc` and `Effect.Rpc`:
//! 1. Canonical module-owned documentation on `Text.Rpc` and `Effect.Rpc`.
//! 2. Pure codec and framing tests: JSON-RPC 1.0 & 2.0 (single, notifications, id:null,
//!    auto-detection, all standard error codes, custom error data, batch arrays, empty batch,
//!    all-notification batch), 4-byte BE length-prefixed framing (multi-frame, incomplete,
//!    size limits), gob-RPC framing & typed value/error round-trips, server registry
//!    introspection, and `rpc-http-handler` (200 JSON, 204 notification, 405 non-POST).
//! 3. Live loopback TCP round-trips with `CINCH_LEAK_CHECK=1` (`leaked=0`):
//!    - Cinch JSON-RPC 2.0 + 1.0 server (`rpc-serve-jsonrpc-listener`) <-> Cinch clients
//!      (`rpc-dial-jsonrpc`, `rpc-dial-jsonrpc1`, `rpc-call-json`, `rpc-notify-json`,
//!      `rpc-call-json-batch`, `rpc-close`).
//!    - Independent Rust TCP client <-> Cinch JSON-RPC server verifying exact wire JSON for
//!      JSON-RPC 2.0 single/notification/batch/parse-error and JSON-RPC 1.0 calls/errors.
//!    - Cinch gob-RPC server (`rpc-serve-gob-listener`) <-> Cinch gob-RPC client
//!      (`rpc-dial`, `rpc-call-gob` with structs, slices, maps, errors with data, `rpc-close`).
mod common;

use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

/// All non-embedded source-distributed prelude modules transitively required by `Text.Rpc` and `Effect.Rpc`.
fn rpc_modules<'a>(main: &'a str) -> Vec<(&'a str, &'a str)> {
    vec![
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
        ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
        ("Text.Pem.cinch", include_str!("../prelude/Text/Pem.cinch")),
        ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
        ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
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
        ("Effect.Http.cinch", include_str!("../prelude/Effect/Http.cinch")),
        ("Text.Http.cinch", include_str!("../prelude/Text/Http.cinch")),
        ("Effect.Http.Server.cinch", include_str!("../prelude/Effect/Http/Server.cinch")),
        ("Text.Gob.cinch", include_str!("../prelude/Text/Gob.cinch")),
        ("Text.Rpc.cinch", include_str!("../prelude/Text/Rpc.cinch")),
        ("Effect.Rpc.cinch", include_str!("../prelude/Effect/Rpc.cinch")),
        ("Main.cinch", main),
    ]
}

/// Every declaration in `Text.Rpc` and `Effect.Rpc` carries canonical documentation.
#[test]
fn rpc_modules_have_canonical_documentation() {
    common::assert_documented("Text.Rpc", include_str!("../prelude/Text/Rpc.cinch"));
    common::assert_documented("Effect.Rpc", include_str!("../prelude/Effect/Rpc.cinch"));
}

/// Multi-mode Cinch harness exercising codecs, framing, HTTP JSON-RPC handler, and live loopback TCP.
const RPC_HARNESS: &str = r#"-- | End-to-end verification harness for Text.Rpc and Effect.Rpc.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Int
  import Data.Json
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Http.Server
  import Effect.Net
  import Effect.Process
  import Effect.Rpc
  import Effect.Stream (writer-from-fd, stream-write-all)
  import Text.Gob
  import Text.Http
  import Text.Json (json-eq)
  import Text.Rpc

  -- | Parse a decimal ASCII string into a Nat.
  parse-nat-str :: String -> Nat
  parse-nat-str s = parse-nat-loop (string-to-bytes s) 0 0

  -- | Accumulate decimal digits into a Nat.
  parse-nat-loop :: Bytes -> Nat -> Nat -> Nat
  parse-nat-loop b i acc =
    if i >= bytes-len b then acc
    else
      let c = bytes-get b i in
      if c >= 48 && c <= 57 then parse-nat-loop b (i + 1) (acc * 10 + (c - 48))
      else acc

  -- | Return the 1-based index of the first False check, or 200 when all pass.
  first-fail :: List Bool -> Int -> Int
  first-fail xs idx = case xs of
    Nil -> 200
    Cons ok rest -> if ok then first-fail rest (idx + 1) else idx

  -- | Look up a named member in a Json object spine.
  lookup-jobj :: String -> Json -> Option Json
  lookup-jobj key obj = case obj of
    JObj k v rest -> if k == key then Some v else lookup-jobj key rest
    _ -> None

  -- | JSON-RPC add handler accepting [a, b] or {"a": a, "b": b}.
  add-json-handler :: Json -> Result RpcError Json
  add-json-handler params = case params of
    JArr (JNum a) (JArr (JNum b) JArrEnd) -> Ok (JNum (a + b))
    JObj _ _ _ ->
      let oa = lookup-jobj "a" params in
      let ob = lookup-jobj "b" params in
      case (oa, ob) of
        (Some (JNum a), Some (JNum b)) -> Ok (JNum (a + b))
        _ -> Err (rpc-invalid-params "expected integer fields a and b")
    _ -> Err (rpc-invalid-params "expected [a, b] or {a, b}")

  -- | JSON-RPC subtract handler accepting [a, b].
  sub-json-handler :: Json -> Result RpcError Json
  sub-json-handler params = case params of
    JArr (JNum a) (JArr (JNum b) JArrEnd) -> Ok (JNum (a - b))
    _ -> Err (rpc-invalid-params "expected [a, b]")

  -- | JSON-RPC error handler returning a custom error with structured detail data.
  fail-json-handler :: Json -> Result RpcError Json
  fail-json-handler _ =
    Err (rpc-error-with-data (wrapping-neg-i64 32001) "custom failure" (JObj "retry" (JBool True) JObjEnd))

  -- | Gob-RPC multiply handler reading int fields A and B from a struct.
  mul-gob-handler :: GobValue -> Result RpcError GobValue
  mul-gob-handler arg = case gob-field-int "A" arg of
    Err _ -> Err (rpc-invalid-params "missing int field A")
    Ok a -> case gob-field-int "B" arg of
      Err _ -> Err (rpc-invalid-params "missing int field B")
      Ok b ->
        Ok (gob-struct "MulReply" [
          ("Product", GobInt (wrapping-mul-i64 a b)),
          ("Label", GobString "ok")])

  -- | Gob-RPC echo handler returning its input value unchanged.
  echo-gob-handler :: GobValue -> Result RpcError GobValue
  echo-gob-handler arg = Ok arg

  -- | Gob-RPC failure handler returning an RpcError with JSON data.
  fail-gob-handler :: GobValue -> Result RpcError GobValue
  fail-gob-handler _ =
    Err (rpc-error-with-data (wrapping-neg-i64 32050) "gob boom" (JArr (JStr "detail") JArrEnd))

  -- | Construct the shared test RpcServer with JSON and gob handlers.
  make-server :: forall r. RpcServer r
  make-server =
    let s0 = rpc-server-new in
    let s1 = rpc-register-pure "Arith.Add" add-json-handler s0 in
    let s2 = rpc-register-pure "Arith.Sub" sub-json-handler s1 in
    let s3 = rpc-register-pure "Arith.Fail" fail-json-handler s2 in
    let s4 = rpc-register "Ping" (\p -> Ok (JObj "pong" p JObjEnd)) s3 in
    let s5 = rpc-register-gob-pure "Arith.Mul" mul-gob-handler s4 in
    let s6 = rpc-register-gob-pure "Echo" echo-gob-handler s5 in
    rpc-register-gob-pure "FailGob" fail-gob-handler s6

  -- | Verify pure codecs, batching, length-prefixed framing, gob-RPC, and HTTP JSON-RPC handler.
  run-unit-tests :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  run-unit-tests u =
    let srv = make-server in

    let req2 = rpc-request "Arith.Add" (JArr (JNum 10) (JArr (JNum 32) JArrEnd)) (RpcIdInt 7) in
    let c1 = (case jsonrpc-decode-request (jsonrpc-encode-request req2) of
      Ok d -> rpc-request-eq d req2
      Err _ -> False) in

    let req2-obj = rpc-request "Arith.Add" (JObj "a" (JNum 4) (JObj "b" (JNum 5) JObjEnd)) (RpcIdString "req-a") in
    let c2 = (case jsonrpc-decode-request (jsonrpc-encode-request req2-obj) of
      Ok d -> rpc-request-eq d req2-obj
      Err _ -> False) in

    let notif2 = rpc-notification "Ping" JNull in
    let c3 = (case jsonrpc-decode-request (jsonrpc-encode-request notif2) of
      Ok d -> rpc-request-is-notification d && rpc-request-eq d notif2
      Err _ -> False) in

    let null-id-bytes = string-to-bytes "{\"jsonrpc\":\"2.0\",\"method\":\"Ping\",\"id\":null}\n" in
    let c4 = (case jsonrpc-decode-request null-id-bytes of
      Ok d -> not (rpc-request-is-notification d) && (case rpc-request-id d of Some RpcIdNull -> True; _ -> False)
      Err _ -> False) in

    let req1 = rpc-request-v1 "Arith.Add" (JArr (JNum 3) (JArr (JNum 9) JArrEnd)) (RpcIdInt 1) in
    let c5 = (case jsonrpc-decode-request (jsonrpc-encode-request req1) of
      Ok d -> rpc-request-eq d req1 && rpc-codec-eq (rpc-request-codec d) RpcCodecJson1
      Err _ -> False) in

    let notif1-bytes = string-to-bytes "{\"method\":\"Ping\",\"params\":[],\"id\":null}\n" in
    let c6 = (case jsonrpc-decode-request notif1-bytes of
      Ok d -> rpc-request-is-notification d && rpc-codec-eq (rpc-request-codec d) RpcCodecJson1
      Err _ -> False) in

    let c7 = (
      (case jsonrpc-decode-request (string-to-bytes "{bad") of
        Err e -> eq-i64 (rpc-error-code e) rpc-err-parse-error; _ -> False)
      && (case jsonrpc-decode-request (string-to-bytes "123") of
        Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; _ -> False)
      && (case jsonrpc-decode-request (string-to-bytes "{\"jsonrpc\":\"1.0\",\"method\":\"x\",\"id\":1}") of
        Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; _ -> False)
      && (case jsonrpc-decode-request (string-to-bytes "{\"jsonrpc\":\"2.0\",\"method\":\"x\",\"params\":42,\"id\":1}") of
        Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; _ -> False)
      && (case jsonrpc-decode-request (string-to-bytes "{\"jsonrpc\":\"2.0\",\"method\":\"x\",\"id\":true}") of
        Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; _ -> False)) in

    let resp-ok = rpc-response-ok (RpcIdInt 5) (JNum 99) in
    let resp-err = rpc-response-err (RpcIdString "e1") (rpc-error-with-data rpc-err-invalid-params "bad" (JStr "x")) in
    let c8 = (
      (case jsonrpc-decode-response (jsonrpc-encode-response resp-ok) of
        Ok d -> rpc-response-eq d resp-ok; Err _ -> False)
      && (case jsonrpc-decode-response (jsonrpc-encode-response resp-err) of
        Ok d -> rpc-response-eq d resp-err; Err _ -> False)
      && (case jsonrpc-decode-response (jsonrpc-encode-response-with RpcCodecJson1 resp-ok) of
        Ok d -> rpc-response-eq d resp-ok; Err _ -> False)
      && (case jsonrpc-decode-response (string-to-bytes "{\"id\":3,\"result\":null,\"error\":\"boom\"}\n") of
        Ok d -> (case rpc-response-result d of Err e -> rpc-error-message e == "boom"; Ok _ -> False)
        Err _ -> False)) in

    let batch-wire = string-to-bytes "[{\"jsonrpc\":\"2.0\",\"method\":\"Arith.Add\",\"params\":[1,2],\"id\":1},{\"jsonrpc\":\"2.0\",\"method\":\"Ping\",\"params\":[]},{\"foo\":\"boo\"},{\"jsonrpc\":\"2.0\",\"method\":\"NoSuch\",\"id\":\"2\"},{\"jsonrpc\":\"2.0\",\"method\":\"Arith.Sub\",\"params\":[10,4],\"id\":3}]\n" in
    let c9 = (case rpc-dispatch-json-bytes srv batch-wire of
      Some reply -> case jsonrpc-decode-batch-responses reply of
        Ok [r1, r2, r3, r4] ->
          rpc-response-eq r1 (rpc-response-ok (RpcIdInt 1) (JNum 3))
          && (case rpc-response-result r2 of Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; Ok _ -> False)
          && rpc-id-eq (rpc-response-id r2) RpcIdNull
          && (case rpc-response-result r3 of Err e -> eq-i64 (rpc-error-code e) rpc-err-method-not-found; Ok _ -> False)
          && rpc-id-eq (rpc-response-id r3) (RpcIdString "2")
          && rpc-response-eq r4 (rpc-response-ok (RpcIdInt 3) (JNum 6))
        _ -> False
      None -> False) in

    let c10 = (
      (case rpc-dispatch-json-bytes srv (string-to-bytes "[]") of
        Some reply -> case jsonrpc-decode-response reply of
          Ok r -> rpc-id-eq (rpc-response-id r) RpcIdNull && (case rpc-response-result r of Err e -> eq-i64 (rpc-error-code e) rpc-err-invalid-request; _ -> False)
          Err _ -> False
        None -> False)
      && (case rpc-dispatch-json-bytes srv (string-to-bytes "[{\"jsonrpc\":\"2.0\",\"method\":\"Ping\"},{\"jsonrpc\":\"2.0\",\"method\":\"Missing\"}]") of
        None -> True
        Some _ -> False)) in

    let c11 = (
      case rpc-frame-encode 1024 (string-to-bytes "hello") of
        Err _ -> False
        Ok f1 -> case rpc-frame-encode 1024 (string-to-bytes "world!") of
          Err _ -> False
          Ok f2 ->
            let stream = bytes-concat [f1, f2] in
            let ok-multi = (case rpc-frame-decode 1024 stream of
              Ok (p1, rem1) ->
                eq-i64 (bytes-compare p1 (string-to-bytes "hello")) 0
                && (case rpc-frame-decode 1024 rem1 of
                  Ok (p2, rem2) -> eq-i64 (bytes-compare p2 (string-to-bytes "world!")) 0 && bytes-len rem2 == 0
                  Err _ -> False)
              Err _ -> False) in
            let ok-inc1 = (case rpc-frame-decode 1024 (bytes-from-list [0, 0]) of Err RpcFrameIncomplete -> True; _ -> False) in
            let ok-inc2 = (case rpc-frame-decode 1024 (bytes-from-list [0, 0, 0, 10, 1, 2]) of Err RpcFrameIncomplete -> True; _ -> False) in
            let ok-big-enc = (case rpc-frame-encode 4 (string-to-bytes "12345") of Err (RpcFrameTooLarge 5) -> True; _ -> False) in
            let ok-big-dec = (case rpc-frame-decode 4 f1 of Err (RpcFrameTooLarge 5) -> True; _ -> False) in
            ok-multi && ok-inc1 && ok-inc2 && ok-big-enc && ok-big-dec) in

    let gob-arg = gob-struct "MulArgs" [("A", GobInt 6), ("B", GobInt 7)] in
    let greq = GobRpcRequest "Arith.Mul" 42 gob-arg in
    let c12 = (
      case gobrpc-encode-request greq of
        Err _ -> False
        Ok wire -> case gobrpc-decode-request rpc-default-max-frame-bytes wire of
          Err _ -> False
          Ok (dreq, rem) ->
            bytes-len rem == 0
            && gobrpc-request-method dreq == "Arith.Mul"
            && eq-i64 (gobrpc-request-seq dreq) 42
            && gob-value-eq (gobrpc-request-body dreq) gob-arg
            && (let gresp = rpc-dispatch-gob srv dreq in
                case gobrpc-encode-response gresp of
                  Err _ -> False
                  Ok rwire -> case gobrpc-decode-response rpc-default-max-frame-bytes rwire of
                    Err _ -> False
                    Ok (dresp, rrem) ->
                      bytes-len rrem == 0
                      && eq-i64 (gobrpc-response-seq dresp) 42
                      && (case gobrpc-response-result dresp of
                        Ok v -> case gob-field-int "Product" v of Ok p -> eq-i64 p 42; Err _ -> False
                        Err _ -> False))) in

    let fail-req = GobRpcRequest "FailGob" 9 (GobString "payload") in
    let fail-resp = rpc-dispatch-gob srv fail-req in
    let c13 = (case gobrpc-encode-response fail-resp of
      Err _ -> False
      Ok wire -> case gobrpc-decode-response rpc-default-max-frame-bytes wire of
        Err _ -> False
        Ok (dresp, _) -> case gobrpc-response-result dresp of
          Err e ->
            eq-i64 (rpc-error-code e) (wrapping-neg-i64 32050)
            && rpc-error-message e == "gob boom"
            && (case rpc-error-data e of Some d -> json-eq d (JArr (JStr "detail") JArrEnd); None -> False)
          Ok _ -> False) in

    let c14 = (rpc-has-method srv "Arith.Add"
      && rpc-has-method srv "Arith.Mul"
      && not (rpc-has-method srv "Unknown")
      && list-length (rpc-methods srv) == 7) in

    let hdl = rpc-http-handler srv in
    let call-body = jsonrpc-encode-request (rpc-request "Arith.Add" (JArr (JNum 20) (JArr (JNum 22) JArrEnd)) (RpcIdInt 1)) in
    let http-call-wire = http-render-request (req-with-header (http-new-request method-post "/rpc" call-body) "Connection" "close") in
    let notif-body = jsonrpc-encode-request (rpc-notification "Ping" JNull) in
    let http-notif-wire = http-render-request (req-with-header (http-new-request method-post "/rpc" notif-body) "Connection" "close") in
    let get-req = string-to-bytes "GET /rpc HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n" in
    let c15 = (
      (case http-serve-bytes http-server-config-default http-call-wire hdl of
        Ok (out, 1) -> case http-parse-response out of
          Ok (resp, _) ->
            resp-status resp == 200
            && (case resp-header resp "Content-Type" of Some ct -> ct == "application/json"; None -> False)
            && (case jsonrpc-decode-response (resp-body resp) of
              Ok rr -> rpc-response-eq rr (rpc-response-ok (RpcIdInt 1) (JNum 42))
              Err _ -> False)
          Err _ -> False
        _ -> False)
      && (case http-serve-bytes http-server-config-default http-notif-wire hdl of
        Ok (out, 1) -> case http-parse-response out of
          Ok (resp, _) -> resp-status resp == 204 && bytes-len (resp-body resp) == 0
          Err _ -> False
        _ -> False)
      && (case http-serve-bytes http-server-config-default get-req hdl of
        Ok (out, 1) -> case http-parse-response out of
          Ok (resp, _) ->
            resp-status resp == 405
            && (case resp-header resp "Allow" of Some a -> a == "POST"; None -> False)
          Err _ -> False
        _ -> False)) in

    first-fail [c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15] 1

  -- | Listen on 127.0.0.1:0, write the bound address to stdout, and serve 2 JSON-RPC connections.
  run-jsonrpc-server :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  run-jsonrpc-server u =
    let srv = make-server in
    case net-listen "127.0.0.1" 0 of
      Err _ -> 10
      Ok listener -> case net-listener-addr listener of
        Err _ -> let c = net-listener-close listener in 11
        Ok addr ->
          let p = stream-write-all (writer-from-fd 1) (string-to-bytes (string-concat addr "\n")) in
          let res = rpc-serve-jsonrpc-listener srv listener 2 in
          let c = net-listener-close listener in
          case res of
            Ok count -> if count >= 6 then 200 else 12
            Err _ -> 13

  -- | Exercise both JSON-RPC 2.0 and JSON-RPC 1.0 clients against the loopback server port.
  run-jsonrpc-client :: forall r. using (io :: Io r) => Nat ->{IO r} Int
  run-jsonrpc-client port =
    case rpc-dial-jsonrpc "127.0.0.1" port of
      Err _ -> 20
      Ok c0 ->
        let (r1, c1) = rpc-call-json c0 "Arith.Add" (JArr (JNum 19) (JArr (JNum 23) JArrEnd)) in
        let (r2, c2) = rpc-notify-json c1 "Ping" (JArr (JStr "hi") JArrEnd) in
        let (r3, c3) = rpc-call-json c2 "Arith.Add" (JObj "a" (JNum 100) (JObj "b" (JNum 23) JObjEnd)) in
        let (r4, c4) = rpc-call-json c3 "DoesNotExist" JNull in
        let batch = [rpc-request "Arith.Sub" (JArr (JNum 50) (JArr (JNum 8) JArrEnd)) (RpcIdString "b1"), rpc-notification "Ping" JNull, rpc-request "Arith.Fail" JNull (RpcIdInt 99)] in
        let (r5, c5) = rpc-call-json-batch c4 batch in
        let closed2 = rpc-close c5 in
        let ok2 = (
          (case r1 of Ok (JNum n) -> n == 42; _ -> False)
          && (case r2 of Ok () -> True; _ -> False)
          && (case r3 of Ok (JNum n) -> n == 123; _ -> False)
          && (case r4 of Err (RpcRemoteError e) -> eq-i64 (rpc-error-code e) rpc-err-method-not-found; _ -> False)
          && (case r5 of
            Ok [br1, br2] ->
              rpc-response-eq br1 (rpc-response-ok (RpcIdString "b1") (JNum 42))
              && (case rpc-response-result br2 of
                Err e -> eq-i64 (rpc-error-code e) (wrapping-neg-i64 32001) && rpc-error-message e == "custom failure"
                Ok _ -> False)
            _ -> False)) in
        if not ok2 then 21
        else case rpc-dial-jsonrpc1 "127.0.0.1" port of
          Err _ -> 22
          Ok v1c0 ->
            let (v1r1, v1c1) = rpc-call-json v1c0 "Arith.Add" (JArr (JNum 7) (JArr (JNum 8) JArrEnd)) in
            let (v1r2, v1c2) = rpc-notify-json v1c1 "Ping" JArrEnd in
            let closed1 = rpc-close v1c2 in
            case (v1r1, v1r2) of
              (Ok (JNum n), Ok ()) -> if n == 15 then 200 else 23
              _ -> 24

  -- | Listen on 127.0.0.1:0, write the bound address to stdout, and serve 1 gob-RPC connection.
  run-gob-server :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  run-gob-server u =
    let srv = make-server in
    case net-listen "127.0.0.1" 0 of
      Err _ -> 30
      Ok listener -> case net-listener-addr listener of
        Err _ -> let c = net-listener-close listener in 31
        Ok addr ->
          let p = stream-write-all (writer-from-fd 1) (string-to-bytes (string-concat addr "\n")) in
          let res = rpc-serve-gob-listener srv listener 1 in
          let c = net-listener-close listener in
          case res of
            Ok count -> if count == 4 then 200 else 32
            Err _ -> 33

  -- | Exercise the gob-RPC client against the loopback server port.
  run-gob-client :: forall r. using (io :: Io r) => Nat ->{IO r} Int
  run-gob-client port =
    case rpc-dial "127.0.0.1" port of
      Err _ -> 40
      Ok c0 ->
        let arg1 = gob-struct "MulArgs" [("A", GobInt 11), ("B", GobInt 12)] in
        let (r1, c1) = rpc-call-gob c0 "Arith.Mul" arg1 in
        let arg2 = gob-slice [GobString "alpha", GobString "beta"] in
        let (r2, c2) = rpc-call-gob c1 "Echo" arg2 in
        let (r3, c3) = rpc-call-gob c2 "FailGob" (GobBool True) in
        let (r4, c4) = rpc-call-gob c3 "NoMethod" (GobInt 0) in
        let closed = rpc-close c4 in
        let ok = (
          (case r1 of
            Ok v -> case gob-field-int "Product" v of Ok p -> eq-i64 p 132; Err _ -> False
            Err _ -> False)
          && (case r2 of
            Ok v -> gob-value-eq v arg2
            Err _ -> False)
          && (case r3 of
            Err (RpcRemoteError e) ->
              eq-i64 (rpc-error-code e) (wrapping-neg-i64 32050)
              && rpc-error-message e == "gob boom"
              && (case rpc-error-data e of Some d -> json-eq d (JArr (JStr "detail") JArrEnd); None -> False)
            _ -> False)
          && (case r4 of
            Err (RpcRemoteError e) -> eq-i64 (rpc-error-code e) rpc-err-method-not-found
            _ -> False)
          && eq-i64 (rpc-client-seq c4) 5) in
        if ok then 200 else 41

  -- | Dispatch test mode from CLI arguments.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case process-args () of
      Err _ -> 93
      Ok args -> case args of
        Cons mode rest ->
          if mode == "unit" then run-unit-tests ()
          else if mode == "json-server" then run-jsonrpc-server ()
          else if mode == "json-client" then
            (case rest of Cons p _ -> run-jsonrpc-client (parse-nat-str p); Nil -> 90)
          else if mode == "gob-server" then run-gob-server ()
          else if mode == "gob-client" then
            (case rest of Cons p _ -> run-gob-client (parse-nat-str p); Nil -> 91)
          else 92
        Nil -> run-unit-tests ()
"#;

fn assert_no_leaks(label: &str, stderr: &[u8]) {
    let text = String::from_utf8_lossy(stderr);
    assert!(
        text.contains("leaked=0"),
        "{label}: expected leaked=0 in stderr, got: {text}"
    );
}

fn spawn_server_and_read_addr(exe: &std::path::Path, mode: &str) -> (std::process::Child, String) {
    let mut child = Command::new(exe)
        .arg(mode)
        .env("CINCH_LEAK_CHECK", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {mode}: {e}"));
    let stdout = child.stdout.take().expect("server stdout");
    let mut reader = BufReader::new(stdout);
    let mut addr = String::new();
    reader.read_line(&mut addr).expect("read bound addr");
    let addr = addr.trim().to_string();
    assert!(addr.contains(':'), "{mode}: expected host:port, got {addr:?}");
    (child, addr)
}

/// Exercise pure codecs, framing, HTTP JSON-RPC handler, live Cinch-to-Cinch loopback TCP
/// (JSON-RPC 2.0, JSON-RPC 1.0, gob-RPC), and Rust wire-level interop against the Cinch server.
#[test]
fn rpc_codecs_framing_http_and_loopback_roundtrips() {
    let dir = common::project("rpc_suite", &rpc_modules(RPC_HARNESS));
    let exe = common::build_executable(&dir, "Main.cinch", "rpc_harness")
        .expect("build rpc harness");

    // 1. Pure unit tests + HTTP JSON-RPC 2.0 handler
    let unit = Command::new(&exe)
        .arg("unit")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .expect("run unit");
    assert_eq!(unit.status.code(), Some(200), "unit failed: {:?}", unit.status);
    assert_no_leaks("unit", &unit.stderr);

    // 2. Live loopback TCP JSON-RPC 2.0 + 1.0 (Cinch server <-> Cinch clients)
    let (srv, addr) = spawn_server_and_read_addr(&exe, "json-server");
    let port = addr.rsplit(':').next().expect("port");
    let cli = Command::new(&exe)
        .args(["json-client", port])
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .expect("run json-client");
    let srv_out = srv.wait_with_output().expect("wait json-server");
    assert_eq!(cli.status.code(), Some(200), "json-client failed: {:?}", cli.status);
    assert_eq!(srv_out.status.code(), Some(200), "json-server failed: {:?}", srv_out.status);
    assert_no_leaks("json-client", &cli.stderr);
    assert_no_leaks("json-server", &srv_out.stderr);

    // 3. Live loopback TCP gob-RPC (Cinch server <-> Cinch client)
    let (srv, addr) = spawn_server_and_read_addr(&exe, "gob-server");
    let port = addr.rsplit(':').next().expect("port");
    let cli = Command::new(&exe)
        .args(["gob-client", port])
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .expect("run gob-client");
    let srv_out = srv.wait_with_output().expect("wait gob-server");
    assert_eq!(cli.status.code(), Some(200), "gob-client failed: {:?}", cli.status);
    assert_eq!(srv_out.status.code(), Some(200), "gob-server failed: {:?}", srv_out.status);
    assert_no_leaks("gob-client", &cli.stderr);
    assert_no_leaks("gob-server", &srv_out.stderr);

    // 4. Independent Rust TCP peer verifying exact wire JSON-RPC 2.0 and 1.0 framing
    let (srv, addr) = spawn_server_and_read_addr(&exe, "json-server");
    {
        let mut stream = TcpStream::connect(&addr).expect("connect v2");
        let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));

        stream
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"Arith.Add\",\"params\":[15,27],\"id\":1}\n")
            .unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(v["jsonrpc"], "2.0");
        assert_eq!(v["result"], 42);
        assert_eq!(v["id"], 1);

        stream
            .write_all(b"{\"jsonrpc\":\"2.0\",\"method\":\"Ping\",\"params\":[]}\n")
            .unwrap();
        stream
            .write_all(b"[{\"jsonrpc\":\"2.0\",\"method\":\"Arith.Sub\",\"params\":[20,5],\"id\":\"k\"},{\"jsonrpc\":\"2.0\",\"method\":\"Ping\"},{\"bad\":1}]\n")
            .unwrap();
        line.clear();
        reader.read_line(&mut line).unwrap();
        let batch: Value = serde_json::from_str(&line).unwrap();
        let arr = batch.as_array().expect("batch array");
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0]["result"], 15);
        assert_eq!(arr[0]["id"], "k");
        assert!(arr[1]["id"].is_null());
        assert_eq!(arr[1]["error"]["code"], -32600);

        stream.write_all(b"{not valid json\n").unwrap();
        line.clear();
        reader.read_line(&mut line).unwrap();
        let pe: Value = serde_json::from_str(&line).unwrap();
        assert!(pe["id"].is_null());
        assert_eq!(pe["error"]["code"], -32700);
    }
    {
        let mut stream = TcpStream::connect(&addr).expect("connect v1");
        let mut reader = BufReader::new(stream.try_clone().expect("clone stream"));

        stream
            .write_all(b"{\"method\":\"Arith.Add\",\"params\":[3,4],\"id\":7}\n")
            .unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        assert!(v.get("jsonrpc").is_none());
        assert_eq!(v["id"], 7);
        assert_eq!(v["result"], 7);
        assert!(v["error"].is_null());

        stream
            .write_all(b"{\"method\":\"Ping\",\"params\":[],\"id\":null}\n")
            .unwrap();
        stream
            .write_all(b"{\"method\":\"Unknown\",\"params\":[],\"id\":8}\n")
            .unwrap();
        line.clear();
        reader.read_line(&mut line).unwrap();
        let ve: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(ve["id"], 8);
        assert!(ve["result"].is_null());
        assert_eq!(ve["error"]["code"], -32601);
    }
    let srv_out = srv.wait_with_output().expect("wait rust-peer json-server");
    assert_eq!(srv_out.status.code(), Some(200), "rust-peer json-server failed: {:?}", srv_out.status);
    assert_no_leaks("rust-peer json-server", &srv_out.stderr);

    common::cleanup(&dir);
}
