# Data.Ascii

ASCII-refined values and checked construction at text boundaries.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Ascii`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Ascii.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Ascii: the ASCII string as a refinement (refinement types
design; decision D151).

`type Ascii = refine (s :: String) with Ascii s`: `Ascii` IS `String`
at runtime (the witness is `Prop`-erased, decision D31). The
predicate `Ascii` decides text values (`String`, and `Bytes` for the
buffer-level twin); over anything else it is refused naming the
predicate. `String` wraps its `Bytes` backing (decision D88), so the
decision procedure reads the bytes directly.

The total constructor `to-ascii` discharges at runtime through the
`Dec`/`decide` world and returns `Option`; the transparent
constructor `Ascii s w` takes the string and its witness.

The instance below is the container's own `<>` (string
concatenation, the transfer rule): the concatenation of ASCII
strings is ASCII (preserved), and concatenation is associative;
both checked sample-wise by the law layer.

</details>

# Types and constructors

<a id="type-4173636969"></a>

## `Ascii`

```cinch
type Ascii  = refine (s :: String) with Ascii s
```

[Source](/reference-sources/Data.Ascii.cinch) · line 25

# Values

<a id="value-746f2d6173636969"></a>

## `to-ascii`

```cinch
to-ascii :: (String -> Option Ascii)
```

The total constructor (refinement types design: the total API).

[Source](/reference-sources/Data.Ascii.cinch) · line 28

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Ascii.Semigroup_Ascii` | `Semigroup Ascii` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Ascii.cinch`. Source SHA-256: `5ca0b2a99f6380073373fdff71fd0e3e95eb4575eb5ccc6aa56a407c8eb284b2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
