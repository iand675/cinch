# Algebra.Semigroup

Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Semigroup`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Semigroup.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Semigroup: associative binary combination on a carrier.

`Semigroup` is the value-level class of the base library scheme
(design/base-library.md): one operator method `<>`, associative by
law. This module declares the operator and owns its fixity. Consumers
inherit the declaration through imports and cannot redeclare it.

Laws (asserted over sample values in compiler/tests/lib_algebra.rs):
  associativity: (x <> y) <> z == x <> (y <> z)
A class constraint entails nothing (the standing no-superbundle
rule): this module states no Monoid import and every function names
exactly the constraints it uses.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module". The carriers that were parked here (List, Option,
Ordering, String) got their instances at the prelude split: they
live in the types' Data.* homes now. Function and tuple instances
are structural and here permanently. The instance-carrying newtypes
(Sum, Product, Dual, First, Last, Endo) are declared in
Algebra.Monoid with both of their instances, because an instance
lives with its type and a newtype's home is its class module.

</details>

# Types and constructors

# Signatures and methods

<a id="class-53656d6967726f7570"></a>

## `Semigroup`

```cinch
signature Semigroup a
```

[Source](/reference-sources/Algebra.Semigroup.cinch) · line 27

<a id="method-53656d6967726f75702e3c3e"></a>

## `Semigroup.<>`

```cinch
<> :: forall a . using (dict :: Semigroup a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Semigroup.cinch) · line 28

# Values

<a id="value-636f6d62696e652d7061697273"></a>

## `combine-pairs`

```cinch
combine-pairs :: forall a . using (sa :: Semigroup a) => (List a -> List a)
```

One round of a balanced combine: adjacent pairs combined in order,
an odd last value carried unchanged (shared with Algebra.Monoid's
`mconcat`).

[Source](/reference-sources/Algebra.Semigroup.cinch) · line 81

<a id="value-73636f6e636174"></a>

## `sconcat`

```cinch
sconcat :: forall a . using (s :: Semigroup a) => (List a -> Option a)
```

The class-generic folds. These are free functions with a `using`
constraint, not new class methods (the scheme of `sum-list` and
`enum-from-to`): the class is the interface and reaching one of
these costs an explicit import of this module, since the facade
re-exports no class home.

`sconcat`: combine the whole list with `<>`, in order, or `None`
when there is nothing to combine. The carrier is `List a` and the
emptiness is reported as an `Option` (the prelude's partiality
convention). `NonEmpty` would be the more precise argument, but its
home (Data.NonEmpty) imports this module for `<>`, so naming it here
would be a cycle; the `Option` result is the version that lives in
this module.

The bracketing is balanced (rounds of `combine-pairs`), which the
associativity law makes equal to base's right fold `foldr1 (<>)`
with the same n - 1 combinations; for a `<>` that copies its
operands (String) the right fold re-copies the accumulated suffix at
every step, O(n^2) bytes, where the rounds copy O(n log n).

[Source](/reference-sources/Algebra.Semigroup.cinch) · line 71

<a id="value-7374696d6573"></a>

## `stimes`

```cinch
stimes :: forall a . using (s :: Semigroup a) => (Nat -> (a -> a))
```

`stimes n x`: `n` copies of `x` combined with `<>`. A `Nat` cannot
be negative, but it can be zero, and a bare `Semigroup` has no
identity to return; base's `stimes 0` is an error and the prelude
refuses faults, so `0` reads as one copy. That is the reading under
which the law `stimes (m + n) x == stimes m x <> stimes n x` is
stated for `m, n >= 1`. By repeated doubling (associativity makes
any bracketing of the n copies equal): O(log n) combinations, where
chaining one copy at a time costs n - 1 and, for a `<>` that copies
its operands, O(n^2) bytes.

[Source](/reference-sources/Algebra.Semigroup.cinch) · line 97

# Operator fixities

```cinch
infixr 6 <>
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Algebra.Semigroup.Semigroup_Function` | `Semigroup (t -> u)` | Implicit |
| `Algebra.Semigroup.Semigroup_Pair` | `Semigroup (t, u)` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Semigroup.cinch`. Source SHA-256: `f932fc5a9425e91a4823c988d6196b7e8afea0c5c2d23066e9d8bd2ae8fef68e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
