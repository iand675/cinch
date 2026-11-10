; Text objects for Vim mode (`af`/`if` and the `[m`/`]m` motions).
; Cinch has no closure/class syntax: the analogue of a function is an
; `equation` and the analogue of a class is a `where` block owner.

(equation) @function.around

[
  (module_declaration)
  (signature_declaration)
  (data_declaration)
  (record_declaration)
  (codata_declaration)
  (instance_declaration)
  (effect_declaration)
  (contract_declaration)
] @class.around
