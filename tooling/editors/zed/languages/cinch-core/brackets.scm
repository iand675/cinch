; Bracket pairs for Zed's rainbow brackets and bracket matching.
; The delimiters are anonymous tokens in the grammar (there is no node named
; `lparen`), so each pair is matched as a token pair: `(`/`)` around
; s-expressions and `{`/`}` around a dictionary's `static` fields.

(("(" @open ")" @close)
 ("{" @open "}" @close))
