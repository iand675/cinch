# Buffered sources with I/O and mutation

This is a review experiment. The maintained prelude remains byte-for-byte equal to `8d0557bd`; the generated copies here are not promoted APIs. `generate.py` verifies the frozen inputs and records every old/new source hash and exported signature in `signature-impact.json`.

The new `buffer-source-with-mutation` factory stores a callback with `{IO r, Mut r}` under a new private source variant. The same region identity appears in both effects. Existing IO-only raw source construction remains available, but a general `BufferSource r` can now advance to either kind of continuation. Its consumers must conservatively retain both effects, including when a particular call happens to use buffered bytes only.

The exact existing public signature impact is:

| Home | Existing signatures affected |
| --- | --- |
| `Effect.Buffered.Core` | `buffer-source-with-transfer`, `buffer-sink-with-transfer`, `buffer-source-read`, `buffer-source-transfer`, `buffer-sink-transfer` |
| `Effect.Buffered.Reader` | `buffered-read`, `buffered-read-into`, `buffered-peek`, `buffered-discard`, `buffered-read-byte`, `buffered-read-rune`, `buffered-read-slice`, `buffered-read-bytes`, `buffered-read-line`, `buffered-read-string`, `buffered-write-to` |
| `Effect.Buffered.Writer` | `buffered-read-from` |
| `Effect.Scanner` | `scanner-next` |

These are 18 existing signatures plus the new factory. None previously required both effects. Reader/Writer/Scanner getters and configuration remain pure; ordinary sink writes, text writes and flushing retain their IO-only signatures. The ReaderFrom shortcut now accepts a read callback that also requires mutation. Widening only its enclosing transfer operation would leave an invalid higher-order effect erasure.

`Main.cinch` checks that an arbitrary genuinely both-effect callback can flow through the ordinary reader, writer and scanner APIs and through a sink's ReaderFrom capability, while independent writing/flushing still typecheck in an IO-only context. It is a declaration gate, not an executable adapter test.

The motivating live operations have these actual shapes:

```text
multipart-part-read : MultipartPart r e -> Nat
  ->{IO r, Mut r} (Bytes, MultipartReadStatus (MultipartError e))
multipart-file-read : MultipartFile r e key fileHandle state -> Nat
  ->{IO r, Mut r} (Bytes, MultipartReadStatus (MultipartError e))
```

Their adapter must retain the actual part/file handle, forward each read at invocation time, preserve a positive byte prefix independently of `More`, `End` or `ReadFailure`, and return its next adapter over that same live handle. `End` maps to `BufferEnd`; failure uses an explicit caller-supplied `MultipartError e -> BufferError` mapper. Native opaque `e` is unchanged. The returned bytes are already fresh; no eager snapshot substitutes for the live source. Dropping the adapter does not close a borrowed file. The native raw part's zero-read behavior may prefetch; the buffered reader's existing zero-read policy remains inert and must be documented separately.

`check.py` freezes the source closure, overlays only these four experimental modules, and records exact 855/c4 or 2f/interface identities and diagnostics. It does not alter a provider or build a compiler. The Native source entry uses raw `--check --interfaces` with two positional paths, without `--objects` or IR emission.

This fixed both-effect model has a compatibility cost: existing IO-only callers of these consuming methods need the additional effect. A general row-indexed source is an alternative only after its support is demonstrated. This experiment makes no such assumption. Runtime, effect-refusal, live adapter, callback/owner and complete-corpus gates follow API review. The separately recorded 22-block retained-callback cycle still blocks promotion; no runtime collection behavior has changed.

Both complete source/type/proof gates are now green in `gates.json`: actual frozen 855/c4 and raw 2f with its admitted interfaces. This supersedes the creation-time pending status recorded in the immutable checked manifest. The earlier private transfer-callback annotation diagnostic is retained. No runtime, effect-refusal, real multipart adapter or promotion result is implied by these declaration gates.

The separate `composition-gates.json` now records twelve additional exact source gates. Both frontends accept the concrete `MultipartAdapter.cinch` composition through ordinary Reader/Writer/Scanner consumers, and both accept the complete `TransferPriority.cinch` assertion bodies. Eight negative gates reject the four deliberate attempts to erase mutation: the legacy IO-only source constructor, an IO-only general reader, an IO-only ReaderFrom callback invocation, and an IO-only native multipart-part read. These are source checks, not executed assertions. Verify the saved compiler verdicts without repeating compilation:

```sh
python3 experiments/buffered-both-effects-v1/verify_composition.py
```

The Native concrete composition uses Compare's admitted 2f/V2 snapshot with interface hash `229ee932...`, object-manifest hash `36d5ec97...` and runtime `80bd5b54...`, plus its exact frozen multipart source closure. The prior whole-family source check used the distinct `1d22...` interface epoch; these receipts retain both identities separately. `check.py --native-snapshot` and `--dependency-snapshot` select the explicit matched tuple and source closure. No snapshot or provider is modified.

The first Native legacy-constructor check correctly reported `type mismatch: expected {(IO r)}, found {(IO r), (Mut r)}`. Its original harness expected the words “effect” or “row” and therefore rejected the wording. The source/compiler result is preserved; the corrected verifier recognizes that precise effect mismatch without rerunning the compiler or accepting an unrelated parse/import failure.

The priority fixture asserts source WriterTo before sink ReaderFrom, the sink fallback when WriterTo is absent, delegation from an empty writer, and a pending prefix that fills/flushes before delegation starts at the advanced destination. Returned capability drops and counts excluding the earlier prefix are observable.

These four priority/advanced-destination groups now also execute successfully under both 855/c4 and admitted 2f/V2/80bd with explicit LLVM20 O2 links. Normal32MiB and poisoned256MiB heaps both use the fixed8MiB stack; all four processes report zero live/leaked blocks. `run.py` reuses exact positively source-gated input closures, limits each runtime unit to20MiB of new artifacts, and enforces the standing250MiB start/200MiB reserve guards. The earlier guard refusal produced no IR or executable and remains preserved. Recheck the successful exact artifact receipts without rebuilding:

```sh
python3 experiments/buffered-both-effects-v1/verify_runtime.py
```

`runtime-gates.json` records the distinct source/compiler/provider/runtime identities and all four memory outcomes. Actual live native alias, prefix-plus-error and caller-owned file-close runtime cases, retained callbacks and the complete functional corpus remain pending. The known retention cycle still blocks promotion; no collector or callback-lifetime behavior changes.

Closed qualified ELF/IR outputs may be preserved in the verified `build/buffered-completed-functional-v3.tar.gz` recovery archive. The verifier streams indexed artifact bytes and checks their hashes, sizes, modes and unchanged report identity; it does not need to restore large outputs. All source, logs, receipts and benchmark inputs remain plain.
