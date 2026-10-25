# Tutorial 11 Foreign libraries

Source: <https://chatgpt.com/space/page_eb03321bfde48191bcd197291ad9eadf>  
Exported: 2026-10-22T18:59:36Z

---

Use an existing native library through a small, explicit boundary. This tutorial uses a fictional C codec API; the syntax and wrapper APIs are proposals.

# Start with the public contract

The application should see an owned decoder and a typed result, rather than pointers and integer status codes.

```haskell
decode :: forall region. Mut region Decoder -> Ref region Bytes
       -> (limit :: Nat) ->{Foreign Codec, Blocking, Diverge}
          Result DecodeError (BoundedBytes limit)
```

`BoundedBytes limit` contains bytes together with evidence that their length is at most `limit`. This says nothing yet about whether the bytes are a correct decompression. A stronger interface can add that semantic relationship when evidence or an explicitly accepted assumption supports it.

The effect signature records that the call crosses into foreign code, may block, and may fail to terminate. `Result` records ordinary reported errors. A crash or undefined behavior inside an unverified native library does not become a recoverable `Err` merely because the wrapper returns `Result`.

# Describe the raw ABI separately

The raw declaration must specify the calling convention, integer widths, pointer validity, output initialization and allocator pairing. The wrapper must establish the input preconditions and interpret successful outputs according to the foreign contract.

For this codec, the assumed contract is: input and output pointers are used only during the call; the codec never writes beyond the supplied capacity; success initializes exactly the reported prefix; and destruction must use the codec's own close operation. These are reviewable assumptions unless proved against the linked implementation.

The wrapper allocates storage, lends it exclusively, calls the decoder, checks the status and reported length, and exposes only the initialized prefix. A successful length check establishes the returned length bound. It does **not** establish that the callee avoided an earlier out-of-bounds write. A safe public wrapper over native code is therefore conditional on the foreign implementation meeting its safety contract.

# Choose the trust profile

The build can accept an audited native codec, require a verified implementation with matching artifact evidence, or call an isolated worker/component whose outputs are validated. Isolation changes the failure boundary; it does not prove that decoded content is semantically correct. The manifest records which profile the application chose and its transitive dependencies.

Generated declarations help with layout and naming. They cannot infer a correct ownership or threading contract from a C header alone.

# Handle blocking and cancellation

Put blocking work on an executor that supports it. Move an owned input buffer into the operation, or keep its borrow alive until completion. If the task receives cancellation while the C function is running, the runtime requests cancellation if supported and waits for the protocol's completion acknowledgment before releasing memory. A timeout is not acknowledgment.

Callbacks need a stable trampoline and a registration resource. Closing that resource must establish that no future or in-flight callback can use its context. When that context refers to replaceable code, the old code stays resident until registration and execution have both ended.

# Check your understanding

Would changing the result type to `Result<DecodeError, Bytes>` remove the need to trust the native library's memory behavior? No: weakening the result contract removes a length theorem, not the pointer and lifetime obligations. To reduce that trust, change the implementation evidence or isolation boundary.

See the foreign interface specification for layouts, callbacks, unwinding, linkage and deployment policy. The source example is `examples/09-foreign-codec.lang`.

[Documentation overview](../overview.md)

# Continue reading

[Tutorial 10 Platform specific SIMD](10-platform-specific-simd.md) · [Tutorial 12 WebAssembly components](12-webassembly-components.md) · [Language design overview](../overview.md)