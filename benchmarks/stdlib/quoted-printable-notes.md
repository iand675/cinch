# Quoted-printable package checkpoint

The standalone functional matrix and final policy/empty-write regression epoch
are green. Parent Cargo integration remains pending. Both timing windows are
complete, and every source epoch and raw sample remains separately identified.

The pinned SDK surface is `mime/quotedprintable` (not the nonexistent
`encoding/quotedprintable`). The surface audit yields eight declaration records.

| Audited declaration | Cinch capability |
| --- | --- |
| `NewReader(io.Reader) *Reader` | `quoted-printable-reader` over an explicit borrowed `ByteReader` |
| `Reader` | Abstract `QuotedPrintableReader` continuation |
| `(*Reader).Read([]uint8) (int,error)` | `quoted-printable-read` returns fresh bytes, simultaneous Ready/EOF/error status, and advanced continuation |
| `NewWriter(io.Writer) *Writer` | `quoted-printable-writer` with explicit text/binary policy and borrowed sink |
| `Writer` | Abstract `QuotedPrintableWriter` continuation |
| `Writer.Binary bool` | `quoted-printable-writer-policy` reads the mode; writer/encoder mode updates preserve pending line and CR carry, including readback after failure |
| `(*Writer).Write([]uint8) (int,error)` | `quoted-printable-write` returns accepted input count; errors retain that count and advanced sink |
| `(*Writer).Close() error` | `quoted-printable-finish` flushes the final line and returns its advanced borrowed sink; never closes that sink |

The value module also provides fresh whole-buffer encoding/partial decoding,
immutable ASCII encoding, explicit encoder events and immutable feedable decoder
continuations. Constructor/continuation state is private. Feed inputs, emitted
events, output buffers and retained surplus do not alias caller-owned mutable
buffers. Caller code advances stream handles explicitly; operating-system cursor
ownership follows the existing stream contract.

Writer events carry the input prefix reportable at that exact flush, independently
of output bytes accepted by a short-writing sink. Plain input batches increment
the reportable count only after their entire batch completes. Output draining uses
`stream-write-all`; it is not independently reimplemented in this package.
Failures are sticky in the adapter, which avoids replaying a partially emitted
line. Empty input still returns zero success without touching or advancing even
a failed handle; positive input and finish continue to report the stored failure.
Empty underlying byte writes follow Effect.Stream's inert-operation
contract. A final empty encoder event is a logical flush rather than a guarantee
that a custom sink callback is invoked with zero bytes.

The decoder retains the physical 4096-byte line boundary and exact buffered source
read-ahead. It distinguishes source interruption from genuine EOF when validating
a trailing soft marker. Literal malformed non-newline escapes, accepted lowercase
hex, high bytes, transport whitespace and inline-versus-source-error precedence
are represented in the independent corpus. The stream bridge returns a partial
prefix once before exposing a sticky error. New direct APIs additionally expose
original physical-byte offsets and consumed feed input. The capacity error has a
named Cinch diagnostic; fixture normalization maps it to the reference's
`bufio: buffer full` vocabulary without changing the public library diagnostic.

Sixteen-byte windows classify through checked inline LLVM; exact scalar fringes
and first exceptional bytes preserve ordering and error progress. Two bounded
little-endian word loads feed vector byte comparisons and a sixteen-lane mask.
All loads/copies carry explicit bounds; no speculative tail load is used. The
big-endian body explicitly preserves logical input lane order. Actual selected
little-endian foreign object tests cover every byte at every lane in all four
classes plus simultaneous mixed-lane masks. Big-endian execution is not claimed
by host tests. Public benchmarks retain a scalar-source scanning baseline as well
as SIMD and independently produced reference samples; they include allocation,
whole-output traversal and checksum costs, so classification speed alone cannot
predict end-to-end performance.

Functional consumers use explicit frozen cad70 Rust and 9e2 Native compiler
identities, matching runtime/provider objects, compiler host 64 MiB stacks and
consumer 8 MiB stacks. Normal 32 MiB heap and poisoned-free 256 MiB heap runs are
separate. The current-Rust Cargo integration target executes representative
committed fixtures through `CARGO_BIN_EXE_cinch`, initializes the exact host
runtime via an ordinary compiler build, and requires no SDK. It explicitly
refuses external Native selectors; standalone Native coverage uses explicit
manifest/provider arguments and `--rc-debug` during emission. Rust has no CLI
`--rc-debug` emission flag, so that limitation is reported rather than inferred
from runtime environment settings.

The zero-write review correction changes only the writer entry branch. Its exact
previous source, final source hash and small patch are preserved in
`quoted-printable-zero-write-repair.json`. Full earlier value/read matrices remain
valid because those implementations did not change. All writer traces and the
new failed-empty-write control are revalidated against the final source and the
separately gated shared Stream helper `2201b484` (source `e2c2dfb9...`). Earlier
helper/timing reports remain distinct rather than having their identities edited.

The independent supplemental corpus combines unescaped-byte failures with
malformed soft breaks and full physical buffers, ensuring that first inline
errors retain precedence over delayed transport/buffer failures. Its eight cases
run on both frozen frontends under both heap profiles.

Final validation covers 1,368 value/error cases, 41 read traces, 60 writer traces,
six continuation/offset/bridge/policy/failed-zero controls and eight supplemental error
precedence cases. Both frozen frontends ran separate normal and poisoned-free
profiles with zero leaks. The final writer epoch repeats all 60 writer traces,
six controls and eight representative readers on both frontends against the
corrected shared helper. All 104 new library declarations/constructors have
canonical documentation with zero diagnostics/orphans. Regenerating both fixture
corpora through the pinned SDK produces byte-identical committed JSON.

Final decoder documentation explicitly describes the already-tested liberal
acceptance rules. The comment-only patch changes no non-comment source line;
64 value/error cases plus all six controls reran on both frontends afterward.
The exact source hashes and patch remain distinct from the earlier full matrix.

The Binary field mapping includes readback through the encoder/writer policy
observers. Failed handles retain their last policy; policy updates remain
observable while error and byte progress stay sticky. The policy controls also
check immutable string output after mutation of its original byte input.

## Public pipeline measurements

Seven paired samples per row are retained, including unfavorable cases and
cross-window noise. Initial six-variant measurements took 11.24 seconds on the
exclusive CPU0 slot; the final two-stream refresh took 4.33 seconds in its own
exclusive slot. Other compiler/functional work could share memory and caches.
Different reference medians across the two windows make parity or causal claims
inappropriate. The scope includes complete fresh-output construction and output
checksum traversal, excluding input preparation and the retained-input traversal
performed after the end clock. Short/error behavior is covered outside timing.

Medians at 4096 input bytes, microseconds per complete operation. The SDK column
is the reference paired with the SIMD value row; each raw report retains its own
paired reference samples.

| Frontend | Operation | SIMD value | Scalar value | Final stream | SDK paired with SIMD value |
| --- | --- | ---: | ---: | ---: | ---: |
| rust | ascii-encode | 31.95 | 80.03 | 65.30 | 25.52 |
| rust | binary-encode | 158.77 | 176.90 | 185.40 | 57.85 |
| rust | ascii-decode | 59.15 | 87.18 | 62.88 | 37.23 |
| rust | binary-decode | 291.94 | 320.94 | 309.32 | 58.51 |
| native | ascii-encode | 526.97 | 744.60 | 643.40 | 29.49 |
| native | binary-encode | 2042.17 | 1995.14 | 2168.78 | 71.29 |
| native | ascii-decode | 570.49 | 703.10 | 556.98 | 38.47 |
| native | binary-decode | 1449.46 | 1534.11 | 1528.99 | 54.63 |

SIMD improves long ASCII scans over the scalar-source baseline in this sample;
it is not uniformly favorable for dense escapes or tiny inputs. Native complete
pipelines remain substantially slower than the reference. Optimized live scan
functions contain SSE2 byte comparisons and a lane mask. Rust inlines the bit
search; Native retains out-of-line machine/Nat helper calls. The scalar snapshot
has no vector classification in its live scan function, even when unused global
primitive definitions remain in the object. Bulk copies use the existing bounded
byte-copy primitive. No speculative fringe read or architecture-wide speed claim
is inferred from these measurements.

The integration sequence is implementation `377419c9`, then policy/readback and
documentation completion `cb00cdd2`, followed by this evidence checkpoint. The
source target is `cargo test --test lib_quoted_printable`; it has canonical-doc
and real current-Rust CLI runtime tests. Parent owns this Cargo gate; the lane did
not run Cargo or a full suite. Publication waits for that gate.
