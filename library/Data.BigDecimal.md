# Data.BigDecimal

Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.BigDecimal</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.BigDecimal/source/">View source</a><a href="/packages/search/?q=%2BData.BigDecimal">Search module</a><a href="/reference-sources/Data.BigDecimal.cinch" download="Data.BigDecimal.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.BigDecimal: exact decimal arithmetic over the bignum `Int`.

A `BigDecimal` is an unscaled `Int` times ten to an `Int` exponent:
`BigDecimal u e` stands for `u * 10^e`. Every value this family
hands out is in CANONICAL FORM: `unscaled` is zero with exponent
zero, or it is not divisible by ten. The smart constructor
`big-decimal` is the only way in (the representation constructor is
not exported, so an unnormalised value cannot exist outside this
module), which makes the canonical form an invariant of the type and
not a convention its users must remember.

Design law shape. Canonical form makes field equality value
equality, and the laws below are what that buys:
  - `0.1` and `0.10` are the same value, because both normalise to
    `BigDecimal 1 (-1)`: `from-string`/`big-decimal` normalise what
    they read, so a decimal has exactly one spelling.
  - round trip: `from-string (to-string x)` has the same fields as
    `x`, and `to-string` never emits exponent notation.
  - arithmetic: `add`/`sub`/`mul` are EXACT (the result is the
    arithmetic on the represented values, with no rounding and no
    precision guess), and `div`/`round` are the only rounding
    operations, each naming its fractional-digit count and its
    rounding mode as arguments.
  - comparison: `cmp` is the order that `sub` induces, `cmp x y` is
    the sign of `sub x y`; `Eq`/`Ord` here are the class spellings
    of that same relation.

NAMED REFUSALS (boundaries this family deliberately does not cross):
  - No `NaN` and no `Infinity`. Ruby's `BigDecimal` carries both
    because its precision is ambient state (`BigDecimal.limit`) and
    a quotient can be asked for without one. Here every rounding
    operation takes its precision, so a quotient always exists at
    the caller's precision when the divisor is not zero; `div` by
    zero is `None` (division is total by construction, and a zero
    divisor has no quotient to name). An unrepresentable remainder is
    rounded to the requested precision, never widened to a float.
  - No implicit coercion to `F64` inside arithmetic. `to-f64` is the
    one conversion out and it renders the exact decimal and lets the
    runtime's correctly-rounded decimal reader parse it, so no
    arithmetic path can acquire a binary rounding error; there is no
    `BigDecimal`/`F64` mixed operation at all.
  - No precision or rounding mode taken from anywhere but the
    arguments: `div` and `round` name both (Ruby's `/` picks up
    `BigDecimal.limit`, its rounding picks up `BigDecimal.mode`),
    and no operand's precision is inferred from the other's.
  - No exponent notation in `to-string` (the plain decimal is the
    only spelling emitted; `to-f64` renders that same text) and no
    exponent parsing in `from-string`: the accepted literal is
    `[+-]? ( digits ( "." digits )? | "." digits )`. Anything else,
    trailing junk included, is `MalformedLiteral`, never a silent
    zero and never a truncated parse.
  - No rounding above the units place: `round` takes a `Nat`
    fractional-digit count, so Ruby's negative precision (round to
    tens and above) is outside the surface.

Home (design/base-library.md): `Data.*` holds concrete types, and
this is the exact decimal. `Rounding` and `BigDecimalError` are this
family's own vocabulary (a rounding policy and a literal error are
not a mathematical kind the scheme names), so they live here with
the type; the `Eq`/`Ord` instances live with their type, as the
scheme requires. Ported item: Ruby's `BigDecimal`
(design/stdlib-parity-ruby.md, "Numeric: `BigDecimal`
(arbitrary-precision decimal; money and measurement)") under the P2
discipline (types and operations, no numeric tower: D125 lists
numeric towers beyond `Semiring` as deliberately out).

</details>

<details class="doc-synopsis"><summary>Synopsis (19 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BigDecimal">BigDecimal</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BigDecimal</strong></code></li><li><a class="syn-name" href="#t-BigDecimalError">BigDecimalError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BigDecimalError</strong></code></li><li><a class="syn-name" href="#t-Rounding">Rounding</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rounding</strong></code></li><li><a class="syn-name" href="#v-abs">abs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-add">add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-big-decimal">big-decimal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">big-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-div">div</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-exponent">exponent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">exponent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-from-int">from-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-from-string">from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimalError" title="Data.BigDecimal.BigDecimalError">BigDecimalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-mul">mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-neg">neg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-render-big-decimal-error">render-big-decimal-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-big-decimal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimalError" title="Data.BigDecimal.BigDecimalError">BigDecimalError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-round">round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-sub">sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li><li><a class="syn-name" href="#v-to-f64">to-f64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unscaled">unscaled</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unscaled</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BigDecimal">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BigDecimal</strong></code><span class="doc-sig-links"><a href="#t-BigDecimal" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

The representation: unscaled times ten to the exponent. Not
exported (the canonical-form invariant above is the reason; D143
is the surface rule that makes the omission bind).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BigDecimal"><code><strong class="tok-constructor">BigDecimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.BigDecimal/#instances">Data.BigDecimal</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.BigDecimal/#instances">Data.BigDecimal</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-BigDecimalError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BigDecimalError</strong></code><span class="doc-sig-links"><a href="#t-BigDecimalError" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

The literal reader's only error. It carries the offending text so
the caller's diagnostic can name it; there is no representation
for "unparsed" that a value could be confused with.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MalformedLiteral"><code><strong class="tok-constructor">MalformedLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimalError" title="Data.BigDecimal.BigDecimalError">BigDecimalError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Rounding">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Rounding</strong></code><span class="doc-sig-links"><a href="#t-Rounding" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

The rounding modes (Ruby's `ROUND_*` constants, spelled as
constructors per the house naming): away from zero, toward zero,
tie away from zero, tie toward zero, tie to even, toward positive
infinity, toward negative infinity.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RoundUp"><code><strong class="tok-constructor">RoundUp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundDown"><code><strong class="tok-constructor">RoundDown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundHalfUp"><code><strong class="tok-constructor">RoundHalfUp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundHalfDown"><code><strong class="tok-constructor">RoundHalfDown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundHalfEven"><code><strong class="tok-constructor">RoundHalfEven</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundCeiling"><code><strong class="tok-constructor">RoundCeiling</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li><li id="k-RoundFloor"><code><strong class="tok-constructor">RoundFloor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-abs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">abs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-abs" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L183" title="View source at line 183">Source</a></span></div>
</article>

<article class="doc-entry" id="v-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-add" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

Exact addition: both operands are read at the smaller exponent
(the scaling is exact, a power of ten times a bignum), so every
digit of either operand survives.

</div>
</article>

<article class="doc-entry" id="v-big-decimal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">big-decimal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-big-decimal" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

The normalising smart constructor: the trailing zeros of the
unscaled part move into the exponent, so `big-decimal 10 (-2)`,
`big-decimal 1 (-1)` and `from-string "0.10"` are one value. Zero
is `BigDecimal 0 0` whatever exponent it arrived with.

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Comparison (the order `sub` induces).

</div>
</article>

<article class="doc-entry" id="v-div">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-div" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

`p` fractional digits, rounded by `mode`, then normalised.

The precision is the fractional-digit count of the quotient
BEFORE normalisation, so `div 10 RoundHalfUp one three` is
`Some 0.3333333333` and `div 2 RoundHalfUp one eight` is `Some
0.13`. The magnitude is rounded (the mode names below read as
their names say from there) and the sign is re-applied, so
`RoundUp` is away from zero, `RoundCeiling` is toward positive
infinity, and `RoundHalfUp` breaks a tie away from zero. A zero
divisor is `None`: there is no result to return and no infinity
to spell.

</div>
</article>

<article class="doc-entry" id="v-exponent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">exponent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-exponent" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L138" title="View source at line 138">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-from-int" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L131" title="View source at line 131">Source</a></span></div>
</article>

<article class="doc-entry" id="v-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimalError" title="Data.BigDecimal.BigDecimalError">BigDecimalError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-from-string" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L400" title="View source at line 400">Source</a></span></div>
<div class="doc-body">

The literal reader: `[+-]? ( digits ( "." digits )? | "." digits )`.
Every other shape (empty, a lone sign, a lone point, a trailing
point, trailing junk, an exponent part) is `MalformedLiteral`
carrying the text.

</div>
</article>

<article class="doc-entry" id="v-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L174" title="View source at line 174">Source</a></span></div>
<div class="doc-body">

Exact multiplication: the unscaled parts multiply and the
exponents add, which is the representation's own product rule.

</div>
</article>

<article class="doc-entry" id="v-neg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">neg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-neg" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L179" title="View source at line 179">Source</a></span></div>
</article>

<article class="doc-entry" id="v-render-big-decimal-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-big-decimal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimalError" title="Data.BigDecimal.BigDecimalError">BigDecimalError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-big-decimal-error" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L112" title="View source at line 112">Source</a></span></div>
</article>

<article class="doc-entry" id="v-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-Rounding" title="Data.BigDecimal.Rounding">Rounding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-round" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Rounding a value to `p` fractional digits: the division by one at
the same precision, written without a divisor test. With `k`
fractional digits to spare the value is exact; otherwise the
divisor is `10^j` and the quotient is `j` divisions by the numeral
ten (truncation toward zero composes), so no runtime zero test
stands between `round` and its result.

</div>
</article>

<article class="doc-entry" id="v-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code><span class="doc-sig-links"><a href="#v-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Subtraction is addition of the negation: the alignment above is
the only place exponents are reconciled, and a negated value is
still canonical, so the exactness argument is the same one.

</div>
</article>

<article class="doc-entry" id="v-to-f64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-f64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-to-f64" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L393" title="View source at line 393">Source</a></span></div>
<div class="doc-body">

The one conversion to a binary float: the exact decimal text,
read by the runtime's correctly-rounded decimal parser. Nothing
else in this module sees an `F64`.

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L367" title="View source at line 367">Source</a></span></div>
<div class="doc-body">

The plain decimal: no exponent notation, sign first, a leading
`0.` when the fraction is wider than the digit run.

</div>
</article>

<article class="doc-entry" id="v-unscaled">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unscaled</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-unscaled" title="Permalink">#</a><a href="/packages/stdlib/Data.BigDecimal/source/#L134" title="View source at line 134">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_BigDecimal"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">Implicit · <code>Data.BigDecimal.Eq_BigDecimal</code></span><div class="doc-child-prose">The class spellings of cmp: canonical form makes the recursive field comparison agree, and routing both through cmp is what keeps Eq/Ord and the function surface one relation rather than two that could drift.</div></li>
<li id="i-Ord_BigDecimal"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">Implicit · <code>Data.BigDecimal.Ord_BigDecimal</code></span></li>
</ul>
