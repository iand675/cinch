# Bounded full-runtime test executable plan

Status: source prepared only. No full-runtime compilation or new runtime archive has started. Parent approval is required before the command below runs. The prior bounded helper gates are sealed in commit `a2f11aef`.

Before applying the proposal in this lane, every baseline source and every proposed source was checked against the original `manifest.json`; all hashes matched. The exact original patch was then applied only under `/workspace/dasl-stdlib-entropy/compiler/runtime`. The original F4F0 platform source copies remain plain in `baseline/`, and the frozen D91 runtime and Root lane remain unchanged. A test-only `entropy_boundary_tests` module is the sole addition beyond the original provider proposal. Its source and the actual candidate tree hashes are recorded alongside this plan.

The runtime package has no dependencies or build script. Build one stripped test executable directly with rustc rather than asking Cargo to emit its staticlib, cdylib, and rlib. The compile-time environment must set `CARGO_MANIFEST_DIR=/workspace/dasl-stdlib-entropy/compiler/runtime`, because the existing syscall declaration audit uses `env!` for its source walk. Use a new output epoch and place linker/compiler temporary files under that epoch through `TMPDIR`; preserve any failure products and diagnostics.

Proposed command, with the same rustc 1.100.0 identity as the previous gates:

```text
taskset -c 4 /workspace/.tools/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
  --edition=2021 --test --crate-name cinchruntime
  -C debuginfo=0 -C strip=symbols -C codegen-units=1 -C opt-level=0
  /workspace/dasl-stdlib-entropy/compiler/runtime/src/lib.rs
  -o NEW_EPOCH/runtime-tests
```

This command emits a test executable only; it does not produce or replace `libcinchruntime.a`. The estimate is 5–15 MiB final executable plus 5–15 MiB transient codegen/linker scratch; this estimate is not measured and may be too high for the unit budget. Require at least 350 MiB free immediately before starting, retain a 250 MiB reserve, and monitor the entire output/scratch directory with an absolute 30 MiB cap. Stop the compiler process group on a guard breach; retain its outputs as a failed epoch. Do not automatically retry with another configuration. The existing successful standalone outputs remain plain and occupy about 3.23 MiB total. Current observed free space is 368 MiB; recheck at the actual scheduling boundary. One CPU-4 child at a time, no benchmark or Cargo child, and an initial 60-second compile guard are proposed.

After compilation, capture `--list` and run only selected relevant tests with `--test-threads=1`. The proposed public module has three documented tests: invalid signed lengths (-1 and i64::MIN) return exact `-INVALID_ARGUMENT` without allocating a header; actual draws twice each at 0, 1, 16, 255, 256, 257, 4096, and 1 MiB retain `KIND_BYTES`/`RC_FRESH`, requested length and independent mutable storage; and Linux errno mapping retains the named error codes. Owned guards release every result on both success and assertion unwinding. One result per length uses raw `cinch_free`, the other uses normal `cinch_drop`, with an exact two-header release delta. Each public test invokes `cinch_exit_check(0)`.

Run the new public module in its own process first with `CINCH_ALLOC_STATS=1 CINCH_LEAK_CHECK=1`, then in another process with `CINCH_ALLOC_STATS=1 CINCH_LEAK_CHECK=1 CINCH_RC_DEBUG=1`. Require every test to pass and every emitted leak line to report zero leaked headers. Poisoning retains freed storage by design and is not a live-byte memory benchmark. Header/free checks and the existing ABI payload-release test together check the owned-buffer path; no allocator speed claim follows.

Then run the integrated `entropy::tests::` loop suite, `sys::tests::`, `bytes::tests::`, `abi::tests::`, `platform::tests::` with the archive-export test explicitly skipped, and `platform::pipe_tests::`. Existing unrelated tests do not all release their runtime headers, so those broader selections run without global leak assertions; the new public module's isolated leak checks remain the ownership evidence. Record actual selected counts after `--list`, rather than assuming counts from source inspection.

`platform_exports_are_global_text_symbols` requires an adjacent runtime archive and is deferred. Do not copy the old D91 archive beside the candidate test executable to make it pass: that would inspect the wrong artifact. A future candidate staticlib/symbol gate must use a separately named, newly built archive with its exact hash and source identity. Candidate/base archives are approximately 24.7 MiB each before any link scratch, so matched full-pipeline benchmarking requires separate storage planning and an exclusive timing grant.

The structured-error test maps genuine `std::io::Error` values through the same mapping used at the boundary; it does not claim the current kernel naturally returned those errors. Natural/injected ENOSYS/EPERM provider branches remain separately labeled and pending. Public `Effect.Random` and UUID consumers also remain later gates requiring the separately qualified candidate runtime archive.
