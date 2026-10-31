# Data.List

Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.List`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.List.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.List: the inductive list: `Nil` or `Cons x xs`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its `FromList` instance (collection literals at `List` are the
identity). `append-list` is the list append the instance family
shares; it is deliberately not in the export list (the curated
surface spells append through `Semigroup`'s `<>`).

`map` here is the concrete list map (the prelude surface's `map`,
decision D106); the `Functor` class method of the same name is
reached through its constraint or an explicit `import
Category.Functor`, which shadows this one per the import shadowing
rule (decision D145).

The concrete `base` combinators live here beside the type: the four
partial projections, the pairing and splitting family, the searching
family, the bounded generators (`iterate`/`unfold`/`cycle` take their
bound as a parameter, because every list-producing walk in this
language must end), the scans and the element-seeded folds,
`transpose`, the stable merge sort, the run grouping, the joining
family, and `nub`/`subsequences`. The container-generic twins
(`sum`, `elem`, the folds with an element seed, ...) live in
`Category.Foldable`, which is reached with an explicit `import`; a
program that wants the list operation without a class constraint uses
the name here, which is the `map` situation over again.

</details>

# Types and constructors

<a id="type-4c697374"></a>

## `List`

```cinch
data List (a :: Type)
```

[Source](/reference-sources/Data.Core.cinch) · line 26

```cinch
Nil :: forall a . List a
```

```cinch
Cons :: forall a . (a -> (List a -> List a))
```

# Values

<a id="value-627265616b"></a>

## `break`

```cinch
break :: forall a . ((a -> Bool) -> (List a -> (List a, List a)))
```

The longest prefix whose elements do NOT satisfy `p`, and the rest.
Written out rather than spelled `span` of a negated predicate:
`not` lives in `Data.Bool`, which is checked after this module.

[Source](/reference-sources/Data.List.cinch) · line 219

<a id="value-6379636c65"></a>

## `cycle`

```cinch
cycle :: forall a . (Nat -> (List a -> List a))
```

The list repeated `n` times. `base`'s `cycle` on an empty list
never returns; here the empty list stays empty, because every
list-producing walk in this language must end.

[Source](/reference-sources/Data.List.cinch) · line 298

<a id="value-64726f70"></a>

## `drop`

```cinch
drop :: forall a . (Nat -> (List a -> List a))
```

Everything but the first `n` elements.

[Source](/reference-sources/Data.List.cinch) · line 106

<a id="value-656c656d2d696e646578"></a>

## `elem-index`

```cinch
elem-index :: forall a . using (eq :: Eq a) => (a -> (List a -> Option Nat))
```

The position of the first element equal to `x` (`None` when
absent).

[Source](/reference-sources/Data.List.cinch) · line 262

<a id="value-66696e64"></a>

## `find`

```cinch
find :: forall a . ((a -> Bool) -> (List a -> Option a))
```

The first element `p` accepts (`None` when none does).

[Source](/reference-sources/Data.List.cinch) · line 239

<a id="value-66696e642d696e646578"></a>

## `find-index`

```cinch
find-index :: forall a . ((a -> Bool) -> (List a -> Option Nat))
```

The position of the first element `p` accepts (`None` when none
does). The position is a `Nat`: it counts from zero, so it cannot
be negative.

[Source](/reference-sources/Data.List.cinch) · line 249

<a id="value-666f6c642d6c65667431"></a>

## `fold-left1`

```cinch
fold-left1 :: forall a . ((a -> (a -> a)) -> (List a -> Option a))
```

The left fold seeded with the first element, so an empty list has
nothing to return (`None`; `base`'s `foldl1` faults).

[Source](/reference-sources/Data.List.cinch) · line 329

<a id="value-666f6c642d726967687431"></a>

## `fold-right1`

```cinch
fold-right1 :: forall a . ((a -> (a -> a)) -> (List a -> Option a))
```

The right fold seeded with the first element (`None` on an empty
list).

[Source](/reference-sources/Data.List.cinch) · line 342

<a id="value-666f6c6472"></a>

## `foldr`

```cinch
foldr :: forall a b . ((a -> (b -> b)) -> (b -> (List a -> b)))
```

[Source](/reference-sources/Data.List.cinch) · line 67

<a id="value-67726f75702d6279"></a>

## `group-by`

```cinch
group-by :: forall a . ((a -> (a -> Bool)) -> (List a -> List (List a)))
```

The runs of adjacent elements the `same` test groups together: a
run starts at an element and grows while `same` holds of that
element and the next candidate, so `same` is the same-group test
rather than an equality (`base`'s `groupBy`).

[Source](/reference-sources/Data.List.cinch) · line 427

<a id="value-68656164"></a>

## `head`

```cinch
head :: forall a . (List a -> Option a)
```

The partial projections of a list: the first element, everything
but the first, the last element, and everything but the last. Each
reports the empty list as `None` (the prelude's partiality
convention: a partial accessor returns `Option`, never a fault).
`head` is `nth 0` spelled for the reader.

[Source](/reference-sources/Data.List.cinch) · line 139

<a id="value-696e6974"></a>

## `init`

```cinch
init :: forall a . (List a -> Option (List a))
```

[Source](/reference-sources/Data.List.cinch) · line 156

<a id="value-696e74657263616c617465"></a>

## `intercalate`

```cinch
intercalate :: forall a . (List a -> (List (List a) -> List a))
```

The lists joined with `sep` between them (`base`'s `intercalate`,
which is `concat` of `intersperse sep`); `Nil` and a singleton come
back unchanged. The join is the private `join-lists` below rather
than `Category.Foldable`'s class-level `concat`: this is a list
operation, its join is four lines, and a home needs no class
constraint to concatenate its own type.

[Source](/reference-sources/Data.List.cinch) · line 453

<a id="value-696e746572737065727365"></a>

## `intersperse`

```cinch
intersperse :: forall a . (a -> (List a -> List a))
```

`sep` between the elements, one separator fewer than the list has
elements, so `Nil` and a singleton come back unchanged (`base`'s
`intersperse`).

[Source](/reference-sources/Data.List.cinch) · line 436

<a id="value-69732d656d707479"></a>

## `is-empty`

```cinch
is-empty :: forall a . (List a -> Bool)
```

[Source](/reference-sources/Data.List.cinch) · line 129

<a id="value-69746572617465"></a>

## `iterate`

```cinch
iterate :: forall a . (Nat -> ((a -> a) -> (a -> List a)))
```

The enumeration `x, f x, f (f x), ...` cut to `n` elements. The
bound is what keeps the walk total (`base`'s `iterate` is
infinite).

[Source](/reference-sources/Data.List.cinch) · line 280

<a id="value-6c617374"></a>

## `last`

```cinch
last :: forall a . (List a -> Option a)
```

[Source](/reference-sources/Data.List.cinch) · line 149

<a id="value-6c656e677468"></a>

## `length`

```cinch
length :: forall a . (List a -> Int)
```

[Source](/reference-sources/Data.List.cinch) · line 72

<a id="value-6c6973742d6c656e677468"></a>

## `list-length`

```cinch
list-length :: forall a . (List a -> Nat)
```

The same count as a `Nat`, for the indexing and sizing arithmetic
that works in `Nat` (`take`/`drop`/`nth` below, offsets, widths).
`length` stays `Int` because the prelude surface has always
returned `Int` there.

[Source](/reference-sources/Data.List.cinch) · line 81

<a id="value-6c6f6f6b7570"></a>

## `lookup`

```cinch
lookup :: forall k v . using (eq :: Eq k) => (k -> (List (k, v) -> Option v))
```

The value paired with `key`: the first pair whose key matches wins,
so a repeated key keeps the earliest value (`None` when absent).
`lookup` is the association-list read `Data.Map`/`Data.HashMap`
build their trees over.

[Source](/reference-sources/Data.List.cinch) · line 269

<a id="value-6d6170"></a>

## `map`

```cinch
map :: forall a b . ((a -> b) -> (List a -> List b))
```

`List`, `Nil` and `Cons` are declared in `Data.Core` and re-exported
here (that module's header says why: `Data.List` and `Data.Option`
name each other in signatures, so neither can be checked first).
This home carries the type's instance family and its operations.

[Source](/reference-sources/Data.List.cinch) · line 62

<a id="value-6e7468"></a>

## `nth`

```cinch
nth :: forall a . (Nat -> (List a -> Option a))
```

The element at index `n` (`None` past the end).

[Source](/reference-sources/Data.List.cinch) · line 114

<a id="value-6e7562"></a>

## `nub`

```cinch
nub :: forall a . using (eq :: Eq a) => (List a -> List a)
```

The list with later duplicates removed, keeping the first
occurrence of each element (`base`'s `nub`; the `Eq` constraint is
what decides sameness).

[Source](/reference-sources/Data.List.cinch) · line 465

<a id="value-706172746974696f6e"></a>

## `partition`

```cinch
partition :: forall a . ((a -> Bool) -> (List a -> (List a, List a)))
```

The elements that satisfy `p` and the ones that do not, each in
input order (so the two results interleave back into the input).

[Source](/reference-sources/Data.List.cinch) · line 229

<a id="value-7265706c6963617465"></a>

## `replicate`

```cinch
replicate :: forall a . (Nat -> (a -> List a))
```

`x` repeated `n` times.

[Source](/reference-sources/Data.List.cinch) · line 124

<a id="value-72657665727365"></a>

## `reverse`

```cinch
reverse :: forall a . (List a -> List a)
```

Reversal (the accumulator is the reverse built so far, so the walk
is linear; `reverse-into` is the private twin every accumulator
walk here shares).

[Source](/reference-sources/Data.List.cinch) · line 89

<a id="value-7363616e6c"></a>

## `scanl`

```cinch
scanl :: forall a b . ((b -> (a -> b)) -> (b -> (List a -> List b)))
```

The successive reductions from the left, the seed first: `scanl f z
xs` has one element more than `xs` (`base`'s `scanl`).

[Source](/reference-sources/Data.List.cinch) · line 307

<a id="value-7363616e72"></a>

## `scanr`

```cinch
scanr :: forall a b . ((a -> (b -> b)) -> (b -> (List a -> List b)))
```

The successive reductions from the right: the first element is the
whole fold and the seed is last (`base`'s `scanr`).

[Source](/reference-sources/Data.List.cinch) · line 314

<a id="value-736f7274"></a>

## `sort`

```cinch
sort :: forall a . using (ord :: Ord a) => (List a -> List a)
```

The list in ascending order under `Ord` (a stable merge sort).

[Source](/reference-sources/Data.List.cinch) · line 410

<a id="value-736f72742d6279"></a>

## `sort-by`

```cinch
sort-by :: forall a . ((a -> (a -> Ordering)) -> (List a -> List a))
```

Merge sort under a three-way comparator. Stable: the left element
is kept unless the right one compares strictly before it, so equal
elements stay in their input order (`base`'s `sortBy`).

[Source](/reference-sources/Data.List.cinch) · line 381

<a id="value-7370616e"></a>

## `span`

```cinch
span :: forall a . ((a -> Bool) -> (List a -> (List a, List a)))
```

The longest prefix whose elements satisfy `p`, and the rest.

[Source](/reference-sources/Data.List.cinch) · line 208

<a id="value-73706c69742d6174"></a>

## `split-at`

```cinch
split-at :: forall a . (Nat -> (List a -> (List a, List a)))
```

The first `n` elements and the rest; the split clamps at the end,
so `split-at n` of a shorter list is the whole list and `Nil`.

[Source](/reference-sources/Data.List.cinch) · line 199

<a id="value-73756273657175656e636573"></a>

## `subsequences`

```cinch
subsequences :: forall a . (List a -> List (List a))
```

Every subsequence, the empty one first, in `base`'s order:
`subsequences [a, b]` is `[[], [a], [b], [a, b]]`. The count is
`2^n` for a list of `n` elements, so this is the one combinator
here whose result can dwarf its input; it still ends, because the
input does.

[Source](/reference-sources/Data.List.cinch) · line 483

<a id="value-73756d2d6c697374"></a>

## `sum-list`

```cinch
sum-list :: forall a . using (add :: Add a) => (a -> (List a -> a))
```

[Source](/reference-sources/Data.List.cinch) · line 500

<a id="value-7461696c"></a>

## `tail`

```cinch
tail :: forall a . (List a -> Option (List a))
```

[Source](/reference-sources/Data.List.cinch) · line 144

<a id="value-74616b65"></a>

## `take`

```cinch
take :: forall a . (Nat -> (List a -> List a))
```

The first `n` elements (fewer when the list is shorter).

[Source](/reference-sources/Data.List.cinch) · line 98

<a id="value-7472616e73706f7365"></a>

## `transpose`

```cinch
transpose :: forall a . (List (List a) -> List (List a))
```

The transpose of a ragged list of lists: round k is the k-th
element of every row that still has one, so a short row contributes
nothing to the later rounds and the result has as many rows as the
longest input row (`base`'s `transpose`). A rectangular `Matrix`
record keeps its own row-level helper; this is the general list
operation.

[Source](/reference-sources/Data.List.cinch) · line 359

<a id="value-756e666f6c64"></a>

## `unfold`

```cinch
unfold :: forall a b . ((b -> Option (a, b)) -> (b -> List a))
```

The unfolding: `f` either yields the next element together with the
next seed, or stops. `base` spells this `unfoldr`; the `r` suffix
is historical (the walk is not a right fold), so the name here is
the plain one.

[Source](/reference-sources/Data.List.cinch) · line 289

<a id="value-756e7a6970"></a>

## `unzip`

```cinch
unzip :: forall a b . (List (a, b) -> (List a, List b))
```

The inverse of `zip`: the first components and the second
components, each in order.

[Source](/reference-sources/Data.List.cinch) · line 190

<a id="value-7a6970"></a>

## `zip`

```cinch
zip :: forall a b . (List a -> (List b -> List (a, b)))
```

Pairing, truncated at the shorter side (so a length mismatch
shortens the result, never faults).

[Source](/reference-sources/Data.List.cinch) · line 173

<a id="value-7a69702d77697468"></a>

## `zip-with`

```cinch
zip-with :: forall a b c . ((a -> (b -> c)) -> (List a -> (List b -> List c)))
```

`zip` with the pair built by `f`, truncated the same way.

[Source](/reference-sources/Data.List.cinch) · line 181

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.List.Alternative_List` | `Alternative List` | Implicit |
| `Data.List.Applicative_List` | `Applicative List` | Implicit |
| `Data.List.Foldable_List` | `Foldable List` | Implicit |
| `Data.List.FromList_List` | `FromList (List e) e` | Implicit |
| `Data.List.Functor_List` | `Functor List` | Implicit |
| `Data.List.Monad_List` | `Monad List` | Implicit |
| `Data.List.Monoid_List` | `Monoid (List t)` | Implicit |
| `Data.List.Semigroup_List` | `Semigroup (List t)` | Implicit |
| `Data.List.Traversable_List` | `Traversable List` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/List.cinch`. Source SHA-256: `07bee14b1520df1e5a0de73e0faa956c03da610014f97773b101dfe463675575`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
