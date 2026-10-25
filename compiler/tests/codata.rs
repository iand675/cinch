//! Codata and guardedness tests (P8): `codata` observations, `Later` as a
//! guarded modality distinct from `Thunk`, `corec` with productivity
//! checking, and the productivity-loss rule for unrestricted filtering
//! (core language specification section 9; codata and guardedness design).

use safelang::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_check_ok(name: &str, src: &str) {
    if let Err(e) = check_sources(src, "<test>") {
        panic!("{name}: expected the program to check, but it failed: {e}");
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

const STREAM: &str = r#"
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)
"#;

#[test]
fn corec_stream_observes_head_and_tail() {
    expect_ok(
        "stream-observe",
        &format!(
            r#"
module Main where
{STREAM}
  corec from :: Nat -> Stream Nat
  corec from n = Stream {{ head = n, tail = delay (\() -> from (n + 1)) }}

  corec map :: forall a b. (a -> b) -> Stream a -> Stream b
  corec map f xs = Stream {{ head = f xs.head, tail = delay (\() -> map f (force xs.tail)) }}

  main :: Int
  main =
    let s = map (\x -> x * 10) (from 1) in
    if s.head == 10 then
      (if (force s.tail).head == 20 then
        (if (force (force s.tail).tail).head == 30 then 0 else 3)
       else 2)
    else 1
"#
        ),
        0,
    );
}

#[test]
fn guarded_delay_and_force_accepted() {
    expect_ok(
        "guarded-later",
        r#"
module Main where
  later :: Int -> Later Int
  later n = delay (\() -> n)

  run :: Later Int -> Int
  run t = force t

  main :: Int
  main = if run (later 5) == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn unproductive_corec_is_rejected_naming_the_call() {
    // `corec loop = loop` masquerades as a total stream: the guard check
    // rejects it and names the unguarded call.
    expect_err(
        "corec-loop",
        &format!(
            r#"
module Main where
{STREAM}
  corec loop :: Stream Int
  corec loop = loop

  main :: Int
  main = 0
"#
        ),
        "unguarded recursive call to `loop`",
    );
    // A recursive call outside a `delay` in a `Later` observation is
    // unguarded wherever it sits.
    expect_err(
        "corec-unguarded-head",
        &format!(
            r#"
module Main where
{STREAM}
  corec bad :: Int -> Stream Int
  corec bad n = Stream {{ head = bad n, tail = delay (\() -> bad (n + 1)) }}

  main :: Int
  main = 0
"#
        ),
        "unguarded recursive call to `bad`",
    );
    expect_err(
        "corec-unguarded-body",
        &format!(
            r#"
module Main where
{STREAM}
  corec worse :: Int -> Stream Int
  corec worse n = worse (n + 1)

  main :: Int
  main = 0
"#
        ),
        "unguarded recursive call to `worse`",
    );
}

#[test]
fn corec_needs_an_observation_layer_per_step() {
    expect_err(
        "corec-no-construction",
        &format!(
            r#"
module Main where
{STREAM}
  mk :: Int -> Stream Int
  mk n = Stream {{ head = n, tail = delay (\() -> mk n) }}

  corec build :: Int -> Stream Int
  corec build n = mk n

  main :: Int
  main = 0
"#
        ),
        "must build its codata type at every step",
    );
}

#[test]
fn later_is_not_interchangeable_with_thunk() {
    // A `Thunk` where a `Later` is expected: rejected, never converted.
    expect_err(
        "thunk-where-later",
        r#"
module Main where
  run :: Later Int -> Int
  run t = force t

  leak :: forall e. Thunk {e} Int -> Int
  leak t = run t

  main :: Int
  main = 0
"#,
        "Later and Thunk are distinct",
    );
    // And the other direction.
    expect_err(
        "later-where-thunk",
        r#"
module Main where
  run :: forall e. Thunk {e} Int ->{e} Int
  run t = force t

  leak :: Later Int -> Int
  leak t = run t

  main :: Int
  main = 0
"#,
        "Later and Thunk are distinct",
    );
    // `Later` is a modality with no effect row; `Thunk {e} a` is where
    // effectful deferral lives.
    expect_err(
        "later-with-effect-row",
        r#"
module Main where
  f :: Later {IO} Int -> Int
  f t = force t

  main :: Int
  main = 0
"#,
        "Later must be applied to exactly one type",
    );
}

#[test]
fn later_must_not_be_an_unrestricted_thunk() {
    // The usage-multiplicity rule: a `Later` is consumed by `force`, so a
    // `%many` Later (an unrestricted thunk in disguise) is rejected.
    expect_err(
        "later-many",
        r#"
module Main where
  f :: Later Int %many -> Int
  f t = force t

  main :: Int
  main = 0
"#,
        "unrestricted-thunk",
    );
    // Forcing a bound `Later` twice violates its `%aff` default.
    expect_err(
        "later-force-twice",
        r#"
module Main where
  dup :: Later Int -> Int
  dup t = force t + force t

  main :: Int
  main = 0
"#,
        "at most once",
    );
    // `force` of a Later performs nothing; a Later body must be pure.
    expect_err(
        "later-delay-not-pure",
        r#"
module Main where
  loop :: forall a b. a ->{Diverge} b
  loop x = loop x

  f :: Int -> Later Int
  f n = delay (\() -> loop n)

  main :: Int
  main = 0
"#,
        "a `delay` building a `Later` must be pure",
    );
}

const PSTREAM: &str = r#"
  codata PStream (a :: Type) where
    head :: a
    tail :: Thunk {Diverge} (PStream a)
"#;

#[test]
fn unrestricted_filter_loses_productivity() {
    // An unrestricted filter cannot be a `corec`: the skip branch
    // recurses unguarded.
    expect_err(
        "filter-corec",
        &format!(
            r#"
module Main where
{STREAM}
  corec filter :: (Int -> Bool) -> Stream Int -> Stream Int
  corec filter p xs = if p xs.head
    then Stream {{ head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }}
    else filter p (force xs.tail)

  main :: Int
  main = 0
"#
        ),
        "unguarded recursive call to `filter`",
    );
    // Written as an ordinary definition it may not promise productivity:
    // the unguarded search must be reflected in its type as `Diverge`.
    expect_err(
        "filter-pure-type",
        &format!(
            r#"
module Main where
{STREAM}
  filter :: (Int -> Bool) -> Stream Int -> Stream Int
  filter p xs = if p xs.head
    then Stream {{ head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }}
    else filter p (force xs.tail)

  main :: Int
  main = 0
"#
        ),
        "reflect the loss of productivity",
    );
    // With `Diverge` declared it still cannot return a productive
    // `Stream`: the next-match search would hide in a `Later` guard.
    expect_err(
        "filter-into-later",
        &format!(
            r#"
module Main where
{STREAM}
  filter :: (Int -> Bool) -> Stream Int ->{{Diverge}} Stream Int
  filter p xs = if p xs.head
    then Stream {{ head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }}
    else filter p (force xs.tail)

  main :: Int
  main = 0
"#
        ),
        "a `delay` building a `Later` must be pure",
    );
    // The honest form: the loss of productivity lives in the observation
    // types (`Thunk {Diverge}` instead of `Later`), and the search runs at
    // call time under a declared `Diverge`. The definition checks.
    let filter_src = |main: &str| {
        format!(
            r#"
module Main where
{STREAM}
{PSTREAM}
  corec from :: Int -> Stream Int
  corec from n = Stream {{ head = n, tail = delay (\() -> from (n + 1)) }}

  filter :: (Int -> Bool) -> Stream Int ->{{Diverge}} PStream Int
  filter p xs = if p xs.head
    then PStream {{ head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }}
    else filter p (force xs.tail)

  main :: Int
  main = {main}
"#
        )
    };
    expect_check_ok("filter-pstream", &filter_src("0"));
    // And its `Diverge` does not leak into total code: consuming the
    // filtered stream from a pure `main` is refused, so the filter never
    // promises a productive output stream (core language specification
    // section 9).
    expect_err(
        "filter-pstream-total-use",
        &filter_src("if (filter (\\x -> x > 1) (from 0)).head == 2 then 0 else 1"),
        "must be pure",
    );
}

#[test]
fn codata_keeps_thunk_and_memo_discipline() {
    // An observation closure may run any number of times: a `%1` thunk
    // cannot be forced from one (ownership and usage design, section 2).
    expect_err(
        "one-shot-thunk-in-observation",
        r#"
module Main where
  codata Box where
    unbox :: Int

  leak :: Thunk {} Int %1 -> Box
  leak t = Box { unbox = force t }

  main :: Int
  main = 0
"#,
        "unboundedly",
    );
    // A memo cell caches `Share` values; an infinite stream is not one, so
    // the bare use is refused rather than silently shared (laziness
    // specification, section 5).
    expect_err(
        "memo-of-stream",
        &format!(
            r#"
module Main where
{STREAM}
  corec from :: Int -> Stream Int
  corec from n = Stream {{ head = n, tail = delay (\() -> from (n + 1)) }}

  memoHead :: Int -> Int
  memoHead n = s.head
    where ~s = from n

  main :: Int
  main = 0
"#
        ),
        "Copy",
    );
}

#[test]
fn codata_is_observed_not_destructured() {
    expect_err(
        "codata-pattern",
        &format!(
            r#"
module Main where
{STREAM}
  peek :: Stream Int -> Int
  peek s = case s of
    Stream {{ head = h }} -> h

  main :: Int
  main = 0
"#
        ),
        "observed, not destructured",
    );
    expect_err(
        "codata-constructor-value",
        &format!(
            r#"
module Main where
{STREAM}
  bad :: Int -> Stream Int
  bad n = Stream n (delay (\() -> bad n))

  main :: Int
  main = 0
"#
        ),
        "observed construction form",
    );
}

#[test]
fn acceptance_example_04_productive_stream_checks() {
    let src = include_str!("../../examples/source/04-productive-stream.lang");
    if let Err(e) = check_sources(src, "04-productive-stream.lang") {
        panic!("example 04 must check end to end: {e}");
    }
}
