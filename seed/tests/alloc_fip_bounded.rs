//! Compile-time safety assertion tests for `#[alloc("bounded(n)")]` and
//! `#[fip]` (fully in-place): positive execution/check cases, negative
//! compile-fail diagnostics, `.cinchif` interface contract facts, and
//! `build/<Entry>.manifest.json` safety summaries.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;
use common::source_frontend::{check_source_interfaces, check_sources, run_source};
use std::path::PathBuf;

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation/run failed: {e}"),
    }
}

fn expect_checks(name: &str, src: &str) {
    if let Err(e) = check_sources(src, "Main.cinch") {
        panic!("{name}: expected program to check, got error: {e}");
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "Main.cinch") {
        Ok(_) => panic!("{name}: expected error containing {needles:?}, but program checked"),
        Err(e) => {
            for n in needles {
                assert!(e.contains(n), "{name}: error {e:?} does not contain {n:?}");
            }
        }
    }
}

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("cinch_alloc_fip_{}_{}", std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

// =========================================================================
// `#[alloc("bounded(n)")]`
// =========================================================================

#[test]
fn alloc_bounded_accepts_zero_sequential_branch_max_and_callee_composition() {
    let src = r#"
module Main where
  #[repr(value)]
  record Vec2 where
    x :: I64
    y :: I64

  #[no-alloc]
  bump :: Int -> Int
  bump x = x + 1

  #[alloc("bounded(0)")]
  zeroAlloc :: Int -> Int
  zeroAlloc x = bump x + x

  #[alloc("bounded(0)")]
  makeVec :: I64 -> I64 -> Vec2
  makeVec a b = Vec2 { x = a, y = b }

  #[alloc("once")]
  wrapOne :: Int -> List Int -> List Int
  wrapOne x xs = Cons x xs

  #[alloc("bounded(1)")]
  singleton :: Int -> List Int
  singleton x = Cons x Nil

  -- Branching takes worst-case `max(2, 1) = 2`.
  #[alloc("bounded(2)")]
  branchMax :: Bool -> Int -> List Int
  branchMax flag x =
    if flag then Cons x (Cons (bump x) Nil)
    else Cons x Nil

  -- `case` branching takes `max(0, 2) = 2`.
  #[alloc("bounded(2)")]
  caseMax :: List Int -> List Int
  caseMax xs = case xs of
    Nil -> Nil
    Cons x _ -> Cons x (Cons (x + 1) Nil)

  -- Sequential composition + callee bounds: `wrapOne` (1) + `singleton` (1) + tuple (1) = 3.
  #[alloc("bounded(3)")]
  composeBounds :: Bool -> Int -> (List Int, List Int)
  composeBounds flag x =
    let a = wrapOne x Nil in
    let b = singleton x in
    (a, b)

  main :: Int
  main =
    let v = makeVec 3 4 in
    let _ = branchMax True (zeroAlloc 1) in
    let _ = caseMax (singleton 5) in
    let _ = composeBounds False 7 in
    if v.x == 3 && v.y == 4 then 0 else 1
"#;
    expect_ok("alloc-bounded-ok", src, 0);
}

#[test]
fn alloc_bounded_rejects_exceeding_bound_sequential_and_branch() {
    expect_err(
        "alloc-bounded-seq-exceed",
        r#"
module Main where
  #[alloc("bounded(2)")]
  three :: Int -> List Int
  three x = Cons x (Cons x (Cons x Nil))

  main :: Int
  main = 0
"#,
        &["bounded(2)", "3", "above the bound of 2"],
    );

    expect_err(
        "alloc-bounded-branch-exceed",
        r#"
module Main where
  #[alloc("bounded(1)")]
  branchTooBig :: Bool -> Int -> List Int
  branchTooBig b x =
    if b then Cons x (Cons x Nil)
    else Cons x Nil

  main :: Int
  main = 0
"#,
        &["bounded(1)", "2", "above the bound of 1"],
    );

    expect_err(
        "alloc-bounded-callee-sum-exceed",
        r#"
module Main where
  #[alloc("bounded(2)")]
  two :: Int -> List Int
  two x = Cons x (Cons x Nil)

  #[alloc("bounded(2)")]
  threeViaCallee :: Int -> List Int
  threeViaCallee x = Cons x (two x)

  main :: Int
  main = 0
"#,
        &["bounded(2)", "3", "above the bound of 2"],
    );
}

#[test]
fn alloc_bounded_forbids_unbounded_recursion_allocating_closures_and_unannotated_callees() {
    expect_err(
        "alloc-bounded-unbounded-recursion",
        r#"
module Main where
  #[alloc("bounded(4)")]
  repeat :: Int -> List Int
  repeat n = if n <= 0 then Nil else Cons n (repeat (n - 1))

  main :: Int
  main = 0
"#,
        &["bounded(4)", "unbounded recursion"],
    );

    expect_err(
        "alloc-bounded-allocating-closure",
        r#"
module Main where
  #[alloc("bounded(4)")]
  mkClo :: Int -> Int
  mkClo x =
    let f = \y -> Cons y Nil in
    0

  main :: Int
  main = 0
"#,
        &["bounded(4)", "allocating closure"],
    );

    expect_err(
        "alloc-bounded-unannotated-callee",
        r#"
module Main where
  helper :: Int -> List Int
  helper x = Cons x Nil

  #[alloc("bounded(2)")]
  caller :: Int -> List Int
  caller x = helper x

  main :: Int
  main = 0
"#,
        &["bounded(2)", "helper"],
    );

    expect_err(
        "alloc-bounded-bad-arg",
        r#"
module Main where
  #[alloc("bounded(-1)")]
  bad :: Int -> Int
  bad x = x

  main :: Int
  main = 0
"#,
        &["not an allocation bound"],
    );
}

// =========================================================================
// `#[fip]` (fully in-place)
// =========================================================================

#[test]
fn fip_accepts_in_place_list_tree_pair_value_niche_and_fip_no_alloc_callees() {
    let src = r#"
module Main where
  data Pair where
    MkPair :: Int -> Int -> Pair

  data Tree where
    Leaf :: Int -> Tree
    Node :: Tree -> Int -> Tree -> Tree

  #[repr(value)]
  record Point where
    px :: I64
    py :: I64

  #[repr(niche)]
  data SmallOpt where
    SNone :: SmallOpt
    SSome :: U8 -> SmallOpt

  #[no-alloc]
  inc :: Int -> Int
  inc x = x + 1

  #[fip]
  swapPair :: Pair -> Pair
  swapPair (MkPair a b) = MkPair b a

  #[fip]
  incList :: List Int -> List Int
  incList xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (inc x) (incList rest)

  #[fip]
  revAcc :: List Int -> List Int -> List Int
  revAcc xs acc = case xs of
    Nil -> acc
    Cons x rest -> revAcc rest (Cons x acc)

  #[fip]
  mirror :: Tree -> Tree
  mirror t = case t of
    Leaf x -> Leaf (inc x)
    Node l x r -> Node (mirror r) x (mirror l)

  #[fip]
  addPoints :: Point -> Point -> Point
  addPoints a b = Point { px = wrapping-add-i64 a.px b.px, py = wrapping-add-i64 a.py b.py }

  #[fip]
  wrapSmall :: U8 -> SmallOpt
  wrapSmall x = SSome x

  #[fip]
  doubleIncList :: List Int -> List Int
  doubleIncList xs = incList (incList xs)

  sumList :: List Int -> Int
  sumList xs = case xs of
    Nil -> 0
    Cons x rest -> x + sumList rest

  treeSum :: Tree -> Int
  treeSum t = case t of
    Leaf x -> x
    Node l x r -> treeSum l + x + treeSum r

  main :: Int
  main =
    let p = swapPair (MkPair 10 20) in
    let pOk = case p of MkPair a b -> a == 20 && b == 10 in
    let xs = revAcc (doubleIncList (Cons 1 (Cons 2 (Cons 3 Nil)))) Nil in
    let t = mirror (Node (Leaf 1) 10 (Leaf 2)) in
    let pt = addPoints (Point { px = 3, py = 4 }) (Point { px = 5, py = 6 }) in
    let opt = wrapSmall 9 in
    let optOk = case opt of SNone -> False; SSome v -> v == 9 in
    if pOk && sumList xs == 12 && treeSum t == 15 && pt.px == 8 && pt.py == 10 && optOk then 0
    else 1
"#;
    expect_ok("fip-valid-suite", src, 0);
}

#[test]
fn fip_rejects_fresh_alloc_excess_alloc_arity_mismatch_aliasing_closures_and_allocating_callees() {
    // 1. Fresh constructor allocation without any destructured input.
    expect_err(
        "fip-fresh-alloc",
        r#"
module Main where
  #[fip]
  freshCons :: Int -> List Int -> List Int
  freshCons x xs = Cons x xs

  main :: Int
  main = 0
"#,
        &["#[fip]", "Cons"],
    );

    // 2. Net positive allocation: 2 `Cons` allocations vs 1 reuse credit.
    expect_err(
        "fip-net-positive",
        r#"
module Main where
  #[fip]
  dupHead :: List Int -> List Int
  dupHead xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons x (Cons x rest)

  main :: Int
  main = 0
"#,
        &["#[fip]", "Cons"],
    );

    // 3. Arity mismatch: 1-field credit cannot satisfy 2-field constructor.
    expect_err(
        "fip-arity-mismatch",
        r#"
module Main where
  data Box1 where
    B1 :: Int -> Box1
  data Box2 where
    B2 :: Int -> Int -> Box2

  #[fip]
  grow :: Box1 -> Box2
  grow b = case b of
    B1 x -> B2 x x

  main :: Int
  main = 0
"#,
        &["#[fip]", "B2"],
    );

    // 4. Aliased/still-live scrutinee cannot grant a unique reuse credit.
    expect_err(
        "fip-aliased-scrutinee",
        r#"
module Main where
  #[fip]
  keepAndCons :: List Int -> List Int
  keepAndCons xs = case xs of
    Nil -> Nil
    Cons x _ -> Cons x xs

  main :: Int
  main = 0
"#,
        &["#[fip]", "Cons"],
    );

    // 5. Closure allocation is forbidden inside `#[fip]`.
    expect_err(
        "fip-closure",
        r#"
module Main where
  #[fip]
  mkLam :: Int -> Int
  mkLam x = let f = \y -> y + 1 in f x

  main :: Int
  main = 0
"#,
        &["#[fip]"],
    );

    // 6. Calling an allocating callee (`#[alloc("once")]`) is forbidden inside `#[fip]`.
    expect_err(
        "fip-allocating-callee",
        r#"
module Main where
  #[alloc("once")]
  wrap :: Int -> List Int -> List Int
  wrap x xs = Cons x xs

  #[fip]
  badCall :: Int -> List Int
  badCall x = wrap x Nil

  main :: Int
  main = 0
"#,
        &["#[fip]", "wrap"],
    );
}

// =========================================================================
// `.cinchif` interface contract facts & `build/<Entry>.manifest.json`
// =========================================================================

#[test]
fn alloc_bounded_and_fip_record_interface_and_manifest_facts() {
    let src = r#"
module Main where
  export makeTwo, incList

  #[alloc("bounded(2)")]
  makeTwo :: Bool -> Int -> List Int
  makeTwo flag x =
    if flag then Cons x (Cons (x + 1) Nil)
    else Cons x Nil

  #[fip]
  incList :: List Int -> List Int
  incList xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (x + 1) (incList rest)

  main :: Int
  main = 0
"#;
    expect_checks("iface-checks", src);
    let ifaces = check_source_interfaces(src, "Main.cinch").expect("interfaces");
    let main_iface = ifaces
        .iter()
        .find(|i| i.module == "Main")
        .expect("Main interface");

    let bounded_fact = main_iface
        .assertions
        .iter()
        .find(|a| a.name == "makeTwo")
        .expect("makeTwo assertion in .cinchif");
    assert!(
        bounded_fact.facts.iter().any(|f| f == "alloc=bounded(2)"),
        "expected alloc=bounded(2) in .cinchif assertions, got {:?}",
        bounded_fact.facts
    );

    let fip_fact = main_iface
        .assertions
        .iter()
        .find(|a| a.name == "incList")
        .expect("incList assertion in .cinchif");
    assert!(
        fip_fact.facts.iter().any(|f| f == "fip"),
        "expected fip in .cinchif assertions, got {:?}",
        fip_fact.facts
    );

    // Check manifest emission via `drive_with_profile`.
    let dir = write_project(&[("Main.cinch", src)]);
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("drive check");
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");
    assert!(
        manifest.contains("\"attribute\":\"alloc=bounded(2)\""),
        "manifest missing alloc=bounded(2): {manifest}"
    );
    assert!(
        manifest.contains("\"attribute\":\"fip\""),
        "manifest missing fip: {manifest}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
