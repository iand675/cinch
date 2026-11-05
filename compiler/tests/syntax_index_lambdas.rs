//! Index lambdas retain lexical handles and checked annotations.
use cinch::driver::{check_sources, run_source_with, StagingCx};

fn program(body: &str, captured: bool) -> String {
    let body = format!("congr ({body}) (same next)");
    let body = if captured { format!("wrap!(({body}))") } else { body };
    format!(r#"module Main where
  macro wrap :: forall a. Syntax -> Code a
  macro wrap source = Syntax.capture source
  data Count where
    Z :: Count
    S :: Count -> Count
  same :: (value :: Count) -> Eq value value
  same value = case value of Z -> refl; S next -> {body}
  main :: Int
  main = 42
"#)
}

#[test]
fn index_lambda_binders_work_in_raw_and_captured_induction() {
    for captured in [false, true] {
        assert_eq!(run_source_with(&program("\\item -> S item", captured), &mut StagingCx::default()).unwrap(), 42);
    }
}

#[test]
fn index_lambda_annotations_are_checked_in_the_binders_scope() {
    for captured in [false, true] {
        assert_eq!(run_source_with(&program("\\item -> S (item :: Count)", captured), &mut StagingCx::default()).unwrap(), 42);
    }
}

#[test]
fn index_lambdas_cannot_discard_a_wrong_carrier_annotation() {
    for captured in [false, true] {
        let error = check_sources(&program("\\item -> S (item :: Bool)", captured), "wrong-lambda-carrier.cinch")
            .err().expect("an index lambda cannot drop its annotation").to_string();
        assert!(error.contains("Bool") && error.contains("Count"), "{error}");
    }
}

#[test]
fn nested_captured_index_lambdas_keep_distinct_same_spelled_binders() {
    for captured in [false, true] {
        assert_eq!(run_source_with(&program("\\item -> S ((\\item -> item) item)", captured), &mut StagingCx::default()).unwrap(), 42);
    }
}

const OUTER: &str = r#"module Main where
  macro constant :: Syntax -> Name -> Code (Nat -> Nat)
  macro constant source name = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.capture source)
  same :: (point :: Nat) -> Eq point point
  same point = congr (constant!((point), point)) refl
  main :: Int
  main = 42
"#;

#[test]
fn an_index_lambda_cannot_capture_an_existing_outer_subject_by_spelling() {
    assert_eq!(run_source_with(OUTER, &mut StagingCx::default()).unwrap(), 42);
}

#[test]
fn an_introduced_index_binder_cannot_retarget_an_outer_proof_goal() {
    let source = OUTER.replace("(point :: Nat) -> Eq point point", "(point :: Nat) -> (other :: Nat) -> Eq point other")
        .replace("same point =", "same point other =");
    let error = check_sources(&source, "index-lambda-retarget.cinch")
        .err().expect("a captured outer subject is not the new lambda parameter").to_string();
    assert!(error.contains("index mismatch"), "{error}");
}

#[test]
fn index_lambdas_in_captured_type_annotations_retain_their_handles() {
    let source = r#"module Main where
  macro wrap :: Syntax -> Code Int
  macro wrap source = Syntax.capture source
  data Tagged (index :: Nat) where
    Tag :: Int -> Tagged index
  use :: Tagged 0 -> Int
  use token = BODY
  main :: Int
  main = use (Tag 42)
"#;
    let body = "case (token :: Tagged ((\\index -> index) 0)) of Tag value -> value";
    for captured in [false, true] {
        let body = if captured { format!("wrap!(({body}))") } else { body.to_string() };
        assert_eq!(run_source_with(&source.replace("BODY", &body), &mut StagingCx::default()).unwrap(), 42);
    }
}
