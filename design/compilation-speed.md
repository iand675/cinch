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
| Core text output, verifier, diff, server, grammar, editors | `compiler/src/coretext.rs`, `compiler/src/lsp.rs`, `tooling/tree-sitter-cinch-core`, `tooling/editors/*` | makes each stage's codegen input inspectable and comparable |
| `--opt-level` and `-O2` library objects and stage links | `compiler/src/driver.rs`, `tooling/bootstrap.py` | 28% on the toolkit check |
| Core simplifier: case-merge, known-constructor, direct dictionary calls | `compiler/src/simplify.rs` | removes `Option`/`Result` scaffolding and static-dictionary `__apply` calls; reports ticks (section 10) |
| Package store keyed by content | `compiler/src/store.rs` | reuses checked artifacts per compile |
| Analysis layer + LSP incremental update | `compiler/src/analysis.rs` | editor path, not build path |
| Work-stealing parallel runtime (`cinch_par_*`) | `compiler/runtime/src/par.rs` | available; the compiler is not yet parallel |

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

## 8a. Tried and measured: PGO, LTO, and the compiler's own build

Profile-guided optimization of a stage compiler works and is cheap to wire:
link the compiler IR with `-fprofile-generate`, train it on a real workload
(a 4K-line bundle check is enough), merge with `llvm-profdata`, and relink
with `-fprofile-use`. On the toolkit check the same IR ran in a median of
13.7 s against 15.3 s plain `-O2`, about 10%, with identical output. The
profile is keyed to the compiler source, so it must be regenerated when the
sources change; and because every stage is the same program, one trained
profile serves all of them (functions whose IR differs are simply skipped,
with the profile-use warnings suppressed). Link cost is one extra
instrumented build per source revision.

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
known-constructor reduction orphaned. Case-of-case and join points are
deliberately absent: they need a join node in Core and a printer for it.

Measured on the self-build corpus, the pass converges before the default
cap: on `compiler/tests/core/ownership.cinch` it runs two passes with 160
ticks; the distribution is in the report and in section 11's harness.

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

1. **PGO for the stage compilers.** Wire the measured 10% into
   `tooling/bootstrap.py`: instrument, train on the bundle, merge, relink,
   key the profile by the compiler source digest. Do this after the shard
   work lands so both changes touch the link path once.
2. **Profile harness.** A `tooling/profile_build.py` that runs a stage
   chain and emits phase timings, clang `-ftime-trace` summaries, sample
   profiles of the running compiler, and per-target test times, as one JSON
   file. Everything below is measured through it. Also add a `-j` sweep
   (1, 2, 4, 8) over four workloads: many small modules, a wide dependency
   graph, a deep import chain, one huge generated module.
3. **Shard the module for parallel code generation.** One 218 MB
   translation unit runs on one thread; the time trace above says 54 s of
   the 55 s `-O2` cost is backend work that shards in parallel. Emit one
   `.ll` per shard (grouped by module prefix, fixed order), compile shards
   with `clang -O2 -c` in parallel under one `-j` budget, and link the
   objects in sorted order so the result stays deterministic and the
   Stage2/Stage3 binaries still match byte for byte. Measure shard counts
   1, 2, 4, 8 for wall time, total CPU and the resulting compiler's own
   speed (cross-shard inlining is what we give up; measure it).
4. **Shrink clang's input further.** Re-measure the IR after the
   reference-count and match changes, then target the largest remaining
   constructs: functions above 20K IR lines, repeated `case` spines, and
   every remaining out-of-line helper call.
5. **Interface format.** Sectioned or indexed `.cinchif` decoding, plus an
   optimization section (size, arity, inline guidance). Needs a schema
   version bump and a producer rebuild.
6. **Per-declaration query reuse inside one build**, with fingerprints and
   early cutoff. Start with `signature` and `elaboratedBody` because they
   dominate a self-build.
7. **Interning `Ty` and names** in the Cinch compiler, and in the seed
   where the profile still shows clone-heavy code.
8. **Finish the GHC-style simplifier** on our Core, in this order: case-merge,
   case-of-case with join points, call-arity and eta, loopification,
   CPR plus worker/wrapper for single-field constructors, spec-constr and
   liberate-case. Gate every step on the fixed point and on Core output
   equality, and measure the compiler's own runtime, which is the best
   benchmark we have. The pass shape, the tick budget and the first three
   rewrites (case-merge, known-constructor, direct dictionary calls) are in
   the tree (section 10.1); case-of-case and join points are the next
   slice, and they need the join node in Core first.
9. **LTO across the library boundary**: finish the investigation in section
   8a. It is worth one focused pass because it halves the binary and removes
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
