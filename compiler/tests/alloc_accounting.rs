//! Heap accounting and the heap cap, observed from a built program.
//!
//! Every block a compiled program allocates reaches the runtime's counting
//! allocator (`compiler/runtime/src/heap.rs`): `CINCH_ALLOC_STATS=1` makes
//! the process print its totals at exit, and `CINCH_HEAP_LIMIT` (default
//! 8 GiB) turns runaway allocation into a named fault with exit code 73.
//! Every spawned program here runs with an explicit small cap, so a broken
//! cap fails the test instead of exhausting the machine.

mod common;

use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// The heap-limit fault's exit code (`EXIT_HEAP_LIMIT` in the runtime).
const EXIT_HEAP_LIMIT: i32 = 73;

const LIST_1000: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = case n == 0 of
    True -> acc
    False -> build (n - 1) (Cons n acc)

  main :: Unit -> Int
  main u = case length (build 1000 Nil) == 1000 of
    True -> 0
    False -> 1
"#;

const UNBOUNDED: &str = r#"
module Main where
  import Data.List

  grow :: Int -> List Int -> Int
  grow n acc = grow (n + 1) (Cons n acc)

  main :: Unit -> Int
  main u = grow 0 Nil
"#;

fn built(tag: &str, src: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let dir = common::project(tag, &[("Main.cinch", src)]);
    let exe = common::build(&dir, "Main.cinch", "prog").expect("build");
    (dir, exe)
}

fn run_with(exe: &Path, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env_remove("CINCH_ALLOC_STATS").env("CINCH_HEAP_LIMIT", "256M");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn program")
}

fn stats_line(out: &Output) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|l| l.starts_with("cinch-alloc:"))
        .map(str::to_owned)
}

fn field(line: &str, name: &str) -> u64 {
    line.split_whitespace()
        .find_map(|w| w.strip_prefix(&format!("{name}=")))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no {name} in {line:?}"))
}

#[test]
fn stats_line_counts_list_cells_only_when_asked() {
    let (dir, exe) = built("alloc_stats", LIST_1000);
    let out = run_with(&exe, &[("CINCH_ALLOC_STATS", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let line = stats_line(&out).expect("cinch-alloc line");
    let blocks = field(&line, "blocks");
    let bytes = field(&line, "bytes");
    let peak = field(&line, "peak");
    assert!(blocks >= 1000, "{line}");
    assert!(bytes >= 1000 * 24, "{line}");
    assert!(peak > 0 && peak <= bytes, "{line}");
    let (hblocks, hbytes) = (field(&line, "header-blocks"), field(&line, "header-bytes"));
    assert_eq!(
        line,
        format!(
            "cinch-alloc: blocks={blocks} bytes={bytes} peak={peak} header-blocks={hblocks} \
             header-bytes={hbytes} immortal=0 freed=0"
        )
    );
    assert!(hblocks >= 1000 && hblocks <= blocks, "{line}");

    let quiet = run_with(&exe, &[]);
    assert_eq!(quiet.status.code(), Some(0));
    assert!(quiet.stderr.is_empty(), "{quiet:?}");
    common::cleanup(&dir);
}

/// A list of `n` machine `I64`s built and counted through the primitive
/// operations directly (no `Int`/`Nat` arithmetic, no class dictionaries),
/// so the only heap values that depend on `n` are its `n` `Cons` cells.
fn cons_program(n: usize) -> String {
    format!(
        r#"
module Main where
  import Data.List

  foreign import ccall "cinch.prim.eqI64" eq64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.wrappingSubI64" sub64 :: I64 -> I64 -> I64
  foreign import ccall "cinch.prim.wrappingAddI64" add64 :: I64 -> I64 -> I64

  build :: I64 -> List I64 -> List I64
  build n acc = case eq64 n 0 of
    True -> acc
    False -> build (sub64 n 1) (Cons n acc)

  count :: List I64 -> I64 -> I64
  count xs k = case xs of
    Nil -> k
    Cons h t -> count t (add64 k 1)

  main :: Unit -> Int
  main u = case eq64 (count (build {n} Nil) 0) {n} of
    True -> 0
    False -> 1
"#
    )
}

fn stats_of(exe: &Path) -> String {
    let out = run_with(exe, &[("CINCH_ALLOC_STATS", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    stats_line(&out).expect("cinch-alloc line")
}

/// Header blocks are exactly the program's Cinch values, kept apart from the
/// runtime's own allocations: 2000 more `Cons` cells are 2000 more header
/// blocks of 32 bytes each (the 24-byte `{ tag; head; tail }` block plus its
/// 8-byte header) and no other allocation at all.
#[test]
fn header_blocks_count_constructor_cells_with_their_header() {
    let (dir_a, small) = built("hdr_small", &cons_program(1000));
    let (dir_b, large) = built("hdr_large", &cons_program(3000));
    let (a, b) = (stats_of(&small), stats_of(&large));
    let delta = |name: &str| field(&b, name) - field(&a, name);
    assert_eq!(delta("header-blocks"), 2000, "{a}\n{b}");
    assert_eq!(delta("header-bytes"), 2000 * 32, "{a}\n{b}");
    assert_eq!(delta("blocks"), 2000, "no runtime-internal allocation per cell\n{a}\n{b}");
    assert_eq!(delta("bytes"), 2000 * 32, "{a}\n{b}");
    for line in [&a, &b] {
        assert!(field(line, "header-blocks") >= 1000, "{line}");
        assert!(field(line, "header-blocks") <= field(line, "blocks"), "{line}");
        assert!(field(line, "header-bytes") <= field(line, "bytes"), "{line}");
        assert_eq!(field(line, "freed"), 0, "{line}");
    }
    common::cleanup(&dir_a);
    common::cleanup(&dir_b);
}

/// Interned symbols are immortal header blocks, one per distinct name: two
/// names interned five times are two immortal blocks.
#[test]
fn interned_symbols_are_counted_immortal_once_per_name() {
    const SYMS: &str = r#"
module Main where
  import Data.Sym

  main :: Unit -> Int
  main u = case intern "alpha" == intern "alpha" of
    True -> case intern "beta" == intern "alpha" of
      True -> 2
      False -> case intern "beta" == intern "beta" of
        True -> 0
        False -> 3
    False -> 1
"#;
    let (dir, exe) = built("hdr_syms", SYMS);
    let line = stats_of(&exe);
    assert_eq!(field(&line, "immortal"), 2, "{line}");
    assert!(field(&line, "header-blocks") >= 2, "{line}");
    common::cleanup(&dir);
}

#[test]
fn unbounded_allocation_hits_the_named_cap() {
    let (dir, exe) = built("alloc_cap", UNBOUNDED);
    let start = Instant::now();
    let out = run_with(&exe, &[("CINCH_HEAP_LIMIT", "64M")]);
    assert_eq!(out.status.code(), Some(EXIT_HEAP_LIMIT), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(&format!("cinch: heap limit exceeded ({} bytes)", 64u64 << 20)),
        "{stderr}"
    );
    assert!(start.elapsed() < Duration::from_secs(30));
    common::cleanup(&dir);
}

#[test]
fn normal_program_runs_under_the_default_cap() {
    let (dir, exe) = built("alloc_default", LIST_1000);
    // The default cap applies when the variable is unset; this program is
    // bounded by construction (1000 cells), so leaving it unset is safe.
    let out = Command::new(&exe)
        .env_remove("CINCH_HEAP_LIMIT")
        .env_remove("CINCH_ALLOC_STATS")
        .output()
        .expect("spawn program");
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(out.stderr.is_empty());
    common::cleanup(&dir);
}
