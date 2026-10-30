//! Small `Nat`/`Int` immediates (design/reclamation.md section 2.6; the
//! runtime ABI's "Small `Nat`/`Int` immediates" in
//! `compiler/runtime/src/lib.rs`), observed through compiled programs.
//!
//! A `Nat` word carries `0 ..= 2^63 - 1` and an `Int` word
//! `-2^62 ..= 2^62 - 1` without a heap block; outside those ranges the
//! value is a bignum block. Every result is canonical, so these programs
//! cross each edge in both directions: a sum or product that overflows an
//! immediate must come back exact, and a difference, quotient or negation
//! that falls back in range must equal the value every other path builds
//! (a folded literal, a decimal parse, a machine conversion). Each program
//! takes a zero from its own (empty) command line, so no operand is a
//! compile-time constant the optimizer could fold away: the runtime's
//! fast and slow paths are what run.
//!
//! The exit code is the 1-based position of the first failed check (0
//! when all pass), so a red run names the check.

use cinch::driver::run_source;

/// Compile and run `src`; its exit code.
fn run(name: &str, src: &str) -> i32 {
    match run_source(src) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// The shared helpers: the verdict, a decimal parser written in Cinch
/// (so the round trip does not lean on the runtime's own formatter), and
/// the `U64` crossings, which the language spells through `I64`.
const HELPERS: &str = r#"
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 0
    Cons x rest -> if x then first-fail rest (i + 1) else i

  -- The digit's value as its low nibble: `c - 48` on every ASCII digit,
  -- and no lower bound to prove. The scan's own obligation keeps each
  -- read inside the buffer.
  digits-nat :: using (i <= n, n <= bytes-len b) => (b :: Bytes) -> (i :: Nat) -> (n :: Nat) -> Nat -> Nat
  digits-nat b i n acc =
    if i == n then acc else digits-nat b (i + 1) n (acc * 10 + bytes-get b i % 16)

  parse-nat :: String -> Nat
  parse-nat s =
    let b = string-to-bytes s in
    digits-nat b 0 (bytes-len b) 0

  digits-int :: using (i <= n, n <= bytes-len b) => (b :: Bytes) -> (i :: Nat) -> (n :: Nat) -> Int -> Int
  digits-int b i n acc =
    if i == n then acc
    else digits-int b (i + 1) n (acc * 10 + nat-to-int (bytes-get b i % 16))

  parse-int :: String -> Int
  parse-int s =
    let b = string-to-bytes s in
    if bytes-len b == 0 then 0
    else if bytes-get b 0 == 45 then int-neg (digits-int b 1 (bytes-len b) 0)
    else digits-int b 0 (bytes-len b) 0

  nat-round-trips :: Nat -> String -> Bool
  nat-round-trips n s = nat-to-string n == s && parse-nat s == n

  int-round-trips :: Int -> String -> Bool
  int-round-trips n s = int-to-string n == s && parse-int s == n

  two63 :: Nat
  two63 = 9223372036854775808

  -- The sign bit cleared is the low 63 bits (`mask-i64`, a non-negative
  -- word): a negative word's `i - min-bound`, the same value.
  u64-to-nat :: U64 -> Nat
  u64-to-nat u =
    let i = wrapping-u64-to-i64 u in
    if i >= 0 then nat-from-i64 i
    else nat-from-i64 (mask-i64 i 9223372036854775807) + two63

  -- The low 63 bits of `n - 2^63` (a residue below 2^63, so it is an
  -- `I64`): the value itself for every `n` below 2^64.
  nat-to-u64 :: Nat -> U64
  nat-to-u64 n =
    if n < 9223372036854775808 then wrapping-i64-to-u64 (nat-to-i64 n)
    else
      let low = nat-rem (n - 9223372036854775808) 9223372036854775808 in
      wrapping-i64-to-u64 (wrapping-add-i64 (nat-to-i64 low) min-bound-i64)

  -- An `Int` the checks know is in `I64` range (`0` would mark a
  -- conversion the check did not expect to leave the range).
  i64-of :: Int -> I64
  i64-of x = case checked-int-to-i64 x of
    Some v -> v
    None -> 0

  nat-found :: Nat -> Nat -> HashMap Nat Nat -> Bool
  nat-found k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  int-found :: Int -> Nat -> HashMap Int Nat -> Bool
  int-found k v m = case hash-map-lookup k m of
    None -> False
    Some w -> w == v

  nat-label :: Nat -> Nat
  nat-label n = case n of
    9223372036854775807 -> 1
    9223372036854775808 -> 2
    18446744073709551616 -> 3
    other -> 4
"#;

/// A program whose `checks` takes the runtime zero `z` (the length of
/// the empty argument list).
fn program(checks: &str) -> String {
    format!(
        r#"
module Main where
  import Data.List
  import Data.Nat
  import Data.Int
  import Data.Bytes
  import Data.String
  import Data.Result
  import Data.HashMap
  import Text.Show
  import Effect.Process
{HELPERS}
{checks}

  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in case process-args () of
    Err e -> 200
    Ok args -> first-fail (checks (list-length args)) 1
"#
    )
}

#[test]
fn nat_crosses_the_immediate_edge_exactly_in_both_directions() {
    let src = program(
        r#"
  -- A runtime divisor is decided before it divides (the divisor is a
  -- `NonZero`); a zero would fail the check rather than fault.
  nat-quot-is :: Nat -> Nat -> Nat -> Bool
  nat-quot-is a d q = case decide (NonZero d) of
    Yes w -> nat-div a (NonZero d w) == q
    No _ -> False

  nat-rem-is :: Nat -> Nat -> Nat -> Bool
  nat-rem-is a d r = case decide (NonZero d) of
    Yes w -> nat-rem a (NonZero d w) == r
    No _ -> False

  -- The edge values are built at runtime from `z`, so no fact orders
  -- them: each difference goes through `checked-sub` (the runtime
  -- subtraction on the same words), and a reversed pair would show as a
  -- failed check, not a fault.
  minus :: Nat -> Nat -> Nat
  minus a b = case checked-sub a b of
    Some d -> d
    None -> 0

  checks :: Nat -> List Bool
  checks z =
    let m62 = 4611686018427387903 + z in
    let p62 = m62 + 1 in
    let m63 = minus (p62 * 2) 1 in
    let p63 = m63 + 1 in
    let p64 = p63 * 2 in
    [ nat-round-trips m62 "4611686018427387903"
    , nat-round-trips p62 "4611686018427387904"
    , nat-round-trips m63 "9223372036854775807"
    , nat-round-trips p63 "9223372036854775808"
    , nat-round-trips p64 "18446744073709551616"
    , nat-round-trips z "0"
    , m63 == 9223372036854775807
    , p63 == 9223372036854775808
    , p64 == 18446744073709551616
    , p63 + m63 == 18446744073709551615
    , minus p64 p63 == p63
    , minus (minus p64 p63) 1 == m63
    , minus p63 m63 == 1
    , p64 / 2 == p63
    , nat-quot-is p64 p63 2
    , nat-rem-is p64 p63 z
    , p63 / 3 == 3074457345618258602
    , p63 % 3 == 2
    , nat-quot-is m63 m63 1
    , m63 * m63 == 85070591730234615847396907784232501249
    , nat-quot-is (m63 * m63) m63 m63
    , minus (m63 + m63) m63 == m63
    , m63 < p63
    , p63 > m63
    , p64 > p63
    , not (p63 < m63)
    , nat-cmp p63 9223372036854775808 == 0
    , i64-of (nat-to-int m63) == max-bound-i64
    , nat-from-i64 max-bound-i64 == m63
    , nat-to-u64 p63 == wrapping-i64-to-u64 min-bound-i64
    , nat-to-u64 (minus p64 1) == max-bound-u64
    , u64-to-nat max-bound-u64 == minus p64 1
    , u64-to-nat (nat-to-u64 p63) == p63
    , u64-to-nat (nat-to-u64 m63) == m63
    , nat-label m63 == 1
    , nat-label p63 == 2
    , nat-label p64 == 3
    , nat-label (minus p64 1) == 4
    ]
"#,
    );
    // A nonzero exit is the position of the first failed check.
    assert_eq!(run("nat-edges", &src), 0);
}

#[test]
fn int_crosses_both_immediate_edges_and_their_negations() {
    let src = program(
        r#"
  -- A runtime divisor is decided before it divides (see the `Nat` edges).
  int-quot-is :: Int -> Int -> Int -> Bool
  int-quot-is a d q = case decide (NonZero d) of
    Yes w -> int-div a (NonZero d w) == q
    No _ -> False

  int-rem-is :: Int -> Int -> Int -> Bool
  int-rem-is a d r = case decide (NonZero d) of
    Yes w -> int-rem a (NonZero d w) == r
    No _ -> False

  checks :: Nat -> List Bool
  checks z =
    let zi = nat-to-int z in
    let imax = 4611686018427387903 + zi in
    let imin = (0 - 4611686018427387904) + zi in
    let above = imax + 1 in
    let below = imin - 1 in
    let minus-one = zi - 1 in
    [ int-round-trips imax "4611686018427387903"
    , int-round-trips imin "-4611686018427387904"
    , int-round-trips above "4611686018427387904"
    , int-round-trips below "-4611686018427387905"
    , int-round-trips zi "0"
    , int-round-trips (zi - 18446744073709551616) "-18446744073709551616"
    , above == 4611686018427387904
    , below == 0 - 4611686018427387905
    , above - 1 == imax
    , below + 1 == imin
    , int-neg imin == above
    , int-neg above == imin
    , int-neg imax == imin + 1
    , int-neg below == above + 1
    , int-quot-is imin minus-one above
    , int-rem-is imin minus-one zi
    , above * minus-one == imin
    , imax * minus-one == imin + 1
    , (zi - 7) / 2 == 0 - 3
    , (zi - 7) % 2 == 0 - 1
    , (zi + 7) / (-2) == 0 - 3
    , (zi + 7) % (-2) == 1
    , int-quot-is (above * above) above above
    , below < imin
    , imin < imax
    , above > imax
    , below < above
    , int-cmp below (0 - 4611686018427387905) == 0
    , int-to-i64 (int-from-i64 max-bound-i64) == max-bound-i64
    , int-to-i64 (int-from-i64 min-bound-i64) == min-bound-i64
    , int-from-i64 min-bound-i64 == 0 - 9223372036854775808
    , int-from-i64 max-bound-i64 + 1 == 9223372036854775808
    , i64-of above == wrapping-add-i64 (i64-of imax) 1
    , i64-of imin == wrapping-sub-i64 0 (i64-of above)
    ]
"#,
    );
    assert_eq!(run("int-edges", &src), 0);
}

#[test]
fn equal_values_agree_as_map_keys_whichever_path_built_them() {
    // `Hash` and `Eq` must agree across spellings: a key stored from one
    // computation is found under the literal of the same value and under
    // a different computation of it, on both sides of each edge.
    let src = program(
        r#"
  checks :: Nat -> List Bool
  checks z =
    let m63 = (9223372036854775808 + z) - 1 in
    let p63 = (4611686018427387904 + z) * 2 in
    let p64 = p63 + p63 in
    let nats = hash-map-insert p64 3 (hash-map-insert p63 2 (hash-map-insert m63 1 hash-map-empty)) in
    let zi = nat-to-int z in
    let above = (4611686018427387903 + zi) + 1 in
    let imin = (zi - 4611686018427387905) + 1 in
    let ints = hash-map-insert imin 2 (hash-map-insert above 1 hash-map-empty) in
    [ nat-found 9223372036854775807 1 nats
    , nat-found 9223372036854775808 2 nats
    , nat-found 18446744073709551616 3 nats
    , nat-found (nat-from-i64 max-bound-i64) 1 nats
    , nat-found (parse-nat "9223372036854775808") 2 nats
    , nat-found (p64 / 2) 2 nats
    , hash-map-size nats == 3
    , int-found 4611686018427387904 1 ints
    , int-found (int-neg (zi - 4611686018427387904)) 1 ints
    , int-found (0 - 4611686018427387904) 2 ints
    , int-found (parse-int "-4611686018427387904") 2 ints
    , hash-map-size ints == 2
    ]
"#,
    );
    assert_eq!(run("map-keys", &src), 0);
}

#[test]
fn division_by_an_immediate_takes_a_decided_divisor() {
    // The divisor is a `NonZero`, so the immediate fast path never sees a
    // zero from well-typed code: a runtime divisor is decided first, and
    // both arms run on immediates (`z` is 0 here).
    for (name, expr) in [
        ("nat-div", "let d = 5 + z in case decide (NonZero d) of\n      Yes w -> nat-div (10 + z) (NonZero d w) == 2\n      No _ -> False"),
        ("nat-rem", "let d = 3 + z in case decide (NonZero d) of\n      Yes w -> nat-rem (10 + z) (NonZero d w) == 1\n      No _ -> False"),
        ("zero-refused", "case decide (NonZero z) of\n      Yes w -> False\n      No _ -> True"),
    ] {
        let checks = format!("  checks :: Nat -> List Bool\n  checks z = [ {expr} ]\n");
        assert_eq!(run(name, &program(&checks)), 0, "{name}");
    }
}

#[test]
fn nat_subtraction_on_immediates_takes_its_evidence_from_a_test() {
    // Both operands immediate: a difference needs `b <= a`, so the
    // below-zero case is the guard's other branch, never a wrapped
    // immediate (`z` is 0 here).
    for (name, expr) in [
        ("nat-sub-in-order", "if 4 <= 7 + z then (7 + z) - 4 == 3 else False"),
        ("nat-sub-reversed", "if 4 <= 3 + z then False else (case checked-sub (3 + z) 4 of None -> True; Some _ -> False)"),
    ] {
        let checks = format!("  checks :: Nat -> List Bool\n  checks z = [ {expr} ]\n");
        assert_eq!(run(name, &program(&checks)), 0, "{name}");
    }
}

#[test]
fn nat_from_a_possibly_negative_i64_is_refused_and_the_checked_form_answers() {
    // `nat-from-i64` states `0 <= x`: an argument that may be negative is
    // a compile error (the runtime's negative check is a compiler-bug
    // trap), and `checked-nat-from-i64` answers `None` for it.
    let refused = "  checks :: Nat -> List Bool\n  checks z = [ nat-from-i64 (wrapping-sub-i64 (i64-of (nat-to-int z)) 1) == 0 ]\n";
    match cinch::driver::check_sources(&program(refused), "<test>") {
        Ok(_) => panic!("a possibly negative argument must be refused"),
        Err(e) => assert!(e.to_string().contains("the obligation `0 <= "), "{e}"),
    }
    let checks = "  none-nat :: Option Nat -> Bool\n  none-nat o = case o of\n    None -> True\n    Some _ -> False\n\n  checks :: Nat -> List Bool\n  checks z = [ none-nat (checked-nat-from-i64 (wrapping-sub-i64 (i64-of (nat-to-int z)) 1)) ]\n";
    assert_eq!(run("checked-nat-from-neg-i64", &program(checks)), 0, "checked-nat-from-neg-i64");
}
