//! Ownership, usage, sharing and region tests (P5): the `%0`/`%1`/
//! `%aff`/`%many` usage discipline, the `Copy` class and `copyOut`,
//! `Ref`/`Mut` borrows with region escape checking, and the thunk
//! multiplicity discipline. See `../design/ownership-and-usage.md`.

use cinch::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!(
            "{name}: expected an error containing {needles:?}, but the program checked"
        ),
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

// ------------------------------------------------------------ multiplicities

#[test]
fn linear_double_use_is_rejected() {
    expect_err(
        "linear-double-use",
        r#"
module Main where
  dup :: Nat %1 -> Nat
  dup x = x + x

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "used 2 time"],
    );
}

#[test]
fn linear_use_once_per_if_branch_is_accepted() {
    expect_ok(
        "linear-per-branch",
        r#"
module Main where
  pick :: Bool -> Nat %1 -> Nat
  pick b x = if b then x else x

  main :: Int
  main = if pick True 3 == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn linear_use_once_per_case_branch_is_accepted() {
    expect_ok(
        "linear-per-case-branch",
        r#"
module Main where
  pick :: Bool -> Nat %1 -> Nat
  pick b x = case b of
    True -> x
    False -> x

  main :: Int
  main = if pick False 3 == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn linear_dropped_on_one_path_is_rejected() {
    expect_err(
        "linear-dropped",
        r#"
module Main where
  dropOne :: Bool -> Nat %1 -> Nat
  dropOne b x = if b then x else 0

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "unused on some path"],
    );
}

#[test]
fn erased_use_is_rejected_and_non_use_is_accepted() {
    expect_err(
        "erased-used",
        r#"
module Main where
  erase :: Nat %0 -> Nat
  erase x = x

  main :: Int
  main = 0
"#,
        &["multiplicity %0", "used 1 time"],
    );
    expect_ok(
        "erased-unused",
        r#"
module Main where
  erase :: Nat %0 -> Nat
  erase x = 7

  main :: Int
  main = if erase 1 == 7 then 0 else 1
"#,
        0,
    );
}

#[test]
fn affine_double_use_is_rejected() {
    expect_err(
        "affine-double-use",
        r#"
module Main where
  twice :: Nat %aff -> Nat
  twice x = x + x

  main :: Int
  main = 0
"#,
        &["multiplicity %aff", "used 2 time"],
    );
    expect_ok(
        "affine-drop-or-use",
        r#"
module Main where
  atMost :: Bool -> Nat %aff -> Nat
  atMost b x = if b then x else 0

  main :: Int
  main = if atMost True 3 == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn unrestricted_use_is_free() {
    expect_ok(
        "unrestricted",
        r#"
module Main where
  twice :: Nat %many -> Nat
  twice x = x + x

  main :: Int
  main = if twice 3 == 6 then 0 else 1
"#,
        0,
    );
}

#[test]
fn unannotated_binders_default_to_unrestricted() {
    // Unannotated parameters are unrestricted in this slice (decision
    // D15): previously checked programs keep their meaning.
    expect_ok(
        "default-many",
        r#"
module Main where
  twice :: Nat -> Nat
  twice x = x + x

  applyTwice :: (Nat -> Nat) -> Nat -> Nat
  applyTwice f x = f (f x)

  main :: Int
  main = if applyTwice twice 2 == 8 then 0 else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ the Copy class

#[test]
fn copy_class_from_the_vector_filter_shape_works() {
    // The shape of examples/source/02-vector-filter.cinch with plain types:
    // borrow matching yields references, `copyOut` copies out of a borrow
    // with `Copy` evidence, and the recursive call reuses the predicate
    // and the tail borrow.
    expect_ok(
        "filter-shape",
        r#"
module Main where
  data List2 a where
    Nil2 :: List2 a
    Cons2 :: a -> List2 a -> List2 a

  filter2 :: forall a region. using (copy :: Copy a) =>
             (a -> Bool) -> Ref region (List2 a) -> List2 a
  filter2 predicate xs = case xs of
    Nil2 -> Nil2
    Cons2 x rest ->
      let keep = predicate (copyOut x)
      in if keep then Cons2 (copyOut x) (filter2 predicate rest)
         else filter2 predicate rest

  main :: Int
  main = case filter2 (\x -> x == 2) (borrow (Cons2 1 (Cons2 2 (Cons2 2 Nil2)))) of
    Cons2 a (Cons2 b Nil2) -> if a == 2 && b == 2 then 0 else 1
    Nil2 -> 2
    Cons2 a Nil2 -> 3
    Cons2 a (Cons2 b (Cons2 c rest)) -> 4
"#,
        0,
    );
}

#[test]
fn copy_class_is_structural_and_not_user_extensible() {
    expect_err(
        "copy-no-evidence",
        r#"
module Main where
  cacheId :: forall a. a -> a
  cacheId x = let ~y = x in y

  main :: Int
  main = 0
"#,
        &["no Copy evidence"],
    );
    expect_err(
        "copy-class-cannot-be-declared",
        r#"
module Main where
  signature Copy a where
    copy :: a -> a

  main :: Int
  main = 0
"#,
        &["compiler-known class"],
    );
    expect_err(
        "copy-class-has-no-instances",
        r#"
module Main where
  implicit instance Copy_Nat :: Copy Nat where
    copy x = x

  main :: Int
  main = 0
"#,
        &["compiler-known class"],
    );
}

#[test]
fn copy_out_coheres_with_multiplicity() {
    // Copying duplicates a value; a linear value must not be duplicated
    // (ownership and usage design, sections 2 and 3).
    expect_err(
        "copy-out-linear",
        r#"
module Main where
  steal :: Nat %1 -> Nat
  steal x = copyOut x

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "copyOut"],
    );
}

// ------------------------------------------------------------ regions and borrows

#[test]
fn scoped_borrow_is_accepted() {
    // A borrow introduced by `borrow` and consumed by `copyOut` in the
    // region's scope.
    expect_ok(
        "scoped-borrow",
        r#"
module Main where
  peek :: forall region. Ref region Nat -> Nat
  peek b = copyOut b + 1

  main :: Int
  main = if peek (borrow 41) == 42 then 0 else 1
"#,
        0,
    );
    // A memo demand's borrow is copied out inside the group's scope.
    expect_ok(
        "scoped-demand",
        r#"
module Main where
  main :: Int
  main = if copyOut (demand y) == 21 then 0 else 1
    where ~y = 20 + 1
"#,
        0,
    );
}

#[test]
fn borrow_escape_is_rejected() {
    expect_err(
        "demand-escape",
        r#"
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in demand y)
"#,
        &["region", "escape"],
    );
    expect_err(
        "cell-capture-escape",
        r#"
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in delay (\u -> copyOut (demand y)))
"#,
        &["region", "escape"],
    );
}

#[test]
fn borrow_matching_yields_field_borrows() {
    // The core notation table's `first`: matching a borrowed value binds
    // references to the fields, and the result may be a borrow of the
    // same region (core specification, section 1).
    expect_ok(
        "first-borrow",
        r#"
module Main where
  data List2 a where
    Nil2 :: List2 a
    Cons2 :: a -> List2 a -> List2 a

  firstOr :: forall a region. Ref region a -> Ref region (List2 a) -> Ref region a
  firstOr d xs = case xs of
    Nil2 -> d
    Cons2 x rest -> x

  main :: Int
  main = if copyOut (firstOr (borrow 0) (borrow (Cons2 5 Nil2))) == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn exclusive_borrows_are_lexically_unique() {
    expect_err(
        "two-mut-borrows",
        r#"
module Main where
  use2 :: forall region. Mut region Nat -> Mut region Nat -> Nat
  use2 a b = copyOut a

  main :: Int
  main = 0
"#,
        &["region", "exclusive"],
    );
    expect_err(
        "ref-with-mut-borrow",
        r#"
module Main where
  use2 :: forall region. Mut region Nat -> Ref region Nat -> Nat
  use2 a b = copyOut b

  main :: Int
  main = 0
"#,
        &["region", "exclusive"],
    );
}

// ------------------------------------------------------------ records and places

#[test]
fn linear_record_disjoint_projections_split_once() {
    // The shape of examples/source/06-counter-upgrade.cinch: `c.base` and
    // `c.delta` are disjoint places of the record, one split of a `%1`
    // value (ownership and usage design, section 2).
    expect_ok(
        "linear-record",
        r#"
module Main where
  record Counter where
    base :: Nat
    delta :: Nat

  increment :: Counter %1 -> Counter
  increment c = Counter { base = c.base, delta = c.delta + 1 }

  main :: Int
  main = let r = increment (Counter { base = 1, delta = 2 }) in
           if r.base == 1 && r.delta == 3 then 0 else 1
"#,
        0,
    );
    expect_err(
        "linear-record-repeated-place",
        r#"
module Main where
  record Counter where
    base :: Nat
    delta :: Nat

  twice :: Counter %1 -> Nat
  twice c = c.base + c.base

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "used 2 time"],
    );
}

// ------------------------------------------------------------ thunk discipline

#[test]
fn linear_thunk_is_forced_exactly_once() {
    expect_err(
        "linear-thunk-twice",
        r#"
module Main where
  forceTwice :: Thunk {} Nat %1 -> Nat
  forceTwice t = force t + force t

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "used 2 time"],
    );
    expect_err(
        "linear-thunk-never",
        r#"
module Main where
  forceNever :: Thunk {} Nat %1 -> Nat
  forceNever t = 0

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "never used"],
    );
    expect_ok(
        "linear-thunk-once",
        r#"
module Main where
  forceOnce :: Thunk {} Nat %1 -> Nat
  forceOnce t = force t

  main :: Int
  main = if forceOnce (delay (\u -> 7)) == 7 then 0 else 1
"#,
        0,
    );
}

#[test]
fn affine_thunk_is_forced_at_most_once() {
    expect_err(
        "affine-thunk-twice",
        r#"
module Main where
  forceTwice :: Thunk {} Nat %aff -> Nat
  forceTwice t = force t + force t

  main :: Int
  main = 0
"#,
        &["multiplicity %aff", "used 2 time"],
    );
    expect_ok(
        "affine-thunk-drop-or-force",
        r#"
module Main where
  maybeForce :: Bool -> Thunk {} Nat %aff -> Nat
  maybeForce b t = if b then force t else 0

  main :: Int
  main = if maybeForce False (delay (\u -> 7)) == 0 then 0 else 1
"#,
        0,
    );
}

#[test]
fn unannotated_thunk_defaults_to_affine() {
    // A thunk is used at `%1` or `%aff` by default (laziness and
    // reclamation specification, section 3); the default is `%aff`.
    expect_err(
        "default-thunk-twice",
        r#"
module Main where
  forceTwice :: Thunk {} Nat -> Nat
  forceTwice t = force t + force t

  main :: Int
  main = 0
"#,
        &["multiplicity %aff", "used 2 time"],
    );
}

#[test]
fn linear_thunk_forced_once_per_branch_is_accepted() {
    expect_ok(
        "thunk-per-branch",
        r#"
module Main where
  forceBranch :: Bool -> Thunk {} Nat %1 -> Nat
  forceBranch b t = if b then force t else force t

  main :: Int
  main = if forceBranch True (delay (\u -> 5)) == 5 then 0 else 1
"#,
        0,
    );
}
