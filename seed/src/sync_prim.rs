//! Typed native concurrency primitives:
//! - `AtomicCell`: lock-free 64-bit hardware atomic word (`Effect.Concurrent.Atomic`).
//! - `RwLockCell a`: reader-writer lock box (`Effect.Concurrent.RwLock`).

use crate::types::Ty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AtomicOp {
    New,
    Load,
    Store,
    Swap,
    FetchAdd,
    FetchSub,
    FetchAnd,
    FetchOr,
    FetchXor,
    Cas,
    Fence,
}

impl AtomicOp {
    pub const ALL: [Self; 11] = [
        Self::New,
        Self::Load,
        Self::Store,
        Self::Swap,
        Self::FetchAdd,
        Self::FetchSub,
        Self::FetchAnd,
        Self::FetchOr,
        Self::FetchXor,
        Self::Cas,
        Self::Fence,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::New => "atomicNew",
            Self::Load => "atomicLoad",
            Self::Store => "atomicStore",
            Self::Swap => "atomicSwap",
            Self::FetchAdd => "atomicFetchAdd",
            Self::FetchSub => "atomicFetchSub",
            Self::FetchAnd => "atomicFetchAnd",
            Self::FetchOr => "atomicFetchOr",
            Self::FetchXor => "atomicFetchXor",
            Self::Cas => "atomicCas",
            Self::Fence => "atomicFence",
        }
    }

    pub fn extern_name(self) -> &'static str {
        match self {
            Self::New => "cinch_atomic_new",
            Self::Load => "cinch_atomic_load",
            Self::Store => "cinch_atomic_store",
            Self::Swap => "cinch_atomic_swap",
            Self::FetchAdd => "cinch_atomic_fetch_add",
            Self::FetchSub => "cinch_atomic_fetch_sub",
            Self::FetchAnd => "cinch_atomic_fetch_and",
            Self::FetchOr => "cinch_atomic_fetch_or",
            Self::FetchXor => "cinch_atomic_fetch_xor",
            Self::Cas => "cinch_atomic_cas",
            Self::Fence => "cinch_atomic_fence",
        }
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        use AtomicOp::*;
        let i64t = Ty::Prim(crate::types::Scalar::I64);
        let cell = Ty::Adt("AtomicCell".into(), vec![]);
        match self {
            New => (vec![i64t], cell),
            Load => (vec![cell, i64t.clone()], i64t),
            Store => (vec![cell, i64t.clone(), i64t], Ty::Unit),
            Swap | FetchAdd | FetchSub | FetchAnd | FetchOr | FetchXor => {
                (vec![cell, i64t.clone(), i64t.clone()], i64t)
            }
            Cas => (
                vec![cell, i64t.clone(), i64t.clone(), i64t.clone(), i64t],
                Ty::Bool,
            ),
            Fence => (vec![i64t], Ty::Unit),
        }
    }

    pub fn consumes(self, _i: usize) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RwLockOp {
    New,
    Read,
    Write,
    Swap,
    TryRead,
    TryWrite,
}

impl RwLockOp {
    pub const ALL: [Self; 6] = [
        Self::New,
        Self::Read,
        Self::Write,
        Self::Swap,
        Self::TryRead,
        Self::TryWrite,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::New => "rwlockNew",
            Self::Read => "rwlockRead",
            Self::Write => "rwlockWrite",
            Self::Swap => "rwlockSwap",
            Self::TryRead => "rwlockTryRead",
            Self::TryWrite => "rwlockTryWrite",
        }
    }

    pub fn params(self) -> Vec<String> {
        vec!["a".into()]
    }

    pub fn returns_option(self) -> bool {
        self == Self::TryRead
    }

    pub fn blocking(self) -> bool {
        matches!(self, Self::Read | Self::Write | Self::Swap)
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        use RwLockOp::*;
        let a = Ty::Param("a".into());
        let cell = Ty::Adt("RwLockCell".into(), vec![a.clone()]);
        let option = Ty::Adt(crate::known::OPTION.into(), vec![a.clone()]);
        match self {
            New => (vec![a], cell),
            Read => (vec![cell], a),
            Write => (vec![cell, a], Ty::Unit),
            Swap => (vec![cell, a.clone()], a),
            TryRead => (vec![cell], option),
            TryWrite => (vec![cell, a], Ty::Bool),
        }
    }

    /// New/Write/Swap/TryWrite take ownership of the supplied value word on
    /// every outcome (the runtime drops it if TryWrite fails).
    pub fn consumes(self, i: usize) -> bool {
        (self == Self::New && i == 0)
            || (i == 1 && matches!(self, Self::Write | Self::Swap | Self::TryWrite))
    }
}
