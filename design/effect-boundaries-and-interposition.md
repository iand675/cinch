# Effect boundaries and interposition

Status: draft for review. Examples show proposed syntax and behavior, not
current compiler support. This proposal extends the [runtime effects
specification](../specifications/runtime-capabilities-effects-and-sequencing.md)
and revisits the executable entry rule (D65) and the handler call boundary (D23).

## The boundary

The compiler gives the selected executable `main` its host `Io r` capability
when `main` explicitly declares `IO r`. Ordinary code may *use* that capability
through a checked parameter. Creating host authority, bypassing it at a raw
host call, or concealing an effect is a trust boundary. User effects instead
have scoped handlers: the caller can replace, translate, or forward an
operation, even when it happens several calls down.

Today `newIO` is a pure builtin available to ordinary code, and the generated
entry adapter just applies the source function to `()`. Some raw host bindings
also declare `IO r` without taking `Io r`; requiring the capability on public
wrappers alone leaves those bindings as a second way in. Separately, the
[current handler implementation](bootstrap-handlers.md) rejects effects that
reach a handler through a call. Fixing either half in isolation would still
leave libraries with a reason to use raw IO.

## IO belongs to the host entry

Keep `main :: Int` and the existing `Unit -> … Int` entry form. An explicitly
declared `IO r` on the selected entry's outer arrow activates the implicit
provider. Check the body with a fresh rigid runtime identity and a lexical
`io :: Io r`; lower the entry to a private worker taking that evidence. The
compiler-generated target adapter creates the runtime, passes its evidence
and `()`, and tears it down according to the runtime protocol. The worker's
row still contains `IO r`; the adapter accounts for it as host IO, including
in the executable's effect contract. There is no source-level pure rewrite.

```cinch
greet :: forall r. using (io :: Io r) => Bytes ->{IO r} Unit
greet bytes = ioWrite bytes

main :: forall r. Unit ->{IO r} Int
main u = do
  greet (string-to-bytes "hello\n")
  0
```

The selected entry is the unique `main` chosen for the executable, not every
binding with that name. In the first slice, a definition using the implicit
provider is entry-only: it cannot be exported or referenced as an ordinary
value, including recursively. Put reusable or recursive code in a helper
with an explicit `using` requirement. Check the same entry contract in
check-only mode when a root is selected. A library, a foreign export, or a
Wasm component export does not acquire an implicit provider. An embedding
host can supply its own adapter under a separate contract.

The final entry row must be closed. For the initial host adapter there is at
most one host runtime identity. The existing structured task operations can
share it (`Async r`, `Cancel r`); they retain their rows and their structured
scope rules. If an existing task entry needs `Io r` but otherwise does no IO,
it explicitly opts in with `IO r` when migrating off `newIO`. Passive
observations such as `Diverge` stay in the entry contract. The adapter
rejects an unhandled user operation, an unsupported host effect, and a
second runtime identity for host operations. Neither an unindexed foreign
`IO` annotation nor a user-defined label spelled `IO` grants authority.

### What requires trust

- Remove the ordinary `newIO` and `Runtime.newIO` constructors. Existing
  `using`, `with`, and `@{…}` only pass evidence already in scope. They do
  not construct a host capability or remove `IO r`. Apply the same region
  and escape rules to the implicit provider as to an explicit one: closures
  and handles cannot outlive the runtime that issued their evidence.
- Registered host primitives require the matching `Io r` at the checked
  boundary, even if the machine ABI erases the token. Match registrations by
  checked operation identity and target, with an explicit effect contract.
  The registry is controlled by the toolchain or an explicitly trusted host
  package; a foreign symbol name or row annotation does not register a call.
  Audit existing builtins, `prim` bodies and base-library foreign bindings
  as part of the entry change. The checked source/Core and interface paths
  must enforce the same rule when the primitive is imported or aliased.
- An arbitrary host call or inline primitive that bypasses the checked
  registry needs an explicit unsafe foreign/primitive boundary. A boundary
  declaration authorizes the call under a stated contract; it does not
  authorize a safe or pure signature that hides what the call does.
  Preserve the foreign identity and effects such as `Blocking` or `Global`
  through wrappers and interfaces. An unchecked host call must remain
  distinguishable from a registered one at the executable boundary and
  require a separate artifact opt-in; declaring `IO r` on `main` is not that
  opt-in. The first slice has no general `unsafePerformIO` or public secondary
  runtime constructor. Old objects that bypass the new check must be rebuilt
  or refused. Compile-time evaluation does not inherit the executable's
  authority.

Raw `IO r` has no user-defined discharge or interception. If application
code needs mockable filesystem, clock or console behavior, it can depend on
a narrower operation with a real interpreter that performs `IO r`, and a
test interpreter that does not. This works without substituting a fake
`Io r` or granting test code host authority.

Some scopes need to bind a set of handlers and limit the body's row at the
same boundary. [Scoped effect policies](scoped-effect-policies.md) define
that mechanism; [transactions](transaction-effect-policies.md) show how it
admits database operations, telemetry, and a named host operation.

## Handlers follow the call

`handle body with handler E …` binds a handler for a fully applied nominal
effect label `L`. Execution of `body` uses that binding through ordinary
calls, separately compiled calls, recursion and higher-order calls. A
closure uses the handler at **invocation** for its latent operation row;
building or passing it performs nothing. This is separate from `using`
capabilities, which a closure may capture lexically and which keep their
existing region rules. No process-global or thread-local lookup is part
of the semantics.

- The innermost matching handler sees the operation. Match nominal identity
  and all label arguments, not the spelling of the effect. `Throw ParseError`
  and `Throw ValidationError` are distinct. Implement this with typed
  evidence, without comparing erased types at runtime.
- A handler supplies a return clause and a clause for each operation. The
  clauses run with the handler environment outside the installation.
  Handlers between the operation and that boundary are suspended with the
  continuation. To intercept a `Console.write`, a clause can record an audit
  event, call `Console.write` to forward to the next provider, then resume
  with `()`. Forwarding contributes `Console` to the clause's residual row;
  if there is no outer handler, it is still unhandled. A test handler can
  instead resume with `()` without forwarding. A real console handler can
  call `ioWrite` and resume, translating `Console` to `IO r`.
- Resumptions are deep and one-shot: resuming reinstalls the handler for
  the rest of the computation. Normal completion runs the return clause;
  a clause that exits without resuming supplies the answer itself. In the
  first slice, resumptions are scoped to the clause and cannot be stored,
  copied, or transferred to another task. A task starts its own user-handler
  scope. A child handles its own operations or reports an explicit outcome
  via the task protocol; a child cannot jump to the parent's handler.
- For a computation returning `A` with `{L | e}`, the handler can return a
  different answer `B`. The residual row is `e` union the effects of
  evaluating the handler expression and running its clauses. A resumption
  takes the operation's result to `B` and includes the effects of the
  remainder and the clauses it can reach. It never asserts that the
  remainder is pure. Row subtraction must prove it removed exactly `L`;
  it may split a row using checked evidence, but cannot erase an opaque
  tail or confuse distinct region or type arguments. Reperforming `L`
  in a clause can put it back in the residual row.

For example, with a real console handler outside an audit handler, a call
to `render-report` performs `Console.write` through the inner handler even
if the write happens in an imported helper. The inner handler records the
write and forwards it; the outer handler performs IO. Replacing the outer
handler with an in-memory sink leaves the audit in place and needs no IO.

The handler model applies to operational effects. Existing runtime-specific
operations such as scheduling or cancellation need an operation interface
and an interpreter that preserves scope, lifetime and cancellation contracts
before they can use the same mechanism. `Diverge`, `Blocking`, and other
observations without operations are not interception points. They require
proof or an explicit runtime contract to change how they are accounted for.
`do` sequencing remains independent.

## `Throw`: one operation, several interpretations

Introduce an abortive operation mode and allow ordinary type parameters on
effect labels. A `Throw err` operation carries an error and never produces
an operation result. In the sketch below `a` is a fresh operation-local type
variable that cannot appear in the operation's arguments or label: `throw`
can appear in a position expecting any `a`, but the clause has no resumption.
Abortiveness is checked declaration metadata, not inferred from a function
called `throw` or a clause that happens to ignore `k`.

```cinch
effect Throw (err :: Type) where
  abortive throw :: forall a. err ->{Throw err} a

runThrow :: forall err a e.
            (Unit ->{Throw err | e} a) ->{|e} Result err a
runThrow work = handle work () with (handler Throw err where
  return value -> Ok value
  throw err -> Err err)

parsePort :: Bytes ->{Throw ParseError} Port
-- implementation calls throw on an invalid port

tryPort :: Bytes -> Result ParseError Port
tryPort bytes = runThrow (\u -> parsePort bytes)

portOrDefault :: Bytes -> Port
portOrDefault bytes = handle parsePort bytes with (handler Throw ParseError where
  return port -> port
  throw err -> defaultPort)
```

The signature of `runThrow` uses a row split: `e` excludes `Throw err` and
can include other effects. `tryPort` materializes a `Result` at its boundary.
`portOrDefault` selects a fallback without materializing a `Result`. Another
handler can map `ParseError` to `ConfigError` and rethrow; its residual row contains
`Throw ConfigError`. None of these choices changes `parsePort` or makes
`Throw` the same thing as binding a `Result` with a sequencing provider.

An abort skips the rest of the handled computation, including its normal
return clause, and runs the cleanup required by the scopes it exits before
the abortive clause runs. It does not roll back already performed effects.
If a resumable clause abandons its continuation, it must dispose of it
through the same cleanup protocol before returning or transferring control
out of the clause. Require a checked cleanup path for every linear obligation
between the exit point and its destination; otherwise refuse the program.
Check the exceptional edges of calls too, including opaque rows unless
constrained to exclude such exits. A `%1` value consumed only after a
potentially aborting call is not thereby consumed on the abort path. Keep
cleanup's own effects in the row. Initially, cleanup may not itself initiate
an unhandled control transfer. Reject exits across scopes whose cleanup the
compiler cannot verify, including a task scope if it cannot yet cancel and
join its children correctly.

### Lowering and the fast path

Use checked handler evidence at calls as the first lowering model. A
callee with latent user operations receives the relevant evidence, with an
explicit calling convention for answer transfer and row-polymorphic
callbacks. The effect interface needs the canonical label, parameter kinds,
stable operation slots and modes; the compiler must agree on the lowered
calling convention across objects or refuse the mix. Core checking must
verify handler installation and residual rows rather than trusting an
annotation. [Generalized evidence passing](https://www.microsoft.com/en-us/research/wp-content/uploads/2021/08/genev-icfp21.pdf)
is prior art for this direction; it does not settle Cinch's cleanup rules.

For an abortive operation, lower a statically known same-function exit to a
branch through the required cleanup. Across calls, start with an internal
tagged outcome or explicit answer passing, with no resumable continuation
captured. That internal carrier is distinct from a source `Result`; build a
source `Err` only where a handler asks for it. Specialize the carrier away
when the target and cleanup path are known. Neither the general path nor
the fast path can skip a closer matching handler, skip cleanup, unwind
across an unsupported foreign ABI, or introduce a trap for an ordinary
`Throw`. Use a portable lowering on Wasm before considering target-specific
unwinding. Benchmark the fast path before making speed claims.

## First implementation slices and acceptance gates

1. **Root authority.** Implement entry selection, implicit evidence, the
   checked host boundary and migration away from `newIO` together. Test
   a pure entry, a registered IO helper reached from `main`, a task entry,
   and a root whose residual row includes an unhandled user effect. Refuse
   direct and indirect constructor access, IO in an entry without `IO r`,
   authority from a lookalike `IO` label or an imported module named `Main`,
   a raw host call mislabeled as pure, and an aliased
   host call with the wrong identity. Include the source compiler and its
   independent Core checker; accepting syntax alone is insufficient.
2. **Cross-call handlers.** Start with an existing resumable effect and
   exercise a helper imported through its compiled interface, a callback
   invoked under a different handler than where it was constructed, nested
   forwarding, and deep resumption. Check that a nonmatching label reaches
   the outer handler, forwarding leaves the correct residual, and an open
   row cannot be used to launder an effect. Keep the current refusal on
   backends until their call and handler conventions agree.
3. **Abortive `Throw`.** Add the operation mode, type-indexed label and
   ownership checks. Run the same caller under a result, fallback, and
   error translation handler. Assert the order of cleanup and observations,
   refusal of abandoned linear obligations, and that no code after `throw`
   executes. Compare success and failure at several call depths against
   hand-written `Result` propagation; inspect allocation and generated IR
   to verify that the specialized path captures no continuation and that
   fallback does not allocate a source `Result`.

Open choices for review: the spelling of unsafe declarations and the
artifact opt-in, the initial cross-object carrier ABI, and how much of
abort cleanup to support before adding the first library `Throw` handler.
