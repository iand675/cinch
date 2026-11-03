# Rewrites, laws, specialization, and optimizer reports

A rewrite is an explicit transformation with typed binders, a matching expression, a replacement, and justification. It changes representation/execution while preserving the program's contract.

# Declaration form

The following is a declaration fragment inside a module that imports `Algebra.Semigroup`:

```text
rewrite reassociate forall (x :: U32) (y :: U32) (z :: U32).
  using (sg :: Semigroup U32) where
    match ((x <> y) <> z)
    replace (x <> (y <> z))
    justify Semigroup.assoc x y z
```

The cited law must establish the rule's equation. A law name is not permission to replace an unrelated expression. Kernel equality evidence and signature law methods are accepted in their supported contexts; undischarged law assumptions are recorded. Effectful rewrite equations are rejected. Definitional equality remains a kernel relation and is not extended by user optimizer rules.

`target rewrite` scopes a rule to a target condition with required feature evidence. Baseline code remains necessary when the feature is unavailable. Selection belongs to the artifact manifest; a representation choice should not change an otherwise identical source contract.

# Laws and instance checking

Signatures can state laws. Primitive/instance checking can evaluate admitted finite samples. Reports distinguish successful sample checks, failures, assumptions, and checks that could not be performed. `not checked` is not a successful proof, and `checked` finite samples are not a universal theorem.

Refinement instance transfer adds preservation obligations, named per refinement and method. A method whose identity violates the predicate is rejected even if the carrier implementation is an otherwise valid monoid. Defaults are checked through the selected dictionary.

# Optimization preservation

The optimizer can specialize concrete dictionaries, fold constants and cases, fuse admitted map pipelines, and retain effects in order. It uses barriers for linear values, capabilities, memo sharing, codata productivity, erasure, and rigid indices. A transformation cannot duplicate a consumed resource, reorder visible I/O, or turn a partial producer into a productive stream.

Residual programs are rechecked. Driving fuel and a whistle bound specialization and generalization; hitting a bound yields residual code and a report, not an invalid partial optimization. Reports name blockers such as a linear value, capability scope, or exhausted fuel. Branch weights are reconciled through transformations rather than copied into a changed control-flow graph without adjustment.

# Hints, measurement, and JIT

Inlining/weight hints are representation facts and do not change interface contract identity. Inspect `emit-ir` and the entry manifest to confirm what happened. A target-specific body being available is different from that body being selected.

The versioned runtime has variant install/retire and consumption-safe deoptimization mechanisms. Their existence does not imply a user-facing automatic JIT service or a CLI tiering switch. See [runtime evolution](17-upgrades-and-temporal-contracts.md).

<span class="support-tag support-tag--planned">Coming soon</span> **handler-vtable specialization and step certificates.** Concrete handler propagation across function boundaries remains a specialization boundary. The higher trust level with independently checkable per-step certificates is planned; current full residual rechecking must not be described as that future certificate format.

Implementation: `core.rs`, `specialize.rs`, `check_decls.rs`, `codegen.rs`; suites: `optimizer.rs`, `laws.rs`, `prim.rs`, `attributes.rs`, `evolution.rs`.
