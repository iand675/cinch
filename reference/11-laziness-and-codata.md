# Explicit laziness, memoization, and codata

Cinch evaluates strictly, left to right. A type that mentions a deferred form does not make any call lazy. Everything on this page is written where it is wanted, and the three mechanisms stay separate: an affine thunk defers one computation, a memo binding shares one evaluation inside a region, and a `Later` observation guards one step of a productive codata value.

| Form | Type | Sharing | When the captured environment dies |
| --- | --- | --- | --- |
| `delay (\() -> e)`, `force t` | `Thunk {e} a` | none; a force consumes the thunk | at the force, or with the last owner |
| the same thunk at a `%many` binder | `Thunk {e} a` | none; each force runs the body again | with each copy |
| `~name = term` in a `let` or `where` block | `Lazy {e} r a` | one evaluation per memo group | at region teardown, which is block end or return |
| a codata observation of type `Later a` | `Later a` | none; it is a guarded observation | not applicable, the guard stores nothing |

`Thunk`, `Later` and `Lazy` are distinct type formers. `Thunk` never unifies with `Later`, and `Later` takes no effect row.

# Thunks

`delay` takes a closure over `Unit` and returns a thunk. Creating the thunk performs nothing. `force` runs the body and yields its result, and the effect row of the force is exactly the row of the body.

```text
Thunk  :: {e} -> Type -> Type
delay  :: forall a e. (() ->{|e} a) -> Thunk {e} a
force  :: forall a e. Thunk {e} a ->{|e} a
```

A thunk typed `Thunk {Diverge} a` is how a deferred computation carries divergence. The row is part of the type, so a pure signature cannot hide a deferred effect or a deferred divergence.

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

# Usage: a force consumes the thunk

An unannotated `Thunk` binder is affine (`%aff`). Forcing it twice uses it twice, and the checker rejects that:

<!-- cinch:reject contains="at most once" -->
```cinch
module Main where
  main :: Int
  main =
    let t = delay (\() -> nat-add 1 1) in
    nat-to-int (nat-add (force t) (force t))
```

`cinch check` reports `cinch: 4:9: multiplicity %aff: t must be used at most once, but is used 2 time(s) (in Main.cinch)`. Write `%1` when every path must force or consume the thunk exactly once, and `%aff` when at most once suffices.

An unused thunk is allowed at `%aff`. There is no `destroy` name in the bootstrap; an unforced thunk is reclaimed by the ordinary ownership rules, which run the destructors of its captures when the last owner goes away.

# Duplicable thunks: `%many`

A `Thunk` binder annotated `%many` is duplicable and unshared. Each force runs the body again, so a `%many` thunk is call-by-later evaluation with recomputation, never a cache. The count below increments a byte inside the thunk body; the program passes only when the body ran twice.

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

The compiler never turns a `%many` thunk into a shared memo cell: that would introduce a lifetime the type does not mention.

A deferred diverging computation needs `Diverge` in the thunk's row, which is what makes it different from a pure total thunk:

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

# Memo bindings: `~name = term`

The memo binding form is `~name = term` inside a `let` block or a `where` block. All `~` bindings of one block form one memo group: one region, one effect label `Eval r`, mutually visible cells. The binding is not evaluated when the block is entered.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  tally :: Nat -> Nat -> Nat -> Nat
  tally a b c = nat-add a (nat-add k k)
    where ~k = nat-add b c

  main :: Int
  main = if tally 1 2 3 == 11 then 0 else 1
```

A bare use of the name is the demanded value. It elaborates to `demand` followed by `copyOut`, so the cached value is copied out and the copy needs `Copy` evidence. Using the same memo name twice, in the same expression, still evaluates the body once. The count below is 1, and the body increments the same byte the `%many` thunk incremented twice above:

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

Write the demand and the copy out explicitly when you want them in view. `demand` performs `Eval r` and returns a borrow of the cached value; `copyOut` turns that borrow into an owned value:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let ~v = nat-add 20 1 in
    if copyOut (demand v) == 21 then 0 else 1
```

# Memo regions and `Eval r`

Each memo group owns a region, and every demand performs that region's effect label. The checker enforces one rule: the evaluation row of a cell body must not contain `Eval r` of its own group. A body that demands its own group is a black hole, and the row is what detects it.

A direct cycle is rejected:

<!-- cinch:reject contains="cannot demand its own group" -->
```cinch
module Main where
  loop :: Int -> Int
  loop n = x
    where ~x = x + 1

  main :: Int
  main = loop 0
```

The diagnostic is `cinch: 4:16: effect row must not contain Eval r#13 (a memo cell cannot demand its own group): this memo binding demands its own memo group (in Main.cinch)`. The generated region number after `r#` differs between compilations.

The rule is conservative in one direction: demanding a different cell of the same group is a dependency, not a cycle, and it is rejected too. Separate the cells into separate blocks, since each block is its own group. This program is accepted and returns 21:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let ~a = 5 * 2 in
    let ~b = a + 1 in
    if a + b == 21 then 0 else 1
```

## What the region rejects

A memo cell is bound to its region, so it cannot leave the block that owns it. Deferring a demand past the region is a capture of a dead region:

<!-- cinch:reject contains="region escape" -->
```cinch
module Main where
  bad :: Int -> Thunk {} Int
  bad n = delay (\() -> y)
    where ~y = n * 2

  main :: Int
  main = force (bad 3)
```

`cinch check` reports `cinch: 3:25: region escape: y is captured in a lambda that may outlive region r#13 (in Main.cinch)`.

A memo cell caches a copyable value. Caching a codata value is refused, because a stream is not `Copy` and an infinite stream is not a memo of its elements:

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

`cinch check` reports ``cinch: 10:12: no Copy evidence for Main.Stream Int (copying needs a Copy type or a `using (copy :: Copy a)` constraint) (in Main.cinch)``.

## The `Lazy` type is not usable as a value yet

A signature may name the cell type. It checks, and it needs `Eval r` on the arrow because `demand` performs it:

```text
hold :: forall r. Lazy {} r Nat ->{Eval r} Nat
hold c = copyOut (demand c)
```

No expression in the bootstrap produces a cell value to pass to that parameter. A `~` binding produces the cell, but naming the binding anywhere in an expression is the demanded value, so a call `hold v` reports a type mismatch rather than passing the cell. The parameter form is accepted and unusable; write the function over the demanded value instead.

## Current limitation: a strict binding that reads a memo cell

A strict binding in the same block that reads a memo cell is refused by the ownership verifier, with an internal message rather than a diagnostic about memo groups:

```text
main =
  let ~k = 1 + 1
      s = k + 1
  in s
```

`cinch run` on that shape reports `cinch: compiler bug: ownership verifier rejected Main.main: borrowed read of v1, which is not alive here`. The same failure appears for the `where` form. Move the demand into the equation body, or put the second binding in a nested block, which is a separate group and works. If you hit this message on a shape that does not fit the pattern here, `CINCH_RC_TRACE=1` on a debug build of the compiler writes the verifier's input and output before the message, which makes a report actionable ([environment variables](25-allocators.md#environment-variables)).

The cycle that the specification rules out through a strict sibling is not caught by the checker today. This program passes `cinch check` and fails at build time with the same verifier message, so treat it as a limitation and not as an accepted form:

```text
f n = s
  where ~k = s + 1
        s = k + 1
```

The runtime still has defense in depth for paths that bypass the static rule: a demand that finds a cell already being evaluated takes the `BlackHole` fault, which exits with status 70 (`runtime/src/abi.rs::cinch_blackhole`, emitted as `__cinch_blackhole`). Well-typed code cannot reach it.

# Codata: observations and `Later`

A `codata` declaration lists observations. Construction uses the observed record form, and each field becomes a closure that runs at observation time.

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

`xs.head` runs one observation. `xs.tail` is a `Later`, and `force` advances to the next layer. Codata has no constructor patterns and no field-by-field destructuring:

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

`cinch check` reports ``cinch: 8:5: a codata value is observed, not destructured: use `.field` projection (codata has no constructor patterns) (in Main.cinch)``. The positional application form is refused the same way: ``cinch: 7:11: a codata value is built only by its observed construction form `Stream { ... }`: its observations are closures run at observation time (in Main.cinch)``.

`Later` is a guarded observation, not a deferred computation. It cannot hold an effect row, and it cannot be unrestricted:

| Attempt | Diagnostic |
| --- | --- |
| `Later {IO} Int` | ``Later must be applied to exactly one type (`Later a`): it is a logical modality and takes no effect row; effectful deferral is `Thunk {e} a` `` |
| a `%many` `Later` binder | `a Later-typed parameter must be used at most once: %many would give Later unrestricted-thunk semantics` |
| forcing one `Later` twice | `multiplicity %aff: t must be used at most once, but is used 2 time(s)` |
| threading a `Thunk` into a `Later` parameter | `Later and Thunk are distinct types; a Later is a guarded observation, not a deferred computation` |

A `Later` body must be pure. Deferring a diverging computation behind a `Later` is refused, and the diagnostic points at the thunk that can carry it:

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

`cinch check` reports ``cinch: 6:9: a `delay` building a `Later` must be pure: `Later` is a logical modality with no effect row (core language specification section 9); defer effectful or diverging computation in a `Thunk {e}` value (in Main.cinch)``.

# The productivity rule for `corec`

A `corec` definition claims that every finite observation appears after finitely many steps. The checker enforces two syntactic conditions on the body of a `corec` definition:

1. Every recursive call of the definition's own recursion group sits inside a `delay` whose result flows to a `Later`-typed observation.
2. The body builds the codata type at every step, so each recursion level produces its observations before the next call.

A self application with no observation layer violates both. The checker names the call:

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

`cinch check` reports ``cinch: 8:9: unguarded recursive call to `loop`: a `corec` definition must place every recursive call under `delay` in a `Later`-typed observation (core language specification section 9) (in Main.cinch)``.

A recursive call in an observation position is still unguarded, because the observation must produce its value in finite time:

```text
corec bad n = Stream { head = bad n, tail = delay (\() -> bad (n + 1)) }
```

`head = bad n` is refused with ``unguarded recursive call to `bad` in a codata observation: an observation must produce its value in finite time, so an unproductive search cannot run here; compute it before construction or defer it in a `Thunk {{Diverge}}` observation``.

Guarding the calls but hiding the construction behind an ordinary helper breaks the second condition. A `corec` definition must construct the codata value itself at each step:

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

`cinch check` reports ``cinch: 11:3: a `corec` body must build its codata type at every step (an observation layer per recursion level); this body does not construct a codata value (core language specification section 9) (in Main.cinch)``.

The same syntactic discipline is what makes `map` over a stream productive. Each step observes the head and defers the tail:

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

# Unrestricted filtering loses productivity

An ordinary definition that returns codata is not certified productive. When its recursion is unguarded, its type must say so with `Diverge`, and marking it `corec` is refused outright:

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

`cinch check` reports ``cinch: 9:10: unguarded recursive call to `filter`: this definition is not productive, so its type must reflect the loss of productivity: declare `Diverge` on its arrows (as in `... ->{Diverge} ...`), or guard the call under `delay` in a `Later` observation (core language specification section 9) (in Main.cinch)``. With `corec filter` instead, the message is ``unguarded recursive call to `filter`: a `corec` definition must place every recursive call under `delay` in a `Later`-typed observation``.

The honest form moves the loss of productivity into the observation types. A `PStream` whose tail is a `Thunk {Diverge}` can be built by a filter declared `->{Diverge}`, and the search runs at call time:

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

That definition cannot be consumed from a pure context. `main` is nullary here, so forcing the filtered stream from it is refused: `cinch: 19:3: effect rows do not match: {} vs {Diverge | ?r906} (a nullary definition must be pure in the bootstrap slice) (in Main.cinch)`.

For finite or bounded pipelines prefer the producer vocabulary of `Data.Producer`, whose step form is `Done`, `Skip state` and `Yield value state`, and which has no productivity claim to break. See [streams and producers](../learn/15-streams-and-producers.md).

# What is not supported

| Construct | Status |
| --- | --- |
| `~pattern` in a function argument (Haskell's irrefutable pattern) | refused: ``irrefutable (`~`) patterns are not supported in the bootstrap slice`` |
| a lazy field that hides its region | no such form: a field is strict unless its type is `Lazy {e} r a` with `r` bound (`forall r. Lazy {} r Nat -> Box` checks), and an unbound region reports `unknown region variable r` |
| `%many` on a `Later` | refused as unrestricted-thunk semantics |
| a `Later` with an effect row | refused; use `Thunk {e} a` |
| a memo cell as a value passed to a `Lazy` parameter | no producing expression, so the parameter is unusable |
| a memo cell cached in a value that outlives its region | refused as a region escape |
| memoizing a codata value | refused for want of `Copy` evidence |
| `destroy` | no such name in the bootstrap |
| bisimulation as a relation or as conversion | not implemented; rewrites over streams need ordinary evidence |

Productive codata also has no implicit field laziness. Fields are strict, deferred work is a `Thunk` you write, and an infinite stream is not a memo of its elements.

The normative text is [the laziness and reclamation specification](../specifications/lazy-computation-and-reclamation-specification.md); the guardedness decisions are in [the codata design note](../design/codata-guardedness.md). Rewrites that must cite evidence over streams are covered in [rewrites and optimization](12-rewrites-and-optimization.md).

Implementation: `check_terms.rs` (bare-use demand, `demand`, `copyOut`, `delay`, `force`), `check_decls.rs` (`check_productivity`, `walk_guard`, `construction_layer`, the `Eval r` row rule), `codegen.rs`, `runtime/src/abi.rs`. Suites: `compiler/tests/codata.rs`, `compiler/tests/ownership.rs`.
