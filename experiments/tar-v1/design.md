# Tape archive candidate

This is an isolated implementation draft. Metadata and block codecs pass one
explicit frozen-compiler source check and canonical documentation extraction;
`source-qualification-results.json` preserves all three attempts and the exact
selected source hashes. The stream framing draft remains unchecked. No reference
generator, runtime, portability test or benchmark has run. Nothing is installed
into the prelude. The 85-record audit is a capability plan, not a
completion claim. `operation-mapping.json` preserves every installed API record,
including duplicate historical constant declarations, with a concrete target.

## Module boundaries

`Data.Tar` contains immutable byte names, complete header metadata, timestamp
presence, format choices and named errors. `TarText` snapshots input bytes and
copies on byte extraction. Invalid UTF-8 names remain representable. Numeric
owner IDs use signed 64-bit carriers on every target, avoiding host-width
truncation. Nanoseconds use floor seconds plus a nonnegative fractional part;
missing time and the Unix epoch are distinct.

`Text.Tar.Block` implements exact 512-byte blocks, zero detection, both checksum
conventions, bounded NUL fields, padded octal and signed base-256 numbers, and
checksum sealing. The checksum field is replaced by eight ASCII spaces during
both sums. A zero block is framing, not an invalid header. A checked block
retains a fresh copy. Recognition of a checksum-valid block does not yet decode
its metadata or validate a whole archive.

`Effect.Tar.Framing` implements one physical block read/write using the existing
`Effect.Buffered.Core` event contract. Its accepted prefix and optional terminal
status remain observable together. Callback counts are validated before buffer
access. Source buffers are copied before another callback. At most 100 empty
nonterminal reads are attempted; a positive read with a terminal status does not
discard the data. The next archive layer must retain pending terminal status,
including an EOF that accompanies the 512th byte.

The remaining planned codec layers are `Text.Tar.Header`, `Text.Tar.Pax`, and
`Text.Tar.Sparse`; the reader/writer controller will live in `Effect.Tar`.
There are no placeholder exported functions for these unfinished components.

## Existing capabilities inspected

The exact inspected files and SHA-256 values are in the audit JSON. The current
`Effect.Stream` reader returns `Result StreamError Bytes`, and its writer returns
`Result StreamError Nat`; neither carries positive progress and failure in the
same result. Its descriptor adapters borrow their descriptors. Its memory
adapters snapshot retained bytes. These adapters can be accepted through
`buffer-source-from-stream` and `buffer-sink-from-stream`, with the loss of
same-call progress/error information inherent in the source contract documented.

`Effect.Buffered.Core` supplies signed `ReadEvent` and `WriteEvent` counts,
optional errors, callback buffer snapshots and authoritative next continuations.
Its validated event count functions are the primary transport boundary. Direct
transfer capabilities are optional optimizations; archive body limits and sparse
framing must remain enforced if such a capability is used.

`Data.Bytes` supplies fixed-length mutable buffers, proved byte/range access,
bulk overlap-safe copies and a bytewise comparator. `Data.Bytes.Word` supplies
unaligned, endian-normalized exact four/eight-byte reads. The draft uses those
loads rather than inventing another raw-pointer ABI. `Data.String` requires
valid UTF-8 on byte conversion, so raw archive names cannot use String alone.

Current `Effect.Fs.FileMeta` contains only kind and size; filesystem stat follows
symlinks and does not supply archive modes, owner names, all timestamps, device
numbers or read-link. It cannot silently satisfy full tree ingestion. The
archive API needs a typed abstract filesystem capability with these operations.

## Owned stream and entry design

The proposed controller has an explicit region parameter and owns its current
source/sink continuation. Every operation returns the authoritative continuation,
including after a callback fails. It never closes a caller-owned descriptor.
The basic API works through the controller rather than returning a second
independently advanceable body stream.

If a body stream adapter is added, its callbacks and `tar-next` must share one
private IO cell and an entry generation. Advancing to the next entry invalidates
older body handles, which return a named stale-entry error without consuming
another entry. A busy marker prevents a callback from reentering a half-updated
controller. No callback may observe cached state across another callback, and
no backing-buffer address is retained across effects. The IO cell uses verified
existing ownership APIs; no unsafe circular object graph is needed.

The planned signatures are:

```
tar-reader : TarReadOptions -> BufferSource r -> TarReader r
tar-next : TarReader r ->{IO r} (TarNextEvent, TarReader r)
tar-read : TarReader r -> Nat ->{IO r} (TarReadEvent, TarReader r)
tar-reader-source : TarReader r -> BufferSource r
tar-writer : BufferSink r -> TarWriter r
tar-write-header : TarWriter r -> TarHeader ->{IO r} (Option TarFailure, TarWriter r)
tar-write : TarWriter r -> Bytes ->{IO r} (TarWriteEvent, TarWriter r)
tar-flush : TarWriter r ->{IO r} (Option TarFailure, TarWriter r)
tar-finish : TarWriter r ->{IO r} (Option TarFailure, TarWriter r)
tar-writer-sink : TarWriter r -> BufferSink r
```

These are design signatures, not checked declarations. `TarNextEvent` carries an
optional header independently of a diagnostic. `TarReadEvent` carries copied
logical bytes and an optional status. `TarWriteEvent` carries the accepted prefix
and optional status. EOF is distinct from a malformed/truncated header.

## Required state and option semantics

Reader state tracks physical offset, remaining physical data, remaining logical
data, padding, sparse fragments, a pending transport status and sticky failure.
`next` discards unread physical bytes and padding before parsing the next header.
At end-of-input, zero available bytes or one complete zero block followed by EOF
is normal archive termination. Two zero blocks terminate without a further
read. A zero block followed by a nonzero block is invalid; a partial block is
unexpected EOF. A body read cannot consume bytes of the next header.

Path checking is an explicit policy, with a selected POSIX/local-host interpretation;
there is no hidden environment-variable dependency. A nonlocal entry name can
return its parsed header together with `TarInsecurePath`. This advisory must not
poison later reads. Link targets are not checked by the entry-name policy.
Path safety does not implement extraction or promise to secure a later filesystem
operation. Metadata size, sparse entry count and empty-callback allowances are
explicit limits; the compatibility defaults are one MiB, one Mi sparse entries,
and 100 consecutive empty nonterminal callbacks respectively.

Writer state tracks declared logical bytes, padding, current kind, sticky
transport failure and finished status. Overlong writes report the accepted
prefix with `TarWriteTooLong`; this error is not a sticky sink failure. A header
or flush with an incomplete current body reports its missing count without
discarding the body. Finish attempts padding and two trailer blocks and marks
the writer finished even if body validation or a trailer write fails. Later
finish calls succeed and later writes are refused. The first finish result
preserves any missing-body or sink error. A preexisting
sticky transport error is returned before another output attempt. Finishing the
archive does not flush or close an external buffered sink.

Header input is snapshotted before retention. All encoding refusals are computed
before writing that header's first byte. Automatic output rounds modification
time to nearest whole seconds and omits access/change times, then chooses the
first representable USTAR/PAX/GNU encoding. An explicit PAX request may use the
USTAR subset. Negative body size is invalid, except size is ignored by header-only
types. Legacy zero flag becomes directory for a trailing-slash path, otherwise
regular file. Reserved/unknown flags remain representable and are tested rather
than silently treated as a known kind.

## PAX, long names and sparse data

Local PAX headers are consumed and affect the next visible entry. Duplicate
ordinary keys are last-wins. Empty values do not overwrite basic fields. Global
PAX headers remain visible records and do not persist implicit state to later
entries. GNU long pathname/link records are consumed in sequence, terminated at
the first NUL, and override the corresponding following field when nonempty.
Extension bodies remain bounded independently of the eventual logical file size.

PAX record lengths count decimal-length digits, space, key, equals sign, value
and newline, and require an exact self-consistent byte count. Unknown valid keys
are retained. Encoding sorts keys bytewise. Basic fields derived from the header
override raw PAX keys; compatibility attributes override corresponding
`SCHILY.xattr.` records. Local raw sparse keys cannot forge a sparse writer plan.
Raw metadata values remain byte-exact; required UTF-8/NUL/key rules are validated
where the selected format requires them.

Reader sparse support includes old GNU inline maps and extension blocks, PAX
0.0 ordered offset/length records, 0.1 comma-separated maps and 1.0 body-prefixed
newline maps with block alignment. Fragments must be nonnegative, ordered,
nonoverlapping and within logical size. Physical size and logical size are
separate. Body reads synthesize holes as zeros; `next` skips only physical
remainders. Empty/all-hole files, zero-length fragments, truncated data and
unreferenced physical bytes need distinct fixtures. Unknown sparse versions
must retain the pinned observed fallback, not be misparsed as a known version.

The audited writer does not provide a public sparse-file construction facility.
The implementation must not claim sparse encoding from an accepted flag alone.
Tests will record the actual sparse-flag behavior and review any intentional
named refusal independently. Parsing sparse archives remains required.

## Metadata and tree capabilities

The pure metadata projection preserves archive mode flags, directory basename,
size, modification time and typed source-header provenance. Header reconstruction
from that provenance clones attributes and records and preserves hard links,
owner fields and additional timestamps. Missing metadata is an explicit input
refusal rather than a dereference of an absent object.

`TarFileMetadata` will carry base name, kind/mode, size, modification time and
optional source-header provenance. A separate optional owner-name provider can
fail group and user lookup independently, in that order. No ambient account
lookup is invented. Device information is explicit. A host adapter can supply
these capabilities later without narrowing the archive metadata model.

Tree ingestion needs lexically ordered child listing, lstat, read-link and
open/read/close capabilities. It skips the root dot, appends directory slashes,
includes regular files/directories/symlinks, refuses unsupported nodes, never
follows a symlink to recurse, and closes every opened file after its copy even
when copying fails. Header/copy/close outcomes remain inspectable. Streaming the
input tree is required; reading all files into a single buffer is not equivalent.

## Acceleration and ownership gates

Eight-byte SIMD checksum reductions widen each lane to 16 bits before summing;
the maximum unsigned chunk sum is 2040 and the signed range is -1024..1016.
Reduction order is endian independent. Word loads are exact and boundary checked.
SWAR zero-byte masks identify the first NUL in normalized little-endian words;
borrow propagation can mark later bytes but cannot move the first match earlier.
Scalar tails read exactly the remaining field bytes. Binary/octal parsing has
short sequential carry dependencies and preserves checked overflow; it must not
be declared vectorized merely because checksum processing is.

Future scalar/SIMD public pipeline comparisons must include ordinary ownership
traffic, snapshots, metadata decoding and real stream callbacks. Benchmark
workloads include header decode/encode, short/long names, PAX maps, dense bodies,
sparse holes and next-with-unread-body. Measured codegen/allocation costs are
reviewed before selecting representations or advertising acceleration gains.

## Independent and maintained gates

`reference/main.go` uses the installed public SDK directly and emits raw archive
bytes, decoded full headers and ordered writer/reader operation observations.
All byte strings are hex, preserving malformed UTF-8. It also loads bounded raw
installed fixtures for independent GNU/PAX sparse and legacy inputs. The source
has not been compiled or executed. Its eventual receipt must identify actual
SDK executable/source/input hashes and command, not infer observations from this
draft. Compressed fixtures require a separately bounded decompression gate.

Additional oracle cases before qualification: partial progress with simultaneous
transport errors, no-progress/invalid callback counts, sticky vs recoverable
errors, format combinations, owner-name callback failures, strict/permissive
locality including retained advisory headers, PAX length/duplicate mutations,
base-256 extrema, signed checksum fixtures and every split around 512-byte
boundaries. The existing generator is a starting corpus, not full coverage.

Maintained SDK-free targets will be `lib_tar_blocks`, `lib_tar_headers`,
`lib_tar_reader`, `lib_tar_writer`, `lib_tar_sparse` and `lib_tar_tree`. They must
retain failed projects and execute independent exact vectors under checked
ownership, normal 32 MiB/8 MiB stack and poisoned 256 MiB/8 MiB stack with zero
leaks. Every target needs actual portable objects on the supported target set;
source acceptance alone is not a runtime, ABI, ownership or performance gate.
