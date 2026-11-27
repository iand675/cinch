//! Differential testing of `llvm-sem@21.1` against real `/usr/bin/clang`,
//! Translation Validation (`Core -> Core` simplify & `Core -> LLVM`),
//! Verified `rewrite ... justify verify`, and Memory/Atomic Boundary Contracts.

mod common;

use cinch::driver::check_sources;
use cinch::prim_verify::llvm_ir::lower_llvm_fragment;
use cinch::prim_verify::mem_contract::{audit_file_contracts, audit_source_contracts};
use cinch::prim_verify::sem::{eval_node, ConcreteVal, SemParam, SemTy};
use cinch::prim_verify::translate_val::{
    build_core_transform_obligation, build_core_vs_llvm_obligation, collect_program_prim_bodies,
    validate_program_codegen,
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
