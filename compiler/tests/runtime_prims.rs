//! Runtime platform and crypto externs (P34 + the content-integrity
//! family): the mutating filesystem operations, OS entropy and signal
//! delivery wired through `cinch.runtime.*` intrinsics, and the SHA-512
//! / SHA-1 / MD5 / HMAC-SHA-256 digests.
//!
//! Every program is a real project: a small `Prims` module carries the
//! `foreign import ccall "cinch.runtime.…"` declarations the prelude
//! will wrap, and a `Main` module drives each extern and reports a
//! bitmask (check k contributes bit 2^(k-1), `main` sums them), so a
//! nonzero exit code names the failed check. The digest checks compare
//! against the published vectors (FIPS 180-4, FIPS 180-1, RFC 1321 and
//! RFC 4231).

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The intrinsics under test, as the prelude will declare them.
const PRIMS: &str = r#"
module Prims where

  import Data.Bytes
  import Data.Nat

  export sha512, sha1, md5, hmac-sha256
  export random-bytes, fs-mkdir, fs-remove, fs-temp-dir, signal-send

  foreign import ccall "cinch.runtime.textSha512" sha512 :: Bytes -> Bytes
  foreign import ccall "cinch.runtime.textSha1" sha1 :: Bytes -> Bytes
  foreign import ccall "cinch.runtime.textMd5" md5 :: Bytes -> Bytes
  foreign import ccall "cinch.runtime.textHmacSha256" hmac-sha256 :: Bytes -> Bytes -> Bytes
  foreign import ccall "cinch.runtime.randomBytes" random-bytes :: Nat -> Bytes
  foreign import ccall "cinch.runtime.fsMkdir" fs-mkdir :: Bytes -> Bool -> I64
  foreign import ccall "cinch.runtime.fsRemove" fs-remove :: Bytes -> Bool -> I64
  foreign import ccall "cinch.runtime.fsTempDir" fs-temp-dir :: Unit -> Bytes
  foreign import ccall "cinch.runtime.signalSend" signal-send :: I64 -> I64 -> I64
"#;

fn run_project(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_prims_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Prims.cinch"), PRIMS).expect("write Prims");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main");
    // The checker's elaboration recursion runs near the default
    // test-thread stack at the whole-prelude baseline, and the digest
    // checks carry deep list literals; compile on a thread with a
    // generous stack so the slice is not sensitive to the harness
    // default (as `tests/lib_base64.rs` does).
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

fn expect_ok(name: &str, main_src: &str) {
    let code = run_project(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// A fresh scratch directory for one probe.
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_prims_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// A path as a Cinch string literal.
fn lit(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[test]
fn crypto_digests_match_published_vectors() {
    expect_ok(
        "crypto",
        r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.String
  import Prims

  digest-is-at :: Bytes -> List Nat -> Nat -> Bool
  digest-is-at b xs i = case xs of
    Nil -> i == bytes-len b
    Cons y ys -> case i == bytes-len b of
      True -> False
      False -> if bytes-get b i == y then digest-is-at b ys (i + 1) else False

  digest-is :: Bytes -> List Nat -> Bool
  digest-is b xs = digest-is-at b xs 0

  aa-list :: Nat -> List Nat
  aa-list n = if n == 0 then Nil else Cons 170 (aa-list (n - 1))

  aa-key :: Nat -> Bytes
  aa-key n = bytes-from-list (aa-list n)

  sha512-abc :: List Nat
  sha512-abc = (Cons 221 (Cons 175 (Cons 53 (Cons 161 (Cons 147 (Cons 97 (Cons 122 (Cons 186 (Cons 204 (Cons 65 (Cons 115 (Cons 73 (Cons 174 (Cons 32 (Cons 65 (Cons 49 (Cons 18 (Cons 230 (Cons 250 (Cons 78 (Cons 137 (Cons 169 (Cons 126 (Cons 162 (Cons 10 (Cons 158 (Cons 238 (Cons 230 (Cons 75 (Cons 85 (Cons 211 (Cons 154 (Cons 33 (Cons 146 (Cons 153 (Cons 42 (Cons 39 (Cons 79 (Cons 193 (Cons 168 (Cons 54 (Cons 186 (Cons 60 (Cons 35 (Cons 163 (Cons 254 (Cons 235 (Cons 189 (Cons 69 (Cons 77 (Cons 68 (Cons 35 (Cons 100 (Cons 60 (Cons 232 (Cons 14 (Cons 42 (Cons 154 (Cons 201 (Cons 79 (Cons 165 (Cons 76 (Cons 164 (Cons 159 Nil))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))

  sha512-empty :: List Nat
  sha512-empty = (Cons 207 (Cons 131 (Cons 225 (Cons 53 (Cons 126 (Cons 239 (Cons 184 (Cons 189 (Cons 241 (Cons 84 (Cons 40 (Cons 80 (Cons 214 (Cons 109 (Cons 128 (Cons 7 (Cons 214 (Cons 32 (Cons 228 (Cons 5 (Cons 11 (Cons 87 (Cons 21 (Cons 220 (Cons 131 (Cons 244 (Cons 169 (Cons 33 (Cons 211 (Cons 108 (Cons 233 (Cons 206 (Cons 71 (Cons 208 (Cons 209 (Cons 60 (Cons 93 (Cons 133 (Cons 242 (Cons 176 (Cons 255 (Cons 131 (Cons 24 (Cons 210 (Cons 135 (Cons 126 (Cons 236 (Cons 47 (Cons 99 (Cons 185 (Cons 49 (Cons 189 (Cons 71 (Cons 65 (Cons 122 (Cons 129 (Cons 165 (Cons 56 (Cons 50 (Cons 122 (Cons 249 (Cons 39 (Cons 218 (Cons 62 Nil))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))

  sha1-abc :: List Nat
  sha1-abc = (Cons 169 (Cons 153 (Cons 62 (Cons 54 (Cons 71 (Cons 6 (Cons 129 (Cons 106 (Cons 186 (Cons 62 (Cons 37 (Cons 113 (Cons 120 (Cons 80 (Cons 194 (Cons 108 (Cons 156 (Cons 208 (Cons 216 (Cons 157 Nil))))))))))))))))))))

  md5-abc :: List Nat
  md5-abc = (Cons 144 (Cons 1 (Cons 80 (Cons 152 (Cons 60 (Cons 210 (Cons 79 (Cons 176 (Cons 214 (Cons 150 (Cons 63 (Cons 125 (Cons 40 (Cons 225 (Cons 127 (Cons 114 Nil))))))))))))))))

  hmac2 :: List Nat
  hmac2 = (Cons 91 (Cons 220 (Cons 193 (Cons 70 (Cons 191 (Cons 96 (Cons 117 (Cons 78 (Cons 106 (Cons 4 (Cons 36 (Cons 38 (Cons 8 (Cons 149 (Cons 117 (Cons 199 (Cons 90 (Cons 0 (Cons 63 (Cons 8 (Cons 157 (Cons 39 (Cons 57 (Cons 131 (Cons 157 (Cons 236 (Cons 88 (Cons 185 (Cons 100 (Cons 236 (Cons 56 (Cons 67 Nil))))))))))))))))))))))))))))))))

  hmac6 :: List Nat
  hmac6 = (Cons 96 (Cons 228 (Cons 49 (Cons 89 (Cons 30 (Cons 224 (Cons 182 (Cons 127 (Cons 13 (Cons 138 (Cons 38 (Cons 170 (Cons 203 (Cons 245 (Cons 183 (Cons 127 (Cons 142 (Cons 11 (Cons 198 (Cons 33 (Cons 55 (Cons 40 (Cons 197 (Cons 20 (Cons 5 (Cons 70 (Cons 4 (Cons 15 (Cons 14 (Cons 227 (Cons 127 (Cons 84 Nil))))))))))))))))))))))))))))))))

  main :: Int
  main =
    let abc = string-to-bytes "abc" in
    let c1 = if digest-is (sha512 abc) sha512-abc then 0 else 1 in
    let c2 = if digest-is (sha1 abc) sha1-abc then 0 else 2 in
    let c3 = if digest-is (md5 abc) md5-abc then 0 else 4 in
    let c4 = if digest-is (hmac-sha256 (string-to-bytes "Jefe") (string-to-bytes "what do ya want for nothing?")) hmac2 then 0 else 8 in
    let c5 = if digest-is (sha512 (string-to-bytes "")) sha512-empty then 0 else 16 in
    let c6 = if digest-is (hmac-sha256 (aa-key 131) (string-to-bytes "Test Using Larger Than Block-Size Key - Hash Key First")) hmac6 then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}

#[test]
fn random_bytes_have_the_requested_length_and_differ() {
    expect_ok(
        "random",
        r#"
module Main where

  import Data.Bytes
  import Prims

  differs-at :: Bytes -> Bytes -> Nat -> Bool
  differs-at a b i = case i == bytes-len a of
    True -> False
    False -> if bytes-get a i == bytes-get b i then differs-at a b (i + 1) else True

  main :: Int
  main =
    let r1 = random-bytes 16 in
    let r2 = random-bytes 16 in
    let c1 = if bytes-len r1 == 16 then 0 else 1 in
    let c2 = if bytes-len r2 == 16 then 0 else 2 in
    let c3 = if differs-at r1 r2 0 then 0 else 4 in
    let c4 = if bytes-len (random-bytes 0) == 0 then 0 else 8 in
    c1 + c2 + c3 + c4
"#,
    );
}

#[test]
fn temp_dir_is_a_nonempty_existing_path() {
    expect_ok(
        "tempdir",
        r#"
module Main where

  import Data.Bytes
  import Data.Path
  import Data.String
  import Effect.Fs
  import Prims

  check-exists :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-exists u = case string-from-bytes (fs-temp-dir ()) of
    None -> False
    Some s -> case fs-metadata (path-from-string s) of
      Ok m -> case file-meta-kind m of
        FileIsDirectory -> True
        _ -> False
      Err e -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let t = fs-temp-dir () in
    let c1 = if bytes-len t > 0 then 0 else 1 in
    let c2 = if check-exists () then 0 else 2 in
    c1 + c2
"#,
    );
}

#[test]
fn fs_mkdir_creates_a_directory_and_a_nested_chain() {
    let dir = scratch("mkdir");
    let src = r#"
module Main where

  import Data.List
  import Data.Path
  import Data.String
  import Effect.Fs
  import Prims

  contains :: String -> List String -> Bool
  contains s xs = case xs of
    Nil -> False
    Cons y ys -> if y == s then True else contains s ys

  in-list :: forall r. using (io :: Io r) => Path -> String ->{IO r} Bool
  in-list d name = case fs-list d of
    Err e -> False
    Ok names -> contains name names

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let dir = path-from-string __DIR__ in
    let sub = path-join dir (path-from-string "sub") in
    let mk1 = fs-mkdir (string-to-bytes (path-to-string sub)) False in
    let c1 = if mk1 == 0 then 0 else 1 in
    let c2 = if in-list dir "sub" then 0 else 2 in
    let nest = path-join sub (path-from-string "a/b/c") in
    let mk2 = fs-mkdir (string-to-bytes (path-to-string nest)) True in
    let c3 = if mk2 == 0 then 0 else 4 in
    let mk3 = fs-mkdir (string-to-bytes (path-to-string sub)) False in
    let c4 = if mk3 == -3 then 0 else 8 in
    let mk4 = fs-mkdir (string-to-bytes (path-to-string nest)) True in
    let c5 = if mk4 == 0 then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#;
    expect_ok("mkdir", src.replace("__DIR__", &lit(&dir)).as_str());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fs_remove_deletes_a_file_and_subtree_and_missing_is_notfound() {
    let dir = scratch("remove");
    let src = r#"
module Main where

  import Data.Path
  import Data.String
  import Effect.Fs
  import Prims

  gone :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  gone p = case fs-metadata p of
    Ok m -> False
    Err e -> case e of
      NotFound -> True
      _ -> False

  wrote :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  wrote p = case fs-write p (string-to-bytes "hi") of
    Ok _ -> True
    Err e -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let dir = path-from-string __DIR__ in
    let f = path-join dir (path-from-string "f.txt") in
    let c0 = if wrote f then 0 else 32 in
    let rm1 = fs-remove (string-to-bytes (path-to-string f)) False in
    let c1 = if rm1 == 0 then 0 else 1 in
    let c2 = if gone f then 0 else 2 in
    let sub = path-join dir (path-from-string "sub") in
    let deep = path-join sub (path-from-string "deep") in
    let mk = fs-mkdir (string-to-bytes (path-to-string deep)) True in
    let rm2 = fs-remove (string-to-bytes (path-to-string sub)) True in
    let c3 = if rm2 == 0 then 0 else 4 in
    let c4 = if gone sub then 0 else 8 in
    let miss = path-join dir (path-from-string "nope") in
    let rm3 = fs-remove (string-to-bytes (path-to-string miss)) False in
    let c5 = if rm3 == -1 then 0 else 16 in
    c0 + c1 + c2 + c3 + c4 + c5
"#;
    expect_ok("remove", src.replace("__DIR__", &lit(&dir)).as_str());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn signal_zero_to_the_current_process_succeeds() {
    expect_ok(
        "signal",
        r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Path
  import Effect.Process
  import Prims

  is-digit :: Nat -> Bool
  is-digit c = if c >= 48 then c <= 57 else False

  digits-nat :: Bytes -> Nat -> Nat -> Nat
  digits-nat b i acc = case i == bytes-len b of
    True -> acc
    False -> case is-digit (bytes-get b i) of
      True -> digits-nat b (i + 1) (acc * 10 + (bytes-get b i - 48))
      False -> acc

  self-pid :: forall r. using (io :: Io r) => Path ->{IO r, Diverge} Nat
  self-pid sh = case process-run (command sh (Cons "-c" (Cons "echo $PPID" Nil))) "" of
    Err e -> 0
    Ok p -> case p of
      (code, out) -> digits-nat out 0 0

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    let pid = self-pid (path-from-string "/bin/sh") in
    let c1 = if pid > 0 then 0 else 1 in
    let c2 = if signal-send (nat-to-i64 pid) 0 == 0 then 0 else 2 in
    c1 + c2
"#,
    );
}
