# Actual live fixture, source-first checkpoint

The single executable fixture in `tooling/data/multipart-live-effects.cinchpart`
uses all three proposed factories with real handles and real C I/O. It combines:

- A mutable source over a real Part, using stale and advanced aliases against
  one shared cursor and observing final EOF through the borrowed part as well.
- A mutable source over an actual spilled File. The backend returns positive
  prefixes with an explicit typed caller error. Aliases continue from the advanced
  real cursor; positioned reads, seek, caller-owned Close and repeated Close are
  observed independently.
- A mutable sink over a real output part. An independent C script confirms
  short prefixes with and without errors, exact offers, output and trace hashes,
  latest-error replacement, repeated writer Close and finished old aliases.
- A mutable storage factory whose eight callbacks use actual scratch operations
  and mutate a captured caller `Var`. State, keys and handles contain retained
  managed byte payloads. Backend state, caller counter and actual C operation
  count must agree; descriptor/path cleanup is checked before fixture teardown.

The fresh source gate uses the qualified **pre-Role Rust66** tuple:
`bfacd9af23b3a6770da9ecfb55c12547bf38bcd530399fad82ab00022257e1a9`
with the explicitly recorded future link archive
`d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf`.
This includes the consuming-callback heap repair. It is separate from the earlier
855 and historical Native source checks. No in-flight nominal-Role compiler or
annotated carrier proposal is used.

`live-runtime-source-gate.json` retains the historical source-first checkpoint;
`live-runtime-gates.json` records the subsequent successful actual execution.
`check_multipart_live_effects.py` prepares/checks the fixture;
`run_multipart_live_effects.py` verifies that exact source gate before guarded
emission/link and separate normal 32 MiB/poison 256 MiB runs with an 8 MiB runtime stack.
Compiler host stack is 64 MiB. Successful functional ELF/LLVM output is recoverably
archived only after both profiles pass. Both profiles passed with zero managed
leaks and actual descriptor/path cleanup. Maximum recorded new output was
16,940,929 bytes; the conservative archive-phase bound also remains below 20 MiB.
The independent actual matcher object ran its 1,048,576 scalar lane observations.
All source/compiler/runtime/object/output hashes and full recovery records are
retained. No production adoption, Native runtime, complete selected-production
matrix or source-absent kind admission is claimed by this one Rust fixture.
The static 19-carrier Region proposal remains completely unchecked.
