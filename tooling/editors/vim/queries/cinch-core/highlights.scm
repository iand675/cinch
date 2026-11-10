; nvim-treesitter highlight query for Cinch Core.
;
; Neovim reads this file from `queries/cinch-core/highlights.scm` when the
; `cinch-core` parser from tooling/tree-sitter-cinch-core is installed. The
; capture names are the ones Neovim's default links understand; the query is
; the same set as the Zed extension's
; `languages/cinch-core/highlights.scm`, since both editors run the same
; grammar. Comments are a named node here (`(comment)`), so they need no
; language-server help.
;
; To use it: build the parser from tooling/tree-sitter-cinch-core (`tree-sitter
; generate`, then point Neovim's parser directory at the result), and put this
; `queries/cinch-core` directory on the runtimepath, for example as
; `~/.config/nvim/queries/cinch-core`. The plain Vim path (ftdetect plus
; syntax) needs neither.

(comment) @comment

; Header: `core 1`, `module X`, `target <triple> opt-level <n>`,
; `source sha256:<hex>`.
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

; Patterns bind a name; a pattern's constructor name is a plain `name`.
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
