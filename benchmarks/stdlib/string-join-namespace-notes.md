# String join namespace integration

The first combined-provider compiler build stopped during resolution:
`Text.Parse` imported both `Data.String.string-join` and
`Text.Scan.string-join` without selecting a home. Its diagnostic and original
verified Stage0/Stage1 reuse remain in
`build/combined-bootstrap-initial-name-collision`.

The affected text consumers now select the scanning helper under the local
name `scan-string-join`. The resolver module selects its four actual scanning
dependencies. This keeps the existing implementations and public exports while
avoiding a namespace collision with the new embedded string operation.
All modified helper declarations have canonical documentation.

The namespace consumer imports all five affected modules and observes scanning
and shell-word joins. It passes both the older pre-join frozen compiler
(`cad70e53`, actual String source hash `2469698158938128847`) and the fresh
combined producer (`96c4f542`, String source hash `416560347689830784`). Both
O2 executables pass normal 32 MiB and poisoned-free 256 MiB heap profiles on
an 8 MiB stack with zero leaks. Their exact interfaces, source closure,
executable and LLVM hashes are retained in the two namespace receipts.
Neither client shadows the embedded String module.

Every recorded base source still matches provider `0f34c2be`; these five
source-distributed importer changes do not alter that artifact. The compiler
source bundle does change, to `e494aac81b1dd7b6169e5c7db09591ead572ee8c1421f87e38b9aa5a985bc778`.
The resumed fixed-point gate must therefore build Stage1 afresh. It may reuse
only the unchanged, independently verified Stage0 seed and matching runtime;
the earlier Stage1 is retained as evidence rather than counted as a fresh
compiler build. The full fixed-point gate is a separate pending observation.
