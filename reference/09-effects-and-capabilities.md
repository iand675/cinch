# Effects, handlers, capabilities, and sequencing

An effect row describes what a computation may do. A capability supplies access to one runtime identity. A handler interprets the operations of a user effect. A sequencing provider fixes the meaning of `do` for one definition. The four mechanisms compose, and none of them substitutes for another: supplying an `Io r` capability does not remove `IO r` from a row, and handling an effect does not select a `do` provider.

# Effect rows

An arrow carries a row in one of these forms:

| Spelling | Meaning |
| --- | --- |
| `A -> B` | Pure. The closed empty row. |
| `A ->{} B` | The same closed empty row, written explicitly. |
| `A ->{Poke} B` | A closed row holding the labels written between the braces. |
| `A ->{\|e} B` | An open row: a row variable and no fixed labels. |
| `A ->{Poke \| e} B` | An open row: the labels, then `\|`, then one row variable. |

Labels in a brace list are separated by commas, for example `->{Async r, IO r, Cancel r, Diverge}`. The tail after `|` must be a variable; a constructor there is refused with ``expected a row variable after `|`, found constructor `Eval` ``.

A row variable is a type parameter of kind `Row`. `forall e.` and `forall (e :: Row).` both declare one. Another kind annotation on a row position is refused:

<!-- cinch:reject contains="incompatible kind or non-row use" -->
```cinch
module Main where
  callback :: forall (e :: Type). (Unit ->{|e} Unit) ->{|e} Unit
  callback = callback

  main :: Int
  main = 0
```

A curried definition carries its effects on the arrow that produces the result. `f :: Int -> Int ->{Poke} Int` may perform `Poke`; the same row on an earlier arrow is refused, because the earlier arrows of a curried definition unify with the pure row:

```text
cinch: 6:3: effect rows do not match: {Poke} vs {}
```

A declared row is the contract callers see. The body may perform fewer labels than it declares and never more. Here `plain` is pure, `pokePlus` performs `Poke`, and `main` stays pure because it calls only `plain`:

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

Declaring a label the body does not perform still puts that label on every caller. A body that performs a label its declaration omits is refused. Both directions are the same row unification, and each is reported as `effect rows do not match` with the two rows:

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

## Row variables in higher-order signatures

A higher-order API quantifies over the row rather than duplicating a pure and an effectful variant. One definition of `twice` then accepts a pure callback and an effectful one, and the caller's declared row records what the chosen callback performs:

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

`effectfulCall` declares `{Poke}` and checks; instantiating `e` with the empty row gives `pureCall` no effect at all. The same instantiation rule is why `pureCall` can be evaluated by a pure `main`.

## `Diverge`

`Diverge` is an ordinary label in the row vocabulary and records that a computation may not return. A non-terminating definition is written with it:

```cinch
loop :: forall a b. a ->{Diverge} b
loop x = loop x
```

The label is admitted by declaration, not proved absent by handling. A pure entry point cannot contain it, so a `main :: Int` that calls `loop` is refused:

```text
cinch: 6:3: effect rows do not match: {} vs {Diverge | ?r895} (a nullary definition must be pure in the bootstrap slice)
```

Wrapping a looping computation in a handler does not discharge `Diverge`. A handler clause that loops makes the whole `handle` expression divergent, so the definition that owns it must declare `{Diverge}`, and a pure `main` that reaches that definition is refused with the same message.

# Capability indices

A label may take a runtime identity as its argument, which ties the operation to one runtime. The labels a user meets are:

| Label | Argument | Introduced by |
| --- | --- | --- |
| `IO r` | runtime identity | the builtin `Io` head over `r` |
| `Log r`, `Trace r` | runtime identity | [Effect.Log](../library/Effect.Log.md), [Effect.Trace](../library/Effect.Trace.md) |
| `Async r`, `Cancel r` | runtime identity | the structured-task builtins ([concurrency](13-concurrency-and-runtime.md)) |
| `Eval r` | memo region | the `~` memo group that introduces `r` |
| `Diverge` | none | any definition that may not return |
| `Foreign`, `IO`, `Global`, `Blocking`, `Fail`, `Reentrant`, `ThreadAffine` | `Foreign` takes the boundary name | a `foreign import` whose row accounts for the behaviour |

A `foreign import` has to account for what the C function can do, so its declared row is where `Blocking`, `Global`, `Fail`, `Reentrant`, and `ThreadAffine` appear. Leaving out a behaviour the call has is an effect honesty failure, and `Blocking` in particular reaches the spawning scope's row when a task calls it.

`Io r` is the one class head the checker treats as a runtime capability. A `using (x :: C ...)` binder whose head is `Io` is a capability parameter, solved only from a lexical provider or an explicit supply, never from an instance and never from global state. Every other `using` head is an ordinary compile-time class constraint. This split is decision D21 in [the specification](../specifications/runtime-capabilities-effects-and-sequencing.md).

## `using` contexts, `with`, and `@{...}`

A definition requests the capability in its `using` context. A caller supplies it lexically with `with (io = newIO ()) in body`, or at the call with `@{...}`:

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

`newIO ()` constructs a fresh runtime identity. The capability name is a term-level value of type `Io r`, so an inner `with` shadows an outer one and forwarding a capability is written `with (io = io) in ...`. Three refusals cover the mistakes:

| Mistake | Diagnostic |
| --- | --- |
| No provider anywhere in scope | ``no Io r#846 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause`` |
| `with` supplies a value of the wrong type | ``supply io must have type `Io r` or another checked signature, got Int`` |
| `@{...}` names a parameter the definition does not have | `greet has no capability parameter cap` |

<!-- cinch:reject contains="capability in scope" -->
```cinch
module Main where
  greet :: forall r. using (io :: Io r) => String ->{IO r} Unit
  greet s = ioWrite (string-to-bytes s)

  main :: Unit ->{IO r} Int
  main u = case greet "x\n" of
    _ -> 0
```

Supplying a capability does not remove the effect. The row `{IO r}` stays on the call and on the caller, which is why the examples above declare `main :: Unit ->{IO r} Int` rather than `main :: Int`. `with` supplies evidence; it is not a handler.

## Runtime identity

The identity `r` is a rigid region-like parameter, and two identities do not unify. A capability for one identity cannot stand in for a row naming another, and the refusal is row unification again:

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

The checker reports `effect rows do not match: {IO r#847} vs {IO s#846 | ?r895}`: `writeAt` needs `IO r`, the supplied capability writes at `IO s`, and the two are distinct. The identity also appears in every task and scope contract, and a checked interface keeps it printed, for example `io :: Io r` with `r :: Region` in the `.cinchif` contract of an exported helper.

## Memo regions

A `~` memo group introduces one region `r` and one `Eval r` label for the group. A demand performs `Eval r`, so the label appears in rows, and a memo cell whose evaluation row contains its own group's label is refused statically:

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

The message names the identity: `effect row must not contain Eval r#13 (a memo cell cannot demand its own group): this memo binding demands its own memo group`. The group's label is discharged at the end of its scope, so a definition whose `~` bindings are local does not export `Eval r`. The value types carry the row instead: `Thunk {e} a` holds the row of the computation it stores, `Lazy {e} r a` names the region as well, and both must be fully applied. Writing a bare `Thunk a` is refused with ``Thunk and Lazy must be fully applied as `Thunk {e} a` and `Lazy {e} r a` ``.

# Handler declarations and clauses

`effect` declares the operation signatures of a user effect, and each operation performs exactly its effect's label:

```cinch
effect Poke where
  poke :: Unit ->{Poke} Int
```

`handle` discharges the labels of one effect from the handled expression and replaces them with whatever the clauses perform:

```cinch
handle poke () + 10 with (handler Poke where
  return x -> x * 2
  poke u resume k -> k 5)
```

The handler form is `handler Effect where` followed by clauses at the same indentation. A `return` clause receives the value of the handled computation when it finishes without an unhandled operation. An operation clause matches the operation's arguments, then its resumption, and names the continuation `k` after `resume`. The parentheses matter: `handle e with h` takes a handler expression, and a handler is an ordinary value, so `let h = (handler Poke where ...) in handle e with h` is the same program.

## Which handler programs work today

| Form | Today |
| --- | --- |
| Operation performed by the handled expression itself | Dispatched to its clause. |
| Operation performed inside a `do` block under the handler | Dispatched, when the block's statements are direct. |
| Operation value passed to another function without being performed | Accepted. Constructing or passing `poke` performs nothing. |
| Operation reached through a called computation | Refused at `handle`. This is the cross-call boundary below. |
| Clause that resumes once | Accepted; the continuation runs the rest of the handled computation and the return clause. |
| Clause that never resumes | Accepted; the clause's value is the result of the `handle` and the rest of the computation never runs. |
| Clause that resumes twice | Refused by the multiplicity checker. |
| Resumption that consumes a `%1` capture | Accepted exactly once; the resumption binder is promoted to `%1`, so a second use is refused. |
| Clause that performs effects | Accepted; the effects join the `handle` expression's row, so they reach the enclosing definition. |
| Duplicate clause for one operation | Refused: `duplicate handler clause for operation poke`. |
| Handler bound to a name, then used | Accepted. |

Three verified programs cover the accepted shapes. Resuming once runs the rest of the handled expression and then the return clause, and the clause's own value is what the continuation becomes:

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

Never resuming returns the clause's value and abandons the rest: the same handler with `poke u resume k -> 7` gives exit 7, because `+ 10` and the return clause never run.

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

A clause may perform effects of its own. The handled label is removed and the clause's row is added, so `greet` below declares `{IO r}` for the write inside the clause and does not declare `Ask` at all:

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

## One-shot resumption

The continuation is `%aff` by construction: it may be used at most once, and the checker enforces that at the second use. Two resumes in one clause are refused even when the continuation captures nothing linear:

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

The full diagnostic is `multiplicity %aff: k must be used at most once, but is used 2 time(s)`. When the handled expression consumes a `%1` value, the continuation holds that value, and both dropping the resumption and duplicating it are then refused with `must be used exactly once`. Resuming exactly once is accepted and consumes the capture once:

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

An effect may take a region identity, which is how a user effect is indexed by a runtime. The parameter is declared `(r :: Region)` and the handler takes the same binder:

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

Omitting the binder is refused at the handler with ``effect Async has a region parameter: write `handler Async r where` ``. Any other kind annotation on the parameter is refused at the declaration with ``effect parameters are region identities (`(r :: Region)`); other kinds are not supported in the bootstrap slice``.

## Handler typing boundaries

An operation's type parameters are universal in the clause, so a clause cannot pick a result type for a polymorphic operation. A definition of `magic :: forall a. Unit ->{Magic} a` handled at `Int` refuses a clause that resumes with a literal and reports `type mismatch`. A clause cannot return an operation-local type parameter as the answer either; that case is refused with `operation-local type parameter escapes`. A polymorphic operation that never resumes is accepted at any result type, because the clause's own value is the answer.

<span class="support-tag support-tag--planned">Coming soon</span> **cross-call operation dispatch.** Handlers interpret operations performed by the handled expression itself, including those inside a `do` block under the handler. An operation reached through a separate call is not dispatched, and the checker refuses the `handle` rather than leaving the operation to run uninterpreted:

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

The diagnostic is `the handled effect {Poke} is performed through a called computation; the bootstrap handler calculus dispatches operations only from the handled computation's own expression`. Answer-passing elaboration is the planned direction, and until it lands the restriction is a real limit: a helper that performs an operation cannot be handled from its caller.

Handler-vtable specialization is a separate planned optimizer extension. A handler is already a dictionary of clause closures at runtime; specializing a statically known handler is an optimization, not a change in which programs check.

# Sequencing providers

`do` notation reads the same under three providers. The provider supplies `pure` and `bind`, and the whole difference between the three is those two signatures, which are their declarations in `Effect/Sequencing.cinch`:

| Provider | `pure` | `bind` |
| --- | --- | --- |
| `DirectSequencing` | `forall a. a -> a` | `forall a b ef. a -> (a ->{ef} b) ->{ef} b` |
| `ResultSequencing e` | `forall a. a -> Result e a` | `forall a b ef. Result e a -> (a ->{ef} Result e b) ->{ef} Result e b` |
| `OptionSequencing` | `forall a. a -> Option a` | `forall a b ef. Option a -> (a ->{ef} Option b) ->{ef} Option b` |

## Selecting a provider

The selection is lexical, per definition, and comes from one place: a `using` binder whose class head is one of the three providers. The binder's name is yours; only the head matters.

| Rule | Consequence |
| --- | --- |
| No provider binder in the definition | Direct sequencing. |
| One provider binder, of any name | That provider, for every `do` and every bare `pure` in the definition. |
| Two provider binders in one selection scope | Refused at each `do` and `pure`: `two active sequencing providers in one selection scope are ambiguous (ResultSequencing e#844, OptionSequencing); bind one explicitly`. |
| A return type that mentions `Result` or `Option` | Selects nothing. |
| A `Category.*` class constraint such as `Monad` | Selects nothing; those classes abstract over containers for polymorphic code and never feed `do`. |

The return type rule is the one that surprises people, so it is worth a refusal of its own:

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

Under the default direct provider, `x <- r` binds the whole `Result`, `pure` is the identity, and `x + 1` has no meaning; the literal reports `integer literal cannot have type Result Int Int`. Writing `using (provider :: ResultSequencing Int)` in the signature is what changes the block's meaning.

## What `do`, `<-`, and `let` become

With `bind` and `pure` the two provider methods, a block elaborates as follows.

| Statement | Elaboration |
| --- | --- |
| `x <- e` then `rest` | `bind e (\x -> rest)`. The payload type of `e` is what `x` names. |
| `e` then `rest`, with no binding | `bind e (\_ -> rest)`. The payload is discarded, so an unconstrained literal there needs an annotation. |
| `let` and its local block | Bound strictly, as a pure local group, and not sequenced through `bind`. |
| Last statement | Must be an expression, and it is the block's value. |
| `pure x` | The provider's `pure` at `x`, which builds the block's result shape. |

Consequences that follow from that table and are worth stating separately:

- A `let` statement must be pure. Its interpretation does not sequence effects, so `let x = poke ()` inside `do` is refused with ``a `let` statement in `do` must be pure (its interpretation does not sequence effects)``.
- One `let` group binds one whole local block, so `let { inc :: Int -> Int; inc n = n + 1; v = inc 40 }` and a multi-line group both work, and the same name twice in one group is refused with ``duplicate binding `x` in the `let` group``.
- Under a handled computation only simple value bindings sequence; a local function equation or a `~` memo binding in the group is refused with `local functions and memo bindings under a handled computation`.
- A refutable pattern on the left of `<-` is refused, because the three providers have no typed failure operation: ``refutable patterns in `do` bindings need a provider failure operation; the direct, Result and Option bootstrap providers do not supply one``.
- The last statement must be an expression: `the last statement of a do block must be an expression`.
- Binds do not hide effects. A `do` block in a `main :: Int` that performs a user effect is refused exactly as the same expression outside `do` would be.

## Direct provider

Direct sequencing is the default and the identity: `bind v k = k v` and `pure x = x`. The payload of a statement is the statement's own value, so a literal on the right of `<-` needs no container:

<!-- cinch:run exit=7 -->
```cinch
module Main where
  main :: Int
  main = do
    x <- pure 3
    y <- 4
    pure (x + y)
```

`y <- 4` binds `4` itself, and the final `pure (x + y)` is `3 + 4`. Naming the provider explicitly changes nothing:

```cinch
step :: using (provider :: DirectSequencing) => Int -> Int
step n = do
  x <- pure (n + 1)
  pure x
```

## `Result` provider

Under `ResultSequencing e`, a statement on the right of `<-` has type `Result e a`, the success payload is what `x` names, and a failure returns at once. The block's final expression must therefore be a `Result e b`; `pure` wraps a payload in `Ok`.

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

`step (Ok 41)` gives `Ok 42` and `step (Err 3)` gives `Err 3`, so the exit status is `42 + 3`. The error type is the provider's parameter `e`, which the expected type pins to `Int`; the block never names it.

## `Option` provider

`OptionSequencing` is the same shape with `Some` and `None`. There is no error payload, so the provider takes no parameter:

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

`step (Some 4)` gives `Some 5` and `step None` gives `None`, so the exit status is `5`.

## `pure` resolution

`pure` is a class method. Inside a definition with a provider binder, a bare `pure` is the provider's method. When a non-provider class constraint with a `pure` method is in scope, that constraint answers the bare use first, and `Applicative.pure` is the qualified spelling that always names the class method:

```cinch
wrap :: forall f. using (app :: Applicative f) => Int -> f Int
wrap n = Applicative.pure n
```

`Monad` has no `return`; the word does not exist as a method, and `return` appears only as handler clause syntax.

## Module parameters

The design also describes selecting a provider through a module parameter, as in `module ImportConfig (Seq :: ResultSequencing ConfigError) where`. The checker accepts that header and typechecks the module body. A program whose module declares a parameter does not currently run, whatever the parameter is:

```text
$ cinch check Main.cinch      # accepts the header and the body; prints nothing
$ cinch run Main.cinch
cinchruntime: compiler bug: null or wrong-kind word or corrupt generated data
$ echo $?
71
```

The same fault occurs for a parameter with no relation to sequencing, such as `module Main (N :: Nat) where`, so the limit belongs to module parameters rather than to provider selection. Use the `using` binder form above today.

# Computations stored as values

<span class="support-tag support-tag--planned">Coming soon</span> **`Comp {e} a` and resource-safe effectful pull streams.** The selected design packages an effectful unit closure in an ordinary transparent library type, with `suspend :: (Unit ->{e} a) -> Comp {e} a` and `run :: Comp {e} a ->{e} a`. The law is `run (suspend k) = k ()`: storing a computation executes nothing and sequences nothing, and the closure's ownership and row govern running and sharing it.

`Effect.Comp` is not a shipped module, so the import fails: ``module Effect.Comp is imported, but Effect.Comp.cinch does not exist and no other source file beside the entry declares `module Effect.Comp` ``. Programs use explicitly typed effectful closures and the existing thunk and codata APIs instead of assuming a container-polymorphic effectful-stream library. The [computation-type decision](../design/comp-computation-type.md) fixes the direction and leaves the implementing streaming slice responsible for the exact resource-safe stream API.

# Entry points and boundaries

The entry signature is `main :: Int` for a pure program or `main :: Unit ->{e} Int` for an effectful one. Nothing else is admitted, and `main` takes no other parameters and no `using` context:

| Written | Diagnostic |
| --- | --- |
| `main :: Unit -> Unit -> Int` | `main must have type Int or Unit ->{e} Int in the bootstrap slice, got (Unit -> (Unit -> Int))` |
| `main :: forall r. using (io :: Io r) => Int` | `main must have type Int or Unit ->{e} Int in the bootstrap slice` |

The effectful entry's declared row is the program's effect envelope. A capability is supplied at the entry with `with (io = newIO ()) in`, as every runnable example above does; a pure `main :: Int` cannot hide an unhandled effect, which is the `must be pure` refusal quoted under [rows](#effect-rows).

Foreign operations declare their effects honestly, including `Blocking` and failure where the foreign call blocks or fails. Staging uses its own recorded capabilities rather than inheriting ambient runtime I/O.

# Implementation

The checker rules live in `check_terms.rs` and `check_decls.rs`: row unification in `types.rs`, handler elaboration and `@{...}` application in `check_terms.rs`, the entry rule in `check_decls.rs`. The provider declarations are `compiler/prelude/Effect/Sequencing.cinch`. End-to-end suites are `effects.rs`, `concurrency.rs`, `observability.rs`, and `interface_region_effects.rs`.

Tutorial chapters: [effects and I/O](../learn/10-effects-and-io.md) and [handlers and sequencing](../learn/11-handlers-and-sequencing.md). The normative text is [runtime capabilities, effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md); the implementation decisions are in [the design note](../design/effects-capabilities-sequencing.md).
