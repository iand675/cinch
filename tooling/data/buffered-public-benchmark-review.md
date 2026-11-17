# Buffered public pipeline review

This checkpoint observes the maintained `8d0557bd` library under frozen Rust frontend `7e0d98d1`, runtime `c4b1097b`, and LLVM 20 O2. The executable uses ordinary reclamation, without compiler RC-debug instrumentation. It does not promote the buffered family or the separate combined-effect experiment.

`buffered-public-benchmark-checkpoint.json` contains exact tool/source/executable/IR/assembly identities, all 54 timing conditions with three raw paired samples, allocation receipts, and static hot-function call inventories. Reproduce its verification with:

```sh
python3 tooling/review_buffered_bench.py \
  --artifacts build/buffered-benchmark-rust-v1 \
  --preflights build/buffered-benchmark-preflight-v1.json build/buffered-benchmark-preflight-fringes-v1.json \
  --timing build/buffered-benchmark-timing-rust-v1.json \
  --output tooling/data/buffered-public-benchmark-checkpoint.json
```

The public driver alternates two prepared owners, retains result observations, and keeps input preparation outside the measured interval. The independent pinned SDK observer uses explicit fresh source events and copied public line/scanner tokens to model these ownership contracts. Fresh allocation/copy is a separate control. Checksums observe lengths, selected byte positions, scalar values, actual scalar widths and operation statuses; separate complete functional corpora check exact bytes and malformed-input behavior.

All 729 preflight conditions passed, covering nine pipelines, three patterns, nine sizes from zero through 64 KiB, and zero/one/three iterations. The 1,458 Cinch normal/poisoned processes reported zero live/leaked blocks. A single exclusive CPU0 window then measured 4 KiB/64 KiB inputs with three paired samples after warmup. Reference order alternates. The shared cgroup's `nr_throttled` and `throttled_usec` did not increase; raw counters remain in the receipt. Root runtime work on CPU3, JSON Native work on CPU2, and possible Collections work on CPU1 were recorded as background load. These are raw host observations, not load-adjusted timings.

The measured old-epoch scalar paths remain slow: byte/rune reading takes about 30–39 times the reference time; rune scanning takes about 33–54 times. On 64 KiB inputs, writes and transfers take about 0.39–0.61 times the reference time, while the fresh-copy control takes about 0.25–0.34 times. The complete condition table retains the smaller/noisier cases rather than discarding them.

On ASCII 4 KiB inputs, subtracting the setup-only allocation observation yields approximately 6.007 headers per byte read, 9.007 per rune read and 13.011 per rune token scanned. All are reclaimed. The public byte consumer occupies 2,006 lines of optimized assembly; the byte-reading implementation contains extensive inline reference-count and ownership work. Static emitted callsite counts are kept distinct from dynamic allocation counts. Bulk-memory results do not explain the scalar slowdown.

The existing packed byte/whitespace classifiers remain appropriate for bulk scans; their separate exact production-kernel receipt covers host O0/O2 and foreign object compilation. SIMD cannot eliminate each public scalar result's current ownership cost. A fair next comparison must compile this same frozen source with an explicitly admitted borrow/reuse compiler epoch, preserve ordinary reclamation and checksums, and inspect changed allocations/calls before selecting an optimization.

Native public benchmarks, remaining complete-family runtime gates, and the retained-callback cycle are still open. No API retention contract is narrowed, and no collector behavior is adopted here.
