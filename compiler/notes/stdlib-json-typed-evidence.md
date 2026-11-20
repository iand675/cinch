# Typed JSON records and binary64 checkpoint

This partial package increment adds `Text.Json.Codec`. A `JsonType a` contains explicit default, zero/empty classification, fragment encoding and immutable update capabilities. `JsonDecoded a` preserves the resulting application value together with its first error and a separate fatal-continuation flag. Record fields combine typed projection/replacement callbacks, JSON names, omission rules, quoted scalars, embedding depth, naming priority and exact/fold matching. Existing `Text.Json`, `Data.Json`, `Text.Codec` and the previously validated `Text.Json.Value` source are unchanged.

The new module SHA-256 is `ae4470f5497bb4558ed11df1bb534d7777b041e02ab0898015172971813eb138`; the source-backed value dependency remains `f52e7fe9d0ea33f455b7876484a8fa028f6fb32695c2a1d232b65a0625dc5793`. All 203 module declarations, private helpers and constructors have canonical `-- |` comments, with no documentation diagnostics/orphans or foreign-library framing in library comments.

Implemented descriptors cover booleans, scalar strings, signed64 integers, finite binary64 numbers, validated exact numeric spellings, raw fragments, nullable values, immutable lists, nullable slices, fixed arrays, custom fragments and application records. Binary64 marshaling preserves negative zero, uses shortest round-trip coefficients and selects fixed/scientific notation at the JSON thresholds. Nonfinite values return structured value errors. Complete syntax validation precedes typed updates; recoverable field/range errors retain their original source offsets while later fields continue. Custom decoder callbacks explicitly choose recoverable or fatal outcomes.

Record compilation resolves shallow/named embedding dominance in a persistent balanced AVL tree. Exact lookup is logarithmic; Unicode-fold fallback visits eligible fields in declaration order after exact lookup fails. The load-only inline LLVM kernel compares complete 16-byte ASCII blocks, folding uppercase lanes and leaving Unicode/delimiter boundaries to scalar handling. Both backing owners remain live across the raw loads. The kernel performs no stores. The actual production primitive compiles into wasm32 and AArch64 objects; this is a compilation claim, not foreign execution. The prepared Rust integration target additionally checks the big-endian PowerPC object path.

The pinned independent Go 1.27.4 reference uses its active default JSONv2 facade, whose exact selected source identities are recorded separately. Reference reflection is used only by the fixture generator to observe a runtime-selected record tag. Cinch record serialization uses explicit typed descriptors and does not claim runtime reflection or source-language method sets.

| Independent observations | Rust O2 | Native wrapper O0 |
| --- | --- | --- |
| Primitive/number/raw/list/fixed-array/application-record updates | 56 | 56 |
| Binary64 shortest formatting, payload boundaries and typed updates | 163 | 163 |
| Field-name SIMD boundaries, every 32-byte mismatch position and Unicode folds | 104 | 104 |
| Additional policy/custom/embedding/large-record/64 KiB controls | 14 | 14 |
| Ordinary 32 MiB heap / 8 MiB consumer stack | zero live/leaks | zero live/leaks |
| Poisoned 256 MiB heap / 8 MiB consumer stack | zero live/leaks | zero live/leaks |

An additional Rust O0 normal/poison pair runs the same 14 controls, including the 1000-field weighted lookup test and 64 KiB typed string pipeline. `../evidence/stdlib/stdlib-json-typed-evidence.json` verifies every source/consumer identity across the 18 same-source memory runs. The compiler host main stack is explicitly 64 MiB; the consumer limits above are unchanged. The copied runtime archives are verified before and after each gate.

Rust uses repaired compiler `cad70e533fcab8305fe6e79ae95f135f61252338c6a5fd5e5358e8f27563e3f4` and observed host runtime `c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac`. Native uses gated Stage3 `9e2a64e0e519fc908f17cd7329b72be90f1161513084bfe2e5e3ebbd4a846f90` with matched runtime `12945b719660f1fe3a9b450c7da03161da0b93498d7a504bcb2b9e069de5bf90`. Its functional wrapper supplies no clang optimization flag, so these Native runs are default O0. No seed/runtime substitution is combined into a single epoch.

The generated source consumers and reference data are reproducible with `go_json_codec_vectors.go` (default, `--float-codecs`, `--field-codecs`) and `generate_json_codec_cases.py`. `validate_json_codecs.py` freezes each complete source closure before compiling and checks normal/poison memory results. `validate_json_field_kernel.py` freezes and compiles the actual production primitive. `record_json_typed_checkpoint.py` checks the final matrix and documentation identities. `lib_json_codec.rs` provides six parent-owned Rust integration tests; no Cargo build was run in this lane.

The 12 compatibility choices are explicit immutable policy values; applicable record/scalar/sequence/array branches are implemented and custom descriptors inspect the same options. This does not claim all behavior of separate successor packages. Map/key descriptors, binary bytes, unsigned/narrow integers, binary32, dynamic values, general numeric string-cast outcomes, complete containing-record error metadata and real incremental effectful Decoder/Encoder adapters remain open. The 92-record audit and per-record plan remain partial. Public-pipeline comparisons and further classifier lowering review are a separate performance checkpoint; no JSON speedup is claimed here.
