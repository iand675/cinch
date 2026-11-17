# Generic wide array boundary candidate

This source-only candidate follows Slots commit `f118027f` and has not been
compiled or run. It makes no Vec interoperability, Native-family, artifact or
performance claim. The independent Slots fix remains unchanged.

## Representation

An ordinary `Array I128` or `Array U128` uses the same object-format word array
as wide mutable Slots: each initialized slot owns a box with a complete inline
i128 payload. The runtime's existing format-1 scan, sharing, persistent copies,
destructor and mark-on-spawn traversal apply without a new runtime format.
Scalar primitive arguments cross as i128. No high limb is converted to i64.

The new generic-wide dispatch derives this representation from checked class
vectors, not source spelling. Raw, object and tagged non-wide dispatch remains
unchanged. Explicit packed-wide constructors remain a separate capability and
retain their existing named refusal; a packed-width-16 buffer must never be
passed to Vec's word-only Slots operations. Current packed constructors cover
only widths 1/2/4/8, so no admitted packed-wide value is silently reinterpreted.

## Operation coverage

| Boundary | Candidate handling |
| --- | --- |
| FromList | Compiler traverses genuine List@W nodes with inline i128 heads and tail at byte 16, allocates format 1, and transfers a new box into every slot. The word-list runtime entry is not used. |
| Replicate | Box the i128 once; runtime duplicates the reference per slot; release the temporary even at length zero. |
| Get | Runtime returns an owned element box. Unbox and drop that reference, then build the source `Some@W` with its inline i128 payload. None reads no payload. |
| Set | Box the i128; preserve the existing borrowed/in-place runtime choice; release the temporary on both successful and invalid-index paths. Returned Some contains the array pointer, not the element. |
| Slice / Append | Existing runtime copies/retains format-1 words and preserves the input aliases. It neither truncates payloads nor changes their scan class. |
| Tensor reshape / shape / data | Existing runtime retains container references and format; dimension arrays remain ordinary Nat arrays. |
| Tensor get / set | Same owned-box and source Option conversion as ordinary Get/Set; invalid rank/axis returns None before reading storage. Persistent set retains the original tensor. |
| Column / rows / columns / leaf | Preserve existing tree and container ownership. A scalar-wide leaf Get unboxes its owned box and returns source Some@W. |
| Zip with ordinary word classes | Existing runtime remains valid, including an already boxed tuple element containing wide fields; the tuple is a pointer-sized field. |
| Zip with a direct wide field | **Named refusal before LLVM emission**: `columnarZip ... wide tuple layout bridge`. Runtime currently allocates a two-word row while source `(U128, b)` needs three or more words. Functionality remains open. |
| Left / Right | Only navigate retained child pointers; source tuple-leaf values still return None as before. No field conversion is attempted. |
| Vec freeze / from-array | The proposed representations match at all slots. This is a static layout argument only; interoperability remains unqualified until the focused actual runtime test passes. |
| Parallel generic array producers | Existing wide-edge lowering already stores object-box words for Par payloads. The new generic Get/Set path must be checked against that producer in the follow-up gates, not assumed from class equality alone. |

## Tuple and columnar completion still open

The interim Zip refusal is essential: changing only the descriptor from Wide to
Obj would produce a source tuple containing box pointers where its constructor
and projections expect inline i128 fields. The later `Columnar (a,b)` Get sees
only the aggregate Obj class, so a local scalar unbox at that point cannot
recover the lost field layout.

A complete implementation needs a checked row-layout recipe attached to the
columnar tree: each child result's owned word representation, the destination
field byte offsets and scan codes, and explicit wide unbox/drop actions. The
runtime's iterative postorder traversal must use that recipe to allocate the
full row size and write inline wide fields; nested zips need their own recipes.
These recipes require defining-identity/layout validation across object
boundaries, not a descriptor restamp or guessing a box from its block shape.
No such metadata or runtime change is included here. Wide tuple values that
were already constructed by source code can be ordinary object array elements
without this new recipe; the focused fixture exercises that distinction.

## Required gates before integration

1. Type-check the four changed Rust files and new target in the parent's warmed
   cache only after explicit resource allocation. Source-only `rustfmt` parsing
   and `git diff --check` do not establish compiler correctness.
2. Run `wide_arrays` at O0/O2, checked-RC emission, normal 32MiB and poisoned
   256MiB heaps, 8MiB program stack. Require complete unsigned and signed high
   limbs, list and Option layout correctness, persistent aliases, zero/invalid
   ranges, tensor accesses, tuple leaf zip and Vec freeze/thaw copies, zero leaks.
3. Exercise every remaining columnar navigation operation and nested ordinary
   zips, both orders of the direct-wide Zip refusal, empty wide zip, and library
   source-absent/reexport callers. Refusal must precede any malformed LLVM.
4. Add actual higher-order/partial Get/Set, parallel Array tabulation/map and
   retained aliases after bulk copying. Include signed MIN, unsigned MAX and
   opposite high/low limbs. Existing scalar operation assertions are not enough.
5. Run `arrays`, `wide`, `lib_mutable`, `mvar` and the parent's direct Slots/
   nominal Region payload controls. Verify raw/packed 1/2/4/8-byte-width storage
   behavior and object/tagged ownership remain unchanged. Inspect LLVM prototype
   consistency and source List/Some byte offsets at each wide boundary.
6. Emit a fresh W-specialized library object and exercise source-absent clients.
   Do not reuse an older object whose metadata claimed a Wide boundary while
   its actual body used i64: matching the intended class letter cannot repair
   that body. Record producer source, object and interface hashes separately.
7. No Native adoption or provider compatibility claim follows from these Rust
   gates. Native currently has separate wide-array refusal/lowering limits, and
   object/provider source and ABI identities must be admitted independently.

List conversion allocates one box per element, so ownership and pointer chasing
prevent a sound contiguous SIMD copy. Slice/append reuse the existing runtime
bulk loops; optimization of their pointer-map copies is a separate measured
change. No inline LLVM containment or source proof contract is weakened.
