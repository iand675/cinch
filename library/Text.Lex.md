# Text.Lex

A source-distributed lexer toolkit: tokens, spans, and lexical errors for compiler/parser clients.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Lex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Lex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Lex](Text.Lex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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
own lexer and layout pass (`compiler/src/lexer.rs`,
`compiler/src/layout.rs`) in `compiler/tests/parsers.rs`: the
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

# Types and constructors

<a id="type-4c546f6b656e"></a>

## `LToken`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LToken</span></code></pre>

A token with its span; the flag marks `{`, `;`, `}` inserted by
the layout pass (the compiler's `Token.virtual_`).

[Source](/reference-sources/Text.Lex.cinch) · line 179

<a id="type-4c65784572726f72"></a>

## `LexError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LexError</span></code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LexError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexError</span>))</code></pre>

<a id="type-4c65784572726f724b696e64"></a>

## `LexErrorKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LexErrorKind</span></code></pre>

Every rejection this module can produce. The layout kinds carry
the columns the compiler's diagnostics name; the refusal kinds are
the named unproductions of the module header.

[Source](/reference-sources/Text.Lex.cinch) · line 336

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TabInIndentation</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnterminatedComment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnterminatedString</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnterminatedChar</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BadEscape</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MalformedNumber</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedChar</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MalformedTick</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LayoutTooShallow</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MismatchedDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnclosedDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InWithoutLet</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InCrossesDelimiter</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RefusedAttr</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RefusedPrim</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RefusedTickName</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span></code></pre>

<a id="type-4c65786564"></a>

## `Lexed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Lexed</span></code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LexedTokens</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Lexed</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LexedFail</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Lexed</span>))</code></pre>

<a id="type-5370616e"></a>

## `Span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Span</span></code></pre>

A source span: byte offsets [start, end) and the start position as
line and character column (the compiler's `Span` shape).

[Source](/reference-sources/Text.Lex.cinch) · line 125

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Span</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>))))</code></pre>

<a id="type-546f6b656e"></a>

## `Token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Token</span></code></pre>

One token of the surface grammar's tokenization: the lexical
classes of "Layout and lexing" after the local amendments. The
spelling payloads are the canonical forms the compiler's lexer
records: `TInt` is the canonical decimal digit string (hex folded,
underscores dropped, leading zeros stripped, a leading `-` for a
signed literal), `TFloat` the scanned decimal text, `TStr` the
decoded UTF-8 content, `TChar` the decoded scalar.

[Source](/reference-sources/Text.Lex.cinch) · line 151

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TVar</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TCon</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TOp</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TInt</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TFloat</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TStr</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TChar</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TWord</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TPunct</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TAttr</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TPrimFrag</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TTickName</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TTickType</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TEOF</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span></code></pre>

# Values

<a id="value-62797465732d736c696365"></a>

## `bytes-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bytes-slice</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg2</span> <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-63686172732d6279746573"></a>

## `chars-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chars-bytes</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Char</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-65712d746f6b656e"></a>

## `eq-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-token</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c61796f75742d746f6b656e73"></a>

## `layout-tokens`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">layout-tokens</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Lexed</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c65782d6572726f722d6b696e64"></a>

## `lex-error-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lex-error-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c65782d6572726f722d6d657373616765"></a>

## `lex-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lex-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexErrorKind</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c65782d6572726f722d7370616e"></a>

## `lex-error-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lex-error-span</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LexError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c65782d736f75726365"></a>

## `lex-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lex-source</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Lexed</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c65782d746f6b656e73"></a>

## `lex-tokens`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lex-tokens</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Lexed</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c742d7370616e"></a>

## `lt-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-span</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c742d746f6b656e"></a>

## `lt-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-token</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-6c742d7669727475616c"></a>

## `lt-virtual`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-virtual</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-72656e6465722d746f6b656e"></a>

## `render-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-token</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-7370616e2d636f6c"></a>

## `span-col`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-col</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-7370616e2d656e64"></a>

## `span-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-end</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-7370616e2d6c696e65"></a>

## `span-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-line</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-7370616e2d7374617274"></a>

## `span-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-start</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

<a id="value-746f6b656e2d6b696e642d6e616d65"></a>

## `token-kind-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">token-kind-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Lex.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Lex.cinch`. Source SHA-256: `c7cc9f8fe6373e93ad0c32ddf262e746da44015394f40548c3ba6d84352dc786`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
