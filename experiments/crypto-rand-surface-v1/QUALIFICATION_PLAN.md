# Handoff plan; no execution performed

All source is static and unqualified. No SDK generator or proposed Cinch module has been compiled/run, and no runtime/archive/timing input was changed. New proposal artifacts must remain below 5 MiB. Subsequent builds require a separate handoff with the ordinary 350 MiB start / 250 MiB reserve and bounded output rules; do not treat the earlier existing-binary exception as authority for a new build.

First perform source-only checks on each proposed module with the explicitly selected current qualified compiler. Preserve every first diagnostic and source epoch. In particular, confirm dependent arithmetic/range obligations, raw-source callback/continuation types and private unsafe primitive acceptance. There is no claim that textual inspection establishes typing. Keep the existing default `Effect.Random` module and its zero-request contract unchanged.

Then build the independent SDK generator using the pinned SDK identity and a captured dry-run plan. Require only the main package to compile from the populated cache; stop before any unbudgeted dependency rebuild. Count new shared-cache payloads and temporary files as output. The generator imports the actual public APIs and `math/big` oracle, not a copied implementation. Capture exact SDK/version/configuration/source/executable identities, stdout/stderr, exit status and all selected compatibility/policy settings.

Modes are separate processes:

| Mode | Purpose | Expected data records, excluding metadata |
| --- | --- | ---: |
| `int-model` | Arbitrary bounds, high masks, rejection and reader edge cases | 1823 |
| `text-model` | Actual current Text with a scoped model Reader plus independent alphabet/lane oracle | 4393 |
| `natural-model` | SDK mathematical big-integer byte-import oracle | 729 |
| `prime-compat` | Explicit deprecated-reader branch, known-prime candidates, optional callback probe captured | 9 |
| `secure-default` | Actual default Read/Text/Int/Prime, including ignored-reader and invalid-size/policy observations | 61 |
| `read-failure` | Expected irrecoverable public Read error, isolated child, full failure diagnostics retained | 0 after metadata |

These are source-derived expected counts, not executed fixture results. Deterministic modes must remain labeled models; they never qualify or benchmark a substituted default provider. `prime-compat` requires `GODEBUG=cryptocustomrand=1`; `secure-default` refuses that setting. Run supported policy cases separately and record startup/policy failures honestly. `read-failure` intentionally exits through the SDK's fatal boundary; retain its complete stdout/stderr and nonzero status rather than treating it as a successful real-entropy read.

Use captured integer rows to check exact values, byte consumption, rejection, no-read bound one, named invalid bounds and error progress in the proposed adapters. Preserve the simpler ByteReader contract difference; use the raw BufferSource overload for zero/no-status and simultaneous data/status cases. Validate original borrowed buffers and source capabilities after success, rejection and failure. Check signed/big-int cases beyond machine limits without narrowing through I64.

Validate natural import for empty input, leading zeros, full bit63, 7/8/9- and 15/16/17-byte fringes, arbitrary masks and wide values. Compare every vector/SWAR text lane, all 256 input byte values, sixteen lane placements and lengths 0–40 against independent alphabet rows. Actual secure tokens must use the real source and satisfy length/alphabet/ownership properties without predicting or requiring different bytes. A current SDK length of26 is pinned fixture behavior; the public contract allows future growth.

For candidate shaping, check all bit residues, small widths, wrong byte lengths, top two bits, odd parity and caller immutability. Those checks do not qualify primality. Keep Prime blocked until the dependencies/gates in `PRIME_API.md` exist.

After source gates, inspect actual LLVM and assembly: vector/SWAR operations and bounded loads/stores must remain, both owners must outlive private pointer operations, and no store may disappear because a strict kernel result was unused. Check little/big wire order with actual host execution and available portable objects, labeling object-only evidence honestly. Run normal and poison/leak checks through actual public consumers before any package inventory promotion. The pure default provider is independently maintained; do not relabel older provider archives or benchmarks as qualification of these new modules.
