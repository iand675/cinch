# Data.Bytes

Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Bytes`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bytes.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Bytes: boxed byte buffers.

`Bytes` is a core runtime type: indices and lengths are `Nat`, and a
byte is an `InRange 0 255 Nat`. Every operation is total by
construction (design/no-faults.md): a read or write takes evidence
that its index is below `bytes-len b`, a range copy evidence that both
ranges end inside their buffers, a read's result carries its range and
a write's value must prove it. A buffer's length is fixed when it is
made, so a fact over `bytes-len b` survives writes to `b`, and
`bytes-new n` gives `bytes-len out == n` to the name it is bound to.
A length no allocation can hold is the heap `Exhausted` exit.

Module scheme (design/base-library.md): the type's home carries its
instance family: `FromList` (`[1, 2, 3]` at `Bytes` fills a buffer)
over `U8` elements, so a literal's numerals are range-checked as
bytes at compile time, and the key pair `Eq`/`Hash` (a byte slice is
a plausible key: a compiler's names arrive as the lexer's bytes).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62797465732d636f6d70617265"></a>

## `bytes-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-compare</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

The lexicographic order of two buffers' contents as the raw -1/0/1
immediate (the `nat-cmp` convention), read in place: no copy and
no allocation.

[Source](/reference-sources/Data.Bytes.cinch) · line 53

<a id="value-62797465732d636f70792d72616e6765"></a>

## `bytes-copy-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-copy-range</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg2</span> <span class="tok-operator">+</span> <span class="tok-variable">arg4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>), (<span class="tok-variable">arg3</span> <span class="tok-operator">+</span> <span class="tok-variable">arg4</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg1</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg3</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg4</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Unit</span>)))))</code></pre>

`bytes-copy-range dst src di si n`: the `n` bytes of `src` from `si`
into `dst` at `di`, as one runtime `memmove` (the two may be the
same buffer and overlap). A copy loop written here would box a `Nat`
for every index step, so every bulk copy goes through this.

[Source](/reference-sources/Data.Bytes.cinch) · line 48

<a id="value-62797465732d66726f6d2d6c697374"></a>

## `bytes-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-from-list</span> :: (<span class="tok-constructor">List</span> (<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The list's bytes, in order. A byte is an `InRange 0 255 Nat`, so a
list of numerals decides each element at compile time and a
computed list carries its elements' ranges.

[Source](/reference-sources/Data.Bytes.cinch) · line 64

<a id="value-62797465732d66726f6d2d753873"></a>

## `bytes-from-u8s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-from-u8s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The `FromList` buffer: `bytes-from-list`'s shape over `U8`.

[Source](/reference-sources/Data.Bytes.cinch) · line 89

<a id="value-62797465732d676574"></a>

## `bytes-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-get</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg1</span> <span class="tok-operator">&lt;</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>))</code></pre>

[Source](/reference-sources/Data.Bytes.cinch)

<a id="value-62797465732d6c656e"></a>

## `bytes-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-len</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">Nat</span>)</code></pre>

`Bytes`: boxed byte buffers. `bytes-len` comes first: the other
signatures name it. A length is an `i64` in the runtime, so it is at
most `2^63 - 1` and every index below it converts to `I64`. The
index obligation is `nat-sub`'s shape: a loop's own guard (`if i <
bytes-len b`), a fresh buffer's length or an enclosing `using (i <=
bytes-len b)` supplies it. A string literal's read-only backing is
never one of these buffers: it lives inside a `String`, and every
conversion out of a `String` copies.

[Source](/reference-sources/Data.Bytes.cinch) · line 39

<a id="value-62797465732d6e6577"></a>

## `bytes-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-new</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-variable">refined</span> ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">r</span>) <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Data.Bytes.cinch)

<a id="value-62797465732d736574"></a>

## `bytes-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-set</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg1</span> <span class="tok-operator">&lt;</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Unit</span>)))</code></pre>

[Source](/reference-sources/Data.Bytes.cinch)

<a id="value-7368613235362d6279746573"></a>

## `sha256-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

SHA-256 (FIPS 180-4) at content-addressing strength over a
buffer (the P31 content identity; `compiler/runtime/src/text.rs`,
the `cinch.runtime.textSha256` intrinsic, nm-verified there): the
digest as a fresh 32-byte buffer.

[Source](/reference-sources/Data.Bytes.cinch) · line 59

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Bytes.Eq_Bytes` | `Eq Bytes` | Implicit |
| `Data.Bytes.FromList_Bytes` | `FromList Bytes U8` | Implicit |
| `Data.Bytes.Hash_Bytes` | `Hash Bytes` | Implicit |

**`Data.Bytes.Hash_Bytes`**: The buffer's digest (instances live with their type). `Data.String`
hashes its UTF-8 backing through THIS instance, so a string and the
bytes it converts to agree by construction rather than by two
copies of one algorithm.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bytes.cinch`. Source SHA-256: `47f2c8ee650238fae49e2f10f60f84d4d7b1fce1ab2fc8b5588dcd85c50cfffe`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
