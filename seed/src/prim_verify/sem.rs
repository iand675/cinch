//! Shared declarative semantic IR (`SemGraph`) and canonical `Obligation`s.
//!
//! This module is the single source of truth for primitive semantics shared by
//! every verification backend (bit-blaster/LRAT, SMT-LIB 2, and concrete
//! evaluator). Deriving all backends from one representation prevents
//! double-encoding drift between SMT and kernel-checked proofs.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt;

/// Floating-point formats supported by the shared semantic IR.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FpFmt {
    F32,
    F64,
}

impl FpFmt {
    pub fn eb(self) -> u16 {
        match self {
            FpFmt::F32 => 8,
            FpFmt::F64 => 11,
        }
    }

    pub fn sb(self) -> u16 {
        match self {
            FpFmt::F32 => 24,
            FpFmt::F64 => 53,
        }
    }

    pub fn bit_width(self) -> u16 {
        match self {
            FpFmt::F32 => 32,
            FpFmt::F64 => 64,
        }
    }
}

/// Scalar semantic types in `SemGraph`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SemTy {
    Bool,
    Bv(u16),
    Fp(FpFmt),
}

impl fmt::Display for SemTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemTy::Bool => write!(f, "Bool"),
            SemTy::Bv(w) => write!(f, "Bv{w}"),
            SemTy::Fp(FpFmt::F32) => write!(f, "F32"),
            SemTy::Fp(FpFmt::F64) => write!(f, "F64"),
        }
    }
}

/// Floating-point comparison predicates matching LLVM `fcmp`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FpPred {
    False,
    Oeq,
    Ogt,
    Oge,
    Olt,
    Ole,
    One,
    Ord,
    Uno,
    Ueq,
    Ugt,
    Uge,
    Ult,
    Ule,
    Une,
    True,
}

/// Unary IEEE-754 floating-point intrinsics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FpUnaryOp {
    Abs,
    Sqrt,
    Floor,
    Ceil,
    Trunc,
    Round,
    RoundEven,
}

/// AES-128 hardware round core transformations (FIPS-197, without AddRoundKey).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AesRoundKind {
    /// `MixColumns(ShiftRows(SubBytes(state)))` (`aesenc` / `aese`+`aesmc`)
    EncCore,
    /// `ShiftRows(SubBytes(state))` (`aesenclast` / `aese`)
    EncLastCore,
    /// `InvMixColumns(InvShiftRows(InvSubBytes(state)))` (`aesdec` / `aesd`+`aesimc`)
    DecCore,
    /// `InvShiftRows(InvSubBytes(state))` (`aesdeclast` / `aesd`)
    DecLastCore,
}

/// Index into `SemGraph::nodes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId(pub u32);

/// One node in the declarative semantic DAG.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SemOp {
    BoolConst(bool),
    BvConst { width: u16, bits: u128 },
    FpConst { fmt: FpFmt, bits: u64 },
    Param { index: usize, name: String, ty: SemTy },

    // Boolean logic
    Not(NodeId),
    And(NodeId, NodeId),
    Or(NodeId, NodeId),
    Xor(NodeId, NodeId),
    Iff(NodeId, NodeId),
    Implies(NodeId, NodeId),
    Ite(NodeId, NodeId, NodeId),

    // Bit-vector unary / structural
    BvNot(NodeId),
    BvNeg(NodeId),
    BvExtract { x: NodeId, hi: u16, lo: u16 },
    BvConcat(NodeId, NodeId),
    BvZExt { x: NodeId, to: u16 },
    BvSExt { x: NodeId, to: u16 },
    BvBitReverse(NodeId),
    BvBSwap(NodeId),
    BvCtPop(NodeId),
    BvCtlz(NodeId),
    BvCttz(NodeId),

    // Bit-vector binary
    BvAdd(NodeId, NodeId),
    BvSub(NodeId, NodeId),
    BvMul(NodeId, NodeId),
    BvUDiv(NodeId, NodeId),
    BvURem(NodeId, NodeId),
    BvSDiv(NodeId, NodeId),
    BvSRem(NodeId, NodeId),
    BvAnd(NodeId, NodeId),
    BvOr(NodeId, NodeId),
    BvXor(NodeId, NodeId),
    BvShl(NodeId, NodeId),
    BvLShr(NodeId, NodeId),
    BvAShr(NodeId, NodeId),

    // Bit-vector funnel shifts: fshl(a, b, s) / fshr(a, b, s)
    BvFshl(NodeId, NodeId, NodeId),
    BvFshr(NodeId, NodeId, NodeId),

    // Hardware crypto / SIMD bit-vector primitives (exact GF(2) clmul & FIPS-197 AES round cores)
    /// Exact 64x64->128 carry-less (GF(2)) multiplication (`pclmulqdq` / `pmull`).
    BvClmul(NodeId, NodeId),
    /// Exact 128-bit FIPS-197 AES round core transformation (`aesenc`/`aesdec`/`aese`/`aesmc`).
    AesCore(AesRoundKind, NodeId),

    // Bit-vector comparisons
    BvEq(NodeId, NodeId),
    BvULt(NodeId, NodeId),
    BvULe(NodeId, NodeId),
    BvSLt(NodeId, NodeId),
    BvSLe(NodeId, NodeId),

    // Floating-point operations (IEEE-754 RNE unless noted)
    FpNeg(NodeId),
    FpAdd(NodeId, NodeId),
    FpSub(NodeId, NodeId),
    FpMul(NodeId, NodeId),
    FpDiv(NodeId, NodeId),
    FpCmp(FpPred, NodeId, NodeId),
    FpToBvBits(NodeId),
    BvBitsToFp(FpFmt, NodeId),
    BvToFpSigned(FpFmt, NodeId),
    BvToFpUnsigned(FpFmt, NodeId),
    FpToBvSigned(u16, NodeId),
    FpToBvUnsigned(u16, NodeId),
    FpToFp(FpFmt, NodeId),
    FpUnary(FpUnaryOp, NodeId),
    FpCopySign(NodeId, NodeId),
    /// True iff `f` is finite (not NaN/Inf) and trunc(f) fits in signed `width`-bit integer.
    FpFitsSigned(u16, NodeId),
    /// True iff `f` is finite (not NaN/Inf) and trunc(f) fits in unsigned `width`-bit integer.
    FpFitsUnsigned(u16, NodeId),
    /// True iff `f` is IEEE-754 NaN (`exp == all_ones && sig != 0`).
    FpIsNaN(NodeId),
    /// True iff `f` is IEEE-754 finite (`exp != all_ones`).
    FpIsFinite(NodeId),
}

/// Bitmask of theories used by a `SemGraph` or supported by a `Prover`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheorySet {
    pub bv: bool,
    pub bv_nonlinear: bool,
    pub fp: bool,
    /// True when floating-point arithmetic (`fadd`/`fsub`/`fmul`/`fdiv`/`sqrt`/rounding/casts)
    /// is present, which requires an SMT/FP backend rather than pure bit-level IEEE-754 bit-blasting.
    pub fp_arith: bool,
}

impl TheorySet {
    pub fn pure_bv() -> Self {
        Self { bv: true, bv_nonlinear: false, fp: false, fp_arith: false }
    }

    pub fn bv_all() -> Self {
        Self { bv: true, bv_nonlinear: true, fp: true, fp_arith: false }
    }

    pub fn all() -> Self {
        Self { bv: true, bv_nonlinear: true, fp: true, fp_arith: true }
    }

    pub fn subsumes(self, needed: TheorySet) -> bool {
        (!needed.bv || self.bv)
            && (!needed.bv_nonlinear || self.bv_nonlinear)
            && (!needed.fp || self.fp)
            && (!needed.fp_arith || self.fp_arith)
    }
}

/// Hash-consed semantic DAG.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SemGraph {
    nodes: Vec<(SemOp, SemTy)>,
    #[serde(skip)]
    intern: HashMap<SemOp, NodeId>,
}

impl SemGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn op(&self, id: NodeId) -> &SemOp {
        &self.nodes[id.0 as usize].0
    }

    pub fn ty(&self, id: NodeId) -> SemTy {
        self.nodes[id.0 as usize].1
    }

    pub fn nodes(&self) -> &[(SemOp, SemTy)] {
        &self.nodes
    }

    pub fn mask(width: u16) -> u128 {
        if width == 0 {
            0
        } else if width >= 128 {
            u128::MAX
        } else {
            (1u128 << width) - 1
        }
    }

    pub fn bv_width(&self, id: NodeId) -> u16 {
        match self.ty(id) {
            SemTy::Bv(w) => w,
            other => panic!("expected Bv node, got {other}"),
        }
    }

    pub fn fp_fmt(&self, id: NodeId) -> FpFmt {
        match self.ty(id) {
            SemTy::Fp(f) => f,
            other => panic!("expected Fp node, got {other}"),
        }
    }

    fn push(&mut self, op: SemOp, ty: SemTy) -> NodeId {
        if let Some(&id) = self.intern.get(&op) {
            return id;
        }
        let id = NodeId(self.nodes.len() as u32);
        self.nodes.push((op.clone(), ty));
        self.intern.insert(op, id);
        id
    }

    pub fn bool_const(&mut self, b: bool) -> NodeId {
        self.push(SemOp::BoolConst(b), SemTy::Bool)
    }

    pub fn bv_const(&mut self, width: u16, bits: u128) -> NodeId {
        self.push(SemOp::BvConst { width, bits: bits & Self::mask(width) }, SemTy::Bv(width))
    }

    pub fn fp_const(&mut self, fmt: FpFmt, bits: u64) -> NodeId {
        self.push(SemOp::FpConst { fmt, bits }, SemTy::Fp(fmt))
    }

    pub fn param(&mut self, index: usize, name: impl Into<String>, ty: SemTy) -> NodeId {
        self.push(SemOp::Param { index, name: name.into(), ty }, ty)
    }

    /// Import all nodes from `other` into `self`, remapping `SemOp::Param { index, .. }`
    /// to `param_map[index]` so two independently lowered `SemProgram`s share parameters.
    pub fn import_graph(&mut self, other: &SemGraph, param_map: &[NodeId]) -> Vec<NodeId> {
        let mut map: Vec<NodeId> = Vec::with_capacity(other.nodes.len());
        for (op, ty) in &other.nodes {
            let new_id = match op {
                SemOp::Param { index, .. } if *index < param_map.len() => param_map[*index],
                _ => {
                    let remapped = remap_op_slice(op, &map);
                    self.push(remapped, *ty)
                }
            };
            map.push(new_id);
        }
        map
    }

    pub fn not(&mut self, a: NodeId) -> NodeId {
        match self.op(a) {
            SemOp::BoolConst(b) => return self.bool_const(!b),
            SemOp::Not(inner) => return *inner,
            _ => {}
        }
        self.push(SemOp::Not(a), SemTy::Bool)
    }

    pub fn and(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return a;
        }
        match (self.op(a), self.op(b)) {
            (SemOp::BoolConst(false), _) | (_, SemOp::BoolConst(false)) => return self.bool_const(false),
            (SemOp::BoolConst(true), _) => return b,
            (_, SemOp::BoolConst(true)) => return a,
            _ => {}
        }
        self.push(SemOp::And(a, b), SemTy::Bool)
    }

    pub fn or(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return a;
        }
        match (self.op(a), self.op(b)) {
            (SemOp::BoolConst(true), _) | (_, SemOp::BoolConst(true)) => return self.bool_const(true),
            (SemOp::BoolConst(false), _) => return b,
            (_, SemOp::BoolConst(false)) => return a,
            _ => {}
        }
        self.push(SemOp::Or(a, b), SemTy::Bool)
    }

    pub fn xor(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return self.bool_const(false);
        }
        match (self.op(a), self.op(b)) {
            (SemOp::BoolConst(false), _) => return b,
            (_, SemOp::BoolConst(false)) => return a,
            (SemOp::BoolConst(true), _) => return self.not(b),
            (_, SemOp::BoolConst(true)) => return self.not(a),
            _ => {}
        }
        self.push(SemOp::Xor(a, b), SemTy::Bool)
    }

    pub fn iff(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return self.bool_const(true);
        }
        self.push(SemOp::Iff(a, b), SemTy::Bool)
    }

    pub fn implies(&mut self, a: NodeId, b: NodeId) -> NodeId {
        match (self.op(a), self.op(b)) {
            (SemOp::BoolConst(false), _) | (_, SemOp::BoolConst(true)) => return self.bool_const(true),
            (SemOp::BoolConst(true), _) => return b,
            (_, SemOp::BoolConst(false)) => return self.not(a),
            _ => {}
        }
        self.push(SemOp::Implies(a, b), SemTy::Bool)
    }

    pub fn ite(&mut self, c: NodeId, t: NodeId, e: NodeId) -> NodeId {
        if t == e {
            return t;
        }
        match self.op(c) {
            SemOp::BoolConst(true) => return t,
            SemOp::BoolConst(false) => return e,
            _ => {}
        }
        let ty = self.ty(t);
        debug_assert_eq!(ty, self.ty(e));
        self.push(SemOp::Ite(c, t, e), ty)
    }

    pub fn bv_not(&mut self, a: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvNot(a), SemTy::Bv(w))
    }

    pub fn bv_neg(&mut self, a: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvNeg(a), SemTy::Bv(w))
    }

    pub fn bv_extract(&mut self, x: NodeId, hi: u16, lo: u16) -> NodeId {
        let w = self.bv_width(x);
        debug_assert!(lo <= hi && hi < w);
        if lo == 0 && hi + 1 == w {
            return x;
        }
        let out_w = hi - lo + 1;
        self.push(SemOp::BvExtract { x, hi, lo }, SemTy::Bv(out_w))
    }

    pub fn bv_concat(&mut self, hi: NodeId, lo: NodeId) -> NodeId {
        let w = self.bv_width(hi) + self.bv_width(lo);
        self.push(SemOp::BvConcat(hi, lo), SemTy::Bv(w))
    }

    pub fn bv_zext(&mut self, x: NodeId, to: u16) -> NodeId {
        let from = self.bv_width(x);
        if from == to {
            return x;
        }
        debug_assert!(to > from);
        self.push(SemOp::BvZExt { x, to }, SemTy::Bv(to))
    }

    pub fn bv_sext(&mut self, x: NodeId, to: u16) -> NodeId {
        let from = self.bv_width(x);
        if from == to {
            return x;
        }
        debug_assert!(to > from);
        self.push(SemOp::BvSExt { x, to }, SemTy::Bv(to))
    }

    pub fn bv_trunc(&mut self, x: NodeId, to: u16) -> NodeId {
        let from = self.bv_width(x);
        if from == to {
            return x;
        }
        debug_assert!(to < from && to > 0);
        self.bv_extract(x, to - 1, 0)
    }

    pub fn bv_bitreverse(&mut self, x: NodeId) -> NodeId {
        let w = self.bv_width(x);
        self.push(SemOp::BvBitReverse(x), SemTy::Bv(w))
    }

    pub fn bv_bswap(&mut self, x: NodeId) -> NodeId {
        let w = self.bv_width(x);
        self.push(SemOp::BvBSwap(x), SemTy::Bv(w))
    }

    pub fn bv_ctpop(&mut self, x: NodeId) -> NodeId {
        let w = self.bv_width(x);
        self.push(SemOp::BvCtPop(x), SemTy::Bv(w))
    }

    pub fn bv_ctlz(&mut self, x: NodeId) -> NodeId {
        let w = self.bv_width(x);
        self.push(SemOp::BvCtlz(x), SemTy::Bv(w))
    }

    pub fn bv_cttz(&mut self, x: NodeId) -> NodeId {
        let w = self.bv_width(x);
        self.push(SemOp::BvCttz(x), SemTy::Bv(w))
    }

    pub fn bv_add(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvAdd(a, b), SemTy::Bv(w))
    }

    pub fn bv_sub(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvSub(a, b), SemTy::Bv(w))
    }

    pub fn bv_mul(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvMul(a, b), SemTy::Bv(w))
    }

    pub fn bv_udiv(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvUDiv(a, b), SemTy::Bv(w))
    }

    pub fn bv_urem(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvURem(a, b), SemTy::Bv(w))
    }

    pub fn bv_sdiv(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvSDiv(a, b), SemTy::Bv(w))
    }

    pub fn bv_srem(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvSRem(a, b), SemTy::Bv(w))
    }

    pub fn bv_and(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvAnd(a, b), SemTy::Bv(w))
    }

    pub fn bv_or(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvOr(a, b), SemTy::Bv(w))
    }

    pub fn bv_xor(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvXor(a, b), SemTy::Bv(w))
    }

    pub fn bv_shl(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvShl(a, b), SemTy::Bv(w))
    }

    pub fn bv_lshr(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvLShr(a, b), SemTy::Bv(w))
    }

    pub fn bv_ashr(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvAShr(a, b), SemTy::Bv(w))
    }

    pub fn bv_fshl(&mut self, a: NodeId, b: NodeId, s: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvFshl(a, b, s), SemTy::Bv(w))
    }

    pub fn bv_fshr(&mut self, a: NodeId, b: NodeId, s: NodeId) -> NodeId {
        let w = self.bv_width(a);
        self.push(SemOp::BvFshr(a, b, s), SemTy::Bv(w))
    }

    pub fn bv_eq(&mut self, a: NodeId, b: NodeId) -> NodeId {
        if a == b {
            return self.bool_const(true);
        }
        self.push(SemOp::BvEq(a, b), SemTy::Bool)
    }

    pub fn bv_ult(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::BvULt(a, b), SemTy::Bool)
    }

    pub fn bv_ule(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::BvULe(a, b), SemTy::Bool)
    }

    pub fn bv_slt(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::BvSLt(a, b), SemTy::Bool)
    }

    pub fn bv_sle(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::BvSLe(a, b), SemTy::Bool)
    }

    pub fn fp_neg(&mut self, a: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpNeg(a), SemTy::Fp(fmt))
    }

    pub fn fp_add(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpAdd(a, b), SemTy::Fp(fmt))
    }

    pub fn fp_sub(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpSub(a, b), SemTy::Fp(fmt))
    }

    pub fn fp_mul(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpMul(a, b), SemTy::Fp(fmt))
    }

    pub fn fp_div(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpDiv(a, b), SemTy::Fp(fmt))
    }

    pub fn fp_cmp(&mut self, pred: FpPred, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::FpCmp(pred, a, b), SemTy::Bool)
    }

    pub fn fp_to_bv_bits(&mut self, a: NodeId) -> NodeId {
        let w = self.fp_fmt(a).bit_width();
        self.push(SemOp::FpToBvBits(a), SemTy::Bv(w))
    }

    pub fn bv_bits_to_fp(&mut self, fmt: FpFmt, a: NodeId) -> NodeId {
        self.push(SemOp::BvBitsToFp(fmt, a), SemTy::Fp(fmt))
    }

    pub fn bv_to_fp_signed(&mut self, fmt: FpFmt, a: NodeId) -> NodeId {
        self.push(SemOp::BvToFpSigned(fmt, a), SemTy::Fp(fmt))
    }

    pub fn bv_to_fp_unsigned(&mut self, fmt: FpFmt, a: NodeId) -> NodeId {
        self.push(SemOp::BvToFpUnsigned(fmt, a), SemTy::Fp(fmt))
    }

    pub fn fp_to_bv_signed(&mut self, width: u16, a: NodeId) -> NodeId {
        self.push(SemOp::FpToBvSigned(width, a), SemTy::Bv(width))
    }

    pub fn fp_to_bv_unsigned(&mut self, width: u16, a: NodeId) -> NodeId {
        self.push(SemOp::FpToBvUnsigned(width, a), SemTy::Bv(width))
    }

    pub fn fp_to_fp(&mut self, fmt: FpFmt, a: NodeId) -> NodeId {
        self.push(SemOp::FpToFp(fmt, a), SemTy::Fp(fmt))
    }

    pub fn fp_unary(&mut self, op: FpUnaryOp, a: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpUnary(op, a), SemTy::Fp(fmt))
    }

    pub fn fp_copysign(&mut self, a: NodeId, b: NodeId) -> NodeId {
        let fmt = self.fp_fmt(a);
        self.push(SemOp::FpCopySign(a, b), SemTy::Fp(fmt))
    }

    pub fn fp_fits_signed(&mut self, width: u16, a: NodeId) -> NodeId {
        self.push(SemOp::FpFitsSigned(width, a), SemTy::Bool)
    }

    pub fn fp_fits_unsigned(&mut self, width: u16, a: NodeId) -> NodeId {
        self.push(SemOp::FpFitsUnsigned(width, a), SemTy::Bool)
    }

    pub fn fp_is_nan(&mut self, a: NodeId) -> NodeId {
        self.push(SemOp::FpIsNaN(a), SemTy::Bool)
    }

    pub fn fp_is_finite(&mut self, a: NodeId) -> NodeId {
        self.push(SemOp::FpIsFinite(a), SemTy::Bool)
    }

    pub fn bv_clmul(&mut self, a: NodeId, b: NodeId) -> NodeId {
        self.push(SemOp::BvClmul(a, b), SemTy::Bv(128))
    }

    pub fn aes_core(&mut self, kind: AesRoundKind, state: NodeId) -> NodeId {
        self.push(SemOp::AesCore(kind, state), SemTy::Bv(128))
    }

    /// True if any node in the entire graph (even outside `goal`) uses FP arithmetic
    /// (`fadd`/`fsub`/`fmul`/`fdiv`/`sqrt`/rounding/casts), distinguishing pure bit-level
    /// IEEE-754 prims from arithmetic float prims that require an SMT/FP solver.
    pub fn has_fp_arith_anywhere(&self) -> bool {
        self.nodes.iter().any(|(op, _)| is_fp_arith_op(op))
    }

    /// Convert a `Bool` node to a 1-bit `Bv(1)`.
    pub fn bool_to_bv1(&mut self, b: NodeId) -> NodeId {
        let one = self.bv_const(1, 1);
        let zero = self.bv_const(1, 0);
        self.ite(b, one, zero)
    }

    /// Convert a 1-bit `Bv(1)` node to `Bool`.
    pub fn bv1_to_bool(&mut self, v: NodeId) -> NodeId {
        let one = self.bv_const(1, 1);
        self.bv_eq(v, one)
    }

    /// Compute the theories used by nodes reachable from `root`.
    pub fn theories_of(&self, root: NodeId) -> TheorySet {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![root];
        let mut theories = TheorySet::default();
        while let Some(id) = stack.pop() {
            let idx = id.0 as usize;
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            let (op, ty) = &self.nodes[idx];
            match ty {
                SemTy::Bv(_) => theories.bv = true,
                SemTy::Fp(_) => theories.fp = true,
                SemTy::Bool => {}
            }
            match op {
                SemOp::BvMul(..)
                | SemOp::BvUDiv(..)
                | SemOp::BvURem(..)
                | SemOp::BvSDiv(..)
                | SemOp::BvSRem(..) => {
                    theories.bv = true;
                    theories.bv_nonlinear = true;
                }
                SemOp::FpAdd(..)
                | SemOp::FpSub(..)
                | SemOp::FpMul(..)
                | SemOp::FpDiv(..)
                | SemOp::BvToFpSigned(..)
                | SemOp::BvToFpUnsigned(..)
                | SemOp::FpToBvSigned(..)
                | SemOp::FpToBvUnsigned(..)
                | SemOp::FpToFp(..)
                | SemOp::FpFitsSigned(..)
                | SemOp::FpFitsUnsigned(..) => {
                    theories.fp = true;
                    theories.fp_arith = true;
                }
                SemOp::FpUnary(u, _) if !matches!(u, FpUnaryOp::Abs) => {
                    theories.fp = true;
                    theories.fp_arith = true;
                }
                SemOp::FpConst { .. }
                | SemOp::FpNeg(_)
                | SemOp::FpCmp(..)
                | SemOp::FpToBvBits(_)
                | SemOp::BvBitsToFp(..)
                | SemOp::FpUnary(FpUnaryOp::Abs, _)
                | SemOp::FpCopySign(..)
                | SemOp::FpIsNaN(_)
                | SemOp::FpIsFinite(_) => {
                    theories.fp = true;
                }
                _ => {}
            }
            for child in op.children() {
                stack.push(child);
            }
        }
        theories
    }
}

impl SemOp {
    pub fn children(&self) -> Vec<NodeId> {
        match *self {
            SemOp::BoolConst(_) | SemOp::BvConst { .. } | SemOp::FpConst { .. } | SemOp::Param { .. } => vec![],
            SemOp::Not(a)
            | SemOp::BvNot(a)
            | SemOp::BvNeg(a)
            | SemOp::BvExtract { x: a, .. }
            | SemOp::BvZExt { x: a, .. }
            | SemOp::BvSExt { x: a, .. }
            | SemOp::BvBitReverse(a)
            | SemOp::BvBSwap(a)
            | SemOp::BvCtPop(a)
            | SemOp::BvCtlz(a)
            | SemOp::BvCttz(a)
            | SemOp::FpNeg(a)
            | SemOp::FpToBvBits(a)
            | SemOp::BvBitsToFp(_, a)
            | SemOp::BvToFpSigned(_, a)
            | SemOp::BvToFpUnsigned(_, a)
            | SemOp::FpToBvSigned(_, a)
            | SemOp::FpToBvUnsigned(_, a)
            | SemOp::FpToFp(_, a)
            | SemOp::FpUnary(_, a)
            | SemOp::FpFitsSigned(_, a)
            | SemOp::FpFitsUnsigned(_, a)
            | SemOp::FpIsNaN(a)
            | SemOp::FpIsFinite(a)
            | SemOp::AesCore(_, a) => vec![a],
            SemOp::And(a, b)
            | SemOp::Or(a, b)
            | SemOp::Xor(a, b)
            | SemOp::Iff(a, b)
            | SemOp::Implies(a, b)
            | SemOp::BvConcat(a, b)
            | SemOp::BvAdd(a, b)
            | SemOp::BvSub(a, b)
            | SemOp::BvMul(a, b)
            | SemOp::BvClmul(a, b)
            | SemOp::BvUDiv(a, b)
            | SemOp::BvURem(a, b)
            | SemOp::BvSDiv(a, b)
            | SemOp::BvSRem(a, b)
            | SemOp::BvAnd(a, b)
            | SemOp::BvOr(a, b)
            | SemOp::BvXor(a, b)
            | SemOp::BvShl(a, b)
            | SemOp::BvLShr(a, b)
            | SemOp::BvAShr(a, b)
            | SemOp::BvEq(a, b)
            | SemOp::BvULt(a, b)
            | SemOp::BvULe(a, b)
            | SemOp::BvSLt(a, b)
            | SemOp::BvSLe(a, b)
            | SemOp::FpAdd(a, b)
            | SemOp::FpSub(a, b)
            | SemOp::FpMul(a, b)
            | SemOp::FpDiv(a, b)
            | SemOp::FpCmp(_, a, b)
            | SemOp::FpCopySign(a, b) => vec![a, b],
            SemOp::Ite(a, b, c) | SemOp::BvFshl(a, b, c) | SemOp::BvFshr(a, b, c) => vec![a, b, c],
        }
    }
}

/// Three-valued LLVM denotation inside `SemGraph`:
/// every scalar/lane pairs its value expression with a `Bool` poison flag.
#[derive(Clone, Debug)]
pub enum SemVal {
    Scalar { val: NodeId, poison: NodeId },
    Vector { elem_ty: SemTy, lanes: Vec<(NodeId, NodeId)> },
    Tuple(Vec<SemVal>),
}

impl SemVal {
    /// Returns a `Bool` node that is true iff *any* scalar or lane is poison.
    pub fn any_poison(&self, g: &mut SemGraph) -> NodeId {
        match self {
            SemVal::Scalar { poison, .. } => *poison,
            SemVal::Vector { lanes, .. } => {
                let mut acc = g.bool_const(false);
                for &(_, p) in lanes {
                    acc = g.or(acc, p);
                }
                acc
            }
            SemVal::Tuple(items) => {
                let mut acc = g.bool_const(false);
                for item in items {
                    let p = item.any_poison(g);
                    acc = g.or(acc, p);
                }
                acc
            }
        }
    }

    pub fn as_scalar(&self) -> Option<(NodeId, NodeId)> {
        match *self {
            SemVal::Scalar { val, poison } => Some((val, poison)),
            _ => None,
        }
    }
}

/// Parameter descriptor in a lowered `SemProgram` / `Obligation`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemParam {
    pub index: usize,
    pub name: String,
    pub ty: SemTy,
    pub signed: bool,
}

/// Result of lowering an inline IR body via an `IrSemantics` plugin.
#[derive(Clone, Debug)]
pub struct SemProgram {
    pub semantics_id: String,
    pub graph: SemGraph,
    pub params: Vec<(SemParam, NodeId)>,
    pub result: SemVal,
    /// `Bool` node that is true iff any instruction triggered immediate UB.
    pub ub_cond: NodeId,
}

/// What property an `Obligation` asserts.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObligationKind {
    /// No instruction triggers immediate UB under parameter preconditions.
    NoUb,
    /// Returned value (or any vector lane) is never poison under preconditions.
    NoPoison,
    /// Combined well-definedness (`!ub && !poison`).
    WellDefined,
    /// Return refinement predicate holds (and body is well-defined).
    Refinement { description: String },
    /// Body matches a pure mathematical specification (and is well-defined).
    FunctionalEquiv { spec_name: String },
    /// Two target-specialized bodies of the same `prim` are semantically equivalent.
    CrossTargetEquiv { body_a: String, body_b: String },
    /// A `rewrite` rule's LHS and RHS are semantically equivalent under its precondition.
    RewriteEquiv { rule_name: String },
    /// Core function and emitted LLVM IR (or pre/post optimization Core) agree semantically.
    TranslationVal { fn_name: String, stage: String },
}

impl fmt::Display for ObligationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObligationKind::NoUb => write!(f, "no-ub"),
            ObligationKind::NoPoison => write!(f, "no-poison"),
            ObligationKind::WellDefined => write!(f, "well-defined"),
            ObligationKind::Refinement { description } => write!(f, "refinement({description})"),
            ObligationKind::FunctionalEquiv { spec_name } => write!(f, "equiv({spec_name})"),
            ObligationKind::CrossTargetEquiv { body_a, body_b } => {
                write!(f, "cross-target-equiv({body_a} == {body_b})")
            }
            ObligationKind::RewriteEquiv { rule_name } => write!(f, "rewrite-equiv({rule_name})"),
            ObligationKind::TranslationVal { fn_name, stage } => {
                write!(f, "translation-val({fn_name}:{stage})")
            }
        }
    }
}

/// A canonical, backend-agnostic, content-addressed verification obligation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Obligation {
    pub name: String,
    pub semantics_id: String,
    pub kind: ObligationKind,
    pub params: Vec<SemParam>,
    pub graph: SemGraph,
    /// `Bool` node that must be valid (`true` for every assignment to `params`).
    pub goal: NodeId,
    /// Deterministic SHA-256 hash over `(semantics_id, params, reachable graph, goal)`.
    pub goal_hash: String,
}

impl Obligation {
    pub fn new(
        name: impl Into<String>,
        semantics_id: impl Into<String>,
        kind: ObligationKind,
        params: Vec<SemParam>,
        graph: SemGraph,
        goal: NodeId,
    ) -> Self {
        let semantics_id = semantics_id.into();
        let goal_hash = compute_goal_hash(&semantics_id, &params, &graph, goal);
        Self {
            name: name.into(),
            semantics_id,
            kind,
            params,
            graph,
            goal,
            goal_hash,
        }
    }

    pub fn theories(&self) -> TheorySet {
        let mut t = self.graph.theories_of(self.goal);
        if self.graph.has_fp_arith_anywhere() {
            t.fp = true;
            t.fp_arith = true;
        }
        t
    }
}

fn is_fp_arith_op(op: &SemOp) -> bool {
    matches!(
        op,
        SemOp::FpAdd(..)
            | SemOp::FpSub(..)
            | SemOp::FpMul(..)
            | SemOp::FpDiv(..)
            | SemOp::BvToFpSigned(..)
            | SemOp::BvToFpUnsigned(..)
            | SemOp::FpToBvSigned(..)
            | SemOp::FpToBvUnsigned(..)
            | SemOp::FpToFp(..)
            | SemOp::FpFitsSigned(..)
            | SemOp::FpFitsUnsigned(..)
    ) || matches!(op, SemOp::FpUnary(u, _) if !matches!(u, FpUnaryOp::Abs))
}

fn compute_goal_hash(
    semantics_id: &str,
    params: &[SemParam],
    graph: &SemGraph,
    goal: NodeId,
) -> String {
    // Canonical serialization over reachable subgraph from `goal`.
    let mut visited = HashMap::new();
    let mut order = Vec::new();
    fn dfs(
        id: NodeId,
        graph: &SemGraph,
        visited: &mut HashMap<NodeId, u32>,
        order: &mut Vec<NodeId>,
    ) {
        if visited.contains_key(&id) {
            return;
        }
        for c in graph.op(id).children() {
            dfs(c, graph, visited, order);
        }
        let next = order.len() as u32;
        visited.insert(id, next);
        order.push(id);
    }
    dfs(goal, graph, &mut visited, &mut order);

    let mut hasher = Sha256::new();
    hasher.update(b"cinch-obligation-v1\0");
    hasher.update(semantics_id.as_bytes());
    hasher.update(b"\0");
    for p in params {
        hasher.update(format!("{}:{}:{}|", p.index, p.ty, p.signed).as_bytes());
    }
    hasher.update(b"\0");
    if graph.has_fp_arith_anywhere() {
        hasher.update(b"fp_arith=1\0");
    }
    for &id in &order {
        let op = graph.op(id);
        let ty = graph.ty(id);
        // Hash canonical op structure with remapped child indices.
        hasher.update(format!("{ty}:{:?};", remap_op(op, &visited)).as_bytes());
    }
    let digest = hasher.finalize();
    format!("sha256:{:x}", digest)
}

fn remap_op_slice(op: &SemOp, map: &[NodeId]) -> SemOp {
    let r = |id: NodeId| map[id.0 as usize];
    match op.clone() {
        SemOp::BoolConst(b) => SemOp::BoolConst(b),
        SemOp::BvConst { width, bits } => SemOp::BvConst { width, bits },
        SemOp::FpConst { fmt, bits } => SemOp::FpConst { fmt, bits },
        SemOp::Param { index, name, ty } => SemOp::Param { index, name, ty },
        SemOp::Not(a) => SemOp::Not(r(a)),
        SemOp::And(a, b) => SemOp::And(r(a), r(b)),
        SemOp::Or(a, b) => SemOp::Or(r(a), r(b)),
        SemOp::Xor(a, b) => SemOp::Xor(r(a), r(b)),
        SemOp::Iff(a, b) => SemOp::Iff(r(a), r(b)),
        SemOp::Implies(a, b) => SemOp::Implies(r(a), r(b)),
        SemOp::Ite(c, t, e) => SemOp::Ite(r(c), r(t), r(e)),
        SemOp::BvNot(a) => SemOp::BvNot(r(a)),
        SemOp::BvNeg(a) => SemOp::BvNeg(r(a)),
        SemOp::BvExtract { x, hi, lo } => SemOp::BvExtract { x: r(x), hi, lo },
        SemOp::BvConcat(a, b) => SemOp::BvConcat(r(a), r(b)),
        SemOp::BvZExt { x, to } => SemOp::BvZExt { x: r(x), to },
        SemOp::BvSExt { x, to } => SemOp::BvSExt { x: r(x), to },
        SemOp::BvBitReverse(a) => SemOp::BvBitReverse(r(a)),
        SemOp::BvBSwap(a) => SemOp::BvBSwap(r(a)),
        SemOp::BvCtPop(a) => SemOp::BvCtPop(r(a)),
        SemOp::BvCtlz(a) => SemOp::BvCtlz(r(a)),
        SemOp::BvCttz(a) => SemOp::BvCttz(r(a)),
        SemOp::BvAdd(a, b) => SemOp::BvAdd(r(a), r(b)),
        SemOp::BvSub(a, b) => SemOp::BvSub(r(a), r(b)),
        SemOp::BvMul(a, b) => SemOp::BvMul(r(a), r(b)),
        SemOp::BvUDiv(a, b) => SemOp::BvUDiv(r(a), r(b)),
        SemOp::BvURem(a, b) => SemOp::BvURem(r(a), r(b)),
        SemOp::BvSDiv(a, b) => SemOp::BvSDiv(r(a), r(b)),
        SemOp::BvSRem(a, b) => SemOp::BvSRem(r(a), r(b)),
        SemOp::BvAnd(a, b) => SemOp::BvAnd(r(a), r(b)),
        SemOp::BvOr(a, b) => SemOp::BvOr(r(a), r(b)),
        SemOp::BvXor(a, b) => SemOp::BvXor(r(a), r(b)),
        SemOp::BvShl(a, b) => SemOp::BvShl(r(a), r(b)),
        SemOp::BvLShr(a, b) => SemOp::BvLShr(r(a), r(b)),
        SemOp::BvAShr(a, b) => SemOp::BvAShr(r(a), r(b)),
        SemOp::BvFshl(a, b, c) => SemOp::BvFshl(r(a), r(b), r(c)),
        SemOp::BvFshr(a, b, c) => SemOp::BvFshr(r(a), r(b), r(c)),
        SemOp::BvClmul(a, b) => SemOp::BvClmul(r(a), r(b)),
        SemOp::AesCore(k, a) => SemOp::AesCore(k, r(a)),
        SemOp::BvEq(a, b) => SemOp::BvEq(r(a), r(b)),
        SemOp::BvULt(a, b) => SemOp::BvULt(r(a), r(b)),
        SemOp::BvULe(a, b) => SemOp::BvULe(r(a), r(b)),
        SemOp::BvSLt(a, b) => SemOp::BvSLt(r(a), r(b)),
        SemOp::BvSLe(a, b) => SemOp::BvSLe(r(a), r(b)),
        SemOp::FpNeg(a) => SemOp::FpNeg(r(a)),
        SemOp::FpAdd(a, b) => SemOp::FpAdd(r(a), r(b)),
        SemOp::FpSub(a, b) => SemOp::FpSub(r(a), r(b)),
        SemOp::FpMul(a, b) => SemOp::FpMul(r(a), r(b)),
        SemOp::FpDiv(a, b) => SemOp::FpDiv(r(a), r(b)),
        SemOp::FpCmp(p, a, b) => SemOp::FpCmp(p, r(a), r(b)),
        SemOp::FpToBvBits(a) => SemOp::FpToBvBits(r(a)),
        SemOp::BvBitsToFp(f, a) => SemOp::BvBitsToFp(f, r(a)),
        SemOp::BvToFpSigned(f, a) => SemOp::BvToFpSigned(f, r(a)),
        SemOp::BvToFpUnsigned(f, a) => SemOp::BvToFpUnsigned(f, r(a)),
        SemOp::FpToBvSigned(w, a) => SemOp::FpToBvSigned(w, r(a)),
        SemOp::FpToBvUnsigned(w, a) => SemOp::FpToBvUnsigned(w, r(a)),
        SemOp::FpToFp(f, a) => SemOp::FpToFp(f, r(a)),
        SemOp::FpUnary(u, a) => SemOp::FpUnary(u, r(a)),
        SemOp::FpCopySign(a, b) => SemOp::FpCopySign(r(a), r(b)),
        SemOp::FpFitsSigned(w, a) => SemOp::FpFitsSigned(w, r(a)),
        SemOp::FpFitsUnsigned(w, a) => SemOp::FpFitsUnsigned(w, r(a)),
        SemOp::FpIsNaN(a) => SemOp::FpIsNaN(r(a)),
        SemOp::FpIsFinite(a) => SemOp::FpIsFinite(r(a)),
    }
}

fn remap_op(op: &SemOp, map: &HashMap<NodeId, u32>) -> SemOp {
    let r = |id: NodeId| NodeId(map[&id]);
    match op.clone() {
        SemOp::BoolConst(b) => SemOp::BoolConst(b),
        SemOp::BvConst { width, bits } => SemOp::BvConst { width, bits },
        SemOp::FpConst { fmt, bits } => SemOp::FpConst { fmt, bits },
        SemOp::Param { index, ty, .. } => SemOp::Param { index, name: format!("p{index}"), ty },
        SemOp::Not(a) => SemOp::Not(r(a)),
        SemOp::And(a, b) => SemOp::And(r(a), r(b)),
        SemOp::Or(a, b) => SemOp::Or(r(a), r(b)),
        SemOp::Xor(a, b) => SemOp::Xor(r(a), r(b)),
        SemOp::Iff(a, b) => SemOp::Iff(r(a), r(b)),
        SemOp::Implies(a, b) => SemOp::Implies(r(a), r(b)),
        SemOp::Ite(c, t, e) => SemOp::Ite(r(c), r(t), r(e)),
        SemOp::BvNot(a) => SemOp::BvNot(r(a)),
        SemOp::BvNeg(a) => SemOp::BvNeg(r(a)),
        SemOp::BvExtract { x, hi, lo } => SemOp::BvExtract { x: r(x), hi, lo },
        SemOp::BvConcat(a, b) => SemOp::BvConcat(r(a), r(b)),
        SemOp::BvZExt { x, to } => SemOp::BvZExt { x: r(x), to },
        SemOp::BvSExt { x, to } => SemOp::BvSExt { x: r(x), to },
        SemOp::BvBitReverse(a) => SemOp::BvBitReverse(r(a)),
        SemOp::BvBSwap(a) => SemOp::BvBSwap(r(a)),
        SemOp::BvCtPop(a) => SemOp::BvCtPop(r(a)),
        SemOp::BvCtlz(a) => SemOp::BvCtlz(r(a)),
        SemOp::BvCttz(a) => SemOp::BvCttz(r(a)),
        SemOp::BvAdd(a, b) => SemOp::BvAdd(r(a), r(b)),
        SemOp::BvSub(a, b) => SemOp::BvSub(r(a), r(b)),
        SemOp::BvMul(a, b) => SemOp::BvMul(r(a), r(b)),
        SemOp::BvUDiv(a, b) => SemOp::BvUDiv(r(a), r(b)),
        SemOp::BvURem(a, b) => SemOp::BvURem(r(a), r(b)),
        SemOp::BvSDiv(a, b) => SemOp::BvSDiv(r(a), r(b)),
        SemOp::BvSRem(a, b) => SemOp::BvSRem(r(a), r(b)),
        SemOp::BvAnd(a, b) => SemOp::BvAnd(r(a), r(b)),
        SemOp::BvOr(a, b) => SemOp::BvOr(r(a), r(b)),
        SemOp::BvXor(a, b) => SemOp::BvXor(r(a), r(b)),
        SemOp::BvShl(a, b) => SemOp::BvShl(r(a), r(b)),
        SemOp::BvLShr(a, b) => SemOp::BvLShr(r(a), r(b)),
        SemOp::BvAShr(a, b) => SemOp::BvAShr(r(a), r(b)),
        SemOp::BvFshl(a, b, c) => SemOp::BvFshl(r(a), r(b), r(c)),
        SemOp::BvFshr(a, b, c) => SemOp::BvFshr(r(a), r(b), r(c)),
        SemOp::BvClmul(a, b) => SemOp::BvClmul(r(a), r(b)),
        SemOp::AesCore(k, a) => SemOp::AesCore(k, r(a)),
        SemOp::BvEq(a, b) => SemOp::BvEq(r(a), r(b)),
        SemOp::BvULt(a, b) => SemOp::BvULt(r(a), r(b)),
        SemOp::BvULe(a, b) => SemOp::BvULe(r(a), r(b)),
        SemOp::BvSLt(a, b) => SemOp::BvSLt(r(a), r(b)),
        SemOp::BvSLe(a, b) => SemOp::BvSLe(r(a), r(b)),
        SemOp::FpNeg(a) => SemOp::FpNeg(r(a)),
        SemOp::FpAdd(a, b) => SemOp::FpAdd(r(a), r(b)),
        SemOp::FpSub(a, b) => SemOp::FpSub(r(a), r(b)),
        SemOp::FpMul(a, b) => SemOp::FpMul(r(a), r(b)),
        SemOp::FpDiv(a, b) => SemOp::FpDiv(r(a), r(b)),
        SemOp::FpCmp(p, a, b) => SemOp::FpCmp(p, r(a), r(b)),
        SemOp::FpToBvBits(a) => SemOp::FpToBvBits(r(a)),
        SemOp::BvBitsToFp(f, a) => SemOp::BvBitsToFp(f, r(a)),
        SemOp::BvToFpSigned(f, a) => SemOp::BvToFpSigned(f, r(a)),
        SemOp::BvToFpUnsigned(f, a) => SemOp::BvToFpUnsigned(f, r(a)),
        SemOp::FpToBvSigned(w, a) => SemOp::FpToBvSigned(w, r(a)),
        SemOp::FpToBvUnsigned(w, a) => SemOp::FpToBvUnsigned(w, r(a)),
        SemOp::FpToFp(f, a) => SemOp::FpToFp(f, r(a)),
        SemOp::FpUnary(u, a) => SemOp::FpUnary(u, r(a)),
        SemOp::FpCopySign(a, b) => SemOp::FpCopySign(r(a), r(b)),
        SemOp::FpFitsSigned(w, a) => SemOp::FpFitsSigned(w, r(a)),
        SemOp::FpFitsUnsigned(w, a) => SemOp::FpFitsUnsigned(w, r(a)),
        SemOp::FpIsNaN(a) => SemOp::FpIsNaN(r(a)),
        SemOp::FpIsFinite(a) => SemOp::FpIsFinite(r(a)),
    }
}

/// Concrete runtime value for counterexample validation and finite-domain evaluation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcreteVal {
    Bool(bool),
    Bv { width: u16, bits: u128 },
    Fp { fmt: FpFmt, bits: u64 },
}

impl ConcreteVal {
    pub fn as_bool(self) -> bool {
        match self {
            ConcreteVal::Bool(b) => b,
            _ => panic!("expected Bool"),
        }
    }

    pub fn as_bv(self) -> (u16, u128) {
        match self {
            ConcreteVal::Bv { width, bits } => (width, bits),
            _ => panic!("expected Bv"),
        }
    }

    pub fn as_f64(self) -> f64 {
        match self {
            ConcreteVal::Fp { fmt: FpFmt::F64, bits } => f64::from_bits(bits),
            ConcreteVal::Fp { fmt: FpFmt::F32, bits } => f32::from_bits(bits as u32) as f64,
            _ => panic!("expected Fp"),
        }
    }

    pub fn format_with_sign(self, signed: bool) -> String {
        match self {
            ConcreteVal::Bool(b) => format!("{b}"),
            ConcreteVal::Bv { width, bits } => {
                if signed {
                    let s = sign_extend_128(bits, width);
                    format!("{s} (0x{bits:x} : i{width})")
                } else {
                    format!("{bits} (0x{bits:x} : u{width})")
                }
            }
            ConcreteVal::Fp { fmt: FpFmt::F32, bits } => {
                format!("{} (0x{:08x} : f32)", f32::from_bits(bits as u32), bits as u32)
            }
            ConcreteVal::Fp { fmt: FpFmt::F64, bits } => {
                format!("{} (0x{:016x} : f64)", f64::from_bits(bits), bits)
            }
        }
    }
}

pub fn sign_extend_128(bits: u128, width: u16) -> i128 {
    if width == 0 {
        return 0;
    }
    if width >= 128 {
        return bits as i128;
    }
    let shift = 128 - width;
    ((bits << shift) as i128) >> shift
}

/// Evaluate `target` in `graph` under concrete parameter assignments.
/// Used to independently validate every counterexample returned by any prover.
pub fn eval_node(graph: &SemGraph, target: NodeId, args: &[ConcreteVal]) -> ConcreteVal {
    let mut memo: Vec<Option<ConcreteVal>> = vec![None; graph.len()];
    fn go(
        g: &SemGraph,
        id: NodeId,
        args: &[ConcreteVal],
        memo: &mut [Option<ConcreteVal>],
    ) -> ConcreteVal {
        if let Some(v) = memo[id.0 as usize] {
            return v;
        }
        let val = match g.op(id) {
            SemOp::BoolConst(b) => ConcreteVal::Bool(*b),
            SemOp::BvConst { width, bits } => ConcreteVal::Bv { width: *width, bits: *bits },
            SemOp::FpConst { fmt, bits } => ConcreteVal::Fp { fmt: *fmt, bits: *bits },
            SemOp::Param { index, .. } => args[*index],
            SemOp::Not(a) => ConcreteVal::Bool(!go(g, *a, args, memo).as_bool()),
            SemOp::And(a, b) => {
                ConcreteVal::Bool(go(g, *a, args, memo).as_bool() && go(g, *b, args, memo).as_bool())
            }
            SemOp::Or(a, b) => {
                ConcreteVal::Bool(go(g, *a, args, memo).as_bool() || go(g, *b, args, memo).as_bool())
            }
            SemOp::Xor(a, b) => {
                ConcreteVal::Bool(go(g, *a, args, memo).as_bool() ^ go(g, *b, args, memo).as_bool())
            }
            SemOp::Iff(a, b) => {
                ConcreteVal::Bool(go(g, *a, args, memo).as_bool() == go(g, *b, args, memo).as_bool())
            }
            SemOp::Implies(a, b) => {
                ConcreteVal::Bool(!go(g, *a, args, memo).as_bool() || go(g, *b, args, memo).as_bool())
            }
            SemOp::Ite(c, t, e) => {
                if go(g, *c, args, memo).as_bool() {
                    go(g, *t, args, memo)
                } else {
                    go(g, *e, args, memo)
                }
            }
            SemOp::BvNot(a) => {
                let (w, x) = go(g, *a, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: (!x) & SemGraph::mask(w) }
            }
            SemOp::BvNeg(a) => {
                let (w, x) = go(g, *a, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: x.wrapping_neg() & SemGraph::mask(w) }
            }
            SemOp::BvExtract { x, hi, lo } => {
                let (_, v) = go(g, *x, args, memo).as_bv();
                let out_w = hi - lo + 1;
                ConcreteVal::Bv { width: out_w, bits: (v >> lo) & SemGraph::mask(out_w) }
            }
            SemOp::BvConcat(a, b) => {
                let (wa, va) = go(g, *a, args, memo).as_bv();
                let (wb, vb) = go(g, *b, args, memo).as_bv();
                let w = wa + wb;
                let bits = ((va << wb) | vb) & SemGraph::mask(w);
                ConcreteVal::Bv { width: w, bits }
            }
            SemOp::BvZExt { x, to } => {
                let (_, v) = go(g, *x, args, memo).as_bv();
                ConcreteVal::Bv { width: *to, bits: v & SemGraph::mask(*to) }
            }
            SemOp::BvSExt { x, to } => {
                let (from, v) = go(g, *x, args, memo).as_bv();
                let s = sign_extend_128(v, from) as u128;
                ConcreteVal::Bv { width: *to, bits: s & SemGraph::mask(*to) }
            }
            SemOp::BvBitReverse(a) => {
                let (w, v) = go(g, *a, args, memo).as_bv();
                let rev = v.reverse_bits() >> (128 - w);
                ConcreteVal::Bv { width: w, bits: rev & SemGraph::mask(w) }
            }
            SemOp::BvBSwap(a) => {
                let (w, v) = go(g, *a, args, memo).as_bv();
                let bytes = (w / 8) as usize;
                let mut out = 0u128;
                for i in 0..bytes {
                    let b = (v >> (i * 8)) & 0xff;
                    out |= b << ((bytes - 1 - i) * 8);
                }
                ConcreteVal::Bv { width: w, bits: out }
            }
            SemOp::BvCtPop(a) => {
                let (w, v) = go(g, *a, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: (v & SemGraph::mask(w)).count_ones() as u128 }
            }
            SemOp::BvCtlz(a) => {
                let (w, v) = go(g, *a, args, memo).as_bv();
                let m = v & SemGraph::mask(w);
                let lz = if m == 0 { w as u32 } else { m.leading_zeros() - (128 - w as u32) };
                ConcreteVal::Bv { width: w, bits: lz as u128 }
            }
            SemOp::BvCttz(a) => {
                let (w, v) = go(g, *a, args, memo).as_bv();
                let m = v & SemGraph::mask(w);
                let tz = if m == 0 { w as u32 } else { m.trailing_zeros() };
                ConcreteVal::Bv { width: w, bits: tz as u128 }
            }
            SemOp::BvAdd(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: va.wrapping_add(vb) & SemGraph::mask(w) }
            }
            SemOp::BvSub(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: va.wrapping_sub(vb) & SemGraph::mask(w) }
            }
            SemOp::BvMul(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: va.wrapping_mul(vb) & SemGraph::mask(w) }
            }
            SemOp::BvClmul(a, b) => {
                let (_, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: 128, bits: eval_bv_clmul(va as u64, vb as u64) }
            }
            SemOp::AesCore(kind, a) => {
                let (_, va) = go(g, *a, args, memo).as_bv();
                ConcreteVal::Bv { width: 128, bits: eval_aes_core(*kind, va) }
            }
            SemOp::BvUDiv(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let q = if vb == 0 { SemGraph::mask(w) } else { va / vb };
                ConcreteVal::Bv { width: w, bits: q & SemGraph::mask(w) }
            }
            SemOp::BvURem(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let r = if vb == 0 { va } else { va % vb };
                ConcreteVal::Bv { width: w, bits: r & SemGraph::mask(w) }
            }
            SemOp::BvSDiv(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let sa = sign_extend_128(va, w);
                let sb = sign_extend_128(vb, w);
                let q = if sb == 0 {
                    if sa >= 0 { -1i128 } else { 1i128 }
                } else {
                    sa.wrapping_div(sb)
                };
                ConcreteVal::Bv { width: w, bits: (q as u128) & SemGraph::mask(w) }
            }
            SemOp::BvSRem(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let sa = sign_extend_128(va, w);
                let sb = sign_extend_128(vb, w);
                let r = if sb == 0 { sa } else { sa.wrapping_rem(sb) };
                ConcreteVal::Bv { width: w, bits: (r as u128) & SemGraph::mask(w) }
            }
            SemOp::BvAnd(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: (va & vb) & SemGraph::mask(w) }
            }
            SemOp::BvOr(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: (va | vb) & SemGraph::mask(w) }
            }
            SemOp::BvXor(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bv { width: w, bits: (va ^ vb) & SemGraph::mask(w) }
            }
            SemOp::BvShl(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let bits = if vb >= w as u128 { 0 } else { (va << (vb as u32)) & SemGraph::mask(w) };
                ConcreteVal::Bv { width: w, bits }
            }
            SemOp::BvLShr(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let bits = if vb >= w as u128 { 0 } else { (va >> (vb as u32)) & SemGraph::mask(w) };
                ConcreteVal::Bv { width: w, bits }
            }
            SemOp::BvAShr(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let sa = sign_extend_128(va, w);
                let shift = vb.min((w as u128).saturating_sub(1)) as u32;
                let bits = ((sa >> shift) as u128) & SemGraph::mask(w);
                ConcreteVal::Bv { width: w, bits }
            }
            SemOp::BvFshl(a, b, s) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let (_, vs) = go(g, *s, args, memo).as_bv();
                let sh = (vs % (w as u128)) as u32;
                let res = if sh == 0 {
                    va
                } else {
                    ((va << sh) | (vb >> (w as u32 - sh))) & SemGraph::mask(w)
                };
                ConcreteVal::Bv { width: w, bits: res }
            }
            SemOp::BvFshr(a, b, s) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                let (_, vs) = go(g, *s, args, memo).as_bv();
                let sh = (vs % (w as u128)) as u32;
                let res = if sh == 0 {
                    vb
                } else {
                    ((va << (w as u32 - sh)) | (vb >> sh)) & SemGraph::mask(w)
                };
                ConcreteVal::Bv { width: w, bits: res }
            }
            SemOp::BvEq(a, b) => {
                let (_, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bool(va == vb)
            }
            SemOp::BvULt(a, b) => {
                let (_, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bool(va < vb)
            }
            SemOp::BvULe(a, b) => {
                let (_, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bool(va <= vb)
            }
            SemOp::BvSLt(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bool(sign_extend_128(va, w) < sign_extend_128(vb, w))
            }
            SemOp::BvSLe(a, b) => {
                let (w, va) = go(g, *a, args, memo).as_bv();
                let (_, vb) = go(g, *b, args, memo).as_bv();
                ConcreteVal::Bool(sign_extend_128(va, w) <= sign_extend_128(vb, w))
            }
            SemOp::FpNeg(a) => match go(g, *a, args, memo) {
                ConcreteVal::Fp { fmt: FpFmt::F32, bits } => {
                    ConcreteVal::Fp { fmt: FpFmt::F32, bits: (-(f32::from_bits(bits as u32))).to_bits() as u64 }
                }
                ConcreteVal::Fp { fmt: FpFmt::F64, bits } => {
                    ConcreteVal::Fp { fmt: FpFmt::F64, bits: (-(f64::from_bits(bits))).to_bits() }
                }
                _ => unreachable!(),
            },
            SemOp::FpAdd(a, b) => eval_fp_bin(go(g, *a, args, memo), go(g, *b, args, memo), |x, y| x + y, |x, y| x + y),
            SemOp::FpSub(a, b) => eval_fp_bin(go(g, *a, args, memo), go(g, *b, args, memo), |x, y| x - y, |x, y| x - y),
            SemOp::FpMul(a, b) => eval_fp_bin(go(g, *a, args, memo), go(g, *b, args, memo), |x, y| x * y, |x, y| x * y),
            SemOp::FpDiv(a, b) => eval_fp_bin(go(g, *a, args, memo), go(g, *b, args, memo), |x, y| x / y, |x, y| x / y),
            SemOp::FpCmp(pred, a, b) => {
                let x = go(g, *a, args, memo).as_f64();
                let y = go(g, *b, args, memo).as_f64();
                let ord = !x.is_nan() && !y.is_nan();
                let res = match pred {
                    FpPred::False => false,
                    FpPred::True => true,
                    FpPred::Ord => ord,
                    FpPred::Uno => !ord,
                    FpPred::Oeq => ord && x == y,
                    FpPred::Ogt => ord && x > y,
                    FpPred::Oge => ord && x >= y,
                    FpPred::Olt => ord && x < y,
                    FpPred::Ole => ord && x <= y,
                    FpPred::One => ord && x != y,
                    FpPred::Ueq => !ord || x == y,
                    FpPred::Ugt => !ord || x > y,
                    FpPred::Uge => !ord || x >= y,
                    FpPred::Ult => !ord || x < y,
                    FpPred::Ule => !ord || x <= y,
                    FpPred::Une => !ord || x != y,
                };
                ConcreteVal::Bool(res)
            }
            SemOp::FpToBvBits(a) => match go(g, *a, args, memo) {
                ConcreteVal::Fp { fmt, bits } => ConcreteVal::Bv { width: fmt.bit_width(), bits: bits as u128 },
                _ => unreachable!(),
            },
            SemOp::BvBitsToFp(fmt, a) => {
                let (_, bits) = go(g, *a, args, memo).as_bv();
                ConcreteVal::Fp { fmt: *fmt, bits: bits as u64 }
            }
            SemOp::BvToFpSigned(fmt, a) => {
                let (w, bits) = go(g, *a, args, memo).as_bv();
                let s = sign_extend_128(bits, w);
                match fmt {
                    FpFmt::F32 => ConcreteVal::Fp { fmt: *fmt, bits: (s as f32).to_bits() as u64 },
                    FpFmt::F64 => ConcreteVal::Fp { fmt: *fmt, bits: (s as f64).to_bits() },
                }
            }
            SemOp::BvToFpUnsigned(fmt, a) => {
                let (_, bits) = go(g, *a, args, memo).as_bv();
                match fmt {
                    FpFmt::F32 => ConcreteVal::Fp { fmt: *fmt, bits: (bits as f32).to_bits() as u64 },
                    FpFmt::F64 => ConcreteVal::Fp { fmt: *fmt, bits: (bits as f64).to_bits() },
                }
            }
            SemOp::FpToBvSigned(w, a) => {
                let x = go(g, *a, args, memo).as_f64();
                let v = (x.trunc() as i128 as u128) & SemGraph::mask(*w);
                ConcreteVal::Bv { width: *w, bits: v }
            }
            SemOp::FpToBvUnsigned(w, a) => {
                let x = go(g, *a, args, memo).as_f64();
                let v = (x.trunc() as u128) & SemGraph::mask(*w);
                ConcreteVal::Bv { width: *w, bits: v }
            }
            SemOp::FpToFp(fmt, a) => {
                let x = go(g, *a, args, memo).as_f64();
                match fmt {
                    FpFmt::F32 => ConcreteVal::Fp { fmt: *fmt, bits: (x as f32).to_bits() as u64 },
                    FpFmt::F64 => ConcreteVal::Fp { fmt: *fmt, bits: x.to_bits() },
                }
            }
            SemOp::FpUnary(op, a) => match go(g, *a, args, memo) {
                ConcreteVal::Fp { fmt: FpFmt::F32, bits } => {
                    let x = f32::from_bits(bits as u32);
                    let r = match op {
                        FpUnaryOp::Abs => x.abs(),
                        FpUnaryOp::Sqrt => x.sqrt(),
                        FpUnaryOp::Floor => x.floor(),
                        FpUnaryOp::Ceil => x.ceil(),
                        FpUnaryOp::Trunc => x.trunc(),
                        FpUnaryOp::Round => x.round(),
                        FpUnaryOp::RoundEven => x.round_ties_even(),
                    };
                    ConcreteVal::Fp { fmt: FpFmt::F32, bits: r.to_bits() as u64 }
                }
                ConcreteVal::Fp { fmt: FpFmt::F64, bits } => {
                    let x = f64::from_bits(bits);
                    let r = match op {
                        FpUnaryOp::Abs => x.abs(),
                        FpUnaryOp::Sqrt => x.sqrt(),
                        FpUnaryOp::Floor => x.floor(),
                        FpUnaryOp::Ceil => x.ceil(),
                        FpUnaryOp::Trunc => x.trunc(),
                        FpUnaryOp::Round => x.round(),
                        FpUnaryOp::RoundEven => x.round_ties_even(),
                    };
                    ConcreteVal::Fp { fmt: FpFmt::F64, bits: r.to_bits() }
                }
                _ => unreachable!(),
            },
            SemOp::FpCopySign(a, b) => match (go(g, *a, args, memo), go(g, *b, args, memo)) {
                (ConcreteVal::Fp { fmt: FpFmt::F32, bits: ba }, ConcreteVal::Fp { bits: bb, .. }) => {
                    let r = f32::from_bits(ba as u32).copysign(f32::from_bits(bb as u32));
                    ConcreteVal::Fp { fmt: FpFmt::F32, bits: r.to_bits() as u64 }
                }
                (ConcreteVal::Fp { fmt: FpFmt::F64, bits: ba }, ConcreteVal::Fp { bits: bb, .. }) => {
                    let r = f64::from_bits(ba).copysign(f64::from_bits(bb));
                    ConcreteVal::Fp { fmt: FpFmt::F64, bits: r.to_bits() }
                }
                _ => unreachable!(),
            },
            SemOp::FpFitsSigned(w, a) => {
                let x = go(g, *a, args, memo).as_f64();
                let fits = if !x.is_finite() {
                    false
                } else {
                    let t = x.trunc();
                    let lo = -((1u128 << (w - 1)) as f64);
                    let hi = ((1u128 << (w - 1)) - 1) as f64;
                    t >= lo && t <= hi
                };
                ConcreteVal::Bool(fits)
            }
            SemOp::FpFitsUnsigned(w, a) => {
                let x = go(g, *a, args, memo).as_f64();
                let fits = if !x.is_finite() {
                    false
                } else {
                    let t = x.trunc();
                    let hi = if *w >= 64 { u64::MAX as f64 } else { ((1u64 << w) - 1) as f64 };
                    t >= 0.0 && t <= hi
                };
                ConcreteVal::Bool(fits)
            }
            SemOp::FpIsNaN(a) => {
                let x = go(g, *a, args, memo).as_f64();
                ConcreteVal::Bool(x.is_nan())
            }
            SemOp::FpIsFinite(a) => {
                let x = go(g, *a, args, memo).as_f64();
                ConcreteVal::Bool(x.is_finite())
            }
        };
        memo[id.0 as usize] = Some(val);
        val
    }
    go(graph, target, args, &mut memo)
}

fn eval_fp_bin(
    a: ConcreteVal,
    b: ConcreteVal,
    f32_op: impl FnOnce(f32, f32) -> f32,
    f64_op: impl FnOnce(f64, f64) -> f64,
) -> ConcreteVal {
    match (a, b) {
        (ConcreteVal::Fp { fmt: FpFmt::F32, bits: ba }, ConcreteVal::Fp { bits: bb, .. }) => {
            ConcreteVal::Fp {
                fmt: FpFmt::F32,
                bits: f32_op(f32::from_bits(ba as u32), f32::from_bits(bb as u32)).to_bits() as u64,
            }
        }
        (ConcreteVal::Fp { fmt: FpFmt::F64, bits: ba }, ConcreteVal::Fp { bits: bb, .. }) => {
            ConcreteVal::Fp {
                fmt: FpFmt::F64,
                bits: f64_op(f64::from_bits(ba), f64::from_bits(bb)).to_bits(),
            }
        }
        _ => unreachable!(),
    }
}

/// Exact 64x64 -> 128-bit GF(2) carry-less multiplication (`pclmulqdq` / `pmull`).
pub fn eval_bv_clmul(a: u64, b: u64) -> u128 {
    let mut acc = 0u128;
    let a128 = a as u128;
    for i in 0..64 {
        if ((b >> i) & 1) != 0 {
            acc ^= a128 << i;
        }
    }
    acc
}

const AES_SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

const AES_INV_SBOX: [u8; 256] = [
    0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
    0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
    0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
    0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
    0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
    0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
    0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
    0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
    0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
    0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
    0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
    0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
    0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
    0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
    0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
    0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d,
];

fn gf_xtime(x: u8) -> u8 {
    (x << 1) ^ if (x & 0x80) != 0 { 0x1b } else { 0 }
}

fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0u8;
    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        a = gf_xtime(a);
        b >>= 1;
    }
    p
}

/// Exact FIPS-197 AES round core transformation (state stored in little-endian byte order,
/// matching `<16 x i8>` / `i128` bitcast on x86_64 AES-NI & aarch64 ARMv8 Crypto).
pub fn eval_aes_core(kind: AesRoundKind, state_bits: u128) -> u128 {
    let mut s = state_bits.to_le_bytes();
    match kind {
        AesRoundKind::EncCore | AesRoundKind::EncLastCore => {
            for b in &mut s {
                *b = AES_SBOX[*b as usize];
            }
            // ShiftRows (column-major 4x4 matrix):
            let t = s;
            s[1] = t[5]; s[5] = t[9]; s[9] = t[13]; s[13] = t[1];
            s[2] = t[10]; s[6] = t[14]; s[10] = t[2]; s[14] = t[6];
            s[3] = t[15]; s[7] = t[3]; s[11] = t[7]; s[15] = t[11];
            if kind == AesRoundKind::EncCore {
                for c in 0..4 {
                    let i = c * 4;
                    let (a0, a1, a2, a3) = (s[i], s[i + 1], s[i + 2], s[i + 3]);
                    s[i]     = gf_mul(a0, 2) ^ gf_mul(a1, 3) ^ a2 ^ a3;
                    s[i + 1] = a0 ^ gf_mul(a1, 2) ^ gf_mul(a2, 3) ^ a3;
                    s[i + 2] = a0 ^ a1 ^ gf_mul(a2, 2) ^ gf_mul(a3, 3);
                    s[i + 3] = gf_mul(a0, 3) ^ a1 ^ a2 ^ gf_mul(a3, 2);
                }
            }
        }
        AesRoundKind::DecCore | AesRoundKind::DecLastCore => {
            // InvShiftRows:
            let t = s;
            s[1] = t[13]; s[5] = t[1]; s[9] = t[5]; s[13] = t[9];
            s[2] = t[10]; s[6] = t[14]; s[10] = t[2]; s[14] = t[6];
            s[3] = t[7]; s[7] = t[11]; s[11] = t[15]; s[15] = t[3];
            for b in &mut s {
                *b = AES_INV_SBOX[*b as usize];
            }
            if kind == AesRoundKind::DecCore {
                for c in 0..4 {
                    let i = c * 4;
                    let (a0, a1, a2, a3) = (s[i], s[i + 1], s[i + 2], s[i + 3]);
                    s[i]     = gf_mul(a0, 14) ^ gf_mul(a1, 11) ^ gf_mul(a2, 13) ^ gf_mul(a3, 9);
                    s[i + 1] = gf_mul(a0, 9) ^ gf_mul(a1, 14) ^ gf_mul(a2, 11) ^ gf_mul(a3, 13);
                    s[i + 2] = gf_mul(a0, 13) ^ gf_mul(a1, 9) ^ gf_mul(a2, 14) ^ gf_mul(a3, 11);
                    s[i + 3] = gf_mul(a0, 11) ^ gf_mul(a1, 13) ^ gf_mul(a2, 9) ^ gf_mul(a3, 14);
                }
            }
        }
    }
    u128::from_le_bytes(s)
}
