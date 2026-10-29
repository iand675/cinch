//! The bootstrap contract: the shared AST and report modules compile, run
//! and answer questions about themselves, so every later rung can be built
//! against a verified interface. A failure of this test means the contract
//! changed shape, not that a rung is wrong.

mod common;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");

const MAIN: &str = r#"module Main where
  import Bootstrap.Ast
  import Bootstrap.Report
  import Text.Lex (Span, span-start, span-end, span-line, span-col)

  sp :: Span
  sp = Span 0 4 1 1

  wide :: Span
  wide = span-join sp (Span 9 10 3 4)

  report-text :: String
  report-text = render-diagnostic (MkDiagnostic "bootstrap.contract" (span-here sp) "example")

  -- if 1 == 2 then 0 else 1, as a clause whose body is an if.
  clause :: (List Pat, Expr)
  clause = (Nil, EIf sp (EBin sp "==" (ELitInt sp "1") (ELitInt sp "2")) (ELitInt sp "0") (ELitInt sp "1"))

  -- one module, one function declaration.
  decls :: Decl
  decls = decl-spine sp (Cons (DFun sp "main" (Cons clause Nil)) Nil)

  ast :: Program
  ast = MkProgram sp (Cons (MkModule sp "Main" decls) Nil)

  -- a constructor application whose argument spine holds two literals.
  ctor-args :: Expr
  ctor-args = ECon sp "Wrap" (expr-spine sp (Cons (ELitInt sp "7") (Cons (ELitInt sp "8") Nil)))

  -- How a consumer reaches a spine: match the constructor, then walk its
  -- spine field (an `exprs-of` on the constructor itself is not a spine and
  -- yields the empty list, which is the documented malformed-spine answer).
  ctor-arg-exprs :: Expr -> List Expr
  ctor-arg-exprs e = case e of
    ECon s n args -> exprs-of args
    other -> Nil

  program-name :: Program -> String
  program-name p = case p of
    MkProgram s ms -> modules-name ms

  modules-name :: List Module -> String
  modules-name ms = case ms of
    Nil -> "none"
    Cons m rest -> module-name m

  main :: Int
  main = if program-unmodeled-count ast == 0 then step1 else 1

  step1 :: Int
  step1 = if string-compare (program-name ast) "Main" == 0 then step2 else 2

  step2 :: Int
  step2 = if list-length (module-decls (case ast of MkProgram s ms -> case ms of Cons m rest -> m; Nil -> MkModule sp "x" (DDeclsEnd sp))) == 1 then step3 else 3

  step3 :: Int
  step3 = if list-length (ctor-arg-exprs ctor-args) == 2 then step4 else 4

  step4 :: Int
  step4 = if span-start wide == 0 then step5 else 5

  step5 :: Int
  step5 = if span-end wide == 10 then step6 else 6

  step6 :: Int
  step6 = if span-line wide == 1 then step7 else 7

  step7 :: Int
  step7 = if string-compare report-text "bootstrap.contract:1:1: example" == 0 then 0 else 8
"#;

#[test]
fn the_shared_contract_compiles_and_answers() {
    let dir = common::project(
        "contract",
        &[
            ("Bootstrap.Ast.cinch", AST_SRC),
            ("Bootstrap.Report.cinch", REPORT_SRC),
            ("main.cinch", MAIN),
        ],
    );
    let code = common::run(&dir, "main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0, "contract program: the failing assertion's bit (1 unmodeled, 2 name, 3 span start, 4 span end, 5 line, 6 render)");
}
