# Separate Native MVar capability plan

This document specifies missing source and metadata work. No Native MVar
implementation, effect-allowlist change, emitted object or runtime test is
claimed. The seven-file source curation remains separate.

## Checked authority and carrier

Install kernel `MVarCell :: Type -> Type` in Infer and the nominal kernel-type
identity table. Reserve that defining identity against source declarations,
foreign nominal headers and aliases, using the same identity checks as other
kernel carriers. The public `Effect.Concurrent.MVar.MVar a` remains a distinct
source datatype with a private MkMVar wrapper. Neither the public spelling
MVar nor a user effect with that display name grants kernel authority.

Install the **zero-argument kernel effect MVar** consistently in source row
conversion and checked Core typing. Preserve row kind checks and require the
actual checked kernel identity. The Rust exporter can later gain this one
closed kernel-row encoding only in a separate reviewed patch with forged
same-display-name negatives; do not accept an arbitrary serialized MVar label.
Closed MVar exports need their exact foreign and resolved schemes/identities
and payload contracts, while unresolved/open rows keep their current refusal.
No Cancel region argument is invented for the -or-cancel operations.

## Exact intrinsic contract table

All entries have exactly one Type parameter a. `cell` below is the checked
kernel `MVarCell a`. The arrow's effect applies only to the final argument.

| Name | Arguments | Result | Effects | Consumed value argument |
| --- | --- | --- | --- | --- |
| mvarNew | a | cell | MVar | 0 |
| mvarNewEmpty | Unit | cell | MVar | none |
| mvarTake | cell | a | MVar,Diverge | none |
| mvarPut | cell,a | Unit | MVar,Diverge | 1 |
| mvarRead | cell | a | MVar,Diverge | none |
| mvarSwap | cell,a | a | MVar,Diverge | 1 |
| mvarTryTake | cell | Option a | MVar | none |
| mvarTryPut | cell,a | Bool | MVar | 1 |
| mvarTryRead | cell | Option a | MVar | none |
| mvarIsEmpty | cell | Bool | MVar | none |
| mvarTakeOrCancel | cell | Option a | MVar,Diverge | none |
| mvarPutOrCancel | cell,a | Bool | MVar,Diverge | 1 |
| mvarReadOrCancel | cell | Option a | MVar,Diverge | none |
| mvarSwapOrCancel | cell,a | Option a | MVar,Diverge | 1 |

Foreign symbols are exactly `cinch.runtime.` plus the names in the table.
Require the canonical scheme, full saturation, checked type-class vector,
effects and runtime ABI. Incorrect effects, carrier, arity or result wrapper
must be refused, not translated by spelling. A proposed `mvar-plan` registry in
Builtin should be the single source for checker and emitter plans, following
existing checked array operations without using their wide-element refusal as
MVar semantics.

## Word runtime lowering and ownership

Current Rust/runtime is the reference for this actual ABI, not permission to
load an arbitrary row:

- `cinch_mvar_new(value, format, full)` returns a fresh cell. Formats are 0 raw,
  1 owned object, 2 tagged/counted. Empty initializes value0/full0; zero remains
  a valid full raw value and never represents absence.
- `cinch_mvar_access(cell, value, operation, mode, &out)` borrows the handle;
  runtime operation0 take,1 put,2 read,3 swap,4 is-empty. Mode0 waits,1 tries,
  2 permits cooperative cancellation. Status is separate from the output word.
- Read produces its own owned duplicate while under the runtime mutex. Take
  and swap transfer the removed value's ownership. Put/swap/try-put and their
  cancellation forms consume the supplied value on every outcome, including
  rejection/cancellation. Lowering must not drop it a second time.
- Optional results branch on status before loading out; None never reads an
  uninitialized output. Some has the actual source Option constructor layout.
  Bool uses the existing checked bridge; Unit returns0 independently of status.
- The runtime marks every published value shared and owns queued references.
  The cell's kind18 destructor, FIFO queue and park/scheduler paths must come
  from the exact admitted new runtime. An old ABI10 archive without MVar fails
  capability/source admission; matching ABI10 is insufficient.

Blocking entries are neither pure nor nonrecursive guarantees: opaque runtime
calls may schedule callbacks. Do not add readnone, readonly, willreturn or
norecurse from this table. Preserve actual source effects and existing ARC
argument order for partial/higher-order application.

## Wide payload bridge

I128/U128 cannot occupy one unboxed runtime word. Use the same owned16-byte
wide box as the validated Rust MVar/Slots bridge: physical runtime class Obj,
source ABI i128. New/put/swap box a complete i128; read/take/swap unbox owned
results and drop the box reference; optional variants construct Some@W inline
only on status success. NewEmpty retains formatObj but no payload reference.
Bulk operations are runtime queue operations, not unchecked copies of two
inline scalar limbs.

Native's present generic wide boundary restrictions are a concrete additional
implementation task. A dedicated MVar class planner and marshaling path must
prove this boundary end-to-end. Until then, refuse the precise unsupported
MVar wide payload by name and keep complete MVar payload support **open**;
never classify an i128 as Raw or silently pass a truncated i64. A word-only
prototype can be useful as an intermediate tested patch but cannot qualify the
complete public feature.

## Smallest meaningful gate sequence

1. Source-only positive contracts for all14 imports with raw zero/all-one I64,
   counted Bytes, boxed Nat, Unit and wide signed/unsigned payloads; exact arity,
   pure-row, wrong carrier, erased Option and effect-forgery negatives. Repeat
   through aliases and qualified/reexport imports without restamping headers.
2. Actual source-absent producer/consumer tests carrying the closed MVar effect,
   private kernel nominal identity and public wrapper. Mutate one identity,
   effect, payload, foreign row or runtime claim at a time and require refusal.
3. Emit IR and verify every wrapper's class/prototype, consumed-argument map,
   Bool/Unit/Option bridge and no load of out on failure. Include partial and
   higher-order use with ordered, exactly-once independent C traces.
4. Runtime nonblocking full/empty transitions for all14 operations' reachable
   cases, read alias retention, zero as data, failed try-put consuming a fresh
   Bytes value, swap/take ownership, destructors for full and empty cells and
   source wrapper copies. O0/O2, checked RC, normal/poison, zero leaks.
5. Runtime scheduler/FIFO, blocked read/take/put/swap and cancellation/handoff
   races with independently observed traces. Share rejection for unshareable
   payloads and successful task handoff for admitted payloads. Existing Rust
   runtime MVar155 tests and frontend MVar tests remain separate evidence until
   the new Native compiler executes the same contracts with its exact runtime.
6. Wide MAX/MIN and mixed high/low limbs through all operations, including
   failed/cancelled puts and surviving read aliases. No generic Array<Wide>,
   packed-wide or source proof contract is implicitly granted by this bridge.

Only after these independent gates should the complete new source51/provider66
family be built and its fixed point and changed-source checks attempted. The
current static curation and any Rust66 checkpoint do not satisfy this plan.
