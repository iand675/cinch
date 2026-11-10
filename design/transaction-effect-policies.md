# Transaction effect policies

Status: draft. This is a database driver built on the general
[scoped effect policy](scoped-effect-policies.md) mechanism. The driver
owns the transaction lifecycle; it uses the same allowlist and handler
composition rules as any other scoped policy.

## Two rows, two authorities

A transaction runner checks the effects of its **body** separately from
the effects of the **driver**. The body gets a fresh transaction identity
`tx`, an opaque `TxScope tx`, and a closed allowlist of operation labels.
The driver holds `Io r` and owns the database connection. A SQL query can
therefore use the network through the driver without giving the body a
socket, a connection it can use directly, or a way to run arbitrary IO.

Schematic contract (the fresh scope can elaborate like `withScope`):

```text
run-transaction-with db policy body
  body:  for fresh tx, TxScope tx -> a ! bodyRow
  check: bodyRow ⊆ {DbTx tx, Throw err} ∪ policy.allowed(tx)
  check: neither a nor err can carry or capture tx
  result: Result (TxError err) a ! (dbHostEffects ∪ policy.hostEffects)
```

For a synchronous database driver, `dbHostEffects` includes `IO r`,
`Blocking` and `Diverge`. The default policy adds no operations. The
observability policy can add the existing `Log r` (which includes `metric`)
and `Trace r` labels. An open body row is accepted only with checked evidence
that every possible member is allowed. Matching uses nominal labels and
their arguments; banning a single `IO r` would miss another runtime's IO,
raw foreign calls, or the ability to spawn work that outlives the attempt.

The checker verifies the body after local handlers discharge their effects.
A pure fake or an interceptor that forwards `DbTx tx` is fine; a local
handler that translates `Log r` to `IO r` leaves IO in the row and is
rejected. The same check covers captured callbacks, imported helpers,
cleanup and explicit capability arguments. Neither capturing `Io r`
from `main` nor opting the executable into unsafe foreign calls grants an
exception inside the body. Transaction-bound values and closures cannot
escape, including through an existential or an exporter queue.

`run-transaction-with` installs the fixed `DbTx tx` and `Throw err`
interpreters and composes them with the supplied policy fragments. The
driver owns the connection and is the only code that decides whether to
commit. A fragment cannot override the transaction's fixed handlers or
fall through to an outer handler.

## Example: logging plus one reservation check

Proposed Cinch surface. `ReservationClient r` is an opaque, configured
client for the reservation service, not a general HTTP client. The
registered adapter owns it and has no transaction connection or token.

```cinch
effect ReservationCheck where
  check-reservation :: ReservationId
    ->{ReservationCheck} Result ProbeError ReservationStatus

create-order :: forall r tx. TxScope tx -> Draft
  ->{DbTx tx, Log r, ReservationCheck, Throw OrderError} OrderId
create-order tx draft = do
  log info "checking reservation" no-fields
  status <- check-reservation (reservation-id draft)
  case status of
    Err err -> throw (ReservationUnavailable err)
    Ok Inactive -> throw ReservationInactive
    Ok Active -> do
      order <- insert-order tx draft
      log info "order staged" no-fields
      order

place-order :: forall r. using (io :: Io r) =>
  Db r -> LogSink r -> ReservationClient r -> Draft
    ->{IO r, Blocking, Diverge} Result (TxError OrderError) OrderId
place-order db logs reservations draft =
  run-transaction-with db
    (compose-policy
      (buffered-log-policy logs AttemptOutcome)
      (reservation-check-policy reservations
        (budget 1 (duration-from-millis 150))))
    (\tx -> create-order tx draft)
```

The `ReservationCheck` adapter may start at most one request in this
attempt. A second call is refused with a typed budget error before
doing IO. The adapter cannot silently retry or follow a redirect to make
another request. The deadline produces a typed timeout if no answer arrives;
the adapter's cancellation contract says what happens to an in-flight
request. This policy runs a single attempt with no automatic retry.
The check is a point-in-time observation and does not commit atomically
with the database write.

Logs are handed off after the attempt with its outcome attached. Calling
`ioWrite`, a general HTTP client, or an imported helper that sends email
from `create-order` fails the body row check, even if the helper captures
`Io r`. The outer `place-order` row accounts for the database connection,
the reservation check and the log adapter.

## Observability: say when events leave the attempt

Use the same `log`, `metric` and trace operations with different policies:

| Policy | What the body does | When signals are visible |
| --- | --- | --- |
| Buffered until outcome (recommended) | Append owned event data to a bounded attempt buffer; drop on overflow. | Hand off for export after the outcome is known or reported unknown. The policy can keep all attempts with outcome tags, or emit only after confirmed commit. |
| Live, best effort | Offer bounded event data to a queue without waiting for the exporter. Drop on overflow. | May export before commit and survive rollback. Tag records with attempt identity; report the eventual outcome separately. |
| Durable business or audit event | Write an event as transaction data, such as an outbox row, using `DbTx tx`. | The record commits with the database change. A separate worker delivers it under its own retry and deduplication contract. |

An observation returns no delivery acknowledgement to the body. The runner
does not wait for delivery, even after commit. Slow or failed exports cannot
block the attempt or change its commit decision.
Bound individual records and total queued data, including buffered spans.
Only owned, region-free event data reaches an exporter. Finish spans on
failure and rollback as well as success. Separate attempt counters from
committed business counters; a retry may legitimately produce several
attempt events. Buffering is not durable. A live exporter is an explicit
choice to expose some data before commit. The
[OpenTelemetry tracing SDK](https://opentelemetry.io/docs/specs/otel/trace/sdk/#span-processor)
provides prior art for nonblocking span hooks and bounded export queues.

## Cautious IO: admit a named operation

When the body truly needs an external answer before it can continue, admit
a narrow operation under an explicit policy, like `check-reservation`.
The registration pins which backend the adapter can reach and which
requests it can issue. The adapter cannot access the transaction's
connection or token. Because a query may already be holding
database locks, require a per-attempt call budget, a deadline, typed failure
and cancellation behavior, and an explicit replay contract. A timeout
contract must say whether the external request can still complete after
the caller stops waiting. Read-only is not enough to
infer replay safety: repeated reads can produce different answers or have
externally visible costs. Keep unrestricted HTTP, raw IO and foreign calls
outside the body. This policy is for exceptional operations; first prefer
fetching the input before the transaction or deferring the action until
after a confirmed commit.

Treat an admitted external operation's side effects as outside database
rollback. Make it single-attempt by default. Automatic retries require an
explicit replay contract for **every** admitted operation and a body whose
captures may be reused; a one-shot closure with linear captures cannot
be run twice. An operation that depends on idempotency needs a key stable
across attempts and a backend contract that honors it. Retries rerun the
whole body with a fresh scope, never after an unknown commit outcome. See
[PostgreSQL's serialization retry rules](https://www.postgresql.org/docs/current/mvcc-serialization-failure-handling.html)
for why retrying just the failed statement is insufficient.

## Database and exit contracts

`DbTx tx` uses one leased connection and exposes typed, registered statements
with bound parameters. The initial policy offers no raw SQL, nested runner,
or manual `COMMIT`; savepoints can be scoped operations later. The statement
contract includes reachable server code and database permissions. An IO-free
client body alone says nothing about server-side effects: PostgreSQL's
[`COPY … PROGRAM`](https://www.postgresql.org/docs/current/sql-copy.html)
can execute a program, and [`nextval`](https://www.postgresql.org/docs/current/functions-sequence.html)
is not rolled back. The adapter must account for those behaviors rather than
claim every registered statement is rollback or replay safe.

Normal return requests commit; `Throw err` that reaches the runner requests
rollback. A `Throw` caught inside the body, or an `Err` returned as ordinary
data, does not on its own request rollback. A driver failure can still make
the attempt uncommittable. Roll back or discard the connection on failure
and cancellation; if commit was sent and its result is unknown, report
`CommitUnknown`, not success or confirmed rollback. Only confirmed commit
permits a commit-only export or caller-side follow-up.

First test this with an in-memory driver and the default and buffered
telemetry policies. Accept database operations and permitted observations
through imported helpers. Refuse captured IO callbacks, a custom IO-backed
log handler, policy fallthrough, and a transaction handle hidden in a
returned closure. Exercise rollback, unknown commit, buffer overflow, and
retry without double-counting committed metrics. For live or cautious IO,
also test a stalled sink, cancellation, and rejection of an unregistered
adapter or a retry policy with a single-attempt operation.
