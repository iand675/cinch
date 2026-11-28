//! Arena-delimited O(1) fast escape and typed heap budget boundary (`Effect.Arena`).
//!
//! Routed through `common::build_executable` / `common::check` so the same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;
use std::process::Command;

fn run_leak_checked(tag: &str, source: &str) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build_executable(&dir, "Main.cinch", "arena-escape")
        .unwrap_or_else(|e| panic!("{tag}: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("run the built program");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: every block must be released cleanly on O(1) escape; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
}

fn refused_any(tag: &str, source: &str, needles: &[&str]) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let error = common::check(&dir, "Main.cinch").expect_err(tag);
    assert!(
        needles.iter().any(|needle| error.contains(needle)),
        "{tag}: expected one of {needles:?} in:\n{error}"
    );
    common::cleanup(&dir);
}

#[test]
fn arena_normal_and_deep_recursive_abort_are_leak_free() {
    run_leak_checked(
        "arena-deep-abort",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Arena

  data Tree where
    Leaf :: Int -> Tree
    Node :: Tree -> Tree -> Tree

  build-tree :: Int -> Tree
  build-tree n =
    if n <= 0 then Leaf 1
    else Node (build-tree (n - 1)) (Leaf n)

  tree-sum :: Tree -> Int
  tree-sum t = case t of
    Leaf v -> v
    Node l r -> tree-sum l + tree-sum r

  deep-search :: forall (r :: Region). ArenaToken r Int -> Int -> String -> Int
  deep-search tok depth label =
    let s = label <> "-step" in
    let t = build-tree 4 in
    if depth <= 0 then arena-abort tok (tree-sum t + nat-to-int (string-length s))
    else
      let sub = deep-search tok (depth - 1) s in
      sub + tree-sum t

  unpack-ok :: Result Int Int -> Int
  unpack-ok res = case res of
    Ok v -> v
    Err _ -> 0

  unpack-err :: Result Int Int -> Int
  unpack-err res = case res of
    Ok _ -> 0
    Err code -> code

  main :: Int
  main =
    let normal = with-arena (\tok ->
      let t = build-tree 5 in
      tree-sum t) in
    let aborted = with-arena (\tok ->
      deep-search tok 60 "go") in
    let a = unpack-ok normal in
    let b = unpack-err aborted in
    if a > 0 && b > 0 then 42 else 1
"#,
    );
}

#[test]
fn arena_budget_catches_heap_exhaustion_with_zero_leaks() {
    run_leak_checked(
        "arena-budget-exhausted",
        r#"module Main where
  import Data.Result
  import Effect.Arena

  data Chain where
    End :: Chain
    Link :: Int -> Chain -> Chain

  alloc-many :: Int -> Chain -> Int
  alloc-many n acc =
    if n <= 0 then
      case acc of
        End -> 0
        Link h _ -> h
    else alloc-many (n - 1) (Link n acc)

  is-one :: Result BudgetError Int -> Bool
  is-one res = case res of
    Ok v -> v == 1
    Err Exhausted -> False

  is-exhausted :: Result BudgetError Int -> Bool
  is-exhausted res = case res of
    Ok _ -> False
    Err Exhausted -> True

  main :: Int
  main =
    let small = with-arena-budget 65536 (\tok -> alloc-many 20 End) in
    let blown = with-arena-budget 4096 (\tok -> alloc-many 50000 End) in
    if is-one small && is-exhausted blown then 42 else 1
"#,
    );
}

#[test]
fn arena_refuses_unsafe_escapes_linear_captures_and_effects() {
    // 1. Non-scalar/Copy return type (List / heap ADT) cannot escape arena.
    refused_any(
        "arena-refuse-list-escape",
        r#"module Main where
  import Data.List (List, Nil, Cons)
  import Data.Result
  import Effect.Arena

  bad :: Result Int (List Int)
  bad = with-arena (\tok -> Cons 1 Nil)

  main :: Int
  main = 0
"#,
        &["scalar/Copy", "cannot escape"],
    );

    // 2. String return type (heap constructor wrapper) cannot escape arena.
    refused_any(
        "arena-refuse-string-escape",
        r#"module Main where
  import Data.Result
  import Data.String (String)
  import Effect.Arena

  bad :: Result Int String
  bad = with-arena (\tok -> "nope")

  main :: Int
  main = 0
"#,
        &["scalar/Copy", "cannot escape"],
    );

    // 3. ArenaToken r e cannot escape region r.
    refused_any(
        "arena-refuse-token-escape",
        r#"module Main where
  import Data.Result
  import Effect.Arena

  bad = with-arena (\tok -> tok)

  main :: Int
  main = 0
"#,
        &["region", "escape", "scalar/Copy", "cannot"],
    );

    // 4. Linear (%1) binder cannot be captured across an O(1) arena escape jump.
    refused_any(
        "arena-refuse-linear-capture",
        r#"module Main where
  import Data.Result
  import Effect.Arena

  data Handle where
    Handle :: Int -> Handle

  consume :: (%1 h :: Handle) -> Int
  consume h = case h of
    Handle n -> n

  bad :: (%1 h :: Handle) -> Int
  bad h = case with-arena (\tok -> consume h) of
    Ok v -> v
    Err _ -> 0

  main :: Int
  main = 0
"#,
        &["%1", "linear", "capture", "multiplicity", "once"],
    );

    // 5. Effectful / Async / IO operations are forbidden inside the pure arena jump extent.
    refused_any(
        "arena-refuse-effect-in-arena",
        r#"module Main where
  import Data.Result
  import Effect.Arena

  effect Ping where
    ping :: Unit ->{Ping} Int

  bad :: Unit ->{Ping} Int
  bad u = case with-arena (\tok -> ping ()) of
    Ok v -> v
    Err _ -> 0

  main :: Int
  main = 0
"#,
        &["effect", "Ping", "pure", "row"],
    );
}
