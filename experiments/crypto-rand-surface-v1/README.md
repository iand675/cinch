# Random-library surface proposal

This is an isolated static proposal, not an installed or qualified library. It changes no prelude, runtime, package inventory or sealed provider experiment. No proposed module, SDK fixture generator, runtime or benchmark has been compiled or executed for this proposal.

`AUDIT.md` maps all five pinned reference public objects to the actual current source. `proposed/` contains seven concrete modules: full-width big-endian natural import, reader-aware arbitrary-bound uniform integers, raw exact reads, buffer filling, SIMD/SWAR alphabet mapping, secure text conveniences and candidate shaping. `PRIME_API.md` records the full-generation dependency gap without supplying an unqualified prime generator.

`reference/generate.go` is an independent fixture generator with model and actual-provider modes in separate processes. `reference/INVOCATION_PLAN.json` records source-derived expected counts and settings; no generated results are included. `QUALIFICATION_PLAN.md` and `BENCHMARK_PLAN.md` describe the next functional, ownership, code-generation and timing gates. They grant no execution lease.

`STATIC_REVIEW.json` records only textual checks and their limits. `MANIFEST.json` pins the SDK sources, audited Root dependency sources and proposal artifacts. `SHA256SUMS` seals all proposal files except itself. A receiver must select the qualified compiler and request the separately bounded build/runtime handoff before treating any declaration as accepted code.

Security boundaries remain explicit: default operations use actual system entropy; caller-source overloads inherit their source's quality; deterministic fixture input is only a model. Existing arbitrary-size natural sampling remains intact. The alternate masked algorithm is not claimed to be faster for every bound, and proposed filling includes its temporary allocation/copy.
