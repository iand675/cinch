# Tutorial 10 Platform specific SIMD

Source: <https://chatgpt.com/space/page_cb25fa2f09f881918ae41fba88919548>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn to expose a portable operation with target-specific verified implementations. Prerequisite: Tutorial 7 on rewrites and Tutorial 9 on compiled dependencies. The SIMD APIs and rule syntax below are proposals, not implemented intrinsics.

# Choose exact arithmetic

```haskell
sumWrap32 :: forall n region. Ref region (Vec n U32) -> U32
sumWrap32 xs = Stream.fold U32.zero U32.wrappingAdd (Vec.stream xs)
```

The contract is the sum modulo `2^32`. That arithmetic is associative, so a vector implementation may accumulate independent lanes and reduce them at the end. This statement is not a theorem about checked addition or floating-point addition.

# State the target requirement

```haskell
target rewrite sumWrap32Vector
  forall t w n region (xs :: Ref region (Vec n U32)).
  using (vectors :: VecU32Target t w) where
  match Scalar.sumWrap32 xs
  replace Simd.sumWrap32 vectors xs
  require Available vectors
  justify SumProofs.scalarToSimd vectors xs
```

`V` supplies a positive lane count, intrinsic semantics, memory preconditions, and a lowering identity. Availability evidence connects that model to the permitted execution environment. The law proves the algorithm under the model; detection does not independently prove the hardware implements the model.

# Prove complete blocks and the tail

For width four and length ten, process elements zero through seven with two full loads, then elements eight and nine scalarly. The general proof uses `n = q * W + r`, with `r < W`.

Maintain an invariant equating the modular sum of the consumed prefix with the accumulator lanes. Prove each load remains within the allocation and has the alignment its intrinsic requires. Pointer-index calculations must not overflow their target representation.

Do not load twelve elements and mask away the final two afterward. Those memory accesses already happened. A different implementation may use a masked-load intrinsic, but it needs evidence that inactive lanes are not accessed by that lowering.

# Select a safe implementation

```haskell
case Target.detectVecU32 () of
  Some available -> Simd.sumWrap32 available xs
  None -> Scalar.sumWrap32 xs
```

The successful branch unpacks a target dictionary and its availability evidence. The actual type may be existential in the vector width. Detection and the fallback must themselves run on the baseline platform.

For a scalable target, quantify over the permitted active vector length and respect its validity scope. Do not treat it as a universal compile-time constant.

# Keep updates compatible

An upgraded SIMD implementation may require a different feature set or arithmetic contract. Activation checks those requirements and updates the dependent specialization set. Existing old-version frames and closures retain their valid code until they finish.

# Exercise and answer

Can a vectorized floating-point sum reuse the wrapping-integer theorem? No. Floating-point regrouping can change values and other declared observations. Require an exact theorem for the chosen semantics or an explicit approximate-result contract with its own error proof.

[Previous tutorial](09-compatible-hot-upgrades.md) · [Documentation overview](../overview.md)

# Continue reading

[Tutorial 9 Compatible hot upgrades](09-compatible-hot-upgrades.md) · [Tutorial 11 Foreign libraries](11-foreign-libraries.md) · [Language design overview](../overview.md)