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

## For language models (2026-11-02)

The user wants the page to argue that Cinch is the best language for LLMs to write. The section after the average example (`#agents`, orange field) holds:

- `AgentLoop.astro`: an agent session on a teletype, three turns stepped with a keycap. Turn 1 checks `average-unchecked.cinch` (rejected), turn 2 checks `average-match.cinch` (remedy 2, accepted), turn 3 runs `average-report.cinch`. All commands and output come from `recordings.json`.
- Six reasons, each linked to the chapter that backs it: errors that say what to do next; start loose and tighten (gradual refinement); nothing to fake (kernel re-check, no switch that silences a check, per `design/no-faults.md` section 7); the signature is the context (effects, borrows, and requirements in types; `.cinchif` interfaces); effects you can review; fast without a rewrite (ownership, LLVM, specialization, checked SIMD).

Do not cite token counts, benchmark timings, or comparisons with named languages until they are measured.

## Annotation review (2026-11-03)

The user reviewed the landing page with Agentation and removed more labels:

- Sections have no mono running head (`.section-index`). The heading names the section. The user marked seven; the other three were removed too so the page stays consistent. The language-jump links under the introduction still give section names.
- The console shows only the maker's plate in its bottom strip. The instruction line ("Take every order away…") was dropped. On narrow screens the strip is hidden, since the plate already was.
- The closing call to action is the heading and arrow alone, with no "Tutorial 1 /" label and no note under it.
- The documentation intro says where the tutorial goes (chapter 1 runs a program, chapter 22 builds the order calculator) and that the compiler guide marks current and coming support. "Start with a working program. Keep the compiler guide nearby." was rejected as not useful.
- Runtime-contract numbers are segmented gauges. Row n of N lights n of N orange segments, so the gauge represents position in the list. The list has no rule under its last row.
- The proof selector is a row of tabs on one baseline. There is no top rule, and the divider beside the active tab is hidden so it does not cut the tab's curved shoulder.
- The console's "Interactive example" label was replaced by the wordmark embossed into the housing: the same orange as the case, lit from above, keyline as grooves. The section keeps "Interactive example" as its accessible name.
- The agent-session caption states what the reader cannot see from the turns: the output is recorded from the real compiler, and the rejection names the missing fact. The fact (`NotNull orders`) is read from the recording at build time, so the caption follows the example.
- The incremental-builds dependency diagram was replaced by a live-editable Game of Life, at the user's request. The board runs `step` from `src/data/examples/life.cinch` (bounded; cells past the edge are dead), which `record-examples.py` checks. The reader edits the `rule` declaration. When they stop typing, `src/lib/life.ts` reads the edit and swaps it into the running board, keeping the cells and the generation count. The reload is labelled as simulated until the browser build of the compiler exists.
- The reader accepts only Cinch the Rust compiler also accepts: a `case` on the cell with `Alive`, `Dead` or `_` arms, and `if` chains over comparisons of the count with literals, joined by `&&`, `||` and `not (...)`. A missing arm is reported as the compiler reports it. A changed signature or name is held as a contract change that would need a state migration. Anything else is "cannot read", never a guess. Widen the reader only alongside a `cinch check` of the new form.
- The board runs only after the reader presses Run, and only while it is on screen and the tab is visible. Step and Reset are keycaps, and clicking a cell switches it.
- The "Incremental builds" and "Hot reload" mono labels were dropped. The user marked the first; the second matched it.
- The average section's heading is "Crashing code never ships." The user rejected "The empty-list crash never ships." and suggested "Wrong code never ships". "Crashing" keeps the claim to what the compiler rules out: the hero promises no crashes, not correct business logic.
- The specialization heading is "Same type. Cheaper layout." The user rejected "Store a list of booleans as bits." as a headline: the BitList example is one instance of the feature, which is that a type can take a cheaper isomorphic representation. The lede now states that feature and keeps the packed list as the example. The "Changing the representation changes the cost" footnote was dropped as too far into the weeds.
- The documentation intro is "One idea per chapter, each with an exercise and an answer. It assumes you can program, not that you know a dependently typed language." It answers whether the tutorial is for the reader, instead of listing chapters.
- Every small mono label in the tour grew: .64rem for section and figure labels (.55 to .57 before), .62rem for staging and target labels (.51 to .54), .6rem for gadget controls (.52), .58rem for in-diagram rows (.48 to .5). Code captions went from .54 to .62rem (.48 to .56 on mobile).
- The staging heading is "Codegen with confidence.", as the user wrote it, replacing "Generate code that gets checked." The lede still says the output is checked like hand-written code.
- The representation figure's caption was dropped. Its figure keeps an `aria-label` naming the two forms and the press-a-bit interaction, so the bits stay announced without prose under the diagram.
- The agent session's paper now escapes its housing: negative inline margins put it 6px past each side of the olive case, and it hangs 16px below it with a longer shadow. It reads as continuous feed out of the slot rather than a panel inside a box. The mobile case overhangs the same way at its smaller padding.
