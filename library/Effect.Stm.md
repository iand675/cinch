# Effect.Stm

Software transactional memory: TVar cells, composable STM transactions run by atomically, retry and or-else for blocking and choice, and the STM do provider. The transaction algorithm is Cinch source over inline target IR.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Stm`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stm.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-53544d"></a>

## `STM`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">STM</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A transaction: a function from an attempt to its outcome. The row
says the function reads shared cells; it writes none before commit.

[Source](/reference-sources/Effect.Stm.cinch) · line 403

# Signatures and methods

<a id="class-53746d53657175656e63696e67"></a>

## `StmSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">StmSequencing</span> </code></pre>

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

[Source](/reference-sources/Effect.Stm.cinch) · line 866

# Values

<a id="value-61746f6d6963616c6c79"></a>

## `atomically`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atomically</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-variable">a</span>)</code></pre>

Run a transaction: its reads and writes take effect together, at one
instant, or the transaction runs again. Returns when an attempt
commits, which `retry` and contention can postpone without bound.

[Source](/reference-sources/Effect.Stm.cinch) · line 717

<a id="value-61746f6d6963616c6c792d6f722d63616e63656c"></a>

## `atomically-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">atomically-or-cancel</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">STM</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

`atomically` for a task that must stay cancellable while it waits:
`None` when the task's cancellation was requested while the
transaction was waiting in `retry`, with nothing committed. A
request does not interrupt an attempt that is running, and a
transaction that never waits never observes it.

[Source](/reference-sources/Effect.Stm.cinch) · line 739

<a id="value-636865636b"></a>

## `check`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">check</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>))</code></pre>

Wait until the condition holds: `retry` when it does not.

[Source](/reference-sources/Effect.Stm.cinch) · line 789

<a id="value-6f722d656c7365"></a>

## `or-else`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">or-else</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>)))</code></pre>

`first`, unless it would wait: then its writes are discarded and
`second` runs in its place. If both would wait, the transaction
waits on what either of them read.

[Source](/reference-sources/Effect.Stm.cinch) · line 780

<a id="value-7265747279"></a>

## `retry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">retry</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>)</code></pre>

Abandon the attempt and run the whole transaction again once a
variable it has read changes. Inside `or-else`, give way to the
alternative instead.

[Source](/reference-sources/Effect.Stm.cinch) · line 774

<a id="value-73746d2d62696e64"></a>

## `stm-bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stm-bind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>)))</code></pre>

`m`, then the transaction `k` makes of its result, as one
transaction.

[Source](/reference-sources/Effect.Stm.cinch) · line 752

<a id="value-73746d2d6d6170"></a>

## `stm-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stm-map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Effect.Stm.cinch)

<a id="value-73746d2d70757265"></a>

## `stm-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stm-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The transaction that does nothing and answers `x`.

[Source](/reference-sources/Effect.Stm.cinch) · line 747

<a id="value-73746d2d7468656e"></a>

## `stm-then`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stm-then</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>)))</code></pre>

`m`, then `n`, keeping `n`'s result.

[Source](/reference-sources/Effect.Stm.cinch) · line 765

<a id="value-747661722d6d6f64696679"></a>

## `tvar-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-modify</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

[Source](/reference-sources/Effect.Stm.cinch)

<a id="value-747661722d6e6577"></a>

## `tvar-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>)))</code></pre>

A new variable holding `x`. Until the transaction commits (and
stores it somewhere another task can reach) the variable is private
to the attempt that made it.

[Source](/reference-sources/Effect.Stm.cinch) · line 795

<a id="value-747661722d6e65772d696f"></a>

## `tvar-new-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-new-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>)</code></pre>

A new variable, made outside any transaction.

[Source](/reference-sources/Effect.Stm.cinch) · line 817

<a id="value-747661722d72656164"></a>

## `tvar-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.cinch)

<a id="value-747661722d726561642d696f"></a>

## `tvar-read-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-read-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-variable">a</span>)</code></pre>

The variable's committed value at some instant during the call,
outside any transaction. Two such reads are two instants: use a
transaction to read variables consistently with one another.

[Source](/reference-sources/Effect.Stm.cinch) · line 823

<a id="value-747661722d7374617465"></a>

## `tvar-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">b</span>)))</code></pre>

Apply a state transition: `f` answers a result and the next value.

[Source](/reference-sources/Effect.Stm.cinch) · line 812

<a id="value-747661722d73776170"></a>

## `tvar-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-swap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>)))</code></pre>

Store `x` and answer the value it replaced.

[Source](/reference-sources/Effect.Stm.cinch) · line 808

<a id="value-747661722d7772697465"></a>

## `tvar-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tvar-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">TVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

[Source](/reference-sources/Effect.Stm.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Stm.Alternative_STM` | `Category.Alternative.Alternative Effect.Stm.STM` | Implicit |
| `Effect.Stm.Applicative_STM` | `Category.Applicative.Applicative Effect.Stm.STM` | Implicit |
| `Effect.Stm.Functor_STM` | `Category.Functor.Functor Effect.Stm.STM` | Implicit |
| `Effect.Stm.Monad_STM` | `Category.Monad.Monad Effect.Stm.STM` | Implicit |
| `Effect.Stm.SeqStm` | `Effect.Stm.StmSequencing` | Implicit |

**`Effect.Stm.Functor_STM`**: The class instances (design/base-library.md: instances live with
their type). `empty` is `retry` and `<|>` is `or-else`.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stm.cinch`. Source SHA-256: `2b71270681ca8f7bc4f84111b851c1c3624cc201ea38c84accf64871ada87810`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
