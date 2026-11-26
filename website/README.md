# Cinch documentation site (`website/`)

Astro + Starlight documentation site for Cinch, published at [cinch.systems](https://cinch.systems). `/documentation/` is the reader entry point: `learn/` is the 22-chapter language tutorial, `reference/` is the 29-chapter compiler user's guide, and `library/` is generated from checked `.cinchif` interfaces and published under `/packages/stdlib/` with Hoogle-style API search. Design notes and normative specifications (`design/`, `specifications/`, `examples/`) remain in the repository and are not published as website routes.

## Commands

Run with Node 24 (`devenv shell` or `.node-version`):

```sh
cd website
npm ci
npm run dev              # http://localhost:4321
npm run build            # static output in website/dist/
npm run preview          # serve the built site
npm run check            # astro check (TypeScript and content schemas)
npm run verify:preview   # audit package routes, links, and nightly download wiring
npm run verify:docs      # audit built reader links, anchors, search index, and highlighting
npm run verify:downloads # test nightly release selection and checksum logic
npm run verify:examples -- --compiler /absolute/path/to/cinch
```

Enter `devenv shell` from the repository root to use the public npm registry and install the `prek` hook. Outside `devenv`, pass `--registry=https://registry.npmjs.org/` when updating dependencies.

## Visual design

Before making visual changes, read [`AGENTS.md`](AGENTS.md), [`design/design-handbook.md`](design/design-handbook.md), and [`design/inspiration-atlas.md`](design/inspiration-atlas.md). Reference imagery and provenance live in `design/references/` outside `public/`.

- **Landing page (`src/pages/index.astro`):** cassette-futurist composition using broad charcoal ink, warm ivory, and ember orange fields with static grain, circular index plates, curved equipment-bay links, an interactive contract console (`ContractConsole.astro`), and the live editable Game of Life demo (`LifeReload.astro`).
- **Typography:** self-hosted **Momo Trust Display** (display headings), **DM Sans** (body and UI), and **IBM Plex Mono** (code and labels) in `public/fonts/` with OFL licences. Historical dev-only type-study routes (`/type-study/`, `/domaine-alternatives/`, `/variable-type-study/`, `/requested-type-study/`) are excluded from Pagefind and marked `noindex`.
- **Wordmark:** `public/brand/cinch-wordmark-instrument-v2.webp` (orange fill with fine ivory keyline; see `design/logo-generation.md`).
- **Theming & highlighting:** reading and platform pages share Light / Dark / System theme persistence (`starlight-theme`), styled via `src/styles/docs-material.css`, `src/styles/platform-theme.css`, `src/styles/scrollbars.css`, and the custom Cinch Instrument / Cinch Paper Expressive Code themes (`src/themes/syntax.mjs`).

## How docs pages are built

`src/loaders/safelang-docs.ts` loads `documentation.md`, `learn/*.md`, and `reference/*.md` directly from the repository root:

- extracts the first `# Heading` as the Starlight page title and demotes remaining headings by one level;
- lifts `Source:`, `Exported:`, and `Added:` headers into metadata chrome;
- rewrites sibling `.md` links to site routes and `library/*.md` links to `/packages/stdlib/<Module>/`;
- aliases `ebnf` code fences to plain text while leaving fenced code untouched;
- includes Astro config, syntax themes, and the Cinch TextMate grammar in the render cache digest.

Regenerate standard library reference pages after changing `seed/prelude/` exports:

```sh
python3 scripts/generate-library-reference.py --compiler /absolute/path/to/cinch
python3 scripts/generate-library-reference.py --check
```

## Live Life demo

The landing page ships a precompiled Cinch board (`src/data/examples/life-board.wasm`) and loads the Rust seed compiler in a Web Worker (`src/data/examples/life-compiler.wasm`) on first edit so users can edit `life.cinch` live (adapted from [Blaze's Life demo](https://github.com/indexable-inc/blaze#demo-a-game-of-life-you-can-edit)).

Rebuild the checked-in Wasm assets after changing the demo, compiler, or runtime:

```sh
WASI_SDK_PATH=/path/to/wasi-sdk bash website/scripts/build-life-wasm.sh
cd website && npm run verify:life
```

Requires Clang 20+, `wasm-ld`, and Rust's `wasm32-wasip1` and `wasm32-unknown-unknown` targets (`CINCH_CLANG` and `CINCH_WASM_LD` can override tool paths).

## Packages catalogue and nightly downloads

- **`/packages/`:** statically generated at build time from `CINCH_CATALOGUE_REGISTRY` (default `http://127.0.0.1:8787`; pass `CINCH_CATALOGUE_OFFLINE=1` for an explicit offline build). Falls back to `src/data/catalogue-snapshot.json` when the registry is unreachable (refresh with `npm run catalogue:snapshot` after a live build; test end-to-end with `node scripts/verify-catalogue.mjs`). `.github/workflows/catalogue-refresh.yml` triggers the Vercel deploy hook daily.
- **`/downloads/`:** reads complete GitHub nightly prereleases (Linux x86-64, Linux ARM64, macOS Apple Silicon Stage 3 standalone binaries with SHA-256 sidecars) at build time and refreshes from GitHub in the browser on page load.

## Deployment

`npm run build` outputs a static site with Pagefind search to `website/dist/`. Vercel builds `website/` on pushes to `main`, using `scripts/vercel-ignore.sh` (tested via `bash website/scripts/test-vercel-ignore.sh`) to skip builds when no site source, published doc, stdlib source, or TextMate grammar changed.
