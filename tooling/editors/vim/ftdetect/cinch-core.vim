" Cinch Core filetype detection.
"
" `.core` is the extension (`design/core-output.md` section 8.1); the first
" line (`core 1`) is the file signature, so a file without the extension is
" still recognised when it starts with it.
augroup cinch_core_ftdetect
  autocmd!
  autocmd BufRead,BufNewFile *.core setfiletype cinch-core
  autocmd BufRead,BufNewFile * if !did_filetype() && getline(1) =~# '^core \d\+$' | setfiletype cinch-core | endif
augroup END
