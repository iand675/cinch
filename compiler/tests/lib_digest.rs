//! Text.Digest consumer semantics: the published digest vectors for every algorithm the selector names (SHA-256, SHA-512, SHA-1, MD5 of `"abc"`), the distinct empty-input digests of SHA-256 and SHA-512, that two different algorithms disagree on one input (so the selector is live), the lower-case, 2N-character shape of `digest-hex` with the `digest-length`/`digest-name` accessors, the encode-then-digest law of `digest-string`, and the RFC 4231 case 2 HMAC-SHA-256 vector (`5bdcc146...`) through `hmac-sha256-hex`.
//!
//! Each test compiles a real two-module project (Text/Digest.cinch beside a `Main` that informs the checks) through the CLI pipeline's driver, exactly as lib_base64.rs does. Checks are top-level `check-*` style functions returning 0 or their bit; `main` sums them, so a nonzero exit code is a bitmask naming the failed checks (check k contributes bit 2^(k-1)). The checks are top-level because a `case` in a `let` binder's right-hand side must indent past the binder's column under the layout rules; and the compiler runs on a generously stacked thread because the elaboration recursion sits near the default test-thread stack at the whole-prelude baseline.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const DIGEST: &str = include_str!("../prelude/Text/Digest.cinch");

/// The shared helpers every `Main` uses: structural byte equality (there is no `Eq Bytes`), the lower-case hex predicate over the rendered text, and the scalar-code reader for it. Kept top-level so a `case` never sits in a `let` binder's right-hand side.
const HELPERS: &str = r#"
  bytes-equal-from :: Bytes -> Bytes -> Nat -> Bool
  bytes-equal-from a b i =
    if i >= bytes-len a then i >= bytes-len b
    else if i >= bytes-len b then False
    else if bytes-get a i == bytes-get b i then bytes-equal-from a b (i + 1)
    else False

  bytes-equal :: Bytes -> Bytes -> Bool
  bytes-equal a b = bytes-equal-from a b 0

  char-nat :: Char -> Nat
  char-nat c = nat-from-i64 (u32-to-i64 (char-to-u32 c))

  hex-digit :: Char -> Bool
  hex-digit c =
    let n = char-nat c in
    (n >= 48 && n <= 57) || (n >= 97 && n <= 102)

  all-hex :: List Char -> Bool
  all-hex cs = case cs of
    Nil -> True
    Cons c rest -> if hex-digit c then all-hex rest else False

  hex-ok :: String -> Bool
  hex-ok s = all-hex (unpack s)
"#;

/// The `Main` module: the standard imports around the shared helpers and the test body.
fn main_module(body: &str) -> String {
    let mut src = String::from(
        "module Main where\n\n  import Text.Digest\n  import Data.Bool\n  import Data.Bytes\n  import Data.Char\n  import Data.List\n  import Data.Nat\n  import Data.String\n",
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
        "cinch_digesttest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Digest.cinch", DIGEST), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    // Compile on a thread with a generous stack: the checker's
    // elaboration recursion runs near the default test-thread stack at
    // the whole-prelude baseline, as lib_base64.rs notes.
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

/// The published vectors, one check per algorithm (`"abc"`), the two distinct empty-input digests, the `digest-hex` shape (lower case, 2N characters) with the accessors, the selector being live (SHA-256 and SHA-1 disagree on `"abc"`), and the RFC 4231 case 2 HMAC-SHA-256 tag.
#[test]
fn digest_vectors_shapes_and_hmac() {
    expect_ok(
        "vectors",
        "  main :: Int\n\
         \x20 main =\n\
         \x20   let abc = string-to-bytes \"abc\" in\n\
         \x20   let empty = string-to-bytes \"\" in\n\
         \x20   let c1 = if digest-hex DigestSha256 abc == \"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\" then 0 else 1 in\n\
         \x20   let c2 = if digest-hex DigestSha512 abc == \"ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f\" then 0 else 2 in\n\
         \x20   let c3 = if digest-hex DigestSha1 abc == \"a9993e364706816aba3e25717850c26c9cd0d89d\" then 0 else 4 in\n\
         \x20   let c4 = if digest-hex DigestMd5 abc == \"900150983cd24fb0d6963f7d28e17f72\" then 0 else 8 in\n\
         \x20   let c5 = if (digest-hex DigestSha256 empty == \"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\") && (digest-hex DigestSha512 empty == \"cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e\") then 0 else 16 in\n\
         \x20   let h = digest-hex DigestSha512 abc in\n\
         \x20   let c6 = if (string-byte-length h == 2 * digest-length DigestSha512) && hex-ok h && (digest-name DigestSha512 == \"sha512\") then 0 else 32 in\n\
         \x20   let c7 = if (not (bytes-equal (digest-bytes DigestSha256 abc) (digest-bytes DigestSha1 abc))) && bytes-equal (digest-string DigestMd5 \"abc\") (digest-bytes DigestMd5 abc) then 0 else 64 in\n\
         \x20   let c8 = if hmac-sha256-hex (string-to-bytes \"Jefe\") (string-to-bytes \"what do ya want for nothing?\") == \"5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843\" then 0 else 128 in\n\
         \x20   c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8\n",
    );
}
