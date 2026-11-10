//! `prim` declarations with inline target-IR bodies (primitive
//! operations and inline target IR specification sections 2-8): the
//! per-family bodies compile, containment validation names its
//! violations, the `clr`/`jvm` slots are reserved, and capability-scoped
//! body selection is recorded in the artifact manifest only (never in
//! the contract projection). Evidence is IR observation and manifest
//! observation (design/llvm-contract-attributes.md), never wiring
//! tests.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{
    check_sources, check_sources_profile, driver_with_profile, target_profile, DriverOptions, Mode,
};
use common::source_frontend::run_source;
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
    match common::source_frontend::check_sources(src, name) {
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
    assert!(baseline.contains("\"condition\":null"), "{baseline}");
    assert!(baseline.contains("baseline"), "{baseline}");
    assert!(baseline.contains("\"family\":\"llvm\""), "{baseline}");

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
    assert!(m.contains("\"tier\":\"justified\""), "{m}");
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
    assert!(m.contains("\"tier\":\"assumed\""), "{m}");
    assert!(m.contains("prim:Main.addB#"), "{m}");
    assert!(m.contains("\"family\":\"llvm\""), "{m}");

    let with_intrinsic = src.replace(
        "module Main where",
        "module Main where\n  foreign import ccall \"cinch.prim.eqI64\" equal :: I64 -> I64 -> Bool",
    );
    let interface = |source: &str| {
        let interfaces = common::source_frontend::check_source_interfaces(source, "assumed-prim.cinch")
            .expect("the assumed primitive interface checks");
        interfaces.into_iter().find(|iface| iface.module == "Main").unwrap()
    };
    let original = interface(&with_intrinsic);
    let updated = interface(&with_intrinsic.replace("add i32", "sub i32"));
    let assumed = |iface: &cinch::interface::Iface| {
        assert!(iface.assumptions.primitives.iter().any(|id| id == "cinch.prim.eqI64"));
        iface.assumptions.primitives.iter().find(|id| id.starts_with("prim:Main.addB#"))
            .expect("the compiled interface keeps the assumed primitive identity")
            .clone()
    };
    assert_ne!(assumed(&original), assumed(&updated));
    assert_ne!(original.iface_hash(), updated.iface_hash());
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
    common::source_frontend::check_sources(src, "pure-prim-evidence").expect("a pure prim call leaves evidence intact");
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
    assert!(m.contains("\"trust\":[") && m.contains("trusted not verified"), "{m}");
    assert!(m.contains("prim:Main.tick#"), "{m}");
    let i = m
        .find("\"declared\":[")
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

// ------------------------------ section 13: target-gated body selection

/// A target profile for one triple with its components set by hand. A
/// body's `when` atoms read these fields, so a test states exactly what
/// the compile target is instead of inheriting the host's os or arch
/// (which is the difference between reading a target and reading the
/// build machine).
fn profile_for(triple: &str, os: &str, arch: &str, libc: &str, word_size: u32) -> TargetProfile {
    let mut profile = TargetProfile::native();
    profile.name = triple.to_string();
    profile.triple = triple.to_string();
    profile.os = os.to_string();
    profile.arch = arch.to_string();
    profile.libc = libc.to_string();
    profile.word_size = word_size;
    profile
}

/// The emitted LLVM IR of one checked source under an explicit profile.
fn ir_profile(name: &str, src: &str, profile: &TargetProfile) -> String {
    let program = check_sources_profile(src, name, profile)
        .unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// Checking one source under an explicit profile fails with a message
/// containing `needle` (and every `also` needle).
fn expect_err_profile(
    name: &str,
    src: &str,
    profile: &TargetProfile,
    needle: &str,
    also: &[&str],
) -> String {
    let diagnostic = check_sources_profile(src, name, profile).err()
        .unwrap_or_else(|| {
            panic!("{name}: expected an error containing {needle:?}, but it checked")
        });
    let msg = diagnostic.to_string();
    assert!(msg.contains(needle), "{name}: {msg:?} does not contain {needle:?}");
    for extra in also {
        assert!(msg.contains(extra), "{name}: {msg:?} does not contain {extra:?}");
    }
    msg
}

/// The `x86_64-unknown-linux-gnu` profile: a GNU/Linux target whose every
/// component is stated, built by hand rather than taken from the host.
fn linux_x86_64() -> TargetProfile {
    profile_for("x86_64-unknown-linux-gnu", "linux", "x86_64", "glibc", 64)
}

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
    } when arch x86_64 immarg 0
    unsafe asm {
      cnt
    } immarg 0 when arch aarch64
    llvm {
      ret i32 %0
    }

  main :: Int
  main =
    if pop 3 == 0 then 0 else 0
"#;
    let x86 = profile_for("x86_64-unknown-linux-gnu", "linux", "x86_64", "glibc", 64);
    let arm = profile_for("aarch64-unknown-linux-gnu", "linux", "aarch64", "glibc", 64);
    let other = profile_for("riscv64-unknown-linux-gnu", "linux", "riscv64", "glibc", 64);

    // x86_64: the matching `asm` body wins, and the manifest names the
    // platform.
    let mx = manifest_profile("pop-x86", src, &x86);
    assert!(mx.contains("arch x86_64"), "{mx}");
    assert!(mx.contains("condition holds in the target profile"), "{mx}");
    let irx = ir_profile("pop-x86", src, &x86);
    assert!(body_of(&irx, "Main.pop.asm").contains("popcnt"), "{irx}");
    assert!(define_of(&irx, "Main.pop.asm").contains("!immarg"));
    assert!(!body_of(&irx, "Main.pop.asm").contains(" cnt"), "{irx}");

    // aarch64: the other conditional wins.
    let ma = manifest_profile("pop-arm", src, &arm);
    assert!(ma.contains("arch aarch64"), "{ma}");
    let ira = ir_profile("pop-arm", src, &arm);
    assert!(body_of(&ira, "Main.pop.asm").contains("cnt"), "{ira}");
    assert!(define_of(&ira, "Main.pop.asm").contains("!immarg"));
    assert!(!body_of(&ira, "Main.pop.asm").contains("popcnt"), "{ira}");

    // A third architecture: no condition matches, so the portable `llvm`
    // baseline is the fallback and no `asm` unit is emitted.
    let mo = manifest_profile("pop-other", src, &other);
    assert!(mo.contains("baseline"), "{mo}");
    assert!(!mo.contains("arch x86_64\\\""), "{mo}");
    let iro = ir_profile("pop-other", src, &other);
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
fn os_gated_bodies_select_the_target_os() {
    // Two bodies gated on the operating system, both `asm`. The profiles
    // are stated by hand, so this fails if a condition reads the build
    // host (this host is darwin) instead of the compile target.
    let src = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      linuxmarker
    } when os linux
    unsafe asm {
      darwinmarker
    } when os darwin

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let linux = linux_x86_64();
    let darwin = profile_for("aarch64-apple-darwin", "darwin", "aarch64", "libSystem", 64);

    let mlin = manifest_profile("os-linux", src, &linux);
    assert!(mlin.contains("os linux"), "{mlin}");
    assert!(mlin.contains("condition holds in the target profile"), "{mlin}");
    let irlin = ir_profile("os-linux", src, &linux);
    assert!(body_of(&irlin, "Main.probe.asm").contains("linuxmarker"), "{irlin}");
    assert!(!body_of(&irlin, "Main.probe.asm").contains("darwinmarker"), "{irlin}");

    let mdar = manifest_profile("os-darwin", src, &darwin);
    assert!(mdar.contains("os darwin"), "{mdar}");
    let irdar = ir_profile("os-darwin", src, &darwin);
    assert!(body_of(&irdar, "Main.probe.asm").contains("darwinmarker"), "{irdar}");
    assert!(!body_of(&irdar, "Main.probe.asm").contains("linuxmarker"), "{irdar}");
}

#[test]
fn the_driver_target_names_the_body_that_compiles() {
    // The same selection through the driver's own target option: naming
    // `x86_64-unknown-linux-gnu` on a Darwin machine compiles the Linux
    // body and not the Darwin one, which is the cross-compilation shape
    // the syscall layer needs and the whole point of reading a target
    // profile rather than the host.
    let src = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      linuxmarker
    } when os linux
    unsafe asm {
      darwinmarker
    } when os darwin

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let options = |target: &str| DriverOptions {
        target: Some(target.to_string()),
        ..DriverOptions::default()
    };
    let linux = target_profile(&options("x86_64-unknown-linux-gnu"))
        .expect("the cross target's profile");
    assert_eq!(linux.os, "linux");
    assert_eq!(linux.arch, "x86_64");
    let irlin = ir_profile("driver-linux", src, &linux);
    assert!(body_of(&irlin, "Main.probe.asm").contains("linuxmarker"), "{irlin}");
    assert!(!irlin.contains("darwinmarker"), "{irlin}");

    let darwin = target_profile(&options("aarch64-apple-darwin"))
        .expect("the host target's profile");
    assert_eq!(darwin.os, "darwin");
    let irdar = ir_profile("driver-darwin", src, &darwin);
    assert!(body_of(&irdar, "Main.probe.asm").contains("darwinmarker"), "{irdar}");
    assert!(!irdar.contains("linuxmarker"), "{irdar}");
}

#[test]
fn word_size_gated_bodies_select_the_target_width() {
    // `word-size` reads the profile's `Usize`/`Isize` width, not the
    // host's: a 32-bit target takes the narrow body on this 64-bit
    // machine.
    let src = r#"
module Main where
  prim word :: U32 -> U32 where
    unsafe asm {
      narrowmarker
    } when word-size 32
    unsafe asm {
      widemarker
    } when word-size 64

  main :: Int
  main =
    if word 3 == 0 then 0 else 0
"#;
    let narrow = profile_for("i686-unknown-linux-gnu", "linux", "x86", "glibc", 32);
    let wide = linux_x86_64();

    let mn = manifest_profile("word-32", src, &narrow);
    assert!(mn.contains("word-size 32"), "{mn}");
    let irn = ir_profile("word-32", src, &narrow);
    assert!(body_of(&irn, "Main.word.asm").contains("narrowmarker"), "{irn}");
    assert!(!body_of(&irn, "Main.word.asm").contains("widemarker"), "{irn}");

    let mw = manifest_profile("word-64", src, &wide);
    assert!(mw.contains("word-size 64"), "{mw}");
    let irw = ir_profile("word-64", src, &wide);
    assert!(body_of(&irw, "Main.word.asm").contains("widemarker"), "{irw}");
    assert!(!body_of(&irw, "Main.word.asm").contains("narrowmarker"), "{irw}");
}

#[test]
fn a_negated_atom_holds_where_the_target_is_not_that_os() {
    // `not (os windows)` is the negation of the atom, so the two bodies
    // partition the targets: Windows takes the first, everything else
    // the second.
    let src = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      windowsmarker
    } when os windows
    unsafe asm {
      portablemarker
    } when not (os windows)

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let linux = linux_x86_64();
    let windows = profile_for("x86_64-pc-windows-msvc", "windows", "x86_64", "msvcrt", 64);

    let mlin = manifest_profile("not-windows-linux", src, &linux);
    assert!(mlin.contains("not (os windows)"), "{mlin}");
    let irlin = ir_profile("not-windows-linux", src, &linux);
    assert!(body_of(&irlin, "Main.probe.asm").contains("portablemarker"), "{irlin}");
    assert!(!body_of(&irlin, "Main.probe.asm").contains("windowsmarker"), "{irlin}");

    let mwin = manifest_profile("not-windows-win", src, &windows);
    assert!(mwin.contains("os windows"), "{mwin}");
    let irwin = ir_profile("not-windows-win", src, &windows);
    assert!(body_of(&irwin, "Main.probe.asm").contains("windowsmarker"), "{irwin}");
    assert!(!body_of(&irwin, "Main.probe.asm").contains("portablemarker"), "{irwin}");

    // The negated atom alone, with no other body, is unsatisfiable on the
    // target it negates: the refusal names the negated clause.
    let only_negated = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      portablemarker
    } when not (os windows)

  main :: Int
  main = 0
"#;
    expect_err_profile(
        "not-windows-alone",
        only_negated,
        &windows,
        "not (os windows)",
        &["the target's os is `windows`"],
    );
}

#[test]
fn clauses_on_one_body_are_a_conjunction() {
    // Two clauses on one body, both required. Reading either clause alone
    // (or reading them as a disjunction) would select the `asm` body on
    // one of the two profiles that must fall back to the baseline.
    let src = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      linuxarmmarker
    } when os linux when arch aarch64
    llvm {
      ret i32 %0
    }

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let linux_arm = profile_for("aarch64-unknown-linux-gnu", "linux", "aarch64", "glibc", 64);
    let linux_x86 = linux_x86_64();
    let darwin_arm = profile_for("aarch64-apple-darwin", "darwin", "aarch64", "libSystem", 64);

    // Both atoms hold: the conditional body wins and the manifest renders
    // the conjunction.
    let mb = manifest_profile("conj-arm-linux", src, &linux_arm);
    assert!(mb.contains("os linux and arch aarch64"), "{mb}");
    let irb = ir_profile("conj-arm-linux", src, &linux_arm);
    assert!(body_of(&irb, "Main.probe.asm").contains("linuxarmmarker"), "{irb}");

    // The os alone: the arch clause does not hold, so the baseline is the
    // fallback.
    let mx = manifest_profile("conj-x86-linux", src, &linux_x86);
    assert!(mx.contains("baseline"), "{mx}");
    let irx = ir_profile("conj-x86-linux", src, &linux_x86);
    assert!(!irx.contains("Main.probe.asm"), "{irx}");

    // The arch alone: the os clause does not hold either.
    let md = manifest_profile("conj-arm-darwin", src, &darwin_arm);
    assert!(md.contains("baseline"), "{md}");
    let ird = ir_profile("conj-arm-darwin", src, &darwin_arm);
    assert!(!ird.contains("Main.probe.asm"), "{ird}");
}

#[test]
fn a_condition_no_body_satisfies_names_the_atom() {
    // The refusal is honest: it names the atom that failed, what the
    // target is instead, and the whole vocabulary, so a mistyped or
    // unsatisfiable condition is not confused with a missing body.
    let source = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      windowsmarker
    } when os windows

  main :: Int
  main = 0
"#;
    let msg = expect_err_profile(
        "no-body-platform",
        source,
        &linux_x86_64(),
        "no body applies",
        &[
            "os windows",
            "the target's os is `linux`",
            "x86_64-unknown-linux-gnu",
            "Available <capability>",
            "arch <name>",
            "os <name>",
            "libc <name>",
            "endian <little|big>",
            "word-size <bits>",
            "target <triple>",
            "not (<atom>)",
        ],
    );
    assert!(msg.contains("the `asm` body's `when os windows`"), "{msg}");

    // A capability atom fails differently: it names the evidence the
    // profile does carry, so a missing capability reads as a capability
    // answer rather than a platform mismatch.
    let capability_source = r#"
module Main where
  prim probe :: U32 -> U32 where
    llvm {
      ret i32 %0
    } when Available VecU32Target

  main :: Int
  main = 0
"#;
    let mut profile = linux_x86_64();
    profile.required.clear();
    profile.features.clear();
    expect_err_profile(
        "no-body-capability",
        capability_source,
        &profile,
        "Available VecU32Target",
        &["the target profile carries no capability evidence"],
    );
    profile.required.push("SomeOtherCapability".to_string());
    expect_err_profile(
        "no-body-capability-listed",
        capability_source,
        &profile,
        "the target profile carries SomeOtherCapability",
        &[],
    );
}

#[test]
fn conditional_only_prims_reject_naming_the_missing_platform() {
    // The reject-or-select rule (section 13): a conditional-only prim
    // whose conditions do not match rejects, naming the platform.
    let source = r#"
module Main where
  prim pop :: U32 -> U32 where
    unsafe asm {
      popcnt
    } when arch x86_64

  main :: Int
  main = 0
"#;
    let arm = profile_for("aarch64-unknown-linux-gnu", "linux", "aarch64", "glibc", 64);
    expect_err_profile("pop-missing", source, &arm, "arch x86_64", &["the target's arch is `aarch64`"]);

    let x86 = linux_x86_64();
    check_sources_profile(source, "pop-present", &x86)
        .expect("the same primitive has a body on x86_64");
}

#[test]
fn every_atom_and_negation_read_the_target_profile() {
    // The atoms that no other case pins: `libc`, `endian`, `target`, and
    // the negated forms of the remaining atoms. Each body is the only one
    // in its family, so a wrong evaluation rejects instead of picking a
    // baseline.
    let src = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      gnu
    } when libc glibc when endian little when target x86_64-unknown-linux-gnu
    unsafe asm {
      musl
    } when libc musl

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let linux = linux_x86_64();
    let irlin = ir_profile("atoms-linux", src, &linux);
    assert!(body_of(&irlin, "Main.probe.asm").contains("gnu"), "{irlin}");
    assert!(!body_of(&irlin, "Main.probe.asm").contains("musl"), "{irlin}");

    let musl = profile_for("x86_64-unknown-linux-musl", "linux", "x86_64", "musl", 64);
    let irmusl = ir_profile("atoms-musl", src, &musl);
    assert!(body_of(&irmusl, "Main.probe.asm").contains("musl"), "{irmusl}");
    assert!(!body_of(&irmusl, "Main.probe.asm").contains("gnu"), "{irmusl}");

    // A big-endian target takes the negated endian atom, and a mismatched
    // triple is refused by name.
    let negated = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      littleendian
    } when not (endian big)
    unsafe asm {
      bigendian
    } when endian big

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let irle = ir_profile("endian-little", negated, &linux);
    assert!(body_of(&irle, "Main.probe.asm").contains("littleendian"), "{irle}");
    let mut big = linux_x86_64();
    big.endian = "big".to_string();
    let irbe = ir_profile("endian-big", negated, &big);
    assert!(body_of(&irbe, "Main.probe.asm").contains("bigendian"), "{irbe}");

    let wrong_triple = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      exact
    } when target aarch64-apple-darwin

  main :: Int
  main = 0
"#;
    expect_err_profile(
        "atoms-wrong-triple",
        wrong_triple,
        &linux,
        "target aarch64-apple-darwin",
        &["the target's triple is `x86_64-unknown-linux-gnu`"],
    );

    // The `target` atom is a declaration keyword in the rest of the
    // grammar, so its clause spelling is pinned here, negated and as a
    // body's last clause.
    let negated_triple = r#"
module Main where
  prim probe :: U32 -> U32 where
    unsafe asm {
      nottriplemarker
    } when not (target aarch64-apple-darwin)
    unsafe asm {
      exacttriplemarker
    } when target aarch64-apple-darwin

  main :: Int
  main =
    if probe 3 == 0 then 0 else 0
"#;
    let irnot = ir_profile("triple-not-darwin", negated_triple, &linux);
    assert!(body_of(&irnot, "Main.probe.asm").contains("nottriplemarker"), "{irnot}");
    assert!(!body_of(&irnot, "Main.probe.asm").contains("exacttriplemarker"), "{irnot}");
    let darwin = profile_for("aarch64-apple-darwin", "darwin", "aarch64", "libSystem", 64);
    let irtriple = ir_profile("triple-darwin", negated_triple, &darwin);
    assert!(body_of(&irtriple, "Main.probe.asm").contains("exacttriplemarker"), "{irtriple}");
    assert!(!body_of(&irtriple, "Main.probe.asm").contains("nottriplemarker"), "{irtriple}");
}
