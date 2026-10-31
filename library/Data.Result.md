# Data.Result

Success and named failure values, error-aware transformations, partitioning, and sequencing instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Result`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Result.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Data.Core](Data.Core.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Result: a computation's outcome: `Err e` or `Ok x`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its D22 sequencing provider instance (`ResultSequencing`: `do`
short-circuits on the first `Err`; the class lives in
Effect.Sequencing).

</details>

# Types and constructors

<a id="type-526573756c74"></a>

## `Result`

```cinch
data Result (e :: Type) (a :: Type)
```

`Result` is here for the same reason: `Data.Result`'s
`partition-results` and `results` consume a `List`, and it too is
checked before `Data.List`.

[Source](/reference-sources/Data.Core.cinch) · line 37

```cinch
Err :: forall e a . (e -> Result e a)
```

```cinch
Ok :: forall e a . (a -> Result e a)
```

# Values

<a id="value-656974686572"></a>

## `either`

```cinch
either :: forall e a b . ((e -> b) -> ((a -> b) -> (Result e a -> b)))
```

`Result`, `Err` and `Ok` are declared in `Data.Core` and re-exported
here (that module's header says why: this home consumes a `List` in
its signatures and is checked before `Data.List`, so one of the two
types has to be declared ahead of both).

The combinators: `Data.Either`'s folds and conversions, as free
functions beside the class instances below. Every one of them is a
total function, and every one of them is a `case` over the two
constructors: no combinator re-wraps an untouched side, so a caller
can see which side it costs.

The two-branch fold: `f` consumes an error, `g` a value. The error
branch leads, matching Haskell's `either` and this type's
constructor order (the error is the first argument everywhere,
including `Result e a` itself).

[Source](/reference-sources/Data.Result.cinch) · line 40

<a id="value-6572726f7273"></a>

## `errors`

```cinch
errors :: forall e a . (List (Result e a) -> List e)
```

The error projection, the values dropped (`Data.Either.lefts`).

[Source](/reference-sources/Data.Result.cinch) · line 103

<a id="value-66726f6d2d726573756c74"></a>

## `from-result`

```cinch
from-result :: forall e a . (a -> (Result e a -> a))
```

The value, or the default when there is an error
(`Data.Either.fromRight`); the default leads, like `from-option`'s.

[Source](/reference-sources/Data.Result.cinch) · line 74

<a id="value-69732d657272"></a>

## `is-err`

```cinch
is-err :: forall e a . (Result e a -> Bool)
```

[Source](/reference-sources/Data.Result.cinch) · line 51

<a id="value-69732d6f6b"></a>

## `is-ok`

```cinch
is-ok :: forall e a . (Result e a -> Bool)
```

The branch tests: the tag read, no payload work, no constraint.

[Source](/reference-sources/Data.Result.cinch) · line 46

<a id="value-6d61702d657272"></a>

## `map-err`

```cinch
map-err :: forall e f a . ((e -> f) -> (Result e a -> Result f a))
```

The error's map (`Data.Either`'s `first`, `mapLeft` in ML-family
spellings: it maps the LEFT type). The value side is untouched.

[Source](/reference-sources/Data.Result.cinch) · line 67

<a id="value-6d61702d726573756c74"></a>

## `map-result`

```cinch
map-result :: forall e a b . ((a -> b) -> (Result e a -> Result e b))
```

The value's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.
The error side is untouched.

[Source](/reference-sources/Data.Result.cinch) · line 60

<a id="value-6f7074696f6e2d746f2d726573756c74"></a>

## `option-to-result`

```cinch
option-to-result :: forall e a . (e -> (Option a -> Result e a))
```

The option as a result, with the error value supplied for `None`
(`Data.Either.note`); the error leads, as everywhere in this module.

[Source](/reference-sources/Data.Result.cinch) · line 122

<a id="value-706172746974696f6e2d726573756c7473"></a>

## `partition-results`

```cinch
partition-results :: forall e a . (List (Result e a) -> (List e, List a))
```

The classic `partitionEithers`: every error and every value, each
side in the input order with its relative order preserved. One pass,
and the recursion happens once per element (the returned tuple is
taken apart by pattern, so neither side is walked twice).

[Source](/reference-sources/Data.Result.cinch) · line 83

<a id="value-726573756c742d746f2d6f7074696f6e"></a>

## `result-to-option`

```cinch
result-to-option :: forall e a . (Result e a -> Option a)
```

The value as an option, the error dropped. This is the projection a
caller reaches for in this repo because `Ok` is the success; the
error-side twin is `Data.Either`'s `hush` and is deliberately not
added (its name would have to be spelled for the error side, and no
caller has asked for it).

[Source](/reference-sources/Data.Result.cinch) · line 115

<a id="value-726573756c7473"></a>

## `results`

```cinch
results :: forall e a . (List (Result e a) -> List a)
```

The value projection, the errors dropped (`Data.Either.rights`).
`partition-results` is the pair form; these two are the single-side
walks a caller reaches for when it wants one side only.

[Source](/reference-sources/Data.Result.cinch) · line 95

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Result.Applicative_Result` | `Applicative (Result e)` | Implicit |
| `Data.Result.Functor_Result` | `Functor (Result e)` | Implicit |
| `Data.Result.Monad_Result` | `Monad (Result e)` | Implicit |
| `Data.Result.SeqResult` | `ResultSequencing e` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Result.cinch`. Source SHA-256: `b35bc2ca2f50d03522fa0e036ee1c735405ed38dce048d7d8e64d54bdc99ef8d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
