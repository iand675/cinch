# Effects, handlers, capabilities, and sequencing

Cinch separates four orthogonal mechanisms that other languages frequently conflate:

- **Effect rows** declare *what* a computation may do on its function arrows.
- **Capabilities** (`Io r`) grant lexical *access* to a specific runtime identity.
- **Effect handlers** interpret user-defined algebraic operations and discharge their labels from a row.
- **Sequencing providers** choose the meaning of `do` notation (`bind` and `pure`) lexically for a definition.

None substitutes for another: supplying an `Io r` capability with `with` does not remove `IO r` from an effect row, and handling an effect does not change how `do` sequences statements.

# Effect rows

Every function arrow carries an effect row in one of five forms:

| Syntax | Meaning |
| --- | --- |
| `A -> B` | Pure (closed empty row). |
| `A ->{} B` | Closed empty row, written explicitly. |
| `A ->{Poke} B` | Closed row containing exactly the listed labels. |
| `A ->{\|e} B` | Open row consisting solely of row variable `e`. |
| `A ->{Poke \| e} B` | Open row containing `Poke` plus tail variable `e`. |

Multiple labels in braces are comma-separated (e.g., `->{Async r, IO r, Cancel r, Diverge}`). The tail after `|` must be a row variable, never a concrete effect constructor.

Row variables have kind `Row`. Both `forall e.` (inferred in row position) and `forall (e :: Row).` declare a row variable; annotating a row variable with another kind is rejected:

<!-- cinch:reject contains="incompatible kind or non-row use" -->
```cinch
module Main where
  callback :: forall (e :: Type). (Unit ->{|e} Unit) ->{|e} Unit
  callback = callback

  main :: Int
  main = 0
```

In a curried function definition, effects belong on the final arrow that produces the result (`Int -> Int ->{Poke} Int`). Placing an effect row on an earlier arrow of a curried definition fails row unification against `{}`.

A declared row is an upper bound on the body and an exact contract for callers. A body may perform fewer effects than declared, never more; omitting a performed label fails with `effect rows do not match`:

<!-- cinch:run exit=42 -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  plain :: Int -> Int
  plain n = n + 1

  pokePlus :: Int ->{Poke} Int
  pokePlus n = poke () + n

  main :: Int
  main = plain 41
```

<!-- cinch:reject contains="effect rows do not match" -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  fired :: Unit ->{} Int
  fired u = poke ()

  main :: Int
  main = 0
```

## Row polymorphism in higher-order signatures

Quantifying over a row variable lets a higher-order function accept both pure and effectful callbacks without duplication. The caller's row receives whatever effects the supplied callback performs:

<!-- cinch:run exit=8 -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  twice :: forall e. (Unit ->{|e} Int) ->{|e} Int
  twice f = f () + f ()

  pureCall :: Int
  pureCall = twice (\u -> 4)

  effectfulCall :: Int ->{Poke} Int
  effectfulCall n = twice (\u -> poke () + n)

  main :: Int
  main = pureCall
```

Here `pureCall` instantiates `e` to `{}` so it can be called from a pure `main`, while `effectfulCall` instantiates `e` to `{Poke}`.

## `Diverge`

`Diverge` is an effect label indicating that a computation may fail to return:

```cinch
loop :: forall a b. a ->{Diverge} b
loop x = loop x
```

`Diverge` cannot be discharged by a handler. A pure nullary `main :: Int` cannot call a `Diverge` computation, and a handler whose own clauses diverge propagates `Diverge` to the enclosing `handle` expression.

# Capabilities and runtime identities

Effect labels may be indexed by a region-like identity `r` that binds operations to a specific runtime or memo scope:

| Label | Parameter | Origin |
| --- | --- | --- |
| `IO r` | runtime identity | `Io r` capability operations |
| `Log r`, `Trace r` | runtime identity | [Effect.Log](../library/Effect.Log.md), [Effect.Trace](../library/Effect.Trace.md) |
| `Async r`, `Cancel r` | runtime identity | Structured concurrency ([Chapter 13](13-concurrency-and-runtime.md)) |
| `Eval r` | memo region | Local `~` memo groups ([Chapter 11](11-laziness-and-codata.md)) |
| `Diverge` | none | Potentially non-terminating computations |
| `Foreign`, `IO`, `Global`, `Blocking`, `Fail`, `Reentrant`, `ThreadAffine` | boundary name on `Foreign` | `foreign import` declarations ([Chapter 15](15-ffi-layout-and-attributes.md)) |

A `foreign import` must declare every foreign behavior it can exhibit (`Blocking`, `Global`, `Fail`, `Reentrant`, `ThreadAffine`). In particular, calling a `Blocking` foreign function inside a spawned task propagates `Blocking` to the enclosing task scope's effect row.

Among `using` constraints, `Io r` is treated specially as a **lexical runtime capability**: it is resolved exclusively from an enclosing `with` binding or an explicit call-site `@{...}` argument, never from type-class instances or global state. All other `using` heads are ordinary compile-time class constraints.

## `using`, `with`, and `@{...}`

A function declares its capability requirement in a `using` clause. Callers provide the capability either lexically with `with (io = ...) in body` or directly at a call site with `@{io = ...}`:

<!-- cinch:run stdout="direct\ninside\n" exit=0 -->
```cinch
module Main where
  greet :: forall r. using (io :: Io r) => String ->{IO r} Unit
  greet s = ioWrite (string-to-bytes s)

  main :: Unit ->{IO r} Int
  main u = case greet @{io = newIO ()} "direct\n" of
    _ -> with (io = newIO ()) in case greet "inside\n" of
      _ -> 0
```

`newIO ()` creates a fresh runtime identity. Because capability bindings are ordinary lexically scoped values of type `Io r`, an inner `with` shadows an outer one and forwarding a capability is written `with (io = io) in ...`. Supplying a capability never removes `IO r` from the effect row; `with` supplies runtime access, not a handler.

Omitting a capability provider is rejected at the call site:

<!-- cinch:reject contains="capability in scope" -->
```cinch
module Main where
  greet :: forall r. using (io :: Io r) => String ->{IO r} Unit
  greet s = ioWrite (string-to-bytes s)

  main :: Unit ->{IO r} Int
  main u = case greet "x\n" of
    _ -> 0
```

Supplying a non-capability value in `with` or naming a nonexistent capability parameter in `@{...}` is likewise rejected at check time.

## Runtime identity isolation

Each runtime identity `r` is a rigid parameter of kind `Region`. Distinct identities never unify, so a capability for one runtime cannot satisfy an `IO` effect tagged with another:

<!-- cinch:reject contains="effect rows do not match" -->
```cinch
module Main where
  writeAt :: forall r. using (io :: Io r) => Unit ->{IO r} Unit
  writeAt u = ioWrite []

  confused :: forall r s. using (io :: Io r) => Io s -> Unit ->{IO r} Unit
  confused other u = writeAt @{io = other} u

  main :: Int
  main = 0
```

Here `writeAt` requires `IO r`, while passing `other :: Io s` produces `IO s`; the mismatch is caught by row unification. Runtime identities are preserved in checked `.cinchif` interfaces and across task/scope boundaries.

## Memo regions

A `~` memo binding block introduces a fresh region `r` and effect label `Eval r`. Demanding a cell performs `Eval r`. A cell's own body cannot contain `Eval r` for its own group, statically preventing self-referential black holes:

<!-- cinch:reject contains="memo cell cannot demand its own group" -->
```cinch
module Main where
  f :: Int -> Int
  f n = let ~x = copyOut (demand y) + 1
            ~y = copyOut (demand x) + 1
        in x

  main :: Int
  main = f 1
```

`Eval r` is discharged when the memo block exits. Explicit deferred types must be fully applied as `Thunk {e} a` and `Lazy {e} r a`; see [Chapter 11](11-laziness-and-codata.md).

# Algebraic effects and handlers

An `effect` block declares user-defined operations, each carrying its effect's label in its arrow:

```cinch
effect Poke where
  poke :: Unit ->{Poke} Int
```

A `handle expr with h` expression discharges the effect from `expr` and replaces it with any effects performed by the handler's clauses:

```cinch
handle poke () + 10 with (handler Poke where
  return x -> x * 2
  poke u resume k -> k 5)
```

A `handler Effect where` block contains:

- A **`return` clause** (`return x -> ...`), invoked when the handled computation finishes with a normal value.
- One **operation clause** per operation (`op args... resume k -> ...`), binding the operation arguments and the delimited continuation `k`. Duplicate clauses for the same operation are rejected.

Handlers are first-class values: `let h = (handler Poke where ...) in handle e with h` is equivalent to writing the handler inline.

## Handler semantics

| Case | Behavior |
| --- | --- |
| Direct operation in handled expression or `do` block | Dispatched to its matching handler clause. |
| Operation passed as a value without calling it | Accepted; referencing `poke` performs no effect. |
| Clause resumes once (`k v`) | Runs the remainder of the handled computation, then the `return` clause. |
| Clause does not resume | Short-circuits immediately; neither the remainder nor `return` runs. |
| Clause resumes more than once | Rejected (`k` is one-shot). |
| Clause performs other effects | Those effects join the `handle` expression's output row. |
| Operation performed inside a called helper function | Rejected at `handle` (see [handler boundaries](#handler-boundaries)). |

Resuming once runs the rest of `poke () + 10` (yielding `15`) and then `return x -> x * 2` (yielding `30`):

<!-- cinch:run exit=30 -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = handle poke () + 10 with (handler Poke where
    return x -> x * 2
    poke u resume k -> k 5)
```

Binding a handler to a local variable:

<!-- cinch:run exit=42 -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  answer :: Unit -> Int
  answer u = let h = (handler Ask where
      return x -> x
      ask a resume k -> k 41) in
    handle ask () + 1 with h

  main :: Int
  main = answer ()
```

Handler clauses may perform other effects. The handled label (`Ask`) is removed from the row, while the clause's effects (`IO r`) propagate outward:

<!-- cinch:run stdout="asked\n" exit=7 -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  greet :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  greet u = handle ask () with (handler Ask where
    return x -> x
    ask a resume k -> case ioWrite (string-to-bytes "asked\n") of
      _ -> k 7)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in greet ()
```

## One-shot resumptions

Handler continuations are strictly one-shot:

- By default, `k` has multiplicity `%aff` (at most once). Resuming twice is rejected even with no linear captures:

<!-- cinch:reject contains="must be used at most once" -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  main :: Int
  main = handle poke () with (handler Poke where
    return x -> x
    poke u resume k -> k 1 + k 2)
```

- When the handled computation captures a `%1` (linear) value, `k` is promoted to `%1` (exactly once), so both dropping `k` and calling `k` twice are rejected. Resuming once consumes the linear value cleanly:

<!-- cinch:run exit=6 -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  consume :: Int %1 -> Int
  consume x = x

  run :: Int %1 -> Int
  run lin = handle poke () + consume lin with (handler Poke where
    return x -> x
    poke u resume k -> k 1)

  main :: Int
  main = run 5
```

## Region-parameterized effects

User effects can be indexed by a region identity `(r :: Region)`. The corresponding `handler` header repeats the binder (`handler Mark r where`):

<!-- cinch:run exit=9 -->
```cinch
module Main where
  effect Mark (r :: Region) where
    mark :: Unit ->{Mark r} Int

  main :: Int
  main = handle mark () with (handler Mark r where
    return x -> x
    mark u resume k -> k 9)
```

Omitting `r` in the handler header or annotating an effect parameter with a kind other than `Region` is rejected.

## Handler boundaries

- **Polymorphic operations.** Type parameters on an operation are universally quantified inside the handler clause, so a clause cannot resume with an arbitrary concrete type or allow an operation-local type variable to escape into the handler's result. (A clause that never resumes can return any answer type.)
- <span class="support-tag support-tag--planned">Coming soon</span> **Cross-call operation dispatch.** Handlers currently dispatch operations performed directly within the handled expression (including direct statements inside a `do` block), not operations invoked transitively through a separate function call:

<!-- cinch:reject contains="performed through a called computation" -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  fired :: Unit ->{Poke} Int
  fired u = poke ()

  main :: Int
  main = handle fired () with (handler Poke where
    return x -> x
    poke u resume k -> k 1)
```

Until answer-passing elaboration lands, keep handled operations direct within the `handle` body rather than behind helper calls.

# Sequencing providers (`do` notation)

`do` notation is not hardwired to a `Monad` type class. Instead, each definition selects a **sequencing provider** that supplies `pure` and `bind`. Three core providers live in `Effect.Sequencing`:

| Provider | `pure` | `bind` |
| --- | --- | --- |
| `DirectSequencing` | `forall a. a -> a` | `forall a b ef. a -> (a ->{ef} b) ->{ef} b` |
| `ResultSequencing e` | `forall a. a -> Result e a` | `forall a b ef. Result e a -> (a ->{ef} Result e b) ->{ef} Result e b` |
| `OptionSequencing` | `forall a. a -> Option a` | `forall a b ef. Option a -> (a ->{ef} Option b) ->{ef} Option b` |

## Provider selection

Selection is strictly lexical and per definition:

- **No provider binder in `using`:** `DirectSequencing` is active by default.
- **One provider binder in `using`:** That provider governs every `do` block and bare `pure` in the definition (the binder's variable name does not matter).
- **Two provider binders in one scope:** Rejected as ambiguous at each `do` and `pure`.
- **Return types and `Category.*` classes:** Returning `Result` or `Option`, or constraining `using (m :: Monad f)`, **never** selects a `do` provider.

Returning `Result` without a `ResultSequencing` binder leaves `DirectSequencing` active, where `x <- r` binds the `Result` itself rather than unwrapping `Ok`:

<!-- cinch:reject contains="integer literal cannot have type Result Int Int" -->
```cinch
module Main where
  step :: Result Int Int -> Result Int Int
  step r = do
    x <- r
    pure (x + 1)

  main :: Int
  main = 0
```

## Elaboration of `do` blocks

| Statement | Desugaring |
| --- | --- |
| `x <- e` followed by `rest` | `bind e (\x -> rest)` |
| `e` followed by `rest` | `bind e (\_ -> rest)` (discarded value; unconstrained literals need an annotation) |
| `let ...` | Pure, strict local binding group; not passed through `bind`. |
| Final statement | Must be an expression; produces the block's result. |
| `pure x` | Calls the active provider's `pure`. |

Key rules for `do` statements:

- `let` statements inside `do` must be pure (performing an effect in a `let` inside `do` is rejected). One `let` group may bind multiple non-duplicate names and local helpers, except directly under a `handle` body where only simple value bindings are allowed.
- Patterns on the left of `<-` must be irrefutable; refutable patterns are rejected because the built-in providers define no typed pattern-failure hook.
- Sequencing never hides effects: effects performed inside `do` propagate to the enclosing definition's row identically to explicit expressions.

## Built-in providers

**`DirectSequencing`** (default) uses `pure x = x` and `bind v k = k v`, so `<-` sequences plain values with no wrapper:

<!-- cinch:run exit=7 -->
```cinch
module Main where
  main :: Int
  main = do
    x <- pure 3
    y <- 4
    pure (x + y)
```

**`ResultSequencing e`** unwraps `Ok` payloads and short-circuits on `Err`, wrapping the final value with `pure` into `Ok`:

<!-- cinch:run exit=45 -->
```cinch
module Main where
  step :: forall e. using (provider :: ResultSequencing e) => Result e Int -> Result e Int
  step r = do
    x <- r
    let y = x + 1
    pure y

  main :: Int
  main = case (step (Ok 41), step (Err 3)) of
    (Ok n, Err m) -> n + m
    (Ok n, Ok m) -> n + m
    _ -> 0
```

**`OptionSequencing`** unwraps `Some` payloads and short-circuits on `None`:

<!-- cinch:run exit=5 -->
```cinch
module Main where
  step :: using (provider :: OptionSequencing) => Option Int -> Option Int
  step o = do
    x <- o
    pure (x + 1)

  main :: Int
  main = case (step (Some 4), step None) of
    (Some n, None) -> n
    (Some n, Some m) -> n + m
    _ -> 0
```

**`StmSequencing`** (in `Effect.Stm`) provides `pure :: a -> STM a` and `bind :: STM a -> (a -> STM b) -> STM b` via `using (seq :: StmSequencing)`. Because an `STM` transaction may retry, the continuation after `<-` can execute multiple times and therefore cannot capture a `%1` linear value. See [software transactional memory](13-concurrency-and-runtime.md#software-transactional-memory).

## `pure` resolution

Inside a definition with an active sequencing provider, bare `pure` resolves to that provider's `pure` unless a non-provider class constraint in scope also provides `pure` (which takes precedence). To target the `Applicative` class method unambiguously, qualify it as `Applicative.pure`:

```cinch
wrap :: forall f. using (app :: Applicative f) => Int -> f Int
wrap n = Applicative.pure n
```

`Monad` has no `return` method; `return` is reserved exclusively for handler clauses.

## Module-level provider parameters

Imported library modules may accept signature or value parameters in their module headers (see [module parameters](07-modules-and-packages.md#module-parameters)), but an **entry module** declaring `main` cannot take module parameters, and module-header provider selection is not yet wired into executable entry points. Select sequencing providers per definition using a `using` binder.

# Entry points and boundaries

Executable entry points admit exactly two signatures—`main :: Int` (pure) and `main :: Unit ->{e} Int` (effectful, with no `using` clause)—where the declared row establishes the program's effect envelope and capabilities are introduced inside `main` via `with (io = newIO ()) in`. See [entry points and exit statuses](01-using-the-compiler.md#entry-points-and-exit-statuses) and [entry points, exit statuses, and runtime diagnostics](13-concurrency-and-runtime.md#entry-points-exit-statuses-and-runtime-diagnostics) for the full entry diagnostics and reserved runtime exit codes.

Compile-time staging runs with its own recorded capabilities rather than ambient runtime I/O (see [staging and macros](14-staging-and-macros.md)).

- <span class="support-tag support-tag--planned">Coming soon</span> **`Comp {e} a` and resource-safe effectful pull streams.** Packaging an effectful unit closure as `Comp {e} a` (with `suspend :: (Unit ->{e} a) -> Comp {e} a` and `run :: Comp {e} a ->{e} a`) in `Effect.Comp` is planned ([computation-type design](../design/comp-computation-type.md)); until it ships, store explicit effectful closures or use `Thunk {e} a` and `codata`.
- <span class="support-tag support-tag--planned">Coming soon</span> **Handler specialization.** Handlers are represented as clause-closure dictionaries at runtime; specializing statically known handlers is a planned optimization that does not change typing rules.

For guided walkthroughs, see [effects and I/O](../learn/10-effects-and-io.md) and [handlers and sequencing](../learn/11-handlers-and-sequencing.md). Normative details are in the [runtime capabilities, effects, and sequencing specification](../specifications/runtime-capabilities-effects-and-sequencing.md).
