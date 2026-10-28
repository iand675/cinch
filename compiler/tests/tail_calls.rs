//! The constant-stack tail-call proof (spec requirement mapping row
//! 122, `surf#9-tail-constant-stack`): tail recursion runs in constant
//! stack, self and mutual, over REAL programs compiled and run through
//! the same driver the CLI uses.
//!
//! The proof is depth: each program drives its recursion 1,000,001
//! calls deep. Without the loop-header lowering (a saturated self tail
//! call jumps back to the function's loop header) and the `musttail`
//! lowering (mutual tail calls between signature-identical units), each
//! call would push a frame and a million frames would exhaust any
//! process stack; the control case pins exactly that, over the same
//! depth without tail calls. The tail marks name the lowering that is
//! actually in force (`core.rs::tail_report`, the `tail_calls` manifest
//! section's source): a silently falling back group would die like the
//! control, so the marks are asserted beside the runs.

use cinch::core::tail_report;
use cinch::driver::{check_sources, run_source};

/// Run a `Main` through the driver; the exit code is the program's
/// `main :: Int` result (0 on success).
fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// The `mark` of the recursive group containing `member` in the checked
/// program's tail report.
fn group_mark(name: &str, src: &str, member: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    tail_report(&program)
        .into_iter()
        .find(|g| g.members.iter().any(|m| m == member))
        .map(|g| g.mark)
        .unwrap_or_else(|| panic!("{name}: no recursive group contains {member}"))
}

/// Self tail recursion, 1,000,001 deep (the loop-header self jump): the
/// countdown sums 1..=1_000_000 and the group's tail mark is the
/// lowering's own word, `marked`.
#[test]
fn self_tail_recursion_runs_a_million_deep_in_constant_stack() {
    let src = r#"
module Main where
  sum-to :: Int -> Int -> Int
  sum-to acc n = if n == 0 then acc else sum-to (acc + n) (n - 1)

  main :: Int
  main = if sum-to 0 1000000 == 500000500000 then 0 else 1
"#;
    expect_ok("self-tail-1m", src, 0);
    assert_eq!(
        group_mark("self-tail-1m", src, "Main.sum-to"),
        "marked",
        "the self tail group must be tail-marked (surface grammar local amendments section 9)"
    );
}

/// Mutual tail recursion, 1,000,001 deep (`musttail` ping-pong between
/// signature-identical units): the odd depth lands on the `pong` base
/// case and the group's tail mark is `marked`.
#[test]
fn mutual_tail_recursion_runs_a_million_deep_in_constant_stack() {
    let src = r#"
module Main where
  ping :: Int -> Int
  ping n = if n == 0 then 0 else pong (n - 1)

  pong :: Int -> Int
  pong n = if n == 0 then 1 else ping (n - 1)

  main :: Int
  main = if ping 1000001 == 1 then 0 else 1
"#;
    expect_ok("mutual-tail-1m", src, 0);
    assert_eq!(
        group_mark("mutual-tail-1m", src, "Main.ping"),
        "marked",
        "the mutual tail group must be tail-marked (surface grammar local amendments section 9)"
    );
}

/// The control that makes the depth honest: the same 1,000,000-deep
/// descent with NO tail call to lower. Every call pushes a frame, the
/// frames exhaust the process stack, and the binary dies on the signal
/// (the driver reports a signal death as exit 1). A green run here
/// would mean the two tests above proved nothing about depth.
#[test]
fn deep_recursion_without_a_tail_call_exhausts_the_stack() {
    let src = r#"
module Main where
  deep :: Int -> Int
  deep n = if n == 0 then 0 else 1 + deep (n - 1)

  main :: Int
  main = if deep 1000000 == 1000000 then 0 else 1
"#;
    match run_source(src) {
        Ok(0) => panic!(
            "deep-recursion-control: the non-tail descent ran 1,000,000 deep to completion: \
             the stack-overflow premise of the constant-stack proof is gone"
        ),
        Ok(code) => assert_eq!(
            code, 1,
            "deep-recursion-control: expected the signal death the driver reports as exit 1, got {code}"
        ),
        Err(e) => panic!("deep-recursion-control: compilation failed: {e}"),
    }
}
