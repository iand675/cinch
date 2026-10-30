//! P31 JSON and canonical serialization: `Text.Json` (the parser over
//! the P30 toolkit's error vocabulary, the total printer, the
//! canonical form and its SHA-256 content hash), `Text.Json`'s
//! `JsonCodec` over the A6 schema vocabulary, `Text.Args` (CLI
//! argument parsing), and the bootstrap manifest round trip: a real
//! `build/*.cinchif` through parse -> `Json` -> canonical print.
//!
//! Every program is a real `Main` compiled and run through the same
//! driver the CLI uses. The toolkit modules beside it are the checked
//! in `compiler/prelude/Text/*.cinch` sources (the driver does not
//! embed them: the P30 note). Test programs import the two toolkits
//! by name selection (`import Text.Json (parse-json, ...)`) because
//! both export the P30 combinator vocabulary; P19's qualified import
//! syntax (`import Text.Json as Json`) is the other disambiguation.
//!
//! The canonical printer's byte-level claims are pinned here against
//! exact expected texts, and the content hash is pinned against an
//! independent SHA-256 computation of the canonical document.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};

const LEX_SRC: &str = include_str!("../prelude/Text/Lex.cinch");
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");
const PARSE_SRC: &str = include_str!("../prelude/Text/Parse.cinch");
const JSON_SRC: &str = include_str!("../prelude/Text/Json.cinch");
const BUILDER_SRC: &str = include_str!("../prelude/Text/Builder.cinch");
const ARGS_SRC: &str = include_str!("../prelude/Text/Args.cinch");

// ----------------------------------------------------------------
// Shared harness
// ----------------------------------------------------------------

/// A self-contained project: the toolkit sources beside a `Main`.
fn write_project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!(
        "cinch_json_{}_{}_{}",
        name,
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Text.Lex.cinch"), LEX_SRC).expect("write Text.Lex");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN_SRC).expect("write Text.Scan");
    std::fs::write(dir.join("Text.Parse.cinch"), PARSE_SRC).expect("write Text.Parse");
    std::fs::write(dir.join("Text.Json.cinch"), JSON_SRC).expect("write Text.Json");
    std::fs::write(dir.join("Text.Builder.cinch"), BUILDER_SRC).expect("write Text.Builder");
    std::fs::write(dir.join("Text.Args.cinch"), ARGS_SRC).expect("write Text.Args");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).unwrap_or_else(|e| panic!("write {name}: {e}"));
    }
    dir
}

/// Run a driver invocation on a thread with a large stack (the
/// checker recurses over a program's elaborated code; the toolkit
/// programs are bigger than a default test thread's stack).
fn drive(mode: Mode, entry: &Path, out: Option<&Path>) -> i32 {
    let entry = entry.to_path_buf();
    let out = out.map(|p| p.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &entry, out.as_deref()))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .unwrap_or_else(|e| panic!("compilation failed: {e}"))
}

/// Compile and run a `Main`; the exit code is the program's
/// `main :: Int` result (0 on success, the failing case number on a
/// named failure).
fn run_main(name: &str, main: &str) -> i32 {
    let dir = write_project(name, &[("main.cinch", main)]);
    let code = drive(Mode::Run, &dir.join("main.cinch"), None);
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
    let code = drive(Mode::Build, &dir.join("main.cinch"), Some(&exe));
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

// ----------------------------------------------------------------
// Programs (the IO shape: read the document from the input source,
// write the result to the output sink)
// ----------------------------------------------------------------

/// stdin -> parse -> canonical print -> stdout; the error bundle on
/// failure (exit 1).
const CANONICAL_IO: &str = r#"module Main where
  import Text.Json (parse-json, canonical-json)
  import Text.Parse (ParseOutcome, Parsed, Failed, render-bundle)
  import Data.Bytes
  import Data.List
  import Data.Nat

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case read-all-chunks 4096 Nil of
    Nil -> case run-doc (bytes-new 0) of
      n -> n
    Cons c rest -> case run-doc (concat-bytes (Cons c rest)) of
      n -> n

  run-doc :: forall r. using (io :: Io r) => Bytes ->{IO r, Diverge} Int
  run-doc src = case parse-json "<stdin>" src of
    Failed b -> case ioWrite (string-to-bytes (render-bundle b)) of
      w -> 1
    Parsed v soft -> case ioWrite (string-to-bytes (canonical-json v)) of
      w -> 0

  read-all-chunks :: forall r. using (io :: Io r) => Nat -> List Bytes ->{IO r, Diverge} List Bytes
  read-all-chunks n acc = case ioRead n of
    None -> rev-bytes acc
    Some b -> read-all-chunks n (Cons b acc)

  rev-bytes :: List Bytes -> List Bytes
  rev-bytes xs = rev-bytes-acc xs Nil

  rev-bytes-acc :: List Bytes -> List Bytes -> List Bytes
  rev-bytes-acc xs acc = case xs of
    Nil -> acc
    Cons y ys -> rev-bytes-acc ys (Cons y acc)

  concat-bytes :: List Bytes -> Bytes
  concat-bytes cs = case cs of
    Nil -> bytes-new 0
    Cons c rest -> cat-bytes c (concat-bytes rest)

  cat-bytes :: Bytes -> Bytes -> Bytes
  cat-bytes a b =
    let out = bytes-new (bytes-len a + bytes-len b) in
    let u = copy-bytes-into out a 0 0 (bytes-len a) in
    let v = copy-bytes-into out b (bytes-len a) 0 (bytes-len b) in
    out
"#;

/// stdin -> parse -> canonical-hash -> stdout (the hex digest).
const DIGEST_IO: &str = r#"module Main where
  import Text.Json (parse-json, canonical-hash)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Data.Bytes
  import Data.List
  import Data.Nat

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case read-all-chunks 4096 Nil of
    Nil -> case run-doc (bytes-new 0) of
      n -> n
    Cons c rest -> case run-doc (concat-bytes (Cons c rest)) of
      n -> n

  run-doc :: forall r. using (io :: Io r) => Bytes ->{IO r, Diverge} Int
  run-doc src = case parse-json "<stdin>" src of
    Failed b -> 1
    Parsed v soft -> case ioWrite (string-to-bytes (canonical-hash v)) of
      w -> 0

  read-all-chunks :: forall r. using (io :: Io r) => Nat -> List Bytes ->{IO r, Diverge} List Bytes
  read-all-chunks n acc = case ioRead n of
    None -> rev-bytes acc
    Some b -> read-all-chunks n (Cons b acc)

  rev-bytes :: List Bytes -> List Bytes
  rev-bytes xs = rev-bytes-acc xs Nil

  rev-bytes-acc :: List Bytes -> List Bytes -> List Bytes
  rev-bytes-acc xs acc = case xs of
    Nil -> acc
    Cons y ys -> rev-bytes-acc ys (Cons y acc)

  concat-bytes :: List Bytes -> Bytes
  concat-bytes cs = case cs of
    Nil -> bytes-new 0
    Cons c rest -> cat-bytes c (concat-bytes rest)

  cat-bytes :: Bytes -> Bytes -> Bytes
  cat-bytes a b =
    let out = bytes-new (bytes-len a + bytes-len b) in
    let u = copy-bytes-into out a 0 0 (bytes-len a) in
    let v = copy-bytes-into out b (bytes-len a) 0 (bytes-len b) in
    out
"#;

// ----------------------------------------------------------------
// The grammar end to end (stdin corpus against pinned canonical
// texts) and the error bundles
// ----------------------------------------------------------------

/// The whole RFC 8259 grammar through the executable: each input
/// parses and canonically prints to the pinned text.
#[test]
fn json_parser_covers_the_grammar_end_to_end() {
    let (dir, exe) = build_exe("grammar-main", CANONICAL_IO);
    // (input, expected canonical text): objects, arrays, strings with
    // every escape (including `\uXXXX` and the surrogate pair form),
    // numbers (integer, float, exponent, negative zero, bignum),
    // true/false/null, nesting, and insignificant whitespace.
    let corpus: &[(&str, &str)] = &[
        (r#"null"#, "null"),
        ("true", "true"),
        ("false", "false"),
        ("0", "0"),
        ("-0", "0"),
        ("42", "42"),
        ("-7", "-7"),
        ("-1234", "-1234"),
        ("98765432109876543210", "98765432109876543210"),
        ("1000000000000000000000000000000000001", "1000000000000000000000000000000000001"),
        ("2.5", "2.5e0"),
        ("0.5", "5e-1"),
        ("1.0", "1e0"),
        ("1E2", "1e2"),
        ("-1.5e-10", "-1.5e-10"),
        ("  \t\r\n42\n ", "42"),
        ("\"\"", "\"\""),
        (r#""hello""#, "\"hello\""),
        (r#""\"\\\/\b\f\n\r\t""#, "\"\\\"\\\\/\\b\\f\\n\\r\\t\""),
        (r#""Aé""#, "\"Aé\""),
        (r#""𝄞""#, "\"\u{1D11E}\""),
        ("\"raw é text\"", "\"raw é text\""),
        ("[]", "[]"),
        ("[ ]", "[]"),
        ("[1, 2.5, true, null]", "[1,2.5e0,true,null]"),
        ("[ 1 , [ 2 , [ 3 ] ] ]", "[1,[2,[3]]]"),
        ("{}", "{}"),
        ("{ }", "{}"),
        (r#"{"b":1,"a":2}"#, "{\"a\":2,\"b\":1}"),
        (r#"{"z":{"y":1,"x":2},"a":[true]}"#, "{\"a\":[true],\"z\":{\"x\":2,\"y\":1}}"),
        (r#"{"esc":"a\"b"}"#, "{\"esc\":\"a\\\"b\"}"),
    ];
    for (input, expected) in corpus {
        let (code, out) = run_exe(&exe, input.as_bytes());
        assert_eq!(code, 0, "{input}: parse failed: {out}");
        assert_eq!(out, *expected, "{input}: canonical rendering");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// Malformed input reports a real bundle: the `unexpected` and
/// `expecting` lines of `render-bundle`, and the named refusals.
#[test]
fn json_parser_reports_unexpected_and_expecting() {
    let (dir, exe) = build_exe("errors-main", CANONICAL_IO);
    // Cases whose bundle must carry the `unexpected`/`expecting`
    // rendering (the megaparsec shape).
    let needles: &[&str] = &[
        "{",
        "[1,]",
        r#"{"a"}"#,
        r#"{"a" 1}"#,
        r#"{"a":}"#,
        "tru",
        "\"unterminated",
        "x",
        "{\"a\":1} x",
        "1.",
        "[1 2]",
        r#"{"a": tru}"#,
    ];
    for input in needles {
        let (code, out) = run_exe(&exe, input.as_bytes());
        assert_eq!(code, 1, "{input}: expected a failed parse, got {code}: {out}");
        assert!(out.contains("unexpected"), "{input}: no unexpected line in:\n{out}");
        assert!(out.contains("expecting"), "{input}: no expecting line in:\n{out}");
    }
    // The named refusals (fancy errors): each message names itself.
    let named: &[(&str, &str)] = &[
        ("01", "leading zero in a JSON number"),
        (r#"{"a": 01}"#, "leading zero in a JSON number"),
        (r#"{"a":1,"a":2}"#, "duplicate object key"),
        (r#""\udc00""#, "unpaired low surrogate in a string escape"),
        (r#""\ud834x""#, "expected"),
    ];
    for (input, needle) in named {
        let (code, out) = run_exe(&exe, input.as_bytes());
        assert_eq!(code, 1, "{input}: expected a failed parse, got {code}: {out}");
        assert!(out.contains(needle), "{input}: no {needle:?} in:\n{out}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ----------------------------------------------------------------
// Round trip and canonical form (the pure probes)
// ----------------------------------------------------------------

/// The round trip (the P31 acceptance): `parse . print . parse` is
/// the identity on the corpus, and the second print is byte
/// identical to the first.
const ROUND_TRIP_PROBE: &str = r#"module Main where
  import Text.Json (parse-json, print-json, canonical-json, json-eq)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Data.Bytes
  import Data.Json
  import Data.Int
  import Data.Machine
  import Data.Nat
  import Data.String

  first-bad :: List Nat -> Nat
  first-bad xs = case xs of
    Nil -> 0
    Cons x rest -> if x == 0 then first-bad rest else x

  int-of :: Nat -> Int
  int-of n = nat-to-int n

  -- Parse, print, parse again: the values agree and the second print
  -- is the first print byte for byte. A parse failure is `n + 50`.
  round-trip :: Nat -> String -> Nat
  round-trip n text = case parse-json "<probe>" (string-to-bytes text) of
    Failed b -> n + 50
    Parsed v soft -> case parse-json "<probe>" (string-to-bytes (print-json v)) of
      Failed b2 -> n + 50
      Parsed v2 soft2 ->
        if json-eq v v2 then
          (if string-compare (print-json v) (print-json v2) == 0 then 0 else n)
        else n

  eq-same-arr :: Nat
  eq-same-arr = if json-eq (JArr (JNum 1) JArrEnd) (JArr (JNum 1) JArrEnd) then 0 else 1

  eq-diff-arr :: Nat
  eq-diff-arr = if json-eq (JArr (JNum 1) JArrEnd) (JArr (JNum 2) JArrEnd) then 2 else 0

  eq-diff-key :: Nat
  eq-diff-key = if json-eq (JObj "a" (JNum 1) JObjEnd) (JObj "b" (JNum 1) JObjEnd) then 3 else 0

  eq-same-obj :: Nat
  eq-same-obj = if json-eq (JObj "a" (JNum 1) JObjEnd) (JObj "a" (JNum 1) JObjEnd) then 0 else 4

  eq-cross-family :: Nat
  eq-cross-family = if json-eq JNull (JBool True) then 5 else 0

  eq-same-str :: Nat
  eq-same-str = if json-eq (JStr "x") (JStr "x") then 0 else 6

  eq-float :: Nat
  eq-float = if json-eq (JFloat (f64-from-decimal (string-to-bytes "2.5"))) (JFloat (f64-from-decimal (string-to-bytes "2.5"))) then 0 else 7

  rt1 :: Nat
  rt1 = round-trip 1 "[1, 2.5, true, null]"

  rt2 :: Nat
  rt2 = round-trip 2 "{\"b\":1,\"a\":2}"

  rt3 :: Nat
  rt3 = round-trip 3 "\"\\u00e9\\u0041\\/\\b\\f\\n\\r\\t\\\\\\\"\""

  rt4 :: Nat
  rt4 = round-trip 4 "\"\\ud834\\udd1e\""

  rt5 :: Nat
  rt5 = round-trip 5 "-1.5e-10"

  rt6 :: Nat
  rt6 = round-trip 6 "1000000000000000000000000000000000001"

  rt7 :: Nat
  rt7 = round-trip 7 "{\"a\":[1,2.5,{\"b\":\"x\"}],\"c\":null}"

  rt8 :: Nat
  rt8 = round-trip 8 "[[[]]]"

  rt9 :: Nat
  rt9 = round-trip 9 "0"

  rt10 :: Nat
  rt10 = round-trip 10 "1E2"

  main :: Int
  main = int-of (rt1 + rt2 + rt3 + rt4 + rt5 + rt6 + rt7 + rt8 + rt9 + rt10 + eq-same-arr + eq-diff-arr + eq-diff-key + eq-same-obj + eq-cross-family + eq-same-str + eq-float)
"#;

/// The canonical form (the P31 acceptance): sorted keys, stable
/// numbers, fixed whitespace, byte-identical repeats, the fixed-point
/// property, and the non-finite rendering.
const CANONICAL_PROBE: &str = r#"module Main where
  import Text.Json (parse-json, print-json, canonical-json)
  import Text.Parse (ParseOutcome, Parsed, Failed)
  import Data.Bytes
  import Data.Json
  import Data.Int
  import Data.Machine
  import Data.Nat
  import Data.String

  first-bad :: List Nat -> Nat
  first-bad xs = case xs of
    Nil -> 0
    Cons x rest -> if x == 0 then first-bad rest else x

  int-of :: Nat -> Int
  int-of n = nat-to-int n

  -- The pinned canonical rendering (sorted keys, stable numbers, no
  -- insignificant whitespace). A parse failure is `n + 50`.
  golden :: Nat -> String -> String -> Nat
  golden n text want = case parse-json "<probe>" (string-to-bytes text) of
    Failed b -> n + 50
    Parsed v soft ->
      let got = canonical-json v in
      if string-compare got want == 0 then
        -- two prints of one value are byte identical
        (if string-compare got (canonical-json v) == 0 then 0 else n)
      else n

  -- The canonical form is a fixed point: printing the parsed
  -- canonical text again is the same bytes.
  fixed-point :: Nat -> String -> Nat
  fixed-point n text = case parse-json "<probe>" (string-to-bytes text) of
    Failed b -> n + 50
    Parsed v soft ->
      let c1 = canonical-json v in
      case parse-json "<probe>" (string-to-bytes c1) of
        Failed b2 -> n + 50
        Parsed v2 soft2 ->
          if string-compare c1 (canonical-json v2) == 0 then 0 else n

  -- The printer keeps entry order where the canonical form sorts it.
  entry-order :: Nat
  entry-order = case parse-json "<probe>" (string-to-bytes "{\"b\":1,\"a\":2}") of
    Failed b -> 20
    Parsed v soft ->
      if string-compare (print-json v) "{\"b\":1,\"a\":2}" == 0 then 0 else 20

  -- A non-finite float renders `null` (the named boundary).
  non-finite-nan :: Nat
  non-finite-nan =
    let zero = f64-from-decimal (string-to-bytes "0") in
    if string-compare (print-json (JFloat (div-f64 zero zero))) "null" == 0 then 0 else 1

  non-finite-inf :: Nat
  non-finite-inf =
    let zero = f64-from-decimal (string-to-bytes "0") in
    let one = f64-from-decimal (string-to-bytes "1") in
    if string-compare (canonical-json (JFloat (div-f64 one zero))) "null" == 0 then 0 else 2

  g1 :: Nat
  g1 = golden 1 "{\"b\":1,\"a\":2}" "{\"a\":2,\"b\":1}"

  g2 :: Nat
  g2 = golden 2 "{\"z\":{\"y\":1,\"x\":2}}" "{\"z\":{\"x\":2,\"y\":1}}"

  g3 :: Nat
  g3 = golden 3 "2.5" "2.5e0"

  g4 :: Nat
  g4 = golden 4 "0.5" "5e-1"

  g5 :: Nat
  g5 = golden 5 "-0" "0"

  g6 :: Nat
  g6 = golden 6 "[ ]" "[]"

  g7 :: Nat
  g7 = golden 7 "{ }" "{}"

  g8 :: Nat
  g8 = golden 8 " -1.5e-10 " "-1.5e-10"

  g9 :: Nat
  g9 = golden 9 "42" "42"

  g10 :: Nat
  g10 = golden 10 "-1234" "-1234"

  -- A control character with no named escape prints `\u00` then its
  -- high nibble then its low one; swapped nibbles still look like a
  -- valid escape (`\u0010`, `\u00f1`) and name a different character.
  g11 :: Nat
  g11 = golden 11 "\"\\u0001\\u001f\"" "\"\\u0001\\u001f\""

  fp1 :: Nat
  fp1 = fixed-point 20 "{\"a\":[1,2.5e0],\"b\":\"x\"}"

  fp2 :: Nat
  fp2 = fixed-point 21 "\"\\u00e9A/\\b\\f\\n\\r\\t\\\\\\\"\""

  main :: Int
  main = int-of (g1 + g2 + g3 + g4 + g5 + g6 + g7 + g8 + g9 + g10 + g11 + fp1 + fp2 + entry-order + non-finite-nan + non-finite-inf)
"#;

// ----------------------------------------------------------------
// JsonCodec: the Text.Codec round-trip law over the A6 covered set
// ----------------------------------------------------------------

/// `Text.Codec`'s law shape (`json_codec_round_trips_the_d65_set`):
/// `decode-json (encode-json x)` is `Ok x` over decision D65's
/// covered set (`I64`, `Bool`), with the named decode errors.
const CODEC_PROBE: &str = r#"module Main where
  import Text.Json (json-eq)
  import Text.Codec (JsonCodec, encode-json, decode-json, JsonDecodeError, json-error-message)
  import Data.Bytes
  import Data.Json
  import Data.Int
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String

  first-bad :: List Nat -> Nat
  first-bad xs = case xs of
    Nil -> 0
    Cons x rest -> if x == 0 then first-bad rest else x

  int-of :: Nat -> Int
  int-of n = nat-to-int n

  decode-i64 :: Json -> Result JsonDecodeError I64
  decode-i64 j = decode-json j

  decode-bool :: Json -> Result JsonDecodeError Bool
  decode-bool j = decode-json j

  decode-json-json :: Json -> Result JsonDecodeError Json
  decode-json-json j = decode-json j

  ok-i64 :: Result JsonDecodeError I64 -> I64 -> Bool
  ok-i64 r x = case r of
    Err e -> False
    Ok y -> eq-i64 y x

  ok-bool :: Result JsonDecodeError Bool -> Bool -> Bool
  ok-bool r x = case r of
    Err e -> False
    Ok y -> y == x

  is-err :: forall a. Result JsonDecodeError a -> Bool
  is-err r = case r of
    Err e -> True
    Ok y -> False

  -- The law over the covered set: `decode-json (encode-json x)` is
  -- `Ok x`. A failure is `n`.
  law-i64 :: Nat -> I64 -> Nat
  law-i64 n x = if ok-i64 (decode-i64 (encode-json x)) x then 0 else n

  law-bool :: Nat -> Bool -> Nat
  law-bool n x = if ok-bool (decode-bool (encode-json x)) x then 0 else n

  law-bool-true :: Nat
  law-bool-true = law-bool 1 True

  law-bool-false :: Nat
  law-bool-false = law-bool 2 False

  law-zero :: Nat
  law-zero = law-i64 3 0

  law-one :: Nat
  law-one = law-i64 4 1

  law-neg :: Nat
  law-neg = law-i64 5 (wrapping-neg-i64 1)

  law-min :: Nat
  law-min = law-i64 6 min-bound-i64

  law-max :: Nat
  law-max = law-i64 7 max-bound-i64

  law-shift :: Nat
  law-shift = law-i64 8 (wrapping-shl-i64 1 56)

  identity-column :: Nat
  identity-column = case decode-json-json JNull of
    Err e -> 9
    Ok j -> if json-eq j JNull then 0 else 9

  err-shape :: Nat
  err-shape = if is-err (decode-i64 (JStr "x")) then 0 else 10

  err-bool :: Nat
  err-bool = if is-err (decode-bool (JNum 1)) then 0 else 11

  err-float :: Nat
  err-float = if is-err (decode-i64 (JFloat (f64-from-decimal (string-to-bytes "2.5")))) then 0 else 12

  err-big :: Nat
  err-big = if is-err (decode-i64 (JNum (int-mul (int-from-i64 max-bound-i64) (int-from-i64 2)))) then 0 else 13

  err-small :: Nat
  err-small = if is-err (decode-i64 (JNum (int-mul (int-from-i64 min-bound-i64) (int-from-i64 2)))) then 0 else 14

  err-message :: Nat
  err-message = case decode-i64 (JStr "x") of
    Err e -> if string-compare (json-error-message e) "expected integer" == 0 then 0 else 15
    Ok y -> 15

  main :: Int
  main = int-of (law-bool-true + law-bool-false + law-zero + law-one + law-neg + law-min + law-max + law-shift + identity-column + err-shape + err-bool + err-float + err-big + err-small + err-message)
"#;

// ----------------------------------------------------------------
// Text.Args: named errors and the render round trip
// ----------------------------------------------------------------

/// CLI argument parsing (the P31 acceptance): the argv walk, the
/// named errors for unknown and missing arguments, the `--`
/// terminator, and the `render-argv` round-trip probe.
const ARGS_PROBE: &str = r#"module Main where
  import Text.Args
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String

  specs :: List ArgSpec
  specs = Cons (Required (ValueSpec "out")) (Cons (FlagSpec "verbose") (Cons (ValueSpec "name") Nil))

  first-bad :: List Nat -> Nat
  first-bad xs = case xs of
    Nil -> 0
    Cons x rest -> if x == 0 then first-bad rest else x

  int-of :: Nat -> Int
  int-of n = nat-to-int n

  err-is :: ArgsError -> ArgsError -> Bool
  err-is a b = case a of
    UnknownOption x -> case b of
      UnknownOption y -> string-compare x y == 0
      _ -> False
    MissingValue x -> case b of
      MissingValue y -> string-compare x y == 0
      _ -> False
    UnexpectedValue x -> case b of
      UnexpectedValue y -> string-compare x y == 0
      _ -> False
    MissingRequired x -> case b of
      MissingRequired y -> string-compare x y == 0
      _ -> False

  eq-args :: Args -> Args -> Bool
  eq-args a b = eq-opts (args-options a) (args-options b) && eq-strs (args-values a) (args-values b)

  eq-opts :: List OptVal -> List OptVal -> Bool
  eq-opts x y = case x of
    Nil -> case y of
      Nil -> True
      _ -> False
    Cons o os -> case y of
      Nil -> False
      Cons p ps -> case o of
        OptVal n1 v1 -> case p of
          OptVal n2 v2 ->
            if string-compare n1 n2 /= 0 then False
            else case v1 of
              None -> case v2 of
                None -> eq-opts os ps
                Some z -> False
              Some s1 -> case v2 of
                None -> False
                Some s2 -> if string-compare s1 s2 == 0 then eq-opts os ps else False

  eq-strs :: List String -> List String -> Bool
  eq-strs x y = case x of
    Nil -> case y of
      Nil -> True
      _ -> False
    Cons a xs -> case y of
      Nil -> False
      Cons b bs -> if string-compare a b == 0 then eq-strs xs bs else False

  good-args :: Args
  good-args = Args (Cons (OptVal "out" (Some "bin")) (Cons (OptVal "verbose" None) Nil)) (Cons "file.cinch" Nil)

  check-argv-words :: Nat
  check-argv-words = case parse-args specs (Cons "--out" (Cons "bin" (Cons "--verbose" (Cons "file.cinch" Nil)))) of
    Err e -> 1
    Ok a -> if eq-args a good-args then 0 else 1

  check-inline-value :: Nat
  check-inline-value = case parse-args specs (Cons "--out=bin" (Cons "--verbose" (Cons "file.cinch" Nil))) of
    Err e -> 2
    Ok a -> if eq-args a good-args then 0 else 2

  check-unknown :: Nat
  check-unknown = case parse-args specs (Cons "--bogus" Nil) of
    Ok a -> 3
    Err e -> if err-is e (UnknownOption "bogus") then 0 else 3

  check-missing-value :: Nat
  check-missing-value = case parse-args specs (Cons "--out" Nil) of
    Ok a -> 4
    Err e -> if err-is e (MissingValue "out") then 0 else 4

  check-flag-value :: Nat
  check-flag-value = case parse-args specs (Cons "--verbose=1" (Cons "--out" (Cons "x" Nil))) of
    Ok a -> 5
    Err e -> if err-is e (UnexpectedValue "verbose") then 0 else 5

  check-missing-required :: Nat
  check-missing-required = case parse-args specs (Cons "--verbose" Nil) of
    Ok a -> 6
    Err e -> if err-is e (MissingRequired "out") then 0 else 6

  check-terminator :: Nat
  check-terminator = case parse-args specs (Cons "--out" (Cons "bin" (Cons "--" (Cons "--weird" (Cons "z" Nil))))) of
    Err e -> 7
    Ok a -> if eq-args a (Args (Cons (OptVal "out" (Some "bin")) Nil) (Cons "--weird" (Cons "z" Nil))) then 0 else 7

  check-error-text :: Nat
  check-error-text =
    if string-compare (render-args-error (UnknownOption "bogus")) "unknown option --bogus" == 0 then
      (if string-compare (render-args-error (MissingRequired "out")) "missing required argument --out" == 0 then 0 else 8)
    else 8

  check-render-round-trip :: Nat
  check-render-round-trip = case parse-args specs (render-argv good-args) of
    Err e -> 9
    Ok a -> if eq-args a good-args then 0 else 9

  check-list :: List Nat
  check-list = Cons check-argv-words (Cons check-inline-value (Cons check-unknown (Cons check-missing-value
    (Cons check-flag-value (Cons check-missing-required (Cons check-terminator (Cons check-error-text
    (Cons check-render-round-trip Nil))))))))

  main :: Int
  main = int-of (first-bad check-list)
"#;

// ----------------------------------------------------------------
// The content hash and the bootstrap manifest round trip
// ----------------------------------------------------------------

/// The digest of a canonical document is pinned (an independent
/// SHA-256 of the canonical text) and stable across runs.
#[test]
fn canonical_hash_is_stable_and_pinned() {
    let (dir, exe) = build_exe("digest-main", DIGEST_IO);
    // The canonical text of this input is
    // {"a":[1,2.5e0,true,null],"b":"x"}; the digest is pinned against
    // an independent SHA-256 computation of those exact bytes.
    let input = br#"{"a":[1,2.5,true,null],"b":"x"}"#;
    let pinned = "b797e90889e23382dbcea8cc56cfca2c0cb798f15ce674a86a3e381ca085b628";
    let (code, out) = run_exe(&exe, input);
    assert_eq!(code, 0, "digest run failed: {out}");
    assert_eq!(out, pinned, "the pinned digest of the canonical document");
    let (code2, out2) = run_exe(&exe, input);
    assert_eq!(code2, 0);
    assert_eq!(out, out2, "the hash must be stable across runs");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The bootstrap milestone (P35's manifest half): a real
/// `build/*.cinchif` file round trips byte identically through parse
/// -> `Json` -> canonical print, keeps every manifest fact, and
/// hashes stably.
#[test]
fn bootstrap_manifest_round_trip_is_byte_identical() {
    // A real two-module project with data, instances and values to
    // fill the interface tables.
    let mdir = write_project(
        "manifest-src",
        &[
            (
                "Lib.cinch",
                "module Lib where\n\
                 \x20 export Token, Tok, describe, peek\n\
                 \x20 data Token where\n\
                 \x20   Tok :: I64 -> Token\n\
                 \x20 describe :: Token -> I64\n\
                 \x20 describe t = case t of\n\
                 \x20   Tok n -> n\n\
                 \x20 peek :: Token -> Token\n\
                 \x20 peek t = case t of\n\
                 \x20   Tok n -> Tok n\n\
                 \x20 foreign import ccall \"cinch.prim.eqI64\" primEqI64 :: I64 -> I64 -> Bool\n\
                 \x20 implicit instance Eq_Token :: Eq Token where\n\
                 \x20   (==) a b = case a of\n\
                 \x20     Tok x -> case b of\n\
                 \x20       Tok y -> primEqI64 x y\n\
                 \x20   (/=) a b = case a of\n\
                 \x20     Tok x -> case b of\n\
                 \x20       Tok y -> not (primEqI64 x y)\n",
            ),
            (
                "Main.cinch",
                "module Main where\n\
                 \x20 import Lib\n\
                 \x20 main :: Int\n\
                 \x20 main = if Lib.describe (Lib.Tok 7) == 7 then 0 else 1\n",
            ),
        ],
    );
    let code = drive(Mode::Check, &mdir.join("Main.cinch"), None);
    assert_eq!(code, 0, "the manifest project must check");
    let original =
        std::fs::read(mdir.join("build/Main.cinchif")).expect("read build/Main.cinchif");
    let original_text = String::from_utf8(original.clone()).expect("manifest is UTF-8");

    let (dir, exe) = build_exe("manifest-round", CANONICAL_IO);
    let (c1, out1) = run_exe(&exe, &original);
    assert_eq!(c1, 0, "the manifest must parse: {out1}");
    let (c2, out2) = run_exe(&exe, &original);
    assert_eq!(c2, 0);
    assert_eq!(out1, out2, "the same input must print the same bytes");

    // The round trip is byte identical: the canonical form of a real
    // manifest is a fixed point of parse then canonical print.
    let (c3, out3) = run_exe(&exe, out1.as_bytes());
    assert_eq!(c3, 0);
    assert_eq!(out3, out1, "the canonical manifest must round trip byte identically");
    // The compiler already writes the canonical form, so the Cinch
    // printer reproduces the artifact's own bytes.
    assert_eq!(out1, original_text, "the written interface is already canonical");

    // No manifest fact is lost: the interface parses back to the same
    // canonical projection on both sides.
    let before =
        cinch::interface::Iface::from_json(&original_text).expect("parse the written iface");
    let after = cinch::interface::Iface::from_json(&out1).expect("parse the canonical iface");
    assert_eq!(
        before.to_json(),
        after.to_json(),
        "canonicalization must not lose manifest facts"
    );

    // The content identity is stable across runs.
    let (ddir, dexe) = build_exe("manifest-digest", DIGEST_IO);
    let (d0, digest0) = run_exe(&dexe, out1.as_bytes());
    let (d1, digest1) = run_exe(&dexe, out1.as_bytes());
    assert_eq!(d0, 0);
    assert_eq!(d1, 0);
    assert_eq!(digest0, digest1, "the manifest hash must be stable across runs");
    assert_eq!(digest0.len(), 64, "a hex SHA-256 digest: {digest0}");

    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&ddir);
    let _ = std::fs::remove_dir_all(&mdir);
}

// ----------------------------------------------------------------
// The law and probe tests (pure `main :: Int` programs)
// ----------------------------------------------------------------

/// The round trip (the P31 acceptance): `parse . print . parse` is
/// the identity over the corpus, with byte-identical repeat prints.
#[test]
fn json_round_trip_parse_print_parse_is_identity() {
    expect_exit("round-trip-probe", ROUND_TRIP_PROBE, 0);
}

/// The canonical form (the P31 acceptance): sorted keys, stable
/// numbers, fixed whitespace, byte-identical repeats, and the
/// fixed-point property.
#[test]
fn canonical_form_is_sorted_stable_and_byte_identical() {
    expect_exit("canonical-probe", CANONICAL_PROBE, 0);
}

/// JsonCodec's round-trip law over the D65 covered set, named like
/// `Text.Codec`'s (`codec_round_trips_the_d65_set`).
#[test]
fn json_codec_round_trips_the_d65_set() {
    expect_exit("codec-round-trip", CODEC_PROBE, 0);
}

/// Text.Args: named errors for unknown and missing arguments and the
/// render round trip.
#[test]
fn args_parsing_names_errors_and_round_trips() {
    expect_exit("args-probe", ARGS_PROBE, 0);
}

/// The canonical printer prints `200 * (argument count)` small objects,
/// so one build measures two sizes.
const SCALING_PROBE: &str = r#"module Main where
  import Text.Json (canonical-bytes)
  import Data.Json
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Effect.Process

  -- Keys out of order, so the canonical sort runs; a quote, so the
  -- string escaping runs.
  item :: Nat -> Json
  item n =
    let k = nat-to-int n in
    JObj "name" (JStr "item") (JObj "id" (JNum k) (JObj "ok" (JBool True) (JObj "tags" (JArr (JStr "a\"b") (JArr (JNum k) JArrEnd)) JObjEnd)))

  items :: Nat -> Json -> Json
  items n acc = if n == 0 then acc else items (n - 1) (JArr (item n) acc)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in case process-args () of
    Err e -> 2
    Ok got -> if bytes-len (canonical-bytes (items (200 * list-length got) JArrEnd)) == 0 then 1 else 0
"#;

/// The canonical printer's cost is linear in the document: four times
/// the objects allocate about four times the bytes (`CINCH_ALLOC_STATS`
/// counts every allocation, so the ratio is deterministic, not a timing).
/// Concatenating each piece onto the rendering of the rest, the shape
/// this printer replaced, copied the built suffix at every step and
/// showed the ratio of a quadratic, well above 10 at these sizes.
#[test]
fn canonical_printing_allocates_linearly_in_the_document() {
    let (dir, exe) = build_exe("canonical-scaling", SCALING_PROBE);
    let alloc_bytes = |args: &[&str]| -> u64 {
        let out = std::process::Command::new(&exe)
            .args(args)
            .env("CINCH_ALLOC_STATS", "1")
            .env("CINCH_HEAP_LIMIT", "1G")
            .output()
            .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()));
        assert_eq!(out.status.code(), Some(0), "{out:?}");
        let stderr = String::from_utf8_lossy(&out.stderr);
        let line = stderr
            .lines()
            .find(|l| l.starts_with("cinch-alloc:"))
            .unwrap_or_else(|| panic!("no cinch-alloc line: {stderr}"));
        line.split_whitespace()
            .find_map(|w| w.strip_prefix("bytes="))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| panic!("no bytes= in {line:?}"))
    };
    let small = alloc_bytes(&["x"]);
    let large = alloc_bytes(&["x", "x", "x", "x"]);
    let ratio = large as f64 / small as f64;
    assert!(
        ratio < 6.0,
        "4x the objects allocated {ratio:.2}x the bytes ({small} -> {large}); linear is about 4"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
