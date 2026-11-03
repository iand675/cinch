# Category.Foldable

Generic folds, conversion to lists, size/emptiness, and related operations over container instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Foldable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Foldable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Algebra.Semiring](Algebra.Semiring.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Foldable: the fold class over containers.

Class shape: `Foldable (t :: Type -> Type)` with `foldr`, `foldl`,
`to-list`, `null` and `length` at the applied container `t a`. The
defunctionalized two-parameter scaffolding (`Foldable t a`) that
this module carried is gone: higher-kinded parameters landed and the
class is the mathematical one now. The methods have exactly their
mathematical meanings at every instance.

`to-list`/`null`/`length` agree with the fold at every lawful
instance (`foldr Cons Nil (to-list xs) == to-list xs`, `length` is
the `to-list` length, `null` is the empty check); the semantics
tests in compiler/tests/lib_foldable.rs carry them over sample
values for List, Option and NonEmpty.

Instance homes. The ownership rule is "instances live with their
type" (design/base-library.md). The List and Option instances got
theirs at the prelude split (Data.List/Data.Option). NonEmpty's
instance lives with its type in Data/NonEmpty.cinch.

The free combinators below the class are the folds `base` spells as
methods or helpers. The class has no default methods, so each one is
a free function stating its constraints (the `sum-list`/`enum-from-to`
scheme), and each reaches a container only through the class methods,
so one definition serves every instance. The partial ones
(`fold-right1`, `fold-left1`, `maximum`, `minimum`, `maximum-by`,
`minimum-by`) report an empty container as `None` (the prelude's
partiality convention). They are reached with an explicit `import
Category.Foldable`: the facade re-exports no class home, and
`Data.List` deliberately carries no concrete twin of them (reported
to the lead with this slice).

</details>

# Types and constructors

# Signatures and methods

<a id="class-466f6c6461626c65"></a>

## `Foldable`

```cinch
signature Foldable t
```

[Source](/reference-sources/Category.Foldable.cinch) · line 40

<a id="method-466f6c6461626c652e666f6c6472"></a>

## `Foldable.foldr`

```cinch
foldr :: forall (t :: Type -> Type) a b . using (dict :: Foldable t) => ((a -> (b -> b)) -> (b -> ((t a) -> b)))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 41

<a id="method-466f6c6461626c652e666f6c646c"></a>

## `Foldable.foldl`

```cinch
foldl :: forall (t :: Type -> Type) a b . using (dict :: Foldable t) => ((b -> (a -> b)) -> (b -> ((t a) -> b)))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 42

<a id="method-466f6c6461626c652e746f2d6c697374"></a>

## `Foldable.to-list`

```cinch
to-list :: forall (t :: Type -> Type) a . using (dict :: Foldable t) => ((t a) -> List a)
```

[Source](/reference-sources/Category.Foldable.cinch) · line 43

<a id="method-466f6c6461626c652e6e756c6c"></a>

## `Foldable.null`

```cinch
null :: forall (t :: Type -> Type) a . using (dict :: Foldable t) => ((t a) -> Bool)
```

[Source](/reference-sources/Category.Foldable.cinch) · line 44

<a id="method-466f6c6461626c652e6c656e677468"></a>

## `Foldable.length`

```cinch
length :: forall (t :: Type -> Type) a . using (dict :: Foldable t) => ((t a) -> Int)
```

[Source](/reference-sources/Category.Foldable.cinch) · line 45

# Values

<a id="value-616c6c"></a>

## `all`

```cinch
all :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> Bool) -> ((t a) -> Bool))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 98

<a id="value-616e79"></a>

## `any`

```cinch
any :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> Bool) -> ((t a) -> Bool))
```

`any`/`all`: a predicate decided over the whole container. The
names collide with nothing: `Data.Bool`'s `and`/`or` are the binary
operations `Bool -> Bool -> Bool`, these take a predicate and a
container. The walk is the class `foldr`, which this language
evaluates strictly in its recursive position, so every element is
visited; the answer is the same, the short-circuit is not there.

[Source](/reference-sources/Category.Foldable.cinch) · line 95

<a id="value-626574746572"></a>

## `better`

```cinch
better :: forall a . ((a -> (a -> Ordering)) -> (a -> (a -> a)))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 169

<a id="value-636f6e636174"></a>

## `concat`

```cinch
concat :: forall (t :: Type -> Type) a . using (fold :: Foldable t, mon :: Monoid a, sg :: Semigroup a) => ((t a) -> a)
```

`concat`: the monoid fold over the container's elements, so
`List String` gives a `String` and `List (List a)` a `List a`. The
identity keeps the empty container meaningful; the combination is
`mconcat` (see `fold-map`), whose `Semigroup` is stated separately
(a `Monoid` entails nothing).

[Source](/reference-sources/Category.Foldable.cinch) · line 139

<a id="value-656c656d"></a>

## `elem`

```cinch
elem :: forall (t :: Type -> Type) a . using (fold :: Foldable t, ea :: Eq a) => (a -> ((t a) -> Bool))
```

`elem`: membership under `Eq`, in `base`'s argument order (the
element, then the container).

[Source](/reference-sources/Category.Foldable.cinch) · line 103

<a id="value-66696e64"></a>

## `find`

```cinch
find :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> Bool) -> ((t a) -> Option a))
```

`find`: the first element the predicate accepts, or `None`. The
predicate does the deciding, so no `Eq` constraint is stated (the
same reason `base`'s `find` takes a predicate).

[Source](/reference-sources/Category.Foldable.cinch) · line 112

<a id="value-66696e642d6c697374"></a>

## `find-list`

```cinch
find-list :: forall a . ((a -> Bool) -> (List a -> Option a))
```

Private: the list-level search the container search materializes
into.

[Source](/reference-sources/Category.Foldable.cinch) · line 117

<a id="value-666f6c642d6c65667431"></a>

## `fold-left1`

```cinch
fold-left1 :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> (a -> a)) -> ((t a) -> Option a))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 77

<a id="value-666f6c642d6d6170"></a>

## `fold-map`

```cinch
fold-map :: forall (t :: Type -> Type) a m . using (fold :: Foldable t, mon :: Monoid m, sg :: Semigroup m) => ((a -> m) -> ((t a) -> m))
```

The free combinators. Each states its constraints and walks the
container through the class methods, so a new instance inherits all
of them with no further code.

`fold-map`: map every element into a monoid and combine with `<>`,
starting from `mempty`. The empty container therefore gives the
identity, never an absent value. Both constraints are stated: the
identity comes from `Monoid` and the combination from
`Semigroup`, and a `Monoid` does not entail a `Semigroup` (the
no-superbundle rule; the same pair Data.Const's Applicative
instance states).
The combination is `Monoid`'s `mconcat` over the mapped elements, so
a carrier with a one-pass combine (String) uses it and every other
one gets the balanced fold; a right fold of `<>` re-copies the
accumulated suffix at every element for a `<>` that copies.

[Source](/reference-sources/Category.Foldable.cinch) · line 63

<a id="value-666f6c642d726967687431"></a>

## `fold-right1`

```cinch
fold-right1 :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> (a -> a)) -> ((t a) -> Option a))
```

`fold-right1`/`fold-left1`: the fold with an element of the
container as its seed, so an empty container has nothing to return.
The right form seeds with the first element and folds the tail into
it; the left form is the class `foldl` from the first element over
the tail. Both names are `base`'s `foldr1`/`foldl1` spelled the way
a name may be spelled here (a digit cannot follow the dash).

[Source](/reference-sources/Category.Foldable.cinch) · line 72

<a id="value-666f6c642d7269676874312d696e746f"></a>

## `fold-right1-into`

```cinch
fold-right1-into :: forall a . ((a -> (a -> a)) -> (a -> (List a -> a)))
```

Private: the `foldr1` walk over the tail, seeded with the first
element.

[Source](/reference-sources/Category.Foldable.cinch) · line 84

<a id="value-6c6172676572"></a>

## `larger`

```cinch
larger :: forall a . using (oa :: Ord a) => (a -> (a -> a))
```

Private: the four comparisons the `Ord` walks and the comparator
walks share. Each keeps the element the comparison favours and, on
a tie, the one `base`'s `min`/`max`/`minBy`/`maxBy` keep.

[Source](/reference-sources/Category.Foldable.cinch) · line 163

<a id="value-6d6178696d756d"></a>

## `maximum`

```cinch
maximum :: forall (t :: Type -> Type) a . using (fold :: Foldable t, oa :: Ord a) => ((t a) -> Option a)
```

`maximum`/`minimum`: partial over an empty container (an `Option`
value), and decided by comparison rather than position, so an
element before the greatest one still wins when it is greater. A
tie keeps the first element for `minimum` and the last one for
`maximum`, exactly as `base`'s `min`/`max` do. The `-by` forms take
the comparator, so non-`Ord` shapes and keys compare too.

[Source](/reference-sources/Category.Foldable.cinch) · line 148

<a id="value-6d6178696d756d2d6279"></a>

## `maximum-by`

```cinch
maximum-by :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> (a -> Ordering)) -> ((t a) -> Option a))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 154

<a id="value-6d696e696d756d"></a>

## `minimum`

```cinch
minimum :: forall (t :: Type -> Type) a . using (fold :: Foldable t, oa :: Ord a) => ((t a) -> Option a)
```

[Source](/reference-sources/Category.Foldable.cinch) · line 151

<a id="value-6d696e696d756d2d6279"></a>

## `minimum-by`

```cinch
minimum-by :: forall (t :: Type -> Type) a . using (fold :: Foldable t) => ((a -> (a -> Ordering)) -> ((t a) -> Option a))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 157

<a id="value-6e6f742d656c656d"></a>

## `not-elem`

```cinch
not-elem :: forall (t :: Type -> Type) a . using (fold :: Foldable t, ea :: Eq a) => (a -> ((t a) -> Bool))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 106

<a id="value-70726f64756374"></a>

## `product`

```cinch
product :: forall (t :: Type -> Type) a . using (fold :: Foldable t, sr :: Semiring a) => ((t a) -> a)
```

[Source](/reference-sources/Category.Foldable.cinch) · line 131

<a id="value-736d616c6c6572"></a>

## `smaller`

```cinch
smaller :: forall a . using (oa :: Ord a) => (a -> (a -> a))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 166

<a id="value-73756d"></a>

## `sum`

```cinch
sum :: forall (t :: Type -> Type) a . using (fold :: Foldable t, sr :: Semiring a) => ((t a) -> a)
```

`sum`/`product`: the container's additive or multiplicative
combination. The constraint is `Semiring`, not `Add`: an empty
container needs an identity to return and `Add` carries none, while
`Semiring`'s `zero`/`one` are exactly the two identities
(Algebra.Semiring). `sum-list` (Data.List) is the seeded list twin
of `sum`: the same walk with an explicit starting value.

[Source](/reference-sources/Category.Foldable.cinch) · line 128

<a id="value-776f727365"></a>

## `worse`

```cinch
worse :: forall a . ((a -> (a -> Ordering)) -> (a -> (a -> a)))
```

[Source](/reference-sources/Category.Foldable.cinch) · line 174

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Foldable.cinch`. Source SHA-256: `f3379c5293e6125a6072d12973b9933b164f7c8154e64dcb4e8bb01bad2fbaab`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
