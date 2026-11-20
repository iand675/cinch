//! Token captures inside a declaration quote retain its owned namespaces.
mod common;
use common::source_frontend::{check_sources, run_source};

const OWNED: &str = r#"module Main where
  macro keep :: forall a. Syntax -> Code a
  macro keep source = Syntax.capture source
  data Decoy where
    Constructor :: Bool -> Decoy
  maker :: Bool
  maker = False
  stage family :: Declarations
  stage family = quoteDecls where {
    data Local where { Constructor :: Int -> Local };
    maker :: Int -> Local;
    maker value = Constructor value;
    result :: Local;
    result = keep!((maker 42 :: Local)) }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'Constructor) (sym-to-name 'Kept)
      (Syntax.renameDeclarations (sym-to-name 'Local) (sym-to-name 'Actual)
        (Syntax.renameDeclarations (sym-to-name 'maker) (sym-to-name 'build) family)))
  main :: Int
  main = case result of Kept value -> value
"#;

#[test]
fn quoted_macro_arguments_keep_owned_values_constructors_and_annotation_types() {
    assert_eq!(run_source(OWNED).unwrap(), 42);
    let direct_constructor = OWNED.replace("maker 42 :: Local", "Constructor 42 :: Local");
    assert_eq!(run_source(&direct_constructor).unwrap(), 42);
}

#[test]
fn an_existing_caller_capture_does_not_acquire_a_quoted_sibling_by_spelling() {
    let source = r#"module Main where
  maker :: Bool
  maker = False
  macro expose :: Syntax -> Declarations
  macro expose source = quoteDecls where {
    maker :: Int;
    maker = 42;
    result :: Int;
    result = $(Syntax.capture source) }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'maker) (sym-to-name 'private-maker) (expose!((maker))))
  main :: Int
  main = result
"#;
    let error = check_sources(source, "quoted-macro-scope.cinch")
        .err().expect("the original caller Bool must remain Bool").to_string();
    assert!(error.contains("Bool") && error.contains("Int"), "{error}");
}
