//! Experimental goal-directed elaboration must preserve ordinary evidence rules.
use cinch::driver::check_sources;

fn checks(body: &str) {
    let src = format!("module Main where\n{body}\n  main :: Int\n  main = 0\n");
    if let Err(e) = check_sources(&src, "<tactic-test>") {
        panic!("expected acceptance: {e}");
    }
}

fn refuses(body: &str, needle: &str) {
    let src = format!("module Main where\n{body}\n  main :: Int\n  main = 0\n");
    let err = match check_sources(&src, "<tactic-test>") {
        Ok(_) => panic!("expected rejection containing {needle:?}"),
        Err(e) => e.to_string(),
    };
    assert!(err.contains(needle), "expected {needle:?}, got {err}");
}

#[test]
fn actual_expected_goals_and_multiple_facts_drive_search() {
    checks(r#"
  safe :: (i :: Nat) -> Nat -> (n :: Nat) -> Option (Fin n)
  safe i j n = if i <= j then
    if j + 1 <= n then Some (Fin i (Proof.search "assumption linear")) else None
    else None
  direct :: (i :: Nat) -> (n :: Nat) -> Option (Fin n)
  direct i n = case decide (i < n) of
    Yes bound -> Some (Fin i (Proof.search "assumption"))
    No _ -> None
"#);
}

#[test]
fn a_failed_assumption_does_not_solve_an_arithmetic_goal() {
    refuses(r#"
  safe :: (i :: Nat) -> (n :: Nat) -> Option (Fin n)
  safe i n = if i + 1 <= n then Some (Fin i (Proof.search "assumption")) else None
"#, "could not prove");
}

#[test]
fn missing_and_false_goals_are_refused() {
    refuses(r#"
  unsafe :: (i :: Nat) -> (n :: Nat) -> Fin n
  unsafe i n = Fin i (Proof.search "linear")
"#, "could not prove");
    refuses(r#"
  unsafe :: Fin 5
  unsafe = Fin 5 (Proof.search "linear")
"#, "obligation disproved");
}

#[test]
fn rebound_indices_cannot_reuse_old_facts() {
    refuses(r#"
  unsafe :: Nat -> (n :: Nat) -> Option (Fin n)
  unsafe i n = case decide (i < n) of
    Yes bound -> let i = n in Some (Fin i (Proof.search "assumption linear"))
    No _ -> None
"#, "could not prove");
}

#[test]
fn assumption_matches_free_local_indices_without_alpha_renaming() {
    for fact in ["n < i", "j < n"] {
        refuses(&format!(r#"
  unsafe :: (i :: Nat) -> Nat -> (n :: Nat) -> Option (Fin n)
  unsafe i j n = case decide ({fact}) of
    Yes bound -> Some (Fin i (Proof.search "assumption"))
    No _ -> None
"#), "could not prove");
    }
}

#[test]
fn tactics_preserve_bootstrap_value_identity_across_aliases_and_shadowing() {
    for (fact, plan) in [("i < n", "assumption"), ("i + 1 <= n", "linear")] {
        checks(&format!(r#"
  safe :: (i :: Nat) -> (n :: Nat) -> Option (Fin n)
  safe i n = case decide ({fact}) of
    Yes bound -> let saved = i in let i = n in Some (Fin saved (Proof.search "{plan}"))
    No _ -> None
"#));
    }
}

#[test]
fn arithmetic_tactics_keep_opaque_measure_goals_in_the_facts_scope() {
    for (fact, plan) in [("i < string-length name", "assumption"),
                         ("i + 1 <= string-length name", "linear")] {
        checks(&format!(r#"
  safe :: (name :: String) -> (i :: Nat) -> Option (Fin (string-length name))
  safe name i = if {fact} then Some (Fin i (Proof.search "{plan}")) else None
"#));
    }
}

#[test]
fn both_strategies_record_mutable_evidence_dependencies() {
    for (fact, plan) in [("copyOut m < 5", "assumption"), ("copyOut m + 1 <= 5", "linear")] {
        let body = format!(r#"
  unsafe :: forall region. Mut region Nat -> Option (Fin 5)
  unsafe m = case decide ({fact}) of
    Yes bound -> let u = refSet m 9 in Some (Fin (copyOut m) (Proof.search "{plan}"))
    No _ -> None
"#);
        refuses(&body, "stale evidence");
    }
}

#[test]
fn plans_are_literal_validated_and_limited_before_execution() {
    for plan in ["", "assumption bogus", "linear linear linear linear linear linear linear linear linear"] {
        refuses(&format!(r#"
  safe :: (i :: Nat) -> (n :: Nat) -> Option (Fin n)
  safe i n = case decide (i < n) of
    Yes bound -> Some (Fin i (Proof.search "{plan}"))
    No _ -> None
"#), "tactic plan needs");
    }
    refuses(r#"
  unsafe :: String -> Fin 5
  unsafe plan = Fin 2 (Proof.search plan)
"#, "one literal tactic plan");
}

#[test]
fn the_elaboration_hook_cannot_escape_into_runtime_or_inference() {
    refuses(r#"
  unsafe :: Unit
  unsafe = Proof.search "linear"
"#, "needs an expected proposition");
    refuses(r#"
  unsafe :: Unit
  unsafe = let attempt = Proof.search in ()
"#, "needs an expected proposition");
}

#[test]
fn arithmetic_certificate_checker_refuses_changed_goal_facts_and_empty_certificate() {
    use cinch::types::{check_linear_proof, linear_entails, DiffCert, ITerm, IdxCmp, IntSort, PTerm, Sorts};
    let i = ITerm::Var("i".into());
    let n = ITerm::Var("n".into());
    let goal = PTerm::Cmp(IdxCmp::Lt, Box::new(i.clone()), Box::new(n.clone()));
    let hyps = vec![goal.clone()];
    let sorts = Sorts { vars: vec![("i".into(), IntSort::Nat), ("n".into(), IntSort::Nat)], fns: vec![] };
    let proof = linear_entails(&hyps, &goal, &sorts).expect("a checked certificate").remove(0);
    assert!(check_linear_proof(&hyps, &proof, &sorts));
    assert!(!check_linear_proof(&[], &proof, &sorts));
    let mut changed = proof.clone();
    changed.goal = PTerm::Cmp(IdxCmp::Lt, Box::new(n), Box::new(i));
    assert!(!check_linear_proof(&hyps, &changed, &sorts));
    changed = proof;
    changed.cert = DiffCert::Cycle(vec![]);
    assert!(!check_linear_proof(&hyps, &changed, &sorts));
}
