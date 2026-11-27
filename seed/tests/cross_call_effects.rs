//! Cross-call Koka-style algebraic effect handlers (`Effect.Throw`, tail-resumptive
//! and abortive handlers, cooperative bubbling, exact Perceus RC, `%1` linearity
//! soundness, flow-sensitive narrowing, and zero-overhead pure functions).
//!
//! Routed through `common::build_executable` / `common::check` / `common::drive`
//! so the same suite runs against both the Rust seed and Stage 3.

mod common;
use std::process::Command;

fn run_leak_checked(tag: &str, source: &str) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build_executable(&dir, "Main.cinch", "cross-eff")
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
        "{tag}: every block must be released cleanly on abort/resume; stderr:\n{stderr}"
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
fn throw_deep_recursive_short_circuit_is_leak_free() {
    run_leak_checked(
        "throw-deep-recursive",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Throw

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

  deep-walk :: Int -> String ->{Throw String} Int
  deep-walk depth label =
    let s = label <> "-x" in
    let t = build-tree 4 in
    if depth <= 0 then throw (s <> "-boom")
    else
      let sub = deep-walk (depth - 1) s in
      sub + tree-sum t

  lift-step :: Int ->{Throw String} Int
  lift-step n =
    let a = try-throw (if n > 0 then Ok (n * 2) else Err ("neg" <> "!")) in
    a + 1

  err-len :: Result String Int -> Int
  err-len r = case r of
    Ok _ -> 0
    Err msg -> nat-to-int (string-length msg)

  ok-val :: Result String Int -> Int
  ok-val r = case r of
    Ok v -> v
    Err _ -> 0

  main :: Int
  main =
    let r1 = catch (\() -> deep-walk 8 "go") in
    let r2 = catch (\() -> lift-step 10) in
    let r3 = catch (\() -> lift-step (0 - 3)) in
    let a = err-len r1 in
    let b = ok-val r2 in
    let c = err-len r3 in
    if a == 25 && b == 21 && c == 4 then 42 else 1
"#,
    );
}

#[test]
fn throw_higher_order_map_and_fold_short_circuit_leak_free() {
    run_leak_checked(
        "throw-higher-order-map-fold",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Throw

  data StrList where
    SNil :: StrList
    SCons :: String -> StrList -> StrList

  map-str :: forall (r :: Row). (String ->{|r} String) -> StrList ->{|r} StrList
  map-str f xs = case xs of
    SNil -> SNil
    SCons h t ->
      let h2 = f h in
      let t2 = map-str f t in
      SCons h2 t2

  fold-str :: forall a (r :: Row). (a -> String ->{|r} a) -> a -> StrList ->{|r} a
  fold-str f acc xs = case xs of
    SNil -> acc
    SCons h t ->
      let acc2 = f acc h in
      fold-str f acc2 t

  check-nonempty :: String ->{Throw String} String
  check-nonempty s =
    if nat-to-int (string-length s) == 0 then throw ("empty" <> "-item")
    else s <> "!"

  sum-checked :: Int -> String ->{Throw String} Int
  sum-checked acc s =
    let n = nat-to-int (string-length s) in
    if n > 4 then throw ("too-long:" <> s)
    else acc + n

  err-len-list :: Result String StrList -> Int
  err-len-list r = case r of
    Ok _ -> 0
    Err msg -> nat-to-int (string-length msg)

  err-len-int :: Result String Int -> Int
  err-len-int r = case r of
    Ok _ -> 0
    Err msg -> nat-to-int (string-length msg)

  main :: Int
  main =
    let xs = SCons ("a" <> "b") (SCons ("c" <> "d") (SCons "" (SCons ("e" <> "f") SNil))) in
    let ys = SCons ("hi" <> "") (SCons ("ok" <> "") (SCons ("toolong" <> "!") (SCons "z" SNil))) in
    let r1 = catch (\() -> map-str check-nonempty xs) in
    let r2 = catch (\() -> fold-str sum-checked 0 ys) in
    let v1 = err-len-list r1 in
    let v2 = err-len-int r2 in
    if v1 == 10 && v2 == 17 then 42 else 1
"#,
    );
}

#[test]
fn nested_multiple_typed_throws_and_polymorphic_ensure() {
    run_leak_checked(
        "nested-typed-throws",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Throw

  data ParseErr where
    BadSyntax :: String -> ParseErr

  data IoErr where
    Disconnected :: Int -> IoErr

  ensure :: forall e. Bool -> e ->{Throw e} Unit
  ensure ok err = if ok then () else throw err

  work :: Bool -> Bool ->{Throw ParseErr, Throw IoErr} Int
  work ok-parse ok-io =
    let _ = ensure ok-parse (BadSyntax ("bad" <> "!")) in
    let _ = ensure ok-io (Disconnected 9) in
    100

  run-both :: Bool -> Bool -> Result IoErr (Result ParseErr Int)
  run-both ok-parse ok-io =
    catch (\() -> catch (\() -> work ok-parse ok-io))

  decode-parse :: Result IoErr (Result ParseErr Int) -> Int
  decode-parse r = case r of
    Ok (Err (BadSyntax s)) -> nat-to-int (string-length s)
    _ -> 0

  decode-io :: Result IoErr (Result ParseErr Int) -> Int
  decode-io r = case r of
    Err (Disconnected code) -> code
    _ -> 0

  decode-ok :: Result IoErr (Result ParseErr Int) -> Int
  decode-ok r = case r of
    Ok (Ok v) -> v
    _ -> 0

  main :: Int
  main =
    let a = decode-parse (run-both False True) in
    let b = decode-io (run-both True False) in
    let c = decode-ok (run-both True True) in
    if a == 4 && b == 9 && c == 100 then 42 else 1
"#,
    );
}

#[test]
fn non_tail_handle_expressions_compose_cleanly() {
    run_leak_checked(
        "non-tail-handles",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)
  import Effect.Throw

  fail-if-neg :: Int ->{Throw String} Int
  fail-if-neg n =
    if n < 0 then throw ("neg" <> "-val")
    else n + 10

  err-len :: Result String Int -> Int
  err-len r = case r of
    Err s -> nat-to-int (string-length s)
    Ok _ -> 0

  ok-val :: Result String Int -> Int
  ok-val r = case r of
    Ok v -> v
    Err _ -> 0

  main :: Int
  main =
    let r1 = catch (\() -> fail-if-neg (0 - 1)) in
    let r2 = catch (\() -> fail-if-neg 5) in
    let r3 = catch (\() -> fail-if-neg (0 - 2)) in
    let a = err-len r1 in
    let b = ok-val r2 in
    let c = err-len r3 in
    if a == 7 && b == 15 && c == 7 then 42 else 1
"#,
    );
}

#[test]
fn cross_call_tail_resumptive_and_mixed_handlers() {
    run_leak_checked(
        "tail-and-mixed-handlers",
        r#"module Main where
  import Algebra.Semigroup
  import Data.Result
  import Data.String (String, string-length)
  import Data.Nat (nat-to-int)

  effect Ask where
    ask :: Unit ->{Ask} Int

  effect Parse where
    fail-at :: Int -> String ->{Parse} Int
    note :: String ->{Parse} Int

  consume :: Int %1 -> Int
  consume x = x

  use-ask :: Int ->{Ask} Int
  use-ask x =
    let v = ask () in
    v + x

  run-ask-linear :: Int %1 -> Int
  run-ask-linear lin = handle (let v = ask () in v + consume lin) with (handler Ask where
    return r -> r
    ask u resume k -> k 10)

  parse-step :: Int ->{Parse} Int
  parse-step n =
    let w = note ("tok" <> "!") in
    if n == 0 then fail-at 7 ("eof" <> "!")
    else w + parse-step (n - 1)

  run-parse :: Int -> Result Int Int
  run-parse n =
    handle parse-step n with (handler Parse where
      return x -> Ok x
      fail-at pos msg -> Err (pos + nat-to-int (string-length msg))
      note s resume k -> k (nat-to-int (string-length s)))

  err-val :: Result Int Int -> Int
  err-val r = case r of
    Err v -> v
    Ok _ -> 0

  run-ask :: Int -> Int
  run-ask x = handle use-ask x with (handler Ask where
    return v -> v + 1
    ask u resume k -> k 20)

  main :: Int
  main =
    let a = run-ask 12 in
    let b = err-val (run-parse 3) in
    let c = run-ask-linear 5 in
    if a == 33 && b == 11 && c == 15 then 42 else 1
"#,
    );
}

#[test]
fn flow_sensitive_narrowing_after_throw_guard() {
    run_leak_checked(
        "flow-sensitive-throw",
        r#"module Main where
  import Data.Result
  import Effect.Throw

  only-pos :: using (0 < x) => (x :: Int) -> Int
  only-pos x = x + 1

  checked-pos :: Int ->{Throw Int} Int
  checked-pos n =
    let _ = if n <= 0 then throw 99 else () in
    only-pos n

  ok-val :: Result Int Int -> Int
  ok-val r = case r of
    Ok v -> v
    Err _ -> 0

  err-val :: Result Int Int -> Int
  err-val r = case r of
    Err e -> e
    Ok _ -> 0

  main :: Int
  main =
    let a = ok-val (catch (\() -> checked-pos 41)) in
    let b = err-val (catch (\() -> checked-pos 0)) in
    if a == 42 && b == 99 then 42 else 1
"#,
    );
}

#[test]
fn static_refusals_for_linear_abort_and_nontail_cross_call() {
    refused_any(
        "linear-live-across-throw",
        r#"module Main where
  import Effect.Throw

  consume :: Int %1 -> Int
  consume x = x

  bad :: Int %1 ->{Throw Int} Int
  bad lin =
    let _ = throw 1 in
    consume lin

  main :: Int
  main = 0
"#,
        &["must be used exactly once", "linear"],
    );

    refused_any(
        "linear-live-across-aborting-handler",
        r#"module Main where
  effect Boom where
    boom :: Unit ->{Boom} Int

  consume :: Int %1 -> Int
  consume x = x

  bad :: Int %1 -> Int
  bad lin =
    handle boom () + consume lin with (handler Boom where
      return x -> x
      boom u -> 0)

  main :: Int
  main = 0
"#,
        &["must be used exactly once", "linear"],
    );

    refused_any(
        "nontail-cross-call-refused",
        r#"module Main where
  effect Shift where
    shift :: Unit ->{Shift} Int

  helper :: Unit ->{Shift} Int
  helper u = shift ()

  main :: Int
  main = handle helper () with (handler Shift where
    return x -> x
    shift u resume k -> k 10 + 1)
"#,
        &["performed through a called computation"],
    );
}

#[test]
fn pure_functions_keep_zero_overhead_llvm_ir() {
    let dir = common::project(
        "pure-zero-overhead-ir",
        &[(
            "Main.cinch",
            r#"module Main where
  import Data.Result
  import Effect.Throw

  #[inline(never)]
  pure-add :: Int -> Int -> Int
  pure-add a b = a * 2 + b

  eff-step :: Int ->{Throw Int} Int
  eff-step n = if n == 0 then throw 7 else pure-add n 1

  main :: Int
  main = case catch (\() -> eff-step 20) of
    Ok v -> v + 1
    Err e -> e
"#,
        )],
    );
    let ir_path = dir.join("out.ll");
    common::drive(cinch::driver::Mode::EmitIr, &dir, "Main.cinch", Some(ir_path.clone()))
        .expect("emit IR");
    let ir = std::fs::read_to_string(&ir_path).expect("read emitted IR");
    let pure_fn_start = ir
        .find("pure-add\"")
        .or_else(|| ir.find("pure_add"))
        .expect("pure-add function should appear in emitted IR");
    let pure_slice = &ir[pure_fn_start..];
    let pure_fn_body = &pure_slice[..pure_slice.find("\n}\n").unwrap_or(pure_slice.len())];
    assert!(
        !pure_fn_body.contains("cinch_ev_is_yielding"),
        "pure function must not emit cooperative yield checks:\n{pure_fn_body}"
    );
    let exe = common::build_executable(&dir, "Main.cinch", "pure-ir-exe")
        .expect("build pure-ir-exe");
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .output()
        .expect("run pure-ir-exe");
    assert_eq!(out.status.code(), Some(42));
    common::cleanup(&dir);
}
