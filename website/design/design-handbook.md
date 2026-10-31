# Cinch design handbook

This is the implementation companion to [AGENTS.md](../AGENTS.md). The [inspiration atlas](inspiration-atlas.md) preserves the user's resources, original attachments, and decisions. Read the two together: a list of CSS colours cannot explain this design on its own.

## 1. The visual idea

Cinch combines the confidence of a large printed headline with the physical vocabulary of a scientific instrument and the clarity of a technical manual. Its space-age character comes from continuous shaped surfaces, warm materials, and precise assembly. Large areas of the page carry the idea before small details do.

The landing page should sell the language through concrete examples and readable explanation. Documentation should support sustained reading. The library index should help someone find a package; the API reference should help them follow a type. Each has a different task. Their relationship comes from shared type, colour, materials, edge shapes, and editorial discipline.

The central balance is **bold composition with quiet detail**. This was earned through several revisions. A softer, boxed treatment lost the impact the user liked. Adding more controls around the hero display did not satisfy the request to integrate the aesthetic throughout the page. More ornament is not automatically a closer match.

## 2. Composition and scale

### Large fields first

Begin with broad ink, paper, or orange sections. Establish one dominant heading and a useful next action. Use generous margins, substantial section spacing, and strong alignment. The landing page generally uses about 4.5% horizontal gutters on desktop; responsive rules adapt the composition. Match nearby sections rather than treating that value as an immutable global token.

The existing hero uses an asymmetric text/console grid. Later sections use wide prose-and-example relationships, horizontal feature rows, and shaped documentation bays. Repetition happens through rails, rules, typography, and materials. Avoid converting every subject into an independently floating card.

Scale is part of the identity. The hero heading currently uses `clamp(4.7rem, 9.7vw, 11rem)` with a tight `.95` line-height; landing section titles are substantially larger than body copy. These are reference values from `landing.css`, not instructions to give every page an eleven-rem headline. A module title must accommodate names such as `Net.Http.Client` without competing with the landing hero.

### Manufactured edges

Use asymmetry in edge treatment: a largely rectangular plate with one generous rounded shoulder, a continuous concave/convex section transition, a capsule end on an active navigation item, or a small radius where a panel meets its casing. Existing examples include:

- The large curved lower-right shoulder of platform banners.
- Starlight chapter title plates with a curved lower-right edge.
- The rounded end of the selected module or chapter in a dark navigation spine.
- Equipment-bay links shaped as a continuous row rather than unrelated tiles.
- Code and figure housings with restrained, unequal corner radii.

Circles belong in index plates, gauges, callouts, end caps, and a few navigation details. They refer to physical forms and assembly. Repeating decorative spheres behind every section would overwhelm that connection. Do not turn all corners into the same generic pill or attach a shadow to every surface.

## 3. Typography

| Role | Selected face | Treatment |
| --- | --- | --- |
| Hero and display headings | Momo Trust Display | Static Regular, no synthetic bold; substantial scale; tight tracking; deliberate wrapping |
| Prose, descriptions, controls | DM Sans | Readable sizes, measured line length, generous leading; actual available weights |
| Code, API signatures, labels, indices, captions within instruments | IBM Plex Mono | Precise alignment; modest tracking for short uppercase labels; normal case for code |

Use `--cinch-font-display` for the display family. Font definitions and shared roles live in `src/styles/custom.css`. Fonts are self-hosted with licences in `public/fonts/`. Typography studies remain available for context, but are not a menu of equally approved production options.

Momo's broad rounded forms connect to the plastic reference language. Keep enough room around them to read their shapes. Do not squeeze long headings into narrow cards, fake a variable weight, force line breaks that only work at one viewport, or use the display face for a long paragraph. Existing landing headings often use roughly `-.035em` tracking; small UI text should not inherit that treatment indiscriminately.

Body copy usually sits near `.9–1rem` with generous leading; documentation paragraphs use approximately `.97rem` and `1.9`. Technical microtype is a secondary detail. Existing tiny labels establish atmosphere, but new essential content must remain comfortably readable. Do not copy a decorative `.5rem` label style onto form instructions or an error message.

Code remains selectable text. Signatures can scroll within their own wells. Prefer meaningful wrapping for prose and long identifiers; do not shrink everything until it fits. A full-page font comparison must include headings, body copy, labels, and mobile wrapping: the original font decision was made on that basis.

## 4. Colour and themes

The palette is warm and material: ink charcoal, ivory paper, orange enamel, olive screens, amber printing, and brownish rules. Pure black/white, purple glows, cyan neon, and cold blue dashboard greys are poor default additions.

### Current platform tokens

`src/styles/platform-theme.css` owns these values. This table is a guide to roles; the stylesheet remains the executable source of truth.

| Token | Light | Dark | Use |
| --- | --- | --- | --- |
| `--platform-paper` | `#eee6d5` | `#20251c` | Main reading surface |
| `--platform-ink` | `#272b21` | `#eee6d5` | Primary text |
| `--platform-muted` | `#70644e` | `#b1b29b` | Supporting text and metadata |
| `--platform-rule` | `#bdb59e` | `#59614b` | Dividers and assembly lines |
| `--platform-panel` | `#ded4bc` | `#2c3325` | Inset/supporting reading panel |
| `--platform-control` | `#e2d7be` | `#303927` | Native fields and selectors |
| `--platform-accent` | `#98532d` | `#efad78` | Links and warm technical accents |
| `--platform-active` | `#dfcfad` | `#454e32` | Selected backgrounds |
| `--platform-olive` | `#58613f` | `#c0c995` | Type/module and olive text accents |

The landing palette additionally uses ink `#191a17`, ivory `#eee6d5`, and broad orange accents around `#ff8546`. The console casing uses a more subdued orange around `#e88448`. Do not collapse those material distinctions merely to use one orange everywhere.

Starlight uses `--cinch-doc-paper`, `--cinch-doc-panel`, `--cinch-doc-rule`, `--cinch-doc-muted`, and the `--sl-*` variables. Its reading sheet currently shifts from `#f2ecde` to `#20251c`, while the dark header/sidebar remain dark. Reuse those variables when changing docs; platform tokens are not automatically the right API for a Starlight component.

### Theme behaviour

- Both docs and platform pages use the `starlight-theme` storage key. Explicit values are `light` and `dark`; an empty/absent preference means system mode. The platform selector calls it **System** and represents it as `auto` in the UI.
- `ThemeInit.astro` applies the preference before first paint. `ThemeControl.astro` handles platform selection, system changes, storage events, and page restoration. Docs use Starlight's provider/selector.
- Use `data-theme` for the resolved light/dark theme. Do not create a second preference key or an unrelated toggle on a new page.
- The header, footer, platform banners, signature wells, source listings, build commands, and download plate intentionally retain dark materials in both modes. Give native controls and scrollbars the colour scheme of their actual surrounding surface.
- The landing page is a composed sequence of dark, paper, and orange sections. Its fixed composition is intentional; reading-page dark mode is not a global inversion filter.

Check contrast in both palettes, including muted captions, links inside highlighted blocks, selected/unselected controls, focus rings, and native pop-up menus. A token's name alone is not proof of contrast against a newly chosen surface.

### Syntax highlighting

Docs use two custom Expressive Code themes from `src/themes/syntax.mjs`: **Cinch Instrument** (dark olive `#1b2218`, ivory text, amber types, orange keywords, olive strings/functions) and **Cinch Paper** (warm paper `#e7deca`, ink text, rust keywords, ochre types, dark olive strings/functions). Keep token distinctions within this material family. Do not reintroduce the stock editor theme's purple, cyan, or electric blue. Comments remain legible and upright; ordinary variables remain quieter than keywords/types. Other fenced languages use the same palette.

The theme colours are contrast-adjusted with a 5.5:1 minimum against their code surface. Expressive Code follows Starlight's `data-theme`; its UI-colour overrides are disabled so the explicit paper/instrument materials survive. Use the shared Cinch grammar already registered in `astro.config.mjs` rather than changing code text to achieve colouring.

The custom content loader's digest includes the Astro configuration, syntax-theme source, and Cinch grammar. Preserve this coupling: rendered code contains token colours and hashed stylesheet/script links. A source-Markdown-only digest can retain old HTML that points at an asset the new build no longer emits. After changing the theme, inspect actual computed colours in both modes and verify emitted code assets exist, including on a subsequent cached build.

## 5. Materials and texture

Use the existing `public/textures/surface-grain.svg` as a quiet static material cue. Current landing overlays are often around `.065` opacity, light sections around `.1`, and docs around `.055–.07`; the console uses a somewhat stronger local treatment. These settings depend on blending and background colour. Inspect the actual result instead of copying an opacity blindly.

A material layer should sit behind the content, follow the surface's clipping/radius, and ignore pointer events. Existing components use local pseudo-elements and isolation. Avoid applying opacity or a blend mode to an entire text-containing section. Grain must not reduce code or prose clarity.

The console is the richest material object. Its dark bezel, recessed olive screen, very slight local shading, printed labels, seams, fasteners, scale, and control deck describe one assembled thing. That local depth is compatible with the mostly flat, bold page. It does not justify bevels, drop shadows, scanlines, or a glass highlight on every block.

Age and tactility come from palette, fine texture, and fitted construction. Do not add random scratches, rust, peeling paint, fake liquid-crystal faults, or film-grain animation. The supplied room imagery includes wear and upholstery; that is an inspiration for material warmth, not a requirement to make documentation look damaged.

## 6. Ornament with a job

Use the following as a vocabulary, not a checklist to fill on every page.

| Motif | Useful purpose | Typical location | Failure mode |
| --- | --- | --- | --- |
| Thin/double rule | Divide sections or imply a fitted joint | Section boundary, sidebar start, source block | Borders around everything |
| Numbered rail | Establish sequence or hierarchy | Manual section, tutorial, figure | Arbitrary numbers with no referent |
| Leader line/callout circle | Connect a note to a real part | Technical illustration | Diagram-looking clutter without an explanation |
| Small fastener/seam/vent | Make a casing feel assembled | Console or major material boundary | Screws on every paragraph/card |
| Gauge/tick scale | Show an actual quantity or progression | Vector length example | Invented live metrics or redundant progress |
| Rounded shoulder/end cap | Define a physical section or selected item | Banner, chapter plate, navigation | Uniform oversized rounding everywhere |
| Small stripe/running head | Mark a chapter or subject | Hero/chapter rail | Decorative slogan disguised as data |
| Static grain | Give an existing surface a little tactility | Broad field or casing | Visible noise over reading text |

As a working heuristic, give a new utility section one dominant structural/material gesture, then only the small details needed to explain or align it. This is guidance derived from the chosen direction, not a fixed count the user prescribed. Review the page at a distance: the heading and content relationship should register before the screws do.

Do not add fake switches, random LEDs, decorative terminal prompts, uptime counters, invented certification marks, or impossible “control panels.” A true status should be expressed in ordinary language with text as well as colour. Decorative parts should not look like actionable controls. Real controls should have clear labels, focus, and feedback.

## 7. Technical illustrations

The military maintenance manual is the reference for technical communication. Borrow its hierarchy and clarity: figure number, descriptive title, consistent stroke weights, explicit part labels, leader lines, construction lines, occasional crosshatching, and a caption that explains the relationship.

`TechnicalFigure.astro` is the current example. It shows an actual nonempty vector, its tail, and the borrowed result. Its isometric cells, dimension labels, dashed construction, and numbered legend explain the code alongside it. An illustration of an unrelated engine pasted beside code would copy the surface of the manual and lose the point.

For a new figure:

1. Choose the specific type, ownership, data-flow, lifetime, or module relationship to explain.
2. Sketch only the parts needed for that relationship. Keep the reading path obvious.
3. Draw simple explanatory geometry in SVG. Reuse colour roles and stroke logic; avoid a new illustration style for every topic.
4. Supply unique SVG IDs, a title/description, a caption, and text equivalents for essential labels. Multiple instances must not collide.
5. Keep visible text legible at mobile sizes. Stack the legend when needed; simplify geometry before shrinking essential labels.
6. Preserve the source Markdown and section anchors when inserting it into Starlight. Inspect `MarkdownContent.astro` before extending that mechanism.

Reserve raster generation for work that actually needs raster imagery. Existing code-native diagrams, grain, CSS material details, and SVG geometry should normally be extended in their native form. Brand image editing follows the available image-editing workflow and the [logo notes](logo-generation.md).

## 8. Brand assets

The production mark is `public/brand/cinch-wordmark-instrument-v2.webp`, rendered by `Brand.astro`. The form is connected italic script with a broad C, tall h, and wide looping swashes. Its orange fill and slender ivory keyline connect the original identity to the current palette.

The source is opaque on black and uses screen blending on dark chrome. Do not place this asset unchanged on a pale background and assume the black disappears. Use the existing dark placement, or prepare and verify an appropriate asset if a task requires a light-background mark. Keep proportions and inspect around 105–138px wide, not just at the original image size. The user specifically found thick cream edging hard to read at small sizes.

The gold wordmark and many-tailed fox are archived brand origins. Their continued presence in the repo is not a request to restore them to the hero. The approved production landing direction currently omits the mascot. Preserve that choice unless the user changes it.

## 9. Page recipes

### Landing page

The user subsequently asked for a substantially richer tour of the intended language, including unfinished features. Keep the design ambitious and use the language design documents as the content source. A single development note can establish that scope; do not dilute every example with a separate implementation disclaimer. [The landing language-tour source map](landing-language-tour.md) records the current examples and their constraints.

Lead with the language's purpose, a confident typographic hero, and a useful action. Use a concrete interactive example as evidence. Continue with broad alternating fields, prose/code relationships, a manual-style illustration, horizontal feature rows, and clear paths into documentation, downloads, and libraries. Preserve the spaciousness between these ideas.

Apply the space-age vocabulary to section contours, rails, transitions, navigation bays, and material surfaces. Do not leave all the character inside the console. Equally, do not turn the whole page into a literal control room with every text block bolted to its own screen.

### Documentation

Keep Starlight's reading and navigation functions. Use the dark spine, active chapter end cap, warm or olive reading sheet, curved title plate, running rail, numbered rules, shaped code frames, quiet contents column, and native scrollbars. Decoration becomes quieter as reading density increases. Preserve search, provenance, heading anchors, pagination, and mobile navigation.

### Package catalogue

Use the existing platform shell and a bold page title. The inset search and small filter controls lead into a calm list of libraries/modules/declarations. Thin separators and aligned category/version metadata are preferable to a new grid of floating cards. Keep keyboard search, filters, counts, empty states, and shareable query parameters functional.

### Package and API reference

Keep a visible relationship between package, version, module, and declaration. Use the package navigation spine, concise title, local section navigation, linked signatures, constructor detail, examples, and source/permalink actions. The type links are part of the product's value; do not replace them with a screenshot or unlinked decorative code.

Side columns may fold away on narrow screens; local code/signature overflow remains usable. Long module names, older versions, and empty/small declaration groups are useful layout checks. Preserve the older-version notice and version-specific declaration filtering.

### Downloads

Use platform/channel selection as real controls, followed by the selected artifact and verification detail. The dark download plate may feel like an instrument face while the surrounding instructions stay ordinary and readable. Filename, target, version, size, checksum, and link must all describe the same selection. Keep the sample-bundle explanation until actual compiler artifacts are connected.

### New utility pages

Start with `PlatformLayout.astro`, shared navigation, theme controls, and existing paper/panel tokens. Choose a clear content structure, then add one recognisable curved plate, rail, or inset treatment where it helps. A changelog, search result, or error page should not require a new mascot, new font, or a fresh visual metaphor.

## 10. Editorial voice

State what a feature does, what a type requires, what a control changes, and where a link leads. Concrete nouns and observable behaviour do more work than mood-setting copy. Readers should learn something about Cinch rather than about the designer's chosen aesthetic.

| Avoid | Prefer for the relevant context |
| --- | --- |
| “A field guide, not a black box.” | “Read the language design.” / “Module reference” |
| “Unlock the power of seamless, next-generation safety.” | Describe the required evidence, effect, ownership rule, or supported action |
| “Systems nominal. Your journey begins.” | “Choose a platform.” / An actual implementation status |
| A mock serial number on an ordinary paragraph | A real chapter/figure number or no label |
| Repeating “X, not Y” to add attitude | Explain the actual behaviour directly |

The alternatives are examples of a style, not phrases to repeat across every page. Existing concise slogans may remain when intentional; their presence is not a licence to fill every spare corner with another one. Keep sample/design status accurate. Check live source before turning a design intention into a compiler capability claim.

## 11. Review and maintenance

For a visual change, inspect a desktop view, roughly 390px mobile, the relevant reading themes, and the complete page. Inspect focus, selection, overflow, scrollbars, and native menus. Follow any changed interactions. Use the appropriate commands listed in [AGENTS.md](../AGENTS.md) and [README.md](../README.md); do not add elaborate tests for a reversible spacing change.

Review against these questions:

- Does the large-scale composition still feel bold before any ornament is noticed?
- Do type and spacing communicate the content hierarchy clearly?
- Are the curved surfaces and material details integrated across the page?
- Does every prominent technical detail correspond to real information or structure?
- Can someone comfortably read and use the page in both themes and on a small screen?
- Have we introduced an unnecessary font, colour, component variant, texture asset, or preference mechanism?
- Are sample content and design examples still labelled accurately?
- Did an old screenshot accidentally restore obsolete copy, navigation, or the mascot?

When a deliberate design decision changes, update this handbook and the relevant atlas entry. Add a dated screenshot when helpful, keeping earlier exploration distinguishable. Avoid accumulating override after override: make the owning component or semantic token express the final choice.
