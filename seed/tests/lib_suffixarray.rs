//! Suffix array parity against Go's `index/suffixarray` (`tooling/suffixarray_vectors.go`):
//! construction on empty, single-byte, repeated-character, all-256-byte binary, Fibonacci,
//! Thue-Morse, English, and UTF-8 corpora; `Lookup` with negative, zero, and bounded limits;
//! sorted lookup; `FindAllIndex` regular expression scanning; snapshot immutability; versioned
//! binary serialization and corruption rejection; large-input O(n log n) performance; and
//! canonical module documentation.
mod common;

use serde_json::Value;
use std::process::Command;
use std::time::Instant;

/// Encode a byte slice as a Cinch `Bytes` literal via `bytes-from-u8s [...]`.
fn bytes_expr(bytes: &[u8]) -> String {
    let items: Vec<String> = bytes.iter().map(|b| b.to_string()).collect();
    format!("(bytes-from-u8s [{}])", items.join(", "))
}

/// Decode a hex string into bytes.
fn decode_hex(s: &str) -> Vec<u8> {
    assert_eq!(s.len() % 2, 0);
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

/// Format a Cinch string literal.
fn string_lit(s: &str) -> String {
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

/// Format a Cinch Int literal.
fn int_expr(n: i64) -> String {
    if n < 0 {
        format!("(0 - {})", -n)
    } else {
        n.to_string()
    }
}

/// Format a Cinch `List Nat` literal.
fn nat_list(values: &[Value]) -> String {
    let items: Vec<String> = values
        .iter()
        .map(|v| v.as_u64().unwrap().to_string())
        .collect();
    format!("[{}]", items.join(", "))
}

/// Format a Cinch `List (Nat, Nat)` literal.
fn pair_list(values: &[Value]) -> String {
    let items: Vec<String> = values
        .iter()
        .map(|pair| {
            let arr = pair.as_array().unwrap();
            format!(
                "({}, {})",
                arr[0].as_u64().unwrap(),
                arr[1].as_u64().unwrap()
            )
        })
        .collect();
    format!("[{}]", items.join(", "))
}

/// Build per-corpus Cinch check functions for all reference vectors.
fn build_reference_program() -> (String, usize) {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/suffixarray-vectors.json")).unwrap();
    let corpora = fixture["corpora"].as_array().unwrap();
    assert!(!corpora.is_empty());

    let mut defs = String::new();
    let mut calls = Vec::new();
    let mut total_checks = 0usize;

    for (ci, corpus) in corpora.iter().enumerate() {
        assert!(corpus["go_roundtrip"].as_bool().unwrap());
        let data = decode_hex(corpus["data_hex"].as_str().unwrap());
        let sa = nat_list(corpus["sa"].as_array().unwrap());
        let lookups = corpus["lookups"].as_array().unwrap();
        let regexes = corpus["regexes"].as_array().unwrap();

        let mut lookup_rows = Vec::new();
        for lv in lookups {
            let pat = decode_hex(lv["pattern_hex"].as_str().unwrap());
            let limit = lv["limit"].as_i64().unwrap();
            let offsets = nat_list(lv["offsets"].as_array().unwrap());
            let sorted = nat_list(lv["sorted"].as_array().unwrap());
            let count = lv["count"].as_u64().unwrap();
            lookup_rows.push(format!(
                "check-lookup idx {} {} {} {} {}",
                bytes_expr(&pat),
                int_expr(limit),
                offsets,
                sorted,
                count
            ));
            total_checks += 1;
        }

        let mut regex_rows = Vec::new();
        for rv in regexes {
            let pat = rv["pattern"].as_str().unwrap();
            let limit = rv["limit"].as_i64().unwrap();
            let matches = pair_list(rv["matches"].as_array().unwrap());
            regex_rows.push(format!(
                "check-regex idx {} {} {}",
                string_lit(pat),
                int_expr(limit),
                matches
            ));
            total_checks += 1;
        }

        // Count corpus SA + roundtrip check.
        total_checks += 1;

        let fn_name = format!("corpus-check-{}", alpha_tag(ci));
        defs.push_str(&format!(
            "  -- | Verify corpus {name}.\n  {fn_name} :: Bool\n  {fn_name} =\n    let raw = {data_expr} in\n    let idx = suffix-array-new raw in\n    let want-sa = {sa} in\n    if not (check-sa idx raw want-sa) then False\n    else if not (check-roundtrip idx raw want-sa) then False\n    else if not (all-true [{lookups_body}]) then False\n    else all-true [{regexes_body}]\n\n",
            name = corpus["name"].as_str().unwrap(),
            data_expr = bytes_expr(&data),
            lookups_body = lookup_rows.join(", "),
            regexes_body = regex_rows.join(", "),
        ));
        calls.push(fn_name);
    }

    assert!(calls.len() < 200);
    let source = format!(
        r#"-- | Reference suffix array vector harness.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.SuffixArray
  import Text.Regex

  -- | Check whether every Boolean in a list is True.
  all-true :: List Bool -> Bool
  all-true xs = case xs of
    Nil -> True
    Cons b rest -> if b then all-true rest else False

  -- | Check equality of two List Nat values.
  eq-nat-list :: List Nat -> List Nat -> Bool
  eq-nat-list xs ys = case xs of
    Nil -> case ys of Nil -> True; Cons _ _ -> False
    Cons x xrest -> case ys of
      Nil -> False
      Cons y yrest -> if x == y then eq-nat-list xrest yrest else False

  -- | Check equality of two List (Nat, Nat) values.
  eq-pair-list :: List (Nat, Nat) -> List (Nat, Nat) -> Bool
  eq-pair-list xs ys = case xs of
    Nil -> case ys of Nil -> True; Cons _ _ -> False
    Cons a arest -> case ys of
      Nil -> False
      Cons b brest -> case a of
        (a0, a1) -> case b of
          (b0, b1) -> if a0 == b0 && a1 == b1 then eq-pair-list arest brest else False

  -- | Check every entry of the suffix array against the reference SA list.
  check-sa-entries :: SuffixArray -> List Nat -> Nat -> Bool
  check-sa-entries idx want i = case want of
    Nil -> case suffix-array-at idx i of None -> True; Some _ -> False
    Cons x rest -> case suffix-array-at idx i of
      None -> False
      Some got -> if got == x then check-sa-entries idx rest (i + 1) else False

  -- | Check length, snapshot bytes, and every SA entry.
  check-sa :: SuffixArray -> Bytes -> List Nat -> Bool
  check-sa idx raw want-sa =
    suffix-array-len idx == bytes-len raw &&
    suffix-array-bytes idx == raw &&
    check-sa-entries idx want-sa 0

  -- | Check binary encode/decode roundtrip preserves bytes, SA, and canonical encoding.
  check-roundtrip :: SuffixArray -> Bytes -> List Nat -> Bool
  check-roundtrip idx raw want-sa =
    let enc = suffix-array-encode idx in
    case suffix-array-decode enc of
      Err _ -> False
      Ok dec ->
        check-sa dec raw want-sa &&
        suffix-array-encode dec == enc

  -- | Check lookup, sorted lookup, and count against reference expectations.
  check-lookup :: SuffixArray -> Bytes -> Int -> List Nat -> List Nat -> Nat -> Bool
  check-lookup idx pat lim want-raw want-sorted want-count =
    eq-nat-list (suffix-array-lookup idx pat lim) want-raw &&
    eq-nat-list (suffix-array-lookup-sorted idx pat lim) want-sorted &&
    suffix-array-count idx pat == want-count

  -- | Check regex FindAllIndex against reference expectations.
  check-regex :: SuffixArray -> String -> Int -> List (Nat, Nat) -> Bool
  check-regex idx pat lim want = case parse-regex pat of
    Err _ -> False
    Ok re -> eq-pair-list (suffix-array-find-all-regex idx re lim) want

  -- | Index of first failing check, or 200 if all pass.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i

{defs}
  -- | Run every corpus vector check.
  main :: Int
  main = first-fail [{calls}] 0
"#,
        defs = defs,
        calls = calls.join(", ")
    );
    (source, total_checks)
}

/// Encode a small index as letters (no hyphen-digit identifiers in Cinch).
fn alpha_tag(mut n: usize) -> String {
    let mut chars = Vec::new();
    loop {
        chars.push((b'a' + (n % 26) as u8) as char);
        if n < 26 {
            break;
        }
        n = n / 26 - 1;
    }
    chars.into_iter().rev().collect()
}

/// Standard source modules needed to compile programs using Data.SuffixArray.
fn modules_with_main(main: &str) -> Vec<(&str, &str)> {
    vec![
        ("Text.Regex.cinch", include_str!("../prelude/Text/Regex.cinch")),
        (
            "Data.SuffixArray.cinch",
            include_str!("../prelude/Data/SuffixArray.cinch"),
        ),
        ("Main.cinch", main),
    ]
}

#[test]
fn suffix_array_matches_reference_vectors() {
    let (main, total_checks) = build_reference_program();
    assert!(total_checks >= 700, "expected >= 700 reference checks, got {total_checks}");
    let dir = common::project("suffixarray_vectors", &modules_with_main(&main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 200,
        "exit status names first failing corpus index; 200 means all {total_checks} reference checks passed"
    );
}

#[test]
fn suffix_array_immutability_and_corruption_rejections() {
    let main = r#"-- | Immutability, invalid UTF-8 regex safety, and binary corruption rejection checks.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Word
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Data.SuffixArray
  import Text.Regex

  -- | Check equality of two List Nat values.
  eq-nat-list :: List Nat -> List Nat -> Bool
  eq-nat-list xs ys = case xs of
    Nil -> case ys of Nil -> True; Cons _ _ -> False
    Cons x xrest -> case ys of
      Nil -> False
      Cons y yrest -> if x == y then eq-nat-list xrest yrest else False

  -- | Copy a buffer and mutate one byte at pos to val.
  with-byte :: Bytes -> Nat -> InRange 0 255 Nat -> Bytes
  with-byte src pos val =
    let n = bytes-len src in
    let out = bytes-new n in
    let u1 = bytes-copy-range out src 0 0 n in
    let u2 = (if pos < bytes-len out then bytes-set out pos val else ()) in
    out

  -- | Copy the first len bytes of src.
  take-bytes :: Bytes -> Nat -> Bytes
  take-bytes src count =
    let n = (if count <= bytes-len src then count else bytes-len src) in
    let out = bytes-new n in
    let u = (if n <= bytes-len src then bytes-copy-range out src 0 0 n else ()) in
    out

  -- | Append one zero byte to src.
  append-zero :: Bytes -> Bytes
  append-zero src =
    let n = bytes-len src in
    let out = bytes-new (n + 1) in
    let u = bytes-copy-range out src 0 0 n in
    out

  -- | Write a 32-bit little-endian word into a copy of src at pos.
  with-le32 :: Bytes -> Nat -> U32 -> Bytes
  with-le32 src pos val =
    let n = bytes-len src in
    let out = bytes-new n in
    let u1 = bytes-copy-range out src 0 0 n in
    let u2 = (if pos + 4 <= bytes-len out then bytes-write-le32 out pos val else ()) in
    out

  -- | Check that decoding fails with SuffixArrayInvalidHeader.
  expect-invalid-header :: Bytes -> Bool
  expect-invalid-header b = case suffix-array-decode b of
    Err SuffixArrayInvalidHeader -> True
    _ -> False

  -- | Check that decoding fails with SuffixArrayTruncated.
  expect-truncated :: Bytes -> Bool
  expect-truncated b = case suffix-array-decode b of
    Err SuffixArrayTruncated -> True
    _ -> False

  -- | Check that decoding fails with SuffixArrayCorruptIndex.
  expect-corrupt :: Bytes -> Bool
  expect-corrupt b = case suffix-array-decode b of
    Err SuffixArrayCorruptIndex -> True
    _ -> False

  -- | Verify mutating caller input or returned snapshot bytes never mutates the index.
  check-immutability :: Bool
  check-immutability =
    let src = string-to-bytes "banana" in
    let idx = suffix-array-new src in
    let u1 = (if 0 < bytes-len src then bytes-set src 0 120 else ()) in
    let snap = suffix-array-bytes idx in
    let u2 = (if 1 < bytes-len snap then bytes-set snap 1 121 else ()) in
    suffix-array-bytes idx == string-to-bytes "banana" &&
    eq-nat-list (suffix-array-lookup idx (string-to-bytes "ban") (0 - 1)) [0] &&
    eq-nat-list (suffix-array-lookup idx (string-to-bytes "xan") (0 - 1)) []

  -- | Verify invalid UTF-8 bytes safely yield Nil for regex search.
  check-invalid-utf8-regex :: Bool
  check-invalid-utf8-regex =
    let idx = suffix-array-new (bytes-from-u8s [255, 254, 128, 129]) in
    case parse-regex "a*" of
      Err _ -> False
      Ok re -> case suffix-array-find-all-regex idx re (0 - 1) of
        Nil -> True
        Cons _ _ -> False

  -- | Build a valid 64-bit-width encoded index for "banana" (n = 6, width = 8).
  encode-wide-banana :: Bytes
  encode-wide-banana =
    let out = bytes-new (16 + 6 + 48) in
    let u0 = (if 16 <= bytes-len out then
      let a = bytes-set out 0 67 in
      let b = bytes-set out 1 83 in
      let c = bytes-set out 2 65 in
      let d = bytes-set out 3 49 in
      let e = bytes-set out 4 1 in
      let f = bytes-set out 5 8 in
      let g = bytes-set out 6 0 in
      let h = bytes-set out 7 0 in
      bytes-write-le64 out 8 6
      else ()) in
    let raw = string-to-bytes "banana" in
    let u1 = (if 22 <= bytes-len out && 6 <= bytes-len raw then bytes-copy-range out raw 16 0 6 else ()) in
    let u2 = (if 70 <= bytes-len out then
      let a = bytes-write-le64 out 22 5 in
      let b = bytes-write-le64 out 30 3 in
      let c = bytes-write-le64 out 38 1 in
      let d = bytes-write-le64 out 46 0 in
      let e = bytes-write-le64 out 54 4 in
      bytes-write-le64 out 62 2
      else ()) in
    out

  -- | Check that a 64-bit-width encoding of a small corpus decodes cleanly,
  -- and that setting bit 63 in a 64-bit offset is rejected as corrupt.
  check-wide-decode :: Bool
  check-wide-decode =
    let good = encode-wide-banana in
    let bad = with-byte good 29 128 in
    let ok-good = (case suffix-array-decode good of
      Err _ -> False
      Ok idx ->
        suffix-array-bytes idx == string-to-bytes "banana" &&
        eq-nat-list (suffix-array-lookup idx (string-to-bytes "an") (0 - 1)) [3, 1]) in
    ok-good && expect-corrupt bad

  -- | Verify error messages are non-empty and distinct.
  check-error-messages :: Bool
  check-error-messages =
    let m1 = suffix-array-error-message SuffixArrayInvalidHeader in
    let m2 = suffix-array-error-message SuffixArrayTruncated in
    let m3 = suffix-array-error-message SuffixArrayCorruptIndex in
    m1 /= "" && m2 /= "" && m3 /= "" && m1 /= m2 && m2 /= m3 && m1 /= m3

  -- | Index of first failing check, or 200 if all pass.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i

  -- | Run all immutability and corruption checks.
  main :: Int
  main =
    let banana-enc = suffix-array-encode (suffix-array-new (string-to-bytes "banana")) in
    let aaaa-enc = suffix-array-encode (suffix-array-new (string-to-bytes "aaaa")) in
    -- banana: 16 header + 6 data = offset 22 for sa[0..5] = [5, 3, 1, 0, 4, 2]
    -- aaaa:   16 header + 4 data = offset 20 for sa[0..3] = [3, 2, 1, 0]
    let high-bit-len = with-byte banana-enc 15 128 in
    let width4-too-big = with-byte banana-enc 12 1 in
    let out-of-range-sa = with-le32 banana-enc 22 99 in
    let dup-sa = with-le32 banana-enc 22 3 in
    let swapped-first-sa = with-le32 (with-le32 banana-enc 22 3) 26 5 in
    let swapped-same-byte-sa = with-le32 (with-le32 banana-enc 26 1) 30 3 in
    let swapped-aaaa-sa = with-le32 (with-le32 aaaa-enc 24 1) 28 2 in
    first-fail [
      check-immutability,
      check-invalid-utf8-regex,
      check-wide-decode,
      check-error-messages,
      expect-truncated (bytes-new 0),
      expect-truncated (take-bytes banana-enc 15),
      expect-invalid-header (with-byte banana-enc 0 88),
      expect-invalid-header (with-byte banana-enc 3 50),
      expect-invalid-header (with-byte banana-enc 4 2),
      expect-invalid-header (with-byte banana-enc 5 2),
      expect-invalid-header (with-byte banana-enc 5 16),
      expect-invalid-header (with-byte banana-enc 6 1),
      expect-invalid-header (with-byte banana-enc 7 1),
      expect-invalid-header high-bit-len,
      expect-invalid-header width4-too-big,
      expect-truncated (take-bytes banana-enc 18),
      expect-truncated (take-bytes banana-enc 45),
      expect-corrupt (append-zero banana-enc),
      expect-corrupt out-of-range-sa,
      expect-corrupt dup-sa,
      expect-corrupt swapped-first-sa,
      expect-corrupt swapped-same-byte-sa,
      expect-corrupt swapped-aaaa-sa
    ] 0
"#;
    let dir = common::project("suffixarray_errors", &modules_with_main(main));
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "first failing immutability/corruption check index: {code}");
}

#[test]
fn suffix_array_large_input_performance() {
    let main = r#"-- | Large-input O(n log n) performance and correctness checks on 12,000-byte inputs.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Data.SuffixArray

  -- | Fill a buffer of length n with a single repeated byte.
  make-repeated :: Nat -> InRange 0 255 Nat -> Bytes
  make-repeated n byte =
    let out = bytes-new n in
    let u = fill-repeated out 0 n byte in
    out

  -- | Fill loop for repeated bytes.
  fill-repeated :: Bytes -> Nat -> Nat -> InRange 0 255 Nat -> Unit
  fill-repeated out i n byte =
    if i < n && i < bytes-len out then
      let u = bytes-set out i byte in
      fill-repeated out (i + 1) n byte
    else ()

  -- | Generate a deterministic pseudo-random lowercase ASCII buffer with an injected needle.
  make-prng-text :: Nat -> Bytes
  make-prng-text n =
    let out = bytes-new n in
    let u1 = fill-prng out 0 n 1234567 in
    let needle = string-to-bytes "cinchneedle" in
    let nl = bytes-len needle in
    let u2 = (if 1000 + nl <= bytes-len out then bytes-copy-range out needle 1000 0 nl else ()) in
    let u3 = (if 5000 + nl <= bytes-len out then bytes-copy-range out needle 5000 0 nl else ()) in
    let u4 = (if 11000 + nl <= bytes-len out then bytes-copy-range out needle 11000 0 nl else ()) in
    out

  -- | Fill buffer using a linear congruential generator over lowercase ASCII letters.
  fill-prng :: Bytes -> Nat -> Nat -> U64 -> Unit
  fill-prng out i n state =
    if i < n && i < bytes-len out then
      let next = wrapping-add-u64 (wrapping-mul-u64 state 6364136223846793005) 1442695040888963407 in
      let raw = u32-to-nat (wrapping-u64-to-u32 (wrapping-shr-u64 next 33)) in
      let ch = 97 + nat-rem raw 26 in
      let u = (if ch <= 255 then bytes-set out i ch else ()) in
      fill-prng out (i + 1) n next
    else ()

  -- | Verify exact reverse-order SA on a 12,000-byte constant buffer ("aaaa...").
  check-const-sa :: SuffixArray -> Nat -> Nat -> Bool
  check-const-sa idx i n =
    if i < n then
      case suffix-array-at idx i of
        None -> False
        Some got ->
          if i + 1 <= n && got == n - (i + 1) then check-const-sa idx (i + 1) n
          else False
    else True

  -- | Fill a buffer of length n with alternating bytes 'a' and 'b'.
  make-alternating :: Nat -> Bytes
  make-alternating n =
    let out = bytes-new n in
    let u = fill-alternating out 0 n in
    out

  -- | Fill loop for alternating 'a'/'b' bytes.
  fill-alternating :: Bytes -> Nat -> Nat -> Unit
  fill-alternating out i n =
    if i < n && i < bytes-len out then
      let u = (if nat-rem i 2 == 0 then bytes-set out i 97 else bytes-set out i 98) in
      fill-alternating out (i + 1) n
    else ()

  -- | Check List Nat equality.
  eq-nat-list :: List Nat -> List Nat -> Bool
  eq-nat-list xs ys = case xs of
    Nil -> case ys of Nil -> True; Cons _ _ -> False
    Cons x xrest -> case ys of
      Nil -> False
      Cons y yrest -> if x == y then eq-nat-list xrest yrest else False

  -- | Run 12,000-byte repeated, periodic alternating, and pseudo-random benchmarks plus encode/decode validation.
  main :: Int
  main =
    let n = (12000 :: Nat) in
    let const-buf = make-repeated n 97 in
    let const-idx = suffix-array-new const-buf in
    if not (check-const-sa const-idx 0 n) then 1
    else if suffix-array-count const-idx (make-repeated 100 97) /= 11901 then 2
    else case suffix-array-decode (suffix-array-encode const-idx) of
      Err _ -> 3
      Ok const-dec ->
        if not (check-const-sa const-dec 0 n) then 4
        else
          let alt-buf = make-alternating n in
          let alt-idx = suffix-array-new alt-buf in
          -- In 12,000-byte "abab...ab", "abab" occurs at every even offset 0..11996 (5999 times),
          -- and the lexicographically smallest suffix is "ab" at offset 11998.
          if suffix-array-count alt-idx (string-to-bytes "abab") /= 5999 then 5
          else case suffix-array-at alt-idx 0 of
            None -> 6
            Some first-alt ->
              if first-alt /= 11998 then 7
              else case suffix-array-decode (suffix-array-encode alt-idx) of
                Err _ -> 8
                Ok alt-dec ->
                  if suffix-array-count alt-dec (string-to-bytes "baba") /= 5998 then 9
                  else
                    let text-buf = make-prng-text n in
                    let text-idx = suffix-array-new text-buf in
                    let needle = string-to-bytes "cinchneedle" in
                    if not (eq-nat-list (suffix-array-lookup-sorted text-idx needle (0 - 1)) [1000, 5000, 11000]) then 10
                    else if suffix-array-count text-idx needle /= 3 then 11
                    else case suffix-array-decode (suffix-array-encode text-idx) of
                      Err _ -> 12
                      Ok text-dec ->
                        if eq-nat-list (suffix-array-lookup-sorted text-dec needle (0 - 1)) [1000, 5000, 11000] then 200
                        else 13
"#;
    let dir = common::project("suffixarray_perf", &modules_with_main(main));
    let exe = common::build_executable(&dir, "Main.cinch", "perf_bench")
        .expect("build large-input suffixarray benchmark");
    let start = Instant::now();
    let status = Command::new(&exe)
        .status()
        .expect("run large-input suffixarray benchmark");
    let elapsed = start.elapsed();
    eprintln!(
        "suffix_array_large_input_performance: 3x 12,000-byte (const + periodic + PRNG) build + lookup + encode/decode elapsed = {:.2} ms",
        elapsed.as_secs_f64() * 1000.0
    );
    common::cleanup(&dir);
    assert_eq!(status.code(), Some(200), "large-input benchmark failed with {:?}", status.code());
    assert!(
        elapsed.as_secs_f64() < 5.0,
        "3x 12,000-byte construction & encode/decode took {:.3}s (expected well under 5s)",
        elapsed.as_secs_f64()
    );
}

#[test]
fn suffix_array_declarations_have_canonical_documentation() {
    common::assert_documented(
        "Data.SuffixArray",
        include_str!("../prelude/Data/SuffixArray.cinch"),
    );
}
