//! Default method implementations in a `signature` block.
//!
//! An equation beneath a method's signature is that method's default. An
//! instance that omits the method takes the default, checked at the
//! instance's types; an instance that defines it overrides it. A default
//! resolves names in the class's module (it is the class author's code),
//! is type-checked once at the class even before any instance takes it,
//! and is part of the class's interface contract, because an instance
//! elsewhere inlines it.

mod common;

use cinch::driver::Mode;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

fn project(files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "cinch_class_defaults_{}_{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

fn outcome(mode: Mode, dir: &Path) -> Result<i32, String> {
    common::drive(mode, dir, "Main.cinch", None)
}

/// The class module. `double-of` is private: a default may use it, an
/// instance in another module cannot name it.
const SHAPES: &str = "module Shapes where
  import Data.Nat
  import Algebra.Semigroup

  export Describe, size, twice, Combine, unit, combine-all

  double-of :: Nat -> Nat
  double-of n = n + n

  signature Describe a where
    size :: a -> Nat
    twice :: a -> Nat
    twice x = double-of (size x)

  signature Combine a where
    unit :: a
    combine-all :: using (s :: Semigroup a) => List a -> a
    combine-all xs = case xs of
      Nil -> unit
      Cons y ys -> y <> combine-all ys
";

const MAIN: &str = "module Main where
  import Shapes
  import Algebra.Semigroup
  import Data.Int
  import Data.Nat
  import Data.Machine

  data Box where
    Box :: Nat -> Box

  data Pair where
    Pair :: Nat -> Nat -> Pair

  implicit instance Describe_Box :: Describe Box where
    size b = case b of
      Box n -> n

  implicit instance Describe_Pair :: Describe Pair where
    size p = case p of
      Pair a b -> a + b
    twice p = 7

  implicit instance Semigroup_Box :: Semigroup Box where
    (<>) x y = case x of
      Box a -> case y of
        Box b -> Box (a + b)

  implicit instance Combine_Box :: Combine Box where
    unit = Box 0

  unbox :: Box -> Nat
  unbox b = case b of
    Box n -> n

  check :: Bool -> Nat -> Nat
  check ok bit = if ok then 0 else bit

  main :: Int
  main =
    let a = check (twice (Box 5) == 10) 1 in
    let b = check (twice (Pair 2 3) == 7) 2 in
    let c = check (unbox (combine-all (Cons (Box 1) (Cons (Box 2) (Cons (Box 3) Nil)))) == 6) 4 in
    let d = check (unbox (combine-all (Nil :: List Box)) == 0) 8 in
    nat-to-int (a + b + c + d)
";

#[test]
fn an_omitted_method_takes_its_default_and_a_definition_overrides_it() {
    // Bits: 1 the default (through the class module's private helper),
    // 2 the override, 4 a default using its method's own constraint,
    // 8 that default's empty case.
    let dir = project(&[("Shapes.cinch", SHAPES), ("Main.cinch", MAIN)]);
    assert_eq!(outcome(Mode::Run, &dir), Ok(0));
}

#[test]
fn an_omitted_method_without_a_default_is_still_refused() {
    let main = MAIN.replace(
        "    size b = case b of\n      Box n -> n\n",
        "    twice b = 3\n",
    );
    let dir = project(&[("Shapes.cinch", SHAPES), ("Main.cinch", &main)]);
    let err = outcome(Mode::Check, &dir).expect_err("size has no default");
    assert!(err.contains("instance Describe_Box does not define method size"), "{err}");
}

#[test]
fn a_default_for_an_undeclared_method_is_refused_at_the_class() {
    let shapes = SHAPES.replace(
        "    twice x = double-of (size x)\n",
        "    twice x = double-of (size x)\n    thrice x = 3\n",
    );
    let dir = project(&[("Shapes.cinch", &shapes), ("Main.cinch", MAIN)]);
    let err = outcome(Mode::Check, &dir).expect_err("thrice is not a method");
    assert!(err.contains("`thrice` is not a method of Describe"), "{err}");
    assert!(err.contains("Shapes.cinch"), "the refusal names the class's file: {err}");
}

#[test]
fn an_ill_typed_default_is_reported_at_its_class_with_no_instance() {
    // No instance anywhere takes the broken default: the class's own check
    // must still find it.
    let shapes = SHAPES.replace("twice x = double-of (size x)", "twice x = double-of x");
    let main = "module Main where\n  import Shapes\n  main :: Int\n  main = 0\n";
    let dir = project(&[("Shapes.cinch", &shapes), ("Main.cinch", main)]);
    let err = outcome(Mode::Check, &dir).expect_err("the default is ill-typed");
    assert!(err.contains("type mismatch"), "{err}");
    assert!(err.contains("Shapes.cinch"), "{err}");
}

/// The default identities `Shapes`'s interface records, by method.
fn default_ids(dir: &Path) -> Vec<(String, Option<String>)> {
    outcome(Mode::Build, dir).expect("the project builds");
    let text = std::fs::read_to_string(dir.join("build").join("Shapes.cinchif")).expect("interface");
    let iface = cinch::interface::Iface::from_json(&text).expect("the interface parses");
    let mut out = Vec::new();
    for s in &iface.exports.signatures {
        for m in &s.methods {
            out.push((format!("{}.{}", s.name, m.name), m.default.as_ref().map(|body| body.to_json())));
        }
    }
    out.sort();
    out
}

#[test]
fn a_changed_default_changes_the_class_contract_and_only_that_method() {
    let dir = project(&[("Shapes.cinch", SHAPES), ("Main.cinch", MAIN)]);
    let before = default_ids(&dir);
    let get = |ids: &[(String, Option<String>)], k: &str| {
        ids.iter().find(|(n, _)| n == k).map(|(_, d)| d.clone()).expect(k)
    };
    assert!(get(&before, "Describe.twice").is_some(), "{before:?}");
    assert!(get(&before, "Combine.combine-all").is_some(), "{before:?}");
    assert_eq!(get(&before, "Describe.size"), None, "a method without a default records none");

    std::fs::write(
        dir.join("Shapes.cinch"),
        SHAPES.replace("twice x = double-of (size x)", "twice x = double-of (size x) + 0"),
    )
    .unwrap();
    let after = default_ids(&dir);
    assert_ne!(get(&before, "Describe.twice"), get(&after, "Describe.twice"));
    assert_eq!(get(&before, "Combine.combine-all"), get(&after, "Combine.combine-all"));
}

/// The prelude's own `mconcat` default taken where it is hardest: at a
/// function carrier (the instance's method type has one more arrow than
/// the default binds, so the taken copy is eta-expanded) and at a
/// refinement instance (checked at `NonZero`, whose methods the default
/// composes, rather than at the container `Int`, which has no `Monoid`).
const CARRIERS: &str = r#"module Main where
  import Algebra.Semigroup
  import Algebra.Monoid
  import Data.NonZero
  import Data.String
  import Data.Option

  greet :: String -> String
  greet s = string-concat "hi " s

  bang :: String -> String
  bang s = string-concat s "!"

  nzs :: List Int -> List (NonZero Int)
  nzs xs = case xs of
    Nil -> Nil
    Cons y ys -> case non-zero y of
      None -> nzs ys
      Some v -> Cons v (nzs ys)

  value :: NonZero Int -> Int
  value v = unrefine v

  main :: Int
  main =
    let f = mconcat (Cons greet (Cons bang (Cons greet Nil))) in
    let a = if f "x" == "hi xx!hi x" then 0 else 1 in
    let b = if value (mconcat (nzs (Cons 2 (Cons 3 (Cons 7 Nil))))) == 42 then 0 else 2 in
    let c = if value (mconcat (Nil :: List (NonZero Int))) == 1 then 0 else 4 in
    a + b + c
"#;

#[test]
fn the_mconcat_default_works_at_a_function_carrier_and_a_refinement() {
    // Bits: 1 pointwise over functions, 2 the NonZero product, 4 its unit.
    let dir = project(&[("Main.cinch", CARRIERS)]);
    assert_eq!(outcome(Mode::Run, &dir), Ok(0));
}

#[test]
fn annotations_keep_definition_and_default_parameter_identities() {
    let class = "module Defaults where\n  signature Identity a where\n    seed :: a -> a\n    identity :: a -> a\n    identity x = (x :: a)\n";
    let instance = "module Main where\n  import Defaults\n  implicit instance IdentityInt :: Identity Int where\n    seed x = x\n  main :: Int\n  main = identity 42\n";
    for (name, source, accepted) in [
        ("class parameter in inherited default", format!("{class}{instance}"), true),
        ("term and type names remain separate", format!("{}{instance}", class.replace("identity x = (x :: a)", "identity a = (a :: a)")), true),
        ("unused default still checks abstract carrier", format!("{}module Main where\n  import Defaults\n  main :: Int\n  main = 0\n", class.replace("identity x = (x :: a)", "identity x = (x :: Int)")), false),
        ("override cannot hide invalid generic default", format!("{}{}", class.replace("identity x = (x :: a)", "identity x = (True :: a)"), instance.replace("    seed x = x\n", "    seed x = x\n    identity x = x\n")), false),
        ("method parameter in inherited default", "module Defaults where\n  signature Pick a where\n    seed :: a -> a\n    pick :: forall b. b -> a -> b\n    pick x ignored = (x :: b)\nmodule Main where\n  import Defaults\n  implicit instance PickInt :: Pick Int where\n    seed x = x\n  main :: Int\n  main = pick 42 0\n".into(), true),
        ("generic body annotation", "module Main where\n  identity :: forall a. a -> a\n  identity x = (x :: a)\n  main :: Int\n  main = identity 42\n".into(), true),
        ("different rigid parameters do not unify", "module Main where\n  bad :: forall a b. a -> b -> a\n  bad x ignored = (x :: b)\n  main :: Int\n  main = 0\n".into(), false),
        ("unknown type is not inferred", "module Main where\n  bad :: forall a. a -> a\n  bad x = (x :: unknown)\n  main :: Int\n  main = 0\n".into(), false),
        ("type scope ends at its definition", "module Main where\n  identity :: forall a. a -> a\n  identity x = (x :: a)\n  bad :: Int -> Int\n  bad x = (x :: a)\n  main :: Int\n  main = 0\n".into(), false),
        ("nested forall shadows the outer binder", "module Main where\n  identity :: forall a. a -> a\n  identity x = let inner = ((\\value -> (value :: a)) :: forall a. a -> a) in inner x\n  main :: Int\n  main = identity 42\n".into(), true),
        ("unused class parameter cannot be concretized", "module Main where\n  signature Bad a where\n    bad :: a -> a\n    bad x = (1 :: a)\n  main :: Int\n  main = 0\n".into(), false),
    ] {
        let result = common::source_frontend::check_sources(&source, "default-annotation-scope.cinch");
        assert_eq!(result.is_ok(), accepted, "{name}: {result:?}\n{source}");
    }
    let dir = project(&[("Defaults.cinch", class), ("Main.cinch", &instance[instance.find("module Main where").unwrap()..])]);
    assert_eq!(outcome(Mode::Run, &dir), Ok(42), "the inherited annotated default must execute");
    std::fs::remove_dir_all(dir).unwrap();
}
