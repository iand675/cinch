# JSON machine numbers and binary values

This partial increment extends `Text.Json.Codec` with all signed/unsigned machine integer widths and binary byte values. It preserves the existing legacy JSON modules and the exact `Text.Json.Value` source. The new Codec SHA-256 is `7f976b62e406a8e9ea517f850da5ef226bf97388b1ed7dd2acb017af9637a1a1`; all 252 declarations, including private helpers and primitives, have canonical documentation with zero diagnostics/orphans and no foreign-library framing in library comments.

Unsigned64 decoding uses complete eight-digit SWAR blocks, scalar fringes and explicit pre-multiplication overflow checks. Signed/narrow descriptors use exact widening and checked narrowing; target-sized integers use the target's bounds. Encoding the full unsigned64 range avoids signed or floating intermediates. The private machine arithmetic has no wrapping-arithmetic proof refinements. Public indexes and byte intervals remain mathematical naturals, and all buffer reads/writes/copies are checked.

Binary slices distinguish null from a present empty buffer. Fixed byte buffers use numeric arrays by default, with explicit binary-text and decoded-length policies. Encoding/decoding delegates to the existing configurable SIMD Base64 module. The text compatibility option controls embedded CR/LF independently of unused padding bits: ordinary padded input and its alphabet are always required, while nonzero unused bits remain accepted. A separate load-free SWAR predicate skips complete line-break-free words; the byte-zero classifier also reads complete words before its final fringe.

The independent active reference exposed a partial binary-error behavior. With prior bytes `[7,8]`, malformed `"AA==="` confirms one decoded zero byte before reporting trailing garbage; malformed `"AAAA="` requires a larger reservation and leaves that prior value unchanged. The native immutable adaptation returns a fresh prior snapshot overlaid with the confirmed prefix only when its reservation fits the owned logical buffer. It never mutates original aliases, infers hidden spare capacity or reproduces speculative writes beyond the confirmed prefix. No-progress failures retain the supplied application value. This rule is explicit rather than claiming arbitrary source-runtime slice-backing behavior.

| Independent observations per frontend | Count |
| --- | ---: |
| Existing typed updates and record continuation | 56 |
| Binary64 payload formatting and typed updates | 163 |
| Field matching across SIMD/Unicode boundaries | 104 |
| All signed/unsigned machine-width updates | 390 |
| Binary text, partial arrays and fixed byte arrays | 86 |
| Additional prior record/policy/large controls | 14 groups |
| Binary alias/policy/word-fringe/large controls | 10 groups |

The full matrix contains 30 same-source Codec memory runs: Rust O2 and Native wrapper-default O0 for every family, plus an extra Rust O0 large-control pair. Every consumer uses ordinary 32 MiB heap / 8 MiB stack and separate poisoned 256 MiB heap / 8 MiB stack, with zero live/leaked allocations. The binary controls include all 25 inserted line-break positions across word/fringe boundaries, fixed-array zero-fill and refusal progress, prior-alias independence and a 64 KiB public typed binary round trip. Existing 1000-field balanced-record and 64 KiB string controls are revalidated against this combined source.

The actual production `decimal-eight` and `binary-linebreak-eight` primitives additionally pass 2048 independent lane-byte mutations at Rust O0 and O2 in both memory modes (four isolated memory runs), with the consumer OS main-stack limit also set to 8 MiB. Their actual source compiles into wasm32 and AArch64 objects; foreign execution is not claimed. The factored checked-in control body reconstructs the exact qualified consumer hash. No new runtime intrinsic or hidden LLVM store is introduced.

Rust seed/runtime identities remain `cad70e533fcab8305fe6e79ae95f135f61252338c6a5fd5e5358e8f27563e3f4` / `c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac`. Native remains gated Stage3 `9e2a64e0e519fc908f17cd7329b72be90f1161513084bfe2e5e3ebbd4a846f90` / matched runtime `12945b719660f1fe3a9b450c7da03161da0b93498d7a504bcb2b9e069de5bf90`. No epoch is silently substituted. The compiler host alone receives the established 64 MiB main-stack allowance; compiled consumer limits are unchanged. `record_json_numeric_binary_checkpoint.py` verifies source, consumer, seed/runtime, documentation and matrix identities.

The expanded `lib_json_codec.rs` target contains ten parent-owned integration tests; no Cargo build is run in this lane. Reference fixtures use the pinned active Go 1.27.4 facade, retaining exact errors and partial results. The failed first binary consumer/source remains separately preserved under `build/json-codec-binary-rust-v1`; final results use the corrected combined source.

Binary32 exact conversion, typed maps/keys, dynamic values, raw numeric string-cast outcomes, complete containing-record metadata, legacy codec bridges, real incremental Effect.Json adapters and final 92-record review remain open. The per-record capability plan stays partial. Public pipeline comparisons remain a separate performance checkpoint; no JSON speedup is claimed by these functional gates.
