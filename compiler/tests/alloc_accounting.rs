//! Heap accounting and the heap cap, observed from a built program.
//!
//! Every block a compiled program allocates reaches the runtime's counting
//! allocator (`compiler/runtime/src/heap.rs`): `CINCH_ALLOC_STATS=1` makes
//! the process print its totals at exit, and `CINCH_HEAP_LIMIT` (default
//! 8 GiB) caps the live heap: a program whose live bytes would pass it
//! takes the `Exhausted` exit (code 73, `cinch: exhausted: heap (<n>
//! bytes)`), while a program that frees what it is done with runs on
//! however much it allocates over its life. Every spawned program here
//! runs with an explicit small cap, so a broken cap fails the test
//! instead of exhausting the machine.

mod common;

use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// The `Exhausted` exit code (`EXIT_EXHAUSTED` in the runtime).
const EXIT_EXHAUSTED: i32 = 73;

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
    let freed = field(&line, "freed");
    assert_eq!(
        line,
        format!(
            "cinch-alloc: blocks={blocks} bytes={bytes} peak={peak} header-blocks={hblocks} \
             header-bytes={hbytes} immortal=0 freed={freed} live={}",
            hblocks - freed
        )
    );
    assert!(hblocks >= 1000 && hblocks <= blocks, "{line}");
    // The list dies once `length` has counted it: every cell is freed.
    assert!(freed >= 1000, "{line}");

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
/// blocks of 24 bytes each (the 16-byte `{ head; tail }` block plus its
/// 8-byte header, which carries the tag in its layout id) and no other
/// allocation at all.
#[test]
fn header_blocks_count_constructor_cells_with_their_header() {
    let (dir_a, small) = built("hdr_small", &cons_program(1000));
    let (dir_b, large) = built("hdr_large", &cons_program(3000));
    let (a, b) = (stats_of(&small), stats_of(&large));
    let delta = |name: &str| field(&b, name) - field(&a, name);
    assert_eq!(delta("header-blocks"), 2000, "{a}\n{b}");
    assert_eq!(delta("header-bytes"), 2000 * 24, "{a}\n{b}");
    assert_eq!(delta("blocks"), 2000, "no runtime-internal allocation per cell\n{a}\n{b}");
    assert_eq!(delta("bytes"), 2000 * 24, "{a}\n{b}");
    // The extra cells die once counted.
    assert_eq!(delta("freed"), 2000, "{a}\n{b}");
    assert_eq!(delta("live"), 0, "{a}\n{b}");
    for line in [&a, &b] {
        assert!(field(line, "header-blocks") >= 1000, "{line}");
        assert!(field(line, "header-blocks") <= field(line, "blocks"), "{line}");
        assert!(field(line, "header-bytes") <= field(line, "bytes"), "{line}");
    }
    common::cleanup(&dir_a);
    common::cleanup(&dir_b);
}

/// A data type of 70 constructors, more than one layout group's 64 tags,
/// so every constructor is escaped (`block_kinds.rs`): its tag lives in a
/// descriptor word before the header. `n` cells of the one-field `C69`
/// are chained and counted, and `rank` matches constructors on both
/// sides of tag 64, by name and by wildcard.
fn escaped_program(n: usize) -> String {
    let mut ctors = String::new();
    for i in 0..69 {
        ctors.push_str(&format!("    C{i} :: Big\n"));
    }
    ctors.push_str("    C69 :: Big -> Big\n");
    format!(
        r#"
module Main where
  foreign import ccall "cinch.prim.eqI64" eq64 :: I64 -> I64 -> Bool
  foreign import ccall "cinch.prim.wrappingSubI64" sub64 :: I64 -> I64 -> I64
  foreign import ccall "cinch.prim.wrappingAddI64" add64 :: I64 -> I64 -> I64

  data Big where
{ctors}
  build :: I64 -> Big -> Big
  build n acc = case eq64 n 0 of
    True -> acc
    False -> build (sub64 n 1) (C69 acc)

  count :: Big -> I64 -> I64
  count b k = case b of
    C69 t -> count t (add64 k 1)
    C0 -> k
    _ -> sub64 0 1

  rank :: Big -> I64
  rank b = case b of
    C0 -> 0
    C63 -> 63
    C64 -> 64
    C68 -> 68
    _ -> 99

  main :: Unit -> Int
  main u = case eq64 (count (build {n} C0) 0) {n} of
    True -> case eq64 (add64 (add64 (rank C63) (rank C64)) (add64 (rank C68) (rank (C69 C1)))) 294 of
      True -> 0
      False -> 2
    False -> 1
"#
    )
}

/// Escaped constructors match correctly, and each heap cell pays exactly
/// its descriptor word: a one-field `C69` cell is 24 bytes (field, header,
/// descriptor), and nothing else allocates per cell.
#[test]
fn escaped_constructors_match_and_carry_one_descriptor_word() {
    let (dir_a, small) = built("esc_small", &escaped_program(1000));
    let (dir_b, large) = built("esc_large", &escaped_program(3000));
    let (a, b) = (stats_of(&small), stats_of(&large));
    let delta = |name: &str| field(&b, name) - field(&a, name);
    assert_eq!(delta("header-blocks"), 2000, "{a}\n{b}");
    assert_eq!(delta("header-bytes"), 2000 * 24, "{a}\n{b}");
    assert_eq!(delta("blocks"), 2000, "{a}\n{b}");
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
    assert_eq!(out.status.code(), Some(EXIT_EXHAUSTED), "{out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    // `<n>` is the live heap the refused allocation would have made: just
    // past the cap, since the growing list keeps every cell live.
    let n: u64 = stderr
        .lines()
        .find_map(|l| l.strip_prefix("cinch: exhausted: heap ("))
        .and_then(|r| r.strip_suffix(" bytes)"))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no exhausted line in {stderr}"));
    assert!(n > 64 << 20 && n < 65 << 20, "{stderr}");
    assert!(start.elapsed() < Duration::from_secs(30));
    common::cleanup(&dir);
}

/// Builds and drops a 1000-cell list 4000 times: about 32 KiB live at a
/// time, over 100 MiB allocated over the run.
const CHURN: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = case n == 0 of
    True -> acc
    False -> build (n - 1) (Cons n acc)

  churn :: Int -> Int
  churn k = case k == 0 of
    True -> 0
    False -> case length (build 1000 Nil) == 1000 of
      True -> churn (k - 1)
      False -> 1

  main :: Unit -> Int
  main u = churn 4000
"#;

/// The cap bounds the live heap, not the lifetime total: a program that
/// frees every list it is done with runs to completion under a cap far
/// below what it allocates in total.
#[test]
fn the_cap_bounds_live_bytes_not_the_lifetime_total() {
    let (dir, exe) = built("alloc_churn", CHURN);
    let out = run_with(&exe, &[("CINCH_HEAP_LIMIT", "16M"), ("CINCH_ALLOC_STATS", "1")]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    let line = stats_line(&out).expect("cinch-alloc line");
    assert!(field(&line, "bytes") > 64 << 20, "{line}");
    assert!(field(&line, "peak") <= 16 << 20, "{line}");
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

/// A program whose `build` returns a structure over every kind of word a
/// pointer map distinguishes: constructor copies at three class vectors
/// (`List Shape`, `List (Nat -> Nat)`, the pair of them), raw `U8`
/// fields, boxed and immediate `Nat`s, `Bytes` buffers, a partially
/// applied primitive and a lambda capturing a boxed `Nat`. Every value is
/// used once, so no block is shared: nothing dups yet. `main` hands
/// control to the C probe, which calls `build` through its symbol and,
/// as `RC_PROBE` says, keeps the result, drops it once, or drops it twice.
const DROP_PROGRAM: &str = r#"
module Main where
  import Data.List

  data Shape where
    Leaf :: Shape
    Node :: Shape -> U8 -> Nat -> Bytes -> Shape -> Shape

  foreign import ccall "cinch.runtime.bytesNew" bytesNew :: Nat -> Bytes
  foreign import ccall "cinch.runtime.natAdd" natAdd :: Nat -> Nat -> Nat
  foreign import ccall "cinch.runtime.natMul" natMul :: Nat -> Nat -> Nat
  foreign import ccall "rc_probe" rcProbe :: I64 ->{Foreign Probe, IO} I64

  shapes :: Unit -> List Shape
  shapes u =
    Cons (Node Leaf 7 18446744073709551616 (bytesNew 16) (Node Leaf 9 3 (bytesNew 4) Leaf)) (Cons Leaf Nil)

  adders :: Unit -> List (Nat -> Nat)
  adders u =
    let b = natMul 4294967296 8589934592 in
    Cons (natAdd 36893488147419103232) (Cons (\x -> natAdd x b) Nil)

  build :: Unit -> (List Shape, List (Nat -> Nat))
  build u = (shapes u, adders u)

  main :: Unit ->{Foreign Probe, IO} Int
  main u = if rcProbe 0 == 0 then 0 else 1
"#;

const DROP_PROBE: &str = r#"
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#ifdef __APPLE__
#define UNIT(s) "_" s
#else
#define UNIT(s) s
#endif
extern int64_t cinch_build(int64_t) __asm__(UNIT("Main.build"));
extern void cinch_drop(int64_t);
int64_t rc_probe(int64_t unused) {
  (void)unused;
  const char *mode = getenv("RC_PROBE");
  int64_t w = cinch_build(0);
  if (mode && strcmp(mode, "drop") == 0) cinch_drop(w);
  if (mode && strcmp(mode, "twice") == 0) { cinch_drop(w); cinch_drop(w); }
  return 0;
}
"#;

/// The leak check's `leaked=` count from a run's stderr.
fn leaked(out: &Output) -> u64 {
    let stderr = String::from_utf8_lossy(&out.stderr);
    let line = stderr.lines().find(|l| l.starts_with("cinch-leak:")).unwrap_or_else(|| panic!("no leak line: {stderr}"));
    field(line, "leaked")
}

/// The emitted layout table drives the runtime's release of a real
/// program's structure: dropped through `cinch_drop`, every block `build`
/// allocated is freed (the live count returns to the immortal baseline,
/// so the leak check passes), kept, the check reports exactly those
/// blocks and exits 74, and a second drop is caught by the debug mode's
/// header check. A map that missed a counted word would leak it; one that
/// counted a raw word would dereference a non-pointer.
#[test]
fn a_compiled_structure_drops_back_to_the_immortal_baseline() {
    let dir = common::project("rc_drop", &[]);
    let c = dir.join("probe.c");
    let o = dir.join("probe.o");
    std::fs::write(&c, DROP_PROBE).expect("write probe");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = Command::new(&clang).arg("-c").arg(&c).arg("-o").arg(&o).status().expect("run clang");
    assert!(status.success(), "the probe did not compile");
    let exe = dir.join("prog");
    let o = o.to_string_lossy().into_owned();
    let src = DROP_PROGRAM.to_string();
    let out_path = exe.clone();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || cinch::driver::build_source_objects(&src, &[&o], &out_path))
        .expect("spawn build")
        .join()
        .expect("build thread")
        .unwrap_or_else(|e| panic!("build: {e}"));

    // Kept: the structure is what is live. Two `Cons` and two `Node`
    // cells, two buffers and a boxed `Nat` under `shapes`; two `Cons`
    // cells, two closures and two boxed `Nat`s under `adders`; the pair.
    let kept = run_with(&exe, &[("CINCH_LEAK_CHECK", "1"), ("RC_PROBE", "keep")]);
    assert_eq!(kept.status.code(), Some(74), "{kept:?}");
    assert_eq!(leaked(&kept), 14, "{kept:?}");

    let dropped = run_with(&exe, &[("CINCH_LEAK_CHECK", "1"), ("RC_PROBE", "drop")]);
    assert_eq!(dropped.status.code(), Some(0), "{dropped:?}");
    assert_eq!(leaked(&dropped), 0, "{dropped:?}");

    let debug = run_with(&exe, &[("CINCH_LEAK_CHECK", "1"), ("CINCH_RC_DEBUG", "1"), ("RC_PROBE", "drop")]);
    assert_eq!(debug.status.code(), Some(0), "{debug:?}");
    assert_eq!(leaked(&debug), 0, "{debug:?}");

    let twice = run_with(&exe, &[("CINCH_RC_DEBUG", "1"), ("RC_PROBE", "twice")]);
    assert_eq!(twice.status.code(), Some(71), "{twice:?}");
    assert!(
        String::from_utf8_lossy(&twice.stderr).contains("compiler bug: reference count operation on a freed or corrupt block"),
        "{twice:?}"
    );

    // Without the variable the check is silent and the exit code is the
    // program's.
    let quiet = run_with(&exe, &[("RC_PROBE", "keep")]);
    assert_eq!(quiet.status.code(), Some(0), "{quiet:?}");
    assert!(quiet.stderr.is_empty(), "{quiet:?}");
    common::cleanup(&dir);
}
