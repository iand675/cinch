# Character readings in conversion contracts

`Data.Char.char-to-u32` promises its actual input's scalar value and the
inclusive range 0 through 1114111. The equality is explicit in its source
declaration. The primitive checker already guarantees that identity, and its
integer proof sort already gives `Char` the same range. The interface writer
renames binders; it does not invent the equality.

Two narrower tables omitted characters. The portable Rust contract writer
could not record the character binder and left `resolved: null`. Native's
predicate conversion then rejected the unchanged display scheme because it
could not promote that checked character value to a mathematical reading.

The correction retains the source contract. Characters acquire a proof-only
promotion and proof bounds, and the portable writer treats their readings as
machine values whose arithmetic remains opaque. Runtime integer admission,
numeric literals, integer bounds, Unicode constructors, primitive promises and
the interface schema are unchanged. The inclusive bounds overapproximate the
valid scalars; they do not admit the surrogate interval as character values.

`compiler/tests/interface_machine_contracts.rs` checks three independent
obligations:

- Source conversion and scalar-bound declarations export resolved contracts
  through a facade without losing equality, bounds or the `Char`/`U32`
  carriers. Character arithmetic remains outside the portable encoding.
- A real compiled Cinch checker rechecks the exact input identity and implicit
  scalar bounds. It rejects the wrong input identity, stronger bounds, a false
  result range, numeric character literals, runtime character arithmetic,
  character-to-integer coercion and floating proof readings. The same cases
  run with resolved contracts and coherent display-only contracts, retaining
  compatibility with existing schema-10 interfaces.
- `programs/Run.CharConversion.cinch` exercises literal conversion and valid
  scalar round trips at zero, ASCII, Greek, supplementary and maximum scalar
  values. Surrogates and integers above the scalar range still return `None`.

`character-proof-contract-results.json` records an isolated emitter validation:
the frozen Rust compiler checked the ordinary pipeline entry's reachable
closure in a 51-file source snapshot and built an O0 emitter.
All 31 expected checking outcomes passed, including the nine source cases,
native defining and facade metadata, the same nine cases through both resolved
and display-only reexports, and the old/new direct conversion comparison. The
Unicode runtime probe passed normal and poisoned-free execution with zero
leaks against the original display-only library interface.

This is partial validation. The current Rust exporter tests, preserved compiler
gates and a new full native bootstrap fixed point remain required before
integration. The earlier scoped-argument fixed-point receipt belongs to its own
source snapshot and does not verify this correction.
