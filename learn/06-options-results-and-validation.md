# 6. Missing values, errors, and validation

A missing map key is an ordinary possibility, not a compiler failure. Represent such possibilities in the result type. `Option a` says that a value might be absent. `Result e a` says that a computation either succeeds with an `a` or fails with an explanation of type `e`.

# Option forces a decision

`Some x` carries a value; `None` carries none. Pattern matching makes the caller choose what to do in both cases. A default is one policy, propagation is another, and an error can be a third. Choose based on the application rather than inserting a convenient zero everywhere.

# Give errors meaning

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data OrderError where
    EmptyOrder :: OrderError
    TooMany :: Nat -> OrderError

  validate-count :: Nat -> Result OrderError Nat
  validate-count n =
    if n == 0 then Err EmptyOrder
    else if n > 12 then Err (TooMany n)
    else Ok n

  main :: Int
  main = case validate-count 4 of
    Err _ -> 1
    Ok count -> if count == 4 then 0 else 2
```

A failure can carry the value that caused it. `TooMany 15` retains the attempted count, so a caller can produce an explanation instead of guessing what failed. The success branch exposes a validated count; its type is still `Nat`. If downstream code must rely on the upper bound without rechecking it, a refinement is the next step, covered in [chapter 13](13-refinements-and-safe-operations.md).

# Compose dependent checks

For a sequence where step two needs step one's result, pattern match on the first `Result`, propagate `Err e`, and continue from `Ok x`. Cinch's `do` notation can express the same dependency with an explicit `ResultSequencing e` provider. We introduce provider selection after modules and signatures so that the short syntax does not hide the mechanism.

The provider short-circuits: once a step returns `Err`, later steps do not run. This is suitable for reading bytes before parsing them. It does not accumulate independent validation errors. For independent fields, evaluate the checks separately and combine their results explicitly, for example by matching on a pair of results and collecting both error values. Do not describe fail-fast sequencing as accumulating validation.

# Keep expected failure out of the fault path

A parse error, missing file, or rejected user choice should appear in a return value. Machine overflow has checked operations returning `Option`; a possibly zero divisor can be validated into `NonZero`. A compiler diagnostic is for an invalid program. Runtime exhaustion is a separate process outcome. Treating these as interchangeable makes callers harder to write correctly.

# Exercise and answer

Write `describe :: Result OrderError Nat -> Int` that returns `0` on success, `1` on `EmptyOrder`, and `2` on `TooMany _`.

<details><summary>Answer</summary>

Match `Ok _ -> 0` and `Err e -> case e of EmptyOrder -> 1; TooMany _ -> 2`, with the inner alternatives indented beneath the inner `case`. A single `Err _ -> 1` would discard the distinction that the error type provides.

</details>
