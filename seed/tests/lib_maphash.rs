//! Keyed 64-bit streaming and comparable hashing (`Data.MapHash` and `Effect.MapHash`) verified
//! against Go 1.24 `hash/maphash` contract fixtures (`tooling/maphash_vectors.go`): determinism,
//! cross-seed independence, exhaustive streaming split invariance across 0..=257 bytes,
//! avalanche and collision resistance, all 21 `MapHashable` instances (including `-0.0 == +0.0`
//! normalization and prefix-free composite framing), OS-entropy seeding via `Effect.MapHash`,
//! zero inline LLVM, and canonical documentation.
mod common;

use serde_json::Value;

/// Shared source modules needed alongside the embedded prelude.
fn maphash_sources() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
        ("Data.MapHash.cinch", include_str!("../prelude/Data/MapHash.cinch")),
        ("Effect.MapHash.cinch", include_str!("../prelude/Effect/MapHash.cinch")),
        ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
    ]
}

/// Escape a UTF-8 string as a Cinch string literal using only supported escapes.
fn cinch_string_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\0' => out.push_str("\\0"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Verify all Go 1.24 `hash/maphash` contract vectors, seed operations, and `MapHashable` instances.
#[test]
fn maphash_matches_reference_contracts() {
    let f: Value = serde_json::from_str(include_str!("fixtures/maphash-vectors.json")).unwrap();
    assert!(
        f["go_checks_verified"].as_u64().unwrap() >= 5000,
        "fixture must come from verified Go 1.24 contract run"
    );

    let mut rows = Vec::new();

    // Constants.
    let want_size = f["constants"]["size"].as_u64().unwrap();
    let want_block = f["constants"]["block_size"].as_u64().unwrap();
    rows.push(format!("maphash-size == {want_size} && maphash-block-size == {want_block}"));

    // Seed round-trip and Eq checks.
    let u64_seeds: Vec<u64> = f["u64_seeds"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
    for s in &u64_seeds {
        rows.push(format!("check-u64-seed {s}"));
    }
    for pair in u64_seeds.windows(2) {
        rows.push(format!("maphash-seed-from-u64 {} /= maphash-seed-from-u64 {}", pair[0], pair[1]));
    }
    let u128_seeds: Vec<(u64, u64)> = f["u128_seeds"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| (v["hi"].as_u64().unwrap(), v["lo"].as_u64().unwrap()))
        .collect();
    for (hi, lo) in &u128_seeds {
        rows.push(format!("check-u128-seed {hi} {lo}"));
    }
    for pair in u128_seeds.windows(2) {
        rows.push(format!(
            "mk-seed128 {} {} /= mk-seed128 {} {}",
            pair[0].0, pair[0].1, pair[1].0, pair[1].1
        ));
    }
    rows.push("check-seed-bytes-refusals ()".to_string());

    // Byte contract vectors (lengths 0..=1024).
    for h in f["byte_cases_hex"].as_array().unwrap() {
        let hex_str = h.as_str().unwrap();
        rows.push(format!("check-byte-case {:?}", hex_str));
    }

    // Distinctness groups (0 collisions).
    for group_key in ["all_zeros_distinct", "trailing_zeros_group", "single_bit_flips"] {
        let items = f[group_key]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| format!("{:?}", v.as_str().unwrap()))
            .collect::<Vec<_>>()
            .join(", ");
        rows.push(format!("check-distinct-hex [{items}]"));
    }

    // String contract vectors (ASCII, UTF-8, NUL, multi-block).
    for s in f["string_cases"].as_array().unwrap() {
        let raw = s.as_str().unwrap();
        if raw.contains('\0') {
            // Represent strings with embedded NUL via hex -> string-from-bytes to avoid string literal escaping ambiguities.
            let hex_bytes = raw.as_bytes().iter().map(|b| format!("{b:02x}")).collect::<String>();
            rows.push(format!("check-string-hex {:?}", hex_bytes));
        } else {
            rows.push(format!("check-string-case {}", cinch_string_literal(raw)));
        }
    }

    // Multi-chunk split cases.
    for sc in f["split_cases"].as_array().unwrap() {
        let full_hex = sc["input_hex"].as_str().unwrap();
        let chunks = sc["chunks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| format!("{:?}", c.as_str().unwrap()))
            .collect::<Vec<_>>()
            .join(", ");
        rows.push(format!("check-split-case {:?} [{chunks}]", full_hex));
    }

    // Comparable vectors from Go 1.24 fixture.
    let cmp = &f["comparable"];
    let f64_zeros = cmp["f64_zero_bits"].as_array().unwrap();
    rows.push(format!(
        "check-f64-equal-bits {} {}",
        f64_zeros[0].as_u64().unwrap(),
        f64_zeros[1].as_u64().unwrap()
    ));
    let f32_zeros = cmp["f32_zero_bits"].as_array().unwrap();
    rows.push(format!(
        "check-f32-equal-bits {} {}",
        f32_zeros[0].as_u64().unwrap(),
        f32_zeros[1].as_u64().unwrap()
    ));
    let f64_distinct = cmp["f64_distinct_bits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    rows.push(format!("check-f64-distinct [{f64_distinct}]"));

    let f32_distinct = cmp["f32_distinct_bits"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    rows.push(format!("check-f32-distinct [{f32_distinct}]"));

    let i64_distinct = cmp["i64_distinct"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let n = v.as_i64().unwrap();
            if n == i64::MIN {
                "min-bound-i64".to_string()
            } else if n < 0 {
                format!("(wrapping-neg-i64 {})", n.unsigned_abs())
            } else {
                n.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    rows.push(format!("check-i64-distinct [{i64_distinct}]"));

    let u64_distinct = cmp["u64_distinct"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    rows.push(format!("check-u64-distinct [{u64_distinct}]"));

    let pairs = cmp["string_pair_distinct"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            format!(
                "({}, {})",
                cinch_string_literal(p["first"].as_str().unwrap()),
                cinch_string_literal(p["second"].as_str().unwrap())
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    rows.push(format!("check-string-pairs-distinct [{pairs}]"));

    // All remaining MapHashable instances + persistent snapshot cloning + non-commuting WriteComparable + unaligned scalar writes.
    rows.push("check-all-comparable-instances ()".to_string());
    rows.push("check-persistent-branching ()".to_string());
    rows.push("check-write-comparable-non-commuting ()".to_string());
    rows.push("check-scalar-alignments 0".to_string());

    let main_src = format!(
        "{CONTRACT_HELPERS}\n  -- | Run all contract checks; 200 means every row passed.\n  main :: Int\n  main = first [\n    {}\n  ] 0\n",
        rows.join(",\n    ")
    );

    let mut files = maphash_sources();
    files.push(("Main.cinch", &main_src));
    let dir = common::project("maphash_contracts", &files);
    assert_eq!(
        common::run(&dir, "Main.cinch"),
        200,
        "exit status is the first failing contract row; 200 means all passed"
    );
    common::cleanup(&dir);
}

/// Exhaustively verify streaming split invariance across ALL lengths 0..=257 and EVERY 2-way split,
/// byte-by-byte writes, 3-way mixed writes, and 1024-byte one-shot vs chunked writes.
#[test]
fn maphash_exhaustive_streaming_splits_0_to_257() {
    let main_src = r#"-- | Exhaustive streaming split invariance across 0..=257 bytes and 1024 bytes.
module Main where
  import Data.Bytes
  import Data.Machine
  import Data.MapHash
  import Data.Nat
  import Data.Wide

  -- | Fill a deterministic byte buffer of length n.
  fill-buf :: Bytes -> Nat -> Unit
  fill-buf b i =
    if i < bytes-len b then
      let v = nat-rem (i * 131 + nat-div i 7 * 17 + 29) 256 in
      let u = bytes-set b i v in
      fill-buf b (i + 1)
    else ()

  -- | Create a deterministic n-byte buffer.
  make-buf :: Nat -> Bytes
  make-buf n = let b = bytes-new n in let u = fill-buf b 0 in b

  -- | Slice [start, start + len) from b.
  slice-buf :: Bytes -> Nat -> Nat -> Bytes
  slice-buf b start len =
    let out = bytes-new len in
    if start + len <= bytes-len b && len <= bytes-len out then
      let u = bytes-copy-range out b 0 start len in out
    else out

  -- | Write bytes one-by-one via maphash-write-byte.
  write-all-bytes :: MapHash -> Bytes -> Nat -> MapHash
  write-all-bytes h b i =
    if i < bytes-len b then
      let byte = wrapping-i64-to-u8 (nat-to-i64 (bytes-get b i)) in
      write-all-bytes (maphash-write-byte h byte) b (i + 1)
    else h

  -- | Check every 2-way split k in 0..=n matches want.
  check-all-splits :: MapSeed -> Bytes -> U64 -> Nat -> Bool
  check-all-splits seed b want k =
    let n = bytes-len b in
    if k > n then True
    else
      let left = slice-buf b 0 k in
      let right = slice-buf b k (n - k) in
      let h = maphash-write (maphash-write (maphash-new seed) left) right in
      if eq-u64 (maphash-sum64 h) want then check-all-splits seed b want (k + 1)
      else False

  -- | Check 3-way mixed split (write + write-byte + write) for n >= 1.
  check-three-way :: MapSeed -> Bytes -> U64 -> Bool
  check-three-way seed b want =
    let n = bytes-len b in
    if n == 0 then True
    else
      let mid = nat-div n 2 in
      if mid < n then
        let left = slice-buf b 0 mid in
        let mid-byte = wrapping-i64-to-u8 (nat-to-i64 (bytes-get b mid)) in
        let right = slice-buf b (mid + 1) (n - (mid + 1)) in
        let h = maphash-write (maphash-write-byte (maphash-write (maphash-new seed) left) mid-byte) right in
        eq-u64 (maphash-sum64 h) want
      else True

  -- | Verify every length from n up to limit.
  check-lengths :: MapSeed -> Nat -> Nat -> Int
  check-lengths seed n limit =
    if n > limit then 200
    else
      let b = make-buf n in
      let want = maphash-bytes seed b in
      let whole = maphash-sum64 (maphash-write (maphash-new seed) b) in
      let by-byte = maphash-sum64 (write-all-bytes (maphash-new seed) b 0) in
      if not (eq-u64 want whole) then 1
      else if not (eq-u64 want by-byte) then 2
      else if not (check-all-splits seed b want 0) then 3
      else if not (check-three-way seed b want) then 4
      else check-lengths seed (n + 1) limit

  -- | Verify 1024-byte one-shot vs 7-byte chunked streaming.
  chunk-write :: MapHash -> Bytes -> Nat -> Nat -> MapHash
  chunk-write h b pos step =
    let n = bytes-len b in
    if pos >= n then h
    else
      let rem = n - pos in
      let take = (if step < rem then step else rem) in
      let part = slice-buf b pos take in
      chunk-write (maphash-write h part) b (pos + take) step

  -- | Entry point: 200 on full pass.
  main :: Int
  main =
    let seed = maphash-seed-from-u128 (bit-or-u128 (wrapping-shl-u128 (u64-to-u128 0x1122334455667788) 64) (u64-to-u128 0x99aabbccddeeff01)) in
    let res = check-lengths seed 0 257 in
    if res /= 200 then res
    else
      let big = make-buf 1024 in
      let want = maphash-bytes seed big in
      let c7 = maphash-sum64 (chunk-write (maphash-new seed) big 0 7) in
      let c13 = maphash-sum64 (chunk-write (maphash-new seed) big 0 13) in
      let c128 = maphash-sum64 (chunk-write (maphash-new seed) big 0 128) in
      let c129 = maphash-sum64 (chunk-write (maphash-new seed) big 0 129) in
      if eq-u64 want c7 && eq-u64 want c13 && eq-u64 want c128 && eq-u64 want c129 then 200 else 5
"#;

    let mut files = maphash_sources();
    files.push(("Main.cinch", main_src));
    let dir = common::project("maphash_splits", &files);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "all 0..=257 exhaustive splits and 1024-byte chunking must pass");
    common::cleanup(&dir);
}

/// Verify strong bit avalanche across input and seed bits, plus collision resistance over 2,000+ keys.
#[test]
fn maphash_avalanche_and_collision_resistance() {
    let main_src = r#"-- | Avalanche and collision-resistance checks for Data.MapHash.
module Main where
  import Data.Bytes
  import Data.Bytes.Word
  import Data.List
  import Data.Machine
  import Data.MapHash
  import Data.Nat
  import Data.Wide

  -- | Sum of popcount(base ^ hash(x ^ (1 << bit))) across all 64 bits of a U64 key.
  u64-avalanche-sum :: MapSeed -> U64 -> U64 -> U32 -> Nat -> Nat
  u64-avalanche-sum seed base-val base-hash bit acc =
    if ge-u32 bit 64 then acc
    else
      let flipped = bit-xor-u64 base-val (wrapping-shl-u64 1 bit) in
      let h = maphash-comparable seed flipped in
      let diff = u32-to-nat (count-ones-u64 (bit-xor-u64 base-hash h)) in
      if diff < 12 || diff > 52 then 999999
      else u64-avalanche-sum seed base-val base-hash (wrapping-add-u32 bit 1) (acc + diff)

  -- | Sum of popcount across all 128 seed bit flips on a fixed input.
  seed-avalanche-sum :: U128 -> Bytes -> U64 -> U32 -> Nat -> Nat
  seed-avalanche-sum base-key msg base-hash bit acc =
    if ge-u32 bit 128 then acc
    else
      let flipped = bit-xor-u128 base-key (wrapping-shl-u128 1 bit) in
      let h = maphash-bytes (maphash-seed-from-u128 flipped) msg in
      let diff = u32-to-nat (count-ones-u64 (bit-xor-u64 base-hash h)) in
      if diff < 12 || diff > 52 then 999999
      else seed-avalanche-sum base-key msg base-hash (wrapping-add-u32 bit 1) (acc + diff)

  -- | Check whether a U64 digest already appears in a sorted/unsorted list? Sort then check adjacent!
  -- Collect N digests of sequential U64 keys.
  collect-seq :: MapSeed -> U64 -> U64 -> List U64 -> List U64
  collect-seq seed i limit acc =
    if ge-u64 i limit then acc
    else collect-seq seed (wrapping-add-u64 i 1) limit (Cons (maphash-comparable seed i) acc)

  -- | Collect digests of 256 all-zero buffers of lengths 0..255.
  collect-zeros :: MapSeed -> Nat -> Nat -> List U64 -> List U64
  collect-zeros seed i limit acc =
    if i >= limit then acc
    else collect-zeros seed (i + 1) limit (Cons (maphash-bytes seed (bytes-new i)) acc)

  -- | Collect digests of 128 single-bit-set 16-byte buffers.
  collect-one-hot :: MapSeed -> Nat -> List U64 -> List U64
  collect-one-hot seed bit acc =
    if bit >= 128 then acc
    else
      let b = bytes-new 16 in
      let byte-idx = nat-div bit 8 in
      let bit-idx = nat-rem bit 8 in
      let mask = wrapping-shl-u8 1 (wrapping-i64-to-u32 (nat-to-i64 bit-idx)) in
      let u = (if byte-idx < bytes-len b then bytes-set b byte-idx (u8-to-nat mask) else ()) in
      collect-one-hot seed (bit + 1) (Cons (maphash-bytes seed b) acc)

  -- | Verify a sorted list of U64s has strictly increasing elements (no duplicates).
  all-strictly-increasing :: List U64 -> Bool
  all-strictly-increasing xs = case xs of
    Nil -> True
    Cons x rest -> case rest of
      Nil -> True
      Cons y more -> if lt-u64 x y then all-strictly-increasing rest else False

  -- | Check no duplicates in a list of U64 digests via Data.List.sort.
  no-collisions :: List U64 -> Bool
  no-collisions xs = all-strictly-increasing (sort xs)

  -- | Entry point: 200 on pass.
  main :: Int
  main =
    let seed-key = bit-or-u128 (wrapping-shl-u128 (u64-to-u128 0x55aa33cc11ee7799) 64) (u64-to-u128 0x0123456789abcdef) in
    let seed = maphash-seed-from-u128 seed-key in
    -- Input avalanche on 4 distinct base U64s (256 bit flips total -> expect ~8192 total flipped bits, allow 7400..9000).
    let a0 = u64-avalanche-sum seed 0 (maphash-comparable seed (0 :: U64)) 0 0 in
    let a1 = u64-avalanche-sum seed 0xdeadbeefcafebabe (maphash-comparable seed (0xdeadbeefcafebabe :: U64)) 0 0 in
    let a2 = u64-avalanche-sum seed 0xffffffffffffffff (maphash-comparable seed (0xffffffffffffffff :: U64)) 0 0 in
    let a3 = u64-avalanche-sum seed 0x0102030405060708 (maphash-comparable seed (0x0102030405060708 :: U64)) 0 0 in
    let in-total = a0 + a1 + a2 + a3 in
    -- Seed avalanche across all 128 seed bits on a 32-byte message (expect ~4096 total flipped bits, allow 3600..4600).
    let msg = bytes-new 32 in
    let seed-total = seed-avalanche-sum seed-key msg (maphash-bytes seed msg) 0 0 in
    if in-total < 7400 || in-total > 9000 then 1
    else if seed-total < 3600 || seed-total > 4600 then 2
    else if not (no-collisions (collect-seq seed 0 2000 Nil)) then 3
    else if not (no-collisions (collect-zeros seed 0 256 Nil)) then 4
    else if not (no-collisions (collect-one-hot seed 0 Nil)) then 5
    else 200
"#;

    let mut files = maphash_sources();
    files.push(("Main.cinch", main_src));
    let dir = common::project("maphash_avalanche", &files);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "avalanche and collision resistance must pass");
    common::cleanup(&dir);
}

/// Verify `Effect.MapHash.maphash-make-seed` draws distinct 128-bit OS-entropy seeds that yield distinct hashes.
#[test]
fn maphash_effect_make_seed_distinct() {
    let main_src = r#"-- | OS-entropy seed independence test for Effect.MapHash.
module Main where
  import Data.Bytes
  import Data.Machine
  import Data.MapHash
  import Data.Option
  import Effect.MapHash
  import Relation.Eq

  -- | Draw 4 OS-entropy seeds and check pairwise distinctness, 16-byte round-trip, and distinct digests.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let s1 = maphash-make-seed () in
    let s2 = maphash-make-seed () in
    let s3 = maphash-make-seed () in
    let s4 = maphash-make-seed () in
    let b1 = maphash-seed-bytes s1 in
    let rt1 = (case maphash-seed-from-bytes b1 of Some x -> x == s1; None -> False) in
    let d1 = maphash-string s1 "cinch maphash entropy check" in
    let d2 = maphash-string s2 "cinch maphash entropy check" in
    let d3 = maphash-string s3 "cinch maphash entropy check" in
    let d4 = maphash-string s4 "cinch maphash entropy check" in
    if s1 == s2 || s1 == s3 || s1 == s4 || s2 == s3 || s2 == s4 || s3 == s4 then 1
    else if not (bytes-len b1 == 16 && rt1) then 2
    else if eq-u64 d1 d2 || eq-u64 d1 d3 || eq-u64 d1 d4 || eq-u64 d2 d3 || eq-u64 d2 d4 || eq-u64 d3 d4 then 3
    else 200
"#;

    let mut files = maphash_sources();
    files.push(("Main.cinch", main_src));
    let dir = common::project("maphash_effect", &files);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "OS-entropy seeds must be distinct");
    common::cleanup(&dir);
}

/// Every declaration in `Data.MapHash` and `Effect.MapHash` has canonical documentation and ZERO inline LLVM.
#[test]
fn maphash_declarations_have_canonical_documentation() {
    let data_src = include_str!("../prelude/Data/MapHash.cinch");
    let effect_src = include_str!("../prelude/Effect/MapHash.cinch");
    common::assert_documented("Data.MapHash", data_src);
    common::assert_documented("Effect.MapHash", effect_src);
    assert!(!data_src.contains("llvm {"), "Data.MapHash must contain ZERO inline LLVM");
    assert!(!effect_src.contains("llvm {"), "Effect.MapHash must contain ZERO inline LLVM");
    assert!(!data_src.contains("prim "), "Data.MapHash must contain ZERO prim declarations");
    assert!(!effect_src.contains("prim "), "Effect.MapHash must contain ZERO prim declarations");
}

/// Cinch helper module used by `maphash_matches_reference_contracts`.
const CONTRACT_HELPERS: &str = r#"-- | Contract verification helpers for Data.MapHash.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Word
  import Data.Char
  import Data.Float (f32-from-bits, f64-from-bits)
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.MapHash
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Data.Wide
  import Relation.Eq
  import Text.Hex

  -- | Decode hex into Bytes.
  hb :: String -> Bytes
  hb s = case hex-decode s of Ok b -> b; Err e -> bytes-new 0

  -- | Construct a 128-bit MapSeed from high and low 64-bit words.
  mk-seed128 :: U64 -> U64 -> MapSeed
  mk-seed128 hi lo = maphash-seed-from-u128 (bit-or-u128 (wrapping-shl-u128 (u64-to-u128 hi) 64) (u64-to-u128 lo))

  -- | Primary test seed.
  seed-a :: MapSeed
  seed-a = mk-seed128 0x0123456789abcdef 0xfedcba9876543210

  -- | Secondary test seed (differs by 1 bit from seed-a).
  seed-b :: MapSeed
  seed-b = mk-seed128 0x0123456789abcdef 0xfedcba9876543211

  -- | Tertiary test seed (differs in high word by 1 bit from seed-a).
  seed-c :: MapSeed
  seed-c = mk-seed128 0x0123456789abcdee 0xfedcba9876543210

  -- | Check U64 seed round-trip and Eq.
  check-u64-seed :: U64 -> Bool
  check-u64-seed x =
    let s = maphash-seed-from-u64 x in
    let b = maphash-seed-bytes s in
    let rt = (case maphash-seed-from-bytes b of Some s2 -> s == s2; None -> False) in
    bytes-len b == 16 && s == maphash-seed-from-u64 x && maphash-seed (maphash-new s) == s && rt

  -- | Check U128 seed round-trip and Eq.
  check-u128-seed :: U64 -> U64 -> Bool
  check-u128-seed hi lo =
    let s = mk-seed128 hi lo in
    let b = maphash-seed-bytes s in
    let rt = (case maphash-seed-from-bytes b of Some s2 -> s == s2; None -> False) in
    let hi-read = (if 8 <= bytes-len b then swap-bytes-u64 (bytes-read-le64 b 0) else 0) in
    let lo-read = (if 16 <= bytes-len b then swap-bytes-u64 (bytes-read-le64 b 8) else 0) in
    bytes-len b == 16 && eq-u64 hi-read hi && eq-u64 lo-read lo && rt && maphash-seed (maphash-new s) == s

  -- | Check that maphash-seed-from-bytes rejects non-16-byte inputs.
  check-seed-bytes-refusals :: Unit -> Bool
  check-seed-bytes-refusals u =
    is-none (maphash-seed-from-bytes (bytes-new 0))
      && is-none (maphash-seed-from-bytes (bytes-new 8))
      && is-none (maphash-seed-from-bytes (bytes-new 15))
      && is-none (maphash-seed-from-bytes (bytes-new 17))
      && is-none (maphash-seed-from-bytes (bytes-new 32))

  -- | Write bytes one-by-one into a MapHash.
  write-each-byte :: MapHash -> Bytes -> Nat -> MapHash
  write-each-byte h b i =
    if i < bytes-len b then
      let byte = wrapping-i64-to-u8 (nat-to-i64 (bytes-get b i)) in
      write-each-byte (maphash-write-byte h byte) b (i + 1)
    else h

  -- | Check big-endian 8-byte digest appended by maphash-sum.
  check-sum-append :: MapHash -> U64 -> Bool
  check-sum-append h want =
    let prefix = hb "deadbeef" in
    let out = maphash-sum h prefix in
    let p-ok = (bytes-len out == 12 && bytes-get out 0 == 0xde && bytes-get out 1 == 0xad && bytes-get out 2 == 0xbe && bytes-get out 3 == 0xef) in
    let d-read = (if 12 <= bytes-len out then swap-bytes-u64 (bytes-read-le64 out 4) else 0) in
    let prefix-intact = (bytes-len prefix == 4 && bytes-get prefix 0 == 0xde && bytes-get prefix 3 == 0xef) in
    p-ok && prefix-intact && eq-u64 d-read want

  -- | Verify every byte contract on a hex-encoded payload.
  check-byte-case :: String -> Bool
  check-byte-case hex-str =
    let b = hb hex-str in
    let one = maphash-bytes seed-a b in
    let two = maphash-bytes seed-a b in
    let h1 = maphash-write (maphash-new seed-a) b in
    let s1 = maphash-sum64 h1 in
    let s1-again = maphash-sum64 h1 in
    let h-reset = maphash-reset h1 in
    let h-reseed = maphash-set-seed h1 seed-b in
    let one-b = maphash-bytes seed-b b in
    let one-c = maphash-bytes seed-c b in
    let by-byte-ok = (if bytes-len b <= 260 then eq-u64 (maphash-sum64 (write-each-byte (maphash-new seed-a) b 0)) one else True) in
    eq-u64 one two
      && eq-u64 s1 one
      && eq-u64 s1-again one
      && check-sum-append h1 one
      && maphash-seed h-reset == seed-a
      && eq-u64 (maphash-sum64 h-reset) (maphash-bytes seed-a (bytes-new 0))
      && maphash-seed h-reseed == seed-b
      && eq-u64 (maphash-sum64 h-reseed) (maphash-bytes seed-b (bytes-new 0))
      && eq-u64 (maphash-sum64 (maphash-write h-reseed b)) one-b
      && ne-u64 one one-b
      && ne-u64 one one-c
      && ne-u64 one-b one-c
      && by-byte-ok

  -- | Verify string contracts for a String value.
  check-string-case :: String -> Bool
  check-string-case s =
    let d1 = maphash-string seed-a s in
    let db = maphash-bytes seed-a (string-to-bytes s) in
    let ds = maphash-sum64 (maphash-write-string (maphash-new seed-a) s) in
    let d2 = maphash-string seed-b s in
    eq-u64 d1 db && eq-u64 d1 ds && ne-u64 d1 d2

  -- | Verify string contracts for a hex-encoded UTF-8 string (for embedded NULs).
  check-string-hex :: String -> Bool
  check-string-hex hex-str = case string-from-bytes (hb hex-str) of
    Some s -> check-string-case s
    None -> False

  -- | Write a list of hex chunks sequentially into a MapHash.
  write-hex-chunks :: MapHash -> List String -> MapHash
  write-hex-chunks h xs = case xs of
    Nil -> h
    Cons c rest -> write-hex-chunks (maphash-write h (hb c)) rest

  -- | Verify a multi-chunk split case matches one-shot maphash-bytes.
  check-split-case :: String -> List String -> Bool
  check-split-case full-hex chunks =
    let want = maphash-bytes seed-a (hb full-hex) in
    let got = maphash-sum64 (write-hex-chunks (maphash-new seed-a) chunks) in
    eq-u64 got want

  -- | Verify a sorted list of U64s has no duplicates.
  strictly-increasing :: List U64 -> Bool
  strictly-increasing xs = case xs of
    Nil -> True
    Cons x rest -> case rest of
      Nil -> True
      Cons y more -> if lt-u64 x y then strictly-increasing rest else False

  -- | Check all U64s in xs are distinct.
  all-distinct :: List U64 -> Bool
  all-distinct xs = strictly-increasing (sort xs)

  -- | Hash a list of hex payloads and assert all digests are distinct.
  check-distinct-hex :: List String -> Bool
  check-distinct-hex xs = all-distinct (map (\h -> maphash-bytes seed-a (hb h)) xs)

  -- | Check two F64 bit patterns hash identically under maphash-comparable.
  check-f64-equal-bits :: U64 -> U64 -> Bool
  check-f64-equal-bits a b =
    eq-u64 (maphash-comparable seed-a (f64-from-bits a)) (maphash-comparable seed-a (f64-from-bits b))

  -- | Check two F32 bit patterns hash identically under maphash-comparable.
  check-f32-equal-bits :: U32 -> U32 -> Bool
  check-f32-equal-bits a b =
    eq-u64 (maphash-comparable seed-a (f32-from-bits a)) (maphash-comparable seed-a (f32-from-bits b))

  -- | Check all F64 bit patterns produce distinct hashes and match WriteComparable.
  check-f64-distinct :: List U64 -> Bool
  check-f64-distinct xs =
    let hashes = map (\bits -> maphash-comparable seed-a (f64-from-bits bits)) xs in
    let write-ok = foldr (\bits acc -> acc && eq-u64 (maphash-comparable seed-a (f64-from-bits bits)) (maphash-sum64 (maphash-write-comparable (maphash-new seed-a) (f64-from-bits bits)))) True xs in
    all-distinct hashes && write-ok

  -- | Check all F32 bit patterns produce distinct hashes and match WriteComparable.
  check-f32-distinct :: List U32 -> Bool
  check-f32-distinct xs =
    let hashes = map (\bits -> maphash-comparable seed-a (f32-from-bits bits)) xs in
    let write-ok = foldr (\bits acc -> acc && eq-u64 (maphash-comparable seed-a (f32-from-bits bits)) (maphash-sum64 (maphash-write-comparable (maphash-new seed-a) (f32-from-bits bits)))) True xs in
    all-distinct hashes && write-ok

  -- | Check all I64 values produce distinct hashes.
  check-i64-distinct :: List I64 -> Bool
  check-i64-distinct xs = all-distinct (map (\v -> maphash-comparable seed-a v) xs)

  -- | Check all U64 values produce distinct hashes.
  check-u64-distinct :: List U64 -> Bool
  check-u64-distinct xs = all-distinct (map (\v -> maphash-comparable seed-a v) xs)

  -- | Check string pairs avoid boundary collisions.
  check-string-pairs-distinct :: List (String, String) -> Bool
  check-string-pairs-distinct xs =
    let hashes = map (\p -> maphash-comparable seed-a p) xs in
    let write-ok = foldr (\p acc -> acc && eq-u64 (maphash-comparable seed-a p) (maphash-sum64 (maphash-write-comparable (maphash-new seed-a) p))) True xs in
    all-distinct hashes && write-ok

  -- | Exercise every MapHashable instance (all 21 types), including bignum Nat/Int, U128/I128, NaN, Option, List, Pair.
  check-all-comparable-instances :: Unit -> Bool
  check-all-comparable-instances u =
    let bool-ok = ne-u64 (maphash-comparable seed-a False) (maphash-comparable seed-a True) in
    let u8-ok = all-distinct [maphash-comparable seed-a (0 :: U8), maphash-comparable seed-a (1 :: U8), maphash-comparable seed-a (255 :: U8)] in
    let u16-ok = all-distinct [maphash-comparable seed-a (0 :: U16), maphash-comparable seed-a (1 :: U16), maphash-comparable seed-a (65535 :: U16)] in
    let u32-ok = all-distinct [maphash-comparable seed-a (0 :: U32), maphash-comparable seed-a (1 :: U32), maphash-comparable seed-a (4294967295 :: U32)] in
    let i8-ok = all-distinct [maphash-comparable seed-a (min-bound-i8), maphash-comparable seed-a (wrapping-neg-i8 1), maphash-comparable seed-a (0 :: I8), maphash-comparable seed-a (max-bound-i8)] in
    let i16-ok = all-distinct [maphash-comparable seed-a (min-bound-i16), maphash-comparable seed-a (wrapping-neg-i16 1), maphash-comparable seed-a (0 :: I16), maphash-comparable seed-a (max-bound-i16)] in
    let i32-ok = all-distinct [maphash-comparable seed-a (min-bound-i32), maphash-comparable seed-a (wrapping-neg-i32 1), maphash-comparable seed-a (0 :: I32), maphash-comparable seed-a (max-bound-i32)] in
    let u128-ok = all-distinct [maphash-comparable seed-a (0 :: U128), maphash-comparable seed-a (1 :: U128), maphash-comparable seed-a (wrapping-shl-u128 1 64), maphash-comparable seed-a max-bound-u128] in
    let i128-ok = all-distinct [maphash-comparable seed-a min-bound-i128, maphash-comparable seed-a (wrapping-neg-i128 1), maphash-comparable seed-a (0 :: I128), maphash-comparable seed-a max-bound-i128] in
    let nan64-a = f64-from-bits 0x7ff8000000000001 in
    let nan64-b = f64-from-bits 0xfff8000000000002 in
    let nan32-a = f32-from-bits 0x7fc00001 in
    let nan32-b = f32-from-bits 0xffc00002 in
    let nan-ok = eq-u64 (maphash-comparable seed-a nan64-a) (maphash-comparable seed-a nan64-b) && eq-u64 (maphash-comparable seed-a nan32-a) (maphash-comparable seed-a nan32-b) in
    let char-ok = (case (char-from-code 65, char-from-code 66, char-from-code 129408) of
      (Some ca, Some cb, Some cc) -> all-distinct [maphash-comparable seed-a ca, maphash-comparable seed-a cb, maphash-comparable seed-a cc]
      other -> False) in
    let big-nat = (340282366920938463463374607431768211456 :: Nat) in
    let nat-ok = all-distinct [
      maphash-comparable seed-a (0 :: Nat),
      maphash-comparable seed-a (1 :: Nat),
      maphash-comparable seed-a (9223372036854775807 :: Nat),
      maphash-comparable seed-a (9223372036854775808 :: Nat),
      maphash-comparable seed-a (18446744073709551615 :: Nat),
      maphash-comparable seed-a (18446744073709551616 :: Nat),
      maphash-comparable seed-a big-nat,
      maphash-comparable seed-a (nat-add big-nat 1)
    ] in
    let big-int = (340282366920938463463374607431768211456 :: Int) in
    let int-ok = all-distinct [
      maphash-comparable seed-a (int-sub (int-neg big-int) 1),
      maphash-comparable seed-a (int-neg big-int),
      maphash-comparable seed-a (-1 :: Int),
      maphash-comparable seed-a (0 :: Int),
      maphash-comparable seed-a (1 :: Int),
      maphash-comparable seed-a big-int,
      maphash-comparable seed-a (int-add big-int 1)
    ] in
    let bytes-pair-ok = all-distinct [
      maphash-comparable seed-a (hb "0102", hb "03"),
      maphash-comparable seed-a (hb "01", hb "0203"),
      maphash-comparable seed-a (hb "", hb "010203"),
      maphash-comparable seed-a (hb "010203", hb "")
    ] in
    let opt-ok = all-distinct [
      maphash-comparable seed-a (None :: Option U64, Some (1 :: U64)),
      maphash-comparable seed-a (Some (1 :: U64), None :: Option U64),
      maphash-comparable seed-a (None :: Option U64, None :: Option U64),
      maphash-comparable seed-a (Some (0 :: U64), None :: Option U64)
    ] in
    let list-ok = all-distinct [
      maphash-comparable seed-a ([[1 :: U8, 2 :: U8], [3 :: U8]] :: List (List U8)),
      maphash-comparable seed-a ([[1 :: U8], [2 :: U8, 3 :: U8]] :: List (List U8)),
      maphash-comparable seed-a ([[], [1 :: U8, 2 :: U8, 3 :: U8]] :: List (List U8)),
      maphash-comparable seed-a ([[1 :: U8, 2 :: U8, 3 :: U8]] :: List (List U8)),
      maphash-comparable seed-a ([] :: List (List U8))
    ] in
    let sample-pair = ("hello", [1 :: Int, 2 :: Int] :: List Int) in
    let seed-indep = ne-u64 (maphash-comparable seed-a sample-pair) (maphash-comparable seed-b sample-pair) in
    bool-ok && u8-ok && u16-ok && u32-ok && i8-ok && i16-ok && i32-ok && u128-ok && i128-ok && nan-ok && char-ok && nat-ok && int-ok && bytes-pair-ok && opt-ok && list-ok && seed-indep

  -- | Verify MapHash is persistent and immutable: branching from h1 never mutates h1.
  check-persistent-branching :: Unit -> Bool
  check-persistent-branching u =
    let h1 = maphash-write-string (maphash-new seed-a) "shared-prefix:" in
    let d1-before = maphash-sum64 h1 in
    let h2 = maphash-write-string h1 "branch-alpha" in
    let h3 = maphash-write-string h1 "branch-beta" in
    let d1-after = maphash-sum64 h1 in
    eq-u64 d1-before d1-after
      && eq-u64 (maphash-sum64 h2) (maphash-string seed-a "shared-prefix:branch-alpha")
      && eq-u64 (maphash-sum64 h3) (maphash-string seed-a "shared-prefix:branch-beta")
      && ne-u64 (maphash-sum64 h2) (maphash-sum64 h3)

  -- | Verify Write + WriteComparable does not commute with WriteComparable + Write.
  check-write-comparable-non-commuting :: Unit -> Bool
  check-write-comparable-non-commuting u =
    let h1 = maphash-write-comparable (maphash-write-string (maphash-new seed-a) "a") "b" in
    let h2 = maphash-write-string (maphash-write-comparable (maphash-new seed-a) "a") "b" in
    ne-u64 (maphash-sum64 h1) (maphash-sum64 h2)

  -- | Verify write-u64-le (via MapHashable U64) matches byte-by-byte LE writes at every buffer alignment 0..=7.
  check-scalar-alignments :: Nat -> Bool
  check-scalar-alignments pad =
    if pad >= 8 then True
    else
      let prefix = bytes-new pad in
      let v = (0x0123456789abcdef :: U64) in
      let le-buf = bytes-new 8 in
      let u = (if 8 <= bytes-len le-buf then bytes-write-le64 le-buf 0 v else ()) in
      let h-fast = maphash-write-comparable (maphash-write (maphash-new seed-a) prefix) v in
      let h-ref = maphash-write (maphash-write (maphash-new seed-a) prefix) le-buf in
      let h-fast2 = maphash-write-comparable h-fast (0xfedcba9876543210 :: U64) in
      let le-buf2 = bytes-new 8 in
      let u2 = (if 8 <= bytes-len le-buf2 then bytes-write-le64 le-buf2 0 0xfedcba9876543210 else ()) in
      let h-ref2 = maphash-write h-ref le-buf2 in
      if eq-u64 (maphash-sum64 h-fast) (maphash-sum64 h-ref) && eq-u64 (maphash-sum64 h-fast2) (maphash-sum64 h-ref2) then
        check-scalar-alignments (pad + 1)
      else False

  -- | Return the index of the first failing row, or 200 when every row passes.
  first :: List Bool -> Int -> Int
  first xs i = case xs of
    Nil -> 200
    Cons ok rest -> if ok then first rest (i + 1) else i
"#;
