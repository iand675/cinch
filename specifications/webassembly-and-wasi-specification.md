# WebAssembly and WASI specification

Source: <https://chatgpt.com/space/page_93721e2e8a6081919c8e74d0c3f11211>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2 extension. WebAssembly and WASI are first-class compilation and interoperability targets. This chapter proposes artifact profiles, binding rules, and runtime obligations. Standards status was checked on 22 October 2026; exact implementation support is a target-profile requirement rather than a universal assumption.

# 1 Standards baseline

[WASI 0.3.0 was released on 11 June 2026](https://wasi.dev/releases/wasi-p3), including Component Model async functions, futures, and streams. This draft therefore proposes WASI 0.3 as the preferred component profile, with a pinned WASI 0.2 compatibility profile and explicit adapters where their semantics can be preserved. Exact WIT package versions and engine capabilities are recorded in each artifact.

Core Wasm, the Component Model, WASI interfaces, browser host APIs, and shared-memory threads are separate dimensions. Supporting one does not imply all the others. A browser deployment needs an explicit browser host binding or component adapter; it must not assume access to the server's WASI filesystem or sockets.

# 2 Artifact profiles

| Proposed profile | Intended output and environment |
| --- | --- |
| Core Wasm | A module with explicit imports and exports for embedding |
| Browser | Core module plus generated host bindings and an explicit browser capability policy |
| WASI component | Component Model artifact, WIT world, canonical bindings and pinned WASI dependencies |
| WASI 0.2 compatibility | Versioned older interfaces and checked or explicitly trusted adapters |

Each profile lists optional features such as SIMD, relaxed SIMD, shared memory, atomics, exception handling, memory64, GC, and stack switching. The compiler MUST reject a missing required capability or select a supported lowering. These are design profile names, not claims about existing compiler command-line flags.

The initial portable runtime should not require shared-memory threads or a particular GC extension for basic programs. A linear-memory implementation with explicit task state machines is a candidate baseline; faster host features may specialize it under their own proof and trust contracts.

# 3 Representation and dependent boundaries

The compiler MUST define how owned allocations, references, function values, opaque resources, and runtime type indices are represented. Proof erasure remains governed by the core rules. Exported APIs do not expose arbitrary dependent terms through an ordinary WIT signature.

```wit
package example:codec@0.1.0;

interface decoder {
  enum decode-error { malformed, too-large }
  decode: func(input: list<u8>, limit: u32)
    -> result<list<u8>, decode-error>;
}

world codec-plugin {
  export decoder;
}
```

This is an interface example, not an implementation. WIT describes shapes and resource interfaces; additional functional laws require a side contract, validated values, or checked proof artifacts. A wrapper can validate that returned bytes satisfy `length <= limit` and return an existential length-indexed value. It cannot infer that the bytes are the correct decompression solely from their length.

For same-language components, optional proof metadata may travel alongside the interface, but the consumer must check evidence, semantics versions, dependencies, and artifact identity. Metadata saying verified is not evidence.

# 4 Canonical ABI and resources

Generated import and export bindings MUST implement the selected Canonical ABI version, including value conversion, allocation, cleanup and resource behavior. WIT resources map to opaque owned or borrowed handles under the adapter's exact lifetime rules. They are not arbitrary host pointers.

Strings, variants, lists and integers need checked conversion. Zero-copy requires a specifically supported sharing arrangement and proof; it is not a default consequence of matching element types. Memory growth and asynchronous suspension can affect host views and retained buffers, so adapters must preserve the required lifetimes.

The [WIT reference](https://component-model.bytecodealliance.org/design/wit.html) defines the interface constructs, including resources and ownership transfer. These constructs do not replace the language's semantic and temporal contracts.

# 5 Capabilities and isolation

A component world lists its imports. Linking supplies concrete capabilities such as a permitted directory, clock, entropy source or network service. A generator, test or deployed plugin does not acquire all host authority merely by targeting WASI.

The host runtime and import implementations are part of the trust boundary. Wasm's sandbox boundary does not prove memory safety between allocations inside a guest linear memory. Unsafe foreign code sharing the language heap can corrupt that heap; put untrusted code behind a separate instance/component with separated memory or an external process when that isolation is required. Imported shared memory and powerful host APIs can deliberately weaken that separation.

Fuel limits, deadlines and host termination controls can bound execution resources under a specified engine policy. They do not prove semantic termination, undo external effects, or guarantee guest destructors ran. Trap and forced-termination recovery must account for resource revocation at the host boundary.

# 6 Async semantics

The proposed language adapter maps a WASI 0.3 future to a fallible asynchronous completion and a WASI stream to an asynchronous source or sink. A WASI stream may end, fail or wait indefinitely; it MUST NOT automatically become the language's productive infinite `Stream a`.

Async imports integrate with the language scheduler's wakeups, backpressure and cancellation protocol. If an API reports data through a stream and completion through another value, the adapter must preserve both channels: end of data alone is not necessarily successful completion.

WASI 0.2 adapters must preserve their declared readiness, close and error behavior. An adapter that cannot preserve cancellation or another required guarantee must expose the weaker guarantee instead of claiming equivalent semantics. Component async scheduling does not by itself establish CPU parallelism or a universal fairness theorem.

# 7 Concurrency and browser execution

Structured concurrency MUST work on a single-threaded cooperative backend as well as supported multithreaded backends. Thread migration, shared-memory atomics, and true parallel execution require explicit backend capabilities and appropriate transfer constraints.

Blocking operations must not freeze a cooperative event loop under an API promising asynchronous progress. Browser and component profiles may need state-machine lowering or host-driven suspension. Optional stack switching is a lowering choice, not an excuse to make programs depend on unsupported instructions.

Foreign promises and callbacks have the FFI retention rules. Requesting cancellation does not authorize releasing a buffer still retained by an asynchronous host operation.

# 8 SIMD and feature selection

Wasm SIMD rewrites need proofs against Wasm instruction semantics. The engine's subsequent native lowering remains a separate trust or verification boundary. Relaxed SIMD requires its own allowed-results contract; it cannot be substituted into an exact-result API without evidence.

A module containing unsupported instructions can fail validation before its dispatch branch executes. Therefore a portable distribution may require separate baseline and feature-specific modules selected before instantiation, rather than putting unsupported instructions behind a Boolean branch in one module.

The loader records the selected feature profile. Backend-specific type sizes and memory64 assumptions cannot be imported from the compile-time host. SIMD kernels retain the bounds, tail, alignment and ownership obligations in the platform specification.

# 9 Replacement and component state

Wasm does not supply our checked hot-swapping protocol automatically. The language runtime must provide versioned dispatch, state export/migration or stable handle adapters, pending-task coordination, and old-instance retention where needed.

Component interface compatibility is necessary but insufficient. Replacing an instance must preserve resource identity or provide a checked transfer, handle in-flight imports and callbacks, and maintain pending temporal obligations. Raw linear-memory copying is not a general migration algorithm for pointers, resource tables, async frames or host handles.

Forced destruction can contain a failed component, but it does not establish that pending external operations were rolled back. Restart policies must preserve unknown outcomes and deduplication state when the protocol requires them.

# 10 Required tooling and acceptance cases

The toolchain SHOULD import and export WIT, generate browser bindings, report required features and capabilities, preserve source-level diagnostics, and emit reproducible component artifacts. Tests should cover both embedding and the selected WASI worlds, with validation and execution on pinned runtimes.

Acceptance cases include a core pure function, a capability-scoped file operation, an async stream with a distinct completion error, a component resource's lifecycle, and a baseline/SIMD distribution. Rejection cases include unsupported required features, malformed foreign values, a productive-stream claim for an arbitrary async source, and an update that loses a live resource table.

No Wasm compiler or runtime implementation has been built by this documentation task. These are conformance requirements for the planned toolchain.

[Documentation overview](../overview.md)