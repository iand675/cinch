# Platform proofs and SIMD specification

Source: <https://chatgpt.com/space/page_b24cfef55f608191b1494b251d782fe4>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Draft 0.2. Platform-specific proofs and rewrite rules are a required part of the design. Libraries may provide scalar, SIMD, and other target-specialized implementations under explicit target assumptions. This chapter proposes how those assumptions enter typing, verification, dispatch, and hot replacement.

# 1 Target descriptions

A target profile MUST identify the instruction semantics and deployment assumptions used by a proof. Relevant fields include instruction-set version, feature set, data layout, endianness, pointer representation, ABI, memory model, vector shape, and floating-point behavior. The proof need only depend on the fields it uses, but those dependencies must be recorded.

A target triple or architecture name is not sufficient evidence that a particular instruction can execute on the deployment machine. The actual runtime environment must satisfy the selected feature and operating-environment contract. Cross-compilation MUST distinguish generator-host capabilities from target capabilities.

Profiles and instruction models are versioned. Changing an intrinsic's modeled semantics invalidates dependent proof artifacts unless a checked compatibility argument preserves them.

# 2 Target capabilities as evidence

Proposed interfaces include a logical target description and a capability whose construction establishes that the required operations may execute in the current environment:

```haskell
signature VecU32Target (t :: Target) (w :: Nat) where
  proof positiveWidth :: w > 0
  -- Intrinsics, memory preconditions, modeled semantics and lowering identity.
  -- See the platform contract table for the required associated operations.
```

This is schematic. Feature detection is a trusted or verified runtime boundary, not a proof of physical hardware behavior produced from nothing. It must account for all execution requirements in the target contract, rather than testing only an architecture string.

The capability's validity scope MUST be defined. If a process can move to an incompatible execution environment, the runtime must preserve a baseline guarantee, constrain placement, revalidate safely, or otherwise prevent using stale availability evidence. Thread-specific vector configuration must not be silently assumed to apply to every thread.

# 3 Conditional rewrite declarations

A rewrite may be parameterized by a target module and require both logical and operational evidence:

```haskell
target rewrite sumWrap32Vector
  forall t w n region (xs :: Ref region (Vec n U32)).
  using (vectors :: VecU32Target t w) where
  match Scalar.sumWrap32 xs
  replace Simd.sumWrap32 vectors xs
  require Available vectors
  justify SumProofs.scalarToSimd vectors xs
```

The compiler MUST apply the rule only in a region where the required capability is valid, or under a deployment profile that supplies it. An assumed deployment baseline must be enforced at artifact admission. A runtime-guarded variant must not execute unsupported instructions while evaluating the guard or taking the fallback.

Target rules remain optimizer rules. They MUST NOT alter global definitional equality or make a source type mean something different merely because another machine is compiling it. Explicit target-indexed types may legitimately differ, and their target dependencies are then part of the interface.

# 4 SIMD proof obligations

| Obligation | Required reasoning |
| --- | --- |
| Lane computation | Each lane implements the selected scalar operation with the same bit width and arithmetic mode |
| Memory access | Addresses, allocation bounds, pointer provenance, and alignment satisfy the intrinsic contract |
| Remainders | Short inputs and partial final blocks do not introduce invalid accesses |
| Aliasing | Reordered or widened reads and writes preserve the required dependency and ownership rules |
| Control and effects | Speculated or reordered operations preserve errors, effects, and observable termination |
| Reduction | Reassociation is justified by the operation's actual semantics |
| Representation | Lane order, conversions, masks, and ABI agree with the target model |
| Exceptional behavior | Traps, floating-point flags, NaNs, signed zero, and other declared observations are preserved |

Loading a full vector and masking the result afterward is not a proof that out-of-range lanes were never accessed. A masked-load lowering needs the specific memory-access semantics promised by its intrinsic and backend. A scalar remainder loop is the proposed conservative default when no such proof is available.

Vectorizing ordinary immutable memory does not authorize vectorizing volatile I/O or arbitrary atomics. Their access granularity and ordering may be observable. Such transformations need dedicated semantics and laws.

# 5 A precise first example

Use `U32.wrappingAdd` for an initial reduction rule. The scalar model is the sum modulo `2^32`; its associativity permits lane accumulation and a final horizontal reduction.

For width `W > 0` and input length `n`, let `q = n div W` and `r = n mod W`. Process `q` complete blocks, reduce lane totals with wrapping addition, and process the remaining `r` elements scalarly. The proof combines modular arithmetic with a loop invariant relating the consumed prefix to the accumulator model.

The memory proof must establish `q * W <= n`, full-block load bounds, and safe conversion from mathematical indices to target pointer arithmetic. Evaluating a bounds formula with overflowing machine arithmetic is not a valid substitute for that proof.

The rule is not valid for checked addition merely because both operations use 32-bit storage. Different regrouping can change which checked operation overflows. Nor does it justify floating-point reassociation. Those variants require separate contracts.

# 6 Fixed and scalable vector shapes

Fixed-width variants may quantify over a compile-time positive width. Scalable-vector variants need a model parameterized by the active runtime vector length and its permitted changes. They MUST NOT assume an observed length remains fixed beyond the target contract's validity scope.

Proofs can be generic over a target module if the module exposes sufficient instruction laws. Architecture-specific modules discharge those laws against their own instruction semantics. This avoids duplicating the entire algorithm proof while keeping target assumptions explicit.

# 7 Dispatch and profitability

A portable artifact may contain multiple verified implementations and a scalar fallback. Dispatch checks required capabilities and chooses among valid candidates. Selection based on estimated cost does not establish correctness; all selectable paths already need evidence.

An ahead-of-time build may omit the fallback when deployment admission guarantees the required profile. The requirement must be recorded and enforced rather than silently assumed. A valid vectorized implementation may be slower for short inputs; profitability remains a cost-model or profiling question unless the contract includes a proved cost bound.

Dispatch caching must respect capability validity. Cached target availability cannot outlive the environment or thread configuration on which it relies.

# 8 Backend trust and certificates

A proof of source-to-vector-IR refinement does not establish that arbitrary backend machine code implements that IR. The artifact MUST identify which lowering and instruction semantics were checked and which backend stages remain trusted.

Possible implementations include verified lowerings, instruction certificates, or translation validation. A bounded validator result must retain its bound and modeling restrictions; it cannot be relabeled as a theorem for arbitrary loop iterations. Proof certificates checked by the language kernel are the preferred long-term interface where practical.

The trust boundary includes the fidelity of the instruction model and execution environment. Hardware faults and undocumented deviations are outside the proof unless modeled explicitly.

# 9 Fusion and hot replacement

Fusion can expose vectorizable loops, and target specialization can lower them further. Each stage must preserve the preceding stage's contract; a proof of fusion is not automatically a proof of vectorization.

The dependency manifest records target profiles, feature requirements, intrinsic semantics, arithmetic modes, and laws used by specialized consumers. Hot replacement that introduces new instruction requirements must establish their availability before activation. Existing frames may still require the old variant.

Upgrading a target module or rewrite law may require replacing dependent specialized code. Replaceable dispatch boundaries retain the ability to select a compatible implementation, at the cost of limiting unrestricted cross-boundary specialization.

# 10 Related implementation references

[LLVM's language reference](https://llvm.org/docs/LangRef.html) specifies vector types, masked memory operations, and other IR-level details a backend proof must respect. It is an IR contract, not an end-to-end guarantee for this proposed language. [Alive2](https://github.com/AliveToolkit/alive2) and the [bounded translation validation paper](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf) are useful references for checking transformations and understanding bounded validation's scope.

# Wasm validation and feature selection

For Wasm, feature dispatch may need to occur before module instantiation. Unsupported SIMD instructions can invalidate an entire module even when their branch would not execute. Package separate baseline and specialized variants when required, detect supported features and select a valid variant before loading. The proof manifest must match the selected module and target semantics; relaxed SIMD requires its own semantic relation. See [WebAssembly and WASI specification](webassembly-and-wasi-specification.md).

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Target model and capability evidence (D58-D59), 2026-10-28.** This
  amends sections 2-3: target profiles and vector shapes are marker
  types (`VecU32Target t w`, `CapAvailable t w`) and `Available v` is
  the operational evidence of this chapter's validity scope (lexical
  region + thread; no cached evidence). A `target rewrite` fires only
  where the firing unit carries a matching capability parameter, and
  `Target.detectVecU32` is a replaceable dispatch boundary (never
  folded).
- **Admission split (D96, D99), 2026-10-28.** `cinch check` is not
  artifact admission: `admit_artifact` enforces the assumed deployment
  baseline this chapter requires, and world-export implementation
  obligations (a `world` export without its implementation is
  rejected). The canonical ABI is the wasm32 shape (i32 pointer slots,
  canonical memory layout), so adapter glue emits for Wasm artifact
  profiles only and native canonical emission refuses with a named
  diagnostic (D99 revises D96's FFI-shim remark: there is no native
  canonical lowering).
- **LLVM contract facts (D187-D191), 2026-10-28.** This amends section
  8's checked-lowering facts: `!range` is stated only for `Bool` and
  the narrow primitive scalars (`Fin` values are constructor-block
  pointers and full-width spans are unstatable, both refused as
  unsound); `llvm.assume` has no `range` operand bundle in this LLVM,
  so range facts use the `range` return attribute and `!range`
  metadata; `willreturn`/`nofree` are scan-proven and transitive
  (checked definitions restate them only under their P20/totality rows)
  and the unit scan classifies every call, load, store, and poison
  source so effectful units never claim `memory(none)`. Aggregate
  parameter spelling: `byref`/`sret`, see the FFI chapter's amendment.
- **Cross-compilation widths (A4, closed P14), 2026-10-28.** `Usize`/
  `Isize` widths and capability checks key on the target profile, never
  the generator host (rows 27-28 of `compiler/SPEC-MAPPING.md`).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources `.cinch`, interfaces `.cinchif`, runtime externs `cinch_*`;
  artifact names in this chapter read with those names.