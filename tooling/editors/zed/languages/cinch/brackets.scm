; Bracket pairs for Zed's rainbow brackets and bracket matching.
; The delimiters are named nodes in the grammar, not anonymous tokens, so
; each pair is matched by node name. `[|`/`|]` are single-token nodes
; (`lquote`/`rquote`).

((lparen) @open
  (rparen) @close)

((lbracket) @open
  (rbracket) @close)

((lbrace) @open
  (rbrace) @close)

((lquote) @open
  (rquote) @close)
