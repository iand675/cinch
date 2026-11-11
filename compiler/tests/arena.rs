//! The bump arena from source: a scope enters one, builds constructor
//! cells in it, and leaves, and the cells cost the counted heap nothing.
//!
//! `compiler/runtime/src/arena.rs` serves `KIND_CTOR`, `KIND_CLOSURE` and
//! `KIND_DICT` blocks from chunked bump storage while an arena is entered.
//! The entries are `cinch.runtime.arenaNew`/`arenaLeave` (the reserved
//! intrinsic namespace), so the program below reaches them the only way a
//! program can. The same binary runs both arms: `CINCH_ARENA=0` makes the
//! entries no-ops, which is what "the arena off" means everywhere.

mod common;

use std::process::{Command, Output};

const SCOPED_LIST: &str = r#"
module Main where
  import Data.List

  foreign import ccall "cinch.runtime.arenaNew" arena-new :: I64 -> I64
  foreign import ccall "cinch.runtime.arenaLeave" arena-leave :: I64 -> I64

  cells :: Int -> List Int -> List Int
  cells n acc = case n == 0 of
    True -> acc
    False -> cells (n - 1) (Cons n acc)

  outcome :: Int -> Int
  outcome total = case total == 1000 of
    True -> 0
    False -> 1

  main :: Unit -> Int
  main u =
    let arena = arena-new 65536 in
    let list = cells 1000 Nil in
    let total = length list in
    case arena-leave arena of _ -> outcome total
"#;

fn run_with(exe: &std::path::Path, arena: &str) -> Output {
    Command::new(exe)
        .env_remove("CINCH_RC_DEBUG")
        .env("CINCH_ARENA", arena)
        .env("CINCH_ALLOC_STATS", "1")
        .env("CINCH_ALLOC_KINDS", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("run the built program")
}

/// The `cinch-alloc-kind:`/`cinch-arena-kind:` block count of one kind.
fn kind_blocks(stderr: &str, prefix: &str, kind: &str) -> u64 {
    for line in stderr.lines().filter(|line| line.starts_with(prefix)) {
        let mut fields = line.split_whitespace();
        let _ = fields.next();
        let Some(named) = fields.next().and_then(|field| field.strip_prefix("kind=")) else {
            continue;
        };
        let Some(blocks) = fields.next().and_then(|field| field.strip_prefix("blocks=")) else {
            continue;
        };
        if named == kind {
            return blocks.parse().unwrap_or(0);
        }
    }
    0
}

#[test]
fn a_scoped_arena_carries_a_program_list_and_leaves_nothing_behind() {
    let dir = common::project("arena_scope", &[("Main.cinch", SCOPED_LIST)]);
    let exe = common::build(&dir, "Main.cinch", "prog").expect("build the arena program");

    let entered = run_with(&exe, "1");
    assert_eq!(entered.status.code(), Some(0), "the arena arm runs");
    let stderr = String::from_utf8_lossy(&entered.stderr);
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "leave returns every chunk and the count check sees nothing live:\n{stderr}"
    );
    let arena_cells = kind_blocks(&stderr, "cinch-arena-kind:", "ctor");
    let heap_cells = kind_blocks(&stderr, "cinch-alloc-kind:", "ctor");
    assert!(
        arena_cells >= 1000,
        "the 1000 cells are served from the arena ({arena_cells}):\n{stderr}"
    );
    assert!(
        heap_cells * 10 < arena_cells,
        "the counted heap keeps no share of the list ({heap_cells} cells against {arena_cells}):\n{stderr}"
    );

    let disabled = run_with(&exe, "0");
    assert_eq!(disabled.status.code(), Some(0), "the disabled arm runs the same program");
    let stderr = String::from_utf8_lossy(&disabled.stderr);
    assert_eq!(
        kind_blocks(&stderr, "cinch-arena-kind:", "ctor"),
        0,
        "CINCH_ARENA=0 enters no arena:\n{stderr}"
    );
    assert!(
        kind_blocks(&stderr, "cinch-alloc-kind:", "ctor") >= 1000,
        "the disabled arm puts the same cells on the counted heap:\n{stderr}"
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "the disabled arm frees the list it dropped:\n{stderr}"
    );
}
