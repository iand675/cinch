//! Data.Complex consumer semantics: the field operations over `F64`.
//!
//! The real `compiler/prelude/Data/Complex.cinch` source sits beside a
//! `Main` module in a temp project, compiled and run through the same
//! driver the CLI uses. Every check is a consumer-visible identity or
//! boundary case: a top-level `c*` value returns 0 or its bit, and `main`
//! sums them, so a nonzero exit code is a bitmask naming the failed check
//! (bit 2^(k-1) is `c<k>`):
//!   bit 1    the imaginary unit squares to -1, and `from-real` crosses in
//!   bit 2    the Ruby sample product (1+2i)(3+4i) == -5+10i, through the
//!            `Mul`/`Sub`/`Neg` operator instances
//!   bit 4    distributivity of `mul` over `add`
//!   bit 8    the Pythagorean magnitude |3+4i| == 5 within tolerance
//!   bit 16   conjugate involution
//!   bit 32   `div` inverts `mul` within tolerance
//!   bit 64   the `a+bi` string form, including the negative imaginary case
//!   bit 128  the `arg` approximation (the axis cases and pi/4)
//!
//! `Data.Complex` is not embedded in the prelude, so the test writes its
//! source beside the entry file and imports it explicitly (the facade
//! integration is the lead's step).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const COMPLEX: &str = include_str!("../prelude/Data/Complex.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_complex_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Complex.cinch"), COMPLEX).expect("write Complex");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The `Main` module: shared helpers (an F64 tolerance, pointwise
/// closeness for reals and for complex numbers) then the program body.
fn main_module(body: &str) -> String {
    let mut src = String::from(
        r#"module Main where

  import Data.Complex

  tol :: F64
  tol = f64-from-decimal (string-to-bytes "1e-9")

  quarter-pi :: F64
  quarter-pi = f64-from-decimal (string-to-bytes "0.7853981633974483")

  half-pi :: F64
  half-pi = f64-from-decimal (string-to-bytes "1.5707963267948966")

  pi-const :: F64
  pi-const = f64-from-decimal (string-to-bytes "3.141592653589793")

  close :: F64 -> F64 -> Bool
  close a b = flt-f64 (abs-f64 (sub-f64 a b)) tol

  close-c :: Complex -> Complex -> Bool
  close-c x y = close (real-part x) (real-part y) && close (imag-part x) (imag-part y)

"#,
    );
    src.push_str(body);
    src.push('\n');
    src
}

/// The field identities and the boundary cases, one device of the module.
#[test]
fn complex_field_identities_and_boundaries() {
    expect_ok(
        "complex",
        &main_module(
            r#"  main :: Int
  main =
    let x = complex 1 2 in
    let y = complex 3 4 in
    let z = complex 5 -6 in
    let i-sq = mul (complex 0 1) (complex 0 1) in
    let prod = (complex 1 2) * (complex 3 4) in
    let quo = div prod (complex 3 4) in
    let c1 = if i-sq == complex -1 0 && from-real 2 == complex 2 0 then 0 else 1 in
    let c2 = if prod == complex -5 10 && negate y == complex -3 -4 && (x - y) == complex -2 -2 then 0 else 2 in
    let c3 = if mul x (add y z) == add (mul x y) (mul x z) then 0 else 4 in
    let c4 = if close (abs (complex 3 4)) 5 then 0 else 8 in
    let c5 = if conjugate (conjugate (complex 2 -3)) == complex 2 -3 then 0 else 16 in
    let c6 = if close-c quo x then 0 else 32 in
    let c7 = if to-string (complex 1 -1) == "1-1i" then (if to-string (complex 3 4) == "3+4i" then 0 else 64) else 64 in
    let c8 = if close (arg (complex 1 1)) quarter-pi && close (arg (complex 0 1)) half-pi && close (arg (complex -1 0)) pi-const && close (arg (complex 3 0)) 0 then 0 else 128 in
    if c1 == 0 then (if c2 == 0 then (if c3 == 0 then (if c4 == 0 then (if c5 == 0 then (if c6 == 0 then (if c7 == 0 then (if c8 == 0 then 0 else 128) else 64) else 32) else 16) else 8) else 4) else 2) else 1
"#,
        ),
    );
}
