//! Binding boundaries shared by native quotations and library syntax objects.
mod common;

use cinch::driver::StagingCx;
use common::source_frontend::{check_sources, run_source};
use cinch::syntax::{CapturedTerm, SyntaxScope, TermTemplate};
use cinch::{ast, layout, lexer, parser};

fn parse(source: &str) -> ast::Term {
    let tokens = layout::layout(lexer::lex(source).unwrap()).unwrap();
    let tokens: Vec<_> = tokens
        .into_iter()
        .filter(|t| !matches!(t.tok, lexer::Tok::Eof))
        .collect();
    parser::parse_term_tokens(&tokens, &parser::FixityTable::new()).unwrap()
}

fn check_syntax_tree(body: CapturedTerm) -> cinch::Result<cinch::core::Program> {
    check_syntax_tree_in(body, "")
}

fn check_syntax_tree_in(
    body: CapturedTerm,
    declarations: &str,
) -> cinch::Result<cinch::core::Program> {
    let prelude = cinch::driver::parse_prelude(&[])?;
    let parsed = cinch::driver::parse_source(
        &format!(
            "module Main where\n  x :: Int\n  x = 100\n{declarations}  main :: Int\n  main = 0\n"
        ),
        "scoped-syntax.cinch",
        &prelude.base,
        &prelude.exported_fixities,
    )?;
    let mut module = parsed.modules.into_iter().next().unwrap();
    let eq = module
        .decls
        .iter_mut()
        .find_map(|d| match d {
            ast::Decl::Equation { eq, .. } if eq.name.text == "main" => Some(eq),
            _ => None,
        })
        .unwrap();
    eq.body = body.into_datum();
    let cx = StagingCx {
        entry_modules: vec!["Main".into()],
        ..Default::default()
    };
    Ok(cinch::check::check_program_multi(&prelude.modules, &[module], &Default::default(), &cx)?.0)
}

fn scoped_parse(source: &str, scope: &SyntaxScope) -> CapturedTerm {
    let tokens = layout::layout(lexer::lex(source).unwrap()).unwrap();
    let tokens: Vec<_> = tokens
        .into_iter()
        .filter(|t| !matches!(t.tok, lexer::Tok::Eof))
        .collect();
    let prelude = cinch::driver::parse_prelude(&[]).unwrap();
    CapturedTerm::parse(&tokens, scope.clone(), &prelude.base).unwrap()
}

fn syntax_int(value: i32) -> CapturedTerm {
    CapturedTerm::new(parse(&value.to_string()), SyntaxScope::new("Main"))
}

#[test]
fn scoped_syntax_capture_keeps_outer_binding_under_same_spelled_inner_binder() {
    let span = ast::Span::new(7, 3);
    let root = SyntaxScope::new("Main");
    let (outer_scope, outer) = root.bind_value("x", span);
    let captured = outer_scope.reference("x", span).unwrap();
    let (_, inner) = outer_scope.bind_value("x", span);
    assert_ne!(outer.name().local_binding(), inner.name().local_binding());
    let body = outer.let_in(syntax_int(40), inner.let_in(syntax_int(2), captured));
    let program = check_syntax_tree(body).unwrap();
    assert_eq!(
        cinch::driver::run_program(&program, &cinch::core::TargetProfile::native()).unwrap(),
        40
    );
}

#[test]
fn scoped_syntax_deliberate_binding_uses_the_new_scope() {
    let span = ast::Span::new(8, 3);
    let root = SyntaxScope::new("Main");
    let (outer_scope, outer) = root.bind_value("x", span);
    let (inner_scope, inner) = outer_scope.bind_value("x", span);
    let body = scoped_parse("x", &inner_scope);
    let body = outer.let_in(syntax_int(40), inner.let_in(syntax_int(2), body));
    let program = check_syntax_tree(body).unwrap();
    assert_eq!(
        cinch::driver::run_program(&program, &cinch::core::TargetProfile::native()).unwrap(),
        2
    );
}

#[test]
fn scoped_syntax_out_of_scope_handle_cannot_fall_back_to_a_spelling() {
    let span = ast::Span::new(9, 3);
    let root = SyntaxScope::new("Main");
    let (gone, _) = root.bind_value("x", span);
    let captured = gone.reference("x", span).unwrap();
    let (_, other) = root.bind_value("x", span);
    let error = check_syntax_tree(other.let_in(syntax_int(2), captured))
        .err()
        .expect("an expired handle must be refused");
    assert!(
        error.message.contains("captured local `x` is not in scope"),
        "{error}"
    );
    assert!(SyntaxScope::new("Main").reference("x", span).is_err());
}

#[test]
fn scoped_syntax_parsing_respects_lexical_binders_and_sibling_scopes() {
    let root = SyntaxScope::new("Main");
    let (scope, outer) = root.bind_value("x", ast::Span::new(7, 3));
    for (source, expected) in [
        ("x", 40),
        ("let x = x + 2 in x", 42),
        ("(\\x -> x) 2 + x", 42),
        ("(case 2 of { x -> x }) + x", 42),
        ("let f x = x in f 2 + x", 42),
        ("letrec f n = if n == 0 then x else f (n - 1) in f 2", 40),
    ] {
        let body = outer.let_in(syntax_int(40), scoped_parse(source, &scope));
        let program = check_syntax_tree(body).unwrap_or_else(|e| panic!("{source}: {e}"));
        assert_eq!(
            cinch::driver::run_program(&program, &cinch::core::TargetProfile::native()).unwrap(),
            expected,
            "{source}"
        );
    }
}

#[test]
fn scoped_syntax_template_preparation_remaps_owned_binders_and_preserves_free_handles() {
    let root = SyntaxScope::new("Main");
    let span = ast::Span::new(7, 3);
    let (outer_scope, outer) = root.bind_value("x", span);
    let (_, inner) = outer_scope.bind_value("x", span);
    let captured = outer_scope.reference("x", span).unwrap();
    let tree = inner.let_in(syntax_int(2), captured);
    let template = TermTemplate::new(tree.into_datum(), 0, root).unwrap();
    let resolve = |_: &ast::RefSite| None;
    let mut serial = 0;
    let mut fresh = |name: &str| {
        serial += 1;
        format!("{name}${serial}")
    };
    let mut cx = ast::HygCx {
        resolve: &resolve,
        fresh: &mut fresh,
        own_values: vec![],
        own_types: vec![],
        owned_declarations: Vec::new(),
    };
    let output = template.prepare(&mut cx).unwrap().substitute(&[]).unwrap();
    let ast::Term::Let(decls, body, _) = output.datum() else {
        panic!("let")
    };
    let ast::LocalDecl::Bind {
        pat: ast::Pat::Var(binder, _),
        ..
    } = &decls[0]
    else {
        panic!("binder")
    };
    assert_ne!(binder.local_binding(), inner.name().local_binding());
    let ast::Term::Var(free) = &**body else {
        panic!("free capture")
    };
    assert_eq!(free.name.local_binding(), outer.name().local_binding());
    let program = check_syntax_tree(outer.let_in(syntax_int(40), output)).unwrap();
    assert_eq!(
        cinch::driver::run_program(&program, &cinch::core::TargetProfile::native()).unwrap(),
        40
    );
}

#[test]
fn scoped_syntax_parsed_linear_captures_keep_usage_obligations() {
    let root = SyntaxScope::new("Main");
    let declarations = "  data Token where\n    Token :: Int -> Token\n";
    let good =
        "((\\token -> case token of { Token value -> value }) :: Token %1 -> Int) (Token 42)";
    let ordinary = check_syntax_tree_in(CapturedTerm::new(parse(good), root.clone()), declarations);
    assert!(
        ordinary.is_ok(),
        "ordinary linear lambda: {}",
        ordinary.err().map_or("accepted".into(), |e| e.message)
    );
    let body = scoped_parse(good, &root);
    let program = check_syntax_tree_in(body, declarations).unwrap();
    assert_eq!(
        cinch::driver::run_program(&program, &cinch::core::TargetProfile::native()).unwrap(),
        42
    );
    for replacement in [
        "let first = token in let second = token in case first of { Token a -> case second of { Token b -> a + b } }",
        "let thunk u = case token of { Token value -> value } in thunk ()",
    ] {
        let bad = good.replace("case token of { Token value -> value }", replacement);
        let error = check_syntax_tree_in(scoped_parse(&bad, &root), declarations).err().expect("linear duplication or deferred capture is refused");
        assert!(error.message.contains("multiplicity %1"), "{error}");
    }
}

#[test]
fn scoped_syntax_captured_index_does_not_use_an_inner_values_facts() {
    let root = SyntaxScope::new("Main");
    let span = ast::Span::new(9, 3);
    let (outer_scope, outer) = root.bind_value("x", span);
    let (_, inner) = outer_scope.bind_value("x", span);
    for (wanted, accepted) in [(40, true), (0, false)] {
        let captured = outer_scope.reference("x", span).unwrap();
        let call = CapturedTerm::new(
            ast::Term::App(Box::new(parse("need")), vec![captured.into_datum()], span),
            root.clone(),
        );
        let then = inner.let_in(scoped_parse("(0 :: Nat)", &root), call);
        let branch = CapturedTerm::new(
            ast::Term::If {
                cond: Box::new(scoped_parse("x == 40", &outer_scope).into_datum()),
                then: Box::new(then.into_datum()),
                els: Box::new(parse("1")),
                span,
                cond_attrs: vec![],
                then_attrs: vec![],
                else_attrs: vec![],
            },
            outer_scope.clone(),
        );
        let body = outer.let_in(scoped_parse("(40 :: Nat)", &root), branch);
        let declarations =
            format!("  need :: using (n == {wanted}) => (n :: Nat) -> Int\n  need n = 42\n");
        let result = check_syntax_tree_in(body, &declarations);
        if let Ok(program) = &result {
            assert_eq!(
                cinch::driver::run_program(program, &cinch::core::TargetProfile::native()).unwrap(),
                42
            );
        }
        assert_eq!(
            result.is_ok(),
            accepted,
            "wanted {wanted}: {}",
            result.err().map_or("accepted".into(), |e| e.message)
        );
    }
}

#[test]
fn scoped_syntax_value_handles_are_not_reinterpreted_as_type_variables() {
    let root = SyntaxScope::new("Main");
    let (scope, outer) = root.bind_value("x", ast::Span::new(9, 3));
    let body = scoped_parse("(0 :: x)", &scope);
    let error = check_syntax_tree(outer.let_in(syntax_int(40), body))
        .err()
        .expect("value handles are not type-variable handles");
    assert!(
        error
            .message
            .contains("a captured lexical value is not a type variable"),
        "{error}"
    );
}

#[test]
fn scoped_type_indices_keep_their_value_binding_after_shadowing() {
    let root = SyntaxScope::new("Main");
    let span = ast::Span::new(9, 3);
    let (outer_scope, outer) = root.bind_value("x", span);
    let (inner_scope, inner) = outer_scope.bind_value("x", span);
    let declarations = "  data Tagged (n :: Nat) where\n    Tag :: Int -> Tagged n\n  read :: (n :: Nat) -> Tagged n -> Int\n  read n tag = case tag of { Tag value -> value }\n";
    let annotated = scoped_parse("(Tag 42 :: Tagged x)", &outer_scope);
    for (scope, accepted) in [(&outer_scope, true), (&inner_scope, false)] {
        let call = CapturedTerm::new(
            ast::Term::App(
                Box::new(parse("read")),
                vec![
                    scope.reference("x", span).unwrap().into_datum(),
                    annotated.datum().clone(),
                ],
                span,
            ),
            root.clone(),
        );
        let body = outer.let_in(
            scoped_parse("(40 :: Nat)", &root),
            inner.let_in(scoped_parse("(2 :: Nat)", &root), call),
        );
        let result = check_syntax_tree_in(body, declarations);
        assert_eq!(
            result.is_ok(),
            accepted,
            "captured index: {}",
            result
                .as_ref()
                .err()
                .map_or("accepted".into(), |e| e.message.clone())
        );
        if let Ok(program) = result {
            assert_eq!(
                cinch::driver::run_program(&program, &cinch::core::TargetProfile::native())
                    .unwrap(),
                42
            );
        }
    }
}

#[test]
fn native_macro_arguments_retain_caller_values_in_type_annotations() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  read :: (n :: Nat) -> Tagged n -> Int
  read n tag = case tag of { Tag value -> value }
  macro wrap :: Syntax -> Code Int
  macro wrap body = [| let n = 2 in $(body) |]
  use :: Nat -> Int
  use n = wrap!(read n (Tag 42 :: Tagged n))
  main :: Int
  main = use 40
"#;
    for body in [
        "wrap!(read n (Tag 42 :: Tagged n))",
        "let k = n in wrap!(read k (Tag 42 :: Tagged k))",
        "(\\k -> wrap!(read k (Tag 42 :: Tagged k))) n",
        "case n of { k -> wrap!(read k (Tag 42 :: Tagged k)) }",
        "letrec f k = wrap!(read k (Tag 42 :: Tagged k)) in f n",
        "f n\n    where\n      f k = wrap!(read k (Tag 42 :: Tagged k))",
    ] {
        let program = source.replace("wrap!(read n (Tag 42 :: Tagged n))", body);
        assert_eq!(
            run_source(&program)
                .unwrap_or_else(|e| panic!("{body}: {e}")),
            42
        );
    }
    let wrong = source.replace("wrap!(read n", "wrap!(read 2");
    assert!(check_sources(&wrong, "captured-index-mismatch.cinch").is_err());
}

#[test]
fn scoped_type_indices_check_kinds_and_expired_handles() {
    let root = SyntaxScope::new("Main");
    let span = ast::Span::new(9, 3);
    let (scope, binder) = root.bind_value("x", span);
    let declarations = "  data Tagged (n :: Nat) where\n    Tag :: Int -> Tagged n\n  consume :: forall (n :: Nat). Tagged n -> Int\n  consume tag = 42\n";
    let expression = scoped_parse("consume (Tag 42 :: Tagged x)", &scope);
    let wrong_kind = check_syntax_tree_in(
        binder.let_in(syntax_int(40), expression.clone()),
        declarations,
    )
    .err()
    .expect("Int cannot be used as a Nat index");
    assert!(
        wrong_kind
            .message
            .contains("captured index `x` has kind Int, expected Nat"),
        "{wrong_kind}"
    );
    let (_, unrelated) = root.bind_value("x", span);
    let expired = check_syntax_tree_in(
        unrelated.let_in(scoped_parse("(40 :: Nat)", &root), expression),
        declarations,
    )
    .err()
    .expect("a same-spelled binder is not the captured index");
    assert!(
        expired
            .message
            .contains("captured local `x` is not in scope"),
        "{expired}"
    );
}

#[test]
fn scoped_type_indices_require_the_declared_refined_kind() {
    let root = SyntaxScope::new("Main");
    let (scope, binder) = root.bind_value("x", ast::Span::new(9, 3));
    for kind in ["Fin 5", "InRange 0 5 Nat"] {
        let declarations = format!("  data Bounded (n :: {kind}) where\n    Bound :: Int -> Bounded n\n  consume :: forall (n :: {kind}). Bounded n -> Int\n  consume tag = 42\n");
        let body = scoped_parse("consume (Bound 42 :: Bounded x)", &scope);
        let result = check_syntax_tree_in(
            binder.let_in(scoped_parse("(40 :: Nat)", &root), body.clone()),
            &declarations,
        );
        let error = result
            .err()
            .expect("a raw Nat does not establish a refined kind");
        assert!(error.message.contains("captured index"), "{kind}: {error}");
        let ast::Term::Ann(_, function_type, _) = parse(&format!("(0 :: ({kind}) -> Int)")) else {
            panic!("function type")
        };
        let span = binder.name().span;
        let function = ast::Term::Ann(
            Box::new(ast::Term::Lam(
                vec![ast::Pat::Var(binder.name().clone(), span)],
                Box::new(body.into_datum()),
                span,
            )),
            function_type,
            span,
        );
        let body = CapturedTerm::new(
            ast::Term::Let(
                vec![ast::LocalDecl::Bind {
                    pat: ast::Pat::Wild(span),
                    body: function,
                    span,
                }],
                Box::new(parse("42")),
                span,
            ),
            root.clone(),
        );
        let valid =
            check_syntax_tree_in(body, &declarations).unwrap_or_else(|e| panic!("{kind}: {e}"));
        assert_eq!(
            cinch::driver::run_program(&valid, &cinch::core::TargetProfile::native()).unwrap(),
            42
        );
    }
}

#[test]
fn scoped_syntax_reintroducing_one_handle_as_two_binders_is_refused() {
    let root = SyntaxScope::new("Main");
    let span = ast::Span::new(9, 3);
    let (scope, binder) = root.bind_value("x", span);
    let body = scope.reference("x", span).unwrap();
    let error =
        check_syntax_tree(binder.let_in(syntax_int(40), binder.let_in(syntax_int(2), body)))
            .err()
            .expect("distinct introductions require distinct handles");
    assert!(error.message.contains("introduced twice"), "{error}");
}

#[test]
fn native_token_capture_keeps_private_source_global_under_caller_shadowing() {
    let source = r#"module Library where
  export outer
  hidden :: Int
  hidden = 40
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) + 2 |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!(hidden)
module Main where
  import Library (outer)
  hidden :: Int
  hidden = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_does_not_bind_an_unbound_name_at_insertion() {
    let source = r#"module Library where
  export outer
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!(missing)
module Main where
  import Library (outer)
  missing :: Int
  missing = 42
  main :: Int
  main = outer!(0)
"#;
    let error = check_sources(source, "unbound-source-token.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("captured source scope Library"), "{error}");
}

#[test]
fn native_token_capture_keeps_imported_member_and_module_aliases() {
    let source = r#"module Helpers where
  hidden :: Int
  hidden = 40
module Facade where
  import Helpers (hidden as exposed)
  export exposed
module Library where
  import Facade (exposed as chosen)
  export outer
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) + 2 |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!(chosen)
module Main where
  import Library (outer)
  chosen :: Int
  chosen = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let qualified = source
        .replace("import Facade (exposed as chosen)", "import Facade as F")
        .replace("inner!(chosen)", "inner!(F.exposed)");
    assert_eq!(
        run_source(&qualified).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_keeps_nested_macro_names_and_argument_scopes() {
    let source = r#"module Library where
  export outer
  hidden :: Int
  hidden = 40
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) + 2 |]
  macro relay :: Syntax -> Code Int
  macro relay arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = relay!(inner!(hidden))
module Main where
  import Library (outer)
  hidden :: Int
  hidden = 1
  macro inner :: Syntax -> Code Int
  macro inner ignored = [| 3 |]
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let caller_source = r#"module Library where
  export outer
  hidden :: Int
  hidden = 1
  macro inner :: Syntax -> Code Int
  macro inner ignored = [| 3 |]
  macro outer :: Syntax -> Code Int
  macro outer arg = [| $(arg) |]
module Main where
  import Library (outer)
  hidden :: Int
  hidden = 40
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) + 2 |]
  main :: Int
  main = outer!(inner!(hidden))
"#;
    assert_eq!(
        run_source(caller_source).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_builtin_forms_ignore_insertion_site_shadowing() {
    let source = r#"module Library where
  export outer
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!(force (delay (\() -> 42)))
module Main where
  import Library (outer)
  force :: Int -> Int
  force ignored = 1
  delay :: (Unit -> Int) -> Int
  delay ignored = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_provider_pure_ignores_insertion_site_shadowing() {
    let source = r#"module Library where
  export outer
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!(pure 42)
module Main where
  import Library (outer)
  pure :: Int -> Int
  pure ignored = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_keeps_private_type_and_constructor_authority() {
    let source = r#"module Library where
  export outer
  data HiddenBox where
    Hide :: Int -> HiddenBox
  macro inner :: Syntax -> Code Int
  macro inner arg = [| case ($(arg) :: HiddenBox) of { Hide value -> value + 2 } |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!((Hide 40 :: HiddenBox))
module Main where
  import Library (outer)
  data HiddenBox where
    Hide :: Int -> HiddenBox
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let forged = r#"module Library where
  export outer
  data HiddenBox where
    Hide :: Int -> HiddenBox
  macro outer :: Syntax -> Code Int
  macro outer arg = [| case ($(arg) :: HiddenBox) of { Hide value -> value + 2 } |]
module Main where
  import Library (outer)
  main :: Int
  main = outer!(Hide 40)
"#;
    let error = check_sources(forged, "captured-private-constructor.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("captured source scope Main"), "{error}");
}

#[test]
fn native_token_capture_retains_type_directed_constructor_alternatives() {
    let source = r#"module Library where
  export outer
  macro outer :: Syntax -> Code Int
  macro outer arg = [| case ($(arg) :: Vec 0 Int) of { Nil -> 42 } |]
module Main where
  import Library (outer)
  main :: Int
  main = outer!(Nil)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_token_capture_nested_arguments_keep_the_sources_imported_fixity() {
    let source = r#"module Operators where
  infixr 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) a b = a * 10 + b
module Library where
  export outer
  infixl 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) a b = 0
  macro inner :: Syntax -> Code Int
  macro inner ignored = [| 0 |]
  macro outer :: Syntax -> Code Int
  macro outer arg = [| $(arg) |]
module Main where
  import Operators
  import Library (outer)
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  main :: Int
  main = outer!(inner!(1 <#> 2 <#> 3))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        33
    );
}

#[test]
fn native_captured_index_keeps_its_global_definition() {
    let source = r#"module Library where
  export outer
  limit :: Nat
  limit = 40
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  read :: Tagged Library.limit -> Int
  read tag = case tag of { Tag value -> value }
  macro inner :: Syntax -> Code Int
  macro inner arg = [| read $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!((Tag 42 :: Tagged limit))
module Main where
  import Library (outer)
  limit :: Nat
  limit = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let qualified = source.replace("Tagged limit))", "Tagged Library.limit))");
    assert_eq!(
        run_source(&qualified).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_quantified_value_shadows_same_named_global() {
    let source = r#"module Main where
  limit :: Nat
  limit = 1
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  read :: forall (n :: Nat). Tagged n -> Int
  read tag = case tag of { Tag value -> value }
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| let limit = 2 in $(arg) |]
  use :: forall (limit :: Nat). Tagged limit -> Int
  use tag = wrap!(read (tag :: Tagged limit))
  main :: Int
  main = use (Tag 42 :: Tagged 40)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_unbound_name_cannot_acquire_caller_definition() {
    let source = r#"module Library where
  export outer
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!((case (Tag 42 :: Tagged missing) of { Tag value -> value }))
module Main where
  import Library (outer)
  missing :: Nat
  missing = 40
  main :: Int
  main = outer!(0)
"#;
    let error = check_sources(source, "unbound-captured-index.cinch")
        .err()
        .expect("the captured index was unbound at its source")
        .to_string();
    assert!(error.contains("captured source scope Library"), "{error}");
}

#[test]
fn native_captured_index_uses_the_named_functions_argument_carrier() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: Int -> Nat
  measure ignored = 40
  read :: (payload :: Int) -> Tagged (measure payload) -> Int
  read payload tag = case tag of { Tag value -> value }
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) |]
  use :: Int -> Int
  use payload = wrap!(read payload (Tag 42 :: Tagged (measure payload)))
  main :: Int
  main = use 7
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_builtin_observations_keep_their_argument_carriers() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  read :: (payload :: Bytes) -> Tagged (length payload) -> Int
  read payload tag = case tag of { Tag value -> value }
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) |]
  use :: (payload :: Bytes) -> Tagged (length payload) -> Int
  use payload tag = wrap!((let length :: Bytes -> Nat; length ignored = 0 in read payload (tag :: Tagged (length payload))))
  main :: Int
  main = let payload = bytes-new 3 in use payload (Tag 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let word = source
        .replace("Bytes", "U32")
        .replace("length", "toNat")
        .replace(
            "let payload = bytes-new 3 in use payload (Tag 42)",
            "use 3 (Tag 42)",
        );
    assert_eq!(
        run_source(&word).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_does_not_freeze_owned_dependent_binders_as_globals() {
    let source = r#"module Main where
  limit :: Nat
  limit = 40
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  unpack :: (Sigma (n :: Nat). Tagged n) -> Int
  unpack pair = case pair of { (n, Tag value) -> value }
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) |]
  main :: Int
  main = wrap!(unpack ((40, Tag 42) :: Sigma (limit :: Nat). Tagged limit))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_function_values_retain_their_binding_when_applied() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  bump :: Nat -> Nat
  bump n = n + 1
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| let f :: Nat -> Nat; f n = 0 in $(arg) |]
  use :: (f :: Nat -> Nat) -> (n :: Nat) -> Tagged (f n) -> Int
  use f n tag = wrap!((case (tag :: Tagged (f n)) of { Tag value -> value }))
  main :: Int
  main = use bump 40 (Tag 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let wrong = source
        .replace(
            "(n :: Nat) -> Tagged (f n)",
            "(g :: Nat -> Nat) -> (n :: Nat) -> Tagged (f n)",
        )
        .replace("use f n tag", "use f g n tag")
        .replace("tag :: Tagged (f n)", "tag :: Tagged (g n)")
        .replace("use bump 40", "use bump bump 40");
    let error = check_sources(&wrong, "distinct-captured-index-functions.cinch")
        .err()
        .expect("a different function cannot establish the tag's index")
        .to_string();
    assert!(error.contains("index mismatch"), "{error}");
    let effectful = source.replace(
        "use :: (f :: Nat -> Nat)",
        "use :: (f :: Nat ->{Diverge} Nat)",
    );
    let error = check_sources(&effectful, "effectful-captured-index-function.cinch")
        .err()
        .expect("a pure generated helper cannot conceal the captured function's effects")
        .to_string();
    assert!(
        error.contains("index function application must be pure"),
        "{error}"
    );
}

#[test]
fn native_captured_index_global_carrier_cannot_use_the_callers_kind() {
    let source = r#"module Library where
  export inner, outer, Tagged, Tag
  limit :: Int
  limit = -1
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  macro outer :: Syntax -> Code Int
  macro outer ignored = inner!((case (Tag 42 :: Tagged limit) of { Tag value -> value }))
module Main where
  import Library (inner, outer, Tagged, Tag)
  limit :: Nat
  limit = 1
  main :: Int
  main = outer!(0)
"#;
    let caller = source.replace(
        "main = outer!(0)",
        "main = inner!((case (Tag 42 :: Tagged limit) of { Tag value -> value }))",
    );
    assert_eq!(
        run_source(&caller).unwrap(),
        42
    );
    let error = check_sources(source, "captured-global-index-kind.cinch")
        .err()
        .expect("the captured Int definition cannot be used as a Nat index")
        .to_string();
    assert!(error.contains("index") && error.contains("kind"), "{error}");
}

#[test]
fn native_captured_index_polymorphic_function_keeps_argument_carriers() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: forall a. a -> Nat
  measure value = let unit = ignore value in 40
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) |]
  use :: (payload :: Int) -> Tagged (measure payload) -> Int
  use payload tag = wrap!((case (tag :: Tagged (measure payload)) of { Tag value -> value }))
  main :: Int
  main = use 7 (Tag 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let bytes = source
        .replace("(payload :: Int)", "(payload :: Bytes)")
        .replace(
            "main = use 7 (Tag 42)",
            "main = let payload = bytes-new 3 in use payload (Tag 42)",
        );
    assert_eq!(
        run_source(&bytes).unwrap(),
        42
    );
    let nested = source
        .replace(
            "  measure ::",
            "  identity :: forall a. a -> a\n  identity value = value\n  measure ::",
        )
        .replace("measure payload", "measure (identity payload)");
    assert_eq!(
        run_source(&nested).unwrap(),
        42
    );
    let shadow = source.replace(
        "macro wrap arg = [| $(arg) |]",
        "macro wrap arg = [| let payload :: Nat; payload = 0 in $(arg) |]",
    );
    assert_eq!(
        run_source(&shadow).unwrap(),
        42
    );
    let independent = source
        .replace(
            "  main :: Int",
            r#"  use-bytes :: (payload :: Bytes) -> Tagged (measure payload) -> Int
  use-bytes payload tag = wrap!((case (tag :: Tagged (measure payload)) of { Tag value -> value }))
  main :: Int"#,
        )
        .replace(
            "main = use 7 (Tag 42)",
            "main = let bytes = bytes-new 3 in use 7 (Tag 20) + use-bytes bytes (Tag 22)",
        );
    assert_eq!(
        run_source(&independent).unwrap(),
        42
    );
}

#[test]
fn native_captured_index_polymorphic_arguments_share_their_type_parameter() {
    let source = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: forall a. a -> a -> Nat
  measure first second = let unit = ignore first in let unit2 = ignore second in 40
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) |]
  use :: (payload :: Int) -> (other :: Bytes) -> Tagged (measure payload payload) -> Int
  use payload other tag = wrap!((case (tag :: Tagged (measure payload payload)) of { Tag value -> value }))
  main :: Int
  main = let other = bytes-new 3 in use 7 other (Tag 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let wrong = source.replace(
        "tag :: Tagged (measure payload payload)",
        "tag :: Tagged (measure payload other)",
    );
    let error = check_sources(&wrong, "inconsistent-generic-index-carriers.cinch")
        .err()
        .expect("one generic parameter cannot take both Int and Bytes")
        .to_string();
    assert!(error.contains("index function argument carrier"), "{error}");
    let numeric = source
        .replace("(payload :: Int)", "(payload :: Bytes)")
        .replace("use 7 other", "use other other")
        .replace(
            "tag :: Tagged (measure payload payload)",
            "tag :: Tagged (measure payload 3)",
        );
    let error = check_sources(&numeric, "non-numeric-generic-index-operand.cinch")
        .err()
        .expect("a numeric literal cannot acquire a Bytes carrier")
        .to_string();
    assert!(
        error.contains("index literal needs a numeric carrier"),
        "{error}"
    );
}

#[test]
fn nullary_native_macros_keep_definition_scope_and_unary_empty_input() {
    let source = r#"module Library where
  export number, empty
  hidden :: Int
  hidden = 40
  macro helper :: Syntax -> Code Int
  macro helper source = Syntax.capture source
  macro number :: Code Int
  macro number = helper!(hidden + 2)
  macro empty :: Syntax -> Code Int
  macro empty source = [| 42 |]
module Main where
  import stage Library
  hidden :: Int
  hidden = 0
  main :: Int
  main = if empty!() == 42 then number!() else 0
"#;
    assert_eq!(run_source(source).unwrap(), 42);
    let wrong = source.replace("number!()", "number!(hidden)");
    let error = check_sources(&wrong, "nullary-macro-with-an-argument.cinch")
        .err().expect("a nullary macro cannot silently discard input").to_string();
    assert!(error.contains("takes 0 token-tree argument(s)"), "{error}");
}

#[test]
fn native_syntax_api_once_preserves_rhs_and_intentional_body_binding() {
    let source = r#"module Library where
  export once
  macro once :: Syntax -> Name -> Syntax -> Code Int
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
module Main where
  import stage Library (once)
  main :: Int
  main = let x = 40 in once!(x + 1, x, x + 1)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_api_generic_once_supports_nested_calls_and_distinct_types() {
    let source = r#"module Main where
  macro once :: forall a. Syntax -> Name -> Syntax -> Code a
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
  main :: Int
  main = if once!(True, flag, flag) then once!(40, x, once!(x + 1, x, x + 1)) else 0
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_api_capture_keeps_outer_binding_and_reference_uses_the_handle() {
    let source = r#"module Main where
  macro keep :: Syntax -> Name -> Code Int
  macro keep arg name = let binding = Syntax.newBinding name in Syntax.letIn binding [| (2 :: Int) |] (Syntax.capture arg)
  macro useNew :: Name -> Code Int
  macro useNew name = let binding = Syntax.newBinding name in Syntax.letIn binding [| 42 |] (Syntax.reference binding)
  main :: Int
  main = let x = 40 in keep!(x, x) + useNew!(x) - 40
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_api_scope_extension_retains_source_private_authority() {
    let source = r#"module Library where
  export outer
  hidden :: Int
  hidden = 40
  macro once :: forall a. Syntax -> Name -> Syntax -> Code a
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
  macro outer :: Syntax -> Code Int
  macro outer ignored = once!(1, x, x + hidden + 1)
module Main where
  import stage Library (outer)
  hidden :: Int
  hidden = 1
  main :: Int
  main = outer!(0)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_api_escaped_or_reintroduced_handles_are_refused() {
    let source = r#"module Main where
  x :: Int
  x = 42
  macro bad :: Name -> Code Int
  macro bad name = Syntax.reference (Syntax.newBinding name)
  main :: Int
  main = bad!(x)
"#;
    let error = check_sources(source, "escaped-native-syntax-binding.cinch")
        .err()
        .expect("a textual global cannot supply a missing lexical handle")
        .to_string();
    assert!(
        error.contains("captured local `x` is not in scope"),
        "{error}"
    );
    let duplicate = source.replace("Syntax.reference (Syntax.newBinding name)",
        "let binding = Syntax.newBinding name in Syntax.letIn binding [| 40 |] (Syntax.letIn binding [| 2 |] (Syntax.reference binding))");
    let error = check_sources(&duplicate, "reintroduced-native-syntax-binding.cinch")
        .err()
        .expect("one lexical handle cannot bind two nested variables")
        .to_string();
    assert!(error.contains("introduced twice"), "{error}");
    let recursive = source.replace("Syntax.reference (Syntax.newBinding name)",
        "let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.reference binding) [| 42 |]");
    let error = check_sources(&recursive, "native-syntax-binding-in-rhs.cinch")
        .err()
        .expect("the binding is not in scope in its own RHS")
        .to_string();
    assert!(
        error.contains("captured local `x` is not in scope"),
        "{error}"
    );
    let splice = source.replace(
        "Syntax.reference (Syntax.newBinding name)",
        "let binding = Syntax.newBinding name in [| $(binding) |]",
    );
    let error = check_sources(&splice, "native-syntax-binding-is-not-code.cinch")
        .err()
        .expect("a binding cannot be reified as ordinary code")
        .to_string();
    assert!(error.contains("SyntaxBinding"), "{error}");
}

#[test]
fn native_syntax_api_once_preserves_linear_use_and_rejects_duplication_and_drop() {
    let source = r#"module Main where
  data Resource where
    Resource :: Int -> Resource
  macro once :: forall a. Syntax -> Name -> Syntax -> Code a
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
  use :: Resource %1 -> Int
  use resource = once!(resource, item, case item of { Resource value -> value })
  main :: Int
  main = use (Resource 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    for body in [
        "(case item of { Resource value -> value }) + (case item of { Resource value -> value })",
        "42",
    ] {
        let wrong = source.replace("case item of { Resource value -> value }", body);
        let error = check_sources(&wrong, "native-syntax-linear-refusal.cinch")
            .err()
            .expect("structural syntax must retain linear restrictions")
            .to_string();
        assert!(
            error.contains("%1") || error.contains("exactly once"),
            "{error}"
        );
    }
}

#[test]
fn native_syntax_api_once_evaluates_allocating_rhs_once() {
    let source = r#"module Main where
  macro once :: forall a. Syntax -> Name -> Syntax -> Code a
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
  main :: Int
  main = once!(bytes-new 1, buffer, let unit = bytes-set buffer 0 42 in nat-to-int (bytes-get buffer 0))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let out_of_bounds = source.replace("bytes-new 1", "bytes-new 0");
    let error = check_sources(&out_of_bounds, "native-syntax-empty-buffer.cinch")
        .err()
        .expect("preserving a real length fact must not invent one for an empty buffer")
        .to_string();
    assert!(error.contains("0 <") && error.contains("evidence"), "{error}");
}

#[test]
fn native_syntax_api_operates_through_stage_values_and_declaration_splices() {
    let source = r#"module Main where
  stage answerCode :: Code Int
  stage answerCode = let binding = Syntax.newBinding (sym-to-name 'answer) in Syntax.letIn binding [| 42 |] (Syntax.reference binding)
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(answerCode))
  main :: Int
  main = answer
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_api_cannot_escape_staging_or_bypass_generated_typechecking() {
    let runtime = r#"module Main where
  main :: Int
  main = Syntax.reference (Syntax.newBinding (sym-to-name 'x))
"#;
    let error = check_sources(runtime, "runtime-syntax-api.cinch")
        .err()
        .expect("scoped syntax operations exist only at compile time")
        .to_string();
    assert!(error.contains("compile-time only"), "{error}");
    let wrong = r#"module Main where
  macro once :: forall a. Syntax -> Name -> Syntax -> Code a
  macro once rhs name body = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture rhs) (Syntax.captureUnder binding body)
  main :: Int
  main = once!(40, x, True)
"#;
    assert!(check_sources(wrong, "ill-typed-native-syntax.cinch").is_err());
    let forged = r#"module Main where
  data SyntaxBinding a where
    Forge :: SyntaxBinding a
  main :: Int
  main = 0
"#;
    assert!(check_sources(forged, "forged-syntax-binding.cinch").is_err());
}

#[test]
fn native_syntax_api_capture_preserves_nested_imported_fixities() {
    let source = r#"module Operators where
  infixr 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) a b = a * 10 + b
module Library where
  export outer
  infixl 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) a b = 0
  macro inner :: Syntax -> Code Int
  macro inner ignored = [| 0 |]
  macro outer :: Syntax -> Code Int
  macro outer arg = Syntax.capture arg
module Main where
  import Operators
  import Library (outer)
  macro inner :: Syntax -> Code Int
  macro inner arg = [| $(arg) |]
  main :: Int
  main = outer!(inner!(1 <#> 2 <#> 3))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        33
    );
}

#[test]
fn native_syntax_apply_resolves_overloaded_helpers_after_operand_capture() {
    let source = r#"module Main where
  signature Count a where
    count :: a -> Int
  implicit instance CountInt :: Count Int where
    count value = value
  implicit instance CountBool :: Count Bool where
    count value = if value then 42 else 0
  macro helper :: forall a. Syntax -> Code a
  macro helper source = Syntax.capture source
  macro applyCount :: Syntax -> Code Int
  macro applyCount value = Syntax.apply (helper!(count)) (Syntax.capture value)
  main :: Int
  main = applyCount!(True) + applyCount!((21 :: Int)) - 21
"#;
    let direct = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("macro "))
        .collect::<Vec<_>>()
        .join("\n")
        .replace("applyCount!(True)", "count True")
        .replace("applyCount!((21 :: Int))", "count (21 :: Int)");
    assert_eq!(
        run_source(&direct).unwrap(),
        42
    );
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_apply_keeps_private_definition_helpers_and_caller_locals() {
    let source = r#"module Library where
  export invoke, apply
  hidden :: Int -> Int
  hidden value = value + 1
  macro helper :: forall a. Syntax -> Code a
  macro helper source = Syntax.capture source
  macro invoke :: Syntax -> Code Int
  macro invoke value = Syntax.apply (helper!(hidden)) (Syntax.capture value)
  macro apply :: forall a b. Syntax -> Syntax -> Code b
  macro apply function value = Syntax.apply (Syntax.capture function) (Syntax.capture value)
module Main where
  import stage Library as Macros
  hidden :: Int -> Int
  hidden value = 0
  main :: Int
  main = let step value = value + 1 in Macros.apply!(step, Macros.invoke!(40))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_apply_does_not_retarget_captured_code_under_a_generated_binder() {
    let source = r#"module Main where
  macro bind :: Syntax -> Name -> Code Int
  macro bind function name = let binding = Syntax.newBinding name in Syntax.letIn binding [| (0 :: Int) |] (Syntax.apply (Syntax.capture function) [| (41 :: Int) |])
  main :: Int
  main = let step value = value + 1 in bind!(step, step)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn native_syntax_apply_accepts_typed_linear_arrows_and_preserves_linear_use() {
    let source = r#"module Main where
  data Resource where
    Resource :: Int -> Resource
  consume :: Resource %1 -> Int
  consume resource = case resource of Resource value -> value
  stage apply :: Code (Resource %1 -> Int) -> Code Resource -> Code Int
  stage apply function value = Syntax.apply function value
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(apply [| consume |] [| Resource 42 |]))
  macro invoke :: Syntax -> Syntax -> Code Int
  macro invoke function value = Syntax.apply (Syntax.capture function) (Syntax.capture value)
  use :: Resource %1 -> Int
  use resource = invoke!(consume, resource)
  main :: Int
  main = if answer == 42 then use (Resource 42) else 0
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let duplicate = source.replace(
        "macro invoke function value = Syntax.apply (Syntax.capture function) (Syntax.capture value)",
        "macro invoke function value = let applied = Syntax.apply (Syntax.capture function) (Syntax.capture value) in [| $(applied) + $(applied) |]",
    );
    let error = check_sources(&duplicate, "duplicate-syntax-application.cinch")
        .err()
        .expect("assembled applications must not duplicate a linear capture")
        .to_string();
    assert!(
        error.contains("%1") || error.contains("exactly once"),
        "{error}"
    );
}

#[test]
fn native_syntax_apply_preserves_latent_effects_and_requires_handlers() {
    let source = r#"module Main where
  effect Answer where
    answer :: Unit ->{Answer} Int
  macro operation :: Syntax -> Code (Unit ->{Answer} Int)
  macro operation source = Syntax.capture source
  stage apply :: Code (Unit ->{Answer} Int) -> Code Unit -> Code Int
  stage apply function value = Syntax.apply function value
  generate do
    emit (quoteDecls where
      main :: Int
      main = handle $(apply (operation!(answer)) [| () |]) with (handler Answer where
        return value -> value
        answer unit resume next -> next 42))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let unhandled = source.replace(
        "handle $(apply (operation!(answer)) [| () |]) with (handler Answer where\n        return value -> value\n        answer unit resume next -> next 42)",
        "$(apply (operation!(answer)) [| () |])",
    );
    let error = check_sources(&unhandled, "unhandled-syntax-application.cinch")
        .err()
        .expect("syntax construction must not hide a performed effect")
        .to_string();
    assert!(
        error.contains("pure") || error.contains("effect rows do not match"),
        "{error}"
    );
    let erased_carrier = source.replace("Code (Unit ->{Answer} Int)", "Code (Unit -> Int)");
    let error = check_sources(&erased_carrier, "false-pure-operation-code.cinch")
        .err()
        .expect("a surrounding handler does not make a falsely pure Code carrier valid")
        .to_string();
    assert!(error.contains("pure") || error.contains("effect rows do not match"), "{error}");
}

#[test]
fn native_syntax_apply_cannot_escape_staging_or_apply_ill_typed_code() {
    let runtime = r#"module Main where
  main :: Int
  main = Syntax.apply [| (\value -> value) |] [| (42 :: Int) |]
"#;
    let error = check_sources(runtime, "runtime-syntax-application.cinch")
        .err()
        .expect("syntax assembly is compile-time only")
        .to_string();
    assert!(error.contains("compile-time only"), "{error}");
    let wrong = r#"module Main where
  next :: Int -> Int
  next value = value + 1
  macro invoke :: Syntax -> Syntax -> Code Int
  macro invoke function value = Syntax.apply (Syntax.capture function) (Syntax.capture value)
  main :: Int
  main = invoke!(next, True)
"#;
    let error = check_sources(wrong, "ill-typed-syntax-application.cinch")
        .err()
        .expect("the assembled AST must still pass ordinary checking")
        .to_string();
    assert!(error.contains("Int") && error.contains("Bool"), "{error}");
}
#[test]
fn native_syntax_record_keeps_definition_constructor_and_source_fields() {
    let source = r#"module Operators where
  infixr 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) left right = left - right
module Factory where
  export create
  macro inc :: Syntax -> Code Int
  macro inc ignored = [| 0 |]
  infixl 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) left right = 0
  payload :: Int
  payload = 0
  record ItemProps where
    value :: Int
  inspect-Item :: ItemProps -> Int
  inspect-Item props = props.value
  macro create :: Name -> Syntax -> Code Int
  macro create tag fields = Syntax.apply (Syntax.definition (Name.prefix "inspect-" tag)) (Syntax.recordValue (Name.append tag (sym-to-name 'Props)) fields)
module Main where
  import Operators
  import stage Factory
  macro inc :: Syntax -> Code Int
  macro inc source = Syntax.capture source
  record ItemProps where
    value :: Bool
  inspect-Item :: ItemProps -> Int
  inspect-Item props = 0
  main :: Int
  main = let payload = 33 in create!(Item, { value = inc!(payload + (10 <#> 3 <#> 2)) })
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let unknown_field = source.replace("{ value = inc!", "{ absent = inc!");
    let error = check_sources(&unknown_field, "unknown-native-record-field.cinch")
        .err()
        .expect("the native record must retain its field schema")
        .to_string();
    assert!(error.contains("unknown record field absent"), "{error}");
    let wrong_field = source.replace("payload + (10 <#> 3 <#> 2)", "True");
    let error = check_sources(&wrong_field, "wrong-native-record-field-type.cinch")
        .err()
        .expect("the native record must retain its field type")
        .to_string();
    assert!(error.contains("Bool") && error.contains("Int"), "{error}");
}

#[test]
fn native_syntax_definition_uses_actual_macro_scope_without_private_name_forgery() {
    let source = r#"module Vault where
  export public
  public :: Int
  public = 42
  hiddenValue :: Int
  hiddenValue = 42
module Factory where
  import Vault
  export read
  macro read :: Name -> Code Int
  macro read name = Syntax.definition name
module Main where
  import stage Factory
  public :: Int
  public = 0
  hiddenValue :: Int
  hiddenValue = 42
  main :: Int
  main = read!(public)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let forbidden = source.replace("main = read!(public)", "main = read!(hiddenValue)");
    let error = check_sources(&forbidden, "forged-definition-name.cinch")
        .err()
        .expect("descriptive Names cannot acquire private imports or caller bindings")
        .to_string();
    assert!(
        error.contains("hiddenValue")
            && error.contains("not bound in captured source scope Factory"),
        "{error}"
    );
}
fn prepare(source: &str) -> cinch::syntax::PreparedTerm {
    let mut datum = parse(source);
    let mut holes = Vec::new();
    ast::collect_template_holes(&mut datum, &mut holes);
    let template = TermTemplate::new(datum, holes.len(), SyntaxScope::new("Library")).unwrap();
    let resolve = |site: &ast::RefSite| {
        (site.name == "helper").then(|| ("Library".into(), site.name.clone()))
    };
    let mut serial = 0;
    let mut fresh = |name: &str| {
        serial += 1;
        format!("{name}${serial}")
    };
    let mut cx = ast::HygCx {
        resolve: &resolve,
        fresh: &mut fresh,
        own_values: vec![],
        own_types: vec![],
        owned_declarations: Vec::new(),
    };
    template.prepare(&mut cx).unwrap()
}

#[test]
fn syntax_objects_splice_captured_trees_after_template_binders_are_fresh() {
    let captured = CapturedTerm::new(parse("helper"), SyntaxScope::new("Caller"));
    let span = captured.datum().span();
    let output = prepare("let helper = 2 in ($(captured), helper)")
        .substitute(&[captured.clone()])
        .unwrap();
    let ast::Term::Let(decls, body, _) = output.datum() else {
        panic!("a let tree")
    };
    let name = match &decls[0] {
        ast::LocalDecl::Eq { name, .. }
        | ast::LocalDecl::Bind {
            pat: ast::Pat::Var(name, _),
            ..
        } => name,
        _ => panic!("a fresh binder"),
    };
    let ast::Term::Tuple(items, _) = &**body else {
        panic!("a pair")
    };
    let ast::Term::Var(caller) = &items[0] else {
        panic!("captured caller reference")
    };
    let ast::Term::Var(generated) = &items[1] else {
        panic!("generated reference")
    };
    assert_eq!(caller.name.text, "helper");
    assert_eq!(caller.name.span, span);
    assert_eq!(generated.name.text, name.text);
    assert_ne!(name.text, caller.name.text);
    assert_eq!(captured.scope().module(), "Caller");
}

#[test]
fn syntax_objects_keep_definition_site_helpers_separate_from_captures() {
    let output = prepare("(helper, $(captured))")
        .substitute(&[CapturedTerm::new(
            parse("helper"),
            SyntaxScope::new("Caller"),
        )])
        .unwrap();
    let ast::Term::Tuple(items, _) = output.datum() else {
        panic!("a pair")
    };
    let ast::Term::Var(helper) = &items[0] else {
        panic!("definition-site helper")
    };
    let ast::Term::Var(captured) = &items[1] else {
        panic!("call-site capture")
    };
    assert_eq!(helper.quals[0].text, "Library");
    assert!(captured.quals.is_empty());
}

#[test]
fn syntax_objects_refuse_out_of_range_slots_and_wrong_capture_counts() {
    let datum = ast::Term::SpliceHole {
        idx: 1,
        span: ast::Span::new(4, 9),
    };
    let error = TermTemplate::new(datum, 1, SyntaxScope::new("Library")).unwrap_err();
    assert!(error.message.contains("outside 1 capture slots"));
    assert_eq!(error.span.unwrap().line(), 4);
    assert!(prepare("$(value)")
        .substitute(&[])
        .unwrap_err()
        .message
        .contains("expects 1 captures"));
    let captured = CapturedTerm::new(parse("helper"), SyntaxScope::new("Caller"));
    assert!(prepare("helper")
        .substitute(&[captured])
        .unwrap_err()
        .message
        .contains("expects 0 captures"));
}

#[test]
fn failed_template_walk_restores_its_lexical_scope() {
    let resolve = |site: &ast::RefSite| {
        (site.name == "helper").then(|| ("Library".into(), site.name.clone()))
    };
    let mut serial = 0;
    let mut fresh = |name: &str| {
        serial += 1;
        format!("{name}${serial}")
    };
    let mut cx = ast::HygCx {
        resolve: &resolve,
        fresh: &mut fresh,
        own_values: vec![],
        own_types: vec![],
        owned_declarations: Vec::new(),
    };
    let mut scope = ast::HygScope::new();
    assert!(ast::hyg_term(&parse("let helper = 2 in missing"), &mut cx, &mut scope).is_err());
    let ast::Term::Var(helper) = ast::hyg_term(&parse("helper"), &mut cx, &mut scope).unwrap()
    else {
        panic!("definition-site helper")
    };
    assert_eq!(helper.quals[0].text, "Library");
    assert_eq!(helper.name.text, "helper");
}

fn run_quote(expression: &str, expected: i32) {
    let source = format!(
        r#"module Main where
  helper :: Int
  helper = 40
  stage quoted :: Code Int
  stage quoted = [| {expression} |]
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(quoted))
  main :: Int
  main = answer
"#
    );
    let code = run_source(&source)
        .unwrap_or_else(|e| panic!("quotation {expression:?}: {e}"));
    assert_eq!(code, expected, "quotation {expression:?}");
}

/// A quotation bound to a stage declaration and spliced by name evaluates
/// end to end: `$(quoted)` resolves the declaration anywhere in the module,
/// fills the template, and the filled code is checked where it lands. It is
/// the only form that proves the lookup sees the module rather than the
/// walk's remaining suffix.
#[test]
fn named_quotation_declaration_splices_into_generated_declarations() {
    let source = r#"module Main where
  helper :: Int
  helper = 40
  stage quoted :: Code Int
  stage quoted = [| helper + 2 |]
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(quoted))
  main :: Int
  main = answer
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn child_terms_do_not_export_bindings_to_siblings() {
    for child in [
        "let helper = 2 in helper",
        "(\\helper -> helper) 2",
        "let f helper = helper in f 2",
    ] {
        run_quote(
            &format!("case ({child}, helper) of {{ (a, b) -> a + b }}"),
            42,
        );
    }
}

#[test]
fn branches_and_case_alternatives_have_separate_scopes() {
    run_quote("if False then let helper = 2 in helper else helper", 40);
    run_quote(
        "case True of { True -> let helper = 2 in helper; False -> helper }",
        2,
    );
}

#[test]
fn local_signatures_and_definitions_share_one_binding() {
    run_quote("let value :: Int; value = 2 in value + helper", 42);
    run_quote("let helper :: Int; helper = helper + 2 in helper", 42);
    run_quote("let f :: Int -> Int; f helper = helper in f 2 + helper", 42);
}

#[test]
fn recursive_groups_prebind_all_names_and_pair_their_annotations() {
    run_quote(
        r#"letrec even :: Nat -> Bool; odd :: Nat -> Bool; even = \n -> if n == 0 then True else odd (n - 1); odd = \n -> if n == 0 then False else even (n - 1) in if even 4 then helper + 2 else 1"#,
        42,
    );
}

#[test]
fn local_function_parameters_do_not_capture_later_local_bindings() {
    run_quote("let f helper = helper; result = helper + 2 in result", 42);
}

#[test]
fn captured_call_site_parameters_survive_generated_binders() {
    let source = r#"module Main where
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| let helper = 2 in $(arg) + helper |]
  call :: Int -> Int
  call helper = wrap!(helper)
  main :: Int
  main = call 40
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn declaration_parameters_do_not_leak_between_generated_equations() {
    let source = r#"module Main where
  helper :: Int
  helper = 40
  generate do
    emit (quoteDecls where
      first :: Int -> Int
      first helper = helper
      second :: Int
      second = first 2 + helper)
  main :: Int
  main = second
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn spliced_syntax_keeps_native_linear_usage_checks() {
    let source = r#"module Main where
  data Token where
    Token :: Int -> Token
  macro once :: Syntax -> Code Int
  macro once arg = [| case ($(arg) :: Token) of { Token value -> value } |]
  use :: Token %1 -> Int
  use token = once!(token)
  main :: Int
  main = use (Token 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let duplicated = source.replace("case ($(arg) :: Token) of { Token value -> value }",
        "let first = ($(arg) :: Token) in let second = ($(arg) :: Token) in case first of { Token a -> case second of { Token b -> a + b } }");
    let error = run_source(&duplicated)
        .unwrap_err()
        .to_string();
    assert!(error.contains("multiplicity %1"), "{error}");
}

#[test]
fn quoted_handler_return_patterns_bind_their_own_bodies() {
    for (body, expected) in [("2", 2), ("ask ()", 40)] {
        let source = format!(
            r#"module Main where
  effect Ask where
    ask :: Unit ->{{Ask}} Int
  helper :: Int
  helper = 40
  stage quoted :: Code Int
  stage quoted = [| handle {body} with (handler Ask where
    return helper -> helper
    ask unit resume continuation -> helper) |]
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(quoted))
  main :: Int
  main = answer
"#
        );
        assert_eq!(
            run_source(&source).unwrap(),
            expected
        );
    }
}

#[test]
fn qualified_effect_helpers_ignore_unrelated_caller_bindings() {
    let source = r#"module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int
  main :: Int
  main = let ask :: Unit -> Int; ask = \unit -> 99 in
    handle Main.ask () with (handler Ask where
      return value -> value
      ask unit resume continuation -> continuation 42)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let unhandled = r#"module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int
  main :: Int
  main = let ask :: Unit -> Int; ask = \unit -> 99 in Main.ask ()
"#;
    let error = run_source(unhandled)
        .unwrap_err()
        .to_string();
    assert!(error.contains("must be pure"), "{error}");
}

#[test]
fn qualified_comparison_facts_follow_the_resolved_method_after_renaming() {
    let source = r#"module Flipped where
  import Relation.Ord ((<) as (>))
  export bounded
  macro bounded :: Syntax -> Code Nat
  macro bounded arg = [| let n = ($(arg) :: Nat) in if n > 5 then 5 - n else 0 |]
module Main where
  import stage Flipped
  result :: Nat
  result = bounded!(3)
  main :: Int
  main = if result == 2 then 0 else 1
"#;
    assert_eq!(
        run_source(source).unwrap(),
        0
    );
    let forged = source.replace("then 5 - n", "then n - 5");
    let error = check_sources(&forged, "renamed-comparison.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("obligation"), "{error}");
}

#[test]
fn same_spelled_qualified_user_operators_do_not_grant_arithmetic_facts() {
    let source = r#"module Pretend where
  export (<), guarded
  infix 4 <
  (<) :: Nat -> Nat -> Bool
  (<) i n = True
  macro guarded :: Syntax -> Syntax -> Code Nat
  macro guarded test result = [| if $(test) < 5 then $(result) else 0 |]
module Main where
  import stage Pretend
  unsafe :: Nat -> Nat -> Nat
  unsafe i n = guarded!(i, n - i)
  main :: Int
  main = 0
"#;
    check_sources(
        &source.replace("n - i", "(0 :: Nat)"),
        "user-comparison.cinch",
    )
    .unwrap();
    let error = check_sources(source, "user-comparison.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("obligation"), "{error}");
}

#[test]
fn definition_site_private_helpers_survive_without_becoming_caller_names() {
    let source = r#"module Library where
  export wrap
  secret :: Int
  secret = 2
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) + secret |]
module Main where
  import stage Library
  secret :: Int
  secret = 40
  main :: Int
  main = wrap!(secret)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let direct = source.replace("main = wrap!(secret)", "main = Library.secret");
    let error = check_sources(&direct, "private-helper.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("unknown name Library.secret"), "{error}");
}

#[test]
fn helper_member_and_module_aliases_resolve_where_the_template_is_defined() {
    let source = r#"module Helpers where
  helper :: Int
  helper = 2
module Library where
  import Helpers (helper as renamed)
  export wrap
  macro wrap :: Syntax -> Code Int
  macro wrap arg = [| $(arg) + renamed |]
module Main where
  import stage Library
  renamed :: Int
  renamed = 40
  main :: Int
  main = wrap!(renamed)
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let qualified = source
        .replace("import Helpers (helper as renamed)", "import Helpers as H")
        .replace("$(arg) + renamed", "$(arg) + H.helper");
    assert_eq!(
        run_source(&qualified).unwrap(),
        42
    );
}

#[test]
fn captured_source_does_not_acquire_a_templates_private_binding_authority() {
    let source = r#"module Library where
  export identity
  secret :: Int
  secret = 2
  macro identity :: Syntax -> Code Int
  macro identity arg = [| $(arg) |]
module Main where
  import stage Library
  main :: Int
  main = identity!(Library.secret)
"#;
    let error = check_sources(source, "captured-private-helper.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("unknown name Library.secret"), "{error}");
}

#[test]
fn binding_identity_scheduling_keeps_ordinary_recursive_components() {
    let source = r#"module Main where
  export main
  even :: Nat -> Int
  even n = if n == 0 then 42 else odd (n - 1)
  odd :: Nat -> Int
  odd n = if n == 0 then 42 else even (n - 1)
  main :: Int
  main = even 4
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}

#[test]
fn ordinary_recursive_components_cannot_hide_a_staging_cycle() {
    let source = r#"module Main where
  stage staged :: Nat
  stage staged = first 0
  first :: Nat -> Nat
  first n = if n == 0 then second n else produced
  second :: Nat -> Nat
  second n = first n
  generate do
    let value = staged
    emit (quoteDecls where
      produced :: Nat
      produced = 1)
  main :: Int
  main = 0
"#;
    let error = check_sources(source, "mixed-staging-cycle.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("stage dependency cycle"), "{error}");
    assert!(
        error.contains("generate block") && error.contains("staged"),
        "{error}"
    );
}

#[test]
fn checked_quotations_retain_type_directed_constructor_identity() {
    let source = r#"module Main where
  stage empty :: Code (Vec 0 Int)
  stage empty = [| Nil |]
  stage inspect :: Code (Vec 0 Int -> Int)
  stage inspect = [| \v -> case v of { Nil -> 42 } |]
  generate do
    emit (quoteDecls where
      value :: Vec 0 Int
      value = $(empty)
      read :: Vec 0 Int -> Int
      read = $(inspect))
  main :: Int
  main = read value
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    // Checking this quote as a List must not let later emission choose the
    // same-spelled Vec constructor instead.
    let changed = source.replace("empty :: Code (Vec 0 Int)", "empty :: Code (List Int)");
    let error = check_sources(&changed, "constructor-identity.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("List") && error.contains("Vec"), "{error}");
}

#[test]
fn quoted_private_constructors_work_in_terms_and_patterns() {
    let source = r#"module Library where
  export Box, make, inspect
  data Box where
    Hide :: Int -> Box
  macro make :: Syntax -> Code Box
  macro make ignored = [| Hide 42 |]
  macro inspect :: Syntax -> Code Int
  macro inspect value = [| case ($(value) :: Box) of { Hide x -> x } |]
module Main where
  import stage Library
  main :: Int
  main = inspect!(make!(()))
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let forged = source.replace("inspect!(make!(()))", "inspect!(Library.Hide 42)");
    let error = check_sources(&forged, "captured-private-constructor.cinch")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Hide") && (error.contains("unknown") || error.contains("not exported")),
        "{error}"
    );
}

#[test]
fn private_type_annotations_stay_with_their_defining_module() {
    let source = r#"module Library where
  export value
  data Hidden where
    Hide :: Int -> Hidden
  macro value :: Syntax -> Code Int
  macro value ignored = [| let x :: Hidden; x = Hide 42 in case x of { Hide n -> n } |]
module Main where
  import stage Library
  main :: Int
  main = value!(())
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
    let forged = source.replace(
        "main = value!(())",
        "main = let x :: Library.Hidden; x = Library.Hide 42 in 0",
    );
    let error = check_sources(&forged, "private-type.cinch")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("Hidden") || error.contains("Hide"),
        "{error}"
    );
}

#[test]
fn sharing_a_source_file_does_not_grant_macro_private_access() {
    let source = r#"module Library where
  export public
  secret :: Int
  secret = 42
  public :: Int
  public = 0
module Other where
  import Library
  macro steal :: Syntax -> Code Int
  macro steal ignored = [| Library.secret |]
module Main where
  import stage Other
  main :: Int
  main = steal!(())
"#;
    let error = check_sources(source, "same-file-private.cinch")
        .unwrap_err()
        .to_string();
    assert!(error.contains("unknown name Library.secret"), "{error}");
}

#[test]
fn checked_helper_identity_cannot_be_captured_by_a_caller_parameter() {
    let source = r#"module Library where
  export wrap
  helper :: Int
  helper = 2
  macro wrap :: Syntax -> Code Int
  macro wrap value = [| helper + $(value) |]
module Main where
  import stage Library
  call :: Int -> Int
  call helper = wrap!(helper)
  main :: Int
  main = call 40
"#;
    assert_eq!(
        run_source(source).unwrap(),
        42
    );
}
