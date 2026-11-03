# Array primitives

Native implementation; main integration, 2026-11-03. This change preserves
the ABI5 self-hosting checkpoint and its pinned producer. Runtime ABI **6** is
required. Interface schema remains 8; the existing kinded schemes express
these constructors without a schema change.

`Array a`, `Tensor a` and `Columnar row` are opaque compiler primitives of
kind `Type -> Type`. Source declarations cannot redefine them or forge
their representations. The element and row types are invariant under
unification. They are immutable: setting an element returns a new value.
Bounds, rank, shape and column-length failures return `None`. Allocations
use the existing accounted heap and its `Exhausted` outcome.

The operations live in explicit source modules `Data.Array`,
`Data.Array.Packed`, `Data.Tensor` and `Data.Columnar`. Ship those modules
beside an application's entry, as with `Text.Lex`; they are deliberately
absent from the pinned seed's embedded prelude. The type constructors
themselves are builtin and do not need an export declaration.

## Ordinary arrays

`Array a` owns one contiguous buffer. The generic constructor stores
elements as ABI words: machine scalars inline, `Nat`/`Int` as tagged words,
and structured values as counted pointers. Its immutable length never
changes. Reads are constant time. Persistent updates and slices copy the
selected buffer; append allocates one combined buffer.

| Operation | Signature after `forall a.` |
| --- | --- |
| `array-from-list` | `List a -> Array a` |
| `array-replicate` | `Nat -> a -> Array a` |
| `array-length` | `Array a -> Nat` |
| `array-get` | `Array a -> Nat -> Option a` |
| `array-set` | `Array a -> Nat -> a -> Option (Array a)` |
| `array-slice` | `Array a -> Nat -> Nat -> Option (Array a)` |
| `array-append` | `Array a -> Array a -> Array a` |

The `FromList` instance also accepts `[1, 2, 3]` at an expected `Array Int`.
Slice arguments are start and count. The empty slice at the end is valid.
Counts and indices are mathematical `Nat`; out-of-range bignum indices
return `None` without narrowing overflow.

`Data.Array.Packed` supplies `array-from-u8s`, `array-from-i16s`, and the
corresponding constructors for `I8`–`I64`, `U8`–`U64`, `F32`, `F64`,
`Char`, `Usize` and `Isize`. `Bool` and `Unit` arrays currently use generic
word storage. Packed buffers use exactly the scalar's
byte stride. The format stays with the buffer, so generic reads and updates
also work on packed arrays. Signed subword elements are sign extended on
read; float bits are preserved. Appending differently formatted arrays of
the same type may use ABI-word storage to preserve both inputs losslessly.

This refines the primitive-types chapter's blanket stride statement:
packed scalar array constructors have scalar stride; a generic polymorphic
array uses the current word ABI. Automatic choice of packed storage is
coming soon. SIMD `Vec` layout is unaffected.

## Multidimensional arrays

`tensor-reshape shape data` returns `Option (Tensor a)`. Shape is an
immutable `Array Nat`; data is an `Array a`. The product of dimensions
must equal the data length. Reshape shares both buffers and allocates only
a descriptor. It never exposes a partially validated shape.

`tensor-get value indices` and `tensor-set value indices element` require
one index per axis and check every axis independently. The last axis varies
fastest (row-major order). An invalid axis cannot spill into the next row.
`tensor-shape` and `tensor-data` return shared immutable buffers.

An empty shape has rank zero and contains one scalar. A zero dimension
means zero elements; zero is detected before multiplying other dimensions,
including bignum dimensions. Product and offset arithmetic is checked.
Static type-level dimension lists, strided views and transpose are coming
soon; `Tensor a` currently carries its checked shape at runtime.

## Columnar arrays

`columnar-column :: Array a -> Columnar a` shares one typed buffer.
`columnar-zip :: Columnar a -> Columnar b -> Option (Columnar (a, b))`
checks equal row counts and shares its two inputs. Composition supports
arbitrarily many heterogeneous columns through nested pairs. Each leaf can
use packed storage. Zipping columns never allocates an array of row tuples.

`columnar-left` and `columnar-right` return the typed child columns of a
zip node. `columnar-array` returns a leaf's original buffer. These accessors
return `None` when the requested structural node is absent, including a
leaf whose element happens to be a tuple. They never materialize rows.

`columnar-rows` and `columnar-columns` return counts. `columnar-get` returns
`Option row`, materializing only the requested row. Reconstruction uses an
explicit postorder stack, and the tuple descriptors contain the actual
field pointer maps. A column can hold a record as one element; automatic
decomposition of record rows into named fields is coming soon.

## Ownership and representation

ABI6 adds runtime kinds 14 (`Array`), 15 (`Tensor`) and 16 (`Columnar`).
Array payloads start with format and length, followed by inline elements.
The format distinguishes packed raw scalars and word arrays of Raw, Obj
or TObj elements. Reclamation and mark-on-spawn scan only counted words;
scalar bits and metadata are never treated as pointers. Tensor scans its
two buffers. Columnar scans its children, with row counts and tuple layout
descriptors remaining raw.

Polymorphic array intrinsics are checked against canonical signatures and
become ordinary named primitive units. The existing representation
collector specializes them by class vector. That vector determines owned
argument drops, array element scanning, and `Some` payload maps. Runtime
getters return owned references before the caller drops their containers.
The `Copy` check descends into array element/row types, rather than treating
an opaque constructor with no source fields as vacuously Copy.

Collection constant folding must retain constructor instantiations. The
candidate also fixes a prerequisite bug where a folded `List Item` or
`List Int` used `Cons@R`, losing counted element ownership before the
list-to-array conversion. Reification now follows the checked result and
field types, including nested products and collections.

## Bootstrap integration boundary

The native implementation and runtime are independently testable. The
Cinch compiler does **not yet** implement these array operations. Do not
change an ABI5 field to 6 and assume that establishes support. Integration
requires, at minimum:

1. Add the three primitive kinds and reserved-name rules to
   `Bootstrap.Infer` and the kind/type validation paths.
2. Extend `Bootstrap.Builtin.intrinsic-contract` with checked polymorphic
   array schemes and extern identities, and preserve those through
   `Bootstrap.Contracts`, interface loading and independent Core checking.
3. Implement specialization, owned argument/result handling and array
   lowering in the Cinch IR/emitter, including Option/tuple pointer maps,
   packed scalar formats and every new runtime extern.
4. Retain the exact selected ABI in kernel metadata and interface admission.
   Recheck byte observations in `Bootstrap.Core`; retain historical ABI5
   evidence as ABI5 evidence.
5. Run negative ABI compatibility checks, native/Cinch conformance and an
   actual new stage chain, then intentionally update the producer pin.

Until then, unknown array intrinsics/kinds must be refused by the
self-hosted checker. The native compile-time evaluator also returns its
named `primitive ... is not available at compile time in this slice`
diagnostic for array operations. It must never reinterpret an array as
Bytes or silently substitute an ABI5 implementation.

`Bootstrap.RuntimeAbi` selects one exact ABI per compiler build. Live
sources select 6, so Native-built Cinch loader/checker tests use matching
ABI6 interfaces and kernel metadata for the operations they support.
`tooling/bootstrap.py` renders this small configuration module from the
verified stage0 lock when preparing the pinned chain, currently selecting
5. The rendered bytes enter the source hash, bundled compiler and exported
stage source files. Unsupported ABI selections fail before building. Each
loader admits only its selected version, including across interface sets;
there is no artifact translation or mixed-ABI admission. The pinned chain
still links its verified seed runtime archive. Its seed lock, seed patch,
and historical reports are unchanged. ABI selection does not implement
Cinch array kinds, intrinsic checking, or lowering.
