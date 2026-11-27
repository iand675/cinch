//! Representation classes of words (reclamation design section 2.2;
//! layouts and specialization design section 2, option C).
//!
//! Every `Core` value is one `i64` word, except a wide scalar's. A
//! word's class says what a reclamation pass may do with it: `Raw` words
//! are never counted, `Obj` words are always counted pointers (possibly
//! to immortal blocks), and `TObj` words are either an immediate with low
//! bit 1 or an `Obj`. `Wide` is the one class that is not a word: an
//! `I128`/`U128` value held as LLVM `i128`, never counted, two words in
//! any layout (`design/wide-scalars.md`).
//! Classes are attached to binders, constructor fields and instantiation
//! sites during lowering, while the checker still knows the types. Option
//! C's collector copies each polymorphic unit once per reached vector of
//! its type parameters' classes, named by [`spec_name`].

use crate::types::Ty;

/// One word's representation class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Class {
    /// Never counted: machine scalars, `Bool`, `Char`, `Unit`, `Sym`,
    /// runtime handles and capabilities, erased evidence.
    Raw,
    /// A counted pointer to a heap or static block: constructor blocks,
    /// tuples, closures, dictionaries, bytes, strings, memo cells, quotes.
    Obj,
    /// A `Nat`/`Int` word: an immediate with low bit 1, or a pointer.
    TObj,
    /// An `I128`/`U128` value: LLVM `i128` in a wide position, two raw
    /// words in a layout, never counted. A word-uniform position (the
    /// apply frame, a closure environment, a dictionary value slot, a
    /// cell) holds it boxed, as an `Obj` word (`crate::wide`).
    Wide,
    /// The class of the enclosing unit's type parameter at this index.
    /// Only a polymorphic unit's generic body carries it; the collector
    /// substitutes every one before emission.
    Param(u16),
}

impl Class {
    /// Resolve a parameter class against an instantiation vector. An
    /// index outside the vector stays a parameter, which the collector's
    /// final check reports.
    pub fn subst(self, v: &[Class]) -> Class {
        match self {
            Class::Param(i) => v.get(i as usize).copied().unwrap_or(self),
            c => c,
        }
    }

    pub fn is_param(self) -> bool {
        matches!(self, Class::Param(_))
    }

    /// The words this class occupies in a layout: two for `Wide`, one
    /// for every word class.
    pub fn words(self) -> usize {
        if self == Class::Wide {
            2
        } else {
            1
        }
    }

    /// The class's suffix letter, or `None` for a letter no class has.
    pub fn from_letter(c: char) -> Option<Class> {
        Some(match c {
            'R' => Class::Raw,
            'O' => Class::Obj,
            'T' => Class::TObj,
            'W' => Class::Wide,
            _ => return None,
        })
    }

    fn letter(self) -> char {
        match self {
            Class::Raw => 'R',
            Class::Obj => 'O',
            Class::TObj => 'T',
            Class::Wide => 'W',
            Class::Param(_) => 'P',
        }
    }
}

/// Substitute a whole vector.
pub fn subst_all(cs: &[Class], v: &[Class]) -> Vec<Class> {
    cs.iter().map(|c| c.subst(v)).collect()
}

/// The identity vector of a unit with `n` type parameters.
pub fn identity(n: usize) -> Vec<Class> {
    (0..n).map(|i| Class::Param(i as u16)).collect()
}

/// The suffix letters of a concrete vector (`RO` for `[Raw, Obj]`).
pub fn letters(v: &[Class]) -> String {
    v.iter().map(|c| c.letter()).collect()
}

/// The unit name of one specialized copy: the generic name, `@`, and the
/// vector's letters (`Main.pair@RO`). The copy's companion symbols
/// (`.clo`, `.app`, `#tail`) follow the suffix, so the incremental
/// splitter's unit rule owns them, and [`base_unit`] recovers the
/// declaration's own symbol.
pub fn spec_name(base: &str, v: &[Class]) -> String {
    format!("{base}@{}", letters(v))
}

/// The generic symbol a specialized symbol was copied from: the one
/// `@<letters>` segment followed by the end, `.` or `#` is removed. Source
/// operator names may contain `@`, but never `@` directly followed by a
/// letter, so the segment is unambiguous.
pub fn base_unit(sym: &str) -> std::borrow::Cow<'_, str> {
    let b = sym.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'@' {
            let mut j = i + 1;
            while j < b.len() && matches!(b[j], b'R' | b'O' | b'T' | b'W') {
                j += 1;
            }
            if j > i + 1 && (j == b.len() || b[j] == b'.' || b[j] == b'#') {
                let mut out = String::with_capacity(sym.len());
                out.push_str(&sym[..i]);
                out.push_str(&sym[j..]);
                return std::borrow::Cow::Owned(out);
            }
        }
        i += 1;
    }
    std::borrow::Cow::Borrowed(sym)
}

/// What a named type's own definition says about its words.
pub enum AdtRepr {
    /// A data or record type: constructor blocks (nullary ones are
    /// static blocks, still `Obj`).
    Obj,
    /// A tagged union word (heap pointer or odd immediate, e.g. `#[repr(niche)]`
    /// over a heap pointer with 2+ nullary constructors or over `Nat`/`Int`).
    TObj,
    /// A foreign opaque type or a runtime handle: never counted.
    Raw,
    /// A refinement type: its runtime form is the container itself.
    Base(Ty),
}

/// The checker's view of named types, for [`repr_of`].
pub trait TypeReprs {
    fn adt(&self, name: &str, args: &[Ty]) -> AdtRepr;
    /// Pointer width in bits for target-dependent immediate bounds (default 64).
    fn ptr_bits(&self) -> u32 {
        64
    }
}

fn idx_int_const(t: &crate::types::ITerm) -> Option<i128> {
    use crate::types::{ITerm, IdxOp};
    match t {
        ITerm::Num(s) => s.as_str().parse::<i128>().ok(),
        ITerm::Bin(IdxOp::Sub, l, r) => {
            if idx_int_const(l) == Some(0) {
                idx_int_const(r).and_then(|v| v.checked_neg())
            } else {
                idx_int_const(l)?.checked_sub(idx_int_const(r)?)
            }
        }
        ITerm::Bin(IdxOp::Add, l, r) => idx_int_const(l)?.checked_add(idx_int_const(r)?),
        ITerm::Bin(IdxOp::Mul, l, r) => idx_int_const(l)?.checked_mul(idx_int_const(r)?),
        _ => None,
    }
}

fn merge_lo(a: Option<i128>, b: Option<i128>) -> Option<i128> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(x), Some(y)) => Some(x.max(y)),
    }
}

fn merge_hi(a: Option<i128>, b: Option<i128>) -> Option<i128> {
    match (a, b) {
        (None, x) | (x, None) => x,
        (Some(x), Some(y)) => Some(x.min(y)),
    }
}

/// Extract conservative inclusive `[lo, hi]` bounds on `binder` from a
/// refinement predicate conjunction.
pub fn extract_pred_bounds(binder: &str, pred: &crate::types::PTerm) -> (Option<i128>, Option<i128>) {
    use crate::types::{ITerm, IdxCmp, PTerm};
    match pred {
        PTerm::And(a, b) => {
            let (la, ha) = extract_pred_bounds(binder, a);
            let (lb, hb) = extract_pred_bounds(binder, b);
            (merge_lo(la, lb), merge_hi(ha, hb))
        }
        PTerm::Eq(l, r) => {
            if matches!(l.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                if let Some(k) = idx_int_const(r) {
                    return (Some(k), Some(k));
                }
            } else if matches!(r.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                if let Some(k) = idx_int_const(l) {
                    return (Some(k), Some(k));
                }
            }
            (None, None)
        }
        PTerm::Cmp(op, l, r) => {
            if matches!(l.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                if let Some(k) = idx_int_const(r) {
                    return match op {
                        IdxCmp::Lt => (None, k.checked_sub(1)),
                        IdxCmp::Le => (None, Some(k)),
                        IdxCmp::Gt => (k.checked_add(1), None),
                        IdxCmp::Ge => (Some(k), None),
                        IdxCmp::Eq => (Some(k), Some(k)),
                        IdxCmp::Ne => (None, None),
                    };
                }
            } else if matches!(r.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                if let Some(k) = idx_int_const(l) {
                    return match op {
                        IdxCmp::Lt => (k.checked_add(1), None),
                        IdxCmp::Le => (Some(k), None),
                        IdxCmp::Gt => (None, k.checked_sub(1)),
                        IdxCmp::Ge => (None, Some(k)),
                        IdxCmp::Eq => (Some(k), Some(k)),
                        IdxCmp::Ne => (None, None),
                    };
                }
            }
            (None, None)
        }
        _ => (None, None),
    }
}

/// Extract constant integer values explicitly excluded from `binder` by `pred`
/// (e.g. `x /= 0`, `NonZero x`, `not (x == 0)`).
pub fn extract_pred_exclusions(binder: &str, pred: &crate::types::PTerm) -> Vec<i128> {
    use crate::types::{ITerm, IdxCmp, PTerm, PredFn};
    let mut out = Vec::new();
    fn go(binder: &str, pred: &PTerm, out: &mut Vec<i128>) {
        match pred {
            PTerm::And(a, b) => {
                go(binder, a, out);
                go(binder, b, out);
            }
            PTerm::Pred(PredFn::NonZero, x) => {
                if matches!(x.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                    out.push(0);
                }
            }
            PTerm::Cmp(IdxCmp::Ne, l, r) => {
                if matches!(l.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                    if let Some(k) = idx_int_const(r) {
                        out.push(k);
                    }
                } else if matches!(r.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                    if let Some(k) = idx_int_const(l) {
                        out.push(k);
                    }
                }
            }
            PTerm::Not(inner) => match inner.as_ref() {
                PTerm::Eq(l, r) | PTerm::Cmp(IdxCmp::Eq, l, r) => {
                    if matches!(l.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                        if let Some(k) = idx_int_const(r) {
                            out.push(k);
                        }
                    } else if matches!(r.as_ref(), ITerm::Var(v) if v.as_str() == binder) {
                        if let Some(k) = idx_int_const(l) {
                            out.push(k);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
    go(binder, pred, &mut out);
    out.sort_unstable();
    out.dedup();
    out
}

/// First free variable in a refinement predicate (matches `check::pred_binder`).
pub fn anonymous_pred_binder(pred: &crate::types::PTerm) -> Option<String> {
    let mut fv = Vec::new();
    pred.free_vars(&mut fv);
    fv.into_iter().next()
}

/// True when `base` is `Nat` or `Int` and `pred` proves every inhabitant fits
/// strictly inside the target's tagged-immediate range (so the runtime word
/// always carries low bit 1 and never allocates a heap bignum, giving RC class
/// `Class::Raw` while keeping the tagged-immediate bit encoding).
pub fn refinement_fits_immediate(
    base: &Ty,
    binder: &str,
    pred: &crate::types::PTerm,
    ptr_bits: u32,
) -> bool {
    let wasm32 = ptr_bits <= 32;
    let (lo, hi) = extract_pred_bounds(binder, pred);
    match base {
        Ty::Nat => {
            let max_nat: i128 = if wasm32 { 2_147_483_647 } else { 4_611_686_018_427_387_903 };
            hi.is_some_and(|h| h <= max_nat)
        }
        Ty::Int => {
            let max_int: i128 = if wasm32 { 1_073_741_823 } else { 4_611_686_018_427_387_903 };
            let min_int: i128 = -max_int;
            lo.is_some_and(|l| l >= min_int) && hi.is_some_and(|h| h <= max_int)
        }
        _ => false,
    }
}

/// The class of a zonked type, where `params` are the rigid names of the
/// enclosing unit's type parameters in vector order. A type variable
/// outside `params` or an undetermined metavariable has no class of its
/// own; `miss` is told and the word is `Raw` (never counted), which can
/// only leak, never free a live block.
///
/// Termination of the collector rests on the `App`, `Adt` and `Tuple`
/// cases: a constructor-applied type is `Obj` whatever its arguments, so
/// a class vector never grows with the type.
pub fn repr_of(ty: &Ty, params: &[String], types: &dyn TypeReprs, miss: &mut dyn FnMut(&Ty)) -> Class {
    match ty {
        Ty::Int | Ty::Nat => Class::TObj,
        Ty::Prim(s) if s.is_wide() => Class::Wide,
        Ty::Bool | Ty::Unit | Ty::Sym | Ty::Prim(_) | Ty::Region(_) | Ty::Ptr { .. } => Class::Raw,
        // Erased evidence and types occupy no counted word.
        Ty::Sort(_) | Ty::Idx(_) | Ty::Prp(_) => Class::Raw,
        Ty::Bytes => Class::Obj,
        Ty::Param(n) => match params.iter().position(|p| p == n) {
            Some(i) => Class::Param(i as u16),
            None if n.starts_with("$gadt_t_") => Class::Obj,
            None if n.starts_with("$sing_") => Class::Raw,
            None => {
                miss(ty);
                Class::Raw
            }
        },
        Ty::Meta(_) => {
            miss(ty);
            Class::Raw
        }
        // Shared access preserves the referent word. Exclusive access
        // carries the owned mutable cell so a call preserves its identity.
        Ty::Ref(_, a) => repr_of(a, params, types, miss),
        Ty::Mut(..) => Class::Obj,
        Ty::Adt(name, args) => match name.as_str() {
            "refined" => match args.as_slice() {
                [Ty::Prp(pred), base] => {
                    if let Some(binder) = anonymous_pred_binder(pred) {
                        if refinement_fits_immediate(base, &binder, pred, types.ptr_bits()) {
                            return Class::Raw;
                        }
                    }
                    repr_of(base, params, types, miss)
                }
                [_, base] => repr_of(base, params, types, miss),
                _ => {
                    miss(ty);
                    Class::Raw
                }
            },
            // Zero-cost runtime singleton: `Sing T x` has the exact same runtime
            // representation and reference-counting class as its carrier `T`.
            "Sing" => match args.as_slice() {
                [carrier, _] => repr_of(carrier, params, types, miss),
                _ => {
                    miss(ty);
                    Class::Raw
                }
            },
            // Built-in poly-kinded equality witness (`Refl` is a static nullary block).
            "Same" => Class::Obj,
            // Capability evidence and structured-concurrency handles are
            // runtime-owned words.
            "Io" | "Scope" | "Task" | "IoCap" | "FileReadCap" | "FileWriteCap" | "ProcCap"
            | "Secret" | "SchemaSnapshot" | "Database" | "SchemaRead" => Class::Raw,
            _ => match types.adt(name, args) {
                AdtRepr::Obj => Class::Obj,
                AdtRepr::TObj => Class::TObj,
                AdtRepr::Raw => Class::Raw,
                AdtRepr::Base(b) => repr_of(&b, params, types, miss),
            },
        },
        Ty::Tuple(_) | Ty::Sigma { .. } | Ty::App(..) | Ty::Former(_) => Class::Obj,
        Ty::Fun(..) | Ty::Pi { .. } | Ty::Thunk { .. } | Ty::Later { .. } | Ty::Lazy { .. } | Ty::Handler { .. } => {
            Class::Obj
        }
        // A polymorphic value is its family's dictionary block
        // (design/rank-n-polymorphism.md section 2).
        Ty::Forall(_) => Class::Obj,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_unit_strips_one_vector_segment() {
        assert_eq!(base_unit("Main.pair@RO"), "Main.pair");
        assert_eq!(base_unit("Main.pair@RO.clo"), "Main.pair.clo");
        assert_eq!(base_unit("Main.go@T#tail"), "Main.go#tail");
        assert_eq!(base_unit("Main.Show_List@O.mk"), "Main.Show_List.mk");
        assert_eq!(base_unit("Main.<@>"), "Main.<@>");
        assert_eq!(base_unit("Main.@@"), "Main.@@");
        assert_eq!(spec_name("Main.f.lam3", &[Class::Raw, Class::TObj]), "Main.f.lam3@RT");
    }
}
