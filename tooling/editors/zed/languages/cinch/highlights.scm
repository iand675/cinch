; Cinch highlight query for Zed (tree-sitter).
;
; Node names come from tooling/tree-sitter-cinch/grammar.js. Comments are
; not captured: the grammar keeps the comment token a hidden extra, so a
; comment never becomes a node a query can match. The scanner has to see
; comments as trivia for the minus and symbol rules, which is why they are
; hidden. Zed colours them from the language server's `comment` semantic
; tokens instead, so `semantic_tokens` must be on for Cinch (see
; tooling/README.md).

[
  (kw_always)
  (kw_as)
  (kw_asm)
  (kw_assume)
  (kw_attributes)
  (kw_case)
  (kw_ccall)
  (kw_clr)
  (kw_codata)
  (kw_component)
  (kw_contract)
  (kw_corec)
  (kw_data)
  (kw_do)
  (kw_effect)
  (kw_else)
  (kw_erased)
  (kw_eventually)
  (kw_export)
  (kw_for)
  (kw_forall)
  (kw_foreign)
  (kw_generate)
  (kw_handle)
  (kw_handler)
  (kw_if)
  (kw_immarg)
  (kw_implicit)
  (kw_import)
  (kw_in)
  (kw_infix)
  (kw_infixl)
  (kw_infixr)
  (kw_instance)
  (kw_justify)
  (kw_jvm)
  (kw_law)
  (kw_let)
  (kw_letrec)
  (kw_liveness)
  (kw_llvm)
  (kw_macro)
  (kw_match)
  (kw_module)
  (kw_of)
  (kw_opaque)
  (kw_own)
  (kw_prim)
  (kw_proof)
  (kw_quotedecls)
  (kw_record)
  (kw_recur)
  (kw_recurrence)
  (kw_replace)
  (kw_replaceable)
  (kw_require)
  (kw_resume)
  (kw_return)
  (kw_rewrite)
  (kw_safety)
  (kw_sigma)
  (kw_signature)
  (kw_spec)
  (kw_specialized)
  (kw_stability)
  (kw_stable)
  (kw_stage)
  (kw_target)
  (kw_then)
  (kw_to)
  (kw_type)
  (kw_unsafe)
  (kw_upgrade)
  (kw_uses)
  (kw_using)
  (kw_wasm)
  (kw_when)
  (kw_where)
  (kw_with)
] @keyword

; Reserved multiplicities: %0, %1, %aff, %many.
[
  (mult_zero)
  (mult_one)
  (mult_aff)
  (mult_many)
] @keyword

(identifier) @variable
(wildcard) @variable.special
(constructor) @constructor
(type_variable) @variable
(type_name_literal) @type
(effect_label) @label

(int_literal) @number
(float_literal) @number
(string_literal) @string
(char_literal) @string
(symbol_literal) @string.special.symbol

(attr_name) @attribute
(attr_args) @string
(prim_fragment) @string
(token_tree) @string

[
  (op_amp)
  (op_andand)
  (op_barbar)
  (op_bind)
  (op_caret)
  (op_eqeq)
  (op_ge)
  (op_gt)
  (op_gtgt)
  (op_le)
  (op_lt)
  (op_ltbar)
  (op_ltgt)
  (op_ltlt)
  (op_ltstar)
  (op_minus)
  (op_percent)
  (op_plus)
  (op_slash)
  (op_slasheq)
  (op_star)
  (op_stargt)
  (operator)
  (arrow)
  (larrow)
  (darrow)
  (equals)
  (backslash)
  (dollar)
  (pipe)
  (tilde)
  (bang)
  (colon_colon)
] @operator

[
  (lparen)
  (rparen)
  (lbracket)
  (rbracket)
  (lbrace)
  (rbrace)
] @punctuation.bracket

[
  (comma)
  (semicolon)
  (dot)
] @punctuation.delimiter

[
  (at)
  (attr_open)
  (lquote)
  (rquote)
  (splice_open)
] @punctuation.special

(qualified_value
  (constructor_path
    (constructor) @constructor)
  (identifier) @function)
