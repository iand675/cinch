# Stage 3 compiler suite handoff

The goal is to pass every compiler integration target against one self-built Stage 3, including ignored tests. Keep the existing assertions, public-interface round trips, reference-counting and leak checks. Instance and proof dictionaries must retain their actual wrapper type and select the declared factory across module boundaries.

## State on 2026-11-20

This section is newer than the ones below it. The failing-target list of
2026-11-19 is unchanged: nothing here fixes a test, and the work is on how
fast the compiler can be changed and on what it emits.

### What changed in the compiler

| Change | Why |
| --- | --- |
| Completing an imported constructor table registers the carriers its constructors name | a family reached only through an imported constructor's field had no runtime representation |
| A library root with more than thirteen parameters is callable and not a value | every root got a closure, and a closure describes at most thirteen parameter classes |
| `FiniteLawResult`, `LawFunction`, the rest of `StageValue` and `WithLayout` are exported | a partly exported family is abstract to an importing module |
| Locals are numbered per unit; constants, static closures and dictionaries, lambdas and recursive workers are named by their unit | one counter renumbered every function after an edit, and a lifted name was unique only within a compile |
| `--emit-object` with an object path of `-` writes the IR, the interfaces and the module's own units and facts | a build that compiles the IR itself needs neither an unoptimised object nor the merged manifest |
| A saturated call to an imported value or closed instance method whose total body is a tree of primitives is emitted as that tree | each was an opaque call into the library object |

`design/compilation-speed.md` sections 2.23, 2.26 and 10.7 have the
measurements. Two things on the earlier list were looked at and left, with
the reason: using `parMap` inside the compiler waits on the `share`
attribute `design/parallel-evaluation.md` plans for `String`, which needs
interface schema 11 and a library rebuild; and the totality retry's
repeated import prework is now 8% of a module's compile, so it matters
only to the whole-program chain.

### The compiler as a graph of modules

`tooling/module_build.py` compiles each module alone against its imports'
interfaces and links the result. From a development Stage 2 on four cores:
166 s from nothing, 0.7 s when nothing changed, about 30 s after a body
edit in `Bootstrap.Infer`, about 6 s for a small module, 95 s after a new
export in a module sixteen others import. The compiler it links emits the
development Stage 2 module byte for byte, an incremental build equals a
clean one file for file, and the result can build itself.
`run_stage_targets.py --compiler` runs targets against it.

### The chain

Stage 1 is no longer linked with a profile unless `--pgo-seed` asks. The
chain at the branch head reaches the fixed point either way with the same
Stage 3 module; without the seed profile it used 25 CPU minutes against 34.
On the audited compilers, the handoff tip against the head: a self-compile
takes 94 CPU seconds against 493, and the fib smoke against the base
interfaces 1.10 against 2.40.

## State on 2026-11-19, 23:40 UTC

This section is newer than the ones below it. Work continues on
`claude/affectionate-goldberg-yxn29v`, which is `lane/stage3-repr` at
`697c6f93` plus build tooling and six frontend speed changes. No test,
assertion or language rule changed.

### The whole matrix has run once, on one compiler

`tooling/run_stage_targets.py --stage 3 --all-integration`, ignored tests
included, against the Stage 3 built from `1dd765f8`:

| | |
| --- | ---: |
| targets | 345 |
| targets passing | 271 |
| tests passing | 2130 |
| tests failing | 335 |
| selected-compiler invocations | 3054 |

`bootstrap/current-stage3-matrix-results.json` lists every failing target
with its failing tests. The run took 3 h 31 min on four cores it shared with
other builds.

`1dd765f8` differs from the integration tip only by speed changes, so each
of the 74 failing targets was also run against the Stage 3 built from the
tip itself (`697c6f93`). All 74 fail there with the same tests: nothing in
the list is a regression from this branch, and nothing in it was fixed by
it. The seven targets the earlier handoff named:

| Target | Passing | Failing tests |
| --- | ---: | --- |
| `native_dictionary_checks` | 3/3 | |
| `ownership` | 31/31 | |
| `lattice` | 15/26 | `generic_preserves_transfer_works_where_the_method_preserves`, `incomparable_heads_at_a_common_refinement_are_a_coherence_error`, `narrow_back_up_is_accepted_with_a_dec`, `narrow_back_up_is_refused_without_a_dec`, `narrow_rejects_forged_dec_evidence`, `postcondition_rows_round_trip_through_the_interface`, `signature_obligations_constrain_callers_identically`, `specialized_sorted_merge_wins_by_most_specific_resolution`, `transfer_accepts_non_empty_with_preservation_evidence`, `transfer_rejects_sized_ascii_naming_the_conjunct`, `using_obligations_discharge_through_the_dec_evidence_path` |
| `dependent` | 20/22 | `acceptance_probe_01_bounded_index_runs`, `pi_sigma_construction_and_elimination` |
| `syntax_objects` | 74/78 | the four `native_captured_index_*` cases |
| `ffi` | 9/15 | `foreign_signature_contradicting_repr_facts_is_rejected`, `opaque_values_are_unprojectable_and_foreign_only`, `out_cells_are_one_word`, `owned_borrowed_roles_and_the_deallocator_rule`, `packed_boundary_fields_marshal_without_padding_reads`, `repr_c_field_access_uses_the_recorded_offsets` |
| `stage_code_expectations` | 4/11 | `generic_declaration_producers_cannot_freshen_an_int_caller_to_bool`, `generic_declaration_producers_keep_the_bool_callers_type`, `one_monomorphic_code_value_cannot_choose_two_parameter_types`, `partial_application_retains_the_generic_callers_type`, `returned_stage_closures_cannot_capture_affine_arguments`, `returned_stage_closures_retain_the_generic_callers_type`, `separate_stage_instantiations_can_choose_different_types` |

The next-work column of the table further down still describes these. The
two lanes it names (`lane/stage3-scoped-narrow`,
`lane/stage3-compiler-fixture-interfaces`) were not touched. Many of the
other 67 failing targets were never reached by the earlier run; a number of
the `bootstrap_*` ones fail on what look like stale expectations (a module
count, a diagnostic's wording) and have not been triaged.

### Three compilers, three fixed points

Each was bootstrapped from the seed on a fresh 4-core x86_64 Linux machine
with LLVM 20.1.8. Stages 2 and 3 are identical in executable, IR and
interfaces, and the interface consumer and changed-source checks pass. A
hash is specific to a host and a build path, so none is comparable with
the ones recorded below.

| Source | What it adds | Chain |
| --- | --- | ---: |
| `697c6f93` | the integration tip | 41 m 11 s |
| `1dd765f8` | the data reference graph, the balanced binding index, one decode per ownership row, the JSON offset scanner | 22 m 24 s |
| `15965558` | the foreign-table owner test, the carved object read | 27 m 52 s on two jobs, sharing the machine |

All 347 Rust library tests and the tooling tests pass at the branch head.
`15965558` was not given the whole matrix. It was run on 39 targets (the
seven above and every target that reads the library object or an
interface), and each matches the tip test for test.

`design/compilation-speed.md` sections 2.22 to 2.25 have the measurements,
what the profile says is left, the naming change an incremental backend
needs, and a probe of a per-module build of the compiler.

### A fresh machine

The seed and library-producer pins are not in a shallow clone, the emitted
IR needs clang 19 or newer, and the grammar binding runs the `tree-sitter`
CLI at build time. On a machine with Nix:

```sh
git fetch origin "$(jq -r .revision bootstrap/stage0.lock.json)" \
                 "$(jq -r .revision bootstrap/library-provider.lock.json)"
nix build nixpkgs#llvmPackages_20.clang --out-link build/clang
nix build nixpkgs#llvmPackages_20.llvm  --out-link build/llvm   # llvm-profdata
npm install --prefix build/ts tree-sitter-cli@0.25
export CINCH_CLANG=$PWD/build/clang/bin/clang
export PATH=$PWD/build/llvm/bin:$PWD/build/ts/node_modules/.bin:$PATH
export CINCH_CARGO_JOBS=4 CINCH_BUILD_SLOTS=3    # a machine nobody else is using
```

With a binary cache configured (`tooling/nix/README.md`), `nix build
.#cinch-pins` and `tooling/bootstrap.py hydrate --from` replace the two
release cargo builds and the library object, and `nix build .#cinch-ladder`
is the checked chain for a commit. No cache exists yet: the workflow that
fills one needs the `CACHIX_CACHE` variable and the `CACHIX_AUTH_TOKEN`
secret.

Never edit `tooling/` under a running chain: the driver imports its helper
modules lazily, so a chain started from one revision can load the next
one's code. Run a chain or a suite from a worktree at a commit, with
`CINCH_BOOTSTRAP_BUILD` naming the build tree.

To iterate on the compiler's sources (`tooling/README.md`, Development
builds):

```sh
# The compiler as a graph of modules: 3 to 30 s after a body edit, by the
# size of the module edited. The builder is any stage or the last result.
python3 tooling/module_build.py --builder build/bootstrap/stage3/cinch --out build/modules -j 4
python3 tooling/run_stage_targets.py --compiler build/modules/cinch --allow-dirty \
    --out build/dev-evidence --cargo-target build/test-cargo lattice

# A whole stage through the seed: about 20 s to a type error.
python3 tooling/bootstrap.py build --stage 1 --dev -j 4
```

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
