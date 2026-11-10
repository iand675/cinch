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
| Full Stage0 to Stage3 chain | 20 min | cold, with the library provider |
| Cinch-built Stage2 checks the 4K-line toolkit | 11 s at `-O2`, 21 s at `-O0` | |

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

ThinLTO across the compiler module and the library object links and halves
the binary (15.9 MB to 11.9 MB) but the result faults at run time in the
stack probe (`ldr xzr, [sp]`) inside an inlined caller. That is the same
AArch64 `probe-stack` interaction with dynamic allocas found earlier: our IR
rounds every dynamic alloca to 16 bytes and marks it `align 16`, so the
fault is either an LLVM defect on a path we have not isolated or a site that
still bypasses the rounding. It is worth one investigation because LTO also
removes the cross-module call cost to the library's string and `Nat`
routines, which the profiles put at 14% of the compiler's time. Until then,
LTO stays off.

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

The next lever is the missing half of the fold: a pushed decision on a
constructor application whose arguments are *computed* would collapse if
the substitution were allowed when the field is read at most once, which
needs an evaluation-order argument the pass does not yet state.

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
2. **Profile harness.** A `tooling/profile_build.py` that runs a stage
   chain and emits phase timings, clang `-ftime-trace` summaries, sample
   profiles of the running compiler, and per-target test times, as one JSON
   file. Everything below is measured through it. Also add a `-j` sweep
   (1, 2, 4, 8) over four workloads: many small modules, a wide dependency
   graph, a deep import chain, one huge generated module.
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
4. **Shrink the emitted IR further.** Re-measure after the reference-count
   and match changes, then target the largest remaining constructs:
   functions above 20K IR lines, repeated `case` spines, and every
   remaining out-of-line helper call.
5. **Interface format.** Sectioned or indexed `.cinchif` decoding, plus an
   optimization section (size, arity, inline guidance). Needs a schema
   version bump and a producer rebuild.
6. **The frontend is now the critical path.** Stage2's build spends 200.7 s
   of 245.8 s in the single-threaded frontend, so per-declaration query
   reuse with fingerprints and early cutoff (section 3) and a parallel
   phase schedule (section 9) matter more than any backend work.
7. **Interning `Ty` and names.** Names are done in the seed (section 2.3,
   -11.1% on its check). What is left on this item: `Ty` *nodes* (an arena
   plus integer ids, which needs the mutation paths in `core.rs` to take
   ids), and the same interning in the Cinch compiler, where identity
   decoding and export scans still compare spellings.
8. **Finish the GHC-style simplifier** on our Core: case-of-case with join
   points, call-arity and eta, loopification, CPR plus worker/wrapper for
   single-field constructors, spec-constr and liberate-case. The pass
   shape, the tick budget and the first three rewrites (case-merge,
   known-constructor, direct dictionary calls) are in the tree (section
   10.1); case-of-case needs the join node in Core first.
9. **LTO across the library boundary**: finish the investigation in section
   8. It is worth one focused pass because it halves the binary and removes
   the call cost to the library's string and `Nat` routines (14% of the
   compiler's profile), but only if the stack-probe fault is understood.
10. **Parallel phases** using the runtime pool, once section 11's corpus
   says which phases have enough independent work: RuntimeIr emission per
   unit, then inference per definition, then interface export per module.
11. **Development codegen tier** (Cranelift or a direct emitter) once the
   backend is the measured bottleneck for edit-compile-run rather than for
   clean builds.

Related designs: `parallel-evaluation.md`, `core-output.md`,
`incremental-recompilation.md`, `self-hosting.md`, `layouts-and-specialization.md`,
`inline-target-ir.md`, `reclamation.md`, `supercompilation-and-jit.md`.
