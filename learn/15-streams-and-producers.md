# 15. Productive streams and partial producers

An infinite sequence cannot be evaluated completely, because no finite amount of work produces all of it. Cinch separates two claims about such a value: a productive `Stream` promises that any observation arrives after finitely many steps, and a `Producer` is the type for a search that may not answer at all. The difference is checked, so a filter that might loop cannot be passed off as a stream.

Prerequisites: [recursion and collections](05-recursion-and-collections.md) for the termination story behind `Diverge`, and [ownership and borrowing](12-ownership-and-regions.md) for the usage multiplicities that `Later` obeys. Explicit `delay` and memo cells are in [explicit laziness](16-explicit-laziness.md), and the guarded proof surface is in [explicit laziness, memoization, and codata](../reference/11-laziness-and-codata.md).

# Declare observations, not constructors

A `codata` declaration lists the observations its values answer, and construction uses the record form. A `corec` definition must produce one observation layer per step of recursion.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  nth :: Nat -> Stream Nat -> Nat
  nth k s = if k == 0 then s.head else nth (k - 1) (force s.tail)

  main :: Int
  main =
    if nth 0 (from 5) == 5 then (if nth 3 (from 5) == 8 then 0 else 1) else 2
```

- `codata Stream (a :: Type) where head :: a` declares an observation, so `s.head` is a projection, and a codata value is constructed with the record form `Stream { head = e, tail = ... }`.
- `tail :: Later (Stream a)` defers the next layer behind the guarded modality `Later`. `force s.tail` produces that next `Stream`, and the recursive call inside it runs at that moment.
- `corec` marks a definition whose recursion must be productive: every recursive call must sit under a `delay` that flows into a `Later`-typed observation. `from` builds one layer, then hands the continuation to `delay`.
- `nth` is an ordinary definition, not a `corec` one. It consumes a finite prefix, so it terminates on the descent of `k`, and `k - 1` inside the `k == 0` else branch carries the subtraction evidence that [chapter 13](13-refinements-and-safe-operations.md) describes.
- A codata value is observed, never destructured. `case s of Stream { head = h } -> h` is refused with ``a codata value is observed, not destructured: use `.field` projection (codata has no constructor patterns)``.

# Guarded production is checked on the source

The checker accepts a recursive call in a `corec` body only when it sits in a `delay` that lands in a `Later` observation. A definition with no observation layer at all is the clearest failure, and it is the one that would otherwise look like a total stream.

<!-- cinch:reject contains="unguarded recursive call to `loop`" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec loop :: Stream Nat
  corec loop = loop

  main :: Int
  main = 0
```

The report is ``unguarded recursive call to `loop`: a `corec` definition must place every recursive call under `delay` in a `Later`-typed observation``. Three other shapes fail for related reasons: a recursive call in the head, as in `corec bad n = Stream { head = bad n, tail = ... }`, is unguarded and named the same way; a `corec` body that builds nothing is refused with ``a `corec` body must build its codata type at every step (an observation layer per recursion level)``; and a `corec` that delegates to an ordinary helper which builds the layer is refused, because the helper hides the construction the guard check needs to see.

The guard also constrains what a `delay` may contain. A `delay` that produces a `Later` must be pure, so an effectful or diverging body belongs in a `Thunk {e}` instead.

# Later is not a convenient thunk

`Later a` is a guarded observation with no effect row, and `Thunk {e} a` is a deferred computation with one. They do not convert.

<!-- cinch:reject contains="Later and Thunk are distinct" -->
```cinch
module Main where
  run :: Later Int -> Int
  run t = force t

  leak :: forall e. Thunk {e} Int -> Int
  leak t = run t

  main :: Int
  main = 0
```

The message is `type mismatch: expected Thunk {e#844} Int, got Later Int (Later and Thunk are distinct types; a Later is a guarded observation, not a deferred computation)`, where the `#844` suffix is a freshness index on the variable. The reverse direction is refused with the same sentence in the other order.

`Later` also obeys the multiplicity rules of [chapter 12](12-ownership-and-regions.md). Forcing consumes it, so `dup t = force t + force t` is rejected with `multiplicity %aff: t must be used at most once, but is used 2 time(s)`, and a `%many` annotation is rejected on the same grounds:

```text
cinch: 2:8: a Later-typed parameter must be used at most once: %many would give Later unrestricted-thunk semantics, which the laziness specification forbids (annotate %aff or %1)
```

# A producer that may not answer

An observation layer per step is easy to write for a stream of naturals and impossible for a stream of the elements that satisfy a predicate, since an infinite input can contain no such element. Writing the filter as `corec` is refused:

<!-- cinch:reject contains="unguarded recursive call to `filter`" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  corec filter :: (Nat -> Bool) -> Stream Nat -> Stream Nat
  corec filter p xs = if p xs.head
    then Stream { head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }
    else filter p (force xs.tail)

  main :: Int
  main = 0
```

The skip branch recurses without producing anything, so `filter` is reported as ``unguarded recursive call to `filter``` even though its productive branch looks conventional. The honest form keeps the same code and changes the type: the search's partiality moves into the observation, and the definition becomes an ordinary one carrying `{Diverge}`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  codata PStream (a :: Type) where
    head :: a
    tail :: Thunk {Diverge} (PStream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  filter :: (Nat -> Bool) -> Stream Nat ->{Diverge} PStream Nat
  filter p xs = if p xs.head
    then PStream { head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }
    else filter p (force xs.tail)

  main :: Int
  main = 0
```

`PStream` puts a `Thunk {Diverge}` in the `tail` observation instead of a `Later`, so the next layer may take unbounded time. `filter` is accepted because it no longer promises productivity. The price is that the loss travels: a pure definition cannot consume the result, and a `main` that reads `(filter (\x -> x > 1) (from 0)).head` is refused with `effect rows do not match: {} vs {Diverge | ?r906} (a nullary definition must be pure in the bootstrap slice)`. Consuming a partial producer requires a caller that admits `Diverge` in its own row.

# Converting production into consumption

Every consumer fixes how much of a producer it will inspect. For codata the consumer is a projection loop such as `nth`, which consumes one `Later` per step. For the step form the library supplies the fold, and the producer is a state machine with three steps.

| Constructor | Meaning |
| --- | --- |
| `Done` | The producer has finished |
| `Skip s` | Nothing to yield from this state |
| `Yield a s` | A value, plus the next state |

`Producer s a` wraps a step function with its initial state, `vec-stream` builds one over a borrowed vector, and `prod-fold` runs it to a result.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Producer
  import Algebra.Arithmetic

  total :: forall n region. Ref region (Vec n Nat) -> Nat
  total xs = prod-fold 0 (\acc x -> acc + x) (vec-stream xs)

  main :: Int
  main = if total (borrow (Cons 1 (Cons 2 (Cons 3 Nil)))) == 6 then 0 else 1
```

`prod-fold` is the consumer: it applies the step function until `Done`, accumulating `Yield` values and following `Skip` states. `prod-map` and `prod-take` wrap one producer in another, so a pipeline is composed first and consumed once by `prod-fold`. The step form does not claim anything about how many `Skip` steps precede a `Yield`, which is exactly the freedom a partial search needs.

A `Producer` is not a `Stream`. Converting between them is not a type ascription, because the producer side carries no guard certificate. Use the step form when the number of steps per result is unbounded, and codata when each observation is guaranteed.

# What the compiler enforces

- A `corec` body must build its codata type at every step, and every recursive call must sit under a `delay` in a `Later`-typed observation.
- A `delay` that produces a `Later` must have an empty effect row, so effectful or diverging work goes into a `Thunk {e}`.
- `Later` and `Thunk {e}` are distinct types and never convert, in either direction.
- A `Later`-typed parameter is used at most once: `%many` is refused, and so is forcing it twice.
- Codata values are observed by projection. Pattern matching on one is refused, and a codata value is never `Copy`, which is why a `~` memo cell cannot cache a stream.
- Productivity is certified only by `corec`. An ordinary definition that produces codata without a guard must record the loss in its type as `{Diverge}`.

<span class="support-tag support-tag--planned">Coming soon</span> **coinductive equality.** Comparing two streams as equal objects needs a bisimulation proof, which relates corresponding observations under a guarded rule. That machinery is not available: two streams can be compared observation by observation for a finite prefix, or by `refl` when both sides are the same term, and no general stream equality is checked.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `corec loop = loop` | ``unguarded recursive call to `loop`: a `corec` definition must place every recursive call under `delay` in a `Later`-typed observation`` |
| A `corec` body that calls a helper | ``a `corec` body must build its codata type at every step (an observation layer per recursion level)`` |
| `delay (\() -> loop n)` at `Later Int` | ``a `delay` building a `Later` must be pure: `Later` is a logical modality with no effect row`` |
| `Thunk {e} Int` used at `Later Int` | `Later and Thunk are distinct types; a Later is a guarded observation, not a deferred computation` |
| `dup t = force t + force t` | `multiplicity %aff: t must be used at most once, but is used 2 time(s)` |
| Reading a `{Diverge}` producer from a pure definition | ``effect rows do not match: {} vs {Diverge \| ?r906} (a nullary definition must be pure in the bootstrap slice)`` |

The `unguarded recursive call` report and the `must build its codata type` report point at different parts of one rule. The first finds a recursive call in a position the guard does not cover. The second finds a body with no observation layer, where no position could be covered.

# Exercises

**Exercise 1.** `corec repeat :: Nat -> Stream Nat` should produce the same value forever. Write the observation layer, and say what makes it productive.

<details><summary>Answer</summary>

`corec repeat n = Stream { head = n, tail = delay (\() -> repeat n) }`. Each step produces a `head` immediately and defers the recursive call behind `delay` in the `Later` observation, so the first observation is available after one layer. The variant `corec repeat n = repeat n` is refused as an unguarded recursive call, because it produces no observation layer.

</details>

**Exercise 2.** Why is `filter` accepted at `Stream Nat ->{Diverge} PStream Nat` but refused at `Stream Nat ->{Diverge} Stream Nat`?

<details><summary>Answer</summary>

The return type carries the claim. `Stream` promises an observation layer per step, and the skip branch of `filter` may take unbounded steps before it produces one, so the guard check refuses the definition when the result is a `Stream`, and the `{Diverge}` row cannot buy the claim back: a `delay` building a `Later` must be pure, so a diverging delay cannot stand in for a guarded one. `PStream` types its next observation as `Thunk {Diverge} (PStream a)`, which is where the unbounded search belongs, so the same branches check. The cost is visible at the use site, where only a definition that admits `Diverge` may consume the result.

</details>

Next: [delay work and memoize deliberately](16-explicit-laziness.md).
