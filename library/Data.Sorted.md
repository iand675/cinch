# Data.Sorted

Lists refined by sortedness, with construction and combination that preserve ordering.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Sorted`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Sorted.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Sorted: the sorted list as a refinement (refinement types
design; decision D151).

`type Sorted = refine (xs :: List Nat) with Ordered xs`: `Sorted` IS
`List Nat` at runtime (the witness is `Prop`-erased, decision D31).
The predicate form: `Ordered` decides lists of numerics (the
ordering relation over an abstract element type is refused naming the
predicate: the `Dec` world decides numerics, not dictionaries).

The total constructor `to-sorted` discharges at runtime through the
`Dec`/`decide` world and returns `Option`; the transparent
constructor `Sorted xs w` takes the list and its witness (a `Dec
(Ordered xs)` discharge that erases, or `Yes` evidence).

The instance below is the sorted MERGE, not list append: append of
two sorted lists is not sorted in general, so the container's `<>`
does NOT transfer (the preservation law rejects it naming
`Sorted.preserve-<>`; asserted in compiler/tests/refinements.rs,
together with the deliberately non-preserving reverse-append).

</details>

# Types and constructors

<a id="type-536f72746564"></a>

## `Sorted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">type</span> <span class="tok-constructor">Sorted</span>  = <span class="tok-variable">refine</span> (<span class="tok-variable">xs</span> :: <span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) <span class="tok-keyword">with</span> <span class="tok-constructor">Ordered</span> <span class="tok-variable">xs</span></code></pre>

[Source](/reference-sources/Data.Sorted.cinch)

# Values

<a id="value-6d65726765"></a>

## `merge`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">merge</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)))</code></pre>

The sorted merge: the output of merging sorted lists is sorted
(preservation), and merge is associative (`Semigroup.assoc`);
both checked sample-wise by the law layer.

[Source](/reference-sources/Data.Sorted.cinch) · line 36

<a id="value-746f2d736f72746564"></a>

## `to-sorted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-sorted</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Sorted</span>)</code></pre>

The total constructor (refinement types design: the total API).

[Source](/reference-sources/Data.Sorted.cinch) · line 28

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Sorted.Semigroup_Sorted` | `Semigroup Sorted` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Sorted.cinch`. Source SHA-256: `860d71b688c670fe7cbb0e116d6f72e3971fffb48489e2752592859a2d813611`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
