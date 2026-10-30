//! Reclamation (design/reclamation.md slice S6): compiled programs built
//! with reference counting (`TargetProfile::reclaim`) free what they
//! allocate. Each program runs under `CINCH_LEAK_CHECK=1`, which reports
//! `cinch-leak: live= immortal= leaked=` at exit and exits 74 when a
//! non-immortal block is still live, and again from a `rc_debug` build
//! under `CINCH_RC_DEBUG=1`, where a count operation on a freed block
//! traps (exit 71).

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn project(tag: &str, src: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_rc_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    dir
}

fn build(dir: &Path, debug: bool) -> PathBuf {
    let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    let (src, out) = (dir.join("Main.cinch"), exe.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver_with_profile(Mode::Build, StageMode::Refresh, &src, Some(&out), &profile))
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("build failed: {e}"));
    exe
}

fn run(exe: &Path, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M").env_remove("CINCH_ALLOC_STATS");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn program")
}

fn field(out: &Output, prefix: &str, name: &str) -> u64 {
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err.lines().find(|l| l.starts_with(prefix)).unwrap_or_else(|| panic!("no {prefix} line: {err}"));
    line.split_whitespace()
        .find_map(|w| w.strip_prefix(&format!("{name}=")))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no {name} in {line}"))
}

/// Build `src` both ways, check the program exits `code` with
/// `leaked` blocks left, plainly and under the debug checks.
fn expect_leaked(tag: &str, src: &str, code: i32, leaked: u64) {
    let dir = project(tag, src);
    for debug in [false, true] {
        let exe = build(&dir, debug);
        let mut env = vec![("CINCH_LEAK_CHECK", "1")];
        if debug {
            env.push(("CINCH_RC_DEBUG", "1"));
        }
        let out = run(&exe, &env);
        let n = field(&out, "cinch-leak:", "leaked");
        assert_eq!(n, leaked, "{tag} (debug {debug}): {out:?}");
        let want = if leaked == 0 { code } else { 74 };
        assert_eq!(out.status.code(), Some(want), "{tag} (debug {debug}): {out:?}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

const LISTS: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  sumL :: List Int -> Int -> Int
  sumL xs acc = case xs of
    Nil -> acc
    Cons h t -> sumL t (acc + h)

  mapL :: (Int -> Int) -> List Int -> List Int
  mapL f xs = case xs of
    Nil -> Nil
    Cons h t -> Cons (f h) (mapL f t)

  filterL :: (Int -> Bool) -> List Int -> List Int
  filterL p xs = case xs of
    Nil -> Nil
    Cons h t -> if p h then Cons h (filterL p t) else filterL p t

  main :: Int
  main =
    let xs = build 1000 Nil in
    let ys = mapL (\x -> x * 2) (filterL (\x -> x > 500) xs) in
    if sumL xs 0 == 500500 then (if sumL ys 0 == 750500 then 0 else 2) else 1
"#;

#[test]
fn lists_built_mapped_filtered_and_folded_are_freed() {
    expect_leaked("lists", LISTS, 0, 0);
}

const TREES: &str = r#"
module Main where
  data Tree where
    Leaf :: Tree
    Node :: Tree -> Int -> Tree -> Tree

  insert :: Int -> Tree -> Tree
  insert x t = case t of
    Leaf -> Node Leaf x Leaf
    Node l v r -> if x < v then Node (insert x l) v r else Node l v (insert x r)

  total :: Tree -> Int
  total t = case t of
    Leaf -> 0
    Node l v r -> total l + v + total r

  fill :: Int -> Tree -> Tree
  fill n t = if n == 0 then t else fill (n - 1) (insert ((n * 37) - (n / 7) * 7) t)

  main :: Int
  main = if total (fill 200 Leaf) > 0 then 0 else 1
"#;

#[test]
fn trees_rebuilt_by_insertion_are_freed() {
    expect_leaked("trees", TREES, 0, 0);
}

const CLOSURES: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  add3 :: Int -> Int -> Int -> Int
  add3 a b c = a + b + c

  apply :: (Int -> Int) -> Int -> Int
  apply f x = f x

  main :: Int
  main =
    let xs = build 50 Nil in
    let count = \u -> length xs in
    let g = add3 1 in
    let h = g 2 in
    if count () + count () == 100 then (if apply h 3 + apply (add3 1 1) 1 == 9 then 0 else 2) else 1
"#;

#[test]
fn closures_and_partial_applications_are_freed() {
    expect_leaked("closures", CLOSURES, 0, 0);
}

const DICTS: &str = r#"
module Main where
  signature Box a where
    unbox :: a -> Int

  data Wrap where
    W :: Int -> Wrap

  implicit instance Box_Wrap :: Box Wrap where
    unbox w = case w of
      W n -> n

  twice :: forall a. using (b :: Box a) => a -> Int
  twice w = unbox w + unbox w

  addN :: forall a. using (add :: Add a) => a -> a -> a
  addN x y = x + y

  main :: Int
  main = if twice (W 3) + addN 1 1 == 8 then 0 else 1
"#;

#[test]
fn dictionaries_are_freed() {
    expect_leaked("dicts", DICTS, 0, 0);
}

const BIGNUMS: &str = r#"
module Main where
  grow :: Int -> Int -> Int
  grow n acc = if n == 0 then acc else grow (n - 1) (acc * 3)

  main :: Int
  main = if grow 200 1 > 1000000000000000000000000000000 then 0 else 1
"#;

#[test]
fn bignum_temporaries_are_freed() {
    expect_leaked("bignums", BIGNUMS, 0, 0);
}

/// A `letrec` value knot stays allocated until S8 tears knots down
/// (D396): the box keeps its value (an uncounted slot), and the value's
/// closure keeps the box. Exactly those blocks remain: the box, the
/// tuple, the closure.
const KNOT: &str = r#"
module Main where
  main :: Int
  main =
    letrec r :: (Nat, Unit -> Nat); r = (41, \u -> case r of (a, g) -> a + 1)
    in case r of (a, g) -> if g () == 42 then 0 else 1
"#;

#[test]
fn a_letrec_knot_leaks_exactly_its_cycle() {
    expect_leaked("knot", KNOT, 0, 3);
}

fn loop_program(n: usize, rounds: usize) -> String {
    format!(
        r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  rounds :: Int -> Int -> Int
  rounds k acc = if k == 0 then acc else rounds (k - 1) (acc + length (build {n} Nil))

  main :: Int
  main = if rounds {rounds} 0 == {total} then 0 else 1
"#,
        total = n * rounds
    )
}

#[test]
fn building_and_dropping_a_list_repeatedly_peaks_near_one_list() {
    let dir = project("peak", &loop_program(1000, 1000));
    let exe = build(&dir, false);
    let out = run(&exe, &[("CINCH_ALLOC_STATS", "1"), ("CINCH_LEAK_CHECK", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let peak = field(&out, "cinch-alloc:", "peak");
    let bytes = field(&out, "cinch-alloc:", "bytes");
    // One list of 1000 cells is 32 KB with headers; a thousand of them
    // would be 32 MB.
    assert!(peak < 1_000_000, "peak {peak} of {bytes} allocated: {out:?}");
    assert!(bytes > 30_000_000, "{out:?}");
    assert_eq!(field(&out, "cinch-leak:", "leaked"), 0, "{out:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_million_cell_list_drops_without_stack_growth() {
    let dir = project("million", &loop_program(1_000_000, 1));
    let exe = build(&dir, false);
    let out = run(&exe, &[("CINCH_LEAK_CHECK", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert_eq!(field(&out, "cinch-leak:", "leaked"), 0, "{out:?}");
    let _ = std::fs::remove_dir_all(&dir);
}
