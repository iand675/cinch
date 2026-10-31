# Algebra.Group

Group-style inverse/subtraction vocabulary over admitted carrier instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Group`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Group.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Group: carriers with a total inverse.

The base library scheme's group (design/base-library.md):
`Group` adds `invert` to the associative `<>` with identity
`mempty` (Algebra.Semigroup, Algebra.Monoid). A class constraint
entails nothing (the standing no-superbundle rule): the class
states `invert` alone, and the laws below are the law shape the P18
law-axiom layer will formalize.

Laws (asserted over sample values in compiler/tests/prelude_split.rs):
  left inverse:  invert x <> x == mempty
  right inverse: x <> invert x == mempty
For `Int` the monoid is the semiring's additive one (`add`/`zero`,
Algebra.Semiring), the form the tests state.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module" (design/base-library.md). `Int`'s instance lives in
Data.Int with its family; `Dual`'s lives with its type in
Algebra.Monoid; the function and tuple instances are structural and
here. `Nat` gets no `Group` instance: its subtraction truncates
(there is no additive inverse below zero).

</details>

# Types and constructors

# Signatures and methods

<a id="class-47726f7570"></a>

## `Group`

```cinch
signature Group a
```

[Source](/reference-sources/Algebra.Group.cinch) · line 27

<a id="method-47726f75702e696e76657274"></a>

## `Group.invert`

```cinch
invert :: forall a . using (dict :: Group a) => (a -> a)
```

[Source](/reference-sources/Algebra.Group.cinch) · line 28

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Algebra.Group.Group_Function` | `Group (t -> u)` | Implicit |
| `Algebra.Group.Group_Pair` | `Group (t, u)` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Group.cinch`. Source SHA-256: `e247f07f004aed06c6c258d5b4184f12186ffcc1bbefec740799112f38168d3c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
