# Tabwriter source candidate and current evidence

This is an isolated implementation checkpoint, with **no production selection or full-package claim**. The nineteen cumulative SDK records still map to twelve current identities; historical constant-typing records remain explicit duplicates. All nineteen now name concrete source operations in `capability-plan.json`.

`Text.Tabwriter` provides checked configuration, exact fresh output offers, complete byte formatting and checked valid-text conversion. `Effect.Tabwriter` provides initialized alias-shared writers, reset, incremental writes, flush and a live writer-as-output capability. Its output callbacks preserve signed reports, original errors and advanced handles. I/O-only and combined I/O/mutation callbacks have separate constructors; operations that may invoke either retain both effects. Initialization and allocation need only I/O. Storage constructors and arrays are private. A stream adapter is prepared in the working lane for separate qualification; it is not part of this checkpoint.

The parser snapshots caller input, retains checked private spans and decodes a pending width segment only after all its fragments arrive. It therefore counts a scalar split across writes once, while malformed byte progress remains exact. Complete eight-byte intervals use a straight-line SIMD classifier; scalar fringes retain exact bounds. HTML and raw escape modes retain all required output bytes. Completed cells expose only fresh copies.

Layout uses an explicit work stack over immutable row tails. Each contiguous column block computes its width once, descends into cell tails and later emits complete original rows with the resolved width path. Padding stays a compact natural count until the next underlying offer; no huge constructor list is generated for a large width. Checked arithmetic rejects coordinates beyond the supported signed 64-bit interval. Padding arithmetic occurs at its actual left/right output position, preserving preceding text offers when a later padding calculation refuses.

The writer stores an advanced output after every callback, including errors. Original errors win over a mismatching report. A report unequal to the offered length with no original error produces the short-write family; an untrusted report never becomes an index or subtraction operand. Failed automatic flushes retain parsed rows and the active width path for exact replay. An explicit flush resets those rows even after failure. Initialization replaces pending data and output without flushing or closing either borrowed output. Same-call reentry is a named refusal; ordinary operations on one shared writer require external synchronization.

## Qualified evidence

`candidate-v3/gates.json` and the SDK-free `verify_candidate.py` bind:

- All **549** complete-input literal reference cases, checking both public byte output and every underlying offer, including empty offers. All 23 source/runtime units pass the exact 330/d91 compiler/runtime pair in normal 32 MiB and poisoned 256 MiB heaps with an 8 MiB managed stack, empty stdout and zero live/leaked objects.
- All **51** ordinary streaming cases. They check each operation's consumed input and public error family, cumulative underlying calls, signed reports, physically accepted prefixes, retained offer buffers and advanced capability epochs. Both memory profiles pass. The earlier six- and twelve-case receipts remain unchanged in `candidate-v1` and `candidate-v2`.
- All **12** explicit adaptation policies pass the same 330/d91 source and O2 runtime tuple in both fixed zero-leak profiles. `controls-v1/adaptation-gates.json` and `verify_adaptations.py` bind each original case identity to its documented native contract. Constructor/initialization refusals, typed callback errors, mathematical overflow behavior and initialized-only representation remain explicitly distinct from the recorded panics or missing return counts.
- The three `ErrorOwnerMain` groups pass actual O2 runtime in both fixed zero-leak profiles: exact NaN payload, negative zero and 128 callable errors retaining owned bytes after local writer factories return. `controls-v1/runtime-gates.json` and `verify_controls.py` bind the source, executed runner, actual managed environments, minimum sampled disk headroom and verified success archive. These observations do not claim automatic cycle reclamation.
- Four genuine source refusals preserve actual effect-row mismatch, distinct-region mismatch and both private writer/storage constructor boundaries, with the exact first diagnostics in `refusals-v1`.
- Genuine defining metadata records `TabOutput` and `TabWriter` with an explicit Region parameter and ordinary error-type parameter, preserves combined effects on write/flush/dispatch and excludes writer/output/storage constructors from the public value surface.

Earlier configuration, split-scalar, escape/HTML, input/export-alias and writer reset/shared-handle controls also pass their respective exact source epochs. Their receipts remain plain in the owned build tree. The preserved earlier failures include layout syntax, a fixture byte-index obligation, explicit refinement widening and the SDK observer's cache-growth budget failure. None is relabeled as a compiler or runtime success.

All completed successful ELF/IR pairs have verified durable recovery archives. The 23 completed pure input epochs are sealed against any future compiler write; their byte/size/mode-identical source/CIF copies are plain hardlinks under the exact reviewed consolidation ledger. Original per-path timestamps are recorded, aliases share canonical timestamps and no metadata was restamped. Failed, frozen, provider, runtime and benchmark inputs remain untouched.

## Remaining full-package work

All 51 ordinary streaming cases now cover later failure checkpoints, explicit-flush recovery, empty/short output calls, malformed signed reports, reset aliases, distinct new handles and bounded huge-width output. All twelve explicit adaptation controls now pass source and runtime on the exact Rust330/d91 tuple. Their original panic observations and adaptation differences remain preserved; the independent gates below stay open.

The twelve original panic/uninitialized cases require explicit native controls rather than invented equivalent returns:

| Reference observation | Qualified native policy on Rust330/d91 |
| --- | --- |
| Three negative constructor coordinates | `tab-config` returns a named coordinate error before any writer is constructed. The complete mapping retains the observed constructor panics. |
| Three invalid initializations | Checked configuration is produced before `tab-writer-init` can run. The qualified control verifies that refusal leaves the old alias, pending rows and output unchanged. |
| Three panicking output callbacks | Total native callbacks return typed original errors. Qualified controls distinguish this checked outcome from a reference panic with no returned count, while checking preserved replay and exact offered bytes. |
| Maximum padding overflow | An out-of-range mathematical width becomes `TabLayoutOverflow`, with a defined input checkpoint and explicitly documented output-prefix policy. |
| Maximum tab-width intermediate overflow | Exact mathematical arithmetic can produce a valid bounded tab count where the captured machine implementation panics. Qualified controls check the defined native output without claiming panic equivalence. |
| Uninitialized zero receiver | Native handles have only an initialized opaque factory. Initialized empty behavior and private-constructor refusal are qualified; the observed uninitialized behavior stays an explicit representation difference. |

Remaining gates include direct valid-text/invalid-output ownership, remaining retained-layer callbacks, deep/wide tables, live stream composition, actual production-kernel exhaustive/foreign-object checks, full Native comparison and public paired benchmark/codegen review. No existing buffered-I/O callback contract or known 22-object cycle failure is changed by this work.

Performance review will measure complete byte formatting and incremental writer pipelines separately, with independent checksums and fresh-copy controls. One static candidate for later investigation is duplicate coalescing of a plain cell's width segment and its final contents. It is not adopted or described as a measured regression; the current source and exact qualification inputs remain fixed until a benchmark justifies an affected change.
