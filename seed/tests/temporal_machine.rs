//! End-to-end integration tests for the TLA+-like temporal prototype:
//! - Explicit-state `machine` coalgebras with `always` and `deadlock_free` safety contracts
//! - Bounded model checking with shortest BFS safety traces and Tarjan/Emerson-Lei fair-SCC lassos
//! - True Weak Fairness (`fair`) vs Strong Fairness (`strong_fair`) semantics on `leads_to`/`eventually`/`recur`/`stable`
//! - Type-directed finite domain enumeration (`InRange`, finite ADT enums)
//! - Automatic sound symmetry reduction (enforcing absence of `Ord`) and static field-footprint POR
//! - Checked stuttering simulation refinement (`refines Concrete Abstract`) preventing model-code drift
//! - Stuttering-invariance guard rejecting raw `next` in public contracts
//! - Clocked `Later k` (`laterPure`, `laterAp`, `laterFix`, `forceClock`) and clock-mismatch/escape rejection
//! - Fair `MVar` automatic contention-fairness discharge vs honest refusal on unfair `STM`

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;
use common::source_frontend::{check_sources, run_source};
use std::path::PathBuf;

fn write_temp_project(src: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("sltempmc_{}_{}", std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    dir
}

/// Checks `src` using the configured compiler (Rust seed or `CINCH_TEST_COMPILER` Stage 3)
/// and returns the emitted `Main.manifest.json` containing verified temporal rows.
fn expect_ok(name: &str, src: &str) -> String {
    let dir = write_temp_project(src);
    if let Err(e) = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    ) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{name}: expected check to succeed, but failed:\n{e}");
    }
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("{name}: missing build/Main.manifest.json: {e}"));
    let _ = std::fs::remove_dir_all(&dir);
    manifest
}

fn expect_ok_stdlib(name: &str, src: &str) {
    if let Err(e) = check_sources(src, "<temporal-machine-test>") {
        panic!("{name}: expected check to succeed, but failed:\n{e}");
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<temporal-machine-test>") {
        Ok(_) => panic!("{name}: expected check to fail with {needles:?}, but it succeeded"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(
                    msg.contains(n),
                    "{name}: expected error containing {n:?}, got:\n{msg}"
                );
            }
        }
    }
}

// ----------------------------------------------------------------- 1. Safety & Counterexamples

#[test]
fn machine_safety_always_verifies_and_catches_violation_with_shortest_trace() {
    let ok_src = r#"
module Main where
  record Pool where
    queued :: Nat
    running :: Nat
    done :: Nat

  accept :: Pool -> Option Pool
  accept s =
    if s.queued + s.running < 2
    then Some (Pool { queued = s.queued + 1, running = s.running, done = s.done })
    else None

  start :: Pool -> Option Pool
  start s =
    let q = s.queued in
    if q > 0 && s.running < 1
    then Some (Pool { queued = q - 1, running = s.running + 1, done = s.done })
    else None

  finish :: Pool -> Option Pool
  finish s =
    let r = s.running in
    if r > 0 && s.done < 3
    then Some (Pool { queued = s.queued, running = r - 1, done = s.done + 1 })
    else None

  machine WorkerPool where
    state = Pool
    init = Pool { queued = 0, running = 0, done = 0 }
    actions = [accept, start, finish]

  contract safety boundInv for WorkerPool where
    always = (\s -> s.queued + s.running <= 2 && s.running <= 1)

  main :: Int
  main = 0
"#;
    let manifest = expect_ok("worker-pool-safety", ok_src);
    assert!(manifest.contains("\"contract\":\"boundInv\""), "got: {manifest}");
    assert!(manifest.contains("model-checked"), "got: {manifest}");
    assert!(manifest.contains("0 violations"), "got: {manifest}");

    // Buggy version: `accept` forgets the capacity guard, violating `queued + running <= 2`.
    let bad_src = r#"
module Main where
  record Pool where
    queued :: Nat
    running :: Nat
    done :: Nat

  accept :: Pool -> Option Pool
  accept s =
    if s.queued < 4
    then Some (Pool { queued = s.queued + 1, running = s.running, done = s.done })
    else None

  machine BuggyPool where
    state = Pool
    init = Pool { queued = 0, running = 0, done = 0 }
    actions = [accept]

  contract safety boundInv for BuggyPool where
    always = (\s -> s.queued <= 2)

  main :: Int
  main = 0
"#;
    expect_err(
        "buggy-pool-safety",
        bad_src,
        &[
            "temporal safety violation",
            "boundInv",
            "State 0 (init)",
            "--[accept]-->",
            "queued = 3",
        ],
    );
}

#[test]
fn machine_deadlock_freedom_verifies_and_reports_deadlock_trace() {
    let live_src = r#"
module Main where
  record Toggle where
    on :: Bool

  flip :: Toggle -> Toggle
  flip s = Toggle { on = not s.on }

  machine Lamp where
    state = Toggle
    init = Toggle { on = False }
    actions = [flip]

  contract safety noDeadlock for Lamp where
    deadlock_free = (True)

  main :: Int
  main = 0
"#;
    expect_ok("deadlock-free-ok", live_src);

    let dead_src = r#"
module Main where
  record Counter where
    n :: Nat

  inc :: Counter -> Option Counter
  inc s = if s.n < 2 then Some (Counter { n = s.n + 1 }) else None

  machine Stuck where
    state = Counter
    init = Counter { n = 0 }
    actions = [inc]

  contract safety noDeadlock for Stuck where
    deadlock_free = (True)

  main :: Int
  main = 0
"#;
    expect_err(
        "deadlock-caught",
        dead_src,
        &["deadlock", "noDeadlock", "State 0 (init)", "--[inc]-->", "n = 2"],
    );
}

// ----------------------------------------------------------------- 2. Liveness, Leads-To, WF vs SF

#[test]
fn liveness_leads_to_distinguishes_unfair_weak_fair_and_strong_fair() {
    // Classic WF vs SF system:
    // - `toggle` flips `ready` between False and True while `done == False`.
    // - `finish` is ONLY enabled when `ready == True && done == False` (repeatedly enabled, NOT continuously!).
    // - `idle` self-loops once `done == True` (so no deadlock).
    // Result:
    // - Unfair: fails `leads_to` (lasso toggling forever).
    // - Weak fairness on `finish`: STILL fails `leads_to` because `finish` is disabled whenever `ready == False`!
    // - Strong fairness on `finish`: PASSES `leads_to` because `finish` is enabled infinitely often along any toggle cycle!
    let base = r#"
module Main where
  record S where
    ready :: Bool
    done :: Bool

  toggle :: S -> Option S
  toggle s =
    if not s.done
    then Some (S { ready = not s.ready, done = False })
    else None

  finish :: S -> Option S
  finish s =
    if s.ready && not s.done
    then Some (S { ready = False, done = True })
    else None

  idle :: S -> Option S
  idle s = if s.done then Some s else None
"#;

    let unfair = format!(
        "{base}
  machine M where
    state = S
    init = S {{ ready = False, done = False }}
    actions = [toggle, finish, idle]

  contract liveness completes for M where
    leads_to = (\\s -> not s.done) ~> (\\s -> s.done)

  main :: Int
  main = 0
"
    );
    expect_err(
        "unfair-lasso",
        &unfair,
        &["temporal liveness violation", "fair cycle", "--[toggle]-->"],
    );

    let weak_only = format!(
        "{base}
  machine M where
    state = S
    init = S {{ ready = False, done = False }}
    actions = [toggle, finish, idle]
    fair = [finish]

  contract liveness completes for M where
    leads_to = (\\s -> not s.done) ~> (\\s -> s.done)

  main :: Int
  main = 0
"
    );
    expect_err(
        "wf-insufficient-for-intermittent-action",
        &weak_only,
        &["temporal liveness violation", "fair cycle"],
    );

    let strong_ok = format!(
        "{base}
  machine M where
    state = S
    init = S {{ ready = False, done = False }}
    actions = [toggle, finish, idle]
    strong_fair = [finish]

  contract liveness completes for M where
    leads_to = (\\s -> not s.done) ~> (\\s -> s.done)

  contract liveness reachesDone for M where
    eventually = (\\s -> s.done)

  contract stability staysDone for M where
    stable = (\\s -> s.done)

  main :: Int
  main = 0
"
    );
    expect_ok("sf-discharges-intermittent-leads-to", &strong_ok);
}

#[test]
fn weak_fairness_discharges_continuously_enabled_leads_to_and_recurrence() {
    let src = r#"
module Main where
  record Job where
    pending :: Bool
    ticks :: Nat

  stutter :: Job -> Option Job
  stutter s =
    if s.ticks < 2
    then Some (Job { pending = s.pending, ticks = s.ticks + 1 })
    else Some (Job { pending = s.pending, ticks = 0 })

  complete :: Job -> Option Job
  complete s =
    if s.pending
    then Some (Job { pending = False, ticks = s.ticks })
    else None

  submit :: Job -> Option Job
  submit s =
    if not s.pending
    then Some (Job { pending = True, ticks = s.ticks })
    else None

  machine JobService where
    state = Job
    init = Job { pending = True, ticks = 0 }
    actions = [stutter, complete, submit]
    fair = [complete, submit]

  contract liveness alwaysCompletes for JobService where
    leads_to = (\s -> s.pending) ~> (\s -> not s.pending)

  contract recurrence keepsServing for JobService where
    recur = (\s -> not s.pending)

  main :: Int
  main = 0
"#;
    expect_ok("wf-continuously-enabled", src);
}

// ----------------------------------------------------------------- 3. Parameterized Actions, Symmetry & POR

#[test]
fn parameterized_refined_actions_symmetry_reduction_and_por_work_and_reject_ord() {
    // Two symmetric nodes `N0`, `N1` without `Ord`:
    // Choosing either node to hold the lock produces symmetric states that merge!
    // Also includes two actions touching disjoint fields (`left`, `right`) to exercise POR analysis.
    let ok_src = r#"
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  record Sys where
    owner :: Option Node
    left :: Nat
    right :: Nat

  acquire :: Node -> Sys -> Option Sys
  acquire n s = case s.owner of
    None -> Some (Sys { owner = Some n, left = s.left, right = s.right })
    Some _ -> None

  release :: Sys -> Option Sys
  release s = case s.owner of
    Some _ -> Some (Sys { owner = None, left = s.left, right = s.right })
    None -> None

  bumpLeft :: InRange 0 1 Nat -> Sys -> Option Sys
  bumpLeft d s =
    if s.left == 0 && d == 1
    then Some (Sys { owner = s.owner, left = 1, right = s.right })
    else None

  bumpRight :: Sys -> Option Sys
  bumpRight s =
    if s.right == 0
    then Some (Sys { owner = s.owner, left = s.left, right = 1 })
    else None

  machine LockSys where
    state = Sys
    init = Sys { owner = None, left = 0, right = 0 }
    actions = [acquire, release, bumpLeft, bumpRight]
    symmetry = [Node]

  contract safety boundedCounts for LockSys where
    always = (\s -> s.left <= 1 && s.right <= 1)

  main :: Int
  main = 0
"#;
    let manifest = expect_ok("symmetry-and-por", ok_src);
    assert!(manifest.contains("\"contract\":\"boundedCounts\""), "got: {manifest}");
    assert!(
        manifest.contains("symmetry_merged: ") && !manifest.contains("symmetry_merged: 0"),
        "expected symmetry reduction to merge symmetric states, got: {manifest}"
    );
    assert!(
        manifest.contains("por_independent_pairs: ") && !manifest.contains("por_independent_pairs: 0"),
        "expected POR to detect disjoint field footprints, got: {manifest}"
    );

    // Adding an `Ord` instance on `Node` makes symmetry reduction unsound -> must be rejected!
    let bad_sym = r#"
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  nodeTag :: Node -> Nat
  nodeTag n = case n of
    N0 -> 0
    N1 -> 1

  implicit instance EqNode :: Eq Node where
    (==) a b = nodeTag a == nodeTag b
    (/=) a b = nodeTag a /= nodeTag b

  implicit instance OrdNode :: Ord Node where
    (<) a b = nodeTag a < nodeTag b
    (<=) a b = nodeTag a <= nodeTag b
    (>) a b = nodeTag a > nodeTag b
    (>=) a b = nodeTag a >= nodeTag b

  record Sys where
    owner :: Option Node

  acquire :: Node -> Sys -> Option Sys
  acquire n s = Some (Sys { owner = Some n })

  machine BadSym where
    state = Sys
    init = Sys { owner = None }
    actions = [acquire]
    symmetry = [Node]

  main :: Int
  main = 0
"#;
    expect_err(
        "unsound-symmetry-with-ord",
        bad_sym,
        &["unsound symmetry reduction", "Ord"],
    );
}

// ----------------------------------------------------------------- 4. Checked Stuttering Refinement (`refines`)

#[test]
fn stuttering_refinement_checks_simulation_measure_and_catches_drift() {
    // Concrete machine takes two internal steps (Preparing -> Committed) to simulate
    // one abstract step (Idle -> Done). `measure` strictly decreases on the stuttering step!
    let ok_src = r#"
module Main where
  record AbsState where
    done :: Bool

  absFinish :: AbsState -> Option AbsState
  absFinish s = if not s.done then Some (AbsState { done = True }) else None

  machine AbsSpec where
    state = AbsState
    init = AbsState { done = False }
    actions = [absFinish]

  -- Concrete state: phase 0 = Idle, phase 1 = Preparing (stutter), phase 2 = Done
  record ConcState where
    phase :: Nat

  prepare :: ConcState -> Option ConcState
  prepare s = if s.phase == 0 then Some (ConcState { phase = 1 }) else None

  commit :: ConcState -> Option ConcState
  commit s = if s.phase == 1 then Some (ConcState { phase = 2 }) else None

  machine ConcImpl where
    state = ConcState
    init = ConcState { phase = 0 }
    actions = [prepare, commit]

  project :: ConcState -> AbsState
  project s = AbsState { done = s.phase == 2 }

  stutterRank :: ConcState -> Nat
  stutterRank s = if s.phase == 0 then 2 else (if s.phase == 1 then 1 else 0)

  refines ConcImpl AbsSpec where
    view = project
    measure = stutterRank

  main :: Int
  main = 0
"#;
    let manifest = expect_ok("refines-valid-stuttering", ok_src);
    assert!(manifest.contains("\"kind\":\"refinement\""), "got: {manifest}");
    assert!(manifest.contains("stuttering simulation"), "got: {manifest}");
    assert!(manifest.contains("measure strictly decreasing"), "got: {manifest}");

    // Drift case: concrete implementation adds a `rollback` transition from Done -> Idle,
    // which AbsSpec forbids! Compiler MUST catch the drift.
    let drift_src = r#"
module Main where
  record AbsState where
    done :: Bool

  absFinish :: AbsState -> Option AbsState
  absFinish s = if not s.done then Some (AbsState { done = True }) else None

  machine AbsSpec where
    state = AbsState
    init = AbsState { done = False }
    actions = [absFinish]

  record ConcState where
    phase :: Nat

  commit :: ConcState -> Option ConcState
  commit s = if s.phase == 0 then Some (ConcState { phase = 1 }) else None

  rollback :: ConcState -> Option ConcState
  rollback s = if s.phase == 1 then Some (ConcState { phase = 0 }) else None

  machine DriftedImpl where
    state = ConcState
    init = ConcState { phase = 0 }
    actions = [commit, rollback]

  project :: ConcState -> AbsState
  project s = AbsState { done = s.phase == 1 }

  refines DriftedImpl AbsSpec where
    view = project

  main :: Int
  main = 0
"#;
    expect_err(
        "refines-catches-drift",
        drift_src,
        &["refinement violation", "DriftedImpl", "AbsSpec", "--[rollback]-->"],
    );

    // Infinite silent stuttering case: `spin` stutters `phase 0 -> phase 0`, failing `measure`.
    let stutter_bad = r#"
module Main where
  record AbsState where
    done :: Bool

  absFinish :: AbsState -> Option AbsState
  absFinish s = if not s.done then Some (AbsState { done = True }) else None

  machine AbsSpec where
    state = AbsState
    init = AbsState { done = False }
    actions = [absFinish]

  record ConcState where
    phase :: Nat

  spin :: ConcState -> Option ConcState
  spin s = if s.phase == 0 then Some (ConcState { phase = 0 }) else None

  machine SpinningImpl where
    state = ConcState
    init = ConcState { phase = 0 }
    actions = [spin]

  project :: ConcState -> AbsState
  project s = AbsState { done = s.phase == 1 }

  stutterRank :: ConcState -> Nat
  stutterRank s = if s.phase == 0 then 1 else 0

  refines SpinningImpl AbsSpec where
    view = project
    measure = stutterRank

  main :: Int
  main = 0
"#;
    expect_err(
        "refines-catches-infinite-stuttering",
        stutter_bad,
        &["refinement liveness violation", "did not decrease `measure`"],
    );
}

// ----------------------------------------------------------------- 5. Stuttering-Invariance Guard

#[test]
fn public_contract_forbids_raw_next_operator() {
    let src = r#"
module Main where
  record S where
    x :: Nat

  step :: S -> S
  step s = S { x = s.x }

  machine M where
    state = S
    init = S { x = 0 }
    actions = [step]

  contract safety badNext for M where
    next = (\s -> s.x == 0)

  main :: Int
  main = 0
"#;
    expect_err("forbid-raw-next", src, &["`next` is forbidden", "stuttering-invariant"]);
}

// ----------------------------------------------------------------- 6. Clocked `Later k` + Löb Induction

#[test]
fn clocked_later_combinators_lob_fix_and_safe_clock_elimination() {
    let ok_src = r#"
module Main where
  codata CStream (k :: Region) (a :: Type) where
    head :: a
    tail :: Later k (CStream k a)

  -- Compositional guarded recursion via `laterFix` (Löb induction) — no syntactic `corec` needed!
  ones :: forall k. Unit -> CStream k Nat
  ones u = laterFix (\self -> CStream { head = 1, tail = self })

  -- Applicative guarded composition under the same clock `k`:
  addLater :: forall k. Later k Nat -> Later k Nat -> Later k Nat
  addLater x y = laterAp (laterAp (laterPure (\a -> \b -> a + b)) x) y

  -- Safe clock elimination: `forall k. Later k Nat` has no escaping clock `k` in `Nat`.
  runClosed :: (forall k. Later k Nat) -> Nat
  runClosed lx = forceClock lx

  main :: Int
  main =
    let s = ones () in
    let v = runClosed (laterPure (40 + s.head)) in
    if v == 41 then 0 else 1
"#;
    expect_ok("clocked-later-ok", ok_src);
    let exit_code = run_source(ok_src).expect("clocked-later-run");
    assert_eq!(exit_code, 0, "expected clocked Later program to run and return 0");

    // Mismatched clocks `k1` vs `k2` in `laterAp` must be rejected!
    let mismatch_src = r#"
module Main where
  badMix :: forall k1 k2. Later k1 (Nat -> Nat) -> Later k2 Nat -> Later k1 Nat
  badMix lf lx = laterAp lf lx

  main :: Int
  main = 0
"#;
    match check_sources(mismatch_src, "<temporal-machine-test>") {
        Ok(_) => panic!("clock-mismatch: expected mismatched clocks k1/k2 to be rejected"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains("regions do not match") || (msg.contains("k1") && msg.contains("k2")),
                "clock-mismatch: expected region/clock mismatch diagnostic, got:\n{msg}"
            );
        }
    }

    // `forceClock` on a non-polymorphic clock bound in outer scope must be rejected!
    let escape_src = r#"
module Main where
  badForce :: forall k. Later k Nat -> Nat
  badForce lx = forceClock lx

  main :: Int
  main = 0
"#;
    expect_err("clock-force-non-poly", escape_src, &["forceClock requires a clock-polymorphic"]);
}

// ----------------------------------------------------------------- 7. Fair MVar vs Unfair STM Discharge

#[test]
fn fair_mvar_discharges_handoff_liveness_while_unfair_stm_requires_assumption() {
    let mvar_src = r#"
module Main where
  import Effect.Concurrent.MVar

  exchange :: MVar Nat -> Nat ->{MVar, Diverge} Nat
  exchange slot x = do
    _ <- mvar-put slot x
    mvar-take slot

  contract liveness mvarFair for exchange where
    eventually = (fair_handoff)

  main :: Int
  main = 0
"#;
    expect_ok_stdlib("fair-mvar-discharges-liveness", mvar_src);

    let stm_unassumed = r#"
module Main where
  import Effect.Stm

  bump :: TVar Nat ->{STM, Diverge} Nat
  bump v = atomically (tvar-read v)

  contract liveness stmFair for bump where
    eventually = (fair_handoff)

  main :: Int
  main = 0
"#;
    expect_err(
        "unfair-stm-refused-without-assume",
        stm_unassumed,
        &["Effect.Stm", "does not guarantee fairness", "switch to fair `MVar`"],
    );

    let stm_assumed = r#"
module Main where
  import Effect.Stm

  bump :: TVar Nat ->{STM, Diverge} Nat
  bump v = atomically (tvar-read v)

  contract liveness stmFair for bump where
    eventually = (fair_handoff)
    assume = "stm-fair-commit"

  main :: Int
  main = 0
"#;
    expect_ok_stdlib("stm-with-explicit-fairness-assume", stm_assumed);
}

// ----------------------------------------------------------------- 8. Multi-Param Actions, Option/Tuple Domains & Field Diffs

#[test]
fn multi_param_option_tuple_actions_and_field_diff_traces() {
    // Multi-actor transfer taking two Node parameters (`from`, `to`) plus `Option Bool`
    // (or tuple `(Node, Bool)`) verifying total conservation and field diffs on violation!
    let ok_src = r#"
module Main where
  data Node where
    A :: Node
    B :: Node

  record Bank where
    balA :: Nat
    balB :: Nat

  transfer :: Node -> Node -> InRange 1 1 Nat -> Bank -> Option Bank
  transfer src dst amt s = case (src, dst) of
    (A, B) ->
      let a = s.balA in
      if a >= 1 then Some (Bank { balA = a - 1, balB = s.balB + 1 }) else None
    (B, A) ->
      let b = s.balB in
      if b >= 1 then Some (Bank { balA = s.balA + 1, balB = b - 1 }) else None
    _ -> None

  touch :: Option Node -> Bank -> Option Bank
  touch opt s = case opt of
    None -> Some s
    Some _ -> None

  machine Ledger where
    state = Bank
    init = Bank { balA = 2, balB = 0 }
    actions = [transfer, touch]

  contract safety conservation for Ledger where
    always = (\s -> s.balA + s.balB == 2)

  main :: Int
  main = 0
"#;
    let manifest = expect_ok("multi-param-ledger", ok_src);
    assert!(manifest.contains("\"contract\":\"conservation\""), "got: {manifest}");

    // Field-diff counterexample check: buggy pool shows `(changed: queued: ...)` in trace.
    let diff_src = r#"
module Main where
  record Pool where
    queued :: Nat
    running :: Nat

  enqueue :: Pool -> Option Pool
  enqueue s =
    if s.queued < 3
    then Some (Pool { queued = s.queued + 1, running = s.running })
    else None

  machine DiffPool where
    state = Pool
    init = Pool { queued = 0, running = 0 }
    actions = [enqueue]

  contract safety capTwo for DiffPool where
    always = (\s -> s.queued <= 1)

  main :: Int
  main = 0
"#;
    expect_err(
        "field-diff-trace",
        diff_src,
        &["(changed: queued: 0 -> 1)", "(changed: queued: 1 -> 2)"],
    );
}

// ----------------------------------------------------------------- 9. Compositional Verification: Assume-Guarantee & Multi-View Refinement

#[test]
fn assume_guarantee_environment_assumptions_and_horizontal_refinement() {
    // Component machine `Worker` has an unconstrained environment action `dropPower`
    // that can flip `powered = False`. Without an environment assumption, `always`
    // on `healthy` fails; WITH `assume = (\s -> s.powered)` it verifies honestly and
    // records the assumption in the manifest!
    let base = r#"
module Main where
  record EnvWorker where
    powered :: Bool
    jobs :: Nat

  runJob :: EnvWorker -> Option EnvWorker
  runJob s =
    if s.powered && s.jobs < 2
    then Some (EnvWorker { powered = True, jobs = s.jobs + 1 })
    else None

  dropPower :: EnvWorker -> Option EnvWorker
  dropPower s =
    if s.powered
    then Some (EnvWorker { powered = False, jobs = s.jobs })
    else None

  machine Worker where
    state = EnvWorker
    init = EnvWorker { powered = True, jobs = 0 }
    actions = [runJob, dropPower]
"#;

    let unassumed = format!(
        "{base}
  contract safety staysPowered for Worker where
    always = (\\s -> s.powered)

  main :: Int
  main = 0
"
    );
    expect_err("assume-guarantee-without-env", &unassumed, &["dropPower"]);

    let assumed = format!(
        "{base}
  contract safety boundedUnderPower for Worker where
    always = (\\s -> s.powered && s.jobs <= 2)
    assume = (\\s -> s.powered)

  main :: Int
  main = 0
"
    );
    let manifest = expect_ok("assume-guarantee-with-env", &assumed);
    assert!(manifest.contains("assume ("), "expected assume row in manifest: {manifest}");

    // Horizontal product composition + dual projection refinements (`FullSystem refines AuthSpec`
    // AND `FullSystem refines QueueSpec`) verifying two independent subsystem specs simultaneously!
    let composed = r#"
module Main where
  record Auth where
    authed :: Bool

  login :: Auth -> Option Auth
  login s = if not s.authed then Some (Auth { authed = True }) else None

  machine AuthSpec where
    state = Auth
    init = Auth { authed = False }
    actions = [login]

  record Queue where
    depth :: Nat

  push :: Queue -> Option Queue
  push s = if s.depth < 2 then Some (Queue { depth = s.depth + 1 }) else None

  machine QueueSpec where
    state = Queue
    init = Queue { depth = 0 }
    actions = [push]

  record Full where
    authed :: Bool
    depth :: Nat

  sysLogin :: Full -> Option Full
  sysLogin s = if not s.authed then Some (Full { authed = True, depth = s.depth }) else None

  sysEnqueue :: Full -> Option Full
  sysEnqueue s =
    if s.authed && s.depth < 2
    then Some (Full { authed = s.authed, depth = s.depth + 1 })
    else None

  machine FullSystem where
    state = Full
    init = Full { authed = False, depth = 0 }
    actions = [sysLogin, sysEnqueue]

  viewAuth :: Full -> Auth
  viewAuth s = Auth { authed = s.authed }

  viewQueue :: Full -> Queue
  viewQueue s = Queue { depth = s.depth }

  refines FullSystem AuthSpec where
    view = viewAuth

  refines FullSystem QueueSpec where
    view = viewQueue

  main :: Int
  main = 0
"#;
    let cman = expect_ok("horizontal-dual-refines", composed);
    assert!(cman.contains("refines FullSystem AuthSpec"), "got: {cman}");
    assert!(cman.contains("refines FullSystem QueueSpec"), "got: {cman}");
}

// ----------------------------------------------------------------- 11. Data.Finite Extensible Domains & Symmetry Guard

#[test]
fn finite_typeclass_enumerates_payload_adts_and_records_and_guards_symmetry() {
    let finite_mod = include_str!("../prelude/Data/Finite.cinch");

    // 1. User-defined payload-carrying ADT (`Msg`) + record (`Cmd`) enumerated via `Data.Finite.Finite`:
    let ok_src = r#"
module Main where
  import Data.Finite

  data Client where
    C0 :: Client
    C1 :: Client

  data Msg where
    Req :: Client -> Nat -> Msg
    Ack :: Client -> Msg

  implicit instance Finite_Client :: Finite Client where
    enumerate _ = [C0, C1]

  implicit instance Finite_Msg :: Finite Msg where
    enumerate _ =
      finite-sum [
        finite-product (\c -> \w -> Req c w) (enumerate ()) [1, 2],
        finite-map Ack (enumerate ())
      ]

  record Broker where
    load :: Nat
    acks :: Nat

  dispatch :: Msg -> Broker -> Option Broker
  dispatch msg s = case msg of
    Req _ w ->
      if s.load + w <= 3
      then Some (Broker { load = s.load + w, acks = s.acks })
      else None
    Ack _ ->
      let l = s.load in
      if l > 0 && s.acks < 2
      then Some (Broker { load = l - 1, acks = s.acks + 1 })
      else None

  machine BrokerMachine where
    state = Broker
    init = Broker { load = 0, acks = 0 }
    actions = [dispatch]

  contract safety boundedLoad for BrokerMachine where
    always = (\s -> s.load <= 3 && s.acks <= 2)

  main :: Int
  main = 0
"#;
    let dir_ok = common::project("finite-adt-ok", &[
        ("Data.Finite.cinch", finite_mod),
        ("Main.cinch", ok_src),
    ]);
    let res_ok = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir_ok,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    );
    assert!(res_ok.is_ok(), "expected Data.Finite payload ADT machine to pass, got: {res_ok:?}");
    common::cleanup(&dir_ok);

    // 2. Counterexample triggers specifically on payload ADT value `Req _ 2`:
    let bad_src = r#"
module Main where
  import Data.Finite

  data Client where
    C0 :: Client
    C1 :: Client

  data Msg where
    Req :: Client -> Nat -> Msg
    Ack :: Client -> Msg

  implicit instance Finite_Client :: Finite Client where
    enumerate _ = [C0, C1]

  implicit instance Finite_Msg :: Finite Msg where
    enumerate _ =
      finite-sum [
        finite-product (\c -> \w -> Req c w) (enumerate ()) [1, 2],
        finite-map Ack (enumerate ())
      ]

  record Broker where
    load :: Nat

  dispatch :: Msg -> Broker -> Option Broker
  dispatch msg s = case msg of
    Req _ w ->
      if s.load + w <= 2
      then Some (Broker { load = s.load + w })
      else None
    Ack _ -> None

  machine BrokerBad where
    state = Broker
    init = Broker { load = 0 }
    actions = [dispatch]

  contract safety capOne for BrokerBad where
    always = (\s -> s.load <= 1)

  main :: Int
  main = 0
"#;
    let dir_bad = common::project("finite-adt-bad", &[
        ("Data.Finite.cinch", finite_mod),
        ("Main.cinch", bad_src),
    ]);
    let err_bad = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir_bad,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect_err("expected safety violation from Req _ 2")
    .to_string();
    assert!(err_bad.contains("temporal safety violation"), "got: {err_bad}");
    assert!(err_bad.contains("changed: load: 0 -> 2") || err_bad.contains("changed: load: 1 -> 2"), "got: {err_bad}");
    common::cleanup(&dir_bad);

    // 3. Soundness guard: `symmetry = [Node]` MUST be rejected when `Finite Node` is in scope!
    let bad_sym = r#"
module Main where
  import Data.Finite

  data Node where
    N0 :: Node
    N1 :: Node

  implicit instance Finite_Node :: Finite Node where
    enumerate _ = [N0, N1]

  record S where
    owner :: Option Node

  take :: Node -> S -> Option S
  take n s = case s.owner of
    None -> Some (S { owner = Some n })
    Some _ -> None

  machine UnsoundSym where
    state = S
    init = S { owner = None }
    actions = [take]
    symmetry = [Node]

  main :: Int
  main = 0
"#;
    let dir_sym = common::project("finite-symmetry-reject", &[
        ("Data.Finite.cinch", finite_mod),
        ("Main.cinch", bad_sym),
    ]);
    let err_sym = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir_sym,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect_err("symmetry with Finite instance in scope must be rejected")
    .to_string();
    assert!(err_sym.contains("unsound symmetry reduction"), "got: {err_sym}");
    assert!(err_sym.contains("Finite"), "got: {err_sym}");
    common::cleanup(&dir_sym);
}

// ----------------------------------------------------------------- 12. Effect.Test.Hegel Pure Library & Shrinking

#[test]
fn hegel_choice_stream_generators_and_state_machine_shrinking() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Finite
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Test.Hegel

  list-len-local :: forall a. List a -> Nat
  list-len-local xs = case xs of
    Nil -> 0
    Cons _ rest -> 1 + list-len-local rest

  sum-nats :: List Nat -> Nat
  sum-nats xs = case xs of
    Nil -> 0
    Cons x rest -> x + sum-nats rest

  -- Shared pure state-machine transition functions (same style as `machine` actions!)
  record Vault where
    balance :: Nat

  deposit :: Nat -> Vault -> Option Vault
  deposit amt s =
    if s.balance + amt <= 10
    then Some (Vault { balance = s.balance + amt })
    else None

  withdraw :: Vault -> Option Vault
  withdraw s =
    let b = s.balance in
    if b > 0 then Some (Vault { balance = b - 1 }) else None

  -- 1. Passing property over gen-finite + gen-pair
  check-pass :: Bool
  check-pass = case hegel-check (gen-pair (gen-finite :: Gen Bool) (gen-range 1 5)) (\p -> case p of (_, n) -> n >= 1 && n <= 5) of
    Ok trials -> trials == 200
    Err _ -> False

  -- 2. Failing list property shrinks automatically to length 3 and all zeros [0, 0, 0]!
  check-list-shrink :: Bool
  check-list-shrink = case hegel-check (gen-list 8 (gen-nat 50)) (\xs -> list-len-local xs < 3) of
    Ok _ -> False
    Err fail ->
      let shrunk = hegel-failure-shrunk fail in
      list-len-local shrunk == 3 && sum-nats shrunk == 0

  -- 3. Native-speed state-machine simulation finds balance >= 5 and shrinks trace to 1 step (`deposit` of 5)!
  check-machine-shrink :: Bool
  check-machine-shrink =
    let actions = [
          step-action-gen "deposit" (gen-range 1 5) deposit,
          step-action "withdraw" withdraw
        ] in
    case hegel-check-machine (Vault { balance = 0 }) actions (\s -> s.balance < 5) of
      Ok _ -> False
      Err fail ->
        let tr = machine-failure-trace fail in
        let fin = machine-failure-final-state fail in
        fin.balance >= 5 && list-len-local tr <= 2

  main :: Int
  main =
    if not check-pass then 1
    else if not check-list-shrink then 2
    else if not check-machine-shrink then 3
    else 0
"#;
    let dir = common::project("hegel-shrinking-and-sim", &[
        ("Data.Finite.cinch", include_str!("../prelude/Data/Finite.cinch")),
        ("Effect.Test.Hegel.cinch", include_str!("../prelude/Effect/Test/Hegel.cinch")),
        ("Main.cinch", main_src),
    ]);
    assert_eq!(
        common::run(&dir, "Main.cinch"),
        0,
        "exit 1=check-pass, 2=check-list-shrink, 3=check-machine-shrink, 0=all passed"
    );
    common::cleanup(&dir);
}
