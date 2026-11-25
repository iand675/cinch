# Text.Lex

Lexer toolkit: tokens, spans, and lexical errors for grammar and parser clients.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Lex</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Lex/source/">View source</a><a href="/packages/search/?q=%2BText.Lex">Search module</a><a href="/reference-sources/Text.Lex.cinch" download="Text.Lex.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Lex: token-level lexing and the layout pass.

Module scheme (design/base-library.md): `Text.*` is text processing
over String/Bytes. This module is the tokenization half of the P30
parsing toolkit (`Text.Parse` is the combinator half): it turns
UTF-8 source bytes into the located token stream the surface
grammar's EBNF consumes, WITH the deterministic layout pass already
applied ("the EBNF consumes explicit `{`, `;` and `}` tokens after a
deterministic layout pass", surface syntax and grammar, Layout and
lexing).

No drift from the layout spec is the design rule here. Every rule
this scanner implements cites the sentence it follows, and the
behavior is pinned token for token against the bootstrap compiler's
own lexer and layout pass (`seed/src/lexer.rs`,
`seed/src/layout.rs`) in `seed/tests/parsers.rs`: the
toolkit's token stream must agree with the compiler's on the shared
corpus (kind, spelling, line, column, virtual flag), so a spec
change that reaches the compiler cannot silently leave this module
behind. Where the two could disagree, this module follows the
compiler.

The lexical classes follow "Layout and lexing" and the local
amendments: identifiers with dash glue where a `-` is tight on both
sides between identifier characters, digits not continuing a name
(amendment section 8); a `-` tight only to what follows signing a
negative literal unless the previous character ends an expression
(amendment section 8); reserved punctuation matched longest first
(`::`, `->`, `<-`, `=>`, `[|`, `|]`, `$(`, the multiplicities, and
the bare reserved symbols `!`, `@`, `.`, `..`, `=`, `|`, `$`, `\`,
`~`); comment recognition taking priority over symbol identifiers;
`$` continuing an operator run (`<$>`) but never starting one;
strings supporting `\\`, `\"`, `\'`, `\n`, `\r`, `\t`, `\u{HEX}`
with a valid scalar; tabs rejected in indentation; line endings
normalizing to LF.

The self-hosting forms the compiler's own sources use are lexed
here, each as a token that carries what a parser needs:
  `#[name]`, `#[name(...)]` and `#[name=...]` attribute annotations
    (amendment section 5) become one `TAttr` token holding the inner
    text between `#[` and `]`;
  `prim` bodies (primitive operations and inline target IR
    specification section 2) become the `prim` keyword plus the
    balanced `{ ... }` region a registered family name opens, as
    the compiler emits it: `TPunct "{"`, one `TPrimFrag` token
    holding the verbatim inner IR text, and `TPunct "}"`;
  the tick name forms `'name` and `''T` (amendment section 12)
    become `TTickName` and `TTickType` tokens holding the name
    (only the `Char` literal form `'c'` stays a `TChar`).

Named refusals (malformed input each is an error naming itself,
never a silent drift): `RefusedAttr` for an attribute with no valid
name or no closing `]`, `RefusedPrim` for an unterminated `prim`
body fragment, and `RefusedTickName` for a tick that is neither a
`Char` literal nor a name. All three kinds stay exported names (a
module that mentions one keeps resolving, and `Text.Parse`'s error
vocabulary is unchanged); what moved is which inputs reach them: the
three well-formed forms above once refused and now lex to the
carrying tokens, so a refusal means malformed input, not a form
this module has not learned.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-LToken">LToken</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LToken</strong></code></li><li><a class="syn-name" href="#t-LexError">LexError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LexError</strong></code></li><li><a class="syn-name" href="#t-LexErrorKind">LexErrorKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LexErrorKind</strong></code></li><li><a class="syn-name" href="#t-Lexed">Lexed</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Lexed</strong></code></li><li><a class="syn-name" href="#t-Span">Span</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Span</strong></code></li><li><a class="syn-name" href="#t-Token">Token</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Token</strong></code></li><li><a class="syn-name" href="#v-bytes-slice">bytes-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-slice</strong> :: <span class="tok-variable">arg2</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-chars-bytes">chars-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chars-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-eq-token">eq-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eq-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-layout-tokens">layout-tokens</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">layout-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code></li><li><a class="syn-name" href="#v-lex-error-kind">lex-error-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lex-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexError" title="Text.Lex.LexError">LexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li><a class="syn-name" href="#v-lex-error-message">lex-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lex-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-lex-error-span">lex-error-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lex-error-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexError" title="Text.Lex.LexError">LexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li><li><a class="syn-name" href="#v-lex-source">lex-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lex-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code></li><li><a class="syn-name" href="#v-lex-tokens">lex-tokens</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lex-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code></li><li><a class="syn-name" href="#v-lt-span">lt-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lt-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li><li><a class="syn-name" href="#v-lt-token">lt-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lt-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li><a class="syn-name" href="#v-lt-virtual">lt-virtual</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lt-virtual</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-render-token">render-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-span-col">span-col</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-col</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-span-end">span-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-span-line">span-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-span-start">span-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-token-kind-name">token-kind-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">token-kind-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-LToken">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LToken</strong></code><span class="doc-sig-links"><a href="#t-LToken" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

A token with its span; the flag marks `{`, `;`, `}` inserted by
the layout pass (the compiler's `Token.virtual_`).

</div>
</article>

<article class="doc-entry" id="t-LexError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LexError</strong></code><span class="doc-sig-links"><a href="#t-LexError" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L357" title="View source at line 357">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LexError"><code><strong class="tok-constructor">LexError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexError" title="Text.Lex.LexError">LexError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-LexErrorKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LexErrorKind</strong></code><span class="doc-sig-links"><a href="#t-LexErrorKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L336" title="View source at line 336">Source</a></span></div>
<div class="doc-body">

Every rejection this module can produce. The layout kinds carry
the columns the compiler's diagnostics name; the refusal kinds are
the named unproductions of the module header.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TabInIndentation"><code><strong class="tok-constructor">TabInIndentation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-UnterminatedComment"><code><strong class="tok-constructor">UnterminatedComment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-UnterminatedString"><code><strong class="tok-constructor">UnterminatedString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-UnterminatedChar"><code><strong class="tok-constructor">UnterminatedChar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-BadEscape"><code><strong class="tok-constructor">BadEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-MalformedUnicodeEscape"><code><strong class="tok-constructor">MalformedUnicodeEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-NonScalarEscape"><code><strong class="tok-constructor">NonScalarEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-MalformedNumber"><code><strong class="tok-constructor">MalformedNumber</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-UnexpectedChar"><code><strong class="tok-constructor">UnexpectedChar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-MalformedTick"><code><strong class="tok-constructor">MalformedTick</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-LayoutTooShallow"><code><strong class="tok-constructor">LayoutTooShallow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-MismatchedDelimiter"><code><strong class="tok-constructor">MismatchedDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-UnclosedDelimiter"><code><strong class="tok-constructor">UnclosedDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-InWithoutLet"><code><strong class="tok-constructor">InWithoutLet</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-InCrossesDelimiter"><code><strong class="tok-constructor">InCrossesDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-RefusedAttr"><code><strong class="tok-constructor">RefusedAttr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-RefusedPrim"><code><strong class="tok-constructor">RefusedPrim</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li><li id="k-RefusedTickName"><code><strong class="tok-constructor">RefusedTickName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Lexed">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Lexed</strong></code><span class="doc-sig-links"><a href="#t-Lexed" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L390" title="View source at line 390">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LexedTokens"><code><strong class="tok-constructor">LexedTokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code></li><li id="k-LexedFail"><code><strong class="tok-constructor">LexedFail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Span">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Span</strong></code><span class="doc-sig-links"><a href="#t-Span" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

A source span: byte offsets [start, end) and the start position as
line and character column (the compiler's `Span` shape).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Span"><code><strong class="tok-constructor">Span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Token">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Token</strong></code><span class="doc-sig-links"><a href="#t-Token" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

One token of the surface grammar's tokenization: the lexical
classes of "Layout and lexing" after the local amendments. The
spelling payloads are the canonical forms the compiler's lexer
records: `TInt` is the canonical decimal digit string (hex folded,
underscores dropped, leading zeros stripped, a leading `-` for a
signed literal), `TFloat` the scanned decimal text, `TStr` the
decoded UTF-8 content, `TChar` the decoded scalar.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TVar"><code><strong class="tok-constructor">TVar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TCon"><code><strong class="tok-constructor">TCon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TOp"><code><strong class="tok-constructor">TOp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TInt"><code><strong class="tok-constructor">TInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TFloat"><code><strong class="tok-constructor">TFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TStr"><code><strong class="tok-constructor">TStr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TChar"><code><strong class="tok-constructor">TChar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TWord"><code><strong class="tok-constructor">TWord</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TPunct"><code><strong class="tok-constructor">TPunct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TAttr"><code><strong class="tok-constructor">TAttr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TPrimFrag"><code><strong class="tok-constructor">TPrimFrag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TTickName"><code><strong class="tok-constructor">TTickName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TTickType"><code><strong class="tok-constructor">TTickType</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li id="k-TEOF"><code><strong class="tok-constructor">TEOF</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-bytes-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-slice</strong> :: <span class="tok-variable">arg2</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-slice" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-chars-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chars-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-chars-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-eq-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eq-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-eq-token" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-layout-tokens">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">layout-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code><span class="doc-sig-links"><a href="#v-layout-tokens" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lex-error-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lex-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexError" title="Text.Lex.LexError">LexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a></code><span class="doc-sig-links"><a href="#v-lex-error-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lex-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lex-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexErrorKind" title="Text.Lex.LexErrorKind">LexErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-lex-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lex-error-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lex-error-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LexError" title="Text.Lex.LexError">LexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code><span class="doc-sig-links"><a href="#v-lex-error-span" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lex-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lex-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code><span class="doc-sig-links"><a href="#v-lex-source" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lex-tokens">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lex-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Lexed" title="Text.Lex.Lexed">Lexed</a></code><span class="doc-sig-links"><a href="#v-lex-tokens" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lt-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lt-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code><span class="doc-sig-links"><a href="#v-lt-span" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lt-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lt-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code><span class="doc-sig-links"><a href="#v-lt-token" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-lt-virtual">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lt-virtual</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-lt-virtual" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-render-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-token" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-col">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-col</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-span-col" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-span-end" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-span-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-span-start" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-token-kind-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">token-kind-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-token-kind-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Lex/source/" title="View source">Source</a></span></div>
</article>
