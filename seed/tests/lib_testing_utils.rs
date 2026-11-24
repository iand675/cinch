//! Independent Go 1.24 reference parity and unit tests for `Effect.Test.Io`,
//! `Effect.Test.Fs`, and `Effect.Test.Quick` (`testing/iotest`, `testing/fstest`,
//! `testing/quick`).
mod common;

use serde_json::Value;

/// Escape a string as a Cinch string literal.
fn cinch_str(s: &str) -> String {
    let mut out = String::from("\"");
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Format a slice of strings as a Cinch `List String`.
fn cinch_str_list(items: &[String]) -> String {
    let mut out = String::from("Nil");
    for s in items.iter().rev() {
        out = format!("(Cons {} {})", cinch_str(s), out);
    }
    out
}

/// Format a slice of u64/i64 values as a Cinch `List Nat`.
fn cinch_nat_list(items: &[u64]) -> String {
    let mut out = String::from("Nil");
    for n in items.iter().rev() {
        out = format!("(Cons {} {})", n, out);
    }
    out
}

/// Format a slice of `(name, is_dir)` pairs as a Cinch `List (String, Bool)`.
fn cinch_dir_entries(entries: &[(String, bool)]) -> String {
    let mut out = String::from("Nil");
    for (name, is_dir) in entries.iter().rev() {
        let b = if *is_dir { "True" } else { "False" };
        out = format!("(Cons ({}, {}) {})", cinch_str(name), b, out);
    }
    out
}

/// Every declaration in all three testing utility modules has canonical module-owned documentation.
#[test]
fn testing_utils_declarations_have_canonical_documentation() {
    common::assert_documented("Effect.Test.Io", include_str!("../prelude/Effect/Test/Io.cinch"));
    common::assert_documented("Effect.Test.Fs", include_str!("../prelude/Effect/Test/Fs.cinch"));
    common::assert_documented("Effect.Test.Quick", include_str!("../prelude/Effect/Test/Quick.cinch"));
}

/// `Effect.Test.Io` matches every Go 1.24 `testing/iotest` vector plus logger and error checks.
#[test]
fn io_test_matches_reference_vectors() {
    let v: Value = serde_json::from_str(include_str!("fixtures/testing-utils-vectors.json")).unwrap();
    let io_v = &v["iotest"];
    let mut checks = Vec::new();

    // 1. OneByteReader vectors
    for tc in io_v["one_byte"].as_array().unwrap() {
        let inp = tc["input"].as_str().unwrap();
        let chunks: Vec<String> = tc["chunks"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_string()).collect();
        checks.push(format!("check-one-byte {} {}", cinch_str(inp), cinch_str_list(&chunks)));
    }

    // 2. HalfReader vectors
    for tc in io_v["half_reader"].as_array().unwrap() {
        let inp = tc["input"].as_str().unwrap();
        let reqs: Vec<u64> = tc["requests"].as_array().unwrap().iter().map(|r| r.as_u64().unwrap()).collect();
        let chunks: Vec<String> = tc["chunks"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_string()).collect();
        checks.push(format!("check-half {} {} {}", cinch_str(inp), cinch_nat_list(&reqs), cinch_str_list(&chunks)));
    }

    // 3. TimeoutReader vector
    {
        let tc = &io_v["timeout"];
        let inp = tc["input"].as_str().unwrap();
        let mut step_list = String::from("Nil");
        for st in tc["steps"].as_array().unwrap().iter().rev() {
            let req = st["request"].as_u64().unwrap();
            let data = st["data"].as_str().unwrap();
            let is_to = if st["is_timeout"].as_bool().unwrap() { "True" } else { "False" };
            let is_eof = if st["is_eof"].as_bool().unwrap() { "True" } else { "False" };
            step_list = format!("(Cons (TimeoutExpect {} {} {} {}) {})", req, cinch_str(data), is_to, is_eof, step_list);
        }
        checks.push(format!("check-timeout {} {}", cinch_str(inp), step_list));
    }

    // 4. DataErrReader vectors
    for tc in io_v["data_err"].as_array().unwrap() {
        let inp = tc["input"].as_str().unwrap();
        let wrapper = tc["wrapper"].as_str().unwrap();
        let out = tc["output"].as_str().unwrap();
        checks.push(format!("check-data-err {} {} {}", cinch_str(inp), cinch_str(wrapper), cinch_str(out)));
    }

    // 5. TruncateWriter vectors
    for tc in io_v["truncate"].as_array().unwrap() {
        let chunks: Vec<String> = tc["chunks"].as_array().unwrap().iter().map(|c| c.as_str().unwrap().to_string()).collect();
        let limit = tc["limit"].as_u64().unwrap();
        let reported: Vec<u64> = tc["reported"].as_array().unwrap().iter().map(|n| n.as_u64().unwrap()).collect();
        let written = tc["written"].as_str().unwrap();
        checks.push(format!(
            "check-truncate {} {} {} {}",
            cinch_str_list(&chunks),
            limit,
            cinch_nat_list(&reported),
            cinch_str(written)
        ));
    }

    // 6. TestReader vectors
    for tc in io_v["test_reader"].as_array().unwrap() {
        let kind = tc["kind"].as_str().unwrap();
        let inp = tc["input"].as_str().unwrap();
        let expected = tc["expected"].as_str().unwrap();
        let pass = if tc["should_pass"].as_bool().unwrap() { "True" } else { "False" };
        checks.push(format!("check-test-reader {} {} {} {}", cinch_str(kind), cinch_str(inp), cinch_str(expected), pass));
    }

    // 7. Extra Cinch-specific checks: data-err-reader in test-reader, data-err error propagation,
    // err-reader, read-logger, write-logger, extra-data detection, error messages, is-timeout-error.
    checks.push("check-test-reader \"data_err\" \"lookahead payload\" \"lookahead payload\" True".to_string());
    checks.push("check-data-err-propagates-error ()".to_string());
    checks.push("check-err-reader ()".to_string());
    checks.push("check-read-logger ()".to_string());
    checks.push("check-write-logger ()".to_string());
    checks.push("check-extra-data-and-messages ()".to_string());

    let mut runner = String::from("200");
    for (idx, expr) in checks.iter().enumerate().rev() {
        runner = format!("(if {} then {} else {})", expr, runner, idx);
    }

    let main_src = format!(
        "{IO_HELPERS}\n  -- | Run every Effect.Test.Io check.\n  main :: Unit ->{{IO r}} Int\n  main u = with (io = newIO ()) in\n    {runner}\n"
    );

    let dir = common::project("testing-utils-io", &[
        ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
        ("Effect.Test.Io.cinch", include_str!("../prelude/Effect/Test/Io.cinch")),
        ("Main.cinch", &main_src),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "exit status is first failing Effect.Test.Io check index; 200 = all passed");
    common::cleanup(&dir);
}

/// `Effect.Test.Fs` matches every Go 1.24 `testing/fstest` vector plus immutability and stream interop checks.
#[test]
fn fs_test_matches_reference_vectors() {
    let v: Value = serde_json::from_str(include_str!("fixtures/testing-utils-vectors.json")).unwrap();
    let fs_v = &v["fstest"];

    // Build the fixture MapFs from JSON specs
    let mut files_list = String::from("Nil");
    for spec in fs_v["files"].as_array().unwrap().iter().rev() {
        let path = spec["path"].as_str().unwrap();
        let data = spec["data"].as_str().unwrap();
        let mode = spec["mode"].as_u64().unwrap();
        let mod_time = spec["mod_time"].as_i64().unwrap();
        let is_dir = if spec["is_dir"].as_bool().unwrap() { "True" } else { "False" };
        files_list = format!(
            "(Cons ({}, MapFile (string-to-bytes {}) {} {} {}) {})",
            cinch_str(path), cinch_str(data), mode, mod_time, is_dir, files_list
        );
    }

    let mut checks = Vec::new();

    // 1. ValidPath vectors (30 cases)
    for tc in fs_v["valid_path"].as_array().unwrap() {
        let p = tc["path"].as_str().unwrap();
        let valid = if tc["valid"].as_bool().unwrap() { "True" } else { "False" };
        checks.push(format!("(fs-valid-path {} == {})", cinch_str(p), valid));
    }

    // 2. ReadFile vectors (11 cases)
    for tc in fs_v["read_file"].as_array().unwrap() {
        let p = tc["path"].as_str().unwrap();
        let data = tc["data"].as_str().unwrap();
        let err = tc["error"].as_str().unwrap();
        checks.push(format!("check-read-file fixture {} {} {}", cinch_str(p), cinch_str(data), cinch_str(err)));
    }

    // 3. Stat vectors (13 cases)
    for tc in fs_v["stat"].as_array().unwrap() {
        let p = tc["path"].as_str().unwrap();
        let name = tc["name"].as_str().unwrap();
        let size = tc["size"].as_u64().unwrap();
        let mode = tc["mode"].as_u64().unwrap();
        let is_dir = if tc["is_dir"].as_bool().unwrap() { "True" } else { "False" };
        let err = tc["error"].as_str().unwrap();
        checks.push(format!(
            "check-stat fixture {} {} {} {} {} {}",
            cinch_str(p), cinch_str(name), size, mode, is_dir, cinch_str(err)
        ));
    }

    // 4. ReadDir vectors (11 cases)
    for tc in fs_v["read_dir"].as_array().unwrap() {
        let p = tc["path"].as_str().unwrap();
        let err = tc["error"].as_str().unwrap();
        let entries: Vec<(String, bool)> = tc["entries"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|e| (e["name"].as_str().unwrap().to_string(), e["is_dir"].as_bool().unwrap()))
                    .collect()
            })
            .unwrap_or_default();
        checks.push(format!("check-read-dir fixture {} {} {}", cinch_str(p), cinch_dir_entries(&entries), cinch_str(err)));
    }

    // 5. Glob vectors (16 cases)
    for tc in fs_v["glob"].as_array().unwrap() {
        let pat = tc["pattern"].as_str().unwrap();
        let matches: Vec<String> = tc["matches"].as_array().unwrap().iter().map(|m| m.as_str().unwrap().to_string()).collect();
        let err = tc["error"].as_str().unwrap();
        checks.push(format!("check-glob fixture {} {} {}", cinch_str(pat), cinch_str_list(&matches), cinch_str(err)));
    }

    // 6. Sub vectors (5 cases)
    for tc in fs_v["sub"].as_array().unwrap() {
        let dir = tc["dir"].as_str().unwrap();
        let entries: Vec<(String, bool)> = tc["entries"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .map(|e| (e["name"].as_str().unwrap().to_string(), e["is_dir"].as_bool().unwrap()))
                    .collect()
            })
            .unwrap_or_default();
        let sample_path = tc["sample_path"].as_str().unwrap();
        let sample_data = tc["sample_data"].as_str().unwrap();
        let err = tc["error"].as_str().unwrap();
        checks.push(format!(
            "check-sub fixture {} {} {} {} {}",
            cinch_str(dir), cinch_dir_entries(&entries), cinch_str(sample_path), cinch_str(sample_data), cinch_str(err)
        ));
    }

    // 7. TestFS vectors (6 cases)
    for tc in fs_v["test_fs"].as_array().unwrap() {
        let use_empty = tc["use_empty"].as_bool().unwrap();
        let expected: Vec<String> = tc["expected"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
        let pass = if tc["should_pass"].as_bool().unwrap() { "True" } else { "False" };
        let fs_expr = if use_empty { "map-fs-empty" } else { "fixture" };
        checks.push(format!("check-test-fs {} {} {}", fs_expr, cinch_str_list(&expected), pass));
    }

    // 8. Extra Cinch-specific checks: snapshot immutability, map-dir/metadata accessors,
    // map-fs-sub error cases, invalid raw key rejection in test-fs, map-fs-open-reader + test-reader interop.
    checks.push("check-fs-extra fixture".to_string());
    checks.push("check-fs-stream-interop fixture".to_string());

    let mut runner = String::from("200");
    for (idx, expr) in checks.iter().enumerate().rev() {
        runner = format!("(if {} then {} else {})", expr, runner, idx);
    }

    let main_src = format!(
        "{FS_HELPERS}\n  -- | Fixture filesystem matching Go reference specs.\n  build-fixture :: MapFs\n  build-fixture = map-fs-from-list {files_list}\n\n  -- | Run every Effect.Test.Fs check.\n  main :: Unit ->{{IO r}} Int\n  main u = with (io = newIO ()) in\n    let fixture = build-fixture in\n    {runner}\n"
    );

    let dir = common::project("testing-utils-fs", &[
        ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
        ("Text.Path.cinch", include_str!("../prelude/Text/Path.cinch")),
        ("Effect.Test.Io.cinch", include_str!("../prelude/Effect/Test/Io.cinch")),
        ("Effect.Test.Fs.cinch", include_str!("../prelude/Effect/Test/Fs.cinch")),
        ("Main.cinch", &main_src),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "exit status is first failing Effect.Test.Fs check index; 200 = all passed");
    common::cleanup(&dir);
}

/// `Effect.Test.Quick` matches every Go 1.24 `testing/quick` vector plus all `Arbitrary` instances and helpers.
#[test]
fn quick_test_matches_reference_vectors() {
    let v: Value = serde_json::from_str(include_str!("fixtures/testing-utils-vectors.json")).unwrap();
    let q_v = &v["quick"]["checks"];
    let mut checks = Vec::new();

    for tc in q_v.as_array().unwrap() {
        let name = tc["name"].as_str().unwrap();
        let max_count = tc["max_count"].as_u64().unwrap();
        let pass = if tc["should_pass"].as_bool().unwrap() { "True" } else { "False" };
        let fail_count = tc["fail_count"].as_u64().unwrap();
        checks.push(format!("check-quick-vector {} {} {} {}", cinch_str(name), max_count, pass, fail_count));
    }

    // Extra Cinch-specific checks covering all Arbitrary instances, config/failure accessors,
    // arities 1/2/3, equal checks, and deterministic replay.
    checks.push("check-all-arbitrary-instances ()".to_string());
    checks.push("check-quick-config-and-failures ()".to_string());
    checks.push("check-quick-determinism ()".to_string());

    let mut runner = String::from("200");
    for (idx, expr) in checks.iter().enumerate().rev() {
        runner = format!("(if {} then {} else {})", expr, runner, idx);
    }

    let main_src = format!(
        "{QUICK_HELPERS}\n  -- | Run every Effect.Test.Quick check.\n  main :: Int\n  main = {runner}\n"
    );

    let dir = common::project("testing-utils-quick", &[
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.ChaCha8.cinch", include_str!("../prelude/Data/ChaCha8.cinch")),
        ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
        ("Data.Rand.cinch", include_str!("../prelude/Data/Rand.cinch")),
        ("Effect.Test.Quick.cinch", include_str!("../prelude/Effect/Test/Quick.cinch")),
        ("Main.cinch", &main_src),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "exit status is first failing Effect.Test.Quick check index; 200 = all passed");
    common::cleanup(&dir);
}

const IO_HELPERS: &str = r#"-- | Consumer harness for Effect.Test.Io reference vectors.
module Main where
  import Algebra.Semigroup
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Stream
  import Effect.Test.Io

  -- | Expected outcome for a single TimeoutReader step.
  data TimeoutExpect where
    -- | Request size, expected data, timeout flag, EOF flag.
    TimeoutExpect :: Nat -> String -> Bool -> Bool -> TimeoutExpect

  -- | Compare a Bytes buffer against a UTF-8 String.
  bytes-eq-str :: Bytes -> String -> Bool
  bytes-eq-str b s = eq-i64 (bytes-compare b (string-to-bytes s)) 0

  -- | Drain a reader using 8-byte requests and verify the chunk sequence.
  drain-chunks :: forall r. using (io :: Io r) => ByteReader r -> List String ->{IO r} Bool
  drain-chunks r expected =
    case stream-read r 8 of
      (Err _, _) -> False
      (Ok chunk, next) ->
        if bytes-len chunk == 0 then
          case expected of
            Nil -> True
            _ -> False
        else
          case expected of
            Nil -> False
            Cons want rest ->
              if bytes-eq-str chunk want then drain-chunks next rest
              else False

  -- | Verify OneByteReader vector.
  check-one-byte :: forall r. using (io :: Io r) => String -> List String ->{IO r} Bool
  check-one-byte inp chunks = drain-chunks (one-byte-reader (reader-from-string inp)) chunks

  -- | Verify HalfReader vector against exact request and chunk sequences.
  check-half-loop :: forall r. using (io :: Io r) => ByteReader r -> List Nat -> List String ->{IO r} Bool
  check-half-loop r reqs wants = case (reqs, wants) of
    (Nil, Nil) -> True
    (Cons req rrest, Cons want wrest) ->
      case stream-read r req of
        (Err _, _) -> False
        (Ok got, next) ->
          if bytes-eq-str got want then check-half-loop next rrest wrest
          else False
    _ -> False

  -- | Verify HalfReader vector.
  check-half :: forall r. using (io :: Io r) => String -> List Nat -> List String ->{IO r} Bool
  check-half inp reqs wants = check-half-loop (half-reader (reader-from-string inp)) reqs wants

  -- | Verify TimeoutReader steps.
  check-timeout-loop :: forall r. using (io :: Io r) => ByteReader r -> List TimeoutExpect ->{IO r} Bool
  check-timeout-loop r steps = case steps of
    Nil -> True
    Cons st rest -> case st of
      TimeoutExpect req want-data want-to want-eof ->
        case stream-read r req of
          (Err e, next) ->
            if want-to && is-timeout-error e then check-timeout-loop next rest
            else False
          (Ok got, next) ->
            if want-to then False
            else if want-eof then
              if bytes-len got == 0 then check-timeout-loop next rest else False
            else
              if bytes-eq-str got want-data then check-timeout-loop next rest else False

  -- | Verify TimeoutReader vector.
  check-timeout :: forall r. using (io :: Io r) => String -> List TimeoutExpect ->{IO r} Bool
  check-timeout inp steps = check-timeout-loop (timeout-reader (reader-from-string inp)) steps

  -- | Verify DataErrReader vector with plain, one_byte, or half wrapper.
  check-data-err :: forall r. using (io :: Io r) => String -> String -> String ->{IO r} Bool
  check-data-err inp wrapper expected =
    let base = reader-from-string inp in
    let wrapped =
          if wrapper == "one_byte" then one-byte-reader base
          else if wrapper == "half" then half-reader base
          else base in
    case stream-read-all 4 (data-err-reader wrapped) of
      (Ok got, _) -> bytes-eq-str got expected
      (Err _, _) -> False

  -- | Write chunks into a writer and verify reported write counts and final buffer.
  check-truncate-loop :: forall r. using (io :: Io r) => ByteWriter r -> List String -> List Nat -> String ->{IO r} Bool
  check-truncate-loop w chunks wants expected = case (chunks, wants) of
    (Nil, Nil) -> True
    (Cons ch crest, Cons want wrest) ->
      case stream-write w (string-to-bytes ch) of
        (Err _, _) -> False
        (Ok n, next) ->
          if n == want then check-truncate-loop next crest wrest expected
          else False
    _ -> False

  -- | Capture written bytes using a side-channel preallocated buffer and length slot.
  buf-capture-writer :: forall r. using (io :: Io r) => Bytes -> Bytes -> ByteWriter r
  buf-capture-writer cap-len cap-buf = stream-writer (\chunk ->
    let n = bytes-len chunk in
    let pos = if 0 < bytes-len cap-len then bytes-get cap-len 0 else 0 in
    let written =
          if pos + n <= bytes-len cap-buf && 0 < bytes-len cap-len && pos + n < 256 then
            let c = bytes-copy-range cap-buf chunk pos 0 n in
            bytes-set cap-len 0 (pos + n)
          else () in
    (Ok n, buf-capture-writer cap-len cap-buf))

  -- | Verify TruncateWriter vector.
  check-truncate :: forall r. using (io :: Io r) => List String -> Nat -> List Nat -> String ->{IO r} Bool
  check-truncate chunks limit reported expected =
    let cap-len = bytes-new 1 in
    let cap-buf = bytes-new 64 in
    let sink = truncate-writer (buf-capture-writer cap-len cap-buf) limit in
    if not (check-truncate-loop sink chunks reported expected) then False
    else
      let total = if 0 < bytes-len cap-len then bytes-get cap-len 0 else 0 in
      let want = string-to-bytes expected in
      if total /= bytes-len want then False
      else if total <= bytes-len cap-buf then
        let actual = bytes-new total in
        let c = bytes-copy-range actual cap-buf 0 0 total in
        eq-i64 (bytes-compare actual want) 0
      else False

  -- | Verify TestReader vector.
  check-test-reader :: forall r. using (io :: Io r) => String -> String -> String -> Bool ->{IO r} Bool
  check-test-reader kind inp expected should-pass =
    let base = reader-from-string inp in
    let r =
          if kind == "one_byte" then one-byte-reader base
          else if kind == "half" then half-reader base
          else if kind == "one_byte_half" then one-byte-reader (half-reader base)
          else if kind == "data_err" then data-err-reader base
          else if kind == "timeout" then timeout-reader base
          else if kind == "err" then err-reader (StreamEncodingError "test" 0 "boom")
          else base in
    case test-reader r (string-to-bytes expected) of
      Ok _ -> should-pass
      Err _ -> not should-pass

  -- | Verify DataErrReader surfaces underlying errors after delivering preceding data.
  check-data-err-propagates-error :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-data-err-propagates-error u =
    let r = data-err-reader (timeout-reader (one-byte-reader (reader-from-string "abcd"))) in
    case stream-read r 1 of
      (Ok b1, r1) ->
        if not (bytes-eq-str b1 "a") then False
        else case stream-read r1 1 of
          (Err e, _) -> is-timeout-error e
          _ -> False
      _ -> False

  -- | Verify err-reader behavior on zero and positive reads.
  check-err-reader :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-err-reader u =
    let custom = StreamClosed "custom" in
    let r = err-reader custom in
    case stream-read r 0 of
      (Ok z, r1) ->
        if bytes-len z /= 0 then False
        else case stream-read r1 4 of
          (Err (StreamClosed tag), r2) ->
            if tag /= "custom" then False
            else case stream-read r2 1 of
              (Err (StreamClosed tag2), _) -> tag2 == "custom"
              _ -> False
          _ -> False
      _ -> False

  -- | Append prefix and bytes into a capture buffer (slot 0 holds byte count).
  log-into :: Bytes -> Bytes -> String -> Bytes -> Unit
  log-into cap-len cap-buf prefix payload =
    let pb = string-to-bytes prefix in
    let plen = bytes-len pb in
    let dlen = bytes-len payload in
    let pos = if 0 < bytes-len cap-len then bytes-get cap-len 0 else 0 in
    let total = pos + plen + dlen in
    if total <= bytes-len cap-buf && total < 256 && 0 < bytes-len cap-len then
      let c1 = bytes-copy-range cap-buf pb pos 0 plen in
      let c2 = bytes-copy-range cap-buf payload (pos + plen) 0 dlen in
      bytes-set cap-len 0 total
    else ()

  -- | Check captured log bytes against an expected string.
  captured-eq :: Bytes -> Bytes -> String -> Bool
  captured-eq cap-len cap-buf expected =
    let total = if 0 < bytes-len cap-len then bytes-get cap-len 0 else 0 in
    let want = string-to-bytes expected in
    if total /= bytes-len want then False
    else if total <= bytes-len cap-buf then
      let actual = bytes-new total in
      let c = bytes-copy-range actual cap-buf 0 0 total in
      eq-i64 (bytes-compare actual want) 0
    else False

  -- | Verify read-logger invokes callback with prefix and read chunks.
  check-read-logger :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-read-logger u =
    let cap-len = bytes-new 1 in
    let cap-buf = bytes-new 128 in
    let r = read-logger "R:" (\p b -> log-into cap-len cap-buf p b) (one-byte-reader (reader-from-string "hi")) in
    case stream-read-all 4 r of
      (Ok got, _) -> bytes-eq-str got "hi" && captured-eq cap-len cap-buf "R:hR:iR:"
      _ -> False

  -- | Verify write-logger invokes callback with prefix and written chunks.
  check-write-logger :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-write-logger u =
    let cap-len = bytes-new 1 in
    let cap-buf = bytes-new 128 in
    let w = write-logger "W:" (\p b -> log-into cap-len cap-buf p b) writer-buffer in
    case stream-write w (string-to-bytes "ab") of
      (Ok n1, w1) ->
        if n1 /= 2 then False
        else case stream-write w1 (string-to-bytes "cd") of
          (Ok n2, _) -> n2 == 2 && captured-eq cap-len cap-buf "W:abW:cd"
          _ -> False
      _ -> False

  -- | A faulty reader that emits data after signaling EOF once.
  extra-after-eof-reader :: forall r. using (io :: Io r) => Nat -> ByteReader r
  extra-after-eof-reader step = stream-reader (\n ->
    if step == 0 then (Ok (string-to-bytes "a"), extra-after-eof-reader 1)
    else if step == 1 then (Ok (bytes-new 0), extra-after-eof-reader 2)
    else (Ok (string-to-bytes "!"), extra-after-eof-reader 3))

  -- | Verify IoTestExtraData detection, is-timeout-error, and io-test-error-message coverage.
  check-extra-data-and-messages :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-extra-data-and-messages u =
    let extra-ok = case test-reader (extra-after-eof-reader 0) (string-to-bytes "a") of
          Err (IoTestExtraData b) -> bytes-eq-str b "!"
          _ -> False in
    let to-ok = is-timeout-error err-timeout && not (is-timeout-error (StreamClosed "x")) in
    let msgs-ok =
          io-test-error-message (IoTestStreamError err-timeout) /= "" &&
          io-test-error-message (IoTestContentMismatch (bytes-new 0) (bytes-new 0)) /= "" &&
          io-test-error-message (IoTestShortRead 5 2) /= "" &&
          io-test-error-message (IoTestExtraData (bytes-new 1)) /= "" &&
          io-test-error-message (IoTestZeroReadNonEmpty (bytes-new 1)) /= "" in
    extra-ok && to-ok && msgs-ok
"#;

const FS_HELPERS: &str = r#"-- | Consumer harness for Effect.Test.Fs reference vectors.
module Main where
  import Algebra.Semigroup
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Stream
  import Effect.Test.Fs
  import Effect.Test.Io

  -- | Compare Bytes with a UTF-8 String.
  bytes-eq-str :: Bytes -> String -> Bool
  bytes-eq-str b s = eq-i64 (bytes-compare b (string-to-bytes s)) 0

  -- | Map MapFsError to fixture error classification string.
  fs-err-tag :: MapFsError -> String
  fs-err-tag e = case e of
    FsInvalidPath _ -> "invalid"
    FsNotFound _ -> "not_found"
    FsIsDirectory _ -> "is_dir"
    FsNotDirectory _ -> "not_dir"
    FsBadPattern _ -> "bad_pattern"

  -- | Verify map-fs-read-file vector.
  check-read-file :: MapFs -> String -> String -> String -> Bool
  check-read-file fs path want-data want-err = case map-fs-read-file fs path of
    Ok b -> want-err == "" && bytes-eq-str b want-data
    Err e -> want-err /= "" && fs-err-tag e == want-err

  -- | Verify map-fs-stat vector.
  check-stat :: MapFs -> String -> String -> Nat -> U32 -> Bool -> String -> Bool
  check-stat fs path want-name want-size want-mode want-dir want-err = case map-fs-stat fs path of
    Ok info ->
      want-err == "" &&
      file-info-name info == want-name &&
      file-info-size info == want-size &&
      eq-u32 (file-info-mode info) want-mode &&
      file-info-is-dir info == want-dir
    Err e -> want-err /= "" && fs-err-tag e == want-err

  -- | Compare directory entry list against expected (name, is_dir) pairs.
  dir-entries-match :: List MapDirEntry -> List (String, Bool) -> Bool
  dir-entries-match got wants = case (got, wants) of
    (Nil, Nil) -> True
    (Cons e erest, Cons pair wrest) -> case pair of
      (wname, wdir) ->
        if dir-entry-name e == wname && dir-entry-is-dir e == wdir then
          dir-entries-match erest wrest
        else False
    _ -> False

  -- | Verify map-fs-read-dir vector.
  check-read-dir :: MapFs -> String -> List (String, Bool) -> String -> Bool
  check-read-dir fs path wants want-err = case map-fs-read-dir fs path of
    Ok entries -> want-err == "" && dir-entries-match entries wants
    Err e -> want-err /= "" && fs-err-tag e == want-err

  -- | Compare two string lists for exact equality.
  str-list-eq :: List String -> List String -> Bool
  str-list-eq xs ys = case (xs, ys) of
    (Nil, Nil) -> True
    (Cons a ra, Cons b rb) -> if a == b then str-list-eq ra rb else False
    _ -> False

  -- | Verify map-fs-glob vector.
  check-glob :: MapFs -> String -> List String -> String -> Bool
  check-glob fs pat wants want-err = case map-fs-glob fs pat of
    Ok matches -> want-err == "" && str-list-eq matches wants
    Err e -> want-err /= "" && fs-err-tag e == want-err

  -- | Verify map-fs-sub vector.
  check-sub :: MapFs -> String -> List (String, Bool) -> String -> String -> String -> Bool
  check-sub fs dir wants sample-path sample-data want-err = case map-fs-sub fs dir of
    Err e -> want-err /= "" && fs-err-tag e == want-err
    Ok sub ->
      if want-err /= "" then False
      else case map-fs-read-dir sub "." of
        Err _ -> False
        Ok entries ->
          if not (dir-entries-match entries wants) then False
          else if sample-path == "" then True
          else case map-fs-read-file sub sample-path of
            Ok b -> bytes-eq-str b sample-data
            Err _ -> False

  -- | Verify test-fs vector.
  check-test-fs :: MapFs -> List String -> Bool -> Bool
  check-test-fs fs expected should-pass = case test-fs fs expected of
    Ok _ -> should-pass
    Err _ -> not should-pass

  -- | Verify snapshot immutability, explicit directory helpers, error messages, and invalid raw key detection.
  check-fs-extra :: MapFs -> Bool
  check-fs-extra fixture =
    let raw = string-to-bytes "abc" in
    let f = map-file raw in
    let mutated-in = if 0 < bytes-len raw then bytes-set raw 0 88 else () in
    let fs1 = map-fs-insert map-fs-empty "f.txt" f in
    let snap-ok = case map-fs-read-file fs1 "f.txt" of
          Ok b1 ->
            let mutated-out = if 0 < bytes-len b1 then bytes-set b1 0 90 else () in
            case map-fs-read-file fs1 "f.txt" of
              Ok b2 -> bytes-eq-str b2 "abc"
              Err _ -> False
          Err _ -> False in
    let dir-f = map-dir 493 1700000099 in
    let dir-ok =
          map-file-is-dir dir-f &&
          eq-i64 (map-file-mod-time dir-f) 1700000099 &&
          eq-u32 (bit-and-u32 (map-file-mode dir-f) fs-mode-dir) fs-mode-dir in
    let fs2 = map-fs-insert fs1 "d" dir-f in
    let info-ok = case map-fs-stat fs2 "d" of
          Ok st -> eq-i64 (file-info-mod-time st) 1700000099 && file-info-is-dir st
          Err _ -> False in
    let entry-ok = case map-fs-read-dir fs2 "." of
          Ok (Cons e1 (Cons e2 Nil)) ->
            dir-entry-name e1 == "d" &&
            eq-u32 (dir-entry-type e1) fs-mode-dir &&
            file-info-is-dir (dir-entry-info e1) &&
            dir-entry-name e2 == "f.txt" &&
            eq-u32 (dir-entry-type e2) 0
          _ -> False in
    let sub-errs-ok =
          (case map-fs-sub fixture "README.md" of Err (FsNotDirectory _) -> True; _ -> False) &&
          (case map-fs-sub fixture "missing" of Err (FsNotFound _) -> True; _ -> False) in
    let bad-fs = map-fs-insert map-fs-empty "/illegal" (map-file (string-to-bytes "x")) in
    let bad-key-ok = case test-fs bad-fs Nil of Err _ -> True; Ok _ -> False in
    let msgs-ok =
          map-fs-error-message (FsInvalidPath "x") /= "" &&
          map-fs-error-message (FsNotFound "x") /= "" &&
          map-fs-error-message (FsIsDirectory "x") /= "" &&
          map-fs-error-message (FsNotDirectory "x") /= "" &&
          map-fs-error-message (FsBadPattern "[") /= "" in
    snap-ok && dir-ok && info-ok && entry-ok && sub-errs-ok && bad-key-ok && msgs-ok

  -- | Verify map-fs-open-reader interoperates with Effect.Test.Io test-reader and reports errors cleanly.
  check-fs-stream-interop :: forall r. using (io :: Io r) => MapFs ->{IO r} Bool
  check-fs-stream-interop fixture =
    let ok-read = case map-fs-open-reader fixture "a/b/c.txt" of
          Err _ -> False
          Ok r -> case test-reader (one-byte-reader r) (string-to-bytes "hello") of
            Ok _ -> True
            Err _ -> False in
    let err-dir = case map-fs-open-reader fixture "a" of
          Err (FsIsDirectory _) -> True
          _ -> False in
    let err-miss = case map-fs-open-reader fixture "nope" of
          Err (FsNotFound _) -> True
          _ -> False in
    let err-inv = case map-fs-open-reader fixture "/nope" of
          Err (FsInvalidPath _) -> True
          _ -> False in
    ok-read && err-dir && err-miss && err-inv
"#;

const QUICK_HELPERS: &str = r#"-- | Consumer harness for Effect.Test.Quick reference vectors.
module Main where
  import Algebra.Semigroup
  import Data.Bool
  import Data.Bytes
  import Data.Char
  import Data.Float
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Rand
  import Data.Result
  import Data.String
  import Effect.Test.Quick

  -- | Always-true I64 property.
  p-true-i64 :: I64 -> Bool
  p-true-i64 x = True

  -- | Always-false I64 property.
  p-false-i64 :: I64 -> Bool
  p-false-i64 x = False

  -- | Wrapping addition commutativity on I64.
  p-add-comm-i64 :: I64 -> I64 -> Bool
  p-add-comm-i64 x y = eq-i64 (wrapping-add-i64 x y) (wrapping-add-i64 y x)

  -- | Self-XOR is zero on U32.
  p-xor-zero-u32 :: U32 -> Bool
  p-xor-zero-u32 x = eq-u32 (bit-xor-u32 x x) 0

  -- | Double negation on Bool.
  p-double-neg :: Bool -> Bool
  p-double-neg b = not (not b) == b

  -- | String concatenation associativity.
  p-str-assoc :: String -> String -> String -> Bool
  p-str-assoc a b c = ((a <> b) <> c) == (a <> (b <> c))

  -- | Double an I64 via addition.
  f-add-self :: I64 -> I64
  f-add-self x = wrapping-add-i64 x x

  -- | Double an I64 via multiplication.
  f-mul-two :: I64 -> I64
  f-mul-two x = wrapping-mul-i64 x 2

  -- | Identity on I64.
  f-id-i64 :: I64 -> I64
  f-id-i64 x = x

  -- | Flip lowest bit on I64.
  f-flip-i64 :: I64 -> I64
  f-flip-i64 x = bit-xor-i64 x 1

  -- | Run a named Go quick vector case.
  check-quick-vector :: String -> Nat -> Bool -> Nat -> Bool
  check-quick-vector name max-count should-pass fail-count =
    let cfg = QuickConfig max-count 50 (pcg-new 12345 67890) in
    if name == "always_true_100" then
      case quick-check-with cfg p-true-i64 of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "always_false_10" then
      case quick-check-with cfg p-false-i64 of
        Ok _ -> False
        Err f -> not should-pass && quick-failure-count f == fail-count
    else if name == "add_comm_i64" then
      case quick-check2-with cfg p-add-comm-i64 of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "xor_self_zero_u32" then
      case quick-check-with cfg p-xor-zero-u32 of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "double_neg_bool" then
      case quick-check-with cfg p-double-neg of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "string_concat_assoc" then
      case quick-check3-with cfg p-str-assoc of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "eq_double" then
      case quick-check-equal-with cfg f-add-self f-mul-two of
        Ok n -> should-pass && n == max-count
        Err _ -> False
    else if name == "eq_mismatch" then
      case quick-check-equal-with cfg f-id-i64 f-flip-i64 of
        Ok _ -> False
        Err f ->
          not should-pass &&
          quick-equal-count f == fail-count &&
          eq-i64 (quick-equal-left f) (quick-equal-input f) &&
          eq-i64 (quick-equal-right f) (bit-xor-i64 (quick-equal-input f) 1)
    else False

  -- | Check whether a Result is Ok with expected count.
  is-ok-count :: forall e. Result e Nat -> Nat -> Bool
  is-ok-count res expected = case res of
    Ok n -> n == expected
    Err _ -> False

  -- | Typed properties for each Arbitrary instance.
  p-bool :: Bool -> Bool
  p-bool b = b || not b

  -- | U8 identity under XOR 0.
  p-u8 :: U8 -> Bool
  p-u8 x = eq-u8 (bit-xor-u8 x 0) x

  -- | U16 identity under XOR 0.
  p-u16 :: U16 -> Bool
  p-u16 x = eq-u16 (bit-xor-u16 x 0) x

  -- | U32 identity under XOR 0.
  p-u32 :: U32 -> Bool
  p-u32 x = eq-u32 (bit-xor-u32 x 0) x

  -- | U64 identity under XOR 0.
  p-u64 :: U64 -> Bool
  p-u64 x = eq-u64 (bit-xor-u64 x 0) x

  -- | I8 identity under adding 0.
  p-i8 :: I8 -> Bool
  p-i8 x = eq-i8 (wrapping-add-i8 x 0) x

  -- | I16 identity under adding 0.
  p-i16 :: I16 -> Bool
  p-i16 x = eq-i16 (wrapping-add-i16 x 0) x

  -- | I32 identity under adding 0.
  p-i32 :: I32 -> Bool
  p-i32 x = eq-i32 (wrapping-add-i32 x 0) x

  -- | I64 identity under adding 0.
  p-i64 :: I64 -> Bool
  p-i64 x = eq-i64 (wrapping-add-i64 x 0) x

  -- | Nat additive identity.
  p-nat :: Nat -> Bool
  p-nat n = n + 0 == n

  -- | Int double negation.
  p-int :: Int -> Bool
  p-int n = (0 - (0 - n)) == n

  -- | F64 finiteness.
  p-f64 :: F64 -> Bool
  p-f64 x = not (is-nan-f64 x) && not (is-inf-f64 x 0)

  -- | Char scalar bound.
  p-char :: Char -> Bool
  p-char c = le-u32 (char-to-u32 c) 1114111

  -- | String reflexivity.
  p-str :: String -> Bool
  p-str s = s == s

  -- | Bytes size bound.
  p-bytes :: Bytes -> Bool
  p-bytes b = bytes-len b <= 12

  -- | Option exhaustiveness.
  p-opt :: Option U16 -> Bool
  p-opt o = case o of None -> True; Some _ -> True

  -- | List size bound and reverse length preservation.
  p-list :: List U8 -> Bool
  p-list xs = list-len xs <= 12 && list-len (reverse xs) == list-len xs

  -- | Pair property.
  p-pair :: U32 -> Bool -> Bool
  p-pair a b = eq-u32 a a && (b || not b)

  -- | Triple property.
  p-triple :: I16 -> Char -> Option Bool -> Bool
  p-triple a b c = eq-i16 a a

  -- | Length of reversed Nat list.
  f-rev-rev :: List Nat -> Nat
  f-rev-rev xs = list-len (reverse xs)

  -- | Length of Nat list.
  f-id-list :: List Nat -> Nat
  f-id-list xs = list-len xs

  -- | Small U8 predicate that falsifies on >= 10.
  p-u8-small :: U8 -> Bool
  p-u8-small n = lt-u8 n 10

  -- | Small U8 predicate that falsifies on >= 128.
  p-u8-half :: U8 -> Bool
  p-u8-half n = lt-u8 n 128

  -- | Exercise every exported Arbitrary instance across all primitive and compound types.
  check-all-arbitrary-instances :: Unit -> Bool
  check-all-arbitrary-instances u =
    let cfg = QuickConfig 25 12 (pcg-new 7 13) in
    let ok-bool = is-ok-count (quick-check p-bool) 100 in
    let ok-u8 = is-ok-count (quick-check-with cfg p-u8) 25 in
    let ok-u16 = is-ok-count (quick-check-with cfg p-u16) 25 in
    let ok-u32 = is-ok-count (quick-check-with cfg p-u32) 25 in
    let ok-u64 = is-ok-count (quick-check-with cfg p-u64) 25 in
    let ok-i8 = is-ok-count (quick-check-with cfg p-i8) 25 in
    let ok-i16 = is-ok-count (quick-check-with cfg p-i16) 25 in
    let ok-i32 = is-ok-count (quick-check-with cfg p-i32) 25 in
    let ok-i64 = is-ok-count (quick-check-with cfg p-i64) 25 in
    let ok-nat = is-ok-count (quick-check-with cfg p-nat) 25 in
    let ok-int = is-ok-count (quick-check-with cfg p-int) 25 in
    let ok-f64 = is-ok-count (quick-check-with cfg p-f64) 25 in
    let ok-char = is-ok-count (quick-check-with cfg p-char) 25 in
    let ok-str = is-ok-count (quick-check-with cfg p-str) 25 in
    let ok-bytes = is-ok-count (quick-check-with cfg p-bytes) 25 in
    let ok-opt = is-ok-count (quick-check-with cfg p-opt) 25 in
    let ok-list = is-ok-count (quick-check-with cfg p-list) 25 in
    let ok-pair = is-ok-count (quick-check2 p-pair) 100 in
    let ok-triple = is-ok-count (quick-check3 p-triple) 100 in
    let ok-eq = is-ok-count (quick-check-equal f-rev-rev f-id-list) 100 in
    ok-bool && ok-u8 && ok-u16 && ok-u32 && ok-u64 && ok-i8 && ok-i16 && ok-i32 && ok-i64 &&
    ok-nat && ok-int && ok-f64 && ok-char && ok-str && ok-bytes && ok-opt && ok-list &&
    ok-pair && ok-triple && ok-eq

  -- | Compute list length tail-recursively.
  list-len :: forall a. List a -> Nat
  list-len xs = list-len-acc xs 0

  -- | Tail-recursive list length accumulator.
  list-len-acc :: forall a. List a -> Nat -> Nat
  list-len-acc xs acc = case xs of
    Nil -> acc
    Cons _ rest -> list-len-acc rest (acc + 1)

  -- | Verify QuickConfig accessors and QuickFailure/QuickEqualFailure counterexample fields.
  check-quick-config-and-failures :: Unit -> Bool
  check-quick-config-and-failures u =
    let cfg-ok =
          quick-config-max-count default-quick-config == 100 &&
          quick-config-max-size default-quick-config == 50 in
    let seed-ok = case pcg-uint64 (quick-config-seed default-quick-config) of
          (w1, _) -> case pcg-uint64 (pcg-new 1 2) of
            (w2, _) -> eq-u64 w1 w2 in
    let fail-ok = case quick-check p-u8-small of
          Ok _ -> False
          Err f ->
            quick-failure-count f >= 1 &&
            not (lt-u8 (quick-failure-input f) 10) &&
            quick-failure-reason f /= "" in
    cfg-ok && seed-ok && fail-ok

  -- | Verify deterministic replay under identical PCG seeds.
  check-quick-determinism :: Unit -> Bool
  check-quick-determinism u =
    let cfg1 = QuickConfig 100 20 (pcg-new 42 99) in
    let cfg2 = QuickConfig 100 20 (pcg-new 42 99) in
    case (quick-check-with cfg1 p-u8-half, quick-check-with cfg2 p-u8-half) of
      (Err f1, Err f2) ->
        quick-failure-count f1 == quick-failure-count f2 &&
        eq-u8 (quick-failure-input f1) (quick-failure-input f2)
      _ -> False
"#;
