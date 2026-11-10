; Outline items for Zed's project/outline panel.
; Top-level declarations are wrapped in `declaration` nodes; the name token
; differs per form, so each rule anchors on the keyword that introduces it.

(module_declaration
  (module_path) @name) @item

(signature_declaration
  (kw_signature) . (constructor) @name) @item

(data_declaration
  (kw_data) . (constructor) @name) @item

(record_declaration
  (kw_record) . (constructor) @name) @item

(codata_declaration
  (kw_codata) . (constructor) @name) @item

(instance_declaration
  (kw_instance) . (constructor) @name) @item

(effect_declaration
  (kw_effect) . (constructor) @name) @item

(opaque_type_declaration
  (kw_type) . (constructor) @name) @item

; Value definitions: `name pats = body`, and the parenthesised operator form
; `(<) x y = body`. The leading anchor keeps the capture on the defined name
; rather than on every identifier in the argument patterns.
(declaration
  (equation
    . (identifier) @name)) @item

(declaration
  (equation
    . (lparen) . (operator) @name . (rparen))) @item
