//! Class heads carry the same definition-site authority as other type names.
mod common;

use common::source_frontend::{check_sources, run_source};

const SOURCE: &str = include_str!("fixtures/syntax/signature-scope.cinch");

#[test]
fn a_quoted_helper_keeps_its_private_signature_and_provider_under_caller_decoys() {
    assert_eq!(run_source(SOURCE).unwrap(), 42);
}

#[test]
fn a_caller_provider_cannot_discharge_a_different_same_shaped_private_signature() {
    let source = SOURCE.replace("main = call!((40))", "main = supplied!((40), Word)");
    let error = check_sources(&source, "signature-provider-scope.cinch").err()
        .expect("the two nominal signature owners differ").to_string();
    assert!(error.contains("Private") && (error.contains("mismatch") || error.contains("not of") || error.contains("dictionary") || error.contains("instance")), "{error}");
}

#[test]
fn a_captured_call_selects_the_callers_signature_and_provider() {
    let source = SOURCE.replace("  main :: Int", "  macro keep :: Syntax -> Code Int\n  macro keep source = Syntax.capture source\n  local-use :: using (dict :: Private Int) => Int -> Int\n  local-use value = mark value\n  main :: Int")
        .replace("mark value = 0", "mark value = value + 2")
        .replace("main = call!((40))", "main = keep!((with (dict = Word) in local-use 40))");
    assert_eq!(run_source(&source).unwrap(), 42);
}

#[test]
fn a_captured_using_head_keeps_its_source_owner_when_emitted_in_another_module() {
    let source = include_str!("fixtures/syntax/captured-signature-scope.cinch");
    assert_eq!(run_source(source).unwrap(), 42);
    let wrong = source.replace("  stage family :: Declarations", "  signature Other a where\n    other-mark :: a -> Int\n  instance OtherWord :: Other Int where\n    other-mark value = value + 2\n  stage family :: Declarations")
        .replace("=> Int -> Int), mark, Word)", "=> Int -> Int), mark, OtherWord)");
    assert_ne!(wrong, source);
    let error = check_sources(&wrong, "captured-signature-wrong-provider.cinch").err()
        .expect("captured type and provider must retain the same class owner").to_string();
    assert!(error.contains("Private") && (error.contains("Other") || error.contains("dictionary") || error.contains("instance")), "{error}");
}

#[test]
fn a_quoted_using_head_cannot_acquire_a_signature_missing_at_definition_site() {
    let source = SOURCE.replace("  signature Private a where\n    mark :: a -> Int\n  instance Word :: Private Int where\n    mark value = value + 2\n", "");
    let error = check_sources(&source, "signature-missing-definition.cinch").err()
        .expect("a later caller signature cannot repair the quote's missing class").to_string();
    assert!(error.contains("Private") && (error.contains("unknown") || error.contains("resolve")), "{error}");
}

#[test]
fn a_quoted_declaration_using_head_keeps_its_private_signature() {
    let source = SOURCE.replace("  main :: Int", "  generate do\n    emit family\n  main :: Int")
        .replace("main = call!((40))", "main = answer");
    assert_eq!(run_source(&source).unwrap(), 42);
}

#[test]
fn quoted_declaration_using_heads_retain_compiler_known_copy_constraints() {
    let source = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    twice :: using (shared :: Copy Int) => Int -> Int;
    twice value = value + value }
  generate do
    emit family
  main :: Int
  main = twice 21
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}
