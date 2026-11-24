# Lexical structure and layout

A Cinch source file passes through tokenization, offside layout insertion (turning significant indentation into explicit `{`, `;`, and `}` delimiters), parsing, and operator fixity resolution before type checking. This chapter specifies the lexical grammar, operator fixity rules, and layout rules.

# Identifiers and keywords

Value identifiers begin with an ASCII lowercase letter or `_`. Constructor, type, and module identifiers begin with an ASCII uppercase letter. After the first character, identifiers in both namespaces may contain ASCII letters, digits, `_`, `'`, `!`, and `?`. Non-ASCII characters are not valid in identifiers.

Sigils (`!` and `?`) may appear anywhere after the first character and in runs (`print!`, `empty?`, `read!?`), but never begin an identifier: in leading position they start an operator (`!=`, `?`, `(!)`). Because sigils bind rightwards into the preceding name, an operator beginning with `!` or `?` must be preceded by whitespace. One exception applies: a single `!` immediately before `(`, `[`, or `{` invokes a macro (`name!(...)`), so a value named `print!` is invoked as a macro via `print!!(...)`.

A hyphen `-` joins two parts into a single identifier when it has no surrounding whitespace and the immediately following character is an ASCII letter, `_`, `'`, `!`, or `?`. Digits do not continue an identifier across a hyphen, so `x-1` lexes as subtraction. Because this rule runs in the lexer, spacing unambiguously separates hyphenated names from subtraction and signed literals:

| Spelling | Tokenization |
| --- | --- |
| `x-y` | Single identifier `x-y`. |
| `x - y` | Subtraction of `x` and `y`. |
| `x-1` | Subtraction of `x` and `1` (digits do not glue across `-`). |
| `x-y'` | Single identifier `x-y'` (`'` continues after `-`). |
| `_x` | Identifier `_x` (`_` alone is the pattern wildcard). |
| `x'` | Identifier `x'` (`'` continues an identifier but never starts one). |
| `empty?` | Single identifier `empty?`. |
| `print!` | Single identifier `print!`. |
| `read!?` | Single identifier `read!?`. |
| `x!=y` | Identifier `x!`, then `=`, then `y`. |
| `x != y` | Operator application with `!=`. |
| `café` | Lexical error (identifiers are ASCII-only). |

Hyphenated names and spaced subtraction coexist in the same scope:

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

Leading underscores and trailing apostrophes are ordinary identifier characters:

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

Uppercase names belong to the type/constructor/module namespace and cannot declare values:

<!-- cinch:reject contains="expected a declaration, found constructor" -->
```cinch
module Main where
  Bad-name :: Int
  Bad-name = 1

  main :: Int
  main = 0
```

Keywords are reserved in every syntactic position:

<!-- cinch:reject contains="expected a declaration, found Let" -->
```cinch
module Main where
  let :: Int
  let = 5

  main :: Int
  main = let
```

The reserved keywords are:

`as`, `case`, `ccall`, `codata`, `component`, `contract`, `corec`, `data`, `do`, `effect`, `else`, `erased`, `export`, `forall`, `foreign`, `generate`, `handle`, `if`, `implicit`, `import`, `in`, `infix`, `infixl`, `infixr`, `instance`, `justify`, `law`, `let`, `letrec`, `macro`, `match`, `module`, `of`, `opaque`, `own`, `prim`, `proof`, `quoteDecls`, `record`, `replace`, `require`, `resume`, `return`, `rewrite`, `Sigma`, `signature`, `spec`, `stage`, `target`, `then`, `to`, `type`, `upgrade`, `using`, `where`, `with`.

Non-ASCII letters are rejected outside string and character literals and comments:

<!-- cinch:reject contains="unexpected character" -->
```cinch
module Main where
  café :: Int
  café = 1

  main :: Int
  main = 0
```

# Comments

`--` introduces a line comment extending to the end of the line. `{-` and `-}` delimit block comments, which nest to arbitrary depth. Comment recognition takes precedence over operator runs, so `5 -- 3` parses as `5` followed by a comment:

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

An unclosed block comment is a lexical error:

<!-- cinch:reject contains="unterminated block comment" -->
```cinch
module Main where
  {- open
  main :: Int
  main = 0
```

# Numeric literals

Integer literals are decimal digit sequences or `0x`-prefixed hexadecimal digit sequences. Single `_` characters may separate digits and are stripped during lexing, and leading zeros are canonicalized (`007` is `7`, `0x1F` is `31`). Integer literals carry arbitrary precision through lexing and are range-checked against their expected type during type checking (see [numbers and primitives](05-numbers-and-primitives.md)).

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

A digit immediately following `.` after digits forms a fractional token, which is rejected because floating-point literals are not part of the surface syntax. Construct floating-point values through the conversion functions in [numbers and primitives](05-numbers-and-primitives.md):

<!-- cinch:reject contains="float literals are not supported" -->
```cinch
module Main where
  f :: Int
  f = 1.5

  main :: Int
  main = 0
```

Because fractional recognition requires a digit immediately after `.`, module qualification (`Module.name`) and record projection (`record.field`) remain unambiguous.

# Strings, characters, and symbols

Double-quoted string literals produce `String` values (immutable UTF-8). Raw newlines are not permitted inside string literals. Supported escape sequences are `\n`, `\t`, `\\`, `\"`, and `\u{...}` (hexadecimal Unicode scalar value):

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

Unknown escapes and out-of-range or surrogate Unicode code points are lexical errors:

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

Single-quote (`'`) forms are disambiguated lexically:

| Form | Meaning |
| --- | --- |
| `'c'`, `'\n'` | Character literal (`Char`): one character or escape between single quotes. |
| `'name` | Interned symbol literal (`Sym`). |
| `''Type` | Quoted type name (see [staging and macros](14-staging-and-macros.md)). |

Note that `'ab'` is not a malformed character literal: because no closing `'` follows the first character, it lexes as the symbol literal whose name is `ab'` (with the trailing `'` continuing the identifier):

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

Operator names are sequences of symbol characters drawn from `!`, `#`, `%`, `&`, `*`, `+`, `/`, `<`, `=`, `>`, `?`, `@`, `^`, `|`, `~`, `:`, and `-`. The `$` character may continue an operator (such as `<$>`) but never starts one; `$(` begins a splice. Parenthesizing an operator (`(<+>)`) turns it into an ordinary prefix value.

The exact spellings `::`, `->`, `<-`, `=>`, `!`, `@`, `=`, `|`, and `~` are reserved tokens rather than general operators. Longer runs containing those characters (`<=`, `<+>`) are ordinary operators. As a special case, `|` may be declared and defined as an infix operator (except in guard and effect-row positions, where grammar syntax claims `|` first), whereas `!` is always reserved.

Every infix operator requires a fixity declaration (`infix`, `infixl`, or `infixr`) with integer precedence from `0` (lowest) to `9` (highest). There is no default fixity for undeclared operators. Only the module declaring an operator specifies its fixity; importing modules inherit that fixity automatically.

The standard library defines the following fixities:

| Fixity | Operators | Declaring module |
| --- | --- | --- |
| `infixl 9` | `*`, `/`, `%` | `Algebra.Arithmetic` |
| `infixl 8` | `+`, `-` | `Algebra.Arithmetic` |
| `infixl 7` | `<<`, `>>` | `Algebra.Bits` |
| `infixr 6` | `<>` | `Algebra.Semigroup` |
| `infixl 6` | `&` | `Algebra.Bits` |
| `infixl 5` | `^` | `Algebra.Bits` |
| `infixl 4` | `\|` | `Algebra.Bits` |
| `infixl 4` | `<$>`, `<*>` | `Category.Functor`, `Category.Applicative` |
| `infixl 3` | `<\|>` | `Category.Alternative` |
| `infix 3` | `==`, `/=`, `<`, `<=`, `>`, `>=` | `Relation.Eq`, `Relation.Ord` |
| `infixr 2` | `&&` | `Data.Bool` |
| `infixr 1` | `\|\|` | `Data.Bool` |
| `infixl 1` | `>>=` | `Category.Monad` |
| `infixr 1` | `>=>`, `<=<` | `Category.Monad` |

In `Algebra.Bits`, `|` is a `Bits` method, so `1 | 2` on `Int` fails with `no instance for Bits Int`. A module may declare and define its own `|` operator locally:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  infixl 5 |

  (|) :: Int -> Int -> Int
  (|) a b = a * 100 + b

  main :: Int
  main = if 1 | 2 == 102 then 0 else 1
```

User-defined operators follow the same rules:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  infixl 5 <+>

  (<+>) :: Int -> Int -> Int
  (<+>) a b = a * 10 + b

  main :: Int
  main = if 1 <+> 2 <+> 3 == 123 then 0 else 1
```

Precedences outside `0`–`9`, reserved tokens such as `!`, and unparenthesized chains of non-associative (`infix`) operators are rejected:

<!-- cinch:reject contains="precedence 10 is out of range" -->
```cinch
module Main where
  infixl 10 +

  main :: Int
  main = 1 + 2
```

<!-- cinch:reject contains="expected an operator in the fixity declaration, found Bang" -->
```cinch
module Main where
  infixl 5 !

  (!) :: Int -> Int -> Int
  (!) a b = a + b

  main :: Int
  main = 0
```

<!-- cinch:reject contains="cannot be chained at precedence 3" -->
```cinch
module Main where
  main :: Int
  main = if 1 == 2 == 3 then 0 else 1
```

Function application binds tighter than every infix operator (`f x + g y` parses as `(f x) + (g y)`). Low-precedence right-associative application is written with `$`: `add 1 $ add 2 3` parses as `add 1 (add 2 3)`. See the [library reference](../library/00-index.md) for all standard operator signatures.

# Layout and indentation

The keywords `where`, `of`, `do`, `let`, and `letrec` open an implicit layout block unless immediately followed by an explicit `{`. The first token inside the block establishes its reference column:

- A token on a new line at the **same column** starts a new item (inserts `;`).
- A token at a **smaller column** closes the innermost layout block (inserts `}`), repeating while still to the left of enclosing blocks.
- A token at a **greater column** continues the current item.

Every nested layout block must indent strictly past its enclosing block. Explicit delimiters `(`, `[`, and `{` shield their contents from outer layout rules; a matching closing delimiter closes any implicit blocks opened inside it. The `in` keyword closes the nearest enclosing implicit `let` or `letrec` block.

Indentation must use spaces only; leading tab characters are rejected:

<!-- cinch:reject contains="tabs are rejected in indentation" -->
```cinch
module Main where
	main :: Int
	main = 0
```

Alternatives of a `case` form a layout block opened by `of` and must indent past the enclosing declaration:

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

An expression continued onto the next line must indent strictly past its declaration's column; returning to the declaration's column starts a new item instead:

<!-- cinch:reject contains="expected an expression, found Semicolon" -->
```cinch
module Main where
  main :: Int
  main =
  0
```

Explicit brackets shield layout regardless of indentation, while indented continuation lines extend the enclosing declaration:

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

See [expressions, bindings, and patterns](03-expressions-and-patterns.md) for expression grammar and `let`/`in` scoping, and [surface syntax and grammar](../specifications/surface-syntax-and-grammar.md) for the formal grammar specification.
