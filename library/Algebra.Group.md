# Algebra.Group

Group-style inverse/subtraction vocabulary over admitted carrier instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Group`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Group.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Group: carriers with a total inverse.

The base library scheme's group (design/base-library.md):
`Group` adds `invert` to the associative `<>` with identity
`mempty` (Algebra.Semigroup, Algebra.Monoid). A class constraint
entails nothing (the standing no-superbundle rule): the class
states `invert` alone, and the laws below are the law shape the P18
law-axiom layer will formalize.

Laws (asserted over sample values in compiler/tests/prelude_split.rs):
  left inverse:  invert x <> x == mempty
  right inverse: x <> invert x == mempty
For `Int` the monoid is the semiring's additive one (`add`/`zero`,
Algebra.Semiring), the form the tests state.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module" (design/base-library.md). `Int`'s instance lives in
Data.Int with its family; `Dual`'s lives with its type in
Algebra.Monoid; the function and tuple instances are structural and
here. `Nat` gets no `Group` instance: its subtraction truncates
(there is no additive inverse below zero).

</details>

# Types and constructors

# Signatures and methods

<a id="class-47726f7570"></a>

## `Group`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Group</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Group.cinch)

<a id="method-47726f75702e696e76657274"></a>

## `Group.invert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">invert</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Group</span>.<span class="tok-constructor">Group</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Algebra.Group.Group_Function` | `Algebra.Group.Group (t -> u)` | Implicit |
| `Algebra.Group.Group_Pair` | `Algebra.Group.Group (t, u)` | Implicit |

**`Algebra.Group.Group_Function`**: Structural: pointwise inversion of functions into a group. The
method is eta-expanded (`invert f` at x is `invert (f x)`)
because the bootstrap checks equations against every arrow of the
substituted method type.
**`Algebra.Group.Group_Pair`**: Structural: a pair inverts componentwise, each side under its own
constraint.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Group.cinch`. Source SHA-256: `e247f07f004aed06c6c258d5b4184f12186ffcc1bbefec740799112f38168d3c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
