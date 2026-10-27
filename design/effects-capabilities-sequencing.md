# Effects, capabilities and sequencing design

Design note for the Cinch toolchain, roadmap slice P6 in
`compiler/CONFORMANCE.md`. Companion to the runtime capabilities, effects
and sequencing specification, which fixes the three independent choices
(effect rows, lexical capabilities, sequencing modules); this note fixes the
surface forms, their elaboration, and the runtime shape.

Status: design accepted for implementation.

# 1 What already exists

Effect rows are row-typed in the checker (labels, tails, lacks constraints),
effect honesty is checked by row unification, and body effects ride the last
arrow of a curried definition. Memo/thunk machinery already threads `Eval r`
labels and discharges them at scope end. This slice adds the *handlers* and
*capability plumbing* the spec describes, on top of that row system.

# 2 Effect vocabulary

Effect labels are constructor paths with optional region arguments:
`IO r`, `Async r`, `Cancel r`, `Blocking`, `Diverge`, `Foreign Name`,
`Eval r`. A signature may quantify row variables (`forall e.`), and
higher-order APIs quantify over rows (the `traverse` example: `(a ->{|e} b)
-> List a ->{|e} List b`). The checker's existing rows cover this.

New in this slice: user-declared effects (`effect_decl` in the grammar:

```haskell
effect Async (r :: Region) where
  await :: Task r a ->{|Async r} a
```

) whose operations are ordinary functions performing the label; handler
forms below discharge them.

# 3 Handlers

Surface (draft EBNF): `handle term with handler` and

```haskell
handler Async r where
  return x -> pure x
  await t resume k -> ...
```

Elaboration: `handle e with h` is checked as: `e` performs a row containing
the handled label(s); the handler clause bodies must type to the result
type; the handled labels are removed from `handle`'s own performed row (the
type-level analogue of the memo discharge in the laziness specification),
and any effects the clause bodies perform are added instead. Resumptions
(`resume k`) are one-shot in this design (the core specification requires
multi-shot resumption to carry duplication evidence; the bootstrap provides
one-shot only and rejects a second `resume` with a usage error from the P5
multiplicity checker).

The core elaborates `handle` to a runtime handler record: a vtable of
operation clauses plus a return clause, with the handled computation
lowered to a function receiving the handler. The bootstrap runtime needs no
support beyond closures and dictionaries: a handler is a dictionary of
closures, and operations dispatch through it. (A later optimization may
specialize handlers to direct style.)

# 4 Capabilities

`using (io :: Io r) => ...` is a dictionary parameter in the existing class
system: `Io r` is a signature whose instance is the runtime's I/O
implementation for identity `r`. `with (io = chosen) in body` supplies the
dictionary lexically (elaborates to an explicit dictionary application);
`f @{io = chosen}` is the explicit call-site form. No thread-local lookup
exists, per the spec.

Runtime identity `r` is a region-like name carried in effect labels
(`IO r`), so the escape rules of the ownership design prevent I/O
capabilities leaking past their scope. `Io r` instances are constructed at
the entry point (a runtime creation form, see section 6).

# 5 `do` notation and sequencing modules

`do { x <- e; rest; final }` with direct sequencing by default. The
sequencing module is selected by the enclosing module's parameter or
signature (`module ImportConfig (Seq :: ResultSequencing ConfigError)`);
the three initial providers are direct binding, `Result` short-circuiting
and `Option` short-circuiting. Elaboration: `do` desugars to the provider's
`bind`/`pure` (class methods of a `Sequencing` signature), so the same
spelling types differently per provider. Refutable patterns in `x <- e`
require the provider to supply a typed failure operation; direct sequencing
rejects them (the draft says so).

# 6 Entry points and runtime scopes

An entry point installs handlers and supplies capabilities:

```haskell
main ::{IO r} Int
main = with (io = Runtime.newIO ()) in
         handle run with catchAll
```

The bootstrap keeps `main :: Int` pure (current driver rule) and adds
`main ::{IO r} Int` support here: the driver generates a C-ABI `main` that
constructs the runtime identity and runs the computation, mapping failures
to the runtime fault policy. Capability construction is explicit; nothing is
global.

# 7 Implementation mapping

- `parser.rs`: `effect_decl`, `handler_expr`, `handle`, `with`, `do`, `@{...}`
  (all currently named rejections).
- `check.rs`/`check_terms.rs`: handler typing (label discharge + clause
  effects), `do` desugaring through sequencing dictionaries, capability
  dictionaries via the existing class machinery, `@{...}` explicit
  dictionary application.
- `core.rs`/`codegen.rs`: handler records (dict of closures), one-shot
  resumption closures, `do` bind as ordinary calls.
- Runtime (`cinchruntime`): `Runtime.newIO`, a fault policy for unhandled
  effects (an unhandled operation is a defined `BlackHole`-class fault with
  its own exit code), and the identity allocation for `r`.

# 8 Open decisions

- Multi-shot resumption and the duplication evidence it requires.
- Whether handler labels are row-discharged (as here) or typed as an
  explicit `Handled` row difference.
- The full effect vocabulary of the spec (async, cancellation, blocking)
  and their interaction with the concurrency slice's scheduler.
- Whether `main ::{IO r} Int` should instead be `main :: (io :: Io r) ->{IO
  r} Int` with the runtime as an explicit argument.
