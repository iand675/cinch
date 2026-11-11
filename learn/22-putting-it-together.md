# 22. Build a small order calculator

Everything in this book so far has been one construct at a time. This chapter builds a program: two modules, a record for the input, a refinement for the rule that makes a line meaningful, an error type for the cases the business rejects, and a single place where the program decides what to print and which exit status to return. The calculation stays pure so you can test it without a terminal, and the policy sits at the entry point where it belongs.

Prerequisites: [modules](08-modules.md), [options, results, and validation](06-options-results-and-validation.md), [refinements](13-refinements-and-safe-operations.md), and [effects and IO](10-effects-and-io.md).

# The requirements

An order is a list of lines. Each line has a unit price and a quantity, both `Nat`. A line with quantity `0` is invalid and the order is rejected as a whole. A valid order totals the subtotals of its lines.

The program is given two orders, one valid and one invalid. It prints one line per order: the total for a valid order, and a rejection with the price of the offending line for an invalid one. It exits `1` if any order in the batch was rejected, so a shell can tell that the report contains a rejection without parsing the text.

For the sample orders in this chapter the program prints exactly

```text
38
rejected: a line at price 8 has quantity zero
```

and exits `1`. The `38` is `7 * 2 + 8 * 3`.

# The data module

The records, the error type, and the pure calculation live in `Orders.cinch`:

<!-- cinch:project group=orders file=Orders.cinch -->
```cinch
module Orders where
  export Line, Order, OrderError, ZeroQuantity, line-total, order-total

  record Line where
    price :: Nat
    quantity :: Nat

  record Order where
    lines :: List Line

  type Positive = refine (n :: Nat) with n > 0

  data OrderError where
    ZeroQuantity :: Nat -> OrderError

  positive :: Nat -> Option Positive
  positive n = case decide (n > 0) of
    Yes witness -> Some (Positive n witness)
    No _ -> None

  line-total :: Line -> Result OrderError Nat
  line-total line = case positive line.quantity of
    None -> Err (ZeroQuantity line.price)
    Some quantity -> Ok (line.price * unrefine quantity)

  total-of :: List Line -> Result OrderError Nat
  total-of lines = case lines of
    Nil -> Ok 0
    Cons line rest -> case line-total line of
      Err error -> Err error
      Ok subtotal -> case total-of rest of
        Err error -> Err error
        Ok remaining -> Ok (subtotal + remaining)

  order-total :: Order -> Result OrderError Nat
  order-total order = total-of order.lines
```

The export list is the module's contract. `Line`, `Order`, `OrderError`, the `ZeroQuantity` constructor, `line-total`, and `order-total` leave the module. `Positive`, `positive`, and `total-of` do not: they are implementation, and a caller that cannot name them cannot depend on them. That freedom is the reason to write the list at all.

`Positive` is a refinement, not a wrapper: `Positive n witness` is the numeral `n` with erased evidence that it is above zero, and `unrefine` returns the carrier. `positive` produces one with `decide`, which hands the witness to the `Yes` branch. Everything downstream is total arithmetic.

# The entry point

`Main.cinch` holds no business rules. It builds the sample orders, prints a line for each, and turns the batch outcome into an exit status:

<!-- cinch:project group=orders file=Main.cinch exit=1 stdout="38\nrejected: a line at price 8 has quantity zero\n" -->
```cinch
module Main where
  import Orders

  sample :: Orders.Order
  sample = Orders.Order { lines = [Orders.Line { price = 7, quantity = 2 }, Orders.Line { price = 8, quantity = 3 }] }

  broken :: Orders.Order
  broken = Orders.Order { lines = [Orders.Line { price = 8, quantity = 0 }] }

  report :: forall r. using (io :: Io r) => Orders.Order ->{IO r} Bool
  report order = case Orders.order-total order of
    Ok total ->
      let written = ioWrite (string-to-bytes (nat-to-string total)) in
      let newline = ioWrite (string-to-bytes "\n") in
      True
    Err (Orders.ZeroQuantity price) ->
      let written = ioWrite (string-to-bytes "rejected: a line at price ") in
      let digits = ioWrite (string-to-bytes (nat-to-string price)) in
      let reason = ioWrite (string-to-bytes " has quantity zero\n") in
      False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let good = report sample in
    let bad = report broken in
    if good && bad then 0 else 1
```

Both files are one project: `cinch build Main.cinch -o example` finds `Orders.cinch` beside the entry file, checks it, and links the program. The exact stdout and exit status above are part of this book's documentation checks, so the chapter cannot drift from the program.

# Walk through the calculation

`report` asks `order-total` for one answer and handles both outcomes. `Ok total` prints the total and reports success. `Err (Orders.ZeroQuantity price)` prints the rejection, naming the price from the error payload, and reports failure. The pattern has one alternative per constructor of `OrderError`, so adding a constructor to the error type makes this function a type error until the policy for the new case is written. That is the error policy being checked rather than remembered.

`main` runs the two reports and combines them with `&&`. `good && bad` is `True` only if both orders were accepted, so a batch containing one rejection exits `1`. The `with (io = newIO ()) in` line creates the `Io` capability that `report` requires; the capability is lexical, so `report`'s signature states `using (io :: Io r) =>` and no call site can conjure an `Io` from nowhere.

`order-total` delegates to `total-of`, which is private and recursive. Its `Nil -> Ok 0` case is the empty remainder of a non-empty order, which is why the empty order is not an error here: this stated rule accepts an empty order and totals it as `0`. If the business requires at least one line, that is a policy for `order-total` to state, and the third exercise does exactly that.

# What the refinement buys

`line-total` asks `positive` for evidence and multiplies only when it receives it. There is no `if line.quantity == 0` test anywhere, and no path on which a rejected quantity reaches the multiplication. The quantity that multiplies is the one the witness describes, because `unrefine quantity` projects the same value the predicate examined.

The decision is not free: `decide (n > 0)` is a runtime comparison, and it runs for every line. What the refinement removes is the possibility of the check being skipped or inverted later, since the multiplication's argument has type `Positive` and only the `Yes` branch constructs one.

# What the compiler enforces

- The export list is the module's public surface. Naming a constructor outside it fails, even when its type is exported.
- A refinement construction needs a discharge the checker has settled. `decide` supplies it only inside the branch that received the witness.
- `Result` alternatives are exhaustive: a new `OrderError` constructor breaks the caller until it is handled.
- An `Io` operation needs an `Io` capability in scope, and a function that performs one states the capability in its `using` row.
- A module split is checked as one project: `cinch build Main.cinch` resolves `import Orders` from the sibling file with no extra flag.

# Diagnostics you will meet

Three failures you will hit while extending this project, each copied from a real run:

| You wrote | The compiler says |
| --- | --- |
| `Err (Orders.ZeroQuantity price)` with `OrderError` but not `ZeroQuantity` in the export list | ``constructor ZeroQuantity is not exported by Orders`` |
| `Positive 0 (decide (0 > 0))` | ``the discharge `0 > 0` is disproved: no refined value can be constructed here (a failed check must not produce evidence)`` |
| `ioWrite` in a function without the capability | ``no Io r capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause`` |
| `total n = positive n` at result type `Positive` | ``type mismatch: expected Option Positive, got Positive`` |

# Extend it in stages

**Exercise 1.** Add a discount: `apply-discount :: Nat -> Nat` subtracts five units, and never goes below zero. Wire it into `order-total` and predict the new output.

<details><summary>Answer</summary>

Add to the export list and the module:

```text
  apply-discount :: Nat -> Nat
  apply-discount total = if 5 <= total then total - 5 else 0

  order-total :: Order -> Result OrderError Nat
  order-total order = case total-of order.lines of
    Err error -> Err error
    Ok total -> Ok (apply-discount total)
```

The subtraction needs evidence that the subtrahend does not exceed the minuend, and the `if` condition `5 <= total` supplies exactly that branch fact, so no `decide` is needed. The rejected order never reaches the discount, because the error branch returns first. The output becomes `33` followed by the rejection line, still exiting `1`, since the valid order totals `38 - 5`.

</details>

**Exercise 2.** Make an empty order an error without making every order an error.

<details><summary>Answer</summary>

Add `EmptyOrder :: OrderError`, export `EmptyOrder`, and decide emptiness once in the entry to the calculation:

```text
  order-total :: Order -> Result OrderError Nat
  order-total order = case order.lines of
    Nil -> Err EmptyOrder
    Cons _ _ -> total-of order.lines
```

Do not change `total-of`'s `Nil -> Ok 0`. That case is the empty remainder of a non-empty list, reached once per line, so rejecting it there rejects every order. The bug is instructive: `Nil` means two different things at the two levels, and only the outer level knows which one it is. With the fix, `report` needs one more alternative, and the sample order still prints `38` while an empty order prints `rejected: the order has no lines`.

</details>

**Exercise 3.** Read the orders from the command line instead of the two hardcoded values.

<details><summary>Answer</summary>

`process-args :: using (io :: Io r) => Unit ->{IO r} Result ProcessError (List String)` is the binding, from `Effect.Process`. Call it inside `main`'s `with (io = newIO ()) in` scope and match the `Result`: `Err ProcessInvalidArgument` means a word was not valid UTF-8, which the boundary refuses instead of substituting text. Each word then needs decoding into a `Nat`, which is a small loop over `string-char-at` that folds the digits and returns `Result` for a non-digit. Keep that parse error distinct from `OrderError`: the parse failed at the input boundary and the business rule failed later, and the caller can report which. Run the result with `cinch run Main.cinch -- 7 2 8 3`; the `--` ends the compiler's own flags.

</details>

# Where to go next

The [compiler guide](../reference/00-guide.md) states the exact rules behind each construct in this chapter. The [library index](../library/00-index.md) lists the signatures that exist, with fixities and instances, and the [support matrix](../reference/16-support-and-roadmap.md) separates today's behaviour from the features marked **Coming soon**. Reference pages name the Rust modules and test suites behind each claim, so a statement you doubt can be traced to code you can run.
