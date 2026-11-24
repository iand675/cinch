# Data.Mutable.Map.Float

Explicit IEEE floating key policies without global equality or hashing instances.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Map.Float`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Map.Float.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Mutable.Map.Float](Data.Mutable.Map.Float.md), [Data.Mutable.Var](Data.Mutable.Var.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Explicit IEEE floating key policies without global equality or hashing instances.
Signed zeros share a digest and compare equally. NaNs remain non-reflexive:
they can be stored and enumerated but cannot be found or deleted by key.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6d61702d6633322d696565652d706f6c696379"></a>

## `map-f32-ieee-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-f32-ieee-policy</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-constructor">F32</span>)</code></pre>

Select binary32 IEEE equality explicitly, including non-reflexive NaN keys.

[Source](/reference-sources/Data.Mutable.Map.Float.cinch) · line 42

<a id="value-6d61702d6636342d696565652d706f6c696379"></a>

## `map-f64-ieee-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-f64-ieee-policy</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-constructor">F64</span>)</code></pre>

Select binary64 IEEE equality explicitly, including equivalent signed zeros.

[Source](/reference-sources/Data.Mutable.Map.Float.cinch) · line 46

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Mutable/Map/Float.cinch`. Source SHA-256: `ae5cf39b89f2d062b225740c1b5105ed47934a52755a5886c6705f005a4f317a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
