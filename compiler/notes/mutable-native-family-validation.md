# Verified mutation and inference compiler family

The [validation receipt](../evidence/other/mutable-native-family-validation-results.json) records
the complete self-built Native family for contextual mutation effect labels,
source-owned phantom regions, and strict local value restrictions. This epoch
adds only `8aff20d0` and `6759fb9d` to the completed phase-1/Char baseline.
Its 51-module compiler source hash is `4de3bd70`; Stage1 was rebuilt from that
source using the certified old Stage0/runtime bridge. No prior Stage1 or
compiler Cargo cache was copied.

Stages 2 and 3 produce executable `58676dec`, LLVM `93f32993`, and 51 identical
canonical interfaces. All stage smokes, the source-absent compiler-entry
consumer, and the changed-source diagnostic rebuild passed. The separately
changed compiler is `eee7bb4b`. Both current stages link the explicit phase-1
base provider `e70250a5`, object `645a3d0e`, and runtime `80bd5b54`. Its producer
source remains `92f5cd23` and String source remains `54ae8a33`; this validation
reuses those verified bytes and does not claim another Rust producer build.

The actual Stage3 frontend passes all 32 mutation/value-restriction vectors and
the unchanged 26 positive and 30 negative source programs from `Run.Infer`.
The focused vectors inspect the emitted Region/Type parameter headers, rather
than treating acceptance alone as proof of kind inference. Negative cases
require named refusal categories. Three CLI diagnostic categories differ from
the in-process oracle because the full CLI resolves dictionaries and
refinement evidence earlier; all 30 prior-epoch negatives were independently
baselined, the precise three differences were reviewed, and the original
source programs and in-process expected strings remain unchanged.

A separate producer exports `Vector [Region, Type]` and `Ordinary [Type]`, then
removes its source before four interface-only consumers. The region roundtrip
passes; forbidden imported kinds and allocation polymorphism are rejected by
their named categories. The 15 forged-Core authority observations also pass
against the actual checked kind implementation, with normal 32 MiB and
poisoned-free 256 MiB heaps on an 8 MiB stack, reporting zero leaks.

`build/mutable-stage3-frozen` preserves the compiler, source/LLVM, completed
gate receipt, adapter, and matched provider. Its local provider receipt only
relocates interface manifest paths; original payload bytes and their hashes
remain intact. Imported scalar primitive lowering is a separate later epoch.
