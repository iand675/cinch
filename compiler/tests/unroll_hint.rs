//! `#[unroll(N)]`: the advisory loop hint of attribute annotations
//! specification section 5, lowered to LLVM loop metadata on the loop a
//! definition's self tail recursion becomes.
//!
//! Every assertion here is IR observation of the real compiler on real
//! source. The metadata's shape is load bearing: a loop node must be
//! `distinct` and must name itself, because LLVM compares loop ids by node
//! identity and finds a node's own id by its first operand. A definition
//! with no self tail call has no back edge, so its hint produces no
//! metadata at all rather than an error.

mod common;

use cinch::driver::check_sources;
use common::source_frontend::run_source;

/// A definition whose self tail call becomes a loop, plus a `main` that
/// reaches it (code generation emits the units the entry point reaches).
/// The result depends on the loop's arithmetic, so a hint that changed the
/// program's values would change the exit status.
///
/// `attr` is the hint as written, placed on the function declaration it
/// annotates; an empty string is the unhinted program.
fn tail(attr: &str) -> String {
    format!(
        "\
module Main where
  {attr}
  sum :: Nat -> Nat -> Nat
  sum acc n = if n == 0 then acc else sum (acc + n) (n - 1)

  main :: Int
  main = case sum 0 100 of
    total -> if total == 5050 then 0 else 1
"
    )
}

/// A definition with no self tail call: its body has no back edge, so the
/// hint has nowhere to attach.
fn straight(attr: &str) -> String {
    format!(
        "\
module Main where
  {attr}
  double :: Nat -> Nat
  double x = x + x

  main :: Int
  main = case double 21 of
    total -> if total == 42 then 0 else 1
"
    )
}

/// The IR of one checked source.
fn ir_of(name: &str, src: &str) -> String {
    let program = check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The body of one unit's `define` (its lines between `{` and `}`).
fn body_of(ir: &str, unit: &str) -> String {
    let quoted = format!("@\"{unit}\"(");
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if l.starts_with("define ") && l.contains(&quoted) {
            let mut body = String::new();
            for l in lines.by_ref() {
                if l == "}" {
                    return body;
                }
                body.push_str(l);
                body.push('\n');
            }
        }
    }
    panic!("{unit}: no define in {ir}")
}

/// The loop node and property node the `!llvm.loop` attachment in `unit`'s
/// body names. Asserts the loop node is `distinct` and self-referential,
/// which is the form LLVM's verifier requires.
fn loop_policy(ir: &str, unit: &str) -> (String, String) {
    let body = body_of(ir, unit);
    let reference = body
        .lines()
        .find_map(|l| {
            let (_, rest) = l.split_once("!llvm.loop !")?;
            let id: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if id.is_empty() {
                None
            } else {
                Some(id)
            }
        })
        .unwrap_or_else(|| panic!("{unit}: no `!llvm.loop` attachment in {body}"));
    // The loop node: `!N = distinct !{!N, !M}`.
    let prefix = format!("!{reference} = distinct !{{!{reference}, !");
    let node = ir
        .lines()
        .find(|l| l.starts_with(&prefix) && l.ends_with('}'))
        .unwrap_or_else(|| {
            panic!("!{reference} is not a distinct self-referential loop node in {ir}")
        });
    let property = node[prefix.len()..node.len() - 1].to_string();
    let property_line = format!("!{property} = ");
    let text = ir
        .lines()
        .find_map(|l| l.strip_prefix(&property_line))
        .unwrap_or_else(|| panic!("!{property} is not defined in {ir}"));
    (node.to_string(), text.to_string())
}

/// Assert that checking the program fails with a message containing every
/// one of `needles`.
fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected an error, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for needle in needles {
                assert!(
                    msg.contains(needle),
                    "{name}: error {msg:?} does not contain {needle:?}"
                );
            }
        }
    }
}

// ------------------------------------------------- a count reaches LLVM

#[test]
fn unroll_count_reaches_the_loop_node() {
    let ir = ir_of("unroll-count", &tail("#[unroll(4)]"));
    let (_, property) = loop_policy(&ir, "Main.sum");
    assert_eq!(property, "!{!\"llvm.loop.unroll.count\", i32 4}", "{ir}");
}

// --------------------------------------- the two spellings of disabling

#[test]
fn unroll_disable_and_zero_both_disable() {
    for (name, attr) in [
        ("unroll-disable", "#[unroll(disable)]"),
        ("unroll-zero", "#[unroll(0)]"),
    ] {
        let ir = ir_of(name, &tail(attr));
        let (_, property) = loop_policy(&ir, "Main.sum");
        assert_eq!(property, "!{!\"llvm.loop.unroll.disable\"}", "{name}: {ir}");
        // Disabling is not the count form with a zero count: the property
        // node carries no count at all.
        assert!(!ir.contains("llvm.loop.unroll.count"), "{name}: {ir}");
    }
}

// ------------------------------------------------------- the no-loop case

#[test]
fn unroll_without_a_loop_is_accepted_and_emits_nothing() {
    let ir = ir_of("unroll-no-loop", &straight("#[unroll(4)]"));
    // Advisory, never an error: the definition is emitted as usual.
    assert!(!body_of(&ir, "Main.double").is_empty(), "{ir}");
    assert!(
        !ir.contains("llvm.loop"),
        "a definition with no back edge has nowhere to put the metadata: {ir}"
    );
}

// -------------------------------------------------------- the refusal case

#[test]
fn a_non_hint_argument_is_refused_naming_the_accepted_forms() {
    expect_err(
        "unroll-bad-arg",
        &straight("#[unroll(abc)]"),
        &[
            "`abc` is not an unroll hint",
            "natural number",
            "`disable`",
            "`#[unroll(4)]`",
        ],
    );
    // Two `unroll` attributes on one declaration are the section 1.3
    // duplicate, like any repeated attribute name.
    expect_err(
        "unroll-duplicate",
        &straight("#[unroll(2)]\n  #[unroll(disable)]"),
        &["two `unroll` attributes on one construct"],
    );
    // A hint may ride the signature and the equation, but the two are one
    // function's one hint family, so writing it twice is still a
    // duplicate.
    expect_err(
        "unroll-duplicate-across-declarations",
        "\
module Main where
  #[unroll(2)]
  sum :: Nat -> Nat -> Nat
  #[unroll(disable)]
  sum acc n = if n == 0 then acc else sum (acc + n) (n - 1)

  main :: Int
  main = 0
",
        &["carries two unroll hints", "one hint family"],
    );
}

// ------------------------------------------------- behaviour is unchanged

#[test]
fn the_hint_never_changes_the_result() {
    let plain = run_source(&tail("")).unwrap_or_else(|e| panic!("unhinted: {e}"));
    for attr in ["#[unroll(4)]", "#[unroll(disable)]"] {
        let hinted = run_source(&tail(attr)).unwrap_or_else(|e| panic!("{attr}: {e}"));
        assert_eq!(hinted, plain, "{attr} changed the exit status");
    }
}
