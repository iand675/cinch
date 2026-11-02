# Checked bootstrap handlers

This implements the handler design in
[Effects, capabilities and sequencing](effects-capabilities-sequencing.md)
and decision D23 in the runtime capabilities specification. It does not
change the language's handler syntax or the runtime ABI. Self-hosting still
requires executable lowering and the remaining usage and totality checks.

The elaborator retains checked effect declarations in its type context.
Before checking bodies, the independent typing context verifies each
declaration's nominal kind, complete operation list, declaration-order
slots, and actual global operation schemes. An operation with the right
display name but a different role or scheme cannot establish this registry.

Core retains a handler value, its return closure, and its operation closures.
Its private type records the nominal label, input, output, and clause effect
row. Operation entries retain their slot, universal type openings, actual
clause row and closure. A handle node retains the computation, handler
expression, discharged label and residual row. Recursive type mapping,
closure, reference, proof and dictionary checks traverse these forms.

The independent checker derives each operation closure's expected telescope
from the registered declaration. Effect parameters specialize to the actual
label arguments; operation-local quantifiers open to fresh rigid types.
Concrete substitutions, missing or duplicate openings, wrong slots, omitted
clauses, and forged closure types are refused. The continuation consumes an
operation result and returns the handler's answer. Return and operation
clauses must produce that answer type.

Effect discharge removes explicit occurrences of the handled label from the
computation's row, then adds the clause and handler-expression effects.
Effects re-performed by a clause remain visible. Flexible operation label
arguments can match the handler's actual arguments during elaboration;
distinct rigid regions remain distinct. The independent checker recomputes
the residual row from the closed, solved computation rather than trusting
the cached residual. Opaque row tails are retained, and unsupported dispatch
through an opaque row receives a refusal.

D23 permits dispatch only from operations in the handled computation's own
expression. The elaborator and independent checker both reject a handled
effect arriving through an ordinary function call. Creating a closure or
handler does not perform its latent effects.

Continuation calls are counted along executable paths. Alternative branches
take the maximum; sequential computations add. A failed guard can continue
to later alternatives, so calls in its condition add to calls on that later
path. Repeated calls, storing a continuation as a value and capturing it in
another closure are refused by this initial usage check. Safe continuation
aliases and richer escape patterns require fuller usage evidence. Promotion
to exactly-once use for linear captures and complete ownership checking are
still required; this check is not a replacement for that kernel.

The runtime emitter still needs typed closure/vtable construction and CPS
operation dispatch. Operation class evidence, additional guarded dependent
operation forms, interface-only effect registries and the complete usage
and totality rules remain work. Passing source and Core checks establishes
neither an executable handler emitted by Cinch nor a stage2 compiler.
