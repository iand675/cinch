# Prelude

The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prelude`. This module is implicitly available.

[Download exact module source](/reference-sources/Prelude.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Bits](Algebra.Bits.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Enum](Data.Enum.md), [Data.FromList](Data.FromList.md), [Data.FromMap](Data.FromMap.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Producer](Data.Producer.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Sym](Data.Sym.md), [Data.Target](Data.Target.md), [Effect.Concurrent](Effect.Concurrent.md), [Effect.Io](Effect.Io.md), [Effect.Sequencing](Effect.Sequencing.md), [Function.Combinators](Function.Combinators.md), `Prelude.Generated` (compiler-generated vocabulary), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Prelude: the curated re-export facade (decision D107's export
machinery, D143's surface rule, D146's re-export forms).

The prelude is no longer one big module: every type, class and
function family lives in its scheme home (design/base-library.md:
`Category.*`, `Algebra.*`, `Relation.*`, `Data.*`, `Function.*`,
`Text.*`, `Effect.*`), and this module is the implicit base layer
every program sees. Its surface is EXACTLY the names the old
single-module prelude declared (the compatibility rule is absolute:
every program that checked before the split checks unchanged), by
`export module X` re-exports of the homes' own curated surfaces and
`export A.b` items for the `Text.Show` conversions. Curation
trimming is deliberately NOT done here; a trim proposal is a
separate step. One addition is deliberate: `Data.Sym`'s `intern` and
`sym-name` (interned symbols, primitive types specification section
9) extend the surface, since that specification puts both in the
prelude.

The negative space is real: a name this list does not name is
unreachable through `Prelude` (reach it with an explicit `import`
of its home). That covers the new base-library families
(Algebra.Group, Algebra.Semiring, Data.Set, Data.Hash,
Data.HashMap, Text.Show's class, Text.Codec) and every home's
private helpers.

`Prelude.Generated` is the WIT adapter vocabulary module: the
generated vocabulary types (WebAssembly and WASI specification
section 3) are declared there and re-exported here, so they stay
part of the implicit surface exactly as when they appended to the
old single module.

The operator table (decision D90's order) is inherited through these
exports from the defining homes. Only an operator's declaring module
may declare its fixity; this facade carries those facts without
redeclaring them (user direction, 2026-10-30).

</details>

# Types and constructors

<a id="type-415658325f323536"></a>

## `AVX2_256`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">AVX2_256</span></code></pre>

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58).

[Source](/reference-sources/Data.Target.cinch) · line 47

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AVX2_256Tag</span> :: <span class="tok-constructor">AVX2_256</span></code></pre>

<a id="type-436170417661696c61626c65"></a>

## `CapAvailable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CapAvailable</span> (<span class="tok-variable">t</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">w</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it. A rewrite's `require` clause names it.

[Source](/reference-sources/Data.Target.cinch) · line 69

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Available</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> . (<span class="tok-constructor">VecU32Target</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> -&gt; <span class="tok-constructor">CapAvailable</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>)</code></pre>

<a id="type-436f6e666967"></a>

## `Config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Config</span></code></pre>

[Source](/reference-sources/Effect.Io.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Config</span>)</code></pre>

<a id="type-436f6e6669674572726f72"></a>

## `ConfigError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ConfigError</span></code></pre>

The worked vocabulary of the sequencing chapter's `loadConfig`
sketch (runtime capabilities, effects and sequencing
specification), provided over the filesystem like Effect.Fs's
operations. A `Path` (Data.Path) names bytes to read;
`read-config-bytes` performs the read under the `Io r`
capability and reports a missing or unreadable file as a
`ConfigError`. `parse-config` is pure over the read bytes: the
bootstrap's config document is the payload, and an empty
document is not a config.

[Source](/reference-sources/Effect.Io.cinch) · line 30

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BadConfig</span> :: <span class="tok-constructor">ConfigError</span></code></pre>

<a id="type-446f776e"></a>

## `Down`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Down</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The order-reversing newtype: `Down x` is `x` with the order
swapped, so `comparing (\p -> Down (key p))` sorts by `key`
descending. Equality is unaffected by the flip (only the four
ordering methods reverse).

[Source](/reference-sources/Relation.Ord.cinch) · line 46

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Down</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Down</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-45696768744c616e6573"></a>

## `EightLanes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">EightLanes</span></code></pre>

[Source](/reference-sources/Data.Target.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EightLanesTag</span> :: <span class="tok-constructor">EightLanes</span></code></pre>

<a id="type-496e52616e6765"></a>

## `InRange`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">type</span> <span class="tok-constructor">InRange</span> (<span class="tok-variable">lo</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">hi</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) = <span class="tok-variable">refine</span> (<span class="tok-variable">x</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> (<span class="tok-variable">lo</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">x</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">x</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">hi</span>)</code></pre>

A range of a number, the result type of every operation whose
result lies in known bounds by construction (`u32-to-i64 :: U32 ->
InRange 0 4294967295 I64`, `bytes-get`'s `InRange 0 255 Nat`,
`char-code`'s `InRange 0 1114111 Nat`). The bounds are inclusive
index parameters, so the predicate is two comparisons the
difference-logic discharge reads directly: a let-bound result
carries `lo <= x && x <= hi` as a fact, an application in an
obligation carries it for its term, and where no range is demanded
the value is its number (`c - 48`, `nat-add c 1`).

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 41

<a id="type-4a736f6e"></a>

## `Json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Json</span></code></pre>

The `Json`-shaped value a file converter binds for a JSON file with
no declared type (surface grammar local amendments section 11); with
a declared record type the converter decodes into it instead. An
array is a cons spine (`JArr item tail`, ended by `JArrEnd`) and an
object entry is `JObj key value tail` (ended by `JObjEnd`): the
positivity discipline allows only direct recursive occurrences, so
the payloads stay inside the family.

[Source](/reference-sources/Data.Json.cinch) · line 23

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JNull</span> :: <span class="tok-constructor">Json</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JBool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Json</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JNum</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Json</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JFloat</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Json</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JStr</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Json</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JArr</span> :: (<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Json</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JObj</span> :: (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Json</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JArrEnd</span> :: <span class="tok-constructor">Json</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JObjEnd</span> :: <span class="tok-constructor">Json</span></code></pre>

<a id="type-4c697374"></a>

## `List`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">List</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Core.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Nil</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">List</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Cons</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-4c6f61644572726f72"></a>

## `LoadError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LoadError</span></code></pre>

[Source](/reference-sources/Effect.Concurrent.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadError</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">LoadError</span>)</code></pre>

<a id="type-4d6170"></a>

## `Map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Map</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The ordered map: an unbalanced binary search tree over `Ord k`
(decision D105). `map-insert` keeps the left subtree strictly
smaller and the right strictly greater, so a repeated key replaces
the stored value in place.

[Source](/reference-sources/Data.Map.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Tip</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> . <span class="tok-constructor">Map</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bin</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> . (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; (<span class="tok-constructor">Map</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-constructor">Map</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; <span class="tok-constructor">Map</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))))</code></pre>

<a id="type-4e454f4e3634"></a>

## `NEON64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">NEON64</span></code></pre>

[Source](/reference-sources/Data.Target.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NEON64Tag</span> :: <span class="tok-constructor">NEON64</span></code></pre>

<a id="type-4e6f6e5a65726f"></a>

## `NonZero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">type</span> <span class="tok-constructor">NonZero</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) = <span class="tok-variable">refine</span> (<span class="tok-variable">n</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> <span class="tok-constructor">NonZero</span> <span class="tok-variable">n</span></code></pre>

The divisor type, declared here because the `Div`/`Rem` classes
and the `Nat`/`Int` homes (checked right after this module) name
it in their signatures; its total constructors and instances live
in its public home, `Data.NonZero`. One refinement states the
predicate for every carrier it decides (`NonZero Nat`, `NonZero
Int`, a machine integer): the carrier is the container, so a
`NonZero Nat` IS its `Nat` at runtime, and a numeral decides its
predicate at compile time.

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 30

<a id="type-4f7074696f6e"></a>

## `Option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Option</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Core.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">None</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Option</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Some</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-4f72646572696e67"></a>

## `Ordering`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ordering</span></code></pre>

[Source](/reference-sources/Data.Ordering.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LT</span> :: <span class="tok-constructor">Ordering</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EQ</span> :: <span class="tok-constructor">Ordering</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GT</span> :: <span class="tok-constructor">Ordering</span></code></pre>

<a id="type-50617468"></a>

## `Path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Path</span></code></pre>

The path text (POSIX; UTF-8 at the boundary).

[Source](/reference-sources/Data.Path.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Path</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Path</span>)</code></pre>

<a id="type-50726f6475636572"></a>

## `Producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Producer</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A producer: a state with its step function.

[Source](/reference-sources/Data.Producer.cinch) · line 23

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Producer</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . ((<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-52657175657374"></a>

## `Request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Request</span></code></pre>

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

[Source](/reference-sources/Effect.Concurrent.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Request</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Request</span>)</code></pre>

<a id="type-526573756c74"></a>

## `Result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Result</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

`Result` is here for the same reason: `Data.Result`'s
`partition-results` and `results` consume a `List`, and it too is
checked before `Data.List`.

[Source](/reference-sources/Data.Core.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Err</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ok</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-5363616c61626c655368617065"></a>

## `ScalableShape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ScalableShape</span></code></pre>

[Source](/reference-sources/Data.Target.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ScalableShapeTag</span> :: <span class="tok-constructor">ScalableShape</span></code></pre>

<a id="type-53746570"></a>

## `Step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Step</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The vector step form (modules section 6): `Done`, `Skip(state)` and
`Yield(value, state)`. Producer pipelines lower to state machines.

[Source](/reference-sources/Data.Producer.cinch) · line 17

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Done</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Skip</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Yield</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-537472696e67"></a>

## `String`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">String</span></code></pre>

The representation constructor: module-private (the immutability
contract: no API hands out a string's backing). A `StrBuf` wraps a
buffer that is either fresh (`bytes-new` filled here) or a copy,
never one a caller retains.

[Source](/reference-sources/Data.String.cinch) · line 70

<a id="type-54616b655374"></a>

## `TakeSt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TakeSt</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TakeSt</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">TakeSt</span> <span class="tok-variable">s</span>))</code></pre>

# Signatures and methods

<a id="class-416464"></a>

## `Add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Add</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Arithmetic.cinch)

<a id="method-4164642e2b"></a>

## `Add.+`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">+</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Add</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="class-42697473"></a>

## `Bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span></code></pre>

Bit manipulation (primitive types specification section 4).

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers. All methods are total,
following the specification: the bitwise operations are total,
shifts reduce the count modulo the width (the wrapping family), and
rotates are total. The checked shift family stays as the per-type
`checkedShlX`/`checkedShrX` primitives. `test-bit`/`set-bit`/
`clear-bit`/`complement-bit` are expressed through the mask rule: bit
position n means position n modulo the width, consistent with the
shift rule.

[Source](/reference-sources/Algebra.Bits.cinch) · line 41

<a id="method-426974732e6269742d616e64"></a>

## `Bits.bit-and`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d6f72"></a>

## `Bits.bit-or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d786f72"></a>

## `Bits.bit-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d6e6f74"></a>

## `Bits.bit-not`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-426974732e73686966742d6c656674"></a>

## `Bits.shift-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shift-left</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e73686966742d7269676874"></a>

## `Bits.shift-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shift-right</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e726f746174652d6c656674"></a>

## `Bits.rotate-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e726f746174652d7269676874"></a>

## `Bits.rotate-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e746573742d626974"></a>

## `Bits.test-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">test-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-426974732e7365742d626974"></a>

## `Bits.set-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">set-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636c6561722d626974"></a>

## `Bits.clear-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">clear-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636f6d706c656d656e742d626974"></a>

## `Bits.complement-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">complement-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636f756e742d6f6e6573"></a>

## `Bits.count-ones`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



<a id="method-426974732e6c656164696e672d7a65726f73"></a>

## `Bits.leading-zeros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



<a id="method-426974732e747261696c696e672d7a65726f73"></a>

## `Bits.trailing-zeros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



<a id="class-44697265637453657175656e63696e67"></a>

## `DirectSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">DirectSequencing</span> </code></pre>

Sequencing providers for `do` notation (runtime capabilities,
effects and sequencing specification, "Unqualified do through
modules and typeclasses"; design section 5). The same `do` spelling
types differently per provider: direct sequencing is the default and
a `using` binder with a provider head selects another. `pure` and
`bind` are the provider's class methods (decision D2).

[Source](/reference-sources/Effect.Sequencing.cinch) · line 29

<a id="method-44697265637453657175656e63696e672e70757265"></a>

## `DirectSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">DirectSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-44697265637453657175656e63696e672e62696e64"></a>

## `DirectSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">DirectSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-variable">b</span>))</code></pre>



<a id="class-446976"></a>

## `Div`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Div</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Arithmetic.cinch)

<a id="method-4469762e2f"></a>

## `Div./`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">/</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Div</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">NonZero</span> <span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="class-456e756d"></a>

## `Enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Data.Enum.cinch)

<a id="method-456e756d2e73756363"></a>

## `Enum.succ`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">succ</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-456e756d2e70726564"></a>

## `Enum.pred`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pred</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>



<a id="method-456e756d2e746f2d656e756d"></a>

## `Enum.to-enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-enum</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>



<a id="method-456e756d2e66726f6d2d656e756d"></a>

## `Enum.from-enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-enum</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>



<a id="class-4571"></a>

## `Eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Eq</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Relation.Eq.cinch)

<a id="method-45712e3d3d"></a>

## `Eq.==`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">==</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-45712e2f3d"></a>

## `Eq./=`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">/=</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



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



<a id="class-46726f6d4d6170"></a>

## `FromMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">FromMap</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></pre>

[Source](/reference-sources/Data.FromMap.cinch)

<a id="method-46726f6d4d61702e66726f6d2d6d6170"></a>

## `FromMap.from-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-map</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">FromMap</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) =&gt; (<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) -&gt; <span class="tok-variable">m</span>)</code></pre>



<a id="class-4d756c"></a>

## `Mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Mul</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Arithmetic.cinch)

<a id="method-4d756c2e2a"></a>

## `Mul.*`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">*</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Mul</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="class-4e6567"></a>

## `Neg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Neg</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Arithmetic.cinch)

<a id="method-4e65672e6e6567617465"></a>

## `Neg.negate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">negate</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Neg</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="class-4f7074696f6e53657175656e63696e67"></a>

## `OptionSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">OptionSequencing</span> </code></pre>

[Source](/reference-sources/Effect.Sequencing.cinch)

<a id="method-4f7074696f6e53657175656e63696e672e70757265"></a>

## `OptionSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">OptionSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>



<a id="method-4f7074696f6e53657175656e63696e672e62696e64"></a>

## `OptionSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">OptionSequencing</span>) =&gt; (<span class="tok-constructor">Option</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Option</span> <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Option</span> <span class="tok-variable">b</span>))</code></pre>



<a id="class-4f7264"></a>

## `Ord`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Ord</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Relation.Ord.cinch)

<a id="method-4f72642e3c"></a>

## `Ord.<`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&lt;</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-4f72642e3c3d"></a>

## `Ord.<=`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&lt;=</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-4f72642e3e"></a>

## `Ord.>`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&gt;</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-4f72642e3e3d"></a>

## `Ord.>=`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&gt;=</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="class-52656d"></a>

## `Rem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Rem</span> <span class="tok-variable">a</span></code></pre>

Modulo and division for the mathematical types (decision D91).
The divisor carries its evidence: a `NonZero a`, so no instance
can reach a zero divisor. A numeral divisor needs nothing written
(`n / 10` and `n % 10` decide `NonZero 10` at compile time); any
other divisor is a threaded `NonZero a` or the product of a
runtime decision (`decide (NonZero d)`). A total division whose
divisor may be zero is not this operation: `Data.Complex.div`'s
IEEE quotient stays a named function. Machine integers keep the
checked family only, per the primitive types specification.

[Source](/reference-sources/Algebra.Arithmetic.cinch) · line 65

<a id="method-52656d2e25"></a>

## `Rem.%`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">%</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Rem</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">NonZero</span> <span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="class-526573756c7453657175656e63696e67"></a>

## `ResultSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span></code></pre>

[Source](/reference-sources/Effect.Sequencing.cinch)

<a id="method-526573756c7453657175656e63696e672e70757265"></a>

## `ResultSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code></pre>



<a id="method-526573756c7453657175656e63696e672e62696e64"></a>

## `ResultSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span>) =&gt; (<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">b</span>))</code></pre>



<a id="class-537562"></a>

## `Sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Sub</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Algebra.Arithmetic.cinch)

<a id="method-5375622e2d"></a>

## `Sub.-`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">-</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Sub</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="class-53756d50726f6f6673"></a>

## `SumProofs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">SumProofs</span> </code></pre>

The instruction law (platform section 5): the scalar model and the
SIMD intrinsic compute the same sum. Discharging it is a proof
obligation of the target library; a rewrite citing it records the
assumption in the artifact manifest (decision D55).

[Source](/reference-sources/Data.Target.cinch) · line 89

<a id="method-53756d50726f6f66732e7363616c61722d746f2d73696d64"></a>

## `SumProofs.scalar-to-simd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scalar-to-simd</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> <span class="tok-variable">n</span> <span class="tok-variable">region</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">SumProofs</span>) =&gt; ((<span class="tok-variable">vectors</span> :: <span class="tok-constructor">VecU32Target</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) -&gt; ((<span class="tok-variable">xs</span> :: <span class="tok-constructor">Ref</span> <span class="tok-variable">region</span> <span class="tok-constructor">Vec</span> <span class="tok-variable">n</span> <span class="tok-constructor">U32</span>) -&gt; <span class="tok-constructor">Eq</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-variable">scalar-sum-wrap32</span> <span class="tok-variable">xs</span>) (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Target</span>.<span class="tok-variable">simd-sum-wrap32</span> <span class="tok-variable">vectors</span> <span class="tok-variable">xs</span>)))</code></pre>



<a id="class-566563553332546172676574"></a>

## `VecU32Target`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">VecU32Target</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></pre>

The u32-sum capability: its construction establishes that the sum
intrinsics may execute in the current environment (platform proofs
and SIMD specification section 2). The method is the width evidence
a generic proof consumes (section 6).

[Source](/reference-sources/Data.Target.cinch) · line 63

<a id="method-5665635533325461726765742e706f7369746976652d7769647468"></a>

## `VecU32Target.positive-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">positive-width</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">VecU32Target</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>



# Values

This facade re-exports the following checked values. Follow a defining module for declaration documentation. Compiler-installed operations such as `ioWrite` have a [separate reference](../reference/20-compiler-vocabulary.md).

| Value | Defining module |
| --- | --- |
| `&` | [Algebra.Bits](Algebra.Bits.md#value-26) |
| `&&` | [Data.Bool](Data.Bool.md#value-2626) |
| `<<` | [Algebra.Bits](Algebra.Bits.md#value-3c3c) |
| `>>` | [Algebra.Bits](Algebra.Bits.md#value-3e3e) |
| `^` | [Algebra.Bits](Algebra.Bits.md#value-5e) |
| `abs-f32` | [Data.Machine](Data.Machine.md#value-6162732d663332) |
| `abs-f64` | [Data.Machine](Data.Machine.md#value-6162732d663634) |
| `add-f32` | [Data.Machine](Data.Machine.md#value-6164642d663332) |
| `add-f64` | [Data.Machine](Data.Machine.md#value-6164642d663634) |
| `and` | [Data.Bool](Data.Bool.md#value-616e64) |
| `apply-n-times` | [Function.Combinators](Function.Combinators.md#value-6170706c792d6e2d74696d6573) |
| `apply-to` | [Function.Combinators](Function.Combinators.md#value-6170706c792d746f) |
| `bit-and-i16` | [Data.Machine](Data.Machine.md#value-6269742d616e642d693136) |
| `bit-and-i32` | [Data.Machine](Data.Machine.md#value-6269742d616e642d693332) |
| `bit-and-i64` | [Data.Machine](Data.Machine.md#value-6269742d616e642d693634) |
| `bit-and-i8` | [Data.Machine](Data.Machine.md#value-6269742d616e642d6938) |
| `bit-and-isize` | [Data.Machine](Data.Machine.md#value-6269742d616e642d6973697a65) |
| `bit-and-u16` | [Data.Machine](Data.Machine.md#value-6269742d616e642d753136) |
| `bit-and-u32` | [Data.Machine](Data.Machine.md#value-6269742d616e642d753332) |
| `bit-and-u64` | [Data.Machine](Data.Machine.md#value-6269742d616e642d753634) |
| `bit-and-u8` | [Data.Machine](Data.Machine.md#value-6269742d616e642d7538) |
| `bit-and-usize` | [Data.Machine](Data.Machine.md#value-6269742d616e642d7573697a65) |
| `bit-not-i16` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d693136) |
| `bit-not-i32` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d693332) |
| `bit-not-i64` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d693634) |
| `bit-not-i8` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d6938) |
| `bit-not-isize` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d6973697a65) |
| `bit-not-u16` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d753136) |
| `bit-not-u32` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d753332) |
| `bit-not-u64` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d753634) |
| `bit-not-u8` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d7538) |
| `bit-not-usize` | [Data.Machine](Data.Machine.md#value-6269742d6e6f742d7573697a65) |
| `bit-or-i16` | [Data.Machine](Data.Machine.md#value-6269742d6f722d693136) |
| `bit-or-i32` | [Data.Machine](Data.Machine.md#value-6269742d6f722d693332) |
| `bit-or-i64` | [Data.Machine](Data.Machine.md#value-6269742d6f722d693634) |
| `bit-or-i8` | [Data.Machine](Data.Machine.md#value-6269742d6f722d6938) |
| `bit-or-isize` | [Data.Machine](Data.Machine.md#value-6269742d6f722d6973697a65) |
| `bit-or-u16` | [Data.Machine](Data.Machine.md#value-6269742d6f722d753136) |
| `bit-or-u32` | [Data.Machine](Data.Machine.md#value-6269742d6f722d753332) |
| `bit-or-u64` | [Data.Machine](Data.Machine.md#value-6269742d6f722d753634) |
| `bit-or-u8` | [Data.Machine](Data.Machine.md#value-6269742d6f722d7538) |
| `bit-or-usize` | [Data.Machine](Data.Machine.md#value-6269742d6f722d7573697a65) |
| `bit-xor-i16` | [Data.Machine](Data.Machine.md#value-6269742d786f722d693136) |
| `bit-xor-i32` | [Data.Machine](Data.Machine.md#value-6269742d786f722d693332) |
| `bit-xor-i64` | [Data.Machine](Data.Machine.md#value-6269742d786f722d693634) |
| `bit-xor-i8` | [Data.Machine](Data.Machine.md#value-6269742d786f722d6938) |
| `bit-xor-isize` | [Data.Machine](Data.Machine.md#value-6269742d786f722d6973697a65) |
| `bit-xor-u16` | [Data.Machine](Data.Machine.md#value-6269742d786f722d753136) |
| `bit-xor-u32` | [Data.Machine](Data.Machine.md#value-6269742d786f722d753332) |
| `bit-xor-u64` | [Data.Machine](Data.Machine.md#value-6269742d786f722d753634) |
| `bit-xor-u8` | [Data.Machine](Data.Machine.md#value-6269742d786f722d7538) |
| `bit-xor-usize` | [Data.Machine](Data.Machine.md#value-6269742d786f722d7573697a65) |
| `bool` | [Data.Bool](Data.Bool.md#value-626f6f6c) |
| `both` | [Function.Combinators](Function.Combinators.md#value-626f7468) |
| `break` | [Data.List](Data.List.md#value-627265616b) |
| `bytes-compare` | [Data.Bytes](Data.Bytes.md#value-62797465732d636f6d70617265) |
| `bytes-copy-range` | [Data.Bytes](Data.Bytes.md#value-62797465732d636f70792d72616e6765) |
| `bytes-from-list` | [Data.Bytes](Data.Bytes.md#value-62797465732d66726f6d2d6c697374) |
| `bytes-from-u8s` | [Data.Bytes](Data.Bytes.md#value-62797465732d66726f6d2d753873) |
| `bytes-get` | [Data.Bytes](Data.Bytes.md#value-62797465732d676574) |
| `bytes-len` | [Data.Bytes](Data.Bytes.md#value-62797465732d6c656e) |
| `bytes-new` | [Data.Bytes](Data.Bytes.md#value-62797465732d6e6577) |
| `bytes-set` | [Data.Bytes](Data.Bytes.md#value-62797465732d736574) |
| `cat-options` | [Data.Option](Data.Option.md#value-6361742d6f7074696f6e73) |
| `char-code` | [Data.Char](Data.Char.md#value-636861722d636f6465) |
| `char-from-code` | [Data.Char](Data.Char.md#value-636861722d66726f6d2d636f6465) |
| `char-string` | [Text.Show](Text.Show.md#value-636861722d737472696e67) |
| `char-to-u32` | [Data.Char](Data.Char.md#value-636861722d746f2d753332) |
| `checked-add-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d693136) |
| `checked-add-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d693332) |
| `checked-add-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d693634) |
| `checked-add-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d6938) |
| `checked-add-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d6973697a65) |
| `checked-add-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d753136) |
| `checked-add-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d753332) |
| `checked-add-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d753634) |
| `checked-add-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d7538) |
| `checked-add-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6164642d7573697a65) |
| `checked-div` | [Data.Nat](Data.Nat.md#value-636865636b65642d646976) |
| `checked-div-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d693136) |
| `checked-div-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d693332) |
| `checked-div-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d693634) |
| `checked-div-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d6938) |
| `checked-div-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d6973697a65) |
| `checked-div-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d753136) |
| `checked-div-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d753332) |
| `checked-div-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d753634) |
| `checked-div-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d7538) |
| `checked-div-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6469762d7573697a65) |
| `checked-f32-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d693136) |
| `checked-f32-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d693332) |
| `checked-f32-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d693634) |
| `checked-f32-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d6938) |
| `checked-f32-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d6973697a65) |
| `checked-f32-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d753136) |
| `checked-f32-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d753332) |
| `checked-f32-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d753634) |
| `checked-f32-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d7538) |
| `checked-f32-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6633322d746f2d7573697a65) |
| `checked-f64-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d693136) |
| `checked-f64-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d693332) |
| `checked-f64-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d693634) |
| `checked-f64-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d6938) |
| `checked-f64-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d6973697a65) |
| `checked-f64-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d753136) |
| `checked-f64-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d753332) |
| `checked-f64-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d753634) |
| `checked-f64-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d7538) |
| `checked-f64-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6636342d746f2d7573697a65) |
| `checked-i16-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d693136) |
| `checked-i16-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d693332) |
| `checked-i16-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d693634) |
| `checked-i16-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d6938) |
| `checked-i16-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d6973697a65) |
| `checked-i16-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d753136) |
| `checked-i16-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d753332) |
| `checked-i16-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d753634) |
| `checked-i16-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d7538) |
| `checked-i16-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6931362d746f2d7573697a65) |
| `checked-i32-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d693136) |
| `checked-i32-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d693332) |
| `checked-i32-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d693634) |
| `checked-i32-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d6938) |
| `checked-i32-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d6973697a65) |
| `checked-i32-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d753136) |
| `checked-i32-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d753332) |
| `checked-i32-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d753634) |
| `checked-i32-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d7538) |
| `checked-i32-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6933322d746f2d7573697a65) |
| `checked-i64-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d693136) |
| `checked-i64-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d693332) |
| `checked-i64-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d693634) |
| `checked-i64-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d6938) |
| `checked-i64-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d6973697a65) |
| `checked-i64-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d753136) |
| `checked-i64-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d753332) |
| `checked-i64-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d753634) |
| `checked-i64-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d7538) |
| `checked-i64-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6936342d746f2d7573697a65) |
| `checked-i8-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d693136) |
| `checked-i8-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d693332) |
| `checked-i8-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d693634) |
| `checked-i8-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d6938) |
| `checked-i8-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d6973697a65) |
| `checked-i8-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d753136) |
| `checked-i8-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d753332) |
| `checked-i8-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d753634) |
| `checked-i8-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d7538) |
| `checked-i8-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d69382d746f2d7573697a65) |
| `checked-int-to-i64` | [Data.Int](Data.Int.md#value-636865636b65642d696e742d746f2d693634) |
| `checked-isize-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d693136) |
| `checked-isize-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d693332) |
| `checked-isize-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d693634) |
| `checked-isize-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d6938) |
| `checked-isize-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d6973697a65) |
| `checked-isize-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d753136) |
| `checked-isize-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d753332) |
| `checked-isize-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d753634) |
| `checked-isize-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d7538) |
| `checked-isize-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6973697a652d746f2d7573697a65) |
| `checked-mul-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d693136) |
| `checked-mul-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d693332) |
| `checked-mul-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d693634) |
| `checked-mul-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d6938) |
| `checked-mul-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d6973697a65) |
| `checked-mul-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d753136) |
| `checked-mul-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d753332) |
| `checked-mul-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d753634) |
| `checked-mul-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d7538) |
| `checked-mul-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6d756c2d7573697a65) |
| `checked-nat-from-i64` | [Data.Nat](Data.Nat.md#value-636865636b65642d6e61742d66726f6d2d693634) |
| `checked-nat-to-i64` | [Data.Nat](Data.Nat.md#value-636865636b65642d6e61742d746f2d693634) |
| `checked-neg-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d693136) |
| `checked-neg-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d693332) |
| `checked-neg-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d693634) |
| `checked-neg-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d6938) |
| `checked-neg-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d6973697a65) |
| `checked-neg-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d753136) |
| `checked-neg-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d753332) |
| `checked-neg-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d753634) |
| `checked-neg-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d7538) |
| `checked-neg-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d6e65672d7573697a65) |
| `checked-rem` | [Data.Nat](Data.Nat.md#value-636865636b65642d72656d) |
| `checked-rem-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d693136) |
| `checked-rem-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d693332) |
| `checked-rem-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d693634) |
| `checked-rem-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d6938) |
| `checked-rem-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d6973697a65) |
| `checked-rem-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d753136) |
| `checked-rem-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d753332) |
| `checked-rem-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d753634) |
| `checked-rem-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d7538) |
| `checked-rem-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d72656d2d7573697a65) |
| `checked-shl-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d693136) |
| `checked-shl-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d693332) |
| `checked-shl-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d693634) |
| `checked-shl-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d6938) |
| `checked-shl-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d6973697a65) |
| `checked-shl-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d753136) |
| `checked-shl-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d753332) |
| `checked-shl-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d753634) |
| `checked-shl-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d7538) |
| `checked-shl-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d73686c2d7573697a65) |
| `checked-shr-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d693136) |
| `checked-shr-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d693332) |
| `checked-shr-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d693634) |
| `checked-shr-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d6938) |
| `checked-shr-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d6973697a65) |
| `checked-shr-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d753136) |
| `checked-shr-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d753332) |
| `checked-shr-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d753634) |
| `checked-shr-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d7538) |
| `checked-shr-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7368722d7573697a65) |
| `checked-sub` | [Data.Nat](Data.Nat.md#value-636865636b65642d737562) |
| `checked-sub-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d693136) |
| `checked-sub-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d693332) |
| `checked-sub-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d693634) |
| `checked-sub-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d6938) |
| `checked-sub-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d6973697a65) |
| `checked-sub-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d753136) |
| `checked-sub-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d753332) |
| `checked-sub-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d753634) |
| `checked-sub-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d7538) |
| `checked-sub-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7375622d7573697a65) |
| `checked-u16-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d693136) |
| `checked-u16-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d693332) |
| `checked-u16-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d693634) |
| `checked-u16-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d6938) |
| `checked-u16-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d6973697a65) |
| `checked-u16-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d753136) |
| `checked-u16-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d753332) |
| `checked-u16-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d753634) |
| `checked-u16-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d7538) |
| `checked-u16-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7531362d746f2d7573697a65) |
| `checked-u32-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d693136) |
| `checked-u32-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d693332) |
| `checked-u32-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d693634) |
| `checked-u32-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d6938) |
| `checked-u32-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d6973697a65) |
| `checked-u32-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d753136) |
| `checked-u32-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d753332) |
| `checked-u32-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d753634) |
| `checked-u32-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d7538) |
| `checked-u32-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7533322d746f2d7573697a65) |
| `checked-u64-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d693136) |
| `checked-u64-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d693332) |
| `checked-u64-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d693634) |
| `checked-u64-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d6938) |
| `checked-u64-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d6973697a65) |
| `checked-u64-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d753136) |
| `checked-u64-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d753332) |
| `checked-u64-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d753634) |
| `checked-u64-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d7538) |
| `checked-u64-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7536342d746f2d7573697a65) |
| `checked-u8-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d693136) |
| `checked-u8-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d693332) |
| `checked-u8-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d693634) |
| `checked-u8-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d6938) |
| `checked-u8-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d6973697a65) |
| `checked-u8-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d753136) |
| `checked-u8-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d753332) |
| `checked-u8-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d753634) |
| `checked-u8-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d7538) |
| `checked-u8-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d75382d746f2d7573697a65) |
| `checked-usize-to-i16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d693136) |
| `checked-usize-to-i32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d693332) |
| `checked-usize-to-i64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d693634) |
| `checked-usize-to-i8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d6938) |
| `checked-usize-to-isize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d6973697a65) |
| `checked-usize-to-u16` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d753136) |
| `checked-usize-to-u32` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d753332) |
| `checked-usize-to-u64` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d753634) |
| `checked-usize-to-u8` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d7538) |
| `checked-usize-to-usize` | [Data.Machine](Data.Machine.md#value-636865636b65642d7573697a652d746f2d7573697a65) |
| `compare-bytes-from` | [Data.String](Data.String.md#value-636f6d706172652d62797465732d66726f6d) |
| `comparing` | [Relation.Ord](Relation.Ord.md#value-636f6d706172696e67) |
| `compose` | [Function.Combinators](Function.Combinators.md#value-636f6d706f7365) |
| `const` | [Function.Combinators](Function.Combinators.md#value-636f6e7374) |
| `copy-bytes-into` | [Data.String](Data.String.md#value-636f70792d62797465732d696e746f) |
| `count-ones-i16` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d693136) |
| `count-ones-i32` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d693332) |
| `count-ones-i64` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d693634) |
| `count-ones-i8` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d6938) |
| `count-ones-isize` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d6973697a65) |
| `count-ones-u16` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d753136) |
| `count-ones-u32` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d753332) |
| `count-ones-u64` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d753634) |
| `count-ones-u8` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d7538) |
| `count-ones-usize` | [Data.Machine](Data.Machine.md#value-636f756e742d6f6e65732d7573697a65) |
| `curry` | [Function.Combinators](Function.Combinators.md#value-6375727279) |
| `cycle` | [Data.List](Data.List.md#value-6379636c65) |
| `detect-vec-u32` | [Data.Target](Data.Target.md#value-6465746563742d7665632d753332) |
| `digit-value` | [Data.Char](Data.Char.md#value-64696769742d76616c7565) |
| `div-f32` | [Data.Machine](Data.Machine.md#value-6469762d663332) |
| `div-f64` | [Data.Machine](Data.Machine.md#value-6469762d663634) |
| `drop` | [Data.List](Data.List.md#value-64726f70) |
| `either` | [Data.Result](Data.Result.md#value-656974686572) |
| `elem-index` | [Data.List](Data.List.md#value-656c656d2d696e646578) |
| `empty-string` | [Data.String](Data.String.md#value-656d7074792d737472696e67) |
| `enum-from-then-to` | [Data.Enum](Data.Enum.md#value-656e756d2d66726f6d2d7468656e2d746f) |
| `enum-from-to` | [Data.Enum](Data.Enum.md#value-656e756d2d66726f6d2d746f) |
| `eq-char` | [Data.Char](Data.Char.md#value-65712d63686172) |
| `eq-i16` | [Data.Machine](Data.Machine.md#value-65712d693136) |
| `eq-i32` | [Data.Machine](Data.Machine.md#value-65712d693332) |
| `eq-i64` | [Data.Machine](Data.Machine.md#value-65712d693634) |
| `eq-i8` | [Data.Machine](Data.Machine.md#value-65712d6938) |
| `eq-isize` | [Data.Machine](Data.Machine.md#value-65712d6973697a65) |
| `eq-u16` | [Data.Machine](Data.Machine.md#value-65712d753136) |
| `eq-u32` | [Data.Machine](Data.Machine.md#value-65712d753332) |
| `eq-u64` | [Data.Machine](Data.Machine.md#value-65712d753634) |
| `eq-u8` | [Data.Machine](Data.Machine.md#value-65712d7538) |
| `eq-usize` | [Data.Machine](Data.Machine.md#value-65712d7573697a65) |
| `errors` | [Data.Result](Data.Result.md#value-6572726f7273) |
| `f32-to-f32` | [Data.Machine](Data.Machine.md#value-6633322d746f2d663332) |
| `f32-to-f64` | [Data.Machine](Data.Machine.md#value-6633322d746f2d663634) |
| `f64-from-decimal` | [Data.Machine](Data.Machine.md#value-6636342d66726f6d2d646563696d616c) |
| `f64-to-decimal` | [Data.Machine](Data.Machine.md#value-6636342d746f2d646563696d616c) |
| `f64-to-f32` | [Data.Machine](Data.Machine.md#value-6636342d746f2d663332) |
| `f64-to-f64` | [Data.Machine](Data.Machine.md#value-6636342d746f2d663634) |
| `feq-f32` | [Data.Machine](Data.Machine.md#value-6665712d663332) |
| `feq-f64` | [Data.Machine](Data.Machine.md#value-6665712d663634) |
| `fetch` | [Effect.Concurrent](Effect.Concurrent.md#value-6665746368) |
| `fge-f32` | [Data.Machine](Data.Machine.md#value-6667652d663332) |
| `fge-f64` | [Data.Machine](Data.Machine.md#value-6667652d663634) |
| `fgt-f32` | [Data.Machine](Data.Machine.md#value-6667742d663332) |
| `fgt-f64` | [Data.Machine](Data.Machine.md#value-6667742d663634) |
| `find` | [Data.List](Data.List.md#value-66696e64) |
| `find-index` | [Data.List](Data.List.md#value-66696e642d696e646578) |
| `fle-f32` | [Data.Machine](Data.Machine.md#value-666c652d663332) |
| `fle-f64` | [Data.Machine](Data.Machine.md#value-666c652d663634) |
| `flip` | [Function.Combinators](Function.Combinators.md#value-666c6970) |
| `flt-f32` | [Data.Machine](Data.Machine.md#value-666c742d663332) |
| `flt-f64` | [Data.Machine](Data.Machine.md#value-666c742d663634) |
| `fne-f32` | [Data.Machine](Data.Machine.md#value-666e652d663332) |
| `fne-f64` | [Data.Machine](Data.Machine.md#value-666e652d663634) |
| `fold-left1` | [Data.List](Data.List.md#value-666f6c642d6c65667431) |
| `fold-right1` | [Data.List](Data.List.md#value-666f6c642d726967687431) |
| `foldr` | [Data.List](Data.List.md#value-666f6c6472) |
| `from-option` | [Data.Option](Data.Option.md#value-66726f6d2d6f7074696f6e) |
| `from-result` | [Data.Result](Data.Result.md#value-66726f6d2d726573756c74) |
| `fst` | [Function.Combinators](Function.Combinators.md#value-667374) |
| `ge-char` | [Data.Char](Data.Char.md#value-67652d63686172) |
| `ge-i16` | [Data.Machine](Data.Machine.md#value-67652d693136) |
| `ge-i32` | [Data.Machine](Data.Machine.md#value-67652d693332) |
| `ge-i64` | [Data.Machine](Data.Machine.md#value-67652d693634) |
| `ge-i8` | [Data.Machine](Data.Machine.md#value-67652d6938) |
| `ge-isize` | [Data.Machine](Data.Machine.md#value-67652d6973697a65) |
| `ge-u16` | [Data.Machine](Data.Machine.md#value-67652d753136) |
| `ge-u32` | [Data.Machine](Data.Machine.md#value-67652d753332) |
| `ge-u64` | [Data.Machine](Data.Machine.md#value-67652d753634) |
| `ge-u8` | [Data.Machine](Data.Machine.md#value-67652d7538) |
| `ge-usize` | [Data.Machine](Data.Machine.md#value-67652d7573697a65) |
| `group-by` | [Data.List](Data.List.md#value-67726f75702d6279) |
| `gt-char` | [Data.Char](Data.Char.md#value-67742d63686172) |
| `gt-i16` | [Data.Machine](Data.Machine.md#value-67742d693136) |
| `gt-i32` | [Data.Machine](Data.Machine.md#value-67742d693332) |
| `gt-i64` | [Data.Machine](Data.Machine.md#value-67742d693634) |
| `gt-i8` | [Data.Machine](Data.Machine.md#value-67742d6938) |
| `gt-isize` | [Data.Machine](Data.Machine.md#value-67742d6973697a65) |
| `gt-u16` | [Data.Machine](Data.Machine.md#value-67742d753136) |
| `gt-u32` | [Data.Machine](Data.Machine.md#value-67742d753332) |
| `gt-u64` | [Data.Machine](Data.Machine.md#value-67742d753634) |
| `gt-u8` | [Data.Machine](Data.Machine.md#value-67742d7538) |
| `gt-usize` | [Data.Machine](Data.Machine.md#value-67742d7573697a65) |
| `head` | [Data.List](Data.List.md#value-68656164) |
| `hex-digit-value` | [Data.Char](Data.Char.md#value-6865782d64696769742d76616c7565) |
| `i16-to-f32` | [Data.Machine](Data.Machine.md#value-6931362d746f2d663332) |
| `i16-to-f64` | [Data.Machine](Data.Machine.md#value-6931362d746f2d663634) |
| `i16-to-i16` | [Data.Machine](Data.Machine.md#value-6931362d746f2d693136) |
| `i16-to-i32` | [Data.Machine](Data.Machine.md#value-6931362d746f2d693332) |
| `i16-to-i64` | [Data.Machine](Data.Machine.md#value-6931362d746f2d693634) |
| `i16-to-isize` | [Data.Machine](Data.Machine.md#value-6931362d746f2d6973697a65) |
| `i32-to-f32` | [Data.Machine](Data.Machine.md#value-6933322d746f2d663332) |
| `i32-to-f64` | [Data.Machine](Data.Machine.md#value-6933322d746f2d663634) |
| `i32-to-i32` | [Data.Machine](Data.Machine.md#value-6933322d746f2d693332) |
| `i32-to-i64` | [Data.Machine](Data.Machine.md#value-6933322d746f2d693634) |
| `i32-to-isize` | [Data.Machine](Data.Machine.md#value-6933322d746f2d6973697a65) |
| `i64-le-byte` | [Data.Machine](Data.Machine.md#value-6936342d6c652d62797465) |
| `i64-to-f32` | [Data.Machine](Data.Machine.md#value-6936342d746f2d663332) |
| `i64-to-f64` | [Data.Machine](Data.Machine.md#value-6936342d746f2d663634) |
| `i64-to-i64` | [Data.Machine](Data.Machine.md#value-6936342d746f2d693634) |
| `i64-to-isize` | [Data.Machine](Data.Machine.md#value-6936342d746f2d6973697a65) |
| `i8-to-f32` | [Data.Machine](Data.Machine.md#value-69382d746f2d663332) |
| `i8-to-f64` | [Data.Machine](Data.Machine.md#value-69382d746f2d663634) |
| `i8-to-i16` | [Data.Machine](Data.Machine.md#value-69382d746f2d693136) |
| `i8-to-i32` | [Data.Machine](Data.Machine.md#value-69382d746f2d693332) |
| `i8-to-i64` | [Data.Machine](Data.Machine.md#value-69382d746f2d693634) |
| `i8-to-i8` | [Data.Machine](Data.Machine.md#value-69382d746f2d6938) |
| `i8-to-isize` | [Data.Machine](Data.Machine.md#value-69382d746f2d6973697a65) |
| `identity` | [Function.Combinators](Function.Combinators.md#value-6964656e74697479) |
| `init` | [Data.List](Data.List.md#value-696e6974) |
| `int-add` | [Data.Int](Data.Int.md#value-696e742d616464) |
| `int-cmp` | [Data.Int](Data.Int.md#value-696e742d636d70) |
| `int-div` | [Data.Int](Data.Int.md#value-696e742d646976) |
| `int-from-i64` | [Data.Int](Data.Int.md#value-696e742d66726f6d2d693634) |
| `int-mod-nat` | [Data.Int](Data.Int.md#value-696e742d6d6f642d6e6174) |
| `int-mul` | [Data.Int](Data.Int.md#value-696e742d6d756c) |
| `int-neg` | [Data.Int](Data.Int.md#value-696e742d6e6567) |
| `int-rem` | [Data.Int](Data.Int.md#value-696e742d72656d) |
| `int-sub` | [Data.Int](Data.Int.md#value-696e742d737562) |
| `int-to-i64` | [Data.Int](Data.Int.md#value-696e742d746f2d693634) |
| `int-to-string` | [Text.Show](Text.Show.md#value-696e742d746f2d737472696e67) |
| `intercalate` | [Data.List](Data.List.md#value-696e74657263616c617465) |
| `intern` | [Data.Sym](Data.Sym.md#value-696e7465726e) |
| `intersperse` | [Data.List](Data.List.md#value-696e746572737065727365) |
| `is-ascii-alpha` | [Data.Char](Data.Char.md#value-69732d61736369692d616c706861) |
| `is-ascii-alpha-numeric` | [Data.Char](Data.Char.md#value-69732d61736369692d616c7068612d6e756d65726963) |
| `is-ascii-digit` | [Data.Char](Data.Char.md#value-69732d61736369692d6469676974) |
| `is-ascii-hex-digit` | [Data.Char](Data.Char.md#value-69732d61736369692d6865782d6469676974) |
| `is-ascii-lower` | [Data.Char](Data.Char.md#value-69732d61736369692d6c6f776572) |
| `is-ascii-space` | [Data.Char](Data.Char.md#value-69732d61736369692d7370616365) |
| `is-ascii-upper` | [Data.Char](Data.Char.md#value-69732d61736369692d7570706572) |
| `is-empty` | [Data.List](Data.List.md#value-69732d656d707479) |
| `is-err` | [Data.Result](Data.Result.md#value-69732d657272) |
| `is-finite-f32` | [Data.Machine](Data.Machine.md#value-69732d66696e6974652d663332) |
| `is-finite-f64` | [Data.Machine](Data.Machine.md#value-69732d66696e6974652d663634) |
| `is-infinite-f32` | [Data.Machine](Data.Machine.md#value-69732d696e66696e6974652d663332) |
| `is-infinite-f64` | [Data.Machine](Data.Machine.md#value-69732d696e66696e6974652d663634) |
| `is-na-nf32` | [Data.Machine](Data.Machine.md#value-69732d6e612d6e663332) |
| `is-na-nf64` | [Data.Machine](Data.Machine.md#value-69732d6e612d6e663634) |
| `is-none` | [Data.Option](Data.Option.md#value-69732d6e6f6e65) |
| `is-normal-f32` | [Data.Machine](Data.Machine.md#value-69732d6e6f726d616c2d663332) |
| `is-normal-f64` | [Data.Machine](Data.Machine.md#value-69732d6e6f726d616c2d663634) |
| `is-ok` | [Data.Result](Data.Result.md#value-69732d6f6b) |
| `is-some` | [Data.Option](Data.Option.md#value-69732d736f6d65) |
| `isize-to-f32` | [Data.Machine](Data.Machine.md#value-6973697a652d746f2d663332) |
| `isize-to-f64` | [Data.Machine](Data.Machine.md#value-6973697a652d746f2d663634) |
| `isize-to-i64` | [Data.Machine](Data.Machine.md#value-6973697a652d746f2d693634) |
| `isize-to-isize` | [Data.Machine](Data.Machine.md#value-6973697a652d746f2d6973697a65) |
| `iterate` | [Data.List](Data.List.md#value-69746572617465) |
| `last` | [Data.List](Data.List.md#value-6c617374) |
| `le-char` | [Data.Char](Data.Char.md#value-6c652d63686172) |
| `le-i16` | [Data.Machine](Data.Machine.md#value-6c652d693136) |
| `le-i32` | [Data.Machine](Data.Machine.md#value-6c652d693332) |
| `le-i64` | [Data.Machine](Data.Machine.md#value-6c652d693634) |
| `le-i8` | [Data.Machine](Data.Machine.md#value-6c652d6938) |
| `le-isize` | [Data.Machine](Data.Machine.md#value-6c652d6973697a65) |
| `le-u16` | [Data.Machine](Data.Machine.md#value-6c652d753136) |
| `le-u32` | [Data.Machine](Data.Machine.md#value-6c652d753332) |
| `le-u64` | [Data.Machine](Data.Machine.md#value-6c652d753634) |
| `le-u8` | [Data.Machine](Data.Machine.md#value-6c652d7538) |
| `le-usize` | [Data.Machine](Data.Machine.md#value-6c652d7573697a65) |
| `leading-zeros-i16` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d693136) |
| `leading-zeros-i32` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d693332) |
| `leading-zeros-i64` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d693634) |
| `leading-zeros-i8` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d6938) |
| `leading-zeros-isize` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d6973697a65) |
| `leading-zeros-u16` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d753136) |
| `leading-zeros-u32` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d753332) |
| `leading-zeros-u64` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d753634) |
| `leading-zeros-u8` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d7538) |
| `leading-zeros-usize` | [Data.Machine](Data.Machine.md#value-6c656164696e672d7a65726f732d7573697a65) |
| `length` | [Data.List](Data.List.md#value-6c656e677468) |
| `list-length` | [Data.List](Data.List.md#value-6c6973742d6c656e677468) |
| `list-to-option` | [Data.Option](Data.Option.md#value-6c6973742d746f2d6f7074696f6e) |
| `lookup` | [Data.List](Data.List.md#value-6c6f6f6b7570) |
| `lt-char` | [Data.Char](Data.Char.md#value-6c742d63686172) |
| `lt-i16` | [Data.Machine](Data.Machine.md#value-6c742d693136) |
| `lt-i32` | [Data.Machine](Data.Machine.md#value-6c742d693332) |
| `lt-i64` | [Data.Machine](Data.Machine.md#value-6c742d693634) |
| `lt-i8` | [Data.Machine](Data.Machine.md#value-6c742d6938) |
| `lt-isize` | [Data.Machine](Data.Machine.md#value-6c742d6973697a65) |
| `lt-u16` | [Data.Machine](Data.Machine.md#value-6c742d753136) |
| `lt-u32` | [Data.Machine](Data.Machine.md#value-6c742d753332) |
| `lt-u64` | [Data.Machine](Data.Machine.md#value-6c742d753634) |
| `lt-u8` | [Data.Machine](Data.Machine.md#value-6c742d7538) |
| `lt-usize` | [Data.Machine](Data.Machine.md#value-6c742d7573697a65) |
| `map` | [Data.List](Data.List.md#value-6d6170) |
| `map-delete` | [Data.Map](Data.Map.md#value-6d61702d64656c657465) |
| `map-empty` | [Data.Map](Data.Map.md#value-6d61702d656d707479) |
| `map-err` | [Data.Result](Data.Result.md#value-6d61702d657272) |
| `map-filter` | [Data.Map](Data.Map.md#value-6d61702d66696c746572) |
| `map-fold` | [Data.Map](Data.Map.md#value-6d61702d666f6c64) |
| `map-from-pairs` | [Data.Map](Data.Map.md#value-6d61702d66726f6d2d7061697273) |
| `map-insert` | [Data.Map](Data.Map.md#value-6d61702d696e73657274) |
| `map-is-empty` | [Data.Map](Data.Map.md#value-6d61702d69732d656d707479) |
| `map-keys` | [Data.Map](Data.Map.md#value-6d61702d6b657973) |
| `map-lookup` | [Data.Map](Data.Map.md#value-6d61702d6c6f6f6b7570) |
| `map-map` | [Data.Map](Data.Map.md#value-6d61702d6d6170) |
| `map-option` | [Data.Option](Data.Option.md#value-6d61702d6f7074696f6e) |
| `map-result` | [Data.Result](Data.Result.md#value-6d61702d726573756c74) |
| `map-size` | [Data.Map](Data.Map.md#value-6d61702d73697a65) |
| `map-to-list` | [Data.Map](Data.Map.md#value-6d61702d746f2d6c697374) |
| `map-union` | [Data.Map](Data.Map.md#value-6d61702d756e696f6e) |
| `map-union-with` | [Data.Map](Data.Map.md#value-6d61702d756e696f6e2d77697468) |
| `map-values` | [Data.Map](Data.Map.md#value-6d61702d76616c756573) |
| `mask-i64` | [Data.Machine](Data.Machine.md#value-6d61736b2d693634) |
| `max-bound-i16` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d693136) |
| `max-bound-i32` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d693332) |
| `max-bound-i64` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d693634) |
| `max-bound-i8` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d6938) |
| `max-bound-isize` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d6973697a65) |
| `max-bound-u16` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d753136) |
| `max-bound-u32` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d753332) |
| `max-bound-u64` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d753634) |
| `max-bound-u8` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d7538) |
| `max-bound-usize` | [Data.Machine](Data.Machine.md#value-6d61782d626f756e642d7573697a65) |
| `maximum-f32` | [Data.Machine](Data.Machine.md#value-6d6178696d756d2d663332) |
| `maximum-f64` | [Data.Machine](Data.Machine.md#value-6d6178696d756d2d663634) |
| `maybe` | [Data.Option](Data.Option.md#value-6d61796265) |
| `min-bound-i16` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d693136) |
| `min-bound-i32` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d693332) |
| `min-bound-i64` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d693634) |
| `min-bound-i8` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d6938) |
| `min-bound-isize` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d6973697a65) |
| `min-bound-u16` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d753136) |
| `min-bound-u32` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d753332) |
| `min-bound-u64` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d753634) |
| `min-bound-u8` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d7538) |
| `min-bound-usize` | [Data.Machine](Data.Machine.md#value-6d696e2d626f756e642d7573697a65) |
| `minimum-f32` | [Data.Machine](Data.Machine.md#value-6d696e696d756d2d663332) |
| `minimum-f64` | [Data.Machine](Data.Machine.md#value-6d696e696d756d2d663634) |
| `mul-f32` | [Data.Machine](Data.Machine.md#value-6d756c2d663332) |
| `mul-f64` | [Data.Machine](Data.Machine.md#value-6d756c2d663634) |
| `nat-add` | [Data.Nat](Data.Nat.md#value-6e61742d616464) |
| `nat-cmp` | [Data.Nat](Data.Nat.md#value-6e61742d636d70) |
| `nat-digit-count` | [Text.Show](Text.Show.md#value-6e61742d64696769742d636f756e74) |
| `nat-digits-into` | [Text.Show](Text.Show.md#value-6e61742d6469676974732d696e746f) |
| `nat-div` | [Data.Nat](Data.Nat.md#value-6e61742d646976) |
| `nat-from-i64` | [Data.Nat](Data.Nat.md#value-6e61742d66726f6d2d693634) |
| `nat-from-int` | [Data.Nat](Data.Nat.md#value-6e61742d66726f6d2d696e74) |
| `nat-mul` | [Data.Nat](Data.Nat.md#value-6e61742d6d756c) |
| `nat-rem` | [Data.Nat](Data.Nat.md#value-6e61742d72656d) |
| `nat-sub` | [Data.Nat](Data.Nat.md#value-6e61742d737562) |
| `nat-to-i64` | [Data.Nat](Data.Nat.md#value-6e61742d746f2d693634) |
| `nat-to-int` | [Data.Nat](Data.Nat.md#value-6e61742d746f2d696e74) |
| `nat-to-string` | [Text.Show](Text.Show.md#value-6e61742d746f2d737472696e67) |
| `ne-char` | [Data.Char](Data.Char.md#value-6e652d63686172) |
| `ne-i16` | [Data.Machine](Data.Machine.md#value-6e652d693136) |
| `ne-i32` | [Data.Machine](Data.Machine.md#value-6e652d693332) |
| `ne-i64` | [Data.Machine](Data.Machine.md#value-6e652d693634) |
| `ne-i8` | [Data.Machine](Data.Machine.md#value-6e652d6938) |
| `ne-isize` | [Data.Machine](Data.Machine.md#value-6e652d6973697a65) |
| `ne-u16` | [Data.Machine](Data.Machine.md#value-6e652d753136) |
| `ne-u32` | [Data.Machine](Data.Machine.md#value-6e652d753332) |
| `ne-u64` | [Data.Machine](Data.Machine.md#value-6e652d753634) |
| `ne-u8` | [Data.Machine](Data.Machine.md#value-6e652d7538) |
| `ne-usize` | [Data.Machine](Data.Machine.md#value-6e652d7573697a65) |
| `neg-f32` | [Data.Machine](Data.Machine.md#value-6e65672d663332) |
| `neg-f64` | [Data.Machine](Data.Machine.md#value-6e65672d663634) |
| `not` | [Data.Bool](Data.Bool.md#value-6e6f74) |
| `nth` | [Data.List](Data.List.md#value-6e7468) |
| `nub` | [Data.List](Data.List.md#value-6e7562) |
| `on` | [Function.Combinators](Function.Combinators.md#value-6f6e) |
| `option-to-list` | [Data.Option](Data.Option.md#value-6f7074696f6e2d746f2d6c697374) |
| `option-to-result` | [Data.Result](Data.Result.md#value-6f7074696f6e2d746f2d726573756c74) |
| `or` | [Data.Bool](Data.Bool.md#value-6f72) |
| `or-else` | [Data.Option](Data.Option.md#value-6f722d656c7365) |
| `pack` | [Data.String](Data.String.md#value-7061636b) |
| `parse-config` | [Effect.Io](Effect.Io.md#value-70617273652d636f6e666967) |
| `partition` | [Data.List](Data.List.md#value-706172746974696f6e) |
| `partition-results` | [Data.Result](Data.Result.md#value-706172746974696f6e2d726573756c7473) |
| `pow10` | [Text.Show](Text.Show.md#value-706f773130) |
| `prim-and-bool` | [Data.Bool](Data.Bool.md#value-7072696d2d616e642d626f6f6c) |
| `prim-eq-bool` | [Data.Bool](Data.Bool.md#value-7072696d2d65712d626f6f6c) |
| `prim-ne-bool` | [Data.Bool](Data.Bool.md#value-7072696d2d6e652d626f6f6c) |
| `prim-not-bool` | [Data.Bool](Data.Bool.md#value-7072696d2d6e6f742d626f6f6c) |
| `prim-or-bool` | [Data.Bool](Data.Bool.md#value-7072696d2d6f722d626f6f6c) |
| `prod-fold` | [Data.Producer](Data.Producer.md#value-70726f642d666f6c64) |
| `prod-fold-go` | [Data.Producer](Data.Producer.md#value-70726f642d666f6c642d676f) |
| `prod-map` | [Data.Producer](Data.Producer.md#value-70726f642d6d6170) |
| `prod-take` | [Data.Producer](Data.Producer.md#value-70726f642d74616b65) |
| `prod-take-step` | [Data.Producer](Data.Producer.md#value-70726f642d74616b652d73746570) |
| `read-config-bytes` | [Effect.Io](Effect.Io.md#value-726561642d636f6e6669672d6279746573) |
| `replicate` | [Data.List](Data.List.md#value-7265706c6963617465) |
| `result-to-option` | [Data.Result](Data.Result.md#value-726573756c742d746f2d6f7074696f6e) |
| `results` | [Data.Result](Data.Result.md#value-726573756c7473) |
| `reverse` | [Data.List](Data.List.md#value-72657665727365) |
| `reverse-bits-i16` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d693136) |
| `reverse-bits-i32` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d693332) |
| `reverse-bits-i64` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d693634) |
| `reverse-bits-i8` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d6938) |
| `reverse-bits-isize` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d6973697a65) |
| `reverse-bits-u16` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d753136) |
| `reverse-bits-u32` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d753332) |
| `reverse-bits-u64` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d753634) |
| `reverse-bits-u8` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d7538) |
| `reverse-bits-usize` | [Data.Machine](Data.Machine.md#value-726576657273652d626974732d7573697a65) |
| `rotate-left-i16` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d693136) |
| `rotate-left-i32` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d693332) |
| `rotate-left-i64` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d693634) |
| `rotate-left-i8` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d6938) |
| `rotate-left-isize` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d6973697a65) |
| `rotate-left-u16` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d753136) |
| `rotate-left-u32` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d753332) |
| `rotate-left-u64` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d753634) |
| `rotate-left-u8` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d7538) |
| `rotate-left-usize` | [Data.Machine](Data.Machine.md#value-726f746174652d6c6566742d7573697a65) |
| `rotate-right-i16` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d693136) |
| `rotate-right-i32` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d693332) |
| `rotate-right-i64` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d693634) |
| `rotate-right-i8` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d6938) |
| `rotate-right-isize` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d6973697a65) |
| `rotate-right-u16` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d753136) |
| `rotate-right-u32` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d753332) |
| `rotate-right-u64` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d753634) |
| `rotate-right-u8` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d7538) |
| `rotate-right-usize` | [Data.Machine](Data.Machine.md#value-726f746174652d72696768742d7573697a65) |
| `saturating-add-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d693136) |
| `saturating-add-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d693332) |
| `saturating-add-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d693634) |
| `saturating-add-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d6938) |
| `saturating-add-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d6973697a65) |
| `saturating-add-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d753136) |
| `saturating-add-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d753332) |
| `saturating-add-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d753634) |
| `saturating-add-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d7538) |
| `saturating-add-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6164642d7573697a65) |
| `saturating-f32-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d693136) |
| `saturating-f32-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d693332) |
| `saturating-f32-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d693634) |
| `saturating-f32-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d6938) |
| `saturating-f32-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d6973697a65) |
| `saturating-f32-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d753136) |
| `saturating-f32-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d753332) |
| `saturating-f32-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d753634) |
| `saturating-f32-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d7538) |
| `saturating-f32-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6633322d746f2d7573697a65) |
| `saturating-f64-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d693136) |
| `saturating-f64-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d693332) |
| `saturating-f64-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d693634) |
| `saturating-f64-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d6938) |
| `saturating-f64-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d6973697a65) |
| `saturating-f64-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d753136) |
| `saturating-f64-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d753332) |
| `saturating-f64-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d753634) |
| `saturating-f64-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d7538) |
| `saturating-f64-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6636342d746f2d7573697a65) |
| `saturating-i16-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d693136) |
| `saturating-i16-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d693332) |
| `saturating-i16-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d693634) |
| `saturating-i16-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d6938) |
| `saturating-i16-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d6973697a65) |
| `saturating-i16-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d753136) |
| `saturating-i16-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d753332) |
| `saturating-i16-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d753634) |
| `saturating-i16-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d7538) |
| `saturating-i16-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6931362d746f2d7573697a65) |
| `saturating-i32-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d693136) |
| `saturating-i32-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d693332) |
| `saturating-i32-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d693634) |
| `saturating-i32-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d6938) |
| `saturating-i32-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d6973697a65) |
| `saturating-i32-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d753136) |
| `saturating-i32-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d753332) |
| `saturating-i32-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d753634) |
| `saturating-i32-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d7538) |
| `saturating-i32-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6933322d746f2d7573697a65) |
| `saturating-i64-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d693136) |
| `saturating-i64-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d693332) |
| `saturating-i64-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d693634) |
| `saturating-i64-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d6938) |
| `saturating-i64-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d6973697a65) |
| `saturating-i64-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d753136) |
| `saturating-i64-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d753332) |
| `saturating-i64-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d753634) |
| `saturating-i64-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d7538) |
| `saturating-i64-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6936342d746f2d7573697a65) |
| `saturating-i8-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d693136) |
| `saturating-i8-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d693332) |
| `saturating-i8-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d693634) |
| `saturating-i8-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d6938) |
| `saturating-i8-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d6973697a65) |
| `saturating-i8-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d753136) |
| `saturating-i8-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d753332) |
| `saturating-i8-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d753634) |
| `saturating-i8-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d7538) |
| `saturating-i8-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d69382d746f2d7573697a65) |
| `saturating-isize-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d693136) |
| `saturating-isize-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d693332) |
| `saturating-isize-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d693634) |
| `saturating-isize-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d6938) |
| `saturating-isize-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d6973697a65) |
| `saturating-isize-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d753136) |
| `saturating-isize-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d753332) |
| `saturating-isize-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d753634) |
| `saturating-isize-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d7538) |
| `saturating-isize-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6973697a652d746f2d7573697a65) |
| `saturating-mul-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d693136) |
| `saturating-mul-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d693332) |
| `saturating-mul-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d693634) |
| `saturating-mul-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d6938) |
| `saturating-mul-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d6973697a65) |
| `saturating-mul-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d753136) |
| `saturating-mul-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d753332) |
| `saturating-mul-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d753634) |
| `saturating-mul-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d7538) |
| `saturating-mul-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6d756c2d7573697a65) |
| `saturating-neg-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d693136) |
| `saturating-neg-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d693332) |
| `saturating-neg-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d693634) |
| `saturating-neg-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d6938) |
| `saturating-neg-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d6973697a65) |
| `saturating-neg-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d753136) |
| `saturating-neg-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d753332) |
| `saturating-neg-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d753634) |
| `saturating-neg-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d7538) |
| `saturating-neg-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d6e65672d7573697a65) |
| `saturating-sub` | [Data.Nat](Data.Nat.md#value-73617475726174696e672d737562) |
| `saturating-sub-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d693136) |
| `saturating-sub-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d693332) |
| `saturating-sub-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d693634) |
| `saturating-sub-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d6938) |
| `saturating-sub-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d6973697a65) |
| `saturating-sub-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d753136) |
| `saturating-sub-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d753332) |
| `saturating-sub-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d753634) |
| `saturating-sub-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d7538) |
| `saturating-sub-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7375622d7573697a65) |
| `saturating-u16-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d693136) |
| `saturating-u16-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d693332) |
| `saturating-u16-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d693634) |
| `saturating-u16-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d6938) |
| `saturating-u16-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d6973697a65) |
| `saturating-u16-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d753136) |
| `saturating-u16-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d753332) |
| `saturating-u16-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d753634) |
| `saturating-u16-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d7538) |
| `saturating-u16-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7531362d746f2d7573697a65) |
| `saturating-u32-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d693136) |
| `saturating-u32-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d693332) |
| `saturating-u32-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d693634) |
| `saturating-u32-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d6938) |
| `saturating-u32-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d6973697a65) |
| `saturating-u32-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d753136) |
| `saturating-u32-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d753332) |
| `saturating-u32-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d753634) |
| `saturating-u32-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d7538) |
| `saturating-u32-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7533322d746f2d7573697a65) |
| `saturating-u64-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d693136) |
| `saturating-u64-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d693332) |
| `saturating-u64-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d693634) |
| `saturating-u64-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d6938) |
| `saturating-u64-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d6973697a65) |
| `saturating-u64-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d753136) |
| `saturating-u64-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d753332) |
| `saturating-u64-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d753634) |
| `saturating-u64-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d7538) |
| `saturating-u64-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7536342d746f2d7573697a65) |
| `saturating-u8-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d693136) |
| `saturating-u8-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d693332) |
| `saturating-u8-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d693634) |
| `saturating-u8-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d6938) |
| `saturating-u8-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d6973697a65) |
| `saturating-u8-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d753136) |
| `saturating-u8-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d753332) |
| `saturating-u8-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d753634) |
| `saturating-u8-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d7538) |
| `saturating-u8-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d75382d746f2d7573697a65) |
| `saturating-usize-to-i16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d693136) |
| `saturating-usize-to-i32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d693332) |
| `saturating-usize-to-i64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d693634) |
| `saturating-usize-to-i8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d6938) |
| `saturating-usize-to-isize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d6973697a65) |
| `saturating-usize-to-u16` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d753136) |
| `saturating-usize-to-u32` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d753332) |
| `saturating-usize-to-u64` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d753634) |
| `saturating-usize-to-u8` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d7538) |
| `saturating-usize-to-usize` | [Data.Machine](Data.Machine.md#value-73617475726174696e672d7573697a652d746f2d7573697a65) |
| `scalar-sum-wrap32` | [Data.Producer](Data.Producer.md#value-7363616c61722d73756d2d777261703332) |
| `scanl` | [Data.List](Data.List.md#value-7363616e6c) |
| `scanr` | [Data.List](Data.List.md#value-7363616e72) |
| `sha256-bytes` | [Data.Bytes](Data.Bytes.md#value-7368613235362d6279746573) |
| `sign-bit-f32` | [Data.Machine](Data.Machine.md#value-7369676e2d6269742d663332) |
| `sign-bit-f64` | [Data.Machine](Data.Machine.md#value-7369676e2d6269742d663634) |
| `simd-sum-wrap32` | [Data.Target](Data.Target.md#value-73696d642d73756d2d777261703332) |
| `snd` | [Function.Combinators](Function.Combinators.md#value-736e64) |
| `sort` | [Data.List](Data.List.md#value-736f7274) |
| `sort-by` | [Data.List](Data.List.md#value-736f72742d6279) |
| `span` | [Data.List](Data.List.md#value-7370616e) |
| `split-at` | [Data.List](Data.List.md#value-73706c69742d6174) |
| `sqrt-f32` | [Data.Machine](Data.Machine.md#value-737172742d663332) |
| `sqrt-f64` | [Data.Machine](Data.Machine.md#value-737172742d663634) |
| `string-byte-index-from` | [Data.String](Data.String.md#value-737472696e672d627974652d696e6465782d66726f6d) |
| `string-byte-length` | [Data.String](Data.String.md#value-737472696e672d627974652d6c656e677468) |
| `string-char-at` | [Data.String](Data.String.md#value-737472696e672d636861722d6174) |
| `string-char-at-from` | [Data.String](Data.String.md#value-737472696e672d636861722d61742d66726f6d) |
| `string-compare` | [Data.String](Data.String.md#value-737472696e672d636f6d70617265) |
| `string-concat` | [Data.String](Data.String.md#value-737472696e672d636f6e636174) |
| `string-concat-all` | [Data.String](Data.String.md#value-737472696e672d636f6e6361742d616c6c) |
| `string-from-bytes` | [Data.String](Data.String.md#value-737472696e672d66726f6d2d6279746573) |
| `string-length` | [Data.String](Data.String.md#value-737472696e672d6c656e677468) |
| `string-length-from` | [Data.String](Data.String.md#value-737472696e672d6c656e6774682d66726f6d) |
| `string-repeat` | [Data.String](Data.String.md#value-737472696e672d726570656174) |
| `string-slice` | [Data.String](Data.String.md#value-737472696e672d736c696365) |
| `string-to-bytes` | [Data.String](Data.String.md#value-737472696e672d746f2d6279746573) |
| `sub-f32` | [Data.Machine](Data.Machine.md#value-7375622d663332) |
| `sub-f64` | [Data.Machine](Data.Machine.md#value-7375622d663634) |
| `subsequences` | [Data.List](Data.List.md#value-73756273657175656e636573) |
| `sum-list` | [Data.List](Data.List.md#value-73756d2d6c697374) |
| `swap` | [Function.Combinators](Function.Combinators.md#value-73776170) |
| `swap-bytes-i16` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d693136) |
| `swap-bytes-i32` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d693332) |
| `swap-bytes-i64` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d693634) |
| `swap-bytes-i8` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d6938) |
| `swap-bytes-isize` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d6973697a65) |
| `swap-bytes-u16` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d753136) |
| `swap-bytes-u32` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d753332) |
| `swap-bytes-u64` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d753634) |
| `swap-bytes-u8` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d7538) |
| `swap-bytes-usize` | [Data.Machine](Data.Machine.md#value-737761702d62797465732d7573697a65) |
| `sym-name` | [Data.Sym](Data.Sym.md#value-73796d2d6e616d65) |
| `tail` | [Data.List](Data.List.md#value-7461696c) |
| `take` | [Data.List](Data.List.md#value-74616b65) |
| `to-lower-ascii` | [Data.Char](Data.Char.md#value-746f2d6c6f7765722d6173636969) |
| `to-upper-ascii` | [Data.Char](Data.Char.md#value-746f2d75707065722d6173636969) |
| `total-order-f32` | [Data.Machine](Data.Machine.md#value-746f74616c2d6f726465722d663332) |
| `total-order-f64` | [Data.Machine](Data.Machine.md#value-746f74616c2d6f726465722d663634) |
| `trailing-zeros-i16` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d693136) |
| `trailing-zeros-i32` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d693332) |
| `trailing-zeros-i64` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d693634) |
| `trailing-zeros-i8` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d6938) |
| `trailing-zeros-isize` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d6973697a65) |
| `trailing-zeros-u16` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d753136) |
| `trailing-zeros-u32` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d753332) |
| `trailing-zeros-u64` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d753634) |
| `trailing-zeros-u8` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d7538) |
| `trailing-zeros-usize` | [Data.Machine](Data.Machine.md#value-747261696c696e672d7a65726f732d7573697a65) |
| `transpose` | [Data.List](Data.List.md#value-7472616e73706f7365) |
| `u16-to-f32` | [Data.Machine](Data.Machine.md#value-7531362d746f2d663332) |
| `u16-to-f64` | [Data.Machine](Data.Machine.md#value-7531362d746f2d663634) |
| `u16-to-i32` | [Data.Machine](Data.Machine.md#value-7531362d746f2d693332) |
| `u16-to-i64` | [Data.Machine](Data.Machine.md#value-7531362d746f2d693634) |
| `u16-to-isize` | [Data.Machine](Data.Machine.md#value-7531362d746f2d6973697a65) |
| `u16-to-nat` | [Data.Machine](Data.Machine.md#value-7531362d746f2d6e6174) |
| `u16-to-u16` | [Data.Machine](Data.Machine.md#value-7531362d746f2d753136) |
| `u16-to-u32` | [Data.Machine](Data.Machine.md#value-7531362d746f2d753332) |
| `u16-to-u64` | [Data.Machine](Data.Machine.md#value-7531362d746f2d753634) |
| `u16-to-usize` | [Data.Machine](Data.Machine.md#value-7531362d746f2d7573697a65) |
| `u32-to-char` | [Data.Char](Data.Char.md#value-7533322d746f2d63686172) |
| `u32-to-f32` | [Data.Machine](Data.Machine.md#value-7533322d746f2d663332) |
| `u32-to-f64` | [Data.Machine](Data.Machine.md#value-7533322d746f2d663634) |
| `u32-to-i64` | [Data.Machine](Data.Machine.md#value-7533322d746f2d693634) |
| `u32-to-isize` | [Data.Machine](Data.Machine.md#value-7533322d746f2d6973697a65) |
| `u32-to-nat` | [Data.Machine](Data.Machine.md#value-7533322d746f2d6e6174) |
| `u32-to-u32` | [Data.Machine](Data.Machine.md#value-7533322d746f2d753332) |
| `u32-to-u64` | [Data.Machine](Data.Machine.md#value-7533322d746f2d753634) |
| `u32-to-usize` | [Data.Machine](Data.Machine.md#value-7533322d746f2d7573697a65) |
| `u64-to-f32` | [Data.Machine](Data.Machine.md#value-7536342d746f2d663332) |
| `u64-to-f64` | [Data.Machine](Data.Machine.md#value-7536342d746f2d663634) |
| `u64-to-u64` | [Data.Machine](Data.Machine.md#value-7536342d746f2d753634) |
| `u64-to-usize` | [Data.Machine](Data.Machine.md#value-7536342d746f2d7573697a65) |
| `u8-to-f32` | [Data.Machine](Data.Machine.md#value-75382d746f2d663332) |
| `u8-to-f64` | [Data.Machine](Data.Machine.md#value-75382d746f2d663634) |
| `u8-to-i16` | [Data.Machine](Data.Machine.md#value-75382d746f2d693136) |
| `u8-to-i32` | [Data.Machine](Data.Machine.md#value-75382d746f2d693332) |
| `u8-to-i64` | [Data.Machine](Data.Machine.md#value-75382d746f2d693634) |
| `u8-to-isize` | [Data.Machine](Data.Machine.md#value-75382d746f2d6973697a65) |
| `u8-to-nat` | [Data.Machine](Data.Machine.md#value-75382d746f2d6e6174) |
| `u8-to-u16` | [Data.Machine](Data.Machine.md#value-75382d746f2d753136) |
| `u8-to-u32` | [Data.Machine](Data.Machine.md#value-75382d746f2d753332) |
| `u8-to-u64` | [Data.Machine](Data.Machine.md#value-75382d746f2d753634) |
| `u8-to-u8` | [Data.Machine](Data.Machine.md#value-75382d746f2d7538) |
| `u8-to-usize` | [Data.Machine](Data.Machine.md#value-75382d746f2d7573697a65) |
| `uncurry` | [Function.Combinators](Function.Combinators.md#value-756e6375727279) |
| `unfold` | [Data.List](Data.List.md#value-756e666f6c64) |
| `unpack` | [Data.String](Data.String.md#value-756e7061636b) |
| `unzip` | [Data.List](Data.List.md#value-756e7a6970) |
| `usize-to-f32` | [Data.Machine](Data.Machine.md#value-7573697a652d746f2d663332) |
| `usize-to-f64` | [Data.Machine](Data.Machine.md#value-7573697a652d746f2d663634) |
| `usize-to-u64` | [Data.Machine](Data.Machine.md#value-7573697a652d746f2d753634) |
| `usize-to-usize` | [Data.Machine](Data.Machine.md#value-7573697a652d746f2d7573697a65) |
| `utf8-at` | [Data.String](Data.String.md#value-757466382d6174) |
| `utf8-char` | [Data.String](Data.String.md#value-757466382d63686172) |
| `utf8-cont` | [Data.String](Data.String.md#value-757466382d636f6e74) |
| `utf8-encode-into` | [Data.String](Data.String.md#value-757466382d656e636f64652d696e746f) |
| `utf8-encode-width` | [Data.String](Data.String.md#value-757466382d656e636f64652d7769647468) |
| `utf8-lead-payload` | [Data.String](Data.String.md#value-757466382d6c6561642d7061796c6f6164) |
| `utf8-min` | [Data.String](Data.String.md#value-757466382d6d696e) |
| `utf8-valid-from` | [Data.String](Data.String.md#value-757466382d76616c69642d66726f6d) |
| `utf8-width` | [Data.String](Data.String.md#value-757466382d7769647468) |
| `vec-stream` | [Data.Producer](Data.Producer.md#value-7665632d73747265616d) |
| `wrapping-add-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d693136) |
| `wrapping-add-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d693332) |
| `wrapping-add-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d693634) |
| `wrapping-add-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d6938) |
| `wrapping-add-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d6973697a65) |
| `wrapping-add-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d753136) |
| `wrapping-add-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d753332) |
| `wrapping-add-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d753634) |
| `wrapping-add-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d7538) |
| `wrapping-add-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6164642d7573697a65) |
| `wrapping-i16-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d693136) |
| `wrapping-i16-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d693332) |
| `wrapping-i16-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d693634) |
| `wrapping-i16-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d6938) |
| `wrapping-i16-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d6973697a65) |
| `wrapping-i16-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d753136) |
| `wrapping-i16-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d753332) |
| `wrapping-i16-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d753634) |
| `wrapping-i16-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d7538) |
| `wrapping-i16-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6931362d746f2d7573697a65) |
| `wrapping-i32-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d693136) |
| `wrapping-i32-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d693332) |
| `wrapping-i32-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d693634) |
| `wrapping-i32-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d6938) |
| `wrapping-i32-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d6973697a65) |
| `wrapping-i32-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d753136) |
| `wrapping-i32-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d753332) |
| `wrapping-i32-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d753634) |
| `wrapping-i32-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d7538) |
| `wrapping-i32-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6933322d746f2d7573697a65) |
| `wrapping-i64-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d693136) |
| `wrapping-i64-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d693332) |
| `wrapping-i64-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d693634) |
| `wrapping-i64-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d6938) |
| `wrapping-i64-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d6973697a65) |
| `wrapping-i64-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d753136) |
| `wrapping-i64-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d753332) |
| `wrapping-i64-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d753634) |
| `wrapping-i64-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d7538) |
| `wrapping-i64-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6936342d746f2d7573697a65) |
| `wrapping-i8-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d693136) |
| `wrapping-i8-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d693332) |
| `wrapping-i8-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d693634) |
| `wrapping-i8-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d6938) |
| `wrapping-i8-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d6973697a65) |
| `wrapping-i8-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d753136) |
| `wrapping-i8-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d753332) |
| `wrapping-i8-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d753634) |
| `wrapping-i8-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d7538) |
| `wrapping-i8-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d69382d746f2d7573697a65) |
| `wrapping-isize-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d693136) |
| `wrapping-isize-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d693332) |
| `wrapping-isize-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d693634) |
| `wrapping-isize-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d6938) |
| `wrapping-isize-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d6973697a65) |
| `wrapping-isize-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d753136) |
| `wrapping-isize-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d753332) |
| `wrapping-isize-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d753634) |
| `wrapping-isize-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d7538) |
| `wrapping-isize-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6973697a652d746f2d7573697a65) |
| `wrapping-mul-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d693136) |
| `wrapping-mul-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d693332) |
| `wrapping-mul-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d693634) |
| `wrapping-mul-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d6938) |
| `wrapping-mul-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d6973697a65) |
| `wrapping-mul-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d753136) |
| `wrapping-mul-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d753332) |
| `wrapping-mul-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d753634) |
| `wrapping-mul-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d7538) |
| `wrapping-mul-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6d756c2d7573697a65) |
| `wrapping-neg-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d693136) |
| `wrapping-neg-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d693332) |
| `wrapping-neg-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d693634) |
| `wrapping-neg-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d6938) |
| `wrapping-neg-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d6973697a65) |
| `wrapping-neg-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d753136) |
| `wrapping-neg-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d753332) |
| `wrapping-neg-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d753634) |
| `wrapping-neg-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d7538) |
| `wrapping-neg-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d6e65672d7573697a65) |
| `wrapping-shl-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d693136) |
| `wrapping-shl-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d693332) |
| `wrapping-shl-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d693634) |
| `wrapping-shl-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d6938) |
| `wrapping-shl-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d6973697a65) |
| `wrapping-shl-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d753136) |
| `wrapping-shl-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d753332) |
| `wrapping-shl-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d753634) |
| `wrapping-shl-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d7538) |
| `wrapping-shl-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d73686c2d7573697a65) |
| `wrapping-shr-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d693136) |
| `wrapping-shr-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d693332) |
| `wrapping-shr-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d693634) |
| `wrapping-shr-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d6938) |
| `wrapping-shr-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d6973697a65) |
| `wrapping-shr-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d753136) |
| `wrapping-shr-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d753332) |
| `wrapping-shr-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d753634) |
| `wrapping-shr-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d7538) |
| `wrapping-shr-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7368722d7573697a65) |
| `wrapping-sub-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d693136) |
| `wrapping-sub-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d693332) |
| `wrapping-sub-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d693634) |
| `wrapping-sub-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d6938) |
| `wrapping-sub-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d6973697a65) |
| `wrapping-sub-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d753136) |
| `wrapping-sub-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d753332) |
| `wrapping-sub-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d753634) |
| `wrapping-sub-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d7538) |
| `wrapping-sub-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7375622d7573697a65) |
| `wrapping-u16-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d693136) |
| `wrapping-u16-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d693332) |
| `wrapping-u16-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d693634) |
| `wrapping-u16-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d6938) |
| `wrapping-u16-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d6973697a65) |
| `wrapping-u16-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d753136) |
| `wrapping-u16-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d753332) |
| `wrapping-u16-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d753634) |
| `wrapping-u16-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d7538) |
| `wrapping-u16-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7531362d746f2d7573697a65) |
| `wrapping-u32-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d693136) |
| `wrapping-u32-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d693332) |
| `wrapping-u32-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d693634) |
| `wrapping-u32-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d6938) |
| `wrapping-u32-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d6973697a65) |
| `wrapping-u32-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d753136) |
| `wrapping-u32-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d753332) |
| `wrapping-u32-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d753634) |
| `wrapping-u32-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d7538) |
| `wrapping-u32-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7533322d746f2d7573697a65) |
| `wrapping-u64-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d693136) |
| `wrapping-u64-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d693332) |
| `wrapping-u64-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d693634) |
| `wrapping-u64-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d6938) |
| `wrapping-u64-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d6973697a65) |
| `wrapping-u64-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d753136) |
| `wrapping-u64-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d753332) |
| `wrapping-u64-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d753634) |
| `wrapping-u64-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d7538) |
| `wrapping-u64-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7536342d746f2d7573697a65) |
| `wrapping-u8-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d693136) |
| `wrapping-u8-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d693332) |
| `wrapping-u8-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d693634) |
| `wrapping-u8-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d6938) |
| `wrapping-u8-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d6973697a65) |
| `wrapping-u8-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d753136) |
| `wrapping-u8-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d753332) |
| `wrapping-u8-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d753634) |
| `wrapping-u8-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d7538) |
| `wrapping-u8-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d75382d746f2d7573697a65) |
| `wrapping-usize-to-i16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d693136) |
| `wrapping-usize-to-i32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d693332) |
| `wrapping-usize-to-i64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d693634) |
| `wrapping-usize-to-i8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d6938) |
| `wrapping-usize-to-isize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d6973697a65) |
| `wrapping-usize-to-u16` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d753136) |
| `wrapping-usize-to-u32` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d753332) |
| `wrapping-usize-to-u64` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d753634) |
| `wrapping-usize-to-u8` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d7538) |
| `wrapping-usize-to-usize` | [Data.Machine](Data.Machine.md#value-7772617070696e672d7573697a652d746f2d7573697a65) |
| `xor` | [Data.Bool](Data.Bool.md#value-786f72) |
| `zero-u32` | [Data.Machine](Data.Machine.md#value-7a65726f2d753332) |
| `zip` | [Data.List](Data.List.md#value-7a6970) |
| `zip-with` | [Data.List](Data.List.md#value-7a69702d77697468) |
| `\|` | [Algebra.Bits](Algebra.Bits.md#value-7c) |
| `\|\|` | [Data.Bool](Data.Bool.md#value-7c7c) |

# Operator fixities

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">infixl</span> <span class="tok-number">9</span> <span class="tok-operator">%</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">6</span> <span class="tok-operator">&amp;</span>
<span class="tok-keyword">infixr</span> <span class="tok-number">2</span> <span class="tok-operator">&amp;&amp;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">9</span> <span class="tok-operator">*</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">8</span> <span class="tok-operator">+</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">8</span> <span class="tok-operator">-</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">9</span> <span class="tok-operator">/</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">/=</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">&lt;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">7</span> <span class="tok-operator">&lt;&lt;</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">&lt;=</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">==</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">&gt;</span>
<span class="tok-keyword">infix</span> <span class="tok-number">3</span> <span class="tok-operator">&gt;=</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">7</span> <span class="tok-operator">&gt;&gt;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">5</span> <span class="tok-operator">^</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">4</span> |
<span class="tok-keyword">infixr</span> <span class="tok-number">1</span> <span class="tok-operator">||</span></code></pre>

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Prelude.cinch`. Source SHA-256: `7c66ae8376fdb2aec7c267b7a2a164e60eb7ada7c3b41fe8414cad81c55502f7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
