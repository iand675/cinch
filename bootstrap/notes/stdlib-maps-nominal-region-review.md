# Map nominal Region review

This is a static proposal. No compiler, linker or runtime was invoked, no historical interface was rewritten, and no production source changed. C839 remains unselected. The new explicit nominal Region feature requires its own qualified compiler epoch; the frozen855 source results do not qualify the proposed declarations.

The original C839 source remains `build/maps-entry-candidate/sources/Data.Mutable.Map.cinch`, SHA256 `c839d9b0868aaddfc9914fe515eeb613ba23bd934cc07dc4fa58e44463c00078`. The baseline remains `compiler/prelude/Data/Mutable/Map.cinch`, SHA256 `0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6`.

| Declaration | Exposure and role | Proposed declaration |
| --- | --- | --- |
| `MutableMap r k v` | Public handle; its operations use `Mut r`. C839 holds a digest index and scoped state cells. | `data MutableMap (r :: Region) k v where` |
| `MapEntry r k v` | C839 private stable identity; its state is `Var r (EntryState k v)`. | `data MapEntry (r :: Region) k v where` |
| `HashTable r k v` | Public dependency; `r` scopes mutation and is absent from its array field representation. | `data HashTable (r :: Region) k v where` |
| `Var r a` | Public dependency of C839 and insertion sessions; `r` scopes mutation and is absent from its array field representation. | `data Var (r :: Region) a where` |
| `LinkedList r a` | Baseline dependency; `r` scopes access to the root's privately mutable arrays. | `data LinkedList (r :: Region) a where` |
| `ListElement r a` | Baseline dependency; element handles must carry the same scope role as their owning list. | `data ListElement (r :: Region) a where` |

The role names a scope used by runtime mutation; it is erased from the runtime representation. None of these proposals adds a constructor argument, changes field order, changes a runtime symbol or changes key/value representation. The C839 experiment needs the first four declarations together. The baseline experiment needs `MutableMap`, `HashTable`, `Var`, `LinkedList` and `ListElement` together. Changing only the public map header would leave private `MapEntry`, cells or imported dependency parameters inconsistent.

`MapKeyPolicy k`, `EntryState k v`, `Table k v`, `ListNode a`, `MapError` and `InsertStatus` have ordinary payload types or no parameters. Their declarations remain unchanged. The persistent `Data.Map.Map k v`, `Data.HashMap.HashMap k v` and private hash-map `Spine k v` also retain ordinary `Type` parameters. No role is inferred merely because a variable is called `r`. The Optional and Float modules declare no new regional nominal type, so they need no declaration patch.

The patches contain only declaration binder replacements. Existing constructor and public/private function signatures remain byte-identical. In the new feature epoch, checked nominal Region argument positions must determine the scoped binders in these signatures, including pure private selectors such as `policy-of`, `entry-serial`, `index-of`, `cell` and `table-of`; latent or performed mutation is not the only source of the role. `k`, `v` and `a` continue to mean ordinary runtime payload types. The `r` and `s` scopes on cross-map copy/equality stay independent.

The isolated copies are under `build/maps-nominal-region-static/{candidate,common,baseline}`. Reconstructible minimal proposals are `tooling/map-candidate-nominal-region-proposal.patch` and `tooling/map-baseline-nominal-region-proposal.patch`. `python3 tooling/prepare_maps_nominal_region.py` checks the original source hashes, makes those copies, verifies exact inverse substitutions, records historical headers, and prepares the IEEE64 consumer. It does not run a language compiler.

Actual historical headers remain evidence. Both frozen855 source outputs export `MutableMap`, `HashTable` and `Var` with first parameter `r :: Type`; the baseline also exports `LinkedList` and `ListElement` that way. The C839 private `MapEntry` has no public exported header. The separate genuine9dc interface receipt retains its contradictory `MutableMap r :: Type` header and closed `forall (r :: Region)` map-size/copy schemes, the experimental4a map-size kind refusals, and the independent map-all missing resolved metadata refusal. Those files and their hashes are recorded without modification in `../evidence/stdlib/stdlib-maps-nominal-region-static-results.json` and the original `../evidence/stdlib/stdlib-maps-interface-results.json`.

Once the qualified new producer is available, first check the full declaration closure and its pure selectors, then export genuine new interfaces and copy them byte-for-byte into a directory with no library sources. Inspect the actual defining identities and Region parameter metadata before testing closed map-size, independent-region copy, aliases and reexports. Keep the old genuine Type headers as negative/compatibility evidence: consumers must not reinterpret or rewrite them to Region. Test explicit `Type`, a concrete `Nat`, a shadowed unrelated handle and a foreign defining identity as refusals. Open-row map-all metadata admission remains a separate gate; correcting a nominal role does not itself prove that transport feature works. No metadata restamping is part of this proposal.

IEEE64 preparation is independent of these role changes. The seven original reference observations retain duplicate quiet NaNs, distinct signaling/negative NaN payloads, signed-zero replacement, non-reflexive self equality, clone/copy multisets and deletion by key. The prepared consumer is byte-identical to the historically qualified fa3 fixture: SHA256 `ae7d15d9332bc9d721463a509a56400326c7390c76acfc1b817c961b480e9f1f`. It uses full-width bitcasts, checks exact key/value multiplicities without NaN lookup, and retains the high-bit patterns `0xfff8000000000017` and `0x8000000000000000`. Static preparation does not advance either pending frozen855 IEEE64 runtime verdict.

## Qualified Role+Slots source job

The next source-only job pins `/workspace/dasl-stdlib-io/build/stdlib-nominal-region-slots-frozen/cinch`, SHA256 `330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2`, and the adjacent runtime, SHA256 `d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`. The qualified tuple's provenance SHA256 is `c303ec5ab34c80e19c19e659fa34ee35d1d5b73d0e2d9e81a1c9d990c7712251`.

At an authorized CPU1 handoff, run:

```sh
source /workspace/.tools/cinch-env.sh
CINCH_CLANG=/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang python3 tooling/qualify_maps_nominal_source.py --cpu 1
```

The wrapper verifies both exact tuple hashes, all proposal/original hashes, and the unchanged historical interface hashes before checking a fresh350MiB free-space guard. Below that guard it records a resource pause and invokes no compiler. The first guard observation was178,163,712 bytes free; `../evidence/stdlib/stdlib-maps-role-slots-source-results.json` records the pause. The job is therefore queued, without a new source verdict.

Once permitted, the sole requested mode is whole-source checking of the C839 declaration-only proposal with its HashTable/Var dependency proposals. The source and interface outputs are bounded to5MiB. No emitter, linker or runtime is requested. It uses these isolated sources and the tuple's embedded modules; it selects no working Root Array implementation.

On source success, the wrapper inspects actual generated `Data.Mutable.Map.MutableMap`, `Data.Mutable.HashTable.HashTable` and `Data.Mutable.Var.Var` defining headers, requiring `r :: Region` with ordinary payload parameters retained as `Type`. `MapEntry` is private: its explicit defining declaration must pass source checking, but it has no public exported type header. The report distinguishes those facts rather than manufacturing metadata or adding a test export. Source-absent transport, historical refusals, open-row capabilities, runtime behavior and Native-family admission remain separate gates.

The existing mixed IO consumer is also prepared unchanged: `compiler/tests/fixtures/mutable-map-effect-control.cinch`. Its source supplies duplicate key1 bindings, yielding two live bindings and a value sum of50; both value and EqualBy callbacks retain `Mut r` and `IO r`, with two observable equality calls. `../evidence/stdlib/stdlib-maps-mixed-io-preparation-results.json` records the exact fixture hash and expectations. This preparation neither executes that consumer nor replaces its original source, result fingerprints or capability rows.

The subsequently authorized source-only200MiB start/180MiB reserve exception completed successfully on CPU1. The actual check took3.058 seconds, started with358,834,176 bytes free and finished with356,139,008 bytes free, producing2,532,025 bytes including the runner receipt. `../evidence/stdlib/stdlib-maps-role-slots-source-qualification-results.json` preserves the exact330/d91 tuple and genuine emitted headers. All three actual defining headers have `r :: Region`; key/value payload parameters remain `Type`. Private `MapEntry` was accepted as an explicit Region declaration and remains absent from public exported type headers. Historical Type-interface hashes were rechecked unchanged. The initial350MiB resource pause remains preserved independently.

The narrow exception command was:

```sh
CINCH_CLANG=/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang python3 tooling/qualify_maps_nominal_source.py --cpu 1 --minimum-free-mib 200 --minimum-reserve-mib 180 --artifacts build/maps-role-slots-source-a2 --output build/maps-role-slots-source-a2-results.json
```

That source-only exception does not reduce runtime or emission guards. The accepted source is the isolated declaration-only proposal (`7085aebe4c10dcd0cf27be6ffc0348a239398b07d54f60cb098722802b244c65`), not a replacement of historical C839. It does not qualify source-absent consumers, runtime behavior, a Native family, candidate selection or package completion.

## One bounded Role+Slots runtime unit

A separately authorized runtime unit passed on exact330/d91 using the unchanged350MiB start guard and250MiB reserve. Its source is the isolated annotated-Region C839 proposal, not a production source replacement. `tooling/qualify_maps_nominal_runtime.py` combines the original generated edge consumer, original generated IEEE64 consumer and unchanged mixed IO+Mut consumer under unique test module names. Its inverse transformation assertion verifies that only each module name and an export of the existing `main` entry change; all helpers, source values, callback behaviors and result assertions remain intact. The harness returns a distinct failure range for each original group.

The combined unit covers three independent value-NaN/callback-mutation edge groups, seven full-bit IEEE64 observations and the mixed IO+Mut source/value/EqualBy checks. It compiled in7.483 seconds and produced10,081,843 bytes in the functional directory. Fresh free space was417,722,368 bytes; afterward it was407,429,120 bytes. Both normal32MiB and separately poisoned256MiB runs used an8MiB stack and exited0 with empty output and `live=0 immortal=0 leaked=0`. The recorded allocation counts describe this complete test unit only; they are not a performance comparison.

`../evidence/stdlib/stdlib-maps-role-slots-runtime-results.json` preserves all exact source, generated fixture, executable, LLVM and tuple hashes plus both complete runtime diagnostics. The original attempt report remains `build/maps-role-slots-runtime-a1-results.json`, SHA256 `37be0c3c0be8dadd5f641add1f001a470cedcbc73040c64e7b52e83ae8a5327e`. All historical Type-interface hashes were rechecked unchanged. Full800 traces, the remaining ownership/control matrix, source-absent consumers, Native qualification and matched candidate benchmarks remain pending. No candidate was selected, no production source was promoted and no Native family was relabeled.

The subsequent resumable330/d91 matrix uses `tooling/qualify_maps_role_matrix.py`: one unchanged12-group ownership/policy/live-mutation consumer and eight unchanged independently generated100-step consumers. Each unit retains the350MiB fresh/250MiB reserve guards, caps new functional outputs at30MiB, and runs normal32MiB and poisoned256MiB heaps with an8MiB stack. A first genuine failure is retained and stops progression. Passed units are not rerun on resume.

At the first natural CPU boundary, all12 control groups and trace0's100 observations passed both profiles with zero leaks. Build times were6.831 and7.528 seconds; functional output sizes10,387,871 and10,036,767 bytes. `../evidence/stdlib/stdlib-maps-role-slots-matrix-results.json` records this partial boundary separately from the original combined edge/IEEE64/mixedIO unit and older855 evidence. Traces1–7 remain pending at this checkpoint.

`tooling/archive_maps_nominal_runtime.py` archives only each closed successful unit's exact ELF, LLVM and newly generated disposable interface caches. It verifies both actual runtime verdicts, input tuple/source identities, every original hash/size/mode, and every decompressed member. Archive/result/index durability precedes unlink; original parent directories are fsynced afterward. The private archive-aware index is `build/maps-role-slots-closed-unit-archive/index.json`; all source inputs, fixtures, logs, historical interfaces, frozen/compiler/provider/runtime and benchmark inputs remain plain.

The completed330/d91 matrix now passes all800 independent trace observations and all12 ownership/policy/live-mutation groups. All18 matrix processes passed normal32MiB and poisoned256MiB heaps,8MiB stacks and complete zero-leak/result checks. Nine bounded builds took67.157 seconds in total; the largest functional unit was10,387,871 bytes, below30MiB. Every unit retained the original generated consumer or control fixture unchanged. The full receipt is `../evidence/stdlib/stdlib-maps-role-slots-matrix-results.json`.

Each success was durably archived before starting the next unit:89,575,083 original functional/cache bytes became22,124,307 archive bytes, recovering67,450,776 bytes. The final private archive index SHA256 is `7364b3485313b53c4ed051151fc92b025102bcf85a5ffafe78c225bb7840597a`; the exact matrix ledger SHA256 is `9e118dbafbfdccb87884367190472e89933ea65d369f0fcc1be18416b02be891`. These results remain separate from the already passing two-process combined edge3/IEEE64seven/mixedIO unit and older855 evidence.

Read-only inspection of the new genuine source interfaces confirms the remaining transport distinction: closed map-size/map-copy exports have resolved schemes with the corrected defining Region header, while open-row map-all/map-collect/map-equal-by still have `resolved:null`. No source-absent import was attempted in this matrix. Historical Type headers/refusals remain unchanged. Source-selected runtime qualification does not settle that transport boundary, actual Native qualification, matched public pipeline performance or production selection.
