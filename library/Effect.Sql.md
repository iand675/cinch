# Effect.Sql

Capability-explicit SQL driver protocol, STM-backed connection pooling, prepared statements, transactions, retries and an in-memory transactional driver.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Sql`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Sql.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Sql](Data.Sql.md), [Data.String](Data.String.md), [Effect.Stm](Effect.Stm.md), [Relation.Eq](Relation.Eq.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Sql](Data.Sql.md), [Effect.Sql](Effect.Sql.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Database driver protocol, STM-backed connection pooling, prepared
statements, transactions, streaming row cursors, pool statistics, and an
in-memory transactional driver.

Drivers are passed explicitly as values to `sql-open` rather than registered
in global mutable state. Connection pool state is coordinated through
`Effect.Stm`, providing atomic checkout, idle connection reuse, bounded open
connection limits with blocking wakeups, idle eviction, transparent retries
on `SqlErrBadConn`, and fallback from direct `Exec`/`Query` to prepared
statements on `SqlErrSkip`.

For concurrent workloads spanning multiple tasks, `SqlPoolGate` and
`SqlMemoryStore` contain only shareable representations and may be captured
across tasks alongside `sql-open-with-gate`.

</details>

# Types and constructors

<a id="type-53716c4462"></a>

## `SqlDb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDb</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Connection pool handle backed by an explicit driver and STM coordination.

[Source](/reference-sources/Effect.Sql.cinch) · line 333

<a id="type-53716c44625374617473"></a>

## `SqlDbStats`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDbStats</span></code></pre>

Snapshot of connection pool limits, utilization, and lifetime counters.

[Source](/reference-sources/Effect.Sql.cinch) · line 236

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDbStats</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span>))))))</code></pre>

<a id="type-53716c447269766572"></a>

## `SqlDriver`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDriver</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Explicit database driver value that opens connections from a DSN string.

[Source](/reference-sources/Effect.Sql.cinch) · line 218

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDriver</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>)) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>)</code></pre>

<a id="type-53716c447269766572436f6e6e"></a>

## `SqlDriverConn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDriverConn</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Low-level connection to a database backend. Used by at most one caller at a time.

[Source](/reference-sources/Effect.Sql.cinch) · line 164

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDriverConn</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>)) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>))))))</code></pre>

<a id="type-53716c447269766572526f7773"></a>

## `SqlDriverRows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDriverRows</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Low-level row cursor supplied by a database driver.

[Source](/reference-sources/Effect.Sql.cinch) · line 61

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDriverRows</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>))) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>))))</code></pre>

<a id="type-53716c44726976657253746d74"></a>

## `SqlDriverStmt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDriverStmt</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Low-level prepared statement bound to a single driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 100

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDriverStmt</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>)) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>))))</code></pre>

<a id="type-53716c4472697665725478"></a>

## `SqlDriverTx`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlDriverTx</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Low-level transaction handle bound to a single driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 139

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlDriverTx</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>))</code></pre>

<a id="type-53716c4d656d6f727953746f7265"></a>

## `SqlMemoryStore`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlMemoryStore</span></code></pre>

Shareable transactional in-memory database store.
Contains only shareable types and a `TVar`, so concurrent tasks may share one store safely.

[Source](/reference-sources/Effect.Sql.cinch) · line 1057

<a id="type-53716c506f6f6c47617465"></a>

## `SqlPoolGate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlPoolGate</span></code></pre>

Shareable STM coordination gate for connection pool limits and stats.
Contains only shareable scalars and a `TVar`, so it may be shared across concurrent tasks.

[Source](/reference-sources/Effect.Sql.cinch) · line 284

<a id="type-53716c526f7773"></a>

## `SqlRows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlRows</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Active query result cursor. Must be exhausted or closed to release its connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 683

<a id="type-53716c53746d74"></a>

## `SqlStmt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlStmt</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A prepared statement bound either to a connection pool or an active transaction.

[Source](/reference-sources/Effect.Sql.cinch) · line 812

<a id="type-53716c5478"></a>

## `SqlTx`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlTx</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An active database transaction pinning one pooled connection until commit or rollback.

[Source](/reference-sources/Effect.Sql.cinch) · line 913

# Values

<a id="value-6472697665722d636f6e6e2d626567696e"></a>

## `driver-conn-begin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-begin</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>))))</code></pre>

Start a transaction on a driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 208

<a id="value-6472697665722d636f6e6e2d636c6f7365"></a>

## `driver-conn-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a driver connection and release its backend resources.

[Source](/reference-sources/Effect.Sql.cinch) · line 213

<a id="value-6472697665722d636f6e6e2d65786563"></a>

## `driver-conn-exec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-exec</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))))</code></pre>

Execute a non-query statement directly on a driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 193

<a id="value-6472697665722d636f6e6e2d70696e67"></a>

## `driver-conn-ping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-ping</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Verify that a driver connection is alive.

[Source](/reference-sources/Effect.Sql.cinch) · line 188

<a id="value-6472697665722d636f6e6e2d70726570617265"></a>

## `driver-conn-prepare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-prepare</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>))))</code></pre>

Prepare a statement on a driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 203

<a id="value-6472697665722d636f6e6e2d7175657279"></a>

## `driver-conn-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-conn-query</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>)))))</code></pre>

Execute a row-returning query directly on a driver connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 198

<a id="value-6472697665722d6f70656e"></a>

## `driver-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-open</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>))))</code></pre>

Open one fresh connection using a driver and DSN string.

[Source](/reference-sources/Effect.Sql.cinch) · line 227

<a id="value-6472697665722d726f77732d636c6f7365"></a>

## `driver-rows-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-rows-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a driver row cursor.

[Source](/reference-sources/Effect.Sql.cinch) · line 95

<a id="value-6472697665722d726f77732d636f6c756d6e2d7479706573"></a>

## `driver-rows-column-types`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-rows-column-types</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>))</code></pre>

Fetch column metadata from a driver row cursor.

[Source](/reference-sources/Effect.Sql.cinch) · line 85

<a id="value-6472697665722d726f77732d636f6c756d6e73"></a>

## `driver-rows-columns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-rows-columns</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Fetch column names from a driver row cursor.

[Source](/reference-sources/Effect.Sql.cinch) · line 80

<a id="value-6472697665722d726f77732d6e657874"></a>

## `driver-rows-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-rows-next</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>))))</code></pre>

Advance a driver row cursor to the next row, returning `Ok None` at end of set.

[Source](/reference-sources/Effect.Sql.cinch) · line 90

<a id="value-6472697665722d73746d742d636c6f7365"></a>

## `driver-stmt-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-stmt-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a driver prepared statement.

[Source](/reference-sources/Effect.Sql.cinch) · line 129

<a id="value-6472697665722d73746d742d65786563"></a>

## `driver-stmt-exec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-stmt-exec</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>)))</code></pre>

Execute a driver prepared statement without returning rows.

[Source](/reference-sources/Effect.Sql.cinch) · line 119

<a id="value-6472697665722d73746d742d6e756d2d696e707574"></a>

## `driver-stmt-num-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-stmt-num-input</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Return the expected placeholder count for a driver statement (`-1` if unknown).

[Source](/reference-sources/Effect.Sql.cinch) · line 134

<a id="value-6472697665722d73746d742d7175657279"></a>

## `driver-stmt-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-stmt-query</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>))))</code></pre>

Execute a driver prepared statement that returns rows.

[Source](/reference-sources/Effect.Sql.cinch) · line 124

<a id="value-6472697665722d74782d636f6d6d6974"></a>

## `driver-tx-commit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-tx-commit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Commit a driver transaction.

[Source](/reference-sources/Effect.Sql.cinch) · line 154

<a id="value-6472697665722d74782d726f6c6c6261636b"></a>

## `driver-tx-rollback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">driver-tx-rollback</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Roll back a driver transaction.

[Source](/reference-sources/Effect.Sql.cinch) · line 159

<a id="value-73716c2d64622d626567696e"></a>

## `sql-db-begin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-begin</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>)))</code></pre>

Begin a transaction with default options (`IsolationDefault`, read-write).

[Source](/reference-sources/Effect.Sql.cinch) · line 937

<a id="value-73716c2d64622d626567696e2d7478"></a>

## `sql-db-begin-tx`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-begin-tx</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>))))</code></pre>

Begin a transaction with explicit isolation and read-only options.

[Source](/reference-sources/Effect.Sql.cinch) · line 941

<a id="value-73716c2d64622d636c6f7365"></a>

## `sql-db-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close the database pool, preventing new operations and closing all idle connections.
Idempotent when called more than once.

[Source](/reference-sources/Effect.Sql.cinch) · line 612

<a id="value-73716c2d64622d65786563"></a>

## `sql-db-exec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-exec</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))))</code></pre>

Execute a statement that does not return rows, retrying on `SqlErrBadConn`.

[Source](/reference-sources/Effect.Sql.cinch) · line 675

<a id="value-73716c2d64622d70696e67"></a>

## `sql-db-ping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-ping</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Verify connectivity to the database, establishing a connection if necessary.
Retries automatically on `SqlErrBadConn`.

[Source](/reference-sources/Effect.Sql.cinch) · line 671

<a id="value-73716c2d64622d70726570617265"></a>

## `sql-db-prepare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-prepare</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>))))</code></pre>

Prepare a statement against the pool, validating it on a connection immediately.

[Source](/reference-sources/Effect.Sql.cinch) · line 819

<a id="value-73716c2d64622d7175657279"></a>

## `sql-db-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-query</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>)))))</code></pre>

Execute a query returning a streaming row cursor, retrying on `SqlErrBadConn`.

[Source](/reference-sources/Effect.Sql.cinch) · line 786

<a id="value-73716c2d64622d71756572792d726f77"></a>

## `sql-db-query-row`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-query-row</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>))))</code></pre>

Execute a query expected to return at most one row and automatically close its cursor.
Returns `Err SqlErrNoRows` when the query yields no rows.

[Source](/reference-sources/Effect.Sql.cinch) · line 802

<a id="value-73716c2d64622d7365742d6d61782d69646c652d636f6e6e73"></a>

## `sql-db-set-max-idle-conns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-set-max-idle-conns</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Set the maximum number of idle connections retained in the pool (`0` retains none).
Clamped to `max-open` when `max-open > 0`, closing any surplus idle connections immediately.

[Source](/reference-sources/Effect.Sql.cinch) · line 434

<a id="value-73716c2d64622d7365742d6d61782d6f70656e2d636f6e6e73"></a>

## `sql-db-set-max-open-conns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-set-max-open-conns</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Set the maximum number of open connections (`0` means unlimited).
If positive and lower than `max-idle`, reduces `max-idle` and closes surplus idle connections.

[Source](/reference-sources/Effect.Sql.cinch) · line 425

<a id="value-73716c2d64622d7374617473"></a>

## `sql-db-stats`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-db-stats</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span>)</code></pre>

Return a consistent snapshot of the pool's current statistics.

[Source](/reference-sources/Effect.Sql.cinch) · line 353

<a id="value-73716c2d647269766572"></a>

## `sql-driver`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-driver</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>))</code></pre>

Construct a database driver from its connection-opening callback.

[Source](/reference-sources/Effect.Sql.cinch) · line 223

<a id="value-73716c2d6472697665722d636f6e6e"></a>

## `sql-driver-conn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-driver-conn</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>)))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>))) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverConn</span> <span class="tok-variable">r</span>)))))))</code></pre>

Construct a driver connection from its operation callbacks.

[Source](/reference-sources/Effect.Sql.cinch) · line 176

<a id="value-73716c2d6472697665722d726f7773"></a>

## `sql-driver-rows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-driver-rows</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>)) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)))) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>)))))</code></pre>

Construct a driver row cursor from its callbacks.

[Source](/reference-sources/Effect.Sql.cinch) · line 71

<a id="value-73716c2d6472697665722d73746d74"></a>

## `sql-driver-stmt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-driver-stmt</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>)) -&gt; (((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverRows</span> <span class="tok-variable">r</span>))) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverStmt</span> <span class="tok-variable">r</span>)))))</code></pre>

Construct a driver prepared statement.

[Source](/reference-sources/Effect.Sql.cinch) · line 110

<a id="value-73716c2d6472697665722d7478"></a>

## `sql-driver-tx`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-driver-tx</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriverTx</span> <span class="tok-variable">r</span>)))</code></pre>

Construct a driver transaction handle.

[Source](/reference-sources/Effect.Sql.cinch) · line 147

<a id="value-73716c2d6d656d6f72792d647269766572"></a>

## `sql-memory-driver`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-memory-driver</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlMemoryStore</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>))</code></pre>

Create an in-memory transactional driver backed by an explicit `SqlMemoryStore`.

[Source](/reference-sources/Effect.Sql.cinch) · line 1842

<a id="value-73716c2d6d656d6f72792d6472697665722d6e6577"></a>

## `sql-memory-driver-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-memory-driver-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">STM</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>))</code></pre>

Create an in-memory transactional driver backed by a fresh private store.

[Source](/reference-sources/Effect.Sql.cinch) · line 1846

<a id="value-73716c2d6d656d6f72792d73746f72652d636c6f73652d636f756e74"></a>

## `sql-memory-store-close-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-memory-store-close-count</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlMemoryStore</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">I64</span>)</code></pre>

Return the total number of driver connections closed against a memory store.

[Source](/reference-sources/Effect.Sql.cinch) · line 1072

<a id="value-73716c2d6d656d6f72792d73746f72652d6e6577"></a>

## `sql-memory-store-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-memory-store-new</span> :: (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlMemoryStore</span>)</code></pre>

Create an empty shareable in-memory SQL store.

[Source](/reference-sources/Effect.Sql.cinch) · line 1062

<a id="value-73716c2d6d656d6f72792d73746f72652d6f70656e2d636f756e74"></a>

## `sql-memory-store-open-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-memory-store-open-count</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlMemoryStore</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">I64</span>)</code></pre>

Return the total number of driver connections opened against a memory store.

[Source](/reference-sources/Effect.Sql.cinch) · line 1066

<a id="value-73716c2d6f70656e"></a>

## `sql-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-open</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>))))</code></pre>

Open a connection pool lazily for the given driver and DSN string.
Does not open a backend connection until `sql-db-ping` or a query executes.

[Source](/reference-sources/Effect.Sql.cinch) · line 339

<a id="value-73716c2d6f70656e2d776974682d67617465"></a>

## `sql-open-with-gate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-open-with-gate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDriver</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDb</span> <span class="tok-variable">r</span>)))))</code></pre>

Open a pool handle participating in a shared `SqlPoolGate` across concurrent tasks.
Connections checked out through a shared gate are closed on release so their slot returns immediately to waiting tasks.

[Source](/reference-sources/Effect.Sql.cinch) · line 347

<a id="value-73716c2d706f6f6c2d676174652d636c6f7365"></a>

## `sql-pool-gate-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-pool-gate-close</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">STM</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Mark a shared pool gate as closed, waking any waiting acquirers.

[Source](/reference-sources/Effect.Sql.cinch) · line 326

<a id="value-73716c2d706f6f6c2d676174652d6e6577"></a>

## `sql-pool-gate-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-pool-gate-new</span> :: (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span>)</code></pre>

Allocate a new pool coordination gate with unlimited max open connections and default max idle of 2.

[Source](/reference-sources/Effect.Sql.cinch) · line 289

<a id="value-73716c2d706f6f6c2d676174652d7365742d6d61782d69646c65"></a>

## `sql-pool-gate-set-max-idle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-pool-gate-set-max-idle</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">STM</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Update the maximum idle connections limit on a shared pool gate.
Clamps the limit to `max-open` when `max-open > 0`.

[Source](/reference-sources/Effect.Sql.cinch) · line 318

<a id="value-73716c2d706f6f6c2d676174652d7365742d6d61782d6f70656e"></a>

## `sql-pool-gate-set-max-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-pool-gate-set-max-open</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">STM</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Update the maximum open connections limit on a shared pool gate.
Reduces `max-idle` if the new positive limit is smaller than `max-idle`.

[Source](/reference-sources/Effect.Sql.cinch) · line 309

<a id="value-73716c2d706f6f6c2d676174652d7374617473"></a>

## `sql-pool-gate-stats`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-pool-gate-stats</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlPoolGate</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span>)</code></pre>

Read a consistent snapshot of pool statistics from a shared gate.

[Source](/reference-sources/Effect.Sql.cinch) · line 304

<a id="value-73716c2d726f77732d616c6c"></a>

## `sql-rows-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>)))</code></pre>

Materialize all remaining rows into a list and close the cursor.

[Source](/reference-sources/Effect.Sql.cinch) · line 758

<a id="value-73716c2d726f77732d636c6f7365"></a>

## `sql-rows-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a row cursor and return any associated connection to the pool. Idempotent.

[Source](/reference-sources/Effect.Sql.cinch) · line 722

<a id="value-73716c2d726f77732d636f6c756d6e2d7479706573"></a>

## `sql-rows-column-types`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-column-types</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>))</code></pre>

Return detailed column metadata for a result set.

[Source](/reference-sources/Effect.Sql.cinch) · line 731

<a id="value-73716c2d726f77732d636f6c756d6e73"></a>

## `sql-rows-columns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-columns</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the column names of a result set.

[Source](/reference-sources/Effect.Sql.cinch) · line 726

<a id="value-73716c2d726f77732d6e657874"></a>

## `sql-rows-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-next</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>)))</code></pre>

Read the next row from the cursor. Automatically closes the cursor on EOF or error.

[Source](/reference-sources/Effect.Sql.cinch) · line 736

<a id="value-73716c2d73746d742d636c6f7365"></a>

## `sql-stmt-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-stmt-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a prepared statement. Idempotent.

[Source](/reference-sources/Effect.Sql.cinch) · line 899

<a id="value-73716c2d73746d742d65786563"></a>

## `sql-stmt-exec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-stmt-exec</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>)))</code></pre>

Execute a prepared statement without returning rows.

[Source](/reference-sources/Effect.Sql.cinch) · line 863

<a id="value-73716c2d73746d742d7175657279"></a>

## `sql-stmt-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-stmt-query</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>))))</code></pre>

Execute a prepared statement returning a row cursor.

[Source](/reference-sources/Effect.Sql.cinch) · line 880

<a id="value-73716c2d73746d742d71756572792d726f77"></a>

## `sql-stmt-query-row`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-stmt-query-row</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>)))</code></pre>

Execute a prepared statement expected to return a single row, closing the cursor automatically.

[Source](/reference-sources/Effect.Sql.cinch) · line 893

<a id="value-73716c2d74782d636f6d6d6974"></a>

## `sql-tx-commit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-commit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Commit an active transaction and release its connection back to the pool.
Returns `Err SqlErrTxDone` if the transaction was already committed or rolled back.

[Source](/reference-sources/Effect.Sql.cinch) · line 992

<a id="value-73716c2d74782d65786563"></a>

## `sql-tx-exec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-exec</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))))</code></pre>

Execute a non-query statement within an active transaction.

[Source](/reference-sources/Effect.Sql.cinch) · line 952

<a id="value-73716c2d74782d70726570617265"></a>

## `sql-tx-prepare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-prepare</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlStmt</span> <span class="tok-variable">r</span>))))</code></pre>

Prepare a statement bound to an active transaction.

[Source](/reference-sources/Effect.Sql.cinch) · line 979

<a id="value-73716c2d74782d7175657279"></a>

## `sql-tx-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-query</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRows</span> <span class="tok-variable">r</span>)))))</code></pre>

Execute a query within an active transaction. Closing the rows does not release the transaction's connection.

[Source](/reference-sources/Effect.Sql.cinch) · line 959

<a id="value-73716c2d74782d71756572792d726f77"></a>

## `sql-tx-query-row`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-query-row</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>))))</code></pre>

Execute a single-row query within an active transaction, returning `SqlErrNoRows` when empty.

[Source](/reference-sources/Effect.Sql.cinch) · line 973

<a id="value-73716c2d74782d726f6c6c6261636b"></a>

## `sql-tx-rollback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-rollback</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTx</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">STM</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Roll back an active transaction and release its connection back to the pool.
Returns `Err SqlErrTxDone` if the transaction was already committed or rolled back.

[Source](/reference-sources/Effect.Sql.cinch) · line 1009

<a id="value-73746174732d69646c65"></a>

## `stats-idle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-idle</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of idle connections retained in the pool.

[Source](/reference-sources/Effect.Sql.cinch) · line 256

<a id="value-73746174732d696e2d757365"></a>

## `stats-in-use`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-in-use</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of connections currently checked out for queries or transactions.

[Source](/reference-sources/Effect.Sql.cinch) · line 251

<a id="value-73746174732d6d61782d69646c652d636c6f736564"></a>

## `stats-max-idle-closed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-max-idle-closed</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Cumulative count of healthy connections closed due to the max-idle limit.

[Source](/reference-sources/Effect.Sql.cinch) · line 266

<a id="value-73746174732d6d61782d6f70656e2d636f6e6e656374696f6e73"></a>

## `stats-max-open-connections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-max-open-connections</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Maximum open connections permitted (`0` means unlimited).

[Source](/reference-sources/Effect.Sql.cinch) · line 241

<a id="value-73746174732d6f70656e2d636f6e6e656374696f6e73"></a>

## `stats-open-connections`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-open-connections</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Total established connections currently tracked by the pool (in-use plus idle).

[Source](/reference-sources/Effect.Sql.cinch) · line 246

<a id="value-73746174732d776169742d636f756e74"></a>

## `stats-wait-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stats-wait-count</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlDbStats</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Cumulative count of connection checkouts that blocked waiting for a slot.

[Source](/reference-sources/Effect.Sql.cinch) · line 261

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Sql.Eq_SqlDbStats` | `Relation.Eq.Eq Effect.Sql.SqlDbStats` | Implicit |

**`Effect.Sql.Eq_SqlDbStats`**: Structural equality instance for `SqlDbStats`.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Sql.cinch`. Source SHA-256: `b44578cf0f963743993d3f73b2fa74a37129e114d223bc20ab18a23b4bb33301`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
