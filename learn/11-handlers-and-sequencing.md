# 11. Handle effects and select sequencing

An effect declaration names operations that a computation may request. A handler gives those operations an interpretation. Unlike a capability, which supplies a particular runtime resource, a handler can decide whether and how to continue the handled computation.

# An operation and its handler

<!-- cinch:run exit=0 -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  main :: Int
  main =
    let answer = handle ask () + 1 with (handler Ask where
          return value -> value
          ask u resume k -> k 41)
    in if answer == 42 then 0 else 1
```

`ask ()` requests an integer. Its handler receives a resumption `k`, representing the rest of the handled computation. Calling `k 41` continues with 41 as the operation result, so the addition produces 42. The `return` clause handles normal completion.

A clause may return without resuming; then the rest of the computation does not run. A resumption is one-shot: calling it twice is rejected. If its captured continuation contains linear resources, dropping it can also violate an exactly-once obligation. Handling an operation does not grant permission to duplicate the continuation's resources.

# The current handler boundary

<span class="support-tag support-tag--planned">Coming soon</span> **operations through arbitrary calls.** The Rust implementation handles operations in the handled expression itself. It rejects the case where the operation is performed through a separately called computation. The intended handler model extends across calls; the current diagnostic names that boundary. Keep the operation in the direct handled expression for now. This is a compiler restriction, not a general law of algebraic effects.

A handler clause can perform effects. Those effects remain in the resulting expression's row unless another enclosing handler accounts for them. Handling `Ask` does not erase unrelated I/O or divergence.

# Select a Result provider

Here is a complete short-circuiting calculation:

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
    Ok n -> if n == 5 then 0 else 1
    Err _ -> 2
```

The provider determines what `<-`, `pure`, and `bind` mean in this definition. `ResultSequencing Problem` unwraps an `Ok` and stops on `Err`. It is distinct from declaring a custom algebraic effect, and distinct from propagating `IO r` in an arrow.

A single definition uses one sequencing container. If two operations use different result containers, make the conversion explicit in a helper. Arbitrarily mixing providers inside one definition is not implicit composition.

# Exercise and answer

Change the handled expression to `ask () * 2` and the clause to `k 21`. Then make the clause return `7` without using `k`. What are the two results?

<details><summary>Answer</summary>

The resumed version returns 42. The nonresuming version returns 7: it skips the multiplication because it does not run the continuation. The `return` clause applies when the handled computation completes normally, rather than replacing the handler clause's own result.

</details>
