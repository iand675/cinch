//! Differential testing of `llvm-sem@21.1` against real `/usr/bin/clang`,
//! Translation Validation (`Core -> Core` simplify & `Core -> LLVM`),
//! Verified `rewrite ... justify verify`, and Memory/Atomic Boundary Contracts.

mod common;

use cinch::driver::check_sources;
use cinch::prim_verify::llvm_ir::lower_llvm_fragment;
use cinch::prim_verify::mem_contract::{audit_file_contracts, audit_source_contracts};
use cinch::prim_verify::sem::{eval_node, ConcreteVal, SemParam, SemTy};
use cinch::prim_verify::translate_val::{
    build_core_transform_obligation, build_core_vs_llvm_obligation,
    build_core_vs_llvm_obligation_with_fns, collect_program_prim_bodies, validate_fn_rc_safety,
    validate_llvm_memory_equiv, validate_module_rc_safety, validate_program_codegen,
};
use cinch::prim_verify::{Policy, TrustReceipt, TrustTier, VerificationStatus, VerifierEngine};
use std::path::PathBuf;
use std::process::Command;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn pick(&mut self, n: usize) -> usize {
        (self.next() as usize) % n
    }
}

#[test]
fn differential_llvm_sem_matches_clang_across_150_randomized_fragments() {
    if Command::new("clang").arg("--version").output().is_err() {
        eprintln!("skipping differential clang test: clang not found");
        return;
    }

    let mut rng = Rng(0x5eed_cafe_1234_5678);
    let mut llvm_ir = String::from(
        "declare i32 @llvm.bswap.i32(i32)\n\
         declare i32 @llvm.bitreverse.i32(i32)\n\
         declare i32 @llvm.ctpop.i32(i32)\n\
         declare i32 @llvm.ctlz.i32(i32, i1)\n\
         declare i32 @llvm.cttz.i32(i32, i1)\n\
         declare i32 @llvm.fshl.i32(i32, i32, i32)\n\
         declare i32 @llvm.fshr.i32(i32, i32, i32)\n\
         declare i32 @llvm.uadd.sat.i32(i32, i32)\n\
         declare i32 @llvm.usub.sat.i32(i32, i32)\n\
         declare i32 @llvm.sadd.sat.i32(i32, i32)\n\
         declare i32 @llvm.ssub.sat.i32(i32, i32)\n\
         declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32)\n\
         declare <4 x i32> @llvm.smin.v4i32(<4 x i32>, <4 x i32>)\n\n",
    );

    let mut fragments: Vec<(String, String, Vec<SemParam>, SemTy)> = Vec::new();

    // 1. 120 randomized straight-line + intrinsic + freeze fragments on i32 (2 params)
    let binops = ["add", "sub", "mul", "and", "or", "xor"];
    let cmp_preds = ["eq", "ne", "ult", "ule", "ugt", "uge", "slt", "sle", "sgt", "sge"];
    for idx in 0..120 {
        let mut frag = String::new();
        let mut regs = vec!["%0".to_string(), "%1".to_string()];
        let steps = 3 + rng.pick(4);
        for s in 0..steps {
            let rname = format!("%t{s}");
            let a = regs[rng.pick(regs.len())].clone();
            let b = regs[rng.pick(regs.len())].clone();
            match rng.pick(10) {
                0..=4 => {
                    let op = binops[rng.pick(binops.len())];
                    frag.push_str(&format!("  {rname} = {op} i32 {a}, {b}\n"));
                }
                5 => {
                    // Masked shift (guaranteed non-poison)
                    let m = format!("%m{s}");
                    let sop = ["shl", "lshr", "ashr"][rng.pick(3)];
                    frag.push_str(&format!("  {m} = and i32 {b}, 31\n"));
                    frag.push_str(&format!("  {rname} = {sop} i32 {a}, {m}\n"));
                }
                6 => {
                    let pred = cmp_preds[rng.pick(cmp_preds.len())];
                    let c = format!("%c{s}");
                    frag.push_str(&format!("  {c} = icmp {pred} i32 {a}, {b}\n"));
                    frag.push_str(&format!("  {rname} = select i1 {c}, i32 {a}, i32 {b}\n"));
                }
                7 => {
                    let intr = match rng.pick(7) {
                        0 => format!("call i32 @llvm.bswap.i32(i32 {a})"),
                        1 => format!("call i32 @llvm.bitreverse.i32(i32 {a})"),
                        2 => format!("call i32 @llvm.ctpop.i32(i32 {a})"),
                        3 => format!("call i32 @llvm.ctlz.i32(i32 {a}, i1 false)"),
                        4 => format!("call i32 @llvm.cttz.i32(i32 {a}, i1 false)"),
                        5 => format!("call i32 @llvm.fshl.i32(i32 {a}, i32 {b}, i32 {b})"),
                        _ => format!("call i32 @llvm.fshr.i32(i32 {a}, i32 {b}, i32 {b})"),
                    };
                    frag.push_str(&format!("  {rname} = {intr}\n"));
                }
                8 => {
                    let intr = match rng.pick(4) {
                        0 => format!("call i32 @llvm.uadd.sat.i32(i32 {a}, i32 {b})"),
                        1 => format!("call i32 @llvm.usub.sat.i32(i32 {a}, i32 {b})"),
                        2 => format!("call i32 @llvm.sadd.sat.i32(i32 {a}, i32 {b})"),
                        _ => format!("call i32 @llvm.ssub.sat.i32(i32 {a}, i32 {b})"),
                    };
                    frag.push_str(&format!("  {rname} = {intr}\n"));
                }
                _ => {
                    // Trunc + sext/zext round-trip + with.overflow + freeze
                    let ov = format!("%ov{s}");
                    let flag = format!("%fl{s}");
                    let val = format!("%vl{s}");
                    frag.push_str(&format!("  {ov} = call {{ i32, i1 }} @llvm.sadd.with.overflow.i32(i32 {a}, i32 {b})\n"));
                    frag.push_str(&format!("  {val} = extractvalue {{ i32, i1 }} {ov}, 0\n"));
                    frag.push_str(&format!("  {flag} = extractvalue {{ i32, i1 }} {ov}, 1\n"));
                    frag.push_str(&format!("  {rname} = select i1 {flag}, i32 {a}, i32 {val}\n"));
                }
            }
            regs.push(rname);
        }
        let last = regs.last().unwrap().clone();
        frag.push_str(&format!("  ret i32 {last}\n"));
        let fname = format!("diff_fn_{idx}");
        llvm_ir.push_str(&format!("define i32 @{fname}(i32 %0, i32 %1) {{\nentry:\n{frag}}}\n\n"));
        fragments.push((
            fname,
            frag,
            vec![
                SemParam { index: 0, name: "a".into(), ty: SemTy::Bv(32), signed: false },
                SemParam { index: 1, name: "b".into(), ty: SemTy::Bv(32), signed: false },
            ],
            SemTy::Bv(32),
        ));
    }

    // 2. 20 acyclic CFG fragments (`br i1`, `br label`, `phi`, guarded `udiv`/`sdiv`)
    for idx in 0..20 {
        let frag = if idx % 2 == 0 {
            "  %z = icmp eq i32 %1, 0\n\
             \x20 br i1 %z, label %zero, label %nz\n\
             zero:\n\
             \x20 %fb = xor i32 %0, 42\n\
             \x20 br label %join\n\
             nz:\n\
             \x20 %q = udiv i32 %0, %1\n\
             \x20 %rem = urem i32 %0, %1\n\
             \x20 %comb = add i32 %q, %rem\n\
             \x20 br label %join\n\
             join:\n\
             \x20 %r = phi i32 [ %fb, %zero ], [ %comb, %nz ]\n\
             \x20 ret i32 %r\n"
                .to_string()
        } else {
            "  %z = icmp eq i32 %1, 0\n\
             \x20 %m = icmp eq i32 %0, -2147483648\n\
             \x20 %n1 = icmp eq i32 %1, -1\n\
             \x20 %ov = and i1 %m, %n1\n\
             \x20 %bad = or i1 %z, %ov\n\
             \x20 br i1 %bad, label %fail, label %ok\n\
             fail:\n\
             \x20 br label %done\n\
             ok:\n\
             \x20 %q = sdiv i32 %0, %1\n\
             \x20 %rem = srem i32 %0, %1\n\
             \x20 %s = xor i32 %q, %rem\n\
             \x20 br label %done\n\
             done:\n\
             \x20 %r = phi i32 [ -1, %fail ], [ %s, %ok ]\n\
             \x20 ret i32 %r\n"
                .to_string()
        };
        let fname = format!("diff_cfg_{idx}");
        llvm_ir.push_str(&format!("define i32 @{fname}(i32 %0, i32 %1) {{\nentry:\n{frag}}}\n\n"));
        fragments.push((
            fname,
            frag,
            vec![
                SemParam { index: 0, name: "a".into(), ty: SemTy::Bv(32), signed: true },
                SemParam { index: 1, name: "b".into(), ty: SemTy::Bv(32), signed: true },
            ],
            SemTy::Bv(32),
        ));
    }

    // 3. 16 SIMD vector fragments (`<4 x i8>`, `<4 x i32>` packed into `i32` / `i128` bitcasts)
    for idx in 0..16 {
        let vop = ["add", "sub", "xor", "and"][idx % 4];
        let frag = format!(
            "  %va = bitcast i32 %0 to <4 x i8>\n\
             \x20 %vb = bitcast i32 %1 to <4 x i8>\n\
             \x20 %vc = {vop} <4 x i8> %va, %vb\n\
             \x20 %e0 = extractelement <4 x i8> %vc, i64 0\n\
             \x20 %v2 = insertelement <4 x i8> %vc, i8 %e0, i64 3\n\
             \x20 %cmp = icmp ult <4 x i8> %v2, %va\n\
             \x20 %msk = sext <4 x i1> %cmp to <4 x i8>\n\
             \x20 %fin = xor <4 x i8> %v2, %msk\n\
             \x20 %r = bitcast <4 x i8> %fin to i32\n\
             \x20 ret i32 %r\n"
        );
        let fname = format!("diff_vec_{idx}");
        llvm_ir.push_str(&format!("define i32 @{fname}(i32 %0, i32 %1) {{\nentry:\n{frag}}}\n\n"));
        fragments.push((
            fname,
            frag,
            vec![
                SemParam { index: 0, name: "a".into(), ty: SemTy::Bv(32), signed: false },
                SemParam { index: 1, name: "b".into(), ty: SemTy::Bv(32), signed: false },
            ],
            SemTy::Bv(32),
        ));
    }

    assert!(fragments.len() >= 150);

    // Build C harness invoking all 156 functions across edge-case + random input pairs.
    let mut c_src = String::from("#include <stdint.h>\n#include <stdio.h>\n\n");
    for (fname, _, _, _) in &fragments {
        c_src.push_str(&format!("extern uint32_t {fname}(uint32_t, uint32_t);\n"));
    }
    let test_inputs: Vec<(u32, u32)> = vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (0x8000_0000, 0xffff_ffff),
        (0x7fff_ffff, 1),
        (0xffff_ffff, 0xffff_ffff),
        (0x1234_5678, 0x9abc_def0),
        (42, 7),
        (0xdead_beef, 31),
        (0x00ff_00ff, 0xff00_ff00),
    ];
    c_src.push_str("int main(void) {\n");
    c_src.push_str("  const uint32_t A[] = {");
    for (a, _) in &test_inputs {
        c_src.push_str(&format!("{a}u,"));
    }
    c_src.push_str("};\n  const uint32_t B[] = {");
    for (_, b) in &test_inputs {
        c_src.push_str(&format!("{b}u,"));
    }
    c_src.push_str("};\n");
    for (fname, _, _, _) in &fragments {
        c_src.push_str(&format!(
            "  for (int i = 0; i < {}; ++i) printf(\"%s %d %u\\n\", \"{fname}\", i, {fname}(A[i], B[i]));\n",
            test_inputs.len()
        ));
    }
    c_src.push_str("  return 0;\n}\n");

    let tmp = tempfile::tempdir().expect("tempdir");
    let ll_path = tmp.path().join("diff.ll");
    let c_path = tmp.path().join("driver.c");
    let exe_path = tmp.path().join("diff_exe");
    std::fs::write(&ll_path, &llvm_ir).unwrap();
    std::fs::write(&c_path, &c_src).unwrap();

    let compile = Command::new("clang")
        .args(["-O2", "-Wno-override-module"])
        .arg(&ll_path)
        .arg(&c_path)
        .arg("-o")
        .arg(&exe_path)
        .output()
        .expect("clang compile");
    assert!(
        compile.status.success(),
        "clang failed:\n{}\n{}",
        String::from_utf8_lossy(&compile.stdout),
        String::from_utf8_lossy(&compile.stderr)
    );

    let run = Command::new(&exe_path).output().expect("run diff_exe");
    assert!(run.status.success(), "diff_exe failed");
    let stdout = String::from_utf8_lossy(&run.stdout);
    let mut clang_results = std::collections::HashMap::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            let idx: usize = parts[1].parse().unwrap();
            let val: u32 = parts[2].parse().unwrap();
            clang_results.insert((parts[0].to_string(), idx), val);
        }
    }

    // Compare EVERY SINGLE fragment x input pair against `llvm-sem@21.1` concrete evaluator!
    let mut checked_cases = 0usize;
    for (fname, frag, params, ret_ty) in &fragments {
        let prog = lower_llvm_fragment(frag, params, *ret_ty)
            .unwrap_or_else(|e| panic!("lower {fname}: {e}"));
        let (res_node, res_poison) = prog.result.as_scalar().expect("scalar");
        for (i, &(a, b)) in test_inputs.iter().enumerate() {
            let args = [
                ConcreteVal::Bv { width: 32, bits: a as u128 },
                ConcreteVal::Bv { width: 32, bits: b as u128 },
            ];
            let ub = eval_node(&prog.graph, prog.ub_cond, &args).as_bool();
            let poison = eval_node(&prog.graph, res_poison, &args).as_bool();
            assert!(!ub && !poison, "{fname} on ({a:#x}, {b:#x}) had unexpected ub={ub} poison={poison}");
            let (_, bits) = eval_node(&prog.graph, res_node, &args).as_bv();
            let expected = bits as u32;
            let got = clang_results[&(fname.clone(), i)];
            assert_eq!(
                expected, got,
                "DIFFERENTIAL MISMATCH on {fname}({a:#x}, {b:#x}): llvm-sem={expected:#x}, clang={got:#x}\n{frag}"
            );
            checked_cases += 1;
        }
    }
    assert_eq!(checked_cases, fragments.len() * test_inputs.len());
}

#[test]
fn translation_validation_proves_simplify_and_codegen_with_lrat_and_catches_tamper() {
    let src = r#"
module Main where
  mix :: U32 -> U32 -> U32
  mix a b =
    let s = wrapping-add-u32 a b in
    let x = bit-xor-u32 s a in
    if eq-u32 x 0 then wrapping-sub-u32 a b else x

  main :: Int
  main = if mix 10 20 == 20 then 0 else 1
"#;
    let program = check_sources(src, "<tv>").expect("check");
    let prim_bodies = collect_program_prim_bodies(&program);

    // 1. Core -> Core optimizer (`simplify::simplify`) translation validation!
    let mut simplified = program.clone();
    let mut srep = cinch::core::SimplifyReport::default();
    cinch::simplify::simplify(&mut simplified, &cinch::simplify::SimplifyOptions::default(), &mut srep);
    let f_before = program.fns.iter().find(|f| f.id.0 == "Main.mix").expect("Main.mix before");
    let f_after = simplified.fns.iter().find(|f| f.id.0 == "Main.mix").expect("Main.mix after");
    let opt_ob = build_core_transform_obligation(f_before, f_after, &prim_bodies, "simplify")
        .expect("build Core->Core TV obligation");
    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let opt_rep = engine.verify(&opt_ob);
    assert!(
        matches!(
            opt_rep.status,
            VerificationStatus::Verified {
                receipt: TrustReceipt { tier: TrustTier::KernelCert, .. },
                ..
            }
        ),
        "expected simplify TV to verify with LRAT, got {opt_rep:?}"
    );

    // 2. Core -> emitted LLVM IR (`codegen::emit`) translation validation!
    let ir = cinch::codegen::emit(&program).expect("emit IR");
    let reports = validate_program_codegen(&program, &ir, "Main.mix");
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].status, "verified", "got {:?}", reports[0]);

    // 3. Tampering with the emitted LLVM IR (swapping `xor` to `or`) MUST be disproved!
    let bad_ir = ir.replace("xor i32", "or i32");
    assert_ne!(bad_ir, ir, "expected emitted IR to contain `xor i32`");
    let bad_ob = build_core_vs_llvm_obligation(f_before, &bad_ir, &prim_bodies)
        .expect("build tampered Core->LLVM obligation");
    match engine.verify(&bad_ob).status {
        VerificationStatus::Disproved { counterexample, .. } => {
            assert!(counterexample.validated);
        }
        other => panic!("expected tampered LLVM IR to be disproved, got {other:?}"),
    }
}

#[test]
fn verified_rewrite_rules_prove_valid_identities_and_reject_bogus_rewrites() {
    let good = r#"
module Main where
  rewrite xor-cancel forall (x :: U32) (y :: U32). where
    match (bit-xor-u32 (bit-xor-u32 x y) y)
    replace x
    justify verify

  rewrite double-shift forall (x :: U32). where
    match (wrapping-add-u32 x x)
    replace (wrapping-shl-u32 x 1)
    justify verify

  main :: Int
  main =
    let a = bit-xor-u32 (bit-xor-u32 123 456) 456 in
    let b = wrapping-add-u32 21 21 in
    if a == 123 && b == 42 then 0 else 1
"#;
    let code = common::source_frontend::run_source(good).expect("verified rewrites should compile & run");
    assert_eq!(code, 0);

    let bad = r#"
module Main where
  rewrite bogus-rule forall (x :: U32). where
    match (wrapping-add-u32 x x)
    replace (wrapping-add-u32 x 1)
    justify verify

  main :: Int
  main = 0
"#;
    let err = check_sources(bad, "<bad-rewrite>").expect_err("bogus rewrite must be rejected");
    let msg = err.to_string();
    assert!(
        msg.contains("failed formal verification"),
        "expected formal verification failure, got: {msg}"
    );
}

#[test]
fn memory_and_atomic_boundary_contracts_audit_prelude_cleanly_and_catch_violations() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for rel in [
        "prelude/Effect/Stm.cinch",
        "prelude/Prim/Sys/Linux.cinch",
        "prelude/Prim/Sys/Darwin.cinch",
    ] {
        let rep = audit_file_contracts(&root.join(rel))
            .unwrap_or_else(|e| panic!("audit {rel} failed: {e}"));
        assert!(
            rep.violations.is_empty(),
            "{rel} had boundary contract violations: {:?}",
            rep.violations
        );
        assert!(rep.unsafe_bodies_audited > 0, "{rel} should audit unsafe bodies");
    }

    // Negative tests: invalid atomic orderings, alignment, pure effect leak, missing syscall clobbers!
    let bad_contracts = r#"
module Bad where
  prim bad-load-rel :: I64 ->{STM} I64 where
    unsafe llvm {
      %p = inttoptr i64 %0 to ptr
      %v = load atomic i64, ptr %p release, align 4
      ret i64 %v
    }
  prim bad-cas :: I64 -> I64 -> I64 ->{STM} I64 where
    unsafe llvm {
      %p = inttoptr i64 %0 to ptr
      %r = cmpxchg ptr %p, i64 %1, i64 %2 monotonic seq_cst
      %ok = extractvalue { i64, i1 } %r, 1
      %z = zext i1 %ok to i64
      ret i64 %z
    }
  prim bad-pure-store :: I64 -> I64 -> I64 where
    unsafe llvm {
      %p = inttoptr i64 %0 to ptr
      store atomic i64 %1, ptr %p acquire, align 8
      ret i64 %1
    }
  prim bad-syscall :: I64 -> I64 where
    unsafe llvm {
      %r = call i64 asm sideeffect inteldialect "mov rax, 39\0A syscall\0A mov $0, rax", "={rax},~{memory}"()
      ret i64 %r
    }
"#;
    let rep = audit_source_contracts(bad_contracts).expect("parse bad contracts");
    let rules: Vec<&str> = rep.violations.iter().map(|v| v.rule.as_str()).collect();
    assert!(rules.contains(&"atomic-ordering"), "missing atomic-ordering: {rep:?}");
    assert!(rules.contains(&"atomic-alignment"), "missing atomic-alignment: {rep:?}");
    assert!(rules.contains(&"effect-accountability"), "missing effect-accountability: {rep:?}");
    assert!(rules.contains(&"inline-asm-clobbers"), "missing inline-asm-clobbers: {rep:?}");
}

#[test]
fn target_arch_pointer_and_word_size_end_to_end() {
    use cinch::core::{ArtifactProfile, TargetProfile};
    use cinch::driver::check_sources_profile;
    use cinch::prim_verify::mem_contract::audit_source_contracts_for_word_bits;
    use cinch::prim_verify::translate_val::validate_program_codegen_for_word_bits;

    // 1. Atomic `ptr` load alignment obeys target pointer width (4 bytes on 32-bit, 8 bytes on 64-bit).
    let ptr_atomic_src = r#"
module PtrAtomic where
  prim load-ptr :: Usize ->{STM} Usize where
    unsafe llvm {
      %p = inttoptr i32 %0 to ptr
      %v = load atomic ptr, ptr %p acquire, align 4
      %r = ptrtoint ptr %v to i32
      ret i32 %r
    }
"#;
    let rep32 = audit_source_contracts_for_word_bits(ptr_atomic_src, 32).expect("audit 32-bit");
    assert!(
        rep32.violations.is_empty(),
        "32-bit ptr align 4 should pass, got: {:?}",
        rep32.violations
    );
    let rep64 = audit_source_contracts_for_word_bits(ptr_atomic_src, 64).expect("audit 64-bit");
    assert!(
        rep64.violations.iter().any(|v| v.rule == "atomic-alignment"),
        "64-bit ptr align 4 must fail atomic-alignment, got: {:?}",
        rep64.violations
    );

    // 2. FFI C ABI, `repr(C)` layout, `prim` verification, verified `rewrite`, and Core->LLVM TV
    //    all use 32-bit / 4-byte pointers on `wasm32` and 64-bit / 8-byte pointers on 64-bit native.
    let wasm32_src = r#"
module Main where
  #[repr(C)]
  record PtrPair where
    tag :: U8
    len :: Usize
    buf :: RawPtr U8

  foreign import ccall "c_touch" c-touch :: forall r . RawPtr U8 -> Usize -> Isize ->{IO r} Usize

  prim add-usize-word :: Usize -> Usize -> Usize where
    llvm {
      %r = add i32 %0, %1
      ret i32 %r
    } justify verify

  rewrite usize-xor-cancel forall (x :: Usize) (y :: Usize). where
    match (bit-xor-usize (bit-xor-usize x y) y)
    replace x
    justify verify

  dbl-usize :: Usize -> Usize
  dbl-usize x = add-usize-word x x

  main :: Int
  main = if dbl-usize 21 == 42 then 0 else 1
"#;
    let wasm_profile = TargetProfile::wasm32(ArtifactProfile::CoreWasm);
    let prog32 = check_sources_profile(wasm32_src, "<wasm32-ptrs>", &wasm_profile)
        .expect("wasm32 target should verify 32-bit Usize prim, rewrite, repr(C), and FFI");

    // Check `repr(C)` layout for `PtrPair`: U8(0..1) + pad(3) + Usize(4..8) + RawPtr(8..12) = size 12, align 4.
    let pair32 = prog32
        .plans
        .get("Main.PtrPair")
        .expect("PtrPair ctor layout on wasm32");
    assert_eq!(pair32.payload, 12, "wasm32 repr(C) PtrPair payload should be 12 bytes");
    assert_eq!(pair32.fields[0].offset, 0);
    assert_eq!(pair32.fields[0].size, 1);
    assert_eq!(pair32.fields[1].offset, 4);
    assert_eq!(pair32.fields[1].size, 4);
    assert_eq!(pair32.fields[2].offset, 8);
    assert_eq!(pair32.fields[2].size, 4);

    // Check FFI ABI plan widths on wasm32: 4-byte ptr, Usize, Isize.
    let ffi32 = prog32
        .foreigns
        .values()
        .find(|f| f.symbol == "c_touch")
        .expect("c_touch foreign plan on wasm32");
    assert_eq!(ffi32.params[0].width, 4);
    assert_eq!(ffi32.params[0].align, 4);
    assert_eq!(ffi32.params[1].width, 4);
    assert_eq!(ffi32.params[2].width, 4);
    assert_eq!(ffi32.result.width, 4);

    // Check emitted LLVM IR uses `i32` for `c_touch` on wasm32.
    let ir32 = cinch::codegen::emit_for(&wasm_profile, &prog32).expect("emit wasm32 IR");
    assert!(
        ir32.contains("declare i32 @\"c_touch\"(ptr nonnull, i32 noundef, i32 noundef)"),
        "wasm32 FFI declare must use i32 for Usize/Isize:\n{ir32}"
    );

    // Core -> LLVM translation validation at 32 bits for `Main.dbl-usize`.
    let tv32 = validate_program_codegen_for_word_bits(&prog32, &ir32, "Main.dbl-usize", 32);
    assert!(
        tv32.iter().any(|r| r.fn_name == "Main.dbl-usize" && r.status == "verified"),
        "expected Main.dbl-usize to verify via LRAT TV at 32 bits, got: {tv32:?}"
    );

    // The same 32-bit `i32` baseline Usize prim MUST be rejected on 64-bit native.
    let native_profile = TargetProfile::native();
    assert!(
        check_sources_profile(wasm32_src, "<native64-ptrs-bad>", &native_profile).is_err(),
        "32-bit i32 Usize prim must fail on 64-bit native target"
    );

    // On 64-bit native, the matching 64-bit version gives 8-byte ptr/Usize/Isize and 24-byte PtrPair.
    let native64_src = wasm32_src.replace("i32", "i64");
    let prog64 = check_sources_profile(&native64_src, "<native64-ptrs>", &native_profile)
        .expect("64-bit native target should verify 64-bit Usize prim, rewrite, repr(C), and FFI");
    let pair64 = prog64
        .plans
        .get("Main.PtrPair")
        .expect("PtrPair ctor layout on 64-bit");
    assert_eq!(pair64.payload, 24, "64-bit repr(C) PtrPair payload should be 24 bytes");
    assert_eq!(pair64.fields[1].offset, 8);
    assert_eq!(pair64.fields[1].size, 8);
    assert_eq!(pair64.fields[2].offset, 16);
    assert_eq!(pair64.fields[2].size, 8);
    let ir64 = cinch::codegen::emit_for(&native_profile, &prog64).expect("emit 64-bit IR");
    assert!(
        ir64.contains("declare i64 @\"c_touch\"(ptr nonnull, i64 noundef, i64 noundef)"),
        "64-bit FFI declare must use i64 for Usize/Isize:\n{ir64}"
    );
}

#[test]
fn wasm32_clang_backend_and_repr_c_layout_parity() {
    use cinch::core::{ArtifactProfile, TargetProfile};
    use cinch::driver::check_sources_profile;

    let src = r#"
module Main where
  #[repr(C)]
  record Mixed where
    tag :: U8
    ptr-idx :: Usize
    count :: U32
    delta :: Isize

  #[repr(packed)]
  record Tight where
    a :: U8
    b :: Usize

  #[repr(C, align(16))]
  record Aligned where
    x :: U32

  prim add-usize-word :: Usize -> Usize -> Usize where
    llvm {
      %r = add i32 %0, %1
      ret i32 %r
    } when word-size 32 justify verify
    llvm {
      %r = add i64 %0, %1
      ret i64 %r
    } justify verify

  main :: Int
  main = if add-usize-word 20 22 == 42 then 0 else 1
"#;

    let wasm_profile = TargetProfile::wasm32(ArtifactProfile::CoreWasm);
    let native_profile = TargetProfile::native();

    // 1. Check `abi.layouts` parity through `common::check_interfaces_profile` (runs on BOTH seed and Stage 3!).
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir32 = tmp.path().join("wasm32");
    let dir64 = tmp.path().join("native64");
    std::fs::create_dir_all(&dir32).unwrap();
    std::fs::create_dir_all(&dir64).unwrap();
    std::fs::write(dir32.join("Main.cinch"), src).unwrap();
    std::fs::write(dir64.join("Main.cinch"), src).unwrap();

    let ifaces32 = common::check_interfaces_profile(&dir32, "Main.cinch", &wasm_profile)
        .expect("wasm32 repr(C) interfaces");
    let main32 = ifaces32.iter().find(|i| i.module == "Main").expect("Main wasm32 iface");
    let mixed32 = main32.layouts.iter().find(|l| l.name == "Mixed").expect("Mixed wasm32 layout");
    assert_eq!(mixed32.repr, "C");
    assert_eq!(mixed32.size, 16);
    assert_eq!(mixed32.align, 4);
    assert_eq!(mixed32.fields[0].offset, 0);
    assert_eq!(mixed32.fields[1].offset, 4);
    assert_eq!(mixed32.fields[1].size, 4);
    assert_eq!(mixed32.fields[2].offset, 8);
    assert_eq!(mixed32.fields[3].offset, 12);
    assert_eq!(mixed32.fields[3].size, 4);

    let tight32 = main32.layouts.iter().find(|l| l.name == "Tight").expect("Tight wasm32 layout");
    assert_eq!(tight32.repr, "packed");
    assert_eq!(tight32.size, 5);
    assert_eq!(tight32.align, 1);

    let aligned32 = main32.layouts.iter().find(|l| l.name == "Aligned").expect("Aligned wasm32 layout");
    assert_eq!(aligned32.repr, "C, align(16)");
    assert_eq!(aligned32.size, 16);
    assert_eq!(aligned32.align, 16);

    let ifaces64 = common::check_interfaces_profile(&dir64, "Main.cinch", &native_profile)
        .expect("native64 repr(C) interfaces");
    let main64 = ifaces64.iter().find(|i| i.module == "Main").expect("Main native64 iface");
    let mixed64 = main64.layouts.iter().find(|l| l.name == "Mixed").expect("Mixed native64 layout");
    assert_eq!(mixed64.repr, "C");
    assert_eq!(mixed64.size, 32);
    assert_eq!(mixed64.align, 8);
    assert_eq!(mixed64.fields[1].offset, 8);
    assert_eq!(mixed64.fields[1].size, 8);
    assert_eq!(mixed64.fields[2].offset, 16);
    assert_eq!(mixed64.fields[3].offset, 24);
    let tight64 = main64.layouts.iter().find(|l| l.name == "Tight").expect("Tight native64 layout");
    assert_eq!(tight64.size, 9);
    assert_eq!(tight64.align, 1);

    // 2. Emit real `wasm32` LLVM IR and compile it through clang's WebAssembly backend (`-S` and `-c`)!
    let prog32 = check_sources_profile(src, "<wasm32-codegen>", &wasm_profile)
        .expect("check wasm32 codegen module");
    let ir32 = cinch::codegen::emit_for(&wasm_profile, &prog32).expect("emit wasm32 IR");
    assert!(ir32.contains("target triple = \"wasm32-unknown-unknown\""));
    let ll_path = tmp.path().join("wasm32_module.ll");
    let s_path = tmp.path().join("wasm32_module.s");
    let o_path = tmp.path().join("wasm32_module.o");
    std::fs::write(&ll_path, &ir32).unwrap();

    let s_out = std::process::Command::new("clang")
        .args(["--target=wasm32-unknown-unknown", "-x", "ir", "-S"])
        .arg(&ll_path)
        .arg("-o")
        .arg(&s_path)
        .output()
        .expect("run clang wasm32 -S");
    assert!(
        s_out.status.success(),
        "clang --target=wasm32-unknown-unknown -S failed:\n{}",
        String::from_utf8_lossy(&s_out.stderr)
    );
    let asm = std::fs::read_to_string(&s_path).expect("read wasm32 .s");
    assert!(
        asm.contains("i32.add"),
        "expected real WebAssembly `i32.add` in wasm32 assembly:\n{}",
        &asm[..asm.len().min(2000)]
    );

    let o_out = std::process::Command::new("clang")
        .args(["--target=wasm32-unknown-unknown", "-x", "ir", "-c"])
        .arg(&ll_path)
        .arg("-o")
        .arg(&o_path)
        .output()
        .expect("run clang wasm32 -c");
    assert!(
        o_out.status.success(),
        "clang --target=wasm32-unknown-unknown -c failed:\n{}",
        String::from_utf8_lossy(&o_out.stderr)
    );
    let obj_bytes = std::fs::read(&o_path).expect("read wasm32 .o");
    assert!(
        obj_bytes.starts_with(b"\0asm"),
        "expected genuine WebAssembly binary magic (\\0asm), got {:?}",
        &obj_bytes[..obj_bytes.len().min(8)]
    );
}

#[test]
fn prelude_verified_rewrites_and_verify_codegen_cli() {
    // 1. Prelude verified algebraic rewrites in Data.Machine & Data.Wide fire & execute cleanly.
    let src = r#"
module Main where
  import Data.Wide

  dbl32 :: U32 -> U32
  dbl32 x = wrapping-add-u32 x x

  cancel64 :: U64 -> U64 -> U64
  cancel64 x y = bit-xor-u64 (bit-xor-u64 x y) y

  id128 :: U128 -> U128
  id128 x = bit-not-u128 (bit-not-u128 (wrapping-add-u128 x 0))

  main :: Int
  main =
    let a = dbl32 21 in
    let b = cancel64 100 777 in
    let c = id128 99 in
    let z = bit-and-u128 c 0 in
    if a == 42 && b == 100 && c == 99 && z == 0 then 0 else 1
"#;
    let code = common::source_frontend::run_source(src)
        .expect("prelude verified rewrites program should compile & run");
    assert_eq!(code, 0);

    // 2. `cinch verify-codegen` CLI verifies scalar functions on both host and wasm32!
    let tmp = tempfile::tempdir().expect("tempdir");
    let file_path = tmp.path().join("tv_cli.cinch");
    std::fs::write(&file_path, src).unwrap();
    let rc_host = cinch::prim_verify::cli_verify_codegen(&[file_path.to_string_lossy().to_string()]);
    assert_eq!(rc_host, 0, "cinch verify-codegen should exit 0 on valid scalar program");
    let rc_wasm = cinch::prim_verify::cli_verify_codegen(&[
        file_path.to_string_lossy().to_string(),
        "--target=wasm32-unknown-unknown".to_string(),
    ]);
    assert_eq!(rc_wasm, 0, "cinch verify-codegen --target=wasm32-unknown-unknown should exit 0");
}

#[test]
fn float_refinements_not_nan_finite_in_range_and_verified_ieee754_prims() {
    // 1. Runtime `decide (NotNaN x)` / `decide (Finite x)` + static `Finite x => NotNaN x` implication
    //    + LRAT-verified IEEE-754 float prims preserving `NotNaN`, `Finite`, and `InRange`!
    let src = r#"
module Main where
  -- `abs` preserves `NotNaN`, `Finite`, and maps `InRange (-10) 10` into `InRange 0 10` —
  -- all proved by our built-in kernel-checked LRAT bit-blaster over IEEE-754 bits!
  prim abs-finite-f64 :: (x :: refined (Finite x) F64) -> refined (Finite r) F64 where
    llvm {
      %r = call double @llvm.fabs.f64(double %0)
      ret double %r
    } justify verify

  prim neg-not-nan-f32 :: (x :: refined (NotNaN x) F32) -> refined (NotNaN r) F32 where
    llvm {
      %r = fneg float %0
      ret float %r
    } justify verify

  prim abs-range-f64 :: (x :: refined (InRange (-10) 10 x) F64) -> refined (InRange 0 10 r) F64 where
    llvm {
      %r = call double @llvm.fabs.f64(double %0)
      ret double %r
    } justify verify

  use-not-nan :: using (NotNaN x) => (x :: F64) -> F64
  use-not-nan x = neg-f64 x

  -- `Finite x` statically implies `NotNaN x` without an extra runtime check!
  finite-to-not-nan :: using (Finite x) => (x :: F64) -> F64
  finite-to-not-nan x = use-not-nan x

  main :: Int
  main =
    let nan64 = f64-from-bits 9221120237041090560 in
    let inf64 = f64-from-bits 9218868437227405312 in
    let ok64 = i32-to-f64 42 in
    let r1 = case decide (NotNaN nan64) of Yes _ -> 1; No _ -> 0 in
    let r2 = case decide (Finite inf64) of Yes _ -> 1; No _ -> 0 in
    let r3 = case decide (NotNaN inf64) of Yes _ -> 0; No _ -> 1 in
    let r4 = (case decide (Finite ok64) of
      Yes _ ->
        let _ = finite-to-not-nan ok64 in
        let _ = abs-finite-f64 ok64 in
        0
      No _ -> 1) in
    if r1 == 0 && r2 == 0 && r3 == 0 && r4 == 0 then 0 else 1
"#;
    let code = common::source_frontend::run_source(src)
        .expect("float refinements and verified IEEE-754 prims should compile & run");
    assert_eq!(code, 0);

    // 2. Bogus IEEE-754 float refinement postcondition MUST be disproved with a counterexample!
    let bad_fp_post = r#"
module Main where
  -- `fneg` does NOT turn NaN into NotNaN!
  prim bogus-neg-not-nan :: F32 -> refined (NotNaN r) F32 where
    llvm {
      %r = fneg float %0
      ret float %r
    } justify verify

  main :: Int
  main = 0
"#;
    let err = check_sources(bad_fp_post, "<bad-fp-post>")
        .expect_err("bogus float NotNaN postcondition must be disproved");
    assert!(
        err.to_string().contains("failed formal verification"),
        "expected formal verification counterexample, got: {err}"
    );
}

#[test]
fn verify_codegen_interproc_tail_loops_aggregates_and_perceus_rc() {
    use cinch::prim_verify::sem::SemParam;
    use std::collections::HashMap;

    // 1. Real Cinch program with:
    //    - interprocedural call (`caller` -> `#[inline(never)] helper`),
    //    - self-tail-recursive loop (`sum-down`),
    //    - heap allocation + Perceus RC (`pair-sum` / `main`).
    let src = r#"
module Main where
  import Data.Machine
  export helper, caller, sum-down, pair-sum, main

  #[inline(never)]
  helper :: U64 -> U64
  helper x = wrapping-add-u64 x x

  caller :: U64 -> U64
  caller x = wrapping-add-u64 (helper x) 1

  sum-down :: U64 -> U64 -> U64
  sum-down n acc =
    if eq-u64 n 0 then acc
    else sum-down (wrapping-sub-u64 n 1) (wrapping-add-u64 acc n)

  #[repr(C)]
  data Pair where
    MkPair :: U64 -> U64 -> Pair

  #[inline(never)]
  pair-sum :: Pair -> U64
  pair-sum p = case p of
    MkPair x y -> wrapping-add-u64 x y

  main :: Int
  main =
    let p = MkPair 1 2 in
    let s = wrapping-add-u64 (caller 3) (wrapping-add-u64 (sum-down 5 0) (pair-sum p)) in
    if eq-u64 s 0 then 1 else 0
"#;
    let program = check_sources(src, "<tv-ext>").expect("extended TV sample should check");
    let program = cinch::rc::insert(program, &cinch::core::TargetProfile::native()).expect("rc insert");
    let ir = cinch::codegen::emit(&program).expect("emit IR");

    let reports = validate_program_codegen(&program, &ir, "Main.");
    assert!(
        reports.iter().any(|r| r.fn_name == "Main.helper" && r.status == "verified"),
        "Main.helper should be LRAT-verified, got: {reports:?}"
    );
    assert!(
        reports.iter().any(|r| r.fn_name == "Main.caller" && r.status == "verified"),
        "interprocedural Main.caller should be LRAT-verified, got: {reports:?}"
    );
    assert!(
        reports.iter().any(|r| {
            r.fn_name == "Main.sum-down"
                && r.stage == "core->llvm(loop-step)"
                && r.status == "verified"
        }),
        "tail-recursive Main.sum-down should be LRAT loop-step bisimulation verified, got: {reports:?}"
    );
    assert!(
        reports.iter().any(|r| r.stage == "rc-safety" && r.status == "verified"),
        "Perceus RC-managed function should pass rc-safety, got: {reports:?}"
    );
    assert!(
        reports.iter().all(|r| r.status != "disproved"),
        "valid emitted IR must have zero disproved reports, got: {reports:?}"
    );

    // 2. Tamper with the tail-recursive loop latch (`wrapping-sub-u64` -> `wrapping-add-u64`):
    //    inductive loop-step bisimulation MUST DISPROVE it with a counterexample!
    let sum_fn = program.fns.iter().find(|f| f.id.0 == "Main.sum-down").unwrap();
    let prim_bodies = collect_program_prim_bodies(&program);
    let all_fns: HashMap<String, _> = program.fns.iter().map(|f| (f.id.0.clone(), f)).collect();
    let sd_pos = ir.find("@\"Main.sum-down\"").expect("Main.sum-down in IR");
    let tampered_loop_ir = format!(
        "{}{}",
        &ir[..sd_pos],
        ir[sd_pos..].replacen(
            "call i64 @\"Prim.Machine.wrapping-sub-u64\"",
            "call i64 @\"Prim.Machine.wrapping-add-u64\"",
            1,
        )
    );
    let ob_bad_loop = build_core_vs_llvm_obligation_with_fns(
        sum_fn,
        &tampered_loop_ir,
        &prim_bodies,
        &all_fns,
        64,
    )
    .expect("tampered loop should still build bisimulation obligation");
    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let rep_bad_loop = engine.verify(&ob_bad_loop);
    assert!(
        matches!(rep_bad_loop.status, VerificationStatus::Disproved { .. }),
        "tampered tail-loop latch MUST be disproved, got: {:?}",
        rep_bad_loop.status
    );

    // 3. Aggregate #[repr(C)] struct + FixedArray symbolic memory equivalence (LRAT) + OOB rejection!
    let p0 = SemParam { index: 0, name: "a".into(), ty: SemTy::Bv(64), signed: false };
    let p1 = SemParam { index: 1, name: "b".into(), ty: SemTy::Bv(64), signed: false };
    let struct_ir = r#"
      %s = alloca { i64, i64 }, align 8
      %f0 = getelementptr inbounds { i64, i64 }, ptr %s, i64 0, i32 0
      store i64 %0, ptr %f0, align 8
      %f1 = getelementptr inbounds { i64, i64 }, ptr %s, i64 0, i32 1
      store i64 %1, ptr %f1, align 8
      %a = load i64, ptr %f0, align 8
      %b = load i64, ptr %f1, align 8
      %sum = add i64 %a, %b
      ret i64 %sum
    "#;
    let array_ir = r#"
      %arr = alloca [2 x i64], align 8
      %e0 = getelementptr inbounds [2 x i64], ptr %arr, i64 0, i64 0
      store i64 %0, ptr %e0, align 8
      %e1 = getelementptr inbounds [2 x i64], ptr %arr, i64 0, i64 1
      store i64 %1, ptr %e1, align 8
      %x = load i64, ptr %e1, align 8
      %y = load i64, ptr %e0, align 8
      %sum = add i64 %x, %y
      ret i64 %sum
    "#;
    let ob_mem = validate_llvm_memory_equiv(
        "repr_c_vs_fixed_array",
        struct_ir,
        array_ir,
        &[p0.clone(), p1.clone()],
        SemTy::Bv(64),
    )
    .expect("valid struct vs FixedArray memory fragments should lower");
    assert!(
        matches!(engine.verify(&ob_mem).status, VerificationStatus::Verified { .. }),
        "#[repr(C)] struct and FixedArray commutative add should verify with LRAT"
    );

    let oob_array_ir = r#"
      %arr = alloca [2 x i64], align 8
      %bad = getelementptr inbounds [2 x i64], ptr %arr, i64 0, i64 2
      store i64 %0, ptr %bad, align 8
      ret i64 %0
    "#;
    let oob_err = validate_llvm_memory_equiv(
        "oob_array",
        struct_ir,
        oob_array_ir,
        &[p0.clone(), p1.clone()],
        SemTy::Bv(64),
    )
    .expect_err("out-of-bounds FixedArray GEP MUST be rejected");
    assert!(
        oob_err.contains("out-of-bounds"),
        "expected out-of-bounds error, got: {oob_err}"
    );

    // 4. Perceus RC safety validator: catches double-free, use-after-free, and RC leak!
    let rc_double_free = r#"
define i64 @"Main.bad-double-free"(i64 %p0) {
entry:
  call void @cinch.rc.drop.obj(i64 %p0)
  call void @cinch.rc.drop.obj(i64 %p0)
  ret i64 0
}
"#;
    let rep_df = validate_fn_rc_safety("Main.bad-double-free", rc_double_free).unwrap();
    assert_eq!(rep_df.status, "disproved");
    assert!(rep_df.details.contains("double-free"), "got: {}", rep_df.details);

    let rc_uaf = r#"
define i64 @"Main.bad-uaf"(i64 %p0) {
entry:
  call void @cinch.rc.drop.obj(i64 %p0)
  %x = call i64 @"Main.use"(i64 %p0)
  ret i64 %x
}
"#;
    let rep_uaf = validate_fn_rc_safety("Main.bad-uaf", rc_uaf).unwrap();
    assert_eq!(rep_uaf.status, "disproved");
    assert!(rep_uaf.details.contains("use-after-free"), "got: {}", rep_uaf.details);

    let rc_leak = r#"
define i64 @"Main.bad-leak"(i64 %p0) {
entry:
  call void @cinch.rc.dup.obj(i64 %p0)
  call void @cinch.rc.drop.obj(i64 %p0)
  ret i64 0
}
"#;
    let rep_leak = validate_fn_rc_safety("Main.bad-leak", rc_leak).unwrap();
    assert_eq!(rep_leak.status, "disproved");
    assert!(rep_leak.details.contains("RC leak"), "got: {}", rep_leak.details);

    let rc_good = r#"
define i64 @"Main.good-rc"(i64 %p0) {
entry:
  call void @cinch.rc.dup.obj(i64 %p0)
  call void @cinch.rc.drop.obj(i64 %p0)
  ret i64 %p0
}
"#;
    let mod_reports = validate_module_rc_safety(rc_good, "Main.");
    assert_eq!(mod_reports.len(), 1);
    assert_eq!(mod_reports[0].status, "verified");
}

#[test]
fn proof_producing_smt_alethe_cache_and_fp_arith_casts() {
    use cinch::prim_verify::smt::{
        emit_smtlib2, extract_alethe_proof, AletheCertificate, AletheProofChecker,
        ExternalSmtProver, ALETHE_FORMAT_ID,
    };
    use cinch::prim_verify::{
        Checker, Evidence, EvidenceStore, Policy, Prover, ProverOutcome, TrustTier,
        VerificationStatus, VerifierEngine,
    };
    use cinch::prim_verify::sem::{FpFmt, Obligation, ObligationKind, SemParam, SemTy};
    use cinch::prim_verify::llvm_ir::lower_llvm_fragment;

    // 1. Verify QF_FPBV SMT-LIB 2 emission for FP arithmetic & FP<->int casts.
    let params_add = vec![
        SemParam { index: 0, name: "a".into(), ty: SemTy::Fp(FpFmt::F64), signed: false },
        SemParam { index: 1, name: "b".into(), ty: SemTy::Fp(FpFmt::F64), signed: false },
    ];
    let mut lowered_add = lower_llvm_fragment(
        "%r = fadd double %0, %1\nret double %r\n",
        &params_add,
        SemTy::Fp(FpFmt::F64),
    )
    .expect("fadd lowers");
    let ret_add = lowered_add.result.as_scalar().unwrap().0;
    let p0 = lowered_add.graph.param(0, "a", SemTy::Fp(FpFmt::F64));
    let p1 = lowered_add.graph.param(1, "b", SemTy::Fp(FpFmt::F64));
    let rev_add = lowered_add.graph.fp_add(p1, p0);
    let comm_eq = lowered_add.graph.fp_cmp(cinch::prim_verify::sem::FpPred::Ueq, ret_add, rev_add);
    let ob_add = Obligation::new(
        "add-f64:comm",
        "llvm-sem@21.1",
        ObligationKind::FunctionalEquiv { spec_name: "add-f64-comm".to_string() },
        params_add,
        lowered_add.graph,
        comm_eq,
    );
    let smt_add = emit_smtlib2(&ob_add);
    assert!(smt_add.contains("(set-logic QF_FPBV)"), "got:\n{smt_add}");
    assert!(smt_add.contains("fp.add"), "expected fp.add in:\n{smt_add}");

    let params_cast = vec![SemParam {
        index: 0,
        name: "x".into(),
        ty: SemTy::Bv(32),
        signed: true,
    }];
    let mut lowered_cast = lower_llvm_fragment(
        "%f = sitofp i32 %0 to double\n%i = fptosi double %f to i64\nret i64 %i\n",
        &params_cast,
        SemTy::Bv(64),
    )
    .expect("sitofp/fptosi lower");
    let (cast_val, cast_poison) = lowered_cast.result.as_scalar().unwrap();
    let not_poison = lowered_cast.graph.not(cast_poison);
    let p0_cast = lowered_cast.graph.param(0, "x", SemTy::Bv(32));
    let p0_i64 = lowered_cast.graph.bv_sext(p0_cast, 64);
    let rt_eq = lowered_cast.graph.bv_eq(cast_val, p0_i64);
    let rt_goal = lowered_cast.graph.and(not_poison, rt_eq);
    let ob_cast = Obligation::new(
        "i32-f64-roundtrip:spec",
        "llvm-sem@21.1",
        ObligationKind::FunctionalEquiv { spec_name: "i32-f64-i64-roundtrip".to_string() },
        params_cast,
        lowered_cast.graph,
        rt_goal,
    );
    let smt_cast = emit_smtlib2(&ob_cast);
    assert!(smt_cast.contains("(set-logic QF_FPBV)"), "got:\n{smt_cast}");
    assert!(smt_cast.contains("to_fp"), "expected to_fp in:\n{smt_cast}");
    assert!(smt_cast.contains("fp.roundToIntegral"), "expected fp.roundToIntegral in:\n{smt_cast}");
    assert!(smt_cast.contains("fp.to_sbv"), "expected fp.to_sbv in:\n{smt_cast}");

    // 2. Test extended cvc5 Alethe rules (implies, or, not_or, not_and, equiv_*, ite*, refl, symm, trans, cong, reordering).
    let rich_alethe = r#"
(assume h1 (=> a b))
(assume h2 (and a (not b)))
(step s1 (cl (not a) b) :rule implies :premises (h1))
(step s2 (cl a) :rule and :premises (h2))
(step s3 (cl (not b)) :rule and :premises (h2))
(step s4 (cl b) :rule resolution :premises (s1 s2))
(step s5 (cl (= x x)) :rule refl)
(step s6 (cl (= (f x) (f x))) :rule cong :premises (s5))
(step s7 (cl (not (ite c t e)) c e) :rule ite1)
(step s8 (cl (not (= p q)) p (not q)) :rule equiv_pos1)
(step s9 (cl) :rule resolution :premises (s4 s3))
"#;
    let checker = AletheProofChecker;
    let good_ev = Evidence {
        format: ALETHE_FORMAT_ID.to_string(),
        prover_id: "smt-cvc5-alethe".to_string(),
        payload: serde_json::to_string(&AletheCertificate {
            goal_hash: ob_add.goal_hash.clone(),
            semantics_id: ob_add.semantics_id.clone(),
            proof: rich_alethe.to_string(),
        })
        .unwrap(),
    };
    let rcpt = checker
        .check(&ob_add, &good_ev)
        .expect("rich cvc5 Alethe proof should kernel-verify");
    assert_eq!(rcpt.tier, TrustTier::KernelCert);
    assert!(Policy::jpl_strict().admits(&rcpt).is_ok());

    // Tampered / unverified `trust` / `hole` Alethe proof must be REJECTED fail-closed.
    for bad_rule in ["hole", "trust"] {
        let bad_ev = Evidence {
            format: ALETHE_FORMAT_ID.to_string(),
            prover_id: "smt-cvc5-alethe".to_string(),
            payload: serde_json::to_string(&AletheCertificate {
                goal_hash: ob_add.goal_hash.clone(),
                semantics_id: ob_add.semantics_id.clone(),
                proof: format!("(assume h1 p)\n(step t1 (cl) :rule {bad_rule} :premises (h1))\n"),
            })
            .unwrap(),
        };
        assert!(
            checker.check(&ob_add, &bad_ev).is_err(),
            "Alethe `{bad_rule}` step must be rejected"
        );
    }

    // 3. End-to-end live `ExternalSmtProver::cvc5_alethe` -> `AletheProofChecker` -> disk `EvidenceStore`
    //    -> offline `replay-only` under `Policy::jpl_strict()` using a deterministic mock cvc5 script.
    let tmp = std::env::temp_dir().join(format!("cinch_mock_cvc5_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let mock_bin = tmp.join("mock-cvc5.sh");
    let cache_dir = tmp.join("cache");
    std::fs::write(
        &mock_bin,
        "#!/bin/sh\ncat >/dev/null\nprintf 'unsat\\n(\\n(assume h1 p)\\n(assume h2 (not p))\\n(step t1 (cl) :rule resolution :premises (h1 h2))\\n)\\n'\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&mock_bin).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&mock_bin, perms).unwrap();
    }
    let prover = ExternalSmtProver::cvc5_alethe(mock_bin.to_string_lossy().to_string());
    assert!(prover.produce_alethe);
    match prover.prove(&ob_add) {
        ProverOutcome::Proved(ev) => {
            assert_eq!(ev.format, ALETHE_FORMAT_ID);
            let extracted = extract_alethe_proof("unsat\n(\n(assume h1 p)\n(step t1 (cl) :rule resolution :premises (h1))\n)\n");
            assert!(extracted.is_some());
        }
        other => panic!("expected Proved(alethe/v1) from mock cvc5_alethe, got: {other:?}"),
    }

    // Populate persistent disk cache using live engine under JPL strict policy.
    let mut live_engine = VerifierEngine::default_engine(Policy::jpl_strict())
        .with_store(EvidenceStore::with_disk_dir(cache_dir.clone()));
    live_engine.provers.clear();
    live_engine.add_prover(Box::new(prover));
    let live_rep = live_engine.verify(&ob_add);
    match &live_rep.status {
        VerificationStatus::Verified { receipt, from_cache } => {
            assert!(!from_cache);
            assert_eq!(receipt.tier, TrustTier::KernelCert);
            assert_eq!(receipt.evidence_format, ALETHE_FORMAT_ID);
        }
        other => panic!("expected JPL-strict KernelCert from cvc5_alethe, got: {other:?}"),
    }

    // Offline `replay-only` build with ZERO provers replays the cached Alethe proof at KernelCert!
    let mut replay_engine = VerifierEngine::default_engine(Policy::jpl_strict())
        .with_store(EvidenceStore::with_disk_dir(cache_dir.clone()))
        .with_replay_only(true);
    let replay_rep = replay_engine.verify(&ob_add);
    match &replay_rep.status {
        VerificationStatus::Verified { receipt, from_cache } => {
            assert!(*from_cache);
            assert_eq!(receipt.tier, TrustTier::KernelCert);
            assert_eq!(receipt.evidence_format, ALETHE_FORMAT_ID);
        }
        other => panic!("expected offline replay-only KernelCert, got: {other:?}"),
    }

    // Tamper the cached disk Alethe certificate -> offline replay MUST fail closed!
    let disk_file = replay_engine
        .store
        .disk_path(&ob_add.goal_hash, ALETHE_FORMAT_ID)
        .unwrap();
    let tampered = std::fs::read_to_string(&disk_file)
        .unwrap()
        .replace(":rule resolution", ":rule hole");
    std::fs::write(&disk_file, tampered).unwrap();
    let mut tamper_engine = VerifierEngine::default_engine(Policy::jpl_strict())
        .with_store(EvidenceStore::with_disk_dir(cache_dir))
        .with_replay_only(true);
    let tamper_rep = tamper_engine.verify(&ob_add);
    assert!(
        matches!(tamper_rep.status, VerificationStatus::Unverified { .. }),
        "tampered cached Alethe proof MUST fail closed, got: {:?}",
        tamper_rep.status
    );
    let _ = std::fs::remove_dir_all(&tmp);
}
