//! Integration tests for Wave 5 `Store.Lsm` building blocks
//! (`Store.Lsm.Bloom` and `Store.Lsm.BufferPool`):
//!
//!   1. Stdlib package manifest registration (`seed/prelude/.cinchpkg`),
//!      canonical documentation, POD ABI layout, and `#[no-alloc]` audit.
//!   2. **512-bit Cache-Line-Aligned Blocked Bloom Filter (`Store.Lsm.Bloom`)**:
//!      10,000 keys inserted -> 0 false negatives, empirical false-positive
//!      rate over 10,000 disjoint absent keys (< 2.5% at 10 bits/key), 64B
//!      cache-line alignment, and CRC32C serialize/attach round-trip + tamper
//!      rejection.
//!   3. **S3-FIFO `O_DIRECT` Buffer Pool (`Store.Lsm.BufferPool`)**:
//!      Zipfian hot working set + large one-hit sequential scan proving
//!      scan resistance (> 85% hot-set hit rate vs pure FIFO thrashing),
//!      strict `pin-count > 0` eviction immunity, Ghost queue `G` promotion
//!      into `M`, dirty-victim tracking, and clean `pool-evict-one`.
//!   4. **Prefix-Compressed SSTable 4 KiB Block Codec (`Store.Lsm.BufferPool`)**:
//!      packs 200 sorted keys (with shared prefixes, MVCC seq-nos, and
//!      tombstones) into a single 4 KiB DMA block -> verifies CRC32C
//!      integrity + 1-bit tamper rejection across data/restarts/footer/CRC,
//!      exact binary-search lookup of all 200 present keys, and absent
//!      boundary keys.
//!   5. **Steady-State Zero-Allocation Hot Loop**:
//!      100,000 Bloom probes + 50,000 S3-FIFO cache hits + 25,000 SSTable
//!      block binary searches asserting `allocs == 0`, `atomic_rc == 0`, and
//!      `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::drive_with_profile_opts` / `common::check_interfaces`
//! so the exact same suite runs against both the Rust seed and Stage 3.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const STORE_LSM_BLOOM: &str = include_str!("../prelude/Store/Lsm/Bloom.cinch");
const STORE_LSM_BUFFER_POOL: &str = include_str!("../prelude/Store/Lsm/BufferPool.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-store-lsm-parts-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("lsm-parts-{tag}"));
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
        .env("CINCH_HEAP_LIMIT", "256M")
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
// 1. Manifest registration, documentation, POD ABI & #[no-alloc] audit
// =========================================================================

#[test]
fn store_lsm_parts_manifest_pod_abi_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Store.Lsm.Bloom", "Store/Lsm/Bloom.cinch"),
        ("Store.Lsm.BufferPool", "Store/Lsm/BufferPool.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, path);
        assert!(
            pkg.order.iter().any(|n| n == name),
            "{name} missing from seed/prelude/.cinchpkg order"
        );
    }

    let pos = |name: &str| {
        pkg.order
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} missing from order"))
    };
    assert!(pos("Data.Wide") < pos("Store.Lsm.Bloom"));
    assert!(pos("Engine.Simd.Crc32c") < pos("Store.Lsm.Bloom"));
    assert!(pos("Engine.Types") < pos("Store.Lsm.Bloom"));
    assert!(pos("Engine.Simd.Crc32c") < pos("Store.Lsm.BufferPool"));
    assert!(pos("Engine.Types") < pos("Store.Lsm.BufferPool"));

    common::assert_documented("Store.Lsm.Bloom", STORE_LSM_BLOOM);
    common::assert_documented("Store.Lsm.BufferPool", STORE_LSM_BUFFER_POOL);

    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Prim.Atomic
  import Store.Lsm.Bloom
  import Store.Lsm.BufferPool

  -- Compile-time #[no-alloc] audit over every hot-path entry point in
  -- Store.Lsm.Bloom and Store.Lsm.BufferPool.
  #[no-alloc]
  audit-no-alloc :: BlockedBloom -> S3FifoPool -> SstBlockBuilder -> I64 -> I64 -> I64 ->{MVar} U32
  audit-no-alloc bf pool bld key-buf ser-buf blk-buf =
    let _k0 = raw-store-u64 key-buf 0x0102030405060708 in
    let h = wyhash-raw key-buf 8 0x123456789ABCDEF0 in
    let _a0 = bloom-add-hash bf h in
    let _a1 = bloom-add-raw bf key-buf 8 in
    let ok-b0 = bloom-may-contain-hash bf h in
    let ok-b1 = bloom-may-contain-raw bf key-buf 8 in
    let ser-n = bloom-serialize-raw bf ser-buf 4096 in
    let att = bloom-attach-raw ser-buf ser-n in
    let ok-att = if bloom-is-cacheline-aligned att then bloom-may-contain-raw att key-buf 8 else eq-u32 0 1 in

    let p1 = pool-pin pool 7 42 in
    let p2 = pool-pin-hit pool 7 42 in
    let _u1 = pool-unpin pool p1.pin-frame-idx (eq-u32 0 1) in
    let _u2 = pool-unpin pool p2.pin-frame-idx (eq-u32 0 1) in
    let ok-p = if eq-u32 p1.pin-status pool-status-miss then eq-u32 p2.pin-status pool-status-hit else eq-u32 0 1 in

    let _rst = sst-block-builder-reset bld blk-buf in
    let ok-add = sst-block-builder-add-raw bld key-buf 8 key-buf 8 99 (eq-u32 0 1) in
    let fin-n = sst-block-builder-finish bld in
    let r = sst-block-open-raw blk-buf in
    let lk0 = sst-block-reader-get-raw r key-buf 8 in
    let lk1 = sst-block-get-raw blk-buf key-buf 8 in
    let ok-sst = if eq-u32 fin-n 4096 then (if eq-u32 lk0.sst-status sst-status-found then eq-u64 lk1.sst-seq-no 99 else eq-u32 0 1) else eq-u32 0 1 in

    if not ok-b0 then 1
    else if not ok-b1 then 2
    else if not ok-att then 3
    else if not ok-p then 4
    else if not ok-add then 5
    else if not ok-sst then 6
    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let bf = bloom-create 64 10 in
    let pool = s3fifo-pool-new 16 in
    let dma = engine-dma-alloc 12288 4096 in
    if not (bloom-is-valid bf) || not (s3fifo-pool-is-valid pool) || eq-u64 dma 0 then 99
    else
      let key-buf = wrapping-u64-to-i64 dma in
      let ser-buf = wrapping-add-i64 key-buf 4096 in
      let blk-buf = wrapping-add-i64 key-buf 8192 in
      let bld = sst-block-builder-new blk-buf in
      if not (sst-block-builder-is-valid bld) then
        let _0 = bloom-destroy bf in
        let _1 = s3fifo-pool-destroy pool in
        let _2 = engine-dma-free dma 12288 in
        98
      else
        let rc = audit-no-alloc bf pool bld key-buf ser-buf blk-buf in
        let _0 = sst-block-builder-destroy bld in
        let _1 = bloom-destroy bf in
        let _2 = s3fifo-pool-destroy pool in
        let _3 = engine-dma-free dma 12288 in
        int-from-i64 (u32-to-i64 rc)
"#;

    let dir = fresh_dir("iface-check");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("Store.Lsm.* typechecks cleanly with #[no-alloc]");
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    assert!(ifaces.iter().any(|i| i.module == "Main"));
    let _ = std::fs::remove_dir_all(&dir);

    build_and_run_leak_checked("manifest-pod-no-alloc", src, 0);
}

// =========================================================================
// 2. Blocked Bloom Filter: 10,000 keys -> 0% FN, < 2.5% FP, 64B align, CRC32C
// =========================================================================

#[test]
fn blocked_bloom_10k_keys_zero_fn_empirical_fp_and_crc32c_roundtrip() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Lsm.Bloom
  import Text.Show

  foreign import ccall "write" c-write :: I32 -> I64 -> I64 ->{IO} I64

  write-str :: String ->{IO} Unit
  write-str s =
    let b = string-to-bytes s in
    let _ = c-write 1 (bytes-data b) (nat-to-i64 (bytes-len b)) in
    let _ = bytes-len b in
    ()

  write-kv :: String -> U32 ->{IO} Unit
  write-kv k v =
    let _0 = write-str k in
    let _1 = write-str (int-to-string (int-from-i64 (u32-to-i64 v))) in
    write-str "\n"

  -- Write a 12-byte key `<prefix4><u64_id>` into DMA scratch buffer `buf`.
  #[inline(always)]
  #[no-alloc]
  write-key :: I64 -> U32 -> U64 ->{MVar} Unit
  write-key buf prefix id =
    let _0 = raw-store-u32 buf prefix in
    raw-store-u64 (wrapping-add-i64 buf 4) id

  #[no-alloc]
  insert-keys :: BlockedBloom -> I64 -> U32 -> U32 -> U32 ->{MVar} Unit
  insert-keys bf buf prefix i n =
    if ge-u32 i n then ()
    else
      let _w = write-key buf prefix (u32-to-u64 i) in
      let _a = bloom-add-raw bf buf 12 in
      insert-keys bf buf prefix (wrapping-add-u32 i 1) n

  #[no-alloc]
  count-hits :: BlockedBloom -> I64 -> U32 -> U32 -> U32 -> U32 ->{MVar} U32
  count-hits bf buf prefix i n acc =
    if ge-u32 i n then acc
    else
      let _w = write-key buf prefix (u32-to-u64 i) in
      let hit = bloom-may-contain-raw bf buf 12 in
      let next-acc = if hit then wrapping-add-u32 acc 1 else acc in
      count-hits bf buf prefix (wrapping-add-u32 i 1) n next-acc

  main :: Unit ->{IO, MVar} Int
  main _ =
    let bf = bloom-create 10000 10 in
    -- 10,000 * 10 = 100,000 bits -> ceil(100000 / 512) = 196 blocks (12,544B + 80B = 12,624B)
    let dma = engine-dma-alloc 20480 4096 in
    if not (bloom-is-valid bf) || not (bloom-is-cacheline-aligned bf) || eq-u64 dma 0 then 99
    else
      let key-buf = wrapping-u64-to-i64 dma in
      let ser-buf = wrapping-add-i64 key-buf 4096 in
      let expected-blocks = bloom-blocks-for-keys 10000 10 in
      let _ins = insert-keys bf key-buf 0x50524553 0 10000 in
      let present-hits = count-hits bf key-buf 0x50524553 0 10000 0 in
      let fp-count = count-hits bf key-buf 0x4142534E 0 10000 0 in

      -- Serialize to 4 KiB-aligned DMA buffer and attach zero-copy
      let ser-bytes = bloom-serialize-raw bf ser-buf 16384 in
      let expected-ser = bloom-serialized-size expected-blocks in
      let att = bloom-attach-raw ser-buf ser-bytes in
      let att-aligned = bloom-is-cacheline-aligned att in
      let att-present = count-hits att key-buf 0x50524553 0 10000 0 in
      let att-fp = count-hits att key-buf 0x4142534E 0 10000 0 in

      -- Tamper checks: flip 1 bit in header, payload, and CRC footer -> all MUST fail attach!
      let b-hdr = raw-load-u8 (wrapping-add-i64 ser-buf 8) in
      let _th1 = raw-store-u8 (wrapping-add-i64 ser-buf 8) (bit-xor-u8 b-hdr 1) in
      let bad-hdr = bloom-attach-raw ser-buf ser-bytes in
      let _th2 = raw-store-u8 (wrapping-add-i64 ser-buf 8) b-hdr in

      let b-pay = raw-load-u8 (wrapping-add-i64 ser-buf 500) in
      let _tp1 = raw-store-u8 (wrapping-add-i64 ser-buf 500) (bit-xor-u8 b-pay 4) in
      let bad-pay = bloom-attach-raw ser-buf ser-bytes in
      let _tp2 = raw-store-u8 (wrapping-add-i64 ser-buf 500) b-pay in

      let crc-off = wrapping-sub-u32 ser-bytes 12 in
      let b-crc = raw-load-u8 (wrapping-add-i64 ser-buf (u32-to-i64 crc-off)) in
      let _tc1 = raw-store-u8 (wrapping-add-i64 ser-buf (u32-to-i64 crc-off)) (bit-xor-u8 b-crc 1) in
      let bad-crc = bloom-attach-raw ser-buf ser-bytes in
      let _tc2 = raw-store-u8 (wrapping-add-i64 ser-buf (u32-to-i64 crc-off)) b-crc in

      -- Unaligned attach must also be rejected
      let bad-align = bloom-attach-raw (wrapping-add-i64 ser-buf 8) (wrapping-sub-u32 ser-bytes 8) in

      let _p = write-kv "BLOOM_FP=" fp-count in
      let _d = bloom-destroy bf in
      let _f = engine-dma-free dma 20480 in
      if ne-u32 bf.bloom-num-blocks 196 then 1
      else if ne-u32 present-hits 10000 then 2
      else if ge-u32 fp-count 250 then 3
      else if ne-u32 ser-bytes expected-ser then 4
      else if not att-aligned then 5
      else if ne-u32 att-present 10000 then 6
      else if ne-u32 att-fp fp-count then 7
      else if bloom-is-valid bad-hdr then 8
      else if bloom-is-valid bad-pay then 9
      else if bloom-is-valid bad-crc then 10
      else if bloom-is-valid bad-align then 11
      else 0
"#;

    let (stdout, _stderr) = build_and_run_leak_checked("bloom-10k", src, 0);
    let fp_line = stdout
        .lines()
        .find(|l| l.starts_with("BLOOM_FP="))
        .unwrap_or_else(|| panic!("missing BLOOM_FP= in stdout:\n{stdout}"));
    let fp_count: u32 = fp_line
        .trim_start_matches("BLOOM_FP=")
        .trim()
        .parse()
        .expect("parse BLOOM_FP");
    let fp_pct = (fp_count as f64) / 100.0;
    assert!(
        fp_count < 250,
        "expected Bloom FP < 2.5% (250/10000), got {fp_count}/10000 ({fp_pct:.2}%)"
    );
    eprintln!(
        "[Store.Lsm.Bloom 10k keys @ 10 bits/key] blocks=196 (64B aligned) FN=0/10000 FP={fp_count}/10000 ({fp_pct:.2}%)"
    );
}

// =========================================================================
// 3. S3-FIFO Scan Resistance, Pin Safety, Ghost Queue G Promotion & Eviction
// =========================================================================

#[test]
fn s3fifo_scan_resistance_ghost_promotion_and_pin_safety() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Lsm.BufferPool
  import Text.Show

  foreign import ccall "write" c-write :: I32 -> I64 -> I64 ->{IO} I64

  write-str :: String ->{IO} Unit
  write-str s =
    let b = string-to-bytes s in
    let _ = c-write 1 (bytes-data b) (nat-to-i64 (bytes-len b)) in
    let _ = bytes-len b in
    ()

  write-kv :: String -> U32 ->{IO} Unit
  write-kv k v =
    let _0 = write-str k in
    let _1 = write-str (int-to-string (int-from-i64 (u32-to-i64 v))) in
    write-str "\n"

  -- Helper: pin + immediately unpin a clean page, returning 1 on hit else 0.
  #[inline(always)]
  #[no-alloc]
  access-page :: S3FifoPool -> U32 -> U32 ->{MVar} U32
  access-page pool fid pid =
    let r = pool-pin pool fid pid in
    if eq-u32 r.pin-status pool-status-hit then
      let _u = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
      1
    else if eq-u32 r.pin-status pool-status-miss then
      let _u = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
      0
    else 0

  -- Part A: Pin immunity, S->M promotion, G->M ghost promotion, dirty eviction & pool-evict-one.
  test-mechanics :: S3FifoPool ->{MVar} U32
  test-mechanics pool =
    -- pool has capacity = 10: small-cap = 1, main-cap = 9, ghost-cap = 9
    -- 1. Fill all 10 frames while keeping every frame pinned -> 11th pin MUST return pool-status-full!
    let ok-fill = fill-pinned pool 1 0 10 in
    if not ok-fill then 1
    else
      let full-res = pool-pin pool 1 99 in
      if ne-u32 full-res.pin-status pool-status-full then 2
      else
        -- Keep frame for (1, 0) double-pinned (pin-count = 2) and unpin frames for pages 1..9 (marking page 1 dirty)
        let r0-hit = pool-pin-hit pool 1 0 in
        let f0 = r0-hit.pin-frame-idx in
        let ok-p0 = eq-u32 (pool-frame-pin-count pool f0) 2 in
        let _un = unpin-range pool 1 1 10 in
        -- Thrash 50 cold scan pages (file 9, pages 0..49): pinned page (1, 0) MUST NEVER be evicted!
        let _sc = scan-pages pool 9 0 50 in
        let still-pinned = pool-contains pool 1 0 in
        let still-pc = eq-u32 (pool-frame-pin-count pool f0) 2 in
        -- Unpin (1, 0) twice cleanly
        let u0a = pool-unpin pool f0 (eq-u32 0 1) in
        let u0b = pool-unpin pool f0 (eq-u32 0 1) in
        let u0c = pool-unpin pool f0 (eq-u32 0 1) in -- extra unpin must return False!

        -- 2. Verify Ghost queue G promotion directly into Main queue M:
        -- Insert page (5, 777) -> enters S with freq=0; push cold pages to evict it from S into G.
        let r-g1 = pool-pin pool 5 777 in
        let in-s = eq-u8 (pool-frame-queue pool r-g1.pin-frame-idx) pool-queue-small in
        let _ug1 = pool-unpin pool r-g1.pin-frame-idx (eq-u32 0 0) in -- mark dirty!
        -- Push 3 cold pages to evict (5, 777) out of S (small-cap=1) into G!
        let r-ev1 = pool-pin pool 6 1 in
        let _ue1 = pool-unpin pool r-ev1.pin-frame-idx (eq-u32 0 1) in
        let saw-dirty-evict = eq-u32 r-ev1.pin-evicted-valid 1 in
        let ev-was777 = if eq-u32 r-ev1.pin-evicted-file-id 5 then eq-u32 r-ev1.pin-evicted-page-no 777 else eq-u32 0 1 in
        let ev-was-dirty = eq-u32 r-ev1.pin-evicted-dirty 1 in
        let in-ghost = pool-ghost-contains pool 5 777 in
        -- Re-pin (5, 777): MUST hit Ghost queue G and land DIRECTLY in Main queue M!
        let gh-before = pool-ghost-hits pool in
        let r-g2 = pool-pin pool 5 777 in
        let promoted-m = eq-u8 (pool-frame-queue pool r-g2.pin-frame-idx) pool-queue-main in
        let gh-after = pool-ghost-hits pool in
        let _ug2 = pool-unpin pool r-g2.pin-frame-idx (eq-u32 0 1) in

        -- 3. Verify explicit clean pool-evict-one returns a valid frame to free stack
        let free-before = pool-free-count pool in
        let freed = pool-evict-one pool in
        let free-after = pool-free-count pool in

        if not ok-p0 then 3
        else if not still-pinned || not still-pc then 4
        else if not u0a || not u0b || u0c then 5
        else if not in-s then 6
        else if not saw-dirty-evict || not ev-was777 || not ev-was-dirty then 7
        else if not in-ghost then 8
        else if not promoted-m || ne-u64 gh-after (wrapping-add-u64 gh-before 1) then 9
        else if eq-u32 freed s3fifo-invalid-idx || ne-u32 free-after (wrapping-add-u32 free-before 1) then 10
        else 0

  #[no-alloc]
  fill-pinned :: S3FifoPool -> U32 -> U32 -> U32 ->{MVar} Bool
  fill-pinned pool fid i n =
    if ge-u32 i n then eq-u32 0 0
    else
      let r = pool-pin pool fid i in
      if ne-u32 r.pin-status pool-status-miss then eq-u32 0 1
      else fill-pinned pool fid (wrapping-add-u32 i 1) n

  #[no-alloc]
  unpin-range :: S3FifoPool -> U32 -> U32 -> U32 ->{MVar} Unit
  unpin-range pool fid i n =
    if ge-u32 i n then ()
    else
      let r = pool-pin-hit pool fid i in
      let _u1 = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
      let _u2 = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
      unpin-range pool fid (wrapping-add-u32 i 1) n

  #[no-alloc]
  scan-pages :: S3FifoPool -> U32 -> U32 -> U32 ->{MVar} Unit
  scan-pages pool fid i n =
    if ge-u32 i n then ()
    else
      let _ = access-page pool fid i in
      scan-pages pool fid (wrapping-add-u32 i 1) n

  -- Simple deterministic Zipfian-like hot page selector in `0 .. 69`:
  -- 60% of accesses hit pages `0..14`, 30% hit `15..39`, 10% hit `40..69`.
  #[inline(always)]
  #[no-alloc]
  pick-zipf-hot :: U64 -> U32
  pick-zipf-hot rng =
    let bucket = rem-u64 rng 100 in
    let r2 = wrapping-shr-u64 rng 16 in
    if lt-u64 bucket 60 then wrapping-u64-to-u32 (rem-u64 r2 15)
    else if lt-u64 bucket 90 then wrapping-add-u32 15 (wrapping-u64-to-u32 (rem-u64 r2 25))
    else wrapping-add-u32 40 (wrapping-u64-to-u32 (rem-u64 r2 30))

  #[inline(always)]
  #[no-alloc]
  xorshift64 :: U64 -> U64
  xorshift64 s0 =
    let s1 = bit-xor-u64 s0 (wrapping-shl-u64 s0 13) in
    let s2 = bit-xor-u64 s1 (wrapping-shr-u64 s1 7) in
    bit-xor-u64 s2 (wrapping-shl-u64 s2 17)

  -- Warm up 70 hot pages (`file 1, 0..69`) so they promote into Main queue M.
  #[no-alloc]
  warmup-hot :: S3FifoPool -> U32 -> U32 -> U64 ->{MVar} U64
  warmup-hot pool i n rng =
    if ge-u32 i n then rng
    else
      let r1 = xorshift64 rng in
      let pid = if lt-u32 i 70 then i else if lt-u32 i 140 then wrapping-sub-u32 i 70 else pick-zipf-hot r1 in
      let _ = access-page pool 1 pid in
      warmup-hot pool (wrapping-add-u32 i 1) n r1

  -- Interleaved workload: for 2,000 steps, every 2nd op accesses a Zipfian hot
  -- page (`file 1, 0..69`), and every other op streams a unique one-hit cold
  -- sequential scan page (`file 2, 0..999`, 10x larger than pool capacity 100!).
  -- Returns number of hot-page hits out of 1,000 hot accesses.
  #[no-alloc]
  run-scan-workload :: S3FifoPool -> U32 -> U32 -> U64 -> U32 ->{MVar} U32
  run-scan-workload pool step total rng hot-hits =
    if ge-u32 step total then hot-hits
    else
      let r1 = xorshift64 rng in
      -- One-hit sequential scan page (never repeated -> enters S with freq=0, evicts out of S!)
      let _scan = access-page pool 2 step in
      -- Zipfian hot working-set access
      let hot-pid = pick-zipf-hot r1 in
      let h = access-page pool 1 hot-pid in
      run-scan-workload pool (wrapping-add-u32 step 1) total r1 (wrapping-add-u32 hot-hits h)

  -- Probe all 70 hot pages once after a pure 500-page sequential scan burst!
  #[no-alloc]
  probe-hot-set :: S3FifoPool -> U32 -> U32 -> U32 ->{MVar} U32
  probe-hot-set pool i n acc =
    if ge-u32 i n then acc
    else
      let h = access-page pool 1 i in
      probe-hot-set pool (wrapping-add-u32 i 1) n (wrapping-add-u32 acc h)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let p-small = s3fifo-pool-new 10 in
    let p-work = s3fifo-pool-new 100 in
    if not (s3fifo-pool-is-valid p-small) || not (s3fifo-pool-is-valid p-work) then 99
    else
      let rc-mech = test-mechanics p-small in
      let _d1 = s3fifo-pool-destroy p-small in
      if ne-u32 rc-mech 0 then
        let _d2 = s3fifo-pool-destroy p-work in
        int-from-i64 (u32-to-i64 rc-mech)
      else
        -- Warm up 70 hot pages into M (capacity=100, S=10, M=90)
        let rng1 = warmup-hot p-work 0 600 0xCAFEBABE12345678 in
        -- Measure hot-set hit rate across 1,000 interleaved Zipfian hot + 1,000 one-hit scan pages
        let hot-hits = run-scan-workload p-work 0 1000 rng1 0 in
        -- Now run a pure 500-page sequential scan burst (5x pool size! Pure FIFO loses 100% of hot set!)
        let _burst = scan-pages p-work 3 0 500 in
        let post-burst-hits = probe-hot-set p-work 0 70 0 in
        let _p1 = write-kv "S3FIFO_HOT_HITS=" hot-hits in
        let _p2 = write-kv "S3FIFO_POST_BURST=" post-burst-hits in
        let _d2 = s3fifo-pool-destroy p-work in
        -- Assert > 85% hot-set hit rate during 10x scan AND > 85% retention after 5x pure scan burst!
        if lt-u32 hot-hits 850 then 20
        else if lt-u32 post-burst-hits 60 then 21
        else 0
"#;

    let (stdout, _stderr) = build_and_run_leak_checked("s3fifo-scan", src, 0);
    let hot_hits: u32 = stdout
        .lines()
        .find(|l| l.starts_with("S3FIFO_HOT_HITS="))
        .unwrap_or_else(|| panic!("missing S3FIFO_HOT_HITS= in stdout:\n{stdout}"))
        .trim_start_matches("S3FIFO_HOT_HITS=")
        .trim()
        .parse()
        .expect("parse S3FIFO_HOT_HITS");
    let post_burst: u32 = stdout
        .lines()
        .find(|l| l.starts_with("S3FIFO_POST_BURST="))
        .unwrap_or_else(|| panic!("missing S3FIFO_POST_BURST= in stdout:\n{stdout}"))
        .trim_start_matches("S3FIFO_POST_BURST=")
        .trim()
        .parse()
        .expect("parse S3FIFO_POST_BURST");
    let hot_pct = (hot_hits as f64) / 10.0;
    let burst_pct = (post_burst as f64) * 100.0 / 70.0;
    assert!(hot_pct > 85.0, "expected hot hit rate > 85%, got {hot_pct:.1}%");
    assert!(
        burst_pct > 85.0,
        "expected post-scan-burst retention > 85%, got {burst_pct:.1}%"
    );
    eprintln!(
        "[Store.Lsm.BufferPool S3-FIFO] cap=100 (S=10,M=90,G=90) interleaved_hot_hit_rate={hot_pct:.1}% ({hot_hits}/1000 vs 1000-page scan) post_500p_scan_burst_retention={burst_pct:.1}% ({post_burst}/70 vs 0% pure FIFO)"
    );
}

// =========================================================================
// 4. Prefix-Compressed SSTable 4 KiB Block Codec (200 sorted keys + CRC32C)
// =========================================================================

#[test]
fn sst_block_200_sorted_keys_prefix_compression_crc32c_and_bsearch() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Lsm.BufferPool
  import Text.Show

  foreign import ccall "write" c-write :: I32 -> I64 -> I64 ->{IO} I64

  write-str :: String ->{IO} Unit
  write-str s =
    let b = string-to-bytes s in
    let _ = c-write 1 (bytes-data b) (nat-to-i64 (bytes-len b)) in
    let _ = bytes-len b in
    ()

  write-kv :: String -> U32 ->{IO} Unit
  write-kv k v =
    let _0 = write-str k in
    let _1 = write-str (int-to-string (int-from-i64 (u32-to-i64 v))) in
    write-str "\n"

  -- Write a 24-byte sorted key `"tenant/alpha/record/NNNN"` (20B shared prefix!).
  #[no-alloc]
  write-sst-key :: I64 -> U32 ->{MVar} Unit
  write-sst-key buf idx =
    -- "tenant/alpha/record/" = 20 bytes
    let _0 = raw-store-u64 buf 0x612F746E616E6574 in                  -- "tenant/a"
    let _1 = raw-store-u64 (wrapping-add-i64 buf 8) 0x6365722F6168706C in -- "lpha/rec"
    let _2 = raw-store-u32 (wrapping-add-i64 buf 16) 0x2F64726F in    -- "ord/"
    let d3 = div-u32 idx 1000 in
    let r3 = rem-u32 idx 1000 in
    let d2 = div-u32 r3 100 in
    let r2 = rem-u32 r3 100 in
    let d1 = div-u32 r2 10 in
    let d0 = rem-u32 r2 10 in
    let _3 = raw-store-u8 (wrapping-add-i64 buf 20) (wrapping-add-u8 48 (wrapping-u32-to-u8 d3)) in
    let _4 = raw-store-u8 (wrapping-add-i64 buf 21) (wrapping-add-u8 48 (wrapping-u32-to-u8 d2)) in
    let _5 = raw-store-u8 (wrapping-add-i64 buf 22) (wrapping-add-u8 48 (wrapping-u32-to-u8 d1)) in
    raw-store-u8 (wrapping-add-i64 buf 23) (wrapping-add-u8 48 (wrapping-u32-to-u8 d0))

  -- Pack 200 sorted keys (keys 0000..0199) into one 4 KiB DMA block.
  -- Every 10th key (`idx % 10 == 0`) is a tombstone (`val-len = 0`);
  -- live keys carry an 8-byte little-endian value `0x56414C0000000000 | idx` and `seq-no = 1000 + idx * 7`.
  #[no-alloc]
  pack200 :: SstBlockBuilder -> I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  pack200 bld key-buf val-buf i n =
    if ge-u32 i n then eq-u32 0 0
    else
      let _wk = write-sst-key key-buf i in
      let is-tomb = eq-u32 (rem-u32 i 10) 0 in
      let seq = wrapping-add-u64 1000 (wrapping-mul-u64 (u32-to-u64 i) 7) in
      let vlen = if is-tomb then 0 else 8 in
      let _wv = raw-store-u64 val-buf (bit-or-u64 0x56414C0000000000 (u32-to-u64 i)) in
      let ok = sst-block-builder-add-raw bld key-buf 24 val-buf vlen seq is-tomb in
      if not ok then eq-u32 0 1
      else pack200 bld key-buf val-buf (wrapping-add-u32 i 1) n

  -- Verify all 200 keys via `sst-block-get-raw` (CRC32C + binary search + <= 16 scan).
  #[no-alloc]
  verify-all200 :: I64 -> I64 -> U32 -> U32 ->{MVar} U32
  verify-all200 blk key-buf i n =
    if ge-u32 i n then 0
    else
      let _wk = write-sst-key key-buf i in
      let res = sst-block-get-raw blk key-buf 24 in
      let is-tomb = eq-u32 (rem-u32 i 10) 0 in
      let exp-seq = wrapping-add-u64 1000 (wrapping-mul-u64 (u32-to-u64 i) 7) in
      if is-tomb then
        if ne-u32 res.sst-status sst-status-tombstone then 10
        else if ne-u32 res.sst-is-tombstone 1 then 11
        else if ne-u32 res.sst-val-len 0 then 12
        else if ne-u64 res.sst-seq-no exp-seq then 13
        else verify-all200 blk key-buf (wrapping-add-u32 i 1) n
      else
        let exp-val = bit-or-u64 0x56414C0000000000 (u32-to-u64 i) in
        if ne-u32 res.sst-status sst-status-found then 14
        else if ne-u32 res.sst-is-tombstone 0 then 15
        else if ne-u32 res.sst-val-len 8 then 16
        else if ne-u64 res.sst-seq-no exp-seq then 17
        else if ne-u64 (raw-load-u64 res.sst-val-addr) exp-val then 18
        else verify-all200 blk key-buf (wrapping-add-u32 i 1) n

  -- Flip 1 bit at `off`, assert `sst-block-get-raw` returns `sst-status-bad-crc`, then restore byte.
  #[no-alloc]
  check-tamper1 :: I64 -> I64 -> U32 ->{MVar} Bool
  check-tamper1 blk key-buf off =
    let p = wrapping-add-i64 blk (u32-to-i64 off) in
    let orig = raw-load-u8 p in
    let _f = raw-store-u8 p (bit-xor-u8 orig 0x08) in
    let res = sst-block-get-raw blk key-buf 24 in
    let _r = raw-store-u8 p orig in
    eq-u32 res.sst-status sst-status-bad-crc

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let blk = wrapping-u64-to-i64 dma in
      let key-buf = wrapping-add-i64 blk 4096 in
      let val-buf = wrapping-add-i64 blk 4200 in
      let bld = sst-block-builder-new blk in
      if not (sst-block-builder-is-valid bld) then
        let _ = engine-dma-free dma 8192 in 98
      else
        let ok-pack = pack200 bld key-buf val-buf 0 200 in
        let cnt = sst-block-builder-count bld in
        let used = sst-block-builder-used-bytes bld in
        let nrest = sst-block-builder-restart-count bld in
        let fin = sst-block-builder-finish bld in
        let _db = sst-block-builder-destroy bld in

        let crc-ok = sst-block-verify-crc blk in
        let rc200 = verify-all200 blk key-buf 0 200 in

        -- Absent boundary keys:
        -- (a) Strictly before key 0000: "aaaa"
        let _a0 = raw-store-u32 key-buf 0x61616161 in
        let abs-before = eq-u32 (sst-block-get-raw blk key-buf 4).sst-status sst-status-not-found in
        -- (b) Prefix of key 0000: "tenant/alpha/record/000" (23 bytes)
        let _a1 = write-sst-key key-buf 0 in
        let abs-prefix = eq-u32 (sst-block-get-raw blk key-buf 23).sst-status sst-status-not-found in
        -- (c) Between key 0015 and restart boundary key 0016: "tenant/alpha/record/0015x" (25 bytes)
        let _a2 = write-sst-key key-buf 15 in
        let _a2b = raw-store-u8 (wrapping-add-i64 key-buf 24) 120 in
        let abs-mid1 = eq-u32 (sst-block-get-raw blk key-buf 25).sst-status sst-status-not-found in
        -- (d) Right after restart boundary key 0032: "tenant/alpha/record/0032a" (25 bytes)
        let _a3 = write-sst-key key-buf 32 in
        let _a3b = raw-store-u8 (wrapping-add-i64 key-buf 24) 97 in
        let abs-mid2 = eq-u32 (sst-block-get-raw blk key-buf 25).sst-status sst-status-not-found in
        -- (e) Strictly after last key 0199: "tenant/alpha/record/0200"
        let _a4 = write-sst-key key-buf 200 in
        let abs-after = eq-u32 (sst-block-get-raw blk key-buf 24).sst-status sst-status-not-found in

        -- 1-bit tamper rejection across data, middle, restart table, footer, and CRC trailer
        let _wk0 = write-sst-key key-buf 42 in
        let t0 = check-tamper1 blk key-buf 0 in
        let t1 = check-tamper1 blk key-buf 1500 in
        let t2 = check-tamper1 blk key-buf 4060 in
        let t3 = check-tamper1 blk key-buf 4088 in
        let t4 = check-tamper1 blk key-buf 4094 in

        let _p = write-kv "SST_USED=" used in
        let _f = engine-dma-free dma 8192 in
        if not ok-pack then 1
        else if ne-u32 cnt 200 then 2
        else if ne-u32 nrest 13 then 3
        else if ne-u32 fin 4096 then 4
        else if not crc-ok then 5
        else if ne-u32 rc200 0 then int-from-i64 (u32-to-i64 rc200)
        else if not abs-before || not abs-prefix || not abs-mid1 || not abs-mid2 || not abs-after then 20
        else if not t0 || not t1 || not t2 || not t3 || not t4 then 21
        else 0
"#;

    let (stdout, _stderr) = build_and_run_leak_checked("sst-block-200", src, 0);
    let used: u32 = stdout
        .lines()
        .find(|l| l.starts_with("SST_USED="))
        .unwrap_or_else(|| panic!("missing SST_USED= in stdout:\n{stdout}"))
        .trim_start_matches("SST_USED=")
        .trim()
        .parse()
        .expect("parse SST_USED");
    // Uncompressed 200 * 24B keys + 180 * 8B values = 6,240 bytes; prefix compression packs into < 3,800B!
    assert!(
        used < 4000,
        "expected prefix-compressed 200 entries to fit cleanly in 4 KiB, got used={used}"
    );
    eprintln!(
        "[Store.Lsm.BufferPool SST 4KiB block] 200 sorted keys (20B shared prefix, 20 tombstones, 13 restarts) compressed_bytes={used}/4096 (vs 6240B raw) CRC32C=verified"
    );
}

// =========================================================================
// 5. Steady-State Zero-Allocation Hot Loop:
//    100,000 Bloom probes + 50,000 S3-FIFO hits + 25,000 SST binary searches
//    asserting allocs == 0, atomic_rc == 0, and leaked == 0
// =========================================================================

#[test]
fn steady_state_bloom_s3fifo_sst_zero_alloc_hot_loop() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Prim.Atomic
  import Store.Lsm.Bloom
  import Store.Lsm.BufferPool

  #[no-alloc]
  write-sst-key :: I64 -> U32 ->{MVar} Unit
  write-sst-key buf idx =
    let _0 = raw-store-u64 buf 0x612F746E616E6574 in
    let _1 = raw-store-u64 (wrapping-add-i64 buf 8) 0x6365722F6168706C in
    let _2 = raw-store-u32 (wrapping-add-i64 buf 16) 0x2F64726F in
    let d3 = div-u32 idx 1000 in
    let r3 = rem-u32 idx 1000 in
    let d2 = div-u32 r3 100 in
    let r2 = rem-u32 r3 100 in
    let d1 = div-u32 r2 10 in
    let d0 = rem-u32 r2 10 in
    let _3 = raw-store-u8 (wrapping-add-i64 buf 20) (wrapping-add-u8 48 (wrapping-u32-to-u8 d3)) in
    let _4 = raw-store-u8 (wrapping-add-i64 buf 21) (wrapping-add-u8 48 (wrapping-u32-to-u8 d2)) in
    let _5 = raw-store-u8 (wrapping-add-i64 buf 22) (wrapping-add-u8 48 (wrapping-u32-to-u8 d1)) in
    raw-store-u8 (wrapping-add-i64 buf 23) (wrapping-add-u8 48 (wrapping-u32-to-u8 d0))

  #[no-alloc]
  setup-bloom :: BlockedBloom -> I64 -> U32 -> U32 ->{MVar} Unit
  setup-bloom bf key-buf i n =
    if ge-u32 i n then ()
    else
      let _w = write-sst-key key-buf i in
      let _a = bloom-add-raw bf key-buf 24 in
      setup-bloom bf key-buf (wrapping-add-u32 i 1) n

  #[no-alloc]
  setup-pool :: S3FifoPool -> U32 -> U32 ->{MVar} Unit
  setup-pool pool i n =
    if ge-u32 i n then ()
    else
      let r = pool-pin pool 1 i in
      let _u = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
      setup-pool pool (wrapping-add-u32 i 1) n

  #[no-alloc]
  setup-sst :: SstBlockBuilder -> I64 -> I64 -> U32 -> U32 ->{MVar} Unit
  setup-sst bld key-buf val-buf i n =
    if ge-u32 i n then ()
    else
      let _wk = write-sst-key key-buf i in
      let _wv = raw-store-u64 val-buf (u32-to-u64 i) in
      let _ok = sst-block-builder-add-raw bld key-buf 24 val-buf 8 (wrapping-add-u64 1 (u32-to-u64 i)) (eq-u32 0 1) in
      setup-sst bld key-buf val-buf (wrapping-add-u32 i 1) n

  -- 100,000 zero-alloc Bloom probes
  #[no-alloc]
  hot-bloom100k :: BlockedBloom -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  hot-bloom100k bf key-buf i limit hits =
    if ge-u32 i limit then hits
    else
      let k = rem-u32 i 200 in
      let _w = write-sst-key key-buf k in
      let ok = bloom-may-contain-raw bf key-buf 24 in
      let next-hits = if ok then wrapping-add-u32 hits 1 else hits in
      hot-bloom100k bf key-buf (wrapping-add-u32 i 1) limit next-hits

  -- 50,000 zero-alloc S3-FIFO buffer-pool cache hits (`pool-pin-hit` + `pool-unpin`)
  #[no-alloc]
  hot-s3fifo50k :: S3FifoPool -> U32 -> U32 -> U32 ->{MVar} U32
  hot-s3fifo50k pool i limit hits =
    if ge-u32 i limit then hits
    else
      let pid = bit-and-u32 i 63 in
      let r = pool-pin-hit pool 1 pid in
      if eq-u32 r.pin-status pool-status-hit then
        let _u = pool-unpin pool r.pin-frame-idx (eq-u32 0 1) in
        hot-s3fifo50k pool (wrapping-add-u32 i 1) limit (wrapping-add-u32 hits 1)
      else
        hot-s3fifo50k pool (wrapping-add-u32 i 1) limit hits

  -- 25,000 zero-alloc SSTable 4 KiB block binary-search lookups (with CRC32C verification)
  #[no-alloc]
  hot-sst25k :: I64 -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  hot-sst25k blk key-buf i limit found =
    if ge-u32 i limit then found
    else
      let k = rem-u32 i 200 in
      let _w = write-sst-key key-buf k in
      let res = sst-block-get-raw blk key-buf 24 in
      let ok = if eq-u32 res.sst-status sst-status-found then eq-u64 (raw-load-u64 res.sst-val-addr) (u32-to-u64 k) else eq-u32 0 1 in
      let next-found = if ok then wrapping-add-u32 found 1 else found in
      hot-sst25k blk key-buf (wrapping-add-u32 i 1) limit next-found

  main :: Unit ->{IO, MVar} Int
  main _ =
    let bf = bloom-create 2000 10 in
    let pool = s3fifo-pool-new 64 in
    let dma = engine-dma-alloc 8192 4096 in
    if not (bloom-is-valid bf) || not (s3fifo-pool-is-valid pool) || eq-u64 dma 0 then 99
    else
      let blk = wrapping-u64-to-i64 dma in
      let key-buf = wrapping-add-i64 blk 4096 in
      let val-buf = wrapping-add-i64 blk 4200 in
      let bld = sst-block-builder-new blk in
      if not (sst-block-builder-is-valid bld) then
        let _0 = bloom-destroy bf in
        let _1 = s3fifo-pool-destroy pool in
        let _2 = engine-dma-free dma 8192 in
        98
      else
        let _sb = setup-bloom bf key-buf 0 200 in
        let _sp = setup-pool pool 0 64 in
        let _ss = setup-sst bld key-buf val-buf 0 200 in
        let _fin = sst-block-builder-finish bld in
        let _db = sst-block-builder-destroy bld in

        -- Warmup 1 iteration before resetting counters
        let _w0 = hot-bloom100k bf key-buf 0 1 0 in
        let _w1 = hot-s3fifo50k pool 0 1 0 in
        let _w2 = hot-sst25k blk key-buf 0 1 0 in

        let _rst = engine-thread-counters-reset () in
        let n-bloom = hot-bloom100k bf key-buf 0 100000 0 in
        let n-pool = hot-s3fifo50k pool 0 50000 0 in
        let n-sst = hot-sst25k blk key-buf 0 25000 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in

        let _d0 = bloom-destroy bf in
        let _d1 = s3fifo-pool-destroy pool in
        let _d2 = engine-dma-free dma 8192 in
        if ne-u32 n-bloom 100000 then 10
        else if ne-u32 n-pool 50000 then 11
        else if ne-u32 n-sst 25000 then 12
        else if ne-u64 allocs 0 then 13
        else if ne-u64 atomic-rcs 0 then 14
        else 0
"#;

    let t0 = Instant::now();
    let _ = build_and_run_leak_checked("steady-state-hot-loop", src, 0);
    let elapsed = t0.elapsed();
    eprintln!(
        "[Store.Lsm steady-state zero-alloc] 100k Bloom + 50k S3-FIFO hits + 25k SST bsearch (incl. compile+run): {:.2}s, allocs=0, atomic_rc=0, leaked=0",
        elapsed.as_secs_f64()
    );
}
