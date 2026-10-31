# Effects, handlers, capabilities, and sequencing

An effect row describes possible actions. A capability supplies access to a runtime identity. A handler interprets effect operations. A sequencing provider chooses the meaning of `do`. These mechanisms cooperate but are not interchangeable.

# Rows and honesty

Pure arrows use `->`; effectful arrows use `->{Label, Other r}`. Open rows admit a tail variable, allowing callbacks' effects to propagate. The checker unifies row structure and rejects a body whose performed effects do not match its declared row. `Diverge` represents admitted partiality; wrapping a looping computation in a handler does not prove termination.

Effects can be indexed by runtime/region identity, such as `IO r`, `Async r`, `Cancel r`, and memo-region `Eval r`. Confusing two capabilities with different identities is not permitted merely because their short type names agree.

# Runtime capabilities

A helper states `forall r. using (io :: Io r) => A ->{IO r} B`. `with (io = newIO ()) in body` supplies a capability lexically; `call @{io = capability} argument` supplies it explicitly. Unknown supply names and wrong supply types are errors. A capability value alone does not remove the required effect from the call.

Runtime I/O includes `ioWrite`, `ioRead`, `ioSleep`, and cancellation polling through the supported runtime vocabulary. Files, clocks, subprocesses, sockets, logging, and tracing live in their `Effect.*` homes. Observe each function's actual result/error type and effect row in the [library reference](../library/00-index.md).

# User effects and handlers

An `effect` declares operation signatures. `handle expression with (handler Effect where ...)` gives return and operation clauses. An operation clause can call its named `resume` continuation once or return without resuming, subject to captured-resource usage. Resuming twice is rejected. Clause-introduced effects remain visible.

<span class="support-tag support-tag--planned">Coming soon</span> **cross-call operation dispatch.** Current handlers interpret operations in the handled expression itself and refuse operations performed through a separately called computation. The intended handler calculus crosses call boundaries. The Rust tests explicitly check this refusal, so a general handler example must not claim that behaviour already works.

Handler-vtable specialization is a separate planned optimizer extension. Runtime semantic support and specializing a concrete handler dictionary are distinct questions.

# Do providers

The direct provider is the default and sequences direct computations. `ResultSequencing e` unwraps `Ok` and short-circuits `Err`; `OptionSequencing` unwraps `Some` and short-circuits `None`. A lexical provider context selects the container for a definition. `<-` binds a step's result; `let` names a local binding; the last expression returns the block's result through the selected vocabulary.

This mechanism has one concrete container per definition; it is not implicit container-polymorphic monad notation. The separate `Category.Functor`, `Applicative`, and `Monad` classes supply polymorphic abstractions. A module parameter can select a sequencing dictionary deliberately.

# Computations stored as values

<span class="support-tag support-tag--planned">Coming soon</span> **`Comp {e} a` and resource-safe effectful pull streams.** The selected design packages an effectful unit closure in an ordinary transparent library type, with `suspend :: (Unit ->{e} a) -> Comp {e} a` and `run :: Comp {e} a ->{e} a`. The law is `run (suspend k) = k ()`; storing a computation does not execute or implicitly sequence it. Its closure's ownership and row govern running/sharing it.

`Effect.Comp` is not a shipped module today. Current programs can use explicitly typed effectful closures and the existing thunk/codata APIs; they should not import the planned module or assume a container-polymorphic effectful-stream library is already installed. The [computation-type decision](../design/comp-computation-type.md) fixes that direction while leaving the implementing streaming slice responsible for its exact resource-safe stream API.

# Entry points and boundaries

An effectful main uses `Unit ->{effects} Int`. A pure `main :: Int` cannot hide an unhandled effect. External foreign operations must declare effects honestly, including blocking and failure where appropriate. Staging uses its own recorded capabilities instead of inheriting runtime ambient I/O.

Implementation: `check_terms.rs`, `check_decls.rs`, `types.rs`, `Effect/Sequencing.cinch`; suites: `effects.rs`, `concurrency.rs`, `observability.rs`, `interface_region_effects.rs`. Tutorial: [effects](../learn/10-effects-and-io.md), [handlers](../learn/11-handlers-and-sequencing.md).
