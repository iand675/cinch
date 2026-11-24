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
        ("Text.Flate.cinch", include_str!("../prelude/Text/Flate.cinch")),
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
    checks.push("deflate-levels-round-trip ()".into());
    checks.push(format!("errors {:?}", f["written"][0]["archive"].as_str().unwrap()));
    run(&checks);
}

/// Deflated zip entries compress varied text/JSON with real LZ77 + Huffman DEFLATE
/// and interoperate with Python's `zipfile.ZipFile`.
#[test]
fn zip_deflate_compresses_prose_and_interoperates_with_python() {
    let prose = "{\"event\":\"commit\",\"module\":\"Text.Zip\",\"codec\":\"deflate\",\"status\":\"ok\"}\n".repeat(120);
    let mut main_src = String::from(r#"-- | Write a multi-entry zip archive for Python zipfile verification.
module Main where
  import Data.Bytes
  import Data.Path
  import Data.Result
  import Effect.Fs
  import Text.Flate
  import Text.Zip

  -- | Whether opening an entry produces the expected bytes.
  open-matches :: Bytes -> ZipEntry -> Bytes -> Bool
  open-matches raw entry want = case zip-open raw entry of
    Ok b -> eq-i64 (bytes-compare b want) 0
    Err _ -> False

  -- | Entrypoint writing prose entries at default, best, no-compression, and stored settings.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    case fs-read (path-from-string "PROSE_PATH") of
      Err _ -> 1
      Ok prose ->
        let files = [
          zip-file (string-to-bytes "default.json") zip-deflate prose,
          zip-file-with-level (string-to-bytes "best.json") FlateBestCompression prose,
          zip-file-with-level (string-to-bytes "nocomp.json") FlateNoCompression prose,
          zip-file (string-to-bytes "stored.json") zip-store prose
        ] in
        case zip-write files (string-to-bytes "cinch-flate") of
          Err _ -> 2
          Ok raw ->
            case zip-read raw of
              Err _ -> 3
              Ok archive ->
                case zip-entries archive of
                  Cons e1 (Cons e2 (Cons e3 (Cons e4 Nil))) ->
                    let ok-ratio = zip-entry-compressed-size e1 < bytes-len prose && zip-entry-compressed-size e2 < bytes-len prose && zip-entry-compressed-size e4 == bytes-len prose in
                    let ok-open = open-matches raw e1 prose && open-matches raw e2 prose && open-matches raw e3 prose && open-matches raw e4 prose in
                    if not (ok-ratio && ok-open) then 4
                    else case fs-write (path-from-string "ARCHIVE_PATH") raw of
                      Ok _ -> 0
                      Err _ -> 5
                  _ -> 6
"#);
    let dir = common::project("zip_flate", &[
        ("Text.Zip.cinch", include_str!("../prelude/Text/Zip.cinch")),
        ("Text.Zlib.cinch", include_str!("../prelude/Text/Zlib.cinch")),
        ("Text.Flate.cinch", include_str!("../prelude/Text/Flate.cinch")),
        ("Text.Checksum.cinch", include_str!("../prelude/Text/Checksum.cinch")),
        ("Text.Adler32.cinch", include_str!("../prelude/Text/Adler32.cinch")),
        ("Text.Crc64.cinch", include_str!("../prelude/Text/Crc64.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ]);
    let prose_path = dir.join("prose.json");
    let archive_path = dir.join("out.zip");
    std::fs::write(&prose_path, prose.as_bytes()).unwrap();
    let esc = |p: &std::path::Path| p.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"");
    main_src = main_src
        .replace("PROSE_PATH", &esc(&prose_path))
        .replace("ARCHIVE_PATH", &esc(&archive_path));
    std::fs::write(dir.join("Main.cinch"), main_src).unwrap();
    assert_eq!(common::run(&dir, "Main.cinch"), 0, "zip deflate compression + round-trip check failed");

    let py = std::path::Path::new("/usr/bin/python3");
    if py.exists() {
        let out = std::process::Command::new(py)
            .args([
                "-c",
                r#"
import sys, zipfile
want = open(sys.argv[2], 'rb').read()
with zipfile.ZipFile(sys.argv[1], 'r') as zf:
    assert zf.testzip() is None, 'CRC check failed'
    assert zf.comment == b'cinch-flate'
    for name in ('default.json', 'best.json', 'nocomp.json', 'stored.json'):
        info = zf.getinfo(name)
        assert zf.read(name) == want, f'mismatch for {name}'
        if name in ('default.json', 'best.json'):
            assert info.compress_type == zipfile.ZIP_DEFLATED
            assert info.compress_size < len(want) // 10, f'poor compression for {name}: {info.compress_size}/{len(want)}'
        elif name == 'stored.json':
            assert info.compress_type == zipfile.ZIP_STORED
            assert info.compress_size == len(want)
"#,
            ])
            .arg(&archive_path)
            .arg(&prose_path)
            .output()
            .expect("spawn python3");
        assert!(
            out.status.success(),
            "python3 zipfile verification failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    common::cleanup(&dir);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn zip_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Zip", include_str!("../prelude/Text/Zip.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference zip vector consumers.
module Main where
  import Data.Bytes.Build
  import Text.Flate
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
  -- | Fill a run of one byte, which DEFLATE encoding shrinks.
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
  -- | Explicit compression levels compress non-RLE repeated patterns and round-trip.
  deflate-levels-round-trip :: Unit -> Bool
  deflate-levels-round-trip unit =
    let chunk = string-to-bytes "alpha beta gamma delta epsilon zeta 0123456789\n" in
    let sample = bytes-concat [chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk] in
    case zip-write [zip-file-with-level (string-to-bytes "fast.txt") FlateBestSpeed sample, zip-file-with-level (string-to-bytes "best.txt") FlateBestCompression sample] (bytes-new 0) of
      Err e -> False
      Ok raw -> case zip-read raw of
        Err e -> False
        Ok a -> case zip-entries a of
          Cons e1 (Cons e2 Nil) ->
            zip-entry-method e1 == zip-deflate && zip-entry-method e2 == zip-deflate
              && zip-entry-compressed-size e1 < bytes-len sample
              && zip-entry-compressed-size e2 < bytes-len sample
              && (case zip-open raw e1 of Ok out -> hex out == hex sample; Err x -> False)
              && (case zip-open raw e2 of Ok out -> hex out == hex sample; Err x -> False)
          other -> False
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
