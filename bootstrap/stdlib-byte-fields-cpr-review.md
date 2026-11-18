# UTF8 owner touch and constructed-product returns

The experiment is separate from the published Fields measurements. Production
Fields remains `1fe6b9ac…` and UTF8 remains `f1e87cec…`. The source snapshot
`benchmarks/stdlib/byte-fields-utf8-reassociated.cinch` has hash `f516885a…` and changes
only the valid, nonempty branch of `utf8-decode-between`:

```cinch
-- Original placement:
hold-source source (packed-step (decode-raw base size offset))
-- Experimental placement:
packed-step (hold-source source (decode-raw base size offset))
```

Both forms evaluate strictly. `decode-raw` completes its bounded raw loads and
returns a copied U32. `hold-source` then reads `bytes-len source`, preserving the
owner through that scan. `packed-step` interprets only the copied U32: it masks
the scalar, validates/converts that number, converts the consumed width and
selects a status. It accesses no source pointer and invokes no caller callback.
Moving that numeric interpretation after the owner touch therefore leaves no
raw access beyond the owner's last required use. The step's three observations
do not retain the input buffer.

The existing current Rust706/d91 baseline IR constructs `DecodedStep`, reads
`bytesLen`, and returns the saved product pointer. The owner-touch helper has
already been inlined. The relevant remaining obstacle is a late local-variable
return: the current `cpr_units` tail-leaf analysis recognizes direct constructor
applications and known product-returning calls, but does not follow that saved
product through the intervening owner touch.

The experiment enables existing machinery without any compiler or Root edit.
The emitted decoder and sealed-view adapter now have three-word aggregate
entries, and their public boxed entries remain available:

```llvm
define { i64, i64, i64 } @"Data.Bytes.Utf8.utf8-decode-between#cc"(...)
define { i64, i64, i64 } @"Data.Bytes.View.byte-view-utf8-at#cc"(...)
```

In the actual decoder aggregate body, the full-word/tail loads and `decode-word`
return precede the final `bytesLen` touch. The shared continuation then truncates
the copied result to i32, calls `packed-step#cc`, and returns its aggregate. There
are no later source-pointer loads. Exact IR hashes, instruction sequences and
caller identities are retained in
`build/byte-fields-cpr-probe/aggregate-lifetime-review.json`. The probe also keeps
the entry module's lowered Core; a dependency-module Core dump is a separate
stage and is not claimed by the entry dump.

This does not eliminate the Fields decoded-step allocation. The aggregate view
adapter forwards the decoded aggregate directly, but all three decoder/view
call sites in each Fields scanner reconstruct `DecodedStep`. The residual
scanner path then invokes non-inlined scalar/width getters. The exact built
benchmark IR and linked O2 assembly confirm that the product allocation moves
into the caller rather than disappearing.

A general compiler improvement could recognize a fresh, nonescaping local
constructed product returned after strict sequencing. It must preserve the
original field evaluation point, every owner touch, and field-by-field reference
counts. Extending inference alone is insufficient: `virtual_let` and return
lowering must agree on the virtual product. Escapes, captures, opaque uses,
unknown aliases and sharing operations require the existing boxed behavior.
Globally treating polymorphic return-argument helpers as one fixed product, or
annotating owner reads as memory-free identities, would be unsound. The narrower
source placement is preferable if it passes the required semantic/lifetime and
performance gates.

Qualification requires independent malformed/truncated/bounded status and width
checks; temporary-owner and mutable-alias observations; huge-bound refusal before
narrowing; early-stop and saved-width mutation callbacks; escaped/reused step
values; and repeated allocator churn under normal/poison 8 MiB program stacks.
A future generalized compiler test must also use a product containing an owned
large Nat or byte owner: UTF8's small widths alone cannot establish ownership
correctness for arbitrary product fields.

No aggregate-return proof is a throughput claim. Full owner/span and Fields
controls, plus a fresh matched baseline/candidate public-pipeline comparison with
all regressions and generic callback controls retained, precede any production adoption.

The completed functional receipt is
`bootstrap/stdlib-byte-fields-cpr-functional-results.json`. On the matched
Rust706/d91 tuple, both baseline and candidate pass the complete independent
bounded-span corpus: 147,105 spans and 1,028,230 decoded observations each. Each
variant runs one full normal process and 189 bounded poison processes. The
existing alias/temporary-owner and huge-bound controls add eight matched
processes. Candidate Fields passes all 1,266 rows plus mutable callback, early
stop, factory replay, deep traversal and distinct IO/Mut controls in 342 further
processes. All 730 processes report zero leaks; the CLI compilation profile has
checked-RC instrumentation disabled and runtime poison is enabled separately.
The separate public benchmark preflight adds 650 matching output/retained-owner
observations, all zero leaks, using the same Rust706/d91 tuple. Its built LLVM
has the same hash as the original aggregate/lifetime probe. Current candidate
checked-RC API and historical Native qualification remain unclaimed.

The fresh matched comparison completed all 104 observations in 2.66 seconds:
13 workloads, two binaries, one excluded warmup and three rotated/interleaved
samples. The raw receipt is
`bootstrap/stdlib-byte-fields-cpr-benchmark-samples.json`. No cgroup throttling
occurred while all peer heavy work was paused. Dense eager, Unicode eager and
the generic predicate workload regress in their sample medians; producer dense
improves and visitor dense is near neutral. The microsecond ASCII workloads
retain large individual outliers. Three samples cannot establish the cause of
every small difference, and the mixed result does not support production
adoption. The candidate remains unselected; production UTF8/Fields is unchanged.

The linked assembly review counts static constructor allocation call sites:
12 to 15 in each scanner and four to zero in the decoder. These are static code
sites, not dynamic allocation totals. A further compiler experiment would need
to carry the known product through the field-projector calls and late locals;
the owner-touch tail rearrangement alone does not solve the consumer's boxing.
