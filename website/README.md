# Cinch documentation site

Astro + Starlight documentation for the Rust Cinch compiler. `/documentation/` is the reader entry point: `learn/` is the ordered language tutorial, `reference/` is the compiler user’s guide, and `library/` is generated from real checked library interfaces. Historical design specs and notes (`overview.md`, `specifications/`, `tutorials/`, `design/`, `examples/`, `context/`) remain in the repository and are not published on the website. Markdown stays in the repository and the site reads it directly.

## Commands

```sh
cd website
npm ci
npm run dev      # http://localhost:4321
npm run build    # static output in website/dist/
npm run preview  # serve the build
npm run check    # astro check (TypeScript and content types)
npm run verify:preview # audit registry-derived package routes, links, and download archives
```

The devenv shell provides Node 24 and npm from nixpkgs. Astro 7 requires Node 22.23 or newer; `website/.node-version` pins 24.21.0 for fnm and asdf when working outside devenv.

Enter `devenv shell` from the repository root to use the public npm registry and install the `prek` commit hook. The hook checks the staged `.npmrc` and lockfile. When updating dependencies outside devenv, pass `--registry=https://registry.npmjs.org/` to `npm install`.

## Visual design

Future site work starts with [AGENTS.md](AGENTS.md), [the design handbook](design/design-handbook.md), and [the inspiration atlas](design/inspiration-atlas.md). These preserve the design rules, the user's original images and source links, the reasoning behind the materials and ornament, and the directions that were tried and rejected. Exact attachment copies and their provenance manifest live in `design/references/`, outside the public site assets.

The landing page is `src/pages/index.astro`. It introduces the language and links directly into `/documentation/`, `/learn/`, `/reference/`, and `/packages/stdlib/`.

The landing page uses a cassette-futurist direction: large Momo Trust Display typography, full-width ink/ivory/orange fields, subtle static surface grain, sculpted section transitions, curved equipment-bay links, circular index plates, and an orange instrument faceplate with an inset olive display. The console includes a length gauge, casing seams, and fasteners; page details remain static and decorative. DM Sans handles reading text; IBM Plex Mono handles code and labels. The vector-length control in `src/components/ContractConsole.astro` illustrates the nonempty-input contract for `first`; it does not execute a compiler. The mascot is retained as an asset but is not displayed on the landing page. The documentation theme pairs ink, warm ivory, and ember orange. Fonts are self-hosted in `public/fonts/`; the supplied wordmark and mascot are optimized WebP assets in `public/brand/`. Shared documentation styles are in `src/styles/custom.css`; the landing page composition is in `src/styles/landing.css`. Reading pages support light, dark, and system themes.

## How the pages are produced

`src/loaders/safelang-docs.ts` is a custom Astro content loader. For every Markdown file in the directories listed in `DOC_DIRS` it:

- takes the first `# Heading` as the page title and removes it from the body, because Starlight renders the title itself;
- lifts `Source:`, `Exported:` and `Added:` lines into page metadata, rendered under the title by `src/components/PageTitle.astro`;
- demotes every remaining heading one level, so numbered `# N` sections become `h2` and appear in the table of contents;
- rewrites links to sibling Markdown files into site routes, so `](04-types-and-kinds.md)` and `](../documentation.md)` both work;
- maps fence languages the highlighter does not know (`ebnf`) to plain text;
- renders the Markdown with the site's Markdown pipeline.

Fenced code blocks are never rewritten. Adding a chapter means adding the `.md` file: the loader gives it a page and `astro.config.mjs` builds the sidebar from the same directories and headings, so nothing else needs editing.

## Deployment

`npm run build` produces a static site in `dist/`. Publish that directory anywhere that serves files; search is indexed at build time by Pagefind, so no server is required.

The public origin is `https://cinch.systems`. Vercel hosts the `cinch` project with
`website` as its root directory, Node 24, `npm ci`, `npm run build`, and `dist` as
the output directory. Pushes to `deploy/cinch-site` create production deployments.
The lockfile uses the public npm registry so builds need no private network access.
Include source files outside the root directory: the docs
loader and navigation read Markdown from the repository's parent directories.
Cloudflare remains the authoritative DNS provider; web records point to the
project-specific DNS target returned by Vercel with proxying disabled.

## Live Life demo

The landing page ships a precompiled Cinch board and loads the seed compiler
in a worker on the first edit. The entire `life.cinch` module is editable.
The compiler checks and specializes the cell-age rule, neighbour counting,
edge behaviour, palette, generation interval, and grid dimensions. The Cinch
runtime activates those settings between generations. When the grid resizes,
cells at coordinates that still fit keep their ages, newly added cells start
dead, and the generation stays the same. The fixed WebAssembly bridge lives in
`life-abi.cinch`.
The example is based on [Blaze’s Life demo](https://github.com/indexable-inc/blaze#demo-a-game-of-life-you-can-edit).

After changing the demo's Cinch source, compiler, or runtime, regenerate its
checked-in Wasm assets with `bash website/scripts/build-life-wasm.sh`. Set
`WASI_SDK_PATH` for the worker build and use Clang 20 or newer and `wasm-ld`
for the board (`CINCH_CLANG` and `CINCH_WASM_LD` can point to those tools).
The script needs Rust's `wasm32-wasip1` and `wasm32-unknown-unknown` targets.
Run `npm run verify:life` from `website/` to check both assets together.

## Hero type study

`/type-study/` compares Archivo, Newsreader, Grenze, Domaine Sans Display, and Founders Grotesk. The first three use self-hosted SIL OFL fonts in `public/fonts/study/` with their licences. The paid specimens are screenshots of the supplied headline entered into Klim’s official type testers; no commercial font binaries are included.

The development server supports `/?hero=archivo`, `newsreader`, `grenze`, `domaine`, or `founders` for full hero comparisons. This selection is disabled in production, and the default hero uses Momo Trust Display. The study is excluded from Pagefind and marked noindex. Paid samples need properly licensed webfonts before becoming live text.

The two paid families also have full landing-page mockups at `/?typepage=domaine` and `/?typepage=founders`. Section titles are rendered from official foundry specimen captures, applied as luminance masks to preserve the site’s colors. These are development-only comparison views, with readable heading labels and no commercial font files. `src/premium-heading-specimens.json` records the source rendering dimensions.

Free static alternatives are compared at `/domaine-alternatives/`. The variable follow-up is `/variable-type-study/`, with actual Russolo, Bodoni Moda, and Playfair variable font files and OFL licences. The controls change real font axes; full-page links preserve those settings in the query string. `src/variable-font-axes.json` records the ranges read from the downloaded font files.

The user-requested Noto Serif, Doto, and Momo Trust Display renders are at `/requested-type-study/`, with full-page links at `/?freepage=notoserif`, `/?freepage=doto`, and `/?freepage=momotrustdisplay`. Noto Serif and Doto use their actual variable fonts; Momo Trust Display uses its static Regular cut. All are self-hosted with OFL licences.

## Selected typography

Momo Trust Display is the production display face for the landing page and documentation headings. DM Sans remains the reading and UI face; IBM Plex Mono remains the code and metadata face. The chosen static Regular font and its OFL licence are in `public/fonts/momo-trust-display.*` and `public/fonts/momo-trust-display-LICENSE.txt`. The default `/` route needs no comparison query parameters.

The selected `/?freepage=momotrustdisplay` URL now shows the default landing page without the old comparison bar. Other development font comparisons remain available.

## Manual and instrument details

The docs use `src/styles/docs-material.css`: a dark navigation spine, warm chapter plates, numbered section rules, curved code surrounds, and light/dark reading palettes. `src/styles/scrollbars.css` styles page, navigation, and code scrollbars without replacing native scrolling.

Documentation code uses the custom Cinch Instrument / Cinch Paper Expressive Code themes in `src/themes/syntax.mjs`, with warm orange, amber, ivory, and olive token colours. Both follow the docs theme selector. The loader includes theme/config/grammar inputs in its render digest to regenerate token colours and hashed code assets when the highlighting changes.

`TechnicalFigure.astro` is an original, labelled vector diagram of the nonempty-vector contract. It appears on the landing page and after that example in Tutorial 1 through the MarkdownContent override. The source Markdown and generated heading anchors are preserved.

The selected wordmark is `public/brand/cinch-wordmark-instrument-v2.webp`. It retains the original lettering and loops with an orange fill and a fine ivory keyline for small-format readability. The opaque black source uses screen blending on the dark header/footer. The original gold asset remains available. See `design/logo-generation.md` for the built-in image-generation prompt and source asset.

## Package and release prototypes

`/packages/` is a static catalogue generated at build time from public releases in a Cinch registry. Search covers package names, modules, paths, and descriptions; category and result-type filters are reflected in the URL. Package pages show the published README, declared dependencies, version history, module list, release identity, and a registry-pinned install command. Module pages show the manifest name and path, the package identity, declared dependencies, and the published source. Registry releases do not carry declaration signatures, so module pages do not invent them: they state that generated `.cinchdoc` data is required for declaration-level documentation.

`src/lib/catalogue.ts` reads `GET /api/v1/index`, then each release's metadata and bundle. Astro builds the site statically, so all registry requests happen once during `npm run build`; the published site makes no registry request from a browser.

| Variable | Effect |
| --- | --- |
| `CINCH_CATALOGUE_REGISTRY` | Base URL of the registry. Defaults to `http://127.0.0.1:8787`, the local prototype. |
| `CINCH_CATALOGUE_OFFLINE=1` | Build a minimal catalogue with no package data. The page says the build is offline. |

When the registry is unreachable, empty, or a release cannot be read, the build uses `src/data/catalogue-snapshot.json` as a last-synced snapshot. With no snapshot it builds a cached-empty catalogue with a warning, so a Vercel build does not fail when the registry is down. It never falls back to sample libraries. `/packages/index.json` exposes the same model with `sample: false`, `fallback`, the registry URL, and the build time.

Refresh the committed snapshot after a successful live build:

```sh
CINCH_CATALOGUE_REGISTRY=https://registry.example npm run build
npm run catalogue:snapshot
```

Run that refresh periodically, or point a scheduled CI job at the same commands. Vercel's static build then has a real last-known catalogue when the registry is unavailable.

`.github/workflows/catalogue-refresh.yml` triggers the Vercel production deploy hook daily when the `VERCEL_DEPLOY_HOOK_URL` repository secret is set. Each build fetches the registry live. A failed fetch keeps the site deployable through the cached fallback.

`node scripts/verify-catalogue.mjs` starts `registry/server.mjs` with a temporary data directory, publishes `fixture/hello@1.2.3`, builds the site against it, checks the package, module, and source pages, and runs `verify-preview.py`.

Package, module, module-source, and download pages share `PlatformLayout.astro`. Their Light / Dark / System selector uses the same `starlight-theme` preference as the docs, initializes before first paint, and follows system changes in System mode. `src/styles/platform-theme.css` defines the warm paper and dark olive palettes; the header, footer, and instrument wells retain their dark surfaces in both modes. Native controls and scrollbars follow their surrounding surfaces.

`/downloads/` shows real nightly compiler distributions from GitHub
prereleases. `src/lib/downloads.ts` selects the newest complete nightly
with all three native executables, SHA-256 digests and checksum sidecars.
The static build includes the current release when GitHub is reachable;
the browser refreshes the public release list on load, so new nightlies
appear without redeploying. Failed refreshes preserve the build snapshot
and explain that it may be stale. Before the first successful nightly,
the page links to source and releases without offering fake downloads.

The supported targets match `.github/workflows/ladder-cache.yml`: Linux
x86-64, Linux ARM64 and Apple Silicon macOS. These are standalone Stage 3 binaries; running the compiler requires no
Nix installation. Linux binaries are statically linked and macOS uses system libraries. The page
keeps platform, revision, filename, size, digest and download URL aligned.
Sample ZIP routes have been removed.

After `npm run build`, run `npm run verify:preview`. It checks the package
routes and shared navigation along with the nightly page's install and
release wiring. `npm run verify:downloads` exercises release selection,
checksums, incomplete publications and repository URL boundaries.



## Reader documentation and verification

The Rust implementation is the current behaviour authority. The tutorial and guide mark specified but unavailable behaviour with visible **Coming soon** tags and distinguish partial support and API-only mechanisms. `/reference/16-support-and-roadmap/` summarizes current support and planned features across the compiler.

`learn/` has the tutorial contents and 22 chapters. `reference/` has the guide contents and 29 subject pages, including dedicated attributes, type layout, allocator, REPL, language server, documentation, and package management references. The tooling chapters distinguish available frontends from remaining intended work with support labels and explicit current implementation tables. Standard library API docs (`library/`, extracted from checked `.cinchif` interfaces) are published under the package catalogue as `stdlib`: overview at `/packages/stdlib/`, Hackage-style module pages at `/packages/stdlib/<Module>/`, line-numbered source views at `/packages/stdlib/<Module>/source/`, and Hoogle-style API search at `/packages/search/`. Hand-written pages use the editor TextMate grammar; generated library pages carry compiler tree-sitter token spans.

Run these checks with the Node version in `.node-version`:

```sh
npm run check
npm run build
npm run verify:preview
npm run verify:docs
npm run verify:examples -- --compiler /absolute/path/to/rust-built/cinch
```

`verify:docs` checks source drift plus built reader-page links/anchors, source downloads, Hoogle search index, excluded archive routes, status labels, and code rendering. `verify:examples` extracts marked complete examples from the tutorial/guide, checks them, builds and executes positive programs, checks exact exit/stdout expectations, and checks named negative diagnostics. It uses temporary projects and a private store. Explanatory fragments are unmarked; every complete module must have a marker. Multi-file examples use `cinch:project` markers. This is meaningful executable documentation coverage, not a claim that every compiler test or library operation was rerun.

Regenerate the library after changing source or public exports:

```sh
python3 scripts/generate-library-reference.py --compiler /absolute/path/to/rust-built/cinch
python3 scripts/generate-library-reference.py --check
```

Use a Rust executable built from the same source snapshot. The generator imports each non-embedded module from a temporary consumer project, reads checked `.cinchif` exports, excludes private signature closure, attaches source documentation, records per-file SHA-256 witnesses, and copies exact source downloads to `public/reference-sources/`. All checked sources remain in `seed/prelude/`; generated pages are reference outputs rather than a parallel editable tutorial. No remote package fetch is implied by a library import.
