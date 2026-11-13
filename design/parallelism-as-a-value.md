# Parallelism as a value

Status: implemented, 2026-11-13. Supersedes the surface of
`design/parallel-evaluation.md` sections 2.1 and 4.1: the three
compiler-known builtins `parBoth`, `parTabulate` and `parMap` become the
methods of one kernel signature, `Parallelism`, whose host instance is the
implicit one. A program can supply its own instance without changing a call
site or the compiler.

## 1 The interface

```text
signature Parallelism where
  both    :: forall a b e1 e2.
             (Unit ->{|e1} a) ->{|e2} (Unit ->{|e2} b) ->{|e1} (a, b)
  tabulate :: forall a e. Nat -> (Nat ->{|e} a) ->{|e} Array a
  mapArray :: forall a b e. (a ->{|e} b) -> Array a ->{|e} Array b
  workers :: Nat -> Unit
```

`both` keeps the two-branch scheme the builtin had: each branch gets its own
row variable, because rows unify by equality and one shared variable would
refuse a total branch beside a diverging one, which the defining equation
`(f (), g ())` admits.

The host instance is `HostParallelism`, declared implicit. Its four methods
lower to today's `cinch_par_both`, `cinch_par_tabulate`,
`cinch_par_map_array` and `cinch_par_set_workers`. Nothing else in the
compiler names those entries any more: `Bootstrap.RuntimeIr` lowers the
primitive operations the instance's methods elaborate to, and the runtime
ABI is no longer part of any surface declaration.

A container-generic surface sits above the primitive:

```text
parTraverse :: forall t a b e. using (trav :: Traversable t, par :: Parallelism)
             => (a ->{|e} b) -> t a ->{|e} t b
parFoldMap  :: forall t m a. using (trav :: Foldable t, mon :: Monoid m, par :: Parallelism)
             => (a -> m) -> t a -> m
```

`parTraverse` is the applicative traversal over the `Parallel` applicative:

```text
parTraverse f xs = traverse (\x -> MkParallel (\() -> f x)) xs
```

where `MkParallel` wraps a computation that may run concurrently with its
siblings and the `Parallel` applicative's `lift-a2` is `both`. `parFoldMap`
is the same walk at `Monoid`. Both are written once and work for every
container with a `Traversable` or `Foldable` instance; there is no
per-container code in the compiler.

## 2 Where it lives, and why

Both declarations are **kernel declarations** installed by the checker
(`Bootstrap.Builtin.parallelism-declarations`), not a new prelude module.
The reason is a hard constraint, not a preference: the embedded base library
is pinned. `bootstrap/library-provider.lock.json` fixes a `revision` and a
`source_sha256` over the whole `compiler/prelude` tree, and the ladder's
`library_provider_lock()` refuses to run when
`Bootstrap.RuntimeAbi.cinch` or the pinned source hash disagree. Adding
`Category/Parallelism.cinch` to `compiler/prelude` would change that hash and
require a lock bump, which this change must not do.

The kernel already owns exactly this kind of vocabulary: `builtin-declarations`
carries `withScope`, `spawn`, `joinAllOrCancel`, the `Io r` operations and the
`par*` builtins as `DExternal` declarations with compiler-owned schemes. The
`Parallelism` signature, its four method declarations and the host instance
join that list, so a program sees them without importing anything and the
pinned library stays byte-identical.

`Data.Array` is not in the embedded family either (a program ships it beside
its entry), which is why `mapArray`'s scheme names `Array` without a defining
module: the type is primitive from ABI 6 (`array-type-name`).

## 3 The rule keying decision

The three safety rules were keyed on the builtin *name*. They are now keyed
on the **signature method identity**: the pair `(owner, slot)` the checker
records for a method, where `owner` is the `Parallelism` class identity and
`slot` is the method's position in it.

Why that identity and not something else:

- It is decided **before instance selection**. `parallel-callback` runs while
  the call is being elaborated, when the receiver is still a `CMethod` whose
  evidence is an unresolved `CENeeded`. The rule therefore reads the class
  and slot the *declaration* fixed, not the dictionary the resolver will
  later choose. That is the whole point: a caller's captures are examined
  whatever instance answers.
- It cannot be forged by declaring a like-named instance. An instance names
  its class by a checked type (`Parallelism`), and
  `checked-source-instances` refuses a factory whose target has no defining
  signature identity (`source instance target has no defining signature
  identity`). A user writing `implicit instance Sneaky :: Parallelism` adds a
  dictionary, not a new owner: its methods keep `owner = Parallelism` and the
  same slots. There is no way for user source to declare a method with a
  different owner while claiming the same name, because `DMethod` identities
  are assigned from the class's own member list (`source-class-methods`
  numbers slots from 0 in declaration order).
- It is stable across the two spellings. A call `both f g` elaborates to
  `CMethod sp ty "both" "Parallelism" 0 ...`; the instance's own body reaches
  `cinch_par_both` through the primitive operation, so the rule table has one
  key and the lowering has another, and neither can drift into the other.

The rules themselves are unchanged in strength. Re-read from the code:

| rule | where | keyed on |
| --- | --- | --- |
| callback form (literal lambda, global, partial application of one) | `Bootstrap.Infer.parallel-callback` | `(Parallelism, slot)` |
| parallel-safe row (closed `{}` or `{Diverge}`) | `Bootstrap.Infer.parallel-safe-row` | `(Parallelism, slot)` |
| captures (`Share`, one `%1`/`%aff` per `both` branch, no memo cell, no `Mut`) | `Bootstrap.Typing.par-definition-error` | the primitive op the instance body elaborates to |

The capture rule is the one that keys on the *lowering* rather than the
method, and that is deliberate. It runs at the definition's end, after
elaboration, over checked Core; by then the method call has been resolved to
the instance's own method body, and the primitive `par-both` /
`par-tabulate` / `par-map` node is what carries the callback shape the rule
needs (which argument is the callback, whether the callback is one-shot). The
`CMethod` is gone from the term at that point in the resolved form. Keying it
on the method instead would mean running it before resolution, where the
callback's own body is not yet available. So the capture rule keys on the
operation the host instance produces, and a *user* instance produces no such
operation at all: its body is ordinary checked code, and ordinary checked
code already obeys the usage and `Share` rules of `design/reclamation.md`.
That is the correct asymmetry, not a hole: the capture rule exists because
`par-both` runs a callback on another thread, and only the host instance does
that.

## 4 Why concurrency is sound for a traversal

`Traversable`'s law constrains the **order** in which the applicative's
effects are combined: `traverse` must combine left to right, so a
short-circuiting applicative reports the first failure in traversal order.
The parallel-safe row rule already admits only the closed empty row or
`{Diverge}` for a parallel callback. Under a parallel-safe row there is no
observable effect order to preserve: nothing in `{}` or `{Diverge}` records
which branch ran first, and `Diverge` is admitted precisely because a
diverging branch's timing is already unobservable in the sequential reading.

So executing the applicative's branches concurrently cannot violate the
traversal law. **The existing restriction is the licence for this design.**
The rule set does not weaken and no new safety argument is needed: the row
rule that made `parBoth` sound is exactly what makes `traverse` sound at the
`Parallel` applicative. This is the reason the reshape is not an escape
hatch.

Note what this does *not* say: `Parallel` is not a general `Applicative` a
user may instantiate with arbitrary effects. `runParallel` and `MkParallel`
are ordinary source, but a `parTraverse` call site is only accepted when the
callback's row is parallel-safe, so the concurrent reading is always the one
whose order is unobservable.

## 5 Instance selection

Selection is the ordinary dictionary path, with no parallel-specific
machinery:

1. `both` in scope is a `MkMethodBinding` (owner `Parallelism`, slot 0),
   registered by `register-source-methods` from the kernel signature.
2. Using it builds `CMethod ... (CENeeded ...)` with the requirement
   `Parallelism`.
3. `resolve-dictionary-term-with` resolves that evidence against
   `env-instances`, which holds every `implicit instance`. The host instance
   is there because the kernel declares it implicit.
4. `Bootstrap.RuntimeIr.static-instance-method` sees a single `CEInstance`
   with no context and no strict value field, so the projection collapses to
   a direct call to the instance's method unit at the method's own
   representation. The dictionary block is not built and no indirect apply
   survives (the property `compiler/tests/rc_specialisation.rs` already
   pins for static dictionaries).

A user instance takes the same path. A user instance *with* a context (say
`Parallelism` given `Traversable t`) keeps the projection, which is correct
and costs one indirect call per method use.

## 6 What a user-supplied pool costs

- **Nothing at the call site.** `both f g` is unchanged. The instance is
  implicit, so no runtime is named.
- **One indirect call per method use** when the instance has a context or a
  strict value field; zero when it is static, because the projection
  collapses to a direct call.
- **A dictionary block** for an instance that is not static: one immortal
  static block holding the method closures, which the existing
  static-dictionary rules already sweep when nothing reads it.
- **No recompilation of callers.** A different instance is a different
  dictionary; the call site's Core is identical.
- **The captures are still checked at the call site**, so a user instance
  cannot smuggle a `Mut` borrow or a non-`Share` capture into a concurrent
  branch. That is the invariant the whole design rests on.

A serial instance (`SerialPool`, the test's second instance) is the cheapest
possible demonstration: `both f g = (f (), g ())`, `workers n = ()`, and the
program's answers are identical because parallelism was always permission and
never semantics.

## 6.1 The `Share` limit, measured

Parallelism inherits the capture rule, and the capture rule inherits the
`Share` judgement. Measured 2026-11-13 under the seed, by element type,
`parMap` accepts `Nat`, `(Nat, Nat)`, `List Nat` and a local four-`Nat` ADT,
and refuses `String`, `(String, Nat)`, `Json` and `(String, Json)`.

`String` is `StrBuf :: Bytes -> String` behind a module-private constructor,
and `Bytes` is transfer-only (`design/reclamation.md`), so no `String`-bearing
element type is `Share`. This is the real reason a phase that carries names
cannot traverse in parallel today. **The container choice does not fix it:**
`parTraverse` at any `Traversable` instance inherits the same restriction,
because the restriction is on the captured *values*, not on the container.
A generic surface widens which shapes can be walked; it does not widen which
element types may cross a thread.

## 7 What this does not buy

- **Schedulers.** Choosing *when* a branch runs is not a dictionary
  question. A real scheduler needs to suspend a computation and resume it,
  which is the effect and task story (`design/green-threads.md`, the
  `withScope`/`spawn`/`joinAllOrCancel` surface), not an interface method.
  `Parallelism` says *who runs the branches*; it does not say *how a branch
  yields*.
- **Green threads.** Same reason. A user instance can be a thread pool, a
  serial loop, or a deterministic test scheduler that runs branches in a
  fixed order, but it cannot make a computation that does not terminate
  cooperatively yield.
- **Cancellation.** `joinAllOrCancel`'s cancellation story is a scope
  property (`Cancel r`), not a dictionary one.
- **A different parallelism *semantics*.** The law of the interface is that
  the result equals the sequential reading. An instance that computed
  something else would be unlawful, and the row rule that admits only `{}`
  or `{Diverge}` is what keeps that law meaningful. `Parallelism` is a
  *scheduling* interface, not an effect interface.
- **A lock bump.** The interface is kernel-installed precisely so the pinned
  library hash is untouched.

## 8 Migration

`parMap` does not survive as a second path. `parTraverse` at the array
instance replaces it: `parMap f xs` becomes
`array-to-list (parTraverse f (array-from-list xs))`, and the array
`Traversable` instance is where the `cinch_par_map_array` fast path lives, so
the specialization is a property of the container instance rather than of a
hard-wired builtin. The in-tree call sites that moved are listed in the
change's report.

`parTabulate` stays: it is `Nat`-indexed, not a container operation, and
`Traversable` has nothing to say about it.

## 9 ABI

`runtime_abi` stays at 9. The lock's `runtime_abi` field is untouched, and
the `cinch_par_*` entries are referenced only by the host instance's method
bodies. The ABI number did not have to move because the runtime's entry
points did not change: the reshape is entirely above them.
