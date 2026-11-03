# Second pass: projected conversations and tensor reverse mode

This round exercises declaration projection, library abstraction boundaries,
source-library expansion, and shape-indexed numerical programs. Everything below
runs through this worktree's actual Cinch checker and native backend.

## A choreography becomes participant APIs

`libraries/Checkout.cinch` describes the conversation once:

```text
choreography!{
  Checkout roles { Buyer; Shop; Bank; }
  prefix {
    Buyer -> Shop : Order;
    Shop -> Bank : Payment;
  }
  choice Bank {
    Approved {
      Bank -> Shop : Receipt;
      Shop -> Buyer : Confirmation;
    }
    Declined {
      Bank -> Shop : Reason;
      Shop -> Buyer : Rejection;
    }
  }
}
```

`syntax/choreography.syntax.json` projects this declaration into each role's
sequence of local states, send/receive operations, branch selection/offering,
and close operations. Buyer has one prefix step; Shop has two; Bank has one.
The branch suffixes have their own state and message types. Uninvolved roles
do not acquire an operation for another participant's message.

For example, a generated send operation has the shape:

```text
send-Checkout-Buyer-Order ::
  Thunk {} CheckoutBuyerP0 %1 -> Nat ->
  (Thunk {} CheckoutBuyerP1, Thunk {} CheckoutWireOrder)
```

The wire is a one-shot local packet. Sending constructs it; the receiving
participant consumes it and advances its own endpoint. `LocalWire.cinch`
implements this local transport using ordinary affine thunks. No network
service, host callback, or protocol-specific compiler operation is involved.

The chooser supplies branch notifications to the other roles. An `offer`
returns a sum whose constructors carry the correctly typed branch endpoint.
This makes handling the remote choice explicit in participant code.

`examples/Choreography.cinch` runs complete conversations for both runtime
decisions. It checks the delivered result and all participants' local step
counts. Its shop and bank prefix functions accept only their projected input
states and incoming message types.

The generator also emits the library's export surface. Endpoint, wire, and
notification representation constructors remain private; callers cannot
manufacture an advanced endpoint. Public offer constructors support ordinary
pattern matching after receiving a choice.

Negative programs check:

- Sending from the wrong local state.
- Supplying an endpoint from the other branch.
- Receiving a different branch's message type.
- Replaying an affine packet.
- Accessing a private endpoint constructor.

The grammar reader separately checks role membership and duplicate role/branch
names before emitting code.

The present choreography grammar has a prefix and a single terminal choice,
with arbitrary declared roles/messages and two or more branches. Message payloads
are `Nat`. It broadcasts the choice explicitly. General nested choices, joins,
payload schemas, runtime session identity, cancellation, and actual network
transport remain extensions. This is not a proof of network delivery or global
deadlock freedom. `%1` API arguments consume supplied tokens exactly once;
the underlying affine transport permits an explicit abort to drop local packets.

## Shape-checked reverse-mode tensor differentiation

The new `tensor-ad` library accepts an expression such as:

```text
weight-loss = tensor-ad!{
  at {weights} of sum (({inputs} @ x) * ({inputs} @ x))
}
```

`x` denotes the selected matrix variable. Host holes supply constant matrices.
`@` is matrix multiplication; `*` is elementwise multiplication. Addition,
transpose, parentheses and sum are also supported. The point is evaluated once.

Expansion produces ordinary calls into `TensorAD.cinch`. A differentiated value
has a primal matrix and a pullback:

```text
Diff r c xr xc
  primal   :: Matrix r c
  pullback :: Matrix r c -> Matrix xr xc
```

The output shape and original variable shape are distinct parameters. Matrix
multiplication requires its inner dimensions to agree, including inside the
pullback. Backpropagation uses the matrix chain rules:

```text
dA = seed @ transpose(B)
dB = transpose(A) @ seed
```

The upstream pullbacks receive those matrices; contributions to the same
variable are added. The implementation is reverse mode over a graph of ordinary
typed closures. It does not build an untyped Python tape or compute gradients
in the grammar engine.

`examples/TensorAD.cinch` checks a matrix squared loss of 5194, gradients for both
weights and inputs, and independent central finite differences for every weight
entry. It additionally exercises rectangular matrices, transpose/addition,
constant gradients, zero-sized inner dimensions and rejection of ragged input.
The arithmetic is exact `Int` polynomial arithmetic; floating-point numerical
contracts and nonlinear activation/loss operations are not implied.

`Tensor.cinch` exports `Matrix r c` with a private representation constructor.
Its general factory takes dependent dimensions and validates the physical row
lengths. Operations preserve that representation invariant. Negative programs
check incompatible multiplication shapes and attempts to bypass construction.

The initial implementation attempt used `Vec` directly. The isolated compiler
snapshot rejects matching `Nil`/`Cons` at an unknown rigid index, so a general
vector map cannot be written that way yet. The current backend uses checked
abstract matrices backed by lists. The public shape constraints remain checked
by the compiler. Representation preservation and the derivative rules are not
yet accompanied by general kernel-checked proofs; the native tests provide
specific executable evidence.

## What the common extension interface gained

The generic template language now has finite sequence iteration, nested capture
paths, filtering, local sequence bindings, cardinalities, conditional templates,
and trimming. These operations work on captured syntax. The choreography is
authored entirely in the descriptor using them; there is no choreography
implementation in `engine.py`.

Libraries can themselves import syntax. The CLI and LSP expand the same
explicit dependency closure in dependency order. Unused libraries do not change
the project. Library source maps accompany generated code, and go-to-definition
on a generated API returns to the original library declaration. Opening that
library in the editor reuses its existing analysis document.

Tree-sitter verification now includes the original library sources as well as
the example programs, keeping the new declaration grammar and numerical syntax
on the same path as the HTML experiment.

The aggregate verifier passes 38 tests, covering twelve native programs,
eighteen intended compiler refusals, fourteen syntax descriptors, the actual
native language server and composed highlighting. Source hashes in
`verification.json` bind that evidence to the checked code.

The next unresolved interface questions are scoped syntax objects, access to a
checked elaboration context, and replay/interface identity for syntax libraries.
The tensor example demonstrates how much can already live in typed runtime
libraries; it does not implement a general post-elaboration transformer API.
