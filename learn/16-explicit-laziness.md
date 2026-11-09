# 16. Delay work and memoize deliberately

Cinch evaluates strictly and left to right. Nothing is postponed behind your back: a function's body runs when you call it, a `case` scrutinises the value you handed it, and a strict binding in a `where` block runs when the call is entered. When you do want to postpone work or share one result, you say so with a construct whose type records the consequence. There are two, and the difference that matters is how many times the body runs and how long the result lives.

| Construct | Written as | Body runs | Result lives until |
| --- | --- | --- | --- |
| plain function value | `f ()` for `f = \x -> e` | once per call | the call returns |
| affine thunk | `delay (\() -> e)`, consumed by `force` | at most once | the `force` that consumes it |
| memo cell | `~name = e` in a `let` or `where` block, read by name or by `demand` | at most once per activation of its block | the block that owns the group exits |

Prerequisites: [effects and IO](10-effects-and-io.md) for the `Io r` capability and `with (io = newIO ()) in`, [ownership and regions](12-ownership-and-regions.md) for multiplicities, `%aff` and `%1`, and copies out of borrows. [Productive streams](15-streams-and-producers.md) covers `Later`, the guarded modality that is a third thing again. The normative account is [laziness and codata](../reference/11-laziness-and-codata.md).

# A thunk delays a function body

`delay` takes a function over `Unit`; `force` runs it. Building the thunk runs nothing.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let pending = delay (\() -> 20 + 22)
    in if force pending == 42 then 0 else 1
```

- `delay (\() -> 20 + 22)` has type `Thunk {} Int`. The `{}` is the effect row of the body, and here it is empty because the body is pure.
- `force pending` performs the row of the body, here nothing, and yields the `Int`. `pending` is consumed by that force.
- The `let` binding is `%aff` because its type is a `Thunk`, so using `pending` a second time is refused rather than silently shared: `multiplicity %aff: pending must be used at most once, but is used 2 time(s)`.

The effect row of a thunk is not a comment. A thunk whose body writes is forced in a context that admits `IO r`, and the enclosing definition must say so.

<!-- cinch:run exit=0 stdout="A" -->
```cinch
module Main where
  one-byte :: InRange 0 255 Nat -> Bytes
  one-byte v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  mark :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Nat
  mark v = case ioWrite (one-byte v) of
    _ -> 1

  once :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  once u =
    let pending = delay (\() -> mark 65)
    in if force pending == 1 then 0 else 1

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in once ()
```

`pending` has type `Thunk {IO r} Int`. Constructing it performs nothing, so building it inside `once` costs a closure and no effect. `force pending` performs `IO r`, which is why `once` carries `IO r` in its row. Delete that label from `once`'s signature and the checker refuses the definition with `effect rows do not match: {} vs {IO r#847 | ?r901}`. Delaying IO never removes IO from a signature; it only moves where the row is discharged.

Forcing consumes, so a program that forces the same thunk twice is a usage error, not a program that computes twice.

<!-- cinch:reject contains="multiplicity %aff: pending must be used at most once" -->
```cinch
module Main where
  main :: Int
  main =
    let pending = delay (\() -> 20 + 22)
        a = force pending
        b = force pending
    in if a + b == 84 then 0 else 1
```

There is no sharing question to answer, because a `Thunk` has no cache to consult. If you need the body to run once and be read twice, that is the next section's construct, not this one.

# A memo binding shares a result inside a block

`~name = term` in a `let` or `where` block creates a memo cell. Its body runs at the first demand, and later demands read the result.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  square :: Nat -> Nat
  square n = n * n

  probe :: Nat -> Nat
  probe k = copyOut (demand result)
    where
      ~result = square k

  report :: Nat -> Int
  report k = if result == 9 then 0 else 1
    where
      ~result = square k

  main :: Int
  main = if probe 7 == 49 then report 3 else 2
```

- `~result = square k` is a memo binding. The name `result` has type `Lazy {} r Nat`, where `r` is the group's region, not `Nat`.
- `demand result` performs `Eval r` and returns `Ref r Nat`, a borrow of the cached value. `copyOut` turns that borrow into an owned `Nat` and needs `Copy` evidence for the cached type.
- A bare use of `result`, as in `report`, elaborates to the same demand followed by a copy. That is why `report` needs no `copyOut`.
- The whole block is one memo group: one region, one `Eval` label, and every `~` binding in the block is visible to every other.

A group belongs to one activation of its block. Two calls to a function with a `where`-block cell therefore run the body once each, and nothing survives the call.

<!-- cinch:run exit=0 stdout="XX" -->
```cinch
module Main where
  one-byte :: InRange 0 255 Nat -> Bytes
  one-byte v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  mark :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Nat
  mark v = case ioWrite (one-byte v) of
    _ -> 1

  probe :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  probe u = cell + cell
    where ~cell = mark 88

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    if probe () + probe () == 4 then 0 else 1
```

The program writes the byte `X` twice, once for each `probe ()`. Within a call, `cell + cell` demanded one evaluation and reused it. Across calls, the group is new.

An effect in a cell's body is part of the block's cost, not a hidden side channel. The body's row `{IO r}` has to appear in the enclosing signature exactly as if the body ran eagerly, and the group's own `Eval r` is discharged where the block ends. The block is a handler of its own label. <span class="support-tag support-tag--planned">Coming soon</span> Region escape checking for memo cells is not implemented in this compiler, so the specification's requirement that a `Lazy {e} r a` not outlive `r` is stated there and not yet enforced here; the rule that does run is the cell body's `Eval r` check below.

# The cost model you can reason about

Three constructs, one observable difference: how many times a body runs. This program makes a body write one byte each time it runs, then reads the output.

<!-- cinch:run exit=0 stdout="AAB" -->
```cinch
module Main where
  one-byte :: InRange 0 255 Nat -> Bytes
  one-byte v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  mark :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Nat
  mark v = case ioWrite (one-byte v) of
    _ -> 1

  calledTwice :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  calledTwice u = body () + body ()
    where body = \x -> mark 65

  demandedTwice :: forall r. using (io :: Io r) => Unit ->{IO r} Nat
  demandedTwice u = cell + cell
    where ~cell = mark 66

  neverDemanded :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  neverDemanded u = 0
    where ~spare = mark 67

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let a = calledTwice ()
    in let b = demandedTwice ()
    in if a == 2 then (if b == 2 then neverDemanded () else 1) else 2
```

The output is `AAB`: two `A`s from `calledTwice`, one `B` from `demandedTwice`, and nothing from `neverDemanded`.

| Definition | Body of `mark` runs | Bytes written |
| --- | --- | --- |
| `calledTwice`, a `%many` function value used twice | once per use | `AA` |
| `demandedTwice`, a memo cell demanded twice | once | `B` |
| `neverDemanded`, a memo cell in a block that ends without a demand | zero | nothing |

The first row is the baseline: a plain function value at `%many` is reusable, and each call re-runs the body, which is the cheapest thing in the language when the body is small. A memo cell buys at most one run per group activation and pays a cell allocation, a state word, and an `Eval r` label to be discharged. An affine thunk sits between them: a closure, at most one run, and no lifetime to police. So a one-shot expensive computation with effects is a thunk, a pure calculation read several times inside one call is a memo cell, and an unbounded service cache is neither, because it needs storage and eviction that outlive every block this chapter describes.

# Regions, dependencies, and the black hole

All `~` bindings in one block share a region, so demanding one cell while evaluating another cell of the same group would re-enter the group. The checker forbids that by effect: a cell body may not perform its own group's `Eval` label.

<!-- cinch:reject contains="this memo binding demands its own memo group" -->
```cinch
module Main where
  probe :: Nat -> Nat
  probe k = copyOut (demand a)
    where
      ~a = b + 1
      ~b = k

  main :: Int
  main = if probe 1 == 2 then 0 else 1
```

The refusal is conservative on purpose. `b` does not depend on `a`, so this particular pair would terminate, and it is still refused because the two cells are one group. The program-level fix is structure: put the dependent cell in its own block, which gives it its own region and its own label.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  step :: Nat -> Nat
  step k =
    let ~outer = inner + 1
    in copyOut (demand outer)
    where
      ~inner = k + 1

  main :: Int
  main = if step 1 == 3 then 0 else 1
```

The `where` block owns one group holding `inner`, and the `let` owns a second holding `outer`. `outer`'s body performs the first group's `Eval` label, which is not its own, so the definition checks. Splitting blocks is the whole mechanism; there is no per-cell label to ask for.

A cycle that survives the effect rule and reaches the runtime meets the `BlackHole` fault, which exits with status 70. Well-typed code cannot get there; code crossing a trusted boundary can, and the runtime refuses rather than looping. Memo cells are also refused while a `generate` block runs, where the compile-time evaluator reports ``primitive `cinch.prim.delay` is not available at compile time in this slice``.

# This is not a Haskell irrefutable pattern

<span class="support-tag support-tag--planned">Coming soon</span> **Open design question.** Cinch's `~` marks a memo binding, and only a binding: `~name = e` in a `let` or `where` block. It does not mark a pattern, so `f ~x = x` and `~x` in term position are both parse errors (`irrefutable`, and `memo` respectively). Haskell's `~pattern` in a function argument is a different feature that the language does not have, and the laziness specification lists "whether `~` should also mark irrefutable patterns in function arguments, with the fields of such a pattern elaborating to thunks" as an open decision rather than a promised addition. The shared intuition is only that the bound thing is not evaluated at entry to the block.

# What the compiler enforces

- The binder's default multiplicity follows its type: `%aff` for a `Thunk`, `%1` for anything mentioning `Task`, `%many` otherwise. Forcing consumes a thunk, so no sharing is invented for it.
- `force` performs exactly the row of the thunk's body, and that row must be admitted by the enclosing signature. `force` accepts a `Thunk` or a `Later` and nothing else.
- `demand` performs `Eval r` for the cell's group and returns `Ref r a`, never an owned value. Reading an owned value requires `copyOut` and `Copy` evidence.
- The `Eval r` label of a group is discharged at the end of the block that owns it, so a function with a `where` cell has no `Eval` in its public row.
- A cell body may not perform its own group's `Eval` label. All `~` bindings of a block are one group, so the rule covers mutual demand inside a block.
- Memo bindings take variable patterns only in this slice, must have a name, and are refused inside compile-time evaluation.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `force pending` twice on one thunk | `multiplicity %aff: pending must be used at most once, but is used 2 time(s)` |
| `force 42` | ``type mismatch: expected Int, got Thunk {?r895} ?m2014 (force expects a Thunk or a Later)`` |
| `probe k = demand result` with a `Nat` result type | `type mismatch: expected Ref r#13 Nat, got Nat` |
| a bare use of a cell whose cached type is `Bytes` | ``no Copy evidence for Bytes (copying needs a Copy type or a `using (copy :: Copy a)` constraint)`` |
| a `where` cell whose body performs IO, in a signature without `IO r` | `effect rows do not match: {} vs {IO r#847 \| ?r899}` |
| `~a = b + 1` and `~b = k` in one block | ``effect row must not contain Eval r#13 (a memo cell cannot demand its own group): this memo binding demands its own memo group`` |
| a memo cell in a `generate` block | ``primitive `cinch.prim.delay` is not available at compile time in this slice`` |

Region identifiers such as `r#13` and metavariables such as `?r899` are allocated per compilation, so the numbers differ between programs; the text around them names the rule that refused.

# Exercises

**Exercise 1.** `probe` below is meant to return the square once and compare it with itself. Explain the refusal, then write the accepted version without adding a second evaluation.

```cinch
  probe :: Nat -> Nat
  probe k = demand result
    where
      ~result = square k
```

<details><summary>Answer</summary>

`demand result` returns `Ref r Nat`, a borrow of the cached value, and the signature promises a `Nat`. The refusal is `type mismatch: expected Ref r#13 Nat, got Nat`. Write `copyOut (demand result)`: `copyOut` copies the borrow out and requires `Copy` evidence for `Nat`, which the checker discharges. A bare `result` in that position is equivalent, because a bare use of a memo-bound name already elaborates to a demand plus a copy. Neither version evaluates `square k` more than once per activation of `probe`, so the fix costs a word-sized copy and no second call.

</details>

**Exercise 2.** You have a pure `summary :: List Nat -> String` that takes about a millisecond and is read three times in one function, and an `audit :: Request -> Result LoadError String` that reads the network and must happen at most once. Which construct does each want, and what does the type of the binding tell the checker?

<details><summary>Answer</summary>

`summary` wants a memo binding: `~s = summary xs`, read three times by name. The binding's type is `Lazy {} r String`, so the body runs at most once per activation of the block and the three reads copy the cached `String` out. That needs `Copy` evidence for `String`; if the cached type is not `Copy`, the checker says `no Copy evidence for ...` and the cell is the wrong construct.

`audit` wants an affine thunk: `pending = delay (\() -> audit req)`, consumed by one `force pending`. Its type is `Thunk {IO r, ...} String`, so the row travels with the value, `force` performs it once, and a second force is a multiplicity error rather than a second network call. The binding type is the specification: `Thunk` says at most once and no cache, `Lazy` says once per group with a borrow on read.

</details>

Next: [structured tasks and cancellation](17-structured-concurrency.md).
