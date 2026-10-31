# Algebra.Bits

Bit operations, shifts, and the corresponding operator/fixity vocabulary.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Bits`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Bits.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Bits: the bit manipulation class.

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers (primitive types
specification section 4). All methods are total: the bitwise
operations are total, shifts reduce the count modulo the width (the
wrapping family), and rotates are total. The checked shift family
stays as the per-type `checkedShlX`/`checkedShrX` primitives.
`test-bit`/`set-bit`/`clear-bit`/`complement-bit` follow the mask
rule: bit position n means position n modulo the width, consistent
with the shift rule. Instances live with their type (Data.Machine).

The traditional operator spellings below are wrappers over the
class methods (decision D90), like the weight wrappers, never new
compiler mechanisms. Bitwise NOT has no operator spelling because
both candidate symbols are reserved punctuation (`!` and `~`); use
`bit-not`.

</details>

# Types and constructors

# Signatures and methods

<a id="class-42697473"></a>

## `Bits`

```cinch
signature Bits a
```

Bit manipulation (primitive types specification section 4).

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers. All methods are total,
following the specification: the bitwise operations are total,
shifts reduce the count modulo the width (the wrapping family), and
rotates are total. The checked shift family stays as the per-type
`checkedShlX`/`checkedShrX` primitives. `test-bit`/`set-bit`/
`clear-bit`/`complement-bit` are expressed through the mask rule: bit
position n means position n modulo the width, consistent with the
shift rule.

[Source](/reference-sources/Algebra.Bits.cinch) · line 41

<a id="method-426974732e6269742d616e64"></a>

## `Bits.bit-and`

```cinch
bit-and :: forall a . using (dict :: Bits a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 42

<a id="method-426974732e6269742d6f72"></a>

## `Bits.bit-or`

```cinch
bit-or :: forall a . using (dict :: Bits a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 43

<a id="method-426974732e6269742d786f72"></a>

## `Bits.bit-xor`

```cinch
bit-xor :: forall a . using (dict :: Bits a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 44

<a id="method-426974732e6269742d6e6f74"></a>

## `Bits.bit-not`

```cinch
bit-not :: forall a . using (dict :: Bits a) => (a -> a)
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 45

<a id="method-426974732e73686966742d6c656674"></a>

## `Bits.shift-left`

```cinch
shift-left :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 46

<a id="method-426974732e73686966742d7269676874"></a>

## `Bits.shift-right`

```cinch
shift-right :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 47

<a id="method-426974732e726f746174652d6c656674"></a>

## `Bits.rotate-left`

```cinch
rotate-left :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 48

<a id="method-426974732e726f746174652d7269676874"></a>

## `Bits.rotate-right`

```cinch
rotate-right :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 49

<a id="method-426974732e746573742d626974"></a>

## `Bits.test-bit`

```cinch
test-bit :: forall a . using (dict :: Bits a) => (a -> (U32 -> Bool))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 50

<a id="method-426974732e7365742d626974"></a>

## `Bits.set-bit`

```cinch
set-bit :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 51

<a id="method-426974732e636c6561722d626974"></a>

## `Bits.clear-bit`

```cinch
clear-bit :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 52

<a id="method-426974732e636f6d706c656d656e742d626974"></a>

## `Bits.complement-bit`

```cinch
complement-bit :: forall a . using (dict :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 53

<a id="method-426974732e636f756e742d6f6e6573"></a>

## `Bits.count-ones`

```cinch
count-ones :: forall a . using (dict :: Bits a) => (a -> U32)
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 54

<a id="method-426974732e6c656164696e672d7a65726f73"></a>

## `Bits.leading-zeros`

```cinch
leading-zeros :: forall a . using (dict :: Bits a) => (a -> U32)
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 55

<a id="method-426974732e747261696c696e672d7a65726f73"></a>

## `Bits.trailing-zeros`

```cinch
trailing-zeros :: forall a . using (dict :: Bits a) => (a -> U32)
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 56

# Values

<a id="value-26"></a>

## `&`

```cinch
& :: forall a . using (bits :: Bits a) => (a -> (a -> a))
```

Traditional bitwise operator spellings over the `Bits` class
(decision D90): wrappers like the weight wrappers, never new
compiler mechanisms. Bitwise NOT has no operator spelling because
both candidate symbols are reserved punctuation (`!` and `~`); use
`bit-not`. Machine integers deliberately get no arithmetic operator
instances (primitive types specification section 8): `%` and `/`
below cover the mathematical `Nat` and `Int` only.

[Source](/reference-sources/Algebra.Bits.cinch) · line 65

<a id="value-3c3c"></a>

## `<<`

```cinch
<< :: forall a . using (bits :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 74

<a id="value-3e3e"></a>

## `>>`

```cinch
>> :: forall a . using (bits :: Bits a) => (a -> (U32 -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 77

<a id="value-5e"></a>

## `^`

```cinch
^ :: forall a . using (bits :: Bits a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 71

<a id="value-7c"></a>

## `|`

```cinch
| :: forall a . using (bits :: Bits a) => (a -> (a -> a))
```

[Source](/reference-sources/Algebra.Bits.cinch) · line 68

# Operator fixities

```cinch
infixl 6 &
infixl 7 <<
infixl 7 >>
infixl 5 ^
infixl 4 |
```

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Bits.cinch`. Source SHA-256: `8901a9f4e10c73cbb55e7bb14808c700a7c7e14b7142d222808696754429c712`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
