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

use cinch::check::{check_program_multi_full, StageReport, StagingCx};
use cinch::core::TargetProfile;
use cinch::driver::{check_sources, parse_prelude, parse_source};
use std::collections::HashMap;

fn expect_ok(name: &str, src: &str) -> StageReport {
    let prelude = parse_prelude(&[]).expect("prelude");
    let user = parse_source(src, "<temporal-machine-test>", &prelude.base, &HashMap::new())
        .unwrap_or_else(|e| panic!("{name}: parse failed:\n{e}"));
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<temporal-machine-test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    match check_program_multi_full(
        &prelude.modules,
        &user.modules,
        &hashes,
        &scx,
        &[],
        &TargetProfile::native(),
    ) {
        Ok((_p, _i, report, _d)) => report,
        Err(e) => panic!("{name}: expected check to succeed, but failed:\n{e}"),
    }
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
    let rep = expect_ok("worker-pool-safety", ok_src);
    let row = rep
        .temporal
        .iter()
        .find(|r| r.contract == "boundInv")
        .expect("boundInv temporal row recorded");
    assert!(row.result.contains("model-checked"), "got: {}", row.result);
    assert!(row.result.contains("0 violations"), "got: {}", row.result);

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
    let rep = expect_ok("symmetry-and-por", ok_src);
    let row = rep
        .temporal
        .iter()
        .find(|r| r.contract == "boundedCounts")
        .expect("boundedCounts row");
    assert!(
        row.result.contains("symmetry_merged: ") && !row.result.contains("symmetry_merged: 0"),
        "expected symmetry reduction to merge symmetric states, got: {}",
        row.result
    );
    assert!(
        row.result.contains("por_independent_pairs: ") && !row.result.contains("por_independent_pairs: 0"),
        "expected POR to detect disjoint field footprints, got: {}",
        row.result
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
    let rep = expect_ok("refines-valid-stuttering", ok_src);
    let row = rep
        .temporal
        .iter()
        .find(|r| r.kind == "refinement")
        .expect("refinement row");
    assert!(row.result.contains("stuttering simulation"), "got: {}", row.result);
    assert!(row.result.contains("measure strictly decreasing"), "got: {}", row.result);

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

    // Mismatched clocks `k1` vs `k2` in `laterAp` must be rejected!
    let mismatch_src = r#"
module Main where
  badMix :: forall k1 k2. Later k1 (Nat -> Nat) -> Later k2 Nat -> Later k1 Nat
  badMix lf lx = laterAp lf lx

  main :: Int
  main = 0
"#;
    expect_err("clock-mismatch", mismatch_src, &["regions do not match"]);

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
