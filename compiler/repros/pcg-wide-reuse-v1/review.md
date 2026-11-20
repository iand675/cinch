# Wide constructor reuse prototype

This is an **unapplied, untyped compiler proposal**, not an admitted compiler or
performance result. No compiler, linker, consumer, SDK or benchmark was built or
executed for this investigation. Only static source checks and disassembly of
an independently verified existing executable were performed. Root's inputs,
frozen compilers, providers, sources and failures remain unchanged.

## Observed overhead

The independently qualified PCG source has SHA256
`e04fcaa48fe2d6a15c26a6d26d5945bc2e4d98c9f51fc5e9b7fc91f692c36c76`.
Its state is an immutable nominal constructor containing one inline U128.
The inspected LLVM SHA256 is
`30568ecf424284d7f0053eddef54426dc1e3da111b11ee67da2294c06af386fa`.
The plain LLVM in the preserved O0-v1 infrastructure failure is byte-identical
to the LLVM recorded by the successful O2-v1 receipt. The former receipt is
not being relabeled as a successful runtime gate.

`pcg-next#cc` (LLVM2526–2571) already returns the outer product in registers.
Its transition and permutation lower into ordinary full-width multiply/carry
and scalar arithmetic. The ordinary closure wrapper can box a tuple; that
does not imply every direct worker call boxes that tuple.

`PcgState` (LLVM3311–3319) calls `cinch_alloc_ctor(16,0)` and stores an i128.
In an inlined iteration (LLVM3428–3447), generated code loads the full state,
checks uniqueness, frees the unique old state and allocates its successor.
This is actual generated overhead, not a conclusion drawn only from source.

The verified existing O2 executable SHA256 is
`c6e440e4840913f059a5e6b1f642ed9f4d73756af29e753b64e1fb63a94cd0c5`;
its qualified producer is `ffb5d39c…`, runtime `bdf524d8…`.
`verified-o2-excerpts.s` was obtained with static objdump after decoding the
archived ELF into a memory file descriptor, verifying the complete compressed
and decoded identities, and never executing it or recreating a Root ELF path.
The worker allocates at address1e69e. An inlined loop frees the unique state at
1f49a and allocates at1f5a9. The normal register return and good arithmetic
survive O2. `assembly-audit.json` retains full static call inventories.

## Cause and narrow change

Existing reuse already handles ordinary word fields, call arguments, computed
heads and branches. It coordinates borrowed-argument inference with ownership
insertion: potential reset targets must be owned, then actual uniqueness picks
reuse or fresh allocation. It is not safe to mutate a borrowed state merely
because a downstream value looks unique.

`rc::Ctors::of` deliberately excludes constructors containing Wide fields.
This protects the current emitter: `emit_reuse` allocates eight bytes per logical
field and writes i64 per field. Removing that exclusion alone would truncate
U128/I128 values and could corrupt layouts or reference maps.

The proposal keeps ordinary reuse unchanged and adds a separate guarded path:

1. Source pattern checking records the actual defining constructor identity.
   It is optional, and not inferred from a tag, a type header, a display name,
   physical similarity, private-field usage proof or optimization artifact.
2. Wide reuse requires that checked identity and the successor's exact same
   defining constructor, tag and concrete field-class vector. Boundary plans,
   unresolved representation parameters and cross-constructor matches refuse
   this new path. Mutable-cell exclusion remains in force.
3. The emitter uses its existing physical field-offset and descriptor machinery:
   one Wide field is16bytes/two uncounted words, with an i128 store aligned8.
   `store_uniform_field` is shared with ordinary wide constructor emission.
   Both cold and hot paths write complete fields. The descriptor is the same
   checked layout as the old cell; mixed counted fields retain the existing
   moved/rest ownership discipline and their expanded physical reference map.
4. Existing shared, frame, virtual-product and checked-RC paths remain unchanged.
   They supply a zero token and allocate a fresh result. Only a uniquely owned
   heap cell is reused. Kept-field indices denote whole logical fields,
   including both raw words of a Wide field.

The prototype adds optional Core pattern metadata, so migration includes
synthetic pattern literals and exact pattern equality/hashing. It does not
change a runtime ABI, public library signature, effect row or interface schema.
It currently targets the Rust compiler only; no Native parity or family claim
is made.

## Source, imported-body and cache transport

The source checker sets identity from its resolved constructor Value. Lowering
and elaborated-unit renaming preserve it; Core binder renaming and cloning
preserve it; class substitution preserves the defining identity while changing
the vector against which the guard compares. Pattern equality and hash include
the identity, so common-subexpression or rewrite matching cannot substitute
another nominal certificate. Specialized constructor names are compared through
the existing representation suffix decoder, not by stripping arbitrary names.

Synthetic patterns use None. The existing `.cinchopt` schema deliberately does
not serialize this local certificate, and decoding uses None. Therefore a
cached imported unfolding may lose this optimization but cannot manufacture
wide-layout authority. `optimization_roundtrip_drops_untrusted_reuse_identity`
tests that boundary. Cached/uncached runtime equivalence remains a required gate
below; no cache-performance equivalence is claimed. Contract header, private
usage proof, kernel MVarCell and nominal Region admission paths are unchanged.

## Prepared tests and required gates

The proposed Rust target covers a complete16-byte allocation and hot/cold i128
stores; owned-parameter inference; missing/legacy identity and unrelated same
shape constructors; boundary-layout refusal; swapped Wide/Raw fields; counted
neighbors in both physical positions; genuine checked source; and optimization
artifact loss of the optional certificate. These tests have parsed, but have
not been typechecked or executed. They remain in `wide_reuse.rs` and the patch.

After Root's resource handoff, the smallest meaningful next gate is one warm
Rust check/test of this target, followed by existing reuse, borrow inference,
specialization, inline-copy, stack allocation and wide-value targets. Preserve
any first failure and original inputs. Broader unit gates follow only after
the focused tests pass; no separate Cargo cache is needed.

The real PCG consumer plan must retain the exact source and independent2048
output vectors and add these externally observable controls:

- Retain original state and encoded bytes while drawing two independent forks;
  compare both forks to the independent stream and re-encode the original.
- Return an original and successor through a non-inlined helper, retain states
  in lists/tuples/closures and revisit them after further draws. Check high and
  low128-bit halves through independent encoded snapshots.
- Advance a uniquely owned state for a runtime-supplied long count. In ordinary
  RC, compare allocation counts as the count grows; after warmup a working
  worker should not allocate a state per draw. Retained aliases must still
  allocate when sharing actually requires it. Keep outputs observably used.
- Exercise false branches and both branches that consume the reset token,
  mixed counted-neighbor cells, self-field stores, frame inputs and unboxed
  products. Verify complete outputs and zero leaks at O0/O2 with normal32MiB
  and poisoned256MiB heaps, stack8MiB. Debug mode intentionally need not reuse.
- Run the same producer/consumer through current source and actual serialized
  `.cinchopt` inputs; require identical values, alias behavior and lifetime
  results, while allowing the documented cached allocation fallback.
- Re-run genuine private Nat/Bytes MVar ownership and constructor-privacy
  controls. No nominal/layout optimization may grant private constructors,
  task sharing, payload Transfer or kernel identity authority.

Inspect new optimized worker assembly before any elapsed performance claim.
Require the hot path to retain its cell with two complete machine-word stores,
and the shared cold path still to allocate16bytes. CPU timing remains separately
scheduled, bounded and dependent on correctness, codegen and clock preflights.

## Byte word-store contract assessment

Current `bytes-write-le64` contains eight checked scalar `bytes-set` operations;
PCG encoding performs two such word writes plus four prefix writes. The inspected
LLVM contains20 checked byte-write call sites. The O2 executable has37 static
`cinch_bytes_set` sites across its control-flow graph; that static duplication
is not a claim of37 writes on every runtime path.

The existing unsafe word loads are conservative in this exact artifact:
`Data.Bytes.Word.load-le64` and its `.llvm` body both use attribute group6,
`memory(read) norecurse nounwind`. Neither has `memory(none)` or `speculatable`.
The owner-retaining wrapper and exact unaligned bounds are still relevant.

The current Rust primitive envelope already accepts closed explicit effect
rows and machine-scalar arguments/results. It rejects a Unit result and Bytes
arguments. A `uses` declaration can return Unit, but that is not the primitive's
own result envelope. Pure LLVM stores are rejected even under `unsafe`;
`unsafe` does not waive a visible write. An effectful scalar store's emitted
body would be scanned as `memory(readwrite)` and would not be speculatable.
ASM is explicitly side-effecting and opaque, which is not a sound escape hatch
for undeclared memory writes or missing buffer contracts.

An effectful arbitrary-pointer primitive returning a dummy scalar would not
preserve the current public pure Unit Bytes API, owner lifetime, fixed-length
fact treatment or runtime readonly-kind checks. The appropriate narrow next
design is a compiler-owned typed32/64-bit Bytes word-store operation: an actual
buffer owner, proved exact range, Unit result, precise byte order and align1,
the same readonly/wrong-kind/compiler-bug bounds traps as `bytes-set`, and the
existing mutation semantics. It may lower to a checked writable-pointer bridge
plus an inline LLVM store, or to one checked runtime word-store operation.
Neither exists yet. A generic checked inline-buffer memory contract would be
a separate larger language feature, not a change to pure containment rules.

Before implementation, require read-after-write/alias and unused-Unit-result
tests (DCE/CSE must preserve writes), all alignments and endiannesses, full-width
canaries/guard pages, owner survival, readonly/wrong-kind refusal, and existing
refinement/effect-order controls. A new backend path must not emit memory(none),
speculatable, noalias or stronger lifetime facts without proving them.

## Static validation only

`inputs.json` records exact base/target hashes for every proposed path and the
Root revision. The inspected `rc.rs` and `codegen.rs` are byte-identical to
those in the qualified PCG producer's source revisionb9c4e48c, so the diagnosis
matches the actual code-generating implementation rather than a stale lane.
`git apply --check` succeeds against those current inputs. Rustfmt parsed all13
changed existing Rust files and the proposed test file without compiling them.
No claim is made about Rust typing, complete tests, runtime safety of this new
patch, allocation improvement or elapsed performance until those gates run.
