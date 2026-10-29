//! `Data.Rational` consumer semantics: the exact rational over the
//! bignum `Int`.
//!
//! Every program is the real `compiler/prelude/Data/Rational.cinch`
//! source written beside a `Main.cinch` in a fresh temp project, then
//! compiled and run through the same driver the CLI uses. A check
//! returns 0 or its bit and `main` sums them, so a nonzero exit code is
//! a bitmask naming the failed check (check k contributes bit 2^(k-1)).
//! The checks are top-level functions, because a `case` in a `let`
//! binder's right-hand side must indent past the binder's column under
//! the layout rules.
//!
//! Coverage: the normalisation invariant and its idempotence, `Eq`
//! agreement with reduction, the sign-on-the-numerator rule, the
//! `add`/`mul` associativity and commutativity laws, the multiplicative
//! inverse for non-zero values, the cross-multiplication order and its
//! `Ord` operators, the `to-f64` boundary, and the two named refusals
//! (`ZeroDenominator`, `DivideByZero`) observed as their own
//! constructors.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const RATIONAL: &str = include_str!("../prelude/Data/Rational.cinch");

fn run_project(name: &str, module_src: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_rational_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Rational.cinch"), module_src).expect("write Rational");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn run_main(name: &str, main_src: &str) -> i32 {
    run_project(name, RATIONAL, main_src)
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

// The sample set shared by the programs: positive, negative, whole,
// zero, and non-reduced spellings, so every law is exercised across
// sign and reduction. `sample` cannot fail on these pairs (every
// denominator is non-zero); `check1` proves the fallback is unused.
const SAMPLES: &str = r#"
  sample :: Int -> Int -> Rational
  sample n d = case rational n d of
    Ok r -> r
    Err _ -> from-int 0

  samples :: List Rational
  samples = Cons (sample 1 2) (Cons (sample 2 4) (Cons (sample -1 2) (Cons (sample 1 -2) (Cons (sample 3 1) (Cons (sample 0 5) (Cons (sample -6 -4) Nil))))))

  all-samples :: (Rational -> Bool) -> List Rational -> Int
  all-samples p xs = case xs of
    Nil -> 0
    Cons r rest -> if p r then all-samples p rest else 1

  scale :: Int -> Int -> Int
  scale k f = if f == 0 then 0 else k
"#;

// ----------------------------------------------------- construction

#[test]
fn construction_reduction_and_sign() {
    expect_ok(
        "construct",
        &format!(
            "module Main where\n\n\
             \x20 import Data.Rational\n\
             \x20 import Data.Int\n\
             \x20 import Data.Machine\n\
             \x20 import Data.String\n\
             {SAMPLES}\n\
             \x20 pos-den :: Rational -> Bool\n\
             \x20 pos-den r = gt-i64 (int-cmp (denominator r) 0) 0\n\
             \n\
             \x20 check1 :: Int\n\
             \x20 check1 = scale 1 (all-samples pos-den samples)\n\
             \n\
             \x20 renorms :: Rational -> Bool\n\
             \x20 renorms r = case rational (numerator r) (denominator r) of\n\
             \x20   Err _ -> False\n\
             \x20   Ok r2 -> r2 == r\n\
             \n\
             \x20 check2 :: Int\n\
             \x20 check2 = scale 2 (all-samples renorms samples)\n\
             \n\
             \x20 check3 :: Int\n\
             \x20 check3 =\n\
             \x20   let a = sample 1 2 in\n\
             \x20   let b = sample 2 4 in\n\
             \x20   let c = sample -1 2 in\n\
             \x20   let d = sample 1 -2 in\n\
             \x20   let e = sample -6 -4 in\n\
             \x20   let f = sample 3 2 in\n\
             \x20   let z1 = sample 0 5 in\n\
             \x20   let z2 = from-int 0 in\n\
             \x20   if (a == b) && (c == d) && (e == f) && (z1 == z2) && (a /= sample 1 3) then 0 else 4\n\
             \n\
             \x20 check4 :: Int\n\
             \x20 check4 =\n\
             \x20   let r = sample -1 2 in\n\
             \x20   let z = sample 0 5 in\n\
             \x20   if eq-i64 (int-cmp (numerator r) (-1)) 0 then\n\
             \x20     if eq-i64 (int-cmp (denominator r) 2) 0 then\n\
             \x20       if eq-i64 (int-cmp (numerator z) 0) 0 then\n\
             \x20         if eq-i64 (int-cmp (denominator z) 1) 0 then 0 else 8\n\
             \x20       else 8\n\
             \x20     else 8\n\
             \x20   else 8\n\
             \n\
             \x20 check5 :: Int\n\
             \x20 check5 =\n\
             \x20   let a = from-int 5 in\n\
             \x20   let b = sample 5 1 in\n\
             \x20   let c = from-int (-7) in\n\
             \x20   if (a == b) && eq-i64 (int-cmp (denominator a) 1) 0 && eq-i64 (int-cmp (numerator c) (-7)) 0 then 0 else 16\n\
             \n\
             \x20 check6 :: Int\n\
             \x20 check6 =\n\
             \x20   if eq-i64 (string-compare (to-string (sample 2 4)) \"1/2\") 0 then\n\
             \x20     if eq-i64 (string-compare (to-string (sample -1 2)) \"-1/2\") 0 then\n\
             \x20       if eq-i64 (string-compare (to-string (sample 3 1)) \"3/1\") 0 then\n\
             \x20         if eq-i64 (string-compare (to-string (sample 0 5)) \"0/1\") 0 then 0 else 32\n\
             \x20       else 32\n\
             \x20     else 32\n\
             \x20   else 32\n\
             \n\
             \x20 selfeq :: Rational -> Bool\n\
             \x20 selfeq r = (r == r) && not (r /= r)\n\
             \n\
             \x20 check7 :: Int\n\
             \x20 check7 = scale 64 (all-samples selfeq samples)\n\
             \n\
             \x20 check8 :: Int\n\
             \x20 check8 =\n\
             \x20   let a = sample 10 4 in\n\
             \x20   let b = sample 5 2 in\n\
             \x20   let c = sample 6 -4 in\n\
             \x20   let d = sample -3 2 in\n\
             \x20   if (a == b) && (c == d) && eq-i64 (int-cmp (numerator a) 5) 0 && eq-i64 (int-cmp (denominator a) 2) 0 then 0 else 128\n\
             \n\
             \x20 main :: Int\n\
             \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
        ),
    );
}

// -------------------------------------------------------- arithmetic

const PAIRS: &str = r#"
  all-pairs :: (Rational -> Rational -> Bool) -> List Rational -> List Rational -> Int
  all-pairs holds xs ys = case xs of
    Nil -> 0
    Cons a rest -> case row-pairs holds a ys of
      0 -> all-pairs holds rest ys
      b -> b

  row-pairs :: (Rational -> Rational -> Bool) -> Rational -> List Rational -> Int
  row-pairs holds a ys = case ys of
    Nil -> 0
    Cons b rest -> if holds a b then row-pairs holds a rest else 1

  all-triples :: (Rational -> Rational -> Rational -> Bool) -> List Rational -> List Rational -> List Rational -> Int
  all-triples holds xs ys zs = case xs of
    Nil -> 0
    Cons a rest -> case mid-triples holds a ys zs of
      0 -> all-triples holds rest ys zs
      b -> b

  mid-triples :: (Rational -> Rational -> Rational -> Bool) -> Rational -> List Rational -> List Rational -> Int
  mid-triples holds a ys zs = case ys of
    Nil -> 0
    Cons b rest -> case last-triples holds a b zs of
      0 -> mid-triples holds a rest zs
      c -> c

  last-triples :: (Rational -> Rational -> Rational -> Bool) -> Rational -> Rational -> List Rational -> Int
  last-triples holds a b zs = case zs of
    Nil -> 0
    Cons c rest -> if holds a b c then last-triples holds a b rest else 1
"#;

#[test]
fn arithmetic_laws() {
    expect_ok(
        "arith",
        &format!(
            "module Main where\n\n\
             \x20 import Data.Rational\n\
             \x20 import Data.Int\n\
             {SAMPLES}\n\
             {PAIRS}\n\
             \x20 add-assoc :: Rational -> Rational -> Rational -> Bool\n\
             \x20 add-assoc x y z = add (add x y) z == add x (add y z)\n\
             \n\
             \x20 add-comm :: Rational -> Rational -> Bool\n\
             \x20 add-comm x y = add x y == add y x\n\
             \n\
             \x20 mul-assoc :: Rational -> Rational -> Rational -> Bool\n\
             \x20 mul-assoc x y z = mul (mul x y) z == mul x (mul y z)\n\
             \n\
             \x20 mul-comm :: Rational -> Rational -> Bool\n\
             \x20 mul-comm x y = mul x y == mul y x\n\
             \n\
             \x20 nonzero :: Rational -> Bool\n\
             \x20 nonzero r = ne-i64 (int-cmp (numerator r) 0) 0\n\
             \n\
             \x20 inv-one :: Rational -> Int\n\
             \x20 inv-one r = case div (from-int 1) r of\n\
             \x20   Err _ -> 1\n\
             \x20   Ok inv -> if mul r inv == from-int 1 then 0 else 1\n\
             \n\
             \x20 inv-law :: List Rational -> Int\n\
             \x20 inv-law xs = case xs of\n\
             \x20   Nil -> 0\n\
             \x20   Cons r rest ->\n\
             \x20     if nonzero r && inv-one r == 0 then inv-law rest\n\
             \x20     else if nonzero r then 1\n\
             \x20     else inv-law rest\n\
             \n\
             \x20 add-zero :: Rational -> Bool\n\
             \x20 add-zero r = add r (from-int 0) == r\n\
             \n\
             \x20 mul-one :: Rational -> Bool\n\
             \x20 mul-one r = mul r (from-int 1) == r\n\
             \n\
             \x20 sub-law :: Rational -> Rational -> Bool\n\
             \x20 sub-law x y = sub x y == add x (mul (from-int (-1)) y)\n\
             \n\
             \x20 check1 :: Int\n\
             \x20 check1 = scale 1 (all-triples add-assoc samples samples samples)\n\
             \n\
             \x20 check2 :: Int\n\
             \x20 check2 = scale 2 (all-pairs add-comm samples samples)\n\
             \n\
             \x20 check3 :: Int\n\
             \x20 check3 = scale 4 (all-triples mul-assoc samples samples samples)\n\
             \n\
             \x20 check4 :: Int\n\
             \x20 check4 = scale 8 (all-pairs mul-comm samples samples)\n\
             \n\
             \x20 check5 :: Int\n\
             \x20 check5 = scale 16 (inv-law samples)\n\
             \n\
             \x20 check6 :: Int\n\
             \x20 check6 = scale 32 (all-samples add-zero samples)\n\
             \n\
             \x20 check7 :: Int\n\
             \x20 check7 = scale 64 (all-samples mul-one samples)\n\
             \n\
             \x20 check8 :: Int\n\
             \x20 check8 = scale 128 (all-pairs sub-law samples samples)\n\
             \n\
             \x20 main :: Int\n\
             \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
        ),
    );
}

// ------------------------------------------- order, float, refusals

#[test]
fn order_float_and_named_refusals() {
    expect_ok("order", &order_main());
}

#[test]
fn refusal_bit_names_the_construction_failure() {
    // Non-vacuity: with the construction guard disabled, `rational 1 0`
    // builds a value instead of refusing, so the order program's check 7
    // (bit 64) fires and the exit code names itself. Every other check
    // is unaffected (the sample denominators are non-zero), so the code
    // is exactly that bit.
    let mutated = RATIONAL.replace(
        "if eq-i64 (int-cmp d 0) 0 then Err ZeroDenominator",
        "if False then Err ZeroDenominator",
    );
    assert_ne!(mutated, RATIONAL, "the mutation must apply to the module source");
    let code = run_project("refuse-bit", &mutated, &order_main());
    assert_eq!(
        code, 64,
        "the disabled construction guard must trip bit 64 only (got {code})"
    );
}

fn order_main() -> String {
    format!(
            "module Main where\n\n\
             \x20 import Data.Rational\n\
             \x20 import Data.Int\n\
             \x20 import Data.Machine\n\
             \x20 import Data.Ordering\n\
             {SAMPLES}\n\
             {PAIRS}\n\
             \x20 ord-tag :: Ordering -> Int\n\
             \x20 ord-tag o = case o of\n\
             \x20   LT -> -1\n\
             \x20   EQ -> 0\n\
             \x20   GT -> 1\n\
             \n\
             \x20 sign-of :: I64 -> Int\n\
             \x20 sign-of c = if lt-i64 c 0 then -1 else if gt-i64 c 0 then 1 else 0\n\
             \n\
             \x20 cmp-agrees :: Rational -> Rational -> Bool\n\
             \x20 cmp-agrees x y =\n\
             \x20   let l = int-mul (numerator x) (denominator y) in\n\
             \x20   let r = int-mul (numerator y) (denominator x) in\n\
             \x20   eq-i64 (int-cmp (ord-tag (rational-cmp x y)) (sign-of (int-cmp l r))) 0\n\
             \n\
             \x20 ord-ops :: Rational -> Rational -> Bool\n\
             \x20 ord-ops x y =\n\
             \x20   let t = ord-tag (rational-cmp x y) in\n\
             \x20   (x < y) == (eq-i64 (int-cmp t (-1)) 0) && (x <= y) == (le-i64 (int-cmp t 0) 0) && (x > y) == (eq-i64 (int-cmp t 1) 0) && (x >= y) == (ge-i64 (int-cmp t 0) 0)\n\
             \n\
             \x20 trans :: Rational -> Rational -> Rational -> Bool\n\
             \x20 trans x y z = if (x <= y) && (y <= z) then x <= z else True\n\
             \n\
             \x20 tol :: F64\n\
             \x20 tol = div-f64 (i64-to-f64 1) (i64-to-f64 1000000)\n\
             \n\
             \x20 close :: F64 -> F64 -> Bool\n\
             \x20 close a b = flt-f64 (abs-f64 (sub-f64 a b)) tol\n\
             \n\
             \x20 f64-scale :: Rational -> Bool\n\
             \x20 f64-scale r = feq-f64 (to-f64 (mul r (from-int (denominator r)))) (i64-to-f64 (int-to-i64 (numerator r)))\n\
             \n\
             \x20 f64-integral :: Bool\n\
             \x20 f64-integral = feq-f64 (to-f64 (from-int 7)) (i64-to-f64 7) && feq-f64 (to-f64 (from-int (-9))) (i64-to-f64 (-9))\n\
             \n\
             \x20 f64-add-coherent :: Rational -> Rational -> Bool\n\
             \x20 f64-add-coherent r s = close (to-f64 (add r s)) (add-f64 (to-f64 r) (to-f64 s))\n\
             \n\
             \x20 f64-mul-coherent :: Rational -> Rational -> Bool\n\
             \x20 f64-mul-coherent r s = close (to-f64 (mul r s)) (mul-f64 (to-f64 r) (to-f64 s))\n\
             \n\
             \x20 div-eq :: Rational -> Rational -> Rational -> Int\n\
             \x20 div-eq x y expected = case div x y of\n\
             \x20   Err _ -> 1\n\
             \x20   Ok q -> if q == expected then 0 else 1\n\
             \n\
             \x20 div-normal :: Int\n\
             \x20 div-normal =\n\
             \x20   let a = sample 1 2 in\n\
             \x20   let b = sample 3 4 in\n\
             \x20   let c = sample 2 3 in\n\
             \x20   let d = sample -3 4 in\n\
             \x20   let e = sample -2 3 in\n\
             \x20   div-eq a b c + div-eq a d e\n\
             \n\
             \x20 refusal-build :: Int\n\
             \x20 refusal-build = case rational 1 0 of\n\
             \x20   Ok _ -> 1\n\
             \x20   Err e -> case e of\n\
             \x20     ZeroDenominator -> 0\n\
             \x20     DivideByZero -> 1\n\
             \n\
             \x20 refusal-div :: Int\n\
             \x20 refusal-div = case div (sample 1 2) (from-int 0) of\n\
             \x20   Ok _ -> 1\n\
             \x20   Err e -> case e of\n\
             \x20     DivideByZero -> 0\n\
             \x20     ZeroDenominator -> 1\n\
             \n\
             \x20 check1 :: Int\n\
             \x20 check1 = scale 1 (all-pairs cmp-agrees samples samples)\n\
             \n\
             \x20 check2 :: Int\n\
             \x20 check2 = scale 2 (all-pairs ord-ops samples samples)\n\
             \n\
             \x20 check3 :: Int\n\
             \x20 check3 = scale 4 (all-triples trans samples samples samples)\n\
             \n\
             \x20 check4 :: Int\n\
             \x20 check4 = scale 8 (if all-samples f64-scale samples == 0 && f64-integral then 0 else 1)\n\
             \n\
             \x20 check5 :: Int\n\
             \x20 check5 = scale 16 (if all-pairs f64-add-coherent samples samples == 0 && all-pairs f64-mul-coherent samples samples == 0 then 0 else 1)\n\
             \n\
             \x20 check6 :: Int\n\
             \x20 check6 = scale 32 (if div-normal == 0 then 0 else 1)\n\
             \n\
             \x20 check7 :: Int\n\
             \x20 check7 = scale 64 refusal-build\n\
             \n\
             \x20 check8 :: Int\n\
             \x20 check8 = scale 128 refusal-div\n\
             \n\
             \x20 main :: Int\n\
             \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
    )
}
