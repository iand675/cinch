# Documentation site

Astro + Starlight rendering of the design documentation. The Markdown chapters stay where they are (`overview.md`, `specifications/`, `tutorials/`, `design/`, `examples/`, `context/`); this site reads them from disk and publishes them as one book. There is no copy step and no generated HTML twin to keep in sync.

## Commands

```sh
cd website
npm install
npm run dev      # http://localhost:4321
npm run build    # static output in website/dist/
npm run preview  # serve the build
npm run check    # astro check (TypeScript and content types)
```

The devenv shell provides Node 24 and npm from nixpkgs. Astro 7 requires Node 22.23 or newer; `website/.node-version` pins 24.21.0 for fnm and asdf when working outside devenv.

## How the pages are produced

`src/loaders/safelang-docs.ts` is a custom Astro content loader. For every Markdown file in the directories listed in `DOC_DIRS` it:

- takes the first `# Heading` as the page title and removes it from the body, because Starlight renders the title itself;
- lifts `Source:`, `Exported:` and `Added:` lines into page metadata, rendered under the title by `src/components/PageTitle.astro`;
- demotes every remaining heading one level, so numbered `# N` sections become `h2` and appear in the table of contents;
- rewrites links to sibling Markdown files into site routes, so `](core-language-specification.md)` and `](../overview.md)` both work;
- maps fence languages the highlighter does not know (`ebnf`) to plain text;
- renders the Markdown with the site's Markdown pipeline.

Fenced code blocks are never rewritten. Adding a chapter means adding the `.md` file and, if it should appear in the navigation, an entry in the `sidebar` array in `astro.config.mjs`.

## Deployment

`npm run build` produces a static site in `dist/`. Publish that directory anywhere that serves files; search is indexed at build time by Pagefind, so no server is required.
