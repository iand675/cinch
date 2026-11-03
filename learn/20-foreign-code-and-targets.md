# 20. Foreign code, layouts, and target profiles

A foreign boundary connects two independently compiled interpretations of memory and calling conventions. Matching function names is only one part of the contract: argument layout, ownership, effects, and unwinding policy must also agree.

# Declare a boundary

A declaration such as `foreign import ccall "foreign_add" foreign-add :: I64 -> I64 ->{Foreign Lib, IO} I64` names a C symbol and its Cinch type. It is a boundary fragment, not a standalone runnable example: linking also needs an object or library that supplies the symbol. The public CLI has no arbitrary object-file flag today; compiler APIs and the FFI tests supply objects explicitly.

Use ABI-appropriate machine types at the C boundary. Mathematical `Int` and `Nat` are bignum-capable values and are not interchangeable with C integer arguments. Records need an admitted representation and layout contract. Opaque handles and `RawPtr` values need explicit ownership roles and the required deallocator discipline. A raw address does not acquire a safe lifetime merely by being placed in a record.

# Imports, exports, and blocking

`foreign export ccall` exposes a checked Cinch definition through a C symbol. The foreign contract records the symbol, type, layout, and relevant effects in compiled interface metadata. A foreign function that may block must follow the blocking policy; a function that can fail or diverge must not be declared effect-free to make a call convenient.

Packed fields can require byte-exact copying. Returning an aligned borrow of an unaligned packed field is rejected. The no-unwind boundary prevents exceptions from crossing a calling convention that cannot carry them safely.

# Target evidence

The Rust compiler has target profiles controlling word width, backend features, admitted runtime evaluation, and target rewrites. `Usize` and `Isize` depend on the profile. Target-specific code must require evidence for the features it uses rather than silently assume the build machine's capabilities.

The implementation includes a Wasm32 profile and a WIT/canonical-ABI component subset. A `WasiStream` or `WasiFuture` carries weaker external observations than a productive `Stream`; importing it does not establish a productivity theorem.

<span class="support-tag support-tag--planned">Coming soon</span> **target selection in the CLI.** `cinch build --target ...` and an end-user component packaging command are not current commands. The target/profile APIs and tests exist in the Rust implementation. The guide explains that current surface separately from the intended portable deployment workflow.

# Exercise and answer

Why is it wrong to replace an `I64` foreign parameter with `Int` because both can represent the values used in your tests?

<details><summary>Answer</summary>

The calling convention depends on representation, not merely the set of mathematical values. `Int` can use an immediate or heap-backed bignum representation. C expects the agreed machine ABI, so the foreign declaration must use an admitted boundary type and an explicit checked conversion where needed.

</details>
