; Cinch Core highlights for Zed (tree-sitter).
;
; Node names come from tooling/tree-sitter-cinch-core/grammar.js (grammar
; `cinch_core`, scope `source.cinch-core`); the format is
; design/core-output.md sections 4 and 10. Later matches win, so the
; dictionary-binding rule below overrides the ordinary unit-name rule for
; `$d.*` and `$static`.

(comment) @comment

; Header: `core 1`, `module X`, `target <triple> opt-level <n>`,
; `source sha256:<hex>`. `header` and `source_line` are leaves, so their
; words are anonymous tokens inside them.
(header) @keyword
(module_line (name) @type)
(target_line (target_triple) @constant)
(source_line) @constant

; Units: `unit <Name> :: <type>`, indented annotations, `body`, `end`.
(unit_name (name) @function)
((unit_name (name) @variable.special)
 (#match? @variable.special "^\\$"))
(annotation_name (name) @attribute)
(annotation_args) @attribute
(class) @type
(ownership_counts) @number
(span) @attribute
(param (name) @variable)

; Literals.
(int_lit) @number
(nat_lit) @number
(bytes_lit) @string.special
(string_lit) @string
(sym_lit) @string.special.symbol
(bool_lit) @boolean
(unit_lit) @constant

; References. `foreign_ref` is a leaf (its symbol is an anonymous token).
(local_ref (name) @variable)
(global_ref (name) @function)
(ctor_ref (name) @constructor)
(prim_ref (name) @function.builtin)
(foreign_ref) @function.builtin

; Patterns bind a name. A pattern's constructor name is a plain `name`,
; never a `ctor_ref`.
(pat (name) @variable)
"_" @variable.special

; Types.
(type (name) @type)
(rigid_var) @type
(meta_var) @type

; Structural words.
[
  "unit"
  "end"
  "body"
  "forall"
  "refine"
  "with"
  "moved"
  "rest"
  "from"
] @keyword

[
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  "::"
  "->"
  ":"
  "."
] @punctuation.delimiter

"@" @punctuation.special
