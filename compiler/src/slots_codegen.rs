//! Emission of the mutable slot units (`crate::slots`). The hot four
//! (length, read, write, take) are inline IR over the `KIND_ARRAY` payload
//! layout (`compiler/runtime/src/arrays.rs`): word 0 the element format,
//! word 1 the length, the elements from word 2, eight bytes each for the
//! word formats a slot array always has.
use super::*;
use crate::slots::{counted, SlotOp};

/// The payload pointer, the slot index checked against the length, and the
/// slot's address. An index at or past the length traps (code 1).
fn slot_addr(b: &mut Builder) -> String {
    let base = b.reg();
    b.line(&format!("{base} = inttoptr i64 %p0 to ptr"));
    let lenp = b.reg();
    b.line(&format!("{lenp} = getelementptr inbounds i64, ptr {base}, i64 1"));
    let len = b.reg();
    b.line(&format!("{len} = load i64, ptr {lenp}, align 8"));
    let ok = b.reg();
    b.line(&format!("{ok} = icmp ult i64 %p1, {len}"));
    let (lok, lbad) = (b.label(), b.label());
    b.line(&format!("br i1 {ok}, label %{lok}, label %{lbad}"));
    b.start(&lbad);
    b.line("call i64 @cinch_slots_fault(i64 1)");
    b.line("unreachable");
    b.start(&lok);
    let at = b.reg();
    b.line(&format!("{at} = add i64 %p1, 2"));
    let p = b.reg();
    b.line(&format!("{p} = getelementptr inbounds i64, ptr {base}, i64 {at}"));
    p
}

/// Trap (code 2) when a counted word is a hole.
fn not_hole(b: &mut Builder, w: &str, class: Class) {
    if !counted(class) {
        return;
    }
    let z = b.reg();
    b.line(&format!("{z} = icmp eq i64 {w}, 0"));
    let (lok, lbad) = (b.label(), b.label());
    b.line(&format!("br i1 {z}, label %{lbad}, label %{lok}"));
    b.start(&lbad);
    b.line("call i64 @cinch_slots_fault(i64 2)");
    b.line("unreachable");
    b.start(&lok);
}

pub(super) fn emit(b: &mut Builder, reg: &Registry, op: SlotOp, classes: [Class; 2]) {
    let class = classes[0];
    match op {
        SlotOp::New => {
            let out = rt_call(b, "cinch_slots_new", &["i64 %p0".to_string(), format!("i64 {}", scan_code(class))]);
            ret(b, &out);
        }
        SlotOp::Len => {
            let base = b.reg();
            b.line(&format!("{base} = inttoptr i64 %p0 to ptr"));
            let lenp = b.reg();
            b.line(&format!("{lenp} = getelementptr inbounds i64, ptr {base}, i64 1"));
            let len = b.reg();
            b.line(&format!("{len} = load i64, ptr {lenp}, align 8"));
            ret(b, &len);
        }
        SlotOp::Read => {
            let p = slot_addr(b);
            let w = b.reg();
            b.line(&format!("{w} = load i64, ptr {p}, align 8"));
            not_hole(b, &w, class);
            rc_op(b, reg.rc_debug, false, &w, class);
            ret(b, &w);
        }
        SlotOp::Take => {
            let p = slot_addr(b);
            let w = b.reg();
            b.line(&format!("{w} = load i64, ptr {p}, align 8"));
            not_hole(b, &w, class);
            if counted(class) {
                b.line(&format!("store i64 0, ptr {p}, align 8"));
            }
            ret(b, &w);
        }
        SlotOp::Write => {
            // Store first, then release what the slot held: a write of the
            // value a slot already holds owns its own reference (the unit
            // consumes `%p2`), so the release cannot free it.
            let p = slot_addr(b);
            let old = b.reg();
            b.line(&format!("{old} = load i64, ptr {p}, align 8"));
            b.line(&format!("store i64 %p2, ptr {p}, align 8"));
            rc_op(b, reg.rc_debug, true, &old, class);
            ret(b, "0");
        }
        SlotOp::Prefetch => {
            // In range only: the hint is free to issue, but an address past
            // the block is not one to name.
            let base = b.reg();
            b.line(&format!("{base} = inttoptr i64 %p0 to ptr"));
            let lenp = b.reg();
            b.line(&format!("{lenp} = getelementptr inbounds i64, ptr {base}, i64 1"));
            let len = b.reg();
            b.line(&format!("{len} = load i64, ptr {lenp}, align 8"));
            let ok = b.reg();
            b.line(&format!("{ok} = icmp ult i64 %p1, {len}"));
            let (lgo, ldone) = (b.label(), b.label());
            b.line(&format!("br i1 {ok}, label %{lgo}, label %{ldone}"));
            b.start(&lgo);
            let at = b.reg();
            b.line(&format!("{at} = add i64 %p1, 2"));
            let p = b.reg();
            b.line(&format!("{p} = getelementptr inbounds i64, ptr {base}, i64 {at}"));
            b.line(&format!("call void @llvm.prefetch.p0(ptr {p}, i32 0, i32 3, i32 1)"));
            b.line(&format!("br label %{ldone}"));
            b.start(&ldone);
            ret(b, "0");
        }
        SlotOp::Move => {
            let _ = rt_call(
                b,
                "cinch_slots_move",
                &(0..5).map(|i| format!("i64 %p{i}")).collect::<Vec<_>>(),
            );
            ret(b, "0");
        }
        SlotOp::Clear => {
            let _ = rt_call(b, "cinch_slots_clear", &(0..3).map(|i| format!("i64 %p{i}")).collect::<Vec<_>>());
            ret(b, "0");
        }
        SlotOp::FoldNat => {
            // An immediate `Nat` is `(v << 1) | 1`; a boxed one asks the
            // runtime for its low limbs.
            let tag = b.reg();
            b.line(&format!("{tag} = and i64 %p0, 1"));
            let imm = b.reg();
            b.line(&format!("{imm} = icmp ne i64 {tag}, 0"));
            let (lfast, lslow) = (b.label(), b.label());
            b.line(&format!("br i1 {imm}, label %{lfast}, label %{lslow}"));
            b.start(&lfast);
            let v = b.reg();
            b.line(&format!("{v} = lshr i64 %p0, 1"));
            ret(b, &v);
            b.start(&lslow);
            let w = rt_call(b, "cinch_nat_low_word", &["i64 %p0".to_string()]);
            ret(b, &w);
        }
    }
}
