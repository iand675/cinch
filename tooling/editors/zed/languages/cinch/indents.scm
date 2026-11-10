; Indentation ranges for Cinch's layout blocks.
;
; The keyword that opens a block (`where`, `of`, `do`, `let`, `with`) sits on
; the header line, so the range starts at the end of that keyword (`@start`)
; rather than at the first item. Zed indents a row when a range starts on the
; previous row, so this makes the first item line one level deeper than the
; header and every following item line align with it.

[
  (module_declaration
    (kw_where) @start)
  (signature_declaration
    (kw_where) @start)
  (data_declaration
    (kw_where) @start)
  (record_declaration
    (kw_where) @start)
  (codata_declaration
    (kw_where) @start)
  (instance_declaration
    (kw_where) @start)
  (effect_declaration
    (kw_where) @start)
  (contract_declaration
    (kw_where) @start)
  (prim_declaration
    (kw_where) @start)
  (rewrite_declaration
    (kw_where) @start)
  (handler_expression
    (kw_where) @start)
  (quote_decls
    (kw_where) @start)
  (case_expression
    (kw_of) @start)
  (do_expression
    (kw_do) @start)
  (let_expression
    (kw_let) @start)
  (letrec_expression
    (kw_letrec) @start)
  (with_expression
    (kw_with) @start)
  (do_statement
    (kw_let) @start)
] @indent

; Explicit brace blocks (`where { ... }`) open on the same header line, so
; the range also starts after the `{`.
[
  (declaration_block
    (lbrace) @start)
  (signature_block
    (lbrace) @start)
  (constructor_block
    (lbrace) @start)
  (field_block
    (lbrace) @start)
  (instance_block
    (lbrace) @start)
  (operation_block
    (lbrace) @start)
  (handler_block
    (lbrace) @start)
  (alternative_block
    (lbrace) @start)
  (do_block
    (lbrace) @start)
  (local_block
    (lbrace) @start)
  (prim_body_block
    (lbrace) @start)
  (contract_clause_block
    (lbrace) @start)
  (rewrite_block
    (lbrace) @start)
] @indent

; `in` closes the nearest implicit `let`/`with` block, so the `in` line
; aligns with the line that opened it and the body after `in` is one level
; back out as well.
[
  (let_expression
    (kw_in) @outdent)
  (letrec_expression
    (kw_in) @outdent)
  (with_expression
    (kw_in) @outdent)
]
