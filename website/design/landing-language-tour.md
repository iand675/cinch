# Landing language tour: content sources

The expanded landing page follows the intended language design. The user explicitly asked to feature unfinished functionality as well as implemented features, rather than limit this private iteration to today's compiler. Keep one development note explaining that scope; avoid inserting implementation caveats into every section. This is not a declaration of public release readiness.

The hero and nonempty-vector example are followed by `LanguageTour.astro`, then the broader runtime feature list in `src/pages/index.astro`. `CodeExcerpt.astro` renders selectable, escaped text with lightweight display highlighting; it does not parse, type-check, or execute Cinch. The staging expansion is an explanatory illustration.

| Landing material | Primary repository source | Editorial constraints |
| --- | --- | --- |
| Runtime bounds evidence and `plusZero` induction | `tutorials/02-proofs-and-runtime-checks.md` | Concrete decision evidence differs from a universal proof; erasure does not remove the executable comparison |
| `NonEmpty` refinement | `design/refinement-types.md` | Keep the container representation; method availability depends on preservation; distinguish refinements from semantic newtypes |
| Tactics, certificate checking, proof kernel | `design/proof-automation.md` | Search is untrusted; accepted evidence is checked; unresolved work is not a proof |
| `List Bool` / `BitList` and specialization | `design/layouts-and-specialization.md`, sections 4–7 | The instance snippet is an excerpt, omitting view/law definitions; round-trip laws remain required; semantics do not imply equal asymptotic cost; growing recursive layouts need the boxing rule |
| Typed `addOne` and generated declaration | `tutorials/08-staging-and-schema-generation.md`; concrete `Int` example in `compiler/tests/staging.rs`, `quote_with_typed_splice_evaluates_at_runtime` | Hygienic binding; output still checked; the displayed expansion preserves the generated `let` rather than claiming a specific optimizer output |
| Pure `mapMap` rewrite | `tutorials/07-rewrites-and-stream-fusion.md`, `examples/source/03-map-fusion.cinch` | Pure finite lists and total callbacks; no effect-order change; do not extend type-checker conversion with optimizer rules |
| Four-lane SIMD with ten elements | `tutorials/10-platform-specific-simd.md` | Two full loads plus two scalar elements; avoid out-of-bounds loads; distinguish wrapping arithmetic from floating point |
| Ownership/effects | `tutorials/03-ownership-and-effects.md` | Borrow lifetimes also constrain state-dependent facts; higher-order effects are explicit |
| Structured concurrency | `tutorials/13-structured-concurrency.md`, `examples/source/11-structured-tasks.cinch` | Scoped tasks, bounded channels, cancellation/join/cleanup remain distinct |
| Temporal contracts | `tutorials/06-processes-and-temporal-contracts.md` | Keep scheduler/environment assumptions and distinguish safety from progress |
| Hot upgrades | `tutorials/09-compatible-hot-upgrades.md` | State relation, migration, resource transfer, specialized dependencies, old-code lifetime |
| Incremental rebuild graph | `design/incremental-recompilation.md`, `specifications/runtime-evolution-specification.md` section 8 | Body and contract hashes have different invalidation roles; specialized imports keep implementation dependencies; no invented latency measurements |
| Live activation sequence | `specifications/runtime-evolution-specification.md` sections 4–8, `tutorials/09-compatible-hot-upgrades.md` | Prepare the complete update set; reach a valid boundary; transfer ownership/publish; retain old code for live references. A changed contract still requires compatibility/adapters |
| Streams | `tutorials/05-streams-and-coinduction.md` | Productivity and unbounded search have different guarantees |
| Foreign code and Wasm | `tutorials/11-foreign-libraries.md`, `tutorials/12-webassembly-components.md` | ABI, ownership, callbacks, effects, handles, and failure boundaries remain explicit |

## Visual choices

The new sections extend the existing full-width field composition rather than introduce a card grid. The proof selector uses a printed rail and one changing example. The warm orange specialization surface carries an original build/view diagram and packed-bit strip. The staging area pairs a generator and expansion inside an olive field. The optimization section combines a code excerpt with a plain transformation and a numbered SIMD/tail diagram. Shapes, grain, type, and colours follow the handbook and archived plastic/manual references.

Proof choices are native buttons with pressed state and associated panels. `LiveEvolution.astro` gives incremental rebuilding and stateful hot reload their own section, with a native body/contract edit selector and an explanatory dependency graph. These are local illustrations, not a live compiler session. Long code scrolls within its own frame. The new diagrams use meaningful labels and captions; their geometry represents the example rather than fictional telemetry. Section links near the introduction help navigation through the longer page.

When editing these examples, revisit the source design rather than invent syntax or silently turn a design intention into an implementation claim. Intentional future-facing language is appropriate to the current brief; invented results, benchmark numbers, and unsupported safety guarantees are not.
