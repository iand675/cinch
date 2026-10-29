//! Representation classes of words (reclamation design section 2.2;
//! layouts and specialization design section 2, option C).
//!
//! Every `Core` value is one `i64` word. A word's class says what a
//! reclamation pass may do with it: `Raw` words are never counted, `Obj`
//! words are always counted pointers (possibly to immortal blocks), and
//! `TObj` words are either an immediate with low bit 1 or an `Obj`.
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

    fn letter(self) -> char {
        match self {
            Class::Raw => 'R',
            Class::Obj => 'O',
            Class::TObj => 'T',
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
            while j < b.len() && matches!(b[j], b'R' | b'O' | b'T') {
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
    /// A foreign opaque type or a runtime handle: never counted.
    Raw,
    /// A refinement type: its runtime form is the container itself.
    Base(Ty),
}

/// The checker's view of named types, for [`repr_of`].
pub trait TypeReprs {
    fn adt(&self, name: &str, args: &[Ty]) -> AdtRepr;
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
        Ty::Bool | Ty::Unit | Ty::Sym | Ty::Prim(_) | Ty::Region(_) | Ty::Ptr { .. } => Class::Raw,
        // Erased evidence and types occupy no counted word.
        Ty::Sort(_) | Ty::Idx(_) | Ty::Prp(_) => Class::Raw,
        Ty::Bytes => Class::Obj,
        Ty::Param(n) => match params.iter().position(|p| p == n) {
            Some(i) => Class::Param(i as u16),
            None => {
                miss(ty);
                Class::Raw
            }
        },
        Ty::Meta(_) => {
            miss(ty);
            Class::Raw
        }
        // A borrow elaborates to its operand's own word.
        Ty::Ref(_, a) | Ty::Mut(_, a) => repr_of(a, params, types, miss),
        Ty::Adt(name, args) => match name.as_str() {
            "refined" => match args.as_slice() {
                [_, base] => repr_of(base, params, types, miss),
                _ => {
                    miss(ty);
                    Class::Raw
                }
            },
            // Capability evidence and structured-concurrency handles are
            // runtime-owned words.
            "Io" | "Scope" | "Task" | "IoCap" | "FileReadCap" | "FileWriteCap" | "ProcCap"
            | "Secret" | "SchemaSnapshot" | "Database" | "SchemaRead" => Class::Raw,
            _ => match types.adt(name, args) {
                AdtRepr::Obj => Class::Obj,
                AdtRepr::Raw => Class::Raw,
                AdtRepr::Base(b) => repr_of(&b, params, types, miss),
            },
        },
        Ty::Tuple(_) | Ty::Sigma { .. } | Ty::App(..) | Ty::Former(_) => Class::Obj,
        Ty::Fun(..) | Ty::Pi { .. } | Ty::Thunk { .. } | Ty::Later(_) | Ty::Lazy { .. } | Ty::Handler { .. } => {
            Class::Obj
        }
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
