# Base32 public pipeline measurements

The isolated runner reads a selected prelude root without modifying it, freezes
each reachable module's exact bytes, copies and verifies the compiler executable,
and records compiler provenance plus source/object hashes. Explicit
`--source-override MODULE=PATH` arguments support reproducible experiments.
Measurement uses only retained artifacts and verifies their source snapshots.

[Baseline samples](base32-encoding-O2-2026-11-16.json) cover twelve pipelines at
32, 1024, 16384 and 65536 input bytes: fixed/custom byte encoding and decoding,
preserved fixed text APIs, 64-byte streaming, append and offset-buffer operations.
All 48 results match independent checksums. Both experimental sources also match
all 48 one-operation preflights under 32 MiB heap and 8 MiB stack limits, with
zero leaks. The full baseline library harnesses belong to the parent lane.

## Private reader cursor

The [cursor-copy baseline patch](base32-cursor-copy-baseline.patch) reverses only
the private surplus drain: every positive read copies its remaining surplus
instead of retaining a buffer and cursor. Scalar decoder fields, zero-coded
inverse tables and bounded legacy helpers are identical. This baseline is built
from the validated snapshot, rather than an earlier implementation.

The [alternating comparison](interleaved-base32-cursor-O2-2026-11-16.json)
measures about 1.14x for 16 KiB and 1.17x for 64 KiB stream decoding. Large byte
controls vary by at most 3.22%; the tiny 32-byte encode control varied by 7.35%,
and no 32-byte stream improvement is claimed. Equal-length executable paths in
the [space probes](base32-encoding-space-2026-11-16.json) show 64 KiB streaming
allocation falling from 9659 to 7637 blocks and from 2140836 to 850596 cumulative
bytes. Peak live memory remains 328036 bytes. Probes use ordinary reclamation,
include startup/preparation, and finish with zero live allocations.

## Adopted constant-time padding

The legacy padded text convenience scans `string-length raw` only to determine
padding, although both fixed alphabets guarantee ASCII output. The
[padding-only patch](base32-o1-padding-candidate.patch) replaces that Unicode
scan with the input-length remainder map 0→0, 1→6, 2→4, 3→3, 4→1. The first
zero is explicitly typed `Nat` to avoid local numeric defaulting.

The [paired comparison](interleaved-base32-padding-O2-2026-11-16.json) improves
legacy encoding about 1.86x at 32 bytes, 12.47x at 16 KiB and 12.97x at 64 KiB;
byte controls vary by at most 3.13%. Equal-path 64 KiB allocation falls from
209984 to 265 blocks and from 4685542 to 491162 cumulative bytes, with the same
275795-byte peak. All five remainders and empty input match baseline/reference
checksums at sizes 0–14. The padding-only change is adopted after100 focused standard/hex padded
goldens passed on both frozen frontends under ordinary and poisoned-free
validation with zero leaks. The full48-row timing sweep remains the pre-padding
snapshot; the fully measured
[before-padding report](base32-before-padding-O2-2026-11-16.json) is preserved.
The full baseline harness passed14 Rust and12 selected Native tests before this
three-line change.

## Generated code and reproduction

The actual word helpers contain packed SIMD shifts and shuffles; their only
remaining calls are backing-buffer data/length access. The outer loops still
contain checked byte stores, ownership helpers and natural-number paths.
Containment rules remain intact, and no unchecked store is hidden in assembly.
These public-pipeline results do not claim isolated kernel or allocation parity.

The [validation record](base32-benchmark-validation-2026-11-16.json) and
[snapshot manifest](base32-source-snapshots-2026-11-16.json) identify each source
role. Source snapshots and executable/LLVM/assembly artifacts remain at the
recorded local paths. The committed text module has adopted the padding patch: reverse
`base32-o1-padding-candidate.patch` to reconstruct its pre-padding baseline,
then verify the recorded hash. Apply the cursor-copy patch independently to
the unchanged Effect module. Pass the reconstructed sources via
`--source-override` when building. Pass all other frozen modules as overrides to keep the comparison
independent of later parent edits. `compare_base32_encoding.py` rejects
unexpected compiler, driver, runtime or source changes and retains every
measured sample; each case has one excluded warmup process.

The benchmark lane prepared isolated snapshots; parent integration adopted the
padding change and retained the full source identities. The full standard-library
goal remains open.
