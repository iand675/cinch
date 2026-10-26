//! Core intermediate representation consumed by the code generator.
//!
//! The checker elaborates surface programs into [`Core`]: curried
//! applications with explicit spines, hoisted lambdas, explicit dictionary
//! construction and projection, and primitive operations.
//!
//! # Runtime representation
//!
//! Every value is one 64-bit machine word:
//!
//! - `Int`: the `i64` itself.
//! - `Bool`: 0 or 1.
//! - `Unit`: 0.
//! - ADT values: `ptrtoint` of a heap block `{ i64 tag; i64 field0; ... }`
//!   with `tag` the constructor index in declaration order.
//! - Function values: `ptrtoint` of a closure record (below).
//! - Dictionaries: `ptrtoint` of a record `{ i64 nmeth; i64 meth0; ... }`
//!   where each `methN` is a function value.
//!
//! # Closure and application ABI
//!
//! A closure record is `{ i64 code; i64 arity; i64 nfilled; i64 saved[0..] }`.
//! `code` points to the function's apply stub with signature
//! `i64 (ptr %self, i64 %argc, ptr %argv)`. The stub merges `saved` with
//! `argv`; when the merged argument count reaches `arity` it calls the
//! underlying function, when it is lower it allocates a bigger closure, and
//! when it is higher it calls the underlying function and applies the rest
//! of the arguments to the result via `__apply`. `__apply` itself is an
//! indirect call of `code`, so every call site can pass an argument vector
//! of any length without allocating intermediate closures.

use crate::types::{Scalar, Ty};
use crate::{Error, Result};
use std::collections::HashMap;
use std::fmt;

/// A resolved code generation unit name, e.g. `"Prelude.natAdd"`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalId(pub String);

impl fmt::Display for GlobalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A local binder identity, unique within a function body.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalId(pub usize);

// ------------------------------------------------------------ primitive operations
//
// Primitive operations come in two shapes: the bootstrap `Int`/`Bool`
// intrinsics, and the machine scalar families of the primitive types
// specification (sections 4, 5, 6 and 7). Names follow decision D6: an
// operation is a prelude-level name of the form `<op><Type>`
// (`wrappingAddU32`) or `<src>To<dst>` for conversions, bound directly from
// `foreign import ccall "safelang.prim.*"`.

/// Integer arithmetic operations (primitive types specification, section 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArithOp {
    Add,
    Sub,
    Mul,
}

impl ArithOp {
    fn name(self) -> &'static str {
        match self {
            ArithOp::Add => "Add",
            ArithOp::Sub => "Sub",
            ArithOp::Mul => "Mul",
        }
    }
}

/// The overflow families of integer arithmetic (section 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArithMode {
    Checked,
    Wrapping,
    Saturating,
}

impl ArithMode {
    fn name(self) -> &'static str {
        match self {
            ArithMode::Checked => "checked",
            ArithMode::Wrapping => "wrapping",
            ArithMode::Saturating => "saturating",
        }
    }
}

/// Shift operations (section 4): `checked*` return `None` on counts at or
/// above the width, `wrapping*` reduce the count modulo the width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ShiftOp {
    Shl,
    Shr,
}

impl ShiftOp {
    fn name(self) -> &'static str {
        match self {
            ShiftOp::Shl => "Shl",
            ShiftOp::Shr => "Shr",
        }
    }
}

/// Bitwise binary operations (section 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BitOp {
    And,
    Or,
    Xor,
}

impl BitOp {
    fn name(self) -> &'static str {
        match self {
            BitOp::And => "bitAnd",
            BitOp::Or => "bitOr",
            BitOp::Xor => "bitXor",
        }
    }
}

/// Bit inspection operations (section 4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BitScanOp {
    CountOnes,
    LeadingZeros,
    TrailingZeros,
    ReverseBits,
    SwapBytes,
}

impl BitScanOp {
    fn name(self) -> &'static str {
        match self {
            BitScanOp::CountOnes => "countOnes",
            BitScanOp::LeadingZeros => "leadingZeros",
            BitScanOp::TrailingZeros => "trailingZeros",
            BitScanOp::ReverseBits => "reverseBits",
            BitScanOp::SwapBytes => "swapBytes",
        }
    }

    /// `countOnes`, `leadingZeros` and `trailingZeros` return `U32` counts;
    /// `reverseBits` and `swapBytes` map the type to itself (`swapBytes` is
    /// the identity on one-byte types).
    fn returns_self(self) -> bool {
        matches!(self, BitScanOp::ReverseBits | BitScanOp::SwapBytes)
    }
}

/// Comparisons: integer and `Char` ordering (sections 6 and 8) and the
/// quiet IEEE comparisons (section 5, spelled `feq` and friends).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    fn name(self) -> &'static str {
        match self {
            CmpOp::Eq => "eq",
            CmpOp::Ne => "ne",
            CmpOp::Lt => "lt",
            CmpOp::Le => "le",
            CmpOp::Gt => "gt",
            CmpOp::Ge => "ge",
        }
    }

    fn parse(s: &str) -> Option<CmpOp> {
        match s {
            "eq" => Some(CmpOp::Eq),
            "ne" => Some(CmpOp::Ne),
            "lt" => Some(CmpOp::Lt),
            "le" => Some(CmpOp::Le),
            "gt" => Some(CmpOp::Gt),
            "ge" => Some(CmpOp::Ge),
            _ => None,
        }
    }
}

/// Binary floating-point operations (section 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloatBinOp {
    Add,
    Sub,
    Mul,
    Div,
    /// IEEE 754-2019 `minimum`: propagates NaN, `-0 < +0`.
    Minimum,
    /// IEEE 754-2019 `maximum`: propagates NaN, `-0 < +0`.
    Maximum,
}

impl FloatBinOp {
    fn name(self) -> &'static str {
        match self {
            FloatBinOp::Add => "add",
            FloatBinOp::Sub => "sub",
            FloatBinOp::Mul => "mul",
            FloatBinOp::Div => "div",
            FloatBinOp::Minimum => "minimum",
            FloatBinOp::Maximum => "maximum",
        }
    }
}

/// Unary floating-point operations (section 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FloatUnOp {
    Neg,
    Abs,
    Sqrt,
}

impl FloatUnOp {
    fn name(self) -> &'static str {
        match self {
            FloatUnOp::Neg => "neg",
            FloatUnOp::Abs => "abs",
            FloatUnOp::Sqrt => "sqrt",
        }
    }
}

/// Floating-point classification (section 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FClassOp {
    IsNan,
    IsInfinite,
    IsFinite,
    IsNormal,
    SignBit,
}

impl FClassOp {
    fn name(self) -> &'static str {
        match self {
            FClassOp::IsNan => "isNaN",
            FClassOp::IsInfinite => "isInfinite",
            FClassOp::IsFinite => "isFinite",
            FClassOp::IsNormal => "isNormal",
            FClassOp::SignBit => "signBit",
        }
    }
}

/// Conversion families (section 7). `Total` is `sToT` (value-preserving
/// integer conversions, integer-to-float and float-to-float with IEEE
/// rounding), `Checked` returns `Option T`, `Saturating` clamps (`NaN` maps
/// to zero) and `Wrapping` truncates two's complement (integer-to-integer
/// only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConvMode {
    Total,
    Checked,
    Saturating,
    Wrapping,
}

impl ConvMode {
    fn name(self) -> &'static str {
        match self {
            ConvMode::Total => "",
            ConvMode::Checked => "checked",
            ConvMode::Saturating => "saturating",
            ConvMode::Wrapping => "wrapping",
        }
    }
}

/// Mathematical `Nat` operations (`safelang.runtime.nat*`). Words are
/// boxed bignums (the runtime blocks); `Div`/`Rem` fault on a zero divisor
/// and `Sub` faults when the result would be negative (the inductive `Nat`
/// subtraction is truncated to exactness: it faults, it does not clamp).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NatOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    /// `natCmp a b` is the raw immediate -1, 0 or 1 (`Nat -> Nat -> I64`).
    Cmp,
    /// `natFromI64`: box a machine integer, faulting on a negative input.
    FromI64,
    /// `natToI64`: value as a machine integer, faulting out of `i64` range.
    ToI64,
}

impl NatOp {
    fn name(self) -> &'static str {
        match self {
            NatOp::Add => "Add",
            NatOp::Sub => "Sub",
            NatOp::Mul => "Mul",
            NatOp::Div => "Div",
            NatOp::Rem => "Rem",
            NatOp::Cmp => "Cmp",
            NatOp::FromI64 => "FromI64",
            NatOp::ToI64 => "ToI64",
        }
    }

    /// The runtime extern this operation is emitted against.
    pub fn extern_name(self) -> &'static str {
        match self {
            NatOp::Add => "sl_nat_add",
            NatOp::Sub => "sl_nat_sub",
            NatOp::Mul => "sl_nat_mul",
            NatOp::Div => "sl_nat_div",
            NatOp::Rem => "sl_nat_rem",
            NatOp::Cmp => "sl_nat_cmp",
            NatOp::FromI64 => "sl_nat_from_i64",
            NatOp::ToI64 => "sl_nat_to_i64",
        }
    }
}

/// Mathematical `Int` operations (`safelang.runtime.int*`). Words are
/// boxed bignums (sign-magnitude); `Div`/`Rem` truncate toward zero and
/// fault on a zero divisor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Neg,
    /// `intCmp a b` is the raw immediate -1, 0 or 1 (`Int -> Int -> I64`).
    Cmp,
    /// `intFromI64`: box a machine integer.
    FromI64,
    /// `intToI64`: value as a machine integer, faulting out of `i64` range.
    ToI64,
}

impl IntOp {
    fn name(self) -> &'static str {
        match self {
            IntOp::Add => "Add",
            IntOp::Sub => "Sub",
            IntOp::Mul => "Mul",
            IntOp::Div => "Div",
            IntOp::Rem => "Rem",
            IntOp::Neg => "Neg",
            IntOp::Cmp => "Cmp",
            IntOp::FromI64 => "FromI64",
            IntOp::ToI64 => "ToI64",
        }
    }

    /// The runtime extern this operation is emitted against.
    pub fn extern_name(self) -> &'static str {
        match self {
            IntOp::Add => "sl_int_add",
            IntOp::Sub => "sl_int_sub",
            IntOp::Mul => "sl_int_mul",
            IntOp::Div => "sl_int_div",
            IntOp::Rem => "sl_int_rem",
            IntOp::Neg => "sl_int_neg",
            IntOp::Cmp => "sl_int_cmp",
            IntOp::FromI64 => "sl_int_from_i64",
            IntOp::ToI64 => "sl_int_to_i64",
        }
    }
}

/// `Bytes` operations (`safelang.runtime.bytes*`). The runtime externs
/// take and return machine `i64` counts, indices and byte values; the
/// intrinsic units convert through `sl_nat_to_i64`/`sl_nat_from_i64` (a
/// `Nat` outside `i64` range faults, exit 71), and out-of-bounds indices
/// and values outside `0..=255` fault (exit 71).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BytesOp {
    /// `bytesNew :: Nat -> Bytes`: a zeroed buffer of the given length.
    New,
    /// `bytesLen :: Bytes -> Nat`.
    Len,
    /// `bytesGet :: Bytes -> Nat -> Nat`: the byte value at an index.
    Get,
    /// `bytesSet :: Bytes -> Nat -> Nat -> Unit`.
    Set,
}

impl BytesOp {
    fn name(self) -> &'static str {
        match self {
            BytesOp::New => "New",
            BytesOp::Len => "Len",
            BytesOp::Get => "Get",
            BytesOp::Set => "Set",
        }
    }

    /// The runtime extern this operation is emitted against.
    pub fn extern_name(self) -> &'static str {
        match self {
            BytesOp::New => "sl_bytes_new",
            BytesOp::Len => "sl_bytes_len",
            BytesOp::Get => "sl_bytes_get",
            BytesOp::Set => "sl_bytes_set",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrimOp {
    /// Mathematical `Nat` operations on boxed bignum words
    /// (`safelang.runtime.nat*`; runtime ABI, `sl_nat_*`).
    Nat(NatOp),
    /// Mathematical `Int` operations on boxed bignum words
    /// (`safelang.runtime.int*`; runtime ABI, `sl_int_*`).
    Int(IntOp),
    /// `Bytes` operations on boxed byte buffers
    /// (`safelang.runtime.bytes*`; runtime ABI, `sl_bytes_*`).
    Bytes(BytesOp),
    EqBool,
    NeBool,
    NotBool,
    AndBool,
    OrBool,
    /// `delay f` = `f` itself (a thunk is a closure over Unit).
    Delay,
    /// `force t`: apply the thunk closure to Unit.
    Force,
    /// `demand d`: run the memo cell state machine.
    Demand,
    /// `minBound`/`maxBound` (section 4), nullary values.
    Bound { ty: Scalar, max: bool },
    /// `checkedAdd`/`wrappingAdd`/`saturatingAdd`, `Sub`, `Mul` (section 4).
    Arith { ty: Scalar, op: ArithOp, mode: ArithMode },
    /// `checkedNeg`/`wrappingNeg`/`saturatingNeg` (section 4).
    Neg { ty: Scalar, mode: ArithMode },
    /// `checkedDiv`/`checkedRem`: `None` on a zero divisor and on
    /// `minBound / -1` for signed types (section 4).
    CheckedDiv { ty: Scalar },
    CheckedRem { ty: Scalar },
    /// `checkedShl`/`checkedShr`/`wrappingShl`/`wrappingShr` (section 4).
    Shift { ty: Scalar, op: ShiftOp, checked: bool },
    /// `rotateLeft`/`rotateRight`: total, count modulo the width (section 4).
    Rotate { ty: Scalar, right: bool },
    /// `bitAnd`/`bitOr`/`bitXor` (section 4).
    Bit { ty: Scalar, op: BitOp },
    /// `bitNot` (section 4).
    BitNot { ty: Scalar },
    /// `countOnes`/`leadingZeros`/`trailingZeros`/`reverseBits`/
    /// `swapBytes` (section 4).
    BitScan { ty: Scalar, op: BitScanOp },
    /// `eq`/`ne`/`lt`/`le`/`gt`/`ge` for integer types, `Usize`, `Isize`
    /// and `Char` (sections 6 and 8).
    Cmp { ty: Scalar, op: CmpOp },
    /// `add`/`sub`/`mul`/`div`/`minimum`/`maximum` (section 5).
    FloatBin { ty: Scalar, op: FloatBinOp },
    /// `neg`/`abs`/`sqrt` (section 5).
    FloatUn { ty: Scalar, op: FloatUnOp },
    /// `feq`/`fne`/`flt`/`fle`/`fgt`/`fge`: quiet IEEE comparisons (section 5).
    FCmp { ty: Scalar, op: CmpOp },
    /// `isNaN`/`isInfinite`/`isFinite`/`isNormal`/`signBit` (section 5).
    FClass { ty: Scalar, op: FClassOp },
    /// `totalOrderF32`/`totalOrderF64` (section 5).
    TotalOrder { ty: Scalar },
    /// `charToU32 :: Char -> U32` (section 6), total.
    CharToU32,
    /// `u32ToChar :: U32 -> Option Char` (sections 6 and 7): rejects
    /// surrogates and values above `0x10FFFF`.
    U32ToChar,
    /// `u8ToI32`, `checkedU64ToI32`, `saturatingF64ToI32`, `wrappingU32ToU8`
    /// and friends (section 7).
    Conv { from: Scalar, to: Scalar, mode: ConvMode },
}

impl PrimOp {
    /// The intrinsic symbol names recognised on `foreign import ccall`.
    pub fn from_symbol(sym: &str) -> Option<PrimOp> {
        use PrimOp::*;
        if let Some(name) = sym.strip_prefix("safelang.runtime.") {
            return Some(match name {
                "natAdd" => Nat(NatOp::Add),
                "natSub" => Nat(NatOp::Sub),
                "natMul" => Nat(NatOp::Mul),
                "natDiv" => Nat(NatOp::Div),
                "natRem" => Nat(NatOp::Rem),
                "natCmp" => Nat(NatOp::Cmp),
                "natFromI64" => Nat(NatOp::FromI64),
                "natToI64" => Nat(NatOp::ToI64),
                "intAdd" => Int(IntOp::Add),
                "intSub" => Int(IntOp::Sub),
                "intMul" => Int(IntOp::Mul),
                "intDiv" => Int(IntOp::Div),
                "intRem" => Int(IntOp::Rem),
                "intNeg" => Int(IntOp::Neg),
                "intCmp" => Int(IntOp::Cmp),
                "intFromI64" => Int(IntOp::FromI64),
                "intToI64" => Int(IntOp::ToI64),
                "bytesNew" => Bytes(BytesOp::New),
                "bytesLen" => Bytes(BytesOp::Len),
                "bytesGet" => Bytes(BytesOp::Get),
                "bytesSet" => Bytes(BytesOp::Set),
                _ => return None,
            });
        }
        let name = sym.strip_prefix("safelang.prim.")?;
        Some(match name {
            "eqBool" => EqBool,
            "neBool" => NeBool,
            "notBool" => NotBool,
            "andBool" => AndBool,
            "orBool" => OrBool,
            "delay" => Delay,
            "force" => Force,
            "demand" => Demand,
            "charToU32" => CharToU32,
            "u32ToChar" => U32ToChar,
            _ => {
                if let Some(op) = parse_conv(name) {
                    op
                } else {
                    parse_typed_op(name)?
                }
            }
        })
    }

    /// The unit name this operation is emitted under (and the
    /// `safelang.prim.*`/`safelang.runtime.*` foreign symbol it is bound
    /// by).
    pub fn symbol(self) -> String {
        use PrimOp::*;
        let n = match self {
            Nat(op) => return format!("safelang.runtime.nat{}", op.name()),
            Int(op) => return format!("safelang.runtime.int{}", op.name()),
            Bytes(op) => return format!("safelang.runtime.bytes{}", op.name()),
            EqBool => return "safelang.prim.eqBool".to_string(),
            NeBool => return "safelang.prim.neBool".to_string(),
            NotBool => return "safelang.prim.notBool".to_string(),
            AndBool => return "safelang.prim.andBool".to_string(),
            OrBool => return "safelang.prim.orBool".to_string(),
            Delay => return "safelang.prim.delay".to_string(),
            Force => return "safelang.prim.force".to_string(),
            Demand => return "safelang.prim.demand".to_string(),
            CharToU32 => return "safelang.prim.charToU32".to_string(),
            U32ToChar => return "safelang.prim.u32ToChar".to_string(),
            Bound { ty, max } => {
                format!("{}Bound{}", if max { "max" } else { "min" }, ty.name())
            }
            Arith { ty, op, mode } => {
                format!("{}{}{}", mode.name(), op.name(), ty.name())
            }
            Neg { ty, mode } => format!("{}Neg{}", mode.name(), ty.name()),
            CheckedDiv { ty } => format!("checkedDiv{}", ty.name()),
            CheckedRem { ty } => format!("checkedRem{}", ty.name()),
            Shift { ty, op, checked } => format!(
                "{}{}{}",
                if checked { "checked" } else { "wrapping" },
                op.name(),
                ty.name()
            ),
            Rotate { ty, right } => {
                format!("rotate{}{}", if right { "Right" } else { "Left" }, ty.name())
            }
            Bit { ty, op } => format!("{}{}", op.name(), ty.name()),
            BitNot { ty } => format!("bitNot{}", ty.name()),
            BitScan { ty, op } => format!("{}{}", op.name(), ty.name()),
            Cmp { ty, op } => format!("{}{}", op.name(), ty.name()),
            FloatBin { ty, op } => format!("{}{}", op.name(), ty.name()),
            FloatUn { ty, op } => format!("{}{}", op.name(), ty.name()),
            FCmp { ty, op } => format!("f{}{}", op.name(), ty.name()),
            FClass { ty, op } => format!("{}{}", op.name(), ty.name()),
            TotalOrder { ty } => format!("totalOrder{}", ty.name()),
            Conv { from, to, mode } => conv_name(from, to, mode),
        };
        format!("safelang.prim.{n}")
    }

    pub fn arity(self) -> usize {
        use PrimOp::*;
        match self {
            Bound { .. } => 0,
            Nat(NatOp::FromI64 | NatOp::ToI64)
            | Int(IntOp::FromI64 | IntOp::ToI64 | IntOp::Neg)
            | Bytes(BytesOp::New | BytesOp::Len) => 1,
            Bytes(BytesOp::Set) => 3,
            Neg { .. } | BitNot { .. } | BitScan { .. } | FloatUn { .. } | FClass { .. }
            | CharToU32 | U32ToChar | Conv { .. } | Delay | Force | Demand | NotBool => 1,
            _ => 2,
        }
    }

    /// Does the operation return `Option t` (so its unit needs a `None`
    /// block and boxes `Some`)?
    pub fn returns_option(self) -> bool {
        use PrimOp::*;
        match self {
            CheckedDiv { .. } | CheckedRem { .. } | U32ToChar => true,
            Arith { mode: ArithMode::Checked, .. } => true,
            Neg { mode: ArithMode::Checked, .. } => true,
            Shift { checked: true, .. } => true,
            Conv { mode: ConvMode::Checked, .. } => true,
            _ => false,
        }
    }

    /// Does the operation return the prelude `Ordering`?
    pub fn returns_ordering(self) -> bool {
        matches!(self, PrimOp::TotalOrder { .. })
    }

    /// The argument types and result type of the primitive (the prelude
    /// `foreign import` declaration must match this exactly).
    pub fn sig(self) -> (Vec<Ty>, Ty) {
        use PrimOp::*;
        let prim = |s: Scalar| Ty::Prim(s);
        let opt = |t: Ty| Ty::Adt("Option".to_string(), vec![t]);
        let u32t = prim(Scalar::U32);
        let i64t = prim(Scalar::I64);
        match self {
            Nat(NatOp::Add | NatOp::Sub | NatOp::Mul | NatOp::Div | NatOp::Rem) => {
                (vec![Ty::Nat, Ty::Nat], Ty::Nat)
            }
            // The comparison prims return the raw -1/0/1 immediate;
            // `eqI64 (natCmp a b) 0` is the equality test.
            Nat(NatOp::Cmp) => (vec![Ty::Nat, Ty::Nat], i64t),
            Nat(NatOp::FromI64) => (vec![i64t], Ty::Nat),
            Nat(NatOp::ToI64) => (vec![Ty::Nat], i64t),
            Int(IntOp::Add | IntOp::Sub | IntOp::Mul | IntOp::Div | IntOp::Rem) => {
                (vec![Ty::Int, Ty::Int], Ty::Int)
            }
            Int(IntOp::Neg) => (vec![Ty::Int], Ty::Int),
            Int(IntOp::Cmp) => (vec![Ty::Int, Ty::Int], i64t),
            Int(IntOp::FromI64) => (vec![i64t], Ty::Int),
            Int(IntOp::ToI64) => (vec![Ty::Int], i64t),
            Bytes(BytesOp::New) => (vec![Ty::Nat], Ty::Bytes),
            Bytes(BytesOp::Len) => (vec![Ty::Bytes], Ty::Nat),
            Bytes(BytesOp::Get) => (vec![Ty::Bytes, Ty::Nat], Ty::Nat),
            Bytes(BytesOp::Set) => (vec![Ty::Bytes, Ty::Nat, Ty::Nat], Ty::Unit),
            EqBool | NeBool | AndBool | OrBool => (vec![Ty::Bool, Ty::Bool], Ty::Bool),
            NotBool => (vec![Ty::Bool], Ty::Bool),
            Delay | Force | Demand => {
                unreachable!("deferral forms are builtin, not foreign imports")
            }
            Bound { ty, .. } => (vec![], prim(ty)),
            Arith { ty, mode, .. } => {
                let res = if mode == ArithMode::Checked { opt(prim(ty)) } else { prim(ty) };
                (vec![prim(ty), prim(ty)], res)
            }
            Neg { ty, mode } => {
                let res = if mode == ArithMode::Checked { opt(prim(ty)) } else { prim(ty) };
                (vec![prim(ty)], res)
            }
            CheckedDiv { ty } | CheckedRem { ty } => {
                (vec![prim(ty), prim(ty)], opt(prim(ty)))
            }
            Shift { ty, checked, .. } => {
                let res = if checked { opt(prim(ty)) } else { prim(ty) };
                (vec![prim(ty), u32t], res)
            }
            Rotate { ty, .. } => (vec![prim(ty), u32t], prim(ty)),
            Bit { ty, .. } => (vec![prim(ty), prim(ty)], prim(ty)),
            BitNot { ty } => (vec![prim(ty)], prim(ty)),
            BitScan { ty, op } => {
                let res = if op.returns_self() { prim(ty) } else { u32t };
                (vec![prim(ty)], res)
            }
            Cmp { ty, .. } => (vec![prim(ty), prim(ty)], Ty::Bool),
            FloatBin { ty, .. } => (vec![prim(ty), prim(ty)], prim(ty)),
            FloatUn { ty, .. } => (vec![prim(ty)], prim(ty)),
            FCmp { ty, .. } => (vec![prim(ty), prim(ty)], Ty::Bool),
            FClass { ty, .. } => (vec![prim(ty)], Ty::Bool),
            TotalOrder { ty } => (
                vec![prim(ty), prim(ty)],
                Ty::Adt("Ordering".to_string(), vec![]),
            ),
            CharToU32 => (vec![prim(Scalar::Char)], u32t),
            U32ToChar => (vec![u32t], opt(prim(Scalar::Char))),
            Conv { from, to, mode } => {
                let res = if mode == ConvMode::Checked { opt(prim(to)) } else { prim(to) };
                (vec![prim(from)], res)
            }
        }
    }
}

/// The conversion name for one ordered pair: `u8ToI32` in the total family,
/// `checkedU8ToI32`, `saturatingU8ToI32` and `wrappingU8ToI32` otherwise.
fn conv_name(from: Scalar, to: Scalar, mode: ConvMode) -> String {
    if mode == ConvMode::Total {
        format!("{}To{}", from.lower_name(), to.name())
    } else {
        format!("{}{}To{}", mode.name(), from.name(), to.name())
    }
}

/// Is this conversion family defined for the pair (section 7)?
fn conv_valid(from: Scalar, to: Scalar, mode: ConvMode) -> bool {
    if from == Scalar::Char || to == Scalar::Char {
        // `charToU32` and `u32ToChar` are their own names.
        return false;
    }
    match mode {
        ConvMode::Total => {
            if from.is_float() || to.is_float() {
                // Integer-to-float and float-to-float are total with IEEE
                // rounding; float-to-integer is never total.
                !(from.is_float() && to.is_int_like())
            } else {
                from.fits(to)
            }
        }
        ConvMode::Checked | ConvMode::Saturating => {
            // Float-to-int and int-to-int only; int-to-float is total.
            (from.is_int_like() && to.is_int_like()) || (from.is_float() && to.is_int_like())
        }
        ConvMode::Wrapping => from.is_int_like() && to.is_int_like(),
    }
}

/// `u8ToI32`, `checkedU64ToI32`, ... (checked and saturating spell the
/// source capitalized, the total family lowercase).
fn parse_conv(name: &str) -> Option<PrimOp> {
    let (mode, rest) = match name.strip_prefix("checked") {
        Some(r) => (ConvMode::Checked, r),
        None => match name.strip_prefix("saturating") {
            Some(r) => (ConvMode::Saturating, r),
            None => match name.strip_prefix("wrapping") {
                Some(r) => (ConvMode::Wrapping, r),
                None => (ConvMode::Total, name),
            },
        },
    };
    // Split at each `To`; both sides must name types. The source is
    // capitalized in the prefixed families and lowercase in the total one.
    for (i, _) in rest.match_indices("To") {
        let src = &rest[..i];
        let dst = &rest[i + 2..];
        let from = if mode == ConvMode::Total {
            Scalar::ALL.iter().copied().find(|s| s.lower_name() == src)?
        } else {
            Scalar::parse(src)?
        };
        let to = Scalar::parse(dst)?;
        if !conv_valid(from, to, mode) {
            return None;
        }
        return Some(PrimOp::Conv { from, to, mode });
    }
    None
}

/// `<op><Type>` names: `wrappingAddU32`, `checkedShlI64`, `feqF64`, ...
fn parse_typed_op(name: &str) -> Option<PrimOp> {
    // Longest type suffix wins so `checkedShlU16` splits `Shl`/`U16`.
    let mut suffixes: Vec<(&str, Scalar)> =
        Scalar::ALL.iter().map(|s| (s.name(), *s)).collect();
    suffixes.sort_by_key(|(n, _)| std::cmp::Reverse(n.len()));
    for (suffix, ty) in suffixes {
        let Some(head) = name.strip_suffix(suffix) else {
            continue;
        };
        if let Some(op) = typed_op_from_head(head, ty) {
            return Some(op);
        }
    }
    None
}

fn typed_op_from_head(head: &str, ty: Scalar) -> Option<PrimOp> {
    use PrimOp::*;
    // Integer arithmetic families (section 4).
    for mode in [ArithMode::Checked, ArithMode::Wrapping, ArithMode::Saturating] {
        for op in [ArithOp::Add, ArithOp::Sub, ArithOp::Mul] {
            if head == format!("{}{}", mode.name(), op.name()) {
                if !ty.is_int_like() {
                    return None;
                }
                return Some(Arith { ty, op, mode });
            }
        }
        if head == format!("{}Neg", mode.name()) {
            if !ty.is_int_like() {
                return None;
            }
            return Some(Neg { ty, mode });
        }
    }
    if head == "checkedDiv" && ty.is_int_like() {
        return Some(CheckedDiv { ty });
    }
    if head == "checkedRem" && ty.is_int_like() {
        return Some(CheckedRem { ty });
    }
    for (h, op, checked) in [
        ("checkedShl", ShiftOp::Shl, true),
        ("checkedShr", ShiftOp::Shr, true),
        ("wrappingShl", ShiftOp::Shl, false),
        ("wrappingShr", ShiftOp::Shr, false),
    ] {
        if head == h && ty.is_int_like() {
            return Some(Shift { ty, op, checked });
        }
    }
    if head == "rotateLeft" && ty.is_int_like() {
        return Some(Rotate { ty, right: false });
    }
    if head == "rotateRight" && ty.is_int_like() {
        return Some(Rotate { ty, right: true });
    }
    for (h, op) in [("bitAnd", BitOp::And), ("bitOr", BitOp::Or), ("bitXor", BitOp::Xor)] {
        if head == h && ty.is_int_like() {
            return Some(Bit { ty, op });
        }
    }
    if head == "bitNot" && ty.is_int_like() {
        return Some(BitNot { ty });
    }
    for (h, op) in [
        ("countOnes", BitScanOp::CountOnes),
        ("leadingZeros", BitScanOp::LeadingZeros),
        ("trailingZeros", BitScanOp::TrailingZeros),
        ("reverseBits", BitScanOp::ReverseBits),
        ("swapBytes", BitScanOp::SwapBytes),
    ] {
        if head == h && ty.is_int_like() {
            return Some(BitScan { ty, op });
        }
    }
    if head == "minBound" && ty.is_int_like() {
        return Some(Bound { ty, max: false });
    }
    if head == "maxBound" && ty.is_int_like() {
        return Some(Bound { ty, max: true });
    }
    // Integer and Char comparisons (sections 6 and 8).
    if let Some(op) = CmpOp::parse(head) {
        if ty.is_int_like() || ty == Scalar::Char {
            return Some(Cmp { ty, op });
        }
    }
    // Floating point (section 5).
    if ty.is_float() {
        for (h, op) in [
            ("add", FloatBinOp::Add),
            ("sub", FloatBinOp::Sub),
            ("mul", FloatBinOp::Mul),
            ("div", FloatBinOp::Div),
            ("minimum", FloatBinOp::Minimum),
            ("maximum", FloatBinOp::Maximum),
        ] {
            if head == h {
                return Some(FloatBin { ty, op });
            }
        }
        for (h, op) in [
            ("neg", FloatUnOp::Neg),
            ("abs", FloatUnOp::Abs),
            ("sqrt", FloatUnOp::Sqrt),
        ] {
            if head == h {
                return Some(FloatUn { ty, op });
            }
        }
        for (h, op) in [
            ("feq", CmpOp::Eq),
            ("fne", CmpOp::Ne),
            ("flt", CmpOp::Lt),
            ("fle", CmpOp::Le),
            ("fgt", CmpOp::Gt),
            ("fge", CmpOp::Ge),
        ] {
            if head == h {
                return Some(FCmp { ty, op });
            }
        }
        for (h, op) in [
            ("isNaN", FClassOp::IsNan),
            ("isInfinite", FClassOp::IsInfinite),
            ("isFinite", FClassOp::IsFinite),
            ("isNormal", FClassOp::IsNormal),
            ("signBit", FClassOp::SignBit),
        ] {
            if head == h {
                return Some(FClass { ty, op });
            }
        }
        if head == "totalOrder" {
            return Some(TotalOrder { ty });
        }
    }
    None
}

#[derive(Clone, Debug)]
pub enum CPat {
    /// Bind the scrutinee word.
    Var(LocalId),
    Wild,
    /// Zero-arity constructor; `tag` selects the branch.
    Con { tag: usize, fields: Vec<LocalId> },
}

/// One branch of a two-way decision point (`if`, a guard test, a
/// compiled comparison) with its optional weight beside it (attribute
/// annotations specification section 2.1: a weight is a property of the
/// alternative itself, never a parallel list on the decision node).
#[derive(Clone, Debug)]
pub struct CBranch<B> {
    pub weight: Option<u32>,
    pub body: Box<B>,
}

/// One alternative of a `case` decision point, weight beside it (same
/// rule as [`CBranch`]).
#[derive(Clone, Debug)]
pub struct CAlt<B> {
    pub pat: CPat,
    pub weight: Option<u32>,
    pub body: Box<B>,
}

impl<B> CBranch<B> {
    /// An unweighted branch.
    pub fn new(body: B) -> Self {
        CBranch { weight: None, body: Box::new(body) }
    }

    /// A weighted branch (weight is `Some` exactly when the decision
    /// point is fully annotated).
    pub fn weighted(weight: Option<u32>, body: B) -> Self {
        CBranch { weight, body: Box::new(body) }
    }
}

impl<B> CAlt<B> {
    /// An unweighted alternative.
    pub fn new(pat: CPat, body: B) -> Self {
        CAlt { pat, weight: None, body: Box::new(body) }
    }

    /// A weighted alternative (weight is `Some` exactly when the
    /// decision point is fully annotated).
    pub fn weighted(pat: CPat, weight: Option<u32>, body: B) -> Self {
        CAlt { pat, weight, body: Box::new(body) }
    }
}

#[derive(Clone, Debug)]
pub enum Core {
    /// Raw `i64` word constant (machine scalar values and words).
    Int(i64),
    /// Mathematical `Nat`/`Int` constant, carried as a decimal digit
    /// string with an optional leading `-` (`nat` selects the `Nat`
    /// runtime). Code generation materialises it through
    /// `sl_nat_from_decimal`/`sl_int_from_decimal` over a private constant
    /// string, with an `sl_nat_from_i64`/`sl_int_from_i64` fast path when
    /// the value fits an `i64`.
    ConstBig { nat: bool, digits: String },
    Bool(bool),
    Unit,
    /// Local binder (value or dictionary binder).
    Var(LocalId),
    /// Top-level function used as a value (behaves as its closure).
    Global(GlobalId),
    /// Constructor function used as a value.
    Ctor(GlobalId),
    /// Primitive function used as a value.
    Prim(PrimOp),
    /// A real foreign import used as a value: the synthesized wrapper
    /// unit's name (foreign function interface specification section 2).
    /// Like `Prim`, it is opaque to the optimizer.
    Foreign(String),
    /// Method selection from a dictionary value.
    DictProj(Box<Core>, usize),
    /// Record field projection: load the field from the record block
    /// (uniform-word slot, or the recorded offset for boundary layouts).
    Proj(Box<Core>, ProjField),
    /// Instance dictionary construction: call the instance's `mk` unit.
    InstDict(GlobalId, Vec<Core>),
    /// Curried application spine with at least one argument.
    App(Box<Core>, Vec<Core>),
    /// Hoisted lambda: create a closure over `env` for function `fn_id`,
    /// whose parameters are `env` followed by the lambda's own parameter(s).
    Lam { fn_id: GlobalId, env: Vec<Core> },
    Let(LocalId, Box<Core>, Box<Core>),
    /// A two-way decision point: the condition and its two weighted
    /// branches (attribute annotations specification section 2.1).
    If(Box<Core>, CBranch<Core>, CBranch<Core>),
    /// Case on a boxed ADT value; the alternatives carry their weights
    /// beside them (attribute annotations specification section 2.1).
    /// `default` is the fall-through for the single wildcard or variable
    /// alternative, if any.
    Case {
        scrut: Box<Core>,
        alts: Vec<CAlt<Core>>,
    },
    /// Demand a memo cell: run its state machine (lazy allocation is
    /// `LazyAlloc` + `LazyInit`; see the laziness and reclamation
    /// specification).
    Demand(Box<Core>),
    /// Allocate an empty memo cell: `{ state; thunk; value }`.
    LazyAlloc,
    /// Store a thunk closure into a memo cell.
    LazyInit(LocalId, Box<Core>),
    /// The runtime fault path: exit 70 through `__safelang_blackhole`,
    /// reached when a match falls through with pattern coverage unprovable.
    Fault,
    /// A typed quotation template under construction (staging): the
    /// template carrying `SpliceHole` placeholders plus the elaborated
    /// splice expressions, evaluated when the quote is constructed
    /// (modules specification section 7).
    Quote {
        template: Box<crate::ast::Term>,
        holes: Vec<Core>,
    },
    /// A `quoteDecls` template under construction (modules specification
    /// section 8: generated declarations are checked under ordinary rules).
    DeclsQuote {
        template: Vec<crate::ast::Decl>,
        holes: Vec<Core>,
    },
    /// A macro argument's raw token tree: a `Syntax` value.
    SyntaxLit(Vec<crate::lexer::Token>),
    /// A compile-time string constant (`String` values).
    StrLit(String),
}

/// The builtin compile-time capability API (modules specification section
/// 10): values of these operations live only at compile time, and the
/// interpreter performs their I/O under explicit capability arguments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageOp {
    /// `Stage.snapshot name k`: supply the root capability for a named
    /// input snapshot and record the reads performed under it.
    Snapshot,
    /// `Stage.withReadFiles io k`: refine `io` to a file-read capability.
    WithReadFiles,
    /// `Stage.withWriteFiles io k`: refine `io` to a file-write capability.
    WithWriteFiles,
    /// `Stage.withProcess io k`: refine `io` to a subprocess capability.
    WithProcess,
    /// `Stage.readFile cap path`.
    ReadFile,
    /// `Stage.writeFile cap path contents`.
    WriteFile,
    /// `Stage.run cap tool args`: run a subprocess, capturing stdout.
    Run,
    /// `Stage.secret name`: a credential reference; its value never enters
    /// generated source, logs, or snapshots.
    Secret,
    /// `Stage.withDatabase io cfg op k`: run the schema introspection
    /// tool over the subprocess capability (modules specification
    /// section 10's delegation, decision D45) and hand the raw
    /// introspection result to the continuation (section 11).
    WithDatabase,
    /// `ReadSchema`: the schema-read operation token.
    ReadSchema,
    /// `introspectConsistently db`: interpret and consistency-check a raw
    /// introspection result into a schema snapshot (section 11).
    Introspect,
    /// `CodecGenerator.derive snapshot`: generate codec declarations
    /// from a schema snapshot (section 11).
    CodecDerive,
    /// `Stage.emit decls`: append declarations to the generator's output.
    Emit,
    /// `Stage.quoteNat n`: approved constant transport (modules
    /// specification section 9): a `Nat` becomes generated code.
    QuoteNat,
    /// `Stage.quoteInt n`: approved constant transport.
    QuoteInt,
    /// `Stage.quoteBool b`: approved constant transport.
    QuoteBool,
    /// `Stage.strNat s`: parse decimal digits into a `Nat`.
    StrNat,
    /// `Stage.natStr n`: render a `Nat` as decimal digits.
    NatStr,
    /// `Stage.strAppend a b`.
    StrAppend,
    /// `Stage.strLen s`.
    StrLen,
}

impl StageOp {
    pub fn from_name(name: &str) -> Option<StageOp> {
        use StageOp::*;
        Some(match name {
            "snapshot" => Snapshot,
            "withReadFiles" => WithReadFiles,
            "withWriteFiles" => WithWriteFiles,
            "withProcess" => WithProcess,
            "readFile" => ReadFile,
            "writeFile" => WriteFile,
            "run" => Run,
            "secret" => Secret,
            "withDatabase" => WithDatabase,
            "readSchema" => ReadSchema,
            "introspectConsistently" => Introspect,
            "codecDerive" => CodecDerive,
            "emit" => Emit,
            "quoteNat" => QuoteNat,
            "quoteInt" => QuoteInt,
            "quoteBool" => QuoteBool,
            "strNat" => StrNat,
            "natStr" => NatStr,
            "strAppend" => StrAppend,
            "strLen" => StrLen,
            _ => return None,
        })
    }

    pub fn global_id(self) -> GlobalId {
        use StageOp::*;
        let n = match self {
            Snapshot => "snapshot",
            WithReadFiles => "withReadFiles",
            WithWriteFiles => "withWriteFiles",
            WithProcess => "withProcess",
            ReadFile => "readFile",
            WriteFile => "writeFile",
            Run => "run",
            Secret => "secret",
            WithDatabase => "withDatabase",
            ReadSchema => "readSchema",
            Introspect => "introspectConsistently",
            CodecDerive => "codecDerive",
            Emit => "emit",
            QuoteNat => "quoteNat",
            QuoteInt => "quoteInt",
            QuoteBool => "quoteBool",
            StrNat => "strNat",
            NatStr => "natStr",
            StrAppend => "strAppend",
            StrLen => "strLen",
        };
        GlobalId(format!("<builtin>.Stage.{n}"))
    }
}

/// Compile-time capability tokens (modules specification section 10):
/// opaque values threaded through the `Stage` API. Capabilities determine
/// permitted targets; effects record that I/O occurs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    /// The root capability a `Stage.snapshot` callback receives.
    Root,
    /// File access; `write` distinguishes the write refinement (a reader
    /// receives no write authority by default).
    File { write: bool },
    /// Subprocess execution.
    Proc,
}

/// An interpreted, consistency-checked database schema snapshot
/// (modules specification section 11): the recorded schema plus the
/// dialect, collation and snapshot assumptions the binding semantics
/// needs. Generated binding proofs are relative to this capture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaData {
    /// The provider's SQL dialect where the tool reports one.
    pub dialect: String,
    /// The recorded collation assumption.
    pub collation: String,
    /// The provider's consistent-snapshot identity where supported
    /// (empty records the assumption that no such identity exists).
    pub snapshot: String,
    /// The recorded extension assumptions.
    pub extensions: Vec<String>,
    pub tables: Vec<SchemaTable>,
    /// The captured document's content hash (a build identity input).
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaTable {
    pub name: String,
    pub columns: Vec<SchemaColumn>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchemaColumn {
    pub name: String,
    /// `INTEGER`, `REAL`, `BOOLEAN`, or `TEXT`.
    pub ty: String,
    pub nullable: bool,
}

/// A compile-time value (staging evaluation): the value domain of the
/// interpreter that runs `stage` definitions, macros and `generate` blocks
/// during compilation. Compile-time-only types (`Code`, `Syntax`,
/// `Declarations`, `String`, `Secret`, capabilities) never reach target
/// code.
#[derive(Clone, Debug)]
pub enum Value {
    /// Machine scalar and `Char` words.
    Word(i64),
    /// Mathematical `Nat`/`Int`, canonical decimal digits with an optional
    /// leading `-` (same form as [`Core::ConstBig`]).
    Big { nat: bool, digits: String },
    Bool(bool),
    Unit,
    /// Boxed ADT or record: tag plus runtime fields.
    Adt { tag: usize, fields: Vec<Value> },
    /// An instance dictionary: its methods as (possibly partial) closures.
    Dict { methods: Vec<Value> },
    /// Closure over a function unit: captured environment and applied
    /// arguments so far (`arity` is the unit's parameter count).
    Clos {
        fn_id: GlobalId,
        env: Vec<Value>,
        args: Vec<Value>,
        arity: usize,
    },
    /// Constructor function value (apply to its runtime fields).
    CtorFn { tag: usize, arity: usize, args: Vec<Value> },
    /// Primitive function value (apply to its arguments).
    PrimFn { op: PrimOp, args: Vec<Value> },
    /// A typed code value: the hygienically prepared template.
    Code(crate::ast::Term),
    /// A declaration-group value.
    Decls(Vec<crate::ast::Decl>),
    /// A token-tree value; `origin` names the module the tokens were
    /// written in (spliced syntax parses with its fixities).
    Syntax {
        tokens: Vec<crate::lexer::Token>,
        origin: String,
    },
    /// A compile-time string.
    Str(String),
    /// A secret reference; the value never leaves the interpreter. `name`
    /// is the environment variable the credential lives in (it records
    /// into the build identity; the value does not).
    Secret { name: String, value: String },
    /// A raw database introspection result (the schema tool's output).
    Db(String),
    /// A checked schema snapshot (modules specification section 11).
    Schema(SchemaData),
    /// A capability token (opaque to source code).
    Cap(Cap),
    /// A `Stage` builtin partially applied.
    StageFn { op: StageOp, args: Vec<Value> },
}

/// Arbitrary-precision integers for compile-time `Nat`/`Int` arithmetic:
/// sign plus base-1e9 limbs, little-endian (`mag` empty is zero).
pub mod big {
    /// Base-1e9 limbs; one limb renders as exactly nine decimal digits.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Big {
        pub neg: bool,
        pub mag: Vec<u32>,
    }

    const BASE: u64 = 1_000_000_000;

    fn trim(mag: &mut Vec<u32>) {
        while mag.last() == Some(&0) {
            mag.pop();
        }
    }

    impl Big {
        pub fn zero() -> Big {
            Big { neg: false, mag: Vec::new() }
        }

        pub fn is_zero(&self) -> bool {
            self.mag.is_empty()
        }

        pub fn from_i64(v: i64) -> Big {
            if v == 0 {
                return Big::zero();
            }
            let neg = v < 0;
            let mut n = (v as i128).unsigned_abs();
            let mut mag = Vec::new();
            while n > 0 {
                mag.push((n % BASE as u128) as u32);
                n /= BASE as u128;
            }
            Big { neg, mag }
        }

        pub fn to_i64(&self) -> Option<i64> {
            let mut v: u64 = 0;
            for l in self.mag.iter().rev() {
                v = v.checked_mul(BASE)?.checked_add(*l as u64)?;
            }
            if self.neg {
                i64::try_from(v as i128 * -1).ok()
            } else {
                i64::try_from(v).ok()
            }
        }

        /// Canonical decimal digits with a leading `-` for negatives.
        pub fn to_dec(&self) -> String {
            if self.is_zero() {
                return "0".to_string();
            }
            let mut out = String::new();
            if self.neg {
                out.push('-');
            }
            for (i, l) in self.mag.iter().rev().enumerate() {
                if i == 0 {
                    out.push_str(&l.to_string());
                } else {
                    out.push_str(&format!("{l:09}"));
                }
            }
            out
        }

        /// Parse decimal digits with an optional leading `-`; `None` on
        /// malformed input.
        pub fn from_dec(s: &str) -> Option<Big> {
            let (neg, digits) = match s.strip_prefix('-') {
                Some(rest) => (true, rest),
                None => (false, s),
            };
            if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            let mut mag = Vec::new();
            let bytes = digits.as_bytes();
            let mut i = bytes.len();
            while i > 0 {
                let start = i.saturating_sub(9);
                let chunk = std::str::from_utf8(&bytes[start..i]).ok()?;
                mag.push(chunk.parse::<u32>().ok()?);
                i = start;
            }
            trim(&mut mag);
            if mag.is_empty() {
                return Some(Big::zero());
            }
            Some(Big { neg, mag })
        }

        fn cmp_abs(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            if a.len() != b.len() {
                return a.len().cmp(&b.len());
            }
            for (x, y) in a.iter().rev().zip(b.iter().rev()) {
                match x.cmp(y) {
                    Ordering::Equal => {}
                    other => return other,
                }
            }
            Ordering::Equal
        }

        pub fn cmp(&self, other: &Big) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            match (self.is_zero(), other.is_zero()) {
                (true, true) => return Ordering::Equal,
                (true, false) => {
                    return if other.neg { Ordering::Greater } else { Ordering::Less }
                }
                (false, true) => {
                    return if self.neg { Ordering::Less } else { Ordering::Greater }
                }
                _ => {}
            }
            match (self.neg, other.neg) {
                (false, true) => Ordering::Greater,
                (true, false) => Ordering::Less,
                (false, false) => Self::cmp_abs(&self.mag, &other.mag),
                (true, true) => Self::cmp_abs(&other.mag, &self.mag),
            }
        }

        fn add_abs(a: &[u32], b: &[u32]) -> Vec<u32> {
            let mut out = Vec::with_capacity(a.len().max(b.len()) + 1);
            let mut carry = 0u64;
            for i in 0..a.len().max(b.len()) {
                let x = *a.get(i).unwrap_or(&0) as u64 + *b.get(i).unwrap_or(&0) as u64 + carry;
                out.push((x % BASE) as u32);
                carry = x / BASE;
            }
            if carry > 0 {
                out.push(carry as u32);
            }
            trim(&mut out);
            out
        }

        /// `a >= b` in magnitude.
        fn sub_abs(a: &[u32], b: &[u32]) -> Vec<u32> {
            let mut out = Vec::with_capacity(a.len());
            let mut borrow = 0i64;
            for i in 0..a.len() {
                let x = a[i] as i64 - *b.get(i).unwrap_or(&0) as i64 - borrow;
                if x < 0 {
                    out.push((x + BASE as i64) as u32);
                    borrow = 1;
                } else {
                    out.push(x as u32);
                    borrow = 0;
                }
            }
            trim(&mut out);
            out
        }

        pub fn add(&self, other: &Big) -> Big {
            match (self.neg, other.neg) {
                (false, false) => Big { neg: false, mag: Self::add_abs(&self.mag, &other.mag) },
                (true, true) => Big { neg: true, mag: Self::add_abs(&self.mag, &other.mag) },
                (false, true) => self.sub_mag(&other.mag),
                (true, false) => other.sub_mag(&self.mag),
            }
        }

        /// `self - |m2|` where `self` is non-negative.
        fn sub_mag(&self, m2: &[u32]) -> Big {
            match Self::cmp_abs(&self.mag, m2) {
                std::cmp::Ordering::Less => Big {
                    neg: true,
                    mag: Self::sub_abs(m2, &self.mag),
                },
                _ => Big { neg: false, mag: Self::sub_abs(&self.mag, m2) },
            }
        }

        pub fn sub(&self, other: &Big) -> Big {
            self.add(&other.negate())
        }

        pub fn negate(&self) -> Big {
            Big { neg: !self.neg && !self.is_zero(), mag: self.mag.clone() }
        }

        pub fn mul(&self, other: &Big) -> Big {
            if self.is_zero() || other.is_zero() {
                return Big::zero();
            }
            let mut acc = vec![0u64; self.mag.len() + other.mag.len()];
            for (i, x) in self.mag.iter().enumerate() {
                let mut carry = 0u64;
                for (j, y) in other.mag.iter().enumerate() {
                    let cur = acc[i + j] + *x as u64 * *y as u64 + carry;
                    acc[i + j] = cur % BASE;
                    carry = cur / BASE;
                }
                let mut j = i + other.mag.len();
                while carry > 0 {
                    let cur = acc[j] + carry;
                    acc[j] = cur % BASE;
                    carry = cur / BASE;
                    j += 1;
                }
            }
            let mag = acc.into_iter().map(|x| x as u32).collect::<Vec<_>>();
            let mut mag = mag;
            trim(&mut mag);
            Big { neg: self.neg != other.neg, mag }
        }

        /// Truncating division and remainder (the `Nat`/`Int` runtime
        /// operations divide toward zero); division by zero is `None`.
        pub fn divmod(&self, other: &Big) -> Option<(Big, Big)> {
            if other.is_zero() {
                return None;
            }
            if self.is_zero() {
                return Some((Big::zero(), Big::zero()));
            }
            // Long division on base-1e9 limbs.
            let mut rem: Vec<u32> = Vec::new();
            let mut quot = vec![0u32; self.mag.len()];
            for (i, l) in self.mag.iter().enumerate().rev() {
                rem.insert(0, *l);
                trim(&mut rem);
                // Find the largest q with |other| * q <= rem by per-limb
                // estimation refined against the base.
                let mut q = 0u32;
                let mut lo = 0u64;
                let mut hi = BASE - 1;
                while lo <= hi {
                    let mid = (lo + hi) / 2;
                    let prod = Self::mul_abs(&other.mag, mid);
                    match Self::cmp_abs(&rem, &prod) {
                        std::cmp::Ordering::Less => {
                            if mid == 0 {
                                break;
                            }
                            hi = mid - 1;
                        }
                        _ => {
                            q = mid as u32;
                            lo = mid + 1;
                        }
                    }
                }
                if q > 0 {
                    let prod = Self::mul_abs(&other.mag, q as u64);
                    rem = Self::sub_abs(&rem, &prod);
                }
                quot[i] = q;
            }
            trim(&mut quot);
            trim(&mut rem);
            let qneg = !quot.is_empty() && self.neg != other.neg;
            let rneg = !rem.is_empty() && self.neg;
            Some((Big { neg: qneg, mag: quot }, Big { neg: rneg, mag: rem }))
        }

        fn mul_abs(a: &[u32], k: u64) -> Vec<u32> {
            if k == 0 {
                return Vec::new();
            }
            let mut out = Vec::with_capacity(a.len() + 1);
            let mut carry = 0u64;
            for x in a {
                let cur = *x as u64 * k + carry;
                out.push((cur % BASE) as u32);
                carry = cur / BASE;
            }
            while carry > 0 {
                out.push((carry % BASE) as u32);
                carry /= BASE;
            }
            trim(&mut out);
            out
        }
    }
}

/// An inline hint (attribute annotations specification section 5): a
/// hint attribute on a function declaration, erasable and never part of
/// the contract projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlineHint {
    /// `#[inline]`: inlining is desirable at the optimizer's discretion
    /// (LLVM `inlinehint`).
    Inline,
    /// `#[inline(always)]`: inlining is required unless impossible
    /// (LLVM `alwaysinline`).
    Always,
    /// `#[inline(never)]`: the function must not be inlined (LLVM
    /// `noinline`).
    Never,
}

/// The hint attributes recorded on one function (attribute annotations
/// specification sections 3 and 5). Hints may only affect performance,
/// never observable behavior: they are erased before the contract
/// projection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FnHints {
    pub inline: Option<InlineHint>,
    /// `#[cold]` on a function declaration: the function is unlikely to
    /// be called (LLVM `cold`).
    pub cold: bool,
}

/// Per-function metadata the optimizer's preservation discipline needs
/// (supercompilation and JIT specification sections 3-4). Everything here
/// is also re-derivable from the checked program; it is recorded on the
/// [`FnDef`] so the optimizer can consult it on residual programs.
#[derive(Clone, Debug)]
pub struct FnMeta {
    /// Parameters with a `%1` or `%aff` usage multiplicity (`(index,
    /// mult)`); every unlisted parameter behaves as `%many`. Driving
    /// never duplicates or drops these (section 3, linear values).
    pub linear: Vec<(usize, crate::types::Mult)>,
    /// Parameters whose function type's effect rows are empty (an honest
    /// pure callback may be reordered or fused; an effectful one may not:
    /// modules and compilation specification section 5).
    pub pure_arrows: Vec<usize>,
    /// Dictionary and capability parameters as `(index, class)` pairs,
    /// for target-rewrite application inside a capability's validity
    /// scope (platform proofs and SIMD specification section 2).
    pub dicts: Vec<(usize, String)>,
    /// Whether the definition's own effect rows are all empty.
    pub pure: bool,
}

impl Default for FnMeta {
    fn default() -> Self {
        // Units the checker does not annotate (builtins and synthesized
        // helpers) are pure unless their body says otherwise; the
        // optimizer's purity scan backstops this.
        FnMeta {
            linear: Vec::new(),
            pure_arrows: Vec::new(),
            dicts: Vec::new(),
            pure: true,
        }
    }
}

/// A record field projection's lowering plan: the runtime field slot,
/// and for boundary-layout types (`repr(C)`, `repr(packed)`,
/// `repr(transparent)`) the recorded-offset access (attribute
/// annotations specification section 4.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjField {
    pub slot: usize,
    pub access: Option<FieldAccess>,
}

impl ProjField {
    /// A uniform-word slot projection (no recorded layout).
    pub fn slot(slot: usize) -> ProjField {
        ProjField { slot, access: None }
    }
}

/// One field's runtime access inside a boundary-layout block (attribute
/// annotations specification section 4.3): the recorded offset and size
/// from the interface ABI table, plus how the stored bytes convert to
/// and from the value word.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldAccess {
    /// Byte offset from the payload base (the recorded layout offset).
    pub offset: u64,
    /// Byte size (the recorded layout size).
    pub size: u64,
    pub shape: FieldShape,
}

/// The stored shape of a boundary-layout field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldShape {
    /// A machine scalar stored in `size` bytes: `signed` selects sign
    /// extension into the value word; floats and `Char` are unsigned bit
    /// patterns (the word holds `F32` bits zero-extended, `F64` bits).
    Scalar { size: u64, signed: bool },
    /// A boxed or pointer word: 8 bytes holding the value word directly.
    Word,
    /// A nested boundary-layout aggregate stored inline: its payload is
    /// `size` bytes of `fields`, boxed out (with `tag`) when projected.
    Inline { size: u64, tag: usize, fields: Vec<FieldAccess> },
}

/// A constructor's boundary layout plan: fields stored at their recorded
/// offsets in the block payload (uniform-word types have none and keep
/// the slot convention).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CtorLayout {
    /// The recorded payload size in bytes (after the tag word).
    pub payload: u64,
    /// The store plan per runtime field slot.
    pub fields: Vec<FieldAccess>,
}

/// The marshalling class of one foreign parameter or result (foreign
/// function interface specification section 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AbiClass {
    /// `Unit`: void (elided parameter; void result).
    Unit,
    /// A machine scalar crossing by value.
    Scalar(crate::types::Scalar),
    /// Any pointer role, including opaque handles: crossing as `ptr`.
    Ptr,
    /// An aggregate crossing by const-pointer to a call-scoped C-layout
    /// copy (parameters only; a foreign result cannot be an aggregate).
    Aggregate,
}

/// One foreign parameter or result's binding contract: the widths,
/// signedness, offsets, alignment, role, nullability and ownership the
/// binding must specify (foreign function interface specification
/// section 2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbiParam {
    /// The SafeLang type as written (contract text).
    pub ty: String,
    pub class: AbiClass,
    /// The ownership role: `value`, `raw`, `borrowed`, `retained`,
    /// `owned`, `out`, `maybe-null`, or `opaque-handle` (section 3).
    pub role: String,
    /// The `Owned` allocator identity (empty otherwise).
    pub allocator: String,
    /// Whether the pointer may be null (only `MaybeNull` roles may).
    pub nullable: bool,
    /// The byte width crossing the boundary, its alignment, and (for
    /// scalars) signedness.
    pub width: u64,
    pub align: u64,
    pub signed: bool,
    /// For aggregates: the constructor tag of the boxed block built on
    /// the way in (the C-layout field plan is `fields`).
    pub tag: u64,
    /// For aggregates: the C-layout field plan (recorded offsets).
    pub fields: Vec<FieldAccess>,
}

/// One foreign binding's contract and marshalling plan (foreign function
/// interface specification sections 2-4).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForeignPlan {
    /// The C symbol identity.
    pub symbol: String,
    /// `foreign export`: emit a C-callable entry for `unit`.
    pub export: bool,
    /// The synthesized wrapper unit (import) or target unit (export).
    pub unit: String,
    /// The calling convention (`ccall`).
    pub convention: String,
    pub params: Vec<AbiParam>,
    pub result: AbiParam,
    /// For an `Owned` result: the matching deallocator's external symbol
    /// (section 4); empty when the result is not owned.
    pub deallocator: String,
}

impl ForeignPlan {
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

#[derive(Clone, Debug)]
pub struct FnDef {
    pub id: GlobalId,
    /// Parameter names for diagnostics only.
    pub params: Vec<String>,
    pub body: Core,
    /// Non-null for `foreign import ccall "safelang.prim.*"` functions; the
    /// body is ignored and the primitive is emitted instead.
    pub prim: Option<PrimOp>,
    /// For constructors: `(tag, field_count)`; the body is generated as
    /// block allocation.
    pub ctor: Option<(usize, usize)>,
    /// Hint attributes on the function (erasable; never in the contract
    /// projection).
    pub hints: FnHints,
    /// Optimizer metadata (usage multiplicities, purity, dictionaries).
    pub meta: FnMeta,
}

impl FnDef {
    pub fn arity(&self) -> usize {
        self.params.len()
    }
}

#[derive(Clone, Debug)]
pub struct InstanceDef {
    /// Dictionary construction unit, e.g. `"Main.Add_Int"`.
    pub id: GlobalId,
    pub method_count: usize,
    /// Arity of `mk`: one parameter per `using` constraint on the instance.
    pub subdict_count: usize,
    /// Method function ids, parallel to the signature's method order.
    pub methods: Vec<GlobalId>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub fns: Vec<FnDef>,
    pub instances: Vec<InstanceDef>,
    /// The synthesized exit adapter for `main :: Int`, when the entry module
    /// defines `main`. `None` for library modules, which check fine but
    /// cannot be emitted or run (codegen names the requirement).
    pub main: Option<GlobalId>,
    /// Checked rewrite rules (modules and compilation specification
    /// section 5), in declaration order, and their `target rewrite` forms
    /// (platform proofs and SIMD specification section 3). Rules never
    /// extend definitional equality: conversion is exactly the D28 rules.
    pub rewrites: Vec<RewriteRule>,
    /// Boundary layout plans for constructors of `repr(C)`,
    /// `repr(packed)` and `repr(transparent)` types, keyed by the
    /// constructor unit name (attribute annotations specification
    /// section 4.3: runtime field access uses the recorded offsets).
    pub plans: std::collections::BTreeMap<String, CtorLayout>,
    /// Foreign binding contracts, keyed by unit name (foreign function
    /// interface specification section 2).
    pub foreigns: std::collections::BTreeMap<String, ForeignPlan>,
}

/// A checked rewrite rule. `pattern` is the elaborated form of the rule's
/// `match` term, whose occurrences of the pattern variables (the rule's
/// `forall` and `using` binders, by [`LocalId`]) are the match holes;
/// `template` is the elaborated `replace` term over the same binder ids.
/// Matching is first-order: a hole matches any subterm, and a repeated
/// hole must match structurally equal subterms.
#[derive(Clone, Debug)]
pub struct RewriteRule {
    pub name: String,
    pub module: String,
    /// `target rewrite`: may only be applied where its capability
    /// condition is valid (platform proofs and SIMD specification
    /// section 3).
    pub target: bool,
    /// The pattern variables' local ids (the match holes).
    pub holes: Vec<LocalId>,
    pub pattern: Core,
    pub template: Core,
    /// The `require` applicability condition over the same binders.
    pub require: Option<Core>,
    /// The condition's operative form (the optimizer evaluates this at a
    /// match site; `require` is kept for the report).
    pub cond: RequireCond,
    /// Holes whose bound argument must be a pure callback for the rule to
    /// apply (modules and compilation specification section 5: a theorem
    /// proved for pure values must not be reused for effectful callbacks).
    pub pure_holes: Vec<LocalId>,
    /// Holes whose bound argument supplies a capability or dictionary of
    /// this class (the rule's `using` binders).
    pub dict_holes: Vec<(LocalId, String)>,
    /// Justification identities recorded in the artifact manifest
    /// (`assumptions.declared` when no instance discharges them).
    pub assumptions: Vec<String>,
    /// Source line of the declaration (for the specialization report).
    pub line: u32,
}

/// A `require` clause's operative form. The optimizer applies a rule only
/// where its condition holds (platform proofs and SIMD specification
/// section 3: a target rule applies only in a region where the required
/// capability is valid).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequireCond {
    /// No `require` clause (or one that is statically true).
    Always,
    /// `Available v` over the rule's `using` binder `hole` of class
    /// `class`: the rule applies only where that capability is in the
    /// enclosing function's validity scope.
    CapabilityInScope { hole: LocalId, class: String },
    /// Statically false: the rule never applies.
    Never,
    /// Not statically decidable at the match site: refused there.
    Unknown,
}

// ============================== P10: the optimizer ==============================
//
// Rewrites, the driving engine, and target profiles (modules and
// compilation specification sections 5, 6, 10 and 13; supercompilation
// and JIT specification sections 2-5, 8 and 9; attribute annotations
// specification section 2.4). The engine is a library over the core
// language: it drives symbolic evaluation bounded by a whistle and a fuel
// bound taken from the target profile, residualizes at the barriers of
// supercompilation section 3, reconciles decision-point weights as the
// attribute chapter requires, and rechecks every residual program before
// installing it (trust level 2, full recheck).

/// The target's vector shape (platform proofs and SIMD specification
/// section 6): a fixed lane width, or a scalable vector whose active
/// length is a runtime parameter of the proof. A rule over a scalable
/// shape must not assume an observed length stays fixed beyond the
/// capability's validity scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorShape {
    Fixed(u64),
    Scalable,
}

/// A target profile: the instruction semantics and deployment assumptions
/// of the artifact, and the driving bounds of specialization (platform
/// proofs and SIMD specification section 1; supercompilation and JIT
/// specification section 5: fuel and measure bounds are target profile
/// parameters). The profile records which fields a proof depends on; a
/// changed intrinsic model invalidates dependent specializations.
#[derive(Clone, Debug)]
pub struct TargetProfile {
    pub name: String,
    /// Pointer and word width in bits (`Usize`/`Isize` width).
    pub word_size: u32,
    /// `"little"` or `"big"`.
    pub endian: String,
    pub vector_shape: VectorShape,
    /// Feature assumptions the deployment contract requires.
    pub features: Vec<String>,
    /// Driving fuel: how many unfoldings one driving site may spend
    /// before it must residualize (supercompilation section 5).
    pub fuel: u32,
    /// The whistle's measure bound: residual programs may not grow past
    /// `measure` times the original size.
    pub measure: u32,
    /// Whether this profile admits runtime code generation (`eval` of a
    /// typed quote; supercompilation section 7: a profile MAY forbid it).
    pub allow_eval: bool,
}

impl TargetProfile {
    /// The profile of the build host's own 64-bit target: little endian,
    /// fixed 8-lane 32-bit vectors, the bootstrap's driving defaults.
    pub fn native() -> Self {
        TargetProfile {
            name: "native64".to_string(),
            word_size: 64,
            endian: "little".to_string(),
            vector_shape: VectorShape::Fixed(8),
            features: vec!["u32.wrappingAdd".to_string()],
            fuel: 8,
            measure: 8,
            allow_eval: true,
        }
    }
}

/// One specialization site's outcome (supercompilation and JIT
/// specification section 8: the report says where specialization stopped
/// and which barrier or bound stopped it).
#[derive(Clone, Debug)]
pub struct OptSite {
    /// The residual unit the transformation applies to.
    pub fn_id: String,
    /// `"unfold"`, `"case-fold"`, `"const-fold"`, `"dictionary-specialize"`,
    /// `"effect-static"`, `"rewrite:<name>"`, `"fusion"`, `"case-merge"`,
    /// `"dead-alternative"`, or `"case-push"`.
    pub kind: String,
    /// `None` when the transformation ran to completion, otherwise the
    /// barrier or bound that stopped it (`"fuel-exhausted"`,
    /// `"whistle: ..."`, `"effect barrier: ..."`, `"allocation barrier:
    /// ..."`, `"linear value: ..."`, `"opaque boundary: ..."`,
    /// `"capability scope: ..."`).
    pub stop: Option<String>,
}

/// The compiler report of specialization (modules and compilation
/// specification section 10; supercompilation section 8).
#[derive(Clone, Debug, Default)]
pub struct OptReport {
    /// Every driving site with its stopping reason, in program order.
    pub sites: Vec<OptSite>,
    /// Fusion steps that occurred (the map-fusion case of modules
    /// section 6: which allocation or effect prevented a requested
    /// transformation is named in `sites[].stop`).
    pub fused: Vec<String>,
    /// Residual programs rechecked before installation (trust level 2).
    pub rechecked: Vec<String>,
    /// Candidates whose recheck failed: discarded, never installed
    /// (supercompilation section 9).
    pub discarded: Vec<String>,
    /// Justification identities the rules relied on, for the artifact
    /// manifest's `assumptions.declared`.
    pub assumptions: Vec<String>,
}

// ------------------------------------------------------------- core utilities

/// Structural equality of core terms (pattern matching needs it for a
/// repeated pattern variable).
fn core_eq(a: &Core, b: &Core) -> bool {
    match (a, b) {
        (Core::Int(x), Core::Int(y)) => x == y,
        (Core::ConstBig { nat: n1, digits: d1 }, Core::ConstBig { nat: n2, digits: d2 }) => {
            n1 == n2 && d1 == d2
        }
        (Core::Bool(x), Core::Bool(y)) => x == y,
        (Core::Unit, Core::Unit) | (Core::Fault, Core::Fault) | (Core::LazyAlloc, Core::LazyAlloc) => true,
        (Core::Var(x), Core::Var(y)) => x == y,
        (Core::Global(x), Core::Global(y)) | (Core::Ctor(x), Core::Ctor(y)) => x == y,
        (Core::Prim(x), Core::Prim(y)) => x == y,
        (Core::Foreign(x), Core::Foreign(y)) => x == y,
        (Core::DictProj(d1, i1), Core::DictProj(d2, i2)) => i1 == i2 && core_eq(d1, d2),
        (Core::Proj(d1, i1), Core::Proj(d2, i2)) => i1 == i2 && core_eq(d1, d2),
        (Core::InstDict(g1, a1), Core::InstDict(g2, a2)) => {
            g1 == g2 && a1.len() == a2.len() && a1.iter().zip(a2).all(|(x, y)| core_eq(x, y))
        }
        (Core::App(f1, a1), Core::App(f2, a2)) => {
            core_eq(f1, f2) && a1.len() == a2.len() && a1.iter().zip(a2).all(|(x, y)| core_eq(x, y))
        }
        (Core::Lam { fn_id: g1, env: e1 }, Core::Lam { fn_id: g2, env: e2 }) => {
            g1 == g2 && e1.len() == e2.len() && e1.iter().zip(e2).all(|(x, y)| core_eq(x, y))
        }
        (Core::Let(l1, v1, b1), Core::Let(l2, v2, b2)) => l1 == l2 && core_eq(v1, v2) && core_eq(b1, b2),
        (Core::If(c1, t1, f1), Core::If(c2, t2, f2)) => {
            core_eq(c1, c2)
                && t1.weight == t2.weight
                && f1.weight == f2.weight
                && core_eq(&t1.body, &t2.body)
                && core_eq(&f1.body, &f2.body)
        }
        (Core::Case { scrut: s1, alts: a1 }, Core::Case { scrut: s2, alts: a2 }) => {
            core_eq(s1, s2)
                && a1.len() == a2.len()
                && a1.iter().zip(a2).all(|(x, y)| {
                    x.weight == y.weight && pat_eq(&x.pat, &y.pat) && core_eq(&x.body, &y.body)
                })
        }
        (Core::Demand(d1), Core::Demand(d2)) => core_eq(d1, d2),
        (Core::LazyInit(l1, t1), Core::LazyInit(l2, t2)) => l1 == l2 && core_eq(t1, t2),
        (Core::StrLit(s1), Core::StrLit(s2)) => s1 == s2,
        (Core::SyntaxLit(t1), Core::SyntaxLit(t2)) => format!("{t1:?}") == format!("{t2:?}"),
        _ => false,
    }
}

fn pat_eq(a: &CPat, b: &CPat) -> bool {
    match (a, b) {
        (CPat::Var(x), CPat::Var(y)) => x == y,
        (CPat::Wild, CPat::Wild) => true,
        (CPat::Con { tag: t1, fields: f1 }, CPat::Con { tag: t2, fields: f2 }) => {
            t1 == t2 && f1 == f2
        }
        _ => false,
    }
}

/// The largest local id a term mentions (fresh ids must exceed it).
fn max_local(e: &Core) -> usize {
    let mut m = 0;
    walk_locals(e, &mut |l: LocalId| {
        m = m.max(l.0);
    });
    m
}

pub fn walk_locals(e: &Core, f: &mut impl FnMut(LocalId)) {
    match e {
        Core::Var(l) => f(*l),
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
        | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
        | Core::StrLit(_) => {}
        Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => walk_locals(d, f),
        Core::InstDict(_, args) | Core::App(_, args) => {
            if let Core::App(h, _) = e {
                walk_locals(h, f);
            }
            for a in args {
                walk_locals(a, f);
            }
        }
        Core::Lam { env, .. } => {
            for a in env {
                walk_locals(a, f);
            }
        }
        Core::Let(l, v, body) => {
            f(*l);
            walk_locals(v, f);
            walk_locals(body, f);
        }
        Core::LazyInit(l, t) => {
            f(*l);
            walk_locals(t, f);
        }
        Core::If(c, t, fe) => {
            walk_locals(c, f);
            walk_locals(&t.body, f);
            walk_locals(&fe.body, f);
        }
        Core::Case { scrut, alts } => {
            walk_locals(scrut, f);
            for a in alts {
                match &a.pat {
                    CPat::Var(l) => f(*l),
                    CPat::Con { fields, .. } => {
                        for l in fields {
                            f(*l);
                        }
                    }
                    CPat::Wild => {}
                }
                walk_locals(&a.body, f);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                walk_locals(h, f);
            }
        }
    }
}

/// Per-path use counts of one local: `(min, max)` over the evaluation
/// paths (a branch contributes the sum across its taken side only).
fn use_range(e: &Core, l: LocalId) -> (usize, usize) {
    let seq = |parts: &[(usize, usize)]| {
        (
            parts.iter().map(|p| p.0).sum(),
            parts.iter().map(|p| p.1).sum(),
        )
    };
    match e {
        Core::Var(v) => {
            if *v == l {
                (1, 1)
            } else {
                (0, 0)
            }
        }
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
        | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
        | Core::StrLit(_) => (0, 0),
        Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => use_range(d, l),
        Core::InstDict(_, args) | Core::App(_, args) => {
            let mut parts: Vec<(usize, usize)> = Vec::new();
            if let Core::App(h, _) = e {
                parts.push(use_range(h, l));
            }
            for a in args {
                parts.push(use_range(a, l));
            }
            seq(&parts)
        }
        Core::Lam { env, .. } => {
            let parts: Vec<(usize, usize)> = env.iter().map(|a| use_range(a, l)).collect();
            seq(&parts)
        }
        Core::Let(x, v, body) => {
            let (vmin, vmax) = use_range(v, l);
            if *x == l {
                (vmin, vmax)
            } else {
                let (bmin, bmax) = use_range(body, l);
                (vmin + bmin, vmax + bmax)
            }
        }
        Core::LazyInit(x, t) => {
            if *x == l {
                (0, 0)
            } else {
                use_range(t, l)
            }
        }
        Core::If(c, t, fe) => {
            let (cmin, cmax) = use_range(c, l);
            let (tmin, tmax) = use_range(&t.body, l);
            let (fmin, fmax) = use_range(&fe.body, l);
            (cmin + tmin.min(fmin), cmax + tmax.max(fmax))
        }
        Core::Case { scrut, alts } => {
            let (smin, smax) = use_range(scrut, l);
            let mut mins = Vec::new();
            let mut maxs = Vec::new();
            for a in alts {
                let shadowed = match &a.pat {
                    CPat::Var(x) => *x == l,
                    CPat::Con { fields, .. } => fields.contains(&l),
                    CPat::Wild => false,
                };
                let r = if shadowed { (0, 0) } else { use_range(&a.body, l) };
                mins.push(r.0);
                maxs.push(r.1);
            }
            let (bmin, bmax) = if alts.is_empty() {
                (0, 0)
            } else {
                (mins.into_iter().min().unwrap_or(0), maxs.into_iter().max().unwrap_or(0))
            };
            (smin + bmin, smax + bmax)
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            let parts: Vec<(usize, usize)> = holes.iter().map(|h| use_range(h, l)).collect();
            seq(&parts)
        }
    }
}

/// Alpha-rename every binder of `e` densely, parameters first (mirroring
/// the checker's `renumber_fn` for lowered core).
fn renumber_core(e: &Core, nparams: usize) -> Core {
    let mut map: HashMap<LocalId, LocalId> = HashMap::new();
    for i in 0..nparams {
        map.insert(LocalId(i), LocalId(i));
    }
    let mut next = nparams;
    let mut note = |l: LocalId, map: &mut HashMap<LocalId, LocalId>, next: &mut usize| {
        map.entry(l).or_insert_with(|| {
            let id = LocalId(*next);
            *next += 1;
            id
        });
    };
    // First assign, then rewrite (a body use may precede its binder's
    // textual position in the id order).
    fn assign(e: &Core, map: &mut HashMap<LocalId, LocalId>, next: &mut usize) {
        let mut note = |l: LocalId, map: &mut HashMap<LocalId, LocalId>, next: &mut usize| {
            map.entry(l).or_insert_with(|| {
                let id = LocalId(*next);
                *next += 1;
                id
            });
        };
        match e {
            Core::Var(l) => note(*l, map, next),
            Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
            | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
            | Core::StrLit(_) => {}
            Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => assign(d, map, next),
            Core::InstDict(_, args) | Core::App(_, args) => {
                if let Core::App(h, _) = e {
                    assign(h, map, next);
                }
                for a in args {
                    assign(a, map, next);
                }
            }
            Core::Lam { env, .. } => {
                for a in env {
                    assign(a, map, next);
                }
            }
            Core::Let(l, v, body) => {
                note(*l, map, next);
                assign(v, map, next);
                assign(body, map, next);
            }
            Core::LazyInit(l, t) => {
                note(*l, map, next);
                assign(t, map, next);
            }
            Core::If(c, t, fe) => {
                assign(c, map, next);
                assign(&t.body, map, next);
                assign(&fe.body, map, next);
            }
            Core::Case { scrut, alts } => {
                assign(scrut, map, next);
                for a in alts {
                    match &a.pat {
                        CPat::Var(l) => note(*l, map, next),
                        CPat::Con { fields, .. } => {
                            for l in fields {
                                note(*l, map, next);
                            }
                        }
                        CPat::Wild => {}
                    }
                    assign(&a.body, map, next);
                }
            }
            Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
                for h in holes {
                    assign(h, map, next);
                }
            }
        }
    }
    fn rewrite(e: &Core, map: &HashMap<LocalId, LocalId>) -> Core {
        let rw = |x: &Core| rewrite(x, map);
        match e {
            Core::Var(l) => Core::Var(*map.get(l).unwrap_or(l)),
            Core::Int(v) => Core::Int(*v),
            Core::ConstBig { nat, digits } => Core::ConstBig { nat: *nat, digits: digits.clone() },
            Core::Bool(b) => Core::Bool(*b),
            Core::Unit => Core::Unit,
            Core::Global(g) => Core::Global(g.clone()),
            Core::Ctor(g) => Core::Ctor(g.clone()),
            Core::Prim(p) => Core::Prim(*p),
            Core::Foreign(n) => Core::Foreign(n.clone()),
            Core::Fault => Core::Fault,
            Core::LazyAlloc => Core::LazyAlloc,
            Core::SyntaxLit(t) => Core::SyntaxLit(t.clone()),
            Core::StrLit(s) => Core::StrLit(s.clone()),
            Core::DictProj(d, i) => Core::DictProj(Box::new(rw(d)), *i),
            Core::Proj(d, f) => Core::Proj(Box::new(rw(d)), f.clone()),
            Core::Demand(d) => Core::Demand(Box::new(rw(d))),
            Core::InstDict(g, args) => Core::InstDict(g.clone(), args.iter().map(rw).collect()),
            Core::App(f, args) => Core::App(Box::new(rw(f)), args.iter().map(rw).collect()),
            Core::Lam { fn_id, env } => Core::Lam {
                fn_id: fn_id.clone(),
                env: env.iter().map(rw).collect(),
            },
            Core::Let(l, v, body) => Core::Let(
                *map.get(l).unwrap_or(l),
                Box::new(rw(v)),
                Box::new(rw(body)),
            ),
            Core::LazyInit(l, t) => {
                Core::LazyInit(*map.get(l).unwrap_or(l), Box::new(rw(t)))
            }
            Core::If(c, t, fe) => Core::If(
                Box::new(rw(c)),
                CBranch { weight: t.weight, body: Box::new(rw(&t.body)) },
                CBranch { weight: fe.weight, body: Box::new(rw(&fe.body)) },
            ),
            Core::Case { scrut, alts } => Core::Case {
                scrut: Box::new(rw(scrut)),
                alts: alts
                    .iter()
                    .map(|a| {
                        let pat = match &a.pat {
                            CPat::Var(l) => CPat::Var(*map.get(l).unwrap_or(l)),
                            CPat::Con { tag, fields } => CPat::Con {
                                tag: *tag,
                                fields: fields
                                    .iter()
                                    .map(|l| *map.get(l).unwrap_or(l))
                                    .collect(),
                            },
                            CPat::Wild => CPat::Wild,
                        };
                        CAlt { pat, weight: a.weight, body: Box::new(rw(&a.body)) }
                    })
                    .collect(),
            },
            Core::Quote { template, holes } => Core::Quote {
                template: template.clone(),
                holes: holes.iter().map(rw).collect(),
            },
            Core::DeclsQuote { template, holes } => Core::DeclsQuote {
                template: template.clone(),
                holes: holes.iter().map(rw).collect(),
            },
        }
    }
    let _ = &mut note;
    assign(e, &mut map, &mut next);
    rewrite(e, &map)
}

/// Instantiate a rule's template at a match site: pattern variables
/// (`holes`) take their bound subterms, and the template's own binders are
/// alpha-renamed away from the residual program's locals.
fn instantiate(
    tmpl: &Core,
    holes: &std::collections::HashSet<LocalId>,
    binds: &HashMap<LocalId, Core>,
    next: &mut usize,
) -> Core {
    let mut ren: HashMap<LocalId, LocalId> = HashMap::new();
    fn go(
        e: &Core,
        holes: &std::collections::HashSet<LocalId>,
        binds: &HashMap<LocalId, Core>,
        ren: &mut HashMap<LocalId, LocalId>,
        next: &mut usize,
    ) -> Core {
        let fresh = |l: LocalId, ren: &mut HashMap<LocalId, LocalId>, next: &mut usize| {
            *ren.entry(l).or_insert_with(|| {
                let id = LocalId(*next);
                *next += 1;
                id
            })
        };
        match e {
            Core::Var(l) => {
                if holes.contains(l) {
                    binds.get(l).cloned().unwrap_or(Core::Fault)
                } else {
                    Core::Var(*ren.get(l).unwrap_or(l))
                }
            }
            Core::Int(v) => Core::Int(*v),
            Core::ConstBig { nat, digits } => Core::ConstBig { nat: *nat, digits: digits.clone() },
            Core::Bool(b) => Core::Bool(*b),
            Core::Unit => Core::Unit,
            Core::Global(g) => Core::Global(g.clone()),
            Core::Ctor(g) => Core::Ctor(g.clone()),
            Core::Prim(p) => Core::Prim(*p),
            Core::Foreign(n) => Core::Foreign(n.clone()),
            Core::Fault => Core::Fault,
            Core::LazyAlloc => Core::LazyAlloc,
            Core::SyntaxLit(t) => Core::SyntaxLit(t.clone()),
            Core::StrLit(s) => Core::StrLit(s.clone()),
            Core::DictProj(d, i) => {
                Core::DictProj(Box::new(go(d, holes, binds, ren, next)), *i)
            }
            Core::Proj(d, f) => Core::Proj(Box::new(go(d, holes, binds, ren, next)), f.clone()),
            Core::Demand(d) => Core::Demand(Box::new(go(d, holes, binds, ren, next))),
            Core::InstDict(g, args) => Core::InstDict(
                g.clone(),
                args.iter().map(|a| go(a, holes, binds, ren, next)).collect(),
            ),
            Core::App(f, args) => Core::App(
                Box::new(go(f, holes, binds, ren, next)),
                args.iter().map(|a| go(a, holes, binds, ren, next)).collect(),
            ),
            Core::Lam { fn_id, env } => Core::Lam {
                fn_id: fn_id.clone(),
                env: env.iter().map(|a| go(a, holes, binds, ren, next)).collect(),
            },
            Core::Let(l, v, body) => {
                let v = go(v, holes, binds, ren, next);
                let nl = fresh(*l, ren, next);
                let body = go(body, holes, binds, ren, next);
                ren.remove(l);
                Core::Let(nl, Box::new(v), Box::new(body))
            }
            Core::LazyInit(l, t) => {
                let nl = fresh(*l, ren, next);
                let t = go(t, holes, binds, ren, next);
                ren.remove(l);
                Core::LazyInit(nl, Box::new(t))
            }
            Core::If(c, t, fe) => Core::If(
                Box::new(go(c, holes, binds, ren, next)),
                CBranch {
                    weight: t.weight,
                    body: Box::new(go(&t.body, holes, binds, ren, next)),
                },
                CBranch {
                    weight: fe.weight,
                    body: Box::new(go(&fe.body, holes, binds, ren, next)),
                },
            ),
            Core::Case { scrut, alts } => Core::Case {
                scrut: Box::new(go(scrut, holes, binds, ren, next)),
                alts: alts
                    .iter()
                    .map(|a| {
                        let mut locals: Vec<LocalId> = Vec::new();
                        let pat = match &a.pat {
                            CPat::Var(l) => {
                                let nl = fresh(*l, ren, next);
                                locals.push(*l);
                                CPat::Var(nl)
                            }
                            CPat::Con { tag, fields } => {
                                let mut out = Vec::new();
                                for l in fields {
                                    let nl = fresh(*l, ren, next);
                                    locals.push(*l);
                                    out.push(nl);
                                }
                                CPat::Con { tag: *tag, fields: out }
                            }
                            CPat::Wild => CPat::Wild,
                        };
                        let body = go(&a.body, holes, binds, ren, next);
                        for l in locals {
                            ren.remove(&l);
                        }
                        CAlt { pat, weight: a.weight, body: Box::new(body) }
                    })
                    .collect(),
            },
            Core::Quote { template, holes: hs } => Core::Quote {
                template: template.clone(),
                holes: hs.iter().map(|h| go(h, holes, binds, ren, next)).collect(),
            },
            Core::DeclsQuote { template, holes: hs } => Core::DeclsQuote {
                template: template.clone(),
                holes: hs.iter().map(|h| go(h, holes, binds, ren, next)).collect(),
            },
        }
    }
    go(tmpl, holes, binds, &mut ren, next)
}

/// First-order matching of a rule's pattern against a core subterm: a
/// pattern variable matches anything, and a repeated variable must match
/// structurally equal subterms.
fn match_pat(
    pat: &Core,
    holes: &std::collections::HashSet<LocalId>,
    sub: &Core,
    binds: &mut HashMap<LocalId, Core>,
) -> bool {
    if let Core::Var(l) = pat {
        if holes.contains(l) {
            match binds.get(l) {
                Some(prev) => return core_eq(prev, sub),
                None => {
                    binds.insert(*l, sub.clone());
                    return true;
                }
            }
        }
    }
    match (pat, sub) {
        (Core::Int(x), Core::Int(y)) => x == y,
        (Core::ConstBig { nat: n1, digits: d1 }, Core::ConstBig { nat: n2, digits: d2 }) => {
            n1 == n2 && d1 == d2
        }
        (Core::Bool(x), Core::Bool(y)) => x == y,
        (Core::Unit, Core::Unit) | (Core::Fault, Core::Fault) | (Core::LazyAlloc, Core::LazyAlloc) => {
            true
        }
        (Core::Var(x), Core::Var(y)) => x == y,
        (Core::Global(x), Core::Global(y)) | (Core::Ctor(x), Core::Ctor(y)) => x == y,
        (Core::Prim(x), Core::Prim(y)) => x == y,
        (Core::Foreign(x), Core::Foreign(y)) => x == y,
        (Core::DictProj(d1, i1), Core::DictProj(d2, i2)) => {
            i1 == i2 && match_pat(d1, holes, d2, binds)
        }
        (Core::Proj(d1, i1), Core::Proj(d2, i2)) => {
            i1 == i2 && match_pat(d1, holes, d2, binds)
        }
        (Core::InstDict(g1, a1), Core::InstDict(g2, a2)) => {
            g1 == g2
                && a1.len() == a2.len()
                && a1.iter().zip(a2).all(|(x, y)| match_pat(x, holes, y, binds))
        }
        (Core::App(f1, a1), Core::App(f2, a2)) => {
            a1.len() == a2.len()
                && match_pat(f1, holes, f2, binds)
                && a1.iter().zip(a2).all(|(x, y)| match_pat(x, holes, y, binds))
        }
        (Core::Lam { fn_id: g1, env: e1 }, Core::Lam { fn_id: g2, env: e2 }) => {
            g1 == g2
                && e1.len() == e2.len()
                && e1.iter().zip(e2).all(|(x, y)| match_pat(x, holes, y, binds))
        }
        (Core::Let(_, v1, b1), Core::Let(_, v2, b2)) => {
            match_pat(v1, holes, v2, binds) && match_pat(b1, holes, b2, binds)
        }
        (Core::If(c1, t1, f1), Core::If(c2, t2, f2)) => {
            t1.weight == t2.weight
                && f1.weight == f2.weight
                && match_pat(c1, holes, c2, binds)
                && match_pat(&t1.body, holes, &t2.body, binds)
                && match_pat(&f1.body, holes, &f2.body, binds)
        }
        (Core::Case { scrut: s1, alts: a1 }, Core::Case { scrut: s2, alts: a2 }) => {
            a1.len() == a2.len()
                && match_pat(s1, holes, s2, binds)
                && a1.iter().zip(a2).all(|(x, y)| {
                    x.weight == y.weight
                        && pat_shape_eq(&x.pat, &y.pat)
                        && match_pat(&x.body, holes, &y.body, binds)
                })
        }
        (Core::Demand(d1), Core::Demand(d2)) => match_pat(d1, holes, d2, binds),
        (Core::LazyInit(_, t1), Core::LazyInit(_, t2)) => match_pat(t1, holes, t2, binds),
        (Core::StrLit(s1), Core::StrLit(s2)) => s1 == s2,
        (Core::SyntaxLit(t1), Core::SyntaxLit(t2)) => format!("{t1:?}") == format!("{t2:?}"),
        _ => false,
    }
}

/// Pattern shapes match when their binders are alpha-equivalent (the
/// pattern's own binders are freshened at every use).
fn pat_shape_eq(a: &CPat, b: &CPat) -> bool {
    match (a, b) {
        (CPat::Var(_), CPat::Var(_)) | (CPat::Wild, CPat::Wild) => true,
        (CPat::Con { tag: t1, fields: f1 }, CPat::Con { tag: t2, fields: f2 }) => {
            t1 == t2 && f1.len() == f2.len()
        }
        _ => false,
    }
}

// ------------------------------------------------------- constant folding

/// Word width and signedness of a machine scalar (the word model stores
/// every scalar as its raw `i64` bit pattern, zero/sign extended).
fn scalar_bits(s: Scalar) -> (u32, bool) {
    use Scalar::*;
    match s {
        I8 => (8, true),
        I16 => (16, true),
        I32 => (32, true),
        I64 => (64, true),
        U8 => (8, false),
        U16 => (16, false),
        U32 => (32, false),
        U64 => (64, false),
        Usize | Isize => (64, s == Isize),
        Char => (32, false),
        F32 | F64 => (64, false),
    }
}

fn scalar_max(s: Scalar) -> i128 {
    let (bits, signed) = scalar_bits(s);
    if signed {
        (1i128 << (bits - 1)) - 1
    } else if bits >= 64 {
        u64::MAX as i128
    } else {
        (1i128 << bits) - 1
    }
}

fn scalar_min(s: Scalar) -> i128 {
    let (bits, signed) = scalar_bits(s);
    if signed {
        -(1i128 << (bits - 1))
    } else {
        0
    }
}

/// Truncate a raw word to the scalar's stored pattern.
fn trunc_word(v: i128, s: Scalar) -> i64 {
    let (bits, signed) = scalar_bits(s);
    if bits >= 64 {
        v as i64
    } else {
        let m = (1i128 << bits) - 1;
        let t = v & m;
        if signed && (t & (1i128 << (bits - 1))) != 0 {
            (t | !m) as i64
        } else {
            t as i64
        }
    }
}

/// The Option constructors a folded checked primitive returns to.
struct FoldCtors {
    none: Option<GlobalId>,
    some: Option<GlobalId>,
}

fn opt_value(ctors: &FoldCtors, v: Option<Core>) -> Option<Core> {
    match v {
        Some(v) => {
            let g = ctors.some.as_ref()?;
            Some(Core::App(Box::new(Core::Ctor(g.clone())), vec![v]))
        }
        None => {
            let g = ctors.none.as_ref()?;
            Some(Core::App(Box::new(Core::Ctor(g.clone())), vec![]))
        }
    }
}

/// Fold a primitive application on known arguments (supercompilation and
/// JIT specification section 2: constant folding of pure total
/// primitives, with the defined fault behavior for out-of-domain results:
/// a folded `checkedDiv` on zero folds to `None` and a division that
/// faults is never silently dropped).
fn fold_prim(op: PrimOp, args: &[Core], ctors: &FoldCtors) -> Option<Core> {
    use self::big::Big;
    let words: Vec<i64> = args
        .iter()
        .map(|a| match a {
            Core::Int(v) => Some(*v),
            Core::Bool(b) => Some(*b as i64),
            _ => None,
        })
        .collect::<Option<_>>()?;
    let bigs: Option<Vec<(bool, Big)>> = args
        .iter()
        .map(|a| match a {
            Core::ConstBig { nat, digits } => Some((*nat, Big::from_dec(digits)?)),
            Core::Int(v) => Some((false, Big::from_i64(*v))),
            _ => None,
        })
        .collect();
    match op {
        PrimOp::Arith { ty, op: aop, mode } => {
            let (x, y) = (words[0] as i128, words[1] as i128);
            let raw = match aop {
                ArithOp::Add => x + y,
                ArithOp::Sub => x - y,
                ArithOp::Mul => x * y,
            };
            Some(Core::Int(match mode {
                ArithMode::Wrapping => trunc_word(raw, ty),
                ArithMode::Saturating => trunc_word(raw.clamp(scalar_min(ty), scalar_max(ty)), ty),
                ArithMode::Checked => {
                    return opt_value(
                        ctors,
                        if raw < scalar_min(ty) || raw > scalar_max(ty) {
                            None
                        } else {
                            Some(Core::Int(trunc_word(raw, ty)))
                        },
                    )
                }
            }))
        }
        PrimOp::Neg { ty, mode } => {
            let raw = -(words[0] as i128);
            Some(Core::Int(match mode {
                ArithMode::Wrapping => trunc_word(raw, ty),
                ArithMode::Saturating => trunc_word(raw.clamp(scalar_min(ty), scalar_max(ty)), ty),
                ArithMode::Checked => {
                    return opt_value(
                        ctors,
                        if raw < scalar_min(ty) || raw > scalar_max(ty) {
                            None
                        } else {
                            Some(Core::Int(trunc_word(raw, ty)))
                        },
                    )
                }
            }))
        }
        PrimOp::CheckedDiv { ty } | PrimOp::CheckedRem { ty } => {
            let (x, y) = (words[0] as i128, words[1] as i128);
            if y == 0 {
                return opt_value(ctors, None);
            }
            let (bits, signed) = scalar_bits(ty);
            let _ = bits;
            let q = if matches!(op, PrimOp::CheckedDiv { .. }) { x / y } else { x % y };
            if signed && q < scalar_min(ty) || q > scalar_max(ty) {
                return opt_value(ctors, None);
            }
            opt_value(ctors, Some(Core::Int(trunc_word(q, ty))))
        }
        PrimOp::Shift { ty, op: sop, checked } => {
            let (x, sh) = (words[0] as i128, words[1] as u32);
            let (bits, _) = scalar_bits(ty);
            if checked {
                return opt_value(
                    ctors,
                    if sh >= bits {
                        None
                    } else {
                        Some(Core::Int(match sop {
                            ShiftOp::Shl => trunc_word(x << sh, ty),
                            ShiftOp::Shr => trunc_word(x >> sh, ty),
                        }))
                    },
                );
            }
            let sh = sh % bits.max(1);
            Some(Core::Int(match sop {
                ShiftOp::Shl => trunc_word(x << sh, ty),
                ShiftOp::Shr => trunc_word(x >> sh, ty),
            }))
        }
        PrimOp::Rotate { ty, right } => {
            let (bits, _) = scalar_bits(ty);
            let mask: u64 = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
            let pat = (words[0] as u64) & mask;
            let sh = (words[1] as u32) % bits.max(1);
            let r = if sh == 0 {
                pat
            } else if right {
                (pat >> sh) | (pat << (bits - sh))
            } else {
                (pat << sh) | (pat >> (bits - sh))
            };
            Some(Core::Int(trunc_word((r & mask) as i128, ty)))
        }
        PrimOp::Bit { op: bop, ty } => {
            let (x, y) = (words[0], words[1]);
            let raw = match bop {
                BitOp::And => x & y,
                BitOp::Or => x | y,
                BitOp::Xor => x ^ y,
            };
            Some(Core::Int(trunc_word(raw as i128, ty)))
        }
        PrimOp::BitNot { ty } => Some(Core::Int(trunc_word(!(words[0] as i128), ty))),
        PrimOp::BitScan { op: bop, ty } => {
            let (bits, _) = scalar_bits(ty);
            let mask: u64 = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
            let pat = (words[0] as u64) & mask;
            let v = match bop {
                BitScanOp::CountOnes => pat.count_ones(),
                BitScanOp::LeadingZeros => pat.leading_zeros() - (64 - bits),
                BitScanOp::TrailingZeros => {
                    if pat == 0 {
                        bits
                    } else {
                        pat.trailing_zeros()
                    }
                }
                BitScanOp::ReverseBits => (pat.reverse_bits() >> (64 - bits)) as u32,
                BitScanOp::SwapBytes => ((pat.swap_bytes() >> (64 - bits)) & mask) as u32,
            };
            let v = match bop {
                BitScanOp::ReverseBits | BitScanOp::SwapBytes => {
                    return Some(Core::Int(trunc_word(v as i128, ty)))
                }
                _ => v,
            };
            Some(Core::Int(v as i64))
        }
        PrimOp::Cmp { op: cop, .. } => {
            let (x, y) = (words[0], words[1]);
            let b = match cop {
                CmpOp::Eq => x == y,
                CmpOp::Ne => x != y,
                CmpOp::Lt => x < y,
                CmpOp::Le => x <= y,
                CmpOp::Gt => x > y,
                CmpOp::Ge => x >= y,
            };
            Some(Core::Bool(b))
        }
        PrimOp::NotBool => Some(Core::Bool(words[0] == 0)),
        PrimOp::AndBool => Some(Core::Bool(words[0] != 0 && words[1] != 0)),
        PrimOp::OrBool => Some(Core::Bool(words[0] != 0 || words[1] != 0)),
        PrimOp::Nat(nop) => {
            let bs = bigs?;
            match nop {
                NatOp::Add => {
                    let v = bs[0].1.add(&bs[1].1);
                    Some(Core::ConstBig { nat: true, digits: v.to_dec() })
                }
                NatOp::Sub => {
                    let v = bs[0].1.sub(&bs[1].1);
                    Some(Core::ConstBig { nat: true, digits: v.to_dec() })
                }
                NatOp::Mul => {
                    let v = bs[0].1.mul(&bs[1].1);
                    Some(Core::ConstBig { nat: true, digits: v.to_dec() })
                }
                NatOp::Div | NatOp::Rem => {
                    if bs[1].1.is_zero() {
                        return None;
                    }
                    let (q, r) = bs[0].1.divmod(&bs[1].1)?;
                    let v = if matches!(nop, NatOp::Div) { q } else { r };
                    Some(Core::ConstBig { nat: true, digits: v.to_dec() })
                }
                NatOp::Cmp => {
                    let c = match bs[0].1.cmp(&bs[1].1) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };
                    Some(Core::Int(c))
                }
                NatOp::FromI64 => {
                    let v = Big::from_i64(words[0]);
                    Some(Core::ConstBig { nat: true, digits: v.to_dec() })
                }
                NatOp::ToI64 => {
                    let v = bs[0].1.to_i64()?;
                    Some(Core::Int(v))
                }
            }
        }
        PrimOp::Int(iop) => {
            let bs = bigs?;
            match iop {
                IntOp::Add => {
                    let v = bs[0].1.add(&bs[1].1);
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::Sub => {
                    let v = bs[0].1.add(&bs[1].1.negate());
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::Mul => {
                    let v = bs[0].1.mul(&bs[1].1);
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::Div | IntOp::Rem => {
                    if bs[1].1.is_zero() {
                        return None;
                    }
                    let (q, r) = bs[0].1.divmod(&bs[1].1)?;
                    let v = if matches!(iop, IntOp::Div) { q } else { r };
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::Neg => {
                    let v = bs[0].1.negate();
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::Cmp => {
                    let c = match bs[0].1.cmp(&bs[1].1) {
                        std::cmp::Ordering::Less => -1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => 1,
                    };
                    Some(Core::Int(c))
                }
                IntOp::FromI64 => {
                    let v = Big::from_i64(words[0]);
                    Some(Core::ConstBig { nat: false, digits: v.to_dec() })
                }
                IntOp::ToI64 => {
                    let v = bs[0].1.to_i64()?;
                    Some(Core::Int(v))
                }
            }
        }
        PrimOp::Conv { from, to, mode } => {
            // Integer-to-integer only: a float operand's word model is
            // the backend's business, and folding it here would invent a
            // representation.
            if matches!(from, Scalar::F32 | Scalar::F64)
                || matches!(to, Scalar::F32 | Scalar::F64)
            {
                return None;
            }
            let x = words[0] as i128;
            let (from_bits, from_signed) = scalar_bits(from);
            let _ = from_bits;
            let xv = if from_signed { x } else { x & ((1i128 << 63) - 1) };
            let raw = xv;
            Some(Core::Int(match mode {
                ConvMode::Total => trunc_word(raw, to),
                ConvMode::Wrapping => trunc_word(raw, to),
                ConvMode::Saturating => {
                    trunc_word(raw.clamp(scalar_min(to), scalar_max(to)), to)
                }
                ConvMode::Checked => {
                    return opt_value(
                        ctors,
                        if raw < scalar_min(to) || raw > scalar_max(to) {
                            None
                        } else {
                            Some(Core::Int(trunc_word(raw, to)))
                        },
                    )
                }
            }))
        }
        _ => None,
    }
}

// ------------------------------------------------------------- purity map

/// Whether a primitive is pure (performed effects are observable order:
/// the buffer operations mutate, everything else is total computation).
fn prim_is_pure(op: PrimOp) -> bool {
    !matches!(op, PrimOp::Bytes(_))
}

/// A term's allocation and demand structure (supercompilation section 3:
/// a memo cell or thunk is never duplicated or reordered). Used as a
/// fusion barrier when it appears inside a callback.
fn has_alloc(e: &Core) -> bool {
    let mut found = false;
    fn go(e: &Core, found: &mut bool) {
        if *found {
            return;
        }
        match e {
            Core::LazyAlloc | Core::LazyInit(_, _) | Core::Demand(_) | Core::Quote { .. }
            | Core::DeclsQuote { .. } => *found = true,
            Core::Prim(PrimOp::Delay) => *found = true,
            Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
            | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::SyntaxLit(_) | Core::StrLit(_)
            | Core::Var(_) => {}
            Core::DictProj(d, _) | Core::Proj(d, _) => go(d, found),
            Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
                for a in args {
                    go(a, found);
                }
            }
            Core::Let(_, v, body) => {
                go(v, found);
                go(body, found);
            }
            Core::LazyInit(_, t) => go(t, found),
            Core::If(c, t, fe) => {
                go(c, found);
                go(&t.body, found);
                go(&fe.body, found);
            }
            Core::Case { scrut, alts } => {
                go(scrut, found);
                for a in alts {
                    go(&a.body, found);
                }
            }
        }
    }
    go(e, &mut found);
    found
}

// ---------------------------------------------------- the driving engine

/// Per-program optimization context.
struct OptCx {
    /// Purity per global id (effect honesty plus a scan for the mutating
    /// primitives and the fault path).
    pure: HashMap<String, bool>,
    ctors: FoldCtors,
    /// Units that must residualize: replaceable dispatch boundaries
    /// retain their selection ability (platform proofs and SIMD
    /// specification section 7).
    opaque: std::collections::HashSet<String>,
    measure: u32,
    fuel: u32,
}

fn scan_calls(e: &Core, f: &mut impl FnMut(&GlobalId)) {
    match e {
        Core::Global(g) => f(g),
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Ctor(_)
        | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_) | Core::StrLit(_)
        | Core::Var(_) => {}
        Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => scan_calls(d, f),
        Core::InstDict(_, args) | Core::App(_, args) => {
            if let Core::App(h, _) = e {
                scan_calls(h, f);
            }
            for a in args {
                scan_calls(a, f);
            }
        }
        Core::Lam { fn_id, env } => {
            f(fn_id);
            for a in env {
                scan_calls(a, f);
            }
        }
        Core::Let(_, v, body) => {
            scan_calls(v, f);
            scan_calls(body, f);
        }
        Core::LazyInit(_, t) => scan_calls(t, f),
        Core::If(c, t, fe) => {
            scan_calls(c, f);
            scan_calls(&t.body, f);
            scan_calls(&fe.body, f);
        }
        Core::Case { scrut, alts } => {
            scan_calls(scrut, f);
            for a in alts {
                scan_calls(&a.body, f);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                scan_calls(h, f);
            }
        }
    }
}

/// Count memo and demand nodes (the recheck pins the residual's memo
/// sharing: supercompilation section 4).
fn memo_count(e: &Core) -> usize {
    let mut n = 0;
    scan_memo(e, &mut n);
    n
}

fn scan_memo(e: &Core, n: &mut usize) {
    match e {
        Core::LazyAlloc | Core::LazyInit(_, _) | Core::Demand(_) => *n += 1,
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
        | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::SyntaxLit(_) | Core::StrLit(_)
        | Core::Var(_) => {}
        Core::DictProj(d, _) | Core::Proj(d, _) => scan_memo(d, n),
        Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
            for a in args {
                scan_memo(a, n);
            }
        }
        Core::Let(_, v, body) => {
            scan_memo(v, n);
            scan_memo(body, n);
        }
        Core::LazyInit(_, t) => scan_memo(t, n),
        Core::If(c, t, fe) => {
            scan_memo(c, n);
            scan_memo(&t.body, n);
            scan_memo(&fe.body, n);
        }
        Core::Case { scrut, alts } => {
            scan_memo(scrut, n);
            for a in alts {
                scan_memo(&a.body, n);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                scan_memo(h, n);
            }
        }
    }
}

/// Every residual decision point keeps the attribute chapter's
/// all-or-nothing rule (attribute annotations specification section 2.1).
fn weights_ok(e: &Core) -> bool {
    let mut ok = true;
    fn go(e: &Core, ok: &mut bool) {
        match e {
            Core::If(c, t, fe) => {
                if t.weight.is_some() != fe.weight.is_some() {
                    *ok = false;
                }
                go(c, ok);
                go(&t.body, ok);
                go(&fe.body, ok);
            }
            Core::Case { scrut, alts } => {
                let any = alts.iter().any(|a| a.weight.is_some());
                let all = alts.iter().all(|a| a.weight.is_some());
                if any && !all {
                    *ok = false;
                }
                go(scrut, ok);
                for a in alts {
                    go(&a.body, ok);
                }
            }
            Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
            | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
            | Core::StrLit(_) | Core::Var(_) => {}
            Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => go(d, ok),
            Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
                for a in args {
                    go(a, ok);
                }
            }
            Core::Let(_, v, body) => {
                go(v, ok);
                go(body, ok);
            }
            Core::LazyInit(_, t) => go(t, ok),
            Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
                for h in holes {
                    go(h, ok);
                }
            }
        }
    }
    go(e, &mut ok);
    ok
}

/// Scope well-formedness of a residual body (every local bound).
fn scope_ok(e: &Core, nparams: usize) -> bool {
    let mut bound: std::collections::HashSet<LocalId> = (0..nparams).map(LocalId).collect();
    fn go(e: &Core, bound: &mut std::collections::HashSet<LocalId>) -> bool {
        match e {
            Core::Var(l) => bound.contains(l),
            Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
            | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
            | Core::StrLit(_) => true,
            Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => go(d, bound),
            Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
                args.iter().all(|a| go(a, bound))
            }
            Core::Let(l, v, body) => {
                if !go(v, bound) {
                    return false;
                }
                bound.insert(*l);
                let ok = go(body, bound);
                bound.remove(l);
                ok
            }
            Core::LazyInit(l, t) => {
                bound.insert(*l);
                let ok = go(t, bound);
                bound.remove(l);
                ok
            }
            Core::If(c, t, fe) => go(c, bound) && go(&t.body, bound) && go(&fe.body, bound),
            Core::Case { scrut, alts } => {
                if !go(scrut, bound) {
                    return false;
                }
                for a in alts {
                    let mut added: Vec<LocalId> = Vec::new();
                    match &a.pat {
                        CPat::Var(l) => {
                            bound.insert(*l);
                            added.push(*l);
                        }
                        CPat::Con { fields, .. } => {
                            for l in fields {
                                bound.insert(*l);
                                added.push(*l);
                            }
                        }
                        CPat::Wild => {}
                    }
                    let ok = go(&a.body, bound);
                    for l in added {
                        bound.remove(&l);
                    }
                    if !ok {
                        return false;
                    }
                }
                true
            }
            Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
                holes.iter().all(|h| go(h, bound))
            }
        }
    }
    go(e, &mut bound)
}

/// Build the purity map: effect honesty gives each unit's own flag, and
/// the closure over calls covers indirect effects. The mutating `Bytes`
/// primitives and the fault path are impure regardless.
fn build_purity(program: &Program) -> HashMap<String, bool> {
    let mut pure = HashMap::new();
    for f in &program.fns {
        let own = if let Some(op) = f.prim {
            prim_is_pure(op)
        } else if f.ctor.is_some() {
            true
        } else {
            f.meta.pure && !body_impure(&f.body)
        };
        pure.insert(f.id.0.clone(), own);
    }
    loop {
        let mut changed = false;
        for f in &program.fns {
            if !pure.get(&f.id.0).copied().unwrap_or(false) {
                continue;
            }
            let mut impure = false;
            scan_calls(&f.body, &mut |g: &GlobalId| {
                if !pure.get(&g.0).copied().unwrap_or(false) {
                    impure = true;
                }
            });
            if impure {
                pure.insert(f.id.0.clone(), false);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    pure
}

fn body_impure(e: &Core) -> bool {
    let mut bad = false;
    fn go(e: &Core, bad: &mut bool) {
        if *bad {
            return;
        }
        match e {
            Core::Fault => *bad = true,
            // A real foreign call is opaque and effectful by contract
            // (foreign function interface specification section 4).
            Core::Foreign(_) => *bad = true,
            Core::Prim(op) => {
                if !prim_is_pure(*op) {
                    *bad = true;
                }
            }
            Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
            | Core::Ctor(_) | Core::LazyAlloc | Core::SyntaxLit(_) | Core::StrLit(_) | Core::Var(_) => {}
            Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => go(d, bad),
            Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
                for a in args {
                    go(a, bad);
                }
            }
            Core::Let(_, v, body) => {
                go(v, bad);
                go(body, bad);
            }
            Core::LazyInit(_, t) => go(t, bad),
            Core::If(c, t, fe) => {
                go(c, bad);
                go(&t.body, bad);
                go(&fe.body, bad);
            }
            Core::Case { scrut, alts } => {
                go(scrut, bad);
                for a in alts {
                    go(&a.body, bad);
                }
            }
            Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
                for h in holes {
                    go(h, bad);
                }
            }
        }
    }
    go(e, &mut bad);
    bad
}

/// A callback argument is usable in a fused or rewritten position when it
/// is an honest pure function and allocates no memo cells (modules and
/// compilation specification section 5; supercompilation section 3).
fn pure_value(
    v: &Core,
    cx: &OptCx,
    defs: &HashMap<String, FnDef>,
    caller: &FnDef,
) -> Result<(), String> {
    match v {
        Core::Global(g) => {
            if !cx.pure.get(&g.0).copied().unwrap_or(false) {
                return Err("effect barrier: the callback performs effects".to_string());
            }
            if let Some(f) = defs.get(&g.0) {
                if has_alloc(&f.body) {
                    return Err("allocation barrier: the callback allocates a memo cell".to_string());
                }
            }
            Ok(())
        }
        Core::Lam { fn_id, env } => {
            if !cx.pure.get(&fn_id.0).copied().unwrap_or(false) {
                return Err("effect barrier: the callback performs effects".to_string());
            }
            if let Some(f) = defs.get(&fn_id.0) {
                if has_alloc(&f.body) {
                    return Err("allocation barrier: the callback allocates a memo cell".to_string());
                }
            }
            for a in env {
                pure_value(a, cx, defs, caller)?;
            }
            Ok(())
        }
        Core::Var(l) => {
            if caller.meta.pure_arrows.iter().any(|i| *i == l.0) {
                Ok(())
            } else {
                Err("effect barrier: the callback's purity is unknown (an effectful argument would change effect order)".to_string())
            }
        }
        _ => Err("effect barrier: the callback is not a known pure function".to_string()),
    }
}

/// Split a producer pipeline into its stages and base producer
/// (`prodMap`/`prodTake` are the prelude's step-form producer library).
fn split_chain(e: &Core) -> (Vec<(bool, Core)>, Core) {
    let mut stages: Vec<(bool, Core)> = Vec::new();
    let mut cur = e.clone();
    loop {
        match &cur {
            Core::App(h, args)
                if matches!(&**h, Core::Global(g) if g.0.ends_with(".prodMap"))
                    && args.len() == 2 =>
            {
                stages.push((true, args[0].clone()));
                cur = args[1].clone();
            }
            Core::App(h, args)
                if matches!(&**h, Core::Global(g) if g.0.ends_with(".prodTake"))
                    && args.len() == 2 =>
            {
                stages.push((false, args[0].clone()));
                cur = args[1].clone();
            }
            _ => break,
        }
    }
    (stages, cur)
}

/// The transformation set of supercompilation section 2 and the weight
/// reconciliation of attribute annotations section 2.4, driven over one
/// residual unit with the profile's fuel and whistle bounds.
struct Driver<'a> {
    cx: &'a OptCx,
    defs: HashMap<String, FnDef>,
    instances: Vec<InstanceDef>,
    rules: Vec<RewriteRule>,
    caller: FnDef,
    stack: Vec<String>,
    sites: Vec<OptSite>,
    fused: Vec<String>,
    new_fns: Vec<FnDef>,
    fuse_counter: usize,
    fresh: usize,
    changed: bool,
    fuel_left: u32,
    fuel_reported: bool,
}

impl Driver<'_> {
    fn site(&mut self, kind: &str, stop: Option<String>) {
        self.sites.push(OptSite {
            fn_id: self.caller.id.0.clone(),
            kind: kind.to_string(),
            stop,
        });
    }

    /// Fuse producer/consumer pipelines over the `Done`/`Skip`/`Yield`
    /// step form into a single state machine (modules and compilation
    /// specification section 6: no intermediate allocation, the
    /// observation model preserved).
    fn fuse(&mut self, e: &Core) -> Option<Core> {
        let (is_fold, head, args) = match e {
            Core::App(h, args) => match &**h {
                Core::Global(g) if g.0.ends_with(".prodFold") && args.len() == 3 => {
                    (true, g.0.clone(), args.clone())
                }
                Core::Global(g) if g.0.ends_with(".prodMap") && args.len() == 2 => {
                    (false, g.0.clone(), args.clone())
                }
                _ => return None,
            },
            _ => return None,
        };
        let (stages, base) = split_chain(if is_fold { &args[2] } else { &args[1] });
        let maps: Vec<Core> = stages.iter().filter(|(m, _)| *m).map(|(_, f)| f.clone()).collect();
        if maps.len() < 2 && !(is_fold && maps.len() == 1) {
            return None;
        }
        if self.fuel_left == 0 {
            self.note_fuel();
            return None;
        }
        // Barriers: composition over the pull-based step form runs each
        // callback at the same point of the same element's processing as
        // the unfused pipeline, so effects keep their order; what fusion
        // must not move is memo and demand structure (supercompilation
        // section 3). An eager pipeline's pure-justification rule is the
        // one that refuses effectful callbacks (modules section 5).
        let callbacks: Vec<&Core> = if is_fold {
            let mut v = vec![&args[1]];
            v.extend(maps.iter());
            v
        } else {
            maps.iter().collect()
        };
        for c in callbacks {
            let blocked = match c {
                Core::Global(g) => self
                    .defs
                    .get(&g.0)
                    .map(|f| has_alloc(&f.body))
                    .unwrap_or(false),
                Core::Lam { fn_id, env } => {
                    self.defs
                        .get(&fn_id.0)
                        .map(|f| has_alloc(&f.body))
                        .unwrap_or(false)
                        || env.iter().any(has_alloc)
                }
                other => has_alloc(other),
            };
            if blocked {
                self.site(
                    "fusion",
                    Some("allocation barrier: the callback allocates a memo cell, which fusion would reorder relative to its demand".to_string()),
                );
                return None;
            }
        }
        // Composed callback: `\acc x -> k acc (m1 (m2 ... (mn x)))` for a
        // fold, `\x -> m1 (m2 ... (mn x))` for a map.
        let env: Vec<Core> = if is_fold {
            let mut v = vec![args[1].clone()];
            v.extend(maps.iter().cloned());
            v
        } else {
            maps.clone()
        };
        let nmaps = maps.len();
        let gid = GlobalId(format!("$fuse.{}", self.fuse_counter));
        self.fuse_counter += 1;
        let (params, body) = if is_fold {
            let acc = LocalId(nmaps + 1);
            let x = LocalId(nmaps + 2);
            // k is env slot 0; map i is slot i + 1.
            let mut inner = Core::Var(x);
            for i in (1..=nmaps).rev() {
                inner = Core::App(Box::new(Core::Var(LocalId(i))), vec![inner]);
            }
            let body = Core::App(Box::new(Core::Var(LocalId(0))), vec![Core::Var(acc), inner]);
            let params: Vec<String> = std::iter::once("k".to_string())
                .chain((1..=nmaps).map(|i| format!("m{i}")))
                .chain(["acc".to_string(), "x".to_string()])
                .collect();
            (params, body)
        } else {
            let x = LocalId(nmaps);
            let mut inner = Core::Var(x);
            for i in (1..nmaps).rev() {
                inner = Core::App(Box::new(Core::Var(LocalId(i))), vec![inner]);
            }
            let params: Vec<String> = (1..=nmaps)
                .map(|i| format!("m{i}"))
                .chain(["x".to_string()])
                .collect();
            (params, inner)
        };
        let nparams = params.len();
        let body = renumber_core(&body, nparams);
        self.new_fns.push(FnDef {
            id: gid.clone(),
            params,
            body,
            prim: None,
            ctor: None,
            hints: FnHints::default(),
            meta: FnMeta { pure: true, ..FnMeta::default() },
        });
        // Rebuild the producer: the map stages collapse into the composed
        // callback, the take stages keep their order over the base.
        let mut producer = base;
        for (is_map, arg) in stages.iter().rev() {
            if *is_map {
                continue;
            }
            producer = Core::App(
                Box::new(Core::Global(GlobalId(
                    self.defs
                        .keys()
                        .find(|k| k.ends_with(".prodTake"))
                        .cloned()
                        .unwrap_or_else(|| "Prelude.prodTake".to_string()),
                ))),
                vec![arg.clone(), producer],
            );
        }
        self.changed = true;
        self.fuel_left -= 1;
        self.fused.push(format!(
            "{}: fused {} map stage(s) into the {} state machine (Done/Skip/Yield); eliminated the intermediate step allocation",
            self.caller.id.0,
            nmaps,
            if is_fold { "fold loop" } else { "map" }
        ));
        self.site("fusion", None);
        let lam = Core::Lam { fn_id: gid, env };
        Some(if is_fold {
            Core::App(
                Box::new(Core::Global(GlobalId(head))),
                vec![args[0].clone(), lam, producer],
            )
        } else {
            Core::App(Box::new(Core::Global(GlobalId(head))), vec![lam, producer])
        })
    }

    /// Dead alternatives in the default alternatives of a decision (the
    /// branch that decided the scrutinee's negation): dropping an
    /// impossible alternative is conditioning on its negation, and the
    /// surviving weights are NOT rescaled (attribute annotations
    /// specification section 2.4).
    fn dead_alts(&mut self, e: &mut Core) {
        let mut syn: HashMap<LocalId, LocalId> = HashMap::new();
        self.dead_alts_in(e, &mut syn);
    }

    fn dead_alts_in(&mut self, e: &mut Core, syn: &mut HashMap<LocalId, LocalId>) {
        match e {
            Core::Let(l, v, _) => {
                if let Core::Var(src) = &**v {
                    let root = syn.get(src).copied().unwrap_or(*src);
                    syn.insert(*l, root);
                }
            }
            Core::Case { scrut, alts } => {
                if let Core::Var(v) = &**scrut {
                    let root = syn.get(v).copied().unwrap_or(*v);
                    // The default alternatives share one nested decision
                    // on the scrutinee; the explicit alternatives are the
                    // constructors the default excludes.
                    let mut shared: Vec<usize> = Vec::new();
                    let mut neg: Vec<usize> = Vec::new();
                    for (i, a) in alts.iter().enumerate() {
                        if contains_case_on(&a.body, syn, root) {
                            shared.push(i);
                        } else if let CPat::Con { tag, .. } = &a.pat {
                            neg.push(*tag);
                        }
                    }
                    if !shared.is_empty() && !neg.is_empty() {
                        for i in shared {
                            drop_dead(&mut alts[i].body, root, syn, &neg, self);
                        }
                    }
                }
            }
            _ => {}
        }
        let mut kids = child_slots(e);
        for k in kids.iter_mut() {
            self.dead_alts_in(k, syn);
        }
        let _ = &mut kids;
    }

    /// Decision merging: two decisions on the same scrutinee merge into
    /// one, reconciling the weight spaces by the rescale formula and a
    /// greatest-common-divisor reduction (attribute annotations
    /// specification section 2.4). The pattern compiler expands a default
    /// alternative into one alternative per uncovered constructor, all
    /// sharing the default's body and weight: those are the alternatives
    /// the merged decision absorbs.
    fn merge_cases(&mut self, e: &mut Core) {
        let mut syn: HashMap<LocalId, LocalId> = HashMap::new();
        self.merge_cases_in(e, &mut syn);
    }

    fn merge_cases_in(&mut self, e: &mut Core, syn: &mut HashMap<LocalId, LocalId>) {
        if let Core::Let(l, v, _) = e {
            if let Core::Var(src) = &**v {
                let root = syn.get(src).copied().unwrap_or(*src);
                syn.insert(*l, root);
            }
        }
        if let Some(merged) = try_merge_case(e, syn) {
            *e = merged;
            self.changed = true;
            self.site("case-merge", None);
        }
        let mut kids = child_slots(e);
        for k in kids.iter_mut() {
            self.merge_cases_in(k, syn);
        }
        let _ = &mut kids;
    }

    /// Decision-of-decision independence: a decision on a value produced
    /// by another decision is pushed inward; every duplicated decision
    /// keeps its original weights.
    fn push_cases(&mut self, e: &mut Core) {
        if let Core::Case { scrut, alts } = e {
            if let Core::If(c, t, fe) = &**scrut {
                let mk = |branch: &Core| -> Core {
                    Core::Case {
                        scrut: Box::new(branch.clone()),
                        alts: alts
                            .iter()
                            .map(|a| CAlt {
                                pat: a.pat.clone(),
                                weight: a.weight,
                                body: a.body.clone(),
                            })
                            .collect(),
                    }
                };
                let new = Core::If(
                    c.clone(),
                    CBranch { weight: t.weight, body: Box::new(mk(&t.body)) },
                    CBranch { weight: fe.weight, body: Box::new(mk(&fe.body)) },
                );
                *e = new;
                self.changed = true;
                self.site("case-push", None);
            }
        }
        let mut kids = child_slots(e);
        for k in kids.iter_mut() {
            self.push_cases(k);
        }
        let _ = &mut kids;
    }

    /// One driving pass over a term: rewrite rules and fusion see the
    /// pipeline as written (before its children are driven), then the
    /// children, then the local managed transformations.
    fn drive(&mut self, e: &mut Core) {
        if self.try_rules(e) {
            let mut kids = child_slots(e);
            for k in kids.iter_mut() {
                self.drive(k);
            }
            return;
        }
        if let Some(fused) = self.fuse(e) {
            *e = fused;
            self.drive(e);
            return;
        }
        let mut kids = child_slots(e);
        for k in kids.iter_mut() {
            self.drive(k);
        }
        drop(kids);
        self.node(e);
    }

    /// Note fuel exhaustion once per residual unit.
    fn note_fuel(&mut self) {
        if !self.fuel_reported {
            self.fuel_reported = true;
            self.site("unfold", Some("fuel-exhausted".to_string()));
        }
    }

    /// Rewrite-rule application at one node (modules and compilation
    /// specification section 5): a rule fires where its pattern matches
    /// and its barriers hold; the template's weights replace the target's
    /// entirely (attribute annotations specification section 2.4).
    fn try_rules(&mut self, e: &mut Core) -> bool {
        let rules: Vec<RewriteRule> = self.rules.clone();
        for rule in &rules {
            let holes: std::collections::HashSet<LocalId> =
                rule.holes.iter().copied().collect();
            let mut binds: HashMap<LocalId, Core> = HashMap::new();
            if !match_pat(&rule.pattern, &holes, e, &mut binds) {
                continue;
            }
            if self.fuel_left == 0 {
                self.note_fuel();
                return false;
            }
            // A `using` binder the pattern does not bind comes from the
            // firing unit's capability parameters of that class.
            let mut outside: Option<String> = None;
            for (hole, class) in &rule.dict_holes {
                if binds.contains_key(hole) {
                    continue;
                }
                match self.caller.meta.dicts.iter().find(|(i, c)| c == class) {
                    Some((i, _)) => {
                        binds.insert(*hole, Core::Var(LocalId(*i)));
                    }
                    None => {
                        outside = Some(format!(
                            "capability scope: the rule applies only where a `{class}` capability is valid"
                        ));
                        break;
                    }
                }
            }
            if let Some(stop) = outside {
                self.site(&format!("rewrite:{}", rule.name), Some(stop));
                continue;
            }
            if let Err(stop) = self.rule_barriers(rule, &binds) {
                self.site(&format!("rewrite:{}", rule.name), Some(stop));
                continue;
            }
            let mut next = self.fresh;
            let inst = instantiate(&rule.template, &holes, &binds, &mut next);
            self.fresh = next.max(self.fresh);
            *e = inst;
            self.changed = true;
            self.fuel_left -= 1;
            self.site(&format!("rewrite:{}", rule.name), None);
            return true;
        }
        false
    }

    fn node(&mut self, e: &mut Core) {
        // Constant folding of total primitives on known arguments.
        if let Core::App(h, args) = e {
            if let Core::Prim(op) = &**h {
                let ctors = FoldCtors {
                    none: self.cx.ctors.none.clone(),
                    some: self.cx.ctors.some.clone(),
                };
                if let Some(v) = fold_prim(*op, args, &ctors) {
                    if !self.spend_fuel() {
                        return;
                    }
                    *e = v;
                    self.changed = true;
                    self.site("const-fold", None);
                    return;
                }
            }
        }
        // Case of a known constructor (its weights are discarded: the
        // decision point is eliminated).
        if let Core::Case { scrut, alts } = e {
            let (tag, fields) = ctor_shape(&**scrut, &self.defs);
            if let Some(tag) = tag {
                let mut pick: Option<usize> = None;
                let mut fallback: Option<usize> = None;
                for (i, a) in alts.iter().enumerate() {
                    match &a.pat {
                        CPat::Con { tag: t, .. } if *t == tag => pick = Some(i),
                        CPat::Var(_) | CPat::Wild => fallback = Some(i),
                        _ => {}
                    }
                }
                if let Some(i) = pick.or(fallback) {
                    if !self.spend_fuel() {
                        return;
                    }
                    let alt = alts[i].clone();
                    let mut out = *alt.body.clone();
                    match &alt.pat {
                        CPat::Con { fields: fs, .. } => {
                            for (slot, f) in fs.iter().enumerate() {
                                if let Some(val) = fields.get(slot) {
                                    out = Core::Let(*f, Box::new(val.clone()), Box::new(out));
                                }
                            }
                        }
                        CPat::Var(l) => {
                            out = Core::Let(*l, Box::new((**scrut).clone()), Box::new(out));
                        }
                        CPat::Wild => {}
                    }
                    *e = out;
                    self.changed = true;
                    self.site("case-fold", None);
                    return;
                }
            }
        }
        // Dictionary and effect-static specialization: a call through a
        // statically known dictionary becomes a direct call.
        if let Core::App(h, args) = e {
            if let Core::DictProj(d, idx) = &**h {
                if let Core::InstDict(id, subs) = &**d {
                    if let Some(inst_def) = self.caller_program_instance(id) {
                        if let Some(mid) = inst_def.methods.get(*idx) {
                            let want = self.defs.get(&mid.0).map(|f| f.params.len());
                            if want == Some(subs.len() + args.len()) && self.spend_fuel() {
                                let mut new_args = subs.clone();
                                new_args.extend(args.iter().cloned());
                                *e = Core::App(Box::new(Core::Global(mid.clone())), new_args);
                                self.changed = true;
                                self.site("dictionary-specialize", None);
                                return;
                            }
                        }
                    }
                }
            }
        }
        // Known-application unfolding, bounded by fuel and the whistle.
        if let Core::App(h, args) = e {
            if let Core::Global(g) = &**h {
                if let Some(new) = self.try_unfold(g, args) {
                    *e = new;
                    return;
                }
            }
        }
    }

    /// One unit of driving fuel, or a noted exhaustion.
    fn spend_fuel(&mut self) -> bool {
        if self.fuel_left == 0 {
            self.note_fuel();
            return false;
        }
        self.fuel_left -= 1;
        true
    }

    fn caller_program_instance(&self, id: &GlobalId) -> Option<InstanceDef> {
        self.instances
            .iter()
            .find(|i| i.id.0 == id.0)
            .cloned()
    }

    /// The barriers of supercompilation section 3 for one rule firing.
    fn rule_barriers(&self, rule: &RewriteRule, binds: &HashMap<LocalId, Core>) -> Result<(), String> {
        for (hole, class) in &rule.dict_holes {
            let v = binds.get(hole).ok_or_else(|| {
                "capability scope: the capability is not in scope".to_string()
            })?;
            match v {
                Core::Var(l)
                    if self.caller.meta.dicts.iter().any(|(i, c)| *i == l.0 && c == class) => {}
                _ => {
                    return Err(format!(
                        "capability scope: the rule applies only where a `{class}` capability is valid"
                    ))
                }
            }
        }
        match &rule.cond {
            RequireCond::Always => {}
            RequireCond::Never => {
                return Err("applicability condition: the `require` clause is false here".to_string())
            }
            RequireCond::Unknown => {
                return Err(
                    "applicability condition: the `require` clause is not statically decidable here"
                        .to_string(),
                )
            }
            RequireCond::CapabilityInScope { hole, class } => {
                let v = binds
                    .get(hole)
                    .ok_or_else(|| "capability scope: the capability is not in scope".to_string())?;
                match v {
                    Core::Var(l)
                        if self
                            .caller
                            .meta
                            .dicts
                            .iter()
                            .any(|(i, c)| *i == l.0 && c == class) => {}
                    _ => {
                        return Err(format!(
                            "capability scope: the rule applies only where a `{class}` capability is valid"
                        ))
                    }
                }
            }
        }
        for hole in &rule.pure_holes {
            if let Some(v) = binds.get(hole) {
                pure_value(v, self.cx, &self.defs, &self.caller)?;
            }
        }
        Ok(())
    }

    fn try_unfold(&mut self, g: &GlobalId, args: &[Core]) -> Option<Core> {
        if self.cx.opaque.contains(&g.0) {
            self.site("unfold", Some("opaque boundary: a replaceable dispatch boundary residualizes".to_string()));
            return None;
        }
        let f = self.defs.get(&g.0)?.clone();
        if f.prim.is_some() || f.ctor.is_some() || args.len() != f.params.len() {
            return None;
        }
        if self.stack.iter().any(|s| s == &g.0) {
            self.site("unfold", Some("whistle: a repeated configuration folds to a residual call".to_string()));
            return None;
        }
        if self.fuel_left == 0 {
            self.note_fuel();
            return None;
        }
        // Linear values are never duplicated or dropped; a configuration
        // that would require either generalizes instead.
        let mut binds: HashMap<LocalId, Core> = HashMap::new();
        for (i, arg) in args.iter().enumerate() {
            let l = LocalId(i);
            let (min, max) = use_range(&f.body, l);
            let arg_is_linear = matches!(arg, Core::Var(v)
                if self.caller.meta.linear.iter().any(|(j, _)| *j == v.0));
            let param_is_linear = self.caller_callee_linear(&f, i);
            if arg_is_linear || param_is_linear {
                if min != 1 || max != 1 {
                    self.site(
                        "unfold",
                        Some("linear value: generalizing instead of duplicating or dropping a %1/%aff value".to_string()),
                    );
                    return None;
                }
                binds.insert(l, arg.clone());
                continue;
            }
            let atom = matches!(
                arg,
                Core::Var(_) | Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_)
                    | Core::Unit | Core::Global(_) | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_)
            );
            if atom {
                binds.insert(l, arg.clone());
            } else {
                // Let-sharing keeps the strict argument order of the
                // original call (effect order is a barrier of its own).
                let fresh_v = LocalId(self.fresh);
                self.fresh += 1;
                binds.insert(l, Core::Var(fresh_v));
                // Enclose after building the body (below).
                binds.insert(LocalId(0x8000_0000 + i), Core::Var(fresh_v));
                binds.insert(LocalId(0x4000_0000 + i), arg.clone());
            }
        }
        let holes: std::collections::HashSet<LocalId> = (0..args.len()).map(LocalId).collect();
        let mut next = self.fresh;
        let mut body = instantiate(&f.body, &holes, &binds, &mut next);
        self.fresh = next;
        for i in (0..args.len()).rev() {
            let fresh_v = LocalId(0x8000_0000 + i);
            let arg = LocalId(0x4000_0000 + i);
            if let (Some(Core::Var(v)), Some(a)) = (binds.get(&fresh_v), binds.get(&arg)) {
                body = Core::Let(*v, Box::new(a.clone()), Box::new(body));
            }
        }
        self.fuel_left -= 1;
        self.changed = true;
        self.stack.push(g.0.clone());
        self.node(&mut body);
        self.stack.pop();
        self.site("unfold", None);
        Some(body)
    }

    fn caller_callee_linear(&self, f: &FnDef, i: usize) -> bool {
        f.meta.linear.iter().any(|(j, _)| *j == i)
    }
}

/// Dead-alternative pruning inside `e`: the nested decision on `v` drops
/// the alternatives the enclosing decision excluded (`impossible`),
/// conditioning on their negation without rescaling the survivors.
fn drop_dead(
    e: &mut Core,
    v: LocalId,
    syn: &mut HashMap<LocalId, LocalId>,
    impossible: &[usize],
    drv: &mut Driver,
) {
    match e {
        Core::Let(l, src, _) => {
            if let Core::Var(s) = &**src {
                let root = syn.get(s).copied().unwrap_or(*s);
                syn.insert(*l, root);
            }
        }
        Core::Case { scrut, alts } => {
            if let Core::Var(sv) = &**scrut {
                let root = syn.get(sv).copied().unwrap_or(*sv);
                if root == v {
                    let before = alts.len();
                    alts.retain(|a| match &a.pat {
                        CPat::Con { tag, .. } => !impossible.contains(tag),
                        _ => true,
                    });
                    if alts.len() != before {
                        drv.changed = true;
                        drv.site("dead-alternative", None);
                    }
                }
            }
        }
        _ => {}
    }
    let mut kids = child_slots(e);
    for k in kids.iter_mut() {
        drop_dead(k, v, syn, impossible, drv);
    }
    let _ = &mut kids;
}

/// Mutable views of a term's child positions (for bottom-up rewrites).
fn child_slots(e: &mut Core) -> Vec<&mut Core> {
    let mut out: Vec<&mut Core> = Vec::new();
    match e {
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
        | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
        | Core::StrLit(_) | Core::Var(_) => {}
        Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => out.push(&mut **d),
        Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
            for a in args {
                out.push(a);
            }
        }
        Core::Let(_, v, body) => {
            out.push(&mut **v);
            out.push(&mut **body);
        }
        Core::LazyInit(_, t) => out.push(&mut **t),
        Core::If(c, t, fe) => {
            out.push(&mut **c);
            out.push(&mut *t.body);
            out.push(&mut *fe.body);
        }
        Core::Case { scrut, alts } => {
            out.push(&mut **scrut);
            for a in alts {
                out.push(&mut *a.body);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                out.push(h);
            }
        }
    }
    out
}

/// The constructor a scrutinee applies, if it is a known one.
fn ctor_shape(e: &Core, defs: &HashMap<String, FnDef>) -> (Option<usize>, Vec<Core>) {
    match e {
        Core::Ctor(g) => {
            let tag = defs.get(&g.0).and_then(|f| f.ctor).map(|(t, _)| t);
            (tag, Vec::new())
        }
        Core::App(h, args) => match &**h {
            Core::Ctor(g) => {
                let tag = defs.get(&g.0).and_then(|f| f.ctor).map(|(t, _)| t);
                (tag, args.clone())
            }
            _ => (None, Vec::new()),
        },
        _ => (None, Vec::new()),
    }
}

/// Alpha-equivalence of two terms: the pattern compiler clones a default
/// alternative's body once per uncovered constructor, renaming its local
/// binders.
fn alpha_eq(a: &Core, b: &Core) -> bool {
    fn go(a: &Core, b: &Core, map: &mut HashMap<LocalId, LocalId>) -> bool {
        match (a, b) {
            (Core::Var(x), Core::Var(y)) => match map.get(x) {
                Some(q) => q == y,
                None => x == y,
            },
            (Core::Int(x), Core::Int(y)) => x == y,
            (
                Core::ConstBig { nat: n1, digits: d1 },
                Core::ConstBig { nat: n2, digits: d2 },
            ) => n1 == n2 && d1 == d2,
            (Core::Bool(x), Core::Bool(y)) => x == y,
            (Core::Unit, Core::Unit) | (Core::Fault, Core::Fault) | (Core::LazyAlloc, Core::LazyAlloc) => true,
            (Core::Global(x), Core::Global(y)) | (Core::Ctor(x), Core::Ctor(y)) => x == y,
            (Core::Prim(x), Core::Prim(y)) => x == y,
            (Core::Foreign(x), Core::Foreign(y)) => x == y,
            (Core::DictProj(d1, i1), Core::DictProj(d2, i2)) => i1 == i2 && go(d1, d2, map),
            (Core::Proj(d1, i1), Core::Proj(d2, i2)) => i1 == i2 && go(d1, d2, map),
            (Core::InstDict(g1, a1), Core::InstDict(g2, a2)) => {
                g1 == g2 && a1.len() == a2.len() && a1.iter().zip(a2).all(|(x, y)| go(x, y, map))
            }
            (Core::App(f1, a1), Core::App(f2, a2)) => {
                a1.len() == a2.len()
                    && go(f1, f2, map)
                    && a1.iter().zip(a2).all(|(x, y)| go(x, y, map))
            }
            (Core::Lam { fn_id: g1, env: e1 }, Core::Lam { fn_id: g2, env: e2 }) => {
                g1 == g2 && e1.len() == e2.len() && e1.iter().zip(e2).all(|(x, y)| go(x, y, map))
            }
            (Core::Let(l1, v1, b1), Core::Let(l2, v2, b2)) => {
                if !go(v1, v2, map) {
                    return false;
                }
                map.insert(*l1, *l2);
                let ok = go(b1, b2, map);
                map.remove(l1);
                ok
            }
            (Core::LazyInit(l1, t1), Core::LazyInit(l2, t2)) => {
                map.insert(*l1, *l2);
                let ok = go(t1, t2, map);
                map.remove(l1);
                ok
            }
            (Core::If(c1, t1, f1), Core::If(c2, t2, f2)) => {
                t1.weight == t2.weight
                    && f1.weight == f2.weight
                    && go(c1, c2, map)
                    && go(&t1.body, &t2.body, map)
                    && go(&f1.body, &f2.body, map)
            }
            (Core::Case { scrut: s1, alts: a1 }, Core::Case { scrut: s2, alts: a2 }) => {
                if !go(s1, s2, map) || a1.len() != a2.len() {
                    return false;
                }
                for (x, y) in a1.iter().zip(a2) {
                    if x.weight != y.weight {
                        return false;
                    }
                    let mut bound: Vec<LocalId> = Vec::new();
                    let ok = match (&x.pat, &y.pat) {
                        (CPat::Wild, CPat::Wild) => true,
                        (CPat::Var(p), CPat::Var(q)) => {
                            map.insert(*p, *q);
                            bound.push(*p);
                            true
                        }
                        (CPat::Con { tag: t1, fields: f1 }, CPat::Con { tag: t2, fields: f2 })
                            if t1 == t2 && f1.len() == f2.len() =>
                        {
                            for (p, q) in f1.iter().zip(f2) {
                                map.insert(*p, *q);
                                bound.push(*p);
                            }
                            true
                        }
                        _ => false,
                    };
                    let ok = ok && go(&x.body, &y.body, map);
                    for p in bound {
                        map.remove(&p);
                    }
                    if !ok {
                        return false;
                    }
                }
                true
            }
            (Core::StrLit(s1), Core::StrLit(s2)) => s1 == s2,
            (Core::SyntaxLit(t1), Core::SyntaxLit(t2)) => format!("{t1:?}") == format!("{t2:?}"),
            _ => false,
        }
    }
    go(a, b, &mut HashMap::new())
}

/// The nested decision an outer alternative's body holds on the same
/// scrutinee, through the compiler's synonym `let`s only: a default
/// alternative is `let y = x in case y of ...` up to renaming.
fn shared_inner_case<'a>(
    body: &'a Core,
    syn: &HashMap<LocalId, LocalId>,
    root: LocalId,
) -> Option<(LocalId, &'a Vec<CAlt<Core>>)> {
    let (sub, case): (LocalId, &Core) = match body {
        Core::Let(l, src, rest) => match &**src {
            Core::Var(s) if syn.get(s).copied().unwrap_or(*s) == root => (*l, &**rest),
            _ => return None,
        },
        _ => (LocalId(usize::MAX), body),
    };
    match case {
        Core::Case { scrut: s2, alts: inner } => {
            let w = match &**s2 {
                Core::Var(w) => *w,
                _ => return None,
            };
            let resolved = syn.get(&w).copied().unwrap_or(w);
            if w == sub || resolved == root {
                Some((sub, inner))
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Does the term contain a decision on the scrutinee's root synonym?
fn contains_case_on(e: &Core, syn: &HashMap<LocalId, LocalId>, root: LocalId) -> bool {
    let mut syn = syn.clone();
    let mut found = false;
    fn go(e: &Core, syn: &mut HashMap<LocalId, LocalId>, root: LocalId, found: &mut bool) {
        if *found {
            return;
        }
        match e {
            Core::Let(l, src, _) => {
                if let Core::Var(s) = &**src {
                    let r = syn.get(s).copied().unwrap_or(*s);
                    syn.insert(*l, r);
                }
            }
            Core::Case { scrut, .. } => {
                if let Core::Var(sv) = &**scrut {
                    if syn.get(sv).copied().unwrap_or(*sv) == root {
                        *found = true;
                        return;
                    }
                }
            }
            _ => {}
        }
        for k in child_slots_imm(e) {
            go(k, syn, root, found);
        }
    }
    go(e, &mut syn, root, &mut found);
    found
}

/// Merge a decision with the nested decision its default alternatives
/// share (attribute annotations specification section 2.4): for each
/// outer alternative of weight `w_i` the merged weight is `w_i * S` (S
/// the sum of the inner weights), for each inner alternative of weight
/// `u_j` under the outer default `w_d` it is `w_d * u_j`, and the merged
/// set is divided by its greatest common divisor. A partially annotated
/// decision merges with no weights at all.
fn try_merge_case(e: &Core, syn: &HashMap<LocalId, LocalId>) -> Option<Core> {
    let (scrut, alts) = match e {
        Core::Case { scrut, alts } => (scrut.clone(), alts.clone()),
        _ => return None,
    };
    let v = match &*scrut {
        Core::Var(v) => *v,
        _ => return None,
    };
    let root = syn.get(&v).copied().unwrap_or(v);
    // The shared nested decision: every alternative whose body is the
    // same case on the scrutinee is a default alternative.
    let mut default_idx: Vec<usize> = Vec::new();
    let mut shared: Option<(LocalId, &Vec<CAlt<Core>>)> = None;
    for (i, a) in alts.iter().enumerate() {
        if let Some(inner) = shared_inner_case(&a.body, syn, root) {
            match &shared {
                None => shared = Some(inner),
                Some((_sub, alts2)) => {
                    let same = alts2.len() == inner.1.len()
                        && alts2.iter().zip(inner.1).all(|(x, y)| {
                            x.weight == y.weight
                                && pat_eq(&x.pat, &y.pat)
                                && alpha_eq(&x.body, &y.body)
                        });
                    if !same {
                        return None;
                    }
                }
            }
            default_idx.push(i);
        }
    }
    let (sub, inner) = shared?;
    if default_idx.is_empty() || default_idx.len() == alts.len() {
        return None;
    }
    // The inner decision must cover the default alternatives'
    // constructors (first-match semantics would change otherwise). Its
    // own alternatives outside that set are impossible under the default
    // and are conditioned away without rescaling (section 2.4's
    // dead-alternative rule).
    let mut inner_tags: Vec<usize> = Vec::new();
    for a in inner.iter() {
        match &a.pat {
            CPat::Con { tag, .. } => {
                if inner_tags.contains(tag) {
                    return None;
                }
                inner_tags.push(*tag);
            }
            _ => return None,
        }
    }
    let mut default_tags: Vec<usize> = Vec::new();
    let mut other_tags: Vec<usize> = Vec::new();
    for (i, a) in alts.iter().enumerate() {
        if let CPat::Con { tag, .. } = &a.pat {
            if default_idx.contains(&i) {
                default_tags.push(*tag);
            } else {
                other_tags.push(*tag);
            }
        }
    }
    default_tags.sort();
    if !default_tags.iter().all(|t| inner_tags.contains(t)) {
        return None;
    }
    let kept: Vec<usize> = (0..inner.len())
        .filter(|j| match &inner[*j].pat {
            CPat::Con { tag, .. } => default_tags.contains(tag),
            _ => false,
        })
        .collect();
    // Weights: all-or-nothing on both decisions, and the default
    // alternatives must agree on their weight.
    let w_d = alts[default_idx[0]].weight;
    if default_idx.iter().any(|i| alts[*i].weight != w_d) {
        return None;
    }
    let outer_w: Vec<Option<u32>> = alts.iter().map(|a| a.weight).collect();
    let inner_w: Vec<Option<u32>> = kept.iter().map(|j| inner[*j].weight).collect();
    let outer_all = outer_w.iter().all(|w| w.is_some()) && outer_w.iter().any(|w| w.is_some());
    let inner_all = inner_w.iter().all(|w| w.is_some()) && inner_w.iter().any(|w| w.is_some());
    let mut merged_w: Vec<Option<u32>> = Vec::new();
    if outer_all && inner_all {
        let s: u128 = inner_w.iter().map(|w| w.unwrap() as u128).sum();
        let wd = w_d.unwrap() as u128;
        let mut vals: Vec<u128> = Vec::new();
        for (i, w) in outer_w.iter().enumerate() {
            if !default_idx.contains(&i) {
                vals.push(w.unwrap() as u128 * s);
            }
        }
        for w in &inner_w {
            vals.push(wd * w.unwrap() as u128);
        }
        let g = vals.iter().copied().reduce(|a, b| gcd(a, b)).unwrap_or(1).max(1);
        merged_w = vals.iter().map(|v| Some((v / g) as u32)).collect();
    }
    // Assemble: the outer alternatives keep their bodies, the inner
    // alternatives arrive from the shared default body.
    let mut out: Vec<CAlt<Core>> = Vec::new();
    let mut wi = 0;
    for (i, a) in alts.iter().enumerate() {
        if default_idx.contains(&i) {
            continue;
        }
        out.push(CAlt {
            pat: a.pat.clone(),
            weight: merged_w.get(wi).copied().flatten(),
            body: a.body.clone(),
        });
        wi += 1;
    }
    for j in kept {
        let a = &inner[j];
        let body = if sub != LocalId(usize::MAX) {
            Box::new(subst_var(&a.body, sub, &Core::Var(v)))
        } else {
            a.body.clone()
        };
        out.push(CAlt {
            pat: a.pat.clone(),
            weight: merged_w.get(wi).copied().flatten(),
            body,
        });
        wi += 1;
    }
    Some(Core::Case { scrut, alts: out })
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Substitute one local throughout a term.
fn subst_var(e: &Core, from: LocalId, to: &Core) -> Core {
    let mut out = e.clone();
    subst_in_place(&mut out, from, to);
    out
}

fn subst_in_place(e: &mut Core, from: LocalId, to: &Core) {
    for slot in child_slots(e) {
        subst_in_place(slot, from, to);
    }
    if let Core::Var(l) = e {
        if *l == from {
            *e = to.clone();
        }
    }
}

/// The residual program recheck (supercompilation and JIT specification
/// section 4): every candidate passes before it is installed; a failing
/// candidate is discarded and the generic form stays.
fn verify_residual(orig: &FnDef, body: &Core) -> Result<(), String> {
    if !scope_ok(body, orig.params.len()) {
        return Err("recheck failed: an unbound local in the residual".to_string());
    }
    for (i, _) in &orig.meta.linear {
        let before = use_range(&orig.body, LocalId(*i));
        let after = use_range(body, LocalId(*i));
        if before != after {
            return Err(format!(
                "recheck failed: the usage of linear parameter {i} changed ({before:?} to {after:?})"
            ));
        }
    }
    if memo_count(&orig.body) != memo_count(body) {
        return Err("recheck failed: memo/demand sharing changed".to_string());
    }
    if !weights_ok(body) {
        return Err("recheck failed: a decision point lost its all-or-nothing weights".to_string());
    }
    Ok(())
}

/// Specialize a checked program (supercompilation and JIT specification
/// sections 2-5 and 8-9). Returns the residual program and the compiler
/// report; the residual's contract identity is the generic form's.
pub fn optimize(program: &Program, profile: &TargetProfile) -> Result<(Program, OptReport)> {
    let pure = build_purity(program);
    let mut defs: HashMap<String, FnDef> = HashMap::new();
    for f in &program.fns {
        defs.insert(f.id.0.clone(), f.clone());
    }
    let mut ctor_none = None;
    let mut ctor_some = None;
    for f in &program.fns {
        if f.ctor == Some((0, 0)) && f.id.0.ends_with(".None") {
            ctor_none = Some(f.id.clone());
        }
        if f.ctor == Some((1, 1)) && f.id.0.ends_with(".Some") {
            ctor_some = Some(f.id.clone());
        }
    }
    let cx = OptCx {
        pure,
        ctors: FoldCtors { none: ctor_none, some: ctor_some },
        opaque: std::collections::HashSet::new(),
        measure: profile.measure,
        fuel: profile.fuel,
    };
    let mut out = program.clone();
    let mut report = OptReport::default();
    let mut new_fns: Vec<FnDef> = Vec::new();
    for fi in 0..program.fns.len() {
        let orig = program.fns[fi].clone();
        if orig.prim.is_some() || orig.ctor.is_some() {
            continue;
        }
        let mut drv = Driver {
            cx: &cx,
            defs: defs.clone(),
            instances: program.instances.clone(),
            rules: program.rewrites.clone(),
            caller: orig.clone(),
            stack: Vec::new(),
            sites: Vec::new(),
            fused: Vec::new(),
            new_fns: Vec::new(),
            fuse_counter: 0,
            fresh: max_local(&orig.body) + 1,
            changed: false,
            fuel_left: profile.fuel,
            fuel_reported: false,
        };
        let mut body = orig.body.clone();
        let base_size = size_of(&body).max(1);
        let mut rounds = 0;
        loop {
            drv.changed = false;
            drv.drive(&mut body);
            drv.merge_cases(&mut body);
            drv.push_cases(&mut body);
            drv.dead_alts(&mut body);
            rounds += 1;
            if !drv.changed || rounds >= 8 {
                break;
            }
            if size_of(&body) > base_size * profile.measure.max(1) as usize {
                drv.site("unfold", Some("whistle: the residual grew past the measure bound".to_string()));
                break;
            }
        }
        body = renumber_core(&body, orig.params.len());
        match verify_residual(&orig, &body) {
            Ok(()) => {
                out.fns[fi].body = body;
                report.rechecked.push(orig.id.0.clone());
            }
            Err(why) => {
                report.discarded.push(format!("{}: {why}", orig.id.0));
            }
        }
        report.sites.extend(drv.sites);
        report.fused.extend(drv.fused);
        for f in drv.new_fns {
            new_fns.push(f);
        }
        defs.insert(out.fns[fi].id.0.clone(), out.fns[fi].clone());
    }
    out.fns.extend(new_fns);
    // The justification identities the rules relied on (supercompilation
    // and JIT specification section 8; decision D55).
    report.assumptions = program
        .rewrites
        .iter()
        .flat_map(|r| r.assumptions.clone())
        .collect();
    // Contract identity (supercompilation section 4): specialization is
    // representation-only; the residual's exported units are unchanged.
    for (a, b) in program.fns.iter().zip(out.fns.iter()) {
        if a.id != b.id || a.params.len() != b.params.len() {
            return Err(Error::plain(
                "internal error: specialization changed a unit's identity",
            ));
        }
    }
    Ok((out, report))
}

/// Node count for the whistle's measure bound.
fn size_of(e: &Core) -> usize {
    let mut n = 0;
    fn count(e: &Core, n: &mut usize) {
        *n += 1;
        for slot in child_slots_imm(e) {
            count(slot, n);
        }
    }
    count(e, &mut n);
    n
}

fn child_slots_imm(e: &Core) -> Vec<&Core> {
    let mut out: Vec<&Core> = Vec::new();
    match e {
        Core::Int(_) | Core::ConstBig { .. } | Core::Bool(_) | Core::Unit | Core::Global(_)
        | Core::Ctor(_) | Core::Prim(_) | Core::Foreign(_) | Core::Fault | Core::LazyAlloc | Core::SyntaxLit(_)
        | Core::StrLit(_) | Core::Var(_) => {}
        Core::DictProj(d, _) | Core::Proj(d, _) | Core::Demand(d) => out.push(&**d),
        Core::InstDict(_, args) | Core::App(_, args) | Core::Lam { env: args, .. } => {
            for a in args {
                out.push(a);
            }
        }
        Core::Let(_, v, body) => {
            out.push(&**v);
            out.push(&**body);
        }
        Core::LazyInit(_, t) => out.push(&**t),
        Core::If(c, t, fe) => {
            out.push(&**c);
            out.push(&*t.body);
            out.push(&*fe.body);
        }
        Core::Case { scrut, alts } => {
            out.push(&**scrut);
            for a in alts {
                out.push(&*a.body);
            }
        }
        Core::Quote { holes, .. } | Core::DeclsQuote { holes, .. } => {
            for h in holes {
                out.push(h);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every operation's symbol parses back to the same operation.
    #[test]
    fn symbol_round_trips() {
        let ops = [
            PrimOp::Nat(NatOp::Add),
            PrimOp::Nat(NatOp::Sub),
            PrimOp::Nat(NatOp::Div),
            PrimOp::Nat(NatOp::Cmp),
            PrimOp::Nat(NatOp::FromI64),
            PrimOp::Nat(NatOp::ToI64),
            PrimOp::Int(IntOp::Add),
            PrimOp::Int(IntOp::Neg),
            PrimOp::Int(IntOp::Cmp),
            PrimOp::Int(IntOp::ToI64),
            PrimOp::Bytes(BytesOp::New),
            PrimOp::Bytes(BytesOp::Get),
            PrimOp::Bytes(BytesOp::Set),
            PrimOp::NotBool,
            PrimOp::Delay,
            PrimOp::Bound { ty: Scalar::U8, max: false },
            PrimOp::Bound { ty: Scalar::I64, max: true },
            PrimOp::Arith { ty: Scalar::U32, op: ArithOp::Add, mode: ArithMode::Wrapping },
            PrimOp::Arith { ty: Scalar::I8, op: ArithOp::Mul, mode: ArithMode::Checked },
            PrimOp::Arith { ty: Scalar::Usize, op: ArithOp::Sub, mode: ArithMode::Saturating },
            PrimOp::Neg { ty: Scalar::I16, mode: ArithMode::Checked },
            PrimOp::Neg { ty: Scalar::U64, mode: ArithMode::Wrapping },
            PrimOp::CheckedDiv { ty: Scalar::I32 },
            PrimOp::CheckedRem { ty: Scalar::U8 },
            PrimOp::Shift { ty: Scalar::I64, op: ShiftOp::Shl, checked: true },
            PrimOp::Shift { ty: Scalar::U32, op: ShiftOp::Shr, checked: false },
            PrimOp::Rotate { ty: Scalar::U8, right: true },
            PrimOp::Bit { ty: Scalar::U16, op: BitOp::Xor },
            PrimOp::BitNot { ty: Scalar::Isize },
            PrimOp::BitScan { ty: Scalar::U32, op: BitScanOp::CountOnes },
            PrimOp::BitScan { ty: Scalar::U8, op: BitScanOp::SwapBytes },
            PrimOp::Cmp { ty: Scalar::Char, op: CmpOp::Ge },
            PrimOp::FloatBin { ty: Scalar::F32, op: FloatBinOp::Minimum },
            PrimOp::FloatUn { ty: Scalar::F64, op: FloatUnOp::Sqrt },
            PrimOp::FCmp { ty: Scalar::F64, op: CmpOp::Ne },
            PrimOp::FClass { ty: Scalar::F32, op: FClassOp::IsNormal },
            PrimOp::TotalOrder { ty: Scalar::F64 },
            PrimOp::CharToU32,
            PrimOp::U32ToChar,
            PrimOp::Conv { from: Scalar::U8, to: Scalar::I64, mode: ConvMode::Total },
            PrimOp::Conv { from: Scalar::U64, to: Scalar::I32, mode: ConvMode::Checked },
            PrimOp::Conv { from: Scalar::F64, to: Scalar::I32, mode: ConvMode::Saturating },
            PrimOp::Conv { from: Scalar::I32, to: Scalar::U8, mode: ConvMode::Wrapping },
            PrimOp::Conv { from: Scalar::F32, to: Scalar::F64, mode: ConvMode::Total },
        ];
        for op in ops {
            let sym = op.symbol();
            assert_eq!(
                PrimOp::from_symbol(&sym),
                Some(op),
                "round trip failed for {sym}"
            );
        }
    }

    /// Conversion families follow the rules of section 7: the total family
    /// exists when the value is preserved (plus the rounding float
    /// directions), `wrapping` is integer-to-integer only, and float to
    /// integer is never total.
    #[test]
    fn conversion_family_validation() {
        let ok = PrimOp::from_symbol("safelang.prim.u8ToI32");
        assert_eq!(
            ok,
            Some(PrimOp::Conv { from: Scalar::U8, to: Scalar::I32, mode: ConvMode::Total })
        );
        assert!(PrimOp::from_symbol("safelang.prim.checkedU8ToI64").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.saturatingF64ToI32").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.wrappingU64ToU32").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.i64ToF64").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.f32ToF64").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.f64ToF32").is_some());
        // No total form where values are lost, and none at all from a
        // float to an integer without a family.
        assert!(PrimOp::from_symbol("safelang.prim.i32ToU8").is_none());
        assert!(PrimOp::from_symbol("safelang.prim.u8ToI8").is_none());
        assert!(PrimOp::from_symbol("safelang.prim.f64ToI32").is_none());
        // Wrapping is integer-to-integer only.
        assert!(PrimOp::from_symbol("safelang.prim.wrappingF64ToI32").is_none());
        // Conversions never touch Char: `charToU32` and `u32ToChar` are
        // their own names.
        assert!(PrimOp::from_symbol("safelang.prim.charToU32").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.u32ToChar").is_some());
        assert!(PrimOp::from_symbol("safelang.prim.checkedU32ToChar").is_none());
    }

    /// The declared primitive types (primitive types specification,
    /// sections 4 and 7): checked operations return `Option`, and there is
    /// no `Add` instance-level name for machine types.
    #[test]
    fn primitive_signatures() {
        let (args, res) = PrimOp::Arith {
            ty: Scalar::U8,
            op: ArithOp::Add,
            mode: ArithMode::Checked,
        }
        .sig();
        assert_eq!(args, vec![Ty::Prim(Scalar::U8), Ty::Prim(Scalar::U8)]);
        assert_eq!(res, Ty::Adt("Option".into(), vec![Ty::Prim(Scalar::U8)]));
        let (_, res) = PrimOp::Arith {
            ty: Scalar::U8,
            op: ArithOp::Add,
            mode: ArithMode::Wrapping,
        }
        .sig();
        assert_eq!(res, Ty::Prim(Scalar::U8));
        let (_, res) = PrimOp::TotalOrder { ty: Scalar::F64 }.sig();
        assert_eq!(res, Ty::Adt("Ordering".into(), vec![]));
        let (args, res) = PrimOp::U32ToChar.sig();
        assert_eq!(args, vec![Ty::Prim(Scalar::U32)]);
        assert_eq!(res, Ty::Adt("Option".into(), vec![Ty::Prim(Scalar::Char)]));
    }

    /// The legacy wrapping-64 `safelang.prim.*Int` family is retired: its
    /// symbols no longer resolve (the machine case is the P2 `*I64` and
    /// `*I8`-`*U64` families, the mathematical case is `safelang.runtime.*`).
    #[test]
    fn retired_int_family_does_not_resolve() {
        for name in [
            "addInt", "subInt", "mulInt", "divInt", "remInt", "negInt", "eqInt", "neInt", "ltInt",
            "leInt", "gtInt", "geInt",
        ] {
            assert!(
                PrimOp::from_symbol(&format!("safelang.prim.{name}")).is_none(),
                "safelang.prim.{name} must no longer resolve"
            );
        }
    }

    /// The mathematical and `Bytes` families (primitive types
    /// specification, section 1): bignum arithmetic on boxed words, the
    /// comparison prims returning the raw -1/0/1 immediate at `I64`, and
    /// `Nat`-typed `Bytes` helpers.
    #[test]
    fn bignum_and_bytes_signatures() {
        assert_eq!(
            PrimOp::Nat(NatOp::Add).sig(),
            (vec![Ty::Nat, Ty::Nat], Ty::Nat)
        );
        assert_eq!(
            PrimOp::Nat(NatOp::Cmp).sig(),
            (vec![Ty::Nat, Ty::Nat], Ty::Prim(Scalar::I64))
        );
        assert_eq!(PrimOp::Nat(NatOp::FromI64).sig(), (vec![Ty::Prim(Scalar::I64)], Ty::Nat));
        assert_eq!(PrimOp::Nat(NatOp::ToI64).sig(), (vec![Ty::Nat], Ty::Prim(Scalar::I64)));
        assert_eq!(PrimOp::Int(IntOp::Neg).sig(), (vec![Ty::Int], Ty::Int));
        assert_eq!(
            PrimOp::Int(IntOp::Cmp).sig(),
            (vec![Ty::Int, Ty::Int], Ty::Prim(Scalar::I64))
        );
        assert_eq!(PrimOp::Bytes(BytesOp::New).sig(), (vec![Ty::Nat], Ty::Bytes));
        assert_eq!(PrimOp::Bytes(BytesOp::Len).sig(), (vec![Ty::Bytes], Ty::Nat));
        assert_eq!(
            PrimOp::Bytes(BytesOp::Get).sig(),
            (vec![Ty::Bytes, Ty::Nat], Ty::Nat)
        );
        assert_eq!(
            PrimOp::Bytes(BytesOp::Set).sig(),
            (vec![Ty::Bytes, Ty::Nat, Ty::Nat], Ty::Unit)
        );
        assert_eq!(PrimOp::Bytes(BytesOp::Set).arity(), 3);
    }
}
