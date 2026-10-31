# Cinch website: instructions for future agents

These instructions apply to `website/`. This site has an established, user-selected design. Extend it deliberately; do not replace it with a new visual direction as a routine implementation choice. A new explicit user request can change the direction. The guidelines below do not introduce an approval requirement for ordinary work.

## Read before designing

1. Read [the design handbook](design/design-handbook.md) for composition, typography, materials, ornament, page patterns, and review criteria.
2. Read [the inspiration atlas and decision history](design/inspiration-atlas.md). Open the relevant reference images, rather than interpreting the phrase “cassette futurism” from memory.
3. Inspect the current implementation and the page being changed. Screenshots record stages of the design; they are not exact specifications of today's navigation, content, or behaviour.
4. Read [README.md](README.md) for commands and site architecture. Check the current repository state before editing: compiler, deployment, and other site work may be happening concurrently.

## Design commitments

- Preserve the bold typographic composition: very large headings, broad ink/ivory/orange fields, generous space, and a clear reading order. Do not shrink the page into a dashboard of little rounded cards.
- Use **Momo Trust Display Regular** for display headings, **DM Sans** for reading text and UI, and **IBM Plex Mono** for code, technical labels, indices, and dimensions. Momo was explicitly selected after many full-page comparisons. Do not substitute a fashionable technical-site font or synthesize extra weights.
- Think of manufactured space-age plastic, paper manuals, and scientific instruments. The reference language includes rounded housings, spherical/circular forms, large curved shoulders, inset wells, fitted seams, restrained fasteners, and fine printed technical detail.
- Work that language into the **page composition**, not only a decorative computer next to an otherwise generic landing page. Existing section transitions, curved navigation ends, title plates, index rails, code surrounds, and native scrollbars demonstrate the distribution.
- Keep technical ornament purposeful and subordinate. A label should identify something; a gauge should represent something; a leader line should point to something. Avoid invented coordinates, serial numbers, telemetry, controls, or “system online” messages added only to look technical.
- Use the existing static grain asset sparingly. Texture should become apparent on closer inspection. Preserve crisp text, clean controls, and calm reading surfaces. Avoid page-wide CRT effects, moving noise, flicker, fake damage, and continuous motion.
- Keep the material palette warm: charcoal ink, cream paper, orange, amber, and olive. The direction does not require red controls, black/green terminal styling, chrome, neon, or a literal cockpit everywhere.
- Use the selected instrument wordmark through `Brand.astro`: orange strokes with a **fine ivory keyline**, connected script, and wide looping silhouette. Thick cream edges were rejected for small-format readability. The mascot is retained as an asset and is currently absent from the production landing page.
- Write concrete, useful copy about Cinch. The user rejected trite decorative copy such as “a field guide, not a black box.” Avoid slogans that explain the aesthetic, generic AI marketing language, fake precision, and repetitive rhetorical contrasts. The conversational tone of an agent's replies is not the site's editorial voice.
- Preserve Light / Dark / System behaviour on docs and platform pages. Theme choice must carry between them. The landing page currently has its own fixed, alternating material composition; do not mistake that for a missing reading-page theme implementation.

## Where changes belong

| Concern | Owning files |
| --- | --- |
| Landing composition and material treatments | `src/pages/index.astro`, `src/styles/landing.css` |
| Shared fonts, base docs typography, brand | `src/styles/custom.css`, `src/components/Brand.astro` |
| Starlight reading materials and navigation | `src/styles/docs-material.css`, `src/components/Header.astro`, `src/components/Sidebar.astro`, `src/components/PageTitle.astro` |
| Platform page shell | `src/layouts/PlatformLayout.astro`, `src/components/SiteNavigation.astro` |
| Package/download layout and themed colours | `src/styles/platform.css`, `src/styles/platform-theme.css` |
| Theme preference and initial paint | `src/components/ThemeInit.astro`, `src/components/ThemeControl.astro`; Starlight's own selector on docs |
| Native scrollbar materials | `src/styles/scrollbars.css` |
| Docs code palettes and render-cache identity | `src/themes/syntax.mjs`, `astro.config.mjs`, `src/loaders/safelang-docs.ts` |
| Interactive type illustration | `src/components/ContractConsole.astro` and its rules in `landing.css` |
| Manual-style technical figure | `src/components/TechnicalFigure.astro`, `src/styles/technical-figure.css` |
| Docs content and route adaptation | `src/loaders/safelang-docs.ts`, `src/components/MarkdownContent.astro`, `astro.config.mjs` |
| Package fixtures and generated API references | `src/lib/catalogue.ts`, `src/pages/packages/`, `src/components/catalogue/` |
| Sample releases and downloads | `src/lib/downloads.ts`, `src/pages/downloads/` |
| Repository and source links | `src/lib/project.ts` |

Reuse the owning layout/component before duplicating it. Prefer semantic colour variables over additional literal colours. Keep selectors scoped to `.cinch-landing`, `.cinch-platform`, or the relevant Starlight context; the three page families share assets but have different composition requirements. Do not spread one-off fixes across unrelated stylesheets.

## Content and interaction constraints

- Preserve the source Markdown, heading anchors, existing documentation routes, and navigation. The loader reads documentation outside `website/`; do not create parallel copies of the book or change technical examples merely to fit a layout.
- The console illustrates a type relationship; it does not run a compiler. Its controls must keep the length, visual cells, equation, explanation, and accessible state in agreement.
- Packages, versions, API signatures, and release bundles currently include **explicitly labelled sample data**. Keep those labels and indexing rules until real data replaces the fixtures. A sample ZIP is not a compiler distribution. Do not invent compatibility, adoption, benchmark, security, or release claims.
- Keep real controls native and keyboard-operable where practical. Preserve skip links, visible focus, meaningful link names, selectable code, figure descriptions, and local overflow for long signatures. Decorative SVG/CSS belongs outside the accessibility tree; informative diagrams need a description and caption.
- Keep font licences. Paid Domaine/Founders captures are historical design specimens, not licensed production webfonts. Archived reference imagery lives outside `public/`; use it to understand the design, not as unreviewed page artwork.

## Working and verification

1. Identify the page family and the closest existing example. Read the relevant handbook recipe and reference entry.
2. Make the change in the owning component/style. Keep an obvious content hierarchy before adding ornament. Prefer a small number of well-integrated details.
3. Run the checks appropriate to the change. For implementation changes, the usual sequence from `website/` is `npm run check`, `npm run build`, then `npm run verify:preview`. Use Node from `.node-version` or the project's devenv; an older system Node may fail.
4. For visual changes, inspect the actual browser at a desktop size and around 390px wide. Inspect the full affected page, not only the hero. For themed pages, inspect both modes, a mode switch, reload persistence, and navigation to/from docs. Check long names, code overflow, mobile navigation, focus, and scrollbars.
5. For interaction changes, follow the real path: search → package → module → declaration/type/source; or channel/platform → matching artifact/checksum. A screenshot alone does not verify behaviour.
6. Compare against the reference images and current site. Ask whether typography still leads, the page still feels manufactured, and the content is still easier to use than the decoration is to notice.
7. Save representative screenshots for visual work and report what changed and what was checked. Do not overwrite historical screenshots merely to make them look current. Update these notes when the user deliberately changes a design commitment.

Documentation-only edits do not require rebuilding the site. Check their local links, file paths, and factual accuracy instead. Keep unrelated changes intact; do not stage or commit them with site work. Publishing, deployment, and Git actions follow the user's current request, not the presence of deployment configuration.
