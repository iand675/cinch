# Malleable Cinch: experimental acceptance criteria

This worktree is an isolated snapshot of the current compiler. Its snapshot commit
is a baseline, not part of the proposed syntax implementation.

Build an experimental library-authored syntax front end, and use actual Cinch
checking and native execution to test its output. Keep the prototype separate
from the production compiler until its grammar and source mapping contracts have
been exercised. Syntax declarations are data, not Python plug-ins.

## Shared mechanism

- Explicit source imports of syntax libraries; no ambient project grammar changes.
- Declarative recursive grammars with token patterns, captures, repetition,
  templates, generated fresh identifiers, constraints and embedded Cinch regions.
- Distinct lexical treatment of an island's raw text and its host code holes.
- Definition-site qualified helper references and capture-preserving splices.
- Bounded matching and expansion, useful failures, deterministic output.
- Origin maps for copied and generated code, consumed by an LSP adapter.
- Tree-sitter grammars and highlight/injection queries generated from the same
  declarations, verified on both complete and unfinished examples.

## Runnable design probes

1. TSX-style HTML: text, nested tags, attributes, typed components, embedded host
   expressions, callbacks, escaping and local scope.
2. Declarative binding/control syntax: evaluate once, fresh temporaries,
   call-site names and effect/usage checking.
3. Schema-backed queries: schema-specific typed records and projections.
4. Resource-safe task blocks: delayed work and ordinary capture checks.
5. Proof notation: emitted evidence checked by the existing kernel fragment.
6. Binary format: generated codec functions, malformed-input rejection.
7. Protocol choreography: generated endpoint states and legal transitions.
8. Differentiable numerical expressions: forward and derivative evaluation.
9. Reactive state-machine UI: generated states/actions plus HTML rendering.
10. Audio graph: executable processing graph and checked buffer/channel structure.
11. Circuit: cycle-indexed signal operations and executable clocked simulation.

Each probe needs a runnable positive example and meaningful negative evidence.
These are bounded proofs of concept, not claims to have completed the ambitious
runtime systems discussed in chat. Record the precise supported subset and what
each experiment teaches about the extension interface.

## Verification

Run the syntax engine tests, native positive/negative probes, generated grammar
checks, and actual framed LSP requests over original source. Record commands and
results. Existing staging/LSP checks should continue to pass in this worktree.
