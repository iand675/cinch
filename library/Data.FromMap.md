# Data.FromMap

The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.FromMap`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.FromMap.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.FromMap: the map-literal construction class.

`{k => v}` desugars to `from-map` over a `List (k, v)` of pairs at
the expected type's `FromMap m k v` instance; unconstrained it
defaults to `Map` (surface grammar local amendments section 10).
Instances live with their type (Data.Map, Data.HashMap).

</details>

# Types and constructors

# Signatures and methods

<a id="class-46726f6d4d6170"></a>

## `FromMap`

```cinch
signature FromMap m k v
```

[Source](/reference-sources/Data.FromMap.cinch) · line 13

<a id="method-46726f6d4d61702e66726f6d2d6d6170"></a>

## `FromMap.from-map`

```cinch
from-map :: forall m k v . using (dict :: FromMap m k v) => (List (k, v) -> m)
```

[Source](/reference-sources/Data.FromMap.cinch) · line 14

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/FromMap.cinch`. Source SHA-256: `af8cdaf5ec98b0c87e4c9b2c85fabfa72cb0602ea5f14ba50da7ee9e99311fcf`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
