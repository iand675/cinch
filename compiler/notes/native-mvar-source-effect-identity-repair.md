# Declared effects and kernel label identity

Status: source-only repair proposal. No Cargo, runtime or compiler-family gate has
run for this increment. The retained first parent run demonstrated that a genuine
unowned `effect MVar` kept the bare semantic head MVar and therefore matched the
actual primitive signature before interface rendering. Its emitted false-new row
was already a canonical resolved MVar row; changing export strings cannot repair
that source identity collision.

A single documented declaration-head helper is used at both early header reservation
and actual operation-table registration. Owned declarations keep their exact previous
module-qualified heads. Ordinary unowned declarations keep the existing global label
unless their spelling overlaps a real kernel effect head. Overlapping declarations
receive their defining module; they remain legal ordinary effects with handlers.
No effect spelling is forbidden.

The complete kernel namespace considered here is the existing FFI-accounted set
Foreign/IO/Global/Blocking/Diverge/Fail/Reentrant/ThreadAffine plus the semantic heads
Async/Cancel/Eval/Mut/MVar. No new kernel effect or arbitrary reserved name is added.
No embedded library in this source checkpoint declares any of those labels as an
ordinary effect.

Registration/lookup audit:

- Early reservation and operation installation now compute the identical head, so
  forward signatures and operations cannot choose different identities.
- effect_head reads actual lexical/declaration/scoped/import bindings. Its frozen
  symbolic fallback stays unchanged, so an earlier symbolic kernel label does not
  acquire a later declaration's identity.
- resolve_top's legacy global fallback and syntax_global_scope's legacy injection
  operate only when head equals the ordinary effect name. Unrelated legacy effects
  retain that behavior; overlapping labels now require their actual owner/import.
- Handler checking uses effects[head] and operation ValueKind heads from registration;
  it does not select a handler by a displayed source or interface string.
- Primitive equality and FFI accounted-effect checks stay unchanged. A declared
  Main.MVar or Main.Diverge cannot authorize the corresponding kernel primitive.
- Owned generated declarations retain their prior qualified contracts and namespace
  validation; existing syntax_effect_declarations and effect scope tests remain gates.

Prepared genuine controls cover all thirteen overlapping spellings plus a legacy
Notice effect, forward headers, actual MVar handler execution in two memory profiles,
a declared Diverge intrinsic refusal, failure to hide real kernel mutation under an
ordinary MVar handler, and matching versus wrong-owner qualified handlers. The old
ordinary-MVar intrinsic refusal stays in place. Source/diagnostics and any panic
project remain inspectable; diagnostic source copies use .source to avoid recursive
project discovery. No old failure or snapshot was changed.

This source repair does not grant portable authority to an old unresolved display
scheme. The demonstrated Native acceptance of UserMVar.signal with resolved:null
needs its separate checked portable-row boundary; a newly qualified owner spelling
must not be used to dismiss that existing captured counterexample.
