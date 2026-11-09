# 11. Handle effects and select sequencing

An effect row names what a computation may perform. An `effect` declaration gives a name to a set of operations, and a handler supplies their meaning. A handler is not a capability: a capability hands over a runtime resource, while a handler receives the operation's arguments and a resumption that stands for the rest of the computation, so it can decide whether that rest runs at all. This chapter declares an effect, writes handlers, quotes the refusals that constrain them today, and selects among the three `do` providers. Check with `$C check Main.cinch` and run with `$C run Main.cinch`, where `$C` is `compiler/target/debug/cinch`.

# Before you start

You need effect rows, capabilities, `with`, and `do` from [chapter 10](10-effects-and-io.md), and `using` clauses from [chapter 9](09-signatures-and-instances.md), because a sequencing provider is a class like any other.

# Declare an effect and its operations

```cinch
  effect Ask where
    ask :: Unit ->{Ask} Int
    hint :: Int ->{Ask} Int
```

That fragment declares two operations. `effect Ask` names the effect, each indented line declares one operation with an ordinary type, and each type must carry the effect's own label in its row. An operation whose arrow leaves out the label is rejected:

<!-- cinch:reject contains="must perform its effect's label" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit -> Int

  main :: Int
  main = 0
```

```text
cinch: 3:5: effect operation ask must perform its effect's label {Ask} (in Main.cinch)
```

The row requirement keeps an operation from masquerading as a pure function. `ask :: Unit ->{Ask} Int` returns an `Int` in a computation that may perform `Ask`; it is not a function you can call in a pure arrow.

| Piece | Declared with | Supplies | Given by |
| --- | --- | --- | --- |
| effect | `effect Ask where` | operation names and types | the module |
| capability | a value of type `Io r` | a runtime handle | `newIO ()` under `with` |
| handler | `handle e with (handler Ask where ...)` | an interpretation of each operation | the expression |

# Write a handler

A handler has one clause per operation plus a `return` clause, as in this fragment:

```cinch
  answer :: Unit ->{Ask} Int
  answer u = ask ()

  handled :: Unit -> Int
  handled u = handle answer () with (handler Ask where
      return v -> v
      ask unit resume k -> k 20)
```

`handle` takes the handled expression, then `with`, then a parenthesised handler. `handler Ask` names the effect being interpreted. `return v -> v` says what the handled computation's own final value is; the clause is required, and omitting it is rejected with ``a handler needs a `return p -> e` clause``.

Each operation clause gives one pattern per operation argument, then the literal word `resume`, then a binder for the resumption. `ask` takes one `Unit` argument, so the clause is `ask unit resume k -> ...`. `resume` is syntax, not a name you choose: replacing it with another binder makes the clause ill formed. `k` is yours, and `k 20` resumes the handled computation with `20` as the result of `ask ()`.

`handled`'s type is `Unit -> Int`, with no row and no `using` clause. Handling removes `Ask` from the row of the handled expression, so the effect is discharged where the handler is written.

# A complete program

<!-- cinch:run exit=0 stdout="42 7\n" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  twice :: Unit -> Int
  twice u = handle ask () * 2 with (handler Ask where
      return v -> v
      ask unit resume k -> k 21)

  once :: Unit -> Int
  once u = handle ask () * 2 with (handler Ask where
      return v -> v
      ask unit resume k -> 7)

  line :: Int -> Int -> String
  line a b = string-concat (int-to-string a) (string-concat " " (string-concat (int-to-string b) "\n"))

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in do
    resumed <- pure (twice ())
    skipped <- pure (once ())
    written <- ioWrite (string-to-bytes (line resumed skipped))
    pure 0
```

The run prints `42 7` and exits `0`.

`twice` handles `ask () * 2`. The handler resumes with `21`, so the multiplication runs and `twice ()` is `42`. `once` handles the same expression and returns `7` without calling `k`, so the multiplication never runs; the clause's own value becomes the value of the whole handled expression. The `return` clause is unused here, because both handlers stop at the operation clause. It applies when the handled computation finishes without performing the operation.

`main` binds both results with `pure`, because `<-` sequences an effect and a handled expression is already pure. Writing an effectful expression under `let` is rejected with ``a `let` statement in `do` must be pure (its interpretation does not sequence effects)``.

# A resumption is used at most once

`k`'s multiplicity is `%aff`: at most once on every path. Calling it twice duplicates the continuation, and the checker refuses:

<!-- cinch:reject contains="multiplicity %aff: k must be used at most once" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  main :: Int
  main =
    let value = handle ask () with (handler Ask where
          return v -> v
          ask unit resume k -> k 20 + k 22)
    in value
```

```text
cinch: 9:27: multiplicity %aff: k must be used at most once, but is used 2 time(s) (in Main.cinch)
```

This is the discipline [chapter 12](12-ownership-and-regions.md) applies to resources, applied to the resumption itself. Dropping `k` instead of calling it is allowed, and that is what a non-resuming clause does; the rest of the handled computation is abandoned. Handling does not grant permission to duplicate whatever the continuation holds.

# A handler clause can perform effects

Nothing in a clause is restricted to pure code. This program writes from inside the clause:

<!-- cinch:run exit=41 stdout="served\n" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let answer = handle ask () with (handler Ask where
          return v -> v
          ask unit resume k -> let wrote = ioWrite (string-to-bytes "served\n") in k 41)
    in answer
```

It prints `served` and exits `41`, because the handled expression's value is `41` and `main` returns it. The `IO r` in the clause's body must appear in the row of the enclosing definition unless another handler accounts for it. Handling `Ask` removes `Ask` only.

# The three sequencing providers

`do` notation takes its meaning from a provider class. The same spelling selects a different class according to the definition's `using` clause, and [Effect.Sequencing](../library/Effect.Sequencing.md) holds the classes.

| Provider | Class | `bind` | `pure` | Selection |
| --- | --- | --- | --- | --- |
| direct | `DirectSequencing` | `a -> (a ->{ef} b) ->{ef} b` | `a -> a` | the default |
| result | `ResultSequencing e` | `Result e a -> (a ->{ef} Result e b) ->{ef} Result e b` | `a -> Result e a` | `using (seq :: ResultSequencing e)` |
| option | `OptionSequencing` | `Option a -> (a ->{ef} Option b) ->{ef} Option b` | `a -> Option a` | `using (seq :: OptionSequencing)` |

The direct provider's instance is `Effect.Sequencing.SeqDirect`, an implicit instance of `DirectSequencing`, and its `pure` is the identity. Direct sequencing adds no container: a `do` block in a definition with no provider clause evaluates its statements in order and yields the last one's value.

The result provider unwraps `Ok` and short-circuits on `Err`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Problem where
    BadCount :: Problem

  positive :: Int -> Result Problem Int
  positive n = if n > 0 then Ok n else Err BadCount

  compute :: using (seq :: ResultSequencing Problem) => Int -> Result Problem Int
  compute n = do
    value <- positive n
    pure (value + 1)

  main :: Int
  main = case compute 4 of
    Ok n -> case compute 0 of
      Ok _ -> 2
      Err BadCount -> if n == 5 then 0 else 1
    Err _ -> 3
```

`compute 4` is `Ok 5` and `compute 0` is `Err BadCount`, so the program exits `0`. The `using` clause selects the provider: `ResultSequencing Problem` fixes the error type, `bind` stops at the first `Err`, and `pure` wraps a value in `Ok`.

The option provider keeps the same shape with `Some` and `None`: `OptionSequencing.pure` wraps a value in `Some`, and `OptionSequencing.bind` stops at the first `None`. `import Effect.Sequencing` and `using (seq :: OptionSequencing)` select it, and nothing else about the block changes. Selecting a provider does not change the surface syntax; `<-`, `let`, and `pure` keep their spelling, and the class the definition names decides what they mean.

# One provider per definition

A definition selects exactly one provider. Naming two makes every `<-` ambiguous:

<!-- cinch:reject contains="two active sequencing providers" -->
```cinch
module Main where
  import Effect.Sequencing

  compute :: using (r :: ResultSequencing Int, o :: OptionSequencing) => Int -> Result Int Int
  compute n = do
    a <- Ok n
    b <- Some (a + 1)
    pure b

  main :: Int
  main = 0
```

```text
cinch: 5:15: two active sequencing providers in one selection scope are ambiguous (ResultSequencing Int, OptionSequencing); bind one explicitly (in Main.cinch)
```

Omitting the provider clause does not leave the choice open; it selects the direct provider. A `Result`-shaped block under the direct provider binds the `Result` itself rather than its payload, so arithmetic on the bound name fails with `integer literal cannot have type Result Int Int`. If two operations use different containers, convert explicitly in a helper and keep one provider per definition. Container-polymorphic library code uses the `Category.*` classes instead of `do`; the two mechanisms do not interact.

# The current handler boundary

<span class="support-tag support-tag--planned">Coming soon</span> **Operations dispatched through a called computation.** The bootstrap handler calculus dispatches operations from the handled expression itself. If the handled expression calls an ordinary function that performs the operation, the program is refused:

<!-- cinch:reject contains="is performed through a called computation" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  answer :: Unit ->{Ask} Int
  answer u = ask ()

  main :: Int
  main =
    let value = handle answer () with (handler Ask where
          return v -> v
          ask unit resume k -> k 20)
    in value
```

```text
cinch: 10:40: the handled effect {Ask} is performed through a called computation; the bootstrap handler calculus dispatches operations only from the handled computation's own expression (in Main.cinch)
```

What works today is an operation in the handled expression's own syntax, including inside `case` alternatives and operator applications in that expression. What does not work is reaching the operation through a helper call, however many steps away it is. Keep the operation in the handled expression, or inline the helper's body, until the extension lands. This is a compiler restriction rather than a law of algebraic effects.

Handling an effect does not erase unrelated effects. A handled expression may still perform `IO r` or `Diverge`, and those stay in the result's row unless another enclosing handler accounts for them.

# What the compiler enforces

| Rule | Diagnostic |
| --- | --- |
| an operation's row carries its effect's label | `effect operation ask must perform its effect's label {Ask}` |
| a handler has a `return` clause | ``a handler needs a `return p -> e` clause`` |
| each clause takes its arguments, then `resume`, then the continuation | a renamed `resume` position is an ill-formed clause |
| `k` is used at most once | `multiplicity %aff: k must be used at most once` |
| one sequencing provider per definition | `two active sequencing providers in one selection scope are ambiguous` |
| operations dispatch from the handled expression itself | `is performed through a called computation` |

# Diagnostics you will meet

| You wrote | Message |
| --- | --- |
| an operation without its label | `cinch: 3:5: effect operation ask must perform its effect's label {Ask} (in Main.cinch)` |
| a handler without `return` | ``cinch: Main.cinch: 7:37: a handler needs a `return p -> e` clause`` |
| two resumptions | `cinch: 9:27: multiplicity %aff: k must be used at most once, but is used 2 time(s) (in Main.cinch)` |
| an operation behind a call | `cinch: 10:40: the handled effect {Ask} is performed through a called computation; ... (in Main.cinch)` |
| two providers in one definition | `cinch: 5:15: two active sequencing providers in one selection scope are ambiguous (ResultSequencing Int, OptionSequencing); bind one explicitly (in Main.cinch)` |

An effect that reaches a definition with no arrow has nowhere to go:

```text
cinch: 6:3: effect rows do not match: {} vs {Ask | ?r894} (a nullary definition must be pure in the bootstrap slice) (in Main.cinch)
```

The message names the rule in force: a definition with no arrow is pure, so an operation it performs has to be handled inside it. Variable identities in these messages are fresh per run.

# Exercises

1. Change the first program's `once` handler to resume with `21`. What are the two values, and what changes in the output?
2. Write an effect `Count` with one operation `tick :: Unit ->{Count} Int`, and explain why a clause that returns a counter value without resuming reports only the first tick.

<details><summary>Answer to exercise 1</summary>

Both values become `42`, and the program prints `42 42` instead of `42 7`. The multiplication runs in both handlers, because both call `k`. The difference between the two clauses was never the value passed to `k`; it was whether `k` was called at all.

</details>

<details><summary>Answer to exercise 2</summary>

`tick unit resume k -> k n` runs the rest of the handled computation and returns that rest's value, so a count is threaded through the resumption rather than accumulated in the clause. A clause that returns a number without resuming abandons the continuation, which is why it can only report the first tick.

</details>

Continue with [chapter 12](12-ownership-and-regions.md), where a resumption's at-most-once rule generalises to multiplicities on every binding.
