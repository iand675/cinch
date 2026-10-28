//! `prim` declarations with inline target-IR bodies (primitive
//! operations and inline target IR specification sections 2-8): the
//! per-family bodies compile, containment validation names its
//! violations, the `clr`/`jvm` slots are reserved, and capability-scoped
//! body selection is recorded in the artifact manifest only (never in
//! the contract projection). Evidence is IR observation and manifest
//! observation (design/llvm-contract-attributes.md), never wiring
//! tests.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{check_sources, check_sources_profile, driver_with_profile, run_source, Mode};
use std::path::PathBuf;

/// The `prim` acceptance fixture: one prim with `llvm` and `asm` bodies
/// carrying immediate parameters.
const BODIES: &str = include_str!("fixtures/prim/bodies.cinch");

/// The emitted LLVM IR of one checked source (every IR test carries a
/// `main`, which code generation requires).
fn ir_of(name: &str, src: &str) -> String {
    let program =
        check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// The one line of `ir` defining one unit (`define ...`).
fn define_of(ir: &str, unit: &str) -> String {
    let quoted = format!("@\"{unit}\"(");
    ir.lines()
        .find(|l| l.starts_with("define ") && l.contains(&quoted))
        .unwrap_or_else(|| panic!("{unit}: no define in {ir}"))
        .to_string()
}

/// The body of one unit's define (its lines between `{` and `}`).
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
    panic!("{unit}: no body in {ir}")
}

/// Assert that checking the program fails with a message containing
/// `needle`.
fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, name) {
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

/// Write one source to a fresh temporary project and return its
/// directory.
fn write_project(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch-prim-{}-{name}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    dir
}

/// The artifact manifest of one checked source under `profile`.
fn manifest_of(name: &str, src: &str, profile: &TargetProfile) -> String {
    let dir = write_project(name, src);
    driver_with_profile(Mode::Check, StageMode::Refresh, &dir.join("Main.cinch"), None, profile)
        .unwrap_or_else(|e| panic!("{name}: check run failed: {e}"));
    std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .unwrap_or_else(|e| panic!("{name}: manifest missing: {e}"))
}

// ------------------------------------- the bodies check and compile

#[test]
fn prim_with_llvm_and_asm_bodies_checks_and_compiles() {
    // The fixture checks, compiles and runs: the `llvm` body is the
    // artifact's selected body.
    let code = run_source(BODIES).expect("the prim compiles and runs");
    assert_eq!(code, 0);

    // IR observation: the immediate parameter carries `immarg` on the
    // unit it belongs to (LLVM admits the `immarg` parameter attribute
    // only on intrinsics, so the contract rides the unit as `!immarg`
    // metadata naming the immediate parameters; design/
    // llvm-contract-attributes.md).
    let ir = ir_of("bodies", BODIES);
    let llvm_define = define_of(&ir, "Main.add2.llvm");
    assert!(llvm_define.contains("immarg"), "{llvm_define}");
    assert!(body_of(&ir, "Main.add2.llvm").contains("add i32"), "{ir}");

    // The `asm` body carries `llvm.sideeffect`: the marker call keeping
    // it alive and the `sideeffect` inline-asm flag on the fragment.
    let asm_define = define_of(&ir, "Main.add2.asm");
    assert!(asm_define.contains("immarg"), "{asm_define}");
    let asm_body = body_of(&ir, "Main.add2.asm");
    assert!(asm_body.contains("llvm.sideeffect"), "{asm_body}");
    assert!(asm_body.contains("asm sideeffect"), "{asm_body}");
}

#[test]
fn intrinsic_families_migrate_onto_prim_bodies() {
    // The intrinsic migration path (section 7): an operation the closed
    // `PrimOp` set lowers by hand (`checkedAddI64` becomes
    // `llvm.sadd.with.overflow.i64`) is expressible as a `prim` with the
    // same inline body, so the intrinsic families move onto `prim`
    // declarations without a foreign call surface.
    let src = r#"
module Main where
  prim addWrap :: I64 -> I64 -> I64 where
    llvm {
      %o = call { i64, i1 } @llvm.sadd.with.overflow.i64(i64 %0, i64 %1)
      %v = extractvalue { i64, i1 } %o, 0
      ret i64 %v
    }

  main :: Int
  main =
    if addWrap 40 2 == 42 then 0 else 1
"#;
    let code = run_source(src).expect("the prim reproduces the intrinsic's inline lowering");
    assert_eq!(code, 0);
}

// --------------------------------------------- containment validation

#[test]
fn escaping_body_fails_containment_naming_no_escape() {
    expect_err(
        "escape",
        r#"
module Main where
  prim leak :: U32 -> U32 where
    llvm {
      %v = call i32 @sneaky(i32 %0)
      ret i32 %v
    }

  main :: Int
  main = 0
"#,
        "no-escape",
    );
}

#[test]
fn extra_parameters_fail_containment_naming_signature_discipline() {
    expect_err(
        "extra-params",
        r#"
module Main where
  prim wide :: U32 -> U32 where
    llvm {
      define i32 @extra(i32 %x, i32 %y) {
        %s = add i32 %x, %y
        ret i32 %s
      }
      ret i32 %0
    }

  main :: Int
  main = 0
"#,
        "signature discipline",
    );
}

#[test]
fn unwinding_body_fails_containment_naming_no_unwind() {
    expect_err(
        "unwind",
        r#"
module Main where
  prim raises :: U32 -> U32 where
    llvm {
      %v = invoke i32 @f(i32 %0) to label %ok unwind label %bad
      ret i32 %v
    }

  main :: Int
  main = 0
"#,
        "no-unwind",
    );
}

#[test]
fn effectful_body_fails_containment_naming_effects() {
    expect_err(
        "effects",
        r#"
module Main where
  prim writes :: U32 -> U32 where
    llvm {
      store i32 %0, ptr %1
      ret i32 %0
    }

  main :: Int
  main = 0
"#,
        "determinism of effects",
    );
}

// ------------------------------------------------- reserved families

#[test]
fn clr_and_jvm_body_slots_are_reserved() {
    expect_err(
        "clr",
        r#"
module Main where
  prim net :: U32 -> U32 where
    clr {
      ldarg.0
    }

  main :: Int
  main = 0
"#,
        "reserved",
    );
    expect_err(
        "jvm",
        r#"
module Main where
  prim jit :: U32 -> U32 where
    jvm {
      iload 1
    }

  main :: Int
  main = 0
"#,
        "reserved",
    );
}

// -------------------------------------- capability-scoped selection

#[test]
fn capability_scoped_selection_is_manifest_only() {
    // Two `llvm` bodies distinguished only by their `when` condition:
    // the same contract, different instruction shapes. Which one runs
    // is selection: fixed per artifact, recorded in the manifest, and
    // never observable from the source.
    let src = r#"
module Main where
  prim pick :: U32 -> U32 where
    llvm {
      %s = add i32 1, %0
      ret i32 %s
    } when Available VecU32Target
    llvm {
      %s = add i32 %0, 1
      ret i32 %s
    }

  main :: Int
  main =
    if pick 41 == 42 then 0 else 1
"#;

    // The baseline rule: without the capability the baseline body is
    // the fallback, and the manifest names it.
    let baseline = manifest_of("pick-baseline", src, &TargetProfile::native());
    assert!(baseline.contains("\"condition\": null"), "{baseline}");
    assert!(baseline.contains("baseline"), "{baseline}");
    assert!(baseline.contains("\"family\": \"llvm\""), "{baseline}");

    // With the capability evidence the conditional body is selected,
    // and the manifest names the capability and why.
    let mut capped = TargetProfile::native();
    capped.required.push("VecU32Target".to_string());
    let cap = manifest_of("pick-cap", src, &capped);
    assert!(cap.contains("Available VecU32Target"), "{cap}");
    assert!(cap.contains("condition holds in the target profile"), "{cap}");

    // The two artifacts' IR shows the different bodies, while a source
    // probe cannot observe which body was chosen: both programs compute
    // the same result.
    let ir0 = ir_of("pick-baseline", src);
    assert!(body_of(&ir0, "Main.pick.llvm").contains("add i32 %0, 1"), "{ir0}");
    let ir1_program =
        check_sources_profile(src, "pick-cap", &capped).expect("the capped artifact checks");
    let ir1 = cinch::codegen::emit(&ir1_program).expect("emit");
    assert!(body_of(&ir1, "Main.pick.llvm").contains("add i32 1, %0"), "{ir1}");
    assert_eq!(run_source(src).expect("baseline run"), 0);

    // Selection is representation: the contract projection carries no
    // family and no condition.
    let dir = write_project("pick-iface", src);
    driver_with_profile(Mode::Check, StageMode::Refresh, &dir.join("Main.cinch"), None, &capped)
        .expect("check run");
    let iface =
        std::fs::read_to_string(dir.join("build/Main.cinchif")).expect("interface");
    assert!(!iface.contains("Available"), "{iface}");
    assert!(!iface.contains("VecU32Target"), "{iface}");
}

// ------------------------------------------- the tier identity rules

#[test]
fn justified_prim_records_its_law_identity() {
    // A body justified by a law method: the contract is the signature
    // plus the law identity (section 5), recorded in the manifest's
    // declared assumptions.
    let src = r#"
module Main where
  prim twice :: U32 -> U32 where
    llvm {
      %s = add i32 %0, %0
      ret i32 %s
    } justify TwiceLaws.twiceLaw

  signature TwiceLaws where
    law twiceLaw :: (x :: U32) -> Eq (twice x) (wrapping-add-u32 x x)

  main :: Int
  main =
    if twice 21 == 42 then 0 else 1
"#;
    let code = run_source(src).expect("the justified prim compiles and runs");
    assert_eq!(code, 0);
    let m = manifest_of("tier-justified", src, &TargetProfile::native());
    assert!(m.contains("\"tier\": \"justified\""), "{m}");
    assert!(m.contains("Main.twiceLaw"), "{m}");
}

#[test]
fn assumed_prim_identity_is_the_body_hash() {
    // Without justification the body content is contract (section 5):
    // the assumed prim's identity is a body hash, recorded in the
    // assumption manifest and tagged with its selected family.
    let src = r#"
module Main where
  prim addB :: U32 -> U32 -> U32 where
    llvm {
      %s = add i32 %0, %1
      ret i32 %s
    }

  main :: Int
  main =
    if addB 40 2 == 42 then 0 else 1
"#;
    let m = manifest_of("tier-assumed", src, &TargetProfile::native());
    assert!(m.contains("\"tier\": \"assumed\""), "{m}");
    assert!(m.contains("prim:Main.addB#"), "{m}");
    assert!(m.contains("\"family\": \"llvm\""), "{m}");
}

/// The text of one unit's function attribute group (empty when the unit
/// carries no group).
fn attrs_of(ir: &str, unit: &str) -> String {
    let define = define_of(ir, unit);
    let Some((_, tail)) = define.split_once(" #") else {
        return String::new();
    };
    let n = tail
        .split_whitespace()
        .next()
        .expect("an attribute group number");
    let head = format!("attributes #{n} = {{ ");
    ir.lines()
        .find_map(|l| l.strip_prefix(&head).and_then(|r| r.strip_suffix(" }")))
        .unwrap_or_else(|| panic!("{unit}: attribute group #{n} missing in {ir}"))
        .to_string()
}

/// One checked source's manifest under an explicit profile.
fn manifest_profile(name: &str, src: &str, profile: &TargetProfile) -> String {
    manifest_of(name, src, profile)
}

// ---------------------------------- section 12: effect rows and unsafe

#[test]
fn pure_prim_calls_leave_evidence_intact_and_carry_memory_none() {
    // A prim with an empty effect row is proven pure by containment:
    // its calls leave mutable-location evidence intact, and a caller
    // through it keeps `memory(none)`.
    let src = r#"
module Main where
  prim bump :: U32 -> U32 where
    llvm {
      %s = add i32 %0, 1
      ret i32 %s
    }

  caller :: U32 -> U32
  caller x = bump x

  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = caller 7 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#;
    check_sources(src, "pure-prim-evidence").expect("a pure prim call leaves evidence intact");
    let ir = ir_of("pure-prim-evidence", src);
    assert!(attrs_of(&ir, "Main.caller").contains("memory(none)"), "{ir}");
}

#[test]
fn effectful_prim_calls_invalidate_per_row() {
    // A prim whose row carries `Io` is effectful per its row, exactly
    // like any function with that row: its call invalidates
    // mutable-location evidence.
    expect_err(
        "io-prim",
        r#"
module Main where
  prim churn :: I64 ->{IO} I64 where
    llvm {
      ret i64 %0
    }

  prog :: forall region. Mut region Nat ->{IO} Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = churn 7 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
"#,
        "stale evidence",
    );
}

#[test]
fn unsafe_pure_prim_records_its_assumption_and_stays_scan_gated() {
    // An `unsafe` body's declared row is trusted, not verified: the
    // trust is a visible assumption (the per-body identity), and the
    // scan-gated attributes stay gated (trust is recorded, the claim is
    // not free).
    let src = r#"
module Main where
  prim tick :: U32 -> U32 where
    unsafe asm {
      nop
    }

  main :: Int
  main = 0
"#;
    let m = manifest_of("unsafe-trust", src, &TargetProfile::native());
    assert!(m.contains("\"trust\": [") && m.contains("trusted not verified"), "{m}");
    assert!(m.contains("prim:Main.tick#"), "{m}");
    let i = m
        .find("\"declared\": [")
        .unwrap_or_else(|| panic!("declared assumptions: {m}"));
    let declared = &m[i..m[i..].find(']').map(|j| i + j).unwrap_or(m.len())];
    assert!(declared.contains("prim:Main.tick#"), "{declared}");
    let ir = ir_of("unsafe-trust", src);
    let attrs = attrs_of(&ir, "Main.tick.asm");
    assert!(!attrs.contains("willreturn") && !attrs.contains("nofree"), "{attrs}");
}

#[test]
fn containment_rejects_a_pure_row_with_stores_even_unsafe() {
    // Visible effects are rejected even under `unsafe`: trust covers
    // only what containment cannot see.
    expect_err(
        "unsafe-stores",
        r#"
module Main where
  prim writes :: U32 -> U32 where
    unsafe llvm {
      store i32 %0, ptr %1
      ret i32 %0
    }

  main :: Int
  main = 0
"#,
        "determinism of effects",
    );
}

#[test]
fn unverifiable_bodies_without_unsafe_are_rejected() {
    // Nothing is silently trusted: an `asm` template declares its row
    // under `unsafe` or it does not compile.
    expect_err(
        "asm-trust-required",
        r#"
module Main where
  prim silent :: U32 -> U32 where
    asm {
      nop
    }

  main :: Int
  main = 0
"#,
        "mark it `unsafe`",
    );
}

// --------------------------------------- section 13: platform gating

#[test]
fn platform_gated_asm_bodies_select_per_arch() {
    // The canonical shape: a portable `llvm` baseline beside per-arch
    // `asm` bodies. Selection is manifest-only with the platform in the
    // body row's reason; a source probe cannot observe the choice.
    let src = r#"
module Main where
  prim pop :: U32 -> U32 where
    unsafe asm {
      popcnt
    } when arch x86_64
    unsafe asm {
      cnt
    } when arch aarch64
    llvm {
      ret i32 %0
    }

  main :: Int
  main =
    if pop 3 == 0 then 0 else 0
"#;
    let mut x86 = TargetProfile::native();
    x86.triple = "x86_64-unknown-linux-gnu".to_string();
    let mut arm = TargetProfile::native();
    arm.triple = "aarch64-unknown-linux-gnu".to_string();
    let mut other = TargetProfile::native();
    other.triple = "riscv64-unknown-linux-gnu".to_string();

    // x86_64: the matching `asm` body wins, and the manifest names the
    // platform.
    let mx = manifest_profile("pop-x86", src, &x86);
    assert!(mx.contains("arch x86_64"), "{mx}");
    assert!(mx.contains("condition holds in the target profile"), "{mx}");
    let irx = {
        let p = check_sources_profile(src, "pop-x86", &x86).expect("x86 checks");
        cinch::codegen::emit(&p).expect("emit")
    };
    assert!(body_of(&irx, "Main.pop.asm").contains("popcnt"), "{irx}");
    assert!(!irx.contains("cnt\\0A") && !body_of(&irx, "Main.pop.asm").contains(" cnt"), "{irx}");

    // aarch64: the other conditional wins.
    let ma = manifest_profile("pop-arm", src, &arm);
    assert!(ma.contains("arch aarch64"), "{ma}");
    let ira = {
        let p = check_sources_profile(src, "pop-arm", &arm).expect("arm checks");
        cinch::codegen::emit(&p).expect("emit")
    };
    assert!(body_of(&ira, "Main.pop.asm").contains("cnt"), "{ira}");
    assert!(!body_of(&ira, "Main.pop.asm").contains("popcnt"), "{ira}");

    // A third platform: no condition matches, so the portable `llvm`
    // baseline is the fallback and no `asm` unit is emitted.
    let mo = manifest_profile("pop-other", src, &other);
    assert!(mo.contains("baseline"), "{mo}");
    assert!(!mo.contains("arch x86_64\\\""), "{mo}");
    let iro = {
        let p = check_sources_profile(src, "pop-other", &other).expect("other checks");
        cinch::codegen::emit(&p).expect("emit")
    };
    assert!(body_of(&iro, "Main.pop.llvm").contains("ret i32"), "{iro}");
    assert!(!iro.contains("Main.pop.asm"), "{iro}");

    // Selection is representation: the contract projection carries no
    // platform, and the program's behavior does not depend on the body
    // chosen.
    let dir = write_project("pop-iface", src);
    driver_with_profile(Mode::Check, StageMode::Refresh, &dir.join("Main.cinch"), None, &x86)
        .expect("check run");
    let iface = std::fs::read_to_string(dir.join("build/Main.cinchif")).expect("interface");
    assert!(!iface.contains("x86_64") && !iface.contains("arch "), "{iface}");
}

#[test]
fn conditional_only_prims_reject_naming_the_missing_platform() {
    // The reject-or-select rule (section 13): a conditional-only prim
    // whose conditions do not match rejects, naming the platform.
    expect_err(
        "pop-missing",
        r#"
module Main where
  prim pop :: U32 -> U32 where
    unsafe asm {
      popcnt
    } when arch x86_64

  main :: Int
  main = 0
"#,
        "arch x86_64",
    );
}
