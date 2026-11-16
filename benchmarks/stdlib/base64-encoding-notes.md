# Configurable byte Base64

`Text.Base64.Encoding` supplies standard, URL and custom byte alphabets,
optional one-byte padding, strict unused-bit validation, exact encoded lengths,
decoded capacity bounds, copied-prefix append, checked caller buffers and
persistent encoder/decoder continuations. `Effect.Base64` supplies explicitly
finished writers and decoding readers. Existing `Text.Base64` text conveniences
retain their established behavior.

| Reference capability | Cinch mapping |
| --- | --- |
| Encoding / NewEncoding | Opaque `Base64Encoding`; `base64-encoding` validates and copies 64 distinct non-newline bytes into private alphabet/inverse tables, returning named configuration errors |
| StdEncoding / URLEncoding / RawStdEncoding / RawURLEncoding | Four public immutable configurations |
| StdPadding / NoPadding, including duplicate constant records | `Some 61` from the standard configuration's padding getter; `None` selects no padding |
| WithPadding / Strict | Checked `base64-with-padding` and persistent `base64-strict`, sharing immutable tables |
| EncodedLen / DecodedLen | Exact `base64-encoded-length` and conservative `base64-decoded-length` capacity bounds |
| Encode / EncodeToString / AppendEncode | Fresh byte encoding, optional UTF-8 text conversion, copied-prefix append and checked offset stores |
| Decode / DecodeString / AppendDecode | Atomic convenience plus progress-preserving byte/text, append and caller-buffer decoding |
| CorruptInputError / Error | Named original-byte errors and diagnostic/offset accessors |
| NewEncoder / Write / Close | `Base64Writer`, accepted-byte writes, explicit `base64-finish` and advanced-sink inspection |
| NewDecoder / Read | `base64-reader` follows the byte-reader protocol, drains complete output before errors and keeps failures sticky |

All 24 recorded declarations and the returned stream capabilities have
functional mappings. Configuration errors use `Result`; padding policy uses
`Option Nat`. Encoded custom bytes can be invalid UTF-8, so text encoding returns
`Option String` while byte APIs preserve every output byte. Alphabet construction
permits an equals-sign symbol; callers must change or disable padding before
requiring an invertible final short quantum for that alphabet.

Input diagnostics count original bytes, including ignored CR/LF. A strict-bit
error identifies the final alphabet symbol with nonzero unused low bits.
Missing padding identifies the first symbol of the unfinished quantum. These
module-owned positions and messages deliberately avoid incidental reference
offset behavior around incorrect pads and intervening newlines. Caller-buffer
decoding checks its maximum capacity bound before mutation, including when
actual padding/newline removal would make the output shorter.

## SIMD and ownership

Six complete input bytes become eight six-bit SIMD lanes. Fixed mapping and
classification use lane comparisons/selections; custom mapping performs eight
exact private table loads. Decoding compacts eight validated lanes into six
bytes and returns a separate invalid-lane mask. Every endian-sensitive bitcast
has an explicitly normalized counterpart. Six-byte reads use two overlapping
four-byte loads, confined to the complete six-byte interval; decoding reads
exactly eight bytes. Output stores retain the checked byte/word layer.

Lookup owners stay alive around raw loads. Custom alphabets and inverse maps
never alias caller buffers, and returned alphabet buffers are copied. Encoder
states retain at most two copied bytes; decoder states retain packed values.
Reader surplus is private and returned chunks are copied. Finishing returns
the advanced borrowed sink without flushing or closing it.

## Validation

The independent fixture file records nine configurations, 333 valid wire cases,
1125 malformed-input cases, 32 strict-bit cases and six invalid alphabets.
Short wires are compared in full; long wires have independent every-byte FNV
fingerprints. Encoder splits are exhaustive through 32 input bytes, decoder
splits through 32 encoded bytes, with selected larger boundaries through 257
input bytes. Larger whole-buffer cases reach 64 KiB.

Both frontends pass all 22 offline fixture groups under normal 32 MiB heap and
8 MiB stack limits, separately from a 256 MiB poisoned-free quarantine. Every
run finishes with zero leaks. Strengthened constructor checks rerun through both
frontends. Rust's default O0 driver also passes the complete 37-case standard
group; actual kernels compile to WebAssembly and AArch64 objects. The integration
Rust target additionally contains a big-endian PowerPC object check; its result
is recorded separately when the parent lane runs that target. Foreign execution
is not claimed.

Canonical documentation extraction reports no diagnostics or orphan comments
in either new module. Kernel generation is reproducible, and three generator/
offline-projection unit tests pass. Tests require Python for offline fixture
projection, but require no reference SDK at execution time.

## Measured public pipelines

[Final samples](base64-encoding-O2-2026-11-16.json) cover eight pipelines at
32, 1024, 16384 and 65536 bytes. All 32 observable checksums match the
independent reference. The input/configuration is prepared before timing and
mutated between iterations. Stream pipelines include their owned chunks,
adapter construction, finalization and output collection.

The first decoder allocated a state constructor for every eight-byte block.
The final private loop carries scalar fields and constructs state only on
return. [Equal-path allocation probes](base64-fields-space-2026-11-16.json)
reduce a complete 64 KiB standard decoding process from 11076 allocations and
1227802 cumulative bytes to 147 allocations and 353482 bytes. Custom decoding
falls from 11081 to 152 allocations. Peak live bytes remain 219087/219407;
fresh output buffers determine that peak. The encode control remains identical
at 131 allocations and 156557 bytes. These totals include startup and common
input/configuration preparation, and use ordinary reclamation rather than
poison quarantine.

The [alternating comparison](interleaved-base64-fields-O2-2026-11-16.json)
measures about 1.89x for standard and 1.84x for custom 64 KiB decoding, with
1.74x/1.78x at 32 bytes. Encode controls differ by at most 2.9%, and preserved
text-decode controls by at most 3.8%. Compiler, driver, runtime and other library
source identities match. The measured improvement removes a decoder-state allocation from each complete
block; SIMD kernels are unchanged. Final code still calls checked word/byte output
helpers and remains slower than the reference public pipelines; no throughput
or allocation parity is claimed.

The [baseline patch](base64-decoder-state-baseline.patch) reconstructs the
measured state-per-block source from the final module. Apply it to a temporary
copy, verify the hash against the [baseline report](base64-before-fields-O2-2026-11-16.json),
then compile with `bench_base64_encoding.py --source-override Text.Base64.Encoding=PATH`.
Frozen artifacts remain at paths recorded in each report.

One unrelated Native lowering issue appeared in the every-byte test fingerprint:
nested raw arithmetic in the final recursive argument created partial
applications instead of a tail jump. Sequencing those values into local bindings
restores bounded execution. The compiler lane has the original and repaired
minimal probes; this library's field loop uses the same explicit sequencing.

[Validation evidence](../../bootstrap/stdlib-base64-encoding-validation-results.json)
records exact source/compiler identities, independent results and integration
checks still delegated to the parent lane. The full standard-library goal
remains open.
