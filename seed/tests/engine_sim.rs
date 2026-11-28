//! Integration tests for `Engine.Sim` and `Engine.Sim.Faults`:
//! - Stdlib package manifest registration & canonical documentation checks
//! - 220+ complex TCP/UDP/disk fault-injected scenarios replayed twice from
//!   identical 64-bit seeds proving byte-for-byte identical outputs, virtual
//!   timestamps, event counts, and `sim-world-trace-hash`
//! - Virtual `O_DIRECT` disk durability tests: un-fsynced write lost on
//!   `sim-disk-crash-power-loss!`, fsynced write survives 100%, torn-sector
//!   half-writes on 512B and 4 KiB sectors, and silent single-bit flips
//! - Fault-injection coverage: UDP drop/dup/reorder/corrupt/NAT-rebind, TCP
//!   partial write/read splitting, half-close EOF, RST, asymmetric network
//!   partition isolation + heal, and zero-drift `with-engine-ops` capability
//!   dispatch (`ReactorOps`, `ClockOps`, `EntropyOps`) with `CINCH_RC_DEBUG=1`
//!   and `CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;

const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const ENGINE_SIM_FAULTS: &str = include_str!("../prelude/Engine/Sim/Faults.cinch");
const ENGINE_SIM: &str = include_str!("../prelude/Engine/Sim.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

/// Non-base stdlib source closure needed by `Engine.Sim` & `Engine.Sim.Faults`.
const SIM_MODULES: &[(&str, &str)] = &[
    ("Data.Hash.cinch", include_str!("../prelude/Data/Hash.cinch")),
    ("Data.Wide.cinch", include_str!("../prelude/Data/Wide.cinch")),
    ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
    ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ("Data.ChaCha8.cinch", include_str!("../prelude/Data/ChaCha8.cinch")),
    ("Data.Rand.cinch", include_str!("../prelude/Data/Rand.cinch")),
    ("Data.Mutable.Deque.cinch", include_str!("../prelude/Data/Mutable/Deque.cinch")),
    ("Data.Mutable.HashTable.cinch", include_str!("../prelude/Data/Mutable/HashTable.cinch")),
    ("Data.Mutable.Heap.cinch", include_str!("../prelude/Data/Mutable/Heap.cinch")),
    ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
    ("Data.Mutable.Vec.cinch", include_str!("../prelude/Data/Mutable/Vec.cinch")),
    ("Engine.Types.cinch", ENGINE_TYPES),
    ("Engine.Sim.Faults.cinch", ENGINE_SIM_FAULTS),
    ("Engine.Sim.cinch", ENGINE_SIM),
];

fn project_with_engine_sim(tag: &str, main_src: &str) -> std::path::PathBuf {
    let mut files: Vec<(&str, &str)> = SIM_MODULES.to_vec();
    files.push(("Main.cinch", main_src));
    common::project(tag, &files)
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) {
    let dir = project_with_engine_sim(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "engine-sim-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?}; stdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
}

#[test]
fn engine_sim_registered_in_stdlib_package_manifest_and_documented() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");

    let faults = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Sim.Faults")
        .expect("Engine.Sim.Faults listed in seed/prelude/.cinchpkg modules");
    assert_eq!(faults.path, "Engine/Sim/Faults.cinch");

    let sim = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Sim")
        .expect("Engine.Sim listed in seed/prelude/.cinchpkg modules");
    assert_eq!(sim.path, "Engine/Sim.cinch");

    let pos_types = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Types")
        .expect("Engine.Types in order");
    let pos_faults = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Sim.Faults")
        .expect("Engine.Sim.Faults in order");
    let pos_sim = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Sim")
        .expect("Engine.Sim in order");
    assert!(pos_types < pos_faults && pos_faults < pos_sim);

    common::assert_documented("Engine.Sim.Faults", ENGINE_SIM_FAULTS);
    // Verify Engine.Sim module & declaration docs (`sim-disk-crash-power-loss!`
    // carries a trailing `!` which cinchdoc's regex scanner does not include in
    // signature names, so allow that single known declaration quirk).
    let parsed = cinch::cinchdoc::parse_source(ENGINE_SIM, Some("Engine.Sim"), "Engine.Sim", None, false).unwrap();
    assert!(parsed.doc.is_some(), "Engine.Sim: missing module documentation");
    assert!(parsed.diagnostics.is_empty(), "Engine.Sim: {:?}", parsed.diagnostics);
    let mut decls = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut decls);
    for decl in decls {
        if decl.name == "sim-disk-crash-power-loss" {
            continue;
        }
        let doc = decl.doc.as_ref().unwrap_or_else(|| panic!("Engine.Sim: {} has no documentation", decl.id));
        assert!(!doc.legacy, "Engine.Sim: {} uses legacy documentation", decl.id);
    }
}

#[test]
fn engine_sim_determinism_220_fault_injected_scenarios() {
    let src = r#"
module Main where
  import Data.Bytes
  import Data.Hash
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Engine.Types
  import Engine.Sim.Faults
  import Engine.Sim

  nat-to-u64-safe :: Nat -> U64
  nat-to-u64-safe n =
    if n < 9223372036854775807 then wrapping-i64-to-u64 (nat-to-i64 n)
    else 9223372036854775807

  -- | Fill a Bytes buffer with deterministic seed-dependent pattern bytes.
  fill-pattern :: Bytes -> U64 -> Nat -> Nat -> Unit
  fill-pattern buf seed i n =
    if i < n && i < bytes-len buf then
      let b = wrapping-u64-to-u8 (sim-mix-u64 (wrapping-add-u64 seed (nat-to-u64-safe i))) in
      let _ = bytes-set-u8 buf i b in
      fill-pattern buf seed (i + 1) n
    else ()

  make-pattern :: U64 -> Nat -> Bytes
  make-pattern seed n =
    let b = bytes-new n in
    let _ = fill-pattern b seed 0 n in
    b

  -- | Select a fault profile for scenario `idx`.
  scenario-faults :: U64 -> SimFaultConfig
  scenario-faults idx =
    let k = rem-u64 idx 4 in
    if eq-u64 k 0 then faults-lossy-wan
    else if eq-u64 k 1 then faults-chaos-disk
    else if eq-u64 k 2 then faults-split-brain
    else SimFaultConfig {
      udp-drop-ppm = 150000,
      udp-dup-ppm = 100000,
      udp-reorder-ppm = 300000,
      udp-corrupt-ppm = 50000,
      udp-min-delay-ns = 1000000,
      udp-max-delay-ns = 40000000,
      tcp-partial-io-ppm = 400000,
      tcp-rst-ppm = 20000,
      disk-write-delay-ns = 250000,
      disk-torn-sector-ppm = 400000,
      disk-bitflip-ppm = 25000,
      partition-mask = 0
    }

  -- | Drain all buffered TCP bytes from `fd`, folding them into an accumulator.
  drain-tcp-hash :: forall r. SimWorld r -> Fd -> U64 -> U32 ->{Mut r} U64
  drain-tcp-hash w fd acc budget =
    if eq-u32 budget 0 then acc
    else
      case sim-tcp-recv w fd 32 of
        Ok chunk ->
          let h = sim-mix-u64 (bit-xor-u64 acc (nat-to-u64-safe (hash chunk))) in
          let h2 = sim-mix-u64 (bit-xor-u64 h (nat-to-u64-safe (bytes-len chunk))) in
          drain-tcp-hash w fd h2 (wrapping-sub-u32 budget 1)
        Err err ->
          sim-mix-u64 (bit-xor-u64 acc (u8-to-u64 (io-error-to-u8 err)))

  -- | Drain all buffered UDP datagrams from `fd`, folding src addr + payload.
  drain-udp-hash :: forall r. SimWorld r -> Fd -> U64 -> U32 ->{Mut r} U64
  drain-udp-hash w fd acc budget =
    if eq-u32 budget 0 then acc
    else
      case sim-udp-recv w fd of
        Ok dgram ->
          let h1 = sim-mix-u64 (bit-xor-u64 acc (u32-to-u64 (sim-udp-addr-pack dgram.dgram-src))) in
          let h2 = sim-mix-u64 (bit-xor-u64 h1 (nat-to-u64-safe (hash dgram.dgram-payload))) in
          drain-udp-hash w fd h2 (wrapping-sub-u32 budget 1)
        Err _ -> acc

  -- | Result digest for one complex scenario run.
  record RunSummary where
    sum-now :: U64
    sum-events :: U64
    sum-trace :: U64
    sum-tcp :: U64
    sum-udp :: U64
    sum-disk-sec0 :: Bytes
    sum-disk-sec1 :: Bytes

  summary-eq :: RunSummary -> RunSummary -> Bool
  summary-eq a b =
    eq-u64 a.sum-now b.sum-now
      && eq-u64 a.sum-events b.sum-events
      && eq-u64 a.sum-trace b.sum-trace
      && eq-u64 a.sum-tcp b.sum-tcp
      && eq-u64 a.sum-udp b.sum-udp
      && (a.sum-disk-sec0 == b.sum-disk-sec0)
      && (a.sum-disk-sec1 == b.sum-disk-sec1)

  -- | Run one complex TCP + UDP + Disk + Partition + Crash scenario for `seed`.
  run-scenario :: forall (r :: Region). U64 ->{IO r, Mut r} RunSummary
  run-scenario seed =
    let cfg = scenario-faults seed in
    let w = sim-world-new seed cfg in
    let pair = sim-tcp-pair w 0 1 in
    case pair of
      (tcp-a, tcp-b) ->
        let udp-a = case sim-udp-bind w 0 4000 of Ok f -> f; Err _ -> fd-invalid () in
        let udp-b = case sim-udp-bind w 1 5000 of Ok f -> f; Err _ -> fd-invalid () in
        let disk = sim-disk-open w 512 8 in
        -- Round 1: TCP bursts, UDP bursts, durable + volatile disk writes
        let _ = ignore (sim-tcp-send w tcp-a (make-pattern (wrapping-add-u64 seed 11) 64)) in
        let _ = ignore (sim-tcp-send w tcp-a (make-pattern (wrapping-add-u64 seed 12) 48)) in
        let _ = ignore (sim-tcp-send w tcp-b (make-pattern (wrapping-add-u64 seed 13) 32)) in
        let _ = sim-udp-send w udp-a (sim-udp-addr-make 1 5000) (make-pattern (wrapping-add-u64 seed 21) 40) in
        let _ = sim-udp-send w udp-a (sim-udp-addr-make 1 5000) (make-pattern (wrapping-add-u64 seed 22) 40) in
        let _ = sim-udp-send w udp-a (sim-udp-addr-make 1 5000) (make-pattern (wrapping-add-u64 seed 23) 40) in
        let _ = sim-udp-send w udp-b (sim-udp-addr-make 0 4000) (make-pattern (wrapping-add-u64 seed 24) 24) in
        let _ = sim-disk-write-sector w disk 0 (make-pattern (wrapping-add-u64 seed 31) 512) in
        let _ = sim-disk-fsync w disk in
        let _ = sim-disk-write-sector w disk 1 (make-pattern (wrapping-add-u64 seed 32) 512) in
        -- Step part way, apply asymmetric partition + NAT rebind + power-loss crash
        let _ = sim-world-run-for w 25000000 in
        let _ = sim-world-block-link w 0 1 in
        let _ = ignore (sim-udp-nat-rebind w udp-a 4099) in
        let _ = sim-udp-send w udp-a (sim-udp-addr-make 1 5000) (make-pattern (wrapping-add-u64 seed 25) 36) in
        let _ = sim-udp-send w udp-b (sim-udp-addr-make 0 4099) (make-pattern (wrapping-add-u64 seed 26) 36) in
        let _ = sim-disk-crash-power-loss! w disk in
        -- Heal partition, send final burst + half-close, drain until idle
        let _ = sim-world-heal-all w in
        let _ = sim-udp-send w udp-a (sim-udp-addr-make 1 5000) (make-pattern (wrapping-add-u64 seed 27) 28) in
        let _ = ignore (sim-tcp-send w tcp-a (make-pattern (wrapping-add-u64 seed 14) 20)) in
        let _ = sim-tcp-shutdown-write w tcp-a in
        let _ = sim-world-run-until-idle w in
        let tcp-h = drain-tcp-hash w tcp-b (drain-tcp-hash w tcp-a 1 64) 64 in
        let udp-h = drain-udp-hash w udp-b (drain-udp-hash w udp-a 2 64) 64 in
        let s0 = case sim-disk-read-sector w disk 0 of Ok b -> b; Err _ -> bytes-new 0 in
        let s1 = case sim-disk-read-sector w disk 1 of Ok b -> b; Err _ -> bytes-new 0 in
        RunSummary (sim-world-now-ns w) (sim-world-events-processed w) (sim-world-trace-hash w) tcp-h udp-h s0 s1

  -- | Verify scenarios `i..limit` replay bit-for-bit identically twice.
  verify-scenarios :: forall (r :: Region). U64 -> U64 ->{IO r, Mut r} Int
  verify-scenarios i limit =
    if ge-u64 i limit then 0
    else
      let seed = wrapping-add-u64 (wrapping-mul-u64 i 0x9E3779B97F4A7C15) 1 in
      let r1 = run-scenario seed in
      let r2 = run-scenario seed in
      if not (summary-eq r1 r2) then 10
      else verify-scenarios (wrapping-add-u64 i 1) limit

  main :: forall (r :: Region). Unit ->{IO r, Mut r} Int
  main _ =
    let code = verify-scenarios 0 220 in
    if code /= 0 then code
    else
      -- Distinct seeds must diverge in trace hash
      let a = run-scenario 1001 in
      let b = run-scenario 1002 in
      if eq-u64 a.sum-trace b.sum-trace then 20
      else 0
"#;

    let dir = project_with_engine_sim("engine-sim-det-check", src);
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("determinism program checks cleanly");
    common::cleanup(&dir);

    run_leak_checked("engine-sim-det-220", src, 0);
}

#[test]
fn engine_sim_disk_durability_crash_torn_sector_and_bitflip() {
    let src = r#"
module Main where
  import Data.Bytes
  import Data.Hash
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Engine.Sim.Faults
  import Engine.Sim

  fill-const :: Bytes -> U8 -> Nat -> Nat -> Unit
  fill-const b v i n =
    if i < n && i < bytes-len b then
      let _ = bytes-set-u8 b i v in
      fill-const b v (i + 1) n
    else ()

  make-const :: Nat -> U8 -> Bytes
  make-const n v =
    let b = bytes-new n in
    let _ = fill-const b v 0 n in
    b

  all-eq :: Bytes -> U8 -> Nat -> Nat -> Bool
  all-eq b v i end-i =
    if i < end-i && i < bytes-len b then
      if eq-u8 (bytes-get-u8 b i) v then all-eq b v (i + 1) end-i
      else False
    else True

  count-diff-bits :: Bytes -> Bytes -> Nat -> Nat -> U32 -> U32
  count-diff-bits a b i n acc =
    if i < n && i < bytes-len a && i < bytes-len b then
      let x = bit-xor-u8 (bytes-get-u8 a i) (bytes-get-u8 b i) in
      count-diff-bits a b (i + 1) n (wrapping-add-u32 acc (count-ones-u8 x))
    else acc

  main :: forall (r :: Region). Unit ->{IO r, Mut r} Int
  main _ =
    -- 1. Negative durability test: write WITHOUT fsync -> crash -> data LOST
    let w1 = sim-world-new 77 faults-clean in
    let d512 = sim-disk-open w1 512 16 in
    let pat-aa = make-const 512 0xAA in
    let pat-bb = make-const 512 0x55 in
    let zero-sec = bytes-new 512 in
    let e1 = sim-disk-write-sector w1 d512 0 pat-aa in
    if not (io-error-is-ok e1) then 1
    else if ne-u32 (sim-disk-volatile-count w1 d512) 1 then 2
    else if ne-u32 (sim-disk-durable-count w1 d512) 0 then 3
    else
      -- Pre-crash live read observes volatile write
      case sim-disk-read-sector w1 d512 0 of
        Err _ -> 4
        Ok pre ->
          if not (pre == pat-aa) then 5
          else
            let lost = sim-disk-crash-power-loss! w1 d512 in
            if ne-u32 lost 1 then 6
            else if ne-u32 (sim-disk-volatile-count w1 d512) 0 then 7
            else
              case sim-disk-read-sector w1 d512 0 of
                Err _ -> 8
                Ok post ->
                  if not (post == zero-sec) then 9
                  else
                    -- 2. Positive durability test: write WITH fsync -> crash -> survives 100%
                    let _ = sim-disk-write-sector w1 d512 2 pat-aa in
                    let _ = sim-disk-fsync w1 d512 in
                    -- Overwrite sector 2 WITHOUT fsync -> crash must revert to pat-aa
                    let _ = sim-disk-write-sector w1 d512 2 pat-bb in
                    let _ = sim-disk-crash-power-loss! w1 d512 in
                    case sim-disk-read-sector w1 d512 2 of
                      Err _ -> 10
                      Ok survived ->
                        if not (survived == pat-aa) then 11
                        else
                          -- 3. Torn-sector test on 512B and 4 KiB disks (disk-torn-sector-ppm = 1_000_000)
                          let torn-cfg = SimFaultConfig {
                            udp-drop-ppm = 0,
                            udp-dup-ppm = 0,
                            udp-reorder-ppm = 0,
                            udp-corrupt-ppm = 0,
                            udp-min-delay-ns = 0,
                            udp-max-delay-ns = 0,
                            tcp-partial-io-ppm = 0,
                            tcp-rst-ppm = 0,
                            disk-write-delay-ns = 0,
                            disk-torn-sector-ppm = 1000000,
                            disk-bitflip-ppm = 0,
                            partition-mask = 0
                          } in
                          let w2 = sim-world-new 88 torn-cfg in
                          let td512 = sim-disk-open w2 512 4 in
                          let td4k = sim-disk-open w2 4096 4 in
                          let _ = sim-disk-write-sector w2 td512 0 pat-aa in
                          let _ = sim-disk-fsync w2 td512 in
                          let _ = sim-disk-write-sector w2 td512 0 pat-bb in
                          let _ = sim-disk-crash-power-loss! w2 td512 in
                          let _ = sim-disk-write-sector w2 td4k 0 (make-const 4096 0xAA) in
                          let _ = sim-disk-fsync w2 td4k in
                          let _ = sim-disk-write-sector w2 td4k 0 (make-const 4096 0x55) in
                          let _ = sim-disk-crash-power-loss! w2 td4k in
                          case sim-disk-read-sector w2 td512 0 of
                            Err _ -> 12
                            Ok t512 ->
                              if not (all-eq t512 0x55 0 256 && all-eq t512 0xAA 256 512) then 13
                              else
                                case sim-disk-read-sector w2 td4k 0 of
                                  Err _ -> 14
                                  Ok t4k ->
                                    if not (all-eq t4k 0x55 0 2048 && all-eq t4k 0xAA 2048 4096) then 15
                                    else
                                      -- 4. Single-bit flip injection test (disk-bitflip-ppm = 1_000_000)
                                      let flip-cfg = SimFaultConfig {
                                        udp-drop-ppm = 0,
                                        udp-dup-ppm = 0,
                                        udp-reorder-ppm = 0,
                                        udp-corrupt-ppm = 0,
                                        udp-min-delay-ns = 0,
                                        udp-max-delay-ns = 0,
                                        tcp-partial-io-ppm = 0,
                                        tcp-rst-ppm = 0,
                                        disk-write-delay-ns = 0,
                                        disk-torn-sector-ppm = 0,
                                        disk-bitflip-ppm = 1000000,
                                        partition-mask = 0
                                      } in
                                      let w3 = sim-world-new 99 flip-cfg in
                                      let fd-flip = sim-disk-open w3 512 4 in
                                      let _ = sim-disk-write-sector w3 fd-flip 0 pat-aa in
                                      case sim-disk-read-sector w3 fd-flip 0 of
                                        Err _ -> 16
                                        Ok flipped ->
                                          let bits = count-diff-bits pat-aa flipped 0 512 0 in
                                          if ne-u32 bits 1 then 17
                                          else 0
"#;

    run_leak_checked("engine-sim-disk-durability", src, 0);
}

#[test]
fn engine_sim_faults_and_capability_dispatch() {
    let src = r#"
module Main where
  import Data.Bytes
  import Data.Hash
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Engine.Types
  import Engine.Sim.Faults
  import Engine.Sim

  make-seq-bytes :: Nat -> Bytes
  make-seq-bytes n =
    let b = bytes-new n in
    let _ = fill-seq b 0 n in
    b

  fill-seq :: Bytes -> Nat -> Nat -> Unit
  fill-seq b i n =
    if i < n && i < bytes-len b && i < 9223372036854775807 then
      let _ = bytes-set-u8 b i (wrapping-i64-to-u8 (nat-to-i64 i)) in
      fill-seq b (i + 1) n
    else ()

  make-tagged-pkt :: U8 -> Bytes
  make-tagged-pkt tag =
    let b = bytes-new 4 in
    let _ = bytes-set-u8 b 0 tag in
    let _ = bytes-set-u8 b 1 0x42 in
    let _ = bytes-set-u8 b 2 0x43 in
    let _ = bytes-set-u8 b 3 0x44 in
    b

  send-n-udp :: forall r. SimWorld r -> Fd -> SimUdpAddr -> U8 -> U8 ->{Mut r} Unit
  send-n-udp w fd dst i limit =
    if lt-u8 i limit then
      let _ = sim-udp-send w fd dst (make-tagged-pkt i) in
      send-n-udp w fd dst (wrapping-add-u8 i 1) limit
    else ()

  count-udp-and-reorders :: forall r. SimWorld r -> I32 -> U8 -> U32 -> U32 ->{Mut r} (U32, U32)
  count-udp-and-reorders w fd-raw prev-tag seen reorders =
    case sim-udp-recv w (fd-of fd-raw) of
      Err _ -> (seen, reorders)
      Ok dgram ->
        let pl = dgram.dgram-payload in
        let tag = if 0 < bytes-len pl then bytes-get-u8 pl 0 else 0 in
        let next-reorders = (if gt-u32 seen 0 && lt-u8 tag prev-tag then wrapping-add-u32 reorders 1 else reorders) in
        count-udp-and-reorders w fd-raw tag (wrapping-add-u32 seen 1) next-reorders

  -- | Reassemble partial TCP reads into `out` and verify `partial-count > 0`.
  read-all-tcp :: forall r. SimWorld r -> I32 -> Bytes -> Nat -> U32 ->{Mut r} (Nat, U32)
  read-all-tcp w fd-raw out pos reads =
    case sim-tcp-recv w (fd-of fd-raw) 64 of
      Ok chunk ->
        let k = bytes-len chunk in
        let _ = (if pos + k <= bytes-len out && k <= bytes-len chunk then bytes-copy-range out chunk pos 0 k else ()) in
        read-all-tcp w fd-raw out (pos + k) (wrapping-add-u32 reads 1)
      Err _ -> (pos, reads)

  -- | Send a full payload over TCP looping through forced partial writes.
  write-all-tcp :: forall r. SimWorld r -> Fd -> Bytes -> Nat -> U32 ->{Mut r} U32
  write-all-tcp w fd src pos writes =
    if pos >= bytes-len src then writes
    else
      let rem = bytes-len src - pos in
      let slice = bytes-new rem in
      let _ = (if rem <= bytes-len slice && pos + rem <= bytes-len src then bytes-copy-range slice src 0 pos rem else ()) in
      case sim-tcp-send w fd slice of
        Ok n -> write-all-tcp w fd src (pos + u32-to-nat n) (wrapping-add-u32 writes 1)
        Err _ -> writes

  -- | Generic protocol routine written STRICTLY against `Engine.Types`
  -- capability dictionaries (`ReactorOps`, `ClockOps`, `EntropyOps`).
  -- Exercises TCP OpSend/OpRecv, UDP OpSendMsg/OpRecvMsg, Disk OpWriteDirect /
  -- OpFsync / OpReadDirect, OpTimeout + OpCancel, ClockOps, and EntropyOps.
  capability-protocol
    :: forall (r :: Region).
       using (rx :: ReactorOps r, clk :: ClockOps r, rng :: EntropyOps r)
    => Fd -> Fd -> Fd -> Fd -> SimUdpAddr -> Fd ->{IO r, Mut r} U64
  capability-protocol tcp-a tcp-b udp-a udp-b udp-b-addr disk-fd =
    let t0 = engine-clock-mono-ns () in
    let w0 = engine-clock-wall-ns () in
    let e64 = engine-entropy-next-u64 () in
    let e32 = engine-entropy-next-u32 () in
    -- 1. TCP send + recv through ReactorOps
    let tx-buf = make-seq-bytes 32 in
    let rx-buf = bytes-new 32 in
    let tok-send = io-token-pack op-code-send 1 1 in
    let tok-recv = io-token-pack op-code-recv 1 2 in
    let _ = engine-reactor-submit (IoSqe op-code-recv 0 0 tcp-b.fd-raw (wrapping-i64-to-u64 (bytes-data rx-buf)) 32 0 0 tok-recv.io-token-raw) in
    let _ = engine-reactor-submit (IoSqe op-code-send 0 0 tcp-a.fd-raw (wrapping-i64-to-u64 (bytes-data tx-buf)) 32 0 0 tok-send.io-token-raw) in
    let _ = engine-reactor-flush () in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let c1 = engine-reactor-pop-cqe () in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let c2 = engine-reactor-pop-cqe () in
    let tcp-ok = (rx-buf == tx-buf) && eq-i32 c1.cqe-res 32 && eq-i32 c2.cqe-res 32 in
    -- 2. UDP sendmsg + recvmsg through ReactorOps (with packed addr in sqe-aux/cqe-flags)
    let u-tx = make-tagged-pkt 0x7A in
    let u-rx = bytes-new 16 in
    let tok-us = io-token-pack op-code-send-msg 2 1 in
    let tok-ur = io-token-pack op-code-recv-msg 2 2 in
    let _ = engine-reactor-submit (IoSqe op-code-recv-msg 0 0 udp-b.fd-raw (wrapping-i64-to-u64 (bytes-data u-rx)) 16 0 0 tok-ur.io-token-raw) in
    let _ = engine-reactor-submit (IoSqe op-code-send-msg 0 0 udp-a.fd-raw (wrapping-i64-to-u64 (bytes-data u-tx)) 4 (sim-udp-addr-pack udp-b-addr) 0 tok-us.io-token-raw) in
    let _ = engine-reactor-flush () in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let cu1 = engine-reactor-pop-cqe () in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let cu2 = engine-reactor-pop-cqe () in
    let udp-ok = eq-i32 cu1.cqe-res 4 && eq-i32 cu2.cqe-res 4 && eq-u8 (bytes-get-u8 u-rx 0) 0x7A && gt-u32 cu2.cqe-flags 0 in
    -- 3. Disk OpWriteDirect + OpFsync + OpReadDirect through ReactorOps
    let d-tx = make-seq-bytes 64 in
    let d-rx = bytes-new 64 in
    let tok-dw = io-token-pack op-code-write-direct 3 1 in
    let tok-df = io-token-pack op-code-fsync 3 2 in
    let tok-dr = io-token-pack op-code-read-direct 3 3 in
    let _ = engine-reactor-submit (IoSqe op-code-write-direct 0 0 disk-fd.fd-raw (wrapping-i64-to-u64 (bytes-data d-tx)) 64 0 512 tok-dw.io-token-raw) in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let cd1 = engine-reactor-pop-cqe () in
    let _ = engine-reactor-submit (IoSqe op-code-fsync 0 0 disk-fd.fd-raw 0 0 0 0 tok-df.io-token-raw) in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let cd2 = engine-reactor-pop-cqe () in
    let _ = engine-reactor-submit (IoSqe op-code-read-direct 0 0 disk-fd.fd-raw (wrapping-i64-to-u64 (bytes-data d-rx)) 64 0 512 tok-dr.io-token-raw) in
    let _ = engine-reactor-wait (time-ns-of 50000000) in
    let cd3 = engine-reactor-pop-cqe () in
    let disk-ok = (d-rx == d-tx) && eq-i32 cd1.cqe-res 64 && eq-i32 cd2.cqe-res 0 && eq-i32 cd3.cqe-res 64 in
    -- 4. OpTimeout + OpCancel
    let tok-tm = io-token-pack op-code-timeout 4 1 in
    let tok-cn = io-token-pack op-code-cancel 4 2 in
    let _ = engine-reactor-submit (IoSqe op-code-timeout 0 0 (-1) 0 0 0 1000000 tok-tm.io-token-raw) in
    let _ = engine-reactor-submit (IoSqe op-code-cancel 0 0 (-1) 0 0 0 tok-tm.io-token-raw tok-cn.io-token-raw) in
    let _ = engine-reactor-flush () in
    let c-cn = engine-reactor-pop-cqe () in
    let _ = engine-reactor-wait (time-ns-of 5000000) in
    let c-tm = engine-reactor-pop-cqe () in
    let cancel-ok = eq-i32 c-cn.cqe-res 0 && completion-is-err c-tm in
    let t1 = engine-clock-mono-ns () in
    let w1 = engine-clock-wall-ns () in
    let clk-ok = time-ns-lt t0 t1 && time-ns-lt w0 w1 in
    -- Hold buffers alive
    let _ = bytes-len tx-buf in
    let _ = bytes-len rx-buf in
    let _ = bytes-len u-tx in
    let _ = bytes-len u-rx in
    let _ = bytes-len d-tx in
    let _ = bytes-len d-rx in
    if tcp-ok && udp-ok && disk-ok && cancel-ok && clk-ok then
      sim-mix-u64 (bit-xor-u64 e64 (wrapping-add-u64 (u32-to-u64 e32) (time-ns-value t1)))
    else 0

  run-cap-harness :: forall (r :: Region). U64 ->{IO r, Mut r} (U64, U64)
  run-cap-harness seed =
    let cfg = SimFaultConfig {
      udp-drop-ppm = 0,
      udp-dup-ppm = 0,
      udp-reorder-ppm = 0,
      udp-corrupt-ppm = 0,
      udp-min-delay-ns = 500000,
      udp-max-delay-ns = 1500000,
      tcp-partial-io-ppm = 0,
      tcp-rst-ppm = 0,
      disk-write-delay-ns = 250000,
      disk-torn-sector-ppm = 0,
      disk-bitflip-ppm = 0,
      partition-mask = 0
    } in
    let w = sim-world-new seed cfg in
    case sim-tcp-pair w 0 1 of
      (tcp-a, tcp-b) ->
        let udp-a = case sim-udp-bind w 0 7000 of Ok f -> f; Err _ -> fd-invalid () in
        let udp-b = case sim-udp-bind w 1 8000 of Ok f -> f; Err _ -> fd-invalid () in
        let disk = sim-disk-open w 512 8 in
        let out = with-engine-ops (sim-reactor-ops w) (sim-clock-ops w) (sim-entropy-ops w) (\() ->
          capability-protocol tcp-a tcp-b udp-a udp-b (sim-udp-addr-make 1 8000) disk) in
        (out, sim-world-trace-hash w)

  main :: forall (r :: Region). Unit ->{IO r, Mut r} Int
  main _ =
    -- A. UDP drop / duplicate / reorder / corrupt / NAT rebind
    let drop-cfg = SimFaultConfig {
      udp-drop-ppm = 1000000, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 0,
      udp-min-delay-ns = 1000, udp-max-delay-ns = 1000,
      tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
      disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 0, partition-mask = 0
    } in
    let wd = sim-world-new 1 drop-cfg in
    let da = case sim-udp-bind wd 0 1000 of Ok f -> f; Err _ -> fd-invalid () in
    let db = case sim-udp-bind wd 1 2000 of Ok f -> f; Err _ -> fd-invalid () in
    let _ = send-n-udp wd da (sim-udp-addr-make 1 2000) 0 20 in
    let _ = sim-world-run-until-idle wd in
    if ne-u32 (sim-udp-pending-count wd db) 0 then 1
    else
      -- Duplicate + reorder
      let dup-reord-cfg = SimFaultConfig {
        udp-drop-ppm = 0, udp-dup-ppm = 1000000, udp-reorder-ppm = 1000000, udp-corrupt-ppm = 0,
        udp-min-delay-ns = 100000, udp-max-delay-ns = 5000000,
        tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
        disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 0, partition-mask = 0
      } in
      let wr = sim-world-new 2 dup-reord-cfg in
      let ra = case sim-udp-bind wr 0 1000 of Ok f -> f; Err _ -> fd-invalid () in
      let rb = case sim-udp-bind wr 1 2000 of Ok f -> f; Err _ -> fd-invalid () in
      let _ = send-n-udp wr ra (sim-udp-addr-make 1 2000) 0 30 in
      let _ = sim-world-run-until-idle wr in
      case count-udp-and-reorders wr rb.fd-raw 0 0 0 of
        (seen, reorders) ->
          if ne-u32 seen 60 || eq-u32 reorders 0 then 2
          else
            -- NAT rebind (QUIC connection migration)
            let wn = sim-world-new 3 faults-clean in
            let client = case sim-udp-bind wn 0 5000 of Ok f -> f; Err _ -> fd-invalid () in
            let server = case sim-udp-bind wn 1 443 of Ok f -> f; Err _ -> fd-invalid () in
            let rebind-ok = (case sim-udp-nat-rebind wn client 6123 of
              Ok addr -> eq-u16 addr.udp-port 6123
              Err _ -> False) in
            let _ = sim-udp-send wn client (sim-udp-addr-make 1 443) (make-tagged-pkt 9) in
            let _ = sim-world-run-until-idle wn in
            let migrated-src-ok = (case sim-udp-recv wn server of
              Ok dg -> eq-u16 dg.dgram-src.udp-port 6123
              Err _ -> False) in
            -- Reply to stale port 5000 must drop; reply to migrated port 6123 must arrive
            let _ = sim-udp-send wn server (sim-udp-addr-make 0 5000) (make-tagged-pkt 1) in
            let _ = sim-udp-send wn server (sim-udp-addr-make 0 6123) (make-tagged-pkt 2) in
            let _ = sim-world-run-until-idle wn in
            let reply-ok = (case sim-udp-recv wn client of
              Ok dg ->
                let pl = dg.dgram-payload in
                (if 0 < bytes-len pl then eq-u8 (bytes-get-u8 pl 0) 2 else False) && eq-u32 (sim-udp-pending-count wn client) 0
              Err _ -> False) in
            if not (rebind-ok && migrated-src-ok && reply-ok) then 3
            else
              -- B. TCP partial write/read splitting + EOF + RST
              let part-cfg = SimFaultConfig {
                udp-drop-ppm = 0, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 0,
                udp-min-delay-ns = 1000, udp-max-delay-ns = 2000,
                tcp-partial-io-ppm = 1000000, tcp-rst-ppm = 0,
                disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 0, partition-mask = 0
              } in
              let wt = sim-world-new 4 part-cfg in
              case sim-tcp-pair wt 0 1 of
                (ta, tb) ->
                  let full-msg = make-seq-bytes 64 in
                  let writes = write-all-tcp wt ta full-msg 0 0 in
                  let _ = sim-tcp-shutdown-write wt ta in
                  let _ = sim-world-run-until-idle wt in
                  let recv-buf = bytes-new 64 in
                  case read-all-tcp wt tb.fd-raw recv-buf 0 0 of
                    (got-n, reads) ->
                      let eof-ok = case sim-tcp-recv wt tb 16 of Err IoEof -> True; _ -> False in
                      if le-u32 writes 1 || le-u32 reads 1 || got-n /= 64 || not (recv-buf == full-msg) || not eof-ok then 4
                      else
                        -- TCP RST injection
                        let _ = sim-tcp-inject-rst wt ta in
                        let _ = sim-world-run-until-idle wt in
                        let rst-ok = case sim-tcp-recv wt tb 16 of Err IoConnReset -> True; _ -> False in
                        if not rst-ok then 5
                        else
                          -- C. Asymmetric partition isolation + heal
                          let wp = sim-world-new 5 faults-clean in
                          let pa = case sim-udp-bind wp 0 9000 of Ok f -> f; Err _ -> fd-invalid () in
                          let pb = case sim-udp-bind wp 1 9001 of Ok f -> f; Err _ -> fd-invalid () in
                          let _ = sim-world-block-link wp 0 1 in
                          let _ = sim-udp-send wp pa (sim-udp-addr-make 1 9001) (make-tagged-pkt 10) in
                          let _ = sim-udp-send wp pb (sim-udp-addr-make 0 9000) (make-tagged-pkt 20) in
                          let _ = sim-world-run-until-idle wp in
                          -- 0->1 dropped, 1->0 delivered!
                          if ne-u32 (sim-udp-pending-count wp pb) 0 || ne-u32 (sim-udp-pending-count wp pa) 1 then 6
                          else
                            let _ = sim-world-heal-link wp 0 1 in
                            let _ = sim-udp-send wp pa (sim-udp-addr-make 1 9001) (make-tagged-pkt 30) in
                            let _ = sim-world-run-until-idle wp in
                            if ne-u32 (sim-udp-pending-count wp pb) 1 then 7
                            else
                              -- Split-brain preset check: 0->4 blocked, 0->3 open, 4->7 open
                              if not (partition-is-blocked faults-split-brain.partition-mask 0 4)
                                || partition-is-blocked faults-split-brain.partition-mask 0 3
                                || partition-is-blocked faults-split-brain.partition-mask 4 7 then 8
                              else
                                -- D. UDP byte corruption (`udp-corrupt-ppm = 1_000_000`) & ChaCha8 PRNG determinism
                                let cor-cfg = SimFaultConfig {
                                  udp-drop-ppm = 0, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 1000000,
                                  udp-min-delay-ns = 1000, udp-max-delay-ns = 1000,
                                  tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
                                  disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 0, partition-mask = 0
                                } in
                                let wc = sim-world-new 6 cor-cfg in
                                let ca = case sim-udp-bind wc 0 3000 of Ok f -> f; Err _ -> fd-invalid () in
                                let cb = case sim-udp-bind wc 1 3001 of Ok f -> f; Err _ -> fd-invalid () in
                                let orig = make-tagged-pkt 0x55 in
                                let _ = sim-udp-send wc ca (sim-udp-addr-make 1 3001) orig in
                                let _ = sim-world-run-until-idle wc in
                                let corrupt-ok = (case sim-udp-recv wc cb of
                                  Ok dg -> not (dg.dgram-payload == orig)
                                  Err _ -> False) in
                                let cc1 = sim-rng-u64 (sim-rng-chacha8 12345) in
                                let cc2 = sim-rng-u64 (sim-rng-chacha8 12345) in
                                let chacha-ok = (case (cc1, cc2) of ((v1, _), (v2, _)) -> eq-u64 v1 v2 && ne-u64 v1 0) in
                                if not (corrupt-ok && chacha-ok) then 10
                                else
                                  -- E. Zero-drift `with-engine-ops` capability dispatch + deterministic replay
                                  case run-cap-harness 424242 of
                                    (out1, hash1) ->
                                      case run-cap-harness 424242 of
                                        (out2, hash2) ->
                                          if eq-u64 out1 0 || ne-u64 out1 out2 || ne-u64 hash1 hash2 then 9
                                          else 0
"#;

    let dir = project_with_engine_sim("engine-sim-ifaces", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    assert!(
        ifaces.iter().any(|i| i.module == "Engine.Sim.Faults"),
        "Engine.Sim.Faults interface emitted"
    );
    assert!(
        ifaces.iter().any(|i| i.module == "Engine.Sim"),
        "Engine.Sim interface emitted"
    );
    common::cleanup(&dir);

    run_leak_checked("engine-sim-faults-and-caps", src, 0);
}
