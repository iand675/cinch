# Explicit nominal Region parameters

This checkpoint adds a declaration-owned `Region` parameter role to the Rust
checker. Syntax, whitespace and the isolated `cargo check --lib --offline` gate
passed on source revision `97515d7d` in 63.25 seconds. Focused tests and Native
family gates have not run. No provider, runtime, interface schema or pinned
revision changes.

The preserved Map producer exported `MutableMap` with `r :: Type` while its
closed mutation functions quantified the same parameter as `r :: Region`.
`nominal_params` previously had no Region role, and nominal headers and private
resolved contracts both used its default Type classification. Rewriting the
consumer's header would erase defining authority, so this patch requires an
explicit `(r :: Region)` on a nominal declaration instead.

`ParamKind::Region` converts only scoped identity arguments to `Ty::Region`.
Data, record and codata constructors quantify those identities in
`Scheme.region_params`; they do not acquire type representation or value
arguments. Opaque declarations preserve the same role while hiding their
representation. The existing `parameters[].kind` encoding carries `Region` in
both the public header and private resolved nominal rows. No new metadata flag
or execution authority is introduced.

Explicit Type and Region annotations remain lexical. Explicit Type parameters
cannot become mutation identities, and explicit Region parameters cannot become
field types, ordinary explicit Type arguments, numeric indices or higher-kinded
arguments. Unannotated source slots keep their existing contextual behavior;
their exported Type headers are not changed. Source-absent imports continue to
treat that recorded Type role as authoritative, including the older conflicting
Type-header/Region-function refusal control. Open callback rows still produce
`resolved: null` under the existing transport limit.

The focused target `interface_nominal_regions` prepares genuine header,
resolved-contract and qualified reexport checks for data, phantom, record,
codata and opaque carriers. It also prepares nine source refusal cases and an
actual compiled Native checker that reads those produced contracts after the
producer source is deleted. Its forged-header control keeps dependency hashes
coherent while requiring the loader to reject contradictory nominal roles.
These tests are not claimed passed until their actual execution receipts exist.

Required next gates are the focused target,
existing function Region/owned mutation and interface-kind controls, then actual
self-built Native parity and source-absent consumers under a separately admitted
family. The historical source-body projection and V2/V4 families remain unchanged.

The exact type-check command, configurations, source claims and output accounting
are recorded in `nominal-region-role-check-results.json`. The final cache used
200.43 MiB including directory metadata, slightly above the 200 MiB output cap;
the running monitor had counted regular-file blocks only. No further compiler
command ran after that boundary. The generated check cache is separately eligible
for verified lossless archival; sources and receipt inputs remain plain.

A later static scope review found that nested quantifier notes could escape by
spelling into a sibling quantifier, and inherited notes could accumulate across
top-level signatures. The follow-up isolates the complete nested conversion,
propagates only free-name notes, and keeps the original top-level signature clear
boundary. Sibling Region/Type binders in both orders and enclosing-binder shadow
refusals are now focused regressions. This follow-up is not included in the
recorded successful type check of `97515d7d`; its semantic gates remain pending.

The unannotated Legacy control is intentionally different: the defining source
retains a contextual Type slot that adds no kind note, while its function's
latent Mut row classifies only that function's quantifier as Region. Neither the
datatype's checked parameter vector nor either exported nominal header changes.
The source-absent consumer still reads the recorded Type header and refuses the
inconsistent Region application.
