# Retained transfer callback cycle

These frozen sources record a known leak, not a completed buffered-library or runtime fix. The source modules are the exact unpublished flush-capability draft used by the observed runs; they do not replace production modules.

The first transfer returns a source retaining its supplied write operation, which owns a private sink-continuation cell. The second transfer returns a sink retaining its supplied read operation, which owns a source-continuation cell. Updating the first cell closes the indirect strong-reference cycle. The client does not create mutable cells.

`original.cinchpart` records the first minimal composition. In `eof.cinchpart`, the source transfer writes its complete `AB` input and the sink transfer performs an actual positive read receiving end-of-input before either operation returns. Both runs return normally from `main`, then exit74 under the leak checker with22 live, non-immortal blocks. Neither poisoned nor Native runs are claimed in this receipt.

`receipt.json` identifies all sources, the compiler, runtime, LLVM and executable bytes, exact commands, fixed memory limits, and complete observed diagnostics. IR and executable artifacts remain outside Git.

To verify the existing failure with the frozen producer and runtime:

```sh
source /workspace/.tools/cinch-env.sh
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RUST_MIN_STACK=67108864
python3 seed/runtime/repros/buffered-transfer-cycle/replay.py --case eof --artifacts build/cycle-eof-baseline
```

The replay tool accepts a separately identified runtime via `--runtime` and requires `--expect-reclaimed` before treating it as a collector candidate. That mode preserves the source and compiler identities and requires zero live/leaked blocks; it never relaxes the leak check. `--poison` selects the separate256MiB poisoned profile. All consumer stacks remain8MiB; the compiler-host stack is64MiB.
