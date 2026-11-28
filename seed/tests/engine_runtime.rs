//! Integration tests for `Engine.Runtime` (share-nothing pinned worker runtime loop):
//!
//! 1. Stdlib package manifest registration, canonical CinchDoc documentation,
//!    and clean typechecking via `common::drive_with_profile`.
//! 2. Deterministic `Engine.Sim` execution of `runtime-run-ops`: exercises all
//!    5 canonical tick phases (inbound `CoreMesh` SPSC drain, simulated TCP echo
//!    via `sim-reactor-ops`, `TimerWheel` + `CoarseClockCache` 1s RFC 9110 Date
//!    refresh, coalesced outbound SPSC + SQE flush, per-tick `BumpArena` reset)
//!    and graceful drain shutdown, proving bit-exact replay across two runs with
//!    the same 64-bit seed and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//! 3. Live multi-core pinned OS worker test on macOS: spawns 2 pinned workers
//!    via `runtime-spawn-workers` (`engine-spawn-pinned` + `CoreMesh` + atomic
//!    ready/shutdown flags), runs live Darwin `kqueue` loopback TCP echo across
//!    workers alongside bidirectional `CoreMesh` SPSC messaging, verifies
//!    graceful drain shutdown (`runtime-request-shutdown` + `runtime-join-all`)
//!    and `leaked=0`.
//! 4. Steady-state 20,000-tick hot loop across both `runtime-run-ops` and
//!    `runtime-run-raw` asserting `engine-thread-alloc-count == 0`,
//!    `engine-thread-atomic-rc-count == 0`, and `leaked=0`.
//!
//! Routed through `common::drive_with_profile` / `common::drive_with_profile_opts`
//! / `common::build_executable` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const ENGINE_RUNTIME: &str = include_str!("../prelude/Engine/Runtime.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-engine-runtime-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[cfg(target_os = "macos")]
fn c_fixture(name: &str, src: &str) -> PathBuf {
    let dir = fresh_dir(&format!("cfix-{name}"));
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write C fixture");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = Command::new(&clang)
        .arg("-O2")
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile C fixture");
    o_path
}

fn build_and_run_leak_checked(tag: &str, src: &str, objects: &[PathBuf], expected_code: i32) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("runtime-{tag}"));
    let mut opts = DriverOptions::default();
    opts.native_objects = objects.to_vec();
    let build_res = common::drive_with_profile_opts(
        Mode::Build,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &TargetProfile::native(),
        &opts,
    );
    if let Err(e) = build_res {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{tag}: build failed:\n{e}");
    }

    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

    assert_eq!(
        out.status.code(),
        Some(expected_code),
        "{tag}: expected exit {expected_code}, got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report (`leaked=0`), got stderr:\n{stderr}"
    );
}

// =========================================================================
// 1. Manifest registration, documentation, and typecheck verification
// =========================================================================

#[test]
fn engine_runtime_registered_documented_and_checks() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Runtime")
        .expect("Engine.Runtime listed in seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Engine/Runtime.cinch");

    let pos_of = |name: &str| {
        pkg.order
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg order"))
    };
    let pos_rt = pos_of("Engine.Runtime");
    for dep in [
        "Engine.Types",
        "Engine.Mem.Arena",
        "Engine.Mem.DmaPool",
        "Engine.Ring.Spsc",
        "Engine.Timer",
        "Engine.Reactor",
    ] {
        assert!(
            pos_of(dep) < pos_rt,
            "{dep} must precede Engine.Runtime in seed/prelude/.cinchpkg order"
        );
    }

    common::assert_documented("Engine.Runtime", ENGINE_RUNTIME);

    let dir = fresh_dir("check-iface");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Engine.Runtime

  main :: Int
  main =
    let cfg = worker-config-single-core () in
    if eq-u32 cfg.cfg-worker-count 1 then 0 else 1
"#,
    )
    .expect("write Main.cinch");
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("Engine.Runtime typechecks cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Deterministic Engine.Sim execution of `runtime-run-ops`
//    (SPSC mesh + timers + CoarseClockCache + simulated TCP echo +
//     BumpArena reset + graceful drain; bit-exact across 2 replays)
// =========================================================================

#[test]
fn engine_runtime_sim_deterministic_replay_spsc_timers_tcp_echo_and_drain() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Hash
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Mem.Arena
  import Engine.Mem.DmaPool
  import Engine.Ring.Spsc
  import Engine.Runtime
  import Engine.Sim
  import Engine.Sim.Faults
  import Engine.Timer
  import Engine.Types
  import Prim.Atomic

  -- Scratch DMA layout for one deterministic simulation scenario (4096 bytes):
  --   +0   : tcp-tx-buf (64B)
  --   +64  : tcp-srv-rx-buf (64B)
  --   +128 : tcp-cli-rx-buf (64B)
  --   +256 : echo-count :: U64
  --   +264 : timer-sum :: U64
  --   +272 : msg-sum :: U64
  --   +280 : phase-order-ok :: U64 (1 = intact)
  --   +288 : last-phase :: U64
  --   +296 : last-tick-seen :: U64
  --   +304 : payload-hash :: U64

  record SimRunResult where
    res-trace :: U64
    res-now-ns :: U64
    res-events :: U64
    res-ticks :: U64
    res-msgs-drained :: U64
    res-msgs-sent :: U64
    res-cqes :: U64
    res-timers :: U64
    res-date-refreshes :: U64
    res-arena-resets :: U64
    res-arena-peak :: U64
    res-echo-count :: U64
    res-timer-sum :: U64
    res-msg-sum :: U64
    res-payload-hash :: U64
    res-date-str :: String
    res-order-ok :: Bool

  sim-run-eq :: SimRunResult -> SimRunResult -> Bool
  sim-run-eq a b =
    eq-u64 a.res-trace b.res-trace
      && eq-u64 a.res-now-ns b.res-now-ns
      && eq-u64 a.res-events b.res-events
      && eq-u64 a.res-ticks b.res-ticks
      && eq-u64 a.res-msgs-drained b.res-msgs-drained
      && eq-u64 a.res-msgs-sent b.res-msgs-sent
      && eq-u64 a.res-cqes b.res-cqes
      && eq-u64 a.res-timers b.res-timers
      && eq-u64 a.res-date-refreshes b.res-date-refreshes
      && eq-u64 a.res-arena-resets b.res-arena-resets
      && eq-u64 a.res-arena-peak b.res-arena-peak
      && eq-u64 a.res-echo-count b.res-echo-count
      && eq-u64 a.res-timer-sum b.res-timer-sum
      && eq-u64 a.res-msg-sum b.res-msg-sum
      && eq-u64 a.res-payload-hash b.res-payload-hash
      && (string-compare a.res-date-str b.res-date-str == 0)
      && (a.res-order-ok == b.res-order-ok)

  -- Verify strict monotonic phase order 1 -> 2 -> 3 -> 4 within every tick.
  record-phase :: I64 -> WorkerState -> U64 ->{MVar} Unit
  record-phase scratch ws phase =
    let m = worker-metrics-read ws in
    let cur-tick = m.metric-ticks in
    let prev-tick = raw-load-u64 (wrapping-add-i64 scratch 296) in
    let prev-phase = raw-load-u64 (wrapping-add-i64 scratch 288) in
    let ok = if ne-u64 cur-tick prev-tick then True else ge-u64 phase prev-phase in
    let _o = if not ok then raw-store-u64 (wrapping-add-i64 scratch 280) 0 else () in
    let _t = raw-store-u64 (wrapping-add-i64 scratch 296) cur-tick in
    raw-store-u64 (wrapping-add-i64 scratch 288) phase

  fill-payload :: I64 -> U64 ->{MVar} Unit
  fill-payload buf seq =
    let _a = raw-store-u64 buf seq in
    let _b = raw-store-u64 (wrapping-add-i64 buf 8) (bit-xor-u64 (wrapping-mul-u64 seq 0x9E3779B97F4A7C15) 0xCAFEBABE12345678) in
    let _c = raw-store-u64 (wrapping-add-i64 buf 16) (wrapping-add-u64 seq 99) in
    raw-store-u64 (wrapping-add-i64 buf 24) (bit-not-u64 seq)

  run-dst-once :: forall (r :: Region). U64 ->{IO, IO r, MVar, Mut r} Option SimRunResult
  run-dst-once seed =
    let faults = SimFaultConfig {
      udp-drop-ppm = 0, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 0,
      udp-min-delay-ns = 2000000, udp-max-delay-ns = 5000000,
      tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
      disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 0,
      partition-mask = 0
    } in
    let w = sim-world-new seed faults in
    -- Set wall base to 2026-11-28 14:00:00.900 UTC so advancing 200ms crosses into 14:00:01!
    let _wb = sim-world-set-wall-base w (time-ns-of 1795874400900000000) in
    case sim-tcp-pair w 0 1 of
      (cli-fd, srv-fd) ->
        case core-mesh-new 2 64 of
          None -> None
          Some mesh ->
            let cfg = WorkerConfig 2 4096 8 16 16 16 128 3 8192 64 25000000 0 in
            case worker-state-new (core-id 0) cfg mesh 0 0 of
              None ->
                let _fm = core-mesh-destroy mesh in
                None
              Some ws ->
                let scratch-u = engine-dma-alloc 4096 4096 in
                if eq-u64 scratch-u 0 then
                  let _dw = worker-state-destroy ws in
                  let _fm = core-mesh-destroy mesh in
                  None
                else
                  let scratch = wrapping-u64-to-i64 scratch-u in
                  let tx-buf = scratch in
                  let srv-rx = wrapping-add-i64 scratch 64 in
                  let cli-rx = wrapping-add-i64 scratch 128 in
                  let _init0 = raw-store-u64 (wrapping-add-i64 scratch 280) 1 in
                  let _init1 = raw-store-u64 (wrapping-add-i64 scratch 296) max-bound-u64 in
                  let rx = sim-reactor-ops w in
                  let clk = sim-clock-ops w in
                  -- Seed 4 inbound SPSC messages from peer core 1 -> core 0
                  let _m1 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 1 (core-id 1) (core-id 0) 10 100 0 0 0 0 0) in
                  let _m2 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 1 (core-id 1) (core-id 0) 11 200 0 0 0 0 0) in
                  let _m3 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 1 (core-id 1) (core-id 0) 12 300 0 0 0 0 0) in
                  let _m4 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 1 (core-id 1) (core-id 0) 13 400 0 0 0 0 0) in
                  -- Schedule 3 timers across 30ms, 80ms, and 140ms (the 140ms timer triggers graceful shutdown!)
                  let _t1 = timer-wheel-schedule (worker-timers ws) 30 (io-token-pack 9 1 101) in
                  let _t2 = timer-wheel-schedule (worker-timers ws) 80 (io-token-pack 9 1 102) in
                  let _t3 = timer-wheel-schedule (worker-timers ws) 140 (io-token-pack 9 1 999) in
                  -- Kick off TCP echo #1: park recv on server + send 32B from client
                  let _f0 = fill-payload tx-buf 1 in
                  let tok-srv-r = io-token-pack op-code-recv 1 1 in
                  let tok-cli-s = io-token-pack op-code-send 1 1 in
                  let _s1 = rx.reactor-submit (IoSqe op-code-recv 0 0 srv-fd.fd-raw (wrapping-i64-to-u64 srv-rx) 32 0 0 tok-srv-r.io-token-raw) in
                  let _s2 = rx.reactor-submit (IoSqe op-code-send 0 0 cli-fd.fd-raw (wrapping-i64-to-u64 tx-buf) 32 0 0 tok-cli-s.io-token-raw) in
                  let hdl = TickHandler {
                    tick-on-msg = (\msg ->
                      let _p = record-phase scratch ws 1 in
                      -- Allocate per-tick scratch from BumpArena (must be reset at Phase 5!)
                      let _a = worker-arena-alloc ws 128 64 in
                      let cur = raw-load-u64 (wrapping-add-i64 scratch 272) in
                      let _u = raw-store-u64 (wrapping-add-i64 scratch 272) (wrapping-add-u64 cur msg.msg-w0) in
                      -- Queue coalesced outbound reply to peer core 1 (flushed in Phase 4!)
                      let reply = msg-envelope-make 2 (core-id 0) (core-id 1) msg.msg-seq (wrapping-add-u64 msg.msg-w0 1) 0 0 0 0 0 in
                      let _q = worker-enqueue-msg ws (core-id 1) reply in
                      ()),
                    tick-on-cqe = (\cqe ->
                      let _p = record-phase scratch ws 2 in
                      let _a = worker-arena-alloc ws 256 64 in
                      let tok = completion-token cqe in
                      let kind = io-token-kind tok in
                      let slot = io-token-slot tok in
                      if eq-u8 kind op-code-recv && eq-u32 slot 1 && eq-i32 cqe.cqe-res 32 then
                        -- Server received 32B -> echo back to client and arm client recv
                        let tok-cli-r = io-token-pack op-code-recv 1 2 in
                        let tok-srv-s = io-token-pack op-code-send 1 2 in
                        let _r1 = rx.reactor-submit (IoSqe op-code-recv 0 0 cli-fd.fd-raw (wrapping-i64-to-u64 cli-rx) 32 0 0 tok-cli-r.io-token-raw) in
                        let _r2 = rx.reactor-submit (IoSqe op-code-send 0 0 srv-fd.fd-raw (wrapping-i64-to-u64 srv-rx) 32 0 0 tok-srv-s.io-token-raw) in
                        ()
                      else if eq-u8 kind op-code-recv && eq-u32 slot 2 && eq-i32 cqe.cqe-res 32 then
                        -- Client received 32B echo -> fold hash and start next echo up to 4
                        let ec = wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 scratch 256)) 1 in
                        let _ec = raw-store-u64 (wrapping-add-i64 scratch 256) ec in
                        let w0 = raw-load-u64 cli-rx in
                        let w1 = raw-load-u64 (wrapping-add-i64 cli-rx 8) in
                        let ph = raw-load-u64 (wrapping-add-i64 scratch 304) in
                        let ph2 = sim-mix-u64 (bit-xor-u64 (wrapping-add-u64 ph w0) w1) in
                        let _ph = raw-store-u64 (wrapping-add-i64 scratch 304) ph2 in
                        if lt-u64 ec 4 then
                          let next-seq = wrapping-add-u64 ec 1 in
                          let _fn = fill-payload tx-buf next-seq in
                          let tr = io-token-pack op-code-recv 1 1 in
                          let ts = io-token-pack op-code-send 1 1 in
                          let _n1 = rx.reactor-submit (IoSqe op-code-recv 0 0 srv-fd.fd-raw (wrapping-i64-to-u64 srv-rx) 32 0 0 tr.io-token-raw) in
                          let _n2 = rx.reactor-submit (IoSqe op-code-send 0 0 cli-fd.fd-raw (wrapping-i64-to-u64 tx-buf) 32 0 0 ts.io-token-raw) in
                          ()
                        else ()
                      else ()),
                    tick-on-timer = (\tok ->
                      let _p = record-phase scratch ws 3 in
                      let _a = worker-arena-alloc ws 64 64 in
                      let slot = u32-to-u64 (io-token-slot tok) in
                      let cur = raw-load-u64 (wrapping-add-i64 scratch 264) in
                      let _u = raw-store-u64 (wrapping-add-i64 scratch 264) (wrapping-add-u64 cur slot) in
                      if eq-u64 slot 999 then worker-request-local-shutdown ws else ()),
                    tick-on-flush = (\_ ->
                      let _p = record-phase scratch ws 4 in
                      ())
                  } in
                  let metrics = runtime-run-ops rx clk ws hdl 200 in
                  -- Verify BumpArena is reset to 0 bytes used at exit!
                  let arena-zero = eq-u64 (bump-arena-used (worker-arena ws)) 0 in
                  -- Verify peer core 1 received all 4 coalesced outbound replies!
                  let p1 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
                  let p2 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
                  let p3 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
                  let p4 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
                  let p5 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
                  let replies-ok = (pop-envelope-is-some p1 && eq-u64 p1.pop-w0 101 && pop-envelope-is-some p2 && eq-u64 p2.pop-w0 201 && pop-envelope-is-some p3 && eq-u64 p3.pop-w0 301 && pop-envelope-is-some p4 && eq-u64 p4.pop-w0 401 && not (pop-envelope-is-some p5)) in
                  let date-str = coarse-date-to-string (worker-clock-cache ws) in
                  let echo-cnt = raw-load-u64 (wrapping-add-i64 scratch 256) in
                  let timer-sum = raw-load-u64 (wrapping-add-i64 scratch 264) in
                  let msg-sum = raw-load-u64 (wrapping-add-i64 scratch 272) in
                  let order-ok = eq-u64 (raw-load-u64 (wrapping-add-i64 scratch 280)) 1 && arena-zero && replies-ok in
                  let phash = raw-load-u64 (wrapping-add-i64 scratch 304) in
                  let _fs = engine-dma-free scratch-u 4096 in
                  let _dw = worker-state-destroy ws in
                  let _fm = core-mesh-destroy mesh in
                  Some (SimRunResult
                    (sim-world-trace-hash w)
                    (sim-world-now-ns w)
                    (sim-world-events-processed w)
                    metrics.metric-ticks
                    metrics.metric-msgs-drained
                    metrics.metric-msgs-sent
                    metrics.metric-cqes-reaped
                    metrics.metric-timers-fired
                    metrics.metric-date-refreshes
                    metrics.metric-arena-resets
                    metrics.metric-arena-peak-bytes
                    echo-cnt
                    timer-sum
                    msg-sum
                    phash
                    date-str
                    order-ok)

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    case (run-dst-once 20261128, run-dst-once 20261128) of
      (Some r1, Some r2) ->
        if not (sim-run-eq r1 r2) then 1
        else if not r1.res-order-ok then 2
        else if ne-u64 r1.res-msgs-drained 4 || ne-u64 r1.res-msgs-sent 4 || ne-u64 r1.res-msg-sum 1000 then 3
        else if ne-u64 r1.res-timers 3 || ne-u64 r1.res-timer-sum 1202 then 4
        else if ne-u64 r1.res-echo-count 4 || eq-u64 r1.res-payload-hash 0 then 5
        -- Date cache must refresh at 14:00:00 and again at 14:00:01!
        else if lt-u64 r1.res-date-refreshes 2 || string-compare r1.res-date-str "Sat, 28 Nov 2026 14:00:01 GMT" /= 0 then 6
        -- Arena resets must equal ticks and peak bytes must reflect per-tick scratch usage!
        else if ne-u64 r1.res-arena-resets r1.res-ticks || eq-u64 r1.res-arena-peak 0 then 7
        -- Graceful drain must stop well before max-ticks (200)
        else if ge-u64 r1.res-ticks 200 then 8
        else
          -- Distinct seed must diverge in trace hash
          case run-dst-once 99999999 of
            None -> 9
            Some r3 ->
              if eq-u64 r1.res-trace r3.res-trace then 10
              else 0
      _ -> 11
"#;

    build_and_run_leak_checked("sim-deterministic-replay", src, &[], 0);
}

// =========================================================================
// 3. Live multi-core pinned OS worker test on macOS
//    (2 pinned OS workers communicating via CoreMesh SPSC + live Darwin
//     kqueue loopback TCP echo across workers + graceful drain shutdown)
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn engine_runtime_live_macos_multicore_pinned_workers_kqueue_tcp_and_spsc() {
    let bridge = c_fixture(
        "engine_runtime_worker_bridge",
        r#"
#include <stdint.h>

extern int64_t rt_pinned_worker_entry(int64_t slot_addr);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

/* KIND_CLOSURE = 2, bits 35..40 (`block_kinds.rs`: RC_BITS = 35); rc = 0 (RC_IMMORTAL). */
#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

static int64_t stub_rt_worker(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return rt_pinned_worker_entry(argv[0]);
}

static CinchStaticClosure clo_rt_worker;

int64_t rt_get_worker_closure(void) {
    clo_rt_worker.header = IMMORTAL_CLOSURE_HEADER;
    clo_rt_worker.code = (int64_t)(uintptr_t)stub_rt_worker;
    clo_rt_worker.arity = 1;
    clo_rt_worker.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_rt_worker.code;
}
"#,
    );

    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Data.Result
  import Engine.Mem.Arena
  import Engine.Mem.DmaPool
  import Engine.Reactor.Darwin
  import Engine.Ring.Spsc
  import Engine.Runtime
  import Engine.Timer
  import Engine.Types
  import Prim.Atomic

  foreign import ccall "rt_get_worker_closure" getWorkerClo :: Unit ->{IO} I64

  -- Shared test coordination slab (`cfg-user-ctx` points here, 4096B DMA):
  --   +0  : atomic client-done :: I64 (0 -> 1 once client verified all 64 TCP echoes & SPSC acks)
  --   +8  : atomic server-echoes :: I64
  --   +16 : atomic client-echoes :: I64
  --   +24 : atomic spsc-pongs :: I64
  --   +32 : atomic error-code :: I64

  total-echoes :: U64
  total-echoes = 64

  -- Per-worker local state slab layout (4096B DMA):
  --   +0   : listen-fd :: I32 (server only)
  --   +4   : peer-fd :: I32 (accepted conn on server, connected sock on client)
  --   +8   : state :: U32 (client: 0=wait-port, 1=connecting/send, 2=recv, 3=done)
  --   +12  : rx-got :: U32
  --   +16  : seq :: U64
  --   +24  : spsc-acks :: U64
  --   +32  : timer-fired :: U64
  --   +64  : tx-buf (32B)
  --   +128 : rx-buf (32B)

  fill-msg32 :: I64 -> U64 ->{MVar} Unit
  fill-msg32 buf seq =
    let _a = raw-store-u64 buf seq in
    let _b = raw-store-u64 (wrapping-add-i64 buf 8) (bit-xor-u64 (wrapping-mul-u64 seq 0x9E3779B97F4A7C15) 0xA5A55A5A11223344) in
    let _c = raw-store-u64 (wrapping-add-i64 buf 16) (wrapping-add-u64 seq 777) in
    raw-store-u64 (wrapping-add-i64 buf 24) (bit-not-u64 seq)

  verify-msg32 :: I64 -> U64 ->{MVar} Bool
  verify-msg32 buf seq =
    eq-u64 (raw-load-u64 buf) seq
      && eq-u64 (raw-load-u64 (wrapping-add-i64 buf 8)) (bit-xor-u64 (wrapping-mul-u64 seq 0x9E3779B97F4A7C15) 0xA5A55A5A11223344)
      && eq-u64 (raw-load-u64 (wrapping-add-i64 buf 16)) (wrapping-add-u64 seq 777)
      && eq-u64 (raw-load-u64 (wrapping-add-i64 buf 24)) (bit-not-u64 seq)

  send-all-nb :: Fd -> I64 -> U32 -> U32 -> U32 ->{IO, MVar} Bool
  send-all-nb fd buf sent total budget =
    if ge-u32 sent total then True
    else if eq-u32 budget 0 then False
    else
      let rem = wrapping-sub-u32 total sent in
      let nw = darwin-send-dma fd (wrapping-add-i64 buf (u32-to-i64 sent)) rem in
      if gt-i32 nw 0 then send-all-nb fd buf (wrapping-add-u32 sent (wrapping-i32-to-u32 nw)) total budget
      else if eq-i32 nw -35 || eq-i32 nw -4 then
        let _p = cpu-pause () in
        send-all-nb fd buf sent total (wrapping-sub-u32 budget 1)
      else False

  -- Core 0: Live Darwin kqueue TCP echo server + CoreMesh SPSC peer
  run-server-worker :: I64 -> WorkerState -> I64 ->{IO, MVar} I64
  run-server-worker slot-addr ws coord =
    case darwin-reactor-new 128 of
      Err _ -> -10
      Ok drx ->
        case darwin-tcp-listen-reuseport 0 64 of
          Err _ ->
            let _d = darwin-reactor-destroy drx in
            -11
          Ok (lfd, bound-port) ->
            let lslab-u = engine-dma-alloc 4096 4096 in
            if eq-u64 lslab-u 0 then
              let _cl = darwin-close-fd lfd in
              let _d = darwin-reactor-destroy drx in
              -12
            else
              let lslab = wrapping-u64-to-i64 lslab-u in
              let rx-buf = wrapping-add-i64 lslab 128 in
              let _i0 = raw-store-u32 lslab (wrapping-i32-to-u32 lfd.fd-raw) in
              let _i1 = raw-store-u32 (wrapping-add-i64 lslab 4) (wrapping-i32-to-u32 (-1)) in
              let _arm = darwin-reactor-arm-read drx lfd (io-token-pack op-code-accept 1 1) in
              let _f0 = darwin-reactor-poll drx 0 (worker-cqe-scratch-addr ws) 16 in
              let _rf = worker-set-reactor-fd ws (darwin-reactor-kq-fd drx) in
              -- Schedule a heartbeat timer 2ms out
              let now0 = div-u64 (darwin-clock-monotonic-ns ()) 1000000 in
              let _tm = timer-wheel-schedule (worker-timers ws) (wrapping-add-u64 now0 2) (io-token-pack 7 1 55) in
              -- Enqueue SPSC announcement carrying our live bound TCP port to Core 1!
              let announce = msg-envelope-make 100 (core-id 0) (core-id 1) 1 (u16-to-u64 bound-port) 0 0 0 0 0 in
              let _enq = worker-enqueue-msg ws (core-id 1) announce in
              let _rdy = runtime-boot-mark-ready slot-addr in
              let hdl = RawTickHandler {
                raw-on-msg = (\msg ->
                  let _a = worker-arena-alloc ws 64 64 in
                  if eq-u32 msg.msg-kind 200 then
                    -- Echo SPSC ping from Core 1 back as kind 201 pong
                    let pong = msg-envelope-make 201 (core-id 0) (core-id 1) msg.msg-seq (wrapping-add-u64 msg.msg-w0 1) 0 0 0 0 0 in
                    let _q = worker-enqueue-msg ws (core-id 1) pong in
                    ()
                  else ()),
                raw-on-poll = (\timeout ->
                  let cqe-buf = worker-cqe-scratch-addr ws in
                  let nev = darwin-reactor-poll drx (time-ns-value timeout) cqe-buf 32 in
                  -- Try accepting if no client connected yet
                  let cur-peer = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 lslab 4)) in
                  let peer-fd = (if lt-i32 cur-peer 0 then
                    case darwin-tcp-accept-nonblock lfd of
                      Ok cfd ->
                        let _sp = raw-store-u32 (wrapping-add-i64 lslab 4) (wrapping-i32-to-u32 cfd.fd-raw) in
                        let _ar = darwin-reactor-arm-read drx cfd (io-token-pack op-code-recv 1 2) in
                        cfd
                      Err _ -> fd-invalid ()
                    else fd-of cur-peer) in
                  -- Drain readable bytes on peer socket and echo 32-byte frames immediately
                  let _io = (if fd-is-valid peer-fd then
                    let got = raw-load-u32 (wrapping-add-i64 lslab 12) in
                    let rem = wrapping-sub-u32 32 got in
                    let nr = darwin-recv-dma peer-fd (wrapping-add-i64 rx-buf (u32-to-i64 got)) rem in
                    if gt-i32 nr 0 then
                      let got2 = wrapping-add-u32 got (wrapping-i32-to-u32 nr) in
                      if ge-u32 got2 32 then
                        let _clr = raw-store-u32 (wrapping-add-i64 lslab 12) 0 in
                        let ok-s = send-all-nb peer-fd rx-buf 0 32 10000 in
                        if ok-s then
                          let _inc = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 coord 8) 1 in
                          ()
                        else
                          atomic-store-release-i64 (wrapping-add-i64 coord 32) (-20)
                      else
                        raw-store-u32 (wrapping-add-i64 lslab 12) got2
                    else ()
                    else ()) in
                  if gt-i32 nev 0 then wrapping-i32-to-u32 nev else 0),
                raw-on-timer = (\tok ->
                  if eq-u32 (io-token-slot tok) 55 then
                    raw-store-u64 (wrapping-add-i64 lslab 32) 1
                  else ()),
                raw-on-flush = (\_ ->
                  let cl = darwin-reactor-changelist-len drx in
                  if gt-u32 cl 0 then
                    let _ = darwin-reactor-poll drx 0 (worker-cqe-scratch-addr ws) 16 in
                    cl
                  else 0),
                raw-mono-ns = (\_ -> time-ns-of (darwin-clock-monotonic-ns ())),
                raw-wall-ns = (\_ -> time-ns-of (darwin-clock-wall-ns ()))
              } in
              let _m = runtime-run-raw ws hdl 0 in
              let peer-raw = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 lslab 4)) in
              let _cp = if ge-i32 peer-raw 0 then darwin-close-fd (fd-of peer-raw) else IoOk in
              let _cl = darwin-close-fd lfd in
              let _dr = darwin-reactor-destroy drx in
              let tfire = raw-load-u64 (wrapping-add-i64 lslab 32) in
              let _fl = engine-dma-free lslab-u 4096 in
              let _pub = runtime-boot-publish-and-done slot-addr ws 0 0 (wrapping-u64-to-i64 tfire) in
              0

  -- Core 1: Live Darwin kqueue TCP echo client + CoreMesh SPSC peer
  run-client-worker :: I64 -> WorkerState -> I64 ->{IO, MVar} I64
  run-client-worker slot-addr ws coord =
    case darwin-reactor-new 128 of
      Err _ -> -30
      Ok drx ->
        let lslab-u = engine-dma-alloc 4096 4096 in
        if eq-u64 lslab-u 0 then
          let _d = darwin-reactor-destroy drx in
          -31
        else
          let lslab = wrapping-u64-to-i64 lslab-u in
          let tx-buf = wrapping-add-i64 lslab 64 in
          let rx-buf = wrapping-add-i64 lslab 128 in
          let _i0 = raw-store-u32 (wrapping-add-i64 lslab 4) (wrapping-i32-to-u32 (-1)) in
          let _i1 = raw-store-u32 (wrapping-add-i64 lslab 8) 0 in
          let _rf = worker-set-reactor-fd ws (darwin-reactor-kq-fd drx) in
          let _rdy = runtime-boot-mark-ready slot-addr in
          let hdl = RawTickHandler {
            raw-on-msg = (\msg ->
              let _a = worker-arena-alloc ws 64 64 in
              if eq-u32 msg.msg-kind 100 then
                -- Received server's bound TCP port via CoreMesh SPSC! Connect via Darwin TCP!
                let port = wrapping-u64-to-u16 msg.msg-w0 in
                case darwin-tcp-connect-nonblock ipv4-loopback port of
                  Err _ -> atomic-store-release-i64 (wrapping-add-i64 coord 32) -32
                  Ok cfd ->
                    let _sp = raw-store-u32 (wrapping-add-i64 lslab 4) (wrapping-i32-to-u32 cfd.fd-raw) in
                    let _ar = darwin-reactor-arm-read drx cfd (io-token-pack op-code-recv 1 10) in
                    let _aw = darwin-reactor-arm-write drx cfd (io-token-pack op-code-send 1 11) in
                    let _st = raw-store-u32 (wrapping-add-i64 lslab 8) 1 in
                    -- Also send an SPSC ping to Core 0!
                    let ping = msg-envelope-make 200 (core-id 1) (core-id 0) 1 41 0 0 0 0 0 in
                    let _q = worker-enqueue-msg ws (core-id 0) ping in
                    ()
              else if eq-u32 msg.msg-kind 201 then
                -- Received SPSC pong from Core 0!
                if eq-u64 msg.msg-w0 42 then
                  let _pa = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 coord 24) 1 in
                  raw-store-u64 (wrapping-add-i64 lslab 24) 1
                else atomic-store-release-i64 (wrapping-add-i64 coord 32) -33
              else ()),
            raw-on-poll = (\timeout ->
              let cqe-buf = worker-cqe-scratch-addr ws in
              let nev = darwin-reactor-poll drx (time-ns-value timeout) cqe-buf 32 in
              let st-mode = raw-load-u32 (wrapping-add-i64 lslab 8) in
              let peer-raw = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 lslab 4)) in
              let _step = (if eq-u32 st-mode 1 && ge-i32 peer-raw 0 then
                -- Send next 32B frame
                let seq = raw-load-u64 (wrapping-add-i64 lslab 16) in
                let _fm = fill-msg32 tx-buf seq in
                let nw = darwin-send-dma (fd-of peer-raw) tx-buf 32 in
                if eq-i32 nw 32 then
                  let _cg = raw-store-u32 (wrapping-add-i64 lslab 12) 0 in
                  raw-store-u32 (wrapping-add-i64 lslab 8) 2
                else if gt-i32 nw 0 then
                  let ok-rest = send-all-nb (fd-of peer-raw) tx-buf (wrapping-i32-to-u32 nw) 32 10000 in
                  if ok-rest then
                    let _cg = raw-store-u32 (wrapping-add-i64 lslab 12) 0 in
                    raw-store-u32 (wrapping-add-i64 lslab 8) 2
                  else atomic-store-release-i64 (wrapping-add-i64 coord 32) -34
                else ()
              else if eq-u32 st-mode 2 && ge-i32 peer-raw 0 then
                -- Receive 32B echo frame and verify bit-exact contents
                let got = raw-load-u32 (wrapping-add-i64 lslab 12) in
                let rem = wrapping-sub-u32 32 got in
                let nr = darwin-recv-dma (fd-of peer-raw) (wrapping-add-i64 rx-buf (u32-to-i64 got)) rem in
                if gt-i32 nr 0 then
                  let got2 = wrapping-add-u32 got (wrapping-i32-to-u32 nr) in
                  if ge-u32 got2 32 then
                    let seq = raw-load-u64 (wrapping-add-i64 lslab 16) in
                    if verify-msg32 rx-buf seq then
                      let next-seq = wrapping-add-u64 seq 1 in
                      let _ns = raw-store-u64 (wrapping-add-i64 lslab 16) next-seq in
                      let _ce = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 coord 16) 1 in
                      if ge-u64 next-seq total-echoes then
                        raw-store-u32 (wrapping-add-i64 lslab 8) 3
                      else
                        raw-store-u32 (wrapping-add-i64 lslab 8) 1
                    else atomic-store-release-i64 (wrapping-add-i64 coord 32) -35
                  else
                    raw-store-u32 (wrapping-add-i64 lslab 12) got2
                else ()
              else if eq-u32 st-mode 3 then
                -- Once both all 64 TCP echoes AND SPSC ping-pong succeeded, signal coordinator!
                if eq-u64 (raw-load-u64 (wrapping-add-i64 lslab 24)) 1 then
                  let _d = atomic-store-release-i64 coord 1 in
                  raw-store-u32 (wrapping-add-i64 lslab 8) 4
                else ()
              else ()) in
              if gt-i32 nev 0 then wrapping-i32-to-u32 nev else 0),
            raw-on-timer = (\_ -> ()),
            raw-on-flush = (\_ ->
              let cl = darwin-reactor-changelist-len drx in
              if gt-u32 cl 0 then
                let _ = darwin-reactor-poll drx 0 (worker-cqe-scratch-addr ws) 16 in
                cl
              else 0),
            raw-mono-ns = (\_ -> time-ns-of (darwin-clock-monotonic-ns ())),
            raw-wall-ns = (\_ -> time-ns-of (darwin-clock-wall-ns ()))
          } in
          let _m = runtime-run-raw ws hdl 0 in
          let peer-raw = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 lslab 4)) in
          let _cp = if ge-i32 peer-raw 0 then darwin-close-fd (fd-of peer-raw) else IoOk in
          let _dr = darwin-reactor-destroy drx in
          let done-echoes = raw-load-u64 (wrapping-add-i64 lslab 16) in
          let _fl = engine-dma-free lslab-u 4096 in
          let _pub = runtime-boot-publish-and-done slot-addr ws 0 0 (wrapping-u64-to-i64 done-echoes) in
          0

  worker-entry :: I64 ->{IO, MVar} I64
  worker-entry slot-addr =
    let now-ms = div-u64 (darwin-clock-monotonic-ns ()) 1000000 in
    case runtime-boot-init slot-addr now-ms of
      None -> -1
      Some ws ->
        let coord = runtime-boot-user-ctx slot-addr in
        let cid = core-id-value (worker-core-id ws) in
        let rc = if eq-u16 cid 0 then run-server-worker slot-addr ws coord else run-client-worker slot-addr ws coord in
        let _dw = worker-state-destroy ws in
        rc

  foreign export ccall "rt_pinned_worker_entry" worker-entry :: I64 ->{IO, MVar} I64

  wait-coord-done :: I64 -> U64 ->{MVar} Bool
  wait-coord-done coord rem =
    if ne-i64 (atomic-load-acquire-i64 coord) 0 then True
    else if ne-i64 (atomic-load-acquire-i64 (wrapping-add-i64 coord 32)) 0 then False
    else if eq-u64 rem 0 then False
    else
      let _p = cpu-pause () in
      wait-coord-done coord (wrapping-sub-u64 rem 1)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let coord-u = engine-dma-alloc 4096 4096 in
    if eq-u64 coord-u 0 then 1
    else
      let coord = wrapping-u64-to-i64 coord-u in
      let cfg = WorkerConfig 2 4096 8 32 32 32 256 4 16384 64 500000 coord in
      let clo = wrapping-i64-to-u64 (getWorkerClo ()) in
      case runtime-spawn-workers cfg clo of
        None ->
          let _fc = engine-dma-free coord-u 4096 in
          2
        Some rt ->
          let rdy = runtime-wait-all-ready rt 100000000 in
          let done-ok = if rdy then wait-coord-done coord 200000000 else False in
          let _sh = runtime-request-shutdown rt in
          let join-rc = runtime-join-all rt in
          let r0 = runtime-worker-result rt (core-id 0) in
          let r1 = runtime-worker-result rt (core-id 1) in
          let m0 = runtime-worker-slot-metrics rt (core-id 0) in
          let m1 = runtime-worker-slot-metrics rt (core-id 1) in
          let done-cnt = runtime-done-count rt in
          let srv-e = atomic-load-acquire-i64 (wrapping-add-i64 coord 8) in
          let cli-e = atomic-load-acquire-i64 (wrapping-add-i64 coord 16) in
          let pongs = atomic-load-acquire-i64 (wrapping-add-i64 coord 24) in
          let err-c = atomic-load-acquire-i64 (wrapping-add-i64 coord 32) in
          let dest-err = runtime-destroy rt in
          let _fc = engine-dma-free coord-u 4096 in
          if not rdy then 3
          else if not done-ok || ne-i64 err-c 0 then 4
          else if ne-i64 join-rc 0 || ne-i64 r0 0 || ne-i64 r1 0 then 5
          else if ne-u32 done-cnt 2 then 6
          else if ne-i64 srv-e 64 || ne-i64 cli-e 64 || ne-i64 pongs 1 then 7
          else if eq-u64 m0.metric-ticks 0 || eq-u64 m1.metric-ticks 0 then 8
          else if ne-u64 m0.metric-arena-resets m0.metric-ticks || ne-u64 m1.metric-arena-resets m1.metric-ticks then 9
          else if eq-u64 m0.metric-msgs-drained 0 || eq-u64 m1.metric-msgs-drained 0 then 10
          else if eq-u64 m0.metric-msgs-sent 0 || eq-u64 m1.metric-msgs-sent 0 then 11
          else if eq-u64 m0.metric-cqes-reaped 0 || eq-u64 m1.metric-cqes-reaped 0 then 12
          else if not (io-error-is-ok dest-err) then 13
          else 0
"#;

    build_and_run_leak_checked("live-macos-pinned-workers", src, &[bridge], 0);
}

// =========================================================================
// 4. Steady-state 20,000-tick hot loop asserting:
//    - engine-thread-alloc-count () == 0
//    - engine-thread-atomic-rc-count () == 0
//    - CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)
//    across BOTH `runtime-run-ops` (ReactorOps + ClockOps) AND `runtime-run-raw`!
// =========================================================================

#[test]
fn engine_runtime_20000_ticks_zero_alloc_zero_atomic_rc_and_zero_leak() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Mutable.Var
  import Data.Option
  import Engine.Mem.Arena
  import Engine.Ring.Spsc
  import Engine.Runtime
  import Engine.Timer
  import Engine.Types
  import Prim.Atomic

  -- Region-backed zero-allocation mock reactor & clock state (pre-allocated once
  -- before the 20,000-tick loop; in-place scalar `Var r` updates perform 0 heap
  -- allocations and 0 atomic RC ops!).
  record ZeroAllocOpsState (r :: Region) where
    za-mono :: Var r U64
    za-wall :: Var r U64
    za-cqe-pending :: Var r U32
    za-sqe-pending :: Var r U32

  -- Non-inlined scalar helper consuming one pending CQE without inlining `Var`
  -- constructor switches inside a `{ i64, i64 }` `Completion`-returning function.
  #[inline(never)]
  za-take-pending :: forall (r :: Region). ZeroAllocOpsState r ->{Mut r} U32
  za-take-pending zs =
    let p = var-read zs.za-cqe-pending in
    if eq-u32 p 0 then 0
    else
      let _ = var-write zs.za-cqe-pending (wrapping-sub-u32 p 1) in
      1

  -- Named helper for `reactor-pop-cqe` returning `Completion` (`{ i64, i64 }`).
  za-pop-cqe-impl :: forall (r :: Region). ZeroAllocOpsState r ->{IO r, Mut r} Completion
  za-pop-cqe-impl zs =
    if eq-u32 (za-take-pending zs) 0 then completion-empty ()
    else completion-make (io-token-pack op-code-recv 1 7) 64 0

  -- Build a 0-alloc `ReactorOps r` over `ZeroAllocOpsState r`.
  make-zero-alloc-reactor-ops :: forall (r :: Region). ZeroAllocOpsState r -> ReactorOps r
  make-zero-alloc-reactor-ops zs = ReactorOps {
    reactor-submit = (\_ ->
      let n = var-read zs.za-sqe-pending in
      let _ = var-write zs.za-sqe-pending (wrapping-add-u32 n 1) in
      IoOk),
    reactor-flush = (\_ ->
      let n = var-read zs.za-sqe-pending in
      let _ = var-write zs.za-sqe-pending 0 in
      wrapping-u32-to-i32 n),
    reactor-wait = (\_ ->
      let t = var-read zs.za-mono in
      let w = var-read zs.za-wall in
      let _1 = var-write zs.za-mono (wrapping-add-u64 t 1000000) in
      let _2 = var-write zs.za-wall (wrapping-add-u64 w 1000000) in
      let _3 = var-write zs.za-cqe-pending 1 in
      1),
    reactor-cqe-count = (\_ -> var-read zs.za-cqe-pending),
    reactor-pop-cqe = (\_ -> za-pop-cqe-impl zs),
    reactor-close = (\_ -> IoOk)
  }

  -- Build a 0-alloc `ClockOps r` over `ZeroAllocOpsState r`.
  make-zero-alloc-clock-ops :: forall (r :: Region). ZeroAllocOpsState r -> ClockOps r
  make-zero-alloc-clock-ops zs = ClockOps {
    clock-mono-ns = (\_ -> time-ns-of (var-read zs.za-mono)),
    clock-wall-ns = (\_ -> time-ns-of (var-read zs.za-wall))
  }

  -- Top-level zero-alloc TickHandler callbacks (reading scratch ptr from `worker-user-ctx`).
  hot-on-msg :: forall (r :: Region). WorkerState -> MsgEnvelope ->{IO r, Mut r, MVar} Unit
  hot-on-msg ws _ =
    let st = worker-user-ctx ws in
    let _a = worker-arena-alloc ws 64 64 in
    let c = raw-load-u64 (wrapping-add-i64 st 24) in
    raw-store-u64 (wrapping-add-i64 st 24) (wrapping-add-u64 c 1)

  hot-on-cqe :: forall (r :: Region). WorkerState -> Completion ->{IO r, Mut r, MVar} Unit
  hot-on-cqe ws _ =
    let st = worker-user-ctx ws in
    let _a = worker-arena-alloc ws 128 64 in
    let c = raw-load-u64 (wrapping-add-i64 st 32) in
    let _u = raw-store-u64 (wrapping-add-i64 st 32) (wrapping-add-u64 c 1) in
    -- Schedule a timer due on tick `c + 1` ms (fires in Phase 3 of this very tick!)
    let _t = timer-wheel-schedule (worker-timers ws) (wrapping-add-u64 c 1) (io-token-pack 3 1 42) in
    ()

  hot-on-timer :: forall (r :: Region). WorkerState -> IoToken ->{IO r, Mut r, MVar} Unit
  hot-on-timer ws _ =
    let st = worker-user-ctx ws in
    let c = raw-load-u64 (wrapping-add-i64 st 40) in
    raw-store-u64 (wrapping-add-i64 st 40) (wrapping-add-u64 c 1)

  hot-on-flush :: forall (r :: Region). WorkerState -> Unit ->{IO r, Mut r, MVar} Unit
  hot-on-flush ws _ =
    -- Stage 1 outbound SPSC message to core 1 (flushed in Phase 4) and bounce 1 inbound
    -- from core 1 -> core 0 so every tick exercises BOTH Phase 1 drain and Phase 4 flush!
    let m = msg-envelope-make 5 (core-id 0) (core-id 1) 1 99 0 0 0 0 0 in
    let _q = worker-enqueue-msg ws (core-id 1) m in
    -- Also drain the peer ring on core 1's side and push 1 back so rings never fill!
    let _pop1 = core-mesh-recv-unboxed (worker-mesh ws) (core-id 0) (core-id 1) in
    let back = msg-envelope-make 6 (core-id 1) (core-id 0) 1 88 0 0 0 0 0 in
    let _push0 = core-mesh-send (worker-mesh ws) (core-id 1) (core-id 0) back in
    ()

  -- Top-level zero-alloc RawTickHandler callbacks for `runtime-run-raw` 20k check.
  raw-hot-msg :: WorkerState -> MsgEnvelope ->{IO, MVar} Unit
  raw-hot-msg ws _ =
    let st = worker-user-ctx ws in
    let _a = worker-arena-alloc ws 64 64 in
    let c = raw-load-u64 (wrapping-add-i64 st 24) in
    raw-store-u64 (wrapping-add-i64 st 24) (wrapping-add-u64 c 1)

  raw-hot-poll :: WorkerState -> TimeNs ->{IO, MVar} U32
  raw-hot-poll ws _ =
    let st = worker-user-ctx ws in
    let t = raw-load-u64 st in
    let w = raw-load-u64 (wrapping-add-i64 st 8) in
    let t2 = wrapping-add-u64 t 1000000 in
    let _1 = raw-store-u64 st t2 in
    let _2 = raw-store-u64 (wrapping-add-i64 st 8) (wrapping-add-u64 w 1000000) in
    let _a = worker-arena-alloc ws 128 64 in
    let _tm = timer-wheel-schedule (worker-timers ws) (div-u64 t2 1000000) (io-token-pack 3 1 42) in
    1

  raw-hot-timer :: WorkerState -> IoToken ->{IO, MVar} Unit
  raw-hot-timer ws _ =
    let st = worker-user-ctx ws in
    let c = raw-load-u64 (wrapping-add-i64 st 40) in
    raw-store-u64 (wrapping-add-i64 st 40) (wrapping-add-u64 c 1)

  raw-hot-flush :: WorkerState -> Unit ->{IO, MVar} U32
  raw-hot-flush ws _ =
    let m = msg-envelope-make 5 (core-id 0) (core-id 1) 1 99 0 0 0 0 0 in
    let _q = worker-enqueue-msg ws (core-id 1) m in
    let _p = core-mesh-recv-unboxed (worker-mesh ws) (core-id 0) (core-id 1) in
    let back = msg-envelope-make 6 (core-id 1) (core-id 0) 1 88 0 0 0 0 0 in
    let _b = core-mesh-send (worker-mesh ws) (core-id 1) (core-id 0) back in
    1

  raw-hot-mono :: WorkerState -> Unit ->{IO, MVar} TimeNs
  raw-hot-mono ws _ = time-ns-of (raw-load-u64 (worker-user-ctx ws))

  raw-hot-wall :: WorkerState -> Unit ->{IO, MVar} TimeNs
  raw-hot-wall ws _ = time-ns-of (raw-load-u64 (wrapping-add-i64 (worker-user-ctx ws) 8))

  test-ops-twentyk :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  test-ops-twentyk _ =
    let st-u = engine-dma-alloc 4096 4096 in
    if eq-u64 st-u 0 then 1
    else
      let st = wrapping-u64-to-i64 st-u in
      let _w0 = raw-store-u64 st 0 in
      let _w1 = raw-store-u64 (wrapping-add-i64 st 8) 1795874400000000000 in
      case core-mesh-new 2 64 of
        None ->
          let _f = engine-dma-free st-u 4096 in
          2
        Some mesh ->
          let cfg = WorkerConfig 2 4096 4 16 16 16 256 0 4096 64 1000000 st in
          case worker-state-new (core-id 0) cfg mesh 0 0 of
            None ->
              let _fm = core-mesh-destroy mesh in
              let _f = engine-dma-free st-u 4096 in
              3
            Some ws ->
              -- Construct ops & handler ONCE before resetting thread telemetry counters!
              let zs = ZeroAllocOpsState (var-new (0 :: U64)) (var-new (1795874400000000000 :: U64)) (var-new (0 :: U32)) (var-new (0 :: U32)) in
              let rx = make-zero-alloc-reactor-ops zs in
              let clk = make-zero-alloc-clock-ops zs in
              let hdl = TickHandler (hot-on-msg ws) (hot-on-cqe ws) (hot-on-timer ws) (hot-on-flush ws) in
              -- Prime 1 inbound message so tick #1 drains immediately
              let _p0 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 6 (core-id 1) (core-id 0) 0 88 0 0 0 0 0) in
              -- Reset per-thread allocation & atomic RC counters right before 20,000-tick hot loop!
              let _rst = engine-thread-counters-reset () in
              let metrics = runtime-run-ops rx clk ws hdl 20000 in
              let allocs = engine-thread-alloc-count () in
              let atomic-rcs = engine-thread-atomic-rc-count () in
              let arena-used = bump-arena-used (worker-arena ws) in
              let msg-c = raw-load-u64 (wrapping-add-i64 st 24) in
              let cqe-c = raw-load-u64 (wrapping-add-i64 st 32) in
              let tmr-c = raw-load-u64 (wrapping-add-i64 st 40) in
              let _dw = worker-state-destroy ws in
              let _fm = core-mesh-destroy mesh in
              let _fs = engine-dma-free st-u 4096 in
              if ne-u64 metrics.metric-ticks 20000 then 10
              else if ne-u64 metrics.metric-arena-resets 20000 || ne-u64 arena-used 0 then 11
              else if ne-u64 metrics.metric-cqes-reaped 20000 || ne-u64 cqe-c 20000 then 12
              else if ne-u64 metrics.metric-timers-fired 20000 || ne-u64 tmr-c 20000 then 13
              else if ne-u64 metrics.metric-msgs-drained 20000 || ne-u64 msg-c 20000 then 14
              else if ne-u64 metrics.metric-msgs-sent 20000 || ne-u64 metrics.metric-flushes 20000 then 15
              -- 20,000 ticks * 1ms = 20 seconds -> 21 RFC 9110 Date refreshes (t=0s..20s)
              else if lt-u64 metrics.metric-date-refreshes 20 then 16
              else if ne-u64 allocs 0 then 17
              else if ne-u64 atomic-rcs 0 then 18
              else 0

  test-raw-twentyk :: Unit ->{IO, MVar} Int
  test-raw-twentyk _ =
    let st-u = engine-dma-alloc 4096 4096 in
    if eq-u64 st-u 0 then 20
    else
      let st = wrapping-u64-to-i64 st-u in
      let _w0 = raw-store-u64 st 0 in
      let _w1 = raw-store-u64 (wrapping-add-i64 st 8) 1795874400000000000 in
      case core-mesh-new 2 64 of
        None ->
          let _f = engine-dma-free st-u 4096 in
          21
        Some mesh ->
          let cfg = WorkerConfig 2 4096 4 16 16 16 256 0 4096 64 1000000 st in
          case worker-state-new (core-id 0) cfg mesh 0 0 of
            None ->
              let _fm = core-mesh-destroy mesh in
              let _f = engine-dma-free st-u 4096 in
              22
            Some ws ->
              let hdl = RawTickHandler (raw-hot-msg ws) (raw-hot-poll ws) (raw-hot-timer ws) (raw-hot-flush ws) (raw-hot-mono ws) (raw-hot-wall ws) in
              let _p0 = core-mesh-send mesh (core-id 1) (core-id 0) (msg-envelope-make 6 (core-id 1) (core-id 0) 0 88 0 0 0 0 0) in
              let _rst = engine-thread-counters-reset () in
              let metrics = runtime-run-raw ws hdl 20000 in
              let allocs = engine-thread-alloc-count () in
              let atomic-rcs = engine-thread-atomic-rc-count () in
              let _dw = worker-state-destroy ws in
              let _fm = core-mesh-destroy mesh in
              let _fs = engine-dma-free st-u 4096 in
              if ne-u64 metrics.metric-ticks 20000 then 30
              else if ne-u64 metrics.metric-arena-resets 20000 then 31
              else if ne-u64 metrics.metric-cqes-reaped 20000 then 32
              else if ne-u64 metrics.metric-timers-fired 20000 then 33
              else if ne-u64 metrics.metric-msgs-drained 20000 || ne-u64 metrics.metric-msgs-sent 20000 then 34
              else if ne-u64 allocs 0 then 35
              else if ne-u64 atomic-rcs 0 then 36
              else 0

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    let r1 = test-ops-twentyk () in
    if r1 /= 0 then r1
    else test-raw-twentyk ()
"#;

    build_and_run_leak_checked("steady-state-20k-zero-alloc", src, &[], 0);
}
