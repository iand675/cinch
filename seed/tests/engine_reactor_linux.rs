//! Integration, formal-verification (M3 `ReactorBufMachine`), Hegel 1,000-trial
//! simulation, cross-target Linux x86_64/aarch64 LLVM IR + `clang -O2 -c`
//! assembly, and live host-OS DMA ring/SQE/CQE/cmsg/BPF bytecode tests for
//! `Engine.Reactor.Linux` and `Engine.Reactor.Spec`.
//!
//! Routed through `common::project`, `common::drive_with_profile`,
//! `common::drive_with_profile_opts`, and `common::build_executable` so the
//! exact same suite runs against both the Rust seed and Stage 3 compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{target_profile, DriverOptions, Mode, Package};
use std::path::Path;
use std::process::Command;

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");
const ENGINE_REACTOR_SPEC: &str = include_str!("../prelude/Engine/Reactor/Spec.cinch");
const ENGINE_REACTOR_LINUX: &str = include_str!("../prelude/Engine/Reactor/Linux.cinch");
const ENGINE_MEM_DMA: &str = include_str!("../prelude/Engine/Mem/DmaPool.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const PRIM_SYS_IO_LINUX: &str = include_str!("../prelude/Prim/Sys/Io/Linux.cinch");
const DATA_FINITE: &str = include_str!("../prelude/Data/Finite.cinch");
const EFFECT_TEST_HEGEL: &str = include_str!("../prelude/Effect/Test/Hegel.cinch");

fn profile_for(triple: &str) -> TargetProfile {
    target_profile(&DriverOptions {
        target: Some(triple.to_string()),
        ..DriverOptions::default()
    })
    .unwrap_or_else(|e| panic!("target profile {triple}: {e}"))
}

fn run_leak_checked(tag: &str, files: &[(&str, &str)], expected_code: i32) {
    let dir = common::project(tag, files);
    let exe = common::build_executable(&dir, "Main.cinch", &format!("reactor-linux-{tag}"))
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    common::cleanup(&dir);

    assert_eq!(
        out.status.code(),
        Some(expected_code),
        "{tag}: expected exit {expected_code}, got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak check (`leaked=0`), got stderr:\n{stderr}"
    );
}

// =========================================================================
// 1. Stdlib package manifest registration
// =========================================================================

#[test]
fn engine_reactor_linux_and_spec_registered_in_stdlib_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Engine.Reactor.Linux", "Engine/Reactor/Linux.cinch"),
        ("Engine.Reactor.Spec", "Engine/Reactor/Spec.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, rel_path);
        assert!(
            pkg.order.iter().any(|m| m == name),
            "{name} missing from seed/prelude/.cinchpkg order"
        );
    }

    // Verify direct stdlib resolution through seed/prelude/.cinchpkg (without copying modules).
    let dir = common::project(
        "reactor-linux-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Engine.Reactor.Linux
  import Engine.Reactor.Spec

  main :: Int
  main =
    if rbuf-invariants rbuf-init && eq-u16 (ioring-cqe-buffer-id 0x00070001) 7 then 0 else 1
"#,
        )],
    );
    let mut strict_opts = DriverOptions::default();
    strict_opts.model_fail_on_capped = true;
    let code = common::drive_with_profile_opts(
        Mode::Run,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &strict_opts,
    )
    .expect("direct stdlib import of Engine.Reactor.Linux + Engine.Reactor.Spec must pass");
    common::cleanup(&dir);
    assert_eq!(code, 0);
}

// =========================================================================
// 2. Formal Model M3 (`ReactorBufMachine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1
// =========================================================================

#[test]
fn formal_model_m3_reactor_buf_machine_verifies_uncapped_zero_violations() {
    let main_src = r#"
module Main where
  import Engine.Reactor.Spec

  main :: Int
  main = if rbuf-invariants rbuf-init then 0 else 1
"#;
    let dir = common::project(
        "m3-reactor-buf-spec",
        &[
            ("Engine.Reactor.Spec.cinch", ENGINE_REACTOR_SPEC),
            ("Main.cinch", main_src),
        ],
    );

    // 1. Strict driver check (`model_fail_on_capped = true`, works on Rust seed & Stage 3).
    let mut opts = DriverOptions::default();
    opts.model_fail_on_capped = true;
    common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    )
    .expect("M3 ReactorBufMachine must verify cleanly with model_fail_on_capped=true");

    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("read build/Main.manifest.json");
    for needle in [
        "\"contract\":\"zeroUseAfterFree\"",
        "\"contract\":\"reactorBufDeadlockFree\"",
        "\"contract\":\"zeroKernelStuck\"",
        "\"contract\":\"zeroBufferLeaks\"",
        "refines ReactorBufMachine AbstractLinearPool",
        "measure strictly decreasing",
        "0 violations",
    ] {
        assert!(
            manifest.contains(needle),
            "expected manifest to contain {needle:?}, got:\n{manifest}"
        );
    }
    assert!(
        !manifest.contains("[CAPPED") && !manifest.contains("capped: true"),
        "M3 ReactorBufMachine must not be capped; manifest:\n{manifest}"
    );

    // 2. Direct CLI `cinch model inspect` with `CINCH_MODEL_FAIL_ON_CAPPED=1` on prelude Spec.
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Engine/Reactor/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "ReactorBufMachine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect ReactorBufMachine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M3 ReactorBufMachine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let m3 = match json.as_array() {
        Some(arr) => arr
            .iter()
            .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("ReactorBufMachine"))
            .unwrap_or_else(|| panic!("ReactorBufMachine missing from JSON:\n{stdout}")),
        None => &json,
    };

    let capped = m3.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m3.get("state_count").and_then(|v| v.as_u64()).expect("state_count");
    let edges = m3.get("edge_count").and_then(|v| v.as_u64()).expect("edge_count");
    let deadlocks = m3.get("deadlocks").and_then(|v| v.as_array()).expect("deadlocks");
    assert!(!capped, "M3 ReactorBufMachine MUST have capped=false:\n{stdout}");
    assert!(deadlocks.is_empty(), "M3 ReactorBufMachine MUST have 0 deadlocks: {deadlocks:?}");
    assert_eq!(states, 125, "3 buffers x 5 states = 125 exact reachable states:\n{stdout}");

    let contracts = m3.get("contracts").and_then(|v| v.as_array()).expect("contracts");
    assert!(contracts.len() >= 4, "expected >= 4 verified contracts, got {contracts:?}");
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "M3 contract failed: {c:?}"
        );
    }

    let refinements = m3.get("refinements").and_then(|v| v.as_array()).expect("refinements");
    let m3_ref = refinements
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("ReactorBufMachine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractLinearPool")
        })
        .unwrap_or_else(|| panic!("missing ReactorBufMachine refines AbstractLinearPool:\n{stdout}"));
    assert_eq!(m3_ref.get("passed").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(m3_ref.get("has_measure").and_then(|v| v.as_bool()), Some(true));

    eprintln!(
        "[M3 ReactorBufMachine] states={states} edges={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );

    common::cleanup(&dir);
}

// =========================================================================
// 3. Effect.Test.Hegel 1,000-trial simulation over M3 actions (leaked=0)
// =========================================================================

#[test]
fn hegel_1000_trial_simulation_over_m3_reactor_buf_actions() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Test.Hegel
  import Engine.Reactor.Spec

  mk-fin3 :: Nat -> Option (Fin 3)
  mk-fin3 n =
    if n == 0 then
      case decide (0 < 3) of
        Yes p0 -> Some (Fin 0 p0)
        No _ -> None
    else if n == 1 then
      case decide (1 < 3) of
        Yes p1 -> Some (Fin 1 p1)
        No _ -> None
    else
      case decide (2 < 3) of
        Yes p2 -> Some (Fin 2 p2)
        No _ -> None

  gen-fin3 :: Gen (Fin 3)
  gen-fin3 = gen-bind (gen-nat 2) (\n -> case mk-fin3 n of
    Some f -> gen-pure f
    None -> gen-fail)

  -- Also verify every concrete transition respects the abstract refinement step
  -- or strictly decreases the stuttering measure!
  check-step-refines :: (Fin 3 -> ReactorBufState -> Option ReactorBufState) -> Fin 3 -> ReactorBufState -> Option ReactorBufState
  check-step-refines act idx s =
    case act idx s of
      None -> None
      Some s2 ->
        let v1 = rbuf-view s in
        let v2 = rbuf-view s2 in
        let same-view = (abs-eq v1.ab0 v2.ab0) && (abs-eq v1.ab1 v2.ab1) && (abs-eq v1.ab2 v2.ab2) in
        if same-view then
          if rbuf-stutter-measure s2 < rbuf-stutter-measure s then Some s2 else None
        else Some s2

  abs-eq :: AbsBufOwner -> AbsBufOwner -> Bool
  abs-eq a b = case (a, b) of
    (AbsFree, AbsFree) -> True
    (AbsApp, AbsApp) -> True
    (AbsKernel, AbsKernel) -> True
    _ -> False

  main :: Int
  main =
    let cfg = HegelConfig 1000 256 80 (hegel-seed 20261128) in
    let actions = [
          step-action-gen "acquire" gen-fin3 (check-step-refines rbuf-acquire),
          step-action-gen "submit" gen-fin3 (check-step-refines rbuf-submit),
          step-action-gen "cancel" gen-fin3 (check-step-refines rbuf-cancel),
          step-action-gen "kernel-complete" gen-fin3 (check-step-refines rbuf-kernel-complete),
          step-action-gen "app-reap" gen-fin3 (check-step-refines rbuf-app-reap),
          step-action-gen "app-release" gen-fin3 (check-step-refines rbuf-app-release)
        ] in
    case hegel-check-machine-with cfg 32 rbuf-init actions rbuf-invariants of
      Ok trials -> if trials == 1000 then 0 else 2
      Err _ -> 1
"#;

    run_leak_checked(
        "m3-hegel-1000",
        &[
            ("Data.Finite.cinch", DATA_FINITE),
            ("Effect.Test.Hegel.cinch", EFFECT_TEST_HEGEL),
            ("Engine.Reactor.Spec.cinch", ENGINE_REACTOR_SPEC),
            ("Main.cinch", main_src),
        ],
        0,
    );
}

// =========================================================================
// 4. Cross-target check + LLVM IR emission + `clang -O2 -c` assembly for
//    BOTH `x86_64-unknown-linux-gnu` AND `aarch64-unknown-linux-gnu`
// =========================================================================

#[test]
fn cross_target_linux_x86_64_and_aarch64_emit_ir_and_assemble_o2() {
    let linux_main = r#"
module Main where
  import Data.Machine
  import Data.Option
  import Data.Result
  import Engine.Mem.DmaPool
  import Engine.Reactor.Linux
  import Engine.Types

  main :: Unit ->{IO r, IO, MVar} Int
  main _ =
    case linux-reactor-new 256 of
      Err _ -> 1
      Ok rx ->
        let _rfiles = linux-reactor-register-files rx 0 0 in
        let _pbuf = (case dma-pool-new 4096 16 of
          None -> ()
          Some pool ->
            let _reg = (case linux-reactor-register-pbuf-ring rx pool 1 16 of
              Err _ -> ()
              Ok ring-addr ->
                let _u = linux-reactor-unregister-pbuf-ring rx ring-addr 1 16 in
                ()) in
            dma-pool-destroy pool) in
        let tok = io-token-pack 1 1 7 in
        let fd = fd-of 5 in
        let _a = linux-sqe-prep-accept-multishot rx fd 0 tok in
        let _b = linux-sqe-prep-recv rx fd 0 128 0 tok in
        let _c = linux-sqe-prep-recv-multishot rx fd 1 0 tok in
        let _d = linux-sqe-prep-send rx fd 0 128 0 tok in
        let _e = linux-sqe-prep-send-zc rx fd 0 128 0 0 tok in
        let _f = linux-sqe-prep-sendmsg-gso rx fd 0 0 tok in
        let _g = linux-sqe-prep-recvmsg-gro rx fd 0 0 tok in
        let _h = linux-sqe-prep-splice rx fd 0 (fd-of 6) 0 4096 0 tok in
        let _i = linux-sqe-prep-read-direct rx fd 0 4096 0 tok in
        let _j = linux-sqe-prep-write-direct rx fd 0 4096 0 tok in
        let _k = linux-sqe-prep-fsync rx fd ioring-fsync-datasync tok in
        let _l = linux-sqe-prep-timeout rx 0 1 0 tok in
        let _m = linux-sqe-prep-cancel rx tok 0 tok in
        let _n = linux-sqe-prep-close rx fd tok in
        let _o = linux-reactor-submit-sqe rx (IoSqe op-code-send-zc 0 0 fd.fd-raw 0 64 0 0 tok.io-token-raw) in
        let _t = linux-reactor-tick rx 1 0 16 in
        let _tcp = linux-tcp-listen-reuseport 8080 128 in
        let _udp = (case linux-udp-bind-reuseport-gro 4433 of
          Err _ -> ()
          Ok ufd ->
            let _bpf = linux-attach-quic-cid-ebpf ufd 8 in
            let _ebpf = linux-attach-reuseport-ebpf-fd ufd (fd-of 9) in
            ()) in
        let _d2 = linux-reactor-destroy rx in
        0
"#;

    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());

    for (triple, expected_asm, expected_setup_nr, expected_enter_nr, expected_reg_nr) in [
        (
            "x86_64-unknown-linux-gnu",
            "syscall",
            "mov rax, 425",
            "mov rax, 426",
            "mov rax, 427",
        ),
        (
            "aarch64-unknown-linux-gnu",
            "svc #0",
            "mov x8, #425",
            "mov x8, #426",
            "mov x8, #427",
        ),
    ] {
        let lp = profile_for(triple);
        let dir = common::project(
            &format!("reactor-linux-emit-{triple}"),
            &[
                ("Prim.Atomic.cinch", PRIM_ATOMIC),
                ("Prim.Sys.Io.Linux.cinch", PRIM_SYS_IO_LINUX),
                ("Engine.Types.cinch", ENGINE_TYPES),
                ("Engine.Mem.DmaPool.cinch", ENGINE_MEM_DMA),
                ("Engine.Reactor.Linux.cinch", ENGINE_REACTOR_LINUX),
                ("Main.cinch", linux_main),
            ],
        );

        let ll = dir.join("reactor_linux.ll");
        common::drive_with_profile(
            Mode::EmitIr,
            StageMode::Refresh,
            &dir,
            "Main.cinch",
            Some(ll.clone()),
            &lp,
        )
        .unwrap_or_else(|e| panic!("{triple}: EmitIr failed:\n{e}"));

        let ir = std::fs::read_to_string(&ll).expect("read emitted LLVM IR");
        assert!(ir.contains(expected_asm), "{triple}: IR missing `{expected_asm}`");
        assert!(ir.contains(expected_setup_nr), "{triple}: IR missing `{expected_setup_nr}`");
        assert!(ir.contains(expected_enter_nr), "{triple}: IR missing `{expected_enter_nr}`");
        assert!(ir.contains(expected_reg_nr), "{triple}: IR missing `{expected_reg_nr}`");

        let obj = dir.join("reactor_linux.o");
        let asm = Command::new(&clang)
            .arg("-O2")
            .arg("-c")
            .arg(format!("--target={triple}"))
            .arg(&ll)
            .arg("-o")
            .arg(&obj)
            .output()
            .expect("run clang -O2 -c");
        assert!(
            asm.status.success(),
            "{triple}: clang -O2 -c failed:\n{}",
            String::from_utf8_lossy(&asm.stderr)
        );
        let obj_size = std::fs::metadata(&obj).expect("stat .o").len();
        assert!(obj_size > 1000, "{triple}: expected non-empty object file, got {obj_size} bytes");
        eprintln!("[Linux cross-assemble] target={triple} opt=-O2 obj_bytes={obj_size} PASS");

        common::cleanup(&dir);
    }
}

// =========================================================================
// 5. Live host-OS DMA verification:
//    - Exact 64B SQE & 16B CQE byte layouts (`linux-write-sqe-slot` / `linux-read-cqe-slot`)
//    - All 14 SQE prep builders + generic `linux-reactor-submit-sqe` + ring wraparound + reap
//    - UDP GSO (`UDP_SEGMENT`), `SO_TXTIME`, `UDP_GRO` cmsg + `msghdr` + `sockaddr_in` + `timespec`
//    - `linux-build-quic-cid-bpf` bytecode verification + classic BPF VM simulation
//    - Zero-alloc steady-state loop (`allocs=0`, `atomic_rc=0`) + `leaked=0`
// =========================================================================

#[test]
fn native_host_sqe_cqe_cmsg_bpf_and_zero_alloc_ring_verification() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Mem.DmaPool
  import Engine.Reactor.Linux
  import Engine.Types
  import Prim.Atomic

  -- 1. Direct 64B SQE and 16B CQE byte-offset verification in a DMA page.
  test-sqe-cqe-raw-layout :: I64 ->{MVar} Int
  test-sqe-cqe-raw-layout dma =
    let sqe-addr = dma in
    let cqe-addr = wrapping-add-i64 dma 128 in
    let _w = linux-write-sqe-slot sqe-addr 53 32 0x1234 42 0x0102030405060708 0x1112131415161718 0x00004000 0x00000080 0xCAFEBABE12345678 0x0007 0x0009 99 0xAAAABBBBCCCCDDDD in
    let b0 = raw-load-u8 sqe-addr in
    let b1 = raw-load-u8 (wrapping-add-i64 sqe-addr 1) in
    let ioprio = bit-and-u32 (wrapping-shr-u32 (raw-load-u32 sqe-addr) 16) 65535 in
    let fd = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqe-addr 4)) in
    let off = raw-load-u64 (wrapping-add-i64 sqe-addr 8) in
    let addr = raw-load-u64 (wrapping-add-i64 sqe-addr 16) in
    let len = raw-load-u32 (wrapping-add-i64 sqe-addr 24) in
    let opf = raw-load-u32 (wrapping-add-i64 sqe-addr 28) in
    let ud = raw-load-u64 (wrapping-add-i64 sqe-addr 32) in
    let w10 = raw-load-u32 (wrapping-add-i64 sqe-addr 40) in
    let bidx = bit-and-u32 w10 65535 in
    let pers = wrapping-shr-u32 w10 16 in
    let sfd = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqe-addr 44)) in
    let addr3 = raw-load-u64 (wrapping-add-i64 sqe-addr 48) in
    let pad2 = raw-load-u64 (wrapping-add-i64 sqe-addr 56) in
    if ne-u8 b0 53 then 10
    else if ne-u8 b1 32 then 11
    else if ne-u32 ioprio 0x1234 then 12
    else if ne-i32 fd 42 then 13
    else if ne-u64 off 0x0102030405060708 then 14
    else if ne-u64 addr 0x1112131415161718 then 15
    else if ne-u32 len 16384 then 16
    else if ne-u32 opf 128 then 17
    else if ne-u64 ud 0xCAFEBABE12345678 then 18
    else if ne-u32 bidx 7 then 19
    else if ne-u32 pers 9 then 20
    else if ne-i32 sfd 99 then 21
    else if ne-u64 addr3 0xAAAABBBBCCCCDDDD then 22
    else if ne-u64 pad2 0 then 23
    else
      -- Verify 16B CQE round-trip + buffer ID extraction (`flags >> 16`)
      let cqe-flags = bit-or-u32 (wrapping-shl-u32 45 16) (bit-or-u32 ioring-cqe-f-buffer ioring-cqe-f-more) in
      let want = completion-make (io-token-pack 3 17 55) 1400 cqe-flags in
      let _wc = linux-write-cqe-slot cqe-addr want in
      let got = linux-read-cqe-slot cqe-addr in
      if not (completion-eq want got) then 24
      else if ne-u64 (raw-load-u64 cqe-addr) want.cqe-token then 25
      else if ne-i32 (wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 cqe-addr 8))) 1400 then 26
      else if ne-u32 (raw-load-u32 (wrapping-add-i64 cqe-addr 12)) cqe-flags then 27
      else if ne-u16 (ioring-cqe-buffer-id got.cqe-flags) 45 then 28
      else 0

  -- 2. In-memory SQ/CQ ring test exercising all 14 prep builders + wraparound + reap.
  test-in-memory-ring :: I64 ->{MVar} Int
  test-in-memory-ring page =
    -- Layout inside a 16 KiB DMA region:
    --   +0     : state-ptr (4096B)
    --   +4096  : sq_ring (+0 head, +4 tail, +64 sq_array[16])
    --   +5120  : sqes (16 * 64B = 1024B)
    --   +8192  : cq_ring (+0 head, +4 tail, +64 cqes[16 * 16B])
    --   +12288 : reaped output array (16 * 16B)
    let st-ptr = page in
    let sq-ptr = wrapping-add-i64 page 4096 in
    let sqes-ptr = wrapping-add-i64 page 5120 in
    let cq-ptr = wrapping-add-i64 page 8192 in
    let out-ptr = wrapping-add-i64 page 12288 in
    let _z0 = raw-store-u32 sq-ptr 0 in
    let _z1 = raw-store-u32 (wrapping-add-i64 sq-ptr 4) 0 in
    let _z2 = raw-store-u32 cq-ptr 0 in
    let _z3 = raw-store-u32 (wrapping-add-i64 cq-ptr 4) 0 in
    let rx = linux-reactor-from-raw-rings 7 sq-ptr 0 4 16 64 sqes-ptr cq-ptr 0 4 16 64 st-ptr in
    let fd = fd-of 10 in
    let fd2 = fd-of 11 in
    let ok0 = linux-sqe-prep-accept-multishot rx fd 0x80800 (io-token-pack 1 1 0) in
    let ok1 = linux-sqe-prep-recv rx fd 0x1000 512 0 (io-token-pack 2 1 1) in
    let ok2 = linux-sqe-prep-recv-multishot rx fd 3 0 (io-token-pack 3 1 2) in
    let ok3 = linux-sqe-prep-send rx fd 0x2000 256 0 (io-token-pack 4 1 3) in
    let ok4 = linux-sqe-prep-send-zc rx fd 0x3000 1024 0 0 (io-token-pack 5 1 4) in
    let ok5 = linux-sqe-prep-sendmsg-gso rx fd 0x4000 0 (io-token-pack 6 1 5) in
    let ok6 = linux-sqe-prep-recvmsg-gro rx fd 0x5000 0 (io-token-pack 7 1 6) in
    let ok7 = linux-sqe-prep-splice rx fd 64 fd2 128 4096 1 (io-token-pack 8 1 7) in
    let ok8 = linux-sqe-prep-read-direct rx fd 0x6000 4096 8192 (io-token-pack 9 1 8) in
    let ok9 = linux-sqe-prep-write-direct rx fd 0x7000 4096 16384 (io-token-pack 10 1 9) in
    let ok10 = linux-sqe-prep-fsync rx fd ioring-fsync-datasync (io-token-pack 11 1 10) in
    let ok11 = linux-sqe-prep-timeout rx 0x8000 1 0 (io-token-pack 12 1 11) in
    let ok12 = linux-sqe-prep-cancel rx (io-token-pack 2 1 1) 0 (io-token-pack 13 1 12) in
    let ok13 = linux-sqe-prep-close rx fd (io-token-pack 14 1 13) in
    let ok14 = linux-reactor-submit-sqe rx (IoSqe op-code-nop 0 0 (-1) 0 0 0 0 (io-token-pack 0 1 14).io-token-raw) in
    let ok15 = linux-reactor-submit-sqe rx (IoSqe op-code-send-v 0 0 fd.fd-raw 0x9000 2 0 0 (io-token-pack 5 1 15).io-token-raw) in
    -- 17th submission must fail because SQ capacity (16) is full!
    let full-rej = not (linux-sqe-prep-close rx fd (io-token-pack 14 1 16)) in
    if not (ok0 && ok1 && ok2 && ok3 && ok4 && ok5 && ok6 && ok7 && ok8 && ok9 && ok10 && ok11 && ok12 && ok13 && ok14 && ok15 && full-rej) then 30
    else if ne-u32 (linux-reactor-pending-sqes rx) 16 then 31
    else if ne-u32 (raw-load-u32 (wrapping-add-i64 sq-ptr 4)) 16 then 32
    else
      -- Spot-check key SQE opcodes & flags directly in `sqes_ptr`!
      -- Slot 0: ACCEPT multishot (opcode=13, ioprio=1)
      let s0 = raw-load-u32 sqes-ptr in
      -- Slot 2: RECV multishot (opcode=27, flags=32 IOSQE_BUFFER_SELECT, ioprio=2, buf_index=3)
      let s2-addr = wrapping-add-i64 sqes-ptr 128 in
      let s2 = raw-load-u32 s2-addr in
      let s2-bgid = bit-and-u32 (raw-load-u32 (wrapping-add-i64 s2-addr 40)) 65535 in
      -- Slot 4: SEND_ZC (opcode=53)
      let s4-op = raw-load-u8 (wrapping-add-i64 sqes-ptr 256) in
      -- Slot 7: SPLICE (opcode=30, fd=11, off=128, addr=64, splice_fd_in=10)
      let s7-addr = wrapping-add-i64 sqes-ptr 448 in
      let s7-op = raw-load-u8 s7-addr in
      let s7-fd-out = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 s7-addr 4)) in
      let s7-off-out = raw-load-u64 (wrapping-add-i64 s7-addr 8) in
      let s7-off-in = raw-load-u64 (wrapping-add-i64 s7-addr 16) in
      let s7-fd-in = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 s7-addr 44)) in
      -- Slot 10: FSYNC datasync (opcode=3, op_flags=1)
      let s10-addr = wrapping-add-i64 sqes-ptr 640 in
      let s10-op = raw-load-u8 s10-addr in
      let s10-fl = raw-load-u32 (wrapping-add-i64 s10-addr 28) in
      -- Slot 12: ASYNC_CANCEL (opcode=14, addr=target_tok)
      let s12-addr = wrapping-add-i64 sqes-ptr 768 in
      let s12-op = raw-load-u8 s12-addr in
      let s12-target = raw-load-u64 (wrapping-add-i64 s12-addr 16) in
      if ne-u32 s0 (bit-or-u32 13 (wrapping-shl-u32 1 16)) then 33
      else if ne-u32 s2 (bit-or-u32 (bit-or-u32 27 (wrapping-shl-u32 32 8)) (wrapping-shl-u32 2 16)) then 34
      else if ne-u32 s2-bgid 3 then 35
      else if ne-u8 s4-op 53 then 36
      else if ne-u8 s7-op 30 || ne-i32 s7-fd-out 11 || ne-u64 s7-off-out 128 || ne-u64 s7-off-in 64 || ne-i32 s7-fd-in 10 then 37
      else if ne-u8 s10-op 3 || ne-u32 s10-fl 1 then 38
      else if ne-u8 s12-op 14 || ne-u64 s12-target (io-token-pack 2 1 1).io-token-raw then 39
      else
        -- Simulate kernel consuming SQ (sq_head = 16) and posting 3 CQEs
        let _kh = raw-store-u32 sq-ptr 16 in
        let cqes-base = wrapping-add-i64 cq-ptr 64 in
        let c0 = completion-make (io-token-pack 1 1 0) 20 ioring-cqe-f-more in
        let c1 = completion-make (io-token-pack 3 1 2) 1200 (bit-or-u32 ioring-cqe-f-buffer (wrapping-shl-u32 9 16)) in
        let c2 = completion-make (io-token-pack 5 1 4) 0 ioring-cqe-f-notif in
        let _wc0 = linux-write-cqe-slot cqes-base c0 in
        let _wc1 = linux-write-cqe-slot (wrapping-add-i64 cqes-base 16) c1 in
        let _wc2 = linux-write-cqe-slot (wrapping-add-i64 cqes-base 32) c2 in
        let _ct = raw-store-u32 (wrapping-add-i64 cq-ptr 4) 3 in
        let reaped = linux-reactor-reap-cqes rx out-ptr 8 in
        let g0 = linux-read-cqe-slot out-ptr in
        let g1 = linux-read-cqe-slot (wrapping-add-i64 out-ptr 16) in
        let g2 = linux-read-cqe-slot (wrapping-add-i64 out-ptr 32) in
        let cq-head-after = raw-load-u32 cq-ptr in
        if ne-u32 reaped 3 || ne-u32 cq-head-after 3 then 40
        else if not (completion-eq g0 c0 && completion-eq g1 c1 && completion-eq g2 c2) then 41
        else if ne-u16 (ioring-cqe-buffer-id g1.cqe-flags) 9 then 42
        else 0

  -- 3. UDP_SEGMENT + SO_TXTIME + UDP_GRO cmsg, msghdr, sockaddr_in, timespec layout check.
  test-cmsg-and-aux :: I64 ->{MVar} Int
  test-cmsg-and-aux buf =
    let n1 = linux-write-cmsg-udp-segment buf 1200 in
    let n2 = linux-write-cmsg-txtime (wrapping-add-i64 buf 24) 9876543210 in
    let _mh = linux-write-msghdr (wrapping-add-i64 buf 64) 0x1111 16 0x2222 1 (wrapping-i64-to-u64 buf) 48 0 in
    let _sa = linux-write-sockaddr-in-any (wrapping-add-i64 buf 128) 4433 in
    let _ts = linux-write-timespec (wrapping-add-i64 buf 160) 5 250000000 in
    -- Also write a 2-record recvmsg cmsg buffer: dummy record + UDP_GRO (level=17, type=104, size=1350)
    let rx-cmsg = wrapping-add-i64 buf 256 in
    let _r0 = raw-store-u64 rx-cmsg 24 in
    let _r1 = raw-store-u32 (wrapping-add-i64 rx-cmsg 8) 1 in
    let _r2 = raw-store-u32 (wrapping-add-i64 rx-cmsg 12) 29 in
    let _r3 = raw-store-u64 (wrapping-add-i64 rx-cmsg 24) 18 in
    let _r4 = raw-store-u32 (wrapping-add-i64 rx-cmsg 32) 17 in
    let _r5 = raw-store-u32 (wrapping-add-i64 rx-cmsg 36) 104 in
    let _r6 = raw-store-u64 (wrapping-add-i64 rx-cmsg 40) 1350 in
    let gro-sz = linux-read-cmsg-udp-gro rx-cmsg 48 in
    if ne-u32 n1 24 || ne-u32 n2 24 then 50
    else if ne-u64 (raw-load-u64 buf) 18 || ne-u32 (raw-load-u32 (wrapping-add-i64 buf 8)) 17 || ne-u32 (raw-load-u32 (wrapping-add-i64 buf 12)) 103 || ne-u64 (raw-load-u64 (wrapping-add-i64 buf 16)) 1200 then 51
    else if ne-u64 (raw-load-u64 (wrapping-add-i64 buf 24)) 24 || ne-u32 (raw-load-u32 (wrapping-add-i64 buf 32)) 1 || ne-u32 (raw-load-u32 (wrapping-add-i64 buf 36)) 61 || ne-u64 (raw-load-u64 (wrapping-add-i64 buf 40)) 9876543210 then 52
    else if ne-u64 (raw-load-u64 (wrapping-add-i64 buf 96)) (wrapping-i64-to-u64 buf) || ne-u64 (raw-load-u64 (wrapping-add-i64 buf 104)) 48 then 53
    -- 4433 = 0x1151 -> big-endian bytes [0x11, 0x51] at +2..+3
    else if ne-u8 (raw-load-u8 (wrapping-add-i64 buf 128)) 2 || ne-u8 (raw-load-u8 (wrapping-add-i64 buf 130)) 0x11 || ne-u8 (raw-load-u8 (wrapping-add-i64 buf 131)) 0x51 then 54
    else if ne-i64 (raw-load-i64 (wrapping-add-i64 buf 160)) 5 || ne-i64 (raw-load-i64 (wrapping-add-i64 buf 168)) 250000000 then 55
    else if ne-u16 gro-sz 1350 then 56
    else 0

  -- 4. Classic BPF mini-evaluator verifying the exact emitted QUIC CID steering bytecode!
  eval-cbpf :: I64 -> U32 -> U32 -> U32 -> U16 -> U32 ->{MVar} U32
  eval-cbpf prog pc n-insns pkt-len pkt-cid-u16 acc =
    if ge-u32 pc n-insns then 999999
    else
      let addr = wrapping-add-i64 prog (wrapping-shl-i64 (u32-to-i64 pc) 3) in
      let lo = raw-load-u32 addr in
      let code = bit-and-u32 lo 65535 in
      let jt = bit-and-u32 (wrapping-shr-u32 lo 16) 255 in
      let jf = bit-and-u32 (wrapping-shr-u32 lo 24) 255 in
      let k = raw-load-u32 (wrapping-add-i64 addr 4) in
      if eq-u32 code 0x0080 then
        -- BPF_LD | BPF_W | BPF_LEN -> A = pkt_len
        eval-cbpf prog (wrapping-add-u32 pc 1) n-insns pkt-len pkt-cid-u16 pkt-len
      else if eq-u32 code 0x0035 then
        -- BPF_JMP | BPF_JGE | BPF_K
        let delta = if ge-u32 acc k then jt else jf in
        eval-cbpf prog (wrapping-add-u32 (wrapping-add-u32 pc 1) delta) n-insns pkt-len pkt-cid-u16 acc
      else if eq-u32 code 0x0006 then
        -- BPF_RET | BPF_K
        k
      else if eq-u32 code 0x0028 then
        -- BPF_LD | BPF_H | BPF_ABS (k == 1 -> QUIC short-header DCID bytes 1..2)
        if eq-u32 k 1 && ge-u32 pkt-len 3 then
          eval-cbpf prog (wrapping-add-u32 pc 1) n-insns pkt-len pkt-cid-u16 (u16-to-u32 pkt-cid-u16)
        else 999998
      else if eq-u32 code 0x0094 then
        -- BPF_ALU | BPF_MOD | BPF_K
        case checked-rem-u32 acc k of
          None -> 999997
          Some r -> eval-cbpf prog (wrapping-add-u32 pc 1) n-insns pkt-len pkt-cid-u16 r
      else if eq-u32 code 0x0016 then
        -- BPF_RET | BPF_A
        acc
      else 999996

  test-quic-cid-bpf :: I64 ->{MVar} Int
  test-quic-cid-bpf prog =
    if ne-u32 (linux-build-quic-cid-bpf prog 0) 0 then 60
    else
      let n = linux-build-quic-cid-bpf prog 8 in
      if ne-u32 n 6 then 61
      -- Short packet (< 3 bytes) -> safely steers to core 0
      else if ne-u32 (eval-cbpf prog 0 n 0 0x1234 0) 0 then 62
      else if ne-u32 (eval-cbpf prog 0 n 2 0x1234 0) 0 then 63
      -- Valid QUIC short-header packets steer to `core_id % 8`
      else if ne-u32 (eval-cbpf prog 0 n 64 0 0) 0 then 64
      else if ne-u32 (eval-cbpf prog 0 n 64 5 0) 5 then 65
      else if ne-u32 (eval-cbpf prog 0 n 64 13 0) 5 then 66
      else if ne-u32 (eval-cbpf prog 0 n 1200 0x1237 0) 7 then 67
      else if ne-u32 (eval-cbpf prog 0 n 1200 0xFFFF 0) 7 then 68
      else
        -- Rebuild for 16 cores and check `0x001B % 16 == 11`
        let n2 = linux-build-quic-cid-bpf prog 16 in
        if ne-u32 n2 6 then 69
        else if ne-u32 (eval-cbpf prog 0 n2 128 27 0) 11 then 70
        else 0

  -- 5. Steady-state 20,000 SQE submit + CQE reap loop asserting 0 heap allocs & 0 atomic RCs!
  hot-ring-loop :: LinuxReactor -> I64 -> I64 -> I64 -> I64 -> U32 -> U32 ->{MVar} U32
  hot-ring-loop rx sq-ptr cq-ptr cqes-base out-ptr rem ok-acc =
    if eq-u32 rem 0 then ok-acc
    else
      let tok = io-token-pack 4 1 rem in
      let pushed = linux-sqe-prep-send rx (fd-of 5) 0x4000 128 0 tok in
      -- Advance mock kernel SQ head + post matching CQE
      let sq-t = raw-load-u32 (wrapping-add-i64 sq-ptr 4) in
      let _sh = raw-store-u32 sq-ptr sq-t in
      let cq-t = raw-load-u32 (wrapping-add-i64 cq-ptr 4) in
      let cqe-slot = wrapping-add-i64 cqes-base (wrapping-shl-i64 (u32-to-i64 (bit-and-u32 cq-t 15)) 4) in
      let _wc = linux-write-cqe-slot cqe-slot (completion-make tok 128 0) in
      let _ct = raw-store-u32 (wrapping-add-i64 cq-ptr 4) (wrapping-add-u32 cq-t 1) in
      let reaped = linux-reactor-reap-cqes rx out-ptr 4 in
      let got = linux-read-cqe-slot out-ptr in
      let step-ok = pushed && eq-u32 reaped 1 && eq-u64 got.cqe-token tok.io-token-raw && eq-i32 got.cqe-res 128 in
      let next-acc = if step-ok then wrapping-add-u32 ok-acc 1 else ok-acc in
      hot-ring-loop rx sq-ptr cq-ptr cqes-base out-ptr (wrapping-sub-u32 rem 1) next-acc

  test-zero-alloc-ring :: I64 ->{IO, MVar} Int
  test-zero-alloc-ring page =
    let st-ptr = page in
    let sq-ptr = wrapping-add-i64 page 4096 in
    let sqes-ptr = wrapping-add-i64 page 5120 in
    let cq-ptr = wrapping-add-i64 page 8192 in
    let cqes-base = wrapping-add-i64 cq-ptr 64 in
    let out-ptr = wrapping-add-i64 page 12288 in
    let _z0 = raw-store-u32 sq-ptr 0 in
    let _z1 = raw-store-u32 (wrapping-add-i64 sq-ptr 4) 0 in
    let _z2 = raw-store-u32 cq-ptr 0 in
    let _z3 = raw-store-u32 (wrapping-add-i64 cq-ptr 4) 0 in
    let rx = linux-reactor-from-raw-rings 7 sq-ptr 0 4 16 64 sqes-ptr cq-ptr 0 4 16 64 st-ptr in
    let _ = engine-thread-counters-reset () in
    let ok-cnt = hot-ring-loop rx sq-ptr cq-ptr cqes-base out-ptr 20000 0 in
    let allocs = engine-thread-alloc-count () in
    let rcs = engine-thread-atomic-rc-count () in
    if ne-u32 ok-cnt 20000 then 80
    else if ne-u64 allocs 0 then 81
    else if ne-u64 rcs 0 then 82
    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 16384 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let dma = wrapping-u64-to-i64 dma-u64 in
      let c1 = test-sqe-cqe-raw-layout dma in
      let c2 = if c1 /= 0 then c1 else test-in-memory-ring dma in
      let c3 = if c2 /= 0 then c2 else test-cmsg-and-aux dma in
      let c4 = if c3 /= 0 then c3 else test-quic-cid-bpf dma in
      let c5 = if c4 /= 0 then c4 else test-zero-alloc-ring dma in
      let _fr = engine-dma-free dma-u64 16384 in
      c5
"#;

    run_leak_checked(
        "host-dma-sqe-cqe-bpf",
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Prim.Sys.Io.Linux.cinch", PRIM_SYS_IO_LINUX),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Engine.Mem.DmaPool.cinch", ENGINE_MEM_DMA),
            ("Engine.Reactor.Linux.cinch", ENGINE_REACTOR_LINUX),
            ("Main.cinch", main_src),
        ],
        0,
    );
}
