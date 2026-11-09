# 4. Model data and handle every case

A `data` declaration introduces a type and the constructors that build its values. A `record` declaration introduces a type with named fields. `case` takes a value apart again, and the checker refuses a function that forgets an alternative. This chapter builds a program with both declarations, a nested pattern, and a total function, using:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

Prerequisites: [chapter 1](01-first-program.md), [chapter 2](02-values-and-layout.md), and [chapter 3](03-functions.md). You need module layout, signatures, values, and the indentation rule from those chapters.

# A complete program

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Item where
    price :: Nat
    quantity :: Nat

  data Delivery where
    Collect :: Delivery
    Ship :: Nat -> Delivery
    Express :: Nat -> Delivery

  fee :: Delivery -> Nat
  fee method = case method of
    Collect -> 0
    Ship distance -> distance * 2
    Express distance -> 5 + distance * 3

  total :: Item -> Delivery -> Nat
  total item method = item.price * item.quantity + fee method

  main :: Int
  main =
    if total (Item { price = 7, quantity = 3 }) (Express 2) == 32
    then 0
    else 1
```

The file starts with `record Item where`, which declares the fields `price :: Nat` and `quantity :: Nat`. Then `data Delivery where` declares three constructors: `Collect` with no fields, and `Ship` and `Express` with one `Nat` each. `fee :: Delivery -> Nat` examines its argument with `case` and returns `0`, `distance * 2`, or `5 + distance * 3`. `total :: Item -> Delivery -> Nat` projects both fields with `item.price` and `item.quantity`, multiplies them, and adds `fee method`. `main :: Int` constructs `Item { price = 7, quantity = 3 }` and `Express 2`, passes them to `total`, and exits 0 because `7 * 3` is `21` and the express fee is `11`.

# data declarations

`data` names a type and lists its constructors, one per line, each with a type:

| Line | Meaning |
| --- | --- |
| `data Delivery where` | opens the declaration and names the type |
| `Collect :: Delivery` | a constructor with no fields |
| `Ship :: Nat -> Delivery` | a constructor holding one `Nat` |
| `Express :: Nat -> Delivery` | a second constructor holding one `Nat` |

A constructor's type lists its fields and ends with the type it builds: `Ship :: Nat -> Delivery` takes one `Nat` and produces a `Delivery`. Constructors are written with a capital letter and are applied like functions, so `Ship 4` is a `Delivery`. A data type with several constructors describes alternatives, and a value is exactly one of them.

One constructor may hold several fields. In a `data Span where` declaration, `Line :: Nat -> Nat -> Span` builds a `Span` from two numbers.

# case and patterns

`case` examines a value and selects an alternative by constructor:

Fragment, from `fee` above:

```cinch
    case method of
      Collect -> 0
      Ship distance -> distance * 2
      Express distance -> 5 + distance * 3
```

The alternatives are indented under `of` and aligned with each other. Each one is a pattern, then `->`, then an expression. A pattern that names a constructor with the same number of fields as that constructor has binds those fields: `Ship distance` binds the stored `Nat` to `distance`. All alternatives must produce the same type, here `Nat`.

Alignment is a requirement. Indenting one alternative further than the others makes the compiler read it as a continuation of the previous one, and the report is ``cinch: Main.cinch: 9:11: expected `;` or `}` in the alternative block, found Arrow``.

Patterns also match tuples. `case pair of (left, right) -> left + right` binds both halves, and a `let` may destructure the same way: `let (flag, count) = (True, 7) in count`.

# Records

`record` declares a named product. Each field has a type. The order of the fields in the declaration does not constrain construction or patterns, which use the names.

Construction names every field: `Item { price = 7, quantity = 3 }`. Fields may appear in any order, and the checker rejects a construction that omits one:

| Message | What produced it |
| --- | --- |
| `cinch: 7:14: record construction is missing field quantity (in Main.cinch)` | `Item { price = 7 }` |
| `cinch: 6:32: unknown record field quantity on price (in Main.cinch)` | a field name the record does not declare |

Projection reads one field with a dot: `item.price`. That is an expression of the field's type, and it needs no pattern match.

A pattern may name fields, and it may list them in any order:

Fragment, from a function that checks whether an item is free:

```cinch
    case item of
      Item { quantity = 0, price = p } -> True
      Item { quantity = n, price = p } -> False
```

`Item { quantity = n }` alone is not a valid pattern: like construction, a record pattern must name every field, and the checker reports `cinch: 8:5: record pattern is missing field price (in Main.cinch)`. Changing one field means constructing the whole record again and reading the untouched fields from the old value:

Fragment:

```cinch
  reprice :: Item -> Nat -> Item
  reprice item newprice = Item { price = newprice, quantity = item.quantity }
```

# Nested patterns

A pattern may match the fields of a constructor in place, without a second `case`. Here the field of `Held` is itself a `Box`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Box where
    Empty :: Box
    Held :: Box -> Box

  weight :: Box -> Nat
  weight box = case box of
    Empty -> 1
    Held Empty -> 11
    Held (Held inner) -> 21

  main :: Int
  main = if weight (Held (Held Empty)) == 21 then 0 else 1
```

`Held Empty` is one pattern that requires a `Held` whose field is `Empty`, and `Held (Held inner)` requires two layers. A record pattern nests inside a constructor pattern too: with a record `Pair` holding `left` and `right`, `Box Pair { left = 0, right = r }` matches a box that contains a record whose `left` field is `0`.

# Coverage

The checker requires every constructor of the matched type to be handled. Deleting the `Collect` alternative from `fee` is an error:

<!-- cinch:reject contains="non-exhaustive patterns: constructor Collect is not covered" -->
```cinch
module Main where
  data Delivery where
    Collect :: Delivery
    Ship :: Nat -> Delivery

  fee :: Delivery -> Nat
  fee method = case method of
    Ship distance -> distance * 2

  main :: Int
  main = if fee (Ship 4) == 8 then 0 else 1
```

The full text is `cinch: 7:16: non-exhaustive patterns: constructor Collect is not covered (in Main.cinch)`. Coverage is checked on the patterns, not on which values you believe you construct, so adding a constructor to a `data` declaration forces an update at every `case` that matches it.

A guarded alternative counts as covering nothing, because its guard can be false. Add an unguarded alternative for the case the guard rejects:

<!-- cinch:reject contains="non-exhaustive patterns: constructor Small is not covered by an unguarded row" -->
```cinch
module Main where
  data Size where
    Small :: Nat -> Size
    Huge :: Size

  band :: Size -> Nat
  band size = case size of
    Small n | n > 100 -> 3
    Huge -> 2

  main :: Int
  main = if band (Small 5) == 1 then 0 else 1
```

The message names the constructor and explains the rule: ``cinch: 7:15: non-exhaustive patterns: constructor Small is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail); add an unguarded final row such as `_ -> ...` (in Main.cinch)``. The guard here is `n > 100`, which is false for `Small 5`, so a second `Small` alternative with no guard is required.

# Order, wildcards, and equations

Alternatives are tried in source order, and the first match wins. A catch-all `_` before a constructor alternative makes the later alternative unreachable: a `case` whose first alternative is `_ -> 1` returns `1` for `On` as well, even when `On -> 0` follows. Put `_` last.

`_` matches without binding. `whole@pattern` binds the whole matched value as well as its parts, so `whole@(Ship distance) -> distance` may use `whole` and `distance` in the same body.

A function may also be written as several equations with patterns instead of one `case`:

Fragment:

```cinch
  delivery-cost :: Delivery -> Nat
  delivery-cost Collect = 0
  delivery-cost (Ship distance) = distance * 2
```

A constructor pattern in an equation needs parentheses; a bare constructor name does not. Source order and coverage rules are the same as inside `case`.

A variable may appear only once per pattern. `case pair of (n, n) -> True` is rejected with `cinch: 4:9: variable n is bound twice in the same pattern (in Main.cinch)`. Use two names and compare them in a guard or in the body.

A constructor pattern must supply one subpattern per field, no more and no fewer. Writing `Ship a b` for the one-field constructor `Ship` is rejected with `cinch: 9:5: constructor Ship expects 1 field patterns, got 2 (in Main.cinch)`.

# What the compiler enforces

- A `case` or a pattern-equation group covers every constructor of the matched type, with an unguarded alternative for anything a guard can exclude.
- Every alternative produces the same type, and that type matches the enclosing signature.
- A record construction names every declared field, and so does a record pattern; neither may name a field the record does not declare.
- A pattern binds each variable at most once, and a constructor pattern has exactly as many subpatterns as the constructor has fields.
- Alternatives are ordered, and an earlier alternative shadows any later one.

# Diagnostics you will meet

| Message | What produced it |
| --- | --- |
| `cinch: 7:16: non-exhaustive patterns: constructor Collect is not covered (in Main.cinch)` | a `case` on `Delivery` with only a `Ship` alternative |
| ``cinch: 7:15: non-exhaustive patterns: constructor Small is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail); add an unguarded final row such as `_ -> ...` (in Main.cinch)`` | a `Small n` alternative whose only row is guarded by `n > 100` |
| `cinch: 7:14: record construction is missing field quantity (in Main.cinch)` | `Item { price = 7 }` |
| `cinch: 8:5: record pattern is missing field price (in Main.cinch)` | the pattern `Item { quantity = n }` |
| `cinch: 6:32: unknown record field quantity on price (in Main.cinch)` | a construction naming a field the record does not have |
| `cinch: 4:9: variable n is bound twice in the same pattern (in Main.cinch)` | `(n, n)` |
| `cinch: 9:5: constructor Ship expects 1 field patterns, got 2 (in Main.cinch)` | `Ship a b` for the one-field constructor `Ship` |

# Exercises

1. Add `Drone :: Nat -> Delivery`, charge `9 + distance * 4` for it, and explain what `cinch check` reports if you add the constructor and forget the alternative.

<details><summary>Answer</summary>

Add the constructor line to `data Delivery` and `Drone distance -> 9 + distance * 4` to `fee`. Without the alternative, `check` reports `cinch: 13:16: non-exhaustive patterns: constructor Drone is not covered (in Main.cinch)`, because the declaration admits a value that `fee` does not handle. With both changes, `fee (Drone 2)` is `17`.

</details>

2. Rewrite `total` with a `case` on the record, using the pattern `Item { price = p, quantity = q }`, and explain why the body no longer mentions `item`.

<details><summary>Answer</summary>

`total item method = case item of Item { price = p, quantity = q } -> p * q + fee method`. The pattern binds the two fields, so the body uses `p` and `q` directly and `item` is only the scrutinee. The result is unchanged: `total (Item { price = 7, quantity = 3 }) (Ship 1)` is `23`.

</details>

Next: [recursion, lists, and maps](05-recursion-and-collections.md).
