# Data.Compose

Composition of higher-kinded containers with the admitted generic instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Compose`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Compose.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Compose: composition of type constructors, the composing piece
of the category family.

`Compose f g a` wraps `f (g a)`: the composite of `f` after `g` at
`a`. `get-compose` is the elimination.

Instance homes: the Functor/Applicative/Monad instances of the
composite arrive with the prelude split's instance homes (they need
the component instances as constraints); the type ships first so the
family's shape is complete.

</details>

# Types and constructors

<a id="type-436f6d706f7365"></a>

## `Compose`

```cinch
data Compose (f :: Type) (g :: Type) (a :: Type)
```

[Source](/reference-sources/Data.Compose.cinch) · line 13

```cinch
Compose :: forall f g a . ((f ((g a))) -> Compose f g a)
```

# Values

<a id="value-6765742d636f6d706f7365"></a>

## `get-compose`

```cinch
get-compose :: forall (f :: Type -> Type) (g :: Type -> Type) a . (Compose f g a -> (f ((g a))))
```

[Source](/reference-sources/Data.Compose.cinch) · line 16

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Compose.cinch`. Source SHA-256: `e3ccc746196d71bfa0a30c6e77e7faaef88b3373244beee3a051ffbe1b205b80`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
