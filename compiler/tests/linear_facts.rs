//! Linear facts over variables (`design/refinement-types.md`,
//! "narrowings and the implication theory"; `design/proof-automation.md`
//! section 4 item 2): a refinement obligation discharges from every fact
//! in scope together, read as difference constraints `x - y <= c` over
//! the integer-valued types and refuted with the obligation's negation
//! (a negative cycle). `i < n` gives `i + 1 <= n`, `n - i >= 1`, `0 < n`;
//! `i <= j` and `j < n` give `i < n`; a `Nat` is never negative; a `/=`
//! fact splits into its two strict cases. Every term denotes the value it
//! computes: `Nat` and `Int` arithmetic is exact, a machine integer is a
//! value in its range whose arithmetic (a user instance of some overflow
//! family) is opaque. A fact over a rebound name or a written place still
//! never participates.

use cinch::driver::{check_sources, run_source};

fn expect_checks(name: &str, src: &str) {
    if let Err(e) = check_sources(src, "<test>") {
        panic!("{name}: expected the program to check, but it failed: {e}");
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

fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Obligation-bearing functions the programs below call: a strict and a
/// non-strict index bound over two arguments, and a positive argument.
const DEFS: &str = r#"
  below :: using (i < n) => (i :: Nat) -> (n :: Nat) -> Nat
  below i n = n + i * 10

  upto :: using (i <= n) => (i :: Nat) -> (n :: Nat) -> Nat
  upto i n = n + i * 10

  positive :: using (0 < k) => (k :: Nat) -> Nat
  positive k = k
"#;

fn program(body: &str, main: &str) -> String {
    format!("\nmodule Main where\n{DEFS}\n{body}\n\n  main :: Int\n  main = {main}\n")
}

/// The refusal every undischarged obligation names.
const NO_EVIDENCE: &str = "needs `Dec` evidence or a branch fact";

// ------------------------------------------------------------ loops

#[test]
fn loop_index_advances_under_its_bound() {
    // The recursive call owes `i + 1 <= n`; the branch's `i < n` is the
    // only fact, and `i < n` over integers is `i + 1 <= n`.
    expect_run(
        "nat-loop",
        &program(
            r#"
  sum-from :: using (i <= n) => (i :: Nat) -> (n :: Nat) -> Nat -> Nat
  sum-from i n acc = if i < n then sum-from (i + 1) n (acc + i) else acc
"#,
            "if sum-from 0 10 0 == 45 && sum-from 3 3 7 == 7 then 0 else 1",
        ),
        0,
    );
    // The bound is a measure: `list-length xs` is one opaque `Nat` value
    // (a pure global applied to its argument), here in both the
    // obligation and the fact.
    expect_run(
        "measure-loop",
        &program(
            r#"
  at :: using (i < list-length xs) => (i :: Nat) -> (xs :: List Nat) -> Nat
  at i xs = i

  pairs :: Nat -> List Nat -> Nat
  pairs i xs = if i + 1 < list-length xs then at i xs + at (i + 1) xs else 0
"#,
            "if pairs 2 (Cons 5 (Cons 6 (Cons 7 (Cons 8 Nil)))) == 5 then 0 else 1",
        ),
        0,
    );
    // A polymorphic global's `Nat` result (`list-length :: List a ->
    // Nat`) is a fact term in a subtraction too.
    expect_run(
        "polymorphic-measure",
        &program(
            r#"
  room :: List Char -> Nat
  room cs = if list-length cs < 9 then positive (9 - list-length cs) else 0
"#,
            "if room (Cons 'a' (Cons 'b' Nil)) == 7 then 0 else 1",
        ),
        0,
    );
}

// ------------------------------------------------------ differences

#[test]
fn a_difference_under_a_strict_bound_is_positive() {
    // `0 < n - i` is `i - n <= -1`, which is `i < n` itself.
    expect_run(
        "difference-positive",
        &program(
            r#"
  remaining :: Nat -> Nat -> Nat
  remaining i n = if i < n then positive (n - i) else 0
"#,
            "if remaining 3 10 == 7 && remaining 10 3 == 0 then 0 else 1",
        ),
        0,
    );
    // The same fact makes `n - i` a `NonZero` divisor (its negation `n -
    // i == 0` contradicts `i < n`).
    expect_run(
        "difference-divisor",
        &program(
            r#"
  share :: Nat -> Nat -> Nat -> Nat
  share total i n = if i < n then nat-div total (n - i) else 0
"#,
            "if share 100 6 10 == 25 && share 100 10 10 == 0 then 0 else 1",
        ),
        0,
    );
    // A strict bound `i < n` also says `0 < n` (since `0 <= i`).
    expect_checks(
        "bound-positive",
        &program(
            r#"
  any-below :: Nat -> Nat -> Nat
  any-below i n = if i < n then positive n else 0
"#,
            "0",
        ),
    );
}

// --------------------------------------------------- chained bounds

#[test]
fn bounds_chain_through_other_variables() {
    // `i <= j` and `j < n` give `i < n`: `j` is in neither the obligation
    // nor the call, so the two facts are combined, not matched by name.
    expect_run(
        "nested-chain",
        &program(
            r#"
  chained :: Nat -> Nat -> Nat -> Nat
  chained i j n = if i <= j then (if j < n then below i n else 0) else 0
"#,
            "if chained 2 4 5 == 25 && chained 2 5 5 == 0 then 0 else 1",
        ),
        0,
    );
    // The same chain through `&&`, and a three-link chain with slack.
    expect_checks(
        "and-chain",
        &program(
            r#"
  chained :: Nat -> Nat -> Nat -> Nat
  chained i j n = if i <= j && j < n then below i n else 0

  three :: Nat -> Nat -> Nat -> Nat -> Nat
  three i j k n = if i < j && j < k && k <= n then upto (i + 2) n else 0
"#,
            "0",
        ),
    );
    // A `/=` fact splits into its two strict cases: with `i <= n`, `i /=
    // n` leaves only `i < n`.
    expect_run(
        "disequality-split",
        &program(
            r#"
  step :: Nat -> Nat -> Nat
  step i n = if i <= n then (if i == n then 0 else below i n) else 1
"#,
            "if step 2 5 == 25 && step 5 5 == 0 then 0 else 1",
        ),
        0,
    );
}

// ---------------------------------------------------- non-negativity

#[test]
fn nat_values_are_never_negative() {
    // No fact at all: `0 < i + 1` and `0 <= n` hold of every `Nat`.
    expect_run(
        "nat-nonnegative",
        &program(
            r#"
  succ-positive :: Nat -> Nat
  succ-positive i = positive (i + 1)

  from-zero :: Nat -> Nat
  from-zero n = upto 0 n
"#,
            "if succ-positive 0 == 1 && from-zero 4 == 4 then 0 else 1",
        ),
        0,
    );
    // An `Int` may be negative: the same shape over `Int` is refused.
    expect_err(
        "int-may-be-negative",
        r#"
module Main where
  positive-int :: using (0 < k) => (k :: Int) -> Int
  positive-int k = k

  succ-positive :: Int -> Int
  succ-positive i = positive-int (i + 1)

  main :: Int
  main = 0
"#,
        &[NO_EVIDENCE, "0 < (i + 1)"],
    );
}

// ---------------------------------------------------------- refusals

#[test]
fn off_by_one_is_refused() {
    // `i <= n` admits `i == n`, where `i < n` fails.
    expect_err(
        "off-by-one",
        &program(
            r#"
  edge :: Nat -> Nat -> Nat
  edge i n = if i <= n then below i n else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "i < n"],
    );
    // A `/=` fact alone leaves `i > n` open.
    expect_err(
        "disequality-alone",
        &program(
            r#"
  edge :: Nat -> Nat -> Nat
  edge i n = if i /= n then below i n else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "i < n"],
    );
}

#[test]
fn a_fact_over_another_variable_is_refused() {
    expect_err(
        "other-variable",
        &program(
            r#"
  other :: Nat -> Nat -> Nat -> Nat
  other i j n = if j < n then upto (i + 1) n else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "(i + 1) <= n"],
    );
    // A rebound name no longer denotes what the fact was about.
    expect_err(
        "rebound-name",
        &program(
            r#"
  rebound :: Nat -> Nat -> Nat
  rebound i n = if i < n then (let i = n in upto (i + 1) n) else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "(i + 1) <= n"],
    );
}

#[test]
fn a_stale_fact_after_a_write_is_refused() {
    // The fact `copyOut m < n` speaks about the cell's contents before
    // the write; the combined discharge cites it, so the write refuses
    // it exactly as a single-fact discharge would be refused.
    expect_err(
        "stale-after-write",
        &program(
            r#"
  probe :: forall region. Mut region Nat -> Nat -> Nat
  probe m n = if copyOut m < n then (let u = refSet m 9 in upto (copyOut m + 1) n) else 0
"#,
            "0",
        ),
        &["stale evidence", "invalidated by the write"],
    );
    // Before any write the fact is valid.
    expect_checks(
        "use-before-write",
        &program(
            r#"
  probe :: forall region. Mut region Nat -> Nat -> Nat
  probe m n = if copyOut m < n then upto (copyOut m + 1) n else 0
"#,
            "0",
        ),
    );
    // Re-tested after the write: the fresh fact is the one cited, not
    // the stale one of the same statement.
    expect_checks(
        "re-test-after-write",
        &program(
            r#"
  probe :: forall region. Mut region Nat -> Nat -> Nat
  probe m n = if copyOut m < n then (let u = refSet m 3 in if copyOut m < n then upto (copyOut m + 1) n else 0) else 0
"#,
            "0",
        ),
    );
}

// ------------------------------------------------- machine integers

/// `I64` with a user `Add` instance (the prelude gives machine integers
/// none): the instance wraps, so `i + 1` at `maxBound` is `minBound`.
const I64_DEFS: &str = r#"
  implicit instance AddI64 :: Add I64 where
    (+) x y = wrapping-add-i64 x y

  lt64 :: using (i < n) => (i :: I64) -> (n :: I64) -> I64
  lt64 i n = i

  le64 :: using (i <= n) => (i :: I64) -> (n :: I64) -> I64
  le64 i n = i

  under-max :: using (i < 9223372036854775807) => (i :: I64) -> I64
  under-max i = i
"#;

fn i64_program(body: &str, main: &str) -> String {
    format!("\nmodule Main where\n{I64_DEFS}\n{body}\n\n  main :: Int\n  main = {main}\n")
}

#[test]
fn machine_integer_arithmetic_never_implies_through_overflow() {
    // Over `Nat`, `i + 1 < n` gives `i < n`.
    expect_checks(
        "nat-successor-bound",
        &program(
            r#"
  shifted :: Nat -> Nat -> Nat
  shifted i n = if i + 1 < n then below i n else 0
"#,
            "0",
        ),
    );
    // Over `I64` it does not: at `i = maxBound` the instance's `i + 1`
    // wraps to `minBound`, below any `n`, while `i < n` fails.
    expect_err(
        "i64-successor-bound",
        &i64_program(
            r#"
  shifted :: I64 -> I64 -> I64
  shifted i n = if i + 1 < n then lt64 i n else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "i < n"],
    );
    // The edge is real: the condition holds at `maxBound` with `n = 0`.
    expect_run(
        "i64-wrap-edge",
        &i64_program(
            r#"
  wraps :: I64 -> I64 -> Bool
  wraps i n = i + 1 < n
"#,
            "if wraps 9223372036854775807 0 then 0 else 1",
        ),
        0,
    );
    // `i + 1 <= n` from `i < n` is refused too: the instance's `+` is
    // code the solver does not read.
    expect_err(
        "i64-successor-goal",
        &i64_program(
            r#"
  advance :: I64 -> I64 -> I64
  advance i n = if i < n then le64 (i + 1) n else 0
"#,
            "0",
        ),
        &[NO_EVIDENCE, "(i + 1) <= n"],
    );
    // Comparisons over machine values still chain, and the type's range
    // bounds a value: `i < n` with `n <= maxBound` gives `i < maxBound`.
    expect_run(
        "i64-chain-and-range",
        &i64_program(
            r#"
  chained :: I64 -> I64 -> I64 -> I64
  chained i j n = if i < j then (if j < n then lt64 i n else 0) else 0

  not-max :: I64 -> I64 -> I64
  not-max i n = if i < n then under-max i else 0
"#,
            "if chained 1 2 3 == 1 && not-max 4 5 == 4 && not-max 9223372036854775807 0 == 0 then 0 else 1",
        ),
        0,
    );
}
