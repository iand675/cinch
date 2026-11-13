//! Count-operation savings of design/reclamation.md section 4 ("As
//! built"): borrowed parameters of a closed program, drop specialisation
//! (`Core::Release`) and static dictionaries. Each program is checked for
//! where the operations went in its IR, and runs leak-free both plainly
//! and from a `rc_debug` build under `CINCH_RC_DEBUG=1`.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
use std::path::{Path, PathBuf};
use std::process::Command;

const PROGRAM: &str = r#"
module Main where
  import Data.List

  signature Size a where
    size :: a -> Int

  implicit instance SizeInt :: Size Int where
    size x = 1

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons h t -> size h + count t

  lastOr :: Int -> List Int -> Int
  lastOr d xs = case xs of
    Nil -> d
    Cons h t -> lastOr h t

  bump :: List Int -> List Int
  bump xs = case xs of
    Nil -> xs
    Cons h t -> Cons (h + 1) (bump t)

  main :: Int
  main =
    let xs = build 200 Nil in
    let ys = bump xs in
    let zs = bump (build 50 Nil) in
    if count xs == 200 then (if count ys == 200 then (if count zs == 50 then (if lastOr 0 ys == 201 then 0 else 4) else 3) else 2) else 1
"#;

fn project(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_rc_spec_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), PROGRAM).expect("write source");
    dir
}

fn drive(mode: Mode, dir: &Path, out: &Path, debug: bool) {
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    let (src, out) = (dir.join("Main.cinch"), out.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver_with_profile(mode, StageMode::Refresh, &src, Some(&out), &profile))
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("driver failed: {e}"));
}

/// The body of the function `name` in `ir`.
fn body<'a>(ir: &'a str, name: &str) -> &'a str {
    let head = format!("@\"{name}\"(");
    let start = ir
        .lines()
        .scan(0usize, |at, line| {
            let here = *at;
            *at += line.len() + 1;
            Some((here, line))
        })
        .find(|(_, line)| line.starts_with("define ") && line.contains(&head))
        .map(|(at, _)| at)
        .unwrap_or_else(|| panic!("no definition of {name}"));
    let end = ir[start..].find("\n}\n").expect("function end");
    &ir[start..start + end]
}

#[test]
fn borrowed_parameters_release_and_static_dictionaries_place_fewer_operations() {
    let dir = project("ir");
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // The instance has only methods: one immortal static block. The
    // simplifier's `direct-dict-call` rule turns the projection from that
    // block into a direct method call, so nothing reads the block and no
    // The instance has only methods and the method's body is a literal, so
    // the unfolding inliner copies it to the site and both the dictionary
    // block and the method unit are swept: neither is named anywhere. This
    // was once a surviving `Main.SizeInt.dict` block with a direct method
    // call; the copy is the last step, and the block is now dead.
    assert!(
        !ir.contains("Main.SizeInt"),
        "the static dictionary or its method survived:\n{ir}"
    );
    let count = body(&ir, "Main.count");
    assert!(!count.contains("@__apply("), "count still applies a closure:\n{count}");
    assert!(
        count.contains("call i64 @\"cinch.runtime.intAdd\""),
        "the sum is not the runtime addition:\n{count}"
    );
    // `lastOr` only reads its list, so the parameter is borrowed and no
    // count operation names it. The assertion is on the parameter's own
    // slot, not on every cell in the body: the loop re-reads interior cells
    // after the recursion returns, and those are not the parameter.
    let last = body(&ir, "Main.lastOr");
    for param in ["%p0", "%p1"] {
        for op in ["cinch.rc.dup.obj", "cinch.rc.dup.tobj", "cinch.rc.drop.obj", "cinch.rc.drop.tobj"] {
            let named = format!("{op}(i64 {param})");
            assert!(!last.contains(&named), "lastOr counts its parameter {param}:\n{last}");
        }
    }
    // `bump` returns its list on one path, so it owns the cell and releases
    // it. The release is the counted drop: the last-reference test and the
    // free live inside `cinch.rc.drop.obj` (whose tail switches on the
    // count and calls `cinch_drop_slow`), so the body names the drop rather
    // than a bare `cinch_free` it used to.
    let bump = body(&ir, "Main.bump");
    assert!(
        bump.contains("call void @cinch.rc.drop.obj(i64"),
        "bump does not release the cell it owns:\n{bump}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn borrowed_parameters_release_and_static_dictionaries_free_everything() {
    let dir = project("run");
    for debug in [false, true] {
        let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
        drive(Mode::Build, &dir, &exe, debug);
        let mut cmd = Command::new(&exe);
        cmd.env("CINCH_HEAP_LIMIT", "64M").env("CINCH_LEAK_CHECK", "1").env_remove("CINCH_ALLOC_STATS");
        if debug {
            cmd.env("CINCH_RC_DEBUG", "1");
        }
        let out = cmd.output().expect("spawn program");
        let err = String::from_utf8_lossy(&out.stderr);
        assert!(err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")), "debug {debug}: {err}");
        assert_eq!(out.status.code(), Some(0), "debug {debug}: {err}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------- branch reuse
//
// `expandResetReuse` (Lean 4's `ExpandResetReuse.lean`): when no constructor
// allocation sits on an arm's straight-line spine, a branch whose *every*
// alternative allocates a compatible constructor takes its own reset/reuse
// pair per alternative. Each alternative binds its own token, so the
// one-token-one-reuse rule holds per path and the join merges the arms'
// results. These tests pin the shape that fires, the shape that must be
// refused, and the recursion the rule has to survive.

/// One project per source, so each shape is measured on its own program.
fn source_project(tag: &str, source: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_rc_branch_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), source).expect("write source");
    dir
}

/// Build, run, and return (exit code, stderr) under the leak check.
fn build_and_run(dir: &Path, name: &str, debug: bool) -> (Option<i32>, String) {
    let exe = dir.join(name);
    drive(Mode::Build, dir, &exe, debug);
    let mut cmd = Command::new(&exe);
    cmd.env("CINCH_HEAP_LIMIT", "256M").env("CINCH_LEAK_CHECK", "1").env_remove("CINCH_ALLOC_STATS");
    if debug {
        cmd.env("CINCH_RC_DEBUG", "1");
    }
    let out = cmd.output().expect("spawn program");
    (out.status.code(), String::from_utf8_lossy(&out.stderr).into_owned())
}

/// How many times a unit's body frees a block, which is what a reuse pair
/// removes from a path. A reuse emits no named symbol (it is inline stores
/// into the kept cell), so the observable is the `cinch_free` the unique
/// path would otherwise run: a branch whose every arm reuses frees nothing,
/// and an arm with no allocation to reuse into still frees.
fn frees(ir: &str, name: &str) -> usize {
    body(ir, name).matches("call void @cinch_free(i64").count()
}

const IF_SHAPED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> if y == 0 then Cons 1 (rebuild ys) else Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) == 1000 then 0 else 1
"#;

const CASE_SHAPED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> case y == 0 of
      True -> Cons 1 (rebuild ys)
      False -> Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) == 1000 then 0 else 1
"#;

/// One arm allocates, the other returns the tail unchanged. The pair must
/// be refused: the second arm has nothing to reuse the cell into, so a
/// per-arm pair there would strand the cell on that path.
const MIXED_ARMS: &str = r#"
module Main where
  import Data.List

  mixed :: List Int -> List Int
  mixed xs = case xs of
    Nil -> Nil
    Cons y ys -> case y == 0 of
      True -> Cons 1 (mixed ys)
      False -> ys

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (mixed (build 1000 Nil)) >= 0 then 0 else 1
"#;

/// A self-recursive rebuild whose allocation is inside a branch on every
/// path: the rule has to survive recursion, which is what the borrow-mode
/// fixed point is for (a recursive callee's own modes are derived with its
/// recursive call in view).
const RECURSIVE_BRANCHED: &str = r#"
module Main where
  import Data.List

  rebuild :: List Int -> List Int
  rebuild xs = case xs of
    Nil -> Nil
    Cons y ys -> if y == 0 then Cons (count (rebuild ys)) Nil else Cons y (rebuild ys)

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  count :: List Int -> Int
  count xs = case xs of
    Nil -> 0
    Cons _ t -> 1 + count t

  main :: Int
  main = if count (rebuild (build 1000 Nil)) >= 0 then 0 else 1
"#;

#[test]
fn an_if_shaped_rebuild_reuses_one_pair_per_branch() {
    let dir = source_project("if", IF_SHAPED);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // Both alternatives allocate a `Cons`, so both take a pair. The unit
    // names the reuse twice and, critically, frees the cell on neither
    // path: a free here would mean the arm kept the block alive with no
    // reuse to take it.
    assert_eq!(
        frees(&ir, "Main.rebuild"),
        0,
        "the if-shaped rebuild still frees the dying cell on a path:\n{}",
        body(&ir, "Main.rebuild")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the if-shaped rebuild leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_case_shaped_rebuild_reuses_one_pair_per_alternative() {
    let dir = source_project("case", CASE_SHAPED);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    assert_eq!(
        frees(&ir, "Main.rebuild"),
        0,
        "the case-shaped rebuild still frees the dying cell on a path:\n{}",
        body(&ir, "Main.rebuild")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the case-shaped rebuild leaked: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_branch_whose_arm_allocates_nothing_is_refused() {
    let dir = source_project("mixed", MIXED_ARMS);
    let ir_path = dir.join("prog.ll");
    drive(Mode::EmitIr, &dir, &ir_path, false);
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    // The `False` arm returns the tail itself, so there is no allocation to
    // reuse the dying cell into. The pair must not be placed: a per-arm
    // release on that path would keep the cell alive with nothing to take
    // it, which is the leak the straight-line rule exists to prevent.
    assert!(
        frees(&ir, "Main.mixed") > 0,
        "a branch with a non-allocating arm took a per-arm pair, which would \
         strand the cell on that path:\n{}",
        body(&ir, "Main.mixed")
    );
    let (code, err) = build_and_run(&dir, "prog", false);
    assert_eq!(code, Some(0), "{err}");
    assert!(
        err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
        "the refused shape leaked anyway: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_recursive_branched_rebuild_is_leak_free_in_both_builds() {
    let dir = source_project("rec", RECURSIVE_BRANCHED);
    // The property the rule has to survive: a recursive unit whose
    // allocation is inside a branch on every path. Both a plain build and
    // an `rc_debug` build must run leak-free, because the debug profile
    // traps on a use after free rather than reading reused memory.
    for (name, debug) in [("prog", false), ("prog_dbg", true)] {
        let (code, err) = build_and_run(&dir, name, debug);
        assert_eq!(code, Some(0), "debug {debug}: {err}");
        assert!(
            err.lines().any(|l| l.starts_with("cinch-leak:") && l.split_whitespace().any(|w| w == "leaked=0")),
            "debug {debug} leaked: {err}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
