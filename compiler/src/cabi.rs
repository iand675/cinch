//! The C calling conventions' rules for aggregates crossing by value
//! (foreign function interface specification section 2; the plan note
//! `design/llvm-contract-attributes.md` item 6).
//!
//! A `#[byval]` parameter or `#[sret]` result is a C struct passed or
//! returned *by value*. LLVM does not lower that for us: clang decides,
//! per target, whether the struct travels in registers (and as which
//! register-sized pieces), as a stack copy, or behind a pointer, and
//! hands the backend the result of that decision. A foreign call from
//! Cinch has to make the same decision, or the two sides disagree about
//! where the bytes are. The decision is a function of two things only:
//!
//! * the target's convention ([`Convention`]), and
//! * the aggregate's machine scalars ([`CLeaf`], recorded by the
//!   checker), plus, on SysV, the registers the other arguments use.
//!
//! [`classify`] makes it once per binding, and code generation spells the
//! declare, the call and the export entry from the [`Crossing`] it
//! returns, so the three cannot drift apart.

use crate::core::{AbiClass, AbiParam, CLeaf, ForeignPlan, TargetProfile};

/// The C calling convention family a target uses for by-value
/// aggregates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Convention {
    /// The System V AMD64 ABI (x86-64 Linux and Darwin): eightbyte
    /// classification into INTEGER and SSE registers, MEMORY beyond 16
    /// bytes or when the registers run out.
    SysV64,
    /// The Arm 64-bit procedure call standard (AArch64 Linux and Darwin):
    /// homogeneous floating-point aggregates in vector registers, other
    /// composites of 16 bytes or less in general registers, larger ones
    /// by reference to a caller copy.
    Aapcs64,
    /// The Microsoft x64 convention: 1, 2, 4 and 8-byte aggregates as an
    /// integer of that size, every other size by reference.
    Win64,
    /// The WebAssembly C ABI (clang's basic ABI for wasm32): a
    /// single-scalar aggregate as that scalar, every other aggregate as a
    /// `byval` copy, and a result through a result buffer.
    Wasm32,
}

impl Convention {
    /// The convention a target's by-value aggregates follow, or `None`
    /// for a target this compiler has no rule for (a by-value crossing
    /// there is refused, not guessed).
    pub fn of(profile: &TargetProfile) -> Option<Convention> {
        match (profile.arch.as_str(), profile.os.as_str()) {
            ("x86_64", "windows") => Some(Convention::Win64),
            ("x86_64", _) => Some(Convention::SysV64),
            ("aarch64", _) => Some(Convention::Aapcs64),
            ("wasm32", _) => Some(Convention::Wasm32),
            _ => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Convention::SysV64 => "System V x86-64",
            Convention::Aapcs64 => "AAPCS64",
            Convention::Win64 => "Microsoft x64",
            Convention::Wasm32 => "WebAssembly C",
        }
    }
}

/// One register-sized piece of an aggregate passed or returned directly:
/// the LLVM type it travels as, and the byte range of the aggregate's
/// copy it is loaded from or stored to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub offset: u64,
    pub ty: String,
    pub bytes: u64,
}

/// How one by-value aggregate parameter crosses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParamPass {
    /// In registers: one LLVM argument per piece, each loaded from the
    /// call-scoped copy.
    Direct(Vec<Piece>),
    /// As a copy in the argument area: LLVM's `byval`, a pointer in the
    /// IR whose pointee the backend copies onto the stack (SysV MEMORY,
    /// and the WebAssembly ABI's indirect aggregate).
    Byval,
    /// As a plain pointer to the caller's copy (AAPCS64 and Microsoft x64
    /// composites the convention passes by reference).
    Reference,
}

/// How one by-value aggregate result crosses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResultPass {
    /// In registers, as one LLVM first-class value stored at the base of
    /// the result buffer.
    Direct { ty: String, bytes: u64 },
    /// Through a caller-provided result buffer, LLVM's `sret`.
    Sret,
}

/// One binding's by-value decisions: per parameter (in `plan.params`
/// order, `None` for a parameter that is not a by-value aggregate) and
/// for the result (`None` when it is not an aggregate).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Crossing {
    pub params: Vec<Option<ParamPass>>,
    pub result: Option<ResultPass>,
}

impl ParamPass {
    /// The bytes the call-scoped copy needs: the aggregate itself, and
    /// enough beyond it that every piece's load stays inside the copy (a
    /// 12-byte struct travels as two eightbytes).
    pub fn buffer_bytes(&self, p: &AbiParam) -> u64 {
        match self {
            ParamPass::Direct(pieces) => pieces_end(pieces).max(p.width),
            ParamPass::Byval | ParamPass::Reference => p.width,
        }
    }
}

impl ResultPass {
    /// The bytes the result buffer needs (see [`ParamPass::buffer_bytes`]).
    pub fn buffer_bytes(&self, p: &AbiParam) -> u64 {
        match self {
            ResultPass::Direct { bytes, .. } => (*bytes).max(p.width),
            ResultPass::Sret => p.width,
        }
    }
}

fn pieces_end(pieces: &[Piece]) -> u64 {
    pieces.iter().map(|p| p.offset + p.bytes).max().unwrap_or(0)
}

fn by_value(p: &AbiParam) -> bool {
    matches!(p.class, AbiClass::Aggregate) && p.by_value
}

/// Decide every by-value crossing of one binding for one convention.
///
/// A binding with no by-value aggregate needs no convention and always
/// classifies. One with a by-value aggregate is refused, with the reason,
/// when the target has no convention here or the aggregate holds a
/// scalar the rules below do not place (a 16-byte integer, a leaf the
/// recorded layout does not align); refusing is the alternative to
/// guessing a placement the C side does not share.
pub fn classify(plan: &ForeignPlan, convention: Option<Convention>) -> Result<Crossing, String> {
    let result_agg = matches!(plan.result.class, AbiClass::Aggregate);
    if !result_agg && !plan.params.iter().any(by_value) {
        return Ok(Crossing { params: vec![None; plan.params.len()], result: None });
    }
    let Some(convention) = convention else {
        return Err(format!(
            "`{}` passes a C aggregate by value, and this target has no by-value aggregate convention here \
             (x86-64 System V and Microsoft x64, AAPCS64 and wasm32 do); cross it by pointer instead",
            plan.symbol
        ));
    };
    let refuse = |p: &AbiParam, why: &str| {
        format!(
            "`{}`: `{}` cannot cross by value under the {} convention: {why}",
            plan.symbol,
            p.ty,
            convention.name()
        )
    };
    let result = if result_agg {
        Some(classify_result(convention, &plan.result).map_err(|why| refuse(&plan.result, &why))?)
    } else {
        None
    };
    let params = match convention {
        Convention::SysV64 => sysv_params(plan, result.as_ref(), &refuse)?,
        _ => plan
            .params
            .iter()
            .map(|p| {
                if !by_value(p) {
                    return Ok(None);
                }
                classify_param(convention, p).map(Some).map_err(|why| refuse(p, &why))
            })
            .collect::<Result<_, String>>()?,
    };
    Ok(Crossing { params, result })
}

fn classify_param(convention: Convention, p: &AbiParam) -> Result<ParamPass, String> {
    match convention {
        Convention::SysV64 => unreachable!("SysV parameters classify with their register budget"),
        Convention::Aapcs64 => {
            if let Some(ty) = hfa(p) {
                return Ok(ParamPass::Direct(vec![Piece { offset: 0, ty, bytes: p.width }]));
            }
            if p.width > 16 {
                return Ok(ParamPass::Reference);
            }
            no_wide_leaf(p)?;
            // clang's spelling: one `i64` up to 8 bytes, a pair as
            // `[2 x i64]`, which the backend keeps in consecutive
            // registers or moves wholly to the stack (AAPCS64 C.12).
            let ty = if p.width <= 8 { "i64" } else { "[2 x i64]" };
            Ok(ParamPass::Direct(vec![Piece {
                offset: 0,
                ty: ty.to_string(),
                bytes: p.width.div_ceil(8) * 8,
            }]))
        }
        Convention::Win64 => Ok(match win64_int(p) {
            Some(piece) => ParamPass::Direct(vec![piece]),
            None => ParamPass::Reference,
        }),
        Convention::Wasm32 => Ok(match single_scalar(p) {
            Some(piece) => ParamPass::Direct(vec![piece]),
            None => ParamPass::Byval,
        }),
    }
}

fn classify_result(convention: Convention, p: &AbiParam) -> Result<ResultPass, String> {
    let direct = |ty: String, bytes: u64| ResultPass::Direct { ty, bytes };
    match convention {
        Convention::SysV64 => Ok(match sysv_eightbytes(p)? {
            None => ResultPass::Sret,
            Some(classes) => {
                let pieces = sysv_pieces(p, &classes);
                let bytes = pieces_end(&pieces);
                if pieces.len() == 1 {
                    direct(pieces[0].ty.clone(), bytes)
                } else {
                    let tys: Vec<&str> = pieces.iter().map(|p| p.ty.as_str()).collect();
                    // `{ T0, T1 }` lays its second member at offset 8 for
                    // every pair this produces (each member is at most 8
                    // bytes wide and 8-byte members are 8-aligned).
                    direct(format!("{{ {} }}", tys.join(", ")), 16)
                }
            }
        }),
        Convention::Aapcs64 => {
            if let Some(ty) = hfa(p) {
                return Ok(direct(ty, p.width));
            }
            if p.width > 16 {
                return Ok(ResultPass::Sret);
            }
            no_wide_leaf(p)?;
            // clang's spelling for both Linux and Darwin: `x0`, or the
            // `x0`/`x1` pair as `[2 x i64]`.
            Ok(if p.width <= 8 {
                direct("i64".to_string(), 8)
            } else {
                direct("[2 x i64]".to_string(), 16)
            })
        }
        Convention::Win64 => Ok(match win64_int(p) {
            Some(piece) => direct(piece.ty, piece.bytes),
            None => ResultPass::Sret,
        }),
        Convention::Wasm32 => Ok(match single_scalar(p) {
            Some(piece) => direct(piece.ty, piece.bytes),
            None => ResultPass::Sret,
        }),
    }
}

// ------------------------------------------------------------ System V

/// The class of one SysV eightbyte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Eightbyte {
    Integer,
    Sse,
}

/// The SysV classification of an aggregate: its eightbytes' classes, or
/// `None` for MEMORY (more than 16 bytes, or a leaf the layout does not
/// align naturally, which is how `repr(packed)` reaches MEMORY).
fn sysv_eightbytes(p: &AbiParam) -> Result<Option<Vec<Eightbyte>>, String> {
    if p.width > 16 {
        return Ok(None);
    }
    if p.leaves.iter().any(|l| l.size > 0 && l.offset % l.size != 0) {
        return Ok(None);
    }
    no_wide_leaf(p)?;
    let mut classes: Vec<Option<Eightbyte>> = vec![None; p.width.div_ceil(8) as usize];
    for leaf in &p.leaves {
        // A naturally aligned leaf of at most 8 bytes sits in one
        // eightbyte. INTEGER wins a merge; SSE needs every leaf SSE.
        let slot = &mut classes[(leaf.offset / 8) as usize];
        *slot = Some(match (*slot, leaf.float) {
            (Some(Eightbyte::Integer), _) | (_, false) => Eightbyte::Integer,
            _ => Eightbyte::Sse,
        });
    }
    classes
        .into_iter()
        .map(|c| c.ok_or_else(|| "an eightbyte holds only padding".to_string()))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

/// The LLVM pieces of a register-classified SysV aggregate, clang's
/// spelling: an INTEGER eightbyte is `i64`, an SSE eightbyte `double`
/// for an `F64`, `<2 x float>` for two `F32`s and `float` for one.
fn sysv_pieces(p: &AbiParam, classes: &[Eightbyte]) -> Vec<Piece> {
    classes
        .iter()
        .enumerate()
        .map(|(i, class)| {
            let base = i as u64 * 8;
            let (ty, bytes) = match class {
                Eightbyte::Integer => ("i64", 8),
                Eightbyte::Sse => {
                    let inside: Vec<&CLeaf> = p
                        .leaves
                        .iter()
                        .filter(|l| l.offset / 8 == i as u64)
                        .collect();
                    if inside.iter().any(|l| l.size == 8) {
                        ("double", 8)
                    } else if inside.iter().any(|l| l.offset - base >= 4) {
                        ("<2 x float>", 8)
                    } else {
                        ("float", 4)
                    }
                }
            };
            Piece { offset: base, ty: ty.to_string(), bytes }
        })
        .collect()
}

/// SysV parameters in order, with the register budget clang keeps: six
/// INTEGER and eight SSE registers, a hidden result pointer taking the
/// first INTEGER one. A register-classified aggregate whose eightbytes do
/// not all fit in what is left goes to MEMORY whole (the ABI never splits
/// one between registers and the stack); one that fits takes them.
fn sysv_params(
    plan: &ForeignPlan,
    result: Option<&ResultPass>,
    refuse: &dyn Fn(&AbiParam, &str) -> String,
) -> Result<Vec<Option<ParamPass>>, String> {
    let mut int_free: u32 = 6;
    let mut sse_free: u32 = 8;
    if matches!(result, Some(ResultPass::Sret)) {
        int_free -= 1;
    }
    let mut passes = Vec::with_capacity(plan.params.len());
    for p in &plan.params {
        match &p.class {
            AbiClass::Unit => passes.push(None),
            AbiClass::Scalar(s) if s.is_float() => {
                sse_free = sse_free.saturating_sub(1);
                passes.push(None);
            }
            AbiClass::Scalar(_) | AbiClass::Ptr => {
                int_free = int_free.saturating_sub(1);
                passes.push(None);
            }
            AbiClass::Aggregate if !p.by_value => {
                int_free = int_free.saturating_sub(1);
                passes.push(None);
            }
            AbiClass::Aggregate => {
                let pass = match sysv_eightbytes(p).map_err(|why| refuse(p, &why))? {
                    None => ParamPass::Byval,
                    Some(classes) => {
                        let ints = classes.iter().filter(|c| **c == Eightbyte::Integer).count() as u32;
                        let sses = classes.len() as u32 - ints;
                        if ints <= int_free && sses <= sse_free {
                            int_free -= ints;
                            sse_free -= sses;
                            ParamPass::Direct(sysv_pieces(p, &classes))
                        } else {
                            ParamPass::Byval
                        }
                    }
                };
                passes.push(Some(pass));
            }
        }
    }
    Ok(passes)
}

// ------------------------------------------------------- the other rules

/// The AAPCS64 homogeneous floating-point aggregate spelling: one to four
/// floats of one width filling the aggregate contiguously, as
/// `[N x float]` or `[N x double]`.
fn hfa(p: &AbiParam) -> Option<String> {
    let first = p.leaves.first()?;
    let n = p.leaves.len() as u64;
    let uniform = p.leaves.iter().enumerate().all(|(i, l)| {
        l.float && l.size == first.size && l.offset == i as u64 * first.size
    });
    if !uniform || n > 4 || p.width != n * first.size {
        return None;
    }
    let elem = if first.size == 8 { "double" } else { "float" };
    Some(format!("[{n} x {elem}]"))
}

/// The Microsoft x64 integer spelling of a 1, 2, 4 or 8-byte aggregate.
fn win64_int(p: &AbiParam) -> Option<Piece> {
    matches!(p.width, 1 | 2 | 4 | 8).then(|| Piece {
        offset: 0,
        ty: format!("i{}", p.width * 8),
        bytes: p.width,
    })
}

/// The WebAssembly ABI's single-element aggregate: one scalar that is
/// the whole aggregate travels as that scalar.
fn single_scalar(p: &AbiParam) -> Option<Piece> {
    let [leaf] = p.leaves.as_slice() else { return None };
    if leaf.offset != 0 || leaf.size != p.width {
        return None;
    }
    let ty = match (leaf.float, leaf.size) {
        (true, 4) => "float".to_string(),
        (true, 8) => "double".to_string(),
        (_, size) => format!("i{}", size * 8),
    };
    Some(Piece { offset: 0, ty, bytes: leaf.size })
}

/// The register rules here place scalars of at most 8 bytes; a wider one
/// (a 16-byte integer, with its 16-byte alignment rules) is refused.
fn no_wide_leaf(p: &AbiParam) -> Result<(), String> {
    match p.leaves.iter().find(|l| l.size > 8) {
        Some(l) => Err(format!(
            "it holds a {}-byte scalar at offset {}, which the register rules here do not place",
            l.size, l.offset
        )),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(offset: u64, size: u64) -> CLeaf {
        CLeaf { offset, size, float: false }
    }
    fn float(offset: u64, size: u64) -> CLeaf {
        CLeaf { offset, size, float: true }
    }

    fn agg(width: u64, align: u64, leaves: Vec<CLeaf>) -> AbiParam {
        AbiParam {
            ty: "S".to_string(),
            class: AbiClass::Aggregate,
            role: "value".to_string(),
            allocator: String::new(),
            nullable: false,
            width,
            align,
            signed: false,
            pointee_align: 0,
            by_value: true,
            tag: 0,
            fields: Vec::new(),
            leaves,
        }
    }

    fn scalar(s: crate::types::Scalar) -> AbiParam {
        AbiParam { class: AbiClass::Scalar(s), by_value: false, leaves: Vec::new(), ..agg(8, 8, Vec::new()) }
    }

    fn unit() -> AbiParam {
        AbiParam { class: AbiClass::Unit, by_value: false, ..agg(0, 1, Vec::new()) }
    }

    fn plan(params: Vec<AbiParam>, result: AbiParam) -> ForeignPlan {
        ForeignPlan {
            symbol: "f".to_string(),
            export: false,
            unit: "Main.f".to_string(),
            convention: "ccall".to_string(),
            params,
            result,
            deallocator: String::new(),
            canon: None,
            blocking: false,
        }
    }

    fn param(c: Convention, p: AbiParam) -> ParamPass {
        classify(&plan(vec![p], unit()), Some(c)).unwrap().params[0].clone().unwrap()
    }

    fn result(c: Convention, p: AbiParam) -> ResultPass {
        classify(&plan(vec![], p), Some(c)).unwrap().result.unwrap()
    }

    fn tys(pass: &ParamPass) -> Vec<&str> {
        match pass {
            ParamPass::Direct(pieces) => pieces.iter().map(|p| p.ty.as_str()).collect(),
            other => panic!("expected a direct pass, got {other:?}"),
        }
    }

    fn big() -> AbiParam {
        agg(24, 8, vec![int(0, 8), int(8, 8), int(16, 8)])
    }

    #[test]
    fn sysv_memory_beyond_sixteen_bytes() {
        assert_eq!(param(Convention::SysV64, big()), ParamPass::Byval);
        assert_eq!(result(Convention::SysV64, big()), ResultPass::Sret);
    }

    #[test]
    fn sysv_eightbytes_follow_their_leaves() {
        let pair = agg(16, 8, vec![int(0, 8), int(8, 8)]);
        assert_eq!(tys(&param(Convention::SysV64, pair.clone())), ["i64", "i64"]);
        let mixed = agg(16, 8, vec![float(0, 8), int(8, 4)]);
        assert_eq!(tys(&param(Convention::SysV64, mixed)), ["double", "i64"]);
        // Three `F32`s: two share the first SSE eightbyte, the third is
        // alone in the second.
        let floats = agg(12, 4, vec![float(0, 4), float(4, 4), float(8, 4)]);
        assert_eq!(tys(&param(Convention::SysV64, floats.clone())), ["<2 x float>", "float"]);
        assert_eq!(
            result(Convention::SysV64, floats),
            ResultPass::Direct { ty: "{ <2 x float>, float }".to_string(), bytes: 16 }
        );
        // An integer and a float in one eightbyte make it INTEGER.
        let shared = agg(8, 4, vec![int(0, 4), float(4, 4)]);
        assert_eq!(tys(&param(Convention::SysV64, shared)), ["i64"]);
        assert_eq!(
            result(Convention::SysV64, pair),
            ResultPass::Direct { ty: "{ i64, i64 }".to_string(), bytes: 16 }
        );
    }

    #[test]
    fn sysv_packed_misalignment_is_memory() {
        let packed = agg(9, 1, vec![int(0, 1), int(1, 8)]);
        assert_eq!(param(Convention::SysV64, packed), ParamPass::Byval);
    }

    #[test]
    fn sysv_out_of_registers_is_memory_whole() {
        let pair = || agg(16, 8, vec![int(0, 8), int(8, 8)]);
        let i = || scalar(crate::types::Scalar::I64);
        // Five integer registers used, one left: a two-eightbyte struct
        // does not fit and goes to memory, and the scalar after it still
        // takes the last register.
        let p = plan(vec![i(), i(), i(), i(), i(), pair(), i()], unit());
        let c = classify(&p, Some(Convention::SysV64)).unwrap();
        assert_eq!(c.params[5], Some(ParamPass::Byval));
        // The hidden result pointer takes the first register.
        let p = plan(vec![i(), i(), i(), pair()], big());
        let c = classify(&p, Some(Convention::SysV64)).unwrap();
        assert!(matches!(c.params[3], Some(ParamPass::Direct(_))));
        let p = plan(vec![i(), i(), i(), i(), pair()], big());
        let c = classify(&p, Some(Convention::SysV64)).unwrap();
        assert_eq!(c.params[4], Some(ParamPass::Byval));
    }

    #[test]
    fn aapcs64_by_reference_beyond_sixteen_bytes_except_hfas() {
        assert_eq!(param(Convention::Aapcs64, big()), ParamPass::Reference);
        assert_eq!(result(Convention::Aapcs64, big()), ResultPass::Sret);
        let hfa = agg(24, 8, vec![float(0, 8), float(8, 8), float(16, 8)]);
        assert_eq!(tys(&param(Convention::Aapcs64, hfa.clone())), ["[3 x double]"]);
        assert_eq!(result(Convention::Aapcs64, hfa), ResultPass::Direct { ty: "[3 x double]".to_string(), bytes: 24 });
        let small = agg(12, 4, vec![int(0, 4), int(4, 4), int(8, 4)]);
        assert_eq!(tys(&param(Convention::Aapcs64, small.clone())), ["[2 x i64]"]);
        assert_eq!(small_buffer(&param(Convention::Aapcs64, small.clone()), &small), 16);
        assert_eq!(result(Convention::Aapcs64, small), ResultPass::Direct { ty: "[2 x i64]".to_string(), bytes: 16 });
        // Mixed widths are not homogeneous.
        let mixed = agg(16, 8, vec![float(0, 4), float(8, 8)]);
        assert_eq!(tys(&param(Convention::Aapcs64, mixed)), ["[2 x i64]"]);
    }

    fn small_buffer(pass: &ParamPass, p: &AbiParam) -> u64 {
        pass.buffer_bytes(p)
    }

    #[test]
    fn win64_powers_of_two_up_to_eight_are_integers() {
        let two_ints = agg(8, 4, vec![int(0, 4), int(4, 4)]);
        assert_eq!(tys(&param(Convention::Win64, two_ints.clone())), ["i64"]);
        assert_eq!(result(Convention::Win64, two_ints), ResultPass::Direct { ty: "i64".to_string(), bytes: 8 });
        let pair = agg(16, 8, vec![int(0, 8), int(8, 8)]);
        assert_eq!(param(Convention::Win64, pair.clone()), ParamPass::Reference);
        assert_eq!(result(Convention::Win64, pair), ResultPass::Sret);
    }

    #[test]
    fn wasm32_single_scalars_are_direct() {
        let one = agg(4, 4, vec![float(0, 4)]);
        assert_eq!(tys(&param(Convention::Wasm32, one)), ["float"]);
        let two = agg(8, 4, vec![int(0, 4), int(4, 4)]);
        assert_eq!(param(Convention::Wasm32, two.clone()), ParamPass::Byval);
        assert_eq!(result(Convention::Wasm32, two), ResultPass::Sret);
    }

    #[test]
    fn no_convention_refuses_only_by_value_crossings() {
        let plain = plan(vec![scalar(crate::types::Scalar::I64)], unit());
        assert!(classify(&plain, None).is_ok());
        assert!(classify(&plan(vec![big()], unit()), None).is_err());
    }

    #[test]
    fn wide_leaves_are_refused_in_registers() {
        let wide = agg(16, 16, vec![int(0, 16)]);
        assert!(classify(&plan(vec![wide.clone()], unit()), Some(Convention::SysV64)).is_err());
        assert!(classify(&plan(vec![wide], unit()), Some(Convention::Aapcs64)).is_err());
    }
}
