//! Round 2 Stdlib & Data Structures Performance + Linear-Type Safety Verification Suite.
//! Covers:
//!   1. Text.Base64: O(1) stack on 150KB inputs, SIMD encode/decode/lenient parity, UTF-8 error edge cases.
//!   2. Text.Bzip2: all 48 libbzip2 reference vectors (multi-block, multi-stream, RLE, long Huffman codes) + errors.
//!   3. Text.Crc64: 4-way CLMUL ISO/ECMA & custom polynomial bit-for-bit parity across lengths 0..256 and 64KB.
//!   4. Text.Lzw: all 270 reference vectors (LSB & MSB, widths 2..8, >4096 dictionary resets) + bounded/errors.
//!   5. Data.Array.Search: exhaustive parity of EytzingerIndex and 16-way STreeI64 against binary search.
//!   6. Static `%1` linear-type rejection checks + `CINCH_LEAK_CHECK=1` (`leaked=0`) across all runtime tests.
mod common;

use cinch::driver::{DriverOptions, Mode};
use serde_json::Value;
use std::path::PathBuf;

fn build_and_run_leak_checked(tag: &str, files: &[(&str, &str)], stack_size: &str) {
    let dir = common::project(tag, files);
    let exe = dir.join("runner");
    let entry = dir.join("Main.cinch");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        ..cinch::core::TargetProfile::native()
    };
    let (e_clone, o_clone, p_clone) = (entry.clone(), exe.clone(), profile.clone());
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            cinch::driver::driver_with_profile_opts(
                Mode::Build,
                cinch::check::StageMode::Refresh,
                &e_clone,
                Some(&o_clone),
                &p_clone,
                &DriverOptions {
                    opt_level: Some(2),
                    ..Default::default()
                },
            )
        })
        .unwrap()
        .join()
        .unwrap()
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));

    let output = std::process::Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "2G")
        .env("CINCH_STACK_SIZE", stack_size)
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(0),
        "{tag}: nonzero exit {:?}, stderr:\n{stderr}",
        output.status.code()
    );
    assert!(
        stderr.contains("leaked=0"),
        "{tag}: expected leaked=0 in stderr, got:\n{stderr}"
    );
    common::cleanup(&dir);
}

#[test]
fn r2_base64_large_stack_safety_and_edge_cases() {
    let main = r#"module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Char
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Base64

  fill :: Bytes -> Nat -> Unit
  fill b i =
    if i < bytes-len b then
      let w = bytes-set b i (nat-rem (i * 131 + 17) 256) in
      fill b (i + 1)
    else ()

  make-buf :: Nat -> Bytes
  make-buf n = let b = bytes-new n in let u = fill b 0 in b

  roundtrip-std :: Nat -> Bool
  roundtrip-std n =
    let src = make-buf n in
    let enc = base64-encode src in
    let expected-len = nat-mul (nat-div (n + 2) 3) 4 in
    string-byte-length enc == expected-len
      && (case base64-decode enc of
            Ok dec -> eq-i64 (bytes-compare dec src) 0
            Err _ -> False)
      && eq-i64 (bytes-compare (base64-decode-lenient enc) src) 0

  roundtrip-url :: Nat -> Bool
  roundtrip-url n =
    let src = make-buf n in
    let enc = base64-url-encode src in
    case base64-url-decode enc of
      Ok dec -> eq-i64 (bytes-compare dec src) 0
      Err _ -> False

  utf8-error-checks :: Bool
  utf8-error-checks =
    -- 4 scalars where one is 2-byte UTF-8 'é' (U+00E9): byte len 5, scalar len 4 -> InvalidBase64Character
    (case base64-decode "ABéD" of
      Err (InvalidBase64Character c) -> char-code c == 233
      _ -> False)
    -- 2 scalars 'éé': byte len 4, scalar len 2 -> InvalidBase64Length 2
    && (case base64-decode "éé" of
      Err (InvalidBase64Length n) -> n == 2
      _ -> False)
    -- misplaced padding after large valid SIMD prefix (16 valid + bad pad group)
    && (case base64-decode "AAAAAAAAAAAAAAAA=AAA" of
      Err InvalidBase64Padding -> True
      _ -> False)
    -- invalid char after 16-byte SIMD block
    && (case base64-decode "AAAAAAAAAAAAAAAAAA!A" of
      Err (InvalidBase64Character c) -> char-code c == 33
      _ -> False)
    -- lenient with embedded newlines every 16 chars
    && eq-i64 (bytes-compare (base64-decode-lenient "Zm9vYmFy\nZm9vYmFy\r\n!!") (string-to-bytes "foobarfoobar")) 0

  main :: Int
  main =
    let c1 = if roundtrip-std 0 && roundtrip-std 1 && roundtrip-std 2 && roundtrip-std 11 && roundtrip-std 12 && roundtrip-std 13 && roundtrip-std 15 && roundtrip-std 16 && roundtrip-std 17 then 0 else 1 in
    let c2 = if roundtrip-std 153601 && roundtrip-std 153602 && roundtrip-std 153600 then 0 else 2 in
    let c3 = if roundtrip-url 153601 && roundtrip-url 65537 then 0 else 4 in
    let c4 = if utf8-error-checks then 0 else 8 in
    c1 + c2 + c3 + c4
"#;
    build_and_run_leak_checked(
        "r2_base64",
        &[
            ("Text.Base64.cinch", include_str!("../prelude/Text/Base64.cinch")),
            ("Main.cinch", main),
        ],
        "4M",
    );
}

#[test]
fn r2_bzip2_all_48_reference_vectors_and_errors() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/bzip2-vectors.json")).unwrap();
    let rows = fixture["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 48);

    for (bi, chunk) in rows.chunks(24).enumerate() {
        let checks: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "decodes {} {} {:?} {:?}",
                    r["kind"],
                    r["size"],
                    r["packed"].as_str().unwrap(),
                    r["digest"].as_str().unwrap()
                )
            })
            .collect();
        let main = format!(
            r#"module Main where
  import Data.Bytes
  import Data.Bytes.Build
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Bzip2
  import Text.Hex

  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0

  expected :: Nat -> Nat -> Bytes
  expected kind size = if kind == 6 then bytes-concat [corpus 2 700, corpus 4 900] else corpus kind size

  corpus :: Nat -> Nat -> Bytes
  corpus kind size = let out = bytes-new size in let filled = fill out kind 0 12345 in out

  fill :: Bytes -> Nat -> Nat -> U32 -> Unit
  fill out kind i state =
    if i < bytes-len out then
      let next = wrapping-add-u32 (wrapping-mul-u32 state 1103515245) 12345 in
      let value = value-of kind i next in
      let written = if value < 256 then bytes-set out i value else () in
      fill out kind (i + 1) next
    else ()

  value-of :: Nat -> Nat -> U32 -> Nat
  value-of kind i s =
    if kind == 0 then nat-rem (i * 37 + 11) 256
    else if kind == 1 then 97 + nat-rem i 3
    else if kind == 2 then u32-to-nat (bit-and-u32 (wrapping-shr-u32 s 16) 255)
    else if kind == 3 then nat-rem (nat-div i 300) 7 * 40
    else if kind == 4 then 65 + u32-to-nat (bit-and-u32 (wrapping-shr-u32 s 16) 3)
    else u32-to-nat (trailing-zeros-u32 s)

  decodes :: Nat -> Nat -> String -> String -> Bool
  decodes kind size packed digest = case bzip2-decompress (input packed) of
    Ok out ->
      let exp = expected kind size in
      bytes-len out == bytes-len exp && eq-i64 (bytes-compare out exp) 0 && hex-encode (sha256-bytes out) == digest
    Err e -> False

  all-ok :: List Bool -> Bool
  all-ok xs = case xs of Nil -> True; Cons x rest -> if x then all-ok rest else False

  main :: Int
  main = if all-ok [{}] then 0 else 1
"#,
            checks.join(",\n    ")
        );
        build_and_run_leak_checked(
            &format!("r2_bzip2_b{bi}"),
            &[
                ("Text.Bzip2.cinch", include_str!("../prelude/Text/Bzip2.cinch")),
                ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
                ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
                ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
                ("Main.cinch", &main),
            ],
            "8M",
        );
    }
}

#[test]
fn r2_crc64_clmul_iso_ecma_and_custom_parity() {
    let main = r#"module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Text.Crc64

  -- Independent bit-by-bit reflected CRC-64 reference
  ref-bit :: U64 -> U64 -> U64
  ref-bit acc poly =
    let m = wrapping-sub-u64 0 (bit-and-u64 acc 1) in
    bit-xor-u64 (wrapping-shr-u64 acc 1) (bit-and-u64 poly m)

  ref-byte :: U64 -> U64 -> U64
  ref-byte acc poly =
    let a1 = ref-bit acc poly in let a2 = ref-bit a1 poly in
    let a3 = ref-bit a2 poly in let a4 = ref-bit a3 poly in
    let a5 = ref-bit a4 poly in let a6 = ref-bit a5 poly in
    let a7 = ref-bit a6 poly in ref-bit a7 poly

  ref-loop :: U64 -> Bytes -> Nat -> U64 -> U64
  ref-loop poly b i acc =
    if i < bytes-len b then
      let v = u8-to-u64 (bytes-get-u8 b i) in
      ref-loop poly b (i + 1) (ref-byte (bit-xor-u64 acc v) poly)
    else acc

  ref-crc64 :: U64 -> U64 -> Bytes -> U64
  ref-crc64 poly prev b = bit-not-u64 (ref-loop poly b 0 (bit-not-u64 prev))

  fill :: Bytes -> Nat -> Unit
  fill b i =
    if i < bytes-len b then
      let w = bytes-set b i (nat-rem (i * 53 + 29) 256) in
      fill b (i + 1)
    else ()

  make-buf :: Nat -> Bytes
  make-buf n = let b = bytes-new n in let u = fill b 0 in b

  check-one :: U64 -> Crc64Table -> U64 -> Nat -> Bool
  check-one poly tbl seed n =
    let b = make-buf n in
    let exp = ref-crc64 poly seed b in
    eq-u64 (crc64-update poly seed b) exp
      && eq-u64 (crc64-table-update tbl seed b) exp

  check-range :: U64 -> Crc64Table -> Nat -> Nat -> Bool
  check-range poly tbl i end =
    if i > end then True
    else if check-one poly tbl 0 i && check-one poly tbl 0x123456789ABCDEF0 i then
      check-range poly tbl (i + 1) end
    else False

  check-poly :: U64 -> Bool
  check-poly poly =
    let tbl = crc64-table poly in
    check-range poly tbl 0 260
      && check-one poly tbl 0 511
      && check-one poly tbl 0 512
      && check-one poly tbl 0 513
      && check-one poly tbl 0xFEDCBA9876543210 4096
      && check-one poly tbl 0 65536

  main :: Int
  main =
    let c1 = if check-poly crc64-ecma then 0 else 1 in
    let c2 = if check-poly crc64-iso then 0 else 2 in
    let c3 = if check-poly 0x42F0E1EBA9EA3693 then 0 else 4 in
    c1 + c2 + c3
"#;
    build_and_run_leak_checked(
        "r2_crc64",
        &[
            ("Text.Crc64.cinch", include_str!("../prelude/Text/Crc64.cinch")),
            ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Main.cinch", main),
        ],
        "8M",
    );
}

#[test]
fn r2_lzw_all_270_reference_vectors_and_bounded() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/lzw-vectors.json")).unwrap();
    let arr = rows.as_array().unwrap();
    assert_eq!(arr.len(), 270);

    for (bi, chunk) in arr.chunks(90).enumerate() {
        let checks: Vec<String> = chunk
            .iter()
            .map(|r| {
                format!(
                    "lzw-case {} {} {} {} {:?} {:?} {}",
                    r["order"],
                    r["width"],
                    r["kind"],
                    r["size"],
                    r["encoded"].as_str().unwrap(),
                    r["digest"].as_str().unwrap(),
                    r["length"]
                )
            })
            .collect();
        let main = format!(
            r#"module Main where
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Hex
  import Text.Lzw

  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0

  pow :: Nat -> Nat
  pow n = if n == 0 then 1 else 2 * pow (n - 1)

  corpus :: Nat -> Nat -> Nat -> Bytes
  corpus kind width size = let out = bytes-new size in let filled = fill out kind width 0 12345 in out

  fill :: Bytes -> Nat -> Nat -> Nat -> U32 -> Unit
  fill out kind width i state =
    if i < bytes-len out then
      let next = wrapping-add-u32 (wrapping-mul-u32 state 1103515245) 12345 in
      let value = value-of kind width i next in
      let written = if value < 256 then bytes-set out i value else () in
      fill out kind width (i + 1) (if kind == 2 then next else state)
    else ()

  value-of :: Nat -> Nat -> Nat -> U32 -> Nat
  value-of kind width i next = reduce (if kind == 0 then i * 37 + 11 else if kind == 1 then nat-rem i 3 else u8-to-nat (wrapping-u32-to-u8 (wrapping-shr-u32 next 16))) (pow width)

  reduce :: Nat -> Nat -> Nat
  reduce value bound = if bound > 0 then nat-rem value bound else 0

  order :: Nat -> LzwOrder
  order code = if code == 0 then LzwLsb else LzwMsb

  decodes :: LzwOrder -> Nat -> Bytes -> Bytes -> Bool
  decodes o width encoded expected = case lzw-decompress o width encoded of
    Ok out -> eq-i64 (bytes-compare out expected) 0
    Err e -> False

  lzw-case :: Nat -> Nat -> Nat -> Nat -> String -> String -> Nat -> Bool
  lzw-case code width kind size encoded digest length =
    let plain = corpus kind width size in
    case lzw-compress (order code) width plain of
      Err e -> False
      Ok ours ->
        bytes-len ours == length && hex-encode (sha256-bytes ours) == digest
          && (if bytes-len (input encoded) > 0 then hex-encode ours == encoded && decodes (order code) width (input encoded) plain else True)
          && decodes (order code) width ours plain

  all-ok :: List Bool -> Bool
  all-ok xs = case xs of Nil -> True; Cons x rest -> if x then all-ok rest else False

  main :: Int
  main = if all-ok [{}] then 0 else 1
"#,
            checks.join(",\n    ")
        );
        build_and_run_leak_checked(
            &format!("r2_lzw_b{bi}"),
            &[
                ("Text.Lzw.cinch", include_str!("../prelude/Text/Lzw.cinch")),
                ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
                ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
                ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
                ("Main.cinch", &main),
            ],
            "8M",
        );
    }
}

#[test]
fn r2_array_search_eytzinger_and_stree_exhaustive_parity() {
    let main = r#"module Main where
  import Data.Array
  import Data.Array.Search
  import Data.Bool
  import Data.Compare
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Ordering

  nat-to-i64-safe :: Nat -> I64
  nat-to-i64-safe x = if x <= 9223372036854775807 then nat-to-i64 x else 0

  -- Build sorted arrays of various shapes:
  -- mode 0: spaced distinct (i * 3 - 50)
  -- mode 1: all duplicates (42)
  -- mode 2: clustered duplicates (i / 3 * 5 - 20)
  -- mode 3: extremes (min-bound-i64, 0, max-bound-i64)
  elem-for :: Nat -> Nat -> Nat -> I64
  elem-for mode n i =
    let ii = nat-to-i64-safe i in
    if mode == 0 then wrapping-sub-i64 (wrapping-mul-i64 ii 3) 50
    else if mode == 1 then 42
    else if mode == 2 then
      let qi = nat-to-i64-safe (nat-div i 3) in
      wrapping-sub-i64 (wrapping-mul-i64 qi 5) 20
    else
      let t = nat-div n 3 in
      if i < t then min-bound-i64
      else if i < 2 * t then 0
      else max-bound-i64

  build-list :: Nat -> Nat -> Nat -> List I64 -> List I64
  build-list mode n i acc =
    if i == 0 then acc
    else
      let idx = i - 1 in
      build-list mode n idx (Cons (elem-for mode n idx) acc)

  make-arr :: Nat -> Nat -> Array I64
  make-arr mode n = array-from-list (build-list mode n n Nil)

  check-query :: Array I64 -> EytzingerIndex I64 -> STreeI64 -> I64 -> Bool
  check-query arr eytz st key =
    case array-binary-search arr key of
      (b-idx, b-found) ->
        let e-lb = array-eytzinger-lower-bound-by compare eytz key in
        let s-lb = stree-i64-lower-bound st key in
        let s-has = stree-i64-contains st key in
        case array-eytzinger-search eytz key of
          (e-idx, e-found) ->
            b-idx == e-lb && b-idx == e-idx && b-idx == s-lb
              && (b-found == e-found) && (b-found == s-has)

  check-keys :: Array I64 -> EytzingerIndex I64 -> STreeI64 -> List I64 -> Bool
  check-keys arr eytz st ks = case ks of
    Nil -> True
    Cons k rest -> if check-query arr eytz st k then check-keys arr eytz st rest else False

  check-range-keys :: Array I64 -> EytzingerIndex I64 -> STreeI64 -> I64 -> I64 -> Bool
  check-range-keys arr eytz st lo hi =
    if gt-i64 lo hi then True
    else if check-query arr eytz st lo then
      check-range-keys arr eytz st (wrapping-add-i64 lo 1) hi
    else False

  check-array :: Nat -> Nat -> Bool
  check-array mode n =
    let arr = make-arr mode n in
    let eytz = array-eytzinger-from-sorted arr in
    let st = stree-i64-from-sorted arr in
    stree-i64-length st == n
      && check-keys arr eytz st [min-bound-i64, wrapping-add-i64 min-bound-i64 1, -100, -51, -50, -49, -1, 0, 1, 41, 42, 43, wrapping-sub-i64 max-bound-i64 1, max-bound-i64]
      && check-range-keys arr eytz st (-60) 120

  check-sizes :: Nat -> Nat -> Bool
  check-sizes n max-n =
    if n > max-n then True
    else if check-array 0 n && check-array 1 n && check-array 2 n && check-array 3 n then
      check-sizes (n + 1) max-n
    else False

  check-large-random :: Array I64 -> EytzingerIndex I64 -> STreeI64 -> Nat -> U64 -> Bool
  check-large-random arr eytz st queries state =
    if queries == 0 then True
    else
      let nxt = wrapping-add-u64 (wrapping-mul-u64 state 6364136223846793005) 1442695040888963407 in
      let raw = wrapping-u64-to-i64 (bit-and-u64 (wrapping-shr-u64 nxt 33) 16383) in
      let key = wrapping-sub-i64 raw 100 in
      if check-query arr eytz st key then
        check-large-random arr eytz st (queries - 1) nxt
      else False

  check-large :: Nat -> Bool
  check-large n =
    let arr = make-arr 0 n in
    let eytz = array-eytzinger-from-sorted arr in
    let st = stree-i64-from-sorted arr in
    stree-i64-length st == n
      && check-keys arr eytz st [min-bound-i64, -51, -50, 0, max-bound-i64]
      && check-large-random arr eytz st 500 42

  main :: Int
  main =
    let c1 = if check-sizes 0 35 then 0 else 1 in
    let c2 = if check-large 255 && check-large 256 && check-large 257 then 0 else 2 in
    let c3 = if check-large 1000 && check-large 4096 && check-large 4097 then 0 else 4 in
    c1 + c2 + c3
"#;
    build_and_run_leak_checked(
        "r2_search",
        &[
            ("Data.Search.cinch", include_str!("../prelude/Data/Search.cinch")),
            ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
            ("Data.Array.Search.cinch", include_str!("../prelude/Data/Array/Search.cinch")),
            ("Data.Compare.cinch", include_str!("../prelude/Data/Compare.cinch")),
            ("Main.cinch", main),
        ],
        "8M",
    );
}

#[test]
fn r2_linear_type_safety_static_rejection_and_docs() {
    // 1. Canonical docs on Bzip2, Crc64, Lzw
    common::assert_documented("Text.Bzip2", include_str!("../prelude/Text/Bzip2.cinch"));
    common::assert_documented("Text.Crc64", include_str!("../prelude/Text/Crc64.cinch"));
    common::assert_documented("Text.Lzw", include_str!("../prelude/Text/Lzw.cinch"));

    // 2. Verify static `%1` linearity rejects duplicating a linearly-owned Bytes buffer
    let dup_dir = common::project(
        "r2_lin_dup",
        &[(
            "Main.cinch",
            r#"module Main where
  import Data.Bytes
  import Data.Machine

  put-u8-raw :: Bytes -> I64 -> U8 -> Bytes
  put-u8-raw b pos v = if 0 <= pos && pos < bytes-len-i64 b then let w = bytes-set-u8-at b pos v in b else b

  bad-dup :: Bytes %1 -> Bytes
  bad-dup b =
    let b1 = put-u8-raw b 0 1 in
    put-u8-raw b 1 2

  main :: Int
  main = 0
"#,
        )],
    );
    let dup_err = common::drive(Mode::Check, &dup_dir, "Main.cinch", None)
        .expect_err("duplicating a %1 Bytes buffer must fail static linearity checking");
    assert!(
        dup_err.contains("is used 2 time(s)"),
        "unexpected linearity error: {dup_err}"
    );
    common::cleanup(&dup_dir);

    // 3. Verify static `%1` linearity rejects dropping a linearly-owned Bytes buffer on an error path
    let drop_dir = common::project(
        "r2_lin_drop",
        &[(
            "Main.cinch",
            r#"module Main where
  import Data.Bytes
  import Data.Result

  bad-drop :: Bool -> Bytes %1 -> Result Nat Bytes
  bad-drop ok b = if ok then Ok b else Err 1

  main :: Int
  main = 0
"#,
        )],
    );
    let drop_err = common::drive(Mode::Check, &drop_dir, "Main.cinch", None)
        .expect_err("dropping a %1 Bytes buffer on an error path must fail static linearity checking");
    assert!(
        drop_err.contains("unused on some path"),
        "unexpected linearity error: {drop_err}"
    );
    common::cleanup(&drop_dir);
    let _ = PathBuf::new();
}
