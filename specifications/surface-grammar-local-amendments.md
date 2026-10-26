# Surface grammar local amendments

Source: local draft in this repository (added after the documentation export)  
Added: 2026-10-24

---

[Documentation overview](../overview.md)

Draft 0.1. The exported [Surface syntax and grammar](surface-syntax-and-grammar.md)
is the normative draft 0.2 grammar. This page collects the amendments the
implementation has adopted where the draft EBNF is silent or has been
superseded, so the deltas are auditable in one place. Each entry names its
decision record in `compiler/CONFORMANCE.md` where applicable.

# 1 Type grammar

- **Tuple types.** The draft has tuple expressions `(e1, e2)` and tuple
  patterns but no tuple types, which leaves tuples untypeable. The type
  grammar gains `tuple_type = "(", type, ",", type, { ",", type }, ")"` for
  arity 2 and above. Tuples are structural products; they are not `Sigma`
  (dependent pairs keep their own syntax and arrive with the dependent core).
- **Effect rows as type arguments.** `Thunk {e} a` and `Lazy {e} r a`
  (laziness and reclamation specification) apply an effect row as the first
  argument of a type constructor. In the argument position `{e}`, a lone
  lowercase identifier is the row variable; labels are constructor paths as
  elsewhere in rows. See [Lazy computation and reclamation
  specification](lazy-computation-and-reclamation-specification.md).
- **Effect label arguments.** An effect label takes at most one region
  argument in this draft: `Eval r`. The draft's `effect_label = con_path,
  { atomic_term }` is restricted accordingly for now.

# 2 Binding forms

- **`~` memo bindings.** `~ pattern = term` inside `let` and `where` blocks
  introduces a memo binding (laziness and reclamation specification,
  section 5). The bare `~` is reserved punctuation; symbol operators such as
  `~>` and `<~>` remain valid. In the draft, `lazy`-style deferral had no
  binding form; this replaces the interim reserved word `lazy`, which is
  therefore not a keyword.
- **`~` patterns are not irrefutable patterns.** The draft's `pattern`
  production admits `~p`-style irrefutable patterns only in Haskell; this
  grammar reserves `~` for the binding form. Whether argument positions gain
  irrefutable patterns is an open decision in the laziness specification.

# 3 Records

- The draft's `record_decl` and `field_decl` are adopted with named-field
  construction `R { x = e }` and postfix projection `e.x`. Record patterns
  `R { x = p }` bind fields by name; fields may be listed in any order.
  `codata` shares the record syntax but is reserved for the codata chapter.
  `erased` fields are parsed and rejected until the erasure slice lands.

# 4 Reserved words and punctuation

- Reserved punctuation adds bare `~` to the draft's list (which already has
  `$`, `=`, `|`, `!`, `@`, and the multi-character forms). Maximal munch of
  symbol runs still applies: `~>` is an operator, `~` is not.
- `delay`, `force` and `demand` remain ordinary names; they are builtin
  forms in the bootstrap compiler only where no user definition shadows
  them, and the language-level story is the prelude's (laziness and
  reclamation specification, section 2).

# 5 Attribute annotations

- `#[...]` attributes (attribute annotations specification) prefix
  declarations, constructor and field declarations, function clauses,
  `case` alternatives, guards, and the branches of `if`. Grammar:
  `attribute = "#[", attribute_name, [ attribute_args ], "]"` where the
  name may be dotted and arguments are literals or a `name = literal`
  list. `#` is otherwise unused in the token grammar (comments are
  `--`), so `#[` is unambiguous. The forms are adopted here ahead of the
  implementation slice: the compiler currently rejects `#[` at the
  lexer.

# 6 Bitwise and arithmetic operators

- New operator spellings over the class stack (decision D90/D91):
  `&`, `|`, `^`, `<<`, `>>` are wrappers over the `Bits` class;
  `%` and `/` are the `Rem` and `Div` class methods, whose instances
  cover the mathematical `Nat` and `Int` only (machine integers keep the
  explicit checked/wrapping/saturating families, primitive types
  specification sections 4 and 8). Division or remainder by zero is the
  defined fault (exit 72).
- Precedence follows the Rust reference table, laid onto the draft's
  0-9 scale; the new binary operators are left associative like Rust's:

  ```
  infixl 9 *, /, %
  infixl 8 +, -
  infixl 7 <<, >>
  infixl 6 &
  infixl 5 ^
  infixl 4 |
  infix  3 ==, /=, <, <=, >, >=
  infixr 2 &&
  infixr 1 ||
  ```

  The relative order of the pre-existing operators is unchanged, so no
  previously valid program re-parses differently.
- Bitwise NOT has no operator spelling: both candidate symbols are
  reserved punctuation (`!` and `~`), so `bitNot` remains the spelling.
- A bare `|` is reserved punctuation (the guard and effect-row
  separator) but doubles as the operator `|` in operator-name position:
  fixity declarations, parenthesized operator definitions, and infix
  application. Elsewhere it keeps its contextual role.

# 7 Capability supply clauses

- The runtime capability supply `with (io = e) in body` (runtime
  capabilities specification) adds `with (...) in` to the grammar: a
  `with` followed by a parenthesized binding list is closed by `in`.
  Layout resolves which `in` belongs to which construct by nesting: a
  supply clause's `in` is the token immediately after the clause's
  closing `)`, so `let x = with (io = e) in x in y` interleaves
  correctly, and a `let`'s `in` is unaffected by supplies around it.
- Known ambiguity: a `handle` operand written as a parenthesized
  expression immediately before a `let`-closing `in` (as in `let x =
  handle e with (h) in x`) is indistinguishable from a supply clause at
  the layout layer, which cannot see inside the parens. The paren group
  is read as a supply clause; write the handler operand unparenthesized
  or bind it first. A `with (...)` group without `name = e` bindings is
  rejected by the parser with a supply-binding diagnostic.

# 8 Identifiers, dashes, and minus

- Identifiers may contain dashes: a `-` tight on both sides between
  identifier characters is identifier glue (`int-digits-into`, `x-y`).
  Digits do not continue an identifier, so `1-2` and `x-1` are
  subtraction (`x-1`: the `-` cannot glue `1`, so it is the operator).
- A `-` tight only to what follows (space before, digits after) is the
  sign of a negative literal: `f -42`, `x = -3`, and `(-42)` all treat
  `-42` as one atom. A signed literal is one atom everywhere an atom is
  expected; no `negate` is needed to write a negative number.
- Otherwise `-` is the binary minus (spaced `x - 1`; tight-left before
  digits `x-1`).
- Prefix `-` in a prefix position (after `(`, `,`, `=`, `->`, `then`,
  `else`, `of`, or an operator) negates any expression through the
  `Neg` class method, not only literals.
- Prelude function names use dash style (`nat-to-string`,
  `int-digits-into`); type and class names stay capitalized. Intrinsic
  symbol names (`safelang.prim.*`) are symbol identity, not surface
  API, and are unchanged.

# 9 Binding groups and recursion

- `let b1; b2; ... in e` binds a group; each binding uses the `where`
  declaration grammar (pattern equations, `~` memo bindings, local
  function equations). The group form replaces nested single `let`s.
- `letrec` admits a binding group whose names occur in their own or
  siblings' bodies only under a lambda, a `~` memo body, or a `delay`.
  These are the language's explicit suspension positions, so recursion
  never introduces implicit laziness. An eager cycle is a compile error
  naming the variable and the missing guard ("unguarded recursion; wrap
  the use in `delay` or a lambda").
- Tail recursion MUST run in constant stack: self and mutual tail calls
  in a recursive group compile to jumps or tail-marked calls
  (design/recursive-bindings-and-tail-calls.md section 3); a group that
  cannot be tail-marked falls back to ordinary calls and the compiler
  report says so.

# 10 Collection literals

- `[a, b, c]` desugars to `from-list` at the expected type's `FromList
  c e` instance (elements typed by the literal); unconstrained, it
  defaults to the prelude `List`.
- `{k => v, k2 => v2}` desugars to `from-map` over a `List (k, v)` of
  pairs at the expected type's `FromMap m k v` instance.
- Instances opt in per container; the slice also introduces the
  prelude `Map k v` (ordered map over `Ord k`).
- The desugar is a compile-time macro expansion: when the elements are
  compile-time known (stage constants), the construction is evaluated
  at compile time and the result is emitted as a constant value (no
  runtime `from-list`/`from-map` call); otherwise the expansion is the
  ordinary runtime class call. The fold boundary (which literals folded
  and which stayed runtime) is a compiler report item.

# 11 Module exports and imports

- `export name, ...` declarations re-export names, including imported
  ones (`export Config.parse`, `export module Data` re-exports
  everything imported from `Data`). Without `export` declarations a
  module exports its declared names as before; imports are private
  until re-exported.
- Import renaming: `import Foo as F` renames the module alias;
  `import Foo (x as y, z)` renames and selects members. Duplicate
  unrenamed imported names are a compile error naming both sources;
  renaming is the fix.
- File-type import converters (the TypeScript loader analogy):
  `import data "config.json" as config` binds the converted value of a
  non-source file; the extension selects the converter. Built-ins:
  `json` (to a `Json`-shaped value or a decoded record at the declared
  type), `txt` (to `String`), `bytes` (to `Bytes`). Plugins are
  stage modules registered per extension, running under the P9 staging
  discipline (budget, capability limits, secrets redaction), with
  provenance and the converter identity plus file content hash recorded
  in the manifest (the import's contract covers the conversion, not the
  file bytes). A converter failure is a build error naming the file and
  converter.

# 12 Tick syntax: symbols and names

The tick lexeme `'` has Template Haskell's correspondence (a name
quote) and Lisp's (a quoted symbol); it is one spelling in three
contexts, decided 2026-10-26 at the user's direction.

- **`'foo` in expression or pattern position** is a `Sym` literal
  (primitive-types specification section 9): interned at compile time,
  identity-tested in patterns. Dash names work (`'int-digits-into`).
- **`'foo` in a staging or macro name position** is a value `Name` (the
  `Name` macro signature member of decision D46); **`''T`** is a type
  `Name`. These feed `quoteDecls` token trees and macro APIs that want
  names rather than values.
- **Char literal precedence**: `'a'` (one character between ticks) is a
  `Char` literal and wins; `'a` followed by anything else is a symbol;
  `'foo` is always a symbol. The rule is lexical and deterministic:
  tick, one character, tick means `Char`.
- One lexeme, no dual spelling: there is no `:name` keyword form.

The runtime value form of a name is the interned `Sym`; the staging
form is a `Name`. A macro that reifies a `Name` into a symbol literal
gets the interned constant for the same spelling.

# 13 Rejected for now (see compiler/CONFORMANCE.md)

Everything else in the draft EBNF parses only to a named rejection:
`do` bodies and handlers, quotations and macros, `rewrite`/`upgrade`
declarations, type and capability application with `@`, tuples of arity 1
(none), and dependent types. Each has a roadmap slice and will move from
this list to an amendment or to the draft grammar as it lands.
