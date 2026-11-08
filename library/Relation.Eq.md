# Relation.Eq

Equality/inequality class vocabulary. Instances live with their types.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Relation.Eq`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Relation.Eq.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Relation.Eq: value-level equality.

The `Eq` class of the base library scheme (design/base-library.md:
`Relation.*` is value-level comparison and decision). A class
constraint entails nothing (the standing no-superbundle rule):
`Ord` does not extend `Eq`, and every function states the
constraints it uses. Instances live with their type.

</details>

# Types and constructors

# Signatures and methods

<a id="class-4571"></a>

## `Eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Eq</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Relation.Eq.cinch)

<a id="method-45712e3d3d"></a>

## `Eq.==`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">==</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-45712e2f3d"></a>

## `Eq./=`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">/=</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



# Operator fixities

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">/=</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">==</span></code></pre>

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Relation/Eq.cinch`. Source SHA-256: `a5dc4a0e15057dac649ace6409c5a84ff531f0aa4ea6d038c3d74ac0afab878c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
