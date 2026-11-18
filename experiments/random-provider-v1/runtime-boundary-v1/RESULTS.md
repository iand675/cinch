# Runtime boundary results

Candidate source commit `843ddb8d` compiled directly to one stripped runtime test executable with rustc 1.100.0. Every prepared runtime source matched `prepared-source-manifest.json` before compilation. `run-1/receipt.json` records the complete command, compiler/source/executable hashes, explicit manifest/TMPDIR environment, statuses, and monitored resource usage. The executable lists 169 tests; only the agreed relevant selections ran.

The unit completed **37 successful test executions**, with zero failures:

| Selection | Passed |
| --- | ---: |
| New public boundary, allocation stats and leak checking | 3 |
| New public boundary, allocation stats, leak checking and RC poisoning | 3 |
| Integrated entropy exact-fill loop | 6 |
| Syscall layer audit and controls | 4 |
| Byte buffer tests | 5 |
| ABI tests, including payload release | 8 |
| Platform behavior, excluding archive export inspection | 6 |
| Piped process behavior | 2 |

An initial progress message incorrectly summed these as 40; it was immediately corrected to 37. The per-selection counts and raw receipt were always correct. This result explicitly preserves that arithmetic correction.

Both public-boundary processes report three `cinch-leak: live=0 immortal=0 leaked=0` lines. Each process allocated exactly 16 header blocks and freed all 16, corresponding to two owned results at each of eight requested lengths, including zero and 1 MiB. The test checks `KIND_BYTES`, `RC_FRESH`, exact length, independent mutable backing storage, raw deallocation and ordinary reference-count release. Invalid negative requests return exact `-INVALID_ARGUMENT` without header allocation. Linux errno mapping checks are deterministic mapping tests, not natural kernel error observations.

The compiler's eleven warnings are retained in `run-1/compile.stderr`; they concern pre-existing unused functions/constants/fields and unnecessary unsafe blocks. No warnings or failures were removed from the record. Existing broader tests were run without global leak assertions because some retain their own test headers. Their passing results do not expand the new module's ownership claim.

The archive-export test was explicitly excluded because this unit produced no archive. No old D91 archive was placed beside the executable. Staticlib symbol inspection, public Cinch consumer linkage, injected provider branches, cross-target execution and matched performance remain later gates.

Compile wall time was 4.427 seconds, recorded only as process bookkeeping. The observed output-plus-scratch peak was 10,516,021 bytes, and final output was 2,860,564 bytes at receipt capture. Free space started at 384,958,464 bytes, reached a minimum of 375,005,184, and ended at 382,046,208. The 350 MiB start, 250 MiB reserve, 30 MiB output, and 60-second compiler guards were respected. CPU 4 was returned after completion.

The test executable remains plain locally with SHA256 `6e9e59f9d8f9dba9b09af7783b3947cc9b18ed6895ee6761fa7aabb902425389`. Root's platform source rehashed as F4F0 (`f4f0e6519e526590ae2f779b9ba5b81b3c5cd685f14abf901eeeed7e9f60477b`) and the frozen runtime rehashed as D91 (`d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`), unchanged.
