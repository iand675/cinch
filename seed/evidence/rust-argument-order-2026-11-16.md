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

A second isolated O2 probe exposes an earlier simplifier error:
`let partial = combine (advance cell) in let changed = advance cell in
partial (read-word cell)`. The original compiler fails the expected result;
turning off `saturate-partial` makes the identical source pass. Both runs use
the original frozen CLI and the normal 32 MiB heap / 8 MiB stack with RC
poisoning and zero leaks. The final two-case `Run.SavedPartialOrder.cinch`
fixture, SHA-256
`6c70847240c8a56a619b264c875bc4c8b16708ed9e58cb4a600ad50bb2198b38`,
exits 3 with that original O2 compiler: both the single saved computation
and mixed-representation saved pair run after the intervening write.

The second correction keeps fusion but binds saved computations in their
original position and order, using the callee's instantiated parameter
classes. The saturated call receives those local values, so it still
qualifies for direct constant-stack tail lowering. Primitive heads and
polymorphic class vectors survive the merge. A nullary global reference is
a computation rather than an inert function value; the value classifier
must preserve its new binding too, or the next pre-inline pass would move
it again. Structural tests cover these ownership and instantiation facts
alongside the executable byte-alias regression.

Runtime identities are recorded separately: the Rust CLI controls use the
unchanged host release archive SHA-256
`c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac`
(archive timestamp 2026-11-15 19:14:38 UTC), frozen under the lane's
`build/rust-order-runtime-frozen` for explicit final linking. The Native
control uses its provider's frozen archive SHA-256
`80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141`.
No runtime source changes form part of either correction, and the checks
make no cross-runtime performance comparison.

The final patched Rust CLI, SHA-256
`16f2f4bf883d2e5358c3d09b2e97d0d3c43f2c6c1996da296eef922c9395d98f`,
passes all three fixtures at O0 and O2. All twelve runs exit 0 with zero
leaks, with explicit linkage to the frozen Rust runtime archive above:
normal 32 MiB heap / 8 MiB stack and RC poisoning 256 MiB heap / 8 MiB stack.
The fresh saved-prefix fixture also passes both modes through the original
frozen Native frontend. The coordinated parent compiler unit gate passes
302/302, including all five application-boundary checks. The final JSON
retains full compiler provenance, source hashes, runtime identity, and every
run's exit code and leak line.

The final integrated Rust gate passes all 489 checks: 302 compiler unit
checks, 45 simplifier checks, 11 tail-call checks, 29 primitive checks, 10
wide-primitive checks, 18 wide-scalar checks, and the Base32 (14), Base64
(12), binary (13), CSV (11), HTML (7), path (9), and URL (8) consumers.
This includes the saved-prefix regression, all four new simplifier
controls, and the preserved deep constant-stack tests. Raw gate logs are
frozen and hashed in the JSON receipt.
