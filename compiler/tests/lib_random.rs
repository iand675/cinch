//! Effect.Random consumer semantics: the OS entropy surface
//! (`random-bytes`, its lengths and its freshness), the bias-free
//! bounded draws built on it (`random-nat-below` over its rejection
//! region, `random-nat-in-range`'s half-open shift, `random-nat`'s full
//! word), the named bound refusals (`EmptyBound`, `EmptyRange`), and the
//! hex renderers (`hex-of-bytes`, `random-hex`).
//!
//! Each test compiles a real project: the module under test
//! (`prelude/Effect/Random.cinch`) is written beside `Main.cinch`, which
//! drives it through the CLI pipeline like `modules.rs` does. The checks
//! are top-level functions returning 0 or the check's weight; `main`
//! sums the weights, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes 2^(k-1)) and any failure mode of
//! one check shares its bit, keeping every total inside the 8-bit exit
//! status. The checks are top-level because a `case` in a `let` binder's
//! right-hand side must indent past the binder's column under the layout
//! rules (the `lib_foldable.rs` note).
//!
//! The probabilistic checks are pinned, not sampled into flakiness: two
//! 32-byte draws colliding has probability 2^-256, 64 draws of a uniform
//! 4-value draw all equalling the first has probability 4^-63, and two
//! 64-bit draws colliding has probability 2^-64. A fixed expectation is
//! therefore safe for each, and the reasoning is repeated at each
//! observation's site.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;

const RANDOM: &str = include_str!("../prelude/Effect/Random.cinch");

/// The module under test, plus a `Main.cinch` that drives it.
fn run_main(name: &str, main_src: &str) -> i32 {
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_random_{}_{}",
        std::process::id(),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Random.cinch"), RANDOM).expect("write Effect.Random");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main");
    // The checker's elaboration recursion runs near the default test
    // thread stack at the whole-prelude baseline, so compile on a thread
    // with a generous stack (as `tests/lib_base64.rs` and
    // `tests/runtime_prims.rs` do).
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

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn random_bytes_have_the_requested_length_and_two_draws_differ() {
    expect_ok(
        "lengths",
        r#"
module Main where

  import Data.Bytes
  import Data.Nat
  import Effect.Random
  import Relation.Eq
  import Relation.Ord

  -- The weight of one check: 0 when it passed, `w` when it failed.
  weight :: Nat -> Int -> Int
  weight r w = case r == 0 of
    True -> 0
    False -> w

  -- 0 when `b` has exactly `n` bytes.
  len-is :: Bytes -> Nat -> Nat
  len-is b n = case bytes-len b of
    k -> if k == n then 0 else 1

  -- 0 when the two buffers differ somewhere (the first difference is
  -- enough); 1 when every byte agrees. A collision of two independent
  -- 32-byte OS draws has probability 2^-256, so pinning the inequality
  -- as an expectation is safe: it is not a plausible event in the life
  -- of the universe, and a failure would mean the entropy source is not
  -- entropy.
  differ :: Bytes -> Bytes -> Nat -> Nat
  differ a b i = case i == bytes-len a of
    True -> 1
    False -> case bytes-get a i == bytes-get b i of
      True -> differ a b (i + 1)
      False -> 0

  -- Two draws of the same length, compared byte for byte. (A name here
  -- may not carry a digit straight after a hyphen: `fresh-32` lexes as
  -- `fresh` applied to the literal `-32`, so the declarations below
  -- spell counts out in words.)
  two-draws-differ :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  two-draws-differ u =
    let a = random-bytes 32 in
    let b = random-bytes 32 in
    differ a b 0

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    -- 0 is the PINNED legal empty request: the runtime boxes an empty
    -- buffer and the zero-byte read succeeds, so it is a length like any
    -- other and not an error.
    let c1 = weight (len-is (random-bytes 0) 0 + len-is (random-bytes 1) 1 + len-is (random-bytes 16) 16 + len-is (random-bytes 65) 65) 1 in
    let c2 = weight (two-draws-differ ()) 2 in
    c1 + c2
"#,
    );
}

#[test]
fn bounded_draws_stay_in_range_and_reach_distinct_values() {
    expect_ok(
        "bounds",
        r#"
module Main where

  import Data.Nat
  import Data.Result
  import Effect.Random
  import Relation.Eq
  import Relation.Ord

  -- The weight of one check: 0 when it passed, `w` when it failed.
  weight :: Nat -> Int -> Int
  weight r w = case r == 0 of
    True -> 0
    False -> w

  -- `random-nat-below 1`: bound one has a zero-byte draw and the
  -- acceptance region `1`, so every draw must be 0 and no entropy is
  -- consumed (the error arm is a failure signal: bound one is legal).
  below-one :: forall r. using (io :: Io r) => Nat ->{IO r} Nat
  below-one n = case n == 0 of
    True -> 0
    False -> case random-nat-below 1 of
      Err e -> 1
      Ok v -> if v == 0 then below-one (n - 1) else 1

  -- `random-nat-below 4` over 64 draws: every draw in `0 .. 3`, and at
  -- least two distinct values. 64 draws of a uniform 4-value draw all
  -- equalling the first has probability 4^-63 (about 2^-126), so the
  -- distinctness expectation is safe.
  below-four :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  below-four u = case random-nat-below 4 of
    Err e -> 1
    Ok first -> four-rest 63 first False

  four-rest :: forall r. using (io :: Io r) => Nat -> Nat -> Bool ->{IO r} Nat
  four-rest n first differs = case n == 0 of
    True -> if differs then 0 else 1
    False -> case random-nat-below 4 of
      Err e -> 1
      Ok v -> case v < 4 of
        True -> four-rest (n - 1) first (if v == first then differs else True)
        False -> 1

  -- `random-nat-in-range 5 10` over 64 draws: the upper bound is
  -- EXCLUSIVE, so every draw is in `5 .. 9`, and at least two distinct
  -- values appear (the same 4^-63 argument over the five-value range).
  in-range :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  in-range u = case random-nat-in-range 5 10 of
    Err e -> 1
    Ok first -> range-rest 63 first False

  range-rest :: forall r. using (io :: Io r) => Nat -> Nat -> Bool ->{IO r} Nat
  range-rest n first differs = case n == 0 of
    True -> if differs then 0 else 1
    False -> case random-nat-in-range 5 10 of
      Err e -> 1
      Ok v -> case 5 <= v && v < 10 of
        True -> range-rest (n - 1) first (if v == first then differs else True)
        False -> 1

  -- Two full-word draws: equal ones have probability 2^-64, so pinning
  -- the inequality is safe and it observes that `random-nat` is a real
  -- 64-bit draw rather than a narrow one.
  full-word :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  full-word u =
    let a = random-nat () in
    let b = random-nat () in
    if a == b then 1 else 0

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = weight (below-one 8) 1 in
    let c2 = weight (below-four ()) 2 in
    let c3 = weight (in-range ()) 4 in
    let c4 = weight (full-word ()) 8 in
    c1 + c2 + c3 + c4
"#,
    );
}

#[test]
fn bound_and_range_errors_are_named_constructors() {
    expect_ok(
        "errors",
        r#"
module Main where

  import Data.Nat
  import Data.Result
  import Effect.Random
  import Relation.Eq
  import Relation.Ord

  -- The weight of one check: 0 when it passed, `w` when it failed.
  weight :: Nat -> Int -> Int
  weight r w = case r == 0 of
    True -> 0
    False -> w

  -- A zero bound is the named `EmptyBound` (never a fault: the process
  -- reaching the sum at all is part of the evidence here).
  zero-bound :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  zero-bound u = case random-nat-below 0 of
    Err EmptyBound -> 0
    Err EmptyRange -> 1
    Ok v -> 1

  -- An empty range is the named `EmptyRange`.
  empty-range :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  empty-range u = case random-nat-in-range 3 3 of
    Err EmptyRange -> 0
    Err EmptyBound -> 1
    Ok v -> 1

  -- An inverted range is the same named `EmptyRange` (there is one empty
  -- range, however it was spelled, and `hi - lo` is never formed below
  -- the guard).
  inverted-range :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  inverted-range u = case random-nat-in-range 9 2 of
    Err EmptyRange -> 0
    Err EmptyBound -> 1
    Ok v -> 1

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = weight (zero-bound ()) 1 in
    let c2 = weight (empty-range ()) 2 in
    let c3 = weight (inverted-range ()) 4 in
    c1 + c2 + c3
"#,
    );
}

#[test]
fn hex_renderers_are_lower_case_with_two_characters_per_byte() {
    expect_ok(
        "hex",
        r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.String
  import Effect.Random
  import Relation.Eq
  import Relation.Ord

  -- The weight of one check: 0 when it passed, `w` when it failed.
  weight :: Nat -> Int -> Int
  weight r w = case r == 0 of
    True -> 0
    False -> w

  -- The rendering of a known buffer: 0xAB 0xCD 0xEF 0x01 is
  -- "abcdef01", which pins the two-characters-per-byte length and the
  -- lower-case alphabet (the `a`..`f` half, where upper case would show)
  -- at once.
  known-bytes :: Bytes
  known-bytes = bytes-from-list (Cons 171 (Cons 205 (Cons 239 (Cons 1 Nil))))

  hex-known :: Nat
  hex-known = case hex-of-bytes known-bytes of
    s -> if s == "abcdef01" then 0 else 1

  -- `random-hex 8` is 16 characters, every one a lower-case hex digit
  -- (and ASCII, so the character count equals the byte count).
  hex-drawn :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  hex-drawn u = case random-hex 8 of
    s -> all-hex (string-to-bytes s) 0 16

  all-hex :: Bytes -> Nat -> Nat -> Nat
  all-hex b i n = case i == n of
    True -> if bytes-len b == n then 0 else 1
    False -> case hex-byte (bytes-get b i) of
      True -> all-hex b (i + 1) n
      False -> 1

  hex-byte :: Nat -> Bool
  hex-byte c =
    if c >= 48 && c <= 57 then True
    else if c >= 97 && c <= 102 then True
    else False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = weight hex-known 1 in
    let c2 = weight (hex-drawn ()) 2 in
    c1 + c2
"#,
    );
}
