//! Owned effect labels and operations survive declaration composition.
mod common;
use common::source_frontend::{check_sources, run_source};

const FAMILY: &str = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    effect Poke where { poke :: Unit ->{Poke} Int };
    answer :: Int;
    answer = handle poke () with (handler Poke where { return value -> value; poke _ resume k -> k 42 }) }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'answer) (sym-to-name 'first)
      (Syntax.renameDeclarations (sym-to-name 'poke) (sym-to-name 'first-poke)
        (Syntax.renameDeclarations (sym-to-name 'Poke) (sym-to-name 'FirstPoke) family)))
  main :: Int
  main = first
"#;

#[test]
fn owned_effect_renaming_keeps_rows_operations_and_handler_members_together() {
    assert_eq!(run_source(FAMILY).unwrap(), 42);
}

fn two_families() -> String {
    FAMILY.replace("  main :: Int", r#"    emit (Syntax.renameDeclarations (sym-to-name 'answer) (sym-to-name 'second)
      (Syntax.renameDeclarations (sym-to-name 'poke) (sym-to-name 'second-poke)
        (Syntax.renameDeclarations (sym-to-name 'Poke) (sym-to-name 'SecondPoke) family)))
  main :: Int"#).replace("main = first", "main = if first + second == 84 then 42 else 0")
}

#[test]
fn two_owned_effect_instantiations_keep_independent_operation_tables() {
    assert_eq!(run_source(&two_families()).unwrap(), 42);
}

#[test]
fn an_operation_cannot_be_handled_by_another_owned_effect() {
    let source = two_families().replace("main = if first + second == 84 then 42 else 0",
        "main = handle second-poke () with (handler FirstPoke where { return value -> value; first-poke _ resume k -> k 42 })");
    let error = check_sources(&source, "crossed-owned-effects.cinch").err()
        .expect("one effect's handler cannot discharge another effect").to_string();
    assert!(error.contains("row") || error.contains("effect"), "{error}");
}

#[test]
fn type_and_effect_names_share_spelling_without_sharing_binding_authority() {
    let source = FAMILY.replace("effect Poke where", "data Poke where { Token :: Int -> Poke }; effect Poke where")
        .replace("answer = handle poke () with", "answer = case Token (handle poke () with")
        .replace("poke _ resume k -> k 42 }) }", "poke _ resume k -> k 42 })) of Token value -> value }");
    assert_eq!(run_source(&source).unwrap(), 42);
}

#[test]
fn owned_effect_names_must_remain_valid_source_constructor_names() {
    let source = FAMILY.replace("sym-to-name 'FirstPoke", "sym-to-name 'first-effect");
    let error = check_sources(&source, "owned-effect-name.cinch").err().expect("invalid effect name must fail").to_string();
    assert!(error.contains("constructor identifier"), "{error}");
}

#[test]
fn owned_effect_references_do_not_depend_on_header_order() {
    let source = FAMILY.replace("effect Poke where { poke :: Unit ->{Poke} Int };\n    answer :: Int;",
        "invoke :: Unit ->{Poke} Int; invoke unit = poke unit; effect Poke where { poke :: Unit ->{Poke} Int }; answer :: Int;");
    assert_ne!(source, FAMILY);
    // The forward header and its body are checked even though this fixture's
    // runtime handler dispatches the operation directly (the bootstrap handler
    // calculus does not yet dispatch through arbitrary function calls).
    check_sources(&source, "forward-owned-effect.cinch").unwrap();
    assert_eq!(run_source(&source).unwrap(), 42);
    let bad_header = source.replace("invoke :: Unit ->{Poke} Int", "invoke :: Unit ->{Poke} Bool");
    let error = check_sources(&bad_header, "forward-owned-effect-wrong-result.cinch").err()
        .expect("the unused forward definition must still be checked").to_string();
    assert!(error.contains("Bool") && error.contains("Int"), "{error}");
}

#[test]
fn a_visible_owned_effect_can_be_selected_by_its_qualified_name() {
    let source = FAMILY.replace("main = first", "main = handle first-poke () with (handler Main.FirstPoke where { return value -> value; first-poke _ resume k -> k 42 })");
    assert_eq!(run_source(&source).unwrap(), 42);
}
