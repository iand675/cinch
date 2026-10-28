//! End-to-end tests for effects, capabilities and sequencing (P6):
//! handlers (`handle ... with`, operation clauses, one-shot resumptions),
//! runtime capabilities (`using (io :: Io r)`, `with` supplies, `@{...}`)
//! and `do` notation over the direct, `Result` and `Option` providers.

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

// ---------------------------------------------------------------- handlers

#[test]
fn handled_operation_returns_through_its_clause() {
    // The clause resumes the continuation once: `k 5` runs the rest of the
    // handled computation (`+ 10`) and the return clause (`* 2`), and the
    // clause's own value is the result of the `handle`.
    expect_ok(
        "handle-resume",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = handle poke () + 10 with (handler Poke where
    return x -> x * 2
    poke u resume k -> k 5)
"#,
        30,
    );
    // A clause that never resumes simply returns its value: the rest of
    // the handled computation never runs.
    expect_ok(
        "handle-drop",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = handle poke () + 10 with (handler Poke where
    return x -> x * 2
    poke u resume k -> 7)
"#,
        7,
    );
    // A clause may ignore its argument patterns.
    expect_ok(
        "handle-return-only",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  twice :: Int -> Int
  twice n = handle poke () * n with (handler Poke where
    return x -> x
    poke _ resume k -> k 3)

  main :: Int
  main = twice 5
"#,
        15,
    );
}

#[test]
fn unhandled_operation_is_rejected_at_check_time() {
    // An unhandled operation surfaces its label honestly, so a pure
    // definition cannot hide it (decision D23: the check-time rejection is
    // the rule; the elaboration falls back to the fault path as defense in
    // depth for library code that never runs).
    expect_err(
        "unhandled-main",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = poke ()
"#,
        "must be pure",
    );
    expect_err(
        "unhidden-effect",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  f :: Int -> Int
  f x = poke ()

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
    // A handler that reaches the label through a call is rejected: the
    // bootstrap dispatches operations only from the handled computation's
    // own expression (decision D23).
    expect_err(
        "handled-through-call",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  fired :: Unit ->{Poke} Int
  fired u = poke ()

  main :: Int
  main = handle fired () with (handler Poke where
    return x -> x
    poke u resume k -> k 1)
"#,
        "performed through a called computation",
    );
}

#[test]
fn handler_effect_typing_shows_introduced_effects() {
    // The effects a clause body performs appear in the handler's type and
    // in the `handle` expression's performed row (core specification
    // section 6: handling an effect does not establish totality).
    expect_err(
        "diverging-clause",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  loop :: forall a b. a ->{Diverge} b
  loop x = loop x

  f :: Int -> Int
  f n = handle poke () with (handler Poke where
    return x -> x
    poke u resume k -> loop u)

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
    // The divergence reaches the entry point too when nothing hides it.
    expect_err(
        "diverging-clause-at-main",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  loop :: forall a b. a ->{Diverge} b
  loop x = loop x

  main :: Int
  main = handle poke () with (handler Poke where
    return x -> x
    poke u resume k -> loop u)
"#,
        "must be pure",
    );
}

#[test]
fn one_shot_resumption_rejects_a_second_resume() {
    // The resumption is one-shot: a second `resume` is a usage error even
    // when the continuation captures nothing linear (design section 3).
    expect_err(
        "two-resumes",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = handle poke () with (handler Poke where
    return x -> x
    poke u resume k -> k 1 + k 2)
"#,
        "must be used at most once",
    );
}

#[test]
fn resumption_not_duplicating_a_linear_capture_is_rejected() {
    // The continuation of `poke` consumes the `%1` parameter `lin`, so the
    // resumption must run exactly once: resuming twice would duplicate the
    // linear capture, and the usage checker rejects the second resume.
    expect_err(
        "resume-duplicates-linear",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  consume :: Int %1 -> Int
  consume x = x

  run :: Int %1 -> Int
  run lin = handle poke () + consume lin with (handler Poke where
    return x -> x
    poke u resume k -> k 1 + k 2)

  main :: Int
  main = 0
"#,
        "must be used exactly once",
    );
    // Dropping the resumption would drop the linear capture: also
    // rejected.
    expect_err(
        "resume-drops-linear",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  consume :: Int %1 -> Int
  consume x = x

  run :: Int %1 -> Int
  run lin = handle poke () + consume lin with (handler Poke where
    return x -> x
    poke u resume k -> 99)

  main :: Int
  main = 0
"#,
        "must be used exactly once",
    );
    // Resuming exactly once consumes the capture exactly once: accepted,
    // and the capture is consumed on the resumed path.
    expect_ok(
        "resume-once-linear",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  consume :: Int %1 -> Int
  consume x = x

  run :: Int %1 -> Int
  run lin = handle poke () + consume lin with (handler Poke where
    return x -> x
    poke u resume k -> k 1)

  main :: Int
  main = run 5
"#,
        6,
    );
}

// ---------------------------------------------------------------- capabilities

#[test]
fn with_supplies_a_capability() {
    expect_ok(
        "with-supply",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = with (io = newIO ()) in act ()
"#,
        7,
    );
    // The capability binder is a term-level value too.
    expect_ok(
        "with-supply-forward",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  forward :: forall r. using (io :: Io r) => Unit -> Int
  forward u = with (io = io) in act ()

  main :: Int
  main = with (io = newIO ()) in forward ()
"#,
        7,
    );
}

#[test]
fn with_rejects_a_wrong_type_supply() {
    expect_err(
        "with-int-supply",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = with (io = 3) in act ()
"#,
        "must have type `Io r`",
    );
}

#[test]
fn explicit_capability_supplies_apply_at_the_call() {
    expect_ok(
        "explicit-supply",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = act @{io = newIO ()} ()
"#,
        7,
    );
    expect_err(
        "explicit-supply-unknown-name",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = act @{cap = newIO ()} ()
"#,
        "has no capability parameter",
    );
    expect_err(
        "explicit-supply-wrong-type",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = act @{io = True} ()
"#,
        "got Io",
    );
}

#[test]
fn capability_scoping_is_lexical() {
    // An inner `with` shadows the outer provider.
    expect_ok(
        "with-shadows",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = with (io = newIO ()) in with (io = newIO ()) in act ()
"#,
        7,
    );
    // With no provider in scope, a capability want is rejected.
    expect_err(
        "no-capability",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  main :: Int
  main = act ()
"#,
        "capability in scope",
    );
    // A capability name does not exist outside its `with` scope.
    expect_err(
        "capability-out-of-scope",
        r#"
module Main where
  act :: forall r. using (io :: Io r) => Unit -> Int
  act u = 7

  plus :: Int -> Int -> Int
  plus x y = x + y

  main :: Int
  main = plus (with (io = newIO ()) in act ()) (act @{io = io} ())
"#,
        "unknown name io",
    );
}

// ---------------------------------------------------------------- do notation

#[test]
fn do_blocks_sequence_through_the_direct_provider() {
    expect_ok(
        "do-direct",
        r#"
module Main where
  main :: Int
  main = do
    x <- 3
    y <- 4
    pure (x + y)
"#,
        7,
    );
    // A `let` statement binds a pure value strictly in either
    // interpretation.
    expect_ok(
        "do-let",
        r#"
module Main where
  main :: Int
  main = do
    let x = 3
    y <- pure x
    pure y
"#,
        3,
    );
    expect_err(
        "do-let-impure",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = do
    let x = poke ()
    pure x
"#,
        "must be pure",
    );
    // Refutable bindings need a typed failure operation; the bootstrap
    // providers do not supply one (decision D25).
    expect_err(
        "do-refutable",
        r#"
module Main where
  main :: Int
  main = do
    Some x <- pure 3
    pure x
"#,
        "failure operation",
    );
}

#[test]
fn do_let_binds_a_whole_local_block_group() {
    // The surface EBNF `do_statement = "let", local_block`: one `let`
    // statement binds a whole group (the binding-group form of the
    // surface grammar local amendments section 9), and later members see
    // earlier ones as if the group were nested single `let`s.
    expect_ok(
        "do-let-group",
        r#"
module Main where
  main :: Int
  main = do
    let x = 3
        y = x + 4
    z <- pure y
    pure z
"#,
        7,
    );
    // The explicit-brace local block takes the whole local grammar: a
    // signature, a local function equation, and a value bound from them.
    expect_ok(
        "do-let-group-grammar",
        r#"
module Main where
  main :: Int
  main = do
    let { inc :: Int -> Int; inc n = n + 1; v = inc 40 }
    pure v
"#,
        41,
    );
    // A `~` memo binding is a group member too; its body is deferred to
    // `demand` (laziness and reclamation specification section 5).
    expect_ok(
        "do-let-group-memo",
        r#"
module Main where
  main :: Int
  main = do
    let ~ y = 40
    z <- pure (copyOut (demand y) + 1)
    pure z
"#,
        41,
    );
    // One group never binds the same name twice: a second binding is a
    // mistake, not a shadow of the first.
    expect_err(
        "do-let-group-duplicate",
        r#"
module Main where
  main :: Int
  main = do
    let x = 1
        x = 2
    pure x
"#,
        "duplicate binding `x` in the `let` group",
    );
    // Every strict member keeps the pure discipline: a `let` statement
    // does not sequence effects in either interpretation.
    expect_err(
        "do-let-group-impure",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = do
    let a = 1
        b = poke ()
    pure b
"#,
        "must be pure",
    );
    // Under a handled computation only simple value bindings sequence
    // (decision D24): a local function in the group rejects.
    expect_err(
        "do-let-group-handled",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  handled :: Int -> Int
  handled n = handle (do
      let f x = x + 1
      y <- poke ()
      pure (f (n + y))) with (handler Poke where
    return x -> x
    poke u resume k -> k 4)

  main :: Int
  main = handled 1
"#,
        "local functions and memo bindings under a handled computation",
    );
}

#[test]
fn do_blocks_short_circuit_through_the_result_provider() {
    expect_ok(
        "do-result",
        r#"
module Main where
  step :: forall e. using (seq :: ResultSequencing e) => Result e Int -> Result e Int
  step r = do
    x <- r
    pure (x + 1)

  main :: Int
  main = case (step (Ok 41), step (Err 3)) of
    (Ok n, Err m) -> n + m
    (Ok n, Ok m) -> n + m
    _ -> 0
"#,
        45,
    );
    // Two active sequencing providers in one selection scope are
    // ambiguous.
    expect_err(
        "do-two-providers",
        r#"
module Main where
  step :: forall e. using (seq :: ResultSequencing e, other :: OptionSequencing) => Result e Int -> Result e Int
  step r = do
    x <- r
    pure x

  main :: Int
  main = 0
"#,
        "two active sequencing providers",
    );
}

#[test]
fn do_blocks_short_circuit_through_the_option_provider() {
    expect_ok(
        "do-option",
        r#"
module Main where
  step :: using (seq :: OptionSequencing) => Option Int -> Option Int
  step o = do
    x <- o
    pure (x + 1)

  main :: Int
  main = case (step (Some 4), step None) of
    (Some n, None) -> n
    (Some n, Some m) -> n + m
    _ -> 0
"#,
        5,
    );
}

#[test]
fn do_keeps_effect_honesty() {
    // A `do` block sequences values; the desugared binds do not hide the
    // statements' effects.
    expect_err(
        "do-hides-nothing",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = do
    x <- poke ()
    pure x
"#,
        "must be pure",
    );
    // ... and a declared row still checks against the body.
    expect_err(
        "do-effect-row",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  f :: Int -> Int
  f n = do
    x <- poke ()
    pure x

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
    // An honest row is accepted end to end through `do` under a handler
    // that dispatches the operation from the handled computation itself.
    expect_ok(
        "do-effect-row-ok",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  handled :: Int -> Int
  handled n = handle (do
      x <- poke ()
      pure (x + n)) with (handler Poke where
    return x -> x
    poke u resume k -> k 4)

  main :: Int
  main = handled 1
"#,
        5,
    );
    // A definition declaring the row keeps it: the program checks.
    expect_ok(
        "do-effect-row-declared",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  run :: Int ->{Poke} Int
  run n = do
    x <- poke ()
    pure (x + n)

  main :: Int
  main = 0
"#,
        0,
    );
}
