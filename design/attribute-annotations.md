# Attribute annotations on the AST

Design note for the attribute annotation effort
(`specifications/attribute-annotations-specification.md` is the normative
chapter; this note covers the compiler-side representation and the split
with the macro machinery). No em dashes.

## 1 Why attributes are a shared surface

Three performance-facing features need annotations on existing syntax:
branch weights (and the `cold`/`likely`/`unlikely` wrappers), inline
hints, and manual layout/representation/alignment. None of them deserve
their own bespoke syntax; the macro slice (P9) will also want to attach
tool- and macro-defined metadata to the same nodes. The attribute system
is the common carrier.

## 2 AST representation

- One `Attr` node: `name: Vec<Name>` (dotted path), `args: AttrArgs`
  (`Lit` value or argument list), `span`, and a `provenance: AttrOrigin`
  (`Source` or `MacroExpanssion(stage, site)`).
- Attribute lists ride on the AST nodes that own them, not as a side
  table: `Decl`, `CtorDecl`, `FieldDecl`, `Equation`, `MatchAlt`,
  `Guard`, and the branch positions of `If`/`Case`. Keeping them on the
  alternative mirrors the core rule that a weight is a property of the
  alternative (`Maybe u32` beside it in `core.rs`), never a parallel
  list, so no transformation can separate them.
- `Pat` and `Ty` carry attribute lists only where section 4 of the spec
  admits them (field types); do not give every node an empty list.

## 3 Parsing and validation pipeline

1. Lexer: `#[` opens an attribute; `]` closes it. Content is tokenized
   normally (names, integer/char/string literals, commas, `=`). This
   cannot clash with existing syntax: `#` is unused (comments are `--`).
2. Parser: builds `Attr` nodes with literal args only (no expressions);
   placement errors (an attribute where section 1.1 admits none) are
   syntax errors here.
3. Check: per-family validation. Compiler attributes validate name,
   argument grammar, placement, and cross-attribute rules (all-or-nothing
   weights, `repr(align)` power of two, `transparent` single field).
   Macro-owned names are deferred to the macro's declared validation;
   `meta(...)` is unchecked payload.
4. Core: only semantic content survives to lowering. Weights become the
   `Maybe u32` on alternatives; `repr` attributes have already been
   resolved into layout facts recorded in the interface ABI table; inline
   hints attach to the function's codegen record; `meta` is dropped.

## 4 Ownership split (compiler vs macros)

- Compiler-known families: `weight`, `likely`, `unlikely`, `cold`,
  `inline`, `noinline`, `repr`. The compiler MUST reject unknown names
  outside the
  `meta` namespace until an attribute macro declares them (P9); the
  declaration mechanism is the macro's, and hygiene applies to attribute
  names like other identifiers.
- The optimizer treats hint attributes as erasable (preservation
  discipline): dropping or rewriting them is always sound. Semantic
  attributes (`repr` family) never reach the optimizer as hints; they
  are already compiled into layout facts and ABI identity.

## 5 Identity and interface metadata

- Semantic attributes participate in `iface_hash` (contract projection,
  decision D9): a `repr` change is an ABI change and must stale
  dependents.
- Hint attributes are excluded from the contract projection but MAY be
  recorded in the artifact manifest for build reproducibility and
  post-link tooling (BOLT-style workflows care about hot/cold layout).
- Upgrade (P13): a `repr` change on a persisted type is a
  state-representation change and triggers migration checking.

## 6 Lowering targets

- Weights: LLVM `!prof !{!"branch_weights", i32, i32}` on `br` and
  `switch`, per the spec's section 3.1; native backends consume the same
  `Maybe u32` per alternative.
- Inline: LLVM function attributes `inlinehint`/`alwaysinline`/
  `noinline`.
- Layout: computed offsets/size/align go into the interface ABI table and
  shape the struct lowering in codegen (`repr(C)` field order is source
  order under the C algorithm).

## 7 Open implementation questions

- Whether attribute macros may attach attributes to nodes the macro
  cannot otherwise construct (proposal: yes, but every emitted attribute
  revalidates under ordinary rules).
- Whether the all-or-nothing weight rule should be enforced before or
  after macro expansion (proposal: after; a macro may legitimately emit a
  complete set).
