# Checked imported scalar lowering

A Native client previously called separately compiled `Data.Machine` wrappers for most scalar operations. A recurrence using comparison, subtraction, multiplication, addition, shift and XOR therefore made six imported calls per step. The candidate lowers checked canonical operations to typed LLVM instructions through the existing operand and result ABI bridges.

The focused candidate is experimental. Its full compiler-family fixed point, changed-source probe, source-absent compiler-entry consumer and parent preserved compiler tests remain pending. No seed/provider pin changes are included.

## Authority and ABI

The closed subset covers signed and unsigned 8/16/32/64-bit wrapping arithmetic, bit operations, comparisons, masked shifts and wrapping conversions. A canonical opcode, exact checked nominal scheme, `ccall` ABI and empty effects are required. Display names and serialized LLVM supply no operation authority. Shifts mask the canonical U32 count before an LLVM shift; narrow results use the existing sign/zero-extension bridge and comparisons use the existing Boolean bridge.

The existing primitive layout validator is shared without changing foreign-call admission or inline-LLVM containment. Total-body imports retain their existing authority. Legacy display-only imports retain ordinary object calls; they do not acquire an intrinsic implementation. Unsupported carriers and operations keep their prior paths.

Code-free scalar facades must match every field of an already represented defining value contract except the local alias name. Their defining foreign row must be a supported pure scalar intrinsic. Represented module hashes remain exact. New types, instances, signatures, foreign declarations and serialized units cannot enter through this facade path.

Native interface production also retains source scalar declarations using the checked binding's canonical scheme and opcode. A genuine binary/unary/Boolean producer is consumed after deleting its source. That pure-word client links only the matched runtime; it needs no provider object or source implementation. This no-object CLI path uses ordinary RC emission, while the other positive authority clients and complete arithmetic corpus explicitly compile checked RC.

## Focused evidence

The exact aggregate receipt is `native-scalar-intrinsics-validation-results.json`; `native-scalar-intrinsics-evidence/` retains raw commands, hashes, frontend verdicts, ownership results, codegen and timing observations.

- Candidate: compiler `167a24969606655e3450cd28470a01da67f9138ac6175e89892002a9e02d47cb`, compiler source `937efd9faa46c5015ebbe564db80880fda4cdea0995360e73759f3eeed06355e` across 51 files. Its host was linked at O0 by the frozen Rust producer `ae9c3f81…`; consumer optimization is recorded independently.
- Baseline: certified self-built Native compiler `58676dec…`, source `4de3bd70…`. Both baseline and candidate execute all 667 independent finite-word observations at consumer O0/O2 with normal 32 MiB and poisoned 256 MiB heaps, 8 MiB stacks, and zero leaks.
- Both consumer levels pass 27 authority observations: source overrides, ordinary source/foreign lookalikes, exact schemes/effects/ABI rejection, preserved refusal categories, source-absent aliases and defining descriptors, legacy fallback, independently observed operand effects and forged-interface refusals.
- The recurrence has six imported call sites in the baseline loop IR and none in the candidate. Its optimized baseline assembly has seven call instructions across the function, including separate initial and loop-bottom comparison sites; steady-state iterations use six helpers. Candidate recurrence assembly has no calls. LLVM and machine-code inspection place the complete recurrence between the two clocks and checksum observation after the final clock.
- Nine alternating pairs of five million steps on CPU0 measured median 38.380710 ms baseline and 8.811263 ms candidate, a 4.355869× speedup for this recurrence. Both consumers are O2 and share exact source, foreign fixture, provider and runtime. Every sample has the independently computed checksum and zero leaks. Parent Stage3 activity on CPU3 is recorded; these are not whole-library or compiler-throughput measurements.
- Nine canonical documentation artifacts have no diagnostics or orphan comments. The 667-vector fixture regenerates byte-identically.

The explicit provider receipt is `e70250a5…`, object `645a3d0e…`, runtime `80bd5b54…`, ABI10, x86_64 Linux. Its 65 recorded source modules include phase-one `Data.String` source `54ae8a33…`. This is a separate epoch from the newer combined String provider. Corpus and benchmark clients contain no library source override; the named override control deliberately checks that real source behavior wins over stale imported metadata.

## Reproduction

Use the exact frozen compiler/provider paths in the receipt and source `/workspace/.tools/cinch-env.sh`. Set `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RUST_MIN_STACK=67108864 CARGO_INCREMENTAL=0`. No Cargo build is needed for the focused consumers.

```sh
python3 tooling/generate_scalar_intrinsic_vectors.py
python3 tooling/validate_scalar_intrinsics.py --compiler COMPILER --library-artifact PROVIDER --artifacts NEW_DIRECTORY --output REPORT.json --opt-level 0
python3 tooling/validate_scalar_intrinsic_authority.py --compiler COMPILER --library-artifact PROVIDER --artifacts NEW_DIRECTORY --output REPORT.json --opt-level 0
python3 tooling/validate_scalar_intrinsic_codegen.py --baseline BASELINE --compiler COMPILER --library-artifact PROVIDER --artifacts NEW_DIRECTORY --output REPORT.json
```

Repeat the first two validation commands at `--opt-level 2`. Artifact directories must be fresh. Timing is a separate bounded, explicitly pinned run after preflights:

```sh
python3 tooling/benchmark_scalar_intrinsics.py --codegen-report VERIFIED_REPORT.json --output TIMING.json --cpu CPU --iterations 5000000 --samples 9 --background-context CONTEXT
```

## Separate nested foreign-function limitation

The unchanged nested/sequenced fixtures and raw baseline/experimental receipts are retained. The nested form fails with `foreign call must supply every checked ABI argument`; the sequenced form passes O0/O2 and both ownership modes under both observed compilers.

The dense Core has the correct U8 argument type. Nested dependent argument elaboration first creates a bare `#argument-function` alias of the inline primitive. Materializing that foreign function value enters the existing foreign-call path with zero supplied arguments, which it rejects. This patch does not add a uniform-word foreign-function-value bridge or alter that limitation.
