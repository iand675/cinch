//! Guarded `letrec` binding groups (surface grammar local amendments
//! section 9): recursion is admitted under a lambda, a `~` memo body, or
//! a `delay`; an eager cycle is a compile error naming the variable and
//! the missing guard. Multi-binding `let` groups cover the sequential
//! side.

use cinch::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

#[test]
fn multi_binding_let_group() {
    expect_ok(
        "let-group",
        r#"
module Main where
  main :: Int
  main =
    let a = 3; b = a + 4; c = a * b in
    if c == 21 then 0 else 1
"#,
        0,
    );
}

#[test]
fn letrec_local_function_recurses() {
    expect_ok(
        "letrec-self",
        r#"
module Main where
  main :: Int
  main =
    letrec sumTo :: Nat -> Nat -> Nat; sumTo = \n acc -> if n == 0 then acc else sumTo (n - 1) (acc + n)
    in if sumTo 10 0 == 55 then 0 else 1
"#,
        0,
    );
}

#[test]
fn letrec_mutual_recursion() {
    expect_ok(
        "letrec-mutual",
        r#"
module Main where
  main :: Int
  main =
    letrec even :: Nat -> Bool; odd :: Nat -> Bool; even = \n -> if n == 0 then True else odd (n - 1); odd = \n -> if n == 0 then False else even (n - 1)
    in if even 10 then (if odd 7 then 0 else 1) else 1
"#,
        0,
    );
}

#[test]
fn letrec_group_of_functions_and_values() {
    expect_ok(
        "letrec-mixed",
        r#"
module Main where
  main :: Int
  main =
    letrec base :: Int; twice :: Int -> Int; base = 20; twice = \x -> x + x
    in if twice base == 40 then 0 else 1
"#,
        0,
    );
}

#[test]
fn eager_cycle_is_rejected_naming_the_variable() {
    expect_err(
        "letrec-eager-self",
        r#"
module Main where
  main :: Int
  main =
    letrec x = x + 1
    in x
"#,
        "unguarded recursion",
    );
    expect_err(
        "letrec-eager-mutual",
        r#"
module Main where
  main :: Int
  main =
    letrec x = y + 1; y = x + 1
    in x
"#,
        "unguarded recursion",
    );
}

#[test]
fn eager_call_cycle_is_rejected() {
    // The recursive use sits under a lambda, but the group member is
    // called eagerly from a sibling's binding: an eager cycle.
    expect_err(
        "letrec-eager-call",
        r#"
module Main where
  main :: Int
  main =
    letrec f = \n -> x; x = f 1
    in x
"#,
        "unguarded recursion",
    );
}

#[test]
fn guarded_delay_cycle_runs() {
    // The self-use sits under `delay`, one of the suspension positions.
    expect_ok(
        "letrec-delay",
        r#"
module Main where
  main :: Int
  main =
    letrec f :: Nat -> Nat; f = \n -> if n == 0 then 0 else force (delay (\() -> f (n - 1)))
    in if f 5 == 0 then 0 else 1
"#,
        0,
    );
}

#[test]
fn value_knot_read_under_a_lambda_yields_the_value() {
    // `r` is a value member whose own use sits under a lambda, so it
    // lives in a fixpoint box. Reading the box must return the stored
    // value (word 1 of `{ state; value }`), not the state word: reading
    // the state handed `case` the word 1 as if it were a block.
    expect_ok(
        "letrec-knot-read",
        r#"
module Main where
  main :: Int
  main =
    letrec r :: (Nat, Unit -> Nat); r = (41, \u -> case r of (a, g) -> a + 1)
    in case r of (a, g) -> if g () == 42 then 0 else 1
"#,
        0,
    );
}

#[test]
fn guarded_delay_value_cycle_is_accepted() {
    // A cyclic value through `delay`: the group checks (forcing the
    // cycle runs into the black-hole rule, by design).
    check_sources(
        r#"
module Main where
  main :: Int
  main =
    letrec x = 1 + force (delay (\() -> x))
    in 0
"#,
        "letrec-delay-value",
    )
    .unwrap_or_else(|e| panic!("letrec-delay-value: a `delay` cycle must check: {e}"));
}

#[test]
fn guarded_lambda_cycle_is_accepted() {
    check_sources(
        r#"
module Main where
  main :: Int
  main =
    letrec f :: Nat -> Nat; g :: Nat -> Nat; f = \n -> if n == 0 then 0 else g (n - 1); g = \n -> if n == 0 then 1 else f (n - 1)
    in if f 4 == 0 then 0 else 1
"#,
        "letrec-lambda",
    )
    .unwrap_or_else(|e| panic!("letrec-lambda: a lambda-guarded cycle must check: {e}"));
}

#[test]
fn nested_let_depth_limit_is_a_named_diagnostic() {
    // Nested `let` elaboration is bounded by the checker's let-nesting
    // depth limit: a nesting past it gets the named depth-limit
    // diagnostic, never a stack overflow and never a silent pass. The
    // limit is the hardening; these probes run on a wide stack (as the
    // JPL rule cases do) so what is tested is the limit, not the
    // thread's edge. The boundary is pinned: 128 nested `let`s still
    // check and run, 129 refuses.
    let deep = |levels: usize| {
        let mut body = format!("if a{levels} == {} then 0 else 1", levels - 1);
        for i in (1..=levels).rev() {
            let rhs =
                if i == 1 { "0".to_string() } else { format!("a{} + 1", i - 1) };
            body = format!("let a{i} = {rhs} in {body}");
        }
        format!("module Main where\n  main :: Int\n  main = {body}\n")
    };
    let at_limit = deep(128);
    let past_limit = deep(129);
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || {
            expect_ok("let-depth-at-limit", &at_limit, 0);
            let e = check_sources(&past_limit, "<test>").expect_err("past the limit");
            let msg = e.to_string();
            assert!(
                msg.contains("let nesting depth limit (128) exceeded"),
                "the depth-limit diagnostic names its limit: {msg}"
            );
        })
        .expect("spawn the let-depth probes")
        .join()
        .expect("the let-depth probes");
}
