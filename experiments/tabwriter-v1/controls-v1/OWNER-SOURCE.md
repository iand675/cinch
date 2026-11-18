# Ownership source checkpoints

The two owned entry sources and adjacent exact reports below pass parsing, type checking and proofs on the explicit 330/d91 tuple. Original Main/library hashes were verified unchanged before and after each isolated check. These are source observations only: no LLVM emission, executable, runtime ownership result or package promotion is claimed.

- `error-owner-source-v1`: 4.05 seconds, 2,514,454 source-check bytes. The three groups retain exact floating error payloads (NaN payload and negative zero), preserve an original error ahead of a malformed signed report, and return function errors owning bytes from 128 local writer factories.
- `offer-owner-source-v1`: 3.73 seconds, 2,502,431 source-check bytes. Five groups specify observable independent input/offer mutations, valid text output, lossless raw bytes in an invalid-text result, 10,000 columns under fixed stack limits, and a complete 64 KiB table compared with an independent repeated-row byte expectation.

Both checks used Root's scoped source-only 320 MiB start / 250 MiB reserve / 5 MiB output / 15-second deadline. The ordinary 350 MiB emission/runtime start threshold remains unchanged. Bodies of the actual tabwriter library remain identical to checkpoint `3f5baacc`.

All runtime gates, the separately prepared layer/stream controls, production classifier execution/portable objects and public performance review remain open. The prepared layer fixture clears its own temporary callback back edge explicitly; it makes no cycle-collection claim. The existing buffered-library callback-cycle failure and production collector hold are unchanged.
