# Saturated calls across argument witness scopes

A tail call whose final argument contains checked operations can lose its known
callee in Native lowering. The checker opens the argument's witness `let`s by
first binding the partially applied function to `#argument-function`. This keeps
evaluation strict and ordered, but the emitter then sees a local closure rather
than the saturated source call. A 64 KiB fixed-width byte recurrence exhausted
the ordinary 8 MiB stack through that path.

`Bootstrap.RuntimeIr` now recognizes one narrow case: a compiler-generated
monomorphic alias of a proper partial source application, followed by a single
saturated use after the final argument's witness scopes. It captures the prefix
arguments in strict `let`s, once and in their original order, then retains the
known source callee through the witness scopes. Tail-group discovery uses the
same transformation, allowing the worker to emit a `musttail` call.

Admission uses actual source parameter counts. Genuine producers returning
functions, overapplications, user aliases, qualified evidence calls, dependent
or erased arrows, dictionary-field workers and wide definitions keep their
existing lowering. The change does not alter proof checking, inline-LLVM
containment or the restrictions on wide primitive closures and tail groups.
The regression's `U64` accumulator uses the ordinary word ABI.

The runtime probes are maintained as source:

- `programs/Run.NestedArgumentTail.cinch` checks the exact 64 KiB recurrence on
  an 8 MiB stack; the unchanged source fails with exit 73 through the old emitter.
- `programs/Run.NestedArgumentTailSmall.cinch` runs the same recurrence at a
  depth that both emitters complete, allowing ordinary allocation comparison.
- `programs/Run.ScopedArgumentOrder.cinch` checks visible prefix mutations,
  single execution, shared user aliases and producers returning functions.

The two semantic regressions also belong to `compiler/tests/tail_calls.rs`.
`tooling/verify_scoped_argument_codegen.py` compares unchanged programs through
two explicitly selected staged emitters using a matched interface/object/runtime
set. It checks the specific recursive worker's tail instruction and executes
normal 32 MiB and poisoned-free 256 MiB modes with zero leaks. Equal-length
executable names keep complete-process allocation totals comparable; poisoned
allocation totals are not ordinary live-space measurements.

The pinned full bootstrap chain must pass independently before this compiler
change is integrated. Targeted emitter results alone do not establish the
stage-2/stage-3 fixed point or the full preserved target baseline.

`scoped-argument-tail-results.json` records the restored-pin native fixed point:
stages 2 and 3 have identical executables, LLVM and all 51 canonical interfaces;
the interface consumer and changed-source diagnostic rebuild also passed.
The final stage-3 comparison retains the 64 KiB tail worker and completes both
ownership modes with the exact result and zero leaks. At the short depth, the
ordinary run allocates 168 blocks and 6962 bytes versus 200 blocks and 8498
bytes through the old emitter. This is an allocation comparison, not a timing
claim. Rust unit tests and the preserved target baseline remain separate gates.
