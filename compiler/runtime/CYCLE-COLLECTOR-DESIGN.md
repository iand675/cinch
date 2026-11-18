# Bounded local cycle prototype

Status: design and test-only prototype. No allocation-time collection, exit-time collection, ABI change, header/layout change, count inflation, callback expiry, or disposal requirement is introduced. The frozen22-block repro is under `repros/buffered-transfer-cycle`.

## Safepoint and lifetime assumptions

The first entry is crate-private and test-only. Its unsafe caller must establish a quiescent local graph: no pointer-reversal release/mark walk is active, no parallel initialization can write candidate arrays, and every live borrowed value remains backed by an owned reference. Tests explicitly own their live aliases. Ordinary reference counts remain unchanged throughout discovery and classification, so uniqueness checks see exactly the original counts.

Positive counts alone do not prove that a fresh array has no concurrent writers: `par.rs` fills fresh output arrays from multiple participants before publication. Production integration requires a proved join/quiescence boundary, not an assumption that all positive arrays are immutable. Existing pool queue counters alone do not prove that running jobs have joined. The current `cinch_exit_check` remains unchanged; enabling the leak checker does not trigger collection.

## Precise registration inventory

| Site | Proposed test-only hook | Lifetime rule |
| --- | --- | --- |
| `arrays::array` | Register after zeroed allocation, format and length initialization, only formats1/2 | Sole `KIND_ARRAY` allocation site; slot storage, ordinary builders and parallel output constructors funnel here |
| `block::free_in` | Remove before either poisoned or ordinary deallocation | Covers `block::free`, ARC disposal and direct ABI box release; no stale address may survive reuse |
| `rc::claim` | Remove an array before changing its positive count to negative | Both `cinch_mark_mt` and `mark_mt_array` use this path before sharing |
| `par::tabulate_ctx` / `map_ctx` | No collection permitted during their output initialization | Production publication/quiescence handling remains a review gate |
| `cinch_exit_check` | No hook in the first prototype | Worker joins and borrowed-root absence must first be established |

The registry is non-owning and thread-local. It does not retain blocks, inflate counts, or require compiler instrumentation of inlined slot writes. Its hooks compile away outside tests initially. Per-test registration is disabled unless a focused test installs its guard, leaving other runtime tests' allocations unchanged. Retirement occurs on the owning thread before publication; a later atomic free must not access the old owner's registry.

### Complete runtime callsite review at the baseline

The source baseline is `9bc5237aede73322b100b5ac2d58c4f4d1a90f20`; the evidence-only child commit is `60c4119c6b06f55602cbb38f8ea673775a65f1bb`. Searches cover every Rust source under `compiler/runtime/src`, rather than only the proposed hook files.

- `arrays.rs::array` contains the sole `block::allocate(... KIND_ARRAY ...)` call. `slots.rs` uses these arrays; its inline generated slot writes provide no runtime callback. Registration therefore belongs after format/length initialization, not at individual writes.
- `block.rs::free` funnels into `free_in`. `abi.rs::cinch_free` calls `block::free`; `rc.rs::dispose` calls `block::free_in` after dropping external Rust-owned payloads. The prototype's detached disposal reuses that same path. Retirement precedes both debug poisoning/accounting and ordinary small/system deallocation. Arrays never use `arena.rs` wholesale chunk reclamation, which serves only constructor, closure and dictionary kinds.
- `rc.rs::claim` is the only positive-to-negative publication write. Generic `cinch_mark_mt`, its pointer-reversal descendants, and specialized `mark_mt_array` all call it. Runtime callers are `par.rs::RangeCtx::mark`, `par.rs::cinch_par_both`, `task.rs::Scope::spawn`, and `stm.rs::cinch_stm_publish`. Other search matches are tests. `rc.rs::take_atomic` restores a last atomic count to fresh immediately before release; it must not reregister that dying array.
- `par.rs::tabulate_ctx` and `map_ctx` allocate fresh positive output arrays. `RangeCtx::invoke` writes disjoint output slots from workers. `RangeCtx::mark` marks the function and input, **not the output**. `run_range` joins every published range before returning, but no candidate scan may overlap those writes. `cinch_par_both` similarly publishes a closure and joins its result. Pool queue emptiness alone is insufficient to establish this boundary.
- `task.rs::Scope::finish` publishes a returned result **without marking its positive graph**. `hand_over`, `join_succeeded`, `join_failed`, `Scope::await_children` and `discard_results` transfer or dispose that graph on another context after the synchronization boundary. A production registry must retire or migrate all associated candidates before this handoff; the mark hook does not cover it. A stale non-owning entry on the producer thread would otherwise survive a consumer-side free.
- The cooperative scheduler explicitly permits a parked context to resume on a different worker (`task.rs::Scope::wait_for_child`). Positive references carried on its stack can therefore migrate without either a mark or a result handoff. A production design must account for context migration as well as published results; a bare OS-thread-local registry is not sufficient. This prototype has only explicit test guards and no migrating tracked managed graph.

These publication/migration findings are unresolved production gates, not promises that an exit hook is safe. The current module is absent from non-test builds, and no candidate registration is enabled outside an isolated guard.

## Trial graph

A bounded breadth-first walk follows `block::Scan::counted_below`, counting every pointer edge precisely and retaining duplicate edges. It visits only positive managed heap nodes. It explicitly excludes `RC_FRAME` and `RC_FRAME_RELEASED`, which are positive marker values, and never reads atomic payloads. Nulls and tagged immediates do not contribute references. Atomic/immortal boundaries are held by ordinary counted edges; an encountered frame or unsupported condition aborts the component unchanged.

Actual counts are copied to metadata. Internal incoming references are subtracted only in shadow counters. A node with an external owner is live, as is everything it reaches. A budget limit, inconsistent incoming count, or changed header aborts before any payload/count mutation. Iterative worklists bound stack use, and explicit node/edge/slot/root limits bound one test pass.

## Reclamation

For a fully analyzed component, clear every counted pointer edge of all proven unreachable nodes before releasing any outgoing edge or block. Internal edges are not individually dropped into objects being reclaimed. Outgoing edges to live or excluded boundary objects are released through the ordinary ARC entry. The existing payload destructor and block-free path dispose each detached block exactly once, preserving byte/bignum resources, debug poisoning and allocation accounting. The non-owning registry loses freed arrays before address reuse.

## Focused evidence required

- Unreachable two-array and closure-mediated cycles reclaim.
- External aliases preserve their complete reachable graph, payloads and original counts.
- Duplicate internal edges and shared live tails account exactly once per edge.
- A uniquely owned reachable array stays at count1 before and after the pass.
- Atomic publication retires candidates before sharing; collector never inspects an atomic mutable payload.
- Budget abort preserves every header and slot; a later sufficient pass can reclaim.
- Registry removals cover free, poison and address reuse.
- Large cycles use bounded native stack.
- Only after these pass: explicitly review production quiescence hooks, then relink the unchanged frozen repro against a distinct runtime epoch and require actual zero live/leaked blocks.
