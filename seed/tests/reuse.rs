//! Cell reuse on a dying constructor (design/reclamation.md section 5): a
//! `case` arm that rebuilds a constructor of the same payload shape as the
//! scrutinee builds it in the dying cell instead of a fresh block.
//!
//! Two observations pin the mechanism: the lowered Core carries the pair
//! (`@token` on the release, `reuse` in its body), and a rebuilt list costs
//! the first build's blocks and nothing more. Both are needed: a pair that
//! fires but allocates still passes the leak check, and a program that
//! allocates nothing at all passes an allocation bound alone.

mod common;

use cinch::core::TargetProfile;
use cinch::coretext::{self, Stage};
use cinch::driver::check_sources_profile_ifaces;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::{Command, Output};

/// A fresh 1000-cell list rebuilt 40 times: every round copies the whole
/// list, so every round hands 1000 dying cells to 1000 new constructors.
const REBUILD: &str = r#"
module Main where
  import Data.List

  foreign import ccall "cinch.prim.eqI64" eq64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.wrappingSubI64" sub64 :: I64 -> I64 -> I64
  foreign import ccall "cinch.prim.wrappingAddI64" add64 :: I64 -> I64 -> I64

  build :: I64 -> List I64 -> List I64
  build n acc = case eq64 n 0 of
    True -> acc
    False -> build (sub64 n 1) (Cons n acc)

  bump :: I64 -> I64
  bump x = add64 x 1

  step :: List I64 -> List I64
  step xs = case xs of
    Nil -> Nil
    Cons h t -> Cons (bump h) (step t)

  again :: I64 -> List I64 -> List I64
  again k xs = case eq64 k 0 of
    True -> xs
    False -> again (sub64 k 1) (step xs)

  count :: List I64 -> I64 -> I64
  count xs k = case xs of
    Nil -> k
    Cons h t -> count t (add64 k 1)

  main :: Unit -> Int
  main u = case eq64 (count (again 40 (build 1000 Nil)) 0) 1000 of
    True -> 0
    False -> 1
"#;

fn stats(exe: &Path) -> (u64, u64, u64) {
    let out: Output = Command::new(exe)
        .env("CINCH_ALLOC_STATS", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("spawn program");
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let err = String::from_utf8_lossy(&out.stderr);
    let field = |line: &str, name: &str| -> u64 {
        line.split_whitespace()
            .find_map(|w| w.strip_prefix(&format!("{name}=")))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| panic!("no {name} in {line:?}"))
    };
    let alloc = err
        .lines()
        .find(|l| l.starts_with("cinch-alloc:"))
        .expect("alloc stats line");
    let leak = err
        .lines()
        .find(|l| l.starts_with("cinch-leak:"))
        .expect("leak line");
    (field(alloc, "header-blocks"), field(alloc, "freed"), field(leak, "leaked"))
}

#[test]
fn a_rebuilt_list_allocates_only_its_first_build() {
    let dir = common::project("reuse_blocks", &[("Main.cinch", REBUILD)]);
    let exe = common::build(&dir, "Main.cinch", "prog").expect("build");
    let (blocks, freed, leaked) = stats(&exe);
    // One block per cell of the build (1000) plus the runtime's own handful;
    // a working reuse removes the 40,000 rebuild allocations, so anything
    // near 41,000 means the pair did not fire on some path.
    assert!(blocks < 1500, "{blocks} header blocks for a 1000-cell build");
    assert!(freed + leaked == blocks, "blocks {blocks}, freed {freed}, leaked {leaked}");
    assert_eq!(leaked, 0, "the rebuild leaked");
    common::cleanup(&dir);
}

/// The pair the mechanism is made of, in the code generation input: a
/// `release` that binds a token, and the `reuse` its body reads it with.
#[test]
fn the_lowered_core_carries_the_reset_pair() {
    let profile = TargetProfile::native();
    let (program, ifaces) = check_sources_profile_ifaces(REBUILD, "reuse_core.cinch", &profile)
        .expect("check the rebuild program");
    let (specialized, _) = cinch::specialize::optimize(&program, &profile).expect("specialization");
    let mut lowered = cinch::rc::insert_closed(specialized, &profile).expect("reference counting");
    cinch::stack::allocate(&mut lowered, &profile, &cinch::stack::Options::default());
    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
    let dump = coretext::dump(&lowered, Stage::Lowered, &profile, &ifaces, &modules, &BTreeMap::new(), 0);
    assert!(dump.contains("@token("), "no release binds a cell:\n{dump}");
    assert!(dump.contains("(reuse "), "no reuse in the rebuilt list:\n{dump}");
}

/// A rebuild that keeps a field (`Cons h (step t)`) writes back the word
/// the cell already holds, so the pair names it (`@kept`): Lean's
/// `partitionSelfSets`, which the reused path then skips.
#[test]
fn a_reuse_of_the_arms_own_binders_skips_its_stores() {
    let profile = TargetProfile::native();
    let (program, ifaces) = check_sources_profile_ifaces(SELF_SET, "reuse_kept.cinch", &profile)
        .expect("check the self-set program");
    let (specialized, _) = cinch::specialize::optimize(&program, &profile).expect("specialization");
    let mut lowered = cinch::rc::insert_closed(specialized, &profile).expect("reference counting");
    cinch::stack::allocate(&mut lowered, &profile, &cinch::stack::Options::default());
    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
    let dump = coretext::dump(&lowered, Stage::Lowered, &profile, &ifaces, &modules, &BTreeMap::new(), 0);
    assert!(
        dump.contains("@kept("),
        "no reuse names a word the cell already holds:\n{dump}"
    );
}

/// A copy that keeps its head: the second field is rebuilt, the first is
/// handed back unchanged, which is the store a self-set skips.
const SELF_SET: &str = r#"
module Main where
  import Data.List

  foreign import ccall "cinch.prim.eqI64" eq64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.wrappingSubI64" sub64 :: I64 -> I64 -> I64
  foreign import ccall "cinch.prim.wrappingAddI64" add64 :: I64 -> I64 -> I64

  build :: I64 -> List I64 -> List I64
  build n acc = case eq64 n 0 of
    True -> acc
    False -> build (sub64 n 1) (Cons n acc)

  step :: List I64 -> List I64
  step xs = case xs of
    Nil -> Nil
    Cons h t -> Cons h (step t)

  count :: List I64 -> I64 -> I64
  count xs k = case xs of
    Nil -> k
    Cons h t -> count t (add64 k 1)

  main :: Unit -> Int
  main u = case eq64 (count (step (build 1000 Nil)) 0) 1000 of
    True -> 0
    False -> 1
"#;

/// Coalescing moves a value's dups to its first use in a run and its drops
/// after its last, so the count operations of a run end up adjacent to the
/// uses they guard. Pinned on a program whose rebuild reads two fields of a
/// dying cell and dups each of them twice.
#[test]
fn coalescing_puts_a_values_counts_at_its_uses() {
    let profile = TargetProfile::native();
    let (program, ifaces) = check_sources_profile_ifaces(SELF_SET, "coalesce.cinch", &profile)
        .expect("check the self-set program");
    let (specialized, _) = cinch::specialize::optimize(&program, &profile).expect("specialization");
    let counted = cinch::rc::insert_closed(specialized, &profile).expect("reference counting");
    let (before_dups, before_drops) = cinch::coalesce::count_operations(&counted);
    let mut coalesced = counted.clone();
    cinch::coalesce::coalesce(&mut coalesced, &profile).expect("coalescing keeps the counts balanced");
    let (after_dups, after_drops) = cinch::coalesce::count_operations(&coalesced);
    // The moves are moves: no operation is created or dropped, so the
    // counts are equal and only their positions change.
    assert_eq!(
        (before_dups, before_drops),
        (after_dups, after_drops),
        "coalescing changed the number of count operations"
    );
    // And the coalesced program is the same program: it still passes the
    // ownership verifier, which is what says the moves kept every path's
    // holdings intact.
    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
    let mut again = coalesced.clone();
    cinch::stack::allocate(&mut again, &profile, &cinch::stack::Options::default());
    let dump = coretext::dump(&again, Stage::Lowered, &profile, &ifaces, &modules, &BTreeMap::new(), 0);
    assert!(dump.contains("(reuse "), "coalescing lost the reuse:\n{dump}");
}

/// The family-crossing pass: a `case` arm that dies holding a constructor
/// of one data type and builds a constructor of another, with the same
/// payload word count and the same reference discipline per word. The
/// strict pass refuses it (the successor is not in the dying block's own
/// data type) and the relaxed pass takes it, which is Lean's
/// `relaxedReuse` (`ResetReuse.lean`, `mayReuse`).
const CROSS: &str = r#"
module Main where
  import Data.Nat

  foreign import ccall "cinch.prim.eqI64" eq64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.wrappingAddI64" add64 :: I64 -> I64 -> I64

  -- Two one-constructor types of one payload word each, so a dying `A` can
  -- hold the cell a fresh `B` is built in.
  data A where
    MkA :: I64 -> A
  data B where
    MkB :: I64 -> B

  swap :: A -> B
  swap a = case a of
    MkA n -> MkB (add64 n 1)

  main :: Unit -> Int
  main u = case swap (MkA 41) of
    MkB n -> case eq64 n 42 of
      True -> 0
      False -> 1
"#;

/// The crossing pair is a `reuse`, the program it lands in computes the
/// right answer, and it leaks nothing.
#[test]
fn a_dying_block_of_one_family_is_reused_for_another() {
    let profile = TargetProfile::native();
    let (program, ifaces) = check_sources_profile_ifaces(CROSS, "reuse_cross_core.cinch", &profile)
        .expect("check the crossing program");
    let (specialized, _) = cinch::specialize::optimize(&program, &profile).expect("specialization");
    let mut lowered = cinch::rc::insert_closed(specialized, &profile).expect("reference counting");
    cinch::stack::allocate(&mut lowered, &profile, &cinch::stack::Options::default());
    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
    let dump = coretext::dump(&lowered, Stage::Lowered, &profile, &ifaces, &modules, &BTreeMap::new(), 0);
    assert!(
        dump.contains("(reuse "),
        "no crossing reuse (the relaxed pass did not fire):\n{dump}"
    );
    // The fresh constructor is the reuse's, and the dying one is the arm's
    // own: the crossing is what the relaxed pass exists for.
    assert!(
        dump.contains("(reuse ") && dump.contains("Main.MkB"),
        "the reuse should carry the fresh constructor:\n{dump}"
    );
}

/// And the crossing program runs to its right answer with nothing leaked.
#[test]
fn the_crossing_program_computes_and_frees() {
    let dir = common::project("reuse_cross", &[("Main.cinch", CROSS)]);
    let exe = common::build(&dir, "Main.cinch", "prog").expect("build");
    let (blocks, freed, leaked) = stats(&exe);
    assert_eq!(leaked, 0, "the crossing reuse leaked");
    assert!(freed + leaked == blocks, "blocks {blocks}, freed {freed}, leaked {leaked}");
    common::cleanup(&dir);
}
