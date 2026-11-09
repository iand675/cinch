# 12. Ownership, borrowing, and regions

An effect says what a computation may do. A multiplicity says how many times it may use a value. The distinction matters for anything with a protocol: a task handle must be joined once, a mutable cell must be written by one borrow at a time, and a duplicated linear value would run its protocol twice. This chapter reads usage annotations, writes a linear value that is consumed exactly once, borrows with `Ref` and `Mut`, refuses to let a borrow outlive its region, and states what the runtime actually reclaims.

Check with `$C check Main.cinch` and run with `$C run Main.cinch`.

# Before you start

You need function arrows from [chapter 3](03-functions.md), `case` and records from [chapter 4](04-data-and-matching.md), effect rows and capabilities from [chapter 10](10-effects-and-io.md), and handlers from [chapter 11](11-handlers-and-sequencing.md) for the closing program.

# Read the usage annotation

Multiplicity is written between the argument and the arrow: `Int %1 -> Int` is a function that must consume its argument exactly once.

| Arrow | Meaning | Common cause |
| --- | --- | --- |
| `a %0 -> b` | the argument is erased; the body must not use it | a phantom or dictionary argument |
| `a %1 -> b` | consume the argument exactly once on every path | a resource with a protocol |
| `a %aff -> b` | consume it at most once | a resumption, an optional close |
| `a %many -> b` | any number of uses, subject to the value's other rules | ordinary data |

The default for an unannotated arrow is `%many`. A `%1` argument may be used zero times only on paths that are not admitted; the checker requires exactly one use on every path that can be reached, which makes `%1` a statement about the protocol rather than a count of text occurrences.

# Exactly once on every path

Usage is counted per path, so two branches that each consume the value once satisfy `%1`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  choose :: Bool -> Int %1 -> Int
  choose flag value = if flag then value else value

  main :: Int
  main = if choose False 42 == 42 then 0 else 1
```

Only one branch runs, so `value` is used once on each admitted path. Changing one branch to return a constant leaves `value` unused on that path, and the message names a range rather than a count:

```text
cinch: 3:15: multiplicity %1: value must be used exactly once on every path, but is used 0 to 1 time(s) (unused on some path) (in Main.cinch)
```

Projections count the same way. In `increment`, `c.base` and `c.delta` are disjoint places of one `%1` record, so reading each once and building a new record uses the value exactly once. Reading the same place in two positions is two uses of one place:

<!-- cinch:reject contains="but is used 2 time(s)" -->
```cinch
module Main where
  record Counter where
    base :: Int
    delta :: Int

  twice :: Counter %1 -> Int
  twice c = c.base + c.base

  main :: Int
  main = 0
```

```text
cinch: 7:9: multiplicity %1: c must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

# A linear value consumed once

A data declaration plus a `%1` parameter is enough to give a resource a protocol, as in this fragment:

```cinch
  data Permit where
    Permit :: Int -> Permit

  release :: Permit %1 -> Int
  release p = case p of
    Permit budget -> budget
```

`release` is the whole protocol: a caller holding a `Permit` must pass it to `release` exactly once, and the type carries that obligation from the construction site to the consumption site. Hiding the constructor behind the module boundary is what makes the protocol enforceable, which is the export rule from [chapter 8](08-modules.md).

# A duplicated or dropped linear value

Two uses of one `%1` binder is the error to recognise first:

<!-- cinch:reject contains="but is used 2 time(s)" -->
```cinch
module Main where
  duplicate :: Int %1 -> Int
  duplicate n = n + n

  main :: Int
  main = 0
```

```text
cinch: 3:13: multiplicity %1: n must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)
```

Never using it is the other half of the rule, and it reports `but is never used` at the binder. The weaker annotations have their own counts: `%aff` allows zero or one use and refuses two, and `%0` refuses any use because an erased argument has no runtime value to read. Do not resolve either error by widening `%1` to `%many`: the annotation is the protocol, and widening it moves the obligation into a comment.

# Copy is controlled by the compiler

`Copy` is a compiler-known class with structural instances. Declaring an instance for your own type is rejected with `Copy is a compiler-known class with structural instances (ownership and usage design, section 3); it has no user instances`.

`copyOut` produces an owned copy of a borrow when the structural rule allows it. It cannot duplicate a linear binder, because copying is by definition a second use:

<!-- cinch:reject contains="cannot be duplicated" -->
```cinch
module Main where
  steal :: Int %1 -> Int
  steal x = copyOut x

  main :: Int
  main = 0
```

```text
cinch: 3:21: multiplicity %1: copyOut would copy x, but a value bound %1 cannot be duplicated (copying needs a %many binder or a borrow) (in Main.cinch)
```

The message states both permitted routes: take the value at `%many`, or take a borrow of it and copy out of the borrow.

# Borrow when you only need to read

`Ref region a` is a shared borrow, and `Mut region a` is an exclusive one. A `Ref` reaches the value without owning it, so reading through one is not a use of the owner:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  peek :: forall region. Ref region Int -> Int
  peek value = copyOut value + 1

  main :: Int
  main = if peek (borrow 41) == 42 then 0 else 1
```

`borrow e` introduces a `Ref` to `e`'s value in a fresh region, and `copyOut value` copies the contents out of the borrow. The region is the scope in which the borrow is valid, and the signature states it as the `forall region.` binder, so the caller learns from the type that the borrow does not outlive the call.

An exclusive borrow is created with `mutBorrow` and written with `refSet`. Reads after the write observe the new contents:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  wrote :: forall region. Mut region Int -> Int
  wrote cell = let written = refSet cell 42 in copyOut cell

  main :: Int
  main =
    let observed = wrote (mutBorrow 41)
    in if observed == 42 then 0 else 1
```

`refSet cell 42` takes the `Mut` and the new value and returns `Unit`, and `copyOut cell` afterwards reads `42`. Two exclusive borrows of one region cannot be live at once, and the checker says so with `region region#844: exclusive borrows are unique; b would alias another live borrow of region#844`. The same refusal covers a `Ref` and a `Mut` of one region in a single signature, because a shared borrow and a write cannot coexist. A `borrow` whose region cannot be determined from context is rejected too, with a message that asks for an annotation against `Ref r a` or `Mut r a`.

# A borrow must not outlive its region

The region is checked at the boundary of the scope that owns the borrowed value. Returning a borrow of a value owned by a smaller scope is refused:

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

`~y` is a lazily initialised binding whose value is cached in a cell owned by the enclosing expression; `demand y` hands back a borrow of that cache. Letting the borrow leave the scope would leave a reference behind a value that is already reclaimed, so the checker refuses it. The closure form of the same mistake has its own message, `region escape: y is captured in a lambda that may outlive region r#13`. Laziness and its cells are [chapter 16](16-explicit-laziness.md)'s subject; the rule here is that a borrow is valid in its region, and a region is a scope.

# What reclamation does at runtime

Values are reference counted, not traced. Every heap block carries a header word with a count; generated code calls `cinch_dup` when a reference is copied and `cinch_drop` when one is released, and the drop that takes the count to zero frees the block together with everything only it reached. A program's footprint therefore tracks the data it still holds rather than the total it has allocated.

Two kinds of block are never freed: static blocks emitted into the program image, and interned `Sym` cells, both marked immortal with a count of `0`. A block reachable from a task body is marked before the task's thread starts and its count becomes atomic, so a value shared across threads is released exactly once by whichever thread drops the last reference.

The runtime reports exhaustion rather than failing silently. Running out of heap ends the process with status `73`:

```text
cinch: exhausted: heap (100010 bytes)
```

`CINCH_HEAP_LIMIT` caps live bytes in the compiler's environment, which is why that message reports a small number under a test limit. Stack and thread exhaustion take the same status with their own words. This is distinct from a nonzero status your own `main` returns, so a program can distinguish its own failure from a runtime resource failure.

Multiplicity and reclamation answer different questions. A `%1` annotation is a compile-time protocol obligation, checked before the program runs, and no runtime count can excuse a violation of it.

# A program with one handler and one linear resource

The pieces combine: a handler that resumes, a linear value consumed exactly once, and one capability scope for output.

<!-- cinch:run exit=0 stdout="42 7\n" -->
```cinch
module Main where
  effect Ask where
    ask :: Unit ->{Ask} Int

  data Permit where
    Permit :: Int -> Permit

  release :: Permit %1 -> Int
  release p = case p of
    Permit budget -> budget

  line :: Int -> Int -> String
  line a b = string-concat (int-to-string a) (string-concat " " (string-concat (int-to-string b) "\n"))

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let total = handle ask () + 2 with (handler Ask where
          return value -> value
          ask unit resume k -> k 40)
    in let granted = release (Permit 7)
    in let written = ioWrite (string-to-bytes (line total granted))
    in 0
```

The run prints `42 7` and exits `0`.

Read it in three parts. `total` is `handle ask () + 2 with ...`: the operation clause resumes with `40`, the addition runs, and the handle expression has type `Int` with no row, because handling removed `Ask`. `granted` is `release (Permit 7)`: the permit is constructed and consumed in one expression, so the exactly-once obligation is discharged where it is created. `written` is the single capability use, and `main` returns `0`.

If the permit had to cross a function boundary, its type would carry the obligation. A helper named `grant :: Permit %1 -> Int -> Int` promises in its signature alone that it consumes the permit, and a caller could not pass the same permit to two such helpers.

# What the compiler enforces

| Rule | Diagnostic family |
| --- | --- |
| a `%1` binder is used once on every path | `multiplicity %1: ... must be used exactly once on every path` |
| an `%aff` binder is used at most once | `multiplicity %aff: ... must be used at most once` |
| a `%0` binder is never used | `multiplicity %0: ... is erased and must never be used` |
| `copyOut` does not duplicate a linear binder | `copyOut would copy ... but a value bound %1 cannot be duplicated` |
| `Copy` has no user instances | `Copy is a compiler-known class with structural instances` |
| exclusive borrows of one region are unique | `exclusive borrows are unique; b would alias another live borrow` |
| a borrow stays inside its region | `region escape: ... leaves its scope in Ref r#13 Int` |
| a borrow is not captured by an escaping closure | `region escape: ... captured in a lambda that may outlive` |
| live bytes are capped at runtime | `cinch: exhausted: heap (<n> bytes)` with exit `73` |

# Diagnostics you will meet

| You wrote | Message |
| --- | --- |
| a duplicated linear use | `cinch: 3:13: multiplicity %1: n must be used exactly once on every path, but is used 2 time(s) (in Main.cinch)` |
| a dropped linear use | `cinch: 3:11: multiplicity %1: n must be used exactly once on every path, but is never used (in Main.cinch)` |
| an erased argument used | `cinch: 3:9: multiplicity %0: n is erased and must never be used, but is used 1 time(s) (in Main.cinch)` |
| an at-most-once argument used twice | `cinch: 3:16: multiplicity %aff: n must be used at most once, but is used 2 time(s) (in Main.cinch)` |
| a copy of a linear binder | `cinch: 3:21: multiplicity %1: copyOut would copy x, but a value bound %1 cannot be duplicated (copying needs a %many binder or a borrow) (in Main.cinch)` |
| an aliased exclusive borrow | `cinch: 3:10: region region#844: exclusive borrows are unique; b would alias another live borrow of region#844 (in Main.cinch)` |
| an escaping borrow | `cinch: 6:18: region escape: a borrow, memo cell or Eval effect of region r#13 leaves its scope in Ref r#13 Int (in Main.cinch)` |

Variable and region identities such as `n`, `region#844`, and `r#13` are fresh for each program. The words to read are the multiplicity and the rule name.

# Exercises

1. `consume :: Int %1 -> Int` with the body `if True then x else 0` is rejected. Explain which path fails and why a constant condition does not help.
2. Write `describe :: Permit %1 -> String` that reports a permit's budget and consumes it, then explain what a caller cannot do with the result.

<details><summary>Answer to exercise 1</summary>

The checker counts uses on every admitted path, not on the paths a constant condition happens to select. The `else` branch returns `0` without using `x`, so that path consumes the value zero times, and the definition fails with `multiplicity %1: x must be used exactly once on every path, but is used 0 to 1 time(s) (unused on some path)`. The fix is to consume the resource in both branches, rather than to make one branch unreachable.

</details>

<details><summary>Answer to exercise 2</summary>

The fragment `describe p = case p of Permit budget -> string-concat "permit " (int-to-string budget)` uses the permit exactly once, by matching on it and reading its field, and the result is an ordinary `String` with no lifetime attached. A caller cannot then pass the same permit to `release`, because it was already consumed. That is the point of a linear protocol: the type records that the value is spent, so a second use is a compile error rather than a runtime surprise.

</details>

Continue with [chapter 13](13-refinements-and-safe-operations.md), where a type carries a fact that a bounds check can rely on.
