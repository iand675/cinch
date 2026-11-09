# Expressions, bindings, and patterns

This page gives the spelling of each expression, binding, and pattern form, with one program per group that the compiler accepts and one input per rejection it refuses. Types, effects, ownership, coverage theory, and evaluation order belong to the pages linked from each section; the [lexical page](02-lexical-structure.md) covers names, literals, operators, and where a declaration ends.

# Application and operators

| Form | Spelling |
| --- | --- |
| Application | `f x y`, left associative, so `f x y` is `(f x) y` |
| An operator as a value | `(+) 2 3` |
| Infix use | `1 + 2`, resolved by the operators' declared fixity |
| Low-precedence application | `add 1 $ add 2 3` is `add 1 (add 2 3)` |
| Prefix negation | `-(add 1 2)`; negation binds to one atomic term, so negate an application in parentheses |
| Signed literal argument | `f -42` applies `f` to the literal `-42` |
| Projection | `value.field` |
| Named capability supply | `f @{io = capability} argument`; see [effects and capabilities](09-effects-and-capabilities.md) |

Spacing decides whether a minus is glue, subtraction, or a literal sign; the [lexical page](02-lexical-structure.md) has the table and the program that separates them. Fixities come from the declaring module, and an imported operator keeps the precedence its module declared.

This program exits 0 only if `$`, the parenthesized operator, negated application, and the signed literal each mean what the table says:

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

# if, case, and guards

`if condition then then-branch else else-branch` is an expression. The condition and the guard of a `case` alternative must be `Bool`; an integer in that position is refused with `integer literal cannot have type Bool`.

`case scrutinee of` opens a layout block of alternatives. Each alternative is `pattern -> body`, or `pattern | guard -> body`. There is at most one guard per alternative, and the fallback is a separate alternative, not a second guard on the same one. Alternatives are tried in source order.

A guarded row covers nothing, because its guard can fail. This program has an unguarded last row and exits 0:

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

A second guard written after the first alternative's body is a parse error, because the alternative block expects its next separator:

<!-- cinch:reject contains="expected `;` or `}` in the alternative block, found Arrow" -->
```cinch
module Main where
  classify :: Int -> Int
  classify n = case n of
    v | v > 10 -> 1 | otherwise -> 2

  main :: Int
  main = classify 5
```

The coverage check states the rule in its own words when the only row for a constructor is guarded:

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

Coverage of constructors, literal fallback, tuples, and index-refined cases is shared with the dependent-type checker; [dependent types and proofs](08-dependent-types-and-proofs.md) covers the index case where an arm is refused because the index makes it impossible.

# let, letrec, where, and memo bindings

`let declarations in body` is a sequential local group, and `in` closes it. Its declarations sit in a layout block, so several bindings line up in one column. `letrec declarations in body` is the recursive group: recursion is visible across the group, so a self-referencing local function needs it. The lexer emits `letrec` as a marker token plus a `let` token, which is why the same layout rules open both blocks.

An equation may carry a `where` block of declarations that belongs to that equation and can use its arguments. A memo binding is written `~name = expression`.

| Local form | Spelling |
| --- | --- |
| Signature | `name :: Type` |
| Equation | `name arguments = expression` |
| Pattern binding | `pattern = expression` |
| Memo binding | `~name = expression` |
| Nested block | `where` followed by a layout block of the forms above |

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

A plain `let` group is not recursive: referring to a binding from its own definition inside `let` is refused with `unknown name go (in Main.cinch)`, where `letrec` accepts the same definition. [Dependent types and proofs](08-dependent-types-and-proofs.md) and [laziness and codata](11-laziness-and-codata.md) describe what ordering and guardedness the checker enforces on local groups.

# Lambdas

`\patterns -> body` takes one or more atomic patterns and a body. A captured value keeps its usage and region restrictions, so the type checker treats a closure like any other value; see [ownership and memory](10-ownership-and-memory.md).

```cinch
  inc = \n -> n + 1
  pick = \a b -> a
```

A lambda with no pattern is refused with `a lambda needs at least one pattern`.

# Records

A record is declared with `record Name where` and one `field :: Type` line per field. Construction is `Name { field = value, ... }`, projection is `value.field`, and a record pattern names its fields with `=` and must name every field.

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

A partial record pattern is refused rather than treated as a wildcard for the missing field:

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

Record updates, `erased` fields, and the layout guarantee of a record are in [attributes and pragmas](23-attributes-and-pragmas.md) and [type layout](24-type-layout.md).

# Lists, pairs, and unit

| Form | Type |
| --- | --- |
| `()` | `Unit` |
| `(a, b)` | A pair; `(a, b, c)` continues the same shape |
| `[a, b, c]` | `FromList` construction at the expected container type, `List` when unconstrained |
| `[start .. end]` | Finite [`Enum`](../library/Data.Enum.md) range; `[1 .. 3]` has three elements |
| `[start, next .. end]` | The same range with an explicit step; `[1, 3 .. 7]` has four |
| `{key => value}` | `FromMap` construction at the expected map type; `{"a" => 1, "b" => 2}` is a `Map String Int` |

The list and pair literals are ordinary values, so `[4, 5]` is a two-element list and `(2, 3)` is one pair. A pair has no `Eq` instance in the current base family, so `(1, 2) == (1, 2)` is refused with `no instance for Eq (Int, Int)`; compare the components instead.

# Patterns

| Pattern | Spelling |
| --- | --- |
| Variable | `name` |
| Wildcard | `_` |
| Literal | An integer or `Sym` literal, such as `0` or `'answer` |
| Unit | `()` |
| Constructor | `Some v`, `Cons x rest`, nested as in `Some (v @ 0)` |
| Tuple | `(a, b)` |
| List | `[a, b]` |
| Record fields | `Point { x = a, y = b }` |
| As-pattern | `name @ pattern` |

Repeating a binder in one pattern is refused, because a pattern binds names and does not test equality. The equality belongs in a guard or a branch body. Annotating a pattern with its type parses in the grammar but is not implemented here: `(v :: Int)` in a `case` alternative is refused with `pattern type annotations are not supported in the bootstrap slice`.

<!-- cinch:reject contains="variable a is bound twice in the same pattern" -->
```cinch
module Main where
  eq2 :: (Int, Int) -> Int
  eq2 p = case p of
    (a, a) -> 1

  main :: Int
  main = eq2 (1, 1)
```

A term-position hole is not a valid expression; `_` is a pattern form only:

<!-- cinch:reject contains="typed holes are not supported in the bootstrap slice" -->
```cinch
module Main where
  main :: Int
  main = _
```

This program exercises the as-pattern, the integer literal, the unit, the constructor, the tuple, and the list pattern, and exits 0:

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

# A complete program

Every form on this page in one file: a record, constructor patterns with a nested as-pattern, guards, `let` and a nested `let`, a lambda, an application chain, a list literal, a pair, and an ascription-free final comparison. It exits 0 only if all of them behave:

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

# Diagnostics

Each of these was produced by the program above it on this page, or by a program named in its row.

| Text | Cause |
| --- | --- |
| `non-exhaustive patterns: constructor None is not covered` | A `case` omits a reachable constructor. |
| `constructor Some is not covered by an unguarded row (a guarded row covers nothing, since its guard can fail)` followed by advice to add an unguarded final row | Every row for that constructor is guarded. |
| `variable a is bound twice in the same pattern` | Two binders in one pattern share a name. |
| `record pattern is missing field y` | A record pattern omits a field. |
| `typed holes are not supported in the bootstrap slice` | `_` used as an expression. |
| `integer literal cannot have type Bool` | An integer where a condition or guard belongs. |
| `pattern type annotations are not supported in the bootstrap slice` | `(v :: Int)` written as a pattern. |
| ``expected `;` or `}` in the alternative block, found Arrow`` | A second guard on one alternative. |
| `no instance for Eq (Int, Int)` | Equality on a pair, which has no instance. |

Evaluation order, strictness, and the folding of constant collections are covered in [laziness and codata](11-laziness-and-codata.md) and [rewrites and optimization](12-rewrites-and-optimization.md). Effectful expression forms have their own pages: capability supplies and `do` in [effects and capabilities](09-effects-and-capabilities.md), handlers in the same page, quotations, splices, and `name!(tokens)` macro calls in [staging and macros](14-staging-and-macros.md), and `forget` and `narrow` in [refinements](06-refinements.md).

Implementation: `compiler/src/ast.rs` (the term and pattern shapes), `compiler/src/parser.rs` (the grammar as accepted), `compiler/src/check_terms.rs` and `compiler/src/check_decls.rs` (types, guards, coverage). Suites: `surface.rs`, `guard_coverage.rs`, `pattern_field_names.rs`, `letrec.rs`, `tail_calls.rs`.
