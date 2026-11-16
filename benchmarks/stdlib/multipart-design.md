# Multipart capability design

Status: audited design, no implementation or runtime qualification yet. SDK
`go1.27.4` (2026-11-02 build), obtained from the pinned audit tool. Isolated
lane `lane/stdlib-multipart`, base Root `4c15582a`. Existing `Text.Mime`,
`Effect.Mime`, and `Effect.Stream` are unchanged. The adjacent
`multipart-surface-plan.json` maps every one of the **37** pinned SDK records to
proposed operations and records exact audited source hashes. Operation names
below are proposals, not exports or completed capabilities.

## Representation and ownership

Use `Text.Multipart` for pure byte-level boundary validation, disposition
escaping, exact header serialization, bounded delimiter/header scanning and
resource-budget arithmetic. Use `Effect.Multipart` for source/sink continuations,
part sessions, form accumulation, scratch storage and random-boundary creation.
Every declaration, constructor and private helper receives canonical `-- |`
documentation; public library comments describe the native contract without
references to another language.

All wire text, header keys/values, field names, filenames and values have byte
representations. UTF-8 convenience wrappers must be explicit. Reusing a `String`
surface that rejects arbitrary bytes would narrow this package. Parsed media
parameters reuse the checked MIME parser through an explicit byte bridge; header
serialization does not use the MIME media-type formatter because disposition
escaping is different.

The effectful API uses an explicitly shared session with `Mut r` and `IO r`
in its operation rows. A private `Var r` stores each reader/writer continuation;
the caller owns the session handle, and source/sink callbacks always return their
advanced continuations for storage there. Each part handle carries its session
cell and a separate part-state cell. Reader/writer session state may point to the
current part-state cell, but that cell never points back to the session. Thus the
representation is acyclic: it preserves actual aliases without an RC cycle or
hidden global identity. Constructors remain private. Operations on part handles
do not take an independently supplied owner/index pair, so no foreign-owner ID
comparison or fabricated generative token is needed.

Advancing to another input part drains the previous body through the same shared
source. Old handles retain their metadata/cache and observe the exhausted body.
Output part handles retain their closed/error state after replacement. Headers
and file-header metadata use explicit shared mutable cells as needed; snapshots
returned to clients are fresh bytes/lists, with named update operations modifying
the shared metadata. Source/sink inspection returns the current continuation,
not a stale copy captured at original construction. The API documents borrowed
versus owned handles and never closes the original source/sink implicitly.

`Data.Mutable.Var` supplies existing checked storage. A first bounded proof must
check the combined `Mut r`/`IO r` rows, callback-state representation and acyclic
part/session lifetime before the full parser is implemented. A pure explicitly
advanced scanner state is still appropriate underneath, but it must not replace
the effectful alias protocol with replayable snapshots. Public form/file values
use the same explicit shared ownership model so moving a file header between
lists retains its identity and editable metadata.

Header objects expose exact raw-key get/replace/append/delete operations and
canonical-key convenience lookup separately. A value is an ordered byte list;
empty lists and arbitrary raw keys are representable. `Part.Header` is replaceable
and its entry lists are editable. `Form.Value` and `Form.File` support enumeration,
whole-list replacement, append and deletion. File-header identity is separate
from the lists containing it, so the same file header can occur under multiple
keys. `Filename`, `Header`, and signed `Size` are all readable and replaceable.
Replacement never rewrites private backing-resource identity.

## I/O capabilities and adapters

A source callback receives requested capacity and returns fresh bytes, an
independent status (`More`, `End`, or `Failure e`), and its advanced state. A sink
callback receives a borrowed byte offer and returns the confirmed prefix length,
an optional caller error `e`, and advanced state. The prefix is not hidden inside
an error. Zero-byte reads and writes can invoke callbacks. Invalid counts are
explicit capability-contract errors; valid counts plus errors retain both.
Callbacks may retain input offers: internally generated offers are fresh or
immutable and never subsequently reused as writable scratch storage.

Do **not** change `Effect.Stream`. Its `Result StreamError Bytes` cannot represent
bytes plus an error, its writes cannot represent an error with a positive count,
and its zero reads are inert. Provide explicit adapters: incoming stream success
maps to `More` or `End`; stream failure maps to a caller-error variant with zero
confirmed bytes. Outgoing stream bridges deliver retained bytes before the
terminal error and document their zero-read policy. The richer native interface
is authoritative for exact multipart behavior. `stream-write-all` is useful only
where the operation requires draining; it must not add retries to single-write
operations.

Scratch storage is an explicit caller capability with independently advanced
backend state: create exclusive temporary object, append/write with count and
error, close write handle, open a fresh read handle, sequential read, read-at,
signed seek, close read handle, and unlink with distinguishable missing-object
status. Every failure retains backend progress and outstanding ownership. File
handles have independent cursors; read-at does not change that cursor. A real
filesystem test backend and an adversarial scripted backend are required. A
memory-only store advertised as disk spilling is not an implementation.

Existing `Effect.Fs` only exposes whole-file read/write/metadata/list/rename.
`Prim.Sys` exposes borrowed descriptors, seek, unlink and ordinary reads/writes,
but no positioned read or exclusive temporary create facade. Existing
`FileUtils.temp-file` is explicitly check-then-create, not exclusive; do not use
it as an atomic scratch allocator. The multipart backend interface permits real
caller-supplied scratch storage without altering either shared module. A built-in
OS backend would require a separately reviewed capability/runtime increment;
do not implement read-at as seek+read on a shared cursor without restoration and
the required concurrency contract.

Random construction takes an explicit entropy source and draws exactly 30 bytes,
rendered as 60 lowercase hex bytes. Entropy failure is typed and retains state;
the reference panic becomes an explicit capability failure. Also provide a
deterministic validated-boundary constructor. `Effect.Random` can supply a
convenience adapter, but its runtime entropy failure policy is not silently
represented as recoverable success.

An explicit policy carries filename-basename behavior, limits, scratch sharing,
and temporary namespace. Default limits are 10 MiB part-header memory, 10,000
header entries, 1,000 form parts, and combined scratch files. Named explicit
overrides replace ambient environment controls. Filename basename follows the
selected host path profile: on Linux a backslash is a byte, not a separator.

## Operation-level commitments

The callback-level shape is `read(request, state) -> (bytes, status, next-state)`
and `write(offer, state) -> (count, optional-error, next-state)` under the caller's
I/O capability. `e` is a generic caller error type, not a stringified exception.
The engine returns `MultipartFailure e`, whose transport/scratch variants retain
the original `e`, and whose protocol variants retain raw offending bytes and
absolute progress. A form-read failure additionally exposes outstanding cleanup
work and the advanced scratch backend. That report supplements the original
primary error instead of replacing it with cleanup noise.

Public state operations are shaped as follows (schematic, not checked source):

* `multipart-next-part reader -> Result failure (Option part)`;
  `None` is clean multipart EOF, distinct from wrapped source EOF failure.
* `multipart-part-read part capacity -> (bytes, status)`; the shared reader
  and part cells retain exact progress, including on failure.
* `multipart-create-part writer headers -> Result failure output-part`;
  `multipart-part-write output-part bytes -> (count, status)`.
* `multipart-read-form reader policy scratch -> Result failure form`;
  reader/scratch sessions preserve their advanced state even without a form result.
  Form access/edit operations retain stable file-header handles. Backend key and
  file handle types stay generic, with no assumed global descriptor namespace.

This separates callback/backing state from payload snapshots. Byte results and
published header/value views are fresh snapshots unless an explicitly documented
editable context operation is used. Failed partial output does not roll back the
sink, failed input does not rewind the source, and no automatic retry repeats
already-confirmed bytes. Native-width inputs are bounded against the actual
target profile; offset/size accounting uses signed 64-bit semantics explicitly.

| Pinned operation/record group | Native contract |
| --- | --- |
| `FileContentDisposition` | Exact `form-data; name="..."; filename="..."`; escape quote/backslash with backslash and CR/LF as `%0D`/`%0A`; retain every other byte. |
| `NewReader`, `Reader` | Borrow source, snapshot arbitrary boundary bytes without writer validation, retain sticky terminal source status and a 4096-byte lookahead policy. Empty boundary fails on next-part. |
| `NextPart` | Drain current part, suppress drain error as specified, then parse next headers. Hide first canonical transfer-encoding value when Unicode case-fold equal to `quoted-printable`; decode only its body. |
| `NextRawPart` | Same cursor and framing rules, preserving transfer header and raw body. |
| `Part`, `Part.Header` | Scoped session handle plus live shared header replacement/entry operations; metadata remains available after body exhaustion. |
| `Part.FormName` | Parse disposition lazily once; return name only for `form-data`. Header edits before first accessor affect cache, edits afterward do not. |
| `Part.FileName` | Share same lazy cache; filename independent of disposition kind, with explicit path-basename policy. Failed parse initializes an empty cache. |
| `Part.Read` | At most requested bytes plus terminal status and shared progress; preserve positive prefix with EOF/error. Zero reads can prefetch; do not shortcut them through inert stream read. |
| `Part.Close` | Drain body, ignore its drain error, return success with advanced shared source. Never close borrowed source. |
| `ReadForm`, `Form` | Complete form accumulator with signed memory budget, exact accounting, real spill capability, field/file grouping and cleanup on failure. Return advanced source/backend even when no form result survives. |
| `Form.Value` | Raw byte keys to ordered byte values, all read/replace/append/drop operations. |
| `Form.File` | Raw byte keys to ordered file-header handles, all read/replace/append/drop operations; duplicates and moved handles remain representable. |
| `FileHeader`, `Filename`, `Header`, `Size` | Stable private backing identity with editable public metadata, including signed size; header view preserves multi-values. |
| `FileHeader.Open` | New independent file handle on each call, using actual memory/shared-section/distinct-disk behavior described below. |
| `File` and its four members | Full sequential read, read-at, signed seek with raw native-width whence, and close. Preserve prefix/status, cursor position, reopen, and backing-specific close behavior. |
| `Form.RemoveAll` | Attempt unlink for every disk header reachable through current file lists; ignore missing, continue after errors, return first encountered nonmissing error and exact backend progress. |
| `NewWriter`, `Writer` | Borrow sink and choose random boundary through explicit entropy capability; deterministic constructor is additional. Retain current output-part status. |
| `Boundary`, `SetBoundary` | Get exact bytes. Set requires no current part, then length 1–70, then permitted ASCII validation. Interior spaces allowed; trailing space refused. Error precedence is preserved. |
| `FormDataContentType` | Exact `multipart/form-data; boundary=...`; quote boundary iff it contains a MIME special or space. |
| `CreatePart` | Close old part first, preserve sticky last error; exact sorted raw keys and original value order. One header write with short-write detection, then allocate new part only on success. |
| `CreateFormField`, `CreateFormFile` | Exact disposition convenience headers; file includes `application/octet-stream`. Return explicit writer owner and borrowed output-part handle. |
| Returned part writer | One callback per write, including empty writes; pass through positive count plus error. Last nonnil error is sticky but later errors replace it. Finished-part write refuses. |
| `WriteField` | Create field then exactly one body write; return its error, not a synthesized short-write error when the callback reports a short successful count. |
| `Writer.Close` | Close current part; sticky failure aborts before footer. Otherwise clear current part and make one footer write. Not a terminal state: repeated close repeats footer; later part creation starts a first-part boundary. Footer short count without error remains success. Never close sink. |
| `ErrMessageTooLarge` | Dedicated `MultipartMessageTooLarge` constructor, separate from caller transport/backend errors and malformed-header diagnostics. |

Writer header creation calls `io.Copy` on a buffer with `WriteTo`, which makes
**one** underlying write; short success becomes `io.ErrShortWrite`. `WriteField`
and footer formatting each make one write and do not synthesize that short error.
This difference is a required fixture group, not an opportunity to replace all
three paths with an unconditional drain loop. A failed new header write already
closed the previous output part; the previous current-part pointer remains until
successful replacement/finish. Repeated errors and recovery must preserve that.

## Framing, headers, and bounded scanning

Skip arbitrary preamble before the first boundary. Ordinary delimiters allow
trailing space/tab. A first delimiter ending in LF selects LF framing; otherwise
CRLF remains required. A final delimiter permits no ending newline even at EOF.
After the first part, unexpected nonseparator lines are errors. Retain the exact
quoted offending bytes and distinguish wrapped source EOF from clean final EOF.
The body scanner recognizes initial `--boundary` and later newline-prefixed
delimiters only with the reference follow-byte rules: whitespace/CR/LF, `--`, or
end status; a single final dash is not a delimiter. Preserve incomplete delimiter
prefixes across every input chunk boundary and do not emit their preceding NL.
Boundaries accepted by the reader may exceed writer length limits; finite
lookahead buffer-full behavior must be fixture-observed, not assumed unlimited.

Header parsing accepts folded lines, normalizes continuation whitespace, rejects
initial continuation and missing colon, validates value bytes, and canonicalizes
ASCII token keys. Space-containing keys are accepted without canonicalization;
tab/non-token invalid keys are refused. A malformed/limited header does not
publish a partial new part even if an internal header parser collected entries.
Duplicate values count toward limits. For QP header selection use the actual
case-fold policy, not an ASCII-only equality shortcut. QP body decoding must use
the richer raw-part source so input errors/prefixes and zero-read observations
are not lost through a Stream bridge. The QP wrapper itself makes zero reads
inert, unlike raw part reads, because its decoder loop requires positive output
capacity. Existing pure QP decoder behavior and
actual pinned reader must be checked before deciding whether additional local
state is needed; no shared QP source changes are assumed.

SIMD/SWAR candidates: classify CR/LF/colon/invalid header bytes, find first
delimiter candidate, escape disposition safe spans, and copy confirmed payload
spans with checked byte-range primitives. Full delimiter verification remains
bounded and exact. A prefix automaton or equivalent search prevents adversarial
repeated prefixes from quadratic rescanning. Retain scalar fringes and prove
all lane loads before inline LLVM access. Small state transitions and one-byte
decisions remain scalar where vector setup gives no benefit. No unbounded load,
host-endian assumption, or unsafe eager match beyond valid input is permitted.

## Form budget, backing modes, and cleanup

Normalize negative max-memory to zero. File-content budget starts at max-memory,
except signed MaxInt64 is reduced by one. Total budget adds 10 MiB; signed
overflow/nonpositive sum saturates to MaxInt64. Charge name length plus 200 per
named part, including repeated names. Values charge full byte length. File
metadata charges header size (400 plus key length +200 per key plus all value
lengths), another 200 map overhead, and 100 file-header overhead. Only retained
in-memory file data reduces the file budget; disk bytes do not. Named non-file
parts never spill. Skipped unnamed parts still count toward part limit.
ReadForm uses shrinking total-memory/header limits for new part parsing, and
decrements the aggregate header count for file headers only. Preserve rejection
order around limit checks and reads; prechecking all parts could change effects.

Spill writes the buffered threshold+1 prefix and subsequent content to real
scratch storage. Combined mode appends files to one backing object with offsets;
distinct mode closes each backing object after its file. More than one disk file
in combined mode marks disk headers as sections. Memory Open ignores edited
public Size and uses content length. Shared-disk Open uses stored offset plus
current public Size; distinct-disk Open reads the entire object and ignores Size.
Memory close is a no-op (including later reads and repeated closes), whereas
disk close/repeated close/read-after-close follow backend results. A successful
unlink does not itself close already-open descriptors.

Resource cleanup is explicit. Failed form parsing attempts scratch close/unlink
while preserving the original failure, except final close failure becomes the
result when no earlier failure exists. Failed cleanup operations remain visible
in returned backend state/cleanup report so callers can retry; they must not
silently disappear. Successful forms retain all file IDs even if detached from
the public field lists. `RemoveAll` deliberately traverses only current lists,
matching published field-mutation behavior. An additional explicit release of
detached headers can prevent leaks without changing `RemoveAll`. No automatic unlink
is tied to session/context dropping, since it could invalidate retained aliases.
Map enumeration/first-error order is explicit in native context; no promise of
matching randomized reference map iteration. Fixtures should assert all attempted
cleanup and first error under their explicit order rather than invent cross-run
ordering guarantees.

## Independent fixtures, codegen and benchmark gates

The public pinned SDK oracle must emit exact bytes, statuses, errors, confirmed
prefixes, callback traces, file offsets and cleanup traces. An independent scalar
reference validates framing/search and fixtures without invoking the library.
Committed fixtures and C host-object callbacks keep CI SDK-free. Required groups:

* Every reader/writer boundary byte, lengths 0/1/70/71, arbitrary long reader
  boundaries, every delimiter follow byte, CRLF/LF, preamble/epilogue, final EOF,
  all chunk split positions, repeated-prefix adversaries and buffer-full errors.
* Empty and malformed/folded headers, repeated values, exact raw-key ordering,
  empty key/list/value serialization, all byte escaping, Unicode case-fold
  transfer header, raw/default QP, malformed QP and terminal errors with prefixes.
* Scripted source zero/short reads, simultaneous bytes+EOF/error, sticky terminal
  source, part close/drain failures, old handles after next, header replacement before
  and after lazy disposition cache, interleaved independent owners, retained/mutated offers.
* Writer single-write count/error matrix, closed old handle, last-error replacement,
  header error after closing old part, retry, repeated close, post-close boundary
  reset/part creation, zero writes, entropy short/error, borrowed sink ownership.
* Signed memory edge values and exact accounting thresholds, zero/override limits,
  aggregate file-header count, skipped unnamed parts, memory/shared/distinct spill,
  mid-spill write/close/open/unlink errors and failure cleanup progress.
* Every published Form/FileHeader field mutation; same header in several lists,
  detached header cleanup, negative/changed Size on each backing mode, independent
  opens, zero/short sequential/read-at, negative/end-relative/invalid-whence seek,
  close twice, read after close, remove while open, reopen after remove, path profile.
* Independent C scalar/kernel comparison for all byte lanes, vector fringes,
  suffix lengths and escaped-byte cases. Actual AArch64 and wasm foreign objects,
  plus host execution; no execution claim for foreign targets without execution.

Run bounded batches on explicit frozen Rust and Native identities with their
matched interfaces/objects/runtime; record any compiler admission refusal rather
than source-shadowing. Qualify O0/O2 normal 32 MiB heap/8 MiB consumer stack and
separate 256 MiB poisoned-free/8 MiB stack, with zero leaks. Compiler host stack
is 64 MiB (including Native runtime worker), separate from consumer stack. Root
owns Cargo integration; add a representative current Rust CLI normal/poison test
once executable library source exists. No result is claimed by this design.

Public benchmarks cover parse/drain, write parts, form values, memory files,
scratch-spill forms and file read-at/seek where meaningful. Use exact immutable
SIMD/scalar source snapshots and a public SDK reference driver, preflight complete
output plus retained-input/context checksums before timing. Match boundary,
headers, callback chunking, temp backend and limits; separate kernel-only from
public-pipeline results. End clock before retained-input traversal and teardown,
check actual optimized IR/assembly, retain all samples and regressions, report
quota/background work and backend differences. Ask for an exclusive bounded
CPU0 timing window only after functional preflight; coordinate CPU0/4 with peer
lanes. Static audit/design used no CPU compile slot.
