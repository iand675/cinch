# Narrow shared decoder promotion proposal

Against committed Root `820d234b784f459eaf8e99dc82871105c8731086`, the only required source import is:

`experiments/multipart-optimization-v1/sources/Text.QuotedPrintable.cinch` → `compiler/prelude/Text/QuotedPrintable.cinch`

The exact candidate is SHA256 `ae2def9c5e872f55fddb1fb908801d71263bd499e4882f6dc9bd205f2b816e00`; the prior Root source is `cfc475a514b56bc919e4aa3de4e97ed19d09816e25ccdb530cebaee90bf8d6cb`. This replaces private Nat/Option hexadecimal work with a bounded unpacked scalar classifier. Invalid pairs still reach the original recovery/error path. It changes no public declaration, constructor scheme, continuation ownership, Stream operation or Multipart API.

`import-manifest.json` records all 31 identical exported declarations, all six byte-identical data blocks, Root dependency hashes and existing fixture hashes. The three other nonbuiltin source dependencies used by the direct corpus already match Root exactly. No historical experiment, archive tool, benchmark binary or broad Multipart candidate is needed for the source import.

The complete inherited independent corpus remains unchanged: 1,368 values, 41 reader traces, 60 writer traces and six ownership controls, totaling 1,475 observations. Qualified Rust330/d91 emitted 37 consumers at O2; the exact accepted IR was separately relinked at O0. Both optimizations pass normal32MiB and poison256MiB heaps with8MiB runtime stacks, all148 processes zero leaks. Compiler host stacks are64MiB. The committed receipts are under `experiments/multipart-optimization-v1/`, anchored in commit `0b900f67`; the manifest gives their full hashes. No new Native admission or current parent-compiler gate is implied.

The actual new LLVM primitive passes all256 byte inputs and65,536 digit pairs against an independent literal-table C oracle. Real AArch64 and wasm32 objects pass compilation; foreign execution is not claimed. The maintained tool and C fixture are two optional narrow imports if Root wants to retain that independent gate. Every one of the76 declarations in the selected text module has canonical documentation, with zero diagnostics/orphans.

Root's existing `lib_quoted_printable` target needs no source override once the selected production file is imported. Its four runtime groups check malformed first/second hex digits, valid uppercase/numeric decoding, exact chunk/error progress, decoder forks and input/output mutation, plus writer controls. They do not contain lowercase valid pairs. Root can add the already committed `values-0256` and `values-0896` groups for explicit low/high lowercase-nibble CI coverage without changing any fixtures. The complete standalone corpus and exhaustive actual primitive gate already cover both positions.

Root owns the final current-compiler target, failed-artifact retention, documentation integration and explicit production selection. This proposal makes no Root edit and does not select the candidate.
