//! P20: the safety assertion attributes acceptance matrix (attribute
//! annotations specification section 10;
//! design/safety-assertion-attributes.md; decisions D111-D114): every
//! assertion has a positive and a negative case whose error names the
//! rule and the offending site, the contract facts pin interface
//! identity both ways, the `jpl` profile enforces the rule numbers and
//! reports its summary, the manifest records the assertions summary,
//! and the `Result` must-use default holds.

use cinch::check::{
    check_program_multi_full, Diagnostic, JplSummary, SafetyReport, StagingCx,
};
use cinch::core::{Program, TargetProfile};
use cinch::driver::{
    check_sources, check_sources_ifaces, driver_with_profile, parse_prelude, parse_source,
    run_source, Mode, StageMode,
};
use cinch::interface::Iface;
use cinch::Result;
use std::collections::HashMap;
use std::path::PathBuf;

/// Check `src` under `profile`, returning the elaborated program, the
/// interfaces (the family first), the safety report, and the non-fatal
/// diagnostics.
fn check_with(
    src: &str,
    profile: &TargetProfile,
) -> Result<(Program, Vec<Iface>, SafetyReport, Vec<Diagnostic>)> {
    let prelude = parse_prelude(&[])?;
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (program, ifaces, report, diags) =
        check_program_multi_full(&prelude.modules, &user.modules, &hashes, &scx, &[], profile)?;
    Ok((program, ifaces, report.safety, diags))
}

/// The safety report of a checked source under the native profile.
fn check_report(src: &str) -> SafetyReport {
    check_with(src, &TargetProfile::native())
        .unwrap_or_else(|e| panic!("check failed: {e}"))
        .2
}

/// The safety report of a checked source under the `jpl` profile.
fn check_jpl_report(src: &str) -> Result<SafetyReport> {
    Ok(check_with(src, &TargetProfile::jpl())?.2)
}

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needles:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

fn expect_err_jpl(name: &str, src: &str, needles: &[&str]) {
    match check_jpl_report(src) {
        Ok(_) => panic!("{name}: expected an error containing {needles:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

/// Assert the report carries `definition`'s `attribute` row with a
/// result mentioning `result`.
fn assert_row(report: &SafetyReport, definition: &str, attribute: &str, result: &str) {
    let hit = report.assertions.iter().find(|a| {
        a.definition == definition && a.attribute == attribute
    });
    let Some(row) = hit else {
        panic!(
            "no row for {definition} {attribute}; rows: {:?}",
            report
                .assertions
                .iter()
                .map(|a| (&a.definition, &a.attribute))
                .collect::<Vec<_>>()
        )
    };
    assert!(
        row.result.contains(result),
        "{definition} {attribute}: result {:?} does not mention {result:?}",
        row.result
    );
}

/// A temp project directory holding the given `(file, source)` pairs.
fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("cinch_safety_{}_{}", std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

// ------------------------------------------- `#[recursion ...]` (rule 1)

#[test]
fn recursion_none_accepts_a_non_recursive_definition() {
    let src = r#"
module Main where
  #[recursion("none")]
  double :: Int -> Int
  double x = x + x

  main :: Int
  main = if double 2 == 4 then 0 else 1
"#;
    expect_ok("recursion-none-ok", src, 0);
    let report = check_report(src);
    assert_row(&report, "Main.double", "recursion=none", "verified (no recursive calls)");
}

#[test]
fn recursion_none_names_the_recursive_call() {
    expect_err(
        "recursion-none-bad",
        r#"
module Main where
  #[recursion("none")]
  loopy :: Int -> Int
  loopy n = if n == 0 then 0 else loopy (n - 1)

  main :: Int
  main = loopy 2
"#,
        &[
            "`#[recursion \"none\"]` on Main.loopy",
            "no-recursion rule (JPL rule 1)",
            "the recursive call at this site",
        ],
    );
}

#[test]
fn recursion_tail_covers_self_and_mutual_groups() {
    let src = r#"
module Main where
  #[recursion("tail")]
  sum-to :: Int -> Int -> Int
  sum-to acc n = if n == 0 then acc else sum-to (acc + n) (n - 1)

  #[recursion("tail")]
  ping :: Int -> Int
  ping n = if n == 0 then 0 else pong (n - 1)

  #[recursion("tail")]
  pong :: Int -> Int
  pong n = if n == 0 then 1 else ping (n - 1)

  main :: Int
  main = if sum-to 0 3 == 6 && ping 3 == 1 then 0 else 1
"#;
    expect_ok("recursion-tail-ok", src, 0);
    let report = check_report(src);
    for d in ["Main.sum-to", "Main.ping", "Main.pong"] {
        assert_row(&report, d, "recursion=tail", "verified (D104 tail marks)");
    }
}

#[test]
fn recursion_tail_names_the_non_tail_call() {
    expect_err(
        "recursion-tail-bad",
        r#"
module Main where
  #[recursion("tail")]
  notail :: Int -> Int
  notail n = if n == 0 then 0 else 1 + notail (n - 1)

  main :: Int
  main = notail 2
"#,
        &[
            "`#[recursion \"tail\"]` on Main.notail",
            "tail-recursion rule (JPL rule 1",
            "the call to `Main.notail` at this site is not in tail position",
        ],
    );
}

/// `bounded(x)` proves the depth and the report says which method did
/// it (decision D113): a structural descent proves the bound named by
/// the argument (its size), the literal counter ladder proves a numeric
/// depth conservatively, and a claim over a bound that does not match
/// is refused.
#[test]
fn bounded_claims_name_their_proof_method_or_are_refused() {
    // Structural depth: the descent on the decreasing argument, cited
    // by the argument's name or its size index.
    let structural = r#"
module Main where
  #[recursion("bounded(xs)")]
  len :: List Int -> Int
  len xs = case xs of
    Nil -> 0
    Cons x rest -> 1 + len rest

  #[recursion("bounded(n)")]
  count :: Nat -> Nat
  count n = if n == 0 then 0 else 1 + count (n - 1)

  main :: Int
  main = 0
"#;
    let report = check_report(structural);
    assert_row(
        &report,
        "Main.len",
        "recursion=bounded(xs)",
        "structural (depth <= xs: the size of the decreasing argument)",
    );
    assert_row(
        &report,
        "Main.count",
        "recursion=bounded(n)",
        "structural (depth <= n: the size of the decreasing argument)",
    );

    // `bounded(3)` accepts a depth of 3 established conservatively (the
    // literal counter ladder), and the report names the method.
    let counter = r#"
module Main where
  #[recursion("bounded(3)")]
  down :: Int -> Int
  down 3 = down 2
  down 2 = down 1
  down 1 = 0
  down _ = 0

  main :: Int
  main = down 3
"#;
    let report = check_report(counter);
    assert_row(
        &report,
        "Main.down",
        "recursion=bounded(3)",
        "call-graph (the literal counter ladder: depth <= 3)",
    );

    // The unprovable is refused, with the refusal named.
    expect_err(
        "bounded-unprovable",
        r#"
module Main where
  #[recursion("bounded(3)")]
  lenL :: List Int -> Int
  lenL (Cons x rest) = 1 + lenL rest
  lenL Nil = 0

  main :: Int
  main = 0
"#,
        &[
            "`#[recursion \"bounded(3)\"]` on Main.lenL",
            "no depth bound is established",
            "refused, never trusted",
        ],
    );
    // A false claim on unbounded recursion refuses.
    expect_err(
        "bounded-unbounded",
        r#"
module Main where
  #[recursion("bounded(3)")]
  up :: Int -> Int
  up n = if n == 0 then 0 else up (n + 1)

  main :: Int
  main = 0
"#,
        &[
            "`#[recursion \"bounded(3)\"]` on Main.up",
            "no depth bound is established",
            "refused, never trusted",
        ],
    );
    // A numeric claim over an established structural bound refuses (a
    // structural bound names the argument's size; it is not a number).
    expect_err(
        "bounded-numeric-over-structural",
        r#"
module Main where
  #[recursion("bounded(3)")]
  count :: Nat -> Nat
  count n = if n == 0 then 0 else 1 + count (n - 1)

  main :: Int
  main = 0
"#,
        &[
            "`#[recursion \"bounded(3)\"]` on Main.count",
            "the established bound is structural (depth <= n: the size of the decreasing argument)",
            "does not bound the depth by 3",
            "refused, never trusted",
        ],
    );
    // A numeric claim above the established depth refuses.
    expect_err(
        "bounded-over-claim",
        r#"
module Main where
  #[recursion("bounded(2)")]
  down :: Int -> Int
  down 3 = down 2
  down 2 = down 1
  down 1 = 0
  down _ = 0

  main :: Int
  main = 0
"#,
        &[
            "`#[recursion \"bounded(2)\"]` on Main.down",
            "the established bound is call-graph (the literal counter ladder: depth <= 3)",
            "does not bound the depth by 2",
        ],
    );
    // A numeric claim over a descent whose depth only its size bounds
    // refuses: a literal index is not a size the descent shrinks (the
    // matrix surfaced this false positive in the size-index detection).
    expect_err(
        "bounded-index-is-not-a-size",
        r#"
module Main where
  data Wrap (n :: Nat) where
    Mk :: Wrap n -> Wrap n

  #[recursion("bounded(3)")]
  depth3 :: Wrap 3 -> Int
  depth3 w = case w of
    Mk inner -> 1 + depth3 inner

  main :: Int
  main = 0
"#,
        &[
            "`#[recursion \"bounded(3)\"]` on Main.depth3",
            "the established bound is structural (depth <= w: the size of the decreasing argument)",
            "does not bound the depth by 3",
        ],
    );
    // A mutual cycle establishes no depth bound.
    expect_err(
        "bounded-mutual",
        r#"
module Main where
  #[recursion("bounded(3)")]
  ping :: Int -> Int
  ping n = if n == 0 then 0 else pong (n - 1)

  #[recursion("bounded(3)")]
  pong :: Int -> Int
  pong n = if n == 0 then 1 else ping (n - 1)

  main :: Int
  main = 0
"#,
        &["`#[recursion \"bounded(3)\"]` on Main.ping", "is mutual", "mutual cycle"],
    );
}

// -------------------------------------------- `#[stack ...]` (rule 1)

#[test]
fn stack_constant_implies_tail_or_non_recursion() {
    let src = r#"
module Main where
  #[stack("constant")]
  sum-to :: Int -> Int -> Int
  sum-to acc n = if n == 0 then acc else sum-to (acc + n) (n - 1)

  #[stack("constant")]
  double :: Int -> Int
  double x = x + x

  main :: Int
  main = if sum-to 0 3 == 6 && double 2 == 4 then 0 else 1
"#;
    expect_ok("stack-constant-ok", src, 0);
    let report = check_report(src);
    assert_row(&report, "Main.sum-to", "stack=constant", "verified");
    assert_row(&report, "Main.double", "stack=constant", "verified");
}

#[test]
fn stack_constant_names_the_non_tail_call() {
    expect_err(
        "stack-constant-bad",
        r#"
module Main where
  #[stack("constant")]
  notail :: Int -> Int
  notail n = if n == 0 then 0 else 1 + notail (n - 1)

  main :: Int
  main = 0
"#,
        &[
            "`#[stack \"constant\"]` on Main.notail",
            "asserts constant stack (JPL rule 1: tail or non-recursive)",
            "is not in tail position",
        ],
    );
}

#[test]
fn stack_bounded_establishes_or_refuses_the_bound() {
    let src = r#"
module Main where
  #[stack("bounded(xs)")]
  len :: List Int -> Int
  len xs = case xs of
    Nil -> 0
    Cons x rest -> 1 + len rest

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(
        &report,
        "Main.len",
        "stack=bounded(xs)",
        "structural (depth <= xs: the size of the decreasing argument)",
    );
    expect_err(
        "stack-bounded-bad",
        r#"
module Main where
  #[stack("bounded(3)")]
  up :: Int -> Int
  up n = if n == 0 then 0 else up (n + 1)

  main :: Int
  main = 0
"#,
        &[
            "`#[stack \"bounded(3)\"]` on Main.up",
            "no stack bound is established",
            "refused, never trusted",
        ],
    );
}

// ------------------------------- `#[no-alloc]` and `#[alloc "once"]` (rule 3)

#[test]
fn no_alloc_accepts_an_allocation_free_body() {
    let src = r#"
module Main where
  #[no-alloc]
  arith :: Int -> Int
  arith x = x * x + x

  main :: Int
  main = if arith 3 == 12 then 0 else 1
"#;
    expect_ok("no-alloc-ok", src, 0);
    let report = check_report(src);
    assert_row(&report, "Main.arith", "no-alloc", "verified (0 allocation site(s)");
}

/// Every allocation kind the analysis classifies (attribute annotations
/// specification section 10: cons, closures, thunks, `Mut`, strings,
/// `Bytes`, foreign aggregates) names its construction site.
#[test]
fn no_alloc_names_the_constructor() {
    let cases: &[(&str, &str, &str)] = &[
        (
            "no-alloc-cons",
            r#"
module Main where
  #[no-alloc]
  cons1 :: List Nat -> List Nat
  cons1 xs = Cons 1 xs

  main :: Int
  main = 0
"#,
            "the constructor `Cons` is constructed here",
        ),
        (
            "no-alloc-local-fn",
            r#"
module Main where
  #[no-alloc]
  mkAdd :: Int -> Int
  mkAdd x = let g y = x + y in g x

  main :: Int
  main = 0
"#,
            "a closure (a local function) is constructed here",
        ),
        (
            "no-alloc-lambda",
            r#"
module Main where
  #[no-alloc]
  lam :: Int -> Int
  lam x = let g = \y -> y + x in g x

  main :: Int
  main = 0
"#,
            "a closure (a lambda) is constructed here",
        ),
        (
            "no-alloc-thunk",
            r#"
module Main where
  #[no-alloc]
  memo :: Int -> Int
  memo x = y
    where ~y = x + 1

  main :: Int
  main = 0
"#,
            "a `delay` thunk (a memo binding) is constructed here",
        ),
        (
            "no-alloc-mut",
            r#"
module Main where
  #[no-alloc]
  cell :: forall region. Int -> Mut region Int
  cell x = mutBorrow x

  main :: Int
  main = 0
"#,
            "a `Mut` cell (the `mutBorrow` introduction) is constructed here",
        ),
        (
            "no-alloc-bytes",
            r#"
module Main where
  #[no-alloc]
  buf :: Bytes
  buf = [1, 2]

  main :: Int
  main = 0
"#,
            "a list construction (a `Bytes` buffer or cons cells) is constructed here",
        ),
        (
            "no-alloc-foreign-aggregate",
            r#"
module Main where
  #[repr(C)]
  record R where
    a :: U8
    b :: U32

  foreign import ccall "fix_take" take :: R ->{Foreign Fix, IO} Unit

  #[no-alloc]
  consume :: R ->{Foreign Fix, IO} Unit
  consume r = take r

  main :: Int
  main = 0
"#,
            "a foreign aggregate (the call to `take`) is constructed here",
        ),
    ];
    for (name, src, site) in cases {
        expect_err(
            name,
            src,
            &["`#[no-alloc]` on Main.", "no-allocation rule (JPL rule 3)", site],
        );
    }
    // A string literal is NOT an allocation: its data lives in the
    // data section as one shared read-only constant every evaluation
    // references directly (no construction), so a `#[no-alloc]`
    // function may use literals in an ordinary term and in the
    // compile-time body of a `stage` definition the generator compiles
    // alike (the treatment is pinned in
    // compiler/tests/string_literals.rs).
    let report = check_report(
        r#"
module Main where
  #[no-alloc]
  greet :: String
  greet = "hello"

  #[no-alloc]
  stage echo :: Nat -> Nat
  stage echo n = strNat "42"

  generate do
    emit (quoteDecls where
      answer :: Nat
      answer = $(quoteNat (echo 1)))

  main :: Int
  main = int-from-i64 (nat-to-i64 answer)
"#,
    );
    assert_row(
        &report,
        "Main.greet",
        "no-alloc",
        "verified (0 allocation site(s)",
    );
    assert_row(
        &report,
        "Main.echo",
        "no-alloc",
        "verified (0 allocation site(s)",
    );
}

#[test]
fn alloc_once_allows_one_site_and_names_the_violation() {
    let src = r#"
module Main where
  #[alloc("once")]
  wrap :: List Nat -> List Nat
  wrap xs = Cons 1 xs

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(&report, "Main.wrap", "alloc=once", "verified (1 allocation site(s) run per call)");
    expect_err(
        "alloc-once-two-sites",
        r#"
module Main where
  #[alloc("once")]
  two :: List Nat -> List Nat
  two xs = Cons 1 (Cons 2 xs)

  main :: Int
  main = 0
"#,
        &[
            "`#[alloc \"once\"]` on Main.two",
            "asserts the initialization pattern (JPL rule 3)",
            "the constructor `Cons` is constructed here as well",
            "at most 1 allocation site(s) run per call",
        ],
    );
    expect_err(
        "alloc-once-under-closure",
        r#"
module Main where
  #[alloc("once")]
  mk :: Int -> Int
  mk x = let g y = Cons y Nil in 0

  main :: Int
  main = 0
"#,
        &[
            "`#[alloc \"once\"]` on Main.mk",
            "asserts the initialization pattern (JPL rule 3)",
            "is constructed under a closure here",
        ],
    );
}

// ------------------------ `#[max-complexity n]`, `#[max-lines n]`, `#[assert-min n]`

#[test]
fn max_complexity_holds_the_boundary() {
    // Complexity is branches + 1: one `if` is exactly 2.
    let src = r#"
module Main where
  #[max-complexity(2)]
  one :: Int -> Int
  one x = if x == 0 then 1 else 2

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(&report, "Main.one", "complexity<=2", "verified (cyclomatic complexity 2 <= 2)");
    expect_err(
        "max-complexity-bad",
        r#"
module Main where
  #[max-complexity(2)]
  two :: Int -> Int
  two x = if x == 0 then 1 else if x == 1 then 2 else 3

  main :: Int
  main = 0
"#,
        &[
            "`#[max-complexity 2]` on Main.two",
            "bounded-complexity rule (JPL rule 4)",
            "the body's cyclomatic complexity (branches + 1) is 3, above 2",
        ],
    );
}

#[test]
fn max_lines_holds_the_boundary() {
    // The body's syntax spans two source lines: exactly the cap.
    let src = r#"
module Main where
  #[max-lines(2)]
  three :: Int -> Int
  three x = if x == 0 then 1
    else 2

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(&report, "Main.three", "lines<=2", "verified (2 source line(s) <= 2)");
    expect_err(
        "max-lines-bad",
        r#"
module Main where
  #[max-lines(1)]
  three :: Int -> Int
  three x = if x == 0 then 1
    else 2

  main :: Int
  main = 0
"#,
        &[
            "`#[max-lines 1]` on Main.three",
            "small-functions rule (JPL rule 4)",
            "the body spans 2 source lines, above 1",
        ],
    );
}

#[test]
fn assert_min_counts_runtime_decides_not_proof_obligations() {
    let src = r#"
module Main where
  #[assert-min(1)]
  one :: Nat -> Nat
  one x = case decide (x < 5) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(&report, "Main.one", "assert-min=1", "verified (1 runtime `decide` check(s) >= 1)");
    expect_err(
        "assert-min-bad",
        r#"
module Main where
  #[assert-min(2)]
  one :: Nat -> Nat
  one x = case decide (x < 5) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
"#,
        &[
            "`#[assert-min 2]` on Main.one",
            "runtime-assertion rule (JPL rule 5)",
            "the body has 1 runtime `decide` check(s) (proof obligations do not count), below 2",
        ],
    );
    // A folded proof obligation is a compile-time discharge, not a
    // runtime check.
    expect_err(
        "assert-min-proof-obligation",
        r#"
module Main where
  #[assert-min(1)]
  proofonly :: Nat -> Nat
  proofonly x = case decide (2 < 3) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
"#,
        &["`#[assert-min 1]` on Main.proofonly", "proof obligations do not count", "below 1"],
    );
}

// ------------------------------------------------ `#[checked-params]` (rule 7)

#[test]
fn checked_params_accepts_validation_before_use() {
    let src = r#"
module Main where
  #[checked-params]
  validated :: Int -> Int
  validated x = case x of
    0 -> 1
    _ -> x

  #[checked-params]
  decider :: Nat -> Nat
  decider x = case decide (x < 5) of
    Yes _ -> x
    No _ -> 0

  main :: Int
  main = if validated 0 == 1 then 0 else 1
"#;
    expect_ok("checked-params-ok", src, 0);
    let report = check_report(src);
    for d in ["Main.validated", "Main.decider"] {
        assert_row(&report, d, "checked-params", "verified (every parameter validated before first use)");
    }
}

#[test]
fn checked_params_names_the_unchecked_use() {
    expect_err(
        "checked-params-bad",
        r#"
module Main where
  #[checked-params]
  raw :: Int -> Int
  raw x = x + 1

  main :: Int
  main = 0
"#,
        &[
            "`#[checked-params]` on Main.raw",
            "boundary-validation rule (JPL rule 7)",
            "the parameter `x` is used here before any validating check",
        ],
    );
}

// ---------------------------------------------------- `#[no-macros]` (rule 8)

#[test]
fn no_macros_accepts_handwritten_code_and_refuses_expansion() {
    let src = r#"
module Main where
  #[no-macros]
  plain :: Int -> Int
  plain x = x + 1

  main :: Int
  main = if plain 1 == 2 then 0 else 1
"#;
    expect_ok("no-macros-ok", src, 0);
    let report = check_report(src);
    assert_row(&report, "Main.plain", "no-macros", "verified (no macro expansion inside the definition)");
    // A definition the macro expansion produced carries the expansion
    // provenance its assertion refuses.
    expect_err(
        "no-macros-bad",
        r#"
module Main where
  macro mkGen :: Syntax -> Declarations
  macro mkGen arg = quoteDecls where
    #[no-macros]
    gen :: Nat -> Nat
    gen x = x + 1

  generate do
    emit (mkGen!(z))

  main :: Int
  main = 0
"#,
        &[
            "`#[no-macros]` on Main.gen",
            "preprocessor rule (JPL rule 8)",
            "a macro expansion occurred at this site",
        ],
    );
}

// --------------------------------------------------- `#[no-raw-ptr]` (rule 9)

#[test]
fn no_raw_ptr_accepts_pointer_free_code_and_refuses_pointers() {
    let src = r#"
module Main where
  #[no-raw-ptr]
  arith :: Int -> Int
  arith x = x * x

  main :: Int
  main = if arith 3 == 9 then 0 else 1
"#;
    expect_ok("no-raw-ptr-ok", src, 0);
    let report = check_report(src);
    assert_row(&report, "Main.arith", "no-raw-ptr", "verified (no raw pointer or opaque type in the signature");
    expect_err(
        "no-raw-ptr-signature",
        r#"
module Main where
  #[no-raw-ptr]
  ptrFn :: RawPtr U8 -> Nat -> Nat
  ptrFn p x = case decide (x < 5) of
    Yes _ -> 1
    No _ -> 2

  main :: Int
  main = 0
"#,
        &[
            "`#[no-raw-ptr]` on Main.ptrFn",
            "pointer rule (JPL rule 9)",
            "RawPtr appears in the signature",
        ],
    );
    expect_err(
        "no-raw-ptr-opaque",
        r#"
module Main where
  opaque type H :: Type

  #[no-raw-ptr]
  hold :: H -> Int
  hold h = 1

  main :: Int
  main = 0
"#,
        &[
            "`#[no-raw-ptr]` on Main.hold",
            "pointer rule (JPL rule 9)",
            "the opaque type `H` appears in the signature",
        ],
    );
}

// ---------------------------------------------------- `#[deny-warnings]` (rule 10)

#[test]
fn deny_warnings_scopes_warn_hash_error_to_the_definition() {
    let src = r#"
module Main where
  #[deny-warnings]
  clean :: Bool -> Int
  clean a = if a then 1 else 2

  main :: Int
  main = 0
"#;
    let report = check_report(src);
    assert_row(&report, "Main.clean", "deny-warnings", "verified (warnings in scope are compile errors)");
    // The partial weight report (attribute annotations specification
    // section 2.1) is a warning; under the assertion it is an error
    // naming the assertion.
    expect_err(
        "deny-warnings-bad",
        r#"
module Main where
  #[deny-warnings]
  weighted :: Int -> Int
  weighted a = if a == 0 then #[weight(2)] 1 else 2

  main :: Int
  main = 0
"#,
        &["partly annotated with weights", "denied by `#[deny-warnings]`"],
    );
    // Without the assertion the same body only warns: it checks.
    let warned = r#"
module Main where
  weighted :: Int -> Int
  weighted a = if a == 0 then #[weight(2)] 1 else 2

  main :: Int
  main = 0
"#;
    let (_p, _i, _s, diags) =
        check_with(warned, &TargetProfile::native()).expect("warning-only body checks");
    assert!(
        diags.iter().any(|d| d.message.contains("partly annotated")),
        "the partial weight warning must be reported: {:?}",
        diags
    );
}

// --------------------------------------------- contract facts (decision D112)

#[test]
fn contract_facts_hash_assertions_and_must_use_both_ways() {
    let plain = r#"
module Main where
  loop :: Int -> Int -> Int
  loop acc n = if n == 0 then acc else loop (acc + n) (n - 1)

  mightFail :: Int -> Result Int Int
  mightFail x = if x == 0 then Ok 1 else Err 2
"#;
    let asserted = plain.replace(
        "  loop :: Int -> Int -> Int",
        "  #[recursion(\"tail\")]\n  loop :: Int -> Int -> Int",
    );
    let iface_of = |src: &str| -> Iface {
        let (_p, ifaces) = check_sources_ifaces(src, "<test>").expect("check");
        ifaces
            .into_iter()
            .find(|i| i.module == "Main")
            .expect("Main interface")
    };
    let base = iface_of(plain);
    let with = iface_of(&asserted);

    // The satisfied assertion is a contract fact recorded in the
    // interface exactly like `repr` facts.
    assert!(
        base.assertions.is_empty(),
        "no assertion, no fact: {:?}",
        base.assertions
    );
    let fact = with
        .assertions
        .iter()
        .find(|a| a.name == "loop")
        .expect("assertion row for `loop`");
    assert_eq!(fact.facts, vec!["recursion=tail".to_string()]);
    let text = with.to_json();
    assert!(text.contains("\"assertions\""), "{text}");
    assert!(text.contains("recursion=tail"), "{text}");

    // The `must_use` rows of the return-value discipline.
    for iface in [&base, &with] {
        assert!(
            iface.must_use.iter().any(|n| n == "mightFail"),
            "must_use must record the Result-returning value: {:?}",
            iface.must_use
        );
        assert!(iface.to_json().contains("\"must_use\""), "{}", iface.to_json());
    }

    // Interface identity is pinned both ways: the assertion's
    // appearance changes `iface_hash`, and its disappearance returns to
    // the identity it left (the `repr` precedent).
    assert_ne!(
        base.iface_hash(),
        with.iface_hash(),
        "an assertion appearing must change the interface identity"
    );
    let base_again = iface_of(plain);
    let with_again = iface_of(&asserted);
    assert_eq!(
        base.iface_hash(),
        base_again.iface_hash(),
        "the same interface must hash identically"
    );
    assert_eq!(
        with.iface_hash(),
        with_again.iface_hash(),
        "the same interface must hash identically"
    );
}

// ------------------------------------------------------- the `jpl` profile

/// The compliance case for one exported definition under the `jpl`
/// profile: bounded recursion or none, complexity and lines at the rule
/// numbers, and two runtime `decide` checks.
const JPL_CHECKED: &str = r#"
  classify :: Nat -> Nat
  classify x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> 1
      No _ -> 2
    No _ -> 3
"#;

#[test]
fn jpl_profile_reports_the_whole_program_summary() {
    let src = format!(
        r#"
module Main where
{JPL_CHECKED}

  #[recursion("bounded(xs)")]
  walk :: List Nat -> Nat
  walk xs = case xs of
    Nil -> 0
    Cons x rest -> case decide (x < 5) of
      Yes _ -> case decide (x < 3) of
        Yes _ -> x + walk rest
        No _ -> walk rest
      No _ -> walk rest

  #[recursion("bounded(3)")]
  down :: Int -> Int -> Int
  down 3 x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> down 2 x
      No _ -> down 2 x
    No _ -> down 2 x
  down 2 x = down 1 x
  down 1 x = x
  down _ x = x

  main :: Int
  main = case 2 + 2 of
    n -> case decide (n < 5) of
      Yes _ -> case decide (n < 3) of
        Yes _ -> 1
        No _ -> 2
      No _ -> 3
"#
    );
    let report = check_jpl_report(&src).expect("the rule numbers must pass");
    let JplSummary {
        modules,
        definitions,
        assertions,
    } = report.jpl.clone().expect("the profile reports its summary");
    assert_eq!(modules, vec!["Main".to_string()]);
    assert_eq!(definitions, 4);
    assert!(assertions >= definitions, "each definition reports its rules: {assertions}");
    // Every definition reports the mandated set, and the explicit
    // claims report their proof method.
    assert_row(&report, "Main.classify", "jpl recursion", "verified (no recursive calls)");
    assert_row(&report, "Main.classify", "jpl complexity<=10", "verified (cyclomatic complexity 3)");
    assert_row(&report, "Main.classify", "jpl assert-min=2", "verified (2 runtime `decide` checks)");
    assert_row(&report, "Main.classify", "jpl alloc=none", "verified (0 allocation site(s))");
    assert_row(&report, "Main.classify", "jpl no-macros", "verified (no macro expansion");
    assert_row(&report, "Main.classify", "jpl no-raw-ptr", "verified (no raw pointer");
    assert_row(&report, "Main.classify", "jpl deny-warnings", "verified (warnings in scope");
    assert_row(
        &report,
        "Main.walk",
        "recursion=bounded(xs)",
        "structural (depth <= xs: the size of the decreasing argument)",
    );
    assert_row(
        &report,
        "Main.down",
        "recursion=bounded(3)",
        "call-graph (the literal counter ladder: depth <= 3)",
    );
}

#[test]
fn jpl_profile_rejects_each_rule_and_never_looser() {
    // The rule cases include a legal 61-nested-`let` program whose
    // elaboration depth needs more stack than a default test thread
    // provides in the debug build (the shipped driver elaborates on the
    // main thread); run the cases on a wide-stack thread.
    std::thread::Builder::new()
        .stack_size(16 * 1024 * 1024)
        .spawn(jpl_rule_cases)
        .expect("spawn the jpl rule cases")
        .join()
        .expect("the jpl rule cases");
}

/// The `jpl` profile's per-rule rejection cases (wide stack; see the
/// test above).
fn jpl_rule_cases() {
    // Rule 4: complexity 10.
    expect_err_jpl(
        "jpl-complexity",
        r#"
module Main where
  busy :: Int -> Int
  busy x = if x == 0 then 0 else if x == 1 then 1 else if x == 2 then 2 else if x == 3 then 3 else if x == 4 then 4 else if x == 5 then 5 else if x == 6 then 6 else if x == 7 then 7 else if x == 8 then 8 else if x == 9 then 9 else if x == 10 then 10 else 11
"#,
        &[
            "the `jpl` profile mandates complexity 10",
            "the body's cyclomatic complexity (branches + 1) is 12, above 10",
        ],
    );
    // Rule 4: 60 source lines.
    expect_err_jpl(
        "jpl-lines",
        r#"
module Main where
  long :: Int -> Int
  long x =
    let a1 = x + 1 in
    let a2 = a1 + 1 in
    let a3 = a2 + 1 in
    let a4 = a3 + 1 in
    let a5 = a4 + 1 in
    let a6 = a5 + 1 in
    let a7 = a6 + 1 in
    let a8 = a7 + 1 in
    let a9 = a8 + 1 in
    let a10 = a9 + 1 in
    let a11 = a10 + 1 in
    let a12 = a11 + 1 in
    let a13 = a12 + 1 in
    let a14 = a13 + 1 in
    let a15 = a14 + 1 in
    let a16 = a15 + 1 in
    let a17 = a16 + 1 in
    let a18 = a17 + 1 in
    let a19 = a18 + 1 in
    let a20 = a19 + 1 in
    let a21 = a20 + 1 in
    let a22 = a21 + 1 in
    let a23 = a22 + 1 in
    let a24 = a23 + 1 in
    let a25 = a24 + 1 in
    let a26 = a25 + 1 in
    let a27 = a26 + 1 in
    let a28 = a27 + 1 in
    let a29 = a28 + 1 in
    let a30 = a29 + 1 in
    let a31 = a30 + 1 in
    let a32 = a31 + 1 in
    let a33 = a32 + 1 in
    let a34 = a33 + 1 in
    let a35 = a34 + 1 in
    let a36 = a35 + 1 in
    let a37 = a36 + 1 in
    let a38 = a37 + 1 in
    let a39 = a38 + 1 in
    let a40 = a39 + 1 in
    let a41 = a40 + 1 in
    let a42 = a41 + 1 in
    let a43 = a42 + 1 in
    let a44 = a43 + 1 in
    let a45 = a44 + 1 in
    let a46 = a45 + 1 in
    let a47 = a46 + 1 in
    let a48 = a47 + 1 in
    let a49 = a48 + 1 in
    let a50 = a49 + 1 in
    let a51 = a50 + 1 in
    let a52 = a51 + 1 in
    let a53 = a52 + 1 in
    let a54 = a53 + 1 in
    let a55 = a54 + 1 in
    let a56 = a55 + 1 in
    let a57 = a56 + 1 in
    let a58 = a57 + 1 in
    let a59 = a58 + 1 in
    let a60 = a59 + 1 in
    let a61 = a60 + 1 in
    a61
"#,
        &[
            "the `jpl` profile mandates at most 60 source lines",
            "the body spans 62 source lines, above 60",
        ],
    );
    // Rule 5: two runtime checks on exported functions.
    expect_err_jpl(
        "jpl-assert-min",
        r#"
module Main where
  thin :: Int -> Int
  thin x = x + 1
"#,
        &[
            "the `jpl` profile mandates 2 runtime checks on exported functions",
            "below 2",
        ],
    );
    // Rule 1: a static recursion bound for every definition.
    expect_err_jpl(
        "jpl-recursion",
        r#"
module Main where
  climb :: Nat -> Nat
  climb x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> 1
      No _ -> 2
    No _ -> 1 + climb (x + 1)
"#,
        &[
            "the `jpl` profile requires a static recursion bound for every definition",
            "Main.climb establishes none",
        ],
    );
    // Rule 3: no allocation outside initialization.
    expect_err_jpl(
        "jpl-no-alloc",
        r#"
module Main where
  wrap :: Int -> List Nat -> List Nat
  wrap x xs = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> Cons 1 xs
      No _ -> xs
    No _ -> Cons 2 xs
"#,
        &[
            "the `jpl` profile admits no allocation outside initialization",
            "the constructor `Cons` is constructed here",
            "no allocation site may run per call",
        ],
    );
    // Rule 8: no macro expansion inside a definition.
    expect_err_jpl(
        "jpl-no-macros",
        r#"
module Main where
  macro mkGen :: Syntax -> Declarations
  macro mkGen arg = quoteDecls where
    gen :: Nat -> Nat
    gen x = case decide (x < 5) of
      Yes _ -> case decide (x < 3) of
        Yes _ -> 1
        No _ -> 2
      No _ -> 3

  generate do
    emit (mkGen!(z))
"#,
        &[
            "the `jpl` profile forbids macro expansion inside a definition",
            "a macro expansion occurred at this site",
        ],
    );
    // Rule 9: no raw pointers.
    expect_err_jpl(
        "jpl-no-raw-ptr",
        r#"
module Main where
  ptrFn :: RawPtr U8 -> Nat -> Nat
  ptrFn p x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> 1
      No _ -> 2
    No _ -> 3
"#,
        &[
            "the `jpl` profile forbids raw pointers and opaque types",
            "RawPtr appears in the signature",
        ],
    );
    // Rule 10: warnings are errors under the profile.
    expect_err_jpl(
        "jpl-deny-warnings",
        r#"
module Main where
  weighted :: Int -> Int
  weighted a = if a == 0 then #[weight(2)] 1 else 2
"#,
        &["partly annotated with weights", "denied by `#[deny-warnings]`"],
    );
    // A definition's own attributes may be stricter, never looser.
    expect_err_jpl(
        "jpl-never-looser",
        r#"
module Main where
  #[max-complexity(20)]
  loose :: Int -> Int
  loose x = x + 1
"#,
        &[
            "`#[max-complexity 20]` is looser than the `jpl` profile's rule (complexity 10)",
            "never looser",
        ],
    );
}

// ---------------------------------------- the artifact manifest's safety section

#[test]
fn manifest_records_the_safety_summary_without_paths() {
    let dir = write_project(&[(
        "Main.cinch",
        r#"
module Main where
  #[recursion("bounded(xs)")]
  walk :: List Nat -> Nat
  walk xs = case xs of
    Nil -> 0
    Cons x rest -> case decide (x < 5) of
      Yes _ -> case decide (x < 3) of
        Yes _ -> x + walk rest
        No _ -> walk rest
      No _ -> walk rest

  main :: Int
  main = case 2 + 2 of
    n -> case decide (n < 5) of
      Yes _ -> case decide (n < 3) of
        Yes _ -> 1
        No _ -> 2
      No _ -> 3
"#,
    )]);
    driver_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir.join("Main.cinch"),
        None,
        &TargetProfile::jpl(),
    )
    .expect("jpl check run");
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");

    // Per definition: the assertion, its verification result, and for
    // `bounded(x)` which proof method established the bound.
    let section_start = manifest.find("\"safety\":{").expect("safety section");
    let section_end = manifest[section_start..]
        .find("\"specialization\":")
        .map(|i| section_start + i)
        .expect("the section after safety");
    let section = &manifest[section_start..section_end];
    assert!(section.contains("\"assertions\":["), "{section}");
    assert!(
        section.contains(
            "{\"attribute\":\"recursion=bounded(xs)\",\"definition\":\"Main.walk\",\"result\":\"structural (depth <= xs: the size of the decreasing argument)\"}"
        ),
        "{section}"
    );
    assert!(section.contains("{\"attribute\":\"jpl assert-min=2\",\"definition\":\"Main.walk\""), "{section}");
    // The `jpl` summary (decision D114).
    assert!(section.contains("\"jpl\":{\"assertions\":"), "{section}");
    assert!(section.contains("\"modules\":[\"Main\"]}}"), "{section}");
    // The summary names definitions and rules, never paths.
    assert!(!section.contains('/'), "no paths in the safety summary: {section}");
    assert!(!section.contains(&dir.display().to_string()), "{section}");
    let _ = std::fs::remove_dir_all(&dir);
}

// ------------------------------------------ the Result must-use default (rule 7)

#[test]
fn result_must_use_rejects_the_bare_discard() {
    expect_err(
        "must-use-discard",
        r#"
module Main where
  mightFail :: Int -> Result Int Int
  mightFail x = if x == 0 then Ok 1 else Err 2

  main :: Int
  main = let _ = mightFail 1 in 0
"#,
        &[
            "discarding a `Result`-typed expression without `ignore`",
            "return-value discipline of JPL rule 7",
        ],
    );
    // The purposeful discard passes and runs.
    expect_ok(
        "must-use-ignore",
        r#"
module Main where
  mightFail :: Int -> Result Int Int
  mightFail x = if x == 0 then Ok 1 else Err 2

  main :: Int
  main = let _ = ignore (mightFail 1) in 0
"#,
        0,
    );
}
