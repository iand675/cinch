# Explicit laziness, memoization, and codata

Cinch is **strict and call-by-value**, evaluating arguments left to right. Deferred computation is never implicit; when you want laziness, sharing, or infinite structures, you choose among three distinct, non-interchangeable constructs:

| Construct | Type | Evaluation & sharing | Environment lifetime |
| --- | --- | --- | --- |
| `delay (\() -> e)`, `force t` (`%aff` / `%1`) | `Thunk {e} a` | Deferred; unshared; forcing consumes the thunk | Freed on force or when dropped |
| `Thunk {e} a %many` | `Thunk {e} a` | Deferred; unshared; each `force` re-evaluates `e` | Freed with each copy |
| `~name = term` in `let` or `where` | `Lazy {e} r a` | Memoized; evaluated at most once per memo group | Released at region teardown (block exit) |
| Codata observation of type `Later a` | `Later a` | Guarded pure observation; unshared; takes no effect row | No hidden cache; guards one productive step |

`Thunk`, `Lazy`, and `Later` are separate type formers that never unify with one another.

# Thunks

`delay` wraps a nullary closure into a `Thunk {e} a` without running it; `force` runs the body and performs its exact effect row `{e}`:

```text
Thunk :: {e} -> Type -> Type
delay :: forall a e. (() ->{|e} a) -> Thunk {e} a
force :: forall a e. Thunk {e} a ->{|e} a
```

Because the effect row is part of `Thunk {e} a`, a deferred effect or divergence (`Thunk {Diverge} a`) remains visible in the type and cannot be smuggled into a pure signature.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  expensive :: Nat -> Nat
  expensive n = nat-mul n n

  main :: Int
  main =
    let t = delay (\() -> expensive 6) in
    if force t == 36 then 0 else 1
```

## Multiplicity: affine by default, recomputing at `%many`

An unannotated `Thunk` binding is **affine (`%aff`)**: it may be forced at most once or dropped unused (dropping reclaims its captures automatically under normal ownership rules). Use `%1` when every path must force it. Forcing an affine thunk twice is rejected:

<!-- cinch:reject contains="at most once" -->
```cinch
module Main where
  main :: Int
  main =
    let t = delay (\() -> nat-add 1 1) in
    nat-to-int (nat-add (force t) (force t))
```

Annotating a `Thunk` parameter as `%many` permits forcing it multiple times, **re-executing the body on every force**. The compiler never silently converts a `%many` thunk into a memoized cell, because memoization requires a region lifetime. Below, `twice` increments the log byte twice:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  twice :: Thunk {} Nat %many -> Nat
  twice t = nat-add (force t) (force t)

  main :: Int
  main =
    let log = bytes-new 1 in
    let bump m = bytes-set log 0 (nat-rem (nat-add (bytes-get log 0) m) 256) in
    let _v = twice (delay (\() -> let _b = bump 1 in nat-add 20 1)) in
    if nat-to-int (bytes-get log 0) == 2 then 0 else 1
```

Deferring a diverging computation puts `Diverge` in the thunk's effect row:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  spin :: forall a. a ->{Diverge} a
  spin x = spin x

  deferred :: forall a. a -> Thunk {Diverge} a
  deferred x = delay (\() -> spin x)

  main :: Int
  main = 0
```

# Memo bindings (`~name = term`)

A `~name = term` binding inside a `let` or `where` block creates a **memoized cell** scoped to that block. All `~` bindings in the same block belong to one **memo group** sharing a fresh region `r` and effect label `Eval r`. A cell is not evaluated on block entry; it evaluates at most once on first demand and caches its result until the block exits.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  tally :: Nat -> Nat -> Nat -> Nat
  tally a b c = nat-add a (nat-add k k)
    where ~k = nat-add b c

  main :: Int
  main = if tally 1 2 3 == 11 then 0 else 1
```

Referencing `k` bare elaborates to `copyOut (demand k)`: `demand` performs `Eval r`, evaluates the cell if uninitialized, and returns a `Ref r` borrow of the cached value; `copyOut` copies out an owned value (requiring `Copy` evidence). Referring to `k` multiple times evaluates the body only once:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let log = bytes-new 1 in
    let bump m = bytes-set log 0 (nat-rem (nat-add (bytes-get log 0) m) 256) in
    let ~k = let _b = bump 1 in nat-add 20 1 in
    let _v = nat-add k k in
    if nat-to-int (bytes-get log 0) == 1 then 0 else 1
```

You can also write `demand` and `copyOut` explicitly:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let ~v = nat-add 20 1 in
    if copyOut (demand v) == 21 then 0 else 1
```

# Memo regions and `Eval r`

Every memo group introduces a region `r` and an effect `Eval r` discharged when the block ends. A memo cell's body is forbidden from containing `Eval r` for its own group, preventing cyclic self-demands (black holes) statically:

<!-- cinch:reject contains="cannot demand its own group" -->
```cinch
module Main where
  loop :: Int -> Int
  loop n = x
    where ~x = x + 1

  main :: Int
  main = loop 0
```

Because this check applies to the whole group, one `~` binding cannot demand a sibling `~` binding in the same `let`/`where` block. Place dependent memo bindings in nested `let` blocks so each receives its own region:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let ~a = 5 * 2 in
    let ~b = a + 1 in
    if a + b == 21 then 0 else 1
```

## Region confinement and `Copy` requirements

Memo cells obey two additional static rules:

- **No region escape.** A memo cell cannot be captured inside a thunk or lambda that may outlive its owning block:

<!-- cinch:reject contains="region escape" -->
```cinch
module Main where
  bad :: Int -> Thunk {} Int
  bad n = delay (\() -> y)
    where ~y = n * 2

  main :: Int
  main = force (bad 3)
```

- **Cached values must be `Copy`.** Because bare reads copy the cached value out of the cell's borrow, non-`Copy` types such as `codata` streams cannot be memoized:

<!-- cinch:reject contains="no Copy evidence" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Int -> Stream Int
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  peek :: Int -> Int
  peek n = s.head
    where ~s = from n

  main :: Int
  main = peek 0
```

## Known memo limitations

- **First-class `Lazy {e} r a` values.** Although `Lazy {e} r a` is accepted in type signatures (requiring `Eval r` on arrows that `demand` it), naming a `~` binding in an expression always demands its value rather than passing the cell handle. Write helper functions over the demanded value instead.
- **Strict sibling reading a memo cell in the same block.** A strict binding in the same `let` or `where` block cannot currently read a sibling `~` cell (nor does the checker yet catch a cycle routed through a strict sibling):

```text
-- Not currently supported in a single block:
let ~k = 1 + 1
    s = k + 1
in s
```

  **Workaround:** move the strict binding into a nested `let` block (`let ~k = 1 + 1 in let s = k + 1 in s`), or demand `k` directly in the body expression. As defense in depth, re-entering a cell that is already under evaluation triggers a runtime `BlackHole` fault (exit status `70`).

# Codata: observations and `Later`

A `codata` declaration defines a coinductive type by its **observations** (projections), rather than by finite constructors. A codata value is constructed with `TypeName { field = expr, ... }`, where each observation expression is deferred until projected with `.field`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  main :: Int
  main =
    let s = from 10 in
    if s.head == 10 && (force s.tail).head == 11 then 0 else 1
```

Codata values can only be observed via `.field` and constructed via `TypeName { ... }`. Pattern matching and positional constructor application are rejected:

<!-- cinch:reject contains="observed, not destructured" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  peek :: Stream Int -> Int
  peek s = case s of
    Stream { head = h } -> h

  main :: Int
  main = 0
```

## The `Later` modality

`Later a` marks a guarded recursive observation step in a productive codata definition. Unlike `Thunk {e} a`:

- `Later` takes **one type argument and no effect row** (`Later {IO} Int` is rejected).
- `Later` is **strictly affine**—annotating a `Later` parameter `%many` or forcing a `Later` twice is rejected.
- `Later` and `Thunk` never unify.
- A `delay` producing a `Later` **must be pure**; attempting to hide `Diverge` or any other effect inside `Later` is rejected:

<!-- cinch:reject contains="must be pure" -->
```cinch
module Main where
  spin :: forall a. a ->{Diverge} a
  spin x = spin x

  f :: Int -> Later Int
  f n = delay (\() -> spin n)

  main :: Int
  main = 0
```

# Productive corecursion (`corec`)

Marking a definition `corec` asserts that it is **productive**: every finite prefix of observations terminates in finite time. The checker enforces two syntactic guardedness rules on every `corec` body:

1. **Guarded recursive calls.** Every recursive call in the definition's recursion group must appear directly inside a `delay` whose result populates a `Later`-typed observation.
2. **Explicit observation layer.** Every step of the `corec` body must directly construct the codata value (`TypeName { ... }`) rather than delegating construction to an unguarded helper.

An unguarded self-call violates rule 1 (as does placing a recursive call inside a non-`Later` observation such as `head = bad n`):

<!-- cinch:reject contains="unguarded recursive call to `loop`" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec loop :: Stream Int
  corec loop = loop

  main :: Int
  main = 0
```

Delegating construction to an ordinary helper violates rule 2:

<!-- cinch:reject contains="must build its codata type at every step" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  mk :: Int -> Stream Int
  mk n = Stream { head = n, tail = delay (\() -> mk n) }

  corec build :: Int -> Stream Int
  corec build n = mk n

  main :: Int
  main = 0
```

When each step computes one layer of observations and places the recursive call under `delay` in `tail :: Later (Stream b)`, `corec` accepts the definition:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  corec from :: Nat -> Stream Nat
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  corec map :: forall a b. (a -> b) -> Stream a -> Stream b
  corec map f xs = Stream { head = f xs.head, tail = delay (\() -> map f (force xs.tail)) }

  main :: Int
  main =
    let s = map (\x -> x * 10) (from 1) in
    let s1 = force s.tail in
    let s2 = force s1.tail in
    if s.head == 10 && s1.head == 20 && s2.head == 30 then 0 else 1
```

# Unproductive operations and partial streams

An arbitrary filter over an infinite stream can search forever if no element matches the predicate. The checker rejects an unguarded stream filter unless its loss of productivity is declared with `Diverge`:

<!-- cinch:reject contains="not productive, so its type must reflect the loss of productivity" -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  filter :: (Int -> Bool) -> Stream Int -> Stream Int
  filter p xs = if p xs.head
    then Stream { head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }
    else filter p (force xs.tail)

  main :: Int
  main = 0
```

To express a potentially diverging stream, replace `Later` with `Thunk {Diverge}` in the codata type (`PStream`) and declare `{Diverge}` on the filter arrow:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata Stream (a :: Type) where
    head :: a
    tail :: Later (Stream a)

  codata PStream (a :: Type) where
    head :: a
    tail :: Thunk {Diverge} (PStream a)

  corec from :: Int -> Stream Int
  corec from n = Stream { head = n, tail = delay (\() -> from (n + 1)) }

  filter :: (Int -> Bool) -> Stream Int ->{Diverge} PStream Int
  filter p xs = if p xs.head
    then PStream { head = xs.head, tail = delay (\() -> filter p (force xs.tail)) }
    else filter p (force xs.tail)

  main :: Int
  main = 0
```

Because `filter` carries `{Diverge}`, calling or forcing it requires an enclosing computation that permits `Diverge` (not a pure `main :: Int`).

For finite or bounded stream pipelines, prefer `Data.Producer` (`Done`, `Skip state`, `Yield value state`), which avoids productivity obligations altogether; see [streams and producers](../learn/15-streams-and-producers.md).

# Current boundaries

| Construct | Status |
| --- | --- |
| Irrefutable `~pattern` function parameters | Not supported. |
| Implicitly lazy data fields (without an explicit `Lazy {e} r a` and bound `r`) | Not supported; all record and constructor fields are strict. |
| `%many` on `Later`, or an effect row on `Later` | Rejected; use `Thunk {e} a` for effectful or repeatable deferral. |
| First-class `Lazy {e} r a` cell values | Signature accepted, but bare names always demand; pass demanded values instead. |
| Strict sibling binding reading a `~` cell in the same block | Known limitation; place the strict binding in a nested `let` or demand in the body. |
| Memoizing a non-`Copy` or codata value | Rejected (`no Copy evidence`). |
| Explicit `destroy` primitive for thunks | Not provided; unforced affine thunks are released automatically by ownership rules. |
| Coinductive bisimulation as built-in conversion | Not implemented; rewrites over streams require explicit evidence ([Chapter 12](12-rewrites-and-optimization.md)). |

Normative details are in the [laziness and reclamation specification](../specifications/lazy-computation-and-reclamation-specification.md) and the [codata guardedness design note](../design/codata-guardedness.md).
