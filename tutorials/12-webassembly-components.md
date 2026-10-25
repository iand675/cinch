# Tutorial 12 WebAssembly components

Source: <https://chatgpt.com/space/page_f0fa8d6024188191862d6256eb506b72>  
Exported: 2026-10-22T18:59:36Z

---

Export a library as a Wasm component while keeping its stronger source-language contracts visible inside the program. This tutorial uses a portable, synchronous WIT interface; async APIs can be added with the WASI 0.3 profile.

# Define the interoperable shape

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

WIT describes the values and interfaces that another language can call. It does not express the dependent claim that the returned list has length at most `limit`, nor that it decodes the input correctly. Our generated source signature can expose a stronger result when the local implementation proves it.

# Generate and check an adapter

On export, the adapter translates the checked source result into canonical component values. On import from an arbitrary component, it starts with the weaker WIT result. It checks the length before constructing `BoundedBytes limit` and returns an additional boundary error if validation fails.

The adapter must also establish integer conversions, string or list representation, resource ownership and cleanup. Proof terms are erased; essential runtime tags, lengths and discriminants remain. Passing a dependent value does not imply sending its proof object over the ABI.

The isolated implementation may trap or terminate without producing a result. Treat those outcomes according to the host's component-failure policy. A valid length is evidence about that returned value, not a proof that the external implementation is correct on all inputs.

# Choose a target and capabilities

Use the WASI 0.3 component profile for hosts supporting that interface set. Keep a pinned WASI 0.2 profile where compatibility is needed. For a browser, generate the selected component or core-module JavaScript adapter and request the corresponding host interfaces explicitly. Browser DOM access is a host binding, not an automatic WASI facility.

Wasm itself does not grant filesystem or network access. The component's world declares imports; the application supplies only the selected capabilities. A decoder needing no I/O should have no filesystem or network imports. Build-time schema generation has a separate capability environment from runtime execution.

# Add async deliberately

A WASI future can represent an eventual outcome; a WASI stream transports a sequence with host protocol rules. Neither is automatically our productive infinite `Stream a`. An adapter must account for end-of-stream, errors, cancellation and the separate completion outcome where the interface has one.

For a bounded consumer, backpressure is part of the contract. A cancelled consumer must release its registration or keep it alive until the host acknowledges completion. Async I/O can work on a single-thread event loop; parallel CPU work additionally requires a supported execution capability.

# Add SIMD and hot replacement

Ship a portable module and a SIMD-enabled variant when host support differs. Choose a valid variant before instantiation: placing unsupported Wasm instructions behind a runtime branch may still make the entire module fail validation.

Hot replacement happens through runtime-managed dispatch and resource migration. An old component resource cannot be reinterpreted as a handle owned by a new instance. Drain, translate, or retain it according to the upgrade package, including outstanding async operations.

The accompanying files are `examples/10-wasm-component.lang` and `examples/codec.wit`. See the WebAssembly and WASI specification for the full backend requirements.

[Documentation overview](../overview.md)

# Continue reading

[Tutorial 11 Foreign libraries](11-foreign-libraries.md) · [Tutorial 13 Structured concurrency](13-structured-concurrency.md) · [Language design overview](../overview.md)