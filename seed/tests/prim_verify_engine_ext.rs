//! Extended formal verification tests for Wave 0A (`BvShuffle16`, `BvCrc32c`,
//! `mem_contract` v2 Linux syscall & cache-line layout audits, and `tooling/verification/`).

use cinch::prim_verify::llvm_ir::lower_llvm_fragment;
use cinch::prim_verify::mem_contract::{
    audit_cache_line_layout, audit_source_contracts, lookup_linux_syscall,
    validate_linux_syscall_pair, CacheLineField, CacheLineStructLayout, LINUX_SYSCALL_TABLE,
    MIN_CACHE_LINE_BYTES,
};
use cinch::prim_verify::prelude_bridge::extract_source_obligations;
use cinch::prim_verify::sem::{
    eval_bv_crc32c, eval_bv_shuffle16, Obligation, ObligationKind, SemGraph, SemParam, SemTy,
    CRC32C_CASTAGNOLI_POLY,
};
use cinch::prim_verify::smt::emit_smtlib2;
use cinch::prim_verify::translate_val::validate_llvm_memory_equiv;
use cinch::prim_verify::{Policy, TrustTier, VerificationStatus, VerifierEngine};
use std::path::Path;
use std::process::Command;

/// Generate a pure scalar bit-by-bit LLVM IR Castagnoli CRC32C implementation
/// for `data_bits` ∈ `{8, 32, 64}` using only standard LLVM scalar ops (`xor`/`and`/`lshr`/`icmp`/`select`).
fn scalar_crc32c_llvm(data_bits: u16, poly: u32) -> String {
    let reg_w = data_bits.max(32);
    let mut s = String::new();
    if reg_w == 32 {
        if data_bits == 8 {
            s.push_str("  %d_ext = zext i8 %1 to i32\n");
            s.push_str("  %c0 = xor i32 %0, %d_ext\n");
        } else {
            s.push_str("  %c0 = xor i32 %0, %1\n");
        }
        for i in 0..data_bits {
            let next = i + 1;
            s.push_str(&format!(
                "  %b{i} = and i32 %c{i}, 1\n\
                 \x20 %sh{i} = lshr i32 %c{i}, 1\n\
                 \x20 %nz{i} = icmp ne i32 %b{i}, 0\n\
                 \x20 %x{i} = xor i32 %sh{i}, {poly}\n\
                 \x20 %c{next} = select i1 %nz{i}, i32 %x{i}, i32 %sh{i}\n"
            ));
        }
        s.push_str(&format!("  ret i32 %c{data_bits}\n"));
    } else {
        s.push_str("  %a_ext = zext i32 %0 to i64\n");
        s.push_str("  %c0 = xor i64 %a_ext, %1\n");
        for i in 0..data_bits {
            let next = i + 1;
            s.push_str(&format!(
                "  %b{i} = and i64 %c{i}, 1\n\
                 \x20 %sh{i} = lshr i64 %c{i}, 1\n\
                 \x20 %nz{i} = icmp ne i64 %b{i}, 0\n\
                 \x20 %x{i} = xor i64 %sh{i}, {poly}\n\
                 \x20 %c{next} = select i1 %nz{i}, i64 %x{i}, i64 %sh{i}\n"
            ));
        }
        s.push_str(&format!("  %r = trunc i64 %c{data_bits} to i32\n  ret i32 %r\n"));
    }
    s
}

#[test]
fn bv_shuffle16_pshufb_and_tbl1_lrat_kernel_cert() {
    // 1. Concrete evaluator sanity: lanes 0..15 select from table; 16..127 and 128..255 zero out.
    let tbl_bytes: [u8; 16] = [
        0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87,
        0x98, 0xA9, 0xBA, 0xCB, 0xDC, 0xED, 0xFE, 0x0F,
    ];
    let idx_bytes: [u8; 16] = [
        0, 15, 7, 16, 31, 0x7F, 0x80, 0xFF,
        1, 2, 3, 4, 0x10, 0x91, 14, 0,
    ];
    let got = eval_bv_shuffle16(u128::from_le_bytes(tbl_bytes), u128::from_le_bytes(idx_bytes)).to_le_bytes();
    assert_eq!(
        got,
        [
            0x10, 0x0F, 0x87, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x21, 0x32, 0x43, 0x54, 0x00, 0x00, 0xFE, 0x10,
        ]
    );

    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let params = vec![
        SemParam { index: 0, name: "tbl".into(), ty: SemTy::Bv(128), signed: false },
        SemParam { index: 1, name: "idx".into(), ty: SemTy::Bv(128), signed: false },
    ];

    // 2. Intrinsic vs inline-asm equivalence on both x86_64 (pshufb) and aarch64 (tbl1).
    let x86_intr = r#"
      %vt = bitcast i128 %0 to <16 x i8>
      %vi = bitcast i128 %1 to <16 x i8>
      %vr = call <16 x i8> @llvm.x86.ssse3.pshuf.b.128(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
"#;
    let x86_asm = r#"
      %vt = bitcast i128 %0 to <16 x i8>
      %vi = bitcast i128 %1 to <16 x i8>
      %vr = call <16 x i8> asm "pshufb $2, $0", "=x,0,x"(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
"#;
    let arm_intr = r#"
      %vt = bitcast i128 %0 to <16 x i8>
      %vi = bitcast i128 %1 to <16 x i8>
      %vr = call <16 x i8> @llvm.aarch64.neon.tbl1.v16i8(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
"#;
    let arm_asm = r#"
      %vt = bitcast i128 %0 to <16 x i8>
      %vi = bitcast i128 %1 to <16 x i8>
      %vr = call <16 x i8> asm "tbl $0.16b, {$1.16b}, $2.16b", "=w,w,w"(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
"#;

    for (name, lhs, rhs) in [
        ("pshufb-intr-vs-asm", x86_intr, x86_asm),
        ("tbl1-intr-vs-asm", arm_intr, arm_asm),
    ] {
        let ob = validate_llvm_memory_equiv(name, lhs, rhs, &params, SemTy::Bv(128))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        match &engine.verify(&ob).status {
            VerificationStatus::Verified { receipt, .. } => {
                assert_eq!(receipt.tier, TrustTier::KernelCert, "{name} must reach KernelCert");
            }
            other => panic!("{name} must kernel-verify via LRAT, got: {other:?}"),
        }
    }

    // 3. Conditional cross-target equivalence: when every byte of `idx` is in `0..=15` or `0x80..=0xFF`
    //    (i.e. `(idx & 0x7070...70) == 0`), x86 `pshufb` == aarch64 `tbl1` bit-for-bit via LRAT KernelCert!
    let prog_x86 = lower_llvm_fragment(x86_asm, &params, SemTy::Bv(128)).expect("lower pshufb");
    let prog_arm = lower_llvm_fragment(arm_asm, &params, SemTy::Bv(128)).expect("lower tbl1");
    let mut g = SemGraph::new();
    let p_tbl = g.param(0, "tbl", SemTy::Bv(128));
    let p_idx = g.param(1, "idx", SemTy::Bv(128));
    let map_x = g.import_graph(&prog_x86.graph, &[p_tbl, p_idx]);
    let map_a = g.import_graph(&prog_arm.graph, &[p_tbl, p_idx]);
    let (vx, _) = prog_x86.result.as_scalar().unwrap();
    let (va, _) = prog_arm.result.as_scalar().unwrap();
    let mid_mask = g.bv_const(128, 0x7070_7070_7070_7070_7070_7070_7070_7070);
    let zero128 = g.bv_const(128, 0);
    let masked = g.bv_and(p_idx, mid_mask);
    let precond = g.bv_eq(masked, zero128);
    let eq = g.bv_eq(map_x[vx.0 as usize], map_a[va.0 as usize]);
    let goal = g.implies(precond, eq);
    let ob_guarded = Obligation::new(
        "pshufb-eq-tbl1-valid-indices",
        "llvm-sem@21.1",
        ObligationKind::CrossTargetEquiv {
            body_a: "x86_64:pshufb".into(),
            body_b: "aarch64:tbl1".into(),
        },
        params.clone(),
        g,
        goal,
    );
    let smt = emit_smtlib2(&ob_guarded);
    assert!(smt.contains("(set-logic QF_BV)"), "BvShuffle16 SMT must be pure QF_BV");
    assert!(smt.contains("bvuge"), "BvShuffle16 SMT must check idx >= 16");
    match &engine.verify(&ob_guarded).status {
        VerificationStatus::Verified { receipt, .. } => {
            assert_eq!(receipt.tier, TrustTier::KernelCert);
        }
        other => panic!("guarded pshufb == tbl1 must discharge LRAT KernelCert, got: {other:?}"),
    }

    // 4. Unconstrained `pshufb` vs `tbl1` MUST be disproved (disagrees on 0x10..=0x7F)!
    let ob_unguarded =
        validate_llvm_memory_equiv("pshufb-vs-tbl1-unguarded", x86_asm, arm_asm, &params, SemTy::Bv(128))
            .expect("unguarded obligation");
    match &engine.verify(&ob_unguarded).status {
        VerificationStatus::Disproved { counterexample, .. } => {
            assert!(counterexample.validated, "counterexample must be independently validated");
        }
        other => panic!("unguarded pshufb vs tbl1 MUST be disproved (differs on 0x10..0x7F), got: {other:?}"),
    }

    // 5. End-to-end Cinch source `prim simd-u8x16-shuffle` with `justify verify` discharges to KernelCert
    //    (including both per-arch bodies AND the automatic cross-target equivalence obligation!).
    let cinch_src = r#"
module SimdShuffleTest where
  prim simd-u8x16-shuffle :: U128 -> U128 -> U128 where
    llvm {
      %vt = bitcast i128 %0 to <16 x i8>
      %m = and i128 %1, 190825013606643922648088505344089624463
      %vi = bitcast i128 %m to <16 x i8>
      %vr = call <16 x i8> @llvm.x86.ssse3.pshuf.b.128(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
    } when arch x86_64 justify verify
    llvm {
      %vt = bitcast i128 %0 to <16 x i8>
      %m = and i128 %1, 190825013606643922648088505344089624463
      %vi = bitcast i128 %m to <16 x i8>
      %vr = call <16 x i8> @llvm.aarch64.neon.tbl1.v16i8(<16 x i8> %vt, <16 x i8> %vi)
      %r = bitcast <16 x i8> %vr to i128
      ret i128 %r
    } when arch aarch64 justify verify
"#;
    let sems = engine.semantics_refs();
    let extracted = extract_source_obligations(cinch_src, &sems).expect("extract prim");
    // 2 target bodies + 1 automatic cross-target equivalence entry
    assert_eq!(extracted.len(), 3);
    for body in &extracted {
        assert!(body.skip_reason.is_none(), "unexpected skip: {:?}", body.skip_reason);
        assert!(!body.obligations.is_empty());
        for ob in &body.obligations {
            match &engine.verify(ob).status {
                VerificationStatus::Verified { receipt, .. } => {
                    assert_eq!(receipt.tier, TrustTier::KernelCert);
                }
                other => panic!("prim simd-u8x16-shuffle must verify to KernelCert, got: {other:?}"),
            }
        }
    }
}

#[test]
fn bv_crc32c_hardware_equals_scalar_lrat_kernel_cert() {
    // 1. Concrete RFC 3720 / iSCSI check vector: CRC32C("123456789") == 0xE3069283.
    let msg = b"123456789";
    let mut crc_u8 = 0xFFFF_FFFFu32;
    for &b in msg {
        crc_u8 = eval_bv_crc32c(crc_u8, b as u64, 8);
    }
    assert_eq!(crc_u8 ^ 0xFFFF_FFFF, 0xE306_9283, "u8 Castagnoli RFC 3720 vector");

    let w0 = u32::from_le_bytes([msg[0], msg[1], msg[2], msg[3]]);
    let w1 = u32::from_le_bytes([msg[4], msg[5], msg[6], msg[7]]);
    let crc_u32 = eval_bv_crc32c(
        eval_bv_crc32c(eval_bv_crc32c(0xFFFF_FFFF, w0 as u64, 32), w1 as u64, 32),
        msg[8] as u64,
        8,
    );
    assert_eq!(crc_u32 ^ 0xFFFF_FFFF, 0xE306_9283, "u32 Castagnoli RFC 3720 vector");

    let q0 = u64::from_le_bytes([msg[0], msg[1], msg[2], msg[3], msg[4], msg[5], msg[6], msg[7]]);
    let crc_u64 = eval_bv_crc32c(eval_bv_crc32c(0xFFFF_FFFF, q0, 64), msg[8] as u64, 8);
    assert_eq!(crc_u64 ^ 0xFFFF_FFFF, 0xE306_9283, "u64 Castagnoli RFC 3720 vector");

    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());

    // 2. 8-bit CRC32C: x86_64 crc32b (asm & intrinsic) == aarch64 crc32cb (asm & intrinsic) == scalar reference.
    let p8 = vec![
        SemParam { index: 0, name: "acc".into(), ty: SemTy::Bv(32), signed: false },
        SemParam { index: 1, name: "b".into(), ty: SemTy::Bv(8), signed: false },
    ];
    let scalar_u8 = scalar_crc32c_llvm(8, CRC32C_CASTAGNOLI_POLY);
    let x86_u8_asm = "%r = call i32 asm \"crc32b $2, $0\", \"=r,0,q\"(i32 %0, i8 %1)\nret i32 %r\n";
    let x86_u8_intr = "%r = call i32 @llvm.x86.sse42.crc32.32.8(i32 %0, i8 %1)\nret i32 %r\n";
    let arm_u8_asm = "%d = zext i8 %1 to i32\n%r = call i32 asm \".arch_extension crc\\0Acrc32cb ${0:w}, ${1:w}, ${2:w}\", \"=r,r,r\"(i32 %0, i32 %d)\nret i32 %r\n";
    let arm_u8_intr = "%d = zext i8 %1 to i32\n%r = call i32 @llvm.aarch64.crc32cb(i32 %0, i32 %d)\nret i32 %r\n";

    for (name, hw) in [
        ("crc32b-asm-vs-scalar", x86_u8_asm),
        ("crc32b-intr-vs-scalar", x86_u8_intr),
        ("crc32cb-asm-vs-scalar", arm_u8_asm),
        ("crc32cb-intr-vs-scalar", arm_u8_intr),
    ] {
        let ob = validate_llvm_memory_equiv(name, hw, &scalar_u8, &p8, SemTy::Bv(32))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        match &engine.verify(&ob).status {
            VerificationStatus::Verified { receipt, .. } => assert_eq!(receipt.tier, TrustTier::KernelCert),
            other => panic!("{name} must discharge LRAT KernelCert, got: {other:?}"),
        }
    }

    // 3. 32-bit CRC32C: x86_64 crc32l == aarch64 crc32cw == scalar reference.
    let p32 = vec![
        SemParam { index: 0, name: "acc".into(), ty: SemTy::Bv(32), signed: false },
        SemParam { index: 1, name: "w".into(), ty: SemTy::Bv(32), signed: false },
    ];
    let scalar_u32 = scalar_crc32c_llvm(32, CRC32C_CASTAGNOLI_POLY);
    let x86_u32_asm = "%r = call i32 asm \"crc32l $2, $0\", \"=r,0,r\"(i32 %0, i32 %1)\nret i32 %r\n";
    let x86_u32_intr = "%r = call i32 @llvm.x86.sse42.crc32.32.32(i32 %0, i32 %1)\nret i32 %r\n";
    let arm_u32_asm = "%r = call i32 asm \".arch_extension crc\\0Acrc32cw ${0:w}, ${1:w}, ${2:w}\", \"=r,r,r\"(i32 %0, i32 %1)\nret i32 %r\n";
    let arm_u32_intr = "%r = call i32 @llvm.aarch64.crc32cw(i32 %0, i32 %1)\nret i32 %r\n";

    for (name, hw) in [
        ("crc32l-asm-vs-scalar", x86_u32_asm),
        ("crc32l-intr-vs-scalar", x86_u32_intr),
        ("crc32cw-asm-vs-scalar", arm_u32_asm),
        ("crc32cw-intr-vs-scalar", arm_u32_intr),
    ] {
        let ob = validate_llvm_memory_equiv(name, hw, &scalar_u32, &p32, SemTy::Bv(32))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        match &engine.verify(&ob).status {
            VerificationStatus::Verified { receipt, .. } => assert_eq!(receipt.tier, TrustTier::KernelCert),
            other => panic!("{name} must discharge LRAT KernelCert, got: {other:?}"),
        }
    }

    // 4. 64-bit CRC32C: x86_64 crc32q == aarch64 crc32cx == scalar reference.
    let p64 = vec![
        SemParam { index: 0, name: "acc".into(), ty: SemTy::Bv(32), signed: false },
        SemParam { index: 1, name: "q".into(), ty: SemTy::Bv(64), signed: false },
    ];
    let scalar_u64 = scalar_crc32c_llvm(64, CRC32C_CASTAGNOLI_POLY);
    let x86_u64_asm = "%a64 = zext i32 %0 to i64\n%r64 = call i64 asm \"crc32q $2, $0\", \"=r,0,r\"(i64 %a64, i64 %1)\n%r = trunc i64 %r64 to i32\nret i32 %r\n";
    let x86_u64_intr = "%a64 = zext i32 %0 to i64\n%r64 = call i64 @llvm.x86.sse42.crc32.64.64(i64 %a64, i64 %1)\n%r = trunc i64 %r64 to i32\nret i32 %r\n";
    let arm_u64_asm = "%r = call i32 asm \".arch_extension crc\\0Acrc32cx ${0:w}, ${1:w}, $2\", \"=r,r,r\"(i32 %0, i64 %1)\nret i32 %r\n";
    let arm_u64_intr = "%r = call i32 @llvm.aarch64.crc32cx(i32 %0, i64 %1)\nret i32 %r\n";

    for (name, hw) in [
        ("crc32q-asm-vs-scalar", x86_u64_asm),
        ("crc32q-intr-vs-scalar", x86_u64_intr),
        ("crc32cx-asm-vs-scalar", arm_u64_asm),
        ("crc32cx-intr-vs-scalar", arm_u64_intr),
    ] {
        let ob = validate_llvm_memory_equiv(name, hw, &scalar_u64, &p64, SemTy::Bv(32))
            .unwrap_or_else(|e| panic!("{name}: {e}"));
        let smt = emit_smtlib2(&ob);
        assert!(smt.contains("(set-logic QF_BV)"));
        assert!(smt.contains("82f63b78"));
        match &engine.verify(&ob).status {
            VerificationStatus::Verified { receipt, .. } => assert_eq!(receipt.tier, TrustTier::KernelCert),
            other => panic!("{name} must discharge LRAT KernelCert, got: {other:?}"),
        }
    }

    // 5. Wrong polynomial (IEEE 802.3 0xEDB88320 vs Castagnoli 0x82F63B78) MUST be disproved!
    let wrong_poly_u8 = scalar_crc32c_llvm(8, 0xEDB8_8320);
    let ob_wrong = validate_llvm_memory_equiv("crc32c-wrong-poly", x86_u8_asm, &wrong_poly_u8, &p8, SemTy::Bv(32))
        .expect("wrong-poly obligation");
    match &engine.verify(&ob_wrong).status {
        VerificationStatus::Disproved { counterexample, .. } => {
            assert!(counterexample.validated);
        }
        other => panic!("IEEE polynomial vs Castagnoli hardware CRC32C MUST be disproved, got: {other:?}"),
    }

    // 6. Cinch source `prim hw-crc32c-u8/u32/u64` with `justify verify` discharges to KernelCert!
    let cinch_src = r#"
module HwCrc32cTest where
  prim hw-crc32c-u8 :: U32 -> U8 -> U32 where
    llvm {
      %r = call i32 @llvm.x86.sse42.crc32.32.8(i32 %0, i8 %1)
      ret i32 %r
    } when arch x86_64 justify verify
    llvm {
      %d = zext i8 %1 to i32
      %r = call i32 @llvm.aarch64.crc32cb(i32 %0, i32 %d)
      ret i32 %r
    } when arch aarch64 justify verify

  prim hw-crc32c-u32 :: U32 -> U32 -> U32 where
    llvm {
      %r = call i32 @llvm.x86.sse42.crc32.32.32(i32 %0, i32 %1)
      ret i32 %r
    } when arch x86_64 justify verify
    llvm {
      %r = call i32 @llvm.aarch64.crc32cw(i32 %0, i32 %1)
      ret i32 %r
    } when arch aarch64 justify verify

  prim hw-crc32c-u64 :: U32 -> U64 -> U32 where
    llvm {
      %a64 = zext i32 %0 to i64
      %r64 = call i64 @llvm.x86.sse42.crc32.64.64(i64 %a64, i64 %1)
      %r = trunc i64 %r64 to i32
      ret i32 %r
    } when arch x86_64 justify verify
    llvm {
      %r = call i32 @llvm.aarch64.crc32cx(i32 %0, i64 %1)
      ret i32 %r
    } when arch aarch64 justify verify
"#;
    let sems = engine.semantics_refs();
    let extracted = extract_source_obligations(cinch_src, &sems).expect("extract crc32c prims");
    // 6 target bodies + 3 automatic cross-target equivalence entries
    assert_eq!(extracted.len(), 9);
    for body in &extracted {
        assert!(body.skip_reason.is_none(), "unexpected skip: {:?}", body.skip_reason);
        for ob in &body.obligations {
            match &engine.verify(ob).status {
                VerificationStatus::Verified { receipt, .. } => {
                    assert_eq!(receipt.tier, TrustTier::KernelCert);
                }
                other => panic!("hw-crc32c prim must verify to KernelCert, got: {other:?}"),
            }
        }
    }
}

#[test]
fn mem_contract_v2_syscalls_atomics_and_cache_line_layout() {
    // 1. Pinned Linux syscall table completeness & pair checks.
    assert!(LINUX_SYSCALL_TABLE.len() >= 40);
    assert_eq!(lookup_linux_syscall("linux-io-uring-setup"), Some(("io_uring_setup", 425, 425)));
    assert_eq!(lookup_linux_syscall("linux-socket"), Some(("socket", 41, 198)));
    assert_eq!(lookup_linux_syscall("linux-epoll-pwait"), Some(("epoll_pwait", 281, 22)));
    assert_eq!(validate_linux_syscall_pair(425, 425), Ok("io_uring_setup"));
    assert_eq!(validate_linux_syscall_pair(41, 198), Ok("socket"));
    assert_eq!(validate_linux_syscall_pair(9, 222), Ok("mmap"));
    assert!(validate_linux_syscall_pair(41, 41).is_err());
    assert!(validate_linux_syscall_pair(425, 426).is_err());

    // 2. Valid dual-arch high-throughput Linux syscalls pass with 0 violations.
    let valid_src = r#"
module GoodLinuxSyscalls where
  prim linux-io-uring-setup :: I64 -> I64 ->{IO} I64 where
    unsafe llvm {
      %r = call i64 asm sideeffect "mov x0, $1\0A mov x1, $2\0A mov x8, #425\0A svc #0\0A mov $0, x0", "={x0},{x1},{x2},~{x8},~{memory}"(i64 %0, i64 %1)
      ret i64 %r
    } when arch aarch64
    unsafe llvm {
      %r = call i64 asm sideeffect inteldialect "mov rax, 425\0A mov rdi, $1\0A mov rsi, $2\0A syscall\0A mov $0, rax", "={rax},{rdi},{rsi},~{rcx},~{r11},~{memory}"(i64 %0, i64 %1)
      ret i64 %r
    } when arch x86_64

  prim linux-socket :: I64 -> I64 -> I64 ->{IO} I64 where
    unsafe llvm {
      %r = call i64 asm sideeffect "mov x0, $1\0A mov x1, $2\0A mov x2, $3\0A mov x8, #198\0A svc #0\0A mov $0, x0", "={x0},{x1},{x2},{x3},~{x8},~{memory}"(i64 %0, i64 %1, i64 %2)
      ret i64 %r
    } when arch aarch64
    unsafe llvm {
      %r = call i64 asm sideeffect inteldialect "mov rax, 41\0A mov rdi, $1\0A mov rsi, $2\0A mov rdx, $3\0A syscall\0A mov $0, rax", "={rax},{rdi},{rsi},{rdx},~{rcx},~{r11},~{memory}"(i64 %0, i64 %1, i64 %2)
      ret i64 %r
    } when arch x86_64
"#;
    let good_rep = audit_source_contracts(valid_src).expect("audit valid syscalls");
    assert!(good_rep.violations.is_empty(), "expected 0 violations, got: {:?}", good_rep.violations);
    assert_eq!(good_rep.linux_syscalls_checked, 4);

    // 3. Mismatched syscall numbers (x86_64 vs aarch64) and bad atomic orderings MUST be caught.
    let bad_src = r#"
module BadV2Contracts where
  prim linux-socket :: I64 -> I64 -> I64 ->{IO} I64 where
    unsafe llvm {
      %r = call i64 asm sideeffect "mov x8, #41\0A svc #0\0A mov $0, x0", "={x0},r,r,r,~{x8},~{memory}"(i64 %0, i64 %1, i64 %2)
      ret i64 %r
    } when arch aarch64
    unsafe llvm {
      %r = call i64 asm sideeffect inteldialect "mov rax, 41\0A syscall\0A mov $0, rax", "={rax},r,r,r,~{rcx},~{r11},~{memory}"(i64 %0, i64 %1, i64 %2)
      ret i64 %r
    } when arch x86_64

  prim custom-mismatched-pair :: I64 ->{IO} I64 where
    unsafe llvm {
      %r = call i64 asm sideeffect "mov x8, #215\0A svc #0\0A mov $0, x0", "={x0},r,~{x8},~{memory}"(i64 %0)
      ret i64 %r
    } when arch aarch64
    unsafe llvm {
      %r = call i64 asm sideeffect inteldialect "mov rax, 9\0A syscall\0A mov $0, rax", "={rax},r,~{rcx},~{r11},~{memory}"(i64 %0)
      ret i64 %r
    } when arch x86_64

  prim bad-atomics :: U64 ->{IO} U64 where
    unsafe llvm {
      %p = inttoptr i64 %0 to ptr
      %a = load atomic i64, ptr %p release, align 8
      store atomic i64 %a, ptr %p acquire, align 8
      %c = cmpxchg ptr %p, i64 %a, i64 1 monotonic seq_cst, align 8
      fence monotonic
      ret i64 %a
    }
"#;
    let bad_rep = audit_source_contracts(bad_src).expect("audit bad contracts");
    let rules: Vec<&str> = bad_rep.violations.iter().map(|v| v.rule.as_str()).collect();
    assert!(rules.contains(&"linux-syscall-number"), "must catch wrong pinned aarch64 socket nr: {rules:?}");
    assert!(rules.contains(&"linux-syscall-mismatch"), "must catch cross-arch mmap/munmap mismatch: {rules:?}");
    assert!(
        bad_rep.violations.iter().filter(|v| v.rule == "atomic-ordering").count() >= 4,
        "must catch all 4 bad atomic orderings: {:?}",
        bad_rep.violations
    );

    // 4. Cache-line layout audit: isolated 128B ring passes; 64B false-sharing fails.
    let good_ring = CacheLineStructLayout {
        name: "SpscRingState".into(),
        repr: "C".into(),
        size: 256,
        align: 128,
        fields: vec![
            CacheLineField { name: "head".into(), offset: 0, size: 8, align: 8, is_atomic_cursor: true },
            CacheLineField { name: "_pad0".into(), offset: 8, size: 120, align: 1, is_atomic_cursor: false },
            CacheLineField { name: "tail".into(), offset: 128, size: 8, align: 8, is_atomic_cursor: true },
            CacheLineField { name: "_pad1".into(), offset: 136, size: 120, align: 1, is_atomic_cursor: false },
        ],
    };
    let cl_ok = audit_cache_line_layout(&good_ring, MIN_CACHE_LINE_BYTES);
    assert!(cl_ok.violations.is_empty(), "128B-padded ring must pass: {:?}", cl_ok.violations);
    assert_eq!(cl_ok.cursor_pairs_checked, 1);

    let bad_ring = CacheLineStructLayout {
        name: "FalseSharingRing".into(),
        repr: "C".into(),
        size: 136,
        align: 64,
        fields: vec![
            CacheLineField { name: "head".into(), offset: 0, size: 8, align: 8, is_atomic_cursor: true },
            CacheLineField { name: "tail".into(), offset: 64, size: 8, align: 8, is_atomic_cursor: true },
        ],
    };
    let cl_bad = audit_cache_line_layout(&bad_ring, MIN_CACHE_LINE_BYTES);
    let cl_rules: Vec<&str> = cl_bad.violations.iter().map(|v| v.rule.as_str()).collect();
    assert!(cl_rules.contains(&"cache-line-align"), "expected align violation: {cl_rules:?}");
    assert!(cl_rules.contains(&"cache-line-stride"), "expected stride violation: {cl_rules:?}");
    assert!(cl_rules.contains(&"cache-line-spacing"), "expected spacing violation: {cl_rules:?}");
}

#[test]
fn verification_tooling_allowlist_and_kernel_abi_smoke() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let allowlist_script = repo_root.join("tooling/verification/check_tcb_allowlist.py");
    let abi_script = repo_root.join("tooling/verification/verify_kernel_abi.py");
    let tmp = std::env::temp_dir().join(format!("cinch_w0a_tooling_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();

    // 1. TCB allowlist: allowlisted Prim.Sys.Linux passes; rogue Assumed prim fails with exit code 1.
    let good_manifest = tmp.join("Good.manifest.json");
    std::fs::write(
        &good_manifest,
        r#"{
          "entry": "Good",
          "prims": [
            {"module": "Prim.Sys.Linux", "name": "linux-io-uring-setup", "tier": "assumed", "receipts": [{"tier": "assumed"}]},
            {"module": "Simd.Shuffle", "name": "simd-u8x16-shuffle", "tier": "verified", "receipts": [{"tier": "kernel-cert"}]}
          ]
        }"#,
    )
    .unwrap();
    let out_good = Command::new("python3")
        .arg(&allowlist_script)
        .arg("--strict-jpl")
        .arg(&good_manifest)
        .output()
        .expect("run check_tcb_allowlist.py");
    assert!(
        out_good.status.success(),
        "allowlisted manifest must pass: {}",
        String::from_utf8_lossy(&out_good.stderr)
    );

    let bad_manifest = tmp.join("Bad.manifest.json");
    std::fs::write(
        &bad_manifest,
        r#"{
          "entry": "Bad",
          "prims": [
            {"module": "Rogue.Mod", "name": "unverified-backdoor", "tier": "assumed", "receipts": [{"tier": "assumed"}]}
          ]
        }"#,
    )
    .unwrap();
    let out_bad = Command::new("python3")
        .arg(&allowlist_script)
        .arg(&bad_manifest)
        .output()
        .expect("run check_tcb_allowlist.py on bad manifest");
    assert_eq!(out_bad.status.code(), Some(1), "unallowlisted Assumed prim MUST fail with code 1");

    // 2. Kernel ABI checker: verify `struct timespec` and `struct iovec` against host `clang`.
    let good_cinchif = tmp.join("HostAbi.cinchif");
    std::fs::write(
        &good_cinchif,
        r#"{
          "abi": {
            "convention": "cinch-native-v1",
            "symbols": [],
            "layouts": [
              {
                "type": "Timespec",
                "repr": "C",
                "size": 16,
                "align": 8,
                "fields": [
                  {"name": "tv_sec", "offset": 0, "size": 8, "align": 8},
                  {"name": "tv_nsec", "offset": 8, "size": 8, "align": 8}
                ]
              },
              {
                "type": "Iovec",
                "repr": "C",
                "size": 16,
                "align": 8,
                "fields": [
                  {"name": "iov_base", "offset": 0, "size": 8, "align": 8},
                  {"name": "iov_len", "offset": 8, "size": 8, "align": 8}
                ]
              }
            ]
          }
        }"#,
    )
    .unwrap();
    let out_abi_ok = Command::new("python3")
        .arg(&abi_script)
        .arg(&good_cinchif)
        .output()
        .expect("run verify_kernel_abi.py");
    assert!(
        out_abi_ok.status.success(),
        "valid host Timespec/Iovec ABI must pass: stdout={} stderr={}",
        String::from_utf8_lossy(&out_abi_ok.stdout),
        String::from_utf8_lossy(&out_abi_ok.stderr)
    );

    let bad_cinchif = tmp.join("BadAbi.cinchif");
    std::fs::write(
        &bad_cinchif,
        r#"{
          "abi": {
            "convention": "cinch-native-v1",
            "symbols": [],
            "layouts": [
              {
                "type": "Timespec",
                "repr": "C",
                "size": 16,
                "align": 8,
                "fields": [
                  {"name": "tv_sec", "offset": 0, "size": 8, "align": 8},
                  {"name": "tv_nsec", "offset": 12, "size": 8, "align": 8}
                ]
              }
            ]
          }
        }"#,
    )
    .unwrap();
    let out_abi_bad = Command::new("python3")
        .arg(&abi_script)
        .arg(&bad_cinchif)
        .output()
        .expect("run verify_kernel_abi.py on tampered ABI");
    assert_eq!(out_abi_bad.status.code(), Some(1), "tampered Timespec offset MUST fail with code 1");

    let _ = std::fs::remove_dir_all(&tmp);
}
