" Headless checks for the cinch-core vim integration.
"
"   /usr/bin/vim -Nu NONE -es -S tooling/editors/vim/test/check.vim
"
" Asserts the filetype detection (`*.core`, and the `core 1` signature when
" the extension is missing), then that each highlighter category in
" syntax/cinch-core.vim resolves to the expected standard group at a known
" token in test/fixture.core. Results go to stdout and to test/check.out;
" the exit status is nonzero when any check fails.

let s:test = expand('<sfile>:p:h')
let s:root = fnamemodify(s:test, ':h')
let s:out = []
let s:failed = 0

function! s:Check(name, actual, expected) abort
  let ok = a:actual ==# a:expected
  call add(s:out, printf('%-24s %s %s', a:name, ok ? 'ok  ' : 'FAIL', string(a:actual)))
  if !ok
    let s:failed = 1
  endif
endfunction

function! s:Group(line, col) abort
  return synIDattr(synID(a:line, a:col, 1), 'name')
endfunction

" First line matching `linepat`, and the column of `colpat` within it.
function! s:Find(linepat, colpat) abort
  for lnum in range(1, line('$'))
    if getline(lnum) =~# a:linepat
      let col = match(getline(lnum), a:colpat)
      if col >= 0
        return [lnum, col + 1]
      endif
    endif
  endfor
  return [0, 0]
endfunction

function! s:CheckGroup(name, linepat, colpat, expected) abort
  let [lnum, col] = s:Find(a:linepat, a:colpat)
  if lnum == 0
    call add(s:out, printf('%-24s FAIL no line matches %s', a:name, a:linepat))
    let s:failed = 1
    return
  endif
  call s:Check(a:name, s:Group(lnum, col), a:expected)
endfunction

execute 'set runtimepath^=' . fnameescape(s:root)
filetype on
syntax on
execute 'edit ' . fnameescape(s:test . '/fixture.core')

call s:Check('filetype', &filetype, 'cinch-core')
call s:Check('syntax loaded', get(b:, 'current_syntax', ''), 'cinch-core')

" Header.
call s:CheckGroup('header keyword', '^core ', 'core', 'cinchCoreHeaderKeyword')
call s:CheckGroup('header version', '^core ', '\d', 'cinchCoreNumber')
call s:CheckGroup('header module', '^module ', 'module', 'cinchCoreHeader')
call s:CheckGroup('module name', '^module ', 'Main', 'cinchCoreModuleName')
call s:CheckGroup('target triple', '^target ', 'aarch64', 'cinchCoreTriple')
call s:CheckGroup('opt-level', '^target ', 'opt-level', 'cinchCoreHeader')
call s:CheckGroup('source digest', '^source ', 'sha256:', 'cinchCoreDigest')

" Units, classes, annotations, dictionary bindings.
call s:CheckGroup('unit keyword', '^unit Main\.map', 'unit', 'cinchCoreStructure')
call s:CheckGroup('unit name', '^unit Main\.map', 'Main\.map', 'cinchCoreUnitName')
call s:CheckGroup('unit class', '^unit Main\.map', 'borrowed', 'cinchCoreClass')
call s:CheckGroup('annotation', '^  @span', '@span', 'cinchCoreAnnotation')
call s:CheckGroup('class annotation', '^  @class', '@class', 'cinchCoreAnnotation')
call s:CheckGroup('class value', '^  @class', 'owned', 'cinchCoreClass')
call s:CheckGroup('param class', 'params:', 'borrowed', 'cinchCoreClass')
call s:CheckGroup('params keyword', 'params:', 'params', 'cinchCoreStructure')
call s:CheckGroup('dict binding', 'unit \$d\.', '\$d\.', 'cinchCoreDictBinding')
call s:CheckGroup('dict binding name', 'unit \$d\.', 'Ord\.Nat', 'cinchCoreDictBinding')
call s:CheckGroup('static binding', 'static {', 'static', 'cinchCoreStructure')
call s:CheckGroup('end', '^end', 'end', 'cinchCoreStructure')

" Forms, literals, operators.
call s:CheckGroup('form head', '(case (local', 'case', 'cinchCoreNode')
call s:CheckGroup('local ref head', '(local xs)', 'local', 'cinchCoreNode')
call s:CheckGroup('unit literal', '(unit)', 'unit', 'cinchCoreNode')
call s:CheckGroup('int literal', '(int -42)', '42', 'cinchCoreNumber')
call s:CheckGroup('bool literal', '(bool true)', 'true', 'cinchCoreBoolean')
call s:CheckGroup('string literal', '(string "tab', '"', 'cinchCoreString')
call s:CheckGroup('string escape', '(string "tab', '\\t', 'cinchCoreEscape')
call s:CheckGroup('bytes literal', '(bytes b"', 'b"', 'cinchCoreBytes')
call s:CheckGroup('type arrow', '^unit Main\.map', '->', 'cinchCoreOperator')
call s:CheckGroup('paren', '(case (local', '(', 'cinchCoreParen')
call s:CheckGroup('comment', '^-- Core', '--', 'cinchCoreComment')

" The surface syntax's sigil names (amendment section 8): `!` and `?`
" continue a name, never start one, and leave a keyword spelling alone.
enew
execute 'edit ' . fnameescape(s:test . '/fixture.cinch')
call s:Check('surface filetype', &filetype, 'cinch')
call s:Check('iskeyword sigils', [stridx(&iskeyword, '!') >= 0, stridx(&iskeyword, '?') >= 0], [1, 1])
call s:CheckGroup('empty? name', '^  empty? ::', 'empty?', 'cinchIdentifier')
call s:CheckGroup('print! name', '^  print! ::', 'print!', 'cinchIdentifier')
call s:CheckGroup('read!? name', '^  read!? ::', 'read!?', 'cinchIdentifier')
call s:CheckGroup('if? is a name', '^  if? ::', 'if?', 'cinchIdentifier')
call s:CheckGroup('when? is a name', '^  when? ::', 'when?', 'cinchIdentifier')
call s:CheckGroup('bare if is a keyword', 'main = if ', 'if ', 'cinchConditional')
call s:CheckGroup('spaced bang run', 'spaced = 1 ', '!=', 'cinchOperator')

" The signature path: a file with the `core 1` first line and no `.core`
" extension still gets the filetype. `:filetype detect` only re-runs the
" `filetypedetect` group, and ftdetect files define their own groups, so the
" detection autocmds are fired directly.
enew
call setline(1, ['core 1', 'module Sniffed'])
doautocmd BufRead
call s:Check('signature sniff', &filetype, 'cinch-core')

call writefile(s:out, s:test . '/check.out')
for s:line in s:out
  echo s:line
endfor

if s:failed
  cquit
endif
qall!
