# Isolated UUID candidate

These sources are byte-identical to the qualified UUID candidate. The maintained
`compiler/tests/lib_uuid.rs` test selects them explicitly and writes project-local
`Data.Uuid` and `Effect.Uuid` modules. Integration therefore needs no prelude edit.
The committed SDK vectors are unchanged, and tests require no reference SDK.

`bootstrap/evidence/stdlib/stdlib-uuid-integration-manifest.json` lists the exact additive files
and source hashes. This remains a candidate pending benchmark review. Runtime
and portable failures retain their complete generated projects during panic
unwinding; successful projects are cleaned up. Existing proof receipts retain
the original source paths and compiler/runtime identities.
