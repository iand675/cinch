# 6. Missing values, errors, and validation

A map lookup can miss, a count can be out of range, and a division can have a zero divisor. None of those is a reason for the process to fault, and none of them can be settled by the type checker alone. This chapter uses two types to put those possibilities in the return value, `Option a` for a value that may be absent and `Result e a` for a computation that either succeeds with an `a` or fails with an explanation of type `e`, and then separates a runtime rejection from a compile-time error. Work in a file named `Main.cinch`:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

`check` typechecks and `run` builds and executes, returning the program's exit status. Build the compiler and put it on `PATH` with [using the compiler](../reference/01-using-the-compiler.md).

**Prerequisites.** [Chapter 4](04-data-and-matching.md) for data declarations and `case`, and [chapter 5](05-recursion-and-collections.md) for `List`, `nth` and `Nat` arithmetic. `Option` and `Result` are in the implicit `Prelude`.

# Option says a value might be absent

`Option` is declared in `Data.Core` and re-exported by `Data.Option`. The two constructors are all it has:

```cinch
data Option a where
  None :: Option a
  Some :: a -> Option a
```

A function that can miss says so in its result type, and the caller has to match both constructors:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prices :: List Nat
  prices = [7, 12, 5]

  main :: Int
  main = case nth 1 prices of
    None -> 1
    Some price -> if price == 12 && is-some (nth 0 prices) then 0 else 2
```

`nth` already answers `Option a` rather than faulting on an index past the end, so nothing here has to test a bound. The `case` is one policy for the absent index, `from-option` in the table is another, and `from-option 0 found` supplies a default. The type forces you to choose a policy; it does not choose one for you.

The combinators in `Data.Option` are free functions, so none of them needs a constraint:

| Name | Signature | Result |
| --- | --- | --- |
| `maybe` | `b -> (a -> b) -> Option a -> b` | default, or apply a function |
| `from-option` | `a -> Option a -> a` | default, or the value |
| `is-some`, `is-none` | `Option a -> Bool` | tag test |
| `map-option` | `(a -> b) -> Option a -> Option b` | transform the present value |
| `or-else` | `Option a -> Option a -> Option a` | first present value |
| `list-to-option` | `List a -> Option a` | first element, if any |

There is no `Eq (Option a)` instance, and no `Eq (Result e a)` either, so `found == Some 1` does not typecheck. Match the value instead, as the program above does. That rule exists because a comparison would let you skip the absent branch.

# Result says a computation can fail with a reason

`Result` has the same shape with an extra type parameter for the explanation:

```cinch
data Result e a where
  Err :: e -> Result e a
  Ok :: a -> Result e a
```

The error type is yours. Declare one with a constructor per failure reason, so that a caller can tell them apart and so that a new reason breaks every `case` that has not been updated:

The error type is yours. Declare one constructor per failure reason, so a caller can tell them apart:

```cinch
  data OrderError where
    EmptyOrder :: OrderError
    TooMany :: Nat -> OrderError

  validate-count :: Nat -> Result OrderError Nat
  validate-count n =
    if n == 0 then Err EmptyOrder
    else if 12 < n then Err (TooMany n)
    else Ok n

  total-of :: Nat -> Nat -> Result OrderError Nat
  total-of price count = case validate-count count of
    Err error -> Err error
    Ok valid -> Ok (price * valid)
```

`TooMany n` keeps the rejected count, so the caller can say which count was rejected instead of guessing. `total-of` is the propagation step: it matches the first result and re-wraps `Err error` unchanged, which costs one branch and loses nothing. The successful total is a `Nat` like any other, so a downstream function that needs the bound still checks it or takes a refined argument. `either` collapses both branches in one expression, with the error function first.

The combinators in `Data.Result` mirror the `Option` ones, with the error in the first position throughout:

| Name | Signature | Result |
| --- | --- | --- |
| `either` | `(e -> b) -> (a -> b) -> Result e a -> b` | collapse both branches |
| `is-ok`, `is-err` | `Result e a -> Bool` | tag test |
| `map-result`, `map-err` | one branch transformed | value or error mapped |
| `from-result` | `a -> Result e a -> a` | default, discarding the error |
| `results`, `errors` | `List (Result e a) -> List _` | one side only |
| `partition-results` | `List (Result e a) -> (List e, List a)` | both sides, in order |
| `result-to-option` | `Result e a -> Option a` | error dropped |
| `option-to-result` | `e -> Option a -> Result e a` | absent becomes this error |

# Propagate a failure instead of guessing

Each step of a dependent sequence needs the previous step's value, so each step is a `case` on the previous result. Three steps nest three `case` expressions. The `do` form removes the nesting once a provider is selected by a `using` binder on the definition: `using (seq :: ResultSequencing Failure) => ...` makes `x <- action` bind the `Ok` payload and makes an `Err` leave the block immediately, with `pure` as the injection. A definition cannot mix two providers, because the provider is chosen lexically. The rules are [chapter 11](11-handlers-and-sequencing.md); the complete program below uses explicit `case` so that nothing depends on that later material.

The provider short-circuits, and that is the difference from validation. It reports the first failure and stops, so it suits a pipeline where step two needs step one's output. Independent fields need every failure, which means evaluating the checks separately and combining the results yourself, for example by matching a pair of results and collecting both error values. A fail-fast sequence is not an accumulating validation.

# Validate at the boundary and refine the result

Some properties are worth carrying in the type, so that a later function does not recheck them. `NonZero a` and `NonEmpty a` are refinements of an existing type rather than new data types, and the total constructors that produce them return `Option`, because the property may fail:

| Name | Module | Signature |
| --- | --- | --- |
| `non-zero` | `Data.NonZero` | `Int -> Option (NonZero Int)` |
| `non-zero-nat` | `Data.NonZero` | `Nat -> Option (NonZero Nat)` |
| `non-empty` | `Data.NonEmpty` | `forall a. List a -> Option (NonEmpty a)` |

Both modules need an explicit `import`, because `Prelude` does not re-export them. A `NonZero Nat` is what `nat-div` requires of its divisor, so a validated divisor is the value you pass, and a plain `Nat` is refused with an obligation. A numeral is the exception: `nat-div 8 2` decides `NonZero 2` at compile time and needs no evidence or match.

A `NonEmpty a` is a `List a` that is known not to be empty, with the witness erased at runtime, so passing it to a function costs nothing. It is subject to the ordinary writer rules; if the underlying list were mutable, the witness would die with the write.

# A validation error is not a type error

The two live in different worlds and the difference decides how you write the caller.

| | Validation error | Type error |
| --- | --- | --- |
| When it exists | at run time, as a value | never; the program is refused |
| Where it appears | inside a `Result e a` you return | in the compiler's diagnostic output |
| Who handles it | the caller, by matching `Err` | nobody; there is no handling form |
| How it is tested | run the program | run `cinch check` |
| Example | `TooMany 20` | `cinch: 6:15: type mismatch: ...` |

A rejected user input, a missing file and a failed parse are validation errors: return them. A `Nat` subtraction whose operands are unordered, a divisor that is a plain `Nat`, and a branch that returns the wrong type are type errors: fix the program. Overflow is a third case, and [chapter 7](07-numbers-text-and-bytes.md) shows the checked operations that turn it into an `Option`.

# The complete program

The program validates each line of an order, propagates the first failure through a list recursion, divides by a count that has been refined to non-zero, and refines a list to non-empty at the boundary. It imports the two refinement modules and reports through its exit status.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.NonZero
  import Data.NonEmpty

  data OrderError where
    EmptyOrder :: OrderError
    TooMany :: Nat -> OrderError

  record Line where
    price :: Nat
    quantity :: Nat

  validate-count :: Nat -> Result OrderError Nat
  validate-count n =
    if n == 0 then Err EmptyOrder
    else if 12 < n then Err (TooMany n)
    else Ok n

  line-total :: Line -> Result OrderError Nat
  line-total line = case validate-count line.quantity of
    Err error -> Err error
    Ok valid -> Ok (line.price * valid)

  lines-total :: List Line -> Result OrderError Nat
  lines-total lines = case lines of
    Nil -> Ok 0
    Cons line rest -> case line-total line of
      Err error -> Err error
      Ok subtotal -> case lines-total rest of
        Err error -> Err error
        Ok remaining -> Ok (subtotal + remaining)

  sample :: List Line
  sample = [Line { price = 7, quantity = 2 }, Line { price = 8, quantity = 3 }]

  broken :: List Line
  broken = [Line { price = 7, quantity = 0 }]

  per-line :: Nat -> Nat -> Option Nat
  per-line amount lines = case non-zero-nat lines of
    None -> None
    Some divisor -> Some (nat-div amount divisor)

  at-least-one :: List Line -> Option (NonEmpty Line)
  at-least-one lines = non-empty lines

  main :: Int
  main =
    case lines-total sample of
      Err _ -> 1
      Ok total -> case lines-total broken of
        Ok _ -> 2
        Err error -> case error of
          TooMany _ -> 3
          EmptyOrder -> case per-line total 2 of
            None -> 4
            Some each -> case at-least-one sample of
              None -> 5
              Some group -> if each == 19 then 0 else 6
```

Read it in source order. The two `import` lines bring the refinement constructors in; without them `non-zero-nat` and `non-empty` are unknown names. `OrderError` names the two ways a count can be rejected, and `TooMany` carries the count. `Line` is a record, so a value is built with `Line { price = 7, quantity = 2 }`. `validate-count` is the boundary rule in one place: zero, above twelve, or accepted. `line-total` propagates the error unchanged and multiplies only on `Ok valid`. `lines-total` recurses over the list, matching each line before recursing, so the first `Err` in the order becomes the result and the remaining lines are not validated. `sample` is valid and `broken` contains a zero quantity, which is what the second half of `main` exercises.

`per-line` divides the total by the line count, and it can only do that after `non-zero-nat` produced a `NonZero Nat`, so no zero divisor reaches `nat-div`. `at-least-one` turns the list into a `NonEmpty Line` or reports `None`. `main` reads as the sequence of decisions: a failed valid total is status 1, an accepted broken order is status 2, a wrong error constructor is status 3, an absent refinement is status 4 or 5, and success is 0.

# What the compiler enforces

- **Both constructors of `Option` and `Result` are covered.** Omitting `None` or `Err` is a non-exhaustive match, reported with the constructor name.
- **The error type is part of the result type.** A branch returning a `Result` with a different `e` is a type mismatch, so a caller can rely on the error constructors it matches.
- **The success value keeps its plain type.** `Ok valid` is a `Nat`, which is why a bound survives only if the type says so.
- **No `Eq` or `Ord` instance exists for `Option` or `Result`.** A comparison has to be written as a `case`, so an absent or failed value cannot be skipped accidentally.
- **A refined value cannot be forged.** `NonZero n` and `NonEmpty xs` are produced by `non-zero`, `non-zero-nat`, `non-empty`, or a compile-time decision on a numeral, and each total constructor answers `Option`.
- **A `Nat` divisor is refused where a `NonZero Nat` is required.** The obligation names the predicate, and `decide` or a numeral discharges it.

# Diagnostics you will meet

Each message is copied from `cinch check Main.cinch` on a file named `Main.cinch`.

Omitting the absent branch reports `non-exhaustive patterns: constructor None is not covered`, naming the constructor the match is missing. Comparing optional values reports `no instance for Eq (Option Nat)` rather than a type error about `Some`.

Returning the payload instead of the optional value reports `cinch: 5:19: type mismatch: expected Nat, got Option Nat (in Main.cinch)`.

Forgetting `Ok` in a `Result` function reports the whole result type:

```text
cinch: 6:15: type mismatch: expected Nat, got Result Main.Failure Nat (in Main.cinch)
```

Dividing by a plain `Nat` reports the predicate that is missing, in the same shape as the subtraction obligation from chapter 5:

```text
cinch: 3:38: the obligation `NonZero parts` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide (NonZero parts) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `NonZero parts` is in scope here (in Main.cinch)
```

# Exercises

### Exercise 1

Write `first-positive :: List Nat -> Option Nat` returning the first element greater than zero, and `describe :: Option Nat -> String` returning `"none"` or the number in decimal. Use the combinators rather than explicit recursion for `first-positive`.

<details><summary>Answer</summary>

`find` from `Data.List` is the search, and it already answers `Option`:

```cinch
  first-positive :: List Nat -> Option Nat
  first-positive xs = find (\x -> 0 < x) xs

  describe :: Option Nat -> String
  describe found = maybe "none" (\value -> nat-to-string value) found
```

`maybe` takes the absent answer first and the function for the present value second, which keeps both branches in one expression.

</details>

### Exercise 2

A payment of `amount` is split across `parts` shares. Write `split :: Nat -> Nat -> Result SplitError Nat` that rejects a zero amount, rejects a zero share count, and otherwise returns the amount per share, with the remainder discarded. Name the error type and its constructors.

<details><summary>Answer</summary>

```cinch
  data SplitError where
    NoAmount :: SplitError
    NoParts :: SplitError
    TooManyParts :: Nat -> SplitError

  split :: Nat -> Nat -> Result SplitError Nat
  split amount parts =
    if amount == 0 then Err NoAmount
    else if parts == 0 then Err NoParts
    else if 1000 < parts then Err (TooManyParts parts)
    else case non-zero-nat parts of
      None -> Err NoParts
      Some divisor -> Ok (nat-div amount divisor)
```

The explicit `parts == 0` test exists because the function must report a reason, and `non-zero-nat` reports only absence. `TooManyParts` follows the `TooMany` pattern: a rejected value travels with the failure. The division keeps its `NonZero` argument, so the successful branch cannot divide by zero. The answer needs `import Data.NonZero`.

</details>

[Chapter 7](07-numbers-text-and-bytes.md) turns to the values themselves: which arithmetic type to choose, how a machine overflow becomes an `Option`, and how text and bytes cross a boundary without inventing characters.
