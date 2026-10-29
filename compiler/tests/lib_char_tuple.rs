//! `Data.Char` consumer semantics: the ASCII character classes and the
//! ASCII case fold, observed through real programs.
//!
//! Covers every name this slice adds (`is-ascii-alpha`,
//! `is-ascii-alpha-numeric`, `is-ascii-space`, `is-ascii-upper`,
//! `is-ascii-lower`, `to-upper-ascii`, `to-lower-ascii`) at the
//! boundaries a caller hits: the class edges, the digit and non-ASCII
//! scalars that must NOT match, and the `to-lower (to-upper c)` round
//! trip on a letter.
//!
//! Each test compiles and runs the real prelude sources plus a `Main`
//! module through the same driver the CLI uses (`run_source`). Checks
//! are top-level `check-*` functions returning `Bool`; `main` sums their
//! bits so a nonzero exit code names the failed check (check k
//! contributes bit 2^(k-1)). The non-ASCII scalars are built with
//! `char-from-code` (there is no ASCII-only way to spell them);
//! `233` is LATIN SMALL LETTER E WITH ACUTE, `160` is NO-BREAK SPACE.

use cinch::driver::run_source;

/// The `Main` module header: the shared imports and the two non-ASCII
/// scalar helpers. A `case` in a `let` binder's right-hand side must
/// indent past the binder's column under the layout rules, so the
/// helpers are top-level.
const HEADER: &str = "module Main where\n  non-ascii-lower :: Char\n  non-ascii-lower = case char-from-code 233 of\n    Some c -> c\n    None -> '?'\n  non-ascii-nbsp :: Char\n  non-ascii-nbsp = case char-from-code 160 of\n    Some c -> c\n    None -> '?'\n";

fn main_module(body: &str) -> String {
    let mut src = String::from(HEADER);
    src.push_str(body);
    src.push('\n');
    src
}

/// Compile and run a `Main` body, asserting the exit code.
fn expect_ok(name: &str, body: &str, expected: i32) {
    match run_source(&main_module(body)) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// The class edges: `is-alpha` accepts an ASCII lowercase and uppercase
/// letter, refuses `0`, a space and a non-ASCII letter; the upper and
/// lower tests split the two case ranges and refuse the digit.
#[test]
fn ascii_alpha_upper_lower_boundaries() {
    expect_ok(
        "alpha-upper-lower",
        "  class-checks :: Bool\n\
         \x20 class-checks =\n\
         \x20   is-ascii-alpha 'a'\n\
         \x20   && is-ascii-alpha 'Z'\n\
         \x20   && (not (is-ascii-alpha '0'))\n\
         \x20   && (not (is-ascii-alpha ' '))\n\
         \x20   && (not (is-ascii-alpha non-ascii-lower))\n\
         \x20   && is-ascii-lower 'a'\n\
         \x20   && (not (is-ascii-lower 'A'))\n\
         \x20   && (not (is-ascii-lower '0'))\n\
         \x20   && (not (is-ascii-lower non-ascii-lower))\n\
         \x20   && is-ascii-upper 'Z'\n\
         \x20   && (not (is-ascii-upper 'z'))\n\
         \x20   && (not (is-ascii-upper '0'))\n\
         \x20 main :: Int\n\
         \x20 main = if class-checks then 0 else 1\n",
        0,
    );
}

/// `is-ascii-alpha-numeric` is exactly the letters plus the ASCII
/// digits; a space, a punctuation scalar and a non-ASCII letter are
/// refused. `is-ascii-space` accepts the ASCII whitespace scalars and
/// refuses a letter, a digit and NO-BREAK SPACE (non-ASCII).
#[test]
fn alpha_numeric_and_space_boundaries() {
    expect_ok(
        "alnum-space",
        "  num-checks :: Bool\n\
         \x20 num-checks =\n\
         \x20   is-ascii-alpha-numeric 'a'\n\
         \x20   && is-ascii-alpha-numeric 'Z'\n\
         \x20   && is-ascii-alpha-numeric '5'\n\
         \x20   && (not (is-ascii-alpha-numeric ' '))\n\
         \x20   && (not (is-ascii-alpha-numeric '-'))\n\
         \x20   && (not (is-ascii-alpha-numeric non-ascii-lower))\n\
         \x20 space-checks :: Bool\n\
         \x20 space-checks =\n\
         \x20   is-ascii-space ' '\n\
         \x20   && is-ascii-space '\\t'\n\
         \x20   && is-ascii-space '\\n'\n\
         \x20   && is-ascii-space '\\r'\n\
         \x20   && (not (is-ascii-space 'a'))\n\
         \x20   && (not (is-ascii-space '0'))\n\
         \x20   && (not (is-ascii-space non-ascii-nbsp))\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let c1 = if num-checks then 0 else 1 in\n\
         \x20   let c2 = if space-checks then 0 else 2 in\n\
         \x20   c1 + c2\n",
        0,
    );
}

/// The case folds: a letter maps to the opposite case, a digit and a
/// non-ASCII scalar and a punctuation scalar are unchanged, and the
/// boundaries `z`/`Z` move correctly.
#[test]
fn case_folds_on_letter_digit_and_non_ascii() {
    expect_ok(
        "case-folds",
        "  fold-checks :: Bool\n\
         \x20 fold-checks =\n\
         \x20   to-upper-ascii 'a' == 'A'\n\
         \x20   && to-lower-ascii 'A' == 'a'\n\
         \x20   && to-upper-ascii 'z' == 'Z'\n\
         \x20   && to-lower-ascii 'Z' == 'z'\n\
         \x20   && to-upper-ascii '0' == '0'\n\
         \x20   && to-lower-ascii '0' == '0'\n\
         \x20   && to-upper-ascii '}' == '}'\n\
         \x20   && to-upper-ascii non-ascii-lower == non-ascii-lower\n\
         \x20   && to-lower-ascii non-ascii-lower == non-ascii-lower\n\
         \x20 main :: Int\n\
         \x20 main = if fold-checks then 0 else 1\n",
        0,
    );
}

/// The two composed folds on a letter: `to-lower (to-upper c)` normalizes
/// to lower case (the identity on an originally-lowercase letter, the
/// lowercase form of an originally-uppercase one), and `to-upper
/// (to-lower c)` normalizes to upper case. Each fold's result is always
/// its own case, so the composite is not the identity on the opposite
/// case.
#[test]
fn case_fold_round_trip_on_letter() {
    expect_ok(
        "round-trip",
        "  roundtrip-checks :: Bool\n\
         \x20 roundtrip-checks =\n\
         \x20   to-lower-ascii (to-upper-ascii 'b') == 'b'\n\
         \x20   && to-lower-ascii (to-upper-ascii 'Q') == 'q'\n\
         \x20   && to-upper-ascii (to-lower-ascii 'm') == 'M'\n\
         \x20   && to-upper-ascii (to-lower-ascii 'X') == 'X'\n\
         \x20 main :: Int\n\
         \x20 main = if roundtrip-checks then 0 else 1\n",
        0,
    );
}
