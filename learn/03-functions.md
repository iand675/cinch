# 3. Functions, types, and local definitions

A function describes how to compute a result from its inputs, and its type says how many arguments it takes and what it produces. This chapter builds a program with a two-argument function, a partial application, a higher-order function, a recursive definition, and a local helper, then reads each signature back in words. The commands stay the same:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

Prerequisites: [chapter 1](01-first-program.md) and [chapter 2](02-values-and-layout.md). You need the module layout, signatures, and `let` from those chapters.

# Read a signature

`Int -> Int -> Int` is `Int -> (Int -> Int)`. The arrow groups to the right, so a two-argument function is a one-argument function that returns another function. Every function in Cinch takes exactly one argument, and a call with several arguments is a sequence of calls.

`(Int -> Int) -> Int -> Int` puts an arrow inside parentheses on the left, which means the argument itself is a function. Parentheses on the left of a top-level arrow are how you pass a function to a function.

`forall a. a -> a` begins with a quantifier. It states that the caller chooses `a` at each use, and every `a` inside the signature refers to that one choice. The implementation cannot assume anything about `a`, so it can only return the argument it was given.

A top-level signature is not optional. The compiler rejects a declaration without one:

<!-- cinch:reject contains="missing signature for double" -->
```cinch
module Main where
  double n = n + n

  main :: Int
  main = if double 21 == 42 then 0 else 1
```

The full message is `cinch: 2:3: missing signature for double (this draft does not silently infer top-level types) (in Main.cinch)`. Write the signature first, then the equation, and let the body decide whether you wrote what you meant.

# A complete program

<!-- cinch:run exit=0 -->
```cinch
module Main where
  add :: Int -> Int -> Int
  add x y = x + y

  apply-twice :: (Int -> Int) -> Int -> Int
  apply-twice f x = f (f x)

  fact :: Nat -> Nat
  fact n = if n == 0 then 1 else n * fact (n - 1)

  main :: Int
  main =
    let add-three = add 3
        shifted = apply-twice add-three 4
    in if shifted == 10 && fact 5 == 120
       then 0
       else 1
```

`add :: Int -> Int -> Int` is defined by `add x y = x + y`, so its two parameters arrive one at a time. `apply-twice :: (Int -> Int) -> Int -> Int` is defined by `apply-twice f x = f (f x)`, which applies its function argument to its value twice. `fact :: Nat -> Nat` returns `1` when `n == 0` and otherwise multiplies `n` by `fact (n - 1)`. `main :: Int` binds `add-three` to `add 3` and `shifted` to `apply-twice add-three 4` inside a `let`, then exits 0 because `shifted == 10` and `fact 5 == 120` both hold.

# Application and partial application

Application is written with a space and binds tighter than every operator. `add 3 4` is `(add 3) 4`. The subexpression `add 3` applies `add` to one argument and produces a function; nothing is added until the second argument arrives. `let add-three = add 3` gives that function a name, and `add-three 4` then produces `7`.

Because application is curried, a function can be given fewer arguments than its type mentions and then passed around. `apply-twice add-three 4` applies `add-three` twice: `add-three 4` is `7`, then `add-three 7` is `10`.

Applying too few or too many arguments is a type error, not a runtime failure. `add 1` as the body of `main :: Int` is rejected with `cinch: 6:10: type mismatch: expected (Int -> Int), got Int (in Main.cinch)`, and `inc 1 2` where `inc :: Int -> Int` is rejected with `cinch: 6:16: cannot apply a value of type Int (in Main.cinch)`.

# Lambdas

A lambda writes a function without giving it a top-level name. `\x -> x + 1` is a function that adds one, and `\x y -> x + y` takes two arguments. Apply a lambda by wrapping it in parentheses, because the body extends as far right as it can go:

Fragment:

```cinch
    (\x -> x + 1) 41
```

That expression is `42`. Lambdas follow the same typing rules as named functions and can capture surrounding bindings, so `\y -> add base y` sees `base` from its enclosing scope.

# Recursion and termination

A recursive definition calls itself, and the body must reach a case where it does not. `fact` has two parts: the test `n == 0`, which returns `1`, and the recursive call on `n - 1`.

Two things make that accepted. The recursive argument `n - 1` is smaller than `n`, so repeated calls reach `0`. The subtraction itself is checked: `-` on `Nat` needs evidence that the result is not negative, and the `n == 0` test in the `else` branch is exactly that evidence, because the branch runs only when `n` is at least `1`.

The checker does not prove termination. A definition such as `loop n = loop n`, given a signature, typechecks and runs forever. Termination is your argument, written in a sentence like the one above, and a recursive definition without a decreasing argument compiles and hangs.

The specification treats nontermination as an effect named `Diverge`, which a function's type can carry: `spin :: Nat ->{Diverge} Nat` is a well-formed signature. <span class="support-tag support-tag--planned">Coming soon</span> Enforcement of `Diverge` on ordinary definitions is deferred, so `loop` checks today and a signature that omits the label is not rejected.

Local recursion uses `letrec`. Every name in a `letrec` group carries its own signature, and the body must sit under a lambda so that the recursive use happens when the function runs:

Fragment:

```cinch
    letrec sum-to :: Nat -> Nat -> Nat
           sum-to = \n acc -> if n == 0 then acc else sum-to (n - 1) (acc + n)
    in sum-to 10 0
```

That expression is `55`. A `letrec` binding that refers to itself without a lambda is rejected:

<!-- cinch:reject contains="unguarded recursion" -->
```cinch
module Main where
  main :: Int
  main =
    letrec f :: Int
           f = f + 1
    in f
```

The message is ``cinch: 5:12: unguarded recursion; wrap the use in `delay` or a lambda (the use of `f` closes an eager cycle in the binding group) (in Main.cinch)``.

# Local definitions

`where` attaches helpers to one equation. They can refer to that equation's parameters, and the block is indented under the equation. This fragment is a whole declaration pair:

```cinch
  total :: Nat -> Nat -> Nat
  total price quantity = subtotal + fee
    where
      subtotal :: Nat
      subtotal = price * quantity

      fee :: Nat
      fee = 3
```

`total 4 5` is `23`. Give a helper its own signature when its literals must be a `Nat`: an unannotated `fee = 3` defaults to `Int`, and the sum then fails.

<!-- cinch:reject contains="type mismatch: expected Int, got Nat" -->
```cinch
module Main where
  total :: Nat -> Nat -> Nat
  total price quantity = subtotal + fee
    where
      subtotal = price * quantity
      fee = 3

  main :: Int
  main = if total 4 5 == 23 then 0 else 1
```

That reports `cinch: 3:37: type mismatch: expected Int, got Nat (in Main.cinch)`, naming `fee` as an `Int` where the sum needs a `Nat`.

Use `where` when a helper belongs to one equation, and `let` when the binding belongs inside one expression. A `where` block can see the equation's parameters, and the equation can use its helpers as ordinary names.

# What the compiler enforces

- Every top-level declaration has a signature. Omission is an error, never an inferred type.
- A function takes exactly one argument per arrow. Supplying fewer leaves a function; supplying more is `cannot apply a value of type Int`.
- The body of a definition must produce the type its signature promises.
- `forall` at the top of a signature lets the caller choose the type; the body must work for every choice.
- A `forall` inside a parenthesized argument type, as in `(forall a. a -> a) -> Int`, lets the body choose instead: the argument must work at every type. See [polymorphic arguments](../reference/04-types-and-kinds.md#polymorphic-arguments-and-results).
- `letrec` requires a signature per name and a lambda or `delay` around every recursive use.
- Subtraction on `Nat` requires evidence that the result is not negative, which an `if` test or a guard supplies.

# Diagnostics you will meet

| Message | What produced it |
| --- | --- |
| ``cinch: 2:3: missing signature for double (this draft does not silently infer top-level types) (in Main.cinch)`` | a top-level equation with no signature |
| `cinch: 6:10: type mismatch: expected (Int -> Int), got Int (in Main.cinch)` | `add 1` used where an `Int` is required |
| `cinch: 6:16: cannot apply a value of type Int (in Main.cinch)` | `inc 1 2` where `inc :: Int -> Int` |
| `cinch: 3:14: type mismatch: expected Int, got Bool (in Main.cinch)` | `double :: Int -> Bool` with the body `n + n` |
| ``cinch: 5:12: unguarded recursion; wrap the use in `delay` or a lambda (the use of `f` closes an eager cycle in the binding group) (in Main.cinch)`` | `letrec f :: Int; f = f + 1` |
| `cinch: 3:37: type mismatch: expected Int, got Nat (in Main.cinch)` | an unannotated `where` helper whose literal defaults to `Int` |

# Exercises

1. Write `compose :: forall a b c. (b -> c) -> (a -> b) -> a -> c`, and make `main` check that composing an increment with a doubling gives the same result as applying them in order.

<details><summary>Answer</summary>

`compose f g x = f (g x)`. The inner call `g x` runs first and produces a `b`, which `f` turns into a `c`. With `inc n = n + 1` and `twice n = n * 2`, `compose twice inc 4` is `10`, so `main = if compose twice inc 4 == 10 then 0 else 1`.

</details>

2. Write `power :: Nat -> Nat -> Nat` for a base and an exponent, using only `*`, `==`, and `-`. Then explain why the `if` test is needed for the subtraction to be accepted.

<details><summary>Answer</summary>

`power base exponent = if exponent == 0 then 1 else base * power base (exponent - 1)`. The `else` branch runs only when `exponent` is not zero, and that test is the evidence the checker needs for `exponent - 1` to be a `Nat`. Without a test, `exponent - 1` is rejected because nothing shows the result is nonnegative.

</details>

For the full arrow, binder, and effect-row grammar, see [types and kinds](../reference/04-types-and-kinds.md).

Next: [model data and handle every case](04-data-and-matching.md).
