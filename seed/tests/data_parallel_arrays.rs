//! End-to-end integration tests for Cinch's data-parallel array system,
//! Typed Array IR fusion, shape-indexed views, callback forwarding (`Kernel`),
//! `.cinchif` kernel metadata, and Apple Metal GPU resident pipelines.

mod common;

use std::process::Command;

const SOURCES: &[(&str, &str)] = &[
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Array.Packed.cinch", include_str!("../prelude/Data/Array/Packed.cinch")),
    ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
    ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
    ("Data.Array.Compute.cinch", include_str!("../prelude/Data/Array/Compute.cinch")),
    ("Data.Device.cinch", include_str!("../prelude/Data/Device.cinch")),
];

fn run_cinch(tag: &str, extra_modules: &[(&str, &str)], main_src: &str) {
    let mut sources = SOURCES.to_vec();
    sources.extend_from_slice(extra_modules);
    sources.push(("Main.cinch", main_src));
    let dir = common::project(tag, &sources);
    let exe = common::build(&dir, "Main.cinch", "program").expect("build data-parallel program");
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "tag={tag}\nstdout:\n{stdout}\nstderr:\n{stderr}\ndir: {}",
        dir.display()
    );
    common::cleanup(&dir);
}

#[test]
fn shape_indexed_views_zero_copy_transpose_fin_and_sigma_filter() {
    run_cinch(
        "dpa_views",
        &[],
        r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Tensor
  import Data.Array.Compute
  import Data.Machine

  main :: Int
  main =
    let xs = array-from-f32s [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        dims = array-from-list [2, 3]
    in case tensor-reshape dims xs of
      None -> 1
      Some t -> case view2-from-tensor 2 3 t of
        None -> 2
        Some v2 ->
          let tr = view2-transpose v2
              contig-before = view2-is-contiguous v2
              contig-after = view2-is-contiguous tr
              bad-reshape = is-none (view2-reshape-shared tr 3 2)
              good-reshape = is-some (view2-reshape-shared v2 3 2)
          in if not contig-before || contig-after || not bad-reshape || not good-reshape
             then 3
             else case row-reduce-view2-f32 tr FloatExactTree of
               None -> 4
               Some row-sums ->
                 -- Transposed [[1,4],[2,5],[3,6]] -> row sums [5.0, 7.0, 9.0]
                 case decide (1 < 3) of
                   No _ -> 5
                   Yes bnd -> case view1-get-fin row-sums (Fin 1 bnd) of
                     None -> 6
                     Some v ->
                       if saturating-f32-to-i64 v /= 7 then 7
                       else
                         let iv = array-from-i32s [-3, 10, 2, 25, 1]
                         in case view1-of 5 iv of
                           None -> 8
                           Some v1 -> case filter-view1-i32-gt v1 3 of
                             None -> 9
                             Some (k, bv) ->
                               if k == 2 && view1-len (bounded-view1 bv) == 2 then 0 else 10
"#,
    );
}

#[test]
fn shape_mismatch_in_dot_product_is_rejected_at_compile_time() {
    let main_src = r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine

  bad :: View1 3 F32 -> View1 4 F32 -> F32
  bad a b = dot-view1-f32 a b FloatExactTree

  main :: Int
  main = 0
"#;
    let mut sources = SOURCES.to_vec();
    sources.push(("Main.cinch", main_src));
    let dir = common::project("dpa_shape_mismatch", &sources);
    let err = common::check(&dir, "Main.cinch").expect_err("mismatched static dimensions must fail typecheck");
    assert!(
        err.contains("3") || err.contains("4") || err.contains("mismatch") || err.contains("unify"),
        "unexpected error: {err}"
    );
    common::cleanup(&dir);
}

#[test]
fn producer_consumer_fusion_through_imported_helper_eliminates_allocation() {
    let helper_src = r#"module VecOps where
  import Data.Core
  import Data.Array
  import Data.Array.Compute
  export mul-vecs

  mul-vecs :: Array F32 -> Array F32 -> Option (Array F32)
  mul-vecs a b = array-zip-binop-f32 a b 2
"#;
    let main_src = r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine
  import VecOps

  dot-via-helper :: Array F32 -> Array F32 -> F32
  dot-via-helper xs ys = case mul-vecs xs ys of
    None -> 0.0
    Some prod -> array-reduce-f32 prod 10

  main :: Int
  main =
    let xs = array-from-f32s [2.0, 3.0, 4.0]
        ys = array-from-f32s [5.0, 6.0, 7.0]
        ans = dot-via-helper xs ys
    in if saturating-f32-to-i64 ans == 56 then 0 else 1
"#;
    let mut sources = SOURCES.to_vec();
    sources.push(("VecOps.cinch", helper_src));
    sources.push(("Main.cinch", main_src));
    let dir = common::project("dpa_fusion_helper", &sources);
    let ir_out = dir.join("fused.ll");
    let report_out = dir.join("array-report.json");
    let mut opts = cinch::driver::DriverOptions::default();
    opts.array_report_out = Some(report_out.clone());
    cinch::driver::driver_opts(
        cinch::driver::Mode::EmitIr,
        &dir.join("Main.cinch"),
        Some(&ir_out),
        &opts,
    )
    .expect("emit fused IR");

    let report_json = std::fs::read_to_string(&report_out).expect("read array report JSON");
    assert!(
        report_json.contains("\"fused_op\":\"arrayDotF32\""),
        "expected dot fusion in report: {report_json}"
    );
    assert!(
        report_json.contains("eliminated_allocation") && report_json.contains("4 * n bytes"),
        "expected eliminated allocation in report: {report_json}"
    );
    assert!(
        report_json.contains("f32-add-exact-tree"),
        "expected numerical mode in report: {report_json}"
    );
    let ir = std::fs::read_to_string(&ir_out).expect("read LLVM IR");
    assert!(ir.contains("@cinch_array_dot_f32"), "fused IR must call @cinch_array_dot_f32");
    assert!(
        !ir.contains("call i64 @cinch_array_zip_binop_f32("),
        "fused IR must not allocate intermediate product vector via @cinch_array_zip_binop_f32"
    );

    // Also build and run to verify exact runtime result + zero leaks!
    let exe = common::build(&dir, "Main.cinch", "fused_prog").expect("build fused program");
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    common::cleanup(&dir);
}

#[test]
fn radix_sort_spmv_scan_scatter_and_reference_agreement() {
    run_cinch(
        "dpa_algorithms",
        &[],
        r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine

  main :: Int
  main =
    -- 1. Radix sort built from map + histogram + exclusive scan + scatter-unique
    let unsorted = array-from-u32s [300, 5, 1024, 5, 0, 65537]
        sorted = radix-sort-u32 unsorted
        s0 = case array-get sorted 0 of Some x -> u32-to-i64 x; None -> -1
        s1 = case array-get sorted 1 of Some x -> u32-to-i64 x; None -> -1
        s2 = case array-get sorted 2 of Some x -> u32-to-i64 x; None -> -1
        s5 = case array-get sorted 5 of Some x -> u32-to-i64 x; None -> -1
    in if s0 /= 0 || s1 /= 5 || s2 /= 5 || s5 /= 65537 then 1
    else
      -- 2. Direct packed tabulate (no List!) + exact-tree vs reference agreement
      let xs = array-tabulate-f32 256 (\i -> if i <= 1000000 then i64-to-f32 (nat-to-i64 i) else 0.0)
          ys = array-tabulate-f32 256 (\i -> 1.0)
          _ = array-set-exec-mode 1
          ref-dot = array-dot-f32 xs ys 10
          _ = array-set-exec-mode 0
          cpu-dot = array-dot-f32 xs ys 10
      in if saturating-f32-to-i64 ref-dot /= saturating-f32-to-i64 cpu-dot || saturating-f32-to-i64 cpu-dot /= 32640 then 2
      else
        -- 3. CSR SpMV with empty rows (composed gather + zip-mul + segmented-reduce, fused!)
        let vals = array-from-f32s [2.0, 3.0, 4.0, 5.0]
            cols = array-from-i32s [0, 2, 1, 2]
            rptr = array-from-u32s [0, 2, 2, 4, 4]
            xv = array-from-f32s [10.0, 20.0, 30.0]
            yv = spmv-csr-composed-f32 vals cols rptr xv FloatExactTree
            y0 = case array-get yv 0 of Some v -> saturating-f32-to-i64 v; None -> -1
            y1 = case array-get yv 1 of Some v -> saturating-f32-to-i64 v; None -> -1
            y2 = case array-get yv 2 of Some v -> saturating-f32-to-i64 v; None -> -1
            y3 = case array-get yv 3 of Some v -> saturating-f32-to-i64 v; None -> -1
        in if y0 == 110 && y1 == 0 && y2 == 230 && y3 == 0 then 0 else 3
"#,
    );
}

#[test]
fn callback_forwarding_with_kernel_evidence_and_cinchif_metadata() {
    let lib_src = r#"module ParLib where
  import Data.Array
  export par-tabulate-with, pure-scale

  par-tabulate-with :: using (k :: Kernel (Nat -> Nat)) => (Nat -> Nat) -> Nat -> Array Nat
  par-tabulate-with f n = parTabulate n f

  pure-scale :: Array F32 -> F32 -> Array F32
  pure-scale xs c = xs
"#;
    let main_src = r#"module Main where
  import Data.Array
  import ParLib

  double-idx :: Nat -> Nat
  double-idx i = i + i

  main :: Int
  main =
    let k = (7 :: Nat)
        a = par-tabulate-with double-idx 5
        b = par-tabulate-with (\i -> i + k) 4
    in case array-get a 4 of
      None -> 1
      Some v1 -> case array-get b 3 of
        None -> 2
        Some v2 -> if v1 == 8 && v2 == 10 then 0 else 3
"#;
    let mut sources = SOURCES.to_vec();
    sources.push(("ParLib.cinch", lib_src));
    sources.push(("Main.cinch", main_src));
    let dir = common::project("dpa_kernel_forward", &sources);
    let exe = common::build(&dir, "Main.cinch", "kforward").expect("build callback-forwarding program");
    let obj = dir.join("parlib.o");
    common::drive(cinch::driver::Mode::EmitObject, &dir, "ParLib.cinch", Some(obj.clone()))
        .expect("emit ParLib object");
    let iface_text = common::object_metadata(&obj);
    assert!(
        iface_text.contains("\\\"kernel\\\"") || iface_text.contains("\"kernel\""),
        "expected kernel eligibility in ParLib .cinchif metadata:\n{iface_text}"
    );
    assert!(
        iface_text.contains("cpu_parallel"),
        "expected cpu_parallel in ParLib .cinchif metadata:\n{iface_text}"
    );
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));
    common::cleanup(&dir);
}

#[test]
fn required_metal_target_diagnoses_unsupported_ops_at_compile_time() {
    let main_src = r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine

  bad-gpu :: Array F32 -> F32
  bad-gpu xs = array-foldl-f32 (\a b -> add-f32 a b) 0.0 xs

  main :: Int
  main =
    let xs = array-from-f32s [1.0, 2.0]
        s = bad-gpu xs
    in if saturating-f32-to-i64 s == 3 then 0 else 1
"#;
    let mut sources = SOURCES.to_vec();
    sources.push(("Main.cinch", main_src));
    let dir = common::project("dpa_gpu_refusal", &sources);
    let mut opts = cinch::driver::DriverOptions::default();
    opts.array_target = Some("metal-required".to_string());
    let err = cinch::driver::driver_opts(
        cinch::driver::Mode::EmitIr,
        &dir.join("Main.cinch"),
        Some(&dir.join("out.ll")),
        &opts,
    )
    .expect_err("metal-required target must reject ordered foldl at compile time");
    let msg = err.to_string();
    assert!(
        msg.contains("arrayFoldlF32") && msg.contains("Metal GPU"),
        "expected specific GPU refusal diagnostic, got: {msg}"
    );
    common::cleanup(&dir);
}

#[cfg(target_os = "macos")]
#[test]
fn metal_gpu_resident_stencil_histogram_pipeline_in_idiomatic_cinch() {
    run_cinch(
        "dpa_metal_pipeline",
        &[],
        r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Device
  import Data.Machine
  import Data.Bytes
  import Data.String
  import Data.Result

  bytes-match-at :: Bytes -> Bytes -> Nat -> Nat -> Bool
  bytes-match-at hay needle pos k =
    if k >= bytes-len needle then True
    else if pos + k >= bytes-len hay then False
    else if bytes-get hay (pos + k) == bytes-get needle k then bytes-match-at hay needle pos (k + 1)
    else False

  bytes-has-from :: Bytes -> Bytes -> Nat -> Bool
  bytes-has-from hay needle pos =
    if pos + bytes-len needle > bytes-len hay then False
    else if bytes-match-at hay needle pos 0 then True
    else bytes-has-from hay needle (pos + 1)

  bytes-has :: Bytes -> String -> Bool
  bytes-has hay s = bytes-has-from hay (string-to-bytes s) 0

  main :: Int
  main =
    let frames = 32
        h = 32
        w = 32
        total = frames * h * w
        imgs = array-tabulate-u8 total (\i -> if i <= 10000000 then wrapping-i64-to-u8 (wrapping-mul-i64 (nat-to-i64 i) 17) else 0)
        weights = array-from-i32s [1, 1, 1, 1, 1, 1, 1, 1, 1]
    in case device-open-metal-required () of
      Err _ -> 1
      Ok dev -> case device-upload dev imgs of
        Err _ -> 2
        Ok d-in -> case device-stencil-histogram-u8 dev d-in frames h w weights 9 EdgeClamp 2 256 of
          Err _ -> 3
          Ok d-hist -> case device-download-u32 dev d-hist of
            Err _ -> 4
            Ok gpu-hist ->
              let rep = device-report-bytes dev
                  _ = ignore (device-buffer-free d-in)
                  _ = ignore (device-buffer-free d-hist)
                  -- Also verify few-large-frames launch plan on the same GPU context
                  _ = ignore (device-reset-telemetry dev)
                  f2 = 2
                  h2 = 256
                  w2 = 256
                  imgs2 = array-tabulate-u8 (f2 * h2 * w2) (\i -> if i <= 10000000 then wrapping-i64-to-u8 (wrapping-mul-i64 (nat-to-i64 i) 13) else 0)
                  ok2 = case device-upload dev imgs2 of
                    Err _ -> False
                    Ok d2-in -> case device-stencil-histogram-u8 dev d2-in f2 h2 w2 weights 9 EdgeClamp 2 256 of
                      Err _ -> False
                      Ok d2-hist -> case device-download-u32 dev d2-hist of
                        Err _ -> False
                        Ok gpu-hist2 ->
                          let rep2 = device-report-bytes dev
                              _ = ignore (device-buffer-free d2-in)
                              _ = ignore (device-buffer-free d2-hist)
                          in array-length gpu-hist2 == f2 * 256 && bytes-has rep2 "few-large-frames" && bytes-has rep2 "\"uploads\":1" && bytes-has rep2 "\"downloads\":1"
                  _ = ignore (device-close dev)
              in if not ok2 || not (bytes-has rep "many-small-frames") || not (bytes-has rep "\"uploads\":1") || not (bytes-has rep "\"downloads\":1") then 8
              else case view3-of frames h w imgs of
                None -> 5
                Some v3 -> case stencil-histogram-batch-u8 v3 weights 9 EdgeClamp 2 256 of
                  None -> 6
                  Some cpu-v2 ->
                    let cpu-hist = view2-data cpu-v2
                        g0 = case array-get gpu-hist 0 of Some x -> u32-to-i64 x; None -> (-1)
                        c0 = case array-get cpu-hist 0 of Some x -> u32-to-i64 x; None -> (-2)
                        g120 = case array-get gpu-hist 120 of Some x -> u32-to-i64 x; None -> (-1)
                        c120 = case array-get cpu-hist 120 of Some x -> u32-to-i64 x; None -> (-2)
                    in if array-length gpu-hist == frames * 256 && g0 == c0 && g120 == c120 then 0 else 7
"#,
    );
}

#[test]
fn self_hosted_compiler_packed_execution_and_cross_module_fusion() {
    let dir = common::project("dpa_self_hosted_fusion", &[
        ("Bootstrap.Check.cinch", include_str!("../../compiler/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../compiler/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../compiler/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../compiler/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../compiler/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../compiler/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../compiler/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.RuntimeIr.cinch")),
    ]);
    // Produce Data.Core.cinchif and interfaces.json for the self-hosted compiler.
    let fixities = cinch::parser::FixityTable::new();
    let tables = std::collections::HashMap::new();
    let prelude_source = "module Prelude where\n";
    let core_source = include_str!("../prelude/Data/Core.cinch");
    let dummy_main = "module Main where\n  main :: Int\n  main = 0\n";
    let prelude = cinch::driver::parse_source(prelude_source, "Prelude.cinch", &fixities, &tables).unwrap();
    let core = cinch::driver::parse_source(core_source, "Data.Core.cinch", &fixities, &tables).unwrap();
    let entry = cinch::driver::parse_source(dummy_main, "Main.cinch", &fixities, &tables).unwrap();
    let hashes = [
        ("Prelude".into(), cinch::interface::content_hash(prelude_source)),
        ("Data.Core".into(), cinch::interface::content_hash(core_source)),
        ("Main".into(), cinch::interface::content_hash(dummy_main)),
    ].into_iter().collect();
    let (_, interfaces, _, _) = cinch::check::check_program_multi_full(
        &prelude.modules,
        &[core.modules, entry.modules].concat(),
        &hashes,
        &cinch::check::StagingCx::default(),
        &[],
        &cinch::core::TargetProfile::native(),
    ).expect("check Data.Core interfaces");
    let mut rows = Vec::new();
    for iface in interfaces.into_iter().filter(|i| i.module != "Main") {
        let path = dir.join(format!("{}.cinchif", iface.module));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!(
            "[{},{},{}]",
            serde_json::to_string(&iface.module).unwrap(),
            serde_json::to_string(&iface.iface_hash()).unwrap(),
            serde_json::to_string(&path.display().to_string()).unwrap(),
        ));
    }
    rows.sort();
    std::fs::write(
        dir.join("interfaces.json"),
        format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(",")),
    ).unwrap();

    let compiler = common::build(&dir, "Main.cinch", "self-hosted-array-compiler")
        .expect("build self-hosted Cinch array compiler");

    let input_src = r#"module Helper where
  import Data.Core
  export mul-pair
  foreign import ccall "cinch.runtime.arrayZipBinopI32" zip-i32 :: Array I32 -> Array I32 -> I64 -> Option (Array I32)
  mul-pair :: Array I32 -> Array I32 -> Option (Array I32)
  mul-pair a b = zip-i32 a b 2

module Main where
  import Data.Core
  import Helper
  foreign import ccall "cinch.runtime.arrayFromListI32" from-i32s :: List I32 -> Array I32
  foreign import ccall "cinch.runtime.arrayReduceI32" reduce-i32 :: Array I32 -> I64 -> I32
  foreign import ccall "cinch.runtime.arrayTabulateI32" tab-i32 :: Nat -> (Nat -> I32) -> Array I32
  foreign import ccall "cinch.runtime.arrayLen" arr-len :: forall a. Array a -> Nat
  main :: Int
  main =
    let xs = from-i32s (Cons (2 :: I32) (Cons (3 :: I32) (Cons (4 :: I32) Nil))) in
    let ys = from-i32s (Cons (5 :: I32) (Cons (6 :: I32) (Cons (7 :: I32) Nil))) in
    let zs = tab-i32 4 (\i -> (3 :: I32)) in
    let dot = case mul-pair xs ys of
          Some prods -> reduce-i32 prods 0
          None -> (0 :: I32) in
    let zsum = reduce-i32 zs 0 in
    if dot == (56 :: I32) && zsum == (12 :: I32) && arr-len zs == 4 then 0 else 1
"#;
    std::fs::write(dir.join("input.source"), input_src).unwrap();
    let run = Command::new(&compiler).current_dir(&dir).output().unwrap();
    let status = common::read_output(&dir, "output.status");
    let diags = common::read_output(&dir, "output.diagnostics");
    assert_eq!(run.status.code(), Some(0), "self-hosted compiler exit: {}", String::from_utf8_lossy(&run.stderr));
    assert_eq!(status, "0", "self-hosted compiler diagnostics:\n{diags}");

    let ir = common::read_output(&dir, "output.ll");
    assert!(
        ir.contains("call i64 @cinch_array_tabulate_packed"),
        "expected direct packed tabulate call in self-hosted LLVM IR"
    );
    assert!(
        ir.contains("cinch-array-fusion: fused arrayZipBinopI32(MUL) -> arrayReduceI32(ADD_WRAP) into @cinch_array_dot_i32"),
        "expected cross-module fusion comment in self-hosted LLVM IR"
    );
    assert!(
        ir.contains("call i64 @cinch_array_dot_i32"),
        "expected fused @cinch_array_dot_i32 call in self-hosted LLVM IR"
    );
    assert!(
        !ir.contains("call i64 @cinch_array_zip_binop_i32"),
        "expected intermediate @cinch_array_zip_binop_i32 allocation to be eliminated in self-hosted LLVM IR"
    );

    let exe = dir.join("self_hosted_fused_bin");
    let runtime = common::host_runtime_archive();
    let linked = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
        .arg(dir.join("output.ll"))
        .arg(&runtime)
        .args(["-lpthread", "-ldl", "-lm", "-o"])
        .arg(&exe)
        .output()
        .unwrap();
    assert!(linked.status.success(), "link self-hosted IR: {}", String::from_utf8_lossy(&linked.stderr));

    let exec = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .output()
        .unwrap();
    assert_eq!(exec.status.code(), Some(0), "run self-hosted fused binary: {}", String::from_utf8_lossy(&exec.stderr));
    assert!(
        String::from_utf8_lossy(&exec.stderr).split_whitespace().any(|w| w == "leaked=0"),
        "leak check failed: {}",
        String::from_utf8_lossy(&exec.stderr)
    );
    common::cleanup(&dir);
}
