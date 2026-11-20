# Compiling continuation-passing code to direct code

Status: implemented in the Rust seed (`seed/src/simplify.rs`,
`seed/src/codegen.rs`, `seed/src/stack.rs`). Conformance entries
D430 and D431. Evidence: `seed/tests/cps.rs`,
`seed/tests/calling_convention.rs`.

Continuation-passing style is how Cinch code is written when it needs
early exit, backtracking, resumable parsing, or a monad over `forall r.
(a -> r) -> r`. The same shape shows up in the "record of functions" idiom:
a value of hooks (`before`, `after`, a logger, a metrics sink) is threaded
through a loop so it can be instrumented without changing the loop.
Written naively, both cost a closure allocation and an indirect call per
step, plus a native stack frame per continuation that is never popped.

The goal is for code written this way to compile to the loop it means:
known continuations called directly or folded away, unknown ones
tail-called in constant stack, and the closures that survive kept off the
heap when they do not escape. Six rules do the work. Each is local,
budgeted, and can be switched off for measurement.

| # | Rule | Where | Switch |
| --- | --- | --- | --- |
| 1 | Guaranteed tail calls through unknown closures | codegen | (64-bit native targets) |
| 2 | Contify a tail-only local continuation into a join point | simplifier | `--no-contify` |
| 3 | Project out of a known capturing dictionary or polytype bundle | simplifier | `--no-direct-dict-calls` |
| 4 | Call-pattern specialisation on known closures, units and records of functions | simplifier | `--no-specialise-arguments` |
| 5 | Frame-allocate a continuation a non-tail callee consumes | `stack.rs` | `--no-stack-alloc` |
| 6 | Return a constructed product in registers | codegen | `CINCH_NO_CPR=1` (64-bit native targets) |

## 1. Guaranteed tail calls and the internal convention

The problem. A self tail call was already a jump, and a saturated known
tail call with a matching signature was already `musttail`. A tail call
through an unknown closure, `k (s + x)`, was neither. It went through
`__apply`, an ordinary C call, so every continuation in a chain kept a
native frame. At `-O0` a CPS sum over ten million elements ran out of
stack. At `-O2` LLVM's sibling-call optimisation sometimes rescued it,
but nothing guaranteed it.

### Which units run under `tailcc`

On a 64-bit native target the internal convention is LLVM's `tailcc`
wherever a tail call needs it, and the C convention everywhere else. The
units under `tailcc` are exactly the units whose tail calls are
guaranteed (`codegen.rs::tail_units`), the greatest fixed point of:

- it makes a saturated tail call through a closure it does not know (on
  a target with apply twins); or
- it makes a saturated known tail call to another member, a function
  unit of the same module whose result crosses like its own (the same
  word or narrowed type, or the same product, section 6).

A cycle of known tail calls (mutual recursion) keeps all of its members.
A tail call to a unit outside the set is an ordinary call: the callee
makes no tail call the set has to guarantee, and the tail calls among
units outside the set form no cycle, so a chain of them is bounded by the
program rather than by its input. This is what keeps a tail call into a
small leaf, a class method that adds or compares, from moving the leaf
under `tailcc`: on x86-64 every ordinary call into it would then pay the
callee-pops adjustment below.

Such a unit's body is `X#cc`, a `tailcc` function with the unit's own
linkage, behind a C wrapper `X` with the unit's exact boundary (narrowed
positions included). Every call inside the module goes to the body; the
wrapper is the unit as the C side, another object, the runtime and the
JIT see it, so the object ABI does not change. A call to a unit another
module defines goes to its C symbol and is an ordinary call.

`musttail` under `tailcc` does not compare parameter lists, only the
result, so every tail call between two bodies is guaranteed whatever the
arities and narrowings. That retired two pieces of the C convention's
machinery on these targets: the closed tail set (a component of the
tail-call graph was skipped whole when any member narrowed), and the
widened `#tail` workers a recursive group of mixed arities lowered under.
A tail group's narrowing now only needs the members' results to agree.

Why not `tailcc` for every unit. On x86-64 `tailcc` is callee-pops: every
ordinary call into a `tailcc` function is followed by a `sub $8, %rsp`,
and its returns are `ret $8`. Putting every unit under it cost `fib 35`
4.9% more instructions at `-O2` and 7.6% at `-O0`, and a loop calling a
comparison method 5.9%. A unit outside the set keeps the C convention, and
a program without guaranteed tail calls emits the code it did before. An
apply twin that enters a C unit makes an ordinary call, which adds one
frame that the next call does not grow. AArch64's `tailcc` costs nothing
over the C convention, so the split only matters on x86-64; it is kept on
both for one rule.

Mach-O targets (`*-apple-darwin`) keep `tailcc` and every guaranteed
known tail call, but not the unknown-tail-call fast path below: the
prefix data holds the twin's address as an absolute integer in a code
section, which Mach-O can only encode as a text relocation, and the linker
refuses those. There an unknown tail call takes the ordinary `__apply`
call, and no unit joins the `tailcc` set for making one.

WebAssembly has no `tailcc` (LLVM's backend accepts only the C
convention), and 32-bit targets keep the C convention too. There the
closed widening of section D104 still applies: mixed-arity groups lower
under their widest arity in `#tail` workers, and a group whose members
narrow to different prototypes keeps the word.

### Unknown tail calls

The design follows GHC's LLVM backend, which places info tables next to
code. Each apply stub `X.app` carries LLVM prefix data:

```
define i64 @"X.app"(...) prefix <{ i64, i64 }> <{ i64 MAGIC, i64 ptrtoint (ptr @"X.tapp" to i64) }>
```

`X.tapp` is the stub's tail twin. It has the same merge and saturation
logic, but its exact-saturation path is `musttail call tailcc @"X#cc"`
when `X` runs under `tailcc`, and an ordinary call otherwise.

A tail call through an unknown closure in a `tailcc` caller works as
follows:

1. Load the closure's code word and compare the word at `code - 16` with
   the magic. On a match, `musttail call tailcc` the twin at `code - 8`.
2. The twin takes the closure, the count and the first `tail_regs`
   argument words in registers: four on x86-64 (`tailcc` there has six
   integer argument registers) and six on AArch64 (eight). One signature
   serves every call; unused register words are `poison`, so the
   per-count fan-out of `caml_applyN` does not come back.
3. Words beyond the registers (up to 16 in all) go in the thread-local
   `@cinch.tail.argv` at their own indices. A `musttail` callee cannot
   read the frame it replaced, so they cannot travel in the caller's
   frame.
4. Otherwise, a code word without the magic (an object built by an older
   compiler) takes the slow path: copy the arguments to a local buffer and
   call `__apply` like any other call.

The twin stores the register words and copies any buffer words into its
own frame before it does anything else. A further tail call reuses the
buffer, so it must not be read after that point.

The magic names the twin's signature. `CINCHTL1` was the buffer-only twin
`(ptr, i64, ptr)`; `CINCHTL2` is the register twin. A caller never enters
a twin of the other shape: an object built by the older compiler takes
the slow path.

Measured: the CPS sum over ten million elements (`tests/cps.rs`, which
uses two million) completes at `-O0` in constant stack and leak-free.
With the release runtime (see "The runtime profile" below) that sum takes
527 ms at `-O2` and 732 ms at `-O0`. Passing the words in registers took
2.9% of its instructions at `-O2` and 3.2% at `-O0` off the buffer-only
twin (valgrind instruction counts on x86-64; the wall clock on the shared
measuring machine varied more than that between identical binaries).

## 2. Contification

A local continuation that is only ever tail-called inside its own scope is
not a value. It is a block with parameters:

```
let k = \s -> wrapping-add-i64 (wrapping-mul-i64 s 3) m in
if n == 0 then k 1 else if n == 1 then k 2 else k n
```

The rule (Kennedy 2007; Maurer et al., PLDI 2017) matches `Let(k, Lam {fn_id,
env}, body)` when:

- every captured word is a value;
- every occurrence of `k` in `body` is a saturated call in tail position
  relative to the `let`;
- there is at least one such call.

The lambda's body is copied with fresh binders. The result is `Join {
params, body, rest }`, with each `k a` in `rest` replaced by a `Jump`. The
join's parameters are the captured locals followed by the lambda's own
parameters, and every jump passes the captures. Literal and unit captures
are substituted in place.

The join must be closed over counted locals. Reference counting runs after
the simplifier. It reads a join body's free locals as borrowed and keeps
them live nowhere, so a free capture would be dropped before the jump that
needs it. The first version substituted the captures in place, and a
contified continuation over a string read it after its release
(`tests/cps.rs::a_contified_continuation_passes_its_counted_captures`).
For the same reason, a call of `k` inside an inner join's *body* is not
contified: the jump would pass the captures from inside that body. Only
the continuation may call `k`.

A contified copy can expose the recursion it came from. A recursive callee
unfolds inside the copy and builds the same closure again. The pass
therefore records the lambdas whose copies it is inside, and does not
contify a lambda beneath its own copy. Each simplifier iteration then
unrolls at most one level, and the iteration cap bounds the rest
(`contifying_a_recursive_continuation_terminates`).

No closure is allocated, no indirect call is made, and the join compiles
to a basic block with phis.

The rule runs pre-order at the top of each simplifier pass, so the
inliner sees the join and not the closure. Contification is not
inlining, because the body is emitted once however many calls there are.

## 3. Projection out of known dictionaries and polytypes

A rank-N polytype is a dictionary of the anonymous `#poly.<k>` family
(design/rank-n-polymorphism.md). A generalisation site such as `Cont (\k
-> k x)` becomes an `InstDict` over its captured locals. An instantiation
is a `DictProj` of slot 0. Before this rule, `direct-dict-call` only fired
on static dictionaries, so a capturing one always took the slot call.

The rule extends it:

- A `DictProj` of an `InstDict`, either in place or through a `let`-bound
  local the scope tracks, resolves to a direct call of the method with the
  captured words as its leading arguments. The rule requires the method's
  arity to equal the captures plus the arguments, and every capture to be
  a value.
- `projection` and `known-constructor` see through a `let`-bound
  constructor and through a static record: a nullary unit whose body is a
  constructor application of static fields.
- `known-constructor-computed`, which is off by default, now fires when
  every computed argument is a pure construction (an `InstDict` or a
  value-capturing `Lam`). Such an argument costs nothing to rebuild.

The rank-2 `Cont` probe (`run (pure-c 41)`) folds to a direct call of the
final continuation.

## 4. Call-pattern specialisation and records of functions

Rules 2 and 3 handle a continuation the simplifier can see in one body.
The common case it cannot see is a known function value passed *into* a
callee: a `Hooks` record threaded through a recursive `run`, or a
continuation handed to a helper. Three parts carry the knowledge across
the call:

- **(a) Known-closure calls.** `App(Lam {fn_id, env}, args)` with
  `|env| + |args|` equal to the lambda's arity becomes `App(Global fn_id,
  env ++ args)`. The same holds through a `let`-bound local. A closure
  with no captures becomes the bare global (`closed-lambda`), which the
  inliner can unfold.
- **(b) Static records.** A nullary unit whose body is a constructor
  application of globals, closed lambdas and literals is a static record.
  `default-hooks` is one. `case default-hooks of Hooks b a -> ...` folds,
  and so does a projection from it.
- **(c) Call-pattern specialisation** (`specialise-arguments`, after GHC's
  SpecConstr, on functional arguments). It handles a saturated call
  `f a0 .. an` where some `ai` has a *known shape* and `f` uses that
  parameter in a way the shape helps.

  A known shape is one of:
  - a unit used as a value;
  - a static record;
  - a closure over values;
  - a constructor application whose fields are such shapes or values,
    with at least one function among them.

  A use that helps is any of:
  - applying the parameter;
  - `case` on it;
  - projecting from it;
  - passing it on unchanged in a self-call at the same position.

  Such a call is redirected to a copy `f.spN`. The copy keeps the original
  parameter list, with the specialised position dead (its argument is
  `()`, and its class is `Raw`). The shape's free parts (captured words and
  value fields) are appended as trailing parameters. The copy's body
  starts with `let p = <shape rebuilt from those parameters>`, where
  `known-constructor`, `projection`, `known-closure-call` and the inliner
  fold every use of `p`.

  Inside the copy, a self-call that passes the parameter through unchanged
  is redirected to the copy too. So a record of functions threaded down a
  loop is known in every iteration, not only the first.

  Copies are cached by (callee, position, shape key), so one shape gets
  one copy however many call sites use it. Copies are budgeted: at most 4
  per callee, 256 in a program, and only for callees of at most 4,000
  Core nodes. The step runs once per simplifier iteration, before the
  inliner.

Measured on the instrumentation probe: `run default-hooks 100000000 42`
with a `before` and an `after` hook. Without the rule, every iteration
makes two indirect calls through the record. With it, `run.sp0` has both
hooks inlined, and LLVM closes the loop: `main` compiles to `mov $42`. A
record built at the call site, with hooks capturing a local, gets a copy
that takes the captured word as a parameter (`tests/cps.rs`).

What the analysis does not do: a record stored in a data structure and
read back, or one that arrives through an unknown call, has no known
shape. Its calls stay indirect. That is the boundary of a static,
non-whole-value analysis; a flow analysis (0-CFA) would be the next step.

## 5. Frame-allocated continuations

A closure that survives rules 2 to 4 is usually a continuation handed to a
callee that calls it once, outside tail position, and returns: `with-twice
k x = k (x + x) + 1`. That closure need not live on the heap. The frame
pass (design/stack-allocation.md) already allocated non-escaping
constructors and closures in the frame. Three things kept continuations
out:

- **A closure as a call argument was not `let`-bound.** The pass now
  names `f (lam ...)` as `let k = lam ... in f k` when the call is not in
  tail position, the callee consumes that parameter, and the captures are
  locals, literals or dups of a local. Building the closure then performs
  only count increments, so moving it ahead of its sibling arguments
  changes no observable order.
- **Applying a closure counted as an escape.** A non-tail application now
  counts as a consumption (`applied`). In a callee's parameter summary, an
  application in tail position does too: the parameter's block lives in a
  caller's frame, which outlives the callee's tail call. So does a
  self-call that forwards the parameter to its own position, because it
  is consumed exactly when the summary says so. In the allocating unit, a
  tail application of its own frame closure stays an escape, for the
  reason in section 7.6 of the stack-allocation design.
- **The apply stub decremented the frame marker in place.** A frame
  block's count field holds `RC_FRAME`, which the stub's shared path read
  as an ordinary positive count. The stub now has a frame arm: after
  duplicating the saved words for the call, a frame closure's payload is
  released once (`cinch_frame_release`) instead of counted. A second
  release finds the marker. The tail twin shares the same code.

Soundness rests on the facts the pass already relies on, plus one fact
about borrow inference. A parameter that is applied is always owned,
because application is not a borrowed position (`rc.rs`). So a frame
closure is never lent to a callee that applies it, and the apply's release
is the only release on that path. A `Dup` of a frame closure with a
counted capture is still refused, so an applied closure never has a second
live reference that could read released children.

Measured on the `with-twice` loop over ten million iterations, with
specialisation and inlining off so the closure survives: 204 ms with heap
closures and 190 ms with frame closures (about 7%; against the unoptimised
runtime the same pair was 1,155 ms and 922 ms, because each heap closure
then cost a slow allocation and a slow free). With every rule on, rule 4
removes the closure entirely and the loop takes 4 ms. That ordering is
the point of the design: specialisation removes what it can see, and the
frame keeps the rest off the heap.

## 6. Multi-word results

The problem. A function that returns a pair, a `(value, state)` step or a
`split`, allocates the pair, returns its pointer, and the caller
immediately takes it apart and frees it. GHC removes this with its
constructed product result analysis and worker/wrapper split; Rust and
Swift return small aggregates in two to four registers.

The rule (`codegen.rs::cpr_units`). A unit is a product result when every
leaf its tail position delivers is a saturated application of one
constructor of one to three fields in the uniform word layout (or a
`Reuse` of that constructor's cell), a self tail call, a tail call of
another product result of the same constructor, a jump, or the fault
path. The set is the greatest fixed point of that condition. Three fields
is the portable bound: x86-64 returns `{ i64, i64, i64 }` in `rax`, `rdx`
and `rcx`, and a fourth word makes LLVM return through memory, which is
no better than the block.

Code generation.

- The unit's body `X#cc` returns the fields as an LLVM aggregate. A
  constructor leaf returns its argument words and builds nothing; a
  `Reuse` leaf does the same, and the kept cell is freed with the other
  tokens on the way out. A tail call of another product result is a
  `musttail` of the aggregate.
- The C wrapper `X` calls the body and hands the fields to the
  constructor's own unit, which allocates the block. The object ABI, the
  JIT, the apply stubs and every caller that needs the block go through
  it, so a product that escapes (stored, passed, captured, reached through
  a closure) is boxed exactly once, where it is first needed.
- A caller that takes the result apart never builds it
  (`CoreEmitter::virtual_let`). `let t = e in body` qualifies when `e`'s
  leaves all build the product or call a product result
  (`product_leaves`), and `body` mentions `t` only as the scrutinee of a
  `case` with the constructor's one alternative, as the block of a
  `Release`, or in a `drop`. `e` is emitted with a field destination
  (`Dest::Fields`): each leaf stores the field words into slots. The
  `case` binds its fields from the slots; the `Release` is its unique path
  without a cell (the moved fields already hold their references, the rest
  are dropped, the reuse token is 0); a `drop` drops each counted field.
  Any other mention keeps the ordinary boxed call.

The field destination is what makes the rule reach unrolled recursion.
`split` with its recursive call inlined builds the pair of the inner call
inside a `case`, and the outer `case` takes it apart: the `let`'s value is
that `case`, not a call, so a rule that only recognised calls took a
quarter of the pairs away and this one takes all of them.

A product result's body keeps the C convention unless it is in the
`tailcc` set (section 1); the aggregate return is the same under both, and
a tail call between two product results outside the set is an ordinary
call of the aggregate.

Measured, on `split` over two million elements: 6,000,007 blocks became
4,000,006 (the list and the split cells remain; every pair is gone), and
the instruction count fell 26.7% at `-O2` and 25.9% at `-O0`. A program
with no product result emits identical code. Evidence:
`tests/calling_convention.rs` (every run leak-checked, including products
that escape into a list, through a closure, through a reused cell, and a
product result tail-calling another a million deep) and
`codegen::tests::a_product_result_tail_call_passes_the_aggregate_on`.

## Interactions

- **Ordering.** Specialisation (4c) runs before inlining in each
  simplifier iteration. Contification (2) and the projection rules (3,
  4a, 4b) run in every pass. Frame allocation (5) runs after RC
  insertion. `tailcc` (1) and the product results (6) are decided at
  code generation, from the final Core; the product results first,
  because a tail call between two units is only guaranteed when their
  results cross alike.
- **Occurrence counts.** A rewrite can create a binder that the pass's
  occurrence table has not counted. A missing count now means "unknown",
  never "dead", so dead-binder elimination cannot drop such a binder.
- **Code size.** Rule 4 is the only rule that copies. Its budget, and the
  cache by shape, bound the growth. A callee above the size bound is never
  copied.
- **Leak accounting.** Every test in `tests/cps.rs` runs under
  `CINCH_LEAK_CHECK=1`. A rule that loses a release or releases twice
  fails as a live count or a trap.

## The runtime profile

The measurements above first ran against an unoptimised runtime. The
driver's `ensure_runtime` built `seed/runtime` without `--release` and
linked `target/debug/libcinchruntime.a` into every program `cinch build`
produced. The runtime crate sets no profile, so that archive was opt-level 0.
Under callgrind one allocation cost about 1,800 instructions and one free
about 1,000, because every atomic, counter and bounds check was an
out-of-line call. The same program against the release archive costs about
118 instructions per allocation and 98 per free. A 10M-element list build
and sum went from 3,563 ms to 268 ms, and the CPS sum from 7,012 ms to
608 ms. The bootstrap ladder always linked the release archive, so the
self-hosted compiler was never affected. The driver, its JIT shared library
and the tests that link the archive themselves now all use the release
profile.

## References

- Andrew Kennedy, "Compiling with Continuations, Continued", ICFP 2007.
- Luke Maurer, Paul Downen, Zena M. Ariola, Simon Peyton Jones, "Compiling
  without Continuations", PLDI 2017.
- Simon Peyton Jones, "Call-pattern specialisation for Haskell programs",
  ICFP 2007.
- David Terei, Manuel M. T. Chakravarty, "An LLVM Backend for GHC",
  Haskell Symposium 2010 (tables next to code via prefix data).
