# 21. State evolution and temporal contracts

Changing a running program raises a stronger question than whether its new source typechecks. Existing state and active frames still belong to a particular version. An upgrade must account for both before new code can safely observe old data.

# Write a migration

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record OldState where
    count :: Nat

  record NewState where
    base :: Nat
    extra :: Nat

  migrate :: OldState %1 -> NewState
  migrate old = NewState { base = old.count, extra = 0 }

  upgrade OldState to NewState where
    migrate = migrate

  main :: Int
  main =
    let current = migrate (OldState { count = 5 })
    in if current.base + current.extra == 5 then 0 else 1
```

The migration consumes old state exactly once and produces the new state. The `upgrade` declaration records the relationship. This example verifies a migration; it does not operate a live deployment service. A field-type change or incompatible schema requires an appropriate migration rather than a view that silently reinterprets memory.

# Active code also has a version

The runtime retains code while live frames need it. Complete update sets gate activation, and old code cannot resume over newly migrated state. Target compatibility is checked before reachability. These are separate requirements from the migration function's type, so a correct field mapping alone does not establish a safe runtime update.

<span class="support-tag support-tag--planned">Coming soon</span> **a complete user-facing update workflow.** The Rust implementation has migration checks, metadata, and runtime dispatch mechanisms exercised by tests. The CLI does not expose a service supervisor or an update-package activation command. See [runtime evolution reference](../reference/17-upgrades-and-temporal-contracts.md) for the mechanisms and intended workflow.

# Temporal claims have a scope

A safety contract states an invariant over checked transitions. Stability depends on whether writes touch the fact's footprint. Liveness is currently admitted for finite structured obligations such as joined tasks and cleanup. Recurrence is checked in the supported structural-recursion fragment.

An arbitrary claim that a server will eventually answer every request is stronger than these supported checks. It needs assumptions about scheduling, external services, and unbounded execution that the finite checker does not establish. The compiler refuses unsupported temporal claims rather than treating their syntax as proof.

# Exercise and answer

If a migration adds `extra = 0`, why can a still-running old function be unsafe even when it never mentions `extra`?

<details><summary>Answer</summary>

It may retain an old layout, state identity, or continuation assumption. Compatibility requires controlling which code version can run against which state version, not merely checking whether a new field is mentioned. The runtime's frame retention and activation rules address that relationship.

</details>
