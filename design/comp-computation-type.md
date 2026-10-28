# The value-level computation type former (`Comp {e} a`)

Status: design settled 2026-10-28 (D327). Implementation rides the
P33 streaming slice; this note is the decision record.

## The problem

Cinch puts effects on arrows: `read-line :: Handle ->{IO r, Fs r}
String`. A function arrow's row says what applying the function may
do; `handle ... with` interprets the operations of a computation.
What the language lacks is a TYPE for "a computation, as a value":
something a data structure can carry, so traversal targets can hold
effectful steps. The concrete demand is P33: a pull-based `Stream`
over IO sources where each step is an effectful read. A `Stream a`
of pure values cannot express "the next element requires `IO r` to
produce"; the stream's elements must be effectful computations, or
the stream type itself must carry the row. The A7 backlog called the
missing piece a "value-level computation type former for effect-row
traversal targets (`Comp {e} a`-like)".

## Options

1. **`Comp {e} a` as a library newtype over effectful unit closures**:
   `newtype Comp (e :: Effect) (a :: Type) = MkComp (Unit ->{e} a)`
   with `suspend :: (Unit ->{e} a) -> Comp {e} a` and
   `run :: Comp {e} a ->{e} a`. The type former exists at the kind
   level (`Comp :: Effect -> Type -> Type` - the row parameter is an
   effect-row type argument, the machinery example 11 already uses
   for `Request {e}`), the value is an ordinary latent closure, and
   running it is ordinary application.
2. **A core type former** with its own intro/elim (`suspend`/
   `force`) distinct from closures, carried through elaboration,
   codegen and the evaluator as a new form.
3. **Rows on the container instead**: `Stream {e} a`-style, each
   traversal type carrying its own effect row, computations never
   becoming values.

## Decision (D327): option 1

- `Comp` is ordinary prelude code (`Effect/Comp.cinch` home in the
  `Effect` family), a `repr(transparent)` newtype over
  `Unit ->{e} a`. No new core forms; the type checker, CPS
  elaboration (D24), codegen and the D92 evaluator are untouched.
- **Why not 2**: a core form buys runtime nothing (the closure IS
  the representation) and costs elaboration surface everywhere.
  The whole project's taste is boring machinery over new core forms.
- **Why not 3**: it spreads rows across every container type and
  still needs the "effectful step" type inside `Stream`'s own
  observation; `Comp` solves the general problem once.
- **Semantics**: `run (suspend k) = k ()` definitionaly at the
  library level; `suspend`/`run` are `%1`-neutral (the closure's own
  multiplicity governs). Running a `Comp` is an application, so the
  effect dispatch discipline holds exactly as for direct calls
  (D24's boundary: handled computations dispatch; lambda bodies are
  latent and `Comp` values are lambda bodies in a box). `Comp`
  values are ordinary immutable data: freely stored, passed, and
  (with `Share` closures) shared.
- **Handlers**: `handle (run c) with h` is the ordinary shape; a
  `Comp` carries its row in its type (`Comp {IO r, Fs r} Chunk`), so
  the row-polymorphic signature discipline applies unchanged
  (`forall r. ... -> Comp {IO r} Chunk`).
- **Stream integration (P33)**: the pull-based `Stream` over IO
  sources yields `Comp {e} a` steps (or exposes `next :: Stream e a
  ->{e} Option (a, Stream e a)`); resource safety is the P8 codata
  plus the region bracket story (acquire/release in the row, `Ref`
 /`Mut` region discipline). P33's slice owns the exact Stream shape;
  this note only fixes what `Comp` is.
- **Refusals kept honest**: no implicit sequencing of `Comp` values
  (they are data until `run`); no effect-row subtyping on `Comp`
  beyond the existing implication machinery if any; `Comp` does not
  get a `Monad` instance as compiler magic - if the library wants
  `instance Monad (Comp {e})`, it is an ordinary library instance
  with ordinary obligations.

## What implementation owes (P33)

- `Effect/Comp.cinch`: the newtype, `suspend`/`run`, and any
  instances (with the D159 home rules).
- `Stream` over IO sources using `Comp` for its steps; region
  bracket acquire/release.
- Tests pinning: `run`/`suspend` round trip, a `Comp` stored in a
  data structure and run later under a handler, and the refusal of
  implicit sequencing.
