# Core output

Cinch compiles in stages: parse, resolve, infer and check, elaborate to
Core, resolve dictionaries and evidence, insert reference counts, apply
rewrites and specialisations, and finally emit LLVM IR. Every stage after
checking works on Core, and Core is where the language's answers become
explicit: which instance a method call used, which arguments are erased
proof evidence, which value index is a runtime argument, which field moves
and which is counted, which `prim` body the target selected.

Today that information lives only in the compiler's in-memory form and in
two artifacts that deliberately hide it: `.cinchif` interfaces keep
signatures and `.ll` keeps machine-shaped code. Debugging a wrong answer,
a divergence between the Rust seed and the Cinch compiler, or a
performance change therefore means reading code, not artifacts.

This note specifies a textual Core: a stable, annotated, human-readable
form of exactly what code generation consumes, with one printer, one
reader, one verifier, and one differential tool. It is Cinch's equivalent
of Haskell's Core output, with the differences the language needs: value
indices and erased evidence are explicit, ownership is annotated, and the
text is a checked artifact rather than a debug print.

## 1. What it is and is not

It is:

- The code-generation input. The last Core stage is the program codegen
  reads; the LLVM backend has no other source of truth. If Core prints, it
  is what runs.
- Annotated. Every binder carries its type, effect row and ownership
  class; every dictionary use carries its instance and the slot it came
  from; every rewrite application carries the rule that fired; every unit
  carries its source span.
- Deterministic and diffable. Two runs over the same source and the same
  target produce byte-identical text. No addresses, no hash iteration
  order, no timestamps.
- Readable back. `cinch core verify` parses and re-checks it without
  re-elaborating source, so it can be used as a test input and as the
  fixed-point artifact between compiler stages.

It is not:

- A user-facing language. Syntax is for compilers and `diff`; no attempt
  at pretty ergonomics.
- A stable ABI. The format has a version and the version moves with the
  compiler; only the same compiler reads its own output.
- A replacement for interfaces. `.cinchif` remains the checked contract
  between modules and packages; Core is the body-level view.

## 2. Stages and dump points

Core has four textual stages, named after what they make explicit. Each
is a complete program; each can be emitted or read.

| Stage | Flag | What it makes explicit |
| --- | --- | --- |
| `elaborated` | `--dump-core=elaborated` | Checked elaboration before solving: instance constraints still abstract (`$c.Ord Nat`); evidence parameters named; value indices as terms. |
| `solved` | `--dump-core=solved` | Dictionary and evidence resolution: each constraint becomes a concrete `InstDict` (a static dictionary where closed), each method use a projection from a named dictionary, each proof obligation an erased binder with its discharged proposition. |
| `owned` | `--dump-core=owned` | Reference counting and ownership: `dup`, `drop`, `release`, `box` operations, borrowed and lent parameters, join points for shared continuations. |
| `lowered` | `--dump-core=lowered` | Target-specific: representation classes, selected `prim` bodies, specialisations, rewrite applications, constructors as layouts, joins as blocks. This is exactly the codegen input. |

The LLVM emitter consumes `lowered`. Stages are cumulative: `lowered`
contains `owned`, which contains `solved`, which contains `elaborated`,
minus the information each stage consumes; nothing is silently dropped.
The dictionary at `elaborated` that has no counterpart at `solved` is a
solver bug, and the verifier says so.

`cinch core` subcommands:

- `cinch core emit <file.cinch>` writes `build/<module>.core` at the last
  stage, or `--stage=<name>` for an earlier one, honouring `--target` and
  `--opt-level` the way `emit-ir` does.
- `cinch core verify <file.core>` parses and re-checks: binder types
  agree with uses, erasure marks match the erasure pass, count operations
  are balanced, join points only jump within their group, no free
  variables, dictionary projections name their instance's slot, and every
  constructor's field classes match its layout.
- `cinch core diff <a.core> <b.core>` reports the first divergence as a
  path (`Main.map:alt 2:field 1`), not a line number, so a dump of any
  size is comparable.
- `cinch core html <file.core>` writes an annotated view: spans link to
  source, dictionary bindings highlight their uses, and each unit shows
  its stage history (inlined into, specialised from).

## 3. What is made explicit

### 3.1 Dictionaries and instances

Each resolved dictionary is a named binding: `$d.Ord.Nat` for a static
closed dictionary, `$d.<n>` for a constructed one, with its constructor
arguments printed. A method use prints as a projection that names the
dictionary and the slot:

```core
let $d.Ord.Nat = $static "Data.Nat.Ord_Nat.dict"       -- a static dictionary block
in case $x of
     Nil  -> 0
     Cons h t -> $d.Ord.Nat.3 (h, t)                   -- slot 3 = compare, then applied
```

The stage history of a dictionary is recorded when it was once abstract:
`@resolved(Ord Nat from instance Ord_Nat at Main.cinch:12:5)`.

### 3.2 Evidence and erasure

A proof parameter is printed with its proposition and an erasure mark:

```core
f :: forall (n :: Nat) (e :: Prop). @proof (e : n < 8) -> Vec n U8 -> Int
```

`@proof` binders and their arguments are erased before `owned`; the
`elaborated` and `solved` stages keep them, and the verifier checks that a
runtime term never mentions an erased binder after erasure. A discharged
obligation prints where it was established (`@obligation(0 < n by
parameter index)`), which is what a failed refinement needs to explain.

### 3.3 Value indices

A dependent value index prints as a term, and its runtime status as an
annotation: `@compile-time` for an index that no runtime code reads,
`@runtime(n)` for one that reaches an argument slot, with the slot number.
The `lowered` stage prints both kinds and the layout decision.

### 3.4 Ownership and counts

Parameters and locals carry a class: `owned`, `borrowed`, `lent`,
`uncounted`, `immortal`. Count operations print as what they are:
`dup`, `drop`, `release` (with the moved and left fields named), `refset`.
A derived class prints its derivation (`@borrowed(parameter, never used as
a scrutinee)`), so a performance question about a redundant count answers
from the artifact.

### 3.5 Rewrites, specialisations and prims

A rewrite prints the rule, its binders and a provenance note:

```core
@rewrite(map-fusion v1 at Main.cinch:40)
map f (map g xs) = map (f . g) xs
```

A specialisation prints its key (`@specialised(<Nat, Int>)`) and the
generic unit it came from. A `prim` prints the selected body and the
condition that selected it (`@prim(load-u64 for target.capability=simd128
by manifest condition 3)`).

### 3.6 Joins, tail calls and sharing

A join point prints as `join $j (params) = ...; jump $j (args)` and a
tail call as `tailcall`. A shared continuation introduced by match
compilation prints the rows it stands for, so the sharing decision is
visible rather than implied by block reuse.

## 4. Text format

```
core 1
module Main
target aarch64-apple-darwin opt-level 2
source sha256:9f2c...            -- source tree identity, not a file list

unit Main.map :: forall a b. (a -> b) -> List a -> List b @borrowed(1,2)
  @span Main.cinch:20:3
  @specialised(<Nat, Int>) from List.map
  params: f :: a -> b @borrowed, xs :: List a @borrowed
  body:
    case xs of
      Nil -> Nil
      Cons h t -> Cons (f h) (Main.map f t) @tailrec
end

unit $d.Ord.Nat :: Ord Nat @immortal
  @span Data/Nat.cinch:88:1
  static { compare = Data.Nat.Ord_Nat.compare, eq = Data.Nat.Eq_Nat.eq }
end
```

Rules:

- One unit or binding per record, terminated by `end`; indentation is
  two spaces per level; no tabs.
- Identifiers are the compiler's canonical names, quoted with `'...'` when
  they contain anything but `A-Za-z0-9_.-$`.
- Annotations start a line with `@` and are sorted by name. Span is the
  source location of the unit's defining occurrence.
- Literals print exactly: numbers as decimals, strings as escaped UTF-8
  with their byte length, byte buffers as `b"\x00..."` beyond 32 bytes
  plus a digest.
- Everything sorts: modules by name, units by canonical name, dictionary
  bindings by name, alternatives by constructor tag then by pattern text.
- The header is fixed (`core 1`, `module`, `target`, `source`). The target
  line appears only for stages where the target matters (`lowered`); an
  earlier stage is target-independent and must be byte-identical across
  `--target` values, which is the check that target facts have not leaked
  into the frontend.

## 5. Where it lives

- The Rust seed prints from its own Core (`compiler/src/core.rs`); the
  printer walks `Program`'s `fns`, `instances`, `foreigns`, `prims` and
  `plans`, and reads the annotations the passes already compute
  (`UnitRepr`, `FnMeta`, `Program::lent`, `CtorLayout`).
- The Cinch compiler prints from `Bootstrap.Core` after
  `Bootstrap.RuntimeIr` has decided layouts; its printer is a second
  implementation of the same grammar, and the two must agree byte for byte
  on the same source, target and stage. That agreement is a stronger
  frontend-parity check than comparing LLVM IR, because it is
  target-independent up to `owned` and localises a divergence to a unit
  rather than to an IR line.
- `.core` files go in `build/` beside `.ll`, are named after the entry
  module, and are listed in the build manifest with their stage, target,
  and digest.

## 6. Use in the bootstrap

The bootstrap's fixed point currently compares the executable, the LLVM IR
and the interfaces between Stage2 and Stage3. Core adds two stronger
checks:

1. `owned` Core is target-independent, so Stage2 and Stage3 must agree on
   it for every module, and a disagreement names the unit and the pass that
   produced it. LLVM IR equality only says "same output"; Core equality
   says "same meaning".
2. `elaborated` Core is shared by both compilers, so the seed and the
   Cinch compiler agree or not on the frontend's own terms. A Stage1 that
   checks but elaborates differently is caught before codegen.

`cinch core diff` is the tool for both: it reports the first diverging
path across stages, which turns "the two stages differ in 40 MB of IR"
into "unit Main.resolve differs at alternative 3, field 1".

## 7. Cost and plan

Printer plus reader plus verifier is a small addition to each compiler:
the annotations already exist, so this is mostly formatting, parsing and a
local checker. The plan, with the editor and server work of section 8:

1. Rust seed printer at `checked`, `owned` and `lowered`, with the
   structural verifier, `cinch core emit|verify|diff`, `--dump-core`, and
   a golden corpus (one file per feature) in `compiler/tests/core/`.
   Acceptance: every golden file regenerates byte for byte, verifies
   cleanly, and `checked`/`owned` are identical across targets.
2. Core grammar `tooling/tree-sitter-cinch-core` with its corpus and
   drift check, and a whole-file parse check over the golden files.
3. Language-server branch for `.core` documents: diagnostics from the
   verifier, semantic tokens, hover, definition, symbols, folding, each
   with a test in `compiler/tests/lsp.rs`.
4. Editor integrations for `cinch-core` in vim, emacs, sublime, zed and
   vscode.
5. Cinch compiler printer for the same stages. Acceptance: byte-identical
   `checked` Core against the seed on the toolkit bundle, and Stage2 and
   Stage3 agree on `owned` Core.
6. Wire both into the bootstrap audit: compare `owned` Core between stages
   (it is target-independent, so a divergence names a unit and a pass) and
   keep the LLVM IR comparison as the codegen check.

## 8. Editor and language-server support

Core files are read by people while debugging, so they get the same
tooling as surface syntax: a tree-sitter grammar, an editor integration in
every editor the project ships, and language-server support for the parts
that are mechanical.

### 8.1 File type

`.core` files, language id `cinch-core`, display name "Cinch Core". The
first line (`core 1`) is the file signature; editors that sniff content
use it when the extension is missing. The compiler's `--dump-core` names
files `<entry>.<stage>.core` so a directory listing shows the stage.

### 8.2 tree-sitter grammar

A second grammar, `tooling/tree-sitter-cinch-core`, generated from a
`grammar.js` that follows section 10 line for line: `header`, `unit`,
`annotation`, `params`, `expr` with one node type per form in the table
(`int_lit`, `nat_lit`, `bytes_lit`, `string_lit`, `sym_lit`, `bool_lit`,
`unit_lit`, `local_ref`, `global_ref`, `ctor_ref`, `prim_ref`,
`foreign_ref`, `app`, `lam`, `dict_proj`, `proj`, `inst_dict`, `let`,
`if`, `case`, `alt`, `pat`, `dup`, `drop`, `release`, `box_init`,
`box_get`, `box_clear`, `ref_set`, `quote`, `splice`, `syntax_lit`,
`demand`), plus `type`, `class`, `span`.

There is no scanner complexity here: the format is line- and
s-expression-structured, so the grammar is a plain Lex-like recursive
grammar with an indentation rule for `body` children. The corpus in
`corpus/` has one case per form plus the layout edge cases (a node that
fits on one line, a node that wraps, an empty case, a `release` with no
moved fields). `tooling/check-grammar-drift.sh --core` regenerates and
runs it, and a whole-file property check parses every `compiler/tests/core/*.core`
golden file with zero ERROR nodes, which keeps the grammar honest against
what the printer actually emits.

### 8.3 Editor integrations

Every existing integration gains a `cinch-core` language:

- vim: `ftdetect/cinch-core.vim` (`*.core`), `syntax/cinch-core.vim`
  (units, annotations, classes, literals, comments), and a tree-sitter
  query file for the nvim-treesitter path the shared grammar enables.
- emacs: `cinch-core-mode.el` derived from `prog-mode` with font-lock for
  the same categories and `imenu` over units.
- sublime: a `.sublime-syntax` with the same categories and an `imenu`
  symbol list for units.
- zed: a second grammar entry and a `CincheCore` language in
  `tooling/editors/zed/extension.toml` plus `languages/cinch-core/`
  config and query files; `sync-grammar.sh` pins both grammars.
- vscode: a second language in `package.json` with the grammar
  contribution and a `language-configuration.json` (brackets `()`,
  comments `--`).

The highlighter categories are the ones the format has, not surface
syntax: unit names, dictionary bindings (`$d.*`, `$static`), classes,
annotations (`@span`, `@class`, ...), numerals, string and byte literals,
the `core 1`/`module`/`target` header, and `end`.

### 8.4 Language server

`cinch lsp` already speaks LSP over stdio for `.cinch` documents. It gains
a Core branch keyed on the document's extension, so one server serves both
languages:

- Diagnostics: parse errors from the Core reader and verifier errors from
  `coretext::verify`, each with the range of the offending node, severity
  from the error class, and a code that names the check
  (`core/unbound-local`, `core/duplicate-unit`, `core/slot-missing`,
  `core/annotation-order`, `core/parse`). Diagnostics recompute on every
  change with the same debounce as surface documents.
- Semantic tokens: unit declarations and references, dictionary bindings
  and projections, class names, annotation names, literals, and the
  header.
- Hover: on a `(global X)` the defining unit's type, class and span; on a
  `(local x)` its `let`/`lam`/pattern binder annotation; on a class or a
  dictionary binding its instance and the slots it provides; on `@span`
  the source file and line.
- Definition: a `(global X)` that names a unit in the same file jumps to
  it; a unit whose body has `(global Y)` with no local definition reports
  that Y is imported (no jump), never a wrong jump.
- Document symbols: units, in canonical order, nested bodies as children.
- Folding: units, `lam`, `case` alternatives, `let` chains.
- Workspace symbols: every unit of every open `.core` file, prefixed by
  its file.

The server does not type-check Core: `verify` checks structure, scopes,
slot references and annotations, and the hover data comes from what the
file itself prints. Re-deriving types would mean a second elaborator,
which is out of scope.

### 8.5 Testing the tooling

- `tooling/check-grammar-drift.sh --core` green; `check-real-files-parse.sh`
  gains the Core golden files.
- A vim/sublime/emacs syntax test is a fixture file per editor with the
  expected categories listed as comments, checked by each editor's own
  test shim where one exists; where none exists, the fixture is checked
  in and the limitation recorded in `tooling/README.md`.
- One language-server test per capability, extending
  `compiler/tests/lsp.rs`: open a `.core` document, then request
  diagnostics (one clean file, one broken), semantic tokens, hover on a
  global and on a local, definition within the file, document symbols, and
  folding ranges.

## 9. Open decisions

- Whether `lowered` Core should be the actual pass boundary (codegen
  reads the parsed structure rather than the in-memory one). That makes
  the reader load-bearing, enforces the annotation invariants, and costs a
  round trip in every build; it is the right end state, but not the first
  step.
- Whether annotations are part of the grammar (parseable and ignorable) or
  comments. Parseable annotations let a golden file assert an ownership
  class; comments lose that. This note assumes parseable and ignorable.
- Whether `solved` records failed solver attempts (a "why this instance"
  trail). Useful for dictionary errors; the first version records only the
  winning instance and the goal it satisfied.
- Whether the demonstration HTML view belongs in the compiler or in the
  site's tooling. The compiler should emit only the text.

## 10. Text grammar (normative)

This section is the normative grammar both printers implement. Anything
not stated here is printed by the rules of section 4.

```
file        = header line* module-body
header      = "core 1" NL
line*       = ( "module " name NL | "target " triple " opt-level " digit NL
              | "source sha256:" hex64 NL )*
module-body = unit*
unit        = "unit " name " :: " type ann* NL
              ( "  @span " file ":" int ":" int NL )?
              ( "  @params " params NL )?
              "  body " expr NL
              "end" NL
params      = param ( ", " param )*
param       = name " :: " type " @ " class
```

Expressions are canonical s-expressions, one node per line, indented two
spaces per depth relative to `body`; a node whose children all fit on one
line of at most 100 characters prints on one line. Every node form:

| Form | Meaning |
| --- | --- |
| `(int <i64>)` | machine word constant |
| `(nat "<digits>")` | mathematical Nat constant outside the word range |
| `(bytes "<escaped>" <len>)` | byte buffer constant |
| `(string "<escaped>" <len>)` | String constant |
| `(sym <name>)` | interned symbol constant |
| `(bool true|false)`, `(unit)` | literals |
| `(local <name>)` | local binder |
| `(global <Name>)` | top-level unit |
| `(ctor <Name> <tag> <arity>)` | constructor function |
| `(prim <Name>)` | primitive operation |
| `(foreign "<symbol>")` | foreign function |
| `(app <expr> <expr>*)` | application, callee first |
| `(lam (<params>) <body>)` | closure (its captured `env` prints as a `@captures` annotation) |
| `(dictproj <expr> <slot> <Class>)` | dictionary projection |
| `(proj <expr> <field>)` | constructor or record projection |
| `(instdict <Name> <expr>*)` | constructed dictionary |
| `(let <name> :: <type> @<class> <expr> <body>)` | strict let |
| `(if <expr> <expr> <expr>)` | conditional |
| `(case <expr> (<alt>*) )` | case; alts sorted by tag then text |
| `(dup <name>)`, `(drop <name>)` | count operation on a local |
| `(release <name> (moved <name>*) (rest <name>*) <body>)` | drop specialisation |
| `(boxinit <name> <expr>)`, `(boxget <expr>)`, `(boxclear <name>)` | knot slots |
| `(refset <name> <expr>)` | field store |
| `(quote <n> <body>)`, `(splice <n>)`, `(syntaxlit <n>)` | quotations |
| `(demand <expr>)` | `Later` demand |

`alt = (<pat> <expr>)`; `pat = _ | (<var>) | (<CtorName> <pat>*) | <int>`.

Types print in surface-like canonical form with no spaces around arrows:
`forall a b. a -> b`, `List Nat`, `refine (x :: Nat) with x < 8`,
`{IO r, Diverge} a` for a row applied to a result. Type variables print
`a`, `b`, ... in first-occurrence order of the unit's scheme; a rigid
skolem prints `#a`, a metavariable `?12`.

Annotations print in this order, each on the `unit` line or as an
indented `@` line when long: `@span`, `@class`, `@captures`,
`@specialised(<keys>)`, `@rewrite(<rule> v<n>)`, `@prim(<Name> for
<fact>)`, `@resolved(<instance> at <span>)`, `@tailrec`, `@join(<id>)`,
`@stage(<name>)`. An annotation the stage does not carry is omitted.

Classes print `owned`, `borrowed`, `lent`, `uncounted`, `immortal`.

Sorting: modules by name, units by canonical name, dictionary bindings by
name, alternatives by tag then text, annotations in the order above.
Numbers print without separators; negative `Int` constants keep their
sign; strings escape `\` `"` `\n` `\t` and bytes below 0x20 as `\xNN`.

Related designs: `dependent-core-kernel.md`, `compiled-interface-metadata.md`,
`ownership-and-usage.md`, `reclamation.md`, `inline-target-ir.md`,
`arrays.md`, `self-hosting.md`.
