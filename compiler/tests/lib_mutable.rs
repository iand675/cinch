//! The mutable containers (`Data.Mutable.*`): the behaviour program
//! `fixtures/mutable_containers.cinch`, compiled and run through the
//! driver the CLI uses, plus the source-level properties the trusted core
//! rests on.
//!
//! The program's exit code is its verdict: 0 when every check passed,
//! otherwise the number of the first check that failed, in `CHECKS`
//! order. The checks cover growth and indexing (`Vec`, `Deque`), the
//! middle operations and the introsort (random, sorted, reversed and
//! constant inputs, and a `String` comparator), freezing and thawing
//! against `Data.Array`, iteration whose action mutates the container,
//! the hash table at 50,000 keys, delete churn (twenty rounds of 5,000
//! inserts and deletes must leave the capacity where it was: the F14
//! overflow counts make deletion tombstone-free), 300 keys sharing one
//! digest, `String` keys through the whole update family, the set, the
//! ring wrapping while it grows, the 4-ary heap (min, max and Floyd
//! build), and the fixed array and the cell.
//!
//! A container bug that reads an empty slot is not a wrong answer here but
//! a trap (`cinch_slots_fault`, exit 71), so the leak line and the exit
//! code together say the program both computed the right answers and kept
//! every reference count balanced.

mod common;

const MAIN: &str = include_str!("fixtures/mutable_containers.cinch");

/// The modules a client copies beside its entry file (the non-embedded
/// source closure: `Data.Mutable.*` and `Data.Array`).
const MODULES: &[(&str, &str)] = &[
    ("Data.Mutable.Vec.cinch", include_str!("../prelude/Data/Mutable/Vec.cinch")),
    ("Data.Mutable.HashTable.cinch", include_str!("../prelude/Data/Mutable/HashTable.cinch")),
    ("Data.Mutable.HashSet.cinch", include_str!("../prelude/Data/Mutable/HashSet.cinch")),
    ("Data.Mutable.Deque.cinch", include_str!("../prelude/Data/Mutable/Deque.cinch")),
    ("Data.Mutable.Heap.cinch", include_str!("../prelude/Data/Mutable/Heap.cinch")),
    ("Data.Mutable.Array.cinch", include_str!("../prelude/Data/Mutable/Array.cinch")),
    ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
    ("Data.Mutable.cinch", include_str!("../prelude/Data/Mutable.cinch")),
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
];

/// The checks, in the order `main` runs them (exit code k names check k).
const CHECKS: &[&str] = &[
    "vec-basics",
    "vec-middle",
    "vec-sort",
    "vec-views",
    "table-scale",
    "table-delete-churn",
    "table-collisions",
    "table-updates",
    "set",
    "deque",
    "heap",
    "array-var",
];

fn project(tag: &str, main: &str) -> std::path::PathBuf {
    let mut files: Vec<(&str, &str)> = MODULES.to_vec();
    files.push(("Main.cinch", main));
    common::project(tag, &files)
}

#[test]
fn every_container_check_passes() {
    let dir = project("mutable_behaviour", MAIN);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code,
        0,
        "the mutable containers' first failed check is {}",
        usize::try_from(code)
            .ok()
            .and_then(|k| k.checked_sub(1))
            .and_then(|k| CHECKS.get(k))
            .copied()
            .unwrap_or("not a check number (a trap or a crash)")
    );
}

#[test]
fn the_facade_reexports_every_container() {
    // One import, `Data.Mutable`, reaches every container's vocabulary.
    let main = r#"
module Main where

  import Data.Option
  import Data.Mutable

  main :: Unit ->{Mut r} Int
  main u =
    let v = vec-from-list (Cons (3 :: Nat) (Cons 1 (Cons 2 Nil))) in
    let s = vec-sort v in
    let t = hash-table-new () in
    let i = hash-table-insert t "k" (1 :: Nat) in
    let q = deque-new () in
    let p = deque-push-front q (5 :: Nat) in
    let h = heap-from-list (Cons (4 :: Nat) (Cons 2 Nil)) in
    let c = var-new (0 :: Nat) in
    let w = var-write c 9 in
    let a = mut-array-new 2 (7 :: Nat) in
    let hs = hash-set-from-list (Cons (1 :: Nat) (Cons 1 Nil)) in
    if vec-length v == 3 && hash-table-size t == 1 && deque-length q == 1 && heap-length h == 2
       && var-read c == 9 && mut-array-length a == 2 && hash-set-size hs == 1 then 0 else 1
"#;
    let dir = project("mutable_facade", main);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0);
}

#[test]
fn a_pure_function_cannot_touch_a_container() {
    // The `Mut r` row is the capability: a definition without it in its
    // row cannot call a container operation, so purity is kept by the
    // checker, not by convention.
    let main = r#"
module Main where

  import Data.Mutable.Vec

  sneaky :: Nat -> Nat
  sneaky n =
    let v = vec-new () in
    let p = vec-push v n in
    vec-length v

  main :: Unit ->{Mut r} Int
  main u = if sneaky 3 == 1 then 0 else 1
"#;
    let dir = project("mutable_purity", main);
    let checked = common::check(&dir, "Main.cinch");
    common::cleanup(&dir);
    let err = checked.expect_err("a pure definition calling a container operation must be refused");
    assert!(err.contains("effect rows do not match"), "{err}");
}

#[test]
fn the_trusted_core_never_leaves_its_modules() {
    // Every module declares the `cinch.runtime.slots*` intrinsics it uses
    // privately; none may appear in an export list, since a read of an
    // unwritten slot is only sound behind a container's own invariant.
    for (name, source) in MODULES.iter().filter(|(n, _)| n.starts_with("Data.Mutable.")) {
        let imported: Vec<&str> = source
            .lines()
            .filter_map(|l| l.trim().strip_prefix("foreign import ccall \"cinch.runtime.slots"))
            .filter_map(|rest| rest.split('"').nth(1))
            .filter_map(|rest| rest.split_whitespace().next())
            .collect();
        for line in source.lines().map(str::trim).filter(|l| l.starts_with("export ")) {
            for word in line.trim_start_matches("export ").split(',').map(str::trim) {
                assert!(!imported.contains(&word), "{name} exports its trusted intrinsic {word}");
            }
        }
    }
}
