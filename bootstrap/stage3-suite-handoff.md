# Stage 3 compiler suite handoff

The goal is to pass every compiler integration target against one self-built Stage 3, including ignored tests. Keep the existing assertions, public-interface round trips, reference-counting and leak checks. Instance and proof dictionaries must retain their actual wrapper type and select the declared factory across module boundaries.

## Starting point

Fetch `origin/main` and `origin/lane/stage3-repr`. The latter is the integration branch to continue from. Use a separate worktree and Cargo target directory for each test checkout. Several test binaries compile their source paths into the harness and reject a different worktree. Old test reports can prioritize a run but cannot establish a pass on a newer compiler.

Snapshot recorded 2026-11-19 15:54 UTC.

- Published main [`312127239b1b`](https://github.com/iand675/cinch/commit/312127239b1b1eebe0b03207dae492223787fc11).
- Integration code before this handoff [`4e9d0a6b1070`](https://github.com/iand675/cinch/commit/4e9d0a6b1070a50c4d5d74e8f1373bf20e3c0d3e).
- Main advanced through the executable-only JSON fixtures and package-store repair. The JSON suite passed 9/9 with 11 actual calls on the verified main compiler. For the package-store change, all 347 Rust library, 10 store and 12 link tests passed. The tested package-store tree and the landing differ only in the independently verified JSON test file.
- The integration branch includes the fixes for scoped Copy and linear ownership, checked measures and structural refinements, explicit `forget`, strict literal inference, interface serialization and the measured value-kind fixture change. The whole integration tip still needs its own bootstrap and full matrix.
- `lane/stage3-scoped-narrow` at `e02c4cf5` is an unfinished compiler change. Its development compiler passed 15/25 of the lattice tests on its branch, including missing-evidence and forged-evidence diagnostics. A valid `Yes w -> narrow b to SizedAscii with w` still fails independent Core validation with “a proof clause does not discharge its goal”. Start with `narrow_back_up_is_accepted_with_a_dec` in `compiler/tests/lattice.rs`. Compare `actual-index`, `scope-let`, `scoped-refinement-proof`, and the opening of the decision witness for the same lexical `b`. The preceding commit `5543ec89` tried direct `CRefineProof` lowering and reproduced the same Core failure. Keep the compiler’s independent proof check.
- `lane/stage3-compiler-fixture-interfaces` at `2e79f56f` applies the executable-only test helper to nine more bootstrap fixtures. All nine Rust targets type-checked. The complete native fixture run writes individual results under `/tmp/cinch-compiler-fixture-outputs/`. On a fresh checkout run the ten targets (`bootstrap_{value_kinds,value_indices,decisions,effect_types,pi,proof_terms,handler_core,typing,core,array_float_bits}`) with `tooling/run_stage_targets.py` before landing the remaining nine.
- On the `ab963a28` Stage 3 compiler, `dependent` improved to 20/22. Only `acceptance_probe_01_bounded_index_runs` and `pi_sigma_construction_and_elimination` failed. All 38 selected-compiler calls are recorded in the logical-totality matrix.
- `1d68b4a6` implements the executable `Ordered` decision for the canonical `List Nat` carrier, using arbitrary-precision Nat comparison and an iterative adjacent-pair scan. The added regression passed on the Rust reference compiler and a fresh development compiler. Independent native probes returned 42 in both RC profiles with zero leaks and rejected Int, I8, U64 and Bool lists. Signature and instance `Sorted` obligations still need elaboration and use-site evidence.
- `68a65241` preserves the Rust test name across the native dictionary suite’s large-stack worker threads. All three tests passed against the ad20b21e fixed point. All 27 selected-compiler invocation records now identify their originating test. The third test exercises host incremental sessions.

## Verified results and remaining failures

The published compiler at `e3a74c36` reached the Stage 2/3 fixed point, including identical LLVM IR, executables and interfaces, an independent changed-source compilation and an observable change in diagnostics. The native executable SHA-256 is `855c89275ce0ec19e0f6496e92ec7e368b1484f5ac65a8043525524ea8d143b6`. All 347 Rust library tests and 123 tooling tests passed. A separate producer/consumer probe checked public instance/proof wrappers and rejected a mismatched Bool dictionary.

The broad run on that compiler had completed **33 of 345 targets**. **24 passed** and **9 failed**. Those completed targets recorded 421 passing individual tests and 611 selected-compiler invocations. Earlier runs on different executables do not count toward this total.

| Area on the e3a74c36 compiler | Tests passing | Next work |
| --- | ---: | --- |
| Refinement lattice | 1/23 | Re-run on the integration branch. Newer development builds implement baked list/byte lengths, character-accurate ASCII decisions, weakening and explicit forgetting. Remaining class-resolution work includes `Preserves`, incomparable heads, `Sorted` obligations and authenticated postcondition rows. Retain lexical proof identity when projecting the subject. |
| FFI | 9/15 | Implement checked C and packed layouts, real field offsets, one-word `Out` validation and `Owned` allocator/deallocator identity. Compare against the C fixtures in `compiler/tests/ffi.rs`. |
| Dependent types | 18/21 | Logical totality passes on the later `ab963a28` fixed point. `Vec.getCopy` must use the scoped Copy dictionary and matching bound. Sigma needs binder, pair and elimination support. |
| Syntax objects | 72/78 | Re-test strict literal and caller-scope fixes. The remaining reproductions exercise captured index functions, shared polymorphic arguments and dependent binder scope. |
| Staged code expectations | 4/11 | The stage parser misses `let ... in quoteDecls where`. Preserve the caller’s Code type through partial applications, monomorphic bindings and returned closures. Enforce affine captures. |
| Surface syntax | 12/21 | Restore literal folding and typed source-data conversion with provenance. Preserve runtime construction of closed callbacks and the selected Enum/FromMap dictionaries. |
| Reclamation | 24/25 | `evaluated_quotations_with_heap_holes_are_freed` needs runtime Code/eval with owned, typed splice holes. Test with both RC profiles. |
| REPL and bootstrap argument scopes | 14/15 and 0/1 | Re-run the qualified filesystem result and the strict-literal shadowing case on the newer compiler. |

The later fixed point `ad20b21e` has identical native stage binaries with SHA-256 `4d18bdb2f3bb343cf7578589f01724a073968b4f3faa29c7a385b47adc08027d`. All three native dictionary tests passed and made 27 selected-compiler invocations. The bootstrap Core suite passed. Ownership passed 26/30 on this revision. The four failures involve borrowed pattern fields, vector-filter Copy evidence, erased curried arrows, and disjoint linear projections. Development revision `e2e1f655` passed all 31 ownership tests, including the new regression. Its own Stage 3 run is the verification that matters for landing.

## Measured compilation costs

- `971cfbc8` indexed object-symbol validation. On the same source, the median of two runs per compiler dropped from 48.56 to 42.45 CPU seconds. Peak RSS dropped from about 102,300 to 77,500 KiB. The emitted IR was byte-identical.
- `ed4cd376` reused checked interface hashes and indexed nested module bodies once per program. The five optimisation-interface tests, including the ignored full library census, and all 22 package tests passed. Nested interfaces matched byte-for-byte.
- `50384cf8` stopped generating unused interfaces for the executable-only value-kind test fixture. On the same verified Stage 3 and 1,249,488-byte source, compiling and linking took 140.25 versus 96.52 CPU seconds. IR and linked executables were byte-identical. This was one A/B pair under concurrent load. The end-to-end suite speedup still needs measurement.

## Reproduce and finish

In a clean checkout with the pinned Rust/Cinch seed and LLVM toolchain configured, run the following from the repository root. In the existing workspace, `. /workspace/.tools/cinch-env.sh` configures the tool paths. Reserve separate build, Cargo and evidence directories on a volume with room for the compiler and test fixtures.

```sh
repo=$(pwd)
export CINCH_BOOTSTRAP_BUILD="$repo/build/bootstrap"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864
export CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_DEBUG=0
cargo test --locked --manifest-path compiler/Cargo.toml --lib
python3 -m unittest discover -s tooling/tests
python3 tooling/bootstrap.py check -j 2
python3 tooling/run_stage_targets.py --stage 3 --out "$repo/build/targeted-evidence" \
  --cargo-target "$repo/build/test-cargo" --jobs 2 \
  native_dictionary_checks ownership lattice dependent syntax_objects ffi stage_code_expectations
python3 tooling/run_stage_targets.py --stage 3 --all-integration --jobs 3 \
  --out "$repo/build/full-stage3-evidence" --cargo-target "$repo/build/test-cargo"
```

Use fresh evidence directories for a fresh run. The four-CPU shared workspace benefits from a single broad sweep per compiler revision. Finish the affected targets on intermediate revisions and reserve the full matrix for the chosen release candidate.

The all-integration runner discovers the targets from Cargo, includes ignored cases and records individual frontend calls. Some Rust-only tooling targets legitimately make zero selected-compiler calls. Inspect `summary.json`, the target logs and `invocations-*/` together. For the audited portable gate, run `python3 tooling/verify_stage3.py --evidence "$repo/build/verified-evidence" --cargo-target "$repo/build/test-cargo"`. The Godot integration cases require Godot and godot-cpp. The existing environment uses `CINCH_GODOT=/usr/local/bin/godot` and `CINCH_GODOT_CPP=/dev/shm/cinch-stage3-godot-cpp`.

### Existing workspace

The main checkout is `/workspace/cinch` and the integration worktree is `/workspace/cinch-stage3-repr`. Qualification builds live under `/workspace/cinch-stage3/build/bootstrap-*`. The principal runs use the names `map-collections` (e3a74c36), `scoped-copy` (ad20b21e), `logical-totality` (ab963a28), `ownership-places` (e2e1f655), and `checked-lengths` (2442708a).

Look in `/tmp/cinch-{merged-dictionary-collections,scoped-copy,logical-totality,ownership-places,checked-lengths}-stage3-matrix/` for each run’s `plan.json`, per-target result JSON, logs, and invocation records. The corresponding `*-full-bootstrap.log` contains the fixed-point outcome. Check `/tmp/cinch-audited-*-stage3.log` and the remote branches first on resume. Qualification processes started in this workspace can advance main after passing their gates. These paths are local scratch space. The repository commands above recreate the evidence on a fresh machine.

- `ad20b21e` (scoped-copy) had changed source and fixed point verified and 3 completed integration targets.
- `ab963a28` (logical-totality) had changed source and fixed point verified and 3 completed integration targets.
- `e2e1f655` (ownership-places) had stage 2 built and 0 completed integration targets.
- `2442708a` (checked-lengths) had queued and 0 completed integration targets.
