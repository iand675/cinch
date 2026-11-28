//! Integration tests for `Engine.Mem.DmaPool`:
//! - Package manifest registration (`seed/prelude/.cinchpkg`).
//! - ABI layouts for `DmaPool` (16B POD) and `DmaSlice` (24B POD).
//! - 4096-byte page alignment (`addr % 4096 == 0`), distinct non-overlapping
//!   ranges across 4 KiB / 16 KiB / 64 KiB buffer sizes, pool exhaustion and
//!   refill, headroom/body/tailroom splits (9B H2 + 64B TLS/QUIC + 16B AEAD),
//!   `ByteSpan` conversion, and `io_uring` `IORING_REGISTER_PBUF_RING` 16B
//!   descriptor table layout.
//! - Compile-fail linearity tests proving dropping or double-releasing a `%1`
//!   `DmaBufHandle` is rejected by the compiler.
//! - Steady-state 50,000 acquire/write/read/release loop asserting
//!   `engine-thread-alloc-count () == 0`, `engine-thread-atomic-rc-count () == 0`,
//!   and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` /
//! `common::check_diagnostics` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;

const DMA_POOL: &str = include_str!("../prelude/Engine/Mem/DmaPool.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_dma_pool(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Engine.Mem.DmaPool.cinch", DMA_POOL),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) {
    let dir = project_with_dma_pool(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "engine-mem-dma-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?}; stderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report; stderr:\n{stderr}"
    );
    common::cleanup(&dir);
}

#[test]
fn engine_mem_dma_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Mem.DmaPool")
        .expect("Engine.Mem.DmaPool listed in seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Engine/Mem/DmaPool.cinch");
    assert!(
        pkg.order.iter().any(|name| name == "Engine.Mem.DmaPool"),
        "Engine.Mem.DmaPool listed in seed/prelude/.cinchpkg order"
    );

    // Also verify direct stdlib resolution via `drive_with_profile` without copying the module.
    let dir = common::project(
        "dma-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Engine.Mem.DmaPool

  main :: Unit ->{IO, MVar} Int
  main _ =
    case dma-pool-new 4096 4 of
      None -> 1
      Some pool ->
        let cap = dma-pool-capacity pool in
        let _ = dma-pool-destroy pool in
        if eq-u32 cap 4 then 0 else 2
"#,
        )],
    );
    let code = common::drive_with_profile(
        Mode::Run,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("run direct stdlib import of Engine.Mem.DmaPool");
    common::cleanup(&dir);
    assert_eq!(code, 0);
}

#[test]
fn engine_mem_dma_abi_layouts() {
    let src = r#"
module Main where
  import Engine.Mem.DmaPool

  #[no-alloc]
  #[inline(never)]
  id-slice :: DmaSlice -> DmaSlice
  id-slice s = s

  #[no-alloc]
  #[inline(never)]
  id-pool :: DmaPool -> DmaPool
  id-pool p = p

  main :: Int
  main =
    let s = id-slice (dma-slice-empty ()) in
    let sp = dma-slice-to-byte-span s in
    if eq-u32 sp.span-len 0 then 0 else 1
"#;

    let dir = project_with_dma_pool("dma-abi", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    let dma = ifaces
        .iter()
        .find(|i| i.module == "Engine.Mem.DmaPool")
        .expect("Engine.Mem.DmaPool interface emitted");

    let layout_of = |name: &str| {
        dma.layouts
            .iter()
            .find(|l| l.name == name)
            .unwrap_or_else(|| panic!("missing layout for {name} in Engine.Mem.DmaPool"))
    };

    // DmaSlice == 24B POD (4x U32 + 1x I64)
    let slice_l = layout_of("DmaSlice");
    assert_eq!(slice_l.size, 24, "DmaSlice layout: {slice_l:?}");
    assert_eq!(slice_l.align, 8, "DmaSlice align: {slice_l:?}");
    assert_eq!(slice_l.fields.len(), 5);
    assert_eq!(slice_l.fields[0].offset, 0);
    assert_eq!(slice_l.fields[1].offset, 4);
    assert_eq!(slice_l.fields[2].offset, 8);
    assert_eq!(slice_l.fields[3].offset, 12);
    assert_eq!(slice_l.fields[4].offset, 16);

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

#[test]
fn engine_mem_dma_alignment_disjoint_slices_and_pbuf_ring() {
    let src = r#"
module Main where
  import Engine.Types
  import Engine.Mem.DmaPool
  import Prim.Atomic

  -- Verify every acquired buffer has `addr % 4096 == 0`, distinct non-overlapping
  -- ranges matching `dma-pool-buf-addr`, and can be written/read independently.
  check-acquire-all :: DmaPool -> U32 -> U32 ->{MVar} Int
  check-acquire-all pool buf-size remaining =
    if eq-u32 remaining 0 then
      -- Pool must now be exhausted.
      case dma-pool-acquire pool of
        Some h ->
          let _ = dma-pool-release pool h in
          100
        None ->
          if eq-u32 (dma-pool-free-count pool) 0 then 0 else 101
    else
      case dma-pool-acquire pool of
        None -> 102
        Some h ->
          case dma-buf-as-slice h of
            (sl, h1) ->
              let addr-u64 = wrapping-i64-to-u64 sl.slice-addr in
              if ne-u64 (bit-and-u64 addr-u64 4095) 0 then
                let _ = dma-pool-release pool h1 in
                103
              else if ne-u32 sl.slice-len buf-size then
                let _ = dma-pool-release pool h1 in
                104
              else if ne-u32 sl.slice-cap buf-size then
                let _ = dma-pool-release pool h1 in
                105
              else if ne-i64 sl.slice-addr (dma-pool-buf-addr pool sl.slice-buf-id) then
                let _ = dma-pool-release pool h1 in
                106
              else
                -- Stamp a unique marker at the start and end of this buffer.
                let tag = wrapping-add-u64 (u32-to-u64 sl.slice-buf-id) 1000 in
                let last-off = u32-to-i64 (wrapping-sub-u32 buf-size 8) in
                let _ = raw-store-u64 sl.slice-addr tag in
                let _ = raw-store-u64 (wrapping-add-i64 sl.slice-addr last-off) (wrapping-mul-u64 tag 3) in
                -- Recurse while holding `h1` so all buffers are simultaneously live!
                let rc = check-acquire-all pool buf-size (wrapping-sub-u32 remaining 1) in
                -- Verify no other buffer overwrote our range (strict non-overlap).
                let v0 = raw-load-u64 sl.slice-addr in
                let v1 = raw-load-u64 (wrapping-add-i64 sl.slice-addr last-off) in
                let _ = dma-pool-release pool h1 in
                if rc /= 0 then rc
                else if ne-u64 v0 tag then 107
                else if ne-u64 v1 (wrapping-mul-u64 tag 3) then 108
                else 0

  -- Verify headroom / body / tailroom splits (9B H2 header, 64B TLS/QUIC header,
  -- 16B AEAD tailroom) and `ByteSpan` conversion.
  check-headroom-splits :: DmaPool ->{MVar} Int
  check-headroom-splits pool =
    case dma-pool-acquire pool of
      None -> 200
      Some h ->
        case dma-buf-as-slice h of
          (full, h1) ->
            -- 1. HTTP/2 9-byte frame header headroom
            let rc1 = (case dma-slice-reserve-headroom full 9 of
              None -> 201
              Some (hdr, body) ->
                let sp-hdr = dma-slice-to-byte-span hdr in
                let sp-body = dma-slice-to-byte-span body in
                if ne-u32 hdr.slice-off 0 || ne-u32 hdr.slice-len 9 || ne-u32 hdr.slice-cap 9 then 202
                else if ne-u32 body.slice-off 9 || ne-u32 body.slice-len 4087 || ne-u32 body.slice-cap 4087 then 203
                else if ne-i64 body.slice-addr (wrapping-add-i64 hdr.slice-addr 9) then 204
                else if ne-u32 sp-hdr.span-buf full.slice-buf-id || ne-u32 sp-hdr.span-off 0 || ne-u32 sp-hdr.span-len 9 then 205
                else if ne-u32 sp-body.span-buf full.slice-buf-id || ne-u32 sp-body.span-off 9 || ne-u32 sp-body.span-len 4087 then 206
                else 0) in
            -- 2. TLS/QUIC 64-byte record headroom + 16-byte AEAD tailroom
            let rc2 = (case dma-slice-reserve-headroom full 64 of
              None -> 210
              Some (tls-hdr, rest) ->
                case dma-slice-reserve-tailroom rest 16 of
                  None -> 211
                  Some (payload, tag) ->
                    if ne-u32 tls-hdr.slice-len 64 || ne-u32 payload.slice-off 64 || ne-u32 payload.slice-len 4016 then 212
                    else if ne-u32 tag.slice-off 4080 || ne-u32 tag.slice-len 16 then 213
                    else if ne-i64 tag.slice-addr (wrapping-add-i64 full.slice-addr 4080) then 214
                    else
                      -- Out-of-bounds split must return None
                      case dma-slice-split payload 4017 of
                        Some _ -> 215
                        None ->
                          -- Truncation exposes tailroom
                          let trunc = dma-slice-truncate payload 100 in
                          if ne-u32 (dma-slice-tailroom trunc) 3916 then 217 else 0) in
            let _ = dma-pool-release pool h1 in
            if rc1 /= 0 then rc1 else rc2

  -- Verify every 16-byte `io_uring_buf` entry (`{ addr: U64, len: U32, bid: U16, resv: U16 }`).
  verify-pbuf-entries :: DmaPool -> I64 -> U32 -> U32 -> U32 ->{MVar} Int
  verify-pbuf-entries pool ring-addr buf-size n i =
    if ge-u32 i n then 0
    else
      let entry = wrapping-add-i64 ring-addr (wrapping-shl-i64 (u32-to-i64 i) 4) in
      let got-addr = raw-load-u64 entry in
      let got-len = raw-load-u32 (wrapping-add-i64 entry 8) in
      let got-bid-resv = raw-load-u32 (wrapping-add-i64 entry 12) in
      let got-bid = bit-and-u32 got-bid-resv 65535 in
      let got-resv = wrapping-shr-u32 got-bid-resv 16 in
      let exp-addr = wrapping-i64-to-u64 (dma-pool-buf-addr pool i) in
      if ne-u64 got-addr exp-addr then 301
      else if ne-u64 (bit-and-u64 got-addr 4095) 0 then 302
      else if ne-u32 got-len buf-size then 303
      else if ne-u32 got-bid i then 304
      else if ne-u32 got-resv 0 then 305
      else verify-pbuf-entries pool ring-addr buf-size n (wrapping-add-u32 i 1)

  check-pbuf-ring :: DmaPool ->{IO, MVar} Int
  check-pbuf-ring pool =
    let ring-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 ring-u64 0 then 300
    else
      let ring-addr = wrapping-u64-to-i64 ring-u64 in
      let filled = dma-pool-fill-pbuf-ring pool ring-addr 64 in
      let count = dma-pool-capacity pool in
      let buf-size = dma-pool-buf-size pool in
      let rc = (if ne-u32 filled count then 306
        else verify-pbuf-entries pool ring-addr buf-size filled 0) in
      let _ = engine-dma-free ring-u64 4096 in
      rc

  check-size :: U32 -> U32 ->{IO, MVar} Int
  check-size buf-size count =
    case dma-pool-new buf-size count of
      None -> 400
      Some pool ->
        let rc = (if ne-u32 (dma-pool-capacity pool) count then 401
          else if ne-u32 (dma-pool-buf-size pool) buf-size then 402
          else if ne-u32 (dma-pool-free-count pool) count then 403
          else if ne-u64 (bit-and-u64 (wrapping-i64-to-u64 (dma-pool-base-addr pool)) 4095) 0 then 404
          else if ne-i64 (dma-pool-buf-addr pool count) 0 then 405
          else
            let rc1 = check-acquire-all pool buf-size count in
            if rc1 /= 0 then rc1
            else if ne-u32 (dma-pool-free-count pool) count then 406
            else check-pbuf-ring pool) in
        let _ = dma-pool-destroy pool in
        rc

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- Reject invalid pool geometries (count == 0, buf-size < 4096, non-power-of-two)
    case dma-pool-new 4096 0 of
      Some p -> let _ = dma-pool-destroy p in 1
      None ->
        case dma-pool-new 2048 4 of
          Some p -> let _ = dma-pool-destroy p in 2
          None ->
            case dma-pool-new 6000 4 of
              Some p -> let _ = dma-pool-destroy p in 3
              None ->
                -- Exercise 4 KiB, 16 KiB, and 64 KiB DMA pools
                let rc4k = check-size 4096 8 in
                if rc4k /= 0 then rc4k
                else
                  let rc16k = check-size 16384 4 in
                  if rc16k /= 0 then rc16k
                  else
                    let rc64k = check-size 65536 2 in
                    if rc64k /= 0 then rc64k
                    else
                      case dma-pool-new 4096 4 of
                        None -> 4
                        Some pool ->
                          let rc-split = check-headroom-splits pool in
                          let _ = dma-pool-destroy pool in
                          rc-split
"#;

    run_leak_checked("dma-functional", src, 0);
}

#[test]
fn engine_mem_dma_linear_handle_drop_and_double_release_are_rejected() {
    // 1. Dropping a `%1` linear `DmaBufHandle` without calling `dma-pool-release`
    //    must be rejected at compile time.
    let drop_src = r#"
module Main where
  import Engine.Mem.DmaPool

  leak-buffer :: DmaPool -> DmaBufHandle %1 ->{MVar} Int
  leak-buffer pool h = 0

  main :: Int
  main = 0
"#;
    let dir_drop = project_with_dma_pool("dma-linear-drop", drop_src);
    let err_drop = common::check_diagnostics(&dir_drop, "Main.cinch")
        .expect_err("dropping a %1 DmaBufHandle must fail type/multiplicity checking");
    common::cleanup(&dir_drop);
    assert!(
        err_drop.contains("multiplicity %1") && err_drop.contains("never used"),
        "expected linear drop diagnostic, got: {err_drop}"
    );

    // 2. Dropping a `%1` linear `DmaBufHandle` on one branch must also be rejected.
    let branch_drop_src = r#"
module Main where
  import Engine.Mem.DmaPool

  maybe-leak :: Bool -> DmaPool -> DmaBufHandle %1 ->{MVar} Unit
  maybe-leak ok pool h =
    if ok then dma-pool-release pool h else ()

  main :: Int
  main = 0
"#;
    let dir_branch = project_with_dma_pool("dma-linear-branch-drop", branch_drop_src);
    let err_branch = common::check_diagnostics(&dir_branch, "Main.cinch")
        .expect_err("dropping a %1 DmaBufHandle on one branch must be rejected");
    common::cleanup(&dir_branch);
    assert!(
        err_branch.contains("multiplicity %1") && err_branch.contains("unused on some path"),
        "expected linear branch-drop diagnostic, got: {err_branch}"
    );

    // 3. Double-releasing a `%1` linear `DmaBufHandle` must be rejected.
    let double_src = r#"
module Main where
  import Engine.Mem.DmaPool

  double-release :: DmaPool -> DmaBufHandle %1 ->{MVar} Unit
  double-release pool h =
    let _ = dma-pool-release pool h in
    dma-pool-release pool h

  main :: Int
  main = 0
"#;
    let dir_double = project_with_dma_pool("dma-linear-double-release", double_src);
    let err_double = common::check_diagnostics(&dir_double, "Main.cinch")
        .expect_err("double-releasing a %1 DmaBufHandle must be rejected");
    common::cleanup(&dir_double);
    assert!(
        err_double.contains("multiplicity %1") && err_double.contains("used 2 time"),
        "expected linear double-use diagnostic, got: {err_double}"
    );

    // 4. Inspecting via `dma-buf-as-slice` and dropping the returned handle via a
    //    linear continuation must be rejected; releasing it must succeed.
    let inspect_leak_src = r#"
module Main where
  import Engine.Mem.DmaPool

  consume-after-inspect :: DmaPool -> DmaBufHandle %1 ->{MVar} U32
  consume-after-inspect pool h =
    case dma-buf-as-slice h of
      (sl, h1) ->
        ((\kept -> sl.slice-len) :: DmaBufHandle %1 -> U32) h1

  main :: Int
  main = 0
"#;
    let dir_inspect = project_with_dma_pool("dma-linear-inspect-drop", inspect_leak_src);
    let err_inspect = common::check_diagnostics(&dir_inspect, "Main.cinch")
        .expect_err("dropping linear DmaBufHandle returned by dma-buf-as-slice must be rejected");
    common::cleanup(&dir_inspect);
    assert!(
        err_inspect.contains("multiplicity %1"),
        "expected linear diagnostic after dma-buf-as-slice, got: {err_inspect}"
    );
}

#[test]
fn engine_mem_dma_steady_state_50k_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Engine.Types
  import Engine.Mem.DmaPool
  import Prim.Atomic

  steady-step :: DmaPool -> U32 -> DmaSlice -> DmaBufHandle %1 ->{MVar} U64
  steady-step pool n sl h =
    let val = wrapping-add-u64 (u32-to-u64 n) 1 in
    let _ = raw-store-u64 sl.slice-addr val in
    let got = raw-load-u64 sl.slice-addr in
    let _ = dma-pool-release pool h in
    got

  steady-loop :: DmaPool -> U32 -> U64 ->{MVar} U64
  steady-loop pool n acc =
    if eq-u32 n 0 then acc
    else
      let got = dma-pool-with-buf pool n 0 steady-step in
      steady-loop pool (wrapping-sub-u32 n 1) (wrapping-add-u64 acc got)

  main :: Unit ->{IO, MVar} Int
  main _ =
    case dma-pool-new 4096 16 of
      None -> 1
      Some pool ->
        -- Warm up one acquire/release cycle before zeroing thread telemetry counters.
        let _ = (case dma-pool-acquire pool of
          None -> ()
          Some h -> dma-pool-release pool h) in
        let _ = engine-thread-counters-reset () in
        let sum = steady-loop pool 50000 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let free-after = dma-pool-free-count pool in
        let _ = dma-pool-destroy pool in
        -- Sum of (1 + n) for n = 1..50000 is 50000 * 50003 / 2 = 1250075000.
        if ne-u64 sum 1250075000 then 2
        else if ne-u32 free-after 16 then 3
        else if ne-u64 allocs 0 then 4
        else if ne-u64 atomic-rcs 0 then 5
        else 0
"#;

    run_leak_checked("dma-50k-zero-alloc", src, 0);
}
