# Lexical structure and layout

The compiler lexes source, inserts layout tokens, parses declarations, and resolves operator fixities before checking. Spaces can affect the tokenization of minus, and indentation establishes declaration blocks.

# Names and tokens

Value identifiers begin in the lowercase identifier namespace; constructors, types, modules, and signatures use capitalized names. Qualified module names use dots. Dash-separated names such as `string-from-bytes` are ordinary identifiers. Apostrophes may continue identifiers.

| Spelling | Interpretation |
| --- | --- |
| `x-y` | A dashed identifier when both sides satisfy identifier glue. |
| `x - y` | Binary subtraction. |
| `x-1` | Subtraction; a digit does not satisfy the dash's identifier continuation rule. |
| `f -42` | Application to a signed literal. |
| `(-value)` | Prefix negation of an expression via `Neg`. |
| `'x'` | One-character `Char` literal; wins over symbol tokenization. |
| `'answer` | An interned `Sym` in ordinary expression/pattern context. |
| `''Item` | A quoted type name in staging/macro name context. |
| `#[name(...)]` | An attribute; the name must immediately follow `#[` on that line. |

`--` begins a line comment and takes priority over an operator run. `{-` and `-}` delimit nested block comments. A tab in indentation is rejected. Reserved punctuation includes `=`, `|`, `!`, `@`, `$`, and bare `~`; longer operator spellings use maximal munch where admitted. Bare `|` is contextual: guard/effect-row punctuation or an operator in an operator position.

Integer tokens retain an unbounded decimal digit string until elaboration against the expected type. String literals have their own escape/decoding rules. Fractional floating syntax is not a general runtime numeric literal form; use the admitted conversion/parser vocabulary. A negative literal is not a conversion from a positive value of a narrower signed type: expected-type checking validates its final range.

# Layout blocks

`where`, `of`, `do`, `let`, and `letrec` establish implicit blocks. The first item's column sets the block's indentation. Equal indentation separates items, less indentation closes the block, and a nested body must indent farther. `in` closes the nearest applicable implicit `let` without crossing an explicit delimiter.

Explicit `(`, `[`, and `{` shield their contents from outer layout. Braces and semicolons can replace implicit separation in admitted blocks. When a nested `case` is a binding's right side, its alternatives must indent past that binding's column, not just past the surrounding function declaration.

A `with (name = value) in expression` capability supply has its own `in`. The layout layer recognizes the `in` following that parenthesized supply. A parenthesized handler immediately before a `let`-closing `in` can encounter this ambiguity; bind the handler first or use the unparenthesized form described in the surface amendments.

# Fixity

Operators are method/value names, not built-in arithmetic syntax. A module declares its operator's associativity and precedence using `infix`, `infixl`, or `infixr`, with precedence 0–9. Only the defining module declares its fixity; imported/re-exported bindings carry it with their identity.

Unparenthesized equal-precedence operators with conflicting associativity are rejected. A nonassociative comparison cannot be chained as though `a < b < c` meant conjunction. Write `a < b && b < c` explicitly. Function application binds more tightly than an infix chain; `$` permits low-precedence application.

The actual shipped fixities appear with their declaring signatures in the [library reference](../library/00-index.md). Do not import Haskell's precedence numbers into Cinch: arithmetic uses Cinch's table.

Implementation: `compiler/src/lexer.rs`, `layout.rs`, `parser.rs`. Design grammar and historical amendments remain in [surface grammar](../specifications/surface-syntax-and-grammar.md) and [local amendments](../specifications/surface-grammar-local-amendments.md); some historical status notes there predate implemented features.
