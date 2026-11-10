; Indentation for Cinch Core.
;
; The format is line oriented and indented two spaces per level: a unit's
; annotations and body sit one level in from `unit`, and each nested
; s-expression adds a level. `end` closes the unit, so it is outdented.

(unit "end" @outdent) @indent

; `body` opens the expression the unit evaluates to; the expression itself
; may wrap over several lines at the same level.
"body" @indent

; Forms whose children wrap: an application's arguments, a closure's body, a
; `let` chain, a `case` and its alternatives, and the quoting forms.
[
  (app)
  (lam)
  (let)
  (if)
  (case)
  (alt)
  (release)
  (quote)
  (inst_dict)
  (demand)
  (params)
  (annotation)
] @indent
