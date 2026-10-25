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
    /// Method selection from a dictionary value.
    DictProj(Box<Core>, usize),
    /// Record field projection: load field `idx` from the record block.
    Proj(Box<Core>, usize),
    /// Instance dictionary construction: call the instance's `mk` unit.
    InstDict(GlobalId, Vec<Core>),
    /// Curried application spine with at least one argument.
    App(Box<Core>, Vec<Core>),
    /// Hoisted lambda: create a closure over `env` for function `fn_id`,
    /// whose parameters are `env` followed by the lambda's own parameter(s).
    Lam { fn_id: GlobalId, env: Vec<Core> },
    Let(LocalId, Box<Core>, Box<Core>),
    If(Box<Core>, Box<Core>, Box<Core>),
    /// Case on a boxed ADT value; `default` is the fall-through for the
    /// single wildcard or variable alternative, if any.
    Case {
        scrut: Box<Core>,
        alts: Vec<(CPat, Core)>,
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
    /// A secret reference; the value never leaves the interpreter.
    Secret(String),
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
