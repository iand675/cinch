# Landing language tour: content sources

The expanded landing page follows the intended language design. The user explicitly asked to feature unfinished functionality as well as implemented features, rather than limit this private iteration to today's compiler. The site is not public yet. On 2026-11-01 the user asked for the landing copy to read as if Cinch were at a full v1: present tense, no development note, no implementation caveats. This is a copy decision for an unpublished site, not a declaration of release readiness; revisit it before the site goes public.

The hero console and the average-order-value section are followed by `LanguageTour.astro`, then the broader runtime feature list in `src/pages/index.astro`. `CodeExcerpt.astro` renders selectable, escaped text with lightweight display highlighting; it does not parse, type-check, or execute Cinch. The staging expansion is an explanatory illustration.

| Landing material | Primary repository source | Editorial constraints |
| --- | --- | --- |
| Hero console, "The empty‑list crash never ships", Figure 01 | `src/data/examples/average-report.cinch` (accepted; its run prints `24` then `no orders yet`) and `average-unchecked.cinch` (rejected at `12:27`: the obligation `NotNull orders` needs evidence); `learn/13-refinements-and-safe-operations.md`; `(/) :: a -> NonZero a -> a` in `Algebra.Arithmetic` | Check with `../compiler/target/release/cinch check src/data/examples/<file>`. A plain-list `average` is rejected for `NonZero length orders`. Today a branch on `length xs > 0` does not discharge `NonZero (length xs)`, so the page uses `NonEmpty` and `non-empty` rather than an `if` |
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

## Labels and copy (2026-11-01 pass)

The user asked for less filler on the landing page. Section running heads name the feature in one mono label; the numbered `NN /` prefixes and the right-hand label triplets were removed because nothing referred to them. The repeated "types / effects / ownership" noun list, the hero tick track, the console's `C / 01` mark, and per-example "design example" tags are gone. The closing status aside and the hero development line were both removed under the v1 framing above. The primary hero action is "Get Cinch" (downloads); the first tutorial is the secondary action and the closing call to action. Packages and downloads stay reachable from the header and footer; the separate landing section for them was removed. Hot upgrades appear once, in the live-development section, rather than again in the runtime-contracts list.

Write section copy as what the feature does for the reader, in plain sentences. Add a mono label only when it names something the heading does not.

## Gadgets (2026-11-01)

Each gadget changes a quantity its section is about and animates only when the reader acts. Controls are console keycaps, not tabs.

| Gadget | Section | What it changes |
| --- | --- | --- |
| Orders dial | Hero console | How many orders `average` receives; at zero, why the call does not compile |
| Bit toggles | Type specialization | One `List Bool` shown as cons cells and as packed bits |
| Spliced value | Typed staging | The literal in the generator, the generated declaration, and its value |
| Apply mapMap | Verified optimization | Two traversals and an intermediate list, or one traversal |
| Element count and run | SIMD | How elements split into four-lane loads and a scalar tail |
| Body / contract | Incremental builds | Which dependents rebuild; affected nodes pulse once |
| Run an upgrade | Hot reload | Steps through prepare, coordinate, activate, retire |

## Copy-editing pass (2026-11-02)

The user asked for a pass using the copy-editing skill's seven sweeps (clarity, voice, so what, prove it, specificity, emotion, zero risk) and its AI-tell check, with the page written for Cinch 1.0. Rules that came out of it:

- Each section says what the reader gets in their own work: validate once at the edge, generate code from a schema and have a generator bug fail the build, ship a fix without a restart, keep existing C and Rust libraries.
- Examples come from ordinary services (average order value, a payment captured twice, an order shipped or refunded, log lines), not from type theory.
- Each call to action answers the likely hesitation next to it: platforms and no Haskell required in the hero; chapter 1 from an empty directory and proofs waiting until chapter 14 at the close.
- Avoid repeated two-fragment "X. Y." headlines, "keeps A, not B" contrasts, and insider terms (dispatch epoch, codata, version-coexistence protocol) in body copy.
- Do not invent proof. Adoption, testimonials, benchmark numbers, and licence terms stay off the page until real ones exist.

## Runnables (2026-11-02)

The browser build of the compiler is coming. Until it lands, the `average-report.cinch` window is a placeholder runnable: its Run key plays back output recorded from the real compiler by `scripts/record-examples.py`, line by line. When the browser build registers `window.cinch = { run(file, source) }`, the same key calls it with no page change. The next step is to make the code editable once the engine exists, and to give other code windows on the page the same treatment.

## Positioning against Lean (2026-11-02)

The user's goal is for Cinch to be a nicer language than Lean. Lean's site leads with mathematics (geometric series, Mathlib, Fermat, Terence Tao) and reaches its programmer pitch ("Write software. Prove it correct.") two thirds of the way down. Cinch's page leads with programs instead:

- The hero describes "a functional language with dependent types, for the programs you ship" and says most guarantees need no written proof.
- The examples are business code (average order value), not theorems.
- The compiler's messages are the counterpart to Lean's goal view. The `average.cinch` window feeds the real rejection message for `average-unchecked.cinch` onto continuous form paper (`CompilerPrintout.astro`), with numbered callouts on the three remedies the message names. Each remedy in the legend is backed by a checked example: `average-report.cinch` (decide it with `non-empty`), `average-match.cinch` (match on the list), and `average-report.cinch`'s `average` (ask for `NonEmpty Int`).
- Do not name Lean or other languages on the page, and do not claim comparative facts about them.

Today a branch on `length xs > 0` does not discharge `NotNull xs` or `NonZero (length xs)`; pattern matching on `Cons` does. Keep examples on the forms that check.

## Programs, not proofs; bulletproof code (2026-11-02)

The user wants Cinch positioned as a language for correct programs, not as a mathematics tool, and wants the copy to lean into the retrofuturist design: "better programming from an alternate universe", "bulletproof code", sci-fi and fun.

- Headline: **Bulletproof code.** It is also the language's design constraint (`design/no-faults.md`: if it compiles, it should be bulletproof).
- The alternate-universe idea lives in small printed parts of the hardware, not in body copy: the hero running head ("Software from the future we were promised") and the console's maker's plate ("Cinch · made to not fail"). Keep any new ones short, printed in mono, and attached to an object. Body copy stays plain and specific.
- Keep "proof" out of the hero, section labels, and tour headings. Say checked, guaranteed, or worked out by the compiler. The induction example (`plusZero`) was removed from the landing page; theorem-style examples belong in the tutorials. Proofs stay in the docs and the specification, where library authors meet them.
