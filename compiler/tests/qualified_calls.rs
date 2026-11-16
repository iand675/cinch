//! Saturation and constant-space calls carrying checked runtime evidence.
mod common;

use cinch::driver::Mode;

/// Compile one source and retain the generated LLVM beside a checked runtime execution.
fn run(source: &str) -> String {
    run_expected(source, b"")
}

/// Preserve observable effect order while checking runtime ownership and the emitted calls.
fn run_expected(source: &str, expected_stdout: &[u8]) -> String {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .unwrap();
    let output = std::process::Command::new(&exe)
        .env("CINCH_STACK_SIZE", "8M")
        .env("CINCH_HEAP_LIMIT", "16M")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    assert_eq!(output.stdout, expected_stdout);
    std::fs::read_to_string(exe.with_extension("ll")).unwrap()
}

/// Saturated evidence calls avoid partial closures while genuine partial and excess applications still work.
#[test]
fn qualified_calls_preserve_saturation_partial_and_excess_arguments() {
    let ir = run(r#"module Main where
  -- | Exercise a checked capability prefix followed by ordinary numeric arguments.
  plus :: forall r. using (io :: Io r) => Nat -> Nat ->{IO r} Nat
  plus a b = a + b
  -- | Keep a direct source-to-source evidence call visible in the self-built backend.
  caller :: forall r. using (io :: Io r) => Nat -> Nat ->{IO r} Nat
  caller a b = plus a b
  -- | Return a function after the first effectful arrow, allowing an excess argument.
  choose :: forall r. using (io :: Io r) => Bool ->{IO r} (Nat -> Nat)
  choose selected = if selected then (\n -> n + 3) else (\n -> n + 4)
  -- | Observe direct, partial and excess argument application under the same capability.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let partial = plus 40 in
    let one = partial 2 in
    let two = choose True 7 in
    let three = caller 19 23 in
    if one == 42 && two == 10 && three == 42 then 0 else 1
"#);
    if common::external_frontend::configured().unwrap().is_some() {
        let body = ir
            .split("define i64 @\"cinch.typed.m.Main.caller\"")
            .nth(1)
            .expect("selected source caller")
            .split("\n}")
            .next()
            .unwrap();
        assert!(
            body.contains("call i64 @\"cinch.typed.m.Main.plus\""),
            "{body}"
        );
        assert!(
            !body.contains("@__apply") && !body.contains("@cinch_alloc_shaped"),
            "saturated evidence call still allocates an application chain: {body}"
        );
    }
}

/// Mutual recursion includes the evidence prefix in its shared prototype even when source arities differ.
#[test]
fn qualified_mutual_tail_calls_keep_constant_stack_and_heap() {
    let ir = run(r#"module Main where
  -- | Transfer the same checked runtime capability along with both changing values.
  odd :: forall r. using (io :: Io r) => Nat -> Nat ->{IO r} Nat
  odd n total = if n == 0 then total else even (n - 1) (total + 1) 0
  -- | A different source arity needs padding after the capability prefix.
  even :: forall r. using (io :: Io r) => Nat -> Nat -> Nat ->{IO r} Nat
  even n total extra = if n == 0 then total + extra else odd (n - 1) (total + 1)
  -- | Depth exceeds the small native stack while the live heap remains bounded.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in if odd 1000001 0 == 1000001 then 0 else 1
"#);
    if common::external_frontend::configured().unwrap().is_some() {
        assert!(ir.contains("musttail call i64 @\"cinch.typed.m.Main.odd#tail\""));
        assert!(ir.contains("musttail call i64 @\"cinch.typed.m.Main.even#tail\""));
    }
}

/// Owned evidence survives mutual transfers, partial capture and the final boxed result without count imbalance.
#[test]
fn qualified_owned_evidence_is_retained_before_frame_roots_are_dropped() {
    run(r#"module Main where
  -- | Store a mathematical integer outside the immediate representation.
  data Box where
    -- | Keep the integer alive through dictionary value ownership.
    Box :: Int -> Box
  -- | Supply an owned value rather than a raw capability token.
  signature Stored a where
    -- | Observe the same dictionary-owned value at the end of recursion.
    stored :: a
  -- | Construct an owned dictionary containing a non-immediate payload.
  instance Selected :: Stored Box where
    stored = Box 340282366920938463463374607431768211456
  -- | Pass evidence before a normal source argument.
  left :: using (chosen :: Stored Box) => Nat -> Box
  left n = if n == 0 then stored else right (n - 1) 0
  -- | Use a differently sized source prototype in the same tail cycle.
  right :: using (chosen :: Stored Box) => Nat -> Nat -> Box
  right n extra = if n == 0 then stored else left (n - 1)
  -- | Capture the dictionary in a real partial application before transferring it.
  main :: Int
  main = with (chosen = Selected) in let partially = right 100001 in
    case partially 0 of Box result -> if result == 340282366920938463463374607431768211456 then 0 else 1
"#);
}

/// An effectful application stage runs before evaluating an excess argument to its returned function.
#[test]
fn qualified_excess_arguments_preserve_observable_effect_order() {
    run_expected(
        r#"module Main where
  -- | Print an argument marker and return the same value.
  mark :: forall r. using (io :: Io r) => Nat ->{IO r} Nat
  mark n = let written = ioWrite (string-to-bytes (nat-to-string n)) in n
  -- | Perform an effect before returning one of two ordinary functions.
  choose :: forall r. using (io :: Io r) => Nat ->{IO r} (Nat -> Nat)
  choose n = let written = ioWrite (string-to-bytes "F") in
    if n == 1 then (\next -> n + next) else (\next -> n + next + 1)
  -- | The first marker precedes the function body; the excess argument follows that body.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let result = choose (mark 1) (mark 2) in if result == 3 then 0 else 1
"#,
        b"1F2",
    );
}

/// A function returned through a local alias preserves every effectful application boundary.
#[test]
fn qualified_nested_effectful_stages_keep_arguments_after_their_prefix() {
    run_expected(
        r#"module Main where
  -- | Print the next argument before returning its numeric payload.
  mark :: forall r. using (io :: Io r) => Nat ->{IO r} Nat
  mark n = let written = ioWrite (string-to-bytes (nat-to-string n)) in n
  -- | Produce two successive effectful function stages before the final pure argument.
  choose :: forall r. using (io :: Io r) => Nat ->{IO r} (Nat ->{IO r} (Nat -> Nat))
  choose n = let first = ioWrite (string-to-bytes "F") in
    if n == 1 then (\middle -> let second = ioWrite (string-to-bytes "G") in
      if middle == 2 then (\last -> n + middle + last) else (\last -> 0))
    else (\middle -> \last -> 0)
  -- | Calling through an alias must retain the same stage order as a direct source call.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in let selected = choose in
    let result = selected (mark 1) (mark 2) (mark 3) in if result == 6 then 0 else 1
"#,
        b"1F2G3",
    );
}
