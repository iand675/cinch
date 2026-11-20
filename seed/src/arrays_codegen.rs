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
                        // The pair the zip builds per row, as a
                        // description word the runtime allocates from.
                        format!("i64 {}", super::description_word(reg.shape_desc(&classes, 0))),
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

/// Emit a generic array operation whose element class is wide. Array and
/// tensor storage holds owned wide boxes in object-format words; scalar
/// arguments remain i128 and Option payloads use the source Some@W layout.
/// Zip is refused by the caller until its runtime can build wide tuple fields.
pub(super) fn emit_generic_wide(
    b: &mut Builder,
    reg: &Registry,
    name: &str,
    prim: PrimOp,
    op: ArrayOp,
    classes: [Class; 2],
) {
    use ArrayOp::*;
    let arg = |i| format!("i64 %p{i}");
    let nat = |b: &mut Builder, i| rt_call(b, "cinch_nat_to_len", &[arg(i)]);
    match op {
        FromList => from_wide_list(b, reg),
        Replicate => {
            let n = nat(b, 0);
            let value = wide_box(b, reg, "%p1");
            let out = rt_call(b, "cinch_array_replicate", &[
                format!("i64 {n}"), format!("i64 {value}"), "i64 1".into(),
            ]);
            // The runtime takes its own reference for each destination slot,
            // including zero slots; release the temporary on either path.
            rc_op(b, reg.rc_debug, true, &value, Class::Obj);
            ret(b, &out);
        }
        Get | Row | TensorGet => {
            let (index, bad, sym) = if op == TensorGet {
                let i = rt_call(b, "cinch_tensor_index", &[arg(0), arg(1)]);
                let bad = b.reg();
                b.line(&format!("{bad} = icmp slt i64 {i}, 0"));
                (i, bad, "cinch_tensor_get")
            } else {
                let i = nat(b, 1);
                let bad = b.reg();
                if op == Get {
                    let valid = rt_call(b, "cinch_array_has", &[arg(0), format!("i64 {i}")]);
                    b.line(&format!("{bad} = icmp eq i64 {valid}, 0"));
                } else {
                    let len = rt_call(b, "cinch_columnar_rows", &[arg(0)]);
                    b.line(&format!("{bad} = icmp uge i64 {i}, {len}"));
                }
                (i, bad, if op == Get { "cinch_array_get" } else { "cinch_columnar_get" })
            };
            ret_option_wide_with(b, reg, name, &bad, |b| {
                let boxed = rt_call(b, sym, &[arg(0), format!("i64 {index}")]);
                // The runtime returned an owned box, so unboxing consumes it.
                wide_unbox(b, reg, &boxed)
            });
        }
        Set | TensorSet => {
            let i = if op == Set { nat(b, 1) } else {
                rt_call(b, "cinch_tensor_index", &[arg(0), arg(1)])
            };
            let value = wide_box(b, reg, "%p2");
            let sym = if op == TensorSet { "cinch_tensor_set" } else {
                let borrowed = reg.lent.get(name)
                    .and_then(|m| m.first().copied()).unwrap_or(false);
                if borrowed { "cinch_array_set" } else { "cinch_array_set_inplace" }
            };
            // Tensor index -1 is rejected by the array entry before accessing
            // storage. No source value or box is leaked on a failed update.
            let out = rt_call(b, sym, &[arg(0), format!("i64 {i}"), format!("i64 {value}")]);
            rc_op(b, reg.rc_debug, true, &value, Class::Obj);
            let bad = b.reg();
            b.line(&format!("{bad} = icmp eq i64 {out}, 0"));
            ret_option(b, reg, prim, name, &bad, |_| out);
        }
        // These entries only retain/copy container references or copy owned
        // storage words. The runtime preserves the actual object format.
        Len | Slice | Append | Reshape | Shape | Data | Column | Rows | Columns
        | Left | Right | Leaf => emit(b, reg, name, prim, op, classes, None),
        Zip => unreachable!("wide columnar zip is refused before emission"),
    }
}

/// Copy a source List@W into object-format array storage. Source Cons@W
/// contains an inline i128 head followed by its tail at byte 16; the ordinary
/// runtime list copier expects word heads and cannot read this layout.
fn from_wide_list(b: &mut Builder, reg: &Registry) {
    let cursor = b.reg();
    let count = b.reg();
    b.line(&format!("{cursor} = alloca i64, align 8"));
    b.line(&format!("{count} = alloca i64, align 8"));
    b.line(&format!("store i64 %p0, ptr {cursor}, align 8"));
    b.line(&format!("store i64 0, ptr {count}, align 8"));
    let (scan, next, allocate) = (b.label(), b.label(), b.label());
    b.line(&format!("br label %{scan}"));
    b.start(&scan);
    let word = b.reg();
    let node = b.reg();
    b.line(&format!("{word} = load i64, ptr {cursor}, align 8"));
    b.line(&format!("{node} = inttoptr i64 {word} to ptr"));
    let tag = ctor_tag(b, &node, reg.escapes());
    let cons = b.reg();
    b.line(&format!("{cons} = icmp eq i64 {tag}, 1"));
    b.line(&format!("br i1 {cons}, label %{next}, label %{allocate}"));
    b.start(&next);
    let tailp = b.reg();
    let tail = b.reg();
    let n = b.reg();
    let following = b.reg();
    b.line(&format!("{tailp} = getelementptr inbounds i8, ptr {node}, i64 16"));
    b.line(&format!("{tail} = load i64, ptr {tailp}, align 8"));
    b.line(&format!("store i64 {tail}, ptr {cursor}, align 8"));
    b.line(&format!("{n} = load i64, ptr {count}, align 8"));
    b.line(&format!("{following} = add i64 {n}, 1"));
    b.line(&format!("store i64 {following}, ptr {count}, align 8"));
    b.line(&format!("br label %{scan}"));
    b.start(&allocate);
    let len = b.reg();
    b.line(&format!("{len} = load i64, ptr {count}, align 8"));
    let out = rt_call(b, "cinch_slots_new", &[format!("i64 {len}"), "i64 1".into()]);
    let storage = b.reg();
    b.line(&format!("{storage} = inttoptr i64 {out} to ptr"));
    b.line(&format!("store i64 %p0, ptr {cursor}, align 8"));
    b.line(&format!("store i64 0, ptr {count}, align 8"));
    let (copy, item, done) = (b.label(), b.label(), b.label());
    b.line(&format!("br label %{copy}"));
    b.start(&copy);
    let i = b.reg();
    let more = b.reg();
    b.line(&format!("{i} = load i64, ptr {count}, align 8"));
    b.line(&format!("{more} = icmp ult i64 {i}, {len}"));
    b.line(&format!("br i1 {more}, label %{item}, label %{done}"));
    b.start(&item);
    let word = b.reg();
    let node = b.reg();
    let value = b.reg();
    b.line(&format!("{word} = load i64, ptr {cursor}, align 8"));
    b.line(&format!("{node} = inttoptr i64 {word} to ptr"));
    b.line(&format!("{value} = load i128, ptr {node}, align 8"));
    let boxed = wide_box(b, reg, &value);
    let offset = b.reg();
    let slot = b.reg();
    b.line(&format!("{offset} = add i64 {i}, 2"));
    b.line(&format!("{slot} = getelementptr inbounds i64, ptr {storage}, i64 {offset}"));
    b.line(&format!("store i64 {boxed}, ptr {slot}, align 8"));
    let tailp = b.reg();
    let tail = b.reg();
    let following = b.reg();
    b.line(&format!("{tailp} = getelementptr inbounds i8, ptr {node}, i64 16"));
    b.line(&format!("{tail} = load i64, ptr {tailp}, align 8"));
    b.line(&format!("store i64 {tail}, ptr {cursor}, align 8"));
    b.line(&format!("{following} = add i64 {i}, 1"));
    b.line(&format!("store i64 {following}, ptr {count}, align 8"));
    b.line(&format!("br label %{copy}"));
    b.start(&done);
    ret(b, &out);
}
