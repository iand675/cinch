# Runtime capabilities, effects and sequencing

Source: <https://chatgpt.com/space/page_f7f7e6466b348191803f2f900098b667>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2 design decision: ordinary function definitions, effect-polymorphic suspension, lexical runtime evidence, and unqualified typeclass/module-driven `do`. The grammar and examples use a strict Haskell-like surface.

# Three independent choices

| Choice | Representation | What it controls |
| --- | --- | --- |
| What a computation may do | Effect row | Suspension, I/O, cancellation, divergence and other observations |
| Which implementation executes it | Lexical capability/module arguments and handlers | Scheduler, I/O driver, allocation and host integration |
| How intermediate results compose | Sequencing module selected for `do` | Direct binding, typed short-circuiting or an explicitly modeled computation abstraction |

These choices can be composed. Changing a scheduler does not automatically change how `Result` binds; changing a `do` instance does not grant access to the filesystem or erase suspension effects.

# Ordinary functions with suspension effects

```haskell
loadPair :: forall r. using (io :: Io r) =>
            Request -> Request
            ->{Async r, IO r, Cancel r, Diverge}
              Result LoadError (Bytes, Bytes)
loadPair left right = withScope $ \s -> do
  a <- spawn s (\() -> fetch left)
  b <- spawn s (\() -> fetch right)
  joinAllOrCancel s a b
```

`fetch`, `spawn` and `joinAllOrCancel` are ordinary operations with declared effects. This block uses direct sequencing: `a` and `b` are handles and the final value is a `Result`. There is no future wrapper automatically inserted into every function result. The explicit task handle represents independently scheduled work.

The API contract specifies that spawn either produces a valid scoped handle or follows its typed task-failure/cancellation protocol, and that scope exit joins cleanup. `Diverge` records the possibility that an external dependency or cleanup never completes. The notation does not establish fairness or bounded latency.

A call's effects flow into its caller unless an appropriate handler discharges them. A pure callback cannot call a suspending operation. Higher-order APIs should quantify over effect rows rather than duplicate sync and async variants. This removes a special syntactic and return-type category, while retaining a static distinction between computations that can and cannot suspend.

# Runtime parameters and implicit passing

The `io` parameter is real runtime evidence, not an erased proof. It is passed explicitly in the core. Surface `using` permits lexical implicit passing, and `f @{io = chosen}` makes the argument explicit at a call. Implementations may specialize a statically known dictionary or use checked dispatch; neither optimization is a guaranteed zero-cost theorem.

Runtime identity `r` ties operations, registrations, task handles and dependent proof assumptions to their runtime. It is not freely coercible across runtimes. Runtime creation introduces a fresh identity; its scope prevents dependent handles and borrows from escaping unless an owned runtime package supports that escape.

No global or thread-local default runtime is searched. The nearest explicitly introduced lexical provider binds a capability name; multiple candidates at the same resolution level are an error. Closures capture the provider and its lifetime when created. Calling the closure in a different runtime scope does not rebind its dependencies.

Use small capabilities where possible: scheduling, network access, filesystem handles, time and allocation need not be bundled in one authority. `Io r` in examples is a convenient selected interface, not mandatory access to every OS service. Compile-time capabilities are separate from runtime ones.

# Handlers and entry points

`with (io = runtime.io) in body` supplies evidence; it does not by itself remove effects. A runtime entry function installs the handler and exposes its residual effects. The model is schematically:

```text
body : A with effects {Async r, IO r, Cancel r | e}
handler : handles those effects using Runtime r
run handler body : Outcome A with effects residual(handler, e)
```

This is a typing judgment, not source syntax. Residual effects can include host I/O, failure, allocation and divergence. An event loop may suspend host execution; it cannot claim to synchronously return a result merely because it handles `Async`. The browser entry adapter exposes a Promise/callback boundary as appropriate, while internal calls retain direct-style source syntax.

A handler receives a one-shot continuation with its captured resources and region constraints. It may resume it once, transfer it into an owned suspended task, or cancel it through a cleanup protocol. It may not silently discard linear obligations or clone mutable borrows. General multishot handlers require separate duplicability evidence and are outside the initial safe async handler subset.

# Unqualified do through modules and typeclasses

The language has one `do` spelling. A sequencing interpretation is a module implementing a checked compiler-known signature. The module contains its result/computation families, binding operations, permitted effects, continuation-usage rules and laws. Implicit dictionary syntax is convenience for passing this module; explicit module parameters support Backpack-like wiring and abstract associated types.

The initial direct interpretation executes each statement, binds its ordinary value and continues. The initial `Result` interpretation executes a statement, inspects its `Result`, continues with the success payload or returns the failure. `Option` can use the same one-shot structure. No interpretation is selected solely from the appearance of a `Result` in an ordinary function's return type: direct code must be able to inspect a `Result` as data.

```haskell
module ImportConfig (Seq :: ResultSequencing ConfigError) where
  -- Seq is the sequencing provider declared by this module's interface.
  loadConfig :: forall r. using (io :: Io r) =>
                Path ->{IO r, Async r, Cancel r, Diverge}
                  Result ConfigError Config
  loadConfig path = do
    bytes <- readConfigBytes path
    config <- parseConfig bytes
    pure config
```

Here the module parameter carries the designated sequencing-provider role in the signature. `readConfigBytes` reports `ConfigError`; its `Result` success payload becomes `bytes`. `parseConfig` is pure but returns the same error type. `pure` is the selected provider's ordinary operation. The compiler exposes the selected provider in diagnostics. There is no `Result.do` or other qualified-block requirement.

Outside a module selecting another provider, direct sequencing is the defined default. A generic function can require a sequencing dictionary in its `using` context; its elaborated body retains that dictionary when exported. Two active sequencing providers in the same selection scope are ambiguous unless explicitly bound through module wiring. No global instance priority or import-order tie-breaker changes block meaning.

Expected types constrain a selected provider and may infer its ordinary type parameters, such as `ConfigError`; they do not search noninjective associated families to guess control flow. Public signatures record the provider requirement when it is abstract. The details of the compiler-known provider role are part of the formal elaboration design, not an arbitrary convention that any module named `Seq` can exploit.

# Indexed sequencing and resource laws

A general monadic or stateful interpretation needs indexed action and result families so deferred effects cannot disappear. Its abstract shape is an indexed bind from an action with effects `e1` and a continuation producing an action with effects `e2` to a composed action accounting for both. The exact family may also index state transitions, resource ownership and failures. Ordinary direct expressions are reified into this representation only through the provider's checked elaboration interface.

It is unsound to use a naive unrestricted Haskell `Monad` dictionary that stores an effectful continuation in a supposedly pure value, duplicates a linear capture, or changes effect order without evidence. The initial compiler supports direct, `Result` and `Option` one-shot providers; arbitrary indexed providers require a specified and checked representation bridge before acceptance. This is an explicit implementation boundary, not a claim that all existing Haskell monads already fit.

For direct binding, `x <- rhs; rest` means execute `rhs`, bind its value and then execute `rest`. For the Result provider, it means execute `rhs`, return `Err e` on failure, or bind the `Ok` payload and execute `rest`. The compiler unions the effects of reachable computations conservatively; it can refine them only with checked evidence. `let` remains pure and strict in either interpretation. An unbound expression statement must satisfy the provider's discard/continuation rule. The final expression already has the provider's result shape; `pure` constructs that shape explicitly.

Monad identity and associativity laws are stated under an appropriate observational relation. They do not authorize arbitrary reordering, duplication, cancellation masking or omission of external effects. Effectful fusion rules must preserve the selected sequencing and cancellation contracts.

# Backend and compatibility consequences

Native backends may implement suspension using stackful tasks or transformed continuations. Wasm backends may lower to explicit state machines and WASI async calls without requiring Wasm stack switching. Runtime implementation choice is constrained by the same source effects and ownership contracts; it is not part of ordinary source function coloring.

An arbitrary blocking C call cannot become suspending by changing the `Io` dictionary. It needs an offload adapter, a genuinely asynchronous foreign API, or a declared blocking execution context. Foreign callbacks cannot unwind or suspend across an ABI that does not support that protocol.

Hot replacement checks runtime identity, handler contract, continuation representation, sequencing dictionary identity where specialized, and live registrations. Inlined sequencing operations and specialized runtime calls enter the dependent update set. Changing a scheduling policy can invalidate progress proofs even if all types are unchanged.

# Prior art and remaining formal work

[Zig 0.16's I/O interface](https://ziglang.org/download/0.16.0/release-notes.html) demonstrates explicitly passed I/O implementations. [Koka's effect handlers](https://koka-lang.github.io/koka/doc/book.html) demonstrate effect-based control abstractions. This proposal combines those inspirations with dependent modules, lexical evidence and ownership; it is not a statement that either language implements this exact design.

Required next formal results are effect preservation under handler lowering, safe one-shot continuation ownership, coherent dictionary elaboration, the indexed sequencing bridge, and a correspondence between runtime protocols and temporal models. Grammar checking alone establishes none of these.

[Documentation overview](../overview.md)