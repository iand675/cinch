//! Interned symbols (P24): the `Sym` core type and the tick lexeme
//! (specifications/primitive-types-specification.md section 9,
//! specifications/surface-grammar-local-amendments.md section 12;
//! D122-D124).
//!
//! The acceptance matrix: `'foo` literals are constants with identity
//! `Eq`, name `Ord` and `'foo` `Show`; `intern`/`sym-name` round trip
//! with identity holding across the boundary; literal patterns
//! dispatch by identity; the compile-time interned literal is emitted
//! as a constant (the P14 constant-emission path); char precedence is
//! lexical and deterministic; and the tick name quotes feed macro
//! positions as names (whatever the D46 member currently supports:
//! `Syntax` token trees, whose name lexemes reify to the interned
//! constant).

use cinch::driver::{check_sources, run_source};
use cinch::lexer::{lex, Tok};

/// Compile, run and assert the exit code.
fn run_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Assert that checking the program fails with a message containing
/// `needle`.
fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected a compilation error containing {needle:?}"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// The emitted LLVM IR of one checked source (every IR test carries a
/// `main`, which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The token kinds of one source (the lexical rules are observable
/// without a program around them).
fn kinds(src: &str) -> Vec<Tok> {
    lex(src)
        .unwrap_or_else(|e| panic!("lex {src:?} failed: {e}"))
        .into_iter()
        .map(|t| t.tok)
        .collect()
}

// ---------------------------------------------------------------- D122: Sym

#[test]
fn sym_literals_are_constants_with_identity_eq() {
    // Equal spellings are the same interned cell (`==` is identity),
    // different spellings are different cells, and `Show` prints the
    // spelling with its tick (primitive types specification section 9).
    run_ok(
        "sym-literals",
        r#"
module Main where
  import Text.Show

  -- The expected `'foo` text, built from its bytes (never through
  -- `show` itself): tick 39, then `foo`.
  tick-foo :: String
  tick-foo = case string-from-bytes (bytes-from-list (Cons 39 (Cons 102 (Cons 111 (Cons 111 Nil))))) of
    Some s -> s
    None -> empty-string

  main :: Int
  main =
    let a = 'foo in
    let b = 'foo in
    let c = 'bar in
    if a == b then
      (if not (a == c) then
        (if show 'foo == tick-foo then 0 else 3)
      else 2)
    else 1
"#,
        0,
    );
}

#[test]
fn intern_and_sym_name_round_trip_with_identity_across_the_boundary() {
    // `sym-name` then `intern` is the same cell (the runtime table is
    // keyed by name, so `intern` of an equal name resolves to the
    // literal's cell). The name is built at runtime from bytes, so the
    // `intern` call is a runtime call against the process table.
    run_ok(
        "sym-round-trip",
        r#"
module Main where
  name-of :: List Nat -> String
  name-of xs = case string-from-bytes (bytes-from-list xs) of
    Some s -> s
    None -> empty-string

  foo-name :: String
  foo-name = name-of (Cons 102 (Cons 111 (Cons 111 Nil)))

  main :: Int
  main =
    let s = intern foo-name in
    if s == 'foo then
      (if intern (sym-name s) == 'foo then
        (if sym-name s == foo-name then 0 else 3)
      else 2)
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------- D122: Ord by name

#[test]
fn ord_is_lexicographic_by_name_and_deterministic_across_runs() {
    // The sorted order is the names' order (`bar` < `baz` < `foo` <
    // `int-digits-into`), never insertion order or cell addresses; two
    // separate runs of the same program observe the same order.
    let src = r#"
module Main where
  insert-s :: Sym -> List Sym -> List Sym
  insert-s x xs = case xs of
    Nil -> Cons x Nil
    Cons y ys -> if x < y then Cons x (Cons y ys) else Cons y (insert-s x ys)

  sort-s :: List Sym -> List Sym
  sort-s xs = case xs of
    Nil -> Nil
    Cons y ys -> insert-s y (sort-s ys)

  order-ok :: List Sym -> Bool
  order-ok xs = case xs of
    Cons a (Cons b (Cons c (Cons d Nil))) ->
      if a == 'bar then
        (if b == 'baz then (if c == 'foo then d == 'int-digits-into else False) else False)
      else False
    _ -> False

  main :: Int
  main = if order-ok (sort-s (Cons 'foo (Cons 'bar (Cons 'int-digits-into (Cons 'baz Nil))))) then 0 else 1
"#;
    let first = run_source(src).expect("the ordering program compiles and runs");
    let second = run_source(src).expect("the ordering program compiles and runs again");
    assert_eq!(first, 0, "the sorted order is the names' order");
    assert_eq!(second, first, "the sorted order is stable across runs");
}

// -------------------------------------------------- D123: literal patterns

#[test]
fn literal_patterns_dispatch_by_identity() {
    // `case s of 'foo -> ...` dispatches on the interned cell: literal
    // patterns test identity, a catch-all catches what no literal
    // claims, dash names work, and a runtime `intern` of the same name
    // lands on the literal's arm.
    run_ok(
        "sym-patterns",
        r#"
module Main where
  name-of :: List Nat -> String
  name-of xs = case string-from-bytes (bytes-from-list xs) of
    Some s -> s
    None -> empty-string

  dispatch :: Sym -> Int
  dispatch s = case s of
    'foo -> 1
    'bar -> 2
    'int-digits-into -> 3
    _ -> 4

  main :: Int
  main =
    if dispatch 'foo == 1 then
      (if dispatch 'bar == 2 then
        (if dispatch 'int-digits-into == 3 then
          (if dispatch 'other == 4 then
            (if dispatch (intern (name-of (Cons 98 (Cons 97 (Cons 114 Nil))))) == 2 then 0 else 5)
          else 4)
        else 3)
      else 2)
    else 1
"#,
        0,
    );
}

// ------------------------------------------- D123: compile-time interning

#[test]
fn a_symbol_literal_is_emitted_as_a_constant() {
    // The literal interns at compile time and is emitted as a constant
    // (the P14 constant-emission path): the name's bytes are a private
    // constant and the cell is materialised through the runtime intern
    // table, which `intern` of an equal name shares.
    let ir = ir_of(
        "sym-constant",
        r#"
module Main where
  keep :: Sym -> Sym
  keep s = s

  main :: Int
  main = case keep 'foo of
    'foo -> 0
    _ -> 1
"#,
    );
    assert!(
        ir.contains("c\"foo\""),
        "the name `foo` should be emitted as a constant (the P14 constant-emission path):\n{ir}"
    );
    assert!(
        ir.contains("call i64 @cinch_sym_intern("),
        "the cell should be materialised through the runtime intern table:\n{ir}"
    );
    assert!(
        ir.contains("@\"cinch.runtime.symEq\"("),
        "the identity pattern test should be the `symEq` unit:\n{ir}"
    );
}

// ---------------------------------------------- D123: char precedence

#[test]
fn char_precedence_is_lexical_and_deterministic() {
    // Tick, one character, tick means `Char` and wins; `'a` followed by
    // anything else is a symbol; `'foo` is always a symbol; `''T` is
    // the type-name form.
    assert_eq!(
        kinds("'a' 'a 'ab 'foo 'int-digits-into ''T"),
        vec![
            Tok::CharLit("a".into()),
            Tok::SymLit("a".into()),
            Tok::SymLit("ab".into()),
            Tok::SymLit("foo".into()),
            Tok::SymLit("int-digits-into".into()),
            Tok::TypeNameLit("T".into()),
            Tok::Eof,
        ]
    );
    // One character is one character: an escape is one character, and
    // the tick character itself is one character between ticks.
    assert_eq!(kinds(r"'\n'"), vec![Tok::CharLit("\n".into()), Tok::Eof]);
    assert_eq!(kinds("'''"), vec![Tok::CharLit("'".into()), Tok::Eof]);
}

#[test]
fn char_and_symbol_spellings_type_apart() {
    // `'a'` is a `Char`, `'a` and `'ab` are `Sym`s: the same first two
    // characters diverge on the closing tick alone.
    run_ok(
        "char-vs-sym",
        r#"
module Main where
  name-of :: List Nat -> String
  name-of xs = case string-from-bytes (bytes-from-list xs) of
    Some s -> s
    None -> empty-string

  a-char :: Char
  a-char = 'a'

  a-sym :: Sym
  a-sym = 'a

  ab-sym :: Sym
  ab-sym = 'ab

  main :: Int
  main =
    if char-to-u32 a-char == 97 then
      (if a-char == 'a' then
        (if sym-name a-sym == name-of (Cons 97 Nil) then
          (if sym-name ab-sym == name-of (Cons 97 (Cons 98 Nil)) then 0 else 4)
        else 3)
      else 2)
    else 1
"#,
        0,
    );
    // The deterministic rule is lexical: the wrong spelling for the
    // position is a type error, not a different reading.
    expect_err(
        "sym-not-char",
        "module Main where\n  c :: Char\n  c = 'ab\n",
        "Sym",
    );
}

// ------------------------------------------ D123: names in staging positions

#[test]
fn tick_name_quotes_reify_in_macro_positions() {
    // `'foo` in a macro's name position (its `Syntax` token-tree
    // argument: what the D46 member supports today) is one name lexeme;
    // a macro that reifies the name into a symbol literal gets the
    // interned constant for the same spelling, identical to a direct
    // `'foo` literal.
    run_ok(
        "macro-name-quote",
        r#"
module Main where
  macro bindSym :: Syntax -> Declarations
  macro bindSym arg = quoteDecls where
    result :: Sym
    result = $(arg)

  generate do
    emit (bindSym!('foo))

  main :: Int
  main = if result == 'foo then 0 else 1
"#,
        0,
    );
}

#[test]
fn name_quotes_outside_their_positions_are_refused() {
    // `''T` is a type `Name` for staging and macro name positions only;
    // in expression or pattern position it names nothing. At the type
    // level the `'` spelling is reserved: promoted symbols are deferred
    // (D124), so `'foo` and `''T` are not types.
    expect_err(
        "type-name-quote-in-expression",
        "module Main where\n  main :: Int\n  main = case ''T of\n    _ -> 0\n",
        "type name quote",
    );
    expect_err(
        "type-name-quote-in-pattern",
        "module Main where\n  f :: Sym -> Int\n  f s = case s of\n    ''T -> 0\n    _ -> 1\n  main :: Int\n  main = f 'foo\n",
        "type name quote",
    );
    expect_err(
        "symbol-is-not-a-type",
        "module Main where\n  f :: 'foo -> Int\n  f x = 0\n  main :: Int\n  main = 0\n",
        "deferred",
    );
    expect_err(
        "type-name-quote-is-not-a-type",
        "module Main where\n  f :: ''T -> Int\n  f x = 0\n  main :: Int\n  main = 0\n",
        "deferred",
    );
}

// ------------------------------------------------- D122: the FFI boundary

#[test]
fn sym_has_no_c_counterpart() {
    // Foreign code takes `sym-name` (or `Bytes`) at the boundary: `Sym`
    // itself has no C representation (primitive types specification
    // section 9).
    expect_err(
        "sym-foreign",
        "module Main where\n  foreign import ccall \"sym\" useSym :: Sym ->{Fail} Int\n  main :: Int\n  main = 0\n",
        "has no C representation",
    );
}
