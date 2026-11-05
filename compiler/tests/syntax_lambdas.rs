//! Cinch libraries build native lambda binders around source-scoped bodies.
use cinch::driver::{check_sources, run_source_with, StagingCx};

fn run(source: &str) {
    assert_eq!(
        run_source_with(source, &mut StagingCx::default()).unwrap(),
        42
    );
}

#[test]
fn deliberate_lambda_binding_is_generic_and_nests_without_losing_outer_handles() {
    run(r#"module Main where
  macro function :: forall a b. Name -> Syntax -> Code (a -> b)
  macro function name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.captureUnder binding body)
  main :: Int
  main = if function!(x, x) True then (function!(x, function!(y, x + y)) 40) 2 else 0
"#);
}

#[test]
fn lambda_does_not_retarget_a_previously_captured_outer_reference() {
    run(r#"module Main where
  macro keep :: Name -> Syntax -> Code (Bool -> Int)
  macro keep name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.capture body)
  main :: Int
  main = let x = 40 in keep!(x, x + 2) False
"#);
}

#[test]
fn lambda_body_keeps_its_private_globals_and_source_fixities() {
    run(r#"module Operators where
  export (<#>)
  infixr 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) left right = left - right
module Library where
  export function, outer
  hidden :: Int
  hidden = 40
  macro function :: forall a b. Name -> Syntax -> Code (a -> b)
  macro function name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.captureUnder binding body)
  macro outer :: Syntax -> Code Int
  macro outer source = [| $(function!(x, hidden + x)) (2 :: Int) |]
module Main where
  import Operators
  import stage Library
  hidden :: Bool
  hidden = False
  main :: Int
  main = if outer!(0) == 42 then function!(x, x - (10 <#> 3 <#> 2)) 51 else 0
"#);
}

#[test]
fn lambda_can_be_built_as_a_stage_value_and_spliced_into_a_declaration() {
    run(r#"module Main where
  stage identity :: Code (Int -> Int)
  stage identity = let binding = Syntax.newBinding (sym-to-name 'x) in Syntax.lambda binding (Syntax.reference binding)
  generate do
    emit (quoteDecls where
      apply :: Int -> Int
      apply = $(identity))
  main :: Int
  main = apply 42
"#);
}

#[test]
fn lambda_preserves_linear_parameter_usage() {
    let source = r#"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  macro function :: Name -> Syntax -> Code (Resource %1 -> Int)
  macro function name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.captureUnder binding body)
  main :: Int
  main = function!(resource, consume resource) (Resource 42)
"#;
    run(source);
    for body in ["consume resource + consume resource", "42"] {
        let wrong = source.replace(
            "function!(resource, consume resource)",
            &format!("function!(resource, {body})"),
        );
        let error = check_sources(&wrong, "linear-syntax-lambda.cinch")
            .err()
            .expect("lambda construction must preserve linear usage checks")
            .to_string();
        assert!(
            error.contains("%1") || error.contains("exactly once"),
            "{error}"
        );
    }
}

#[test]
fn lambda_preserves_latent_effects_and_the_native_handler_boundary() {
    let source = r#"module Main where
  effect Answer where
    answer :: Unit ->{Answer} Int
  macro function :: Name -> Syntax -> Code (Unit ->{Answer} Int)
  macro function name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.captureUnder binding body)
  fired :: Unit ->{Answer} Int
  fired = function!(unit, answer unit)
  main :: Int
  main = 42
"#;
    check_sources(source, "latent-syntax-lambda.cinch").unwrap();
    let direct = source.replace(
        "fired = function!(unit, answer unit)",
        "fired = \\unit -> answer unit",
    );
    check_sources(&direct, "latent-direct-lambda.cinch").unwrap();
    for program in [source.to_string(), direct] {
        let wrong = program.replace("fired :: Unit ->{Answer} Int", "fired :: Unit -> Int");
        let error = check_sources(&wrong, "erased-syntax-lambda-effect.cinch")
            .err()
            .expect("the lambda body still needs its latent effect")
            .to_string();
        assert!(
            error.contains("pure") || error.contains("effect rows do not match"),
            "{error}"
        );
        let indirect = program.replace("main = 42", "main = handle fired () with (handler Answer where\n    return value -> value\n    answer unit resume next -> next 42)");
        let error = check_sources(&indirect, "indirect-lambda-handler.cinch")
            .err()
            .expect("source and generated lambdas obey the same bootstrap handler restriction")
            .to_string();
        assert!(
            error.contains("performed through a called computation"),
            "{error}"
        );
    }
    run(r#"module Main where
  effect Answer where
    answer :: Unit ->{Answer} Int
  macro function :: Name -> Syntax -> Code (Unit -> Int)
  macro function name body = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.captureUnder binding body)
  main :: Int
  main = function!(unit, handle answer unit with (handler Answer where
    return value -> value
    answer value resume next -> next 42)) ()
"#);
}

#[test]
fn code_macro_result_types_are_checked_at_insertion() {
    let source = r#"module Main where
  macro number :: Syntax -> Code Int
  macro number source = Syntax.capture source
  main :: Int
  main = number!(42)
"#;
    run(source);
    let wrong = source.replace(
        "main = number!(42)",
        "main = if number!(True) then 42 else 0",
    );
    assert!(check_sources(&wrong, "false-code-result.cinch").is_err());
}

#[test]
fn lambda_cannot_reintroduce_an_active_handle_or_escape_staging() {
    let reused = r#"module Main where
  macro bad :: Name -> Code (Int -> Int -> Int)
  macro bad name = let binding = Syntax.newBinding name in Syntax.lambda binding (Syntax.lambda binding (Syntax.reference binding))
  main :: Int
  main = bad!(x) 0 42
"#;
    let error = check_sources(reused, "duplicate-syntax-lambda.cinch")
        .err()
        .expect("one binding handle cannot be rebound beneath itself")
        .to_string();
    assert!(error.contains("introduced twice"), "{error}");
    let runtime = r#"module Main where
  main :: Int
  main = Syntax.lambda (Syntax.newBinding (sym-to-name 'x)) [| 42 |]
"#;
    let error = check_sources(runtime, "runtime-syntax-lambda.cinch")
        .err()
        .expect("lambda construction is compile-time only")
        .to_string();
    assert!(error.contains("compile-time only"), "{error}");
}
