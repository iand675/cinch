# Configurable CSV records and streams

`Text.Csv.Record` adds immutable configurable readers, original physical line and byte-column positions, structured errors with completed fields, and minimally quoted text/raw-byte rendering. `Effect.Csv` adapts `ByteReader` and `ByteWriter` without owning their lifetime. Legacy `Text.Csv` is unchanged.

Validated dialects support Unicode delimiters/comments, strict or lazy quotes, Unicode leading-space trimming, inferred/exact/unrestricted field counts, LF/CRLF, blank/comment skipping, and multiline quoted fields. Raw interfaces preserve embedded NUL and invalid UTF-8. Text conversion returns the valid field prefix and an encoding error; text streams reject invalid physical input with a sticky source error. Errors expose precise original positions. Reconfiguring an immutable continuation preserves its cursor and unread lookahead.

A source read requests at most 4,096 bytes. Each returned chunk is copied before retaining it, including sources that reuse mutable buffers. Reversed fragments collect an unfinished physical line and record; a completed record is assembled and parsed once. Live memory scales with the largest unfinished record plus bounded lookahead and fragment overhead. One-byte chunks therefore add per-fragment overhead. `read-all` intentionally retains every returned record. Raw field getters also copy mutable byte buffers, preserving immutable record values.

Writers render and synchronously drain one record, retry valid short writes, and retain sticky failures and the advanced sink. `csv-stream-flush` observes completion/error of these synchronous writes. The borrowed `ByteWriter` protocol exposes no underlying flush or close operation. These adapters do not buffer several rendered records or flush a borrowed descriptor.

## SIMD and ownership

Eight-byte inline LLVM classifiers handle short buffers. Buffers of at least 128 bytes use exact sixteen-byte LLVM vector loads within private tail-recursive machine-counter loops, after proving the complete source interval. Each traversal retains its byte owner once around the loop. Scalar fringes handle partial blocks, false Unicode delimiter leads, and quote transitions. Stores use checked byte primitives and complete ordinary spans use checked bulk copies. No unchecked write kernel was introduced.

The earlier eight-byte loop performed mathematical natural-number comparisons/additions and reference-count work at each block. The long traversal removes that work from its inner loop. An unconditional sixteen-byte version regressed small records, so the final implementation dispatches short buffers to the original eight-byte traversal. The retained counter baseline patch changes only the scanning entry functions and quoted-copy traversal; public APIs and compiler/runtime/driver inputs stay matched.

## Validation and reproduction

`tooling/go_csv_record_vectors.go` independently emits 252 text-reader cases, 90 text-writer cases, 160 raw-reader cases and 70 raw-writer cases. `tooling/generate_csv_record_cases.py --check` verifies the generated consumers. Stream consumers add one-byte and selected seven/4,096-byte chunk variants. Manual controls cover buffer reuse, retained continuations, parse-error progress, short writes, zero-progress writes, sticky source/sink errors, reconfiguration, UTF-8 and exact vector/dispatch boundaries.

`tooling/validate_csv_records.py` freezes all source/consumer bytes before building. It records compiler, interfaces, source and LLVM identities, then runs ordinary reclamation with a 32 MiB heap/8 MiB stack and separate poisoned-free validation with a 256 MiB heap. Every successful run must report both zero live allocation bytes and zero leaks. Reports under `bootstrap/stdlib-csv-record-*-validation-results.json` identify separate Rust, old Native and fresh fixed-point Native epochs.

`compiler/tests/lib_csv_record.rs` runs those consumers through the selected frontend, checks canonical documentation, and explicitly compiles the actual classifier/lead-byte/load kernels to wasm32, AArch64 and big-endian PowerPC objects. Foreign-object compilation establishes lowering, not foreign runtime execution.

`tooling/bench_csv_records.py` builds a retained public-pipeline driver and independent SDK comparator. Inputs are prepared before timing and a field byte changes between operations. Observable fingerprints include every field's length and sampled bytes. Timings include public input/field copies, stream ownership and returned outputs. The comparator does not claim equal allocation complexity. `copy` and runtime SHA-256 are unchanged controls. `tooling/bench_stdlib_compare.py` alternates matched before/after executables. Timing thresholds are not test gates.

The preserved character-contract reproducer isolates an existing Native checker/exporter gap around a `Char` numeric refinement. CSV uses the exact bounded mathematical `char-code` conversion instead. The authored `Data.Char` contract was not weakened; the receipt records both Native compiler identities and their original rejection.

## Measured public pipelines

The final retained counter comparison uses nine alternating samples of 256 operations on CPU0. Ratios are baseline time divided by current time. All paired fingerprints match; the independent SDK report additionally retains 32 matching workload/size results.

| Pipeline | 32-byte field | 16 KiB field | 64 KiB field |
| --- | ---: | ---: | ---: |
| Read plain | 1.002x | 5.139x | 6.590x |
| Read quoted | 0.987x | 5.048x | 3.637x |
| Write plain | 0.871x | 7.013x | 8.869x |
| Write quoted | 0.832x | 8.221x | 9.295x |
| Stream read | 0.959x | 4.099x | 3.250x |
| Stream write | 1.040x | 5.897x | 4.279x |
| Unchanged copy control | 0.989x | 1.158x | 0.994x |
| Unchanged runtime SHA-256 control | 0.996x | 1.155x | 1.004x |

The controls show roughly 15% variation at 16 KiB in this run. Large pipeline improvements exceed that variation. The 32-byte pure writers regress by about 15–20% in elapsed time; short dispatch retained small-reader performance but did not eliminate writer dispatcher overhead. Ordinary allocation probes at an identical executable path show about two extra blocks per pure write, with unchanged peak live bytes. Reader and unchanged-control allocation counts match. The traversal improvement primarily removes per-block CPU work rather than heap allocation.

Actual optimized x86-64 assembly for all four private machine-counter traversals contains unaligned sixteen-byte loads and byte-vector comparisons. LLVM unrolls seven/eight loads per body. The bodies contain zero call instructions, including zero mathematical-arithmetic or ownership callbacks. The codegen receipt identifies the exact LLVM/assembly and each isolated body hash.

A separate three-inline-hint experiment improved small writers by about 5–7% relative to current code, but did not restore baseline performance. Its 64 KiB copy control varied about 7%. The experiment is retained separately and does not change the canonical module. Earlier unconditional-wide scans also regressed short records; their original source/paired evidence remains an explicitly older experiment, including its own recorded source identities.

A dispatch-once writer experiment removed the extra two allocations and improved small writes, but the combined boundary-control program then exited 71 with corrupt generated data under both frozen Rust compilers (`f2d5ae4c...` and current `16f2f4bf...`). Every threshold size passed individually. Disabling inlining only for the private field encoder restored the combined normal/poisoned control, but also restored its extra allocations and did not clearly improve small-write performance. Splitting complete short/large encoder branches still failed. Those candidates were rejected; their patches, observed timings and exact compiler/source/LLVM failure receipt are retained separately. The canonical module remains the implementation validated by all three successful compiler epochs. The parent compiler lane owns further reduction of this contextual inlining/ownership defect.


Compiler build receipts identify the frozen Rust executable and its compiled source snapshot. The benchmark's `compiler_source_sha256` field identifies the checkout used for the report; `compiler_build_snapshot` identifies sources compiled into the frozen executable. The observed Rust host runtime archive is recorded separately in `bootstrap/evidence/stdlib/stdlib-csv-record-compiler-provenance.json`; its hash was unchanged before and after benchmark construction/measurement. Old/fresh Native runtime identities are distinct and are not attributed to Rust measurements.

To reproduce the matched baseline, apply `csv-record-counter-baseline.patch` to a copy of the final modules with `git apply --unidiff-zero`, then supply those copies as `--source-override Text.Csv.Record=... --source-override Effect.Csv=...` to `tooling/bench_csv_records.py --build-only`. Build the current artifacts without overrides. Use `--reuse --cpu 0` for independent comparisons and the retained paired report's arguments with `tooling/bench_stdlib_compare.py` for alternating measurements. `--reuse --allocation-only` observes normal allocation statistics at one identical executable path for both candidates. The reported process-wide statistics include input preparation and argument handling, with the same arguments/path in each allocation candidate.

The bounded fixture constructs expected text with a checked tail loop because the existing unoptimized `Data.String.repeat-from` constructor recursively allocates before copying. Its 64 KiB expected string exhausted the ordinary stack while the CSV row and multiline traversals completed. The extracted reproducer and failure receipt preserve that separate standard-library issue. The corrected CSV stress fixture passes Rust O0 as well as all three original frontend/compiler epochs.
