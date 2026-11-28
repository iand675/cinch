//! Integration, Maglev minimal-disruption proof, DST circuit-breaker / EWMA /
//! backpressure tests, Linux io_uring SPLICE descriptor verification, live
//! macOS loopback L7 reverse proxy & L4 bidirectional DMA relay tests, and
//! 50,000-iteration zero-allocation hot-loop tests for
//! `Net.Http.Proxy.{Maglev,Pool,Forward}`.
//!
//! Routed through `common::drive_with_profile_opts` so the exact same suite
//! runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-net-http-proxy-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("http-proxy-{tag}"));
    let opts = DriverOptions::default();
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
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

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
    (stdout, stderr)
}

// =========================================================================
// 1. Manifest registration & #[no-alloc] safety check
// =========================================================================

#[test]
fn net_http_proxy_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Http.Proxy.Maglev", "Net/Http/Proxy/Maglev.cinch"),
        ("Net.Http.Proxy.Pool", "Net/Http/Proxy/Pool.cinch"),
        ("Net.Http.Proxy.Forward", "Net/Http/Proxy/Forward.cinch"),
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

    let pos_maglev = pkg.order.iter().position(|n| n == "Net.Http.Proxy.Maglev").unwrap();
    let pos_pool = pkg.order.iter().position(|n| n == "Net.Http.Proxy.Pool").unwrap();
    let pos_forward = pkg.order.iter().position(|n| n == "Net.Http.Proxy.Forward").unwrap();
    let pos_h1_conn = pkg.order.iter().position(|n| n == "Net.Http.V1.Conn").unwrap();
    let pos_darwin = pkg.order.iter().position(|n| n == "Engine.Reactor.Darwin").unwrap();
    let pos_linux = pkg.order.iter().position(|n| n == "Engine.Reactor.Linux").unwrap();
    assert!(pos_maglev < pos_pool);
    assert!(pos_pool < pos_forward);
    assert!(pos_h1_conn < pos_forward && pos_darwin < pos_forward && pos_linux < pos_forward);

    // Compile-time #[no-alloc] audit across all hot-path Maglev, Pool & Forward functions
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Reactor.Linux
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.Proxy.Maglev
  import Net.Http.Proxy.Pool
  import Net.Http.Proxy.Forward

  #[no-alloc]
  audit-proxy-hot-path
    :: MaglevEpochRouter
    -> UpstreamPool
    -> LinuxReactor
    -> I64
    -> I64
    -> I64
    -> I64
    -> I64
    ->{MVar} U32
  audit-proxy-hot-path rtr pool lrx rx-buf hdrs-buf tx-buf ring-meta ip-buf =
    let tbl = maglev-router-active-table rtr in
    let b0 = maglev-lookup tbl 123456789 in
    let b1 = maglev-lookup-healthy tbl 123456789 in
    let b2 = maglev-router-lookup rtr 987654321 in
    let b3 = pool-select-backend pool tbl 123456789 42 1000000 in
    let acq = pool-acquire pool b3 1000000 in
    let _rel = pool-release-idle pool b3 acq.par-slot 1000500 in
    let _ok = pool-record-success pool b3 50000 1000500 in
    let rv = h1-parse-request-dma rx-buf 0 64 0 hdrs-buf in
    let n-req = proxy-rewrite-request-dma rv rx-buf tx-buf 0 2048 ip-buf 9 (eq-u32 0 1) (eq-u32 0 1) in
    let n-err = proxy-write-error-response-dma tx-buf 0 256 (proxy-error-to-status proxy-err-circuit-open) in
    let w = proxy-ring-write-dma ring-meta rx-buf 16 in
    let r = proxy-ring-read-dma ring-meta tx-buf 16 in
    let sp = proxy-l4-splice-step lrx (fd-of 3) (fd-of 4) (fd-of 5) (fd-of 6) 4096 (io-token-pack 12 1 1) (io-token-pack 12 1 2) in
    wrapping-add-u32
      (wrapping-add-u32 (u16-to-u32 b0) (u16-to-u32 b1))
      (wrapping-add-u32 (wrapping-add-u32 (u16-to-u32 b2) n-req) (wrapping-add-u32 (wrapping-add-u32 n-err w) (wrapping-add-u32 r (u8-to-u32 sp))))

  main :: Int
  main = 0
"#,
    )
    .expect("write Main.cinch");

    let opts = DriverOptions::default();
    common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    )
    .expect("Net.Http.Proxy.{Maglev,Pool,Forward} #[no-alloc] audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Maglev Balance (< 3% deviation) & Minimal-Disruption Proof (< 10% remap
//    on 1-of-16 backend removal) + Weights + Epoch Swap + Unhealthy Failover
// =========================================================================

#[test]
fn maglev_balance_minimal_disruption_epoch_swap_and_unhealthy_failover() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.Proxy.Maglev
  import Prim.Atomic
  import Text.Show

  foreign import ccall "write" c-write-stdout :: I32 -> I64 -> I64 ->{IO} I64

  print-str :: String ->{IO} Unit
  print-str s =
    let b = string-to-bytes s in
    let _0 = c-write-stdout 1 (bytes-data b) (bytes-len-i64 b) in
    case bytes-len b of
      _ -> ()

  init-backends16 :: MaglevTable -> U32 -> U32 ->{MVar} Unit
  init-backends16 tbl i skip-id =
    if ge-u32 i 16 then ()
    else
      let active = ne-u32 i skip-id in
      -- Stable 64-bit backend identity key (e.g. IP:port hash)
      let key = wrapping-add-u64 0x0A00000100001F90 (wrapping-mul-u64 (u32-to-u64 i) 0x10001) in
      let _0 = maglev-set-backend tbl (wrapping-u32-to-u16 i) key 1 active in
      init-backends16 tbl (wrapping-add-u32 i 1) skip-id

  -- Count per-backend hits across `n-keys` keys (`use-hash`: True = hashed keys, False = table slots 0..M-1)
  -- into `counts-addr[16 * 4B]` and record primary assignment into `assign-addr[n-keys * 2B]`.
  measure-distribution :: MaglevTable -> I64 -> I64 -> U32 -> U32 -> Bool ->{MVar} Unit
  measure-distribution tbl counts-addr assign-addr i n-keys use-hash =
    if ge-u32 i n-keys then ()
    else
      let kh = if use-hash then maglev-hash-key (u32-to-u64 i) else u32-to-u64 i in
      let bid = maglev-lookup tbl kh in
      let c-slot = wrapping-add-i64 counts-addr (wrapping-shl-i64 (u16-to-i64 bid) 2) in
      let _0 = raw-store-u32 c-slot (wrapping-add-u32 (raw-load-u32 c-slot) 1) in
      let _w = (
        if ne-i64 assign-addr 0 then
          let a-slot = wrapping-add-i64 assign-addr (wrapping-shl-i64 (u32-to-i64 i) 1) in
          let _1 = raw-store-u8 a-slot (wrapping-u16-to-u8 bid) in
          raw-store-u8 (wrapping-add-i64 a-slot 1) (wrapping-u16-to-u8 (wrapping-shr-u16 bid 8))
        else ()) in
      measure-distribution tbl counts-addr assign-addr (wrapping-add-u32 i 1) n-keys use-hash

  zero-counts16 :: I64 -> U32 ->{MVar} Unit
  zero-counts16 counts-addr i =
    if ge-u32 i 16 then ()
    else
      let _0 = raw-store-u32 (wrapping-add-i64 counts-addr (wrapping-shl-i64 (u32-to-i64 i) 2)) 0 in
      zero-counts16 counts-addr (wrapping-add-u32 i 1)

  -- Compute max absolute deviation from `ideal` across 16 backends.
  max-dev16 :: I64 -> U32 -> U32 -> U32 ->{MVar} U32
  max-dev16 counts-addr i ideal best =
    if ge-u32 i 16 then best
    else
      let c = raw-load-u32 (wrapping-add-i64 counts-addr (wrapping-shl-i64 (u32-to-i64 i) 2)) in
      let diff = if ge-u32 c ideal then wrapping-sub-u32 c ideal else wrapping-sub-u32 ideal c in
      let next-best = if gt-u32 diff best then diff else best in
      max-dev16 counts-addr (wrapping-add-u32 i 1) ideal next-best

  -- Count how many of `n-keys` changed backend after removing backend #7.
  count-remaps :: MaglevTable -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  count-remaps tbl assign-addr i n-keys remaps =
    if ge-u32 i n-keys then remaps
    else
      let kh = maglev-hash-key (u32-to-u64 i) in
      let new-bid = maglev-lookup tbl kh in
      let a-slot = wrapping-add-i64 assign-addr (wrapping-shl-i64 (u32-to-i64 i) 1) in
      let lo = u8-to-u16 (raw-load-u8 a-slot) in
      let hi = u8-to-u16 (raw-load-u8 (wrapping-add-i64 a-slot 1)) in
      let old-bid = bit-or-u16 lo (wrapping-shl-u16 hi 8) in
      let next-r = if ne-u16 new-bid old-bid then wrapping-add-u32 remaps 1 else remaps in
      count-remaps tbl assign-addr (wrapping-add-u32 i 1) n-keys next-r

  -- Verify unhealthy backends (e.g. 0, 3, 7) are NEVER returned by `maglev-lookup-healthy`.
  check-no-unhealthy :: MaglevTable -> U32 -> U32 ->{MVar} Bool
  check-no-unhealthy tbl i n =
    if ge-u32 i n then True
    else
      let kh = maglev-hash-key (u32-to-u64 i) in
      let bid = maglev-lookup-healthy tbl kh in
      if eq-u16 bid 0 || eq-u16 bid 3 || eq-u16 bid 7 || eq-u16 bid maglev-empty-slot then False
      else check-no-unhealthy tbl (wrapping-add-u32 i 1) n

  -- Verify weighted table (backend 0 weight 3, backend 1 weight 1) routes ~75% to backend 0.
  count-weighted-b0 :: MaglevTable -> U32 -> U32 -> U32 ->{MVar} U32
  count-weighted-b0 tbl i n hits =
    if ge-u32 i n then hits
    else
      let bid = maglev-lookup tbl (maglev-hash-key (u32-to-u64 i)) in
      let next = if eq-u16 bid 0 then wrapping-add-u32 hits 1 else hits in
      count-weighted-b0 tbl (wrapping-add-u32 i 1) n next

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 1. Build 16-backend MaglevEpochRouter with prime M = 65537
    let rtr = maglev-router-new maglev-default-size 32 in
    if not (maglev-router-valid rtr) then 1
    else
      let scratch = engine-dma-alloc 262144 4096 in
      if eq-u64 scratch 0 then
        let _ = maglev-router-destroy rtr in 2
      else
        let counts-addr = wrapping-u64-to-i64 scratch in
        let assign-addr = wrapping-add-i64 counts-addr 4096 in
        let t0 = maglev-router-active-table rtr in
        let _i0 = init-backends16 t0 0 99 in
        let pop0 = maglev-rebuild t0 in
        -- A. Exact Maglev table slot balance across all M = 65537 slots (ideal = 4096, max_dev <= 1!)
        let _m-tbl = measure-distribution t0 counts-addr 0 0 maglev-default-size False in
        let tbl-max-dev = max-dev16 counts-addr 0 4096 0 in
        let tbl-dev-bps = div-u32 (wrapping-mul-u32 tbl-max-dev 10000) 4096 in
        -- B. Hashed-key balance across 200,000 keys (ideal = 12500/backend; require < 3% deviation)
        let _zc = zero-counts16 counts-addr 0 in
        let _m-hash = measure-distribution t0 counts-addr 0 0 200000 True in
        let hash-max-dev = max-dev16 counts-addr 0 12500 0 in
        let hash-dev-bps = div-u32 (wrapping-mul-u32 hash-max-dev 10000) 12500 in
        -- C. Record 65,536 hashed-key assignments, remove backend #7, atomic epoch swap & measure disruption!
        let _zc2 = zero-counts16 counts-addr 0 in
        let n-keys = (65536 :: U32) in
        let _m = measure-distribution t0 counts-addr assign-addr 0 n-keys True in
        let t1 = maglev-router-standby-table rtr in
        let _i1 = init-backends16 t1 0 7 in
        let pop1 = maglev-rebuild t1 in
        let ep-before = maglev-router-epoch rtr in
        let ep-after = maglev-router-swap rtr in
        let active-after = maglev-router-active-table rtr in
        let remaps = count-remaps active-after assign-addr 0 n-keys 0 in
        -- Disruption basis points (out of 10000): ideal 1/16 = 625 bps (6.25%), require < 1000 bps (10%).
        let disrupt-bps = if n-keys /= 0 then div-u32 (wrapping-mul-u32 remaps 10000) n-keys else 10000 in
        let _p = print-str (string-concat "MAGLEV_STATS pop0=" (string-concat (show (u32-to-nat pop0))
          (string-concat " pop1=" (string-concat (show (u32-to-nat pop1))
          (string-concat " tbl_dev_bps=" (string-concat (show (u32-to-nat tbl-dev-bps))
          (string-concat " hash_dev_bps=" (string-concat (show (u32-to-nat hash-dev-bps))
          (string-concat " remaps=" (string-concat (show (u32-to-nat remaps))
          (string-concat " disrupt_bps=" (string-concat (show (u32-to-nat disrupt-bps)) "\n")))))))))))) in
        -- 2. Verify unhealthy failover on active table (mark backends 0, 3, 7 unhealthy)
        let _h0 = maglev-backend-set-health active-after 0 False in
        let _h3 = maglev-backend-set-health active-after 3 False in
        let _h7 = maglev-backend-set-health active-after 7 False in
        let ok-healthy = check-no-unhealthy active-after 0 10000 in
        -- Also verify explicit 64-bit mask lookup (only backend 5 healthy: bit 5 = 32)
        let ok-mask = eq-u16 (maglev-lookup-with-mask active-after 12345 32) 5 in
        -- 3. Verify weighted Maglev (M = 4099, backend 0 weight 3, backend 1 weight 1 => ~75% / ~25%)
        let wt = maglev-table-new maglev-size4099 8 in
        let _w0 = maglev-set-backend wt 0 0x1111222233334444 3 True in
        let _w1 = maglev-set-backend wt 1 0x5555666677778888 1 True in
        let wpop = maglev-rebuild wt in
        let b0-hits = count-weighted-b0 wt 0 4000 0 in
        -- Expect ~3000 hits (75% of 4000), allow [2850, 3150]
        let ok-weight = eq-u32 wpop maglev-size4099 && ge-u32 b0-hits 2850 && le-u32 b0-hits 3150 in
        let _dw = maglev-table-destroy wt in
        let _fs = engine-dma-free scratch 262144 in
        let _dr = maglev-router-destroy rtr in
        if ne-u32 pop0 maglev-default-size then 10
        else if ne-u32 pop1 maglev-default-size then 11
        else if ne-u64 ep-before 1 || ne-u64 ep-after 2 then 12
        else if gt-u32 tbl-max-dev 1 || ge-u32 hash-dev-bps 300 then 13
        else if ge-u32 disrupt-bps 1000 then 14
        else if not ok-healthy then 15
        else if not ok-mask then 16
        else if not ok-weight then 17
        else 0
"#;
    let (stdout, _stderr) = build_and_run_leak_checked("maglev-proof", src, 0);
    assert!(
        stdout.contains("MAGLEV_STATS"),
        "expected MAGLEV_STATS in stdout, got:\n{stdout}"
    );
    eprintln!("[Maglev proof] {}", stdout.trim());
}

// =========================================================================
// 3. Circuit Breaker, EWMA, P2C Failover, L7 Hop-by-Hop Rewriting,
//    Backpressure Hysteresis & Linux io_uring SPLICE Descriptor DST Tests
// =========================================================================

#[test]
fn circuit_breaker_ewma_backpressure_l7_rewrite_and_splice_dst_tests() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Reactor.Linux
  import Engine.Sim
  import Engine.Sim.Faults
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.Proxy.Maglev
  import Net.Http.Proxy.Pool
  import Net.Http.Proxy.Forward
  import Prim.Atomic

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-loop b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-loop :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-loop b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-loop b (i + 1) dst n
    else ()

  -- Check if DMA region `[base, base+len)` contains ASCII substring `needle`.
  dma-contains-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-contains-str base len needle =
    let nb = string-to-bytes needle in
    let nlen = wrapping-i64-to-u32 (bytes-len-i64 nb) in
    if eq-u32 nlen 0 then True
    else if gt-u32 nlen len then False
    else dma-search-sub base nb 0 (wrapping-sub-u32 len nlen) nlen

  dma-match-at :: I64 -> Bytes -> U32 -> U32 -> U32 ->{MVar} Bool
  dma-match-at base nb pos j nlen =
    if ge-u32 j nlen then True
    else
      let j-nat = u32-to-nat j in
      if j-nat < bytes-len nb then
        let off = wrapping-add-u32 pos j in
        let got = raw-load-u8 (wrapping-add-i64 base (u32-to-i64 off)) in
        if eq-u8 got (bytes-get-u8 nb j-nat) then dma-match-at base nb pos (wrapping-add-u32 j 1) nlen
        else False
      else False

  dma-search-sub :: I64 -> Bytes -> U32 -> U32 -> U32 ->{MVar} Bool
  dma-search-sub base nb pos max-pos nlen =
    if gt-u32 pos max-pos then False
    else if dma-match-at base nb pos 0 nlen then True
    else dma-search-sub base nb (wrapping-add-u32 pos 1) max-pos nlen

  -- Run DST simulation tests using Engine.Sim virtual clock
  run-dst-pool-tests :: forall (r :: Region). UpstreamPool -> MaglevTable ->{IO r, Mut r, MVar} Int
  run-dst-pool-tests pool mt =
    let w = sim-world-new 7777 faults-clean in
    let t0 = sim-world-now-ns w in
    -- A. EWMA exact math check: sample1=100_000 -> 100_000; sample2=900_000 -> (700_000+900_000)/8 = 200_000
    let _s1 = pool-record-success pool 0 100000 t0 in
    let ewma1 = pool-backend-ewma-ns pool 0 in
    let _s2 = pool-record-success pool 0 900000 t0 in
    let ewma2 = pool-backend-ewma-ns pool 0 in
    -- Backend 1 gets low latency 10_000ns so P2C prefers backend 1 over slow backend 0
    let _b1 = pool-record-success pool 1 10000 t0 in
    let _b2 = pool-record-success pool 2 10000 t0 in
    let _b3 = pool-record-success pool 3 10000 t0 in
    -- Verify P2C score of backend 1 < backend 0
    let ok-ewma = eq-u64 ewma1 100000 && eq-u64 ewma2 200000 && lt-u64 (pool-backend-score pool 1) (pool-backend-score pool 0) in

    -- B. Circuit breaker trip on backend 0: window=8, threshold=50%, min_samples=4.
    -- Backend 0 currently has 2 successes; inject 4 failures => 4/6 = 66% >= 50% -> Open!
    let _f1 = pool-record-failure pool 0 t0 in
    let _f2 = pool-record-failure pool 0 t0 in
    let _f3 = pool-record-failure pool 0 t0 in
    let st-open = pool-record-failure pool 0 t0 in
    let acq-open = pool-acquire pool 0 t0 in
    let status503 = proxy-error-to-status proxy-err-circuit-open in
    let ok-trip = (
      eq-u8 st-open cb-state-open
        && eq-u8 acq-open.par-status pool-acq-circuit-open
        && eq-u16 status503 503
        && not (pool-backend-allow-request pool 0 t0)) in

    -- C. Verify pool-select-backend NEVER picks open backend 0 (automatic failover!)
    let sel-a = pool-select-backend pool mt 111 1 t0 in
    let sel-b = pool-select-backend pool mt 222 2 t0 in
    let sel-c = pool-select-backend pool mt 333 3 t0 in
    let ok-failover = ne-u16 sel-a 0 && ne-u16 sel-b 0 && ne-u16 sel-c 0 && ne-u16 sel-a pool-no-backend in

    -- D. Advance virtual clock past 5s cooldown -> transitions Open -> HalfOpen!
    let _adv1 = sim-world-advance-to w (time-ns-of 6000000000) in
    let t1 = sim-world-now-ns w in
    let st-ho = pool-backend-circuit-state pool 0 t1 in
    -- Single-probe gating: first acquire succeeds with par-is-probe == 1; second fails fast!
    let acq-p1 = pool-acquire pool 0 t1 in
    let acq-p2 = pool-acquire pool 0 t1 in
    -- Probe fails -> re-opens circuit!
    let _rel-p1 = pool-release-close pool 0 acq-p1.par-slot in
    let st-reopen = pool-record-failure pool 0 t1 in
    let ok-ho-fail = (
      eq-u8 st-ho cb-state-half-open
        && eq-u8 acq-p1.par-status pool-acq-need-connect
        && eq-u8 acq-p1.par-is-probe 1
        && eq-u8 acq-p2.par-status pool-acq-circuit-open
        && eq-u8 st-reopen cb-state-open) in

    -- E. Second cooldown -> HalfOpen -> probe succeeds -> Closed!
    let _adv2 = sim-world-advance-to w (time-ns-of 12000000000) in
    let t2 = sim-world-now-ns w in
    let acq-p3 = pool-acquire pool 0 t2 in
    let _att3 = pool-attach-fd pool 0 acq-p3.par-slot (fd-of 42) pool-proto-h1 in
    let _rel3 = pool-release-idle pool 0 acq-p3.par-slot t2 in
    let st-rec = pool-record-success pool 0 25000 t2 in
    -- Reuse idle keep-alive fd=42 immediately at t2 + 1s (within 10s TTL)
    let _adv3 = sim-world-advance-to w (time-ns-of 13000000000) in
    let t3 = sim-world-now-ns w in
    let acq-reuse = pool-acquire pool 0 t3 in
    let _rel4 = pool-release-idle pool 0 acq-reuse.par-slot t3 in
    -- Advance past 10s idle TTL -> idle connection expires cleanly and hands fd=42 back!
    let _adv4 = sim-world-advance-to w (time-ns-of 25000000000) in
    let t4 = sim-world-now-ns w in
    let acq-exp = pool-acquire pool 0 t4 in
    -- Verify max-conns (2) limit enforcement on backend 1
    let m1 = pool-acquire pool 1 t4 in
    let m2 = pool-acquire pool 1 t4 in
    let m3 = pool-acquire pool 1 t4 in
    let _rm1 = pool-release-close pool 1 m1.par-slot in
    let _rm2 = pool-release-close pool 1 m2.par-slot in
    let ok-recover-ttl = (
      eq-u8 acq-p3.par-is-probe 1
        && eq-u8 st-rec cb-state-closed
        && eq-u8 acq-reuse.par-status pool-acq-reused
        && eq-i32 acq-reuse.par-fd 42
        && eq-u8 acq-exp.par-status pool-acq-expired
        && eq-i32 acq-exp.par-fd 42
        && eq-u8 m1.par-status pool-acq-need-connect
        && eq-u8 m2.par-status pool-acq-need-connect
        && eq-u8 m3.par-status pool-acq-full) in

    if not ok-ewma then 10
    else if not ok-trip then 11
    else if not ok-failover then 12
    else if not ok-ho-fail then 13
    else if not ok-recover-ttl then 14
    else 0

  main :: forall (r :: Region). Unit ->{IO r, IO, Mut r, MVar} Int
  main _ =
    -- 1. DST Circuit Breaker, EWMA, P2C Failover & Idle TTL tests
    let cfg = PoolConfig 4 2 10000000000 8 50 4 0 5000000000 0 in
    let pool = pool-new cfg in
    let mt = maglev-table-new maglev-size257 4 in
    let _r0 = pool-register-backend pool 0 0x7F000001 8001 in
    let _r1 = pool-register-backend pool 1 0x7F000001 8002 in
    let _r2 = pool-register-backend pool 2 0x7F000001 8003 in
    let _r3 = pool-register-backend pool 3 0x7F000001 8004 in
    let _m0 = maglev-set-backend mt 0 1001 1 True in
    let _m1 = maglev-set-backend mt 1 1002 1 True in
    let _m2 = maglev-set-backend mt 2 1003 1 True in
    let _m3 = maglev-set-backend mt 3 1004 1 True in
    let _mb = maglev-rebuild mt in
    let dst-rc = run-dst-pool-tests pool mt in
    let _dp = pool-destroy pool in
    let _dm = maglev-table-destroy mt in
    if dst-rc /= 0 then dst-rc
    else
      -- 2. L7 Hop-by-Hop Stripping, XFF/XFP/Via Injection, 502/503/504, Backpressure & Linux SPLICE SQE check
      let dma = engine-dma-alloc 32768 4096 in
      if eq-u64 dma 0 then 20
      else
        let slab = wrapping-u64-to-i64 dma in
        let rx-buf = slab in
        let hdrs-buf = wrapping-add-i64 slab 4096 in
        let tx-buf = wrapping-add-i64 slab 8192 in
        let ip-buf = wrapping-add-i64 slab 12288 in
        let ring-meta = wrapping-add-i64 slab 12544 in
        let ring-data = wrapping-add-i64 slab 12800 in
        let scratch-io = wrapping-add-i64 slab 16384 in
        let sq-ring = wrapping-add-i64 slab 20480 in
        let sqes-buf = wrapping-add-i64 slab 24576 in
        let st-buf = wrapping-add-i64 slab 28672 in

        let ip-len = copy-str-to-dma "192.0.2.42" ip-buf in
        let req-len = copy-str-to-dma "GET /v1/users?active=1 HTTP/1.1\r\nHost: api.example.com\r\nConnection: keep-alive, X-Custom-Hop\r\nKeep-Alive: timeout=15\r\nProxy-Connection: keep-alive\r\nTE: trailers\r\nTrailer: X-Sig\r\nUpgrade: h2c\r\nX-Custom-Hop: drop-me\r\nX-Forwarded-For: 10.1.2.3\r\nX-Request-Id: req-99\r\n\r\n" rx-buf in
        let rv = h1-parse-request-dma rx-buf 0 req-len 0 hdrs-buf in
        -- Rewrite with is-tls=True, allow-upgrade=False (so Upgrade MUST be stripped)
        let out-len = proxy-rewrite-request-dma rv rx-buf tx-buf 0 4096 ip-buf ip-len True False in
        let ok-l7-req = (
          eq-u8 rv.rv-status h1-ok
            && gt-u32 out-len 0
            && le-u32 out-len 4096
            && dma-contains-str tx-buf out-len "GET /v1/users?active=1 HTTP/1.1\r\n"
            && dma-contains-str tx-buf out-len "Host: api.example.com\r\n"
            && dma-contains-str tx-buf out-len "X-Request-Id: req-99\r\n"
            && dma-contains-str tx-buf out-len "X-Forwarded-For: 10.1.2.3, 192.0.2.42\r\n"
            && dma-contains-str tx-buf out-len "X-Forwarded-Proto: https\r\n"
            && dma-contains-str tx-buf out-len "Via: 1.1 cinch-proxy\r\n"
            && not (dma-contains-str tx-buf out-len "Keep-Alive:")
            && not (dma-contains-str tx-buf out-len "Proxy-Connection:")
            && not (dma-contains-str tx-buf out-len "TE:")
            && not (dma-contains-str tx-buf out-len "Trailer:")
            && not (dma-contains-str tx-buf out-len "Upgrade:")
            && not (dma-contains-str tx-buf out-len "X-Custom-Hop:")) in

        -- Explicit WebSocket upgrade preservation (allow-upgrade=True)
        let ws-len = copy-str-to-dma "GET /chat HTTP/1.1\r\nHost: ws.example.com\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n" rx-buf in
        let rv-ws = h1-parse-request-dma rx-buf 0 ws-len 0 hdrs-buf in
        let ws-out = proxy-rewrite-request-dma rv-ws rx-buf tx-buf 0 4096 ip-buf ip-len False True in
        let ok-ws = (
          dma-contains-str tx-buf ws-out "Upgrade: websocket\r\n"
            && dma-contains-str tx-buf ws-out "Connection: Upgrade\r\n"
            && dma-contains-str tx-buf ws-out "X-Forwarded-Proto: http\r\n") in

        -- Error responses: 502, 503, 504
        let n502 = proxy-write-error-response-dma tx-buf 0 512 (proxy-error-to-status proxy-err-upstream-fail) in
        let ok502 = dma-contains-str tx-buf n502 "HTTP/1.1 502 Bad Gateway\r\n" && dma-contains-str tx-buf n502 "Via: 1.1 cinch-proxy\r\n" in
        let n503 = proxy-write-error-response-dma tx-buf 0 512 (proxy-error-to-status proxy-err-circuit-open) in
        let ok503 = dma-contains-str tx-buf n503 "HTTP/1.1 503 Service Unavailable\r\n" in
        let n504 = proxy-write-error-response-dma tx-buf 0 512 (proxy-error-to-status proxy-err-timeout) in
        let ok504 = dma-contains-str tx-buf n504 "HTTP/1.1 504 Gateway Timeout\r\n" in

        -- Backpressure ring hysteresis test: cap=64, high_water=48, low_water=16
        let _ri = proxy-ring-init ring-meta ring-data 64 48 16 in
        let _w1 = proxy-ring-write-dma ring-meta scratch-io 32 in
        let p-before = proxy-ring-is-paused ring-meta in -- 32 < 48 => False
        let _w2 = proxy-ring-write-dma ring-meta scratch-io 20 in
        let p-hit-hw = proxy-ring-is-paused ring-meta in -- 52 >= 48 => True
        -- Attempt to write 100 more bytes: must clamp cleanly to 12 free bytes (no overflow/OOM!)
        let w-clamped = proxy-ring-write-dma ring-meta scratch-io 100 in
        let _r1 = proxy-ring-read-dma ring-meta scratch-io 32 in
        let p-still-paused = proxy-ring-is-paused ring-meta in -- 32 > 16 => still True (hysteresis!)
        let _r2 = proxy-ring-read-dma ring-meta scratch-io 16 in
        let p-resumed = proxy-ring-is-paused ring-meta in -- 16 <= 16 => False (resumed!)
        let ok-bp = (
          not p-before && p-hit-hw && eq-u32 w-clamped 12 && eq-u32 (proxy-ring-buffered ring-meta) 16
            && p-still-paused && not p-resumed) in

        -- Linux io_uring SPLICE linked pair descriptor test (host-portable in-memory SQ ring!)
        let lrx = linux-reactor-from-raw-rings (-1) sq-ring 0 4 8 64 sqes-buf sq-ring 128 132 8 192 st-buf in
        let tok0 = io-token-pack op-code-splice 1 10 in
        let tok1 = io-token-pack op-code-splice 1 11 in
        let sp-rc = proxy-l4-splice-step lrx (fd-of 10) (fd-of 11) (fd-of 12) (fd-of 13) 65536 tok0 tok1 in
        -- Inspect SQE 0 and SQE 1 exact Linux UAPI fields:
        --   +0 opcode=29, +1 flags=IOSQE_IO_LINK(4) on SQE0 / 0 on SQE1, +4 fd_out, +24 len, +28 splice_flags=3, +32 user_data, +44 splice_fd_in
        let s0-w0 = raw-load-u32 sqes-buf in
        let s0-fd-out = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqes-buf 4)) in
        let s0-len = raw-load-u32 (wrapping-add-i64 sqes-buf 24) in
        let s0-sflags = raw-load-u32 (wrapping-add-i64 sqes-buf 28) in
        let s0-udata = raw-load-u64 (wrapping-add-i64 sqes-buf 32) in
        let s0-fd-in = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqes-buf 44)) in
        let sqe1 = wrapping-add-i64 sqes-buf 64 in
        let s1-w0 = raw-load-u32 sqe1 in
        let s1-fd-out = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqe1 4)) in
        let s1-fd-in = wrapping-u32-to-i32 (raw-load-u32 (wrapping-add-i64 sqe1 44)) in
        let s1-udata = raw-load-u64 (wrapping-add-i64 sqe1 32) in
        let plan = proxy-l4-splice-plan (fd-of 10) (fd-of 11) (fd-of 12) (fd-of 13) 65536 tok0 tok1 in
        let ok-splice = (
          eq-u8 sp-rc l4-splice-ok
            && eq-u32 (linux-reactor-pending-sqes lrx) 2
            && eq-u8 (wrapping-u32-to-u8 s0-w0) 29
            && eq-u8 (wrapping-u32-to-u8 (wrapping-shr-u32 s0-w0 8)) iosqe-io-link
            && eq-i32 s0-fd-in 10 && eq-i32 s0-fd-out 11
            && eq-u32 s0-len 65536 && eq-u32 s0-sflags 3
            && eq-u64 s0-udata tok0.io-token-raw
            && eq-u8 (wrapping-u32-to-u8 s1-w0) 29
            && eq-u8 (wrapping-u32-to-u8 (wrapping-shr-u32 s1-w0 8)) 0
            && eq-i32 s1-fd-in 12 && eq-i32 s1-fd-out 13
            && eq-u64 s1-udata tok1.io-token-raw
            && eq-u8 plan.sp-sqe-in.sqe-flags iosqe-io-link
            && eq-u32 plan.sp-chunk-len 65536) in

        let _fr = engine-dma-free dma 32768 in
        if not ok-l7-req then 21
        else if not ok-ws then 22
        else if not (ok502 && ok503 && ok504) then 23
        else if not ok-bp then 24
        else if not ok-splice then 25
        else 0
"#;
    build_and_run_leak_checked("dst-cb-ewma-bp-l7-splice", src, 0);
}

// =========================================================================
// 4. Live macOS Loopback L7 Reverse Proxy + L4 Bidirectional DMA Relay +
//    50,000-Iteration Zero-Alloc Steady-State Hot Loop (allocs==0, atomic_rc==0)
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn live_macos_loopback_l7_proxy_l4_relay_and_50000_iter_zero_alloc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Reactor.Darwin
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.Proxy.Maglev
  import Net.Http.Proxy.Pool
  import Net.Http.Proxy.Forward
  import Prim.Atomic

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-loop b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-loop :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-loop b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-loop b (i + 1) dst n
    else ()

  dma-contains-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-contains-str base len needle =
    let nb = string-to-bytes needle in
    let nlen = wrapping-i64-to-u32 (bytes-len-i64 nb) in
    if eq-u32 nlen 0 then True
    else if gt-u32 nlen len then False
    else dma-search-sub base nb 0 (wrapping-sub-u32 len nlen) nlen

  dma-match-at :: I64 -> Bytes -> U32 -> U32 -> U32 ->{MVar} Bool
  dma-match-at base nb pos j nlen =
    if ge-u32 j nlen then True
    else
      let j-nat = u32-to-nat j in
      if j-nat < bytes-len nb then
        let off = wrapping-add-u32 pos j in
        let got = raw-load-u8 (wrapping-add-i64 base (u32-to-i64 off)) in
        if eq-u8 got (bytes-get-u8 nb j-nat) then dma-match-at base nb pos (wrapping-add-u32 j 1) nlen
        else False
      else False

  dma-search-sub :: I64 -> Bytes -> U32 -> U32 -> U32 ->{MVar} Bool
  dma-search-sub base nb pos max-pos nlen =
    if gt-u32 pos max-pos then False
    else if dma-match-at base nb pos 0 nlen then True
    else dma-search-sub base nb (wrapping-add-u32 pos 1) max-pos nlen

  accept-loop :: DarwinReactor -> Fd -> I64 -> U32 ->{IO, MVar} Fd
  accept-loop rx lfd cqe-buf budget =
    if eq-u32 budget 0 then fd-invalid ()
    else
      case darwin-tcp-accept-nonblock lfd of
        Ok cfd -> cfd
        Err IoAgain ->
          let _ = darwin-reactor-poll rx 50000000 cqe-buf 16 in
          accept-loop rx lfd cqe-buf (wrapping-sub-u32 budget 1)
        Err _ -> fd-invalid ()

  recv-some :: DarwinReactor -> Fd -> I64 -> U32 -> I64 -> U32 ->{IO, MVar} I32
  recv-some rx fd buf cap cqe-buf budget =
    if eq-u32 budget 0 then -60
    else
      let nr = darwin-recv-dma fd buf cap in
      if gt-i32 nr 0 then nr
      else if eq-i32 nr 0 then 0
      else if eq-i32 nr -35 || eq-i32 nr -4 then
        let _ = darwin-reactor-poll rx 50000000 cqe-buf 16 in
        recv-some rx fd buf cap cqe-buf (wrapping-sub-u32 budget 1)
      else nr

  recv-until-eof :: DarwinReactor -> Fd -> I64 -> U32 -> U32 -> I64 -> U32 ->{IO, MVar} I32
  recv-until-eof rx fd buf got cap cqe-buf budget =
    if eq-u32 budget 0 then wrapping-u32-to-i32 got
    else if ge-u32 got cap then wrapping-u32-to-i32 got
    else
      let nr = darwin-recv-dma fd (wrapping-add-i64 buf (u32-to-i64 got)) (wrapping-sub-u32 cap got) in
      if gt-i32 nr 0 then
        recv-until-eof rx fd buf (wrapping-add-u32 got (wrapping-i32-to-u32 nr)) cap cqe-buf budget
      else if eq-i32 nr 0 then wrapping-u32-to-i32 got
      else if eq-i32 nr -35 || eq-i32 nr -4 then
        let _ = darwin-reactor-poll rx 20000000 cqe-buf 16 in
        recv-until-eof rx fd buf got cap cqe-buf (wrapping-sub-u32 budget 1)
      else nr

  -- Pump L4 relay until `l4-relay-done` or budget expires, driving backend echo on EOF.
  pump-l4-until-c2u-eof :: DarwinReactor -> Fd -> Fd -> Fd -> I64 -> I64 -> U32 -> I64 -> U32 ->{IO, MVar} U32
  pump-l4-until-c2u-eof rx p-cli p-up b-srv relay-addr b-buf b-got cqe-buf budget =
    if eq-u32 budget 0 then b-got
    else
      let _st = proxy-l4-relay-dma p-cli p-up relay-addr in
      let nr = darwin-recv-dma b-srv (wrapping-add-i64 b-buf (u32-to-i64 b-got)) (wrapping-sub-u32 1024 b-got) in
      if gt-i32 nr 0 then
        pump-l4-until-c2u-eof rx p-cli p-up b-srv relay-addr b-buf (wrapping-add-u32 b-got (wrapping-i32-to-u32 nr)) cqe-buf (wrapping-sub-u32 budget 1)
      else if eq-i32 nr 0 then b-got
      else
        let _p = darwin-reactor-poll rx 10000000 cqe-buf 16 in
        pump-l4-until-c2u-eof rx p-cli p-up b-srv relay-addr b-buf b-got cqe-buf (wrapping-sub-u32 budget 1)

  pump-l4-until-done :: DarwinReactor -> Fd -> Fd -> Fd -> I64 -> I64 -> U32 -> I64 -> U32 ->{IO, MVar} U32
  pump-l4-until-done rx p-cli p-up cli-fd relay-addr c-buf c-got cqe-buf budget =
    if eq-u32 budget 0 then c-got
    else
      let st = proxy-l4-relay-dma p-cli p-up relay-addr in
      let nr = darwin-recv-dma cli-fd (wrapping-add-i64 c-buf (u32-to-i64 c-got)) (wrapping-sub-u32 1024 c-got) in
      let next-got = if gt-i32 nr 0 then wrapping-add-u32 c-got (wrapping-i32-to-u32 nr) else c-got in
      if eq-u8 st l4-relay-done && eq-i32 nr 0 then next-got
      else
        let _p = darwin-reactor-poll rx 10000000 cqe-buf 16 in
        pump-l4-until-done rx p-cli p-up cli-fd relay-addr c-buf next-got cqe-buf (wrapping-sub-u32 budget 1)

  -- Steady-state 50,000-iteration zero-alloc proxy hot loop:
  -- Maglev lookup + P2C select + pool acquire/release + EWMA update + H1 parse + L7 header rewrite!
  #[no-alloc]
  proxy-hot-loop50000
    :: MaglevEpochRouter
    -> UpstreamPool
    -> I64
    -> U32
    -> I64
    -> I64
    -> I64
    -> U32
    -> U32
    -> U32
    ->{MVar} U32
  proxy-hot-loop50000 rtr pool rx-buf rx-len hdrs-buf tx-buf ip-buf ip-len i limit =
    if ge-u32 i limit then limit
    else
      let kh = maglev-hash-key (u32-to-u64 i) in
      let tbl = maglev-router-active-table rtr in
      let bid = pool-select-backend pool tbl kh (u32-to-u64 i) 1000000 in
      let acq = pool-acquire pool bid 1000000 in
      let rv = h1-parse-request-dma rx-buf 0 rx-len 0 hdrs-buf in
      let out-len = proxy-rewrite-request-dma rv rx-buf tx-buf 0 4096 ip-buf ip-len (eq-u32 0 1) (eq-u32 0 1) in
      let _rel = pool-release-idle pool bid acq.par-slot 1000050 in
      let _rec = pool-record-success pool bid 42000 1000050 in
      if eq-u32 out-len 0 then 0
      else proxy-hot-loop50000 rtr pool rx-buf rx-len hdrs-buf tx-buf ip-buf ip-len (wrapping-add-u32 i 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 65536 4096 in
    if eq-u64 dma 0 then 1
    else
      let slab = wrapping-u64-to-i64 dma in
      let cli-tx = slab in
      let p-rx = wrapping-add-i64 slab 4096 in
      let p-tx = wrapping-add-i64 slab 8192 in
      let b-rx = wrapping-add-i64 slab 12288 in
      let b-tx = wrapping-add-i64 slab 16384 in
      let p-urx = wrapping-add-i64 slab 20480 in
      let p-ctx = wrapping-add-i64 slab 24576 in
      let cli-rx = wrapping-add-i64 slab 28672 in
      let hdrs-buf = wrapping-add-i64 slab 32768 in
      let ip-buf = wrapping-add-i64 slab 36864 in
      let cqe-buf = wrapping-add-i64 slab 37120 in
      let relay-addr = wrapping-add-i64 slab 40960 in
      let ring-c2u = wrapping-add-i64 slab 45056 in
      let ring-u2c = wrapping-add-i64 slab 49152 in
      let relay-scratch = wrapping-add-i64 slab 53248 in

      let rtr = maglev-router-new maglev-size257 8 in
      let pool = pool-new (pool-config-default 8 8) in
      case darwin-reactor-new 128 of
        Err _ ->
          let _0 = pool-destroy pool in
          let _1 = maglev-router-destroy rtr in
          let _2 = engine-dma-free dma 65536 in 2
        Ok rx ->
          -- =================================================================
          -- Part A: Live macOS Loopback L7 Reverse Proxy (client -> proxy -> backend)
          -- =================================================================
          case darwin-tcp-listen-reuseport 0 16 of
            Err _ -> 3
            Ok (backend-lfd, backend-port) ->
              case darwin-tcp-listen-reuseport 0 16 of
                Err _ -> 4
                Ok (proxy-lfd, proxy-port) ->
                  let _reg = pool-register-backend pool 0 ipv4-loopback backend-port in
                  let tbl0 = maglev-router-active-table rtr in
                  let _mb0 = maglev-set-backend tbl0 0 0xCAFEBABE0001 1 True in
                  let _mrb = maglev-rebuild tbl0 in
                  let _al1 = darwin-reactor-arm-read rx proxy-lfd (io-token-pack op-code-accept 1 1) in
                  let _al2 = darwin-reactor-arm-read rx backend-lfd (io-token-pack op-code-accept 1 2) in
                  case darwin-tcp-connect-nonblock ipv4-loopback proxy-port of
                    Err _ -> 5
                    Ok cli-fd ->
                      let _p0 = darwin-reactor-poll rx 200000000 cqe-buf 16 in
                      let p-cli-fd = accept-loop rx proxy-lfd cqe-buf 64 in
                      if not (fd-is-valid p-cli-fd) then 6
                      else
                        let _a3 = darwin-reactor-arm-read rx p-cli-fd (io-token-pack op-code-recv 1 3) in
                        let _a4 = darwin-reactor-arm-read rx cli-fd (io-token-pack op-code-recv 1 4) in
                        let _p1 = darwin-reactor-poll rx 0 cqe-buf 16 in
                        -- Client sends L7 request with hop-by-hop headers
                        let req-len = copy-str-to-dma "GET /hello?x=1 HTTP/1.1\r\nHost: proxy.local\r\nConnection: keep-alive, X-Drop-Me\r\nKeep-Alive: timeout=5\r\nProxy-Connection: keep-alive\r\nX-Drop-Me: secret\r\nX-Forwarded-For: 10.0.0.1\r\nX-End-To-End: yes\r\n\r\n" cli-tx in
                        let _s0 = darwin-send-dma cli-fd cli-tx req-len in
                        let nr-p = recv-some rx p-cli-fd p-rx 4096 cqe-buf 64 in
                        let rv = h1-parse-request-dma p-rx 0 (wrapping-i32-to-u32 nr-p) 0 hdrs-buf in
                        let bid = pool-select-backend pool tbl0 (maglev-hash-key 1) 7 1000000 in
                        let acq = pool-acquire pool bid 1000000 in
                        case darwin-tcp-connect-nonblock (pool-backend-ip pool bid) (pool-backend-port pool bid) of
                          Err _ -> 7
                          Ok p-up-fd ->
                            let _att = pool-attach-fd pool bid acq.par-slot p-up-fd pool-proto-h1 in
                            let _p2 = darwin-reactor-poll rx 200000000 cqe-buf 16 in
                            let b-srv-fd = accept-loop rx backend-lfd cqe-buf 64 in
                            if not (fd-is-valid b-srv-fd) then 8
                            else
                              let _a5 = darwin-reactor-arm-read rx b-srv-fd (io-token-pack op-code-recv 1 5) in
                              let _a6 = darwin-reactor-arm-read rx p-up-fd (io-token-pack op-code-recv 1 6) in
                              let _p3 = darwin-reactor-poll rx 0 cqe-buf 16 in
                              -- Proxy rewrites L7 headers & forwards to live backend
                              let ip-len = copy-str-to-dma "127.0.0.1" ip-buf in
                              let fwd-len = proxy-rewrite-request-dma rv p-rx p-tx 0 4096 ip-buf ip-len False False in
                              let _s1 = darwin-send-dma p-up-fd p-tx fwd-len in
                              let nr-b = recv-some rx b-srv-fd b-rx 4096 cqe-buf 64 in
                              let b-got-len = wrapping-i32-to-u32 nr-b in
                              let ok-backend-saw = (
                                gt-i32 nr-b 0
                                  && dma-contains-str b-rx b-got-len "GET /hello?x=1 HTTP/1.1\r\n"
                                  && dma-contains-str b-rx b-got-len "X-Forwarded-For: 10.0.0.1, 127.0.0.1\r\n"
                                  && dma-contains-str b-rx b-got-len "X-Forwarded-Proto: http\r\n"
                                  && dma-contains-str b-rx b-got-len "Via: 1.1 cinch-proxy\r\n"
                                  && dma-contains-str b-rx b-got-len "X-End-To-End: yes\r\n"
                                  && not (dma-contains-str b-rx b-got-len "Keep-Alive:")
                                  && not (dma-contains-str b-rx b-got-len "Proxy-Connection:")
                                  && not (dma-contains-str b-rx b-got-len "X-Drop-Me:")) in
                              -- Backend sends response back to proxy
                              let resp-len = copy-str-to-dma "HTTP/1.1 200 OK\r\nContent-Length: 18\r\nKeep-Alive: timeout=5\r\nX-Backend: b0\r\n\r\nhello-from-backend" b-tx in
                              let _s2 = darwin-send-dma b-srv-fd b-tx resp-len in
                              let nr-pu = recv-some rx p-up-fd p-urx 4096 cqe-buf 64 in
                              let rw-resp-len = proxy-rewrite-response-dma p-urx 0 (wrapping-i32-to-u32 nr-pu) p-ctx 0 4096 True False in
                              -- Return upstream socket to keep-alive pool for reuse in Part C!
                              let _rel-u = pool-release-idle pool bid acq.par-slot 1000500 in
                              let _rec-u = pool-record-success pool bid 50000 1000500 in
                              let _s3 = darwin-send-dma p-cli-fd p-ctx rw-resp-len in
                              let nr-c = recv-some rx cli-fd cli-rx 4096 cqe-buf 64 in
                              let c-got-len = wrapping-i32-to-u32 nr-c in
                              let ok-client-saw = (
                                gt-i32 nr-c 0
                                  && dma-contains-str cli-rx c-got-len "HTTP/1.1 200 OK\r\n"
                                  && dma-contains-str cli-rx c-got-len "Via: 1.1 cinch-proxy\r\n"
                                  && dma-contains-str cli-rx c-got-len "X-Backend: b0\r\n"
                                  && dma-contains-str cli-rx c-got-len "hello-from-backend"
                                  && not (dma-contains-str cli-rx c-got-len "Keep-Alive:")) in
                              let _cl1 = darwin-close-fd cli-fd in
                              let _cl2 = darwin-close-fd p-cli-fd in
                              let _cl3 = darwin-close-fd b-srv-fd in

                              -- =================================================================
                              -- Part B: Live macOS Loopback L4 Bidirectional DMA Relay + SHUT_WR
                              -- =================================================================
                              case darwin-tcp-connect-nonblock ipv4-loopback proxy-port of
                                Err _ -> 9
                                Ok l4-cli ->
                                  let _p4 = darwin-reactor-poll rx 200000000 cqe-buf 16 in
                                  let l4-p-cli = accept-loop rx proxy-lfd cqe-buf 64 in
                                  case darwin-tcp-connect-nonblock ipv4-loopback backend-port of
                                    Err _ -> 10
                                    Ok l4-p-up ->
                                      let _p5 = darwin-reactor-poll rx 200000000 cqe-buf 16 in
                                      let l4-b-srv = accept-loop rx backend-lfd cqe-buf 64 in
                                      let _a7 = darwin-reactor-arm-read rx l4-p-cli (io-token-pack op-code-recv 1 7) in
                                      let _a8 = darwin-reactor-arm-read rx l4-p-up (io-token-pack op-code-recv 1 8) in
                                      let _a9 = darwin-reactor-arm-read rx l4-b-srv (io-token-pack op-code-recv 1 9) in
                                      let _a10 = darwin-reactor-arm-read rx l4-cli (io-token-pack op-code-recv 1 10) in
                                      let _p6 = darwin-reactor-poll rx 0 cqe-buf 16 in
                                      let _ri = proxy-l4-relay-init relay-addr ring-c2u ring-u2c 2048 1536 512 relay-scratch 1024 in
                                      -- Client sends payload & half-closes write side (SHUT_WR)
                                      let p1-len = copy-str-to-dma "L4-CLIENT-PAYLOAD" cli-tx in
                                      let _sw1 = darwin-send-dma l4-cli cli-tx p1-len in
                                      let _sh1 = proxy-shutdown-wr l4-cli in
                                      let b-l4-len = pump-l4-until-c2u-eof rx l4-p-cli l4-p-up l4-b-srv relay-addr b-rx 0 cqe-buf 128 in
                                      -- Backend sends reply & half-closes write side (SHUT_WR)
                                      let p2-len = copy-str-to-dma "L4-BACKEND-REPLY" b-tx in
                                      let _sw2 = darwin-send-dma l4-b-srv b-tx p2-len in
                                      let _sh2 = proxy-shutdown-wr l4-b-srv in
                                      let c-l4-len = pump-l4-until-done rx l4-p-cli l4-p-up l4-cli relay-addr cli-rx 0 cqe-buf 128 in
                                      let ok-l4 = (
                                        eq-u32 b-l4-len p1-len
                                          && dma-contains-str b-rx b-l4-len "L4-CLIENT-PAYLOAD"
                                          && eq-u32 c-l4-len p2-len
                                          && dma-contains-str cli-rx c-l4-len "L4-BACKEND-REPLY"
                                          && proxy-l4-relay-upstream-shut-wr relay-addr
                                          && proxy-l4-relay-client-shut-wr relay-addr
                                          && eq-u64 (proxy-l4-relay-bytes-c2u relay-addr) (u32-to-u64 p1-len)
                                          && eq-u64 (proxy-l4-relay-bytes-u2c relay-addr) (u32-to-u64 p2-len)) in
                                      let _c4 = darwin-close-fd l4-cli in
                                      let _c5 = darwin-close-fd l4-p-cli in
                                      let _c6 = darwin-close-fd l4-p-up in
                                      let _c7 = darwin-close-fd l4-b-srv in
                                      let _c8 = darwin-close-fd p-up-fd in
                                      let _c9 = darwin-close-fd proxy-lfd in
                                      let _c10 = darwin-close-fd backend-lfd in
                                      let _drx = darwin-reactor-destroy rx in

                                      -- =================================================================
                                      -- Part C: 50,000-Iteration Steady-State Zero-Alloc Hot Loop
                                      -- =================================================================
                                      let hot-len = copy-str-to-dma "GET /api/item?id=7 HTTP/1.1\r\nHost: edge.cinch.dev\r\nConnection: keep-alive\r\nKeep-Alive: timeout=5\r\nX-Trace: abc\r\n\r\n" p-rx in
                                      let _rst = engine-thread-counters-reset () in
                                      let done = proxy-hot-loop50000 rtr pool p-rx hot-len hdrs-buf p-tx ip-buf ip-len 0 50000 in
                                      let allocs = engine-thread-alloc-count () in
                                      let atomic-rcs = engine-thread-atomic-rc-count () in
                                      let _dp = pool-destroy pool in
                                      let _dr = maglev-router-destroy rtr in
                                      let _fdma = engine-dma-free dma 65536 in
                                      if not ok-backend-saw then 20
                                      else if not ok-client-saw then 21
                                      else if not ok-l4 then 22
                                      else if ne-u32 done 50000 then 23
                                      else if ne-u64 allocs 0 then 24
                                      else if ne-u64 atomic-rcs 0 then 25
                                      else 0
"#;
    build_and_run_leak_checked("live-l7-l4-and-50000-hot", src, 0);
}
