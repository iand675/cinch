# 15. Productive streams and partial producers

An infinite sequence cannot be evaluated completely. Instead, each observation must produce a finite answer and delay the rest. Cinch distinguishes a productive `Stream` from a producer that may need to search without a bound before yielding another result.

# Build one layer at a time

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata CountStream where
    head :: Nat
    tail :: Later CountStream

  corec from :: Nat -> CountStream
  corec from n = CountStream {
    head = n,
    tail = delay (\() -> from (n + 1))
  }

  main :: Int
  main =
    let stream = from 5
    in if stream.head == 5 && (force stream.tail).head == 6 then 0 else 1
```

`codata` declares observations. `corec` declares a corecursive definition subject to productivity checking. The current layer provides `head` immediately. The recursive call is guarded by `delay` in a `Later` observation, and forcing that observation advances to the next layer.

`Later` is a guarded modality, not an arbitrary ordinary thunk. The productivity checker needs an observable codata construction at every step. `corec loop = loop` cannot produce a first observation and is rejected. Hiding the construction inside an unrestricted helper is also outside the direct construction discipline enforced today.

# Mapping and filtering have different guarantees

Mapping a total function over a productive stream can preserve productivity: to produce one output, inspect one input and transform it. Filtering for an arbitrary predicate is different. An infinite input may contain no satisfying element, so the next output could require an infinite search.

Use a partial `Producer` result where the operation loses the productivity guarantee. Do not relabel it as a `Stream` just because one test input happens to produce a value quickly. Returning to a productive stream requires the bounded-gap evidence described in the specification and supported constructors.

# Bisimulation

Two streams can have the same observations without being finite data you can compare by reducing their entire representation. Bisimulation relates corresponding heads and tails under the guarded proof rules. The [codata reference](../reference/11-laziness-and-codata.md) separates the supported guarded proof surface from general proof automation that remains planned.

# Exercise and answer

What happens to the guarantee if `from` searches for a prime before constructing each head, using a search whose termination the compiler cannot establish?

<details><summary>Answer</summary>

The productive-stream claim needs evidence for that search, not just a delayed recursive tail. If the head computation is potentially unbounded, account for partiality with the appropriate producer/effect model. A delayed tail does not make an unbounded head productive.

</details>
