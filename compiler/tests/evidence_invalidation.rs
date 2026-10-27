//! Evidence invalidation over mutable state (P16): a proof/evidence item
//! whose statement mentions the contents of a mutable location is scoped
//! to the borrow that produced it and is invalidated by any write to that
//! location (through the borrow or a call that can write it). A rejected
//! program names both the derivation site and the invalidating mutation.
//! `Ref` (read-only borrow) evidence survives its region; immutable
//! evidence never invalidates; `RawPtr` targets carry no evidence and
//! `RawPtr`/FFI writes are opaque mutation (the conservative rule). See
//! the core specification, sections 3-5.

use safelang::driver::{check_sources, run_source};

/// A program that must type-check (evidence used before any invalidating
/// mutation, or re-derived after one).
fn expect_checks(name: &str, src: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => {}
        Err(e) => panic!("{name}: expected the program to check, but it failed: {e}"),
    }
}

/// A program that must be rejected, with every `needle` in the error.
fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!(
            "{name}: expected an error containing {needles:?}, but the program checked"
        ),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(
                    msg.contains(n),
                    "{name}: error {msg:?} does not contain {n:?}"
                );
            }
        }
    }
}

/// A program that must compile and run to `expected` (consumer-visible
/// mutation: reads observe writes through the borrow).
fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

// ------------------------------------------------- derive / mutate / use

#[test]
fn derive_then_mutate_then_use_rejects_naming_both_sites() {
    // `bound` is derived about `m`'s contents, `m` is written, then `bound`
    // is used: rejected, naming the derivation and the write.
    expect_err(
        "derive-mutate-use",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = refSet m 9 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
}

#[test]
fn derive_use_before_mutate_checks() {
    // The evidence is used before any write to the location: valid.
    expect_checks(
        "derive-use-before-mutate",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound -> Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

#[test]
fn re_derive_after_mutate_checks() {
    // After the write, a fresh `decide` re-derives evidence about the new
    // contents; using the fresh item is valid even though an old, stale
    // item existed.
    expect_checks(
        "re-derive-after-mutate",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes _ ->
        let u = refSet m 3 in
        case decide (copyOut m < 5) of
          Yes bound2 -> Some (Fin (copyOut m) bound2)
          No _ -> None
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

// ---------------------------------------------------------- Ref vs Mut

#[test]
fn ref_evidence_survives_its_region() {
    // A `Ref` borrow is read-only: no write to its region can coexist (P5
    // exclusivity), so evidence about its contents is valid throughout the
    // region and never goes stale there.
    expect_checks(
        "ref-survives",
        r#"
module Main where
  prog :: forall region. Ref region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound -> Some (Fin (copyOut m) bound)
      No _ -> None

  useTwice :: forall region. Ref region Nat -> Option (Fin 5)
  useTwice m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let one = Fin (copyOut m) bound in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

// -------------------------------------------------- calls that can write

#[test]
fn call_with_mut_argument_invalidates() {
    // Passing the `Mut` borrow to a callee hands it an exclusive borrow it
    // may write, so the caller's evidence about the location does not
    // survive the call.
    expect_err(
        "mut-arg-call",
        r#"
module Main where
  writer :: forall region. Mut region Nat -> Nat
  writer m = copyOut m

  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = writer m in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
}

#[test]
fn call_with_write_effect_invalidates() {
    // The effect-row reading of "can write": a call whose effect row names
    // the region with a write-capable head (`IO r`) invalidates evidence
    // about it, even without a `Mut` argument.
    expect_err(
        "effect-row-write",
        r#"
module Main where
  writer :: forall region. using (io :: Io region) => Nat ->{IO region} Nat
  writer n = n

  prog :: forall region. using (io :: Io region) => Mut region Nat ->{IO region} Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = writer 7 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
}

#[test]
fn call_with_no_write_does_not_invalidate() {
    // The effect-row reading, negatively: a pure call (no write effect on
    // the region, no `Mut` argument) leaves the evidence valid.
    expect_checks(
        "pure-call-no-invalidate",
        r#"
module Main where
  pureFn :: Nat -> Nat
  pureFn n = n

  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = pureFn 7 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

// ------------------------------------------------------------- immutability

#[test]
fn immutable_evidence_never_invalidates() {
    // Evidence about immutable data (no borrow in its statement) has no
    // mutable footprint: a write to a location cannot make it stale.
    expect_checks(
        "immutable-survives",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Nat -> Option (Fin 5)
  prog m x =
    case decide (x < 5) of
      Yes bound ->
        let u = refSet m 9 in
        Some (Fin x bound)
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

// ---------------------------------------------------------- RawPtr policy

#[test]
fn rawptr_targets_carry_no_evidence() {
    // Conservative RawPtr rule, one way: a `RawPtr` target is opaque and
    // cannot be the subject of derived location evidence.
    expect_err(
        "rawptr-no-evidence",
        r#"
module Main where
  prog :: forall region. RawPtr U8 -> Option (Fin 5)
  prog p =
    case decide (copyOut p < 5) of
      Yes bound -> Some (Fin (copyOut p) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["RawPtr"],
    );
}

#[test]
fn rawptr_writes_invalidate_mutable_evidence() {
    // Conservative RawPtr rule, the other way: a write at an opaque target
    // (a `Bytes` buffer write through its raw pointer, like an FFI/`RawPtr`
    // write) is opaque mutation, so it invalidates every mutable-location
    // evidence item derived before it.
    expect_err(
        "rawptr-write-invalidate",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let b = bytes-new 4 in
        let u = bytes-set b 0 7 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
}

// ------------------------------------------------- erased proof lifetime

#[test]
fn erased_proofs_are_not_resurrected_after_invalidation() {
    // Once a write invalidates evidence, using the stale item is rejected
    // even in an erased proof position (a `Prop`-typed parameter). Wrapping
    // it does not revive it; only a fresh `decide` after the write yields
    // usable evidence.
    expect_err(
        "erased-not-resurrected",
        r#"
module Main where
  consume :: (n :: Nat) -> (i :: Nat) -> (bound :: i < n) -> Nat
  consume n i bound = 0

  prog :: forall region. Mut region Nat -> Nat
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = refSet m 9 in
        consume 5 (copyOut m) bound
      No _ -> 0

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
}

// --------------------------------------------------- consumer-visible run

#[test]
fn mutation_is_observable_through_the_borrow() {
    // Consumer-visible semantics: a write through the borrow changes what
    // a later read observes, so a re-derive after the write decides on the
    // new contents (9 is not < 5, so the second `decide` is `No`).
    expect_run(
        "mutation-observable",
        r#"
module Main where
  prog :: forall region. Mut region Nat -> Int
  prog m =
    case decide (copyOut m < 5) of
      Yes _ ->
        let u = refSet m 9 in
        case decide (copyOut m < 5) of
          Yes _ -> 1
          No _ -> 0
      No _ -> 0

  main :: Int
  main = prog (mutBorrow 3)
"#,
        0,
    );
}
