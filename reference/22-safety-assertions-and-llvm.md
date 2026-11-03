# Safety assertions, must-use results, and LLVM contracts

Safety assertions are checked at definitions and recorded as contract facts. They let callers depend on an admitted behavioural property without treating a performance hint as a proof.

For the complete `#[...]` vocabulary, including hints, metadata, layout, and foreign crossing attributes, use [attributes and pragmas](23-attributes-and-pragmas.md). For the memory representation itself, use [type layout](24-type-layout.md).

# Compiler-owned assertions

The current Rust compiler accepts these function-level attributes. Parenthesized argument forms below match the executable tests; the attribute parser also admits its documented literal-argument surface.

| Attribute | What is checked |
| --- | --- |
| `#[recursion("none")]` | No recursive call in the checked call graph. |
| `#[recursion("tail")]` | Admitted self/mutual recursion remains in tail position. |
| `#[recursion("bounded(n)")]` | A static depth bound, using admitted structural/conservative reasoning. |
| `#[stack("constant")]` | The admitted non-growing-stack discipline. |
| `#[stack("bounded(n)")]` | A supported bound on stack growth, with the proof/reason reported. |
| `#[no-alloc]` | No admitted allocation site/call violating the analysis contract. |
| `#[alloc("once")]` | The admitted per-call single-allocation-site bound; recursive reallocation is not silently ignored. |
| `#[max-complexity(n)]` | Cyclomatic complexity, measured as branches plus one, is at most `n`. |
| `#[max-lines(n)]` | The checked source definition fits the line bound. |
| `#[assert-min(n)]` | At least `n` admitted runtime assertion/decision checks; erased proof obligations do not count. |
| `#[checked-params]` | Parameters are validated before the uses covered by the analysis. |
| `#[no-macros]` | No macro expansion in the definition's recorded provenance. |
| `#[no-raw-ptr]` | No admitted raw-pointer/opaque signature or body operation violating the restriction. |
| `#[deny-warnings]` | Diagnostics in scope follow the warnings-as-errors policy. |

A violation names the attribute and offending site. An unsupported bound is refused or conservatively checked; the compiler does not accept an arbitrary numeric claim because it is written in an attribute. Source-level allocation analysis and runtime byte accounting answer different questions: consult the recorded assertion status and measure dynamic allocations when performance depends on them.

# An asserted function

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[recursion("none")]
  #[no-alloc]
  identity-word :: U32 -> U32
  identity-word x = x

  main :: Int
  main = if identity-word 42 == 42 then 0 else 1
```

The function has no recursion or construction. Its verified facts are retained in the interface. By contrast, `#[inline(always)]` only influences representation and does not prove non-allocation, termination, or constant stack use.

# Must-use Result values

A `Result` must be handled or explicitly ignored. A wildcard binding is not an implicit acknowledgment of an error result:

<!-- cinch:reject contains="without `ignore`" -->
```cinch
module Main where
  attempt :: Int -> Result Int Int
  attempt n = Ok n

  main :: Int
  main = let _ = attempt 1 in 0
```

Use `case` to select success/failure behaviour. If deliberately discarding both outcomes is the stated application policy, write `ignore (attempt 1)` explicitly. `ignore` does not convert an error into success or establish a proof about the operation; it records the deliberate discard. This discipline is separate from the linearity of task handles.

# The JPL profile

`#[profile("jpl")]` on a module header enables the implemented profile checks: bounded recursion, no allocation outside admitted initialization, complexity/line bounds, runtime assertion minimums on the relevant public functions, no macros/raw pointers, and warnings policy. Individual assertions may strengthen the profile; they cannot silently weaken its requirements. The report explains whole-program/definition outcomes.

This is a compiler profile inspired by the rule set. It does not certify a product for any external standard, prove external C code correct, or replace the explicit analysis boundaries above. There is no separate `cinch --profile jpl` CLI flag today.

# LLVM facts are generated output

The backend translates established analyses into LLVM's vocabulary. These names describe emitted IR; they are **not** a general set of accepted source attributes such as `#[nonnull]`.

| Proven input | LLVM output in the admitted lowering |
| --- | --- |
| Appropriate empty effects / memory behaviour | `memory(none)`, `speculatable`, or the corresponding read/readwrite effect. |
| Verified no-allocation / return facts | `nofree`, `willreturn` where established; never guessed for arbitrary recursion. |
| Usage and boundary non-escape facts | Capture constraints such as `captures(none)`. |
| Scalar/width and constructor facts | `noundef` and admitted `!range` metadata. |
| Layout and pointer-boundary facts | Alignment, nonnull/dereferenceability facts, admitted `llvm.assume` bundles. |
| Foreign aggregate ABI | `byval` and `sret` where requested/admitted at the boundary. |
| A fault exit | `noreturn` and an unreachable successor. |
| Untrusted buffer/pointer reads | `freeze` where required before sensitive uses. |
| Proven bounded loop progress | Admitted loop progress metadata. |
| Immediate primitive operands / assembly | Immediate-argument metadata and side-effect preservation. |

The IR tests check both presence where justified and absence where not justified. Fast-math is not a default permission to reassociate IEEE arithmetic. Reserved/opaque primitive bodies must retain their explicit trust and containment status.

<span class="support-tag support-tag--planned">Coming soon</span> broader primitive semantic proofs and solver coverage remain in the [verified primitives](../design/verified-primitives.md) and [proof automation](../design/proof-automation.md) designs. Existing emitted LLVM annotations must not be described as those future proofs.

Implementation: `compiler/src/check_decls.rs` (safety attributes/analysis), `check_terms.rs` (must-use), `core.rs`, `codegen.rs`; behaviour suites: `safety.rs`, `llvm_contract.rs`, `attributes.rs`, `tail_calls.rs`.
