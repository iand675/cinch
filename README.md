# Designing a Safe Language

Complete documentation export — 2026-10-22T18:59:36Z

This archive contains the current overview and **all 26 pages linked from it**: nine specifications, fourteen tutorials, and three example, design-decision, and reference pages. It preserves the complete text and code, not summaries. The original pages were not modified.

## Start here

- Read the rendered documentation as a site: `cd website && npm install && npm run dev`, then open <http://localhost:4321>. `npm run build` writes a static copy to `website/dist/`. See [website/README.md](website/README.md).
- Read [the overview](overview.md) in any Markdown editor.
- Browse [the example catalogue](examples/catalogue.md) and the individual sources below.

The Astro site in `website/` is the rendering of every page: its loader
turns each Markdown file in the repository (overview, specifications,
tutorials, design, examples, context) into a page. The original
documentation export's pre-rendered `*.html` pages were removed on
2026-10-25 in favor of that rendering; `manifest.json` and `SHA256SUMS`
remain as the record of what the export contained. The language was
renamed to Cinch on 2026-10-27, so the recorded file names follow the
new `.cinch`/`.cinchif` spelling while the recorded hash values still
pin the exported bytes.

## Export contents

- **27 documentation pages**, in Markdown (the export's standalone HTML
  renderings were removed on 2026-10-25; see above).
- **12 `.cinch` source files and one WIT interface**, extracted verbatim from the current example catalogue. The final runtime-effects filename is an export label for its named code block.
- Historical [design conversation](context/design-conversation.md), with 25 visible messages; no message content was omitted by the reader.
- `source-data/pages.json` retains the exact original Markdown and ordered block text, plus source URLs, page IDs, update times, and revision numbers.
- `manifest.json` maps pages and example files to their sources. `SHA256SUMS` verifies every other archive file.

## Fidelity and status

Markdown files add a title, source link, and export timestamp, and convert links between included pages to local relative links. Source bodies and code remain unchanged otherwise; exact originals are in `source-data/pages.json`. HTML is a convenience rendering of the same content. Page comments, private back-of-page notes, external websites, and historical sandbox attachments are outside this export.

This is draft language-design documentation, not an implemented or compiler-tested language. Provisional syntax, API sketches, proof obligations, expected rejection cases, and open decisions retain their original qualifications. No source code was executed or typechecked.

## Overview

- [Language design overview](overview.md) · [original page](https://chatgpt.com/space/page_b647a1112d5c8191a20ef25f039a366a)

## Specifications

- [Core language specification](specifications/core-language-specification.md) · [original page](https://chatgpt.com/space/page_1e97031528908191b713513863e30e6b)
- [Modules and compilation specification](specifications/modules-and-compilation-specification.md) · [original page](https://chatgpt.com/space/page_d64504b7cf08819185f30f677ededc93)
- [Runtime evolution specification](specifications/runtime-evolution-specification.md) · [original page](https://chatgpt.com/space/page_d92d79c6a7748191817f7baeb2489fe5)
- [Platform proofs and SIMD specification](specifications/platform-proofs-and-simd-specification.md) · [original page](https://chatgpt.com/space/page_b24cfef55f608191b1494b251d782fe4)
- [Foreign function interface specification](specifications/foreign-function-interface-specification.md) · [original page](https://chatgpt.com/space/page_4f4effb6396c8191a2b8bf41e791ca09)
- [WebAssembly and WASI specification](specifications/webassembly-and-wasi-specification.md) · [original page](https://chatgpt.com/space/page_93721e2e8a6081919c8e74d0c3f11211)
- [Concurrency specification](specifications/concurrency-specification.md) · [original page](https://chatgpt.com/space/page_25f65d18f4508191ada994923cd97c35)
- [Surface syntax and grammar](specifications/surface-syntax-and-grammar.md) · [original page](https://chatgpt.com/space/page_18d4c95146a48191a722223f1e8dd18a)
- [Runtime capabilities, effects and sequencing](specifications/runtime-capabilities-effects-and-sequencing.md) · [original page](https://chatgpt.com/space/page_f7f7e6466b348191803f2f900098b667)

## Tutorials

- [Tutorial 1 Values and dependent types](tutorials/01-values-and-dependent-types.md) · [original page](https://chatgpt.com/space/page_7e6a395e629c819195dc11bcf37cf789)
- [Tutorial 2 Proofs and runtime checks](tutorials/02-proofs-and-runtime-checks.md) · [original page](https://chatgpt.com/space/page_811f5c451ea481919981aba5cd276baf)
- [Tutorial 3 Ownership and effects](tutorials/03-ownership-and-effects.md) · [original page](https://chatgpt.com/space/page_564c957a4e5c8191b7166e65d755285c)
- [Tutorial 4 Modules and laws](tutorials/04-modules-and-laws.md) · [original page](https://chatgpt.com/space/page_c16fa5ea6e148191831f925724d373f2)
- [Tutorial 5 Streams and coinduction](tutorials/05-streams-and-coinduction.md) · [original page](https://chatgpt.com/space/page_7af778a2dd6881919c77d929cd4d8cb8)
- [Tutorial 6 Processes and temporal contracts](tutorials/06-processes-and-temporal-contracts.md) · [original page](https://chatgpt.com/space/page_2d2960d3ed2c8191a601fdeb3e741de8)
- [Tutorial 7 Rewrites and stream fusion](tutorials/07-rewrites-and-stream-fusion.md) · [original page](https://chatgpt.com/space/page_f812b35e00ec8191ae0252ac9dda7bc7)
- [Tutorial 8 Staging and schema generation](tutorials/08-staging-and-schema-generation.md) · [original page](https://chatgpt.com/space/page_668e270bca9c8191a1822f4548af4a0c)
- [Tutorial 9 Compatible hot upgrades](tutorials/09-compatible-hot-upgrades.md) · [original page](https://chatgpt.com/space/page_630a9348c72c8191b3e2cd5fd9f3153d)
- [Tutorial 10 Platform specific SIMD](tutorials/10-platform-specific-simd.md) · [original page](https://chatgpt.com/space/page_cb25fa2f09f881918ae41fba88919548)
- [Tutorial 11 Foreign libraries](tutorials/11-foreign-libraries.md) · [original page](https://chatgpt.com/space/page_eb03321bfde48191bcd197291ad9eadf)
- [Tutorial 12 WebAssembly components](tutorials/12-webassembly-components.md) · [original page](https://chatgpt.com/space/page_f0fa8d6024188191862d6256eb506b72)
- [Tutorial 13 Structured concurrency](tutorials/13-structured-concurrency.md) · [original page](https://chatgpt.com/space/page_1a35ce96cd3c819197921ef44d2d156e)
- [Tutorial 14 Haskell-like syntax, effects and runtime selection](tutorials/14-haskell-like-syntax-effects-and-runtime-selection.md) · [original page](https://chatgpt.com/space/page_edc8263b2390819188b488f4fbcf2372)

## Examples

- [Example catalogue](examples/catalogue.md) · [original page](https://chatgpt.com/space/page_ca01753c1a108191b5a81b2138f93a2e)

## Design decisions and references

- [Open decisions and implementation plan](design/open-decisions-and-implementation-plan.md) · [original page](https://chatgpt.com/space/page_a2ab477ad93881918c2b00c47fe9c608)
- [References and related work](design/references-and-related-work.md) · [original page](https://chatgpt.com/space/page_e5b11110b6408191b7b5389ab932a93f)

## Individual source examples

- [01-bounded-index.cinch](examples/source/01-bounded-index.cinch)
- [02-vector-filter.cinch](examples/source/02-vector-filter.cinch)
- [03-map-fusion.cinch](examples/source/03-map-fusion.cinch)
- [04-productive-stream.cinch](examples/source/04-productive-stream.cinch)
- [05-schema-generator.cinch](examples/source/05-schema-generator.cinch)
- [06-counter-upgrade.cinch](examples/source/06-counter-upgrade.cinch)
- [07-rejected-programs.cinch](examples/source/07-rejected-programs.cinch)
- [08-platform-simd.cinch](examples/source/08-platform-simd.cinch)
- [09-foreign-codec.cinch](examples/source/09-foreign-codec.cinch)
- [10-wasm-component.cinch](examples/source/10-wasm-component.cinch)
- [11-structured-tasks.cinch](examples/source/11-structured-tasks.cinch)
- [codec.wit](examples/source/codec.wit)
- [12-runtime-effects.cinch](examples/source/12-runtime-effects.cinch)

## Original collection

[Language design overview](https://chatgpt.com/space/page_b647a1112d5c8191a20ef25f039a366a)

## Added after the export

Pages authored locally in this repository after the 2026-10-22 export. They are not part of the export manifest and are not covered by `SHA256SUMS`.

- [Primitive types specification](specifications/primitive-types-specification.md) · draft 0.1: the unboxed machine scalars (`Bool`, `Char`, `I8`-`I64`, `U8`-`U64`, `F32`, `F64`, `Usize`, `Isize`), their explicit checked, wrapping and saturating operations, conversions, and their FFI, Wasm and SIMD correspondence.
- [Lazy computation and reclamation specification](specifications/lazy-computation-and-reclamation-specification.md) · draft 0.1: explicit laziness without a garbage collector. Affine thunks (`Thunk {e} a`, force consumes), region memo cells (`Lazy r a`, call-by-need within a region), and duplicable thunks (`%many`, recompute), with effects, erasure, concurrency and upgrade obligations.
- [Surface grammar local amendments](specifications/surface-grammar-local-amendments.md) · draft 0.1: the deltas between the draft 0.2 EBNF and what the implementation adopted (tuple types, `~` memo bindings, effect-row type arguments, record forms).
- [Compiled interface metadata](design/compiled-interface-metadata.md) · design note: how compiled modules share interface metadata (`.cinchif` artifacts, instance coherence, ABI symbol tables, assumption manifests, the Rust runtime link model).
- [Dependent core kernel design](design/dependent-core-kernel.md) · design note: universes, Pi/Sigma, conversion, indexed families, erasure and totality for the dependent core (P7).
- [Ownership, usage and regions design](design/ownership-and-usage.md) · design note: `%0`/`%1`/`%aff`/`%many` usage checking, `Share`, `Ref`/`Mut` regions and escape rules (P5).
- [Effects, capabilities and sequencing design](design/effects-capabilities-sequencing.md) · design note: handler calculus, `do` with sequencing modules, lexical capabilities and runtime identity (P6).
- [Codata and guardedness design](design/codata-guardedness.md) · design note: `codata`, `Later`, `corec` productivity and bisimulation (P8).
