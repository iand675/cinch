# Self-describing blocks: no layout table, no layout ids

Status: decided 2026-11-14 (the user chose the Lean/Koka direction over a
startup layout registry). Both phases are in the tree at `runtime_abi` 10:
the runtime, seed compiler and evaluator (phase 1), then the stage
compiler's lowering with both locks re-pinned (phase 2, section 8).

## 1 The problem

A block's header carried a 16-bit layout id: an index into one layout table
(payload size and pointer map per entry) that generated code registered at
startup. Ids were handed out positionally per program, so two separately
compiled artifacts gave one type different ids, and every artifact linked
into one program had to agree on numbering. Layout ranges per artifact
(`design/reclamation.md` section 3) made that sound but not simple: the
ranges came from install order, two objects from different projects could
collide, and every linked artifact laid out the prelude's types again in
one shared space of 1,023 groups.

## 2 What other runtimes do

Nobody numbers layouts across compilation units.

| Runtime | Per block | Across units |
| --- | --- | --- |
| Lean 4 | header: 32-bit count, tag, field count, size; boxed fields first | nothing |
| Koka (Perceus) | header: 32-bit count, 16-bit tag, 8-bit scan count; boxed fields first | nothing |
| OCaml | header: size and 8-bit tag, every field a tagged word; custom blocks point to a static ops record | nothing |
| GHC | an info pointer to a static table beside the code | the linker |
| Go, Swift | a pointer to a per-type descriptor, deduplicated by name | the linker or the runtime |

Two families: blocks that describe themselves, and blocks that point to a
static descriptor. Cinch takes the first for the common case and the second
for the rare one.

## 3 The header

```text
bits  0..34  rc      35-bit signed count (RC_IMMORTAL, RC_FRESH, frame markers as before)
bits 35..39  kind    5 bits (18 kinds today; KIND_FREED moves to 31)
bits 40..63  layout  24 bits
```

The count stays far from its markers: the heap cap (8 GiB by default)
bounds live words at 2^30, and `CINCH_HEAP_LIMIT` is clamped to 64 GiB
(`HEAP_LIMIT_MAX`), where heap references use at most half the count range
below the frame markers and the other half is left for stacks.

For a shaped kind (constructor, closure, dictionary) the layout field is:

```text
bits 40..47  tag    a constructor's tag; 0 for a closure or dictionary; 0xFF = escaped
bits 48..63  mask   bit i set: field i is counted (scanned when the block dies)
```

"Field i" is payload word i for a constructor, saved argument i for a
closure (after its three prefix words: code, arity, filled count), and slot
i for a dictionary (after its method-count word). Obj and TObj fields are
both "counted"; the release walk tests each counted word for an immediate
anyway, which is all TObj ever asked for.

For every other kind the layout field is the payload size in words, as
before, with all ones meaning the size is in the word before the header.

Fields are not reordered: a block's fields stay where code generation, the
runtime's own builders, the evaluator, boundary (`repr(C)`) layouts and the
stage compiler already put them. Only the header's meaning changes, which
is why this follows Lean and Koka in spirit (the block describes itself)
but not in mechanism (they reorder fields to store a count; a mask needs no
reordering).

## 4 The escape

A shaped block is escaped when its description does not fit: a constructor
with 255 or more tags in its type, a counted field at index 16 or beyond, or
a constructor or dictionary too large for the small-object heap (over 31
payload words, whose free needs a size). Its tag field is 0xFF and the word
before its header points to a static descriptor, which the artifact that
allocates the block emits (private, read-only, 8-aligned):

```text
word 0   the tag (a constructor's; 0 otherwise)
word 1   the payload size in words
word 2.. the counted mask, one bit per field, 64 fields per word
```

A descriptor is content, not identity: two artifacts may emit identical
descriptors at different addresses and nothing compares them. So no table,
no registration, no numbering, and nothing to coordinate between objects.

## 5 Size

A shaped block's size is not in its header. A block of the small-object heap
(256 bytes or less) is freed by address (`small::block_size`), which is
where the size lived all along; an escaped block's descriptor has it; a
closure's is its prefix plus its filled count. A mortal inline constructor
or dictionary the small heap cannot hold (its range used up, or a thread
past teardown) has its payload size recorded by address when it is
allocated and removed when it is freed (`block::FALLBACK_SIZES`), so the
system allocator's free still knows its size. A descriptor interned at run
time (by tag, size and mask) serves the one block code generation cannot
describe ahead of time: an enum lifted from canonical memory, whose tag
arrives at run time (`cinch_alloc_ctor_tagged`).

## 5.1 The entry points

- `cinch_alloc_ctor(size, field)` and `cinch_alloc_shaped(size, kind,
  field)` take an inline layout field; `cinch_alloc_ctor_desc(size, desc)`
  and `cinch_alloc_shaped_desc(size, kind, desc)` an escaped block's
  descriptor; `cinch_alloc_ctor_tagged(size, tag, mask)` decides at run
  time.
- A consumer that takes either form through one word (the quote
  evaluator's `LAM` and `CTOR` nodes, a columnar zip's pair) takes a
  description word: `field << 1 | 1` inline, or the descriptor's address.
- Code generation carries a description as one `u64`: the inline field, or
  a flag bit and the index of a descriptor it emits as a private constant
  `@"cinch.desc.N"`, deduplicated by content within the module.
- A tag read is the header's low layout byte, with one compare for the
  escape tag; it is always made, since any block (another artifact's
  included) may be escaped.

## 6 What it removes

The layout table, `cinch_layouts_register`, positional ids, the
per-artifact layout ranges and their overlap checks, the escaped-tag global
flag, and the `layout_words`/`layout_entries`/`layout_base` fields of the
object ABI. Freeing reads the mask from the header instead of an entry from
a table, so the release walk loses one dependent load per block.

## 7 Phases

1. The runtime (`block_kinds.rs`, `block.rs`, `rc.rs` and the kinds that
   read layouts), the seed compiler's code generation, and the evaluator
   move to the new header together, with a `runtime_abi` bump. The stage
   chain is unaffected: it builds its runtime from the pinned revision.
2. The stage compiler's lowering (`Bootstrap.RuntimeIr`, `Bootstrap.Object`)
   emits the same headers, then the seed and library-provider locks move to
   the new revision and the chain is rebuilt to its fixed point. Until then
   the tests that link stage-compiled code against the current runtime
   (`bootstrap_runtime_objects`, `bootstrap_library_object`) fail by design.

## 8 The stage compiler (phase 2)

`Bootstrap.RuntimeIr` carries a description as one `Nat`, the counterpart of
the seed's `u64`:

- below 2^24 it is the inline layout field, `tag + 256 * mask`;
- otherwise it is escaped, `2^24 + tag + 2^24 * (words + 2^16 * mask)`,
  with the mask over payload words (prefix included), so the number names
  its static descriptor `@cinch.desc.<n>` by content. `with-descriptor`
  emits the descriptor beside each use and `dedupe-globals` keeps one
  definition per name, as it already did for nullary static blocks.

`shape-description kind tag codes` decides between the two with the same
rule as the seed's `describe`: tag below 255, counted fields among the first
16, and a constructor or dictionary payload of at most 31 words.
`shaped-allocation` picks `cinch_alloc_ctor`/`cinch_alloc_shaped` or their
`_desc` forms; static blocks (nullary constructors, closures that save
nothing, fixed dictionaries) put the descriptor address before the header
when escaped; the columnar zip takes a description word. A tag read is the
low layout byte with the escape compare, as in the seed. The inline count
helpers shift by 29 (the 35-bit count).

`LayoutPlan` is now only a cache of each specialized family's descriptions by
tag. It is never seeded from the provider: nothing about layouts is shared
between objects. The object ABI loses `layout_words`, `layout_entries`,
`escaped_descriptors` and the per-type `group`/`base` facts; a constructor's
facts are its tag, arity, header layout field and `escaped`, and every
manifest states `self_describing: true`, which `Bootstrap.Object` requires of
a provider. The provider's String wrapper must be inline with its one field
counted (layout field 256), which is what a string literal's static header
uses.

The stage compiler keeps its existing field-count limits (16 tuple or
constructor fields, 13 closure parameters, 15 dictionary slots): they are
refusals of other lowering paths, not of the header, so the escape path is
reached today only by families of 255 or more constructors.

Both locks pin the commit that removed the JIT's and the Wasm entry's last
layout registrations (`52694cf6bd30`), seed source hash
`ab6bede35ead5ae4f0320fefa7a00026218787fcc96acc1ebc2314dee9413514`, and
`tooling/bootstrap.py` admits `runtime_abi` 10.

Verified on that pin: the ladder reaches its fixed point (stage 2 and
stage 3 emit identical compiler IR, with no layout table and every
allocation carrying its layout field inline); `cargo test --lib` passes;
`bootstrap_runtime_objects`, `bootstrap_runtime_library_abi`,
`bootstrap_runtime_ir` and `arrays` pass against the ABI 10 runtime.
`bootstrap_library_object` passes except its `i16` foreign client, the
known missing sign extension of narrow foreign arguments, and
`bootstrap_runtime_emit`'s CLI build lists its compiler sources by hand
and lacks `Bootstrap.Expand`; neither is a header issue.
