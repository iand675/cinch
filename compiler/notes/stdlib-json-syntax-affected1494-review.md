# Current syntax-partial regression checkpoint

The candidate from 412ce5d1 passes all 1,494 unchanged historical observations, the retained-invalid case and four owner controls on frozen Rust 855/c4. Seven source-first units passed fourteen normal/poison memory processes with empty stdout and zero live/leaked blocks. These add to the earlier 902 syntax observations and 29 focused controls; none of those completed gates or their oracle generations was repeated.

All three candidate JSON modules and all eleven new fixtures retain exactly the bytes from 412ce5d1. Every original observation remains unchanged, including twelve SDK error-rendering panic outcomes. The strict verifier checks source and fixture hashes, actual runtime results, all seven compressed archives and decompressed member hashes/sizes/modes, 413 historical failure files and the three newer syntax failure ELF/IR pairs. Failed inputs and outputs remain plain.

The largest fresh unit used 22,023,821 bytes. The lowest observed free space was 493,731,840 bytes; each phase separately enforced the 350 MiB start and 250 MiB reserve guards. Success archives recovered 88,029,549 bytes after full verification and durable recovery indices before unlink.

Root integration uses `compiler/evidence/stdlib/stdlib-json-syntax-partial-regression-integration-manifest.json`: replace only the three listed JSON modules, add the eleven listed fixture files, and append the eleven functions from the test fragment to the existing `lib_json_codec` target. Preserve all existing targets, the source closure, panic-retention guards and Root-owned dependencies. The resulting target has 45 tests. Run the fresh parent target using Root's actual current producer and matched runtime, then update and validate the generated reference through Root's existing workflow. Child 855/c4 receipts remain a distinct epoch.

The parent gate and production promotion remain pending. Additional semantic corners, Effect.Json with actual streaming IO causes and advanced handles, Native qualification, public pipeline measurements and full 92-record reconciliation remain open.
