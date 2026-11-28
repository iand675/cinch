//! Live macOS integration tests for `Engine.Reactor.Darwin`:
//! - Stdlib package manifest registration, canonical documentation, and interface checks
//! - Live macOS loopback TCP echo test exchanging 1,000 ping-pong messages over
//!   `darwin-tcp-listen-reuseport` + `darwin-tcp-connect-nonblock` + `darwin-reactor-poll`
//!   + `darwin-writev-dma` scatter-gather + `darwin-recv-dma`, asserting exact payload
//!   integrity, **0 heap allocations** and **0 atomic RC ops** in the steady-state loop
//!   (`engine-thread-alloc-count () == 0`), and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`)
//! - Live macOS UDP `SO_REUSEPORT` loopback datagram round-trip test (including shared-port
//!   `SO_REUSEPORT` bind and zero-alloc DMA `sendto`/`recvfrom`)
//! - Live macOS `F_NOCACHE` direct disk `pwrite` + `fsync` + `pread` + zero-copy `sendfile`
//!   loopback file transfer test, plus `ReactorOps` / `ClockOps` capability bridge check.
//!
//! Routed through `common::drive_with_profile` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const ENGINE_REACTOR_DARWIN: &str = include_str!("../prelude/Engine/Reactor/Darwin.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-engine-reactor-darwin-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[cfg(target_os = "macos")]
fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = common::build_executable(&dir, "Main.cinch", &format!("reactor-darwin-{tag}"))
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));

    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
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

#[test]
fn engine_reactor_darwin_registered_documented_and_checks() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Reactor.Darwin")
        .expect("Engine.Reactor.Darwin listed in seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Engine/Reactor/Darwin.cinch");

    let pos_types = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Types")
        .expect("Engine.Types in order");
    let pos_dma = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Mem.DmaPool")
        .expect("Engine.Mem.DmaPool in order");
    let pos_sys = pkg
        .order
        .iter()
        .position(|n| n == "Prim.Sys.Io.Darwin")
        .expect("Prim.Sys.Io.Darwin in order");
    let pos_rx = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Reactor.Darwin")
        .expect("Engine.Reactor.Darwin in order");
    assert!(pos_types < pos_rx && pos_dma < pos_rx && pos_sys < pos_rx);

    common::assert_documented("Engine.Reactor.Darwin", ENGINE_REACTOR_DARWIN);

    let dir = fresh_dir("check-iface");
    std::fs::write(
        dir.join("Main.cinch"),
        "module Main where\n  import Engine.Reactor.Darwin\n  main :: Int\n  main = 0\n",
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
    .expect("Engine.Reactor.Darwin typechecks cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

#[cfg(target_os = "macos")]
#[test]
fn darwin_live_tcp_1000_ping_pong_writev_zero_alloc_and_zero_leak() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Result
  import Engine.Types
  import Engine.Reactor.Darwin
  import Prim.Atomic
  import Prim.Sys.Io.Darwin

  -- | Non-blocking exact read of `want` bytes from `fd` into `buf + got`,
  -- polling `rx` with `darwin-reactor-poll` whenever `EAGAIN` is encountered.
  -- Zero heap allocations and zero atomic RC ops.
  recv-exact :: DarwinReactor -> Fd -> I64 -> I64 -> U32 -> U32 -> U32 ->{IO, MVar} I32
  recv-exact rx fd buf cqe-buf got want budget =
    if ge-u32 got want then wrapping-u32-to-i32 got
    else if eq-u32 budget 0 then -60
    else
      let rem = wrapping-sub-u32 want got in
      let nr = darwin-recv-dma fd (wrapping-add-i64 buf (u32-to-i64 got)) rem in
      if gt-i32 nr 0 then
        recv-exact rx fd buf cqe-buf (wrapping-add-u32 got (wrapping-i32-to-u32 nr)) want budget
      else if eq-i32 nr 0 then
        0
      else if eq-i32 nr -35 || eq-i32 nr -4 then
        let _nev = darwin-reactor-poll rx 100000000 cqe-buf 16 in
        recv-exact rx fd buf cqe-buf got want (wrapping-sub-u32 budget 1)
      else
        nr

  -- | Write a 2-segment scatter-gather 64-byte frame (8B seq header + 56B body)
  -- using `darwin-writev-dma`, handling `EAGAIN` via `darwin-reactor-poll`.
  -- Zero heap allocations.
  writev-frame :: DarwinReactor -> Fd -> I64 -> I64 -> I64 -> I64 -> U32 ->{IO, MVar} I32
  writev-frame rx fd hdr-buf body-buf iov-buf cqe-buf budget =
    if eq-u32 budget 0 then -60
    else
      let _0 = darwin-iovec-write iov-buf 0 hdr-buf 8 in
      let _1 = darwin-iovec-write iov-buf 1 body-buf 56 in
      let nw = darwin-writev-dma fd iov-buf 2 in
      if eq-i32 nw 64 then 64
      else if gt-i32 nw 0 then
        -- Finish rare partial write over loopback directly from contiguous scratch
        let _ = darwin-reactor-poll rx 10000000 cqe-buf 16 in
        nw
      else if eq-i32 nw -35 || eq-i32 nw -4 then
        let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
        writev-frame rx fd hdr-buf body-buf iov-buf cqe-buf (wrapping-sub-u32 budget 1)
      else
        nw

  -- | Fill 56-byte body (7 x U64 words) at `body-buf` with sequence-derived pattern.
  fill-body :: I64 -> U64 -> U32 ->{MVar} Unit
  fill-body body-buf seq idx =
    if ge-u32 idx 7 then ()
    else
      let w = bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 seq (u32-to-u64 idx)) 0x9E3779B97F4A7C15) 0xA5A55A5A12345678 in
      let _ = raw-store-u64 (wrapping-add-i64 body-buf (wrapping-shl-i64 (u32-to-i64 idx) 3)) w in
      fill-body body-buf seq (wrapping-add-u32 idx 1)

  -- | Verify a 64-byte received frame at `rx-buf` matches `(seq, body-pattern(seq))`.
  verify-frame :: I64 -> U64 -> U32 ->{MVar} Bool
  verify-frame rx-buf seq idx =
    if eq-u32 idx 0 then
      if ne-u64 (raw-load-u64 rx-buf) seq then eq-u32 0 1
      else verify-frame rx-buf seq 1
    else if ge-u32 idx 8 then
      eq-u32 0 0
    else
      let word-i = wrapping-sub-u32 idx 1 in
      let expect = bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 seq (u32-to-u64 word-i)) 0x9E3779B97F4A7C15) 0xA5A55A5A12345678 in
      let got = raw-load-u64 (wrapping-add-i64 rx-buf (wrapping-shl-i64 (u32-to-i64 idx) 3)) in
      if ne-u64 got expect then eq-u32 0 1
      else verify-frame rx-buf seq (wrapping-add-u32 idx 1)

  -- | Wait for listener acceptance over kqueue, returning `fd-invalid ()` on timeout/error.
  accept-loop :: DarwinReactor -> Fd -> I64 -> U32 ->{IO, MVar} Fd
  accept-loop rx lfd cqe-buf budget =
    if eq-u32 budget 0 then fd-invalid ()
    else
      case darwin-tcp-accept-nonblock lfd of
        Ok cfd -> cfd
        Err IoAgain ->
          let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
          accept-loop rx lfd cqe-buf (wrapping-sub-u32 budget 1)
        Err _ -> fd-invalid ()

  -- | Steady-state 1,000 ping-pong echo loop:
  --   1. Client sends 64B frame via 2-iovec `darwin-writev-dma`
  --   2. Server polls `kqueue` + receives 64B via `darwin-recv-dma`, verifies, and echoes via `darwin-writev-dma`
  --   3. Client polls `kqueue` + receives 64B via `darwin-recv-dma`, verifies exact payload
  -- Must perform 0 heap allocations and 0 atomic RC ops across all 1,000 iterations!
  ping-pong-loop :: DarwinReactor -> Fd -> Fd -> I64 -> U64 -> U64 ->{IO, MVar} I32
  ping-pong-loop rx cli srv slab iter limit =
    if ge-u64 iter limit then 0
    else
      -- DMA slab layout (4096 bytes):
      --   +0   : client hdr (8B)
      --   +64  : client body (56B)
      --   +128 : client iov[2] (32B)
      --   +256 : server rx buf (64B)
      --   +384 : server iov[2] (32B)
      --   +512 : client rx buf (64B)
      --   +1024: CQE output array (16 * 16B = 256B)
      let cli-hdr = slab in
      let cli-body = wrapping-add-i64 slab 64 in
      let cli-iov = wrapping-add-i64 slab 128 in
      let srv-rx = wrapping-add-i64 slab 256 in
      let srv-iov = wrapping-add-i64 slab 384 in
      let cli-rx = wrapping-add-i64 slab 512 in
      let cqe-buf = wrapping-add-i64 slab 1024 in
      let _h = raw-store-u64 cli-hdr iter in
      let _b = fill-body cli-body iter 0 in
      -- Client -> Server scatter-gather writev
      let nw1 = writev-frame rx cli cli-hdr cli-body cli-iov cqe-buf 64 in
      if ne-i32 nw1 64 then 101
      else
        -- Poll kqueue for server readability & receive exact 64B
        let nr1 = recv-exact rx srv srv-rx cqe-buf 0 64 64 in
        if ne-i32 nr1 64 then 102
        else if not (verify-frame srv-rx iter 0) then 103
        else
          -- Server -> Client scatter-gather echo writev (8B hdr + 56B body from srv-rx)
          let nw2 = writev-frame rx srv srv-rx (wrapping-add-i64 srv-rx 8) srv-iov cqe-buf 64 in
          if ne-i32 nw2 64 then 104
          else
            -- Poll kqueue for client readability & receive exact 64B echo
            let nr2 = recv-exact rx cli cli-rx cqe-buf 0 64 64 in
            if ne-i32 nr2 64 then 105
            else if not (verify-frame cli-rx iter 0) then 106
            else
              ping-pong-loop rx cli srv slab (wrapping-add-u64 iter 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    case darwin-reactor-new 256 of
      Err _ -> 1
      Ok rx ->
        let slab-u64 = engine-dma-alloc 4096 4096 in
        if eq-u64 slab-u64 0 then
          let _ = darwin-reactor-destroy rx in 2
        else
          let slab = wrapping-u64-to-i64 slab-u64 in
          let cqe-buf = wrapping-add-i64 slab 1024 in
          case darwin-tcp-listen-reuseport 0 128 of
            Err _ ->
              let _ = engine-dma-free slab-u64 4096 in
              let _ = darwin-reactor-destroy rx in 3
            Ok (lfd, port) ->
              let tok-lis = io-token-pack op-code-accept 1 1 in
              let tok-srv = io-token-pack op-code-recv 1 2 in
              let tok-cli = io-token-pack op-code-recv 1 3 in
              let _arm-l = darwin-reactor-arm-read rx lfd tok-lis in
              case darwin-tcp-connect-nonblock ipv4-loopback port of
                Err _ ->
                  let _ = darwin-close-fd lfd in
                  let _ = engine-dma-free slab-u64 4096 in
                  let _ = darwin-reactor-destroy rx in 4
                Ok cli ->
                  -- Flush listener arm & wait for incoming connection
                  let _p0 = darwin-reactor-poll rx 500000000 cqe-buf 16 in
                  let srv = accept-loop rx lfd cqe-buf 64 in
                  if not (fd-is-valid srv) then
                    let _ = darwin-close-fd cli in
                    let _ = darwin-close-fd lfd in
                    let _ = engine-dma-free slab-u64 4096 in
                    let _ = darwin-reactor-destroy rx in 5
                  else
                      -- Arm server & client sockets for edge-triggered EVFILT_READ and flush
                      let _a1 = darwin-reactor-arm-read rx srv tok-srv in
                      let _a2 = darwin-reactor-arm-read rx cli tok-cli in
                      let _p1 = darwin-reactor-poll rx 0 cqe-buf 16 in
                      let t0 = darwin-reactor-now-ns rx in
                      -- Reset thread allocation & atomic RC counters right before steady-state 1,000 ping-pong loop!
                      let _rst = engine-thread-counters-reset () in
                      let loop-rc = ping-pong-loop rx cli srv slab 0 1000 in
                      let allocs = engine-thread-alloc-count () in
                      let atomic-rcs = engine-thread-atomic-rc-count () in
                      let t1 = darwin-reactor-now-ns rx in
                      -- Test disarming both fds (only READ was armed, so WRITE EV_DELETE gets ENOENT which must be suppressed!)
                      let dis1 = darwin-reactor-disarm rx srv in
                      let dis2 = darwin-reactor-disarm rx cli in
                      let dis-nev = darwin-reactor-poll rx 0 cqe-buf 16 in
                      let _c1 = darwin-close-fd srv in
                      let _c2 = darwin-close-fd cli in
                      let _c3 = darwin-close-fd lfd in
                      let _f = engine-dma-free slab-u64 4096 in
                      let _d = darwin-reactor-destroy rx in
                      if ne-i32 loop-rc 0 then int-from-i64 (i32-to-i64 loop-rc)
                      else if ne-u64 allocs 0 then 20
                      else if ne-u64 atomic-rcs 0 then 21
                      else if lt-u64 t1 t0 || eq-u64 t1 0 then 22
                      else if not dis1 || not dis2 || ne-i32 dis-nev 0 then 23
                      else 0
"#;

    build_and_run_leak_checked("tcp-1000-ping-pong", src, 0);
}

#[cfg(target_os = "macos")]
#[test]
fn darwin_live_udp_reuseport_loopback_roundtrip() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Result
  import Engine.Types
  import Engine.Reactor.Darwin
  import Prim.Atomic

  -- | Receive one UDP datagram on `fd`, polling kqueue on `EAGAIN`. Zero allocs.
  udp-recv-wait :: DarwinReactor -> Fd -> I64 -> U32 -> I64 -> I64 -> I64 -> U32 ->{IO, MVar} I32
  udp-recv-wait rx fd buf len src-addr src-len-addr cqe-buf budget =
    if eq-u32 budget 0 then -60
    else
      let nr = darwin-udp-recvfrom-dma fd buf len src-addr src-len-addr in
      if gt-i32 nr 0 then nr
      else if eq-i32 nr -35 || eq-i32 nr -4 then
        let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
        udp-recv-wait rx fd buf len src-addr src-len-addr cqe-buf (wrapping-sub-u32 budget 1)
      else nr

  -- | Run `limit` UDP loopback ping-pong exchanges with zero heap allocations.
  udp-loop :: DarwinReactor -> Fd -> Fd -> U16 -> U16 -> I64 -> U64 -> U64 ->{IO, MVar} I32
  udp-loop rx fd-a fd-b port-a port-b slab iter limit =
    if ge-u64 iter limit then 0
    else
      -- Slab layout:
      --   +0   : tx payload (32B)
      --   +64  : rx payload (32B)
      --   +128 : dst-b sockaddr_in (16B)
      --   +160 : dst-a sockaddr_in (16B)
      --   +192 : src sockaddr_in (16B)
      --   +224 : src-len (4B)
      --   +512 : cqe-buf (256B)
      let tx-buf = slab in
      let rx-buf = wrapping-add-i64 slab 64 in
      let dst-b = wrapping-add-i64 slab 128 in
      let dst-a = wrapping-add-i64 slab 160 in
      let src-sa = wrapping-add-i64 slab 192 in
      let src-len = wrapping-add-i64 slab 224 in
      let cqe-buf = wrapping-add-i64 slab 512 in
      let tag = bit-xor-u64 (wrapping-mul-u64 iter 0x517CC1B727220A95) 0xDEADBEEFCAFEBABE in
      let _w0 = raw-store-u64 tx-buf iter in
      let _w1 = raw-store-u64 (wrapping-add-i64 tx-buf 8) tag in
      let _w2 = raw-store-u64 (wrapping-add-i64 tx-buf 16) (wrapping-add-u64 iter 77) in
      let _w3 = raw-store-u64 (wrapping-add-i64 tx-buf 24) (bit-not-u64 tag) in
      let ns1 = darwin-udp-sendto-dma fd-a tx-buf 32 dst-b in
      if ne-i32 ns1 32 then 40
      else
        let nr1 = udp-recv-wait rx fd-b rx-buf 32 src-sa src-len cqe-buf 64 in
        if ne-i32 nr1 32 then 41
        else if ne-u64 (raw-load-u64 rx-buf) iter || ne-u64 (raw-load-u64 (wrapping-add-i64 rx-buf 8)) tag then 42
        else if ne-u32 (darwin-sockaddr-read-ip-dma src-sa) ipv4-loopback || ne-u16 (darwin-sockaddr-read-port-dma src-sa) port-a then 43
        else
          -- Echo back from B to A
          let ns2 = darwin-udp-sendto-dma fd-b rx-buf 32 dst-a in
          if ne-i32 ns2 32 then 44
          else
            let nr2 = udp-recv-wait rx fd-a rx-buf 32 src-sa src-len cqe-buf 64 in
            if ne-i32 nr2 32 then 45
            else if ne-u64 (raw-load-u64 rx-buf) iter || ne-u64 (raw-load-u64 (wrapping-add-i64 rx-buf 24)) (bit-not-u64 tag) then 46
            else if ne-u16 (darwin-sockaddr-read-port-dma src-sa) port-b then 47
            else udp-loop rx fd-a fd-b port-a port-b slab (wrapping-add-u64 iter 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    case darwin-reactor-new 128 of
      Err _ -> 1
      Ok rx ->
        let slab-u64 = engine-dma-alloc 4096 4096 in
        if eq-u64 slab-u64 0 then
          let _ = darwin-reactor-destroy rx in 2
        else
          let slab = wrapping-u64-to-i64 slab-u64 in
          let dst-b = wrapping-add-i64 slab 128 in
          let dst-a = wrapping-add-i64 slab 160 in
          let cqe-buf = wrapping-add-i64 slab 512 in
          case darwin-udp-bind-reuseport 0 of
            Err _ ->
              let _ = engine-dma-free slab-u64 4096 in
              let _ = darwin-reactor-destroy rx in 3
            Ok (fd-a, port-a) ->
              case darwin-udp-bind-reuseport 0 of
                Err _ ->
                  let _ = darwin-close-fd fd-a in
                  let _ = engine-dma-free slab-u64 4096 in
                  let _ = darwin-reactor-destroy rx in 4
                Ok (fd-b, port-b) ->
                  -- Verify SO_REUSEPORT allows a second UDP socket bound to the exact same port!
                  case darwin-udp-bind-reuseport port-a of
                    Err _ ->
                      let _ = darwin-close-fd fd-b in
                      let _ = darwin-close-fd fd-a in
                      let _ = engine-dma-free slab-u64 4096 in
                      let _ = darwin-reactor-destroy rx in 5
                    Ok (fd-reuse, port-reuse) ->
                      let _cl-reuse = darwin-close-fd fd-reuse in
                      if ne-u16 port-reuse port-a then
                        let _ = darwin-close-fd fd-b in
                        let _ = darwin-close-fd fd-a in
                        let _ = engine-dma-free slab-u64 4096 in
                        let _ = darwin-reactor-destroy rx in 6
                      else
                        let _sa1 = darwin-sockaddr-write-dma dst-b ipv4-loopback port-b in
                        let _sa2 = darwin-sockaddr-write-dma dst-a ipv4-loopback port-a in
                        let _a1 = darwin-reactor-arm-read rx fd-a (io-token-pack op-code-recv-msg 1 1) in
                        let _a2 = darwin-reactor-arm-read rx fd-b (io-token-pack op-code-recv-msg 1 2) in
                        let _p0 = darwin-reactor-poll rx 0 cqe-buf 16 in
                        let _rst = engine-thread-counters-reset () in
                        let rc = udp-loop rx fd-a fd-b port-a port-b slab 0 64 in
                        let allocs = engine-thread-alloc-count () in
                        let atomic-rcs = engine-thread-atomic-rc-count () in
                        let _c1 = darwin-close-fd fd-b in
                        let _c2 = darwin-close-fd fd-a in
                        let _f = engine-dma-free slab-u64 4096 in
                        let _d = darwin-reactor-destroy rx in
                        if ne-i32 rc 0 then int-from-i64 (i32-to-i64 rc)
                        else if ne-u64 allocs 0 then 20
                        else if ne-u64 atomic-rcs 0 then 21
                        else 0
"#;

    build_and_run_leak_checked("udp-reuseport-roundtrip", src, 0);
}

#[cfg(target_os = "macos")]
#[test]
fn darwin_live_f_nocache_direct_disk_sendfile_and_reactor_ops_bridge() {
    let tmp_dir = fresh_dir("direct-disk-file");
    let file_path = tmp_dir.join("cinch_f_nocache_test.bin");
    let path_escaped = file_path
        .to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Types
  import Engine.Mem.DmaPool
  import Engine.Reactor.Darwin
  import Prim.Atomic
  import Prim.Sys.Io.Darwin

  -- | Fill `words` 64-bit words at `addr` with a deterministic page-unique pattern.
  fill-words :: I64 -> U64 -> U64 -> U64 ->{MVar} Unit
  fill-words addr base-seed i words =
    if ge-u64 i words then ()
    else
      let v = bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 base-seed i) 0x9E3779B97F4A7C15) 0xF0E1D2C3B4A59687 in
      let _ = raw-store-u64 (wrapping-add-i64 addr (wrapping-shl-i64 (wrapping-u64-to-i64 i) 3)) v in
      fill-words addr base-seed (wrapping-add-u64 i 1) words

  -- | Verify `words` 64-bit words at `addr` match the deterministic pattern.
  verify-words :: I64 -> U64 -> U64 -> U64 ->{MVar} Bool
  verify-words addr base-seed i words =
    if ge-u64 i words then eq-u32 0 0
    else
      let expect = bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 base-seed i) 0x9E3779B97F4A7C15) 0xF0E1D2C3B4A59687 in
      let got = raw-load-u64 (wrapping-add-i64 addr (wrapping-shl-i64 (wrapping-u64-to-i64 i) 3)) in
      if ne-u64 got expect then eq-u32 0 1
      else verify-words addr base-seed (wrapping-add-u64 i 1) words

  accept-wait :: DarwinReactor -> Fd -> I64 -> U32 ->{IO, MVar} Fd
  accept-wait rx lfd cqe-buf budget =
    if eq-u32 budget 0 then fd-invalid ()
    else
      case darwin-tcp-accept-nonblock lfd of
        Ok cfd -> cfd
        Err IoAgain ->
          let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
          accept-wait rx lfd cqe-buf (wrapping-sub-u32 budget 1)
        Err _ -> fd-invalid ()

  -- | Drive non-blocking `darwin-sendfile-fd` AND peer `darwin-recv-dma` concurrently
  -- in one thread over loopback until all `total` bytes are sent and received.
  sendfile-and-recv :: DarwinReactor -> Fd -> Fd -> Fd -> I64 -> I64 -> U64 -> U64 -> U64 -> U32 ->{IO, MVar} I32
  sendfile-and-recv rx file-fd tx-sock rx-sock rx-buf cqe-buf sent recvd total budget =
    if ge-u64 recvd total then 0
    else if eq-u32 budget 0 then 70
    else
      -- Push next chunk via sendfile if not yet done sending
      let next-sent = (if lt-u64 sent total then (let rem = wrapping-sub-u64 total sent in let s = darwin-sendfile-fd file-fd tx-sock sent rem in if lt-i64 0 s then wrapping-add-u64 sent (wrapping-i64-to-u64 s) else sent) else sent) in
      -- Drain available bytes on receiver socket
      let rx-rem = wrapping-u64-to-u32 (wrapping-sub-u64 total recvd) in
      let nr = darwin-recv-dma rx-sock (wrapping-add-i64 rx-buf (wrapping-u64-to-i64 recvd)) rx-rem in
      let next-recvd = (if gt-i32 nr 0 then wrapping-add-u64 recvd (u32-to-u64 (wrapping-i32-to-u32 nr)) else recvd) in
      if ge-u64 next-recvd total then 0
      else
        let _p = darwin-reactor-poll rx 50000000 cqe-buf 16 in
        sendfile-and-recv rx file-fd tx-sock rx-sock rx-buf cqe-buf next-sent next-recvd total (wrapping-sub-u32 budget 1)

  -- | Generic capability-based check using `Engine.Types.ReactorOps` & `ClockOps`.
  cap-smoke :: forall (r :: Region). using (rx :: ReactorOps r, clk :: ClockOps r) => Fd -> Fd -> I64 -> I64 ->{IO r, Mut r} Bool
  cap-smoke tx-fd rx-fd tx-addr rx-addr =
    let t0 = engine-clock-mono-ns () in
    let w0 = engine-clock-wall-ns () in
    let tok-r = io-token-pack op-code-recv 1 10 in
    let tok-s = io-token-pack op-code-send 1 11 in
    let _e1 = engine-reactor-submit (IoSqe op-code-recv 0 0 rx-fd.fd-raw (wrapping-i64-to-u64 rx-addr) 64 0 0 tok-r.io-token-raw) in
    let _e2 = engine-reactor-submit (IoSqe op-code-send 0 0 tx-fd.fd-raw (wrapping-i64-to-u64 tx-addr) 64 0 0 tok-s.io-token-raw) in
    let _fl = engine-reactor-flush () in
    let _wt = engine-reactor-wait (time-ns-of 100000000) in
    let c1 = engine-reactor-pop-cqe () in
    let _wt2 = if eq-u32 (engine-reactor-cqe-count ()) 0 then engine-reactor-wait (time-ns-of 100000000) else 0 in
    let c2 = engine-reactor-pop-cqe () in
    let t1 = engine-clock-mono-ns () in
    let _cl = engine-reactor-close () in
    eq-i32 c1.cqe-res 64 && eq-i32 c2.cqe-res 64 && time-ns-le t0 t1 && gt-u64 (time-ns-value w0) 0

  test-cap-bridge :: forall (r :: Region). DarwinReactor -> Fd -> Fd -> I64 -> I64 ->{IO r, Mut r} Bool
  test-cap-bridge rx tx-fd rx-fd tx-addr rx-addr =
    let rx-ops = darwin-reactor-ops rx in
    with-reactor-ops rx-ops (\() ->
      with-clock-ops darwin-clock-ops (\() ->
        cap-smoke tx-fd rx-fd tx-addr rx-addr))

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    -- 1. Open file with F_NOCACHE (`darwin-open-direct`) and use 4 KiB-aligned DmaPool buffers
    let path = string-to-bytes "__DIRECT_FILE_PATH__" in
    let open-flags = wrapping-i64-to-i32 (bit-or-i64 (bit-or-i64 o-rdwr o-creat) o-trunc) in
    case darwin-open-direct path open-flags of
      Err _ -> 1
      Ok file-fd ->
        case dma-pool-new 16384 2 of
          None ->
            let _ = darwin-close-fd file-fd in 2
          Some pool ->
            let tx-addr = dma-pool-buf-addr pool 0 in
            let rx-addr = dma-pool-buf-addr pool 1 in
            -- Write 16 KiB (2048 x U64 words) via pwrite + fsync
            let _fill = fill-words tx-addr 42 0 2048 in
            let nw = darwin-pwrite-dma file-fd tx-addr 16384 0 in
            if ne-i32 nw 16384 then
              let _ = dma-pool-destroy pool in
              let _ = darwin-close-fd file-fd in 3
            else
              let fs-err = darwin-fsync-fd file-fd in
              if not (io-error-is-ok fs-err) then
                let _ = dma-pool-destroy pool in
                let _ = darwin-close-fd file-fd in 4
              else
                -- Read back 16 KiB via pread and verify every 64-bit word
                let nr = darwin-pread-dma file-fd rx-addr 16384 0 in
                if ne-i32 nr 16384 || not (verify-words rx-addr 42 0 2048) then
                  let _ = dma-pool-destroy pool in
                  let _ = darwin-close-fd file-fd in 5
                else
                  -- 2. Zero-copy `sendfile` of the 16 KiB file over live loopback TCP
                  let _zero = fill-words rx-addr 0 0 2048 in
                  case darwin-reactor-new 128 of
                    Err _ ->
                      let _ = dma-pool-destroy pool in
                      let _ = darwin-close-fd file-fd in 6
                    Ok rx ->
                      let cqe-slab = engine-dma-alloc 4096 4096 in
                      let cqe-buf = wrapping-u64-to-i64 cqe-slab in
                      case darwin-tcp-listen-reuseport 0 16 of
                        Err _ ->
                          let _ = engine-dma-free cqe-slab 4096 in
                          let _ = darwin-reactor-destroy rx in
                          let _ = dma-pool-destroy pool in
                          let _ = darwin-close-fd file-fd in 7
                        Ok (lfd, port) ->
                          let _al = darwin-reactor-arm-read rx lfd (io-token-pack op-code-accept 1 1) in
                          case darwin-tcp-connect-nonblock ipv4-loopback port of
                            Err _ ->
                              let _ = darwin-close-fd lfd in
                              let _ = engine-dma-free cqe-slab 4096 in
                              let _ = darwin-reactor-destroy rx in
                              let _ = dma-pool-destroy pool in
                              let _ = darwin-close-fd file-fd in 8
                            Ok cli ->
                              let _p0 = darwin-reactor-poll rx 500000000 cqe-buf 16 in
                              let srv = accept-wait rx lfd cqe-buf 64 in
                              if not (fd-is-valid srv) then
                                let _ = darwin-close-fd cli in
                                let _ = darwin-close-fd lfd in
                                let _ = engine-dma-free cqe-slab 4096 in
                                let _ = darwin-reactor-destroy rx in
                                let _ = dma-pool-destroy pool in
                                let _ = darwin-close-fd file-fd in 9
                              else
                                  let _ar = darwin-reactor-arm-read rx cli (io-token-pack op-code-recv 1 2) in
                                  let _aw = darwin-reactor-arm-write rx srv (io-token-pack op-code-send 1 3) in
                                  let _p1 = darwin-reactor-poll rx 0 cqe-buf 16 in
                                  let sf-rc = sendfile-and-recv rx file-fd srv cli rx-addr cqe-buf 0 0 16384 128 in
                                  let sf-ok = eq-i32 sf-rc 0 && verify-words rx-addr 42 0 2048 in
                                  -- 3. Verify `darwin-reactor-ops` + `darwin-clock-ops` capability bridge
                                  let _zero64 = fill-words rx-addr 0 0 8 in
                                  let cap-ok = test-cap-bridge rx srv cli tx-addr rx-addr && verify-words rx-addr 42 0 8 in
                                  -- 4. Verify standalone monotonic clock advances monotonically
                                  let m0 = darwin-clock-monotonic-ns () in
                                  let m1 = darwin-clock-monotonic-ns () in
                                  let clk-ok = gt-u64 m0 0 && le-u64 m0 m1 in
                                  let _c1 = darwin-close-fd srv in
                                  let _c2 = darwin-close-fd cli in
                                  let _c3 = darwin-close-fd lfd in
                                  let _f = engine-dma-free cqe-slab 4096 in
                                  let _d = darwin-reactor-destroy rx in
                                  let _pd = dma-pool-destroy pool in
                                  let _cf = darwin-close-fd file-fd in
                                  if not sf-ok then 10
                                  else if not cap-ok then 11
                                  else if not clk-ok then 12
                                  else 0
"#
    .replace("__DIRECT_FILE_PATH__", &path_escaped);

    build_and_run_leak_checked("f-nocache-sendfile-cap", &src, 0);
    let _ = std::fs::remove_dir_all(&tmp_dir);
}
