# Effect.Stm

Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Stm</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Stm/source/">View source</a><a href="/packages/search/?q=%2BEffect.Stm">Search module</a><a href="/reference-sources/Effect.Stm.cinch" download="Effect.Stm.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Stm: software transactional memory.

A `TVar a` is a shared variable. An `STM a` is a description of reads
and writes of such variables, and `atomically` runs one so that the
whole description takes effect at a single instant or not at all:
no other transaction sees some of its writes without the rest, and
it never sees some of another's.

  move :: using (seq :: StmSequencing) => TVar Nat -> TVar Nat -> Nat -> STM Unit
  move src dst n = do
    a <- tvar-read src
    if n <= a then do
        _ <- tvar-write src (a - n)
        tvar-modify dst (\b -> b + n)
      else retry

`retry` abandons the attempt and waits until a variable it read has
changed (here, until `src` holds enough), and `or-else` tries a
second transaction when the first would wait. A transaction is a
value built from pure functions, so it cannot perform I/O, call
foreign code or wait on anything but its variables: a body that tried
would have an effect row, and a transaction's steps are pure
functions of the values before them.

# What is guaranteed

* Atomicity and isolation: committed transactions are serializable,
  in the order of their commit versions.
* Opacity: an attempt that will not commit still only ever reads a
  snapshot some serial order could have produced. A body therefore
  cannot loop or go wrong on a combination of values that never
  existed together.
* No lost wakeup: a transaction waiting in `retry` runs again after
  any commit that writes a variable it read.

# What is not

No fairness and no bound on attempts. A long transaction can be
overtaken indefinitely by short ones that write what it reads, and a
`retry` whose variables nobody writes waits forever, which is why
`atomically` carries `Diverge`. A transaction may run many times, so
what it allocates on an abandoned attempt is released and nothing an
attempt does is visible unless it commits.

# The algorithm

Versioned write locks with a global version clock (TL2, Dice, Shalev
and Shavit 2006), with snapshot extension. Each variable's cell has a
lock word: the version of the commit that last wrote it, a write-lock
bit a committing transaction holds, and a pin bit a reader holds for
the few instructions it takes to count a reference.

* An attempt starts by reading the clock: its snapshot version.
* A read takes the lock word, the value, and the lock word again. The
  value belongs to the snapshot when the word did not change and its
  version is no newer than the snapshot. A newer version does not
  abandon the attempt at once: the attempt reads the clock again and
  checks that nothing it has read so far changed, which moves its
  snapshot forward to now.
* A write stages the value in a private cell and touches nothing
  shared.
* Commit locks the written variables, advances the clock, checks the
  read set against the snapshot (skipped when no other commit
  happened since it was taken), installs the staged values and
  releases the locks with the new version. A read-only attempt
  commits with no shared write at all.

Nothing ever blocks while holding a lock: a lock that is not free
after a bounded spin abandons the attempt, which backs off (a random
spin, then yielding the processor) and runs again.

# Values and reference counts

A value in a variable is an ordinary counted value, and the cell owns
one reference to it. Two rules keep the counts right when the value is
replaced under readers (the runtime's `stm.rs` states them in full):

* A reader takes its own reference to a counted value while it holds
  the cell's pin bit, and a commit replaces a value only while it
  holds the write lock, which excludes the pin. A value that needs no
  count (a machine scalar, a small `Nat` or `Int`) is read with no
  shared write at all.
* A commit swaps each staged value with the value it replaces, so the
  old value leaves in the staging cell and is released with the
  attempt's log, after the locks are gone. Before the swap it marks
  the new value as shared when the variable is shared (the runtime's
  mark-on-spawn rule, applied at publication).

The log holds its cells with their value types forgotten, since one
list holds variables of every type. `cell-cast` and `cell-open` are
the two unchecked steps, and the invariant that makes them sound is
local: a staged cell is paired with the variable it was written to,
and is only ever opened through that variable's type.

# Target specifics

The atomic operations are inline LLVM IR, so each target gets its own
instructions (a `lock cmpxchg`, a `casal`, or the load-linked pair).
Two operations have no portable spelling and carry a body per
architecture: the spin hint (`pause`, `isb`) and the cycle counter
that randomizes backoff (`rdtsc`, the virtual counter). Hardware
transactional memory is deliberately absent: Intel has disabled TSX
on shipping processors and no Arm core implements TME, so a commit
path through it could not be tested and would rarely run.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and the `STM` label marks code that runs
transactions. The class instances for `STM` live here with the type.

</details>

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-STM">STM</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">STM</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-StmSequencing">StmSequencing</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">StmSequencing</strong></code></li><li><a class="syn-name" href="#v-atomically">atomically</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atomically</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-atomically-or-cancel">atomically-or-cancel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atomically-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-check">check</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">check</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-or-else">or-else</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-retry">retry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">retry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-stm-bind">stm-bind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stm-bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-stm-map">stm-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stm-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-stm-pure">stm-pure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stm-pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-stm-then">stm-then</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stm-then</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-tvar-modify">tvar-modify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-modify</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-tvar-new">tvar-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tvar-new-io">tvar-new-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tvar-read">tvar-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tvar-read-io">tvar-read-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-read-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tvar-state">tvar-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-state</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>)) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-tvar-swap">tvar-swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tvar-write">tvar-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tvar-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-STM">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">STM</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-STM" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L402" title="View source at line 402">Source</a></span></div>
<div class="doc-body">

A transaction: a function from an attempt to its outcome. The row
says the function reads shared cells; it writes none before commit.

<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li></ul></details>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-StmSequencing">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">StmSequencing</strong></code><span class="doc-sig-links"><a href="#c-StmSequencing" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L865" title="View source at line 865">Source</a></span></div>
<div class="doc-body">

The `do` provider (decision D22: `do` picks its provider lexically,
from a `using` binder with a provider head). A definition that
declares `using (seq :: StmSequencing)` writes its transaction as a
block:

  move :: using (seq :: StmSequencing) => TVar Nat -> TVar Nat -> STM Unit
  move src dst = do
    n <- tvar-read src
    _ <- tvar-write src 0
    tvar-modify dst (\m -> m + n)

A statement is a transaction, `<-` names its result, and the block
is the transaction that runs them in order. The block's
continuations run once per attempt, so what a statement captures is
used any number of times: a `%1` value cannot cross a `<-`.

<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#c-StmSequencing" title="Effect.Stm.StmSequencing">StmSequencing</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-atomically">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atomically</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-atomically" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L716" title="View source at line 716">Source</a></span></div>
<div class="doc-body">

Run a transaction: its reads and writes take effect together, at one
instant, or the transaction runs again. Returns when an attempt
commits, which `retry` and contention can postpone without bound.

</div>
</article>

<article class="doc-entry" id="v-atomically-or-cancel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atomically-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-atomically-or-cancel" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L738" title="View source at line 738">Source</a></span></div>
<div class="doc-body">

`atomically` for a task that must stay cancellable while it waits:
`None` when the task's cancellation was requested while the
transaction was waiting in `retry`, with nothing committed. A
request does not interrupt an attempt that is running, and a
transaction that never waits never observes it.

</div>
</article>

<article class="doc-entry" id="v-check">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">check</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-check" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L788" title="View source at line 788">Source</a></span></div>
<div class="doc-body">

Wait until the condition holds: `retry` when it does not.

</div>
</article>

<article class="doc-entry" id="v-or-else">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-or-else" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L779" title="View source at line 779">Source</a></span></div>
<div class="doc-body">

`first`, unless it would wait: then its writes are discarded and
`second` runs in its place. If both would wait, the transaction
waits on what either of them read.

</div>
</article>

<article class="doc-entry" id="v-retry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">retry</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-retry" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L773" title="View source at line 773">Source</a></span></div>
<div class="doc-body">

Abandon the attempt and run the whole transaction again once a
variable it has read changes. Inside `or-else`, give way to the
alternative instead.

</div>
</article>

<article class="doc-entry" id="v-stm-bind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stm-bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-stm-bind" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L751" title="View source at line 751">Source</a></span></div>
<div class="doc-body">

`m`, then the transaction `k` makes of its result, as one
transaction.

</div>
</article>

<article class="doc-entry" id="v-stm-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stm-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-stm-map" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L757" title="View source at line 757">Source</a></span></div>
</article>

<article class="doc-entry" id="v-stm-pure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stm-pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-stm-pure" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L746" title="View source at line 746">Source</a></span></div>
<div class="doc-body">

The transaction that does nothing and answers `x`.

</div>
</article>

<article class="doc-entry" id="v-stm-then">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stm-then</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-stm-then" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L764" title="View source at line 764">Source</a></span></div>
<div class="doc-body">

`m`, then `n`, keeping `n`'s result.

</div>
</article>

<article class="doc-entry" id="v-tvar-modify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-modify</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tvar-modify" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L803" title="View source at line 803">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tvar-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tvar-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L794" title="View source at line 794">Source</a></span></div>
<div class="doc-body">

A new variable holding `x`. Until the transaction commits (and
stores it somewhere another task can reach) the variable is private
to the attempt that made it.

</div>
</article>

<article class="doc-entry" id="v-tvar-new-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tvar-new-io" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L816" title="View source at line 816">Source</a></span></div>
<div class="doc-body">

A new variable, made outside any transaction.

</div>
</article>

<article class="doc-entry" id="v-tvar-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tvar-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L797" title="View source at line 797">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tvar-read-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-read-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tvar-read-io" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L822" title="View source at line 822">Source</a></span></div>
<div class="doc-body">

The variable's committed value at some instant during the call,
outside any transaction. Two such reads are two instants: use a
transaction to read variables consistently with one another.

</div>
</article>

<article class="doc-entry" id="v-tvar-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-state</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>)) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-tvar-state" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L811" title="View source at line 811">Source</a></span></div>
<div class="doc-body">

Apply a state transition: `f` answers a result and the next value.

</div>
</article>

<article class="doc-entry" id="v-tvar-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tvar-swap" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L807" title="View source at line 807">Source</a></span></div>
<div class="doc-body">

Store `x` and answer the value it replaced.

</div>
</article>

<article class="doc-entry" id="v-tvar-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tvar-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="TVar">TVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tvar-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm/source/#L800" title="View source at line 800">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Alternative_STM"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">Implicit · <code>Effect.Stm.Alternative_STM</code></span></li>
<li id="i-Applicative_STM"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">Implicit · <code>Effect.Stm.Applicative_STM</code></span></li>
<li id="i-Functor_STM"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">Implicit · <code>Effect.Stm.Functor_STM</code></span><div class="doc-child-prose">The class instances (design/base-library.md: instances live with their type).</div></li>
<li id="i-Monad_STM"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">Implicit · <code>Effect.Stm.Monad_STM</code></span></li>
<li id="i-SeqStm"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#c-StmSequencing" title="Effect.Stm.StmSequencing">StmSequencing</a></code> <span class="doc-inst-meta">Implicit · <code>Effect.Stm.SeqStm</code></span></li>
</ul>
