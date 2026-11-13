# Compilation speed

Cinch's compiler does a lot per declaration: dependent indices, refinements,
proof obligations, instance search, ownership and reference counting, target
selection, rewrites, and specialisation. That buys safety and target
flexibility, and it costs compile time. This note collects what is worth
doing about it, in the order the evidence says to do it, and says which
parts of it are already in the tree.

The structure follows three layers:

1. What our own measurements say (`1`, `2`).
2. What other compilers do, with a decision for each: adopt, adapt or
   reject (`3` to `10`).
3. The benchmark corpus and the ordered backlog (`11`, `12`).

Every speed rule in this note is subordinate to the bootstrap's correctness
gates: the Stage2/Stage3 fixed point (executable, IR, interfaces) and, once
Core output lands in both compilers, agreement on `owned` Core.

## 1. Measured state (2026-11-10)

Self-host compute on this machine, one job, `-O2` links unless stated.

| Step | Time | Notes |
| --- | --- | --- |
| Rust seed builds Stage1 (`--opt-level 2`) | 110 s | 2 s of that is the seed's own cargo build of the runtime |
| Seed frontend: check the bootstrap bundle | 22 s | 35,000 lines, 55 modules |
| Clang, Stage1 IR at `-O0` | 8.5 s | 218 MB module, one translation unit |
| Clang, Stage1 IR at `-O2` | 55 s | 23 s IR optimization, 31 s machine code generation |
| Stage1 compiles Stage2 | 9 min | the largest single compile in the chain |
| Stage2 compiles Stage3 | 6.5 min | |
| Full Stage0 to Stage3 chain | 20 min at the start of the day; 335 s now (warm seed, PGO, sharded links) | measured 2026-11-11 on 18 cores |
| Cinch-built Stage3 checks the 4K-line toolkit | 49 s at the start of the day, 7.45 s at mid-afternoon, 6.13 s now | median of 3, 2026-11-11 |
| Cinch-built Stage3 checks its own 40K-line source | 585 s at the start of the day, 71 s at mid-afternoon, 56.6 s now | median of 2 |

Phase split of a Stage2 self-compile at `-O2` (from stack samples, one per
11 s, before and after the quadratic-pass removal):

| Phase | Before | After |
| --- | --- | --- |
| load inputs | 8 s | 6 s |
| parse | 68 s | under 11 s |
| resolve | 12 s | 23 s |
| totality preliminary | 34 s | 11 s |
| infer (`check-definition`) | 202 s | 67 s |
| Core checks | 134 s | 24 s |
| interface export | 34 s | 34 s |
| RuntimeIr emission | 78 s | 22 s |
| write IR and interfaces | 17 s | 14 s |
| total | 585 s | 195 s |

Runtime profile of the Cinch-built compiler checking the toolkit (32K
working samples, before the size-class allocator): 25% system allocator
plus the counting wrapper, 37% reference counting, 10% string comparison,
4% constructor allocation. After the allocator: allocation and free are
22% and 9%, reference counting 38%, and the rest is checker work spread
thin.

Clang's own time trace (`-ftime-trace`) on that single 218 MB module shows
the split precisely: at `-O2`, the LLVM frontend is about 0.7 s, the IR
optimizer 23 s, and the code-generation passes 31 s; at `-O0` the optimizer
is 1.2 s and code generation 6.6 s. One translation unit means one thread:
none of that work is parallel, and it is all in the backend. Section 8 and
section 12 draw the conclusion.

The lesson from the table above is worth stating plainly: the largest
single lever so far was not a micro-optimization. It was removing
asymptotically wrong code (quadratic trial unification, per-name linear
scans, quadratic identity encoding, one dictionary allocation per use).
Every item in this note is chosen by the same rule: find the asymptotic
problem first, then the constant factor.

## 2. What is already in the tree

| Mechanism | Where | Effect |
| --- | --- | --- |
| Undo-log trial unification (`snapshot`/`rollback_to`/`commit`) | `compiler/src/types.rs` | 1687 s to 146 s on a self-build |
| Chunked copy-on-write substitution store | `compiler/src/types.rs` | makes the remaining independent copies cheap |
| Size cache in the unfolding whistle | `compiler/src/core.rs` | optimizer cost from 10 s to under 1 s |
| Shared wildcard alternative in match compilation | `compiler/src/check_terms.rs` | removed one branch per constructor |
| Core-level subterm sharing (`<unit>.sharedN`) | `compiler/src/core.rs` | IR 1.3 GB to 221 MB |
| Reference-count helpers instead of inline sequences | `compiler/src/codegen.rs`, `bootstrap/Bootstrap.RuntimeIr.cinch` | IR and clang time |
| Static dictionaries for closed instances | both backends | 1M allocations per self-build removed |
| Drop specialisation (`Core::Release`) | `compiler/src/rc.rs` | 104K count calls removed |
| Borrowed parameters for closed programs | `compiler/src/rc.rs` | 2,672 parameters borrowed |
| Per-thread size-class heap | `compiler/runtime/src/small.rs` | 29% faster, 42% less peak RSS |
| Core simplifier with a tick budget, case-merge, known-constructor case, direct static-dictionary calls | `compiler/src/simplify.rs` | fires 597 direct-call and 17 known-constructor rewrites on the toolkit; IR down 2.2% |
| Core inliner: one site for a body up to 40 nodes, every site for a body up to 12 with no call and no allocation, `case` pattern binders admitted | `compiler/src/simplify.rs` | 133 sites / 497 nodes on the toolkit and 7,253 / 17,326 on the compiler source; it spends IR to delete allocations, because `known-constructor` can only fold a constructor case once the call hiding it is gone (1,000-iteration block loop: 506 blocks to 6) |
| Core text output, verifier, diff, server, grammar, editors | `compiler/src/coretext.rs`, `compiler/src/lsp.rs`, `tooling/tree-sitter-cinch-core`, `tooling/editors/*` | makes each stage's codegen input inspectable and comparable |
| `--opt-level` and `-O2` library objects and stage links | `compiler/src/driver.rs`, `tooling/bootstrap.py` | 28% on the toolkit check |
| Core simplifier: case-merge, known-constructor, direct dictionary calls | `compiler/src/simplify.rs` | removes `Option`/`Result` scaffolding and static-dictionary `__apply` calls; reports ticks (section 10) |
| Package store keyed by content | `compiler/src/store.rs` | reuses checked artifacts per compile |
| Analysis layer + LSP incremental update | `compiler/src/analysis.rs` | editor path, not build path |
| Work-stealing parallel runtime (`cinch_par_*`) | `compiler/runtime/src/par.rs` | available; the compiler is not yet parallel |
| Visible surface as an order list plus a name index | `bootstrap/Bootstrap.Resolve.cinch` | a renamed name is one hash probe, not a scan of the declarations, the qualified imports and the kernel bindings |
| Export entries indexed when an interface is read | `bootstrap/Bootstrap.Load.cinch` | `loaded-entry` is a probe, not a walk of the contract document |
| Environment projections cached at the module marker | `bootstrap/Bootstrap.Infer.cinch` | `env-instances`, `env-guarded-bindings` and `env-givens` stop at the marker instead of folding the whole module environment on every use |
| One application-spine walk per expected-type check | `bootstrap/Bootstrap.Infer.cinch` | the ownership, projection, task and thunk head checks share one argument list |
| Interned names in the seed's frontend | `compiler/src/intern.rs` | `Ty`/`ITerm`/`EffLabel`/`RowTail` names and the unifier's region substitution are pointer handles; -11.1% on the seed's check of the bundle (section 2.3) |
| Export index for the law closure | `bootstrap/Bootstrap.Load.cinch` | `interface-global` of every export and private method identity is encoded once per interface, so a law closure query is a scan of pre-encoded globals, not an encoding per row per query; -7.5% of the two-line fixed cost's blocks (section 2.7) |

### 2.1 Frontend names and environment projections (2026-11-10)

Self-compile of the 35,000-line bundle, child CPU (user plus system). The
environment caches were settled by an interleaved A/B on the same input
bundle (A then B, then B then A, five pairs), because the machine carries
other lanes' builds and a single run drifts by tens of seconds:

| Build | Best CPU | Median CPU | Worst CPU |
| --- | --- | --- | --- |
| before | 233.0 s | 244.8 s | 265.2 s |
| after the name indexes and the environment caches | 176.9 s | 209.9 s | 228.1 s |

That is 14% off the median and 24% off the best, and the two builds emit
byte-identical IR. The name index alone measured inside the noise of two
single runs (208.8 s before, 210.6 s after), so the resolver's scans were
not what the phase names suggested.

Fixed point on this tree: Stage2 and Stage3 emit the same `compiler.ll`
(md5 `27eb01fe966103cb91ccef6506905da5`), link the same `cinch`, and agree on
all 46 `.cinchif` files; the fib, precedence and refusal smokes pass on
Stage3.

The 1 kHz sampled profile of the whole compile (542,000 samples of the
working thread) puts inference at 35%, and the rest in interface JSON
parsing, identity decoding, and the runtime's allocation and reference
counting. Three consequences are worth recording.

`binding-at` is the largest single frontend leaf at 6% of all samples. It
cannot go away the way `lookup-binding` did: a binding stored in the
marker's map is a recursive occurrence nested inside another type
constructor, which the positivity discipline refuses, so the map stores
positions and the lookup walks to them.

The interface set is re-encoded from its parsed document to bytes and parsed
again by `load-checked-interface-set`, once per elaborated module, because
`total-interface-inputs` reads the JSON back out of the injected `DInterface`
declarations. That round trip plus the JSON parse underneath it is about a
tenth of the frontend, and it is a load-phase fix: pass the parsed contract
through instead of its canonical bytes. Done, and measured, in section 2.2.

Identity decoding (`export-global` / `decode-identity`) is about 4% on its
own, because every candidate name in an export scan is decoded again. The
nominal export's scans are the next thing to index (section 12, item 7).

### 2.2 Allocation volume: where the fixed cost is (2026-11-10)

`CINCH_ALLOC_STATS=1` says how much a program allocates. The runtime's
`CINCH_ALLOC_KINDS=1` census (`compiler/runtime/src/alloc_kinds.rs`) says
what: one line per block kind and one per size class, counted where every
header block is already accounted and written by the same `atexit` handler.
Unset, a block allocation pays one relaxed load and a branch that is never
taken, and the stats line keeps its format. `tooling/alloc_sample.py` reads a
macOS `sample` call graph and attributes its samples to phase frames and to
the callers of the allocator.

Stage3 checking a two-line program (`module Main where`, `main :: Int`,
`main = 0`) and the 3,965-line toolkit bundle
(`Text.{Lex,Scan,Parse,Json,Builder}` and a `Main`), before any change:

| kind | two-line blocks | two-line bytes | toolkit blocks | toolkit bytes |
| --- | --- | --- | --- | --- |
| constructor | 56,643,277 | 1,336,451,360 | 347,856,188 | 7,587,090,856 |
| closure | 8,786,140 | 352,791,592 | 35,689,690 | 1,440,429,760 |
| dictionary | 5,108,438 | 146,613,408 | 46,863,965 | 1,268,306,584 |
| bytes | 3,988,357 | 95,720,568 | 7,258,938 | 174,214,512 |
| `Nat` box | 3,952,939 | 126,494,048 | 7,130,751 | 228,184,032 |
| `Int` box | 0 | 0 | 7,941 | 317,640 |
| all Cinch value blocks | 78,479,151 | 2,058,070,976 | 444,807,473 | 10,698,543,384 |

| size class | two-line blocks | toolkit blocks |
| --- | --- | --- |
| 16 | 15,884,649 | 125,384,322 |
| 24 | 40,255,703 | 253,896,187 |
| 32 | 8,428,869 | 16,068,211 |
| 40 | 12,672,439 | 39,666,722 |
| 48 | 1,088,582 | 7,734,631 |
| 64 | 145,788 | 1,731,858 |
| 96 | 3,120 | 325,541 |
| 128 | 1 | 1 |

The traffic is not big blocks spread thin: it is 24-byte and 40-byte
constructor cells, two and four words of payload, which is what a list spine,
a tuple and a JSON node cost. Memo, knot, quote and raw blocks never appear
on either input.

The whole fixed cost is the interface set. The same two-line program checked
against a manifest with an empty interface list allocates 22,498 blocks, so
89,773,527 of its 89,796,025 are the 59 interface contracts (1.68 MB) being
loaded, materialized and elaborated. A 1 kHz sample of the whole run, working
threads only, before (`6,352` samples) and after (`3,469`):

| frame, inclusive | before | after |
| --- | --- | --- |
| reading the manifest and the contract files | 767 (12%) | 812 (23%) |
| loading the interface set (`load-interface-set`) | 1,680 (26%) | 1,208 (35%) |
| - `parse-json` underneath it | 616 | 330 |
| - contract identity (`canonical-bytes`, `fnv-step`) | 230 | 105 |
| - export rows (`loaded-exports`, `public-global`) | 447 | 41 |
| resolve (`check-source-phase`) | 913 (14%) | 268 (8%) |
| elaborate | 1,131 (18%) | 495 (14%) |

That is the round trip section 2.1 named, with a price: `total-interface-inputs`
handed each contract's canonical bytes to `load-checked-interface-set`, so
elaboration re-encoded 1.68 MB of JSON (sorting every object) and parsed it
back on top of the load that had already done both. Two fixes landed.

**Admit a contract from the declaration that carries it.** `load-interface`
factors its envelope checks into `check-contract-envelope`, and beside it
`load-interface-from-contract` re-admits the pair a `DInterface` states: the
parsed contract and the identity the load that parsed it derived. The set
loaders carry that pair (`load-interface-set-from-contracts`,
`load-checked-interface-set-from-contracts`), so `total-interface-inputs`
passes `Json` and no consumer re-encodes or re-parses an interface.

**Read an interface's export rows once.** `loaded-exports` walked the whole
contract and allocated a fresh triple and two strings per export row, and
`public-global` asked for that list once per name it bound, so binding one
interface's names cost names x rows allocations (2.07M row visits over the 59
interfaces, twice each). The rows are now read at load, and `ExportEntries`
carries a name-keyed table, so `loaded-identities` - the probe `public-global`
and `overloads` make - is a lookup whose answer is exactly the list the old
scan returned.

| input | blocks before | blocks after | bytes before | bytes after | CPU before | CPU after |
| --- | --- | --- | --- | --- | --- | --- |
| two-line program | 89,796,025 | 54,095,275 | 2.172 GB | 1.326 GB | 2.79-2.92 s | 1.48-1.63 s |
| toolkit bundle | 464,097,719 | 428,396,969 | 10.972 GB | 10.126 GB | 13.86-14.53 s | 12.83-13.87 s |

That is -39.8% of the blocks and -38.9% of the bytes on the fixed cost, and
-45% of the CPU time of checking a two-line program; the toolkit keeps -7.7%
of its blocks and about -9% of its CPU, because its per-line checking
dominates. By kind, the two-line program's constructor blocks fall from 56.6M
to 30.9M and its `Nat` boxes from 3.95M to 2.37M.

The allocation sites agree with that, from the pre-fix compiler linked at
`-O0` so the frames are not inlined away (identical block counts; the toolkit
run, samples under each entry point): `cinch_alloc_ctor` is entered most often
from `Data.String.utf8-at` (126), `Data.Char.char-from-code` (72),
`Data.List.reverse-into` (46), `Bootstrap.Type.read-carrier-tree` (43),
`Data.String.unpack-from` (37), `Bootstrap.Load.value-exports` (36) and
`Data.Core.Cons` (30) - string and identity decoding first, then the export
rows; `cinch_alloc_shaped` from `Data.String.unpack-from` (41),
`Data.Nat.nat-rem` (32), `Data.Core.Cons` (30), `Data.List.from-list` (28) and
`Bootstrap.Nominal.code-digits` (23); `cinch_alloc_raw` is never entered,
which is why no memo, knot or quote block appears in the census.

Verified: Stage2 and Stage3 emit the same `compiler.ll` (md5
`0fece696bbedb59fa950861294e60f32`) and agree on all 46 `.cinchif` files; a
self-compile of the bundle by the pre-fix Stage3 and by the post-fix Stage3
produces byte-identical `compiler.ll` and interfaces, so neither fix changes
what the compiler emits; the fib, precedence and refusal smokes pass on
Stage3; and the target set (integration, ownership, modules, bootstrap_infer,
bootstrap_typing, bootstrap_pipeline, bootstrap_resolve, prim) fails exactly
the tests it failed before the change, all of them the pinned seed's.

The next levers in this path, in measured order: `validate-interface-set` is
20% of the fixed cost and allocates almost nothing (`string-compare` over
dependency names, so it is a hash-set problem), `interface-declarations`
materializes every interface's `DExternal` declarations once per compile, and
elaboration proper is the rest. Beyond the fixed cost the traffic is the
checker's own constructor cells: the toolkit's 428M blocks for 3,965 lines are
108,000 blocks per line, which is where the steepest lever now is.

### 2.3 Interned names (2026-11-10)

The seed's profile, not its phase names, said where to start. Forty
8-second samples of `cinch check` and `cinch emit-ir` on the 40,052-line
compiler bundle put the fixed cost in three places: the allocator (16% of
self samples in `malloc`/`free`), `Core::clone` plus its drop glue (11.6%
inclusive in `check`, 15% in `emit-ir`), and name traffic (2.7% hashing
strings, 1.5% cloning them, 1% byte-comparing them). `Ty::clone` and its
drop glue are 3.4%, not the top of the list: the checker builds far more
`Core` than `Ty`, because match compilation, let-lowering and the
reference-count pass all run through it.

`compiler/src/intern.rs` adds `Interned`: a two-word handle naming one
canonical allocation per spelling. Equality and hashing are pointer
operations, cloning copies two words, and ordering is the *spelling's*, so
a `BTreeSet` or `BTreeMap` keyed by a name keeps the order its emitted
bytes were written in. Interned: the name fields of `Ty`
(`Adt`, `Param`, `Region`, `Ref`, `Mut`, `Lazy`, `Pi`, `Sigma`), of
`ITerm` (`Var`, `Num`, `Fn`, `Lam`), `IndexArgument`, `EffLabel` (head and
region argument), `RowTail::Rigid`, and the unifier's region substitution.
`Unifier::resolve_region` returns a handle, so following a region chain
allocates nothing.

Measured on the seed, five interleaved pairs in both orders per workload
(A then B, then B then A), child CPU from `/usr/bin/time`:

| Step | check bundle | toolkit | emit-ir |
| --- | --- | --- | --- |
| `Ty`/`ITerm`/`IndexArgument` names | 6.84 s to 6.68 s (-2.3%) | 0.70 s to 0.65 s (-7%) | 16.47 s to 16.41 s (-0.4%) |
| plus `EffLabel`, `RowTail`, region maps | 6.84 s to 6.08 s (-11.1%) | 0.65 s to 0.59 s (-9.2%) | 15.90 s to 15.09 s (-5.1%) |

The second step is the one that paid, and the reason is worth recording:
interning the *binder* names of `Ty` barely moved the needle, while
interning effect labels and row tails did, because effect-row equality runs
on every unification (`e1 == e2` over a `Vec<EffLabel>`) and region
substitution is one `HashMap` probe per region occurrence during zonking.
Names that are only printed are cheap already; names that are compared in a
loop are the ones to intern.

Emitted output is unchanged: the seed writes byte-identical IR
(`compiler.ll` md5 `dc30feb2367cdd52f86281b64481b4a9`) and all 105
`.cinchif` files match before and after.

Gates: `cargo test --lib` passes 241 of 241; integration, refinements, hk,
class_defaults, effects, ownership, codata, modules,
interface_class_contracts, bootstrap_check, bootstrap_typing and
core_output pass 188 of 188. `bootstrap_infer` fails its one test with the
built runner exiting 102 instead of 0, and it fails identically at the
commit before this change, so it is pre-existing rather than a regression
(design/next-steps.md already records it as a retained failure).

Two limits, measured rather than guessed, bound what interning can still
reach in the seed:

- `Core` is the hottest cloned structure and interning *it* is not a
  representation change that can be made in place: the optimizer passes
  mutate nodes through `core::child_slots(&mut Core)`, so an arena of
  integer ids would have to be threaded through `simplify`, `rc`,
  `specialize` and `codegen` together. The names inside `Core`'s payloads
  (about 1% of the check run) are what interning can take today.
- `codegen::build_unit` rebuilt `BTreeSet<String>` of every unit name once
  per emitted unit (`reg.arity.keys().cloned().collect()`). That line was
  28% of the `emit-ir` run's samples (3,439 of 12,282, almost all in
  `quicksort<String>` and `memcmp`): an accidental quadratic in the unit
  count, not an interning problem. Fixed, see below.

### 2.4 One unit-symbol set per module (2026-11-10)

The callee classifier only asks membership (`units.contains(sym)`), yet
`build_unit`, `prim_impl_unit` and `prim_wrapper_unit` each rebuilt the whole
name universe and sorted it again. The set now lives in the registry
(`Registry::unit_symbols`, a `OnceLock` filled on first use: every unit's
arity key plus the tail workers derived from it), so the names are collected
and sorted once per module and every body scan is a hash probe.

Interleaved A/B, five pairs in both orders, child CPU:

| Workload | before | after |
| --- | --- | --- |
| `emit-ir`, 40,052-line compiler bundle | 16.12 s | 8.62 s (-46.5%) |
| `emit-ir`, 3,966-line toolkit | 1.03 s | 0.69 s (-33.0%) |
| `check`, 40,052-line compiler bundle | 6.54 s | 5.90 s (-9.8%) |

The `check` row is the surprising one: that path does not emit IR, so the
gain is either the registry build it does share or code layout, not the pass
itself. `compiler.ll` stays
`dc30feb2367cdd52f86281b64481b4a9` and all 105 `.cinchif` files match, so
the emitted bytes did not change.

One more candidate from the same profile was tried and dropped. The intern
table's `Mutex` shows up as `__ulock_wait`, 8% of the seed's check samples,
so `intern` gained a lock-free per-thread cache of the last 512 spellings
(content hash, hit verified against the cached spelling). Measured as five
interleaved pairs against the committed state it did not pay: `check` 6.20 s
to 6.11 s CPU (-1.5%), `emit-ir` 8.35 s to 8.43 s (+1.0%), the toolkit check
0.56 s to 0.60 s. The revert is recorded here rather than the code: the
`ulock_wait` samples are the allocator's arena lock and the second thread,
not the intern table, and a thread-local array is 100 lines that buy
nothing.

After the fix the emit profile's top self entries are the allocator
(`xzm_free`/`xzm_malloc`/`malloc_zone` 14.8%), `__ulock_wait` 9.0%,
`memmove` 5.5%, SipHash 3.7%, `child_slots_imm` 1.6%, `Core::clone` 1.1%,
`memcmp` 1.1% and `Iface::iface_hash` 1.0%; no string sort appears in the
top twenty any more.

### 2.5 Set validation: what it actually costs (2026-11-11)

Section 2.2 named `validate-interface-set` as the next lever and called it a
set-membership problem. That diagnosis was wrong in two ways, and both are
now measured rather than assumed. Attribution below is the two-line program
at `-O0` (identical block counts to the `-O2` link, frames not inlined), one
stage of validation added at a time:

| validation stage | blocks | delta |
| --- | --- | --- |
| none (validation skipped) | 56,532,198 | |
| duplicate check and dependency ordering | 57,141,679 | +0.61M |
| plus `verify-set` (stale and missing dependencies) | 57,186,141 | +0.04M |
| plus `verify-nominal-set` | 78,720,303 | **+21.53M** |
| plus `verify-body-set` | 79,832,313 | +1.11M |

So validation is 29% of the two-line fixed cost, not 20%, and 92% of it is
`verify-nominal-set`; membership and ordering are 0.65M, 2.8% of validation.
Inside `verify-nominal-set`, measured by removing one operation at a time:

| operation | blocks |
| --- | --- |
| `verify-signature-contracts`, of which the law-closure test | 13.02M (7.72M) |
| - within it, the projection comparison (`canonical-bytes`) | 5.14M |
| `canonical-bytes` of every nominal row | 4.78M |
| `signature-owner-valid` (compares two parameter lists) | 2.17M |
| the `prior-nominal` duplicate scan | 0 (no allocations) |

**Hash sets were tried and are a pessimisation here.** An index of
`HashMap String` (module name to interface, name set to `0`, both shapes the
pinned object carries at `[Obj, Obj]` and `[Obj, TObj]`) built once per
validated set, with every `member` and `find-interface` replaced by a probe,
grew the two-line build from 79,832,313 to 81,368,760 blocks (+1.9%) and moved
CPU by +0.6%. The reason is the prelude's `Hash String`: `Data.Bytes` hashes
by folding `nat-mul h 31` over the bytes in `Nat`, so hashing one module name
is a chain of bignum multiplications, while the scan it replaces is a couple
of dozen short `string-compare`s that exit on the first differing byte. A set
of 59 short names is cheaper to scan. `Bootstrap.Load`'s `InterfaceIndex`
therefore carries a list and the reason is recorded where the code is.

What landed instead, in `Bootstrap.Load`:

* **Ownership prefixes are encoded once per set.** `body-node-owner` used to
  build `name <> "."` and encode it per interface per body node; the index
  now carries each module's name, width and encoded prefix.
* **A signature contract owned by the interface being checked is not
  compared with itself.** `signature-owner-valid` and
  `verify-signature-contracts` looked the definition up and then compared two
  independently canonicalised projections of the same row. A row whose
  identity is owned by the module under test is that row, and `canonical-bytes`
  is deterministic, so the comparison is skipped and the law-closure check
  still runs.
* **Private method rows are read at load, once per interface.** They were
  derived per ask, and the law-closure test asks every interface for them once
  per law reference.

Result on the two-line program (`--no-pgo` stage links, median of five):

| metric | before | after |
| --- | --- | --- |
| blocks | 79,832,313 | 77,790,358 (-2.6%) |
| bytes | 2.182 GB | 2.129 GB (-2.4%) |
| CPU | 1.668 s | 1.667 s |

Emitted interfaces are byte-identical for the two-line program and for the
toolkit bundle, and for the compiler bundle itself.

The next largest item in this path was the law-closure test (7.72M blocks,
9.7% of the fixed cost): `law-export-present` re-encoded every export identity
of every interface with `interface-global` on each query. That landed as the
one pass `loaded-exports` already got, applied to the names a law may
reference, and section 2.7 measures it. After that, `canonical-bytes` of
nominal rows (4.78M) is the floor unless rows are deduplicated before encoding.

### 2.6 Serialisation (proposal, not implemented)

`parse-json` is 9.5% of the fixed-cost sample after the last round (330 of
3,469 samples in section 2.2), and the nominal and signature checks above are
22% between them because they re-derive text from the parsed form. A sectioned
encoding is worth it only after the algorithmic items, and it is a schema
bump: `iface-schema-version` 10 to 11, so every `.cinchif` and the pinned
provider must be regenerated, and the fixed point must be re-established at
18 interfaces per module rather than per file.

What each side would have to change:

* The Cinch reader (`Bootstrap.Interface`, `Bootstrap.Load`) gains a section
  table: a fixed header, then length-prefixed sections for the export name
  tables, the value rows, the type and instance rows, the nominal rows and the
  total bodies. `ExportEntries` already wants exactly this shape; today it is
  extracted from a general JSON document.
* The Rust seed (`compiler/src/interface.rs`) writes the sections instead of
  `JsonValue::write_canonical`, and `iface_hash` covers the section bytes
  directly instead of a blanked projection, which removes the text surgery
  `blank-field-string`/`empty-array-field` performs today.
* The fixed point requires the seed, the library provider and all three stages
  to agree on the new bytes, so it is a coordinated bump, not an incremental
  one. Nothing here should be started before the demand-driven load in
  section 12 item 4 lands, because both touch the same reader.

### 2.7 One encoded global list per interface (2026-11-11)

Section 2.5 ended by naming the law closure as the next item: 7.72M blocks,
9.7% of the fixed cost, spent in `law-export-present` re-encoding every
export identity of every interface on each query. That is now one pass at
load.

`law-global` asks, once per law occurrence, whether a name is exported by
some interface in a dependency closure. It walked every row of every
reachable interface and called `interface-global` on the row's identity:
that function re-spells the identity into its module-qualified global, which
allocates a character list and a fresh string. So the closure paid one
encoding per row per query.

`ExportEntries` now carries those globals already encoded, beside the name
rows: every export identity and every private method identity, encoded once
when the interface is loaded. `export-present-in` scans that list with
`string-compare`. A list, not a `HashMap`, because 2.5 measured hash sets as
a pessimisation here: the prelude's `Hash String` folds `nat-mul` over the
bytes, so a probe costs more than the short compares it would replace.

Interleaved A/B on the stage links, three runs per arm in both orders, child
CPU:

| input | blocks before | blocks after | bytes before | bytes after | CPU before | CPU after |
| --- | --- | --- | --- | --- | --- | --- |
| two-line program | 53,096,378 | 49,115,764 (-7.5%) | 1.306 GB | 1.193 GB (-8.7%) | 1.22 s | 1.13 s (-7.1%) |
| toolkit bundle | 427,414,470 | 423,433,856 (-0.9%) | 10.107 GB | 9.993 GB (-1.1%) | 9.74 s | 9.58 s (-1.6%) |

Both rows lose the same 3,980,614 blocks, and that is the shape of the fix:
the work was per compile, not per line. The closure runs over the loaded
interface set, so the two-line program paid all of it (-7.5% of its blocks);
the toolkit bundle's own checking is 374M blocks, and the same saving is
under 1% of its total.

Verified: stage 2 and stage 3 reach the fixed point and the fib, precedence
and refusal smokes pass on all four stages; one frozen source of the compiler
emitted through the pre-change and the post-change stage 2 gives a
byte-identical `compiler.ll` and the same 46 `.cinchif` files, so the change
is not observable in what the compiler emits; the 12 example programs, the
fib and classes smokes and the toolkit and compiler bundles agree in exit
code, output, diagnostics and emitted IR across the two compilers (30 cases,
no disagreements), and the law, load, exported-interface, source-instance and
class-contract targets give identical pass and fail lists through both.

The remaining cost in this path is the closure's membership test itself:
`export-present-in` still walks every loaded interface and scans the
`reachable` name list per interface per law. That is a set problem
`name-set`/`name-in` cannot solve with a hash map (2.5), so the next step is
to shrink the candidate list once per verification rather than per query.


### 2.8 What `check` was recomputing (2026-11-11)

A 4,157-sample profile of `cinch check` on the 40,052-line compiler bundle
put 6.2% of the run inside `Iface::iface_hash` under `verify_chain`, 8.1%
in `std::fmt::format` (4.4% of it in `value_index_id`, 2.7% in the
dictionary-value owner walk), 3.4% in `FnDef::clone` plus 2.7% dropping the
units it had just cloned, and 39% inclusive in `specialize::optimize_with`,
19% in `core::optimize` and 15% in `simplify::simplify`, the optimizer that
`check` runs in full even though it emits nothing. Three of those were
recomputation rather than work.

**The chain check hashed every edge.** `verify_chain` recomputed the
provider's contract hash once per import edge. The compiler bundle's
interface set has 868 edges over 105 modules, so the same 105 documents were
canonicalized and hashed 868 times. The hash is a pure function of the
provider's contract, so `interface::contract_hashes` computes it once per
module and every edge compares against that; `analysis::chain_ok` had copied
the loop and now shares the same treatment. The tamper check is unchanged:
each provider's content is still hashed from the document that was parsed,
and a mismatch still fails with the same message.

**The ownership walks formatted a prefix per candidate.** `value_index_id`
and the method-body ownership loops in `verify_chain` built `format!("{module}.{name}")`
for every candidate they tested, so a linear scan allocated once per element
visited; `dictionary_values::lower` did it for every (unit, method) pair.
`value_names_head` and `under_module` compare the two halves in place, and
`lower` tests the prefix plus its separator, which is the same predicate
without the allocation.

**Whole programs were cloned to be emptied.** `specialize_with_roots` began
with `program.clone()` and then cleared both unit lists; `core::optimize`
cloned the whole program and then cloned each unit again for the driver and
once more for the driver's calling-unit field, four body copies per unit.
`Program::skeleton` copies every non-unit field with both lists empty, the
driver borrows its rule table, its instance table and the calling unit, its
rule application copies the rule reference instead of the rules (a deep
`Core` copy per visited node), a folded `case` moves the chosen alternative
instead of copying it twice, and the simplifier's `let` no longer clones its
constructor binding to test it. Four body copies per unit became two: the
unfolding table's and the emitted unit's.

**The row stripper copied the row.** Consuming column 0 in the match
compiler cloned the alternative (its environment and every remaining
pattern) and then replaced the pattern list. The row is owned there, so it
moves and `remove(0)` shifts the rest down.

**What the second thread is for.** `__ulock_wait` is 5.2% of the samples,
and every one of them is the main thread in `pthread_join` waiting for the
worker that runs `check_laws`; the worker is 5% of the samples itself. That
is the join, not allocator contention, which corrects the sentence above
about the arena lock. A CPU profile does not count it as work, so what it
measures is imbalance: the lever is to give the worker more than the law
check, not to remove a lock.

Measured as interleaved pairs in both orders, child CPU (`user + sys`),
each step against its own parent (so the absolute numbers differ between
rows: the machine carries other lanes' builds):

| Step | median | best |
| --- | --- | --- |
| hash once per provider module | 6.64 s to 6.44 s (-3.0%) | 6.17 s to 6.01 s |
| whole-program clones removed | 5.94 s to 5.75 s (-3.2%) | 5.32 s to 5.20 s |
| prefixes not formatted per candidate | 5.63 s to 5.22 s (-7.3%) | 5.41 s to 4.96 s |
| row consumption moves the row | 5.77 s to 5.63 s (-2.4%) | 5.44 s to 5.33 s |
| the four together | 5.97 s to 4.95 s (-17.1%) | 5.57 s to 4.86 s |

The whole change against the tree before it, eleven interleaved pairs per
workload (nine for `emit-ir`):

| Workload | before | after |
| --- | --- | --- |
| `check`, 40,052-line compiler bundle | 6.67 s median, 6.17 s best | 5.33 s (-20.1%), 5.16 s (-16.4%) |
| `check`, 3,966-line toolkit | 0.59 s median, 0.53 s best | 0.36 s (-39.0%), 0.35 s (-34.0%) |
| `emit-ir`, compiler bundle | 8.88 s median, 8.39 s best | 7.94 s (-10.6%), 7.65 s (-9.1%) |

The toolkit gains most because its run is mostly the fixed cost: the
interface chain check and elaboration of the prelude, which is where the
hash and format work was. `emit-ir` gains least because its time is the
optimizer and the emitter, and only the whole-program clones and the
per-node rule copy reach those.

`compiler.ll` stays `e488d5979fbe9921f1cdb643e80c8da9` and every `.cinchif`
and the manifest are byte-identical across the change, compared on the full
emit artifact tree.

One candidate from the same profile was tried and dropped: `simplify::pass`
and the other child walkers collect their children into a `Vec` per node,
and `core::each_child_mut` removes that allocation. Measured over eleven
pairs it did not pay (5.00 s median to 4.97 s, 4.88 s best to 4.86 s), so
it is not in the tree; `child_slots` stays.

`Iface::iface_hash` still clones the interface and re-serializes it, which
`to_json` could avoid by borrowing its overrides. After the dedup that is
one hash per module instead of one per edge, under 1% of the run, so the
rewrite waits for a slice where the hash is measured again.

### 2.9 A cheap digest, and the indexes it buys back (2026-11-11)

`Data.Hash`'s `String` instance was the reason hash indexes lost. It folded
the bytes with `nat-mul h 31` over `Nat`, so one compiler-length identifier
(8 to 40 bytes) cost about 1,700 ns and 73 allocations for its digest, while
the scan the index would replace costs about 600 ns at 64 short names. No
index can pay at that price, and 2.5/2.7 recorded the reverts.

The instance is now FNV-1a in a wrapping 64-bit word
(`compiler/prelude/Data/Bytes.cinch`): the accumulator is a `U64` at every
step, the multiply is `wrapping-mul-u64` (modulo 2^64, so no value can leave
the word), and the word is reduced to `Nat` exactly once, at the end, by
clearing the top bit and calling `nat-from-i64`. The class law is the only
obligation the digest carries, and it holds: the fold reads content only and
is a deterministic function of the bytes, so equal slices hash alike, no
address or clock enters the result, and two distinct slices may still share
a digest (the bucket scan decides). `Data.String` and `Data.Sym` hash their
backing bytes through the same instance, so the three agree by construction.

The digest before and after, driven by `bootstrap/programs/Run.HashDigest.cinch`
(native programs built by the seed; the program's own monotonic clock, child
CPU, median of three interleaved runs):

| measurement | `nat-mul` 31-fold | FNV-1a 64 |
| --- | --- | --- |
| digest of one 8 to 40 byte name (mean 24) | 1,700 ns | 436 ns |
| digest of 0 / 8 / 24 / 40 bytes | 60 / 316 / 1,445 / 2,951 ns | 60 / 172 / 437 / 679 ns |
| blocks per digest (an isolated 10,000-name run) | 73 | 1 |
| probe in a 10,000-key `HashMap` (digest plus spine) | 1,956 ns | 804 ns |
| insert into the same map (digest plus spine) | 2,792 ns | 1,864 ns |
| lookup by scanning a list of 64 names | 602 ns | 632 ns |
| lookup by scanning a list of 256 names | 2,069 ns | 4,058 ns |
| lookup by scanning a list of 1,024 names | 8,501 ns | 9,658 ns |
| the whole benchmark's blocks and bytes | 5.09M, 155 MB | 0.61M, 17 MB |

The fixed 60 ns both arms share is the reduction: one boxed `Nat` per digest,
which is the floor the class's type imposes. What changed is the slope, 82 to
19 ns per byte.

Two consequences. First, the crossover moved: a probe used to lose to a scan
until a set held roughly a thousand keys, and now a 256-name set is a 5x win
for the probe while a 64-name set still ties (632 against 804 ns), because
the digest alone is 436 of the probe's 804 ns. Second, the allocation traffic
went with the time: 73 blocks per digest to 1, so hashing no longer competes
with the checker's own cells.

**The indexes re-tried, and what each measured.** `Bootstrap.Load`'s
`InterfaceIndex` now carries two `HashMap`s: module name to interface (59
entries), and global name to exporting module (every exported identity and
every private method identity of the set, thousands of names in the
compiler's own set). The second replaces the walk the law closure made per
law reference, which rescanned every interface's exported globals; the
invariant that makes one owner per name sound is that `interface-global`
prefixes an identity with its exporting module, and the fold inserts the
head interface last so the first in the set's order wins, as the scan did.

Measured with stage compilers built against a library object emitted from
this tree's prelude (see the pin note below), child CPU, three interleaved
runs each:

| arm | toolkit check | the 40-module self-check |
| --- | --- | --- |
| old digest, scans | 11.08 s | 99.3 s |
| new digest, scans | 9.82 s (-11.3%) | 97.1 s (-2.2%) |
| new digest, both index tables | 9.76 s (-0.6%) | 96.3 s (-0.8%) |

So the digest is worth about 11% of a toolkit check and a couple of percent
of the self-check (whose time is mostly elaboration, not name traffic), and
the index tables are inside the noise on both. The self-check runs scatter
over 90 to 125 s, so its deltas are weaker than the toolkit's; the toolkit
figure is the reliable one. The index stays because its cost is now bounded
and its algorithmic effect is not a micro-optimisation: the law closure went
from `references x interfaces x globals` string compares to one digest per
global per set, and the table build is that same digest count. What stays a
list scan is the closure membership test (`name-in` over `reachable` and over
the ordering pass's `done`): those sets hold tens of short names, exactly the
size at which 632 ns of scan beats an 804 ns probe, and 2.9's next attempt
should be a smaller digest, not a bigger table.

**Pin note.** The library object the stage chain links is pinned
(`bootstrap/library-provider.lock.json`, revision `12a9bd37`), and the pin
carries the prelude's *compiled* code. A prelude change therefore reaches
the stage compilers only after the pin moves to a revision that contains it.
The three arms above were built by emitting a library object from this
tree's prelude with the release seed (`cinch emit-object`) and running
`build_self` against it, so the numbers are the compiler's own, with the
digest it will have after the bump; the checked-in chain still hashes with
the old instance until then. The `Run.HashDigest` numbers are unaffected by
the pin: that program is compiled by the seed, which reads the prelude from
source.

**Oracles.** The change is contract-preserving: every `.cinchif` of the
prelude is byte-identical across the two preludes except `Data.Bytes`'s own
`source_hash` field. Emitted IR agrees on three programs (fib, classes,
Run.Maps) once two renumberings are normalised (metadata ids, and the
representation-class index that `cinch_alloc_shaped` takes, both of which
shift when new units enter the module): the only added units are
`Data.Bytes.fnv-offset` and three `cinch.prim` wrappers, and the only
differing bodies are `Data.Bytes.hash-bytes-from`, `Data.Bytes.Hash_Bytes.hash`
and the two instances that delegate to it. Behaviour agrees on the same
programs (same exit codes, same check lines). Hash order is not observable
by contract (`hash-map-to-list` documents its order as unstable), so no
program may depend on the reordering a new digest produces.

### 2.10 Identifier comparison: count, not width (2026-11-11)

A 40 s `sample` of a stage-3 self-check put the string comparison family at
19.1% of the working thread's samples: `cinch.runtime.bytesCompare` 5,768 of
30,244 inclusive, `_platform_memcmp` 4,177 self (13.8%), and
`Data.String.string-compare` 1,890 self. Nothing else in the profile was
close except the allocator.

The question was whether that cost is the width of one compare or the number
of compares, because the two answers have different fixes. A census inside
`cinch_bytes_compare` (temporary, compiled in for these runs) answered it by
counting every call, every byte read before the decision, and the call's
operand lengths:

| input | compares | bytes examined per compare | mean shorter operand | equal lengths | equal contents |
| --- | --- | --- | --- | --- | --- |
| 3,964-line toolkit | 187,029,613 | 1.87 | 8.56 | 31.3% | 20.1% |
| 41,223-line compiler bundle | 2,290,295,595 | 4.20 | 17.17 | 11.3% | 4.6% |

The compares are short and there are billions of them: 88% of the
self-check's compares decide within the first four bytes, and half of them
are between operands longer than eight bytes that differ at the front. The
cost is the count.

**The fast path.** `cinch_bytes_compare` recompiled every call through
`memcmp`: a call, a dispatch and a byte loop, for a name one word long. It
now reads the common prefix eight bytes at a time as a big-endian word
(which is byte order, so the first differing byte decides the sign), answers
two words over one region from the pointers alone, and reads the lengths
last, where they break a prefix tie and never decide the order of two
differing prefixes. `Data.Bytes`' `Eq` instance routes through the same
entry instead of its own Cinch byte loop with a bounds-checked `bytes-get`
per byte. Byte-lexicographic order is observable (canonical interface rows,
JSON keys, diagnostics), and it is pinned two ways:
`compiler/runtime/tests/bytes_compare.rs` asserts every pair of a corpus
(empty, prefixes, a difference at each position inside and across a word
boundary, high bytes, 1 to 17 bytes) against `&[u8]::cmp`, and the stage
chain's fixed point keeps stage 2 and stage 3 byte-identical in `compiler.ll`
and in all 46 `.cinchif` files.

Child CPU, interleaved runs in both orders, on the pinned revision's
runtime object:

| input | baseline | fast path | fast path + index |
| --- | --- | --- | --- |
| toolkit check | 6.93 s | 6.44 s (-7.1%) | 6.08 s (-12.3%) |
| toolkit check, wall | 7.14 s | 6.71 s (-6.0%) | 6.09 s (-14.7%) |

**Where the compares came from.** The census said count, so the next
question was count *where*. The repeated work was two passes in the
resolver, each asking a membership question of the reference list once per
imported declaration: `Bootstrap.Contracts.materialize-externals` asked
whether the program mentions an external, and
`Bootstrap.Resolve.imported-record-references` asked the same of every
external's name. The reference list holds every symbol the program mentions
(tens of thousands) and the imported declarations number in the thousands,
so each pass is a product of two large lists, with a `string-compare` per
element: 2.29 billion compares on the self-check.

Both now take one `HashMap String Bool` built from the list, so a question
is a hash probe. The materialized set threaded through
`materialize-externals` is a table by name whose value is the kinds the name
was materialized as, which is the pair the scan compared. The compare count
fell from 2,290,295,595 to 1,836,320,953 (-19.8%) and the toolkit's from
187,029,613 to 163,105,931 (-12.8%); the reference index is 0.1% more
blocks (264.3M to 264.9M on the toolkit) and no visible time.

**Interning, measured and rejected.** The Lean model is an interned name
compared as a handle. It was not taken here, and the measurements say why.
An `intern` is one hash-table probe of the name: a standalone measurement of
the runtime's own table shape (`HashMap<Vec<u8>, usize>`, SipHash, 20,000
identifier-sized names) puts it at 10.3 ns, and an out-of-line identity
compare (`cinch_sym_eq`'s shape) at 0.54 ns. Candidate (b), interning at
`same`, therefore costs two interns per compare (about 21 ns) where the
current compare costs about 5 ns: it loses by four times on 2.29 billion
calls before it removes anything. Candidate (a), interning names where they
are built and comparing handles afterwards, pays one intern per name
(cheap: the compiler's source has a few hundred thousand name occurrences,
a few milliseconds) and turns each compare into a 0.54 ns call, so its best
case is 2.29 billion compares times about half a nanosecond, roughly a
second, and it needs a `Sym` in every AST and interface name, thousands of
lines across the checker. The index removes 454 million of those compares
and the fast path halves the price of the rest, for about sixty lines. If
the remaining 1.84 billion compares are attributed to their callers the same
way, the next lever is another index, not an interned representation.

Remaining compares are concentrated: after the index the same census gives
1,836,320,953 compares (4.8 bytes examined each), so the fix above did not
remove all of the count. The profile's unnamed frames (Cinch functions
inlined into `resolve-context` at compiler build time) hide which pass owns
them; a compiler built with inlining off would name them, and that is the
measurement this section leaves open.

**The pin, and what is not yet in the chain.** The fast path lives in
`compiler/runtime/src/abi.rs`, so it reaches a stage compiler only when the
library pin moves, and every number above came from builds whose locks
pointed at a revision containing it. On current main that bump does not
land: with both locks moved to a revision containing main's compiler
sources, stage 1's profile training dies with SIGBUS (exit -10) after 1.3 s,
three times out of three, while main's own pin (`bb1ad9b4`, source hash
`8974df6b`) builds green end to end. The locks are therefore left at main's
values: the compare and the index are in the tree, the index takes effect
now (it is compiler source), and the compare takes effect at the next bump.
The numbers above were taken on the pre-rebase main (f158d5c4's line), which
trained fine; they are not re-taken on the rebased tree.

### 2.11 Dense Core bodies: types in a side table (2026-11-11)

Every checked `CoreTerm` node carries its `UType`, and `CorePattern`, the
evidence rows and `Scheme` do the same. That is the right shape for
elaboration: the checker reads the type from the node it is already
matching. It is the wrong shape for a body that leaves the compiler. A
document that writes the type at each node writes the whole type tree at
each node, while the compiler's own definitions repeat a small vocabulary:
5,324 definitions hold 106,695 distinct table entries, 20 per definition,
against far more nodes.

`bootstrap/Bootstrap.CoreBody.cinch` writes the transport form. One type
table per definition, deduplicated by structure (`same-shape`, with a cheap
digest as a filter and the structural comparison as the decision), and every
node, instantiation argument, pattern, constructor type, scheme, requirement
and evidence row refers to it by position. Spans are not written: a consumer
of another module's body cannot blame that module's source, and a decoded
body carries `Span 0 0 0 0`. The reader takes a document back to a
`CoreDefinition`. Production checks its own reader: `--emit-bodies` decodes
every definition it writes and refuses the artifact unless re-encoding
reproduces the same canonical bytes.

Measured on the compiler's own checked definitions (stage3, whole bundle):

| quantity | value |
| --- | --- |
| checked definitions | 5,324 |
| type table entries | 106,695 (20.0 per definition) |
| dense document | 34,037,775 bytes |
| the same bodies with the type tree at every occurrence | 65,210,394 bytes |

The side table writes 52.2% of the bytes the per-node form writes, 47.8%
fewer. Check, encode, decode, re-encode, verify and write all of them takes
75.4 s and 76.8 s real, 73.1 s and 74.8 s user, on the stage3 binary.
`--check` on the same bundle takes 85.1 s and 96.9 s real, 83.0 s and 93.1 s
user, but it also exports and writes every module interface while the body
path writes one document and no interfaces, so the pair bounds the codec
rather than isolating it. Two runs produce byte-identical documents.

Self-check A/B, the same bundle, three interleaved pairs, main's stage3
against this tree's stage3:

| pair | main stage3 real / user | this stage3 real / user |
| --- | --- | --- |
| 1 | 86.30 / 83.71 | 91.61 / 84.92 |
| 2 | 95.30 / 88.60 | 99.57 / 91.77 |
| 3 | 91.67 / 87.91 | 96.09 / 88.34 |

Median user CPU is 87.91 s against 88.34 s (+0.5%); median real is 91.67 s
against 96.09 s, inside the spread of single runs on a machine carrying
other lanes. The check is unchanged.

**Why the node field stays a `UType`.** Moving the type out of the node in
memory was measured and rejected for this work. A `Nat` index in range is an
immediate word in the same block slot the `UType` pointer occupies, so the
node does not shrink; what the node retains is one pointer either way. The
representation change would also need a type table that survives suite
rollback: `Store` is snapshotted and rolled back whole, so a table inside it
can lose entries that terms already reference, and the table would have to be
threaded through `Infer`, `Typing`, `RuntimeIr` and `Core` beside it, across
about 1,100 constructor and type-read sites. The transport form gets the
measured win, 52.2% of the bytes and no per-node type at a module boundary,
without those risks. A dense in-memory node is a separate change with its own
evidence, and the node-size fact above is the first thing it has to answer.

The artifact is a separate file today, the typed counterpart of the Rust
writer's `.cinchopt` bodies. Carrying it inside `.cinchif` is the interface
format item in section 12.

+++++++ wyzzvwks 976238be "Record what the span table bought, and why Cinch keeps its Span" (rebased revision)
### 2.12 Source positions in a dense table (2026-11-11)

Section 3 asks for spans in a side table and out of fingerprints. The seed's
frontend carries a `Span` in every syntax node and in every `ast::Name`,
though no pass reads a position except to render a diagnostic. The table is
in the tree; this section records what it bought and what it did not.

**What changed.** `lexer::Span` is a four-byte handle into one dense table,
with an entry per distinct `(line, column)` pair. The line index is a map and
not a vector because the frontend parses files behind synthetic line bands
(`BAND_WIDTH` apart in `analysis.rs`), so line numbers are sparse. The table
interns, it does not merely index: the parser compares tokens' spans (a `$(`
pair shares one), so two positions must be equal exactly when their line and
their column are, and a fresh id per construction would have broken that.
Reads are memoized per thread, because the layout and analysis passes read the
current token's position several times in a row. Generated spans keep their
old encoding, a marker column indexing the expansion arena, so expansion
provenance renders exactly as before.

**Diagnostics are byte-identical.** Three programs (a type error, an unknown
name, and a parse error at end of input) report `3:10`, `3:10` and `4:1`
character for character before and after. `Debug` and `Display` for `Span`
render the position and never the id, so nothing built from either moves:
the `Debug` form is spelled exactly as the derive wrote it.

**Node size and time: nothing.** Measured with `size_of` on the base revision
and on the tree with the table:

| type | before | after |
| --- | --- | --- |
| `Span` | 8 | 4 |
| `ast::Term` | 296 | 296 |
| `ast::Type` | 272 | 272 |
| `ast::Pat` | 88 | 88 |
| `ast::Name` | 72 | 72 |
| `ast::Decl` | 1320 | 1312 |

The four bytes a node gives up fall into alignment padding: the enums are
eight-aligned and their widest variants end on an eight-byte boundary either
way. Only `Decl`, whose widest variant ends four-aligned, loses eight bytes.
Child CPU on the 41,286-line compiler bundle, interleaved pairs in both
orders: 6.88 s, 7.31 s and 7.23 s before against 7.19 s, 7.20 s and 7.22 s
after, which is the run-to-run spread. On the two-line program the table
costs about 0.02 s of the 0.31 s baseline, the interning lock and the
interned lookup of one position per token.

So the table separates provenance from structure, and that is all it does
here. It removes no bytes and no time from the seed.

**Fingerprints.** Declaration identity was already span-free: `surface_rows`
reads the interface contract and `body_rows` debugs `FnDef`, and neither
carries a position (`compiler/src/incremental.rs`). A new test in
`compiler/tests/incremental.rs` pins it: inserting two comment lines and a
blank line into `Base.cinch` moves every following position and leaves every
declaration's `decl_hash` unchanged, while the module's own hash moves,
because a module's body hash is its source text.

**Why the Cinch compiler was left alone.** The self-hosted AST and Core carry
a `Span` per constructor argument (141 in `Bootstrap.Ast`, 82 in
`Bootstrap.Core`), and the shape of the change is large: 459 `POk` results in
`Bootstrap.Parse` carry the token stream the table would have to ride along
with, 32 `span-join` and 3 `span-here` sites construct a new position, and
103 `sp-of` reads would each turn into an id. A pure language has no place to
put the table except threaded through `Ps`, the parser's result type, and
then carried in `Program` into every rung that renders a diagnostic. But the
size argument that motivated the seed's table does not hold there, and this
is the part worth recording:

- A Cinch node is a word array, so its span slot is one word whether it holds
  a `Span` pointer or a `Nat` id. The block does not shrink. That is the same
  measurement that decided the seed's table, where the widest variants are
  set by their payload and not by provenance.
- The 40-byte `Span` block that a leaf node points at is already shared: the
  parser passes the token's own `Span` value into every node the token
  starts, so no allocation happens per leaf. A composite span
  (`span-join`) is one fresh block per construct, and a table would hold one
  row per distinct composite span, which is the same count. There is no
  allocation to remove.
- What the id buys is indirection: every render, and every
  `span-start`/`span-end` comparison, gains a table lookup, and the table
  itself must be threaded through the parser, the program and the
  diagnostics.

The measured cost of a `Span` in the self-hosted compiler is therefore one
word in a block, the same word an id would use, plus an indirection. The
change is not worth 223 constructor arguments and the parser's threading
until a node carries provenance it does not need on the hot path, which is
the case the seed has and Cinch does not.
The open question is a different one: `Bootstrap.RuntimeIr` and
`Bootstrap.Infer` construct `Span 0 0 1 1` at 114 sites, most of them in
functions entered once per Core node, and each construction allocates. That
is a sharing problem (one value instead of one per call), not a side-table
problem, and it wants a census first: the ABI 9 bump arena means
`CINCH_ALLOC_KINDS` no longer sees these blocks, so the census has to be
taken per layout id in the runtime rather than per size class.

### 2.17 The carrier round trip, and two harness defects that hid it (2026-11-13)

Item 11 of section 12 asked for a measurement of the capture rate per unit
before either candidate fix, because the release path's 13.9% had no
attributable caller. Getting that measurement meant fixing the harness
first, and the harness was hiding more than the attribution.

**Two `profile_build.py` defects.** `sample` draws its group markers
(`+`, `!`, `:`, `|`) in the gutter *before* the count, one marker per two
columns of indent, so a frame four levels deep reads
`      !   | + ! 5763 symbol`. The parser allowed exactly one marker and so
matched no frame deeper than level 1: the call graph collapsed onto its
first few frames and every deeper self count landed on the nearest
surviving ancestor, which is why `after-parse-with` absorbed 45% of the
self-check and why nothing inside it could be named. The marker run is
part of the indent. Separately, `sample` leaves a frame `???` when its
stack walk starts inside a function it cannot name, which was 42% of self
samples; the printed `load address B + O` pair gives the link-time offset
directly, so `nm -n` on the sampled binary names it. With both fixed the
toolkit profile names `rc::release` at 30.0% self and the self-check at
17.9%, and its callers are visible. **Any earlier per-symbol number from
this harness was reading a flattened graph**; the lesson is that a
profiler's own parse needs a correctness check (here: does a known-deep
frame appear at its real depth) before its percentages are believed.

**The measurement, per unit.** Attributing `rc::release` and
`allocate_block` to the nearest named frame, on the toolkit check:

|frame|release|allocate|what it does|
|---|---|---|---|
|`typed-index-carrier`|4.9%|1.9%|encode a type to a document, decode it back|
|`valid-comparison`|3.6%|0.4%|two of those round trips, then compare|
|`read-carrier-tree`|3.4%|1.6%|the decode half|
|`carrier-tree`|2.4%|1.9%|the encode half|
|`opaque-index`|1.7%|0.4%|`opaque-carrier` over the same trees|

So the answer to "reset/reuse capture on those cells, or fewer of those
cells" was neither: the cells were being built to carry a value that the
caller already held. `type-kind` wraps a carrier in `KValue` as a
`carrier-tree` document and `value-carrier` parses that document back, so
every kind observation of a type rebuilt the whole type tree, and
`typed-index-carrier` was `value-carrier (type-kind value)`.

**The fix.** `index-carrier` answers the same question directly: each arm
of `type-kind` that answers with `value-kind carrier` returns that carrier,
and the five arms whose kind is a `Kind` rather than a carrier in hand
(`UCon`, `UBound`, `URigid`, `UMeta`, `UApp`) take the same `value-carrier`
step as before. `typed-index-carrier` is that function.

|workload|counter|before|after|change|
|---|---|---|---|---|
|toolkit check|blocks|251,846,803|160,769,014|-36.2%|
|toolkit check|bytes|5,663,759,709|3,759,771,566|-33.6%|
|self-check|blocks|1,068,183,359|910,795,836|-14.7%|
|self-check|bytes|25,893,114,155|22,613,324,564|-12.7%|

Peak live heap is unchanged on both (24,603,579 and 167,373,342), so the
change removed work rather than retained memory, and the toolkit arm
repeats its counter line byte for byte across three runs. Output is byte
identical (stage2 and stage3 `compiler.ll` equal, binaries equal), three
smokes pass, `cargo test --lib` is 275 passed, and both workloads print
`cinch-leak: live=0 immortal=0 leaked=0`.

**What this leaves.** The round trip is gone but `KValue`'s document is
still the representation: `value-kind` still encodes, so every consumer
that builds a kind from a carrier pays one. The next step in this area is
the `Kind` representation itself, not another caller rewrite.

### 2.18 The frontend is superlinear in definition count (2026-11-13)

Found while sizing the `huge-module` workload for the `-j` sweep, and
worth recording because it is the largest scaling defect measured on this
tree. `emit-ir` over one module of `n` independent `Int -> Int`
definitions, stage-3 binary, no imports:

|n|wall|
|---|---|
|50|3.60 s|
|100|5.70 s|
|200|17.02 s|
|400|77.73 s|

That is quadratic. The profile of the n=200 arm puts 14.1% inclusive in
`Bootstrap.Infer.check-global-core` and 12.4% in
`Bootstrap.Core.total-core-row`, with `total-core-merge` 12.2% and
`total-core-call` 10.6% under it. The mechanism is
`total-core-insert`: a sorted insertion into a `List Json`, called once per
definition by `total-core-gather`, so building the total-core table for a
module of `n` definitions is `n` insertions into a list of up to `n` rows,
each one re-reading and re-comparing identities. The table is rebuilt per
module rather than cached, and `core-total-function` calls `total-core-gather`
again per definition it is asked about.

Nothing here was changed: it is a representation change in
`Bootstrap.Core`'s total-function table, not a local fix, and the item 11
landing was already the larger win. The `huge-module` workload is sized at
200 definitions for that reason, and the sweep records `frontend_s`
separately so this curve shows up as a column rather than hiding in a
total.

## 3. The query graph (rustc, Salsa, Shake)

The single largest structural gap. Today a build rechecks whatever the
entry imports, from source, with a content-addressed store only at package
granularity.

Adopt:

- **Demand-driven queries with recorded dependencies.** Queries: `signature`,
  `elaboratedBody`, `proofClosure`, `erasedCore`, `ownedCore`,
  `optSummary`, `object`. Depend on what a query actually read, not on the
  module. rustc's `DepGraph` and Salsa's revisioned memo tables are the
  references; `compiler/src/store.rs` is the seed of this.
- **Early cutoff by fingerprint.** If a recomputed query's output
  fingerprint matches, downstream work is reused. Fingerprint separately:
  exported signature, transparent body, instance set, rewrite set,
  optimization summary. GHC's interface fingerprints and Shake's early
  cutoff are the precedents.
- **Negative lookups are dependencies.** "No instance for C T" must
  invalidate when a matching instance is added. Record the searched bucket
  (class plus head symbol) as a dependency. Lean records the environment
  state its searches observe.
- **Stable declaration identity.** An edit that inserts a line must not
  change any declaration's semantic identity. Keep spans in a side table
  and out of fingerprints.
- **Interface-first pipelining.** Publish a module's checked signature as
  soon as it exists; let dependents start while its bodies, proofs and
  codegen continue. Rocq's asynchronous proof processing and Cargo's
  metadata pipelining are two versions of this.
- **On-demand interface loading.** Index interface files by symbol so a
  consumer decodes the signature, unfolding or instance it needs. Go's
  unified export data and Clang's PCH are the references. Our `.cinchif`
  is JSON today, decoded whole; a sectioned binary or an offset index is a
  contained change with a large effect on cold builds of many small
  modules.

Where our situation differs: the language has no separate compilation of
bodies across modules for a whole program build, so the first cut is
per-declaration reuse inside one build and per-package reuse across
builds. That still removes the second-largest cost after clang.

### 3.1 What landed (2026-11-12)

`compiler/src/query.rs` is the engine and `Session` drives it. The four
mechanisms, each with the thing that makes it correct rather than
plausible:

- **Demand-driven with recorded reads.** A `Query` is a key, an `Engine`
  is a memo table plus a revision counter, and a `Subject` computes a
  value. A subject reads its dependencies through `get`, so the memo
  records what the evaluation actually read. The queries are `Signature`,
  `ElaboratedBody`, `ProofClosure`, `InstanceBucket`, `OptSummary` and
  `Object`, with `Input` as the leaf a client sets. `erasedCore` and
  `ownedCore` from the list above are deliberately absent: erasure and
  reference counting are whole-program passes, not functions of one
  declaration's inputs, so a per-declaration memo of them would record a
  dependency set that does not describe the work.
- **Early cutoff by fingerprint.** An entry whose every recorded
  dependency still carries the fingerprint it was written with is valid at
  the current revision and is not re-executed; a re-execution that
  reproduces its value keeps its old `changed_at` and reports a cutoff, so
  its dependents are reused. Only an input whose fingerprint changes bumps
  the revision, so setting an input to what it already was costs nothing.
- **Negative lookups are dependencies.** The resolver records the bucket
  `(class, head)` it searched before its trials, so a search that finds
  nothing is recorded exactly like one that finds a candidate. The head is
  normalized to its last segment, which makes a checked identity
  (`Relation.Eq.Eq`, `Base.Count`) and a rendered interface target
  (`Count`) name the same bucket; two modules' same-named types then share
  one, which makes the dependency coarser and never narrower.
- **Interface-first pipelining.** `ElaboratedBody` reads a referenced
  declaration's `Signature` and not its `ElaboratedBody`, because the
  checker elaborates a call from the callee's scheme and a dependency is an
  interface (D3). Only `OptSummary` and `Object` read bodies, because the
  optimizer is the layer that copies them.

Three findings from building it, each a bug the tests caught:

- **The graph is only valid for the state it was built from.** Caching the
  item graph across a re-check made the object cache key on the previous
  state's declaration hashes, so nothing was re-placed. `check_slice`
  drops the cached graph before it replaces the state.
- **A contract fact node must not read its owner.** The graph's fact nodes
  and their owners referenced each other, so the demand hit a cycle on
  every instance with a law. A fact node answers with the fact and reads
  only its module's source.
- **The session forces what it re-checked.** A dependent re-checked
  because its context changed has an unchanged source input, so a
  fingerprint comparison against that input would keep a value the fresh
  check replaced. `Engine::force` marks those entries for recomputation
  while keeping their prior fingerprint, so a re-check that reproduces the
  value still reports a cutoff.

`split_ir` was anchoring its unit split on the `.clo` closure global, which
codegen stopped emitting for a unit that is never taken as a value (the
apply-stub collapse), so the split fell back to one blob and the LINK cache
was inert. It now anchors on each unit's `define` and attaches the
module-level tables codegen emits between units to the block they follow,
so a table change re-places exactly one unit instead of dropping the table
from the reassembly.

The reach is narrower than the reference-edge closure and that is the
point: a contract change now re-checks the signatures that read it plus
every module node on the chain, and leaves a declaration that names the
changed one only from a body whose fingerprint cannot move. The measured
effect is not yet a timing claim; what is pinned is the reach (the
`queries.changed`, `queries.cutoff` and `queries.researched` rows of the
rebuild manifest and of `SessionReport`) and the cutoffs the tests assert.

## 4. Make the checker boring (Lean's kernel, elaboration-zoo)

- **Intern immutable terms.** Identity comparison first, structural second,
  rigid-head third, reduction last. Reduction to weak head normal form on
  demand only. Lean's kernel is the model; elaboration-zoo gives small
  implementations to compare against.
- **Sleeping constraints.** Index blocked constraints by the metavariables
  they wait on; wake only those on assignment. Split independent constraint
  components so failures do not serialize.
- **Tabled instance search.** Canonicalize subgoals, share answers across
  branches, detect cycles, index by class and known heads. Our dictionary
  resolver already replays the winning candidate; tabling extends that to
  shared subgoals across a whole module.
- **Bidirectional checking with expected types pushed inward**, and
  overload resolution treated as a language-design cost: Swift's
  type-checker experience is the warning label.
- **Unfold control.** Opaque bodies for stable interfaces, explicit
  transparency where conversion needs computation, and rewrite rules kept
  out of definitional equality. Agda's conversion rules and GHC's phases
  are the two models.
- **Cache selectively.** Closed normal forms, canonical instance goals and
  imported types yes; open terms only with the local context, transparency
  policy and metavariable state in the key. Measure the validation cost
  against the hit rate; a cheap query is not worth caching.

Our state: the unifier is cheap to snapshot now, and `zonk_in`, `occurs`
and `gen_ty` resolve by reference, which removed most copying. Interning
`Ty` is the next step and it is measurable: `Ty::clone` and its drop glue
were 1,700 of 20,000 samples in the seed profile before the clone removal,
and string comparison is 14% of the Cinch compiler's profile now, much of
it symbol comparison that interning would make a pointer compare.

## 5. Proof search as replay (Rocq, Lean, F*, Dafny)

- **Decision procedures first.** Syntactic equality, constructor
  disjointness, bounds, linear arithmetic, bit vectors, each with a fast
  path that emits compact evidence the kernel checks. Lean's `bv_decide`
  and `bv_check` are the model: search produces a certificate, the checker
  consumes it.
- **Cache certificates by canonical obligation.** Canonicalize binders so
  equivalent obligations share evidence across call sites and modules. A
  remote cache may store evidence, but admission always replays it
  locally.
- **Hints, not evidence.** Save unsat cores and fuel settings, retry the
  small context first, fall back to the full context (F*).
- **Budgets and isolation.** Deterministic work budgets per obligation,
  reported with the definition that spent them (Lean heartbeats, Dafny
  resource units).
- **Rewrite schemas proved once.** Check `map f (map g) = map (f . g)` once,
  instantiate per match, discharge only local side conditions, and keep the
  original program when a side condition cannot be proved inside budget.
- **Target-independent evidence.** A stream-fusion theorem is shared across
  targets; a SIMD lane-layout or FFI ABI fact carries its target
  assumptions. The certificate cache keys on the assumptions, so a target
  switch reuses everything that was target-independent.

This is where Cinch's design can beat the general-purpose compilers: a
proof certificate for a refinement or index obligation is reusable
evidence, and our pinning model (library object plus interfaces) already
gives it a stable identity.

## 6. Optimizer budgets (GHC, MLIR, Cranelift)

- **A development codegen tier.** A direct emitter or Cranelift behind the
  same checked Core, with all safety checks preserved, so edit-compile-run
  does not pay for `-O2` LLVM. Cargo's Cranelift backend, V8's Liftoff and
  Zig's self-hosted x86 backend are the references. Our `--opt-level` flag
  already selects `-O0` to `-O3` for clang; a second backend is a larger
  commitment and should wait until the frontend and emitter are the
  measured bottleneck rather than clang.
- **Erase proofs before runtime optimization**, so an irrelevant proof edit
  leaves the object cache untouched. Our Core already separates evidence
  from runtime code (`@proof` binders), which is the prerequisite.
- **Share generic code by representation.** One version per representation
  class with dictionaries passed for the operations, specialised
  selectively. Go's shape stencilling is the reference; our `Class`
  (`Raw`/`Obj`/`TObj`) is already the shape key.
- **Summarize globally, transform locally** (Go export data, ThinLTO): small
  summaries of effects, escape, call edges and inlining candidates decide
  what to import; codegen units cache independently.
- **Hard growth limits** (Flambda 2, MLIR pass manager, GHC phases): inline
  by code growth and depth, drive simplification from a worklist of changed
  nodes, index rewrites by left-hand-side head, and make the fusion window
  explicit.
- **Equality saturation on islands** (egg, Souper): a budgeted e-graph over
  a pure arithmetic or stream fragment, extraction of the best candidate,
  and any expensive synthesis moved offline into a validated rule pack.

## 7. Frontend mechanics (Clang, Oxc, tree-sitter)

- Byte-oriented lexer with a classification table and tight loops; tokens as
  kind plus byte offsets; line and column from a newline index.
- SIMD or `memchr` scans to skip to the next quote, backslash, newline or
  comment delimiter, with scalar handling at the boundary.
- Arena allocation per file or revision, packed arrays with integer IDs,
  lexemes as source slices, names interned when resolution needs them.
- Fixity decoupled from parsing: parse infix chains flat, associate once
  scoped fixities are known, which also lets an imported fixity change
  revisit chains without reparsing.
- Incremental relexing from checkpoints and subtree reuse (tree-sitter's
  model) for the editor path.

Our lexer is hand-written Rust (`compiler/src/lexer.rs`) and already
byte-oriented; the Cinch-side parser is a port of it and is now the
second-largest phase in its self-compile (under 11 s, down from 68 s after
the fixity cache). SIMD scanning is a micro-optimization to schedule after
the query graph, not before.

## 8. Code generation and linking (LLVM MC, mold, LLD, Zig)

- Emit objects directly instead of text IR plus a clang subprocess; size
  the win first by timing `clang` on our own module (we already do: 15 s at
  `-O0`, 95 s at `-O2` for 218 MB).
- Schedule per function or per stable shard, start unusually large
  functions first, and deduplicate specialisations by canonical key before
  they enter the queue.
- Treat debug information as its own workload: line tables and backtraces by
  default, full variable and type information on request, split where the
  target allows.
- Link with a fast linker where available (mold and LLD on ELF); keep the
  hot-swap path's incremental link as a separate, later project.

The empirical fact that shapes this section: for our compiler, clang at
`-O2` is 95 s of a 110 s Stage1 build. Any frontend work is noise beside
that until either the module shrinks further or the backend gets cheaper.
Both are in the backlog: shrinking IR (section 10) and the development
codegen tier (section 6).

### Tried and measured: PGO, LTO, and the compiler's own build

Profile-guided optimization of a stage compiler works and is cheap to wire:
link the compiler IR with `-fprofile-generate`, train it on a real workload
(a 4K-line bundle check is enough), merge with `llvm-profdata`, and relink
with `-fprofile-use`. On the toolkit check the same IR ran in a median of
13.7 s against 15.3 s plain `-O2`, about 10%, with identical output. The
profile is keyed to the compiler source, so it must be regenerated when the
sources change. One profile serves every stage that links the same IR:
Stage2 and Stage3 link byte-identical modules, so they share a profile.
Stage1 is the exception this note first got wrong: its module is emitted by
the Rust seed, so a profile trained on Cinch-emitted IR matches almost none
of it, and Stage1 trains its own. Link cost is one extra instrumented build
per source revision.

ThinLTO across the compiler module, the library object and the runtime was
parked by a run-time fault in the stack probe (`ldr xzr, [sp]`) inside an
inlined caller. That was attributed to the AArch64 `probe-stack`
interaction with the apply stub's dynamic alloca, the only dynamic `alloca`
the emitted module contained. Two later changes removed the cause:
`a9a9a638` addressed static blocks by a constant offset instead of a linker
alias, which is the shape of a release walk storing into the text page, and
`618f276a` replaced the stub with fixed-frame entries, which takes the
stage-3 module's dynamic allocas from 196 to zero. The fault does not
reproduce now, sharded or not, at Stage 1 or at Stage 3.

The cost is module size. One fixed-frame entry per argument count turns 196
stubs into 1,365 functions (196 dispatchers, 973 per-arity entries, 196
`many` bodies) and grows the stage-3 module from 40.4 MB to 46.2 MB (+14%),
which the sharded `-O2` stage link absorbs.

### Link-time optimization: measured, 2026-11-11

The claim above, that LTO "halves the binary and removes the call cost to
the library's string and `Nat` routines, 14% of the compiler's profile",
does not survive contact with the link inputs. LTO can only inline what it
is given as bitcode, and the library object and the runtime archive are not
bitcode. `file` reports `Mach-O 64-bit object arm64` for the pinned library
object; every member object of the runtime archive is `Mach-O 64-bit object
arm64` and none has an `__LLVM,__bitcode` section. The runtime is a cargo
staticlib built with `-C codegen-units=1` and no `-C linker-plugin-lto`,
and the library object comes from the pinned producer's `emit-object`,
which writes machine code directly. The producer retains its `library.ll`
beside the object, so an LTO link over IR inputs is possible as a separate
change, but the stage link is defined to consume `library.o` and the
runtime archive by their pinned identities.

What LTO would have to remove is the cross-object call population. In the
stage-3 module 25,390 of 203,698 call sites name a function that is
declared but not defined in the module: `cinch_alloc_ctor` 15,329,
`cinch_alloc_shaped` 1,134, `memcpy` 777, then the `Data.*` wrappers
(`Eq_I64.==` 728, `string-compare` 653, `Eq_Nat.==` 599, `Bool.not` 416,
`Nat.Add` 340). Every one resolves in the library object or the runtime
archive, so none can be inlined at link time.

Five links of the same stage-3 IR, from the same content-addressed path,
with the same profile, six build slots and `-j 3` (the default on this
18-core machine, so the current link is three shards):

| link | modules | link s | file MB | `__text` MB |
| --- | --- | --- | --- | --- |
| `-O2` objects, three shards (current) | 3 | 24.6 | 19.70 | 14.42 |
| `-O2` objects, one module | 1 | 70.0 | 20.13 | 13.17 |
| `-flto=thin`, one module | 1 | 88.4 | 13.26 | 11.50 |
| `-flto`, one module | 1 | 73.6 | 13.23 | 11.47 |
| `-flto=thin` bitcode shards, thin final link | 3 | 32.7 | 13.18 | 11.37 |

Six interleaved pairs on the toolkit check and four on the 40K-line
self-check, CPU seconds and peak resident set:

| link | toolkit median / best | self-check median / best | peak RSS |
| --- | --- | --- | --- |
| `-O2`, three shards | 6.13 / 5.95 | 62.80 / 57.29 | 37 MB / 175 MB |
| `-O2`, one module | 6.16 / 5.94 | 59.52 / 55.81 | 37 MB / 175 MB |
| `-flto=thin`, one module | 6.27 / 6.03 | 62.70 / 58.64 | 37 MB / 175 MB |
| `-flto`, one module | 6.66 / 6.50 | 66.43 / 59.41 | 38 MB / 176 MB |
| `-flto=thin`, three bitcode shards | 6.72 / 6.62 | 65.50 / 64.28 | 39 MB / 176 MB |

ThinLTO is a wash: 1.5% more CPU on the toolkit and within 2% of the
current link on the self-check, with pairwise ranges from 5.2 s faster to
3.2 s slower, at 26% to 33% more link time. Full LTO and the sharded thin
link cost 4% to 9% of CPU. All five run the toolkit
check and the self-check with exit 0, print `cinch-leak: live=0 immortal=0
leaked=0` under `CINCH_LEAK_CHECK=1`, and emit the same `compiler.ll`
(sha256 `985757f875ccd776...`) for a frozen toolkit source, so the
differences are layout, not semantics.

The size drop is real but does not reach the process. `__text` shrinks 13%
and the export table loses about 5 MB as LTO internalises definitions and
drops the ones nothing calls, while the self-check's peak resident set stays
at 175 to 176 MB in every arm. Stage 1 tells the same story larger, because the seed's module
is 335 MB and carries every prelude body: with the same profile and three
shards, the current link is 89.9 s and 43.13 MB, `-flto=thin` 102.7 s and
22.51 MB, and `-flto` 126.6 s and 22.22 MB. Both LTO binaries pass the
toolkit check, the self-check and the leak gate and emit the baseline's
`compiler.ll`, which is the direct test of the parked fault.

Dropping to one module without LTO is the only shape that is faster on a
check: 5.0% less median CPU on the self-check, because cross-shard calls
can inline, and no change on the toolkit. It triples the stage link's wall
time, 24.6 s to 70.0 s, so the sharded default stays; `-j 1` remains the
documented way to get it, and the job count is part of the stage's inputs.

Nothing was wired. The stage link keeps its plain `-O2` (with the profile
when PGO is on), and `tooling/bootstrap.py` records the measurement at the
link helper. The chain cost that was avoided: a clean chain built in 907.8 s
with six slots and `-j 3`; ThinLTO would add 12.9 s to Stage 1's link and
about 8 s to each later stage link, roughly 3% of a chain, for no check-time
gain.

One measurement the claim deserved and did not get: the interface
unfoldings (`8f123c1f`) already removed 2,833 wrapper calls at the Core
level, and the call census above is what remains. Turning that artifact off
is a compiler change rather than a link flag, so the with-artifact number
is the one reported; it removes exactly the calls LTO would otherwise
inline, which is a second reason the link-time lever is smaller than this
section first estimated.

### Profile-guided stage links (implemented 2026-11-10)

The pipeline above is now the default stage link, with `--no-pgo` (or
`CINCH_BOOTSTRAP_PGO=0`) as the documented off switch. Per stage: link the
emitted IR with `-fprofile-generate`, run the instrumented compiler over the
toolkit bundle, merge with `llvm-profdata`, relink with `-fprofile-use`, and
install only the profile-use binary. The instrumented link and its counters
are training state in the stage's `pgo/` directory, removed before the stage
is recorded. `tooling/README.md` documents the mechanics in full.

Flag plumbing had to reach two different link sites. Stages 2 and 3 are
linked by `bootstrap.py`, so the flags go straight to clang, or to each shard
above one job. Stage1 is linked inside the pinned Rust seed, which builds its
own command line, so `tooling/shard_ir.py` gained `CINCH_CLANG_APPEND` (flags
for the shard compiles and the final link) and `CINCH_CLANG_RECORD` (the
seed's real argv, as JSON lines). The profile-use relink replays that
recorded argv with only the profile flag and the output path changed, so the
seed's own link options stay authoritative.

The cache key is what a profile depends on: the compiler source digest, the
workload digest, the `llvm-profdata` version, and the digest of the IR it was
trained on. A record that no longer matches is retrained, never reused, and
the profile's own digest enters the stage inputs, so a stage cached against
another profile is rebuilt. Two profiles exist per source revision: `seed`
for Stage1 and `cinch` for Stage2 and Stage3, and Stage3 reuses what Stage2's
instrumented link trained.

Two findings from wiring it are load bearing:

- **One profile cannot serve Stage1.** This note assumed every stage is the
  same program, so one profile would cover all of them with the functions
  that differ skipped. In practice Stage1's module is emitted by the Rust
  seed and differs structurally from the Cinch-emitted module of the later
  stages (224 MB against 48 MB for the same source program), so a
  Cinch-trained profile matches almost none of it and Stage1 trains its own.
  That is why the cache is keyed by family as well as by source digest.
- **A profile-use compile depends on the module's path.** Identical shard
  text compiled from two directories produced objects that differ by 38%: a
  profile's records are matched per module, and a module compiled from a
  different path matches differently. That broke the fixed point on the first
  attempt, because Stage2 and Stage3 linked the same IR from their own
  directories and produced different binaries. Every stage now links its
  module from one content-addressed path,
  `build/bootstrap/shards/<ir digest>/`, and the wrapper writes its shards to
  a directory named after the IR instead of a temporary one, so
  instrumentation and profile use see identical paths.

Profile data moves basic-block weights and inline costs only, so the emitted
IR, every `.cinchif` and every build manifest are byte identical to a
`--no-pgo` chain, and Stage2 and Stage3 still meet byte for byte; only the
executables differ. The two chains' Stage3 compilers, five interleaved runs
each on the toolkit check, measure 22.53 s median wall and 20.07 s CPU
without a profile against 20.62 s and 18.06 s with one (8.5% wall, 10.0%
CPU; the machine was fully loaded, and an earlier quieter run of the same
pair gave 15.47 s against 13.96 s). The price is one instrumented link plus
one training run per source revision (26 s and 22 s at `-j 9` for the Cinch
family, 20 s of training for the seed family). The profile files are 5.5 MB
(`cinch`) and 9.9 MB (`seed`). `__text` is 12.08 MB to 11.91 MB; the
executable file grows from 15.98 MB to 20.57 MB because cold call sites stay
out of line and our mangled names are long, so the symbol table is 33k
entries larger.

The chain itself, measured stage by stage. The `--no-pgo` chain was cold
(seed and library producer built inside it): 242.0 s, 502.8 s and 323.1 s for
Stages 1 to 3. The profile chain, with the seed and producer already built
from an earlier attempt, spent 123.5 s on Stage1 (of which 19.8 s is
training and 26.3 s the instrumented link), 266.4 s on Stage2 (one
instrumented link, one training run of 22.3 s, then the profile-use link)
and 247.9 s on Stage3, which is a frontend plus one profile-use link because
it reuses Stage2's profile. Stage3 shrinking from 323.1 s to 247.9 s is the
profile paying for itself on the chain's largest single compile; the cold
seed and producer builds mean the two totals are not directly comparable,
and the marginal cost at the top of a chain is one instrumented link plus
one training run per stage.

### Checking that the profile changes only the binary (2026-11-11)

A stage binary that rejects a program its plain sibling accepts would be
worse than the 10% the profile buys, so the divergence was looked for
directly. One Stage2 module, linked five ways and run on the 3,966-line
toolkit workload: `-O0`, `-O1`, `-O2`, `-O3` and `-fprofile-generate` all
exit 0 with an empty diagnostics file, so neither the optimization level nor
instrumentation changes what the checker decides. A `-fsanitize=undefined`
link of the same module runs the workload with no report, which is the
weakest of these checks but the only one that looks at undefined behaviour
rather than at agreement.

The profile-use Stage3 and a plain `-O2` link of the same Stage3 IR then
agree over a 30-case corpus (the 12 example programs, the fib and classes
smokes, the toolkit bundle and the compiler bundle itself), in check mode and
in `--emit-ir` mode: exit code, stdout, stderr, the diagnostics file and the
emitted IR are equal in every case. One frozen source of the compiler emitted
twice through each binary gives the same `compiler.ll` four times over.

Two things to know when repeating this. The corpus has to be frozen: an
earlier run of the same comparison reported a difference that turned out to
be the lane's stage 1 bundle being rewritten by another build between the two
arms, so the arms had read different sources. And a profile-use compile of a
module depends on the path it is given (section 8), so the arms must be
handed one shared path or their objects differ for that reason alone.

### Alternative allocators: measured, no win (2026-11-10)

Linking the stage compilers against jemalloc or mimalloc is the obvious next
guess, and the measurement says not to. Interleaved five-run medians on the
toolkit check: plain 16.6 s, jemalloc 17.1 s, mimalloc 18.3 s, with a spread
of more than 4 s inside each arm, so the difference is noise.

The reason is structural, and the compiler's own accounting says it: with
`CINCH_ALLOC_STATS=1` the toolkit check performs 464.1M allocations, of which
444.8M (96%) are Cinch value blocks served by the runtime's per-thread
size-class heap, and only 19.3M reach the system allocator. By bytes it is
10.97 GB total against 10.70 GB of value blocks, so the system allocator sees
2.5% of the traffic. An alternative malloc cannot help a workload whose
allocator is already ours.

The number that matters is the volume itself: 464M allocations and 11 GB of
churn for a 15 s check of 4,000 lines. The lever is fewer allocations, not a
better allocator: interning names and types, avoiding transient lists in the
checker, and reusing buffers in the parser and printer. That is section 6's
work, and the allocator question is closed until the volume drops.

### Sharded code generation (implemented 2026-11-10)

`tooling/shard_ir.py` cuts the stage module into K shards and
`tooling/bootstrap.py` compiles them in parallel under one `-j` budget
(default half the cores, `-j` or `CINCH_BOOTSTRAP_JOBS`; `-j 1` keeps the
single translation unit). Functions are grouped by the module prefix in
their symbol, groups are balanced by IR line count into the least loaded
shard in sorted name order, and a group larger than its fair share is cut
at function boundaries. Cross-shard references become `declare`s with the
same signature, and `internal` definitions are copied into the shards that
use them. The split and the link order depend only on the IR text and K,
so Stage2 and Stage3 still meet byte for byte.

Stage1's module is 224,541,238 bytes. Link phase alone, `-O2`, on the
development machine (Apple clang 21, arm64). The split is the serial Python
pass that writes the shards and runs before the budget starts:

| K | split | clang wall | clang CPU | speedup | `cinch` bytes |
| --- | --- | --- | --- | --- | --- |
| 1 | 0 | 128.9 s | 126.4 s | 1.0x | 28,450,024 |
| 2 | 6.3 s | 67.5 s | 126.9 s | 1.9x | 28,064,296 |
| 4 | 7.0 s | 34.4 s | 125.1 s | 3.7x | 27,885,416 |
| 8 | 5.3 s | 20.1 s | 132.3 s | 6.4x | 27,871,432 |

A full `build --stage 1` from a clean stage1 output adds the seed's ~20 s
frontend producing that module:

| K | wall | CPU | peak RSS | notes |
| --- | --- | --- | --- | --- |
| 1 | 154.5 s | 145.3 s | 8.09 GB | one translation unit |
| 2 | 92.2 s | 149.0 s | 4.13 GB | |
| 4 | 67.1 s | 157.0 s | 2.25 GB | |
| 8 | 58.7 s | 189.8 s | 2.25 GB | 2.6x wall, 2.0x CPU of baseline |

The parallel phase's CPU rises because shards re-parse their headers and
metadata, copied `internal` definitions are compiled more than once, and
the eight clang processes contend for memory bandwidth. Peak memory falls
sharply because no single process holds the whole module.

Stage2's module is 48,541,467 bytes. Its frontend (Stage1 compiling
Stage2) is 200.7 s wall and 195.9 s CPU on one thread, so the link is now
the small part of the build:

| K | link wall | link CPU | stage2 build wall (frontend + link) | `cinch` bytes |
| --- | --- | --- | --- | --- |
| 1 | 45.1 s | 44.3 s | 245.8 s | 15,833,032 |
| 2 | 25.2 s | 45.7 s | 225.9 s | 15,833,912 |
| 4 | 15.4 s | 50.2 s | 216.1 s | 15,833,832 |
| 8 | 9.7 s | 55.6 s | 210.5 s | 15,833,736 |

What the split costs at run time: the same Stage2 compiler binary built at
K=1 and K=8 checked the 4K-line toolkit bundle (`--check` with the base
interfaces, median of three runs) in 13.70 s wall and 13.33 s CPU at K=1
and 13.20 s wall and 12.85 s CPU at K=8. Cross-shard inlining loss is not
measurable on that workload, and the compiler binary grows by 704 bytes
(0.004%). K=4 and K=8 are the knee: past 8 the split and the duplicate
compilation dominate, and the frontend remains the critical path.

Determinism evidence: re-splitting the retained stage2 `compiler.ll` at
K=8, recompiling the shards and relinking reproduced the stage2 compiler
byte for byte (sha256 `3eb77b89…`); stage2 and stage3 then matched on all
46 emitted interfaces, on `compiler.ll`, and on `cinch`. `tooling/bootstrap.py
check -j 8` and `smoke --stage 3` pass.

The consequence for the rest of this note is that the backend is no longer
the wall clock: Stage2's 245.8 s build spends 200.7 s in the single-threaded
frontend. The ordered work is now section 5 (query reuse), section 7
(frontend mechanics) and section 9 (parallel phases), with the remaining
backend items (direct object emission, ThinLTO to recover cross-shard
inlining) worth less than the split already bought.

## 9. Scheduling `-j N` (Make jobserver, Ninja pools, Bazel workers)

- One budget across packages, compiler threads, solver jobs and the linker.
- Schedule a graph of ready jobs; park a query waiting on a dependency and
  return the worker to the queue rather than blocking it.
- Prioritize work that unblocks the most dependents; use prior timings to
  estimate job size; batch tiny jobs.
- Budget memory as well as CPU (Ninja pools): parser jobs are cheap, LLVM
  jobs are not.
- Deterministic output regardless of schedule: fixed commit order for
  diagnostics and artifacts.

Current state: `tooling/run_stage_targets.py --jobs` is the only
concurrency we have, and the runtime pool (`cinch_par_*`, phase 1 landed)
is the substrate for a parallel compiler (see
[parallel evaluation](parallel-evaluation.md)). Section 11's corpus is what
will tell us whether the compiler's phases will actually fill workers.

## 10. What GHC's optimiser offers a strict language

GHC's middle end is the best documented body of work on turning
functional Core into good code, and several of its transformations apply
directly to Cinch's Core. Each entry below says what it does, whether it
applies to us, and the first experiment. The pass list is from GHC's user
guide (`-O` implies the first group, `-O2` adds liberate-case); the
descriptions are from the guide and the handbook chapter on the simplifier.

Adopt:

- **case-of-case with join points.** Push an outer `case` into the
  alternatives of an inner `case` on the same scrutinee, so that
  intermediate values are not built and immediately taken apart. This is
  what makes `&&` chains and fused pipelines compile to straight-line
  tests. It duplicates the outer alternatives, so GHC pairs it with join
  points (see below). For Cinch this is the most valuable single
  transformation we do not have: our match compilation produces nested
  cases, and every `Option`/`Result` chain in the compiler's own source
  pays for the intermediate block.
- **Occurrence analysis and join points.** Count how each binder is used
  (once, many times, under a lambda, in tail position) before simplifying.
  A let-bound function that is only ever tail-called becomes a label
  instead of a closure; that is what keeps case-of-case from duplicating a
  closure allocation. Our compiler already marks tail calls
  (`@tailrec`, `tailcall`) and the match compiler shares one wildcard
  alternative, so a join point node in Core is the natural next step, and
  the Core output design already has a syntax for it.
- **Loop breakers.** Break recursive groups into strongly connected
  components and pick a breaker, so an inliner cannot loop. Any rule-based
  simplifier needs this on day one.
- **`-fcase-merge`.** Merge immediately nested cases on the same variable
  (`case x of Red -> e1; _ -> case x of Blue -> e2; Green -> e3` becomes one
  case). Cheap, and it removes the shape our fallthrough-row sharing
  produces.
- **`-fcase-folding`.** Fold cases whose scrutinee is a known primitive
  operation with constant alternatives. Applies to our machine prims
  (`eqI64`, wrapping arithmetic) and to `Nat` comparisons on immediates.
- **`-fcall-arity` and eta-expansion/reduction.** Give a function as many
  arguments as its definition takes before a call is made, so a partial
  application does not allocate a closure per call site. Directly
  applicable: our closures carry `app` stubs and arity, and the runtime
  already has the machinery.
- **`-fcpr-anal` plus worker/wrapper.** Constructed-product return analysis
  finds functions that always return a constructor around known fields;
  worker/wrapper splits the function into a wrapper with the source type
  and a worker returning unboxed values, and the wrapper inlines away. This
  is how a strict `Int` accumulator loop allocates nothing in Haskell. For
  Cinch the analogue is `Nat`/`Int` immediates and single-field
  constructors (`NonZero`, `InRange`, `Sym`, `String`) reaching code
  without a block: our emitter already has immediate words, so the work is
  finding the sites where a constructor call wraps a value that never
  escapes.
- **`-fspec-constr` (call-pattern specialisation).** Specialise a recursive
  function for the argument shapes it is called with, which removes
  repeated case analysis. In Cinch this targets the interpreter-shaped code
  in the compiler itself (`check-inner`, `lower-expr`), where the same
  `Expr` constructor tags are examined at every recursion.
- **`-fliberate-case`.** Unroll a recursive function once in its own
  right-hand side when a free variable is repeatedly case-analysed. The
  same target as spec-constr, for free variables rather than arguments.
- **`-ffloat-in` and `-ffull-laziness` (adapted).** Move a `let` to where it
  is used so a branch that never runs never allocates, or float a binding
  out so several allocations batch. In a strict language the first form is
  a clear win; the second form ("full laziness") has no thunk to share, so
  only the allocation-batching part transfers.
- **`-fstatic-argument-transformation`.** Turn a recursive function's
  invariant arguments into free variables of an inner loop, which removes
  argument shuffling. Applies to any accumulator that never changes.
- **`-floopification`.** Turn saturated self-recursive tail calls into
  local jumps. We emit tail calls; whether the backend turns them into
  jumps is measurable in the emitted assembly.
- **`-fexitification`.** Float exit paths out of a recursive function, so
  the caller can see a returning value as a known constructor. Pairs with
  CPR.
- **`-fstg-cse` and `-fcse`.** Common subexpression elimination at the Core
  and STG levels. Our `share_repeated_subterms` already shares structurally
  equal subterms; CSE generalizes it to repeated computations inside one
  alternative.
- **`-fdo-eta-reduction`, `-fdo-lambda-eta-expansion`.** Cheapen calls
  around closures. Small, safe, measurable.
- **Rules and specialisation of dictionary-heavy code.** GHC's `RULES` are
  library-authored rewrites applied by the simplifier; specialisation
  clones a polymorphic function at a known dictionary so an indirect call
  becomes direct. Our static dictionaries removed the allocation, but the
  call still goes through `__apply`: turning a projection from a known
  static dictionary into a direct call is the same optimization, and it is
  already on the "next levers" list from the reference-count work.
- **Interface unfoldings and loop-breaker guidance.** GHC records, per
  binder, whether its unfolding is worth inlining, a size budget, and
  whether it is a loop breaker. Our `.cinchif` carries contracts; adding a
  small optimization section (size, inline guidance, arity) is a compatible
  schema addition and needed before cross-module inlining.

Reject or defer:

- **Strictness and demand analysis as such.** Cinch is strict, so
  evaluation order is known. The useful residue is the unboxing part
  (worker/wrapper), which is listed above.
- **Laziness-specific machinery** (thunk updates, blackholing, `-feager-blackholing`).
  We have explicit `Later`/`Thunk` forms; the compiler should not acquire a
  demand analysis for them beyond what the guardedness rules require.
- **`-fno-state-hack`, `-fomit-yields`**: runtime-system specific.

One structural lesson from GHC's implementation is worth copying before any
of the individual passes: **the simplifier runs several times with analyses
interleaved, each pass a set of local rewrites applied until nothing
changes, with a tick budget.** That shape is what makes the individual
transformations composable, and it needs two things we do not have yet: a
Core-level simplifier and a per-pass budget with a reported tick count. Both
are cheap to add once Core output exists, because the same printer can show
each iteration's result.

### 10.2 The inlining decision kernel (2026-11-11)

The summary above described GHC's inliner from the user guide. Reading the
implementation (`GHC/Core/Unfold.hs`, `GHC/Core/Unfold/Make.hs`,
`GHC/Core/Opt/Simplify/Inline.hs`, `GHC/Core/Opt/Simplify/Utils.hs`,
`GHC/Core/Opt/OccurAnal.hs`) corrects three things the summary implied:

- The thresholds are 750 for creation and 90 for use, on a scale of tenths
  of a node (`Unfold.hs:94-127`), and **the call's cost is subtracted before
  either comparison** (`computeDiscount`, `Inline.hs:709-751`). There is no
  separate growth allowance any more; the discounts replaced `ug_grow`.
- A `DFunUnfolding` is never unfolded at a call site ("Never unfold a DFun",
  `Inline.hs:117`). Its certainty is narrower: it stops a worker/wrapper
  split and it counts as an interesting argument. The certain-at-a-site
  class is `UnfWhen` plus `CompulsorySrc`.
- Occurrence analysis is size-blind (`OccurAnal`), and a binder read once
  outside a lambda is substituted **regardless of the right hand side's
  size** (`Utils.hs:1524-1528`). That, not a size rule, is what makes chains
  of one-shot helpers disappear.

`design/inline-unfolding-policy.md` section 9 maps every input of that
decision onto this compiler, present or missing with the reason, and section
10 states the size model we adopted on GHC's scale. The round that mapping
motivated added the occurrence rules, interesting arguments, the two
thresholds, and the one change with the largest measured effect, which is
not in the inliner at all: a zero-field constructor is now emitted as its
static block instead of a call to its unit, which removed 65,759 calls from
the compiler's own module (section 13 of that note).

### 10.1 What landed (2026-11-10)

`compiler/src/simplify.rs` is the pass shape above, on the specialized
program and before reference counting, so `crate::rc` balances the
simplified bodies. A pass walks every unit bottom up and applies local
rewrites; a tick is one rewrite application; passes repeat while the
previous one fired, to `--max-simplifier-iterations` passes (default 4,
zero disables the pass). The report records the cap, the passes actually
run, the ticks, and the count per rule, in the artifact manifest under
`simplifier`, and the lowered dump prints `@ticks <n>` in its header.
(`cinch core emit` takes the same two flags as `build`.)

Three rules, each with the preconditions it assumes:

- **case-merge** (`case-merge`): an outer decision's alternative whose body
  decides the same local again merges into one decision. It covers the
  shape the match compiler produces for a source wildcard
  (`case c of Red -> 1; _ -> case c of Blue -> 4; Green -> 3; Red -> 2`) and
  the shared-default expansion with several alternatives per default, which
  the driving pass's weighted merge refuses. It requires unweighted
  decisions (a weighted merge rescales the weight spaces, and the driving
  pass owns that rule), constructor-tag alternatives on both sides, defaults
  after every explicit alternative, and no pattern binder a default
  introduced that the kept alternatives read. Inner alternatives whose tag
  an explicit outer alternative already names are dead under the default and
  are dropped.
- **known-constructor** (`known-constructor`): a decision whose scrutinee is
  a local a `let` bound to a constructor application (through the alias
  `let`s lowering adds) reduces to the tag's alternative with the field
  binders substituted. It requires every argument of the constructor
  application to be a value (local, literal, unit or primitive reference):
  only then can an argument move past the branch's effects and be
  substituted at every use of its field binder without changing what the
  program does. A body carrying a quotation keeps its decision, because a
  quotation's retained body is a separate binding scope.
- **direct-dict-call** (`direct-dict-call`, `--no-direct-dict-calls` turns it
  off): `dictproj (instdict I) idx` applied to exactly the method unit's
  arity calls the method unit directly. It requires a static instance (no
  subdictionaries, a known nonzero method arity), which is what code
  generation's `static_dict` requires, so the projection yielded a shared
  closure with no environment and the application becomes the call.

A pass also drops a `let` whose binder the body no longer reads when its
value is a value, which removes the constructor block a
known-constructor reduction orphaned.

Measured on the self-build corpus, the pass converges before the default
cap: on `compiler/tests/core/ownership.cinch` it runs two passes with 160
ticks (174 once `case-of-case` runs); the distribution is in the report
and in section 11's harness.

### 10.2 Join points and case-of-case (2026-11-10)

Core gained the join node the previous slice deferred, and with it the
transformation that needs it.

**The node.** `Join { id, params, body, rest }` binds a code block inside
the enclosing body: `rest` is the continuation, and `Jump { id, args }`
transfers to it. `JoinId` is its own identity, disjoint from locals, so a
join is never a value. The invariants, stated on the node and checked by
the Core text verifier:

- a jump names exactly one enclosing `Join` of its id, and sits where its
  value is that join's result (a tail position of the function or of an
  enclosing join's body; under destination passing the same rule covers a
  join in a value position, whose jumps deliver to the slot the enclosing
  form stores into);
- a jump never crosses a lambda or a quotation's retained body, both
  being separate code units;
- the parameters are owned exactly as a unit's parameters are: a jump
  hands one reference per parameter over, and the body consumes each
  exactly once on every path, or drops it.

Every consumer handles the pair. `crate::rc` inserts the count operations
(a jump's arguments are owned uses, the body's parameters owned binders)
and its verifier re-derives them, including "the body leaves the
enclosing holdings as it found them". Code generation emits a join as one
basic block and a jump as a branch to it: no closure, no frame, no call.
The printer writes `(join j0 (params (t1 @class(..))) body rest)` and
`(jump j0 args)`, and the verifier checks the three rules above plus the
argument count.

`case-of-case` pushes an outer decision into the alternatives of the
decision it scrutinizes: `case (case s of A -> eA; B -> eB) of alts2`
becomes `case s of A -> case eA of alts2; ...`. It also takes the shape
our lowering actually produces, `let t = case s of ... in case t of ...`,
when `t` is read exactly once, including the alias `let`s that open the
value. It refuses a weighted inner decision, a quotation in either
decision's alternatives, a scrutinee read inside either decision's
alternatives, a computed scrutinee, and a body that already carries a
join. Because it duplicates the outer alternatives, an alternative larger
than `join_threshold` (20 nodes) is lifted into a join and each copy
becomes a jump; smaller ones are copied with fresh binders and reported
as `duplicated`.

Two limits are part of the design, not oversights:

- **A lifted body's parameters are its own pattern binders.** The obvious
  generalisation, "over all its free variables", needs a release point
  inside the join body, because no enclosing code runs after a jump; a
  join body that carries a jump as its value has nowhere to put those
  releases within this Core. So a body that reads an enclosing local is
  copied when it is small and makes the rule refuse when it is not.
- **The decision a push creates is folded only when its scrutinee is a
  constructor application of values.** `known-constructor` now fires on
  that shape directly (not only through a `let`), which is what collapses
  `case (Some x) of ...`; a pushed decision on a computed field keeps its
  decision.

**Measured on the compiler's own bundle** (46 modules, 2.6 MB of source;
Core from `cinch core emit`, wall from the same command):

| configuration | Core lines | Core nodes | units | joins / jumps | largest unit | wall |
| --- | --- | --- | --- | --- | --- | --- |
| before this slice | 1,452,387 | 3,220,635 | 6,472 | 0 / 0 | 18,321 | 3 m 07 s |
| shipped default (fold, rule off) | 1,452,554 | 3,220,428 | 6,472 | 0 / 0 | 18,321 | 2 m 47 s |
| rule on, fold | 1,577,885 | 3,539,984 | 6,478 | 81 / 801 | 18,325 | 2 m 45 s |
| rule on, before the fold | 2,237,111 | 4,661,747 | 6,450 | 1,647 / 11,893 | 58,353 | 3 m 26 s |

The fold is what makes the rule usable at all: without it two units reach
58K and 53K lines from duplicated alternatives and Core grows by 45 %.
With it the largest unit is unchanged and the growth is 9.9 % of nodes,
which is still more than the rule earns, so **`case-of-case` ships off by
default** (`--case-of-case` turns it on). The join node, its
introduction, its printing and its verification stay on: they cost
nothing when no join exists, and `compiler/tests/core/joins.cinch` pins
them (a join with two jumps) through the flag.

The next lever was the missing half of the fold: a decision on a
constructor application whose arguments are *computed*. That half is now
written, sound, measured, and **off by default**, because of what it does
to the inliner rather than to itself. Section 10.6 has the rule and the
numbers.

### 10.6 The computed-argument fold, and the inliner it feeds (2026-11-13)

The join node section 10.2 describes is in Core (`Core::Join`/`Core::Jump`,
`compiler/src/core.rs:1885-1925`) with `case-of-case` built on it, so the
backlog item's first blocker is gone. What remained was the fold's second
half: a decision on `Ctor a1 a2` where the arguments are *computed*.

**The rule, and why it is not a substitution.** The obvious form substitutes
each field binder by its argument, which is why the existing
`known-constructor` rule demands that every argument be a value: substituting
a computed argument at *n* reads runs its work *n* times, and in a strict
language it also moves that work past whatever sits between the construction
and the read. Requiring the field to be read at most once rescues the second
objection and not the first, because a read a nested decision skips still
drops the work.

So the rule does not substitute. It binds each argument to a fresh local at
the position the construction occupied, in argument order, and substitutes
the field binders by those locals:

```text
let t = Ctor a1 a2 in case t of (con k f1 f2) -> body
let x1 = a1 in let x2 = a2 in body[x1/f1, x2/f2]
```

The order is preserved by construction: the first form evaluates `a1`, then
`a2`, then allocates, then dispatches; the second evaluates `a1`, then `a2`,
then `body`. The allocation and the dispatch are what the rule deletes, and
neither is observable. A field read any number of times costs what it cost
before, because `crate::rc` counts a `let` binder and a pattern binder the
same way. That is why the rule needs no single-use condition, and the
read-order argument the earlier note asked for is discharged by not moving
anything rather than by proving a move safe.

It refuses a weighted decision, a variable or wildcard alternative (which
binds the block), a chosen alternative carrying a quotation or a count
operation, a `let`-bound form whose binder has another reader, and an
identity-bearing constructor (`MutCell`, a thunk, a memo cell).

**Population, from the checked-stage dump of the compiler's own bundle**
(`--stage=checked`, 6,345 units): 981 `let ctor-app then case binder` sites,
of which 830 have a computed argument and every one of those reads each field
at most once. The lowered stage has 541, the rest having been folded or
inlined away.

**What it does on its own, and what it does to the inliner.** Both arms from
one binary, a fresh `CINCH_STORE` each, `core emit` on
`build/bootstrap/stage1/compiler.cinch`:

| arm | Core lines | Core nodes | ticks | `read-carrier-tree` | `default-expression` |
|---|---|---|---|---|---|
| rule off (shipped default) | 1,656,750 | 1,940,774 | 68,829 | 1,519 | 1,598 |
| rule on (`--known-ctor-computed`) | 1,728,431 | 1,975,422 | 71,258 | 57,719 | 37,538 |

The fold itself is a win: 2,093 firings, and the per-application node delta
sums to **-5,420**, with no site larger after than before. The growth is all
in two units, and the mechanism is not the fold. The fold deletes the
constructor a body's decision reads, so the body stops being
decision-bearing at the inliner's site test, and the benefit-and-size model
then admits a copy it refused before: `Bootstrap.Type.read-carrier-tree`
reaches 35,662 nodes straight out of the inliner in both arms that run it.
The inliner's credit does not account for a copy whose decision the fold is
about to delete, so it prices the copy as if the branch stayed.

That is the "two rules fighting" case in its concrete form, and the choice
between the two is not a matter of taste: the inliner is the default and
worth more than 5,420 nodes, so **the fold ships off** (`--known-ctor-computed`
turns it on) until the inliner credits a fold it enables. The rule, its
counters (`simplifier.known_constructor_computed`) and its eight tests stay
in the tree, because the population is real and the rule is the half the
backlog asked for; what is missing is one term in the inliner.

**A second, smaller rule from the same census.** `proj (Ctor a1 a2) k` is
`ak` when `ak` is a value (`projection`, on by default). The census finds
1,154 projection nodes at the lowered stage and 935 at the checked stage, but
none of them is a projection of a constructor application: the shape is
reachable from source and from `case-of-case`, not present in this corpus, so
the rule costs one line and fires zero times here. It is kept for the reason
section 10.3 keeps its backend twins: the shape exists, and a probe pins it.

### 10.3 The same folds in the Cinch backend (2026-11-11)

`Bootstrap.RuntimeIr` gained the case fold on a known constructor twice:
directly (`ccf0cadf`) and through a deferred `let` binder (`2935175b`, a
`DeferredConstructor` local, admitted only when its single use is a case
scrutinee and every argument is a value). Both are inert on this corpus:
with the same input compiled by both emitters the module is identical to the
line, 15,006 constructor allocation sites and 35,504 tag tests unchanged, so
neither shape exists in the ladder's own Core, which reads a `let` binder at
more than one case scrutinee or builds it from computed arguments; they are
expected to pay only once occurrence analysis and `case-of-case` produce the
shapes they fold, so they are kept rather than widened with further special
cases.

### 10.5 Benefit-and-size inlining, and what it measured (2026-11-12)

The inliner had one decision: GHC's. A body's weighted size minus the call's
discount, compared against a fixed threshold, with the unfoldable class
bypassing the test and per-shape node caps bounding a certain copy
(section 10.2). This round adds the second decision and makes it the
default: Flambda 2's benefit against size
(`middle_end/flambda2/simplify/inlining/call_site_inlining_decision.ml`,
`terms/code_size_v1.ml`, `terms/removed_operations.ml`, and the survey in
`local/oxcaml-techniques.md` section 1.1).

**The rule.** Code size is the instruction count the generator emits for a
node, in Flambda 2's units: an allocation 5 plus one per field, a direct call
4, an indirect call 6, a C call 10, a primitive 1, a switch 5 per arm, a
projection 1, a closure 5 plus its environment, a literal or a variable 0.
The site test is `size * sites - benefit <= threshold`, with `sites` the
number of duplicated call sites, so the growth term is the body existing once
per site instead of once. A body at or below `small_function_size` (10) is
inlined everywhere with no site test, which is Flambda 2's `Always_inline`;
above `large_function_size` (50) it is not considered at all. The threshold
is 10 size units. A call site whose enclosing unit is a tail worker
(`<unit>#tail`, the shape every source loop lowers to) or a member of a
recursive group divides its allowance by 4, because the copy is entered once
per iteration and its benefit is paid once. A callee in a recursive group is
refused at every site.

**Where this model is weaker than Flambda 2, stated rather than papered
over.** Flambda 2 builds the inlined body in a scratch accumulator,
simplifies it, and takes the difference of two `Cost_metrics`, so it can
credit an allocation the copy *deletes*. This pass has no scratch
simplifier, so the benefit is read off the body and the site and credits only
what is certain: the call every site removes, the alternatives a
`known-constructor` fold deletes when the site supplies a constructor for
every parameter the body decides over, and a projection the same fold turns
into a field. An allocation is **not** credited, because nothing in this pass
deletes one: a copy moves it to the site. That is the one term the two
disagree on, and it makes this rule stricter than Flambda 2 on an
allocation-bearing body.

**The fold test is about duplication, so it does not apply at a body's only
site.** A decision-bearing body copied to one site moves its branch into the
caller and the callee is pruned: one copy of the decision either way. The
test applies from the second site on, where a non-folding copy would
duplicate the branch instead of removing it.

**Measured** on the compiler's own bundle (`build/bootstrap/stage1/compiler.cinch`,
47 sources), `emit-ir`, one binary per arm, a fresh `CINCH_STORE` per arm so
the package store cannot reproduce one arm's artifact for another:

| arm | binary | IR lines | units | call sites | allocation sites | `rc.unique` |
|---|---|---|---|---|---|---|
| inliner off (`--no-inline`) | before | 5,740,641 | 7,138 | 802,693 | 7,686 | 50,048 |
| GHC thresholds, previous default | before | 5,801,514 | 7,128 | 800,516 | 7,686 | 50,080 |
| GHC thresholds, same binary | after | 5,801,327 | 7,127 | 800,489 | 7,686 | 50,076 |
| inert control, six dead 16-byte statics | after | 5,746,754 | 7,179 | 798,339 | 7,670 | 49,463 |
| benefit and size, new default | after | 5,746,640 | 7,158 | 798,343 | 7,670 | 49,468 |

Two rows carry the result and both are `after`: the GHC-threshold arm and the
benefit arm are the same binary, the same sources and the same flags apart
from `--inline-model`, so nothing but the decision differs. Benefit against
threshold there is **54,687 fewer IR lines (-0.94%), 2,146 fewer call sites
(-0.27%), 16 fewer allocation sites, and 608 fewer `rc.unique` sites**, at 31
*more* units (a body the benefit model refuses keeps its own unit, which is
the right trade when the alternative is duplicating its branch).

The control row is the load bound, not a result. It is the benefit arm's own
binary and decision with six dead 16-byte statics added to the source, so the
114-line gap between it and the benefit arm is what a change with no
behavioural content is worth on this measurement. The benefit arm's 54,687
lines against threshold are two orders of magnitude larger than that, and the
repeated benefit emit reproduces to the line, so the line count is a
deterministic function of the decision and not of the machine.

The allocation-site count is the number to check first. Section 13 of
`design/inline-unfolding-policy.md` records that no inlining decision in the
previous round ever duplicated an allocation site, because the model prices
an allocation at six times a call and the class refuses an allocation-bearing
body outright. The benefit model has no such refusal (its allocation price is
the same 5 plus fields), so a duplicated allocation site was the failure this
rule could have introduced. It did not: the count falls by 16, and the drop
is the `known-constructor` fold deleting the block a decision's scrutinee was
built from.

**The counters**, from the artifact manifest's `inlining` block on the
benefit arm: 5,577 sites inlined; 199,614 refused, of which 130,619
`fold` (a decision-bearing body at more than one site whose scrutinee is not
a known constructor), 43,685 `budget` (the two-percent program-growth budget
is exhausted), 14,932 `size` (the growth term exceeds the allowance), and
10,378 `recursive`. Every site the sweep considered lands in exactly one
bucket, so the manifest says what the rule did rather than leaving it to be
inferred from two IR line counts. The block is written by the seed's
`write_artifacts` beside the existing `simplifier` block, so it appears in a
compile the seed performs; the self-hosted compiler writes its own manifest
from `Bootstrap.Manifest.cinch`, which carries neither block, and adding them
there is a separate change to a file this round did not touch.
`CINCH_INLINE_TRACE` prints the per candidate size, benefit, `certain` and
`folds` flags and the per-copy caller and callee.

**What the refusals say about the model.** `fold` dominating is the honest
result: the largest single class of bodies in this program is a dictionary
method that decides over a parameter, and at more than one site the benefit
term cannot pay for duplicating the branch. Flambda 2 would make the same
refusal for the same reason. `budget` at 43,685 is the growth budget doing
its job: with the benefit test admitting more small bodies than the threshold
test did, the two-percent cap binds, which is why the rule's net effect is
0.94% smaller rather than the larger figure the per-site decisions alone
would suggest. `recursive` at 10,378 is the one class the threshold model
refuses at admission rather than at the site; the benefit model admits a
recursive body to the table because its size is what a site test reads, and
refuses every one of its sites here.

**Rejected.** (a) Crediting a moved allocation as a removed one. It is what
Flambda 2 does and it is what would let the rule admit an
allocation-bearing body above the small floor; without a scratch simplifier
that proves the block dies at the site, the credit is an unproven claim, and
the census (`design/compilation-speed.md` section 2) says constructors are
85.9% of blocks, so an unproven claim there is the expensive kind. (b) A
per-site `sites` count. The growth term uses the program-wide site count, so
a body inlined at three of its four sites is still charged for four; counting
per site would need a second pass over the program after every copy, which is
the pass structure the one-budget-per-run design exists to avoid. (c) A
phase restriction (GHC's `INLINE [1]`). Section 11 of the policy note already
records that the monotone budget gets what phases get here, and no
measurement in this round prefers a phase split.

### 10.4 Three regressions the folds and the inliner left (2026-11-11)

The Stage3 suite after the ABI 8 landings was 140 of 228 targets and 860
tests against a baseline of 143 and 863. All three lost tests were in the
Cinch backend, and two of them shared one root cause.

**The inliner was on in the plain emitter entries.** The transform ships
off (`--inline` turns it on; the commit that admitted it measured it as
not paying on this corpus) and the CLI passes the flag, but
`emit-runtime-program-counting`, `emit-runtime-library-with-object` and
the two library artifact entries passed `True` themselves. Those are
the entries a Cinch program calls, so every test that emitted through a
program rather than the driver inlined: `bootstrap_runtime_ir` lost the
physical symbols `Main.getFunction` and friends that its C bridges name,
and `bootstrap_runtime_library` lost `Library.helper` (an internal worker
the test asserts, because a library object must keep every worker its
roots reach). A library source of 3-node units is exactly the shape the
40-node limit copies, so the loss was total, not incidental. Every plain
entry now passes `False`; only `-mode` entries take the flag, which is
what the stage build already did through the CLI default, so the ladder
itself never saw the copies and the regressions showed only in targets
that call a plain entry.

**Deferral crossed a closure boundary.** `DeferredConstructor` was
admitted when the binder's single use was a case scrutinee anywhere in the
let body, including inside a `lambda` or a recursive group. Those rebuild
their environment from `captures-from`, which drops a deferred constructor
(it has no word to capture), so the case inside the closure found no local
and emission refused with `runtime local is absent from its lexical
scope`. `task-immutable-adt-capture` is the shape: `let value = Box n in
withScope $ \s -> spawn s (\() -> case value of ...)`. Admission now
treats a use free in a closure boundary term as an ordinary use, so the
binder is built where the let is and captured by value. The transform's
reach on the ladder is unchanged (it still fires zero times there).

**The load fixture outlived its signature.** Section 2.5 changed
`body-node-owner` to take the prefix table (`List (String, Nat, Bytes)`)
instead of the loaded interface list; the owner-prefix test still passed
`loaded`. The fixture now builds `(prefixes-of (index-interfaces loaded))`,
the same index the production path builds, and keeps its reversed-order
half. No compiler change: the assertion was right and the call was stale.

### 10.5 One dispatch per case, phi joins, select arms (2026-11-11)

The stage-2 module had **one** `switch i64` and 19,313 alternatives reached
through `nextalt`/`nexttailalt` chains: `case-value` and `tail-rows` tested
one alternative at a time, so an N-alternative case read the scrutinee's
header N times and dispatched in source order, and every join was an
`alloca` slot. Three lowerings changed, all in `Bootstrap.RuntimeIr`:

* the case classifies its alternatives first and, when every one of them is
  a plain constructor test of one data type, reads the tag once and emits
  one `switch`. Guards, refutable nested fields, repeated tags, mixed
  shapes and decision kernels keep the chain, where source order is what
  decides which arm runs;
* a switch arm delivers its word from a block of its own, so the join names
  one predecessor per arm and is a `phi`; `branch` does the same. Only the
  chain keeps the slot, because a failed guard or field test must reach the
  alternative after it;
* `select` replaces a diamond when both arms are words that need no code.

Census of the compiler's own module, before and after:

| construct | before | after |
| --- | --- | --- |
| lines | 981,414 | 841,609 |
| `switch i64` | 1 | 5,261 |
| `select i1` | 0 | 0 |
| `br i1` | 43,355 | 24,373 |
| `= alloca` | 13,688 | 6,539 |
| `= phi i64` | 17,815 | 18,159 |
| `call` | 133,042 | 134,210 |
| chain alternatives | 19,313 | 5,777 |

The select buys nothing on this corpus: no value-position `if` in the
compiler's own source has two arms that are literals, immediates or
constants, so it fires zero times. It is kept because the shape is
reachable from source (a probe program emits `select i1` and computes the
right answer) and it costs one line to check. Calls rise 0.9% because the
tag read that the chain open-coded per alternative is now a shared worker.

Timings on this machine, interleaved pairs, loaded (the quiet figures in
section 1 are 7.45 s and 71 s): the toolkit check 11.37 s to 10.40 s
(-8.6%, three pairs), the self check 155.5 s to 114.9 s (-25.9%, one pair).
The stage-3 binary is 20,678,888 bytes against 20,703,496 (-0.1%). The
stage-2 and stage-3 modules, executables and interfaces are byte identical
to each other, and `bootstrap.py check` reaches the fixed point.

One bug this surfaced and had to fix: the decision kernel's case compared
the scrutinee against 0 and 1, which is not a decision's word (a decision
is a produced constructor, so its word is a block address). Naming those
constants as switch cases asserted the comparison could match, and the
case's miss block is `unreachable`, so the optimiser was free to assume an
impossible tag. `named-polymorphic-proof` returned 1 instead of 42, and
the instrumented stage-1 link trapped on a freed block. Decision cases
stay on the chain now.

### 2.16 The totality retry asked the wrong question (2026-11-12)

`source-total-resolved` retried a name by asking `source-total-from-checked`
whether the module elaboration had produced that name's total body, and the
question ran through `elaboration-bindings`, `elaboration-core` and
`elaboration-data`. Each of those answers `Nil` when the elaboration carries
any diagnostic, which is the right rule for a caller that may only act on a
fully accepted elaboration. For the retry it was the wrong one: a module
elaboration that reports a single error looked as though it had produced
nothing, so **every** name in the module took the retry path. The pass paid
one full module elaboration per name, on top of the elaboration that had
already produced the results, and a module of N names that hit one error cost
N + 1 elaborations. The comment above the function claimed the retry ran once
per *failed* name; it ran once per name, and that gap is the whole bug.

The retry now reads the elaboration's own lists through `elaboration-source-*`
and builds one name-keyed table per pass (`source-total-entries`). A name the
module elaboration resolved is emitted from that table and never re-derived;
the unresolved set is read off the same table (`source-total-missing`), so the
two cannot disagree. `source-total-retry` is a flat pass over exactly those
names: it cannot nest, and its length is the number of genuine failures rather
than a function of the module's size. The clean path is byte for byte the same
work as before, including its allocation profile, because no table is built
when the elaboration reports nothing.

The loop's shape, with N total candidates and one failing body, in blocks
(deterministic, unlike wall time on this machine):

|candidates|before|after|saved|
|---|---|---|---|
|5|46,762,359|36,327,677|22.3%|
|10|57,356,987|36,459,274|36.4%|
|20|79,191,240|37,281,782|52.9%|
|40|127,426,620|43,143,982|66.1%|

The marginal cost of one more candidate was 2.12 to 2.41 million blocks before
and 26 thousand to 293 thousand after: the difference between a module
elaboration and a closure elaboration. On the compiler's own 40,681-line
bundle the same change is 960,820,794 to 950,513,287 blocks (-1.07%), which is
the honest size of the win there: that bundle is mostly clean, so most of its
modules never entered the loop. A five-pair interleaved A/B on the self-check
scattered 44.0 to 68.6 s and could not resolve anything under about ten
percent, so the block counts are the evidence and the wall figures are not
quoted.

The two inner scans the profile put at 13.9% and 13.7% self,
`source-total-selected-references` and `binding-positions`, are not a separate
target: both are called once per module elaboration, so they fell with the
loop that was calling them. `source-total-selected-references` runs inside
`source-total-preliminary`, and `binding-positions` runs inside
`binding-names`, which `cache-body-context` calls once per
`elaborate-module-with-source-bodies`. A 40-candidate workload with one
failing body went from 21 elaborations to 2, and a six-second sample of the
working thread fell from 2,687 to 711 samples (-73.5%).

Byte identical: `compiler.ll` sha256 `d927b5f28c000823` from both arms on the
same input, all 179 non-bootstrap `.cinchif` files `cmp`-equal, and an errored
workload's diagnostics identical. The clean path is untouched: a 20-candidate
module with no failing body costs 42,544,261 blocks before and 42,543,049
after, a 0.003% difference, while the same module with one failing body goes
from 79,191,240 to 37,281,782.

### 2.15 One reference table per export (2026-11-12)

`Bootstrap.ExportNominal.binding-reference-names` walked every declaration
in the module once per exported row, so a module with N declarations and M
exported rows paid N times M declaration visits. On the 3,965-line toolkit
check it was the largest hot spot measured in this compiler: **59.3% of the
working thread's samples**, 2,868 of 4,833, with `Export.export-interfaces`
at 16.7% above it.

`reference-index` builds one name-keyed table per export and threads it
through `ExportIndex` to every row function, so a row costs one hash probe.
The table inverts the walk rather than reordering it: each declaration
contributes `(query name, references)` entries in the order the walk emitted
them, and each name's list keeps declaration order. Two first-wins rules are
preserved exactly, which is what keeps the output identical: a constructor
name resolves to its first occurrence, and a name two declarations both
answer for takes the earlier one.

Byte identical: `compiler.ll` sha256 `e2de3fe51fd9cc79` and all 47
`.cinchif` files `cmp`-equal to the pre-change stage-1 arm's output. The hot
frame is gone from the profile entirely (2,854 sample lines to 0), and the
remaining self time is the allocator, the dropper and `cinch_bytes_compare`.
An interleaved nine-pair A/B, both arms built from the same tree with only
these two files differing, put CPU at 6.07 s to 5.56 s (-8.4%); the wall
figures scatter too widely on a machine carrying other lanes to be quoted,
and the profile is the reliable evidence here.

### 2.15 Two hot spots removed, and one that was layout noise

Three findings from profiling the checker and the allocator, in the order they
were made.

**The export reference walk, removed.** `ExportNominal.binding-reference-names`
walked every declaration in the module once per exported row, which is the
product of the declaration count and the row count. It was **59.3% of self
samples** on the toolkit check. The fix builds one name-keyed table per export
and inverts the walk, so a row costs one hash probe; the table keeps the walk's
own emission order and its two first-wins rules, so the output is byte
identical and the frame is gone from the profile. Toolkit check **-8.4%**
(interleaved pairs, both arms from the same tree).

**The totality fallback, still open.** `source-total-bodies` is **50.2%
inclusive** on the self-check: when the preliminary elaboration reports any
error, `source-total-resolved` retries one name at a time and each retry
re-elaborates the whole selected module, reaching 110 nested frames. Its inner
scans are `source-total-selected-references` 13.9% and `binding-positions`
13.7%. A lane is on it.

**The allocator accounting was not the cost.** The survey predicted 2 to 3% from
inlining the allocation fast path. Measured here: `cinch_alloc_ctor` 1.4% of
samples, `allocate_block` 1.5%, drop and release 6.6%. Eliding the accounting
looked like 4.7% until a **control arm** was built: the same runtime with a
96-byte dead static added, no other change, bought **4% of the median**. All
the accounting arms, including one that batched the counters and produced byte
identical stats, sat in that band. The honest reading is that exact accounting
costs nothing measurable once layout is controlled, and the counters are
already per-thread (`Counter::of` writes the thread's own `Tally`, no atomics,
no shared line), so there was nothing to amortise. The lesson generalises: on
this tree a micro A/B on the runtime is indistinguishable from recompilation
layout unless it includes an inert control arm.

### 2.14 The 2026-11-12 round, and what the two workloads are now

Two numbers hold still enough to be a baseline, both on a quiet machine,
median of three and two respectively, with the stage-3 binary from the chain
in `build/bootstrap`: the 4K-line toolkit bundle checks in **6.14 s** and the
40,681-line compiler bundle in **54.4 s**. The binary is 16.46 MB, against
20.03 MB at the end of 2026-11-11, because the apply-stub fan-out collapsed
from 1,365 generated functions to 196.

What landed after section 2.13:

|Landing|Mechanism|Effect|
|---|---|---|
|reset and reuse|a `Release` that keeps its cell, a `Reuse` that writes into it, insertion after the inliner, a compatibility proof by tag and field count, and forced ownership of a reuse scrutinee|1000-cell rebuild 41,000 blocks to 1,000 (82x wall); toolkit constructor blocks -9.6%, wall -5.5%; self-check blocks -10.9%, bytes -11.9%|
|fixed-frame apply stub, collapsed|one `U.app` per function with a static frame and a direct saturated call, replacing 973 per-argument-count entries|1,365 functions to 196, 5.2 MB less IR, 16.46 MB binary; CPU neutral (a controlled pair put the fan-out at +0.16%)|
|static blocks addressed by offset|payload by constant `getelementptr`, not a linker alias|fixed a miscompile in every multi-object link and the PGO SIGBUS behind it|
|Nat immediate carry check|`(a >> 1) + (b >> 1)` re-encoded only when the sum fits 63 bits|fixed `2^63` reading as 0, which broke the compiler's own literal parsing|
|one static block per module|zero-field blocks emitted in the preamble rather than at the first address site|fixed a missing definition in partial modules|
|self-hosted `hiding`|subtract hidden names from an import's surface|`modules` 17 of 17 through both frontends|
|interface identity|a constructor family named by its defining identity, no bare-name alias table|the two producers now hash one contract identically|

The apply-stub fan-out is the one landing this round that cost more than it
bought, and the lesson is worth keeping: it removed 196 dynamic allocas (a
real property) but generated 973 specialised entry bodies of which 54 ran at
all, 0.17% of calls, and the block-interleaved comparison that "found" it
could not tell the stub from eleven other commits. A controlled pair, two
arms from one base differing only in the stub, put the stub at +0.16%. The
collapse keeps both properties and the code size it should have had.

### 2.15 Saturating a partial application, and why the closure ticket was empty (2026-11-12)

The backlog carried "stop passing a closure argument a direct call does not
use, and unbox single-entry closures into parameters", against the 25.3% of
toolkit value blocks that closures and dictionaries were measured at. A
census of both workloads on the current tree says that population is gone,
and says what replaced it.

`CINCH_ALLOC_KINDS=1` on the two workloads, stage-3 binary:

|kind|toolkit blocks|toolkit bytes|self-check blocks|self-check bytes|
|---|---|---|---|---|
|ctor|237,651,660|5.149 GB|990,349,234|22.01 GB|
|closure|10,536,483|437 MB|107,274,312|4.39 GB|
|nat|2,154,399|69 MB|8,251,694|264 MB|
|bytes|5,228,985|125 MB|15,265,787|366 MB|
|dict|0|0|0|0|
|all|266,237,723|5.890 GB|1,156,781,525|27.46 GB|

Closures are 3.96% of toolkit value blocks and 9.28% of the self-check's,
and dictionaries are zero on both: the static-dictionary landing removed
every dictionary allocation, so most of the 25.3% no longer exists as the
kind the ticket named.

Neither of the ticket's two mechanisms has anything to act on. **A closure
argument a direct call does not use** needs a unit with a parameter its body
never reads: 0 of 582 toolkit units, 0 of 6,128 compiler-source units, and 0
of the 6,833 functions in the emitted module, drop-only parameters included.
Every direct call passes exactly the callee's arity (1,578 of 1,578 and
50,062 of 50,062). The reason is structural: parameters are owned, so an
unused one would still need a `drop`, and the ownership pass leaves that drop
in the body. **Unboxing a single-entry closure** needs a `Lam` bound by a
`let` with one occurrence in `App` head position that saturates the callee:
also 0 on both workloads, because the checker's `specialize-local` already
substitutes a let-bound function value into its uses. A three-line fixture
(`let f = add3 x 1 in f 2`) produces the shape, so the detector is sound
rather than blind.

What does have population is the shape those two were standing in for: a
partial application applied once. `let f = g a b in f c` with `g`'s arity 3
builds a closure record only to apply it and release it. The rule
`saturate-partial` (`compiler/src/simplify.rs`) merges the two applications
into one saturated direct call, and refuses unless four conditions hold: the
head is a unit with a known arity, the value is under-applied, the binder is
read exactly once on the spine, and that read is an application's head. An
over-application stops at saturation and leaves the excess applied to the
result; a merge that would stay partial keeps its closure.

The measured population is small and honest about it: the rule fires **6
times on the toolkit bundle and 28 on the compiler source** (manifest
`simplifier.saturate_partial`), against the census's 9 under-applied sites in
the source (5 of them applied exactly once in head position) and 1 in the
toolkit. The census counts source-level sites; the rule's 28 include the
inliner's copies, which is why the two differ.

**What it moves, on the emitted module.** Both arms with a fresh
`CINCH_STORE` each, `--no-saturate-partial` as the other arm:

|measure|rule on|rule off|delta|
|---|---|---|---|
|compiler source `__apply` calls|1,481|1,512|-31|
|compiler source app stubs|196|199|-3|
|compiler source defines|7,022|7,025|-3|
|compiler source IR bytes|193,065,157|193,085,299|-20 KB|
|toolkit `__apply` calls|2|2|0|
|toolkit defines|9|9|0|

The 31 deleted applications are the census's sites plus the inliner's copies
of them: `Bootstrap.Interface.fnv-from` alone drops 6, and nine more units
drop 2 each, including `Text.Parse.line-text-at` and
`Text.Json.scan-literal-at`. Three apply stubs then lose their last taker
(`Bootstrap.Interface.fnv-step.app`, `Text.Lex.bytes-slice.app`,
`cinch.runtime.natCmp.app`), which is where the three defines go. The toolkit
arm is byte identical: its 6 rewrites are all inside units the dead-unit
sweep removes before emission, so the rule reaches no emitted site there.

That is a real but small deletion, and it is the closure item's whole
residue: 31 of 1,512 applications, 2.1%. It is worth keeping for the reason
the profile gives, since each deleted application removes a record, an
`__apply` and the stub's dup-branch-free-or-decrement sequence, but the
measured time is inside the spread of this machine and should not be quoted
as a speedup. Four interleaved pairs on the compiler source, child CPU from
`/usr/bin/time`, one binary, one build slot, each arm with its own store:
median 8.345 s with the rule against 8.360 s without, a difference of -0.18%
with the arms' own spreads (8.11 to 8.51 against 8.15 to 8.41) wider than the
gap. The toolkit check is 0.63 to 0.89 s per run in both arms, also inside the
spread.

**Where the release path actually is.** This lane first attributed 14.4 points
of `cinch_drop` to the apply stub, and that was wrong: the whole program runs
inside the entry chain's `.app` stubs, so every frame has a stub somewhere
above it. Asking which frame *calls* the release entry gives 0.00% from a stub
on the toolkit and 0.02% on the self-check; the stubs' own bodies are 0.27%
and 0.10%. The 13.9% is ordinary constructor releasing in the checker's hot
paths, attributed to the nearest enclosing Cinch unit:
`Bootstrap.Type.typed-index-carrier` 3.0%, `valid-comparison` 2.2%,
`carrier-tree` 1.2%, `read-carrier-tree` 1.1%, `valid-typed-math` 0.5%, and on
the self-check `Bootstrap.Dictionary.matching-heads` 0.4% and
`map-kind-types` 0.4%. (The same samples attributed to the frame that calls the
release entry directly put 1.4% in `carrier-tree`, 0.5% in
`read-carrier-tree` and 0.1% in `valid-comparison`, because most of them are
called from a short inlined helper; the enclosing-unit view is the one that
says which code owns the work.) The Cinch-built stage-3 binary agrees on the
self-check, sampled the same way over 60,890 working samples: release entries
**5.9%**, of which a stub calls **0.02%**, with stub bodies at **0.11%** and
`__apply` at **0.01%**. It is the price of the 990M constructor blocks a
self-check allocates. Backlog item 11 was aimed at the stub on the strength
of the wrong attribution and is replaced by the item the corrected profile
supports.

**The tail-position case, and why there is no second rule.** The one site the
census called the worst offender is
`Bootstrap.Infer.guarded-recursive-reference`, which binds
`guarded-recursive-reference` itself at 5 of its 7 arguments and applies it 15
times, off the spine. A rule for "a binder of known arity read exactly once as
a tail call's head" would be the same shape with the spine condition replaced
by tail position. It has no population, and the reason is visible in the site
itself: the binder is read 15 times, not once, and its uses supply 1 or 2
arguments against a shortfall of 2, so every use is still a partial
application and the shared closure is exactly what those 15 uses want.

That is the whole population of under-applied let-bound partial applications
in the compiler source, 9 sites, and none of them wants a second rule:

| uses | given / arity | caller -> callee |
|---|---|---|
| 15 | 5 / 7 | `Bootstrap.Infer.guarded-recursive-reference` -> itself, args 1 or 2 |
| 10 | 2 / 4 | `Bootstrap.Core.rewrite-structure` -> `rewrite-equal`, args 2 |
| 4 | 1 / 2 | `Bootstrap.Infer.check-rewrite-citation` -> `map-term-types`, args 1 |
| 3 | 1 / 2 | `Bootstrap.Infer.retain-checked-rewrite` -> `map-term-types`, args 1 |
| 1 | 3 / 4 | `Bootstrap.Interface.blank-field-string` -> `bytes-splice` |
| 1 | 1 / 2 | `Bootstrap.Interface.fnv-from` -> `fnv-step` |
| 1 | 2 / 3 | `Text.Parse.line-text-at` -> `bytes-slice` |
| 0 | 2 / 3 | `Bootstrap.Export.method-default` -> `portable-default-export-global` |
| 0 | 2 / 3 | `Bootstrap.Export.signature-pass` -> `portable-default-export-global` |

The four with one use are the rule's population, and the two with no use are
dead binders the occurrence rule already removes. The five shared ones would
need a rule that turned one shared closure into N saturated calls, which is
code growth for a saving that only exists where the shared closure was
actually being released, and the profile says the release path is not where
the stub is.

### 2.13 The 2026-11-11 round, in the order it landed

Nine landings changed the shape of the compiler rather than tuning it. The
numbers below are the ones each lane measured on its own gate, not a single
census, so they are quoted with the workload they came from.

|Landing|Mechanism|Effect|
|---|---|---|
|`9d975ee6`|dead-unit pruning rooted at what an object must export|library object correctness; check stopped recomputing|
|`54376012`|direct static-dictionary calls, small-unit inliner behind a flag|neutral on the compiler's own module, kept off|
|`5d62094a`|totality keeps the bindings it already produced; deferred diagnostics; identity literals; the runtime object parses once|self-check blocks 2.70B to 1.48B (-45%), CPU -16%|
|`12a9bd37`|frame blocks kept out of a tail call's frame|stack allocation sound; volume -4.1% blocks on a toolkit check|
|`b2be74ff`|one `switch` per case, `phi` joins, `select` arms|IR 981,414 to 841,609 lines (-14.2%), `switch` 1 to 5,261, join allocas -52%, toolkit -8.6%, self-check -25.9%|
|`07b02519`|identity spelling escapes only what is ambiguous|encoded names 2.62M chars to 1.19M, IR -5.4%, binary -3.1%|
|`f3610fa3`|GHC's inlining inputs: occurrence rules, argument interestingness, two thresholds, iteration cap|wrapper calls 80,834 to 15,112, IR -1.8%, allocation sites unchanged at 801|
|`3e696293`|the resolver's reference list indexed instead of scanned per declaration|toolkit -4.0%, self-check -11% at fixed pins|
|`a9a9a638`|static blocks addressed by a constant offset, not a linker alias|a real miscompile in every multi-object link, and the cause of the PGO SIGBUS|

The last row is the one to read twice. An immortal static block was emitted
as storage plus a global *alias* at its payload, and a multi-object link
does not preserve the relation, so `payload - 8` was not the header: the
release walk read the layout table's last word and stored into the text
page. On Apple that is `SIGBUS`, which is why the instrumented training run
of a sharded link died with exit -10 while the same module linked as one
unit ran. The fix addresses the payload by a constant `getelementptr`
offset, which travels as a relocation addend against the storage symbol.

Current state of the two workloads, quiescent, median of three and two
respectively: the 4K-line toolkit bundle checks in 6.13 s (49 s at the
start of the day) and the 40,681-line compiler bundle in 56.6 s (585 s).
Both are stage-3 binaries produced by the chain in `build/bootstrap`.

### 2.12a All-nullary types as immediates: the census, and why it does not pay (2026-11-12)

A proposal in circulation said a type whose constructors all have zero
fields should be a small integer rather than a heap block, worth "1 to 2%".
The population is real but the premise is wrong: those constructors are
**already not heap blocks**, so the allocation the proposal would remove was
removed by the static-block landing of 2026-11-11.

Measured on `main` at `2ac1b62e`, stage-3 compiler, `CINCH_ALLOC_STATS=1`:

|Arm (5000 iterations)|`header-blocks`|
|---|---|
|one-field `Some n` rebuilt each step|5001|
|all-nullary `Ordering` rebuilt each step|0|
|all-nullary `None` rebuilt each step|0|

The compiler module's own IR proves the mechanism rather than inferring it:
of 521 distinct descriptors passed to `cinch_alloc_ctor`, **zero** are the
layout id of any of the 96 nullary static blocks. Every nullary construction
is `ptrtoint` of a static block in the data section, so it allocates nothing,
counts nothing and frees nothing. `rc` on such a word reads an immortal
header and returns.

The population, from the checked interfaces of the 112 modules in the
library provider and the compiler (`exports.types[].ctors[].fields`):

- 147 types with constructors, 547 constructors.
- 23 of those types are all-nullary (78 constructors, 14.3%).
- The compiler's own source (`bootstrap/*.cinch`) declares 7 of them in 30
  constructors of 513 (5.8%): `PipelineMode` 8, `Comparison` 6,
  `ObservationPosition` 4, `CaseClass` 4, `ArrowUsage` 4,
  `TypeSubstitution` 3, `ExportEntries` 1.

What an immediate would actually save, per the compiler module (43.3 MB of
IR, 5,497 switches, 12,838 arms, 140,952 rc calls):

|Quantity|Count|Share|
|---|---|---|
|switches whose scrutinee reads only a header|224|4.07%|
|arms in those switches|272|2.12%|
|tag-read preamble bytes attributable to them|271,855|0.63%|
|rc calls on such a word|1,006|0.71%|
|static-block definition bytes, all types|1,411,000|3.26%|

So the ceiling is under 1% of the emitted module and 0.71% of rc traffic,
against a runtime ABI change: a new word class in `compiler/src/repr.rs`
(224 match sites across 15 files), a fourth pointer-map code (the 2-bit
field is full: `SCAN_RAW` 0, `SCAN_OBJ` 1, `SCAN_TOBJ` 2, 3 unused, so
`SCAN_BITS` must widen), a `case` that compares instead of loading a tag,
and a `runtime_abi` bump from 9 with both locks re-pinned. The narrow-value
note already says a layout change is an interface change needing exactly
that, and the survey's own estimate of 1 to 2% was an upper bound that
assumed the allocations existed.

There is a cheaper, strictly smaller change in the same area, and it is a
different bug. The self-hosted emitter mints a fresh `@empty-ctorN` per
construction site (`Bootstrap.RuntimeIr.static-constructor` calls `fresh`),
while the Rust seed dedupes by symbol (`ensure_static_ctor` inserts into
`emitted_blocks`). The compiler module therefore holds **7,126 definitions
for 96 distinct header words**: 7,030 byte-identical duplicates, 699,365
bytes of definitions (1.61% of the module) recoverable by keying the name on
the descriptor. That is a self-hosted-emitter change with no ABI, no class
letter and no lock, and it is worth more than the immediate.

### 2.12b The static blocks are deduplicated, and borrow inference is unreachable (2026-11-12)

Both halves of the 2.12a recommendation were taken.

**The dedup landed.** `static-constructor` now names a nullary block by its
descriptor instead of a per-site counter, and `dedupe-globals` keeps the
first line per symbol, which is the seed's `dedupe_static_blocks` rule
applied at the same scope. A block's bytes are a pure function of its
descriptor (header `rc=0`, `kind=1`, layout in bits 48..63, empty payload),
so descriptor equality is block equality and the name now says so. The
payload is still reached by the constant `getelementptr` offset into the
storage symbol, never a global alias, so the multi-object link is unaffected.

|Measure|Before|After|
|---|---|---|
|compiler module bytes|43,333,108|42,624,237 (-708,871, -1.64%)|
|`@empty-ctor` definitions|7,126|96|
|duplicate definitions (by header word)|7,030|0|
|stage-3 binary bytes|16,616,808|16,136,280 (-480,528, -2.9%)|
|definitions vs references|7,126 / 14,252|96 / 96, no dangling, no orphan|

The pass is not free and the cost is stated rather than hidden: it adds
1,672 rc calls in its own `dedupe-lines` code and removes 1,638 from the
units whose globals it now sees once, for a net +65 of 140,952 (0.05%)
against 708,871 bytes saved. The four fixed-point checks hold (IR, binary
and 47/47 interfaces byte equal), the three smokes pass, the leak line is
clean, `cargo test --lib` is 274/274, and `reclamation` (15 passed, 4
failed) and `ownership` (23 passed, 6 failed) match the baseline per test.
`rc_specialisation` fails one test on unmodified `main` too: it asserts
`@cinch.rc.dup.obj` names the emitter has not produced since the typed
helper rename, which is a stale assertion, not this change.

**Borrow inference already exists, and the self-hosted compiler cannot
reach it.** `compiler/src/rc.rs` has a full whole-program `borrow_modes`
fixed point (a unit takes part when every caller is a saturated direct
call, a parameter starts borrowed and becomes owned until the fixed point),
and `insert_closed` runs it. But `driver.rs` selects it only when the
compile is closed (`mode != EmitObject && emit_modules.is_empty() &&
package_objects.is_empty()`), and the bootstrap emits every stage with
`--emit-ir-with-interfaces --objects`, so the compiler's own module is an
open compile where every parameter is owned by design. The self-hosted
emitter has no borrow analysis at all: `count-op` skips only class 0
(`Raw`) and literal operands, and `Bootstrap.RuntimeIr`'s fourteen "borrow"
mentions are all `Ref`/`Mut` or extern argument conventions.

So the 60.1% of rc operations that sit on parameters (84,777 of 140,952,
measured in 2.12a) are not a missing pass in the seed. They are a missing
pass in `Bootstrap.RuntimeIr`, and 73.5% of the module's units (5,121 of
6,966) are borrow-eligible under the seed's own rule. `Main.lastOr` is the
canonical case and the tree's own `rc_specialisation` test asserts it
should not dup: measured through the self-hosted compiler it does dup its
list parameter, because nothing there infers a borrow.

The lesson for the next slice: do not port `borrow_modes` to
`Bootstrap.RuntimeIr` as a first step. It is a whole-program fixed point
over callers, and the self-hosted emitter is per-unit. The cheap first
move is the shape the seed already trusts, computed where the caller list
is known.

### 2.17 Two-tier interface rows: measured, and why the tail is not skippable (2026-11-13)

The survey's last interface-format item was per-name lazy decode: split each
`.cinchif` row into a small always-decoded tier (name, fingerprint, kind) and
a tail decoded on first use, so a consumer that never asks about a name never
pays for its scheme, its nominals or its body pointer. It was estimated at
1 to 3% of the toolkit check and more on the fixed cost of reading a small
interface. The counts say the shape of the estimate is wrong even where its
magnitude is right: the toolkit figure is too high (the ceiling is 0.65%),
the fixed-cost figure is real but for a different reason (7.5%, and it is an
encoding saving, not a decode tier), and nothing is ever lazily decoded,
because the load is the first consumer of every row. The change was not
landed; this section is the measurement that replaced it.

**Half the fixed cost is the JSON scan, before any row is looked at.** The
two-line program against the 64-interface base manifest allocates 34,679,671
blocks; against an empty manifest, 480. Two arms bracket where that goes.
Replacing every export section with an empty list, in the same 64 modules
with the same imports, gives 1,321,605 blocks, so 96.2% of the cost above
the empty-manifest floor is the export rows (their bytes being parsed plus
everything done with them) and 3.8% is the 64 module envelopes, their import
edges and the dependency chain check. Separately, padding `abi.symbols`
(which `strings-p` validates structurally and no consumer reads) at three
sizes gives the marginal price of a JSON byte:

| padding per module | manifest bytes | blocks | blocks per added byte |
|---|---|---|---|
| none (control) | 1,735,532 | 34,679,671 | |
| 1,024 bytes | 1,801,308 | 35,339,696 | 10.0 |
| 2,048 bytes | 1,866,790 | 35,994,751 | 10.0 |
| 4,096 bytes | 1,997,910 | 37,306,092 | 10.0 |

At 10.0 blocks per byte, the base set's 1,735,532 bytes are 17,355,320
blocks, exactly 50.0% of the fixed cost. So **half of reading an interface
set is `parse-json` scanning and building a tree, and the other half is
everything the load does with it.** No row tiering touches the first half.

**A JSON reader cannot skip a tail.** Two padding arms of the same 256 KiB,
both valid documents:

| arm | shape | manifest bytes | blocks | blocks per added byte |
|---|---|---|---|---|
| control | 64 base interfaces, unmodified | 1,735,532 | 34,679,671 | |
| `strpad256k` | 256 KiB as one long string in `assumptions.primitives` | 18,513,031 | 202,541,466 | 10.0 |
| `treepad256k` | 256 KiB as 4,000 small objects in `exports.foreign` | 19,399,503 | 227,449,089 | 10.9 |

The padding is per module, so these are +16.8 MB of manifest, not +256 KiB;
the rate is what matters. One long string is the *cheapest* possible JSON
payload and it still costs 10.0 blocks per byte: `Text.Json.parse-json`
builds a `Json` value for the document it is given and has no skip path,
because there is no length-prefixed section to jump over. A tree of small
objects costs 10.9, so 0.9 blocks per byte is the tree-building premium and
the other 10.0 is the scan. Any two-tier encoding that stays inside one JSON
document pays the whole scan; any encoding that moves a tail out of the
document breaks the identity rule (`iface-contract` blanks three regions of
the artifact's own bytes and hashes the rest), which is the schema bump
section 2.6 already priced. So a "lazily decoded tail" in JSON is not a
saving: it is the same bytes, the same scan, plus a decision per row.

**Per-section attribution, measured on the real reader.** Each arm below
empties one section of every document and re-settles every interface hash and
every `imports[].iface_hash` so the dependency chain check still passes (an
arm that fails it exits 7 at the envelope and measures nothing). Two-line
program, `--check`, `CINCH_ALLOC_STATS=1`:

| arm | interface bytes | blocks | delta vs control |
|---|---|---|---|
| control | 1,735,532 | 34,679,671 | |
| `foreign` (empty) | 1,419,179 | 30,493,453 | -4,186,214 |
| `instances` (empty) | 1,627,379 | 30,218,585 | -4,461,086 |
| `types` (empty) | 1,630,823 | 32,653,326 | -2,026,345 |
| `ctors` (empty) | 1,664,372 | 33,229,755 | -1,449,916 |
| `symbols` (empty) | 1,631,633 | 33,582,792 | -1,096,879 |
| `laws` (empty) | 1,724,666 | 34,543,324 | -136,347 |
| `assumptions` (empty) | 1,707,606 | 34,307,419 | -372,252 |
| `layouts` (empty) | 1,726,029 | 34,542,443 | -137,228 |
| `schemes` (values, emptied to `"X"`) | 1,566,221 | invalid (exit 7) | |
| `resolved` (nulled where legal) | 1,332,032 | invalid (exit 4) | |

The two arms that a per-name lazy decode would target, `schemes` and
`resolved`, cannot even be emptied: `values-valid` requires a non-empty
scheme, and the checker needs `resolved` for every imported value it
elaborates. `foreign` and `instances` are the two sections worth real blocks,
and neither is per-name: `envelope-valid` walks `exports.foreign` as
`objects-p`, and the instance tables are indexed whole by class and target.

**The demand is not sparse.** Of the base set's 3,008 exported rows, 243
(8.1%) have a name that appears literally in the toolkit bundle, which is the
generous upper bound on "names a consumer asks about". But the load validates
every row regardless: `values-valid`, `types-valid`, `signatures-valid` and
`instances-valid` each fold over the whole section, and `accepted-interface`
builds six row tables and two name lists before any consumer asks for
anything. A tail that is decoded on first use would therefore be decoded at
load for 100% of rows, because the load's own admission checks are the first
user. The one consumer that is genuinely per-name, `loaded-entry`, is already
a hash probe over tables built once.

**What the split could still buy, and why it is not worth the bump.**
Flattening each row from an object with repeated keys to a positional array
with trailing nulls dropped is a real encoding saving, and it is what a
two-tier row would look like:

| set | modules | bytes | rows as arrays | delta |
|---|---|---|---|---|
| base (fixed cost) | 64 | 1,735,532 | 1,474,996 | -15.0% |
| toolkit | 57 | 761,287 | 597,398 | -21.5% |
| compiler | 49 | 1,961,983 | 1,886,426 | -3.9% |

This one *does* show up in the block counts, because a byte saved is a byte
not scanned: at the measured 10.0 blocks per byte, the base set's -260,536
bytes are about 2.6M blocks, 7.5% of the fixed cost, and the toolkit's
-163,889 bytes about 1.64M blocks, 0.65% of its 251.5M-block check. That is
a real but small win, and it is the whole of what the two-tier idea can
deliver: it is an *encoding* saving, not a decode tier, and it needs the
schema bump and a producer rebuild for under 1% of the workload the survey
named. The one duplicated field worth removing outright is
`resolved.scheme`, which repeats the row's own `scheme` verbatim. On the base
set that is 2,513 of 2,746 resolved rows and 104,630 bytes, 6.0% of the
whole set: 2,081 of 2,265 value rows (69,433 bytes), 336 signature-method
rows (32,463 bytes) and 145 constructor rows (2,734 bytes). The compiler's
own set duplicates 107,007 bytes the same way. That is the same free 6% for
whoever next bumps the version, and `resolved-text` already reads the field
through `field "scheme" (field "resolved" metadata)`, so dropping it is a
writer change plus one accessor, not a reader redesign.

**Conclusion.** Two-tier rows with a per-name lazy tail do not pay, and the
reason is structural rather than a matter of tuning: the reader is a
whole-document JSON parser with no skip path, the identity rule requires the
whole document to be canonical JSON, and the load validates every row before
any consumer can ask for one. The levers this measurement leaves are the
ones sections 2.6 and 2.11 already named (a sectioned binary format, priced
as a coordinated schema bump) and the `resolved.scheme` duplicate, which is
6% of the base set for free at the next bump. Section 12 item 5 is updated
to say so.

### 2.18 The interface scan's per-character cons: 10.0 blocks per byte to 6.0 (2026-11-13)

Section 2.17 measured the price of a JSON byte in an interface set at 10.0
blocks and attributed it to the scan rather than to tree building. This
section finds the mechanism, removes it, and measures the law again.

**The mechanism.** `Text.Json.scan-string-body` accumulated the body of every
JSON string as a `List Char`:

```
scan-string-body s acc = case peek-byte s of
  ...
  else case utf8-at (state-src s) (state-off s) of
    Some pair -> case pair of
      (c, w) -> scan-string-body (advance-n s w) (Cons c acc)
```

That is five allocations per character of every string: the cons cell, the
boxed `Nat` width `w`, the `Char`, the `Some` of `utf8-at`, and the pair
inside it, plus a `JState` and a `Some` per `peek-byte`. Then `pack (reverse
acc)` walks the reversed list a second time and encodes every character back
to UTF-8 into a fresh buffer. Interfaces are mostly string bytes (64-character
`iface_hash` values, names, schemes, paths), which is why the cost tracked
bytes rather than entries: at 10.0 blocks per byte and roughly five
allocations per character, the string bodies are essentially the whole scan.

**The fix.** A string body with no backslash is a span of the source, so the
scanner records the span and the string is built from one slice. `scan-string-at`
now runs `scan-string-plain`, which walks the buffer by index (no `JState` per
byte), refuses exactly what the character scanner refused (a raw control byte,
an unterminated body, a malformed UTF-8 sequence) at exactly the same span, and
returns either the end offset or the fact that a backslash appeared. On a
backslash it falls back to the unchanged `scan-string-body` from the same
position, so escapes, `\uXXXX`, surrogate pairing and every diagnostic are the
original code. ASCII bytes step one byte with no decode at all, because
`utf8-at` answers `(the byte, 1)` there; only a lead byte at 128 or above calls
`utf8-at`, and it allocates one `Option` and one pair for that scalar.

**The law, three points, measured the same way as 2.17** (`experiments/iface-scan/measure.py`,
`--check` on a two-line program, `CINCH_ALLOC_STATS=1`, median of three
interleaved repeats, every identity re-settled after the padding):

| padding per module | manifest bytes | blocks before | blocks after | before | after |
|---|---|---|---|---|---|
| none (control) | 1,735,532 | 34,686,730 | 30,490,238 | | |
| 1,024 bytes | 1,801,195 | 35,343,442 | 30,884,996 | 10.00 | 6.01 |
| 2,048 bytes | 1,866,689 | 35,998,191 | 31,277,781 | 10.00 | 6.00 |
| 4,096 bytes | 1,997,747 | 37,308,531 | 32,063,953 | 10.00 | 6.00 |

The law's constant falls from **10.0 to 6.0 blocks per interface byte**, and it
stays linear (6.01, 6.00, 6.00), so the remaining cost is still proportional to
bytes. The empty-manifest floor is unchanged (19,891 blocks before, 19,832
after: noise), so the two-line program's interface load falls from 34,666,839
to 30,470,406 blocks, **-12.1%** of the whole fixed cost, and 4,196,433 blocks
in absolute terms.

**What the remaining 6.0 is made of.** The string fast path removed the
per-character part; what is left is per byte and per node, not per character:
the `Json` tree the parser builds (one `JStr` block and one `String` block per
string, one 24-byte or 40-byte cell per array element and object entry), the
`bytes-slice` copy that `string-from-bytes` then copies again (two copies per
string, because `bytes-slice` makes a fresh buffer and `string-from-bytes`
copies again by the wrap/copy discipline), and the escape-free scan's own
`bytes-get` per byte. A single copy instead of two is the next step and it is
a `Data.String` API question, not a `Text.Json` one.

**The two other workloads** (`experiments/iface-scan/workloads.py`, the PGO
training bundle, `Text.{Lex,Scan,Parse,Json,Builder}` plus an importing entry):

| workload | mode | blocks before | blocks after | delta |
|---|---|---|---|---|
| toolkit (3,991 lines) | `--check` | 251,512,085 | 249,441,758 | -0.82% |
| toolkit (3,991 lines) | `--emit-interfaces` | 248,578,674 | 246,508,350 | -0.83% |

The toolkit check barely moves because its per-line checking dominates, exactly
as section 2.17 found for the same reason: the interface scan is the fixed cost
of a small compile, not of a large one.

**Semantics held, proven rather than argued.** `scan-string-at`'s two callers
(`json-string`, `scan-entry-key`) both go through the new scan, and the escape
path is the original code from the same position. A differential probe over 29
documents (empty and long ASCII, every named escape, `\uXXXX`, a surrogate
pair, a lone low surrogate, a lone high surrogate followed by a non-escape,
raw two-, three- and four-byte UTF-8, a raw tab and a raw NUL, an unterminated
body, an unterminated body after an escape, an unknown escape, truncated,
overlong and lone-continuation UTF-8, strings inside objects and arrays, an
empty key, and trailing junk) produced byte-identical output from the old and
new scanners, diagnostics included: same spans, same `unexpected` and
`expecting` lines. The escape-free scan also runs the original scanner's own
tests in the original order, so every refusal lands on the same byte.

**The same pattern elsewhere, counted, not fixed here.** `Text.Csv`'s field
scanners (`scan-quoted`, `scan-bare`) accumulate `List Char` and finish with
`pack (reverse acc)`, and `Text.Csv.print-field` round-trips a whole field
through `unpack` to decide whether it needs quoting. `Text.Toml` builds a
`List Nat` per scalar and converts through `bytes-from-list`. Neither is on the
interface path, so both are reported here rather than changed in this lane.

## 11. Benchmark corpus and metrics

The corpus is edits, not programs. Each entry is a case in
`compiler/tests/speed/` with a fixed before and after source, and each is
run by the harness in both directions.

| Edit | Work it should cost |
| --- | --- |
| Whitespace change, or move a declaration | relex and reparse that file; no semantic work |
| Replace an opaque proof body | recheck that proof; reuse dependents and objects |
| Edit a private function body | recheck and re-emit the affected units |
| Add an instance | revisit instance searches whose buckets changed, including previous misses |
| Change a staged input snapshot | rerun the affected staged queries, propagate only on changed output |
| Switch x86 to Wasm | reuse target-independent frontend work and evidence; recompute layouts and codegen |

Metrics, reported per edit and per clean build:

- cold build time, warm edit to first diagnostic, warm edit to running, time
  to a fully verified artifact;
- p95 as well as the median;
- the critical path, the invalidation reasons for each rerun query, solver
  work and peak memory;
- how often a one-line edit reaches the solver, how often it reaches
  codegen, and how many dependencies stand between the edit and the first
  useful result;
- for the compiler's own build: phase times (load, parse, resolve,
  totality, infer, Core checks, export, RuntimeIr, clang) with clang's
  `-ftime-trace` JSON for the backend split.

## 12. Backlog, in evidence order

1. **PGO for the stage compilers. Done (2026-11-10).** `tooling/bootstrap.py`
   links each stage with a profile by default (`--no-pgo` or
   `CINCH_BOOTSTRAP_PGO=0` keeps the plain `-O2` link): instrument with
   `-fprofile-generate`, train on the toolkit bundle, `llvm-profdata merge`,
   relink with `-fprofile-use`, and install only the profile-use binary. The
   profile is keyed by the compiler source digest, the workload digest, the
   `llvm-profdata` version and the IR it was trained on, and lives at
   `build/bootstrap/pgo/<source sha256>-<family>.profdata`. The family is
   `seed` for Stage1 (its module comes from the Rust seed) and `cinch` for
   Stage2 and Stage3, which link identical IR and share one profile. Two
   things were learned wiring it: a profile only matches the IR structure it
   was trained on, so Stage1 cannot share Stage2's; and with a profile,
   clang's output for a module depends on the path it is given, so every
   stage now links its module from one content-addressed path or the fixed
   point breaks. The toolkit check falls from 15.47 s to 13.96 s median
   (9.8%, five interleaved runs) and the chain pays one instrumented link
   and one training run per source revision. See section 8.
2. **Profile harness. Done (2026-11-13).** `tooling/profile_build.py`
   runs a stage chain and emits phase timings, clang `-ftime-trace`
   summaries, sample profiles of the running compiler, and per-target test
   times, as one JSON file. The `-j` sweep (1, 2, 4, 8) over the four
   workloads landed with it as `profile_build.py sweep`:
   `many-small`, `wide-graph`, `deep-chain` and `huge-module`, each timed
   as `emit-ir` plus a `shard_ir.py` split compile of that same module, so
   the K>1 arms are one program split differently rather than a different
   program. Two defects in its own sample parser were fixed at the same
   time and had been silently flattening every profile it produced; section
   2.17 has them and the rule they imply.
3. **Parallel code generation. Done (2026-11-10).** `tooling/shard_ir.py`
   emits one `.ll` per shard (grouped by module prefix, fixed order) and
   `tooling/bootstrap.py` compiles them with `clang -O2 -c` in parallel
   under `-j`, then links the objects in shard order. The Stage1 link falls
   from 128.9 s to 20.1 s wall at K=8 and the Stage2 link from 45.1 s to
   9.7 s, the compiler's own toolkit check is unchanged, and the fixed
   point holds. What remains is recovering cross-shard inlining (ThinLTO
   over the shard objects, or inlining at the Core level before emission)
   and emitting objects directly instead of text IR; neither is worth more
   than the frontend work in items 6 and 7 until the module shrinks.
4. **Shrink the emitted IR further. Not taken this round.** Re-measured
   after the item 11 landing: the stage-3 module is unchanged in shape by
   that change (the win is allocation, not IR), and no single remaining
   construct reached the size where a rewrite pays. The largest functions
   and the repeated `case` spines are still there; what is missing is a
   census of IR lines per construct rather than per function, which the
   harness can now supply. Section 2.18's superlinear frontend is the
   larger IR-adjacent finding and is a representation change, not a
   peephole.
5. **Interface format.** Sectioned or indexed `.cinchif` decoding, plus an
   optimization section (size, arity, inline guidance). Needs a schema
   version bump and a producer rebuild. Dense typed bodies with the type
   side table are produced today by `--emit-bodies` (section 2.11); they
   wait here for the sectioned format. **The per-name lazy tier is measured
   and does not pay** (section 2.17): the reader is a whole-document JSON
   parser with no skip path, the identity rule needs the document canonical
   as a whole, and the load validates every row before any consumer asks for
   one, so a "lazy" tail is decoded at load for 100% of rows. What that
   measurement leaves here is the sectioned binary format it already named,
   and `resolved.scheme` (which duplicates the row's own `scheme` for 2,513
   of 2,746 resolved rows, 6.0% of the base set) as a free saving for the
   next bump.
6. **The frontend is now the critical path.** Stage2's build spends 200.7 s
   of 245.8 s in the single-threaded frontend, so per-declaration query
   reuse with fingerprints and early cutoff (section 3) and a parallel
   phase schedule (section 9) matter more than any backend work.
7. **Interning `Ty` and names. Rejected for now, on measurement
   (2026-11-13).** Names are done in the seed (section 2.3, -11.1% on its
   check). The remaining half is `Ty` nodes as an arena plus integer ids,
   which needs `core.rs`'s mutation paths to take ids. Measured before
   starting it, on both workloads with the fixed harness: `Ty` is not in
   the profile. The top self samples are the allocator and the release
   walk (30.0% and 17.9% before item 11, still the top two after), then
   `bytes-compare` 1.8% to 3.2%, `natCmp` 0.9% to 2.4%, and
   `string-compare` 1.5% to 2.3%. No `Ty`-shaped frame reaches 1%: the
   checker's types are already small trees that the allocator serves in a
   few words, and the cost that looked like type copying was the carrier
   document of item 11. An arena would remove one pointer indirection per
   node from a path that is not the bottleneck, and would touch every
   mutation path in `core.rs` to do it. The measurement that would revive
   this item is a workload where `Ty` construction or traversal shows up
   above 2% self after the carrier fix; nothing measured here does.
8. **Finish the GHC-style simplifier** on our Core: case-of-case with join
   points, call-arity and eta, loopification, CPR plus worker/wrapper for
<<<<   single-field constructors, spec-constr and liberate-case. The pass
   shape, the tick budget and the first three rewrites (case-merge,
   known-constructor, direct dictionary calls) are in the tree (section
   10.1); case-of-case needs the join node in Core first.
9. **LTO across the library boundary. Rejected on measurement
   (2026-11-13).** Section 8 has the full table. The fault is understood
   and gone (`a9a9a638` and `618f276a` removed its cause; neither LTO shape
   reproduces it, and both pass the leak gate and emit the baseline
   `compiler.ll`), so the parked question is answered. What rejects the
   work is the cost: ThinLTO is 1.5% *more* CPU on the toolkit check and
   within 2% on the self-check, full LTO and the sharded thin link cost 4%
   to 9%, and all of it adds 26% to 33% of stage link time for no check-time
   gain. The size drop is real (`__text` -13%, ~5 MB of export table) and
   does not reach the process: peak resident set is 175 to 176 MB in every
   arm. The 14% call population it would have inlined is also already
   smaller than the estimate, because the interface unfoldings removed
   2,833 wrapper calls at the Core level first. Nothing was wired.
-   single-field constructors, spec-constr and liberate-case. The pass
-   shape, the tick budget and the first three rewrites (case-merge,
-   known-constructor, direct dictionary calls) are in the tree (section
-   10.1); case-of-case needs the join node in Core first.
   single-field constructors, spec-constr and liberate-case. In the tree:
   the pass shape, the tick budget and five rewrites (case-merge,
   known-constructor, direct dictionary calls, saturate-partial,
   case-of-case with join points) in section 10.1 and 10.2, and the
   computed-argument fold written and measured but **off by default**
   because it makes the inliner duplicate (section 10.6). Loopification is
   already done by code generation, which turns a saturated self tail call
   into a branch to the unit's loop header (`compiler/src/codegen.rs`, the
   `loop_label` in the entry block, decision D104); the Core-level version
   would only restate it. What is genuinely left, in cost order: the
   inliner term that would let the computed fold ship (section 10.6), CPR
   plus worker/wrapper for single-field constructors, call-arity and eta,
   then spec-constr and liberate-case.
 9. **LTO across the library boundary**: finish the investigation in section
    8. It is worth one focused pass because it halves the binary and removes
    the call cost to the library's string and `Nat` routines (14% of the
    compiler's profile), but only if the stack-probe fault is understood.

10. **Parallel phases** using the runtime pool, once section 11's corpus
   says which phases have enough independent work: RuntimeIr emission per
   unit, then inference per definition, then interface export per module.
11. **Constructor release in the checker's hot paths. Done (2026-11-13).**
   Section 2.17. With the profile harness's parser fixed, the release path
   measured at 30.0% of the toolkit check's self samples and 17.9% of the
   self-check's, and the capture rate per unit named the cells: they were
   `value-kind`'s JSON documents, built and dropped by
   `typed-index-carrier` (4.9%), `valid-comparison` (3.6%),
   `read-carrier-tree` (3.4%) and `carrier-tree` (2.4%). Neither candidate
   was the right move: the cells existed only to carry a value the caller
   already held, so the fix was to stop encoding. `index-carrier` reads the
   carrier directly, and the counts fell 36.2% on the toolkit check and
   14.7% on the self-check with the output byte identical. What remains is
   `Kind`'s representation (`value-kind` still encodes), which is the next
   item rather than another caller rewrite. An earlier revision of this
   item sent the work to the apply stub on a stack-attribution artifact; do
   not resurrect that reading.
12. **Development codegen tier. Not yet, and the harness says why
   (2026-11-13).** The trigger condition is "the backend is the measured
   bottleneck for edit-compile-run", and it is not. `profile_build.py sweep`
   measures the two halves separately over the four workload shapes, and
   the backend is a rounding error at every shard count: `many-small`
   spends 2.4 s in `emit-ir` and 0.04 to 0.05 s in the split `clang -O2`,
   `wide-graph` 2.5 s and 0.04 s, `deep-chain` 5.3 s and 0.05 s. The
   frontend is 50x to 100x the backend on the shapes an edit-compile-run
   loop actually sees, and section 2.18 shows it is superlinear in
   definition count. A cheaper codegen tier would optimize the 0.05 s. The
   item stays parked until a sweep arm shows the backend above the
   frontend, which the `sweep` subcommand can now state directly.
13. **Deduplicate the self-hosted emitter's static nullary blocks. Done
   (2026-11-12).** Keying the symbol on the descriptor took the compiler
   module from 7,126 definitions to 96 and from 43,333,108 to 42,624,237
   bytes, and the stage-3 binary by 480,528. Section 2.12b has the table
   and the pass's own +65 rc calls.
14. **Borrow inference in the self-hosted emitter.** The seed has a whole
   program `borrow_modes` fixed point and `insert_closed` runs it, but the
   bootstrap compiles with `--objects`, so every stage module is an open
   compile with every parameter owned; `Bootstrap.RuntimeIr.count-op` skips
   only `Raw` and literals. 84,777 of 140,952 rc operations (60.1%) sit on
   parameters and 73.5% of units are borrow-eligible under the seed's own
   rule. Section 2.12b says why porting the fixed point is the wrong first
   move and what the cheap one is.
Related designs: `parallel-evaluation.md`, `core-output.md`,
`incremental-recompilation.md`, `self-hosting.md`, `layouts-and-specialization.md`,
`inline-target-ir.md`, `reclamation.md`, `supercompilation-and-jit.md`.
