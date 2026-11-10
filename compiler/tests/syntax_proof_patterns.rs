//! Proof patterns share typed lexical scope and coverage with ordinary patterns.
mod common;
use common::source_frontend::{check_sources, run_source};

#[track_caller]
fn refuses(source: &str, reason: &str) {
    let error = check_sources(source, "proof-patterns.cinch")
        .err().expect("invalid proof pattern must be refused").to_string();
    assert!(error.contains(reason), "{error}");
}

#[test]
fn proof_patterns_require_exhaustive_coverage() {
    refuses(include_str!("fixtures/syntax/proof-case-partial.cinch"), "non-exhaustive");
}

#[test]
fn proof_patterns_keep_recursive_calls_visible_to_termination() {
    refuses(include_str!("fixtures/syntax/proof-case-cyclic.cinch"), "must terminate");
}

#[test]
fn proof_patterns_check_the_actual_subject_carrier() {
    refuses(include_str!("fixtures/syntax/proof-case-wrong-carrier.cinch"), "pattern");
}

#[test]
fn typed_structural_induction_still_checks_and_erases() {
    assert_eq!(run_source(include_str!("fixtures/syntax/proof-case-induction.cinch")).unwrap(), 42);
}

#[test]
fn captured_induction_binders_survive_a_generated_shadow() {
    assert_eq!(run_source(include_str!("fixtures/syntax/proof-case-captured-induction.cinch")).unwrap(), 42);
}

const SHADOWED_GOAL: &str = r#"module Main where
  data Count where
    Z :: Count
    S :: Count -> Count
  preserve :: (outer :: Count) -> (inner :: Count) -> Eq outer (S inner) -> Eq outer (S inner)
  preserve outer inner witness = case outer of Z -> witness; S inner -> witness
  main :: Int
  main = 42
"#;

#[test]
fn pattern_shadows_preserve_the_outer_goal_and_its_existing_evidence() {
    assert_eq!(run_source(SHADOWED_GOAL).unwrap(), 42);
}

#[test]
fn a_pattern_binder_cannot_retarget_an_outer_goal_to_make_refl_fit() {
    refuses(&SHADOWED_GOAL.replace("S inner -> witness", "S inner -> refl"), "index mismatch");
}

#[test]
fn nested_constructor_patterns_retain_induction_fields() {
    let source = r#"module Main where
  data Count where
    Z :: Count
    S :: Count -> Count
  same :: (value :: Count) -> Eq value value
  same value = case value of Z -> refl; S Z -> refl; S (S tail) -> congr S (congr S (same tail))
  main :: Int
  main = 42
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn exhaustive_numeric_patterns_keep_the_default_subject() {
    let source = r#"module Main where
  same :: (value :: Nat) -> Eq value value
  same value = case value of 0 -> refl; rest -> refl
  main :: Int
  main = 42
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn exhaustive_boolean_proof_patterns_still_check() {
    let source = r#"module Main where
  same :: (value :: Bool) -> Eq value value
  same value = case value of True -> refl; False -> refl
  main :: Int
  main = 42
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn congruence_cannot_hide_a_non_decreasing_recursive_proof() {
    let source = r#"module Main where
  cycle :: (value :: Nat) -> Eq value 0
  cycle value = case value of same -> congr (\x -> x) (cycle same)
  main :: Int
  main = 42
"#;
    refuses(source, "must terminate");
}

#[test]
fn proof_former_value_arguments_check_their_carrier() {
    let source = r#"module Main where
  same :: (value :: Nat) -> Eq value value
  same value = refl
  wrong :: Eq True True
  wrong = same True
  main :: Int
  main = 42
"#;
    refuses(source, "Nat");
}

#[test]
fn a_strict_let_alias_is_not_a_structural_decrease() {
    let source = r#"module Main where
  cycle :: (value :: Nat) -> Eq value 0
  cycle value = let same = value in cycle same
  main :: Int
  main = 42
"#;
    refuses(source, "must terminate");
}

#[test]
fn generic_constructor_refinement_keeps_its_type_instantiation() {
    let source = r#"module Main where
  data Wrap a where
    Wrap :: a -> Wrap a
  unwrap :: forall a. Wrap a -> a
  unwrap (Wrap item) = item
  rebuild :: forall a. (value :: Wrap a) -> Eq (Wrap (unwrap value)) value
  rebuild value = case value of Wrap item -> refl
  main :: Int
  main = 42
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

const CAPTURED_FIELD_PROOF: &str = r#"module Main where
  macro hide :: forall a. Syntax -> Name -> Code a
  macro hide source name = let binding = Syntax.newBinding name in Syntax.letIn binding [| False |] (Syntax.capture source)
  data Certified where
    Certified :: (index :: Nat) -> Eq index 0 -> Certified
  measure :: Certified -> Nat
  measure (Certified index witness) = index
  isZero :: (value :: Certified) -> Eq (measure value) 0
  isZero value = case value of Certified index witness -> hide!((witness), index)
  main :: Int
  main = 42
"#;

#[test]
fn captured_erased_fields_keep_their_runtime_subject_under_shadowing() {
    assert_eq!(run_source(CAPTURED_FIELD_PROOF).unwrap(), 42);
}

#[test]
fn an_erased_field_cannot_retarget_the_goal_to_an_introduced_subject() {
    refuses(&CAPTURED_FIELD_PROOF.replace("hide!((witness), index)", "hide!((refl), index)"), "index mismatch");
}

#[test]
fn ordinary_matches_preserve_captured_erased_field_subjects() {
    let source = CAPTURED_FIELD_PROOF.replace(
        "isZero :: (value :: Certified) -> Eq (measure value) 0\n  isZero value = case value of Certified index witness -> hide!((witness), index)",
        "type Zero = refine (value :: Nat) with Eq value 0\n  extract :: Certified -> Zero\n  extract (Certified index witness) = Zero index (hide!((witness), index))",
    );
    assert_ne!(source, CAPTURED_FIELD_PROOF);
    assert_eq!(run_source(&source).unwrap(), 42);
}
