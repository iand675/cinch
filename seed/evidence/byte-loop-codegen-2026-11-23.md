# Byte-loop code generation, 2026-11-23

A callgrind profile of `Text.Bzip2` (branch `claude/keen-galileo-w6sob2`)
decoding a 2 MB corpus put the time in generated-code overhead rather than in
the decoder. This note records what each item turned out to be, what changed,
and the measurement.

## Workload

`Text.Bzip2.bzip2-decompress` over 2,000,000 bytes: 300,000 bytes of each of
the six `tooling/bzip2_vectors.py` corpus shapes, followed by 200,000 bytes of
`seed/src/codegen.rs`, compressed by libbzip2 at level 9 to 422,872 bytes. The
program reads the file with `Effect.Fs.fs-read`, decodes it, writes the output,
and the output is compared with `cmp` against the corpus. Linux x86-64, 4
cores, clang 20.1.2. Seven runs per binary; the before binary is the seed at
`d8fc8500`, the after binary is this change.

| link | before, median (min) | after, median (min) | instructions before | instructions after |
| --- | --- | --- | --- | --- |
| default (`-O0`) | 748 ms (708) | 459 ms (423) | 7,157,344,965 | 5,592,177,025 |
| `--opt-level 2` | 171 ms (167) | 165 ms (156) | 2,022,662,349 | 1,907,915,804 |

The default link is the one `cinch build`/`cinch run` and the test harness
use: `opt_level` defaults to 0 (`seed/src/driver.rs`, `link_target`), and an
unoptimized link runs only LLVM's always-inliner. That is why the original
profile showed every primitive unit as its own function. At `-O2` LLVM
already inlined most of it, so the gain there is small.

## What each item was

**`cinch.prim.*` helpers as calls.** A saturated pure scalar primitive was a
`call` to its unit, and the units carry no inline hint, so at `-O0` every
`ltI64`, `leI64`, `wrappingShrU32`, `wrappingU32ToU8` was a call. Now
`open_scalar_prim` names the branch-free, allocation-free primitives
(integer comparisons, wrapping arithmetic and negation, saturating negation,
wrapping shifts, rotates, bit operations, bit scans, total, wrapping and
saturating integer conversions, the Bool operations, `charToU32`), and
`scalar_prim_value` emits them. `CoreEmitter::emit_app` computes a saturated
one in place, and the unit body is the same function over `%p0..`, so the two
cannot disagree. A tail position takes the value arm, since an instruction is
not a tail call. After the change the decoder's IR has no `call` to a
`cinch.prim.*` unit; before it had 543.

**`cinch.rc.drop.tobj` with no caller.** These were not the `Bytes` handles.
They were `Nat` temporaries: indices, lengths and `natCmp` operands, dead
after one use. Every one is a `TObj` drop because a `Nat` may be a bignum.
The helper was one call per operation on purpose, which keeps the IR
proportional, and an optimized link inlines it. At `-O0` it was a call even
for an immediate word, about 100 million of them. The `TObj` helpers are now a
three-instruction tag test marked `alwaysinline` in front of the `Obj` helper,
which a counted word reaches out of line. The IR still has one line per
operation. The self-hosted backend's `cinch.typed.rc.*` helpers were already
`alwaysinline`. After the change `drop.obj` is 0.3% of the profile and
`drop.tobj` no longer appears.

**`bytes-set` at about 120 instructions.** The seed's `bytesSet` already had an
inline fast path. The cost was the unoptimized code around it and the four or
eight calls per word write. `Data.Bytes.Word.bytes-write-le32`/`le64` now each
make one call.

**`4 * i` calling `cinch_nat_mul`.** `natMul` had no immediate fast path,
unlike `natAdd`, `natSub`, `natCmp`, `natDiv` and `natRem`. It now has one:
`umul.with.overflow` on the two payloads plus the immediate range check, with
heap operands, overflow and bignum results going to the runtime. It is also
`alwaysinline`. In this workload it was 0.4%; the change is for the general
case.

**`cinch.runtime.bytesData` at 3%.** Its body is the same validated payload
read as `bytesLen`, so it joins the always-inlined buffer wrappers.

## The word stores

`cinch.runtime.bytesSetLe32 :: Bytes -> Nat -> U32 -> Unit` and
`bytesSetLe64 :: Bytes -> Nat -> U64 -> Unit` store the word's little-endian
bytes at any alignment.

- Seed (`seed/src/core.rs` `BytesOp::SetLe32`/`SetLe64`, `seed/src/codegen.rs`
  `bytes_access`): the inline path checks the buffer kind (writable heap
  bytes) and `i <= len - w` with `len >= w`, then does one `store iN ...,
  align 1`, byte-swapped first when `TargetProfile::endian` is `big`.
  Anything else calls `cinch_bytes_set_le32`/`le64`
  (`seed/runtime/src/abi.rs`, `Bytes::set_word`), which raises the same
  faults `bytes-set` does.
- Compile-time evaluator (`seed/src/check_decls.rs`): the same bytes, low to
  high, whatever the compiling host's byte order.
- `Data.Bytes.Word`: the public writes keep their `using (pos + w <=
  bytes-len b)` contracts and call a private, unconstrained foreign import.
  Contracted intrinsics are refused by the self-hosted checker ("runtime
  intrinsic cannot be polymorphic or constrained").
- Self-hosted compiler (`Bootstrap.Builtin` contract, `Bootstrap.Core` write
  effect, `Bootstrap.RuntimeIr` `word-store`): Stage 2 and Stage 3 link the
  pinned library provider's runtime, which has no `cinch_bytes_set_le*`. The
  worker therefore uses externs that runtime already exports.
  `cinch_bytes_set` at the range's last index validates the buffer and the
  range and faults like a byte loop would, then one unaligned store writes
  the word. That is two runtime calls instead of four or eight plus their
  conversions. Moving it onto the new externs waits for a library-provider
  pin past this change.
- Pinned seed (`compiler/seed/stage0.patch`, `compiler/pins/stage0.lock.json`):
  Stage 1 is linked by the seed against the seed's own patched runtime, so
  the patch adds the two ops and the two externs, the same way it added
  `mapPackageBytes`.

`seed/tests/bytes_word_stores.rs` covers both backends: every offset 0 to 8 of
a 0xAA-filled buffer for both widths, compile-time evaluation (seed only), the
out-of-bounds fault, and the emitted unit (one store; `bswap` on a big-endian
profile; no `cinch.prim.*` call left in the loop).

## Gates

- Fixed point (`tooling/bootstrap.py check --no-pgo -j 2`): Stage 2 and Stage
  3 executables, `compiler.ll` and interface packages are byte-identical. The
  interface consumer check, the changed-source probe, and the fib, precedence
  and refusal smokes pass on Stages 0 to 3. Stage 3's `compiler.ll` contains
  the inline word-store worker, and `Bootstrap.PackageWrite` uses it. The
  baseline chain from `d8fc8500` was run the same way.
- `cargo test --lib`: 358 passed, 0 failed.
- Target set through Stage 3 (integration, ownership, modules,
  bootstrap_infer, bootstrap_typing, bootstrap_pipeline, bootstrap_resolve,
  prim): 7/8 targets, identical to the baseline chain. The one failure,
  `modules::instance_coherence_checks_flexible_peers_in_later_argument_positions`,
  fails the same way on the baseline.
- Leak line: Stage 3 `--emit-ir-only` on the compiler bundle exits 0 with
  `cinch-leak: live=0 immortal=0 leaked=0`, and its IR is byte-identical to
  Stage 3's `compiler.ll`.
- Seed suites: lib_bzip2, lib_word_ops, safe_bytes, word_constant_folding,
  reclamation, rc_specialisation, prim, primitives, lib_crc32, lib_sha256 and
  bytes_word_stores all pass (141 tests). bytes_word_stores and lib_word_ops
  also pass through Stage 3.

`tooling/run_stage_targets.py` runs Cargo from `<tree>/compiler`, the seed's
location before the `seed/` rename. These runs mapped that one working
directory to `seed/` with a wrapper and left the tool unchanged.
