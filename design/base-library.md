# Base library: module scheme and buildout plan

Plan for the base-library buildout (2026-10-26, user direction: split
the Prelude into modules, re-export the wanted surface, flesh out the
category-theoretic classes, principled naming). No em dashes.

## The naming scheme

A module is named for the mathematical KIND of its contents:

- `Category.*`: classes over type constructors (`f :: Type -> Type` and
  friends): Functor, Applicative, Monad, Alternative, Foldable,
  Traversable, Bifunctor, and the universal visitors (Identity, Const,
  Compose).
- `Algebra.*`: value-level operations and laws on a carrier: Semigroup,
  Monoid, Group, Semiring, Lattice. The newtypes that exist only to
  carry instances (Sum, Product, Dual, First, Last, Endo) live with
  their class module.
- `Relation.*`: value-level comparison and decision: Eq, Ord,
  Decidable (the core `Dec`).
- `Data.*`: concrete types only: List, NonEmpty, Option, Result,
  Ordering, Nat, Int, Char, String, Bytes, Sym, Map, Set, Tuple.
- `Function.*`: combinators over plain functions.
- `Text.*`: conversion to and from String/Bytes (Show, Codec; the
  latter aligns with the A6 CodecGenerator vocabulary).
- `Effect.*`: the effect system's vocabulary: Effect.Sequencing (the
  D22 do providers), Effect.Io, Effect.Concurrent (D74-D81).

Ownership rules:

1. A class lives in the module named for its structure
   (`Monoid` in `Algebra.Monoid`).
2. Instances live with their type; structural instances over functions
   and tuples live with the class module; instances never live
   elsewhere (no orphans, matching the coherence discipline).
3. A class constraint does not entail other classes (the standing
   no-superbundle rule): functions state every constraint they use, so
   `Algebra.Monoid` imports `Algebra.Semigroup` explicitly.
4. A home that writes an operator infix declares that operator's fixity
   in its own module (`infixr 6 <>` beside the `import
   Algebra.Semigroup`). Within the embedded family the import supplies
   the operator's NAME and the module must supply the fixity, because
   the family is parsed in two passes and an early home cannot see a
   late home's table (D381). Outside the family the import alone
   suffices: the fixity travels with the compiled interface (D360).

`Prelude` becomes the re-export facade (D107's `export module X` and
`export A.b`), with an explicit curation list. The split itself is an
integration step after the P19 surface slice lands (it owns
`compiler/prelude/Prelude.cinch` and is renaming functions to dash
style (D106)).

## The buildout (dispatched)

- `Algebra.Semigroup` + `Algebra.Monoid`: `Semigroup` (`<>`),
  `Monoid` (`mempty`), newtypes Sum/Dual/First/Last/Endo, instances
  for List, Option, Ordering, Nat under addition and multiplication
  (via newtypes), String, functions, and tuples.
- `Category.Functor`/`Category.Applicative`/`Category.Monad`:
  `map`/`<$>`, `pure`/`<*>`, `>>=`/`join`, `Identity`/`Const`/
  `Compose`, instances for List, Option, Result e, functions ((->) r).
  (`map` is the lisp-style name per the D106 direction; `Monad` has no
  `return` method since `return` is handler syntax.) The
  relationship to the `do` providers (D22) is documented, not
  changed: `do` picks providers lexically; the classes abstract over
  containers.
- `Category.Foldable`/`Category.Traversable` + `Data.NonEmpty` (the
  free semigroup): `foldr`/`foldl`/`toList`/`traverse`/`sequence`,
  instances; `traverse` into `Result e` and the effect rows is the
  showcase.

## NonEmpty via instance delegation (user direction, 2026-10-26)

The bespoke `Data.NonEmpty` (head+tail with hand-written instances) is
dropped after A7 in favor of a newtype-style wrapper that delegates
all instance information to the internal container:

- The wrapper is a `newtype`-as-`repr(transparent)` single-field type
  over `List`; delegation is dictionary coercion through
  wrap/unwrap (zero cost, safe by the representation identity that D86
  pins in the ABI).
- Delegation is opt-in PER CLASS because law transfer is not blanket:
  `Semigroup` transfers (append of non-empties stays non-empty),
  `Monoid` does not (`mempty` is empty). The P18 law-axiom layer
  checks transfer per derived instance, which is the mechanism's
  honest gate.
- The invariant lives at the smart-constructor surface
  (`from-list`/`singleton`); the raw constructor stays out of the
  `export` list (D143's surface rule enforces the boundary). Generic
  behavior is delegated; only the invariant is bespoke.

This generalizes: any "sub-kind of a container" wrapper (NonEmpty,
Sorted, NonZero, Ascii) uses the same delegation form.

## Shared generic vocabulary (user direction, 2026-10-29)

The `Data.*` homes are the only place a generic operation over their
type is written. A module that walks a container uses the home's
polymorphic operation; it does not re-derive a type-specific copy
(`reverse-nats`, `rev-chars`, `list-reverse`, `j-reverse` were all one
function, and the port wave had grown a dozen of them).

`Data.List` carries: `reverse`, `take`, `drop`, `nth`
(`Option`-returning), `replicate`, `list-length` (`Nat`, beside the
`Int` `length` the prelude surface has always had), `is-empty`, plus
the pre-existing `map`/`foldr`/`sum-list` and append as `<>`.
`Data.Char` carries `char-code` (scalar as `Nat`) and `char-from-code`
(partial, exactly as `u32-to-char` is). `Data.String` carries
`string-concat-all` and `string-repeat`. All three are members of the
embedded family, so every module reaches them with no import; the
curated facade re-exports them by the ordinary `export module` route.

Two consequences worth stating, because both were live traps:
the names shadow cleanly (a module may still define its own `reverse`,
and local definitions win), so this is an additive change to the
prelude surface, not a breaking one; and a conversion written as
`nat-from-i64 (u32-to-i64 (char-to-u32 c))` is `char-code`, never a new
private `code-of`.

## Packages and the manifest (`.cinchpkg`, user direction 2026-10-29)

Resolution order is data, not code. The embedded family used to be two
hand-written arrays in `compiler/src/driver.rs` (`PRELUDE_EARLY`,
`PRELUDE_LATE`) whose ORDER was load-bearing in four separate ways: the
fixity thread, eager references in signature types and constraint
classes, the check/interface order, and the facade parsing first but
collecting in the middle. Those arrays become one declarative file.

- The file: `compiler/prelude/cinch-base.cinchpkg`, canonical JSON
  (design/stdlib-and-bootstrap.md fixes canonical JSON as the manifest
  format, and `.cinchif` is the precedent), `schema: 1`.
- Fields: `name`, `version`, `modules[]` (each `name`, `path`,
  `embedded`, `imports`, and `facade` on the re-export facade), and
  `order` (the total check order). The facade's INDEX in `order` is what
  splits the early phase from the late one; there is no second list.
- The Rust side keeps the sources (`include_str!`, so the family stays
  embedded and hash-stable per D161-D174) in an UNORDERED path-to-source
  table, and parses the manifest for everything else. A test asserts the
  two agree name for name, so drift is caught rather than trusted.
- A user package ships `.cinchpkg` beside its entry: the module list
  supplies name-to-path resolution ahead of the decision D42 directory
  scan, which is what a library in subdirectories needs.
- What this does NOT remove, stated so nobody pretends otherwise:
  eager-reference collection order still needs the `order` list (imports
  alone do not encode it), and an early home still cannot see a late
  home's fixity table (D381) until a fixity pre-pass gathers every
  module's `infix` declarations before the ordered parse.

## Shared types and signature cycles (user direction, 2026-10-29)

A signature's type NAME resolves only against the modules checked before
the module being collected. That is fine for a tidy dependency chain and
impossible for a real cycle, and the base library has one: `Data.List`'s
`nth` answers an `Option`, `Data.Option`'s `cat-options` consumes a
`List`, and `Data.Result`'s `partition-results` consumes a `List` while
being checked before either. No order of those three can work.

- The idiom: declare the shared inductive types in an INTERNAL family
  module ordered ahead of every home, and have each public home
  re-export its own type by name. `compiler/prelude/Data/Core.cinch`
  declares `List`, `Option` and `Result` with their constructors and is
  first in the manifest; `Data.List`, `Data.Option` and `Data.Result`
  `import Data.Core` and keep the type names in their `export` lines, so
  their surfaces, their member imports (`import Data.Option (Option,
  None, Some)`) and the facade are all unchanged. `Data.Core` is
  deliberately NOT re-exported by the facade: the names reach the
  implicit surface through the homes, exactly as before.
- A type's INSTANCE family stays in its public home, not in `Data.Core`.
  The homes are where a caller looks for behavior, and the instances
  name the type by a name `Data.Core` provides.
- One name, one declaration site: a module that imports `Data.Core` must
  NOT also import a re-exporting home for the same names, because a name
  imported from two homes is a reported ambiguity, not a silent pick.
  That is why `Data.List` and `Data.Result` import `Data.Core` only.
- Reach for this only for a genuine cycle. For a merely misplaced type
  (or a name that needs to be visible before its home), the manifest
  order is the cheap tool: `Data.Ordering` is first for `Relation.Ord`'s
  `comparing`, and moving one line of `cinch-base.cinchpkg` did it.
- Considered and rejected for now: a checker-side declare pass that
  registers every module's type names before the ordered walk, which
  would make forward references legal everywhere. It widens what
  `unknown type` means across the language (and its diagnostics) to fix
  a problem the library expresses in one small module, so the mechanism
  stays an option for a future decision rather than the default.

## Compiled artifacts and the package store (user direction 2026-10-29)

The user's goal: the compiler STORES compiled artifacts for a package set
and LOADS them, rather than re-parsing the embedded sources on every
compilation. What exists today and what each stage needs, from the
reconnaissance of `interface.rs`, `incremental.rs` and the codegen path:

- Today a `.cinchif` per module is WRITTEN into `<entry>/build/` and
  never read back (`Iface::from_json` has no production caller), and the
  embedded family's interfaces record an empty `source_hash` (fixed: the
  family's sources are hashed now, so an artifact can witness the bytes
  it came from).
- Stage 1 (done): the base package's module list and order are data
  (`cinch-base.cinchpkg`), its sources are hashed, and a package key over
  source bytes plus schema, ABI, language and compiler versions is the
  identity an artifact directory is named by.
- Stage 2: load FIXITIES from stored interfaces (`exports.fixities`
  already has the rows), so a consumer's parse stops needing a parsed
  module just for the operator table.
- Stage 3: load DECLARATION SURFACES, which needs the inverse of
  `render_scheme` (a scheme/type-string parser) and a
  `collect_module_from_iface` beside `collect_module`.
- Stage 4 is the one that actually stops re-compiling the prelude:
  prelude BODIES are load-bearing today (they feed whole-program emission,
  compile-time evaluation and law sample checking), so the prelude leaves
  the compile only when a module's code ships as an object and the link
  path consumes objects instead of re-emitting everything. That is the
  architecture decision, and it is the user's: per-module objects plus a
  real link, or a bodies payload keyed by `iface_hash` inside the
  artifact.

Decisions (made by the user, 2026-10-29, and implemented in the store
slice): the store is a per-user content-keyed cache with a `CINCH_STORE`
override; bodies ship as BOTH per-module objects and an IR form, so
cross-module inlining and rewrite rules stay possible; and a stale or
corrupt artifact is a hard error with a `--rebuild` escape rather than a
silent recompute.

The boundary that makes the hard error safe, and that the first
implementation got wrong: a changed source, profile, adapter set or
schema moves the CONTENT KEY, so it is a MISS (compile, write a new
entry) and never a fault. The hard error is for an artifact that
disagrees with its OWN recorded identity (bit-rot, a truncated or
interleaved write) or for a schema/ABI/language/compiler identity
mismatch, and it names which one. Entries are staged in a private
directory and swapped in with `rename`, so two writers of one key cannot
interleave. The compile path is write-only until loading is safe: the
checker installs family state and codegen needs bodies, which a rendered
contract does not carry, so consulting the store from a compile is a
later stage's decision, not a default.

## Reported and not yet placed (slice reports, 2026-10-29)

Named here so the gaps are decisions rather than omissions:

- `Foldable`/`Traversable` instances for `Identity`: `Data/Identity.cinch`
  ships `Functor`/`Applicative`/`Monad` only, so `fold-map` over
  `Identity` has no instance to resolve and the acceptance item could not
  be exercised (the checks cover `List`, `Option` and `NonEmpty`).
- The concrete `Data.List` surface carries `find`/`find-index`/
  `elem-index` but not `elem`/`not-elem`/`any`/`all`/`sum`/`product`:
  those are the `Category.Foldable` free functions, reachable only by
  importing that class home (the facade re-exports no class home). The
  sweep refused to duplicate them into `Data.List` on purpose.
- `fst`/`snd`/`swap`/`curry`/`uncurry` sit in `Function.Combinators`
  because a tuple has no home module; `Data.Tuple` would be the honest
  home and needs a manifest + facade edit when a second consumer appears.
- `Data.Path.eq-chars` (no base-layer `Eq (List Char)`, D368),
  `Text.Regex.concat-strings` (importing `Text.Scan` would cycle), and
  the byte-typed (`Nat`) digit/hex helpers in the `Text.*` homes and
  `Effect.Random` are deliberate keeps; `D366` (the
  capability-wrapper miscompile) and `D381` (an imported fixity does not
  reach a family module) stay open.

## The brainstorm menu (resolved 2026-10-26, user approved)

Pulled in: `Algebra.Group` and `Algebra.Semiring` (the Sum/Product
newtypes keep the numeric monoids out of any Num tower); `Data.Set`
plus `Data.Hash`/`Data.HashMap` (Sym interning makes symbol hashing
trivial); law declarations wired into the rewrite `justify` machinery
(riding the P18 law/assumption identity tier: a class `law` is citable
evidence, and instances are checked against their laws); `Text.Show`
and `Text.Codec` (aligned with the A6 CodecGenerator).

Queued: `Data.Enum` and range syntax `[1..10]` ride the next surface
slice (parser work, not bolted onto P19 mid-flight). `Data.NonEmpty`
is already building with the Traversable family.

Held until a consumer appears: `Category.Profunctor`/`Category.Category`
/`Kleisli`, `Effect.State`/`Effect.Reader` as newtypes.

Deliberately out (unchanged): the monad-transformer/mtl zoo (row
effects with handlers (P6) do that job natively), IO-as-a-monad
(`Io r` is capability-typed and lexical per D21), a numeric tower
beyond Semiring (the typed families with explicit conversions are the
spec's deliberate choice), lenses (later).
