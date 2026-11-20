# Explicit current-source base artifacts

An ordinary bootstrap keeps using its pinned library producer. Reaching that
fixed point does not establish that a changed embedded `Data.String` was linked.
The explicit artifact workflow keeps the restored pins intact and selects a
separate, fully identified base object for stages 2 and 3 and their clients.

`tooling/build_base_artifact.py` builds the current Rust compiler, whose
`EMBEDDED_SOURCES` includes the current base family. A trivial artifact root
contains no implementation override. The producer emits one complete base
object and its actual checked interfaces, and retains every embedded module's
bytes beside the object and runtime archive. Its receipt records compiler,
source, runtime, object, LLVM, root-request and interface identities.
The family is complete at the module level. Generic specialization coverage
is the explicit concrete demand inventory, rather than every possible
combination of runtime representations. An additional consumer demand must
be emitted by the producer and validated in a new artifact epoch.

`bootstrap --library-artifact RECEIPT` admits that selection without falling
back to the pinned provider. Admission independently checks current module
bytes and their interface source hashes, the interface dependency graph, object
and LLVM content hashes, runtime identity, target, ABI, word size, byte order,
reclamation and self-describing descriptors. The String constructor units must
be present in the object, and every requested unit, dictionary and method
specialization must occur in its actual ABI metadata. An altered receipt, missing payload or incompatible
source family remains a named failure.

The phase-1 String source is the repeat/concatenation implementation from
`32d16d4432b30d4e1b84d06d39aa89b97761aea9`: SHA-256
`54ae8a339b996756323d8f5598287ca3abdfe8ce5e291cd62e71fdf975bd3b0e`,
interface source hash `9736755187434316968`. The independently marked
source-override tests intentionally have a different source hash. Later
packing, byte and search work is outside this source snapshot.

Fresh-base validation requires separate observations:

- The fresh Rust CLI runs all 398 fixture rows, deep repetition/concatenation
  and temporary-owner/alias stress at O0 and O2 without a source override.
- A verified Native frontend consumes only the explicit base interfaces and
  links exactly that object and runtime. Its clients run the same observations
  in normal and poisoned-free modes on an 8 MiB stack. Consumer optimization
  and the base object's own recorded optimization are distinct settings.
- Concrete and generic ABI probes include `Data.List.is-empty` over a list of
  strings. A missing concrete representation is reported explicitly; selecting
  current module sources does not invent an unrequested generic variant. The
  observed `Data.List.is-empty[O]` demand is included in the concrete root
  inventory, and its executable client must pass before the final gate passes.
- The complete compiler fixed point uses `--library-artifact` for stage 2,
  stage 3 and the changed-source rebuild. Verified unchanged seed/stage-1
  artifacts may be reused with their original hashes. Their old embedded base
  is the bootstrap bridge, not evidence for the new String implementation.

This workflow does not expose private backing buffers, change the uniform-word
ABI, merge duplicate definitions or disguise data as addresses. Validation
receipts must state which source and library epoch each gate actually used.

The [retained current-base validation receipt](../evidence/stdlib/stdlib-string-construction-current-base-results.json)
records the completed phase-1 gate. The fresh embedded producer is `ae9c3f81`
(seed/base source tree `92f5cd23`), the emitted base object is `645a3d0e`,
and its ABI-10 runtime is `80bd5b54`. The explicit provider receipt is
`e70250a5`. All 64 retained base modules match their actual checked interfaces;
the object contains the new constructor workers and no old recursive-return
workers. Public String exports and ABI remain equal to the pinned family.

The full self-built gate uses compiler-source epoch `0b11df4d`. Stages 2 and 3
produce executable `35eb9980`, LLVM `d470b081`, and 51 equal canonical
interfaces. All stage smokes, the source-absent compiler-entry consumer, and
the changed-source diagnostic rebuild passed. The changed-source executable
is separately identified as `5655bbaa`; it links the same fresh object and
runtime. These are separate identities from the reused seed bridge.

The fresh embedded compiler, the prior verified Native frontend, and the
fresh Stage3 frontend each run all 398 independent fixture rows, deep
regressions, and temporary-owner/alias stress at consumer O0 and O2. Each
group has its own executable and runs with normal 32 MiB and poisoned-free
256 MiB heaps on an 8 MiB stack, with zero leaks. The base object itself is
produced at O2. The final generic ABI probe uses a heap-constructed UTF-8
string list, retains its string alias across `is-empty[O]`, and passes both
ownership modes. This does not claim the different, cumulative poisoned API
test shape used by the integration test harness.

The artifact admission and preserved bootstrap suites pass 15 and 38 tests
respectively. Fresh documentation extraction reports canonical comments for
all five changed construction declarations, with zero diagnostics or
unattached comments. Earlier source-override/SDK/timing receipts remain
separate and retain their original compiler and source identities.

The independent parent checked-RC API gate uses compiler `90500329` and test
fixture `e7505291`. Its four tests pass across 433 runtime profiles: ordinary
mode runs each complete corpus (399, optimized 29, and embedded 2 observations)
in one 32 MiB process; poisoned mode runs one independent observation per
256 MiB process. Every process uses an 8 MiB stack and reports zero leaks.
The per-process poison bound accounts for quarantine retaining freed objects
until process exit. This harness change preserves the String implementation
and both heap limits. The parent API epoch is recorded separately from the
fresh embedded and Native compiler epochs above.

`build/current-base-stage3-frozen` preserves a self-contained Native adapter,
all original payload bytes, compiler source/LLVM, and the completed original
gate receipt. Its provider receipt differs only because its checked-interface
manifest names the copied local paths. The snapshot records both the original
and relocated receipt hashes, and its heap-list ABI probe passes in both modes.
