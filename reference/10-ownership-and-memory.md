# Ownership, regions, and memory reclamation

Cinch enforces memory and resource safety through two distinct layers:

1. **Static usage and region checking (type checking).** Before any code runs, the compiler verifies that every binding obeys its multiplicity (`%0`, `%1`, `%aff`, `%many`), that no borrow escapes its region, that exclusive borrows never alias, and that mutations invalidate stale facts.
2. **Automatic reference-counted reclamation (code generation).** After type checking and optimization, the compiler inserts retain and release operations to reclaim heap storage deterministically without a tracing garbage collector.

Keeping these layers separate means reference counting never weakens static linearity—a `%1` resource cannot be duplicated simply because its runtime representation has a header count—and a program that passes usage checking remains valid even when compiled with reclamation disabled for benchmarking.

# Multiplicities

Parameter multiplicities are written between the argument type and the arrow (`name :: T %1 -> U`):

| Annotation | Name | Rule |
| --- | --- | --- |
| `%0` | Erased | Never used computationally; occupies no runtime storage. |
| `%1` | Linear | Used exactly once along every control-flow path. |
| `%aff` | Affine | Used at most once along every path; may be dropped. |
| `%many` | Unrestricted | Used zero, one, or many times (default for ordinary parameters). |

An unannotated parameter defaults to `%many`, except `Thunk` parameters, which default to `%aff` because forcing consumes a thunk.

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

Violating a binding's multiplicity—directly or through `let` aliases—is rejected statically:

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  dup :: Nat %1 -> Nat
  dup x = x + x

  main :: Int
  main = 0
```

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  duplicate :: Nat %1 -> Nat
  duplicate value = let temporary = value in let alias = temporary in alias + alias

  main :: Int
  main = 0
```

<!-- cinch:reject contains="is never used" -->
```cinch
module Main where
  discard :: Nat %1 -> Nat
  discard value = 0

  main :: Int
  main = 0
```

<!-- cinch:reject contains="is erased and must never be used" -->
```cinch
module Main where
  erase :: Nat %0 -> Nat
  erase x = x

  main :: Int
  main = 0
```

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  twice :: Nat %aff -> Nat
  twice x = x + x

  main :: Int
  main = 0
```

# Path-sensitive usage and place projections

Usage is tracked **per control-flow path**, not summed across mutually exclusive branches. Consuming a `%1` value once in `then` and once in `else` (or once in every `case` arm) satisfies `%1`:

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

Using a `%1` value on one branch while dropping it on another leaves a path with zero uses and is rejected:

<!-- cinch:reject contains="unused on some path" -->
```cinch
module Main where
  dropOne :: Bool -> Nat %1 -> Nat
  dropOne b x = if b then x else 0

  main :: Int
  main = 0
```

Usage is also tracked **per field place**, not merely per variable. Reading disjoint fields of a `%1` record consumes the record once, enabling linear functional updates:

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

Reading the same field place twice violates `%1`:

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

# Captures: lambdas, thunks, tasks, and resumptions

Every construct that defers or transfers execution respects the multiplicities of the values it captures.

## Lambdas and thunks

An ordinary closure may be invoked zero or many times, so it cannot capture a `%1` value from an enclosing scope, nor may a `%1`-annotated lambda drop its parameter:

<!-- cinch:reject contains="used under a lambda or thunk body" -->
```cinch
module Main where
  data Token where
    Token :: Int -> Token

  main :: Int
  main = ((\(token) -> (\u -> case token of { Token value -> value }) ()) :: Token %1 -> Int) (Token 42)
```

<!-- cinch:reject contains="is never used" -->
```cinch
module Main where
  data Token where
    Token :: Int -> Token

  main :: Int
  main = ((\token -> 0) :: Token %1 -> Int) (Token 42)
```

A lambda whose own parameter is annotated `%1` may consume it once inside its body:

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

A `Thunk` annotated `%1` must be forced exactly once on every path (while an unannotated `Thunk` is `%aff`):

<!-- cinch:reject contains="is used 2 time(s)" -->
```cinch
module Main where
  forceTwice :: Thunk {} Nat %1 -> Nat
  forceTwice t = force t + force t

  main :: Int
  main = 0
```

<!-- cinch:run exit=0 -->
```cinch
module Main where
  forceBranch :: Bool -> Thunk {} Nat %1 -> Nat
  forceBranch b t = if b then force t else force t

  main :: Int
  main = if forceBranch True (delay (\u -> 5)) == 5 then 0 else 1
```

## Task boundaries

Spawning a structured task transfers a `%1` value exactly once into the child task:

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

Every capture crossing a task boundary must be one of: a `Copy` value, a shared `Ref`, or a `%1` value transferred to a single task. Capturing a `%1` value in two tasks, sending an exclusive `Mut` borrow across a task boundary, or sharing a non-`Copy` mutable buffer without shareable evidence is rejected:

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

<!-- cinch:reject contains="without shareable evidence" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (bytes-len b))
    joinAllOrCancel s h
```

## Effect resumptions

A handler continuation `k` is one-shot by construction (`%aff` by default, promoted to `%1` whenever the handled computation captures a `%1` value). Resuming once consumes a linear capture cleanly; resuming twice is rejected in both cases:

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

# `Copy` and `copyOut`

`Copy` is a compiler-known structural property satisfied by machine scalars and immutable aggregates whose fields are all `Copy`. User code cannot declare or override `Copy`:

<!-- cinch:reject contains="compiler-known class" -->
```cinch
module Main where
  signature Copy a where
    copy :: a -> a

  main :: Int
  main = 0
```

`copyOut` produces an owned copy from a borrow or copyable place. In generic code, copying a type parameter `a` requires an explicit `using (copy :: Copy a)` constraint:

<!-- cinch:reject contains="no Copy evidence" -->
```cinch
module Main where
  cacheId :: forall a. a -> a
  cacheId x = let ~y = x in y

  main :: Int
  main = 0
```

With `Copy a` in scope, a function can traverse a borrowed structure and copy out matching elements:

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

`copyOut` never bypasses linearity: applying `copyOut` to a `%1` binder is rejected because duplicating a linear value violates `%1`:

<!-- cinch:reject contains="cannot be duplicated" -->
```cinch
module Main where
  steal :: Nat %1 -> Nat
  steal x = copyOut x

  main :: Int
  main = 0
```

# Shared and exclusive borrows

Borrows are indexed by a static region parameter `r`:

- **`Ref r a`** — a shared, read-only borrow of `a` in region `r`, introduced by `borrow`.
- **`Mut r a`** — an exclusive, mutable borrow of `a` in region `r`, introduced by `mutBorrow` and updated with `refSet`.

`copyOut` reads a `Copy` value out of either borrow form, and pattern matching on a borrowed aggregate binds borrows of its fields in the same region:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  peek :: forall region. Ref region Nat -> Nat
  peek b = copyOut b + 1

  main :: Int
  main = if peek (borrow 41) == 42 then 0 else 1
```

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

## Region confinement, uniqueness, and evidence invalidation

Three static invariants protect borrows:

1. **No region escape.** A borrow cannot be returned from the scope that owns its region, nor captured inside a closure or thunk that can outlive that region:

<!-- cinch:reject contains="region escape" -->
```cinch
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in demand y)
```

<!-- cinch:reject contains="may outlive region" -->
```cinch
module Main where
  ignore :: forall a. a -> Int
  ignore x = 1

  main :: Int
  main = ignore (let ~y = 2 in delay (\u -> copyOut (demand y)))
```

2. **Exclusive uniqueness.** At most one `Mut` borrow may be live for a given region at once, and no `Ref` of that same region may coexist with it (`Mut region Nat -> Ref region Nat -> Nat` is rejected with the same aliasing error):

<!-- cinch:reject contains="exclusive borrows are unique" -->
```cinch
module Main where
  use2 :: forall region. Mut region Nat -> Mut region Nat -> Nat
  use2 a b = copyOut a

  main :: Int
  main = 0
```

3. **Write-driven evidence invalidation.** Writing through a `Mut` borrow with `refSet` invalidates any proof or refinement facts previously derived about that region's contents. Continuing to use pre-write evidence is rejected as stale:

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

Re-checking the predicate after the write establishes fresh evidence over the updated contents:

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

# Automatic reference-counted reclamation

After type checking and Core optimization, the compiler inserts reference-counting operations automatically using an owned/borrowed calling and binding discipline:

- **Minimal retains.** An owned binding with `n` owned uses along a path receives `n - 1` retains; a binding that goes dead on a path is released at the start of that path. Pattern-matching a value borrows its fields rather than retaining them unless a field is later used as owned.
- **Fast local paths and safe cross-task sharing.** Thread-local heap blocks use non-atomic reference count updates on the fast path. Values published across task boundaries transition to atomic reference counting so whichever task drops the final reference frees the block safely. Static immortal blocks skip counting altogether.
- **Immediate unboxed integers.** Small mathematical integers (`Nat` in `0 ..= 2^63 - 1` and `Int` in `-2^62 ..= 2^62 - 1`) are represented as immediate tagged words and never allocate or touch reference counts.
- **Constant-stack teardown.** Heap blocks containing child pointers carry pointer maps so cascading releases run iteratively rather than recursively; freeing deeply nested structures such as million-element lists uses `O(1)` stack space.

## Comparing against un-reclaimed builds

Passing `--no-reclaim` (or setting `CINCH_RECLAIM=0` in the compiler environment) omits retain/release insertion during code generation. Because this is a compile-time switch, it produces a separate binary for benchmarking or debugging without changing static ownership checking:

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

## Runtime diagnostics and limits

Compiled executables honor runtime environment variables for allocation accounting (`CINCH_ALLOC_STATS`), leak detection (`CINCH_LEAK_CHECK`, exiting `74` if any mortal blocks remain after `main`), header validation and freed-block quarantine (`CINCH_RC_DEBUG`), heap caps (`CINCH_HEAP_LIMIT`, default 8 GiB), and stack reservation (`CINCH_STACK_SIZE`, default 1 GiB). Heap or stack exhaustion terminates with exit status `73`.

See [allocators and heap customization](25-allocators.md#environment-variables) for exact formats, examples, and measurement caveats (debug poisoning intentionally retains quarantined memory).

# Planned extensions

<span class="support-tag support-tag--planned">Coming soon</span> Replaceable allocators, scoped arenas, functional-in-place (`#[fip]`) reuse guarantees, and statically proven allocation bounds are planned (see [allocators](25-allocators.md)). For currently available layout attributes and safety assertions, see [type layout](24-type-layout.md) and [safety assertions](22-safety-assertions-and-llvm.md).
