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
    /// A POD unboxed value aggregate (`#[repr(value)]` record or
    /// `FixedArray n a`) of `size` bytes and `align` alignment. Never
    /// refcounted; specialized by `(size, align)` across polymorphic units,
    /// stored inline as raw words in ADTs/records/closures, and passed by
    /// registers (`<= 16B, align <= 16`) or `readonly ptr` + `sret`.
    Val { size: u32, align: u16 },
    /// The class of the enclosing unit's type parameter at this index.
    /// Only a polymorphic unit's generic body carries it; the collector
    /// substitutes every one before emission.
    Param(u16),
    /// Polymorphic `Obj`-pointer niche sum whose required type parameters are
    /// the bitmask `u16`: substitutes to `TObj` when every bit resolves to
    /// `Obj`, or falls back to boxed `Obj` otherwise (resolved in `specialize.rs`).
    PolyNiche(u16),
}

impl Class {
    /// Resolve a parameter class against an instantiation vector. An
    /// index outside the vector stays a parameter, which the collector's
    /// final check reports.
    pub fn subst(self, v: &[Class]) -> Class {
        match self {
            Class::Param(i) => v.get(i as usize).copied().unwrap_or(self),
            Class::PolyNiche(mask) => {
                let mut unresolved: u16 = 0;
                for i in 0..16u16 {
                    if (mask & (1u16 << i)) != 0 {
                        match v.get(i as usize).copied() {
                            Some(Class::Obj) => {}
                            Some(Class::Param(j)) if j < 16 => unresolved |= 1u16 << j,
                            _ => return Class::Obj,
                        }
                    }
                }
                if unresolved != 0 {
                    Class::PolyNiche(unresolved)
                } else {
                    Class::TObj
                }
            }
            c => c,
        }
    }

    pub fn is_param(self) -> bool {
        matches!(self, Class::Param(_) | Class::PolyNiche(_))
    }

    pub fn is_val(self) -> bool {
        matches!(self, Class::Val { .. })
    }

    pub fn val_layout(self) -> Option<(u64, u64)> {
        match self {
            Class::Val { size, align } => Some((u64::from(size), u64::from(align).max(1))),
            _ => None,
        }
    }

    /// The 8-byte words this class occupies in a constructor or closure
    /// layout: two for `Wide`, `ceil(size/8).max(1)` for `Val` (at least
    /// one word for zero-sized values so closure partial application
    /// counts always advance), and one for every word class.
    pub fn words(self) -> usize {
        match self {
            Class::Wide => 2,
            Class::Val { size, .. } => (size as usize).div_ceil(8).max(1),
            _ => 1,
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

    pub fn append_token(self, out: &mut String) {
        match self {
            Class::Raw => out.push('R'),
            Class::Obj => out.push('O'),
            Class::TObj => out.push('T'),
            Class::Wide => out.push('W'),
            Class::Val { size, align } => {
                use std::fmt::Write as _;
                let _ = write!(out, "V{size}a{align}");
            }
            Class::Param(i) => {
                use std::fmt::Write as _;
                let _ = write!(out, "P{i}_");
            }
            Class::PolyNiche(mask) => {
                use std::fmt::Write as _;
                let _ = write!(out, "N{mask}_");
            }
        }
    }
}

/// Parse a class vector encoded by [`letters`]. Accepts `R`, `O`, `T`, `W`,
/// `V<size>a<align>`, and (when `allow_param` is true) `P<idx>_`.
pub fn parse_classes(s: &str, allow_param: bool) -> Option<Vec<Class>> {
    let b = s.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        match b[i] {
            b'R' => { out.push(Class::Raw); i += 1; }
            b'O' => { out.push(Class::Obj); i += 1; }
            b'T' => { out.push(Class::TObj); i += 1; }
            b'W' => { out.push(Class::Wide); i += 1; }
            b'V' => {
                i += 1;
                let s_start = i;
                while i < b.len() && b[i].is_ascii_digit() { i += 1; }
                if i == s_start || i >= b.len() || b[i] != b'a' { return None; }
                let size: u32 = s[s_start..i].parse().ok()?;
                i += 1;
                let a_start = i;
                while i < b.len() && b[i].is_ascii_digit() { i += 1; }
                if i == a_start { return None; }
                let align: u16 = s[a_start..i].parse().ok()?;
                out.push(Class::Val { size, align });
            }
            b'P' if allow_param => {
                i += 1;
                let p_start = i;
                while i < b.len() && b[i].is_ascii_digit() { i += 1; }
                if i == p_start || i >= b.len() || b[i] != b'_' { return None; }
                let idx: u16 = s[p_start..i].parse().ok()?;
                i += 1;
                out.push(Class::Param(idx));
            }
            _ => return None,
        }
    }
    Some(out)
}

/// Whether `name` is a specialized unit whose class vector contains at least
/// one `Class::Val` (e.g. `Lib.id@V24a8`, `Lib.MkBox@OV16a8`). Consumers emit
/// external `Val` specializations privately so prebuilt dependency objects
/// never need to anticipate downstream unboxed layouts.
pub fn is_val_specialization(name: &str) -> bool {
    let base = name
        .strip_suffix("#tail")
        .or_else(|| name.strip_suffix("#b"))
        .unwrap_or(name);
    let Some((_, suffix)) = base.rsplit_once('@') else {
        return false;
    };
    let vec_part = suffix.split('$').next().unwrap_or(suffix);
    let vec_part = vec_part.split('.').next().unwrap_or(vec_part);
    parse_classes(vec_part, false).is_some_and(|cs| cs.iter().any(|c| c.is_val()))
}

/// Substitute a whole vector.
pub fn subst_all(cs: &[Class], v: &[Class]) -> Vec<Class> {
    cs.iter().map(|c| c.subst(v)).collect()
}

/// The identity vector of a unit with `n` type parameters.
pub fn identity(n: usize) -> Vec<Class> {
    (0..n).map(|i| Class::Param(i as u16)).collect()
}

/// The suffix tokens of a class vector (`RO` for `[Raw, Obj]`, `V24a8R` for `[Val(24,8), Raw]`).
pub fn letters(v: &[Class]) -> String {
    let mut out = String::with_capacity(v.len());
    for c in v {
        c.append_token(&mut out);
    }
    out
}

/// The unit name of one specialized copy: the generic name, `@`, and the
/// vector's tokens (`Main.pair@RO`, `Main.id@V24a8`). The copy's companion
/// symbols (`.clo`, `.app`, `#tail`) follow the suffix, so the incremental
/// splitter's unit rule owns them, and [`base_unit`] recovers the
/// declaration's own symbol.
pub fn spec_name(base: &str, v: &[Class]) -> String {
    format!("{base}@{}", letters(v))
}

/// The generic symbol a specialized symbol was copied from: the one
/// `@<tokens>` segment followed by the end, `.` or `#` is removed. Source
/// operator names may contain `@`, but never `@` directly followed by a
/// class token, so the segment is unambiguous.
pub fn base_unit(sym: &str) -> std::borrow::Cow<'_, str> {
    let b = sym.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'@' {
            let mut j = i + 1;
            let mut ok = true;
            while j < b.len() && b[j] != b'.' && b[j] != b'#' {
                match b[j] {
                    b'R' | b'O' | b'T' | b'W' => j += 1,
                    b'V' => {
                        j += 1;
                        let s0 = j;
                        while j < b.len() && b[j].is_ascii_digit() { j += 1; }
                        if j == s0 || j >= b.len() || b[j] != b'a' { ok = false; break; }
                        j += 1;
                        let a0 = j;
                        while j < b.len() && b[j].is_ascii_digit() { j += 1; }
                        if j == a0 { ok = false; break; }
                    }
                    _ => { ok = false; break; }
                }
            }
            if ok && j > i + 1 && (j == b.len() || b[j] == b'.' || b[j] == b'#') {
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
    /// A POD unboxed value record (`#[repr(value)]`): never heap-allocated
    /// or refcounted; references (`Ref`/`Mut`) to it are raw native pointers,
    /// and its by-value class is `Class::Val { size, align }`.
    Value { size: u32, align: u16 },
    /// A refinement type: its runtime form is the container itself.
    Base(Ty),
    /// A polymorphic pointer-tagged niche sum requiring all type arguments
    /// at set bit positions in `u16` to have class `Obj`.
    PolyNiche(u16),
}

/// Whether `ty` is a POD unboxed value type (`#[repr(value)]` record or
/// `FixedArray n a`) whose `Ref`/`Mut` borrows are uncounted native pointers.
pub fn is_value_ty(ty: &Ty, types: &dyn TypeReprs) -> bool {
    match ty {
        Ty::Adt(name, args) => match name.as_str() {
            "FixedArray" => true,
            "refined" => args.get(1).is_some_and(|b| is_value_ty(b, types)),
            "Sing" => args.first().is_some_and(|b| is_value_ty(b, types)),
            _ => match types.adt(name, args) {
                AdtRepr::Value { .. } => true,
                AdtRepr::Base(b) => is_value_ty(&b, types),
                _ => false,
            },
        },
        _ => false,
    }
}

/// The checker's view of named types, for [`repr_of`].
pub trait TypeReprs {
    fn adt(&self, name: &str, args: &[Ty]) -> AdtRepr;
    fn fixed_array_layout(&self, _args: &[Ty]) -> Option<(u32, u16)> {
        None
    }
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
        // Shared access preserves the referent word (or is a raw pointer for
        // unboxed value types). Exclusive access carries the owned mutable
        // cell for boxed/scalar types, or a raw native pointer for unboxed
        // value types (`#[repr(value)]` / `FixedArray`).
        Ty::Ref(_, a) => {
            if is_value_ty(a, types) {
                Class::Raw
            } else {
                repr_of(a, params, types, miss)
            }
        }
        Ty::Mut(_, a) => {
            if is_value_ty(a, types) {
                Class::Raw
            } else {
                Class::Obj
            }
        }
        Ty::Adt(name, args) => match name.as_str() {
            "FixedArray" => match types.fixed_array_layout(args) {
                Some((size, align)) => Class::Val { size, align },
                None => Class::Raw,
            },
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
                AdtRepr::Value { size, align } => Class::Val { size, align },
                AdtRepr::Base(b) => repr_of(&b, params, types, miss),
                AdtRepr::PolyNiche(req_mask) => {
                    let mut unresolved: u16 = 0;
                    for i in 0..16usize {
                        if (req_mask & (1u16 << i)) != 0 {
                            let Some(arg) = args.get(i) else { return Class::Obj; };
                            match repr_of(arg, params, types, miss) {
                                Class::Obj => {}
                                Class::Param(j) if j < 16 => unresolved |= 1u16 << j,
                                _ => return Class::Obj,
                            }
                        }
                    }
                    if unresolved != 0 {
                        Class::PolyNiche(unresolved)
                    } else {
                        Class::TObj
                    }
                }
            },
        },
        Ty::Tuple(_) | Ty::Sigma { .. } | Ty::App(..) | Ty::Former(_) => Class::Obj,
        Ty::Fun(..) | Ty::Pi { .. } | Ty::Thunk { .. } | Ty::Later { .. } | Ty::Lazy { .. } | Ty::Handler { .. } => {
            Class::Obj
        }
        // A polymorphic value is its family's dictionary block
        // (design/rank-n-polymorphism.md section 2).
        Ty::Forall(_) | Ty::Qual(_, _) => Class::Obj,
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
