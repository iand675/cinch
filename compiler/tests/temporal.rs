//! Temporal contracts (P16/A3; core specification section 10): safety,
//! liveness, recurrence and stability contracts over definitions. Each
//! kind is verified where its claim is decidable and refused by name
//! where it is not, never silently unchecked:
//!
//! * `safety` (`always (P)`) checks the inductive invariant over the
//!   definition's per-step transitions (the flow-sensitive write log of
//!   evidence invalidation over mutable state): a violating write is
//!   rejected naming the step, an undecidable step is refused.
//! * `stability` (`stable (P)`) survives exactly the transitions that do
//!   not touch the fact's footprint (the evidence-invalidation rule).
//! * `liveness` (`eventually (Q)`) verifies only the finite
//!   structured-concurrency obligations (`joined`, `cleanup`) from the
//!   P12 guarantees, bounded by the `withScope`; an unbounded eventual
//!   claim is refused naming the reason.
//! * `recurrence` (`recur (P)`) verifies over structural recursion (the
//!   bounded-depth machinery), witnessed at every cycle; a non-structural
//!   recursion is refused and an unwitnessed cycle is rejected naming the
//!   recursive call.
//!
//! The verified contracts are contract facts in `.cinchif`/`iface_hash`
//! (identity pinned both ways, the P20 precedent) and rows in the
//! artifact manifest with their stated assumptions.

use cinch::check::{check_program_multi_full, StagingCx};
use cinch::core::TargetProfile;
use cinch::driver::{
    check_sources, check_sources_ifaces, driver_with_profile, parse_prelude, parse_source,
    Mode, StageMode,
};
use cinch::interface::{ContractFact, Iface};
use std::collections::HashMap;
use std::path::PathBuf;

/// A program that must type-check (the contract verifies).
fn expect_checks(name: &str, src: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => {}
        Err(e) => panic!("{name}: expected the program to check, but it failed: {e}"),
    }
}

/// A program that must be rejected, with every `needle` in the error.
fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!(
            "{name}: expected an error containing {needles:?}, but the program checked"
        ),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(
                    msg.contains(n),
                    "{name}: error {msg:?} does not contain {n:?}"
                );
            }
        }
    }
}

/// The `Main` interface of a checked source.
fn iface_of(src: &str) -> Iface {
    let (_p, ifaces) = check_sources_ifaces(src, "<test>").expect("check");
    ifaces
        .into_iter()
        .find(|i| i.module == "Main")
        .expect("Main interface")
}

/// A temp project directory holding the given `(file, source)` pairs.
fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("sltemporal_{}_{}", std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

/// The mutating loop every safety and recurrence case runs over: one
/// `refSet` step per iteration, then the recursive call.
const LOOP: &str = r#"
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 3 in
      countdown m (n - 1)
"#;

// -------------------------------------------------------------- safety

#[test]
fn safety_invariant_holds_across_a_mutating_loop() {
    expect_checks(
        "safety-loop",
        &format!(
            "{LOOP}
  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"
        ),
    );
}

#[test]
fn safety_violating_write_rejects_naming_the_step() {
    // The loop writes 9, which falsifies the invariant at that write: the
    // error names the step (the write's site).
    let src = r#"
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 9 in
      countdown m (n - 1)

  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"#;
    expect_err(
        "safety-violated",
        src,
        &[
            "temporal safety",
            "is falsified by the step at 8:",
            "the write leaves the state `9 < 5` false",
            "always (copyOut m < 5)",
        ],
    );
}

#[test]
fn safety_undecidable_step_is_refused_never_trusted() {
    // A write of an unknown value leaves the invariant undecidable at
    // that step: refused by name (the D113 discipline), never assumed.
    expect_err(
        "safety-undecidable",
        r#"
module Main where
  countdown :: forall region. Mut region Nat -> Nat -> Nat
  countdown m x =
    let u = refSet m x in
    copyOut m

  contract safety undecidable for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"#,
        &[
            "temporal safety",
            "refused",
            "leaves the state `x < 5`, which the checker cannot decide",
        ],
    );
}

#[test]
fn safety_without_establishment_is_rejected_naming_the_fix() {
    // The inductive invariant checks its initial state: the contract must
    // state its establishment.
    expect_err(
        "safety-unbased",
        &format!(
            "{LOOP}
  contract safety loopInv for countdown where
    always = (copyOut m < 5)

  main :: Int
  main = 0
"
        ),
        &[
            "temporal safety",
            "has no establishment for `always (copyOut m < 5)`",
            "assume = (copyOut m < 5)",
        ],
    );
}

#[test]
fn safety_formula_must_read_borrowed_state() {
    // An `always` claim is over the state transitions of the definition:
    // a formula over immutable data reads no state.
    expect_err(
        "safety-stateless",
        r#"
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n = copyOut m

  contract safety stateless for countdown where
    always = (n < 5)
    assume = (n < 5)

  main :: Int
  main = 0
"#,
        &["temporal safety", "reads no borrowed state"],
    );
}

// ----------------------------------------------------------- stability

#[test]
fn stability_facts_survive_non_touching_transitions() {
    // The fact is about `a`'s location; the definition writes `b`'s (a
    // direct write and a call's potential write) and reads `a`: the fact
    // survives every transition.
    expect_checks(
        "stability-spared",
        r#"
module Main where
  writer :: forall r. Mut r Nat -> Unit
  writer b = refSet b 7

  prog :: forall r1 r2. Mut r1 Nat -> Mut r2 Nat -> Nat
  prog a b =
    let u = writer b in
    let v = refSet b 9 in
    copyOut a

  contract stability stays for prog where
    stable = (copyOut a < 5)

  main :: Int
  main = 0
"#,
    );
}

#[test]
fn stability_facts_die_on_a_touching_write() {
    // The write touches the fact's footprint: the fact dies there, with
    // the evidence-invalidation rule named.
    expect_err(
        "stability-killed",
        r#"
module Main where
  prog :: forall r1 r2. Mut r1 Nat -> Mut r2 Nat -> Nat
  prog a b =
    let u = refSet a 9 in
    copyOut a

  contract stability stays for prog where
    stable = (copyOut a < 5)

  main :: Int
  main = 0
"#,
        &[
            "temporal stability",
            "dies at the step at 5:",
            "evidence invalidation over mutable state",
        ],
    );
}

// ------------------------------------------------------------ liveness

/// The structured spawn/join scope every liveness case runs over.
const WORK: &str = r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2
"#;

#[test]
fn liveness_join_completes_within_the_spawn_scope() {
    // The P12 structured-scope guarantee: every spawned handle is joined
    // before the scope exits.
    expect_checks(
        "liveness-joined",
        &format!(
            "{WORK}
  contract liveness joinsRun for work where
    eventually = (joined)
    assume = \"scheduler-fair\"

  main :: Int
  main = 0
"
        ),
    );
}

#[test]
fn liveness_cleanup_runs_within_the_spawn_scope() {
    // The companion finite obligation: the scope's exit runs its spawned
    // tasks' cleanup through the join/cancel discipline.
    expect_checks(
        "liveness-cleanup",
        &format!(
            "{WORK}
  contract liveness cleanupRuns for work where
    eventually = (cleanup)
    assume = \"scheduler-fair\"

  main :: Int
  main = 0
"
        ),
    );
}

#[test]
fn liveness_unbounded_claim_is_refused_naming_the_reason() {
    // An open-ended eventual claim needs a well-founded progress argument
    // and stated fairness assumptions: refused, never trusted.
    expect_err(
        "liveness-unbounded",
        &format!(
            "{LOOP}
  contract liveness reachesZero for countdown where
    eventually = (copyOut m < 5)
    assume = \"scheduler-fair\"

  main :: Int
  main = 0
"
        ),
        &[
            "temporal liveness",
            "an unbounded eventual claim",
            "refused, never trusted",
        ],
    );
}

#[test]
fn liveness_bounded_claim_needs_a_spawn_scope() {
    // The finite claim has no subject without a structured spawn/join.
    expect_err(
        "liveness-no-subject",
        &format!(
            "{LOOP}
  contract liveness joinsRun for countdown where
    eventually = (joined)
    assume = \"scheduler-fair\"

  main :: Int
  main = 0
"
        ),
        &[
            "temporal liveness",
            "has no subject",
            "withScope",
        ],
    );
}

#[test]
fn liveness_must_state_its_assumptions() {
    // A liveness claim relies on the runtime's scheduling and
    // cancellation policies and must state them.
    expect_err(
        "liveness-unassumed",
        &format!(
            "{WORK}
  contract liveness joinsRun for work where
    eventually = (joined)

  main :: Int
  main = 0
"
        ),
        &["temporal liveness", "states no assumption"],
    );
}

// ----------------------------------------------------------- recurrence

/// The structurally recursive loop whose state recurs every cycle.
const WALK: &str = r#"
module Main where
  walk :: forall region. Mut region Nat -> List Nat -> Nat
  walk m xs = case xs of
    Nil -> copyOut m
    Cons x rest ->
      let u = refSet m 3 in
      walk m rest
"#;

#[test]
fn recurrence_is_bounded_over_structural_recursion() {
    // The state recurs at every cycle of the structural recursion: the
    // bound is the decreasing argument's size.
    expect_checks(
        "recurrence-walk",
        &format!(
            "{WALK}
  contract recurrence marker for walk where
    recur = (copyOut m < 5)

  main :: Int
  main = 0
"
        ),
    );
}

#[test]
fn recurrence_without_a_structural_bound_is_refused() {
    // The bound of a recurrence claim comes from structural recursion:
    // the literal ladder is not one.
    expect_err(
        "recurrence-unbounded",
        r#"
module Main where
  down :: forall region. Mut region Nat -> Int -> Nat
  down m 3 = down m 2
  down m 2 = down m 1
  down m 1 = copyOut m
  down m _ = copyOut m

  contract recurrence marker for down where
    recur = (copyOut m < 5)

  main :: Int
  main = 0
"#,
        &[
            "temporal recurrence",
            "has no structural depth bound",
            "refused, never trusted",
        ],
    );
}

#[test]
fn recurrence_without_a_witness_rejects_naming_the_call() {
    // The recursive cycle never re-establishes the state: rejected at the
    // recursive call.
    expect_err(
        "recurrence-unwitnessed",
        r#"
module Main where
  walk :: forall region. Mut region Nat -> List Nat -> Nat
  walk m xs = case xs of
    Nil -> copyOut m
    Cons x rest -> walk m rest

  contract recurrence marker for walk where
    recur = (copyOut m < 5)

  main :: Int
  main = 0
"#,
        &[
            "temporal recurrence",
            "is not witnessed at the recursive call at 6:",
            "no step of the cycle re-establishes",
        ],
    );
}

// ------------------------------------------------------- contract facts

#[test]
fn contract_facts_hash_both_ways() {
    let plain = format!(
        "{LOOP}
  main :: Int
  main = 0
"
    );
    let contracted = format!(
        "{LOOP}
  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"
    );
    let base = iface_of(&plain);
    let with = iface_of(&contracted);

    // The verified contract is a contract fact recorded in the interface
    // exactly like `repr` facts.
    assert!(
        base.contracts.is_empty(),
        "no contract, no fact: {:?}",
        base.contracts
    );
    let facts: Vec<&ContractFact> =
        with.contracts.iter().filter(|f| f.name == "countdown").collect();
    assert_eq!(facts.len(), 1, "one contract fact: {:?}", with.contracts);
    let fact = facts[0];
    assert_eq!(fact.kind, "safety");
    assert_eq!(fact.formula, "always (copyOut m < 5)");
    assert_eq!(fact.assumptions, vec!["assume (copyOut m < 5)".to_string()]);
    let text = with.to_json();
    assert!(text.contains("\"contracts\""), "{text}");
    assert!(text.contains("always (copyOut m < 5)"), "{text}");

    // Interface identity is pinned both ways: the fact's appearance
    // changes `iface_hash`, and its disappearance returns to the identity
    // it left (the `repr` precedent).
    assert_ne!(
        base.iface_hash(),
        with.iface_hash(),
        "a contract fact appearing must change the interface identity"
    );
    let base_again = iface_of(&plain);
    let with_again = iface_of(&contracted);
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

// ----------------------------------------- the artifact manifest's temporal section

#[test]
fn manifest_records_the_temporal_contracts_with_assumptions() {
    let dir = write_project(&[(
        "Main.cinch",
        r#"
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 3 in
      countdown m (n - 1)

  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)
    assume = "scheduler-fair"

  walk :: forall region. Mut region Nat -> List Nat -> Nat
  walk m xs = case xs of
    Nil -> copyOut m
    Cons x rest ->
      let u = refSet m 3 in
      walk m rest

  contract recurrence marker for walk where
    recur = (copyOut m < 5)

  main :: Int
  main = 0
"#,
    )]);
    driver_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir.join("Main.cinch"),
        None,
        &TargetProfile::native(),
    )
    .expect("check run");
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");

    // Per contract: the definition and contract it names, the kind and
    // formula, the verification result, and the stated assumptions.
    let section_start = manifest.find("\"temporal\":{").expect("temporal section");
    let section_end = manifest[section_start..]
        .find("\"trust\":{")
        .map(|i| section_start + i)
        .expect("trust section");
    let section = &manifest[section_start..section_end];
    assert!(section.contains("\"contracts\":["), "{section}");
    // Rows are canonical JSON, keys in byte order.
    assert!(
        section.contains(
            "\"contract\":\"loopInv\",\"definition\":\"Main.countdown\",\"formula\":\"always (copyOut m < 5)\",\"kind\":\"safety\""
        ),
        "{section}"
    );
    assert!(
        section.contains(
            "\"contract\":\"marker\",\"definition\":\"Main.walk\",\"formula\":\"recur (copyOut m < 5)\",\"kind\":\"recurrence\""
        ),
        "{section}"
    );
    // The verification says which argument discharged the claim.
    assert!(
        section.contains("the invariant holds at every step"),
        "{section}"
    );
    assert!(
        section.contains("the state recurs at every cycle of the structural recursion"),
        "{section}"
    );
    // The stated assumptions ride along (a liveness claim relies on the
    // runtime policies it names).
    assert!(section.contains("\"assume (copyOut m < 5)\""), "{section}");
    assert!(section.contains("\"assume \\\"scheduler-fair\\\"\""), "{section}");
    // The temporal rows name definitions and contracts, never paths.
    assert!(!section.contains('/'), "no paths in the temporal section: {section}");
    assert!(!section.contains(&dir.display().to_string()), "{section}");
    // The stated assumptions join the declared-assumptions table (the
    // acceptance manifest identifies what claimed guarantees rely on).
    assert!(
        manifest.contains("temporal safety on Main.countdown: assume"),
        "{manifest}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// ------------------------------------------------------------ contract shape

#[test]
fn contract_names_no_definition() {
    expect_err(
        "contract-orphan",
        r#"
module Main where
  contract safety orphan for missing where
    always = (1 < 5)
    assume = (1 < 5)

  main :: Int
  main = 0
"#,
        &["the temporal contract `orphan` names no definition `Main.missing`"],
    );
}

#[test]
fn contract_formula_operator_matches_the_kind() {
    // The language distinguishes safety, liveness, recurrence and
    // stability: a `safety` contract states `always (P)`.
    expect_err(
        "contract-operator",
        r#"
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n = copyOut m

  contract safety wrongOp for countdown where
    eventually = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"#,
        &[
            "a `safety` temporal contract states `always = (P)`, not `eventually = (...)`",
            "core specification section 10",
        ],
    );
}

// ------------------------------------------------- the check-time report surface

/// The report rows of a checked source (the manifest rows' source).
fn report_rows(src: &str) -> Vec<(String, String, String, String)> {
    let prelude = parse_prelude(&[]).expect("prelude");
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new()).expect("parse");
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
    let (_p, _i, report, _d) = check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &[],
        &TargetProfile::native(),
    )
    .expect("check");
    report
        .temporal
        .iter()
        .map(|t| {
            (
                t.definition.clone(),
                t.kind.clone(),
                t.formula.clone(),
                t.result.clone(),
            )
        })
        .collect()
}

#[test]
fn report_rows_name_the_verified_contracts() {
    let rows = report_rows(&format!(
        "{LOOP}
  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
"
    ));
    assert_eq!(rows.len(), 1, "{rows:?}");
    let (def, kind, formula, result) = &rows[0];
    assert_eq!(def, "Main.countdown");
    assert_eq!(kind, "safety");
    assert_eq!(formula, "always (copyOut m < 5)");
    assert!(
        result.contains("discharged by the inductive step"),
        "{result}"
    );
}
