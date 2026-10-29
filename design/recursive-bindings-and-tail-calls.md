# Recursive bindings, tail calls, and surface ergonomics

Design note for the language-surface DX batch (dash identifiers, minus
ergonomics, multi-binding `let`/`letrec`, tail calls, class-desugared
literals). The normative rules live in
`specifications/surface-grammar-local-amendments.md`; this note records
the reasoning, especially for `letrec`. No em dashes.

## 1 The minus/identifier whitespace contract

Three natural cases fight each other: `x-1` as subtraction, `f -42` as
application to a negative literal, and `int-digits-into` as one
identifier. No single token rule gives all three; the resolution is one
whitespace rule with three outcomes for the `-` character:

- `-` tight on BOTH sides between identifier characters: identifier
  glue (`int-digits-into`, `x-y`). Dashed identifiers are written tight.
- `-` tight only to what follows (space before, digits after):
  negative-literal sign (`f -42`, `x = -3`, `(-42)`). This is the
  "no `negate` for literals" case: a signed literal is one atom
  everywhere an atom is expected.
- Otherwise: the `-` operator (spaced `x - 1`; tight-left `x-1` is
  subtraction because digits do not continue an identifier).

`1-2` is subtraction (digits cannot continue an identifier). `x -1` is
application to a negative literal (space before, tight after). The one
hard case, `x-y`, is an identifier by the user's request; subtraction of
two names needs spaces. This is Scala 3's trade-off and it is
explainable in a sentence: tight dashes glue, spaced dashes subtract.

For NON-literal negation the `Neg` class method stays (`negate` or its
dashed spelling), but prefix `-` in a prefix position (after `(`, `,`,
`=`, `->`, `then`, `else`, `of`, an operator, ...) negates any
expression, not just literals.

## 2 Multi-binding `let` and `letrec`

`let b1; b2; ... in e` binds a group (each binding a pattern equation, a
`~` memo binding, or a local function equation, exactly the `where`
declaration grammar). Recursion is the hard part.

The language's ethic: laziness is explicit. Haskell's `letrec` ties the
knot through implicit thunks; that would make `letrec x = x + 1` build a
silent black hole, contradicting the memo/`Eval r` discipline the black-
hole rule exists to protect. OCaml's `let rec` restricts to syntactic
values; that is honest but rejects legitimate lazy cycles.

The chosen rule (guarded letrec):

- `letrec` (or `rec` on the binding group) allows a binding's name to
  occur in its own and its siblings' bodies **only under a lambda, a
  `~` memo body, or a `delay`**. Those are exactly the language's
  existing suspension positions, so recursion is guarded by the same
  constructs the language already makes visible.
- An eager cycle (`letrec x = x + 1`) is a compile error naming the
  variable and the guard it lacks ("unguarded recursion; wrap the use in
  `delay` or a lambda"), so the diagnostic teaches the fix.
- Local functions (`f = \x -> ... f ...`) are values (closures), so
  ordinary recursion and mutual recursion through `letrec`/`and`
  groups work naturally; this also feeds the tail-call rule below.

This keeps "no silent laziness" intact while giving the DX of
recursive local definitions.

## 3 Tail recursion

MUST-level: a tail-recursive definition (self or mutual) runs in
constant stack regardless of call depth. Compilation:

- Self tail calls compile to a jump to the entry (loop), including
  through the saturate/grow/over-apply cases of the apply stub where the
  callee is statically known.
- Mutual tail calls within a `letrec`/top-level recursive group use
  LLVM `tail`/`musttail` marked calls so the C ABI boundary does not
  force frames; a group that cannot be tail-marked falls back to the
  trampoline-free normal path and the compiler report says so.
  `musttail` needs identical prototypes, so a group whose members'
  tail calls cross arities lowers under one widened signature (its
  widest member's arity): each narrower member's body runs in a worker
  `@"<unit>#tail"` taking that many words, the unit's own symbol stays
  as a one-call wrapper for closures and exports, and direct calls go
  to the worker (`core.rs::tail_widths`). Only a call that does not
  saturate its callee still falls back.
- Non-tail recursion is unchanged (ordinary calls). A native program
  runs on a 1 GiB reserved stack (`cinch_run_main`, overridable with
  `CINCH_STACK_SIZE`), so a non-tail recursion hundreds of thousands of
  frames deep is an ordinary program rather than a crash.

The test shape: a 1,000,000-deep tail-recursive sum and a two-function
mutual ping-pong both exit 0 in constant stack.

## 4 List and map literals as class desugars

- `[a, b, c]` desugars to `from-list` applied to the prelude `List` of
  elements; `from-list :: List e -> c` is the `FromList c e` class
  method. The expected type selects `c` (bidirectional); unconstrained,
  it defaults to `List e`.
- `{k => v, ...}` desugars to `from-map` on a `List (k, v)` of pairs via
  the `FromMap m k v` class.
- Instances: `List e` (identity), `Map k v` (the new prelude ordered
  map), `Bytes` (from a list of bytes), and `String` where meaningful.
  A `Map k v` prelude type (balanced or unbalanced BST over `Ord k`,
  documented as a bootstrap structure) comes with the slice.

Both classes are ordinary classes: containers opt in by instance, no
new compiler mechanism beyond the literal desugar.

## 5 Prelude naming

All prelude FUNCTION names move to dash style (`nat-to-string`,
`int-digits-into`, `string-from-bytes`, `bit-and` as the method
spelling with `bitAnd` as a transitional alias if needed). Type and
class names stay PascalCase (`String`, `Bits`, `FromList`). This
supersedes the D6/D64 spelling notes for the prelude's own API; the
intrinsic symbol names (`cinch.prim.*`) stay as they are (they are
symbol identity, not surface API).
