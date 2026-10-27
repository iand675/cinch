; SafeLang highlight query for Zed (tree-sitter).
; Node names come from tooling/tree-sitter-safelang/grammar.js; keyword
; spellings are the reserved words of the surface grammar.

(kw_module) @keyword
(kw_where) @keyword
(kw_data) @keyword
(kw_record) @keyword
(kw_codata) @keyword
(kw_effect) @keyword
(kw_signature) @keyword
(kw_instance) @keyword
(kw_implicit) @keyword
(kw_import) @keyword
(kw_export) @keyword
(kw_as) @keyword
(kw_type) @keyword
(kw_opaque) @keyword
(kw_foreign) @keyword
(kw_ccall) @keyword
(kw_component) @keyword
(kw_prim) @keyword
(kw_law) @keyword
(kw_contract) @keyword
(kw_rewrite) @keyword
(kw_target) @keyword
(kw_upgrade) @keyword
(kw_generate) @keyword
(kw_infix) @keyword
(kw_infixl) @keyword
(kw_infixr) @keyword
(kw_proof) @keyword
(kw_spec) @keyword
(kw_stage) @keyword
(kw_corec) @keyword
(kw_macro) @keyword

(kw_forall) @keyword
(kw_sigma) @keyword
(kw_using) @keyword
(kw_in) @keyword
(kw_let) @keyword
(kw_letrec) @keyword
(kw_of) @keyword

(kw_if) @keyword
(kw_then) @keyword
(kw_else) @keyword
(kw_case) @keyword
(kw_do) @keyword
(kw_handle) @keyword
(kw_with) @keyword

(kw_match) @keyword
(kw_replace) @keyword
(kw_require) @keyword
(kw_justify) @keyword
(kw_to) @keyword
(kw_erased) @keyword
(kw_own) @keyword
(kw_return) @keyword
(kw_resume) @keyword

; Contextual words keep identifier colour except where the form makes
; them structural (D221-D227 keeps `unsafe` usable as an identifier).
(kw_unsafe) @keyword
(kw_safety) @keyword
(kw_liveness) @keyword
(kw_recurrence) @keyword
(kw_stability) @keyword
(kw_always) @keyword
(kw_eventually) @keyword
(kw_recur) @keyword
(kw_stable) @keyword
(kw_assume) @keyword
(kw_when) @keyword
(kw_uses) @keyword
(kw_immarg) @keyword
(kw_for) @keyword
(kw_migrate) @keyword
(kw_related) @keyword
(kw_attributes) @keyword
(kw_replaceable) @keyword
(kw_specialized) @keyword
(target_family) @keyword

(identifier) @variable
(constructor) @type
(type_variable) @variable
(type_name_literal) @type
(qualified_value (constructor) @type (identifier) @function)
(variable (identifier) @function)

(int_literal) @number
(float_literal) @number
(string_literal) @string
(char_literal) @string
(symbol_literal) @constant
(attr_name) @attribute
(attr_args) @string

(operator) @operator
(op_star) @operator
(op_slash) @operator
(op_percent) @operator
(op_plus) @operator
(op_minus) @operator
(op_ltlt) @operator
(op_gtgt) @operator
(op_amp) @operator
(op_caret) @operator
(op_ltgt) @operator
(op_ltstar) @operator
(op_stargt) @operator
(op_ltbar) @operator
(op_eqeq) @operator
(op_slasheq) @operator
(op_lt) @operator
(op_le) @operator
(op_gt) @operator
(op_ge) @operator
(op_andand) @operator
(op_barbar) @operator
(op_bind) @operator
(colon_colon) @operator
(arrow) @operator
(larrow) @operator
(darrow) @operator
(dollar) @operator
(pipe) @operator
(tilde) @operator
(bang) @operator
(at) @operator

(prim_fragment) @string
(token_tree) @string
