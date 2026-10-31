# Algebra.Semiring

Zero/one and additive/multiplicative structure, with its law vocabulary.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Semiring`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Semiring.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Semiring: the two monoids of a ring-like carrier.

The base library scheme's semiring (design/base-library.md):
`Semiring` names BOTH operations of the `Sum`/`Product` story: `Sum`
(Algebra.Monoid) carries `Nat` under addition and `Product` carries
it under multiplication, and a `Semiring` is the coherent pair on
one carrier (zero and one are the two identities). This is why the
bare `Nat` and `Int` carry a `Semiring` where they carry no
`Semigroup`: a single `<>` cannot say which of the two monoids is
meant, while `add`/`mul` name each. NO Num tower: there is no
superclass chain and no numeric conversion hierarchy (the typed
families with explicit conversions are the spec's deliberate
choice), and machine scalars keep their explicit overflow families
with no `Semiring` instance (primitive types specification
section 8). A class constraint entails nothing (the standing
no-superbundle rule): the class states its four operations alone.

Laws (asserted over sample values in compiler/tests/prelude_split.rs):
  add associativity:      add (add x y) z == add x (add y z)
  add commutativity:      add x y == add y x
  add identity:           add zero x == x, add x zero == x
  mul associativity:      mul (mul x y) z == mul x (mul y z)
  mul identity:           mul one x == x, mul x one == x
  distributivity:         mul x (add y z) == add (mul x y) (mul x z)
                          mul (add y z) x == add (mul y x) (mul z x)
  annihilation:           mul zero x == zero, mul x zero == zero

Instance homes. The ownership rule is "instances live with their
type": `Semiring Nat` lives in Data.Nat and `Semiring Int` in
Data.Int, beside their other instances.

</details>

# Types and constructors

# Signatures and methods

<a id="class-53656d6972696e67"></a>

## `Semiring`

```cinch
signature Semiring a
```

[Source](/reference-sources/Algebra.Semiring.cinch) · line 35

<a id="method-53656d6972696e672e7a65726f"></a>

## `Semiring.zero`

```cinch
zero :: forall a . using (dict :: Semiring a) => a
```

[Source](/reference-sources/Algebra.Semiring.cinch) · line 36

<a id="method-53656d6972696e672e6f6e65"></a>

## `Semiring.one`

```cinch
one :: forall a . using (dict :: Semiring a) => a
```

[Source](/reference-sources/Algebra.Semiring.cinch) · line 37

<a id="method-53656d6972696e672e616464"></a>

## `Semiring.add`

```cinch
add :: forall a . using (dict :: Semiring a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Semiring.cinch) · line 38

<a id="method-53656d6972696e672e6d756c"></a>

## `Semiring.mul`

```cinch
mul :: forall a . using (dict :: Semiring a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Semiring.cinch) · line 39

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Semiring.cinch`. Source SHA-256: `9b3224f1af4c30436de9625c3e38ff11463d85c985f7fd4fcd61856b8bc74b03`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
