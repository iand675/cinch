# Tutorial 6 Processes and temporal contracts

Source: <https://chatgpt.com/space/page_2d2960d3ed2c8191a601fdeb3e741de8>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn how to specify a service that intentionally runs forever. Prerequisite: Tutorial 5. Temporal formulas are specification notation; they are not runtime assertions that can inspect an infinite future.

# Give requests identities

Imagine a worker that accepts jobs and eventually records an outcome. Track each job by `JobId`, with states such as `Queued`, `Running`, `Succeeded`, and `Failed`. Keep completed-job history if the protocol uses it to prevent duplicate completion.

An abstract model might have transitions `Accept(id)`, `Start(id)`, and `Finish(id, outcome)`. The state invariant says a job has one status and cannot move back from a terminal status to `Running`.

Prove the invariant by checking the initial state and each transition. That inductive proof covers all reachable states even though the service can execute infinitely many transitions.

# State safety and progress separately

```haskell
always (completionCount requestId <= 1)
always (implies (accepted requestId) (eventually (completed requestId)))
```

The first formula forbids duplicate completion records. The second promises eventual completion for every accepted job. To establish the second, the protocol needs assumptions such as eventual scheduling, terminating job execution or a specified timeout outcome, and recoverable dependency behavior.

A job that is permanently disabled is not rescued by weak fairness. Weak fairness requires an action that remains enabled to eventually execute; strong fairness concerns actions enabled repeatedly. Choose the actual needed assumption rather than adding fairness as an unexplained label.

# Connect the implementation

Define a relation from concrete worker state to the abstract model: queue entries map to `Queued`, owned work items map to `Running`, and durable outcomes map to terminal states. Each executable operation must correspond to an allowed abstract transition or a justified internal step.

For liveness, internal steps cannot stutter forever while a response obligation is pending. A progress measure and scheduler assumptions must connect implementation execution to the temporal theorem.

# Preserve uncertainty at external boundaries

A timeout after submitting a payment may mean the payment failed, succeeded, or remains pending. The state machine should contain an unknown-outcome state and a reconciliation transition if the external protocol requires them. Returning `Failed` merely because the local socket timed out can violate the intended contract.

The same issue matters during hot replacement: the new version must inherit pending and unknown outcomes, not just the successful records.

# Exercise and answer

If a model checker explores ten jobs and all allowed queue states under that limit, has it proved the unbounded implementation correct? No. That result covers the modeled instance. A parameterized proof or a justified abstraction theorem is needed to extend it.

Next: Tutorial 7 turns semantic laws into executable optimization rules.

[Previous tutorial](05-streams-and-coinduction.md) · [Next tutorial](07-rewrites-and-stream-fusion.md) · [Documentation overview](../overview.md)