//! The refinement lattice (P28-2; `design/refinement-types.md`):
//! compound predicates (`&&`/`||`, measure atoms) with their discharge
//! in the `Dec`/`decide` world (static and runtime), the table-driven
//! implication theory with implicit weakening and checked `narrow`
//! casts, `forget`/ascription partial weakening, predicate-level
//! instance transfer with preservation postconditions, and instance
//! resolution most-specific along the lattice (with the coherence rule
//! for incomparable heads). The acceptance matrix of the design
//! conversation: the `SizedAscii` compound refinement and its flow
//! down the lattice, `narrow` back up (refused without a `Dec`,
//! accepted with one, forged evidence still refused), measure atoms in
//! `decide`, transfer conditioned on preservation (rejecting `SizedAscii`
//! naming the `<= 200` conjunct), the `Sorted`/merge instance
//! overriding the generic rule by specificity, coherence for
//! incomparable constrained instances, and law/postcondition rows in
//! the manifest.

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

/// Write one source (and any extra `(file, source)` modules beside it)
/// to a fresh temporary project and return its directory.
fn write_project_files(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch-lattice-{}-{name}",
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

/// The `SizedAscii` program of the design conversation: a compound
/// predicate with measure atoms over a list of chars.
const SIZED_ASCII_PRELUDE: &str = r#"
module Main where
  type SizedAscii = refine (xs :: List Char) with length xs > 100 && length xs <= 200 && Ascii xs

  chars :: Int -> List Char
  chars k = if k <= 0 then Nil else Cons 'a' (chars (k - 1))

  to-sized-ascii :: List Char -> Option SizedAscii
  to-sized-ascii xs = case decide (length xs > 100 && length xs <= 200 && Ascii xs) of
    Yes w -> Some (SizedAscii xs w)
    No _ -> None
"#;

// ---------------------------------------------------- compound predicates

#[test]
fn compound_predicate_declares_the_lattice_point() {
    // The headline form (compound predicates): `&&` over measure atoms
    // and a vocabulary predicate over the binder. The measure atoms
    // discharge at runtime through the baked `length` unit and the
    // char scan; the value built at the lattice point runs.
    expect_run(
        "sized-ascii",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  check1 :: Int
  check1 = case to-sized-ascii (chars 150) of
    None -> 1
    Some s -> case decide (length (unrefine s) == 150) of
      Yes _ -> 0
      No _ -> 2

  check2 :: Int
  check2 = case to-sized-ascii (chars 50) of
    None -> 0
    Some _ -> 4

  check3 :: Int
  check3 = case to-sized-ascii (chars 250) of
    None -> 0
    Some _ -> 8

  main :: Int
  main = check1 + check2 + check3
"#
        ),
        0,
    );
}

#[test]
fn disjunction_predicates_decide_too() {
    // `||` is the lattice join: a predicate that admits either bound
    // decides by cases (both disjuncts decide the compound).
    expect_run(
        "disj",
        r#"
module Main where
  type Smallish = refine (n :: Nat) with n < 5 || n > 100

  check1 :: Int
  check1 = case decide (3 < 5 || 3 > 100) of
    Yes _ -> 0
    No _ -> 1

  check2 :: Int
  check2 = case decide (50 < 5 || 50 > 100) of
    Yes _ -> 2
    No _ -> 0

  build :: Nat -> Option Smallish
  build n = case decide (n < 5 || n > 100) of
    Yes w -> Some (Smallish n w)
    No _ -> None

  check3 :: Int
  check3 = case build 3 of
    None -> 4
    Some s -> if unrefine s == 3 then 0 else 8

  main :: Int
  main = check1 + check2 + check3
"#,
        0,
    );
}

#[test]
fn measure_atoms_decide_statically_where_the_spine_folds() {
    // Static discharge of a measure atom: the D92 evaluator walks the
    // constructor spine at check time, so the discharge decides at the
    // construction (the erased witness story is unchanged).
    expect_run(
        "static-measure",
        r#"
module Main where
  type SizedAscii = refine (xs :: List Char) with length xs > 1 && length xs <= 200 && Ascii xs

  small :: SizedAscii
  small = SizedAscii (Cons 'a' (Cons 'b' Nil)) (decide (length (Cons 'a' (Cons 'b' Nil)) > 1 && length (Cons 'a' (Cons 'b' Nil)) <= 200 && Ascii (Cons 'a' (Cons 'b' Nil))))

  main :: Int
  main = case decide (length (unrefine small) == 2) of
    Yes _ -> 0
    No _ -> 1
"#,
        0,
    );
    // The disproved side names the discharge (no refined value can be
    // constructed here).
    expect_err(
        "static-measure-disproved",
        r#"
module Main where
  type SizedAscii = refine (xs :: List Char) with length xs > 100 && Ascii xs

  bad :: SizedAscii
  bad = SizedAscii (Cons 'a' Nil) (decide (length (Cons 'a' Nil) > 100 && Ascii (Cons 'a' Nil)))

  main :: Int
  main = 0
"#,
        &["the discharge"],
    );
}

// --------------------------------------------- weakening down the lattice

#[test]
fn ascription_weakens_implicitly_at_zero_cost() {
    // Type ascription is the everyday form of partial weakening: a
    // value at the compound lattice point flows to `refined (Ascii)`
    // with no cast, and the projection stays the identity function.
    expect_run(
        "ascription",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  down :: SizedAscii -> Int
  down s =
    let a = s :: refined (Ascii) (List Char) in
    0

  main :: Int
  main = case to-sized-ascii (chars 150) of
    None -> 1
    Some s -> down s
"#
        ),
        0,
    );
    // The flow is one way: a weaker value does not reach a stronger
    // point implicitly (that is `narrow`'s checked cast).
    expect_err(
        "no-silent-strengthening",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  up :: refined (Ascii) (List Char) -> Int
  up a =
    let s = a :: SizedAscii in
    0

  main :: Int
  main = 0
"#
        ),
        &["does not flow to"],
    );
}

#[test]
fn forget_drops_named_conjuncts_and_names_absent_ones() {
    // `forget [p1, p2] from e` names what is given up (top-level
    // conjuncts only), typing to the conjunction minus those atoms at
    // zero runtime cost; an atom that is not there is an error naming
    // it.
    expect_run(
        "forget",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  down :: SizedAscii -> Int
  down s =
    let b = forget [length xs > 100, length xs <= 200] from s in
    let a = b :: refined (Ascii) (List Char) in
    0

  main :: Int
  main = case to-sized-ascii (chars 150) of
    None -> 1
    Some s -> down s
"#
        ),
        0,
    );
    // The single-atom spelling drops one conjunct.
    expect_checks(
        "forget-single",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  down :: SizedAscii -> Int
  down s =
    let b = forget (length xs <= 200) from s in
    0

  main :: Int
  main = 0
"#
        ),
    );
    // Forgetting an atom that is not a conjunct names it.
    expect_err(
        "forget-absent",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  down :: SizedAscii -> Int
  down s =
    let b = forget [NotNull xs] from s in
    0

  main :: Int
  main = 0
"#
        ),
        &["NotNull", "not a top-level conjunct"],
    );
}

// ------------------------------------------------ the checked `narrow` cast

#[test]
fn narrow_back_up_is_refused_without_a_dec() {
    // Strengthening is always checked: `narrow` carries a `Dec` of the
    // stronger predicate, and the cast without evidence names the
    // evidence it needs.
    expect_err(
        "narrow-no-dec",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  up :: SizedAscii -> Int
  up s =
    let b = forget [length xs > 100, length xs <= 200] from s in
    let s2 = narrow b to SizedAscii in
    0

  main :: Int
  main = 0
"#
        ),
        &["`narrow` carries a `Dec` of the stronger predicate"],
    );
}

#[test]
fn narrow_back_up_is_accepted_with_a_dec() {
    // The same cast carrying its `Dec` (discharged here at runtime by
    // a `decide`; the evidence path is the transparent constructor's).
    expect_run(
        "narrow-with-dec",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  up :: SizedAscii -> Int
  up s =
    let b = forget [length xs > 100, length xs <= 200] from s in
    case decide (length b > 100 && length b <= 200 && Ascii b) of
      Yes w ->
        let s2 = narrow b to SizedAscii with w in
        0
      No _ -> 2

  main :: Int
  main = case to-sized-ascii (chars 150) of
    None -> 1
    Some s -> up s
"#
        ),
        0,
    );
}

#[test]
fn narrow_rejects_forged_dec_evidence() {
    // Forged evidence is refused exactly as at the transparent
    // constructor: a disproved discharge cannot produce evidence, and
    // evidence of the wrong proposition names both propositions.
    expect_err(
        "narrow-forged-dec",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  up :: SizedAscii -> Int
  up s =
    let b = forget [length xs > 100, length xs <= 200] from s in
    let s2 = narrow b to SizedAscii with (decide (3 < 9)) in
    0

  main :: Int
  main = 0
"#
        ),
        &["3 < 9"],
    );
    expect_err(
        "narrow-wrong-witness",
        &format!(
            r#"{SIZED_ASCII_PRELUDE}
  up :: SizedAscii -> Int
  up s =
    let b = forget [length xs > 100, length xs <= 200] from s in
    case decide (3 < 9) of
      Yes w ->
        let s2 = narrow b to SizedAscii with w in
        0
      No _ -> 2

  main :: Int
  main = 0
"#
        ),
        &["is evidence of"],
    );
}

#[test]
fn the_implication_table_drives_the_flow() {
    // Interval containment (`> 1 && < 9` implies `> 0 && < 10`) and
    // the syntactic rows decide the flow; unknown pairs are not
    // implied (the cast needs a `Dec`, or refuses).
    expect_checks(
        "intervals",
        r#"
module Main where
  type Inner = refine (n :: Nat) with n > 1 && n < 9
  type Outer = refine (n :: Nat) with n > 0 && n < 10

  down :: Inner -> Int
  down v =
    let w = v :: Outer in
    0

  main :: Int
  main = 0
"#,
    );
    // `> 1` alone does not reach `> 5` (the interval is not
    // contained): the flow refuses, naming the obligation that no fact
    // in scope discharges.
    expect_err(
        "unknown-not-implied",
        r#"
module Main where
  type A = refine (n :: Nat) with n > 1
  type B = refine (n :: Nat) with n > 5

  down :: A -> Int
  down v =
    let w = v :: B in
    0

  main :: Int
  main = 0
"#,
        &["the obligation `v > 5`"],
    );
    // The documented vocabulary row: `length xs > 0` implies
    // `NotNull xs`.
    expect_checks(
        "vocab-row",
        r#"
module Main where
  import Data.NonEmpty

  type NonNullish = refine (xs :: List Int) with length xs > 0

  down :: NonNullish -> Int
  down v =
    let w = v :: NonEmpty Int in
    0

  main :: Int
  main = 0
"#,
    );
}

// ============================ instance half: the acceptance matrix ============================
//
// Predicate-level transfer with preservation postconditions, the
// generic `Preserves` transfer against the specialized `Sorted`/merge
// instance (most-specific resolution), the coherence rule for
// incomparable heads, `using` obligations discharged through the `Dec`
// evidence path, and the manifest's law/postcondition rows round-tripped
// through `.cinchif`.

/// The generic transfer instance of the design's shape: the container's
/// `<>` at any point whose predicate it preserves (`Preserves (<>) p`).
const GENERIC_TRANSFER: &str = r#"
  implicit instance Semigroup_Refined p a :: using (sb :: Semigroup (List a), Preserves (<>) p) => Semigroup (refined p (List a)) where
    (<>) x y = (<>) x y
"#;

#[test]
fn transfer_rejects_sized_ascii_naming_the_conjunct() {
    // `Semigroup (List Char)` at `refined SizedAscii`: append grows
    // length, so the preservation law rejects the transfer naming the
    // `<= 200` conjunct (the compound predicates design: a failure
    // names the conjunct that does not survive).
    expect_err(
        "transfer-sized-ascii",
        r#"
module Main where
  import Algebra.Semigroup

  infixr 6 <>

  type SizedAscii = refine (xs :: List Char) with length xs > 100 && length xs <= 200 && Ascii xs

  combine :: SizedAscii -> SizedAscii -> SizedAscii
  combine a b = a <> b

  main :: Int
  main = 0
"#,
        &["SizedAscii.preserve-<>", "length xs <= 200"],
    );
}

#[test]
fn transfer_accepts_non_empty_with_preservation_evidence() {
    // `Semigroup (List Char)` at `refined NonEmpty`: append of
    // non-empties is non-empty, so the transfer is accepted and the
    // manifest carries its preservation row and its postcondition row.
    let m = manifest_of(
        "transfer-nonempty",
        r#"
module Main where
  import Algebra.Semigroup
  import Data.NonEmpty

  infixr 6 <>

  combine :: NonEmpty Char -> NonEmpty Char -> NonEmpty Char
  combine a b = a <> b

  main :: Int
  main = 0
"#,
    );
    let pres = m
        .find("NonEmpty.preserve-<>")
        .unwrap_or_else(|| panic!("the preservation row is in the manifest: {m}"));
    let row = &m[pres.saturating_sub(200)..((pres + 300).min(m.len()))];
    assert!(row.contains("checked ("), "{row}");
    let post = m
        .find("NonEmpty.post-<>")
        .unwrap_or_else(|| panic!("the postcondition row is in the manifest: {m}"));
    let row = &m[post.saturating_sub(200)..((post + 300).min(m.len()))];
    assert!(row.contains("derived:"), "{row}");
}

#[test]
fn generic_preserves_transfer_works_where_the_method_preserves() {
    // The generic `Preserves` transfer at a point whose predicate the
    // method preserves (`Ascii` survives concatenation): accepted.
    expect_checks(
        "generic-ascii",
        &format!(
            r#"
module Main where
  import Algebra.Semigroup

  infixr 6 <>
{GENERIC_TRANSFER}

  combine :: refined (Ascii) (List Char) -> refined (Ascii) (List Char) -> refined (Ascii) (List Char)
  combine a b = a <> b

  main :: Int
  main = 0
"#
        ),
    );
}

#[test]
fn specialized_sorted_merge_wins_by_most_specific_resolution() {
    // At the `Sorted` lattice top both candidates exist (the generic
    // transfer and the specialized `Sorted`/merge instance): the
    // specialized one is selected (merge of sorted lists stays sorted;
    // append would not), law-checked beside its preservation row.
    expect_run(
        "sorted-merge-wins",
        &format!(
            r#"
module Main where
  import Algebra.Semigroup
  import Data.Sorted

  infixr 6 <>
{GENERIC_TRANSFER}

  main :: Int
  main = case to-sorted (Cons 1 (Cons 2 Nil)) of
    None -> 1
    Some a -> case to-sorted (Cons 0 (Cons 3 Nil)) of
      None -> 2
      Some b -> case to-sorted (unrefine (a <> b)) of
        None -> 3
        Some _ -> 0
"#
        ),
        0,
    );
    // The probe that both candidates exist: without the specialized
    // instance the generic candidate serves the point and its `Preserves`
    // obligation rejects (append does not preserve `Ordered xs`).
    let sorted = include_str!("../prelude/Data/Sorted.cinch");
    let sorted_no_instance: String = sorted
        .lines()
        .filter(|l| {
            !l.contains("Semigroup_Sorted") && !l.contains("(<>) p q = merge p q")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let dir = write_project_files(
        "sorted-generic-only",
        &[
            ("Sorted.cinch", &sorted_no_instance),
            (
                "Main.cinch",
                &format!(
                    r#"
module Main where
  import Algebra.Semigroup
  import Data.Sorted

  infixr 6 <>
{GENERIC_TRANSFER}

  combine :: Sorted -> Sorted -> Sorted
  combine a b = a <> b

  main :: Int
  main = 0
"#
                ),
            ),
        ],
    );
    match cinch::driver::driver(Mode::Check, &dir.join("Main.cinch"), None) {
        Ok(_) => panic!(
            "sorted-generic-only: expected the generic candidate to reject, but the program checked"
        ),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("Sorted.preserve-<>") && msg.contains("Ordered xs"),
                "sorted-generic-only: error {msg:?} names the preservation law and the conjunct"
            );
        }
    }
}

#[test]
fn incomparable_heads_at_a_common_refinement_are_a_coherence_error() {
    // The coherence rule (D119 extended): two instances at
    // incomparable heads both match a common refinement, so a use there
    // has no most-specific instance and rejects naming both heads.
    expect_err(
        "coherence-two-heads",
        r#"
module Main where
  signature Combiner a where
    combine :: a -> a -> a

  implicit instance Combiner_Long :: Combiner (refined (length xs > 1) (List Char)) where
    combine a b = a

  implicit instance Combiner_Ascii :: Combiner (refined (Ascii xs) (List Char)) where
    combine a b = b

  use-it :: refined (length xs > 1 && Ascii xs) (List Char) -> refined (length xs > 1 && Ascii xs) (List Char)
  use-it v = combine v v

  main :: Int
  main = 0
"#,
        &["Combiner_Long", "Combiner_Ascii", "incomparable"],
    );
}

#[test]
fn using_obligations_discharge_through_the_dec_evidence_path() {
    // An instance with `using (Sorted ...)` is callable when the caller
    // supplies `Dec` evidence in scope.
    expect_checks(
        "oblig-with-evidence",
        r#"
module Main where
  import Data.Sorted

  signature Combiner a where
    combine :: a -> a -> a

  merge :: List Nat -> List Nat -> List Nat
  merge a b = case a of
    Nil -> b
    Cons x xs -> case b of
      Nil -> a
      Cons y ys -> if x <= y then Cons x (merge xs b) else Cons y (merge a ys)

  implicit instance Combiner_Merge :: using (Sorted xs) => Combiner (List Nat) where
    combine a b = merge a b

  use-it :: List Nat -> List Nat -> List Nat
  use-it a b = case decide (Ordered a) of
    Yes wa -> case decide (Ordered b) of
      Yes wb -> combine a b
      No _ -> a
    No _ -> b

  main :: Int
  main = 0
"#,
    );
    // A caller without the evidence gets the refusal naming the
    // obligation and its `Dec` discharge path.
    expect_err(
        "oblig-without-evidence",
        r#"
module Main where
  import Data.Sorted

  signature Combiner a where
    combine :: a -> a -> a

  merge :: List Nat -> List Nat -> List Nat
  merge a b = case a of
    Nil -> b
    Cons x xs -> case b of
      Nil -> a
      Cons y ys -> if x <= y then Cons x (merge xs b) else Cons y (merge a ys)

  implicit instance Combiner_Merge :: using (Sorted xs) => Combiner (List Nat) where
    combine a b = merge a b

  use-it :: List Nat -> List Nat -> List Nat
  use-it a b = combine a b

  main :: Int
  main = 0
"#,
        &["needs `Dec` evidence", "Ordered a"],
    );
    // Forged evidence (a binder establishing another fact) is refused
    // naming both propositions.
    expect_err(
        "oblig-forged-evidence",
        r#"
module Main where
  import Data.Sorted

  signature Combiner a where
    combine :: a -> a -> a

  merge :: List Nat -> List Nat -> List Nat
  merge a b = case a of
    Nil -> b
    Cons x xs -> case b of
      Nil -> a
      Cons y ys -> if x <= y then Cons x (merge xs b) else Cons y (merge a ys)

  implicit instance Combiner_Merge :: using (Sorted xs) => Combiner (List Nat) where
    combine a b = merge a b

  use-it :: List Nat -> List Nat -> List Nat
  use-it a b = case decide (NotNull a) of
    Yes w -> case decide (Ordered b) of
      Yes wb -> combine a b
      No _ -> a
    No _ -> b

  main :: Int
  main = 0
"#,
        &["is evidence of", "NotNull a", "Ordered a"],
    );
}

#[test]
fn signature_obligations_constrain_callers_identically() {
    // A signature's `using (Sorted xs, Sorted ys)` obligations bind its
    // dependent arguments and constrain callers exactly like the
    // instance form: callable with evidence, refused without it.
    expect_checks(
        "sig-oblig-with-evidence",
        r#"
module Main where
  import Data.Sorted

  merge :: List Nat -> List Nat -> List Nat
  merge a b = case a of
    Nil -> b
    Cons x xs -> case b of
      Nil -> a
      Cons y ys -> if x <= y then Cons x (merge xs b) else Cons y (merge a ys)

  merge2 :: using (Sorted xs, Sorted ys) => (xs :: List Nat) -> (ys :: List Nat) -> List Nat
  merge2 xs ys = merge xs ys

  use-it :: List Nat -> List Nat -> List Nat
  use-it a b = case decide (Ordered a) of
    Yes wa -> case decide (Ordered b) of
      Yes wb -> merge2 a b
      No _ -> a
    No _ -> b

  main :: Int
  main = 0
"#,
    );
    expect_err(
        "sig-oblig-without-evidence",
        r#"
module Main where
  import Data.Sorted

  merge :: List Nat -> List Nat -> List Nat
  merge a b = case a of
    Nil -> b
    Cons x xs -> case b of
      Nil -> a
      Cons y ys -> if x <= y then Cons x (merge xs b) else Cons y (merge a ys)

  merge2 :: using (Sorted xs, Sorted ys) => (xs :: List Nat) -> (ys :: List Nat) -> List Nat
  merge2 xs ys = merge xs ys

  use-it :: List Nat -> List Nat -> List Nat
  use-it a b = merge2 a b

  main :: Int
  main = 0
"#,
        &["needs `Dec` evidence", "Ordered a"],
    );
}

#[test]
fn postcondition_rows_round_trip_through_the_interface() {
    // The manifest's postcondition rows are present and round-tripped
    // through `.cinchif` (the content identity story stays intact: the
    // canonical document re-parses to itself with the same hash).
    let dir = write_project(
        "post-rows",
        r#"
module Main where
  import Algebra.Semigroup
  import Data.NonEmpty

  infixr 6 <>

  combine :: NonEmpty Char -> NonEmpty Char -> NonEmpty Char
  combine a b = a <> b

  main :: Int
  main = 0
"#,
    );
    driver_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir.join("Main.cinch"),
        None,
        &TargetProfile::native(),
    )
    .unwrap_or_else(|e| panic!("post-rows: check run failed: {e}"));
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("post-rows: manifest missing: {e}"));
    assert!(
        manifest.contains("NonEmpty.post-<>") && manifest.contains("derived:"),
        "the manifest carries the postcondition row: {manifest}"
    );
    let text = std::fs::read_to_string(dir.join("build/Data.NonEmpty.cinchif"))
        .unwrap_or_else(|e| panic!("post-rows: interface missing: {e}"));
    assert!(
        text.contains("NonEmpty.post-<>"),
        "the interface carries the postcondition row: {text}"
    );
    let iface = cinch::interface::Iface::from_json(&text)
        .unwrap_or_else(|e| panic!("post-rows: interface does not parse: {e}"));
    let canon = iface.to_json();
    assert_eq!(canon, text, "the interface document is canonical");
    let again = cinch::interface::Iface::from_json(&canon)
        .unwrap_or_else(|e| panic!("post-rows: canonical interface does not re-parse: {e}"));
    assert_eq!(
        iface.iface_hash(),
        again.iface_hash(),
        "the content identity survives the round-trip"
    );
}
