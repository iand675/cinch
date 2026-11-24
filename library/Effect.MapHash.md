# Effect.MapHash

Operating-system entropy seed generation for Data.MapHash.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.MapHash`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.MapHash.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.MapHash](Data.MapHash.md), [Data.Nat](Data.Nat.md), [Data.Wide](Data.Wide.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.MapHash](Data.MapHash.md), [Effect.MapHash](Effect.MapHash.md), [Effect.Random](Effect.Random.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Operating-system entropy seeding for Data.MapHash.

Each call to maphash-make-seed draws 128 bits of fresh entropy from Effect.Random so
independent hash tables obtain distinct, unpredictable keys. All hashing after seed
creation remains pure and deterministic.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6d6170686173682d6d616b652d73656564"></a>

## `maphash-make-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-make-seed</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span>)</code></pre>

Draw 128 bits of operating-system entropy to produce an independent MapSeed.

[Source](/reference-sources/Effect.MapHash.cinch) · line 23

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/MapHash.cinch`. Source SHA-256: `dc8db6edade2291894ffc8ffa2bb2a9cffad3dba14d2fa13ae076285fe1c378d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
