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

# 6 Rejected for now (see compiler/CONFORMANCE.md)

Everything else in the draft EBNF parses only to a named rejection:
`do` bodies and handlers, quotations and macros, `rewrite`/`upgrade`
declarations, type and capability application with `@`, tuples of arity 1
(none), and dependent types. Each has a roadmap slice and will move from
this list to an amendment or to the draft grammar as it lands.
