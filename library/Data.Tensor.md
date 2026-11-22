# Data.Tensor

Immutable row-major tensors with checked runtime shapes of any rank.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Tensor`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Tensor.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Core](Data.Core.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Tensor](Data.Tensor.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable row-major arrays, with checked runtime shapes of any rank.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-74656e736f722d64617461"></a>

## `tensor-data`

```cinch
tensor-data :: forall a . (Tensor a -> Array a)
```

[Source](/reference-sources/Data.Tensor.cinch) · line 9

<a id="value-74656e736f722d676574"></a>

## `tensor-get`

```cinch
tensor-get :: forall a . (Tensor a -> (Array Nat -> Option a))
```

[Source](/reference-sources/Data.Tensor.cinch) · line 10

<a id="value-74656e736f722d72657368617065"></a>

## `tensor-reshape`

```cinch
tensor-reshape :: forall a . (Array Nat -> (Array a -> Option (Tensor a)))
```

[Source](/reference-sources/Data.Tensor.cinch) · line 7

<a id="value-74656e736f722d736574"></a>

## `tensor-set`

```cinch
tensor-set :: forall a . (Tensor a -> (Array Nat -> (a -> Option (Tensor a))))
```

[Source](/reference-sources/Data.Tensor.cinch) · line 11

<a id="value-74656e736f722d7368617065"></a>

## `tensor-shape`

```cinch
tensor-shape :: forall a . (Tensor a -> Array Nat)
```

[Source](/reference-sources/Data.Tensor.cinch) · line 8

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Tensor.cinch`. Source SHA-256: `caeef1f823e842a73b16519af6e7baa9de0ac0ddb00d31472ed9461116836eb7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
