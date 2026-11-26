//! Native codegen wrappers for `AtomicCell` and `RwLockCell a`.
use super::*;
use crate::sync_prim::{AtomicOp, RwLockOp};

pub(super) fn emit_atomic(b: &mut Builder, op: AtomicOp) {
    let arity = op.sig().0.len();
    let args: Vec<String> = (0..arity).map(|i| format!("i64 %p{i}")).collect();
    let out = rt_call(b, op.extern_name(), &args);
    if matches!(op, AtomicOp::Store | AtomicOp::Fence) {
        ret(b, "0");
    } else {
        ret(b, &out);
    }
}

pub(super) fn emit_rwlock(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    prim: PrimOp,
    op: RwLockOp,
    classes: [Class; 2],
) {
    use RwLockOp::*;
    let wide = classes[0] == Class::Wide;
    if op == New {
        let value = if wide {
            wide_box(b, reg, "%p0")
        } else {
            "%p0".into()
        };
        let out = rt_call(
            b,
            "cinch_rwlock_new",
            &[
                format!("i64 {value}"),
                format!("i64 {}", scan_code(frame_class(classes[0]))),
            ],
        );
        ret(b, &out);
        return;
    }
    // ABI op codes matching seed/runtime/src/rwlock.rs.
    let operation = match op {
        Read => 0,
        Write => 1,
        Swap => 2,
        TryRead => 3,
        TryWrite => 4,
        New => unreachable!(),
    };
    let value = if op.consumes(1) && wide {
        wide_box(b, reg, "%p1")
    } else if op.consumes(1) {
        "%p1".into()
    } else {
        "0".into()
    };
    let slot = b.reg();
    b.line(&format!("{slot} = alloca i64, align 8"));
    let status = rt_call(
        b,
        "cinch_rwlock_access",
        &[
            "i64 %p0".into(),
            format!("i64 {value}"),
            format!("i64 {operation}"),
            format!("ptr {slot}"),
        ],
    );
    if op.returns_option() {
        let none = b.reg();
        b.line(&format!("{none} = icmp eq i64 {status}, 0"));
        let load = |b: &mut Builder| {
            let out = b.reg();
            b.line(&format!("{out} = load i64, ptr {slot}, align 8"));
            if wide {
                wide_unbox(b, reg, &out)
            } else {
                out
            }
        };
        if wide {
            ret_option_wide_with(b, reg, name, &none, load);
        } else {
            ret_option(b, reg, prim, name, &none, load);
        }
    } else if op == TryWrite {
        ret(b, &status);
    } else if op == Write {
        ret(b, "0");
    } else {
        let out = b.reg();
        b.line(&format!("{out} = load i64, ptr {slot}, align 8"));
        if wide {
            let value = wide_unbox(b, reg, &out);
            b.line(&format!("ret i128 {value}"));
        } else {
            ret(b, &out);
        }
    }
}
