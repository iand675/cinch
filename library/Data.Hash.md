# Data.Hash

Hashing vocabulary and admitted hash implementations for keys.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Hash</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Hash/">View source</a><a href="/library/search/?q=%2BData.Hash">Search module</a><a href="/reference-sources/Data.Hash.cinch" download="Data.Hash.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Data.Hash: the hashing class.

`hash` maps a key to a `Nat` bucket digest. The class law is
consistency with `Eq` (asserted over sample values in
seed/tests/prelude_split.rs, and over every shipped key type
in seed/tests/lib_hash_keys.rs):
  x == y implies hash x == hash y
Equality without a common hash is a broken key, so every keyed
structure states both constraints (Data.HashMap does).

`Sym` hashing (design/base-library.md) is identity-based and
trivial: a `Sym` is a word handle to a process-wide interned name
(primitive types specification section 1), so its identity IS its
hash. Data.Sym is now that home; because nothing exposes a cell
handle as data, the instance there hashes the interned name's bytes,
which is the reachable stand-in for the identity and the only digest
that is reproducible across runs.

Instance homes. The ownership rule is "instances live with their
type": the instances live in the carriers' vocabulary modules
(Data.Nat, Data.Int, Data.Bool, Data.Char, Data.String, Data.Bytes,
Data.Sym, Data.Rational). The class needs only builtin types
(`hash :: a -> Nat`), so the class home sits at the bottom of the
module order.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Hash">Hash</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Hash</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Hash-hash">hash</a><code class="syn-sig"><strong class="tok-variable">hash</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Hash">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Hash</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Hash" title="Permalink">#</a><a href="/library/source/Data.Hash/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Hash-hash"><div class="doc-method-row"><code><strong class="tok-variable">hash</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#m-Hash-hash" title="Permalink">#</a><a href="/library/source/Data.Hash/#L30">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (10)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bool/#instances">Data.Bool</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bytes/#instances">Data.Bytes</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Char/#instances">Data.Char</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Rational/#instances">Data.Rational</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Sym/#instances">Data.Sym</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Uuid/#instances">Data.Uuid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-constructor">MimeKey</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Mime/#instances">Text.Mime</a></span></li></ul></details>
</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Hash.cinch`](/library/source/Data.Hash/). Source SHA-256: `c80d835dfc4224d8eca8375d24058a77034087077186e85f10b2ba48b1fcc991`.
