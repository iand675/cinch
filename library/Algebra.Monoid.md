# Algebra.Monoid

Identity plus combination vocabulary, default folding, and unit laws with explicit checking status.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Monoid`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Monoid.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Group](Algebra.Group.md), [Algebra.Semigroup](Algebra.Semigroup.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Monoid: semigroups with an identity element.

`Monoid` adds the identity to the associative `<>`
(Algebra.Semigroup). The import below is explicit because a class
constraint entails nothing (the standing no-superbundle rule):
`Monoid` does not extend `Semigroup`, and every function states the
constraints it uses. The imported operator keeps the fixity declared
by Algebra.Semigroup; this module cannot redeclare it.

`mempty` is the nullary class method `mempty :: a` (the higher-kinded
slice lifted the function-type requirement on class methods): the
dictionary carries the identity value directly, so call sites write
`mempty`.

Laws (asserted over sample values in compiler/tests/lib_algebra.rs):
  left unit:  mempty <> x == x
  right unit: x <> mempty == x
plus associativity of `<>` from Algebra.Semigroup.

Instance homes. The newtypes below exist only to carry instances, so
the base library scheme puts them with their class module (here).
Every instance lives with its type: the newtypes' Semigroup and
Monoid instances are both here because the newtypes are declared
here. The carriers that were parked here (List, Option, Ordering,
String) got their instances at the prelude split: they live in the
types' Data.* homes now. Structural instances over functions and
tuples live with the class module permanently; the Semigroup halves
of those live in Algebra.Semigroup. `Dual`'s `Group` instance is
here too (it lives with its type): the opposite group of any group.

</details>

# Types and constructors

<a id="type-416c6c"></a>

## `All`

```cinch
data All
```

Bool under conjunction. A list of `Bool` folds through `Monoid`
only under an explicit choice of which boolean monoid is meant
(the bare `Bool` carries none), so `All` picks conjunction and
`Any` picks disjunction, the pair base's `Data.Monoid` names. The
identity of conjunction is `True`, so `mconcat Nil` (an `All`) is
`All True`.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 110

```cinch
All :: (Bool -> All)
```

Bool under conjunction. A list of `Bool` folds through `Monoid`
only under an explicit choice of which boolean monoid is meant
(the bare `Bool` carries none), so `All` picks conjunction and
`Any` picks disjunction, the pair base's `Data.Monoid` names. The
identity of conjunction is `True`, so `mconcat Nil` (an `All`) is
`All True`.

<a id="type-416e79"></a>

## `Any`

```cinch
data Any
```

Bool under disjunction: `Any`'s identity is `False`, so
`mconcat Nil` (an `Any`) is `Any False`.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 123

```cinch
Any :: (Bool -> Any)
```

Bool under disjunction: `Any`'s identity is `False`, so
`mconcat Nil` (an `Any`) is `Any False`.

<a id="type-4475616c"></a>

## `Dual`

```cinch
data Dual (t :: Type)
```

Dual: composition in the opposite order. `Dual x <> Dual y`
combines y before x, turning any monoid into its opposite.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 136

```cinch
Dual :: forall t . (t -> Dual t)
```

Dual: composition in the opposite order. `Dual x <> Dual y`
combines y before x, turning any monoid into its opposite.

<a id="type-456e646f"></a>

## `Endo`

```cinch
data Endo (t :: Type)
```

Endo: endomorphisms under composition. `Endo f <> Endo g` is
`f` after `g` (like `(.)`), and `mempty` is the identity, so
folding `<>` over endomorphisms chains them left to right.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 185

```cinch
Endo :: forall t . ((t -> t) -> Endo t)
```

Endo: endomorphisms under composition. `Endo f <> Endo g` is
`f` after `g` (like `(.)`), and `mempty` is the identity, so
folding `<>` over endomorphisms chains them left to right.

<a id="type-4669727374"></a>

## `First`

```cinch
data First (t :: Type)
```

First: the leftmost `Some`, in a wrapper that keeps this choice
apart from Option's own first-some reading (they agree today; the
wrapper makes the intent explicit and leaves Option free).

[Source](/reference-sources/Algebra.Monoid.cinch) · line 157

```cinch
First :: forall t . (Option t -> First t)
```

First: the leftmost `Some`, in a wrapper that keeps this choice
apart from Option's own first-some reading (they agree today; the
wrapper makes the intent explicit and leaves Option free).

<a id="type-4c617374"></a>

## `Last`

```cinch
data Last (t :: Type)
```

Last: the rightmost `Some`.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 170

```cinch
Last :: forall t . (Option t -> Last t)
```

Last: the rightmost `Some`.

<a id="type-50726f64756374"></a>

## `Product`

```cinch
data Product
```

Nat under multiplication.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 93

```cinch
Product :: (Nat -> Product)
```

Nat under multiplication.

<a id="type-53756d"></a>

## `Sum`

```cinch
data Sum
```

The instance-carrying newtypes (design/base-library.md: "the
newtypes that exist only to carry instances live with their class
module"). Their instances live with their type, so both the
Semigroup and Monoid instances of each are declared here.

Nat under addition: `Sum x <> Sum y` adds. The bare `Nat` gets no
Semigroup instance (two monoids live on Nat; the newtype chooses).

[Source](/reference-sources/Algebra.Monoid.cinch) · line 81

```cinch
Sum :: (Nat -> Sum)
```

The instance-carrying newtypes (design/base-library.md: "the
newtypes that exist only to carry instances live with their class
module"). Their instances live with their type, so both the
Semigroup and Monoid instances of each are declared here.

Nat under addition: `Sum x <> Sum y` adds. The bare `Nat` gets no
Semigroup instance (two monoids live on Nat; the newtype chooses).

# Signatures and methods

<a id="class-4d6f6e6f6964"></a>

## `Monoid`

```cinch
signature Monoid a
```

[Source](/reference-sources/Algebra.Monoid.cinch) · line 36

<a id="method-4d6f6e6f69642e6d656d707479"></a>

## `Monoid.mempty`

```cinch
mempty :: forall a . using (dict :: Monoid a) => a
```

[Source](/reference-sources/Algebra.Monoid.cinch) · line 37

<a id="method-4d6f6e6f69642e6d636f6e636174"></a>

## `Monoid.mconcat`

```cinch
mconcat :: forall a . using (dict :: Monoid a, sa :: Semigroup a) => (List a -> a)
```

`mconcat`: combine the whole list with `<>`, in order, or `mempty`
for the empty list. This is the operation the text homes kept
rebuilding as a private `concat-strings`; they call it now.

A method with a default (D388), so a carrier whose `<>` copies its
operands can override it with a one-pass combine (`Monoid String`
does) while every other instance inherits the default below. The
`Semigroup a` constraint is the method's own, not the class's (a
class constraint entails nothing, the standing no-superbundle
rule): the default needs it for `<>`, so every caller supplies it,
exactly as the free function it replaces required.

The default's bracketing is BALANCED: each round combines adjacent
pairs, and rounds repeat until one value remains. Associativity (the
Semigroup law) makes every bracketing of the same sequence equal, so
this equals the right fold `foldr (<>) mempty` (the right unit law
covers dropping the fold's trailing `mempty`), in n - 1 combinations
for n values. What the bracketing changes is the cost of a `<>` that
copies its operands: the right fold re-copies the whole accumulated
suffix at every step, O(n^2) bytes, while each balanced round copies
every byte once, O(n log n) over the log n rounds. An override must
equal the default. The rounds are Algebra.Semigroup's balanced
`sconcat`, with `mempty` for the empty list.

[Source](/reference-sources/Algebra.Monoid.cinch) · line 61

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Algebra.Monoid.Group_Dual` | `Group (Dual t)` | Implicit |
| `Algebra.Monoid.Monoid_All` | `Monoid All` | Implicit |
| `Algebra.Monoid.Monoid_Any` | `Monoid Any` | Implicit |
| `Algebra.Monoid.Monoid_Dual` | `Monoid (Dual t)` | Implicit |
| `Algebra.Monoid.Monoid_Endo` | `Monoid (Endo t)` | Implicit |
| `Algebra.Monoid.Monoid_First` | `Monoid (First t)` | Implicit |
| `Algebra.Monoid.Monoid_Function` | `Monoid (t -> u)` | Implicit |
| `Algebra.Monoid.Monoid_Last` | `Monoid (Last t)` | Implicit |
| `Algebra.Monoid.Monoid_Pair` | `Monoid (t, u)` | Implicit |
| `Algebra.Monoid.Monoid_Product` | `Monoid Product` | Implicit |
| `Algebra.Monoid.Monoid_Sum` | `Monoid Sum` | Implicit |
| `Algebra.Monoid.Semigroup_All` | `Semigroup All` | Implicit |
| `Algebra.Monoid.Semigroup_Any` | `Semigroup Any` | Implicit |
| `Algebra.Monoid.Semigroup_Dual` | `Semigroup (Dual t)` | Implicit |
| `Algebra.Monoid.Semigroup_Endo` | `Semigroup (Endo t)` | Implicit |
| `Algebra.Monoid.Semigroup_First` | `Semigroup (First t)` | Implicit |
| `Algebra.Monoid.Semigroup_Last` | `Semigroup (Last t)` | Implicit |
| `Algebra.Monoid.Semigroup_Product` | `Semigroup Product` | Implicit |
| `Algebra.Monoid.Semigroup_Sum` | `Semigroup Sum` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Monoid.cinch`. Source SHA-256: `7f71172956f30dbc67374308cbb6a582b905921936183ff366d2a05c56e05258`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
