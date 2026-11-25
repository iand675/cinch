# Data.Sql

Pure typed SQL values, NULLs, named parameters, isolation levels, column metadata, errors and checked row scanners.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Sql</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Sql/source/">View source</a><a href="/packages/search/?q=%2BData.Sql">Search module</a><a href="/reference-sources/Data.Sql.cinch" download="Data.Sql.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure SQL value model, column metadata, transaction options, named
parameters, structured errors, and typed row scanning.

Values are represented explicitly by `SqlValue`, avoiding untyped pointer
reflection. Nullable columns decode directly to `Option` through the
`scan-null-*` family, and every scanning or bounds failure produces a named
`SqlError` value. Byte buffers returned from `scan-bytes` are freshly copied
so callers cannot alias or mutate an immutable `SqlRow`.

</details>

<details class="doc-synopsis"><summary>Synopsis (77 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-SqlColumnType">SqlColumnType</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlColumnType</strong></code></li><li><a class="syn-name" href="#t-SqlError">SqlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlError</strong></code></li><li><a class="syn-name" href="#t-SqlExecResult">SqlExecResult</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlExecResult</strong></code></li><li><a class="syn-name" href="#t-SqlIsolation">SqlIsolation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlIsolation</strong></code></li><li><a class="syn-name" href="#t-SqlNamedArg">SqlNamedArg</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlNamedArg</strong></code></li><li><a class="syn-name" href="#t-SqlRow">SqlRow</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlRow</strong></code></li><li><a class="syn-name" href="#t-SqlTxOptions">SqlTxOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlTxOptions</strong></code></li><li><a class="syn-name" href="#t-SqlValue">SqlValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlValue</strong></code></li><li><a class="syn-name" href="#v-row-column-index">row-column-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-column-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-row-columns">row-columns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-columns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-row-get">row-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li><a class="syn-name" href="#v-row-get-by-name">row-get-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-get-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li><a class="syn-name" href="#v-row-len">row-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-row-values">row-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">row-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li><a class="syn-name" href="#v-scan-bool">scan-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-scan-bool-by-name">scan-bool-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bool-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-scan-bytes">scan-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scan-bytes-by-name">scan-bytes-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bytes-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scan-f64">scan-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-scan-f64-by-name">scan-f64-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-f64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-scan-i64">scan-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-i64-by-name">scan-i64-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-i64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-null-bool">scan-null-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-scan-null-bool-by-name">scan-null-bool-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-bool-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-scan-null-bytes">scan-null-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-scan-null-bytes-by-name">scan-null-bytes-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-bytes-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-scan-null-f64">scan-null-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-scan-null-f64-by-name">scan-null-f64-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-f64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-scan-null-i64">scan-null-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-scan-null-i64-by-name">scan-null-i64-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-i64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-scan-null-string">scan-null-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-scan-null-string-by-name">scan-null-string-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-string-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-scan-null-timestamp">scan-null-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-scan-null-timestamp-by-name">scan-null-timestamp-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-null-timestamp-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-scan-string">scan-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-scan-string-by-name">scan-string-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-string-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-scan-timestamp">scan-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-scan-timestamp-by-name">scan-timestamp-by-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-timestamp-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-arg">sql-arg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code></li><li><a class="syn-name" href="#v-sql-arg-name">sql-arg-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-arg-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-arg-value">sql-arg-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-arg-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li><a class="syn-name" href="#v-sql-column-database-type-name">sql-column-database-type-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-database-type-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-column-decimal-size">sql-column-decimal-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-decimal-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-sql-column-length">sql-column-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-column-name">sql-column-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-column-nullable">sql-column-nullable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-nullable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-sql-column-precision">sql-column-precision</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-precision</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-column-scale">sql-column-scale</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-scale</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-column-type">sql-column-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-column-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a></code></li><li><a class="syn-name" href="#v-sql-error-message">sql-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-exec-result">sql-exec-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-exec-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code></li><li><a class="syn-name" href="#v-sql-exec-result-no-rows">sql-exec-result-no-rows</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-exec-result-no-rows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code></li><li><a class="syn-name" href="#v-sql-isolation-name">sql-isolation-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-isolation-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-last-insert-id">sql-last-insert-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-last-insert-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-named">sql-named</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code></li><li><a class="syn-name" href="#v-sql-row">sql-row</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-row</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a></code></li><li><a class="syn-name" href="#v-sql-rows-affected">sql-rows-affected</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-rows-affected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-sql-tx-isolation">sql-tx-isolation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-tx-isolation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li><a class="syn-name" href="#v-sql-tx-options">sql-tx-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-tx-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code></li><li><a class="syn-name" href="#v-sql-tx-options-default">sql-tx-options-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-tx-options-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code></li><li><a class="syn-name" href="#v-sql-tx-read-only">sql-tx-read-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-tx-read-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-sql-value-eq">sql-value-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-value-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-sql-value-is-null">sql-value-is-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-value-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-sql-value-show">sql-value-show</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-value-show</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sql-value-type-name">sql-value-type-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sql-value-type-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-value-as-bool">value-as-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-value-as-bytes">value-as-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-value-as-f64">value-as-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-value-as-i64">value-as-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-value-as-null-bool">value-as-null-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-value-as-null-bytes">value-as-null-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-value-as-null-f64">value-as-null-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-value-as-null-i64">value-as-null-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-value-as-null-string">value-as-null-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-value-as-null-timestamp">value-as-null-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-null-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-value-as-string">value-as-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-value-as-timestamp">value-as-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">value-as-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-SqlColumnType">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlColumnType</strong></code><span class="doc-sig-links"><a href="#t-SqlColumnType" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Metadata describing a single column in a query result set.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkSqlColumnType"><code><strong class="tok-constructor">MkSqlColumnType</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlError</strong></code><span class="doc-sig-links"><a href="#t-SqlError" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Named SQL, pooling, driver, and row-scanning errors.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SqlErrNoRows"><code><strong class="tok-constructor">SqlErrNoRows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrTxDone"><code><strong class="tok-constructor">SqlErrTxDone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrConnDone"><code><strong class="tok-constructor">SqlErrConnDone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrPoolClosed"><code><strong class="tok-constructor">SqlErrPoolClosed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrBadConn"><code><strong class="tok-constructor">SqlErrBadConn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrDriver"><code><strong class="tok-constructor">SqlErrDriver</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrScan"><code><strong class="tok-constructor">SqlErrScan</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrTypeMismatch"><code><strong class="tok-constructor">SqlErrTypeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrColumnOutOfRange"><code><strong class="tok-constructor">SqlErrColumnOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li><li id="k-SqlErrSkip"><code><strong class="tok-constructor">SqlErrSkip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlExecResult">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlExecResult</strong></code><span class="doc-sig-links"><a href="#t-SqlExecResult" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Summary returned by a statement execution that does not yield rows.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkSqlExecResult"><code><strong class="tok-constructor">MkSqlExecResult</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlIsolation">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlIsolation</strong></code><span class="doc-sig-links"><a href="#t-SqlIsolation" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

Transaction isolation level requested when beginning a transaction.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-IsolationDefault"><code><strong class="tok-constructor">IsolationDefault</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationReadUncommitted"><code><strong class="tok-constructor">IsolationReadUncommitted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationReadCommitted"><code><strong class="tok-constructor">IsolationReadCommitted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationWriteCommitted"><code><strong class="tok-constructor">IsolationWriteCommitted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationRepeatableRead"><code><strong class="tok-constructor">IsolationRepeatableRead</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationSnapshot"><code><strong class="tok-constructor">IsolationSnapshot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationSerializable"><code><strong class="tok-constructor">IsolationSerializable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li><li id="k-IsolationLinearizable"><code><strong class="tok-constructor">IsolationLinearizable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlNamedArg">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlNamedArg</strong></code><span class="doc-sig-links"><a href="#t-SqlNamedArg" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

A query parameter bound either by ordinal position or by explicit name.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SqlPositional"><code><strong class="tok-constructor">SqlPositional</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code></li><li id="k-SqlNamed"><code><strong class="tok-constructor">SqlNamed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlRow">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlRow</strong></code><span class="doc-sig-links"><a href="#t-SqlRow" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

An immutable materialized result row pairing column names with typed values.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkSqlRow"><code><strong class="tok-constructor">MkSqlRow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlTxOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlTxOptions</strong></code><span class="doc-sig-links"><a href="#t-SqlTxOptions" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Options configuring a new database transaction.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkSqlTxOptions"><code><strong class="tok-constructor">MkSqlTxOptions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-SqlValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SqlValue</strong></code><span class="doc-sig-links"><a href="#t-SqlValue" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

A typed SQL scalar value carried by query parameters and result rows.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SqlNull"><code><strong class="tok-constructor">SqlNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlBool"><code><strong class="tok-constructor">SqlBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlInt"><code><strong class="tok-constructor">SqlInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlFloat"><code><strong class="tok-constructor">SqlFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlString"><code><strong class="tok-constructor">SqlString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlBytes"><code><strong class="tok-constructor">SqlBytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li><li id="k-SqlTimestamp"><code><strong class="tok-constructor">SqlTimestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sql/#instances">Data.Sql</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-row-column-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-column-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-row-column-index" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L491" title="View source at line 491">Source</a></span></div>
<div class="doc-body">

Return the zero-based index of the first column with the given name.

</div>
</article>

<article class="doc-entry" id="v-row-columns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-columns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-row-columns" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L464" title="View source at line 464">Source</a></span></div>
<div class="doc-body">

Return the column names of a row.

</div>
</article>

<article class="doc-entry" id="v-row-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code><span class="doc-sig-links"><a href="#v-row-get" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

Retrieve the cell value at zero-based column index `idx`, or `SqlErrColumnOutOfRange`.

</div>
</article>

<article class="doc-entry" id="v-row-get-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-get-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code><span class="doc-sig-links"><a href="#v-row-get-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L503" title="View source at line 503">Source</a></span></div>
<div class="doc-body">

Retrieve the cell value for the first column matching `name`.

</div>
</article>

<article class="doc-entry" id="v-row-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-row-len" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L474" title="View source at line 474">Source</a></span></div>
<div class="doc-body">

Return the number of values in a row.

</div>
</article>

<article class="doc-entry" id="v-row-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">row-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code><span class="doc-sig-links"><a href="#v-row-values" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Return the raw typed cell values of a row.

</div>
</article>

<article class="doc-entry" id="v-scan-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-scan-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L619" title="View source at line 619">Source</a></span></div>
<div class="doc-body">

Scan a non-null `Bool` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-bool-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bool-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-scan-bool-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L691" title="View source at line 691">Source</a></span></div>
<div class="doc-body">

Scan a non-null `Bool` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L631" title="View source at line 631">Source</a></span></div>
<div class="doc-body">

Scan a non-null `Bytes` column at index `idx`, returning a fresh copy.

</div>
</article>

<article class="doc-entry" id="v-scan-bytes-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bytes-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scan-bytes-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L703" title="View source at line 703">Source</a></span></div>
<div class="doc-body">

Scan a non-null `Bytes` column by column name, returning a fresh copy.

</div>
</article>

<article class="doc-entry" id="v-scan-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-scan-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L613" title="View source at line 613">Source</a></span></div>
<div class="doc-body">

Scan a non-null `F64` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-f64-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-f64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-scan-f64-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L685" title="View source at line 685">Source</a></span></div>
<div class="doc-body">

Scan a non-null `F64` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L607" title="View source at line 607">Source</a></span></div>
<div class="doc-body">

Scan a non-null `I64` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-i64-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-i64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-i64-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L679" title="View source at line 679">Source</a></span></div>
<div class="doc-body">

Scan a non-null `I64` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-null-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L655" title="View source at line 655">Source</a></span></div>
<div class="doc-body">

Scan a nullable `Bool` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-null-bool-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-bool-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-bool-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L727" title="View source at line 727">Source</a></span></div>
<div class="doc-body">

Scan a nullable `Bool` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-null-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L667" title="View source at line 667">Source</a></span></div>
<div class="doc-body">

Scan a nullable `Bytes` column at index `idx`, returning a fresh copy when present.

</div>
</article>

<article class="doc-entry" id="v-scan-null-bytes-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-bytes-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-bytes-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L739" title="View source at line 739">Source</a></span></div>
<div class="doc-body">

Scan a nullable `Bytes` column by column name, returning a fresh copy when present.

</div>
</article>

<article class="doc-entry" id="v-scan-null-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L649" title="View source at line 649">Source</a></span></div>
<div class="doc-body">

Scan a nullable `F64` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-null-f64-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-f64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-f64-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L721" title="View source at line 721">Source</a></span></div>
<div class="doc-body">

Scan a nullable `F64` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-null-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L643" title="View source at line 643">Source</a></span></div>
<div class="doc-body">

Scan a nullable `I64` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-null-i64-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-i64-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-i64-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L715" title="View source at line 715">Source</a></span></div>
<div class="doc-body">

Scan a nullable `I64` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-null-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L661" title="View source at line 661">Source</a></span></div>
<div class="doc-body">

Scan a nullable `String` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-null-string-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-string-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-string-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L733" title="View source at line 733">Source</a></span></div>
<div class="doc-body">

Scan a nullable `String` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-null-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L673" title="View source at line 673">Source</a></span></div>
<div class="doc-body">

Scan a nullable timestamp (`I64` Unix nanoseconds) column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-null-timestamp-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-null-timestamp-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-scan-null-timestamp-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L745" title="View source at line 745">Source</a></span></div>
<div class="doc-body">

Scan a nullable timestamp (`I64` Unix nanoseconds) column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-scan-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L625" title="View source at line 625">Source</a></span></div>
<div class="doc-body">

Scan a non-null `String` column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-string-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-string-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-scan-string-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L697" title="View source at line 697">Source</a></span></div>
<div class="doc-body">

Scan a non-null `String` column by column name.

</div>
</article>

<article class="doc-entry" id="v-scan-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L637" title="View source at line 637">Source</a></span></div>
<div class="doc-body">

Scan a non-null timestamp (`I64` Unix nanoseconds) column at index `idx`.

</div>
</article>

<article class="doc-entry" id="v-scan-timestamp-by-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-timestamp-by-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-scan-timestamp-by-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L709" title="View source at line 709">Source</a></span></div>
<div class="doc-body">

Scan a non-null timestamp (`I64` Unix nanoseconds) column by column name.

</div>
</article>

<article class="doc-entry" id="v-sql-arg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-arg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code><span class="doc-sig-links"><a href="#v-sql-arg" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

Construct a positional query argument.

</div>
</article>

<article class="doc-entry" id="v-sql-arg-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-arg-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-arg-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

Return the explicit parameter name, or `None` for positional arguments.

</div>
</article>

<article class="doc-entry" id="v-sql-arg-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-arg-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code><span class="doc-sig-links"><a href="#v-sql-arg-value" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Extract the underlying `SqlValue` from a query argument.

</div>
</article>

<article class="doc-entry" id="v-sql-column-database-type-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-database-type-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-column-database-type-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

Return the database system type name (empty when unsupported).

</div>
</article>

<article class="doc-entry" id="v-sql-column-decimal-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-decimal-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-sql-column-decimal-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L364" title="View source at line 364">Source</a></span></div>
<div class="doc-body">

Return `(precision, scale)` when both decimal size attributes are available.

</div>
</article>

<article class="doc-entry" id="v-sql-column-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sql-column-length" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

Return the declared length for variable-length column types, when known.

</div>
</article>

<article class="doc-entry" id="v-sql-column-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-column-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

Return the column name or alias.

</div>
</article>

<article class="doc-entry" id="v-sql-column-nullable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-nullable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-sql-column-nullable" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Report whether the column may contain NULL, when known by the driver.

</div>
</article>

<article class="doc-entry" id="v-sql-column-precision">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-precision</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sql-column-precision" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

Return the decimal precision, when applicable and known.

</div>
</article>

<article class="doc-entry" id="v-sql-column-scale">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-scale</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sql-column-scale" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L359" title="View source at line 359">Source</a></span></div>
<div class="doc-body">

Return the decimal scale, when applicable and known.

</div>
</article>

<article class="doc-entry" id="v-sql-column-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-column-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a></code><span class="doc-sig-links"><a href="#v-sql-column-type" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L329" title="View source at line 329">Source</a></span></div>
<div class="doc-body">

Construct column metadata.

</div>
</article>

<article class="doc-entry" id="v-sql-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

Format a `SqlError` as a human-readable diagnostic message.

</div>
</article>

<article class="doc-entry" id="v-sql-exec-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-exec-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code><span class="doc-sig-links"><a href="#v-sql-exec-result" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

Construct a statement execution summary.

</div>
</article>

<article class="doc-entry" id="v-sql-exec-result-no-rows">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-exec-result-no-rows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code><span class="doc-sig-links"><a href="#v-sql-exec-result-no-rows" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L396" title="View source at line 396">Source</a></span></div>
<div class="doc-body">

Execution summary for statements (such as DDL) reporting neither an insert ID nor affected row count.

</div>
</article>

<article class="doc-entry" id="v-sql-isolation-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-isolation-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-isolation-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

Human-readable name for a transaction isolation level.

</div>
</article>

<article class="doc-entry" id="v-sql-last-insert-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-last-insert-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sql-last-insert-id" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L400" title="View source at line 400">Source</a></span></div>
<div class="doc-body">

Return the generated insert identifier, if reported by the driver.

</div>
</article>

<article class="doc-entry" id="v-sql-named">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code><span class="doc-sig-links"><a href="#v-sql-named" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L243" title="View source at line 243">Source</a></span></div>
<div class="doc-body">

Construct a named query argument.

</div>
</article>

<article class="doc-entry" id="v-sql-row">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-row</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a></code><span class="doc-sig-links"><a href="#v-sql-row" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L460" title="View source at line 460">Source</a></span></div>
<div class="doc-body">

Construct an immutable `SqlRow` from column names and cell values.

</div>
</article>

<article class="doc-entry" id="v-sql-rows-affected">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-rows-affected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-sql-rows-affected" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L405" title="View source at line 405">Source</a></span></div>
<div class="doc-body">

Return the number of rows affected, if reported by the driver.

</div>
</article>

<article class="doc-entry" id="v-sql-tx-isolation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-tx-isolation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code><span class="doc-sig-links"><a href="#v-sql-tx-isolation" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L313" title="View source at line 313">Source</a></span></div>
<div class="doc-body">

Read the isolation level from transaction options.

</div>
</article>

<article class="doc-entry" id="v-sql-tx-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-tx-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code><span class="doc-sig-links"><a href="#v-sql-tx-options" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

Construct transaction options with an isolation level and read-only flag.

</div>
</article>

<article class="doc-entry" id="v-sql-tx-options-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-tx-options-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code><span class="doc-sig-links"><a href="#v-sql-tx-options-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Default transaction options: default isolation level and read-write access.

</div>
</article>

<article class="doc-entry" id="v-sql-tx-read-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-tx-read-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-sql-tx-read-only" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L318" title="View source at line 318">Source</a></span></div>
<div class="doc-body">

Read whether transaction options request a read-only transaction.

</div>
</article>

<article class="doc-entry" id="v-sql-value-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-value-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-sql-value-eq" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Structural equality on `SqlValue`.

</div>
</article>

<article class="doc-entry" id="v-sql-value-is-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-value-is-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-sql-value-is-null" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Report whether a SQL value is `SqlNull`.

</div>
</article>

<article class="doc-entry" id="v-sql-value-show">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-value-show</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-value-show" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Human-readable representation of a `SqlValue`.

</div>
</article>

<article class="doc-entry" id="v-sql-value-type-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sql-value-type-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sql-value-type-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L184" title="View source at line 184">Source</a></span></div>
<div class="doc-body">

Return the canonical uppercase SQL type name for a value.

</div>
</article>

<article class="doc-entry" id="v-value-as-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-value-as-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L541" title="View source at line 541">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlBool` into a `Bool`.

</div>
</article>

<article class="doc-entry" id="v-value-as-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-value-as-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L553" title="View source at line 553">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlBytes` into a fresh copy of its `Bytes`.

</div>
</article>

<article class="doc-entry" id="v-value-as-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-value-as-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L535" title="View source at line 535">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlFloat` into an `F64`.

</div>
</article>

<article class="doc-entry" id="v-value-as-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-value-as-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L529" title="View source at line 529">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlInt` into an `I64`.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-bool" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L579" title="View source at line 579">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlBool` into `Option Bool`.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L593" title="View source at line 593">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlBytes` into `Option Bytes`, copying non-null buffers.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L572" title="View source at line 572">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlFloat` into `Option F64`.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L565" title="View source at line 565">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlInt` into `Option I64`.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L586" title="View source at line 586">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlString` into `Option String`.

</div>
</article>

<article class="doc-entry" id="v-value-as-null-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-null-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-value-as-null-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L600" title="View source at line 600">Source</a></span></div>
<div class="doc-body">

Decode a nullable `SqlTimestamp` into `Option I64` Unix nanoseconds.

</div>
</article>

<article class="doc-entry" id="v-value-as-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-value-as-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L547" title="View source at line 547">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlString` into a `String`.

</div>
</article>

<article class="doc-entry" id="v-value-as-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">value-as-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-value-as-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Sql/source/#L559" title="View source at line 559">Source</a></span></div>
<div class="doc-body">

Decode a non-null `SqlTimestamp` into Unix nanoseconds (`I64`).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_SqlColumnType"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlColumnType" title="Data.Sql.SqlColumnType">SqlColumnType</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlColumnType</code></span><div class="doc-child-prose">Structural equality instance for SqlColumnType.</div></li>
<li id="i-Eq_SqlError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlError</code></span><div class="doc-child-prose">Structural equality instance for SqlError.</div></li>
<li id="i-Eq_SqlExecResult"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlExecResult" title="Data.Sql.SqlExecResult">SqlExecResult</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlExecResult</code></span><div class="doc-child-prose">Structural equality instance for SqlExecResult.</div></li>
<li id="i-Eq_SqlIsolation"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlIsolation</code></span><div class="doc-child-prose">Structural equality instance for SqlIsolation.</div></li>
<li id="i-Eq_SqlNamedArg"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlNamedArg" title="Data.Sql.SqlNamedArg">SqlNamedArg</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlNamedArg</code></span><div class="doc-child-prose">Structural equality instance for SqlNamedArg.</div></li>
<li id="i-Eq_SqlRow"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlRow" title="Data.Sql.SqlRow">SqlRow</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlRow</code></span><div class="doc-child-prose">Structural equality instance for SqlRow.</div></li>
<li id="i-Eq_SqlTxOptions"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlTxOptions" title="Data.Sql.SqlTxOptions">SqlTxOptions</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlTxOptions</code></span><div class="doc-child-prose">Structural equality instance for SqlTxOptions.</div></li>
<li id="i-Eq_SqlValue"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Eq_SqlValue</code></span><div class="doc-child-prose">Structural equality instance for SqlValue.</div></li>
<li id="i-Show_SqlError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlError" title="Data.Sql.SqlError">SqlError</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Show_SqlError</code></span><div class="doc-child-prose">Text rendering instance for SqlError.</div></li>
<li id="i-Show_SqlIsolation"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlIsolation" title="Data.Sql.SqlIsolation">SqlIsolation</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Show_SqlIsolation</code></span><div class="doc-child-prose">Text rendering instance for SqlIsolation.</div></li>
<li id="i-Show_SqlValue"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sql/#t-SqlValue" title="Data.Sql.SqlValue">SqlValue</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sql.Show_SqlValue</code></span><div class="doc-child-prose">Text rendering instance for SqlValue.</div></li>
</ul>
