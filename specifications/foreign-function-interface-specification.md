# Foreign function interface specification

Source: <https://chatgpt.com/space/page_4f4effb6396c8191a2b8bf41e791ca09>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2 extension. The language MUST support external libraries without implying that their declarations constitute proofs of their behavior. This chapter proposes a layered FFI for native libraries, host imports, and isolated components. Syntax and wrapper APIs remain provisional.

# 1 Three boundaries

| Boundary | What it supplies | What remains to establish |
| --- | --- | --- |
| Raw ABI binding | Calling convention, layouts, symbols and primitive values | Pointer validity, ownership, behavior and effects |
| Safe language wrapper | Validated arguments, resource protocols, typed errors and scoped access | Any stated assumptions about the foreign implementation |
| Semantic contract module | Logical model and functional or temporal guarantees | Evidence for that exact implementation, or an explicit trusted assumption |

A generated header binding MUST NOT automatically be considered safe. A wrapper can establish its own preconditions and validate returned data, but it cannot retrospectively undo memory corruption that occurred inside an in-process call. An unverified native library therefore remains in the process's memory-safety trust boundary even when its public wrapper is safe to call.

When a library is not trusted with the process heap, offer an isolated component or process transport. The adapter validates messages and capabilities, models failures, and keeps untrusted raw pointers out of the caller's address space. Isolation does not establish the remote algorithm's correctness or availability.

# 2 Native ABI and layout

The initial native interoperability baseline SHOULD be an explicitly declared C ABI with target-specific layout information. Rust and C++ libraries can expose an agreed C shim; direct support for another ABI requires a separate versioned backend contract. A source-language type name is not an ABI specification.

Bindings MUST specify integer widths and signedness, structure offsets and alignment, discriminants, padding policy, calling convention, symbol identity, pointer address spaces, and ownership of returned allocations where relevant. Language-native closures, dictionaries, vectors, dependent pairs, and proofs have no implicit C representation.

```haskell
module CodecRaw where
  opaque type Context :: Type
  foreign import ccall "codec_open" open
    :: () ->{Foreign Codec, Blocking, Diverge} RawPtr Context
  foreign import ccall "codec_close" close
    :: RawPtr Context ->{Foreign Codec, Diverge} ()
  foreign import ccall "codec_decode" decode
    :: RawPtr Context -> RawPtr U8 -> SizeT -> RawPtr U8 -> SizeT
    -> RawPtr SizeT ->{Foreign Codec, Blocking, Diverge} CInt
```

These illustrative declarations do not describe a real named library and do not assert totality or safety. A `foreign` declaration has opaque external behavior until an effect and contract envelope is supplied. It does not inherit the total-pure default of an ordinary `fn`.

Variadic arguments, packed structures, unions, bitfields, and callbacks need explicit supported representations or generated shims. Reject unsupported layouts instead of guessing. Cross-compilation uses target layout metadata rather than the generator host's `sizeof` results.

# 3 Ownership and allocation

Each pointer argument or result needs a role: call-scoped borrow, retained borrow, transferred ownership, opaque handle, or raw unchecked pointer. Safe callers MUST NOT construct these roles by casting integers or claiming ownership of borrowed data.

Allocator identity belongs to the resource protocol. Memory returned by a foreign allocator must be released using its matching deallocator unless a proved allocator-compatibility contract permits otherwise. Nullability, zero-length buffers, alignment, initialization, and out-parameters require explicit rules. A non-null pointer alone is not evidence that an allocation is valid or sufficiently large.

An output-buffer wrapper must establish capacity before the call and initialization of the exposed prefix afterward. Checking `written <= capacity` is necessary for a returned length but cannot prove that the callee respected capacity while running. That memory behavior must be verified, trusted, or contained by a suitable isolation boundary.

If the language runtime moves managed allocations, the wrapper must pin or copy them for the entire foreign retention interval. Pinning stabilizes location; it does not grant aliasing permission. Zero-copy is allowed only when lifetimes, layout, ownership, and mutation rules justify it.

# 4 Effects and foreign outcomes

Bindings MUST account for I/O, global state, blocking, divergence, reentrancy, thread affinity, and failure as applicable. An external function is not pure merely because its C signature has no explicit state parameter. A functional-correctness theorem must refer to the actual effectful behavior or an explicitly justified pure model.

Recoverable error codes become typed results. Unknown external outcomes remain unknown. Panics, language exceptions, foreign exceptions, `longjmp`, and nonlocal exits MUST NOT cross an ABI boundary unless the target explicitly supports a checked unwinding protocol. Otherwise use a shim that catches supported exceptions and translates them; process aborts and invalid memory access are not ordinary catchable results by assumption.

Blocking native calls run through a declared blocking executor or another explicit scheduler contract. Cancelling an awaiting task does not prove the native function stopped. Its buffers, handles, callback contexts, and code version remain live until completion or a documented cancellation acknowledgment.

# 5 Callbacks and reentrancy

A callback registration owns a stable trampoline and environment. It records invocation multiplicity, allowed threads, serialization or concurrency, reentrancy, and the unregister protocol. Borrowed callback environments are valid only if their scope includes every possible invocation.

Unregistration must establish that no future or in-flight callback can use the context before it is freed. A callback running on a foreign thread must enter the language runtime through an approved trampoline. Thread-affine handles cannot silently be marked transferable.

Reentrant calls cannot expose temporarily broken invariants or reacquire an incompatible borrow or lock. The wrapper may forbid reentrancy, serialize callbacks, enqueue owned messages, or provide a proved reentrant protocol. Merely tagging a callback safe does not solve this problem.

# 6 Proof and trust profiles

| Profile | Accepted basis |
| --- | --- |
| Verified foreign implementation | Evidence connected to the shipped implementation and its ABI semantics |
| Audited native contract | Explicit trusted contract, identified transitively in the artifact manifest |
| Validated isolated service | Checked transport, decoding and resource boundary; functional results remain limited to validated claims |

All profiles can be useful. A build policy selects which are allowed. Contract identifiers and version ranges must refer to an established compatibility policy; a version string or binary hash identifies code but does not prove its semantics.

Refinements from foreign data must be checked or obtained under an accepted foreign theorem. Strings, enum tags, lengths, and serialized dependent witnesses require decoding rules. Arbitrary foreign bytes cannot be cast into a language proof object.

# 7 Binding generation and linking

Compile-time generators MAY read headers, WIT, library metadata, schemas, and tool output. Recorded inputs include the target ABI, generator version, link options, and relevant library identity. Generated declarations and wrappers go through ordinary checking; generated trusted contracts are visibly marked and policy-controlled.

Static, dynamic, and host-provided linking are supported deployment choices. The linker or loader checks compatible symbols, layouts, capabilities, and allowed library identities. Libraries compiled to Wasm are not interchangeable with native `.so` or `.dylib` files; a Wasm build needs target-compatible code or an explicit host import.

# 8 Hot replacement and foreign code lifetime

Foreign registrations are dependencies of the owning code version. A module cannot unload while a library may call its trampoline or while a resource still needs its destructor. Upgrade packages must drain, transfer, or preserve those registrations and resources.

Replacing a library's dispatch pointer does not update already-running native calls. Live handles created by an old library cannot be handed to the new library unless a compatibility or migration contract permits it. A callback and a destructor can require keeping an old library loaded long after the last ordinary call returns.

# 9 Initial scope and conformance

Start with C shims, opaque owned handles, call-scoped buffers, explicit errors, and serialized callbacks. Add more complex retained buffers and concurrent callbacks once their protocols are modeled.

Required rejection cases include wrong deallocation, a callback outliving its environment, a language proof decoded from unvalidated bytes, a thread-affine handle moved to a worker, and freeing an output buffer immediately after requesting cancellation of a still-running foreign call.

The [Rustonomicon FFI chapter](https://doc.rust-lang.org/nomicon/ffi.html) is a practical reference for native boundary concerns. This specification defines its own dependent contracts and upgrade obligations; it does not claim that importing an existing library verifies it.

[Documentation overview](../overview.md)