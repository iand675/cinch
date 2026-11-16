# Saturated compiler-generated foreign aliases

Nested dependent arguments can introduce a temporary `#argument-function` holding a bare foreign `CUse`. The current foreign call path requires all checked ABI arguments, so trying to materialize that temporary previously failed before the correctly typed U8 argument reached its inline primitive. Explicit sequencing avoided the temporary.

The repair substitutes that pure static reference only when the checked binding supplies its exact foreign plan, the scheme is closed and monomorphic, the function has ordinary arrows, and the compiler temporary has one fully saturated use after its argument witness lets. The lets remain in their original order. The existing foreign arity checks, ABI marshaling and inline containment rules are unchanged. Escaped, repeated, branching, shadowed, polymorphic, constrained, dependent and partial-value shapes do not enter this path. Existing proper source-call fusion remains separate.

The checkpoint is a focused experimental result. Its full merged Native-family fixed point and parent preserved gates remain pending; it has not been published.

`foreign-function-alias-validation-results.json` records the exact production source, compiler, provider/runtime references and raw receipts under `foreign-function-alias-evidence/`.

- Production source `6d2da12d…` has 51 files and changes only `Bootstrap.RuntimeIr`. The experimental host `f23a11d5…` was linked O0 by the same frozen Rust producer `ae9c3f81…` and matched runtime `80bd5b54…` as the scalar checkpoint. The immutable source bundle passed that durable emit-IR/type-check step and link.
- At both consumer O0 and O2, 12 candidate source cases and seven baseline controls pass their expected verdicts. The exact original nested U8 source now compiles and runs; the exact sequenced source remains valid. Both run normal 32 MiB and poisoned 256 MiB heaps with 8 MiB stacks and zero leaks. The scalar baseline still rejects the original with exit5 and accepts the sequenced control.
- A separately compiled C trace observes actions 1,2,9 or 1,3,9 exactly once in source order. High bytes128/255, the empty branch and lexical function shadowing also return their independently expected values. Effects are explicit Global foreign effects; no assembly hides a side effect.
- Ordinary/repeated/branching foreign function aliases, higher-order foreign values and explicit proper partial foreign prefixes retain the same exit5 refusal under both frontends.
- Twenty private Core observations test the actual implementation with an appended test-only suffix, without adding a compiler API. Positive rewrites preserve result type, effects and free-local invariants. Repeated, escaped, branching, shadowed, overapplied, wrapped, wrong-role, instantiated, polymorphic, constrained, effect-mismatched, dependent, partial-RHS, partial-use and wrong-binding shapes are refused. Both ownership profiles pass with zero leaks.
- All 27 scalar authority observations remain green at consumer O0/O2. Two canonical documentation artifacts have no diagnostics or orphan comments.

The corpus links the explicitly admitted original phase-one provider `e70250a5…`, object `645a3d0e…`, runtime `80bd5b54…`, ABI10. This remains separate from the newer combined String provider and full family.

The maintained runners are `tooling/validate_foreign_aliases.py` and `tooling/check_foreign_alias_guards.py`. Source `/workspace/.tools/cinch-env.sh`, set the standard single-job/64 MiB compiler-stack environment, select the exact compiler/provider/runtime in the receipt, and use fresh artifact directories. The source runner supports `--opt-level 0` and `--opt-level 2`; the private runner builds a separate O0 test executable from the frozen producer. All source/call commands are retained in the raw receipts.
