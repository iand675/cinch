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

1. A choreography description that actually generates participant APIs, handles
   branching and executes a local multi-participant conversation. The current
   explicit-state protocol DSL is only a prerequisite.
2. Tensor/shape-aware differentiation and an elaboration interface that can use
   checked type/context information. Current integer polynomial forward AD is
   only the initial syntax/library probe.
3. Goal-aware proof automation through a checked extension interface. The
   current `by` grammar merely exposes `refl`/exact evidence.
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
- `verify.py` completed successfully: all 32 engine, actual native LSP,
  native positive/negative, and editor integration tests passed. This includes
  all eight editor tests, ten native positive programs covering eleven probes,
  and eleven intended compiler refusals. `verification.json` records the
  commands, output and current source hashes.
- The highlighting handoff needed per-library combined injections and a
  sequence-of-islands grammar entry; isolated grammar tests had missed it.

No PR, push, merge or installation into the user's editors is authorized or has
been performed. Local edits, local commits and continued experiments are
authorized by the task.
