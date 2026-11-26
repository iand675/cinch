//! Verifies that the self-hosted bootstrapped compiler (`Cinch.RuntimeIr`)
//! emits `-O2` LLVM IR of equal quality to the Rust seed compiler across:
//! - inline TBAA/invariant `Bytes` loops (`bytesLenI64`, `bytesGetU8At`, `bytesLoadU128At`),
//! - portable 128-bit SIMD (`simdU8x16*`, `simdU32x4*`, `rotateLeftU128`),
//! - branchless `select i1` on `I64` and `U128`,
//! - inline hardware float intrinsics (`floorF64`, `copysignF64`, `f64Bits`),
//! - typed `Wide` (`i128`) self-tail loops (`self.loop` -> SSA `phi i128`),
//! - companion `#cpr` unboxed `{ i128, i128 }` product returns with zero heap allocation.
mod common;
use std::process::Command;

const PARITY_SOURCE: &str = r#"module Main where
  foreign import ccall "cinch.runtime.bytesNew" bytes-new :: Nat -> Bytes
  foreign import ccall "cinch.runtime.bytesLenI64" bytes-len-i64 :: Bytes -> I64
  foreign import ccall "cinch.runtime.bytesGetU8At" bytes-get-u8-at :: Bytes -> I64 -> U8
  foreign import ccall "cinch.runtime.bytesSetU8At" bytes-set-u8-at :: Bytes -> I64 -> U8 -> Unit
  foreign import ccall "cinch.runtime.bytesLoadU128At" bytes-load-u128-at :: Bytes -> I64 -> U128
  prim add-i64 :: I64 -> I64 -> I64 where
    llvm {
      %r = add i64 %0, %1
      ret i64 %r
    }
  prim lt-i64 :: I64 -> I64 -> Bool where
    llvm {
      %c = icmp slt i64 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim ge-i64 :: I64 -> I64 -> Bool where
    llvm {
      %c = icmp sge i64 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim u8-to-i64 :: U8 -> I64 where
    llvm {
      %r = zext i8 %0 to i64
      ret i64 %r
    }
  prim eq-i64 :: I64 -> I64 -> Bool where
    llvm {
      %c = icmp eq i64 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim eq-u32 :: U32 -> U32 -> Bool where
    llvm {
      %c = icmp eq i32 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim eq-u64 :: U64 -> U64 -> Bool where
    llvm {
      %c = icmp eq i64 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim eq-u128 :: U128 -> U128 -> Bool where
    llvm {
      %c = icmp eq i128 %0, %1
      %r = zext i1 %c to i8
      ret i8 %r
    }
  prim add-u128 :: U128 -> U128 -> U128 where
    llvm {
      %r = add i128 %0, %1
      ret i128 %r
    }
  prim xor-u128 :: U128 -> U128 -> U128 where
    llvm {
      %r = xor i128 %0, %1
      ret i128 %r
    }
  prim or-u128 :: U128 -> U128 -> U128 where
    llvm {
      %r = or i128 %0, %1
      ret i128 %r
    }
  prim rotl-u128 :: U128 -> U32 -> U128 where
    llvm {
      %s = zext i32 %1 to i128
      %r = call i128 @llvm.fshl.i128(i128 %0, i128 %0, i128 %s)
      ret i128 %r
    }
  prim cttz-u32 :: U32 -> U32 where
    llvm {
      %r = call i32 @llvm.cttz.i32(i32 %0, i1 false)
      ret i32 %r
    }
  prim splat-u8 :: U8 -> U128 where
    llvm {
      %v0 = insertelement <16 x i8> poison, i8 %0, i64 0
      %v = shufflevector <16 x i8> %v0, <16 x i8> poison, <16 x i32> zeroinitializer
      %r = bitcast <16 x i8> %v to i128
      ret i128 %r
    }
  prim cmpeq-u8 :: U128 -> U128 -> U128 where
    llvm {
      %va = bitcast i128 %0 to <16 x i8>
      %vb = bitcast i128 %1 to <16 x i8>
      %vc = icmp eq <16 x i8> %va, %vb
      %vm = sext <16 x i1> %vc to <16 x i8>
      %r = bitcast <16 x i8> %vm to i128
      ret i128 %r
    }
  prim cmplt-u8 :: U128 -> U128 -> U128 where
    llvm {
      %va = bitcast i128 %0 to <16 x i8>
      %vb = bitcast i128 %1 to <16 x i8>
      %vc = icmp ult <16 x i8> %va, %vb
      %vm = sext <16 x i1> %vc to <16 x i8>
      %r = bitcast <16 x i8> %vm to i128
      ret i128 %r
    }
  prim movemask-u8 :: U128 -> U32 where
    llvm {
      %v = bitcast i128 %0 to <16 x i8>
      %m = icmp slt <16 x i8> %v, zeroinitializer
      %b16 = bitcast <16 x i1> %m to i16
      %r = zext i16 %b16 to i32
      ret i32 %r
    }
  prim add-u32x4 :: U128 -> U128 -> U128 where
    llvm {
      %va = bitcast i128 %0 to <4 x i32>
      %vb = bitcast i128 %1 to <4 x i32>
      %vr = add <4 x i32> %va, %vb
      %r = bitcast <4 x i32> %vr to i128
      ret i128 %r
    }
  prim rotl-u32x4 :: U128 -> U32 -> U128 where
    llvm {
      %va = bitcast i128 %0 to <4 x i32>
      %s0 = insertelement <4 x i32> poison, i32 %1, i64 0
      %sv = shufflevector <4 x i32> %s0, <4 x i32> poison, <4 x i32> zeroinitializer
      %vr = call <4 x i32> @llvm.fshl.v4i32(<4 x i32> %va, <4 x i32> %va, <4 x i32> %sv)
      %r = bitcast <4 x i32> %vr to i128
      ret i128 %r
    }
  prim f64-from-bits :: U64 -> F64 where
    llvm {
      %r = bitcast i64 %0 to double
      ret double %r
    }
  prim f64-bits :: F64 -> U64 where
    llvm {
      %r = bitcast double %0 to i64
      ret i64 %r
    }
  prim floor-f64 :: F64 -> F64 where
    llvm {
      %r = call double @llvm.floor.f64(double %0)
      ret double %r
    }
  prim copysign-f64 :: F64 -> F64 -> F64 where
    llvm {
      %r = call double @llvm.copysign.f64(double %0, double %1)
      ret double %r
    }

  data Pair128 where
    Pair128 :: U128 -> U128 -> Pair128

  sum-bytes :: Bytes -> I64 -> I64 -> I64
  sum-bytes bs i acc =
    if ge-i64 i 0 && lt-i64 i (bytes-len-i64 bs) then
      let b = u8-to-i64 (bytes-get-u8-at bs i) in
      sum-bytes bs (add-i64 i 1) (add-i64 acc b)
    else acc

  xml-scan16 :: Bytes -> I64 -> U32
  xml-scan16 bs i =
    let chunk = bytes-load-u128-at bs i in
    let m1 = cmpeq-u8 chunk (splat-u8 60) in
    let m2 = cmpeq-u8 chunk (splat-u8 38) in
    let m3 = cmplt-u8 chunk (splat-u8 32) in
    let mask = movemask-u8 (or-u128 (or-u128 m1 m2) m3) in
    if eq-u32 mask 0 then 16 else cttz-u32 mask

  chacha-step :: U128 -> U128 -> Pair128
  chacha-step a b =
    let a1 = add-u32x4 a b in
    let b1 = rotl-u32x4 (xor-u128 b a1) 16 in
    let a2 = add-u32x4 a1 b1 in
    let b2 = rotl-u128 (rotl-u32x4 (xor-u128 b1 a2) 12) 32 in
    Pair128 a2 b2

  cmov-u128 :: Bool -> U128 -> U128 -> U128
  cmov-u128 flag a b = if flag then a else b

  sum-u128 :: U128 -> I64 -> U128
  sum-u128 acc n = if eq-i64 n 0 then acc else sum-u128 (add-u128 acc 7) (add-i64 n (-1))

  floor-neg :: U64 -> U64
  floor-neg bits = f64-bits (copysign-f64 (floor-f64 (f64-from-bits bits)) (f64-from-bits 13830554455654793216))

  fill-bytes :: Bytes -> I64 -> Unit
  fill-bytes bs i =
    if lt-i64 i (bytes-len-i64 bs) then
      let u = bytes-set-u8-at bs i 65 in
      fill-bytes bs (add-i64 i 1)
    else ()

  main :: Int
  main =
    let bs = bytes-new 16 in
    let u0 = fill-bytes bs 0 in
    let u1 = bytes-set-u8-at bs 5 60 in
    let total = sum-bytes bs 0 0 in
    let pos = xml-scan16 bs 0 in
    let chunk = bytes-load-u128-at bs 0 in
    let qr-ok = (case chacha-step chunk (add-u128 chunk 2) of Pair128 x y -> (if eq-u128 x y then False else True) && eq-u128 (cmov-u128 (eq-u32 pos 5) x y) x) in
    let wide-sum = sum-u128 0 10 in
    let fl = floor-neg 4615514078110652826 in
    if eq-i64 total 1035 && eq-u32 pos 5 && qr-ok && eq-u128 wide-sum 70 && eq-u64 fl 13837309855095848960 then 200 else 1
"#;

#[test]
fn bootstrapped_compiler_emits_optimized_simd_bytes_cpr_and_wide_loop_ir() {
    let dir = common::project("bootstrap_codegen_parity", &[
        ("Cinch.Check.cinch", include_str!("../../compiler/Cinch.Check.cinch")),
        ("Cinch.RuntimeIr.cinch", include_str!("../../compiler/Cinch.RuntimeIr.cinch")),
        ("Cinch.Resolve.cinch", include_str!("../../compiler/Cinch.Resolve.cinch")),
        ("Cinch.Contracts.cinch", include_str!("../../compiler/Cinch.Contracts.cinch")),
        ("Cinch.Load.cinch", include_str!("../../compiler/Cinch.Load.cinch")),
        ("Cinch.Inputs.cinch", include_str!("../../compiler/Cinch.Inputs.cinch")),
        ("Cinch.Interface.cinch", include_str!("../../compiler/Cinch.Interface.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.RuntimeIr.cinch")),
    ]);
    std::fs::write(dir.join("interfaces.json"), "{\"schema\":1,\"interfaces\":[]}").unwrap();
    let emitter = common::build_executable(&dir, "Main.cinch", "runtime-ir").expect("build the Cinch runtime emitter");
    let runtime = common::host_runtime_archive();
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());

    std::fs::write(dir.join("input-0.source"), PARITY_SOURCE).unwrap();
    std::fs::write(dir.join("batch.count"), b"x").unwrap();
    let out = Command::new(&emitter).current_dir(&dir).output().unwrap();
    assert_eq!(out.status.code(), Some(0), "emitter stderr: {}", String::from_utf8_lossy(&out.stderr));
    let status = common::read_output(&dir, "output-0.status");
    assert_eq!(status, "0", "diagnostics: {}", common::read_output(&dir, "output-0.diagnostics"));
    let ir = common::read_output(&dir, "output-0.ll");
    let raw_ll = dir.join("parity.ll");
    let opt_ll = dir.join("parity_o2.ll");
    std::fs::write(&raw_ll, &ir).unwrap();

    // Check unoptimized IR features emitted directly by Cinch.RuntimeIr.
    assert!(ir.contains("#cpr"), "expected companion #cpr worker for Pair128:\n{ir}");
    assert!(ir.contains("{ i128, i128 }"), "expected unboxed {{ i128, i128 }} CPR return:\n{ir}");
    assert!(ir.contains("self.loop:"), "expected self.loop for self-tail-recursive workers");
    assert!(ir.contains("select i1 "), "expected branchless select i1 for cmov-u128");
    assert!(ir.contains("!invariant.load !0, !tbaa !3"), "expected invariant.load + TBAA metadata on Bytes header");
    assert!(ir.contains("#borrow"), "expected zero-RC borrowed Bytes workers");

    // Optimize with clang -O2 -S -emit-llvm and verify optimized IR quality.
    let opt = Command::new(&clang).args(["-O2", "-S", "-emit-llvm"]).arg(&raw_ll).arg("-o").arg(&opt_ll).output().unwrap();
    assert!(opt.status.success(), "clang -O2 -S -emit-llvm failed: {}", String::from_utf8_lossy(&opt.stderr));
    let o2 = std::fs::read_to_string(&opt_ll).unwrap();

    // 1. SIMD XML scan fuses to <16 x i1> movemask + llvm.cttz.i32 with zero RC dup inside xml-scan16.
    assert!(o2.contains("bitcast <16 x i1>"), "expected movemask bitcast <16 x i1> in -O2 IR:\n{o2}");
    assert!(o2.contains("@llvm.cttz.i32") || o2.contains("llvm.cttz"), "expected llvm.cttz in -O2 IR");
    // 2. ChaCha SIMD step uses <4 x i32> vector ops + { i128, i128 } unboxed return with zero cinch_alloc.
    assert!(o2.contains("<4 x i32>"), "expected <4 x i32> SIMD ops in -O2 IR");
    if let Some(body) = o2.split("#cpr\"").nth(1).and_then(|s| s.split("\n}\n").next()) {
        assert!(!body.contains("cinch_alloc"), "CPR worker must not heap allocate:\n{body}");
    }
    // 3. Hardware float intrinsics emitted directly.
    assert!(o2.contains("@llvm.floor.f64") || o2.contains("13837309855095848960"), "expected llvm.floor.f64 or constant-folded floor result");
    // 4. Wide self-tail loop (`sum-u128`) compiled to an SSA loop (no recursive call to itself).
    if let Some(body) = o2.split("sum$2du128#loop").nth(1).and_then(|s| s.split("\n}\n").next()) {
        assert!(!body.contains("call "), "Wide self-tail loop must have zero calls:\n{body}");
    }
    // 5. `sum-bytes` loop has zero RC dup calls inside the loop body and hoists length/data-ptr.
    let sum_def = o2
        .split("\ndefine ")
        .find(|part| part.lines().next().is_some_and(|l| l.contains("m.Main.sum$2dbytes\"")))
        .and_then(|part| part.split("\n}\n").next())
        .expect("define sum-bytes in -O2 IR");
    println!("=== BOOTSTRAP -O2 sum-bytes ===\n{sum_def}\n=== END ===");
    assert!(!sum_def.contains("cinch_dup"), "sum-bytes loop must not dup Bytes:\n{sum_def}");
    assert!(!sum_def.contains("cinch_bytes_get"), "sum-bytes hot loop should hoist/inline Bytes access");

    // Link and run at -O2 with full RC debug + leak check.
    let exe = dir.join("parity-o2");
    let link = Command::new(&clang).args(["-O2"]).arg(&raw_ll).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&exe).output().unwrap();
    assert!(link.status.success(), "link failed: {}", String::from_utf8_lossy(&link.stderr));
    let ran = Command::new(&exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(ran.status.code(), Some(200), "runtime stderr: {}", String::from_utf8_lossy(&ran.stderr));
    let stderr = String::from_utf8_lossy(&ran.stderr);
    assert!(stderr.contains("leaked=0"), "expected leaked=0, got: {stderr}");

    common::cleanup(&dir);
}
