//! Integration tests for `Engine.Types`: frozen `#[repr(value)]` /
//! `#[repr(C, value)]` ABI layouts, bit-packing round-trips (`GenHandle`,
//! `IoToken`), `ByteSpan`/`MsgEnvelope`/`IoSqe`/`Completion` POD contracts,
//! `#[no-alloc]` verification + zero heap allocations in tight loops,
//! capability dictionaries (`ReactorOps`, `ClockOps`, `EntropyOps`), and
//! Wave 0A runtime FFI wrappers.
//!
//! Routed through `common::project` / `common::drive` / `common::check_interfaces` /
//! `common::build_executable` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::driver::{Mode, Package};
use std::process::Command;

const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_engine_types(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) {
    let dir = project_with_engine_types(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "engine-types-test")
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
fn engine_types_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Engine.Types")
        .expect("Engine.Types listed in seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Engine/Types.cinch");
    assert!(
        pkg.order.iter().any(|name| name == "Engine.Types"),
        "Engine.Types listed in seed/prelude/.cinchpkg order"
    );
}

#[test]
fn engine_types_abi_layouts_and_register_passing() {
    let src = r#"
module Main where
  import Engine.Types

  #[no-alloc]
  #[inline(never)]
  id-span :: ByteSpan -> ByteSpan
  id-span s = s

  #[no-alloc]
  #[inline(never)]
  id-cqe :: Completion -> Completion
  id-cqe c = c

  #[no-alloc]
  #[inline(never)]
  id-env :: MsgEnvelope -> MsgEnvelope
  id-env m = m

  main :: Int
  main =
    let s = id-span (byte-span-make 1 2 3 4) in
    let c = id-cqe (completion-make (io-token-pack 1 2 3) 64 0) in
    let m = id-env (msg-envelope-make 9 (core-id 1) (core-id 2) 100 1 2 3 4 5 6) in
    if eq-u32 s.span-len 3 && eq-i32 c.cqe-res 64 && eq-u64 m.msg-seq 100 then 0 else 1
"#;

    let dir = project_with_engine_types("engine-types-abi", src);

    // 1. Verify `.cinchif` `abi.layouts` for `Engine.Types`.
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    let eng = ifaces
        .iter()
        .find(|i| i.module == "Engine.Types")
        .expect("Engine.Types interface emitted");

    let layout_of = |name: &str| {
        eng.layouts
            .iter()
            .find(|l| l.name == name)
            .unwrap_or_else(|| panic!("missing layout for {name} in Engine.Types"))
    };

    // Unboxed handles
    let core_id_l = layout_of("CoreId");
    assert_eq!(core_id_l.size, 2);
    assert_eq!(core_id_l.align, 2);

    let fd_l = layout_of("Fd");
    assert_eq!(fd_l.size, 4);
    assert_eq!(fd_l.align, 4);

    let slab_l = layout_of("SlabIdx");
    assert_eq!(slab_l.size, 4);
    assert_eq!(slab_l.align, 4);

    let buf_l = layout_of("BufId");
    assert_eq!(buf_l.size, 4);
    assert_eq!(buf_l.align, 4);

    let time_l = layout_of("TimeNs");
    assert_eq!(time_l.size, 8);
    assert_eq!(time_l.align, 8);

    let gh_l = layout_of("GenHandle");
    assert_eq!(gh_l.size, 8);
    assert_eq!(gh_l.align, 8);

    let tok_l = layout_of("IoToken");
    assert_eq!(tok_l.size, 8);
    assert_eq!(tok_l.align, 8);

    // ByteSpan == 16B (fits in `{ i64, i64 }` registers)
    let span_l = layout_of("ByteSpan");
    assert_eq!(span_l.size, 16, "ByteSpan layout: {span_l:?}");
    assert_eq!(span_l.align, 4, "ByteSpan align: {span_l:?}");
    assert_eq!(span_l.fields.len(), 4);
    assert_eq!(span_l.fields[0].offset, 0);
    assert_eq!(span_l.fields[1].offset, 4);
    assert_eq!(span_l.fields[2].offset, 8);
    assert_eq!(span_l.fields[3].offset, 12);

    // Completion == 16B (fits in `{ i64, i64 }` registers)
    let cqe_l = layout_of("Completion");
    assert_eq!(cqe_l.size, 16, "Completion layout: {cqe_l:?}");
    assert_eq!(cqe_l.align, 8, "Completion align: {cqe_l:?}");
    assert_eq!(cqe_l.fields.len(), 3);
    assert_eq!(cqe_l.fields[0].offset, 0);
    assert_eq!(cqe_l.fields[1].offset, 8);
    assert_eq!(cqe_l.fields[2].offset, 12);

    // IoSqe == 40B POD
    let sqe_l = layout_of("IoSqe");
    assert_eq!(sqe_l.size, 40, "IoSqe layout: {sqe_l:?}");
    assert_eq!(sqe_l.align, 8, "IoSqe align: {sqe_l:?}");
    assert_eq!(sqe_l.fields.len(), 9);
    assert_eq!(sqe_l.fields[0].offset, 0);
    assert_eq!(sqe_l.fields[1].offset, 1);
    assert_eq!(sqe_l.fields[2].offset, 2);
    assert_eq!(sqe_l.fields[3].offset, 4);
    assert_eq!(sqe_l.fields[4].offset, 8);
    assert_eq!(sqe_l.fields[5].offset, 16);
    assert_eq!(sqe_l.fields[6].offset, 20);
    assert_eq!(sqe_l.fields[7].offset, 24);
    assert_eq!(sqe_l.fields[8].offset, 32);

    // MsgEnvelope == 64B align 64 (exact cache line)
    let env_l = layout_of("MsgEnvelope");
    assert_eq!(env_l.size, 64, "MsgEnvelope layout: {env_l:?}");
    assert_eq!(env_l.align, 64, "MsgEnvelope align: {env_l:?}");
    assert_eq!(env_l.fields.len(), 10);
    assert_eq!(env_l.fields[0].offset, 0);
    assert_eq!(env_l.fields[1].offset, 4);
    assert_eq!(env_l.fields[2].offset, 6);
    assert_eq!(env_l.fields[3].offset, 8);
    assert_eq!(env_l.fields[4].offset, 16);
    assert_eq!(env_l.fields[9].offset, 56);

    // 2. Verify LLVM IR register-passing ABI for 16B & 64B types.
    let ir_path = dir.join("program.ll");
    common::drive(Mode::EmitIr, &dir, "Main.cinch", Some(ir_path.clone())).expect("emit IR");
    let ir = std::fs::read_to_string(&ir_path).expect("read IR");
    let def_span = ir
        .lines()
        .find(|l| l.starts_with("define ") && l.contains("Main.id-span"))
        .expect("Main.id-span in IR");
    assert!(
        def_span.contains("{ i64, i64 }"),
        "ByteSpan should pass/return in {{ i64, i64 }} registers: {def_span}"
    );
    let def_cqe = ir
        .lines()
        .find(|l| l.starts_with("define ") && l.contains("Main.id-cqe"))
        .expect("Main.id-cqe in IR");
    assert!(
        def_cqe.contains("{ i64, i64 }"),
        "Completion should pass/return in {{ i64, i64 }} registers: {def_cqe}"
    );
    let def_env = ir
        .lines()
        .find(|l| l.starts_with("define ") && l.contains("Main.id-env"))
        .expect("Main.id-env in IR");
    assert!(
        def_env.contains("sret([64 x i8])") && def_env.contains("align 64"),
        "MsgEnvelope should pass/return indirectly with align 64: {def_env}"
    );

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

#[test]
fn engine_types_pack_unpack_roundtrip_and_zero_alloc_hot_loop() {
    let src = r#"
module Main where
  import Engine.Types

  #[no-alloc]
  both :: Bool -> Bool -> Bool
  both a b = if a then b else eq-u32 0 1

  #[no-alloc]
  check-gen-handle :: U32 -> U32 -> Bool
  check-gen-handle g i =
    let h = gen-handle-pack g i in
    both (eq-u32 (gen-handle-gen h) g) (eq-u32 (gen-handle-idx h) i)

  #[no-alloc]
  check-io-token :: U8 -> U32 -> U32 -> Bool
  check-io-token k g s =
    let masked-g = bit-and-u32 g 16777215 in
    let t = io-token-pack k g s in
    both
      (both (eq-u8 (io-token-kind t) k) (eq-u32 (io-token-gen t) masked-g))
      (eq-u32 (io-token-slot t) s)

  check-boundaries :: Bool
  check-boundaries =
    -- GenHandle boundary values: 0, 1, max 32-bit, mixed bit patterns
    gen-handle-is-null (gen-handle-null ())
      && check-gen-handle 0 0
      && check-gen-handle 0 1
      && check-gen-handle 1 0
      && check-gen-handle 1 1
      && check-gen-handle 4294967295 0
      && check-gen-handle 0 4294967295
      && check-gen-handle 4294967295 4294967295
      && check-gen-handle 2863311530 1431655765
      && check-gen-handle 3735928559 3405691582
      && not (gen-handle-is-null (gen-handle-pack 0 1))
      && not (gen-handle-is-null (gen-handle-pack 1 0))
      -- IoToken boundary values: 0, 1, max 8-bit / 24-bit / 32-bit, overflow masking
      && io-token-is-zero (io-token-zero ())
      && check-io-token 0 0 0
      && check-io-token 1 1 1
      && check-io-token 255 16777215 4294967295
      && check-io-token 255 0 0
      && check-io-token 0 16777215 0
      && check-io-token 0 0 4294967295
      && check-io-token 170 4294967295 305419896
      && check-io-token 128 11184810 2147483648
      && not (io-token-is-zero (io-token-pack 1 0 0))

  check-span-sqe-cqe-env :: Bool
  check-span-sqe-cqe-env =
    let s0 = byte-span-of (buf-id-of 7) 100 50 in
    let s1 = byte-span-slice s0 10 20 in
    let s2 = byte-span-slice s0 40 999 in
    let s3 = byte-span-slice s0 50 10 in
    let tok = io-token-pack op-code-recv 123456 987654 in
    let sqe = io-sqe-make OpRecv 3 (fd-of 42) 65536 4096 0 8192 tok in
    let nop = io-sqe-nop () in
    let cqe = completion-make tok 128 0 in
    let cqe-err = completion-make tok (-11) 0 in
    let cqe0 = completion-empty () in
    let env = msg-envelope-make 42 (core-id 1) (core-id 3) 999 11 22 33 44 55 66 in
    let env0 = msg-envelope-empty () in
    byte-span-is-empty (byte-span-empty ())
      && not (fd-is-valid (fd-invalid ()))
      && not (slab-idx-is-valid (slab-idx-invalid ()))
      && not (buf-id-is-valid (buf-id-invalid ()))
      && eq-u64 (time-ns-value (time-ns-zero ())) 0
      && eq-u64 env0.msg-seq 0
      && io-op-kind-eq (io-sqe-op-kind nop) OpNop
      && eq-i32 cqe0.cqe-res 0
      && not (byte-span-is-empty s0)
      && eq-u32 s1.span-buf 7 && eq-u32 s1.span-off 110 && eq-u32 s1.span-len 20
      && eq-u32 s2.span-off 140 && eq-u32 s2.span-len 10
      && byte-span-is-empty s3
      && io-op-kind-eq (io-sqe-op-kind sqe) OpRecv
      && io-token-eq (io-sqe-token-id sqe) tok
      && not (completion-is-err cqe)
      && completion-is-err cqe-err
      && io-token-eq (completion-token cqe) tok
      && eq-u32 env.msg-kind 42 && eq-u16 env.msg-src 1 && eq-u16 env.msg-dst 3
      && eq-u64 env.msg-seq 999 && eq-u64 env.msg-w5 66
      && io-error-is-ok IoOk
      && io-error-eq (io-error-from-u8 (io-error-to-u8 IoTimedOut)) IoTimedOut
      && io-op-kind-eq (io-op-kind-from-u8 (io-op-kind-to-u8 OpSendZc)) OpSendZc

  #[no-alloc]
  tight-loop :: U32 -> U64 -> U64
  tight-loop i acc =
    if eq-u32 i 0 then acc
    else
      let g = wrapping-mul-u32 i 2654435761 in
      let idx = wrapping-add-u32 i 13 in
      let h = gen-handle-pack g idx in
      let ok-h = both (eq-u32 (gen-handle-gen h) g) (eq-u32 (gen-handle-idx h) idx) in
      let k = wrapping-u32-to-u8 i in
      let t = io-token-pack k g idx in
      let ok-t = both (both (eq-u8 (io-token-kind t) k) (eq-u32 (io-token-gen t) (bit-and-u32 g 16777215))) (eq-u32 (io-token-slot t) idx) in
      let span = byte-span-slice (byte-span-make idx 0 1024 0) 16 128 in
      let sqe = IoSqe op-code-send 0 0 5 4096 span.span-len 0 0 t.io-token-raw in
      let cqe = completion-make (io-sqe-token-id sqe) 128 0 in
      let env = msg-envelope-make idx (core-id 0) (core-id 1) h.gen-handle-raw t.io-token-raw cqe.cqe-token 0 0 0 0 in
      let ok-all = both (both ok-h ok-t) (eq-u64 env.msg-seq h.gen-handle-raw) in
      let next-acc = if ok-all then wrapping-add-u64 acc 1 else acc in
      tight-loop (wrapping-sub-u32 i 1) next-acc

  main :: Unit ->{IO} Int
  main _ =
    if not check-boundaries then 1
    else if not check-span-sqe-cqe-env then 2
    else
      let _ = engine-thread-counters-reset () in
      let passed = tight-loop 50000 0 in
      let allocs = engine-thread-alloc-count () in
      let atomic-rcs = engine-thread-atomic-rc-count () in
      if ne-u64 passed 50000 then 3
      else if ne-u64 allocs 0 then 4
      else if ne-u64 atomic-rcs 0 then 5
      else
        let cpus = engine-cpu-count () in
        if lt-u32 cpus 1 then 6
        else
          let dma = engine-dma-alloc 4096 4096 in
          if eq-u64 dma 0 then 7
          else if ne-u64 (bit-and-u64 dma 4095) 0 then 8
          else
            let freed = engine-dma-free dma 4096 in
            if not (io-error-is-ok freed) then 9
            else 0
"#;

    let dir = project_with_engine_types("engine-types-check", src);
    common::check(&dir, "Main.cinch").expect("cinch check passes");
    common::cleanup(&dir);

    run_leak_checked("engine-types-hot-loop", src, 0);
}

#[test]
fn engine_types_capability_dictionaries_dst_and_live_interface() {
    let src = r#"
module Main where
  import Engine.Types

  -- Generic protocol step parameterized solely over `ReactorOps`, `ClockOps`,
  -- and `EntropyOps` capabilities so live and DST backends share the exact
  -- same code with zero drift.
  protocol-tick :: forall (r :: Region). using (rx :: ReactorOps r, clk :: ClockOps r, rng :: EntropyOps r) => Fd ->{IO r, Mut r} U64
  protocol-tick fd =
    let t0 = engine-clock-mono-ns () in
    let rnd = engine-entropy-next-u32 () in
    let tok = io-token-pack op-code-recv rnd 7 in
    let sqe = io-sqe-make OpRecv 0 fd 4096 64 0 0 tok in
    let err = engine-reactor-submit sqe in
    if not (io-error-is-ok err) then 0
    else
      let _ = engine-reactor-flush () in
      let n = engine-reactor-wait (time-ns-of 1000) in
      if lt-i32 n 1 then 0
      else
        let cqe = engine-reactor-pop-cqe () in
        let t1 = engine-clock-mono-ns () in
        wrapping-add-u64 (u32-to-u64 (io-token-slot (completion-token cqe))) (time-ns-saturating-sub t1 t0)

  make-mock-reactor :: forall (r :: Region). ReactorOps r
  make-mock-reactor = ReactorOps {
    reactor-submit = \_ -> IoOk,
    reactor-flush = \_ -> 1,
    reactor-wait = \_ -> 1,
    reactor-cqe-count = \_ -> 1,
    reactor-pop-cqe = \_ -> completion-make (io-token-pack op-code-recv 99 7) 64 0,
    reactor-close = \_ -> IoOk
  }

  make-mock-clock :: forall (r :: Region). ClockOps r
  make-mock-clock = ClockOps {
    clock-mono-ns = \_ -> time-ns-of 1000000,
    clock-wall-ns = \_ -> time-ns-of 1700000000000000000
  }

  make-mock-entropy :: forall (r :: Region). EntropyOps r
  make-mock-entropy = EntropyOps {
    entropy-next-u64 = \_ -> 42,
    entropy-next-u32 = \_ -> 99
  }

  main :: forall (r :: Region). Unit ->{IO r, Mut r} Int
  main _ =
    let res = with-engine-ops make-mock-reactor make-mock-clock make-mock-entropy (\() -> protocol-tick (fd-of 3)) in
    if eq-u64 res 7 then 0 else 1
"#;

    let dir = project_with_engine_types("engine-types-cap-check", src);
    common::check(&dir, "Main.cinch").expect("capability dicts check");
    common::cleanup(&dir);

    run_leak_checked("engine-types-cap-dicts", src, 0);
}
