# Continuation checkpoint

The user wants an isolated experiment that turns the malleable-language ideas
and ambitious examples into real proofs of concept, with particular emphasis on
library-defined TSX-style HTML/interleaved Cinch and coherent editor tooling.
The active goal must retain that scope.

## Completed first slices

- New worktree: `/Users/ian/Documents/ChatGPT/Slang/cinch-malleable-syntax`, branch
  `codex/malleable-syntax-poc`, based on main `c870c34` plus isolated current-source
  snapshot commit `fb198a8`. Never copy these changes back over the busy original
  checkout without reconciling its current state.
- Generic declarative syntax engine, twelve syntax-library descriptors, seven
  ordinary runtime libraries, ten positive native programs covering eleven
  probes (the `once` probe is exercised inside HTML), eleven native rejection
  fixtures, source maps and expansion traces.
- Generated library and project Tree-sitter grammars; original-source parsing,
  unfinished-input structure, lexical-mode boundaries, and actual whole-file
  Cinch → HTML → Cinch highlighting are exercised.
- Adapter to the real native LSP: mapped diagnostics, top-level symbol hover and
  definitions inside holes, UTF-16 semantic tokens, syntax errors and recovery.
- Compiler fixes and focused regressions: lost multiplicity through direct
  strict aliases; missing LSP notification method; pre-hoist callback folding
  panic. Corrected an invalid existing Zed highlight query.

## Remaining original scope

The small executable slices in README's table are evidence for the shared
mechanism. They must not replace the ambitious examples as the goal's end state.
Continue particularly with:

1. Choreography projection now generates participant APIs, explicit branch
   notifications/offers, private states/wires and local one-shot transport;
   the native example exercises approved and declined conversations. Further
   work includes nested choices, payload schemas and runtime session identity.
2. Tensor/shape-aware reverse-mode differentiation now runs through ordinary
   typed Cinch libraries, including matrix multiplication, transpose, sum,
   shape refusals, rectangular/empty axes and independent finite differences.
   The general checked elaboration/context extension interface remains open.
3. Goal-aware proof automation now has a native experimental hook and `by`
   notation for assumption lookup, checked arithmetic search and ordered
   fallback. See TACTICS.md. The general reflected goal/context interface for
   arbitrary library-written strategies remains open.
4. Resource-aware audio blocks/scheduling and bounded callback behavior. The
   current single-frame pipeline verifies channel composition only.
5. Live reactive UI/event behavior and syntax-aware completion in unfinished
   markup, plus general local-scope navigation. The current renderer is SSR.
6. General binary field composition/borrowed views/round-trip evidence and
   schema-snapshot-backed query generation. Current codecs and schemas are
   intentionally small but genuinely executable.
7. A scoped syntax-object/hygiene bridge and native stage/interface identity.
   Do not market the current fresh-name convention as complete automatic hygiene.

Keep the original syntax and proof examples readable. Every added abstraction
should have native observable behavior and a meaningful refusal case; editor
claims need whole-source tests rather than isolated grammar checks.

## Verification already completed

- `cargo test --test ownership --test lsp --test staging --quiet`: 21 ownership,
  8 LSP, and 23 staging tests passed (logs were under `/private/tmp/`).
- `cargo test --test surface --test string_literals --quiet`: 17 surface and
  11 string tests passed, including the new callback regression.
- The initial checkpoint's `verify.py` completed successfully: all 32 engine, actual native LSP,
  native positive/negative, and editor integration tests passed. This includes
  all eight editor tests, ten native positive programs covering eleven probes,
  and eleven intended compiler refusals. `verification.json` records the
  commands, output and current source hashes.
- The highlighting handoff needed per-library combined injections and a
  sequence-of-islands grammar entry; isolated grammar tests had missed it.

## Second-pass checkpoint

- `SECOND_PASS.md` describes the new executable slices and their boundaries.
- Generic finite template projection operations added to `engine.py`.
- `Checkout.cinch` is a syntax-authored source library. Its descriptor generates
  exports as well as implementation, keeping endpoint/wire constructors private.
- `Tensor.cinch` uses a private checked matrix representation with dependent
  public shape indices. The initial attempt to use general `Vec` pattern matching
  hit the isolated compiler's lack of rigid-index refinement. Do not claim that
  the list representation's preservation laws are kernel-proven.
- `TensorAD.cinch` implements typed reverse-mode pullbacks; no gradient evaluator
  or protocol code was added to the generic host engine.
- The CLI/LSP now share a dependency-ordered source-library expansion path.
  Only explicitly imported libraries are expanded. Generated-library API
  navigation and opening the original library document were checked with the
  actual native language server.
- Engine and editor focused checks passed before the final aggregate run.
- The final aggregate `verify.py` run passed all 38 tests, covering twelve native
  programs, eighteen intended refusals and fourteen syntax descriptors. All
  recorded source hashes were rechecked against the current files.
- The LSP tests now exercise library-first and consumer-first opening, closing
  and dependency reloading, and verify proper native didOpen/didChange messages.
- Next priorities remain goal-aware proof/context extensions, audio processing
  budgets and live reactive UI. Broader syntax-object hygiene and native
  interface/replay identity are still unresolved; this checkpoint does not
  complete the original goal.

## Goal-aware tactic checkpoint (2026-11-02)

- `Proof.search` runs a validated finite plan at native proposition-checking
  time, using the actual expected goal and native scoped evidence snapshot.
  `by` exposes assumption lookup, arithmetic certificate search and ordered
  fallback; arbitrary library-written strategies still need a reflection API.
- Native assumption matching preserves free variable identity. Both strategies
  record evidence dependencies for the existing mutable-state invalidation
  checker. Missing facts, rebound indices, stale evidence, malformed plans,
  phase escapes and changed arithmetic certificates are covered by regressions.
- The native regression run passed 43 tests: dependent (14), evidence
  invalidation (12), goal tactics (9) and linear facts (8).
- The final aggregate run passed all 39 tests, covering thirteen native
  programs, twenty-one intended refusals and fourteen syntax descriptors.
  Recorded source hashes were checked against the final sources. The native
  LSP reports a failed tactic's actual goal at the original notation.
- A base Tree-sitter gap surfaced: inline semicolon-separated case alternatives
  accepted by the compiler fail editor parsing. The example uses supported
  multiline layout; broader grammar parity remains open. See TACTICS.md.
- The broad experiment remains unfinished: reflected tactic APIs, audio
  budgets, live reactive UI, syntax-object hygiene and native extension identity
  remain open.

No PR, push, merge or installation into the user's editors is authorized or has
been performed. Local edits, local commits and continued experiments are
authorized by the task.
