# 10. Effects, capabilities, and a program that prints

Chapter 9 showed how a definition states the vocabulary it needs. Output needs a second kind of statement as well: it must say that writing is something the computation does. In Cinch those are separate declarations. An effect row says what a computation may do. A capability is a value that grants access to a particular runtime. I/O needs both, plus a `using` clause that names the capability in the type of every helper. This chapter separates the three, writes a program that prints, and shows the messages you get when one piece is missing.

Run each complete program with `$C check Main.cinch` for typechecking alone, and `$C run Main.cinch` to build and execute it.

# Before you start

You need function types from [chapter 3](03-functions.md), `case` and `List` from [chapters 4](04-data-and-matching.md) and [5](05-recursion-and-collections.md), and imports from [chapter 8](08-modules.md). You also need `clang` on `PATH` for `run`; `check` does not link.

# Three pieces, kept distinct

| Piece | Where it appears | What it is | Example |
| --- | --- | --- | --- |
| effect row | the arrow of a type | permission to perform a named effect | `->{IO r}` |
| capability | a value bound by `with` or taken as a parameter | a runtime handle of type `Io r` | `newIO ()` |
| `using` context | the constraint list of a type | a dictionary the body may use | `using (io :: Io r)` |

The row alone does not give the code a runtime to write to. The capability alone does not let a pure arrow call an effectful operation. A helper that only states one of them fails to check, and the two failures carry different messages.

A pure arrow is `->`. An effectful arrow is `->{IO r}`, which reads as a function that may perform the `IO` effect in region `r`. A row may list several effects, and it may be left open with a tail variable so a higher-order function can pass its callback's effects through. Rows describe permission and possibility; they do not certify that an operation happened or that a computation finishes.

# The two operations this chapter uses

`newIO` has the scheme `forall r. Unit -> Io r`. It takes `Unit`, allocates a runtime capability, and returns it. It is a builtin value with a compiler-known scheme, so there is no module to import for it.

`ioWrite` has the scheme `forall r. using (io :: Io r) => Bytes %many ->{IO r} Unit`. Read the pieces in order: it requires an `Io r` capability, it takes bytes, it may perform `IO r`, and it returns `Unit`. The `%many` on the argument says the bytes may be used any number of times, which is the default for ordinary data.

`ioWrite` takes `Bytes` rather than `String`, because a string is text and the runtime writes bytes. `string-to-bytes` encodes a string as UTF-8, and it adds nothing else: no terminator, no newline, no padding. A `String` argument of `"total 12"` writes eight bytes. Writing a line therefore means encoding the newline explicitly as part of the string.

# A program that prints

<!-- cinch:run exit=0 stdout="total 12\n" -->
```cinch
module Main where
  total :: List Int -> Int
  total xs = case xs of
    Nil -> 0
    Cons x rest -> x + total rest

  report :: forall r. using (io :: Io r) => Int ->{IO r} Unit
  report n = ioWrite (string-to-bytes (string-concat "total " (string-concat (int-to-string n) "\n")))

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in do
    let values = Cons 3 (Cons 4 (Cons 5 Nil))
    let sum = total values
    written <- report sum
    pure 0
```

The run prints `total 12` followed by a newline and exits `0`.

Read `main`'s type first. `Unit ->{IO r} Int` takes one argument and returns the process exit status after possibly performing `IO r`. The compiler supplies the entry argument for you; `u` is that `Unit` and is unused here. The program's status is the value the body returns.

The expression after `in` is a `do` block. `with (io = newIO ()) in ...` evaluates one capability and binds it to the name `io` for the extent of the block, so `ioWrite` inside the block finds a capability in scope.

`report` states both type-level pieces and nothing about the runtime value: `forall r. using (io :: Io r) => Int ->{IO r} Unit`. A caller supplies the capability, which `report` then passes to `ioWrite`. `total` is pure and has no row and no `using` clause, because summing integers reads no runtime state. Keeping the pure part pure is what makes the effect boundary visible in the types.

# Sequence steps with do

In the direct provider, `do` runs its statements in source order and each statement's result is available to the statements after it. Three forms appear above:

| Form | Meaning |
| --- | --- |
| `x <- action` | run `action`, bind its result to `x` for the following statements |
| `let x = expression` | bind a local result without an effect of its own |
| `pure expression` | the block's result, used as the last statement |

`pure` in the direct provider has the scheme `forall a. using (dict :: DirectSequencing) => a -> a`: it is the identity, and the `SeqDirect` instance is implicit, so a direct `do` block needs no `using` clause of its own. This is not a wrapper type. A direct `do` block evaluates as its statements evaluate, and `written <- report sum` binds `Unit`, the result of the write.

The block's effect row is the union of what its statements perform, so `main`'s `->{IO r}` admits the write that `report` performs. Nothing about `do` creates an effect: the row comes from the operations inside it.

A different provider changes what `<-` and `pure` mean while the spelling stays the same. [Chapter 11](11-handlers-and-sequencing.md) covers the `Result` and `Option` providers.

# Exit statuses are the returned number

`main`'s result is the process status. `0` is success, and a nonzero value signals failure to the shell. The status is truncated to the low eight bits, so a `main` returning `300` exits with status `44`.

| `main` returns | Process status |
| --- | --- |
| `0` | 0 |
| `3` | 3 |
| `300` | 44 |

Programs in earlier chapters declare `main :: Int` with no arrow and no `IO`, and they exit with their returned value just the same. Use `Unit ->{IO r} Int` when the program prints, and keep `main :: Int` when it does not. Command line words are read with `process-args` from [Effect.Process](../library/Effect.Process.md), whose scheme requires an `Io r` capability exactly as `ioWrite` does, and `cinch run` forwards every word after the entry file to the program.

The runtime reports exhaustion separately from a program's own status. Running out of heap ends the process with status `73` and a message on stderr, so a nonzero exit from your own `main` and a runtime exhaustion are distinguishable.

# When one piece is missing

A helper that states the row and forgets the capability fails, and the message names the capability it wanted:

<!-- cinch:reject contains="no Io r" -->
```cinch
module Main where
  shout :: forall r. String ->{IO r} Unit
  shout text = ioWrite (string-to-bytes text)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = shout "hi"
    in 0
```

```text
cinch: 3:16: no Io r#844 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause (in Main.cinch)
```

The `#844` is a fresh variable identity that changes between runs. The row was present and had no bearing on this failure: the body called an operation whose scheme requires a capability, and no capability reached it.

The reverse mistake states the capability and forgets the row:

<!-- cinch:reject contains="effect rows do not match" -->
```cinch
module Main where
  shout :: forall r. using (io :: Io r) => String -> Unit
  shout text = ioWrite (string-to-bytes text)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = shout "hi"
    in 0
```

```text
cinch: 3:3: effect rows do not match: {} vs {IO r#845 | ?r894} (in Main.cinch)
```

The declared arrow is `->`, whose row is `{}`. The body's right-hand side performs `IO r` and may perform the open tail `?r894`. The two do not match. Adding `->{IO r}` fixes it, and the `using` clause stays as it is.

A third variant declares the row and has no capability scope at all, because there is no `with`:

<!-- cinch:reject contains="capability in scope" -->
```cinch
module Main where
  main :: Unit ->{IO r} Int
  main u =
    let written = ioWrite (string-to-bytes "hi\n")
    in 0
```

```text
cinch: 4:19: no Io r#844 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause (in Main.cinch)
```

Here the row is declared correctly on `main` and the failure is a missing runtime value. That is the distinction worth keeping: an effect row is checked in the type, a capability is checked in the scope.

# Pass a capability where you need it

A capability is an ordinary value, so you can bind it and hand it over explicitly. In this `main` fragment, supplication with `@{...}` supplies the dictionary at the call instead of taking it from the enclosing `with`:

```cinch
  main :: Unit ->{IO r} Int
  main u =
    let cap = newIO ()
    in with (io = cap) in
      let written = shout @{io = cap} "explicit\n"
      in 0
```

With `shout :: forall r. using (io :: Io r) => String ->{IO r} Unit` in scope, this prints `explicit` and a newline. Nested `with` clauses nest capability scopes, and the innermost binding of the same name is the one the body sees.

# What the compiler enforces

| Rule | Result |
| --- | --- |
| a body that performs `IO r` needs `IO r` in its declared row | `effect rows do not match` |
| a call to `ioWrite` needs an `Io r` capability in scope | `no Io r capability in scope` |
| `main`'s returned number is the process status | the low eight bits of that number |
| `string-to-bytes` encodes UTF-8 and appends nothing | a newline appears only if the string contains one |
| a pure arrow stays pure | calling an effectful function from `->` fails to check |

An effect row is permission and possibility, not evidence. A row that names `IO r` does not make a computation terminate, and a capability does not record what was written.

# Diagnostics you will meet

An effectful body under a pure arrow, where the row is `{}` and the body performs `IO r`:

```text
cinch: 3:3: effect rows do not match: {} vs {IO r#845 | ?r894} (in Main.cinch)
```

A helper that states the row but not the capability:

```text
cinch: 3:16: no Io r#844 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause (in Main.cinch)
```

An operation performed with no capability scope at all:

```text
cinch: 4:19: no Io r#844 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause (in Main.cinch)
```

The second and third messages are identical and differ only in position, because the checker's state is the same in both: it needs an `Io r` and cannot find one. The position tells you which call site could not be served.

# Exercises

1. Change the printing program to print `total 12` with no trailing newline, then to print it twice on one line separated by a space. Keep exit status `0`.
2. Add a `banner :: forall r. using (io :: Io r) => String ->{IO r} Unit` that writes the string between two lines of dashes, and call it from `main`.

<details><summary>Answer to exercise 1</summary>

Remove `"\n"` from the concatenation for the first version: `report n = ioWrite (string-to-bytes (string-concat "total " (int-to-string n)))` writes eight bytes and leaves the cursor after `2`. For the second, keep the newline only in the second half: `string-concat "total " (string-concat (int-to-string n) (string-concat " " (string-concat "total " (string-concat (int-to-string n) "\n"))))`. In both cases `main` still returns `0`, and the row and the `using` clause do not change.

</details>

<details><summary>Answer to exercise 2</summary>

The definition is this fragment:

```cinch
  banner :: forall r. using (io :: Io r) => String ->{IO r} Unit
  banner text = do
    opened <- ioWrite (string-to-bytes "-----\n")
    body <- ioWrite (string-to-bytes (string-concat text "\n"))
    closed <- ioWrite (string-to-bytes "-----\n")
    pure ()
```

Three writes in order, each bound to its `Unit` result so the block stays in the direct provider. `()` is the single value of type `Unit`, listed with the other literals in [chapter 2](02-values-and-layout.md). The `using` clause and the row are the ones `ioWrite` requires, and `main` must keep `->{IO r}` because the call happens inside it.

</details>

Continue with [chapter 11](11-handlers-and-sequencing.md), which declares its own operations and decides what they mean.
