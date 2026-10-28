//! Attribute annotations (P17): attachment points and argument grammar,
//! branch weights with the `cold`/`likely`/`unlikely` wrappers and the
//! all-or-nothing reports, inline hints, and representations with their
//! computed layout facts (attribute annotations specification sections
//! 1-5, 7).

use cinch::check::{check_program_multi_diag, Diagnostic, StagingCx};
use cinch::core::Program;
use cinch::driver::{check_sources, parse_prelude, parse_source, run_source};
use cinch::interface::Iface;
use cinch::Result;
use std::collections::HashMap;

/// Check `src` against the prelude family, returning the elaborated
/// program, the interfaces (the family first), and the non-fatal
/// diagnostics.
fn check_with_diags(src: &str) -> Result<(Program, Vec<Iface>, Vec<Diagnostic>)> {
    let prelude = parse_prelude(&[])?;
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (program, ifaces, _report, diags) =
        check_program_multi_diag(&prelude.modules, &user.modules, &hashes, &scx)?;
    Ok((program, ifaces, diags))
}

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// The emitted LLVM IR of a program (every IR test carries a `main`,
/// which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The checked interface of a single-module source (main-less is fine).
fn iface_of(name: &str, src: &str) -> Iface {
    let (_p, ifaces, diags) = check_with_diags(src)
        .unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    assert!(diags.is_empty(), "{name}: unexpected diagnostics: {diags:?}");
    ifaces.last().expect("one user interface").clone()
}

// ------------------------------------------- the worked example (section 7)

/// The chapter's section 7 worked example, verbatim.
const WORKED_EXAMPLE: &str = r#"
module Fast where

  -- Layout: this type is the C ABI of a foreign record.
  #[repr(C)]
  record Header where
    #[repr(align(8))] magic :: U64
    len :: U32

  -- Branch weights: hot parse loop, cold error arm.
  classify :: U8 -> Nat
  classify b =
    case b of
      #[weight(2000)] 0 -> 0
      #[weight(1)]    _ -> 1

  -- The wrappers, mixed with a manual weight (complete annotation).
  fastPath :: Bool -> Bool -> Nat
  fastPath a b =
    if #[likely] a
    then if #[weight(100)] b then 1 else 2
    else 0

  -- Inline hint with weights preserved through inlining.
  #[inline]
  saturating :: Nat -> Nat
  saturating n =
    case n of
      #[cold] 0 -> 0
      #[weight(2000)] _ -> n
"#;

#[test]
fn worked_example_checks_cleanly() {
    let (_program, ifaces, diags) =
        check_with_diags(WORKED_EXAMPLE).expect("the worked example must check");
    // The example's `if` annotations are complete: the condition
    // position carries the true branch's weight and completes the pair
    // with the reference base 1 (attribute annotations specification
    // section 2.1), so nothing is dropped and nothing is reported.
    assert!(
        diags.is_empty(),
        "the worked example's annotations are complete; got {diags:?}"
    );
    // The example is main-less: it checks cleanly.
    // The `repr(C)` record's layout facts are in the contract.
    let fast = ifaces.last().expect("Fast interface");
    let header = fast
        .layouts
        .iter()
        .find(|l| l.name == "Header")
        .expect("Header layout");
    assert_eq!(header.repr, "C");
    assert_eq!(header.size, 16);
    assert_eq!(header.align, 8);
    assert_eq!(header.fields.len(), 2);
    assert_eq!((header.fields[0].offset, header.fields[0].size), (0, 8));
    assert_eq!((header.fields[1].offset, header.fields[1].size), (8, 4));
}

#[test]
fn worked_example_compiles_and_runs() {
    // The same program with an entry point exercising every definition.
    let src = WORKED_EXAMPLE.replace(
        "module Fast where",
        "module Main where\n\n  main :: Int\n  main =\n    if classify 0 == 0 then\n      if fastPath True True == 1 then\n        if fastPath True False == 2 then\n          if fastPath False True == 0 then\n            if saturating 0 == 0 then\n              if saturating 5 == 5 then 0 else 6\n            else 5\n          else 4\n        else 3\n      else 2\n    else 1",
    );
    expect_ok("worked-example", &src, 0);
    // Every hint and weight of the example reaches the emitted LLVM.
    let ir = ir_of("worked-example-ir", &src);
    assert!(ir.contains("inlinehint"), "the `#[inline]` hint must reach LLVM");
    assert!(ir.contains("!{!\"branch_weights\", i32 2000, i32 1}"), "{ir}");
    assert!(ir.contains("!{!\"branch_weights\", i32 100, i32 1}"), "{ir}");
    assert!(ir.contains("!{!\"branch_weights\", i32 0, i32 2000}"), "{ir}");
}

// ------------------------------------------------------- weights (sections 2-3)

#[test]
fn weights_on_case_alternatives() {
    let ir = ir_of(
        "case-weights",
        r#"
module Main where
  data T where
    A :: T
    B :: T
    C :: T

  pick :: T -> Nat
  pick t = case t of
    #[weight(2000)] A -> 0
    #[weight(100)] B -> 1
    #[weight(1)] C -> 2

  main :: Int
  main = if pick A == 0 then 0 else 1
"#,
    );
    // One weight per successor; the LLVM switch default (case #0) is the
    // covered match's run-out path at weight 0.
    assert!(
        ir.contains("!{!\"branch_weights\", i32 0, i32 2000, i32 100, i32 1}"),
        "expected the switch weights in:\n{ir}"
    );
}

#[test]
fn weights_on_function_clauses() {
    let ir = ir_of(
        "clause-weights",
        r#"
module Main where
  data T where
    A :: T
    B :: T
    C :: T

  pick :: T -> Nat
  pick #[weight(2000)] A = 0
  pick #[weight(100)] B = 1
  pick #[weight(1)] C = 2

  main :: Int
  main = if pick A == 0 then 0 else 1
"#,
    );
    assert!(
        ir.contains("!{!\"branch_weights\", i32 0, i32 2000, i32 100, i32 1}"),
        "expected the clause weights in:\n{ir}"
    );
}

#[test]
fn weights_on_guards() {
    let ir = ir_of(
        "guard-weights",
        r#"
module Main where
  guardW :: Bool -> Nat
  guardW b = case b of
    #[weight(2000)] True | #[weight(2000)] True -> 0
    #[weight(1)] False -> 1

  main :: Int
  main = if guardW True == 0 then 0 else 1
"#,
    );
    // The guard's own decision point: its pass outcome at 2000, the
    // fallthrough completed with the reference base 1 (section 2.1).
    assert!(
        ir.contains("!{!\"branch_weights\", i32 2000, i32 1}"),
        "expected the guard pair in:\n{ir}"
    );
    // The `Bool` match carries the alternatives' weights too.
    assert!(
        ir.matches("branch_weights").count() >= 2,
        "expected the guard and the decision weights in:\n{ir}"
    );
}

#[test]
fn weights_on_if_branches_both_spellings() {
    // The condition position annotates the true branch and completes the
    // pair; the branch positions annotate their branch directly.
    let pair = ir_of(
        "if-pair",
        r#"
module Main where
  f :: Bool -> Nat
  f a = if #[weight(2000)] a then 1 else 2

  main :: Int
  main = if f True == 1 then 0 else 1
"#,
    );
    assert!(pair.contains("!{!\"branch_weights\", i32 2000, i32 1}"));
    let explicit = ir_of(
        "if-explicit",
        r#"
module Main where
  f :: Bool -> Nat
  f a = if a then #[weight(2000)] 1 else #[weight(1)] 2

  main :: Int
  main = if f True == 1 then 0 else 1
"#,
    );
    assert!(explicit.contains("!{!\"branch_weights\", i32 2000, i32 1}"));
}

#[test]
fn wrapper_expansions_match_manual_weights() {
    // `#[likely]`, `#[unlikely]` and `#[cold]` are exactly the manual
    // weights 2000, 1 and 0 (attribute annotations specification
    // section 3): the observable weight behavior is identical.
    let wrappers = r#"
module Main where
  data T where
    A :: T
    B :: T
    C :: T

  f :: Bool -> Nat
  f a = if #[likely] a then 1 else 0

  g :: T -> Nat
  g t = case t of
    #[cold] A -> 0
    #[weight(2000)] B -> 1
    #[unlikely] C -> 2

  main :: Int
  main = if g A == 0 then 0 else 1
"#;
    let manual = r#"
module Main where
  data T where
    A :: T
    B :: T
    C :: T

  f :: Bool -> Nat
  f a = if #[weight(2000)] a then 1 else 0

  g :: T -> Nat
  g t = case t of
    #[weight(0)] A -> 0
    #[weight(2000)] B -> 1
    #[weight(1)] C -> 2

  main :: Int
  main = if g A == 0 then 0 else 1
"#;
    let w = ir_of("wrappers", wrappers);
    let m = ir_of("manual", manual);
    let first_diff = w
        .lines()
        .zip(m.lines())
        .enumerate()
        .find(|(_, (a, b))| a != b);
    match first_diff {
        None => assert_eq!(
            w.lines().count(),
            m.lines().count(),
            "the wrappers must expand to exactly the manual weights (length)"
        ),
        Some((i, (a, b))) => panic!(
            "the wrappers must expand to exactly the manual weights; first difference at line {}:\n  wrappers: {a}\n  manual:   {b}",
            i + 1
        ),
    }
}

#[test]
fn marginalization_sums_clause_weights() {
    // The reference example of section 2.2: the outer alternatives take
    // the marginal sums (2100, 100) and each inner decision keeps its
    // clause weights.
    let ir = ir_of(
        "marginal",
        r#"
module Main where
  data M where
    N :: M
    J :: M

  f :: M -> Bool -> Nat
  f #[weight(100)] N True = 1
  f #[weight(2000)] N False = 2
  f #[weight(50)] J True = 3
  f #[weight(50)] J False = 4

  main :: Int
  main = if f N True == 1 then 0 else 1
"#,
    );
    assert!(ir.contains("!{!\"branch_weights\", i32 0, i32 2100, i32 100}"), "{ir}");
    assert!(ir.contains("!{!\"branch_weights\", i32 100, i32 2000}"), "{ir}");
    assert!(ir.contains("!{!\"branch_weights\", i32 50, i32 50}"), "{ir}");
}

#[test]
fn partial_annotation_drops_all_weights_and_reports() {
    let (program, _ifaces, diags) = check_with_diags(
        r#"
module Main where
  data T where
    A :: T
    B :: T

  partial :: T -> Nat
  partial t = case t of
    #[weight(5)] A -> 0
    B -> 1

  main :: Int
  main = if partial A == 0 then 0 else 1
"#,
    )
    .expect("partial annotation is not an error");
    let reported: Vec<&Diagnostic> = diags
        .iter()
        .filter(|d| d.code == "incomplete-branch-weights")
        .collect();
    assert_eq!(reported.len(), 1, "expected one report, got {diags:?}");
    assert!(!reported[0].informational, "the drop report is a warning");
    // All weights of the decision point are dropped: none reach LLVM.
    let ir = cinch::codegen::emit(&program).expect("emit");
    assert!(
        !ir.contains("branch_weights"),
        "partial annotation must drop ALL weights of the decision point:\n{ir}"
    );
}

#[test]
fn partial_if_annotation_drops_and_reports() {
    let (_program, _ifaces, diags) = check_with_diags(
        r#"
module Main where
  f :: Bool -> Nat
  f a = if a then #[weight(5)] 1 else 2

  main :: Int
  main = if f True == 1 then 0 else 1
"#,
    )
    .expect("partial annotation is not an error");
    assert!(
        diags.iter().any(|d| d.code == "incomplete-branch-weights"),
        "expected `incomplete-branch-weights`, got {diags:?}"
    );
}

#[test]
fn incoherent_weights_fire_informationally_on_wildcard_inflation() {
    // The wildcard clause is reachable through both constructors, so its
    // weight is counted at more than one path (section 2.3).
    let (program, _ifaces, diags) = check_with_diags(
        r#"
module Main where
  data T where
    A :: T
    B :: T

  f :: T -> Bool -> Nat
  f #[weight(10)] _ True = 1
  f #[weight(5)] A False = 2
  f #[weight(5)] B False = 3

  main :: Int
  main = if f A True == 1 then 0 else 1
"#,
    )
    .expect("inflation is not an error");
    let reported: Vec<&Diagnostic> = diags
        .iter()
        .filter(|d| d.code == "incoherent-weights")
        .collect();
    assert_eq!(reported.len(), 1, "expected one report, got {diags:?}");
    assert!(reported[0].informational, "the inflation report is informational");
    // The weights remain usable per decision point.
    let ir = cinch::codegen::emit(&program).expect("emit");
    assert!(ir.contains("branch_weights"), "the weights must survive:\n{ir}");
}

#[test]
fn two_same_name_attributes_are_rejected() {
    expect_err(
        "dup-inline",
        r#"
module Main where
  #[inline]
  #[inline]
  f :: Nat -> Nat
  f x = x
"#,
        "two `inline` attributes",
    );
    expect_err(
        "dup-weight",
        r#"
module Main where
  data T where
    A :: T
    B :: T

  f :: T -> Nat
  f t = case t of
    #[weight(1)] #[weight(2)] A -> 0
    #[weight(1)] B -> 1
"#,
        "two `weight` attributes",
    );
    expect_err(
        "dup-meta",
        r#"
module Main where
  #[meta(a = 1)]
  #[meta(b = 2)]
  f :: Nat -> Nat
  f x = x
"#,
        "two `meta` attributes",
    );
}

#[test]
fn weight_argument_grammar() {
    expect_err(
        "weight-range",
        r#"
module Main where
  data T where
    A :: T
    B :: T

  f :: T -> Nat
  f t = case t of
    #[weight(4294967296)] A -> 0
    #[weight(1)] B -> 1
"#,
        "32 bits",
    );
    expect_err(
        "weight-not-a-literal",
        r#"
module Main where
  data T where
    A :: T
    B :: T

  f :: T -> Nat
  f t = case t of
    #[weight(x)] A -> 0
    #[weight(1)] B -> 1
"#,
        "non-negative integer literal",
    );
    expect_err(
        "weight-placement",
        r#"
module Main where
  #[weight(1)]
  f :: Nat -> Nat
  f x = x
"#,
        "between the name and its patterns",
    );
}

// ------------------------------------------------------------- inline (section 5)

#[test]
fn inline_hints_reach_llvm() {
    let ir = ir_of(
        "inline-hints",
        r#"
module Main where
  #[inline]
  a :: Nat -> Nat
  a x = x

  #[inline(always)]
  b :: Nat -> Nat
  b x = x

  #[noinline]
  c :: Nat -> Nat
  c x = x

  #[inline(never)]
  d :: Nat -> Nat
  d x = x

  #[cold]
  e :: Nat -> Nat
  e x = x

  main :: Int
  main = if a 1 == 1 then 0 else 1
"#,
    );
    assert!(ir.contains("inlinehint"), "{ir}");
    assert!(ir.contains("alwaysinline"), "{ir}");
    // The group carries the hint alongside the unit's LLVM contract
    // attributes (the contract surface), so the check is the hint's
    // presence in the attribute vocabulary.
    assert!(ir.contains("noinline"), "{ir}");
    // `#[noinline]` and `#[inline(never)]` are the same hint: both
    // definitions take the `noinline` attribute group.
    assert!(ir.contains("@\"Main.c\"(i64 %p0) #"), "{ir}");
    assert!(ir.contains("@\"Main.d\"(i64 %p0) #"), "{ir}");
    assert!(ir.contains("cold"), "{ir}");
}

#[test]
fn noinline_and_inline_never_together_are_a_duplicate() {
    expect_err(
        "noinline-plus-never",
        r#"
module Main where
  #[noinline]
  #[inline(never)]
  f :: Nat -> Nat
  f x = x
"#,
        "one inline hint",
    );
    expect_err(
        "noinline-args",
        r#"
module Main where
  #[noinline(always)]
  f :: Nat -> Nat
  f x = x
"#,
        "takes no arguments",
    );
}

#[test]
fn inline_hint_placement_is_checked() {
    expect_err(
        "inline-on-alt",
        r#"
module Main where
  data T where
    A :: T
    B :: T

  f :: T -> Nat
  f t = case t of
    #[inline] A -> 0
    B -> 1
"#,
        "function declaration",
    );
}

// ---------------------------------------------------------- repr and layout (section 4)

#[test]
fn repr_align_only_raises() {
    expect_err(
        "align-below-natural-field",
        r#"
module Main where
  #[repr(C)]
  record R where
    #[repr(align(4))] x :: U64
"#,
        "only raises",
    );
    expect_err(
        "align-below-natural-type",
        r#"
module Main where
  #[repr(C, align(4))]
  record R where
    x :: U64
"#,
        "only raises",
    );
}

#[test]
fn packed_field_borrow_is_rejected_and_copy_works() {
    expect_err(
        "packed-borrow",
        r#"
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32

  borrowB :: forall region. R -> Ref region U32
  borrowB r = borrow r.b

  main :: Int
  main = 0
"#,
        "cannot be borrowed",
    );
    expect_err(
        "packed-mut-borrow",
        r#"
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32

  mutBorrowB :: forall region. R -> Mut region U32
  mutBorrowB r = mutBorrow r.b

  main :: Int
  main = 0
"#,
        "cannot be borrowed",
    );
    // A plain copy of the same misaligned field works and runs.
    expect_ok(
        "packed-copy",
        r#"
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32

  copyB :: R -> U32
  copyB r = r.b

  main :: Int
  main = if copyB (R { a = 1, b = 7 }) == 7 then 0 else 1
"#,
        0,
    );
}

#[test]
fn repr_transparent_round_trips_layout() {
    let iface = iface_of(
        "transparent",
        r#"
module Main where
  #[repr(transparent)]
  record W where
    x :: U32

  wrap :: U32 -> W
  wrap n = W { x = n }

  unwrap :: W -> U32
  unwrap w = w.x
"#,
    );
    let w = iface
        .layouts
        .iter()
        .find(|l| l.name == "W")
        .expect("W layout");
    assert_eq!(w.repr, "transparent");
    // Exactly the field's layout: `U32` is 4 bytes at alignment 4
    // (primitive types specification section 2).
    assert_eq!((w.size, w.align), (4, 4));
    assert_eq!(w.fields.len(), 1);
    assert_eq!(w.fields[0].name, "x");
    assert_eq!((w.fields[0].offset, w.fields[0].size, w.fields[0].align), (0, 4, 4));
    // Taking the wrapper apart is layout-preserving at run time.
    expect_ok(
        "transparent-round-trip",
        r#"
module Main where
  #[repr(transparent)]
  record W where
    x :: U32

  wrap :: U32 -> W
  wrap n = W { x = n }

  unwrap :: W -> U32
  unwrap w = w.x

  main :: Int
  main = if unwrap (wrap 7) == 7 then 0 else 1
"#,
        0,
    );
}

#[test]
fn repr_layout_facts_recorded_in_the_abi_table() {
    let iface = iface_of(
        "abi-table",
        r#"
module Main where
  #[repr(C, packed(2))]
  record P where
    a :: U8
    b :: U32
    c :: U16
"#,
    );
    let p = iface
        .layouts
        .iter()
        .find(|l| l.name == "P")
        .expect("P layout");
    assert_eq!(p.repr, "C, packed(2)");
    // Field alignments are capped at 2: offsets 0, 2, 6; size 8.
    assert_eq!(
        (p.fields[0].offset, p.fields[0].size, p.fields[0].align),
        (0, 1, 1)
    );
    assert_eq!(
        (p.fields[1].offset, p.fields[1].size, p.fields[1].align),
        (2, 4, 4)
    );
    assert_eq!(
        (p.fields[2].offset, p.fields[2].size, p.fields[2].align),
        (6, 2, 2)
    );
    assert_eq!((p.size, p.align), (8, 2));
}

#[test]
fn repr_combination_rules() {
    expect_err(
        "transparent-exclusive",
        r#"
module Main where
  #[repr(transparent, C)]
  record W where
    x :: U32
"#,
        "combines with no other representation",
    );
    expect_err(
        "packed-align-conflict",
        r#"
module Main where
  #[repr(C, packed, align(8))]
  record R where
    x :: U32
"#,
        "conflict",
    );
    expect_err(
        "align-not-power-of-two",
        r#"
module Main where
  #[repr(C, align(3))]
  record R where
    x :: U32
"#,
        "power of two",
    );
    expect_err(
        "repr-on-multi-constructor-data",
        r#"
module Main where
  #[repr(C)]
  data T where
    A :: T
    B :: T
"#,
        "multi-constructor",
    );
    expect_err(
        "field-repr-needs-repr-c",
        r#"
module Main where
  record R where
    #[repr(align(8))] x :: U32
"#,
        "`repr(C)` record",
    );
    expect_err(
        "repr-unknown-item",
        r#"
module Main where
  #[repr(sizzle)]
  record R where
    x :: U32
"#,
        "not a representation",
    );
}

// ------------------------------------------------------ identity (sections 1.3, 4.3)

#[test]
fn repr_change_changes_the_interface_hash() {
    let c = iface_of(
        "repr-c",
        r#"
module Main where
  #[repr(C)]
  record R where
    x :: U32
"#,
    );
    let transparent = iface_of(
        "repr-transparent",
        r#"
module Main where
  #[repr(transparent)]
  record R where
    x :: U32
"#,
    );
    let plain = iface_of(
        "repr-none",
        r#"
module Main where
  record R where
    x :: U32
"#,
    );
    let aligned = iface_of(
        "repr-aligned",
        r#"
module Main where
  #[repr(C, align(16))]
  record R where
    x :: U32
"#,
    );
    // A representation attribute is semantic: it is in the contract
    // projection, so every change rechecks dependents.
    assert_ne!(c.iface_hash(), transparent.iface_hash());
    assert_ne!(c.iface_hash(), plain.iface_hash());
    assert_ne!(c.iface_hash(), aligned.iface_hash());
    assert_ne!(transparent.iface_hash(), plain.iface_hash());
    assert_ne!(transparent.iface_hash(), aligned.iface_hash());
    assert_ne!(plain.iface_hash(), aligned.iface_hash());
}

#[test]
fn hints_never_change_the_interface_hash() {
    let plain = iface_of(
        "hints-plain",
        r#"
module Main where
  f :: Bool -> Nat
  f True = 1
  f False = 0
"#,
    );
    let weighted = iface_of(
        "hints-weighted",
        r#"
module Main where
  f :: Bool -> Nat
  f #[weight(2000)] True = 1
  f #[weight(1)] False = 0
"#,
    );
    let reweighted = iface_of(
        "hints-reweighted",
        r#"
module Main where
  f :: Bool -> Nat
  f #[cold] True = 1
  f #[likely] False = 0
"#,
    );
    let inlined = iface_of(
        "hints-inlined",
        r#"
module Main where
  #[inline(always)]
  f :: Bool -> Nat
  f True = 1
  f False = 0
"#,
    );
    let noinlined = iface_of(
        "hints-noinlined",
        r#"
module Main where
  #[noinline]
  f :: Bool -> Nat
  f True = 1
  f False = 0
"#,
    );
    // Hint attributes are erasable: they never enter the contract
    // projection (attribute annotations specification section 1.3).
    let base = plain.iface_hash();
    assert_eq!(base, weighted.iface_hash(), "weights are hints");
    assert_eq!(base, reweighted.iface_hash(), "wrappers are weights");
    assert_eq!(base, inlined.iface_hash(), "inline hints are hints");
    assert_eq!(base, noinlined.iface_hash(), "noinline is a hint");
}

#[test]
fn hints_do_not_change_type_checking_outcomes() {
    // The same program with and without hints produces the same values.
    expect_ok(
        "hints-semantics-plain",
        r#"
module Main where
  f :: Bool -> Nat
  f True = 1
  f False = 0

  main :: Int
  main = if f True == 1 then 0 else 1
"#,
        0,
    );
    expect_ok(
        "hints-semantics-hinted",
        r#"
module Main where
  #[inline(always)]
  f :: Bool -> Nat
  f #[likely] True = 1
  f #[unlikely] False = 0

  main :: Int
  main = if f True == 1 then 0 else 1
"#,
        0,
    );
}
