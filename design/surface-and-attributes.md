# Surface syntax and attributes

This document records design decisions for Cinch's AST attribute annotations and surface ergonomics (`letrec`, tail calls, and identifier/operator rules).

Normative syntax rules live in [`specifications/surface-syntax-and-grammar.md`](../specifications/surface-syntax-and-grammar.md), [`specifications/surface-grammar-local-amendments.md`](../specifications/surface-grammar-local-amendments.md), and [`specifications/attribute-annotations-specification.md`](../specifications/attribute-annotations-specification.md). User documentation lives in [`reference/02-lexical-structure.md`](../reference/02-lexical-structure.md), [`reference/03-expressions-and-patterns.md`](../reference/03-expressions-and-patterns.md), and [`reference/23-attributes-and-pragmas.md`](../reference/23-attributes-and-pragmas.md).

---

## 1. Attribute annotations (`#[...]`)

Attributes attach structured metadata to declarations, fields, and parameters without overloading comments or adding ad-hoc keywords:

- **Lexical form:** `#[name]` or `#[name args...]` where `name` immediately follows `#[` on the same line and balanced parentheses/strings are captured verbatim.
- **Categories:**
  - **Representation & layout:** `#[repr(C)]`, `#[repr(packed)]`, `#[repr(align(N))]`, `#[repr(transparent)]`, `#[repr(unboxed)]`, `#[repr(u8|u16|u32|u64)]`, `#[unpack]`, `#[nounpack]`.
  - **Calling convention / FFI:** `#[byval]`, `#[sret]`.
  - **Branch weights & optimization hints:** `#[likely]`, `#[unlikely]`, `#[inline]`, `#[noinline]`, `#[cold]`.
  - **Safety assertions & profiles:** `#[no-alloc]`, `#[alloc "once"]`, `#[recursion "none"|"tail"|"bounded(n)"]`, `#[stack "constant"|"bounded(n)"]`, `#[max-complexity n]`, `#[max-lines n]`, `#[assert-min n]`, `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]`, `#[deny-warnings]`, `#[profile "jpl"]`.
  - **User / macro attributes:** registered via `macro name attributes (...)` so library macros can inspect and transform annotated declarations hygienically.
- **Contract visibility:** semantically relevant attributes (`repr`, safety assertions) are stored in `.cinchif` and hashed into `iface_hash`; unknown attributes are compile errors rather than silently ignored.

---

## 2. Recursive bindings and tail calls

- **Explicit local recursion (`letrec`):** ordinary `let` bindings are non-recursive and evaluated strictly in order; mutually or self-recursive local bindings use `letrec` with guardedness checking (eager cyclic value references are rejected; recursion must pass through a function/lambda or guarded knot).
- **Guaranteed tail calls:** self tail calls compile to loops, and mutual tail calls within a recursive group compile to guaranteed `musttail` transfers (under `tailcc` on 64-bit native targets, or widened `#tail` workers on 32-bit/Wasm targets). Constant stack space is a language guarantee enforced by `#[recursion "tail"]` and `#[stack "constant"]`.

---

## 3. Identifier and operator conventions

- **Kebab-case identifiers:** hyphens join word characters inside identifiers (`nat-add`, `bytes-len`, `non-empty`), while binary minus requires whitespace separation (`x - y`) and unary negation binds tightly to numeric literals (`-1`).
- **Tick disambiguation:** single-character `'a'` is a `Char` literal; multi-character or symbol-named `'foo` is an interned `Sym` literal.
- **Declaring-module fixity:** operator precedence and associativity are declared in the module that defines the operator and transported in `.cinchif`/`.cpk` interfaces so downstream parsers resolve fixities deterministically before type checking.
- **Reserved `~`:** `~ x = e` is reserved exclusively for region-scoped memo bindings; irrefutable `~p` argument patterns are intentionally excluded.
