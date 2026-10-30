//! Quotation evaluator tests (supercompilation and JIT specification
//! section 7): `compiler/runtime/src/eval.rs` over hand-assembled quote
//! blocks and programs (raw words, stand-in closure stubs over the
//! closure apply-stub ABI, as in `tests/tasks.rs`), asserting the
//! consumer-visible semantics of each opcode and the fault behaviour of
//! malformed quotations.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use cinchruntime::abi::{cinch_int_to_i64, cinch_nat_to_i64};
use cinchruntime::block::{ctor_escaped, ctor_id, static_header, KIND_CTOR};
use cinchruntime::eval::{run, KIND_CODE, MAX_NESTING};
use cinchruntime::Error;

// The opcode words, mirroring `quote_op` in `compiler/src/codegen.rs`.
const INT: i64 = 1;
const BIG: i64 = 2;
const BOOL: i64 = 3;
const UNIT: i64 = 4;
const LOCAL: i64 = 5;
const HOLE: i64 = 6;
const LET: i64 = 7;
const IF: i64 = 8;
const CASE: i64 = 9;
const APP: i64 = 10;
const LAM: i64 = 11;
const CTOR: i64 = 12;
const PROJ: i64 = 13;
const FAULT: i64 = 14;
const LAZY_ALLOC: i64 = 15;
const LAZY_INIT: i64 = 16;
const DEMAND: i64 = 17;
const UNITREF: i64 = 18;

type Stub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

/// A leaked word block, standing in for an ADT block, a closure record
/// or a quote block header. Word-aligned, like every real block word.
fn words_block(ws: &[i64]) -> i64 {
    Vec::leak(ws.to_vec()).as_mut_ptr() as i64
}

/// A leaked constructor block over `fields`: an immortal header whose
/// layout field is `layout` (the tag is in it), then the field words.
fn ctor_block(layout: u64, fields: &[i64]) -> i64 {
    let mut ws = vec![static_header(KIND_CTOR, layout as u16) as i64];
    ws.extend_from_slice(fields);
    words_block(&ws) + 8
}

/// A `Code` block over `program` with one hole word per captured quote.
fn quote_block(program: &[i64], holes: &[i64]) -> i64 {
    let code = words_block(program);
    let mut head = vec![KIND_CODE, code, program.len() as i64, holes.len() as i64];
    head.extend_from_slice(holes);
    words_block(&head)
}

/// A closure record `{ stub; arity; nfilled; saved... }`, the stand-in
/// for a generated `*.clo` constant.
fn closure(stub: Stub, arity: i64, saved: &[i64]) -> i64 {
    let mut ws = vec![stub as usize as i64, arity, saved.len() as i64];
    ws.extend_from_slice(saved);
    words_block(&ws)
}

/// A zeroed, word-aligned raw byte block for boundary-layout projections
/// (with a byte view to lay fields out by hand).
fn byte_block() -> (i64, &'static mut [u8]) {
    let words: &'static mut [u64] = Vec::leak(vec![0u64; 8]);
    let word = words.as_mut_ptr() as i64;
    let buf = unsafe { std::slice::from_raw_parts_mut(words.as_mut_ptr() as *mut u8, 64) };
    (word, buf)
}

/// The operand words of a `BIG` constant: the decimal bytes of `s`
/// packed eight per word, little endian.
fn big_program(nat: i64, s: &str) -> Vec<i64> {
    let mut p = vec![BIG, nat, s.len() as i64];
    for chunk in s.as_bytes().chunks(8) {
        let mut w = 0i64;
        for (i, c) in chunk.iter().enumerate() {
            w |= (*c as i64) << (8 * i);
        }
        p.push(w);
    }
    p
}

// ------------------------------------------------------------ constants

#[test]
fn constants_evaluate_to_their_words() {
    assert_eq!(run(quote_block(&[INT, 41], &[])), Ok(41));
    assert_eq!(run(quote_block(&[INT, -7], &[])), Ok(-7));
    assert_eq!(run(quote_block(&[BOOL, 1], &[])), Ok(1));
    assert_eq!(run(quote_block(&[BOOL, 0], &[])), Ok(0));
    assert_eq!(run(quote_block(&[UNIT], &[])), Ok(0));
}

#[test]
fn let_if_and_local_evaluate_to_the_known_word() {
    // let x = 41 in if true then x else unit
    let taken = [LET, 1, INT, 41, IF, BOOL, 1, LOCAL, 1, UNIT];
    assert_eq!(run(quote_block(&taken, &[])), Ok(41));
    // let x = 41 in if false then x else unit
    let other = [LET, 1, INT, 41, IF, BOOL, 0, LOCAL, 1, UNIT];
    assert_eq!(run(quote_block(&other, &[])), Ok(0));
    // A nonzero condition word is true, as in the generated branch.
    let nonzero = [IF, INT, 5, INT, 1, INT, 2];
    assert_eq!(run(quote_block(&nonzero, &[])), Ok(1));
}

// ------------------------------------------------------------ APP and LAM

unsafe extern "C" fn stub_weighted(_self: i64, argc: i64, argv: *mut i64) -> i64 {
    if argc != 3 {
        return -1;
    }
    let a = unsafe { std::slice::from_raw_parts(argv, 3) };
    a[0] * 100 + a[1] * 10 + a[2]
}

#[test]
fn app_applies_a_unit_closure_to_its_arguments_in_order() {
    let clo = closure(stub_weighted, 3, &[]);
    let prog = [APP, UNITREF, clo, 3, INT, 3, INT, 4, INT, 5];
    // 345 proves the arguments arrive in order.
    assert_eq!(run(quote_block(&prog, &[])), Ok(345));
}

static THUNK_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn stub_thunk_42(_self: i64, argc: i64, _argv: *mut i64) -> i64 {
    THUNK_CALLS.fetch_add(1, Ordering::SeqCst);
    if argc != 0 {
        -1
    } else {
        42
    }
}

#[test]
fn app_applies_a_zero_arity_thunk() {
    THUNK_CALLS.store(0, Ordering::SeqCst);
    let clo = closure(stub_thunk_42, 0, &[]);
    assert_eq!(run(quote_block(&[APP, UNITREF, clo, 0], &[])), Ok(42));
    assert_eq!(THUNK_CALLS.load(Ordering::SeqCst), 1);
}

unsafe extern "C" fn stub_env_add(self_word: i64, argc: i64, argv: *mut i64) -> i64 {
    let rec = self_word as *const i64;
    // The `{ stub; arity; nfilled; saved... }` record the LAM opcode
    // builds; a deviation returns the failure sentinel.
    let arity = unsafe { *rec.add(1) };
    let nfilled = unsafe { *rec.add(2) };
    if arity != 2 || nfilled != 2 || argc != 2 {
        return -1;
    }
    let s0 = unsafe { *rec.add(3) };
    let s1 = unsafe { *rec.add(4) };
    s0 * 1000 + s1 * 100 + unsafe { *argv * 10 + *argv.add(1) }
}

#[test]
fn lam_builds_a_closure_record_over_its_saved_words() {
    // (lam with env [10, 20]) applied to [1, 2]: the stub reads the
    // saved words from the record and folds in the arguments.
    let prog = [APP, LAM, stub_env_add as usize as i64, 2, 2, INT, 10, INT, 20, 2, INT, 1, INT, 2];
    assert_eq!(run(quote_block(&prog, &[])), Ok(12012));
}

// ------------------------------------------------------------ CASE and CTOR

static UNTAKEN_CALLS: AtomicUsize = AtomicUsize::new(0);
static TAKEN_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn stub_untaken(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    UNTAKEN_CALLS.fetch_add(1, Ordering::SeqCst);
    999
}

unsafe extern "C" fn stub_add_fields(_self: i64, argc: i64, argv: *mut i64) -> i64 {
    TAKEN_CALLS.fetch_add(1, Ordering::SeqCst);
    if argc != 2 {
        return -1;
    }
    unsafe { *argv + *argv.add(1) }
}

#[test]
fn case_selects_the_matching_arm_and_binds_its_fields() {
    UNTAKEN_CALLS.store(0, Ordering::SeqCst);
    TAKEN_CALLS.store(0, Ordering::SeqCst);
    let clo_u = closure(stub_untaken, 0, &[]);
    let clo_a = closure(stub_add_fields, 2, &[]);
    // Scrutinee block { 7; 8 } of tag 1, injected through UNITREF.
    let blk = ctor_block(ctor_id(0, 1), &[7, 8]);
    let prog = [
        CASE, UNITREF, blk,
        2,
        0, 0, APP, UNITREF, clo_u, 0,
        1, 2, 10, 11, APP, UNITREF, clo_a, 2, LOCAL, 10, LOCAL, 11,
        0,
    ];
    assert_eq!(run(quote_block(&prog, &[])), Ok(15));
    assert_eq!(TAKEN_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(UNTAKEN_CALLS.load(Ordering::SeqCst), 0, "the untaken arm ran");
}

#[test]
fn case_wildcard_default_when_no_tag_matches() {
    let blk = ctor_block(ctor_id(3, 9), &[0]);
    let prog = [CASE, UNITREF, blk, 0, 1, INT, 42];
    assert_eq!(run(quote_block(&prog, &[])), Ok(42));
}

#[test]
fn case_variable_default_binds_the_scrutinee() {
    let blk = ctor_block(ctor_id(3, 9), &[0]);
    let prog = [CASE, UNITREF, blk, 0, 2, 33, LOCAL, 33];
    assert_eq!(run(quote_block(&prog, &[])), Ok(blk));
}

/// A default-only `case` never reads its scrutinee, so an immediate (no
/// header, low bit 1) is a valid scrutinee there.
#[test]
fn case_with_defaults_only_never_reads_an_immediate_scrutinee() {
    assert_eq!(run(quote_block(&[CASE, INT, 0x2b, 0, 1, INT, 42], &[])), Ok(42));
    assert_eq!(run(quote_block(&[CASE, INT, 0x2b, 0, 2, 5, LOCAL, 5], &[])), Ok(0x2b));
}

#[test]
fn ctor_allocates_a_block_case_can_consume() {
    // ctor of layout id 2 (group 0, tag 2) [5, 6], then case on it
    // reading the field words back.
    let id = ctor_id(0, 2) as i64;
    let prog = [
        LET, 1, CTOR, id, 2, INT, 5, INT, 6,
        CASE, LOCAL, 1, 1, 2, 2, 10, 11, LOCAL, 11, 0,
    ];
    assert_eq!(run(quote_block(&prog, &[])), Ok(6));
    // An escaped descriptor keeps its tag (70, beyond a group) in the
    // word before the header, and `case` reads it from there.
    let esc = ctor_escaped(4, 70) as i64;
    let prog = [
        LET, 1, CTOR, esc, 1, INT, 9,
        CASE, LOCAL, 1, 2, 6, 0, INT, 1, 70, 1, 10, LOCAL, 10, 0,
    ];
    assert_eq!(run(quote_block(&prog, &[])), Ok(9));
}

// ------------------------------------------------------------ HOLE

#[test]
fn hole_evaluates_the_captured_quote_block() {
    let inner = quote_block(&[INT, 7], &[]);
    let prog = [LET, 1, HOLE, 0, LOCAL, 1];
    assert_eq!(run(quote_block(&prog, &[inner])), Ok(7));
}

#[test]
fn out_of_range_hole_index_is_an_error() {
    let inner = quote_block(&[INT, 7], &[]);
    assert_eq!(run(quote_block(&[HOLE, 3], &[inner])), Err(Error::CorruptValue));
    assert_eq!(run(quote_block(&[HOLE, 1], &[inner])), Err(Error::CorruptValue));
}

// ------------------------------------------------------------ PROJ

#[test]
fn proj_mode_0_loads_a_field_word() {
    let (blk, buf) = byte_block();
    buf[8..16].copy_from_slice(&777i64.to_le_bytes());
    buf[16..24].copy_from_slice(&(-5i64).to_le_bytes());
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 0, 1], &[])), Ok(777));
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 0, 2], &[])), Ok(-5));
}

#[test]
fn proj_mode_1_extends_signed_and_unsigned() {
    let (blk, buf) = byte_block();
    buf[0] = 0xFF;
    buf[1..3].copy_from_slice(&0x8000u16.to_le_bytes());
    // One byte at offset 0: 255 unsigned, -1 signed.
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 1, 0, 1, 0], &[])), Ok(255));
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 1, 0, 1, 1], &[])), Ok(-1));
    // Two bytes at offset 1: 32768 unsigned, -32768 signed.
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 1, 1, 2, 0], &[])), Ok(32768));
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 1, 1, 2, 1], &[])), Ok(-32768));
    // A zero-width scalar loads the constant 0 (the compiler's shape).
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 1, 0, 0, 0], &[])), Ok(0));
}

#[test]
fn proj_mode_2_loads_a_boundary_word() {
    let (blk, buf) = byte_block();
    buf[8..16].copy_from_slice(&777i64.to_le_bytes());
    assert_eq!(run(quote_block(&[PROJ, UNITREF, blk, 2, 8], &[])), Ok(777));
}

// ------------------------------------------------------------ laziness

static THUNK5_CALLS: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn stub_thunk_5(_self: i64, argc: i64, _argv: *mut i64) -> i64 {
    THUNK5_CALLS.fetch_add(1, Ordering::SeqCst);
    if argc != 1 {
        -1
    } else {
        5
    }
}

unsafe extern "C" fn stub_sum_2(_self: i64, argc: i64, argv: *mut i64) -> i64 {
    if argc != 2 {
        -1
    } else {
        unsafe { *argv + *argv.add(1) }
    }
}

#[test]
fn lazy_cell_demands_its_thunk_once_across_repeated_demands() {
    THUNK5_CALLS.store(0, Ordering::SeqCst);
    let clo = closure(stub_thunk_5, 1, &[]);
    let clo_sum = closure(stub_sum_2, 2, &[]);
    let prog = [
        LET, 1, LAZY_ALLOC,
        LET, 2, LAZY_INIT, 1, UNITREF, clo,
        LET, 3, DEMAND, LOCAL, 1,
        LET, 4, DEMAND, LOCAL, 1,
        APP, UNITREF, clo_sum, 2, LOCAL, 3, LOCAL, 4,
    ];
    assert_eq!(run(quote_block(&prog, &[])), Ok(10));
    assert_eq!(THUNK5_CALLS.load(Ordering::SeqCst), 1, "the thunk ran more than once");
}

unsafe extern "C" fn stub_thunk_zero(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    0
}

#[test]
fn lazy_init_evaluates_to_the_unit_word() {
    let clo = closure(stub_thunk_zero, 1, &[]);
    let prog = [LET, 1, LAZY_ALLOC, LET, 2, LAZY_INIT, 1, UNITREF, clo, LOCAL, 2];
    assert_eq!(run(quote_block(&prog, &[])), Ok(0));
}

static SELF_CALLS: AtomicUsize = AtomicUsize::new(0);
static INNER_ERR: Mutex<Option<Error>> = Mutex::new(None);

/// The self-demanding thunk: it re-enters the evaluator with the quoted
/// demand of its own cell, as a hoisted lambda body would call the
/// generated `__cinch_demand`.
unsafe extern "C" fn stub_self_demand(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    SELF_CALLS.fetch_add(1, Ordering::SeqCst);
    let cell = unsafe { *(self_word as *const i64).add(3) };
    let inner = run(quote_block(&[DEMAND, UNITREF, cell], &[]));
    *INNER_ERR.lock().unwrap() = inner.err();
    0
}

#[test]
fn self_demanding_cell_faults_as_the_black_hole() {
    SELF_CALLS.store(0, Ordering::SeqCst);
    *INNER_ERR.lock().unwrap() = None;
    let cell = run(quote_block(&[LAZY_ALLOC], &[])).expect("LAZY_ALLOC yields the cell");
    let clo = closure(stub_self_demand, 1, &[cell]);
    let prog = [
        LET, 1, UNITREF, cell,
        LET, 2, LAZY_INIT, 1, UNITREF, clo,
        DEMAND, LOCAL, 1,
    ];
    assert_eq!(run(quote_block(&prog, &[])), Err(Error::BlackHole));
    assert_eq!(SELF_CALLS.load(Ordering::SeqCst), 1);
    assert_eq!(*INNER_ERR.lock().unwrap(), Some(Error::BlackHole));
    // The cell stays black-holed: state 1, no stored value.
    unsafe {
        assert_eq!(*(cell as *const i64), 1);
        assert_eq!(*(cell as *const i64).add(2), 0);
    }
}

// ------------------------------------------------------------ faults

#[test]
fn fault_opcode_takes_the_black_hole_fault() {
    assert_eq!(run(quote_block(&[FAULT], &[])), Err(Error::BlackHole));
}

#[test]
fn exit_codes_match_the_fault_table() {
    // cinch_eval exits 70 for BlackHole and 71 for a failing evaluation.
    assert_eq!(Error::BlackHole.exit_code(), 70);
    assert_eq!(Error::CorruptValue.exit_code(), 71);
    assert_eq!(Error::MalformedDecimal.exit_code(), 71);
    assert_eq!(Error::NestingTooDeep.exit_code(), 71);
}

#[test]
fn malformed_program_streams_are_errors() {
    // Truncated stream (LET without its body).
    assert_eq!(run(quote_block(&[LET, 1, INT, 5], &[])), Err(Error::CorruptValue));
    // Unknown opcode.
    assert_eq!(run(quote_block(&[42], &[])), Err(Error::CorruptValue));
    // Trailing words after one complete expression.
    assert_eq!(run(quote_block(&[UNIT, INT, 5], &[])), Err(Error::CorruptValue));
    // Out-of-range local slot.
    assert_eq!(run(quote_block(&[LOCAL, 9], &[])), Err(Error::CorruptValue));
    // The LET value sees no binding for its own slot.
    assert_eq!(
        run(quote_block(&[LET, 1, LOCAL, 1, UNIT], &[])),
        Err(Error::CorruptValue)
    );
    // A boolean operand word outside 0|1.
    assert_eq!(run(quote_block(&[BOOL, 2], &[])), Err(Error::CorruptValue));
    // A lambda capturing more values than its arity.
    assert_eq!(
        run(quote_block(&[LAM, 1, 1, 2, INT, 1, INT, 2], &[])),
        Err(Error::CorruptValue)
    );
}

#[test]
fn malformed_case_is_an_error() {
    let blk = words_block(&[1, 7, 8]);
    // Duplicate constructor tags.
    assert_eq!(
        run(quote_block(
            &[CASE, UNITREF, blk, 2, 1, 0, INT, 0, 1, 0, INT, 0, 0],
            &[]
        )),
        Err(Error::CorruptValue)
    );
    // No matching tag and no default (never reachable for a typed quote).
    assert_eq!(
        run(quote_block(&[CASE, UNITREF, blk, 0, 0], &[])),
        Err(Error::CorruptValue)
    );
    // An unknown default marker.
    assert_eq!(
        run(quote_block(&[CASE, UNITREF, blk, 0, 3], &[])),
        Err(Error::CorruptValue)
    );
}

#[test]
fn malformed_quote_blocks_are_errors() {
    let code = words_block(&[UNIT]);
    // Wrong kind word.
    assert_eq!(run(words_block(&[2, code, 1, 0])), Err(Error::CorruptValue));
    // Null program pointer.
    assert_eq!(run(words_block(&[KIND_CODE, 0, 1, 0])), Err(Error::CorruptValue));
    // Short program (no words at all).
    assert_eq!(run(words_block(&[KIND_CODE, code, 0, 0])), Err(Error::CorruptValue));
    // Insane counts.
    assert_eq!(
        run(words_block(&[KIND_CODE, code, i64::MAX, 0])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(words_block(&[KIND_CODE, code, 1, i64::MAX])),
        Err(Error::CorruptValue)
    );
    // Null or misaligned quote word.
    assert_eq!(run(0), Err(Error::CorruptValue));
    assert_eq!(run(5), Err(Error::CorruptValue));
}

#[test]
fn non_block_words_where_a_block_is_required_are_errors() {
    // Null and misaligned values as scrutinee, projection base, function
    // and memo cell.
    assert_eq!(
        run(quote_block(&[PROJ, INT, 5, 0, 0], &[])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(quote_block(&[PROJ, INT, 0, 0, 0], &[])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(quote_block(&[DEMAND, INT, 5], &[])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(quote_block(&[CASE, INT, 5, 0, 0], &[])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(quote_block(&[APP, INT, 5, 0], &[])),
        Err(Error::CorruptValue)
    );
    assert_eq!(
        run(quote_block(&[LET, 1, INT, 5, LAZY_INIT, 1, UNITREF, 1], &[])),
        Err(Error::CorruptValue)
    );
}

// ------------------------------------------------------------ BIG constants

#[test]
fn big_constants_parse_their_decimal_digits() {
    let nat = run(quote_block(&big_program(1, "12345678901"), &[])).expect("a Nat word");
    assert_eq!(unsafe { cinch_nat_to_i64(nat) }, 12345678901);
    let neg = run(quote_block(&big_program(0, "-128"), &[])).expect("an Int word");
    assert_eq!(unsafe { cinch_int_to_i64(neg) }, -128);
    let zero = run(quote_block(&big_program(1, "000"), &[])).expect("a Nat word");
    assert_eq!(unsafe { cinch_nat_to_i64(zero) }, 0);
}

#[test]
fn big_constants_reject_malformed_digits_and_padding() {
    // A non-digit byte.
    assert_eq!(
        run(quote_block(&big_program(1, "1A"), &[])),
        Err(Error::MalformedDecimal)
    );
    // An empty numeral.
    assert_eq!(
        run(quote_block(&[BIG, 1, 0], &[])),
        Err(Error::MalformedDecimal)
    );
    // A sign on a Nat.
    assert_eq!(
        run(quote_block(&big_program(1, "-3"), &[])),
        Err(Error::MalformedDecimal)
    );
    // A nat flag word outside 0|1.
    assert_eq!(run(quote_block(&[BIG, 2, 1, 0x31], &[])), Err(Error::CorruptValue));
    // Nonzero padding bytes past the digit length.
    assert_eq!(
        run(quote_block(&[BIG, 1, 2, 0x99_35_32], &[])),
        Err(Error::CorruptValue)
    );
    // A digit chunk shorter than the declared byte length.
    assert_eq!(
        run(quote_block(&[BIG, 1, 9, 0x31], &[])),
        Err(Error::CorruptValue)
    );
}

// ------------------------------------------------------------ nesting bound

/// `n` nested `let`s, the innermost body reading the innermost binder.
fn nested_lets(n: usize) -> Vec<i64> {
    let mut p = Vec::new();
    for i in 0..n {
        p.extend([LET, i as i64 + 1, INT, i as i64 + 1]);
    }
    p.extend([LOCAL, n as i64]);
    p
}

/// A chain of `levels` nested quotes (`HOLE` of `HOLE` of ... `INT 7`).
fn nested_hole_chain(levels: usize) -> i64 {
    let mut q = quote_block(&[INT, 7], &[]);
    for _ in 0..levels {
        q = quote_block(&[HOLE, 0], &[q]);
    }
    q
}

#[test]
fn nesting_within_the_bound_evaluates() {
    // Both shapes exercise the budget from the two directions: a deep
    // expression tree (one unit per node) and a deep chain of nested
    // quotations (about two units per quote).
    assert_eq!(run(quote_block(&nested_lets(1000), &[])), Ok(1000));
    assert_eq!(run(nested_hole_chain(450)), Ok(7));
}

#[test]
fn nesting_past_the_bound_is_an_error() {
    assert_eq!(run(quote_block(&nested_lets(3000), &[])), Err(Error::NestingTooDeep));
    assert_eq!(run(nested_hole_chain(600)), Err(Error::NestingTooDeep));
    // The bound is documented and generous.
    assert!(MAX_NESTING >= 512);
}
