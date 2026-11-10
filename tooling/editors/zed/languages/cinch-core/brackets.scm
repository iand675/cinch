; Bracket pairs for Zed's rainbow brackets and bracket matching.
; The delimiters are anonymous tokens in the grammar (there is no node named
; `lparen`), so each pair is matched as a token pair: `(`/`)` around
; s-expressions and `{`/`}` around a dictionary's `static` fields.
;
; One top-level pattern per pair. Grouping both pairs in one outer pattern
; would ask for a `(...)` pair and a `{...}` pair as consecutive siblings of
; the same parent, which never happens, so the query would compile and match
; nothing.

("(" @open ")" @close)
("{" @open "}" @close)
