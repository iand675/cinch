# Scoped effect policies

Status: draft. This extends [effect boundaries and interposition](effect-boundaries-and-interposition.md).
[Database transactions](transaction-effect-policies.md) are one application.

A handler says what an operation means. A **scoped effect policy** says
which operations may leave a body, and binds each of them to a particular
handler. Use the same mechanism for a database attempt, a deterministic
test, or a build step that can read declared inputs. Each scope supplies
its own lifecycle and vocabulary.

## The contract

`with-policy policy (\scope -> body)` is the proposed generic entry form.
Its checked contract is schematically:

```text
policy: for fresh s, body input Scope s, allowed operations Allowed s,
        answer A -> B, outward effects Host
body:   for fresh s, Scope s -> A with effects E s
check:  E s ⊆ Allowed s, and no scope-bound authority escapes
whole:  B with effects Host
```

`Host` is the outward row, including setup, interpreter clauses and exit
work; it is empty for a pure policy. It is charged to the caller of the
scope. Construction of the policy value is evaluated in the surrounding
context and has its own ordinary effects. The fresh scope may supply a
domain-specific token instead of a generic `Scope s`. The runner controls
normal exit, abort and cleanup, and can return a different answer type.
Observations without operations, such as `Diverge`, cannot be discharged
by this mechanism. A later policy may explicitly admit such observations
while retaining them in the outward row.

- **Check the body separately.** Check its residual row after local
  handling, including calls, callbacks and cleanup. A local pure fake is
  fine; a local handler that performs `IO r` leaves IO in the body row.
  An open row requires a checked upper bound. A captured capability,
  erased proof, or unsafe artifact opt-in cannot change this judgment.
- **Bind every admitted operation.** The policy supplies the handler for
  each admitted nominal label with its exact arguments. It never falls
  through to an outer handler for the same label. The body can interpose
  a local handler and forward to the policy. A policy fragment owns a
  complete label; to expose just one operation, give it a narrow effect
  of its own. Raw IO and unchecked host calls cannot be body permissions.
- **Separate the interpreter's authority.** A host adapter is a registered
  implementation of a named protocol with checked outward effects and an
  explicit operational contract. Its host authority is never handed to
  the body. The caller chooses an adapter and may tighten its limits; it
  cannot certify an arbitrary `IO` closure by putting it in a policy.
- **Close over the lifetime.** No token, resumption, or latent computation
  carrying scope authority may escape in a result, error, existential, or
  scheduled callback. Interpreters only retain scope-independent data past
  exit. An inner policy's outward effects are part of the outer body's
  row, so installing an IO-backed policy inside a restricted scope cannot
  grant itself an exception.

## Composition and lowering

`compose-policy` combines interpreter fragments for disjoint labels, keeps
each fragment's limits, and unions their outward rows. Duplicate claims
are rejected; deliberate interception uses a local handler with an
explicit forward. Composition does not merge two lifecycle protocols or
search for handlers in the ambient environment. One runner owns the
lifecycle, fixes the answer type, and wires each fragment to the lifecycle
events it needs. For example, a transaction runner installs `DbTx tx` and
`Throw err`, and lets its telemetry fragment observe the final commit,
rollback, or unknown outcome. The telemetry fragment cannot decide it.

Start with checked policy constructors, an explicit operation slot for
each admission, and existing one-shot handlers and scope checks. The
independent Core checker rechecks the row bound and that all admitted
slots are bound to handlers with the declared outward row. The module
interface carries the policy contract so aliases and separate compilation
cannot erase it. Use a pure policy to exercise the mechanism before adding
registered host adapters.

First acceptance cases: a pure test policy replaces a lookup with fixed
data; a scope rejects an imported callback with an open row unless its
bound is proven; an inner IO-backed policy cannot widen its enclosing
scope; and an operation never reaches an outer handler when the scoped
policy omits or replaces it. The
[transaction example](transaction-effect-policies.md#example-logging-plus-one-reservation-check)
adds an explicit budgeted host adapter and exercises the same boundary
with real outward effects.
