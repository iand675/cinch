# HTML escaping and character references

`Text.Html` supplies `html-escape` and `html-unescape` for UTF-8 text,
`html-escape-bytes` and `html-unescape-bytes` for arbitrary bytes, and an exact
`html-escaped-length` counting API. Escaping replaces only `&`, `'`, `"`, `<`
and `>`, using `&amp;`, `&#39;`, `&#34;`, `&lt;` and `&gt;`. Unescaping is a single
pass and preserves unknown references. Legacy semicolon-free names use the
longest accepted prefix. Numeric accumulation wraps at 32 bits before applying
the HTML scalar and Windows-1252 replacements.

The named-reference lookup covers all 2,231 entries in the independently
vendored HTML5 data, including 106 legacy forms. Full names are packed in four
machine words and routed by a deterministic hash; each leaf checks every name
word and its length, so an unknown colliding name cannot be accepted. The
lookup constructs no runtime dictionary and allocates no temporary name text.
The reproducible generator and data/license notice live in `tooling/`.

Eight-byte LLVM SIMD classifiers scan ordinary runs and count exact escaped
capacity. Machine-counter loops retain their backing owner and load only
complete words or exact scalar fringes. Bounded name scans/fringes also use
machine counters. UTF-8 replacements are packed in one word. Escaping prepares
one private 23-byte spelling buffer only when needed, then uses the existing
checked bulk-copy operation for each replacement. It does not conceal mutable
stores inside an otherwise pure inline primitive.

The reference SDK omits `nLt;` and `nGt;`, whose five-byte references expand to
six UTF-8 bytes. The library supports both complete HTML5 names and sizes its
scratch output for expansion. Six explicit fixture rows exercise these two
differences in isolated and surrounding-text cases. Public documentation
describes HTML behavior without reference-language comparisons.

## Validation

The frozen Rust compiler and self-built compiler each pass 3,448 exact byte/text
rows and 106 larger public pipelines through 64 KiB. The corpus covers every
named reference, ambiguous legacy prefixes, malformed numeric prefixes,
32-bit numeric wrap, scalar boundaries, unrelated arbitrary bytes, output
expansion and retained-buffer independence. Two 64 KiB decimal/hexadecimal
numeric chains check bounded stack behavior. Normal execution uses a 32 MiB
heap and 8 MiB stack; poisoned-free validation separately allows 256 MiB for
the freed-block quarantine. All runs report zero leaks.

The three generator tests pass, fixture regeneration reproduces exact bytes,
and documentation extraction finds 79 canonical declarations with no
diagnostics or orphaned comments. Actual SIMD/SWAR/load kernels are compiled
to objects for WebAssembly and AArch64 without foreign execution. The supplied
Rust regression target also includes big-endian PowerPC compilation; the lane
does not claim that target was executed or that its Cargo target was run before
coordinator integration.

The self-built compiler rejects a nested checked-byte conversion passed
directly to a U8 inline primitive. An explicit let-bound U8 argument and a typed
wrapper both pass. Three small source probes retain the limitation. The final
library uses the let-bound form. Its unoptimized LLVM adds a temporary slot;
optimized LLVM is identical after removing only input-path fields, and the
production O2 assembly is byte-identical to the measured candidate. The
[equivalence record](html-native-abi-equivalence-2026-11-16.json) records exact
identities and commands. No foreign ABI or proof rule was weakened.

## Public pipeline measurements

The [first fast-path report](html-pipelines-O2-2026-11-16.json),
[fixed-store report](html-fixed-stores-O2-2026-11-16.json) and
[bulk-copy report](html-bulk-copy-O2-2026-11-16.json) each retain 72 public/reference
rows: six workloads, three input patterns and four sizes. Every observable
checksum agrees. Text pipelines include byte/text conversions. The reference
byte pipelines use its public text API; its length comparison computes the
length of a fully escaped result and performs different work from counting.
Timings include the operation loop and first-byte input mutation, excluding
process startup and input construction.

The [ordinary-text paired comparison](interleaved-html-ordinary-text-O2-2026-11-16.json)
isolates the first allocation fast paths: no-reference unescaping uses one
fresh result buffer, and unchanged immutable text returns its original value.
Observed ratios are roughly 1.21–1.67x for byte unescaping, 1.64–2.0x for text
escaping and 1.79–2.58x for text unescaping. The 64 KiB copy control varied by
6.6%, and the small byte-escaping case regressed with the extracted helper.
That helper shape was subsequently changed; the earlier regression is retained.

Dense escaping remained comparatively slow. Inspection found recursive
byte stores, zero-length range copies per replacement, repeated classification
of an immediately special byte and natural-number work in short name loops.
The fixed-store revision removes those paths. The next
[paired comparison](interleaved-html-bulk-copy-O2-2026-11-16.json) chooses a
single checked bulk copy over multiple fixed byte stores. At 64 KiB, observed
bulk-copy ratios are 1.25x for dense byte escaping, 1.19x for dense text escaping
and 1.31x for ordinary byte escaping. One sparse 32-byte escaping case regresses
about 9%; preparing a spelling buffer adds a small fixed cost. The short copy
control has noisy 64 KiB samples, with median differences up to 33%, while
length controls differ by up to about 4.4%. These are local observations with
all samples retained, not general throughput guarantees. Dense conversions and
immutable text-copy pipelines still trail the reference; no throughput or
allocation parity is claimed.

[Ordinary-space probes](html-space-2026-11-16.json) retain 96 complete-process
measurements across four implementation snapshots, two sizes, three input
patterns and four public conversion modes. Equal-length executable paths avoid
argv-size accounting differences. At 64 KiB with ordinary text, byte unescaping
drops from 265907 cumulative allocated bytes and 262519 peak live bytes to
134811 cumulative bytes and 131423 peak bytes. The final text-unescaping
pipeline drops from 528212 cumulative bytes and 393654 peak bytes to 265964
cumulative bytes and 196998 peak bytes. Startup and common input setup remain
included. Every probe ends with zero live allocations and zero leaks; these
are not isolated kernel allocation counts.

The [initial baseline](html-initial-baseline.patch),
[first fast path](html-first-fastpath.patch),
[fixed-store intermediate](html-fixed-store-intermediate.patch) and
[pre-binding bulk candidate](html-bulk-prebinding.patch) reconstruct every
measured source from the final module. Apply a patch to a temporary copy, check
its hash against the report and pass it to `bench_html.py build --source-override`.
Compilation and timing are separate invocations so the coordinating agent can
reserve the timing CPU explicitly. The broader standard library goal remains
open.
