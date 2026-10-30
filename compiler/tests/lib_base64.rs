//! Base64 consumer semantics: the RFC 4648 vectors in both directions, the padding law per input length, the round trip over every byte value, the URL-safe alphabet, and the three named decoder refusals (invalid character, invalid length, misplaced or excessive pad). The lenient decoder is exercised against whitespace-containing input (Ruby's `decode64` shape, which `base64-decode-lenient` ports: every non-alphabet scalar ignored, never a failure).
//!
//! Each test compiles a real two-module project (the module under test plus a `Main` that informs the checks) through the CLI pipeline's driver, exactly as lib_foldable.rs does. Checks are top-level `check-*` style functions returning 0 or their bit; `main` sums them, so a nonzero exit code is a bitmask naming the failed checks (check k contributes bit 2^(k-1)) and any failure mode of one check shares that bit. The checks are top-level because a `case` in a `let` binder's right-hand side must indent past the binder's column under the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const BASE64: &str = include_str!("../prelude/Text/Base64.cinch");

/// The shared helpers every `Main` uses: structural byte equality (there
/// is no `Eq Bytes`), a bounded counter for the all-bytes buffer, the
/// error tag (a `Nat` naming which named variant a decode reported, 0
/// for `Ok`), and the two decode predicates. Kept top-level so a `case`
/// never sits in a `let` binder's right-hand side.
const HELPERS: &str = r#"
  list-count :: forall a. List a -> Nat
  list-count xs = case xs of
    Nil -> 0
    Cons _ ys -> 1 + list-count ys

  bytes-equal-from :: Bytes -> Bytes -> Nat -> Bool
  bytes-equal-from a b i =
    if i >= bytes-len a then i >= bytes-len b
    else if i >= bytes-len b then False
    else if bytes-get a i == bytes-get b i then bytes-equal-from a b (i + 1)
    else False

  bytes-equal :: Bytes -> Bytes -> Bool
  bytes-equal a b = bytes-equal-from a b 0

  -- The bytes from `i` up to 255: every value a byte can hold, so the
  -- bound is the byte range itself.
  bytes-upward :: Nat -> List (InRange 0 255 Nat)
  bytes-upward i = if i < 256 then Cons i (bytes-upward (i + 1)) else Nil

  err-tag :: Result Base64Error Bytes -> Nat
  err-tag r = case r of
    Ok _ -> 0
    Err e -> case e of
      InvalidBase64Character _ -> 1
      InvalidBase64Length _ -> 2
      InvalidBase64Padding -> 3

  enc-is :: List (InRange 0 255 Nat) -> String -> Bool
  enc-is xs expected = base64-encode (bytes-from-list xs) == expected

  dec-ok :: String -> List (InRange 0 255 Nat) -> Bool
  dec-ok input expected = case base64-decode input of
    Err _ -> False
    Ok bs -> bytes-equal bs (bytes-from-list expected)

  url-dec-ok :: String -> List (InRange 0 255 Nat) -> Bool
  url-dec-ok input expected = case base64-url-decode input of
    Err _ -> False
    Ok bs -> bytes-equal bs (bytes-from-list expected)

  lenient-is :: String -> List (InRange 0 255 Nat) -> Bool
  lenient-is input expected = bytes-equal (base64-decode-lenient input) (bytes-from-list expected)

  foobar :: List (InRange 0 255 Nat)
  foobar = Cons 102 (Cons 111 (Cons 111 (Cons 98 (Cons 97 (Cons 114 Nil)))))

  all-bytes :: Bytes
  all-bytes = bytes-from-list (bytes-upward 0)
"#;

/// The `Main` module: the standard imports around the shared helpers and
/// the test body.
fn main_module(body: &str) -> String {
    let mut src = String::from(
        "module Main where\n\n  import Text.Base64\n  import Data.Bool\n  import Data.Bytes\n  import Data.List\n  import Data.Nat\n  import Data.Result\n",
    );
    src.push_str(HELPERS);
    src.push('\n');
    src.push_str(body);
    src.push('\n');
    src
}

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_b64test_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Base64.cinch", BASE64), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    // The checker's elaboration recursion runs near the default test-thread
    // stack at the whole-prelude baseline, and this slice's module sits on
    // top of it; compile on a thread with a generous stack so the slice is
    // not sensitive to the harness default.
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
    let code = run_main(name, &main_module(body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The seven RFC 4648 section 10 vectors: `""`, `f`, `fo`, `foo`, `foob`,
/// `fooba`, `foobar`. Encoding asserts the exact text (so each length mod
/// 3's padding is pinned: 0 and 3 mod 3 unpadded, 2 mod 3 one pad, 1 mod 3
/// two), and decoding asserts the original bytes back.
#[test]
fn rfc4648_vectors_both_directions() {
    expect_ok(
        "rfc-vectors",
        "  encode-ok :: Bool\n\
         \x20 encode-ok =\n\
         \x20   enc-is Nil \"\"\n\
         \x20   && enc-is (Cons 102 Nil) \"Zg==\"\n\
         \x20   && enc-is (Cons 102 (Cons 111 Nil)) \"Zm8=\"\n\
         \x20   && enc-is (Cons 102 (Cons 111 (Cons 111 Nil))) \"Zm9v\"\n\
         \x20   && enc-is (Cons 102 (Cons 111 (Cons 111 (Cons 98 Nil)))) \"Zm9vYg==\"\n\
         \x20   && enc-is (Cons 102 (Cons 111 (Cons 111 (Cons 98 (Cons 97 Nil))))) \"Zm9vYmE=\"\n\
         \x20   && enc-is (Cons 102 (Cons 111 (Cons 111 (Cons 98 (Cons 97 (Cons 114 Nil)))))) \"Zm9vYmFy\"\n\
         \x20 decode-ok :: Bool\n\
         \x20 decode-ok =\n\
         \x20   dec-ok \"\" Nil\n\
         \x20   && dec-ok \"Zg==\" (Cons 102 Nil)\n\
         \x20   && dec-ok \"Zm8=\" (Cons 102 (Cons 111 Nil))\n\
         \x20   && dec-ok \"Zm9v\" (Cons 102 (Cons 111 (Cons 111 Nil)))\n\
         \x20   && dec-ok \"Zm9vYg==\" (Cons 102 (Cons 111 (Cons 111 (Cons 98 Nil))))\n\
         \x20   && dec-ok \"Zm9vYmE=\" (Cons 102 (Cons 111 (Cons 111 (Cons 98 (Cons 97 Nil)))))\n\
         \x20   && dec-ok \"Zm9vYmFy\" (Cons 102 (Cons 111 (Cons 111 (Cons 98 (Cons 97 (Cons 114 Nil))))))\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let c1 = if encode-ok then 0 else 1 in\n\
         \x20   let c2 = if decode-ok then 0 else 2 in\n\
         \x20   c1 + c2\n",
    );
}

/// The URL-safe alphabet: the bytes `0xFB 0xF0` and `0xFF 0xFF 0xFF`
/// produce `+` and `/` under the standard alphabet (asserted), and `-`
/// and `_` under the URL-safe one (asserted), and the URL-safe decoder
/// round-trips them and, like Ruby's `urlsafe_decode64`, also accepts
/// the standard `+`/`/` spellings.
#[test]
fn urlsafe_alphabet() {
    expect_ok(
        "urlsafe",
        "  urlsafe-ok :: Bool\n\
         \x20 urlsafe-ok =\n\
         \x20   base64-encode (bytes-from-list (Cons 251 (Cons 240 Nil))) == \"+/A=\"\n\
         \x20   && base64-encode (bytes-from-list (Cons 255 (Cons 255 (Cons 255 Nil)))) == \"////\"\n\
         \x20   && base64-url-encode (bytes-from-list (Cons 251 (Cons 240 Nil))) == \"-_A=\"\n\
         \x20   && base64-url-encode (bytes-from-list (Cons 255 (Cons 255 (Cons 255 Nil)))) == \"____\"\n\
         \x20   && url-dec-ok \"-_A=\" (Cons 251 (Cons 240 Nil))\n\
         \x20   && url-dec-ok \"____\" (Cons 255 (Cons 255 (Cons 255 Nil)))\n\
         \x20   && url-dec-ok \"+/A=\" (Cons 251 (Cons 240 Nil))\n\
         \x20   && err-tag (base64-url-decode \"Zm9v\") == 0\n\
         \x20 main :: Int\n\
         \x20 main = if urlsafe-ok then 0 else 1\n",
    );
}

/// The round trip over every byte value 0..255: a 256-byte buffer (length
/// 1 mod 3, so a two-pad tail) encodes and decodes back byte for byte.
#[test]
fn round_trip_all_byte_values() {
    expect_ok(
        "round-trip",
        "  round-trip-ok :: Bool\n\
         \x20 round-trip-ok = case base64-decode (base64-encode all-bytes) of\n\
         \x20   Err _ -> False\n\
         \x20   Ok bs -> bytes-equal bs all-bytes\n\
         \x20 main :: Int\n\
         \x20 main = if round-trip-ok then 0 else 1\n",
    );
}

/// The invalid-character refusal: a scalar outside the decoded alphabet
/// (the standard decoder refuses the URL-safe `-`/`_` and whitespace) is
/// `InvalidBase64Character` (tag 1), and a valid input is `Ok` (tag 0).
#[test]
fn invalid_character_is_refused() {
    expect_ok(
        "invalid-character",
        "  bad-char-ok :: Bool\n\
         \x20 bad-char-ok =\n\
         \x20   err-tag (base64-decode \"A!BC\") == 1\n\
         \x20   && err-tag (base64-decode \"AB!D\") == 1\n\
         \x20   && err-tag (base64-decode \"AAA \") == 1\n\
         \x20   && err-tag (base64-decode \"AA-D\") == 1\n\
         \x20   && err-tag (base64-decode \"A_BC\") == 1\n\
         \x20   && err-tag (base64-decode \"AA\\nD\") == 1\n\
         \x20   && err-tag (base64-decode \"Zm9v\") == 0\n\
         \x20 main :: Int\n\
         \x20 main = if bad-char-ok then 0 else 1\n",
    );
}

/// The invalid-length refusal: a scalar count that is not a multiple of 4
/// is `InvalidBase64Length` (tag 2), checked before any character or pad
/// validation.
#[test]
fn invalid_length_is_refused() {
    expect_ok(
        "invalid-length",
        "  bad-len-ok :: Bool\n\
         \x20 bad-len-ok =\n\
         \x20   err-tag (base64-decode \"A\") == 2\n\
         \x20   && err-tag (base64-decode \"AA\") == 2\n\
         \x20   && err-tag (base64-decode \"AAA\") == 2\n\
         \x20   && err-tag (base64-decode \"AAAAA\") == 2\n\
         \x20   && err-tag (base64-decode \"AAAAAAA\") == 2\n\
         \x20   && err-tag (base64-decode \"Zg===\") == 2\n\
         \x20   && err-tag (base64-decode \"Zm9v\") == 0\n\
         \x20 main :: Int\n\
         \x20 main = if bad-len-ok then 0 else 1\n",
    );
}

/// The padding refusal: `=` before the last group or as a third pad is
/// `InvalidBase64Padding` (tag 3), while the legal 1-pad and 2-pad tails
/// decode to `Ok` (tag 0).
#[test]
fn invalid_padding_is_refused() {
    expect_ok(
        "invalid-padding",
        "  bad-pad-ok :: Bool\n\
         \x20 bad-pad-ok =\n\
         \x20   err-tag (base64-decode \"=AAA\") == 3\n\
         \x20   && err-tag (base64-decode \"AA=A\") == 3\n\
         \x20   && err-tag (base64-decode \"AB=C\") == 3\n\
         \x20   && err-tag (base64-decode \"A===\") == 3\n\
         \x20   && err-tag (base64-decode \"====\") == 3\n\
         \x20   && err-tag (base64-decode \"AA==AA==\") == 3\n\
         \x20   && err-tag (base64-decode \"AAB=\") == 0\n\
         \x20   && err-tag (base64-decode \"AB==\") == 0\n\
         \x20 main :: Int\n\
         \x20 main = if bad-pad-ok then 0 else 1\n",
    );
}

/// The lenient decoder (Ruby's `decode64` shape): line breaks, spaces,
/// and stray non-alphabet scalars are all ignored, and the result is the
/// bytes; it never reports an error.
#[test]
fn lenient_decode_ignores_whitespace_and_junk() {
    expect_ok(
        "lenient",
        "  lenient-ok :: Bool\n\
         \x20 lenient-ok =\n\
         \x20   lenient-is \"Zm9vYmFy\" foobar\n\
         \x20   && lenient-is \"Zm9v\\nYmFy\\n\" foobar\n\
         \x20   && lenient-is \"Zm 9v\\nYmFy\" foobar\n\
         \x20   && lenient-is \"!!Zm9vYmFy??\" foobar\n\
         \x20   && lenient-is \"Zm8=\\n\" (Cons 102 (Cons 111 Nil))\n\
         \x20   && bytes-equal (base64-decode-lenient \"\") (bytes-new 0)\n\
         \x20 main :: Int\n\
         \x20 main = if lenient-ok then 0 else 1\n",
    );
}
