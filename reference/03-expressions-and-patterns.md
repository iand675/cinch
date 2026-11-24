# Expressions, bindings, and patterns

This chapter defines the syntax and static rules for Cinch expressions, local bindings, records, compound literals, and pattern matching. Lexical conventions are in [lexical structure and layout](02-lexical-structure.md); types, effects, ownership, and evaluation semantics are in [types and kinds](04-types-and-kinds.md), [effects and capabilities](09-effects-and-capabilities.md), [ownership and memory](10-ownership-and-memory.md), and [laziness and codata](11-laziness-and-codata.md).

# Application, operators, and projection

| Form | Syntax | Semantics |
| --- | --- | --- |
| Function application | `f x y` | Left-associative: `(f x) y`; binds tighter than all infix operators. |
| Operator as value | `(+) 2 3` | Parenthesizing an infix operator yields an ordinary prefix function. |
| Infix application | `1 + 2` | Precedence and associativity follow the operator's declared fixity. |
| Low-precedence application | `add 1 $ add 2 3` | Right-associative: `add 1 (add 2 3)`. |
| Prefix negation | `-(add 1 2)` | Binds to a single atomic term; parenthesize applications. |
| Signed literal argument | `f -42` | Applies `f` to the signed integer literal `-42`. |
| Record projection | `value.field` | Accesses a named field of a record value. |
| Named capability supply | `f @{io = cap} arg` | Supplies a named capability explicitly (see [effects and capabilities](09-effects-and-capabilities.md)). |

Spacing disambiguates hyphenated identifiers (`x-y`), subtraction (`x - y`), and signed literals (`-42`); see [lexical structure](02-lexical-structure.md).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  add :: Int -> Int -> Int
  add a b = a + b

  f :: Int -> Int
  f n = n

  a :: Int
  a = add 1 $ (+) 2 3

  b :: Int
  b = -(add 1 2)

  c :: Int
  c = f -42

  main :: Int
  main = if a == 6 && b == -3 && c == -42 then 0 else 1
```

# Conditionals, case expressions, and guards

`if cond then e1 else e2` is an expression whose condition must have type `Bool`. Non-boolean values (such as integers) are not implicitly coerced and fail with `integer literal cannot have type Bool`.

`case scrutinee of` opens a layout block of alternatives evaluated in source order. Each alternative has the form `pattern -> body` or `pattern | guard -> body`, where `guard` has type `Bool`. Each alternative permits at most one guard; fallbacks are written as separate alternatives.

Pattern matching is checked for exhaustiveness. Because a guard can evaluate to `False`, a guarded alternative contributes nothing to coverage: every reachable constructor or value domain must still be covered by an unguarded alternative.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  classify :: Int -> Int
  classify n = case n of
    0 -> 100
    v | v > 10 -> 1
    v -> v

  main :: Int
  main = if classify 0 == 100 && classify 20 == 1 && classify 5 == 5 then 0 else 1
```

Writing a second guard on the same alternative is a syntax error:

<!-- cinch:reject contains="expected `;` or `}` in the alternative block, found Arrow" -->
```cinch
module Main where
  classify :: Int -> Int
  classify n = case n of
    v | v > 10 -> 1 | otherwise -> 2

  main :: Int
  main = classify 5
```

Covering a constructor only with a guarded row fails exhaustiveness checking:

<!-- cinch:reject contains="constructor Some is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail)" -->
```cinch
module Main where
  import Data.Option

  f :: Option Int -> Int
  f o = case o of
    Some v | v > 0 -> v
    None -> 0

  main :: Int
  main = f (Some 1)
```

For index-refined exhaustiveness and refuting impossible branches on indexed families, see [dependent types and proofs](08-dependent-types-and-proofs.md).

# Local bindings: let, letrec, where, and memo

Cinch provides four local binding forms:

- `let declarations in body` introduces a sequential, non-recursive local binding group. Each binding is in scope only for subsequent bindings and `body`; a self-reference inside plain `let` is rejected (`unknown name ...`).
- `letrec declarations in body` introduces a recursive local binding group whose names are mutually in scope across the group and `body`. It follows the same layout rules as `let`.
- `where` attaches a layout block of local declarations to an equation; those declarations may reference the equation's parameters.
- `~name = expr` introduces a memoized binding evaluated on demand (see [laziness and codata](11-laziness-and-codata.md)).

| Local declaration | Syntax |
| --- | --- |
| Type signature | `name :: Type` |
| Function equation | `name arg1 arg2 = expr` |
| Pattern binding | `pattern = expr` |
| Memo binding | `~name = expr` |
| Nested scope | `where` followed by an indented layout block |

<!-- cinch:run exit=0 -->
```cinch
module Main where
  memo :: Int
  memo =
    let ~saved = 40 + 2
    in saved

  countdown :: Int -> Int
  countdown n =
    letrec
      go :: Int -> Int
      go k = if k == 0 then 0 else go (k - 1)
    in go n

  label :: Int -> Int
  label n = if small then 1 else 2
    where
      small = n < 10

  main :: Int
  main = if memo == 42 && countdown 3 == 0 && label 5 == 1 then 0 else 1
```

Totality and guardedness requirements on recursive and corecursive bindings are covered in [dependent types and proofs](08-dependent-types-and-proofs.md) and [laziness and codata](11-laziness-and-codata.md).

# Lambdas

Anonymous functions are written `\p1 p2 -> body` with one or more atomic patterns. Zero-parameter lambdas are rejected (`a lambda needs at least one pattern`). Closures capture free variables under the same multiplicity and region rules as ordinary values (see [ownership and memory](10-ownership-and-memory.md)).

```cinch
inc = \n -> n + 1
pick = \a b -> a
```

# Records

A record type is declared with `record Name where` followed by `field :: Type` lines. Values are constructed with `Name { field = value, ... }` and projected with `value.field`. Record patterns match fields by name (`Name { field = pat, ... }`) and must list every field of the record.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Point where
    x :: Int
    y :: Int

  sum-fields :: Point -> Int
  sum-fields p = case p of
    Point { x = a, y = b } -> a + b

  main :: Int
  main =
    let p = Point { x = 3, y = 4 }
        q = Point { x = 10, y = 20 }
    in if sum-fields p == 7 && q.x + q.y == 30 then 0 else 1
```

Omitting a field in a record pattern is a compile error rather than an implicit wildcard:

<!-- cinch:reject contains="record pattern is missing field y" -->
```cinch
module Main where
  record Point where
    x :: Int
    y :: Int

  sum-x :: Point -> Int
  sum-x p = case p of
    Point { x = a } -> a

  main :: Int
  main = sum-x (Point { x = 2, y = 5 })
```

See [attributes and pragmas](23-attributes-and-pragmas.md) and [type layout](24-type-layout.md) for `erased` fields and representation guarantees.

# Unit, tuples, lists, ranges, and maps

| Literal form | Type and semantics |
| --- | --- |
| `()` | The sole inhabitant of `Unit`. |
| `(a, b)`, `(a, b, c)`, ... | Structural n-ary tuple product. |
| `[a, b, c]` | `FromList` literal at the expected container type (`List` when unconstrained). |
| `[start .. end]` | Finite [`Enum`](../library/Data.Enum.md) range; `[1 .. 3]` has three elements. |
| `[start, next .. end]` | Stepped finite `Enum` range; `[1, 3 .. 7]` has four elements. |
| `{k1 => v1, k2 => v2}` | `FromMap` literal at the expected map type (e.g., `Map String Int`). |

Tuples do not carry a prelude `Eq` instance, so `(1, 2) == (1, 2)` is rejected with `no instance for Eq (Int, Int)`; destructure and compare components instead.

# Patterns

| Pattern | Syntax |
| --- | --- |
| Variable | `name` |
| Wildcard | `_` |
| Literal | Integer or `Sym` literal (`0`, `-1`, `'answer`) |
| Unit | `()` |
| Constructor | `Some v`, `Cons x rest`, nested as `Some (v @ 0)` |
| Tuple | `(a, b)`, `(a, b, c)`, ... |
| List | `[a, b]` |
| Record | `Point { x = a, y = b }` |
| As-pattern | `name @ pattern` |

Patterns must be linear: binding the same variable twice in one pattern is rejected rather than performing an implicit equality test. Express equality with a guard or in the branch body.

<!-- cinch:reject contains="variable a is bound twice in the same pattern" -->
```cinch
module Main where
  eq2 :: (Int, Int) -> Int
  eq2 p = case p of
    (a, a) -> 1

  main :: Int
  main = eq2 (1, 1)
```

`_` is valid only in pattern position; using `_` as a term-level hole is rejected. Inline type annotations on patterns (such as `(v :: Int)` in a `case` alternative) are likewise not supported.

<!-- cinch:reject contains="typed holes are not supported in the bootstrap slice" -->
```cinch
module Main where
  main :: Int
  main = _
```

The following program combines as-patterns, literal, unit, constructor, tuple, and list patterns along with expression type ascription (`(7 :: Int)`):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.List
  import Data.Option

  as-zero :: Option Int -> Int
  as-zero o = case o of
    Some (v @ 0) -> v
    Some v -> v + 100
    None -> -1

  list-sum :: List Int -> Int
  list-sum xs = case xs of
    [a, b] -> a + b
    Cons a rest -> a
    Nil -> 0

  pair-sum :: (Int, Int) -> Int
  pair-sum p = case p of
    (a, b) -> a + b

  unit-code :: Unit -> Int
  unit-code u = case u of
    () -> 5

  main :: Int
  main =
    let ascribed = (7 :: Int)
    in if as-zero (Some 0) == 0
      && as-zero (Some 1) == 101
      && as-zero None == -1
      && list-sum [4, 5] == 9
      && pair-sum (2, 3) == 5
      && unit-code () == 5
      && ascribed == 7
    then 0
    else 1
```

# Complete example

This module combines records, constructors, guards, nested `let` bindings, lambdas, application chains, lists, and tuples:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.List
  import Data.Option

  record Point where
    x :: Int
    y :: Int

  add :: Int -> Int -> Int
  add a b = a + b

  doubled :: List Int -> List Int
  doubled xs = case xs of
    Nil -> Nil
    Cons v rest -> Cons (v * 2) (doubled rest)

  total :: List Int -> Int
  total xs = case xs of
    Nil -> 0
    Cons v rest -> add v (total rest)

  pair-sum :: (Int, Int) -> Int
  pair-sum p = case p of
    (a, b) -> a + b

  classify :: Int -> Int
  classify n = case n of
    0 -> 100
    v | v > 10 -> 1
    _ -> 2

  describe :: Option Int -> Int
  describe o = case o of
    None -> -1
    Some 0 -> 0
    Some v -> v

  main :: Int
  main =
    let point = Point { x = 3, y = 4 }
        pair = (point.x, point.y)
        inc = \n -> n + 1
        inner = let a = 1 in a + 1
    in if pair-sum pair == 7
         && inc inner == 3
         && total (doubled [1, 2]) == 6
         && classify 0 == 100
         && describe (Some 0) == 0
      then 0
      else 1
```

# Common diagnostics

| Diagnostic | Cause |
| --- | --- |
| `non-exhaustive patterns: constructor None is not covered` | A `case` omits a reachable constructor. |
| `constructor Some is not covered by an unguarded row (...)` | Every row matching that constructor carries a guard. |
| `variable a is bound twice in the same pattern` | Duplicate binder in a single pattern. |
| `record pattern is missing field y` | A record pattern omits a declared field. |
| `typed holes are not supported...` | `_` used in expression position. |
| `pattern type annotations are not supported...` | Type annotation `(v :: T)` written inside a pattern. |
| `integer literal cannot have type Bool` | Non-boolean condition in `if` or guard. |
| ``expected `;` or `}` in the alternative block, found Arrow`` | Multiple guards written on one `case` alternative. |
| `no instance for Eq (Int, Int)` | Equality applied to a tuple without an `Eq` instance. |

For strictness and constant collection folding, see [laziness and codata](11-laziness-and-codata.md) and [rewrites and optimization](12-rewrites-and-optimization.md). Specialized expression forms are documented in [refinements](06-refinements.md) (`decide`, `narrow`, `forget`), [effects and capabilities](09-effects-and-capabilities.md) (`do`, `handle`, `with`), and [staging and macros](14-staging-and-macros.md) (quotations, splices, `name!(...)`).
