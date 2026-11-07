# Data.FromList

The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.FromList`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.FromList.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Core](Data.Core.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.FromList: the collection-literal construction class.

`[a, b, c]` desugars to `from-list` at the expected type's
`FromList c e` instance; unconstrained it defaults to `List`
(surface grammar local amendments section 10). Instances opt in per
container and live with their type (Data.List, Data.Bytes,
Data.Set).

</details>

# Types and constructors

# Signatures and methods

<a id="class-46726f6d4c697374"></a>

## `FromList`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">FromList</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span></code></pre>

Collection literals (surface grammar local amendments, section 10):
`[a, b, c]` desugars to `from-list` at the expected type's `FromList
c e` instance and `{k => v}` to `from-map` over a `List (k, v)` of
pairs at the expected type's `FromMap m k v` instance; unconstrained
they default to `List` and `Map`. Instances opt in per container.

[Source](/reference-sources/Data.FromList.cinch) · line 19

<a id="method-46726f6d4c6973742e66726f6d2d6c697374"></a>

## `FromList.from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">FromList</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span>) =&gt; (<span class="tok-constructor">List</span> <span class="tok-variable">e</span> -&gt; <span class="tok-variable">c</span>)</code></pre>



# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/FromList.cinch`. Source SHA-256: `62cc79932ad61b2dc2a269fa592573ca8d514cc0f99ff34ddbf0be60160fd3d8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
