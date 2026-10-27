//! Class `law` declarations as citable `justify` axioms (primitive
//! operations and inline target IR specification section 5): the
//! shipped classes state their equations, instances are checked against
//! them sample-wise where the machinery can decide it, a law-breaking
//! instance is rejected naming the law, and a `rewrite` cites a law by
//! identity (`Semigroup.assoc`), with the undischarged law identity
//! recorded in the artifact manifest's declared assumptions.

use safelang::check::StageMode;
use safelang::core::TargetProfile;
use safelang::driver::{check_sources, driver_with_profile};
use std::path::PathBuf;

/// The law-abiding instance fixture: a concrete carrier whose `<>`
/// obeys `Semigroup.assoc`.
const LAWS: &str = include_str!("fixtures/prim/laws.lang");

/// Assert that checking the program fails with a message containing
/// `needle`.
fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, name) {
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

/// Write one source to a fresh temporary project and return its
/// directory.
fn write_project(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("safelang-laws-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project");
    std::fs::write(dir.join("Main.lang"), src).expect("write Main.lang");
    dir
}

/// The artifact manifest of one checked source.
fn manifest_of(name: &str, src: &str) -> String {
    let dir = write_project(name, src);
    driver_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir.join("Main.lang"),
        None,
        &TargetProfile::native(),
    )
    .unwrap_or_else(|e| panic!("{name}: check run failed: {e}"));
    std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("{name}: manifest missing: {e}"))
}

use safelang::driver::Mode;

// ------------------------------------- the shipped classes' equations

#[test]
fn class_laws_state_the_equations() {
    // Each shipped law cites by identity and justifies the rule that
    // states its own equation: `Semigroup.assoc` proves association,
    // `Monoid.left-unit` proves the left unit, `Functor.map-identity`
    // proves the identity law (the equations of the P25-approved law
    // layer).
    let src = r#"
module LawCite where
  import Algebra.Semigroup
  import Algebra.Monoid
  import Category.Functor
  import Function.Combinators

  infixr 6 <>

  rewrite assocRule forall (x :: U32) (y :: U32) (z :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> z)
    replace (x <> (y <> z))
    justify Semigroup.assoc x y z

  rewrite leftUnitRule forall (x :: U32). using (sg :: Semigroup U32, mo :: Monoid U32) where
    match (mempty <> x)
    replace x
    justify Monoid.left-unit x

  rewrite mapIdentityRule forall (x :: List U32). using (fu :: Functor List) where
    match (map identity x)
    replace x
    justify Functor.map-identity x
"#;
    check_sources(src, "cite").expect("every shipped law proves its own equation");

    // A rule claiming a different equation is not proved by the law.
    expect_err(
        "cite-fail",
        r#"
module LawCiteBad where
  import Algebra.Semigroup

  infixr 6 <>

  rewrite wrongRule forall (x :: U32) (y :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> y)
    replace (x <> y)
    justify Semigroup.assoc x y y
"#,
        "justification does not prove",
    );
}

// --------------------------------------------- sample-wise checking

#[test]
fn law_abiding_instance_passes_sample_wise() {
    // The fixture's `Semigroup Box` instance obeys `Semigroup.assoc`:
    // the sample-wise check runs over the carrier's samples and the
    // manifest reports what it decided.
    let m = manifest_of("law-pass", LAWS);
    let i = m
        .find("LawLib.Semigroup_Box")
        .unwrap_or_else(|| panic!("the instance's law row is in the manifest: {m}"));
    let row = &m[i..(i + 240).min(m.len())];
    assert!(
        row.contains("Semigroup") && row.contains("assoc") && row.contains("checked ("),
        "{row}"
    );
}

#[test]
fn law_breaking_instance_is_rejected_naming_the_law() {
    // Subtraction is not associative: the sample-wise check finds a
    // counterexample and rejects the instance, naming the law.
    expect_err(
        "law-break",
        r#"
module LawBreak where
  import Algebra.Semigroup

  data Box where
    Box :: U32 -> Box

  implicit instance Semigroup_Box :: Semigroup Box where
    (<>) x y = case x of
      Box a -> case y of
        Box b -> Box (wrapping-sub-u32 a b)

  main :: Int
  main = 0
"#,
        "Semigroup.assoc",
    );
}

// ------------------------------------- citation and assumption tiers

#[test]
fn rewrite_citing_a_law_by_identity_records_the_assumption() {
    // A `rewrite` justified by a law method is the assumed tier (the law
    // is undischarged evidence): its identity is the assumption,
    // recorded in the artifact manifest's declared assumptions
    // (decision D55's rule).
    let src = r#"
module LawAssume where
  import Algebra.Semigroup

  infixr 6 <>

  rewrite assocRule forall (x :: U32) (y :: U32) (z :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> z)
    replace (x <> (y <> z))
    justify Semigroup.assoc x y z

  main :: Int
  main = 0
"#;
    check_sources(src, "assume").expect("the law citation justifies the rule");
    let m = manifest_of("law-assume", src);
    let declared = {
        let s = m.find("\"declared\": [").expect("declared assumptions");
        let e = m[s..].find(']').map(|i| s + i).expect("end");
        &m[s..e]
    };
    assert!(
        declared.contains("Algebra.Semigroup.assoc"),
        "{declared}"
    );
}
