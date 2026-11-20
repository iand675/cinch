# Owned byte construction

`Data.Bytes.Build` adds `bytes-clone`, count-first `bytes-repeat`, `bytes-concat`
and `bytes-join`. Every result owns fresh mutable storage. Inputs may contain
arbitrary bytes and are never decoded as text. Empty list elements still produce
their adjacent separators, and an empty separator uses the direct concat path.

Clone uses one checked bulk copy. Repeat seeds a newly allocated output then
copies initialized prefixes by doubling, with an exact final fringe. Concat and
join use tail count/copy passes with one output allocation. All writes remain
ordinary checked `bytes-copy-range` operations; their runtime implementation
already provides accelerated bulk copying. The five traversal workers compile
at O0 to entry backedges without recursive calls.

All 243 repeat and 360 join observations match the independent pinned SDK
reference. The cases include malformed UTF-8, zero and high-bit bytes, empty
pieces, deep lists and doubling/fringe lengths through 65,537. Frozen Rust O0,
Rust O2 and qualified Native all pass the complete corpus, mutable alias checks
and 100 temporary-owner pipelines. All 462 final normal/poison profile processes
finish with zero leaks using 32 MiB/8 MiB and 256 MiB/8 MiB respectively.
The eight-row process batches bound the poison runtime's retained freed blocks;
the heap limits are unchanged. Compiler shell/Rust worker stacks use 64 MiB,
separately from the compiled programs' explicit 8 MiB stack limit.

All nine implementation declarations have canonical documentation, parsed
without diagnostics or orphans. Four Rust tests are supplied; Cargo has not run
in this lane. Exact source, compiler, fixture, executable and IR identities are
in `bootstrap/evidence/stdlib/stdlib-byte-build-results.json`. This source-only module does not
claim to validate the separate changed embedded String/Bytes constructors.
Paired public-pipeline benchmarks and assembly review remain open.

Reproduce using `tooling/validate_byte_build.py` with the frozen Rust compiler
and `--opt-level 0` or `2`, or the qualified Native wrapper and its
`--native-interfaces` manifest. Independent reference generation is in
`tooling/go_byte_build_vectors.go`; the committed observations also match a
separate Python exact-byte implementation.
