# Data.Option

Optional values with None and Some, plus combinators for propagating or consuming absence.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Option`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Option.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Option: the optional value: `None` or `Some x`.

Module scheme (design/base-library.md): `Data.*` homes are the
concrete types; every instance lives with its type, so the instance
family for `Option` is here (the parked-instance deviation of D159
is over). The class modules own only their structural instances.

The D22 sequencing provider (`OptionSequencing`) is a property of
the container, so its provider instance is here too: one instance
per provider class keeps `do` selection order-independent (decision
D22; the classes live in Effect.Sequencing).

</details>

# Types and constructors

<a id="type-4f7074696f6e"></a>

## `Option`

```cinch
data Option (a :: Type)
```

[Source](/reference-sources/Data.Core.cinch) · line 30

```cinch
None :: forall a . Option a
```

```cinch
Some :: forall a . (a -> Option a)
```

# Values

<a id="value-6361742d6f7074696f6e73"></a>

## `cat-options`

```cinch
cat-options :: forall a . (List (Option a) -> List a)
```

The present values, in order; `None`s drop (`Data.Maybe.catMaybes`).
`List` is nameable here because `Data.Core` declares it ahead of
every home (this function's signature is exactly the cycle that
module exists to break).

[Source](/reference-sources/Data.Option.cinch) · line 75

<a id="value-66726f6d2d6f7074696f6e"></a>

## `from-option`

```cinch
from-option :: forall a . (a -> (Option a -> a))
```

The value, or the default when there is none
(`Data.Maybe.fromMaybe`); the default leads for the same reason, and
because the result's type is the default's type.

[Source](/reference-sources/Data.Option.cinch) · line 55

<a id="value-69732d6e6f6e65"></a>

## `is-none`

```cinch
is-none :: forall a . (Option a -> Bool)
```

[Source](/reference-sources/Data.Option.cinch) · line 66

<a id="value-69732d736f6d65"></a>

## `is-some`

```cinch
is-some :: forall a . (Option a -> Bool)
```

The presence tests: the tag read, no payload work, no constraint.

[Source](/reference-sources/Data.Option.cinch) · line 61

<a id="value-6c6973742d746f2d6f7074696f6e"></a>

## `list-to-option`

```cinch
list-to-option :: forall a . (List a -> Option a)
```

The first element, or `None` for the empty list
(`Data.Maybe.listToMaybe`); the tail is deliberately not inspected,
so this is O(1) on any list.

[Source](/reference-sources/Data.Option.cinch) · line 100

<a id="value-6d61702d6f7074696f6e"></a>

## `map-option`

```cinch
map-option :: forall a b . ((a -> b) -> (Option a -> Option b))
```

The option's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.

[Source](/reference-sources/Data.Option.cinch) · line 85

<a id="value-6d61796265"></a>

## `maybe`

```cinch
maybe :: forall a b . (b -> ((a -> b) -> (Option a -> b)))
```

`Option`, `None` and `Some` are declared in `Data.Core` and
re-exported here (that module's header says why: this home consumes
a `List` in `cat-options`, and `Data.List` answers an `Option` in
`nth`, so neither home could be checked first). This home carries
the type's instance family and its combinators.

The combinators: `Data.Maybe`'s folds and conversions, as free
functions beside the class instances below. Every one of them is a
total function: partiality here is `Option` itself, never a fault.

The fold: the default `d` unless there is a value, which `f`
consumes. The default leads and the function follows, matching
Haskell's `maybe` (`maybe d f o`); parity of argument order is the
point of these combinators, and the option is the thing opened last.

[Source](/reference-sources/Data.Option.cinch) · line 47

<a id="value-6f7074696f6e2d746f2d6c697374"></a>

## `option-to-list`

```cinch
option-to-list :: forall a . (Option a -> List a)
```

A `Some` is a one-element list and a `None` the empty list
(`Data.Maybe.maybeToList`); `list-to-option` is its left inverse.

[Source](/reference-sources/Data.Option.cinch) · line 92

<a id="value-6f722d656c7365"></a>

## `or-else`

```cinch
or-else :: forall a . (Option a -> (Option a -> Option a))
```

First success: `None` falls through to the second option, `Some`
keeps the first (`Control.Applicative`'s `(<|>)` for `Option`, which
a prelude-only caller cannot reach: `Alternative` is not in the
implicit surface). The name is the value-fallback one, as in Scala's
`orElse` and this repo's `Text.Json.or-else`; Rust's closure-taking
`or_else` is deliberately not the shape, so an eager second option
is never built.

[Source](/reference-sources/Data.Option.cinch) · line 112

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Option.Alternative_Option` | `Alternative Option` | Implicit |
| `Data.Option.Applicative_Option` | `Applicative Option` | Implicit |
| `Data.Option.Foldable_Option` | `Foldable Option` | Implicit |
| `Data.Option.Functor_Option` | `Functor Option` | Implicit |
| `Data.Option.Monad_Option` | `Monad Option` | Implicit |
| `Data.Option.Monoid_Option` | `Monoid (Option t)` | Implicit |
| `Data.Option.Semigroup_Option` | `Semigroup (Option t)` | Implicit |
| `Data.Option.SeqOption` | `OptionSequencing` | Implicit |
| `Data.Option.Traversable_Option` | `Traversable Option` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Option.cinch`. Source SHA-256: `d6a557314549a9576de94de2260fdd0d8005cb703eeb075274bbe9d54b7c6a49`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
