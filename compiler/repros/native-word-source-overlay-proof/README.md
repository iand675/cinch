# Native source-overlay proof regression

The exact unchanged byte writer emits successfully with the admitted frozen provider. Adding the unchanged `Data.Json` and `Text.Codec` source modules makes object emission reject its second store at Word-local line 23. The identical combined source passes check-only mode. No program is linked or run by this diagnosis.

`word-only.cinch` is the passing baseline. `word-json-codec.cinch` adds only the two source modules; the Word declaration is identical. `report.json` retains actual commands, hashes, durations, diagnostics and the 9,903-byte baseline LLVM identity. The original large JSON bundle and its deterministic tracked reproduction remain in the JSON lane.

The resolver removes source-shadowed interface homes and retains every other `DInterface` row. The retained set then lacks five declared dependency edges. `Bootstrap.Infer.elaborate-module` calls the strict interface-set loader; `collect-imported-total-methods` silently omits all imported total bodies on any loader error. The method body defining Nat addition therefore becomes unavailable, so the computed offset is represented as an opaque runtime witness without its required arithmetic equation. The first write is not sufficient to cause this failure: twelve smaller literal, value-witness and exact-Word context controls pass.

Check-only mode follows `recheck-source-overlays`, while the object-client path checks the combined program directly. The mode difference is material evidence, not a successful object-emission gate.

The negative diagnostic projection removes exactly the two source-shadowed homes from the original manifest. It is deliberately incomplete and must never be used as an admitted provider or runtime configuration. The actual strict loader refuses it with `Data.Bool: missing interface dependency: Data.Json`. No interface document, dependency edge or hash was edited. The complete original Nat dependency closure has eleven homes and survives a conservative dependency-closed projection; `transitive-closure-inventory.json` records every original import and document hash. This inventory does not itself grant body authority or implement a repair.

A general compiler repair remains open. It must independently validate any retained complete dependency closure, refuse stale or forged body authority and preserve changed source runtime behavior. The source, provider, pinned inputs and pending Stage1 compiler epoch are unchanged.
