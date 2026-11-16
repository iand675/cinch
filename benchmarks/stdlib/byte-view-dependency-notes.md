# Byte view UTF-8 dependency snapshot

The source-distributed byte-view and pattern-count work consumes the exact
published `Data.Bytes.Utf8` source from Unicode checkpoint
`1c44dda9` (after `ecf00a71` and `9f690bcb`). It supplies the independently tested
malformed-byte counting policy for empty patterns. This lane does not alter or
reimplement that module. The module imports no String facade and adds no
embedded base dependency.

The parent already integrates the Unicode checkpoints and can skip this
single-file dependency copy when landing view/pattern work. Original Unicode
functional, ownership, portability and factory evidence remains in that lane;
this dependency copy makes standalone consumers in the strings worktree
resolve the same tested source without depending on another worktree's files.

The subsequent exact dependency is `899168d5`: bounded `utf8-count-between`
and `utf8-decode-between` reuse the same private kernels and owner holds.
They let views preserve truncated/interior malformed-byte behavior without
copying the view or reading outside its logical endpoints.
