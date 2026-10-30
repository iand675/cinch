//! The constant-stack tail-call proof (spec requirement mapping row
//! 122, `surf#9-tail-constant-stack`): tail recursion runs in constant
//! stack, self and mutual, over REAL programs compiled through the same
//! driver the CLI uses.
//!
//! The proof is depth: each program drives its recursion 1,000,001
//! calls deep. Without the loop-header lowering (a saturated self tail
//! call jumps back to the function's loop header) and the `musttail`
//! lowering (mutual tail calls between units of one tail signature,
//! widened across arities by `core.rs::tail_widths`), each call would
//! push a frame and a million frames would exhaust the stack; the
//! control case pins exactly that, over the same depth without tail
//! calls. The programs run on an 8 MiB stack (`CINCH_STACK_SIZE=8M`, the
//! platform's main-thread default): the runtime's own 1 GiB reservation
//! would hold a million small frames and prove nothing. The tail marks
//! name the lowering that is actually in force (`core.rs::tail_report`,
//! the `tail_calls` manifest section's source): a silently falling back
//! group would die like the control, so the marks are asserted beside
//! the runs.

use cinch::core::tail_report;
use cinch::driver::{check_sources, driver, Mode};

/// Build a `Main` through the driver and run it on an 8 MiB stack:
/// `Some(code)` is the program's `main :: Int` result, `None` a death on
/// a signal.
fn run_on_8mib(name: &str, src: &str) -> Option<i32> {
    let dir = std::env::temp_dir().join(format!("cinch_tail_{name}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    let entry = dir.join("Main.cinch");
    std::fs::write(&entry, src).expect("write Main.cinch");
    let exe = dir.join("Main");
    if let Err(e) = driver(Mode::Build, &entry, Some(&exe)) {
        panic!("{name}: compilation failed: {e}");
    }
    let status = std::process::Command::new(&exe)
        .env("CINCH_STACK_SIZE", "8M")
        .env("CINCH_HEAP_LIMIT", "2G")
        .status()
        .unwrap_or_else(|e| panic!("{name}: run {}: {e}", exe.display()));
    let _ = std::fs::remove_dir_all(&dir);
    status.code()
}

/// Run a `Main` on the 8 MiB stack; the exit code is the program's
/// `main :: Int` result (0 on success).
fn expect_ok(name: &str, src: &str, expected: i32) {
    assert_eq!(run_on_8mib(name, src), Some(expected), "{name}: expected exit {expected}");
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

/// Mutual tail recursion across arities, 1,000,001 deep: `walk` takes
/// two words and `step` one, so `musttail` is legal only because the
/// group lowers under one widened signature (`step`'s body runs in a
/// two-word worker; `core.rs::tail_widths`). This is the shape of
/// `Text.Lex`'s layout loop, whose depth is the token count.
#[test]
fn mixed_arity_mutual_tail_recursion_runs_a_million_deep_in_constant_stack() {
    let src = r#"
module Main where
  walk :: Int -> Int -> Int
  walk n acc = if n == 0 then acc else step (n - 1)

  step :: Int -> Int
  step n = if n == 0 then 7 else walk n 0

  main :: Int
  main = if walk 1000001 5 == 7 then 0 else 1
"#;
    expect_ok("mixed-arity-tail-1m", src, 0);
    assert_eq!(
        group_mark("mixed-arity-tail-1m", src, "Main.step"),
        "marked",
        "the mixed-arity tail group must be tail-marked (surface grammar local amendments section 9)"
    );
}

/// The control that makes the depth honest: the same 1,000,000-deep
/// descent with NO tail call to lower. Every call pushes a frame, the
/// frames exhaust the 8 MiB stack, and the binary takes the runtime's
/// stack `Exhausted` exit (73, `compiler/runtime/src/stack.rs`). A green
/// run here would mean the tests above proved nothing about depth.
#[test]
fn deep_recursion_without_a_tail_call_exhausts_the_stack() {
    let src = r#"
module Main where
  deep :: Int -> Int
  deep n = if n == 0 then 0 else 1 + deep (n - 1)

  main :: Int
  main = if deep 1000000 == 1000000 then 0 else 1
"#;
    match run_on_8mib("deep-recursion-control", src) {
        Some(0) => panic!(
            "deep-recursion-control: the non-tail descent ran 1,000,000 deep to completion: \
             the stack-overflow premise of the constant-stack proof is gone"
        ),
        Some(73) => {}
        Some(code) => panic!("deep-recursion-control: expected the Exhausted exit 73, got exit {code}"),
        None => panic!("deep-recursion-control: died on a signal instead of the Exhausted exit"),
    }
}
