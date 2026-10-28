//! Runtime evolution (P13) tests: `upgrade` declarations with checked
//! migration field mapping, record views and schema compatibility
//! evidence, and the versioned dispatch / state migration runtime
//! (runtime evolution specification sections 1-13; supercompilation and
//! JIT specification section 6).
//!
//! The compile-time items (migration checking, record views, interface
//! evidence) are driven through `check_sources`. The runtime items
//! (complete update sets, code retention with live frames, no old code
//! over new state, target compatibility before reachability, and
//! transparent tiering with consumption-safe deoptimization) are driven
//! end to end through compiled programs calling the `cinch_evo_*` entry
//! points of the runtime's `evolution` module.

use cinch::driver::{check_sources, check_sources_ifaces, run_source};
use cinch::interface::Iface;

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

// ------------------------------------------------- upgrade: migration (item 1)

#[test]
fn upgrade_with_a_checked_migration_migrates_old_state() {
    // `migrate : StateV1 %1 -> StateV2` is checked as the upgrade's field
    // mapping and, when run, produces the observable new view (runtime
    // evolution specification section 3).
    expect_ok(
        "migrate-observable",
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    base :: Nat
    delta :: Nat
  migrate :: StateV1 %1 -> StateV2
  migrate s = StateV2 { base = s.value, delta = 0 }
  upgrade StateV1 to StateV2 where
    migrate = migrate
  viewV2 :: StateV2 -> Nat
  viewV2 s = s.base + s.delta
  main :: Int
  main = if viewV2 (migrate (StateV1 { value = 5 })) == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn migration_type_mismatch_is_rejected() {
    // The migration must be exactly `Old %1 -> New`.
    expect_err(
        "migrate-wrong-type",
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    base :: Nat
    delta :: Nat
  bad :: StateV1 %1 -> StateV1
  bad s = s
  upgrade StateV1 to StateV2 where
    migrate = bad
  main :: Int
  main = 0
"#,
        "the migration must have type",
    );
}

#[test]
fn incompatible_change_without_migration_is_rejected() {
    // A retyped field is an incompatible change (runtime evolution
    // section 2): it needs a migration. A migration-free record view is
    // refused across it.
    expect_err(
        "retype-no-migration",
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    value :: Int
  upgrade StateV1 to StateV2 where
    related = StateV1
  main :: Int
  main = 0
"#,
        "incompatible field change `value`",
    );
}

// --------------------------------------- record views and compatibility (item 2)

#[test]
fn record_view_admits_compatible_additive_evolution() {
    // A migration-free record view across a purely additive change is
    // admitted (runtime evolution section 2: compatible changes are
    // admitted with record-view evidence).
    expect_ok(
        "view-additive",
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    value :: Nat
    extra :: Option Nat
  upgrade StateV1 to StateV2 where
    related = StateV1
  viewV1 :: StateV1 -> Nat
  viewV1 s = s.value
  main :: Int
  main = if viewV1 (StateV1 { value = 3 }) == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn record_view_preserves_missing_vs_none() {
    // The core rule: a missing field and a field holding `None` stay
    // distinguishable under a record view. The view of an old record
    // (which has no `extra`) marks `extra` missing-able; a new record may
    // hold `extra = None`. The interface evidence keeps the added field
    // distinct rather than collapsing it onto `None`.
    let iface = iface_of(
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    value :: Nat
    extra :: Option Nat
  upgrade StateV1 to StateV2 where
    related = StateV1
  main :: Int
  main = 0
"#,
    );
    let ev = iface
        .upgrades
        .iter()
        .find(|u| u.old == "StateV1" && u.new == "StateV2")
        .expect("the record view records its evidence");
    assert!(!ev.migrated, "a record view has no migration");
    assert_eq!(ev.added, vec![("extra".to_string(), "Option Nat".to_string())]);
    assert!(ev.retyped.is_empty(), "a compatible view retypes nothing");
}

// ------------------------------------ interface compatibility evidence (item 8)

#[test]
fn upgrade_evidence_records_the_migration_field_mapping() {
    // A checked migration records its schema compatibility evidence in
    // the module's interface (runtime evolution section 11): the added
    // fields and the incompatible fields the migration covers.
    let iface = iface_of(
        r#"
module Main where
  record StateV1 where
    value :: Nat
  record StateV2 where
    base :: Nat
    delta :: Nat
  migrate :: StateV1 %1 -> StateV2
  migrate s = StateV2 { base = s.value, delta = 0 }
  upgrade StateV1 to StateV2 where
    migrate = migrate
  main :: Int
  main = 0
"#,
    );
    let ev = iface
        .upgrades
        .iter()
        .find(|u| u.old == "StateV1" && u.new == "StateV2")
        .expect("the migration records its evidence");
    assert!(ev.migrated, "a migration checked the field mapping");
    let mut added: Vec<&str> = ev.added.iter().map(|(n, _)| n.as_str()).collect();
    added.sort_unstable();
    assert_eq!(added, vec!["base", "delta"], "the migration produces the new fields");
    assert!(ev.retyped.is_empty(), "no field is retyped across this evolution");
}

#[test]
fn repr_change_forces_dependent_replacement() {
    // A `#[repr]`-level change is a representation change (runtime
    // evolution section 2's dependency table): dependents that inlined
    // the old layout must be replaced. The layout is part of the
    // interface's contract projection, so a repr change alters the
    // interface identity and a dependent checked against the old identity
    // is stale (the `.cinchif` "checked against" chain extends to
    // versioned identities).
    let a = iface_of(
        r#"
module Main where
  #[repr(C)]
  record R where
    a :: U8
    b :: U32
  main :: Int
  main = 0
"#,
    );
    let b = iface_of(
        r#"
module Main where
  #[repr(packed)]
  record R where
    a :: U8
    b :: U32
  main :: Int
  main = 0
"#,
    );
    assert_ne!(
        a.layouts, b.layouts,
        "a repr change alters the recorded layout"
    );
    assert_ne!(
        a.iface_hash(),
        b.iface_hash(),
        "a repr change alters the interface identity, forcing dependent replacement"
    );
}

// --------------------------------------- runtime: versioned dispatch (item 3)

/// The `cinch_evo_*` binding contracts a runtime test program uses. Each
/// takes and returns `I64` words over the runtime's `Global` state.
const EVO_DECLS: &str = r#"
  foreign import ccall "cinch_evo_module_new" evoNew :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_new" evoVer :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_current_version" evoCur :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_update_begin" evoBeg :: I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_add_replacement" evoAdd :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_mark_complete" evoCmp :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_set_migration" evoMig :: I64 -> I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_activate" evoAct :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_feature_avail" evoFeat :: I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_frame_enter" evoEnt :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_frame_exit" evoExt :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_frame_version" evoFVer :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_frame_resume" evoFRes :: I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_live" evoLive :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_retired" evoRetd :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_resident" evoRes :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_try_drop" evoDrop :: I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_variant_install" evoVInst :: I64 -> I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_variant_enter" evoVEnt :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_variant_retire" evoVRet :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_variant_retired" evoVRetd :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_variant_resident" evoVRes :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_deopt" evoDeopt :: I64 -> I64 ->{Global, IO} I64
"#;

/// Run one runtime test body: a `main` whose result is its exit code.
fn run_evo(name: &str, body: &str, expected: i32) {
    let src = format!(
        "module Main where\n{EVO_DECLS}\n  main :: Unit ->{{Global, IO}} Int\n  main u =\n{body}\n"
    );
    expect_ok(name, &src, expected);
}

#[test]
fn incomplete_update_set_is_rejected_at_activation() {
    // Section 8: an incomplete update set is rejected at activation and
    // never claims a changed symbol updates already-inlined code.
    run_evo(
        "incomplete-set",
        r#"    let m = evoNew 0 in
    let v1 = evoVer m 0 0 in
    let b = evoBeg m v1 in
    let z1 = evoAdd b 0 v1 in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let v2 = evoVer m 1 0 in
    let u = evoBeg m v2 in
    let z4 = evoAdd u 1 v2 in
    let act = evoAct u in
    let cur = evoCur m in
    if act == -2 then (if cur == v1 then 0 else 1) else 2"#,
        0,
    );
}

#[test]
fn live_frames_keep_old_code_and_never_resume_over_new_state() {
    // Section 6: old code is retained while a live frame can enter it.
    // Section 10: old code never resumes over a new state representation
    // without an authorized reverse transition.
    run_evo(
        "retention-and-resume",
        r#"    let m = evoNew 0 in
    let v1 = evoVer m 0 0 in
    let b = evoBeg m v1 in
    let z1 = evoAdd b 0 v1 in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let f = evoEnt m in
    let fv = evoFVer f in
    let v2 = evoVer m 1 0 in
    let u = evoBeg m v2 in
    let z4 = evoAdd u 1 v2 in
    let z5 = evoMig u 0 1 0 in
    let z6 = evoCmp u 1 0 in
    let z7 = evoAct u in
    let live = evoLive v1 in
    let retd = evoRetd v1 in
    let res = evoFRes f 1 in
    if fv == v1 then
      (if live == 1 then
        (if retd == 1 then
          (if res == -4 then 0 else 4) else 3) else 2) else 1"#,
        0,
    );
}

#[test]
fn instruction_requirement_blocks_reachability() {
    // Section 13: a target with unmet instruction requirements never
    // becomes reachable; the dispatch is unchanged.
    run_evo(
        "target-compat",
        r#"    let z0 = evoFeat 6 0 in
    let m = evoNew 0 in
    let v1 = evoVer m 0 0 in
    let b = evoBeg m v1 in
    let z1 = evoAdd b 0 v1 in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let v2 = evoVer m 1 64 in
    let u = evoBeg m v2 in
    let z4 = evoAdd u 1 v2 in
    let z5 = evoCmp u 1 0 in
    let act = evoAct u in
    let cur = evoCur m in
    if act == -3 then (if cur == v1 then 0 else 1) else 2"#,
        0,
    );
}

#[test]
fn variants_install_retire_and_deopt_with_live_frames() {
    // Supercompilation section 6: a variant installs beside its generic
    // form, retires safely with live frames, and deoptimization is
    // consumption-safe (each `%1`/`%aff` obligation transfers once).
    run_evo(
        "variant-deopt",
        r#"    let m = evoNew 0 in
    let g = evoVer m 0 0 in
    let s = evoVer m 0 0 in
    let b = evoBeg m g in
    let z1 = evoAdd b 0 g in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let v = evoVInst m g s 7 in
    let f = evoVEnt v in
    let z4 = evoVRet v in
    let retd = evoVRetd v in
    let res = evoVRes v in
    let d = evoDeopt f 1 in
    let fv = evoFVer f in
    if retd == 1 then
      (if res == 1 then
        (if d == 0 then
          (if fv == g then 0 else 4) else 3) else 2) else 1"#,
        0,
    );
}

#[test]
fn deopt_refuses_an_unbalanced_linear_transfer() {
    // Supercompilation section 6: a deopt that cannot transfer every
    // linear obligation exactly once is refused, never partial.
    run_evo(
        "deopt-unbalanced",
        r#"    let m = evoNew 0 in
    let g = evoVer m 0 0 in
    let s = evoVer m 0 0 in
    let b = evoBeg m g in
    let z1 = evoAdd b 0 g in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let v = evoVInst m g s 1 in
    let f = evoVEnt v in
    let d = evoDeopt f 3 in
    let fv = evoFVer f in
    if d == -7 then (if fv == s then 0 else 1) else 2"#,
        0,
    );
}

/// The `.cinchif` of a single-module program: the compatibility evidence
/// and contract projection the "checked against" chain reads.
fn iface_of(src: &str) -> Iface {
    let (_program, ifaces) = check_sources_ifaces(src, "<test>").expect("the program checks");
    ifaces
        .into_iter()
        .find(|i| i.module == "Main")
        .expect("the entry module publishes an interface")
}
