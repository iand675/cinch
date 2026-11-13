# Parallel evaluation, phase 3: where the self-hosted compiler can and cannot go parallel

Status: survey, 2026-11-12. Phase 3 of `design/parallel-evaluation.md` landed
the primitives in the self-hosted compiler: `parBoth`, `parTabulate` and
`parMap` are installed by `Bootstrap.Builtin`, elaborated by
`Bootstrap.Infer` (scheme, callback-literal rule, parallel-safe row),
capture-checked by `Bootstrap.Typing` (`par-definition-error`) and lowered by
`Bootstrap.RuntimeIr` to `cinch_par_both`, `cinch_par_tabulate` and
`cinch_par_map_array`. This note is the next question: where in the compiler's
*own* pipeline can those primitives be used so the output stays
byte-identical, and what blocks the phases that cannot.

The answer, measured rather than argued: **the two phases worth parallelising
are blocked by the `Share` judgement refusing exactly the two types they
capture**, and by a third, independent fact that `parMap` cannot be used in
the compiler at all until `Data.Array` is part of its source set. Nothing was
changed in the compiler, because a `--par-eval` flag whose only observable
effect is to serialise would be a lie in the commit message.

## 1 How the pipeline is shaped

`compile-source-with-manifest-mode` reads the interface manifest, reads the
source, parses, and hands the program to `after-parse-with`, which resolves,
expands stages, checks and emits. Every stage consumes the previous one's
whole result:

```text
read-interfaces -> parse-program-with -> expand-stages -> resolve-program-with
  -> check-and-retain (elaborate-program) -> export-interfaces -> emit-runtime-program
```

Independent per-module work exists in exactly one place: **interface export**.
`export-interfaces` walks `elaboration-contexts typed`, one `Json` per module,
and each iteration builds one `.cinchif` document. That is 37% of a
self-compile (section 3).

Parse is per-module in principle but not in practice: `parse-program-with`
computes one fixity table over the whole token stream first, and the bundle
`tooling/bootstrap.py` feeds the ladder is a single 2.8 MB file whose
`-- source: NAME` markers are comments, not boundaries the parser sees.
Splitting it is a change to the tooling, not a `parMap` over a list.

Everything else is whole-program by construction:

- `expand-stages` resolves and checks the *entire* program for each module
  that has stage work, and threads one fuel budget across modules.
- `resolve-context` flattens every module into a single `Main` module before
  checking, so `elaborate-modules` iterates once in the compiler's own build.
- `emit-runtime-program` is demand-driven over one `Writer` with one monotonic
  fresh-name counter, so it is sequential until that counter is unit-local
  (design note section 5.3, its phase 4).

## 2 The profile

The compiler bundle was compiled with `--emit-ir-with-interfaces --objects`
and sampled once a second for the whole 145 s run; each sample's call graph
was attributed to the outermost `Bootstrap.Pipeline` frame on the compiler
thread. Eleven samples landed on that thread. Shares are inclusive counts
against `Main.run-cli`'s total for those samples.

| Phase | Share |
| --- | ---: |
| `elaborate-program` (kinds, classes, bodies, Core re-check, laws) | 44.8% |
| `export-interfaces` | 37.0% |
| `write-source-artifacts` (`.cinchif` and manifest writes) | 18.5% |
| object admission | 1.2% |
| parse, resolve, stage expansion, RuntimeIr emission | 0% in this window |

Eight earlier samples of a `--check`-only run agree at a finer grain:
`elaborate-module` was 100% of the run with `check-definition` around 90% of
that. A `--check` of the bundle took 101.6 s wall and 96.5 s user at
`CINCH_THREADS=1`, and 90.1 s wall at the default budget, so the frontend is
single-threaded CPU work and today's pool buys nothing (the compiler contains
no parallel call). Check and export are 81.8% of the compile and are the only
phases worth attacking.

## 3 What blocks the check phase

`check-body-loop` threads one `Store` through every definition. `Store` is
`MkStore label next level cells errors numbers in-law depth`: `next` is the
metavariable counter, `cells` every metavariable ever created, `errors` the
accumulated diagnostics. Two definitions checked in parallel would call
`fresh-id` on the same `next` and collide. The design note's fix, per-body
stores with local numbering, is its phase 4 and changes every metavariable id
in the emitted Core.

The loop also stops at the first failing definition, deliberately: a failed
body leaves recovery types and an undischarged store, so later diagnostics
would be artifacts of the first failure. A `parMap` must therefore pick the
first failure *by source order*, which is the error-ordering property this
phase's acceptance names.

Both are surmountable. The third obstacle is not:

**`Store` is not `Share`.** Probed inside the real ladder bundle (all 41
`bootstrap/*.cinch` modules plus the five `Text.*` toolkit modules, with only
the library interfaces loaded, which is the configuration the self-build
actually uses):

```text
bootstrap.check:39874:24: a parTabulate callback runs once per index, so s
cannot move into it: a parallel capture must be Share
(design/parallel-evaluation.md section 2.3)
```

## 4 What blocks the export phase

`export-loop` looks parallel: a list of per-module `Json` contexts, one
document each. Three things stand in the way.

**Documents name their dependencies' hashes.** `imports` calls
`lookup-hash name produced interfaces`, where `produced` is the list built so
far, so a module's document cannot be built until its imports are. The loop
gets dependency order for free by walking contexts in order. Parallelising it
means computing the module waves first and running each wave as a `parMap`;
the import graph is static, so the wave assignment is deterministic and the
bytes stay identical. This one is feasible.

**`signature-rows` closes over classes across the whole elaboration.** Its
`signature-closure` runs a fixed point against `elaboration-classes typed`,
so a module's signature rows are not a function of that module alone.

**`Elaboration` is not `Share`.** From the same probe run:

```text
bootstrap.check:39878:23: a parTabulate callback runs once per index, so e
cannot move into it: a parallel capture must be Share
(design/parallel-evaluation.md section 2.3)
```

Since `export-interfaces` reads the shared `Elaboration` for every module, the
phase cannot be a `parMap` over contexts until that changes.

## 5 What the `Share` judgement actually admits

Measured in the same bundle, capturing each type in a `parTabulate` callback:

| Type | `Share` |
| --- | --- |
| `Span`, `Nat`, `List Nat` | yes |
| `String`, `Json`, `List (String, Json)` | **no** (measured by a later lane: `String` is `StrBuf :: Bytes -> String` and `Bytes` is transfer-only, so `parMap` over any of these is refused because the element type must be `Share`) |
| `Program`, `Module`, `Decl`, `Expr`, `Pat`, `TypeExpr` (as source) | **yes** |
| `Store` | **no** |
| `Elaboration`, `CheckedProgram` | **no** |

Two corrections to what a reading of the source suggests, both worth
recording because they cost time to find:

- The AST is fine. `Program`, `Module`, `Decl`, `Expr`, `Pat` and `TypeExpr`
  all pass `task-shareable` when the module that declares them is in the
  bundle. A local `TypeExpr` copied verbatim from `Bootstrap.Ast` with all 23
  constructors is accepted, a 40-constructor synthetic ADT is accepted, and a
  self-recursive `Tree` is accepted.
- An *imported* ADT is not `Share`, and the refusal is silent about why.
  `task-data name (context-data context)` looks the declaration up in the
  module's own `CoreData` list; an interface supplies export rows and a
  nominal table, not constructors, so `task-data` answers `None` and
  `task-data-property` answers `False`. `Data.String.String` and the machine
  primitives are admitted by name and nothing else imported is. This is why
  the same probe that is accepted as source is refused when the module is a
  `.cinchif`: with `Bootstrap.Ast` loaded as an interface rather than as
  source, capturing an `Expr` is refused.

So `Store` and `Elaboration` are refused even as source, for structural
reasons inside their own declarations, and that is the real blocker.

## 6 `parMap` cannot be used in the compiler yet

`parMap` lowers through `Data.Array`'s `array-from-list` and `array-to-list`
(`Bootstrap.Infer.par-map-array-call`), and `Data.Array` is a source module,
not part of the embedded base package. `collect_compiler_sources` bundles the
41 `bootstrap/*.cinch` modules and the five `Text.*` toolkit modules and
nothing else, so a `parMap` anywhere in the compiler fails at resolve:

```text
bootstrap.resolve:39869:3: missing source module or checked interface: Data.Array
```

`parTabulate` needs nothing (it returns the `Array` directly), which is why
every probe above uses it. Adding `Data.Array` to the bundle, or moving the
two conversions into a `cinch/prim` module as the design note's section 8
wants, is a prerequisite for using `parMap` over the module or definition
list.

Also recorded: **`parSetWorkers` is not a source builtin.** It exists in the
runtime (`compiler/runtime/src/par.rs`) and is named in the design note, but
`Bootstrap.Builtin`'s `builtin-bindings` does not install it, so a
`--par-eval N` flag written in Cinch would have to set the budget through
`CINCH_THREADS` or have the builtin added (one entry plus its scheme).

## 7 What is safe today

The pool costs nothing when unused, so the default budget can stay on. A
`--check` of a three-line program takes 1.01 s user at the default budget and
1.05 s at `CINCH_THREADS=4`; no worker thread starts, because
`parallel_participant` returns `None` when nothing publishes. The primitives
are reachable and correct: `parMap` over `List Nat` type-checks, emits
`@cinch_par_map_array` in the IR, and its sequential and parallel readings
agree. (`List (String, Json)` does **not** type-check: `String` is not
`Share`, see section 5, which a later lane measured after this survey
recorded it the other way.)

## 8 The order this should happen in

1. **Make `Store` and `Elaboration` `Share`**, or stop capturing them. Find
   which field fails `task-data-property` in each and fix the judgement or
   the declaration. Acceptance: a bundle capturing a `Store` and an
   `Elaboration` in a `parTabulate` callback is accepted, and every refusal
   case of the design note's section 2.3 still refuses.
2. **Get `Data.Array` into the compiler's source set**, or move the two
   `parMap` conversions to a primitive module, so `parMap` resolves at all.
3. **Add `parSetWorkers`** to `Bootstrap.Builtin` and its scheme to
   `Bootstrap.Infer`, so `--par-eval N` can set the budget from Cinch.
4. **Per-definition stores** (design note phase 4), the prerequisite for
   parallel bodies regardless of `Share`.
5. **Wave-ordered export** as the first real parallel phase: its unit is
   already per-module and its merge is already ordered.
6. **Parallel body inference** last: it needs 1, 3 and 4 together, and it is
   the one whose error ordering the acceptance criterion watches.

Until step 1 lands, every candidate phase is blocked on the same judgement,
and a speedup shipped now would be measured on a compiler that had to
serialise to stay correct.
