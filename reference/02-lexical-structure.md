# Lexical structure and layout

A source file goes through five stages before it is checked: `compiler/src/lexer.rs` produces tokens, `compiler/src/layout.rs` turns significant indentation into explicit `{`, `;`, and `}` tokens that carry `virtual_: true`, `compiler/src/parser.rs` builds declarations and expressions, a fixity pass resolves operator chains against the imported tables, and the checker runs. This page states the rules of the first three stages and shows the compiler accepting or refusing one input per rule.

# Identifiers

A value name starts with an ASCII lowercase letter or `_` and continues with ASCII letters, digits, `_`, and `'`. A constructor, type, or module name starts with an ASCII uppercase letter and continues the same way. An operator name is a run of symbol characters or a value name in parentheses, `(<+>)`.

A dash joins two parts into one name when it is tight on both sides and the character after it is alphabetic, `_`, or `'`. Digits do not continue a name across a dash. The rule is decided inside the lexer, so it applies before any meaning is attached to the spelling.

| Spelling | Interpretation |
| --- | --- |
| `x-y` | One name, when `x` and `y` satisfy the dash glue rule. |
| `x - y` | Subtraction of two names. |
| `x-1` | Subtraction: a digit does not continue a name across a dash. |
| `x-y'` | One name: `'` continues the name after the dash. |
| `_x` | A name. `_` alone is the pattern wildcard, not a name. |
| `x'` | A name: an apostrophe continues an identifier. |
| `café` | Not an identifier: identifiers are ASCII. |

This program defines a value named `x-y` and also uses the three-part subtraction `x - y`. It exits 0 only if `x-y` is one name holding 9 and `x - y` is 100 minus 1:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  x-y :: Int
  x-y = 9

  x :: Int
  x = 100

  y :: Int
  y = 1

  main :: Int
  main = if x-y + (x - y) == 108 then 0 else 1
```

An apostrophe continues a name but never starts one. This program passes with both `_total` and `total'`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  _total :: Int
  _total = 20

  total' :: Int
  total' = 2

  main :: Int
  main = if _total + total' == 22 then 0 else 1
```

The capitalized spelling belongs to another namespace, so this declaration is refused:

<!-- cinch:reject contains="expected a declaration, found constructor" -->
```cinch
module Main where
  Bad-name :: Int
  Bad-name = 1

  main :: Int
  main = 0
```

Keywords are reserved in every position, including declaration position:

<!-- cinch:reject contains="expected a declaration, found Let" -->
```cinch
module Main where
  let :: Int
  let = 5

  main :: Int
  main = let
```

The keyword set the lexer recognizes is `module`, `where`, `data`, `record`, `codata`, `effect`, `signature`, `instance`, `implicit`, `import`, `as`, `type`, `opaque`, `forall`, `Sigma`, `using`, `in`, `let`, `letrec`, `if`, `then`, `else`, `case`, `of`, `do`, `handle`, `with`, `rewrite`, `prim`, `law`, `target`, `justify`, `match`, `replace`, `require`, `upgrade`, `contract`, `to`, `generate`, `quoteDecls`, `foreign`, `export`, `ccall`, `component`, `proof`, `spec`, `stage`, `corec`, `macro`, `infix`, `infixl`, `infixr`, `erased`, `own`, `return`, `resume`.

A non-ASCII letter is not an identifier character:

<!-- cinch:reject contains="unexpected character" -->
```cinch
module Main where
  café :: Int
  café = 1

  main :: Int
  main = 0
```

# Comments

`--` starts a line comment. `{-` and `-}` delimit a block comment, and block comments nest. Comment recognition wins over operator runs, so `5 -- 3` is `5` followed by a comment rather than a subtraction. This program exits 0 for that reason:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  -- a line comment
  {- a block {- nested -} comment -}
  v :: Int
  v = 5 -- 3

  main :: Int
  main = if v == 5 then 0 else 1
```

An unclosed block comment runs to end of file:

<!-- cinch:reject contains="unterminated block comment" -->
```cinch
module Main where
  {- open
  main :: Int
  main = 0
```

# Numbers

An integer literal is a run of decimal digits or a `0x` hexadecimal run. Single underscores inside the digits are dropped before the value is formed, and the digits are canonicalized, so `007` is the integer 7 and `0x1F` is 31. The lexer keeps the literal as a decimal digit string with no bound; the expected type decides how it is elaborated. This program exits 0:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  hex :: Int
  hex = 0x1F

  under :: Int
  under = 1_000

  padded :: Int
  padded = 007

  main :: Int
  main = if hex + under + padded == 1038 then 0 else 1
```

A fractional form is lexed as a float token and then refused. Use the conversion and parser vocabulary in [numbers and primitives](05-numbers-and-primitives.md) instead:

<!-- cinch:reject contains="float literals are not supported" -->
```cinch
module Main where
  f :: Int
  f = 1.5

  main :: Int
  main = 0
```

The lexer requires a digit immediately after the `.` of a fractional form, so `1.5` is a float token while `Module.name` and `record.field` stay qualification and projection.

# Strings and characters

A string is double quoted, may not contain a raw newline, and decodes the escapes `\n`, `\t`, `\\`, `\"`, and `\u{...}`, whose hex digits name a Unicode scalar value. A character literal is one character or one escape between single quotes. The tick form is decided lexically: tick, one character, tick is a `Char`; a longer name after the tick is a symbol, `'answer`, and two ticks name a type, `''Item`.

This program exits 0 only if `\u{41}` decodes to `A` and each escape is one character:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Data.String

  upper :: String
  upper = "\u{41}"

  plain :: String
  plain = "A"

  tabbed :: String
  tabbed = "a\tb"

  quoted :: String
  quoted = "say \"hi\""

  slashed :: String
  slashed = "a\\b"

  main :: Int
  main =
    if upper == plain
      && string-length tabbed == 3
      && string-length quoted == 8
      && string-length slashed == 3
    then 0
    else 1
```

<!-- cinch:reject contains="unknown escape sequence" -->
```cinch
module Main where
  s :: String
  s = "a\qb"

  main :: Int
  main = 0
```

<!-- cinch:reject contains="escape is not a Unicode scalar value" -->
```cinch
module Main where
  s :: String
  s = "\u{110000}"

  main :: Int
  main = 0
```

The three tick forms in one program. The last case is the one worth remembering: `'ab'` is not a malformed character literal, it is the symbol whose name is `ab'`, because the trailing apostrophe continues the name once the closing tick is not immediately after one character. The program exits 0 through that branch:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Sym
  import Data.String

  letter :: Char
  letter = 'a'

  newline :: Char
  newline = '\n'

  answer :: Sym
  answer = 'answer

  main :: Int
  main =
    if letter == 'a'
      && newline == '\n'
      && sym-name answer == "answer"
      && sym-name 'ab' == "ab'"
    then 0
    else 1
```

# Operators and fixity

The symbol characters are `!`, `#`, `%`, `&`, `*`, `+`, `/`, `<`, `=`, `>`, `?`, `@`, `^`, `|`, `~`, `:`, and `-`. `$` continues an operator run such as `<$>` but never starts one, and `$(` opens a splice. The runs `::`, `->`, `<-`, `=>`, `!`, `@`, `=`, `|`, and `~` are reserved tokens rather than operators. Every other symbol run is an operator, including a longer run that contains those characters, so `<+>` and `<=` are operators while `!` alone is not. `|` stays usable in operator position: a fixity declaration and a definition accept it, while `!` is refused (both are shown below), and the guard and row positions consume `|` before a term is parsed.

A module declares an operator's associativity and precedence with `infix`, `infixl`, or `infixr` and a precedence from 0 to 9. Only the declaring module declares it; importers inherit it with the operator's identity. The shipped table, as declared in the base family:

| Declaration | Operators | Declared in |
| --- | --- | --- |
| `infixl 9` | `*`, `/`, `%` | `Algebra/Arithmetic.cinch` |
| `infixl 8` | `+`, `-` | `Algebra/Arithmetic.cinch` |
| `infixl 7` | `<<`, `>>` | `Algebra/Bits.cinch` |
| `infixr 6` | `<>` | `Algebra/Semigroup.cinch` |
| `infixl 6` | `&` | `Algebra/Bits.cinch` |
| `infixl 5` | `^` | `Algebra/Bits.cinch` |
| `infixl 4` | `<$>`, `<*>` | `Category/Functor.cinch`, `Category/Applicative.cinch` |
| `infix 3` | `==`, `/=`, `<`, `<=`, `>`, `>=` | `Relation/Eq.cinch`, `Relation/Ord.cinch` |
| `infixr 2` | `&&` | `Data/Bool.cinch` |
| `infixl 1` | `>>=` | `Category/Monad.cinch` |
| `infixr 1` | `>=>`, `<=<` | `Category/Monad.cinch` |

Three declarations hold the pipe character, which a Markdown table cell cannot show, so they are named here. `||` is `infixr 1` in `Data/Bool.cinch`. `<|>` is `infixl 3` in `Category/Alternative.cinch`. `|` is `infixl 4` in `Algebra/Bits.cinch` as a `Bits` method, so `1 | 2` on `Int` is refused with `no instance for Bits Int`. A file may declare `|` for itself: this program declares it at precedence 5 and exits 0 through its own definition.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  infixl 5 |

  (|) :: Int -> Int -> Int
  (|) a b = a * 100 + b

  main :: Int
  main = if 1 | 2 == 102 then 0 else 1
```

Import a module to use its operators; the [library reference](../library/00-index.md) lists each one with its signature and precedence.

A file may declare its own operator. This program declares `<+>` at precedence 5 and exits 0 through left association:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  infixl 5 <+>

  (<+>) :: Int -> Int -> Int
  (<+>) a b = a * 10 + b

  main :: Int
  main = if 1 <+> 2 <+> 3 == 123 then 0 else 1
```

Precedence outside 0 to 9 is refused:

<!-- cinch:reject contains="precedence 10 is out of range" -->
```cinch
module Main where
  infixl 10 +

  main :: Int
  main = 1 + 2
```

The reserved spelling cannot be declared as an operator:

<!-- cinch:reject contains="expected an operator in the fixity declaration, found Bang" -->
```cinch
module Main where
  infixl 5 !

  (!) :: Int -> Int -> Int
  (!) a b = a + b

  main :: Int
  main = 0
```

`infix` is non-associative, so an unparenthesized chain of it is refused rather than given a meaning:

<!-- cinch:reject contains="cannot be chained at precedence 3" -->
```cinch
module Main where
  main :: Int
  main = if 1 == 2 == 3 then 0 else 1
```

Application binds tighter than any infix operator, so `f x + g y` is `(f x) + (g y)`, and `$` is the low-precedence application that avoids parentheses: `add 1 $ add 2 3` is `add 1 (add 2 3)`. The [expressions page](03-expressions-and-patterns.md) shows the forms.

# Layout blocks

`where`, `of`, `do`, and `let` open an implicit block unless an explicit `{` follows. `letrec` opens one too, because the lexer emits it as a marker token and a `let` token sharing one span. The first token of the block sets its column. At a logical line start, equal indentation begins a new item, less indentation closes the innermost block, and greater indentation continues the item already open. A block must indent farther than the block that encloses it. Explicit `(`, `[`, and `{` shield their contents, and a closing delimiter first closes the implicit blocks opened inside it. `in` closes the nearest implicit `let`.

A tab in indentation is rejected, so indentation is spaces only:

<!-- cinch:reject contains="tabs are rejected in indentation" -->
```cinch
module Main where
	main :: Int
	main = 0
```

The alternatives of a `case` are items of the block that `of` opens, so they must indent past the declaration that holds the `case`. Two columns of indent is not enough here:

<!-- cinch:reject contains="layout block must indent farther than its enclosing block" -->
```cinch
module Main where
  classify :: Int -> Int
  classify n = case n of
  0 -> 10
  x -> x

  main :: Int
  main = classify 0
```

A declaration ends where its block's column resumes. An expression continued on the next line must indent past the column of the name it belongs to. Here `0` sits at the same column as `main`, so it starts a new item and the parser expects an expression before it:

<!-- cinch:reject contains="expected an expression, found Semicolon" -->
```cinch
module Main where
  main :: Int
  main =
  0
```

Brackets shield layout, and a continuation line indented past the declaration's column continues it. Both are used here, with the list items placed at column 1 inside `[` and `]`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.List

  total :: List Int -> Int
  total xs = case xs of
    Nil -> 0
    Cons x rest -> x + total rest

  xs :: List Int
  xs = [
  1,
  2,
  3
  ]

  main :: Int
  main =
    if total xs == 6
      then 0
      else 1
```

The `in` that closes a `let` is described with the other expression forms on the [expressions page](03-expressions-and-patterns.md), and the fixity and layout rationale is in the [surface grammar](../specifications/surface-syntax-and-grammar.md) and its [local amendments](../specifications/surface-grammar-local-amendments.md). Some historical status notes on those pages predate implemented features; this page follows the compiler.

Implementation: `compiler/src/lexer.rs` (tokens, escapes, dash glue, tick forms), `compiler/src/layout.rs` (block insertion), `compiler/src/parser.rs` (fixity resolution and declaration boundaries). Suites: `surface.rs`, `driver.rs`, `modules.rs`.
