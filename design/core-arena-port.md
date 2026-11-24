# The dense Core arena: port status and the sharing blocker

Date: 2026-11-12. Former workspace: `dasl-arena-port`. Base: main `981107a5`.

This note records a parked lane. The port itself works and is measured; one
defect in the sharing pass keeps `emit-ir` from completing, and the lane is
stopped there rather than landed half-done.

## What the arena is

`Core` is a `u32` id into `CoreArena`, which is a `Vec<CoreNode>` plus a span
side table. A node is immutable: a pass that changes a node's shape pushes a
new one, and a pass that does not keeps the id. Two things follow, and they
are the whole point:

- a shared subterm is one node rather than a copy per occurrence, so a walk
  can key its memo on a `Core` instead of on a structural digest;
- a graph can hold a node reachable from itself, which the boxed tree could
  not express.

The second property is what exposed the defect below.

## The two commits

Both are in the local `archive/cinch-core-arena` bookmark, based on main
`981107a5`.

### `1931a2f0` Hold the seed's Core in a dense arena of ids

The port. `cargo test --lib` is 257 of 257 and every test target compiles.
Compaction is off in this commit.

Files: `core.rs`, `codegen.rs`, `rc.rs`, `simplify.rs`, `check_decls.rs`,
`coretext.rs`, `dictionary_values.rs`, `driver.rs`, `incremental.rs`,
`specialize.rs`, `stack.rs`, `ifaceopt.rs`, plus the seven test files that
build Core by hand.

Three pieces are worth knowing about:

- **`rc.rs`'s reuse subsystem** (`CtorFact`, `Ctors`, `insert_reuse`,
  `reuse_allocation`, `compatible_ctor`, `reuse_allocation_anywhere`,
  `reuse_scrutinees`) is ported to `&CoreArena` and `Core` ids. `insert_reuse`
  pushes the rebuilt spine nodes rather than returning new boxed nodes.
- **`ifaceopt.rs`** was written on main against boxed `Core` and is not in the
  lane's merge. `BodyExport` now carries the `CoreArena` its body ids index,
  because an artifact body leaves one compilation as ids and enters the next
  as ids. `ForeignBodies { defs, arena, modules }` is the one arena a
  consumer's inliner reads, and `simplify_with_foreign` copies each body into
  the program's arena on entry so every walk downstream sees one node store.
- **`rc::Pass::fv`** is a least fixed point, not a memo filled after the
  recursion. The value is a set union, so a placeholder cannot break a cycle:
  `fv_seed` opens the reachable set seeded with each node's own direct read,
  then `fv_base`/`fv_bind` re-derive every opened node until nothing moves,
  children first.

### `68acd170` Number the binders a join point and a release add to the sharing pass

`share_rename` and the `ShareLocals` collector knew about `let` binders and
pattern binders only. A join point binds its parameters for its body, and a
`Release` binds its token plus the fields it moves or rest-drops, so a unit
containing either reached the renamer with a local it had never numbered and
the pass panicked by design. This commit adds both. It is a real bug fix in
the sharing pass and is worth having whatever the arena does.

## Measurements

Interleaved pairs, release builds of both arms built from this tree
(`CARGO_TARGET_DIR=/tmp/arm-main` and `/tmp/arm-port`), 6 build slots, 3
rounds, bundle of 47 modules (41 bootstrap, the five toolkit modules, the
entry), compaction off.

|arm|mode|wall s|CPU s|peak RSS MB|
|---|---|---|---|---|
|main|check|6.43, 6.01, 6.16|5.87, 5.96, 6.10|1581, 1586, 1585|
|port|check|6.34, 6.30, 6.01|5.79, 6.17, 5.96|2779, 2748, 2746|

`check` is at parity. Peak RSS is **1.74x main** with compaction off. That is
the regression compaction exists to remove, and it is why the ordering in the
brief is port first, compaction second.

`emit-ir` was not measured: main takes 358 to 393 s on this bundle, and the
port arm exits 101 with the panic below.

## The blocker

`emit-ir` on the bundle panics inside `share_repeated_subterms`, with
compaction off:

```
share_rename: local LocalId(4) in a var node has no number (map 12 entries)
```

It reproduces with the full 47-module set and not with the first 45; adding
`Bootstrap.Test` is what triggers it. `check` does not reach the sharing pass,
which is why the port looked green.

### What the divergence method found

Bisect: `emit-ir` over growing prefixes of the dependency order. Prefixes of
45 pass; the prefix that adds `Bootstrap.Test` panics.

Instrument `ShareLocals::of` and `share_rename` to print the root id, the
preorder step count, the numbered `(local, is_binder)` order, and the preorder's
direct reads. Six `share_rename` calls run before the panic:

|root|steps|free locals include 4|map size|
|---|---|---|---|
|5175890|84|yes|10|
|5175995|84|yes|10|
|5176100|50|no|12|
|5176166|50|no|12|
|5176230|193|no|12|

The panic follows root `5176230`. Its preorder is 193 steps and its direct
reads are `{2, 3, 28868, 9, 10, 11, 12, 28886, 28887, 13, 14, 61853}`, with no
`4`. The renamer walks exactly that preorder. So the `Var(4)` it panics on is
not a node of that root as the collector saw it.

### The diagnosis

`share_rename` renames binders **in place** on the subtree it is handed, and
the arena is shared between roots. Roots `5175890` and `5175995` are two copies
of one body and both contain `(4, false)`, so local 4 is a free local there.
The bodies overlap: a node reachable from `5176230` was already renamed by an
earlier `share_rename` in the same pass, so the later walk reads a local number
the earlier pass wrote, and that number is not in the later map.

In the boxed representation each candidate was a distinct copy, so two roots
could not share a node and this could not happen. Making sharing exact turned
a latent aliasing assumption into a reachable bug.

### What was tried and did not fix it

- Adding `Release` (`moved`, `rest`, `token`) and `Join` (`params`) binders to
  both walks. Real, kept, not sufficient.
- Replacing the two walks with **one** enumeration: `share_preorder` returns a
  `Vec<ShareStep>` in preorder, each step carrying the ids it binds and the ids
  it reads, and both `ShareLocals::of` and `share_rename` consume it. This is
  the right shape and it removes the possibility of the two enumerating
  differently, but the panic is aliasing between roots, not disagreement
  between walks, so it did not close the bug. The change is reverted rather
  than landed, because it does not fix anything on its own and a rewrite of a
  pass this size should not land without its test.

## What the next session should run

1. **Fix the aliasing.** `share_rename` must not rename a node another root
   also reaches. Two shapes work: give each shared candidate its own
   `clone_subtree` before renaming (correct, costs a copy per candidate), or
   make the rename a pure rebuild that pushes renamed nodes and installs the
   new root, leaving every original node untouched (better, and the arena is
   built for it: `CoreArena::rebuild` and `clone_subtree` both already
   register before recursing). The second also fixes the same latent hazard
   in `share_match`, which reads classes by local id across overlapping
   bodies.
2. **Add the test that would have caught it.** A program with two
   byte-identical `case`-bearing bodies at two sites, so sharing picks the
   same representative twice and both roots overlap. Assert `emit-ir`
   completes and the shared unit's parameter list is the free set of the
   original, not of a renamed copy.
3. **Then** turn compaction on at the four sites (`rc.rs`, `simplify.rs`,
   `stack.rs`, `dictionary_values.rs`), and take the pairs again: `check` and
   `emit-ir`, both arms release, CPU and peak RSS, plus the two oracles
   (printed Core byte identical or `@ticks` only; emitted IR compared by
   definition count and call shapes, never bytes).
4. **Then** the gates: `--lib`, the reclamation family, the Stage-3 target set
   against `bootstrap/stage3-suite-baseline.json`, and the chain
   (`build --stage 3`, stage2/stage3 `compiler.ll` byte equal to each other,
   `cinch` byte equal, every `.cinchif` byte equal, `smoke --stage 3`,
   `cinch-leak: live=0 immortal=0 leaked=0`).

## Two main-side defects, not this lane's

Both fail identically on plain main `981107a5` in this tree, verified by
checking main out and re-running:

- `alloc_accounting::a_compiled_structure_drops_back_to_the_immortal_baseline`:
  the link fails with `Undefined symbols: _AllocationFixture.build`, so the
  function itself is not emitted, not merely its stub.
- `rc_specialisation::borrowed_parameters_release_and_static_dictionaries_place_fewer_operations`:
  panics with `no static dictionary` at `tests/rc_specialisation.rs:96`.

The first looks related to `4322a70d` "Emit an apply stub only for a unit
whose closure is taken", whose `closure_taken` gate may be reaching the
function definition rather than only the stub.
