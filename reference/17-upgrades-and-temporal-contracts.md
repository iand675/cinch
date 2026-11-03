# Runtime upgrades and temporal contracts

An upgrade must preserve state compatibility and the assumptions of live code. Temporal contracts additionally constrain how facts evolve across checked transitions.

# Checked migrations and views

`upgrade OldState to NewState where migrate = migration` associates a migration with two schemas. The migration must have the admitted exactly-once old-to-new type. Field changes are classified, incompatible retyping requires a migration, and migration-free record views need recorded compatibility evidence.

The interface retains schema/upgrade facts. A plain data conversion in a tutorial proves only that conversion's behaviour; it does not by itself perform a live rollout. [The upgrade tutorial](../learn/21-upgrades-and-contracts.md) makes that distinction with a complete runnable migration.

# Versioned dispatch

The runtime retains code with live frames, gates activation on complete update sets, and prevents old code resuming over incompatible newly migrated state. Target compatibility is checked before reachability. Variant installation, retirement, and consumption-safe deoptimization use the same machinery. These behaviours are exercised through compiled programs calling runtime evolution entry points.

<span class="support-tag support-tag--planned">Coming soon</span> **update packages and operator tooling.** The specifications describe a complete checked update/deployment lifecycle. There is no current `cinch upgrade` CLI, service supervisor, or end-user package activation workflow. Document the runtime mechanisms as current and the operational workflow as planned.

# Temporal contract declarations

The admitted form names a kind and definition, followed by formulas/assumptions:

```text
contract safety invariantName for definition where
  always = predicate
  assume = predicate
```

Supported kinds are safety, liveness, recurrence, and stability. The checker records verified facts and their assumptions in interfaces/manifests. A named external assumption remains an assumption.

| Kind | Current checking boundary |
| --- | --- |
| Safety (`always`) | Check the invariant over admitted transition/write steps; reject a violating or undecidable step. |
| Stability (`stable`) | Preserve a fact exactly across steps that do not invalidate its footprint. |
| Liveness (`eventually`) | Finite structured obligations such as joined tasks/cleanup, bounded by their scope. |
| Recurrence (`recur`) | Supported structural-recursion cycles with the required witness on each cycle. |

A write overlapping a predicate's footprint invalidates the old fact. A liveness declaration does not prove eventual success of arbitrary network access. Nonstructural or unbounded temporal claims receive a named refusal instead of being accepted unchecked.

<span class="support-tag support-tag--planned">Coming soon</span> **general unbounded temporal reasoning.** Richer process/temporal proofs remain part of the intended language design. Until their supported proof procedure exists, use the finite fragment and state external assumptions honestly.

Implementation: `check_decls.rs`, `interface.rs`, runtime `evolution.rs`; suites: `evolution.rs`, `temporal.rs`, `evidence_invalidation.rs`. Intended semantics: [runtime evolution specification](../specifications/runtime-evolution-specification.md) and [core specification](../specifications/core-language-specification.md).
