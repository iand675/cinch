# Public map workload gate

The comparison uses the existing `Data.Mutable.HashTable` and the proposed
`Data.Mutable.Map` façade through their public constructors, updates,
lookups, deletion, size and traversal. Keys and values are bounded naturals.
No callback mutates the map, so both containers can satisfy this shared
workload. Stable-entry traversal under mutation, captured policies and
non-reflexive floating keys are additional façade capabilities verified in
the independent correctness matrix, rather than performance equivalences
claimed by this comparison.

Three measured operations expose different costs:

- Construction, overwrite, all lookups including a miss, even-key deletion
  and complete final traversal. Every binding is built within the interval.
- Complete traversal of a prepared map, with one changing overwrite and
  point lookup per iteration. Preparation is outside the interval.
- Immediate-stop traversal of a prepared map, again with an observable
  changing overwrite and point lookup. Preparation is outside the interval.

The HashTable baseline obtains a public value snapshot with
`hash-table-to-list` and honors the same stopping callback protocol while
walking it. The façade uses `mutable-map-all`, which captures entry
identities before invoking callbacks. Both setup costs remain inside the
interval, including the immediate-stop workload. The underlying HashTable
uses its existing SWAR control-byte matching; the façade adds a digest
index, stable entry owner and captured policy. Different hashing paths,
ownership representations and allocation costs are material results.

`generate_map_benchmark.py` renders each adapter from one common workload
and independently computes result fingerprints using integer bindings.
All lookups, traversal sums/counts, deleted-state sums, final sizes and
iteration-dependent values contribute to the fingerprint. Immediate-stop
does not constrain unspecified iteration order: it observes the callback
count and a deterministic point lookup. This is a bounded workload model,
not a replacement for the independent 800-step package oracle.

`bench_maps.py --action build` preserves exact source, compiler, provider,
runtime, executable, LLVM and assembly hashes. Normal generated RC is used
for both variants. `--action preflight` verifies each independent
fingerprint in normal32M/8M and separate poison256M/8M processes, all with
zero leaks. `--action measure` requires a parent-granted timing window and
rotates variant order after a warmup. Every measured process must still
match its fingerprint and report zero leaks. All processes use the same
argv[0]. Runtime allocation counters include startup, prepared container
construction and final teardown; they are not attributed solely to the
timed interval. Counters run during preflight and are disabled for paired
measurements. Teardown is excluded from the runtime clock interval.

Both fa3/c4 O2 artifacts and all three workloads at sizes0/1/31/256/4096
with eight iterations now pass independent scalar fingerprints in normal
and poison profiles:60 zero-leak processes. The initial Nat-literal
branch and nested-case layout fixture diagnostics are retained separately.
The exact receipts and normal allocation profiles are recorded in
`stdlib-maps-benchmark-results.json`. No measurements have run, and no
throughput, allocation reduction or Native adoption claim follows.
Source-selected Native execution and genuine source-absent interface
admission remain separately reported gates. The latter currently exposes
the genuine Region-scheme/Type-header contradiction and open-row transport
gap, without metadata rewriting.
