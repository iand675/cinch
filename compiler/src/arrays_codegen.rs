use super::*;
use crate::arrays::ArrayOp;

pub(super) fn emit(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    prim: PrimOp,
    op: ArrayOp,
    classes: [Class; 2],
    packed: Option<Scalar>,
) {
    use ArrayOp::*;
    let arg = |i| format!("i64 %p{i}");
    let nat = |b: &mut Builder, i| rt_call(b, "cinch_nat_to_len", &[arg(i)]);
    let f = packed
        .map(|s| 0x100 | s.size() | if s.is_signed() { 0x80 } else { 0 })
        .unwrap_or_else(|| scan_code(classes[0]));
    match op {
        FromList => {
            let out = rt_call(b, "cinch_array_from_list", &[arg(0), format!("i64 {f}")]);
            ret(b, &out);
        }
        Replicate => {
            let n = nat(b, 0);
            let out = rt_call(
                b,
                "cinch_array_replicate",
                &[format!("i64 {n}"), arg(1), format!("i64 {f}")],
            );
            ret(b, &out);
        }
        Len | Rows | Columns => {
            let sym = match op {
                Len => "cinch_array_len",
                Rows => "cinch_columnar_rows",
                _ => "cinch_columnar_columns",
            };
            let n = rt_call(b, sym, &[arg(0)]);
            let out = rt_call(b, "cinch_nat_from_i64", &[format!("i64 {n}")]);
            ret(b, &out);
        }
        Get | Row => {
            let i = nat(b, 1);
            let valid = if op == Get {
                let has = rt_call(b, "cinch_array_has", &[arg(0), format!("i64 {i}")]);
                let bad = b.reg();
                b.line(&format!("{bad} = icmp eq i64 {has}, 0"));
                bad
            } else {
                let n = rt_call(b, "cinch_columnar_rows", &[arg(0)]);
                let bad = b.reg();
                b.line(&format!("{bad} = icmp uge i64 {i}, {n}"));
                bad
            };
            ret_option(b, reg, prim, name, &valid, |b| {
                rt_call(
                    b,
                    if op == Get {
                        "cinch_array_get"
                    } else {
                        "cinch_columnar_get"
                    },
                    &[arg(0), format!("i64 {i}")],
                )
            });
        }
        TensorGet | TensorSet => {
            let i = rt_call(b, "cinch_tensor_index", &[arg(0), arg(1)]);
            let bad = b.reg();
            b.line(&format!("{bad} = icmp slt i64 {i}, 0"));
            ret_option(b, reg, prim, name, &bad, |b| {
                let mut args = vec![arg(0), format!("i64 {i}")];
                if op == TensorSet {
                    args.push(arg(2));
                }
                rt_call(
                    b,
                    if op == TensorGet {
                        "cinch_tensor_get"
                    } else {
                        "cinch_tensor_set"
                    },
                    &args,
                )
            });
        }
        Set | Slice | Reshape | Zip => {
            let (sym, args) = match op {
                Set => {
                    let i = nat(b, 1);
                    // The entries differ only in whether the write may happen
                    // in place, and the borrow inference already decided that
                    // per unit: `Program::lent` holds an entry only for a unit
                    // with at least one borrowed parameter, and its bool is
                    // true for borrowed. A unit that does not borrow its array
                    // parameter is one whose callers hand the reference over,
                    // so it takes the in-place entry; a borrowing unit keeps
                    // the copying one. Both entries share the same count
                    // contract, so this choice is invisible to source.
                    let borrowed = reg
                        .lent
                        .get(name)
                        .and_then(|m| m.first().copied())
                        .unwrap_or(false);
                    let sym = if borrowed {
                        "cinch_array_set"
                    } else {
                        "cinch_array_set_inplace"
                    };
                    (sym, vec![arg(0), format!("i64 {i}"), arg(2)])
                }
                Slice => {
                    let i = nat(b, 1);
                    let n = nat(b, 2);
                    (
                        "cinch_array_slice",
                        vec![arg(0), format!("i64 {i}"), format!("i64 {n}")],
                    )
                }
                Reshape => ("cinch_tensor_reshape", vec![arg(0), arg(1)]),
                Zip => (
                    "cinch_columnar_zip",
                    vec![
                        arg(0),
                        arg(1),
                        format!("i64 {}", reg.shape_desc(&classes, 0)),
                    ],
                ),
                _ => unreachable!(),
            };
            let out = rt_call(b, sym, &args);
            let bad = b.reg();
            b.line(&format!("{bad} = icmp eq i64 {out}, 0"));
            ret_option(b, reg, prim, name, &bad, |_| out);
        }
        Left | Right | Leaf => {
            let out = if op == Leaf {
                rt_call(b, "cinch_columnar_array", &[arg(0)])
            } else {
                rt_call(
                    b,
                    "cinch_columnar_child",
                    &[arg(0), format!("i64 {}", u8::from(op == Right))],
                )
            };
            let bad = b.reg();
            b.line(&format!("{bad} = icmp eq i64 {out}, 0"));
            ret_option(b, reg, prim, name, &bad, |_| out);
        }
        Append | Shape | Data | Column => {
            let (sym, args) = match op {
                Append => ("cinch_array_append", vec![arg(0), arg(1)]),
                Shape => ("cinch_tensor_shape", vec![arg(0)]),
                Data => ("cinch_tensor_data", vec![arg(0)]),
                Column => ("cinch_columnar_column", vec![arg(0)]),
                _ => unreachable!(),
            };
            let out = rt_call(b, sym, &args);
            ret(b, &out);
        }
    }
}
