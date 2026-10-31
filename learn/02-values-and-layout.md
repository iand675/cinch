# 2. Values, expressions, and layout

A value is the result of evaluating an expression. In a strict language, ordinary arguments and bindings are evaluated eagerly; delaying a computation is an explicit operation. You do not need to reason about deferred evaluation to understand an arithmetic expression.

# Values have types

`42` can be an `Int`, `Nat`, or a machine integer depending on the expected type. `Int` represents mathematical signed integers; `Nat` represents mathematical nonnegative integers. They are not aliases for 64-bit arithmetic. `True` and `False` are `Bool` values. `()` is the only `Unit` value. Double quotes delimit a `String`; single quotes delimit a character, as in `'x'`.

Write an ascription when a literal is ambiguous: `(7 :: Nat)`. This says what type the expression must have; it does not perform a numeric conversion. Conversions are explicit functions, discussed in [chapter 7](07-numbers-text-and-bytes.md).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  answer :: Int
  answer = 6 * 7

  main :: Int
  main =
    let base = 40
        result = base + 2
    in if result == answer then 0 else 1
```

`let` introduces local bindings, and `in` starts the expression that uses them. The `result` binding can refer to `base`. These bindings name immutable values; `=` does not assign a new value to an existing variable. A program can give the same spelling a new binding in an inner scope, but that is shadowing, not mutation.

# Indentation is syntax

The first declaration after `where` establishes the indentation of the block. Equal indentation starts another declaration; less indentation ends the block. `let`, `of`, `do`, and `letrec` also introduce layout blocks. Tabs in indentation are rejected. Use spaces consistently.

The right side of `main` above starts on a new, more indented line. The two bindings align with each other. A `case` nested inside a binding must be indented past that binding's column. If a diagnostic seems to join two declarations, check indentation before changing the expression.

Explicit braces and semicolons are also available. They are useful for a short group such as `let a = 3; b = a + 4 in b`. Parentheses, brackets, and braces shield their contents from surrounding layout.

# Application and operators

Application binds more tightly than infix operators: `double 3 + 1` means `(double 3) + 1`. Write `double (3 + 1)` to pass the sum instead. Operator precedence is declared by the library. Multiplication binds more tightly than addition; comparisons bind less tightly than arithmetic. Parentheses make intent explicit when combining unfamiliar operators.

Names may contain dashes. `read-config` is one identifier, and `x-y` can also be one identifier. Put spaces around subtraction: `x - y`. `f -42` passes a negative literal to `f`. The [lexical reference](../reference/02-lexical-structure.md) gives the exact minus rule.

Comments start with `--` and run to the end of the line. Block comments use `{- ... -}` and may nest. Do not put an unexplained `--` into an operator expression; it starts a comment.

# Exercise and answer

Evaluate `let x = 5; y = x * 2 in y + x` by hand. Then use it in `main` to check your prediction.

<details><summary>Answer</summary>

The result is `15`. The binding `y` is `10`; `y + x` adds `5`. Write `main = if (let x = 5; y = x * 2 in y + x) == 15 then 0 else 1`.

</details>
