# Data.Compare

Explicit scalar comparison policies, NaN-aware floating orders, and first non-default selection.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Compare`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Compare.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Char](Data.Char.md), [Data.Core](Data.Core.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Compare](Data.Compare.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Three-way comparison and first non-default selection with explicit policies.
Compare supplies an order without implying Relation.Eq or Relation.Ord.
Floating orders place every NaN before every number, identify all NaNs,
and identify negative and positive zero. Default supplies the distinguished
value and its equality test; custom types choose these explicitly.

</details>

# Types and constructors

# Signatures and methods

<a id="class-436f6d70617265"></a>

## `Compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Compare</span> <span class="tok-variable">a</span></code></pre>

A three-way order. Instances must be reflexive, antisymmetric and transitive.
Equivalent values need not have identical representations or an Eq instance.

[Source](/reference-sources/Data.Compare.cinch) · line 22

<a id="method-436f6d706172652e636f6d70617265"></a>

## `Compare.compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">compare</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Compare</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Return LT, EQ or GT according to this capability's explicit value order.

<a id="class-44656661756c74"></a>

## `Default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Default</span> <span class="tok-variable">a</span></code></pre>

A distinguished default value and the predicate recognizing its equivalents.
The predicate must recognize default-value. Floating zeros are equivalent;
NaNs are non-default. Instances do not imply global equality or ordering.

[Source](/reference-sources/Data.Compare.cinch) · line 36

<a id="method-44656661756c742e64656661756c742d76616c7565"></a>

## `Default.default-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">default-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Default</span> <span class="tok-variable">a</span>) =&gt; <span class="tok-variable">a</span></code></pre>

The result when every candidate is default or the candidate list is empty.

<a id="method-44656661756c742e69732d64656661756c74"></a>

## `Default.is-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-default</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Default</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a candidate equals this capability's distinguished default.

# Values

<a id="value-636f6d706172652d663332"></a>

## `compare-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">compare-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Compare F32 values with NaNs first, all NaNs equivalent, and signed zeros equivalent.

[Source](/reference-sources/Data.Compare.cinch) · line 56

<a id="value-636f6d706172652d663634"></a>

## `compare-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">compare-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Compare F64 values with NaNs first, all NaNs equivalent, and signed zeros equivalent.

[Source](/reference-sources/Data.Compare.cinch) · line 71

<a id="value-66697273742d6e6f6e2d64656661756c74"></a>

## `first-non-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">first-non-default</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">defaults</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Default</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Return the first non-default candidate unchanged, or default-value.
The walk stops at that candidate. An all-default list returns the canonical
default, so a list containing only floating negative zeros returns positive zero.

[Source](/reference-sources/Data.Compare.cinch) · line 45

<a id="value-66697273742d6e6f6e2d64656661756c742d6279"></a>

## `first-non-default-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">first-non-default-by</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)))</code></pre>

Select with an explicit fallback and default predicate, without typeclass
constraints. The predicate runs left to right until the first False result.

[Source](/reference-sources/Data.Compare.cinch) · line 50

<a id="value-6c657373"></a>

## `less`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">less</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Compare</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test strict precedence in the supplied order. Equivalent values return False.

[Source](/reference-sources/Data.Compare.cinch) · line 27

<a id="value-6c6573732d663332"></a>

## `less-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">less-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Strict F32 precedence with NaNs first and signed zeros equivalent.

[Source](/reference-sources/Data.Compare.cinch) · line 65

<a id="value-6c6573732d663634"></a>

## `less-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">less-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Strict F64 precedence with NaNs first and signed zeros equivalent.

[Source](/reference-sources/Data.Compare.cinch) · line 80

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Compare.Compare_Char` | `Data.Compare.Compare Char` | Implicit |
| `Data.Compare.Compare_F32` | `Data.Compare.Compare F32` | Implicit |
| `Data.Compare.Compare_F64` | `Data.Compare.Compare F64` | Implicit |
| `Data.Compare.Compare_I16` | `Data.Compare.Compare I16` | Implicit |
| `Data.Compare.Compare_I32` | `Data.Compare.Compare I32` | Implicit |
| `Data.Compare.Compare_I64` | `Data.Compare.Compare I64` | Implicit |
| `Data.Compare.Compare_I8` | `Data.Compare.Compare I8` | Implicit |
| `Data.Compare.Compare_Int` | `Data.Compare.Compare Int` | Implicit |
| `Data.Compare.Compare_Isize` | `Data.Compare.Compare Isize` | Implicit |
| `Data.Compare.Compare_Nat` | `Data.Compare.Compare Nat` | Implicit |
| `Data.Compare.Compare_String` | `Data.Compare.Compare Data.String.String` | Implicit |
| `Data.Compare.Compare_U16` | `Data.Compare.Compare U16` | Implicit |
| `Data.Compare.Compare_U32` | `Data.Compare.Compare U32` | Implicit |
| `Data.Compare.Compare_U64` | `Data.Compare.Compare U64` | Implicit |
| `Data.Compare.Compare_U8` | `Data.Compare.Compare U8` | Implicit |
| `Data.Compare.Compare_Usize` | `Data.Compare.Compare Usize` | Implicit |
| `Data.Compare.Default_Bool` | `Data.Compare.Default Bool` | Implicit |
| `Data.Compare.Default_Char` | `Data.Compare.Default Char` | Implicit |
| `Data.Compare.Default_F32` | `Data.Compare.Default F32` | Implicit |
| `Data.Compare.Default_F64` | `Data.Compare.Default F64` | Implicit |
| `Data.Compare.Default_I16` | `Data.Compare.Default I16` | Implicit |
| `Data.Compare.Default_I32` | `Data.Compare.Default I32` | Implicit |
| `Data.Compare.Default_I64` | `Data.Compare.Default I64` | Implicit |
| `Data.Compare.Default_I8` | `Data.Compare.Default I8` | Implicit |
| `Data.Compare.Default_Int` | `Data.Compare.Default Int` | Implicit |
| `Data.Compare.Default_Isize` | `Data.Compare.Default Isize` | Implicit |
| `Data.Compare.Default_Nat` | `Data.Compare.Default Nat` | Implicit |
| `Data.Compare.Default_String` | `Data.Compare.Default Data.String.String` | Implicit |
| `Data.Compare.Default_U16` | `Data.Compare.Default U16` | Implicit |
| `Data.Compare.Default_U32` | `Data.Compare.Default U32` | Implicit |
| `Data.Compare.Default_U64` | `Data.Compare.Default U64` | Implicit |
| `Data.Compare.Default_U8` | `Data.Compare.Default U8` | Implicit |
| `Data.Compare.Default_Unit` | `Data.Compare.Default Unit` | Implicit |
| `Data.Compare.Default_Usize` | `Data.Compare.Default Usize` | Implicit |

**`Data.Compare.Compare_Char`**: The explicit value order for Char.
**`Data.Compare.Compare_F32`**: The explicit NaN-first order for F32.
**`Data.Compare.Compare_F64`**: The explicit NaN-first order for F64.
**`Data.Compare.Compare_I16`**: The explicit value order for I16.
**`Data.Compare.Compare_I32`**: The explicit value order for I32.
**`Data.Compare.Compare_I64`**: The explicit value order for I64.
**`Data.Compare.Compare_I8`**: The explicit value order for I8.
**`Data.Compare.Compare_Int`**: The explicit value order for Int.
**`Data.Compare.Compare_Isize`**: The explicit value order for Isize.
**`Data.Compare.Compare_Nat`**: The explicit value order for Nat.
**`Data.Compare.Compare_String`**: The explicit value order for String.
**`Data.Compare.Compare_U16`**: The explicit value order for U16.
**`Data.Compare.Compare_U32`**: The explicit value order for U32.
**`Data.Compare.Compare_U64`**: The explicit value order for U64.
**`Data.Compare.Compare_U8`**: The explicit value order for U8.
**`Data.Compare.Compare_Usize`**: The explicit value order for Usize.
**`Data.Compare.Default_Bool`**: The distinguished value default for Bool.
**`Data.Compare.Default_Char`**: The canonical zero default for Char.
**`Data.Compare.Default_F32`**: The canonical zero default for F32.
**`Data.Compare.Default_F64`**: The canonical zero default for F64.
**`Data.Compare.Default_I16`**: The canonical zero default for I16.
**`Data.Compare.Default_I32`**: The canonical zero default for I32.
**`Data.Compare.Default_I64`**: The canonical zero default for I64.
**`Data.Compare.Default_I8`**: The canonical zero default for I8.
**`Data.Compare.Default_Int`**: The canonical zero default for Int.
**`Data.Compare.Default_Isize`**: The canonical zero default for Isize.
**`Data.Compare.Default_Nat`**: The canonical zero default for Nat.
**`Data.Compare.Default_String`**: The empty string default for String.
**`Data.Compare.Default_U16`**: The canonical zero default for U16.
**`Data.Compare.Default_U32`**: The canonical zero default for U32.
**`Data.Compare.Default_U64`**: The canonical zero default for U64.
**`Data.Compare.Default_U8`**: The canonical zero default for U8.
**`Data.Compare.Default_Unit`**: The distinguished value default for Unit.
**`Data.Compare.Default_Usize`**: The canonical zero default for Usize.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Compare.cinch`. Source SHA-256: `2c8339cfe0420a3ad27bed174308286e4c7e377318adf18dd8dcd5ce172686cb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
