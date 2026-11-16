# Text.Checksum

Incremental Adler-32, CRC-32 and CRC-64 checksums with explicit reflected polynomials.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Checksum`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Checksum.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Text.Adler32](Text.Adler32.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md), [Text.Checksum](Text.Checksum.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Incremental Adler-32, CRC-32 and CRC-64 checksums over byte buffers.
Updates accept finished checksums: updating the checksum of a prefix with a
suffix equals checksumming their concatenation. Start Adler-32 at 1 and CRCs
at 0. Polynomial arguments use reflected representations. Text.Adler32 supplies
the portable SIMD accumulator; CRCs consume native eight-byte blocks.
CRC-32C uses an ISA instruction after CPU feature detection on x86. AArch64
profiles with ArmCrc32 accelerate both IEEE and Castagnoli. Other cases use portable
inline LLVM shift registers. These checksums do not authenticate messages.

</details>

# Types and constructors

# Signatures and methods

<a id="class-41726d4372633332"></a>

## `ArmCrc32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">ArmCrc32</span> </code></pre>

Target-profile evidence that AArch64 CRC instructions may execute.
Only profiles that guarantee the CRC extension should name this capability.

[Source](/reference-sources/Text.Checksum.cinch) · line 25

# Values

<a id="value-61646c657233322d73756d"></a>

## `adler32-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compute Adler-32 over all input bytes, starting at the standard state 1. The empty checksum is 1.

[Source](/reference-sources/Text.Checksum.cinch) · line 30

<a id="value-61646c657233322d757064617465"></a>

## `adler32-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-update</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Continue a finished Adler-32 checksum with another chunk. Empty input preserves the supplied state.

[Source](/reference-sources/Text.Checksum.cinch) · line 34

<a id="value-63726333322d6361737461676e6f6c69"></a>

## `crc32-castagnoli`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-castagnoli</span> :: <span class="tok-constructor">U32</span></code></pre>

The reflected Castagnoli CRC-32C polynomial, 0x82F63B78.

[Source](/reference-sources/Text.Checksum.cinch) · line 42

<a id="value-63726333322d69656565"></a>

## `crc32-ieee`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-ieee</span> :: <span class="tok-constructor">U32</span></code></pre>

The reflected IEEE CRC-32 polynomial, 0xEDB88320.

[Source](/reference-sources/Text.Checksum.cinch) · line 38

<a id="value-63726333322d6b6f6f706d616e"></a>

## `crc32-koopman`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-koopman</span> :: <span class="tok-constructor">U32</span></code></pre>

The reflected Koopman CRC-32 polynomial, 0xEB31D82E.

[Source](/reference-sources/Text.Checksum.cinch) · line 46

<a id="value-63726333322d73756d"></a>

## `crc32-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Compute the IEEE CRC-32 checksum. The empty checksum is zero.

[Source](/reference-sources/Text.Checksum.cinch) · line 50

<a id="value-63726333322d757064617465"></a>

## `crc32-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-update</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)))</code></pre>

Continue a finished CRC-32 with another chunk. Arguments are reflected polynomial, previous checksum, and bytes; initial/final complementing is internal.

[Source](/reference-sources/Text.Checksum.cinch) · line 58

<a id="value-63726333322d77697468"></a>

## `crc32-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32-with</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Compute CRC-32 with the supplied reflected polynomial, starting from the finished state zero.

[Source](/reference-sources/Text.Checksum.cinch) · line 54

<a id="value-63726336342d65636d61"></a>

## `crc64-ecma`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-ecma</span> :: <span class="tok-constructor">U64</span></code></pre>

The reflected ECMA CRC-64 polynomial, 0xC96C5795D7870F42.

[Source](/reference-sources/Text.Checksum.cinch) · line 87

<a id="value-63726336342d69736f"></a>

## `crc64-iso`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-iso</span> :: <span class="tok-constructor">U64</span></code></pre>

The reflected ISO CRC-64 polynomial, 0xD800000000000000.

[Source](/reference-sources/Text.Checksum.cinch) · line 83

<a id="value-63726336342d73756d"></a>

## `crc64-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-sum</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Compute CRC-64 with the supplied reflected polynomial. The empty checksum is zero.

[Source](/reference-sources/Text.Checksum.cinch) · line 91

<a id="value-63726336342d757064617465"></a>

## `crc64-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc64-update</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>)))</code></pre>

Continue a finished CRC-64. Arguments are reflected polynomial, previous checksum, and bytes; an empty update preserves the checksum.

[Source](/reference-sources/Text.Checksum.cinch) · line 95

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Checksum.cinch`. Source SHA-256: `d8fa3abc1d3fed0eea57aeeffb623544bf0c6efb8ef8d5e7ccda15e2018e025e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
