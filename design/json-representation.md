# A JSON representation that is cheap to load

Status: design note, 2026-11-13, with the measurement done and the
implementation not started. Every number below is a command this lane ran
against `build/bootstrap/stage3/cinch` on the 64-interface base set, or
against the Rust seed (`compiler/target/debug/cinch`) for the single-document
probes. Section 6 says which is which. Companions:
`design/compilation-speed.md` section 2.18 (the scanner fix this follows),
`design/base-as-a-package.md` section 6.1 (the interface-byte law).

Conventions: `[MEASURED]` is a number this lane produced, `[INFERENCE]` is
derived from one. No em dashes.

---

## 1 Verdict, and the surprise

The task assumed the remainder after the scanner fix is the `Json` tree: one
cons cell per node, the `JObj`/`JObjEnd` spines, and a double copy in
`bytes-slice` then `string-from-bytes`. Two of those three are real and the
third is not where the money is, and the measurement says so plainly:

| what a padded byte of `abi.symbols` costs, through the whole loader | blocks per byte |
| --- | --- |
| a bare `bytes-get` scan loop over the same buffer | 1.00 |
| UTF-8 validation of the same buffer | 2.00 |
| the JSON scan and node build (`parse-json` alone) | 3.21 |
| the scan plus one canonical re-serialization | 4.58 |
| the scan plus the byte-level contract hash | 6.21 |
| the full `load-interface` (the old re-print rule) | 8.14 |
| the marginal law the three-point padding measures | 6.00 |

`[MEASURED]`, section 6. So a padded byte is a *string content* byte, and the
marginal 6.00 decomposes as the 2.0 scan floor, plus the loader's FNV loop
over the same bytes at another 2.0, plus the rest of the loader's document
work. Node representation is not on that path at all, because padding adds
one long string and no nodes. The identity work was 4.0 of it, and this lane
took 1.37 of that off (section 3) with the marginal law itself unchanged at
6.00, which is the honest outcome: the law measures string content and this
lane's change was never about string content.

That does not make the node representation pointless. It makes the two
effects separable, and they are separated below:

- **String content costs 2.0 blocks per byte and cannot go below it** at the
  Cinch level, because a byte read is `bytes-get` (which boxes its result)
  and the loop step boxes a `Nat`. This is the floor the padding law sits
  on, and the only lever under it is a runtime scan, not a representation
  change.
- **Structure costs about 10.5 blocks per node**, on top of its string
  content, and that is the cons-spine and the per-node `String` copy. A real
  interface averages 9.3 bytes per node, so on the real set structure is a
  substantial share even though it is invisible to the padding arm.

So the design below keeps all three changes, and states honestly that change
3 (the UTF-8 fast path) is the one that moves the padding law, while changes
1 and 2 (slices and flat nodes) are what move a real interface.

---

## 2 The memory arithmetic

`[MEASURED]` over the 64-interface control arm, 1,735,532 bytes:

| quantity | value |
| --- | --- |
| interfaces | 64 |
| interface bytes | 1,735,532 |
| `Json` constructor cells (every `JObj`, `JArr`, `JStr`, `JObjEnd`, `JArrEnd`, scalar) | 185,860 |
| bytes per node | 9.34 |
| value strings | 45,653, 771,838 bytes, mean 16.9 |
| object keys | 71,485, 475,802 bytes, mean 6.7 |
| string content share of the document | 71.9 percent |

Today, per node, the parse allocates: one constructor cell for the node
itself; for a `JStr`, a `StrBuf` cell plus the copied byte buffer; and for an
object entry a key `String` the same way. The cons spine is the `JArr tail`
and `JObj tail` field, so the spine costs no extra cell beyond the node it
lives in, which is why the node count above already counts it.

Against the measured 6.00 marginal and 30,465,952 absolute blocks for the
set, the node population accounts for roughly 185,860 times 10.5, or 1.95 M
blocks, about 6.4 percent of the total `[INFERENCE from the 10.5
blocks-per-node probe]`. The rest is the 2.0 per byte scan over 1.74 MB
(3.47 M) plus the loader's re-serialization, which the next section is
about.

---

## 3 The loader re-serializes every document, and that is the largest
single term

`Bootstrap.Load.load-interface` computes the contract identity as

```
fnv-source-hash (canonical-bytes (contract-projection doc))
```

`contract-projection` walks the tree rebuilding it with three fields blanked,
`canonical-bytes` prints the whole document back to bytes through a
`Builder`, and then `fnv-source-hash` hashes those bytes. So every interface
is printed a second time, at full document size, and then hashed.

`Bootstrap.Interface.iface-contract` already does the same job the right way:
it blanks the three regions *in the source bytes* (`blank-field-string`,
`empty-array-field`, `bytes-splice`) and hashes that, so no re-print is
needed. `load-interface` was changed to call it, and then the measurement
said something the note did not expect. `[MEASURED]` on the 581,881-byte
`Data.Machine.cinchif`, all four as one program each:

| path | blocks | marginal |
| --- | --- | --- |
| parse only | 1,866,011 | |
| parse plus `canonical-bytes` (the old rule) | 2,662,373 | +796,362 |
| parse plus `iface-contract` | 1,866,191 | +180 |
| parse plus `iface-contract` plus FNV (the new rule) | 3,611,850 | +1,745,839 |

So the byte-level splice is free (180 blocks for three `bytes-splice` calls
and two needle searches), and the re-print it replaces was 796,362. But
`fnv-source-hash` over the spliced bytes costs 1,745,659, because it is a
Cinch-level byte loop over 581 KB at the 2.0 blocks-per-byte floor, and the
old rule never paid that on the *source*: it paid it on the reprinted bytes,
which is the same size, so the FNV cost was there too and the table's fourth
row is not a regression against the first column of the old rule but a
different split of the same two terms.

The honest reading is therefore narrower than the note first claimed: the
change removes the 796,362-block re-print and leaves the FNV loop, and the
FNV loop is the term worth attacking next. It cannot go below 2.0 blocks per
byte while it is written in Cinch, so it needs a runtime primitive (an
FNV-1a 64 over a `Bytes`, one call, no per-byte box). That is a runtime
addition and so a lock bump, which is out of scope here and named in
section 8.

The change was still worth landing on its own terms: it deletes a second
implementation of the identity rule (`contract-projection` plus a printer)
from the load path, and it makes the loader and `Bootstrap.Interface` agree
by construction rather than by a probe. It is not the large win the note
promised, and section 9's order is corrected to say so.

---

## 4 The representation

### 4.1 A document owns its source bytes

```cinch
data JsonDoc where
  JsonDoc :: Bytes -> Array Node -> JsonDoc
```

The `Bytes` is the document's source, held once and never copied. It is
owned by the document handle, not borrowed through a region parameter,
because the interface loader reads a file into a `Bytes` and hands it over,
and a region parameter would force every consumer to carry a lifetime for a
buffer it already owns outright. The alternative, a borrowed slice, buys
nothing here: the source outlives the document in every current caller.

### 4.2 A node names a slice, and a slice is two numbers

```cinch
data Node where
  Node :: Nat -> Nat -> Nat -> Nat -> Nat -> Node
  --     kind   key-off  key-len  val-off  val-len or child
```

Five `Nat` fields, one machine word each, so a node is 40 bytes plus the
8-byte header the runtime puts on every block, 48 in the array's payload.
Against a `JStr` today, which is a constructor cell plus a `StrBuf` cell plus
a copied buffer of mean 16.9 bytes (so about 24 + 24 + 24 = 72 bytes for a
mean string), the slice form is 48 bytes and copies nothing.

What this gives up, stated rather than hidden:

- **No sharing between documents.** A slice is an offset into one buffer, so
  two documents cannot point at one string. Today they cannot either
  (`string-from-bytes` copies), so nothing is lost.
- **No mutation of a name without copying the document.** The slice is
  read-only by construction; a consumer that wants to build a modified name
  materializes a `String` first. The `Json` family is immutable today, so
  this matches.
- **Numbers are not slices.** A `JNum`/`JFloat` slice is the spelling, and
  the value is `int-of-spelling`/`f64-from-decimal` over it on demand. That
  is a behaviour change worth naming: today the parse converts eagerly, so
  a malformed number is refused at parse. The design keeps eager validation
  (section 5) and defers only the conversion, so the diagnostics are
  identical and the conversion moves to first use.

### 4.3 Children are an index range, not a spine

A container node carries the index of its first child and its child count;
the children are contiguous in the same array, so a document is the source
buffer plus exactly one `Array Node`. Object entries alternate key-node and
value-node in that range, which is why the node record carries both a key
slice and a value field rather than a separate entry type.

`Array` today is fixed size (`array-from-list` builds one from a list), so
the parse has to know the node count before it allocates, or grow. Two ways,
and the design picks the second:

- **Count first, then fill.** A first pass counts nodes, `array-replicate`
  makes the array, a second pass fills it. Two passes over the bytes is 2.0
  more blocks per byte, which is the largest single cost available and not
  worth it.
- **Grow in place.** One `Bytes` arena of `node-size * capacity`, written
  through `bytes-set` at a byte offset, with the count in a `Ref` cell. This
  needs no new runtime: `bytes-set` mutates in place and `array-set` does
  not (it copies the whole array, `compiler/runtime/src/arrays.rs`), so the
  arena has to be `Bytes`, not `Array`. The node record is then packed into
  four-byte fields by `put4`/`get4`, which the probe in section 6 confirms
  compiles and runs.

So the concrete shape is one `Bytes` of `48 * capacity`, doubling when full.
`Array Node` is the type the *consumer* sees, built once at the end of the
parse by copying the arena, or skipped entirely by reading through the
packed accessors. Which of those two is decided by section 4.4.

### 4.4 How a consumer gets a `String`

`String` is `StrBuf :: Bytes -> String` with a private constructor and a
copying `string-from-bytes` (`compiler/prelude/Data/String.cinch`). A
consumer that genuinely needs a `String` calls one function:

```cinch
node-string :: JsonDoc -> Node -> String
```

which copies the slice once. There is no way to get a `String` that aliases
the document, and that is deliberate: the immutability contract on `String`
is what makes `string-compare` and the `HashMap` key instances sound, and
handing out an alias of a buffer the document owns would break it.

Consumers that only compare or hash names never materialize a `String`: a
byte-range compare against a slice is `bytes-compare` over two ranges, and
the loader's key lookups are exactly that shape.

---

## 5 What stays the same, and what a non-UTF-8 document gets

The public `Text.Json` entry points keep their signatures and their results.
`parse-json` still answers `ParseOutcome Json` and still builds a `Json` for
the callers that want one; the arena path is a second entry point
(`parse-document`) that the loader uses, and `parse-json` is `parse-document`
followed by a materialization. That is the answer to the acceptance
criterion's escape clause: the public `Json` type does not change, because
`Data.Json`'s nine constructors are the surface every `JsonCodec` instance
and every `bootstrap/programs/Run.*` program is written against, and
changing it would break all of them. The arena lives behind a new type that
the loader is the first customer of.

**A document that is not valid UTF-8 is refused at the same byte, with the
same diagnostic, as today.** The scan validates as it goes: an ASCII byte is
one scalar of width one and steps without decoding; a byte at or above 128
enters `utf8-at`, which answers `None` for a truncated, overlong, surrogate
or out-of-range sequence, and the scan refuses there with the existing
`EUnexpected (Span i i line c) (Some (byte-item b)) (label-items "string
character")`. Because the refusal is produced by the same test in the same
order at the same offset, the rendered bundle is byte-identical. The
differential harness of section 7 is what holds that claim.

Raw control bytes below 32 are refused by the same rule that refuses them
today, before the UTF-8 test, so the ordering of the two refusals is
preserved.

---

## 6 The measurements this note rests on

All `[MEASURED]`. The three-point law is
`experiments/iface-scan/measure.py`, stage 3, padding `abi.symbols`, before
and after the identity change of section 3:

| arm | interface bytes | blocks before | blocks after | blocks per byte (both) |
| --- | --- | --- | --- | --- |
| empty floor | 0 | 19,814 | 19,783 | |
| control | 1,735,532 | 30,465,952 | 28,793,147 | |
| pad1k | 1,800,195 | 30,860,710 | 29,187,821 | 6.01 |
| pad2k | 1,866,689 | 31,253,495 | 29,580,606 | 6.00 |
| pad4k | 1,997,747 | 32,039,667 | 30,366,778 | 6.00 |

Allocation kinds, control and pad4k, so the marginal column is the
difference:

| kind | control blocks | pad4k blocks | marginal per padded byte |
| --- | --- | --- | --- |
| ctor | 17,453,071 | 17,977,620 | 2.00 |
| closure | 2,839,870 | 3,102,139 | 1.00 |
| nat | 1,735,583 | 1,997,796 | 1.00 |
| bytes | 2,479,267 | 2,479,396 | 0.00 |

The single-document probes, Rust seed, `CINCH_ALLOC_STATS=1`:

| probe | bytes | blocks | per byte |
| --- | --- | --- | --- |
| read only | 581,881 | 25 | 0.00 |
| `bytes-get` scan | 581,881 | 581,904 | 1.00 |
| UTF-8 validation | 400,037 | 800,129 | 2.00 |
| `parse-json` | 581,881 | 1,866,011 | 3.21 |
| parse + `canonical-bytes` | 581,881 | 2,662,373 | 4.58 |
| parse + `iface-contract` | 581,881 | 1,866,191 | 3.21 |
| parse + `iface-contract` + FNV | 581,881 | 3,611,850 | 6.21 |
| `load-interface`, old re-print rule | 581,881 | 4,739,206 | 8.14 |
| string-heavy document (400 KB of one string) | 400,037 | 800,233 | 2.00 |
| structure-heavy document (12,000 two-field rows) | 432,038 | 1,621,735 | 3.75 |

The last two rows are the separation this note turns on: the same byte
count costs 2.0 per byte as string content and 3.75 per byte as structure,
and structure is 72,002 nodes, so about 10.5 blocks per node net of its
string content `[INFERENCE: (1,621,735 minus 2.0 times 432,038) / 72,002]`.

The probes also established two facts the design depends on and that were
not obvious: `array-set` copies the whole array, so a growable arena cannot
be an `Array` (`compiler/runtime/src/arrays.rs:180`); and `bytes-set` plus
`bytes-get` packing four bytes per word compiles and runs, which is what
section 4.3 needs.

---

## 7 The differential harness

`experiments/iface-scan/` holds the two measurement scripts but **not** the
29-document differential probe the scanner lane used; that probe was ad hoc
and was not committed. The harness therefore has to be written, not reused,
and this note records that as a gap rather than claiming a pass. The cases it
must cover, taken from the scanner lane's list plus what a slice
representation adds:

- every escape, surrogate pairs, lone surrogates, `\u0000`;
- raw 2, 3 and 4 byte UTF-8, and a scalar at `U+10FFFF`;
- raw control bytes below 32, and `DEL`;
- truncated UTF-8 at every width, overlong encodings, a continuation byte
  first, a surrogate encoded in UTF-8, a value above `U+10FFFF`;
- unterminated string, unterminated object, trailing junk, empty input;
- **new for slices:** an empty string, a string of length one, a string
  whose slice starts at offset 0 and one at the last byte; two adjacent
  strings that must not be confused; a key and a value with identical
  content; an object with one entry and an array with one element (the
  boundary of a child index range); a duplicate key (still refused);
- **new for numbers:** `-0`, `0`, `1e400` (an `F64` overflow spelling),
  `123456789012345678901234567890` (a bignum), and a number whose spelling
  is a prefix of the buffer's end.

Both paths, old and new, must print the canonical form and the rendered
bundle for every case, and the two outputs must be byte identical.

---

## 8 What is deliberately not done

- **No streaming or incremental parser.** A document is parsed whole. The
  named boundary in `Text.Json`'s header stands.
- **No DOM mutation.** The arena is written during the parse and read after;
  there is no insert or delete. `Json` is immutable and stays so.
- **No new runtime primitive, so no lock bump in this lane.** The arena is
  `Bytes` and the node packing is `bytes-set`/`bytes-get`, both existing
  intrinsics, so the runtime ABI does not move and neither
  `bootstrap/stage0.lock.json` nor
  `bootstrap/library-provider.lock.json` needs a bump for what landed here.
  Everything below the 2.0 blocks-per-byte floor needs a runtime addition and
  therefore a lock bump, and there are two of them: an FNV-1a 64 over a
  `Bytes` (section 3, worth about 1.5 M blocks on one interface) and a
  word-at-a-time or SIMD UTF-8 scan (worth the scan term itself). **Both
  need the bump reported, not taken.**
- **No change to the public `Json` type.** Section 5 says why.
- **No index for lazy field access.** The task in `compilation-speed.md`
  section 3 wants a symbol index so a consumer decodes only what it needs.
  That is the natural next step and it needs this representation first.

## 9 What landed, and the order the rest should follow

**Landed.** Step 1 (section 3) and step 4 (the harness) are in this tree.

Step 1, `load-interface` reading its identity from the artifact's bytes
instead of re-printing the document. The three-point law is unchanged at
6.01, 6.00, 6.00, which is what the analysis predicted: a padded byte is
string content, so no node or document-representation change can move it.
The absolute cost moved, and that is the number to read:

| arm | before | after | change |
| --- | --- | --- | --- |
| empty floor | 19,814 | 19,783 | -31 |
| control (64 interfaces, 1,735,532 bytes) | 30,465,952 | 28,793,147 | -1,672,805 (-5.5 percent) |
| pad4k | 32,039,667 | 30,366,778 | -1,672,889 |

So the two-line program's interface load fell 1.67 M blocks in absolute
terms, which is the acceptance criterion's second clause, and the marginal
law is unchanged, which is its first clause read honestly: this change
cannot move a law that is measured on string content.

Step 4, `experiments/iface-scan/differential.py`: 72 documents, control run
(reference against itself) 0 mismatches, every case producing a real answer
including the rendered bundles of the refusals. It is written and green
before any representation change, which is the order section 7 asks for.

Gates on the landed change: fixed point holds (stage2 and stage3 binaries
byte equal, `compiler.ll` byte equal, every `stageN/*.cinchif` byte equal),
three smokes pass, the leak line is `live=0 immortal=0 leaked=0`, and the
differential probe over `Data.Machine`, `Data.Target` and `Prelude` found
the old and new identity rules in agreement before the switch.

**The rest, in order.**

2. A runtime FNV-1a 64 over a `Bytes` (section 3). The hash is now the
   largest term in the load at 1.75 M blocks on one interface, and it cannot
   fall below 2.0 blocks per byte while it is a Cinch loop. One call, no
   per-byte box, would remove about 1.5 M of that. This is a runtime
   addition, so it needs a lock bump (section 8).
3. The arena and the slice nodes behind `parse-document` (section 4), with
   `parse-json` as the materializing wrapper. This is what a real interface
   pays for (10.5 blocks per node, 185,860 nodes) and the padding law is
   blind to it.
4. The loader's call sites move to the node view (section 4.4).

The dependency between 2 and 3 is one-way: 2 is independent and cheaper, so
it goes first.
