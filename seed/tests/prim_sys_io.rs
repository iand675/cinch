//! Verification and native integration tests for `Prim.Sys.Io` (`Linux`, `Darwin`, `Portable`, facade).
//!
//! 1. `mem_contract` v2 audit (`cinch verify-prims --audit-unsafe`) on `Prim.Sys.Io.Linux.cinch`
//!    and `Prim.Sys.Io.Darwin.cinch` asserting **0 violations** and verifying TCB allowlist compliance.
//! 2. `tooling/verification/verify_kernel_abi.py` verification of `DarwinKevent`, `Iovec`,
//!    `SockAddrIn`, and `Timespec` against real macOS headers (`<sys/event.h>`, `<sys/uio.h>`,
//!    `<netinet/in.h>`, `<time.h>`), plus exact Linux `io_uring` / socket / timespec struct layout assertions.
//! 3. Cross-target checking, LLVM IR emission, and `clang` assembly verification for
//!    `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `wasm32-unknown-unknown`.
//! 4. Live native loopback test on macOS: `kqueue`, non-blocking `SO_REUSEPORT` TCP listener + client,
//!    `kevent` readiness + `writev`/`read` round-trip, UDP loopback `sendto`/`recvfrom`, `mmap`/`munmap`,
//!    and `clock_gettime` monotonic advance.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{target_profile, DriverOptions, Mode};
use cinch::prim_verify::mem_contract::audit_file_contracts;
use std::path::{Path, PathBuf};
use std::process::Command;

const PRIM_SYS_IO_FILES: &[(&str, &str)] = &[
    (
        "Prim.Sys.Io.Darwin.cinch",
        include_str!("../prelude/Prim/Sys/Io/Darwin.cinch"),
    ),
    (
        "Prim.Sys.Io.Linux.cinch",
        include_str!("../prelude/Prim/Sys/Io/Linux.cinch"),
    ),
    (
        "Prim.Sys.Io.Portable.cinch",
        include_str!("../prelude/Prim/Sys/Io/Portable.cinch"),
    ),
    (
        "Prim.Sys.Io.cinch",
        include_str!("../prelude/Prim/Sys/Io.cinch"),
    ),
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}

fn profile_for(triple: &str) -> TargetProfile {
    target_profile(&DriverOptions {
        target: Some(triple.to_string()),
        ..DriverOptions::default()
    })
    .unwrap_or_else(|e| panic!("target profile {triple}: {e}"))
}

#[test]
fn mem_contract_v2_audit_and_tcb_allowlist_pass_with_zero_violations() {
    let root = repo_root();
    let linux_path = root.join("seed/prelude/Prim/Sys/Io/Linux.cinch");
    let darwin_path = root.join("seed/prelude/Prim/Sys/Io/Darwin.cinch");

    let linux_rep = audit_file_contracts(&linux_path).expect("audit Prim.Sys.Io.Linux");
    assert!(
        linux_rep.violations.is_empty(),
        "Prim.Sys.Io.Linux must have 0 mem_contract violations, got: {:?}",
        linux_rep.violations
    );
    assert_eq!(linux_rep.unsafe_bodies_audited, 72);
    assert_eq!(linux_rep.inline_asm_checked, 72);
    assert!(
        linux_rep.linux_syscalls_checked >= 70,
        "expected >= 70 pinned Linux syscall bodies audited, got {}",
        linux_rep.linux_syscalls_checked
    );

    let darwin_rep = audit_file_contracts(&darwin_path).expect("audit Prim.Sys.Io.Darwin");
    assert!(
        darwin_rep.violations.is_empty(),
        "Prim.Sys.Io.Darwin must have 0 mem_contract violations, got: {:?}",
        darwin_rep.violations
    );
    assert_eq!(darwin_rep.unsafe_bodies_audited, 1);
    assert_eq!(darwin_rep.memory_ops_checked, 1);

    // Run the full `cinch verify-prims --audit-unsafe` CLI pipeline directly.
    let cli_args = vec![
        "--audit-unsafe".to_string(),
        linux_path.to_string_lossy().into_owned(),
        darwin_path.to_string_lossy().into_owned(),
    ];
    let rc = cinch::prim_verify::cli(&cli_args);
    assert_eq!(rc, 0, "cinch verify-prims --audit-unsafe must exit 0");

    // Verify every unsafe prim in Prim.Sys.Io.{Linux,Darwin} is admitted by tcb_allowlist.json.
    let tmp = std::env::temp_dir().join(format!("cinch_prim_sys_io_tcb_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let manifest = tmp.join("PrimSysIo.manifest.json");
    let linux_prims = [
        "linux-io-uring-setup",
        "linux-io-uring-enter",
        "linux-io-uring-register",
        "linux-socket",
        "linux-bind",
        "linux-listen",
        "linux-accept4",
        "linux-setsockopt",
        "linux-getsockopt",
        "linux-close",
        "linux-fcntl",
        "linux-sendto",
        "linux-recvfrom",
        "linux-sendmsg",
        "linux-recvmsg",
        "linux-sendmmsg",
        "linux-recvmmsg",
        "linux-read",
        "linux-write",
        "linux-syscall4",
        "linux-mmap",
        "linux-munmap",
        "linux-madvise",
        "linux-openat",
        "linux-fallocate",
        "linux-fdatasync",
        "linux-fsync",
        "linux-lseek",
        "linux-splice",
        "linux-sendfile",
        "linux-eventfd2",
        "linux-clock-gettime",
        "linux-sched-setaffinity",
        "linux-epoll-create1",
        "linux-epoll-ctl",
        "linux-epoll-pwait",
    ];
    let mut entries: Vec<String> = linux_prims
        .iter()
        .map(|name| {
            format!(
                r#"{{"module":"Prim.Sys.Io.Linux","name":"{name}","tier":"assumed","receipts":[{{"tier":"assumed"}}]}}"#
            )
        })
        .collect();
    entries.push(
        r#"{"module":"Prim.Sys.Io.Darwin","name":"darwin-load-i32","tier":"assumed","receipts":[{"tier":"assumed"}]}"#
            .to_string(),
    );
    let manifest_json = format!(
        r#"{{"entry":"PrimSysIo","prims":[{}]}}"#,
        entries.join(",")
    );
    std::fs::write(&manifest, manifest_json).unwrap();

    let out = Command::new("python3")
        .arg(root.join("tooling/verification/check_tcb_allowlist.py"))
        .arg("--strict-jpl")
        .arg(&manifest)
        .output()
        .expect("run check_tcb_allowlist.py");
    assert!(
        out.status.success(),
        "tcb allowlist check failed: stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn verify_kernel_abi_structs_against_real_headers_and_linux_layouts() {
    let root = repo_root();

    // 1. Check `Prim.Sys.Io.Darwin` via the selected frontend and verify its `.cinchif`
    //    `abi.layouts` byte-for-byte against host macOS headers with `verify_kernel_abi.py`.
    let darwin_profile = profile_for("aarch64-apple-darwin");
    let mut files = PRIM_SYS_IO_FILES.to_vec();
    files.push((
        "Main.cinch",
        "module Main where\n  import Prim.Sys.Io.Darwin\n  main :: Int\n  main = 0\n",
    ));
    let dir = common::project("prim-sys-io-darwin-abi", &files);
    let ifaces = common::check_interfaces_profile(&dir, "Main.cinch", &darwin_profile)
        .expect("check Prim.Sys.Io.Darwin interfaces");
    let darwin_iface = ifaces
        .iter()
        .find(|i| i.module == "Prim.Sys.Io.Darwin")
        .expect("Prim.Sys.Io.Darwin interface");

    // Verify exact layout numbers in the checked interface.
    let by_name = |n: &str| {
        darwin_iface
            .layouts
            .iter()
            .find(|l| l.name == n)
            .unwrap_or_else(|| panic!("missing Darwin layout {n}"))
    };
    assert_eq!((by_name("DarwinKevent").size, by_name("DarwinKevent").align), (32, 4));
    assert_eq!((by_name("Iovec").size, by_name("Iovec").align), (16, 8));
    assert_eq!((by_name("SockAddrIn").size, by_name("SockAddrIn").align), (16, 4));
    assert_eq!((by_name("Timespec").size, by_name("Timespec").align), (16, 8));

    #[cfg(target_os = "macos")]
    {
        let cinchif_path = dir.join("Prim.Sys.Io.Darwin.cinchif");
        std::fs::write(&cinchif_path, darwin_iface.to_json()).unwrap();
        let spec_path = dir.join("darwin_kevent_spec.json");
        std::fs::write(
            &spec_path,
            r#"{
              "kevent": {
                "headers": ["<sys/types.h>", "<sys/event.h>", "<sys/time.h>"],
                "c_type": "struct kevent",
                "fields": {
                  "ident": "ident",
                  "filter": "filter",
                  "flags": "flags",
                  "fflags": "fflags",
                  "ev-data": "data",
                  "udata": "udata"
                }
              },
              "SockAddrIn": {
                "headers": ["<netinet/in.h>"],
                "c_type": "struct sockaddr_in",
                "fields": {
                  "sin-len": "sin_len",
                  "sin-family": "sin_family",
                  "sin-port": "sin_port",
                  "sin-addr": "sin_addr"
                }
              }
            }"#,
        )
        .unwrap();

        let out = Command::new("python3")
            .arg(root.join("tooling/verification/verify_kernel_abi.py"))
            .arg("--require-all-repr-c")
            .arg("--spec")
            .arg(&spec_path)
            .arg(&cinchif_path)
            .output()
            .expect("run verify_kernel_abi.py on Prim.Sys.Io.Darwin.cinchif");
        assert!(
            out.status.success(),
            "Darwin kernel ABI verification failed:\nstdout={}\nstderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(
            stdout.contains("checked=4 skipped=0 violations=0"),
            "expected all 4 Darwin structs verified with 0 violations, got: {stdout}"
        );
    }
    common::cleanup(&dir);

    // 2. Check `Prim.Sys.Io.Linux` across both `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu`
    //    and assert exact Linux UAPI struct sizes, alignments, and field offsets.
    for triple in ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"] {
        let lp = profile_for(triple);
        let mut lfiles = PRIM_SYS_IO_FILES.to_vec();
        lfiles.push((
            "Main.cinch",
            "module Main where\n  import Prim.Sys.Io.Linux\n  main :: Int\n  main = 0\n",
        ));
        let ldir = common::project(&format!("prim-sys-io-linux-abi-{triple}"), &lfiles);
        let lifaces = common::check_interfaces_profile(&ldir, "Main.cinch", &lp)
            .unwrap_or_else(|e| panic!("{triple}: check Prim.Sys.Io.Linux failed: {e}"));
        let linux_iface = lifaces
            .iter()
            .find(|i| i.module == "Prim.Sys.Io.Linux")
            .expect("Prim.Sys.Io.Linux interface");
        let get = |n: &str| {
            linux_iface
                .layouts
                .iter()
                .find(|l| l.name == n)
                .unwrap_or_else(|| panic!("{triple}: missing Linux layout {n}"))
        };

        let sqe = get("IoUringSqe");
        assert_eq!((sqe.size, sqe.align), (64, 8), "{triple}: IoUringSqe");
        let sqe_off = |f: &str| {
            let af = sqe.fields.iter().find(|x| x.name == f).unwrap();
            (af.offset, af.size)
        };
        assert_eq!(sqe_off("opcode"), (0, 1));
        assert_eq!(sqe_off("flags"), (1, 1));
        assert_eq!(sqe_off("ioprio"), (2, 2));
        assert_eq!(sqe_off("fd"), (4, 4));
        assert_eq!(sqe_off("off"), (8, 8));
        assert_eq!(sqe_off("addr"), (16, 8));
        assert_eq!(sqe_off("len"), (24, 4));
        assert_eq!(sqe_off("op-flags"), (28, 4));
        assert_eq!(sqe_off("user-data"), (32, 8));
        assert_eq!(sqe_off("buf-index"), (40, 2));
        assert_eq!(sqe_off("personality"), (42, 2));
        assert_eq!(sqe_off("splice-fd-in"), (44, 4));
        assert_eq!(sqe_off("addr3"), (48, 8));
        assert_eq!(sqe_off("pad2"), (56, 8));

        let cqe = get("IoUringCqe");
        assert_eq!((cqe.size, cqe.align), (16, 8), "{triple}: IoUringCqe");
        assert_eq!((cqe.fields[0].offset, cqe.fields[0].size), (0, 8));
        assert_eq!((cqe.fields[1].offset, cqe.fields[1].size), (8, 4));
        assert_eq!((cqe.fields[2].offset, cqe.fields[2].size), (12, 4));

        let sq_off = get("IoSqringOffsets");
        assert_eq!((sq_off.size, sq_off.align), (40, 8));
        let cq_off = get("IoCqringOffsets");
        assert_eq!((cq_off.size, cq_off.align), (40, 8));

        let params = get("IoUringParams");
        assert_eq!((params.size, params.align), (120, 8), "{triple}: IoUringParams");
        let p_sq = params.fields.iter().find(|f| f.name == "sq-off").unwrap();
        let p_cq = params.fields.iter().find(|f| f.name == "cq-off").unwrap();
        assert_eq!((p_sq.offset, p_sq.size), (40, 40));
        assert_eq!((p_cq.offset, p_cq.size), (80, 40));

        assert_eq!((get("Iovec").size, get("Iovec").align), (16, 8));
        assert_eq!((get("SockAddrIn").size, get("SockAddrIn").align), (16, 4));
        assert_eq!((get("Timespec").size, get("Timespec").align), (16, 8));
        common::cleanup(&ldir);
    }
}

#[test]
fn cross_target_linux_and_portable_emit_ir_and_assemble() {
    let linux_main = r#"
module Main where
  import Prim.Sys.Io

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let _a = sys-io-uring-setup 32 0 in
    let _b = sys-io-uring-enter 3 1 1 ioring-enter-getevents 0 0 in
    let _c = sys-io-uring-register 3 ioring-register-buffers 0 0 in
    let _d = sys-socket af-inet (bit-or-i64 sock-stream sock-nonblock) 0 in
    let _e = sys-bind 3 0 16 in
    let _f = sys-listen 3 128 in
    let _g = sys-accept4 3 0 0 sock-nonblock in
    let _h = sys-setsockopt 3 sol-socket so-reuseport 0 4 in
    let _i = sys-getsockopt 3 sol-socket so-reuseport 0 0 in
    let _j = sys-fcntl 3 f-getfl 0 in
    let _k = sys-sendto 3 0 0 0 0 0 in
    let _l = sys-recvfrom 3 0 0 0 0 0 in
    let _m = sys-sendmsg 3 0 0 in
    let _n = sys-recvmsg 3 0 0 in
    let _o = sys-read 3 0 0 in
    let _p = sys-write 3 0 0 in
    let _q = sys-pread 3 0 0 0 in
    let _r = sys-pwrite 3 0 0 0 in
    let _s = sys-mmap 0 4096 (bit-or-i64 prot-read prot-write) (bit-or-i64 map-private map-anon) -1 0 in
    let _t = sys-munmap 0 4096 in
    let _u = sys-madvise 0 4096 madv-willneed in
    let _v = sys-openat at-fdcwd 0 (bit-or-i64 o-rdwr o-direct) 420 in
    let _w = sys-fallocate 3 0 0 4096 in
    let _x = sys-fdatasync 3 in
    let _y = sys-splice 3 0 4 0 4096 0 in
    let _z = sys-sendfile 4 3 0 4096 in
    let _e1 = sys-eventfd2 0 efd-nonblock in
    let _e2 = sys-clock-gettime clock-monotonic 0 in
    let _e3 = sys-epoll-create1 epoll-cloexec in
    let _e4 = sys-epoll-ctl 3 epoll-ctl-add 4 0 in
    let _e5 = sys-epoll-pwait 3 0 16 0 0 0 in
    let _e6 = sys-close 3 in
    0
"#;

    for (triple, expected_asm, expected_nr) in [
        ("x86_64-unknown-linux-gnu", "syscall", "mov rax, 425"),
        ("aarch64-unknown-linux-gnu", "svc #0", "mov x8, #425"),
    ] {
        let lp = profile_for(triple);
        let mut files = PRIM_SYS_IO_FILES.to_vec();
        files.push(("Main.cinch", linux_main));
        let dir = common::project(&format!("prim-sys-io-emit-{triple}"), &files);
        let ll = dir.join("program.ll");
        common::drive_with_profile(Mode::EmitIr, StageMode::Refresh, &dir, "Main.cinch", Some(ll.clone()), &lp)
            .unwrap_or_else(|e| panic!("{triple}: emit-ir failed: {e}"));
        let ir = std::fs::read_to_string(&ll).expect("read emitted IR");
        assert!(ir.contains(expected_asm), "{triple}: IR missing `{expected_asm}`");
        assert!(ir.contains(expected_nr), "{triple}: IR missing `{expected_nr}`");

        // Assemble with clang to prove every inline-asm constraint & instruction is valid on target!
        let obj = dir.join("program.o");
        let asm = Command::new("clang")
            .arg("-c")
            .arg(format!("--target={triple}"))
            .arg(&ll)
            .arg("-o")
            .arg(&obj)
            .output()
            .expect("run clang assembler");
        assert!(
            asm.status.success(),
            "{triple}: clang refused emitted Linux syscall IR:\n{}",
            String::from_utf8_lossy(&asm.stderr)
        );
        common::cleanup(&dir);
    }

    // Verify wasm32 selects Prim.Sys.Io.Portable cleanly.
    let wasm_main = r#"
module Main where
  import Prim.Sys.Io

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    if sys-socket af-inet sock-stream 0 == -38 then 0 else 1
"#;
    let wp = profile_for("wasm32-unknown-unknown");
    let mut wfiles = PRIM_SYS_IO_FILES.to_vec();
    wfiles.push(("Main.cinch", wasm_main));
    let wdir = common::project("prim-sys-io-wasm", &wfiles);
    common::drive_with_profile(Mode::Check, StageMode::Refresh, &wdir, "Main.cinch", None, &wp)
        .expect("wasm32 Prim.Sys.Io.Portable check");
    common::cleanup(&wdir);
}

#[cfg(target_os = "macos")]
#[test]
fn native_darwin_kqueue_tcp_reuseport_writev_udp_mmap_and_clock() {
    let main_src = r#"
module Main where
  import Algebra.Arithmetic
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Prim.Sys.Io

  -- Write a 32-bit little-endian I32 value into a fresh 4-byte buffer.
  i32-buf :: U32 -> Bytes
  i32-buf v =
    let b = bytes-new 4 in
    let _ = bytes-store-le32 b 0 v in
    b

  -- Build one 32-byte Darwin `struct kevent` buffer:
  --   ident(0..8), filter(8..10), flags(10..12), fflags(12..16), data(16..24), udata(24..32)
  make-kevent-buf :: U64 -> I64 -> I64 -> Bytes
  make-kevent-buf ident-v filter-v flags-v =
    let b = bytes-new 32 in
    let _0 = bytes-store-le64 b 0 ident-v in
    let f16 = wrapping-i64-to-u16 filter-v in
    let fl16 = wrapping-i64-to-u16 flags-v in
    let _1 = bytes-set-u8 b 8 (wrapping-u16-to-u8 f16) in
    let _2 = bytes-set-u8 b 9 (wrapping-u16-to-u8 (wrapping-shr-u16 f16 8)) in
    let _3 = bytes-set-u8 b 10 (wrapping-u16-to-u8 fl16) in
    let _4 = bytes-set-u8 b 11 (wrapping-u16-to-u8 (wrapping-shr-u16 fl16 8)) in
    b

  -- Build a 16-byte `struct timespec` buffer (`tv_sec`, `tv_nsec`).
  make-timespec :: U64 -> U64 -> Bytes
  make-timespec sec nsec =
    let b = bytes-new 16 in
    let _0 = bytes-store-le64 b 0 sec in
    let _1 = bytes-store-le64 b 8 nsec in
    b

  -- Build a 2-entry (32-byte) `struct iovec` array pointing to `a` and `b`.
  make-iovec2 :: Bytes -> Bytes -> Bytes
  make-iovec2 a b =
    let iov = bytes-new 32 in
    let _0 = bytes-store-le64 iov 0 (wrapping-i64-to-u64 (bytes-data a)) in
    let _1 = bytes-store-le64 iov 8 (wrapping-i64-to-u64 (bytes-len-i64 a)) in
    let _2 = bytes-store-le64 iov 16 (wrapping-i64-to-u64 (bytes-data b)) in
    let _3 = bytes-store-le64 iov 24 (wrapping-i64-to-u64 (bytes-len-i64 b)) in
    iov

  -- Compare first `n` bytes of `got` with `expected`.
  bytes-prefix-eq :: Bytes -> Bytes -> Nat -> Bool
  bytes-prefix-eq got expected n =
    let slice = bytes-new n in
    if n <= bytes-len got then
      if n <= bytes-len expected then
        let _ = bytes-copy-range slice got 0 0 n in
        slice == expected
      else False
    else False

  -- Busy-burn a tiny loop so monotonic clock advances measurably.
  burn :: I64 -> I64 -> I64
  burn n acc =
    if le-i64 n 0 then acc
    else burn (wrapping-sub-i64 n 1) (wrapping-add-i64 acc n)

  read-timespec-ns :: Bytes -> U64
  read-timespec-ns ts =
    if 16 <= bytes-len ts then
      let sec = bytes-load-le64 ts 0 in
      let nsec = bytes-load-le64 ts 8 in
      wrapping-add-u64 (wrapping-mul-u64 sec 1000000000) nsec
    else 0

  test-clock :: forall r. using (io :: Io r) => Unit ->{IO r} I64
  test-clock u =
    let ts1 = bytes-new 16 in
    let r1 = sys-held ts1 (sys-clock-gettime clock-monotonic (bytes-data ts1)) in
    if io-is-err r1 then 10
    else
      let t1 = read-timespec-ns ts1 in
      let _spin = burn 2000 0 in
      let ts2 = bytes-new 16 in
      let r2 = sys-held ts2 (sys-clock-gettime clock-monotonic (bytes-data ts2)) in
      if io-is-err r2 then 11
      else
        let t2 = read-timespec-ns ts2 in
        if lt-u64 0 t1 && le-u64 t1 t2 then 0 else 12

  test-mmap :: forall r. using (io :: Io r) => Unit ->{IO r} I64
  test-mmap u =
    let addr = sys-mmap 0 4096 (bit-or-i64 prot-read prot-write) (bit-or-i64 map-private map-anon) -1 0 in
    if io-is-err addr then 20
    else if eq-i64 addr 0 then 21
    else
      let rc = sys-munmap addr 4096 in
      if eq-i64 rc 0 then 0 else 22

  test-kqueue-tcp :: forall r. using (io :: Io r) => Unit ->{IO r} I64
  test-kqueue-tcp u =
    let kq = sys-kqueue () in
    if io-is-err kq then 30
    else
      let listener = sys-socket af-inet sock-stream 0 in
      if io-is-err listener then
        let _ = sys-close kq in 31
      else
        let one = i32-buf 1 in
        let r-ra = sys-held one (sys-setsockopt listener sol-socket so-reuseaddr (bytes-data one) 4) in
        let r-rp = sys-held one (sys-setsockopt listener sol-socket so-reuseport (bytes-data one) 4) in
        let r-nb = sys-set-nonblocking listener in
        if io-is-err r-ra || io-is-err r-rp || io-is-err r-nb then
          let _ = sys-close listener in
          let _ = sys-close kq in 32
        else
          let bind-addr = sys-sockaddr-in-loopback 0 in
          let r-bind = sys-held bind-addr (sys-bind listener (bytes-data bind-addr) 16) in
          if io-is-err r-bind then
            let _ = sys-close listener in
            let _ = sys-close kq in 33
          else
            let bound-addr = bytes-new 16 in
            let addr-len = i32-buf 16 in
            let r-gsn = sys-held bound-addr (sys-held addr-len (sys-getsockname listener (bytes-data bound-addr) (bytes-data addr-len))) in
            let port = sys-sockaddr-in-port bound-addr in
            let r-lis = sys-listen listener 16 in
            if io-is-err r-gsn || io-is-err r-lis || eq-u16 port 0 then
              let _ = sys-close listener in
              let _ = sys-close kq in 34
            else
              -- Register listener for EVFILT_READ in kqueue
              let chg = make-kevent-buf (wrapping-i64-to-u64 listener) evfilt-read (bit-or-i64 ev-add ev-enable) in
              let r-reg = sys-held chg (sys-kevent kq (bytes-data chg) 1 0 0 0) in
              if io-is-err r-reg then
                let _ = sys-close listener in
                let _ = sys-close kq in 35
              else
                -- Connect non-blocking client to 127.0.0.1:<port>
                let client = sys-socket af-inet sock-stream 0 in
                let r-cnb = sys-set-nonblocking client in
                let r-nd = sys-held one (sys-setsockopt client ipproto-tcp tcp-nodelay (bytes-data one) 4) in
                let dst = sys-sockaddr-in-loopback port in
                let r-conn = sys-held dst (sys-connect client (bytes-data dst) 16) in
                let conn-ok = eq-i64 r-conn 0 || io-is-in-progress r-conn in
                if io-is-err client || io-is-err r-cnb || io-is-err r-nd || not conn-ok then
                  let _ = sys-close client in
                  let _ = sys-close listener in
                  let _ = sys-close kq in 36
                else
                  -- Wait for listener readiness via kevent
                  let ev-out = bytes-new 32 in
                  let timeout = make-timespec 1 0 in
                  let nev = sys-held ev-out (sys-held timeout (sys-kevent kq 0 0 (bytes-data ev-out) 1 (bytes-data timeout))) in
                  let ready-ident = bytes-load-le64 ev-out 0 in
                  if lt-i64 nev 1 || ne-u64 ready-ident (wrapping-i64-to-u64 listener) then
                    let _ = sys-close client in
                    let _ = sys-close listener in
                    let _ = sys-close kq in 37
                  else
                    let peer-addr = bytes-new 16 in
                    let peer-len = i32-buf 16 in
                    let srv = sys-held peer-addr (sys-held peer-len (sys-accept listener (bytes-data peer-addr) (bytes-data peer-len))) in
                    if io-is-err srv then
                      let _ = sys-close client in
                      let _ = sys-close listener in
                      let _ = sys-close kq in 38
                    else
                      let _ = sys-set-nonblocking srv in
                      -- Register accepted server fd for EVFILT_READ
                      let srv-chg = make-kevent-buf (wrapping-i64-to-u64 srv) evfilt-read (bit-or-i64 ev-add ev-enable) in
                      let r-sreg = sys-held srv-chg (sys-kevent kq (bytes-data srv-chg) 1 0 0 0) in
                      -- Gather-write two buffers with writev
                      let p1 = string-to-bytes "hello " in
                      let p2 = string-to-bytes "cinch-io!" in
                      let iov = make-iovec2 p1 p2 in
                      let nw = sys-held p1 (sys-held p2 (sys-held iov (sys-writev client (bytes-data iov) 2))) in
                      if io-is-err r-sreg || ne-i64 nw 15 then
                        let _ = sys-close srv in
                        let _ = sys-close client in
                        let _ = sys-close listener in
                        let _ = sys-close kq in 39
                      else
                        -- Wait for readability on srv via kevent, then read
                        let ev2 = bytes-new 32 in
                        let nev2 = sys-held ev2 (sys-held timeout (sys-kevent kq 0 0 (bytes-data ev2) 1 (bytes-data timeout))) in
                        let rbuf = bytes-new 32 in
                        let nr = sys-held rbuf (sys-read srv (bytes-data rbuf) 32) in
                        let want = string-to-bytes "hello cinch-io!" in
                        let ok = le-i64 1 nev2 && eq-i64 nr 15 && bytes-prefix-eq rbuf want 15 in
                        let _ = sys-close srv in
                        let _ = sys-close client in
                        let _ = sys-close listener in
                        let _ = sys-close kq in
                        if ok then 0 else 40

  test-udp :: forall r. using (io :: Io r) => Unit ->{IO r} I64
  test-udp u =
    let rx = sys-socket af-inet sock-dgram 0 in
    let tx = sys-socket af-inet sock-dgram 0 in
    if io-is-err rx || io-is-err tx then
      let _ = sys-close rx in
      let _ = sys-close tx in 50
    else
      let bind-addr = sys-sockaddr-in-loopback 0 in
      let r-bind = sys-held bind-addr (sys-bind rx (bytes-data bind-addr) 16) in
      let bound-addr = bytes-new 16 in
      let addr-len = i32-buf 16 in
      let r-gsn = sys-held bound-addr (sys-held addr-len (sys-getsockname rx (bytes-data bound-addr) (bytes-data addr-len))) in
      let port = sys-sockaddr-in-port bound-addr in
      if io-is-err r-bind || io-is-err r-gsn || eq-u16 port 0 then
        let _ = sys-close rx in
        let _ = sys-close tx in 51
      else
        let dst = sys-sockaddr-in-loopback port in
        let msg = string-to-bytes "udp-ping" in
        let ns = sys-held msg (sys-held dst (sys-sendto tx (bytes-data msg) 8 0 (bytes-data dst) 16)) in
        let rbuf = bytes-new 32 in
        let src = bytes-new 16 in
        let slen = i32-buf 16 in
        let nr = sys-held rbuf (sys-held src (sys-held slen (sys-recvfrom rx (bytes-data rbuf) 32 0 (bytes-data src) (bytes-data slen)))) in
        let ok = eq-i64 ns 8 && eq-i64 nr 8 && bytes-prefix-eq rbuf msg 8 in
        let _ = sys-close rx in
        let _ = sys-close tx in
        if ok then 0 else 52

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = test-clock () in
    if ne-i64 c1 0 then int-from-i64 c1
    else
      let c2 = test-mmap () in
      if ne-i64 c2 0 then int-from-i64 c2
      else
        let c3 = test-kqueue-tcp () in
        if ne-i64 c3 0 then int-from-i64 c3
        else
          let c4 = test-udp () in
          int-from-i64 c4
"#;

    let mut files = PRIM_SYS_IO_FILES.to_vec();
    files.push(("Main.cinch", main_src));
    let dir = common::project("prim-sys-io-darwin-live", &files);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 0, "native Darwin kqueue/TCP/writev/UDP/mmap/clock test exited with {code}");
}
