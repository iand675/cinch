" SafeLang syntax highlighting (draft 0.2 surface + local amendments)
" Reserved words: specifications/surface-syntax-and-grammar.md and
" compiler/src/lexer.rs `keyword` (the two lists are identical).
if exists("b:current_syntax")
  finish
endif

syntax keyword safelangModule module import export as
syntax keyword safelangDecl data record codata effect signature instance implicit
      \ opaque type foreign ccall component prim law contract rewrite target
      \ upgrade generate fixity infix infixl infixr proof spec stage corec macro
syntax keyword safelangKeyword forall Sigma using in let letrec where of
syntax keyword safelangConditional if then else case
syntax keyword safelangRepeat do
syntax keyword safelangHandle handle with
syntax keyword safelangRewrite match replace require justify to
syntax keyword safelangContract safety liveness recurrence stability
      \ always eventually recur stable assume
syntax keyword safelangModifier erased own
syntax keyword safelangReturn return resume
" Contextual words (ordinary identifiers elsewhere; D221-D227 keeps
" `unsafe` usable as an identifier).
syntax keyword safelangContextual handler unsafe when uses immarg for
      \ migrate related attributes replaceable specialized
      \ llvm wasm asm clr jvm
syntax keyword safelangTodo contained TODO FIXME XXX NOTE

" Type and constructor names.
syntax match safelangConstructor "\<[A-Z][A-Za-z0-9_']*"
syntax match safelangIdentifier "[a-z_][A-Za-z0-9_']*\%(-[A-Za-z_'][A-Za-z0-9_']*\)*"

" Comments win over symbol runs; `{- -}` nests.
syntax region safelangComment start="--" end="$" contains=safelangTodo
syntax region safelangComment start="{-" end="-}" contains=safelangComment,safelangTodo

" Literals. A tick, one character, tick is a Char and wins; `'foo` is a
" Sym literal and `''T` a type Name (D123).
syntax match safelangChar "'\%([^'\\]\|\\\\[\\\"'nrt]\|\\\\u{[0-9A-Fa-f]\+}\)'"
syntax match safelangSymbol "'[A-Za-z_][A-Za-z0-9_']*\%(-[A-Za-z_'][A-Za-z0-9_']*\)*"
syntax match safelangTypeName "''[A-Za-z_][A-Za-z0-9_']*"
syntax region safelangString start=+"+ skip=+\\\\\|\\"+ end=+"+
      \ contains=safelangEscape
syntax match safelangEscape contained "\\\\\|\\\"\|\\'\|\\n\|\\r\|\\t\|\\\\u{[0-9A-Fa-f]\+}"
syntax match safelangNumber "-\?\d\+\%(_\d\+\)*\%(\.\d\+\%(_\d\+\)*\)\?\%([eE][+-]\?\d\+\)\?"
syntax match safelangNumber "-\?0[xX][0-9A-Fa-f]\+"

" Attributes: #[name], #[name(args)], #[name = literal].
syntax match safelangAttribute "#\[[A-Za-z_][A-Za-z0-9_.-]*"

" Operators. `$` continues a run (`<$>`) but never starts one; a bare
" `~`, `|`, `!`, `@`, `=`, `$` are reserved punctuation where the
" grammar says so.
syntax match safelangOperator "\%([-!#$%&*+/<=>?@^|~:]\|\$\)\+"
syntax match safelangDelimiter "[()[\]{},;\\]"

hi def link safelangModule Include
hi def link safelangDecl Structure
hi def link safelangKeyword Keyword
hi def link safelangConditional Conditional
hi def link safelangRepeat Repeat
hi def link safelangHandle Keyword
hi def link safelangRewrite Keyword
hi def link safelangContract Keyword
hi def link safelangModifier StorageClass
hi def link safelangReturn Keyword
hi def link safelangContextual Identifier
hi def link safelangTodo Todo
hi def link safelangConstructor Type
hi def link safelangIdentifier Identifier
hi def link safelangComment Comment
hi def link safelangChar Character
hi def link safelangSymbol Constant
hi def link safelangTypeName Type
hi def link safelangString String
hi def link safelangEscape SpecialChar
hi def link safelangNumber Number
hi def link safelangAttribute PreProc
hi def link safelangOperator Operator
hi def link safelangDelimiter Delimiter

let b:current_syntax = "safelang"
