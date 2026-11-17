# Fresh archive and public consumer qualification

Both runtime archives were built freshly from sealed source snapshots using the same rustc executable, target, optimization and codegen flags. The full source-pair manifest enumerates exactly six differing files. The candidate sources match the earlier tested runtime source epoch; the old D91 archive remains an unchanged historical control, not a member of this pair.

| Variant | Archive SHA256 | Bytes |
| --- | --- | ---: |
| Baseline, source `37355b7b` | `e75d162d7ed4fe8c34d48b6a5a2683fcc9c339336437213e176f7e60d5b5fa6a` | 24,569,958 |
| Candidate, source `843ddb8d` | `663d9a953a404b3fb578e66b9bb5d316b0b5b36e87ae92c1a5ba5dece0fa76b3` | 24,571,854 |

The build receipts retain exact commands, source/ compiler identities, environment, statuses and monitored resource usage. Observed output-plus-scratch peaks were 24,573,977 and 24,575,887 bytes respectively, below the 30 MiB unit cap. Both compiles started above 350 MiB free, and the pair started above 417 MiB. Minimum observed free space during the compiles was 723,152,896 bytes. No build guard was breached. The recorded 7.707/7.659-second process durations are bookkeeping, not benchmark observations.

The archives define the same 216 global `cinch_*` symbols. The candidate adds one per-object Rust I/O error reference, which resolves to a definition within its own archive; the final unresolved symbol set adds nothing. There is no new foreign `getrandom` dependency. Actual archive disassembly with relocation records shows baseline device open/read/close calls and the candidate's inline Linux syscall number `0x13e`, zero flags, partial-fill loop and retained device fallback. The original archive-export unit was not rerun; this archive gate directly records the actual full exported `cinch_*` surface instead.

The reviewed C ABI fixture was compiled once to object SHA256 `410b6fe3ce9db44cc7b811c40e9136c60dc8168e82348dea3d84adfb5a4d7f9a` and linked separately to each archive. All four executions (baseline/candidate, normal/poison) passed. Each checked two negative lengths plus two actual secure draws at each of eight nonnegative lengths, independent mutable backing storage, exact byte API behavior, and both raw/RC release paths. Each process allocated and released 16 runtime headers and emitted a zero-leak report.

Six unchanged public Cinch fixtures then passed all **24 executions** (each fixture with both runtimes in normal and poison modes), all with zero leaked headers:

- Random byte lengths and freshness.
- Bounded draws, range behavior and distinct values.
- Named bound and range errors.
- Lowercase hexadecimal rendering.
- UUID ownership and clock/range control behavior.
- UUID actual entropy, wall-clock and shared generator state.

Each fixture was emitted once by frozen C2 (`c2ae24e64f7999060dce8610618f316710f6300ca29e89b9335056936344a86e`), compiled once to a common LLVM-derived object with clang O2, then linked separately to both fresh archives. The UUID fixtures explicitly used the same packed Data.Uuid source SHA `41c8c781da13ff21642870a9884619f0dcc62638d8be9c6a7d8f4ad9985408fb`, unchanged Effect.Uuid and identical public source modules on both sides. The parser variant was not varied during this runtime comparison. Actual entropy remained the provider; random data equality was not expected or asserted. Deterministic model/control cases in the maintained fixtures remain model/control evidence, distinct from their real-provider observations.

The C ABI unit produced 11,737,837 bytes. Each public Cinch project produced approximately 13.9–14.6 MB including its common object, two executables, compiler artifacts, scratch and logs, below 30 MiB. Every compile/project began above 350 MiB free and every monitored process retained the 250 MiB reserve. Receipts capture the actual per-process minimum. CPU 4 was returned after these gates. All compiler durations in this stage are process bookkeeping; **no timing benchmark has run**.

Two setup/launcher failures remain plainly preserved. The first source-extraction prefix guard rejected known parent directory entries before extracting files or invoking a compiler; `source-snapshot-attempt-1.json` records it and the corrected extraction uses a separate `sources-v2` epoch. The first disassembly invocation exited 127 because `libLLVM.so.20.1` was not on the loader search path; its complete diagnostics remain in `archive-inspection-v1`. A separate `archive-inspection-v2` supplies the explicit LLVM library path. Neither archive was rebuilt to obtain passing inspection results.

`qualification-results.json` links the archive and consumer identities. Per-fixture artifact manifests retain hashes of all derived artifacts, including success executables/objects/interfaces kept plain locally. Source snapshots, exact fixture sources, LLVM, disassembly, complete stdout/stderr and commands/receipts are preserved. Root's F4F0 platform source, frozen D91 runtime and frozen C2 compiler were rehashed unchanged after the full gate.

Remaining work includes explicitly labeled injected-provider error branches, portability execution, and a matched actual-entropy public-pipeline benchmark. Neither the syscall count nor passing functional gates establishes a speedup. The candidate remains isolated pending review and later integration decisions.
