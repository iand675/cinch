# 4. Model data and handle every case

A data declaration introduces a type and the constructors that build its values. A constructor can hold other values. Pattern matching reverses construction: it tells you which constructor you have and binds names for the stored fields.

# Alternatives and products

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Delivery where
    Collect :: Delivery
    Ship :: Nat -> Delivery

  delivery-cost :: Delivery -> Nat
  delivery-cost method = case method of
    Collect -> 0
    Ship distance -> distance * 2

  main :: Int
  main = if delivery-cost (Ship 4) == 8 then 0 else 1
```

`Delivery` has two alternatives. `Collect` has no fields. `Ship :: Nat -> Delivery` stores a distance. In the `Ship distance` branch, `distance` names that field. Each branch must return a `Nat`, because that is the result of `delivery-cost`.

A tuple combines a fixed set of values: `(True, 7)` has a type such as `(Bool, Int)`. Match it with `(flag, count)`. Tuples are structural: you do not declare a named type to use them. Prefer a record when the meaning of each position would be hard to remember.

# Named fields

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Item where
    price :: Nat
    quantity :: Nat

  total :: Item -> Nat
  total item = item.price * item.quantity

  main :: Int
  main = if total (Item { price = 7, quantity = 3 }) == 21 then 0 else 1
```

A record is a named product. Construction names the fields, and `item.price` projects one. Named-field patterns such as `Item { quantity = n, price = p }` bind fields by name rather than by the order written in the pattern. Construction must supply the required fields exactly once. This tutorial constructs a new record when a value changes; do not assume Haskell's record-update syntax is supported.

# Coverage and order

Patterns are tried in source order. `_` matches a value without naming it. `whole@pattern` retains the entire matched value as well as its parts. A catch-all row belongs after more specific rows; a later row cannot override an earlier catch-all.

The compiler requires coverage. Omitting `Collect` from `delivery-cost` is an error. A guarded row such as `Ship n | n > 0 -> ...` does not cover every `Ship`: the guard can be false. Add an unguarded row for the remaining case. Guards are ordinary `Bool` expressions and their effects count towards the surrounding function.

Cinch also supports equations with patterns, for example `delivery-cost Collect = 0`. The same source-order and coverage rules apply. Do not repeat a variable in a pattern to ask for equality; use two names and compare them in a guard or expression.

# Exercise and answer

Add `Express :: Nat -> Delivery`. Charge `5 + distance * 3`. What must change besides the declaration?

<details><summary>Answer</summary>

Add `Express distance -> 5 + distance * 3` to `delivery-cost`. Without that branch, a value can be constructed that the function does not handle, so coverage fails. Add a check such as `delivery-cost (Express 2) == 11` to `main`.

</details>
