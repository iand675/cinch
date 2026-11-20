# Required tail calls on PowerPC64

The compressed consumer is the exact emitted LLVM from the failed byte-view
portability check. Decompress it and run the command recorded in `receipt.json`.
The complete seed/test/LLVM diagnostic is retained separately.

The two small LLVM files hold ten incoming word arguments and a matching
callee prototype with internal linkage. The required `tailcc` call fails with
LLVM20.1.8; the matching C-convention control emits a real PowerPC64 object.
These are object-emission observations, without a cross-target execution claim.

The separate unbound C-convention consumer preserves the next diagnostic:
ELFv1 also requires locally bound definitions to retain the same TOC across
a required call. Marking those whole-program definitions `dso_local` emits
the complete object. The production repair therefore selects the internal
tail convention by architecture and marks PowerPC whole-program definitions
locally bound. Relocatable object linkage and shared-library export handling
remain separate. Required tail calls and the complete portability check stay.

PowerPC remains an explicitly constructed big-endian qualification profile;
this change does not add it to the public CLI's supported-target table.
