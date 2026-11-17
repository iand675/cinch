# Pending serial runtime and kernel gates

Nothing in this plan constitutes a runtime observation. Wait for Root's explicit headroom handoff and the shared CPU2 token before starting. Every ordinary source/emission/runtime unit retains a fresh 350 MiB start and 250 MiB reserve. The existing runtime driver is stricter than the 30 MiB allowance: it charges the saved source artifacts plus new project/cache/IR/ELF/logs together against 20 MiB, stops at the first failure, and never overwrites a prior epoch. Source-only exceptions do not authorize runtime.

## First unit: twelve adaptations

The exact source-positive entry is `build/tabwriter-adaptation-source-v4`, Main SHA `68d677d1dfe81598b093393cbf46beec36549b2f7ee69834de5cf3118b1dde7a`. Compiler SHA is `330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2`; matched runtime SHA is `d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`. Both actual artifacts and the saved/current Main were rehashed after restart. No new compiler or provider is substituted.

From the owned buffered lane:

```sh
source /workspace/.tools/cinch-env.sh
python3 -B experiments/tabwriter-v1/run.py --checked build/tabwriter-adaptation-source-v4 --work build/tabwriter-adaptation-runtime-v1 --deadline 60
```

The driver copies only verified source into the new epoch, emits IR with the exact 330 compiler, explicitly links LLVM20 O2 with d91, and runs the same ELF in normal 32 MiB and poisoned 256 MiB heaps, both with an 8 MiB managed stack. Empty stdout, zero exit, and live/leaked zero are required. Any source/runtime mismatch retains all artifacts plain and suppresses subsequent profiles/units.

Estimate: 12–16 MiB complete charged unit, about 8–15 seconds under comparable load; the hard cap remains 20 MiB and the per-phase deadline 60 seconds. A prior source-matched ordinary protocol unit charged 12,309,929 bytes and took 3.75 seconds to emit, 2.77 seconds to link, and 0.103 seconds per runtime profile. These are resource estimates, not benchmark results.

Only after both profiles succeed, and only if the archive's fresh guard passes:

```sh
python3 -B experiments/tabwriter-v1/archive_success.py --work build/tabwriter-adaptation-runtime-v1
```

Archive scratch is capped at 10 MiB. Original/report bindings, full member hashes/sizes/modes/metadata, gzip and decoded hashes, durable archive/index-before-unlink and parent directory fsync are unchanged. If the archive guard fails, keep the successful ELF/IR plain pending a separate authorized recovery window. Preserve all prior failure sources/logs/IR/ELFs and all frozen/provider/benchmark inputs.

## Subsequent natural units

1. `ErrorOwnerMain.cinch`: reuse its exact positive `build/tabwriter-error-owner-source-v1` for one new runtime epoch. Observe NaN payload/signed zero and original error priority, then 128 retained callable errors after local writer reclamation.
2. `OfferOwnerMain.cinch`: reuse `build/tabwriter-offer-owner-source-v1`. Observe actual independent mutations, valid/invalid text outcomes, 10,000 columns and all bytes of the 64 KiB table. These currently have source proof only.
3. `LayerMain.cinch`: first freeze/check a new source epoch, then runtime only if positive. Observe one-layer flush, a retained callback from a local factory, advanced continuation, and actual same-call reentry refusal. Clear the fixture's temporary back edge before reclamation; do not describe this as cycle collection or an expiring callback contract.
4. `StreamMain.cinch`: first source-check the new optional adapter closure, then matched runtime. Use exact explicit-Region `Effect.Stream` dependency SHA `c36045c77c4b5baee21f2756a5abed1de697a318ad549274a2699a6c978c6f8e`, separately bound by `dependencies/manifest.json`. Distinguish baseline Type-head Stream SHA `e2c2dfb9f12e0855b178ca68cb8cad8c3376170f79d8ea1d23216a6d77e3d70d`; do not relabel it as explicit Region. Verify actual live/advanced dispatch and the documented zero convention for an error whose legacy result carries no separate accepted count. Do not infer a physically written prefix from that zero.
5. Production classifier: source-check the prepared `build/tabwriter-kernel-v1/Main.cinch`, verify manifest and exact production LLVM-body hashes, then use `run.py --c-support experiments/tabwriter-v1/kernel_fixture.c` in a fresh matched runtime epoch. Its 16,384 opaque vectors cover eight changed lanes × all 256 byte values × four actual delimiter modes × both filter states. The C oracle classifies one scalar lane at a time and never reuses the SIMD expression. Keep source, C fixture, emitted LLVM and tool identities separate.
6. Portable classifier objects: separately run `qualify_kernel_objects.py --kernel build/tabwriter-kernel-v1 --work <new-work>` under ordinary guards. Compile the same hashed production body into actual wasm32 and AArch64 objects with LLVM20. This is object compilation only; no foreign execution or whole-library portability is claimed.

Coordinate a CPU2 handoff at every peer boundary. Freeze a distinct source/runtime epoch for any routine fixture correction and preserve the original diagnostic. No Root source, shared stream body, runtime, provider, ABI or checker rule changes are part of these units.

## Open boundaries

The existing twelve adaptation controls, three error-owner groups and five offer-owner groups are source-positive only. Layer, stream and production kernel remain unqualified. Public pipeline benchmarks, paired timing/codegen review and admitted current Native comparison remain pending. The new Native checker passed focused tests with an actual supplied 32-module Bootstrap closure drawn from a globally source-checked 52-source family; that is not full provider/exporter/Native-family admission. Do not use old 2f to qualify explicit-Region source or silently substitute a newly frozen compiler. The known buffered callback-cycle failure and production collector hold remain explicit.
