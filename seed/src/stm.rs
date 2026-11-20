//! The transactional variable intrinsics (`cinch.runtime.stm*`): the
//! typed half of the cell the runtime's `stm.rs` allocates.
//!
//! `Effect.Stm` is written in Cinch over inline target IR, and its atomic
//! operations take a cell's address and move ABI words. Five things about
//! a cell cannot be said that way, because they depend on the class of the
//! value's type, which only the compiler knows, or change what the type
//! checker or the count pass believes:
//!
//! * `stmCellNew :: a -> TVar a` allocates the cell and records the
//!   value's class as the cell's scan format, so the runtime's walks keep
//!   the value alive and release it with the cell.
//! * `stmCellAddr :: TVar a -> I64` is the cell's address for the atomic
//!   operations. An address is a plain word: it keeps nothing alive.
//! * `stmCellHold :: TVar a -> I64 -> I64` answers its second argument and
//!   is the last use of the cell an address was taken from. The count
//!   pass releases a reference after its last use, so an operation
//!   through an address is written `hold c (op (addr c))`: the cell is an
//!   argument of the call that returns the operation's result, and
//!   cannot be released before the operation ran. The result is used, so
//!   no simplification can discard the call the way it could discard an
//!   unused read of the cell.
//! * `stmCellOpen :: TVar a -> I64 -> a` gives a value word read out of the
//!   cell its type back. The word carries one reference the caller
//!   already took (or needs none); the cell argument fixes `a` and is
//!   held like `stmCellHold`'s.
//! * `stmCellCast :: TVar a -> TVar b` forgets or restores the value type
//!   of a cell, so a transaction's log can hold cells of every type in one
//!   list. A log entry is cast back only at the type it was cast from,
//!   which `Effect.Stm` guarantees by pairing every staged value with the
//!   variable it was written to.
//!
//! `stmCellOpen` and `stmCellCast` are the library's whole trusted core:
//! everything else it does is checked source. None of the five is
//! exported from `Effect.Stm`.
//!
//! Like the array intrinsics, each is a unit specialized by class vector,
//! so the element class is never recovered from a type's spelling.
use crate::types::{Scalar, Ty};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StmOp {
    CellNew,
    CellAddr,
    CellHold,
    CellOpen,
    CellCast,
}

impl StmOp {
    pub const ALL: [Self; 5] = [Self::CellNew, Self::CellAddr, Self::CellHold, Self::CellOpen, Self::CellCast];

    /// The operation part of the intrinsic symbol (`cinch.runtime.<name>`).
    pub fn name(self) -> &'static str {
        match self {
            Self::CellNew => "stmCellNew",
            Self::CellAddr => "stmCellAddr",
            Self::CellHold => "stmCellHold",
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
        let word = || Ty::Prim(Scalar::I64);
        match self {
            Self::CellNew => (vec![a.clone()], tvar(a)),
            Self::CellAddr => (vec![tvar(a)], word()),
            Self::CellHold => (vec![tvar(a), word()], word()),
            Self::CellOpen => (vec![tvar(a.clone()), word()], a),
            Self::CellCast => (vec![tvar(a)], tvar(b)),
        }
    }

    /// Whether the unit takes over parameter `i`'s reference: the value a
    /// new cell holds, and the cell a cast hands straight back.
    pub fn consumes(self, i: usize) -> bool {
        matches!(self, Self::CellNew | Self::CellCast) && i == 0
    }
}
