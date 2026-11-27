//! Tests for zero-initialized POD `#[static_mut]` and `#[thread_local]`
//! `static` declarations with optional `align(N)`, effect tracking
//! (`Global` / `ThreadAffine`), safety attributes (`#[no-alloc]` /
//! `#[no-raw-ptr]`), wasm32 TLS lowering, and cross-module export/import.

mod common;

use cinch::core::TargetProfile;
use common::source_frontend::{
    check_sources, emit_ir, emit_ir_profile, run_source,
};

fn matches_unit_define(line: &str, unit: &str) -> bool {
    if !line.starts_with("define ") {
        return false;
    }
    let candidates = [
        format!("@\"{unit}\"("),
        format!("@\"cinch.typed.m.{unit}\"("),
        format!("@\"cinch.typed.m.{unit}#b\"("),
    ];
    candidates.iter().any(|c| line.contains(c))
}

fn body_of(ir: &str, unit: &str) -> String {
    let mut lines = ir.lines();
    while let Some(l) = lines.next() {
        if matches_unit_define(l, unit) {
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
    panic!("{unit}: no define found in IR");
}

fn define_of(ir: &str, unit: &str) -> String {
    ir.lines()
        .find(|l| matches_unit_define(l, unit))
        .unwrap_or_else(|| panic!("{unit}: no define found in IR"))
        .to_string()
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(()) => panic!("{name}: expected compile error matching one of {needles:?}, but checked"),
        Err(e) => {
            assert!(
                needles.iter().any(|n| e.contains(n)),
                "{name}: error {e:?} did not contain any of {needles:?}"
            );
        }
    }
}

#[test]
fn static_mut_scalar_struct_and_fixed_array_with_align() {
    let src = r#"
module Main where
  #[repr(value)]
  record Slab where
    head :: U64
    count :: U64

  #[static_mut]
  static counter :: U64

  #[static_mut]
  #[align(64)]
  static slab :: Slab

  #[static_mut(align(32))]
  static ring :: FixedArray 4 U64

  prim loadU64 :: forall a. RawPtr a -> Isize ->{Global} U64 where
    unsafe llvm {
      %p = getelementptr i8, ptr %0, i64 %1
      %v = load i64, ptr %p, align 8
      ret i64 %v
    }

  prim storeU64 :: forall a. RawPtr a -> Isize -> U64 ->{Global} Unit where
    unsafe llvm {
      %p = getelementptr i8, ptr %0, i64 %1
      store i64 %2, ptr %p, align 8
      ret void
    }

  prim atomicAddU64 :: RawPtr U64 -> U64 ->{Global} U64 where
    unsafe llvm {
      %old = atomicrmw add ptr %0, i64 %1 seq_cst
      ret i64 %old
    }

  #[no-alloc]
  bumpAll :: Unit ->{Global} U64
  bumpAll _ =
    let c0 = atomicAddU64 (counter ()) 10 in
    let c1 = atomicAddU64 (counter ()) 5 in
    let s0 = loadU64 (slab ()) 0 in
    let _ = storeU64 (slab ()) 0 (wrapping-add-u64 (wrapping-add-u64 c0 c1) (wrapping-add-u64 s0 7)) in
    let _ = storeU64 (slab ()) 8 20 in
    let _ = storeU64 (ring ()) 24 3 in
    wrapping-add-u64
      (wrapping-add-u64 (loadU64 (slab ()) 0) (loadU64 (slab ()) 8))
      (wrapping-add-u64 (loadU64 (ring ()) 24) (loadU64 (counter ()) 0))

  main :: Unit ->{Global} Int
  main _ = if bumpAll () == 55 then 55 else 1
"#;

    let ir = emit_ir(src).expect("emit native IR");
    assert!(
        ir.contains("= internal global [8 x i8] zeroinitializer, align 8"),
        "expected 8-byte aligned internal global for counter; IR:\n{ir}"
    );
    assert!(
        ir.contains("= internal global [16 x i8] zeroinitializer, align 64"),
        "expected 16-byte 64-aligned internal global for slab; IR:\n{ir}"
    );
    assert!(
        ir.contains("= internal global [32 x i8] zeroinitializer, align 32"),
        "expected 32-byte 32-aligned internal global for ring; IR:\n{ir}"
    );
    let counter_def = define_of(&ir, "Main.counter");
    let has_alwaysinline = counter_def.contains("alwaysinline")
        || counter_def
            .split_whitespace()
            .find(|w| w.starts_with('#'))
            .is_some_and(|grp| {
                ir.lines()
                    .any(|l| l.starts_with(&format!("attributes {grp} =")) && l.contains("alwaysinline"))
            });
    assert!(
        has_alwaysinline,
        "expected alwaysinline on Main.counter, got: {counter_def}"
    );
    assert!(
        counter_def.contains("ptr @"),
        "expected native ptr return on Main.counter, got: {counter_def}"
    );
    let bump_body = body_of(&ir, "Main.bumpAll");
    assert!(
        !bump_body.contains("cinch_alloc"),
        "expected zero heap allocation in #[no-alloc] bumpAll; body:\n{bump_body}"
    );
    assert!(
        !bump_body.contains("inttoptr") && !bump_body.contains("ptrtoint"),
        "expected native ptr without inttoptr/ptrtoint in bumpAll; body:\n{bump_body}"
    );

    // c0 = 0, c1 = 10, s0 = 0 -> slab.head = 17, slab.count = 20, ring[3] = 3, counter = 15 -> sum = 55
    let out = run_source(src).expect("run static_mut program");
    assert_eq!(out, 55, "unexpected runtime result from static_mut");
}

#[test]
fn thread_local_native_ir_wasm32_ir_and_runtime() {
    let src = r#"
module Main where
  #[repr(value)]
  record TlsState where
    cursor :: U64
    limit :: U64

  #[thread_local]
  static tlsCounter :: U64

  #[thread_local(align(64))]
  static tlsState :: TlsState

  prim tlsLoadU64 :: forall a. RawPtr a -> Isize ->{ThreadAffine} U64 where
    unsafe llvm {
      %p = getelementptr i8, ptr %0, i64 %1
      %v = load i64, ptr %p, align 8
      ret i64 %v
    }

  prim tlsStoreU64 :: forall a. RawPtr a -> Isize -> U64 ->{ThreadAffine} Unit where
    unsafe llvm {
      %p = getelementptr i8, ptr %0, i64 %1
      store i64 %2, ptr %p, align 8
      ret void
    }

  #[no-alloc]
  stepTls :: Unit ->{ThreadAffine} U64
  stepTls _ =
    let a = tlsLoadU64 (tlsCounter ()) 0 in
    let _ = tlsStoreU64 (tlsCounter ()) 0 (wrapping-add-u64 a 11) in
    let _ = tlsStoreU64 (tlsState ()) 8 31 in
    wrapping-add-u64 (tlsLoadU64 (tlsCounter ()) 0) (tlsLoadU64 (tlsState ()) 8)

  main :: Unit ->{ThreadAffine} Int
  main _ = if stepTls () == 42 then 42 else 1
"#;

    let native_ir = emit_ir(src).expect("emit native TLS IR");
    assert!(
        native_ir.contains("= internal thread_local global [8 x i8] zeroinitializer, align 8"),
        "expected native thread_local global for tlsCounter; IR:\n{native_ir}"
    );
    assert!(
        native_ir.contains("= internal thread_local global [16 x i8] zeroinitializer, align 64"),
        "expected native 64-aligned thread_local global for tlsState; IR:\n{native_ir}"
    );

    let wasm_profile = TargetProfile::wasm32(cinch::core::ArtifactProfile::CoreWasm);
    let wasm_ir = emit_ir_profile(src, &wasm_profile).expect("emit wasm32 TLS IR");
    assert!(
        !wasm_ir.contains("thread_local"),
        "wasm32 single-threaded target must lower #[thread_local] without LLVM thread_local; IR:\n{wasm_ir}"
    );
    assert!(
        wasm_ir.contains("= internal global [8 x i8] zeroinitializer, align 8"),
        "expected plain internal global on wasm32; IR:\n{wasm_ir}"
    );
    assert!(
        wasm_ir.contains("= internal global [16 x i8] zeroinitializer, align 64"),
        "expected plain 64-aligned internal global on wasm32; IR:\n{wasm_ir}"
    );

    let out = run_source(src).expect("run thread_local program");
    assert_eq!(out, 42, "unexpected runtime result from thread_local");
}

#[test]
fn static_effect_and_no_raw_ptr_enforcement() {
    // 1. Pure function calling #[static_mut] must fail (missing Global).
    expect_err(
        "pure_calls_static_mut",
        r#"
module Main where
  #[static_mut]
  static g :: U64

  bad :: Unit -> RawPtr U64
  bad () = g ()
"#,
        &["Global", "effect"],
    );

    // 2. Pure function calling #[thread_local] must fail (missing ThreadAffine).
    expect_err(
        "pure_calls_thread_local",
        r#"
module Main where
  #[thread_local]
  static t :: U64

  bad :: Unit -> RawPtr U64
  bad () = t ()
"#,
        &["ThreadAffine", "effect"],
    );

    // 3. parBoth callback calling #[thread_local] must fail (ThreadAffine forbidden in parallel work).
    expect_err(
        "par_both_forbids_thread_local",
        r#"
module Main where
  #[thread_local]
  static t :: U64

  bad :: Unit ->{ThreadAffine} Int
  bad () =
    let pair = parBoth (\() -> let _ = t () in 1) (\() -> 2) in
    case pair of (a, b) -> a + b
"#,
        &["ThreadAffine", "parallel", "effect"],
    );

    // 4. #[no-raw-ptr] calling #[static_mut] must fail.
    expect_err(
        "no_raw_ptr_rejects_static_mut",
        r#"
module Main where
  #[static_mut]
  static g :: U64

  #[no-raw-ptr]
  bad :: Unit ->{Global} Int
  bad () =
    let _ = g () in
    0
"#,
        &["no-raw-ptr", "RawPtr", "pointer"],
    );

    // 5. #[no-raw-ptr] calling #[thread_local] must fail.
    expect_err(
        "no_raw_ptr_rejects_thread_local",
        r#"
module Main where
  #[thread_local]
  static t :: U64

  #[no-raw-ptr]
  bad :: Unit ->{ThreadAffine} Int
  bad () =
    let _ = t () in
    0
"#,
        &["no-raw-ptr", "RawPtr", "pointer"],
    );
}

#[test]
fn static_validation_rejects_non_pod_and_bad_attributes() {
    // Non-POD types: Int, String, List U64, Box U64.
    for (label, ty) in [
        ("int", "Int"),
        ("string", "String"),
        ("list", "List U64"),
        ("box", "Box U64"),
    ] {
        let src = format!(
            "module Main where\n  #[static_mut]\n  static bad :: {ty}\n  main :: Int\n  main = 0\n"
        );
        expect_err(label, &src, &["POD", "non-refcounted"]);
    }

    // Missing storage-class attribute.
    expect_err(
        "bare_static_missing_attr",
        "module Main where\n  static x :: U64\n  main :: Int\n  main = 0\n",
        &["static_mut", "thread_local"],
    );

    // Conflicting #[static_mut] and #[thread_local].
    expect_err(
        "conflicting_static_attrs",
        "module Main where\n  #[static_mut]\n  #[thread_local]\n  static x :: U64\n  main :: Int\n  main = 0\n",
        &["conflict", "static_mut", "thread_local"],
    );

    // Non-power-of-two alignment: 3 and 0.
    expect_err(
        "align_non_pow2",
        "module Main where\n  #[static_mut]\n  #[align(3)]\n  static x :: U64\n  main :: Int\n  main = 0\n",
        &["power of two", "alignment"],
    );
    expect_err(
        "align_zero",
        "module Main where\n  #[thread_local(align(0))]\n  static x :: U64\n  main :: Int\n  main = 0\n",
        &["power of two", "alignment"],
    );

    // Misplaced #[static_mut] on a function.
    expect_err(
        "misplaced_static_mut",
        "module Main where\n  #[static_mut]\n  f :: Int\n  f = 1\n",
        &["static", "static_mut"],
    );
}

#[test]
fn static_cross_module_export_and_import() {
    let rt_src = r#"
module Rt where
  export shared_slot, tls_slot

  #[static_mut]
  #[align(64)]
  static shared_slot :: U64

  #[thread_local]
  static tls_slot :: U64
"#;
    let main_src = r#"
module Main where
  import Rt (shared_slot, tls_slot)

  prim loadG :: RawPtr U64 ->{Global} U64 where
    unsafe llvm {
      %v = load i64, ptr %0, align 8
      ret i64 %v
    }

  prim storeG :: RawPtr U64 -> U64 ->{Global} Unit where
    unsafe llvm {
      store i64 %1, ptr %0, align 8
      ret void
    }

  prim loadT :: RawPtr U64 ->{ThreadAffine} U64 where
    unsafe llvm {
      %v = load i64, ptr %0, align 8
      ret i64 %v
    }

  prim storeT :: RawPtr U64 -> U64 ->{ThreadAffine} Unit where
    unsafe llvm {
      store i64 %1, ptr %0, align 8
      ret void
    }

  main :: Unit ->{Global, ThreadAffine} Int
  main _ =
    let _ = storeG (shared_slot ()) 19 in
    let _ = storeT (tls_slot ()) 23 in
    if wrapping-add-u64 (loadG (shared_slot ())) (loadT (tls_slot ())) == 42 then 42 else 1
"#;

    let dir = std::env::temp_dir().join(format!("cinch-static-cross-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    std::fs::write(dir.join("Rt.cinch"), rt_src).expect("write Rt.cinch");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main.cinch");
    let exe = dir.join("main_bin");
    common::drive_with_profile(
        cinch::driver::Mode::Build,
        cinch::check::StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &TargetProfile::native(),
    )
    .expect("build cross-module static program");
    let status = std::process::Command::new(&exe).status().expect("run main_bin");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(status.code(), Some(42), "cross-module static_mut + thread_local sum mismatch");
}
