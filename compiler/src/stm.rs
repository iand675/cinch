//! The transactional variable intrinsics (`cinch.runtime.stm*`): the
//! typed half of the cell the runtime's `stm.rs` allocates.
//!
//! `Effect.Stm` is written in Cinch over inline target IR, and its atomic
//! operations take a cell's address and move ABI words. Four things about
//! a cell cannot be said that way, because they depend on the class of the
//! value's type, which only the compiler knows, or change what the type
//! checker believes:
//!
//! * `stmCellNew :: a -> TVar a` allocates the cell and records the
//!   value's class as the cell's scan format, so the runtime's walks keep
//!   the value alive and release it with the cell.
//! * `stmCellAddr :: TVar a -> I64` is the cell's address for the atomic
//!   operations. The caller keeps its own reference to the cell for as
//!   long as it uses the address.
//! * `stmCellOpen :: TVar a -> I64 -> a` gives a value word read out of the
//!   cell its type back. The word carries one reference the caller
//!   already took (or needs none); the cell argument only fixes `a`.
//! * `stmCellCast :: TVar a -> TVar b` forgets or restores the value type
//!   of a cell, so a transaction's log can hold cells of every type in one
//!   list. A log entry is cast back only at the type it was cast from,
//!   which `Effect.Stm` guarantees by pairing every staged value with the
//!   variable it was written to.
//!
//! The last two are the library's whole trusted core: everything else it
//! does is checked source. None of the four is exported from `Effect.Stm`.
//!
//! Like the array intrinsics, each is a unit specialized by class vector,
//! so the element class is never recovered from a type's spelling.
use crate::core::Class;
use crate::types::{Scalar, Ty};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StmOp {
    CellNew,
    CellAddr,
    CellOpen,
    CellCast,
}

impl StmOp {
    pub const ALL: [Self; 4] = [Self::CellNew, Self::CellAddr, Self::CellOpen, Self::CellCast];

    /// The operation part of the intrinsic symbol (`cinch.runtime.<name>`).
    pub fn name(self) -> &'static str {
        match self {
            Self::CellNew => "stmCellNew",
            Self::CellAddr => "stmCellAddr",
            Self::CellOpen => "stmCellOpen",
            Self::CellCast => "stmCellCast",
        }
    }

    /// The type parameters of the checked signature, in class-vector order.
    pub fn params(self) -> Vec<String> {
        match self {
            Self::CellCast => vec!["a".into(), "b".into()],
            _ => vec!["a".into()],
        }
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        let a = Ty::Param("a".into());
        let b = Ty::Param("b".into());
        let tvar = |t| Ty::Adt("TVar".into(), vec![t]);
        let word = Ty::Prim(Scalar::I64);
        match self {
            Self::CellNew => (vec![a.clone()], tvar(a)),
            Self::CellAddr => (vec![tvar(a)], word),
            Self::CellOpen => (vec![tvar(a.clone()), word], a),
            Self::CellCast => (vec![tvar(a)], tvar(b)),
        }
    }

    /// Whether the unit takes over parameter `i`'s reference: the value a
    /// new cell holds, and the cell a cast hands straight back.
    pub fn consumes(self, i: usize) -> bool {
        matches!(self, Self::CellNew | Self::CellCast) && i == 0
    }

    /// The class of the unit's result under the class vector `classes`.
    pub fn result_class(self, classes: [Class; 2]) -> Class {
        match self {
            Self::CellNew | Self::CellCast => Class::Obj,
            Self::CellAddr => Class::Raw,
            Self::CellOpen => classes[0],
        }
    }
}
