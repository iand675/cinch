# Runtime evolution specification

Source: <https://chatgpt.com/space/page_d92d79c6a7748191817f7baeb2489fe5>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Draft 0.2. Hot replacement is an intended language and runtime feature. This chapter describes the compatibility evidence and operational conditions required to replace a running module. The runtime protocol and package format are proposals.

# 1 Replacement unit

The replacement unit is a versioned module instance with explicit state, resources, exported operations, and dependencies. Pure stateless functions are a simpler case. Arbitrary mutation of machine-code addresses is not the programming interface.

A checked upgrade package identifies the exact source and target artifacts, old and new contracts, migration, compatibility evidence, activation strategy, and environmental assumptions. Artifact identity MUST be authenticated or otherwise verified by the deployment mechanism; an unrelated binary cannot borrow another artifact's proof.

Compatibility means refinement of the published contract for existing clients. It does not require identical internal representation or identical behavior in distinctions the contract intentionally leaves unspecified.

# 2 Compatibility obligations

| Dimension | Required condition |
| --- | --- |
| Inputs | Inputs admitted by old clients remain admitted, directly or through a checked adapter |
| Outputs | Old promised postconditions and observable protocols remain satisfied |
| Effects | Required behavior stays within the allowed effect and authority contract |
| State | Migration establishes a relation between old and new valid states |
| Resources | Ownership and outstanding obligations transfer without duplication or abandonment |
| In-flight work | Calls, messages, callbacks, and suspended tasks have an explicit strategy |
| Temporal properties | Required progress survives the transition under stated assumptions |
| Representation | ABI, layout, dispatch, and destructors remain valid for their consumers |
| Dependencies | Inlined bodies, type equations, and law-dependent specializations remain valid or are replaced |

Fewer effects do not automatically imply behavioral refinement. A replacement that stops sending a required response may have a smaller effect set while violating the protocol. Likewise, weakening a precondition or strengthening a postcondition is necessary in the ordinary contract setting but not a complete stateful compatibility proof.

# 3 State relations and migration

An upgrade defines a relation `R(oldState, newState)` over logical views. Migration must establish the new invariant and the relation whenever it reports success. Failure must satisfy the declared recovery contract.

```haskell
signature Upgrade (old :: Module) (next :: Module) where
  -- View/State/Plan stand for the associated families of these modules.
  spec related :: OldView old -> NewView next -> Prop
  prepare :: forall region e. Ref region (OldState old)
          ->{|e} Result PrepareError (Plan old next)
  commit :: forall e. Plan old next %1 ->{|e} NewState next
  -- Implementation also supplies the invariant, resource and trace proofs.
```

This is a schematic package shape. A real package additionally specifies the invariants, resource permissions, failure and cancellation behavior of `commit`, and any outstanding-work relation. A function returning `New.State` does not by itself prove migration.

For transition-system proofs, a simulation can relate corresponding operations and observations. Safety simulation alone is insufficient for liveness: matching each old step with arbitrarily many internal new steps can permit infinite internal stuttering. A liveness-preserving update needs a divergence-sensitive argument, a progress measure, or another justified temporal refinement.

# 4 Preparation and activation

The proposed activation protocol is:

1. Load the new artifact and check package identity, evidence, assumptions, and deployment policy.

2. Determine the dependent update set and prepare any adapters or replacement artifacts.

3. Obtain the declared safe point, drain boundary, or version-coexistence arrangement.

4. Prepare migration while respecting its ownership and external-effect contract.

5. Revalidate state or generation guards immediately before commit.

6. Transfer state and resource ownership and atomically publish the new dispatch epoch at the declared scope.

7. Retire old code only after all tracked references and obligations permit it.

If preparation reads mutable state, intervening writes can invalidate the plan. The runtime must freeze the relevant state, validate a generation counter, or use a proved concurrent migration protocol. Publishing a pointer atomically does not make a multi-object state migration atomic by itself.

An in-process activation point is not a cluster-wide transaction. Distributed deployments require a separate rollout and protocol-compatibility plan. This draft does not imply atomic upgrades across independent machines.

# 5 In-flight execution strategies

| Strategy | Preconditions and consequences |
| --- | --- |
| Drain and replace | Admission pauses; relevant existing calls finish; migration obtains exclusive access |
| Version coexistence | Old executions retain their version; new calls use the new version; shared state needs an explicit cross-version protocol |
| Continuation migration | Suspended computations expose declared upgrade points and a checked state and continuation mapping |

Drain may fail to complete if an old call never returns. A timeout can reject the update or follow an explicit cancellation contract; it cannot manufacture a safe migration point. Version coexistence does not allow arbitrary simultaneous access to incompatible state representations.

Continuation migration is restricted to declared checkpoints. The runtime must know the live state, resources, exception and cancellation context, outstanding requests, and continuation protocol. Ordinary dependent typing does not automatically prove that arbitrary machine stacks can be transformed.

# 6 Ownership and code lifetime

The runtime MUST retain a code version while live stack frames, closures, callbacks, vtables, destructors, exception cleanup paths, foreign registrations, or other tracked dependencies can enter it. Untracked foreign references prevent safe unloading until their contract is satisfied.

A closure carries its code version and environment representation. New code cannot reinterpret an old closure's environment layout. Existing values with version-specific destructors retain access to those destructors until disposal.

State layout migration requires exclusive access or a proved synchronization scheme. Borrowed views cannot remain live across a representation change unless an adapter or stable representation keeps them valid. Resource identity must be tracked where it matters: moving a socket transfers its ownership; it does not open a logically equivalent replacement socket by default.

# 7 Types and abstract identity

Public reducible type-level definitions are part of the caller's verified assumptions. A replacement MUST NOT change the meaning of an index function under existing checked code. Callers depending on changed equations must be rebuilt and replaced, or compatibility must be established under a permitted abstract boundary.

Abstract types have stable identities only under explicitly defined module-evolution rules. New representations may remain abstractly compatible through a migration relation, but raw old and new representations are not interchangeable. Version-indexed state and capability types can prevent accidental mixing.

Proof objects erased from executable code still matter through the assumptions and optimizations they authorized. The artifact manifest must preserve those dependencies after erasure.

# 8 Replaceable and specialized imports

```haskell
import replaceable Cache
import specialized NumericKernels
```

A replaceable import uses a stable dispatch boundary with a compatible interface or adapter. Its ABI includes relevant ownership, layout, effect, and cancellation conventions. A specialized import permits inlining, fusion, and representation-sensitive optimization; the consumer becomes dependent on the corresponding implementation and law versions.

When a law or implementation changes, the runtime coordinator MUST reject an incomplete update set. It may install a compatible adapter, preserve the old version, or replace affected dependents coherently. It MUST NOT claim that changing one symbol updates already-inlined machine code.

Indirection and specialization are deliberate performance tradeoffs. The compiler SHOULD show the dependent set and explain why a module cannot be replaced in isolation.

# 9 Temporal obligations and external outcomes

Pending obligations survive version changes. If a module accepted request `r`, an upgrade cannot erase `r` from the progress contract. Correlation identifiers, deduplication records, deadlines, and unknown external outcomes may therefore be part of migration state.

Exactly-once external effects cannot be derived from local type compatibility. A network timeout may leave an outcome unknown. The upgraded implementation must preserve the external protocol's uncertainty and recovery rules.

An infinite sequence of otherwise compatible upgrades can starve useful work. A deployment policy claiming liveness must constrain updates, preserve progress measures, or establish a suitable fairness theorem for the combined scheduler and updater.

Real-time deadlines and resource budgets are preserved only if declared, modeled, and checked under a specified cost or timing model. Extensional functional equality alone does not preserve latency or peak memory.

# 10 Failure, cancellation, and rollback

Preparation should be reversible where possible. Commit marks the point at which old ownership ceases and new ownership begins. If commit can fail, its type and recovery protocol must describe which version owns every resource and whether service can resume.

Rollback is not the inverse of forward migration by default. A reverse package may be required, and external effects may be irreversible. The runtime MUST NOT resume old code over a new state representation without an authorized reverse transition.

Crash consistency is a separate contract. A volatile in-process migration does not survive power loss unless state and ownership transfer are durably recorded under a proved recovery protocol.

# 11 Required diagnostics

An update rejection SHOULD identify the failing dimension: incompatible input, missing law, live borrow, non-draining task, stale migration plan, unresolved external outcome, changed type equation, missing dependent artifact, or unproved temporal obligation.

Successful activation reports the artifact identities, dispatch epoch, evidence profile, coexistence status, and remaining old-code references. This is operational evidence of activation, not a replacement for the static compatibility proof.

# 12 Initial implementation scope

The recommended first runtime supports drain-and-replace for explicitly versioned modules with private state and pure or well-scoped migration. Version coexistence follows once code lifetimes and shared-state protocols are modeled. Continuation migration and distributed rollout are later extensions with their own proofs.

These milestones preserve the full hot-swapping goal while making the first implementation's actual guarantees testable.

# 13 Target compatibility during upgrades

An update that changes instruction requirements MUST establish availability before the new code becomes reachable. The compatibility manifest includes target profiles, intrinsic-semantics versions, arithmetic modes, and platform laws used by specialized consumers. Existing frames retain the code and target assumptions under which they were admitted. A change from a scalar implementation to SIMD cannot bypass memory-access, feature-dispatch, or dependent-update obligations.

# Foreign operations, components and task scopes

Quiescence includes registered callbacks, in-flight host operations, child tasks, queued messages and foreign handle destructors. A cancellation request alone does not release their ownership or code-version dependencies. The old artifact remains resident until those dependencies are migrated or discharged.

Wasm resources retain their owning instance identity. Replacement uses checked handle translation, draining or version coexistence; copying linear memory or reusing raw resource integers is insufficient. Concurrent snapshots need synchronization or commit-time revalidation, and old and new code sharing state need a joint invariant. See [Concurrency specification](concurrency-specification.md) and [WebAssembly and WASI specification](webassembly-and-wasi-specification.md).

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Upgrade surface (D82-D87), 2026-10-28.** This amends sections 1
  and 3: `upgrade Old to New where { migrate = f, related = r }` maps
  the reserved component names; `migrate` is the checked migration
  `Old %1 -> New` and migration field checking is field-change
  classification (kept/added/removed/retyped) with a migration required
  for incompatible changes and a type match on the migration itself
  (D82). The migration and `related` components name visible values;
  the bootstrap evolves monomorphic records only (D83, a named limit).
  A migration-free `upgrade` is a record view admitting only additive
  changes; added optional fields are recorded as missing-able, keeping
  missing distinct from present-`None` at the schema-evidence level
  (the value-level `Missing`/`Present` marker needs a partial-record
  view type former the bootstrap `Option` lacks, a named follow-up)
  (D84). `related` is accepted as evidence syntax but its proposition
  type is not yet checked (D87, riding that follow-up).
- **Evolution runtime and representation identity (D85-D86),
  2026-10-28.** This amends sections 6 and 8: runtime evolution is the
  one code-retention/dispatch mechanism (the supercompilation chapter's
  rule), and value-level transforms dispatch on `(from_repr, to_repr)`
  (named follow-up) (D85). `#[repr]`-level changes are a representation
  change: they alter the interface identity so stale dependents must be
  replaced, which the existing staleness chain detects (D86).
- **JIT variants (supercompilation section 6), 2026-10-28.** Installing
  or retiring a specialized variant follows this chapter only: old code
  versions are retained while live frames can enter them, update sets
  complete before activation, and new instruction requirements are
  established before reachability (D92-D101 machinery).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources `.cinch`, interfaces `.cinchif`, runtime externs `cinch_*`;
  artifact names in this chapter read with those names.