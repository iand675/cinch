//! Data.Map / Data.Set / Data.HashMap consumer semantics: the door
//! families the collection homes now ship, checked by compiling and
//! RUNNING real programs through the same driver the CLI uses.
//!
//! What each family pins (the observable behaviour, never the source
//! text):
//!   Data.Map       the empty name, delete of a present and an absent
//!                  key, both unions (left side wins a shared key; the
//!                  combining union sees left first), the ascending
//!                  views (`map-to-list`/`map-keys`/`map-values`), the
//!                  size and emptiness, the value map, the key-visible
//!                  filter, and the right fold (folding with `Cons`
//!                  rebuilds `map-to-list`; the empty map is the
//!                  identity).
//!   Data.HashMap   the same families in the buckets' order: keys,
//!                  values and entries are checked through a Set (the
//!                  order is an implementation detail and is never
//!                  pinned), the fold agrees with `hash-map-to-list`,
//!                  and deleting every key returns the empty map.
//!   Data.Set       delete of a present and an absent element (the
//!                  height-augmented tree tolerates both), the union,
//!                  intersection and difference over disjoint,
//!                  overlapping and empty inputs, subsethood true and
//!                  false, the image under a function (collisions keep
//!                  one copy), the filter, and the right fold with the
//!                  empty set as the identity.
//!
//! Each program is a `Main` whose top-level `check-k` returns 0 or its
//! bit; `main` sums them, so a nonzero exit code is a bitmask naming
//! the failed check (check k contributes bit 2^(k-1), keeping the
//! total inside the 8-bit exit status). The checks are top-level
//! because a `case` in a `let` binder's right-hand side must indent
//! past the binder's column under the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Compile and run one `Main` program (the embedded prelude family is
/// compiled in, so only the entry file is written).
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_mapset_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write module");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// Structural equality on `List Nat` (no `Eq` instance for containers
/// ships, so every program carries its own observer).
const EQ_LIST_NAT: &str = r#"
  eq-list-nat :: List Nat -> List Nat -> Bool
  eq-list-nat a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then eq-list-nat xs ys else False
"#;

/// Structural equality on `List (Nat, Nat)`.
const EQ_ENTRIES: &str = r#"
  eq-entries :: List (Nat, Nat) -> List (Nat, Nat) -> Bool
  eq-entries a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons e es -> case e of
      (k1, v1) -> case b of
        Nil -> False
        Cons f fs -> case f of
          (k2, v2) -> if k1 == k2 && v1 == v2 then eq-entries es fs else False
"#;

/// The keys of an entry list, in the list's own order: this is how a
/// program observes `map-to-list`'s order without pinning a whole
/// (key, value) listing.
const ENTRY_KEYS: &str = r#"
  entry-keys :: List (Nat, Nat) -> List Nat
  entry-keys es = case es of
    Nil -> Nil
    Cons e rest -> case e of
      (k, _) -> Cons k (entry-keys rest)
"#;

/// Order-independent comparison for the hash map's views: collect a
/// `List Nat` into a `Set Nat` and read it back ascending. Valid only
/// for lists of DISTINCT values, which is how the hash checks use it.
const NAT_SET_LIST: &str = r#"
  nat-set-list :: List Nat -> List Nat
  nat-set-list xs = set-to-list (set-from-list xs)
"#;

// ----------------------------------------------------------------
// Data.Map
// ----------------------------------------------------------------

#[test]
fn map_views_and_delete() {
    let src = format!(
        "module Main where\n\n{EQ_LIST_NAT}{EQ_ENTRIES}{ENTRY_KEYS}
  sample :: Map Nat Nat
  sample = map-from-pairs (Cons (2, 20) (Cons (1, 10) (Cons (3, 30) Nil)))

  pruned :: Map Nat Nat
  pruned = map-from-pairs (Cons (1, 10) (Cons (3, 30) Nil))

  check1 :: Int
  check1 = if map-size sample == 3 then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (map-keys sample) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (map-values sample) (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 4

  check4 :: Int
  check4 = if eq-list-nat (entry-keys (map-to-list sample)) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 8

  check5 :: Int
  check5 = case map-lookup 2 (map-delete 2 sample) of
    None -> 0
    Some _ -> 16

  check6 :: Int
  check6 = if eq-entries (map-to-list (map-delete 2 sample)) (map-to-list pruned) then 0 else 32

  check7 :: Int
  check7 = if eq-entries (map-to-list (map-delete 9 sample)) (map-to-list sample) then 0 else 64

  check8 :: Int
  check8 = if map-is-empty (map-empty :: Map Nat Nat) && map-is-empty sample == False && map-size (map-empty :: Map Nat Nat) == 0 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("map-views-delete", &src);
}

#[test]
fn map_union_and_transforms() {
    let src = format!(
        "module Main where\n\n{EQ_LIST_NAT}{EQ_ENTRIES}
  sample :: Map Nat Nat
  sample = map-from-pairs (Cons (2, 20) (Cons (1, 10) (Cons (3, 30) Nil)))

  extra :: Map Nat Nat
  extra = map-from-pairs (Cons (2, 200) (Cons (4, 400) Nil))

  u :: Map Nat Nat
  u = map-union extra sample

  uw :: Map Nat Nat
  uw = map-union-with (\\a b -> a + b) extra sample

  mapped :: Map Nat Nat
  mapped = map-map (\\v -> v + 1) sample

  filtered :: Map Nat Nat
  filtered = map-filter (\\k v -> k /= 2) sample

  dropped :: Map Nat Nat
  dropped = map-filter (\\k v -> False) sample

  lookup-is :: Nat -> Nat -> Map Nat Nat -> Bool
  lookup-is k v m = case map-lookup k m of
    None -> False
    Some w -> w == v

  check1 :: Int
  check1 = if lookup-is 2 200 u then 0 else 1

  check2 :: Int
  check2 = if map-size u == 4 && lookup-is 4 400 u && lookup-is 1 10 u then 0 else 2

  check3 :: Int
  check3 = if lookup-is 2 220 uw then 0 else 4

  check4 :: Int
  check4 = if lookup-is 1 10 uw && map-size uw == 4 then 0 else 8

  check5 :: Int
  check5 = if eq-list-nat (map-keys mapped) (Cons 1 (Cons 2 (Cons 3 Nil))) && eq-list-nat (map-values mapped) (Cons 11 (Cons 21 (Cons 31 Nil))) then 0 else 16

  check6 :: Int
  check6 = if eq-list-nat (map-keys filtered) (Cons 1 (Cons 3 Nil)) && map-size filtered == 2 && map-is-empty dropped then 0 else 32

  check7 :: Int
  check7 = if eq-entries (map-fold (\\k v acc -> Cons (k, v) acc) (Nil :: List (Nat, Nat)) sample) (map-to-list sample) then 0 else 64

  check8 :: Int
  check8 = if map-fold (\\k v acc -> k + acc) 0 (map-empty :: Map Nat Nat) == 0 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("map-union-transforms", &src);
}

// ----------------------------------------------------------------
// Data.HashMap
// ----------------------------------------------------------------

#[test]
fn hash_map_views_and_delete() {
    let src = format!(
        "module Main where\n\n  import Data.Set\n  import Data.HashMap\n{EQ_LIST_NAT}{NAT_SET_LIST}
  sample :: HashMap Nat Nat
  sample = hash-map-from-pairs (Cons (1, 10) (Cons (2, 20) (Cons (3, 30) Nil)))

  hlookup-is :: Nat -> Nat -> HashMap Nat Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  hlookup-absent :: Nat -> HashMap Nat Nat -> Bool
  hlookup-absent k m = case hash-map-lookup k m of
    None -> True
    Some _ -> False

  emptied :: HashMap Nat Nat
  emptied = hash-map-delete 1 (hash-map-delete 2 (hash-map-delete 3 sample))

  check1 :: Int
  check1 = if hash-map-size sample == 3 && hash-map-is-empty (hash-map-empty :: HashMap Nat Nat) && hash-map-is-empty sample == False then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (nat-set-list (hash-map-keys sample)) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 2

  check3 :: Int
  check3 = if list-length (hash-map-values sample) == 3 && eq-list-nat (nat-set-list (hash-map-values sample)) (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 4

  check4 :: Int
  check4 = if list-length (hash-map-to-list sample) == 3 && hash-map-size (hash-map-from-pairs (hash-map-to-list sample)) == 3 && hlookup-is 2 20 (hash-map-from-pairs (hash-map-to-list sample)) then 0 else 8

  check5 :: Int
  check5 = if hlookup-absent 2 (hash-map-delete 2 sample) then 0 else 16

  check6 :: Int
  check6 = if hash-map-size (hash-map-delete 2 sample) == 2 && hlookup-is 1 10 (hash-map-delete 2 sample) then 0 else 32

  check7 :: Int
  check7 = if hash-map-size (hash-map-delete 9 sample) == 3 && hlookup-is 3 30 (hash-map-delete 9 sample) then 0 else 64

  check8 :: Int
  check8 = if hlookup-is 2 22 (hash-map-insert 2 22 (hash-map-delete 2 sample)) && hash-map-size emptied == 0 && hash-map-is-empty emptied then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("hash-map-views-delete", &src);
}

#[test]
fn hash_map_union_and_transforms() {
    let src = format!(
        "module Main where\n\n  import Data.Set\n  import Data.HashMap\n{EQ_LIST_NAT}{EQ_ENTRIES}{NAT_SET_LIST}
  sample :: HashMap Nat Nat
  sample = hash-map-from-pairs (Cons (1, 10) (Cons (2, 20) (Cons (3, 30) Nil)))

  extra :: HashMap Nat Nat
  extra = hash-map-from-pairs (Cons (2, 200) (Cons (4, 400) Nil))

  u :: HashMap Nat Nat
  u = hash-map-union extra sample

  uw :: HashMap Nat Nat
  uw = hash-map-union-with (\\a b -> a + b) extra sample

  mapped :: HashMap Nat Nat
  mapped = hash-map-map (\\v -> v + 1) sample

  filtered :: HashMap Nat Nat
  filtered = hash-map-filter (\\k v -> k /= 2) sample

  hlookup-is :: Nat -> Nat -> HashMap Nat Nat -> Bool
  hlookup-is k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  hlookup-absent :: Nat -> HashMap Nat Nat -> Bool
  hlookup-absent k m = case hash-map-lookup k m of
    None -> True
    Some _ -> False

  check1 :: Int
  check1 = if hlookup-is 2 200 u then 0 else 1

  check2 :: Int
  check2 = if hash-map-size u == 4 && hlookup-is 4 400 u && hlookup-is 1 10 u then 0 else 2

  check3 :: Int
  check3 = if hlookup-is 2 220 uw then 0 else 4

  check4 :: Int
  check4 = if hlookup-is 1 10 uw && hash-map-size uw == 4 then 0 else 8

  check5 :: Int
  check5 = if eq-list-nat (nat-set-list (hash-map-keys mapped)) (Cons 1 (Cons 2 (Cons 3 Nil))) && eq-list-nat (nat-set-list (hash-map-values mapped)) (Cons 11 (Cons 21 (Cons 31 Nil))) then 0 else 16

  check6 :: Int
  check6 = if hash-map-size filtered == 2 && hlookup-absent 2 filtered && hash-map-size (hash-map-filter (\\k v -> False) sample) == 0 then 0 else 32

  check7 :: Int
  check7 = if eq-entries (hash-map-fold (\\k v acc -> Cons (k, v) acc) (Nil :: List (Nat, Nat)) sample) (hash-map-to-list sample) then 0 else 64

  check8 :: Int
  check8 = if hash-map-fold (\\k v acc -> k + acc) 0 (hash-map-empty :: HashMap Nat Nat) == 0 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("hash-map-union-transforms", &src);
}

// ----------------------------------------------------------------
// Data.Set
// ----------------------------------------------------------------

#[test]
fn set_views_delete_and_subset() {
    let src = format!(
        "module Main where\n\n  import Data.Set\n{EQ_LIST_NAT}
  sample :: Set Nat
  sample = set-from-list (Cons 3 (Cons 1 (Cons 2 Nil)))

  small :: Set Nat
  small = set-from-list (Cons 1 (Cons 3 Nil))

  outside :: Set Nat
  outside = set-from-list (Cons 1 (Cons 4 Nil))

  check1 :: Int
  check1 = if set-size sample == 3 && eq-list-nat (set-to-list sample) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (set-to-list (set-delete 2 sample)) (Cons 1 (Cons 3 Nil)) && set-member 2 (set-delete 2 sample) == False then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (set-to-list (set-delete 9 sample)) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 4

  check4 :: Int
  check4 = if set-is-empty (set-delete 1 (set-delete 2 (set-delete 3 sample))) then 0 else 8

  check5 :: Int
  check5 = if set-subset small sample then 0 else 16

  check6 :: Int
  check6 = if set-subset outside sample == False then 0 else 32

  check7 :: Int
  check7 = if set-subset (set-empty :: Set Nat) sample then (if set-subset sample (set-empty :: Set Nat) == False then 0 else 64) else 64

  check8 :: Int
  check8 = if set-is-empty (set-empty :: Set Nat) && set-is-empty sample == False && set-size (set-empty :: Set Nat) == 0 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("set-views-delete-subset", &src);
}

#[test]
fn set_union_intersection_difference() {
    let src = format!(
        "module Main where\n\n  import Data.Set\n{EQ_LIST_NAT}
  a :: Set Nat
  a = set-from-list (Cons 1 (Cons 2 (Cons 3 Nil)))

  b :: Set Nat
  b = set-from-list (Cons 3 (Cons 4 Nil))

  p :: Set Nat
  p = set-from-list (Cons 1 (Cons 2 Nil))

  q :: Set Nat
  q = set-from-list (Cons 5 (Cons 6 Nil))

  check1 :: Int
  check1 = if eq-list-nat (set-to-list (set-union a b)) (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil)))) then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (set-to-list (set-union p q)) (Cons 1 (Cons 2 (Cons 5 (Cons 6 Nil)))) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (set-to-list (set-union (set-empty :: Set Nat) b)) (Cons 3 (Cons 4 Nil)) && eq-list-nat (set-to-list (set-union a (set-empty :: Set Nat))) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 4

  check4 :: Int
  check4 = if eq-list-nat (set-to-list (set-intersection a b)) (Cons 3 Nil) then 0 else 8

  check5 :: Int
  check5 = if set-is-empty (set-intersection p q) && set-is-empty (set-intersection a (set-empty :: Set Nat)) && set-is-empty (set-intersection (set-empty :: Set Nat) a) then 0 else 16

  check6 :: Int
  check6 = if eq-list-nat (set-to-list (set-difference a b)) (set-to-list p) && eq-list-nat (set-to-list (set-difference b a)) (Cons 4 Nil) then 0 else 32

  check7 :: Int
  check7 = if eq-list-nat (set-to-list (set-difference a (set-empty :: Set Nat))) (Cons 1 (Cons 2 (Cons 3 Nil))) && eq-list-nat (set-to-list (set-difference (set-empty :: Set Nat) a)) (Nil :: List Nat) then 0 else 64

  check8 :: Int
  check8 = if set-is-empty (set-union (set-empty :: Set Nat) (set-empty :: Set Nat)) && set-subset (set-difference a b) a && set-subset (set-intersection a b) b then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"
    );
    expect_ok("set-union-intersection-difference", &src);
}

#[test]
fn set_map_filter_fold() {
    let src = format!(
        "module Main where\n\n  import Data.Set\n{EQ_LIST_NAT}
  sample :: Set Nat
  sample = set-from-list (Cons 3 (Cons 1 (Cons 2 Nil)))

  digits :: Set Nat
  digits = set-from-list (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil))))

  check1 :: Int
  check1 = if eq-list-nat (set-to-list (set-map (\\x -> x + 10) sample)) (Cons 11 (Cons 12 (Cons 13 Nil))) then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (set-to-list (set-map (\\x -> x % 2) digits)) (Cons 0 (Cons 1 Nil)) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (set-to-list (set-filter (\\x -> x > 1) sample)) (Cons 2 (Cons 3 Nil)) then 0 else 4

  check4 :: Int
  check4 = if set-is-empty (set-filter (\\x -> False) sample) && eq-list-nat (set-to-list (set-filter (\\x -> True) sample)) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 8

  check5 :: Int
  check5 = if eq-list-nat (set-fold (\\x acc -> Cons x acc) (Nil :: List Nat) sample) (set-to-list sample) then 0 else 16

  check6 :: Int
  check6 = if set-fold (\\x acc -> x + acc) 0 (set-empty :: Set Nat) == 0 && set-fold (\\x acc -> x + acc) 0 sample == 6 then 0 else 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6
"
    );
    expect_ok("set-map-filter-fold", &src);
}
