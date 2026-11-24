# Data.Sql

Pure typed SQL values, NULLs, named parameters, isolation levels, column metadata, errors and checked row scanners.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Sql`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Sql.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Sql](Data.Sql.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure SQL value model, column metadata, transaction options, named
parameters, structured errors, and typed row scanning.

Values are represented explicitly by `SqlValue`, avoiding untyped pointer
reflection. Nullable columns decode directly to `Option` through the
`scan-null-*` family, and every scanning or bounds failure produces a named
`SqlError` value. Byte buffers returned from `scan-bytes` are freshly copied
so callers cannot alias or mutate an immutable `SqlRow`.

</details>

# Types and constructors

<a id="type-53716c436f6c756d6e54797065"></a>

## `SqlColumnType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlColumnType</span></code></pre>

Metadata describing a single column in a query result set.

[Source](/reference-sources/Data.Sql.cinch) · line 92

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlColumnType</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>))))))</code></pre>

<a id="type-53716c4572726f72"></a>

## `SqlError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlError</span></code></pre>

Named SQL, pooling, driver, and row-scanning errors.

[Source](/reference-sources/Data.Sql.cinch) · line 102

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrNoRows</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrTxDone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrConnDone</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrPoolClosed</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrBadConn</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrDriver</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrScan</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrTypeMismatch</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrColumnOutOfRange</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlErrSkip</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span></code></pre>

<a id="type-53716c45786563526573756c74"></a>

## `SqlExecResult`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlExecResult</span></code></pre>

Summary returned by a statement execution that does not yield rows.

[Source](/reference-sources/Data.Sql.cinch) · line 97

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlExecResult</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))</code></pre>

<a id="type-53716c49736f6c6174696f6e"></a>

## `SqlIsolation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlIsolation</span></code></pre>

Transaction isolation level requested when beginning a transaction.

[Source](/reference-sources/Data.Sql.cinch) · line 68

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationDefault</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationReadUncommitted</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationReadCommitted</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationWriteCommitted</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationRepeatableRead</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationSnapshot</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationSerializable</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IsolationLinearizable</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span></code></pre>

<a id="type-53716c4e616d6564417267"></a>

## `SqlNamedArg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlNamedArg</span></code></pre>

A query parameter bound either by ordinal position or by explicit name.

[Source](/reference-sources/Data.Sql.cinch) · line 61

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlPositional</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlNamed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>))</code></pre>

<a id="type-53716c526f77"></a>

## `SqlRow`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlRow</span></code></pre>

An immutable materialized result row pairing column names with typed values.

[Source](/reference-sources/Data.Sql.cinch) · line 125

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlRow</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>))</code></pre>

<a id="type-53716c54784f7074696f6e73"></a>

## `SqlTxOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlTxOptions</span></code></pre>

Options configuring a new database transaction.

[Source](/reference-sources/Data.Sql.cinch) · line 87

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkSqlTxOptions</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span>))</code></pre>

<a id="type-53716c56616c7565"></a>

## `SqlValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SqlValue</span></code></pre>

A typed SQL scalar value carried by query parameters and result rows.

[Source](/reference-sources/Data.Sql.cinch) · line 44

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlNull</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlBool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlInt</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlFloat</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlString</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlBytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SqlTimestamp</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

# Values

<a id="value-726f772d636f6c756d6e2d696e646578"></a>

## `row-column-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-column-index</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Return the zero-based index of the first column with the given name.

[Source](/reference-sources/Data.Sql.cinch) · line 491

<a id="value-726f772d636f6c756d6e73"></a>

## `row-columns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-columns</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the column names of a row.

[Source](/reference-sources/Data.Sql.cinch) · line 464

<a id="value-726f772d676574"></a>

## `row-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-get</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)))</code></pre>

Retrieve the cell value at zero-based column index `idx`, or `SqlErrColumnOutOfRange`.

[Source](/reference-sources/Data.Sql.cinch) · line 496

<a id="value-726f772d6765742d62792d6e616d65"></a>

## `row-get-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-get-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)))</code></pre>

Retrieve the cell value for the first column matching `name`.

[Source](/reference-sources/Data.Sql.cinch) · line 503

<a id="value-726f772d6c656e"></a>

## `row-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-len</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the number of values in a row.

[Source](/reference-sources/Data.Sql.cinch) · line 474

<a id="value-726f772d76616c756573"></a>

## `row-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">row-values</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>))</code></pre>

Return the raw typed cell values of a row.

[Source](/reference-sources/Data.Sql.cinch) · line 469

<a id="value-7363616e2d626f6f6c"></a>

## `scan-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Scan a non-null `Bool` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 619

<a id="value-7363616e2d626f6f6c2d62792d6e616d65"></a>

## `scan-bool-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bool-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Scan a non-null `Bool` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 691

<a id="value-7363616e2d6279746573"></a>

## `scan-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Scan a non-null `Bytes` column at index `idx`, returning a fresh copy.

[Source](/reference-sources/Data.Sql.cinch) · line 631

<a id="value-7363616e2d62797465732d62792d6e616d65"></a>

## `scan-bytes-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-bytes-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Scan a non-null `Bytes` column by column name, returning a fresh copy.

[Source](/reference-sources/Data.Sql.cinch) · line 703

<a id="value-7363616e2d663634"></a>

## `scan-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-f64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">F64</span>)))</code></pre>

Scan a non-null `F64` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 613

<a id="value-7363616e2d6636342d62792d6e616d65"></a>

## `scan-f64-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-f64-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">F64</span>)))</code></pre>

Scan a non-null `F64` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 685

<a id="value-7363616e2d693634"></a>

## `scan-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-i64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>)))</code></pre>

Scan a non-null `I64` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 607

<a id="value-7363616e2d6936342d62792d6e616d65"></a>

## `scan-i64-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-i64-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>)))</code></pre>

Scan a non-null `I64` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 679

<a id="value-7363616e2d6e756c6c2d626f6f6c"></a>

## `scan-null-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>))))</code></pre>

Scan a nullable `Bool` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 655

<a id="value-7363616e2d6e756c6c2d626f6f6c2d62792d6e616d65"></a>

## `scan-null-bool-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-bool-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>))))</code></pre>

Scan a nullable `Bool` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 727

<a id="value-7363616e2d6e756c6c2d6279746573"></a>

## `scan-null-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Scan a nullable `Bytes` column at index `idx`, returning a fresh copy when present.

[Source](/reference-sources/Data.Sql.cinch) · line 667

<a id="value-7363616e2d6e756c6c2d62797465732d62792d6e616d65"></a>

## `scan-null-bytes-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-bytes-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Scan a nullable `Bytes` column by column name, returning a fresh copy when present.

[Source](/reference-sources/Data.Sql.cinch) · line 739

<a id="value-7363616e2d6e756c6c2d663634"></a>

## `scan-null-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-f64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">F64</span>))))</code></pre>

Scan a nullable `F64` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 649

<a id="value-7363616e2d6e756c6c2d6636342d62792d6e616d65"></a>

## `scan-null-f64-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-f64-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">F64</span>))))</code></pre>

Scan a nullable `F64` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 721

<a id="value-7363616e2d6e756c6c2d693634"></a>

## `scan-null-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-i64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))))</code></pre>

Scan a nullable `I64` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 643

<a id="value-7363616e2d6e756c6c2d6936342d62792d6e616d65"></a>

## `scan-null-i64-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-i64-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))))</code></pre>

Scan a nullable `I64` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 715

<a id="value-7363616e2d6e756c6c2d737472696e67"></a>

## `scan-null-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Scan a nullable `String` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 661

<a id="value-7363616e2d6e756c6c2d737472696e672d62792d6e616d65"></a>

## `scan-null-string-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-string-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

Scan a nullable `String` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 733

<a id="value-7363616e2d6e756c6c2d74696d657374616d70"></a>

## `scan-null-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))))</code></pre>

Scan a nullable timestamp (`I64` Unix nanoseconds) column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 673

<a id="value-7363616e2d6e756c6c2d74696d657374616d702d62792d6e616d65"></a>

## `scan-null-timestamp-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-null-timestamp-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))))</code></pre>

Scan a nullable timestamp (`I64` Unix nanoseconds) column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 745

<a id="value-7363616e2d737472696e67"></a>

## `scan-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Scan a non-null `String` column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 625

<a id="value-7363616e2d737472696e672d62792d6e616d65"></a>

## `scan-string-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-string-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Scan a non-null `String` column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 697

<a id="value-7363616e2d74696d657374616d70"></a>

## `scan-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>)))</code></pre>

Scan a non-null timestamp (`I64` Unix nanoseconds) column at index `idx`.

[Source](/reference-sources/Data.Sql.cinch) · line 637

<a id="value-7363616e2d74696d657374616d702d62792d6e616d65"></a>

## `scan-timestamp-by-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scan-timestamp-by-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>)))</code></pre>

Scan a non-null timestamp (`I64` Unix nanoseconds) column by column name.

[Source](/reference-sources/Data.Sql.cinch) · line 709

<a id="value-73716c2d617267"></a>

## `sql-arg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-arg</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>)</code></pre>

Construct a positional query argument.

[Source](/reference-sources/Data.Sql.cinch) · line 239

<a id="value-73716c2d6172672d6e616d65"></a>

## `sql-arg-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-arg-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the explicit parameter name, or `None` for positional arguments.

[Source](/reference-sources/Data.Sql.cinch) · line 247

<a id="value-73716c2d6172672d76616c7565"></a>

## `sql-arg-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-arg-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>)</code></pre>

Extract the underlying `SqlValue` from a query argument.

[Source](/reference-sources/Data.Sql.cinch) · line 253

<a id="value-73716c2d636f6c756d6e2d64617461626173652d747970652d6e616d65"></a>

## `sql-column-database-type-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-database-type-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the database system type name (empty when unsupported).

[Source](/reference-sources/Data.Sql.cinch) · line 339

<a id="value-73716c2d636f6c756d6e2d646563696d616c2d73697a65"></a>

## `sql-column-decimal-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-decimal-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">I64</span>, <span class="tok-constructor">I64</span>)))</code></pre>

Return `(precision, scale)` when both decimal size attributes are available.

[Source](/reference-sources/Data.Sql.cinch) · line 364

<a id="value-73716c2d636f6c756d6e2d6c656e677468"></a>

## `sql-column-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the declared length for variable-length column types, when known.

[Source](/reference-sources/Data.Sql.cinch) · line 349

<a id="value-73716c2d636f6c756d6e2d6e616d65"></a>

## `sql-column-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the column name or alias.

[Source](/reference-sources/Data.Sql.cinch) · line 334

<a id="value-73716c2d636f6c756d6e2d6e756c6c61626c65"></a>

## `sql-column-nullable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-nullable</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>))</code></pre>

Report whether the column may contain NULL, when known by the driver.

[Source](/reference-sources/Data.Sql.cinch) · line 344

<a id="value-73716c2d636f6c756d6e2d707265636973696f6e"></a>

## `sql-column-precision`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-precision</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the decimal precision, when applicable and known.

[Source](/reference-sources/Data.Sql.cinch) · line 354

<a id="value-73716c2d636f6c756d6e2d7363616c65"></a>

## `sql-column-scale`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-scale</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the decimal scale, when applicable and known.

[Source](/reference-sources/Data.Sql.cinch) · line 359

<a id="value-73716c2d636f6c756d6e2d74797065"></a>

## `sql-column-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-column-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlColumnType</span>))))))</code></pre>

Construct column metadata.

[Source](/reference-sources/Data.Sql.cinch) · line 329

<a id="value-73716c2d6572726f722d6d657373616765"></a>

## `sql-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a `SqlError` as a human-readable diagnostic message.

[Source](/reference-sources/Data.Sql.cinch) · line 416

<a id="value-73716c2d657865632d726573756c74"></a>

## `sql-exec-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-exec-result</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span>))</code></pre>

Construct a statement execution summary.

[Source](/reference-sources/Data.Sql.cinch) · line 392

<a id="value-73716c2d657865632d726573756c742d6e6f2d726f7773"></a>

## `sql-exec-result-no-rows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-exec-result-no-rows</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span></code></pre>

Execution summary for statements (such as DDL) reporting neither an insert ID nor affected row count.

[Source](/reference-sources/Data.Sql.cinch) · line 396

<a id="value-73716c2d69736f6c6174696f6e2d6e616d65"></a>

## `sql-isolation-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-isolation-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable name for a transaction isolation level.

[Source](/reference-sources/Data.Sql.cinch) · line 271

<a id="value-73716c2d6c6173742d696e736572742d6964"></a>

## `sql-last-insert-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-last-insert-id</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the generated insert identifier, if reported by the driver.

[Source](/reference-sources/Data.Sql.cinch) · line 400

<a id="value-73716c2d6e616d6564"></a>

## `sql-named`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-named</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlNamedArg</span>))</code></pre>

Construct a named query argument.

[Source](/reference-sources/Data.Sql.cinch) · line 243

<a id="value-73716c2d726f77"></a>

## `sql-row`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-row</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlRow</span>))</code></pre>

Construct an immutable `SqlRow` from column names and cell values.

[Source](/reference-sources/Data.Sql.cinch) · line 460

<a id="value-73716c2d726f77732d6166666563746564"></a>

## `sql-rows-affected`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-rows-affected</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlExecResult</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the number of rows affected, if reported by the driver.

[Source](/reference-sources/Data.Sql.cinch) · line 405

<a id="value-73716c2d74782d69736f6c6174696f6e"></a>

## `sql-tx-isolation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-isolation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span>)</code></pre>

Read the isolation level from transaction options.

[Source](/reference-sources/Data.Sql.cinch) · line 313

<a id="value-73716c2d74782d6f7074696f6e73"></a>

## `sql-tx-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-options</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlIsolation</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span>))</code></pre>

Construct transaction options with an isolation level and read-only flag.

[Source](/reference-sources/Data.Sql.cinch) · line 309

<a id="value-73716c2d74782d6f7074696f6e732d64656661756c74"></a>

## `sql-tx-options-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-options-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span></code></pre>

Default transaction options: default isolation level and read-write access.

[Source](/reference-sources/Data.Sql.cinch) · line 305

<a id="value-73716c2d74782d726561642d6f6e6c79"></a>

## `sql-tx-read-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-tx-read-only</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlTxOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Read whether transaction options request a read-only transaction.

[Source](/reference-sources/Data.Sql.cinch) · line 318

<a id="value-73716c2d76616c75652d6571"></a>

## `sql-value-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-value-eq</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Structural equality on `SqlValue`.

[Source](/reference-sources/Data.Sql.cinch) · line 207

<a id="value-73716c2d76616c75652d69732d6e756c6c"></a>

## `sql-value-is-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-value-is-null</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Report whether a SQL value is `SqlNull`.

[Source](/reference-sources/Data.Sql.cinch) · line 195

<a id="value-73716c2d76616c75652d73686f77"></a>

## `sql-value-show`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-value-show</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Human-readable representation of a `SqlValue`.

[Source](/reference-sources/Data.Sql.cinch) · line 219

<a id="value-73716c2d76616c75652d747970652d6e616d65"></a>

## `sql-value-type-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sql-value-type-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the canonical uppercase SQL type name for a value.

[Source](/reference-sources/Data.Sql.cinch) · line 184

<a id="value-76616c75652d61732d626f6f6c"></a>

## `value-as-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bool</span>))</code></pre>

Decode a non-null `SqlBool` into a `Bool`.

[Source](/reference-sources/Data.Sql.cinch) · line 541

<a id="value-76616c75652d61732d6279746573"></a>

## `value-as-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a non-null `SqlBytes` into a fresh copy of its `Bytes`.

[Source](/reference-sources/Data.Sql.cinch) · line 553

<a id="value-76616c75652d61732d663634"></a>

## `value-as-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-f64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">F64</span>))</code></pre>

Decode a non-null `SqlFloat` into an `F64`.

[Source](/reference-sources/Data.Sql.cinch) · line 535

<a id="value-76616c75652d61732d693634"></a>

## `value-as-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-i64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>))</code></pre>

Decode a non-null `SqlInt` into an `I64`.

[Source](/reference-sources/Data.Sql.cinch) · line 529

<a id="value-76616c75652d61732d6e756c6c2d626f6f6c"></a>

## `value-as-null-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-bool</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Decode a nullable `SqlBool` into `Option Bool`.

[Source](/reference-sources/Data.Sql.cinch) · line 579

<a id="value-76616c75652d61732d6e756c6c2d6279746573"></a>

## `value-as-null-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode a nullable `SqlBytes` into `Option Bytes`, copying non-null buffers.

[Source](/reference-sources/Data.Sql.cinch) · line 593

<a id="value-76616c75652d61732d6e756c6c2d663634"></a>

## `value-as-null-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-f64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">F64</span>)))</code></pre>

Decode a nullable `SqlFloat` into `Option F64`.

[Source](/reference-sources/Data.Sql.cinch) · line 572

<a id="value-76616c75652d61732d6e756c6c2d693634"></a>

## `value-as-null-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-i64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

Decode a nullable `SqlInt` into `Option I64`.

[Source](/reference-sources/Data.Sql.cinch) · line 565

<a id="value-76616c75652d61732d6e756c6c2d737472696e67"></a>

## `value-as-null-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Decode a nullable `SqlString` into `Option String`.

[Source](/reference-sources/Data.Sql.cinch) · line 586

<a id="value-76616c75652d61732d6e756c6c2d74696d657374616d70"></a>

## `value-as-null-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-null-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)))</code></pre>

Decode a nullable `SqlTimestamp` into `Option I64` Unix nanoseconds.

[Source](/reference-sources/Data.Sql.cinch) · line 600

<a id="value-76616c75652d61732d737472696e67"></a>

## `value-as-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Decode a non-null `SqlString` into a `String`.

[Source](/reference-sources/Data.Sql.cinch) · line 547

<a id="value-76616c75652d61732d74696d657374616d70"></a>

## `value-as-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">value-as-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Sql</span>.<span class="tok-constructor">SqlError</span> <span class="tok-constructor">I64</span>))</code></pre>

Decode a non-null `SqlTimestamp` into Unix nanoseconds (`I64`).

[Source](/reference-sources/Data.Sql.cinch) · line 559

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Sql.Eq_SqlColumnType` | `Relation.Eq.Eq Data.Sql.SqlColumnType` | Implicit |
| `Data.Sql.Eq_SqlError` | `Relation.Eq.Eq Data.Sql.SqlError` | Implicit |
| `Data.Sql.Eq_SqlExecResult` | `Relation.Eq.Eq Data.Sql.SqlExecResult` | Implicit |
| `Data.Sql.Eq_SqlIsolation` | `Relation.Eq.Eq Data.Sql.SqlIsolation` | Implicit |
| `Data.Sql.Eq_SqlNamedArg` | `Relation.Eq.Eq Data.Sql.SqlNamedArg` | Implicit |
| `Data.Sql.Eq_SqlRow` | `Relation.Eq.Eq Data.Sql.SqlRow` | Implicit |
| `Data.Sql.Eq_SqlTxOptions` | `Relation.Eq.Eq Data.Sql.SqlTxOptions` | Implicit |
| `Data.Sql.Eq_SqlValue` | `Relation.Eq.Eq Data.Sql.SqlValue` | Implicit |
| `Data.Sql.Show_SqlError` | `Text.Show.Show Data.Sql.SqlError` | Implicit |
| `Data.Sql.Show_SqlIsolation` | `Text.Show.Show Data.Sql.SqlIsolation` | Implicit |
| `Data.Sql.Show_SqlValue` | `Text.Show.Show Data.Sql.SqlValue` | Implicit |

**`Data.Sql.Eq_SqlColumnType`**: Structural equality instance for `SqlColumnType`.
**`Data.Sql.Eq_SqlError`**: Structural equality instance for `SqlError`.
**`Data.Sql.Eq_SqlExecResult`**: Structural equality instance for `SqlExecResult`.
**`Data.Sql.Eq_SqlIsolation`**: Structural equality instance for `SqlIsolation`.
**`Data.Sql.Eq_SqlNamedArg`**: Structural equality instance for `SqlNamedArg`.
**`Data.Sql.Eq_SqlRow`**: Structural equality instance for `SqlRow`.
**`Data.Sql.Eq_SqlTxOptions`**: Structural equality instance for `SqlTxOptions`.
**`Data.Sql.Eq_SqlValue`**: Structural equality instance for `SqlValue`.
**`Data.Sql.Show_SqlError`**: Text rendering instance for `SqlError`.
**`Data.Sql.Show_SqlIsolation`**: Text rendering instance for `SqlIsolation`.
**`Data.Sql.Show_SqlValue`**: Text rendering instance for `SqlValue`.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Sql.cinch`. Source SHA-256: `373d2ada07bae674812cf2289c4584ec3f3eb8e350f4cd2e601c9d18270e3ab0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
