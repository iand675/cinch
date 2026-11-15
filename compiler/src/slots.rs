//! The mutable slot intrinsics (`cinch.runtime.slots*`): the trusted core
//! of `Data.Mutable.*`.
//!
//! A slot array is an `Array a` block (runtime `KIND_ARRAY`, word element
//! format chosen from the element class) that one container owns outright
//! and writes in place. The container never hands out an `Array` alias of
//! it, which is what makes the in-place write unobservable; the runtime's
//! `slots.rs` states the ownership rules. Each module that uses these
//! declares them with a private `foreign import ccall` and exports none of
//! them, the same trust boundary `Effect.Stm` draws around its cell
//! intrinsics.
//!
//! * `slotsNew :: U64 -> Array a`: that many *holes* (word 0) of the
//!   element class's format.
//! * `slotsLen :: Array a -> U64`: the slot count, a field load.
//! * `slotsRead :: Array a -> U64 -> a`: the value in a slot, with its own
//!   reference (one dup for a counted class, none for a raw one).
//! * `slotsWrite :: Array a -> U64 -> a -> Unit`: store, taking over the
//!   value's reference (the unit consumes it, so no dup), and release the
//!   word the slot held.
//! * `slotsTake :: Array a -> U64 -> a`: move the value out and leave a
//!   hole: the reference moves to the caller with no count operation.
//! * `slotsMove :: Array a -> U64 -> Array a -> U64 -> U64 -> Unit` and
//!   `slotsClear :: Array a -> U64 -> U64 -> Unit`: the bulk moves and
//!   drops, runtime entries.
//! * `slotsFoldNat :: Nat -> U64`: a `Nat` modulo 2^64, the word a
//!   `Hash` digest becomes before a table mixes it.
//! * `slotsPrefetch :: Array a -> U64 -> Unit`: a hint that a slot will be
//!   read soon (`llvm.prefetch`, read, high locality). It changes no state
//!   and never faults; an index past the length issues nothing. A table
//!   uses it to start the key and value lines of a probe's home chunk
//!   while the control word is still loading, so the three misses overlap
//!   instead of running one after another.
//!
//! Indices are machine `U64` words because the containers compute them
//! with masks and shifts. Length, read, write and take are emitted inline
//! (a bounds compare, a load or store, the count operation the class
//! needs), so a probe loop costs no runtime call. An index past the length
//! and a read or take of a counted hole trap through `cinch_slots_fault`:
//! both are container bugs, never caller errors, and the trap is the
//! defence that keeps one from becoming a null block word. A raw slot has
//! no hole to detect (every word is some raw value), which is why no
//! module may export a read of an unwritten slot: a raw element type can
//! carry a refinement the zero word does not satisfy.
//!
//! Like the array and cell intrinsics, each is a unit specialized by class
//! vector, so the element class is never recovered from a type's spelling.
use crate::core::Class;
use crate::types::{Scalar, Ty};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SlotOp {
    New,
    Len,
    Read,
    Write,
    Take,
    Move,
    Clear,
    FoldNat,
    Prefetch,
}

impl SlotOp {
    pub const ALL: [Self; 9] = [
        Self::New,
        Self::Len,
        Self::Read,
        Self::Write,
        Self::Take,
        Self::Move,
        Self::Clear,
        Self::FoldNat,
        Self::Prefetch,
    ];

    /// The operation part of the intrinsic symbol (`cinch.runtime.<name>`).
    pub fn name(self) -> &'static str {
        match self {
            Self::New => "slotsNew",
            Self::Len => "slotsLen",
            Self::Read => "slotsRead",
            Self::Write => "slotsWrite",
            Self::Take => "slotsTake",
            Self::Move => "slotsMove",
            Self::Clear => "slotsClear",
            Self::FoldNat => "slotsFoldNat",
            Self::Prefetch => "slotsPrefetch",
        }
    }

    /// The type parameters of the checked signature, in class-vector order.
    pub fn params(self) -> Vec<String> {
        match self {
            Self::FoldNat => vec![],
            _ => vec!["a".into()],
        }
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        let a = Ty::Param("a".into());
        let slots = || Ty::Adt("Array".into(), vec![Ty::Param("a".into())]);
        let word = || Ty::Prim(Scalar::U64);
        match self {
            Self::New => (vec![word()], slots()),
            Self::Len => (vec![slots()], word()),
            Self::Read | Self::Take => (vec![slots(), word()], a),
            Self::Write => (vec![slots(), word(), a], Ty::Unit),
            Self::Move => (vec![slots(), word(), slots(), word(), word()], Ty::Unit),
            Self::Clear => (vec![slots(), word(), word()], Ty::Unit),
            Self::Prefetch => (vec![slots(), word()], Ty::Unit),
            Self::FoldNat => (vec![Ty::Nat], word()),
        }
    }

    /// Whether the unit takes over parameter `i`'s reference: the value a
    /// write stores.
    pub fn consumes(self, i: usize) -> bool {
        self == Self::Write && i == 2
    }
}

/// The runtime entries the emitted units call, as module declarations.
pub const DECLARES: [&str; 6] = [
    "declare void @llvm.prefetch.p0(ptr, i32, i32, i32)",
    "declare i64 @cinch_slots_new(i64, i64)",
    "declare i64 @cinch_slots_move(i64, i64, i64, i64, i64)",
    "declare i64 @cinch_slots_clear(i64, i64, i64)",
    "declare i64 @cinch_slots_fault(i64)",
    "declare i64 @cinch_nat_low_word(i64)",
];

/// Whether words of `class` carry a count, so a zero word is a hole.
pub fn counted(class: Class) -> bool {
    matches!(class, Class::Obj | Class::TObj)
}
