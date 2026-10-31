# Data.FromList

The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.FromList`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.FromList.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.FromList: the collection-literal construction class.

`[a, b, c]` desugars to `from-list` at the expected type's
`FromList c e` instance; unconstrained it defaults to `List`
(surface grammar local amendments section 10). Instances opt in per
container and live with their type (Data.List, Data.Bytes,
Data.Set).

</details>

# Types and constructors

# Signatures and methods

<a id="class-46726f6d4c697374"></a>

## `FromList`

```cinch
signature FromList c e
```

Collection literals (surface grammar local amendments, section 10):
`[a, b, c]` desugars to `from-list` at the expected type's `FromList
c e` instance and `{k => v}` to `from-map` over a `List (k, v)` of
pairs at the expected type's `FromMap m k v` instance; unconstrained
they default to `List` and `Map`. Instances opt in per container.

[Source](/reference-sources/Data.FromList.cinch) · line 19

<a id="method-46726f6d4c6973742e66726f6d2d6c697374"></a>

## `FromList.from-list`

```cinch
from-list :: forall c e . using (dict :: FromList c e) => (List e -> c)
```

[Source](/reference-sources/Data.FromList.cinch) · line 20

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/FromList.cinch`. Source SHA-256: `79adbec390aabbf362dc551bdc475da29e2721656cd005dbf263bbab88ba17c4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
