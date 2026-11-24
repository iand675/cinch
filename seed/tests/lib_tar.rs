//! Tape archives against independent reference vectors from `tooling/tar_vectors.go`.
//! Archives produced by the reference writer (USTAR, prefix splitting, PAX for long or
//! non-ASCII names, large ids, negative times and extra records) must be reproduced byte for
//! byte and read back field for field; GNU, PAX-global and V7 archives must read correctly.
//! Malformed archives report precise errors.
mod common;

use serde_json::Value;

/// Render one fixture entry as a checked-language description.
fn describe(e: &Value) -> String {
    let records = e["records"].as_array().map(|rs| rs.iter().map(|r| format!("({:?}, {:?})", r[0].as_str().unwrap(), r[1].as_str().unwrap())).collect::<Vec<_>>().join(", ")).unwrap_or_default();
    format!(
        "Spec {} {:?} {:?} {} {} {} {} ({}) {:?} {:?} {} {} {:?} [{records}]",
        e["kind"], e["name"].as_str().unwrap(), e["link"].as_str().unwrap(), e["mode"], e["uid"], e["gid"], e["size"],
        e["mtime"].as_i64().unwrap().to_string().replace('-', "0 - "), e["uname"].as_str().unwrap(), e["gname"].as_str().unwrap(),
        e["major"], e["minor"], e["data"].as_str().unwrap()
    )
}

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("tar", &[
        ("Text.Tar.cinch", include_str!("../prelude/Text/Tar.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Reference-written archives are reproduced exactly and read back field for field.
#[test]
fn tar_writer_reproduces_reference_archives_and_reader_reads_them() {
    let f: Value = serde_json::from_str(include_str!("fixtures/tar-vectors.json")).unwrap();
    let checks = f["written"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| format!("archive-case True {:?} [{}]", a["archive"].as_str().unwrap(), a["entries"].as_array().unwrap().iter().map(describe).collect::<Vec<_>>().join(", ")))
        .collect::<Vec<_>>();
    run(&checks);
}

/// GNU long names and links, PAX global headers, base-256 numbers and V7 headers read correctly.
#[test]
fn tar_reader_handles_gnu_global_and_v7_archives() {
    let f: Value = serde_json::from_str(include_str!("fixtures/tar-vectors.json")).unwrap();
    let checks = f["read"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| format!("archive-case False {:?} [{}]", a["archive"].as_str().unwrap(), a["entries"].as_array().unwrap().iter().map(describe).collect::<Vec<_>>().join(", ")))
        .collect::<Vec<_>>();
    run(&checks);
}

/// Truncation, corrupted checksums, stray data after a zero block, bad PAX and unrepresentable fields.
#[test]
fn tar_errors_are_precise() {
    let f: Value = serde_json::from_str(include_str!("fixtures/tar-vectors.json")).unwrap();
    let archive = f["written"][0]["archive"].as_str().unwrap();
    run(&[format!("errors {archive:?}")]);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn tar_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Tar", include_str!("../prelude/Text/Tar.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference tar vector consumers.
module Main where
  import Text.Tar
  import Text.Hex
  -- | An expected entry: type, name, link, mode, uid, gid, size, mtime, uname, gname, major, minor, data and extra records (hex).
  data Spec where
    -- | Fields in the order listed above.
    Spec :: Nat -> String -> String -> Nat -> Nat -> Nat -> Nat -> Int -> String -> String -> Nat -> Nat -> String -> List (String, String) -> Spec
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0
  -- | Parse recorded record pairs.
  pairs :: List (String, String) -> List (Bytes, Bytes)
  pairs records = case records of Nil -> Nil; Cons r rest -> case r of (k, v) -> Cons (input k, input v) (pairs rest)
  -- | Build the entry a wanted describes.
  wanted-entry :: Spec -> TarEntry
  wanted-entry wanted = case wanted of
    Spec kind name link mode uid gid size mtime uname gname major minor contents records ->
      tar-entry (tar-with-pax (tar-with-device (tar-with-mtime (tar-with-owner (tar-with-mode (tar-with-link (tar-header kind (input name)) (input link)) mode) uid gid (input uname) (input gname)) mtime) major minor) (pairs records)) (input contents)
  -- | Hex of bytes.
  hex :: Bytes -> String
  hex b = hex-encode b
  -- | Whether a read entry matches its wanted in every field.
  matches :: TarEntry -> Spec -> Bool
  matches entry wanted = case wanted of
    Spec kind name link mode uid gid size mtime uname gname major minor contents records ->
      let h = tar-entry-header entry in
      tar-kind h == kind && hex (tar-name h) == name && hex (tar-link h) == link && tar-mode h == mode
        && tar-uid h == uid && tar-gid h == gid && tar-size h == size && tar-mtime h == mtime
        && hex (tar-uname h) == uname && hex (tar-gname h) == gname && tar-devmajor h == major && tar-devminor h == minor
        && hex (tar-entry-data entry) == contents && same-records (tar-pax h) records
  -- | Records equal pairwise.
  same-records :: List (Bytes, Bytes) -> List (String, String) -> Bool
  same-records actual expected = case (actual, expected) of
    (Nil, Nil) -> True
    (Cons a more, Cons e es) -> (case (a, e) of ((ak, av), (ek, ev)) -> hex ak == ek && hex av == ev) && same-records more es
    other -> False
  -- | Every entry matches in order.
  all-match :: List TarEntry -> List Spec -> Bool
  all-match entries specs = case (entries, specs) of
    (Nil, Nil) -> True
    (Cons e es, Cons s ss) -> matches e s && all-match es ss
    other -> False
  -- | Build the entries a list of specs describes.
  wanted-entries :: List Spec -> List TarEntry
  wanted-entries specs = case specs of Nil -> Nil; Cons s rest -> Cons (wanted-entry s) (wanted-entries rest)
  -- | Read the archive; when written, also reproduce it exactly from the specs.
  archive-case :: Bool -> String -> List Spec -> Bool
  archive-case written archive specs =
    (case tar-read (input archive) of Ok entries -> all-match entries specs; Err e -> False)
      && (if written then (case tar-write (wanted-entries specs) of Ok out -> hex out == archive; Err e -> False) else True)
  -- | A copy with one byte replaced.
  patched :: Bytes -> Nat -> Nat -> Bytes
  patched b i v = let out = bytes-new (bytes-len b) in let c = bytes-copy-range out b 0 0 (bytes-len b) in
    let w = if i < bytes-len out && v < 256 then bytes-set out i v else () in out
  -- | The first count bytes.
  prefix :: Bytes -> Nat -> Bytes
  prefix b n = if n <= bytes-len b then let out = bytes-new n in let c = bytes-copy-range out b 0 0 n in out else b
  -- | Each malformed variant of a valid single-entry archive fails with its precise error.
  errors :: String -> Bool
  errors archive =
    let raw = input archive in
    (case tar-read (prefix raw 300) of Err (TarTruncated 0) -> True; other -> False)
      && (case tar-read (prefix raw 520) of Err (TarTruncated 0) -> True; other -> False)
      && (case tar-read (patched raw 0 120) of Err (TarBadChecksum 0) -> True; other -> False)
      && (case tar-read (patched raw 1536 1) of Err (TarBadHeader 1536) -> True; other -> False)
      && (case tar-read (prefix raw 1024) of Ok entries -> True; other -> False)
      && (case tar-write [tar-entry (tar-with-mode (tar-header tar-regular (bytes-new 1)) 16777216) (bytes-new 0)] of Err (TarFieldTooLong n) -> True; other -> False)
      && tar-error-message (TarBadChecksum 512) == "tar header checksum mismatch at offset 512"
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
