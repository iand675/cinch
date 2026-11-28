//! Integration tests for per-instance fairness (`fair_each`, `strong_fair_each`),
//! `--model-fail-on-capped` / `CINCH_MODEL_FAIL_ON_CAPPED=1`, and large-graph iterative Tarjan SCC.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_opts, DriverOptions, Mode};
use cinch::model_cli;
use common::source_frontend::check_sources;
use std::path::PathBuf;

fn write_temp_project(tag: &str, src: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("sl_fair_each_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    dir
}

fn expect_ok_manifest(tag: &str, src: &str) -> String {
    let dir = write_temp_project(tag, src);
    if let Err(e) = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    ) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{tag}: expected check to succeed, but failed:\n{e}");
    }
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("{tag}: missing build/Main.manifest.json: {e}"));
    let _ = std::fs::remove_dir_all(&dir);
    manifest
}

fn expect_err(tag: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<model-fair-each-test>") {
        Ok(_) => panic!("{tag}: expected check to fail with {needles:?}, but it succeeded"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(
                    msg.contains(n),
                    "{tag}: expected error containing {n:?}, got:\n{msg}"
                );
            }
        }
    }
}

#[test]
fn two_actor_starvation_fails_under_fair_and_passes_under_fair_each() {
    let base = r#"
module Main where
  data Actor where
    A :: Actor
    B :: Actor

  record S where
    aTick :: Bool
    bDone :: Bool

  step :: Actor -> S -> Option S
  step actor s = case actor of
    A -> Some (S { aTick = not s.aTick, bDone = s.bDone })
    B -> if not s.bDone then Some (S { aTick = s.aTick, bDone = True }) else None
"#;

    // 1. Action-level `fair = [step]` FAILS: actor A runs `step(A)` forever,
    // satisfying WF(step) while starving actor B (`step(B)`) forever.
    let fair_only = format!(
        "{base}
  machine TwoActors where
    state = S
    init = S {{ aTick = False, bDone = False }}
    actions = [step]
    fair = [step]

  contract liveness bEventuallyRuns for TwoActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)

  main :: Int
  main = 0
"
    );
    expect_err(
        "fair-allows-actor-b-starvation",
        &fair_only,
        &["temporal liveness violation", "fair cycle", "step(A)"],
    );

    // 2. Machine-level `fair_each = [step]` PASSES: WF(step(B)) rejects any
    // cycle where `step(B)` is continuously enabled without firing.
    let fair_each_machine = format!(
        "{base}
  machine TwoActors where
    state = S
    init = S {{ aTick = False, bDone = False }}
    actions = [step]
    fair_each = [step]

  contract liveness bEventuallyRuns for TwoActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)

  main :: Int
  main = 0
"
    );
    let manifest = expect_ok_manifest("fair-each-machine-discharges", &fair_each_machine);
    assert!(
        manifest.contains("fair_each step"),
        "expected manifest to record `fair_each step`, got:\n{manifest}"
    );

    // 3. Per-contract `fair_each = [step]` override PASSES even when the machine
    // itself declares no fairness.
    let fair_each_contract = format!(
        "{base}
  machine TwoActors where
    state = S
    init = S {{ aTick = False, bDone = False }}
    actions = [step]

  contract liveness bEventuallyRuns for TwoActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)
    fair_each = [step]

  main :: Int
  main = 0
"
    );
    let manifest_c = expect_ok_manifest("fair-each-contract-override", &fair_each_contract);
    assert!(
        manifest_c.contains("fair_each step"),
        "expected manifest to record `fair_each step` from contract override, got:\n{manifest_c}"
    );

    // 4. `cinch model inspect` (text and `--json`) succeeds and records `fair_each`.
    let dir = write_temp_project("inspect-fair-each", &fair_each_machine);
    let file = dir.join("Main.cinch");
    let file_str = file.to_string_lossy().to_string();
    assert_eq!(model_cli::cli(&["inspect".into(), file_str.clone()]), 0);
    assert_eq!(
        model_cli::cli(&["inspect".into(), file_str, "TwoActors".into(), "--json".into()]),
        0
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn strong_fair_each_discharges_intermittent_per_actor_liveness_where_fair_each_fails() {
    let base = r#"
module Main where
  data Actor where
    A :: Actor
    B :: Actor

  record S where
    gate :: Bool
    bDone :: Bool

  step :: Actor -> S -> Option S
  step actor s = case actor of
    A -> Some (S { gate = not s.gate, bDone = s.bDone })
    B -> if s.gate && not s.bDone then Some (S { gate = False, bDone = True }) else None
"#;

    // 1. Weak per-actor fairness (`fair_each = [step]`) FAILS: `step(B)` is disabled
    // whenever `gate == False`, so toggling `gate` via `step(A)` is a WF(step(B))-fair lasso!
    let weak_each = format!(
        "{base}
  machine GatedActors where
    state = S
    init = S {{ gate = False, bDone = False }}
    actions = [step]
    fair_each = [step]

  contract liveness bEventuallyRuns for GatedActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)

  main :: Int
  main = 0
"
    );
    expect_err(
        "weak-fair-each-fails-intermittent-actor",
        &weak_each,
        &["temporal liveness violation", "fair cycle", "step(A)"],
    );

    // 2. Action-level `strong_fair = [step]` ALSO FAILS: `step(A)` fires in the cycle,
    // satisfying SF(step) while starving `step(B)`!
    let strong_action_only = format!(
        "{base}
  machine GatedActors where
    state = S
    init = S {{ gate = False, bDone = False }}
    actions = [step]
    strong_fair = [step]

  contract liveness bEventuallyRuns for GatedActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)

  main :: Int
  main = 0
"
    );
    expect_err(
        "strong-fair-action-still-starves-actor-b",
        &strong_action_only,
        &["temporal liveness violation", "fair cycle", "step(A)"],
    );

    // 3. Machine-level `strong_fair_each = [step]` PASSES: SF(step(B)) prunes all
    // `gate == True` states where `step(B)` is enabled, eliminating every violating cycle!
    let strong_each_machine = format!(
        "{base}
  machine GatedActors where
    state = S
    init = S {{ gate = False, bDone = False }}
    actions = [step]
    strong_fair_each = [step]

  contract liveness bEventuallyRuns for GatedActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)

  main :: Int
  main = 0
"
    );
    let manifest = expect_ok_manifest("strong-fair-each-machine", &strong_each_machine);
    assert!(
        manifest.contains("strong_fair_each step"),
        "expected manifest to record `strong_fair_each step`, got:\n{manifest}"
    );

    // 4. Per-contract `strong_fair_each = [step]` override PASSES as well.
    let strong_each_contract = format!(
        "{base}
  machine GatedActors where
    state = S
    init = S {{ gate = False, bDone = False }}
    actions = [step]

  contract liveness bEventuallyRuns for GatedActors where
    leads_to = (\\s -> not s.bDone) ~> (\\s -> s.bDone)
    strong_fair_each = [step]

  main :: Int
  main = 0
"
    );
    let manifest_c = expect_ok_manifest("strong-fair-each-contract", &strong_each_contract);
    assert!(
        manifest_c.contains("strong_fair_each step"),
        "expected manifest to record `strong_fair_each step`, got:\n{manifest_c}"
    );
}

#[test]
fn model_fail_on_capped_gate_rejects_capped_and_accepts_uncapped() {
    let capped_src = r#"
module Main where
  record Counter where
    n :: Nat

  inc :: Counter -> Option Counter
  inc s = if s.n < 7 then Some (Counter { n = s.n + 1 }) else None

  machine SmallCap where
    state = Counter
    init = Counter { n = 0 }
    actions = [inc]
    max_states = 4

  contract safety withinBound for SmallCap where
    always = (\s -> s.n <= 7)

  main :: Int
  main = 0
"#;

    let uncapped_src = capped_src.replace("max_states = 4", "max_states = 16");

    // 1. Without `--model-fail-on-capped`, capped exploration succeeds with a bounded receipt.
    let dir_capped = write_temp_project("capped-default", capped_src);
    let entry_capped = dir_capped.join("Main.cinch");
    let default_opts = DriverOptions::default();
    assert!(
        driver_opts(Mode::Check, &entry_capped, None, &default_opts).is_ok(),
        "default check should allow capped machine"
    );

    // 2. With `--model-fail-on-capped`, capped exploration fails with a clear diagnostic.
    let mut strict_opts = DriverOptions::default();
    strict_opts.model_fail_on_capped = true;
    let err = driver_opts(Mode::Check, &entry_capped, None, &strict_opts)
        .expect_err("expected --model-fail-on-capped to fail on capped machine");
    let msg = err.to_string();
    assert!(
        msg.contains("SmallCap") && msg.contains("max_states = 4"),
        "unexpected capped error: {msg}"
    );

    // 3. `cinch model inspect` without vs with `--model-fail-on-capped`.
    let capped_str = entry_capped.to_string_lossy().to_string();
    assert_eq!(model_cli::cli(&["inspect".into(), capped_str.clone()]), 0);
    assert_eq!(
        model_cli::cli(&[
            "inspect".into(),
            capped_str.clone(),
            "--model-fail-on-capped".into(),
        ]),
        1
    );

    // 4. CLI binary with `CINCH_MODEL_FAIL_ON_CAPPED=1` fails on capped machine.
    let bin = env!("CARGO_BIN_EXE_cinch");
    let env_check = std::process::Command::new(bin)
        .args(["check", &capped_str])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch check with CINCH_MODEL_FAIL_ON_CAPPED=1");
    assert!(
        !env_check.status.success(),
        "expected CINCH_MODEL_FAIL_ON_CAPPED=1 to fail `cinch check`"
    );
    let stderr_text = String::from_utf8_lossy(&env_check.stderr);
    assert!(
        stderr_text.contains("max_states = 4"),
        "expected cap diagnostic on stderr, got:\n{stderr_text}"
    );

    let env_inspect = std::process::Command::new(bin)
        .args(["model", "inspect", &capped_str])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect with CINCH_MODEL_FAIL_ON_CAPPED=1");
    assert!(
        !env_inspect.status.success(),
        "expected CINCH_MODEL_FAIL_ON_CAPPED=1 to fail `cinch model inspect`"
    );

    // 5. Uncapped machine succeeds under `--model-fail-on-capped` and `CINCH_MODEL_FAIL_ON_CAPPED=1`.
    let dir_uncapped = write_temp_project("uncapped-strict", &uncapped_src);
    let entry_uncapped = dir_uncapped.join("Main.cinch");
    let uncapped_str = entry_uncapped.to_string_lossy().to_string();
    assert!(
        driver_opts(Mode::Check, &entry_uncapped, None, &strict_opts).is_ok(),
        "uncapped machine should pass under --model-fail-on-capped"
    );
    assert_eq!(
        model_cli::cli(&[
            "inspect".into(),
            uncapped_str.clone(),
            "--model-fail-on-capped".into(),
        ]),
        0
    );
    let env_uncapped = std::process::Command::new(bin)
        .args(["check", &uncapped_str])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch check on uncapped machine with CINCH_MODEL_FAIL_ON_CAPPED=1");
    assert!(
        env_uncapped.status.success(),
        "uncapped machine should pass with CINCH_MODEL_FAIL_ON_CAPPED=1, stderr:\n{}",
        String::from_utf8_lossy(&env_uncapped.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir_capped);
    let _ = std::fs::remove_dir_all(&dir_uncapped);
}

#[test]
fn large_max_states_budget_and_deep_chain_avoid_stack_overflow() {
    // Deep linear chain of 2000 states with `max_states = 262144`:
    // exercises high `max_states` cap, O(N) diameter calculation, and iterative Tarjan SCC.
    let src = r#"
module Main where
  record Step where
    k :: Nat

  advance :: Step -> Option Step
  advance s = if s.k < 2000 then Some (Step { k = s.k + 1 }) else None

  stay :: Step -> Option Step
  stay s = if s.k == 2000 then Some s else None

  machine DeepChain where
    state = Step
    init = Step { k = 0 }
    actions = [advance, stay]
    fair = [advance]
    max_states = 262144

  contract safety inRange for DeepChain where
    always = (\s -> s.k <= 2000)

  contract liveness reachesEnd for DeepChain where
    eventually = (\s -> s.k == 2000)

  main :: Int
  main = 0
"#;
    let dir = write_temp_project("deep-chain-262144", src);
    let entry = dir.join("Main.cinch");
    let mut opts = DriverOptions::default();
    opts.model_fail_on_capped = true;
    driver_opts(Mode::Check, &entry, None, &opts)
        .expect("deep chain with max_states = 262144 should succeed cleanly without capping or stack overflow");
    let _ = std::fs::remove_dir_all(&dir);
}
