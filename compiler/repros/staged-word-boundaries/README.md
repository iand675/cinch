# Staged machine scalar boundary reproductions

The six independent public compiler consumers in this directory were compiled
before the staging repair with the immutable Rust compiler identified in
`baseline.json`. Four builds panic in the host evaluator: signed bounds,
signed saturating arithmetic, full-width unsigned multiplication and a
zero-count rotation. Unicode ordering above U+00FF and logical unsigned
right shift compile successfully but return the wrong generated result.
The exact stderr, source hashes, commands and exit statuses are retained.
All six are compile-time semantics faults; these observations do not measure
runtime library throughput or qualify another compiler family.

The repair resolves pointer-sized primitives against the selected target,
keeps complete 32-bit character scalars, uses signed/unsigned 128-bit
intermediates for their respective saturating arithmetic, avoids host overflow
when constructing signed bounds, and handles unsigned shifts and zero rotations
according to their scalar contracts. Checked arithmetic that was explicitly
unsupported by the staging evaluator remains outside this checkpoint.

Independent Rust scalar operations test the actual staging primitive interpreter
across eight signed and unsigned scalar types and both target word sizes. A combined public consumer
also exercises the actual generate/quote path at O0 and O2 with ordinary and
poisoned-free profiles. The original failed observations remain unchanged;
post-repair results are recorded in
`compiler/evidence/stdlib/stdlib-staged-word-parent-results.json`: all 380 parent tests passed.
