" Cinch Core syntax highlighting.
"
" The format is specified in design/core-output.md: the header (section 4),
" the normative text grammar (section 10) and the highlighter categories
" (section 8.3). Categories: unit names, dictionary bindings (`$d.*`,
" `$static`), ownership classes, annotations, numerals, string and byte
" literals, the header keywords, and `end`.
"
" Every group links to a standard highlight group with `hi def link`, so any
" colorscheme colors the file without knowing these groups.

if exists("b:current_syntax")
  finish
endif

" Header. `core 1` is the file signature; `target` appears only for stages
" where the target matters; `source sha256:<hex64>` identifies the tree.
" `opt-level` is not a keyword (`-` is not a keyword character), so it is a
" match.
syntax keyword cinchCoreHeaderKeyword core
syntax keyword cinchCoreHeader module target source
syntax match cinchCoreHeader "opt-level"
syntax match cinchCoreModuleName "\%(^module\s\+\)\@<=\S\+"
syntax match cinchCoreTriple "\%(^target\s\+\)\@<=\S\+"
syntax match cinchCoreDigest "sha256:\x\+"

" Record structure: one unit per record, closed by `end`. `unit` is a
" match rather than a keyword: a keyword beats a match at the same start
" column, and `(unit)` is the unit literal, which the form match must own.
syntax keyword cinchCoreStructure end body params static
syntax match cinchCoreStructure "^unit\>"
" Type-level words (section 10: `forall a b. a -> b`, `refine ... with ...`).
syntax keyword cinchCoreKeyword forall refine with
syntax keyword cinchCoreBoolean true false

" Ownership classes (section 10: owned, borrowed, lent, uncounted, immortal).
syntax keyword cinchCoreClass owned borrowed lent uncounted immortal

" The unit name on its `unit <Name> :: <type>` line. Dictionary units are
" named `$d.<Class>.<Type>`; the binding group below wins on those.
syntax match cinchCoreUnitName "\%(^unit\s\+\)\@<=\S\+"

" Dictionary bindings and dictionary references.
syntax match cinchCoreDictBinding "\$d\.[A-Za-z0-9_.]\+"
syntax match cinchCoreDictBinding "\$static\>"

" Annotations: `@span`, `@class`, `@captures`, `@specialised`, `@rewrite`,
" `@prim`, `@resolved`, `@tailrec`, `@join`, `@stage`.
syntax match cinchCoreAnnotation "@[A-Za-z_]\+"
" A parameter's class prints as `@<class>` (section 4 example) as well as
" `@ <class>` (section 10). The `@` belongs to the annotation spelling, so
" the class match starts at the `@` and skips it with `hs=s+1`; being
" defined after the annotation it wins that tie.
syntax match cinchCoreClass "@\%(owned\|borrowed\|lent\|uncounted\|immortal\)\>"hs=s+1

" Canonical s-expression heads, one node form per name in section 8.2.
syntax match cinchCoreNode "\%((\)\@<=\%(int\|nat\|bytes\|string\|sym\|bool\|unit\|local\|global\|ctor\|prim\|foreign\|app\|lam\|dictproj\|proj\|instdict\|let\|if\|case\|alt\|pat\|dup\|drop\|release\|boxinit\|boxget\|boxclear\|refset\|quote\|splice\|syntaxlit\|demand\)\>"

" Literals. Byte buffers print as `b"..."`; strings as `"..."` with the
" escapes in section 4 (`\\`, `\"`, `\n`, `\t`, `\xNN`).
syntax region cinchCoreBytes start=+b"+ skip=+\\\\\|\\"+ end=+"+
      \ contains=cinchCoreEscape
syntax region cinchCoreString start=+"+ skip=+\\\\\|\\"+ end=+"+
      \ contains=cinchCoreEscape
syntax match cinchCoreEscape contained "\\\\\|\\\"\|\\n\|\\t\|\\x\x\x"
syntax match cinchCoreNumber "\d\+"

" Operators and parens.
syntax match cinchCoreOperator "->\|::\|=>\|<=\|>=\|[-<>=|:,]"
syntax match cinchCoreParen "[()[\]{}]"

" Comments last so `--` beats the operator and the minus sign.
syntax match cinchCoreTodo contained "\<\%(TODO\|FIXME\|XXX\)\>"
syntax match cinchCoreComment "--.*$" contains=cinchCoreTodo

hi def link cinchCoreHeaderKeyword PreProc
hi def link cinchCoreHeader Include
hi def link cinchCoreModuleName Identifier
hi def link cinchCoreTriple Constant
hi def link cinchCoreDigest Constant
hi def link cinchCoreStructure Keyword
hi def link cinchCoreKeyword Keyword
hi def link cinchCoreBoolean Boolean
hi def link cinchCoreClass StorageClass
hi def link cinchCoreUnitName Function
hi def link cinchCoreDictBinding Special
hi def link cinchCoreAnnotation PreProc
hi def link cinchCoreNode Statement
hi def link cinchCoreBytes String
hi def link cinchCoreString String
hi def link cinchCoreEscape SpecialChar
hi def link cinchCoreNumber Number
hi def link cinchCoreOperator Operator
hi def link cinchCoreParen Delimiter
hi def link cinchCoreComment Comment
hi def link cinchCoreTodo Todo

let b:current_syntax = "cinch-core"
