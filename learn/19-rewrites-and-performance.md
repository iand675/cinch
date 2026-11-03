# 19. Laws, rewrites, and performance

An optimization must preserve the program's meaning, including its effects and resource uses. Cinch exposes rewrite rules with explicit justifications so the compiler can record what each transformation depends on.

# State a rule's equation

A `rewrite` declaration binds typed variables, states a `match` expression and a `replace` expression, and supplies `justify` evidence. A rule citing associativity must replace one association of the same operation with another; it cannot use the law's name to justify an unrelated simplification.

For example, the library's `Semigroup.assoc` law describes reassociation of `(<>)`. A justification can cite that law under a `Semigroup` dictionary. If the evidence is an assumption rather than a discharged kernel proof, the artifact records the assumption. The [rewrite reference](../reference/12-rewrites-and-optimization.md) gives the exact declaration form.

# Pure transformations still have conditions

Mapping `f` after mapping `g` can fuse into one traversal when the compiler can justify the transformation and preserve evaluation behaviour. If callbacks perform effects, changing traversal order can change the sequence of effects. Linear resources add another condition: fusion cannot duplicate a consuming operation or erase a required use.

The optimizer respects barriers, rechecks residual code, and records blockers when an intended transformation does not apply. Failure to fuse is not permission to weaken an effect signature. Inspect emitted IR with `cinch emit-ir Main.cinch -o Main.ll` and consult the generated manifest before claiming a performance result.

# Attribute hints

Attributes such as `#[inline(always)]`, `#[noinline]`, and branch weights influence code generation. They are hints with defined placements and validation rules. A likely branch is still checked for type, effect, ownership, and coverage correctness; an unlikely branch is not dead code.

Representation attributes such as alignment or packing change layout obligations and may affect FFI. In particular, a packed field may be safe to copy through marshaling but unsafe to borrow as an ordinarily aligned reference. Use the [complete pragma index](../reference/23-attributes-and-pragmas.md) to find arguments and placement rules, and [type layout](../reference/24-type-layout.md) for worked byte offsets and the current representation limits.

# Measure the work that matters

Check the output as well as speed. Use representative dynamic inputs so constant folding does not replace the computation under measurement. Account for compilation separately from execution, and choose the runtime heap and stack limits deliberately. `CINCH_ALLOC_STATS=1` reports allocation statistics; the benchmark library provides additional tools when shipped with the project.

<span class="support-tag support-tag--planned">Coming soon</span> **broader automatic proofs and representation selection.** The design includes richer proof automation and specialization strategies. Their intended semantics belong in the guide, but a planned optimization is not a performance guarantee for today's compiler.

# Exercise and answer

Why doesn't testing a rewrite on ten inputs establish that it preserves all programs?

<details><summary>Answer</summary>

Finite samples may miss a counterexample and do not cover all effects, resources, or input values. Keep the reported finite-check status. A universal claim needs an appropriate proof, and the compiler must still enforce the rule's applicability conditions.

</details>
