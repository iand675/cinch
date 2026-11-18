# Byte field traversal: pinned-surface audit and proposed Cinch mapping

This is a static audit and design, not an implementation or completed capability
claim. The initial Split source (`2b534f95`) and both Split optimization candidates
remain unchanged. No fixtures or executables were generated for this audit.

## Reference declarations and dependencies

The pinned SDK is `go1.27.4`. Its four actual declarations are:

| Declaration | Source | Required behavior |
| --- | --- | --- |
| `Fields(s []byte) [][]byte` | `bytes/bytes.go:447` | Nonempty aliases split by Unicode whitespace; discard leading/trailing runs. |
| `FieldsFunc(s []byte, f func(rune) bool) [][]byte` | `bytes/bytes.go:506` | Nonempty aliases split by runs satisfying the supplied scalar predicate. |
| `FieldsSeq(s []byte) iter.Seq[[]byte]` | `bytes/iter.go:83` | Reusable callback traversal over the same fields without a materialized outer slice. |
| `FieldsFuncSeq(s []byte, f func(rune) bool) iter.Seq[[]byte]` | `bytes/iter.go:116` | Reusable predicate/callback traversal; early stop must prevent further predicate calls. |

Source identities:

- `bytes/bytes.go`: `e0695206833688b57e9d26014dcd28e3379fc09aafb4c702a82b90563cc11b23`.
- `bytes/iter.go`: `628f8dc51af2c70325050c0125340ec01c6a7da84c53f2f5ae3ec040c7e5fc6f`.
- Parent `Data.Unicode.Kernel`: `ce38656184b798de7b3b076bc41fa5a2f66b7b4d86afb20290e6ed427cda466b`;
  `unicode-is-space :: Char -> Bool` already implements the required Unicode
  whitespace property. The module remains source-distributed.
- `Data.Producer`: `47b6a989c18527ed19e80914ff23860e4421c239eba407effa85fe855f94f30f`;
  its step is **pure**: `s -> Step s a`. Its advance/replay contract must stay intact.
- `Data.Bytes.View` already provides sealed bounds, retained aliases, and
  `byte-view-utf8-at`; it exports neither backing buffers nor addresses.

## Byte, Unicode and ownership semantics

Returned fields are nonempty half-open views into the original fixed owner.
They retain the complete allocation and expose subsequent alias mutations.
They cannot be extended into adjacent fields; snapshots remain explicit fresh
copies. Empty/all-delimiter input returns no fields. No replacement, repair,
normalization or transcoding is performed on field contents.

The default ASCII whitespace set is exactly bytes `09 0A 0B 0C 0D 20`.
Unicode whitespace also includes U+0085, U+00A0, U+1680, U+2000–U+200A,
U+2028, U+2029, U+202F, U+205F and U+3000. U+001C–U+001F, U+180E,
U+200B and U+FEFF are not separators. Python `str.isspace` is therefore not
an unadjusted independent oracle: it also accepts U+001C–U+001F.

Decode only inside the logical view. A valid scalar advances by its encoded
width; each malformed byte presents U+FFFD and advances one byte. Literal
well-formed U+FFFD presents the same scalar with width three. Raw invalid `85`
or `A0` is **not** Unicode NEL/NBSP, whereas valid `C2 85` / `C2 A0` is.
A view starting in a continuation or ending in a truncated sequence is an
independent input. Classification never examines neighboring bytes outside it.
A custom predicate selecting U+FFFD may therefore separate malformed bytes
without altering the retained raw bytes of other fields.

A factory/producer retains its owner even when a small logical span is used.
An eager empty result need not retain the owner after the call. A reusable
sequence must retain the input while the sequence remains live, including after
one invocation stops, since another invocation must still work.

## Callback effects, order and mutation

The `FieldsFunc` API documentation does not guarantee predicate order and
assumes a consistent result for a given scalar. The actual pinned implementation
calls the predicate once per decoded scalar, left to right. The sequence version
uses that order too, with yield calls interleaved at field boundaries. Those are
separate facts: observed order is not an additional reference API guarantee.

Recommended Cinch contract: define left-to-right, once-per-decoded-scalar calls
explicitly. Permit arbitrary callback effects and stateful decisions; stable
predicates then match the reference contract, while Cinch also has a clear
behavior for state-dependent decisions. Do not demand purity, precompute a
predicate table, retry a predicate, or silently suppress its effects.

A nominally pure Cinch callback can still mutate captured `Bytes` through the
existing byte API. An empty static effect row does not justify reading or
classifying future input ahead of that callback. Each next scalar must be decoded
after the previous callback returns. IO/Mut/open-row callbacks need a genuine
effect-capable path too; a pure-only overload is not the complete capability.

The decoded scalar and width are fixed before the predicate runs. Predicate
mutation of its current bytes does not cause re-decoding or a changed increment.
Mutation of future bytes affects later decoding. Previously returned/recorded
field views reflect later writes to their spans.

At an active field's terminating separator, the sequence calls the predicate,
then yields the field. It has already decoded that separator's width. If the
yield callback mutates those bytes and returns true, the next scan still advances
by that saved width. It must not re-decode the same separator under its changed
contents. For a multi-byte separator this distinction matters. If yield returns
false, return immediately: no trailing scan, predicate, or extra yield.

## Sequence replay is different from SplitSeq

`FieldsSeq` and `FieldsFuncSeq` are not documented as single-use, and their
implementation keeps `start` and `i` local to each invocation. It never consumes
the captured slice header. Re-invoking the same sequence starts again from zero
over current aliased bytes and repeats callback effects. This differs from the
explicitly single-use SplitSeq/Lines closures audited earlier.

Two fresh pull adapters over the same reusable sequence have independent scan
positions. Copies of one adapter's next callback share that adapter's position.
Stopping one traversal does not consume a later fresh invocation. A traversal
factory, a persistent Producer, and a mutable pull cursor must not be conflated.

## Proposed concrete implementation and API shapes

The proposed home is source-distributed `Data.Bytes.Fields`, importing the sealed
view/UTF8 observations and `Data.Unicode.Kernel`. No embedded dependency change
or raw backing accessor is needed.

Use an opaque concrete `ByteFieldsState` holding only a view, consumed-byte
position, and completion flag. A private next operation also takes the predicate;
it scans leading delimiters and one field, returns a retained view and a new
state, and consumes the decoded terminating separator before yielding that state.
No predicate or effect-row parameter has to be hidden inside this datatype.

The following are design signatures, not declarations claimed to compile yet:

```text
byte-view-fields : ByteView -> List ByteView
bytes-fields     : Bytes -> List ByteView

byte-view-fields-func :
  forall (e :: Row). (Char ->{|e} Bool) -> ByteView ->{|e} List ByteView

byte-view-fields-each :
  forall (e :: Row). ByteView -> (ByteView ->{|e} Bool) ->{|e} Unit

byte-view-fields-func-each :
  forall (e :: Row).
  (Char ->{|e} Bool) -> ByteView -> (ByteView ->{|e} Bool) ->{|e} Unit

byte-view-fields-sequence :
  forall (e :: Row).
  ByteView -> ((ByteView ->{|e} Bool) ->{|e} Unit)

byte-view-fields-func-sequence :
  forall (e :: Row).
  (Char ->{|e} Bool) -> ByteView -> ((ByteView ->{|e} Bool) ->{|e} Unit)

byte-view-fields-producer : ByteView -> Producer ByteFieldsState ByteView
byte-view-fields-func-producer :
  (Char -> Bool) -> ByteView -> Producer ByteFieldsState ByteView
```

| Audited operation | Proposed primary whole-buffer API | Additional pure adapter |
| --- | --- | --- |
| Fields | `bytes-fields` | Default `bytes-fields-producer` |
| FieldsFunc | Effect-polymorphic `bytes-fields-func` | Explicit pure-predicate producer only as a convenience |
| FieldsSeq | `bytes-fields-sequence` / `bytes-fields-each` | Persistent default Producer |
| FieldsFuncSeq | Effect-polymorphic `bytes-fields-func-sequence` / `bytes-fields-func-each` | Explicit pure-predicate Producer, never substituted for the general operation |

Whole-buffer wrappers retain `bytes-view input`. The complete sequence mapping
is the ordinary callback factory/visitor, which supports effectful predicates
and yields. Its outer construction is pure and does no input scan or callback;
each invocation starts from fresh explicit state. The final function arrow
carries the effects. Pure Producer adapters are useful additional interfaces,
not a replacement for the effect-capable operations. Never store an effectful
step in `Data.Producer` or add hidden mutation to its persistent state.

Use one combined row `e` for predicate and yield in the general visitor. Validate
actual effect subsumption with predicates/yields needing different IO/Mut rows;
caller wrappers may need an explicit shared union annotation. Do not invent an
unsupported union of two row tails. If a reusable row-indexed effect Producer is
later desirable, it needs its own concrete operations and both-frontend kind/ABI
proof before use; no placeholder effect typeclass is proposed here.

Before implementation, run tiny signature/closure proofs in both frontends for
pure, IO, Mut and mixed callbacks, early stop, and a retained sequence invocation.
Keep any row/region/callback refusal as a compiler issue rather than narrowing
the API to pure or IO-only predicates. This audit does not assume that a nominal
data parameter of kind Row or a higher-rank result already works.

## Acceleration and complexity

Default whitespace scanning can use bounded SIMD/SWAR ASCII blocks and Unicode
scalar fallback. A useful separate dependency would be a minimal sealed View
observation that advances over an ASCII whitespace or non-whitespace run, stops
at the first non-ASCII byte/class change/end, and never exports backing/pointers.
A non-ASCII boundary then uses the existing bounded scalar decoder and Unicode
kernel. Keep ownership/range validation outside a private machine-counter loop;
use straight-line LLVM classifiers with ordinary branches and checked stores.

Do not perform six whole-remainder index-byte queries per field and choose their
minimum: absent whitespace kinds repeatedly scan the remainder, making dense
inputs quadratic. Do not cache block classifications beyond a yield, where
future bytes may change. A linear single-pass field scan is required.

Generic predicate traversal must remain scalar and just-in-time unless an
explicit separate immutable predicate descriptor proves otherwise. Even cached
ASCII rune decoding can become stale after a callback mutates the next bytes.
This is a semantic reason not to batch arbitrary callbacks, not a pure-only API
restriction. Field collection uses a tail accumulator/reverse; no allocation on
recursive unwinding. Benchmark the actual complete list, callback and Producer
pipelines with output/alias checks, and inspect any residual imported scalar-call
overhead before selecting a kernel.

## Planned independent validation, not generated yet

- All 25 Unicode whitespace points and nearby non-space exclusions; all 256
  single raw bytes; valid U+FFFD versus malformed/truncated bytes and all bounded
  start/end positions of two-, three- and four-byte scalars.
- Empty/all-delimiter, leading/trailing runs, embedded NUL, ASCII word/vector
  fringes and long dense/sparse/multibyte fields, preserving every output byte.
- Predicate scalar log and yield log interleaving; no calls at construction;
  no calls after early stop; repeat invocation and two independent pulls;
  copied-next sharing and stable exhaustion/stop behavior.
- Deterministic predicates with observable IO/Mut effects, plus Cinch's chosen
  stateful-decision extension tested independently of reference guarantees.
- Mutation before first scan, during predicate, at yield, of already returned
  aliases, of a multi-byte terminator and of future UTF8; preserve decoded width
  at a yield boundary. Empty-result and temporary-owner retention controls.
- Both compiler frontends, O0/O2, normal 32M/8M and poison 256M/8M zero-leak
  profiles; actual checked-RC API builds explicitly separated from CLI defaults.
- Separate gates for any new sealed ASCII-run observation, then complete public
  wasm/ARM/PPC objects and meaningful changed-input public pipeline timings.

The full strings/bytes initiative remains open. This document claims an operation
inventory and implementation design only.
