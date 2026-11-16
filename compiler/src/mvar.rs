//! Typed native MVar intrinsics. `MVarCell a` is an opaque runtime box;
//! the public library wraps it without exposing its constructor.
use crate::types::Ty;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MVarOp {
    New,
    NewEmpty,
    Take,
    Put,
    Read,
    Swap,
    TryTake,
    TryPut,
    TryRead,
    IsEmpty,
    TakeOrCancel,
    PutOrCancel,
    ReadOrCancel,
    SwapOrCancel,
}

impl MVarOp {
    pub const ALL: [Self; 14] = [
        Self::New,
        Self::NewEmpty,
        Self::Take,
        Self::Put,
        Self::Read,
        Self::Swap,
        Self::TryTake,
        Self::TryPut,
        Self::TryRead,
        Self::IsEmpty,
        Self::TakeOrCancel,
        Self::PutOrCancel,
        Self::ReadOrCancel,
        Self::SwapOrCancel,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::New => "mvarNew",
            Self::NewEmpty => "mvarNewEmpty",
            Self::Take => "mvarTake",
            Self::Put => "mvarPut",
            Self::Read => "mvarRead",
            Self::Swap => "mvarSwap",
            Self::TryTake => "mvarTryTake",
            Self::TryPut => "mvarTryPut",
            Self::TryRead => "mvarTryRead",
            Self::IsEmpty => "mvarIsEmpty",
            Self::TakeOrCancel => "mvarTakeOrCancel",
            Self::PutOrCancel => "mvarPutOrCancel",
            Self::ReadOrCancel => "mvarReadOrCancel",
            Self::SwapOrCancel => "mvarSwapOrCancel",
        }
    }

    pub fn params(self) -> Vec<String> {
        vec!["a".into()]
    }

    pub fn returns_option(self) -> bool {
        matches!(
            self,
            Self::TryTake
                | Self::TryRead
                | Self::TakeOrCancel
                | Self::ReadOrCancel
                | Self::SwapOrCancel
        )
    }

    pub fn blocking(self) -> bool {
        matches!(
            self,
            Self::Take
                | Self::Put
                | Self::Read
                | Self::Swap
                | Self::TakeOrCancel
                | Self::PutOrCancel
                | Self::ReadOrCancel
                | Self::SwapOrCancel
        )
    }

    pub fn sig(self) -> (Vec<Ty>, Ty) {
        use MVarOp::*;
        let a = Ty::Param("a".into());
        let cell = Ty::Adt("MVarCell".into(), vec![a.clone()]);
        let option = Ty::Adt(crate::known::OPTION.into(), vec![a.clone()]);
        match self {
            New => (vec![a], cell),
            NewEmpty => (vec![Ty::Unit], cell),
            Take | Read => (vec![cell], a),
            Swap => (vec![cell, a.clone()], a),
            Put => (vec![cell, a], Ty::Unit),
            TryTake | TryRead | TakeOrCancel | ReadOrCancel => (vec![cell], option),
            SwapOrCancel => (vec![cell, a], option),
            TryPut | PutOrCancel => (vec![cell, a], Ty::Bool),
            IsEmpty => (vec![cell], Ty::Bool),
        }
    }

    // Put/swap take ownership even on failure or cancellation; the runtime
    // releases the supplied word on those paths.
    pub fn consumes(self, i: usize) -> bool {
        (self == Self::New && i == 0)
            || (i == 1
                && matches!(
                    self,
                    Self::Put | Self::Swap | Self::TryPut | Self::PutOrCancel | Self::SwapOrCancel
                ))
    }
}
