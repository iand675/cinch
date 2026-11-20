# Isolated UUID candidate

All 13 installed public declarations now have an operation-level mapping in
[stdlib-uuid-operation-mapping.md](stdlib-uuid-operation-mapping.md), with exact
source declarations, behavior, adaptations, tests and benchmark coverage. The
machine-readable companion records the installed source and candidate hashes.
This is a partial distribution proposal, not a package-completion claim.

Root's published `ae09257f` integration contains the byte-identical candidate
sources under `experiments/uuid-candidate-v1/sources` and maintained SDK-free
tests. Tests write explicit project-local `Data.Uuid`/`Effect.Uuid` modules.
There is no Root prelude or Native-family promotion. The minimal integration
manifest and its 14 input hashes remain unchanged by benchmark work.

Values preserve all 128 bits as two immutable network-order U64 words. Bytes
enter or leave through checked copying conversions. Parsing accepts exactly
compact, canonical, brace and lowercase-URN forms, with mixed-case hex digits.
Canonical rendering and parsing reuse the existing checked Hex SIMD workers;
comparison uses unsigned packed LLVM words. Every public/private declaration,
constructor, primitive and instance has canonical documentation: 47 nodes,
zero diagnostics or orphans.

Three adaptations are explicit. The strict parser returns an opaque checked
witness and permits total extraction instead of raising a runtime panic.
Unmarshal returns an immutable replacement and preserves the previous value
on refusal. V7 sequences have explicitly owned generator state instead of
implicit process-global state. One generator serializes actual wall-clock
state transitions under MVar; equal or backward samples in the same second
advance the logical tick, whole-second rollback resets its ordering constraint,
and fractional carry is preserved. Actual secure entropy is acquired after
committing and releasing the state. No random/time substitutions are used.

The independent pinned-SDK oracle records 660 parser/receiver/strict-parser
observations, 204 arbitrary-prefix append cases, 370 full-value comparisons,
nine controlled-clock sequences including 4,097 ties, and 48 actual secure
generations. The exact executed oracle source and bytes remain preserved;
a subsequently formatted generator has not been relabeled as that execution.
Standalone qualification on frozen `330adb4d`/runtime `d91ce012` passes the
complete matrix, immutable ownership/hash/range controls and real generator
IO under normal 32 MiB and poisoned 256 MiB heaps with 8 MiB stacks, zero
leaks. Root separately reports its maintained release target passed 8/8,
including actual full-adapter wasm, ARM and custom big-endian PPC objects.
That parent compiler identity is distinct from the frozen CLI, whose PPC
unsupported-target refusal remains preserved.

The first clock-boundary control failure remains plain. Its input had three
extra decimal zeros; the exact limit is 2^48 milliseconds times 1,000,000
nanoseconds. Only the fixture was corrected. Library source and independent
vectors did not change. The original source-only parser-layout failure and
subsequent successful snapshots also remain separate.

Historical Native evidence is narrower: exact compiler `2f077`/provider
`46a7`/runtime `80bd` accepts the full pure source bundle and passes 256 of
660 parser observations under both heap modes. Its full pure matrix and
current IO/MVar family remain pending. Closed successful functional artifacts
have verified lossless archive mappings; failures, frozen inputs and every
benchmark input stay plain.

Public pipeline preflights compare separate SIMD/scalar source copies with
one independent SDK reference. The original and direct-argument harnesses each
pass all 180 conditions and 900 processes. Real default/V4/V7 generation passes
nine conditions and 27 processes using invariant counts, without comparing
random payloads. Matching-count preflights add all 13 measured workloads at
4,096 pure operations and 256 real generations, normal/poison, 99 processes.

The exclusive CPU0 timing window completed all 336 observations in 1.276 s.
Original SIMD medians were approximately 491 ns for canonical parsing versus
1,109 ns for scalar and 57 ns for the reference; rendering was 221 ns versus
315/63 ns. Original prepared comparison was 5.5 ns versus the reference's
18.6 ns, while the direct harness was 10.6 ns. Appending a 4,096-byte prefix
was 339 ns versus 655 ns; short-prefix cases favor the reference. Generation
medians were about 1.14–1.47 us versus 0.27–0.30 us. All raw samples are
retained. One comparison sample is a large outlier, generation varies, and
there is no pre-start cgroup snapshot, so no exact throttle-delta or
zero-throttle claim is made. Neither harness is selected from static code
size or call counts.

Ordinary-RC allocation diagnostics normalize argv0 and subtract 0/1/20
baselines. SIMD canonical parsing uses approximately 32 fewer allocated
blocks per iteration than scalar decoding. Direct arguments do not materially
improve the heap slopes. Remaining parse/render gaps include intermediate
compact/decoded/render buffers; secure generation also opens, reads and
closes the OS entropy device on every draw in the matched runtime. A separate
packed parser is authorized for investigation; runtime-provider changes have
separate ownership and are deferred. No optimization candidate is selected.

Open boundaries include production availability, full historical Native pure
coverage, current Native IO/MVar transport, concurrent shared-generator stress,
and individual timing of String/witness/unmarshal convenience wrappers. Error
acceptance and receiver behavior are covered; structured error values do not
promise SDK message identity. Existing entropy failure behavior is inherited
from the platform primitive. Detailed receipts and the full gap list are in
the operation mapping and integration manifest.
