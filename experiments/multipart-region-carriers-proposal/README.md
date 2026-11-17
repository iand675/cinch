# Static carrier Region proposal

This declaration-only proposal annotates the `r` parameter of 19 multipart
carriers: 12 public and seven private. Each `r` is an IO/Mut identity, directly
through callback effects or `Var r`, or transitively through another listed
carrier. `manifest.json` gives every historical/proposed header and its basis.

Only these data declaration headers differ from the reviewed both-effect
candidate. Constructors, fields, callbacks, function signatures and bodies,
other parameter kinds, and historical source/interface artifacts are unchanged.
The independent snapshots in the both-effect experiment preserve those
historical headers and their source hashes.

**This file has not been source-checked or compiled.** Do not check it with 855
and do not reinterpret an existing Native family or serialized interface as
supporting it. A supporting future compiler must independently qualify explicit
carrier kinds, genuine source-absent contracts, original constructor inventory
validation and runtime ownership. No nominal-role inference feature is claimed.
