//! Resource exhaustion is the defined `Exhausted` outcome, never a crash
//! (`design/no-faults.md` sections 4 and 7): running out of the stack, of
//! the heap, or of threads exits with code 73 and one stderr line naming
//! the resource. Every case is a real program built through the driver
//! and run as its own process, under a small `CINCH_HEAP_LIMIT` (and a
//! small `CINCH_STACK_SIZE` where the stack is the point), so a broken
//! guard fails the test instead of exhausting the machine.

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// The `Exhausted` exit code (`EXIT_EXHAUSTED` in the runtime).
const EXIT_EXHAUSTED: i32 = 73;

fn built(tag: &str, src: &str) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &[("Main.cinch", src)]);
    let exe = common::build(&dir, "Main.cinch", "prog").unwrap_or_else(|e| panic!("{tag}: build: {e}"));
    (dir, exe)
}

/// Run `exe` under a 256 MiB heap cap plus `env`.
fn run_with(exe: &Path, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env_remove("CINCH_ALLOC_STATS")
        .env_remove("CINCH_STACK_SIZE")
        .env("CINCH_HEAP_LIMIT", "256M");
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().expect("spawn program")
}

/// Assert `out` is the `Exhausted` exit whose only stderr line is `line`.
fn assert_exhausted(out: &Output, line: &str) {
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(EXIT_EXHAUSTED),
        "expected the Exhausted exit ({line}), got {:?}; stderr:\n{stderr}",
        out.status
    );
    assert_eq!(stderr.trim_end(), line, "{stderr}");
}

/// Non-tail recursion: each call keeps its frame while `1 + _` waits.
const DEEP: &str = r#"
  deep :: Int -> Int
  deep n = if n == 0 then 0 else 1 + deep (n - 1)
"#;

fn deep_main(depth: &str) -> String {
    format!(
        "module Main where\n{DEEP}\n  main :: Int\n  main = if deep {depth} == {depth} then 0 else 1\n"
    )
}

#[test]
fn unbounded_non_tail_recursion_exits_exhausted_stack() {
    // A trillion frames: no stack holds them, so the descent runs until
    // the program thread's guard and stops there.
    let (dir, exe) = built("exh_stack", &deep_main("1000000000000"));
    let out = run_with(&exe, &[("CINCH_STACK_SIZE", "1M")]);
    assert_exhausted(&out, "cinch: exhausted: stack");
    // A larger reservation ends the same way, only later.
    let out = run_with(&exe, &[("CINCH_STACK_SIZE", "64M")]);
    assert_exhausted(&out, "cinch: exhausted: stack");
    common::cleanup(&dir);
}

#[test]
fn deep_recursion_within_the_limit_still_succeeds() {
    // 100,000 frames fit the 1 GiB default with room to spare, and the
    // same descent past a small stack is the Exhausted exit: the depth is
    // real, and the guard costs nothing until it is reached.
    let (dir, exe) = built("exh_within", &deep_main("100000"));
    let out = run_with(&exe, &[]);
    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
    let out = run_with(&exe, &[("CINCH_STACK_SIZE", "1M")]);
    assert_exhausted(&out, "cinch: exhausted: stack");
    common::cleanup(&dir);
}

#[test]
fn deep_recursion_inside_a_task_exits_exhausted_stack() {
    // The overflow happens on the task's own thread (its 8 MiB stack, not
    // the program thread's), which is guarded the same way.
    let src = format!(
        r#"
module Main where
{DEEP}
  work :: forall r. using (io :: Io r) => Unit ->{{Async r, Cancel r, Diverge}} Result LoadError Int
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok (deep 1000000000000))
    joinAllOrCancel s h

  main :: Unit ->{{Async r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> 0
    Err e -> 2
"#
    );
    let (dir, exe) = built("exh_task", &src);
    let out = run_with(&exe, &[]);
    assert_exhausted(&out, "cinch: exhausted: stack");
    common::cleanup(&dir);
}

#[test]
fn unbounded_allocation_exits_exhausted_heap() {
    let src = r#"
module Main where
  import Data.List

  grow :: Int -> List Int -> Int
  grow n acc = grow (n + 1) (Cons n acc)

  main :: Unit -> Int
  main u = grow 0 Nil
"#;
    let (dir, exe) = built("exh_heap", src);
    let out = run_with(&exe, &[("CINCH_HEAP_LIMIT", "32M")]);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(EXIT_EXHAUSTED), "{out:?}");
    // One line, reporting the live heap the refused allocation would
    // have made: just past the cap.
    let n: u64 = stderr
        .trim_end()
        .strip_prefix("cinch: exhausted: heap (")
        .and_then(|r| r.strip_suffix(" bytes)"))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("not one exhausted-heap line: {stderr}"));
    assert!(n > 32 << 20 && n < 33 << 20, "{stderr}");
    common::cleanup(&dir);
}
