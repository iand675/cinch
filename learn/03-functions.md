# 3. Functions, types, and local definitions

A function describes how to compute a result from its inputs. It is also a value: you can name it, pass it to another function, or return it. Cinch functions are curried, so a two-argument function can be applied one argument at a time.

# Read an arrow type

`Int -> Int -> Int` means `Int -> (Int -> Int)`. The first application supplies an integer and returns a function waiting for the second integer. Parentheses on the left of an arrow mean that the argument itself is a function: `(Int -> Int) -> Int`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  add :: Int -> Int -> Int
  add x y = x + y

  apply-twice :: (Int -> Int) -> Int -> Int
  apply-twice f x = f (f x)

  main :: Int
  main =
    let add-three = add 3
    in if apply-twice add-three 4 == 10 then 0 else 1
```

`add 3` is a partial application. It does not add anything yet; it returns a function equivalent to `\y -> 3 + y`. Calling that function twice adds six to the starting value.

A lambda writes a function without a top-level name: `\x -> x + 1`. `\x y -> x + y` accepts two arguments. Lambdas follow the same type rules as named functions. If a lambda captures a resource with restricted ownership, its uses must also respect that resource's multiplicity; we reach that in [chapter 12](12-ownership-and-regions.md).

# Polymorphism is explicit at the top level

A function that simply returns its argument need not choose one concrete type:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  identity :: forall a. a -> a
  identity x = x

  main :: Int
  main = if identity True then identity 0 else 1
```

`forall a.` means that callers may instantiate `a` with any admissible type. Both occurrences of `a` refer to the same choice in a call. `identity True` returns a `Bool`; `identity 0` here returns an `Int`. The implementation cannot assume that `a` is numeric or compare it for equality without an appropriate constraint.

Top-level declarations state their quantifiers. Do not assume that an omitted `forall` will silently generalize a top-level signature. Local inference does generalize eligible value bindings under a value restriction, but locally annotated `forall`/`using` signatures are a current restriction. Use an explicit top-level helper when you need such a signature.

# Local helpers with where

A `where` block belongs to a defining equation. It can name a calculation used by that equation and can refer to the equation's arguments. Prefer a named helper when it gives an intermediate concept a meaningful name; use `let` when the binding naturally belongs inside a particular expression. Ordinary `let` groups are sequential. Recursive local groups use `letrec`, introduced with recursion.

# Exercise and answer

Write `compose :: forall a b c. (b -> c) -> (a -> b) -> a -> c`. Which function runs first?

<details><summary>Answer</summary>

`compose f g x = f (g x)`. `g` runs first because its result is the argument to `f`. The intermediate result has type `b`; the final result has type `c`.

</details>

For exact binder, multiplicity, and effect-arrow syntax, see [types and kinds](../reference/04-types-and-kinds.md).
