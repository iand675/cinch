# Tutorial 9 Compatible hot upgrades

Source: <https://chatgpt.com/space/page_630a9348c72c8191b3e2cd5fd9f3153d>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn to evolve module state without changing what existing clients are promised. Prerequisite: ownership, contracts, temporal reasoning, and compilation dependencies from earlier tutorials.

# Start with a simple contract

A counter exposes `read` and `increment`. Its observable model is a natural-number total. Version 1 stores one value:

```haskell
record StateV1 where
  value :: Nat
```

Version 2 separates a base and an accumulated delta:

```haskell
record StateV2 where
  base :: Nat
  delta :: Nat
```

Its read operation returns `base + delta`; increment increases `delta` by one. The external contract remains the same.

# Define the migration relation

```haskell
spec related :: StateV1 -> StateV2 -> Prop
spec related old next = Eq old.value (next.base + next.delta)

migrate :: StateV1 %1 -> StateV2
migrate old = StateV2 { base = old.value, delta = 0 }
```

The migration theorem follows from addition's zero law. Increment preservation uses the natural-number arithmetic law relating `base + (delta + 1)` to `(base + delta) + 1`.

These proofs establish the chosen counter model. They do not automatically establish concurrent atomicity, bounded memory, or latency. Those properties would need their own contracts.

# Choose drain and replace

For this example, pause new calls, wait for active calls to finish, and acquire exclusive state ownership. The runtime then migrates the state and publishes the new dispatch epoch. No borrowed pointer into `StateV1` may survive the layout change.

If an active call never finishes, the runtime cannot claim quiescence. It may reject the update or use an explicitly justified cancellation protocol.

# Package evidence and operational requirements

```haskell
upgrade CounterV1 to CounterV2 where
  strategy = DrainAndReplace
  stateMigration = migrate
  relation = related
  evidence = CounterUpgradeProofs
```

`CounterUpgradeProofs` is a required module containing invariant, operation-refinement, and resource-transfer evidence. This declaration is a package sketch; it is not a complete proof merely because the field has a name.

The runtime additionally checks artifact identities, old-version matches, safe-point conditions, and dependency completeness. Static evidence establishes what transitions are valid; coordination establishes that the actual process can take one now.

# Account for specialization

Clients using `import replaceable Counter` call through the stable boundary. A client that specialized or inlined version 1 may need replacement too. The dependency manifest identifies that client; updating a symbol does not rewrite its existing machine code.

Old code stays loaded while any old closure, callback, frame, or destructor still requires it. Rollback requires a valid reverse transition if the state has already changed.

# Exercise and answer

Can the same pattern replace a worker with pending external payments? Only with a richer relation preserving request identities, pending work, unknown outcomes, and temporal obligations. Copying successful records and dropping the rest is incompatible even if the exported function types match.

Next: Tutorial 10 adds target-specific proofs and SIMD rules, including their interaction with hot replacement. The Example catalogue and Open decisions collect conformance cases and implementation priorities.

[Previous tutorial](08-staging-and-schema-generation.md) · [Next tutorial](10-platform-specific-simd.md) · [Documentation overview](../overview.md)