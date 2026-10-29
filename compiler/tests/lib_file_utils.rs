//! `Effect.FileUtils` mutating operations and the pure `Data.Path`
//! Pathname operations (P40, the Ruby `FileUtils`/`Pathname` parity
//! target of design/stdlib-parity-ruby.md).
//!
//! The tree probe builds a real tree on disk through the capability:
//! `mkdir-p` creates a nested chain and is idempotent on the existing
//! directory, `cp` copies a file's contents, `mv` moves one, `rm-r`
//! removes a subtree, and `temp-file` creates two distinct empty files
//! under the platform temp directory (the emptiness and existence are
//! asserted from Rust on the paths the program records). The symlink
//! case is constructed from the RUST side (`std::os::unix::fs::symlink`
//! here) because the slice has no symlink-creation operation; the
//! program then `rm-r`s the link and Rust asserts the link is gone
//! while its target directory and the file inside it are untouched.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit
//! and `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (bit 2^(k-1) for check k). The error probe exercises
//! one named `FileUtilsError` per check; the permission case is a third
//! probe, skipped when the process is privileged enough that the OS
//! would not refuse (a container running as root).
//!
//! The pure `Data.Path` operations are a table of hand-computed cases
//! across four small programs (basename/dirname, extname/stem and
//! absoluteness, normalize, relative-to), including `/a/b/../c` and a
//! leading `..` case.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const FILE_UTILS: &str = include_str!("../prelude/Effect/FileUtils.cinch");

/// A vendored `Effect.Random` used while the module is still landing in
/// the prelude (its owner vendors the same copy beside `Main.cinch`); if
/// the prelude copy exists it is preferred, so once the module is
/// embedded the test exercises the real one. The surface used is
/// `random-hex :: using (io :: Io r) => Nat ->{IO r} String`.
const RANDOM_STANDIN: &str = r#"
module Effect.Random where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.String

  export random-bytes, hex-of-bytes, random-hex

  foreign import ccall "cinch.runtime.randomBytes" random-bytes :: Nat -> Bytes

  hex-chars :: List Char
  hex-chars = Cons '0' (Cons '1' (Cons '2' (Cons '3' (Cons '4' (Cons '5' (Cons '6' (Cons '7' (Cons '8' (Cons '9' (Cons 'a' (Cons 'b' (Cons 'c' (Cons 'd' (Cons 'e' (Cons 'f' Nil)))))))))))))))

  nth :: Nat -> List Char -> Char
  nth k xs = case xs of
    Nil -> '0'
    Cons c rest -> case k == 0 of
      True -> c
      False -> nth (k - 1) rest

  byte-hex :: Nat -> List Char
  byte-hex b = Cons (nth (nat-div b 16) hex-chars) (Cons (nth (nat-rem b 16) hex-chars) Nil)

  append-chars :: List Char -> List Char -> List Char
  append-chars a b = case a of
    Nil -> b
    Cons c rest -> Cons c (append-chars rest b)

  hex-list :: Bytes -> Nat -> Nat -> List Char
  hex-list b i n = case i >= n of
    True -> Nil
    False -> append-chars (byte-hex (bytes-get b i)) (hex-list b (i + 1) n)

  hex-of-bytes :: Bytes -> String
  hex-of-bytes b = pack (hex-list b 0 (bytes-len b))

  random-hex :: forall r. using (io :: Io r) => Nat ->{IO r} String
  random-hex n = hex-of-bytes (random-bytes n)
"#;

/// The prelude `Effect.Random` source when it is present, else the
/// vendored stand-in (the module is landing in the prelude in parallel).
fn random_source() -> String {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("prelude/Effect/Random.cinch");
    std::fs::read_to_string(&p).unwrap_or_else(|_| RANDOM_STANDIN.to_string())
}

/// Compile and run a project (the module under test plus `Main.cinch`)
/// in a fresh temp directory, returning the exit code. The prelude is
/// large, so the compile runs on a thread with a generous stack (as
/// `tests/lib_find.rs` does).
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_fileutils_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("FileUtils.cinch", FILE_UTILS.to_string()),
        ("Random.cinch", random_source()),
        ("Main.cinch", main_src.to_string()),
    ] {
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

/// A fresh empty scratch directory (each probe owns its tree).
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_fileutils_{tag}_{}", std::process::id()));
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

const TREE_MAIN: &str = r#"
module Main where

  import Data.Bytes
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Effect.FileUtils

  root :: Path
  root = path-from-string __DIR__

  under :: Path -> String -> Path
  under p n = path-join p (path-from-string n)

  same-bytes :: Bytes -> String -> Bool
  same-bytes b s = case string-from-bytes b of
    Some t -> t == s
    None -> False

  -- mkdir-p creates the whole missing chain and is idempotent on the
  -- existing directory.
  check-mkdir-p :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-mkdir-p u = case mkdir-p (under root "chain/a/b") of
    Err e -> False
    Ok _ -> case mkdir-p (under root "chain/a/b") of
      Err e2 -> False
      Ok _ -> case fs-metadata (under root "chain/a/b") of
        Err e3 -> False
        Ok m -> case file-meta-kind m of
          FileIsDirectory -> True
          _ -> False

  -- cp copies the source file's contents to the destination.
  check-cp :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-cp u = case fs-write (under root "src.txt") (string-to-bytes "hello") of
    Err e -> False
    Ok _ -> case cp (under root "src.txt") (under root "copy.txt") of
      Err e2 -> False
      Ok _ -> case fs-read (under root "copy.txt") of
        Err e3 -> False
        Ok b -> same-bytes b "hello"

  -- mv moves the file (the source is gone, the destination holds its
  -- contents).
  check-mv :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-mv u = case fs-write (under root "msrc.txt") (string-to-bytes "move") of
    Err e -> False
    Ok _ -> case mv (under root "msrc.txt") (under root "mdst.txt") of
      Err e2 -> False
      Ok _ -> case fs-read (under root "mdst.txt") of
        Err e3 -> False
        Ok b -> case same-bytes b "move" of
          False -> False
          True -> case fs-metadata (under root "msrc.txt") of
            Err e4 -> case e4 of
              NotFound -> True
              _ -> False
            Ok m -> False

  -- rm-r removes a whole subtree (directories and the file inside).
  check-rm-r :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-rm-r u = case mkdir-p (under root "tree/x/y") of
    Err e -> False
    Ok _ -> case fs-write (under root "tree/x/y/f.txt") (string-to-bytes "deep") of
      Err e2 -> False
      Ok _ -> case rm-r (under root "tree") of
        Err e3 -> False
        Ok _ -> case fs-metadata (under root "tree") of
          Err e4 -> case e4 of
            NotFound -> True
            _ -> False
          Ok m -> False

  -- rm-r on the symlink (constructed from the RUST side) removes the
  -- link itself, never the target directory it points at.
  check-symlink :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-symlink u = case rm-r (under root "link") of
    Err e -> False
    Ok _ -> case fs-metadata (under root "link") of
      Err e2 -> case e2 of
        NotFound -> True
        _ -> False
      Ok m -> False

  -- temp-file creates two distinct empty regular files under the
  -- platform temp directory; their texts are recorded for Rust to
  -- inspect.
  check-temp :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-temp u = case temp-file () of
    Err e -> False
    Ok p1 -> case temp-file () of
      Err e2 -> False
      Ok p2 -> case path-to-string p1 == path-to-string p2 of
        True -> False
        False -> case fs-metadata p1 of
          Err e3 -> False
          Ok m1 -> case file-meta-kind m1 of
            FileIsFile -> case fs-metadata p2 of
              Err e4 -> False
              Ok m2 -> case file-meta-kind m2 of
                FileIsFile -> case fs-write (under root "temp-paths.txt") (string-to-bytes (string-concat (path-to-string p1) (string-concat "\n" (path-to-string p2)))) of
                  Err e5 -> False
                  Ok _ -> True
                _ -> False
            _ -> False

  checks :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  checks u =
    let c1 = if check-mkdir-p () then 0 else 1 in
    let c2 = if check-cp () then 0 else 2 in
    let c3 = if check-mv () then 0 else 4 in
    let c4 = if check-rm-r () then 0 else 8 in
    let c5 = if check-symlink () then 0 else 16 in
    let c6 = if check-temp () then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    checks ()
"#;

#[test]
fn file_utils_builds_a_tree_and_removes_a_subtree() {
    let root = scratch("tree");
    // The symlink and its target come from Rust (no symlink-creation
    // operation exists in the capability).
    std::fs::create_dir_all(root.join("targetdir")).expect("target dir");
    std::fs::write(root.join("targetdir/keep.txt"), "keep").expect("target file");
    std::os::unix::fs::symlink(root.join("targetdir"), root.join("link")).expect("symlink");

    let src = TREE_MAIN.replace("__DIR__", &lit(&root));
    let code = run_main("tree", &src);
    assert_eq!(
        code, 0,
        "file-utils tree: bitmask {code} (1 mkdir-p, 2 cp, 4 mv, 8 rm-r, 16 symlink, 32 temp-file)"
    );

    // On-disk assertions, independent of the program's own bitmask.
    assert!(root.join("chain/a/b").is_dir(), "mkdir-p created the chain");
    assert_eq!(std::fs::read_to_string(root.join("copy.txt")).unwrap(), "hello");
    assert_eq!(std::fs::read_to_string(root.join("mdst.txt")).unwrap(), "move");
    assert!(!root.join("msrc.txt").exists(), "mv left the source behind");
    assert!(!root.join("tree").exists(), "rm-r left the subtree behind");

    // rm-r removed the LINK, not what it pointed at.
    assert!(
        std::fs::symlink_metadata(root.join("link")).is_err(),
        "rm-r did not remove the symlink"
    );
    assert!(root.join("targetdir").is_dir(), "rm-r followed the symlink to its target");
    assert_eq!(std::fs::read_to_string(root.join("targetdir/keep.txt")).unwrap(), "keep");

    // temp-file: two distinct empty regular files under the platform
    // temp directory.
    let recorded = std::fs::read_to_string(root.join("temp-paths.txt")).expect("temp-paths");
    let mut lines = recorded.lines();
    let p1 = PathBuf::from(lines.next().expect("first temp path"));
    let p2 = PathBuf::from(lines.next().expect("second temp path"));
    assert_ne!(p1, p2, "temp-file returned the same path twice");
    assert!(p1.starts_with(std::env::temp_dir()), "temp file outside the temp dir: {p1:?}");
    assert!(p2.starts_with(std::env::temp_dir()), "temp file outside the temp dir: {p2:?}");
    assert!(p1.is_file() && p2.is_file(), "temp-file did not create a regular file");
    assert_eq!(std::fs::metadata(&p1).unwrap().len(), 0, "temp-file is not empty");
    let name = p1.file_name().unwrap().to_str().unwrap();
    assert!(name.starts_with("cinch-"), "temp name has no prefix: {name}");
    assert_eq!(name.len(), "cinch-".len() + 32, "temp name is not 16 hex bytes: {name}");
    let _ = std::fs::remove_file(&p1);
    let _ = std::fs::remove_file(&p2);
    let _ = std::fs::remove_dir_all(&root);
}

const ERRORS_MAIN: &str = r#"
module Main where

  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Effect.FileUtils

  root :: Path
  root = path-from-string __DIR__

  under :: Path -> String -> Path
  under p n = path-join p (path-from-string n)

  check-not-found :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-not-found u = case rm (under root "no-such") of
    Ok _ -> False
    Err e -> case e of
      FileUtilsNotFound -> True
      _ -> False

  check-already-exists :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-already-exists u = case fs-write (under root "blocked") (string-to-bytes "x") of
    Err e -> False
    Ok _ -> case mkdir-p (under root "blocked") of
      Ok _ -> False
      Err e2 -> case e2 of
        FileUtilsAlreadyExists -> True
        _ -> False

  check-is-a-directory :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-is-a-directory u = case mkdir-p (under root "adir") of
    Err e -> False
    Ok _ -> case cp (under root "adir") (under root "out.txt") of
      Ok _ -> False
      Err e2 -> case e2 of
        FileUtilsIsADirectory -> True
        _ -> False

  check-not-a-directory :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-not-a-directory u = case fs-write (under root "afile") (string-to-bytes "x") of
    Err e -> False
    Ok _ -> case fs-write (under root "srcerr.txt") (string-to-bytes "y") of
      Err e2 -> False
      Ok _ -> case cp (under root "srcerr.txt") (under root "afile/child") of
        Ok _ -> False
        Err e3 -> case e3 of
          FileUtilsNotADirectory -> True
          _ -> False

  check-io-error :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-io-error u = case mkdir-p (under root "nonempty") of
    Err e -> False
    Ok _ -> case fs-write (under root "nonempty/inner.txt") (string-to-bytes "z") of
      Err e2 -> False
      Ok _ -> case rm (under root "nonempty") of
        Ok _ -> False
        Err e3 -> case e3 of
          FileUtilsIoError -> True
          _ -> False

  checks :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  checks u =
    let c1 = if check-not-found () then 0 else 1 in
    let c2 = if check-already-exists () then 0 else 2 in
    let c3 = if check-is-a-directory () then 0 else 4 in
    let c4 = if check-not-a-directory () then 0 else 8 in
    let c5 = if check-io-error () then 0 else 16 in
    c1 + c2 + c3 + c4 + c5

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    checks ()
"#;

#[test]
fn file_utils_reports_the_named_errors() {
    let root = scratch("errors");
    let src = ERRORS_MAIN.replace("__DIR__", &lit(&root));
    let code = run_main("errors", &src);
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(
        code, 0,
        "file-utils errors: bitmask {code} (1 NotFound, 2 AlreadyExists, 4 IsADirectory, \
         8 NotADirectory, 16 IoError)"
    );
}

const PERM_MAIN: &str = r#"
module Main where

  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Effect.FileUtils

  root :: Path
  root = path-from-string __DIR__

  under :: Path -> String -> Path
  under p n = path-join p (path-from-string n)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case cp (under root "psrc.txt") (under root "ro/out.txt") of
      Ok _ -> 2
      Err e -> case e of
        FileUtilsPermissionDenied -> 0
        _ -> 1
"#;

/// Set a directory mode (the permission probe).
fn chmod(p: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(p).expect("metadata").permissions();
    perms.set_mode(mode);
    std::fs::set_permissions(p, perms).expect("set mode");
}

#[test]
fn cp_into_an_unwritable_directory_is_permission_denied() {
    let root = scratch("perm");
    std::fs::create_dir_all(root.join("ro")).expect("ro dir");
    std::fs::write(root.join("psrc.txt"), "data").expect("source file");
    chmod(&root.join("ro"), 0o000);
    // A privileged process (a root container) is not refused by the OS,
    // so the permission case cannot be observed and is skipped.
    let privileged = std::fs::write(root.join("ro/probe"), "x").is_ok();
    let _ = std::fs::remove_file(root.join("ro/probe"));
    if privileged {
        chmod(&root.join("ro"), 0o755);
        let _ = std::fs::remove_dir_all(&root);
        return;
    }
    let src = PERM_MAIN.replace("__DIR__", &lit(&root));
    let code = run_main("perm", &src);
    chmod(&root.join("ro"), 0o755);
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(code, 0, "permission probe: 0 expected (2 means the write succeeded): {code}");
}

const PATH_BASENAME_MAIN: &str = r#"
module Main where

  import Data.Path

  b :: String -> String
  b s = path-basename (path-from-string s)

  d :: String -> String
  d s = path-dirname (path-from-string s)

  main :: Int
  main =
    let c1 = if b "a/b" == "b" then 0 else 1 in
    let c2 = if b "/a/b/" == "b" then 0 else 2 in
    let c3 = if b "/" == "/" then 0 else 4 in
    let c4 = if d "a/b" == "a" then 0 else 8 in
    let c5 = if d "/a" == "/" then 0 else 16 in
    let c6 = if d "a" == "." then 0 else 32 in
    let c7 = if d "/a/b/" == "/a" then 0 else 64 in
    let c8 = if b "a/b/c" == "c" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#;

const PATH_EXT_MAIN: &str = r#"
module Main where

  import Data.Path

  e :: String -> String
  e s = path-extname (path-from-string s)

  st :: String -> String
  st s = path-stem (path-from-string s)

  abs :: String -> Bool
  abs s = path-is-absolute (path-from-string s)

  main :: Int
  main =
    let c1 = if e "a/b.txt" == ".txt" then 0 else 1 in
    let c2 = if e "a/.bashrc" == "" then 0 else 2 in
    let c3 = if e "a/b.tar.gz" == ".gz" then 0 else 4 in
    let c4 = if e "a/b" == "" then 0 else 8 in
    let c5 = if st "a/b.tar.gz" == "b.tar" then 0 else 16 in
    let c6 = if abs "/a" == True then 0 else 32 in
    let c7 = if abs "a" == False then 0 else 64 in
    let c8 = if abs "/" == True then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#;

const PATH_NORM_MAIN: &str = r#"
module Main where

  import Data.Path

  n :: String -> String
  n s = path-to-string (path-normalize (path-from-string s))

  main :: Int
  main =
    let c1 = if n "/a/b/../c" == "/a/c" then 0 else 1 in
    let c2 = if n "a/b/../../c" == "c" then 0 else 2 in
    let c3 = if n "../a" == "../a" then 0 else 4 in
    let c4 = if n "/../a" == "/a" then 0 else 8 in
    let c5 = if n "a/./b" == "a/b" then 0 else 16 in
    let c6 = if n "/a/b/" == "/a/b" then 0 else 32 in
    let c7 = if n "." == "." then 0 else 64 in
    let c8 = if n "/" == "/" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#;

const PATH_REL_MAIN: &str = r#"
module Main where

  import Data.Path

  r :: String -> String -> String
  r t bs = path-to-string (path-relative-to (path-from-string t) (path-from-string bs))

  n :: String -> String
  n s = path-to-string (path-normalize (path-from-string s))

  b :: String -> String
  b s = path-basename (path-from-string s)

  d :: String -> String
  d s = path-dirname (path-from-string s)

  e :: String -> String
  e s = path-extname (path-from-string s)

  main :: Int
  main =
    let c1 = if r "/a/b/c" "/a/b" == "c" then 0 else 1 in
    let c2 = if r "/a/b" "/a/b/c" == ".." then 0 else 2 in
    let c3 = if r "/a/c" "/a/b" == "../c" then 0 else 4 in
    let c4 = if n "a/.." == "." then 0 else 8 in
    let c5 = if n "" == "." then 0 else 16 in
    let c6 = if e "b." == "." then 0 else 32 in
    let c7 = if d "/" == "/" then 0 else 64 in
    let c8 = if b "" == "" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#;

#[test]
fn path_basename_and_dirname_table() {
    let code = run_main("path-basename", PATH_BASENAME_MAIN);
    assert_eq!(
        code, 0,
        "path basename/dirname: bitmask {code} (1,2,4 basename; 8,16,32,64 dirname; 128 basename)"
    );
}

#[test]
fn path_extname_stem_and_absoluteness_table() {
    let code = run_main("path-ext", PATH_EXT_MAIN);
    assert_eq!(
        code, 0,
        "path extname/stem/absolute: bitmask {code} (1..8 extname, 16 stem, 32..128 absolute)"
    );
}

#[test]
fn path_normalize_table() {
    let code = run_main("path-normalize", PATH_NORM_MAIN);
    assert_eq!(
        code, 0,
        "path normalize: bitmask {code} (1 /a/b/../c, 2 double pop, 4 leading .. kept, \
         8 absolute ... dropped, 16 dot, 32 trailing slash, 64 dot, 128 root)"
    );
}

#[test]
fn path_relative_to_and_corner_cases_table() {
    let code = run_main("path-relative", PATH_REL_MAIN);
    assert_eq!(
        code, 0,
        "path relative-to/corners: bitmask {code} (1,2,4 relative-to, 8,16 normalize, 32 extname, \
         64 dirname, 128 basename)"
    );
}
