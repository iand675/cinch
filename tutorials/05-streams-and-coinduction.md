# Tutorial 5 Streams and coinduction

Source: <https://chatgpt.com/space/page_7af778a2dd6881919c77d929cd4d8cb8>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn to distinguish productive infinite data from computations that may search forever. Prerequisite: Tutorials 1 through 4. Guarded syntax is provisional; the exact clock or size discipline remains an open foundation choice.

# Define a productive stream

```haskell
codata Stream (a :: Type) where
  head :: a
  tail :: Later (Stream a)

corec from :: Nat -> Stream Nat
corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }
```

Producing the current head takes finite work. The recursive call is protected by `delay`, so requesting a finite prefix does not evaluate the entire infinite stream. `Later` is checked guarded delay, not a way to label arbitrary nontermination productive.

# Map without losing productivity

```haskell
corec map :: forall a b. (a -> b) -> Stream a -> Stream b
corec map f xs = Stream
  { head = f xs.head
  , tail = delay (\() -> map f (force xs.tail)) }
```

The callback is total and pure. The use of `force` occurs under the permitted delayed observation. A callback that diverges on a head cannot meet this type.

# Prove a fusion law by bisimulation

To establish `map(f, map(g, xs)) ≈ map(f ∘ g, xs)`, relate each pair with that form. Both heads reduce to `f(g(xs.head))`. Their tails have the same relationship after one permitted observation.

This establishes stream bisimilarity. The optimizer must use the associated congruence theorem for permitted contexts; the language does not automatically identify every bisimulation proof with definitional equality.

# See why filter differs

```haskell
filter (\x -> x % 2 == 0) (repeatForever 1)
```

No output element exists. A function accepting arbitrary predicates therefore cannot promise `Stream a`. It can return a `Process a` whose next output may require unbounded search.

For productive filtering, supply computational evidence sufficient to find the next match after each output. One strong approach is a total search operation returning a finite offset, proof of a match there, and proof that earlier elements were rejected. Another is a certified search driven by a retained bound. A merely erased classical existence assertion does not supply executable search.

Knowing a match occurs within each window can establish productivity, but implementing exact filtering must still preserve all matching elements and their order.

# Exercise and answer

Does a state machine with a total `step : State -> SkipOrYield` necessarily yield? No. It can return `Skip` forever. Productivity concerns progress to observations, not just termination of individual internal steps.

Next: Tutorial 6 states temporal properties of services rather than data streams.

[Previous tutorial](04-modules-and-laws.md) · [Next tutorial](06-processes-and-temporal-contracts.md) · [Documentation overview](../overview.md)

