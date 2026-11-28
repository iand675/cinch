//! Constructed Sum Results (CSR): 2-variant 0/1-word uniform sums (`Option`,
//! `Result`, user sums) returned unboxed in `{ i64, i64 }` registers with zero
//! heap allocation on direct `case`-scrutinized calls and exact Perceus RC.
//!
//! Routed through `common::build_executable` so the same suite runs against
//! both the Rust seed and the Stage 3 self-hosted compiler.

mod common;
use std::process::Command;

fn run_leak_checked(tag: &str, source: &str) -> String {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build_executable(&dir, "Main.cinch", "csr-test")
        .unwrap_or_else(|e| panic!("{tag}: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("run the built program");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: every block must be released exactly once; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
    stderr
}

#[test]
fn result_chains_with_counted_payloads_and_wildcards_are_leak_free() {
    run_leak_checked(
        "csr-result-chain",
        r#"module Main where
  import Algebra.Semigroup
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)

  step-int :: Int -> Result String Int
  step-int n =
    if n == 0 then Err ("zero" <> "!")
    else if n < 0 then Err ("neg" <> "!")
    else Ok (n + 1)

  pipeline :: Int -> Result String Int
  pipeline x = case step-int x of
    Err e -> Err e
    Ok a -> case step-int a of
      Err e -> Err e
      Ok b -> step-int b

  ignore-err :: Int -> Int
  ignore-err x = case pipeline x of
    Ok v -> v
    _ -> 100

  err-len :: Int -> Int
  err-len x = case pipeline x of
    Err msg -> nat-to-int (string-length msg)
    _ -> 0

  loop :: Int -> Int -> Int
  loop n acc =
    if n == 0 then acc
    else
      let v = ignore-err (n - 2) in
      let e = err-len (n - 2) in
      loop (n - 1) (acc + v + e)

  unwrap-ok :: Int -> Int
  unwrap-ok x = case pipeline x of
    Ok v -> v
    Err e -> 0

  main :: Int
  main =
    let total = loop 500 0 in
    let ok-val = unwrap-ok 39 in
    if total > 0 && ok-val == 42 then 42 else 1
"#,
    );
}

#[test]
fn option_recursive_and_loop_paths_are_unboxed_and_leak_free() {
    run_leak_checked(
        "csr-option-loop",
        r#"module Main where
  import Algebra.Semigroup
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)

  find-pos :: Int -> Option Int
  find-pos n =
    if n <= 0 then None
    else if n == 7 then Some 42
    else find-pos (n - 1)

  maybe-tag :: Int -> Option String
  maybe-tag n =
    if n == 0 then None
    else Some ("tag:" <> "ok")

  get-pos :: Int -> Int
  get-pos n = case find-pos n of
    None -> 0
    Some v -> v

  tag-len :: Int -> Int
  tag-len n = case maybe-tag n of
    None -> 0
    Some s -> nat-to-int (string-length s)

  tag-flag :: Int -> Int
  tag-flag n = case maybe-tag n of
    None -> 1
    _ -> 2

  sum-opts :: Int -> Int -> Int
  sum-opts n acc =
    if n == 0 then acc
    else
      let a = get-pos 10 in
      let b = tag-len (n - 1) in
      let c = tag-flag n in
      sum-opts (n - 1) (acc + a + b + c)

  main :: Int
  main =
    let r = sum-opts 1000 0 in
    let missed = case find-pos 3 of
          None -> 42
          Some x -> x in
    if r > 0 && missed == 42 then 42 else 1
"#,
    );
}

#[test]
fn user_two_variant_sum_works_unboxed_and_boxed() {
    run_leak_checked(
        "csr-user-sum",
        r#"module Main where
  import Algebra.Semigroup
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)

  data Outcome where
    Fail :: String -> Outcome
    Done :: Int -> Outcome

  classify :: Int -> Outcome
  classify n =
    if n < 0 then Fail ("bad" <> "-input")
    else Done (n * 2)

  -- Direct case-scrutinized path (unboxed CSR)
  eval-direct :: Int -> Int
  eval-direct n = case classify n of
    Fail s -> nat-to-int (string-length s)
    Done v -> v

  -- Escaping / higher-order path (boxed via C wrapper)
  apply-outcome :: (Int -> Outcome) -> Int -> Outcome
  apply-outcome f x = f x

  unwrap-outcome :: Outcome -> Int
  unwrap-outcome o = case o of
    Fail s -> nat-to-int (string-length s)
    Done v -> v

  main :: Int
  main =
    let a = eval-direct 15 in
    let b = eval-direct (0 - 1) in
    let c = unwrap-outcome (apply-outcome classify 3) in
    let d = unwrap-outcome (apply-outcome classify (0 - 2)) in
    -- a=30, b=9, c=6, d=9 -> 30 + 9 + 6 - 9 + 6 = 42
    if a == 30 && b == 9 && c == 6 && d == 9 then 42 else 1
"#,
    );
}

#[test]
fn csr_emits_register_pair_ir_for_option_and_result() {
    let dir = common::project(
        "csr-ir-check",
        &[(
            "Main.cinch",
            r#"module Main where
  find-even :: Int -> Option Int
  find-even n =
    if n <= 0 then None
    else if n == 40 then Some n
    else find-even (n - 1)

  check-rec :: Int -> Result Int Int
  check-rec n =
    if n < 0 then Err (0 - n)
    else if n == 2 then Ok 2
    else check-rec (n - 1)

  use-div :: Int -> Int
  use-div a = case find-even a of
    None -> 0
    Some v -> v

  use-res :: Int -> Int
  use-res n = case check-rec n of
    Err e -> e
    Ok v -> v

  main :: Int
  main = use-div 45 + use-res 5
"#,
        )],
    );
    let ir_path = dir.join("out.ll");
    common::drive(cinch::driver::Mode::EmitIr, &dir, "Main.cinch", Some(ir_path.clone()))
        .expect("emit IR");
    let ir = std::fs::read_to_string(&ir_path).expect("read emitted IR");
    // Both Rust seed (`#cc`) and Stage 3 (`#csr`) emit `{ i8, i64 }` workers.
    assert!(
        ir.contains("{ i8, i64 }"),
        "expected unboxed {{ i8, i64 }} CSR signature in IR"
    );
    let exe = common::build_executable(&dir, "Main.cinch", "csr-ir")
        .expect("build csr-ir");
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .output()
        .expect("run csr-ir");
    assert_eq!(out.status.code(), Some(42));
    common::cleanup(&dir);
}
