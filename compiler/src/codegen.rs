//! LLVM IR text generation from [`crate::core`], following the closure and
//! dictionary ABI documented there.
//!
//! The backend emits LLVM IR *text* for the opaque-pointer LLVM shipped with
//! nixpkgs (LLVM 18+): `ptr` only, `ptrtoint`/`inttoptr` for word
//! conversions, and quoted identifiers so arbitrary [`GlobalId`] names round
//! trip. The result is compiled and linked by `clang`.
//!
//! # Generated globals
//!
//! Every code generation unit `U` contributes:
//!
//! * `@"U"`: the function itself, `i64` result, one `i64` parameter per
//!   argument,
//! * `@"U.app"`: the apply stub `i64 (ptr %self, i64 %argc, ptr %argv)`,
//!   one entry per function whatever count it is handed. It merges the
//!   closure's saved words with the incoming ones into a fixed
//!   `[arity x i64]` frame, so no stub allocates a dynamic stack region,
//!   then branches once on the merged total: at or above the arity it
//!   calls the function and pushes any excess words at the result, below
//!   it it allocates a closure holding the frame,
//! * `@"U.clo"`: a static zero-filled closure record
//!   `{ i64 ptrtoint(@"U.app"), i64 arity, i64 0 }`,
//! * `@"U.blk"` (zero-field constructors): the static ADT block, so repeated
//!   construction returns one pointer,
//! * `@"U.clo#h"`/`@"U.blk#h"`: the storage of those two static blocks,
//!   an immortal block header word followed by the block. The payload is
//!   a constant offset into that one object ([`static_payload`]), so the
//!   header and the payload cannot be separated by a linker, and a static
//!   block is addressed like a heap block (runtime ABI,
//!   `compiler/runtime/src/block.rs`),
//! * `@"U.dec<N>"` (mathematical `Nat`/`Int` literals beyond the `i64`
//!   fast path): the private decimal digit string a bignum literal parses.
//!
//! Units are emitted for every [`FnDef`], for every instance dictionary
//! constructor (`@"<id>.mk"`), and for every [`PrimOp`] under its intrinsic
//! symbol name (`@"cinch.prim.eqBool"`, `@"cinch.runtime.natAdd"` and
//! friends) so [`Core::Prim`] has a first-class value to reference. The
//! `cinch.runtime.*` units are wrappers around the `cinch_*` externs of the
//! runtime crate.
//!
//! # The process entry
//!
//! A native program's C entry receives the process's own command line and
//! stashes it in the runtime before any program code runs:
//!
//! ```text
//! define i32 @main(i32 %argc, ptr %argv) {
//! entry:
//!   %n = sext i32 %argc to i64
//!   call i64 @cinch_argv_init(i64 %n, ptr %argv)
//!   %r = call i64 @cinch_run_main(ptr @"<main>.exit")
//!   %c = trunc i64 %r to i32
//!   ret i32 %c
//! }
//! ```
//!
//! The language-level contract is unchanged (`main :: Int`, or
//! `main :: Unit ->{e} Int` applied to its `Unit` argument by the
//! synthesized `<main>.exit` adapter); the capture only makes
//! `Effect.Process`'s `process-args` answer. `cinch_run_main` runs the
//! adapter on a thread with a 1 GiB reserved stack and answers its word:
//! locals live in stack slots, so the platform's 8 MiB main-thread stack
//! ends after a few thousand frames of an ordinary non-tail recursion. A
//! Wasm artifact keeps the plain `i32 @main()` shape: its entry is the
//! loader's `_start` and the host hands the command line to it, not to
//! this `main`.
//!
//! # Parameter convention
//!
//! `FnDef::params` carries names for diagnostics only; the body refers to
//! parameter `i` as `LocalId(i)`. Every other `LocalId` is bound by `Let` or
//! a case pattern and is unique within a function body. Locals and
//! temporaries are `alloca` slots in the entry block, so `clang`'s later
//! optimisation passes can promote them.

#[path = "arrays_codegen.rs"]
mod arrays_codegen;
#[path = "slots_codegen.rs"]
mod slots_codegen;

use crate::core::{
    AbiClass, AbiParam, ArithMode, ArithOp, BitOp, BitScanOp, BytesOp, CAlt, CBranch, CPat, Class,
    CmpOp, ConcOp, ConvMode, Core, CtorLayout, FClassOp, FieldAccess, FieldShape, FloatBinOp,
    FloatUnOp, FnDef, ForeignPlan, JoinId, LocalId, NetOp, PlatformOp, PrimAbiTy, PrimOp, PrimPlan, ProcOp,
    Program, PtyOp, ShiftOp, SymOp, TextOp, Utf8Op,
};
use crate::types::Scalar;
use crate::Result;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// The runtime's block header encoding (kinds, rc conventions, the header
/// word layout), shared verbatim with `compiler/runtime/src/block.rs` so
/// the two sides cannot drift.
#[allow(dead_code)]
mod block {
    include!("../runtime/src/block_kinds.rs");
}

/// The target's lowering choices (WebAssembly and WASI specification
/// sections 2 and 7): the `Usize`/`Isize` word width and which task ABI
/// the concurrency units call.
struct LowerCx {
    usize_bits: u32,
    coop_tasks: bool,
    /// The artifact is the build host's own native shape: `asm` bodies
    /// lower for it (primitive operations and inline target IR
    /// specification section 3).
    native: bool,
}

impl LowerCx {
    /// The LLVM type of a `size_t`-width value on this target: the type
    /// a length, an index or an allocation size crosses in when it is
    /// sized by the target rather than by the language.
    fn size_ty(&self) -> &'static str {
        if self.usize_bits == 32 { "i32" } else { "i64" }
    }
}

/// Emit one LLVM module for the program at the build host's target.
/// Emit one whole module for `program`, every unit it declares.
///
/// This is the inspection and test entry point: it emits what the
/// program contains, reachable from an entry or not, so a caller can
/// read any unit's body. A whole-program *build* goes through
/// [`emit_for`], which drops the units no entry point reaches.
pub fn emit(program: &Program) -> Result<String> {
    Ok(emit_module(&crate::core::TargetProfile::native(), program, false, &[], false)?.ir)
}

/// Emit one LLVM module for the program at `profile`'s target: a wasm32
/// profile carries the Wasm triple and data layout (WebAssembly and WASI
/// specification sections 2 and 8), the target's `Usize` width
/// (primitive types specification section 2), and the cooperative task
/// ABI (section 7).
pub fn emit_for(profile: &crate::core::TargetProfile, program: &Program) -> Result<String> {
    Ok(emit_module(profile, program, false, &[], true)?.ir)
}

/// Emit executable IR for only `modules`; every other module is an external
/// declaration resolved by a separately produced object at link time. The
/// layout table still covers the checked program.
pub fn emit_for_modules(
    profile: &crate::core::TargetProfile,
    program: &Program,
    modules: &[String],
) -> Result<String> {
    Ok(emit_module(profile, program, false, modules, false)?.ir)
}

/// A relocatable library module and its actual physical ABI. Its consumer
/// must register a combined layout table containing this exact prefix before
/// calling any unit; this object never registers an independent table.
pub struct EmittedObject {
    pub ir: String,
    pub abi: String,
}

pub fn emit_object_for(
    profile: &crate::core::TargetProfile,
    program: &Program,
) -> Result<EmittedObject> {
    emit_object_for_modules(profile, program, &[])
}

/// Emit an object that defines only the units of `modules`. Every other
/// module is an external dependency: its units are declared, not defined, so
/// separately produced objects can link without duplicating the closure.
pub fn emit_object_for_modules(
    profile: &crate::core::TargetProfile,
    program: &Program,
    modules: &[String],
) -> Result<EmittedObject> {
    if program.main.is_some() {
        return err("object emission requires a library program without a process entry");
    }
    emit_module(profile, program, true, modules, false)
}

/// The `target datalayout` string for a profile's target, in clang's own
/// spelling for that triple: `e` little endian, `m:o` Mach-O, `m:e` ELF,
/// `p` pointer width, and the integer, float, and vector alignments the
/// emitted types are laid out against. `None` for a target whose layout
/// this compiler has no table row for; the module header still names the
/// triple, and the linker's own view of the target then decides layout.
fn target_datalayout(profile: &crate::core::TargetProfile) -> Option<&'static str> {
    Some(match (profile.arch.as_str(), profile.os.as_str()) {
        // arm64-apple-darwin: 64-bit pointers, 128-bit i128 alignment,
        // 128-bit stack alignment, globals in address spaces 0, 4 and 5.
        ("aarch64", "darwin") => {
            "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128"
        }
        // x86_64-apple-darwin: the x87 register width and the 128-bit
        // vector register make `f80` and `v128` part of the layout.
        ("x86_64", "darwin") => concat!(
            "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128",
            "-n8:16:32:64-S128"
        ),
        ("x86_64", "linux") => concat!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128",
            "-n8:16:32:64-S128"
        ),
        // aarch64-unknown-linux-gnu: AArch64's 32-bit-aligned i8 and
        // 16-bit-aligned i16 are explicit where Darwin's are not.
        ("aarch64", "linux") => concat!(
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64",
            "-i128:128-n32:64-S128"
        ),
        // wasm32 (WebAssembly and WASI specification sections 2 and 8):
        // 32-bit pointers, and the funcref and externref address spaces
        // at 8 bytes.
        ("wasm32", _) => {
            "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-i128:128-n32:64-S128-ni:1:10:20"
        }
        _ => return None,
    })
}

fn emitted_symbol(symbol: &str, modules: &[String]) -> bool {
    modules.is_empty()
        || modules
            .iter()
            .any(|module| symbol == module || symbol.starts_with(&format!("{module}.")))
}

fn emit_module(
    profile: &crate::core::TargetProfile,
    program: &Program,
    object: bool,
    emit_modules: &[String],
    prune: bool,
) -> Result<EmittedObject> {
    let cx = LowerCx {
        usize_bits: profile.word_size,
        coop_tasks: profile.artifact != crate::core::ArtifactProfile::Native,
        native: profile.artifact == crate::core::ArtifactProfile::Native,
    };
    let mut reg = Registry::build(program, (profile.word_size / 8) as usize)?;
    reg.rc_debug = profile.rc_debug;
    reg.reclaim = profile.reclaim;
    reg.emit_modules = emit_modules.to_vec();
    // An object, or a module emitted without its neighbours, may not define
    // the constructor unit whose block a use would name: such a module keeps
    // the call. A whole-program module emits every constructor unit, so a use
    // there is the address.
    reg.partial_module = object || !emit_modules.is_empty();
    reg.lent = program.lent.clone();
    let reg = reg;
    // A program with `foreign export` entries may be linked as a library
    // with the C side supplying `main` (foreign function interface
    // specification section 1; decision D65).
    let main_exit = match &program.main {
        Some(m) => Some(m),
        None if object => None,
        None if program.foreigns.values().any(|p| p.export) => None,
        None => {
            return err(format!(
                "this program has no `main`; `emit-ir`, `build`, and `run` need `main :: Int` in the entry module"
            ))
        }
    };
    if let Some(main_exit) = main_exit {
        let main_unit = &main_exit.0;
        let main_arity = reg.arity_of(main_unit)?;
        if main_arity != 0 {
            return err(format!(
                "program.main {main_unit:?} must be a zero-arity thunk, got arity {main_arity}"
            ));
        }
    }

    let mut out = String::new();
    out.push_str("; generated by the Cinch bootstrap compiler\n");
    out.push_str(&format!("target triple = \"{}\"\n", profile.triple));
    // Layout decisions follow the target, never the compile-time host
    // (primitive types specification section 2), so the module states the
    // data layout the emitted types were chosen for. `target datalayout`
    // spells it: e is little endian, m:o is Mach-O and m:e is ELF, p the
    // pointer width, i64/i128 alignment, and n/f/S the x86_64 register
    // sizes. The spellings are the ones clang prints for these triples.
    if let Some(layout) = target_datalayout(profile) {
        out.push_str(&format!("target datalayout = \"{layout}\"\n"));
    }
    // Every zero-field constructor's static block, defined once for the
    // module. A use may come from a unit that is emitted while the
    // constructor's own unit is not (an imported constructor, an object
    // build that selects roots), so the module owns the storage rather than
    // the site that takes the address. `ctor_block`, `ctor_body` and the
    // option helper only mint pointers.
    {
        let mut names: Vec<&String> = reg.ctors.keys().collect();
        names.sort();
        for name in names {
            if reg.ctors.get(name).map(|(_, fields)| *fields) != Some(0) {
                continue;
            }
            if let Some(desc) = reg.ctor_descs.get(name).copied() {
                let symbol = format!("{name}.blk");
                if reg.emitted_blocks.borrow_mut().insert(symbol.clone()) {
                    out.push_str(&static_ctor_block(reg.pointer_bytes, &symbol, desc, &[]));
                }
            }
        }
    }
    out.push_str("declare ptr @cinch_alloc_raw(i64, i64)\n");
    out.push_str("declare ptr @cinch_alloc_ctor(i64, i64)\n");
    out.push_str("declare ptr @cinch_alloc_shaped(i64, i64, i64)\n");
    out.push_str("declare void @cinch_layouts_register(ptr, i64)\n");
    out.push_str("declare i64 @cinch_exit_check(i64)\n");
    // `memcpy`'s length is `size_t`, the target's pointer width, so a
    // 32-bit module declares it `i32` and its calls pass `i32` (a
    // 64-bit declaration makes wasm-ld reject the module: the runtime's
    // own `memcpy` is `(i32, i32, i32)`).
    let size_ty = if cx.size_ty() == "i32" { "i32" } else { "i64" };
    out.push_str(&format!("declare ptr @memcpy(ptr, ptr, {size_ty})\n"));
    // The count entry points (`compiler/runtime/src/rc.rs`, design/reclamation.md
    // sections 4 and 8): the slow paths of an inline dup or drop (the
    // zero crossing, and the atomic counts of blocks shared across tasks),
    // the out-of-line forms a `rc_debug` build calls, the raw release an
    // apply stub and a drop specialisation (`Core::Release`) use to move
    // the fields out of a uniquely held block, and the registration that
    // tells the runtime the program counts references.
    out.push_str("declare void @cinch_dup(i64)\n");
    out.push_str("declare void @cinch_drop(i64)\n");
    out.push_str("declare void @cinch_drop_slow(i64)\n");
    out.push_str("declare void @cinch_dup_mt(i64)\n");
    out.push_str("declare void @cinch_drop_mt(i64)\n");
    out.push_str("declare void @cinch_free(i64)\n");
    out.push_str("declare void @cinch_reclaim_enable()\n");
    // The count helpers are private to this module and a program that
    // counts nothing needs none of them, so they are held back until the
    // units are assembled and then emitted only where a body calls them.
    let frame = reg.reclaim && !reg.rc_debug && reg.any_frame();
    if frame {
        out.push_str("declare void @cinch_frame_release(i64)\n");
    }
    // The runtime ABI's mathematical `Nat`/`Int` and `Bytes` externs
    // (`compiler/runtime/src/lib.rs`); `Nat`/`Int` words are opaque
    // runtime words (immediates or bignum block pointers) passed and
    // returned as `i64`.
    for f in [
        "declare i64 @cinch_nat_from_i64(i64)",
        "declare i64 @cinch_nat_to_i64(i64)",
        "declare i64 @cinch_nat_add(i64, i64)",
        "declare i64 @cinch_nat_sub(i64, i64)",
        "declare i64 @cinch_nat_mul(i64, i64)",
        "declare i64 @cinch_nat_div(i64, i64)",
        "declare i64 @cinch_nat_rem(i64, i64)",
        "declare i64 @cinch_nat_cmp(i64, i64) readnone nounwind willreturn",
        "declare i64 @cinch_nat_from_decimal(ptr, i64)",
        "declare i64 @cinch_int_from_i64(i64)",
        "declare i64 @cinch_int_to_i64(i64)",
        "declare i64 @cinch_int_from_nat(i64)",
        "declare i64 @cinch_int_to_nat(i64)",
        "declare i64 @cinch_int_add(i64, i64)",
        "declare i64 @cinch_int_sub(i64, i64)",
        "declare i64 @cinch_int_mul(i64, i64)",
        "declare i64 @cinch_int_div(i64, i64)",
        "declare i64 @cinch_int_rem(i64, i64)",
        "declare i64 @cinch_int_neg(i64)",
        "declare i64 @cinch_int_cmp(i64, i64)",
        "declare i64 @cinch_int_from_decimal(ptr, i64)",
        "declare i64 @cinch_array_from_list(i64, i64)",
        "declare i64 @cinch_array_replicate(i64, i64, i64)",
        "declare i64 @cinch_array_len(i64)",
        "declare i64 @cinch_array_has(i64, i64)",
        "declare i64 @cinch_array_get(i64, i64)",
        "declare i64 @cinch_array_set(i64, i64, i64)",
        "declare i64 @cinch_array_set_inplace(i64, i64, i64)",
        "declare i64 @cinch_array_slice(i64, i64, i64)",
        "declare i64 @cinch_array_append(i64, i64)",
        "declare i64 @cinch_stm_cell_new(i64, i64)",
        crate::slots::DECLARES[0],
        crate::slots::DECLARES[1],
        crate::slots::DECLARES[2],
        crate::slots::DECLARES[3],
        crate::slots::DECLARES[4],
        crate::slots::DECLARES[5],
        "declare i64 @cinch_tensor_reshape(i64, i64)",
        "declare i64 @cinch_tensor_shape(i64)",
        "declare i64 @cinch_tensor_data(i64)",
        "declare i64 @cinch_tensor_index(i64, i64)",
        "declare i64 @cinch_tensor_get(i64, i64)",
        "declare i64 @cinch_tensor_set(i64, i64, i64)",
        "declare i64 @cinch_columnar_column(i64)",
        "declare i64 @cinch_columnar_zip(i64, i64, i64)",
        "declare i64 @cinch_columnar_rows(i64)",
        "declare i64 @cinch_columnar_columns(i64)",
        "declare i64 @cinch_columnar_get(i64, i64)",
        "declare i64 @cinch_columnar_child(i64, i64)",
        "declare i64 @cinch_columnar_array(i64)",
        "declare i64 @cinch_bytes_new(i64)",
        // A `Nat` as a length or read count: bignums clamp to `i64::MAX`,
        // so a request no allocation can hold is the heap `Exhausted` exit.
        "declare i64 @cinch_nat_to_len(i64)",
        "declare i64 @cinch_bytes_len(i64)",
        "declare i64 @cinch_bytes_get(i64, i64)",
        "declare i64 @cinch_bytes_set(i64, i64, i64)",
        "declare i64 @cinch_bytes_copy(i64, i64, i64, i64, i64)",
        "declare i64 @cinch_bytes_compare(i64, i64)",
        // A fresh `Bytes` from a pointer-length span: the constant
        // `Bytes` emission (the P14 constant-emission path) and the
        // canonical ABI glue both materialise buffers through it.
        "declare i64 @cinch_bytes_from_data(i64, i64)",
        // The data pointer of a `Bytes` word (the pointer-length
        // crossing of `compiler/runtime/src/abi.rs`): the UTF-8 and
        // symbol intrinsics read buffer contents through it.
        "declare i64 @cinch_bytes_data(i64)",
        // The interned symbol table (`compiler/runtime/src/sym.rs`):
        // cells in and out as words, names over the pointer-length
        // crossing above.
        "declare i64 @cinch_sym_intern(i64, i64)",
        "declare i64 @cinch_sym_name(i64)",
        "declare i64 @cinch_sym_eq(i64, i64)",
        "declare i64 @cinch_sym_cmp(i64, i64)",
        // Text conversion and content identity (`compiler/runtime/src/text.rs`):
        // SHA-256 at content-addressing strength and the fixed decimal
        // formatting and parsing of `F64` (JSON and canonical
        // serialization).
        "declare i64 @cinch_text_sha256(i64)",
        "declare i64 @cinch_text_sha512(i64)",
        "declare i64 @cinch_text_sha1(i64)",
        "declare i64 @cinch_text_md5(i64)",
        "declare i64 @cinch_text_hmac_sha256(i64, i64)",
        "declare i64 @cinch_text_f64_to_decimal(double)",
        "declare double @cinch_text_f64_from_decimal(i64)",
        // The platform operations beyond the clock/fs-read/process homes
        // (`compiler/runtime/src/platform.rs`): the mutating filesystem,
        // OS entropy and signal delivery.
        "declare i64 @cinch_random_bytes(i64)",
        "declare i64 @cinch_fs_mkdir(i64, i64)",
        "declare i64 @cinch_fs_remove(i64, i64)",
        "declare i64 @cinch_fs_temp_dir()",
        "declare i64 @cinch_signal_send(i64, i64)",
        // The bump arena (`compiler/runtime/src/arena.rs`): a scope enters
        // one, allocates its constructor cells from it uncounted, and
        // leaves it, which frees the chunks wholesale.
        "declare i64 @cinch_arena_new(i64)",
        "declare i64 @cinch_arena_leave(i64)",
        // The networking family (`compiler/runtime/src/net.rs`): TCP
        // sockets by handle and platform name resolution.
        "declare i64 @cinch_net_tcp_connect(i64, i64)",
        "declare i64 @cinch_net_read(i64, i64)",
        "declare i64 @cinch_net_write(i64, i64)",
        "declare i64 @cinch_net_close(i64)",
        "declare i64 @cinch_net_resolve(i64, i64)",
        // This process's own command line (the argv capture of
        // `compiler/runtime/src/platform.rs`): the generated entry glue
        // stashes `argc`/`argv` through `cinch_argv_init`, and the two
        // readers answer with the capture.
        "declare i64 @cinch_argv_init(i64, ptr)",
        "declare i64 @cinch_argv_count()",
        "declare i64 @cinch_argv_get(i64)",
        // The program's own thread (`cinch_run_main`): the native entry
        // glue runs the program on a reserved stack larger than the
        // platform's main-thread default.
        "declare i64 @cinch_run_main(ptr)",
        // The live piped child (the piped section of
        // `compiler/runtime/src/platform.rs`): separately readable
        // stdout and stderr, writable stdin.
        "declare i64 @cinch_process_spawn(i64, i64, i64, i64)",
        "declare i64 @cinch_process_write_stdin(i64, i64)",
        "declare i64 @cinch_process_close_stdin(i64)",
        "declare i64 @cinch_process_read(i64, i64, i64)",
        "declare i64 @cinch_process_wait(i64)",
        "declare i64 @cinch_process_close(i64)",
        // The pseudo-terminal family (`compiler/runtime/src/pty.rs`): a
        // child on a POSIX pseudo-terminal pair, read and written through the
        // master, with the window size controllable.
        "declare i64 @cinch_pty_open()",
        "declare i64 @cinch_pty_spawn(i64, i64, i64, i64, i64)",
        "declare i64 @cinch_pty_read(i64, i64)",
        "declare i64 @cinch_pty_write(i64, i64)",
        "declare i64 @cinch_pty_wait(i64)",
        "declare i64 @cinch_pty_close(i64)",
        "declare i64 @cinch_pty_resize(i64, i64, i64)",
        // Structured concurrency and runtime I/O (concurrency
        // specification sections 2, 4 and 9; `compiler/runtime/src/task.rs`).
        "declare i64 @cinch_scope_open()",
        "declare i64 @cinch_scope_close(i64)",
        "declare i64 @cinch_task_spawn(i64, i64)",
        "declare i64 @cinch_task_join_all_or_cancel(i64, i64, i64, i64)",
        "declare i64 @cinch_task_poll_cancel()",
        "declare i64 @cinch_io_write(i64)",
        "declare i64 @cinch_io_read(i64)",
        "declare i64 @cinch_io_sleep(i64)",
        // The cooperative single-threaded task backend (WebAssembly and
        // WASI specification section 7): the Wasm profile's task ABI.
        "declare i64 @cinch_coop_scope_open()",
        "declare i64 @cinch_coop_scope_close(i64)",
        "declare i64 @cinch_coop_task_spawn(i64, i64)",
        "declare i64 @cinch_coop_task_join_all_or_cancel(i64, i64, i64, i64)",
        "declare i64 @cinch_coop_task_poll_cancel()",
        "declare i64 @cinch_coop_io_sleep(i64)",
        // Runtime evaluation of a reified quotation (supercompilation
        // and JIT specification section 7).
        "declare i64 @cinch_eval(i64)",
        // The defined fault path used by primitive contract violations.
        // It never returns (`noreturn`): every call site is followed by
        // `unreachable`, so a fault marks its successor dead (the plan
        // note's fault audit).
        "declare void @cinch_panic(ptr, i64) noreturn",
        // The contract restatements of the LLVM contract surface
        // (design/llvm-contract-attributes.md item 4).
        "declare void @llvm.assume(i1)",
        // The sideeffect marker keeping a `prim` `asm` body alive
        // (primitive operations and inline target IR specification
        // section 3; design/llvm-contract-attributes.md).
        "declare void @llvm.sideeffect()",
    ] {
        out.push_str(f);
        out.push('\n');
    }
    // Deterministic pure parallelism (design/parallel-evaluation.md
    // section 3; `compiler/runtime/src/par.rs`): the work-stealing entries,
    // or their sequential readings on the cooperative backend
    // (`cinch_coop_par_*`). A program that uses none of the parallel
    // builtins emits none of these lines, so its IR is unchanged.
    if reg
        .prims
        .iter()
        .any(|op| matches!(op, PrimOp::Par { .. }))
    {
        let prefix = if cx.coop_tasks { "cinch_coop_par" } else { "cinch_par" };
        for entry in ["both", "tabulate", "map_array"] {
            out.push_str(&format!("declare i64 @{prefix}_{entry}(i64, i64, i64)\n"));
        }
        out.push_str(&format!("declare i64 @{prefix}_set_workers(i64)\n"));
    }
    // LLVM intrinsics used by the machine scalar primitives (primitive
    // types specification, sections 4 and 7).
    for w in ["i8", "i16", "i32", "i64"] {
        for sign in ["s", "u"] {
            for o in ["add", "sub", "mul"] {
                out.push_str(&format!(
                    "declare {{ {w}, i1 }} @llvm.{sign}{o}.with.overflow.{w}({w}, {w})\n"
                ));
            }
        }
        for f in ["fshl", "fshr"] {
            out.push_str(&format!("declare {w} @llvm.{f}.{w}({w}, {w}, {w})\n"));
        }
    }
    for w in ["i16", "i32", "i64"] {
        out.push_str(&format!("declare {w} @llvm.bswap.{w}({w})\n"));
    }
    out.push_str("declare float @llvm.fabs.f32(float)\n");
    out.push_str("declare double @llvm.fabs.f64(double)\n");
    out.push_str("declare float @llvm.sqrt.f32(float)\n");
    out.push_str("declare double @llvm.sqrt.f64(double)\n");
    for w in ["i8", "i16", "i32", "i64"] {
        out.push_str(&format!("declare {w} @llvm.ctpop.{w}({w})\n"));
        out.push_str(&format!("declare {w} @llvm.ctlz.{w}({w}, i1)\n"));
        out.push_str(&format!("declare {w} @llvm.cttz.{w}({w}, i1)\n"));
        out.push_str(&format!("declare {w} @llvm.bitreverse.{w}({w})\n"));
    }
    out.push('\n');
    if object {
        out.push_str("declare i64 @__apply(i64, i64, ptr)\n");
    } else {
        out.push_str(APPLY_DEF);
    }
    out.push_str(MEMO_HELPERS);

    // Real foreign bindings (foreign function interface specification
    // section 2): the C symbol declarations (over the named by-value
    // layout types where an aggregate crosses `byval`/`sret`), one
    // marshalling wrapper per import, and one C-callable entry per
    // export.
    if !program.foreigns.is_empty() {
        let size_ty = if cx.size_ty() == "i32" { "i32" } else { "i64" };
        out.push_str(&format!(
            "declare void @llvm.memset.p0.{size_ty}(ptr, i8, {size_ty}, i1)\n"
        ));
        out.push_str(&format!(
            "@__cinch_ffi_nn = internal constant [{} x i8] c\"{}\"\n",
            FFI_NULL_MSG.len(),
            FFI_NULL_MSG
        ));
        for (w, name) in &reg.layout_types {
            out.push_str(&format!("%\"{name}\" = type [{w} x i8]\n"));
        }
        // One declaration per external symbol: two modules may import
        // the same C symbol (the platform homes share the word-buffer
        // bridge), and a repeated `declare` is a redefinition.
        let mut declared: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for plan in program.foreigns.values() {
            if !plan.export && declared.insert(plan.symbol.clone()) {
                out.push_str(&foreign_declare(plan, &reg));
            }
        }
        // The bracket around a `Blocking` import's call (`foreign_call`).
        // A program with no such import emits neither line, so its IR is
        // unchanged.
        if !cx.coop_tasks && program.foreigns.values().any(|plan| plan.blocking) {
            for entry in ["cinch_sched_blocking_enter", "cinch_sched_blocking_exit"] {
                if declared.insert(entry.to_string()) {
                    out.push_str(&format!("declare i64 @{entry}()\n"));
                }
            }
        }
        out.push('\n');
    }

    // `prim` `uses` contracts (primitive operations and inline target IR
    // specification section 4): a fragment calls its external symbols
    // directly, so the module states exactly the machine signature the
    // contract gives, and the fragment's call type-checks at link time.
    let prim_use_declares = prim_use_declares(program, &out);
    if !prim_use_declares.is_empty() {
        for line in prim_use_declares {
            out.push_str(&line);
        }
        out.push('\n');
    }

    if !emit_modules.is_empty() {
        let mut declared: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        for f in &program.fns {
            if emitted_symbol(&f.id.0, emit_modules) {
                continue;
            }
            if !declared.insert(f.id.0.clone()) {
                continue;
            }
            // A unit another module defines is declared with the
            // boundary its definition has, narrowed or not.
            let sig = reg.narrow_sigs.get(&f.id.0);
            let params = (0..f.arity())
                .map(|i| sig.map_or(Cross::WORD, |s| s.param(i)).ty())
                .collect::<Vec<_>>()
                .join(", ");
            let ret = sig.map_or(Cross::WORD, |s| s.ret).ty();
            out.push_str(&format!("declare {ret} @{}({params})\n", quote(&f.id.0)));
            out.push_str(&format!(
                "declare i64 @{}()\n",
                quote(&format!("{}.object-closure", f.id.0))
            ));
        }
        for inst in &program.instances {
            let name = format!("{}.mk", inst.id.0);
            if emitted_symbol(&name, emit_modules) || !declared.insert(name.clone()) {
                continue;
            }
            let params = (0..inst.subdict_count)
                .map(|_| "i64")
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("declare i64 @{}({params})\n", quote(&name)));
        }
        for (name, plan) in &reg.foreigns {
            if plan.export || emitted_symbol(name, emit_modules) || !declared.insert(name.clone()) {
                continue;
            }
            let params = (0..plan.arity())
                .map(|_| "i64")
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("declare i64 @{}({params})\n", quote(name)));
        }
        out.push('\n');
    }

    // The units are built and scanned first, attributed under the
    // call-graph classification second, and assembled last: the
    // effect/usage-derived attributes are transitive through calls
    // (design/llvm-contract-attributes.md).
    let mut units: Vec<EmittedUnit> = Vec::new();
    for op in &reg.prims {
        units.push(build_unit(
            &cx,
            &reg,
            &op.symbol(),
            op.arity(),
            UnitBody::Prim(*op),
        )?);
    }
    for (name, plan) in &reg.foreigns {
        if !plan.export {
            units.push(build_unit(
                &cx,
                &reg,
                name,
                plan.arity(),
                UnitBody::Foreign(plan),
            )?);
        }
    }
    for f in &program.fns {
        match (f.prim, f.ctor) {
            (Some(_), Some(_)) => {
                return err(format!(
                    "unit {} is both a primitive and a constructor",
                    f.id
                ))
            }
            (Some(op), None) => {
                if f.arity() != op.arity() {
                    return err(format!(
                        "primitive unit {} takes {} parameters, expected {}",
                        f.id,
                        f.arity(),
                        op.arity()
                    ));
                }
                units.push(build_unit(
                    &cx,
                    &reg,
                    &f.id.0,
                    f.arity(),
                    UnitBody::Prim(op),
                )?);
            }
            (None, Some((_, fields))) => {
                if f.arity() != fields {
                    return err(format!(
                        "constructor unit {} takes {} parameters, expected {fields} fields",
                        f.id,
                        f.arity()
                    ));
                }
                units.push(build_unit(
                    &cx,
                    &reg,
                    &f.id.0,
                    f.arity(),
                    UnitBody::Ctor(fields),
                )?);
            }
            (None, None) => {
                if f.meta.prim_body {
                    // A `prim` declaration's synthesized unit: its
                    // content is the inline target fragment (primitive
                    // operations and inline target IR specification
                    // section 2).
                    prim_decl_units(&mut units, &cx, &reg, program, f)?;
                } else {
                    // A member of a recursive group whose tail calls cross
                    // arities runs its body in a worker of the group's
                    // width (`crate::core::tail_widths`); its own symbol
                    // stays the entry closures and exports call.
                    if let Some(&width) = reg.tail_width.get(&f.id.0) {
                        units.push(build_unit(
                            &cx,
                            &reg,
                            &tail_worker(&f.id.0),
                            width,
                            UnitBody::TailWorker(f),
                        )?);
                    }
                    units.push(build_unit(&cx, &reg, &f.id.0, f.arity(), UnitBody::Fn(f))?);
                }
            }
        }
    }
    for inst in &program.instances {
        let info = reg
            .insts
            .get(&inst.id.0)
            .expect("instance registered in Registry::build");
        let name = format!("{}.mk", inst.id.0);
        units.push(build_unit(
            &cx,
            &reg,
            &name,
            inst.subdict_count,
            UnitBody::Mk(info),
        )?);
    }
    let classes = classify_units(&units);
    // A whole program is emitted closed: every unit is reachable from an
    // entry point, and a unit no entry point reaches is dead. Emitting the
    // unreachable ones costs the assembler a whole prelude per program and
    // leaves thousands of external symbols nothing can delete, so the
    // reachable set is computed and the rest is skipped. A module subset or
    // an object keeps everything: a consumer links against it.
    let reachable = if prune && !object && emit_modules.is_empty() {
        Some(reachable_units(&units, &reg, program, main_exit.map(|e| e.0.clone())))
    } else {
        None
    };
    // A unit needs its apply stub and static closure only when something
    // takes it as a value, which shows up as a reference to `<unit>.clo`
    // or `<unit>.app`. `refs` already records those, so the set is one
    // pass over the units. A program that only ever calls its functions
    // directly emits no stubs at all, and a stub is the largest single
    // piece of a small unit's output.
    let mut closure_taken: HashSet<String> = HashSet::new();
    for u in &units {
        closure_taken.extend(u.scan.closures_taken.iter().cloned());
    }
    for u in &units {
        let primitive = reg.prims.iter().any(|op| op.symbol() == u.name);
        if !emitted_symbol(&u.name, emit_modules) && !primitive {
            continue;
        }
        if let Some(reachable) = &reachable {
            if !reachable.contains(&u.name) {
                continue;
            }
        }
        let group = reg.attr_group(
            u.hints,
            &u.lang_facts,
            &classes[&u.name],
            cx.native,
            profile.arch == "wasm32",
        );
        assemble_unit(
            &mut out,
            u,
            group,
            (object && primitive).then_some("linkonce_odr"),
            // An object is a link-time artifact: any consumer may take
            // any of its exported functions as a value, and the module
            // cannot see those uses. Its `object-closure` accessor is
            // emitted for every unit for the same reason, so the closure
            // and its apply stub are kept whether or not this module
            // takes them. A whole program is closed, so there a unit
            // whose closure nothing names needs neither, and `prune`
            // false (`emit`, the inspection entry point) keeps every unit
            // whole.
            object || !prune || closure_taken.contains(&u.name),
        );
        if object && reg.arity.contains_key(&u.name) {
            // Static closure blocks are module-local. Export an accessor
            // instead of relying on a platform linker to expose storage
            // that belongs to this module.
            out.push_str(&format!(
                "define {}i64 @{}() {{\nentry:\n  ret i64 ptrtoint (ptr {} to i64)\n}}\n",
                if primitive { "linkonce_odr " } else { "" },
                quote(&format!("{}.object-closure", u.name)),
                static_payload(&format!("{}.clo", u.name), reg.closure_desc(&u.name))
            ));
        }
    }
    // The count helpers a body actually calls. Emitting all four into
    // every module put roughly 120 lines of unreachable IR into a program
    // that counts nothing (`main = 0` was 696 lines, 121 of them these).
    // They are `internal`, so the assembler deleted them anyway: this is
    // IR size and the assembler's parse, not code size.
    if reg.reclaim && !reg.rc_debug {
        out.push_str(&rc_helpers(frame, &out));
    }
    // The C-callable entries (foreign function interface specification
    // section 2).
    for plan in reg.foreigns.values() {
        if plan.export {
            export_entry(&mut out, plan, &reg, &cx)?;
        }
    }

    if let Some(main_exit) = main_exit {
        let main_unit = &main_exit.0;
        if cx.native {
            // The native process entry keeps the C signature and hands
            // the process's own command line to the runtime BEFORE the
            // program's code runs (`cinch_argv_init`), so a program can
            // read its own arguments through the prelude's `process-args`
            // and the language-level contract (`main :: Int`, applied to
            // `Unit` when it is effectful) is unchanged. The program then
            // runs on the runtime's reserved-stack thread
            // (`cinch_run_main`). The result word passes the runtime's
            // exit check (`CINCH_LEAK_CHECK`, a no-op otherwise) and
            // truncates to the C `exit` status exactly as before.
            out.push_str(&format!(
                "define i32 @main(i32 %argc, ptr %argv) {{\nentry:\n  %n = sext i32 %argc to i64\n  call i64 @cinch_argv_init(i64 %n, ptr %argv)\n  %r = call i64 @cinch_run_main(ptr @{})\n  %x = call i64 @cinch_exit_check(i64 %r)\n  %c = trunc i64 %x to i32\n  ret i32 %c\n}}\n",
                quote(main_unit)
            ));
        } else {
            // A Wasm artifact has no `argv` to capture (its entry is the
            // loader's `_start`, not this `main`), so the entry stays as
            // it was: the command line reaches it through the platform
            // layer a Wasm target actually provides.
            //
            // The layout table is registered here rather than left to
            // `llvm.global_ctors`: a freestanding module has no `_start`
            // and no libc, so nothing runs the constructors and the
            // table stayed empty. Every shaped block then answered
            // `shaped block with no registered layout`, which surfaced as
            // a trap inside `release` on the first `map` over a closure
            // with saved words, nowhere near the cause. A host that does
            // call `__wasm_call_ctors` runs the same registration twice;
            // `cinch_layouts_register` is idempotent, so that is free.
            out.push_str(&format!(
                "define i32 @main() {{\nentry:\n  call void @__cinch_layouts_init()\n  %r = call i64 @{}()\n  %x = call i64 @cinch_exit_check(i64 %r)\n  %c = trunc i64 %x to i32\n  ret i32 %c\n}}\n",
                quote(main_unit)
            ));
        }
    }
    // The constructor layout table, complete now that every unit has
    // asked for its descriptors.
    if !object {
        out.push_str(&reg.layouts.borrow().finish(reg.reclaim)?);
    }
    // Function attribute groups and metadata nodes (the branch weights
    // of attribute annotations specification section 3.1).
    out.push_str(&reg.finish_meta());
    let abi = if object {
        // The recorded identity is the identity of what is written, so the
        // dedupe happens before it: a hash over the pre-dedupe text would
        // disagree with the file on disk.
        reg.object_abi(program, profile, &dedupe_static_blocks(&out), emit_modules)?
    } else {
        String::new()
    };
    Ok(EmittedObject { ir: dedupe_static_blocks(&out), abi })
}

fn err<T>(msg: impl Into<String>) -> Result<T> {
    Err(crate::Error::plain(msg))
}

/// Quote an identifier for LLVM textual IR.
fn quote(name: &str) -> String {
    let mut s = String::with_capacity(name.len() + 2);
    s.push('"');
    for byte in name.bytes() {
        match byte {
            b'"' | b'\\' | 0x20..=0x7E => {
                if byte == b'"' || byte == b'\\' {
                    s.push_str(&format!("\\{byte:02X}"));
                } else {
                    s.push(byte as char);
                }
            }
            _ => s.push_str(&format!("\\{byte:02X}")),
        }
    }
    s.push('"');
    s
}

/// The LLVM `c"..."` spelling of constant bytes (the constant `Bytes`
/// emission): printable ASCII except `"` and `\` stays literal, every
/// other byte is a hex escape.
fn c_string_bytes(buf: &[u8]) -> String {
    let mut s = String::with_capacity(buf.len());
    for &b in buf {
        match b {
            0x20..=0x7E if b != b'"' && b != b'\\' => s.push(b as char),
            _ => s.push_str(&format!("\\{b:02X}")),
        }
    }
    s
}

/// Instance metadata with method arities resolved up front.
#[derive(Clone, Debug)]
struct InstInfo {
    subdict_count: usize,
    method_count: usize,
    /// Method unit name with the arity of its `FnDef` (subdicts included).
    methods: Vec<(String, usize)>,
    /// Actual stored word classes, including eager value-method results.
    fields: Vec<Class>,
    /// Actual initialized value slots captured ahead of the factory context.
    values: Vec<Vec<usize>>,
}

/// Unit name to arity map plus resolved instances.
#[derive(Default)]
struct Registry {
    arity: HashMap<String, usize>,
    insts: HashMap<String, InstInfo>,
    /// Primitive operations the program mentions through `Core::Prim`;
    /// each gets one synthesized unit under its intrinsic symbol.
    prims: BTreeSet<PrimOp>,
    /// Foreign binding contracts by unit name (foreign function
    /// interface specification section 2); each import gets one
    /// synthesized wrapper unit, each export one C-callable entry.
    foreigns: BTreeMap<String, crate::core::ForeignPlan>,
    /// Boundary layout plans by constructor unit name (attribute
    /// annotations specification section 4.3).
    plans: BTreeMap<String, CtorLayout>,
    /// Module-level metadata nodes (branch weights) in emission order;
    /// metadata ids are module-scoped and the registry is one per
    /// module (attribute annotations specification section 3.1).
    meta_defs: RefCell<Vec<String>>,
    meta_next: Cell<usize>,
    /// Function attribute groups by content (attribute annotations
    /// specification section 5).
    attr_groups: RefCell<Vec<(String, usize)>>,
    /// Result value ranges by unit name (`!range`/`range` facts; the
    /// plan note's item 3), from the primitive signatures and the
    /// checked definitions' result types.
    ranges: HashMap<String, (i64, i64)>,
    /// The width each unit's parameters and result cross at, when
    /// narrowing is proved for that position and resolved against the
    /// target (design/narrow-value-representation.md section 4). A unit
    /// absent here crosses entirely in words, which is every unit a proof
    /// did not reach and every unit whose proved widths are all the slot.
    narrow_sigs: HashMap<String, NarrowSig>,
    /// Constructor units' `(tag, fields)` by unit name (the block words
    /// `ctor_body` writes; a string constant emits the same shape).
    ctors: HashMap<String, (usize, usize)>,
    /// Constructor units' descriptors (`block_kinds.rs`) by unit name.
    ctor_descs: HashMap<String, u64>,
    /// Static constructor blocks this module defines, by symbol. A caller may
    /// reference a zero-field constructor's block before the constructor's
    /// own unit is emitted, so the definition is emitted on first use and
    /// exactly once.
    emitted_blocks: RefCell<std::collections::HashSet<String>>,
    /// Whether this module defines only some units (an object build, or a
    /// module emitted without its neighbours), where a constructor's block
    /// may not be defined at all.
    partial_module: bool,
    /// The program's constructor layout ids and layout table.
    layouts: RefCell<Layouts>,
    /// The named by-value layout types of the aggregate boundary
    /// (`%__sl_layout_N = type [W x i8]`), keyed by width: the `byval`
    /// and `sret` spellings need a pointee type.
    layout_types: BTreeMap<u64, String>,
    /// The symbols the callee classifier tests for: every unit this module
    /// emits plus the tail workers derived from it. One set per module,
    /// built on first use because the registry is complete by the time
    /// emission asks.
    ///
    /// It is a field rather than a local because rebuilding it per emitted
    /// unit re-collected and re-sorted every unit name once per unit, which
    /// was a quarter of the emit pass on the compiler bundle.
    symbols: std::sync::OnceLock<HashSet<String>>,
    /// The widened tail signatures (`crate::core::tail_widths`): a unit
    /// named here runs its body in the worker `tail_worker(name)`, which
    /// takes the mapped number of words.
    tail_width: HashMap<String, usize>,
    /// Each unit's parameter classes (concrete after the collector): the
    /// classes of the words a closure over the unit saves.
    unit_params: HashMap<String, Vec<Class>>,
    /// The target's pointer width in bytes: a static initializer can
    /// hold a pointer only at this width, because a linker relocates the
    /// address, not a widened copy of it.
    pointer_bytes: usize,
    /// Count operations call the out-of-line runtime entries
    /// ([`crate::core::TargetProfile::rc_debug`]).
    rc_debug: bool,
    /// The program counts references
    /// ([`crate::core::TargetProfile::reclaim`]); otherwise no unit emits
    /// an ownership operation and every block leaks.
    reclaim: bool,
    /// The modules this module defines (empty: every module), as
    /// [`emitted_symbol`] reads them.
    emit_modules: Vec<String>,
    /// The primitive units' borrowed parameters ([`Program::lent`]): their
    /// wrappers leave them to the caller.
    lent: std::collections::BTreeMap<String, Vec<bool>>,
    /// Frame allocation decisions by unit name ([`Program::frame`],
    /// `crate::stack`): the allocations a unit builds in its own frame.
    frame: std::collections::BTreeMap<String, crate::stack::Plan>,
    /// Whether the program hands a frame block to a callee's owned
    /// parameter ([`Program::stack`], `crate::stack::Report`): the callees
    /// then need the count paths' frame arms even though they build no
    /// frame block of their own.
    frame_received: bool,
}

impl Registry {
    /// Whether this module can hold or receive a frame block: the inline
    /// count helpers then carry the frame arms. A module of a program the
    /// pass lowered builds none and receives none only when every one of
    /// its units is its own caller, which is what keeps a module that
    /// `--no-stack-alloc` left alone byte-identical to the pre-pass IR.
    fn any_frame(&self) -> bool {
        !self.frame.is_empty() || self.frame_received
    }
}

impl Registry {
    /// The unit symbols the callee classifier tests for: the registry's
    /// units and the tail workers derived from them. Built once, then read
    /// by every unit's body scan.
    fn unit_symbols(&self) -> &HashSet<String> {
        self.symbols.get_or_init(|| {
            let mut symbols: HashSet<String> = self.arity.keys().cloned().collect();
            symbols.extend(self.tail_width.keys().map(|unit| tail_worker(unit)));
            symbols
        })
    }

    /// Whether instance `id`'s dictionary is one immortal static block
    /// `@"<id>.dict"`: it has no subdictionaries and no eagerly computed
    /// value slots, so every method word is the method's own static
    /// closure and every `.mk` call would build the same constant block.
    fn static_dict(&self, id: &str) -> bool {
        self.insts.get(id).is_some_and(|info| {
            info.subdict_count == 0
                && info.values.iter().all(Vec::is_empty)
                && info.methods.iter().all(|(_, arity)| *arity > 0)
        })
    }

    /// Physical facts about the units just emitted. This is deliberately
    /// separate from source exports: a private layout does not make its
    /// constructor available to an importing source module.
    fn object_abi(
        &self,
        p: &Program,
        profile: &crate::core::TargetProfile,
        ir: &str,
        emit_modules: &[String],
    ) -> Result<String> {
        use crate::interface::JsonValue as J;
        fn class(c: Class) -> Result<J> {
            Ok(J::from(match c {
                Class::Raw => "R",
                Class::Obj => "O",
                Class::TObj => "T",
                Class::Param(_) => {
                    return err("object ABI contains an unresolved representation parameter")
                }
            }))
        }
        fn classes(cs: &[Class]) -> Result<J> {
            Ok(J::Arr(
                cs.iter().copied().map(class).collect::<Result<Vec<_>>>()?,
            ))
        }
        // The legacy emitter accepts hand-written Core without annotations,
        // defaulting missing classes to Raw. Such a module cannot supply
        // checked ownership authority to a separately compiled consumer.
        for f in &p.fns {
            let params = if f.ctor.is_some() {
                &f.repr.fields
            } else {
                &f.repr.params
            };
            // A parameter-independent constructor (for example List.Nil)
            // legitimately shares one unit across all type arguments.
            if (f.repr.tparams != 0 && f.ctor.is_none())
                || params.len() != f.arity()
                || params.iter().any(|c| c.is_param())
            {
                return err(format!(
                    "object ABI needs concrete checked parameter classes for {}",
                    f.id
                ));
            }
        }
        let mut names: Vec<_> = self
            .arity
            .keys()
            .filter(|name| {
                emitted_symbol(name, emit_modules)
                    || self.prims.iter().any(|op| op.symbol() == **name)
            })
            .collect();
        names.sort();
        let globals: HashSet<String> = names
            .iter()
            .flat_map(|name| {
                ["", ".app", ".clo", ".blk", ".clo#h", ".blk#h", "#tail"]
                    .map(|suffix| format!("{name}{suffix}"))
            })
            .collect();
        for name in &names {
            let accessor = format!("{name}.object-closure");
            if globals.contains(&accessor) {
                return err(format!(
                    "object closure accessor collides with a unit global: {accessor}"
                ));
            }
        }
        let mut units = Vec::new();
        for name in names {
            let params = self.unit_params.get(name).ok_or_else(|| {
                crate::Error::plain(format!("missing object parameter classes for {name}"))
            })?;
            if params.len() != self.arity[name] {
                return err(format!("object ABI parameter arity mismatch for {name}"));
            }
            let result = if self.insts.keys().any(|i| format!("{i}.mk") == *name)
                || self.ctors.contains_key(name)
            {
                Some(Class::Obj)
            } else {
                p.fns
                    .iter()
                    .find(|f| f.id.0 == *name)
                    .and_then(|f| f.repr.result)
                    .or_else(|| {
                        self.prims
                            .iter()
                            .find(|op| op.symbol() == *name)
                            .and_then(|op| prim_sig(*op).map(|(_, ty)| sig_class(&ty)))
                    })
            };
            units.push(J::obj([
                ("symbol", name.as_str().into()),
                ("closure_entry", format!("{name}.object-closure").into()),
                ("arity", (self.arity[name] as u64).into()),
                ("params", classes(params)?),
                ("result", result.map(class).transpose()?.unwrap_or(J::Null)),
            ]));
        }
        let layouts = self.layouts.borrow();
        let mut types = BTreeMap::new();
        for (name, ids) in &layouts.types {
            types.insert(
                name.clone(),
                match ids {
                    TypeIds::Group(group) => J::obj([("group", (*group).into())]),
                    TypeIds::Escaped { base, shared } => {
                        J::obj([("base", (*base).into()), ("shared", (*shared).into())])
                    }
                },
            );
        }
        let mut ctors = BTreeMap::new();
        for (name, (tag, fields)) in &self.ctors {
            if !emitted_symbol(name, emit_modules) {
                continue;
            }
            ctors.insert(
                name.clone(),
                J::obj([
                    ("tag", (*tag as u64).into()),
                    ("arity", (*fields as u64).into()),
                    ("descriptor", self.ctor_descs[name].into()),
                ]),
            );
        }
        let mut dictionaries = BTreeMap::new();
        for inst in &p.instances {
            if !emitted_symbol(&inst.id.0, emit_modules) {
                continue;
            }
            let info = &self.insts[&inst.id.0];
            dictionaries.insert(
                inst.id.0.clone(),
                J::obj([
                    ("class", inst.class.as_str().into()),
                    ("context_arity", (info.subdict_count as u64).into()),
                    (
                        "methods",
                        J::Arr(
                            info.methods
                                .iter()
                                .map(|(name, _)| J::from(name.as_str()))
                                .collect(),
                        ),
                    ),
                    ("fields", classes(&info.fields)?),
                ]),
            );
        }
        Ok(J::obj([
            ("schema", 1u64.into()),
            ("runtime_abi", crate::interface::RUNTIME_ABI_VERSION.into()),
            ("target", profile.triple.as_str().into()),
            ("word_size", profile.word_size.into()),
            ("endian", profile.endian.as_str().into()),
            ("reclaim", profile.reclaim.into()),
            // The interface content hash is a deterministic cache identity,
            // not a cryptographic artifact digest. Artifact admission must
            // additionally hash the object and matching interface closure.
            ("ir_content_hash", crate::interface::content_hash(ir).into()),
            (
                "modules",
                J::Arr(emit_modules.iter().map(|m| J::from(m.as_str())).collect()),
            ),
            ("units", J::Arr(units)),
            ("types", J::Obj(types)),
            ("constructors", J::Obj(ctors)),
            ("dictionaries", J::Obj(dictionaries)),
            ("escaped_descriptors", layouts.escaped.into()),
            ("layout_entries", (layouts.table.len() as u64).into()),
            (
                "layout_words",
                J::Arr(layouts.words()?.into_iter().map(J::from).collect()),
            ),
        ])
        .to_canonical_string())
    }

    /// Register one module-level metadata node; returns its `!N`
    /// reference.
    fn meta_node(&self, def: String) -> String {
        let id = self.meta_next.get();
        self.meta_next.set(id + 1);
        self.meta_defs.borrow_mut().push(format!("!{id} = {def}\n"));
        format!("!{id}")
    }

    /// A `!prof` branch weights node for one terminator's successors
    /// (attribute annotations specification section 3.1).
    fn branch_weights(&self, ws: &[u32]) -> String {
        let list: Vec<String> = ws.iter().map(|w| format!("i32 {w}")).collect();
        self.meta_node(format!("!{{!\"branch_weights\", {}}}", list.join(", ")))
    }

    /// The `#N` function attribute group for one unit: its hint set and
    /// its LLVM contract attributes, registered once per distinct
    /// group. Every contract attribute is claimed only where the
    /// classification proved it (design/llvm-contract-attributes.md).
    ///
    /// A native unit also carries `"probe-stack"="inline-asm"`: a frame
    /// larger than a page then touches each page in order as it grows,
    /// so it reaches the runtime's stack guard (`compiler/runtime/src/stack.rs`)
    /// instead of stepping over it, and overflowing the stack is always
    /// the `Exhausted` exit. Units are the only generated functions whose
    /// frame grows with the program (every local has its own slot); the
    /// stubs and helpers keep small fixed frames. Wasm has no native
    /// stack to probe.
    fn attr_group(
        &self,
        hints: crate::core::FnHints,
        lang: &UnitLang,
        class: &UnitClass,
        native: bool,
        wasm: bool,
    ) -> Option<String> {
        let mut attrs: Vec<String> = Vec::new();
        if native {
            attrs.push("\"probe-stack\"=\"inline-asm\"".to_string());
            // A frame pointer on every native unit, so a walk of the
            // frame chain is possible without unwinding at all: the
            // failure path (`cinch_panic`, `compiler/runtime/src/abi.rs`)
            // and any foreign unwinder that would rather read the chain
            // than parse CFI both need it, and a backtrace that depends
            // on `.eh_frame` alone cannot name a frame whose CFI the
            // linker dropped. `all` rather than `non-leaf` because a leaf
            // that faults (a `cinch_panic` call is not a leaf, but a
            // `llvm.trap` path is) still has to be walkable.
            attrs.push("\"frame-pointer\"=\"all\"".to_string());
        }
        // WebAssembly's tail calls are outside the MVP: a `musttail` call
        // (which a `D104` group produces when the callee's signature
        // matches the caller's) does not assemble without the feature.
        // It is a per-function attribute, not a module directive, and
        // every function carries it because a callee without it is not a
        // legal `musttail` target.
        if wasm {
            attrs.push("\"target-features\"=\"+tail-call\"".to_string());
        }
        // Cinch has no unwinding: a definition cannot resume past a
        // callee that does not return. The one hole is a foreign call,
        // whose unwinding behaviour the compiler does not know, so a unit
        // that reaches an extern the scan cannot see through does not
        // claim it.
        if !class.externs {
            attrs.push("nounwind".to_string());
        }
        // `norecurse` is the call graph's answer, computed once for the
        // module: a unit outside every cycle cannot call itself back.
        if !class.recursive {
            attrs.push("norecurse".to_string());
        }
        match hints.inline {
            Some(crate::core::InlineHint::Inline) => attrs.push("inlinehint".to_string()),
            Some(crate::core::InlineHint::Always) => attrs.push("alwaysinline".to_string()),
            Some(crate::core::InlineHint::Never) => attrs.push("noinline".to_string()),
            None => {}
        }
        if hints.cold {
            attrs.push("cold".to_string());
        }
        match class.mem {
            MemClass::None => {
                attrs.push("memory(none)".to_string());
                // `speculatable` needs the effect-derived purity too:
                // no effects besides the result and no undefined
                // behavior, on every input.
                if lang.pure && class.speculatable {
                    attrs.push("speculatable".to_string());
                }
            }
            MemClass::Read => attrs.push("memory(read)".to_string()),
            MemClass::ReadWrite => attrs.push("memory(readwrite)".to_string()),
        }
        // `willreturn` and `nofree` restate what the scan proved; a
        // checked definition restates them only under its P20 and
        // totality rows (the plan note: `willreturn` is never guessed).
        if class.willreturn && (!lang.def || lang.returns) {
            attrs.push("willreturn".to_string());
        }
        if class.nofree && (!lang.def || lang.no_alloc) {
            attrs.push("nofree".to_string());
        }
        // `noreturn` restates the lowered control flow: no reachable block
        // returns. A returning unit cannot claim it, and `willreturn` and
        // `noreturn` are mutually exclusive by construction (one says
        // every path returns, the other says none does), so the two
        // cannot both be emitted for one unit.
        debug_assert!(
            !(class.noreturn && class.willreturn),
            "a unit cannot both return and never return"
        );
        if class.noreturn {
            attrs.push("noreturn".to_string());
        }
        if attrs.is_empty() {
            return None;
        }
        attrs.sort_unstable();
        let content = attrs.join(" ");
        let mut groups = self.attr_groups.borrow_mut();
        if let Some((_, n)) = groups.iter().find(|(c, _)| *c == content) {
            return Some(format!("#{n}"));
        }
        let n = groups.len();
        groups.push((content, n));
        Some(format!("#{n}"))
    }

    /// The module's function attribute and metadata definitions.
    fn finish_meta(&self) -> String {
        let mut out = String::new();
        for (content, n) in self.attr_groups.borrow().iter() {
            out.push_str(&format!("attributes #{n} = {{ {content} }}\n"));
        }
        for d in self.meta_defs.borrow().iter() {
            out.push_str(d);
        }
        out
    }
}

impl Registry {
    fn build(p: &Program, pointer_bytes: usize) -> Result<Registry> {
        let mut reg = Registry::default();
        // Set first: the narrowed signatures below resolve `csize`
        // crossings against it.
        reg.pointer_bytes = pointer_bytes;
        reg.frame = p.frame.clone();
        reg.frame_received = !p.stack.receives_frame.is_empty();
        let mut fn_arities: HashMap<String, usize> = HashMap::new();
        for f in &p.fns {
            if fn_arities.insert(f.id.0.clone(), f.arity()).is_some() {
                return err(format!("duplicate function unit {}", f.id));
            }
            reg.add_unit(&f.id.0, f.arity())?;
        }
        for inst in &p.instances {
            if reg.insts.contains_key(&inst.id.0) {
                return err(format!("duplicate instance {}", inst.id));
            }
            let mut methods = Vec::with_capacity(inst.methods.len());
            for m in &inst.methods {
                let arity = fn_arities.get(&m.0).copied().ok_or_else(|| {
                    crate::Error::plain(format!(
                        "instance {} method {} is not a defined function",
                        inst.id, m
                    ))
                })?;
                if arity < inst.subdict_count {
                    return err(format!(
                        "instance {} method {} has arity {arity}, below its subdict count {}",
                        inst.id, m, inst.subdict_count
                    ));
                }
                methods.push((m.0.clone(), arity));
            }
            if methods.len() != inst.method_count {
                return err(format!(
                    "instance {} declares {} methods but lists {}",
                    inst.id,
                    inst.method_count,
                    methods.len()
                ));
            }
            let name = format!("{}.mk", inst.id.0);
            reg.add_unit(&name, inst.subdict_count)?;
            reg.insts.insert(
                inst.id.0.clone(),
                InstInfo {
                    subdict_count: inst.subdict_count,
                    method_count: inst.method_count,
                    fields: methods
                        .iter()
                        .map(|(name, arity)| {
                            let function = p.fns.iter().find(|f| f.id.0 == *name);
                            let hidden = function.map_or(0, |f| f.repr.dictionary_values.len());
                            if *arity == inst.subdict_count + hidden {
                                function.and_then(|f| f.repr.result).unwrap_or(Class::Raw)
                            } else {
                                Class::Obj
                            }
                        })
                        .collect(),
                    values: methods
                        .iter()
                        .map(|(name, _)| {
                            p.fns
                                .iter()
                                .find(|f| f.id.0 == *name)
                                .map(|f| f.repr.dictionary_values.clone())
                                .unwrap_or_default()
                        })
                        .collect(),
                    methods,
                },
            );
        }
        // Only the primitives the program mentions get synthesized units.
        for f in &p.fns {
            collect_prims(&f.body, &mut reg.prims);
            if let Some(spec) = f.ctor {
                reg.ctors.insert(f.id.0.clone(), spec);
            }
        }
        let used: Vec<(String, usize)> = reg
            .prims
            .iter()
            .map(|op| (op.symbol(), op.arity()))
            .collect();
        for (sym, arity) in used {
            reg.add_unit(&sym, arity)?;
        }
        // Boundary layout plans for constructor units, and one wrapper
        // unit per real foreign import (foreign function interface
        // specification section 2).
        reg.plans
            .extend(p.plans.iter().map(|(k, v)| (k.clone(), v.clone())));
        reg.lay_out_constructors(p)?;
        reg.foreigns
            .extend(p.foreigns.iter().map(|(k, v)| (k.clone(), v.clone())));
        for (name, plan) in &p.foreigns {
            if !plan.export {
                reg.add_unit(name, plan.arity())?;
            }
        }
        // The `!range`/`range` facts: primitive units by their result
        // type, primitive aliases by their intrinsic's, and checked
        // definitions by their result range (the plan note's item 3).
        for op in &reg.prims {
            if let Some(r) = prim_sig(*op).and_then(|(_, t)| t.value_range()) {
                reg.ranges.insert(op.symbol(), r);
            }
        }
        for f in &p.fns {
            let r = match f.prim {
                Some(op) => prim_sig(op).and_then(|(_, t)| t.value_range()),
                None => f.meta.contract.result_range,
            };
            if let Some(r) = r {
                reg.ranges.insert(f.id.0.clone(), r);
            }
        }
        // The narrowed boundary signatures, from the same contract the
        // header is built from, so a call site and its callee cannot
        // disagree about a position's width. Only an ordinary function
        // body narrows: a primitive's signature is its declared ABI, a
        // constructor or `prim` declaration unit is word-typed, and a
        // widened unit's symbol wraps a word-typed tail worker. The
        // program's `main` is called by the runtime through a fixed
        // `i64 ()` pointer (`cinch_run_main`), so it keeps the word too.
        reg.tail_width = crate::core::tail_widths(p);
        for f in &p.fns {
            if f.prim.is_some()
                || f.ctor.is_some()
                || f.meta.prim_body
                || reg.tail_width.contains_key(&f.id.0)
                || p.main.as_ref() == Some(&f.id)
            {
                continue;
            }
            let sig = NarrowSig::of(&f.meta.contract, reg.pointer_bytes);
            if !sig.is_word() {
                reg.narrow_sigs.insert(f.id.0.clone(), sig);
            }
        }
        // A recursive group's tail calls are `musttail` (decision D104),
        // which needs the caller's and the callee's prototypes to be
        // identical. A group whose members would narrow to different
        // prototypes keeps the word for all of them: constant stack is a
        // language guarantee, and a narrower boundary is not worth
        // breaking it. On a 64-bit target only a machine scalar narrows,
        // so this rarely fires; on a 32-bit one it is what keeps a group
        // mixing, say, an `I64` parameter with a pointer one in constant
        // stack.
        if !reg.narrow_sigs.is_empty() {
            for group in crate::core::tail_report(p) {
                let proto = |name: &String| reg.narrow_sigs.get(name).map(NarrowSig::proto);
                let first = group.members.first().map(&proto);
                if group.members.iter().any(|m| Some(proto(m)) != first) {
                    for m in &group.members {
                        reg.narrow_sigs.remove(m);
                    }
                }
            }
        }
        // The named by-value layout types (`byval`/`sret` pointees).
        let mut widths: BTreeSet<u64> = BTreeSet::new();
        for plan in p.foreigns.values() {
            for a in plan.params.iter().chain(std::iter::once(&plan.result)) {
                if matches!(a.class, AbiClass::Aggregate) {
                    widths.insert(a.width);
                }
            }
        }
        for (i, w) in widths.into_iter().enumerate() {
            reg.layout_types.insert(w, format!("__sl_layout_{i}"));
        }
        reg.unit_params = unit_param_classes(p, &reg);

        // Every unit emits four global names; all of them must be distinct
        // and must not collide with the runtime helpers.
        let mut globals: HashSet<String> = HashSet::new();
        for unit in reg.arity.keys() {
            for suffix in ["", ".app", ".clo", ".blk", ".clo#h", ".blk#h", "#tail"] {
                let full = format!("{unit}{suffix}");
                for reserved in ["main", "__apply", "malloc", "memcpy"] {
                    if full == reserved {
                        return err(format!(
                            "unit {unit:?} collides with runtime global {reserved:?}"
                        ));
                    }
                }
                if !globals.insert(full.clone()) {
                    return err(format!(
                        "emitted global name {full:?} is used more than once"
                    ));
                }
            }
        }
        Ok(reg)
    }

    fn add_unit(&mut self, name: &str, arity: usize) -> Result<()> {
        if self.arity.insert(name.to_string(), arity).is_some() {
            return err(format!("duplicate code generation unit name {name:?}"));
        }
        Ok(())
    }

    fn arity_of(&self, name: &str) -> Result<usize> {
        self.arity
            .get(name)
            .copied()
            .ok_or_else(|| crate::Error::plain(format!("unknown code generation unit {name:?}")))
    }

    /// The symbol and parameter count a direct call of `unit` (arity
    /// `arity`) targets: its widened worker where it has one, else the
    /// unit itself. Two units whose tail signatures have one count have
    /// identical prototypes, which is what `musttail` needs.
    fn tail_sig(&self, unit: &str, arity: usize) -> (String, usize) {
        match self.tail_width.get(unit) {
            Some(&width) => (tail_worker(unit), width),
            None => (unit.to_string(), arity),
        }
    }
}

impl Registry {
    /// Give every data type of the program its layout ids (`Layouts`):
    /// the constructor units grouped by their data type and class vector,
    /// in a fixed order, each constructor's map its field classes or its
    /// boundary plan's words.
    fn lay_out_constructors(&mut self, p: &Program) -> Result<()> {
        let mut types: BTreeMap<String, BTreeMap<usize, (String, Vec<Class>)>> = BTreeMap::new();
        // The built prelude types' payload words per tag, across copies.
        let mut built: BTreeMap<&str, BTreeMap<usize, u64>> = BTreeMap::new();
        for f in &p.fns {
            let Some((tag, fields)) = f.ctor else {
                continue;
            };
            // A constructor copied for a class vector (`Cons@T`) lays out
            // with its type at that vector.
            let base = crate::repr::base_unit(&f.id.0);
            let suffix = &f.id.0[base.len()..];
            let key = match &f.repr.data {
                Some(d) => format!("{d}{suffix}"),
                None => f.id.0.clone(),
            };
            let map = match self.plan_of(&f.id.0) {
                Some(plan) => boundary_map(plan, &f.repr.fields),
                None if f.repr.fields.len() == fields => f.repr.fields.clone(),
                // A constructor lowering gave no classes (the builtin
                // index family's): never counted.
                None => vec![Class::Raw; fields],
            };
            if let Some(d) = f.repr.data.as_deref() {
                if let Some((name, _)) = BUILT_TYPES.iter().find(|(k, _)| *k == d) {
                    built.entry(name).or_default().insert(tag, map.len() as u64);
                }
            }
            if types
                .entry(key.clone())
                .or_default()
                .insert(tag, (f.id.0.clone(), map))
                .is_some()
            {
                return err(format!(
                    "data type {key} has two constructor units of tag {tag}"
                ));
            }
        }
        for (key, words) in BUILT_TYPES {
            if let Some(got) = built.get(key) {
                let got: Vec<u64> = (0..got.keys().next_back().map_or(0, |t| t + 1))
                    .map(|t| got.get(&t).copied().unwrap_or(0))
                    .collect();
                if got != words {
                    return err(format!(
                        "the prelude type {key} has payload words {got:?} per constructor; code generation builds it as {words:?}"
                    ));
                }
            }
        }
        let mut layouts = Layouts::default();
        // A program that crosses the canonical ABI can lift an enum whose
        // tag arrives at run time; its block may be escaped, so every tag
        // read must allow for one.
        if p.foreigns.values().any(|plan| plan.canon.is_some()) {
            layouts.escaped = true;
        }
        for (key, ctors) in &types {
            let ncons = ctors.keys().next_back().map_or(0, |t| t + 1);
            let mut maps = vec![Vec::new(); ncons];
            for (tag, (_, map)) in ctors {
                maps[*tag] = map.clone();
            }
            layouts.add_type(key, &maps)?;
            for (tag, (unit, _)) in ctors {
                let desc = layouts.desc(key, *tag as u64).expect("laid out just now");
                self.ctor_descs.insert(unit.clone(), desc);
            }
        }
        self.layouts = RefCell::new(layouts);
        Ok(())
    }

    /// The boundary layout plan of constructor unit `unit`, or of the
    /// generic constructor a class-vector copy was made from.
    fn plan_of(&self, unit: &str) -> Option<&CtorLayout> {
        self.plans
            .get(unit)
            .or_else(|| self.plans.get(crate::repr::base_unit(unit).as_ref()))
    }

    /// The descriptor of constructor unit `unit`.
    fn ctor_desc(&self, unit: &str) -> Result<u64> {
        self.ctor_descs
            .get(unit)
            .copied()
            .ok_or_else(|| crate::Error::plain(format!("unknown constructor unit {unit:?}")))
    }

    /// The descriptor of a block of tag `tag` whose payload words have the
    /// classes `map` and that no constructor unit names
    /// (`Layouts::shape_desc`).
    fn shape_desc(&self, map: &[Class], tag: u64) -> u64 {
        self.layouts.borrow_mut().shape_desc(map, tag)
    }

    /// [`Registry::shape_desc`] of `words` uncounted words: a block whose
    /// words' classes are not known here (a boxed-out boundary aggregate,
    /// a task outcome), so it can only leak, never free a live block.
    fn raw_shape_desc(&self, words: u64, tag: u64) -> u64 {
        self.shape_desc(&vec![Class::Raw; words as usize], tag)
    }

    /// The shape descriptor of a closure over unit `unit`: its saved words
    /// are the unit's leading parameters, so its map is the fixed closure
    /// words and then the unit's parameter classes.
    fn closure_desc(&self, unit: &str) -> u64 {
        let mut map = vec![Class::Raw; block::CLOSURE_PREFIX_WORDS as usize];
        match self.unit_params.get(unit) {
            Some(params) => map.extend(params.iter().copied()),
            None => map.extend(std::iter::repeat_n(
                Class::Raw,
                self.arity.get(unit).copied().unwrap_or(0),
            )),
        }
        self.layouts.borrow_mut().shape(map)
    }

    /// Dictionary slots contain either closures or eagerly computed values;
    /// the latter retain the result class recorded by the checker and collector.
    fn dict_desc(&self, info: &InstInfo) -> u64 {
        let mut map = vec![Class::Raw; block::DICT_PREFIX_WORDS as usize];
        map.extend(info.fields.iter().copied());
        self.layouts.borrow_mut().shape(map)
    }

    /// Whether a tag read must allow for an escaped descriptor.
    fn escapes(&self) -> bool {
        self.layouts.borrow().escaped
    }
}

/// The pointer map of a boundary-layout (`repr(C)`) payload: a word field
/// at an aligned offset has its constructor field's class (`fields`), and
/// every other payload word is uncounted. A scalar is not a pointer; a
/// word field at an unaligned offset (`repr(packed)`) and the word fields
/// inside an inline nested aggregate have no word-aligned slot or no
/// recorded class, so they can only leak.
fn boundary_map(plan: &CtorLayout, fields: &[Class]) -> Vec<Class> {
    let mut map = vec![Class::Raw; plan.payload.div_ceil(8) as usize];
    for (i, fa) in plan.fields.iter().enumerate() {
        if matches!(fa.shape, FieldShape::Word { .. }) && fa.offset % 8 == 0 && fa.size == 8 {
            if let (Some(slot), Some(c)) = (map.get_mut((fa.offset / 8) as usize), fields.get(i)) {
                *slot = concrete(*c);
            }
        }
    }
    map
}

/// The checker's view of named types as far as code generation needs it:
/// the types a primitive's signature names are data types, all `Obj`.
struct SigTypes;

impl crate::repr::TypeReprs for SigTypes {
    fn adt(&self, _name: &str, _args: &[crate::types::Ty]) -> crate::repr::AdtRepr {
        crate::repr::AdtRepr::Obj
    }
}

/// The class of a primitive signature's type.
fn sig_class(t: &crate::types::Ty) -> Class {
    crate::repr::repr_of(t, &[], &SigTypes, &mut |_| {})
}

/// A primitive's parameter classes from its signature. The deferral,
/// evaluation and concurrency builtins carry no boundary signature;
/// their classes are fixed here: a thunk, memo cell, quotation or task
/// body is a block, a capability, scope or task handle is raw, and a
/// byte count or duration is a `Nat`.
fn prim_param_classes(op: PrimOp) -> Vec<Class> {
    use ConcOp::*;
    match op {
        PrimOp::Array {
            op,
            classes,
            packed,
        } => {
            let params = op.params(packed);
            op.sig(packed)
                .0
                .iter()
                .map(|t| crate::repr::repr_of(t, &params, &SigTypes, &mut |_| {}).subst(&classes))
                .collect()
        }
        PrimOp::Slots { op, classes } => {
            let params = op.params();
            op.sig()
                .0
                .iter()
                .map(|t| crate::repr::repr_of(t, &params, &SigTypes, &mut |_| {}).subst(&classes))
                .collect()
        }
        PrimOp::Stm { op, classes } => {
            let params = op.params();
            op.sig()
                .0
                .iter()
                .map(|t| crate::repr::repr_of(t, &params, &SigTypes, &mut |_| {}).subst(&classes))
                .collect()
        }
        PrimOp::Delay | PrimOp::Force | PrimOp::ApplyErased | PrimOp::Eval => vec![Class::Obj],
        // The parallel units' parameters: two closures, or the callback and
        // the count / input array. The element classes are immediates in
        // the body, not parameters.
        PrimOp::Par { op, .. } => match op {
            crate::core::ParOp::Both | crate::core::ParOp::MapArray => {
                vec![Class::Obj, Class::Obj]
            }
            crate::core::ParOp::Tabulate => vec![Class::TObj, Class::Obj],
        },
        PrimOp::Conc(WithScope | Spawn | IoWrite) => vec![Class::Raw, Class::Obj],
        PrimOp::Conc(IoRead | IoSleep) => vec![Class::Raw, Class::TObj],
        PrimOp::Conc(PollCancel | JoinN(_)) => vec![Class::Raw; op.arity()],
        _ => match prim_sig(op) {
            Some((params, _)) => params.iter().map(sig_class).collect(),
            None => vec![Class::Raw; op.arity()],
        },
    }
}

/// The primitive ownership table (design/reclamation.md section 4). A
/// primitive unit owns its parameters like every unit. The runtime
/// externs it wraps only borrow theirs, so the unit drops each counted
/// parameter after the extern call returns, on every return path, unless
/// the primitive passes the word on: `delay` returns its thunk, `force`
/// and `withScope` hand their closure to `__apply` (which consumes it),
/// and `spawn` moves the task body to its thread (the runtime marks what
/// it reaches shared, and the thread's apply stub consumes it). Results
/// are fresh, owned words; a `Some`/`Ok` payload is a fresh word the
/// block takes over.
pub(crate) fn prim_consumes(op: PrimOp, i: usize) -> bool {
    match op {
        PrimOp::Delay | PrimOp::Force | PrimOp::ApplyErased => i == 0,
        PrimOp::Stm { op, .. } => op.consumes(i),
        PrimOp::Slots { op, .. } => op.consumes(i),
        PrimOp::Conc(ConcOp::WithScope | ConcOp::Spawn) => i == 1,
        _ => false,
    }
}

/// The payload classes of the prelude block a primitive returns: `[t]`
/// for `Option t`, `[e, t]` for `Result e t` (the `Err` then the `Ok`
/// payload), empty otherwise. Runtime input (`IoRead`) is `Option Bytes`.
fn prim_payload_classes(op: PrimOp) -> Vec<Class> {
    if let PrimOp::Array { op, classes, .. } = op {
        return if op.returns_option() {
            vec![op.result_class(classes)]
        } else {
            vec![]
        };
    }
    if matches!(op, PrimOp::Stm { .. } | PrimOp::Slots { .. }) {
        return vec![];
    }
    if op == PrimOp::Conc(ConcOp::IoRead) {
        return vec![Class::Obj];
    }
    match prim_sig(op).map(|(_, t)| t) {
        Some(crate::types::Ty::Adt(name, args)) if name == crate::known::OPTION || name == crate::known::RESULT => {
            args.iter().map(sig_class).collect()
        }
        _ => Vec::new(),
    }
}

/// The classes of a foreign wrapper unit's parameters: a canonical
/// binding's safe words (a byte payload, an enum, a flags value or a
/// flattened aggregate is a block; a scalar, `Bool`, `Char` or handle is
/// not), or a C binding's (an aggregate parameter is the `repr(C)` block
/// it copies from; scalars and pointer roles are raw).
fn foreign_param_classes(plan: &ForeignPlan) -> Vec<Class> {
    use crate::core::CanonStep;
    match &plan.canon {
        Some(call) => call
            .params
            .iter()
            .map(|steps| match steps.as_slice() {
                [] | [CanonStep::Word { .. }] | [CanonStep::Handle] => Class::Raw,
                _ => Class::Obj,
            })
            .collect(),
        None => plan
            .params
            .iter()
            .map(|a| {
                if matches!(a.class, AbiClass::Aggregate) {
                    Class::Obj
                } else {
                    Class::Raw
                }
            })
            .collect(),
    }
}

/// Every unit's parameter classes, fitted to its arity (a missing or open
/// class is `Raw`). A constructor's parameters are its fields; a
/// dictionary construction unit's are its subdictionaries.
fn unit_param_classes(p: &Program, reg: &Registry) -> HashMap<String, Vec<Class>> {
    let fit = |cs: &[Class], n: usize| -> Vec<Class> {
        (0..n)
            .map(|i| cs.get(i).copied().map_or(Class::Raw, concrete))
            .collect()
    };
    let mut out: HashMap<String, Vec<Class>> = HashMap::new();
    for op in &reg.prims {
        out.insert(op.symbol(), prim_param_classes(*op));
    }
    for f in &p.fns {
        let arity = f.arity();
        let cs = if f.ctor.is_some() {
            &f.repr.fields
        } else {
            &f.repr.params
        };
        let classes = match f.prim {
            Some(op) if cs.len() != arity => prim_param_classes(op),
            _ => fit(cs, arity),
        };
        out.insert(f.id.0.clone(), classes);
    }
    for (name, plan) in &p.foreigns {
        if !plan.export {
            out.insert(
                name.clone(),
                fit(&foreign_param_classes(plan), plan.arity()),
            );
        }
    }
    for inst in &p.instances {
        out.insert(
            format!("{}.mk", inst.id.0),
            vec![Class::Obj; inst.subdict_count],
        );
    }
    out
}

/// The prelude types the code generator builds without a constructor unit
/// in hand: the `Option` results of primitives, the `Result` outcomes of
/// the platform and task units, and the `Ordering` of comparisons. Each
/// row is the data type and its constructors' payload words in tag order,
/// exactly the prelude's declarations (`prelude/Data/Core.cinch`,
/// `prelude/Data/Ordering.cinch`). Those blocks take anonymous shapes
/// (`Layouts::shape_desc`) with the same tags, so a program that declares
/// the types otherwise is refused rather than miscompiled.
const BUILT_TYPES: [(&str, &[u64]); 3] = [
    (crate::known::OPTION, &[0, 1]),
    (crate::known::RESULT, &[1, 1]),
    (crate::known::ORDERING, &[0, 0, 0]),
];

/// The prelude tags of the built types.
const TAG_NONE: u64 = 0;
const TAG_SOME: u64 = 1;
const TAG_ERR: u64 = 0;
const TAG_OK: u64 = 1;

/// A class with every parameter resolved to `Raw`: a word whose class
/// the collector left open is never counted, which can only leak.
fn concrete(c: Class) -> Class {
    if c.is_param() {
        Class::Raw
    } else {
        c
    }
}

/// The pointer-map code (`block_kinds.rs`) of a word's class.
fn scan_code(c: Class) -> u64 {
    match c {
        Class::Obj => block::SCAN_OBJ,
        Class::TObj => block::SCAN_TOBJ,
        Class::Raw | Class::Param(_) => block::SCAN_RAW,
    }
}

/// Layout ids and shapes (design/reclamation.md section 3, slices S1b and
/// S5; the encoding is `block_kinds.rs`'s). Every table index records a
/// payload size and pointer map.
///
/// Constructor blocks: each data type at each class vector its
/// constructor units were copied for (`crate::specialize`) gets one group
/// of `2^CTOR_TAG_BITS` consecutive ids, `base = group << CTOR_TAG_BITS`,
/// so a block's tag is `id - base`, which every reader computes as `id &
/// CTOR_TAG_MASK` without knowing the scrutinee's type (`Core::Case`
/// carries none). The same type at another vector is another group: its
/// constructors' maps differ, its tags do not. A type with more
/// constructors than a group holds, or one past the last group, is
/// escaped: one table index per constructor, its tag in the descriptor
/// word before the header. Blocks no constructor unit names take an
/// anonymous group per pointer map.
///
/// Closures and dictionaries: one table index per distinct map (a shape),
/// placed first in the indices of the type groups that no constructor
/// uses, so shapes cost no group of their own until those run out.
#[derive(Default)]
struct Layouts {
    /// Each laid-out type's ids, by data type (or anonymous shape) key.
    types: HashMap<String, TypeIds>,
    /// The pointer map per table index (its length is the payload
    /// words); `None` for an index no block uses.
    table: Vec<Option<Vec<Class>>>,
    /// The first table index past every assigned one.
    next: u64,
    /// Whether some block may carry an escaped descriptor, so every tag
    /// read must check for one. Fixed before any unit is emitted.
    escaped: bool,
    /// An anonymous shape escaped after the units were laid out without
    /// escapes: the emitted tag reads would be wrong, so the module is
    /// refused (`finish`).
    late_escape: bool,
    /// Closure and dictionary shapes by map.
    shapes: HashMap<Vec<Class>, u64>,
    /// Where the search for an unused index below `next` resumes.
    hole: u64,
}

#[derive(Clone, Copy)]
enum TypeIds {
    /// Regular ids `ctor_id(group, tag)`.
    Group(u64),
    /// Escaped: tag `t` has table index `base + t`, or `base` for every
    /// tag when the constructors share one map (`shared`, an anonymous
    /// shape).
    Escaped { base: u64, shared: bool },
}

impl Layouts {
    fn set_map(&mut self, index: u64, map: &[Class]) {
        let i = index as usize;
        if self.table.len() <= i {
            self.table.resize(i + 1, None);
        }
        self.table[i] = Some(map.iter().map(|c| concrete(*c)).collect());
    }

    /// Reserve `n` table indices for one type: a whole regular group when
    /// `n` fits one and a group is left, else `n` escaped indices.
    fn place(&mut self, n: u64) -> Result<TypeIds> {
        let group = self.next.div_ceil(block::CTOR_TAG_MASK + 1);
        if n <= block::CTOR_TAG_MASK + 1 && group < block::CTOR_GROUPS {
            self.next = block::ctor_id(group + 1, 0);
            return Ok(TypeIds::Group(group));
        }
        let base = self.next;
        self.next += n.max(1);
        if self.next >> (64 - block::CTOR_ESC_INDEX_SHIFT) != 0 {
            return err(
                "the program has more constructor layouts than an escaped descriptor can index",
            );
        }
        Ok(TypeIds::Escaped {
            base,
            shared: false,
        })
    }

    /// Lay out a data type (at one class vector) whose tag `t` has the
    /// pointer map `maps[t]`, one class per payload word.
    fn add_type(&mut self, key: &str, maps: &[Vec<Class>]) -> Result<()> {
        if (maps.len() as u64) >> block::CTOR_ESC_TAG_BITS != 0 {
            return err(format!(
                "data type {key} has more constructors than a descriptor can tag"
            ));
        }
        let ids = self.place(maps.len() as u64)?;
        if matches!(ids, TypeIds::Escaped { .. }) {
            self.escaped = true;
        }
        self.types.insert(key.to_string(), ids);
        for (tag, map) in maps.iter().enumerate() {
            let index = match ids {
                TypeIds::Group(g) => block::ctor_id(g, tag as u64),
                TypeIds::Escaped { base, .. } => base + tag as u64,
            };
            self.set_map(index, map);
        }
        Ok(())
    }

    /// The descriptor of `tag` of the laid-out type `key`.
    fn desc(&self, key: &str, tag: u64) -> Option<u64> {
        Some(match *self.types.get(key)? {
            TypeIds::Group(g) => block::ctor_id(g, tag),
            TypeIds::Escaped { base, shared } => {
                block::ctor_escaped(if shared { base } else { base + tag }, tag)
            }
        })
    }

    /// The descriptor of a block that no constructor unit names (a
    /// canonical ABI lift, a boxed-out nested aggregate, a built prelude
    /// value): tag `tag` of the anonymous shape whose payload words have
    /// the classes `map`. Any id with the right tag reads correctly, so
    /// the shape only has to record the map: one regular group per map,
    /// and one shared escaped index per map for tags past a group.
    fn shape_desc(&mut self, map: &[Class], tag: u64) -> u64 {
        let map: Vec<Class> = map.iter().map(|c| concrete(*c)).collect();
        let regular = tag <= block::CTOR_TAG_MASK;
        let letters = crate::repr::letters(&map);
        let key = if regular {
            format!("<shape {letters}>")
        } else {
            format!("<shape {letters} escaped>")
        };
        if !self.types.contains_key(&key) {
            let group = self.next.div_ceil(block::CTOR_TAG_MASK + 1);
            let ids = if regular && group < block::CTOR_GROUPS {
                self.next = block::ctor_id(group + 1, 0);
                TypeIds::Group(group)
            } else {
                let base = self.next;
                self.next += 1;
                TypeIds::Escaped { base, shared: true }
            };
            match ids {
                TypeIds::Group(g) => {
                    for t in 0..=block::CTOR_TAG_MASK {
                        self.set_map(block::ctor_id(g, t), &map);
                    }
                }
                TypeIds::Escaped { base, .. } => {
                    self.set_map(base, &map);
                    self.late_escape |= !self.escaped;
                }
            }
            self.types.insert(key.clone(), ids);
        }
        self.desc(&key, tag).expect("laid out just now")
    }

    /// The shape descriptor (`block_kinds.rs`'s `shape_desc`) of a
    /// closure or dictionary whose payload words have the classes `map`.
    fn shape(&mut self, map: Vec<Class>) -> u64 {
        let map: Vec<Class> = map.into_iter().map(concrete).collect();
        if let Some(index) = self.shapes.get(&map) {
            return block::shape_desc(*index);
        }
        while self.hole < self.next && matches!(self.table.get(self.hole as usize), Some(Some(_))) {
            self.hole += 1;
        }
        let index = if self.hole < self.next {
            self.hole
        } else {
            let group = self.next.div_ceil(block::CTOR_TAG_MASK + 1);
            let base = if group < block::CTOR_GROUPS {
                block::ctor_id(group, 0)
            } else {
                self.next
            };
            self.next = if group < block::CTOR_GROUPS {
                block::ctor_id(group + 1, 0)
            } else {
                base + 1
            };
            base
        };
        self.hole = index + 1;
        self.set_map(index, &map);
        self.shapes.insert(map, index);
        block::shape_desc(index)
    }

    /// The layout table and the global constructor that registers it
    /// with the runtime before any generated code runs
    /// (`cinch_layouts_register`): one entry per index, then the map
    /// extensions of the entries wider than an entry's own map. A program
    /// that counts references (`reclaim`) also says so there
    /// (`cinch_reclaim_enable`), so the runtime marks task bodies and
    /// drops the task results nobody receives.
    fn words(&self) -> Result<Vec<u64>> {
        if self.late_escape {
            return err("internal error: a constructor shape escaped after the tag reads were emitted without escapes");
        }
        let mut entries: Vec<u64> = Vec::with_capacity(self.table.len());
        let mut ext: Vec<u64> = Vec::new();
        for map in &self.table {
            let map = map.as_deref().unwrap_or(&[]);
            let words = map.len() as u64;
            let n = block::scan_ext_len(words);
            if n == 0 {
                let bits = map
                    .iter()
                    .enumerate()
                    .fold(0, |m, (i, c)| m | block::scan_slot(scan_code(*c), i as u64));
                entries.push(block::layout_entry(words, bits));
            } else {
                let at = (self.table.len() + ext.len()) as u64;
                if at >> (64 - block::LAYOUT_ENTRY_WORDS_BITS) != 0 {
                    return err("the layout table is too large for its map extension indices");
                }
                entries.push(block::layout_entry(words, at));
                let start = ext.len();
                ext.resize(start + n as usize, 0);
                for (i, c) in map.iter().enumerate() {
                    let i = i as u64;
                    ext[start + (i / block::SCAN_EXT_WORDS) as usize] |=
                        block::scan_slot(scan_code(*c), i % block::SCAN_EXT_WORDS);
                }
            }
        }
        entries.extend(ext);
        Ok(entries)
    }

    fn finish(&self, reclaim: bool) -> Result<String> {
        let entries = self.words()?;
        let n = entries.len();
        let words: Vec<String> = entries
            .iter()
            .map(|e| format!("i64 {}", *e as i64))
            .collect();
        let enable = if reclaim {
            "  call void @cinch_reclaim_enable()\n"
        } else {
            ""
        };
        Ok(format!(
            "@__cinch_layouts = internal constant [{n} x i64] [{}]\n\
             define internal void @__cinch_layouts_init() {{\n\
             entry:\n{enable}  call void @cinch_layouts_register(ptr @__cinch_layouts, i64 {n})\n  ret void\n}}\n\
             @llvm.global_ctors = appending global [1 x {{ i32, ptr, ptr }}] [{{ i32, ptr, ptr }} {{ i32 65535, ptr @__cinch_layouts_init, ptr null }}]\n",
            words.join(", ")
        ))
    }
}

/// The `cinch_alloc_ctor` call that allocates a constructor block of
/// `bytes` payload bytes with descriptor `desc` into `dst`.
fn ctor_alloc_call(dst: &str, bytes: impl std::fmt::Display, desc: u64) -> String {
    format!(
        "{dst} = call ptr @cinch_alloc_ctor(i64 {bytes}, i64 {})",
        desc as i64
    )
}

/// Allocate a constructor block with descriptor `desc` holding the words
/// `fields` in order, and return the block word.
fn ctor_block_word(b: &mut Builder, desc: u64, fields: &[String]) -> String {
    let p = b.reg();
    b.line(&ctor_alloc_call(&p, 8 * fields.len(), desc));
    for (i, f) in fields.iter().enumerate() {
        b.store_at(&p, 8 * i, f);
    }
    let w = b.reg();
    b.line(&format!("{w} = ptrtoint ptr {p} to i64"));
    w
}

/// The prelude `String` block (`StrBuf`, one field) over the `Bytes`
/// word `bytes`: the canonical ABI's wrapped byte payloads.
fn string_block_word(b: &mut Builder, reg: &Registry, bytes: &str) -> String {
    let desc = match reg.ctor_descs.get(crate::known::STRING_BUFFER) {
        Some(d) => *d,
        None => reg.shape_desc(&[Class::Obj], 0),
    };
    ctor_block_word(b, desc, &[bytes.to_string()])
}

/// The descriptor, computed at run time, of an anonymous block whose
/// payload words have the classes `map` and whose tag is the word `tag`
/// (an enum lifted from canonical memory): the shape's regular id for a
/// tag that fits a group, its escaped descriptor otherwise.
fn dynamic_shape_desc(b: &mut Builder, reg: &Registry, map: &[Class], tag: &str) -> String {
    let group_base = reg.shape_desc(map, 0);
    let esc = reg.shape_desc(map, block::CTOR_TAG_MASK + 1);
    let index = block::ctor_index(block::LAYOUT_BIG, esc);
    let fits = b.reg();
    b.line(&format!(
        "{fits} = icmp ule i64 {tag}, {}",
        block::CTOR_TAG_MASK
    ));
    let regular = b.reg();
    b.line(&format!("{regular} = or i64 {tag}, {group_base}"));
    let shifted = b.reg();
    b.line(&format!(
        "{shifted} = shl i64 {tag}, {}",
        block::CTOR_ESC_TAG_SHIFT
    ));
    let escaped = b.reg();
    b.line(&format!(
        "{escaped} = or i64 {shifted}, {}",
        block::ctor_escaped(index, 0) as i64
    ));
    let d = b.reg();
    b.line(&format!(
        "{d} = select i1 {fits}, i64 {regular}, i64 {escaped}"
    ));
    d
}

/// The constructor tag of the constructor block at `p`: the header's
/// layout id masked to its tag bits, or, when the program may hold
/// escaped descriptors (`escapes`), the descriptor word's tag for a
/// header that says `LAYOUT_BIG`.
fn ctor_tag(b: &mut Builder, p: &str, escapes: bool) -> String {
    let hp = b.gep_dyn(p, "-8");
    let h = header_load(b, &hp);
    let l = b.reg();
    b.line(&format!("{l} = lshr i64 {h}, {}", block::LAYOUT_SHIFT));
    if !escapes {
        let t = b.reg();
        b.line(&format!("{t} = and i64 {l}, {}", block::CTOR_TAG_MASK));
        return t;
    }
    let big = b.reg();
    b.line(&format!("{big} = icmp eq i64 {l}, {}", block::LAYOUT_BIG));
    let (lesc, lreg, ljoin) = (b.label(), b.label(), b.label());
    b.line(&format!("br i1 {big}, label %{lesc}, label %{lreg}"));
    b.start(&lesc);
    let xp = b.gep_dyn(p, "-16");
    let x = b.reg();
    b.line(&format!("{x} = load i64, ptr {xp}"));
    let xs = b.reg();
    b.line(&format!(
        "{xs} = lshr i64 {x}, {}",
        block::CTOR_ESC_TAG_SHIFT
    ));
    let te = b.reg();
    b.line(&format!(
        "{te} = and i64 {xs}, {}",
        (1u64 << block::CTOR_ESC_TAG_BITS) - 1
    ));
    b.line(&format!("br label %{ljoin}"));
    b.start(&lreg);
    let tr = b.reg();
    b.line(&format!("{tr} = and i64 {l}, {}", block::CTOR_TAG_MASK));
    b.line(&format!("br label %{ljoin}"));
    b.start(&ljoin);
    let t = b.reg();
    b.line(&format!(
        "{t} = phi i64 [ {te}, %{lesc} ], [ {tr}, %{lreg} ]"
    ));
    t
}

/// An immortal static constructor block with descriptor `desc` and
/// payload `words` (`static_shaped_block`).
fn static_ctor_block(pointer_bytes: usize, sym: &str, desc: u64, words: &[StaticWord]) -> String {
    static_shaped_block(pointer_bytes, sym, block::KIND_CTOR, desc, words)
}

/// One word of a static block's payload.
enum StaticWord {
    /// A word whose value the compiler knows.
    Int(i64),
    /// A word holding a pointer: the address the linker relocates.
    Ptr(String),
}

impl StaticWord {
    fn int(v: usize) -> Self {
        StaticWord::Int(v as i64)
    }
}

/// An immortal static block of a shaped kind with descriptor `desc` and
/// payload `words`: the storage `@"<sym>#h"` puts the block header first
/// (an escaped descriptor adds its word before it). Every reference to the
/// block goes through [`static_payload`], the constant offset into that one
/// object, so a static block is addressed exactly as a heap block's is.
///
/// A word holding a pointer is spelled at the target's own pointer width.
/// A linker relocates an address, not a widened copy of one, so
/// `i64 ptrtoint (ptr @f to i64)` is not a constant a 32-bit Wasm module
/// can initialise storage with (`unsupported expression in static
/// initializer`). Where the pointer is 4 bytes the payload is emitted as
/// `[2n x i32]`, each word its low half then its high half: the bytes are
/// the same on a little-endian target, so the runtime reads the block
/// unchanged, and the pointer occupies the half the linker can fill.
fn static_shaped_block(
    pointer_bytes: usize,
    sym: &str,
    kind: u8,
    desc: u64,
    words: &[StaticWord],
) -> String {
    let field = block::ctor_layout_field(desc);
    let hdr = block::static_header(kind, field) as i64;
    let mut init: Vec<StaticWord> = Vec::new();
    if field == block::LAYOUT_BIG {
        init.push(StaticWord::Int(desc as i64));
    }
    init.push(StaticWord::Int(hdr));
    init.extend(words.iter().map(|w| match w {
        StaticWord::Int(v) => StaticWord::Int(*v),
        StaticWord::Ptr(p) => StaticWord::Ptr(p.clone()),
    }));
    if pointer_bytes == 4 {
        // The halves: a value splits into its low and high 32 bits, and a
        // pointer is its own 32-bit address followed by a zero high half
        // (the address is below 4 GiB by construction, so the high half
        // is never a sign extension).
        let mut halves: Vec<String> = Vec::with_capacity(init.len() * 2);
        for w in &init {
            match w {
                StaticWord::Int(v) => {
                    halves.push(format!("i32 {}", *v as u64 as u32 as i32));
                    halves.push(format!("i32 {}", ((*v as u64) >> 32) as u32 as i32));
                }
                StaticWord::Ptr(p) => {
                    halves.push(format!("i32 ptrtoint (ptr {p} to i32)"));
                    halves.push("i32 0".to_string());
                }
            }
        }
        return format!(
            "@{storage} = internal constant [{len} x i32] [{init}], align 8\n\n",
            storage = quote(&format!("{sym}#h")),
            len = halves.len(),
            init = halves.join(", "),
        );
    }
    let items: Vec<String> = init
        .iter()
        .map(|w| match w {
            StaticWord::Int(v) => format!("i64 {v}"),
            StaticWord::Ptr(p) => format!("i64 ptrtoint (ptr {p} to i64)"),
        })
        .collect();
    format!(
        "@{storage} = internal constant [{len} x i64] [{init}]\n\n",
        storage = quote(&format!("{sym}#h")),
        len = items.len(),
        init = items.join(", "),
    )
}

/// The payload of the static block `sym` with descriptor `desc`: the
/// constant offset into the object [`static_shaped_block`] emits.
///
/// The block used to carry a second symbol, `@"<sym>"`, a global alias at
/// the payload. A linker is free to place an alias whose aliasee sits in a
/// mergeable constant section, and `ld64` does: with the stage module split
/// across nine objects, five zero-length `.blk` aliases came out at the
/// address of an unrelated constant while their headers stayed 3 to 12 KB
/// away. The runtime then read a header that was not there, saw a count
/// above one, and decremented a constant in `__TEXT` (a `SIGBUS` on Apple,
/// exit -10). An offset the compiler folds into a relocation against the
/// one storage symbol cannot be separated from its header by any linker.
fn static_payload(sym: &str, desc: u64) -> String {
    let words = if block::ctor_layout_field(desc) == block::LAYOUT_BIG { 2 } else { 1 };
    format!(
        "getelementptr inbounds (i8, ptr @{storage}, i64 {offset})",
        storage = quote(&format!("{sym}#h")),
        offset = 8 * words,
    )
}


/// Emit a zero-field constructor's static block once, at the site that takes
/// its address.
///
/// The address can be taken from a unit that is never emitted (an object
/// build selects roots, and an imported constructor's unit belongs to the
/// library), so the site that mints the pointer owns the definition. The
/// block is one constant word, so emitting it here costs nothing that the
/// constructor's own unit would not have paid.
fn ensure_static_ctor(b: &mut Builder, reg: &Registry, name: &str, desc: u64) {
    let symbol = format!("{name}.blk");
    if reg.emitted_blocks.borrow_mut().insert(symbol.clone()) {
        b.global(static_ctor_block(b.pointer_bytes, &symbol, desc, &[]));
    }
}

/// The immortal static dictionary of an instance whose `.mk` builds a
/// constant block ([`Registry::static_dict`]).
fn static_dict_name(id: &str) -> String {
    format!("{id}.dict")
}

/// The worker symbol of a widened unit. `#` appears in no source
/// identifier, so the name cannot meet another unit's.
fn tail_worker(unit: &str) -> String {
    format!("{unit}#tail")
}

/// A direct call's argument list padded to the callee's tail signature:
/// a widened worker ignores its trailing words.
fn padded_args(vals: &[String], width: usize) -> String {
    let mut out: Vec<String> = vals.iter().map(|v| format!("i64 {v}")).collect();
    out.resize(width.max(vals.len()), "i64 undef".to_string());
    out.join(", ")
}

enum UnitBody<'a> {
    Prim(PrimOp),
    /// A constructor unit of this many fields (its tag is in its layout
    /// id, `Registry::ctor_desc`).
    Ctor(usize),
    Mk(&'a InstInfo),
    Fn(&'a FnDef),
    /// A widened unit's body under its group's tail signature
    /// (`Registry::tail_sig`); the unit's own symbol becomes a one-call
    /// wrapper into it.
    TailWorker(&'a FnDef),
    /// A real foreign import's synthesized marshalling wrapper (foreign
    /// function interface specification section 2).
    Foreign(&'a crate::core::ForeignPlan),
}

/// What one unit's lowered body does, scanned from the emitted
/// instructions (design/llvm-contract-attributes.md: an attribute the
/// scan cannot support is omitted, never guessed).
#[derive(Clone, Debug, Default)]
struct UnitScan {
    /// A load of memory that is not the unit's own stack slot.
    reads: bool,
    /// A store to non-local memory, an allocation or a memory copy.
    writes: bool,
    /// A call to the fault path (`cinch_panic`, `__cinch_blackhole`,
    /// `exit`).
    faults: bool,
    /// An instruction that can produce poison for some inputs
    /// (division, an unmasked shift, float-to-integer narrowing).
    poison: bool,
    /// A call that may free memory allocated before the call (scope
    /// teardown, task joins).
    frees: bool,
    /// An indirect application (`__apply`, `__cinch_demand`): the
    /// callee is unknown.
    indirect: bool,
    /// An extern symbol the scan cannot see through.
    externs: bool,
    /// The module units this body calls directly.
    callees: BTreeSet<String>,
    /// Every module unit this body or its globals mention, by call or as
    /// data. A unit's static block, closure descriptor and per-arity
    /// entries all hang off its name, and a body reaches them with
    /// `ptrtoint` or an alias rather than a call, so reachability has to
    /// follow these too: a program that takes a constructor as a value
    /// calls nothing, yet needs that constructor's `.blk`.
    refs: BTreeSet<String>,
    /// The units whose `.clo` or `.app` this body names, before `refs`
    /// folds an artifact name into the unit it belongs to. A unit whose
    /// closure nobody takes needs no apply stub, which is the largest
    /// single piece of a small unit's output.
    closures_taken: BTreeSet<String>,
}

/// The memory class of one unit (the `memory(...)` function attribute).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
enum MemClass {
    /// No memory access outside the unit's own stack slots.
    #[default]
    None,
    /// Reads memory, writes none.
    Read,
    /// May write memory (the conservative class).
    ReadWrite,
}

/// One unit's classification after the call-graph fixed point: the
/// function attributes of the LLVM contract surface, each one claimed
/// only where the language facts and the body scan agree. The scan
/// properties are what the lowered body proves; the language rows gate
/// which definitions restate them (a fact the analysis cannot state is
/// never guessed).
#[derive(Clone, Copy, Debug)]
struct UnitClass {
    mem: MemClass,
    /// The unit reaches a foreign call the scan cannot see through, so
    /// `nounwind` cannot be claimed: an unwinding extern would unwind
    /// through it.
    externs: bool,
    /// `norecurse`: the unit is in no cycle of the call graph and reaches
    /// no indirect application. Computed from the same `scan.callees`
    /// edges the other classes follow, by finding the strongly connected
    /// components once.
    ///
    /// An indirect application is a cycle the edge set cannot see: the
    /// callee is a word, and a unit's own apply stub calls it back
    /// (`<unit>.app` applies the closure, and an over-application pushes
    /// the excess words through `__apply` at the result). So a unit that
    /// reaches `__apply`, directly or through a callee, may call itself
    /// with nothing in the graph saying so, and it is recursive for this
    /// attribute's purpose.
    recursive: bool,
    /// `speculatable`: no effects and no undefined behavior (no fault
    /// path, no poison source, no opaque call).
    speculatable: bool,
    /// `willreturn`: every path returns (no fault path, no opaque call,
    /// no teardown) and every call it makes returns.
    willreturn: bool,
    /// `nofree`: nothing in the body (or its callees) can deallocate
    /// (the scan cannot see through an opaque call).
    nofree: bool,
    /// `noreturn`: no path through the body reaches a `ret`. Every
    /// block reachable from `entry` ends in `unreachable`, in a call the
    /// module marks `noreturn` (`cinch_panic`, `__cinch_blackhole`,
    /// `exit`), or in a branch, and the walk finds no live return.
    ///
    /// This is a property of the lowered control flow, not of the
    /// declared effect row: a `{Diverge}` row says the definition *may*
    /// not return, so a divergent row is not evidence. What is evidence
    /// is that the emitted body has no returning path at all, which is
    /// exactly what the attribute asserts.
    noreturn: bool,
}

/// The language-level facts gating one unit's contract attributes.
#[derive(Clone, Copy, Debug, Default)]
struct UnitLang {
    /// The definition's effect rows are empty and its body calls no
    /// foreign symbol (the plan's gate for `memory(none) speculatable`).
    pure: bool,
    /// The unit is a checked definition: the P20 and totality rows
    /// below gate its `nofree`/`willreturn` claims. The builtin
    /// vocabulary (primitives, constructors, instances) claims them on
    /// the body scan alone.
    def: bool,
    /// P20's `#[no-alloc]` row (decision D112).
    no_alloc: bool,
    /// The totality/`bounded` machinery proved return.
    returns: bool,
    /// P20's `#[checked-params]` row.
    checked_params: bool,
}

/// One unit's emitted pieces and the facts its attributes rest on.
/// Units are built first and attributed after, so a unit's attribute
/// group can honor its callees' classification.
struct EmittedUnit {
    name: String,
    /// The hint attributes of the unit's definition (erasable).
    hints: crate::core::FnHints,
    /// Private constants this unit needs, emitted before its define.
    globals: String,
    /// The `define` header without its function attribute group.
    header: String,
    allocas: String,
    code: String,
    stub: String,
    clo: String,
    scan: UnitScan,
    lang_facts: UnitLang,
}

/// What one call target does to memory, for the unit scan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Callee {
    /// A module unit: classified through the call-graph fixed point.
    Unit,
    /// An LLVM intrinsic that touches no memory.
    Pure,
    /// An allocation (`cinch_alloc_raw`/`cinch_alloc_ctor`, the runtime's
    /// accounted allocator).
    Alloc,
    /// A memory copy or fill (`memcpy`, `memset`).
    Copy,
    /// The fault path: never returns.
    Fault,
    /// A teardown call that may free memory allocated before it.
    Free,
    /// A runtime extern the scan cannot see through.
    Extern,
    /// An indirect application.
    Indirect,
}

/// Classify one call target by its symbol (the unit names decide
/// module units; everything else is a runtime symbol).
fn classify_callee(sym: &str, units: &HashSet<String>, extra: Option<&str>) -> Callee {
    if units.contains(sym) || extra == Some(sym) {
        return Callee::Unit;
    }
    match sym {
        "cinch_alloc_raw" | "cinch_alloc_ctor" | "cinch_alloc_shaped" => Callee::Alloc,
        "memcpy" => Callee::Copy,
        // A dup writes a header (design/reclamation.md section 11, item 9);
        // the uniqueness test reads one, and its unit also frees.
        "cinch_dup" | "cinch_dup_mt" | "cinch.rc.dup.obj" | "cinch.rc.dup.tobj" | RC_UNIQUE => Callee::Copy,
        // A drop may free what the caller passed or allocated before the
        // call, and so does the raw release of a unique closure, and the
        // frame release, which drops the block's counted children.
        "cinch_drop" | "cinch_drop_slow" | "cinch_drop_mt" | "cinch_free" | "cinch_frame_release" | "cinch.rc.drop.obj"
        | "cinch.rc.drop.tobj" => Callee::Free,
        "__apply" | "__cinch_demand" => Callee::Indirect,
        "exit" | "cinch_panic" | "__cinch_blackhole" => Callee::Fault,
        "cinch_scope_close"
        | "cinch_coop_scope_close"
        | "cinch_task_join_all_or_cancel"
        | "cinch_coop_task_join_all_or_cancel" => Callee::Free,
        s if s.starts_with("llvm.mem") => Callee::Copy,
        // A prefetch reads no value and writes none, but a unit whose only
        // work is one must not be classified pure: a pure call with an
        // unused result is dead code, and the hint would be deleted before
        // it could be inlined (`crate::slots`'s `slotsPrefetch`).
        s if s.starts_with("llvm.prefetch") => Callee::Extern,
        s if s.starts_with("llvm.") => Callee::Pure,
        _ => Callee::Extern,
    }
}

/// A primitive's declared signature, where it has one: the deferral and
/// code-generation forms (`delay`/`force`/`demand`/`eval` and the
/// concurrency builtins) are builtin values, not foreign imports, and
/// carry no boundary signature.
fn prim_sig(op: PrimOp) -> Option<(Vec<crate::types::Ty>, crate::types::Ty)> {
    use PrimOp::*;
    if matches!(op, Delay | Force | ApplyErased | Eval | Conc(_) | PrimOp::Par { .. }) {
        return None;
    }
    Some(op.sig())
}

/// The memory operand of a `load`/`store` line (the first `ptr %reg`
/// or `ptr @global` field after the opcode).
fn mem_operand(line: &str) -> Option<String> {
    let rest = line.split_once(", ")?.1;
    for f in rest.split(", ") {
        if f.starts_with("ptr %") || f.starts_with("ptr @") {
            return Some(f[4..].to_string());
        }
    }
    None
}

/// The callee symbol of a `call` line.
fn callee_symbol(line: &str) -> Option<String> {
    let i = line.find("call ")?;
    let rest = &line[i + "call ".len()..];
    let at = rest.find('@')?;
    let rest = &rest[at + 1..];
    if let Some(q) = rest.strip_prefix('"') {
        let end = q.find('"')?;
        return Some(q[..end].to_string());
    }
    let end = rest.find('(')?;
    Some(rest[..end].to_string())
}

/// The units whose closure artifact (`.clo` or `.app`) `text` names.
///
/// Kept separate from [`referenced_symbols`], which resolves an artifact
/// name to the unit it belongs to and so cannot say whether the unit was
/// used as a value or merely called.
fn closures_named(text: &str, units: &HashSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for unit in units {
        for suffix in [".clo", ".app"] {
            let needle = format!("{unit}{suffix}");
            if text.contains(&needle) {
                out.insert(unit.clone());
            }
        }
    }
    out
}

/// Every module unit `text` names, in any position.
///
/// A unit's static block, closure descriptor and per-arity entries all
/// hang off its name, and a body reaches them as data (`ptrtoint`, an
/// alias, an initializer) rather than with a `call`. Reachability that
/// followed calls alone would drop a constructor a program only ever
/// takes as a value, and the emitted IR would then name a block nothing
/// defines.
fn referenced_symbols(text: &str, units: &HashSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'@' {
            i += 1;
            continue;
        }
        i += 1;
        let (start, end) = if i < bytes.len() && bytes[i] == b'"' {
            let start = i + 1;
            match text[start..].find('"') {
                Some(off) => (start, start + off),
                None => break,
            }
        } else {
            let start = i;
            let mut end = i;
            while end < bytes.len() {
                let c = bytes[end];
                if c.is_ascii_alphanumeric() || c == b'_' || c == b'.' || c == b'$' || c == b'-' {
                    end += 1;
                } else {
                    break;
                }
            }
            (start, end)
        };
        if end > start {
            // A unit's artifacts all hang off its name: the static block
            // (`X.blk#h`), the closure descriptor (`X.clo#h`), the
            // per-arity entries (`X.app`, `X.clo`). The longest unit name
            // the reference starts with is the unit it belongs to.
            let name = &text[start..end];
            // An exact unit name wins outright: a prim family unit
            // (`X.llvm`) is a unit of its own beside its wrapper `X`, and
            // the prefix rule below would otherwise fold it into the
            // wrapper and never emit the body the wrapper calls.
            if units.contains(name) {
                out.insert(name.to_string());
            }
            let mut best: Option<&str> = None;
            for unit in units {
                // A tail worker is `<unit>#w` (`tail_worker`), so `#`
                // separates a unit from its artifacts exactly as `.` does.
                let prefix_matches = name == unit
                    || name
                        .strip_prefix(unit.as_str())
                        .is_some_and(|rest| rest.starts_with('.') || rest.starts_with('#'));
                if prefix_matches && best.is_none_or(|b| unit.len() > b.len()) {
                    best = Some(unit);
                }
            }
            // A tail worker `<unit>#tail` is reached only through its
            // unit, which keeps its own one-word symbol for closures and
            // exports (`tail_worker`), so the worker keeps its parent.
            if let Some(unit) = name.split('#').next().filter(|base| *base != name && units.contains(*base)) {
                out.insert(unit.to_string());
            }
            if let Some(unit) = best {
                out.insert(unit.to_string());
            } else if let Some(id) = name.split('#').next().and_then(|n| n.strip_suffix(".dict")) {
                // A static dictionary block (`<instance>.dict`) is emitted
                // by that instance's `.mk` factory, which is the unit that
                // must stay.
                let mk = format!("{id}.mk");
                if units.contains(&mk) {
                    out.insert(mk);
                }
            }
        }
        i = end.max(i);
    }
    out
}

/// Scan one unit's lowered body (its `alloca` block and its code) for
/// the facts its contract attributes rest on.
fn scan_body(text: &str, units: &HashSet<String>, extra: Option<&str>) -> UnitScan {
    let mut scan = UnitScan::default();
    // Stack slots and the addresses derived from them are the unit's
    // own memory: accesses to them are not memory effects.
    let mut local: HashSet<String> = HashSet::new();
    for line in text.lines() {
        let line = line.trim();
        // A result register: allocas and the addresses derived from
        // them are the unit's own memory. An assigning call still
        // classifies below: a missed call would over-claim every
        // contract attribute the scan feeds.
        if let Some((lhs, rest)) = line.split_once(" = ") {
            let lhs = lhs.trim();
            let rest = rest.trim_start();
            if rest.starts_with("alloca ") {
                local.insert(lhs.to_string());
            } else if rest.starts_with("getelementptr ") {
                if let Some(base) = mem_operand(line) {
                    if local.contains(&base) {
                        local.insert(lhs.to_string());
                    }
                }
            }
        }
        if line.contains("call ") {
            if let Some(sym) = callee_symbol(line) {
                match classify_callee(&sym, units, extra) {
                    Callee::Unit => {
                        scan.callees.insert(sym);
                    }
                    Callee::Pure => {}
                    Callee::Alloc | Callee::Copy => scan.writes = true,
                    Callee::Fault => {
                        scan.writes = true;
                        scan.faults = true;
                    }
                    Callee::Free => {
                        scan.writes = true;
                        scan.frees = true;
                    }
                    Callee::Extern => {
                        scan.writes = true;
                        scan.externs = true;
                    }
                    Callee::Indirect => {
                        scan.writes = true;
                        scan.indirect = true;
                    }
                }
            } else {
                scan.writes = true;
                scan.externs = true;
            }
            continue;
        }
        if line.contains("load ") && line.contains(", ptr ") {
            if let Some(op) = mem_operand(line) {
                if !local.contains(&op) {
                    scan.reads = true;
                }
            } else {
                scan.reads = true;
            }
        } else if line.starts_with("store ") {
            if let Some(op) = mem_operand(line) {
                if !local.contains(&op) {
                    scan.writes = true;
                }
            } else {
                scan.writes = true;
            }
        }
        if line == "unreachable" {
            scan.faults = true;
            continue;
        }
        let poison_op = [
            "= sdiv ", "= udiv ", "= srem ", "= urem ", "= fptosi", "= fptoui",
        ]
        .iter()
        .any(|p| line.contains(p));
        let shift = ["= shl ", "= lshr ", "= ashr "]
            .iter()
            .any(|p| line.contains(p));
        let variable_shift = shift
            && line
                .rsplit_once(", ")
                .map(|(_, amt)| {
                    !(amt.starts_with('-')
                        || amt.chars().next().is_some_and(|c| c.is_ascii_digit()))
                })
                .unwrap_or(true);
        if poison_op || variable_shift {
            scan.poison = true;
        }
    }
    scan
}

/// Whether the lowered body has no returning path, for `noreturn`.
///
/// The proof is a reachability walk over the body's own blocks: start at
/// `entry`, follow every successor of every reachable terminator, and ask
/// whether any reachable block ends in `ret`. A body whose every
/// reachable path ends in `unreachable` (or in a call the module marks
/// `noreturn`, which the emitter always follows with `unreachable`)
/// never returns, so the attribute restates the control flow rather than
/// a claim about it.
///
/// The walk reads the emitted text because that is the artifact the
/// attribute describes. `Builder` already guarantees every block is
/// terminated (`start` clears the flag and `line` sets it on a
/// terminator), so a successor set is complete: a `br` names all of its
/// targets, a `switch` names every case and its default, and a `ret` or
/// `unreachable` names none.
fn body_never_returns(text: &str) -> bool {
    use std::collections::HashMap;
    // One entry per block: the successors its terminator names, and
    // whether that terminator is a return.
    let mut succ: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut returns: HashMap<&str, bool> = HashMap::new();
    // The emitted body starts in `entry`, and `Builder::finish` writes
    // that label explicitly. A body with no further labels is one block,
    // so `entry` is the current block from the first line: an
    // instruction before any label belongs to it.
    let mut current: &str = "entry";
    succ.entry("entry").or_default();
    returns.entry("entry").or_insert(false);
    for line in text.lines() {
        let line = line.trim();
        if let Some(label) = line.strip_suffix(':') {
            // A label opens a block. Register it even when it has no
            // terminator, so a branch to it cannot be mistaken for a
            // missing block.
            current = label;
            succ.entry(label).or_default();
            returns.entry(label).or_insert(false);
            continue;
        }
        let block = current;
        if line.starts_with("ret ") || line == "ret void" {
            returns.insert(block, true);
        }
        if line.starts_with("br label %") {
            if let Some((_, target)) = line.split_once("label %") {
                succ.entry(block).or_default().push(target);
            }
        } else if line.starts_with("br i1") || line.starts_with("switch ") {
            for part in line.split("label %").skip(1) {
                let target = part.split(|c: char| c == ',' || c.is_whitespace()).next();
                if let Some(target) = target {
                    succ.entry(block).or_default().push(target);
                }
            }
        }
    }
    // Reachability from the entry block.
    let mut seen: HashSet<&str> = HashSet::new();
    let mut stack: Vec<&str> = vec!["entry"];
    while let Some(block) = stack.pop() {
        if !seen.insert(block) {
            continue;
        }
        if returns.get(block).copied().unwrap_or(false) {
            // A live return: some input reaches it.
            return false;
        }
        if let Some(next) = succ.get(block) {
            stack.extend(next.iter().copied());
        }
    }
    true
}

/// The units that may call themselves back, for `norecurse`.
///
/// Two edge kinds make a unit recursive, and both have to be followed:
///
/// * a direct call to a unit that reaches it back (a cycle of the call
///   graph), which is the strongly connected component test, and
/// * an indirect application (`__apply`/`__cinch_demand`) anywhere in the
///   unit's reachable closure. The callee is a word, so the graph has no
///   edge to follow, and a unit's own apply stub closes the loop: the
///   stub applies the closure and an over-application pushes the excess
///   words at the result through `__apply`. A unit that reaches an
///   indirect call therefore may re-enter itself with nothing in the edge
///   set saying so.
///
/// The answer is the transitive closure of both, computed once. Every
/// other class in [`UnitClass`] is a "must hold everywhere" property and
/// a fixed point over the direct edges is enough; this one is a "cannot
/// happen anywhere" property, so a cycle (or an invisible edge) anywhere
/// below the unit is what makes it false.
fn recursive_units(units: &[EmittedUnit]) -> HashSet<String> {
    // The direct edges, restricted to units this module defines: a call
    // to a symbol outside the set cannot reach back into it.
    let defined: HashSet<&str> = units.iter().map(|u| u.name.as_str()).collect();
    let edges: HashMap<&str, Vec<&str>> = units
        .iter()
        .map(|u| {
            let mut out: Vec<&str> = u
                .scan
                .callees
                .iter()
                .map(String::as_str)
                .filter(|g| defined.contains(g))
                .collect();
            out.sort_unstable();
            (u.name.as_str(), out)
        })
        .collect();
    let indirect: HashSet<&str> = units
        .iter()
        .filter(|u| u.scan.indirect)
        .map(|u| u.name.as_str())
        .collect();
    // A unit is recursive when it reaches itself, or when it reaches a
    // unit that makes an indirect call. Both are reachability questions
    // over the same edges, so one search per unit answers both.
    let mut out: HashSet<String> = HashSet::new();
    for u in units {
        let start = u.name.as_str();
        let mut seen: HashSet<&str> = HashSet::new();
        let mut stack: Vec<&str> = edges.get(start).cloned().unwrap_or_default();
        let mut recursive = false;
        while let Some(n) = stack.pop() {
            if n == start {
                // A cycle back to the start: the unit calls itself.
                recursive = true;
                break;
            }
            if !seen.insert(n) {
                continue;
            }
            if indirect.contains(n) {
                // Something below this unit applies a closure, and the
                // closure may be this unit's own.
                recursive = true;
                break;
            }
            if let Some(next) = edges.get(n) {
                stack.extend(next.iter().copied());
            }
        }
        if recursive {
            out.insert(u.name.clone());
        }
    }
    out
}

/// The call-graph fixed point over the built units: every claim is
/// transitive through direct calls, and `willreturn` excludes only the
/// unit's own recursion (the rows gate which definitions restate the
/// scan's facts at claim time).
fn classify_units(units: &[EmittedUnit]) -> HashMap<String, UnitClass> {
    let mut out: HashMap<String, UnitClass> = HashMap::new();
    let recursive = recursive_units(units);
    // A unit whose body has no returning path at all. This is the local
    // half of `noreturn`; the call-graph half is a call to a callee that
    // itself never returns, which the fixed point below folds in.
    let local_noreturn: HashMap<&str, bool> = units
        .iter()
        .map(|u| (u.name.as_str(), body_never_returns(&format!("{}{}", u.allocas, u.code))))
        .collect();
    for u in units {
        let mem = if u.scan.writes {
            MemClass::ReadWrite
        } else if u.scan.reads {
            MemClass::Read
        } else {
            MemClass::None
        };
        let clean = !u.scan.faults && !u.scan.poison && !u.scan.indirect && !u.scan.externs;
        // A body with no returning path contradicts both claims: it does
        // not return, so `willreturn` is false, and `speculatable` is
        // false too because executing it is not a finite, side-effect
        // free step the optimizer may repeat or sink.
        let returns = !local_noreturn[u.name.as_str()];
        out.insert(
            u.name.clone(),
            UnitClass {
                mem,
                externs: u.scan.externs,
                recursive: recursive.contains(&u.name),
                speculatable: clean && !u.scan.frees && returns,
                willreturn: clean && !u.scan.frees && returns,
                // The scan cannot see through an opaque call, and a
                // fault path's `noreturn` never frees observably: only
                // a body free of both can claim `nofree`.
                nofree: !u.scan.frees && !u.scan.faults && !u.scan.externs && !u.scan.indirect,
                noreturn: local_noreturn[u.name.as_str()],
            },
        );
    }
    loop {
        let mut changed = false;
        for u in units {
            let mut c = out[&u.name];
            for g in &u.scan.callees {
                let Some(gc) = out.get(g) else {
                    continue;
                };
                c.mem = c.mem.max(gc.mem);
                c.externs = c.externs || gc.externs;
                c.speculatable = c.speculatable && gc.speculatable;
                c.willreturn = c.willreturn && (g == &u.name || gc.willreturn);
                c.nofree = c.nofree && gc.nofree;
                // A body that would return only after a callee returns
                // never returns when that callee never does. This is the
                // one direction the local walk cannot see: the walk stops
                // at the call, so the callee's answer has to fold in.
                c.noreturn = c.noreturn || gc.noreturn;
            }
            if c.mem != out[&u.name].mem
                || c.externs != out[&u.name].externs
                || c.speculatable != out[&u.name].speculatable
                || c.willreturn != out[&u.name].willreturn
                || c.nofree != out[&u.name].nofree
                || c.noreturn != out[&u.name].noreturn
            {
                out.insert(u.name.clone(), c);
                changed = true;
            }
        }
        if !changed {
            return out;
        }
    }
}

/// Assemble one unit's text from its built pieces under the given
/// attribute group.

/// Drop repeated definitions of the same static block storage.
///
/// A zero-field constructor's `@"<sym>.blk#h"` is emitted at the first site
/// that needs its address, and a module can reach that address from more
/// than one unit (a constructor unit of its own, a prim wrapper, a
/// specialised copy). LLVM allows one definition, so the assembler keeps the
/// first and drops the identical repeats; a *differing* repeat is a bug and
/// is left for the verifier to refuse.
fn dedupe_static_blocks(text: &str) -> String {
    use std::collections::HashSet;
    let mut seen: HashSet<&str> = HashSet::new();
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if line.starts_with('@') && line.contains(".blk#h\" = internal constant") {
            let end = line.find('"').map(|first| {
                line[first + 1..].find('"').map(|second| first + 1 + second).unwrap_or(first)
            });
            match end {
                Some(last) => {
                    let symbol = &line[1..last];
                    if !seen.insert(symbol) {
                        continue;
                    }
                }
                None => {}
            }
        }
        out.push_str(line);
    }
    out
}

fn assemble_unit(
    out: &mut String,
    u: &EmittedUnit,
    group: Option<String>,
    linkage: Option<&str>,
    closure_taken: bool,
) {
    out.push_str(&u.globals);
    // The function attribute group precedes metadata attachments on the
    // `define` line (`define ... #4 !immarg !0 {`).
    let header = match linkage {
        Some(linkage) => u
            .header
            .replacen("define ", &format!("define {linkage} "), 1),
        None => u.header.clone(),
    };
    let (head, meta) = match (group.is_some(), header.split_once(" !")) {
        (true, Some((h, m))) => (h.to_string(), format!(" !{m}")),
        _ => (header, String::new()),
    };
    out.push_str(&head);
    if let Some(group) = group {
        out.push(' ');
        out.push_str(&group);
    }
    out.push_str(&meta);
    out.push_str(" {\nentry:\n");
    out.push_str(&u.allocas);
    out.push_str(&u.code);
    out.push_str("}\n\n");
    // The apply stub and the static closure are only reachable through a
    // use of the unit as a value. A unit called directly and never passed
    // as a function needs neither, and the stub is the largest single
    // piece of a small unit's output.
    if closure_taken {
        let stub = match linkage {
            Some(linkage) => with_linkage(&u.stub, linkage),
            None => u.stub.clone(),
        };
        out.push_str(&stub);
        out.push_str(&u.clo);
    }
}

/// The units a whole program can reach from its entry points.
///
/// A unit's body is the only way to reach another unit: `scan.callees` is
/// the call graph, and a unit's stub and closure family hang off the same
/// name, so a reachable name carries its own `.app` entries and `.clo`
/// block. The roots are the process entry's unit and every `foreign
/// export` target, which is what a linker outside the program can name.
///
/// Names that are not units at all (runtime externs, LLVM intrinsics) are
/// absent from the map and end the walk. That is deliberate: they are
/// declared, never defined here.
fn reachable_units(
    units: &[EmittedUnit],
    reg: &Registry,
    program: &Program,
    main: Option<String>,
) -> HashSet<String> {
    let mut callees: HashMap<&str, &BTreeSet<String>> =
        units.iter().map(|u| (u.name.as_str(), &u.scan.refs)).collect();
    // A `prim` declaration's bodies are separate units (`X.llvm`,
    // `X.asm`) that its wrapper calls, and a body's `uses` contract may
    // name another body's symbol, so every family unit of a reached prim
    // is reached with it. A body this artifact does not select is still
    // emitted, as it was before pruning: selection is recorded in the
    // manifest, and an inspection of the IR sees both.
    let mut families: HashMap<String, BTreeSet<String>> = HashMap::new();
    for plan in &program.prims {
        families.insert(
            plan.unit.clone(),
            plan.family_units
                .iter()
                .map(|(fam, _)| format!("{}.{}", plan.unit, fam))
                .collect(),
        );
    }
    let mut roots: Vec<&str> = Vec::new();
    if let Some(main) = &main {
        roots.push(main.as_str());
    }
    for plan in reg.foreigns.values() {
        if plan.export {
            roots.push(plan.unit.as_str());
        }
    }
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<&str> = roots;
    while let Some(name) = stack.pop() {
        if !seen.insert(name.to_string()) {
            continue;
        }
        if let Some(next) = callees.get(name) {
            for callee in next.iter() {
                if !seen.contains(callee.as_str()) {
                    stack.push(callee.as_str());
                }
            }
        }
        if let Some(bodies) = families.get(name) {
            for body in bodies {
                if !seen.contains(body.as_str()) && callees.contains_key(body.as_str()) {
                    stack.push(body.as_str());
                }
            }
        }
    }
    let _ = &mut callees;
    seen
}

/// Prefix `linkage ` to every top-level `define` in `text`. A unit's stub
/// is a family of functions (the dispatcher, its per-arity entries and the
/// shared `many` body), and all of them take the unit's linkage.
fn with_linkage(text: &str, linkage: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("define ") {
            out.push_str("define ");
            out.push_str(linkage);
            out.push(' ');
            out.push_str(rest);
        } else {
            out.push_str(line);
        }
    }
    out
}

/// Build one unit's pieces and the scan of its body (phase one of the
/// emission: the attributes wait for the call-graph classification).
fn build_unit(
    cx: &LowerCx,
    reg: &Registry,
    name: &str,
    arity: usize,
    body: UnitBody,
) -> Result<EmittedUnit> {
    let mut b = Builder::new();
    b.pointer_bytes = reg.pointer_bytes;
    b.rc_debug = reg.rc_debug;
    b.reclaim = reg.reclaim;
    // The counted parameters a primitive or foreign wrapper only lends to
    // the runtime or the foreign side, released on every return.
    let borrowed = |consumes: &dyn Fn(usize) -> bool| -> Vec<(String, Class)> {
        let cs = reg.unit_params.get(name).cloned().unwrap_or_default();
        (0..arity)
            .filter_map(|i| {
                let c = cs.get(i).copied().unwrap_or(Class::Raw);
                (counted_class(c) && !consumes(i)).then(|| (format!("%p{i}"), c))
            })
            .collect()
    };
    let mut globals = String::new();
    let mut hints = crate::core::FnHints::default();
    let mut lang = UnitLang {
        pure: true,
        ..UnitLang::default()
    };
    let mut ret_range: Option<(i64, i64)> = None;
    let mut noundef_params: Vec<bool> = Vec::new();
    // The width each position crosses at, when the analysis proved one
    // (design/narrow-value-representation.md section 4). The registry
    // decided which units narrow, so the header, every call site and the
    // apply stub read the one answer.
    let narrow = match &body {
        UnitBody::Fn(_) => reg.narrow_sigs.get(name),
        _ => None,
    };
    match &body {
        UnitBody::Prim(op) => {
            // A primitive that returns `Option` or `Ordering` boxes its
            // results as the prelude ADTs do: zero-field constructors are
            // static blocks whose headers carry their layout ids (`None`;
            // `LT`/`EQ`/`GT`), `Some` is a fresh `{ value }` block.
            if op.returns_option() {
                globals.push_str(&static_ctor_block(
                    reg.pointer_bytes,
                    &format!("{name}.blk"),
                    reg.shape_desc(&[], TAG_NONE),
                    &[],
                ));
            }
            if op.returns_ordering() {
                for (suf, tag) in [(".lt", 0), (".eq", 1), (".gt", 2)] {
                    globals.push_str(&static_ctor_block(
                        reg.pointer_bytes,
                        &format!("{name}{suf}"),
                        reg.shape_desc(&[], tag),
                        &[],
                    ));
                }
            }
            ret_range = reg.ranges.get(name).copied();
            if let Some((ptys, _)) = prim_sig(*op) {
                noundef_params = ptys
                    .iter()
                    .map(|t| matches!(t, crate::types::Ty::Prim(_) | crate::types::Ty::Bool))
                    .collect();
            }
            let lent = reg.lent.get(name);
            b.ret_drops = borrowed(&|i| prim_consumes(*op, i) || lent.is_some_and(|m| m.get(i).copied().unwrap_or(false)));
            prim_body(&mut b, cx, reg, name, *op);
        }
        UnitBody::Ctor(fields) => {
            let desc = reg.ctor_desc(name)?;
            if *fields == 0
                && reg.emitted_blocks.borrow_mut().insert(format!("{name}.blk"))
            {
                globals.push_str(&static_ctor_block(reg.pointer_bytes, &format!("{name}.blk"), desc, &[]));
            }
            ctor_body(&mut b, reg, name, desc, *fields, reg.plan_of(name));
        }
        UnitBody::Mk(info) => {
            let id = name.strip_suffix(".mk").expect("a dictionary factory's name ends in .mk");
            if reg.static_dict(id) {
                // The factory keeps its symbol and signature for separately
                // compiled consumers; it hands out the one static block.
                let desc = reg.dict_desc(info);
                let mut words = vec![StaticWord::int(info.method_count)];
                words.extend(info.methods.iter().map(|(method, _)| {
                    StaticWord::Ptr(static_payload(&format!("{method}.clo"), reg.closure_desc(method)))
                }));
                globals.push_str(&static_shaped_block(
                    reg.pointer_bytes,
                    &static_dict_name(id),
                    block::KIND_DICT,
                    desc,
                    &words,
                ));
                b.line(&format!("ret i64 ptrtoint (ptr {} to i64)",
                                static_payload(&static_dict_name(id), desc)));
            } else {
                mk_body(&mut b, reg, info)
            }
        }
        UnitBody::Foreign(plan) => {
            b.ret_drops = borrowed(&|_| false);
            foreign_import_body(&mut b, cx, reg, plan)?
        }
        UnitBody::Fn(f) | UnitBody::TailWorker(f) => {
            hints = f.hints;
            lang.def = true;
            lang.pure = f.meta.pure;
            lang.no_alloc = f.meta.contract.no_alloc;
            lang.returns = f.meta.contract.returns;
            lang.checked_params = f.meta.contract.checked_params;
            ret_range = f.meta.contract.result_range;
            noundef_params = f.meta.contract.params.iter().map(|p| p.noundef).collect();
            // A widened unit's own symbol is a one-call wrapper into a
            // word-typed worker, and a primitive-wrapping unit returns the
            // primitive's word, so the registry never narrows either.
            match (&body, reg.tail_width.get(name)) {
                (UnitBody::Fn(_), Some(&width)) => {
                    let params: Vec<String> = (0..arity).map(|i| format!("%p{i}")).collect();
                    let r = b.reg();
                    b.line(&format!(
                        "{r} = call i64 @{}({})",
                        quote(&tail_worker(name)),
                        padded_args(&params, width)
                    ));
                    b.line(&format!("ret i64 {r}"));
                }
                _ => core_body(&mut b, reg, f, arity)?,
            }
        }
    }
    let header = fn_header(name, arity, ret_range, &noundef_params, narrow);
    // The unit's own globals carry its static blocks and closure
    // descriptors, which name the units a body reaches as data rather
    // than by calling them.
    let mut unit_globals = globals;
    unit_globals.push_str(&b.globals);
    let body_text = format!("{}{}", b.allocas, b.code);
    let mut scan = scan_body(&body_text, reg.unit_symbols(), None);
    // A body reaches another unit's static block or closure descriptor as
    // data (`ptrtoint`, an alias) as well as by calling it, so both the
    // code and the globals contribute.
    scan.refs.extend(referenced_symbols(&body_text, reg.unit_symbols()));
    scan.refs.extend(referenced_symbols(&unit_globals, reg.unit_symbols()));
    scan.refs.extend(scan.callees.iter().cloned());
    scan.closures_taken
        .extend(closures_named(&body_text, reg.unit_symbols()));
    scan.closures_taken
        .extend(closures_named(&unit_globals, reg.unit_symbols()));
    // A worker is never a first-class value: closures and the apply
    // stub reach it through the unit's own symbol.
    let worker = matches!(body, UnitBody::TailWorker(_));
    Ok(EmittedUnit {
        name: name.to_string(),
        hints,
        globals: unit_globals,
        header,
        allocas: b.allocas,
        code: b.code,
        stub: if worker {
            String::new()
        } else {
            stub_def(reg, name, arity)
        },
        clo: if worker {
            String::new()
        } else {
            clo_def(reg, name, arity)
        },
        scan,
        lang_facts: lang,
    })
}

fn fn_header(
    name: &str,
    arity: usize,
    ret_range: Option<(i64, i64)>,
    noundef_params: &[bool],
    narrow: Option<&NarrowSig>,
) -> String {
    // A parameter crosses at its proved width
    // (design/narrow-value-representation.md section 4); a unit the
    // registry did not narrow crosses entirely in words.
    let cross = |i: usize| narrow.map_or(Cross::WORD, |s| s.param(i));
    let params: Vec<String> = (0..arity)
        .map(|i| {
            let ty = cross(i).ty();
            let nd = if noundef_params.get(i).copied().unwrap_or(false) {
                " noundef"
            } else {
                ""
            };
            format!("{ty}{nd} %p{i}")
        })
        .collect();
    let ret_cross = narrow.map_or(Cross::WORD, |s| s.ret);
    let ret = match (ret_cross.is_word(), ret_range) {
        // A narrow result states its range in its own type, so `range`
        // rides the narrowed value rather than the word. The bounds must
        // fit that type: `U32`'s exclusive upper bound is 2^32, which an
        // `i32` cannot spell, and the narrowed type itself already
        // implies the whole range, so the attribute is dropped rather
        // than emitted malformed.
        (false, Some((lo, hi))) if lo < hi && range_fits(lo, hi, ret_cross.bits) => {
            format!("range({} {lo}, {hi}) {}", ret_cross.ty(), ret_cross.ty())
        }
        (false, _) => ret_cross.ty().to_string(),
        // The `range` return attribute states the result's value-word
        // range (the plan note's item 3): never the empty range `[a, a)`
        // the verifier rejects.
        (true, Some((lo, hi))) if lo < hi => format!("range(i64 {lo}, {hi}) i64"),
        (true, _) => "i64".to_string(),
    };
    format!("define {ret} @{}({})", quote(name), params.join(", "))
}

/// Whether `[lo, hi)` says something the narrowed type does not already.
///
/// Two ways it does not, and LLVM refuses both:
///
/// * a bound the type cannot spell (`U32`'s exclusive upper bound is
///   2^32, which an `i32` cannot hold), and
/// * a range covering the whole type (`I8`'s `[-128, 128)`), which the
///   verifier reads as "the empty set but limits aren't 0".
///
/// The narrowed type already implies both facts, so dropping the
/// attribute loses nothing.
fn range_fits(lo: i64, hi: i64, bits: u32) -> bool {
    if bits >= 64 {
        return true;
    }
    let (tmin, tmax) = match bits {
        8 => (-128i64, 255i64),
        16 => (-32768, 65535),
        _ => (i32::MIN as i64, u32::MAX as i64),
    };
    if !(tmin..=tmax).contains(&lo) || !(tmin..=tmax).contains(&hi) {
        return false;
    }
    // The whole type: every value it can hold is already in range, so the
    // attribute says nothing. The exclusive upper bound is the type's
    // maximum plus one, which is why it can exceed `tmax` above.
    let exclusive_max = match bits {
        8 => 128i64,
        16 => 32768,
        _ => 2147483648,
    };
    let signed_min = match bits {
        8 => -128i64,
        16 => -32768,
        _ => -2147483648,
    };
    let whole = lo == signed_min && hi == exclusive_max;
    !whole
}

/// The LLVM type a value of `bits` crosses at: the word's own type for
/// 64, the narrower integer otherwise. A 32-bit narrow position is an
/// integer even for `F32`, because the parameter carries the scalar's
/// *bits* across the boundary and the body reads them as it always has
/// (the same convention [`int_ty`] documents for the value word).
fn narrow_llvm_ty(bits: u32) -> &'static str {
    match bits {
        8 => "i8",
        16 => "i16",
        32 => "i32",
        _ => "i64",
    }
}

/// The width and extension one position crosses a call boundary at,
/// resolved against the target from its proved [`Crossing`]. The body
/// on either side works in the 8-byte slot word; only the boundary
/// carries this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cross {
    bits: u32,
    /// Sign-extend back into the slot, rather than zero-extend. The
    /// canonical word of a signed value is sign-extended (`I8`'s `-2`
    /// holds `-2` in the slot, which is what `slt` and the primitives
    /// read), so an `Int` and a signed machine scalar sign-extend; a
    /// pointer, a `Nat` and an unsigned scalar zero-extend, and a float
    /// zero-extends its bits.
    sext: bool,
}

impl Cross {
    const WORD: Cross = Cross { bits: 64, sext: false };

    /// `n` resolved on a target whose pointers are `pointer_bytes` wide:
    /// a `csize` crossing is the slot unless the pointer is narrower.
    fn of(n: Option<crate::core::Crossing>, pointer_bytes: usize) -> Cross {
        use crate::core::Crossing;
        match n {
            Some(Crossing::Bits { bits, signed }) => Cross { bits, sext: signed },
            Some(Crossing::Csize { signed }) if pointer_bytes < 8 => {
                Cross { bits: pointer_bytes as u32 * 8, sext: signed }
            }
            _ => Cross::WORD,
        }
    }

    fn is_word(self) -> bool {
        self.bits >= 64
    }

    fn ty(self) -> &'static str {
        narrow_llvm_ty(self.bits)
    }

    /// The slot word of a value that crossed at this width.
    fn widen(self, b: &mut Builder, v: &str) -> String {
        if self.is_word() {
            return v.to_string();
        }
        let ext = if self.sext { "sext" } else { "zext" };
        let w = b.reg();
        b.line(&format!("{w} = {ext} {} {v} to i64", self.ty()));
        w
    }

    /// The narrowed value of the slot word `v`. Every value of a narrowed
    /// position fits its width, so the truncation drops nothing.
    fn narrow(self, b: &mut Builder, v: &str) -> String {
        if self.is_word() {
            return v.to_string();
        }
        let w = b.reg();
        b.line(&format!("{w} = trunc i64 {v} to {}", self.ty()));
        w
    }
}

/// One unit's narrowed boundary: every parameter's crossing and the
/// result's.
#[derive(Clone, Debug, PartialEq, Eq)]
struct NarrowSig {
    params: Vec<Cross>,
    ret: Cross,
}

impl NarrowSig {
    fn of(c: &crate::core::FnContract, pointer_bytes: usize) -> NarrowSig {
        NarrowSig {
            params: c.params.iter().map(|p| Cross::of(p.narrow, pointer_bytes)).collect(),
            ret: Cross::of(c.result_narrow, pointer_bytes),
        }
    }

    fn is_word(&self) -> bool {
        self.ret.is_word() && self.params.iter().all(|p| p.is_word())
    }

    fn param(&self, i: usize) -> Cross {
        self.params.get(i).copied().unwrap_or(Cross::WORD)
    }

    /// The LLVM prototype this boundary gives a unit: what `musttail`
    /// compares. Two units whose positions extend differently (an `Int`
    /// and a `Nat` at `csize`) still share it.
    fn proto(&self) -> (Vec<&'static str>, &'static str) {
        (self.params.iter().map(|c| c.ty()).collect(), self.ret.ty())
    }
}

/// Call the unit `name` directly with slot words `args` and return its
/// result as a slot word, converting at its narrowed boundary when the
/// registry narrowed it. Every direct call outside the body emitter (a
/// dictionary factory's strict method, a thunk's value, a foreign
/// export's wrapper) goes through here, so none can disagree with the
/// callee's header.
fn call_unit(b: &mut Builder, reg: &Registry, name: &str, args: &[String]) -> String {
    let sig = reg.narrow_sigs.get(name);
    let mut typed = Vec::with_capacity(args.len());
    for (i, a) in args.iter().enumerate() {
        let c = sig.map_or(Cross::WORD, |s| s.param(i));
        let n = c.narrow(b, a);
        typed.push(format!("{} {n}", c.ty()));
    }
    let ret = sig.map_or(Cross::WORD, |s| s.ret);
    let r = b.reg();
    b.line(&format!("{r} = call {} @{}({})", ret.ty(), quote(name), typed.join(", ")));
    ret.widen(b, &r)
}

/// The unit's static closure `@"<name>.clo"`, `{ app; arity; 0 }`, with
/// the unit's closure shape.
fn clo_def(reg: &Registry, name: &str, arity: usize) -> String {
    let app = quote(&format!("{name}.app"));
    static_shaped_block(
        reg.pointer_bytes,
        &format!("{name}.clo"),
        block::KIND_CLOSURE,
        reg.closure_desc(name),
        &[
            StaticWord::Ptr(format!("@{app}")),
            StaticWord::int(arity),
            StaticWord::int(0),
        ],
    )
}

/// Emit the apply stub `@"<name>.app"`.
///
/// A closure's code word points at the stub, which is one entry per
/// function whatever argument count it is handed. It merges the closure's
/// `nf` saved words with the incoming words into a frame of exactly
/// `arity` words, then branches once: a total at or above the arity
/// saturates (call the underlying function, pushing any excess words at
/// the result), a total below it grows (allocate a closure holding the
/// merged frame). The `caml_applyN` shape this replaced needed one entry
/// per supplied count, which is 973 extra functions on the compiler's own
/// module and 7.4% of the toolkit check's CPU for 54 of those entries
/// executing at all (`design/stack-allocation.md` section 7.7).
///
/// The frame is a static `[arity x i64]` allocation, never a dynamic
/// `alloca`: with `"probe-stack"="inline-asm"` LLVM's AArch64 lowering of
/// a dynamic alloca skips its rounding to the stack alignment, leaves `sp`
/// misaligned, and the next stack access faults once a caller inlines the
/// stub. A fixed frame also removes the two dynamic-length `memcpy` calls
/// the merge used to make; the grow path's single copy is from the frame,
/// so it has no length the entry does not already know.
///
/// Ownership (design/reclamation.md section 4): the stub consumes the
/// closure and the arguments. The arguments move on, and so do the saved
/// words once they are copied out: each counted one is dupped, since the
/// closure still holds its own, and the closure is then dropped. A
/// uniquely held closure skips both: its saved words move and its block
/// is released without visiting them (`cinch_free`). An over-applied
/// call's intermediate result is consumed by the `__apply` that receives
/// the rest.
fn stub_def(reg: &Registry, name: &str, arity: usize) -> String {
    let mut b = Builder::new();
    b.pointer_bytes = reg.pointer_bytes;
    b.rc_debug = reg.rc_debug;
    b.reclaim = reg.reclaim;
    let nf = b.load_at("%self", 16);
    let total = b.reg();
    b.line(&format!("{total} = add i64 {nf}, %argc"));
    if arity == 0 {
        // Every application of a nullary closure is saturated, so there is
        // no grow path and no second block to branch to.
        let nb = b.reg();
        b.line(&format!("{nb} = mul i64 {nf}, 8"));
        stub_saturate(&mut b, reg, name, arity, &nf, &nb, &total, "%self");
        return b.finish(&format!(
            "define i64 @{}(ptr %self, i64 %argc, ptr %argv)",
            quote(&format!("{name}.app"))
        ));
    }
    let full = b.reg();
    b.line(&format!("{full} = icmp uge i64 {total}, {arity}"));
    let lpart = b.label();
    let lcall = b.label();
    b.line(&format!("br i1 {full}, label %{lcall}, label %{lpart}"));
    b.start(&lpart);
    stub_grow(&mut b, reg, name, arity, &nf, &total);
    b.start(&lcall);
    let nb = b.reg();
    b.line(&format!("{nb} = mul i64 {nf}, 8"));
    let frame = stub_merge_frame(&mut b, arity, &nf, &nb);
    stub_saturate(&mut b, reg, name, arity, &nf, &nb, &total, &frame);
    b.finish(&format!(
        "define i64 @{}(ptr %self, i64 %argc, ptr %argv)",
        quote(&format!("{name}.app"))
    ))
}

/// Fewer arguments than the arity: build a closure whose payload is the
/// `nf` saved words followed by the `argc` incoming ones, and whose code
/// word is this same stub, so the next application enters it again. No
/// frame is involved: `total < arity` words are copied straight from the
/// two sources, which is the only place either copy length is dynamic.
fn stub_grow(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    arity: usize,
    nf: &str,
    total: &str,
) {
    let words = b.reg();
    b.line(&format!("{words} = add i64 {total}, 3"));
    let bytes = b.reg();
    b.line(&format!("{bytes} = mul i64 {words}, 8"));
    let new = b.reg();
    b.line(&shaped_alloc_call(
        &new,
        &bytes,
        block::KIND_CLOSURE,
        reg.closure_desc(name),
    ));
    let cw = b.load_at("%self", 0);
    b.store_at(&new, 0, &cw);
    b.store_at(&new, 8, &arity.to_string());
    b.store_at(&new, 16, total);
    let nd = b.gep(&new, 24);
    let saved = b.gep("%self", 24);
    let nb = b.reg();
    b.line(&format!("{nb} = mul i64 {nf}, 8"));
    // `memcpy` takes a `size_t`, so the byte count crosses at the target's
    // pointer width: on a 32-bit target the word is narrowed here, where
    // the count is a byte length of a payload that fits one.
    let sz = b.size_ty();
    let nbs = b.to_size(nb.clone());
    b.line(&format!("call ptr @memcpy(ptr {nd}, ptr {saved}, {sz} {nbs})"));
    let na = b.reg();
    b.line(&format!("{na} = mul i64 %argc, 8"));
    let nas = b.to_size(na.clone());
    let nd2 = b.gep_dyn(&nd, &nb);
    b.line(&format!("call ptr @memcpy(ptr {nd2}, ptr %argv, {sz} {nas})"));
    stub_take_saved(b, reg, name, arity, &nd, nf);
    let word = b.reg();
    b.line(&format!("{word} = ptrtoint ptr {new} to i64"));
    b.line(&format!("ret i64 {word}"));
}

/// Copy the first `arity` merged words (the closure's `nf` saved words
/// followed by `argv`) into a static frame. Each slot selects between the
/// two sources with a constant index; `argv` is indexed through a
/// `getelementptr i8` that is deliberately not `inbounds`, because a slot
/// that selects a saved word still computes, but never loads, the
/// matching `argv` slot.
fn stub_merge_frame(b: &mut Builder, arity: usize, nf: &str, nb: &str) -> String {
    let frame = b.alloca_words(arity.max(1));
    for i in 0..arity {
        let saved = b.gep("%self", 24 + i * 8);
        let off = b.reg();
        b.line(&format!("{off} = sub i64 {}, {nb}", i * 8));
        let arg = b.reg();
        b.line(&format!("{arg} = getelementptr i8, ptr %argv, i64 {off}"));
        let has = b.reg();
        b.line(&format!("{has} = icmp ult i64 {i}, {nf}"));
        let src = b.reg();
        b.line(&format!("{src} = select i1 {has}, ptr {saved}, ptr {arg}"));
        let word = b.reg();
        b.line(&format!("{word} = load i64, ptr {src}"));
        let slot = b.gep(&frame, i * 8);
        b.line(&format!("store i64 {word}, ptr {slot}"));
    }
    frame
}

/// The saturated call: `frame` holds the merged words and `total` is
/// `nf + supplied`. The excess words continue at `argv + (arity - nf)`,
/// which is inside `argv` whenever there is an excess.
fn stub_saturate(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    arity: usize,
    nf: &str,
    nb: &str,
    total: &str,
    frame: &str,
) {
    stub_take_saved(b, reg, name, arity, frame, nf);
    // The stub's frame is a word array, and the callee may have a narrowed
    // signature (design/narrow-value-representation.md section 4). The
    // stub is the one caller that does not know the argument types from
    // the source, so it converts here: each word narrows to the callee's
    // proved width, which is exactly the marshal the design calls for at
    // the indirect boundary.
    let narrow = reg.narrow_sigs.get(name);
    let mut args = Vec::with_capacity(arity);
    for i in 0..arity {
        let w = b.load_at(frame, i * 8);
        let c = narrow.map_or(Cross::WORD, |s| s.param(i));
        let n = c.narrow(b, &w);
        args.push(format!("{} {n}", c.ty()));
    }
    let ret = narrow.map_or(Cross::WORD, |s| s.ret);
    let ret_ty = ret.ty();
    let r = b.reg();
    if arity == 0 {
        b.line(&format!("{r} = call {ret_ty} @{}()", quote(name)));
    } else {
        b.line(&format!(
            "{r} = call {ret_ty} @{}({})",
            quote(name),
            args.join(", ")
        ));
    }
    // The frame and the excess-word path work in words, so a narrowed
    // result widens straight back.
    let r = ret.widen(b, &r);
    let ex = b.reg();
    b.line(&format!("{ex} = sub i64 {total}, {arity}"));
    let hx = b.reg();
    b.line(&format!("{hx} = icmp ne i64 {ex}, 0"));
    let lover = b.label();
    let ldone = b.label();
    b.line(&format!("br i1 {hx}, label %{lover}, label %{ldone}"));
    b.start(&ldone);
    b.line(&format!("ret i64 {r}"));

    // Over-applied: push the rest of the argument vector at the result.
    b.start(&lover);
    let off = b.reg();
    b.line(&format!("{off} = sub i64 {}, {nb}", arity * 8));
    let tail = b.reg();
    b.line(&format!("{tail} = getelementptr i8, ptr %argv, i64 {off}"));
    let res = b.reg();
    b.line(&format!(
        "{res} = call i64 @__apply(i64 {r}, i64 {ex}, ptr {tail})"
    ));
    b.line(&format!("ret i64 {res}"));
}

/// The release half of an apply stub: `copy` holds the `nf` saved words
/// just copied out of `%self`, which now owns nothing the copy needs.
fn stub_take_saved(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    arity: usize,
    copy: &str,
    nf: &str,
) {
    if !b.reclaim {
        return;
    }
    // A zero-argument application can enter a closure with every worker
    // parameter already saved. Move or duplicate all of those words.
    let classes: Vec<Class> = (0..arity)
        .map(|i| {
            reg.unit_params
                .get(name)
                .and_then(|cs| cs.get(i))
                .copied()
                .unwrap_or(Class::Raw)
        })
        .collect();
    let selfw = b.reg();
    b.line(&format!("{selfw} = ptrtoint ptr %self to i64"));
    let dup_saved = |b: &mut Builder| {
        for (i, c) in classes.iter().enumerate() {
            if !counted_class(*c) {
                continue;
            }
            let has = b.reg();
            b.line(&format!("{has} = icmp ult i64 {i}, {nf}"));
            let ldup = b.label();
            let lnext = b.label();
            b.line(&format!("br i1 {has}, label %{ldup}, label %{lnext}"));
            b.start(&ldup);
            let w = b.load_at(copy, i * 8);
            rc_op(b, reg.rc_debug, false, &w, *c);
            b.line(&format!("br label %{lnext}"));
            b.start(&lnext);
        }
    };
    if reg.rc_debug {
        dup_saved(b);
        rc_op(b, true, true, &selfw, Class::Obj);
        return;
    }
    let hp = b.gep("%self", 0);
    let hp8 = b.reg();
    b.line(&format!(
        "{hp8} = getelementptr inbounds i8, ptr {hp}, i64 -8"
    ));
    let h = header_load(b, &hp8);
    let top = b.reg();
    b.line(&format!("{top} = shl i64 {h}, {RC_TOP_SHIFT}"));
    let unique = b.reg();
    b.line(&format!(
        "{unique} = icmp eq i64 {top}, {}",
        1i64 << RC_TOP_SHIFT
    ));
    let lmove = b.label();
    let lshare = b.label();
    let ldone = b.label();
    b.line(&format!("br i1 {unique}, label %{lmove}, label %{lshare}"));
    b.start(&lmove);
    b.line(&format!("call void @cinch_free(i64 {selfw})"));
    b.line(&format!("br label %{ldone}"));
    b.start(&lshare);
    dup_saved(b);
    // The count is not one here: above one it drops in place; a closure
    // shared with other tasks drops atomically (and may be released, its
    // saved words already dupped for the call); an immortal (static)
    // closure keeps its count.
    let plain = b.reg();
    b.line(&format!("{plain} = icmp sgt i64 {top}, 0"));
    let (ldec, lother, lmt) = (b.label(), b.label(), b.label());
    b.line(&format!("br i1 {plain}, label %{ldec}, label %{lother}"));
    b.start(&ldec);
    let h1 = b.reg();
    b.line(&format!("{h1} = sub i64 {h}, {}", 1u64 << block::RC_SHIFT));
    b.line(&format!("store i64 {h1}, ptr {hp8}"));
    b.line(&format!("br label %{ldone}"));
    b.start(&lother);
    let shared = b.reg();
    b.line(&format!("{shared} = icmp ne i64 {top}, 0"));
    b.line(&format!("br i1 {shared}, label %{lmt}, label %{ldone}"));
    b.start(&lmt);
    b.line(&format!("call void @cinch_drop_mt(i64 {selfw})"));
    b.line(&format!("br label %{ldone}"));
    b.start(&ldone);
}

fn counted_class(c: Class) -> bool {
    matches!(c, Class::Obj | Class::TObj)
}

/// Memo cell machinery (laziness and reclamation specification). A cell is
/// `{ i64 state; i64 owned; i64 value }` with state 0 = unevaluated,
/// 1 = evaluating, 2 = evaluated. Demanding an evaluating cell is a black
/// hole: the defined `BlackHole` runtime fault (exit 70), never undefined
/// behavior.
///
/// Evaluation moves the thunk out of the cell (its slot becomes 0, which
/// the cell's pointer map skips) and hands it to `__apply`, which
/// consumes it: the thunk and what only it captured are released once it
/// has run (design/reclamation.md section 6). The scanned slot then owns
/// the cached heap pointer, if any. The result slot is always unscanned
/// because a raw value word can look like a pointer. `Core::Demand`
/// duplicates the borrowed result when its class is counted.
const MEMO_HELPERS: &str = r#"
declare void @exit(i32) noreturn

define internal void @"__cinch_blackhole"() noreturn {
entry:
  call void @exit(i32 70)
  unreachable
}

define internal i64 @"__cinch_demand"(i64 %cell, i1 %counted) {
entry:
  %p = inttoptr i64 %cell to ptr
  %sp = getelementptr inbounds i8, ptr %p, i64 0
  %state = load i64, ptr %sp
  switch i64 %state, label %bad [ i64 0, label %eval i64 2, label %done ]
eval:
  %tp = getelementptr inbounds i8, ptr %p, i64 8
  %thunk = load i64, ptr %tp
  %uninitialized = icmp eq i64 %thunk, 0
  br i1 %uninitialized, label %bad, label %force
force:
  store i64 1, ptr %sp
  store i64 0, ptr %tp
  %argv = alloca i64, i64 1
  store i64 0, ptr %argv
  %v = call i64 @__apply(i64 %thunk, i64 1, ptr %argv)
  %tag = and i64 %v, 1
  %even = icmp eq i64 %tag, 0
  %owns = and i1 %counted, %even
  %owned = select i1 %owns, i64 %v, i64 0
  store i64 %owned, ptr %tp
  %vp = getelementptr inbounds i8, ptr %p, i64 16
  store i64 %v, ptr %vp
  store i64 2, ptr %sp
  ret i64 %v
done:
  %vp2 = getelementptr inbounds i8, ptr %p, i64 16
  %v2 = load i64, ptr %vp2
  ret i64 %v2
bad:
  call void @"__cinch_blackhole"()
  unreachable
}

"#;

const APPLY_DEF: &str = "\
define i64 @__apply(i64 %f, i64 %argc, ptr %argv) {
entry:
  %self = inttoptr i64 %f to ptr
  %cp = getelementptr inbounds i8, ptr %self, i64 0
  %cw = load i64, ptr %cp
  %code = inttoptr i64 %cw to ptr
  %r = call i64 %code(ptr %self, i64 %argc, ptr %argv)
  ret i64 %r
}

";

/// Text accumulator for one function body: allocas are hoisted into the
/// entry block, all register and label names come from one counter.
struct Builder {
    allocas: String,
    code: String,
    /// Module-level globals (private constants) this unit needs, emitted
    /// before the function definition.
    globals: String,
    tmp: usize,
    blk: usize,
    dec: usize,
    /// Words to drop on every `ret`: a primitive or foreign wrapper's
    /// parameters that the runtime only borrows (design/reclamation.md
    /// section 4). Such a body returns from many places; each releases
    /// them after the extern call and before returning.
    ret_drops: Vec<(String, Class)>,
    /// The target's pointer width in bytes (`Registry::pointer_bytes`): a
    /// static initializer holding a pointer is spelled at this width.
    pointer_bytes: usize,
    rc_debug: bool,
    /// Ownership operations are emitted at all (the program reclaims).
    reclaim: bool,
    /// The current block already has a terminator (a `br`, `ret`,
    /// `switch` or `unreachable`). A structured node whose value is
    /// delivered elsewhere (a `Jump` branches to its join's block) leaves
    /// the block terminated, and the instructions that would have stored
    /// its value are dead: they are skipped rather than emitted after the
    /// terminator, which is not well-formed IR.
    terminated: bool,
}

impl Builder {
    fn new() -> Builder {
        Builder {
            allocas: String::new(),
            code: String::new(),
            globals: String::new(),
            tmp: 0,
            blk: 0,
            dec: 0,
            ret_drops: Vec::new(),
            // The build host's pointer is the default; an emitter that
            // knows the target overrides it beside `rc_debug`.
            pointer_bytes: std::mem::size_of::<usize>(),
            rc_debug: false,
            reclaim: false,
            terminated: false,
        }
    }

    /// The LLVM type of a `size_t`-width value on this target
    /// ([`LowerCx::size_ty`], which the emitter copies here).
    fn size_ty(&self) -> &'static str {
        if self.pointer_bytes == 4 { "i32" } else { "i64" }
    }

    /// `v`, a word-width value, narrowed to the `size_t` width when the
    /// target's is narrower (the same register on a 64-bit target).
    fn to_size(&mut self, v: String) -> String {
        if self.pointer_bytes == 4 {
            let w = self.reg();
            self.line(&format!("{w} = trunc i64 {v} to i32"));
            w
        } else {
            v
        }
    }

    fn global(&mut self, def: String) {
        self.globals.push_str(&def);
        self.globals.push('\n');
    }

    /// A fresh private constant name for `unit` (unit names spell out
    /// `Module.name`, so `Module.name.dec<N>` cannot collide with another
    /// unit's globals).
    fn dec_name(&mut self, unit: &str) -> String {
        let n = self.dec;
        self.dec += 1;
        format!("{unit}.dec{n}")
    }

    fn reg(&mut self) -> String {
        let n = self.tmp;
        self.tmp += 1;
        format!("%t{n}")
    }

    fn label(&mut self) -> String {
        let n = self.blk;
        self.blk += 1;
        format!("bb{n}")
    }

    fn line(&mut self, s: &str) {
        if !self.ret_drops.is_empty() && s.starts_with("ret ") {
            let drops = std::mem::take(&mut self.ret_drops);
            for (w, c) in &drops {
                rc_op(self, self.rc_debug, true, w, *c);
            }
            self.ret_drops = drops;
        }
        let trimmed = s.trim_start();
        self.terminated = trimmed.starts_with("br ")
            || trimmed.starts_with("ret ")
            || trimmed.starts_with("switch ")
            || trimmed.starts_with("unreachable");
        self.code.push_str("  ");
        self.code.push_str(s);
        self.code.push('\n');
    }

    fn start(&mut self, label: &str) {
        self.terminated = false;
        self.code.push_str(label);
        self.code.push_str(":\n");
    }

    fn alloca_i64(&mut self) -> String {
        let r = self.reg();
        self.allocas.push_str(&format!("  {r} = alloca i64\n"));
        r
    }

    fn alloca_words(&mut self, n: usize) -> String {
        let r = self.reg();
        self.allocas
            .push_str(&format!("  {r} = alloca [{n} x i64]\n"));
        r
    }

    /// A frame block's slot: `n` words (header and payload) rounded up to an
    /// even count and 16-byte aligned, so the stack depth the slot adds is
    /// a multiple of 16 bytes and the payload sits at an 8-mod-16 offset
    /// exactly as a heap block's does.
    fn alloca_frame(&mut self, n: usize) -> String {
        let n = n + (n & 1);
        let r = self.reg();
        self.allocas
            .push_str(&format!("  {r} = alloca [{n} x i64], align 16\n"));
        r
    }

    /// A byte buffer alloca (call-scoped C-layout copies at the foreign
    /// boundary), aligned to the recorded layout's alignment so the C
    /// side reads the copy at its natural alignment.
    fn alloca_bytes(&mut self, n: u64, align: u64) -> String {
        let r = self.reg();
        self.allocas
            .push_str(&format!("  {r} = alloca [{n} x i8], align {align}\n"));
        r
    }

    fn gep(&mut self, base: &str, off: usize) -> String {
        self.gep_dyn(base, &off.to_string())
    }

    fn gep_dyn(&mut self, base: &str, off: &str) -> String {
        let r = self.reg();
        self.line(&format!(
            "{r} = getelementptr inbounds i8, ptr {base}, i64 {off}"
        ));
        r
    }

    fn store_at(&mut self, base: &str, off: usize, val: &str) {
        let p = self.gep(base, off);
        self.line(&format!("store i64 {val}, ptr {p}"));
    }

    fn load_at(&mut self, base: &str, off: usize) -> String {
        let p = self.gep(base, off);
        let r = self.reg();
        self.line(&format!("{r} = load i64, ptr {p}"));
        r
    }

    fn finish(self, header: &str) -> String {
        format!(
            "{}{header} {{\nentry:\n{}{}}}\n\n",
            self.globals, self.allocas, self.code
        )
    }
}

// ------------------------------------------------------------ primitive code generation
//
// Every primitive value is one canonical `i64` word (core.rs): signed
// machine integers are sign-extended to 64 bits, unsigned zero-extended,
// `F32` holds the binary32 bits zero-extended in the low 32, `F64` the
// binary64 bits, `Char` a code point. Every operation canonicalizes its
// result to the declared type's word form; this is where the "sub-word
// types are value-level" rule of the primitive types specification
// (section 9) is enforced.

/// The LLVM integer type holding a scalar's value bits.
fn int_ty(s: Scalar) -> &'static str {
    match s.width() {
        8 => "i8",
        16 => "i16",
        32 => "i32",
        _ => "i64",
    }
}

fn float_ty(s: Scalar) -> &'static str {
    match s {
        Scalar::F32 => "float",
        _ => "double",
    }
}

/// A scalar's bounds as `i64` constants (`maxBoundU64` is the all-ones
/// bit pattern).
fn min_word(s: Scalar) -> i64 {
    s.min_bound_word()
}

fn max_word(s: Scalar) -> i64 {
    s.max_bound_word()
}

/// Zero-extend a value-bit operand to the canonical word (no-op at the
/// word width: `zext i64 to i64` is not a cast).
fn ext_z(b: &mut Builder, s: Scalar, v: &str) -> String {
    if s.width() == 64 {
        return v.to_string();
    }
    let r = b.reg();
    b.line(&format!("{r} = zext {} {v} to i64", int_ty(s)));
    r
}

/// Canonicalize `v` (an `i64` operand name) to scalar `s` by extending the
/// low `width` bits.
fn canon(b: &mut Builder, s: Scalar, v: &str) -> String {
    let w = s.width();
    if w == 64 {
        return v.to_string();
    }
    let sh = b.reg();
    b.line(&format!("{sh} = shl i64 {v}, {}", 64 - w));
    let r = b.reg();
    let inst = if s.is_signed() { "ashr" } else { "lshr" };
    b.line(&format!("{r} = {inst} i64 {sh}, {}", 64 - w));
    r
}

/// Truncate a canonical word to the scalar's value bits.
fn trunc_n(b: &mut Builder, s: Scalar, v: &str) -> String {
    if s.width() == 64 {
        return v.to_string();
    }
    let r = b.reg();
    b.line(&format!("{r} = trunc i64 {v} to {}", int_ty(s)));
    r
}

/// Extend a value-bit operand to the scalar's canonical word.
fn ext_n(b: &mut Builder, s: Scalar, v: &str) -> String {
    if s.width() == 64 {
        return v.to_string();
    }
    let r = b.reg();
    let inst = if s.is_signed() { "sext" } else { "zext" };
    b.line(&format!("{r} = {inst} {} {v} to i64", int_ty(s)));
    r
}

/// The floating-point value held in a canonical word.
fn word_to_float(b: &mut Builder, s: Scalar, v: &str) -> String {
    if s == Scalar::F32 {
        let i = b.reg();
        b.line(&format!("{i} = trunc i64 {v} to i32"));
        let f = b.reg();
        b.line(&format!("{f} = bitcast i32 {i} to float"));
        f
    } else {
        let f = b.reg();
        b.line(&format!("{f} = bitcast i64 {v} to double"));
        f
    }
}

/// The canonical word of a floating-point value.
fn float_to_word(b: &mut Builder, s: Scalar, f: &str) -> String {
    if s == Scalar::F32 {
        let i = b.reg();
        b.line(&format!("{i} = bitcast float {f} to i32"));
        let w = b.reg();
        b.line(&format!("{w} = zext i32 {i} to i64"));
        w
    } else {
        let w = b.reg();
        b.line(&format!("{w} = bitcast double {f} to i64"));
        w
    }
}

/// A float constant: LLVM's hexadecimal float constants carry the
/// binary64 bit pattern of the value, which must be exactly representable
/// in the constant's type.
fn f64c(v: f64) -> String {
    format!("0x{:016X}", v.to_bits())
}

/// The constant for comparisons against scalar `s`'s float type.
fn fcst(s: Scalar, v: f64) -> String {
    if s == Scalar::F32 {
        f64c(v as f32 as f64)
    } else {
        f64c(v)
    }
}

/// Freeze one untrusted value read at a boundary (foreign results,
/// pointers and buffer data; the plan note's poison containment): the
/// value is defined from here on, so a poison bit pattern from foreign
/// input cannot leak into otherwise-safe code. `bits` is the loaded
/// width (`freeze` spells its operand's type).
fn freeze_at(b: &mut Builder, v: &str, bits: u32) -> String {
    let r = b.reg();
    b.line(&format!("{r} = freeze i{bits} {v}"));
    r
}

/// Freeze one untrusted canonical word (boundary words are `i64`).
fn freeze_word(b: &mut Builder, v: &str) -> String {
    freeze_at(b, v, 64)
}

/// The `!range` metadata for one canonical word range `[lo, hi)` as
/// `i64` words (the `!range`/`range` facts of the plan note).
fn range_meta_i64(reg: &Registry, lo: i64, hi: i64) -> String {
    reg.meta_node(format!("!{{i64 {lo}, i64 {hi}}}"))
}

/// The `!range` metadata of one typed load: the field's value-word
/// range (`FieldAccess::range`) spelled in the loaded width, when the
/// width can state it. A full-width span cannot be spelled (`[a, a)`
/// is the empty range LLVM rejects), so such a load stays bare: the
/// type's own claim is already the load's type (the plan note's item 3:
/// a fact the analysis cannot state is never guessed).
fn load_range_meta(reg: &Registry, bits: u32, range: Option<(i64, i64)>) -> Option<String> {
    if bits == 0 || bits > 64 {
        return None;
    }
    let (lo, hi) = range?;
    let ty = format!("i{bits}");
    let width = 1i128 << bits;
    let span = hi as i128 - lo as i128;
    let spellable = lo as i128 >= -(1i128 << (bits - 1)) && hi as i128 <= width - 1;
    if span <= 0 || span >= width || !spellable {
        return None;
    }
    Some(reg.meta_node(format!("!{{{ty} {lo}, {ty} {hi}}}")))
}

/// The defined fault path (exit 71 through `cinch_panic`) when `bad` is
/// true: a primitive's contract violation never produces a value.
fn fault_when(b: &mut Builder, unit: &str, bad: &str, msg: &str) {
    let name = b.dec_name(&format!("{unit}.fault"));
    let n = msg.len();
    b.global(format!(
        "@{} = private constant [{n} x i8] c\"{msg}\"",
        quote(&name)
    ));
    let lbad = b.label();
    let lok = b.label();
    b.line(&format!("br i1 {bad}, label %{lbad}, label %{lok}"));
    b.start(&lbad);
    let p = b.reg();
    b.line(&format!(
        "{p} = getelementptr inbounds [{n} x i8], ptr @{g}, i64 0, i64 0",
        g = quote(&name)
    ));
    b.line(&format!("call void @cinch_panic(ptr {p}, i64 {n})"));
    b.line("unreachable");
    b.start(&lok);
}

/// Return `None` when `none_if` is true, else `Some v` (the prelude
/// `Option` ADT, an anonymous shape of its tags whose field has the class
/// `op`'s signature gives it).
fn ret_option(
    b: &mut Builder,
    reg: &Registry,
    op: PrimOp,
    name: &str,
    none_if: &str,
    val: impl FnOnce(&mut Builder) -> String,
) {
    let lnone = b.label();
    let lsome = b.label();
    b.line(&format!("br i1 {none_if}, label %{lnone}, label %{lsome}"));
    b.start(&lnone);
    ensure_static_ctor(b, reg, name, reg.shape_desc(&[], TAG_NONE));
    let w = b.reg();
    b.line(&format!(
        "{w} = ptrtoint ptr {blk} to i64",
        blk = static_payload(&format!("{name}.blk"), reg.shape_desc(&[], TAG_NONE))
    ));
    b.line(&format!("ret i64 {w}"));
    b.start(&lsome);
    let v = val(b);
    let field = prim_payload_classes(op)
        .first()
        .copied()
        .unwrap_or(Class::Raw);
    let w = ctor_block_word(b, reg.shape_desc(&[field], TAG_SOME), &[v]);
    b.line(&format!("ret i64 {w}"));
}

fn ret(b: &mut Builder, v: &str) {
    b.line(&format!("ret i64 {v}"));
}

/// Return the prelude `Result` a boundary word stands for: a NEGATIVE
/// word is `Err` over its POSITIVE named error code, a non-negative one
/// is `Ok` over the value the caller maps from it (the prelude `Result`
/// tags, as anonymous shapes whose fields have the classes `op`'s
/// signature gives them, the same convention `crate::task`'s outcome ABI
/// documents). The platform's net and piped-child externs answer one word
/// for both outcomes (`crate::platform`'s return-code convention), so
/// their units present that word as this `Result`: a socket read's
/// success value is a fresh `Bytes` buffer, which a single raw word at
/// the language level could not tell apart from a failure code.
fn ret_result(
    b: &mut Builder,
    reg: &Registry,
    op: PrimOp,
    raw: &str,
    ok: impl FnOnce(&mut Builder) -> String,
) {
    let classes = prim_payload_classes(op);
    let class = |i: usize| classes.get(i).copied().unwrap_or(Class::Raw);
    let neg = b.reg();
    b.line(&format!("{neg} = icmp slt i64 {raw}, 0"));
    let lerr = b.label();
    let lok = b.label();
    b.line(&format!("br i1 {neg}, label %{lerr}, label %{lok}"));
    b.start(&lerr);
    let code = b.reg();
    b.line(&format!("{code} = sub i64 0, {raw}"));
    let ew = ctor_block_word(b, reg.shape_desc(&[class(0)], TAG_ERR), &[code]);
    b.line(&format!("ret i64 {ew}"));
    b.start(&lok);
    let v = ok(b);
    let ow = ctor_block_word(b, reg.shape_desc(&[class(1)], TAG_OK), &[v]);
    b.line(&format!("ret i64 {ow}"));
}

/// `llvm.<s|u><op>.with.overflow.iN` for the arithmetic families.
fn overflow_intrinsic(signed: bool, op: ArithOp, s: Scalar) -> String {
    let sign = if signed { "s" } else { "u" };
    let o = match op {
        ArithOp::Add => "add",
        ArithOp::Sub => "sub",
        ArithOp::Mul => "mul",
    };
    format!("llvm.{sign}{o}.with.overflow.{}", int_ty(s))
}

/// Checked and saturating integer arithmetic (primitive types
/// specification, section 4) through the overflow intrinsics.
fn arith_overflow(
    b: &mut Builder,
    reg: &Registry,
    prim: PrimOp,
    name: &str,
    s: Scalar,
    op: ArithOp,
    mode: ArithMode,
) {
    let it = int_ty(s);
    let a = trunc_n(b, s, "%p0");
    let c = trunc_n(b, s, "%p1");
    let res = b.reg();
    b.line(&format!(
        "{res} = call {{ {it}, i1 }} @{}({it} {a}, {it} {c})",
        overflow_intrinsic(s.is_signed(), op, s)
    ));
    let val = b.reg();
    b.line(&format!("{val} = extractvalue {{ {it}, i1 }} {res}, 0"));
    let flag = b.reg();
    b.line(&format!("{flag} = extractvalue {{ {it}, i1 }} {res}, 1"));
    match mode {
        ArithMode::Wrapping => unreachable!("wrapping arithmetic does not use intrinsics"),
        ArithMode::Checked => {
            ret_option(b, reg, prim, name, &flag, |b| {
                let w = ext_n(b, s, &val);
                canon(b, s, &w)
            });
        }
        ArithMode::Saturating => {
            // The overflow direction is decided by the operands' signs:
            // add/sub overflow the way the first operand points, mul the
            // way the product points.
            let hi = match op {
                ArithOp::Add | ArithOp::Sub => {
                    if s.is_signed() {
                        let r = b.reg();
                        b.line(&format!("{r} = icmp sge i64 %p0, 0"));
                        r
                    } else {
                        // Unsigned add and mul overflow upward; sub
                        // underflows to zero.
                        if op == ArithOp::Sub {
                            "false".to_string()
                        } else {
                            "true".to_string()
                        }
                    }
                }
                ArithOp::Mul => {
                    if s.is_signed() {
                        let sa = b.reg();
                        b.line(&format!("{sa} = icmp slt i64 %p0, 0"));
                        let sb = b.reg();
                        b.line(&format!("{sb} = icmp slt i64 %p1, 0"));
                        let r = b.reg();
                        b.line(&format!("{r} = icmp eq i1 {sa}, {sb}"));
                        r
                    } else {
                        "true".to_string()
                    }
                }
            };
            let bound = b.reg();
            b.line(&format!(
                "{bound} = select i1 {hi}, i64 {}, i64 {}",
                max_word(s),
                min_word(s)
            ));
            let w = ext_n(b, s, &val);
            let c = canon(b, s, &w);
            let out = b.reg();
            b.line(&format!("{out} = select i1 {flag}, i64 {bound}, i64 {c}"));
            ret(b, &out);
        }
    }
}

fn rt_call(b: &mut Builder, f: &str, args: &[String]) -> String {
    let r = b.reg();
    b.line(&format!("{r} = call i64 @{f}({})", args.join(", ")));
    r
}

/// The count field shifted to the top of the word: the count's sign is
/// the word's, and a count of `n` reads as `n << RC_TOP_SHIFT`, so one
/// signed compare separates the plain, atomic and immortal cases.
const RC_TOP_SHIFT: u32 = 64 - (block::RC_SHIFT + block::RC_BITS);

/// `RC_FRAME`'s count as the top-shifted word the inline helpers compare
/// against: the frame block's marker in the same form as `0` (immortal) and
/// `1 << RC_TOP_SHIFT` (a count of one).
const RC_FRAME_TOP: i64 = block::RC_FRAME << RC_TOP_SHIFT;

/// `RC_FRAME_RELEASED`'s count in the same form ([`RC_FRAME_TOP`]): the
/// lowest top-shifted word a frame block has, so one signed compare
/// (`top < this`) separates every real count from both markers.
const RC_FRAME_RELEASED_TOP: i64 = block::RC_FRAME_RELEASED << RC_TOP_SHIFT;

/// The header bits above the count field, kept when the released marker
/// replaces it.
const HEADER_RC_MASK: u64 = !((1u64 << block::RC_BITS) - 1);

/// Load the header word at `hp` into a fresh register. The load is
/// atomic (`monotonic`, an ordinary load on every target, measured free
/// against a plain one): a block shared with other tasks has its count
/// changed by their atomic read-modify-writes meanwhile, and a plain load
/// racing with those reads undef in LLVM's memory model. Stores stay
/// plain: only a positive count is written in place, and no other thread
/// can reach that block.
fn header_load(b: &mut Builder, hp: &str) -> String {
    let h = b.reg();
    b.line(&format!(
        "{h} = load atomic i64, ptr {hp} monotonic, align 8"
    ));
    h
}

/// One reference count operation on the word `w` of class `class`
/// (design/reclamation.md sections 3, 4 and 8). `Raw` words are never
/// counted. A counted word's operation is a call to its module's private
/// helper ([`rc_helpers`], the inline fast path of [`rc_inline`]): one
/// line per operation instead of a dozen, which keeps the emitted IR in
/// proportion to the program. An optimizing link (`--opt-level`) inlines
/// the helpers back. Under
/// [`TargetProfile::rc_debug`](crate::core::TargetProfile::rc_debug) both
/// call the out-of-line `cinch_dup`/`cinch_drop` instead, so that
/// `CINCH_RC_DEBUG=1`'s header checks see every operation.
fn rc_op(b: &mut Builder, debug: bool, drop: bool, w: &str, class: Class) {
    if !b.reclaim || !matches!(class, Class::Obj | Class::TObj) {
        return;
    }
    if debug {
        let f = if drop { "cinch_drop" } else { "cinch_dup" };
        b.line(&format!("call void @{f}(i64 {w})"));
        return;
    }
    b.line(&format!("call void @{}(i64 {w})", rc_helper_name(drop, class)));
}

/// The module-private helper performing one count operation of `class`.
fn rc_helper_name(drop: bool, class: Class) -> &'static str {
    match (drop, class) {
        (false, Class::TObj) => "cinch.rc.dup.tobj",
        (true, Class::TObj) => "cinch.rc.drop.tobj",
        (false, _) => "cinch.rc.dup.obj",
        (true, _) => "cinch.rc.drop.obj",
    }
}

/// The definitions of the four count helpers [`rc_op`] calls, private to
/// each emitted module. `frame` adds the arms a frame block takes: a dup
/// of one writes nothing, and a drop of one releases its payload once
/// (`cinch_frame_release`) instead of counting. A module that neither
/// holds nor receives a frame block keeps the exact helper it always had,
/// which is what leaves `--no-stack-alloc` byte-identical.
fn rc_helpers(frame: bool, units: &str) -> String {
    let mut out = String::new();
    for class in [Class::Obj, Class::TObj] {
        for drop in [false, true] {
            // A helper no assembled body calls is not emitted: the
            // definition would be the only occurrence of its name.
            let name = rc_helper_name(drop, class);
            if !units.contains(&format!("@{name}(")) {
                continue;
            }
            let mut b = Builder::new();
            b.reclaim = true;
            rc_inline(&mut b, drop, "%w", class, frame);
            b.line("ret void");
            out.push_str(&format!("define internal void @{}(i64 %w) {{\n{}}}\n", name, b.code));
        }
    }
    // A drop specialisation's test (`Core::Release`): a nonzero word whose
    // block's thread-local count is exactly one.
    if !units.contains(&format!("@{RC_UNIQUE}(")) {
        return out;
    }
    let mut b = Builder::new();
    let (lblock, lno) = (b.label(), b.label());
    b.line("%z = icmp eq i64 %w, 0");
    b.line(&format!("br i1 %z, label %{lno}, label %{lblock}"));
    b.start(&lblock);
    b.line("%p = inttoptr i64 %w to ptr");
    b.line("%hp = getelementptr inbounds i8, ptr %p, i64 -8");
    let h = header_load(&mut b, "%hp");
    b.line(&format!("%top = shl i64 {h}, {RC_TOP_SHIFT}"));
    b.line(&format!("%u = icmp eq i64 %top, {}", 1i64 << RC_TOP_SHIFT));
    b.line("ret i1 %u");
    b.start(&lno);
    b.line("ret i1 false");
    out.push_str(&format!("define internal i1 @{RC_UNIQUE}(i64 %w) {{\n{}}}\n", b.code));
    out
}

/// The helper [`rc_helpers`] defines for a drop specialisation's test.
const RC_UNIQUE: &str = "cinch.rc.unique";

/// The fast path of one count operation. An `Obj` word may be 0 (the
/// `Copy` marker dictionary is the unit word), and a `TObj` word tests its
/// low bit first: an immediate owns no block. A block word then reads its
/// header at -8, whose low `RC_BITS` are the signed count, and shifts the
/// count to the top of the word ([`RC_TOP_SHIFT`]): positive is a count
/// only this thread changes, updated in place; negative is a block shared
/// with other tasks, whose count only `cinch_dup_mt`/`cinch_drop_mt`
/// change (atomically); 0 is immortal (statics, interned symbols) and is
/// never written.
///
/// Dup increments a positive count after one compare. Drop decrements a
/// count above one after one compare, and hands a count of one to
/// `cinch_drop_slow`, which releases the block and everything only it
/// kept alive.
fn rc_inline(b: &mut Builder, drop: bool, w: &str, class: Class, frame: bool) {
    let lblock = b.label();
    let ldone = b.label();
    let skip = b.reg();
    b.line(&format!("{skip} = icmp eq i64 {w}, 0"));
    let skip = if class == Class::TObj {
        let lo = b.reg();
        b.line(&format!("{lo} = and i64 {w}, 1"));
        let imm = b.reg();
        b.line(&format!("{imm} = icmp ne i64 {lo}, 0"));
        let either = b.reg();
        b.line(&format!("{either} = or i1 {skip}, {imm}"));
        either
    } else {
        skip
    };
    b.line(&format!("br i1 {skip}, label %{ldone}, label %{lblock}"));
    b.start(&lblock);
    let p = b.reg();
    b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
    let hp = b.reg();
    b.line(&format!(
        "{hp} = getelementptr inbounds i8, ptr {p}, i64 -8"
    ));
    let h = header_load(b, &hp);
    let top = b.reg();
    b.line(&format!("{top} = shl i64 {h}, {RC_TOP_SHIFT}"));
    let one = 1u64 << block::RC_SHIFT;
    let top_one = 1i64 << RC_TOP_SHIFT;
    let (lplain, lother, lmt) = (b.label(), b.label(), b.label());
    // A frame block's count is `RC_FRAME`, and a released one's is
    // `RC_FRAME_RELEASED` (block_kinds.rs): neither is ever counted, so
    // neither is a plain or a shared count. One signed compare excludes
    // both, and the arm is present exactly when this module can hold or
    // receive a frame block, which keeps every other module's helper
    // byte-identical to the one it always had.
    let not_frame = frame.then(|| {
        let nf = b.reg();
        b.line(&format!("{nf} = icmp slt i64 {top}, {RC_FRAME_RELEASED_TOP}"));
        nf
    });
    let plain = b.reg();
    if drop {
        b.line(&format!("{plain} = icmp sgt i64 {top}, {top_one}"));
    } else {
        b.line(&format!("{plain} = icmp sgt i64 {top}, 0"));
    }
    let plain = match &not_frame {
        Some(nf) => {
            let both = b.reg();
            b.line(&format!("{both} = and i1 {plain}, {nf}"));
            both
        }
        None => plain,
    };
    b.line(&format!("br i1 {plain}, label %{lplain}, label %{lother}"));
    b.start(&lplain);
    let h1 = b.reg();
    b.line(&format!(
        "{h1} = {} i64 {h}, {one}",
        if drop { "sub" } else { "add" }
    ));
    b.line(&format!("store i64 {h1}, ptr {hp}"));
    b.line(&format!("br label %{ldone}"));
    b.start(&lother);
    if drop {
        let lslow = b.label();
        // A frame block's drop is its payload's release, once: the block
        // itself is never freed (design/stack-allocation.md section 7.3),
        // and a second drop of the same block finds the released marker
        // and does nothing. A module that can neither hold nor receive a
        // frame block emits the switch it always emitted.
        let lframe = not_frame.as_ref().map(|_| b.label());
        let frame_arm = match &lframe {
            Some(lf) => format!(
                "\n  i64 {RC_FRAME_TOP}, label %{lf}\n  i64 {RC_FRAME_RELEASED_TOP}, label %{lf}"
            ),
            None => String::new(),
        };
        b.line(&format!(
            "switch i64 {top}, label %{lmt} [\n  i64 0, label %{ldone}\n  i64 {top_one}, label %{lslow}{frame_arm}\n]"
        ));
        b.start(&lslow);
        b.line(&format!("call void @cinch_drop_slow(i64 {w})"));
        b.line(&format!("br label %{ldone}"));
        if let Some(lf) = lframe {
            b.start(&lf);
            b.line(&format!("call void @cinch_frame_release(i64 {w})"));
            b.line(&format!("br label %{ldone}"));
        }
    } else {
        let shared = b.reg();
        b.line(&format!("{shared} = icmp ne i64 {top}, 0"));
        let shared = match &not_frame {
            Some(nf) => {
                let both = b.reg();
                b.line(&format!("{both} = and i1 {shared}, {nf}"));
                both
            }
            None => shared,
        };
        b.line(&format!("br i1 {shared}, label %{lmt}, label %{ldone}"));
    }
    b.start(&lmt);
    b.line(&format!(
        "call void @{}(i64 {w})",
        if drop {
            "cinch_drop_mt"
        } else {
            "cinch_dup_mt"
        }
    ));
    b.line(&format!("br label %{ldone}"));
    b.start(&ldone);
}

/// The `cinch_alloc_raw` call that allocates a `size`-byte block with
/// header kind `kind` into `dst` (runtime ABI, `compiler/runtime/src/block.rs`).
fn alloc_call(dst: &str, size: impl std::fmt::Display, kind: u8) -> String {
    format!("{dst} = call ptr @cinch_alloc_raw(i64 {size}, i64 {kind})")
}

/// The `cinch_alloc_shaped` call that allocates a `size`-byte closure or
/// dictionary block of shape descriptor `desc` into `dst`.
fn shaped_alloc_call(dst: &str, size: impl std::fmt::Display, kind: u8, desc: u64) -> String {
    format!(
        "{dst} = call ptr @cinch_alloc_shaped(i64 {size}, i64 {kind}, i64 {})",
        desc as i64
    )
}

/// The task ABI symbol of the target's backend (WebAssembly and WASI
/// specification section 7): the cooperative scheduler under a Wasm
/// artifact profile, the OS-thread scheduler on the host.
fn task_rt(cx: &LowerCx, base: &str) -> String {
    if cx.coop_tasks {
        format!("cinch_coop_{base}")
    } else {
        format!("cinch_{base}")
    }
}

/// The widest `joinAllOrCancel` whose tuple gets exact pointer maps: one
/// anonymous shape per mask of counted payloads, `2^n` of them, each a
/// layout group (64 table words). Four handles cost 16 groups; wider
/// joins keep uncounted tuple slots.
const JOIN_SHAPED_MAX: usize = 4;

fn prim_body(b: &mut Builder, cx: &LowerCx, reg: &Registry, name: &str, op: PrimOp) {
    // `Usize`/`Isize` cross as the target's concrete word scalar
    // (primitive types specification section 2).
    let op = op.resolve_widths(cx.usize_bits);
    use PrimOp::*;
    match op {
        Array {
            op: array_op,
            classes,
            packed,
        } => arrays_codegen::emit(b, reg, name, op, array_op, classes, packed),
        Slots { op: slot_op, classes } => slots_codegen::emit(b, reg, slot_op, classes),
        Stm { op: stm_op, classes } => {
            use crate::stm::StmOp::*;
            match stm_op {
                // The cell takes the value's reference and remembers its
                // class as the scan code of its value word.
                CellNew => {
                    let out = rt_call(
                        b,
                        "cinch_stm_cell_new",
                        &["i64 %p0".to_string(), format!("i64 {}", scan_code(classes[0]))],
                    );
                    ret(b, &out);
                }
                // A cell's word is its address, and a cast, a held word
                // or an opened value word is the word it was given: the
                // unit only changes what the type checker and the count
                // pass know.
                CellAddr | CellCast => ret(b, "%p0"),
                CellHold | CellOpen => ret(b, "%p1"),
            }
        }
        Nat(n) => {
            // A `Nat` word is `(value << 1) | 1` while the value fits 63
            // bits, and a bignum pointer otherwise, so the low bit says
            // which. `add`, `sub` and `cmp` answer their whole result
            // from the immediate form when both operands are immediate,
            // which is every `Nat` an `i64` can carry: those cases are
            // emitted here and only the bignum case calls the runtime.
            // The arithmetic is exact in 64 bits: `a + b` of two 63-bit
            // payloads is at most 2^64 - 2, so the encoded sum's low bit
            // is still the tag, and the shift-back cannot overflow for
            // any pair whose sum is immediate.
            match n {
                crate::core::NatOp::Add | crate::core::NatOp::Sub | crate::core::NatOp::Cmp => {
                    let both = b.reg();
                    b.line(&format!("{both} = and i64 %p0, %p1"));
                    let tag = b.reg();
                    b.line(&format!("{tag} = and i64 {both}, 1"));
                    let fast = b.reg();
                    b.line(&format!("{fast} = icmp ne i64 {tag}, 0"));
                    let (fast_l, slow_l, done_l) = (b.label(), b.label(), b.label());
                    b.line(&format!("br i1 {fast}, label %{fast_l}, label %{slow_l}"));
                    b.line(&format!("{fast_l}:"));
                    // The block that reaches `done` from the fast side: the
                    // borrow check of `sub` adds one.
                    let mut fast_exit = fast_l.clone();
                    let out = match n {
                        // `(a >> 1) + (b >> 1)` re-encoded, and the
                        // reverse for subtraction.
                        crate::core::NatOp::Add | crate::core::NatOp::Sub => {
                            let av = b.reg();
                            b.line(&format!("{av} = lshr i64 %p0, 1"));
                            let bv = b.reg();
                            b.line(&format!("{bv} = lshr i64 %p1, 1"));
                            let sum = b.reg();
                            let is_sub = matches!(n, crate::core::NatOp::Sub);
                            let o = if is_sub { "sub" } else { "add" };
                            b.line(&format!("{sum} = {o} i64 {av}, {bv}"));
                            // `nat-sub`'s scheme is
                            // `using (b <= a) => (a :: Nat) -> (b :: Nat)
                            // -> Nat` (`prelude/Data/Nat.cinch`), so a
                            // subtraction the checker produced cannot go
                            // negative. Hand-built Core can, and the runtime
                            // traps on it, so the borrow still takes the
                            // slow path: one predictable branch, not a
                            // second runtime call. Subtraction cannot go
                            // negative under `nat-sub`'s `b <= a` scheme,
                            // so a borrow means hand-built Core and the
                            // runtime's trap is the answer. Addition *can*
                            // carry: two 63-bit payloads may sum past
                            // `2^63 - 1`, and `(sum << 1) | 1` would then
                            // drop the high bit and report a smaller value
                            // (the seed's own checker misread `2^63` as
                            // zero through exactly this path). The carry
                            // check sends that case to the runtime.
                            if is_sub {
                                let borrow = b.reg();
                                b.line(&format!("{borrow} = icmp ugt i64 {bv}, {av}"));
                                let ok = b.label();
                                b.line(&format!("br i1 {borrow}, label %{slow_l}, label %{ok}"));
                                b.line(&format!("{ok}:"));
                                fast_exit = ok.clone();
                            } else {
                                let carry = b.reg();
                                b.line(&format!("{carry} = icmp ugt i64 {sum}, {}", nat_immediate_max(b.pointer_bytes)));
                                let ok = b.label();
                                b.line(&format!("br i1 {carry}, label %{slow_l}, label %{ok}"));
                                b.line(&format!("{ok}:"));
                                fast_exit = ok.clone();
                            }
                            let enc = b.reg();
                            b.line(&format!("{enc} = shl i64 {sum}, 1"));
                            let word = b.reg();
                            b.line(&format!("{word} = or i64 {enc}, 1"));
                            word
                        }
                        // `cmp` is the raw -1/0/1 on the payloads.
                        _ => {
                            let av = b.reg();
                            b.line(&format!("{av} = lshr i64 %p0, 1"));
                            let bv = b.reg();
                            b.line(&format!("{bv} = lshr i64 %p1, 1"));
                            let lt = b.reg();
                            b.line(&format!("{lt} = icmp ult i64 {av}, {bv}"));
                            let gt = b.reg();
                            b.line(&format!("{gt} = icmp ugt i64 {av}, {bv}"));
                            let neg = b.reg();
                            b.line(&format!("{neg} = select i1 {lt}, i64 -1, i64 0"));
                            let res = b.reg();
                            b.line(&format!("{res} = select i1 {gt}, i64 1, i64 {neg}"));
                            res
                        }
                    };
                    b.line(&format!("br label %{done_l}"));
                    b.line(&format!("{slow_l}:"));
                    let args: Vec<String> = (0..op.arity()).map(|i| format!("i64 %p{i}")).collect();
                    let slow = rt_call(b, n.extern_name(), &args);
                    b.line(&format!("br label %{done_l}"));
                    b.line(&format!("{done_l}:"));
                    let phi = b.reg();
                    b.line(&format!("{phi} = phi i64 [ {out}, %{fast_exit} ], [ {slow}, %{slow_l} ]"));
                    ret(b, &phi);
                }
                // A conversion is one runtime call whatever the width,
                // and the bignum operations have no immediate form to
                // take (a product overflows the payload at once).
                _ => {
                    let args: Vec<String> = (0..op.arity()).map(|i| format!("i64 %p{i}")).collect();
                    let r = rt_call(b, n.extern_name(), &args);
                    ret(b, &r);
                }
            }
        }
        Int(i) => {
            let args: Vec<String> = (0..op.arity()).map(|i| format!("i64 %p{i}")).collect();
            let r = rt_call(b, i.extern_name(), &args);
            ret(b, &r);
        }
        Eval => {
            // `eval q`: the quote program runs through the runtime
            // evaluator (supercompilation and JIT specification
            // section 7); the fault path is the runtime's defined one.
            let r = rt_call(b, "cinch_eval", &["i64 %p0".to_string()]);
            ret(b, &r);
        }
        Utf8(k) => {
            // The UTF-8 runtime (`compiler/runtime/src/string.rs`) takes
            // pointer and length over the `Bytes` payload; the units pass
            // the buffer through `cinch_bytes_data`/`cinch_bytes_len` and build
            // the surface results over the prelude `Option` and tuple
            // blocks (a malformed buffer in `utf8CountChars` or a
            // non-scalar `utf8Encode` argument is a contract violation:
            // the defined fault path, never a value).
            use Utf8Op::*;
            let ptr = rt_call(b, "cinch_bytes_data", &["i64 %p0".to_string()]);
            let len = rt_call(b, "cinch_bytes_len", &["i64 %p0".to_string()]);
            match k {
                Validate => {
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &[format!("i64 {ptr}"), format!("i64 {len}")],
                    );
                    ret(b, &r);
                }
                CountChars => {
                    let n = rt_call(
                        b,
                        k.extern_name(),
                        &[format!("i64 {ptr}"), format!("i64 {len}")],
                    );
                    let bad = b.reg();
                    b.line(&format!("{bad} = icmp slt i64 {n}, 0"));
                    fault_when(
                        b,
                        name,
                        &bad,
                        "utf8CountChars: the buffer is not valid UTF-8 (the count is undefined)",
                    );
                    let r = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {n}")]);
                    ret(b, &r);
                }
                DecodeAt => {
                    // A clamped index past `i64` is past every buffer's
                    // end, so it decodes as `None`.
                    let i = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let n = rt_call(
                        b,
                        k.extern_name(),
                        &[
                            format!("i64 {ptr}"),
                            format!("i64 {len}"),
                            format!("i64 {i}"),
                        ],
                    );
                    let bad = b.reg();
                    b.line(&format!("{bad} = icmp slt i64 {n}, 0"));
                    ret_option(b, reg, op, name, &bad, |b| {
                        // The extern packs the scalar value in the low
                        // 32 bits and the byte width above them.
                        let cp = b.reg();
                        b.line(&format!("{cp} = and i64 {n}, 4294967295"));
                        let w = b.reg();
                        b.line(&format!("{w} = lshr i64 {n}, 32"));
                        let nw = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")]);
                        // `(Char, Nat)`: the code point is raw, the width a
                        // `Nat` word.
                        ctor_block_word(b, reg.shape_desc(&[Class::Raw, Class::TObj], 0), &[cp, nw])
                    });
                }
                Encode => {
                    let tmp = rt_call(b, "cinch_bytes_new", &["i64 4".to_string()]);
                    let out = rt_call(b, "cinch_bytes_data", &[format!("i64 {tmp}")]);
                    let n = rt_call(
                        b,
                        k.extern_name(),
                        &[
                            "i64 %p0".to_string(),
                            format!("i64 {out}"),
                            "i64 4".to_string(),
                        ],
                    );
                    let bad = b.reg();
                    b.line(&format!("{bad} = icmp slt i64 {n}, 0"));
                    fault_when(
                        b,
                        name,
                        &bad,
                        "utf8Encode: the value is not a Unicode scalar value",
                    );
                    let r = rt_call(
                        b,
                        "cinch_bytes_from_data",
                        &[format!("i64 {out}"), format!("i64 {n}")],
                    );
                    ret(b, &r);
                }
            }
        }
        Bytes(k) => {
            // The `Bytes` externs take machine counts, indices and byte
            // values; the intrinsic units convert `Nat` words through
            // `cinch_nat_to_i64` and results back through `cinch_nat_from_i64`.
            // Every index is below a buffer length (its obligation) and
            // every byte value below 256 (its type), so those conversions
            // never leave `i64` range; a length goes through
            // `cinch_nat_to_len`, where a bignum is the heap `Exhausted`
            // exit of an allocation that cannot be held.
            let args: Vec<String> = (0..op.arity()).map(|i| format!("i64 %p{i}")).collect();
            match k {
                BytesOp::New => {
                    let n = rt_call(b, "cinch_nat_to_len", &[args[0].clone()]);
                    let r = rt_call(b, k.extern_name(), &[format!("i64 {n}")]);
                    ret(b, &r);
                }
                BytesOp::Len => {
                    let l = rt_call(b, k.extern_name(), &args);
                    let r = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {l}")]);
                    ret(b, &r);
                }
                BytesOp::Get => {
                    let i = rt_call(b, "cinch_nat_to_i64", &[args[1].clone()]);
                    let v = rt_call(b, k.extern_name(), &[args[0].clone(), format!("i64 {i}")]);
                    // `Bytes` data is untrusted (foreign spans wrap into
                    // buffers): the byte freezes before it can branch
                    // or compare (the plan note's poison containment).
                    let v = freeze_word(b, &v);
                    let r = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {v}")]);
                    ret(b, &r);
                }
                BytesOp::Set => {
                    let i = rt_call(b, "cinch_nat_to_i64", &[args[1].clone()]);
                    let v = rt_call(b, "cinch_nat_to_i64", &[args[2].clone()]);
                    let _ = rt_call(
                        b,
                        k.extern_name(),
                        &[args[0].clone(), format!("i64 {i}"), format!("i64 {v}")],
                    );
                    ret(b, "0");
                }
                BytesOp::Copy => {
                    let di = rt_call(b, "cinch_nat_to_i64", &[args[2].clone()]);
                    let si = rt_call(b, "cinch_nat_to_i64", &[args[3].clone()]);
                    let n = rt_call(b, "cinch_nat_to_i64", &[args[4].clone()]);
                    let _ = rt_call(
                        b,
                        k.extern_name(),
                        &[
                            args[0].clone(),
                            args[1].clone(),
                            format!("i64 {di}"),
                            format!("i64 {si}"),
                            format!("i64 {n}"),
                        ],
                    );
                    ret(b, "0");
                }
                BytesOp::Data => {
                    // The payload address as a word: the one place a
                    // `Bytes` becomes something the kernel can be handed.
                    // Nothing to box and nothing to freeze, because the
                    // value is used as a pointer, never as a branch.
                    let p = rt_call(b, k.extern_name(), &args);
                    ret(b, &p);
                }
                BytesOp::Compare => {
                    // The order is a machine word (-1/0/1), not a `Nat`:
                    // nothing to box.
                    let r = rt_call(b, k.extern_name(), &args);
                    ret(b, &r);
                }
            }
        }
        Sym(k) => {
            // The intern table (`compiler/runtime/src/sym.rs`) takes a
            // pointer-length name span, like the UTF-8 runtime; the units
            // pass the `Bytes` payload through `cinch_bytes_data`/
            // `cinch_bytes_len` and hands cells back as words.
            use SymOp::*;
            match k {
                Intern => {
                    let ptr = rt_call(b, "cinch_bytes_data", &["i64 %p0".to_string()]);
                    let len = rt_call(b, "cinch_bytes_len", &["i64 %p0".to_string()]);
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &[format!("i64 {ptr}"), format!("i64 {len}")],
                    );
                    ret(b, &r);
                }
                Name => {
                    let r = rt_call(b, k.extern_name(), &["i64 %p0".to_string()]);
                    ret(b, &r);
                }
                Eq | Cmp => {
                    // `symEq` is the identity pointer test (0 or 1);
                    // `symCmp` is the raw -1/0/1 of the name order.
                    let args: Vec<String> = (0..op.arity()).map(|i| format!("i64 %p{i}")).collect();
                    let r = rt_call(b, k.extern_name(), &args);
                    ret(b, &r);
                }
            }
        }
        Text(k) => {
            // Text conversion and content identity
            // (`compiler/runtime/src/text.rs`): the buffer conversions
            // cross as words, the float conversions bitcast through the
            // canonical word like the other `F64` primals.
            use TextOp::*;
            match k {
                Sha256 | Sha512 | Sha1 | Md5 => {
                    let r = rt_call(b, k.extern_name(), &["i64 %p0".to_string()]);
                    ret(b, &r);
                }
                HmacSha256 => {
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), "i64 %p1".to_string()],
                    );
                    ret(b, &r);
                }
                F64ToDecimal => {
                    let f = word_to_float(b, Scalar::F64, "%p0");
                    let r = rt_call(b, k.extern_name(), &[format!("double {f}")]);
                    ret(b, &r);
                }
                F64FromDecimal => {
                    let r = b.reg();
                    b.line(&format!("{r} = call double @{}(i64 %p0)", k.extern_name()));
                    let w = float_to_word(b, Scalar::F64, &r);
                    ret(b, &w);
                }
            }
        }
        Platform(k) => {
            // The platform operations (`compiler/runtime/src/platform.rs`):
            // the `Nat` count crosses as an `i64` (like the `Bytes`
            // counts), the paths and results cross as words, and the
            // flags are `Bool` immediates.
            use PlatformOp::*;
            match k {
                RandomBytes => {
                    let n = rt_call(b, "cinch_nat_to_len", &["i64 %p0".to_string()]);
                    let r = rt_call(b, k.extern_name(), &[format!("i64 {n}")]);
                    ret(b, &r);
                }
                FsMkdir | FsRemove | SignalSend => {
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), "i64 %p1".to_string()],
                    );
                    ret(b, &r);
                }
                // The arena entries take and answer one machine word: the
                // size or the handle, both never blocks.
                ArenaNew | ArenaLeave => {
                    let r = rt_call(b, k.extern_name(), &["i64 %p0".to_string()]);
                    ret(b, &r);
                }
                FsTempDir => {
                    let r = rt_call(b, k.extern_name(), &[]);
                    ret(b, &r);
                }
            }
        }
        Net(k) => {
            // The networking units (`compiler/runtime/src/net.rs`): host
            // text and payloads cross as `Bytes` words, ports and
            // handles as machine scalars, and the boundary word becomes
            // the prelude `Result` through `ret_result`.
            use NetOp::*;
            match k {
                TcpConnect | Resolve => {
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), "i64 %p1".to_string()],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Write => {
                    // `(handle, data)`: the byte count is a `Nat`.
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), "i64 %p1".to_string()],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |b| {
                        rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")])
                    });
                }
                Read => {
                    // `(handle, max)`: the `Nat` cap crosses as an `i64`
                    // (like the `Bytes` counts, clamped by
                    // `cinch_nat_to_len`).
                    let max = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), format!("i64 {max}")],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Close => {
                    let r = rt_call(b, k.extern_name(), &["i64 %p0".to_string()]);
                    let w = freeze_word(b, &r);
                    // `Ok` over the Unit word.
                    ret_result(b, reg, op, &w, |_| "0".to_string());
                }
            }
        }
        Proc(k) => {
            // The process units (the argv capture and the piped section of
            // `compiler/runtime/src/platform.rs`): the program text, the
            // NUL-separated argument and environment words and the
            // payloads cross as `Bytes` words, the handles and the pipe
            // selector as machine scalars, and the child half's boundary
            // word becomes the prelude `Result` through `ret_result`.
            // The command line is raw rather than a `Result`: a captured
            // word has no failure mode.
            use ProcOp::*;
            let raw_args =
                |n: usize| -> Vec<String> { (0..n).map(|i| format!("i64 %p{i}")).collect() };
            match k {
                ArgCount => {
                    // The `Unit` argument is the prelude's arity spelling
                    // only; the extern takes none (as `fsTempDir`), and
                    // the machine count boxes as the `Nat` result.
                    let c = rt_call(b, k.extern_name(), &[]);
                    let n = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {c}")]);
                    ret(b, &n);
                }
                Arg => {
                    // The `Nat` index crosses as an `i64` (it is below
                    // the count by `raw-arg`'s obligation); the result is
                    // already a boxed `Bytes` word.
                    let i = rt_call(b, "cinch_nat_to_len", &["i64 %p0".to_string()]);
                    let w = rt_call(b, k.extern_name(), &[format!("i64 {i}")]);
                    ret(b, &w);
                }
                Spawn => {
                    let r = rt_call(b, k.extern_name(), &raw_args(4));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                WriteStdin => {
                    let r = rt_call(b, k.extern_name(), &raw_args(2));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |b| {
                        rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")])
                    });
                }
                Read => {
                    // `(handle, which, max)`: the `Nat` cap crosses as an
                    // `i64`, clamped by `cinch_nat_to_len`.
                    let max = rt_call(b, "cinch_nat_to_len", &["i64 %p2".to_string()]);
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &[
                            "i64 %p0".to_string(),
                            "i64 %p1".to_string(),
                            format!("i64 {max}"),
                        ],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Wait => {
                    let r = rt_call(b, k.extern_name(), &raw_args(1));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |b| {
                        rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")])
                    });
                }
                CloseStdin | Close => {
                    let r = rt_call(b, k.extern_name(), &raw_args(1));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| "0".to_string());
                }
            }
        }
        Pty(k) => {
            // The pseudo-terminal units (`compiler/runtime/src/pty.rs`):
            // the program text, the NUL-separated argument and
            // environment words and the payloads cross as `Bytes` words,
            // the handles and the window size as machine scalars, and the
            // boundary word becomes the prelude `Result` through
            // `ret_result`.
            use PtyOp::*;
            let raw_args =
                |n: usize| -> Vec<String> { (0..n).map(|i| format!("i64 %p{i}")).collect() };
            match k {
                Open => {
                    // The `Unit` argument is the prelude's arity spelling
                    // only; the extern takes no arguments (as `fsTempDir`).
                    let r = rt_call(b, k.extern_name(), &[]);
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Spawn => {
                    let r = rt_call(b, k.extern_name(), &raw_args(5));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Write => {
                    let r = rt_call(b, k.extern_name(), &raw_args(2));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |b| {
                        rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")])
                    });
                }
                Read => {
                    // `(handle, max)`: the `Nat` cap crosses as an `i64`,
                    // clamped by `cinch_nat_to_len`.
                    let max = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &["i64 %p0".to_string(), format!("i64 {max}")],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| w.clone());
                }
                Wait => {
                    let r = rt_call(b, k.extern_name(), &raw_args(1));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |b| {
                        rt_call(b, "cinch_nat_from_i64", &[format!("i64 {w}")])
                    });
                }
                Close => {
                    let r = rt_call(b, k.extern_name(), &raw_args(1));
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| "0".to_string());
                }
                Resize => {
                    // `(handle, cols, rows)`: the two `Nat` sizes cross as
                    // `i64`, clamped by `cinch_nat_to_len`; the runtime
                    // refuses a size past `u16` as a value.
                    let cols = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let rows = rt_call(b, "cinch_nat_to_len", &["i64 %p2".to_string()]);
                    let r = rt_call(
                        b,
                        k.extern_name(),
                        &[
                            "i64 %p0".to_string(),
                            format!("i64 {cols}"),
                            format!("i64 {rows}"),
                        ],
                    );
                    let w = freeze_word(b, &r);
                    ret_result(b, reg, op, &w, |_| "0".to_string());
                }
            }
        }
        Par { op: par_op, classes } => {
            // Deterministic pure parallelism (design/parallel-evaluation.md
            // sections 2 and 4.3): the units wrap the runtime's work-stealing
            // entries of `compiler/runtime/src/par.rs`, or their sequential
            // readings on the cooperative backend (`cinch_coop_par_*`). The
            // callback closures are borrowed (the caller drops them after the
            // call) and the runtime duplicates one reference per invocation.
            // Every class here is an immediate baked into the body.
            use crate::core::ParOp::*;
            match par_op {
                Both => {
                    // `out[0]`, `out[1]` are owned result words; the pair
                    // takes the two component classes' exact pointer map, so
                    // no mask and no shaped-join limit apply (section 4.3).
                    let out = b.alloca_words(2);
                    let outw = b.reg();
                    b.line(&format!("{outw} = ptrtoint ptr {out} to i64"));
                    let _ = rt_call(
                        b,
                        &task_rt(cx, "par_both"),
                        &[
                            "i64 %p0".to_string(),
                            "i64 %p1".to_string(),
                            format!("i64 {outw}"),
                        ],
                    );
                    let first = b.load_at(&out, 0);
                    let second = b.load_at(&out, 8);
                    let pair = ctor_block_word(b, reg.shape_desc(&classes, 0), &[first, second]);
                    ret(b, &pair);
                }
                Tabulate => {
                    // `n` crosses as a length (`cinch_nat_to_len`), and the
                    // element class as the scan code the array constructors
                    // take.
                    let n = rt_call(b, "cinch_nat_to_len", &["i64 %p0".to_string()]);
                    let class = scan_code(classes[0]);
                    let arr = rt_call(
                        b,
                        &task_rt(cx, "par_tabulate"),
                        &[
                            format!("i64 {n}"),
                            "i64 %p1".to_string(),
                            format!("i64 {class}"),
                        ],
                    );
                    ret(b, &arr);
                }
                MapArray => {
                    let class = scan_code(classes[0]);
                    let arr = rt_call(
                        b,
                        &task_rt(cx, "par_map_array"),
                        &[
                            "i64 %p0".to_string(),
                            "i64 %p1".to_string(),
                            format!("i64 {class}"),
                        ],
                    );
                    ret(b, &arr);
                }
            }
        }
        Conc(k) => {
            // Structured concurrency and runtime I/O (concurrency
            // specification sections 2, 4 and 9): the units wrap the
            // `cinch_*` externs of `compiler/runtime/src/task.rs`. The task
            // outcome ABI fixes the prelude `Result` tags: `Err` tag 0
            // (failure), `Ok` tag 1 (success).
            use ConcOp::*;
            match k {
                WithScope => {
                    let scope = rt_call(b, &task_rt(cx, "scope_open"), &[]);
                    let argv = b.alloca_words(1);
                    b.store_at(&argv, 0, &scope);
                    let val = b.reg();
                    b.line(&format!(
                        "{val} = call i64 @__apply(i64 %p1, i64 1, ptr {argv})"
                    ));
                    let _ = rt_call(b, &task_rt(cx, "scope_close"), &[format!("i64 {scope}")]);
                    ret(b, &val);
                }
                Spawn => {
                    let h = rt_call(
                        b,
                        &task_rt(cx, "task_spawn"),
                        &["i64 %p0".to_string(), "i64 %p1".to_string()],
                    );
                    ret(b, &h);
                }
                JoinN(n) => {
                    let arr = b.alloca_words(n);
                    for i in 0..n {
                        b.store_at(&arr, i * 8, &format!("%p{}", i + 1));
                    }
                    let out = b.alloca_words(n.max(1));
                    let arrw = b.reg();
                    b.line(&format!("{arrw} = ptrtoint ptr {arr} to i64"));
                    let outw = b.reg();
                    b.line(&format!("{outw} = ptrtoint ptr {out} to i64"));
                    let st = rt_call(
                        b,
                        &task_rt(cx, "task_join_all_or_cancel"),
                        &[
                            "i64 %p0".to_string(),
                            format!("i64 {arrw}"),
                            format!("i64 {n}"),
                            format!("i64 {outw}"),
                        ],
                    );
                    // The runtime's hand-over (`crate::task::join_failed`,
                    // `join_succeeded`): a failure, or one member, is that
                    // member's own `Result` block, built by its body with
                    // the payload's class in its map, so it is the join's
                    // result as it stands and owns what it holds.
                    if n == 1 {
                        let r = b.load_at(&out, 0);
                        ret(b, &r);
                        return;
                    }
                    let lfail = b.label();
                    let lok = b.label();
                    let failed = b.reg();
                    b.line(&format!("{failed} = icmp slt i64 {st}, 0"));
                    b.line(&format!("br i1 {failed}, label %{lfail}, label %{lok}"));
                    b.start(&lfail);
                    let ew = b.load_at(&out, 0);
                    ret(b, &ew);
                    // Success over several members: `Ok` over a tuple of
                    // their payloads, which the runtime moved out of their
                    // `Ok` blocks. This unit does not see the bodies' result
                    // types, so the runtime reports which payloads are
                    // counted (bit `i` of the mask) and the tuple takes the
                    // pointer map of that mask, one shape per mask (`TObj`
                    // covers both counted classes). Each shape costs a
                    // layout group, so joins wider than `JOIN_SHAPED_MAX`
                    // keep uncounted slots, which can only leak.
                    b.start(&lok);
                    let parts: Vec<String> = (0..n).map(|i| b.load_at(&out, i * 8)).collect();
                    let desc = if n <= JOIN_SHAPED_MAX {
                        let descs: Vec<String> = (0..1u64 << n)
                            .map(|mask| {
                                let map: Vec<Class> = (0..n)
                                    .map(|i| {
                                        if mask >> i & 1 == 1 {
                                            Class::TObj
                                        } else {
                                            Class::Raw
                                        }
                                    })
                                    .collect();
                                format!("i64 {}", reg.shape_desc(&map, 0) as i64)
                            })
                            .collect();
                        let table = quote(&format!("{name}.shapes"));
                        b.global(format!(
                            "@{table} = internal constant [{} x i64] [{}]",
                            descs.len(),
                            descs.join(", ")
                        ));
                        let at = b.reg();
                        b.line(&format!("{at} = getelementptr inbounds [{} x i64], ptr @{table}, i64 0, i64 {st}", descs.len()));
                        let d = b.reg();
                        b.line(&format!("{d} = load i64, ptr {at}"));
                        d
                    } else {
                        (reg.raw_shape_desc(n as u64, 0) as i64).to_string()
                    };
                    let tp = b.reg();
                    b.line(&format!(
                        "{tp} = call ptr @cinch_alloc_ctor(i64 {}, i64 {desc})",
                        8 * n
                    ));
                    for (i, part) in parts.iter().enumerate() {
                        b.store_at(&tp, 8 * i, part);
                    }
                    let tuple = b.reg();
                    b.line(&format!("{tuple} = ptrtoint ptr {tp} to i64"));
                    let okw = ctor_block_word(b, reg.shape_desc(&[Class::Obj], TAG_OK), &[tuple]);
                    ret(b, &okw);
                }
                IoWrite => {
                    let _ = rt_call(b, "cinch_io_write", &["i64 %p1".to_string()]);
                    ret(b, "0");
                }
                IoRead => {
                    let n = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let r = rt_call(b, "cinch_io_read", &[format!("i64 {n}")]);
                    let none = b.reg();
                    b.line(&format!("{none} = icmp eq i64 {r}, 0"));
                    ret_option(b, reg, op, name, &none, |_| r.clone());
                }
                IoSleep => {
                    // A clamped duration past `i64` milliseconds sleeps until
                    // cancellation, as the exact one would.
                    let n = rt_call(b, "cinch_nat_to_len", &["i64 %p1".to_string()]);
                    let _ = rt_call(b, &task_rt(cx, "io_sleep"), &[format!("i64 {n}")]);
                    ret(b, "0");
                }
                PollCancel => {
                    let c = rt_call(b, &task_rt(cx, "task_poll_cancel"), &[]);
                    ret(b, &c);
                }
            }
        }
        AndBool | OrBool => {
            let inst = match op {
                AndBool => "and",
                OrBool => "or",
                _ => unreachable!(),
            };
            let r = b.reg();
            b.line(&format!("{r} = {inst} i64 %p0, %p1"));
            ret(b, &r);
        }
        NotBool => {
            let r = b.reg();
            b.line(&format!("{r} = xor i64 %p0, 1"));
            ret(b, &r);
        }
        EqBool | NeBool => {
            let pred = if op == EqBool { "eq" } else { "ne" };
            let c = b.reg();
            b.line(&format!("{c} = icmp {pred} i64 %p0, %p1"));
            let r = b.reg();
            b.line(&format!("{r} = zext i1 {c} to i64"));
            ret(b, &r);
        }
        Delay => {
            // A thunk is already a closure over Unit; `delay` is identity.
            ret(b, "%p0");
        }
        Force => {
            let argv = b.reg();
            b.line(&format!("{argv} = alloca i64, i64 1"));
            b.store_at(&argv, 0, "0");
            let r = b.reg();
            b.line(&format!(
                "{r} = call i64 @__apply(i64 %p0, i64 1, ptr {argv})"
            ));
            ret(b, &r);
        }
        ApplyErased => {
            let argv = b.alloca_words(0);
            let r = b.reg();
            b.line(&format!("{r} = call i64 @__apply(i64 %p0, i64 0, ptr {argv})"));
            ret(b, &r);
        }
        Bound { ty, max } => {
            let w = if max { max_word(ty) } else { min_word(ty) };
            ret(b, &w.to_string());
        }
        Arith { ty, op: aop, mode } => {
            let it = match aop {
                ArithOp::Add => "add",
                ArithOp::Sub => "sub",
                ArithOp::Mul => "mul",
            };
            match mode {
                ArithMode::Wrapping => {
                    let r = b.reg();
                    b.line(&format!("{r} = {it} i64 %p0, %p1"));
                    let c = canon(b, ty, &r);
                    ret(b, &c);
                }
                ArithMode::Checked | ArithMode::Saturating => {
                    arith_overflow(b, reg, op, name, ty, aop, mode)
                }
            }
        }
        Neg { ty, mode } => match mode {
            ArithMode::Wrapping => {
                let r = b.reg();
                b.line(&format!("{r} = sub i64 0, %p0"));
                let c = canon(b, ty, &r);
                ret(b, &c);
            }
            ArithMode::Checked => {
                let bad = b.reg();
                b.line(&format!("{bad} = icmp eq i64 %p0, {}", min_word(ty)));
                ret_option(b, reg, op, name, &bad, |b| {
                    let r = b.reg();
                    b.line(&format!("{r} = sub i64 0, %p0"));
                    canon(b, ty, &r)
                });
            }
            ArithMode::Saturating => {
                let is_min = b.reg();
                b.line(&format!("{is_min} = icmp eq i64 %p0, {}", min_word(ty)));
                let r = b.reg();
                b.line(&format!("{r} = sub i64 0, %p0"));
                let c = canon(b, ty, &r);
                let out = b.reg();
                b.line(&format!(
                    "{out} = select i1 {is_min}, i64 {}, i64 {c}",
                    max_word(ty)
                ));
                ret(b, &out);
            }
        },
        CheckedDiv { ty } | CheckedRem { ty } => {
            // `None` on a zero divisor and on `minBound / -1` for signed
            // types (primitive types specification, section 4).
            let z = b.reg();
            b.line(&format!("{z} = icmp eq i64 %p1, 0"));
            let bad = if ty.is_signed() {
                let m1 = b.reg();
                b.line(&format!("{m1} = icmp eq i64 %p1, -1"));
                let mb = b.reg();
                b.line(&format!("{mb} = icmp eq i64 %p0, {}", min_word(ty)));
                let ov = b.reg();
                b.line(&format!("{ov} = and i1 {m1}, {mb}"));
                let r = b.reg();
                b.line(&format!("{r} = or i1 {z}, {ov}"));
                r
            } else {
                z
            };
            ret_option(b, reg, op, name, &bad, |b| {
                let inst = match (op, ty.is_signed()) {
                    (CheckedDiv { .. }, true) => "sdiv",
                    (CheckedDiv { .. }, false) => "udiv",
                    (CheckedRem { .. }, true) => "srem",
                    (CheckedRem { .. }, false) => "urem",
                    _ => unreachable!(),
                };
                let r = b.reg();
                b.line(&format!("{r} = {inst} i64 %p0, %p1"));
                canon(b, ty, &r)
            });
        }
        Shift {
            ty,
            op: sop,
            checked,
        } => {
            let w = ty.width() as i64;
            let count = if checked {
                // `checked*` return `None` for counts at or above the
                // width (primitive types specification, section 4).
                "%p1".to_string()
            } else {
                // `wrapping*` reduce the count modulo the width; every
                // width is a power of two.
                let r = b.reg();
                b.line(&format!("{r} = and i64 %p1, {}", w - 1));
                r
            };
            let sh = |b: &mut Builder| -> String {
                let inst = match sop {
                    ShiftOp::Shl => "shl",
                    ShiftOp::Shr => {
                        if ty.is_signed() {
                            "ashr"
                        } else {
                            "lshr"
                        }
                    }
                };
                let r = b.reg();
                b.line(&format!("{r} = {inst} i64 %p0, {count}"));
                canon(b, ty, &r)
            };
            if checked {
                let bad = b.reg();
                b.line(&format!("{bad} = icmp uge i64 %p1, {w}"));
                ret_option(b, reg, op, name, &bad, sh);
            } else {
                let v = sh(b);
                ret(b, &v);
            }
        }
        Rotate { ty, right } => {
            // Funnel shifts reduce their count modulo the width: total by
            // construction (primitive types specification, section 4).
            let it = int_ty(ty);
            let a = trunc_n(b, ty, "%p0");
            let c = trunc_n(b, ty, "%p1");
            let f = if right { "fshr" } else { "fshl" };
            let r = b.reg();
            b.line(&format!(
                "{r} = call {it} @llvm.{f}.{it}({it} {a}, {it} {a}, {it} {c})"
            ));
            let w = ext_n(b, ty, &r);
            ret(b, &w);
        }
        Bit { ty, op: bop } => {
            let inst = match bop {
                BitOp::And => "and",
                BitOp::Or => "or",
                BitOp::Xor => "xor",
            };
            let r = b.reg();
            b.line(&format!("{r} = {inst} i64 %p0, %p1"));
            let c = canon(b, ty, &r);
            ret(b, &c);
        }
        BitNot { ty } => {
            let r = b.reg();
            b.line(&format!("{r} = xor i64 %p0, -1"));
            let c = canon(b, ty, &r);
            ret(b, &c);
        }
        BitScan { ty, op: sop } => {
            let it = int_ty(ty);
            let a = trunc_n(b, ty, "%p0");
            match sop {
                BitScanOp::CountOnes => {
                    let r = b.reg();
                    b.line(&format!("{r} = call {it} @llvm.ctpop.{it}({it} {a})"));
                    let w = ext_z(b, ty, &r);
                    ret(b, &w);
                }
                BitScanOp::LeadingZeros | BitScanOp::TrailingZeros => {
                    // Counts are `U32`; the zero word is well defined
                    // (`i1 false` selects the width as the answer).
                    let inst = if sop == BitScanOp::LeadingZeros {
                        "ctlz"
                    } else {
                        "cttz"
                    };
                    let r = b.reg();
                    b.line(&format!(
                        "{r} = call {it} @llvm.{inst}.{it}({it} {a}, i1 false)"
                    ));
                    let w = ext_z(b, ty, &r);
                    ret(b, &w);
                }
                BitScanOp::ReverseBits => {
                    let r = b.reg();
                    b.line(&format!("{r} = call {it} @llvm.bitreverse.{it}({it} {a})"));
                    let w = ext_n(b, ty, &r);
                    ret(b, &w);
                }
                BitScanOp::SwapBytes => {
                    if ty.width() == 8 {
                        // Identity on one-byte types (section 4).
                        ret(b, "%p0");
                    } else {
                        let r = b.reg();
                        b.line(&format!("{r} = call {it} @llvm.bswap.{it}({it} {a})"));
                        let w = ext_n(b, ty, &r);
                        ret(b, &w);
                    }
                }
            }
        }
        Cmp { ty, op: cop } => {
            let pred = match (cop, ty.is_signed()) {
                (CmpOp::Eq, _) => "eq",
                (CmpOp::Ne, _) => "ne",
                (CmpOp::Lt, true) => "slt",
                (CmpOp::Le, true) => "sle",
                (CmpOp::Gt, true) => "sgt",
                (CmpOp::Ge, true) => "sge",
                (CmpOp::Lt, false) => "ult",
                (CmpOp::Le, false) => "ule",
                (CmpOp::Gt, false) => "ugt",
                (CmpOp::Ge, false) => "uge",
            };
            let c = b.reg();
            b.line(&format!("{c} = icmp {pred} i64 %p0, %p1"));
            let r = b.reg();
            b.line(&format!("{r} = zext i1 {c} to i64"));
            ret(b, &r);
        }

        FloatBin { ty, op: fop } => {
            let ft = float_ty(ty);
            let x = word_to_float(b, ty, "%p0");
            let y = word_to_float(b, ty, "%p1");
            let v = match fop {
                FloatBinOp::Add | FloatBinOp::Sub | FloatBinOp::Mul | FloatBinOp::Div => {
                    let inst = match fop {
                        FloatBinOp::Add => "fadd",
                        FloatBinOp::Sub => "fsub",
                        FloatBinOp::Mul => "fmul",
                        FloatBinOp::Div => "fdiv",
                        _ => unreachable!(),
                    };
                    let r = b.reg();
                    b.line(&format!("{r} = {inst} {ft} {x}, {y}"));
                    float_to_word(b, ty, &r)
                }
                FloatBinOp::Minimum | FloatBinOp::Maximum => {
                    // IEEE 754-2019 minimum/maximum: propagate NaN (as a
                    // quiet NaN), and distinguish -0 from +0.
                    let qmask: i64 = if ty == Scalar::F32 {
                        0x00400000
                    } else {
                        0x0008000000000000
                    };
                    let nan = b.reg();
                    b.line(&format!("{nan} = fcmp uno {ft} {x}, {y}"));
                    let xnan = b.reg();
                    b.line(&format!("{xnan} = fcmp uno {ft} {x}, {x}"));
                    let sel = b.reg();
                    b.line(&format!("{sel} = select i1 {xnan}, i64 %p0, i64 %p1"));
                    let q = b.reg();
                    b.line(&format!("{q} = or i64 {sel}, {qmask}"));
                    let eq = b.reg();
                    b.line(&format!("{eq} = fcmp oeq {ft} {x}, {y}"));
                    // On equality the words are identical except for -0
                    // and +0, where minimum picks -0 (the `or` of the sign
                    // bits) and maximum picks +0 (the `and`).
                    let bit = if fop == FloatBinOp::Minimum {
                        "or"
                    } else {
                        "and"
                    };
                    let both = b.reg();
                    b.line(&format!("{both} = {bit} i64 %p0, %p1"));
                    let cmp = b.reg();
                    let pred = if fop == FloatBinOp::Minimum {
                        "olt"
                    } else {
                        "ogt"
                    };
                    b.line(&format!("{cmp} = fcmp {pred} {ft} {x}, {y}"));
                    let pick = b.reg();
                    b.line(&format!("{pick} = select i1 {cmp}, i64 %p0, i64 %p1"));
                    let nonnan = b.reg();
                    b.line(&format!(
                        "{nonnan} = select i1 {eq}, i64 {both}, i64 {pick}"
                    ));
                    let r = b.reg();
                    b.line(&format!("{r} = select i1 {nan}, i64 {q}, i64 {nonnan}"));
                    r
                }
            };
            ret(b, &v);
        }
        FloatUn { ty, op: fop } => {
            let ft = float_ty(ty);
            let x = word_to_float(b, ty, "%p0");
            let r = b.reg();
            match fop {
                FloatUnOp::Neg => b.line(&format!("{r} = fneg {ft} {x}")),
                FloatUnOp::Abs => b.line(&format!("{r} = call {ft} @llvm.fabs.{ft}({ft} {x})")),
                FloatUnOp::Sqrt => b.line(&format!("{r} = call {ft} @llvm.sqrt.{ft}({ft} {x})")),
            }
            let w = float_to_word(b, ty, &r);
            ret(b, &w);
        }
        FCmp { ty, op: cop } => {
            // Quiet, non-signaling IEEE comparisons (section 5): `fne` is
            // unordered-or-not-equal, so it is true on NaN.
            let pred = match cop {
                CmpOp::Eq => "oeq",
                CmpOp::Ne => "une",
                CmpOp::Lt => "olt",
                CmpOp::Le => "ole",
                CmpOp::Gt => "ogt",
                CmpOp::Ge => "oge",
            };
            let ft = float_ty(ty);
            let x = word_to_float(b, ty, "%p0");
            let y = word_to_float(b, ty, "%p1");
            let c = b.reg();
            b.line(&format!("{c} = fcmp {pred} {ft} {x}, {y}"));
            let r = b.reg();
            b.line(&format!("{r} = zext i1 {c} to i64"));
            ret(b, &r);
        }
        FClass { ty, op: cop } => {
            let ft = float_ty(ty);
            let x = word_to_float(b, ty, "%p0");
            let c = match cop {
                FClassOp::IsNan => {
                    let r = b.reg();
                    b.line(&format!("{r} = fcmp uno {ft} {x}, {x}"));
                    r
                }
                FClassOp::IsInfinite => {
                    let a = b.reg();
                    b.line(&format!("{a} = call {ft} @llvm.fabs.{ft}({ft} {x})"));
                    let r = b.reg();
                    b.line(&format!(
                        "{r} = fcmp oeq {ft} {a}, {}",
                        fcst(ty, f64::INFINITY)
                    ));
                    r
                }
                FClassOp::IsFinite => {
                    let a = b.reg();
                    b.line(&format!("{a} = call {ft} @llvm.fabs.{ft}({ft} {x})"));
                    let r = b.reg();
                    b.line(&format!(
                        "{r} = fcmp olt {ft} {a}, {}",
                        fcst(ty, f64::INFINITY)
                    ));
                    r
                }
                FClassOp::IsNormal => {
                    // Normal: the exponent bits are neither all zero nor
                    // all ones (subnormals and infinities excluded).
                    let it = int_ty(ty);
                    let emask: i64 = if ty == Scalar::F32 {
                        0x7F800000
                    } else {
                        0x7FF0000000000000
                    };
                    let v = trunc_n(b, ty, "%p0");
                    let e = b.reg();
                    b.line(&format!("{e} = and {it} {v}, {emask}"));
                    let z = b.reg();
                    b.line(&format!("{z} = icmp ne {it} {e}, 0"));
                    let m = b.reg();
                    b.line(&format!("{m} = icmp ne {it} {e}, {emask}"));
                    let r = b.reg();
                    b.line(&format!("{r} = and i1 {z}, {m}"));
                    r
                }
                FClassOp::SignBit => {
                    // The raw sign bit, so -0 and -NaN are negative.
                    let r = if ty == Scalar::F32 {
                        let s = b.reg();
                        b.line(&format!("{s} = and i64 %p0, 2147483648"));
                        let c = b.reg();
                        b.line(&format!("{c} = icmp ne i64 {s}, 0"));
                        c
                    } else {
                        let c = b.reg();
                        b.line(&format!("{c} = icmp slt i64 %p0, 0"));
                        c
                    };
                    r
                }
            };
            let r = b.reg();
            b.line(&format!("{r} = zext i1 {c} to i64"));
            ret(b, &r);
        }
        TotalOrder { ty } => {
            // IEEE 754-2019 totalOrder: -NaN < -inf < ... < -0 < +0 < ...
            // < +inf < +NaN. Map each word to an order key (negated words
            // flip, non-negated gain the sign bit) and compare unsigned.
            let f32words = ty == Scalar::F32;
            let signmask: i64 = if f32words { 2147483648i64 } else { i64::MIN };
            let key = |b: &mut Builder, v: &str| -> String {
                let n = b.reg();
                if f32words {
                    let s = b.reg();
                    b.line(&format!("{s} = and i64 {v}, {signmask}"));
                    b.line(&format!("{n} = icmp ne i64 {s}, 0"));
                } else {
                    b.line(&format!("{n} = icmp slt i64 {v}, 0"));
                }
                let inv = b.reg();
                b.line(&format!(
                    "{inv} = xor i64 {v}, {}",
                    if f32words { 4294967295i64 } else { -1i64 }
                ));
                let pos = b.reg();
                b.line(&format!("{pos} = or i64 {v}, {signmask}"));
                let k = b.reg();
                b.line(&format!("{k} = select i1 {n}, i64 {inv}, i64 {pos}"));
                k
            };
            let kx = key(b, "%p0");
            let ky = key(b, "%p1");
            let lt = b.reg();
            b.line(&format!("{lt} = icmp ult i64 {kx}, {ky}"));
            let eq = b.reg();
            b.line(&format!("{eq} = icmp eq i64 {kx}, {ky}"));
            let llt = b.label();
            let leq = b.label();
            let lgt = b.label();
            let lsecond = b.label();
            b.line(&format!("br i1 {lt}, label %{llt}, label %{lsecond}"));
            b.start(&lsecond);
            b.line(&format!("br i1 {eq}, label %{leq}, label %{lgt}"));
            for (l, suf, tag) in [(llt, ".lt", 0), (leq, ".eq", 1), (lgt, ".gt", 2)] {
                b.start(&l);
                let w = b.reg();
                b.line(&format!(
                    "{w} = ptrtoint ptr {blk} to i64",
                    blk = static_payload(&format!("{name}{suf}"), reg.shape_desc(&[], tag))
                ));
                ret(b, &w);
            }
        }
        CharToU32 => {
            // A `Char` word already is its `U32` code point.
            ret(b, "%p0");
        }
        U32ToChar => {
            // Rejects surrogates and values above 0x10FFFF (section 6).
            let hi = b.reg();
            b.line(&format!("{hi} = icmp ugt i64 %p0, 1114111"));
            let s1 = b.reg();
            b.line(&format!("{s1} = icmp uge i64 %p0, 55296"));
            let s2 = b.reg();
            b.line(&format!("{s2} = icmp ule i64 %p0, 57343"));
            let sur = b.reg();
            b.line(&format!("{sur} = and i1 {s1}, {s2}"));
            let bad = b.reg();
            b.line(&format!("{bad} = or i1 {hi}, {sur}"));
            ret_option(b, reg, op, name, &bad, |_| "%p0".to_string());
        }
        Conv { from, to, mode } => conv_body(b, reg, op, name, from, to, mode),
    }
}

/// Conversions (primitive types specification, section 7): every result is
/// canonicalized to the target's word form.
fn conv_body(
    b: &mut Builder,
    reg: &Registry,
    op: PrimOp,
    name: &str,
    from: Scalar,
    to: Scalar,
    mode: ConvMode,
) {
    if from.is_int_like() && to.is_int_like() {
        match mode {
            ConvMode::Total => {
                // Value-preserving: the canonical words are identical.
                ret(b, "%p0");
            }
            ConvMode::Wrapping => {
                let c = canon(b, to, "%p0");
                ret(b, &c);
            }
            ConvMode::Checked => {
                let c = canon(b, to, "%p0");
                let bad = b.reg();
                b.line(&format!("{bad} = icmp ne i64 {c}, %p0"));
                ret_option(b, reg, op, name, &bad, |_| c.clone());
            }
            ConvMode::Saturating => {
                let c = canon(b, to, "%p0");
                let fits = b.reg();
                b.line(&format!("{fits} = icmp eq i64 {c}, %p0"));
                // Not fitting means below the target's minimum (a negative
                // value into an unsigned target) or above its maximum.
                let low = if from.is_signed() {
                    let r = b.reg();
                    if to.is_signed() {
                        b.line(&format!("{r} = icmp slt i64 %p0, {}", min_word(to)));
                    } else {
                        b.line(&format!("{r} = icmp slt i64 %p0, 0"));
                    }
                    r
                } else {
                    // An unsigned value is never below the minimum.
                    "false".to_string()
                };
                let bound = if low == "false" {
                    max_word(to).to_string()
                } else {
                    let r = b.reg();
                    b.line(&format!(
                        "{r} = select i1 {low}, i64 {}, i64 {}",
                        min_word(to),
                        max_word(to)
                    ));
                    r
                };
                let out = b.reg();
                b.line(&format!("{out} = select i1 {fits}, i64 {c}, i64 {bound}"));
                ret(b, &out);
            }
        }
        return;
    }
    if from.is_int_like() && to.is_float() {
        // Integer-to-float is total with IEEE rounding.
        let it = int_ty(from);
        let v = trunc_n(b, from, "%p0");
        let inst = if from.is_signed() { "sitofp" } else { "uitofp" };
        let f = b.reg();
        b.line(&format!("{f} = {inst} {it} {v} to {}", float_ty(to)));
        let w = float_to_word(b, to, &f);
        ret(b, &w);
        return;
    }
    if from.is_float() && to.is_float() {
        if from == to {
            ret(b, "%p0");
            return;
        }
        let f = word_to_float(b, from, "%p0");
        let g = b.reg();
        if from == Scalar::F32 {
            b.line(&format!("{g} = fpext float {f} to double"));
        } else {
            b.line(&format!("{g} = fptrunc double {f} to float"));
        }
        let w = float_to_word(b, to, &g);
        ret(b, &w);
        return;
    }
    // Float-to-integer: never total. The value truncates toward zero;
    // `checked` returns `None` outside the target's range and for NaN,
    // `saturating` clamps and maps NaN to zero (section 7).
    let ft = float_ty(from);
    let x = word_to_float(b, from, "%p0");
    let w = to.width();
    // Bounds: floats in `(-1, 2^w)` truncate into unsigned targets and
    // floats in `(min - 1, max + 1)` into signed ones; where `min - 1` is
    // not representable in the source float the closed form is used (the
    // gap around the bound contains no source float).
    let exact_lo = match (from, w) {
        (Scalar::F64, 8 | 16 | 32) | (Scalar::F32, 8 | 16) => true,
        _ => false,
    };
    let (clo, chi) = if to.is_signed() {
        let minf = -(2f64.powi(w as i32 - 1));
        let max1 = 2f64.powi(w as i32 - 1);
        if exact_lo {
            (fcst(from, minf - 1.0), fcst(from, max1))
        } else {
            (fcst(from, minf), fcst(from, max1))
        }
    } else {
        (fcst(from, -1.0), fcst(from, 2f64.powi(w as i32)))
    };
    let lo_pred = if to.is_signed() && !exact_lo {
        "oge"
    } else {
        "ogt"
    };
    let lo = b.reg();
    b.line(&format!("{lo} = fcmp {lo_pred} {ft} {x}, {clo}"));
    let hi = b.reg();
    b.line(&format!("{hi} = fcmp olt {ft} {x}, {chi}"));
    let ok = b.reg();
    b.line(&format!("{ok} = and i1 {lo}, {hi}"));
    let nan = b.reg();
    b.line(&format!("{nan} = fcmp uno {ft} {x}, {x}"));
    let it = int_ty(to);
    let v = b.reg();
    let inst = if to.is_signed() { "fptosi" } else { "fptoui" };
    b.line(&format!("{v} = {inst} {ft} {x} to {it}"));
    let word = ext_n(b, to, &v);
    match mode {
        ConvMode::Checked => {
            let bad = b.reg();
            b.line(&format!("{bad} = xor i1 {ok}, true"));
            ret_option(b, reg, op, name, &bad, |_| word.clone());
        }
        ConvMode::Saturating => {
            // NaN maps to zero; out of range clamps to the bound the value
            // is beyond.
            let low = b.reg();
            b.line(&format!("{low} = fcmp ule {ft} {x}, {clo}"));
            let bound = b.reg();
            b.line(&format!(
                "{bound} = select i1 {low}, i64 {}, i64 {}",
                min_word(to),
                max_word(to)
            ));
            let clamped = b.reg();
            b.line(&format!(
                "{clamped} = select i1 {ok}, i64 {word}, i64 {bound}"
            ));
            let out = b.reg();
            b.line(&format!("{out} = select i1 {nan}, i64 0, i64 {clamped}"));
            ret(b, &out);
        }
        ConvMode::Total | ConvMode::Wrapping => {
            unreachable!("float-to-integer conversions are checked or saturating")
        }
    }
}

fn ctor_body(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    desc: u64,
    fields: usize,
    plan: Option<&crate::core::CtorLayout>,
) {
    if fields == 0 {
        ensure_static_ctor(b, reg, name, desc);
        let w = b.reg();
        b.line(&format!(
            "{w} = ptrtoint ptr {blk} to i64",
            blk = static_payload(&format!("{name}.blk"), desc)
        ));
        b.line(&format!("ret i64 {w}"));
        return;
    }
    // A boundary-layout type (`repr(C)`, `repr(packed)`,
    // `repr(transparent)`): the block payload is the recorded C layout
    // and every field is stored at its recorded offset with its recorded
    // size (attribute annotations specification section 4.3).
    if let Some(plan) = plan {
        let p = b.reg();
        b.line(&ctor_alloc_call(&p, plan.payload, desc));
        for (i, fa) in plan.fields.iter().enumerate() {
            store_field_word(b, reg, &p, fa, &format!("%p{i}"));
        }
        let w = b.reg();
        b.line(&format!("{w} = ptrtoint ptr {p} to i64"));
        b.line(&format!("ret i64 {w}"));
        return;
    }
    let params: Vec<String> = (0..fields).map(|i| format!("%p{i}")).collect();
    let w = ctor_block_word(b, desc, &params);
    b.line(&format!("ret i64 {w}"));
}

// ----------------------------------------------- foreign bindings (P11)
//
// The C ABI marshalling for `foreign import`/`foreign export ccall`
// (foreign function interface specification sections 2-4): machine
// scalars cross by value in their recorded widths and signedness, `Unit`
// is void (its parameters elided), every pointer role and opaque handle
// crosses as `ptr`, and an aggregate parameter crosses as a
// const-pointer to a call-scoped C-layout copy whose fields are read and
// written at the recorded offsets with their recorded sizes. Field
// copies are byte-exact, so a packed field marshals without any padding
// read (attribute annotations specification section 4.2). By-value
// aggregate passing follows a platform coercion ABI this slice does not
// guess: such layouts are rejected at check time instead (section 2:
// reject unsupported layouts instead of guessing).

/// The fault message for a foreign boundary contract violation.
const FFI_NULL_MSG: &str =
    "a foreign boundary contract was violated: null pointer where the binding requires non-null";

/// The LLVM type of one foreign parameter or result's C shape.
fn abi_llvm_ty(p: &AbiParam) -> &'static str {
    match &p.class {
        AbiClass::Unit => "void",
        AbiClass::Scalar(s) => {
            if s.is_float() {
                float_ty(*s)
            } else {
                int_ty(*s)
            }
        }
        AbiClass::Ptr | AbiClass::Aggregate => "ptr",
    }
}

/// Convert one canonical value word to its C scalar shape.
fn word_to_c(b: &mut Builder, s: Scalar, w: &str) -> String {
    if s == Scalar::F32 {
        let i = b.reg();
        b.line(&format!("{i} = trunc i64 {w} to i32"));
        let f = b.reg();
        b.line(&format!("{f} = bitcast i32 {i} to float"));
        f
    } else if s == Scalar::F64 {
        let f = b.reg();
        b.line(&format!("{f} = bitcast i64 {w} to double"));
        f
    } else {
        match s.width() {
            64 => w.to_string(),
            bits => {
                let r = b.reg();
                b.line(&format!("{r} = trunc i64 {w} to i{bits}"));
                r
            }
        }
    }
}

/// Convert one C scalar value to its canonical value word.
fn c_to_word(b: &mut Builder, s: Scalar, v: &str) -> String {
    if s == Scalar::F32 {
        let i = b.reg();
        b.line(&format!("{i} = bitcast float {v} to i32"));
        let w = b.reg();
        b.line(&format!("{w} = zext i32 {i} to i64"));
        w
    } else if s == Scalar::F64 {
        let w = b.reg();
        b.line(&format!("{w} = bitcast double {v} to i64"));
        w
    } else {
        match s.width() {
            64 => v.to_string(),
            bits => {
                let r = b.reg();
                let op = if s.is_signed() { "sext" } else { "zext" };
                b.line(&format!("{r} = {op} i{bits} {v} to i64"));
                r
            }
        }
    }
}

/// Load one stored field's raw value at `base + off`: a scalar at its
/// stored width, a word in one piece. The load states the field type's
/// `!range` (typed loads) and `!align` (borrow loads) facts where they
/// are statable (the plan note's items 2-3), and an untrusted value
/// freezes before it can branch or compare (the plan note's poison
/// containment).
fn load_stored_value(
    b: &mut Builder,
    reg: &Registry,
    base: &str,
    off: u64,
    fa: &FieldAccess,
    freeze: bool,
) -> String {
    let p = b.gep(base, off as usize);
    match &fa.shape {
        FieldShape::Scalar { size, .. } => {
            let bits = (size * 8) as u32;
            let rng = match load_range_meta(reg, bits, fa.range) {
                Some(r) => format!(", !range {r}"),
                None => String::new(),
            };
            let v = b.reg();
            b.line(&format!("{v} = load i{bits}, ptr {p}, align 1{rng}"));
            if freeze {
                freeze_at(b, &v, bits)
            } else {
                v
            }
        }
        FieldShape::Word { align } => {
            let al = if *align > 1 {
                format!(", !align {}", reg.meta_node(format!("!{{i64 {align}}}")))
            } else {
                String::new()
            };
            let v = b.reg();
            b.line(&format!("{v} = load ptr, ptr {p}, align 1{al}"));
            let w = b.reg();
            b.line(&format!("{w} = ptrtoint ptr {v} to i64"));
            if freeze {
                freeze_word(b, &w)
            } else {
                w
            }
        }
        FieldShape::Inline { .. } => {
            unreachable!("a nested aggregate boxes out before its fields load")
        }
    }
}

/// Load one stored field's value word from a block payload at the
/// field's recorded offset (attribute annotations specification
/// section 4.3). A scalar load carries the `!range` fact of its type
/// where the loaded width can state it; a pointer word carries the
/// `!align` fact its type asserts (the plan note's items 2-3).
fn load_field_word(b: &mut Builder, reg: &Registry, payload: &str, fa: &FieldAccess) -> String {
    match &fa.shape {
        FieldShape::Scalar { size: 0, .. } => "0".to_string(),
        FieldShape::Scalar { size, signed } => {
            let v = load_stored_value(b, reg, payload, fa.offset, fa, false);
            if *size == 8 {
                v
            } else {
                let w = b.reg();
                let op = if *signed { "sext" } else { "zext" };
                b.line(&format!("{w} = {op} i{} {v} to i64", size * 8));
                w
            }
        }
        FieldShape::Word { .. } => load_stored_value(b, reg, payload, fa.offset, fa, false),
        FieldShape::Inline { size, tag, fields } => {
            // Box the inline payload out into a fresh block: the nested
            // aggregate has no constructor unit in hand here, so its block
            // takes the anonymous shape of its size, uncounted because its
            // word fields' classes are not recorded.
            let blk = b.reg();
            b.line(&ctor_alloc_call(
                &blk,
                size,
                reg.raw_shape_desc(size.div_ceil(8), *tag as u64),
            ));
            let dst = blk.clone();
            let src = b.gep(payload, fa.offset as usize);
            for f in fields {
                copy_field_exact(b, reg, &dst, 0, &src, 0, f, false);
            }
            let w = b.reg();
            b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
            w
        }
    }
}

/// Store one value word into a block payload field at its recorded
/// offset.
fn store_field_word(b: &mut Builder, reg: &Registry, payload: &str, fa: &FieldAccess, val: &str) {
    match &fa.shape {
        FieldShape::Scalar { size: 0, .. } => {}
        FieldShape::Scalar { size, .. } => {
            let p = b.gep(payload, fa.offset as usize);
            if *size == 8 {
                b.line(&format!("store i64 {val}, ptr {p}, align 1"));
            } else {
                let v = b.reg();
                b.line(&format!("{v} = trunc i64 {val} to i{}", size * 8));
                b.line(&format!("store i{} {v}, ptr {p}, align 1", size * 8));
            }
        }
        FieldShape::Word { .. } => {
            let p = b.gep(payload, fa.offset as usize);
            b.line(&format!("store i64 {val}, ptr {p}, align 1"));
        }
        FieldShape::Inline { fields, .. } => {
            // The value is a nested block: copy its payload in.
            let blk = b.reg();
            b.line(&format!("{blk} = inttoptr i64 {val} to ptr"));
            let src = blk;
            let dst = b.gep(payload, fa.offset as usize);
            for f in fields {
                copy_field_exact(b, reg, &dst, 0, &src, 0, f, false);
            }
        }
    }
}

/// Copy one layout field's stored bytes between a block payload and a
/// C-layout buffer, byte-exact at the recorded offsets and sizes:
/// padding is never read or written, so a packed field marshals without
/// padding reads (attribute annotations specification section 4.2).
/// `freeze` marks an untrusted source (a foreign buffer): the loaded
/// bytes freeze before they enter the safe side (the plan note's poison
/// containment).
fn copy_field_exact(
    b: &mut Builder,
    reg: &Registry,
    dst: &str,
    dst_off: u64,
    src: &str,
    src_off: u64,
    fa: &FieldAccess,
    freeze: bool,
) {
    let size = match &fa.shape {
        FieldShape::Scalar { size: 0, .. } => return,
        FieldShape::Scalar { size, .. } => *size,
        FieldShape::Word { .. } => {
            let w = load_stored_value(b, reg, src, src_off + fa.offset, fa, freeze);
            let dp = b.gep(dst, (dst_off + fa.offset) as usize);
            b.line(&format!("store i64 {w}, ptr {dp}, align 1"));
            return;
        }
        FieldShape::Inline { fields, .. } => {
            for f in fields {
                copy_field_exact(
                    b,
                    reg,
                    dst,
                    dst_off + fa.offset,
                    src,
                    src_off + fa.offset,
                    f,
                    freeze,
                );
            }
            return;
        }
    };
    let v = load_stored_value(b, reg, src, src_off + fa.offset, fa, freeze);
    let dp = b.gep(dst, (dst_off + fa.offset) as usize);
    b.line(&format!("store i{} {v}, ptr {dp}, align 1", size * 8));
}

/// Branch to the unit's fault block when `v` is null: a non-null
/// boundary pointer crossing as null is a contract violation, handled by
/// the runtime fault path (the no-unwind rule of foreign function
/// interface specification section 4).
fn check_non_null(b: &mut Builder, v: &str, fault: &str) {
    let chk = b.reg();
    b.line(&format!("{chk} = icmp eq ptr {v}, null"));
    let cont = b.label();
    b.line(&format!("br i1 {chk}, label %{fault}, label %{cont}"));
    b.start(&cont);
}

/// Append one unit's shared fault block.
fn ffi_fault_block(b: &mut Builder, fault: &str) {
    b.start(fault);
    b.line(&format!(
        "call void @cinch_panic(ptr @__cinch_ffi_nn, i64 {})",
        FFI_NULL_MSG.len()
    ));
    b.line("unreachable");
}

/// The `declare` line of one `uses` contract: the C ABI the contract
/// states, in the machine scalar spelling the fragment calls it with
/// (`void` where the contract returns `Unit`).
fn prim_use_declare(u: &crate::core::PrimUseContract) -> String {
    let params = u
        .args
        .iter()
        .map(abi_word)
        .collect::<Vec<_>>()
        .join(", ");
    let ret = match &u.result {
        Some(t) => abi_word(t),
        None => "void".to_string(),
    };
    format!("declare {ret} @{}({params})\n", quote(&u.symbol))
}

/// One `declare` per `uses` symbol in the program, deduped in program
/// order. A symbol the module already declares or defines is skipped: the
/// runtime preamble's helpers, a foreign binding's symbol and another
/// contract's symbol are one declaration each.
fn prim_use_declares(program: &Program, so_far: &str) -> Vec<String> {
    let mut seen: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut lines = Vec::new();
    for p in &program.prims {
        for b in &p.bodies {
            for u in &b.uses {
                if !seen.insert(u.symbol.as_str()) {
                    continue;
                }
                // The preamble spells its runtime declarations bare
                // (`@cinch_dup(`), a contract's own declaration quoted.
                let quoted = format!("@{}(", quote(&u.symbol));
                let bare = format!("@{}(", u.symbol);
                if so_far.contains(&quoted) || so_far.contains(&bare) {
                    continue;
                }
                lines.push(prim_use_declare(u));
            }
        }
    }
    lines
}

/// The native spelling of one by-value aggregate crossing (the plan
/// note's item 6): `sret(<layout>)` for the result-out buffer, and
/// `byref(<layout>)` for the parameter's call-scoped copy. LLVM's
/// `byval` is not that spelling on every platform: it can mean a
/// machine convention (a stack-passed copy) that does not match the C
/// side's by-value aggregates, while `byref` states the pointee type
/// and the pointer's validity without changing the passing convention
/// (the tests/llvm_contract.rs round trip against real C).
fn byvalue_attr(p: &AbiParam, reg: &Registry, result: bool) -> String {
    if !p.by_value {
        return String::new();
    }
    let name = reg
        .layout_types
        .get(&p.width)
        .expect("the aggregate's layout type is registered");
    if result {
        format!("sret(%{name}) align {}", p.align)
    } else {
        format!("byref(%{name}) align {}", p.align)
    }
}

/// One parameter's declaration attributes at the C boundary (the plan
/// note's usage-derived parameter attributes): `captures(none)` for the
/// call-scoped borrow (P5 non-escape: a retained or raw pointer may
/// escape and is not claimed), `nonnull` for the non-null contract
/// (P11's default), `align` from the role's pointee assertion or the
/// aggregate's recorded layout, and `noundef` for scalars. At an export
/// entry (`export`) the pointer side is untrusted: its nullness stays a
/// checked fault, never a `nonnull` promise.
fn param_attrs(p: &AbiParam, export: bool) -> String {
    let mut attrs: Vec<String> = Vec::new();
    match &p.class {
        AbiClass::Unit => {}
        AbiClass::Scalar(_) => {
            if !export {
                attrs.push("noundef".to_string());
            }
        }
        AbiClass::Ptr => {
            if p.role == "borrowed" {
                attrs.push("captures(none)".to_string());
            }
            if !p.nullable && !export {
                attrs.push("nonnull".to_string());
            }
            if p.pointee_align > 1 {
                attrs.push(format!("align {}", p.pointee_align));
            }
        }
        AbiClass::Aggregate => {
            if !p.nullable && !export {
                attrs.push("nonnull".to_string());
            }
            if p.align > 1 {
                attrs.push(format!("align {}", p.align));
            }
        }
    }
    if attrs.is_empty() {
        String::new()
    } else {
        format!(" {}", attrs.join(" "))
    }
}

/// The `declare` for one imported foreign symbol.
fn foreign_declare(plan: &ForeignPlan, reg: &Registry) -> String {
    if let Some(call) = &plan.canon {
        // The canonical ABI's core signature (WebAssembly and WASI
        // specification section 4).
        let (ret, params) = canon_sig(call);
        return format!(
            "declare {ret} @{}({})\n",
            quote(&plan.symbol),
            params.join(", ")
        );
    }
    let mut params: Vec<String> = Vec::new();
    // A by-value aggregate result crosses first, through the
    // platform's result-out buffer (`sret`).
    if matches!(plan.result.class, AbiClass::Aggregate) {
        params.push(format!("ptr {}", byvalue_attr(&plan.result, reg, true)));
    }
    for p in plan
        .params
        .iter()
        .filter(|p| !matches!(p.class, AbiClass::Unit))
    {
        let byval = byvalue_attr(p, reg, false);
        let kind = if byval.is_empty() {
            "ptr".to_string()
        } else {
            format!("ptr {byval}")
        };
        params.push(format!("{kind}{}", param_attrs(p, false)));
    }
    let ret = if matches!(plan.result.class, AbiClass::Aggregate) {
        "void".to_string()
    } else {
        abi_llvm_ty(&plan.result).to_string()
    };
    format!(
        "declare {ret} @{}({})\n",
        quote(&plan.symbol),
        params.join(", ")
    )
}

/// Restate one boundary buffer's recorded layout as an `llvm.assume`
/// with operand bundles (the plan note's item 4): the `repr` facts of
/// attribute annotations specification section 4.3, machine-readably.
fn assume_layout(b: &mut Builder, ptr: &str, align: u64, size: u64) {
    let mut bundles: Vec<String> = Vec::new();
    if align > 1 {
        bundles.push(format!("\"align\"(ptr {ptr}, i64 {align})"));
    }
    if size > 0 {
        bundles.push(format!("\"dereferenceable\"(ptr {ptr}, i64 {size})"));
    }
    if bundles.is_empty() {
        return;
    }
    b.line(&format!(
        "call void @llvm.assume(i1 true) [ {} ]",
        bundles.join(", ")
    ));
}

/// The checked boundary pointer's facts as an `llvm.assume`: the
/// non-null contract (foreign function interface specification
/// decision D70) and the role's pointee alignment, after the boundary
/// check has faulted on a violation.
fn assume_ptr(b: &mut Builder, ptr: &str, p: &AbiParam) {
    let mut bundles: Vec<String> = vec![format!("\"nonnull\"(ptr {ptr})")];
    if p.pointee_align > 1 {
        bundles.push(format!("\"align\"(ptr {ptr}, i64 {})", p.pointee_align));
    }
    b.line(&format!(
        "call void @llvm.assume(i1 true) [ {} ]",
        bundles.join(", ")
    ));
}

/// Emit `call`, the call to a foreign import's symbol. An import whose
/// row carries `Blocking` may block its thread in the operating system,
/// so on the native task backend its call is bracketed as a blocking
/// region of the runtime's scheduler (`compiler/runtime/src/sched.rs`):
/// the other tasks keep running on another thread meanwhile (concurrency
/// specification section 9). The cooperative backend has one thread and
/// no scheduler to tell.
fn foreign_call(b: &mut Builder, cx: &LowerCx, plan: &ForeignPlan, call: &str) {
    let region = plan.blocking && !cx.coop_tasks;
    if region {
        b.line("call i64 @cinch_sched_blocking_enter()");
    }
    b.line(call);
    if region {
        b.line("call i64 @cinch_sched_blocking_exit()");
    }
}

/// The synthesized wrapper for one foreign import: convert the canonical
/// value words to the C ABI shape, call the symbol, and convert the
/// result back (foreign function interface specification section 2).
/// Every word read back at the boundary freezes before it can branch or
/// compare (the plan note's poison containment).
fn foreign_import_body(
    b: &mut Builder,
    cx: &LowerCx,
    reg: &Registry,
    plan: &ForeignPlan,
) -> Result<()> {
    if plan.canon.is_some() {
        // A component adapter's canonical ABI is defined over wasm32
        // core values (WebAssembly and WASI specification section 4):
        // the glue emits for a Wasm artifact profile.
        if cx.usize_bits != 32 {
            return err(format!(
                "the canonical ABI glue for {} is defined for the wasm32 target (WebAssembly and WASI specification section 4); emit under a Wasm artifact profile",
                plan.symbol
            ));
        }
        return canon_import_body(b, reg, plan);
    }
    let fault = b.label();
    let mut any_check = false;
    let mut tys: Vec<String> = Vec::new();
    let mut args: Vec<String> = Vec::new();
    for (i, p) in plan.params.iter().enumerate() {
        match &p.class {
            AbiClass::Unit => {}
            AbiClass::Scalar(s) => {
                tys.push(format!("{}{}", abi_llvm_ty(p), param_attrs(p, false)));
                args.push(word_to_c(b, *s, &format!("%p{i}")));
            }
            AbiClass::Ptr => {
                let v = b.reg();
                b.line(&format!("{v} = inttoptr i64 %p{i} to ptr"));
                if !p.nullable {
                    check_non_null(b, &v, &fault);
                    any_check = true;
                    assume_ptr(b, &v, p);
                }
                tys.push(format!("ptr{}", param_attrs(p, false)));
                args.push(v);
            }
            AbiClass::Aggregate => {
                // A call-scoped C-layout copy of the block payload,
                // laid out and aligned as the recorded layout says.
                let buf = b.alloca_bytes(p.width, p.align.max(1));
                b.line(&format!(
                    "call void @llvm.memset.p0.i64(ptr {buf}, i8 0, i64 {}, i1 false)",
                    p.width
                ));
                assume_layout(b, &buf, p.align, p.width);
                let blk = b.reg();
                b.line(&format!("{blk} = inttoptr i64 %p{i} to ptr"));
                let payload = blk.clone();
                for fa in &p.fields {
                    copy_field_exact(b, reg, &buf, 0, &payload, 0, fa, false);
                }
                let byval = byvalue_attr(p, reg, false);
                let spelled = if byval.is_empty() {
                    format!("ptr{}", param_attrs(p, false))
                } else {
                    format!("ptr {byval}{}", param_attrs(p, false))
                };
                tys.push(spelled);
                args.push(buf);
            }
        }
    }
    let call_args: Vec<String> = tys
        .iter()
        .zip(args.iter())
        .map(|(t, a)| format!("{t} {a}"))
        .collect();
    match &plan.result.class {
        AbiClass::Unit => {
            foreign_call(
                b,
                cx,
                plan,
                &format!("call void @{}({})", quote(&plan.symbol), call_args.join(", ")),
            );
            b.line("ret i64 0");
        }
        AbiClass::Scalar(s) => {
            let rty = abi_llvm_ty(&plan.result);
            let r = b.reg();
            foreign_call(
                b,
                cx,
                plan,
                &format!("{r} = call {rty} @{}({})", quote(&plan.symbol), call_args.join(", ")),
            );
            let w = c_to_word(b, *s, &r);
            let w = freeze_word(b, &w);
            b.line(&format!("ret i64 {w}"));
        }
        AbiClass::Ptr => {
            let r = b.reg();
            foreign_call(
                b,
                cx,
                plan,
                &format!("{r} = call ptr @{}({})", quote(&plan.symbol), call_args.join(", ")),
            );
            if !plan.result.nullable {
                check_non_null(b, &r, &fault);
                any_check = true;
                assume_ptr(b, &r, &plan.result);
            }
            let w = b.reg();
            b.line(&format!("{w} = ptrtoint ptr {r} to i64"));
            let w = freeze_word(b, &w);
            b.line(&format!("ret i64 {w}"));
        }
        AbiClass::Aggregate => {
            // The by-value result crosses through the platform's
            // result-out buffer (`sret`); the C-layout bytes box into a
            // boundary-layout block on the way in.
            let sret = byvalue_attr(&plan.result, reg, true);
            let buf = b.alloca_bytes(plan.result.width, plan.result.align.max(1));
            let mut full = vec![format!("ptr {sret} {buf}")];
            full.extend(call_args);
            foreign_call(
                b,
                cx,
                plan,
                &format!("call void @{}({})", quote(&plan.symbol), full.join(", ")),
            );
            // The block holds C bytes: no word in it is a reference this
            // program owns, so none is counted.
            let blk = b.reg();
            b.line(&ctor_alloc_call(
                &blk,
                plan.result.width,
                reg.raw_shape_desc(plan.result.width.div_ceil(8), plan.result.tag as u64),
            ));
            let payload = blk.clone();
            for fa in &plan.result.fields {
                copy_field_exact(b, reg, &payload, 0, &buf, 0, fa, true);
            }
            let w = b.reg();
            b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
            ret(b, &w);
        }
    }
    if any_check {
        ffi_fault_block(b, &fault);
    }
    Ok(())
}

/// The C-callable entry for one foreign export (foreign function
/// interface specification section 2): convert the C ABI arguments to
/// canonical value words, call the target unit, convert the result back.
fn export_entry(out: &mut String, plan: &ForeignPlan, reg: &Registry, cx: &LowerCx) -> Result<()> {
    if plan.canon.is_some() {
        // As in [`foreign_import_body`]: the canonical ABI is the
        // wasm32 shape (WebAssembly and WASI specification section 4).
        if cx.usize_bits != 32 {
            return err(format!(
                "the canonical ABI trampoline for {} is defined for the wasm32 target (WebAssembly and WASI specification section 4); emit under a Wasm artifact profile",
                plan.symbol
            ));
        }
        return canon_export_entry(out, plan, reg);
    }
    let mut b = Builder::new();
    let fault = b.label();
    let mut any_check = false;
    let mut params: Vec<String> = Vec::new();
    let mut words: Vec<String> = Vec::new();
    // A by-value aggregate result crosses first, through the C
    // caller's result-out buffer (`sret`).
    let sret = matches!(plan.result.class, AbiClass::Aggregate);
    if sret {
        params.push(format!(
            "ptr {} %sret",
            byvalue_attr(&plan.result, reg, true)
        ));
    }
    for (i, p) in plan.params.iter().enumerate() {
        let n = format!("%c{i}");
        match &p.class {
            AbiClass::Unit => {}
            AbiClass::Scalar(s) => {
                params.push(format!("{} {n}{}", abi_llvm_ty(p), param_attrs(p, true)));
                // The C side's argument is foreign input (an `undef`
                // C register would otherwise leak into safe code): the
                // word freezes before it can branch or compare.
                let w = c_to_word(&mut b, *s, &n);
                words.push(freeze_word(&mut b, &w));
            }
            AbiClass::Ptr => {
                params.push(format!("ptr {n}{}", param_attrs(p, true)));
                if !p.nullable {
                    // The pointer side is untrusted: a null crossing is
                    // the checked fault, never a `nonnull` promise.
                    check_non_null(&mut b, &n, &fault);
                    any_check = true;
                    assume_ptr(&mut b, &n, p);
                }
                let w = b.reg();
                b.line(&format!("{w} = ptrtoint ptr {n} to i64"));
                let w = freeze_word(&mut b, &w);
                words.push(w);
            }
            AbiClass::Aggregate => {
                // The C side passes a pointer to a C-layout struct: box
                // it into a boundary-layout block at the recorded
                // offsets. The bytes are untrusted and freeze on the
                // way in.
                let byval = byvalue_attr(p, reg, false);
                params.push(if byval.is_empty() {
                    format!("ptr {n}{}", param_attrs(p, true))
                } else {
                    format!("ptr {byval} {n}{}", param_attrs(p, true))
                });
                let blk = b.reg();
                b.line(&ctor_alloc_call(
                    &blk,
                    p.width,
                    reg.raw_shape_desc(p.width.div_ceil(8), p.tag as u64),
                ));
                let payload = blk.clone();
                for fa in &p.fields {
                    copy_field_exact(&mut b, reg, &payload, 0, &n, 0, fa, true);
                }
                let w = b.reg();
                b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
                words.push(w);
            }
        }
    }
    let r = call_unit(&mut b, reg, &plan.unit, &words);
    let ret_ty = if sret {
        "void".to_string()
    } else {
        abi_llvm_ty(&plan.result).to_string()
    };
    let header = format!(
        "define {} @{}({})",
        ret_ty,
        quote(&plan.symbol),
        params.join(", ")
    );
    match &plan.result.class {
        AbiClass::Unit => b.line("ret void"),
        AbiClass::Scalar(s) => {
            let v = word_to_c(&mut b, *s, &r);
            b.line(&format!("ret {} {v}", abi_llvm_ty(&plan.result)));
        }
        AbiClass::Ptr => {
            let p = b.reg();
            b.line(&format!("{p} = inttoptr i64 {r} to ptr"));
            if !plan.result.nullable {
                check_non_null(&mut b, &p, &fault);
                any_check = true;
            }
            b.line(&format!("ret ptr {p}"));
        }
        AbiClass::Aggregate => {
            // The result-out buffer: copy the boxed result's C-layout
            // bytes into the caller's `sret` slot.
            let blk = b.reg();
            b.line(&format!("{blk} = inttoptr i64 {r} to ptr"));
            let payload = blk.clone();
            for fa in &plan.result.fields {
                copy_field_exact(&mut b, reg, "%sret", 0, &payload, 0, fa, false);
            }
            b.line("ret void");
        }
    }
    if any_check {
        ffi_fault_block(&mut b, &fault);
    }
    out.push_str(&b.finish(&header));
    Ok(())
}

/// Body of `@"<id>.mk"`: build `{ i64 nmeth; i64 meth0; ... }` where each
/// method word is the method function's static closure (when the instance
/// has no subdicts) or a fresh closure filled with the subdict parameters.
fn mk_body(b: &mut Builder, reg: &Registry, info: &InstInfo) {
    let uses = info.methods.len();
    for j in 0..info.subdict_count {
        let word = format!("%p{j}");
        let class = info
            .methods
            .first()
            .and_then(|(method, _)| reg.unit_params.get(method))
            .and_then(|params| params.get(info.values.first().map_or(0, Vec::len) + j))
            .copied()
            .unwrap_or(Class::Obj);
        if uses == 0 {
            rc_op(b, reg.rc_debug, true, &word, class);
        }
        for _ in 1..uses {
            rc_op(b, reg.rc_debug, false, &word, class);
        }
    }
    let pointer = b.reg();
    b.line(&shaped_alloc_call(
        &pointer,
        (1 + info.method_count) * 8,
        block::KIND_DICT,
        reg.dict_desc(info),
    ));
    b.store_at(&pointer, 0, &info.method_count.to_string());
    for i in 0..info.method_count {
        b.store_at(&pointer, (1 + i) * 8, "0");
    }
    let strict: Vec<bool> = info
        .methods
        .iter()
        .enumerate()
        .map(|(i, (_, arity))| *arity == info.subdict_count + info.values[i].len())
        .collect();
    let mut pending: Vec<usize> = strict
        .iter()
        .enumerate()
        .filter_map(|(i, strict)| strict.then_some(i))
        .collect();
    let mut initialized = std::collections::HashSet::new();
    let mut order = Vec::new();
    while !pending.is_empty() {
        let index = pending
            .iter()
            .position(|i| {
                info.values[*i]
                    .iter()
                    .all(|slot| initialized.contains(slot))
            })
            .expect("dictionary initialization cycles were rejected before code generation");
        let slot = pending.remove(index);
        order.push(slot);
        initialized.insert(slot);
    }
    order.extend(
        strict
            .iter()
            .enumerate()
            .filter_map(|(i, strict)| (!strict).then_some(i)),
    );
    for i in order {
        let (method, arity) = &info.methods[i];
        let mut saved = Vec::new();
        for slot in &info.values[i] {
            let value = b.load_at(&pointer, (1 + slot) * 8);
            rc_op(b, reg.rc_debug, false, &value, info.fields[*slot]);
            saved.push(value);
        }
        saved.extend((0..info.subdict_count).map(|j| format!("%p{j}")));
        let word = if strict[i] {
            call_unit(b, reg, method, &saved)
        } else if saved.is_empty() {
            let value = b.reg();
            b.line(&format!(
                "{value} = ptrtoint ptr {} to i64",
                static_payload(&format!("{method}.clo"), reg.closure_desc(method))
            ));
            value
        } else {
            let closure = b.reg();
            b.line(&shaped_alloc_call(
                &closure,
                (3 + saved.len()) * 8,
                block::KIND_CLOSURE,
                reg.closure_desc(method),
            ));
            let code = b.reg();
            b.line(&format!(
                "{code} = ptrtoint ptr @{} to i64",
                quote(&format!("{method}.app"))
            ));
            b.store_at(&closure, 0, &code);
            b.store_at(&closure, 8, &arity.to_string());
            b.store_at(&closure, 16, &saved.len().to_string());
            for (j, value) in saved.iter().enumerate() {
                b.store_at(&closure, 24 + 8 * j, value);
            }
            let value = b.reg();
            b.line(&format!("{value} = ptrtoint ptr {closure} to i64"));
            value
        };
        b.store_at(&pointer, (1 + i) * 8, &word);
    }
    let word = b.reg();
    b.line(&format!("{word} = ptrtoint ptr {pointer} to i64"));
    b.line(&format!("ret i64 {word}"));
}

/// widened worker (the trailing parameters unused).
fn core_body(b: &mut Builder, reg: &Registry, f: &FnDef, width: usize) -> Result<()> {
    let mut bound: Vec<LocalId> = (0..f.arity()).map(LocalId).collect();
    collect_binders(&f.body, &mut bound);
    // The registry's answer, never the contract's directly: a widened
    // unit's worker runs this same body under a word-typed header, and
    // the registry is what leaves it out.
    let narrow = reg.narrow_sigs.get(&f.id.0);
    let cross = |i: usize| narrow.map_or(Cross::WORD, |s| s.param(i));
    let mut e = CoreEmitter {
        b,
        reg,
        unit: f.id.0.clone(),
        width,
        ret: narrow.map_or(Cross::WORD, |s| s.ret),
        loop_label: String::new(),
        loop_meta: crate::loopmeta::LoopMeta::new(f.hints.unroll),
        bound: bound.into_iter().collect(),
        slots: HashMap::new(),
        joins: HashMap::new(),
        frame: reg.frame.get(&f.id.0),
        frame_bases: HashMap::new(),
    };
    // A narrowed parameter arrives at its own width and the body reads a
    // slot word, so the entry widens it once (design/narrow-value-
    // representation.md section 4: the compiler boxes at the expanding
    // edge, and everything below the edge keeps the word it always had).
    // The extension direction is the one the word convention already
    // uses for that type, so the slot holds the same value the body would
    // have seen had the position never narrowed: a signed machine scalar
    // and a `csize` `Int` sign-extend (`-2 :: I8` is `-2` in the slot,
    // not `254`), while an unsigned scalar, a float's bits, a pointer and
    // a `Nat` zero-extend.
    for i in 0..f.arity() {
        let slot = e.slot(LocalId(i));
        let w = cross(i).widen(e.b, &format!("%p{i}"));
        e.b.line(&format!("store i64 {w}, ptr {slot}"));
    }
    // P20's `#[checked-params]` row re-states as `llvm.assume` operand
    // bundles at the entry (design/llvm-contract-attributes.md item 4):
    // a validated parameter is a defined value, and a pointer
    // parameter's role contract (`nonnull`, `align`,
    // `dereferenceable`) holds. Without the row there is no assume.
    if f.meta.contract.checked_params {
        for (i, p) in f.meta.contract.params.iter().enumerate() {
            if p.noundef {
                e.b.line(&format!(
                    "call void @llvm.assume(i1 true) [ \"noundef\"({} %p{i}) ]",
                    cross(i).ty()
                ));
            }
            if p.nonnull || p.pointee_align > 1 || p.pointee_size > 0 {
                let v = e.b.reg();
                e.b.line(&format!("{v} = inttoptr i64 %p{i} to ptr"));
                let mut bundles: Vec<String> = Vec::new();
                if p.nonnull {
                    bundles.push(format!("\"nonnull\"(ptr {v})"));
                }
                if p.pointee_align > 1 {
                    bundles.push(format!("\"align\"(ptr {v}, i64 {})", p.pointee_align));
                }
                if p.pointee_size > 0 {
                    bundles.push(format!(
                        "\"dereferenceable\"(ptr {v}, i64 {})",
                        p.pointee_size
                    ));
                }
                e.b.line(&format!(
                    "call void @llvm.assume(i1 true) [ {} ]",
                    bundles.join(", ")
                ));
            }
        }
    }
    // Tail recursion runs in constant stack (surface grammar local
    // amendments section 9; decision D104): the body runs under a loop
    // header after the entry block's parameter spills and allocas, so a
    // saturated self tail call stores its arguments over the parameter
    // slots and jumps here instead of growing the stack.
    let loop_label = e.b.label();
    e.b.line(&format!("br label %{loop_label}"));
    e.b.start(&loop_label);
    e.loop_label = loop_label;
    e.emit_tail(&f.body)
}

/// The primitive operations mentioned in a body.
fn collect_prims(e: &Core, out: &mut BTreeSet<PrimOp>) {
    match e {
        Core::Int(_)
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::Var(_)
        | Core::Global(..)
        | Core::Ctor(..)
        | Core::Foreign(_)
        | Core::SpliceVal(_)
        | Core::Fault
        | Core::LazyAlloc
        | Core::BoxAlloc
        | Core::DeclsQuote { .. }
        | Core::SyntaxLit(..)
        | Core::StrLit(_) => {}
        // A reified quote body calls primitive units through their
        // closure constants; collect them so the units are emitted
        // beside the module (supercompilation and JIT specification
        // section 7).
        Core::Quote { holes, body, .. } => {
            for h in holes {
                collect_prims(h, out);
            }
            if let Some(b) = body {
                collect_prims(b, out);
            }
        }
        Core::Prim(op) => {
            out.insert(*op);
        }
        Core::DictProj(d, ..) | Core::Proj(d, _) | Core::Demand(d, _) => collect_prims(d, out),
        Core::Dup(_, _, body) | Core::Drop(_, _, body) | Core::Release { body, .. } => collect_prims(body, out),
        Core::Reuse { args, .. } => {
            for a in args {
                collect_prims(a, out);
            }
        }
        Core::InstDict(_, args, _) | Core::App(_, args) | Core::Lam { env: args, .. } => {
            if let Core::App(f, _) = e {
                collect_prims(f, out);
            }
            for a in args {
                collect_prims(a, out);
            }
        }
        Core::Let(_, _, v, body) => {
            collect_prims(v, out);
            collect_prims(body, out);
        }
        Core::Join { body, rest, .. } => {
            collect_prims(body, out);
            collect_prims(rest, out);
        }
        Core::Jump { args, .. } => {
            for a in args {
                collect_prims(a, out);
            }
        }
        Core::If(c, t, f) => {
            collect_prims(c, out);
            collect_prims(&t.body, out);
            collect_prims(&f.body, out);
        }
        Core::LazyInit(_, t) => collect_prims(t, out),
        Core::BoxInit(_, v, _) => collect_prims(v, out),
        Core::RefSet(_, v, _) => collect_prims(v, out),
        Core::BoxGet(v, _) | Core::BoxClear(v, _) => collect_prims(v, out),
        Core::Case { scrut, alts } => {
            collect_prims(scrut, out);
            for a in alts {
                collect_prims(&a.body, out);
            }
        }
    }
}

fn collect_binders(e: &Core, out: &mut Vec<LocalId>) {
    match e {
        Core::Int(_)
        | Core::ConstBig { .. }
        | Core::ConstBytes(_)
        | Core::ConstString { .. }
        | Core::ConstSym(_)
        | Core::Bool(_)
        | Core::Unit
        | Core::Var(_)
        | Core::Global(..)
        | Core::Ctor(..)
        | Core::Prim(_)
        | Core::Foreign(_)
        | Core::SpliceVal(_)
        | Core::Fault
        | Core::Quote { .. }
        | Core::DeclsQuote { .. }
        | Core::SyntaxLit(..)
        | Core::StrLit(_) => {}
        Core::DictProj(d, ..) => collect_binders(d, out),
        Core::Proj(d, _) => collect_binders(d, out),
        Core::InstDict(_, subs, _) | Core::Lam { env: subs, .. } => {
            for s in subs {
                collect_binders(s, out);
            }
        }
        Core::App(f, args) => {
            collect_binders(f, out);
            for a in args {
                collect_binders(a, out);
            }
        }
        Core::Demand(d, _) => collect_binders(d, out),
        Core::Dup(_, _, body) | Core::Drop(_, _, body) => collect_binders(body, out),
        Core::Release { body, token, .. } => {
            if let Some(t) = token {
                out.push(*t);
            }
            collect_binders(body, out);
        }
        Core::Reuse { args, .. } => {
            for a in args {
                collect_binders(a, out);
            }
        }
        Core::LazyAlloc => {}
        Core::LazyInit(_, t) => collect_binders(t, out),
        Core::BoxAlloc => {}
        Core::BoxInit(l, v, _) => {
            out.push(*l);
            collect_binders(v, out);
        }
        Core::RefSet(l, v, _) => {
            out.push(*l);
            collect_binders(v, out);
        }
        Core::BoxGet(v, _) | Core::BoxClear(v, _) => collect_binders(v, out),
        Core::Let(x, _, e, body) => {
            out.push(*x);
            collect_binders(e, out);
            collect_binders(body, out);
        }
        Core::Join { params, body, rest, .. } => {
            for (x, _) in params {
                out.push(*x);
            }
            collect_binders(body, out);
            collect_binders(rest, out);
        }
        Core::Jump { args, .. } => {
            for a in args {
                collect_binders(a, out);
            }
        }
        Core::If(c, t, e) => {
            collect_binders(c, out);
            collect_binders(&t.body, out);
            collect_binders(&e.body, out);
        }
        Core::Case { scrut, alts } => {
            collect_binders(scrut, out);
            for a in alts {
                match &a.pat {
                    CPat::Var(x, _) => out.push(*x),
                    CPat::Wild => {}
                    CPat::Con { fields, .. } => out.extend(fields.iter().copied()),
                }
                collect_binders(&a.body, out);
            }
        }
    }
}

struct CoreEmitter<'a, 'b> {
    b: &'b mut Builder,
    reg: &'a Registry,
    /// The unit's name, for unique private constant names.
    unit: String,
    /// The unit's parameter count (its arity, or the group width of a
    /// widened worker): the tail-call rules compare callee tail
    /// signatures against it (decision D104).
    width: usize,
    /// The width the unit's result crosses at: `Dest::Ret` narrows the
    /// word to it (design/narrow-value-representation.md section 4).
    ret: Cross,
    /// The loop header a saturated self tail call jumps to (decision
    /// D104).
    loop_label: String,
    /// The `#[unroll(N)]` hint's metadata, for the back edges into
    /// `loop_label` (`crate::loopmeta`).
    loop_meta: crate::loopmeta::LoopMeta,
    bound: HashSet<LocalId>,
    slots: HashMap<LocalId, String>,
    /// The joins in scope, by id. A `Jump` branches to its join's block and
    /// stores its arguments in the parameter slots; the join's block reads
    /// them (the slots are entry allocas, so a jump from any depth is the
    /// same store).
    joins: HashMap<JoinId, JoinSite>,
    /// The unit's frame allocation decision (`crate::stack`): the blocks
    /// this unit builds in its own frame instead of the heap.
    frame: Option<&'a crate::stack::Plan>,
    /// The `alloca` of each frame slot, by slot id: one entry block slot
    /// per allocation site, reused when a site executes again.
    frame_bases: HashMap<u32, String>,
}

/// A join's code generation site: the block its body starts at, and the
/// slot of each parameter.
#[derive(Clone)]
struct JoinSite {
    label: String,
    slots: Vec<String>,
}

/// Where a value-position expression delivers its result: the function's
/// return (`Ret`, which is the tail position) or a slot plus the block to
/// branch to after storing `Slot`. Destination passing is what makes a
/// `Jump` legal in a value position: the jump branches to its join's
/// block, and the join's body delivers the value to the destination the
/// jump's context would have used. Every jump to one join shares that
/// destination, which is the continuation of the `Join` node.
#[derive(Clone)]
enum Dest {
    Ret,
    Slot { slot: String, label: String },
}

impl Dest {
    /// Store `v` and leave for the destination. The destination of a
    /// value that was never produced is not reached: a terminating
    /// expression leaves the block closed and the caller skips this.
    fn settle(&self, b: &mut Builder, v: &str, ret: Cross) {
        match self {
            Dest::Ret => {
                // The body computes a word; a narrowed signature hands
                // back the low bits the type is bounded to.
                let w = ret.narrow(b, v);
                b.line(&format!("ret {} {w}", ret.ty()));
            }
            Dest::Slot { slot, label } => {
                b.line(&format!("store i64 {v}, ptr {slot}"));
                b.line(&format!("br label %{label}"));
            }
        }
    }
}

impl CoreEmitter<'_, '_> {
    fn slot(&mut self, x: LocalId) -> String {
        if let Some(s) = self.slots.get(&x) {
            return s.clone();
        }
        let s = self.b.alloca_i64();
        self.slots.insert(x, s.clone());
        s
    }

    /// The first-class value of a code generation unit: its static closure
    /// word, or for a zero-arity unit (a thunk) the result of calling it.
    fn unit_value(&mut self, name: &str) -> Result<String> {
        let arity = self.reg.arity_of(name)?;
        if arity == 0 {
            return Ok(call_unit(self.b, self.reg, name, &[]));
        }
        let r = self.b.reg();
        self.b.line(&format!(
            "{r} = ptrtoint ptr {clo} to i64",
            clo = static_payload(&format!("{name}.clo"), self.reg.closure_desc(name))
        ));
        Ok(r)
    }

    /// If `f` names a code generation unit, its name and arity.
    /// The payload of `@"U.blk"`, the value of a zero-field constructor and
    /// the entire body of its unit. The block is a constant, so a use may
    /// come from a unit emitted without the constructor's own unit (an
    /// object build selects roots): the definition is emitted here on first
    /// use, and the constructor's own unit skips it when it is already
    /// there.
    fn ctor_block(&mut self, unit: &str) -> Result<String> {
        let symbol = format!("{unit}.blk");
        let desc = self.reg.ctor_desc(unit)?;
        if self.reg.emitted_blocks.borrow_mut().insert(symbol.clone()) {
            let global = static_ctor_block(self.reg.pointer_bytes, &symbol, desc, &[]);
            self.b.global(global);
        }
        let r = self.b.reg();
        self.b.line(&format!("{r} = ptrtoint ptr {} to i64", static_payload(&symbol, desc)));
        Ok(r)
    }

    fn known_unit(&self, f: &Core) -> Option<(String, usize)> {
        match f {
            Core::Global(id, _) | Core::Ctor(id, _) => {
                self.reg.arity.get(&id.0).map(|a| (id.0.clone(), *a))
            }
            Core::Prim(op) => Some((op.symbol(), op.arity())),
            Core::Foreign(n) => self.reg.foreigns.get(n).map(|p| (n.clone(), p.arity())),
            _ => None,
        }
    }

    /// Emit `e`; the result is the name of an `i64`-valued operand (an SSA
    /// register or a constant). The current block is left open and
    /// unterminated.
    fn emit_expr(&mut self, e: &Core) -> Result<String> {
        match e {
            // A quotation is a runtime code value: a block over the
            // reified program of its body and the captured hole words
            // (supercompilation and JIT specification section 7).
            Core::Quote { holes, body, .. } => match body {
                Some(b) => emit_quote_block(self, b, holes),
                None => Err(crate::Error::plain(
                    "this quotation was consumed at compile time and has no reified body for target code",
                )),
            },
            Core::SpliceVal(_) => Err(crate::Error::plain(
                "internal error: a splice position reached code generation outside its quotation",
            )),
            Core::DeclsQuote { .. }
            | Core::SyntaxLit(..)
            | Core::StrLit(_) => Err(crate::Error::plain(
                "internal error: a compile-time value reached code generation (staging values never enter target code)",
            )),
            Core::Int(i) => Ok(i.to_string()),
            Core::ConstSym(sym) => {
                // A constant `Sym`: the name is a private constant and
                // the runtime's intern table materialises the cell from
                // it (the P14 constant-emission path). The table is keyed
                // by name, so `intern` of an equal name resolves to this
                // same cell (primitive types specification section 9).
                let unit = self.unit.clone();
                let cname = self.b.dec_name(&unit);
                self.b.global(format!(
                    "@{} = private constant [{} x i8] c\"{}\"",
                    quote(&cname),
                    sym.len(),
                    c_string_bytes(sym.as_bytes())
                ));
                let p = self.b.reg();
                self.b
                    .line(&format!("{p} = ptrtoint ptr @{} to i64", quote(&cname)));
                let r = self.b.reg();
                self.b.line(&format!(
                    "{r} = call i64 @cinch_sym_intern(i64 {p}, i64 {})",
                    sym.len()
                ));
                Ok(r)
            }
            Core::ConstBytes(buf) => {
                // A constant `Bytes` buffer: the contents are a private
                // constant and the runtime materialises a fresh buffer
                // from it (the P14 constant-emission path).
                let unit = self.unit.clone();
                let name = self.b.dec_name(&unit);
                self.b.global(format!(
                    "@{} = private constant [{} x i8] c\"{}\"",
                    quote(&name),
                    buf.len(),
                    c_string_bytes(buf)
                ));
                let p = self.b.reg();
                self.b
                    .line(&format!("{p} = ptrtoint ptr @{} to i64", quote(&name)));
                let r = self.b.reg();
                self.b.line(&format!(
                    "{r} = call i64 @cinch_bytes_from_data(i64 {p}, i64 {})",
                    buf.len()
                ));
                Ok(r)
            }
            Core::ConstBig { nat, digits } => {
                // A literal that fits an immediate is its own word (no
                // call, no box). Otherwise the `i64` path boxes a machine
                // integer and the slow path parses a private decimal
                // constant through the runtime.
                if let Some(w) = immediate_word(self.b.pointer_bytes, *nat, digits) {
                    return Ok(w.to_string());
                }
                let (from_i64, from_dec) = if *nat {
                    ("cinch_nat_from_i64", "cinch_nat_from_decimal")
                } else {
                    ("cinch_int_from_i64", "cinch_int_from_decimal")
                };
                if let Ok(v) = digits.parse::<i64>() {
                    let r = self.b.reg();
                    self.b.line(&format!("{r} = call i64 @{from_i64}(i64 {v})"));
                    return Ok(r);
                }
                let unit = self.unit.clone();
                let name = self.b.dec_name(&unit);
                let len = digits.len();
                self.b.global(format!(
                    "@{} = private constant [{len} x i8] c\"{digits}\"",
                    quote(&name)
                ));
                let r = self.b.reg();
                self.b.line(&format!(
                    "{r} = call i64 @{from_dec}(ptr @{}, i64 {len})",
                    quote(&name)
                ));
                Ok(r)
            }
            Core::ConstString { ctor, bytes } => {
                // The runtime `String` constant (a string literal's
                // value): ONE shared read-only data-section object, the
                // representation constructor's block wrapping a static
                // buffer, referenced directly (no construction, no
                // allocation). `String` is immutable and its backing is
                // never handed out mutably, so the shared object is
                // semantics-transparent. The object is
                // `{ hdr; bytes_word; hdr; len; data_word; data... }`:
                // word 1 is the `StrBuf` block (`ctor_body`'s `{ field0 }`,
                // its tag in its header's layout id), words 3-4 the
                // `Bytes` box (`Bytes { len; data }`, the runtime's
                // `repr(C)` pair), the data bytes inline. Words 0 and 2
                // are the two blocks' immortal headers, so both are
                // addressed one word past their header like any block
                // (an escaped `StrBuf` descriptor adds its word first).
                // The buffer's header kind is `KIND_BYTES_STATIC`: a
                // mutation attempt takes the defined read-only fault
                // instead of writing to constant data.
                let fields = self.reg.ctors.get(&ctor.0).map(|(_, f)| *f).ok_or_else(|| {
                    crate::Error::plain(format!("unknown constructor {ctor}"))
                })?;
                if fields != 1 {
                    return err(format!(
                        "the string constant's constructor {ctor} takes {fields} fields, expected 1"
                    ));
                }
                let desc = self.reg.ctor_desc(&ctor.0)?;
                let field = block::ctor_layout_field(desc);
                let unit = self.unit.clone();
                let name = self.b.dec_name(&unit);
                let qname = quote(&name);
                let mut parts: Vec<(String, String)> = Vec::new();
                if field == block::LAYOUT_BIG {
                    parts.push(("i64".into(), format!("i64 {}", desc as i64)));
                }
                let at = parts.len();
                let data_ty = format!("[{} x i8]", bytes.len());
                let ty = {
                    let mut t: Vec<&str> = parts.iter().map(|(t, _)| t.as_str()).collect();
                    t.extend(["i64", "ptr", "i64", "i64", "ptr", &data_ty]);
                    format!("{{ {} }}", t.join(", "))
                };
                let gep = |i: usize, tail: &str| {
                    format!("getelementptr inbounds ({ty}, ptr @{qname}, i64 0, i32 {}{tail})", at + i)
                };
                let str_hdr = block::static_header(block::KIND_CTOR, field) as i64;
                let buf_hdr = block::static_header(block::KIND_BYTES_STATIC, 2) as i64;
                let data = if bytes.is_empty() {
                    "[0 x i8] zeroinitializer".to_string()
                } else {
                    format!("[{} x i8] c\"{}\"", bytes.len(), c_string_bytes(bytes))
                };
                parts.push((String::new(), format!("i64 {str_hdr}")));
                parts.push((String::new(), format!("ptr {}", gep(3, ""))));
                parts.push((String::new(), format!("i64 {buf_hdr}")));
                parts.push((String::new(), format!("i64 {}", bytes.len())));
                parts.push((String::new(), format!("ptr {}", gep(5, ", i64 0"))));
                parts.push((String::new(), data));
                let init: Vec<&str> = parts.iter().map(|(_, v)| v.as_str()).collect();
                self.b.global(format!("@{qname} = internal constant {ty} {{ {} }}", init.join(", ")));
                let r = self.b.reg();
                self.b.line(&format!("{r} = ptrtoint ptr {} to i64", gep(1, "")));
                Ok(r)
            }
            Core::Bool(b) => Ok(if *b { "1" } else { "0" }.to_string()),
            Core::Unit => Ok("0".to_string()),
            Core::Var(x) => {
                if !self.bound.contains(x) {
                    return err(format!("unbound local {}", x.0));
                }
                let slot = self.slot(*x);
                let r = self.b.reg();
                self.b.line(&format!("{r} = load i64, ptr {slot}"));
                Ok(r)
            }
            Core::Global(id, _) => self.unit_value(&id.0),
            // A zero-field constructor's value is its static block, which is
            // the whole body of its unit (`ptrtoint @"U.blk"`): emitting the
            // address is the same word without a call, a frame and a return.
            // The block is immortal, so the reference counting a block
            // carries is unchanged. A constructor with fields is a function
            // and takes the closure path.
            Core::Ctor(id, _)
                if !self.reg.partial_module
                    && matches!(self.reg.ctors.get(&id.0), Some((_, 0))) =>
            {
                self.ctor_block(&id.0)
            }
            Core::Ctor(id, _) => self.unit_value(&id.0),
            Core::Prim(op) => self.unit_value(&op.symbol()),
            Core::Foreign(n) => self.unit_value(n),
            // A projection hands out an owned reference to the loaded
            // word (design/reclamation.md section 4): the container keeps
            // its own, and `crate::rc` keeps the container alive across
            // the load.
            Core::DictProj(d, idx, _, class) => {
                let dv = self.emit_expr(d)?;
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {dv} to ptr"));
                let r = self.b.load_at(&p, (1 + idx) * 8);
                rc_op(self.b, self.reg.rc_debug, false, &r, *class);
                Ok(r)
            }
            Core::Proj(d, idx) => {
                let dv = self.emit_expr(d)?;
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {dv} to ptr"));
                if let Some(fa) = &idx.access {
                    // A boundary-layout type: the field sits at its
                    // recorded offset in the payload (attribute
                    // annotations specification section 4.3). Only a
                    // word field is a reference the block holds; a
                    // scalar is raw and an inline aggregate is boxed out
                    // into a fresh block.
                    let r = load_field_word(&mut self.b, self.reg, &p, fa);
                    if matches!(fa.shape, crate::core::FieldShape::Word { .. }) {
                        rc_op(self.b, self.reg.rc_debug, false, &r, idx.class);
                    }
                    Ok(r)
                } else {
                    let r = self.b.load_at(&p, idx.slot * 8);
                    rc_op(self.b, self.reg.rc_debug, false, &r, idx.class);
                    Ok(r)
                }
            }
            Core::Fault => {
                // The runtime fault path (exit 70); the value is only the
                // zero word so the surrounding block stays well formed.
                self.b.line(&format!(
                    "call void @{}()",
                    quote("__cinch_blackhole")
                ));
                self.b.line("unreachable");
                let l = self.b.label();
                self.b.start(&l);
                Ok("0".to_string())
            }
            Core::InstDict(id, subs, _) => {
                let info = self
                    .reg
                    .insts
                    .get(&id.0)
                    .ok_or_else(|| crate::Error::plain(format!("unknown instance {id}")))?;
                if subs.len() != info.subdict_count {
                    return err(format!(
                        "instance {id} expects {} subdicts, got {}",
                        info.subdict_count,
                        subs.len()
                    ));
                }
                let mut args = Vec::with_capacity(subs.len());
                for s in subs {
                    let v = self.emit_expr(s)?;
                    args.push(format!("i64 {v}"));
                }
                let r = self.b.reg();
                if self.reg.static_dict(&id.0) && emitted_symbol(&format!("{id}.mk"), &self.reg.emit_modules) {
                    // The factory is defined here, so its static block is
                    // too: no call, no allocation.
                    let desc = self.reg.dict_desc(info);
                    self.b.line(&format!("{r} = ptrtoint ptr {} to i64",
                                         static_payload(&static_dict_name(&id.0), desc)));
                } else {
                    self.b.line(&format!(
                        "{r} = call i64 @{}({})",
                        quote(&format!("{id}.mk")),
                        args.join(", ")
                    ));
                }
                Ok(r)
            }
            Core::App(f, args) => self.emit_app(f, args),
            Core::Lam { fn_id, env, .. } => {
                let arity = self.reg.arity_of(&fn_id.0)?;
                if env.len() > arity {
                    return err(format!(
                        "lambda over {} captures {} values, above its arity {arity}",
                        fn_id,
                        env.len()
                    ));
                }
                let mut vals = Vec::with_capacity(env.len());
                for e in env {
                    vals.push(self.emit_expr(e)?);
                }
                let bytes = (3 + env.len()) * 8;
                let c = self.b.reg();
                self.b.line(&shaped_alloc_call(&c, bytes, block::KIND_CLOSURE, self.reg.closure_desc(&fn_id.0)));
                let cw = self.b.reg();
                self.b.line(&format!(
                    "{cw} = ptrtoint ptr @{app} to i64",
                    app = quote(&format!("{fn_id}.app"))
                ));
                self.b.store_at(&c, 0, &cw);
                self.b.store_at(&c, 8, &arity.to_string());
                self.b.store_at(&c, 16, &env.len().to_string());
                for (j, v) in vals.iter().enumerate() {
                    self.b.store_at(&c, 24 + 8 * j, v);
                }
                let w = self.b.reg();
                self.b.line(&format!("{w} = ptrtoint ptr {c} to i64"));
                Ok(w)
            }
            Core::Let(x, _, e, body) => {
                if !self.bound.contains(x) {
                    return err(format!("let binds unknown local {}", x.0));
                }
                let v = self.emit_let(*x, e)?;
                let slot = self.slot(*x);
                self.b.line(&format!("store i64 {v}, ptr {slot}"));
                self.emit_expr(body)
            }
            // A decision delivers its value to a destination, so its
            // branches may themselves end in a `Jump` (see `Dest`).
            Core::If(..) | Core::Case { .. } => self.emit_value(e),
            // A join in value position: its continuation is the destination
            // slot, and its block is emitted beside it.
            Core::Join { .. } => self.emit_value(e),
            Core::Jump { .. } => err(
                "internal error: a jump in a value position (a jump is only reached from the tail of its join's body or through a destination-carrying form)",
            ),
            Core::Demand(cell, class) => {
                let v = self.emit_expr(cell)?;
                let r = self.b.reg();
                self.b.line(&format!(
                    "{r} = call i64 @{}(i64 {v}, i1 {})",
                    quote("__cinch_demand"),
                    q_counted(*class) != 0,
                ));
                rc_op(self.b, self.reg.rc_debug, false, &r, *class);
                Ok(r)
            }
            Core::BoxAlloc => {
                // `{ state; value }` fixpoint box: an empty read faults
                // through the black-hole rule (exit 70).
                let c = self.b.reg();
                self.b.line(&alloc_call(&c, 16, block::KIND_KNOT));
                self.b.store_at(&c, 0, "0");
                self.b.store_at(&c, 8, "0");
                let w = self.b.reg();
                self.b.line(&format!("{w} = ptrtoint ptr {c} to i64"));
                Ok(w)
            }
            Core::BoxInit(cell, v, class) => {
                if !self.bound.contains(cell) {
                    return err(format!("box init for unknown local {}", cell.0));
                }
                let v = self.emit_expr(v)?;
                // The slot keeps one reference until scope teardown.
                // The initializer result owns another reference.
                rc_op(self.b, self.reg.rc_debug, false, &v, *class);
                let slot = self.slot(*cell);
                let bw = self.b.reg();
                self.b.line(&format!("{bw} = load i64, ptr {slot}"));
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {bw} to ptr"));
                self.b.store_at(&p, 8, &v);
                self.b.store_at(&p, 0, "1");
                Ok(v)
            }
            Core::BoxGet(cell, class) => {
                let w = self.emit_expr(cell)?;
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
                let st = self.b.reg();
                self.b.line(&format!("{st} = load i64, ptr {p}"));
                let empty = self.b.reg();
                self.b.line(&format!("{empty} = icmp eq i64 {st}, 0"));
                let lbad = self.b.label();
                let lok = self.b.label();
                self.b.line(&format!("br i1 {empty}, label %{lbad}, label %{lok}"));
                self.b.start(&lbad);
                self.b.line("call void @exit(i32 70)");
                self.b.line("unreachable");
                self.b.start(&lok);
                let r = self.b.load_at(&p, 8);
                rc_op(self.b, self.reg.rc_debug, false, &r, *class);
                Ok(r)
            }
            Core::BoxClear(cell, class) => {
                let w = self.emit_expr(cell)?;
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
                let payload = self.b.load_at(&p, 8);
                self.b.store_at(&p, 0, "0");
                self.b.store_at(&p, 8, "0");
                rc_op(self.b, self.reg.rc_debug, true, &payload, *class);
                Ok("0".to_string())
            }
            Core::LazyAlloc => {
                // `{ state; owned; value }`; the owned slot is filled by
                // `LazyInit`.
                let c = self.b.reg();
                self.b.line(&alloc_call(&c, 24, block::KIND_MEMO));
                self.b.store_at(&c, 0, "0");
                self.b.store_at(&c, 8, "0");
                self.b.store_at(&c, 16, "0");
                let w = self.b.reg();
                self.b.line(&format!("{w} = ptrtoint ptr {c} to i64"));
                Ok(w)
            }
            Core::LazyInit(cell, thunk) => {
                if !self.bound.contains(cell) {
                    return err(format!("memo init for unknown local {}", cell.0));
                }
                let tv = self.emit_expr(thunk)?;
                let slot = self.slot(*cell);
                let cw = self.b.reg();
                self.b.line(&format!("{cw} = load i64, ptr {slot}"));
                let p = self.b.reg();
                self.b.line(&format!("{p} = inttoptr i64 {cw} to ptr"));
                self.b.store_at(&p, 8, &tv);
                Ok("0".to_string())
            }
            Core::RefSet(l, v, payload) => {
                if !self.bound.contains(l) {
                    return err(format!("ref set for unknown local {}", l.0));
                }
                let v = self.emit_expr(v)?;
                let slot = self.slot(*l);
                let cell = self.b.reg();
                self.b.line(&format!("{cell} = load i64, ptr {slot}"));
                let pointer = self.b.reg();
                self.b.line(&format!("{pointer} = inttoptr i64 {cell} to ptr"));
                let old = self.b.load_at(&pointer, 0);
                self.b.store_at(&pointer, 0, &v);
                rc_op(self.b, self.reg.rc_debug, true, &old, *payload);
                Ok("0".to_string())
            }
            Core::Dup(x, class, body) | Core::Drop(x, class, body) => {
                self.emit_count(e, *x, *class)?;
                self.emit_expr(body)
            }
            Core::Reuse { token, ctor, args, already_set } => {
                self.emit_reuse(*token, &ctor.0, args, already_set)
            }
            Core::Release { block, moved, rest, token, body } => {
                self.emit_release(*block, moved, rest, *token)?;
                self.emit_expr(body)
            }
        }
    }

    /// The count operation of a `Dup`/`Drop` node on its binder's word.
    fn emit_count(&mut self, e: &Core, x: LocalId, class: Class) -> Result<()> {
        if !self.bound.contains(&x) {
            return err(format!("count operation on unknown local {}", x.0));
        }
        // A frame block carries no count: a `dup` is a second static
        // reference (which the pass refuses unless the payload holds
        // nothing counted, so this arm is the backstop), and a `drop`
        // releases the payload here, in place of a count reaching zero and
        // freeing the block.
        if let Some(site) = self.frame_site(x).cloned() {
            if matches!(e, Core::Drop(..)) {
                let w = self.load_local(x);
                self.frame_release(&w, &site.fields);
            }
            return Ok(());
        }
        let slot = self.slot(x);
        let w = self.b.reg();
        self.b.line(&format!("{w} = load i64, ptr {slot}"));
        rc_op(
            self.b,
            self.reg.rc_debug,
            matches!(e, Core::Drop(..)),
            &w,
            class,
        );
        Ok(())
    }

    /// A `Release` (drop specialisation, design/reclamation.md section 4):
    /// a uniquely held block hands its moved fields over, drops the rest
    /// and is freed without visiting its fields; any other block dups the
    /// moved fields and is dropped.
    fn emit_release(
        &mut self,
        block: LocalId,
        moved: &[(LocalId, Class)],
        rest: &[(LocalId, Class)],
        token: Option<LocalId>,
    ) -> Result<()> {
        for x in std::iter::once(&block).chain(moved.iter().chain(rest).map(|(l, _)| l)) {
            if !self.bound.contains(x) {
                return err(format!("count operation on unknown local {}", x.0));
            }
        }
        if let Some(t) = token {
            if !self.bound.contains(&t) {
                return err(format!("count operation on unknown local {}", t.0));
            }
        }
        // The token's slot first: both paths write it before the body runs.
        let tslot = token.map(|t| self.slot(t));
        let cold = |this: &mut Self| {
            if let Some(slot) = &tslot {
                this.b.line(&format!("store i64 0, ptr {slot}"));
            }
        };
        if self.frame_site(block).is_some() {
            // A frame block is never freed and has no count to test, so its
            // death is the unique path of a drop specialisation without the
            // free: the moved fields keep the references the block held and
            // the rest are dropped here. The search for a unique count
            // would always answer `no` (a frame count is `RC_FRAME`), so it
            // is not emitted. The block is marked released first, so a
            // second path into it (a caller's drop of the same reference
            // after the copy that specialisation unrolled) is a no-op
            // rather than a second release of the payload.
            let w = self.load_local(block);
            self.frame_mark_released(&w);
            for (r, c) in rest {
                let v = self.load_local(*r);
                rc_op(self.b, self.reg.rc_debug, true, &v, *c);
            }
            cold(self);
            return Ok(());
        }
        let w = self.load_local(block);
        let shared = |this: &mut Self, w: &str| {
            for (m, c) in moved {
                let v = this.load_local(*m);
                rc_op(this.b, this.reg.rc_debug, false, &v, *c);
            }
            rc_op(this.b, this.reg.rc_debug, true, w, Class::Obj);
        };
        if !self.b.reclaim || self.reg.rc_debug {
            shared(self, &w);
            cold(self);
            return Ok(());
        }
        let (lunique, lshared, ljoin) = (self.b.label(), self.b.label(), self.b.label());
        let unique = self.b.reg();
        self.b.line(&format!("{unique} = call i1 @{RC_UNIQUE}(i64 {w})"));
        self.b.line(&format!("br i1 {unique}, label %{lunique}, label %{lshared}"));
        self.b.start(&lunique);
        for (r, c) in rest {
            let v = self.load_local(*r);
            rc_op(self.b, false, true, &v, *c);
        }
        match &tslot {
            // The cell is the reuse's, so the block is not freed: its
            // payload is empty and its header still counts one owner.
            Some(slot) => self.b.line(&format!("store i64 {w}, ptr {slot}")),
            None => self.b.line(&format!("call void @cinch_free(i64 {w})")),
        }
        self.b.line(&format!("br label %{ljoin}"));
        self.b.start(&lshared);
        shared(self, &w);
        cold(self);
        self.b.line(&format!("br label %{ljoin}"));
        self.b.start(&ljoin);
        Ok(())
    }

    /// `Core::Reuse`: write `ctor`'s fields into the cell the token names
    /// when the `Release` that bound it found the block uniquely held, and
    /// into a fresh block otherwise. The cell's payload was emptied and
    /// its counted fields released by that `Release`, and every payload
    /// word is written here (the reused constructor has the same word
    /// count), so no stale reference survives; the header keeps its count
    /// of one and its kind and takes the new constructor's layout.
    fn emit_reuse(
        &mut self,
        token: LocalId,
        ctor: &str,
        args: &[Core],
        already_set: &[u32],
    ) -> Result<String> {
        if !self.bound.contains(&token) {
            return err(format!("reuse of unknown local {}", token.0));
        }
        let (_, fields) = self
            .reg
            .ctors
            .get(ctor)
            .copied()
            .ok_or_else(|| crate::Error::plain(format!("reuse of unknown constructor {ctor}")))?;
        if fields != args.len() {
            return err(format!(
                "internal error: reuse of {ctor} passes {} fields to {fields}",
                args.len()
            ));
        }
        let desc = self.reg.ctor_desc(ctor)?;
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.emit_expr(a)?);
        }
        let slot = self.slot(token);
        let t = self.b.reg();
        self.b.line(&format!("{t} = load i64, ptr {slot}"));
        let (lcold, lhot, ljoin) = (self.b.label(), self.b.label(), self.b.label());
        let reusing = self.b.reg();
        self.b.line(&format!("{reusing} = icmp ne i64 {t}, 0"));
        self.b.line(&format!("br i1 {reusing}, label %{lhot}, label %{lcold}"));
        // A fresh block holds nothing, so the cold path writes every word.
        self.b.start(&lcold);
        let fresh = self.b.reg();
        self.b.line(&ctor_alloc_call(&fresh, 8 * fields, desc));
        for (i, v) in vals.iter().enumerate() {
            self.b.store_at(&fresh, 8 * i, v);
        }
        self.b.line(&format!("br label %{ljoin}"));
        // The kept cell already holds the words the arm's own binders name
        // (`already_set`): the `Release` moved them out to those binders and
        // released nothing, so the word is still there. Skipping those
        // stores is Lean's `partitionSelfSets`
        // (`ExpandResetReuse.lean`), and both paths stay straight-line, so
        // the skip costs no branch per word.
        self.b.start(&lhot);
        let cell = self.b.reg();
        self.b.line(&format!("{cell} = inttoptr i64 {t} to ptr"));
        let field = block::ctor_layout_field(desc);
        let h = block::header(block::RC_FRESH, block::KIND_CTOR, field) as i64;
        // The header sits one word before the payload (and the escaped
        // descriptor's own word one word before that).
        let hp = self.b.gep_dyn(&cell, "-8");
        self.b.line(&format!("store i64 {h}, ptr {hp}"));
        if field == block::LAYOUT_BIG {
            // The escaped descriptor's own word describes the payload; it
            // is rewritten with the cell's new constructor.
            let ep = self.b.gep_dyn(&cell, "-16");
            self.b.line(&format!("store i64 {desc}, ptr {ep}"));
        }
        for (i, v) in vals.iter().enumerate() {
            if !already_set.contains(&(i as u32)) {
                self.b.store_at(&cell, 8 * i, v);
            }
        }
        self.b.line(&format!("br label %{ljoin}"));
        self.b.start(&ljoin);
        let pointer = self.b.reg();
        self.b.line(&format!(
            "{pointer} = phi ptr [ {fresh}, %{lcold} ], [ {cell}, %{lhot} ]"
        ));
        let w = self.b.reg();
        self.b.line(&format!("{w} = ptrtoint ptr {pointer} to i64"));
        Ok(w)
    }

    /// The frame site of the binder `x`, when the stack allocation pass
    /// lowered its allocation into the frame.
    fn frame_site(&self, x: LocalId) -> Option<&crate::stack::Site> {
        self.frame.and_then(|plan| plan.site(x.0))
    }

    /// The value of a `let`: its frame allocation lowered into the slot the
    /// pass reserved, or the ordinary expression.
    fn emit_let(&mut self, x: LocalId, e: &Core) -> Result<String> {
        match self.frame_site(x).cloned() {
            Some(site) => self.emit_frame_alloc(&site, e),
            None => self.emit_expr(e),
        }
    }

    /// Build the block a frame site decided on in its slot: the payload
    /// words take the values of `e`'s arguments (or a closure's saved
    /// words), the header is written once, and the block word is the
    /// slot's payload pointer. The header's count is `RC_FRAME`
    /// (`block_kinds.rs`), so every count path treats the block as
    /// immortal and the code generator releases the payload at the point
    /// the reference dies ([`CoreEmitter::frame_release`]).
    fn emit_frame_alloc(&mut self, site: &crate::stack::Site, e: &Core) -> Result<String> {
        use crate::stack::Shape;
        let (desc, kind) = match &site.shape {
            Shape::Ctor { ctor, .. } => (self.reg.ctor_desc(&ctor.0)?, block::KIND_CTOR),
            Shape::Closure { fn_id, .. } => (self.reg.closure_desc(&fn_id.0), block::KIND_CLOSURE),
        };
        // The values first: an argument may allocate or release, and the
        // slot's header must describe a block nothing can reach yet.
        let vals: Vec<String> = match (&site.shape, e) {
            (Shape::Ctor { .. }, Core::App(_, args)) if args.len() as u32 == site.words => {
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.emit_expr(a)?);
                }
                vals
            }
            (Shape::Closure { .. }, Core::Lam { env, .. }) if env.len() as u32 + 3 == site.words => {
                let mut vals = Vec::with_capacity(env.len());
                for a in env {
                    vals.push(self.emit_expr(a)?);
                }
                vals
            }
            _ => {
                return err(format!(
                    "internal error: the frame site of slot {} is not the allocation it was decided for",
                    site.slot
                ))
            }
        };
        let payload = self.frame_payload(site, desc, kind);
        if let Shape::Closure { fn_id, env } = &site.shape {
            // `{ code; arity; nfilled; saved... }`: the same prefix the
            // heap closure has (`Core::Lam` above).
            let arity = self.reg.arity_of(&fn_id.0)?;
            let cw = self.b.reg();
            self.b.line(&format!(
                "{cw} = ptrtoint ptr @{app} to i64",
                app = quote(&format!("{fn_id}.app"))
            ));
            self.b.store_at(&payload, 0, &cw);
            self.b.store_at(&payload, 8, &arity.to_string());
            self.b.store_at(&payload, 16, &env.to_string());
        }
        let first = match &site.shape {
            Shape::Ctor { .. } => 0,
            Shape::Closure { .. } => 24,
        };
        for (i, v) in vals.iter().enumerate() {
            self.b.store_at(&payload, first + 8 * i, v);
        }
        let w = self.b.reg();
        self.b.line(&format!("{w} = ptrtoint ptr {payload} to i64"));
        Ok(w)
    }

    /// The payload pointer of the frame slot `site` occupies, writing the
    /// header (and, for an escaped descriptor, the descriptor word) once
    /// per execution. The slot itself is one entry block `alloca`, aligned
    /// and sized as a multiple of 16 bytes (`Builder::alloca_frame`).
    fn frame_payload(&mut self, site: &crate::stack::Site, desc: u64, kind: u8) -> String {
        let field = block::ctor_layout_field(desc);
        let escaped = field == block::LAYOUT_BIG;
        let base = match self.frame_bases.get(&site.slot) {
            Some(base) => base.clone(),
            None => {
                let words = site.words as usize + 1 + usize::from(escaped);
                let base = self.b.alloca_frame(words);
                self.frame_bases.insert(site.slot, base.clone());
                base
            }
        };
        let prefix = usize::from(escaped) * 8;
        if escaped {
            self.b.line(&format!("store i64 {desc}, ptr {base}"));
        }
        let h = block::header(block::RC_FRAME, kind, field) as i64;
        let hp = self.b.gep(&base, prefix);
        self.b.line(&format!("store i64 {h}, ptr {hp}"));
        self.b.gep(&base, prefix + 8)
    }

    /// Release the counted payload of the frame block behind `w`: mark the
    /// block released, then drop each counted field with an ordinary
    /// `Drop`, in reverse payload order (D390: destruction runs in
    /// reverse). The block itself is not freed and nothing is scanned: the
    /// pass knows the counted words, so this is cheaper than the runtime
    /// walk `cinch_frame_release` does for a block whose origin the unit
    /// cannot see (design/stack-allocation.md section 7.3).
    fn frame_release(&mut self, w: &str, fields: &[(u32, Class)]) {
        self.frame_mark_released(w);
        if fields.is_empty() {
            return;
        }
        let p = self.b.reg();
        self.b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
        for (off, class) in fields.iter().rev() {
            let v = self.b.load_at(&p, *off as usize);
            rc_op(self.b, self.reg.rc_debug, true, &v, *class);
        }
    }

    /// Mark the frame block behind `w` released without touching its
    /// payload: whoever dropped its counted words has done so, and every
    /// later path into this block (a second drop of the same reference, a
    /// callee that took it as an owned parameter, a parent frame block
    /// releasing the payload it was stored in) must find the marker and do
    /// nothing. The count field is the low `RC_BITS`; kind and layout stay
    /// as the allocation wrote them, so the block's tag and payload map
    /// still read correctly.
    fn frame_mark_released(&mut self, w: &str) {
        let p = self.b.reg();
        self.b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
        let hp = self.b.reg();
        self.b.line(&format!("{hp} = getelementptr inbounds i8, ptr {p}, i64 -8"));
        let h = self.b.reg();
        self.b.line(&format!("{h} = load i64, ptr {hp}"));
        let keep = self.b.reg();
        self.b.line(&format!("{keep} = and i64 {h}, {}", HEADER_RC_MASK as i64));
        let marked = self.b.reg();
        self.b.line(&format!(
            "{marked} = or i64 {keep}, {}",
            block::RC_FRAME_RELEASED
        ));
        self.b.line(&format!("store i64 {marked}, ptr {hp}"));
    }

    /// The current word of a bound local.
    fn load_local(&mut self, x: LocalId) -> String {
        let slot = self.slot(x);
        let w = self.b.reg();
        self.b.line(&format!("{w} = load i64, ptr {slot}"));
        w
    }

    /// Emit an application: a saturated call of a known unit is a direct
    /// `call`; anything else (a dynamic head, a partial application, an
    /// over-application) goes through the `__apply` stub.
    fn emit_app(&mut self, f: &Core, args: &[Core]) -> Result<String> {
        // A saturated zero-field constructor application is the block, not a
        // call to the constructor's unit. The saturation test is exact: a
        // zero-field constructor takes no arguments, so `args` being empty is
        // the only saturated application there is.
        if args.is_empty() {
            if let Core::Ctor(id, _) = f {
                if !self.reg.partial_module
                    && matches!(self.reg.ctors.get(&id.0), Some((_, 0)))
                {
                    return self.ctor_block(&id.0);
                }
            }
        }
        if let Some((unit, arity)) = self.known_unit(f) {
            if arity == args.len() {
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.emit_expr(a)?);
                }
                // The result's `!range` fact where the callee's type
                // states one (the plan note's item 3).
                let rng = match self.reg.ranges.get(&unit) {
                    Some(&(lo, hi)) if lo < hi => {
                        format!(", !range {}", range_meta_i64(self.reg, lo, hi))
                    }
                    _ => String::new(),
                };
                // A widened callee is called in its worker directly,
                // without the wrapper's extra frame. A worker is
                // word-typed by construction (it exists to give every
                // member of a tail-call group one signature), so a
                // narrowed call never goes to one.
                let (sym, width) = self.reg.tail_sig(&unit, arity);
                let narrow = if width == arity {
                    self.reg.narrow_sigs.get(&unit)
                } else {
                    None
                };
                let Some(sig) = narrow else {
                    let r = self.b.reg();
                    self.b.line(&format!(
                        "{r} = call i64 @{}({}){rng}",
                        quote(&sym),
                        padded_args(&vals, width)
                    ));
                    return Ok(r);
                };
                // The callee's signature is narrow at some position: hand
                // each argument over at its proved width and take the
                // result back as a word, which is what the body of the
                // caller works in (the mirror of the callee's own entry
                // widening).
                let mut args_ty: Vec<String> = Vec::with_capacity(vals.len());
                for (i, v) in vals.iter().enumerate() {
                    let c = sig.param(i);
                    let w = c.narrow(self.b, v);
                    args_ty.push(format!("{} {w}", c.ty()));
                }
                // `!range` metadata is typed like the call, and the
                // registered range is a word range; a narrowed result's
                // type already bounds it, so the metadata stays on word
                // results only.
                let rng = if sig.ret.is_word() { rng } else { String::new() };
                let r = self.b.reg();
                self.b.line(&format!(
                    "{r} = call {} @{}({}){rng}",
                    sig.ret.ty(),
                    quote(&sym),
                    args_ty.join(", ")
                ));
                return Ok(sig.ret.widen(self.b, &r));
            }
        }
        let fv = self.emit_expr(f)?;
        let mut vals = Vec::with_capacity(args.len());
        for a in args {
            vals.push(self.emit_expr(a)?);
        }
        let buf = self.b.alloca_words(args.len());
        for (i, v) in vals.iter().enumerate() {
            let p = self.b.gep(&buf, i * 8);
            self.b.line(&format!("store i64 {v}, ptr {p}"));
        }
        let r = self.b.reg();
        self.b.line(&format!(
            "{r} = call i64 @__apply(i64 {fv}, i64 {}, ptr {buf})",
            args.len()
        ));
        Ok(r)
    }

    /// Emit `e` in tail position: the unit returns whatever `e` produces
    /// (surface grammar local amendments section 9; decision D104). The
    /// caller has already emitted its entry block (the parameter spills
    /// and allocas) and branched to the loop header this emitter jumps
    /// back to.
    ///
    /// - A saturated call of this very unit evaluates its arguments into
    ///   registers first, then stores them over the unit's parameter
    ///   slots and jumps to the loop header: constant stack however deep
    ///   the recursion runs.
    /// - A tail call to a known unit whose tail signature has the
    ///   caller's parameter count is a `musttail` call whose result
    ///   returns directly (mutual tail recursion: the C ABI boundary
    ///   does not keep frames). The members of a recursive group of
    ///   mixed arities share one widened signature
    ///   (`crate::core::tail_widths`), so their mutual calls qualify.
    /// - Anything else is an ordinary call followed by `ret`.
    /// Emit `e` as a value: a destination slot the structured forms store
    /// into, then the load. Decisions, joins and the count and `let` chains
    /// in front of them deliver through [`CoreEmitter::emit_to`], so a
    /// `Jump` inside them still reaches the one destination of this value.
    fn emit_value(&mut self, e: &Core) -> Result<String> {
        let slot = self.b.alloca_i64();
        let label = self.b.label();
        let dest = Dest::Slot { slot: slot.clone(), label: label.clone() };
        self.emit_to(e, &dest)?;
        self.b.start(&label);
        let r = self.b.reg();
        self.b.line(&format!("{r} = load i64, ptr {slot}"));
        Ok(r)
    }

    /// Emit `e` so its value reaches `dest`. `Dest::Ret` is the tail
    /// position: the value is returned, and a saturated self or mutual tail
    /// call stays a jump or a `musttail` call (decision D104). Every other
    /// form evaluates and settles into the destination; a `Jump` closes its
    /// block with a branch to the join's block, which settles into the same
    /// destination, so a value-position jump is not a special case.
    fn emit_to(&mut self, e: &Core, dest: &Dest) -> Result<()> {
        match e {
            Core::Let(x, _, v, body) => {
                if !self.bound.contains(x) {
                    return err(format!("let binds unknown local {}", x.0));
                }
                let v = self.emit_let(*x, v)?;
                let slot = self.slot(*x);
                self.b.line(&format!("store i64 {v}, ptr {slot}"));
                self.emit_to(body, dest)
            }
            // Count operations precede what they guard, so a tail call
            // after them stays the last instruction before `ret`.
            Core::Dup(x, class, body) | Core::Drop(x, class, body) => {
                self.emit_count(e, *x, *class)?;
                self.emit_to(body, dest)
            }
            Core::Reuse { token, ctor, args, already_set } => {
                let w = self.emit_reuse(*token, &ctor.0, args, already_set)?;
                dest.settle(self.b, &w, self.ret);
                Ok(())
            }
            Core::Release { block, moved, rest, token, body } => {
                self.emit_release(*block, moved, rest, *token)?;
                self.emit_to(body, dest)
            }
            Core::If(c, t, f) => {
                let cv = self.emit_expr(c)?;
                let cmp = self.b.reg();
                self.b.line(&format!("{cmp} = icmp ne i64 {cv}, 0"));
                let lthen = self.b.label();
                let lelse = self.b.label();
                // Branch weights (attribute annotations specification
                // section 3.1): `!prof !{!"branch_weights", w_true,
                // w_false}` on the conditional branch.
                let prof = match (t.weight, f.weight) {
                    (Some(tw), Some(fw)) => {
                        format!(", !prof {}", self.reg.branch_weights(&[tw, fw]))
                    }
                    _ => String::new(),
                };
                self.b.line(&format!(
                    "br i1 {cmp}, label %{lthen}, label %{lelse}{prof}"
                ));
                self.b.start(&lthen);
                self.emit_to(&t.body, dest)?;
                self.b.start(&lelse);
                self.emit_to(&f.body, dest)
            }
            Core::Case { scrut, alts } => self.emit_case_to(scrut, alts, dest),
            // A join is a block, not a call: its body is emitted beside the
            // continuation, and the continuation and the body both settle
            // into this destination, which is the one continuation of the
            // join's value.
            Core::Join { id, params, body, rest } => {
                let site = self.enter_join(*id, params)?;
                self.emit_to(rest, dest)?;
                self.b.start(&site.label);
                self.emit_to(body, dest)?;
                self.joins.remove(id);
                Ok(())
            }
            Core::Jump { id, args } => {
                let Some(site) = self.joins.get(id).cloned() else {
                    return err(format!("jump to join j{} has no enclosing join", id.0));
                };
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.emit_expr(a)?);
                }
                if vals.len() != site.slots.len() {
                    return err(format!(
                        "jump to join j{} passes {} arguments to {} parameters",
                        id.0,
                        vals.len(),
                        site.slots.len()
                    ));
                }
                for (v, slot) in vals.iter().zip(&site.slots) {
                    self.b.line(&format!("store i64 {v}, ptr {slot}"));
                }
                self.b.line(&format!("br label %{}", site.label));
                Ok(())
            }
            Core::App(f, args) if matches!(dest, Dest::Ret) => {
                let direct = self.known_unit(f).filter(|(_, arity)| *arity == args.len());
                match direct {
                    Some((unit, _)) if unit == self.unit => {
                        // The self tail call: arguments first (a
                        // cross-assignment like `sum (acc + n) (n - 1)`
                        // must not clobber a slot mid-update), then the
                        // parameter slots, then the jump.
                        let mut vals = Vec::with_capacity(args.len());
                        for a in args {
                            vals.push(self.emit_expr(a)?);
                        }
                        for (i, v) in vals.iter().enumerate() {
                            let slot = self.slot(LocalId(i));
                            self.b.line(&format!("store i64 {v}, ptr {slot}"));
                        }
                        let loop_label = self.loop_label.clone();
                        // The back edge is where LLVM reads the loop's
                        // unrolling policy (attribute annotations
                        // specification section 5), so the hint's metadata
                        // rides this terminator and no other.
                        let meta = self.loop_meta.suffix(&self.reg.meta_defs, &self.reg.meta_next);
                        self.b.line(&format!("br label %{loop_label}{meta}"));
                        Ok(())
                    }
                    Some((unit, arity))
                        if self.reg.tail_sig(&unit, arity).1 == self.width
                            // `musttail` needs the callee's prototype to be
                            // the caller's exactly, parameters included, so
                            // the two narrowed signatures must agree. On a
                            // 32-bit target nearly every unit narrows its
                            // pointer and `Nat`/`Int` positions, and a
                            // tail-call group whose members agree keeps its
                            // constant stack; one that does not takes the
                            // ordinary call (design/narrow-value-
                            // representation.md section 4).
                            && self.reg.narrow_sigs.get(&unit).map(NarrowSig::proto)
                                == self.reg.narrow_sigs.get(&self.unit).map(NarrowSig::proto) =>
                    {
                        // Same prototype as the caller, so `musttail` is
                        // legal: the callee's frame replaces ours.
                        let mut vals = Vec::with_capacity(args.len());
                        for a in args {
                            vals.push(self.emit_expr(a)?);
                        }
                        let (sym, width) = self.reg.tail_sig(&unit, arity);
                        let args = match self.reg.narrow_sigs.get(&unit) {
                            None => padded_args(&vals, width),
                            Some(sig) => {
                                let mut typed = Vec::with_capacity(vals.len());
                                for (i, v) in vals.iter().enumerate() {
                                    let c = sig.param(i);
                                    let n = c.narrow(self.b, v);
                                    typed.push(format!("{} {n}", c.ty()));
                                }
                                typed.join(", ")
                            }
                        };
                        let ty = self.ret.ty();
                        let r = self.b.reg();
                        self.b.line(&format!("{r} = musttail call {ty} @{}({args})", quote(&sym)));
                        self.b.line(&format!("ret {ty} {r}"));
                        Ok(())
                    }
                    _ => {
                        let v = self.emit_app(f, args)?;
                        // The unit's own result narrowing applies here too:
                        // a body that is one application returns through
                        // this arm, and a narrowed `define` must return its
                        // narrowed type on every path.
                        let w = self.ret.narrow(self.b, &v);
                        self.b.line(&format!("ret {} {w}", self.ret.ty()));
                        Ok(())
                    }
                }
            }
            other => {
                let v = self.emit_expr(other)?;
                // A terminating expression (the fault path, or a value that
                // was delivered elsewhere) leaves nothing to settle.
                if !self.b.terminated {
                    dest.settle(self.b, &v, self.ret);
                }
                Ok(())
            }
        }
    }

    /// Open a join's scope: its block label, and the slot of each
    /// parameter. Every `Jump` in the scope stores into those slots and
    /// branches to the label, so the join has one entry block and no
    /// per-site code beyond the stores.
    fn enter_join(&mut self, id: JoinId, params: &[(LocalId, Class)]) -> Result<JoinSite> {
        let mut slots = Vec::with_capacity(params.len());
        for (param, _) in params {
            if !self.bound.contains(param) {
                return err(format!("join j{} binds unknown local {}", id.0, param.0));
            }
            slots.push(self.slot(*param));
        }
        let site = JoinSite { label: self.b.label(), slots };
        if self.joins.insert(id, site.clone()).is_some() {
            return err(format!("join j{} is bound twice in one body", id.0));
        }
        Ok(site)
    }

    /// The value a tail position delivers: the function's return.
    fn emit_tail(&mut self, e: &Core) -> Result<()> {
        self.emit_to(e, &Dest::Ret)
    }

    /// Emit the scrutinee dispatch of a `Case`, delivering the chosen
    /// alternative's value to `dest` (decision D104): in tail position that
    /// is the function's return, so each alternative tail-calls where it
    /// can; in a value position it is a slot and a block. Each alternative
    /// goes through [`CoreEmitter::emit_to`], so a `Jump` in an alternative
    /// reaches the same destination through its join's block.
    fn emit_case_to(&mut self, scrut: &Core, alts: &[CAlt<Core>], dest: &Dest) -> Result<()> {
        let sv = self.emit_expr(scrut)?;
        let sp = self.b.reg();
        self.b.line(&format!("{sp} = inttoptr i64 {sv} to ptr"));
        let mut cons: Vec<(usize, &Vec<LocalId>, &Core, Option<u32>)> = Vec::new();
        let mut default: Option<(Option<LocalId>, &Core, Option<u32>)> = None;
        for a in alts {
            match &a.pat {
                CPat::Con { tag, fields, .. } => {
                    if cons.iter().any(|(t, _, _, _)| t == tag) {
                        return err(format!("duplicate case tag {tag}"));
                    }
                    cons.push((*tag, fields, &a.body, a.weight));
                }
                CPat::Var(x, _) => {
                    if default.is_some() {
                        return err("case has more than one default alternative");
                    }
                    default = Some((Some(*x), &a.body, a.weight));
                }
                CPat::Wild => {
                    if default.is_some() {
                        return err("case has more than one default alternative");
                    }
                    default = Some((None, &a.body, a.weight));
                }
            }
        }

        let ldefault = self.b.label();
        if cons.is_empty() {
            // Defaults alone never read the scrutinee: it may be an
            // immediate, which has no header to read a tag from.
            self.b.line(&format!("br label %{ldefault}"));
        }
        let tagv = if cons.is_empty() {
            String::new()
        } else {
            ctor_tag(&mut self.b, &sp, self.reg.escapes())
        };
        let mut alt_labels = Vec::with_capacity(cons.len());
        let mut sw = format!("switch i64 {tagv}, label %{ldefault} [");
        for (tag, _, _, _) in &cons {
            let l = self.b.label();
            sw.push_str(&format!("\n  i64 {tag}, label %{l}"));
            alt_labels.push(l);
        }
        sw.push_str("\n]");
        // Branch weights (attribute annotations specification
        // section 3.1): one weight per successor, and the default is always case
        // #0 of an LLVM switch, so it is the first weight. The run-out
        // path of a covered match carries weight 0.
        let fully = !cons.is_empty()
            && cons.iter().all(|(_, _, _, w)| w.is_some())
            && default
                .as_ref()
                .map(|(_, _, w)| w.is_some())
                .unwrap_or(true);
        if fully {
            let mut ws = vec![default.as_ref().and_then(|(_, _, w)| *w).unwrap_or(0)];
            for (_, _, _, w) in &cons {
                ws.push(w.unwrap_or(0));
            }
            sw.push_str(&format!(", !prof {}", self.reg.branch_weights(&ws)));
        }
        if !cons.is_empty() {
            self.b.line(&sw);
        }

        for ((_, fields, rhs, _), l) in cons.iter().zip(&alt_labels) {
            self.b.start(l);
            for (i, x) in fields.iter().enumerate() {
                if !self.bound.contains(x) {
                    return err(format!("pattern binds unknown local {}", x.0));
                }
                let w = self.b.load_at(&sp, i * 8);
                let s = self.slot(*x);
                self.b.line(&format!("store i64 {w}, ptr {s}"));
            }
            self.emit_to(rhs, dest)?;
        }

        self.b.start(&ldefault);
        match default {
            Some((bind, rhs, _)) => {
                if let Some(x) = bind {
                    if !self.bound.contains(&x) {
                        return err(format!("pattern binds unknown local {}", x.0));
                    }
                    let s = self.slot(x);
                    self.b.line(&format!("store i64 {sv}, ptr {s}"));
                }
                self.emit_to(rhs, dest)?;
            }
            None => {
                // The checker enforces coverage; an unmatched tag has no
                // continuation, so the enclosing function returns 0.
                self.b.line(&format!("ret {} 0", self.ret.ty()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{GlobalId, InstanceDef, IntOp, NatOp};
    use crate::repr::Class;
    use std::process::Command;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static RUN_ID: AtomicUsize = AtomicUsize::new(0);

    /// Build the Rust runtime staticlib once per test process and return
    /// its path (the driver does the same for end-to-end builds).
    fn runtime_lib() -> std::path::PathBuf {
        use std::sync::LazyLock;
        static LIB: LazyLock<std::path::PathBuf> = LazyLock::new(|| {
            let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let runtime = manifest_dir.join("runtime");
            let out = Command::new("cargo")
                .arg("build")
                .arg("--manifest-path")
                .arg(runtime.join("Cargo.toml"))
                .arg("--target-dir")
                .arg(runtime.join("target"))
                .output()
                .expect("failed to run cargo for the runtime crate");
            assert!(
                out.status.success(),
                "building compiler/runtime (cinchruntime) failed:\n{}",
                String::from_utf8_lossy(&out.stderr)
            );
            let lib = runtime.join("target/debug/libcinchruntime.a");
            assert!(
                lib.exists(),
                "libcinchruntime.a not found after building the runtime"
            );
            lib
        });
        LIB.clone()
    }

    /// Emit `program`, compile and link the module with clang
    /// (`CINCH_CLANG`, else `clang` on PATH) against the runtime
    /// staticlib, run the binary and return its process exit code.
    fn run(name: &str, program: &Program) -> i32 {
        // `emit` compiles for the native profile, which reclaims: the
        // program gets its count operations exactly as a driver build does.
        let program = crate::rc::insert(program.clone(), &crate::core::TargetProfile::native())
            .unwrap_or_else(|e| panic!("[{name}] ownership pass failed: {e}"));
        let ir = emit(&program).expect("codegen emit failed");
        let uniq = format!(
            "cinch_codegen_{}_{}",
            std::process::id(),
            RUN_ID.fetch_add(1, Ordering::SeqCst)
        );
        let bin = std::env::temp_dir().join(&uniq);
        let ll = bin.with_extension("ll");
        std::fs::write(&ll, &ir).expect("failed to write .ll file");
        let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
        let cc = Command::new(&clang)
            .arg(&ll)
            .arg(runtime_lib())
            .arg("-o")
            .arg(&bin)
            .output()
            .unwrap_or_else(|e| panic!("[{name}] failed to invoke {clang}: {e}"));
        assert!(
            cc.status.success(),
            "[{name}] clang rejected emitted IR:\n{}\n--- IR ---\n{ir}",
            String::from_utf8_lossy(&cc.stderr)
        );
        // The platform's main-thread stack size, not the runtime's 1 GiB
        // reservation: the constant-stack tests prove depth against it.
        let out = Command::new(&bin)
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap_or_else(|e| panic!("[{name}] failed to run compiled binary: {e}"));
        let code = out.status.code().unwrap_or_else(|| {
            panic!(
                "[{name}] binary killed by signal, stderr: {}",
                String::from_utf8_lossy(&out.stderr)
            )
        });
        eprint!("{}", String::from_utf8_lossy(&out.stderr));
        let _ = std::fs::remove_file(&ll);
        let _ = std::fs::remove_file(&bin);
        code
    }

    fn gid(s: &str) -> GlobalId {
        GlobalId(s.to_string())
    }

    fn fndef(id: &str, ps: &[&str], body: Core) -> FnDef {
        FnDef {
            meta: crate::core::FnMeta::default(),
            id: gid(id),
            params: ps.iter().map(|s| s.to_string()).collect(),
            body,
            prim: None,
            ctor: None,
            hints: crate::core::FnHints::default(),
            repr: crate::core::UnitRepr::default(),
        }
    }

    fn prim_fn(id: &str, ps: &[&str], op: PrimOp) -> FnDef {
        FnDef {
            meta: crate::core::FnMeta::default(),
            id: gid(id),
            params: ps.iter().map(|s| s.to_string()).collect(),
            body: Core::Unit,
            prim: Some(op),
            ctor: None,
            hints: crate::core::FnHints::default(),
            repr: crate::core::UnitRepr::default(),
        }
    }

    fn ctor_fn(id: &str, ps: &[&str], tag: usize, fields: usize) -> FnDef {
        FnDef {
            meta: crate::core::FnMeta::default(),
            id: gid(id),
            params: ps.iter().map(|s| s.to_string()).collect(),
            body: Core::Unit,
            prim: None,
            ctor: Some((tag, fields)),
            hints: crate::core::FnHints::default(),
            repr: crate::core::UnitRepr::default(),
        }
    }

    fn thunk(id: &str, body: Core) -> FnDef {
        fndef(id, &[], body)
    }

    fn app(f: Core, args: Vec<Core>) -> Core {
        Core::App(Box::new(f), args)
    }

    fn g(id: &str) -> Core {
        Core::Global(gid(id), Vec::new())
    }

    fn ct(id: &str) -> Core {
        Core::Ctor(gid(id), Vec::new())
    }

    fn v(i: usize) -> Core {
        Core::Var(LocalId(i))
    }

    fn n(x: i64) -> Core {
        Core::Int(x)
    }

    fn p(op: PrimOp) -> Core {
        Core::Prim(op)
    }

    /// A wrapping machine `I64` arithmetic op (the tests below compare raw
    /// words, so they stay on the machine family).
    fn arith(op: ArithOp) -> PrimOp {
        PrimOp::Arith {
            ty: Scalar::I64,
            op,
            mode: ArithMode::Wrapping,
        }
    }

    fn negi() -> PrimOp {
        PrimOp::Neg {
            ty: Scalar::I64,
            mode: ArithMode::Wrapping,
        }
    }

    fn cmp(op: CmpOp) -> PrimOp {
        PrimOp::Cmp {
            ty: Scalar::I64,
            op,
        }
    }

    /// A mathematical `Int` constant (decimal digit string).
    fn cbig(s: &str) -> Core {
        Core::ConstBig {
            nat: false,
            digits: s.to_string(),
        }
    }

    /// A mathematical `Nat` constant (decimal digit string).
    fn cnat(s: &str) -> Core {
        Core::ConstBig {
            nat: true,
            digits: s.to_string(),
        }
    }

    fn main_prog(mut fns: Vec<FnDef>, body: Core) -> Program {
        fns.push(thunk("program.main", body));
        Program {
            total_bodies: crate::total::CheckedGraph::empty(),
            rewrites: Vec::new(),
            fns,
            instances: vec![],
            main: Some(gid("program.main")),
            plans: Default::default(),
            foreigns: Default::default(),
            prims: Vec::new(),
            laws: Vec::new(),
            lent: Default::default(),
            frame: Default::default(),
            stack: Default::default(),
            borrowed: Default::default(),
            inline: Default::default(),
        }
    }

    #[test]
    fn object_rejects_process_entries_and_unchecked_representations() {
        let profile = crate::core::TargetProfile::native();
        let mut p = main_prog(vec![], n(0));
        assert!(emit_object_for(&profile, &p)
            .err()
            .unwrap()
            .message
            .contains("process entry"));
        p.main = None;
        p.fns = vec![fndef("Library.unchecked", &["x"], v(0))];
        assert!(emit_object_for(&profile, &p)
            .err()
            .unwrap()
            .message
            .contains("checked parameter classes"));
        p.fns[0].repr.params = vec![Class::Param(0)];
        assert!(emit_object_for(&profile, &p)
            .err()
            .unwrap()
            .message
            .contains("checked parameter classes"));
        p.fns = vec![
            thunk("Library.value", n(1)),
            thunk("Library.value.object-closure", n(2)),
        ];
        assert!(emit_object_for(&profile, &p)
            .err()
            .unwrap()
            .message
            .contains("accessor collides"));
    }

    #[test]
    fn object_links_with_one_consumer_layout_table() {
        std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
            use crate::interface::{parse_json, JsonValue as J};
            let profile = crate::core::TargetProfile { rc_debug: true, ..crate::core::TargetProfile::native() };
            let source = "module Library where\n  data Payload where\n    Payload :: Int -> Payload\n  data Packet where\n    Packet :: Bool -> Payload -> Int -> Packet\n  capture :: Packet -> Unit -> Packet\n  capture x u = x\n  raw :: I64 -> I64\n  raw x = x\n  tagged :: Int -> Int\n  tagged x = x\n  signature Identity a where\n    pick :: a -> a\n  instance IdentityPacket :: Identity Packet where\n    pick x = x\n";
            let source = format!("{source}  make-vec :: Int -> Vec 1 Int\n  make-vec x = Cons x Nil\n  data Indexed (n :: Nat) a where\n    Indexed :: a -> Indexed n a\n  make-indexed :: Int -> Indexed 1 Int\n  make-indexed x = Indexed x\n");
            let checked = crate::driver::check_sources(&source, "<object-library>").unwrap();
            let (mut p, _) = crate::specialize::optimize(&checked, &profile).unwrap();
            // This fixture has no library dependencies: retain the actual
            // checked user units rather than shipping the embedded prelude.
            p.fns.retain(|f| f.id.0.starts_with("Library.") || (f.id.0.starts_with("<builtin>.") && f.ctor.is_some()));
            p.instances.retain(|i| i.id.0.starts_with("Library."));
            let p = crate::rc::insert(p, &profile).unwrap();
            let object = emit_object_for(&profile, &p).unwrap();
            assert!(!object.ir.contains("define i32 @main("));
            assert!(!object.ir.contains("@llvm.global_ctors"));
            assert!(!object.ir.contains("call void @cinch_layouts_register"));
            assert!(!object.ir.contains("define i64 @__apply("));
            assert_eq!(emit_object_for(&profile, &p).unwrap().abi, object.abi);
            let manifest = parse_json(&object.abi).unwrap();
            let m = manifest.as_object().unwrap();
            assert_eq!(m["ir_content_hash"].as_str().unwrap(), crate::interface::content_hash(&object.ir));
            let units = m["units"].as_array().unwrap();
            let unit = |name: &str| units.iter().find(|u| u.as_object().unwrap()["symbol"].as_str().unwrap() == name).unwrap().as_object().unwrap();
            assert_eq!(unit("Library.Packet")["params"].to_canonical_string(), "[\"R\",\"O\",\"T\"]");
            assert_eq!(unit("Library.capture")["params"].to_canonical_string(), "[\"O\",\"R\"]");
            assert_eq!(unit("Library.raw")["result"].as_str().unwrap(), "R");
            assert_eq!(unit("Library.tagged")["result"].as_str().unwrap(), "T");
            let desc = m["constructors"].as_object().unwrap()["Library.Packet"].as_object().unwrap()["descriptor"].as_num().unwrap();
            let mut words: Vec<u64> = m["layout_words"].as_array().unwrap().iter().map(|w| w.as_num().unwrap()).collect();
            let packet_entry = words[desc as usize];
            assert_eq!(packet_entry, block::layout_entry(3, block::scan_slot(scan_code(Class::Obj), 1) | block::scan_slot(scan_code(Class::TObj), 2)));
            // New consumer-owned layouts follow the entire finalized prefix.
            // This must include extension words, not merely descriptor rows.
            let consumer_index = words.len().div_ceil(64) * 64;
            words.resize(consumer_index, 0);
            words.push(block::layout_entry(1, block::scan_slot(scan_code(Class::Obj), 0)));
            let mut consumer = format!("@table = internal constant [{} x i64] [{}]\ndeclare void @cinch_layouts_register(ptr, i64)\ndeclare void @cinch_reclaim_enable()\n", words.len(), words.iter().map(|w| format!("i64 {}", *w as i64)).collect::<Vec<_>>().join(", "));
            consumer.push_str(APPLY_DEF);
            consumer.push_str(&format!("define void @install() {{\n call void @cinch_reclaim_enable()\n call void @cinch_layouts_register(ptr @table, i64 {})\n ret void\n}}\n", words.len()));
            for (alias, name, arity) in [("payload", "Library.Payload", 1), ("packet", "Library.Packet", 3)] {
                let params = (0..arity).map(|i| format!("i64 %a{i}")).collect::<Vec<_>>().join(", ");
                consumer.push_str(&format!("declare i64 @\"{name}\"({})\ndefine i64 @{alias}({params}) {{\n %r = call i64 @\"{name}\"({params})\n ret i64 %r\n}}\n", vec!["i64"; arity].join(", ")));
            }
            let capture_entry = unit("Library.capture")["closure_entry"].as_str().unwrap();
            consumer.push_str(&format!("declare i64 @\"{capture_entry}\"()\ndefine i64 @capture_closure() {{\n %r = call i64 @\"{capture_entry}\"()\n ret i64 %r\n}}\n"));
            consumer.push_str("declare i64 @\"Library.make-vec\"(i64)\ndefine i64 @make_vec(i64 %x) {\n %r = call i64 @\"Library.make-vec\"(i64 %x)\n ret i64 %r\n}\n");
            consumer.push_str("declare i64 @\"Library.make-indexed\"(i64)\ndefine i64 @make_indexed(i64 %x) {\n %r = call i64 @\"Library.make-indexed\"(i64 %x)\n ret i64 %r\n}\n");
            let dict = m["dictionaries"].as_object().unwrap().keys().next().unwrap();
            assert_eq!(m["dictionaries"].as_object().unwrap()[dict].as_object().unwrap()["fields"].to_canonical_string(), "[\"O\"]");
            consumer.push_str(&format!("declare i64 @\"{dict}.mk\"()\ndefine i64 @dictionary() {{\n %r = call i64 @\"{dict}.mk\"()\n ret i64 %r\n}}\n"));
            let dir = std::env::temp_dir().join(format!("cinch_object_{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join("library.ll"), &object.ir).unwrap();
            std::fs::write(dir.join("library.abi.json"), &object.abi).unwrap();
            std::fs::write(dir.join("consumer.ll"), consumer).unwrap();
            std::fs::write(dir.join("main.c"), format!(r#"#include <stdint.h>
extern void install(void), cinch_dup(uint64_t), cinch_drop(uint64_t);
extern uint64_t payload(uint64_t), packet(uint64_t,uint64_t,uint64_t), capture_closure(void), dictionary(void);
extern uint64_t make_vec(uint64_t);
extern uint64_t make_indexed(uint64_t);
extern uint64_t __apply(uint64_t,uint64_t,uint64_t*), cinch_int_from_decimal(const char*,uint64_t);
extern void *cinch_alloc_ctor(uint64_t,uint64_t);
extern int64_t cinch_int_cmp(uint64_t,uint64_t), cinch_exit_check(int64_t);
int main(void) {{
  install();
  uint64_t big = cinch_int_from_decimal("18446744073709551616",20);
  cinch_dup(big);
  uint64_t inner = payload(big), value = packet(1,inner,big);
  if (((uint64_t*)value)[0] != 1) return 1;
  uint64_t captured = __apply(capture_closure(),1,&value);
  uint64_t unit = 0, restored = __apply(captured,1,&unit);
  if (restored != value) return 2;
  uint64_t d = dictionary(), method = ((uint64_t*)d)[{dict_prefix}];
  cinch_dup(method); cinch_drop(d);
  restored = __apply(method,1,&restored);
  uint64_t *fields = (uint64_t*)restored;
  if (cinch_int_cmp(((uint64_t*)fields[1])[0],fields[2]) != 0) return 3;
  uint64_t *own = cinch_alloc_ctor(8,{consumer_index});
  own[0] = restored;
  cinch_drop((uint64_t)own);
  uint64_t vector_big = cinch_int_from_decimal("18446744073709551616",20);
  cinch_dup(vector_big);
  uint64_t indexed = make_indexed(vector_big);
  uint64_t vector = make_vec(vector_big);
  cinch_drop(vector);
  cinch_drop(indexed);
  return cinch_exit_check(0);
}}
"#, dict_prefix = block::DICT_PREFIX_WORDS)).unwrap();
            let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
            let compile = Command::new(&clang).arg("-c").arg(dir.join("library.ll")).arg("-o").arg(dir.join("library.o")).output().unwrap();
            assert!(compile.status.success(), "{}", String::from_utf8_lossy(&compile.stderr));
            let link = Command::new(&clang).arg(dir.join("consumer.ll")).arg(dir.join("main.c")).arg(dir.join("library.o")).arg(runtime_lib()).arg("-o").arg(dir.join("program")).output().unwrap();
            assert!(link.status.success(), "{}", String::from_utf8_lossy(&link.stderr));
            let run = Command::new(dir.join("program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
            assert_eq!(run.status.code(), Some(0), "{}", String::from_utf8_lossy(&run.stderr));
            assert!(String::from_utf8_lossy(&run.stderr).split_whitespace().any(|w| w == "leaked=0"));
            // The manifest really is canonical JSON, including u64 words.
            assert_eq!(manifest.to_canonical_string(), object.abi);
            assert!(matches!(m["layout_words"], J::Arr(_)));
        }).unwrap().join().unwrap();
    }

    #[test]
    fn object_layout_prefix_preserves_extended_pointer_maps() {
        let mut layouts = Layouts::default();
        let map: Vec<Class> = (0..40)
            .map(|i| [Class::Raw, Class::Obj, Class::TObj][i % 3])
            .collect();
        layouts.add_type("Wide", &[map.clone()]).unwrap();
        let desc = layouts.desc("Wide", 0).unwrap();
        let words = layouts.words().unwrap();
        assert!(words.len() > layouts.table.len());
        let ext = words[desc as usize] >> block::LAYOUT_ENTRY_WORDS_BITS;
        for (i, class) in map.iter().enumerate() {
            let w = words[ext as usize + i / block::SCAN_EXT_WORDS as usize];
            assert_eq!(
                (w >> (2 * (i % block::SCAN_EXT_WORDS as usize))) & 3,
                scan_code(*class)
            );
        }
    }

    #[test]
    fn object_driver_records_checked_interface_closure() {
        std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
            use crate::interface::parse_json;
            let dir = std::env::temp_dir().join(format!("cinch_object_driver_{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Library.cinch");
            std::fs::write(&input, "module Library where\n  export library-string\n  library-string :: Unit -> String\n  library-string u = \"boundaries\"\n").unwrap();
            let out = dir.join("base.o");
            crate::driver::driver(crate::driver::Mode::EmitObject, &input, Some(&out)).unwrap();
            let text = std::fs::read_to_string(out.with_extension("abi.json")).unwrap();
            let manifest = parse_json(&text).unwrap();
            let m = manifest.as_object().unwrap();
            assert_eq!(m["runtime_abi"].as_num().unwrap(), crate::interface::RUNTIME_ABI_VERSION as u64);
            let mut ifaces = Vec::new();
            for row in m["interfaces"].as_array().unwrap() {
                let row = row.as_object().unwrap();
                let iface = crate::interface::Iface::from_json(&row["interface"].to_canonical_string()).unwrap();
                assert_eq!(iface.runtime_abi, crate::interface::RUNTIME_ABI_VERSION);
                assert_eq!(iface.module, row["module"].as_str().unwrap());
                assert_eq!(iface.iface_hash(), row["iface_hash"].as_str().unwrap());
                ifaces.push(iface);
            }
            crate::interface::verify_chain(&ifaces).unwrap();
            crate::interface::check_coherence(&ifaces).unwrap();
            let library = ifaces.iter().find(|i| i.module == "Library").unwrap();
            assert_eq!(library.exports.values.len(), 1);
            assert_eq!(library.exports.values[0].name, "library-string");
            assert!(ifaces.iter().any(|i| i.module == "Data.String"));
            assert!(ifaces.iter().all(|i| !i.module.starts_with("Bootstrap.")));
            assert_eq!(m["object_content_hash"].as_str().unwrap(), crate::interface::content_hash_bytes(&std::fs::read(&out).unwrap()));
            let ir = std::fs::read_to_string(out.with_extension("ll")).unwrap();
            assert_eq!(m["ir_content_hash"].as_str().unwrap(), crate::interface::content_hash(&ir));
            assert!(!ir.contains("@llvm.global_ctors"));
            assert!(m["constructors"].as_object().unwrap().keys().any(|k| k.contains("StrBuf")));
            assert_eq!(manifest.to_canonical_string(), text);
            let string_iface = ifaces.iter().find(|i| i.module == "Data.String").unwrap();
            assert!(string_iface.exports.types.iter().all(|t| t.ctors.iter().all(|c| c.name != "StrBuf")));
            let words = m["layout_words"].as_array().unwrap();
            let mut consumer = format!("@table = internal constant [{} x i64] [{}]\ndeclare void @cinch_layouts_register(ptr, i64)\ndeclare void @cinch_reclaim_enable()\n", words.len(), words.iter().map(|w| format!("i64 {}", w.as_num().unwrap() as i64)).collect::<Vec<_>>().join(", "));
            consumer.push_str(APPLY_DEF);
            consumer.push_str(&format!("define void @install() {{\n call void @cinch_reclaim_enable()\n call void @cinch_layouts_register(ptr @table, i64 {})\n ret void\n}}\n", words.len()));
            consumer.push_str("declare i64 @\"Library.library-string\"(i64)\ndefine i64 @library_string() {\n %r = call i64 @\"Library.library-string\"(i64 0)\n ret i64 %r\n}\n");
            std::fs::write(dir.join("consumer.ll"), consumer).unwrap();
            std::fs::write(dir.join("main.c"), r#"#include <stdint.h>
extern void install(void), cinch_drop(uint64_t);
extern uint64_t library_string(void), cinch_bytes_len(uint64_t), cinch_bytes_get(uint64_t,uint64_t);
extern int64_t cinch_exit_check(int64_t);
int main(void) {
  install();
  uint64_t string = library_string(), bytes = ((uint64_t*)string)[0];
  if (cinch_bytes_len(bytes) != 10 || cinch_bytes_get(bytes,0) != 'b' || cinch_bytes_get(bytes,9) != 's') return 1;
  cinch_drop(string);
  return cinch_exit_check(0);
}
"#).unwrap();
            let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
            let link = Command::new(&clang).arg(dir.join("consumer.ll")).arg(dir.join("main.c")).arg(&out).arg(runtime_lib()).arg("-o").arg(dir.join("program")).output().unwrap();
            assert!(link.status.success(), "{}", String::from_utf8_lossy(&link.stderr));
            let run = Command::new(dir.join("program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
            assert_eq!(run.status.code(), Some(0), "{}", String::from_utf8_lossy(&run.stderr));
            assert!(String::from_utf8_lossy(&run.stderr).split_whitespace().any(|w| w == "leaked=0"));
        }).unwrap().join().unwrap();
    }

    #[test]
    fn object_explicit_roots_and_method_families_link() {
        std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(|| {
            use crate::interface::parse_json;
            use crate::specialize::{parse_object_roots, ObjectRoot};
            let dir = std::env::temp_dir().join(format!("cinch_object_roots_{}", std::process::id()));
            std::fs::create_dir_all(&dir).unwrap();
            let input = dir.join("Library.cinch");
            std::fs::write(&input, "module Library where\n  export choose, Poly, keep, PolyInt, PolyBool\n  choose :: forall a. a -> Unit -> a\n  choose x u = x\n  signature Poly a where\n    keep :: forall b. a -> b -> Unit -> b\n  instance PolyInt :: Poly Int where\n    keep anchor value unit = value\n  instance PolyBool :: Poly Bool where\n    keep anchor value unit = value\n").unwrap();
            let roots = parse_object_roots(r#"{"schema":1,"units":[{"symbol":"Library.choose","classes":"T"},{"symbol":"Library.choose","classes":"R"}],"dictionaries":[{"symbol":"Library.PolyInt","classes":""},{"symbol":"Library.PolyBool","classes":""}],"methods":[{"class":"Library.Poly","slot":0,"own":"T"},{"class":"Library.Poly","slot":0,"own":"R"}]}"#).unwrap();
            let profile = crate::core::TargetProfile { rc_debug: true, ..crate::core::TargetProfile::native() };
            let opts = crate::driver::DriverOptions { object_roots: roots.clone(), ..Default::default() };
            let out = dir.join("library.o");
            crate::driver::driver_with_profile_opts(crate::driver::Mode::EmitObject, crate::driver::StageMode::Refresh, &input, Some(&out), &profile, &opts).unwrap();
            let manifest = parse_json(&std::fs::read_to_string(out.with_extension("abi.json")).unwrap()).unwrap();
            let m = manifest.as_object().unwrap();
            let choose: Vec<_> = m["units"].as_array().unwrap().iter().filter_map(|u| {
                let name = u.as_object().unwrap()["symbol"].as_str().unwrap();
                name.starts_with("Library.choose@").then_some(name)
            }).collect();
            assert_eq!(choose, ["Library.choose@R", "Library.choose@T"]);
            let family = m["method_families"].as_array().unwrap().iter().find(|f| f.as_object().unwrap()["class"].as_str().unwrap() == "Library.Poly").unwrap().as_object().unwrap();
            assert_eq!(family["declared_methods"].as_num().unwrap(), 1);
            assert_eq!(family["slots"].to_canonical_string(), "[{\"logical\":0,\"own\":\"R\",\"physical\":0},{\"logical\":0,\"own\":\"T\",\"physical\":1}]");
            let dictionaries = m["dictionaries"].as_object().unwrap();
            for name in ["Library.PolyInt", "Library.PolyBool"] {
                let d = dictionaries[name].as_object().unwrap();
                assert_eq!(d["class"].as_str().unwrap(), "Library.Poly");
                assert_eq!(d["methods"].as_array().unwrap().len(), 2);
            }
            let words = m["layout_words"].as_array().unwrap();
            let mut consumer = format!("@table = internal constant [{} x i64] [{}]\ndeclare void @cinch_layouts_register(ptr, i64)\ndeclare void @cinch_reclaim_enable()\n", words.len(), words.iter().map(|w| format!("i64 {}", w.as_num().unwrap() as i64)).collect::<Vec<_>>().join(", "));
            consumer.push_str(APPLY_DEF);
            consumer.push_str(&format!("define void @install() {{\n call void @cinch_reclaim_enable()\n call void @cinch_layouts_register(ptr @table, i64 {})\n ret void\n}}\n", words.len()));
            for (alias, symbol) in [("raw_choose", "Library.choose@R"), ("tagged_choose", "Library.choose@T")] {
                consumer.push_str(&format!("declare i64 @\"{symbol}\"(i64,i64)\ndefine i64 @{alias}(i64 %x) {{\n %r = call i64 @\"{symbol}\"(i64 %x,i64 0)\n ret i64 %r\n}}\n"));
            }
            for (alias, symbol) in [("int_dict", "Library.PolyInt"), ("bool_dict", "Library.PolyBool")] {
                consumer.push_str(&format!("declare i64 @\"{symbol}.mk\"()\ndefine i64 @{alias}() {{\n %r = call i64 @\"{symbol}.mk\"()\n ret i64 %r\n}}\n"));
            }
            std::fs::write(dir.join("consumer.ll"), consumer).unwrap();
            std::fs::write(dir.join("main.c"), format!(r#"#include <stdint.h>
extern void install(void), cinch_dup(uint64_t), cinch_drop(uint64_t);
extern uint64_t raw_choose(uint64_t), tagged_choose(uint64_t), int_dict(void), bool_dict(void);
extern uint64_t __apply(uint64_t,uint64_t,uint64_t*), cinch_int_from_decimal(const char*,uint64_t);
extern int64_t cinch_int_cmp(uint64_t,uint64_t), cinch_exit_check(int64_t);
int main(void) {{
  install();
  if (raw_choose(19) != 19) return 1;
  uint64_t big = cinch_int_from_decimal("18446744073709551616",20);
  cinch_dup(big);
  uint64_t tagged = tagged_choose(big);
  if (cinch_int_cmp(big,tagged)) return 2;
  cinch_drop(tagged);
  for (int which = 0; which < 2; ++which) {{
    uint64_t d = which ? bool_dict() : int_dict();
    uint64_t raw = ((uint64_t*)d)[{prefix}], heap = ((uint64_t*)d)[{next}];
    cinch_dup(raw); cinch_dup(heap); cinch_drop(d);
    uint64_t raw_args[] = {{which ? 1 : 3, 23, 0}};
    if (__apply(raw,3,raw_args) != 23) return 3;
    cinch_dup(big);
    uint64_t heap_args[] = {{which ? 1 : 3, big}};
    uint64_t partial = __apply(heap,2,heap_args), unit = 0;
    uint64_t result = __apply(partial,1,&unit);
    if (cinch_int_cmp(result,big)) return 4;
    cinch_drop(result);
  }}
  cinch_drop(big);
  return cinch_exit_check(0);
}}
"#, prefix=block::DICT_PREFIX_WORDS, next=block::DICT_PREFIX_WORDS+1)).unwrap();
            let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
            let link = Command::new(&clang).arg(dir.join("consumer.ll")).arg(dir.join("main.c")).arg(&out).arg(runtime_lib()).arg("-o").arg(dir.join("program")).output().unwrap();
            assert!(link.status.success(), "{}", String::from_utf8_lossy(&link.stderr));
            let run = Command::new(dir.join("program")).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
            assert_eq!(run.status.code(), Some(0), "{}", String::from_utf8_lossy(&run.stderr));
            assert!(String::from_utf8_lossy(&run.stderr).split_whitespace().any(|w| w == "leaked=0"));
            let checked = crate::driver::check_sources(&std::fs::read_to_string(&input).unwrap(), "<roots>").unwrap();
            for root in [ObjectRoot::Unit { symbol: "Missing.unit".into(), classes: vec![] },
                ObjectRoot::Unit { symbol: "Library.choose".into(), classes: vec![] },
                ObjectRoot::Unit { symbol: "Library.choose".into(), classes: vec![Class::Param(0)] },
                ObjectRoot::Method { class: "Library.Poly".into(), slot: 7, own: vec![Class::Raw] }] {
                assert!(crate::specialize::optimize_object(&checked, &profile, &[root]).is_err());
            }
            for invalid in [r#"{"schema":2}"#, r#"{"schema":1,"units":[{"symbol":"x","classes":"P"}]}"#,
                r#"{"schema":1,"methods":[{"class":"x","slot":0,"own":"R","guess":true}]}"#] {
                assert!(parse_object_roots(invalid).is_err());
            }
            let mut incomplete = checked.clone();
            incomplete.fns.push(thunk("Library.incomplete", g("Library.choose")));
            assert!(crate::specialize::optimize_object(&incomplete, &profile, &roots).err().unwrap().message.contains("unrecorded representation"));
        }).unwrap().join().unwrap();
    }

    #[test]
    fn prim_fn_direct_call() {
        // A `foreign import` style primitive unit applied to two literals.
        let prog = main_prog(
            vec![prim_fn(
                "Prelude.wrappingAddI64",
                &["a", "b"],
                arith(ArithOp::Add),
            )],
            app(g("Prelude.wrappingAddI64"), vec![n(3), n(4)]),
        );
        assert_eq!(run("prim_fn_direct_call", &prog), 7);
    }

    #[test]
    fn core_prim_arithmetic_chain() {
        // (3 + 4) * 2 - 5 = 9 through Core::Prim spines.
        let body = app(
            p(arith(ArithOp::Sub)),
            vec![
                app(
                    p(arith(ArithOp::Mul)),
                    vec![app(p(arith(ArithOp::Add)), vec![n(3), n(4)]), n(2)],
                ),
                n(5),
            ],
        );
        assert_eq!(
            run("core_prim_arithmetic_chain", &main_prog(vec![], body)),
            9
        );
    }

    /// The net, piped-child and pseudo-terminal units
    /// (`PrimOp::Net`/`PrimOp::Proc`/`PrimOp::Pty`):
    /// every one of them checks its handle or its arguments at the
    /// boundary and answers the named error code as the prelude
    /// `Result`'s `Err` (tag 0) over the POSITIVE code, which the
    /// payload projection here reads back (slot 0 is the constructor
    /// tag). None of these cases needs a socket or a child: a handle no
    /// `cinch_net_tcp_connect`/`cinch_process_spawn` handed out, a port
    /// outside the `u16` range, and a program that does not exist are
    /// exactly the boundary's own checks, so this pins the lowering
    /// (`ret_result`) and those checks together, with no prelude in the
    /// way.
    #[test]
    fn net_and_process_prims_answer_the_named_error_as_result() {
        let bytes = |s: &str| Core::ConstBytes(s.as_bytes().to_vec());
        let cases: Vec<(&str, PrimOp, Vec<Core>, i32)> = vec![
            (
                "net-tcp-connect",
                PrimOp::Net(NetOp::TcpConnect),
                vec![bytes("127.0.0.1"), n(70000)],
                // `InvalidArgument`.
                6,
            ),
            (
                "net-read",
                PrimOp::Net(NetOp::Read),
                vec![n(4242), cnat("16")],
                6,
            ),
            (
                "net-write",
                PrimOp::Net(NetOp::Write),
                vec![n(4242), bytes("x")],
                6,
            ),
            ("net-close", PrimOp::Net(NetOp::Close), vec![n(4242)], 6),
            (
                "net-resolve",
                PrimOp::Net(NetOp::Resolve),
                vec![bytes("localhost"), n(-1)],
                6,
            ),
            (
                "process-spawn",
                PrimOp::Proc(ProcOp::Spawn),
                vec![
                    bytes("/nonexistent/cinch-program"),
                    bytes(""),
                    bytes(""),
                    bytes(""),
                ],
                // `NotFound`.
                1,
            ),
            (
                "process-write-stdin",
                PrimOp::Proc(ProcOp::WriteStdin),
                vec![n(4242), bytes("x")],
                6,
            ),
            (
                "process-close-stdin",
                PrimOp::Proc(ProcOp::CloseStdin),
                vec![n(4242)],
                6,
            ),
            (
                "process-read",
                PrimOp::Proc(ProcOp::Read),
                vec![n(4242), n(0), cnat("16")],
                6,
            ),
            ("process-wait", PrimOp::Proc(ProcOp::Wait), vec![n(4242)], 6),
            (
                "process-close",
                PrimOp::Proc(ProcOp::Close),
                vec![n(4242)],
                6,
            ),
            // The pseudo-terminal units (`PrimOp::Pty`): an invented
            // handle is the named argument error on every operation, and
            // the same `ret_result` lowering presents it.
            (
                "pty-spawn",
                PrimOp::Pty(PtyOp::Spawn),
                vec![
                    n(4242),
                    bytes("/nonexistent/cinch-program"),
                    bytes(""),
                    bytes(""),
                    bytes(""),
                ],
                6,
            ),
            (
                "pty-read",
                PrimOp::Pty(PtyOp::Read),
                vec![n(4242), cnat("16")],
                6,
            ),
            (
                "pty-write",
                PrimOp::Pty(PtyOp::Write),
                vec![n(4242), bytes("x")],
                6,
            ),
            ("pty-wait", PrimOp::Pty(PtyOp::Wait), vec![n(4242)], 6),
            ("pty-close", PrimOp::Pty(PtyOp::Close), vec![n(4242)], 6),
            (
                "pty-resize",
                PrimOp::Pty(PtyOp::Resize),
                vec![n(4242), cnat("70000"), cnat("24")],
                6,
            ),
        ];
        for (name, op, args, expected) in cases {
            let body = Core::Proj(
                Box::new(app(p(op), args)),
                crate::core::ProjField::slot(0, Class::Raw),
            );
            let code = run(&format!("net_proc_err_{name}"), &main_prog(vec![], body));
            assert_eq!(
                code, expected,
                "{name}: the Err payload became the exit code"
            );
        }
    }

    #[test]
    fn neg_int_comparison_and_if() {
        // negInt 3 == -3 selects 2.
        let body = Core::If(
            Box::new(app(
                p(cmp(CmpOp::Eq)),
                vec![app(p(negi()), vec![n(3)]), n(-3)],
            )),
            CBranch::new(n(2)),
            CBranch::new(n(3)),
        );
        assert_eq!(
            run("neg_int_comparison_and_if", &main_prog(vec![], body)),
            2
        );
    }

    #[test]
    fn bool_prims() {
        // eqBool true true -> if; notBool (false orBool (true andBool false)) = 1.
        let body = Core::If(
            Box::new(app(
                p(PrimOp::EqBool),
                vec![Core::Bool(true), Core::Bool(true)],
            )),
            CBranch::new(app(
                p(PrimOp::NotBool),
                vec![app(
                    p(PrimOp::OrBool),
                    vec![
                        Core::Bool(false),
                        app(
                            p(PrimOp::AndBool),
                            vec![Core::Bool(true), Core::Bool(false)],
                        ),
                    ],
                )],
            )),
            CBranch::new(n(9)),
        );
        assert_eq!(run("bool_prims", &main_prog(vec![], body)), 1);
    }

    #[test]
    fn bool_ne_and_int_comparisons() {
        // neBool false true = 1; 3<4 + 4<=4 + 5>4 + 5>=5 + 3!=4 = 5; sum 6.
        let body = app(
            p(arith(ArithOp::Add)),
            vec![
                app(p(PrimOp::NeBool), vec![Core::Bool(false), Core::Bool(true)]),
                app(
                    p(arith(ArithOp::Add)),
                    vec![
                        app(
                            p(arith(ArithOp::Add)),
                            vec![
                                app(
                                    p(arith(ArithOp::Add)),
                                    vec![
                                        app(p(cmp(CmpOp::Lt)), vec![n(3), n(4)]),
                                        app(p(cmp(CmpOp::Le)), vec![n(4), n(4)]),
                                    ],
                                ),
                                app(p(cmp(CmpOp::Gt)), vec![n(5), n(4)]),
                            ],
                        ),
                        app(
                            p(arith(ArithOp::Add)),
                            vec![
                                app(p(cmp(CmpOp::Ge)), vec![n(5), n(5)]),
                                app(p(cmp(CmpOp::Ne)), vec![n(3), n(4)]),
                            ],
                        ),
                    ],
                ),
            ],
        );
        assert_eq!(
            run("bool_ne_and_int_comparisons", &main_prog(vec![], body)),
            6
        );
    }

    #[test]
    fn int_div_and_rem_by_zero_trap_as_a_compiler_bug() {
        // Core built by hand bypasses the checker's `NonZero` divisor, so
        // it reaches the runtime's zero check: a compiler-bug trap on the
        // panic path (exit 71), the exit that well-typed code never takes.
        let body = app(
            p(PrimOp::Int(IntOp::Add)),
            vec![
                app(p(PrimOp::Int(IntOp::Div)), vec![cbig("7"), cbig("0")]),
                app(p(PrimOp::Int(IntOp::Rem)), vec![cbig("7"), cbig("0")]),
            ],
        );
        assert_eq!(
            run("int_div_and_rem_by_zero_trap", &main_prog(vec![], body)),
            71
        );
    }

    #[test]
    fn div_rem_truncating() {
        // 7 / 2 = 3, 7 % 2 = 1 -> 31, computed as bignums and read back
        // through `cinch_int_to_i64`.
        let body = app(
            p(PrimOp::Int(IntOp::ToI64)),
            vec![app(
                p(PrimOp::Int(IntOp::Add)),
                vec![
                    app(
                        p(PrimOp::Int(IntOp::Mul)),
                        vec![
                            app(p(PrimOp::Int(IntOp::Div)), vec![cbig("7"), cbig("2")]),
                            cbig("10"),
                        ],
                    ),
                    app(p(PrimOp::Int(IntOp::Rem)), vec![cbig("7"), cbig("2")]),
                ],
            )],
        );
        assert_eq!(run("div_rem_truncating", &main_prog(vec![], body)), 31);
    }

    #[test]
    fn partial_application_then_rest() {
        // add2 is applied to one argument; the filled closure takes the rest.
        let add2 = fndef(
            "add2",
            &["a", "b"],
            app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
        );
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app(g("add2"), vec![n(20)])),
            Box::new(app(v(10), vec![n(22)])),
        );
        assert_eq!(
            run(
                "partial_application_then_rest",
                &main_prog(vec![add2], body)
            ),
            42
        );
    }

    #[test]
    fn lambda_captures_env() {
        // mk x = \y -> x + y, built as a closure capturing x.
        let inc = fndef(
            "inc",
            &["cap", "y"],
            app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
        );
        let mk = fndef(
            "mk",
            &["x"],
            Core::Lam {
                fn_id: gid("inc"),
                env: vec![v(0)],
                classes: Vec::new(),
            },
        );
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app(g("mk"), vec![n(10)])),
            Box::new(app(v(10), vec![n(32)])),
        );
        assert_eq!(
            run("lambda_captures_env", &main_prog(vec![inc, mk], body)),
            42
        );
    }

    #[test]
    fn over_application() {
        // outer 10 32: outer returns a lambda, the second argument lands on it.
        let inner = fndef(
            "inner",
            &["cap", "y"],
            app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
        );
        let outer = fndef(
            "outer",
            &["x"],
            Core::Lam {
                fn_id: gid("inner"),
                env: vec![v(0)],
                classes: Vec::new(),
            },
        );
        let body = app(g("outer"), vec![n(10), n(32)]);
        assert_eq!(
            run("over_application", &main_prog(vec![inner, outer], body)),
            42
        );
    }

    #[test]
    fn one_apply_stub_per_unit() {
        // One entry per function, whatever argument count it is handed: the
        // `caml_applyN` fan-out (a dispatcher, one entry per supplied count,
        // and an `app.many`) is 973 extra functions on the compiler's own
        // module and 7.4% of the toolkit check's CPU for 54 of those entries
        // executing at all (design/stack-allocation.md section 7.8).
        let add2 = fndef(
            "add2",
            &["a", "b"],
            app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
        );
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(app(g("add2"), vec![n(20)])),
            Box::new(app(v(10), vec![n(22)])),
        );
        let program = crate::rc::insert(
            main_prog(vec![add2], body),
            &crate::core::TargetProfile::native(),
        )
        .expect("ownership pass failed");
        let ir = emit(&program).expect("codegen emit failed");
        let stubs: Vec<&str> = ir
            .lines()
            .filter(|l| l.starts_with("define ") && l.contains(".app"))
            .collect();
        // One stub for each unit the program emits (`add2`, the arithmetic
        // primitive it calls, and `program.main`), and no second entry for
        // any of them.
        assert_eq!(
            stubs.len(),
            3,
            "one apply stub per unit, got:\n{}",
            stubs.join("\n")
        );
        assert!(stubs
            .iter()
            .any(|s| s.contains("\"add2.app\"(ptr %self, i64 %argc, ptr %argv)")));
        assert!(!ir.contains(".app.many"), "no `app.many` entry");
        assert!(!ir.contains(".app.0"), "no per-supplied-count entry");
        // The grow path copies the two sources directly rather than merging
        // into the fixed frame: a frame slot past the supplied words would
        // read `argv` out of bounds. So the merge's selects belong to the
        // saturated block, after the one saturation branch, and the frame
        // stays a single static `alloca` in the entry block.
        let start = ir.find("\"add2.app\"(ptr %self").expect("add2 stub");
        let body = &ir[start..];
        let body = &body[..body.find("\n}\n").expect("stub end")];
        let split = body.find("icmp uge i64").expect("one saturation test");
        let first_select = body.find("select i1").expect("the frame merge");
        assert!(
            split < first_select,
            "the merge runs on the saturated path only:\n{body}"
        );
        assert_eq!(
            body.matches("alloca [").count(),
            1,
            "one static frame, never a dynamic alloca:\n{body}"
        );
        assert!(
            !body.contains("alloca i64, i64"),
            "no dynamic alloca in a stub:\n{body}"
        );
        assert_eq!(
            body.matches("@memcpy").count(),
            2,
            "the grow path's two copies, and nothing else:\n{body}"
        );
    }

    #[test]
    fn instance_dictionary_and_method_call() {
        // class Add a { add : a -> a -> a }; Add_Maybe delegates to its
        // subdict. Methods run through DictProj + __apply.
        let add_int = fndef(
            "add_Int",
            &["a", "b"],
            app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
        );
        let mut add_maybe = fndef(
            "add_Maybe",
            &["d", "x", "y"],
            Core::Let(
                LocalId(10),
                Class::Obj,
                Box::new(Core::DictProj(Box::new(v(0)), 0, None, Class::Obj)),
                Box::new(app(v(10), vec![v(1), v(2)])),
            ),
        );
        add_maybe.repr.params = vec![Class::Obj, Class::Raw, Class::Raw];
        let body = Core::Let(
            LocalId(10),
            Class::Obj,
            Box::new(Core::InstDict(gid("Main.Add_Int"), vec![], vec![])),
            Box::new(Core::Let(
                LocalId(11),
                Class::Obj,
                Box::new(Core::InstDict(gid("Main.Add_Maybe"), vec![v(10)], vec![])),
                Box::new(Core::Let(
                    LocalId(12),
                    Class::Obj,
                    Box::new(Core::DictProj(Box::new(v(11)), 0, None, Class::Obj)),
                    Box::new(app(v(12), vec![n(20), n(22)])),
                )),
            )),
        );
        let mut prog = main_prog(vec![add_int, add_maybe], body);
        prog.instances = vec![
            InstanceDef {
                id: gid("Main.Add_Int"),
                method_count: 1,
                subdict_count: 0,
                methods: vec![gid("add_Int")],
                class: "Add".to_string(),
                tparams: 0,
                own: vec![0],
            },
            InstanceDef {
                id: gid("Main.Add_Maybe"),
                method_count: 1,
                subdict_count: 1,
                methods: vec![gid("add_Maybe")],
                class: "Add".to_string(),
                tparams: 0,
                own: vec![0],
            },
        ];
        assert_eq!(run("instance_dictionary_and_method_call", &prog), 42);
    }

    #[test]
    fn adt_case_bound_field_and_wildcard() {
        // case Some 5 of { Some x -> x + 37; _ -> 7 } = 42
        // case None  of { Some x -> x + 37; _ -> 7 } = 7
        // 42 + 7 = 49
        let some = ctor_fn("T.Some", &["x"], 0, 1);
        let none = ctor_fn("T.None", &[], 1, 0);
        let case_on = |scrut: Core, field: usize, tag: usize| Core::Case {
            scrut: Box::new(scrut),
            alts: vec![
                CAlt::new(
                    CPat::con(tag, vec![LocalId(field)]),
                    app(p(arith(ArithOp::Add)), vec![v(field), n(37)]),
                ),
                CAlt::new(CPat::Wild, n(7)),
            ],
        };
        let body = Core::Let(
            LocalId(10),
            Class::Obj,
            Box::new(app(ct("T.Some"), vec![n(5)])),
            Box::new(Core::Let(
                LocalId(11),
                Class::Raw,
                Box::new(case_on(v(10), 20, 0)),
                Box::new(Core::Let(
                    LocalId(12),
                    Class::Obj,
                    Box::new(ct("T.None")),
                    Box::new(Core::Let(
                        LocalId(13),
                        Class::Raw,
                        Box::new(case_on(v(12), 21, 0)),
                        Box::new(app(p(arith(ArithOp::Add)), vec![v(11), v(13)])),
                    )),
                )),
            )),
        );
        assert_eq!(
            run(
                "adt_case_bound_field_and_wildcard",
                &main_prog(vec![some, none], body)
            ),
            49
        );
    }

    #[test]
    fn ctor_as_value_through_apply() {
        // The constructor closure goes through __apply to build Some 5.
        let some = ctor_fn("T.Some", &["x"], 0, 1);
        let body = Core::Let(
            LocalId(10),
            Class::Obj,
            Box::new(ct("T.Some")),
            Box::new(Core::Let(
                LocalId(11),
                Class::Obj,
                Box::new(app(v(10), vec![n(5)])),
                Box::new(Core::Case {
                    scrut: Box::new(v(11)),
                    alts: vec![
                        CAlt::new(
                            CPat::con(0, vec![LocalId(20)]),
                            app(p(arith(ArithOp::Add)), vec![v(20), n(37)]),
                        ),
                        CAlt::new(CPat::Wild, n(7)),
                    ],
                }),
            )),
        );
        assert_eq!(
            run("ctor_as_value_through_apply", &main_prog(vec![some], body)),
            42
        );
    }

    #[test]
    fn case_var_pattern_binds_scrutinee() {
        // CPat::Var binds the whole owned scrutinee in the default branch.
        // Inspect its real payload; an object is not an I64 comparison operand.
        let some = ctor_fn("T.Some", &["x"], 0, 1);
        let body = Core::Let(
            LocalId(10),
            Class::Obj,
            Box::new(app(ct("T.Some"), vec![n(9)])),
            Box::new(Core::Case {
                scrut: Box::new(v(10)),
                alts: vec![CAlt::new(
                    CPat::Var(LocalId(20), Class::Obj),
                    Core::Case {
                        scrut: Box::new(v(20)),
                        alts: vec![
                            CAlt::new(
                                CPat::con(0, vec![LocalId(21)]),
                                Core::If(
                                    Box::new(app(p(cmp(CmpOp::Eq)), vec![v(21), n(9)])),
                                    CBranch::new(n(4)),
                                    CBranch::new(n(5)),
                                ),
                            ),
                            CAlt::new(CPat::Wild, n(5)),
                        ],
                    },
                )],
            }),
        );
        assert_eq!(
            run(
                "case_var_pattern_binds_scrutinee",
                &main_prog(vec![some], body)
            ),
            4
        );
    }

    #[test]
    fn zero_field_ctor_pointer_identity() {
        // Repeated construction of a zero-field constructor shares one block.
        let none = ctor_fn("T.None", &[], 1, 0);
        let body = Core::Let(
            LocalId(10),
            Class::Raw,
            Box::new(ct("T.None")),
            Box::new(Core::Let(
                LocalId(11),
                Class::Raw,
                Box::new(ct("T.None")),
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(10), v(11)])),
            )),
        );
        assert_eq!(
            run(
                "zero_field_ctor_pointer_identity",
                &main_prog(vec![none], body)
            ),
            1
        );
    }

    #[test]
    fn zero_arity_thunk_as_main() {
        assert_eq!(run("zero_arity_thunk_as_main", &main_prog(vec![], n(3))), 3);
    }

    #[test]
    fn tuple_block_projection() {
        // A boxed product allocates `{ words }` (its tag in the header);
        // `Proj` loads field words in order.
        let pair = ctor_fn("T.pair", &["a", "b"], 0, 2);
        let body = Core::Proj(
            Box::new(app(ct("T.pair"), vec![n(41), n(2)])),
            crate::core::ProjField::slot(1, Class::Raw),
        );
        assert_eq!(
            run("tuple_block_projection", &main_prog(vec![pair], body)),
            2
        );
    }

    #[test]
    fn fault_falls_through_to_the_runtime_fault_path() {
        // `Core::Fault` is the runtime fault path: exit 70.
        let body = Core::If(
            Box::new(Core::Bool(false)),
            CBranch::new(n(1)),
            CBranch::new(Core::Fault),
        );
        assert_eq!(run("fault_path", &main_prog(vec![], body)), 70);
    }

    // ------------------------------------------------ tail calls (D104)

    /// `sum acc n = if n == 0 then acc else sum (acc + n) (n - 1)`: a
    /// self tail call under `If` with cross-assigning arguments.
    fn sum_fn() -> FnDef {
        fndef(
            "T.sum",
            &["acc", "n"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(1), n(0)])),
                CBranch::new(v(0)),
                CBranch::new(app(
                    g("T.sum"),
                    vec![
                        app(p(arith(ArithOp::Add)), vec![v(0), v(1)]),
                        app(p(arith(ArithOp::Sub)), vec![v(1), n(1)]),
                    ],
                )),
            ),
        )
    }

    #[test]
    fn self_tail_recursion_runs_in_constant_stack() {
        // 1,000,000-deep self tail recursion sums 1..=1_000_000 in
        // constant stack (surface grammar local amendments section 9);
        // without the loop jump the process would die on the stack.
        let body = Core::If(
            Box::new(app(
                p(cmp(CmpOp::Eq)),
                vec![
                    app(g("T.sum"), vec![n(0), n(1_000_000)]),
                    n(500_000_500_000),
                ],
            )),
            CBranch::new(n(0)),
            CBranch::new(n(1)),
        );
        let prog = main_prog(vec![sum_fn()], body);
        assert_eq!(run("self_tail_recursion_constant_stack", &prog), 0);
        // The self call is a jump back to the loop header: the emitted
        // unit never calls itself.
        let ir = emit(&prog).expect("emit");
        let unit = ir
            .split("define i64 @\"T.sum\"")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("T.sum body");
        assert!(!unit.contains("call i64 @\"T.sum\""), "{unit}");
        assert!(unit.contains("br label %"), "{unit}");
    }

    #[test]
    fn mutual_tail_ping_pong_is_a_musttail_call() {
        // Two units tail-calling each other 1,000,001 times run in
        // constant stack through `musttail` calls (the signatures match
        // exactly).
        let ping = fndef(
            "T.ping",
            &["n"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                CBranch::new(n(0)),
                CBranch::new(app(
                    g("T.pong"),
                    vec![app(p(arith(ArithOp::Sub)), vec![v(0), n(1)])],
                )),
            ),
        );
        let pong = fndef(
            "T.pong",
            &["n"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                CBranch::new(n(1)),
                CBranch::new(app(
                    g("T.ping"),
                    vec![app(p(arith(ArithOp::Sub)), vec![v(0), n(1)])],
                )),
            ),
        );
        let prog = main_prog(vec![ping, pong], app(g("T.ping"), vec![n(1_000_001)]));
        assert_eq!(run("mutual_tail_ping_pong", &prog), 1);
        let ir = emit(&prog).expect("emit");
        assert!(
            ir.contains("musttail call i64 @\"T.pong\"")
                && ir.contains("musttail call i64 @\"T.ping\""),
            "mutual tail calls must be tail-marked:\n{ir}"
        );
    }

    #[test]
    fn a_tail_group_narrows_only_when_its_prototypes_agree() {
        // On a 32-bit target a `Nat`/`Int` or pointer position crosses at
        // `csize`. A mutual tail call is `musttail`, which needs the
        // caller's prototype to be the callee's exactly, so a group keeps
        // its narrowing only when every member narrows the same way, and
        // otherwise every member keeps the word: constant stack is the
        // language's guarantee (decision D104), not a narrower boundary.
        use crate::core::Crossing;
        let member = |id: &str, other: &str, base: i64| {
            fndef(
                id,
                &["n"],
                Core::If(
                    Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                    CBranch::new(n(base)),
                    CBranch::new(app(
                        g(other),
                        vec![app(p(arith(ArithOp::Sub)), vec![v(0), n(1)])],
                    )),
                ),
            )
        };
        let narrowed = |mut f: FnDef, c: Crossing| {
            f.meta.contract.params = vec![crate::core::ParamContract { narrow: Some(c), ..Default::default() }];
            f
        };
        let wasm = crate::core::TargetProfile::wasm32(crate::core::ArtifactProfile::CoreWasm);
        let int = Crossing::Csize { signed: true };
        let nat = Crossing::Csize { signed: false };

        // An `Int` and a `Nat` position extend differently and still share
        // the `i32` prototype, so the group narrows and stays `musttail`.
        let agree = main_prog(
            vec![
                narrowed(member("T.ping", "T.pong", 0), int),
                narrowed(member("T.pong", "T.ping", 1), nat),
            ],
            app(g("T.ping"), vec![n(10)]),
        );
        let ir = emit_for(&wasm, &agree).expect("emit");
        assert!(ir.contains("define i64 @\"T.ping\"(i32 %p0)"), "{ir}");
        assert!(ir.contains("musttail call i64 @\"T.pong\"(i32 "), "{ir}");
        assert!(ir.contains("musttail call i64 @\"T.ping\"(i32 "), "{ir}");

        // A `U8` position is `i8` and a pointer position `i32`: the two
        // prototypes differ, so neither member narrows.
        let differ = main_prog(
            vec![
                narrowed(member("T.ping", "T.pong", 0), Crossing::Bits { bits: 8, signed: false }),
                narrowed(member("T.pong", "T.ping", 1), nat),
            ],
            app(g("T.ping"), vec![n(10)]),
        );
        let ir = emit_for(&wasm, &differ).expect("emit");
        assert!(ir.contains("define i64 @\"T.ping\"(i64 %p0)"), "{ir}");
        assert!(ir.contains("musttail call i64 @\"T.pong\"(i64 "), "{ir}");
        assert!(ir.contains("musttail call i64 @\"T.ping\"(i64 "), "{ir}");

        // The same pair on a 64-bit target: `csize` is the slot, so only
        // the `U8` would narrow, and the group rule leaves both words.
        let ir = emit(&differ).expect("emit");
        assert!(ir.contains("define i64 @\"T.pong\"(i64 %p0)"), "{ir}");
        assert!(ir.contains("musttail call i64 @\"T.ping\"(i64 "), "{ir}");
    }

    #[test]
    fn self_tail_call_under_let_and_case() {
        // The tail emitter runs through `Let` and `Case` tails: the loop
        // is entered again from inside the case alternative.
        // `boxsum acc n = let m = n - 1; bx = box n in case bx of
        // box x -> if n == 0 then acc else boxsum (acc + x) m`;
        // from `boxsum 0 3` the recursion sums 3 + 2 + 1 = 6.
        let boxx = ctor_fn("T.box", &["x"], 0, 1);
        let body = Core::Let(
            LocalId(2),
            Class::Raw,
            Box::new(app(p(arith(ArithOp::Sub)), vec![v(1), n(1)])),
            Box::new(Core::Let(
                LocalId(4),
                Class::Obj,
                Box::new(app(ct("T.box"), vec![v(1)])),
                Box::new(Core::Case {
                    scrut: Box::new(v(4)),
                    alts: vec![CAlt {
                        pat: CPat::con(0, vec![LocalId(3)]),
                        weight: None,
                        body: Box::new(Core::If(
                            Box::new(app(p(cmp(CmpOp::Eq)), vec![v(1), n(0)])),
                            CBranch::new(v(0)),
                            CBranch::new(app(
                                g("T.boxsum"),
                                vec![app(p(arith(ArithOp::Add)), vec![v(0), v(3)]), v(2)],
                            )),
                        )),
                    }],
                }),
            )),
        );
        let f = fndef("T.boxsum", &["acc", "n"], body);
        let prog = main_prog(vec![boxx, f], app(g("T.boxsum"), vec![n(0), n(3)]));
        assert_eq!(run("self_tail_call_under_let_and_case", &prog), 6);
        let ir = emit(&prog).expect("emit");
        let unit = ir
            .split("define i64 @\"T.boxsum\"")
            .nth(1)
            .and_then(|rest| rest.split("\n}\n").next())
            .expect("T.boxsum body");
        assert!(!unit.contains("call i64 @\"T.boxsum\""), "{unit}");
    }

    #[test]
    fn mixed_arity_mutual_tail_calls_run_in_constant_stack() {
        // `f a b = if a == 0 then 0 else g (a - 1)` and `g x = f x x`
        // tail-call each other across arities. `musttail` needs identical
        // prototypes, so the group lowers under its widest arity: `g`'s
        // body runs in the two-word worker `T.g#tail` and both mutual
        // calls are `musttail` (surface grammar local amendments section
        // 9). From `f 1000000 0` the cycle is 2,000,001 calls deep, which
        // a frame per call could not reach on the 8 MiB test stack.
        let f = fndef(
            "T.f",
            &["a", "b"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                CBranch::new(n(0)),
                CBranch::new(app(
                    g("T.g"),
                    vec![app(p(arith(ArithOp::Sub)), vec![v(0), n(1)])],
                )),
            ),
        );
        let gg = fndef("T.g", &["x"], app(g("T.f"), vec![v(0), v(0)]));
        let prog = main_prog(vec![f, gg], app(g("T.f"), vec![n(1_000_000), n(0)]));
        assert_eq!(run("mixed_arity_mutual_tail", &prog), 0);
        let ir = emit(&prog).expect("emit");
        assert!(
            ir.contains("musttail call i64 @\"T.g#tail\"(")
                && ir.contains("musttail call i64 @\"T.f\"("),
            "mixed-arity mutual tail calls must be tail-marked:\n{ir}"
        );
        // `T.g` keeps its own one-word symbol for closures and exports.
        assert!(ir.contains("define i64 @\"T.g\"(i64 %p0)"), "{ir}");
        assert_eq!(crate::core::tail_widths(&prog).get("T.g"), Some(&2));
        let groups = crate::core::tail_report(&prog);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].members, vec!["T.f", "T.g"]);
        assert_eq!(groups[0].mark, "marked");
    }

    #[test]
    fn tail_call_fallback_on_unsaturated_call_is_reported() {
        // `h x = k x x x` over-applies `k a b`: an unsaturated tail call
        // goes through `__apply`, so the group falls back and the report
        // names the call (surface grammar local amendments section 9).
        let k = fndef("T.k", &["a", "b"], app(g("T.h"), vec![v(0)]));
        let h = fndef("T.h", &["x"], app(g("T.k"), vec![v(0), v(0), v(0)]));
        let prog = main_prog(vec![k, h], n(0));
        let groups = crate::core::tail_report(&prog);
        assert_eq!(groups.len(), 1, "{groups:?}");
        assert_eq!(groups[0].members, vec!["T.h", "T.k"]);
        assert_eq!(
            groups[0].mark,
            "fallback (tail call T.h -> T.k: applied to 3 arguments, unit arity 2)"
        );
    }

    #[test]
    fn tail_report_marks_recursive_groups() {
        // The self-recursive sum and the mutual ping-pong are groups of
        // the call graph, both marked; the non-recursive thunk forms no
        // group.
        let ping = fndef(
            "T.ping",
            &["n"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                CBranch::new(n(0)),
                CBranch::new(app(g("T.pong"), vec![v(0)])),
            ),
        );
        let pong = fndef(
            "T.pong",
            &["n"],
            Core::If(
                Box::new(app(p(cmp(CmpOp::Eq)), vec![v(0), n(0)])),
                CBranch::new(n(1)),
                CBranch::new(app(g("T.ping"), vec![v(0)])),
            ),
        );
        let prog = main_prog(
            vec![sum_fn(), ping, pong],
            app(g("T.sum"), vec![n(0), n(3)]),
        );
        let groups = crate::core::tail_report(&prog);
        assert_eq!(groups.len(), 2, "{groups:?}");
        assert_eq!(groups[0].members, vec!["T.ping", "T.pong"]);
        assert_eq!(groups[0].mark, "marked");
        assert_eq!(groups[1].members, vec!["T.sum"]);
        assert_eq!(groups[1].mark, "marked");
    }

    // ------------------------------------------------ mathematical Nat/Int and Bytes

    #[test]
    fn const_big_bignum_arithmetic() {
        // (10^19 + 7) - 10^19 = 7: the operands take the decimal parse
        // path (`cinch_int_from_decimal`), the fast path sees 7.
        let ten19 = cbig("10000000000000000000");
        let body = app(
            p(PrimOp::Int(IntOp::ToI64)),
            vec![app(
                p(PrimOp::Int(IntOp::Sub)),
                vec![
                    app(p(PrimOp::Int(IntOp::Add)), vec![ten19.clone(), cbig("7")]),
                    ten19,
                ],
            )],
        );
        assert_eq!(
            run("const_big_bignum_arithmetic", &main_prog(vec![], body)),
            7
        );
    }

    #[test]
    fn const_big_beyond_i64_range() {
        // Values beyond 2^63 parse and subtract exactly.
        let body = app(
            p(PrimOp::Int(IntOp::ToI64)),
            vec![app(
                p(PrimOp::Int(IntOp::Sub)),
                vec![cbig("18446744073709551617"), cbig("18446744073709551616")],
            )],
        );
        assert_eq!(
            run("const_big_beyond_i64_range", &main_prog(vec![], body)),
            1
        );
    }

    #[test]
    fn int_cmp_immediate_drives_equality() {
        // `intCmp` returns the raw -1/0/1 immediate: equality is `eqI64
        // (intCmp a b) 0`.
        let eq = |a: &str, b: &str| {
            app(
                p(cmp(CmpOp::Eq)),
                vec![
                    app(p(PrimOp::Int(IntOp::Cmp)), vec![cbig(a), cbig(b)]),
                    n(0),
                ],
            )
        };
        assert_eq!(
            run(
                "int_cmp_eq",
                &main_prog(vec![], eq("18446744073709551616", "18446744073709551616"))
            ),
            1
        );
        assert_eq!(
            run(
                "int_cmp_ne",
                &main_prog(vec![], eq("18446744073709551616", "18446744073709551617"))
            ),
            0
        );
    }

    #[test]
    fn nat_sub_below_zero_is_a_compiler_bug_trap() {
        // Source code cannot build this (`3 - 5` at `Nat` is refused); the
        // raw primitive still traps through the panic path (exit 71).
        let body = app(p(PrimOp::Nat(NatOp::Sub)), vec![cnat("3"), cnat("5")]);
        assert_eq!(
            run("nat_sub_faults_on_negative", &main_prog(vec![], body)),
            71
        );
    }

    #[test]
    fn nat_arith_and_cmp_round_trip() {
        // natToI64 (natMul (natAdd 2 3) 4) = 20; natCmp 3 5 is -1.
        let sum = app(p(PrimOp::Nat(NatOp::Add)), vec![cnat("2"), cnat("3")]);
        let scaled = app(p(PrimOp::Nat(NatOp::Mul)), vec![sum, cnat("4")]);
        let body = app(p(PrimOp::Nat(NatOp::ToI64)), vec![scaled]);
        assert_eq!(run("nat_arith_round_trip", &main_prog(vec![], body)), 20);
    }

    #[test]
    fn bytes_round_trip() {
        // bytesGet (bytesSet (bytesNew 3) 0 65) 0 = 65, and the length is 3.
        let body = Core::Let(
            LocalId(0),
            Class::Obj,
            Box::new(app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("3")])),
            Box::new(Core::Let(
                LocalId(1),
                Class::Raw,
                Box::new(app(
                    p(PrimOp::Bytes(BytesOp::Set)),
                    vec![v(0), cnat("0"), cnat("65")],
                )),
                Box::new(app(
                    p(PrimOp::Nat(NatOp::ToI64)),
                    vec![app(
                        p(PrimOp::Nat(NatOp::Add)),
                        vec![
                            app(p(PrimOp::Bytes(BytesOp::Get)), vec![v(0), cnat("0")]),
                            app(p(PrimOp::Bytes(BytesOp::Len)), vec![v(0)]),
                        ],
                    )],
                )),
            )),
        );
        assert_eq!(run("bytes_round_trip", &main_prog(vec![], body)), 68);
    }

    #[test]
    fn bytes_out_of_bounds_faults() {
        // Out-of-bounds indices and byte values outside 0..=255 fault
        // through the panic path (exit 71).
        let get_oob = Core::Let(
            LocalId(0),
            Class::Obj,
            Box::new(app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("3")])),
            Box::new(app(p(PrimOp::Bytes(BytesOp::Get)), vec![v(0), cnat("3")])),
        );
        assert_eq!(run("bytes_get_oob", &main_prog(vec![], get_oob)), 71);
        let set_bad = Core::Let(
            LocalId(0),
            Class::Obj,
            Box::new(app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("3")])),
            Box::new(app(
                p(PrimOp::Bytes(BytesOp::Set)),
                vec![v(0), cnat("0"), cnat("300")],
            )),
        );
        assert_eq!(run("bytes_set_bad_value", &main_prog(vec![], set_bad)), 71);
    }

    #[test]
    fn bytes_copy_is_a_memmove_and_compare_orders_contents() {
        // b = [1, 2, 0, 0]; bytesCopy b b 1 0 3 moves [1, 2, 0] one to the
        // right: [1, 1, 2, 0] (a byte-at-a-time forward copy would smear
        // to [1, 1, 1, 1]). The answer is b[2] * 10 + b[3] = 20.
        let set =
            |i: &str, x: &str| app(p(PrimOp::Bytes(BytesOp::Set)), vec![v(0), cnat(i), cnat(x)]);
        let get = |i: &str| app(p(PrimOp::Bytes(BytesOp::Get)), vec![v(0), cnat(i)]);
        let answer = app(
            p(PrimOp::Nat(NatOp::ToI64)),
            vec![app(
                p(PrimOp::Nat(NatOp::Add)),
                vec![
                    app(p(PrimOp::Nat(NatOp::Mul)), vec![get("2"), cnat("10")]),
                    get("3"),
                ],
            )],
        );
        let copy = app(
            p(PrimOp::Bytes(BytesOp::Copy)),
            vec![v(0), v(0), cnat("1"), cnat("0"), cnat("3")],
        );
        let body = Core::Let(
            LocalId(0),
            Class::Obj,
            Box::new(app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("4")])),
            Box::new(Core::Let(
                LocalId(1),
                Class::Raw,
                Box::new(set("0", "1")),
                Box::new(Core::Let(
                    LocalId(2),
                    Class::Raw,
                    Box::new(set("1", "2")),
                    Box::new(Core::Let(
                        LocalId(3),
                        Class::Raw,
                        Box::new(copy),
                        Box::new(answer),
                    )),
                )),
            )),
        );
        assert_eq!(run("bytes_copy_memmove", &main_prog(vec![], body)), 20);

        // bytesCompare (bytesNew 2) (bytesNew 3) is -1 (a proper prefix
        // orders first): exit status 255.
        let cmp = app(
            p(PrimOp::Bytes(BytesOp::Compare)),
            vec![
                app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("2")]),
                app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("3")]),
            ],
        );
        assert_eq!(run("bytes_compare_prefix", &main_prog(vec![], cmp)), 255);

        // A range past either end faults through the panic path.
        let oob = Core::Let(
            LocalId(0),
            Class::Obj,
            Box::new(app(p(PrimOp::Bytes(BytesOp::New)), vec![cnat("4")])),
            Box::new(app(
                p(PrimOp::Bytes(BytesOp::Copy)),
                vec![v(0), v(0), cnat("2"), cnat("0"), cnat("3")],
            )),
        );
        assert_eq!(run("bytes_copy_oob", &main_prog(vec![], oob)), 71);
    }
}

// ------------------------------------------- reified quotations (eval)

/// The opcodes of a quotation's reified program (supercompilation and
/// JIT specification section 7). This encoding and the runtime
/// evaluator's decoder (`compiler/runtime/src/eval.rs`) are one
/// specification: change them together. A program is a prefix-encoded
/// expression tree of `u64` words; operands are either immediate words
/// or relocated addresses ([`QWord::Addr`]).
mod quote_op {
    /// `[v]`: a raw word (machine scalar values).
    pub const INT: u64 = 1;
    /// `[nat, ndigits, digits...]`: a bignum decimal constant, its
    /// digits packed eight bytes per word, little endian.
    pub const BIG: u64 = 2;
    /// `[0|1]`.
    pub const BOOL: u64 = 3;
    pub const UNIT: u64 = 4;
    /// `[slot]`: the local's word.
    pub const LOCAL: u64 = 5;
    /// `[idx]`: the captured hole's quote block, evaluated in place
    /// (a `Code t` splice fills a `t` position).
    pub const HOLE: u64 = 6;
    /// `[slot, value, body]`.
    pub const LET: u64 = 7;
    /// `[cond, then, else]`.
    pub const IF: u64 = 8;
    /// `[scrutinee, ncon, cons..., default]`; each `con` is
    /// `[tag, nfields, fields..., body]` binding the block's field words
    /// (the scrutinee's tag is in its header's layout id),
    /// `default` is `[0]` (no default), `[1, body]` (wildcard) or
    /// `[2, slot, body]` (variable).
    pub const CASE: u64 = 9;
    /// `[fn, nargs, args...]`: apply through the closure apply-stub
    /// convention.
    pub const APP: u64 = 10;
    /// `[stub, arity, nenv, env...]`: allocate the closure block
    /// `{ stub; arity; nenv; env... }` of a hoisted lambda.
    pub const LAM: u64 = 11;
    /// `[desc, nfields, fields...]`: allocate the constructor block
    /// `{ fields... }` whose header carries descriptor `desc`.
    pub const CTOR: u64 = 12;
    /// `[value, mode, ..., counted]`: mode 0 `[slot]` loads the block's
    /// word at `8 * slot`; mode 1 `[offset, size, signed]` loads a
    /// boundary-layout scalar at `offset`; mode 2 `[offset]` loads a
    /// boundary-layout word at `offset`. `counted` (0 or 1) says the
    /// loaded word is a counted reference, dupped so the result is owned
    /// (the container is borrowed).
    pub const PROJ: u64 = 13;
    /// The program's own fault (exit 70).
    pub const FAULT: u64 = 14;
    /// Allocate `{ 0; 0; 0 }`: an empty memo cell.
    pub const LAZY_ALLOC: u64 = 15;
    /// `[cell-local, thunk]`: store the thunk into the cell; the value
    /// is the unit word.
    pub const LAZY_INIT: u64 = 16;
    /// `[cell, counted]`: run the memo cell's state machine; a counted
    /// value is dupped (the cell keeps its own reference).
    pub const DEMAND: u64 = 17;
    /// `[clo]`: a unit's closure word as a value.
    pub const UNITREF: u64 = 18;
    /// `[len, words...]`: a constant byte buffer, its data bytes packed
    /// eight per word little endian (the runtime evaluator boxes a
    /// fresh heap buffer per run).
    pub const CONST_BYTES: u64 = 19;
    /// `[cell-local, counted, value]`: replace the cell payload.
    pub const SET: u64 = 23;
    /// `[slot, body]`: add a reference to the counted binder's word.
    pub const DUP: u64 = 21;
    /// `[slot, body]`: release the counted binder's reference.
    pub const DROP: u64 = 22;
}

/// One reified word: an immediate, or a relocated address (a unit's static
/// closure block, or a lambda's apply stub).
#[derive(Clone, Debug)]
enum QWord {
    Imm(u64),
    Addr(String),
}

fn q_imm(out: &mut Vec<QWord>, v: u64) {
    out.push(QWord::Imm(v));
}

/// The `counted` operand of a loaded word of class `c` (an open
/// parameter class is never counted, as in the ownership pass).
fn q_counted(c: crate::repr::Class) -> u64 {
    u64::from(matches!(
        c,
        crate::repr::Class::Obj | crate::repr::Class::TObj
    ))
}

/// The closure constant of a unit (`@"<name>.clo"`).
fn q_clo_name(name: &str) -> String {
    format!("{name}.clo")
}

/// A unit used as a value: its closure word, or for a zero-arity unit
/// (a thunk) the result of applying it (mirroring
/// [`CoreEmitter::unit_value`]).
fn reify_unit_value(out: &mut Vec<QWord>, reg: &Registry, name: &str, arity: usize) {
    let clo = static_payload(&q_clo_name(name), reg.closure_desc(name));
    if arity == 0 {
        q_imm(out, quote_op::APP);
        q_imm(out, quote_op::UNITREF);
        out.push(QWord::Addr(clo));
        q_imm(out, 0);
    } else {
        q_imm(out, quote_op::UNITREF);
        out.push(QWord::Addr(clo));
    }
}

/// The largest `Nat` immediate payload on a target whose pointers are
/// `pointer_bytes` wide: `2^(csize-1) - 1`, the runtime's
/// `NAT_IMMEDIATE_MAX` (`compiler/runtime/src/lib.rs`, "Small `Nat`/`Int`
/// immediates"). The payload follows `csize` so that a proved `Nat` can
/// cross a call at that width.
fn nat_immediate_max(pointer_bytes: usize) -> u64 {
    (1u64 << (pointer_bytes * 8 - 1)) - 1
}

/// The immediate word of a `Nat`/`Int` literal, when its value is in the
/// immediate range of the runtime ABI: `v << 1 | 1` for a `Nat` in
/// `0..=2^(csize-1)-1` or an `Int` in `-2^(csize-2)..=2^(csize-2)-1`. The
/// runtime returns exactly this word for the same value, so a folded
/// literal is indistinguishable from a computed one.
fn immediate_word(pointer_bytes: usize, nat: bool, digits: &str) -> Option<i64> {
    let v: i128 = digits.parse().ok()?;
    let bits = pointer_bytes as u32 * 8;
    let (lo, hi) = if nat {
        (0, i128::from(nat_immediate_max(pointer_bytes)))
    } else {
        (-(1i128 << (bits - 2)), (1i128 << (bits - 2)) - 1)
    };
    if !(lo..=hi).contains(&v) {
        return None;
    }
    Some(((v << 1) | 1) as u64 as i64)
}

/// Reify `e` into the evaluator's word encoding. The body of a
/// quotation is closed runtime-side code (decision D43): its only
/// inputs are the captured hole blocks, so it never carries a join point
/// (join introduction runs after staging, and a jump never crosses a
/// quotation's retained body).
fn reify(out: &mut Vec<QWord>, e: &Core, reg: &Registry) -> Result<()> {
    match e {
        Core::Join { .. } | Core::Jump { .. } => {
            return err("internal error: a join point reached a quotation's reified body")
        }
        Core::Reuse { .. } => {
            return err("internal error: a cell reuse reached a quotation's reified body")
        }
        Core::Int(v) => {
            q_imm(out, quote_op::INT);
            q_imm(out, *v as u64);
        }
        Core::ConstBig { nat, digits } => match immediate_word(reg.pointer_bytes, *nat, digits) {
            // A small literal is already its runtime word: the raw `INT`
            // form, so the evaluator neither parses nor allocates it.
            Some(w) => {
                q_imm(out, quote_op::INT);
                q_imm(out, w as u64);
            }
            None => {
                q_imm(out, quote_op::BIG);
                q_imm(out, if *nat { 1 } else { 0 });
                q_imm(out, digits.len() as u64);
                let b = digits.as_bytes();
                for chunk in b.chunks(8) {
                    let mut w: u64 = 0;
                    for (i, c) in chunk.iter().enumerate() {
                        w |= (*c as u64) << (8 * i);
                    }
                    q_imm(out, w);
                }
            }
        },
        Core::Bool(b) => {
            q_imm(out, quote_op::BOOL);
            q_imm(out, if *b { 1 } else { 0 });
        }
        Core::Unit => q_imm(out, quote_op::UNIT),
        Core::Var(l) => {
            q_imm(out, quote_op::LOCAL);
            q_imm(out, l.0 as u64);
        }
        Core::SpliceVal(i) => {
            q_imm(out, quote_op::HOLE);
            q_imm(out, *i as u64);
        }
        Core::Let(x, _, v, body) => {
            q_imm(out, quote_op::LET);
            q_imm(out, x.0 as u64);
            reify(out, v, reg)?;
            reify(out, body, reg)?;
        }
        Core::RefSet(l, v, payload) => {
            q_imm(out, quote_op::SET);
            q_imm(out, l.0 as u64);
            q_imm(out, q_counted(*payload));
            reify(out, v, reg)?;
        }
        Core::If(c, t, f) => {
            q_imm(out, quote_op::IF);
            reify(out, c, reg)?;
            reify(out, &t.body, reg)?;
            reify(out, &f.body, reg)?;
        }
        Core::Case { scrut, alts } => {
            q_imm(out, quote_op::CASE);
            reify(out, scrut, reg)?;
            let mut cons = Vec::new();
            let mut default: Option<(Option<LocalId>, &Core)> = None;
            for a in alts {
                match &a.pat {
                    CPat::Con { tag, fields, .. } => {
                        if cons.iter().any(|(t, _, _)| t == tag) {
                            return err(format!("duplicate case tag {tag}"));
                        }
                        cons.push((*tag, fields, &a.body));
                    }
                    CPat::Var(x, _) => {
                        if default.is_some() {
                            return err("case has more than one default alternative");
                        }
                        default = Some((Some(*x), &a.body));
                    }
                    CPat::Wild => {
                        if default.is_some() {
                            return err("case has more than one default alternative");
                        }
                        default = Some((None, &a.body));
                    }
                }
            }
            q_imm(out, cons.len() as u64);
            for (tag, fields, body) in cons {
                q_imm(out, tag as u64);
                q_imm(out, fields.len() as u64);
                for f in fields {
                    q_imm(out, f.0 as u64);
                }
                reify(out, body, reg)?;
            }
            match default {
                None => q_imm(out, 0),
                Some((None, body)) => {
                    q_imm(out, 1);
                    reify(out, body, reg)?;
                }
                Some((Some(x), body)) => {
                    q_imm(out, 2);
                    q_imm(out, x.0 as u64);
                    reify(out, body, reg)?;
                }
            }
        }
        // A unit used as a value at arity 0 is already its call (see
        // `reify_unit_value`), as `emit_app` calls a known nullary unit
        // directly: applying the result again would treat the value (a
        // constructor block, say) as a closure.
        Core::App(f, args) if args.is_empty() && reify_unit_arity(f, reg)? == Some(0) => {
            reify(out, f, reg)?;
        }
        Core::App(f, args) => {
            q_imm(out, quote_op::APP);
            reify_value(out, f, reg)?;
            q_imm(out, args.len() as u64);
            for a in args {
                reify(out, a, reg)?;
            }
        }
        Core::Lam { fn_id, env, .. } => {
            let arity = reg.arity_of(&fn_id.0)?;
            if env.len() > arity {
                return err(format!(
                    "reified lambda {fn_id} captures {} values but its unit has arity {arity}",
                    env.len()
                ));
            }
            q_imm(out, quote_op::LAM);
            out.push(QWord::Addr(quote(&format!("{}.app", fn_id.0))));
            q_imm(out, reg.closure_desc(&fn_id.0));
            q_imm(out, arity as u64);
            q_imm(out, env.len() as u64);
            for a in env {
                reify(out, a, reg)?;
            }
        }
        Core::DictProj(d, idx, _, class) => {
            // A dictionary is the `{ nmeth; meth0; ... }` block of the
            // `mk` units: method `idx` is the block's word `1 + idx`.
            q_imm(out, quote_op::PROJ);
            reify(out, d, reg)?;
            q_imm(out, 0);
            q_imm(out, *idx as u64 + 1);
            q_imm(out, q_counted(*class));
        }
        Core::Proj(d, f) => {
            q_imm(out, quote_op::PROJ);
            reify(out, d, reg)?;
            match &f.access {
                None => {
                    q_imm(out, 0);
                    q_imm(out, f.slot as u64);
                }
                Some(fa) => match &fa.shape {
                    crate::core::FieldShape::Word { .. } => {
                        q_imm(out, 2);
                        q_imm(out, fa.offset);
                    }
                    crate::core::FieldShape::Scalar { size, signed } => {
                        q_imm(out, 1);
                        q_imm(out, fa.offset);
                        q_imm(out, *size);
                        q_imm(out, if *signed { 1 } else { 0 });
                    }
                    crate::core::FieldShape::Inline { .. } => {
                        return err(
                            "a quotation cannot reify a nested boundary-layout aggregate yet \
                             (attribute annotations specification section 4.3); keep the \
                             projected field a scalar or a word",
                        )
                    }
                },
            }
            q_imm(out, q_counted(f.class));
        }
        Core::InstDict(id, subs, _) => {
            let info = reg
                .insts
                .get(&id.0)
                .ok_or_else(|| crate::Error::plain(format!("unknown instance {id}")))?;
            if subs.len() != info.subdict_count {
                return err(format!(
                    "instance {id} expects {} subdicts, got {}",
                    info.subdict_count,
                    subs.len()
                ));
            }
            q_imm(out, quote_op::APP);
            q_imm(out, quote_op::UNITREF);
            out.push(QWord::Addr(static_payload(&q_clo_name(&format!("{}.mk", id.0)),
                                              reg.closure_desc(&format!("{}.mk", id.0)))));
            q_imm(out, subs.len() as u64);
            for s in subs {
                reify(out, s, reg)?;
            }
        }
        Core::Fault => q_imm(out, quote_op::FAULT),
        Core::BoxAlloc | Core::BoxInit(..) | Core::BoxGet(..) | Core::BoxClear(..) => {
            return Err(crate::Error::plain(
                "a recursive `letrec` value cannot be reified into a quotation body",
            ))
        }
        Core::LazyAlloc => q_imm(out, quote_op::LAZY_ALLOC),
        Core::LazyInit(cell, thunk) => {
            q_imm(out, quote_op::LAZY_INIT);
            q_imm(out, cell.0 as u64);
            reify(out, thunk, reg)?;
        }
        Core::Demand(cell, class) => {
            q_imm(out, quote_op::DEMAND);
            reify(out, cell, reg)?;
            q_imm(out, q_counted(*class));
        }
        Core::Dup(x, _, body) | Core::Drop(x, _, body) => {
            q_imm(
                out,
                if matches!(e, Core::Dup(..)) {
                    quote_op::DUP
                } else {
                    quote_op::DROP
                },
            );
            q_imm(out, x.0 as u64);
            reify(out, body, reg)?;
        }
        // A quotation body's operations are placed without drop
        // specialisation (`crate::rc`), so the evaluator has no form for it.
        Core::Release { .. } => return err("internal error: a quotation body holds a drop specialisation"),
        Core::Global(g, _) => {
            let arity = reg.arity_of(&g.0)?;
            reify_unit_value(out, reg, &g.0, arity);
        }
        Core::Ctor(g, _) => {
            let arity = reg.arity_of(&g.0)?;
            reify_unit_value(out, reg, &g.0, arity);
        }
        Core::Prim(op) => reify_unit_value(out, reg, &op.symbol(), op.arity()),
        Core::Foreign(n) => {
            let p = reg
                .foreigns
                .get(n)
                .ok_or_else(|| crate::Error::plain(format!("unknown foreign binding {n}")))?;
            reify_unit_value(out, reg, n, p.arity());
        }
        Core::Quote { .. } | Core::DeclsQuote { .. } | Core::SyntaxLit(..) | Core::StrLit(_) => {
            return err("a quotation body cannot contain staging values \
                 (a nested quotation is not reified yet)")
        }
        Core::ConstString { ctor, bytes } => {
            // The constant's stream form: the representation
            // constructor's block over a constant buffer. The evaluator
            // materialises a fresh heap buffer per run (the compiled
            // literal is the shared data-section constant instead).
            let fields = reg
                .ctors
                .get(&ctor.0)
                .map(|(_, f)| *f)
                .ok_or_else(|| crate::Error::plain(format!("unknown constructor {ctor}")))?;
            if fields != 1 {
                return err(format!(
                    "the string constant's constructor {ctor} takes {fields} fields, expected 1"
                ));
            }
            q_imm(out, quote_op::CTOR);
            q_imm(out, reg.ctor_desc(&ctor.0)?);
            q_imm(out, 1);
            q_imm(out, quote_op::CONST_BYTES);
            q_imm(out, bytes.len() as u64);
            for chunk in bytes.chunks(8) {
                let mut w: u64 = 0;
                for (i, &b) in chunk.iter().enumerate() {
                    w |= u64::from(b) << (8 * i);
                }
                q_imm(out, w);
            }
        }
        Core::ConstBytes(_) | Core::ConstSym(_) => {
            return err(
                "a constant `Bytes` buffer is not reified into a quotation body yet \
                 (the literal fold leaves class calls inside quotation bodies)",
            )
        }
    }
    Ok(())
}

/// The arity of a unit head (a global, constructor, primitive or
/// foreign binding), or `None` for a computed expression.
fn reify_unit_arity(e: &Core, reg: &Registry) -> Result<Option<usize>> {
    Ok(match e {
        Core::Global(g, _) | Core::Ctor(g, _) => Some(reg.arity_of(&g.0)?),
        Core::Prim(op) => Some(op.arity()),
        Core::Foreign(n) => Some(
            reg.foreigns
                .get(n)
                .ok_or_else(|| crate::Error::plain(format!("unknown foreign binding {n}")))?
                .arity(),
        ),
        _ => None,
    })
}

/// A value position: a unit used as a value, or a computed expression.
fn reify_value(out: &mut Vec<QWord>, e: &Core, reg: &Registry) -> Result<()> {
    reify(out, e, reg)
}

/// Emit the runtime construction of a quotation value: the reified
/// program of `body` as a private constant, and the block
/// `{ KIND_CODE; code; len; holes... }` the runtime evaluator consumes
/// (`compiler/runtime/src/eval.rs` documents the layout). The hole
/// expressions evaluate in the enclosing scope and their words are
/// captured into the block.
fn emit_quote_block(e: &mut CoreEmitter, body: &Core, holes: &[Core]) -> Result<String> {
    let mut words: Vec<QWord> = Vec::new();
    reify(&mut words, body, e.reg)?;
    let gname = e.b.dec_name(&format!("{}.quote", e.unit));
    let items: Vec<String> = words
        .iter()
        .map(|w| match w {
            QWord::Imm(v) => format!("i64 {v}"),
            QWord::Addr(p) => format!("i64 ptrtoint (ptr {p} to i64)"),
        })
        .collect();
    e.b.global(format!(
        "@{} = private constant [{} x i64] [ {} ]",
        quote(&gname),
        items.len(),
        items.join(", ")
    ));
    let mut hv: Vec<String> = Vec::with_capacity(holes.len());
    for h in holes {
        hv.push(e.emit_expr(h)?);
    }
    let n = hv.len();
    let bytes = (4 + n) * 8;
    let p = e.b.reg();
    e.b.line(&alloc_call(&p, bytes, block::KIND_CODE));
    // The code block's kind word (runtime ABI `KIND_CODE`).
    e.b.store_at(&p, 0, "3");
    e.b.store_at(
        &p,
        8,
        &format!("ptrtoint (ptr @{g} to i64)", g = quote(&gname)),
    );
    e.b.store_at(&p, 16, &items.len().to_string());
    e.b.store_at(&p, 24, &n.to_string());
    for (i, h) in hv.iter().enumerate() {
        e.b.store_at(&p, 32 + 8 * i, h);
    }
    let w = e.b.reg();
    e.b.line(&format!("{w} = ptrtoint ptr {p} to i64"));
    Ok(w)
}

// ------------------------------------- canonical ABI glue (adapters)

/// The core signature of a canonical binding (WebAssembly and WASI
/// specification section 4): the flattened parameter types (the
/// return-area pointer is the last one when the result crosses in
/// memory) and the single core result otherwise.
fn canon_sig(call: &crate::core::CanonCall) -> (String, Vec<String>) {
    use crate::core::CanonType;
    let params: Vec<String> = call
        .core_params
        .iter()
        .map(|t| t.spelling().to_string())
        .collect();
    let ret = match call.core_results.first() {
        Some(CanonType::I32) => "i32".to_string(),
        Some(CanonType::I64) => "i64".to_string(),
        Some(CanonType::F32) => "f32".to_string(),
        Some(CanonType::F64) => "f64".to_string(),
        None => "void".to_string(),
    };
    (ret, params)
}

/// Lower one safe word to its flat core values (the canonical
/// `lower` direction of one parameter).
fn canon_word_to_flat(
    b: &mut Builder,
    reg: &Registry,
    step: &crate::core::CanonStep,
    w: &str,
) -> Result<Vec<(crate::core::CanonType, String)>> {
    use crate::core::{CanonStep, CanonType};
    Ok(match step {
        CanonStep::Word { core, .. } => match core {
            CanonType::I32 => {
                let v = b.reg();
                b.line(&format!("{v} = trunc i64 {w} to i32"));
                vec![(CanonType::I32, v)]
            }
            CanonType::I64 => vec![(CanonType::I64, w.to_string())],
            CanonType::F32 => {
                let i = b.reg();
                b.line(&format!("{i} = trunc i64 {w} to i32"));
                let f = b.reg();
                b.line(&format!("{f} = bitcast i32 {i} to float"));
                vec![(CanonType::F32, f)]
            }
            CanonType::F64 => {
                let f = b.reg();
                b.line(&format!("{f} = bitcast i64 {w} to double"));
                vec![(CanonType::F64, f)]
            }
        },
        CanonStep::Bytes { wrapped } => {
            let ww = if *wrapped {
                let p = b.reg();
                b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
                b.load_at(&p, 0)
            } else {
                w.to_string()
            };
            let len = rt_call(b, "cinch_bytes_len", &[format!("i64 {ww}")]);
            let src = rt_call(b, "cinch_bytes_data", &[format!("i64 {ww}")]);
            let dst = b.reg();
            b.line(&alloc_call(&dst, &len, block::KIND_RAW));
            let sp = b.reg();
            b.line(&format!("{sp} = inttoptr i64 {src} to ptr"));
            let cp = b.reg();
            b.line(&format!(
                "{cp} = call ptr @memcpy(ptr {dst}, ptr {sp}, i64 {len})"
            ));
            let dp = b.reg();
            b.line(&format!("{dp} = ptrtoint ptr {dst} to i32"));
            let lp = b.reg();
            b.line(&format!("{lp} = trunc i64 {len} to i32"));
            vec![(CanonType::I32, dp), (CanonType::I32, lp)]
        }
        CanonStep::Handle | CanonStep::Tag | CanonStep::FlagField { .. } => {
            let raw = if matches!(step, CanonStep::Handle) {
                w.to_string()
            } else {
                let p = b.reg();
                b.line(&format!("{p} = inttoptr i64 {w} to ptr"));
                match step {
                    // An enum's constructor tag, from its header.
                    CanonStep::Tag => ctor_tag(b, &p, reg.escapes()),
                    // A flags block's one field.
                    _ => b.load_at(&p, 0),
                }
            };
            let v = b.reg();
            b.line(&format!("{v} = trunc i64 {raw} to i32"));
            vec![(CanonType::I32, v)]
        }
    })
}

/// Lift one canonical memory shape back to its safe value word.
fn canon_lift_shape(
    b: &mut Builder,
    reg: &Registry,
    shape: &crate::core::CanonShape,
    base: &str,
) -> Result<String> {
    use crate::core::CanonShape;
    Ok(match shape {
        CanonShape::Unit => "0".to_string(),
        CanonShape::Scalar {
            offset,
            size,
            signed,
        } => {
            if *size == 0 {
                "0".to_string()
            } else {
                let p = b.gep(base, *offset as usize);
                let v = b.reg();
                b.line(&format!("{v} = load i{}, ptr {p}, align 1", size * 8));
                if *size == 8 {
                    v
                } else {
                    let w = b.reg();
                    let op = if *signed { "sext" } else { "zext" };
                    b.line(&format!("{w} = {op} i{} {v} to i64", size * 8));
                    w
                }
            }
        }
        CanonShape::Bytes { offset, wrapped } => {
            let pp = b.gep(base, *offset as usize);
            let p32 = b.reg();
            b.line(&format!("{p32} = load i32, ptr {pp}, align 1"));
            let lp = b.gep(base, (*offset + 4) as usize);
            let l32 = b.reg();
            b.line(&format!("{l32} = load i32, ptr {lp}, align 1"));
            let p64 = b.reg();
            b.line(&format!("{p64} = zext i32 {p32} to i64"));
            let l64 = b.reg();
            b.line(&format!("{l64} = zext i32 {l32} to i64"));
            let w = rt_call(
                b,
                "cinch_bytes_from_data",
                &[format!("i64 {p64}"), format!("i64 {l64}")],
            );
            if *wrapped {
                string_block_word(b, reg, &w)
            } else {
                w
            }
        }
        CanonShape::Handle { offset } => {
            let p = b.gep(base, *offset as usize);
            let v = b.reg();
            b.line(&format!("{v} = load i32, ptr {p}, align 1"));
            let w = b.reg();
            b.line(&format!("{w} = zext i32 {v} to i64"));
            w
        }
        CanonShape::Ctor {
            tag,
            tag_at,
            fields,
        } => {
            let mut vals = Vec::with_capacity(fields.len());
            for f in fields {
                vals.push(canon_lift_shape(b, reg, f, base)?);
            }
            let words = fields.len() as u64;
            let map: Vec<Class> = fields.iter().map(canon_class).collect();
            match tag_at {
                Some((offset, size)) => {
                    let p = b.gep(base, *offset as usize);
                    let t = b.reg();
                    b.line(&format!("{t} = load i{}, ptr {p}, align 1", size * 8));
                    let tw = b.reg();
                    b.line(&format!("{tw} = zext i{} {t} to i64", size * 8));
                    let desc = dynamic_shape_desc(b, reg, &map, &tw);
                    let blk = b.reg();
                    b.line(&format!(
                        "{blk} = call ptr @cinch_alloc_ctor(i64 {}, i64 {desc})",
                        8 * words
                    ));
                    for (i, v) in vals.iter().enumerate() {
                        b.store_at(&blk, 8 * i, v);
                    }
                    let w = b.reg();
                    b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
                    w
                }
                None => ctor_block_word(b, reg.shape_desc(&map, *tag), &vals),
            }
        }
    })
}

/// The class of the safe word a canonical shape lifts to: a byte payload
/// and an ADT are fresh blocks, a scalar, a handle and the unit word are
/// not counted.
fn canon_class(shape: &crate::core::CanonShape) -> Class {
    use crate::core::CanonShape;
    match shape {
        CanonShape::Bytes { .. } | CanonShape::Ctor { .. } => Class::Obj,
        CanonShape::Scalar { .. } | CanonShape::Handle { .. } | CanonShape::Unit => Class::Raw,
    }
}

/// Store one safe value word into its canonical memory shape (the
/// `lower` direction over the return area).
fn canon_lower_shape(
    b: &mut Builder,
    shape: &crate::core::CanonShape,
    base: &str,
    w: &str,
) -> Result<()> {
    use crate::core::CanonShape;
    match shape {
        CanonShape::Unit => {}
        CanonShape::Scalar {
            offset,
            size,
            signed,
        } => {
            if *size > 0 {
                let v = if *size == 8 {
                    w.to_string()
                } else {
                    let r = b.reg();
                    b.line(&format!("{r} = trunc i64 {w} to i{}", size * 8));
                    r
                };
                let p = b.gep(base, *offset as usize);
                b.line(&format!("store i{} {v}, ptr {p}, align 1", size * 8));
                let _ = signed;
            }
        }
        CanonShape::Bytes { offset, wrapped } => {
            let ww = if *wrapped {
                let bp = b.reg();
                b.line(&format!("{bp} = inttoptr i64 {w} to ptr"));
                b.load_at(&bp, 0)
            } else {
                w.to_string()
            };
            let len = rt_call(b, "cinch_bytes_len", &[format!("i64 {ww}")]);
            let src = rt_call(b, "cinch_bytes_data", &[format!("i64 {ww}")]);
            let dst = b.reg();
            b.line(&alloc_call(&dst, &len, block::KIND_RAW));
            let sp = b.reg();
            b.line(&format!("{sp} = inttoptr i64 {src} to ptr"));
            let r = b.reg();
            b.line(&format!(
                "{r} = call ptr @memcpy(ptr {dst}, ptr {sp}, i64 {len})"
            ));
            let dp = b.reg();
            b.line(&format!("{dp} = ptrtoint ptr {dst} to i32"));
            let pp = b.gep(base, *offset as usize);
            b.line(&format!("store i32 {dp}, ptr {pp}, align 1"));
            let l32 = b.reg();
            b.line(&format!("{l32} = trunc i64 {len} to i32"));
            let lp = b.gep(base, (*offset + 4) as usize);
            b.line(&format!("store i32 {l32}, ptr {lp}, align 1"));
        }
        CanonShape::Handle { offset }
        | CanonShape::Ctor {
            tag_at: Some((offset, _)),
            ..
        } => {
            let v = b.reg();
            b.line(&format!("{v} = trunc i64 {w} to i32"));
            let p = b.gep(base, *offset as usize);
            b.line(&format!("store i32 {v}, ptr {p}, align 1"));
        }
        CanonShape::Ctor {
            tag_at: None,
            fields,
            ..
        } => {
            // A block word: its tag and fields store into the payload.
            let blk = b.reg();
            b.line(&format!("{blk} = inttoptr i64 {w} to ptr"));
            for (i, f) in fields.iter().enumerate() {
                let fw = b.load_at(&blk, 8 * i);
                canon_lower_shape(b, f, base, &fw)?;
            }
        }
    }
    Ok(())
}

/// The canonical-ABI import wrapper (WebAssembly and WASI specification
/// section 4): lower the safe words through the canonical flattening,
/// call the import symbol, and lift the result from the return area.
fn canon_import_body(b: &mut Builder, reg: &Registry, plan: &ForeignPlan) -> Result<()> {
    let call = plan
        .canon
        .as_ref()
        .ok_or_else(|| crate::Error::plain("a canonical wrapper without a canonical plan"))?;
    if let Some(g) = &call.glue_gap {
        return err(format!(
            "the canonical ABI glue for {} does not marshal {g} (WebAssembly and WASI specification section 4)",
            plan.symbol
        ));
    }
    let mut flats: Vec<String> = Vec::new();
    for (i, steps) in call.params.iter().enumerate() {
        for step in steps {
            for v in canon_word_to_flat(b, reg, step, &format!("%p{i}"))? {
                flats.push(format!("{} {}", v.0.spelling(), v.1));
            }
        }
    }
    let retbuf = if call.ret.is_some() {
        let size = call.ret.as_ref().map(|r| r.size).unwrap_or(0);
        let align = call.ret.as_ref().map(|r| r.align).unwrap_or(1).max(1);
        let p = b.alloca_bytes(size, align);
        b.line(&format!(
            "call void @llvm.memset.p0.i64(ptr {p}, i8 0, i64 {size}, i1 false)"
        ));
        let w = b.reg();
        b.line(&format!("{w} = ptrtoint ptr {p} to i32"));
        flats.push(format!("i32 {w}"));
        Some(p)
    } else {
        None
    };
    let (ret_ty, _) = canon_sig(call);
    let call_res = if ret_ty == "void" {
        b.line(&format!(
            "call {ret_ty} @{}({})",
            quote(&plan.symbol),
            flats.join(", ")
        ));
        None
    } else {
        let r = b.reg();
        b.line(&format!(
            "{r} = call {ret_ty} @{}({})",
            quote(&plan.symbol),
            flats.join(", ")
        ));
        Some(r)
    };
    // The safe result word.
    let out = if let Some(ret) = &call.ret {
        let base = retbuf.as_deref().expect("return area");
        let disc_p = b.gep(base, 0);
        let disc8 = b.reg();
        b.line(&format!("{disc8} = load i8, ptr {disc_p}, align 1"));
        let disc = b.reg();
        b.line(&format!("{disc} = zext i8 {disc8} to i64"));
        let slot = b.alloca_words(1);
        let join = b.label();
        let mut labels = Vec::new();
        for _ in &ret.cases {
            labels.push(b.label());
        }
        // The comparison chain: each case test against a running
        // fallthrough.
        let mut prev_fall = b.label();
        for (i, case) in ret.cases.iter().enumerate() {
            let test = b.reg();
            b.line(&format!("{test} = icmp eq i64 {disc}, {}", case.disc));
            b.line(&format!(
                "br i1 {test}, label %{}, label %{prev_fall}",
                labels[i]
            ));
            b.start(&labels[i]);
            let payload = canon_lift_shape(b, reg, &case.payload, base)?;
            // A zero-field constructor is its header alone.
            let (fields, map) = if matches!(case.payload, crate::core::CanonShape::Unit) {
                (Vec::new(), Vec::new())
            } else {
                (vec![payload], vec![canon_class(&case.payload)])
            };
            let w = ctor_block_word(b, reg.shape_desc(&map, case.safe_tag), &fields);
            b.store_at(&slot, 0, &w);
            b.line(&format!("br label %{join}"));
            if i + 1 < ret.cases.len() {
                let next_fall = b.label();
                b.start(&prev_fall);
                prev_fall = next_fall;
            }
        }
        b.start(&prev_fall);
        // No case matched: a discriminant the typing forbids faults.
        b.line("call void @__cinch_blackhole()");
        b.line("unreachable");
        b.start(&join);
        b.load_at(&slot, 0)
    } else if let Some(shape) = &call.ret_flat {
        let raw = call_res.clone().unwrap_or_default();
        let v = match call.core_results.first() {
            Some(crate::core::CanonType::I32) => {
                let r = b.reg();
                b.line(&format!("{r} = zext i32 {raw} to i64"));
                r
            }
            Some(crate::core::CanonType::F64) => {
                let r = b.reg();
                b.line(&format!("{r} = bitcast double {raw} to i64"));
                r
            }
            Some(crate::core::CanonType::F32) => {
                let i = b.reg();
                b.line(&format!("{i} = bitcast float {raw} to i32"));
                let r = b.reg();
                b.line(&format!("{r} = zext i32 {i} to i64"));
                r
            }
            _ => raw,
        };
        match shape {
            crate::core::CanonShape::Unit => "0".to_string(),
            crate::core::CanonShape::Ctor {
                tag_at: Some(_), ..
            } => {
                // An enum value: its block's header carries the
                // discriminant as its tag.
                let desc = dynamic_shape_desc(b, reg, &[], &v);
                let blk = b.reg();
                b.line(&format!(
                    "{blk} = call ptr @cinch_alloc_ctor(i64 0, i64 {desc})"
                ));
                let w = b.reg();
                b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
                w
            }
            crate::core::CanonShape::Ctor {
                tag,
                tag_at: None,
                fields,
            } => {
                // One flat core value crosses: a scalar in the first
                // field, and nothing else is stored, so nothing is
                // counted.
                let blk = b.reg();
                let words = fields.len() as u64;
                b.line(&ctor_alloc_call(
                    &blk,
                    8 * words,
                    reg.raw_shape_desc(words, *tag),
                ));
                if !fields.is_empty() {
                    b.store_at(&blk, 0, &v);
                }
                let w = b.reg();
                b.line(&format!("{w} = ptrtoint ptr {blk} to i64"));
                w
            }
            _ => v,
        }
    } else {
        "0".to_string()
    };
    ret(b, &out);
    Ok(())
}

/// The canonical-ABI export trampoline: lift the flat core values into
/// safe words, call the implementation unit, lower the result into the
/// return area.
fn canon_export_entry(out: &mut String, plan: &ForeignPlan, reg: &Registry) -> Result<()> {
    let call = plan
        .canon
        .as_ref()
        .ok_or_else(|| crate::Error::plain("a canonical trampoline without a canonical plan"))?;
    if let Some(g) = &call.glue_gap {
        return err(format!(
            "the canonical ABI glue for {} does not marshal {g} (WebAssembly and WASI specification section 4)",
            plan.symbol
        ));
    }
    if !reg.arity.contains_key(&plan.unit) {
        // The program supplies the implementation later (or not at all,
        // which artifact admission rejects): keep the IR well formed.
        let arity = plan.params.len();
        let params: Vec<String> = (0..arity).map(|i| format!("i64 %p{i}")).collect();
        out.push_str(&format!(
            "declare i64 @{}({})\n",
            quote(&plan.unit),
            params.join(", ")
        ));
    }
    let (_, params) = canon_sig(call);
    let header = format!(
        "define void @{}({})",
        quote(&plan.symbol),
        params
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{t} %a{i}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let mut b = Builder::new();
    let mut words: Vec<String> = Vec::new();
    let mut ai = 0usize;
    for steps in &call.params {
        let mut vals: Vec<String> = Vec::new();
        for step in steps {
            let n = if matches!(step, crate::core::CanonStep::Bytes { .. }) {
                2
            } else {
                1
            };
            for _ in 0..n {
                vals.push(format!("%a{ai}"));
                ai += 1;
            }
        }
        // The flat values of one parameter lift to one safe word.
        let mut vi = 0;
        let mut steps_iter = steps.iter();
        let step = steps_iter.next().expect("at least one step");
        use crate::core::{CanonStep, CanonType};
        let w = match step {
            CanonStep::Word { core, .. } => {
                let v = &vals[vi];
                vi += 1;
                match core {
                    CanonType::I64 => v.clone(),
                    CanonType::I32 => {
                        let r = b.reg();
                        b.line(&format!("{r} = zext i32 {v} to i64"));
                        r
                    }
                    CanonType::F32 => {
                        let i = b.reg();
                        b.line(&format!("{i} = bitcast float {v} to i32"));
                        let r = b.reg();
                        b.line(&format!("{r} = zext i32 {i} to i64"));
                        r
                    }
                    CanonType::F64 => {
                        let r = b.reg();
                        b.line(&format!("{r} = bitcast double {v} to i64"));
                        r
                    }
                }
            }
            CanonStep::Bytes { wrapped } => {
                let p = &vals[vi];
                let l = &vals[vi + 1];
                vi += 2;
                let p64 = b.reg();
                b.line(&format!("{p64} = zext i32 {p} to i64"));
                let l64 = b.reg();
                b.line(&format!("{l64} = zext i32 {l} to i64"));
                let w = rt_call(
                    &mut b,
                    "cinch_bytes_from_data",
                    &[format!("i64 {p64}"), format!("i64 {l64}")],
                );
                if *wrapped {
                    string_block_word(&mut b, reg, &w)
                } else {
                    w
                }
            }
            CanonStep::Handle | CanonStep::Tag | CanonStep::FlagField { .. } => {
                let v = &vals[vi];
                vi += 1;
                match step {
                    CanonStep::Handle => {
                        let r = b.reg();
                        b.line(&format!("{r} = zext i32 {v} to i64"));
                        r
                    }
                    _ => {
                        // An enum block whose header carries the
                        // discriminant as its tag, or a flags block whose
                        // one field holds the bits.
                        let t = b.reg();
                        b.line(&format!("{t} = zext i32 {v} to i64"));
                        if matches!(step, CanonStep::Tag) {
                            let desc = dynamic_shape_desc(&mut b, reg, &[], &t);
                            let blk = b.reg();
                            b.line(&format!(
                                "{blk} = call ptr @cinch_alloc_ctor(i64 0, i64 {desc})"
                            ));
                            let r = b.reg();
                            b.line(&format!("{r} = ptrtoint ptr {blk} to i64"));
                            r
                        } else {
                            ctor_block_word(&mut b, reg.shape_desc(&[Class::Raw], 0), &[t])
                        }
                    }
                }
            }
        };
        words.push(w);
    }
    let ret_area = call
        .ret
        .as_ref()
        .map(|r| (b.alloca_bytes(r.size, r.align.max(1)), r));
    if let Some((buf, r)) = &ret_area {
        b.line(&format!(
            "call void @llvm.memset.p0.i64(ptr {buf}, i8 0, i64 {}, i1 false)",
            r.size
        ));
    }
    let res = call_unit(&mut b, reg, &plan.unit, &words);
    if let Some((buf, r)) = &ret_area {
        // The safe result block's tag selects the case.
        let blk = b.reg();
        b.line(&format!("{blk} = inttoptr i64 {res} to ptr"));
        let tag = ctor_tag(&mut b, &blk, reg.escapes());
        let join = b.label();
        let mut fall = b.label();
        for case in &r.cases {
            let test = b.reg();
            b.line(&format!("{test} = icmp eq i64 {tag}, {}", case.safe_tag));
            let body = b.label();
            b.line(&format!("br i1 {test}, label %{body}, label %{fall}"));
            b.start(&body);
            let disc = b.reg();
            b.line(&format!("{disc} = trunc i64 {} to i8", case.disc));
            let dp = b.gep(buf, 0);
            b.line(&format!("store i8 {disc}, ptr {dp}, align 1"));
            if !matches!(case.payload, crate::core::CanonShape::Unit) {
                let payload = b.load_at(&blk, 0);
                canon_lower_shape(&mut b, &case.payload, buf, &payload)?;
            }
            b.line(&format!("br label %{join}"));
            let next = b.label();
            b.start(&fall);
            fall = next;
        }
        b.line("call void @__cinch_blackhole()");
        b.line("unreachable");
        b.start(&join);
    } else if let Some(shape) = &call.ret_flat {
        use crate::core::{CanonShape, CanonType};
        let word = match shape {
            CanonShape::Ctor {
                tag_at: Some(_), ..
            } => {
                let blk = b.reg();
                b.line(&format!("{blk} = inttoptr i64 {res} to ptr"));
                ctor_tag(&mut b, &blk, reg.escapes())
            }
            CanonShape::Ctor { fields, .. } => {
                let blk = b.reg();
                b.line(&format!("{blk} = inttoptr i64 {res} to ptr"));
                match fields.first() {
                    Some(_) => b.load_at(&blk, 0),
                    None => ctor_tag(&mut b, &blk, reg.escapes()),
                }
            }
            CanonShape::Scalar { .. } | CanonShape::Handle { .. } | CanonShape::Unit => res.clone(),
            CanonShape::Bytes { wrapped, .. } => {
                let ww = if *wrapped {
                    let p = b.reg();
                    b.line(&format!("{p} = inttoptr i64 {res} to ptr"));
                    b.load_at(&p, 0)
                } else {
                    res.clone()
                };
                let _ = ww;
                return err("a one-slot byte buffer result cannot cross flat");
            }
        };
        match call.core_results.first() {
            Some(CanonType::I32) => {
                let v = b.reg();
                b.line(&format!("{v} = trunc i64 {word} to i32"));
                b.line(&format!("ret i32 {v}"));
            }
            Some(CanonType::F64) => {
                let v = b.reg();
                b.line(&format!("{v} = bitcast i64 {word} to double"));
                b.line(&format!("ret double {v}"));
            }
            Some(CanonType::F32) => {
                let i = b.reg();
                b.line(&format!("{i} = trunc i64 {word} to i32"));
                let v = b.reg();
                b.line(&format!("{v} = bitcast i32 {i} to float"));
                b.line(&format!("ret float {v}"));
            }
            _ => {}
        }
    }
    if call.ret.is_some() || call.ret_flat.is_none() {
        b.line("ret void");
    }
    out.push_str(&b.finish(&header));
    Ok(())
}

// ==================== `prim` inline-body units (P18) ====================

/// The LLVM type of one prim ABI scalar (the inline fragment's widths).
fn abi_word(t: &PrimAbiTy) -> String {
    match t {
        PrimAbiTy::Int { bits, .. } => format!("i{bits}"),
        PrimAbiTy::F32 => "float".to_string(),
        PrimAbiTy::F64 => "double".to_string(),
    }
}

/// The inline-asm constraint letter of one prim ABI scalar.
fn asm_constraint(t: &PrimAbiTy) -> char {
    match t {
        PrimAbiTy::Int { .. } => 'r',
        PrimAbiTy::F32 | PrimAbiTy::F64 => 'f',
    }
}

/// The LLVM string literal contents of an `asm` fragment: quotes,
/// backslashes and line structure escape to hex bytes so the template
/// survives verbatim.
fn asm_escape(frag: &str) -> String {
    let mut out = String::new();
    for byte in frag.bytes() {
        match byte {
            b'"' => out.push_str("\\22"),
            b'\\' => out.push_str("\\5C"),
            b'\n' => out.push_str("\\0A"),
            b'\t' => out.push_str("\\09"),
            0x20..=0x7E => out.push(byte as char),
            _ => out.push_str(&format!("\\{byte:02X}")),
        }
    }
    out
}

/// One `prim` declaration's units (primitive operations and inline
/// target IR specification sections 2-3): one implementation unit per
/// applicable family body (the `llvm` fragment splices verbatim into
/// its function body; the `asm` fragment is inline asm carrying
/// `llvm.sideeffect` so DCE cannot drop it), and the canonical word
/// unit bridging the closure ABI to the selected body's declared ABI.
/// Selection is fixed at compile time per artifact (section 6).
fn prim_decl_units(
    units: &mut Vec<EmittedUnit>,
    cx: &LowerCx,
    reg: &Registry,
    program: &Program,
    f: &FnDef,
) -> Result<()> {
    let plan = program
        .prims
        .iter()
        .find(|p| p.unit == f.id.0)
        .ok_or_else(|| {
            crate::Error::plain(format!("internal error: no prim plan for unit {}", f.id))
        })?;
    let mut impl_names: Vec<String> = Vec::new();
    for (fam, bi) in &plan.family_units {
        let asm = fam == "asm";
        if asm && !cx.native {
            // An `asm` body is native assembly for the declared target:
            // it lowers for the native artifact only.
            continue;
        }
        impl_names.push(format!("{}.{}", plan.unit, fam));
        units.push(prim_impl_unit(reg, plan, *bi, asm)?);
    }
    units.push(prim_wrapper_unit(cx, reg, plan, f, &impl_names)?);
    Ok(())
}

/// One `prim` body's implementation unit: the fragment receives
/// exactly the declared arguments in the declared ABI (parameters
/// `%0 ...`, section 4's signature discipline) and returns the declared
/// result shape. The `immarg` contract rides the unit as function
/// metadata naming the immediate parameters: LLVM admits the `immarg`
/// parameter attribute only on intrinsics, and the checker enforces the
/// constant arguments (`design/llvm-contract-attributes.md`).
fn prim_impl_unit(reg: &Registry, plan: &PrimPlan, idx: usize, asm: bool) -> Result<EmittedUnit> {
    let body = &plan.bodies[idx];
    let name = format!("{}.{}", plan.unit, body.family);
    let params: Vec<String> = plan
        .args
        .iter()
        .enumerate()
        .map(|(i, t)| format!("{} %{}", abi_word(t), i))
        .collect();
    let ret_ty = abi_word(&plan.res);
    let mut header = format!("define {ret_ty} @{}({})", quote(&name), params.join(", "));
    if !body.immarg.is_empty() {
        let list: Vec<String> = body.immarg.iter().map(|n| format!("i32 {n}")).collect();
        let md = reg.meta_node(format!("!{{{}}}", list.join(", ")));
        header.push_str(&format!(" !immarg {md}"));
    }
    let code = if asm {
        // The asm fragment is an inline-asm template: `$0` is the
        // return operand and `$1 ...` the declared arguments. The call
        // carries `llvm.sideeffect` so DCE cannot drop it
        // (design/llvm-contract-attributes.md).
        let mut constraints = String::from("=");
        constraints.push(asm_constraint(&plan.res));
        for a in &plan.args {
            constraints.push(',');
            constraints.push(asm_constraint(a));
        }
        let args: Vec<String> = plan
            .args
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{} %{}", abi_word(t), i))
            .collect();
        let mut code = String::from("  call void @llvm.sideeffect()\n");
        code.push_str(&format!(
            "  %v = call {ret_ty} asm sideeffect \"{}\", \"{}\"({})\n",
            asm_escape(body.fragment.trim()),
            constraints,
            args.join(", ")
        ));
        code.push_str(&format!("  ret {ret_ty} %v\n"));
        code
    } else {
        format!("{}\n", body.fragment.trim_end())
    };
    let scan = scan_body(&code, reg.unit_symbols(), None);
    Ok(EmittedUnit {
        name,
        hints: crate::core::FnHints::default(),
        globals: String::new(),
        header,
        allocas: String::new(),
        code,
        stub: String::new(),
        clo: String::new(),
        scan,
        // An `asm` fragment is opaque target code: it claims nothing.
        lang_facts: UnitLang {
            pure: plan.pure && !asm,
            def: true,
            ..UnitLang::default()
        },
    })
}

/// The canonical word unit of a `prim`: the closure ABI bridged to the
/// selected body's implementation (the intrinsic units' bridge
/// convention: truncate the word in, canonicalize the result out).
fn prim_wrapper_unit(
    _cx: &LowerCx,
    reg: &Registry,
    plan: &PrimPlan,
    f: &FnDef,
    impl_names: &[String],
) -> Result<EmittedUnit> {
    let name = &plan.unit;
    let sel = &plan.bodies[plan.selected];
    let impl_name = format!("{}.{}", name, sel.family);
    if !impl_names.iter().any(|n| n == &impl_name) {
        return err(format!(
            "this build cannot emit the selected `{}` body of prim `{}` (an `asm` body lowers only for a native artifact; primitive operations and inline target IR specification section 3)",
            sel.family, name
        ));
    }
    let mut b = Builder::new();
    let mut args: Vec<String> = Vec::new();
    for (i, t) in plan.args.iter().enumerate() {
        let v = prim_arg_bridge(&mut b, t, &format!("%p{i}"));
        args.push(format!("{} {v}", abi_word(t)));
    }
    let ret_ty = abi_word(&plan.res);
    let v = b.reg();
    b.line(&format!(
        "{v} = call {ret_ty} @{}({})",
        quote(&impl_name),
        args.join(", ")
    ));
    let r = prim_res_bridge(&mut b, &plan.res, &v);
    ret(&mut b, &r);
    let scan = scan_body(&format!("{}{}", b.allocas, b.code), reg.unit_symbols(), Some(impl_name.as_str()));
    Ok(EmittedUnit {
        name: name.clone(),
        hints: f.hints.clone(),
        globals: String::new(),
        header: fn_header(name, plan.arity, None, &vec![true; plan.arity], None),
        allocas: b.allocas,
        code: b.code,
        stub: stub_def(reg, name, plan.arity),
        clo: clo_def(reg, name, plan.arity),
        scan,
        lang_facts: UnitLang {
            pure: plan.pure,
            def: true,
            ..UnitLang::default()
        },
    })
}

/// The canonical word to one fragment parameter.
fn prim_arg_bridge(b: &mut Builder, t: &PrimAbiTy, word: &str) -> String {
    match t {
        PrimAbiTy::Int { bits, .. } => {
            if *bits == 64 {
                word.to_string()
            } else {
                let r = b.reg();
                b.line(&format!("{r} = trunc i64 {word} to i{bits}"));
                r
            }
        }
        PrimAbiTy::F32 => {
            let i = b.reg();
            b.line(&format!("{i} = trunc i64 {word} to i32"));
            let f = b.reg();
            b.line(&format!("{f} = bitcast i32 {i} to float"));
            f
        }
        PrimAbiTy::F64 => {
            let f = b.reg();
            b.line(&format!("{f} = bitcast i64 {word} to double"));
            f
        }
    }
}

/// The fragment's result to the canonical word (the declared type's
/// word form: sign-extended signed widths, zero-extended unsigned).
fn prim_res_bridge(b: &mut Builder, t: &PrimAbiTy, v: &str) -> String {
    match t {
        PrimAbiTy::Int { bits, signed } => {
            if *bits == 64 {
                v.to_string()
            } else {
                let r = b.reg();
                let inst = if *signed { "sext" } else { "zext" };
                b.line(&format!("{r} = {inst} i{bits} {v} to i64"));
                r
            }
        }
        PrimAbiTy::F32 => {
            let i = b.reg();
            b.line(&format!("{i} = bitcast float {v} to i32"));
            let w = b.reg();
            b.line(&format!("{w} = zext i32 {i} to i64"));
            w
        }
        PrimAbiTy::F64 => {
            let w = b.reg();
            b.line(&format!("{w} = bitcast double {v} to i64"));
            w
        }
    }
}
