# Text.Parse

A source-distributed parser toolkit over the lexer/scanner vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Parse</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Parse/source/">View source</a><a href="/packages/search/?q=%2BText.Parse">Search module</a><a href="/reference-sources/Text.Parse.cinch" download="Text.Parse.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Lex](Text.Lex.md), [Text.Scan](Text.Scan.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Text.Parse`: [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md).

```sh
cp seed/prelude/Text/Lex.cinch ./Text.Lex.cinch
cp seed/prelude/Text/Parse.cinch ./Text.Parse.cinch
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (52 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ErrorItem">ErrorItem</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ErrorItem</strong></code></li><li><a class="syn-name" href="#t-PState">PState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PState</strong></code></li><li><a class="syn-name" href="#t-ParseError">ParseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseError</strong></code></li><li><a class="syn-name" href="#t-ParseErrorBundle">ParseErrorBundle</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseErrorBundle</strong></code></li><li><a class="syn-name" href="#t-ParseOutcome">ParseOutcome</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseOutcome</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-Parser">Parser</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Parser</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-Reply">Reply</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Reply</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-attempt-p">attempt-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">attempt-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-between-p">between-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">between-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">open</span> <span class="tok-variable">close</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">open</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">close</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-bundle-errors">bundle-errors</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bundle-errors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code></li><li><a class="syn-name" href="#v-bundle-name">bundle-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bundle-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-bundle-source">bundle-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bundle-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-choice-p">choice-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">choice-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-error-messages">error-messages</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">error-messages</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a></code></li><li><a class="syn-name" href="#v-error-span">error-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">error-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li><li><a class="syn-name" href="#v-expect-eof">expect-eof</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expect-eof</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-expect-op">expect-op</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expect-op</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-expect-punct">expect-punct</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expect-punct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-expect-tok">expect-tok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expect-tok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code></li><li><a class="syn-name" href="#v-expect-word">expect-word</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">expect-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-fail-p">fail-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fail-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-label-p">label-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">label-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-look-ahead-p">look-ahead-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">look-ahead-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-many-p">many-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">many-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-not-followed-by-p">not-followed-by-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">not-followed-by-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-optional-p">optional-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">optional-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-or-else">or-else</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-parse-bind">parse-bind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-parse-fail">parse-fail</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-fail</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Parse</span>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-parse-fmap">parse-fmap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-fmap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-parse-note">parse-note</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-note</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Parse</span>} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-parse-pure">parse-pure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-pstate-count">pstate-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pstate-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pstate-last">pstate-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pstate-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code></li><li><a class="syn-name" href="#v-pstate-soft">pstate-soft</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pstate-soft</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code></li><li><a class="syn-name" href="#v-pstate-tokens">pstate-tokens</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pstate-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a></code></li><li><a class="syn-name" href="#v-render-bundle">render-bundle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-bundle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-render-item">render-item</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-item</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-run-parser">run-parser</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">run-parser</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-run-parser-fn">run-parser-fn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">run-parser-fn</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Reply" title="Text.Parse.Reply">Reply</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-run-parser-with">run-parser-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">run-parser-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-satisfy-char">satisfy-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-satisfy-con">satisfy-con</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-con</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-satisfy-float">satisfy-float</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-satisfy-int">satisfy-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-satisfy-str">satisfy-str</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-str</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-satisfy-var">satisfy-var</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">satisfy-var</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-sep-by-p">sep-by-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sep-by-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">s</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-some-p">some-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">some-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-spanned-p">spanned-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">spanned-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a>, <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-token-p">token-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">token-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-with-recovery-p">with-recovery-p</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-recovery-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ErrorItem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ErrorItem</strong></code><span class="doc-sig-links"><a href="#t-ErrorItem" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

The item vocabulary of megaparsec's `ErrorItem`: a concrete
token, a named label, or the end of input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ITok"><code><strong class="tok-constructor">ITok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a></code></li><li id="k-ILabel"><code><strong class="tok-constructor">ILabel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a></code></li><li id="k-IEnd"><code><strong class="tok-constructor">IEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PState</strong></code><span class="doc-sig-links"><a href="#t-PState" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Parser state: the remaining located tokens, how many tokens were
consumed (the span combinator's exactness), the span of the last
consumed token, and the recovered (soft) errors so far.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PState"><code><strong class="tok-constructor">PState</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ParseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseError</strong></code><span class="doc-sig-links"><a href="#t-ParseError" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

A typed parse error: megaparsec's trivial error (the offending
item and the expected set) or its fancy `MessageFail`, each with
the source span the error stands at.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-EUnexpected"><code><strong class="tok-constructor">EUnexpected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code></li><li id="k-EFail"><code><strong class="tok-constructor">EFail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ParseErrorBundle">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseErrorBundle</strong></code><span class="doc-sig-links"><a href="#t-ParseErrorBundle" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

The bundle: the source name and text every error's positions
index into, and the errors themselves (the hard failure first,
then the recovered ones in occurrence order).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Bundle"><code><strong class="tok-constructor">Bundle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ParseOutcome">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ParseOutcome</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-ParseOutcome" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L571" title="View source at line 571">Source</a></span></div>
<div class="doc-body">

The parse outcome: the value with its recovered errors, or the
megaparsec-style bundle of the hard failure plus recovered ones.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Parsed"><code><strong class="tok-constructor">Parsed</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code></li><li id="k-Failed"><code><strong class="tok-constructor">Failed</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Parser">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Parser</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Parser" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

A parser: a function from state to reply (megaparsec's `ParsecT`
spine specialized to the pure token stream).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkParser"><code><strong class="tok-constructor">MkParser</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Reply" title="Text.Parse.Reply">Reply</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Reply">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Reply</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Reply" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L304" title="View source at line 304">Source</a></span></div>
<div class="doc-body">

A reply: megaparsec's consumed/empty success and failure
distinction. `ROk` carries the expected-set hint for merging;
`RErr` carries the error and the recovered errors so far.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ROk"><code><strong class="tok-constructor">ROk</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Reply" title="Text.Parse.Reply">Reply</a> <span class="tok-variable">a</span></code></li><li id="k-RErr"><code><strong class="tok-constructor">RErr</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Reply" title="Text.Parse.Reply">Reply</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-attempt-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">attempt-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-attempt-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

Backtracking (megaparsec's `try`): a failure becomes an
empty-failure so `or-else` may try the alternative.

</div>
</article>

<article class="doc-entry" id="v-between-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">between-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">open</span> <span class="tok-variable">close</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">open</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">close</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-between-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L518" title="View source at line 518">Source</a></span></div>
</article>

<article class="doc-entry" id="v-bundle-errors">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bundle-errors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code><span class="doc-sig-links"><a href="#v-bundle-errors" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L129" title="View source at line 129">Source</a></span></div>
</article>

<article class="doc-entry" id="v-bundle-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bundle-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-bundle-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L121" title="View source at line 121">Source</a></span></div>
</article>

<article class="doc-entry" id="v-bundle-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bundle-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bundle-source" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L125" title="View source at line 125">Source</a></span></div>
</article>

<article class="doc-entry" id="v-choice-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">choice-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-choice-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L358" title="View source at line 358">Source</a></span></div>
</article>

<article class="doc-entry" id="v-error-messages">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">error-messages</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a></code><span class="doc-sig-links"><a href="#v-error-messages" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

The expected-item set (empty for `EFail`): the `expecting` line of
the rendered bundle.

</div>
</article>

<article class="doc-entry" id="v-error-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">error-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code><span class="doc-sig-links"><a href="#v-error-span" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L103" title="View source at line 103">Source</a></span></div>
</article>

<article class="doc-entry" id="v-expect-eof">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expect-eof</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-expect-eof" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L456" title="View source at line 456">Source</a></span></div>
<div class="doc-body">

The end-of-input guard (megaparsec's `eof`): succeeds WITHOUT
consuming when the input is at end (the stream is empty or headed
by the lex pass's `TEOF` marker), and fails empty otherwise. A
guard, never a consumer: it is idempotent (running it twice
succeeds twice) and stands alone in a user parser even though
`run-parser-pure` appends its own whole-input `expect-eof` (the
API finding recorded at D307: the old token-consuming form ran
twice under the runner and the second run failed).

</div>
</article>

<article class="doc-entry" id="v-expect-op">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expect-op</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-expect-op" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L413" title="View source at line 413">Source</a></span></div>
</article>

<article class="doc-entry" id="v-expect-punct">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expect-punct</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-expect-punct" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L408" title="View source at line 408">Source</a></span></div>
</article>

<article class="doc-entry" id="v-expect-tok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expect-tok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a></code><span class="doc-sig-links"><a href="#v-expect-tok" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L400" title="View source at line 400">Source</a></span></div>
</article>

<article class="doc-entry" id="v-expect-word">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">expect-word</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-expect-word" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L403" title="View source at line 403">Source</a></span></div>
</article>

<article class="doc-entry" id="v-fail-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fail-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-fail-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L468" title="View source at line 468">Source</a></span></div>
<div class="doc-body">

The explicit failure (megaparsec's `fail`): a fancy error at the
current position.

</div>
</article>

<article class="doc-entry" id="v-label-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">label-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-label-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L367" title="View source at line 367">Source</a></span></div>
<div class="doc-body">

Naming a parser's expectation (megaparsec's `label`): an
empty-failure's expected set becomes the label.

</div>
</article>

<article class="doc-entry" id="v-look-ahead-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">look-ahead-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-look-ahead-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L473" title="View source at line 473">Source</a></span></div>
<div class="doc-body">

Look-ahead: parse without consuming.

</div>
</article>

<article class="doc-entry" id="v-many-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">many-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-many-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L494" title="View source at line 494">Source</a></span></div>
<div class="doc-body">

Repetition and separators. `many-p` never fails; both stop at the
first empty-failure and propagate consumed failures.

</div>
</article>

<article class="doc-entry" id="v-not-followed-by-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">not-followed-by-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-not-followed-by-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L480" title="View source at line 480">Source</a></span></div>
<div class="doc-body">

megaparsec's `notFollowedBy`: succeed (consuming nothing) only
when the parser fails.

</div>
</article>

<article class="doc-entry" id="v-optional-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">optional-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-optional-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L515" title="View source at line 515">Source</a></span></div>
</article>

<article class="doc-entry" id="v-or-else">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">or-else</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-or-else" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L349" title="View source at line 349">Source</a></span></div>
<div class="doc-body">

Choice (megaparsec's `<|>`): the right parser runs only when the
left failed without consuming; two empty failures merge.

</div>
</article>

<article class="doc-entry" id="v-parse-bind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-parse-bind" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L327" title="View source at line 327">Source</a></span></div>
<div class="doc-body">

Sequencing (megaparsec's `>>=`): consumed flags combine, and the
expected hints merge only when the second parser consumed
nothing.

</div>
</article>

<article class="doc-entry" id="v-parse-fail">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-fail</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Parse</span>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-parse-fail" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-fmap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-fmap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-parse-fmap" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L336" title="View source at line 336">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-note">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-note</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Parse</span>} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-parse-note" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-parse-pure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-parse-pure" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

The combinator core.

</div>
</article>

<article class="doc-entry" id="v-pstate-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pstate-count</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pstate-count" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L289" title="View source at line 289">Source</a></span></div>
</article>

<article class="doc-entry" id="v-pstate-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pstate-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a></code><span class="doc-sig-links"><a href="#v-pstate-last" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L293" title="View source at line 293">Source</a></span></div>
</article>

<article class="doc-entry" id="v-pstate-soft">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pstate-soft</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a></code><span class="doc-sig-links"><a href="#v-pstate-soft" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L297" title="View source at line 297">Source</a></span></div>
</article>

<article class="doc-entry" id="v-pstate-tokens">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pstate-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-LToken" title="Text.Lex.LToken">LToken</a></code><span class="doc-sig-links"><a href="#v-pstate-tokens" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L285" title="View source at line 285">Source</a></span></div>
</article>

<article class="doc-entry" id="v-render-bundle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-bundle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseErrorBundle" title="Text.Parse.ParseErrorBundle">ParseErrorBundle</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-bundle" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L235" title="View source at line 235">Source</a></span></div>
<div class="doc-body">

The bundle's rendering: every error, megaparsec-style, in order.

</div>
</article>

<article class="doc-entry" id="v-render-item">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-item</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-item" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L133" title="View source at line 133">Source</a></span></div>
</article>

<article class="doc-entry" id="v-run-parser">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">run-parser</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-run-parser" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L619" title="View source at line 619">Source</a></span></div>
<div class="doc-body">

The runner with the identity recovery handler: the bundle carries
the typed errors exactly as the parser reported them.

</div>
</article>

<article class="doc-entry" id="v-run-parser-fn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">run-parser-fn</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-PState" title="Text.Parse.PState">PState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Reply" title="Text.Parse.Reply">Reply</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-run-parser-fn" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L313" title="View source at line 313">Source</a></span></div>
</article>

<article class="doc-entry" id="v-run-parser-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">run-parser-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseOutcome" title="Text.Parse.ParseOutcome">ParseOutcome</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-run-parser-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L595" title="View source at line 595">Source</a></span></div>
<div class="doc-body">

The recovery handler as an ordinary function on the typed error
(the megaparsec-style bundle's final shape is the handler's
choice). The failing path performs `parse-fail` in the handled
computation's own expression and the `handler Parse where` clause
set recovers the typed error into the bundle: typed Parse-effect
errors with recovery handlers, within the bootstrap's lexical
dispatch (named refusal above, decision D23). The hard error
passes through `recover`; the recovered errors of the run stay in
the bundle.

</div>
</article>

<article class="doc-entry" id="v-satisfy-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-satisfy-char" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L443" title="View source at line 443">Source</a></span></div>
</article>

<article class="doc-entry" id="v-satisfy-con">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-con</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-satisfy-con" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L423" title="View source at line 423">Source</a></span></div>
</article>

<article class="doc-entry" id="v-satisfy-float">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-float</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-satisfy-float" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L433" title="View source at line 433">Source</a></span></div>
</article>

<article class="doc-entry" id="v-satisfy-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-satisfy-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L428" title="View source at line 428">Source</a></span></div>
</article>

<article class="doc-entry" id="v-satisfy-str">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-str</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-satisfy-str" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L438" title="View source at line 438">Source</a></span></div>
</article>

<article class="doc-entry" id="v-satisfy-var">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">satisfy-var</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-satisfy-var" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L418" title="View source at line 418">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sep-by-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sep-by-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">s</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-sep-by-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L508" title="View source at line 508">Source</a></span></div>
</article>

<article class="doc-entry" id="v-some-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">some-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-some-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L505" title="View source at line 505">Source</a></span></div>
</article>

<article class="doc-entry" id="v-spanned-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">spanned-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Span" title="Text.Lex.Span">Span</a>, <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-spanned-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L526" title="View source at line 526">Source</a></span></div>
<div class="doc-body">

Source spans (the toolkit's span story): the consumed tokens'
span, from the first consumed token's start to the last
consumed token's end; a parse that consumes nothing reports a
zero-width span at the current token.

</div>
</article>

<article class="doc-entry" id="v-token-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">token-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lex/#t-Token" title="Text.Lex.Token">Token</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ErrorItem" title="Text.Parse.ErrorItem">ErrorItem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-token-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

The token matcher primitive: consume the head token when the
predicate decodes it, otherwise an empty failure carrying the
offending item and the expected set.

</div>
</article>

<article class="doc-entry" id="v-with-recovery-p">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-recovery-p</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-ParseError" title="Text.Parse.ParseError">ParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Parse/#t-Parser" title="Text.Parse.Parser">Parser</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-with-recovery-p" title="Permalink">#</a><a href="/packages/stdlib/Text.Parse/source/#L548" title="View source at line 548">Source</a></span></div>
<div class="doc-body">

Parser-level recovery (megaparsec's `withRecovery`): on failure
the recovery handler runs at the failed parser's start state and
may consume input to resynchronize; its success keeps the
recovered error in the bundle's soft list and the parse
continues. If the recovery handler fails too, the original error
stands (the handler is tried once: one-shot recovery).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Parse.cinch`](/packages/stdlib/Text.Parse/source/). Source SHA-256: `4fe975aa19fd3e43909a368f57dd13e11e86a5ab90192834a9591e27e448da6b`.
