# Optional imported proof-body authority

This separate candidate preserves only complete imported dependency closures for optional executable proof-body metadata. It does not change the ordinary interface loader, source resolver, foreign ABI checks or provider admission.

The new entry validates every supplied document envelope and recomputes its claimed identity before excluding anything. It removes whole interfaces whose declared imports are missing or have different hashes, repeats that selection to a fixed point, and uses the existing strict set validator on the result. No dependency edge, nominal contract, body or hash is rewritten. `Bootstrap.Infer` alone uses this optional authority entry.

The exact original Word/source-overlay failure remains preserved in `bootstrap/repros/native-word-source-overlay-proof`. The unchanged Nat defining closure survives when the two unrelated Json/Codec source homes are replaced; interfaces depending on replaced homes contribute no imported body authority.

The candidate's 51 modules passed an independently identified source-typing check with the existing fa979 release executable. That compiler is not equivalent to the newer 37dc/855 snapshots; this result does not qualify a new compiler family. The small 15-module loader/harness closure separately passed source typing with 37dc. Its initial test-only import and literal-pattern corrections, and both full-source 37dc timeout receipts, remain distinct.

The source check emitted 116 raw interfaces totaling 3,983,532 bytes. Only its newly generated 5,074,235-byte build manifest was losslessly compressed; the recovery index records the original/compressed/decompressed hashes and original path. The index was fsynced before unlinking the raw manifest. All source and raw interfaces remain intact.

Nine prepared functional cases observe actual checked NatAdd bodies, absent authority after defining/transitive dependency replacement, changed dependency hashes, damaged hashes and bodies in an excluded home, an executable body outside its declared closure, and a contradictory retained nominal contract. Those cases have not run. The loader executable, actual modified Native Word-emission check and full compiler-family gate remain pending resource budget. No publication or adoption is claimed.
