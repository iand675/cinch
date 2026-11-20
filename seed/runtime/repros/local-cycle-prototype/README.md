# Local cycle prototype receipt

This checkpoint is an isolated **test-only** reclamation experiment. All registry hooks and the collector module use `cfg(test)`; ordinary builds have no collector entry point or hooks. Allocation-time collection and `cinch_exit_check` are unchanged. The precise production safety gaps are recorded in [the design review](../../CYCLE-COLLECTOR-DESIGN.md).

Nine focused units pass in each of two fresh processes:

| Profile | Heap cap | Native shell stack | RC debug | Tests | Managed allocated/freed | Live |
| --- | --- | --- | --- | --- | --- | --- |
| Normal | 32 MiB | 8 MiB | 0 | 9 | 10,143 / 10,143 | 0 |
| Poisoned | 256 MiB | 8 MiB | 1 | 9 | 10,143 / 10,143 | 0 |

The ring test explicitly starts its worker with a 128 KiB native stack and reclaims 10,000 cells iteratively. Other units cover live aliases and exact original counts, uniqueness, closure captures, duplicate edges, a shared live tail, node/edge/scan-word budget aborts, synchronized concurrent writes to an excluded atomic child's payload, and immediate registry retirement before free/address reuse. The unit harness's allocation census provides the zero-live evidence; it does not invoke an exit collector or print a `cinch-leak` line.

The frozen original and valid-EOF buffered transfer examples still have their separately recorded 22-block leak. This prototype has **not** been enabled for those compiled consumers. Buffered-I/O retention semantics and package promotion remain held for review.

## Reproduce the focused tests

Use a private target and the repository's tool environment. This runtime crate has no dependencies and requires no external SDK or compiler bootstrap.

```sh
source /workspace/.tools/cinch-env.sh
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RUST_MIN_STACK=67108864
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_TARGET_DIR="$PWD/build/runtime-unit-target"
ulimit -s 65536
taskset -c 2 cargo test --manifest-path seed/runtime/Cargo.toml --lib cycle::tests -- --test-threads=1
```

Run the emitted unit executable reported by Cargo in separate processes with `cycle::tests --test-threads=1`. Set `CINCH_ALLOC_STATS=1`, `CINCH_LEAK_CHECK=1`, `CINCH_STACK_LIMIT=8M`, and an 8 MiB shell stack. Use `CINCH_HEAP_LIMIT=32M CINCH_RC_DEBUG=0` for normal execution and `CINCH_HEAP_LIMIT=256M CINCH_RC_DEBUG=1` for poisoning. The exact observed executable identity, compiler version, source identities, commands and retained logs are in [receipt.json](receipt.json). Build artifacts stay outside Git; both the initial and final focused executable epochs remain under the lane's `build` directory.

The complete runtime test suite, foreign targets, production quiescence, task-result transfer, cooperative context migration, and production collection overhead are outside the claims of this receipt.
