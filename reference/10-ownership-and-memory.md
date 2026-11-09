# Ownership, regions, and memory reclamation

Two jobs run over the same program and must not be confused. The usage checker decides, before any code runs, whether each value is used the number of times its multiplicity allows, whether a borrow escapes its region, and whether an exclusive borrow overlaps another. Reclamation decides, during code generation, where reference counts are incremented and decremented. A reference-counted representation does not make an affine or linear resource logically duplicable, and a program that checks can still be compiled without any reclamation at all.

# Multiplicities

A parameter's multiplicity is written on the arrow:

```text
name :: T %1 -> U
```

| Marked | Name | Obligation | Refusal when broken |
| --- | --- | --- | --- |
| `%0` | erased | Never used in the body. Has no runtime representation. | `` multiplicity %0: x is erased and must never be used, but is used 1 time(s) `` |
| `%1` | linear | Used exactly once on every admitted path. | `` multiplicity %1: x must be used exactly once on every path, but is used 2 time(s) `` |
| `%aff` | affine | Used at most once; may be dropped. | `` multiplicity %aff: x must be used at most once, but is used 2 time(s) `` |
| `%many` | unrestricted | Used any number of times. | none for repeated use |

An unannotated parameter is `%many`. An unannotated `Thunk` parameter is `%aff`, because forcing is the use and a thunk may be dropped instead.

Each multiplicity in a program that checks:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  erasedUse :: Nat %0 -> Nat
  erasedUse x = 7

  once :: Nat %1 -> Nat
  once x = x + 1

  atMostOnce :: Bool -> Nat %aff -> Nat
  atMostOnce b x = if b then x else 0

  freely :: Nat %many -> Nat
  freely x = x + x

  main :: Int
  main =
    if erasedUse 1 == 7 && once 2 == 3 && atMostOnce False 4 == 0 && freely 5 == 10
    then 0 else 1
```

A `%1` value used twice:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  dup :: Nat %1 -> Nat
  dup x = x + x

  main :: Int
  main = 0
```

```text
cinch: 3:7: multiplicity %1: x must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

The same obligation applies through an alias. A `let` does not reset the count:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  duplicate :: Nat %1 -> Nat
  duplicate value = let temporary = value in let alias = temporary in alias + alias

  main :: Int
  main = 0
```

A `%1` value that is never used:

<!-- cinch:reject contains="is never used" -->
```cinch
module Main where
  discard :: Nat %1 -> Nat
  discard value = 0

  main :: Int
  main = 0
```

```text
cinch: 3:11: multiplicity %1: value must be used exactly once on every path, but is never used (in Main.cinch)
```

A `%0` value used at all:

<!-- cinch:reject contains="is erased and must never be used" -->
```cinch
module Main where
  erase :: Nat %0 -> Nat
  erase x = x

  main :: Int
  main = 0
```

```text
cinch: 3:9: multiplicity %0: x is erased and must never be used, but is used 1 time(s) (in Main.cinch)
```

An `%aff` value used twice:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  twice :: Nat %aff -> Nat
  twice x = x + x

  main :: Int
  main = 0
```

```text
cinch: 3:9: multiplicity %aff: x must be used at most once, but is used 2 time(s) (in Main.cinch)
```

# Path-sensitive counting

Branch alternatives are separate paths, and each path is counted on its own. One use in `then` and one use in `else` is one use per path; a linear value is not consumed twice.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  pickIf :: Bool -> Nat %1 -> Nat
  pickIf b x = if b then x else x

  pickCase :: Bool -> Nat %1 -> Nat
  pickCase b x = case b of
    True -> x
    False -> x

  main :: Int
  main = if pickIf True 3 == 3 && pickCase False 4 == 4 then 0 else 1
```

Counting is not a total over the whole body. Using a `%1` value in one branch and discarding it in another leaves a path where it is unused, and the diagnostic reports the range:

<!-- cinch:reject contains="unused on some path" -->
```cinch
module Main where
  dropOne :: Bool -> Nat %1 -> Nat
  dropOne b x = if b then x else 0

  main :: Int
  main = 0
```

```text
cinch: 3:13: multiplicity %1: x must be used exactly once on every path, but is used 0 to 1 time(s) (unused on some path) (in Main.cinch)
```

# Places and projections

Uses are counted per place, not per variable. Two projections of disjoint fields consume a `%1` record once, which is what makes an update in place expressible:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Counter where
    base :: Nat
    delta :: Nat

  increment :: Counter %1 -> Counter
  increment c = Counter { base = c.base, delta = c.delta + 1 }

  main :: Int
  main = let r = increment (Counter { base = 1, delta = 2 }) in
           if r.base == 1 && r.delta == 3 then 0 else 1
```

Consuming the same place twice stacks uses and breaks the obligation:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  record Counter where
    base :: Nat
    delta :: Nat

  twice :: Counter %1 -> Nat
  twice c = c.base + c.base

  main :: Int
  main = 0
```

```text
cinch: 7:9: multiplicity %1: c must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

# Captured values keep their multiplicity

A value placed inside a lambda body is used an unknown number of times, so a `%1` value cannot be captured there:

<!-- cinch:reject contains="used under a lambda or thunk body" -->
```cinch
module Main where
  data Token where
    Token :: Int -> Token

  main :: Int
  main = ((\(token) -> (\u -> case token of { Token value -> value }) ()) :: Token %1 -> Int) (Token 42)
```

```text
cinch: 5:14: multiplicity %1: token must be used exactly once on every path, but is used under a lambda or thunk body (zero or unboundedly many times) (in Main.cinch)
```

The same rule covers a lambda that drops its `%1` parameter:

<!-- cinch:reject contains="is never used" -->
```cinch
module Main where
  data Token where
    Token :: Int -> Token

  main :: Int
  main = ((\token -> 0) :: Token %1 -> Int) (Token 42)
```

```text
cinch: 5:14: multiplicity %1: token must be used exactly once on every path, but is never used (in Main.cinch)
```

A lambda can itself be annotated `%1`. Its parameter is then owned inside the body and used exactly once there, which makes an inline linear function usable:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Token where
    Token :: Int -> Token

  main :: Int
  main =
    let a = ((\token -> case token of { Token value -> value }) :: Token %1 -> Int) (Token 40)
    in let b = ((\(Token value) -> value) :: Token %1 -> Int) (Token 1)
    in if a + b == 41 then 0 else 1
```

Thunks follow the same discipline. A thunk marked `%1` must be forced exactly once, and an unannotated thunk is `%aff`:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  forceTwice :: Thunk {} Nat %1 -> Nat
  forceTwice t = force t + force t

  main :: Int
  main = 0
```

```text
cinch: 3:14: multiplicity %1: t must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

A `%1` thunk forced on one branch at a time is accepted:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  forceBranch :: Bool -> Thunk {} Nat %1 -> Nat
  forceBranch b t = if b then force t else force t

  main :: Int
  main = if forceBranch True (delay (\u -> 5)) == 5 then 0 else 1
```

A task capture transfers a `%1` value into the task exactly once. The transfer is visible to the usage checker at the capture site, so one spawn can consume it:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError Nat
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (bytes-new 3) of
    Ok n -> if n == 3 then 0 else 1
    Err e -> 2
```

Two tasks capturing the same `%1` value would consume it twice:

<!-- cinch:reject contains="used 2 time(s)" -->
```cinch
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    h2 <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h h2
```

Task captures face a second rule: a capture must be `Copy`, a shared `Ref`, or transferred exactly once. An exclusive borrow cannot cross, and a mutable buffer without shareable evidence is refused:

<!-- cinch:reject contains="exclusive `Mut` borrow cannot cross a task boundary" -->
```cinch
module Main where
  readMut :: forall g. Mut g Nat -> Nat
  readMut m = copyOut m

  bad :: forall r g. using (io :: Io r) => Mut g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad m = withScope $ \s -> do
    h <- spawn s (\() -> Ok (readMut m))
    joinAllOrCancel s h
```

```text
cinch: 7:38: an exclusive `Mut` borrow cannot cross a task boundary: m holds one (concurrency specification, section 3) (in Main.cinch)
```

<!-- cinch:reject contains="without shareable evidence" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (bytes-len b))
    joinAllOrCancel s h
```

```text
cinch: 4:40: a task capture must be `Copy`, a shared `Ref`, or transferred exactly once: b is shared across tasks without shareable evidence (concurrency specification, section 3) (in Main.cinch)
```

A resumption is one-shot, and its captures keep their multiplicities. Resuming twice duplicates the continuation and is refused:

<!-- cinch:reject contains="must be used exactly once" -->
```cinch
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Int

  consume :: Int %1 -> Int
  consume x = x

  run :: Int %1 -> Int
  run lin = handle poke () + consume lin with (handler Poke where
    return x -> x
    poke u resume k -> k 1 + k 2)

  main :: Int
  main = 0
```

```text
cinch: 11:19: multiplicity %1: k must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

Resuming exactly once consumes the capture exactly once, and the handler checks:

<!-- cinch:run exit=0 -->
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
  main = if run 5 == 6 then 0 else 1
```

A capture that holds nothing linear is still one-shot. A second `resume` in a handler with no linear capture reports the at-most-once obligation instead:

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

```text
cinch: 8:19: multiplicity %aff: k must be used at most once, but is used 2 time(s) (in Main.cinch)
```

# Copy and `copyOut`

`Copy` is a compiler-known structural property with instances decided by structure: machine scalars, and immutable aggregates of `Copy` fields. It is not a user-extensible class:

<!-- cinch:reject contains="compiler-known class" -->
```cinch
module Main where
  signature Copy a where
    copy :: a -> a

  main :: Int
  main = 0
```

```text
cinch: 2:13: Copy is a compiler-known class with structural instances (ownership and usage design, section 3); it cannot be declared (in Main.cinch)
```

`copyOut` turns an admitted copyable place or borrow into an owned value. Copying needs evidence, and a generic context does not have it unless it asks:

<!-- cinch:reject contains="no Copy evidence" -->
```cinch
module Main where
  cacheId :: forall a. a -> a
  cacheId x = let ~y = x in y

  main :: Int
  main = 0
```

```text
cinch: 3:29: no Copy evidence for a#844 (copying needs a Copy type or a `using (copy :: Copy a)` constraint) (in Main.cinch)
```

With the constraint in scope, a filter over a borrowed list copies each element out and rebuilds an owned list:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data List2 a where
    Nil2 :: List2 a
    Cons2 :: a -> List2 a -> List2 a

  filter2 :: forall a region. using (copy :: Copy a) =>
             (a -> Bool) -> Ref region (List2 a) -> List2 a
  filter2 predicate xs = case xs of
    Nil2 -> Nil2
    Cons2 x rest ->
      let keep = predicate (copyOut x)
      in if keep then Cons2 (copyOut x) (filter2 predicate rest)
         else filter2 predicate rest

  main :: Int
  main = case filter2 (\x -> x == 2) (borrow (Cons2 1 (Cons2 2 (Cons2 2 Nil2)))) of
    Cons2 a (Cons2 b Nil2) -> if a == 2 && b == 2 then 0 else 1
    Nil2 -> 2
    Cons2 a Nil2 -> 3
    Cons2 a (Cons2 b (Cons2 c rest)) -> 4
```

`copyOut` cannot launder a linear argument into multiple uses, because copying a `%1` binder is itself the duplication the multiplicity forbids:

<!-- cinch:reject contains="cannot be duplicated" -->
```cinch
module Main where
  steal :: Nat %1 -> Nat
  steal x = copyOut x

  main :: Int
  main = 0
```

```text
cinch: 3:21: multiplicity %1: copyOut would copy x, but a value bound %1 cannot be duplicated (copying needs a %many binder or a borrow) (in Main.cinch)
```

# Shared and exclusive borrows

`Ref r a` is a shared borrow of `a` in region `r`; `Mut r a` is an exclusive borrow of the same region. `borrow` introduces a shared borrow, `mutBorrow` an exclusive one, and `copyOut` reads a value out through either. A borrow is consumed inside its region:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  peek :: forall region. Ref region Nat -> Nat
  peek b = copyOut b + 1

  main :: Int
  main = if peek (borrow 41) == 42 then 0 else 1
```

Matching a borrowed aggregate binds references to its fields, and a result may be a borrow of the same region:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data List2 a where
    Nil2 :: List2 a
    Cons2 :: a -> List2 a -> List2 a

  firstOr :: forall a region. Ref region a -> Ref region (List2 a) -> Ref region a
  firstOr d xs = case xs of
    Nil2 -> d
    Cons2 x rest -> x

  main :: Int
  main = if copyOut (firstOr (borrow 0) (borrow (Cons2 5 Nil2))) == 5 then 0 else 1
```

A borrow never outlives its region. A returned `Ref` whose region is a scope inside the function is refused:

<!-- cinch:reject contains="region escape" -->
```cinch
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in demand y)
```

```text
cinch: 6:18: region escape: a borrow, memo cell or Eval effect of region r#13 leaves its scope in Ref r#13 Int (in Main.cinch)
```

A closure that could outlive the region cannot capture the borrow either:

<!-- cinch:reject contains="may outlive region" -->
```cinch
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in delay (\u -> copyOut (demand y)))
```

```text
cinch: 6:61: region escape: y is captured in a lambda that may outlive region r#13 (in Main.cinch)
```

Uniqueness is enforced lexically: at most one live `Mut` per region, and no `Ref` of that region while it is live.

<!-- cinch:reject contains="exclusive borrows are unique" -->
```cinch
module Main where
  use2 :: forall region. Mut region Nat -> Mut region Nat -> Nat
  use2 a b = copyOut a

  main :: Int
  main = 0
```

```text
cinch: 3:10: region region#844: exclusive borrows are unique; b would alias another live borrow of region#844 (in Main.cinch)
```

The same refusal names the shared borrow in a mixed signature, so `Mut region Nat -> Ref region Nat -> Nat` is rejected with the same message.

A write through an exclusive borrow invalidates evidence facts about the previous contents. The refusal names both the derivation and the write:

<!-- cinch:reject contains="stale evidence" -->
```cinch
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = refSet m 9 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
```

```text
cinch: 7:31: stale evidence: evidence about the contents of `region#844` derived at 5:11 is invalidated by the write at 6:17 (in Main.cinch)
```

Re-deriving after the write is what makes the same shape check, and a read through the borrow observes the new contents:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prog :: forall region. Mut region Nat -> Int
  prog m =
    case decide (copyOut m < 5) of
      Yes _ ->
        let u = refSet m 9 in
        case decide (copyOut m < 5) of
          Yes _ -> 1
          No _ -> 0
      No _ -> 0

  main :: Int
  main = prog (mutBorrow 3)
```

# Reclamation is code generation

Nothing above proves anything about memory. The checker's obligations are about use counts, regions and evidence; the runtime's obligations are about counts. Keeping them apart explains both of these facts: a program can check and still be built without reclamation, and a build can report leaks without any program error.

| Question | Decided by | Where |
| --- | --- | --- |
| May this value be used here, and how many times? | Usage checker | `compiler/src/check_terms.rs` |
| Does this borrow outlive its region? | Region escape check | `compiler/src/check.rs` |
| Where does a retain or a release go? | The `rc` pass over `Core` | `compiler/src/rc.rs` |
| What does a count word mean? | Runtime header encoding | `compiler/runtime/src/block_kinds.rs` |
| Was storage released? | Runtime counters | `CINCH_ALLOC_STATS`, `CINCH_LEAK_CHECK` |

The insertion pass runs on the residual program after `core::optimize` and before code generation. Its placement rules are stated in the [reclamation design](../design/reclamation.md) section 4 and implemented in `compiler/src/rc.rs`: every counted binder is owned or borrowed, an owned variable with n owned uses on a path gets n-1 retains, a variable dead on a path is released at the start of that path, and constructor fields bound by a `case` are borrowed loads that are retained only when used as owned. `cinch_dup` and `cinch_drop` are emitted inline as header tests rather than as calls. The emitted release for an owned word loads the header atomically, shifts the count bits to the top of the word, and compares against one: a count above one is decremented and stored back with a plain store, a count of one calls `cinch_drop_slow`, a count of zero is an immortal block and does nothing, and a negative count is a block shared across tasks and goes to `cinch_drop_mt`. Blocks whose fields can contain objects carry pointer maps, which reclamation follows to release children iteratively rather than recursively, so releasing a million-cell list uses constant stack.

`CINCH_RECLAIM=0` in the **compiler environment** removes that insertion. Compare the two builds of one program:

```text
$ cinch build Main.cinch -o example
$ CINCH_LEAK_CHECK=1 ./example
cinch-leak: live=0 immortal=0 leaked=0
$ CINCH_RECLAIM=0 cinch build Main.cinch -o example-no-reclaim
$ CINCH_LEAK_CHECK=1 ./example-no-reclaim
cinch-leak: live=1916 immortal=0 leaked=1916
$ echo $?
74
```

The second build releases nothing: 1916 value blocks are still live when `main` returns, so the leak check reports them and returns exit code 74. Both builds check, and both would refuse a duplicated `%1` value. `CINCH_RECLAIM=0` is a comparison and baseline switch, not a recommended application memory mode, and it is not a runtime flag: it changes generated code, so it cannot retroactively change an already built executable.

Reclamation of shared task captures uses the same counts with an atomic form: a block whose count went negative is shared across tasks, and whichever task drops the last reference frees it. Small mathematical integers do not reach the allocator at all: a `Nat` in `0 ..= 2^63 - 1` or an `Int` in `-2^62 ..= 2^62 - 1` is an immediate word, so reference counting skips it.

# Runtime diagnostics and limits

| Variable | Read by | Meaning |
| --- | --- | --- |
| `CINCH_ALLOC_STATS=1` | Running program | Print cumulative and live allocation counters at exit. |
| `CINCH_LEAK_CHECK=1` | Running program | Report live mortal blocks after `main`; exit 74 when any remain. |
| `CINCH_RC_DEBUG=1` | Running program | Poison and quarantine freed blocks, and validate headers on every count operation. |
| `CINCH_HEAP_LIMIT=n[K\|M\|G]` | Running program | Bound live accounted heap; default 8 GiB, `0` disables the cap. |
| `CINCH_STACK_SIZE=n[K\|M\|G]` | Running program | Reserved program-thread stack; zero or unparsable keeps 1 GiB. |
| `CINCH_RECLAIM=0` | Compiler | Skip retain and release insertion. |

Suffixes use binary multiples. Exhaustion is a defined process exit (73, `cinch: exhausted: heap (<n> bytes)` or `cinch: exhausted: stack`), not a successful value or a caught `Result` error. A failed leak check is exit 74. Debug poisoning intentionally retains memory and changes measurements; compare performance with the debug modes disabled.

[Allocators and heap customization](25-allocators.md) has the formats each variable accepts, one observed run per variable, and the boundary between this implemented accounting layer and the planned replaceable allocator.

# What is planned

<span class="support-tag support-tag--planned">Coming soon</span> the [allocator design](../design/allocators.md) and the [layout and specialization design](../design/layouts-and-specialization.md) describe selected future directions for allocation, representation and proven bounds, including `#[fip]` and bounded allocation assertions. Their APIs must be marked planned until accepted by the Rust implementation. Current explicit layout attributes are documented in [type layout](24-type-layout.md), and the implemented allocation assertions in [safety assertions](22-safety-assertions-and-llvm.md).

Implementation: `check_terms.rs`, `check.rs`, `rc.rs`, `core.rs`, runtime `block.rs`, `rc.rs`, `heap.rs`, `stack.rs`; suites: `ownership.rs`, `concurrency.rs`, `effects.rs`, `evidence_invalidation.rs`, `reclamation.rs`, `reclamation_tasks.rs`, `alloc_accounting.rs`, `exhaustion.rs`. Design: [ownership and usage](../design/ownership-and-usage.md), [reclamation](../design/reclamation.md).
