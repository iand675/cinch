# Unicode scalar pipeline review

This review covers the actual pinned 344-record Unicode surface independently of the pending UTF-8 text facade. The public scalar drivers exercise letter/space, all 14 basic predicates, upper/lower/title and invalid case selectors, simple-fold successor/equivalence, built-in and custom range membership, unions, catalog lookups, Turkish/custom mappings, and exact indexed case observations. Four 16-value datasets cover ASCII, Latin/Greek/marks/space, supplementary characters including Unicode 17 additions/noncharacters/unassigned values, and negative/surrogate/out-of-domain numerics. All returned words contribute to an observable unsigned checksum.

The driver uses bounded machine counters/phases and U64 wraparound checksums. There is no per-operation natural counter, checked remainder, optional harness allocation or string dispatch. Fixed per-workload loops dispatch once per timed sample. Tables, custom metadata validation, locale rules and input parsing are prepared outside the timer; actual catalog lookups and case-observation allocations occur inside it. The reference calls actual pinned public SDK methods; fold equivalence independently walks their simple-fold orbit, and numeric invalid inputs retain their exact policy. No normalization/expansion is introduced. The `harness` control retains only input lookup and checksum/loop costs.

An initial variable-dispatch round is preserved as exploratory evidence. Its unrelated controls and some reference samples drifted, so it did not justify adoption. The corrected driver then measured paired/interleaved baseline and candidate samples, rotating candidate order and alternating reference/Cinch order. Both full-local and class-only candidate rounds are retained. Each corrected group uses 32,768 public operations and five raw samples plus warmup. All complete checksums agree. Three exclusive coordinated CPU0 windows took 8.2, 15.5 and 18.8 seconds; background affinity-core activity is recorded in the machine receipt.

The broad candidate replaced six private U32 operations across every table accessor. Although Native lookup sped up, some Rust fold/special paths regressed as their accessor/inlining shape changed. That candidate was rejected. The selected candidate restricts the operations to classification directory/run/metadata routing/search. All 163 unrelated case/fold/locale/range-info/accessor declarations remain source-byte-identical. Original category/script/property/fold bits, all 7,235 ranges, every case delta/orbit and all catalog identities are unchanged.

Generated Native O2 classification-table code previously had 174 imported `Data.Machine` call sites across its accessor/search graph. The selected source has zero such sites; Rust already had zero. Six straight-line local LLVM add/subtract/AND/masked-shift/unsigned-comparison kernels let LLVM fold those operations into direct scalar routing and read-only loads. No backing buffer, layout access, store, proof relaxation, ABI change or whole-table/vector stack copy is involved. A one-codepoint lookup does not benefit from forcing SIMD.

Final paired medians, nanoseconds per public operation:

| Pipeline/pattern | Rust baseline | Rust selected | Native baseline | Native selected |
|---|---:|---:|---:|---:|
| letter Latin/Greek |63.81|61.16|219.87|89.90|
| letter supplementary |35.40|35.63|156.77|58.41|
| all predicates Latin/Greek |669.76|651.04|2619.68|677.75|
| all predicates supplementary |343.07|355.69|1817.91|595.94|
| letter-table Latin/Greek |38.90|34.81|223.78|70.46|
| Greek-table Latin/Greek |39.75|40.09|205.32|72.05|
| table union Latin/Greek |109.27|112.95|565.72|183.37|
| catalog Latin/Greek |110.10|105.66|279.15|190.10|
| custom-table Latin/Greek |12.89|10.87|87.50|98.56|
| case mappings Latin/Greek |165.82|174.38|775.00|762.50|
| fold Latin/Greek |77.22|76.50|334.57|351.39|
| Turkish mappings Latin/Greek |80.39|89.11|794.47|801.48|

The classification/membership Native gains are consistent across targeted code and repeated measurements. Unchanged paths still show reference/harness/layout/scheduling noise and some regressions, all retained rather than hidden. Rust classification is generally near baseline; several scalar paths remain slower than the reference. Catalogs use immutable comparison trees rather than runtime hash maps; copied observation APIs create typed values while reference structs are direct values. Native source-client case/fold/custom paths also retain imported operations outside this targeted graph. These are honest remaining performance costs, not missing functionality or claims of universal speed.

Exact selected source passes all four independent complete-domain modes under both frozen frontends: all 1,114,112 numeric code points, valid `Char` observations, complete classification metadata, and standard-case reconstruction into checked custom rules. There are 4,354 normal/poison memory processes with zero leaks, plus all boundary/selector/locale/custom-refusal and 260-map/262-alias/list/index/catalog checks under both modes/frontends. Normal heap/stack is 32 MiB/8 MiB; poisoned is 256 MiB/8 MiB. Ten generator/corpus tests and reproducible source/layout checks pass. All 336 table declarations have canonical docs with zero diagnostics/orphans. Six actual scalar kernels compile to wasm32/AArch64 objects; the updated `lib_unicode_kernel` target supplies the powerpc64 big-endian object gate for parent Cargo integration.

The exact compiler/source/runtime/provider/input/driver/executable/LLVM/assembly identities, every raw sample, unchanged-declaration list, corpus hashes and codegen call-site review are in `benchmarks/stdlib/unicode-optimization-evidence.json` and `benchmarks/stdlib/unicode-scans`. Rust remains frozen `f2d5ae4c…`/runtime `c4b1097b…`; Native remains Char-contract `9e2a64e0…`/source `0b11df4d…`/runtime `12945b71…`. Functional Native wrappers link O0; every measured Native artifact is explicitly re-linked O2 against its matched provider. Candidate experiments remain reproducible from the factual baseline generator, and no shared inventory/provider/site file is edited in this lane.
