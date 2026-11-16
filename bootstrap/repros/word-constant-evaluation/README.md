# Scalar evaluation regression inputs

These compressed sources and emitted LLVM preserve the successive independent
failures that exposed the repairs. Their uncompressed hashes, sizes and exits
are in `before-results.json`. The original frozen compiler/runtime identities
are recorded separately from the later repaired-Core consumers.

The first fixture reports aggregate failure. The later fixtures return the
one-based first failing assertion:8 identifies unsigned saturating negation,
and24 identifies a checked conversion across the signed64-bit boundary.
Each generated process stayed within32MiB heap/8MiB stack and reported no
live or leaked owners. Sources differ between epochs and are retained exactly.

The maintained regression reports exact constant/emitted results and staged
negation independently. Successful validation is recorded in the follow-up
checkpoint; these preserved failures do not imply any Native compiler fix.
