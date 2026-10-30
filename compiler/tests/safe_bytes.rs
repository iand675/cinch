//! `Bytes` operations are total by construction (design/no-faults.md):
//! `bytes-get` and `bytes-set` demand evidence that the index is below
//! `bytes-len b`, `bytes-copy-range` that both ranges end inside their
//! buffers, and a byte is an `InRange 0 255 Nat` both ways, so a read's
//! result carries its range and a write's value must prove it. A
//! buffer's length never changes, so facts over `bytes-len b` survive
//! writes to `b`, and `bytes-new n` gives `bytes-len out == n` to the
//! name it is bound to. A string literal's read-only backing exists only
//! inside a `String`: every conversion out copies into a fresh heap
//! buffer, so a write to it cannot be written.

mod common;

use cinch::driver::{check_sources, run_source};

fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needles:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

// ---------------------------------------------------------------- loops

#[test]
fn a_guarded_byte_loop_is_proven_in_bounds() {
    expect_run(
        "loops",
        r#"
module Main where
  -- The loop's own guard is the read's and the write's evidence.
  fill :: Bytes -> Nat -> Unit
  fill b i =
    if i < bytes-len b then
      let u = bytes-set b i 7 in
      fill b (i + 1)
    else ()

  sum-from :: Bytes -> Nat -> Nat -> Nat
  sum-from b i acc =
    if i < bytes-len b then sum-from b (i + 1) (acc + bytes-get b i) else acc

  -- A recursion that carries its bound as its own obligation: `i` stops
  -- at the length, and `i /= bytes-len b` together with `i <= bytes-len
  -- b` gives `i < bytes-len b`.
  count-sevens :: using (i <= bytes-len b) => (b :: Bytes) -> (i :: Nat) -> Nat
  count-sevens b i =
    if i == bytes-len b then 0
    else if bytes-get b i == 7 then 1 + count-sevens b (i + 1)
    else count-sevens b (i + 1)

  -- A decreasing index: `j` is one past the byte it reads.
  last-nonzero :: using (j <= bytes-len b) => (b :: Bytes) -> (j :: Nat) -> Nat
  last-nonzero b j =
    if j == 0 then 0
    else if bytes-get b (j - 1) /= 0 then j
    else last-nonzero b (j - 1)

  main :: Int
  main =
    let b = bytes-new 4 in
    let u = fill b 0 in
    -- `bytes-new 4` gives `bytes-len b == 4`: index 3 needs no guard.
    let v = bytes-set b 3 0 in
    if sum-from b 0 0 == 21 && count-sevens b 0 == 3 && last-nonzero b (bytes-len b) == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn an_unguarded_index_is_refused_naming_the_obligation() {
    expect_err(
        "unguarded-get",
        r#"
module Main where
  first :: Bytes -> Nat
  first b = bytes-get b 0

  main :: Int
  main = 0
"#,
        &["the obligation `0 < (Data.Bytes.bytes-len b)`"],
    );
    expect_err(
        "unguarded-set",
        r#"
module Main where
  poke :: Bytes -> Nat -> Unit
  poke b i = bytes-set b i 1

  main :: Int
  main = 0
"#,
        &["the obligation `i < (Data.Bytes.bytes-len b)`"],
    );
    // A guard on another buffer proves nothing about this one.
    expect_err(
        "wrong-buffer",
        r#"
module Main where
  peek :: Bytes -> Bytes -> Nat -> Nat
  peek a b i = if i < bytes-len a then bytes-get b i else 0

  main :: Int
  main = 0
"#,
        &["the obligation `i < (Data.Bytes.bytes-len b)`"],
    );
    // A literal past a fresh buffer's known length is refused as well.
    expect_err(
        "past-fresh",
        r#"
module Main where
  main :: Int
  main =
    let b = bytes-new 2 in
    if bytes-get b 2 == 0 then 0 else 1
"#,
        &["2 < (Data.Bytes.bytes-len b)"],
    );
}

#[test]
fn an_off_by_one_is_refused() {
    expect_err(
        "le-guard",
        r#"
module Main where
  sum-to :: Bytes -> Nat -> Nat
  sum-to b i = if i <= bytes-len b then bytes-get b i else 0

  main :: Int
  main = 0
"#,
        &["the obligation `i < (Data.Bytes.bytes-len b)`"],
    );
    expect_err(
        "own-obligation-le",
        r#"
module Main where
  walk :: using (i <= bytes-len b) => (b :: Bytes) -> (i :: Nat) -> Nat
  walk b i = bytes-get b i

  main :: Int
  main = 0
"#,
        &["the obligation `i < (Data.Bytes.bytes-len b)`"],
    );
    expect_err(
        "recursion-overshoots",
        r#"
module Main where
  walk :: using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> Nat
  walk b i = bytes-get b i + walk b (i + 1)

  main :: Int
  main = 0
"#,
        &["i + 1"],
    );
}

// ---------------------------------------------------------------- values

#[test]
fn a_byte_value_carries_its_range_both_ways() {
    expect_run(
        "byte-values",
        r#"
module Main where
  -- A read's result is an `InRange 0 255 Nat`, so it is a write's value
  -- with nothing to prove.
  copy-byte :: Bytes -> Bytes -> Unit
  copy-byte dst src =
    if 0 < bytes-len dst && 0 < bytes-len src then bytes-set dst 0 (bytes-get src 0) else ()

  -- A plain `Nat` enters the range from a branch fact.
  put-low :: Bytes -> Nat -> Unit
  put-low b v = if 0 < bytes-len b && v < 256 then bytes-set b 0 v else ()

  main :: Int
  main =
    let a = bytes-new 1 in
    let b = bytes-new 1 in
    let u = bytes-set a 0 255 in
    let v = copy-byte b a in
    let w = put-low a 300 in
    if bytes-get b 0 == 255 && bytes-get a 0 == 255 then 0 else 1
"#,
        0,
    );
    expect_err(
        "literal-too-large",
        r#"
module Main where
  main :: Int
  main =
    let b = bytes-new 1 in
    let u = bytes-set b 0 256 in
    0
"#,
        &["256"],
    );
    expect_err(
        "unproven-value",
        r#"
module Main where
  put :: Bytes -> Nat -> Unit
  put b v = if 0 < bytes-len b then bytes-set b 0 v else ()

  main :: Int
  main = 0
"#,
        &["v <= 255"],
    );
}

// ------------------------------------------------------------------ copy

#[test]
fn a_copy_range_is_proven() {
    expect_run(
        "copy",
        r#"
module Main where
  -- `bytes-new` gives `bytes-len out == bytes-len a + bytes-len b`, so
  -- both pieces land inside `out`.
  join :: Bytes -> Bytes -> Bytes
  join a b =
    let out = bytes-new (bytes-len a + bytes-len b) in
    let u = bytes-copy-range out a 0 0 (bytes-len a) in
    let v = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in
    out

  -- A guarded prefix copy.
  prefix :: Bytes -> Bytes -> Nat -> Unit
  prefix dst src n =
    if n <= bytes-len dst && n <= bytes-len src then bytes-copy-range dst src 0 0 n else ()

  main :: Int
  main =
    let a = bytes-new 2 in
    let b = bytes-new 3 in
    let u = bytes-set b 2 9 in
    let j = join a b in
    let v = prefix a b 9 in
    if bytes-len j == 5 && bytes-get j 4 == 9 then 0 else 1
"#,
        0,
    );
    expect_err(
        "copy-overruns",
        r#"
module Main where
  shift :: Bytes -> Unit
  shift b = bytes-copy-range b b 1 0 (bytes-len b)

  main :: Int
  main = 0
"#,
        &["the obligation `(1 + (Data.Bytes.bytes-len b)) <= (Data.Bytes.bytes-len b)`"],
    );
    expect_err(
        "copy-source-unchecked",
        r#"
module Main where
  take :: Bytes -> Bytes -> Nat -> Unit
  take dst src n = if n <= bytes-len dst then bytes-copy-range dst src 0 0 n else ()

  main :: Int
  main = 0
"#,
        &["(0 + n) <= (Data.Bytes.bytes-len src)"],
    );
}

// --------------------------------------------------------------- literals

/// A string literal's backing is a read-only data-section buffer, and a
/// `String` is not a `Bytes`: a write to a literal does not typecheck,
/// the representation constructor is not exported, and the one
/// conversion out copies, so writing the copy leaves the literal as it
/// was.
#[test]
fn a_literal_buffer_cannot_be_written() {
    expect_err(
        "string-is-not-bytes",
        r#"
module Main where
  main :: Int
  main =
    let u = bytes-set "abc" 0 65 in
    0
"#,
        &["String"],
    );
    expect_err(
        "no-representation",
        r#"
module Main where
  backing :: String -> Bytes
  backing s = case s of
    StrBuf b -> b

  main :: Int
  main = 0
"#,
        &["StrBuf"],
    );
    expect_run(
        "copy-out",
        r#"
module Main where
  main :: Int
  main =
    let s = "abc" in
    let b = string-to-bytes s in
    if bytes-len b == 3 then
      let u = bytes-set b 0 120 in
      let c = string-to-bytes s in
      if bytes-len c == 3 && bytes-get c 0 == 97 && bytes-get b 0 == 120 then 0 else 2
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ digit parse

/// The decimal digit loop: the read's range and an `is-digit` guard give
/// `48 <= c`, so `c - 48` needs nothing more; without the lower half of
/// the guard the subtraction is refused.
#[test]
fn the_digit_parse_shape_proves_its_subtraction() {
    expect_run(
        "digits",
        r#"
module Main where
  digits :: Bytes -> Nat -> Nat -> Nat
  digits b i acc =
    if i < bytes-len b then
      let c = bytes-get b i in
      if c >= 48 && c <= 57 then digits b (i + 1) (acc * 10 + (c - 48)) else acc
    else acc

  main :: Int
  main =
    let s = string-to-bytes "4096x7" in
    if digits s 0 0 == 4096 then 0 else 1
"#,
        0,
    );
    expect_err(
        "digit-without-lower-bound",
        r#"
module Main where
  digit :: Bytes -> Nat -> Nat
  digit b i =
    if i < bytes-len b then
      let c = bytes-get b i in
      if c <= 57 then c - 48 else 0
    else 0

  main :: Int
  main = 0
"#,
        &["the obligation `48 <= c`"],
    );
}

// ------------------------------------------------------- the linear pass

/// A copy's range has three terms (`di + n <= bytes-len dst`), outside
/// difference logic: the obligations below discharge only through the
/// linear pass (Fourier-Motzkin with a Farkas certificate the checker
/// re-checks), from a bound that is itself three-term.
#[test]
fn three_term_bounds_discharge_through_the_linear_pass() {
    expect_run(
        "three-term",
        r#"
module Main where
  -- `src` and a separator after it, at `di`.
  put :: using (di + bytes-len src + 1 <= bytes-len dst) => (dst :: Bytes) -> (src :: Bytes) -> (di :: Nat) -> Unit
  put dst src di =
    let u = bytes-copy-range dst src di 0 (bytes-len src) in
    bytes-set dst (di + bytes-len src) 0

  -- The words of a list, each followed by its separator, in one buffer
  -- made at the end of the list: the result's refinement is the bound.
  words :: (i :: Nat) -> List Bytes -> refined (i <= bytes-len out) Bytes
  words i xs = case xs of
    Nil -> bytes-new i
    Cons w rest ->
      let out = words (i + bytes-len w + 1) rest in
      let u = put out w i in
      out

  main :: Int
  main =
    let a = bytes-new 2 in
    let b = bytes-new 3 in
    let u = bytes-set b 2 9 in
    let all = words 0 (Cons a (Cons b Nil)) in
    if bytes-len all == 7 && bytes-get all 5 == 9 && bytes-get all 6 == 0 then 0 else 1
"#,
        0,
    );
}

/// The linear pass refutes the rational relaxation: a fact set with a
/// rational solution and no integer one is not refuted, so an
/// obligation under it is refused even though no integers reach it.
/// Here `x == 2 * y` and `x == 2 * z + 1` say `x` is both even and odd
/// (`y - z == 1/2` over the rationals). This is the procedure's one
/// documented incompleteness (design/proof-automation.md section 5).
#[test]
fn an_integer_only_contradiction_is_not_used() {
    expect_err(
        "parity",
        r#"
module Main where
  f :: Bytes -> Nat -> Nat -> Nat -> Nat
  f b x y z = if x == 2 * y && x == 2 * z + 1 then bytes-get b 0 else 0

  main :: Int
  main = 0
"#,
        &["the obligation `0 < (Data.Bytes.bytes-len b)`"],
    );
}
