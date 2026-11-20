# Provider-internal boxed natural comparison

This planned regression calls the actual admitted base object's `Data.Nat.nat-cmp` worker through its represented C ABI. The source does not implement a comparison or emit a consumer-local runtime helper. Before linking, the runner must admit the provider and verify its genuine defining `Nat -> Nat -> I64` scheme, arity 2, parameter classes `[T,T]`, raw result class `R`, ownership/lent metadata and exported object symbol.

The matched runtime constructs fresh boxed values at 2^63, beyond 2^64 and at 2^127. Each comparison transfers both owned operands to the worker; the shared-operand case explicitly duplicates the reference first. Reclamation is enabled before allocation, and the matched runtime performs final zero-leak accounting. Immediate values use the actual 63-bit natural representation, including the all-ones encoding of the maximum immediate. These are mathematical naturals, whose immediate range differs from mathematical integers.

Run every observation independently with C caller optimizations 0 and 2, normal 32M and poisoned 256M memory limits, and 8M program stacks. Link only the admitted object and its matched runtime, without duplicate-definition overrides. Retain object metadata, object/LLVM/runtime/source hashes and direct-worker disassembly.

The old V4 LLVM declaration incorrectly claims `readnone`, although its current optimized object worker happens to compare before releasing either operand. This fixture therefore must not invent an old runtime failure. The independently demonstrated original Rust consumer O2 failure remains the bug evidence. A V5 artifact must contain the corrected declaration and pass the actual provider-worker observations.

Current status: C syntax checked only. No object link, runtime observation, V5 artifact or Native-family gate has run for this fixture.
