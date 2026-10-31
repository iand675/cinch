# Explicit laziness, memoization, and codata

Strictness is the default. Deferred computations, memo cells, and guarded observations have separate type/effect/usage rules.

# Thunks

`delay (\() -> expression)` constructs a suspended computation. `force thunk` executes it. A `Thunk {e} a` records the delayed effects and result type. Ordinary unannotated thunks are affine by default: forcing consumes them, and a second force is rejected. Exactly-once or duplicable usage must be stated and checked through the applicable multiplicity rules.

A `%many` thunk recomputes on repeated force; it does not imply call-by-need. Captured resources retain their restrictions. A thunk with an I/O row needs that effect admitted when forced. A pure function cannot hide an effect by delaying and forcing it internally.

# Memo groups

`~name = expression` is admitted in `let`, recursive memo contexts, and `where` groups with variable binders. The group owns memo storage; `where` storage is per call. Initialization is on first demand, subsequent demands reuse the result, and scope end controls reclamation.

`demand` obtains a borrow of the cached result. `copyOut` requires structural copy evidence to obtain an owned value. Returning a memo borrow beyond its region, caching an arbitrary noncopyable generic value, or duplicating a restricted capture is rejected.

`Eval r` tracks evaluation in a memo region. Row lacks constraints prevent reentrant demand. The runtime also detects a black hole defensively; that diagnostic is not an ordinary user-level failure value. A memo binding does not make a cyclic eager definition productive.

**Open design question — irrefutable argument patterns.** `~pattern` in a function argument is not supported. The laziness specification leaves this as an open decision; it is not a promised addition. Keep it distinct from the implemented memo-binding form.

# Codata and Later

`codata Stream a where head :: a; tail :: Later (Stream a)` declares observations. A `corec` definition must construct the observable layer and guard recursive occurrences behind the appropriate delayed `Later` observation. `Later` has a productivity role distinct from an ordinary thunk.

The current checker enforces direct codata construction at each step and names unguarded recursive calls. Observing the head terminates under the admitted totality discipline; forcing the tail advances to the next layer. A helper that hides the construction can be refused even if its author believes it productive.

# Producers and bisimulation

Unrestricted filtering can fail to find a next element, so it loses a productive-stream guarantee and belongs in a partial `Producer` vocabulary unless bounded-gap evidence is supplied. An external `WasiStream`/`WasiFuture` has a boundary observation contract, not productive codata semantics.

Bisimulation reasons about corresponding observations and guarded tails. It does not reduce an infinite object completely. General proof automation beyond the supported guarded fragment is planned and must remain labelled accordingly.

See [the explicit laziness specification](../specifications/lazy-computation-and-reclamation-specification.md), [codata design](../design/codata-guardedness.md), and the runnable [stream tutorial](../learn/15-streams-and-producers.md).

Implementation: `check_terms.rs`, `check_decls.rs`, `codegen.rs`, runtime memo support; suites: `codata.rs`, `ownership.rs`, `integration.rs`.
