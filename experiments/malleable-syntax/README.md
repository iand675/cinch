# Library languages for Cinch: experimental workbench

This is an isolated front-end experiment on `codex/malleable-syntax-poc`.
It explores library-authored grammar, expansion, interleaved host code,
compiler checking, and editor composition before choosing a production API.
The original compiler checkout is untouched. `source-snapshot.json` records
the experiment's initial current-source snapshot (`fb198a8`). The bootstrap
integration replays only the prototype commits over the newer compiler; see
[the integration notes](INTEGRATION.md).

The [second pass](SECOND_PASS.md) adds projected, branching participant APIs and
shape-checked reverse-mode tensor differentiation, along with syntax expansion
inside imported source libraries.

The [tactic pass](TACTICS.md) adds goal-directed assumption and arithmetic
search, driven by the native checker's expected proposition and scoped facts.

## Run it

From the worktree root:

```sh
cd compiler
cargo build
cd ..
python3 experiments/malleable-syntax/poc.py run experiments/malleable-syntax/examples/Html.cinch
python3 experiments/malleable-syntax/poc.py expand experiments/malleable-syntax/examples/Html.cinch
python3 experiments/malleable-syntax/poc.py explain experiments/malleable-syntax/examples/Html.cinch
python3 experiments/malleable-syntax/verify.py
```

The build needs the existing Cinch Rust/clang toolchain. The experiment uses
Python's standard library; editor verification additionally needs the
Tree-sitter CLI. Generated sources, origin maps, and native build artifacts
are under `.generated/`. `CINCH_EXPERIMENT_COMPILER` can select another compiler
binary explicitly. The default is this worktree's own compiler binary.

## Start with the HTML example

```text
module Main where
  import Ui
  import syntax "../syntax/html.syntax.json" as html

  person :: String -> Html
  person name = html!{<li>{name}</li>}

  view :: String -> List String -> Html
  view title people = html!{
    <section class="people">
      <h1>{title}</h1>
      {person <$> people}
      <Button onClick={\() -> 42}>Refresh &amp; retry</Button>
    </section>
  }
```

The grammar is `syntax/html.syntax.json`; the runtime semantics are ordinary
Cinch in `libraries/Ui.cinch`. The generic expansion engine has no HTML tag or
component implementation. Markup supports text, entities, paired tags,
self-closing tags/components, fragments, string/Boolean attributes, embedded
expressions, and typed component properties. The backend currently renders HTML
on the server. Text and attribute values are escaped separately from markup.
Event properties are typed closures; constructing a view does not invoke them.
Whitespace in text nodes is preserved, including indentation.

This sample uses proposed syntax. The normal `cinch` command does not yet load
these syntax descriptors: use `poc.py` for the experimental front end.

## Runnable probes and their present boundaries

All positive programs run as actual native Cinch programs, with observable
results asserted in their `main`. Each corresponding negative fixture must be
rejected by the Cinch checker for its intended reason.

| Probe | Current executable slice | Rejection evidence | Further ambition |
| --- | --- | --- | --- |
| `Html.cinch` | SSR, code holes, components, callbacks, escaping, fragments, Boolean attributes | Callback property has the wrong type | Browser event dispatch and reactive rendering |
| `once` inside `Html.cinch` | Evaluate once, fresh temporary, deliberate user binder, call-site scope | Aliasing cannot duplicate a linear argument | General binding-aware syntax objects |
| `Query.cinch` | Generated schema record and typed list query | Projection of an absent field | Captured external schema and a database backend |
| `Tasks.cinch` | Actual structured spawn with delayed body and joined task | Dropping a task handle | Richer task blocks and cancellation notation |
| `Proof.cinch` | Kernel-checked reflexivity/exact notation | False arithmetic equality | A goal/context API for programmable tactics |
| `Tactics.cinch` | Goal-directed assumption lookup, arithmetic certificates and ordered fallback | Missing facts, shadowed indices and stale mutable evidence | A reflected goal/context API for arbitrary library-written tactics |
| `Format.cinch` | Generated packet type, parser, writer, length checks and checksum | Invalid byte-width constant; native malformed-input cases | Composable field formats, borrowed views, round-trip evidence |
| `Protocol.cinch` | Generated linear endpoint states and transitions | Sending in the wrong state | Multiparty choreography, projection, branches and transport |
| `Choreography.cinch` | Projected participant APIs, approved/declined conversations, private constructors and affine local transport | Wrong states, branches, wire types, replay and endpoint forgery | Nested choices, richer payloads and runtime session identity |
| `Differentiate.cinch` | Exact forward differentiation of scalar integer polynomials | A nonnumeric differentiation input | Tensors, shapes and reverse-mode elaboration |
| `TensorAD.cinch` | Shape-indexed matrix operations and reverse-mode pullbacks, checked against finite differences | Incompatible shapes and private representation access | Nonlinear/float contracts, broader elaboration and general proofs |
| `Reactive.cinch` | Generated state transitions combined with typed HTML rendering | Submitting an editing state | Live state storage, async event identity and browser UI |
| `Audio.cinch` | Executable signal pipeline with channel-changing stages | Connecting incompatible channel counts | Blocks, feedback, scheduling and checked callback budgets |
| `Circuit.cinch` | Clocked trace simulation and cycle-indexed operations | Combining signals from different cycles | Clock domains, synthesis and richer verification |

These slices are the first checkpoint of the broader experiment, not a claim
that the systems in the last column have been implemented.

## One descriptor, several consumers

`engine.py` interprets recursive, data-only grammar rules: literals, restricted
regular lexical patterns, captures, alternatives, repetitions, equality checks,
templates, fresh names and host-expression holes. Templates refer to
definition-site-qualified helpers via `${@helper}`. `${!name}` supplies a
collision-free temporary. Explicit captured names can bind user code.
Finite template iteration/filtering, sequence bindings, cardinalities and
conditional output support declaration projection without host-language plug-ins.

The source keeps regular runtime imports separate from syntax imports. Syntax
imports are blanked while preserving positions; their hashes and expansion
traces are included in the origin manifest. Generated code then passes through
the ordinary Cinch checker and backend. The experimental reader respects host
strings, characters, tick names, nested comments and delimiters inside a hole.
Imported source libraries take this same expansion path, in dependency order;
unused libraries are excluded from the project. Their origin maps are used by
both CLI diagnostics and editor navigation.

`editor_generate.py` reads the same grammar descriptors to produce Tree-sitter
grammars, highlighting queries and injections. It composes the existing Cinch
grammar with explicit syntax imports and declaration islands. A library can
declare a `text-tag-code` boundary reader for raw markup; the actual markup
grammar remains in the library descriptor. Imported aliases choose the
injected language. Combined injections parse a sequence of disconnected islands
for each library, and each host-code hole injects the actual Cinch expression
grammar again. The verification suite checks whole original files and actual
highlight output, not just isolated snippets.

`editor_lsp.py` runs the actual Cinch language server over framed JSON-RPC.
Expanded documents live in a temporary shadow workspace. Diagnostics,
definition locations and semantic tokens map back to original source; requests
map in the opposite direction. The adapter converts between native scalar
columns and client UTF-16 columns, and avoids painting generated helper tokens
over structural highlighting. Unfinished markup produces a syntax diagnostic
and recovers when completed.

Run the adapter with:

```sh
python3 experiments/malleable-syntax/poc.py lsp
```

It uses full-document synchronization. No editor settings are installed or
changed automatically. Generated grammars are in `editor/tree-sitter-*` after
verification; `editor/README.md` describes their registration.

## Compiler findings

The experiments exposed three compiler issues and one existing highlight-query
issue. Their fixes are local to this worktree:

1. A strict alias could lose an argument's `%1`/`%aff` restriction. Direct
   aliases now transfer the restriction; tests cover duplicate use, dropping,
   a chain of aliases, and branch-sensitive consumption.
2. Diagnostic LSP notifications lacked `textDocument/publishDiagnostics` in
   their `method` field. The actual protocol test now checks it.
3. Constant folding attempted to lower an unhoisted closed callback in a
   collection literal. Such constructions now remain runtime constructions;
   closures have no supported constant reification in this phase.
4. The Zed query for a qualified value assumed a constructor was an immediate
   child; the grammar has a `constructor_path`. The query now follows that tree.

## Boundaries to carry into the next iteration

- This is a source front-end adapter. Integration with native parsing, staging
  dependency identity, interfaces, provenance and incremental invalidation is
  still required before language adoption.
- The demonstrated hygiene discipline is qualified helpers plus explicitly
  fresh introduced binders. Arbitrary authored templates are not automatically
  analyzed for all binding forms. A production API needs proper scoped syntax
  objects rather than treating a string template as a sound hygiene boundary.
- Syntax imports currently require one module per source. Wider sources fail
  explicitly instead of silently widening a grammar's scope. A project editor
  composition also refuses conflicting aliases instead of choosing one.
- Regexes use a restricted regular subset; matching, nesting, input size and
  output size have explicit experimental limits. This is not a termination
  proof for an unrestricted macro language.
- The external reader conservatively refuses syntax expansion in a source
  containing a no-macros/JPL marker. Native expansion provenance is needed for
  correct per-definition enforcement.
- Hover/navigation inherit the native server's current symbol index. Local
  parameter hover, syntax-aware property completion in unfinished input,
  formatting, rename and richer recovery need additional work.
- Parser recovery and source-map behavior are tested for these probes. A
  production grammar exporter needs a specified compatible subset and
  differential tests for more lexical and layout combinations.

`PLAN.md` preserves the acceptance criteria. `PROGRESS.md` records what remains
of the original experiment so a continuation does not mistake this checkpoint
for completion.
