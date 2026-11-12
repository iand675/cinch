//! The narrow value representation at the function boundary
//! (design/narrow-value-representation.md section 4): a parameter or
//! result crosses at its own width when its type has a fixed narrow
//! layout and the signature is monomorphic at that position, and stays
//! one word everywhere else.
//!
//! The rule is *proved*, never guessed: a position whose width the
//! compiler cannot state keeps the word form, so a type variable's
//! instantiation, a boxed `Nat`/`Int`, a dictionary and a closure all
//! stay `i64`. These tests assert both halves, because a narrowing that
//! fired everywhere would be as wrong as one that never fires.

mod common;

use cinch::core::TargetProfile;

/// Emit one source's IR under the native profile.
fn emit(name: &str, src: &str) -> String {
    let target = TargetProfile::native();
    let program = cinch::driver::check_sources_profile(src, name, &target)
        .unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit_for(&target, &program)
        .unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The `define` line of `unit`, so a test can read its exact prototype.
fn define_of<'a>(ir: &'a str, unit: &str) -> &'a str {
    let needle = format!("@\"{unit}\"(");
    ir.lines()
        .find(|l| l.starts_with("define ") && l.contains(&needle))
        .unwrap_or_else(|| panic!("no define for {unit} in:\n{ir}"))
}

/// A machine scalar narrower than the word crosses at its own width: the
/// microcontroller case, where a `U8` parameter stops costing eight
/// bytes. The body keeps the word it always had, and the entry widens the
/// narrowed parameter once, which is the design's "box at the expanding
/// edge".
#[test]
fn a_narrow_scalar_crosses_at_its_own_width() {
    // Recursive, so the unit survives inlining and the boundary is
    // observable: a `U8` parameter beside a `Nat` one shows both halves
    // of the rule in a single signature.
    let ir = emit(
        "<test>",
        r#"
module Main where
  down :: U8 -> Nat -> Nat
  down limit n = if n == 0 then 0 else 1 + down limit (n - 1)
  main :: Int
  main = if down 9 3 == 3 then 0 else 1
"#,
    );
    let line = define_of(&ir, "Main.down");
    assert!(
        line.contains("i8 noundef %p0"),
        "a U8 parameter crosses as a byte:\n{line}"
    );
    assert!(
        line.contains("i64 %p1"),
        "a Nat parameter keeps the word:\n{line}"
    );
    // The body reads a word, so the entry widens once.
    assert!(
        ir.contains("zext i8 %p0 to i64"),
        "the narrowed parameter is widened at entry:\n{ir}"
    );
}

/// Every width the rule names is narrowed, and every width it does not
/// stays a word.
#[test]
fn the_widths_that_narrow_are_exactly_the_sub_word_scalars() {
    // Each of these is a sub-word machine scalar, so each crosses narrow.
    for (ty, want) in [
        ("U8", "i8"),
        ("I8", "i8"),
        ("U16", "i16"),
        ("I16", "i16"),
        ("U32", "i32"),
        ("I32", "i32"),
        ("F32", "i32"),
    ] {
        // Recursive, so the unit survives inlining and its prototype is
        // observable: a plain identity inlines away and leaves nothing to
        // inspect.
        let ir = emit(
            "<test>",
            &format!(
                r#"
module Main where
  f :: {ty} -> Nat -> Nat
  f x n = if n == 0 then 0 else 1 + f x (n - 1)
  main :: Int
  main = if f 1 3 == 3 then 0 else 1
"#
            ),
        );
        let line = define_of(&ir, "Main.f");
        assert!(
            line.contains(&format!("{want} noundef %p0")),
            "{ty} should cross as {want}:\n{line}"
        );
    }
}

/// The word-width scalars and everything whose width is not a property of
/// the declared type keep one word: `I64`/`U64`/`F64` because they are
/// the word, `Char`/`Bool` because their representation is specified over
/// it, and a type variable, `Nat`/`Int`, a pointer and an ADT because the
/// compiler cannot state a width for them.
#[test]
fn everything_unproved_keeps_the_word() {
    // A type variable's instantiation: the same source at two types must
    // not narrow, because the parameter is one word for every instance.
    let poly = emit(
        "<test>",
        r#"
module Main where
  use :: forall a. (a -> a) -> a -> Nat -> a
  use f x n = if n == 0 then f x else use f x (n - 1)
  main :: Int
  main = if use (\x -> x) 5 0 == 5 then 0 else 1
"#,
    );
    let line = define_of(&poly, "Main.use");
    assert!(
        !line.contains("i8 ") && !line.contains("i16 ") && !line.contains("i32 "),
        "a type-variable position stays one word:\n{line}"
    );

    // The word-width scalars. Each needs a literal of its own type: a
    // `Bool` cannot take `1` and a `Char` needs a character.
    for (ty, lit) in [
        ("I64", "1"),
        ("U64", "1"),
        // `F64` is absent only because a float literal has no AST node in
        // the bootstrap slice; the width rule treats it exactly as `I64`
        // (`narrow_param_bits` returns `None` for every word-width
        // scalar), which the `I64` case pins.
        ("Usize", "1"),
        ("Isize", "1"),
        ("Char", "'a'"),
        ("Bool", "True"),
    ] {
        let ir = emit(
            "<test>",
            &format!(
                r#"
module Main where
  g :: {ty} -> Nat -> Nat
  g x n = if n == 0 then 0 else 1 + g x (n - 1)
  main :: Int
  main = if g {lit} 3 == 3 then 0 else 1
"#
            ),
        );
        let line = define_of(&ir, "Main.g");
        // `noundef` may sit between the type and the name, so match the
        // type and the register separately.
        assert!(
            line.contains("i64") && line.contains("%p0") && !line.contains("i8 ") && !line.contains("i16 ") && !line.contains("i32 "),
            "{ty} is one word, so it must not narrow:\n{line}"
        );
    }
}

/// A narrowed unit takes an ordinary call instead of a `musttail` one.
///
/// `musttail` requires the callee's prototype to be the caller's exactly,
/// parameters included, and a narrowed signature is not the word
/// prototype. Emitting `musttail` anyway is not a subtle miscompile: the
/// LLVM verifier refuses it outright with "cannot guarantee tail call due
/// to mismatched parameter types".
#[test]
fn a_narrowed_unit_does_not_emit_musttail() {
    let ir = emit(
        "<test>",
        r#"
module Main where
  down :: U8 -> Nat -> Nat
  down limit n = if n == 0 then 0 else 1 + down limit (n - 1)
  main :: Int
  main = if down 9 3 == 3 then 0 else 1
"#,
    );
    let body: &str = ir
        .split("@\"Main.down\"(")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("Main.down's body");
    assert!(
        !body.contains("musttail"),
        "a narrowed unit must not emit musttail:\n{body}"
    );
}
