# Category.Monad

Container-dependent composition through the admitted higher-kinded class vocabulary.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Category.Monad`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Category.Monad.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Category.Monad: the monad class over type constructors.

Monad is `>>=` (infixl 1, decision D2's per-module fixity
declarations) and `join`. There is NO `return` method (`return` is
handler-clause syntax; decision D152): `pure` lives on
Category.Applicative, and the monad laws state `pure` under an added
`Applicative f` constraint (the bare-`pure` rule, decision D153).
The import below is explicit because a class constraint entails
nothing (the standing no-superbundle rule): `Monad` does not extend
`Applicative`, and every function states the constraints it uses.

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition; this class abstracts over
containers. The two mechanisms do not interact.

Instance homes: the function instance is structural and lives here
permanently; the List, Option and Result instances got theirs at
the prelude split (Data.List/Data.Option/Data.Result).

The combinators below the class are the free functions `base`'s
Control.Monad carries (`>=>`, `<=<`); each states its constraint and
is reached with an explicit `import Category.Monad` (the facade
re-exports no class home). The traversals that combine this class
with a container live in Category.Traversable, which imports this
module (the class homes collect in dependency order).

</details>

# Types and constructors

# Signatures and methods

<a id="class-4d6f6e6164"></a>

## `Monad`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Monad</span> <span class="tok-variable">f</span></code></pre>

[Source](/reference-sources/Category.Monad.cinch)

<a id="method-4d6f6e61642e3e3e3d"></a>

## `Monad.>>=`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&gt;&gt;=</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> -&gt; <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Monad</span> <span class="tok-variable">f</span>) =&gt; ((<span class="tok-variable">f</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">f</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">f</span> <span class="tok-variable">b</span>)))</code></pre>



<a id="method-4d6f6e61642e6a6f696e"></a>

## `Monad.join`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">join</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> -&gt; <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Monad</span> <span class="tok-variable">f</span>) =&gt; ((<span class="tok-variable">f</span> ((<span class="tok-variable">f</span> <span class="tok-variable">a</span>))) -&gt; (<span class="tok-variable">f</span> <span class="tok-variable">a</span>))</code></pre>



# Values

<a id="value-3c3d3c"></a>

## `<=<`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&lt;=&lt;</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> -&gt; <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <span class="tok-constructor">Monad</span> <span class="tok-variable">m</span>) =&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">c</span>)) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">c</span>))))</code></pre>

Right to left: `g <=< f` is `f >=> g`, the direction that reads
like function composition. Eta-expanded to the signature's arity
(the bootstrap checks a definition against every arrow of its
signature, so a body that stops at the second arrow is read as arity
two against an arity-three signature).

[Source](/reference-sources/Category.Monad.cinch) · line 60

<a id="value-3e3d3e"></a>

## `>=>`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&gt;=&gt;</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> -&gt; <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <span class="tok-constructor">Monad</span> <span class="tok-variable">m</span>) =&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">b</span>)) -&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">c</span>)) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">m</span> <span class="tok-variable">c</span>))))</code></pre>

Left to right: `f >=> g` is the sequenced pair, the order effects
run and results flow in.

[Source](/reference-sources/Category.Monad.cinch) · line 52

# Operator fixities

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">infixr</span> <span class="tok-number">1</span> <span class="tok-operator">&lt;=&lt;</span>
<span class="tok-keyword">infixr</span> <span class="tok-number">1</span> <span class="tok-operator">&gt;=&gt;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">1</span> <span class="tok-operator">&gt;&gt;=</span></code></pre>

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Category.Monad.Monad_Function` | `Monad (((->) r))` | Implicit |

**`Category.Monad.Monad_Function`**: Structural: feed a function's result into the continuation.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Category/Monad.cinch`. Source SHA-256: `6118a2adc3dc804a4d442197f5197ef9975480487dfaa4a2df713b517fa8675f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
