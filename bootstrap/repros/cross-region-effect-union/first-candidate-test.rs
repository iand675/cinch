//! Effect accumulation preserves independently inferred region identities.
mod common;

/// Use actual mutable library modules rather than a synthetic effect carrier.
fn project(source: &str) -> std::path::PathBuf {
    common::project("cross_region_effect_union", &[
        ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
        ("Data.Mutable.HashTable.cinch", include_str!("../prelude/Data/Mutable/HashTable.cinch")),
        ("Main.cinch", source),
    ])
}

/// Distinct region binders survive ordinary, local, curried and open-row calls.
#[test]
fn independently_typed_mutations_check_and_execute() {
    let dir = project(include_str!("fixtures/cross-region-mutation.cinch"));
    common::check(&dir, "Main.cinch").expect("both rigid regions are explicitly permitted");
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

/// Check a real mutable operation against an intentionally insufficient contract.
fn reject(signature: &str, parameters: &str, body: &str) {
    let source = format!("-- | Refuse incomplete or mismatched mutation contracts.\nmodule Main where\n  import Data.Mutable.Var\n  -- | This declaration must not conceal or rename a region.\n  bad :: {signature}\n  bad {parameters} = {body}\n  -- | Keep the invalid declaration in an otherwise pure consumer.\n  main :: Int\n  main = 0\n");
    let dir = project(&source);
    let error = common::check(&dir, "Main.cinch").expect_err("mutation identity must remain enforced");
    assert!(error.contains("region") || error.contains("effect"), "{error}");
    common::cleanup(&dir);
}

/// A caller must account for both independently owned mutation effects.
#[test]
fn a_missing_second_mutation_effect_is_rejected() {
    reject("forall r s. Var r Nat -> Var s Nat ->{Mut r} Nat", "first second",
        "let one = var-read first in let two = var-read second in one + two");
}

/// Preserving flexible identities must not let mutation escape into a pure row.
#[test]
fn mutation_cannot_be_hidden_in_a_pure_function() {
    reject("forall r s. Var r Nat -> Var s Nat -> Nat", "first second",
        "let one = var-read first in let two = var-read second in one + two");
}

/// Distinct rigid region identities remain non-interchangeable.
#[test]
fn a_mutation_cannot_be_relabelled_as_another_region() {
    reject("forall r s. Var r Nat ->{Mut s} Nat", "value", "var-read value");
}

/// Mutable storage identity remains part of the result type independently of effect labels.
#[test]
fn a_cell_cannot_be_returned_under_another_region() {
    reject("forall r s. Var r Nat -> Var s Nat", "value", "value");
}
