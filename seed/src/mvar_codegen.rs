//! Native MVar wrappers. Status is separate from the output word so all
//! raw scalar bit patterns, including zero, are valid stored values.
use super::*;
use crate::mvar::MVarOp;

pub(super) fn emit(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    prim: PrimOp,
    op: MVarOp,
    classes: [Class; 2],
) {
    use MVarOp::*;
    let wide = classes[0] == Class::Wide;
    if matches!(op, New | NewEmpty) {
        let value = if op == New && wide {
            wide_box(b, reg, "%p0")
        } else if op == New {
            "%p0".into()
        } else {
            "0".into()
        };
        let out = rt_call(
            b,
            "cinch_mvar_new",
            &[
                format!("i64 {value}"),
                format!("i64 {}", scan_code(frame_class(classes[0]))),
                format!("i64 {}", i32::from(op == New)),
            ],
        );
        ret(b, &out);
        return;
    }
    // These numbers are the runtime ABI in runtime/src/mvar.rs.
    let operation = match op {
        Take | TryTake | TakeOrCancel => 0,
        Put | TryPut | PutOrCancel => 1,
        Read | TryRead | ReadOrCancel => 2,
        Swap | SwapOrCancel => 3,
        IsEmpty => 4,
        _ => unreachable!(),
    };
    let mode = match op {
        TryTake | TryPut | TryRead | IsEmpty => 1,
        TakeOrCancel | PutOrCancel | ReadOrCancel | SwapOrCancel => 2,
        _ => 0,
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
        "cinch_mvar_access",
        &[
            "i64 %p0".into(),
            format!("i64 {value}"),
            format!("i64 {operation}"),
            format!("i64 {mode}"),
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
    } else if matches!(op, TryPut | PutOrCancel | IsEmpty) {
        ret(b, &status);
    } else if op == Put {
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
