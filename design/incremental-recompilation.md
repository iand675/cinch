# Roadmap: Blaze-style uber-fast recompilation (P29)

Roadmap item, 2026-10-27 (user direction: "our compiler supports a
very similar thing to indexable-inc/blaze in terms of facilitating
uber-fast recompilation"). Not scheduled yet; this note is the seed.
No em dashes.

Blaze's thesis (from the announcement): REAPER builds a whole-program
dependency graph (edges labeled by effect and control-flow
direction), then leaves incremental rebuilds to specialized tools;
INCR recompiles changed functions in a persistent session and
type-checks only what changed, keeping dependency edges per item (type
and effect changes spread by reachability; body-only changes do not);
LINK ties object caching to declaration-level hashes so link-time
changes cost far less than relinking. Goals: no build cache as single
point of failure, and unchanged recompilation latency as codebases
grow.

## Why Cinch is unusually well positioned

- Artifacts are already content-addressed: every unit's identity is a
  content hash, `iface_hash` is a contract-only hash (D9), and a
  dependency is an interface (D3), so a body-only edit already should
  not stale dependents through the hash chain. `.cinchif` is
  declaration-level by construction (LINK's model needs no rework).
- The `.cinchif` dependency chain is already the incremental unit:
  the LSP reports `rechecked`/`kept`/`chain_ok` over exactly it
  (D220).
- Durable code identity exists (D126/D127 unit identity, A5's
  `unit_id`): function-level identity for a persistent session is the
  same machinery scaled down.
- The compiler is deterministic with build identity and secrets
  discipline (D45) and a machine-readable trust/assumption manifest:
  cache failure degrades to a full deterministic rebuild.

## Shape when it lands

1. **REAPER-equivalent**: on-demand construction of a whole-program
   item graph (units, classes, instances, refinements, contracts) with
   edges labeled by kind (type/contract vs body) and direction. The
   `.cinchif` chain plus the P20/P28 contract facts are the labels;
   P20's assertions and law identities are node attributes.
2. **INCR-equivalent**: a persistent compiler session that recompiles
   changed functions only and type-checks the reachability closure of
   changed TYPE/contract nodes (body-only changes skip dependents).
   The REPL/analysis layer (D215-D220) is the session shell; the
   D92 evaluator and the probe path show evaluation of single items
   already works.
3. **LINK-equivalent**: object caching keyed on declaration-level
   hashes (already the identity model) with a link step that only
   re-places changed units; the codegen units (the P18 per-family
   units are the precedent) are the caching grain.

## Non-goals and cautions

- No single build cache as a point of failure: the manifest and
  content hashes must make the cache advisory (deleting it costs
  time, never correctness).
- Latency budget is the acceptance: unchanged rebuild cost must not
  grow with program size except for graph traversal; measure, do not
  assume (the honest-boundary discipline applies to latency claims
  too).
- The prelude should stay embedded and hash-stable (D161-D174) so the
  session warm path does not recompile it.
