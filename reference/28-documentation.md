# Documentation comments and API search

Cinch keeps API documentation beside declarations in structured comments. The `cinch doc` toolchain extracts canonical `.cinchdoc` JSON, renders Hackage-style Markdown and standalone HTML documentation sites, runs Hoogle-style name and type-signature searches, and executes embedded `cinch test` examples.

Documentation is excluded from checked module contracts: editing comments never changes `iface_hash`, invalidates a `.cinchif` interface, or triggers downstream recompilation.

# Comment markers

| Marker | Attaches to |
| --- | --- |
| `-- \|` | Next declaration (line form; continues on following `--` lines until a blank line or new marker) |
| `-- ^` | Preceding declaration (line form) |
| `{-\| ... -}` | Next declaration (block form) |
| `{-^ ... -}` | Preceding declaration (block form) |
| `--` | Implementation note (ignored unless `--legacy` is passed) |

Leading comments skip `#[...]` attributes to reach their declaration, and attach to constructors, record fields, or class methods inside `data`, `record`, `codata`, and `signature` blocks. Non-declaration statements (`import`, `export`, `module`, `where`) break association so stray doc blocks are reported as orphans rather than silently attaching to a later item. Two explicit doc blocks on one declaration produce a `duplicate-doc` diagnostic, and the nearest block wins.

<!-- cinch:run exit=0 -->
```cinch
-- | Geometric shapes and area calculations.
module Shapes where

-- | A rectangle measured in integer units.
--
-- The area is $w \times h$.
data Rect where
  -- | Construct a rectangle from width and height.
  Rect :: (w :: Int) -> (h :: Int) -> Rect

-- | Compute the area of a rectangle.
--
-- @param r rectangle to measure
-- @returns area in square units
area :: Rect -> Int
area r = case r of
  Rect w h -> w * h

main :: Int
main = area (Rect 3 4) - 12
```

# Metadata tags, symbol links, and math

Tag lines beginning with `@` are parsed into structured metadata and stripped from prose:

| Tag | Meaning |
| --- | --- |
| `@param NAME TEXT` / `@arg NAME TEXT` | Parameter documentation (merged with named signature binders such as `(w :: Int)`) |
| `@typeparam NAME TEXT` | Type parameter documentation |
| `@field NAME TEXT` | Record or constructor field documentation |
| `@returns TEXT` | Return value description |
| `@safety TEXT`, `@errors TEXT` | Preconditions and failure modes |
| `@law TEXT`, `@effect TEXT` | Algebraic laws or effect obligations (repeatable) |
| `@since VER`, `@deprecated TEXT`, `@experimental TEXT` | Stability and lifecycle metadata |
| `@see SYMBOL[, SYMBOL]` | Related symbols |

Cross-references use the `symbol:` scheme: ``[`Rect`](symbol:Shapes.Rect)``. Without `--iface`, constructor identities take the provisional `Module.Type.Constructor` form; pass `--iface build/Shapes.cinchif` to resolve checked interface identities and schemes. Unresolved targets emit an `unresolved-symbol` diagnostic and remain plain text.

Inline math (`$...$`), display math (`$$...$$`), and `mermaid` code fences are preserved in `.cinchdoc` for renderers; code fences and inline backticks are excluded from math extraction.

# Extracting `.cinchdoc` JSON

```sh
cinch doc extract Shapes.cinch --iface build/Shapes.cinchif --out Shapes.cinchdoc
```

Output is deterministic canonical JSON (sorted keys, source-order declarations, SHA-256 of the source file, no timestamps). Each declaration carries `id`, `kind`, `name`, `signature` (checked scheme when `--iface` is supplied, otherwise the source line), `doc` (`summary`, `markdown`, `tags`, `links`, `tests`, `math`), `parameters`, `typeParameters`, `fields`, `children`, and `source.line`.

| Flag | Effect |
| --- | --- |
| `--out PATH`, `-o PATH` | Write to a file instead of stdout |
| `--iface MODULE.cinchif` | Pull authoritative identities and schemes from a checked interface |
| `--module NAME` | Override the module name |
| `--legacy` | Treat contiguous `--` blocks immediately before declarations as documentation |

`extract` still writes its output if the file contains unattached doc blocks, then exits `1` naming the orphan condition.

# Rendering Hackage-style HTML and Markdown

## Standalone searchable HTML (`cinch doc html`)

Generate a self-contained, zero-dependency static documentation site for any directory or single file:

```sh
cinch doc html src/ --out-dir docs/ --title "My Package"
cinch doc html src/ --out-dir docs/ --iface-dir build/ --legacy
```

The output directory contains:
- `index.html` — Namespace-grouped module directory and a live client-side Hoogle search bar (press `/` to focus).
- `<Module>.html` — Hackage-style module page with collapsible **Synopsis**, grouped **Types**, **Signatures**, **Values**, **Operator fixities**, and **Instances**, pretty-printed signatures with hyperlinked type constructors, nested constructors/fields/methods, and collapsible cross-module **Instances** drawers.
- `source/<Module>.html` — Line-numbered, tree-sitter syntax-highlighted source viewer (`#L<line>` anchors with target highlighting).
- `search-index.json`, `style.css`, and `search.js` — Themeable light/dark reading palette and offline search index.

## Markdown and syntax highlighting

```sh
cinch doc render Shapes.cinch --iface build/Shapes.cinchif
cinch doc render Shapes.cinch --plain
cinch doc highlight Shapes.cinch
cinch doc markdown < page.md
```

`render` outputs Hackage-style Markdown with a Synopsis, grouped sections, readable anchors (`#t-Rect`, `#v-area`), and pretty-printed signatures. By default, `cinch` code blocks are pre-highlighted with tree-sitter (`<pre class="cinch-highlight">`); pass `--plain` to keep standard ` ```cinch ` fences. `highlight` highlights a `.cinch` source file, and `markdown` highlights ` ```cinch ` fences in a Markdown stream while leaving `cinch test` and non-Cinch fences untouched.

# Hoogle-style API search and indexing

## Searching from the CLI (`cinch doc search`)

`cinch doc search` ranks declarations by name, type signature, module scope, and kind:

```sh
cinch doc search "(a -> b) -> List a -> List b"
cinch doc search "Result e a -> Option a"
cinch doc search "Ord a => List a -> List a"
cinch doc search "+Data.Map lookup"
cinch doc search "-Crypto is:type Bytes"
cinch doc search "map :: (a -> b) -> List a -> List b"
```

Type search normalizes `forall` binders and module qualifiers, matches alpha-renamed variables (`(x -> y) -> List x -> List y` finds `map`), handles polymorphic generalization (`Int -> Int` finds `id :: forall a. a -> a`), tolerates argument reordering (`List a -> (a -> b) -> List b`), and respects constraints and effect rows.

| Query form | Meaning |
| --- | --- |
| `(a -> b) -> List a -> List b` | Type signature match |
| `partition-results` | Name, prefix, segment, fuzzy, or summary match |
| `map :: List a -> List b` | Combined name and type filter |
| `+Data.List` / `module:Text` | Restrict to modules matching the prefix |
| `-Crypto` | Exclude modules matching the prefix |
| `is:type`, `is:value`, `is:class`, `is:method`, `is:ctor`, `is:instance` | Filter by declaration kind |

| Flag | Effect |
| --- | --- |
| `--dir PATH` | Directory of `.cinch` sources to scan (default `seed/prelude`) |
| `--index PATH` | Query a prebuilt `cinch doc index` or website `library-search-index.json` file |
| `--limit N`, `-n N` | Maximum results to display (default `25`) |
| `--json` | Emit ranked matches as canonical JSON |
| `--strict-comments` | Exclude legacy `--` comments when scanning sources |

On the web, [**API Search**](/library/search/) provides the same Hoogle-style query engine over all checked standard-library interfaces with clickable type links and line-anchored source views.

## Exact lookup and indexing (`lookup`, `index`)

```sh
cinch doc index src/ --out symbols.json
cinch doc lookup Shapes.area --dir .
cinch doc lookup Rect --dir . --json
```

`index` walks a directory and writes `{"schema": 1, "symbols": [...]}` where each entry records `id`, `module`, `kind`, `name`, `signature`, `prettySignature`, `summary`, `source`, and `line`. `lookup` matches an exact `id` first and then the first bare `name`, exiting `1` if absent. Pass the qualified `id` when a type and constructor share a name.

# Executable documentation tests

A `cinch test` fence inside a doc comment defines an executable test case with `--- input`, `--- stdin`, `--- args`, `--- expect`, `--- error`, and `--- exit` sections:

````markdown
```cinch test name=rect-area
--- input
width :: Int
width = 3

main :: Int
main = if width * 4 == 12 then 0 else 1
--- exit
0
```
````

Run all doctests in a directory or file:

```sh
cinch doc test src/ --compiler seed/target/debug/cinch
```

Each case runs in an isolated temporary directory as `Main.cinch` (a snippet without a `module` header is wrapped in `module Main where`). Prelude names are embedded and available automatically; sibling project modules are not copied. `--- expect` compares trimmed stdout, `--- exit` compares exit status (default `0`), and `--- error` asserts a nonzero exit whose output contains the given substring. `>` REPL transcripts are recorded in `.cinchdoc` but have no batch runner and fail under `cinch doc test`.

| Flag | Effect |
| --- | --- |
| `--compiler PATH` | Compiler binary used to run cases (default `cinch` on `PATH`) |
| `--legacy` | Include legacy `--` doc blocks |
| `--match TEXT` | Run only cases whose `OWNER.TESTNAME` contains `TEXT` |
| `--timeout SECONDS` | Per-case timeout (default `90`) |
| `--report PATH` | Write a JSON report with compiler SHA-256 and per-case outcomes |

# Standard-library doc generation

The website's standard-library pages and Hoogle index are generated from checked `.cinchif` interfaces by `website/scripts/generate-library-reference.py`:

```sh
python3 website/scripts/generate-library-reference.py --compiler seed/target/debug/cinch
python3 website/scripts/generate-library-reference.py --check
python3 website/scripts/generate-library-reference.py --compiler seed/target/debug/cinch --validate-docs
```

# Legacy comments and limits

- **`--legacy`**: Pre-existing stdlib files using plain `--` blocks above declarations are still parsed with `--legacy` (marked `"legacy": true` in `.cinchdoc`). Explicit `-- |` markers always take precedence over legacy comments.
- **Scanner model**: `cinch doc` is an indentation-aware lexical scanner rather than a full compiler AST pass; supply `--iface` whenever authoritative checked types and identities are required.
