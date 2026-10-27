# Standard library expansion and the bootstrap program (P30-P35)

Roadmap plan, 2026-10-27 (user direction). The framing is the
bootstrap test: every family is ranked by what a Cinch compiler
written in Cinch needs. No em dashes.

## The families and where they land

1. **Parser combinators + lexing (P30)** - megaparsec/flatparse
   territory. The layout machine and span model are already normative
   (the tree-sitter port proves the context rules); parser errors are
   an effect row (`Parse e`) with handlers for recovery, which is the
   DX story. Work: combinators with backtracking control, spans,
   error bundles; token-level lexing that cannot drift from the layout
   spec. Flatparse-style performance is a later pass, correctness
   first.
2. **JSON (P31)** - aeson territory, and half-built already: the
   `Json` family exists (D148), `Text.Codec` has `encode`/`decode`,
   A6 has `CodecGenerator.derive`. Work: a real parser and printer,
   **canonical JSON** (manifests and `.cinchif` round-trips must be
   deterministic), `JsonCodec` instances aligned with `Text.Codec`,
   schema alignment with the A6 vocabulary. FIRST priority: everything
   above us is JSON (manifests, schemas, trace payloads).
3. **Observability (P32)** - pluggable logging/metrics/tracing with
   OTel emission and a lovely DX. The capability system IS this:
   `Log r`/`Trace r` capabilities in the `Io r` mold (D21/D81), spans
   as lexical scopes (the `withScope` shape), trace context as the
   runtime identity `r` (D24-D26). Work: structured records (Json),
   an OTLP/JSON handler, W3C traceparent propagation, span/metric/
   log APIs as ordinary effect operations.
4. **Streaming and resources (P33)** - iterator-like IO streams. The
   codata `Stream`/`Later` (P8) is the lazy stream; `WasiStream`/
   `WasiFuture` are the degraded IO forms; regions give bracket
   semantics for acquire/release. Work: pull-based `Stream` over
   sources (files, stdin, sockets) with resource-safe lifetimes.
   FLAG: this family, observability, and parser errors all want
   `Comp {e} a` (a computation as a value) - A7's open design item.
   Settle it before this family, not after.
5. **Platform layer (P34)** - time, filesystem, Unix API. Capability-
   typed: `Clock` (monotonic + wall; keeps builds deterministic per
   D45), `Fs` (a real `Path` type - the prelude's `Path over Nat` is
   a placeholder; read/write/list/metadata/atomic rename), `Process`
   (spawn/exec for driving clang and lld, env, args, exit codes,
   pipes - bootstrapping depends on this).
6. **Bootstrap milestones (P35)** - the acceptance ladder:
   (1) a Cinch program parses the language's own grammar (P30),
   (2) reads and writes a manifest `.cinchif`-canonically (P31),
   (3) type-checks a fragment (the checker port),
   (4) emits LLVM IR text (the codegen port),
   (5) drives clang via process spawn (P34),
   (6) compiles an example with itself.

## Also needed for bootstrapping (not on the original list)

- Deterministic hashing at content-addressing strength (SHA-256
  level; fnv1a is not enough for cross-machine identity; P29's
  caching depends on it).
- CLI argument parsing (the driver port).
- Canonical serialization round-trips for `.cinchif`/manifests (part
  of P31).
- A Cinch test runner (the bootstrap needs to run its own tests).
- Real `Set`/`HashMap` implementations (the current HashMap is a
  documented 8-bucket bootstrap).

## Discipline

Every family lands in the scheme's homes (`Data.`/`Text.`/`Effect.`/
`System.` for the platform layer) with law-shaped tests and the
established capability rules (capabilities lexical, constraints entail
nothing, refusals named). The bootstrap milestones are the program's
definition of done for this roadmap: each milestone is a runnable
Cinch artifact, not a claim.

## Sequencing

Library slices fan out as parallel work once the in-flight compiler
slice releases the prelude ownership; JSON (P31) and parsing (P30)
start first. P29 (incremental recompilation) remains scheduled after
this program unless reordered by the user.
