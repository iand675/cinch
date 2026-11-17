# Fresh runtime archive pair and public consumer plan

Prepared only. No archive, object diagnostic, new consumer build, or timing has started. The completed runtime tests and their source identities are published at `d96804fa` on `lane/stdlib-entropy`; the standalone and full-runtime gates remain separate evidence.

## Sources and identical build controls

Materialize a plain baseline runtime tree from published commit `37355b7b` under this experiment, with all files hashed before compilation. Build the candidate from a plain snapshot of this lane's runtime sources at `843ddb8d`, checked against `runtime-boundary-v1/prepared-source-manifest.json`. Do not modify either source epoch after sealing it. The baseline platform hash is F4F0; its exact original source is also preserved in the initial experiment. Enumerate every file difference before starting:

| Candidate path | Difference from baseline |
| --- | --- |
| `src/lib.rs` | Private hosted entropy module and test-only boundary module declarations |
| `src/platform.rs` | Provider documentation and delegation to exact-fill helper; public ABI unchanged |
| `src/sys/mod.rs` | Linux-only internal `getrandom` forwarding seam |
| `src/sys/linux.rs` | Three pinned syscall numbers and raw syscall wrapper |
| `src/entropy.rs` | New exact-fill helper, original device fallback, deterministic loop tests |
| `src/entropy_boundary_tests.rs` | New test-only public boundary and ownership checks |

The boundary test module and loop tests are excluded when compiling the staticlib without `--test`. No other runtime source is allowed to differ. The experiment-side `runtime-boundary-v1/entropy_boundary_tests.rs`, `boundary-tests.patch`, and `reproduction.json` retain the exact test module and its source linkage for evidence imports that do not import the candidate runtime source. Replaying the original proposal patch and then the boundary patch reconstructs the tested runtime tree, with a manifest check before execution.

Use exactly the same rustc executable/hash, target, crate name, options, working-directory policy and environment for both archives. The proposed common command is direct rustc with `--edition=2021 --crate-name cinchruntime --crate-type staticlib --target x86_64-unknown-linux-gnu -C opt-level=3 -C debuginfo=0 -C strip=debuginfo -C codegen-units=1 -C lto=off`. Keep the default panic strategy identical. Do not use `strip=symbols` on an archive whose global C symbols must remain linkable. Set the explicit target CPU identically if one is selected; never use `target-cpu=native` on only one side. Record the expanded exact argv and environment in each receipt.

Each invocation receives its own new output/scratch directory and explicit `TMPDIR`, runs alone on the parent's granted CPU, and writes a separately named plain `libcinchruntime.a`. The two build directories and source paths must be recorded, including any path-dependent byte differences. The frozen D91 archive remains an immutable historical/control input. It is not one of this fresh matched pair and must not be renamed, overwritten, or described as identically rebuilt.

## Storage and compiler gates

Require at least **417 MiB available before beginning the pair**, then recheck at least 350 MiB immediately before each individual compile. Retain the 250 MiB shared reserve throughout and enforce at most 30 MiB new output/scratch per unit. Current observed space is about 365 MiB, so the archive pair cannot start yet. Source copies, receipts, raw logs and future consumer outputs also consume headroom; the 417 MiB value is a minimum, not a guarantee that other lanes can write concurrently.

The frozen archive occupies 24,088 KiB on disk (about 24.7 MB decimal); a new archive's exact size is unmeasured. Planning for approximately 25 MiB final archive plus 2–5 MiB object/link scratch approaches the 30 MiB cap. The previous full test build observed a 10.0 MiB output/scratch peak but did not produce a staticlib, so that peak does not establish the archive build peak. If static inspection cannot bound a first archive build safely, request a separate `--emit=obj` diagnostic with the same production optimization/codegen flags, its own output guard and recorded object size. This is an extra diagnostic gate, not an archive or a substitute for the pair. Do not launch or automatically retry any stage without the parent's grant.

Monitor aggregate outputs including temporary files, compiler stderr and child-process outputs; terminate the process group on reserve, size or agreed wall-time breach, retain all failed artifacts, and report the original failure. A second compile starts only after rechecking its own guard. Retain both completed archives plain through all symbol, consumer and future timing gates. Do not replace either archive midway through a measurement epoch. Any reproducible scratch cleanup must target only new closed outputs and be recorded; no frozen/provider/benchmark/failure input is a cleanup candidate.

## Archive and public ABI qualification

Before public consumers, record archive hashes, sizes, Rust compiler identity, every runtime source hash, defined global `cinch_*` symbol sets and unresolved foreign dependencies. Inspect `cinch_random_bytes` as an actual defined text symbol on both sides, verify the same exported C surface, and confirm no new libc `getrandom` symbol dependency. Retain the relevant object/disassembly showing the candidate uses the existing syscall assembly backend and the baseline retains its device path. Do not claim a portable target from a host archive.

Add a small separately reviewed C ABI consumer linked independently against each actual archive: call signed lengths -1 and 0, read positive result lengths/data through the existing public byte ABI, release every returned buffer, and verify exact negative error codes. Run it normally and with RC poisoning/leak checking. Do not infer the ABI from an archive symbol alone. The existing archive-export unit can then inspect the candidate archive beside the already-qualified test executable only in an explicitly identified candidate-artifact directory; preserve the original no-archive test epoch unchanged. The old D91 archive must not satisfy this candidate gate.

## Real Cinch consumers

Keep one frozen C2 compiler (`c2ae24e64f7999060dce8610618f316710f6300ca29e89b9335056936344a86e`), explicit checked interfaces/library objects, exact consumer source set, optimization flags, and linker version/options fixed. The repository already has `CINCH_TEST_RUNTIME_ARCHIVE` selection in the maintained consumer harness; when using it, also pin its required compiler/interfaces/library-object/source-root settings and capture every actual invocation. Confirm the selected frontend and configured runtime path before execution; refuse any silent fallback or automatic runtime build. A fresh compiler-side Rust harness build would be another separately budgeted unit, not part of either archive unit.

An alternative within the same fixed compiler pipeline is to emit each source's LLVM/object artifacts once, then link that exact object and exact checked library object separately with the fresh baseline and candidate archives. The maintained source harness already spells this direct-link route with the selected clang, explicit archive and `-ldl -lpthread -lm` on Linux. Preserve both link commands/reports and resulting executable hashes. This keeps the runtime archive as the intended changing input. Do not use a different parser/compiler or regenerate a sealed fixture to obtain passing results.

Run the existing `Effect.Random` consumer scenarios covering real draw lengths/freshness, bounded rejection sampling and half-open ranges, full-word draws, invalid/empty bounds and ranges, and hexadecimal output. Run the maintained UUID surface with one explicitly recorded Data.Uuid source (the qualified packed SHA `41c8c781da13ff21642870a9884619f0dcc62638d8be9c6a7d8f4ad9985408fb` is appropriate if selected identically for both runtime variants), unchanged Effect.Uuid, actual cryptographic generation, version/variant and clock/generator properties. Do not vary the parser candidate while attributing runtime changes. Both variants must pass normal and poison/leak execution with real OS entropy. Random values need not match between runs; structural properties and error vocabulary must match. Seeded/injected generators remain separately labeled loop/model tests.

Syscall tracing stays untimed and separately scoped. The successful helper traces already show the normal host path; a future supported injected ENOSYS/EPERM run must be labeled as injected rather than a naturally observed kernel or sandbox condition. Such a run does not replace actual secure consumer tests. Cross-target object/execution checks and their limits remain separate gates.

After archive symbols, C ABI, and actual public consumers pass, prepare a matched benchmark protocol with both fresh runtime identities and the same consumer objects. Do not start timing without the parent's exclusive lease and pre-start CPU quota/load telemetry. Keep every earlier D91/SDK observation under its original identity; no current speedup claim is justified by the syscall traces alone.
