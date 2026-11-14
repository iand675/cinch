# Compiling continuation-passing code to direct code

Status: implemented in the Rust seed (`compiler/src/simplify.rs`,
`compiler/src/codegen.rs`, `compiler/src/stack.rs`). Conformance entry D430.
Evidence: `compiler/tests/cps.rs`.

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
heap when they do not escape. Five rules do the work. Each is local,
budgeted, and can be switched off for measurement.

| # | Rule | Where | Switch |
| --- | --- | --- | --- |
| 1 | Guaranteed tail calls through unknown closures | codegen | (64-bit native targets) |
| 2 | Contify a tail-only local continuation into a join point | simplifier | `--no-contify` |
| 3 | Project out of a known capturing dictionary or polytype bundle | simplifier | `--no-direct-dict-calls` |
| 4 | Call-pattern specialisation on known closures, units and records of functions | simplifier | `--no-specialise-arguments` |
| 5 | Frame-allocate a continuation a non-tail callee consumes | `stack.rs` | `--no-stack-alloc` |

## 1. Guaranteed tail calls through unknown closures

The problem. A self tail call was already a jump, and a saturated known
tail call with a matching signature was already `musttail`. A tail call
through an unknown closure, `k (s + x)`, was neither. It went through
`__apply`, an ordinary C call, so every continuation in a chain kept a
native frame. At `-O0` a CPS sum over ten million elements ran out of
stack. At `-O2` LLVM's sibling-call optimisation sometimes rescued it,
but nothing guaranteed it.

The design follows GHC's LLVM backend, which places info tables next to
code. Each unit in the *tail set* gets a private `tailcc` body `X#cc`
behind a C-ABI wrapper `X` with the original name, so every existing
caller, export and stub keeps working. Each apply stub `X.app` carries
LLVM prefix data:

```
define i64 @"X.app"(...) prefix <{ i64, i64 }> <{ i64 MAGIC, i64 ptrtoint (ptr @"X.tapp" to i64) }>
```

`X.tapp` is the stub's tail twin. It has the same merge and saturation
logic, but its exact-saturation path is `musttail call tailcc @"X#cc"`.

A tail call through an unknown closure in a `tailcc` caller works as
follows:

1. Store the arguments (1 to 16) in the thread-local
   `@cinch.tail.argv [16 x i64]`. A `musttail` call must have the caller's
   signature, so the variable-length arguments cannot travel on the stack.
2. Load the closure's code word.
3. Compare the word at `code - 16` with the magic. On a match, `musttail
   call tailcc` the twin at `code - 8`.
4. Otherwise take the slow path: copy the arguments to a local buffer
   and call `__apply` like any other call.

The twin copies the buffer into its own frame before it does anything
else. A further tail call reuses the buffer, so it must not be read after
that point.

The tail set's seeds are units with a saturated tail call whose head is
not a global, constructor, primitive or foreign function. The set is
closed over the undirected known-tail-call graph, so every edge between
two members is a `musttail` between two `tailcc` bodies. A component is
skipped whole if any member has a narrowed signature or a tail-width
entry, which have their own calling conventions. The rule is off on
32-bit targets and wasm, whose `musttail` support differs.

Measured: the CPS sum over ten million elements (`tests/cps.rs`, which
uses two million) completes at `-O0` in constant stack and leak-free. At
`-O2` it takes the same time as before: the continuation chain itself is
the cost, one closure per element, built once and called once.

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

The lambda's body is copied with fresh binders. The captures are
substituted through staged locals, and the parameters become join
parameters with their classes. The result is `Join { params, body, rest
}`, with each `k a` in `rest` replaced by a `Jump`.

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
specialisation and inlining off so the closure survives: 1,155 ms with
heap closures and 922 ms with frame closures. With every rule on, rule 4
removes the closure entirely and the loop takes 4 ms. That ordering is
the point of the design: specialisation removes what it can see, and the
frame keeps the rest off the heap.

## Interactions

- **Ordering.** Specialisation (4c) runs before inlining in each
  simplifier iteration. Contification (2) and the projection rules (3,
  4a, 4b) run in every pass. Frame allocation (5) runs after RC
  insertion. `tailcc` (1) is decided at code generation, from the final
  Core.
- **Occurrence counts.** A rewrite can create a binder that the pass's
  occurrence table has not counted. A missing count now means "unknown",
  never "dead", so dead-binder elimination cannot drop such a binder.
- **Code size.** Rule 4 is the only rule that copies. Its budget, and the
  cache by shape, bound the growth. A callee above the size bound is never
  copied.
- **Leak accounting.** Every test in `tests/cps.rs` runs under
  `CINCH_LEAK_CHECK=1`. A rule that loses a release or releases twice
  fails as a live count or a trap.

## References

- Andrew Kennedy, "Compiling with Continuations, Continued", ICFP 2007.
- Luke Maurer, Paul Downen, Zena M. Ariola, Simon Peyton Jones, "Compiling
  without Continuations", PLDI 2017.
- Simon Peyton Jones, "Call-pattern specialisation for Haskell programs",
  ICFP 2007.
- David Terei, Manuel M. T. Chakravarty, "An LLVM Backend for GHC",
  Haskell Symposium 2010 (tables next to code via prefix data).
