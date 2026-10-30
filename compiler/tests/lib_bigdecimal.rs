//! BigDecimal consumer semantics: the exact decimal over the bignum
//! `Int`, its canonical form, and its named rounding modes.
//!
//! Every test compiles a real project: the prelude module under test
//! (`Data/BigDecimal.cinch`, written beside `Main.cinch` because the
//! family is not embedded in the prelude) plus a `Main` module, driven
//! through the CLI pipeline exactly as lib_foldable.rs does. The checks
//! assert consumer-visible semantics only:
//!   exactness: `add`/`mul` over 0.1/0.2/0.3 land on the decimal digits
//!     (unscaled 3 for `0.1 + 0.2`, never the binary float's
//!     0.30000000000000004), the `0.1 + 0.2 == 0.3` law holds under the
//!     `Eq` instance, and a 30-digit product is exact.
//!   canonical form: `0.1 == 0.10` (equal fields), trailing zeros
//!     stripped through the units, zero spelled `unscaled 0` with
//!     exponent 0, and the sign laws (`neg`/`abs`, `x + -x` landing on
//!     canonical zero).
//!   rounding: every named mode at the units place and at a fractional
//!     precision, including the half-way cases in both directions (2.5
//!     and -2.5 under HALF_UP, HALF_DOWN, HALF_EVEN) and the carry out
//!     of the digit run (9.999 at 2 digits is 10).
//!   division: the precision and the mode are the caller's (1/3 at 10
//!     digits half-up is 0.3333333333), bignum quotients survive, and a
//!     zero divisor is `None` rather than an infinity (division is total
//!     by construction).
//!   order: `cmp` is the sign of `sub` over all pairs of the sample set,
//!     antisymmetric, and the `Eq`/`Ord` instances are that same
//!     relation.
//!   text: `from-string (to-string x)` has the same value for every
//!     sample literal, the rendering is the plain decimal (never
//!     exponent notation), and a malformed literal is `MalformedLiteral`
//!     carrying its text, never a silent zero.
//!   to-f64: the correctly rounded conversion of the exact decimal
//!     (`0.1 + 0.2` converts to the same double as 3/10, which the naive
//!     binary sum does not).
//!
//! Each check is a top-level `cN` binding contributing 2^(n-1) to
//! `main`'s sum, so a nonzero exit code is a bitmask naming the failed
//! checks, and every failure mode of one check shares its bit.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const BIGDECIMAL: &str = include_str!("../prelude/Data/BigDecimal.cinch");

/// The `Main` module's shared prologue: the module under test, the
/// machine-scalar helpers `to-f64`'s checks need, the `Result`
/// unwrapper, and the comparison helpers the order checks run over the
/// sample set.
const PREFIX: &str = r#"module Main where
  import Data.BigDecimal
  import Data.Machine

  unwrap :: Result BigDecimalError BigDecimal -> BigDecimal
  unwrap r = case r of
    Ok x -> x
    Err _ -> from-int 0

  lit :: String -> BigDecimal
  lit s = unwrap (from-string s)

  ord-tag :: Ordering -> Int
  ord-tag o = case o of
    LT -> 0
    EQ -> 1
    GT -> 2

  render-of :: String -> String
  render-of s = case from-string s of
    Err e -> render-big-decimal-error e
    Ok _ -> "no-error"

  refused :: String -> Bool
  refused s = case from-string s of
    Err _ -> True
    Ok _ -> False

  refused-all :: List String -> Bool
  refused-all ss = case ss of
    Nil -> True
    Cons s rest -> if refused s then refused-all rest else False

  round-trips :: List String -> Bool
  round-trips ss = case ss of
    Nil -> True
    Cons s rest -> if lit (to-string (lit s)) == lit s then round-trips rest else False

  sign-of :: BigDecimal -> Int
  sign-of x = if unscaled x < 0 then 0 else if unscaled x == 0 then 1 else 2

  sign-ord :: Int -> Ordering
  sign-ord n = if n == 0 then LT else if n == 1 then EQ else GT

  agree-sign :: BigDecimal -> BigDecimal -> Bool
  agree-sign x y = ord-tag (cmp x y) == ord-tag (sign-ord (sign-of (sub x y)))

  antisym :: BigDecimal -> BigDecimal -> Bool
  antisym x y = ord-tag (cmp x y) + ord-tag (cmp y x) == 2

  lt-ok :: BigDecimal -> BigDecimal -> Bool
  lt-ok x y = (x < y) == (ord-tag (cmp x y) == 0)

  le-ok :: BigDecimal -> BigDecimal -> Bool
  le-ok x y = (x <= y) == (ord-tag (cmp x y) <= 1)

  gt-ok :: BigDecimal -> BigDecimal -> Bool
  gt-ok x y = (x > y) == (ord-tag (cmp x y) == 2)

  ge-ok :: BigDecimal -> BigDecimal -> Bool
  ge-ok x y = (x >= y) == (ord-tag (cmp x y) >= 1)

  eq-ok :: BigDecimal -> BigDecimal -> Bool
  eq-ok x y = (x == y) == (ord-tag (cmp x y) == 1)

  rows :: (BigDecimal -> BigDecimal -> Bool) -> List BigDecimal -> List BigDecimal -> Bool
  rows p xs ys = case xs of
    Nil -> True
    Cons x rest -> if row p x ys then rows p rest ys else False

  row :: (BigDecimal -> BigDecimal -> Bool) -> BigDecimal -> List BigDecimal -> Bool
  row p x ys = case ys of
    Nil -> True
    Cons y rest -> if p x y then row p x rest else False

  ascending :: List BigDecimal -> Bool
  ascending xs = case xs of
    Nil -> True
    Cons x rest -> case rest of
      Nil -> True
      Cons y more -> if ord-tag (cmp x y) == 0 then ascending rest else False

  samples :: List BigDecimal
  samples = Cons (lit "-2.5") (Cons (lit "-1") (Cons (lit "-0.5") (Cons (lit "0") (Cons (lit "0.1") (Cons (lit "0.10") (Cons (lit "1") (Cons (lit "2.5") (Cons (lit "1000000000000000000000") Nil))))))))

  ordered :: List BigDecimal
  ordered = Cons (lit "-2.5") (Cons (lit "-1") (Cons (lit "0") (Cons (lit "0.1") (Cons (lit "2.5") (Cons (from-int 1000000000000000000000) Nil)))))"#;

fn program(body: &str) -> String {
    format!("{PREFIX}\n{body}\n")
}

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_bdtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("BigDecimal.cinch", BIGDECIMAL), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    // The checker elaborates the whole embedded family for every program,
    // so its recursion depth is a function of the family's size, not of
    // this program: once the family grew past the harness default the pool
    // threads aborted here with no diagnostic (every test failed at once
    // and none of them panicked), while the same tests passed serially on
    // the main thread. Compile on a thread with a generous stack so the
    // slice does not depend on the harness default (the reason
    // lib_base64.rs does the same).
    let entry = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None))
        .expect("spawn compiler thread")
        .join()
        .expect("compiler thread panicked");
    match compiled {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// One check bitmask that must come out zero.
fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, &program(body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// `0.1` and `0.2` are not binary floats, so the exactness of `add` and
/// `mul` is visible in the fields: the sum is unscaled 3 at exponent -1
/// (3 * 10^-1), not the double sum's 0.30000000000000004, and the
/// 30-digit product of a long decimal by 2 is exact.
#[test]
fn exact_add_and_mul_over_values_binary_floats_cannot_represent() {
    expect_ok(
        "exact-add-mul",
        r#"  main :: Int
  main =
    let tenth = lit "0.1" in
    let two-tenths = lit "0.2" in
    let three-tenths = lit "0.3" in
    let sum = add tenth two-tenths in
    let product = mul tenth three-tenths in
    let c1 = if unscaled sum == 3 then 0 else 1 in
    let c2 = if exponent sum == -1 then 0 else 2 in
    let c3 = if to-string sum == "0.3" then 0 else 4 in
    let c4 = if sum == three-tenths then 0 else 8 in
    let c5 = if to-string product == "0.03" then 0 else 16 in
    let c6 = if unscaled product == 3 then 0 else 32 in
    let c7 = if to-string (add (lit "0.7") tenth) == "0.8" then 0 else 64 in
    let c8 = if to-string (mul (lit "123456789012345678901234567890.5") (from-int 2)) == "246913578024691357802469135781" then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The canonical form is the equality law: `0.1` and `0.10` have the same
/// fields, trailing zeros strip through the units, zero is `unscaled 0`
/// with exponent 0 however it was built, and the sign laws (`neg`/`abs`,
/// `x + -x`) land on that canonical zero.
#[test]
fn normalisation_makes_equal_values_equal() {
    expect_ok(
        "normalisation",
        r#"  main :: Int
  main =
    let a = lit "0.1" in
    let b = lit "0.10" in
    let x = lit "2.50" in
    let zero-difference = add x (neg x) in
    let c1 = if a == b then 0 else 1 in
    let c2 = if unscaled b == 1 then (if exponent b == -1 then 0 else 2) else 2 in
    let c3 = if to-string (big-decimal 100 (-2)) == "1" then 0 else 4 in
    let c4 = if unscaled (big-decimal 0 (-3)) == 0 then (if exponent (big-decimal 0 (-3)) == 0 then 0 else 8) else 8 in
    let c5 = if to-string x == "2.5" then 0 else 16 in
    let c6 = if to-string (neg x) == "-2.5" then (if abs (neg x) == x then (if abs (lit "-2.5") == x then 0 else 32) else 32) else 32 in
    let c7 = if unscaled zero-difference == 0 then (if exponent zero-difference == 0 then 0 else 64) else 64 in
    let c8 = if to-string (lit "0.000") == "0" then (if exponent (lit "-0") == 0 then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// Every named mode with 0 fractional digits, over both signs: the
/// half-way cases break away from zero (HALF_UP), toward zero (HALF_DOWN)
/// and to the even magnitude (HALF_EVEN), the directed modes read as their
/// names say on each side of zero, an exact value is unchanged, and a
/// half-way case carries out of the digit run (9.5 half-up is 10).
#[test]
fn rounding_modes_at_the_units_place() {
    expect_ok(
        "round-units",
        r#"  main :: Int
  main =
    let c1 = if to-string (round 0 RoundHalfUp (lit "2.5")) == "3" then (if to-string (round 0 RoundHalfUp (lit "-2.5")) == "-3" then 0 else 1) else 1 in
    let c2 = if to-string (round 0 RoundHalfDown (lit "2.5")) == "2" then (if to-string (round 0 RoundHalfDown (lit "-2.5")) == "-2" then 0 else 2) else 2 in
    let c3 = if to-string (round 0 RoundHalfEven (lit "2.5")) == "2" then (if to-string (round 0 RoundHalfEven (lit "3.5")) == "4" then (if to-string (round 0 RoundHalfEven (lit "-2.5")) == "-2" then 0 else 4) else 4) else 4 in
    let c4 = if to-string (round 0 RoundDown (lit "2.7")) == "2" then (if to-string (round 0 RoundDown (lit "-2.7")) == "-2" then (if to-string (round 0 RoundDown (lit "2.0")) == "2" then 0 else 8) else 8) else 8 in
    let c5 = if to-string (round 0 RoundUp (lit "2.1")) == "3" then (if to-string (round 0 RoundUp (lit "-2.1")) == "-3" then (if to-string (round 0 RoundUp (lit "2.0")) == "2" then 0 else 16) else 16) else 16 in
    let c6 = if to-string (round 0 RoundCeiling (lit "2.1")) == "3" then (if to-string (round 0 RoundCeiling (lit "-2.1")) == "-2" then (if to-string (round 0 RoundCeiling (lit "-2.0")) == "-2" then 0 else 32) else 32) else 32 in
    let c7 = if to-string (round 0 RoundFloor (lit "2.9")) == "2" then (if to-string (round 0 RoundFloor (lit "-2.1")) == "-3" then (if to-string (round 0 RoundFloor (lit "2.0")) == "2" then 0 else 64) else 64) else 64 in
    let c8 = if to-string (round 0 RoundHalfUp (lit "9.5")) == "10" then (if to-string (round 0 RoundHalfDown (lit "9.5")) == "9" then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The same modes with fractional digits: `1.25` at one digit half-up is
/// `1.3` and half-even is `1.2`, ties break on the magnitude so the sign
/// does not enter, and rounding an already exact value invents no digits.
#[test]
fn rounding_modes_at_fractional_precision() {
    expect_ok(
        "round-precision",
        r#"  main :: Int
  main =
    let c1 = if to-string (round 1 RoundHalfUp (lit "1.25")) == "1.3" then 0 else 1 in
    let c2 = if to-string (round 1 RoundHalfEven (lit "1.25")) == "1.2" then 0 else 2 in
    let c3 = if to-string (round 1 RoundHalfUp (lit "-1.25")) == "-1.3" then 0 else 4 in
    let c4 = if to-string (round 1 RoundHalfEven (lit "1.35")) == "1.4" then 0 else 8 in
    let c5 = if to-string (round 1 RoundDown (lit "1.29")) == "1.2" then (if to-string (round 1 RoundUp (lit "1.29")) == "1.3" then 0 else 16) else 16 in
    let c6 = if to-string (round 3 RoundUp (lit "1.1005")) == "1.101" then (if to-string (round 3 RoundUp (lit "1.1")) == "1.1" then 0 else 32) else 32 in
    let c7 = if to-string (round 2 RoundFloor (lit "-1.001")) == "-1.01" then (if to-string (round 2 RoundCeiling (lit "-1.001")) == "-1" then 0 else 64) else 64 in
    let c8 = if to-string (round 2 RoundHalfEven (lit "1.005")) == "1" then (if to-string (round 1 RoundHalfEven (lit "0.05")) == "0" then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// `div` names its precision and its mode: 1/3 at 10 digits half-up is the
/// brief's `0.3333333333`, 2/3 rounds up at the same precision, 1/7 at 30
/// digits is exact to the digit, a 20-digit quotient is a bignum quotient
/// (no float could hold it), the rounded result is normalised (`div 5
/// RoundDown 1 2` is `0.5`, not `0.50000`), and rounding carries into the
/// units.
#[test]
fn division_takes_an_explicit_precision_and_carries() {
    expect_ok(
        "division",
        r#"  q :: Option BigDecimal -> String
  q o = case o of
    Some x -> to-string x
    None -> "none"

  main :: Int
  main =
    let c1 = if q (div 10 RoundHalfUp (from-int 1) (from-int 3)) == "0.3333333333" then 0 else 1 in
    let c2 = if q (div 10 RoundHalfEven (from-int 2) (from-int 3)) == "0.6666666667" then 0 else 2 in
    let c3 = if q (div 30 RoundHalfUp (from-int 1) (from-int 7)) == "0.142857142857142857142857142857" then 0 else 4 in
    let c4 = if q (div 0 RoundHalfUp (from-int 10000000000000000000) (from-int 3)) == "3333333333333333333" then 0 else 8 in
    let c5 = if q (div 2 RoundHalfUp (from-int 1) (from-int 8)) == "0.13" then (if q (div 2 RoundHalfEven (from-int 1) (from-int 8)) == "0.12" then 0 else 16) else 16 in
    let c6 = if q (div 5 RoundDown (from-int 1) (from-int 2)) == "0.5" then 0 else 32 in
    let c7 = if to-string (round 2 RoundHalfUp (lit "9.999")) == "10" then (if to-string (round 2 RoundHalfUp (lit "-9.999")) == "-10" then 0 else 64) else 64 in
    let c8 = if q (div 0 RoundUp (from-int 1) (lit "0.001")) == "1000" then (if q (div 1 RoundUp (from-int 1) (from-int 3)) == "0.4" then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The order `sub` induces, over every ordered pair of a 9-value sample
/// set: `cmp`'s three-way result matches the sign of `sub`, `cmp` is
/// antisymmetric, equal values in different spellings compare EQ, the
/// sample set sorts ascending (so the order is numeric, not lexicographic
/// on the fields), and the `Eq`/`Ord` instances are the same relation as
/// the function.
#[test]
fn cmp_agrees_with_the_sign_of_subtraction() {
    expect_ok(
        "cmp-sign",
        r#"  main :: Int
  main =
    let c1 = if rows agree-sign samples samples then 0 else 1 in
    let c2 = if rows antisym samples samples then 0 else 2 in
    let c3 = if ord-tag (cmp (lit "0.1") (lit "0.10")) == 1 then (if ord-tag (cmp (lit "0") (lit "-0")) == 1 then (if ord-tag (cmp (big-decimal 1 3) (from-int 1000)) == 1 then 0 else 4) else 4) else 4 in
    let c4 = if rows lt-ok samples samples then (if rows le-ok samples samples then (if rows gt-ok samples samples then (if rows ge-ok samples samples then 0 else 8) else 8) else 8) else 8 in
    let c5 = if rows eq-ok samples samples then 0 else 16 in
    let c6 = if ascending ordered then 0 else 32 in
    let c7 = if ord-tag (cmp (lit "0.2") (lit "0.10")) == 2 then (if ord-tag (cmp (lit "-0.0001") (from-int 0)) == 0 then 0 else 64) else 64 in
    let c8 = if unscaled (sub (lit "2.5") (lit "2.4")) == 1 then (if exponent (sub (lit "2.5") (lit "2.4")) == -1 then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// `from-string (to-string x)` is `x` for every sample literal (including
/// a 22-digit one and a leading-point literal), and the rendering is the
/// plain decimal: no exponent notation for a positive exponent, a leading
/// `0.` when the fraction is wider than the digit run, and normalised
/// spellings (`.5` is `0.5`, `-0.10` is `-0.1`, `10.0` is `10`).
#[test]
fn text_round_trip_and_plain_decimal_rendering() {
    expect_ok(
        "text",
        r#"  literals :: List String
  literals = Cons "0" (Cons "0.1" (Cons "-0.1" (Cons "123" (Cons "-123.456" (Cons "0.0000001" (Cons "1000000000000000000000.000001" (Cons "-0.10" (Cons ".5" (Cons "10.0" (Nil))))))))))

  main :: Int
  main =
    let c1 = if round-trips literals then 0 else 1 in
    let c2 = if to-string (lit "10.0") == "10" then 0 else 2 in
    let c3 = if to-string (lit ".5") == "0.5" then 0 else 4 in
    let c4 = if to-string (lit "-0.10") == "-0.1" then 0 else 8 in
    let c5 = if to-string (from-int 12345) == "12345" then (if to-string (from-int 0) == "0" then 0 else 16) else 16 in
    let c6 = if to-string (big-decimal 1 3) == "1000" then 0 else 32 in
    let c7 = if to-string (big-decimal 1 21) == "1000000000000000000000" then 0 else 64 in
    let c8 = if to-string (lit "0.0000000001") == "0.0000000001" then (if to-string (big-decimal 25 (-3)) == "0.025" then 0 else 128) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The refusals: the empty string, a lone sign, a lone or trailing point,
/// an exponent part, trailing junk, embedded whitespace, repeated signs,
/// and Ruby's `NaN`/`Infinity` spellings all come back as `Err`, so no
/// malformed literal is silently zero. The reader is not vacuous (leading
/// zeros and signed zeros read as their values), and the error names the
/// offending text in its rendering.
#[test]
fn malformed_literals_are_named_errors_never_a_silent_zero() {
    expect_ok(
        "malformed",
        r#"  main :: Int
  main =
    let signs = Cons "" (Cons "+" (Cons "-" Nil)) in
    let points = Cons "." (Cons "1." (Cons "+." Nil)) in
    let exponents = Cons "e10" (Cons "1e10" (Cons "1E5" Nil)) in
    let junk = Cons "1.5x" (Cons "x" (Cons "1 2" (Cons " 1" (Cons "1 " Nil)))) in
    let structure = Cons "1.2.3" (Cons "--1" (Cons "1-" (Cons "1,5" (Cons "++1" Nil)))) in
    let specials = Cons "NaN" (Cons "Infinity" (Cons "-Infinity" Nil)) in
    let c1 = if refused-all signs then 0 else 1 in
    let c2 = if refused-all points then 0 else 2 in
    let c3 = if refused-all exponents then 0 else 4 in
    let c4 = if refused-all junk then 0 else 8 in
    let c5 = if refused-all structure then 0 else 16 in
    let c6 = if refused-all specials then 0 else 32 in
    let good = Cons "007" (Cons "0" (Cons "+0.0" (Cons "-0" Nil))) in
    let c7 = if to-string (lit "007") == "7" then (if lit "-0" == from-int 0 then (if lit "+0.0" == from-int 0 then 0 else 64) else 64) else 64 in
    let c8 = if render-of "1x" == "malformed-literal: 1x is not a decimal literal (the accepted form is [+-]? digits? ( . digits )?)" then (if render-of "NaN" == render-of "1x" then 128 else 0) else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The one conversion out of the decimal world: exact decimals that are
/// binary-representable (`0.5`, `0.125`, -0.25) convert exactly, `0.1 +
/// 0.2` converts to the same double as 3/10, and the naive binary sum does
/// not (that difference is why the family exists).
#[test]
fn to_f64_is_the_correctly_rounded_conversion() {
    expect_ok(
        "to-f64",
        r#"  main :: Int
  main =
    let half = div-f64 (i64-to-f64 1) (i64-to-f64 2) in
    let eighth = div-f64 (i64-to-f64 1) (i64-to-f64 8) in
    let three-tenths = div-f64 (i64-to-f64 3) (i64-to-f64 10) in
    let tenth = div-f64 (i64-to-f64 1) (i64-to-f64 10) in
    let two-tenths = div-f64 (i64-to-f64 2) (i64-to-f64 10) in
    let quarter = div-f64 (i64-to-f64 1) (i64-to-f64 4) in
    let billion = i64-to-f64 10000000000 in
    let c1 = if feq-f64 (to-f64 (lit "0.5")) half then 0 else 1 in
    let c2 = if feq-f64 (to-f64 (lit "0.125")) eighth then 0 else 2 in
    let c3 = if feq-f64 (to-f64 (add (lit "0.1") (lit "0.2"))) three-tenths then 0 else 4 in
    let c4 = if feq-f64 (add-f64 tenth two-tenths) three-tenths then 8 else 0 in
    let c5 = if feq-f64 (to-f64 (lit "-0.25")) (neg-f64 quarter) then 0 else 16 in
    let c6 = if feq-f64 (to-f64 (from-int 1000000000000000000000)) (mul-f64 (i64-to-f64 1000000000000000000) (i64-to-f64 1000)) then 0 else 32 in
    let c7 = if feq-f64 (to-f64 (from-int 0)) (i64-to-f64 0) then 0 else 64 in
    let c8 = if feq-f64 (to-f64 (lit "0.0000000001")) (div-f64 (i64-to-f64 1) billion) then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8"#,
    );
}

/// The named refusal behind the missing `Infinity`: a zero divisor has no
/// quotient, so `div` answers `None` (division is total by construction).
#[test]
fn division_by_zero_is_none() {
    expect_ok(
        "division-by-zero",
        r#"  main :: Int
  main = case div 2 RoundHalfUp (from-int 1) (from-int 0) of
    None -> 0
    Some _ -> 1"#,
    );
}
