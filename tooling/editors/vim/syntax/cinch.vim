" Cinch syntax highlighting (draft 0.2 surface + local amendments)
" Reserved words: specifications/surface-syntax-and-grammar.md and
" compiler/src/lexer.rs `keyword` (the two lists are identical).
if exists("b:current_syntax")
  finish
endif

syntax keyword cinchModule module import export as
syntax keyword cinchDecl data record codata effect signature instance implicit
      \ opaque type foreign ccall component prim law contract rewrite target
      \ upgrade generate fixity infix infixl infixr proof spec stage corec macro
syntax keyword cinchKeyword forall Sigma using in let letrec where of
syntax keyword cinchConditional if then else case
syntax keyword cinchRepeat do
syntax keyword cinchHandle handle with
syntax keyword cinchRewrite match replace require justify to
syntax keyword cinchContract safety liveness recurrence stability
      \ always eventually recur stable assume
syntax keyword cinchModifier erased own
syntax keyword cinchReturn return resume
" Contextual words (ordinary identifiers elsewhere; D221-D227 keeps
" `unsafe` usable as an identifier).
syntax keyword cinchContextual handler unsafe when uses immarg for
      \ migrate related attributes replaceable specialized
      \ llvm wasm asm clr jvm
syntax keyword cinchTodo contained TODO FIXME XXX NOTE

" Type and constructor names.
syntax match cinchConstructor "\<[A-Z][A-Za-z0-9_']*"
syntax match cinchIdentifier "[a-z_][A-Za-z0-9_']*\%(-[A-Za-z_'][A-Za-z0-9_']*\)*"

" Comments win over symbol runs; `{- -}` nests.
syntax region cinchComment start="--" end="$" contains=cinchTodo
syntax region cinchComment start="{-" end="-}" contains=cinchComment,cinchTodo

" Literals. A tick, one character, tick is a Char and wins; `'foo` is a
" Sym literal and `''T` a type Name (D123).
syntax match cinchChar "'\%([^'\\]\|\\\\[\\\"'nrt]\|\\\\u{[0-9A-Fa-f]\+}\)'"
syntax match cinchSymbol "'[A-Za-z_][A-Za-z0-9_']*\%(-[A-Za-z_'][A-Za-z0-9_']*\)*"
syntax match cinchTypeName "''[A-Za-z_][A-Za-z0-9_']*"
syntax region cinchString start=+"+ skip=+\\\\\|\\"+ end=+"+
      \ contains=cinchEscape
syntax match cinchEscape contained "\\\\\|\\\"\|\\'\|\\n\|\\r\|\\t\|\\\\u{[0-9A-Fa-f]\+}"
syntax match cinchNumber "-\?\d\+\%(_\d\+\)*\%(\.\d\+\%(_\d\+\)*\)\?\%([eE][+-]\?\d\+\)\?"
syntax match cinchNumber "-\?0[xX][0-9A-Fa-f]\+"

" Attributes: #[name], #[name(args)], #[name = literal].
syntax match cinchAttribute "#\[[A-Za-z_][A-Za-z0-9_.-]*"

" Operators. `$` continues a run (`<$>`) but never starts one; a bare
" `~`, `|`, `!`, `@`, `=`, `$` are reserved punctuation where the
" grammar says so.
syntax match cinchOperator "\%([-!#$%&*+/<=>?@^|~:]\|\$\)\+"
syntax match cinchDelimiter "[()[\]{},;\\]"

hi def link cinchModule Include
hi def link cinchDecl Structure
hi def link cinchKeyword Keyword
hi def link cinchConditional Conditional
hi def link cinchRepeat Repeat
hi def link cinchHandle Keyword
hi def link cinchRewrite Keyword
hi def link cinchContract Keyword
hi def link cinchModifier StorageClass
hi def link cinchReturn Keyword
hi def link cinchContextual Identifier
hi def link cinchTodo Todo
hi def link cinchConstructor Type
hi def link cinchIdentifier Identifier
hi def link cinchComment Comment
hi def link cinchChar Character
hi def link cinchSymbol Constant
hi def link cinchTypeName Type
hi def link cinchString String
hi def link cinchEscape SpecialChar
hi def link cinchNumber Number
hi def link cinchAttribute PreProc
hi def link cinchOperator Operator
hi def link cinchDelimiter Delimiter

let b:current_syntax = "cinch"
