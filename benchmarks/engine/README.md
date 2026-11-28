# Cinch Engine Benchmark & Formal-Gate Harness (Waves 1–7)

Reproducible microbenchmark suite, Linux hardware-counter capture, external HTTP/KV load-generator driver, baseline regression checker, and unified formal-verification gate sweep for the Cinch high-performance engine (`Engine.*`, `Net.*`, `Store.*`, `Stream.*`, `Consensus.*`).

## 1. Prerequisites

Build the release Cinch compiler and runtime:

```bash
cargo build --release --manifest-path seed/Cargo.toml --bin cinch
cargo build --release --manifest-path seed/runtime/Cargo.toml
```

(Or pass `--compiler build/bootstrap/stage3/cinch` once the Stage 3 self-hosted compiler is built.)

## 2. Running Wave 1 Microbenchmarks

`benchmarks/engine/microbench.cinch` is a pure Cinch executable that times each Wave 1 primitive using `Effect.Clock.clock-monotonic-ns` and enforces `engine-thread-alloc-count () == 0` and `engine-thread-atomic-rc-count () == 0` on every single hot loop:

| Metric | Module | Workload | Invariant |
| --- | --- | --- | --- |
| `slab_alloc_free` | `Engine.Mem.Slab.RawSlab` | 500,000 alloc/write/read/free cycles | `allocs == 0`, `atomic_rc == 0` |
| `bump_arena_alloc_reset` | `Engine.Mem.Arena.BumpArena` | 500,000 aligned alloc + 1-write reset cycles | `allocs == 0`, `atomic_rc == 0` |
| `dma_pool_acquire_release` | `Engine.Mem.DmaPool` | 500,000 `%1` linear acquire/touch/release cycles | `allocs == 0`, `atomic_rc == 0` |
| `spsc_ring_batch_push_pop` | `Engine.Ring.Spsc` | 1,000,000 64B `MsgEnvelope`s (64-msg batches) | `allocs == 0`, `atomic_rc == 0` |
| `timer_wheel_sched_cancel` | `Engine.Timer` | 500,000 hierarchical schedule + cancel cycles | `allocs == 0`, `atomic_rc == 0` |
| `simd_scan_crlf` | `Engine.Simd.Scan` | 250,000 × 4 KiB (~1.024 GB) DMA CRLF scan | `allocs == 0`, `atomic_rc == 0` |
| `simd_crc32c_4k` | `Engine.Simd.Crc32c` | 250,000 × 4 KiB (~1.024 GB) HW Castagnoli CRC32C | `allocs == 0`, `atomic_rc == 0` |
| `simd_ws_mask_4k` | `Engine.Simd.Mask` | 250,000 × 4 KiB (~1.024 GB) 128-bit SIMD WS XOR | `allocs == 0`, `atomic_rc == 0` |

Run the suite and save a structured JSON report:

```bash
python3 benchmarks/engine/run_suite.py --out build/engine-bench-report.json
```

Compare against `benchmarks/engine/baselines.json` (exits non-zero if any zero-alloc/atomic-RC invariant fails or throughput regresses beyond threshold):

```bash
python3 benchmarks/engine/compare_baseline.py build/engine-bench-report.json
```

To refresh `baselines.json` after intentional performance improvements (refuses if any zero-alloc check failed):

```bash
python3 benchmarks/engine/compare_baseline.py build/engine-bench-report.json --update-baseline
```

### Linux Hardware Counters (`perf stat`)

On Linux hosts with `perf` installed, pass `--perf` to capture `cycles`, `instructions`, `IPC`, `L1-dcache-load-misses`, `LLC-load-misses`, `branch-misses`, and context switches (`cs`):

```bash
python3 benchmarks/engine/run_suite.py --perf --out build/engine-bench-linux.json
```

On macOS or hosts without `perf`, `--perf` reports `status: "skipped"` gracefully without failing the run.

## 3. External HTTP / QUIC / KV / WAL Benchmarks (Waves 3–7)

Target baselines in `benchmarks/engine/baselines.json` track parity with H2O (HTTP/1.1 & HTTP/2), `http.zig` (zero-alloc HTTP/1.1), Cloudflare `quiche` (HTTP/3 & QUIC), and DragonflyDB (share-nothing RESP3 KV).

Pass `--external` along with running server endpoints to invoke installed external load generators (`wrk`, `wrk2`, `h2load`, `redis-benchmark`). Missing tools are reported cleanly as `skipped: tool not installed`:

```bash
# HTTP/1.1 (wrk + wrk2) & HTTP/2 (h2load)
python3 benchmarks/engine/run_suite.py --external \
  --http1-url http://127.0.0.1:8080/plaintext \
  --http2-url https://127.0.0.1:8443/plaintext \
  --threads 4 --connections 64 --duration 15 \
  --out build/engine-http-report.json

# Share-nothing RESP3 KV (redis-benchmark)
python3 benchmarks/engine/run_suite.py --external \
  --kv-host 127.0.0.1 --kv-port 6379 \
  --connections 64 \
  --out build/engine-kv-report.json
```

## 4. Unified Formal-Verification Gate Sweep

Run all registered Wave 1–7 formal models (`M1`–`M12` discovered automatically under `seed/prelude/{Engine,Net,Store,Stream,Consensus}/**/Spec.cinch`), 128-byte cache-line isolation audit (`Engine/Ring/Spsc.cinch`), JPL-strict LRAT primitive proof checks (`Prim/SimdExt.cinch`), and unsafe boundary contract audits (`Prim/Atomic.cinch`, `Prim/Sys/Io/{Linux,Darwin}.cinch`) in a single command:

```bash
python3 tooling/verification/run_formal_gates.py --wave-sweep
```

Emit pure machine-readable JSON for CI artifacts:

```bash
python3 tooling/verification/run_formal_gates.py --wave-sweep --json
```

The sweep fails closed (exit code 1) if any model hits `capped == true`, fails any safety/liveness contract or refinement, or if any primitive, cache-line, or syscall boundary audit reports a violation.
