; Syntax scopes for editor settings that differ inside literals.
; Cinch comments are trivia in the scanner, so they are not named nodes and
; cannot be given a scope.

[
  (string_literal)
  (char_literal)
  (attr_args)
] @string

; `prim` bodies and quoted templates hold foreign or literal text, so
; bracket auto-closing and `.`-triggered completion are wrong inside them.
[
  (prim_fragment)
  (token_tree)
] @raw
