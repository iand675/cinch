//! Runtime integration tests for share-nothing pinned OS worker threads,
//! 4 KiB-aligned DMA `mmap` slabs, and per-thread allocation & atomic-RC
//! telemetry counters (`seed/runtime/src/engine_thread.rs`).

mod common;

use common::source_frontend::{run_source, run_source_objects};

/// Compile a small C fixture to an object file and return its path.
fn c_fixture(name: &str, src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("cinch-engine-rt-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write fixture");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = std::process::Command::new(&clang)
        .arg("-O2")
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile C fixture");
    o_path.to_string_lossy().into_owned()
}

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation/run failed: {e}"),
    }
}

fn expect_ok_objects(name: &str, src: &str, objects: &[&str], expected: i32) {
    match run_source_objects(src, objects) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation/run failed: {e}"),
    }
}

#[test]
fn dma_alloc_alignment_and_free_from_cinch() {
    let src = r#"
module Main where
  foreign import ccall "cinch_engine_cpu_count" cpuCount :: Unit ->{IO} I64
  foreign import ccall "cinch_engine_dma_alloc" dmaAlloc :: I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_dma_free" dmaFree :: I64 -> I64 ->{IO} I64

  main :: Unit ->{IO} Int
  main _ =
    let cpus = cpuCount () in
    if cpus < 1 then 1
    else
      let p4 = dmaAlloc 4096 4096 in
      if p4 == 0 then 2
      else if bit-and-i64 p4 4095 /= 0 then 3
      else
        let r4 = dmaFree p4 4096 in
        if r4 /= 0 then 4
        else
          let p64 = dmaAlloc 16384 65536 in
          if p64 == 0 then 5
          else if bit-and-i64 p64 65535 /= 0 then 6
          else
            let r64 = dmaFree p64 16384 in
            if r64 /= 0 then 7
            else
              let bad = dmaAlloc 4096 1024 in
              if bad /= 0 then 8
              else 0
"#;
    expect_ok("dma-alloc-alignment", src, 0);
}

#[test]
fn pinned_workers_spawn_join_and_observe_zero_alloc_zero_atomic_rc_in_no_alloc_loop() {
    // Minimal C bridge that exposes Cinch-exported worker entry points as
    // static immortal Cinch closure records (`{ header: (KIND_CLOSURE << 40),
    // code: stub, arity: 1, nfilled: 0 }`) matching `seed/runtime/src/task.rs`.
    let bridge = c_fixture(
        "engine_worker_bridge",
        r#"
#include <stdint.h>

extern int64_t worker_pure_loop(int64_t arg);
extern int64_t worker_local_alloc(int64_t arg);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

/* KIND_CLOSURE = 2, bits 35..40 (`block_kinds.rs`: RC_BITS = 35); rc = 0 (RC_IMMORTAL). */
#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

static int64_t stub_pure_loop(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return worker_pure_loop(argv[0]);
}

static int64_t stub_local_alloc(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return worker_local_alloc(argv[0]);
}

static CinchStaticClosure clo_pure_loop;
static CinchStaticClosure clo_local_alloc;

int64_t engine_get_pure_loop_closure(void) {
    clo_pure_loop.header = IMMORTAL_CLOSURE_HEADER;
    clo_pure_loop.code = (int64_t)(uintptr_t)stub_pure_loop;
    clo_pure_loop.arity = 1;
    clo_pure_loop.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_pure_loop.code;
}

int64_t engine_get_local_alloc_closure(void) {
    clo_local_alloc.header = IMMORTAL_CLOSURE_HEADER;
    clo_local_alloc.code = (int64_t)(uintptr_t)stub_local_alloc;
    clo_local_alloc.arity = 1;
    clo_local_alloc.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_local_alloc.code;
}
"#,
    );

    let src = r#"
module Main where
  foreign import ccall "cinch_engine_spawn_pinned" spawnPinned :: I64 -> I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_join" joinPinned :: I64 ->{IO} I64
  foreign import ccall "cinch_engine_thread_alloc_count" threadAllocCount :: Unit ->{IO} I64
  foreign import ccall "cinch_engine_thread_atomic_rc_count" threadAtomicRcCount :: Unit ->{IO} I64
  foreign import ccall "cinch_engine_thread_counters_reset" resetCounters :: Unit ->{IO} I64
  foreign import ccall "engine_get_pure_loop_closure" getPureClosure :: Unit ->{IO} I64
  foreign import ccall "engine_get_local_alloc_closure" getLocalAllocClosure :: Unit ->{IO} I64

  #[no-alloc]
  sumLoop :: I64 -> I64 -> I64
  sumLoop n acc =
    if n <= 0 then acc
    else sumLoop (wrapping-sub-i64 n 1) (wrapping-add-i64 acc n)

  workerPureLoop :: I64 ->{IO} I64
  workerPureLoop n =
    let _ = resetCounters () in
    let total = sumLoop n 0 in
    let allocs = threadAllocCount () in
    let atomicRcs = threadAtomicRcCount () in
    if allocs /= 0 then -100
    else if atomicRcs /= 0 then -200
    else total

  foreign export ccall "worker_pure_loop" workerPureLoop :: I64 ->{IO} I64

  buildList :: I64 -> List I64 -> List I64
  buildList n acc =
    if n <= 0 then acc
    else buildList (wrapping-sub-i64 n 1) (Cons n acc)

  sumList :: List I64 -> I64 -> I64
  sumList xs acc = case xs of
    Nil -> acc
    Cons h t -> sumList t (wrapping-add-i64 acc h)

  workerLocalAlloc :: I64 ->{IO} I64
  workerLocalAlloc n =
    let _ = resetCounters () in
    let xs = buildList n Nil in
    let s = sumList xs 0 in
    let allocs = threadAllocCount () in
    let atomicRcs = threadAtomicRcCount () in
    if allocs <= 0 then -300
    else if atomicRcs /= 0 then -400
    else s

  foreign export ccall "worker_local_alloc" workerLocalAlloc :: I64 ->{IO} I64

  main :: Unit ->{IO} Int
  main _ =
    let pureClo = getPureClosure () in
    let allocClo = getLocalAllocClosure () in
    let h0 = spawnPinned 0 pureClo 100 in
    let h1 = spawnPinned 1 pureClo 200 in
    let h2 = spawnPinned 2 pureClo 300 in
    let h3 = spawnPinned 3 allocClo 50 in
    if h0 <= 0 || h1 <= 0 || h2 <= 0 || h3 <= 0 then 1
    else
      let r0 = joinPinned h0 in
      let r1 = joinPinned h1 in
      let r2 = joinPinned h2 in
      let r3 = joinPinned h3 in
      if r0 /= 5050 then 2
      else if r1 /= 20100 then 3
      else if r2 /= 45150 then 4
      else if r3 /= 1275 then 5
      else if joinPinned h0 /= -1 then 6
      else 0
"#;
    expect_ok_objects("pinned-workers-and-telemetry", src, &[&bridge], 0);
}
