//! Text.Builder consumer semantics: an append-cheap text and byte builder whose pieces are a spine, flattened once. Every claim is observed through a real program compiled and run by `cinch::driver`, never source text: each check is a top-level declaration returning 0 or its bit, and `main` sums them, so a nonzero exit code is a bitmask naming the failed checks (check k contributes bit 2^(k-1)). Because the driver returns the compiled program's process exit status, which is 8 BITS WIDE, every program keeps its checks to at most 8 (bits 1..128) and a program that needs more is split, so no failing check is ever masked off. `Text.Builder` is a NEW module, not yet in `cinch-base.cinchpkg`, so each program writes it beside the `Main` that imports it, exactly as `lib_text_ops.rs` ships `Text.Scan`; `Data.String`/`Text.Show`/`Data.Bytes` ARE embedded, so no copy of them is written. The checks are top-level because a `case` in a `let` binder's right-hand side must indent past the binder's column under the layout rules.
//!
//! The four tests are the four acceptance aspects: a differential oracle against `string-concat` over a large document (empty pieces, embedded newlines, large numbers and non-ASCII through `Data.Char`); the structural claim that N appends leave N chunks and the flattened byte length equals `builder-size`; a few-thousand-line scale case asserted on content properties (newline count, total length, a known line), never on wall-clock time; and an LLVM-module-shaped emission whose exact text is pinned for a small N.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const BUILDER: &str = include_str!("../prelude/Text/Builder.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_builder_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Text.Builder.cinch", BUILDER), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let path = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &path, None))
        .expect("spawn compiler thread")
        .join()
        .expect("compiler thread panicked");
    match compiled {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, &program(body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The shared declarations every program uses: string equality, the document generators (a 400-piece list exercised by the oracle, a 3000-line scale builder, the LLVM-shaped module), and the little walkers the content properties need (a `Char` counter and a prefix comparison). All top-level so a `case` never sits in a `let` binder's right-hand side.
const HELPERS: &str = r#"  same-str :: String -> String -> Bool
  same-str a b = string-compare a b == 0

  -- One non-ASCII scalar (U+4E2D) through Data.Char, never a literal, so the
  -- non-ASCII coverage rides on the tested conversion the spec names.
  non-ascii-char :: String
  non-ascii-char = case char-from-code 20013 of
    Some c -> char-string c
    None -> ""

  -- The pieces of the large document, one per index 1..k, cycling a shape per
  -- index modulo 7: an EMPTY piece, an embedded newline, a large Nat, a
  -- negative large Int, a non-ASCII scalar, a tab-and-number line, and a short
  -- non-ASCII run. Every piece is a `String`; the cycle is deterministic.
  piece-at :: Nat -> String
  piece-at k =
    let m = nat-rem k 7 in
    if m == 0 then ""
    else if m == 1 then "alpha\n"
    else if m == 2 then string-concat "n=" (nat-to-string (k * 1000000000000))
    else if m == 3 then string-concat "i=" (int-to-string (int-neg (nat-to-int (k * 9000000000))))
    else if m == 4 then non-ascii-char
    else if m == 5 then string-concat "\t" (string-concat (nat-to-string k) "\n")
    else string-repeat 3 "\u{e9}"

  pieces-from :: Nat -> List String -> List String
  pieces-from k acc = case k == 0 of
    True -> acc
    False -> pieces-from (k - 1) (Cons (piece-at k) acc)

  count-empty :: List String -> Nat
  count-empty xs = case xs of
    Nil -> 0
    Cons s rest -> if same-str s "" then 1 + count-empty rest else count-empty rest

  -- Fold a piece list into the builder, one append per piece (append order is
  -- document order: the flatten lays the spine out in append order).
  build-all :: List String -> Builder -> Builder
  build-all xs b = case xs of
    Nil -> b
    Cons s rest -> build-all rest (builder-add-string s b)

  pieces-big :: List String
  pieces-big = pieces-from 400 Nil

  big-builder :: Builder
  big-builder = build-all pieces-big builder-new

  -- How many times byte `v` occurs in a string, by a TAIL scan over the bytes
  -- (never a materialized `List Char` or a per-index scalar scan: both would
  -- recurse or rescan on the scale document). O(n).
  count-byte-from :: Nat -> Bytes -> Nat -> Nat -> Nat
  count-byte-from v b i n =
    if i < bytes-len b then
      (if bytes-get b i == v then count-byte-from v b (i + 1) (n + 1) else count-byte-from v b (i + 1) n)
    else n

  count-byte :: Nat -> Bytes -> Nat
  count-byte v b = count-byte-from v b 0 0

  -- The LF count of a string (byte 10 never occurs inside a multi-byte scalar).
  newline-count :: String -> Nat
  newline-count s = count-byte 10 (string-to-bytes s)

  -- The first `n` scalars of `a` and `b` agree.
  char-eq-at :: String -> String -> Nat -> Bool
  char-eq-at a b i = case string-char-at a i of
    None -> False
    Some c -> case string-char-at b i of
      None -> False
      Some d -> eq-char c d

  prefix-eq-from :: String -> String -> Nat -> Nat -> Bool
  prefix-eq-from a b i n = case i == n of
    True -> True
    False -> if char-eq-at a b i then prefix-eq-from a b (i + 1) n else False

  prefix-eq :: String -> String -> Nat -> Bool
  prefix-eq a b n = prefix-eq-from a b 0 n

  -- The scale document: 3000 lines `line-<k>\n` appended in ascending order
  -- (line 1 first), one append per line.
  line-text :: Nat -> String
  line-text k = string-concat "line-" (nat-to-string k)

  line-size :: Nat -> Nat
  line-size k = 5 + nat-digit-count k + 1

  scale-into :: Nat -> Nat -> Builder -> Builder
  scale-into k n b = case k > n of
    True -> b
    False -> scale-into (k + 1) n (builder-add-line (line-text k) b)

  scale-total :: Nat -> Nat
  scale-total k = case k == 0 of
    True -> 0
    False -> line-size k + scale-total (k - 1)

  scale-builder :: Builder
  scale-builder = scale-into 1 3000 builder-new

  -- The LLVM-shaped emission: a header line, N define-ish functions each built
  -- from several pieces with `builder-add-line` for the newlines, a trailing
  -- newline from the last line.
  add-define :: Nat -> Builder -> Builder
  add-define k b =
    let b1 = builder-add-line (string-concat "define i32 @f" (string-concat (nat-to-string k) "() {")) b in
    let b2 = builder-add-line (string-concat "  ret i32 " (nat-to-string k)) b1 in
    builder-add-line "}" b2

  add-defines :: Nat -> Nat -> Builder -> Builder
  add-defines k n b = case k > n of
    True -> b
    False -> add-defines (k + 1) n (add-define k b)

  llvm-module :: Nat -> Builder -> Builder
  llvm-module n b = add-defines 1 n (builder-add-line "; ModuleID = 'm'" b)

  llvm-expected :: String
  llvm-expected = "; ModuleID = 'm'\ndefine i32 @f1() {\n  ret i32 1\n}\ndefine i32 @f2() {\n  ret i32 2\n}\n"
"#;

fn program(body: &str) -> String {
    format!("module Main where\n  import Text.Builder\n\n{HELPERS}\n{body}\n")
}

/// The differential oracle: a 400-piece document (empty pieces, embedded
/// newlines, large `Nat` and negative `Int` numbers, and non-ASCII text built
/// through `Data.Char`) built with the builder equals the same document built
/// with `string-concat-all` over the same pieces, as strings and as bytes. The
/// document is asserted to actually contain the shapes it claims to.
#[test]
fn differential_oracle_matches_string_concat() {
    expect_ok(
        "oracle",
        r#"  c1 :: Int
  c1 = if same-str (builder-run big-builder) (string-concat-all pieces-big) then 0 else 1

  c2 :: Int
  c2 = if string-to-bytes (builder-run big-builder) == string-to-bytes (string-concat-all pieces-big) then 0 else 2

  c3 :: Int
  c3 = if list-length pieces-big == 400 then 0 else 4

  c4 :: Int
  c4 = if count-empty pieces-big > 0 then 0 else 8

  c5 :: Int
  c5 = if newline-count (string-concat-all pieces-big) > 0 then 0 else 16

  c6 :: Int
  c6 = if string-byte-length (string-concat-all pieces-big) > string-length (string-concat-all pieces-big) then 0 else 32

  c7 :: Int
  c7 = if string-byte-length (builder-run big-builder) == builder-size big-builder then 0 else 64

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

/// The structural claim: after N appends the chunk count is N (one append, one
/// piece), the flattened byte length equals `builder-size`, and the edge cases
/// hold (a builder that was never appended to, appended empty pieces,
/// `builder-add-line`'s newline, the `Text.Show` renders, and a raw byte chunk).
#[test]
fn structural_chunk_count_and_size() {
    // Size and chunk count on the small and large documents.
    expect_ok(
        "structural-size",
        r#"  small :: Builder
  small = build-all (Cons "a" (Cons "" (Cons "bc" Nil))) builder-new

  c1 :: Int
  c1 = if builder-chunks small == 3 then 0 else 1

  c2 :: Int
  c2 = if builder-chunks big-builder == 400 then 0 else 2

  c3 :: Int
  c3 = if builder-size small == 3 then 0 else 4

  c4 :: Int
  c4 = if same-str (builder-run small) "abc" then 0 else 8

  c5 :: Int
  c5 = if string-byte-length (builder-run big-builder) == builder-size big-builder then 0 else 16

  c6 :: Int
  c6 = if bytes-len (builder-bytes big-builder) == builder-size big-builder then 0 else 32

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
    // A builder that was never appended to, and `builder-add-line` semantics.
    expect_ok(
        "structural-empty-lines",
        r#"  lines-b :: Builder
  lines-b = builder-add-line "y" (builder-add-line "x" builder-new)

  c1 :: Int
  c1 = if same-str (builder-run builder-new) "" then 0 else 1

  c2 :: Int
  c2 = if builder-is-empty builder-new then 0 else 2

  c3 :: Int
  c3 = if builder-chunks builder-new == 0 then 0 else 4

  c4 :: Int
  c4 = if builder-size builder-new == 0 then 0 else 8

  c5 :: Int
  c5 = if bytes-len (builder-bytes builder-new) == 0 then 0 else 16

  c6 :: Int
  c6 = if same-str (builder-run lines-b) "x\ny\n" then 0 else 32

  c7 :: Int
  c7 = if builder-chunks lines-b == 2 then 0 else 64

  c8 :: Int
  c8 = if builder-size lines-b == 4 then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#,
    );
    // The `Text.Show` renders (Nat and Int, including a negative) and a raw
    // byte chunk, in append order.
    expect_ok(
        "structural-renders-bytes",
        r#"  natint :: Builder
  natint = builder-add-int (nat-to-int 42) (builder-add-int (int-neg (nat-to-int 7)) (builder-add-nat 12345 builder-new))

  bytes-b :: Builder
  bytes-b = builder-add-bytes (bytes-from-list (Cons 104 (Cons 105 Nil))) builder-new

  mixed :: Builder
  mixed = builder-add-bytes (bytes-from-list (Cons 33 Nil)) (builder-add-string "a" builder-new)

  c1 :: Int
  c1 = if same-str (builder-run natint) "12345-742" then 0 else 1

  c2 :: Int
  c2 = if same-str (builder-run bytes-b) "hi" then 0 else 2

  c3 :: Int
  c3 = if string-to-bytes (builder-run bytes-b) == bytes-from-list (Cons 104 (Cons 105 Nil)) then 0 else 4

  c4 :: Int
  c4 = if builder-size bytes-b == 2 then 0 else 8

  c5 :: Int
  c5 = if same-str (builder-run mixed) "a!" then 0 else 16

  main :: Int
  main = c1 + c2 + c3 + c4 + c5
"#,
    );
}

/// A few-thousand-line document flattens correctly: 3000 line appends leave
/// 3000 chunks, `builder-size` equals the hand-summed byte length, the
/// flattened length agrees with that, the newline count is exactly 3000, and
/// the first line's seven characters are `line-1\n`. No timing assertion.
#[test]
fn scale_few_thousand_lines() {
    expect_ok(
        "scale",
        r#"  c1 :: Int
  c1 = if builder-chunks scale-builder == 3000 then 0 else 1

  c2 :: Int
  c2 = if builder-size scale-builder == scale-total 3000 then 0 else 2

  c3 :: Int
  c3 = if string-byte-length (builder-run scale-builder) == scale-total 3000 then 0 else 4

  c4 :: Int
  c4 = if string-length (builder-run scale-builder) == scale-total 3000 then 0 else 8

  c5 :: Int
  c5 = if newline-count (builder-run scale-builder) == 3000 then 0 else 16

  c6 :: Int
  c6 = if prefix-eq (builder-run scale-builder) "line-1\n" 7 then 0 else 32

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}

/// The codegen shape: an LLVM-module-shaped emission (a header line, two
/// `define`-ish functions each built from several pieces with `builder-add-line`
/// for the newlines, a trailing newline) matches the exact expected text, its
/// flattened length equals `builder-size`, and it holds exactly seven newlines.
#[test]
fn llvm_module_shape_exact_text() {
    expect_ok(
        "llvm",
        r#"  c1 :: Int
  c1 = if same-str (builder-run (llvm-module 2 builder-new)) llvm-expected then 0 else 1

  c2 :: Int
  c2 = if string-byte-length (builder-run (llvm-module 2 builder-new)) == builder-size (llvm-module 2 builder-new) then 0 else 2

  c3 :: Int
  c3 = if newline-count (builder-run (llvm-module 2 builder-new)) == 7 then 0 else 4

  main :: Int
  main = c1 + c2 + c3
"#,
    );
}
