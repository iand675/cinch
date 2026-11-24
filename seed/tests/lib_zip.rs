//! Zip archives against independent reference vectors from `tooling/zip_vectors.go`: stored
//! archives are reproduced byte for byte (directories, Unix modes, UTF-8 flags, comments,
//! extended timestamps) and read back; deflated and Zip64 archives are read, inflated and
//! checksum-verified; deflated writing round-trips; corruption is reported precisely.
mod common;

use serde_json::Value;

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("zip", &[
        ("Text.Zip.cinch", include_str!("../prelude/Text/Zip.cinch")),
        ("Text.Zlib.cinch", include_str!("../prelude/Text/Zlib.cinch")),
        ("Text.Checksum.cinch", include_str!("../prelude/Text/Checksum.cinch")),
        ("Text.Adler32.cinch", include_str!("../prelude/Text/Adler32.cinch")),
        ("Text.Crc64.cinch", include_str!("../prelude/Text/Crc64.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Render an archive's entries as checked-language expectations.
fn entries(a: &Value) -> String {
    a["entries"].as_array().unwrap().iter().map(|e| format!(
        "Want {:?} {:?} {} {} {} {} {} ({}) {} {:?} {}",
        e["name"].as_str().unwrap(), e["comment"].as_str().unwrap(), e["method"], e["flags"], e["crc"], e["size"], e["packed"],
        e["modified"], e["mode"], e["data"].as_str().unwrap(), if e["timed"].as_bool().unwrap_or(false) { "True" } else { "False" }
    )).collect::<Vec<_>>().join(", ")
}

/// Stored archives are written identically and every archive reads back field for field.
#[test]
fn zip_matches_reference_archives() {
    let f: Value = serde_json::from_str(include_str!("fixtures/zip-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for a in f["written"].as_array().unwrap() {
        checks.push(format!("archive-case True {:?} {:?} [{}]", a["archive"].as_str().unwrap(), a["comment"].as_str().unwrap(), entries(a)));
    }
    for a in f["read"].as_array().unwrap() {
        checks.push(format!("archive-case False {:?} {:?} [{}]", a["archive"].as_str().unwrap(), a["comment"].as_str().unwrap(), entries(a)));
    }
    checks.push("deflate-round-trip ()".into());
    checks.push(format!("errors {:?}", f["written"][0]["archive"].as_str().unwrap()));
    run(&checks);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn zip_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Zip", include_str!("../prelude/Text/Zip.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference zip vector consumers.
module Main where
  import Text.Zip
  import Text.Hex
  -- | An expected entry: name, comment, method, flags, CRC-32, size, compressed size, modified time, mode, data (hex)
  -- and whether the file was written with a modification time.
  data Want where
    -- | Fields in the order listed above.
    Want :: String -> String -> Nat -> Nat -> Nat -> Nat -> Nat -> Int -> Nat -> String -> Bool -> Want
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input t = case hex-decode t of Ok b -> b; Err e -> bytes-new 0
  -- | Hex of bytes.
  hex :: Bytes -> String
  hex b = hex-encode b
  -- | The file a written expectation describes.
  wanted-file :: Want -> ZipFile
  wanted-file w = case w of
    Want name comment method flags crc size packed modified mode contents timed ->
      let base = zip-file-with-comment (zip-file (input name) method (input contents)) (input comment) in
      let moded = if mode > 0 then zip-file-with-mode base mode else base in
      if timed then zip-file-with-modified moded modified else moded
  -- | All wanted files.
  wanted-files :: List Want -> List ZipFile
  wanted-files ws = case ws of Nil -> Nil; Cons w rest -> Cons (wanted-file w) (wanted-files rest)
  -- | An entry matches its expectation, and opening it yields the data.
  matches :: Bytes -> ZipEntry -> Want -> Bool
  matches raw e w = case w of
    Want name comment method flags crc size packed modified mode contents timed ->
      hex (zip-entry-name e) == name && hex (zip-entry-comment e) == comment && zip-entry-method e == method && zip-entry-flags e == flags
        && u32-to-nat (zip-entry-crc32 e) == crc && zip-entry-size e == size && zip-entry-compressed-size e == packed
        && zip-entry-modified e == modified && zip-entry-mode e == mode
        && (case zip-open raw e of Ok out -> hex out == contents; Err x -> False)
  -- | Every entry matches in order.
  all-match :: Bytes -> List ZipEntry -> List Want -> Bool
  all-match raw es ws = case (es, ws) of
    (Nil, Nil) -> True
    (Cons e more, Cons w rest) -> matches raw e w && all-match raw more rest
    other -> False
  -- | Read the archive, and when written by the reference, also reproduce it exactly.
  archive-case :: Bool -> String -> String -> List Want -> Bool
  archive-case written archive comment ws =
    (case zip-read (input archive) of Ok a -> hex (zip-comment a) == comment && all-match (input archive) (zip-entries a) ws; Err e -> False)
      && (if written then (case zip-write (wanted-files ws) (input comment) of Ok out -> hex out == archive; Err e -> False) else True)
  -- | Repetitive data.
  repeated :: Nat -> Bytes
  repeated n = let b = bytes-new n in let f = fill b 0 in b
  -- | Fill a run of one byte, which run-length DEFLATE encoding shrinks.
  fill :: Bytes -> Nat -> Unit
  fill b i = if i < bytes-len b then let w = bytes-set b i 97 in fill b (i + 1) else ()
  -- | Deflated writing reads back with matching data and shrinks repetitive input.
  deflate-round-trip :: Unit -> Bool
  deflate-round-trip unit =
    let sample = repeated 4000 in
    case zip-write [zip-file (string-to-bytes "a.txt") zip-deflate sample, zip-file (string-to-bytes "d/") zip-deflate (bytes-new 0)] (bytes-new 0) of
      Err e -> False
      Ok raw -> bytes-len raw < 2000 && (case zip-read raw of
        Ok a -> (case zip-entries a of
          Cons e (Cons d Nil) -> (case zip-open raw e of Ok out -> hex out == hex sample; Err x -> False) && zip-entry-is-directory d && zip-entry-method d == zip-store
          other -> False)
        Err e -> False)
  -- | A copy with one byte incremented.
  flip :: Bytes -> Nat -> Bytes
  flip b i = let out = bytes-new (bytes-len b) in let c = bytes-copy-range out b 0 0 (bytes-len b) in
    let v = if i < bytes-len out then nat-rem (bytes-get out i + 1) 256 else 0 in let w = if i < bytes-len out && v < 256 then bytes-set out i v else () in out
  -- | Corrupted archives fail with their precise errors.
  errors :: String -> Bool
  errors archive =
    let raw = input archive in
    (case zip-read (bytes-new 10) of Err ZipNotFound -> True; other -> False)
      && (case zip-read raw of
        Ok a -> (case zip-entries a of
          Cons e rest -> (case zip-open (flip raw 40) e of Err (ZipChecksumMismatch x y) -> True; other -> False)
            && (case zip-open (flip raw 0) e of Err (ZipBadEntry 0) -> True; other -> False)
          Nil -> False)
        Err e -> False)
      && zip-error-message ZipNotFound == "zip: end of central directory not found"
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
