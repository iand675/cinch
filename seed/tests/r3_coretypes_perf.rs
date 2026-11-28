//! Round 3 Core Fundamental Types Performance + Linear-Type Safety Verification Suite.
//! Covers:
//!   1. Data.Bytes / Data.String: wyhash/rapidhash determinism, single-byte avalanche,
//!      length classes 0..129 & 64KB, String/Bytes hash agreement, and O(1)-stack
//!      `%1`-linear `bytes-from-list` / `bytes-from-u8s`.
//!   2. Text.Scan: SIMD Muła substring filter (`string-index-of` / `string-contains`),
//!      SIMD `string-char-index-of`, O(1)-alloc `string-starts-with` / `string-ends-with`,
//!      scalar-index UTF-8 parity, direct slicing (`split-on`, `lines`, `words`, `take`,
//!      `drop`, `strip`, `chomp`, `chop`), and `%1`-linear transforms (`upcase`, `downcase`,
//!      `capitalize`, `normalize-newlines`, `pad-left`, `pad-right`, `center`).
//!   3. Data.HashMap: unboxed `I64` heights + `I64` digest keys + `SBin1`/`SBinMany`
//!      promotion/demotion, 20k `hash-map-from-pairs` stack safety & earlier-pair-wins,
//!      full collision buckets, bignum `Hash` (> 2^63-1) modular fold, delete-to-empty,
//!      and `union` / `union-with` / `map` / `filter` / `fold`.
//!   4. Static `%1` linear-type rejection checks + `CINCH_LEAK_CHECK=1` (`leaked=0`).
mod common;

use cinch::driver::{DriverOptions, Mode};

fn build_and_run_leak_checked(tag: &str, files: &[(&str, &str)], stack_size: &str) {
    let dir = common::project(tag, files);
    let exe = dir.join("runner");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile_opts(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &profile,
        &DriverOptions {
            opt_level: Some(2),
            ..Default::default()
        },
    )
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
fn r3_bytes_wyhash_and_linear_builders() {
    let main = r#"module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Hash
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String

  fill :: Bytes -> I64 -> I64 -> Unit
  fill b i n =
    if lt-i64 i n then
      let v = wrapping-i64-to-u8 (wrapping-add-i64 37 (wrapping-mul-i64 i 131)) in
      let w = if 0 <= i && i < bytes-len-i64 b then bytes-set-u8-at b i v else () in
      fill b (wrapping-add-i64 i 1) n
    else ()

  make-buf :: Nat -> Bytes
  make-buf n =
    let b = bytes-new n in
    let u = fill b 0 (bytes-len-i64 b) in
    b

  -- Determinism and Eq => Hash agreement across lengths 0..129
  check-lengths :: Nat -> Nat -> Bool
  check-lengths i hi =
    if hi < i then True
    else
      let a = make-buf i in
      let b = make-buf i in
      if a == b && hash a == hash b then check-lengths (i + 1) hi
      else False

  -- Flipping byte at offset `pos` in a buffer of length `len` must change the hash
  flip-differs :: Nat -> Nat -> Bool
  flip-differs len pos =
    let a = make-buf len in
    let b = make-buf len in
    let u = if pos < bytes-len b then
              let orig = bytes-get b pos in
              let next = nat-rem (orig + 1) 256 in
              bytes-set b pos next
            else () in
    a /= b && hash a /= hash b

  check-avalanche :: Bool
  check-avalanche =
    flip-differs 1 0
    && flip-differs 2 0 && flip-differs 2 1
    && flip-differs 3 0 && flip-differs 3 1 && flip-differs 3 2
    && flip-differs 4 0 && flip-differs 4 3
    && flip-differs 7 0 && flip-differs 7 3 && flip-differs 7 6
    && flip-differs 8 0 && flip-differs 8 7
    && flip-differs 15 0 && flip-differs 15 7 && flip-differs 15 14
    && flip-differs 16 0 && flip-differs 16 8 && flip-differs 16 15
    && flip-differs 17 0 && flip-differs 17 16
    && flip-differs 32 0 && flip-differs 32 16 && flip-differs 32 31
    && flip-differs 48 0 && flip-differs 48 24 && flip-differs 48 47
    && flip-differs 49 0 && flip-differs 49 48
    && flip-differs 128 0 && flip-differs 128 48 && flip-differs 128 96 && flip-differs 128 127
    && flip-differs 65536 0 && flip-differs 65536 32768 && flip-differs 65536 65535

  -- String and Bytes hashes must agree across every length class + UTF-8
  str-agrees :: String -> Bool
  str-agrees s = hash s == hash (string-to-bytes s)

  check-string-agreement :: Bool
  check-string-agreement =
    str-agrees ""
    && str-agrees "a"
    && str-agrees "ab"
    && str-agrees "abc"
    && str-agrees "abcd"
    && str-agrees "abcdefg"
    && str-agrees "abcdefgh"
    && str-agrees "compiler_symbol!"
    && str-agrees "caf\u{e9} na\u{ef}ve \u{03b1}\u{03b2}\u{03b3} \u{1f600}"
    && str-agrees (string-repeat 60 "abcdefghij")

  -- O(1)-stack linear bytes-from-list and bytes-from-u8s on 10,000 elements
  make-nat-list :: Nat -> List (InRange 0 255 Nat) -> List (InRange 0 255 Nat)
  make-nat-list n acc =
    if n == 0 then acc
    else make-nat-list (n - 1) (Cons (nat-rem n 256) acc)

  make-u8-list :: Nat -> List U8 -> List U8
  make-u8-list n acc =
    if n == 0 then acc
    else
      let v = wrapping-i64-to-u8 (nat-to-i64 (nat-rem n 256)) in
      make-u8-list (n - 1) (Cons v acc)

  verify-seq :: Bytes -> Nat -> Nat -> Bool
  verify-seq b i n =
    if i >= n then True
    else if i < bytes-len b && bytes-get b i == nat-rem (i + 1) 256 then
      verify-seq b (i + 1) n
    else False

  check-linear-builders :: Bool
  check-linear-builders =
    let nl = make-nat-list 10000 Nil in
    let ul = make-u8-list 10000 Nil in
    let b1 = bytes-from-list nl in
    let b2 = bytes-from-u8s ul in
    bytes-len b1 == 10000
    && bytes-len b2 == 10000
    && b1 == b2
    && hash b1 == hash b2
    && verify-seq b1 0 10000
    && bytes-len (bytes-from-list Nil) == 0
    && bytes-len (bytes-from-u8s Nil) == 0

  main :: Int
  main =
    let c1 = if check-lengths 0 129 then 0 else 1 in
    let c2 = if check-avalanche then 0 else 2 in
    let c3 = if check-string-agreement then 0 else 4 in
    let c4 = if check-linear-builders then 0 else 8 in
    c1 + c2 + c3 + c4
"#;
    build_and_run_leak_checked("r3_bytes_wyhash", &[("Main.cinch", main)], "8M");
}

#[test]
fn r3_text_scan_simd_and_linear_transforms() {
    let main = r#"module Main where
  import Data.Bool
  import Data.Char
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.String hiding (string-join)
  import Text.Scan

  same-str :: String -> String -> Bool
  same-str a b = string-compare a b == 0

  same-list :: List String -> List String -> Bool
  same-list a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if same-str x y then same-list xs ys else False

  opt-is :: Option Nat -> Nat -> Bool
  opt-is o n = case o of
    None -> False
    Some m -> m == n

  opt-none :: Option Nat -> Bool
  opt-none o = case o of
    None -> True
    Some _ -> False

  -- False-positive stress for Muła 16-byte first+last byte SIMD filter:
  -- many substrings share first ('a') and last ('z') bytes across 16-byte
  -- lane boundaries, with only one real match at the end.
  check-search-simd :: Bool
  check-search-simd =
    let decoy = "a00000000000000za11111111111111za22222222222222z" in
    let hay = string-concat (string-repeat 20 decoy) "a_MATCH_TARGET_z_tail" in
    -- 20 * 48 = 960 ASCII chars before match
    string-starts-with "" hay
    && string-starts-with "a00000000000000z" hay
    && not (string-starts-with "a00000000000000!" hay)
    && not (string-starts-with (string-concat hay "x") hay)
    && string-ends-with "" hay
    && string-ends-with "TARGET_z_tail" hay
    && not (string-ends-with "TARGET_x_tail" hay)
    && not (string-ends-with (string-concat "x" hay) hay)
    && string-contains "" hay
    && string-contains "a_MATCH_TARGET_z" hay
    && not (string-contains "a_NO_SUCH_MATCH_z" hay)
    && opt-is (string-index-of "" hay) 0
    && opt-is (string-index-of "a_MATCH_TARGET_z" hay) 960
    && opt-none (string-index-of "a_NO_SUCH_MATCH_z" hay)
    && opt-is (string-char-index-of 'M' hay) 962
    && opt-none (string-char-index-of 'Q' hay)
    -- Match straddling 16-byte SIMD lane boundaries (offsets 14, 15, 16, 17, 31)
    && opt-is (string-index-of "WXYZ" "0123456789abcdWXYZefgh") 14
    && opt-is (string-index-of "WXYZ" "0123456789abcdeWXYZfgh") 15
    && opt-is (string-index-of "WXYZ" "0123456789abcdefWXYZgh") 16
    && opt-is (string-index-of "WXYZ" "0123456789abcdefgWXYZh") 17
    && opt-is (string-index-of "WXYZ" "0123456789abcdef0123456789abcdeWXYZ") 31
    -- 1-byte and 2-byte needles
    && opt-is (string-index-of "!" (string-concat (string-repeat 65 "a") "!")) 65
    && opt-is (string-index-of "xy" (string-concat (string-repeat 65 "x") "y")) 64
    && opt-is (string-index-of "" "") 0
    && opt-none (string-index-of "a" "")

  -- Multi-byte UTF-8 scalar-index parity (not byte offset!)
  check-utf8-indices :: Bool
  check-utf8-indices =
    -- Each chunk: "café αβγ 😀 " = 11 scalars (18 UTF-8 bytes)
    let chunk = "caf\u{e9} \u{03b1}\u{03b2}\u{03b3} \u{1f600} " in
    let big = string-concat (string-repeat 100 chunk) "hit_\u{e9}_end" in
    -- 100 * 11 = 1100 scalars before "hit_é_end"
    opt-is (string-index-of "hit_\u{e9}_end" big) 1100
    && opt-is (string-char-index-of 'h' big) 1100
    && opt-is (string-char-index-of '\u{1f600}' chunk) 9
    && opt-is (string-index-of "\u{03b2}\u{03b3}" chunk) 6
    && same-str (string-take 4 chunk) "caf\u{e9}"
    && same-str (string-drop 5 chunk) "\u{03b1}\u{03b2}\u{03b3} \u{1f600} "

  -- Direct byte slicing: split-on, lines, words, strip, chomp, chop
  check-slicing :: Bool
  check-slicing =
    same-list (string-split-on "," "a,b,,c") (Cons "a" (Cons "b" (Cons "" (Cons "c" Nil))))
    && same-list (string-split-on "--" "x--y--") (Cons "x" (Cons "y" Nil))
    && same-list (string-lines "one\ntwo\n") (Cons "one" (Cons "two" Nil))
    && same-list (string-lines "") Nil
    && same-list (string-words "  alpha\t\nbeta  \r\ngamma ") (Cons "alpha" (Cons "beta" (Cons "gamma" Nil)))
    && same-str (string-strip " \t\n hello world \r\n ") "hello world"
    && same-str (string-lstrip "  \t hi ") "hi "
    && same-str (string-rstrip " hi \t ") " hi"
    && same-str (string-chomp "abc\r\n") "abc"
    && same-str (string-chomp "abc\n") "abc"
    && same-str (string-chomp "abc") "abc"
    && same-str (string-chop "caf\u{e9}") "caf"
    && same-str (string-chop "") ""

  -- `%1`-linear transforms: ASCII case, newline normalization, padding, 64KB
  check-linear-transforms :: Bool
  check-linear-transforms =
    let mixed = "a1\r\nb2\rc3\nd4\r\n\r" in
    let big-crlf = string-repeat 2000 "hello_WORLD_123\r\nline2\r" in
    let norm-big = string-normalize-newlines big-crlf in
    let up-big = string-upcase-ascii (string-repeat 2000 "abcXYZ09_é!") in
    same-str (string-upcase-ascii "Hello, Caf\u{e9} 123!") "HELLO, CAF\u{e9} 123!"
    && same-str (string-downcase-ascii "Hello, CAF\u{e9} 123!") "hello, caf\u{e9} 123!"
    && same-str (string-capitalize-ascii "hELLO world") "Hello world"
    && same-str (string-capitalize-ascii "") ""
    && same-str (string-normalize-newlines mixed) "a1\nb2\nc3\nd4\n\n"
    && same-str (string-normalize-newlines "") ""
    && not (string-contains "\r" norm-big)
    && string-byte-length norm-big == 44000
    && string-starts-with "ABCXYZ09_\u{e9}!" up-big
    && same-str (string-pad-left 6 "ab") "    ab"
    && same-str (string-pad-right 6 "ab") "ab    "
    && same-str (string-center 6 "ab") "  ab  "
    && same-str (string-pad-left 2 "abcd") "abcd"

  main :: Int
  main =
    let c1 = if check-search-simd then 0 else 1 in
    let c2 = if check-utf8-indices then 0 else 2 in
    let c3 = if check-slicing then 0 else 4 in
    let c4 = if check-linear-transforms then 0 else 8 in
    c1 + c2 + c3 + c4
"#;
    build_and_run_leak_checked("r3_text_scan", &[("Main.cinch", main)], "8M");
}

#[test]
fn r3_hashmap_unboxed_and_collision_semantics() {
    let main = r#"module Main where
  import Data.Bool
  import Data.Hash
  import Data.HashMap
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.String
  import Text.Show

  opt-eq :: Option Nat -> Nat -> Bool
  opt-eq o v = case o of
    None -> False
    Some x -> x == v

  opt-none :: Option Nat -> Bool
  opt-none o = case o of
    None -> True
    Some _ -> False

  -- 1. 20,000-entry O(1)-stack hash-map-from-pairs + earlier-pair-wins
  make-pairs :: Nat -> List (String, Nat) -> List (String, Nat)
  make-pairs n acc =
    if n == 0 then acc
    else
      let k = string-concat "k_" (nat-to-string n) in
      make-pairs (n - 1) (Cons (k, n) acc)

  check-large-from-pairs :: Bool
  check-large-from-pairs =
    let ps = Cons ("k_1", 999999) (Cons ("k_10000", 888888) (make-pairs 20000 (Cons ("k_20000", 777777) Nil))) in
    let m = hash-map-from-pairs ps in
    hash-map-size m == 20000
    && opt-eq (hash-map-lookup "k_1" m) 999999
    && opt-eq (hash-map-lookup "k_10000" m) 888888
    && opt-eq (hash-map-lookup "k_20000" m) 20000
    && opt-eq (hash-map-lookup "k_42" m) 42
    && opt-none (hash-map-lookup "k_0" m)
    && opt-none (hash-map-lookup "k_20001" m)

  -- 2. Insert, overwrite, lookup, delete-to-empty on 5,000 keys
  ins-range :: Nat -> HashMap String Nat -> HashMap String Nat
  ins-range n m =
    if n == 0 then m
    else ins-range (n - 1) (hash-map-insert (string-concat "s_" (nat-to-string n)) (n * 2) m)

  del-range :: Nat -> HashMap String Nat -> HashMap String Nat
  del-range n m =
    if n == 0 then m
    else del-range (n - 1) (hash-map-delete (string-concat "s_" (nat-to-string n)) m)

  check-fivek-lifecycle :: Bool
  check-fivek-lifecycle =
    let m0 = ins-range 5000 hash-map-empty in
    let m1 = hash-map-insert "s_2500" 777 m0 in
    let m2 = hash-map-delete "absent_key" m1 in
    let empty = del-range 5000 m2 in
    hash-map-size m0 == 5000
    && hash-map-size m1 == 5000
    && hash-map-size m2 == 5000
    && opt-eq (hash-map-lookup "s_2500" m0) 5000
    && opt-eq (hash-map-lookup "s_2500" m1) 777
    && hash-map-is-empty empty
    && hash-map-size empty == 0

  -- 3. Collision key type (constant digest 7): tests SBin1 -> SBinMany -> SBin1 -> STip
  data CKey where
    CKey :: Nat -> CKey

  ckey-n :: CKey -> Nat
  ckey-n k = case k of
    CKey n -> n

  implicit instance Eq_CKey :: Eq CKey where
    (==) a b = ckey-n a == ckey-n b
    (/=) a b = ckey-n a /= ckey-n b

  implicit instance Hash_CKey :: Hash CKey where
    hash _ = 7

  ins-coll :: Nat -> HashMap CKey Nat -> HashMap CKey Nat
  ins-coll n m =
    if n == 0 then m
    else ins-coll (n - 1) (hash-map-insert (CKey n) (n * 10) m)

  del-coll :: Nat -> HashMap CKey Nat -> HashMap CKey Nat
  del-coll n m =
    if n == 0 then m
    else del-coll (n - 1) (hash-map-delete (CKey n) m)

  check-collisions :: Bool
  check-collisions =
    let m1 = hash-map-insert (CKey 1) 10 hash-map-empty in
    let m2 = hash-map-insert (CKey 2) 20 m1 in
    let m3 = hash-map-insert (CKey 3) 30 m2 in
    let m3b = hash-map-insert (CKey 2) 222 m3 in
    let d2 = hash-map-delete (CKey 2) m3b in
    let d1 = hash-map-delete (CKey 1) d2 in
    let d0 = hash-map-delete (CKey 3) d1 in
    let big = ins-coll 64 hash-map-empty in
    let big-empty = del-coll 64 big in
    let fp = hash-map-from-pairs (Cons (CKey 1, 100) (Cons (CKey 2, 200) (Cons (CKey 1, 999) Nil))) in
    hash-map-size m3b == 3
    && opt-eq (hash-map-lookup (CKey 1) m3b) 10
    && opt-eq (hash-map-lookup (CKey 2) m3b) 222
    && opt-eq (hash-map-lookup (CKey 3) m3b) 30
    && hash-map-size d1 == 1
    && opt-eq (hash-map-lookup (CKey 3) d1) 30
    && opt-none (hash-map-lookup (CKey 1) d1)
    && opt-none (hash-map-lookup (CKey 2) d1)
    && hash-map-is-empty d0
    && hash-map-size big == 64
    && opt-eq (hash-map-lookup (CKey 37) big) 370
    && hash-map-is-empty big-empty
    && hash-map-size fp == 2
    && opt-eq (hash-map-lookup (CKey 1) fp) 100
    && opt-eq (hash-map-lookup (CKey 2) fp) 200

  -- 4. Bignum Hash (> 2^63 - 1) exercises digest-key modular fold
  data BigHashKey where
    BHK :: Nat -> BigHashKey

  bhk-n :: BigHashKey -> Nat
  bhk-n k = case k of
    BHK n -> n

  implicit instance Eq_BHK :: Eq BigHashKey where
    (==) a b = bhk-n a == bhk-n b
    (/=) a b = bhk-n a /= bhk-n b

  implicit instance Hash_BHK :: Hash BigHashKey where
    hash k = 100000000000000000000000000000 + bhk-n k

  check-bignum-hash :: Bool
  check-bignum-hash =
    let m = hash-map-insert (BHK 1) 11 (hash-map-insert (BHK 2) 22 (hash-map-insert (BHK 3) 33 hash-map-empty)) in
    hash-map-size m == 3
    && opt-eq (hash-map-lookup (BHK 1) m) 11
    && opt-eq (hash-map-lookup (BHK 2) m) 22
    && opt-eq (hash-map-lookup (BHK 3) m) 33
    && opt-none (hash-map-lookup (BHK 4) m)

  -- 5. Union, union-with, map, filter, fold on HashMap Nat Nat (uses pre-compiled @TT base variants)
  add-vals :: Nat -> Nat -> Nat
  add-vals a b = a + b

  keep-even :: Nat -> Nat -> Bool
  keep-even k _ = nat-rem k 2 == 0

  sum-fold :: Nat -> Nat -> Nat -> Nat
  sum-fold _ v acc = v + acc

  double-val :: Nat -> Nat
  double-val v = v * 2

  check-combinators :: Bool
  check-combinators =
    let a = hash-map-insert 1 10 (hash-map-insert 2 20 hash-map-empty) in
    let b = hash-map-insert 2 200 (hash-map-insert 3 300 hash-map-empty) in
    let u = hash-map-union a b in
    let uw = hash-map-union-with add-vals a b in
    let mm = hash-map-map double-val u in
    let fl = hash-map-filter keep-even uw in
    let total = hash-map-fold sum-fold 0 uw in
    hash-map-size u == 3
    && opt-eq (hash-map-lookup 1 u) 10
    && opt-eq (hash-map-lookup 2 u) 20
    && opt-eq (hash-map-lookup 3 u) 300
    && opt-eq (hash-map-lookup 2 uw) 220
    && opt-eq (hash-map-lookup 2 mm) 40
    && hash-map-size fl == 1
    && opt-eq (hash-map-lookup 2 fl) 220
    && total == 530
    && list-length (hash-map-keys u) == 3
    && list-length (hash-map-values u) == 3
    && list-length (hash-map-to-list u) == 3

  main :: Int
  main =
    let c1 = if check-large-from-pairs then 0 else 1 in
    let c2 = if check-fivek-lifecycle then 0 else 2 in
    let c3 = if check-collisions then 0 else 4 in
    let c4 = if check-bignum-hash then 0 else 8 in
    let c5 = if check-combinators then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#;
    build_and_run_leak_checked("r3_hashmap", &[("Main.cinch", main)], "8M");
}

#[test]
fn r3_coretypes_linear_type_safety_static_checks() {
    // 1. Duplicating a `%1`-linear Bytes buffer must be rejected statically.
    let dup_dir = common::project(
        "r3_lin_dup",
        &[(
            "Main.cinch",
            r#"module Main where
  import Data.Bytes
  import Data.Machine

  put-u8-raw :: Bytes -> I64 -> U8 -> Bytes
  put-u8-raw b pos v =
    if 0 <= pos && pos < bytes-len-i64 b then
      let w = bytes-set-u8-at b pos v in b
    else b

  bad-dup :: Bytes %1 -> Bytes
  bad-dup b =
    let b1 = put-u8-raw b 0 65 in
    put-u8-raw b 1 66

  main :: Int
  main = 0
"#,
        )],
    );
    let dup_err = common::drive(Mode::Check, &dup_dir, "Main.cinch", None)
        .expect_err("duplicating a %1 Bytes buffer must fail static linearity checking");
    assert!(
        dup_err.contains("multiplicity %1") && dup_err.contains("used 2 time(s)"),
        "unexpected linearity error: {dup_err}"
    );
    common::cleanup(&dup_dir);

    // 2. Dropping a `%1`-linear Bytes buffer on a branch must be rejected statically.
    let drop_dir = common::project(
        "r3_lin_drop",
        &[(
            "Main.cinch",
            r#"module Main where
  import Data.Bytes
  import Data.Option

  bad-drop :: Bool -> Bytes %1 -> Option Bytes
  bad-drop keep b = if keep then Some b else None

  main :: Int
  main = 0
"#,
        )],
    );
    let drop_err = common::drive(Mode::Check, &drop_dir, "Main.cinch", None)
        .expect_err("dropping a %1 Bytes buffer on an alternate branch must fail static linearity checking");
    assert!(
        drop_err.contains("multiplicity %1")
            && (drop_err.contains("unused on some path")
                || drop_err.contains("used 0 to 1 time(s)")),
        "unexpected linearity error: {drop_err}"
    );
    common::cleanup(&drop_dir);
}
