# 22. Build a small order calculator

This project combines named data, pure functions, explicit errors, list traversal, and I/O. It deliberately keeps the calculation pure and places the effect at the entry point, so you can test the business rules with ordinary function calls.

# The requirements

An order is a list of lines. Each line has a unit price and quantity. A line with quantity zero is invalid. A valid order totals all line subtotals. The program should print 38 for two units at 7 and three units at 8, followed by a newline, and exit 0. An invalid order should print an explanation and exit 1.

# A complete implementation

<!-- cinch:run exit=0 stdout="38\n" -->
```cinch
module Main where
  record Line where
    price :: Nat
    quantity :: Nat

  data OrderError where
    ZeroQuantity :: OrderError

  line-total :: Line -> Result OrderError Nat
  line-total line =
    if line.quantity == 0 then Err ZeroQuantity
    else Ok (line.price * line.quantity)

  order-total :: List Line -> Result OrderError Nat
  order-total lines = case lines of
    Nil -> Ok 0
    Cons line rest -> case line-total line of
      Err error -> Err error
      Ok subtotal -> case order-total rest of
        Err error -> Err error
        Ok remaining -> Ok (subtotal + remaining)

  sample :: List Line
  sample = [Line { price = 7, quantity = 2 }, Line { price = 8, quantity = 3 }]

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in case order-total sample of
    Err ZeroQuantity ->
      let written = ioWrite (string-to-bytes "Quantity must be positive\n")
      in 1
    Ok total ->
      let written = ioWrite (string-to-bytes (string-concat (nat-to-string total) "\n"))
      in 0
```

The empty order totals zero. Each recursive step validates one line before adding its subtotal. The first invalid line returns `Err`, and the caller propagates it. No error path fabricates a successful total. `main` chooses the output policy and exit status after the pure computation has finished.

# Test the boundaries

Change one quantity to zero; expect the error text and exit 1. Use `Nil` as the sample; expect `0` and exit 0. Use a very large mathematical price; multiplication remains mathematical until process resource limits are reached. Replace a quantity with `-1`; checking should fail because `Nat` is nonnegative.

An empty order is accepted because that is the stated rule, not because the type system inferred an application policy. If the business requires at least one line, validate into `NonEmpty Line` at the input boundary and keep that distinction in the API.

# Extend it in stages

First move `Line`, `OrderError`, and the pure functions into an `Orders` module with an explicit export list. Keep `main` in a separate module. Then add a discount rule with a clear nonnegative-subtraction policy. Finally read input bytes using `Effect.Fs` or process arguments and decode them with a parser returning `Result`.

For a source-distributed parser module, copy its dependency closure into the project using the [library instructions](../library/00-index.md). Do not assume the sample package website installs it. Keep parsing errors distinct from business validation errors so that the caller can report which boundary rejected the input.

# Where to go next

Use the [compiler guide](../reference/00-guide.md) for exact rules, the [real library directory](../library/00-index.md) for available signatures, and the [support matrix](../reference/16-support-and-roadmap.md) for planned features marked **Coming soon**. The original design tutorials remain available as background; they are not the authority for what the Rust compiler accepts today.
