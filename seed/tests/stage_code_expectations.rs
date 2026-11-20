//! Expected Code types must survive stored stage values, splices and composition.
mod common;
use common::source_frontend::{check_sources, run_source};

fn linear_refusal(source: &str) {
    let error = check_sources(source, "staged-linear-code.cinch")
        .err()
        .expect("stored Code must preserve its declared linear parameter")
        .to_string();
    assert!(
        error.contains("%1") || error.contains("exactly once"),
        "{error}"
    );
}

fn carrier_refusal(source: &str) {
    let error = check_sources(source, "staged-code-carrier.cinch")
        .err()
        .expect("Code Int must not expand as a Boolean")
        .to_string();
    assert!(error.contains("Int") && error.contains("Bool"), "{error}");
}

#[test]
fn stored_stage_values_preserve_linear_parameter_usage() {
    assert_eq!(
        run_source(STAGE_ONCE_CONTROL).unwrap(),
        42
    );
    linear_refusal(STAGE_LINEAR_DUPLICATION);
    linear_refusal(STAGE_LINEAR_DROP);
}

#[test]
fn direct_and_explicit_declaration_controls_reject_duplicate_use() {
    linear_refusal(DIRECT_LINEAR_CONTROL);
    linear_refusal(STAGE_DECLARATION_CONTROL);
}

#[test]
fn code_application_preserves_its_function_operands_expectation() {
    linear_refusal(STAGE_APPLICATION_COMPOSITION);
}

#[test]
fn staged_splices_preserve_the_declared_code_carrier() {
    carrier_refusal(DIRECT_CODE_TYPE_CONTROL);
    carrier_refusal(STAGE_CODE_TYPE_EXPECTATION);
}

fn generic_declaration_source(argument: &str, marker: &str) -> String {
    r#"module Main where
  stage captureAs :: forall a. ARGUMENT %aff -> Syntax -> Code a
  stage captureAs ignored source = Syntax.capture source
  stage buildDecls :: forall a. ARGUMENT %aff -> Syntax -> Declarations
  stage buildDecls marker source =
    let code = captureAs marker source in
    quoteDecls where
      main :: Int
      main = if $(code) then 42 else 0
  macro project :: Syntax -> Declarations
  macro project source = buildDecls MARKER source
  generate do
    emit (project!(True))
"#
    .replace("ARGUMENT", argument)
    .replace("MARKER", marker)
}

#[test]
fn generic_declaration_producers_keep_the_bool_callers_type() {
    for (argument, marker) in [("a", "(True :: Bool)"), ("Code a", "[| True |]")] {
        assert_eq!(
            run_source(&generic_declaration_source(argument, marker))
            .unwrap(),
            42
        );
    }
}

#[test]
fn generic_declaration_producers_cannot_freshen_an_int_caller_to_bool() {
    for (argument, marker) in [("a", "(42 :: Int)"), ("Code a", "[| (42 :: Int) |]")] {
        carrier_refusal(&generic_declaration_source(argument, marker));
    }
}

const GENERIC_IDENTITY: &str = r#"module Main where
  stage identity :: forall a. Code (a -> a)
  stage identity = let binding = Syntax.newBinding (sym-to-name 'x) in Syntax.lambda binding (Syntax.reference binding)
"#;

#[test]
fn separate_stage_instantiations_can_choose_different_types() {
    let source = format!(
        "{GENERIC_IDENTITY}{}",
        r#"  generate do
    emit (quoteDecls where
      main :: Int
      main = if $(identity) True then $(identity) (42 :: Int) else 0)
"#
    );
    assert_eq!(
        run_source(&source).unwrap(),
        42
    );
    let producer = format!(
        "{GENERIC_IDENTITY}{}",
        r#"  stage declarations :: Declarations
  stage declarations = let make ignored = identity in quoteDecls where
    main :: Int
    main = if $(make ()) True then $(make ()) (42 :: Int) else 0
  generate do
    emit declarations
"#,
    );
    assert_eq!(run_source(&producer).unwrap(), 42);
}

#[test]
fn one_monomorphic_code_value_cannot_choose_two_parameter_types() {
    let source = format!(
        "{GENERIC_IDENTITY}{}",
        r#"  stage declarations :: Declarations
  stage declarations = let one = identity in quoteDecls where
    main :: Int
    main = if $(one) True then $(one) (42 :: Int) else 0
  generate do
    emit declarations
"#
    );
    carrier_refusal(&source);
    carrier_refusal(&source.replace("let one = identity", "let (one, marker) = (identity, ())"));
}

#[test]
fn partial_application_retains_the_generic_callers_type() {
    for (marker, valid) in [("(True :: Bool)", true), ("(42 :: Int)", false)] {
        let source = generic_declaration_source("a", marker).replace(
            "let code = captureAs marker source in",
            "let produce = captureAs marker in\n    let code = produce source in",
        );
        if valid {
            assert_eq!(
                run_source(&source).unwrap(),
                42
            );
        } else {
            carrier_refusal(&source);
        }
    }
}

#[test]
fn returned_stage_closures_retain_the_generic_callers_type() {
    for (marker, valid) in [("(True :: Bool)", true), ("(42 :: Int)", false)] {
        let source = generic_declaration_source("a", marker)
            .replace(
                "stage buildDecls :: forall a. a %aff -> Syntax -> Declarations",
                "stage buildDecls :: forall a. using (copy :: Copy a) => a %many -> Syntax -> Declarations",
            )
            .replace(
                "stage buildDecls marker source =",
                "stage buildDecls marker = \\source ->",
            );
        if valid {
            assert_eq!(
                run_source(&source).unwrap(),
                42
            );
        } else {
            carrier_refusal(&source);
        }
    }
}

#[test]
fn returned_stage_closures_cannot_capture_affine_arguments() {
    let source = generic_declaration_source("a", "(True :: Bool)").replace(
        "stage buildDecls marker source =",
        "stage buildDecls marker = \\source ->",
    );
    let error = check_sources(&source, "affine-stage-closure.cinch")
        .err()
        .expect("returning a closure does not permit unbounded uses of an affine capture")
        .to_string();
    assert!(
        error.contains("%aff") && error.contains("lambda"),
        "{error}"
    );
}

const STAGE_ONCE_CONTROL: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage code :: Code (Resource %1 -> Int)
  stage code = let binding = Syntax.newBinding (sym-to-name 'resource) in Syntax.lambda binding [| consume $(Syntax.reference binding) |]
  generate do
    emit (quoteDecls where
      main :: Int
      main = $(code) (Resource 42))
"####;

const DIRECT_LINEAR_CONTROL: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  fired :: Resource %1 -> Int
  fired resource = consume resource + consume resource
  main :: Int
  main = fired (Resource 21)
"####;

const STAGE_DECLARATION_CONTROL: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage code :: Code (Resource %1 -> Int)
  stage code = let binding = Syntax.newBinding (sym-to-name 'resource) in Syntax.lambda binding [| consume $(Syntax.reference binding) + consume $(Syntax.reference binding) |]
  generate do
    emit (quoteDecls where
      fired :: Resource %1 -> Int
      fired = $(code)
      main :: Int
      main = fired (Resource 21))
"####;

const DIRECT_CODE_TYPE_CONTROL: &str = r####"module Main where
  macro number :: Syntax -> Code Int
  macro number source = Syntax.capture source
  main :: Int
  main = if number!(True) then 42 else 0
"####;

const STAGE_LINEAR_DUPLICATION: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage code :: Code (Resource %1 -> Int)
  stage code = let binding = Syntax.newBinding (sym-to-name 'resource) in Syntax.lambda binding [| consume $(Syntax.reference binding) + consume $(Syntax.reference binding) |]
  generate do
    emit (quoteDecls where
      main :: Int
      main = $(code) (Resource 21))
"####;

const STAGE_LINEAR_DROP: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage code :: Code (Resource %1 -> Int)
  stage code = let binding = Syntax.newBinding (sym-to-name 'resource) in Syntax.lambda binding [| (42 :: Int) |]
  generate do
    emit (quoteDecls where
      main :: Int
      main = $(code) (Resource 0))
"####;

const STAGE_APPLICATION_COMPOSITION: &str = r####"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage code :: Code (Resource %1 -> Int)
  stage code = let binding = Syntax.newBinding (sym-to-name 'resource) in Syntax.lambda binding [| consume $(Syntax.reference binding) + consume $(Syntax.reference binding) |]
  stage applied :: Code Int
  stage applied = Syntax.apply code [| Resource 21 |]
  generate do
    emit (quoteDecls where
      main :: Int
      main = $(applied))
"####;

const STAGE_CODE_TYPE_EXPECTATION: &str = r####"module Main where
  macro number :: Syntax -> Code Int
  macro number source = Syntax.capture source
  stage wrong :: Code Int
  stage wrong = number!(True)
  generate do
    emit (quoteDecls where
      main :: Int
      main = if $(wrong) then 42 else 0)
"####;
