# Data.Core

Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Core`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Core.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Core: the inductive types the base homes share, declared in one
place so their public homes can name each other in SIGNATURES.

Why this module exists. `Data.List` answers an `Option` (`nth`), and
`Data.Option` consumes a `List` (`cat-options`), so the two homes are
mutually referential at the signature level. The family resolves a
signature's type names against the modules checked before it, so no
order of those two homes can work: whichever comes first cannot see
the other's type. Declaring both types here, ahead of every home, and
re-exporting them from their public homes breaks the cycle without
either home losing its surface.

What stays where. A type's INSTANCE family stays in its public home
(`Data.List` carries `List`'s instances, `Data.Option` carries
`Option`'s), because that is the module a caller imports for the
type's behavior; this module declares the types and their
constructors, and nothing else. It is not re-exported by the facade:
the names reach the implicit surface through `Data.List` and
`Data.Option`, exactly as before.

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Core.cinch`. Source SHA-256: `3ee42eb9b63a1cffe608355914e5c8295561ca8b17bea4d3b39b87f3e2f2a106`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
