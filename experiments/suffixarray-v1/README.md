# Immutable suffix-index experiment, phase one

This isolated candidate is outside the registered prelude and does not change
any library inventory or sealed Fields input. Public source checking and
interface-backed documentation extraction pass on frozen C2. Its first small
O0 public consumer passes normal/poison memory profiles with zero leaks. Full
corpus, adversarial ownership, effectful callbacks and portable gates remain
pending; no performance or complete-package claim is made. No Cargo build has
run in this lane.

The pinned SDK oracle now has 153 inputs, 10,710 literal observations, 9,945
regexp observations and 1,375 persistence observations. Every unrestricted
literal position set is independently crosschecked against a naive scan.
`vectors.json` preserves the actual values. The initial pretty JSON output
exceeded the 4 MiB fixture cap after generation; changing JSON whitespace alone
produces the same semantic data in 1,990,439 bytes. The failed generator epoch
and every failed source project remain plain under `build/`.

The pinned cumulative package surface has seven declarations. `surface.json`
records each exact declaration, its proposed mapping and its remaining gaps.
The audit reads the actual SDK compatibility files and implementation, rather
than treating a package name as coverage.

The initial namespace is `Data.Index.SuffixArray`. Its opaque index owns a fresh
snapshot plus a private packed suffix-offset buffer. Construction finishes all
private mutation before returning; neither buffer is exported. `suffix-index-bytes`
clones, including the empty case. Byte observation returns only a scalar. Match
callbacks receive the sealed index, never a mutable view or backing buffer.
A callback may retain that index, and may mutate any copied bytes, without
changing its contents. Adversarial tests must mutate the original input, every
returned clone and callback-retained copies before repeating lookups.

Construction uses prefix doubling with two stable counting passes per round,
not direct comparisons of full suffixes. It takes O(N log N) time and O(N) scratch
space. The reference construction promises O(N); that stronger bound is not
provided by this candidate. Private counters/ranks use eight-byte words, so the
candidate also uses more memory on small inputs than a four-byte representation.
The input-length refusal leaves all packed offsets within the signed address
range. Every store remains an ordinary checked word-buffer store.

Literal lookup performs two binary searches, compares only the query-length
prefix through the qualified SWAR/SIMD byte-view comparator, and returns suffix
positions in deterministic lexicographic suffix order. It includes overlapping
occurrences and excludes the empty suffix. An empty query or zero count returns
no positions. Explicit `AllSuffixMatches` / `AtMostSuffixMatches Nat` replaces a
negative count sentinel. Limited output selects a prefix of suffix order, not
necessarily the first positions in the input. A huge positive limit is intersected
with the available count before any narrowing or traversal.

The search-from callback is a deliberate capability adaptation, not built-in
raw-byte regular-expression parity. It must return the leftmost next match at or
after the supplied cursor, over the complete original subject. The driver checks
bounds, sorted progress and non-overlap; bounds alone cannot prove leftmost
matching. It preserves a shared effect row and the callback's own error type.
On failure, the error retains the accepted prefix. A zero result limit invokes
no callback. Empty matches immediately following another match are suppressed,
and other empty matches advance by one UTF8 scalar, or one byte for malformed
input. Empty input can yield one empty match. Tests must independently establish
empty-match order, malformed-byte offsets, callback errors and stop behavior.

`Text.Regex` currently consumes valid `String` subjects. Repairing arbitrary
bytes before using it can change both input contents and byte offsets; it is
therefore not silently used here. Actual raw-byte regexp integration remains
open. The oracle records both the suffix-index implementation's bounded choices
and the regex engine's ordinary leftmost results: a positive bounded reference
search can deliberately choose later, nonsuccessive matches after suffix-order
candidate selection. This candidate's leftmost callback policy is explicitly
separate from that unspecified bounded subset.

Persistence is not exported by phase-one source. `Effect.Stream` has suitable
reader/writer continuations for a later adapter, including short operations and
advanced error state. Proposed native persistence uses a versioned data snapshot
and reconstructs the suffix order on read. It must preserve trailing reader data,
return advanced handles on every outcome, retry valid short writes, reject
zero-progress writes and report truncated/invalid lengths. This is not the
reference's private varint/chunk wire format or its mutable receiver behavior.
The oracle records roundtrips, truncation progress and the actual short-writer
behavior without making those observations required Cinch failure behavior.
No persistence capability is claimed implemented yet.

Prepare deterministic input cases without compiling anything:

```sh
python3 experiments/suffixarray-v1/generate.py --prepare-only \
  --output experiments/suffixarray-v1/cases.json
```

After a resource handoff, generate direct SDK observations with the pinned binary:

```sh
python3 experiments/suffixarray-v1/generate.py \
  --go /tmp/cinch-go-current/go/bin/go \
  --output experiments/suffixarray-v1/vectors.json
```

The generator records the SDK binary/version, source and generator hashes. It
uses a naive independent overlapping scan to cross-check every unrestricted
literal query, and a direct bytewise suffix sort to record the complete order.
No private implementation fields are inspected. Generated output is capped at
4 MiB so this entire first checkpoint remains below 5 MiB.

The later qualification matrix must include empty/all-byte inputs; repeated
prefixes; NUL and malformed UTF8; 7/8, 15/16/17, 31/32/33 and 63/64/65 byte fringes;
huge limits; all constructor/snapshot/clone aliases; temporary owners; callback
retention, effect ordering and errors; and 8 MiB program-stack stress. Compile
with the exact immutable C2/d91 pair, preserving normal/poison runs separately.
Actual full host/wasm/ARM/PPC objects must demonstrate the public lookup path
reaches the qualified word/vector comparator, with no newly hidden writes.

The benchmark plan separates index construction, repeated lookup on one retained
index, construction-plus-query, sparse/dense overlap output consumption, and
search-from callback traversal. Use 32/1024/65536 byte random, repetitive and
binary corpora; query lengths 1/7/8/15/16/17/33; hit/miss and common-prefix cases;
and all/bounded results. Consume every position plus a retained-input hash outside
the clock, and keep both index and source owners alive through the end clock.
Compare the same public pipelines with the pinned reference and a linear-search
control. Record normal-RC allocation totals independently. No timing or speed
claim precedes meaningful functional preflights and an exclusive timing grant.
