# Data.Target

Target/platform and worker vocabulary used by the compiler-supported runtime and target examples.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Target`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Target.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Option](Data.Option.md), [Data.Producer](Data.Producer.md), [Relation.Eq](Relation.Eq.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Target: target profiles and capability evidence.

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58). The u32-sum
capability (`VecU32Target`) establishes that the sum intrinsics may
execute in the current environment (platform proofs and SIMD
specification section 2); its method is the width evidence a
generic proof consumes (section 6). `CapAvailable` is the
operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it; a rewrite's `require` clause names it.

`simd-sum-wrap32` is the SIMD intrinsic (the scalar/SIMD pair): its
bootstrap lowering is the scalar loop, and the artifact manifest
records which lowering was checked (platform section 8).
`detect-vec-u32` is capability dispatch (platform section 7): the
bootstrap runtime performs no feature detection, so dispatch
conservatively selects the scalar path. The boundary is
replaceable: the optimizer never folds this call or the `case`
around it.

`SumProofs.scalar-to-simd` is the instruction law (platform section
5): the scalar model and the SIMD intrinsic compute the same sum.
Discharging it is a proof obligation of the target library; a
rewrite citing it records the assumption in the artifact manifest
(decision D55).

</details>

# Types and constructors

<a id="type-415658325f323536"></a>

## `AVX2_256`

```cinch
data AVX2_256
```

Target profiles and the scalar/SIMD pair (P10): target capability
evidence, the scalar/SIMD pair of the platform chapter's section 5
example, capability dispatch, and the step-form producer library
that fusion composes (modules and compilation specification
section 6).

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58).

[Source](/reference-sources/Data.Target.cinch) · line 47

```cinch
AVX2_256Tag :: AVX2_256
```

<a id="type-436170417661696c61626c65"></a>

## `CapAvailable`

```cinch
data CapAvailable (t :: Type) (w :: Type)
```

Operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it. A rewrite's `require` clause names it.

[Source](/reference-sources/Data.Target.cinch) · line 69

```cinch
Available :: forall t w . (VecU32Target t w -> CapAvailable t w)
```

<a id="type-45696768744c616e6573"></a>

## `EightLanes`

```cinch
data EightLanes
```

[Source](/reference-sources/Data.Target.cinch) · line 56

```cinch
EightLanesTag :: EightLanes
```

<a id="type-4e454f4e3634"></a>

## `NEON64`

```cinch
data NEON64
```

[Source](/reference-sources/Data.Target.cinch) · line 50

```cinch
NEON64Tag :: NEON64
```

<a id="type-5363616c61626c655368617065"></a>

## `ScalableShape`

```cinch
data ScalableShape
```

[Source](/reference-sources/Data.Target.cinch) · line 53

```cinch
ScalableShapeTag :: ScalableShape
```

# Signatures and methods

<a id="class-53756d50726f6f6673"></a>

## `SumProofs`

```cinch
signature SumProofs 
```

The instruction law (platform section 5): the scalar model and the
SIMD intrinsic compute the same sum. Discharging it is a proof
obligation of the target library; a rewrite citing it records the
assumption in the artifact manifest (decision D55).

[Source](/reference-sources/Data.Target.cinch) · line 89

<a id="method-53756d50726f6f66732e7363616c61722d746f2d73696d64"></a>

## `SumProofs.scalar-to-simd`

```cinch
scalar-to-simd :: forall t w n region . using (dict :: SumProofs) => ((vectors :: VecU32Target t w) -> ((xs :: Ref region Vec n U32) -> Eq (Data.Producer.scalar-sum-wrap32 xs) (Data.Target.simd-sum-wrap32 vectors xs)))
```

[Source](/reference-sources/Data.Target.cinch) · line 90

<a id="class-566563553332546172676574"></a>

## `VecU32Target`

```cinch
signature VecU32Target t w
```

The u32-sum capability: its construction establishes that the sum
intrinsics may execute in the current environment (platform proofs
and SIMD specification section 2). The method is the width evidence
a generic proof consumes (section 6).

[Source](/reference-sources/Data.Target.cinch) · line 63

<a id="method-5665635533325461726765742e706f7369746976652d7769647468"></a>

## `VecU32Target.positive-width`

```cinch
positive-width :: forall t w . using (dict :: VecU32Target t w) => (Unit -> Nat)
```

[Source](/reference-sources/Data.Target.cinch) · line 64

# Values

<a id="value-6465746563742d7665632d753332"></a>

## `detect-vec-u32`

```cinch
detect-vec-u32 :: (Unit -> Option (VecU32Target AVX2_256 EightLanes))
```

Capability dispatch (platform section 7): the bootstrap runtime
performs no feature detection, so dispatch conservatively selects
the scalar path. The boundary is replaceable: the optimizer never
folds this call or the `case` around it.

[Source](/reference-sources/Data.Target.cinch) · line 82

<a id="value-73696d642d73756d2d777261703332"></a>

## `simd-sum-wrap32`

```cinch
simd-sum-wrap32 :: forall t w (n :: Nat) region . (VecU32Target t w -> (Ref region Vec n U32 -> U32))
```

The SIMD intrinsic (the scalar/SIMD pair): its bootstrap lowering is
the scalar loop, and the artifact manifest records which lowering
was checked (platform section 8).

[Source](/reference-sources/Data.Target.cinch) · line 75

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Target.cinch`. Source SHA-256: `a1d9ceb7eb29444f9a35ecf745234876432be54a81d615d2353b61a8cd1c411e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
