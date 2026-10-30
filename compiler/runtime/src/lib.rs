//! Runtime support library for generated Cinch code.
//!
//! `cinchruntime` is linked into every compiled Cinch artifact (the driver
//! links `libcinchruntime.a` against the generated LLVM IR objects). It is
//! written in Rust with no third-party dependencies and builds both as a
//! `staticlib` (for linking) and as an `rlib` (so compiler-side tooling can
//! use the same value semantics, for example to evaluate literals at
//! compile time).
//!
//! This module comment is the runtime ABI reference: the code generator and
//! the compiled interface metadata cite it.
//!
//! # Word representation
//!
//! Every Cinch value is one 64-bit machine word (`i64` at the ABI),
//! matching `compiler/src/core.rs`:
//!
//! * `Int` (the 64-bit machine integer of the surface prelude): the `i64`
//!   itself.
//! * `Bool`: 0 or 1. `Unit`: 0.
//! * ADT values: `ptrtoint` of a heap or static constructor block
//!   `{ i64 field0; ... }` whose header carries the constructor's layout
//!   id, and with it the tag (the constructor index in declaration order),
//!   instead of a tag word (see Block header). Generated code allocates
//!   them through [`cinch_alloc_ctor`]; this runtime reads a tag only
//!   through `block::ctor_tag_of`.
//! * Function values and dictionaries: `ptrtoint` of their closure or
//!   record blocks (see `compiler/src/core.rs`).
//! * `Nat` and `Int` (the unbounded numbers): a small immediate or
//!   `ptrtoint` of a bignum heap block owned by this runtime (below).
//!
//! The unbounded `Nat` and `Int` of the core spec are distinct from the
//! 64-bit machine `Int` above; only the context says which is meant, and
//! their words are never confused because each is produced and consumed by
//! the correspondingly named entry points.
//!
//! # Small `Nat`/`Int` immediates
//!
//! Heap blocks are 8-aligned, so a `Nat`/`Int` word whose low bit is 1 is
//! never a pointer: it is the immediate `v << 1 | 1`, decoded by one
//! shift right (logical for `Nat`, arithmetic for `Int`).
//!
//! * A `Nat` immediate covers `0 ..= 2^63 - 1`, every value of the 63
//!   payload bits. That is exactly the non-negative `i64` range, so
//!   `cinch_nat_from_i64` never allocates.
//! * An `Int` immediate covers `-2^62 ..= 2^62 - 1`, the 63-bit two's
//!   complement payload. One tag bit leaves 63 bits, and two's complement
//!   keeps decoding to a single arithmetic shift and lets the fast paths
//!   add, subtract and compare decoded values in machine arithmetic.
//!
//! Every entry point returns the immediate whenever its result is in
//! range (the canonical form). A bignum block therefore only ever holds a
//! value outside the immediate range: a `Nat` of at least 2^63, or an
//! `Int` below -2^62 or above 2^62 - 1. Two words of the same in-range
//! value are the same word, and overflow of an immediate operation
//! promotes to a block, while a block result that falls back in range
//! (a difference, a quotient, a negation) demotes to the immediate. The
//! code generator folds a literal in range to its immediate constant
//! (`immediate_word` in `compiler/src/codegen.rs`), and the quote
//! evaluator decodes to the same words. A word that may be an immediate
//! MUST NOT be dereferenced without testing the low bit first.
//!
//! # Heap layouts
//!
//! Bignum blocks are allocated by this runtime and are opaque to generated
//! code. A `Nat`/`Int` word with low bit 0 points at a header block of kind
//! `KIND_NAT`/`KIND_INT` whose payload, at offset 0, is a Rust `Nat` or
//! `Int` value. A box carries no in-block kind word: the header's kind is
//! checked instead, so a corrupt or mismatched word still faults (see
//! Faults) rather than being reinterpreted. The payload is a
//! sign-magnitude bignum: a normalized base 2^32 `Vec<u32>` magnitude
//! (little-endian limbs, no leading zero limbs) plus, for `Int`, a negative
//! flag. Zero is non-negative and has the empty magnitude. The `Vec`
//! header inside the payload is a standard library implementation detail;
//! generated code MUST treat the block as opaque and only pass the word
//! back to this runtime. Words are immutable and may be shared. Generated
//! code never frees them yet; [`cinch_free`] can. A `Bytes` box
//! (`KIND_BYTES`, or `KIND_BYTES_STATIC` for a string literal's
//! data-section buffer) is the `repr(C)` pair `{ len; data }` at offset 0.
//!
//! Blocks allocated through [`cinch_alloc`] are zero-filled byte blocks of a
//! whole number of 64-bit words.
//!
//! # Block header
//!
//! Every block a word points at, heap or static, has one 64-bit header word
//! at offset -8 from the address the word holds. Every offset above and in
//! the code generator is relative to that address, so the header is
//! invisible to field access. The encoding (`compiler/runtime/src/block_kinds.rs`,
//! which the code generator includes verbatim) is, from the low bit. Each
//! width is a named `*_BITS` constant there and every shift and mask is
//! derived from the widths:
//!
//! | bits | field | meaning |
//! |------|-------|---------|
//! | 0..40 | `rc`, signed | 0 immortal (static blocks, interned `Sym` cells, `KIND_BYTES_STATIC` buffers); 1 when allocated; > 0 a single-threaded count up to `RC_MAX` (2^39 - 1); < 0 reserved for atomic counts. A count cannot overflow: the heap is capped below 8 * 2^40 bytes, so no increment needs a check, and there is no sticky state |
//! | 40..48 | `kind: u8` | [`block`]`::KIND_*`: ctor 1, closure 2, dictionary 3, memo 4, knot 5, nat 6, int 7, bytes 8, bytes-static 9, sym 10, code 11, boxed scalar 12, raw 13, freed 255 (0 is never a live block) |
//! | 48..64 | `layout: u16` | a constructor block's layout id; a closure's or dictionary's shape index; for every other kind, the payload size in 64-bit words. `0xFFFF` (`LAYOUT_BIG`) is the escape: the word at offset -16 holds an escaped descriptor (the three shaped kinds), or another kind's payload byte size (a payload of at least 0xFFFF words) |
//!
//! Block memory is only obtained and returned by `block::allocate`,
//! `block::allocate_shaped` and `block::free`, the one place a different
//! backing allocator plugs in.
//!
//! Constructor layout ids. The code generator gives each data type, at
//! each class vector its constructor units are copied for, one group of
//! `2^CTOR_TAG_BITS` (64) consecutive ids, `id = group << 6 | tag`, so
//! `tag = id - base` with the group's `base = group << 6`, read as `id &
//! CTOR_TAG_MASK` without knowing the type. A type of more than 64
//! constructors, or past the last of the 1023 groups, is escaped: its
//! header says `LAYOUT_BIG` and the word at -16 is the descriptor `index
//! << 40 | tag << 16 | 0xFFFF` (a 24-bit tag and a 24-bit table index). A
//! descriptor, regular id or escaped, is what generated code hands
//! [`cinch_alloc_ctor`]. Blocks the code generator builds without a
//! constructor unit in hand (the `Option`, `Result` and `Ordering` values
//! of primitives, tuples of joined tasks, canonical ABI lifts, boxed-out
//! boundary aggregates) take an anonymous group per pointer map, with the
//! prelude's tags; the task runtime reads only the `Result` tag, which
//! needs no group.
//!
//! Closure and dictionary shapes. A closure's layout is the table index
//! of its shape, the classes of its unit's parameters (its saved words
//! are the unit's leading arguments), and a dictionary's is its
//! instance's method-slot map; both escape like a constructor, with tag
//! 0. A closure's size is `3 + filled` words (its words 0..3 are code,
//! arity, filled); a dictionary's is its entry's.
//!
//! The layout table `@__cinch_layouts`, which generated code registers
//! with [`cinch_layouts_register`] from a global constructor, has one
//! `i64` entry per id and shape index, then the map extensions. An entry
//! is the payload size in words in its low 32 bits and the pointer map in
//! its high 32: two bits per payload word, word `i` at bit `32 + 2i`, `0`
//! never touched (`Raw`), `1` a counted pointer (`Obj`; 0 and immortal
//! blocks skipped), `2` an immediate when its low bit is 1 and a counted
//! pointer otherwise (`TObj`), `3` invalid. A block of more than 16 words
//! keeps the table index of its extension in those 32 bits instead:
//! `ceil(words / 32)` table words of 32 codes each. Every other kind's
//! map is fixed by the kind (`block::fixed_scan_code`): a memo cell's
//! thunk (word 1) and a quote block's holes (words 4..) are counted, and
//! nothing else is. [`cinch_free`] sizes shaped blocks from the table and
//! the drop machinery of [`rc`] scans by it.
//!
//! Static blocks are emitted as `{ header; block }` with the symbol
//! (`@"U.clo"`, `@"U.blk"`) an alias one word in (two for an escaped
//! constructor, whose descriptor word comes first); the string-literal
//! constant carries a header before each of its two blocks. A word with
//! low bit 1 is an immediate, never a block, and no header code
//! dereferences it: a `case` reads its scrutinee's header only when some
//! alternative is a constructor pattern. Pointers from the OS resource
//! handles (buffer, directory, process, scope and task handles) are not
//! blocks and have no header: they are freed only by their own close
//! entry points.
//!
//! # Faults
//!
//! Faults are defined runtime failures. At the extern "C" layer they write
//! a message to stderr (when one exists) and terminate the process:
//!
//! | exit code | fault | entry point |
//! |-----------|-------|-------------|
//! | 70 | `BlackHole` (e.g. demanding a memo cell mid-evaluation) | [`cinch_blackhole`] |
//! | 71 | generic fault: `cinch_panic` and every runtime error except division by zero and the `BlackHole` fault | [`cinch_panic`] |
//! | 72 | division or remainder by zero | [`cinch_fault_div_zero`] |
//! | 73 | heap limit exceeded (`CINCH_HEAP_LIMIT`) | [`heap`] |
//!
//! Exit code 70 is coupled to the code generator: `compiler/src/codegen.rs`
//! emits `__cinch_blackhole` calling `exit(i32 70)` directly. Both call
//! sites are the same defined fault and MUST use the same code; the
//! constant lives in [`abi::EXIT_BLACKHOLE`].
//!
//! Exit code 74 is not a fault: it is the opt-in leak check's
//! (`CINCH_LEAK_CHECK=1`, [`rc::cinch_exit_check`]), a diagnostic of the
//! compiler's reclamation that tests ask for. A program run without the
//! variable never produces it.
//!
//! In the safe Rust API ([`Nat`], [`Int`]) the same conditions surface as
//! [`Error`] values, so tooling and tests observe them without dying. The
//! extern "C" layer maps [`Error::DivByZero`] onto exit 72,
//! [`Error::BlackHole`] onto exit 70, [`Error::HeapLimit`] onto exit 73,
//! and every other error onto exit 71.
//!
//! # Extern symbols
//!
//! Every entry point takes and returns `i64` words or raw pointers. Word
//! arguments must be words produced by this runtime: null or wrong-kind
//! blocks fault as [`Error::CorruptValue`] (exit 71). Any other bogus word
//! used as a pointer is a caller bug and faults like any invalid pointer
//! dereference, not as a typed runtime fault.
//!
//! Faults:
//!
//! * `cinch_blackhole() -> !`: the `BlackHole` fault, exit 70.
//! * `cinch_panic(msg: *const u8, len: usize) -> !`: write the message to
//!   stderr and exit 71. A null pointer or zero length writes nothing.
//! * `cinch_fault_div_zero() -> !`: division fault, exit 72.
//!
//! Allocation (sizes are byte counts):
//!
//! * `cinch_alloc(size: i64, kind: i64) -> i64`: allocate `size`
//!   zero-filled bytes (rounded up to a whole number of 64-bit words,
//!   minimum one word) as a block with header kind `kind` and `rc = 1`,
//!   and return the block as a word. Negative sizes, a kind outside
//!   4..=13 (the shaped kinds go through `cinch_alloc_ctor` and
//!   `cinch_alloc_shaped`), size overflow, and allocation failure fault
//!   through `cinch_panic`.
//! * `cinch_free(word: i64)`: deallocate the block behind `word` with the
//!   size its header records (a shaped block's from the layout table),
//!   after releasing what a runtime box owns outside its block (a
//!   bignum's limbs, a heap buffer's data). It never visits the block's
//!   children. Immediates (low bit 1), 0, and immortal blocks (`rc == 0`)
//!   are ignored. Generated code does not call it yet.
//! * `cinch_alloc_raw(size: i64, kind: i64) -> ptr`: allocate `size`
//!   uninitialised bytes (rounded up to whole words, minimum one) as a
//!   block with header kind `kind` and `rc = 1`, and return the payload
//!   pointer; the allocator generated code calls for every memo, knot,
//!   quote and raw block, passing the block's kind. The same kinds as
//!   `cinch_alloc`.
//! * `cinch_alloc_ctor(size: i64, desc: i64) -> ptr`: allocate a
//!   constructor block of `size` uninitialised payload bytes whose header
//!   carries the constructor descriptor `desc` (above), with `rc = 1`, and
//!   return the payload pointer: every constructor, tuple and boundary
//!   block, with no tag word.
//! * `cinch_alloc_shaped(size: i64, kind: i64, desc: i64) -> ptr`: the
//!   same for a closure (kind 2) or dictionary (kind 3) block of shape
//!   descriptor `desc`; any other kind faults.
//! * `cinch_layouts_register(table: *const u64, len: i64)`: install the
//!   program's layout table of `len` words.
//!
//! Reference counts ([`rc`]; generated code does not call them yet):
//!
//! * `cinch_dup(word: i64)`, `cinch_drop(word: i64)`: add or drop one
//!   reference. Immediates, 0 and immortal blocks are ignored. The drop
//!   of a block's last reference releases it.
//! * `cinch_drop_slow(word: i64)`: release a block whose count is 1, and
//!   every block only it kept alive: iteratively (a pointer-reversal walk
//!   through the dead blocks, no native recursion), children in reverse
//!   slot order and before their parent, by the pointer maps above, with
//!   a box's out-of-block payload (limbs, buffer data) released with it.
//! * `cinch_exit_check(code: i64) -> i64`: called by the entry glue when
//!   `main` returns; `code` unless `CINCH_LEAK_CHECK=1` finds live mortal
//!   blocks, then 74 (below).
//!
//! A corrupt header, a negative count or an invalid map code reached by
//! these entry points is a compiler-bug trap (exit 71 with a message
//! naming the compiler bug), not a program fault: no well-typed program
//! reaches one when the compiler is correct (`design/no-faults.md`).
//!
//! Every heap allocation (the allocators above and every runtime
//! `Box`/`Vec`) is counted by the counting global allocator in [`heap`].
//! Environment: `CINCH_ALLOC_STATS=1` prints
//! `cinch-alloc: blocks=<n> bytes=<n> peak=<n> header-blocks=<n>
//! header-bytes=<n> immortal=<n> freed=<n> live=<n>` to stderr at exit
//! (the last five count header blocks only, see [`heap`]);
//! `CINCH_HEAP_LIMIT=<n>[K|M|G]` caps cumulative allocated bytes (default
//! 8 GiB, `0` disables) and faults with exit 73 and
//! `cinch: heap limit exceeded (<n> bytes)` past it;
//! `CINCH_STACK_SIZE=<n>[K|M|G]` sizes the program thread's reserved
//! stack (default 1 GiB, [`platform::cinch_run_main`]);
//! `CINCH_LEAK_CHECK=1` makes `cinch_exit_check` print `cinch-leak:
//! live=<n> immortal=<n> leaked=<n>` and exit 74 when `leaked` (live
//! header blocks that are not immortal) is not 0, an opt-in diagnostic of
//! the compiler's reclamation for tests; `CINCH_RC_DEBUG=1` poisons and
//! quarantines freed blocks and validates every header the count entry
//! points touch.
//!
//! `Nat` (unbounded natural numbers; negative input or results fault).
//! Every result word is canonical (see "Small `Nat`/`Int` immediates"),
//! and an operation whose operands are both immediates allocates only
//! when its result leaves the immediate range:
//!
//! * `cinch_nat_from_i64(v: i64) -> i64`: a machine integer as a word (an
//!   immediate), faulting on negative `v`.
//! * `cinch_nat_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `cinch_nat_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `cinch_nat_add(a: i64, b: i64) -> i64`, `cinch_nat_mul(a, b) -> i64`:
//!   exact arithmetic.
//! * `cinch_nat_sub(a: i64, b: i64) -> i64`: exact difference, faulting on
//!   negative result (defined fault, not wrapping).
//! * `cinch_nat_div(a: i64, b: i64) -> i64`: the quotient of `a / b`;
//!   the remainder is `cinch_nat_rem(a, b) -> i64`. Both fault on division by
//!   zero.
//! * `cinch_nat_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `cinch_nat_from_decimal(ptr: *const u8, len: usize) -> i64`: parse ASCII
//!   digits (`[0-9]+`, leading zeros allowed) into a word, faulting on
//!   malformed input.
//! * `cinch_nat_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII digits into `buf`, returning the length written, and
//!   fault when `cap` is too small (leaving the buffer untouched). Use
//!   `cinch_nat_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Int` (unbounded integers; canonical results as for `Nat`):
//!
//! * `cinch_int_from_i64(v: i64) -> i64`: a machine integer as a word (a
//!   block only outside the immediate range).
//! * `cinch_int_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `cinch_int_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `cinch_int_add(a: i64, b: i64) -> i64`, `cinch_int_sub(a, b) -> i64`,
//!   `cinch_int_mul(a, b) -> i64`, `cinch_int_neg(a) -> i64`: exact arithmetic.
//! * `cinch_int_div(a: i64, b: i64) -> i64`, `cinch_int_rem(a, b) -> i64`:
//!   division truncated toward zero with the remainder taking the sign of
//!   the dividend (Rust `/` and `%` semantics on the underlying values),
//!   faulting on division by zero.
//! * `cinch_int_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `cinch_int_from_decimal(ptr: *const u8, len: usize) -> i64`: parse an
//!   optionally signed ASCII numeral (`[+-]?[0-9]+`, leading zeros
//!   allowed, `-0` is zero) into a word, faulting on malformed input.
//! * `cinch_int_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII representation (sign included) into `buf`, returning
//!   the length written, and fault when `cap` is too small. Use
//!   `cinch_int_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Bytes` (byte buffers; the foundation for the FFI and string slices):
//!
//! * `cinch_bytes_new(len: i64) -> i64`: a zeroed buffer block, faulting on
//!   negative length.
//! * `cinch_bytes_len(word: i64) -> i64`: the length in bytes.
//! * `cinch_bytes_get(word: i64, index: i64) -> i64`: the byte (0..=255) at
//!   `index`, faulting out of bounds.
//! * `cinch_bytes_set(word: i64, index: i64, value: i64) -> i64`: write
//!   `value` (0..=255) at `index`, returning 0 (the Unit word) and
//!   faulting on an out-of-bounds index or a value outside 0..=255. A
//!   read-only data-section buffer (a string literal's backing) takes
//!   the defined read-only fault instead of a write.
//! * `cinch_bytes_copy(dst, src, di, si, n: i64) -> i64`: copy `n` bytes
//!   of `src` at `si` into `dst` at `di` as one `memmove` (same buffer
//!   and overlap allowed), returning 0; an out-of-range index or count
//!   faults out of bounds and a read-only `dst` faults read-only. Every
//!   bulk copy in the prelude goes through it: a Cinch-level loop boxes
//!   a `Nat` per index step.
//! * `cinch_bytes_compare(a, b: i64) -> i64`: the lexicographic order of
//!   the two contents as -1/0/1, read in place (`string-compare`).
//! * `cinch_bytes_data(word: i64) -> i64`, `cinch_bytes_from_data(ptr: *const
//!   u8, len: i64) -> i64`: the pointer-length crossing for buffer
//!   contents (the UTF-8 and symbol intrinsics and the canonical ABI
//!   glue).
//!
//! Interned symbols (primitive types specification section 9; see
//! [`sym`] for the cell discipline):
//!
//! * `cinch_sym_intern(ptr: *const u8, len: i64) -> i64`: the cell for the
//!   `len` bytes at `ptr`; the table is keyed by name, so an equal name
//!   resolves to the same cell (and to a tick literal's cell).
//! * `cinch_sym_name(word: i64) -> i64`: a fresh `Bytes` word holding the
//!   cell's name.
//! * `cinch_sym_eq(a: i64, b: i64) -> i64`: identity, 0 or 1 (an O(1)
//!   pointer test).
//! * `cinch_sym_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1 of the
//!   lexicographic name order (never the cell addresses).
//!
//! Text conversion and content identity (JSON and canonical
//! serialization; see [`text`]):
//!
//! * `cinch_text_sha256(word: i64) -> i64`: the SHA-256 digest (FIPS
//!   180-4) of the buffer in `word`, as a fresh 32-byte `Bytes` word.
//! * `cinch_text_f64_to_decimal(v: f64) -> i64`: one float's shortest
//!   round-trip decimal text (`{:e}` scientific form) as a fresh
//!   `Bytes` word.
//! * `cinch_text_f64_from_decimal(word: i64) -> f64`: the float one
//!   decimal spelling stands for (correctly rounded), faulting on a
//!   spelling that is not a decimal number.
//!
//! Quotation evaluation (supercompilation and JIT specification
//! section 7; see [`eval`] for the quote block layout):
//!
//! * `cinch_eval(quote: i64) -> i64`: evaluate the reified quotation in the
//!   `Code` block `quote` to its value word; a failing evaluation takes
//!   a defined fault path and produces no value (exit 70 for the quoted
//!   program's own fault or a black-holed memo cell, 71 otherwise).
//!
//! Every bignum and byte-buffer fault exits through `cinch_panic` (71)
//! except division by zero (72); see `crate::error::Error`.
//!
//! Structured concurrency and runtime I/O (concurrency specification
//! sections 2, 4 and 9; one OS thread per task, fork-join happens-before,
//! cancellation as a sequentially consistent atomic):
//!
//! * `cinch_scope_open() -> i64`, `cinch_scope_close(scope: i64) -> i64`: the
//!   structured scope of `withScope`; close cancels and joins unjoined
//!   children (section 2).
//! * `cinch_task_spawn(scope: i64, closure: i64) -> i64`: run `closure` on a
//!   task thread, returning its handle. The body runs through the closure
//!   apply-stub ABI and returns the prelude `Result` word (the task
//!   outcome ABI of `crate::task`).
//! * `cinch_task_join_all_or_cancel(scope, handles: i64, n: i64, out: i64) -> i64`:
//!   `joinAllOrCancel`; 0 and the `Ok` payloads in `out`, or 1 and the
//!   first `Err` payload in `out[0]`.
//! * `cinch_task_poll_cancel() -> i64`: the cancellation observation point
//!   (section 4).
//! * `cinch_io_write(word: i64) -> i64`, `cinch_io_read(len: i64) -> i64`,
//!   `cinch_io_sleep(millis: i64) -> i64`: the `Io r` operations (stdout,
//!   stdin, and a cancellation-aware suspension point).
//! * `cinch_coop_*`: the cooperative single-threaded backend of [`coop`] (the Wasm profile's lowering target) mirrors this surface under these names with the same word conventions.
//!
//! The platform layer (P34; `compiler/runtime/src/platform.rs`), the
//! `cinch_buf_*`/`cinch_clock_*`/`cinch_fs_*`/`cinch_proc_*` externs
//! behind the prelude's `Effect.Clock`/`Effect.Fs`/`Effect.Process`
//! homes: word buffers that carry path and content bytes across the
//! scalar foreign boundary, the runtime-capability clocks, the
//! filesystem operations and the subprocess operations over real
//! pipes. The layer also carries this process's own command line: the
//! `cinch_argv_*` externs (`cinch_argv_init`, called by the generated
//! entry glue before any program code runs, then `cinch_argv_count` and
//! `cinch_argv_get`; word 0 is the program name), which is how a
//! compiled program reads its own arguments; and the program's own
//! thread, `cinch_run_main`, which the same glue calls next to run the
//! program on a 1 GiB reserved stack (`CINCH_STACK_SIZE` overrides it).
//!
//! The networking and piped-child layers (P40's networking family;
//! `compiler/runtime/src/net.rs` and the piped section of
//! `compiler/runtime/src/platform.rs`): the `cinch_net_*` socket and
//! name-resolution externs over live sockets in a handle table, and the
//! `cinch_process_*` live-child externs (separately readable stdout and
//! stderr, writable stdin, a handle table). Both take `Bytes` words
//! (`crate::abi`) for text and payload and answer the negated error code
//! of the platform layer's shared vocabulary; the compiler-side units
//! (`PrimOp::Net`/`PrimOp::Proc`) present those words as the prelude
//! `Result`.
//!
//! The pseudo-terminal layer (`compiler/runtime/src/pty.rs`, the
//! `Open3`/`PTY` shape of the standard library's process family): the
//! `cinch_pty_*` externs over a POSIX pseudo-terminal pair, with the
//! child spawned by `std::process::Command` over the slave end and the
//! master end read, written and controlled through the handle
//! (`PrimOp::Pty` presents the same `Result` convention).

pub mod abi;
pub mod block;
pub mod bytes;
pub mod coop;
pub mod error;
pub mod heap;
pub mod eval;
pub mod evolution;
mod int;
mod limbs;
mod nat;
pub mod net;
pub mod platform;
pub mod pty;
pub mod rc;
pub mod string;
pub mod sym;
pub mod task;
pub mod text;

pub use error::{Error, Result};
pub use int::Int;
pub use nat::Nat;
pub use bytes::Bytes;
