//! Refinement types (P28; decision D151, `design/refinement-types.md`):
//! decidable-predicate refinements whose witness is `Prop`-erased, so
//! the runtime representation IS the container. The acceptance matrix:
//! the surface spelling (exercised positively and negatively), the
//! erased representation (an IR observation: no wrapper allocation, the
//! projection is the identity), discharge in the `Dec`/`decide` world
//! (the total constructor's `Option`, direct construction with an
//! erased `Dec` argument, no forged and no stale witness),
//! preservation-checked instance transfer through the P18 law layer
//! (checked samples, rejections naming the preservation law, and the
//! honest `not checked` boundary), and the four refinements with their
//! constructors (`NonEmpty` replacing the bespoke type, `Sorted`,
//! `NonZero`, `Ascii`).

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{check_sources, driver_with_profile, run_source, Mode};
use std::path::PathBuf;

/// Assert that checking the program fails with errors containing every
/// `needle`.
fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected an error, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(
                    msg.contains(n),
                    "{name}: error {msg:?} does not contain {n:?}"
                );
            }
        }
    }
}

/// Assert that the program checks.
fn expect_checks(name: &str, src: &str) {
    if let Err(e) = check_sources(src, name) {
        panic!("{name}: compilation failed: {e}");
    }
}

/// A program that must compile and run to `expected`.
fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// The emitted LLVM IR of one checked source (every IR test carries a
/// `main`, which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The body of one unit's define (its lines between `{` and `}`).
fn body_of(ir: &str, unit: &str) -> String {
    let quoted = format!("@\"{unit}\"(");
    let bare = format!("@{unit}(");
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if l.starts_with("define ") && (l.contains(&quoted) || l.contains(&bare)) {
            let mut body = String::new();
            for l in lines.by_ref() {
                if l == "}" {
                    return body;
                }
                body.push_str(l);
                body.push('\n');
            }
        }
    }
    panic!("{unit}: no body in {ir}")
}

/// Write one source (and any extra `(file, source)` modules beside it)
/// to a fresh temporary project and return its directory.
fn write_project_files(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch-refinements-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project");
    for (file, src) in files {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    dir
}

/// Write one source to a fresh temporary project and return its
/// directory.
fn write_project(name: &str, src: &str) -> PathBuf {
    write_project_files(name, &[("Main.cinch", src)])
}

/// The artifact manifest of one checked source (the law rows and their
/// honest statuses).
fn manifest_of(name: &str, src: &str) -> String {
    let dir = write_project(name, src);
    driver_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir.join("Main.cinch"),
        None,
        &TargetProfile::native(),
    )
    .unwrap_or_else(|e| panic!("{name}: check run failed: {e}"));
    std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("{name}: manifest missing: {e}"))
}

// ------------------------------------------------------ the surface spelling

#[test]
fn surface_spelling_declares_the_refinement() {
    // The headline spelling (`design/refinement-types.md`): `type R p =
    // refine (x :: T) with P`, the dependent binder taking `::` or `:`.
    expect_run(
        "spelling",
        r#"
module Main where
  import Data.NonEmpty

  type Small = refine (n :: Nat) with n < 5
  type SmallColon = refine (n : Nat) with n < 5

  main :: Int
  main = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of
    None -> 1
    Some ne ->
      let s = Small 3 (decide (3 < 5)) in
      let c = SmallColon 3 (decide (3 < 5)) in
      if unrefine s == 3 && unrefine c == 3 then 0 else 2
"#,
        0,
    );
}

#[test]
fn surface_spelling_refuses_a_wrong_predicate_naming_it() {
    // A predicate over the wrong container is refused naming the
    // predicate and its admissible operand.
    expect_err(
        "wrong-pred-operand",
        r#"
module Main where
  type Bad = refine (xs :: List Nat) with NonZero xs

  main :: Int
  main = 0
"#,
        &["`NonZero` decides numeric values", "List Nat"],
    );
    // A comparison over a non-numeric container is refused naming the
    // form.
    expect_err(
        "wrong-pred-form",
        r#"
module Main where
  type Bad = refine (xs :: List Nat) with xs < 5

  main :: Int
  main = 0
"#,
        &["comparison or index equality"],
    );
    // A predicate that does not speak about the refined binder is
    // refused naming the binder.
    expect_err(
        "wrong-pred-binder",
        r#"
module Main where
  type Bad = refine (xs :: List Nat) with NotNull (Cons 1 Nil)

  main :: Int
  main = 0
"#,
        &["must mention the refined binder"],
    );
    // A `type` alias is not a refinement: refused naming the form.
    expect_err(
        "type-alias",
        "module Main where\n  type T = Int\n\n  main :: Int\n  main = 0\n",
        &["a `type` declaration is a refinement"],
    );
}

#[test]
fn unprovable_discharge_rejects_naming_it() {
    // A disproved discharge names the discharge (no refined value can be
    // constructed here).
    expect_err(
        "disproved-discharge",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Small
  bad = Small 9 (decide (9 < 5))

  main :: Int
  main = 0
"#,
        &["the discharge `9 < 5` is disproved"],
    );
    // A witness of the wrong proposition names both propositions.
    expect_err(
        "wrong-witness",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Nat -> Small
  bad k = case decide (3 < 9) of
    Yes w -> Small k w
    No _ -> Small k (decide (3 < 9))

  main :: Int
  main = 0
"#,
        &["is evidence of `3 < 9`, not of `k < 5`"],
    );
    // A discharge that only a runtime decision can settle is refused at
    // a construction that erases its witness, naming the discharge and
    // the way to discharge it.
    expect_err(
        "unresolved-discharge",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Nat -> Small
  bad k = Small k (decide (k < 5))

  main :: Int
  main = 0
"#,
        &["the discharge `k < 5` is not decided at this construction"],
    );
}

// -------------------------------------------------- the erased representation

#[test]
fn erased_representation_is_the_container() {
    // The projection is the identity function at runtime (no wrapper
    // allocation, no load: the container IS the value), and the
    // transparent constructor's unit is its container's own
    // construction, byte for byte.
    let ir = ir_of(
        "erasure",
        r#"
module Main where
  import Data.NonEmpty

  proj :: forall a. NonEmpty a -> List a
  proj ne = unrefine ne

  list1 :: List Int
  list1 = Cons (1 :: Int) Nil

  ne1 :: NonEmpty Int
  ne1 = NonEmpty (Cons (1 :: Int) Nil) (decide (NotNull (Cons (1 :: Int) Nil)))

  main :: Int
  main = 0
"#,
    );
    let proj = body_of(&ir, "Main.proj");
    assert!(
        !proj.contains("cinch_alloc") && !proj.contains("call"),
        "the projection runs at zero cost, got:\n{proj}"
    );
    assert!(
        proj.contains("ret "),
        "the projection returns its argument, got:\n{proj}"
    );
    let ne1 = body_of(&ir, "Main.ne1");
    let list1 = body_of(&ir, "Main.list1");
    assert!(
        !ne1.contains("cinch_alloc") && !ne1.contains("NonEmpty"),
        "a refined value's construction allocates no wrapper, got:\n{ne1}"
    );
    assert_eq!(
        ne1, list1,
        "the refined construction IS the container's construction (the witness erases)"
    );
}

// ------------------------------------------------------------- the discharge

#[test]
fn total_constructor_returns_option() {
    // The total API: `non-empty` discharges at runtime and returns
    // `Option` (refinement types design: `from-list : List a -> Option
    // (NonEmpty a)`). The checks are top-level because a `case` in a
    // `let` binder's right-hand side must indent past the binder's
    // column under the layout rules.
    expect_run(
        "total-ctor",
        r#"
module Main where
  import Category.Foldable
  import Data.NonEmpty

  check1 :: Int
  check1 = case non-empty (Cons (1 :: Int) Nil) of
    None -> 1
    Some ne -> if length ne == 1 then 0 else 2

  check2 :: Int
  check2 = case non-empty (Nil :: List Int) of
    None -> 0
    Some _ -> 4

  main :: Int
  main = check1 + check2
"#,
        0,
    );
}

#[test]
fn direct_construction_checks_and_erases() {
    // Direct construction with the `Dec` discharge: checked at the
    // construction, erased from the runtime form.
    expect_run(
        "direct",
        r#"
module Main where
  import Data.NonEmpty

  type Small = refine (n :: Nat) with n < 5

  ne :: NonEmpty Int
  ne = NonEmpty (Cons (7 :: Int) (Cons 8 Nil)) (decide (NotNull (Cons (7 :: Int) (Cons 8 Nil))))

  small :: Small
  small = Small 3 (decide (3 < 5))

  main :: Int
  main =
    case unrefine ne of
      Nil -> 1
      Cons h t -> if h == 7 && unrefine small == 3 then 0 else 2
"#,
        0,
    );
}

#[test]
fn forged_witness_cannot_be_constructed() {
    // `Fin 7 refl` for `Fin 3` is refused (dependent.rs); the refinement
    // constructor refuses a forged witness exactly so.
    expect_err(
        "forged-refl",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Small
  bad = Small 9 refl

  main :: Int
  main = 0
"#,
        &["9 < 5"],
    );
    // A `Dec` argument that does not decide its discharge cannot forge
    // the construction either: the discharge itself is refuted.
    expect_err(
        "forged-dec",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: (d :: Dec (3 < 9)) -> Small
  bad d = Small 9 d

  main :: Int
  main = 0
"#,
        &["`9 < 5` is false"],
    );
}

#[test]
fn stale_witness_cannot_be_constructed() {
    // The evidence-invalidation interplay (decisions D192-D197): a
    // witness about mutable state dies on the write, so a refined
    // construction cannot consume it afterward.
    expect_err(
        "stale-witness",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  prog :: forall region. Mut region Nat -> Option Small
  prog m =
    case decide (copyOut m < 5) of
      Yes w ->
        let u = refSet m 9 in
        Some (Small (copyOut m) w)
      No _ -> None

  main :: Int
  main = 0
"#,
        &["stale evidence", "invalidated by the write"],
    );
    // The same witness used before the write constructs the refinement.
    expect_checks(
        "witness-before-write",
        r#"
module Main where
  type Small = refine (n :: Nat) with n < 5

  prog :: forall region. Mut region Nat -> Option Small
  prog m =
    case decide (copyOut m < 5) of
      Yes w -> Some (Small (copyOut m) w)
      No _ -> None

  main :: Int
  main = 0
"#,
    );
}

// -------------------------------------------- preservation-checked transfer

#[test]
fn semigroup_nonempty_passes_its_law_samples() {
    // `Semigroup (NonEmpty a)` transfers (the container's own `<>`): the
    // class law AND the preservation law decide over samples, and the
    // manifest reports both honestly.
    let m = manifest_of(
        "semigroup-nonempty",
        r#"
module Main where
  import Data.NonEmpty

  main :: Int
  main = 0
"#,
    );
    let assoc = m
        .find("Semigroup.assoc")
        .or_else(|| m.find("\"assoc\""))
        .unwrap_or_else(|| panic!("the assoc law row is in the manifest: {m}"));
    let row = &m[assoc.saturating_sub(200)..((assoc + 300).min(m.len()))];
    assert!(
        row.contains("Semigroup") && row.contains("checked ("),
        "{row}"
    );
    let pres = m
        .find("NonEmpty.preserve-<>")
        .unwrap_or_else(|| panic!("the preservation row is in the manifest: {m}"));
    let row = &m[pres.saturating_sub(200)..((pres + 300).min(m.len()))];
    assert!(
        row.contains("Data.NonEmpty.Semigroup_NonEmpty") && row.contains("checked ("),
        "{row}"
    );
}

#[test]
fn monoid_over_nonempty_is_rejected_naming_the_preservation_law() {
    // `Monoid` cannot transfer to `NonEmpty`: `mempty` is empty, so the
    // preservation law rejects the instance naming it (the design
    // note's `Semigroup` yes / `Monoid` no).
    expect_err(
        "monoid-nonempty",
        r#"
module Main where
  import Data.NonEmpty
  import Algebra.Monoid

  implicit instance Monoid_NonEmpty x :: Monoid (NonEmpty x) where
    mempty = Nil

  main :: Int
  main = 0
"#,
        &["preservation law `NonEmpty.preserve-mempty`"],
    );
}

#[test]
fn sorted_reverse_append_is_rejected_naming_the_preservation_law() {
    // A deliberately non-preserving method: reverse-append produces
    // unsorted outputs from sorted inputs, so the preservation law
    // rejects the instance naming it (its associativity holds: only
    // preservation refutes it). The prelude's own `Semigroup Sorted`
    // (the merge) is shadowed out of the project so the reverse
    // instance is the only one over the carrier (the coherence rule).
    let sorted = include_str!("../prelude/Data/Sorted.cinch");
    let sorted_no_instance: String = sorted
        .lines()
        .filter(|l| {
            !l.contains("Semigroup_Sorted") && !l.contains("(<>) p q = merge p q")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let dir = write_project_files(
        "sorted-reverse",
        &[
            ("Sorted.cinch", &sorted_no_instance),
            (
                "Main.cinch",
                r#"
module Main where
  import Data.Sorted
  import Algebra.Semigroup

  infixr 6 <>

  implicit instance Semigroup_SortedReverse :: Semigroup Sorted where
    (<>) p q = (<>) q p

  main :: Int
  main = 0
"#,
            ),
        ],
    );
    match cinch::driver::driver(
        Mode::Check,
        &dir.join("Main.cinch"),
        None,
    ) {
        Ok(_) => panic!("sorted-reverse: expected a preservation rejection, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("preservation law `Sorted.preserve-<>`"),
                "sorted-reverse: error {msg:?} does not name the preservation law"
            );
        }
    }
}

#[test]
fn unprovable_transfers_report_not_checked_honestly() {
    // The honest boundary (the P18 law tier): a refined result inside
    // another type former is not decided at samples (`Traversable`'s
    // `traverse : ... -> f (NonEmpty b)`), and laws over methods the
    // instance does not carry report `not checked`, never a false pass.
    let m = manifest_of(
        "not-checked",
        r#"
module Main where
  import Data.NonEmpty
  import Algebra.Monoid

  main :: Int
  main = 0
"#,
    );
    let pres = m
        .find("NonEmpty.preserve-traverse")
        .unwrap_or_else(|| panic!("the nested preservation row is in the manifest: {m}"));
    let row = &m[pres.saturating_sub(200)..((pres + 300).min(m.len()))];
    assert!(
        row.contains("Traversable_NonEmpty") && row.contains("not checked"),
        "{row}"
    );
    assert!(
        m.contains(
            "{\"class\":\"Monoid\",\"instance\":\"Data.NonZero.Monoid_NonZero\",\"law\":\"left-unit\",\"status\":\"not checked"
        ),
        "the unit law over the methods the instance does not carry reports `not checked`: {m}"
    );
}

#[test]
fn nonzero_multiplicative_transfer_checks() {
    // `NonZero`'s multiplicative transfer: the product of non-zeros is
    // non-zero and 1 is non-zero, so both the `Semigroup` and the
    // `Monoid` preservation laws decide `checked` (the contrast with
    // `Monoid (NonEmpty a)`, whose identity violates its predicate).
    let m = manifest_of(
        "nonzero-transfer",
        r#"
module Main where
  import Data.NonZero

  main :: Int
  main = 0
"#,
    );
    for law in ["NonZero.preserve-<>", "NonZero.preserve-mempty"] {
        let at = m
            .find(law)
            .unwrap_or_else(|| panic!("the {law} row is in the manifest: {m}"));
        let row = &m[at.saturating_sub(200)..((at + 300).min(m.len()))];
        assert!(row.contains("checked ("), "{row}");
    }
}

// --------------------------------------------------- the four refinements

#[test]
fn the_four_refinements_exist_with_their_constructors() {
    // `NonEmpty` (over `List`), `Sorted` (over `List Nat`), `NonZero`
    // (over `Int`), `Ascii` (over `String`): each with its total
    // constructor discharging at runtime and its transparent
    // constructor discharging statically.
    expect_run(
        "four",
        r#"
module Main where
  import Category.Foldable
  import Data.NonEmpty
  import Data.Sorted
  import Data.NonZero
  import Data.Ascii
  import Data.String

  check1 :: Int
  check1 = case non-empty (Cons (1 :: Int) Nil) of
    None -> 1
    Some ne -> if length ne == 1 then 0 else 2

  check2 :: Int
  check2 = case to-sorted (Cons 1 (Cons 2 (Cons 3 Nil))) of
    None -> 4
    Some s -> if length (unrefine s) == 3 then 0 else 8

  check3 :: Int
  check3 = case non-zero (0 - 3) of
    None -> 16
    Some z -> if unrefine z == 0 - 3 then 0 else 32

  check4 :: Int
  check4 = case string-from-bytes (bytes-from-list (Cons 65 (Cons 66 Nil))) of
    None -> 512
    Some s -> case to-ascii s of
      None -> 64
      Some a -> if string-length (unrefine a) == 2 then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4
"#,
        0,
    );
    // The negative half: each total constructor refuses its non-values.
    expect_run(
        "four-negative",
        r#"
module Main where
  import Data.NonEmpty
  import Data.Sorted
  import Data.NonZero
  import Data.Ascii
  import Data.String

  check1 :: Int
  check1 = case non-empty (Nil :: List Int) of
    None -> 0
    Some _ -> 1

  check2 :: Int
  check2 = case to-sorted (Cons 2 (Cons 1 Nil)) of
    None -> 0
    Some _ -> 2

  check3 :: Int
  check3 = case non-zero 0 of
    None -> 0
    Some _ -> 4

  check4 :: Int
  check4 = case string-from-bytes (bytes-from-list (Cons 195 (Cons 169 Nil))) of
    None -> 1024
    Some s -> case to-ascii s of
      None -> 0
      Some _ -> 8

  main :: Int
  main = check1 + check2 + check3 + check4
"#,
        0,
    );
}

#[test]
fn the_bespoke_nonempty_type_is_gone() {
    // The old data type's constructor shape (a head and a tail) does
    // not type: the refinement's constructor takes the list and its
    // witness.
    expect_err(
        "bespoke-ctor",
        r#"
module Main where
  import Data.NonEmpty

  main :: Int
  main = case NonEmpty (1 :: Int) (Cons 2 Nil) of
    ne -> 0
"#,
        &["type mismatch", "List"],
    );
    // And it has no constructor patterns: matching goes through
    // `unrefine` to the container's constructors.
    expect_err(
        "bespoke-pattern",
        r#"
module Main where
  import Data.NonEmpty

  f :: NonEmpty Int -> Int
  f ne = case ne of
    NonEmpty h t -> h

  main :: Int
  main = 0
"#,
        &["transparent constructor", "no constructor patterns"],
    );
}

#[test]
fn refined_values_project_explicitly() {
    // No silent subtyping (the design note's conservative first answer):
    // a refined value is not its base type; the projection is explicit.
    expect_err(
        "no-silent-subtyping",
        r#"
module Main where
  import Data.NonEmpty

  f :: List Int -> Int
  f xs = 0

  g :: NonEmpty Int -> Int
  g ne = f ne

  main :: Int
  main = 0
"#,
        &["NonEmpty"],
    );
    // With the explicit projection the same program checks and runs.
    expect_run(
        "explicit-projection",
        r#"
module Main where
  import Data.NonEmpty

  f :: List Int -> Int
  f xs = length xs

  g :: NonEmpty Int -> Int
  g ne = f (unrefine ne)

  main :: Int
  main = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of
    None -> 1
    Some ne -> if g ne == 2 then 0 else 2
"#,
        0,
    );
}
