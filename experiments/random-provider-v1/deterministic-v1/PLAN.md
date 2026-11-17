# Standalone deterministic gate

Prepared while the parent's exclusive timing hold is active. Do not invoke a compiler until that hold is released. CPU 4 has been offered by the Path lane; the parent's scheduling decision controls the actual start.

The wrapper imports the exact copied proposed entropy module by path. Its only syscall boundary is a documented panic-only stub. The six existing unit tests exercise the loop with explicit deterministic closures; no real provider, full runtime, public ABI, allocation ownership, or performance qualification follows from this gate.

Use `/workspace/.tools/rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc` after capturing its exact executable hash and `--version --verbose` output. Compile the wrapper with `--edition=2021 --test -C debuginfo=0 -C strip=symbols -C codegen-units=1 -C opt-level=0`; pin both compiler and tests to CPU 4 and run the resulting executable with `--test-threads=1 --nocapture`.

Check at least 350 MiB is free immediately before compilation, enforce the 250 MiB shared reserve while each child runs, and cap new outputs at 30 MiB total. Expected new executable size is below 10 MiB, but this has not been measured. Capture exact commands, source hashes, compiler identity, stdout, stderr, exit statuses, elapsed wall time, and disk usage. Preserve failures as their own epoch; do not overwrite the source or reinterpret failed tests.

## Next bounded real-syscall proposal

After a separate grant, use another standalone wrapper that includes the exact copied proposed `sys/linux.rs` and `entropy.rs` rather than a new runtime archive. The Linux module's only parent dependency is `EINVAL`, which can be supplied unchanged as 22; a small `sys` forwarding module calls that actual backend. This invokes the existing inline ASM backend and proposed exact-fill provider directly with actual OS entropy. It does not qualify `cinch_random_bytes` or owned boxed `Bytes`.

Check exact fill at 0, 1, 16, 255, 256, 257, 4096, and 1 MiB, with distinct allocated slices and repeated draws. Preserve all returned bytes or compact cryptographic digests as clearly identified real entropy capture; treat freshness comparisons only as smoke checks. Run one small nonempty draw under a syscall tracer if available, with its output retained separately, and confirm one `getrandom` call without entropy-device opens. Trace an empty fill separately to confirm the retained device-open behavior. Avoid timing in this qualification unit. The wrapper can remain well below the 30 MiB output budget, subject to the same measured resource guard.

Injected ENOSYS and denial tests stay in the deterministic gate unless an independently reviewed process-isolation/syscall-filter harness is available. Do not present injected results as actual kernel fallback execution. Public `Effect.Random`, UUID, ownership/leak, candidate-runtime symbol, portable object, and timing gates still require a separately named runtime and the parent's explicit resource/CPU lease.
