# Documentation comments and symbol lookup

Documentation lives beside the declaration it describes, in comments the compiler parses into a canonical JSON document. The syntax and the `.cinchdoc` schema are defined in the [documentation comments specification](../specifications/documentation-comments-specification.md); this page shows the tool that implements it.

Documentation is not part of a module's checked contract. Editing a comment does not change `iface_hash`, invalidate a compiled interface, or force dependents to rebuild.

# Markers

| Marker | Attaches to |
| --- | --- |
| `-- \|` | the next declaration |
| `-- ^` | the nearest declaration above |
| `{-\| ... -}` | the next declaration, block form |
| `{-^ ... -}` | the nearest declaration above, block form |
| `--` | nothing, unless `--legacy` is passed |

A `-- |` block continues on following `--` lines and ends at a blank source line or the next marker. Ordinary `--` comments are implementation notes and are ignored unless you ask for the legacy convention.

A documented module, which is also the running example for the rest of this page:

<!-- cinch:run exit=0 -->
```cinch
-- | The shapes module.
module Shapes where

-- | A rectangle, measured in whole units.
--
-- The area is $w \times h$.
data Rect where
  -- | A rectangle with its width and height.
  Rect :: (w :: Int) -> (h :: Int) -> Rect

-- | The area of a rectangle.
--
-- @param r the rectangle to measure
-- @returns the area in square units
area :: Rect -> Int
area r = case r of
  Rect w h -> w * h

main :: Int
main = area (Rect 3 4) - 12
```

`cinch run Shapes.cinch` accepts this file and exits 0; the `main` exists so the example is checkable end to end. Remove it and the file is still a valid module for `cinch doc`.

# Extract

```sh
cinch doc extract Shapes.cinch
```

The output is canonical JSON: sorted keys, declarations in source order, no timestamps. The declaration and constructor entries for the file above, trimmed to the interesting fields:

```text
{
  "declarations": [
    {
      "children": [
        {
          "doc": {
            "legacy": false,
            "line": 8,
            "links": [],
            "markdown": "A rectangle with its width and height.",
            "math": [],
            "summary": "A rectangle with its width and height.",
            "tags": {},
            "tests": []
          },
          "id": "Shapes.Rect.Rect",
          "kind": "constructor",
          "name": "Rect",
          "parameters": [
            { "doc": "", "name": "w", "type": "Int" },
            { "doc": "", "name": "h", "type": "Int" }
          ],
          "signature": "Rect :: (w :: Int) -> (h :: Int) -> Rect",
          "source": { "line": 9 }
        }
      ],
      "doc": {
        "legacy": false,
        "line": 4,
        "markdown": "A rectangle, measured in whole units.\n\nThe area is $w \\times h$.",
        "math": [ { "display": false, "line": 6, "tex": "w \\times h" } ],
        "summary": "A rectangle, measured in whole units.",
        "tags": {},
        "tests": []
      },
      "id": "Shapes.Rect",
      "kind": "data",
      "name": "Rect",
      "signature": "data Rect where",
      "source": { "line": 7 }
    }
  ]
}
```

Four things in that output are worth reading closely.

`summary` is the first paragraph with inline Markdown removed, and it is what `lookup` prints.

`signature` is the source line, `data Rect where`, because no checked interface was supplied. With `--iface` it becomes the rendered scheme.

Math is lifted out of the body into `math` with its line, and stays in `markdown`, so a renderer can use either.

The constructor identity is `Shapes.Rect.Rect`, the `Module.Type.Constructor` form the specification defines for a document with no interface. That form is provisional, and it is why a `symbol:` link written for the checked identity will not resolve until you pass `--iface`.

Flags, all read from the parser and exercised:

| Flag | Effect |
| --- | --- |
| `--out PATH`, `-o PATH` | write the document to a file instead of standard output |
| `--iface MODULE.cinchif` | take checked identities and rendered schemes from a built interface |
| `--module NAME` | override the module name in the document |
| `--legacy` | accept a contiguous `--` block before a declaration as documentation |

`extract` exits 1 when the source contains an unattached documentation block, after writing the document:

```text
cinch doc: source contains unattached documentation
```

# Association

A leading block attaches to the next significant declaration, skipping attribute lines. A line that is `import`, `export`, `module`, `where`, or another non-declaration statement breaks the association, and the block is reported as an orphan instead of attaching to something later:

```cinch
-- | This block has no declaration after it.
import Data.Path
```

```text
orphans: [{"kind": "leading", "line": 3, "text": "This block has no declaration after it."}]
```

Inside a `data`, `record`, or `signature` block, a leading block attaches to the next member. A trailing `-- ^` attaches to the nearest declaration above, including a multi-line signature.

Attributes may sit between the block and the declaration:

```cinch
-- | The value.
#[inline(always)]
value :: Int
value = 0
```

Two documentation blocks for one declaration are reported as `duplicate-doc` and the nearest wins. The diagnostic names the declaration:

```text
diagnostics: [{"code": "duplicate-doc", "line": 18, "message": "Shapes.Shape.Dot already has a documentation block"}]
```

That case arises when a constructor has both a leading `-- |` block and a trailing `-- ^` block. Use one.

# Tags, links, math, diagrams

A tag line is metadata and is removed from the rendered body. The full set is in the [specification](../specifications/documentation-comments-specification.md): `@since`, `@see`, `@deprecated`, `@experimental`, `@param`, `@arg`, `@typeparam`, `@field`, `@returns`, `@safety`, `@errors`, `@law`, `@effect`. Tags land in `tags`:

```text
"tags": {"params": [{"name": "r", "text": "the rectangle to measure"}], "returns": "the area in square units"}
```

`@param` and `@arg` merge with named binders from the signature into `parameters`. A named binder supplies `type`; a positional binder leaves it empty:

```text
"parameters": [{"doc": "the rectangle to measure", "name": "r", "type": ""}]
```

Named binders such as `(f :: a -> b)` fill it:

```cinch
-- | Apply a function.
--
-- @param f the transformation
map-one :: forall a b. (f :: a -> b) -> a -> b
map-one f x = f x
```

A `symbol:` link is recorded with its resolution:

```cinch
-- | The area of a shape in square units.
--
-- See [`Rect`](symbol:Shapes.Rect).
```

```text
"links": [{"resolved": false, "target": "Shapes.Rect", "text": "`Rect`"}]
```

Without `--iface` the constructor identity is `Shapes.Rect.Rect`, so `Shapes.Rect` does not resolve and the parser records `unresolved-symbol`:

```text
diagnostics: [{"code": "unresolved-symbol", "line": 30, "message": "Shapes.area links to unknown symbol Shapes.Rect"}]
```

With `--iface build/Shapes.cinchif` the same run resolves it:

```text
"links": [{"resolved": true, "target": "Shapes.Rect", "text": "`Rect`"}]
```

Inline math is `$...$`, display math is `$$...$$`, and fences and inline code are excluded from math extraction. A `mermaid` fence stays in `markdown` for the site to render. Neither is executed by the parser.

# Render and highlight

```sh
cinch doc render Shapes.cinch
```

`render` writes a Markdown page: the module summary, a `## Declarations` section, and one `##` or `###` block per declaration and member, with the signature and the body. A `cinch` fence becomes highlighted HTML:

```html
<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Rect</span> <span class="tok-keyword">where</span></code></pre>
```

`--plain` keeps the fence instead:

````text
```cinch
data Rect where
```
````

`cinch test` fences stay plain in both modes, because their heredoc sections are not Cinch source. The highlighting comes from `compiler/src/highlight.rs` over `tooling/tree-sitter-cinch`, the same classifier the REPL uses for its transcript.

`cinch doc highlight` applies that classifier to a bare `.cinch` file:

```sh
cinch doc highlight Shapes.cinch
```

```html
<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-comment">-- | Documentation test modes.</span>
<span class="tok-keyword">module</span> <span class="tok-constructor">T</span> <span class="tok-keyword">where</span>
```

`cinch doc markdown` applies it to the `cinch` fences of a Markdown file or of standard input, and leaves every other fence alone:

```sh
cinch doc markdown < page.md
```

```text
# A page

Prose outside any fence.

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">main</span> :: <span class="tok-constructor">Int</span>
<span class="tok-variable">main</span> = <span class="tok-number">0</span></code></pre>

```text
not cinch
```
```

Both commands accept `--out PATH`.

# Lookup and index

```sh
cinch doc index . --out symbols.json
```

```text
{
  "schema": 1,
  "symbols": [
    {
      "id": "Shapes.Rect",
      "kind": "data",
      "line": 7,
      "module": "Shapes",
      "name": "Rect",
      "source": "./Shapes.cinch",
      "summary": "A rectangle, measured in whole units."
    },
    {
      "id": "Shapes.Rect.Rect",
      "kind": "constructor",
      "line": 9,
      "module": "Shapes",
      "name": "Rect",
      "source": "./Shapes.cinch",
      "summary": "A rectangle with its width and height."
    },
    {
      "id": "Shapes.area",
      "kind": "value",
      "line": 15,
      "module": "Shapes",
      "name": "area",
      "source": "./Shapes.cinch",
      "summary": "The area of a rectangle."
    },
    {
      "id": "Shapes.main",
      "kind": "value",
      "line": 19,
      "module": "Shapes",
      "name": "main",
      "source": "./Shapes.cinch",
      "summary": ""
    }
  ]
}
```

`index` walks a directory of sources. Each record has `id`, `module`, `kind`, `name`, `summary`, `source`, and `line`. An undocumented declaration is still indexed, with an empty `summary`.

`lookup` builds the same index over `--dir` (default `compiler/prelude`) and searches it, matching an exact `id` first and then a bare `name`:

```sh
cinch doc lookup Shapes.area --dir .
```

```text
Shapes.area (value)
./Shapes.cinch:15

The area of a rectangle.
```

```sh
cinch doc lookup Rect --dir .
```

```text
Shapes.Rect (data)
./Shapes.cinch:7

A rectangle, measured in whole units.
```

The bare name found the data type rather than the constructor, because a name search takes the first index record with that name and the type is indexed before its constructor. Pass the exact id when the kind matters.

`--json` prints the index record instead of the summary:

```text
{
  "id": "Shapes.area",
  "kind": "value",
  "line": 15,
  "module": "Shapes",
  "name": "area",
  "source": "./Shapes.cinch",
  "summary": "The area of a rectangle."
}
```

A name that is not documented is an error, not an empty answer:

```text
cinch doc: no documentation for Nosuch
```

`--legacy` applies the legacy comment convention to the scanned sources.

# Executable examples

A `cinch test` fence is a test case rather than prose. Its sections are `--- input`, `--- stdin`, `--- args`, `--- expect`, `--- error`, and `--- exit`, and the fence info may carry `name=`.

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

```sh
cinch doc test . --compiler /path/to/compiler/target/debug/cinch
```

```text
PASS Shapes.Shape.rect-area
1/1 documentation tests passed
```

The case name is `OWNER.TESTNAME`, where the owner is the identity of the documented declaration.

`--- input` is a complete program or a module body. The runner wraps a body in `module Main where`, writes it to `Main.cinch` in a fresh temporary directory, and runs `cinch run` there. Two consequences:

- The fence is self-contained. Sibling modules are not copied, so an `import` of the documented module fails with `unknown name`.
- The prelude is embedded in the compiler, so prelude names work without any files.

`--- args` is passed after `--`. `--- stdin` becomes the child's standard input, or `/dev/null` when absent. `--- expect` is compared with standard output after trailing newlines are trimmed. `--- exit` is compared with the process exit code. `--- error` requires a nonzero exit and the expected text somewhere in stdout or stderr.

A failure prints the case and what happened:

```text
FAIL T.val.wrong-exit
exit=0 (expected 3), stdout="", stderr=""
```

`--- error` is the one mode where the program is meant to fail:

```text
PASS T.val.reject-mismatch
```

A `>` REPL transcript is recorded in `tests` but has no runner; it is reported as a failure of the run rather than silently skipped:

```text
FAIL T.val.session
```

| Flag | Effect |
| --- | --- |
| `--compiler PATH` | the compiler used to build and run each case |
| `--legacy` | the legacy comment convention |
| `--match TEXT` | run cases whose name contains the text |
| `--timeout SECONDS` | per-case timeout, 90 by default |
| `--report PATH` | write a JSON report with the compiler hash and per-case results |

`--match` with no hits is an error, and so is an empty test set:

```text
cinch doc: no documentation tests matched
```

# Legacy comments

The library predates the `-- |` marker. `--legacy` accepts a contiguous `--` block as documentation and marks the result:

<!-- cinch:run exit=0 -->
```cinch
module Legacy where

-- An ordinary implementation comment.
--
-- It is a contiguous block before a declaration.
double :: Int -> Int
double n = n + n

main :: Int
main = double 21 - 42
```

Without the flag the declaration's `doc` is `null`; with it:

```text
{"legacy": true, "line": 3,
 "markdown": "An ordinary implementation comment.\n\nIt is a contiguous block before a declaration.",
 "summary": "An ordinary implementation comment."}
```

New source should use `-- |`. A canonical marker wins over a legacy block when both are present.

# The documentation-build integration

`website/scripts/generate-library-reference.py` calls `cinch doc extract` to build the library pages. Its `--validate-docs` mode parses every library source and reports diagnostics without regenerating pages:

```sh
python3 website/scripts/generate-library-reference.py \
  --compiler compiler/target/debug/cinch --validate-docs
```

That is how the [library reference](../library/00-index.md) stays in sync with the sources, and why those pages are generated rather than edited.

# Limits

- Association is source-order and indentation-aware, not a compiler AST pass. A block before a form the parser does not recognise is reported as an orphan rather than guessed at.
- A `.cinchdoc` is evidence, not a contract. A signature shown there is advisory unless it came from a checked `.cinchif`.
- `symbol:` links resolve only against the supplied index or interface. An unresolved target stays text and is never turned into a live link.
- `cinch doc test` compiles and runs only self-contained fences, one temporary directory per case.
- Mermaid and math are recorded, not rendered, by the compiler. The site renders them.

Implementation: `compiler/src/cinchdoc.rs` for `cinch doc`, `compiler/src/highlight.rs` for token classification over `tooling/tree-sitter-cinch`. The [compiler guide](01-using-the-compiler.md) covers the build commands that produce the `.cinchif` files `--iface` reads.
