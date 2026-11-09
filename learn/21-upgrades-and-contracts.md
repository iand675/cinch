# 21. State evolution and temporal contracts

Changing a program that keeps running asks a harder question than whether the new source typechecks. The data the old version wrote is still there, and the frames that are still executing belong to the code that started them. An upgrade has to answer for both before new code observes old data, and a temporal claim has to say which transitions it covers, over which version of the state. This chapter shows what the compiler checks today, the evidence each check leaves behind, and the claims it refuses.

Prerequisites: [modules](08-modules.md) for records, [ownership and regions](12-ownership-and-regions.md) for the `%1` multiplicity, and [structured concurrency](17-structured-concurrency.md) for `withScope`, `spawn`, and `joined`.

# Version the state

Two records describe the state before and after a change, and one function maps old to new:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    base :: Nat
    delta :: Nat

  migrate :: StateV1 %1 -> StateV2
  migrate s = StateV2 { base = s.value, delta = 0 }

  upgrade StateV1 to StateV2 where
    migrate = migrate

  viewV2 :: StateV2 -> Nat
  viewV2 s = s.base + s.delta

  main :: Int
  main = if viewV2 (migrate (StateV1 { value = 5 })) == 5 then 0 else 1
```

`migrate` has the one type an upgrade accepts, `Old %1 -> New`. The `%1` is the obligation that the old state is consumed exactly once, not copied and not dropped: a migration that could read the old value twice would let one version's data be observed by two owners. The `upgrade` declaration names the pair and binds the function that performs it.

# The migration must match the declared change

The declared type is checked against the upgrade, not accepted on trust, and the mismatch is printed in full:

<!-- cinch:reject contains="the migration must have type" -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    base :: Nat
    delta :: Nat

  bad :: StateV1 %1 -> StateV1
  bad s = s

  upgrade StateV1 to StateV2 where
    migrate = bad

  main :: Int
  main = 0
```

The message reads ``the migration must have type `(Main.StateV1 %1 -> Main.StateV2)`, but it is declared `(Main.StateV1 %1 -> Main.StateV1)` ``. A retyped field is an incompatible change and needs a migration at all; a purely additive change does not, because the old record is a prefix of the new one:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    value :: Nat
    extra :: Option Nat

  upgrade StateV1 to StateV2 where
    related = StateV1

  viewV1 :: StateV1 -> Nat
  viewV1 s = s.value

  main :: Int
  main = if viewV1 (StateV1 { value = 3 }) == 3 then 0 else 1
```

`related = StateV1` is a record view: it asserts that a `StateV1` is readable as a `StateV2` whose added field is missing. The interface evidence keeps that distinct from a `StateV2` whose `extra` holds `None`, and records the row with `migrated` false, `added` holding `extra :: Option Nat`, and `retyped` empty. Its honesty is a structural check, and the direction matters: a view cannot reinterpret bytes that a field change moved.

# The evidence an upgrade leaves

An upgrade is not just a check. It becomes a row in the module interface, so a consumer can see which fields the migration covers:

```text
$ cinch emit-ir Main.cinch -o Main.ll
$ python3 -c 'import json; print(json.load(open("build/Main.cinchif"))["upgrades"])'
[{'added': [{'name': 'base', 'type': 'Nat'}, {'name': 'delta', 'type': 'Nat'}],
  'migrated': True, 'new': 'StateV2', 'old': 'StateV1', 'retyped': []}]
```

`added` lists the fields the migration produces, `retyped` lists the incompatible changes it covers, and `migrated` says a checked function did the mapping rather than a view. A representation-level change, such as a new `#[repr]` on a record, changes the layout that is part of the interface contract, so dependents checked against the old identity are stale and must be replaced.

That is the whole of the migration story in this compiler. Retaining live frames, gating activation on a complete update set, and deoptimization exist in the Rust implementation and are exercised by tests, but the CLI exposes no service supervisor or update-package activation command.

<span class="support-tag support-tag--planned">Coming soon</span> **the user-facing update workflow.** The intended deployment shape, with complete update sets and activation, is described in [upgrades and temporal contracts](../reference/17-upgrades-and-temporal-contracts.md). Today you can check a migration, run it, and read its recorded evidence.

# Temporal contracts

A contract states a property over a definition's transitions. Four kinds are checked, and each one is verified where its claim is decidable and refused by name where it is not:

| Kind | Clause | Checked over |
| --- | --- | --- |
| `safety` | `always = (P)` | Every transition of the definition's mutable state, as an inductive invariant |
| `stability` | `stable = (P)` | The transitions that do not touch the fact's footprint |
| `liveness` | `eventually = (joined)`, `eventually = (cleanup)` | The finite structured-scope guarantees |
| `recurrence` | `recur = (P)` | Structural recursion, at every cycle |

A safety contract needs both its invariant and its establishment, and it must read borrowed state:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 3 in
      countdown m (n - 1)

  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
```

Every write in `countdown` stores `3`, which keeps `copyOut m < 5` true, and `assume` states the initial establishment the checker needs. The verified contract is a fact in the interface and a row in the manifest:

```text
{"assumptions": ["assume (copyOut m < 5)"], "contract": "loopInv", "definition": "Main.countdown",
 "formula": "always (copyOut m < 5)", "kind": "safety",
 "result": "verified (the invariant holds at every step: 2 transition(s) of the state, 1 discharged by the inductive step)"}
```

Write `9` at that site instead, and the message names the step and the state it leaves: ``the contract `loopInv` on `Main.countdown` is falsified by the step at 7:15: the write leaves the state `9 < 5` false, so `always (copyOut m < 5)` fails at this transition``. A write of a value the checker cannot pin down is refused rather than assumed.

# A liveness claim is finite, or it is refused

`eventually = (joined)` and `eventually = (cleanup)` are finite obligations of a structured scope: every handle spawned inside `withScope` is joined or cancelled before the scope exits. A liveness claim must also state the fairness assumption it relies on. An open-ended claim, such as "this loop eventually leaves the invariant true", needs a well-founded progress argument the checker does not derive, and it is refused with the replacement named:

<!-- cinch:reject contains="an unbounded eventual claim" -->
```cinch
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 3 in
      countdown m (n - 1)

  contract liveness reachesZero for countdown where
    eventually = (copyOut m < 5)
    assume = "scheduler-fair"

  main :: Int
  main = 0
```

The refusal ends by stating which claims do verify: ``The finite structured-concurrency obligations `eventually = (joined)` and `eventually = (cleanup)` verify within their scope``. The finite form runs and records its own result:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2

  contract liveness joinsRun for work where
    eventually = (joined)
    assume = "scheduler-fair"

  main :: Int
  main = 0
```

Its manifest row reads `"formula": "eventually (joined)"` with `"result": "verified from the P12 structured-scope guarantee: every spawned handle is joined before the scope exits, so the join completes within the scope's bound (concurrency specification section 2)"` and `"assumptions": ["assume \"scheduler-fair\""]`. The stated assumption is part of the evidence: a liveness claim is a promise about the runtime's scheduling, so the artifact says which promise it made.

Recurrence follows the same discipline. `recur = (P)` over structural recursion verifies at every cycle against the decreasing argument, and the literal-ladder form is refused with `has no structural depth bound` because a chain of equalities is not a structural recursion.

# What the compiler enforces

- `upgrade Old to New` accepts exactly a migration of type `Old %1 -> New`, or a `related = Old` view across a compatible, purely additive change.
- A retyped field is an incompatible change and cannot be crossed by a view.
- The upgrade's field mapping is recorded per module in `upgrades` in the interface, with `added`, `retyped`, and `migrated`.
- A contract names its kind, its definition, and its clauses; `safety` needs establishment and must read borrowed state, and `liveness` must state its fairness assumption.
- A claim the checker cannot decide is refused by name, never recorded as checked.
- Every verified contract and every stated assumption appears in the manifest's `temporal` block.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `migrate` of type `Old %1 -> Old` | ``the migration must have type `(Main.StateV1 %1 -> Main.StateV2)` `` |
| A view across a retyped field | ``incompatible field change `value` `` |
| `eventually = (copyOut m < 5)` | ``an unbounded eventual claim: it needs a well-founded progress argument and stated fairness assumptions`` |
| `eventually = (joined)` with no `assume` | ``states no assumption`` |
| `always` with no establishment | ``has no establishment for `always (copyOut m < 5)` `` |
| `always` over an immutable parameter | ``reads no borrowed state`` |
| `recur` over a literal ladder | ``has no structural depth bound`` |

# Exercises

**Exercise 1.** A migration adds an `extra` field set to `0`, and a function that started before the migration is still running. Why can that function be unsafe even though it never mentions `extra`?

<details><summary>Answer</summary>

The running function retained the old layout or an old continuation assumption, so it can resume over state the migration has already rewritten. Mentioning or not mentioning the new field is irrelevant: what matters is which code version is allowed to run against which state version. That relationship is the runtime's frame retention and activation discipline, not the type of the migration function. `migrate :: Old %1 -> New` proves the field mapping and consumes the old value once; it says nothing about code that is still executing, which is why the CLI can check and run a migration today but exposes no activation command.

</details>

**Exercise 2.** Why does `contract safety` reject a `contract stability`-style claim about a fact that no transition touches, and why does `stability` accept it?

<details><summary>Answer</summary>

`safety` checks an inductive invariant over the definition's own state transitions and requires establishment (`assume`) plus a formula that reads borrowed state; a claim about a fact its transitions never affect gives the checker nothing to establish and it reports `reads no borrowed state`. `stability` asks a different question: the fact survives exactly the transitions that do not touch its footprint, so a program that writes `b` and reads `a` keeps a fact about `a`, and a program that writes `a` invalidates it with `evidence invalidation over mutable state`. Choose the kind by the question you want answered, not by which one accepts the formula.

</details>

Next: [put it together in an order calculator](22-putting-it-together.md).
