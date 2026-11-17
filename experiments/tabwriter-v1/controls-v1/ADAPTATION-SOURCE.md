# Adaptation source checkpoint

All twelve explicit native adaptation controls in `AdaptMain.cinch` pass parsing, type checking and proof checking under compiler `330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2` with matched runtime identity `d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`. The source-only report is `adaptation-source-v4/report.json`; the exact entry and library hashes are recorded, and their original source bytes were unchanged before/after the isolated check.

The check took 4.24 seconds and produced 2,583,990 bytes. Root authorized a scoped 320 MiB start / 250 MiB reserve / 5 MiB maximum / 15-second source-only unit. Emission and runtime retain the ordinary 350 MiB start rule. No executable, LLVM emission, runtime, SDK rerun, Native comparison or package promotion is claimed here.

The controls retain all twelve original reference identities in `adaptation-plan.json`: three negative constructor coordinates, three prevalidation refusals that leave an old writer untouched, three total callback errors distinct from recorded panics with no return count, explicit mathematical overflow refusal, exact bounded tab output despite a narrower intermediate overflow, and initialized-only empty behavior. Original panic observations are preserved; no returned reference count was invented.

Three earlier fixture-only failures remain in `failures/`. Their original input and full diagnostics are retained. The successful entry fixes unsupported formfeed spelling, local conditional layout and an explicit ordinary `Bytes` carrier; it changes no library behavior or proof rule. Library source remains the exact `3f5baacc` candidate. Runtime execution of these twelve controls and the other ownership/composition/kernel gates remain open.
