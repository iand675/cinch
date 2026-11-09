# 2. Values, expressions, and layout

This chapter turns the one-line program from chapter 1 into a program with several named values, arithmetic, comparisons, conditionals, and local bindings. You will also learn the indentation rule, because it decides where declarations and `let` bindings end. The commands are the same throughout:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

Prerequisites: [chapter 1](01-first-program.md). You need a working `cinch` on `PATH`.

# Values have types

Every expression has one type, and the expected type decides how a literal is read. `42` is an `Int` when an `Int` is expected and a `Nat` when a `Nat` is expected. These are mathematical integers, not fixed-width machine words: a program can hold a value above the largest 64-bit integer and compare it correctly.

| Type | Values | Written as |
| --- | --- | --- |
| `Int` | signed integers | `42`, `0x1F`, `1_000`, `-7` |
| `Nat` | nonnegative integers | `42`, `(7 :: Nat)` |
| `Bool` | truth values | `True`, `False` |
| `Unit` | exactly one value | `()` |
| `String` | UTF-8 text | `"hello"` |
| `Char` | one character | `'x'` |
| `(A, B)` | a pair of an `A` and a `B` | `(True, 7)` |

A type ascription states the type of one expression without converting anything: `(7 :: Nat)` is the `Nat` seven. Conversions are functions, covered in [chapter 7](07-numbers-text-and-bytes.md).

# Number literals

Decimal digits are the default. `0x` introduces hexadecimal, `_` separates digits for readability, and leading zeros change nothing. Each literal below denotes the value in the second column:

| Literal | Value |
| --- | --- |
| `0x1F` | 31 |
| `1_000` | 1000 |
| `007` | 7 |

A leading `-` makes a negative literal: `-7`. Arithmetic on `Int` is ordinary integer arithmetic, and `/` truncates towards zero while `%` gives the remainder, so `100 / 7` is `14`, `100 % 7` is `2`, and `(-7) / 2` is `-3`.

Subtraction on `Nat` is different. The compiler requires evidence that the result is not negative, and rejects a call whose arguments make the result negative:

```
cinch: 3:14: the obligation `1 <= 0` of this call is false: its arguments are known here and do not satisfy it (in Main.cinch)
```

The evidence is supplied by an `if` test or a guard on the value, as [chapter 13](13-refinements-and-safe-operations.md) explains.

# Text literals

Double quotes delimit a `String` and single quotes delimit a `Char`. A string literal accepts these escapes: `\n` for a newline, `\t` for a tab, `\\` for a backslash, `\"` for a quote, and `\u{41}` for a character by code point. `"a\nb"` is three characters, and `"\u{41}"` is the same string as `"A"`.

These prelude functions turn values into text and measure text:

| Function | Type |
| --- | --- |
| `int-to-string` | `Int -> String` |
| `nat-to-string` | `Nat -> String` |
| `char-string` | `Char -> String` |
| `string-length` | `String -> Nat` |
| `string-concat` | `String -> String -> String` |

`int-to-string 42` is `"42"`, `int-to-string (-42)` is `"-42"`, and `string-length (int-to-string 1234)` is `4`. Formatting is explicit: no operator converts a number to text, and `==` compares a `String` with a `String` and nothing else.

# A complete program

<!-- cinch:run exit=0 -->
```cinch
module Main where
  answer :: Int
  answer = 6 * 7

  digits :: String
  digits = int-to-string answer

  main :: Int
  main =
    let base = 40
        result = base + 2
        count = string-length digits
    in if result == answer && count == 2
       then 0
       else 1
```

`answer :: Int` is `6 * 7`, so `42`. `digits :: String` is `int-to-string answer`, so `"42"`. `main :: Int` opens a `let` with three bindings: `base` is `40`, `result` is `base + 2`, and `count` is `string-length digits`. The expression after `in` is an `if` whose condition joins two tests with `&&`: `result == answer` holds, and `count == 2` holds because `"42"` has two characters. The `then` branch returns `0`, so the program exits 0.

# Operators and fixity

Operators are ordinary names with a declared precedence and associativity. Higher precedence binds tighter, and application binds tighter than every operator.

| Precedence | Operators | Associativity |
| --- | --- | --- |
| 9 | `*`, `/`, `%` | left |
| 8 | `+`, `-` | left |
| 3 | `==`, `/=`, `<`, `<=`, `>`, `>=` | none |
| 2 | `&&` | right |
| 1 | `\|\|` | right |

Left associativity groups `10 - 3 - 2` as `(10 - 3) - 2`, which is `5`. Precedence groups `2 + 3 * 4` as `2 + (3 * 4)`, which is `14`; write `(2 + 3) * 4` for `20`. Application binds tighter than any operator, so `f 3 + 1` calls `f` with `3` and then adds one, while `f (3 + 1)` passes the sum.

The comparison operators are non-associative, which means a chain is rejected instead of grouped:

<!-- cinch:reject contains="the non-associative operator `==` cannot be chained at precedence 3; add parentheses" -->
```cinch
module Main where
  main :: Int
  main = if 1 == 1 == True then 0 else 1
```

Write `1 == 1 && True` when you mean two tests. The surface operators short-circuit: `False && x` and `True || x` never evaluate `x`. With a `spin` that never returns, `main = if True || (spin 1 == 0) then 7 else 8` exits 7 rather than diverging. The parenthesized `(&&)` and `(||)` are the strict methods of `Data.Bool`, and those do evaluate both arguments, so `(||) True (spin 1 == 0)` hangs. Reach for the operator when a right-hand operand is expensive or can fail; the prefix form is what a class method or a higher-order argument needs.

A dash inside a name is part of the name. `read-config` is one identifier, and so is `x-y`: writing `let x-y = 5` binds a name with a dash, while `x - y` subtracts. Put spaces around subtraction. The exact minus and comment rules are in [lexical structure](../reference/02-lexical-structure.md).

# Conditionals

`if` takes a `Bool` condition and two expressions of the same type. Both branches must produce that type, and the condition must be a `Bool`; in a three-line file whose `main` is on line 3, `if 1 then 0 else 1` is rejected with `cinch: 3:13: integer literal cannot have type Bool (in Main.cinch)`. `&&` and `||` combine tests, and nesting is written by putting another `if` in a branch.

# let and in

`let` introduces local bindings and `in` starts the expression that uses them. Each binding is an immutable value; `=` binds a name, it does not assign to an existing one.

Fragment, to be read in the position of `main`'s body:

```cinch
    let base = 40
        result = base + 2
    in result
```

A later binding refers to an earlier one, so `result` may use `base`, and the reverse is rejected: `base` is not in scope above its own binding, and the compiler reports `cinch: 4:18: unknown name base (in Main.cinch)` when `result` is bound first. Semicolons join bindings on one line: `let a = 3; b = a + 4 in b` is `7`. An inner binding may reuse an outer name, which shadows it:

Fragment:

```cinch
    let x = 1 in let x = 2 in x
```

That expression is `2`. Shadowing creates a new name in a narrower scope; the outer `x` is untouched and becomes visible again where the inner block ends.

# Layout

Indentation is syntax. The first declaration after `where` sets the column of the block. Another declaration at the same column continues the block, and anything further left ends it. `let`, `letrec`, `of`, and `do` open blocks the same way.

The `main` in the complete program above is accepted because its three `let` bindings start in the same column. A binding at a different column ends the group, and the `in` then belongs to no `let`. A continued expression may also sit on a more indented line:

Fragment:

```cinch
    if result == answer
       && count == 2
    then 0
    else 1
```

The rejection case is a declaration that moves left out of the block. Here `main` is indented one space while `answer` is indented two, so the module block ends after `answer`, and a nested `module` header would be required next.

<!-- cinch:reject contains="expected `module`, found identifier `main`" -->
```cinch
module Main where
  answer :: Int
  answer = 42

 main :: Int
 main = if answer == 42 then 0 else 1
```

Tabs are not allowed in indentation at all: a tab-indented declaration is rejected with `cinch: Main.cinch: 2:1: tabs are rejected in indentation`. Use spaces.

Comments start with `--` and run to the end of the line. A block comment starts with `{-` and ends with `-}`, and block comments nest, so `{- outer {- inner -} still a comment -}` is one comment.

# What the compiler enforces

- Every expression has exactly one type, and a literal takes the type its position requires.
- `if` requires a `Bool` condition and two branches of the same type.
- `==` compares two values of the same type. There is no implicit conversion between `Int`, `Nat`, `String`, and `Bool`.
- Operators use their declared precedence, and comparison operators cannot be chained without parentheses.
- A `let` binding cannot refer to a binding that appears later in the same group.
- Indentation is significant, and tabs are rejected in indentation.

# Diagnostics you will meet

| Message | What produced it |
| --- | --- |
| `cinch: 3:13: integer literal cannot have type Bool (in Main.cinch)` | `if 1 then 0 else 1` |
| ``cinch: 3:14: the obligation `1 <= 0` of this call is false: its arguments are known here and do not satisfy it (in Main.cinch)`` | `(0 :: Nat) - 1` |
| ``cinch: Main.cinch: 3:20: the non-associative operator `==` cannot be chained at precedence 3; add parentheses`` | `1 == 1 == True` |
| `cinch: Main.cinch: 2:1: tabs are rejected in indentation` | a declaration indented with a tab |
| ``cinch: Main.cinch: 5:2: expected `module`, found identifier `main` `` | a declaration indented left of its block |

# Exercises

1. Evaluate `let x = 5; y = x * 2 in y + x` by hand, then make `main` succeed only when the value is 15.

<details><summary>Answer</summary>

`y` is `10`, so `y + x` is `15`. Use `main = if (let x = 5; y = x * 2 in y + x) == 15 then 0 else 1`. The parentheses are required because `==` binds less tightly than everything inside the `let`.

</details>

2. Write a program that exits 0 only when `int-to-string 42` has two characters and `"a\nb"` has three.

<details><summary>Answer</summary>

Compare the lengths: `main = if string-length (int-to-string 42) == 2 && string-length "a\nb" == 3 then 0 else 1`. `string-length` counts characters, so the newline counts once.

</details>

Next: [functions, types, and local definitions](03-functions.md).
