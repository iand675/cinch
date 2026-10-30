//! The `Hash` instances a compiler's symbol tables need, and the keyed maps they make usable: a `HashMap Bytes v`, a `HashMap String v` and a `HashMap Sym v` built and queried in real programs through the shared harness (`compiler/tests/common/mod.rs`), plus the `Hash` law as a property over supplied pairs and the name-to-index table a compiler builds from one list and queries by name.  The `Hash` instance for `Bytes` and its `Eq` partner are what make a byte-keyed table possible at all (every `Data.HashMap` operation states both constraints), and the `Sym` instance makes an interned identifier a key.
//
// What each test pins (the observable behaviour, never the source text):
//   bytes keys     a table of 16 keys, arranged as 8 pairs that share a
//                  two-byte prefix and differ only in the last byte, is
//                  built and every key queried; the 16 digests are
//                  pairwise distinct (the index is not one bucket); two
//                  slices that hash alike ([0, 31] and [1, 0]) are
//                  proven to collide and both survive in one bucket;
//                  insert replaces in place and leaves the size fixed;
//                  a never-inserted key (and the empty slice) is absent;
//                  delete removes the key and keeps its collision
//                  partner.
//   string keys    the same story over 16 names, including 'lambda',
//                  'lambda1', 'lambda2' (a three-way shared prefix), a
//                  genuine digest collision built from the bytes above
//                  through `string-from-bytes`, absence, in-place
//                  replacement, and delete; and the string digest agrees
//                  with the digest of its own UTF-8 bytes, so the two
//                  instances agree by construction.
//   the law        `x == y implies hash x == hash y` holds over pairs
//                  the test supplies for `String`, `Bytes` and `Sym`
//                  (equal values built by different routes), checked by
//                  one generic function over a list of pairs.
//   name to index  a symbol table: a name-to-index map built from a list
//                  and queried for present and absent names, the absent
//                  case answering -1 (the shape `Bootstrap.Check` needs),
//                  the same table keyed by the names' bytes answering
//                  identically, and an overwrite visible through the key.
//   home reach     one minimal program per key type: Bytes keys with only
//                  `Data.Bytes` imported beside the class and map homes,
//                  String keys with only `Data.String`, and Sym keys with
//                  only `Data.Sym`, so no other carrier's home is in the
//                  import list.  The declarations themselves are asserted
//                  to live in the carriers' home sources (and not in the
//                  class home), which is the ownership rule the class
//                  header states.
//
// Each program is a `Main` whose top-level `check-k` returns 0 or its
// bit; `main` sums them, so a nonzero exit code is a bitmask naming the
// failed check (check k contributes bit 2^(k-1)), keeping the total
// inside the 8-bit exit status.  The checks are top-level because a
// `case` in a `let` binder's right-hand side must indent past the
// binder's column under the layout rules.
//
// On instance visibility, stated precisely: an implicit instance is a
// module-owned fact, and resolution walks every collected module, so
// "reachable from the type's home" here means the program's explicit
// imports are only the key type's own home plus the class and map homes,
// and the declaration lives in that home (asserted on the sources).  A
// carrier whose home is missing from the compiled family is the case
// that fails: nothing else supplies the instance.

mod common;

use common::{cleanup, project, run};

/// Compile and run one `Main` program in a fresh harness project; return
/// the program's exit code.  The project is removed before the assertion
/// so a failure does not leak a directory.
fn run_main(tag: &str, main_src: &str) -> i32 {
    let dir = project(tag, &[("Main.cinch", main_src)]);
    let code = run(&dir, "Main.cinch");
    cleanup(&dir);
    code
}

fn expect_ok(tag: &str, main_src: &str) {
    let code = run_main(tag, main_src);
    assert_eq!(code, 0, "{tag}: check bitmask {code}");
}

/// The table of 16 keys sharing short prefixes, the proven digest
/// collision, the in-place replacement, absence and delete, all over
/// byte keys.
const BYTES_KEYS: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.Bytes

  hlookup-is :: Bytes -> Nat -> HashMap Bytes Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  hlookup-absent :: Bytes -> HashMap Bytes Nat -> Bool
  hlookup-absent k m = case hash-map-lookup k m of
    None -> True
    Some _ -> False

  -- The key's middle byte is the index's residue (the index stays below
  -- 256 here, so it is the index itself).
  key-a :: Nat -> Bytes
  key-a i = bytes-from-list (Cons 7 (Cons (nat-rem i 256) (Cons 1 Nil)))

  key-b :: Nat -> Bytes
  key-b i = bytes-from-list (Cons 7 (Cons (nat-rem i 256) (Cons 2 Nil)))

  coll-a :: Bytes
  coll-a = bytes-from-list (Cons 0 (Cons 31 Nil))

  coll-b :: Bytes
  coll-b = bytes-from-list (Cons 1 (Cons 0 Nil))

  add-pair :: Nat -> HashMap Bytes Nat -> HashMap Bytes Nat
  add-pair i m = hash-map-insert (key-b i) (100 + i) (hash-map-insert (key-a i) i m)

  build :: Nat -> HashMap Bytes Nat -> HashMap Bytes Nat
  build n m = if n == 0 then m else build (n - 1) (add-pair (n - 1) m)

  table :: HashMap Bytes Nat
  table = build 8 hash-map-empty

  lookups-ok :: Nat -> Nat -> HashMap Bytes Nat -> Bool
  lookups-ok i n m =
    if i >= n then True
    else if hlookup-is (key-a i) i m && hlookup-is (key-b i) (100 + i) m then lookups-ok (i + 1) n m
    else False

  member-nat :: Nat -> List Nat -> Bool
  member-nat x xs = case xs of
    Nil -> False
    Cons y ys -> if x == y then True else member-nat x ys

  digests-distinct :: List Nat -> Bool
  digests-distinct xs = case xs of
    Nil -> True
    Cons x rest -> if member-nat x rest then False else digests-distinct rest

  digests-from :: Nat -> Nat -> List Nat
  digests-from i n =
    if i >= n then Nil
    else Cons (hash (key-a i)) (Cons (hash (key-b i)) (digests-from (i + 1) n))

  overwritten :: HashMap Bytes Nat
  overwritten = hash-map-insert (key-a 2) 222 table

  coll-table :: HashMap Bytes Nat
  coll-table = hash-map-insert coll-b 200 (hash-map-insert coll-a 100 hash-map-empty)

  check1 :: Int
  check1 = if hash coll-a == hash coll-b then 0 else 1

  check2 :: Int
  check2 = if hash-map-size table == 16 then 0 else 2

  check3 :: Int
  check3 = if lookups-ok 0 8 table then 0 else 4

  check4 :: Int
  check4 = if hash (key-a 5) /= hash (key-b 5) && hlookup-is (key-a 5) 5 table && hlookup-is (key-b 5) 105 table then 0 else 8

  check5 :: Int
  check5 = if digests-distinct (digests-from 0 8) then 0 else 16

  check6 :: Int
  check6 = if hlookup-is (key-a 2) 222 overwritten && hash-map-size overwritten == 16 && hlookup-is (key-a 2) 2 table then 0 else 32

  check7 :: Int
  check7 = if hlookup-absent (key-a 99) table && hlookup-absent (bytes-from-list (Cons 42 Nil)) table && hlookup-absent (bytes-new 0) table then 0 else 64

  check8 :: Int
  check8 = if hlookup-is coll-a 100 coll-table && hlookup-is coll-b 200 coll-table && hash-map-size coll-table == 2 && hlookup-absent coll-a (hash-map-delete coll-a coll-table) && hlookup-is coll-b 200 (hash-map-delete coll-a coll-table) then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#;

/// The same story over string keys: a 16-name table (with a three-way
/// shared prefix), the digest collision built through
/// `string-from-bytes`, absence, replacement, delete, and agreement with
/// the digest of the name's own UTF-8 bytes.
const STRING_KEYS: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.Bytes
  import Data.String

  names :: List (String, Nat)
  names = Cons ("alpha", 0) (Cons ("beta", 1) (Cons ("gamma", 2) (Cons ("delta", 3) (Cons ("epsilon", 4) (Cons ("zeta", 5) (Cons ("eta", 6) (Cons ("theta", 7) (Cons ("iota", 8) (Cons ("kappa", 9) (Cons ("lambda", 10) (Cons ("lambda1", 11) (Cons ("lambda2", 12) (Cons ("mu", 13) (Cons ("nu", 14) (Cons ("xi", 15) Nil)))))))))))))))

  name-table :: HashMap String Nat
  name-table = hash-map-from-pairs names

  hlookup-is :: String -> Nat -> HashMap String Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  hlookup-absent :: String -> HashMap String Nat -> Bool
  hlookup-absent k m = case hash-map-lookup k m of
    None -> True
    Some _ -> False

  all-present :: List (String, Nat) -> HashMap String Nat -> Bool
  all-present ps m = case ps of
    Nil -> True
    Cons p rest -> case p of
      (k, v) -> if hlookup-is k v m then all-present rest m else False

  byte-agreement :: List (String, Nat) -> Bool
  byte-agreement ps = case ps of
    Nil -> True
    Cons p rest -> case p of
      (k, _) -> if hash k == hash (string-to-bytes k) then byte-agreement rest else False

  str-of :: List (InRange 0 255 Nat) -> String
  str-of xs = case string-from-bytes (bytes-from-list xs) of
    Some s -> s
    None -> empty-string

  coll-a :: String
  coll-a = str-of (Cons 0 (Cons 31 Nil))

  coll-b :: String
  coll-b = str-of (Cons 1 (Cons 0 Nil))

  coll-table :: HashMap String Nat
  coll-table = hash-map-insert coll-b 200 (hash-map-insert coll-a 100 hash-map-empty)

  overwritten :: HashMap String Nat
  overwritten = hash-map-insert "alpha" 99 name-table

  pruned :: HashMap String Nat
  pruned = hash-map-delete "gamma" name-table

  check1 :: Int
  check1 = if hash-map-size name-table == 16 then 0 else 1

  check2 :: Int
  check2 = if all-present names name-table then 0 else 2

  check3 :: Int
  check3 = if hash "lambda" /= hash "lambda1" && hash "lambda1" /= hash "lambda2" && hash "lambda" /= hash "lambda2" then 0 else 4

  check4 :: Int
  check4 = if hlookup-absent "omega" name-table && hlookup-absent "lambda3" name-table && hlookup-absent "" name-table then 0 else 8

  check5 :: Int
  check5 = if hlookup-is "alpha" 99 overwritten && hash-map-size overwritten == 16 && hlookup-is "alpha" 0 name-table then 0 else 16

  check6 :: Int
  check6 = if byte-agreement names then 0 else 32

  check7 :: Int
  check7 = if hash coll-a == hash coll-b && coll-a /= coll-b && hlookup-is coll-a 100 coll-table && hlookup-is coll-b 200 coll-table && hash-map-size coll-table == 2 then 0 else 64

  check8 :: Int
  check8 = if hlookup-absent "gamma" pruned && hash-map-size pruned == 15 && hlookup-is "mu" 13 pruned then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#;

/// The `Hash` law over pairs the test supplies: one generic function
/// over a list of pairs, applied to `String`, `Bytes` and `Sym` pairs
/// whose two sides are equal by construction but built differently.
const LAW_PAIRS: &str = r#"module Main where

  import Data.Hash
  import Data.Bytes
  import Data.String
  import Data.Sym

  law-pairs :: forall a. using (ha :: Hash a, ea :: Eq a) => List (a, a) -> Bool
  law-pairs ps = case ps of
    Nil -> True
    Cons p rest -> case p of
      (x, y) -> if x == y then hash x == hash y && law-pairs rest else False

  string-pairs :: List (String, String)
  string-pairs = Cons ("alpha", string-concat "al" "pha") (Cons ("table", string-concat "tabl" "e") (Cons ("", string-concat "" empty-string) Nil))

  bytes-built :: Bytes
  bytes-built = let b = bytes-new 1 in let u = bytes-set b 0 3 in b

  bytes-pairs :: List (Bytes, Bytes)
  bytes-pairs = Cons (bytes-new 0, bytes-from-list Nil) (Cons (bytes-from-list (Cons 3 Nil), bytes-built) Nil)

  sym-pairs :: List (Sym, Sym)
  sym-pairs = Cons (intern "alpha", intern "alpha") (Cons (intern (sym-name (intern "beta")), intern "beta") Nil)

  main :: Int
  main =
    if law-pairs string-pairs then
      (if law-pairs bytes-pairs then
        (if law-pairs sym-pairs then 0 else 4)
      else 2)
    else 1
"#;

/// The symbol table a compiler builds: a name-to-index map from one list,
/// queried for present and absent names, with the byte-keyed table
/// answering identically and an overwrite visible through the key.
const NAME_INDEX: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.Bytes
  import Data.String

  names :: List (String, Nat)
  names = Cons ("alpha", 0) (Cons ("beta", 1) (Cons ("gamma", 2) (Cons ("delta", 3) (Cons ("epsilon", 4) (Cons ("zeta", 5) (Cons ("eta", 6) (Cons ("theta", 7) (Cons ("iota", 8) (Cons ("kappa", 9) (Cons ("lambda", 10) (Cons ("mu", 11) Nil)))))))))))

  index-table :: HashMap String Nat
  index-table = hash-map-from-pairs names

  to-bytes-pairs :: List (String, Nat) -> List (Bytes, Nat)
  to-bytes-pairs ps = case ps of
    Nil -> Nil
    Cons p rest -> case p of
      (k, v) -> Cons (string-to-bytes k, v) (to-bytes-pairs rest)

  bytes-table :: HashMap Bytes Nat
  bytes-table = hash-map-from-pairs (to-bytes-pairs names)

  find-name :: String -> HashMap String Nat -> Int
  find-name n m = case hash-map-lookup n m of
    None -> 0 - 1
    Some i -> nat-to-int i

  find-bytes :: Bytes -> HashMap Bytes Nat -> Int
  find-bytes n m = case hash-map-lookup n m of
    None -> 0 - 1
    Some i -> nat-to-int i

  agree :: List (String, Nat) -> Bool
  agree ps = case ps of
    Nil -> True
    Cons p rest -> case p of
      (k, _) -> if find-name k index-table == find-bytes (string-to-bytes k) bytes-table then agree rest else False

  promoted :: HashMap String Nat
  promoted = hash-map-insert "mu" 99 index-table

  promoted-b :: HashMap Bytes Nat
  promoted-b = hash-map-insert (string-to-bytes "mu") 99 bytes-table

  check1 :: Int
  check1 = if find-name "alpha" index-table == 0 && find-name "lambda" index-table == 10 && find-name "mu" index-table == 11 then 0 else 1

  check2 :: Int
  check2 = if find-name "omega" index-table == 0 - 1 && find-name "lambda1" index-table == 0 - 1 && find-name "" index-table == 0 - 1 then 0 else 2

  check3 :: Int
  check3 = if find-bytes (string-to-bytes "zeta") bytes-table == 5 && find-bytes (string-to-bytes "alpha") bytes-table == 0 then 0 else 4

  check4 :: Int
  check4 = if find-bytes (string-to-bytes "omega") bytes-table == 0 - 1 && find-bytes (string-to-bytes "") bytes-table == 0 - 1 then 0 else 8

  check5 :: Int
  check5 = if agree names then 0 else 16

  check6 :: Int
  check6 = if hash-map-size index-table == 12 && hash-map-size bytes-table == 12 then 0 else 32

  check7 :: Int
  check7 = if find-name "mu" promoted == 99 && hash-map-size promoted == 12 && find-name "mu" index-table == 11 then 0 else 64

  check8 :: Int
  check8 = if find-bytes (string-to-bytes "mu") promoted-b == 99 && find-bytes (string-to-bytes "omega") promoted-b == 0 - 1 && hash-map-size promoted-b == 12 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#;

/// Bytes keys with only the key type's home imported beside the class
/// and map homes: no other carrier's home is in the list.
const REACH_BYTES: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.Bytes

  key :: Bytes
  key = bytes-from-list (Cons 4 (Cons 2 Nil))

  table :: HashMap Bytes Nat
  table = hash-map-insert key 7 hash-map-empty

  hlookup-is :: Bytes -> Nat -> HashMap Bytes Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  main :: Int
  main =
    if hlookup-is key 7 table then
      (if hash-map-size table == 1 then 0 else 2)
    else 1
"#;

/// String keys with only the string home imported beside the class and
/// map homes; the second check pins that the string digest is the byte
/// digest of the same text, so the instances agree by construction.
const REACH_STRING: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.String

  key :: String
  key = "alpha"

  table :: HashMap String Nat
  table = hash-map-insert key 7 hash-map-empty

  hlookup-is :: String -> Nat -> HashMap String Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  main :: Int
  main =
    if hlookup-is key 7 table then
      (if hash key == hash (string-to-bytes key) then 0 else 2)
    else 1
"#;

/// Sym keys with only the interned-symbol home imported beside the class
/// and map homes; `Sym` is not a builtin type name, so this program never
/// names `String` (the `intern` argument is a literal, which needs no
/// import).
const REACH_SYM: &str = r#"module Main where

  import Data.Hash
  import Data.HashMap
  import Data.Sym

  key :: Sym
  key = intern "alpha"

  table :: HashMap Sym Nat
  table = hash-map-insert key 7 hash-map-empty

  hlookup-is :: Sym -> Nat -> HashMap Sym Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  main :: Int
  main =
    if hlookup-is key 7 table then
      (if hash key == hash (intern "alpha") then 0 else 2)
    else 1
"#;

#[test]
fn bytes_keys_build_query_overwrite_and_absence() {
    expect_ok("hash-bytes-keys", BYTES_KEYS);
}

#[test]
fn string_keys_build_query_overwrite_and_absence() {
    expect_ok("hash-string-keys", STRING_KEYS);
}

#[test]
fn hash_law_holds_over_supplied_pairs() {
    expect_ok("hash-law-pairs", LAW_PAIRS);
}

#[test]
fn name_index_table_answers_present_and_absent_names() {
    expect_ok("hash-name-index", NAME_INDEX);
}

#[test]
fn instances_are_declared_in_their_types_home() {
    // The ownership rule the class header states, read off the sources:
    // each instance is declared in its carrier's home, and the class
    // home declares none of them.
    const BYTES: &str = include_str!("../prelude/Data/Bytes.cinch");
    const STRING: &str = include_str!("../prelude/Data/String.cinch");
    const SYM: &str = include_str!("../prelude/Data/Sym.cinch");
    const HASH: &str = include_str!("../prelude/Data/Hash.cinch");

    assert!(
        BYTES.contains("implicit instance Hash_Bytes :: Hash Bytes"),
        "Data.Bytes must declare Hash_Bytes"
    );
    assert!(
        BYTES.contains("implicit instance Eq_Bytes :: Eq Bytes"),
        "Data.Bytes must declare Eq_Bytes (every HashMap operation states both)"
    );
    assert!(
        STRING.contains("implicit instance Hash_String :: Hash String"),
        "Data.String must declare Hash_String"
    );
    assert!(
        SYM.contains("implicit instance Hash_Sym :: Hash Sym"),
        "Data.Sym must declare Hash_Sym"
    );
    assert!(
        !HASH.contains("implicit instance Hash_"),
        "the class home must not carry a carrier's instance"
    );
    assert!(
        STRING.contains("hash (string-bytes-raw s)"),
        "the string digest must delegate to the bytes instance (one algorithm)"
    );

    // And each home is enough to use its instance: one minimal program
    // per key type, importing that home and nothing else but the class
    // and map homes.
    expect_ok("hash-home-bytes", REACH_BYTES);
    expect_ok("hash-home-string", REACH_STRING);
    expect_ok("hash-home-sym", REACH_SYM);
}
