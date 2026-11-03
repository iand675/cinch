# Data.Identity

The identity container and its generic abstraction instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Identity`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Identity.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The identity type: the universal visitor that changes nothing. It is
the unit of the category family (the identity functor and the trivial
monad), so its operations below are the instance method bodies the
Category.* classes carry as this type's instances.

Module name: `Data.Identity` (the module scheme's name,
design/base-library.md; the temporary single-segment header is gone
now that dotted module names resolve).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition, through a `using` binder with a
DirectSequencing/ResultSequencing/OptionSequencing head (default:
DirectSequencing). A do-provider is a fixed instance choice: exactly
one container per definition, no polymorphism. The Category.*
classes (Functor/Applicative/Monad) abstract over containers for
polymorphic code; their `map`/`pure`/`>>=` are ordinary class
methods resolved per use site. The two mechanisms do not interact
and neither is changing: `do` never consults the classes, and a class
constraint never selects a do provider.

The functions and the instances: the functions below are the
instance method bodies (Functor `map`, Applicative `pure`/`<*>`/
`lift-a2`, Monad `>>=`/`join`; there is no `return`, decision D152).
The classes have landed (higher-kinded parameters), so the instances
at the bottom are defined through these bodies: the plain functions
stay as the type's direct API and the class methods resolve to them.
The operators `<$>`, `<*>` and `>>=` are class methods per decision
D2 (operators are class methods with per-module fixity
declarations), so no per-type operator stand-ins are defined here.

</details>

# Types and constructors

<a id="type-4964656e74697479"></a>

## `Identity`

```cinch
data Identity (a :: Type)
```

[Source](/reference-sources/Data.Identity.cinch) · line 36

```cinch
Identity :: forall a . (a -> Identity a)
```

# Values

<a id="value-61702d6964656e74697479"></a>

## `ap-identity`

```cinch
ap-identity :: forall a b . (Identity (a -> b) -> (Identity a -> Identity b))
```

Applicative body: apply a wrapped function to a wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 54

<a id="value-62696e642d6964656e74697479"></a>

## `bind-identity`

```cinch
bind-identity :: forall a b . (Identity a -> ((a -> Identity b) -> Identity b))
```

Monad body: sequence a dependent computation.

[Source](/reference-sources/Data.Identity.cinch) · line 63

<a id="value-6a6f696e2d6964656e74697479"></a>

## `join-identity`

```cinch
join-identity :: forall a . (Identity (Identity a) -> Identity a)
```

Monad body: collapse one layer.

[Source](/reference-sources/Data.Identity.cinch) · line 68

<a id="value-6c6966742d61322d6964656e74697479"></a>

## `lift-a2-identity`

```cinch
lift-a2-identity :: forall a b c . ((a -> (b -> c)) -> (Identity a -> (Identity b -> Identity c)))
```

Applicative body: lift a two-argument function.

[Source](/reference-sources/Data.Identity.cinch) · line 59

<a id="value-6d61702d6964656e74697479"></a>

## `map-identity`

```cinch
map-identity :: forall a b . ((a -> b) -> (Identity a -> Identity b))
```

Functor body: map over the wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 45

<a id="value-707572652d6964656e74697479"></a>

## `pure-identity`

```cinch
pure-identity :: forall a . (a -> Identity a)
```

Applicative body.

[Source](/reference-sources/Data.Identity.cinch) · line 50

<a id="value-72756e2d6964656e74697479"></a>

## `run-identity`

```cinch
run-identity :: forall a . (Identity a -> a)
```

Elimination: the wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 40

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Identity.Applicative_Identity` | `Applicative Identity` | Implicit |
| `Data.Identity.Functor_Identity` | `Functor Identity` | Implicit |
| `Data.Identity.Monad_Identity` | `Monad Identity` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Identity.cinch`. Source SHA-256: `2429ddda85cbdaaef07d640c7df7cb7e78e6537c30ff3920ac0ca1675a5922e9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
