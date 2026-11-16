# Original Native Map fixture lowering refusal

`Bundle.cinch.gz` decompresses to the exact original 817,210-byte multi-module consumer whose 9e compiler run refused the computed polymorphic empty-map binding at line 26488. `receipt.json` identifies every source module and the matched compiler/runtime epoch; the diagnostic is retained verbatim. No consumer ran in that failed build. The ordinary Map control adds its intended `JsonMap String I64` annotation and passes.

The two smaller checked-in standalone candidates both emit successfully under the final 2f077 compiler. They are **not** a confirmed reproduction, and do not establish that the original full-source problem was fixed. This full original bundle has not been compared with 2f077.

To inspect the exact source, decompress with Python gzip or `gzip -dc Bundle.cinch.gz > /tmp/json-map-original.cinch`. Diagnostic comparison must retain an explicit compiler epoch and its matching interfaces/objects/archive; do not substitute toolchains within an existing receipt.
