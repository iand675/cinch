# Multipart source checkpoint

The complete 37-record mapping is implemented in `Text.Multipart` and
`Effect.Multipart`. This is a tested source increment; package qualification
remains open. The machine-readable mapping and exact evidence are in
`multipart-surface-plan.json` and `multipart-source-checkpoint.json`.

The pure layer preserves arbitrary wire bytes, exact multi-value headers,
boundary rules and disposition escaping. Checked 16-byte SIMD matching and
8-byte prefix comparisons operate only inside proved spans; scalar fringes
handle the remaining bytes. Physical and folded headers accumulate fragments
before copying rather than repeatedly copying their complete growing prefix.

The effect layer uses explicit shared sessions. Retained aliases observe the
same cursor, part completion and editable metadata. Reads return a fresh prefix
and an independent terminal/error status. Writes preserve actual single-call
short-write progress, the latest body error and repeatable Close behavior.
Entropy and scratch storage are caller capabilities, including arbitrary typed
errors, keys, handles and advanced backend state. Complete File operations and
mutable Form/FileHeader fields are available. Form and error are independently
optional because deferred scratch Close can fail after a form was produced;
failed cleanup actions retain explicit retry ownership.

The independent pinned SDK corpus contains 264 boundaries, 292 dispositions,
40 writer scenarios, 476 reader cases, 27 form/file cases, one lazy-cache case
and 24 entropy cases. Four additional ownership controls exercise retained and
actively mutated buffers. All 1,128 logical observations pass on qualified Rust
855/c4 and admitted Native 2f/V2/80bd, at consumer O2 under separate normal
32 MiB and poisoned-free 256 MiB heaps, both with an 8 MiB runtime stack and zero
leaks. Compiler host stacks are explicitly 64 MiB. Ninety completed memory
processes are indexed by the evidence file; every process also checks the actual
LLVM matcher object against 1,048,576 independent scalar lane observations.
Real scratch operations check descriptor and path cleanup in addition to managed
allocation leaks. Historical failures and source/object epochs remain separate.

The current parser extracted canonical documentation for all 323 declarations,
including constructors and private helpers, with no diagnostics or orphans.
`compiler/tests/lib_multipart.rs` adds a Rust-only public-CLI regression using
committed fixtures and the real C object, requiring no SDK during CI. Its normal
compiler build initializes and captures the exact host runtime archive. Root
owns the pending Cargo execution; standalone frozen-CLI results do not claim that
Cargo target has passed.

Remaining gates include O0 relinks, full portable public objects and optimized
code inspection, paired SIMD/scalar/reference public benchmarks, additional
resource-error controls, genuine source-absent contract admission and live
buffered adapter runtime checks. The buffered both-effect experiment is still
under review. No shared Stream or MIME API was changed, and no mutation effect
was erased to force an adapter through an IO-only callback.
