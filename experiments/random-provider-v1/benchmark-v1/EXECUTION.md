# Prepared full measurement window

No measurement window has started. The launcher requires an explicit parent-issued exclusive lease, the exact sealed plan hash, and a new report directory. It runs only the three existing qualified executables; no compiler, SDK builder, cache writer, source regeneration or executable copy is involved.

The selected plan is the full protocol: 13 conditions, 4096 iterations, five warmups and 30 timed samples for each baseline/candidate/SDK variant. That is 195 warmup observations and 1170 timed observations, 1365 total. For every condition, the timed schedule includes all six variant-order permutations exactly five times; every variant occupies every position exactly ten times. Condition starts also rotate. Static assertions verify all these counts and the 455 complete triples. The earlier unexecuted draft had globally balanced orders but insufficient per-condition balance; its source, plan and review are preserved under `execution-draft-v1` rather than treated as a valid schedule.

The estimate uses only process bookkeeping from the target-count structural preflight. The 39 normal processes covering all conditions/variants took 1.058630513 seconds in total. Extrapolating 35 rounds and adding a two-second metadata/I/O allowance gives **39.052067958 seconds**. Treating all 1365 processes as slow as the single slowest preflight process gives a deliberately pessimistic stress bound of 91.317808564 seconds. Neither number is a performance result or a guarantee. Root selected a dedicated **45-second exclusive cap**. The sampling phase stops at 43 seconds to reserve two seconds for post-run verification and telemetry; a timeout or incomplete sequence remains plainly recorded with no automatic retry.

Resource limits use the existing-binary exception: 300 MiB free initially, 250 MiB reserve, at most 5 MiB of new reports, and CPU 0 exclusively. The ordinary new-build start rule stays 350 MiB. All 33 sealed provenance inputs (about 130 MB, read only) are hashed before and after: all three executables, the exact source/module/provider inputs, frozen compiler/SDK identities, common object, LLVM/assembly, fresh runtime archives and unchanged D91 control. Optional SDK crypto/GC environment settings must still match the preflight context rather than being silently overridden. Allocation/leak/poison instrumentation is disabled for measurements; those behaviors were checked separately in the structural gates.

Immediately before the first warmup and after the last process, the launcher captures affinity, UTC/monotonic boundary timestamps, cgroup quota and throttling counters, load and runnable cgroup process metadata without command-line arguments. Root's peer hold acknowledgments remain the lease authority. Each sample records its exact order, phase, arguments, exit status, raw stdout/stderr, structural check and separate random/retained observations. Unsigned observation checksums are recorded as decimal strings. Every control, regression, warmup, failure and incomplete triple remains in the raw JSONL file. Added throttling is explicitly flagged rather than erased from the record.

The SDK's actual secure kernel vDSO path differs from the Cinch candidate's raw syscall per nonempty draw. Empty-input failure boundaries, allocation, GC/RC and final-owner release also remain explicit comparison limits. Do not normalize away these differences or claim SDK parity from a syscall reduction. The launcher computes no favorable-only aggregate and cannot silently retry a noisy run.

The approved invocation shape, only after START, is:

```text
python3 -B timing_window.py --lease-id ROOT_SUPPLIED_LEASE
  --plan-sha256 SEALED_EXECUTION_PLAN_SHA256 --output NEW_LOCAL_REPORT_DIRECTORY
```

`execution-preparation-review.json` contains the concrete launcher, input-manifest and execution-plan hashes. The final plan requires no change to any qualified source, executable, runtime archive, condition or iteration count. A missing or changed input, configuration mismatch, structural failure, reserve/output guard or deadline produces a diagnostic/incomplete outcome with its raw data retained.
