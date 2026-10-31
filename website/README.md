# Cinch documentation site

Astro + Starlight rendering of the design documentation. The Markdown chapters stay where they are (`overview.md`, `specifications/`, `tutorials/`, `design/`, `examples/`, `context/`); this site reads them from disk and publishes them as one book. There is no copy step and no generated HTML twin to keep in sync.

## Commands

```sh
cd website
npm install
npm run dev      # http://localhost:4321
npm run build    # static output in website/dist/
npm run preview  # serve the build
npm run check    # astro check (TypeScript and content types)
npm run verify:preview # audit built package pages, links, sample ZIPs, and checksums
```

The devenv shell provides Node 24 and npm from nixpkgs. Astro 7 requires Node 22.23 or newer; `website/.node-version` pins 24.21.0 for fnm and asdf when working outside devenv.

## Visual design

The landing page is `src/pages/index.astro`. It introduces the language and links into the original documentation, which lives at `/overview/` and its existing chapter routes. Links to `overview.md` are rewritten to `/overview/`.

The landing page uses a cassette-futurist direction: large Momo Trust Display typography, full-width ink/ivory/orange fields, subtle static surface grain, sculpted section transitions, curved equipment-bay links, circular index plates, and an orange instrument faceplate with an inset olive display. The console includes a length gauge, casing seams, and fasteners; page details remain static and decorative. DM Sans handles reading text; IBM Plex Mono handles code and labels. The vector-length control in `src/components/ContractConsole.astro` illustrates the nonempty-input contract for `first`; it does not execute a compiler. The mascot is retained as an asset but is not displayed on the landing page. The documentation theme pairs ink, warm ivory, and ember orange. Fonts are self-hosted in `public/fonts/`; the supplied wordmark and mascot are optimized WebP assets in `public/brand/`. Shared documentation styles are in `src/styles/custom.css`; the landing page composition is in `src/styles/landing.css`. Reading pages support light, dark, and system themes.

## How the pages are produced

`src/loaders/safelang-docs.ts` is a custom Astro content loader. For every Markdown file in the directories listed in `DOC_DIRS` it:

- takes the first `# Heading` as the page title and removes it from the body, because Starlight renders the title itself;
- lifts `Source:`, `Exported:` and `Added:` lines into page metadata, rendered under the title by `src/components/PageTitle.astro`;
- demotes every remaining heading one level, so numbered `# N` sections become `h2` and appear in the table of contents;
- rewrites links to sibling Markdown files into site routes, so `](core-language-specification.md)` and `](../overview.md)` both work;
- maps fence languages the highlighter does not know (`ebnf`) to plain text;
- renders the Markdown with the site's Markdown pipeline.

Fenced code blocks are never rewritten. Adding a chapter means adding the `.md` file: the loader gives it a page and `astro.config.mjs` builds the sidebar from the same directories and headings, so nothing else needs editing.

## Deployment

`npm run build` produces a static site in `dist/`. Publish that directory anywhere that serves files; search is indexed at build time by Pagefind, so no server is required.

The public origin is `https://cinch.systems`. Vercel hosts the `cinch` project with
`website` as its root directory, Node 24, `npm ci`, `npm run build`, and `dist` as
the output directory. Include source files outside the root directory: the docs
loader and navigation read Markdown from the repository's parent directories.
Cloudflare remains the authoritative DNS provider; web records point to the
project-specific DNS target returned by Vercel with proxying disabled.

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

`TechnicalFigure.astro` is an original, labelled vector diagram of the nonempty-vector contract. It appears on the landing page and after that example in Tutorial 1 through the MarkdownContent override. The source Markdown and generated heading anchors are preserved.

The selected wordmark is `public/brand/cinch-wordmark-instrument-v2.webp`. It retains the original lettering and loops with an orange fill and a fine ivory keyline for small-format readability. The opaque black source uses screen blending on the dark header/footer. The original gold asset remains available. See `design/logo-generation.md` for the built-in image-generation prompt and source asset.

## Package and release prototypes

`/packages/` is a static catalogue with eight sample libraries. Search covers package names, modules, declarations, signatures, and descriptions; category and result-type filters are reflected in the URL. Package pages include README text, dependencies, version history, and modules. Module references include linked types, constructor documentation, examples, declaration permalinks, and a sample source listing. Version selection filters declarations by their `since` version.

The fixture model lives in `src/lib/catalogue.ts`; editing it regenerates the versioned package, module, and declaration-source routes. `/packages/index.json` exposes the same sample data. These pages are marked as sample data and excluded from search-engine indexing. They do not publish packages or extract documentation from the compiler.

Package, module, declaration-source, and download pages share `PlatformLayout.astro`. Their Light / Dark / System selector uses the same `starlight-theme` preference as the docs, initializes before first paint, and follows system changes in System mode. `src/styles/platform-theme.css` defines the warm paper and dark olive palettes; the header, footer, and instrument wells retain their dark surfaces in both modes. Native controls and scrollbars follow their surrounding surfaces.

`/downloads/` demonstrates release-channel and platform selection. `src/lib/downloads.ts` generates eight deterministic sample ZIPs, each containing only a README and release metadata, plus matching SHA-256 sidecars. No compiler is included. Repository and source-download links use the real GitHub project configured in `src/lib/project.ts`. Replace the fixtures with real release metadata and artifacts when those become available.

After `npm run build`, run `npm run verify:preview` to check generated links and anchors, version-specific declarations, ZIP contents, checksums, and shared project navigation.
