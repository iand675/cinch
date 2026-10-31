# Category.Traversable

Generic effect/container-aware traversal and sequencing while retaining the input shape.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Traversable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Traversable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md), [Category.Monad](Category.Monad.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Traversable: the traversal class over containers.

Class shape: `Traversable (t :: Type -> Type)` with the two
mathematical methods at the applied container:
  traverse :: (a -> f b) -> t a -> f (t b)
  sequence :: t (f a) -> f (t a)
The effect constructor `f` is quantified per method under its
`Applicative f` constraint (a class constraint entails nothing, so
the constraint is stated on each method). The defunctionalized
six-parameter scaffolding (`Traversable t x g h tg hs`) is gone,
`hs`-duplicates-`h` and all: higher-kinded parameters landed and
the class is the mathematical one now.

The instances below are written against the effect's Applicative
dictionary (`pure`/`lift-a2`), so one instance serves every effect.
For `Result` and `Option` this preserves the tested short-circuit
order exactly: their applicatives are left-biased and the
combinations nest in traversal order, so the returned error is
always the FIRST error in traversal order (asserted over sample
values in compiler/tests/lib_foldable.rs).

Instance homes. List and Option got theirs at the prelude split
(Data.List/Data.Option). NonEmpty's instance lives with its type in
Data/NonEmpty.cinch and reuses the List instance on the tail.

</details>

# Types and constructors

# Signatures and methods

<a id="class-5472617665727361626c65"></a>

## `Traversable`

```cinch
signature Traversable t
```

[Source](/reference-sources/Category.Traversable.cinch) · line 30

<a id="method-5472617665727361626c652e7472617665727365"></a>

## `Traversable.traverse`

```cinch
traverse :: forall (t :: Type -> Type) (f :: Type -> Type) a b . using (dict :: Traversable t, app :: Applicative f) => ((a -> (f b)) -> ((t a) -> (f ((t b)))))
```

[Source](/reference-sources/Category.Traversable.cinch) · line 31

<a id="method-5472617665727361626c652e73657175656e6365"></a>

## `Traversable.sequence`

```cinch
sequence :: forall (t :: Type -> Type) f a . using (dict :: Traversable t, app :: Applicative f) => ((t ((f a))) -> (f ((t a))))
```

[Source](/reference-sources/Category.Traversable.cinch) · line 32

# Values

<a id="value-666f72"></a>

## `for`

```cinch
for :: forall (t :: Type -> Type) (f :: Type -> Type) a b . using (trav :: Traversable t, app :: Applicative f) => ((t a) -> ((a -> (f b)) -> (f ((t b)))))
```

`for` is `traverse` with the container first, so a call reads as a
walk over the container.

[Source](/reference-sources/Category.Traversable.cinch) · line 47

<a id="value-666f722d6d"></a>

## `for-m`

```cinch
for-m :: forall (t :: Type -> Type) (m :: Type -> Type) a b . using (trav :: Traversable t, mon :: Monad m, app :: Applicative m) => ((t a) -> ((a -> (m b)) -> (m ((t b)))))
```

[Source](/reference-sources/Category.Traversable.cinch) · line 59

<a id="value-6d61702d6d"></a>

## `map-m`

```cinch
map-m :: forall (t :: Type -> Type) (m :: Type -> Type) a b . using (trav :: Traversable t, mon :: Monad m, app :: Applicative m) => ((a -> (m b)) -> ((t a) -> (m ((t b)))))
```

`map-m`/`for-m` are the monadic traversals (base's `mapM`/`forM`).
Beyond the container they state two constraints on the effect: its
`Applicative`, whose dictionary the traversal actually threads, and
its `Monad`, which is the vocabulary these names belong to. Both
are named because a `Monad` does not entail an `Applicative` (the
no-superbundle rule) and the traversal needs the latter.

[Source](/reference-sources/Category.Traversable.cinch) · line 56

<a id="value-73657175656e63652d61"></a>

## `sequence-a`

```cinch
sequence-a :: forall (t :: Type -> Type) (f :: Type -> Type) a . using (trav :: Traversable t, app :: Applicative f) => ((t ((f a))) -> (f ((t a))))
```

The free combinators. Each states its constraints (the class has no
default methods) and reaches the container through the class
method, so a new instance inherits all of them.

`sequence-a` is `sequence` under its `Applicative` spelling: base
keeps both names for the one function, because the traversal only
ever needed the effect's `Applicative`.

[Source](/reference-sources/Category.Traversable.cinch) · line 42

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Traversable.cinch`. Source SHA-256: `f1ee9b65ce36802f6ac83da08c5c9f8b95bc38fe47dff4fcb607a4d631b083ad`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
