//! The SMTP client against transcripts of the reference SDK's client (`tooling/smtp_vectors.go`): for each
//! scenario a loopback server replays the recorded replies, and the Cinch client must write exactly the bytes
//! the reference client wrote and report the same result for every operation: EHLO and the HELO fallback,
//! extensions, BODY=8BITMIME and SMTPUTF8, PLAIN and CRAM-MD5 exchanges and their refusals, rejected
//! recipients and data, multi-line and malformed replies, lines carrying CR or LF, late Hello, and the body's
//! dot-stuffing and line-end repair. Pure reply parsing and body encoding are checked directly as well.
mod common;

use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;

/// A server that sends the greeting, then one reply per client line (a DATA body counts as one line, ending at
/// "." CR LF), and reports everything the client wrote.
fn replay(replies: Vec<String>) -> (u16, mpsc::Receiver<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut record = Vec::new();
        if let Ok((stream, _)) = listener.accept() {
            let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(20)));
            let mut writer = stream.try_clone().unwrap();
            let mut reader = BufReader::new(stream);
            let mut next = 0;
            let mut send = |next: &mut usize| -> bool {
                if *next >= replies.len() {
                    return false;
                }
                let _ = writer.write_all(replies[*next].as_bytes());
                let _ = writer.flush();
                *next += 1;
                true
            };
            send(&mut next);
            let mut in_data = false;
            loop {
                let mut line = Vec::new();
                match reader.read_until(b'\n', &mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {}
                }
                record.extend_from_slice(&line);
                if in_data {
                    if line == b".\r\n" {
                        in_data = false;
                        send(&mut next);
                    }
                    continue;
                }
                let pending = &replies[next.min(replies.len() - 1)];
                if pending.starts_with("354") && line.to_ascii_uppercase().starts_with(b"DATA") {
                    in_data = true;
                }
                if !send(&mut next) {
                    break;
                }
            }
        }
        let _ = tx.send(record);
    });
    (port, rx)
}

/// A Cinch string literal.
fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Every scenario writes the reference client's bytes and reports its results.
#[test]
fn smtp_client_matches_reference_transcripts() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/smtp-vectors.json")).unwrap();
    let rows = rows.as_array().unwrap();
    let mut servers = Vec::new();
    let mut calls = Vec::new();
    for r in rows {
        let replies = r["replies"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect::<Vec<_>>();
        let (port, rx) = replay(replies);
        servers.push(rx);
        let steps = r["steps"].as_array().unwrap().iter().map(|v| {
            let s = v.as_str().unwrap();
            let (op, arg) = s.split_once(' ').unwrap_or((s, ""));
            let arg = if op == "data" { arg.replace("\\r", "\r").replace("\\n", "\n") } else { arg.to_string() };
            format!("({}, {})", lit(op), lit(&arg))
        }).collect::<Vec<_>>().join(", ");
        let results = r["results"].as_array().unwrap().iter().map(|v| lit(v.as_str().unwrap())).collect::<Vec<_>>().join(", ");
        calls.push(format!("scenario {port} [{steps}] [{results}]"));
    }
    let main = format!("{HELPERS}\n  -- | Every scenario in order.\n  main :: Unit ->{{IO r}} Int\n  main u = with (io = newIO ()) in checks [{}] 0\n", calls.join(",\n    "));
    let dir = common::project("smtp", &[
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
        ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
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
        ("Text.Smtp.cinch", include_str!("../prelude/Text/Smtp.cinch")),
        ("Effect.Smtp.cinch", include_str!("../prelude/Effect/Smtp.cinch")),
        ("Main.cinch", &main),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing scenario; 200 means all passed");
    common::cleanup(&dir);
    for (r, rx) in rows.iter().zip(servers) {
        let got = rx.recv_timeout(std::time::Duration::from_secs(30)).expect("server finished");
        assert_eq!(String::from_utf8_lossy(&got), r["client"].as_str().unwrap(), "client bytes of {}", r["name"]);
    }
}

/// Reply parsing and body encoding without a connection.
#[test]
fn smtp_replies_and_bodies_parse_and_encode() {
    let dir = common::project("smtp_pure", &[
        ("Text.Smtp.cinch", include_str!("../prelude/Text/Smtp.cinch")),
        ("Text.Base64.cinch", include_str!("../prelude/Text/Base64.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Main.cinch", PURE),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn smtp_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Smtp", include_str!("../prelude/Text/Smtp.cinch"));
    common::assert_documented("Effect.Smtp", include_str!("../prelude/Effect/Smtp.cinch"));
}

/// Pure checks.
const PURE: &str = r#"-- | Pure SMTP checks.
module Main where
  import Text.Smtp
  -- | The outcome of feeding lines to a reader.
  reply :: Nat -> List String -> String
  reply expect lines = feed (smtp-reply-reader expect) lines
  -- | Feed lines until the reply completes.
  feed :: ReplyReader -> List String -> String
  feed reader lines = case lines of
    Nil -> "incomplete"
    Cons line rest -> (case smtp-reply-line reader line of
      NeedLine next -> feed next rest
      ReplyDone r -> (case r of
        Ok x -> string-concat "ok " (smtp-reply-message x)
        Err e -> smtp-error-message e))
  -- | The body encoding as text.
  dots :: String -> String
  dots s = case string-from-bytes (smtp-dot-encode (string-to-bytes s)) of
    Some t -> t
    None -> "?"
  -- | Run the checks.
  main :: Int
  main =
    if not (reply 250 ["250 ok"] == "ok ok") then 1
    else if not (reply 250 ["250-a", "250-b", "250 c"] == "ok a\nb\nc") then 2
    else if not (reply 250 ["550-no", "551 other", "550 end"] == "550 no\n551 other\nend") then 3
    else if not (reply 25 ["251 forward"] == "ok forward") then 4
    else if not (reply 2 ["354 go"] == "354 go") then 5
    else if not (reply 250 ["25"] == "short response: 25") then 6
    else if not (reply 250 ["2x0 bad"] == "invalid response code: 2x0 bad") then 7
    else if not (reply 250 ["099 low"] == "invalid response code: 099 low") then 8
    else if not (reply 0 ["421 bye"] == "ok bye") then 9
    else if not (dots "a\n.b\r\nc" == "a\r\n..b\r\nc\r\n.\r\n") then 10
    else if not (dots "" == "\r\n.\r\n") then 11
    else if not (dots "x\r\n" == "x\r\n.\r\n") then 12
    else if not (dots "x\r" == "x\r\n.\r\n") then 13
    else if not (dots ".\r.\r\n.." == "..\r.\r\n...\r\n.\r\n") then 14
    else 0
"#;

/// Scenario helpers.
const HELPERS: &str = r#"-- | Reference SMTP transcript consumers.
module Main where
  import Effect.Smtp
  import Text.Smtp
  -- | Run each step, collecting its result text (empty for success).
  run-steps :: forall r. using (io :: Io r) => SmtpClient -> List (String, String) ->{IO r} List String
  run-steps c steps = case steps of
    Nil -> Nil
    Cons s rest -> (case s of
      (op, arg) -> (case step c op arg of
        (text, c2) -> Cons text (run-steps c2 rest)))
  -- | One step.
  step :: forall r. using (io :: Io r) => SmtpClient -> String -> String ->{IO r} (String, SmtpClient)
  step c op arg =
    if op == "hello" then outcome (smtp-hello c arg)
    else if op == "mail" then outcome (smtp-mail c arg)
    else if op == "rcpt" then outcome (smtp-rcpt c arg)
    else if op == "data" then outcome (smtp-data c (string-to-bytes arg))
    else if op == "plain" then (case words arg of
      Cons u (Cons p (Cons h rest)) -> outcome (smtp-auth c (SmtpPlainAuth "" u p h))
      other -> ("bad step", c))
    else if op == "cram" then (case words arg of
      Cons u (Cons k rest) -> outcome (smtp-auth c (SmtpCramMd5Auth u k))
      other -> ("bad step", c))
    else if op == "verify" then outcome (smtp-verify c arg)
    else if op == "reset" then outcome (smtp-reset c)
    else if op == "noop" then outcome (smtp-noop c)
    else if op == "extension" then (case smtp-extension c arg of
      (Some v, c2) -> (string-concat "ext " v, c2)
      (None, c2) -> ("no ext", c2))
    else outcome (smtp-quit c)
  -- | A result as text.
  outcome :: (Result SmtpError Unit, SmtpClient) -> (String, SmtpClient)
  outcome p = case p of
    (Ok u, c) -> ("", c)
    (Err e, c) -> (smtp-error-message e, c)
  -- | Space-separated words.
  words :: String -> List String
  words s = split (string-to-bytes s) 0 0
  -- | Words from start, scanning at i.
  split :: Bytes -> Nat -> Nat -> List String
  split b start i =
    if i >= bytes-len b then Cons (piece b start i) Nil
    else if bytes-get b i == 32 then Cons (piece b start i) (split b (i + 1) (i + 1))
    else split b start (i + 1)
  -- | Bytes [start, end) as text.
  piece :: Bytes -> Nat -> Nat -> String
  piece b start end =
    let n = if end >= start then end - start else 0 in
    let out = bytes-new n in
    let c = (if start + n <= bytes-len b then bytes-copy-range out b 0 start n else ()) in
    (case string-from-bytes out of
      Some t -> t
      None -> "")
  -- | One scenario: dial, run the steps, and compare every result.
  scenario :: forall r. using (io :: Io r) => Nat -> List (String, String) -> List String ->{IO r} Bool
  scenario port steps expected = case smtp-dial "127.0.0.1" port of
    Err e -> same [smtp-error-message e] expected
    Ok c -> (case run-steps c steps of
      results -> same (Cons "" results) expected)
  -- | Whether two lists of texts are equal.
  same :: List String -> List String -> Bool
  same a b = case (a, b) of
    (Nil, Nil) -> True
    (Cons x xs, Cons y ys) -> x == y && same xs ys
    other -> False
  -- | The index of the first failing scenario, or 200.
  checks :: List Bool -> Int -> Int
  checks values i = case values of Nil -> 200; Cons x rest -> if x then checks rest (i + 1) else i
"#;
