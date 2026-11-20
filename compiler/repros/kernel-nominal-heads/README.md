# Source nominal heads sharing kernel spellings

The pre-existing `interface_region_effects` regression panicked while collecting
`Shadow.Scope`. This also reproduces with the immutable 37dc Rust compiler
before the effect-union changes, as recorded in `baseline.json`; it is separate
from the cross-region inference defect. The original source and diagnostic are
retained without changing the assertion or disabling debug checks.

The predeclaration pass classified every kernel-recognized spelling as a
forbidden declaration, while the data/record collectors have a narrower,
declaration-specific reserved-name policy. A permitted source nominal therefore
had no registered head when its constructor was collected.

The repair shares the data/refinement reserved-name predicate with the
predeclaration pass and applies the record collector's own policy to records.
Permitted qualified source names keep their ordinary module-owned identities.
Reserved primitive declarations remain refused. The interface regression now
also checks a source-owned `Shadow.Task`, while the actual kernel `Scope` and
`Task` contracts must retain their kernel ownership and region binders.
