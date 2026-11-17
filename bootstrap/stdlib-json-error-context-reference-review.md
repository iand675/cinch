# Independent JSON error/context observations

Pinned reference observer: `tooling/go_json_error_context_vectors.go`; receipt: `stdlib-json-error-context-reference-v1.json`; vectors: `compiler/tests/fixtures/json-codec-error-context-vectors.json`. The completed run produced1494 observations in0.612s/738646 bytes. This is independent reference evidence, not Cinch equivalence or a performance result. Existing1160/v2 1464 observations and all published receipts remain unchanged.

All1152 quoted-field rows are retained:990 successes and162 semantic refusals. Four generated288-row consumers cover ordinary bool, addressable-only bool method, string and integer. The pointer consumer separately preserves the original published pending row with its exact application type label. These consumers are prepared, not runtime-qualified.

A nonaddressable pointer-only method falls back to bool. With legacy stringification and error reporting both disabled, its quoted field fails before invoking the callback. The actual partial output stops after the quoted member name. The reported position counts the upcoming colon/spacing:

| Format | Actual output | Logical offset |
| --- | --- | --- |
| Compact | `{"value"` |9|
| Two-space indent with tab-space prefix | opening brace, newline, tab plus three spaces, `"value"` |15|
| Multiline with empty indentation | opening brace, newline, `"value"` |11|

Addressable methods control their own representation and bypass the invalid ordinary tag. HTML escaping changes preceding byte lengths, so positions count encoded bytes rather than application characters. Nested objects and sequences prepend escaped JSON pointer components.

The288 method-error rows preserve144 legacy marshal wrappers and144 semantic outcomes. A complete JSON callback's returned semantic offset and pointer compose with containing context. A text callback's existing semantic error retains its original offset and pointer even inside records, arrays or maps. Successful method selection and error context must be tested separately.

The54 update rows include24 legacy type errors,24 semantic errors and6 successes. With legacy reporting disabled, a builtin mismatch identifies the source token's beginning and lexical kind, stops before later fields and has no underlying conversion cause. Legacy reporting identifies a scalar's token end and continues later fields. The record metadata identifies the outer application record; its escaped field path is separate from JSON names. A returned JSON semantic error composes source start/pointer with its original relative context; a returned text semantic error retains its existing context. Both retain the method's advanced value.

Two first observer failures are preserved under `repros/json-error-context-reference-v1`: non-whitespace indentation prefix option refusal, and an SDK legacy error-rendering panic when a method returns semantic metadata with a nil type. The corrected observer records all12 rendering-panic outcomes, nil-type markers and all other structured fields instead of dropping these cases. No requirement to reproduce that SDK panic in a library renderer is inferred.

Full92 remains open: this stage does not complete builtin/read/record error adaptation, nullable value-sensitive quoted-tag checks, method nil/interface/key semantics, reporting-disabled syntax partial mutation, bounded Effect.Json or public pipeline benchmarks.
