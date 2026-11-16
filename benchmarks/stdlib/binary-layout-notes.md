# Typed fixed-size binary layouts

`Text.Binary.Layout` supplies explicit fixed-size descriptors with exact machine
fields, target-native wire order, checked buffer offsets and minimal variable
integers. `Effect.Binary` adds fixed-frame and bounded variable-integer stream
operations. Existing `Text.Binary` behavior is preserved.

A `BinaryLayout a` is an actual codec, with size, validation, encoder and decoder
implementations. Concrete layouts cover `Bool`, signed and unsigned 8/16/32/64-bit
integers, `F32`, `F64`, complex pairs, and the standard `Complex` value. Arrays
carry their fixed element count; exact byte fields, blank padding, products and
pure inverse record views compose nested codecs. Fields have no implicit
alignment. Unsupported application types require an explicit wire view; there
is no reflective guessing or unsupported-type sentinel size.

```cinch
import Text.Binary.Layout as B

-- | A tag followed by an exact binary32 payload, with no implicit padding.
message :: B.BinaryLayout (U16, F32)
message = B.binary-pair B.binary-u16 B.binary-f32
```

`binary-size` observes a descriptor. `binary-encode` allocates complete private
output after validating every nested array/byte-field length. `binary-encode-into`
checks the complete destination first and copies only after private encoding
succeeds, preserving caller output on nested errors and input/output aliasing.
`binary-decode-at` returns a value and consumed bytes; trailing bytes remain
available. Decoded byte fields are fresh snapshots. `binary-append` copies its
prefix. Explicit errors retain required/available counts and invalid offsets.

`binary-read` fills one private frame and copies callback chunks before advancing,
including readers that reuse mutable buffers. Empty EOF differs from partial
truncation. Failures return the advanced reader and preserve upstream errors.
`binary-write` validates before touching the sink, preserves accepted-byte
progress and returns the advanced writer. Offers are copies, so a callback cannot
corrupt privately retained retry bytes. Retry sizes adapt to actual accepted
counts; a one-byte writer does not trigger repeated whole-remainder copying.
Zero-size reads and writes invoke no callback. Borrowed streams are not closed.

Variable integers preserve their complete 64-bit domain, minimal encoding and
zigzag mapping. Buffer decoders distinguish an incomplete ten-byte continuation
prefix from an available eleventh byte proving overflow. Stream readers consume
at most ten bytes and leave subsequent input untouched. They return successful
values only; partial arithmetic values on malformed input are not promised.

## Capability mapping

The pinned `encoding/binary` declaration inventory contains 36 records (including
historical duplicate constant declarations and interface members).

| Reference capability | Concrete public surface |
| --- | --- |
| Little/Big/Native byte order and stable names | `BinaryLittleEndian`, `BinaryBigEndian`, `BinaryNativeEndian`, `binary-order-name` |
| Unsigned 16/32/64-bit reads, puts and appends | `binary-read-u16/u32/u64`, `binary-put-u16/u32/u64`, `binary-append-u16/u32/u64` |
| Aggregate Size/Encode/Decode/Append | `BinaryLayout`, `binary-size`, `binary-encode`, `binary-encode-into`, `binary-decode`, `binary-decode-at`, `binary-append` |
| Fixed arithmetic, float, complex, arrays, records and blank fields | Concrete `binary-*` layouts, `binary-array`, `binary-pair`, `binary-bytes`, `binary-padding`, `binary-map` |
| Aggregate stream Read/Write | `Effect.Binary.binary-read`, `binary-write` |
| Maximum variable-integer lengths | `binary-max-varint16/32/64` |
| Unsigned/signed buffer variable integers and puts/appends | `binary-encode-uvarint/varint`, `binary-put-uvarint/varint`, `binary-append-uvarint/varint`, `binary-decode-uvarint/varint` |
| Unsigned/signed stream variable integers | `binary-read-uvarint/varint`, plus `binary-write-uvarint/varint` |

The order enum and concrete descriptor values replace open interface dictionaries
and mutable reflection destinations. Results replace panic-based short-buffer
behavior. List-backed arrays have a descriptor-declared count and reject length
mismatches. Invalid values fail before allocation, including huge mismatched
array descriptors. These choices are deliberate idiomatic surface differences.

## Complete blocks and exact bits

Private straight-line inline LLVM bitcasts preserve floating signed zeros,
infinities, subnormals and quiet/signaling NaN payloads without floating arithmetic.
Homogeneous arrays pack four 8-bit lanes with SWAR, four 16-bit lanes in one word,
and four 32-bit lanes in a private 128-bit register. SIMD lane shifts/masks convert
byte significance without changing element order. Complete 64-bit pairs use
hardware scalar byte swaps, which already map to one instruction per lane on the
host baseline. Small fringes use exact-width checked access. Composite array
layouts retain the general typed element path; raw byte fields use bulk copies.

All writes remain ordinary checked `Data.Bytes`/`Data.Bytes.Word` operations.
Assembly does not conceal stores or branches. Wide values remain private local
registers; no wide ADT field, closure capture or recursive argument is required.
Native callback sites eta-expand private bitcast primitives to keep their ABI
calls saturated. Exact guards are explicit where the Native proof checker loses
a conjunctive branch fact.

## Independent validation and reproduction

`tooling/go_binary_layout_vectors.go` generates 531 scalar, 396 array, 24 record,
15 full-domain variable-integer and 54 malformed/progress-prefix observations.
All three order values are covered, along with complete-block boundaries and
array fringes. Large arrays retain independently computed full-output digests;
small arrays retain every byte. Concrete shared consumers exercise actual
public layouts rather than merely comparing descriptor names.

`tooling/validate_binary_layout.py` compiles the complete source-distributed
closure using either frozen frontend and runs normal 32 MiB heap/8 MiB stack
and separate 256 MiB poisoned-free modes. `lib_binary_layout` includes these
same consumers, ownership/EOF/short-buffer contracts, an explicit Rust O2 case,
and actual private kernel compilation for WebAssembly, AArch64 and big-endian
PowerPC. Foreign execution is not claimed. Cargo integration remains parent-owned.

`tooling/bench_binary_layout.py` retains a scalar-array source baseline using the
same concrete descriptors and checked element callbacks. The baseline patch is
`binary-scalar-array-baseline.patch`; only the packed array dispatch changes.
The driver measures public encode and encode/decode pipelines, mutates a runtime
head value to prevent repeated-work elimination, consumes all decoded bits, and
checks both an independent exact wire digest and an integer-only checksum.
Build paired artifacts on the functional CPU; measure only inside a coordinated
exclusive benchmark-CPU window. Results retain every alternating sample and all
source/compiler/executable hashes.

## Recorded results

The frozen Rust O2 and Native Stage3 consumers both pass all 1,020 independent
observations in normal and poisoned-free runs, with `leaked=0`. Both pass buffer
bounds/alias/atomicity contracts, nested and complex arrays, exact EOF/progress
contracts, reused callback buffers, and a 64 KiB mutating one-byte writer.
Temporary-owner stress repeats 32 complete encode/decode pipelines over 4097
binary32 elements in each ownership mode. Canonical extraction reports 113
layout declarations and 18 stream declarations, with no missing or legacy docs,
orphans, diagnostics or reference-language prose. WebAssembly and AArch64 kernel
objects compile; the supplied Rust target adds a big-endian profile gate.

[Paired Rust O2 samples](binary-packed-O2-2026-11-16.json) show these median
scalar-baseline/packed ratios. Each condition has five alternating paired
samples, an exact independent full-wire SHA-256 and an integer-only checksum.

| Pipeline | 17 fields | 1024 fields | 16385 fields |
| --- | ---: | ---: | ---: |
| Encode | 1.456× | 1.619× | 1.536× |
| Encode and decode | 1.529× | 1.769× | 1.685× |

CPU0 was exclusively granted for timing; CPU4 was idle. Compiler work continued
on other assigned CPUs, so these are paired same-environment measurements rather
than a whole-machine isolation claim. The unchanged scalar source is reproducible
from the retained patch and hashes. Optimized assembly contains actual SSE lane
shifts/shuffles. Generic descriptor callbacks, tagged natural counters and
checked-store owner/range machinery remain visible. Removing those remaining
costs requires preserving the same checked mutation boundary in further compiler
or runtime work; this change does not conceal stores in a primitive.

The full standard-library initiative remains open. The `lib_binary_layout`
Cargo target is supplied for coordinated integration and was not run in this
lane; the CLI corpus, ownership runs, documentation checks, fixture regeneration,
portable object checks and paired timings are the validation performed here.
