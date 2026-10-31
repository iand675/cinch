# Data.Ordering

The LT, EQ, and GT comparison result vocabulary and its instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Ordering`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Ordering.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Ordering: the three-way comparison result.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

# Types and constructors

<a id="type-4f72646572696e67"></a>

## `Ordering`

```cinch
data Ordering
```

[Source](/reference-sources/Data.Ordering.cinch) · line 13

```cinch
LT :: Ordering
```

```cinch
EQ :: Ordering
```

```cinch
GT :: Ordering
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Ordering.Monoid_Ordering` | `Monoid Ordering` | Implicit |
| `Data.Ordering.Semigroup_Ordering` | `Semigroup Ordering` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Ordering.cinch`. Source SHA-256: `7157a1cbcc1e708ab1800e1e48c42e83aca986ee496d29b1f8d9392a708df537`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
