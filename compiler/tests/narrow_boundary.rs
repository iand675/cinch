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

/// The direction a narrowed position extends back into the slot is the
/// one the word convention already uses for its type, so a value that
/// crossed a boundary is the value the body would have seen had the
/// position never narrowed.
///
/// This is the half of the rule the width alone cannot state, and getting
/// it wrong is silent: `I8`'s `-2` zero-extended is `254`, which every
/// signed comparison then reads as a large positive number.
#[test]
fn a_narrowed_position_extends_the_way_its_type_stores_its_word() {
    // A signed scalar's canonical word is sign-extended, so its narrowed
    // parameter sign-extends; an unsigned scalar's is zero-extended.
    for (ty, ext) in [("I8", "sext"), ("I16", "sext"), ("I32", "sext")] {
        let ir = emit(
            "<test>",
            &format!(
                r#"
module Main where
  f :: {ty} -> Nat -> Nat
  f x n = if n == 0 then 0 else 1 + f x (n - 1)
  main :: Int
  main = if f (-1) 3 == 3 then 0 else 1
"#
            ),
        );
        let body = ir
            .split(&format!("@\"Main.f\"("))
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .unwrap_or_else(|| panic!("Main.f's body for {ty}"));
        assert!(
            body.contains(&format!("{ext} ")) && body.contains("%p0 to i64"),
            "{ty} extends its parameter with {ext}:\n{body}"
        );
        assert!(
            !body.contains(&format!("zext i8 %p0")) && !body.contains("zext i16 %p0")
                && !body.contains("zext i32 %p0"),
            "{ty} must not zero-extend its parameter:\n{body}"
        );
    }
    for (ty, ext) in [("U8", "zext"), ("U16", "zext"), ("U32", "zext")] {
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
        let body = ir
            .split(&format!("@\"Main.f\"("))
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .unwrap_or_else(|| panic!("Main.f's body for {ty}"));
        assert!(
            body.contains(&format!("{ext} ")) && body.contains("%p0 to i64"),
            "{ty} extends its parameter with {ext}:\n{body}"
        );
    }
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

/// The slot-width scalars and everything whose width is not a property of
/// the declared type keep one word: `I64`/`U64`/`F64` because they fill
/// the slot, `Char`/`Bool` because their representation is specified over
/// it, and a type variable because its instantiation may be a 64-bit
/// scalar. On this 64-bit host a pointer, an ADT and a `Nat`/`Int` keep it
/// too, because their `csize` is the slot; the 32-bit case is
/// [`a_csize_position_narrows_only_where_the_pointer_is_smaller`].
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

/// A narrowed unit reached through its apply stub converts at the stub.
///
/// The stub's frame is a word array, so it cannot pass words straight to a
/// narrowed signature: it truncates each argument to the callee's proved
/// width and widens the result back. This is the marshal the design calls
/// for at the indirect boundary, and it is what makes a narrowed unit safe
/// to take as a value at all.
///
/// The closure is held in a list so the stub is genuinely emitted and
/// genuinely called; a direct call would inline and prove nothing.
#[test]
fn a_narrowed_unit_reached_through_its_stub_converts() {
    let ir = emit(
        "<test>",
        r#"
module Main where
  import Data.List
  tag :: U8 -> U8
  tag x = if x == 1 then 2 else if x == 2 then 3 else if x == 3 then 4 else if x == 4 then 5 else if x == 5 then 6 else if x == 6 then 7 else if x == 7 then 8 else x
  fs :: List (U8 -> U8)
  fs = Cons tag Nil
  main :: Int
  main = case fs of
    Nil -> 1
    Cons f _ -> if f 7 == 8 then 0 else 1
"#,
    );
    let unit = define_of(&ir, "Main.tag");
    assert!(
        unit.contains("i8 noundef %p0"),
        "the unit crosses narrow:\n{unit}"
    );
    let stub = ir
        .split("@\"Main.tag.app\"(ptr %self")
        .nth(1)
        .and_then(|rest| rest.split("\n}\n").next())
        .expect("Main.tag's apply stub");
    assert!(
        stub.contains("trunc i64") && stub.contains("to i8"),
        "the stub narrows the argument to the callee's width:\n{stub}"
    );
    assert!(
        stub.contains("call i8 @\"Main.tag\"(i8 "),
        "the stub calls the narrowed signature:\n{stub}"
    );
    assert!(
        stub.contains("zext i8") && stub.contains("to i64"),
        "the stub widens the result back for the word frame:\n{stub}"
    );
}

/// Emit one source's IR for `wasm32-unknown-unknown` (the profile
/// `--target wasm32-unknown-unknown` builds).
fn emit_wasm32(name: &str, src: &str) -> String {
    let target = TargetProfile::wasm32(cinch::core::ArtifactProfile::CoreWasm);
    let program = cinch::driver::check_sources_profile(src, name, &target)
        .unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit_for(&target, &program)
        .unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// A position whose every value fits the pointer crosses at `csize`: a
/// pointer or ADT and a `Nat` zero-extend back into the slot, an `Int`
/// sign-extends, and a 64-bit scalar or a type variable keeps the slot.
/// `csize` is the slot on a 64-bit target, so the same source narrows on
/// wasm32 and not natively. The `Nat`/`Int` immediate payload follows
/// `csize` for exactly this reason, so a literal past it is a box on
/// wasm32 and still an immediate natively.
#[test]
fn a_csize_position_narrows_only_where_the_pointer_is_smaller() {
    let src = r#"
module Main where
  import Data.List
  import Data.Machine
  walk :: List Nat -> Nat -> Int -> I64 -> Nat
  walk xs k d w = case xs of
    Nil -> k
    Cons x rest -> walk rest (k + x) (d - 1) (wrapping-sub-i64 w 1)
  walkers :: List (List Nat -> Nat -> Int -> I64 -> Nat)
  walkers = Cons walk Nil
  use :: forall a. (a -> a) -> a -> Nat -> a
  use f x n = if n == 0 then f x else use f x (n - 1)
  main :: Int
  main = case walkers of
    Nil -> 3
    Cons g _ -> if g (Cons 2147483648 (Cons 5 Nil)) 0 0 7 == 2147483653 then (if use (\x -> x) (2147483647 :: Nat) 0 == 2147483647 then 0 else 2) else 1
"#;
    // `walk` is held in a list so it is a real unit rather than a body
    // the simplifier folded into `main`.
    let w32 = emit_wasm32("<test>", src);
    let walk = define_of(&w32, "Main.walk");
    assert!(
        walk.contains("(i32 %p0, i32 %p1, i32 %p2, i64 noundef %p3)"),
        "a list, a Nat and an Int cross at csize and an I64 keeps the slot:\n{walk}"
    );
    let start = w32.find(walk).expect("the define is in the module");
    let body = w32[start..].split("\n}\n").next().expect("Main.walk's body");
    assert!(body.contains("zext i32 %p0 to i64"), "a pointer zero-extends:\n{body}");
    assert!(body.contains("zext i32 %p1 to i64"), "a Nat zero-extends:\n{body}");
    assert!(body.contains("sext i32 %p2 to i64"), "an Int sign-extends:\n{body}");
    // The polymorphic unit is specialised by class (`Main.use@T`), and
    // its type-variable position stays the slot while its closure and
    // `Nat` positions narrow.
    let poly = w32
        .lines()
        .find(|l| l.starts_with("define ") && l.contains("@\"Main.use"))
        .unwrap_or_else(|| panic!("no define for Main.use in:\n{w32}"));
    assert!(
        poly.contains("(i32 %p0, i64 %p1, i32 %p2)"),
        "a type-variable position keeps the slot on every target:\n{poly}"
    );
    // `2^31 - 1` is the largest wasm32 `Nat` immediate (`v << 1 | 1`
    // fills the 32-bit word), so it folds to its word; `2^31` is past it
    // and goes through the runtime to a box, where a 64-bit target still
    // folds it.
    let boxed = "@cinch_nat_from_i64(i64 2147483648)";
    assert!(w32.contains("i64 4294967295"), "2^31 - 1 folds to its immediate:\n{w32}");
    assert!(w32.contains(boxed), "2^31 is not a wasm32 immediate:\n{w32}");

    let native = emit("<test>", src);
    let walk = define_of(&native, "Main.walk");
    assert!(
        walk.contains("(i64 %p0, i64 %p1, i64 %p2, i64 noundef %p3)"),
        "csize is the slot on a 64-bit target, so nothing narrows:\n{walk}"
    );
    assert!(!native.contains(boxed), "2^31 is a native immediate:\n{native}");
}
