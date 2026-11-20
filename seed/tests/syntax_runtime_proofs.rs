//! Generated and captured evidence retains its runtime value and erasure scope.
mod common;
use common::source_frontend::{check_sources, run_source};

const BASE: &str = include_str!("fixtures/syntax/runtime-proof-bindings.cinch");

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}

#[track_caller]
fn refuses(source: &str, reason: &str) {
    let error = check_sources(source, "runtime-proof-bindings.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(reason), "{error}");
}

fn captured() -> String {
    BASE.replace("  stage family :: Declarations", r#"  macro hide :: forall a. Syntax -> Name -> Code a
  macro hide syntax name = let binding = Syntax.newBinding name in Syntax.letIn binding [| False |] (Syntax.capture syntax)
  stage family :: Declarations"#)
        .replace("Refined value witness", "Refined value (hide!((witness), witness))")
}

fn surrounded(rhs: &str) -> String {
    BASE.replace("  stage family :: Declarations", r#"  macro surround :: forall a. Syntax -> Name -> Syntax -> Code a
  macro surround evidence name input = let binding = Syntax.newBinding name in Syntax.letIn binding (Syntax.capture input) (Syntax.capture evidence)
  stage family :: Declarations"#)
        .replace("Refined value witness", &format!("Refined value (surround!((witness), ignored, ({rhs})))"))
}

#[test]
fn runtime_decision_witnesses_keep_their_subject_across_group_instances() {
    run(BASE);
}

#[test]
fn captured_witness_survives_a_same_spelled_generated_boolean_binder() {
    run(&captured());
}

#[test]
fn proof_lets_cannot_retarget_evidence_to_another_runtime_value() {
    refuses(&BASE.replace("Refined value witness", "Refined candidate witness"), "not of");
    refuses(&captured().replace("Refined value (hide!", "Refined candidate (hide!"), "not of");
}

#[test]
fn erased_proof_lets_refuse_effects_even_when_the_caller_permits_them() {
    let source = surrounded("pulse 0")
        .replace("  stage family :: Declarations", "  foreign import ccall \"cinch_proof_effect\" pulse :: I64 ->{Foreign Probe} I64\n  stage family :: Declarations")
        .replace("Nat -> Nat -> Option Refined", "Nat -> Nat ->{Foreign Probe} Option Refined")
        .replace("  main :: Int\n", "  main :: Unit ->{Foreign Probe} Int\n")
        .replace("  main =", "  main unit =");
    refuses(&source, "an erased proof let must be pure");
}

#[test]
fn erased_proof_lets_cannot_launder_a_linear_runtime_use() {
    let source = surrounded("if token then False else False")
        .replace("Nat -> Nat -> Option Refined", "Nat -> Nat -> Bool %1 -> Option Refined")
        .replace("make value candidate =", "make value candidate token =")
        .replace("No _ -> None", "No _ -> if token then None else None")
        .replace("small-build 3 9", "small-build 3 9 True")
        .replace("small-build 9 3", "small-build 9 3 True")
        .replace("other-build 4 8", "other-build 4 8 False")
        .replace("other-build 8 4", "other-build 8 4 False");
    refuses(&source, "an erased proof let cannot consume a linear or affine runtime value");
    run(&source.replace("Bool %1 ->", "Bool ->"));
}

#[test]
fn proof_local_scopes_keep_termination_and_ghost_boundaries() {
    refuses(r#"module Main where
  proof cycle :: (value :: Nat) -> Eq value 0
  proof cycle value = let alias = cycle value in alias
  main :: Int
  main = 42
"#, "must terminate");
    let source = captured().replace("  stage family :: Declarations", "  proof known :: Eq 3 3\n  proof known = refl\n  stage family :: Declarations")
        .replace("Some (Refined value (hide!((witness), witness)))",
            "let constructed = Some (Refined value (hide!((witness), witness))) in let leaked = known in constructed");
    refuses(&source, "erased ghost code");
}

#[test]
fn recursive_proof_locals_keep_existing_evidence_and_reject_eager_cycles() {
    run(&BASE.replace("Refined value witness", "Refined value (letrec alias = witness in alias)"));
    refuses(&BASE.replace("Refined value witness", "Refined value (letrec alias = alias in alias)"), "unguarded recursion");
}

#[test]
fn proposition_results_require_termination_without_a_proof_modifier() {
    // This used to check, build, and return 42: `cycle 9` was erased,
    // manufacturing a member of the false refinement Zero at runtime.
    refuses(r#"module Main where
  cycle :: (value :: Nat) -> Eq value 0
  cycle value = cycle value
  type Zero = refine (value :: Nat) with Eq value 0
  make :: Nat -> Zero
  make value = Zero value (cycle value)
  main :: Int
  main = if unrefine (make 9) == 9 then 42 else 0
"#, "must terminate");
    refuses(r#"module Main where
  first :: (value :: Nat) -> Eq value 0
  first value = second value
  second :: (value :: Nat) -> Eq value 0
  second value = first value
  main :: Int
  main = 42
"#, "mutually recursive");
    run(r#"module Main where
  identity :: (value :: Nat) -> Eq value value
  identity value = refl
  type Same = refine (value :: Nat) with Eq value value
  make :: Nat -> Same
  make value = Same value (identity value)
  main :: Int
  main = if unrefine (make 9) == 9 then 42 else 0
"#);
}

#[test]
fn proof_admission_rejects_computational_bottom_and_stage_only_evidence() {
    refuses(r#"module Main where
  stage known :: Eq 3 3
  stage known = refl
  type Same = refine (value :: Nat) with Eq value value
  make :: Nat -> Same
  make value = Same value known
  main :: Int
  main = 42
"#, "compile-time definition is not runtime evidence");
    refuses(r#"module Main where
  bottom :: forall a. Unit -> a
  bottom unit = bottom unit
  type Zero = refine (value :: Nat) with Eq value 0
  make :: Nat -> Zero
  make value = Zero value (let forged :: Eq value 0; forged = bottom () in forged)
  main :: Int
  main = 42
"#, "checked, total logical fragment");
}

#[test]
fn erased_runtime_bindings_need_checked_total_bodies_including_forward_references() {
    let looping = surrounded("loop True").replace("  stage family :: Declarations", r#"  loop :: Bool -> Bool
  loop flag = loop flag
  stage family :: Declarations"#);
    refuses(&looping, "must terminate");
    let total = surrounded("total True").replace("  main :: Int", r#"  total :: Bool -> Bool
  total flag = flag
  main :: Int"#);
    run(&total);
}

#[test]
fn erased_destructuring_cannot_extract_evidence_from_a_looping_container() {
    refuses(r#"module Main where
  bottom :: forall a. Unit -> a
  bottom unit = bottom unit
  type Zero = refine (value :: Nat) with Eq value 0
  make :: Nat -> Zero
  make value = Zero value (let (forged, ignored) = (bottom () :: (Eq value 0, Bool)) in forged)
  main :: Int
  main = 42
"#, "must terminate");
}

#[test]
fn erased_bindings_check_generic_data_and_actual_callback_bodies() {
    let data = surrounded("identity (Payload True)").replace("  main :: Int", r#"  data Payload where
    Payload :: Bool -> Payload
  identity :: forall a. a -> a
  identity value = value
  main :: Int"#);
    run(&data);
    let callback = surrounded("apply (\\flag -> flag) True").replace("  main :: Int", r#"  apply :: (Bool -> Bool) -> Bool -> Bool
  apply function flag = function flag
  main :: Int"#);
    run(&callback);
    let looping = surrounded("apply (\\flag -> loop flag) True").replace("  main :: Int", r#"  apply :: (Bool -> Bool) -> Bool -> Bool
  apply function flag = function flag
  loop :: Bool -> Bool
  loop flag = loop flag
  main :: Int"#);
    refuses(&looping, "must terminate");
}
