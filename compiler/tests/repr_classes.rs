//! Representation classes and the option C collector
//! (`design/layouts-and-specialization.md` section 2): real programs are
//! checked, optimized and collected the way the driver does it, then run,
//! and the collector's version set is read from its report.

use cinch::core::TargetProfile;
use cinch::driver::{check_sources, run_source};

/// The representation copies the collector emitted for `src`.
fn versions(name: &str, src: &str) -> Vec<String> {
    let src = src.to_string();
    let name = name.to_string();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            let program =
                check_sources(&src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
            let (_, report) = cinch::specialize::optimize(&program, &TargetProfile::native())
                .unwrap_or_else(|e| panic!("{name}: optimize failed: {e}"));
            report.versions
        })
        .expect("spawn")
        .join()
        .expect("collector thread")
}

fn run(name: &str, src: &str) -> i32 {
    let src = src.to_string();
    let name = name.to_string();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || run_source(&src).unwrap_or_else(|e| panic!("{name}: compile failed: {e}")))
        .expect("spawn")
        .join()
        .expect("run thread")
}

/// The versions of one unit (its `unit@letters` names).
fn of(all: &[String], unit: &str) -> Vec<String> {
    let prefix = format!("{unit}@");
    all.iter().filter(|v| v.starts_with(&prefix)).cloned().collect()
}

/// `pick` recurses on its counter, so driving leaves real calls to it
/// (a fully unfolded call would reach no copy at all).
const PICK: &str = r#"
module Main where
  pick :: forall a. Nat -> a -> a -> a
  pick n x y = if n == 0 then x else pick (n - 1) y x

  atI64 :: I64
  atI64 = pick 40 7 9

  atNat :: Nat
  atNat = pick 41 7 9

  atStr :: String
  atStr = pick 40 "seven" "nine"

  unused :: forall a. a -> a -> a
  unused x y = y

  main :: Int
  main = if atI64 == 7 then (if atNat == 9 then (if atStr == "seven" then 0 else 3) else 2) else 1
"#;

#[test]
fn one_function_at_three_representations() {
    assert_eq!(run("pick", PICK), 0);
    let all = versions("pick", PICK);
    // `I64` is a raw scalar, `String` a counted block, `Nat` the tagged
    // immediate-or-block class: exactly these three vectors are reached.
    assert_eq!(
        of(&all, "Main.pick"),
        vec!["Main.pick@O".to_string(), "Main.pick@R".to_string(), "Main.pick@T".to_string()],
        "{all:?}"
    );
}

#[test]
fn an_unused_polymorphic_function_has_no_copy() {
    let all = versions("pick", PICK);
    assert!(of(&all, "Main.unused").is_empty(), "{all:?}");
}

/// A nested datatype: `Deep` holds a `Nest` of pairs, so `depth` and
/// `build` recurse at `Pair a` (polymorphic recursion). `Pair a` is a
/// constructor application, so its class is `Obj` whatever `a` is, and
/// the recursion reaches `[Obj]` and stops there.
const NEST: &str = r#"
module Main where
  data Pair a where
    Pair :: a -> a -> Pair a

  data Nest a where
    Flat :: a -> Nest a
    Deep :: Nest (Pair a) -> Nest a

  depth :: forall a. Nest a -> Nat
  depth t = case t of
    Flat x -> 0
    Deep inner -> 1 + depth inner

  build :: forall a. Nat -> a -> Nest a
  build k x = if k == 0 then Flat x else Deep (build (k - 1) (Pair x x))

  seed :: I64
  seed = 3

  main :: Int
  main = if depth (build 12 seed) == 12 then 0 else 1
"#;

#[test]
fn polymorphic_recursion_reaches_a_finite_version_set() {
    let start = std::time::Instant::now();
    let all = versions("nest", NEST);
    assert!(start.elapsed().as_secs() < 120, "the collector took {:?}", start.elapsed());
    // Called at `I64` from `main` (driving may unfold that outer call),
    // then at `Pair a`, always `Obj`, from itself: at most `[Raw]` and
    // `[Obj]`, however deep the recursion runs, and `[Obj]` is reached.
    for unit in ["Main.build", "Main.depth"] {
        let got = of(&all, unit);
        assert!(got.contains(&format!("{unit}@O")), "{all:?}");
        assert!(
            got.iter().all(|v| *v == format!("{unit}@O") || *v == format!("{unit}@R")),
            "{all:?}"
        );
    }
    assert_eq!(run("nest", NEST), 0);
}
