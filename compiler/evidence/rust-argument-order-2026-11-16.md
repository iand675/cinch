# Strict application-order regression

The original Rust CLI reads an excess argument before entering a saturated
producer that returns another function. The preserved scoped-argument
fixture's direct and shared partial-application checks pass; its returned
function check fails. A separate observable control proves that the producer
executes exactly once, but too late: the result is 775 with the byte changed
to 4, instead of the required result 1031 with the byte changed to 4.

The baseline executable is
`/workspace/dasl-stdlib-io/build/prim-wide-frozen/cinch`, SHA-256
`f2d5ae4c84e8b69b1cfc7edab79c937f15e1ccaab53f9f27f1d95d926b7e451e`.
Every baseline build uses `--opt-level 0 --rc-debug`. Every run uses
`CINCH_HEAP_LIMIT=32M CINCH_STACK_SIZE=8M CINCH_LEAK_CHECK=1 CINCH_RC_DEBUG=1`.
All six baseline runs report `cinch-leak: live=0 immortal=0 leaked=0`.
The JSON beside this report retains individual source hashes and results.

The checked Core still declares `Main.returned` with two parameters, but
its call is a single three-argument spine. Emitted LLVM evaluates
`cinch.runtime.bytesGet` before passing all three words to `__apply`.
That stub eventually calls the producer, after the old byte has been read.
The cause precedes LLVM optimization and is independent of the Native
frontend's separately repaired argument lowering.

The correction stages application spines before wide-edge insertion and
ownership planning. A known unit collects arguments up to its actual
runtime parameter count; its saturated call completes before excess
arguments run. The arity of a returned or otherwise unknown function value
is not determined by its arrow count, so each following application is a
separate stage. Known saturated and partial direct calls retain their
original shape. Captured lambdas subtract their environment size from their
unit's parameter count. Inspection emission applies the same idempotent
staging when it receives checked Core directly.

The driver and simplifier only unfold calls with the exact declared runtime
arity (`core.rs::Driver::try_unfold` and `simplify.rs::inline_unit`);
overapplied calls do not have their excess arguments rebound by those
rewrites. The new executable regression nevertheless needs both O0 and O2
validation to cover the complete optimization path.

`Run.ReturnedArgumentOrder.cinch` independently selects two functions with
the same type and different runtime arities. It also checks two successive
returned-function boundaries, a computed prefix argument that mutates an
alias, and an explicitly effectful producer. With the original CLI its exit
code is 13: the early-return selection, successive boundaries, and computed
prefix fail; the saturated arity-three and explicitly effectful controls
pass. The expected corrected exit code is 0.

Unknown multiargument calls now preserve strict order by building their
partial applications explicitly. They can allocate more than the previous
incorrect batched application. Static specialization and known saturated
calls keep their existing paths; eliminating allocations for genuinely
unknown runtime arities requires an additional guarded batching optimization
that preserves every saturation boundary.

The first patched Rust CLI, SHA-256
`f81761c729b69a0aca8aabaecd6d1f94fee66e46c71621c99f1c451b825809ba`,
passes both `Run.ScopedArgumentOrder.cinch` and
`Run.ReturnedArgumentOrder.cinch` at O0 and O2. All eight runs exit 0 with
zero leaks: each executable runs with the normal 32 MiB heap and 8 MiB
stack, then RC poisoning with a 256 MiB heap and the same 8 MiB stack. The
fresh returned-function fixture also passes both modes through the original
frozen Native frontend. The parent's coordinated Rust `tail_calls` target
passes all ten cases, including the preserved deep constant-stack controls.
Compilation of a short CSV overlay overlapped the first fixture compilation
on CPU3; these checks make no timing or performance claims.
