# Text.Parse

A source-distributed parser toolkit over the lexer/scanner vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Parse`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Parse.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Lex](Text.Lex.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Parse: parser combinators over the lexed token stream.

Module scheme (design/base-library.md): `Text.*` is text processing
over String/Bytes. This module is the combinator half of the P30
parsing toolkit (`Text.Lex` is the tokenization half): parsers over
the located, laid-out token stream, each parser able to report
source spans for what it consumed, with TYPED errors (the `Parse`
effect's payload) and megaparsec-style error bundles.

The vocabulary follows megaparsec's shape where the bootstrap
allows it:

  * `Parser a` is a state function with the consumed/empty success
    and failure distinction, so `or-else` backtracks exactly when
    megaparsec's `<|>` does and `attempt-p` is the explicit escape;
  * errors are `ParseError` values: `EUnexpected` (a trivial error:
    the offending item plus the expected set) and `EFail` (a fancy
    message), each carrying the source span;
  * `ParseErrorBundle` carries the source name and text with every
    error (the hard failure first, then the recovered ones) and
    renders megaparsec-style with the source line and a caret;
  * `with-recovery-p` is megaparsec's `withRecovery`: a recovery
    handler runs on failure, its recovered error is retained in the
    bundle, and the parse continues from the recovered value.

Typed Parse-effect errors with recovery handlers. `effect Parse` is
the typed error channel (`parse-fail` aborts with a `ParseError`,
`parse-note` records one and resumes). The recovery handlers are
`handler Parse where` clause sets, and `run-parser` /
`run-parser-with` are the worked ones: the runner's failing branch
performs `parse-fail` in its own handled expression and the
recovery handler turns the typed error into the outcome's
megaparsec-style bundle, with `run-parser-with` taking the recovery
handler as an ordinary function on the error value. User code gets
the same shape for its own inline failing computations (`handle
... with (handler Parse where ...)`).

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  per-call effect dispatch (compiler/CONFORMANCE.md D23): the
    bootstrap dispatches `Parse` operations only from the handled
    computation's own expression, so a COMBINATOR's internal
    failure cannot surface through a caller's `handle`; combinators
    therefore thread typed errors in their replies and only the
    runners above perform the effect. When the bootstrap grows
    per-call dispatch, the combinator failures can move onto the
    effect without changing `ParseError`;
  multi-shot recovery: a recovery handler's value is computed once
    (the bootstrap's one-shot resumption discipline), matching
    megaparsec's `withRecovery` contract;
  incremental lexing: `run-parser` lexes and lays out the whole
    source first (`Text.Lex`), so a lex failure is one bundle error,
    not a resumable stream.

</details>

# Types and constructors

<a id="type-4572726f724974656d"></a>

## `ErrorItem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ErrorItem</span></code></pre>

The item vocabulary of megaparsec's `ErrorItem`: a concrete
token, a named label, or the end of input.

[Source](/reference-sources/Text.Parse.cinch) · line 91

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ITok</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ILabel</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span></code></pre>

<a id="type-505374617465"></a>

## `PState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PState</span></code></pre>

Parser state: the remaining located tokens, how many tokens were
consumed (the span combinator's exactness), the span of the last
consumed token, and the recovered (soft) errors so far.

[Source](/reference-sources/Text.Parse.cinch) · line 281

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PState</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span>))))</code></pre>

<a id="type-50617273654572726f72"></a>

## `ParseError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ParseError</span></code></pre>

A typed parse error: megaparsec's trivial error (the offending
item and the expected set) or its fancy `MessageFail`, each with
the source span the error stands at.

[Source](/reference-sources/Text.Parse.cinch) · line 99

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EUnexpected</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EFail</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span>))</code></pre>

<a id="type-50617273654572726f7242756e646c65"></a>

## `ParseErrorBundle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ParseErrorBundle</span></code></pre>

The bundle: the source name and text every error's positions
index into, and the errors themselves (the hard failure first,
then the recovered ones in occurrence order).

[Source](/reference-sources/Text.Parse.cinch) · line 118

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bundle</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span>)))</code></pre>

<a id="type-50617273654f7574636f6d65"></a>

## `ParseOutcome`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ParseOutcome</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The parse outcome: the value with its recovered errors, or the
megaparsec-style bundle of the hard failure plus recovered ones.

[Source](/reference-sources/Text.Parse.cinch) · line 570

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Parsed</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseOutcome</span> <span class="tok-variable">a</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Failed</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseOutcome</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-506172736572"></a>

## `Parser`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Parser</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A parser: a function from state to reply (megaparsec's `ParsecT`
spine specialized to the pure token stream).

[Source](/reference-sources/Text.Parse.cinch) · line 309

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MkParser</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Reply</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)</code></pre>

<a id="type-5265706c79"></a>

## `Reply`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Reply</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A reply: megaparsec's consumed/empty success and failure
distinction. `ROk` carries the expected-set hint for merging;
`RErr` carries the error and the recovered errors so far.

[Source](/reference-sources/Text.Parse.cinch) · line 303

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ROk</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Reply</span> <span class="tok-variable">a</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RErr</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Reply</span> <span class="tok-variable">a</span>)))</code></pre>

# Values

<a id="value-617474656d70742d70"></a>

## `attempt-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">attempt-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))</code></pre>

Backtracking (megaparsec's `try`): a failure becomes an
empty-failure so `or-else` may try the alternative.

[Source](/reference-sources/Text.Parse.cinch) · line 340

<a id="value-6265747765656e2d70"></a>

## `between-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">between-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">open</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">close</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">open</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">close</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-62756e646c652d6572726f7273"></a>

## `bundle-errors`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bundle-errors</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-62756e646c652d6e616d65"></a>

## `bundle-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bundle-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-62756e646c652d736f75726365"></a>

## `bundle-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bundle-source</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-63686f6963652d70"></a>

## `choice-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">choice-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6572726f722d6d65737361676573"></a>

## `error-messages`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">error-messages</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span>))</code></pre>

The expected-item set (empty for `EFail`): the `expecting` line of
the rendered bundle.

[Source](/reference-sources/Text.Parse.cinch) · line 110

<a id="value-6572726f722d7370616e"></a>

## `error-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">error-span</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6578706563742d656f66"></a>

## `expect-eof`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-eof</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Unit</span>)</code></pre>

The end-of-input guard (megaparsec's `eof`): succeeds WITHOUT
consuming when the input is at end (the stream is empty or headed
by the lex pass's `TEOF` marker), and fails empty otherwise. A
guard, never a consumer: it is idempotent (running it twice
succeeds twice) and stands alone in a user parser even though
`run-parser-pure` appends its own whole-input `expect-eof` (the
API finding recorded at D307: the old token-consuming form ran
twice under the runner and the second run failed).

[Source](/reference-sources/Text.Parse.cinch) · line 455

<a id="value-6578706563742d6f70"></a>

## `expect-op`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-op</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6578706563742d70756e6374"></a>

## `expect-punct`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-punct</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6578706563742d746f6b"></a>

## `expect-tok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-tok</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6578706563742d776f7264"></a>

## `expect-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expect-word</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6661696c2d70"></a>

## `fail-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fail-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))</code></pre>

The explicit failure (megaparsec's `fail`): a fancy error at the
current position.

[Source](/reference-sources/Text.Parse.cinch) · line 467

<a id="value-6c6162656c2d70"></a>

## `label-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">label-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)))</code></pre>

Naming a parser's expectation (megaparsec's `label`): an
empty-failure's expected set becomes the label.

[Source](/reference-sources/Text.Parse.cinch) · line 366

<a id="value-6c6f6f6b2d61686561642d70"></a>

## `look-ahead-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">look-ahead-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))</code></pre>

Look-ahead: parse without consuming.

[Source](/reference-sources/Text.Parse.cinch) · line 472

<a id="value-6d616e792d70"></a>

## `many-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">many-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Repetition and separators. `many-p` never fails; both stop at the
first empty-failure and propagate consumed failures.

[Source](/reference-sources/Text.Parse.cinch) · line 493

<a id="value-6e6f742d666f6c6c6f7765642d62792d70"></a>

## `not-followed-by-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">not-followed-by-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Unit</span>))</code></pre>

megaparsec's `notFollowedBy`: succeed (consuming nothing) only
when the parser fails.

[Source](/reference-sources/Text.Parse.cinch) · line 479

<a id="value-6f7074696f6e616c2d70"></a>

## `optional-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-6f722d656c7365"></a>

## `or-else`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">or-else</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)))</code></pre>

Choice (megaparsec's `<|>`): the right parser runs only when the
left failed without consuming; two empty failures merge.

[Source](/reference-sources/Text.Parse.cinch) · line 348

<a id="value-70617273652d62696e64"></a>

## `parse-bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-bind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">b</span>)))</code></pre>

Sequencing (megaparsec's `>>=`): consumed flags combine, and the
expected hints merge only when the second parser consumed
nothing.

[Source](/reference-sources/Text.Parse.cinch) · line 326

<a id="value-70617273652d6661696c"></a>

## `parse-fail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-fail</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt;{<span class="tok-constructor">Parse</span>} <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-70617273652d666d6170"></a>

## `parse-fmap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-fmap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-70617273652d6e6f7465"></a>

## `parse-note`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-note</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt;{<span class="tok-constructor">Parse</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-70617273652d70757265"></a>

## `parse-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>))</code></pre>

The combinator core.

[Source](/reference-sources/Text.Parse.cinch) · line 320

<a id="value-7073746174652d636f756e74"></a>

## `pstate-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pstate-count</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-7073746174652d6c617374"></a>

## `pstate-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pstate-last</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-7073746174652d736f6674"></a>

## `pstate-soft`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pstate-soft</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-7073746174652d746f6b656e73"></a>

## `pstate-tokens`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pstate-tokens</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">LToken</span>))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-72656e6465722d62756e646c65"></a>

## `render-bundle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-bundle</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseErrorBundle</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The bundle's rendering: every error, megaparsec-style, in order.

[Source](/reference-sources/Text.Parse.cinch) · line 234

<a id="value-72656e6465722d6974656d"></a>

## `render-item`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">render-item</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-72756e2d706172736572"></a>

## `run-parser`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">run-parser</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseOutcome</span> <span class="tok-variable">a</span>))))</code></pre>

The runner with the identity recovery handler: the bundle carries
the typed errors exactly as the parser reported them.

[Source](/reference-sources/Text.Parse.cinch) · line 618

<a id="value-72756e2d7061727365722d666e"></a>

## `run-parser-fn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">run-parser-fn</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">PState</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Reply</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-72756e2d7061727365722d77697468"></a>

## `run-parser-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">run-parser-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseOutcome</span> <span class="tok-variable">a</span>)))))</code></pre>

The recovery handler as an ordinary function on the typed error
(the megaparsec-style bundle's final shape is the handler's
choice). The failing path performs `parse-fail` in the handled
computation's own expression and the `handler Parse where` clause
set recovers the typed error into the bundle: typed Parse-effect
errors with recovery handlers, within the bootstrap's lexical
dispatch (named refusal above, decision D23). The hard error
passes through `recover`; the recovered errors of the run stay in
the bundle.

[Source](/reference-sources/Text.Parse.cinch) · line 594

<a id="value-736174697366792d63686172"></a>

## `satisfy-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-char</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736174697366792d636f6e"></a>

## `satisfy-con`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-con</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736174697366792d666c6f6174"></a>

## `satisfy-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-float</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736174697366792d696e74"></a>

## `satisfy-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-int</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736174697366792d737472"></a>

## `satisfy-str`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-str</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Bytes</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736174697366792d766172"></a>

## `satisfy-var`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">satisfy-var</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-7365702d62792d70"></a>

## `sep-by-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sep-by-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">s</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-736f6d652d70"></a>

## `some-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">some-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Text.Parse.cinch)

<a id="value-7370616e6e65642d70"></a>

## `spanned-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">spanned-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Span</span>, <span class="tok-variable">a</span>)))</code></pre>

Source spans (the toolkit's span story): the consumed tokens'
span, from the first consumed token's start to the last
consumed token's end; a parse that consumes nothing reports a
zero-width span at the current token.

[Source](/reference-sources/Text.Parse.cinch) · line 525

<a id="value-746f6b656e2d70"></a>

## `token-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">token-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lex</span>.<span class="tok-constructor">Token</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ErrorItem</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)))</code></pre>

The token matcher primitive: consume the head token when the
predicate decodes it, otherwise an empty failure carrying the
offending item and the expected set.

[Source](/reference-sources/Text.Parse.cinch) · line 381

<a id="value-776974682d7265636f766572792d70"></a>

## `with-recovery-p`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-recovery-p</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">ParseError</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Parse</span>.<span class="tok-constructor">Parser</span> <span class="tok-variable">a</span>)))</code></pre>

Parser-level recovery (megaparsec's `withRecovery`): on failure
the recovery handler runs at the failed parser's start state and
may consume input to resynchronize; its success keeps the
recovered error in the bundle's soft list and the parse
continues. If the recovery handler fails too, the original error
stands (the handler is tried once: one-shot recovery).

[Source](/reference-sources/Text.Parse.cinch) · line 547

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Parse.cinch`. Source SHA-256: `4b6d00ab84e6969cf6fe8a21f29a255a9871c048b6df4baab006734efd7fabed`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
