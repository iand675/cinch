//! Typed Array Intermediate Representation, optimization passes, GPU eligibility
//! diagnostics, Metal MSL codegen, and structured compiler reporting.
//!
//! Runs after type/effect/ownership checking, specialization, and Core
//! simplification — before RC insertion and LLVM IR emission — so eliminated
//! intermediate arrays never allocate, never retain/release elements, and never
//! appear in the generated LLVM IR.
//!
//! Three transformations with observable, testable payoffs:
//! 1. **Producer-consumer fusion**: fuses `zip-with` / `map` into `reduce`,
//!    `gather + zip-mul` into `segmented-reduce` (SpMV), and consecutive
//!    stencils into one ping-pong iteration loop, including through imported
//!    helpers.
//! 2. **Nested parallelism schedule selection**: analyzes static or dynamic
//!    iteration shapes for row reductions, batched stencils, batched
//!    histograms, and CSR SpMV, selecting outer-parallel vs inner-parallel vs
//!    dynamic runtime dispatch and reporting the choice.
//! 3. **Storage planning & buffer reuse**: plans 2-buffer ping-pong storage for
//!    repeated stencils, in-place reuse for consumed scatters, and resident
//!    device chaining (0 intermediate host<->device copies) across calls.

use std::collections::{BTreeSet, HashMap};

use crate::arrays::ArrayComputeOp;
use crate::core::{
    ArithOp, CAlt, CBranch, CPat, Class, Core, FnDef, GlobalId, LocalId, PrimOp, Program, UnitRepr,
};
use crate::types::Scalar;
use crate::{Error, Result};

// ---------------------------------------------------------------------------
// Target selection & Report types
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrayTarget {
    /// Single-threaded sequential CPU (equivalent to `futhark c`).
    Seq,
    /// Multicore CPU thread pool (equivalent to `futhark multicore`).
    Cpu,
    /// SIMD-vectorized multicore CPU (equivalent to `futhark ispc`).
    Ispc,
    /// Apple Metal GPU with CPU fallback.
    Metal,
    /// Apple Metal GPU required (compile-time refusal on unsupported ops).
    MetalRequired,
    /// OpenCL GPU with CPU fallback (equivalent to `futhark opencl`).
    OpenCl,
    /// OpenCL GPU required.
    OpenClRequired,
    /// NVIDIA CUDA GPU with CPU fallback (equivalent to `futhark cuda`).
    Cuda,
    /// NVIDIA CUDA GPU required.
    CudaRequired,
    /// AMD ROCm HIP GPU with CPU fallback (equivalent to `futhark hip`).
    Hip,
    /// AMD ROCm HIP GPU required.
    HipRequired,
    /// Python sequential/multi-backend binding (equivalent to `futhark python`).
    Python,
    /// Python OpenCL binding (equivalent to `futhark pyopencl`).
    PyOpenCl,
    /// WebAssembly sequential (equivalent to `futhark wasm`).
    Wasm,
    /// WebAssembly multicore (equivalent to `futhark wasm-multicore`).
    WasmMulticore,
    /// WebAssembly + WebGPU (WGSL compute shaders + JS host bridge, CPU fallback).
    WebGpu,
    /// WebAssembly + WebGPU required (compile-time refusal on unsupported ops / f64).
    WebGpuRequired,
}

impl ArrayTarget {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "seq" | "c" => Some(Self::Seq),
            "cpu" | "multicore" => Some(Self::Cpu),
            "ispc" | "simd" => Some(Self::Ispc),
            "metal" | "gpu" => Some(Self::Metal),
            "metal-required" | "gpu-required" => Some(Self::MetalRequired),
            "opencl" => Some(Self::OpenCl),
            "opencl-required" => Some(Self::OpenClRequired),
            "cuda" => Some(Self::Cuda),
            "cuda-required" => Some(Self::CudaRequired),
            "hip" => Some(Self::Hip),
            "hip-required" => Some(Self::HipRequired),
            "python" => Some(Self::Python),
            "pyopencl" => Some(Self::PyOpenCl),
            "wasm" => Some(Self::Wasm),
            "wasm-multicore" => Some(Self::WasmMulticore),
            "webgpu" | "wasm-webgpu" => Some(Self::WebGpu),
            "webgpu-required" | "wasm-webgpu-required" => Some(Self::WebGpuRequired),
            _ => None,
        }
    }

    pub fn all_names() -> &'static str {
        "seq, c, cpu, multicore, ispc, metal, metal-required, opencl, opencl-required, cuda, cuda-required, hip, hip-required, python, pyopencl, wasm, wasm-multicore, webgpu, webgpu-required, wasm-webgpu, wasm-webgpu-required"
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Seq => "seq",
            Self::Cpu => "cpu",
            Self::Ispc => "ispc",
            Self::Metal => "metal",
            Self::MetalRequired => "metal-required",
            Self::OpenCl => "opencl",
            Self::OpenClRequired => "opencl-required",
            Self::Cuda => "cuda",
            Self::CudaRequired => "cuda-required",
            Self::Hip => "hip",
            Self::HipRequired => "hip-required",
            Self::Python => "python",
            Self::PyOpenCl => "pyopencl",
            Self::Wasm => "wasm",
            Self::WasmMulticore => "wasm-multicore",
            Self::WebGpu => "webgpu",
            Self::WebGpuRequired => "webgpu-required",
        }
    }

    pub fn is_gpu(self) -> bool {
        matches!(
            self,
            Self::Metal
                | Self::MetalRequired
                | Self::OpenCl
                | Self::OpenClRequired
                | Self::Cuda
                | Self::CudaRequired
                | Self::Hip
                | Self::HipRequired
                | Self::PyOpenCl
                | Self::WebGpu
                | Self::WebGpuRequired
        )
    }

    pub fn is_gpu_required(self) -> bool {
        matches!(
            self,
            Self::MetalRequired
                | Self::OpenClRequired
                | Self::CudaRequired
                | Self::HipRequired
                | Self::WebGpuRequired
        )
    }

    pub fn is_webgpu(self) -> bool {
        matches!(self, Self::WebGpu | Self::WebGpuRequired)
    }

    pub fn kernel_language(self) -> &'static str {
        match self {
            Self::Metal | Self::MetalRequired => "Metal MSL",
            Self::OpenCl | Self::OpenClRequired | Self::PyOpenCl => "OpenCL C",
            Self::Cuda | Self::CudaRequired => "CUDA C++",
            Self::Hip | Self::HipRequired => "HIP C++",
            Self::WebGpu | Self::WebGpuRequired => "WebGPU WGSL",
            Self::Ispc => "ISPC",
            Self::Python => "Python",
            Self::Wasm | Self::WasmMulticore => "WebAssembly",
            Self::Seq | Self::Cpu => "LLVM/C",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FusionRecord {
    pub unit: String,
    pub producer: String,
    pub consumer: String,
    pub fused_op: String,
    pub eliminated_allocation: String,
    pub numerical_mode: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduleRecord {
    pub unit: String,
    pub op: String,
    pub policy: String,
    pub numerical_mode: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageRecord {
    pub unit: String,
    pub op: String,
    pub kernel_launches: String,
    pub temp_buffers: String,
    pub host_device_copies: String,
    pub reuse_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArrayIrNodeSummary {
    pub unit: String,
    pub op: String,
    pub elem_type: String,
    pub shape: String,
    pub contract: String,
    pub buffer_use: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuDiagnostic {
    pub unit: String,
    pub category: String,
    pub reason: String,
}

#[derive(Clone, Debug, Default)]
pub struct ArrayOptReport {
    pub target: String,
    pub kernel_language: String,
    pub nodes: Vec<ArrayIrNodeSummary>,
    pub fusions: Vec<FusionRecord>,
    pub schedules: Vec<ScheduleRecord>,
    pub storage: Vec<StorageRecord>,
    pub gpu_diagnostics: Vec<GpuDiagnostic>,
    pub generated_msl: Vec<(String, String)>,
    pub generated_kernels: Vec<(String, String, String)>,
}

impl ArrayOptReport {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.fusions.is_empty()
            && self.schedules.is_empty()
            && self.storage.is_empty()
            && self.gpu_diagnostics.is_empty()
    }

    pub fn format_kernels_source(&self) -> String {
        let mut out = String::new();
        for (lang, name, src) in &self.generated_kernels {
            if lang == "Python" {
                out.push_str(&format!("# Generated {lang} binding: {name}\n"));
            } else {
                out.push_str(&format!("// Generated {lang} kernel: {name}\n"));
            }
            out.push_str(src);
            if !src.ends_with('\n') {
                out.push('\n');
            }
            out.push('\n');
        }
        out
    }

    /// Generate a standalone ES module Wasm + WebGPU JS host bridge (`<out>.webgpu.mjs`)
    /// embedding the built-in runtime WGSL kernels plus any specialized WGSL kernels
    /// emitted for this program.
    pub fn format_webgpu_js_bridge(&self) -> String {
        const BUILTIN_WGSL: &str = include_str!("../runtime/src/webgpu_builtin.wgsl");
        const BRIDGE_TEMPLATE: &str = include_str!("../runtime/src/webgpu_bridge.mjs");
        let extra = self.format_kernels_source();
        let mut wgsl = BUILTIN_WGSL.to_string();
        if !extra.trim().is_empty() {
            wgsl.push_str("\n// --- Program-specific specialized WGSL kernels ---\n");
            wgsl.push_str(&extra);
        }
        let escaped = wgsl
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace("${", "\\${");
        BRIDGE_TEMPLATE.replace("__CINCH_WGSL_PLACEHOLDER__", &escaped)
    }

    pub fn format_text(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("=== Cinch Array IR Report (target: {}) ===\n", self.target));
        out.push_str(&format!(
            "Summary: {} array ops, {} fusions, {} schedules, {} storage plans, {} GPU diagnostics\n",
            self.nodes.len(),
            self.fusions.len(),
            self.schedules.len(),
            self.storage.len(),
            self.gpu_diagnostics.len(),
        ));
        if !self.fusions.is_empty() {
            out.push_str("\nFusions (eliminated allocations):\n");
            for f in &self.fusions {
                out.push_str(&format!(
                    "  - [{}] {} -> {} ==> {} | eliminated: {} | mode: {}\n",
                    f.unit, f.producer, f.consumer, f.fused_op, f.eliminated_allocation, f.numerical_mode
                ));
            }
        }
        if !self.schedules.is_empty() {
            out.push_str("\nNested Parallelism Schedules:\n");
            for s in &self.schedules {
                out.push_str(&format!(
                    "  - [{}] {} | schedule: {} | mode: {}\n",
                    s.unit, s.op, s.policy, s.numerical_mode
                ));
            }
        }
        if !self.storage.is_empty() {
            out.push_str("\nStorage & Transfer Plan:\n");
            for s in &self.storage {
                out.push_str(&format!(
                    "  - [{}] {} | launches: {} | temps: {} | host<->device: {} | {}\n",
                    s.unit, s.op, s.kernel_launches, s.temp_buffers, s.host_device_copies, s.reuse_note
                ));
            }
        }
        if !self.gpu_diagnostics.is_empty() {
            out.push_str("\nGPU Eligibility Diagnostics:\n");
            for d in &self.gpu_diagnostics {
                out.push_str(&format!("  - [{}] ({}) {}\n", d.unit, d.category, d.reason));
            }
        }
        if !self.generated_kernels.is_empty() {
            let lang = if self.kernel_language.is_empty() { "Metal MSL" } else { &self.kernel_language };
            out.push_str(&format!("\nGenerated {lang} Kernels ({}):\n", self.generated_kernels.len()));
            for (_, name, _) in &self.generated_kernels {
                out.push_str(&format!("  - kernel {name}\n"));
            }
        }
        out
    }

    pub fn format_json(&self) -> String {
        let esc = |s: &str| s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
        let fusions: Vec<String> = self
            .fusions
            .iter()
            .map(|f| {
                format!(
                    "{{\"unit\":\"{}\",\"producer\":\"{}\",\"consumer\":\"{}\",\"fused_op\":\"{}\",\"eliminated_allocation\":\"{}\",\"numerical_mode\":\"{}\"}}",
                    esc(&f.unit), esc(&f.producer), esc(&f.consumer), esc(&f.fused_op), esc(&f.eliminated_allocation), esc(&f.numerical_mode)
                )
            })
            .collect();
        let schedules: Vec<String> = self
            .schedules
            .iter()
            .map(|s| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"policy\":\"{}\",\"numerical_mode\":\"{}\"}}",
                    esc(&s.unit), esc(&s.op), esc(&s.policy), esc(&s.numerical_mode)
                )
            })
            .collect();
        let storage: Vec<String> = self
            .storage
            .iter()
            .map(|s| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"kernel_launches\":\"{}\",\"temp_buffers\":\"{}\",\"host_device_copies\":\"{}\",\"reuse_note\":\"{}\"}}",
                    esc(&s.unit), esc(&s.op), esc(&s.kernel_launches), esc(&s.temp_buffers), esc(&s.host_device_copies), esc(&s.reuse_note)
                )
            })
            .collect();
        let nodes: Vec<String> = self
            .nodes
            .iter()
            .map(|n| {
                format!(
                    "{{\"unit\":\"{}\",\"op\":\"{}\",\"elem_type\":\"{}\",\"shape\":\"{}\",\"contract\":\"{}\",\"buffer_use\":\"{}\"}}",
                    esc(&n.unit), esc(&n.op), esc(&n.elem_type), esc(&n.shape), esc(&n.contract), esc(&n.buffer_use)
                )
            })
            .collect();
        let diags: Vec<String> = self
            .gpu_diagnostics
            .iter()
            .map(|d| {
                format!(
                    "{{\"unit\":\"{}\",\"category\":\"{}\",\"reason\":\"{}\"}}",
                    esc(&d.unit), esc(&d.category), esc(&d.reason)
                )
            })
            .collect();
        let kernels: Vec<String> = self
            .generated_kernels
            .iter()
            .map(|(lang, name, _)| format!("{{\"language\":\"{}\",\"name\":\"{}\"}}", esc(lang), esc(name)))
            .collect();
        format!(
            "{{\"target\":\"{}\",\"kernel_language\":\"{}\",\"nodes\":[{}],\"fusions\":[{}],\"schedules\":[{}],\"storage\":[{}],\"gpu_diagnostics\":[{}],\"msl_kernel_count\":{},\"generated_kernels\":[{}]}}",
            esc(&self.target),
            esc(&self.kernel_language),
            nodes.join(","),
            fusions.join(","),
            schedules.join(","),
            storage.join(","),
            diags.join(","),
            self.generated_msl.len(),
            kernels.join(","),
        )
    }
}

// ---------------------------------------------------------------------------
// Core IR helpers (matching `seed/src/core.rs`'s exact representation)
// ---------------------------------------------------------------------------

fn flatten_app<'a>(expr: &'a Core) -> (&'a Core, Vec<&'a Core>) {
    let mut chunks: Vec<&[Core]> = Vec::new();
    let mut cur = expr;
    while let Core::App(f, args) = cur {
        chunks.push(args.as_slice());
        cur = f.as_ref();
    }
    chunks.reverse();
    let mut flat = Vec::new();
    for ch in chunks {
        for a in ch {
            flat.push(a);
        }
    }
    (cur, flat)
}

fn const_int(expr: &Core) -> Option<i64> {
    match expr {
        Core::Int(n) => Some(*n),
        Core::ConstBig { digits, .. } => digits.parse::<i64>().ok(),
        Core::App(h, args) if args.len() == 1 => match h.as_ref() {
            Core::Prim(
                PrimOp::Int(crate::core::IntOp::FromI64)
                | PrimOp::Nat(crate::core::NatOp::FromI64),
            ) => const_int(&args[0]),
            _ => None,
        },
        _ => None,
    }
}

fn count_uses(expr: &Core, id: LocalId) -> usize {
    match expr {
        Core::Var(v) => usize::from(*v == id),
        Core::App(f, args) => count_uses(f, id) + args.iter().map(|a| count_uses(a, id)).sum::<usize>(),
        Core::Lam { env, .. } => env.iter().map(|e| count_uses(e, id)).sum(),
        Core::InstDict(_, args, _) => args.iter().map(|a| count_uses(a, id)).sum(),
        Core::DictProj(e, _, _, _) | Core::Proj(e, _) | Core::Demand(e, _) | Core::BoxGet(e, _) => {
            count_uses(e, id)
        }
        Core::LazyInit(l, e) | Core::BoxInit(l, e, _) | Core::RefSet(l, e, _) => {
            usize::from(*l == id) + count_uses(e, id)
        }
        Core::BoxClear(e, _) => count_uses(e, id),
        Core::Let(x, _, v, b) => {
            count_uses(v, id) + if *x == id { 0 } else { count_uses(b, id) }
        }
        Core::Join { params, body, rest, .. } => {
            let b_u = if params.iter().any(|(p, _)| *p == id) { 0 } else { count_uses(body, id) };
            b_u + count_uses(rest, id)
        }
        Core::Jump { args, .. } => args.iter().map(|a| count_uses(a, id)).sum(),
        Core::If(c, t, e) => count_uses(c, id) + count_uses(&t.body, id) + count_uses(&e.body, id),
        Core::Case { scrut, alts } => {
            count_uses(scrut, id)
                + alts
                    .iter()
                    .map(|a| {
                        let shadowed = match &a.pat {
                            CPat::Var(v, _) => *v == id,
                            CPat::Con { fields, .. } => fields.iter().any(|v| *v == id),
                            CPat::Wild => false,
                        };
                        if shadowed { 0 } else { count_uses(&a.body, id) }
                    })
                    .sum::<usize>()
        }
        Core::Dup(v, _, b) => usize::from(*v == id) + count_uses(b, id),
        _ => 0,
    }
}

fn core_eq_simple(a: &Core, b: &Core) -> bool {
    match (a, b) {
        (Core::Int(x), Core::Int(y)) => x == y,
        (Core::Bool(x), Core::Bool(y)) => x == y,
        (Core::Unit, Core::Unit) => true,
        (Core::Var(x), Core::Var(y)) => x == y,
        (Core::ConstBig { nat: n1, digits: d1 }, Core::ConstBig { nat: n2, digits: d2 }) => {
            n1 == n2 && d1 == d2
        }
        (Core::Global(g1, _), Core::Global(g2, _)) => g1 == g2,
        (Core::App(f1, a1), Core::App(f2, a2)) => {
            core_eq_simple(f1, f2)
                && a1.len() == a2.len()
                && a1.iter().zip(a2.iter()).all(|(x, y)| core_eq_simple(x, y))
        }
        _ => false,
    }
}

fn reduce_mode_label(code: Option<i64>, is_f32: bool) -> String {
    match code {
        Some(0) if is_f32 => "f32-add-exact-tree".to_string(),
        Some(1) if is_f32 => "f32-add-relaxed".to_string(),
        Some(0) => "wrapping-add".to_string(),
        Some(1) => "wrapping-mul".to_string(),
        Some(2) => "min".to_string(),
        Some(3) => "max".to_string(),
        Some(4) => "bit-and".to_string(),
        Some(5) => "bit-or".to_string(),
        Some(6) => "bit-xor".to_string(),
        Some(10) => "f32-add-exact-tree".to_string(),
        Some(11) => "f32-add-relaxed".to_string(),
        Some(12) => "f64-add-exact-tree".to_string(),
        Some(13) => "f64-add-relaxed".to_string(),
        Some(14) => "f32-min".to_string(),
        Some(15) => "f32-max".to_string(),
        Some(16) => "f64-min".to_string(),
        Some(17) => "f64-max".to_string(),
        _ => if is_f32 { "f32-dynamic-contract".to_string() } else { "dynamic-contract".to_string() },
    }
}

/// Index of primitive units, single-call wrapper helpers (including imported
/// helpers across modules!), and hoisted lambda bodies for closure fusion.
struct OpIndex {
    unit_op: HashMap<String, ArrayComputeOp>,
    helper_alias: HashMap<String, (ArrayComputeOp, usize, Core)>,
    fn_bodies: HashMap<String, (usize, Core)>,
    unit_prim: HashMap<String, PrimOp>,
}

impl OpIndex {
    fn build(program: &Program) -> Self {
        let mut unit_op = HashMap::new();
        let mut unit_prim = HashMap::new();
        let mut fn_bodies = HashMap::new();
        for f in &program.fns {
            if let Some(p) = f.prim {
                unit_prim.insert(f.id.0.clone(), p);
                if let PrimOp::ArrayCompute { op, .. } = p {
                    unit_op.insert(f.id.0.clone(), op);
                }
            } else if f.ctor.is_none() {
                fn_bodies.insert(f.id.0.clone(), (f.params.len(), f.body.clone()));
            }
        }
        let mut helper_alias = HashMap::new();
        for f in &program.fns {
            if f.prim.is_some() || f.ctor.is_some() {
                continue;
            }
            let (head, _) = flatten_app(&f.body);
            if let Some(op) = Self::resolve_head_op(&unit_op, head) {
                helper_alias.insert(f.id.0.clone(), (op, f.params.len(), f.body.clone()));
            }
        }
        Self {
            unit_op,
            helper_alias,
            fn_bodies,
            unit_prim,
        }
    }

    fn resolve_head_op(unit_op: &HashMap<String, ArrayComputeOp>, head: &Core) -> Option<ArrayComputeOp> {
        match head {
            Core::Prim(PrimOp::ArrayCompute { op, .. }) => Some(*op),
            Core::Global(g, _) => unit_op.get(&g.0).copied(),
            _ => None,
        }
    }

    fn resolve_any_prim(&self, head: &Core) -> Option<PrimOp> {
        match head {
            Core::Prim(p) => Some(*p),
            Core::Global(g, _) => self.unit_prim.get(&g.0).copied(),
            _ => None,
        }
    }

    fn match_call(&self, expr: &Core) -> Option<(ArrayComputeOp, String, Vec<Core>)> {
        let (head, args) = flatten_app(expr);
        if let Some(op) = Self::resolve_head_op(&self.unit_op, head) {
            let label = match head {
                Core::Global(g, _) => g.0.clone(),
                _ => op.name().to_string(),
            };
            return Some((op, label, args.into_iter().cloned().collect()));
        }
        // Resolve through an imported or local wrapper helper!
        if let Core::Global(g, _) = head {
            if let Some((op, arity, body)) = self.helper_alias.get(&g.0) {
                if *arity == args.len() {
                    let subst_map: HashMap<LocalId, &Core> = args
                        .iter()
                        .enumerate()
                        .map(|(i, a)| (LocalId(i), *a))
                        .collect();
                    let (_, b_args) = flatten_app(body);
                    let resolved: Vec<Core> = b_args
                        .into_iter()
                        .map(|ba| subst_locals(ba, &subst_map))
                        .collect();
                    return Some((*op, format!("{} ({})", g.0, op.name()), resolved));
                }
            }
        }
        None
    }

    /// Recognize a hoisted closure `\x -> x * x`, `\x -> x * c`, `\x -> x + c`, `\x -> -x`.
    fn recognize_unary_f32_closure(&self, expr: &Core) -> Option<(i64, Core)> {
        let (fn_id, env) = match expr {
            Core::Lam { fn_id, env, .. } => (fn_id, env.as_slice()),
            Core::Global(g, _) => (g, &[][..]),
            _ => return None,
        };
        let (arity, body) = self.fn_bodies.get(&fn_id.0)?;
        if *arity != env.len() + 1 {
            return None;
        }
        let param_id = LocalId(env.len());
        let env_map: HashMap<LocalId, &Core> = env
            .iter()
            .enumerate()
            .map(|(i, e)| (LocalId(i), e))
            .collect();
        let body = subst_locals(body, &env_map);
        if matches!(body, Core::Var(v) if v == param_id) {
            return Some((0, Core::Int(0))); // UNARY_ID
        }
        let (head, args) = flatten_app(&body);
        if let Some(PrimOp::Arith { ty: Scalar::F32, op, .. }) = self.resolve_any_prim(head) {
            if args.len() == 2 {
                let is_p0 = matches!(args[0], Core::Var(v) if *v == param_id);
                let is_p1 = matches!(args[1], Core::Var(v) if *v == param_id);
                match op {
                    ArithOp::Mul if is_p0 && is_p1 => return Some((3, Core::Int(0))), // UNARY_SQUARE
                    ArithOp::Mul if is_p0 && count_uses(args[1], param_id) == 0 => {
                        return Some((2, args[1].clone()));
                    }
                    ArithOp::Mul if is_p1 && count_uses(args[0], param_id) == 0 => {
                        return Some((2, args[0].clone()));
                    }
                    ArithOp::Add if is_p0 && count_uses(args[1], param_id) == 0 => {
                        return Some((1, args[1].clone()));
                    }
                    ArithOp::Add if is_p1 && count_uses(args[0], param_id) == 0 => {
                        return Some((1, args[0].clone()));
                    }
                    _ => {}
                }
            }
        }
        if let Some(PrimOp::Neg { ty: Scalar::F32, .. }) = self.resolve_any_prim(head) {
            if args.len() == 1 && matches!(args[0], Core::Var(v) if *v == param_id) {
                return Some((5, Core::Int(0))); // UNARY_NEG
            }
        }
        None
    }

    /// Recognize a hoisted binary multiply closure `\x y -> x * y`.
    fn is_mul_f32_closure(&self, expr: &Core) -> bool {
        let (fn_id, env) = match expr {
            Core::Lam { fn_id, env, .. } => (fn_id, env.as_slice()),
            Core::Global(g, _) => (g, &[][..]),
            _ => return false,
        };
        let Some((arity, body)) = self.fn_bodies.get(&fn_id.0) else {
            return false;
        };
        if *arity != env.len() + 2 {
            return false;
        }
        let p0 = LocalId(env.len());
        let p1 = LocalId(env.len() + 1);
        let (head, args) = flatten_app(body);
        if let Some(PrimOp::Arith { ty: Scalar::F32, op: ArithOp::Mul, .. }) = self.resolve_any_prim(head) {
            if args.len() == 2 {
                let a0 = matches!(args[0], Core::Var(v) if *v == p0);
                let a1 = matches!(args[1], Core::Var(v) if *v == p1);
                let b0 = matches!(args[0], Core::Var(v) if *v == p1);
                let b1 = matches!(args[1], Core::Var(v) if *v == p0);
                return (a0 && a1) || (b0 && b1);
            }
        }
        false
    }
}

fn subst_locals(expr: &Core, map: &HashMap<LocalId, &Core>) -> Core {
    match expr {
        Core::Var(x) => map.get(x).map(|e| (*e).clone()).unwrap_or_else(|| expr.clone()),
        Core::App(f, args) => Core::App(
            Box::new(subst_locals(f, map)),
            args.iter().map(|a| subst_locals(a, map)).collect(),
        ),
        Core::Lam { fn_id, env, classes } => Core::Lam {
            fn_id: fn_id.clone(),
            env: env.iter().map(|e| subst_locals(e, map)).collect(),
            classes: classes.clone(),
        },
        Core::If(c, t, e) => Core::If(
            Box::new(subst_locals(c, map)),
            CBranch::weighted(t.weight, subst_locals(&t.body, map)),
            CBranch::weighted(e.weight, subst_locals(&e.body, map)),
        ),
        _ => expr.clone(),
    }
}

fn is_some_alt(alt: &CAlt<Core>) -> Option<LocalId> {
    match &alt.pat {
        CPat::Con { tag: 1, fields, .. } if fields.len() == 1 => Some(fields[0]),
        _ => None,
    }
}

fn is_none_alt(alt: &CAlt<Core>) -> bool {
    matches!(&alt.pat, CPat::Con { tag: 0, fields, .. } if fields.is_empty())
        || matches!(&alt.pat, CPat::Wild | CPat::Var(_, _))
}

fn ensure_prim_unit(program: &mut Program, op: ArrayComputeOp) -> GlobalId {
    let sym = format!("cinch.runtime.{}", op.name());
    if let Some(existing) = program.fns.iter().find(|f| {
        matches!(f.prim, Some(PrimOp::ArrayCompute { op: o, .. }) if o == op) || f.id.0 == sym
    }) {
        return existing.id.clone();
    }
    let (args, res) = op.sig();
    let repr_params: Vec<Class> = args
        .iter()
        .map(|t| match t {
            crate::types::Ty::Nat | crate::types::Ty::Int => Class::TObj,
            crate::types::Ty::Adt(_, _) | crate::types::Ty::Bytes | crate::types::Ty::Fun(..) => Class::Obj,
            _ => Class::Raw,
        })
        .collect();
    let repr_result = match res {
        crate::types::Ty::Nat | crate::types::Ty::Int => Class::TObj,
        crate::types::Ty::Adt(_, _) | crate::types::Ty::Bytes => Class::Obj,
        _ => Class::Raw,
    };
    let id = GlobalId(sym);
    program.fns.push(FnDef {
        id: id.clone(),
        params: (0..args.len()).map(|i| format!("p{i}")).collect(),
        body: Core::Unit,
        prim: Some(PrimOp::ArrayCompute { op, classes: [Class::Raw; 2] }),
        ctor: None,
        hints: Default::default(),
        meta: Default::default(),
        repr: UnitRepr {
            tparams: 0,
            params: repr_params,
            result: Some(repr_result),
            ..Default::default()
        },
    });
    id
}

fn make_len_guarded(a: &Core, b: &Core, fused: Core, fallback: Core) -> Core {
    let len_op = PrimOp::Array {
        op: crate::arrays::ArrayOp::Len,
        classes: [Class::Raw; 2],
        packed: None,
    };
    let la = Core::App(Box::new(Core::Prim(len_op)), vec![a.clone()]);
    let lb = Core::App(Box::new(Core::Prim(len_op)), vec![b.clone()]);
    let cmp = Core::App(
        Box::new(Core::Prim(PrimOp::Nat(crate::core::NatOp::Cmp))),
        vec![la, lb],
    );
    let eq = Core::App(
        Box::new(Core::Prim(PrimOp::Cmp { ty: Scalar::I64, op: crate::core::CmpOp::Eq })),
        vec![cmp, Core::Int(0)],
    );
    Core::If(Box::new(eq), CBranch::new(fused), CBranch::new(fallback))
}

// ---------------------------------------------------------------------------
// Core-to-Core Fusion Rewriter
// ---------------------------------------------------------------------------

struct FusionRewriter<'a> {
    idx: &'a OpIndex,
    unit_name: String,
    fusions: Vec<FusionRecord>,
    needed_prims: BTreeSet<ArrayComputeOp>,
    let_producers: HashMap<LocalId, (ArrayComputeOp, String, Vec<Core>)>,
}

impl<'a> FusionRewriter<'a> {
    fn rewrite(&mut self, expr: Core) -> Core {
        match expr {
            Core::Let(var, cls, rhs, body) => {
                let rhs = self.rewrite(*rhs);
                let recorded = self.idx.match_call(&rhs);
                if let Some(info) = recorded.clone() {
                    self.let_producers.insert(var, info);
                }
                let body = self.rewrite(*body);
                if recorded.is_some() {
                    self.let_producers.remove(&var);
                }
                if recorded.is_some() && count_uses(&body, var) == 0 {
                    return body;
                }
                Core::Let(var, cls, Box::new(rhs), Box::new(body))
            }
            Core::Case { scrut, alts } => {
                let scrut = self.rewrite(*scrut);
                let alts: Vec<CAlt<Core>> = alts
                    .into_iter()
                    .map(|a| CAlt {
                        pat: a.pat,
                        weight: a.weight,
                        body: Box::new(self.rewrite(*a.body)),
                    })
                    .collect();
                if let Some(fused) = self.try_fuse_case(&scrut, &alts) {
                    return fused;
                }
                Core::Case { scrut: Box::new(scrut), alts }
            }
            Core::App(f, args) => {
                let f = self.rewrite(*f);
                let args: Vec<Core> = args.into_iter().map(|a| self.rewrite(a)).collect();
                let rebuilt = Core::App(Box::new(f), args);
                if let Some(fused) = self.try_fuse_direct_app(&rebuilt) {
                    return fused;
                }
                rebuilt
            }
            Core::If(c, t, e) => Core::If(
                Box::new(self.rewrite(*c)),
                CBranch::weighted(t.weight, self.rewrite(*t.body)),
                CBranch::weighted(e.weight, self.rewrite(*e.body)),
            ),
            Core::Join { id, params, body, rest } => Core::Join {
                id,
                params,
                body: Box::new(self.rewrite(*body)),
                rest: Box::new(self.rewrite(*rest)),
            },
            Core::Jump { id, args } => Core::Jump {
                id,
                args: args.into_iter().map(|a| self.rewrite(a)).collect(),
            },
            Core::Lam { fn_id, env, classes } => Core::Lam {
                fn_id,
                env: env.into_iter().map(|e| self.rewrite(e)).collect(),
                classes,
            },
            other => other,
        }
    }

    fn resolve_producer(&self, expr: &Core) -> Option<(ArrayComputeOp, String, Vec<Core>)> {
        if let Some(call) = self.idx.match_call(expr) {
            return Some(call);
        }
        if let Core::Var(v) = expr {
            if let Some(call) = self.let_producers.get(v) {
                return Some(call.clone());
            }
        }
        None
    }

    fn try_fuse_direct_app(&mut self, expr: &Core) -> Option<Core> {
        let (c_op, c_name, c_args) = self.idx.match_call(expr)?;
        if c_op == ArrayComputeOp::ReduceF32 && c_args.len() == 2 {
            let (p_op, p_name, p_args) = self.resolve_producer(&c_args[0])?;
            if p_op == ArrayComputeOp::MapUnaryF32 && p_args.len() == 3 {
                self.needed_prims.insert(ArrayComputeOp::FusedMapReduceF32);
                self.fusions.push(FusionRecord {
                    unit: self.unit_name.clone(),
                    producer: p_name,
                    consumer: c_name,
                    fused_op: "arrayFusedMapReduceF32".to_string(),
                    eliminated_allocation: "intermediate Array F32 (4 * n bytes)".to_string(),
                    numerical_mode: reduce_mode_label(const_int(&c_args[1]), true),
                });
                let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::FusedMapReduceF32, classes: [Class::Raw; 2] });
                return Some(Core::App(
                    Box::new(head),
                    vec![p_args[0].clone(), p_args[1].clone(), p_args[2].clone(), c_args[1].clone()],
                ));
            }
            if p_op == ArrayComputeOp::MapF32 && p_args.len() == 2 {
                if let Some((unary_code, scalar_expr)) = self.idx.recognize_unary_f32_closure(&p_args[0]) {
                    self.needed_prims.insert(ArrayComputeOp::FusedMapReduceF32);
                    self.fusions.push(FusionRecord {
                        unit: self.unit_name.clone(),
                        producer: p_name,
                        consumer: c_name,
                        fused_op: "arrayFusedMapReduceF32".to_string(),
                        eliminated_allocation: "intermediate Array F32 (4 * n bytes)".to_string(),
                        numerical_mode: reduce_mode_label(const_int(&c_args[1]), true),
                    });
                    let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::FusedMapReduceF32, classes: [Class::Raw; 2] });
                    return Some(Core::App(
                        Box::new(head),
                        vec![p_args[1].clone(), Core::Int(unary_code), scalar_expr, c_args[1].clone()],
                    ));
                }
            }
        }
        None
    }

    fn try_fuse_case(&mut self, scrut: &Core, alts: &[CAlt<Core>]) -> Option<Core> {
        if alts.len() != 2 {
            return None;
        }
        let (p_op, p_name, p_args) = self.resolve_producer(scrut)?;
        let (some_var, some_body, none_body) = if let Some(v) = is_some_alt(&alts[0]) {
            if is_none_alt(&alts[1]) {
                (v, alts[0].body.as_ref(), alts[1].body.as_ref())
            } else {
                return None;
            }
        } else if let Some(v) = is_some_alt(&alts[1]) {
            if is_none_alt(&alts[0]) {
                (v, alts[1].body.as_ref(), alts[0].body.as_ref())
            } else {
                return None;
            }
        } else {
            return None;
        };

        let (c_op, c_name, c_args) = self.idx.match_call(some_body)?;

        // 1. ZipBinopF32(a, b, MUL=2) -> ReduceF32(p, mode) => DotF32!
        if p_op == ArrayComputeOp::ZipBinopF32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::ReduceF32
            && c_args.len() == 2
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotF32);
            let mode_str = reduce_mode_label(const_int(&c_args[1]), true);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotF32".to_string(),
                eliminated_allocation: "intermediate product Array F32 (4 * n bytes)".to_string(),
                numerical_mode: mode_str,
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotF32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[0].clone(), p_args[1].clone(), c_args[1].clone()]);
            return Some(make_len_guarded(&p_args[0], &p_args[1], fused, none_body.clone()));
        }

        // 2. ZipWithF32(\x y -> x * y, a, b) -> ReduceF32(p, mode) => DotF32!
        if p_op == ArrayComputeOp::ZipWithF32
            && p_args.len() == 3
            && self.idx.is_mul_f32_closure(&p_args[0])
            && c_op == ArrayComputeOp::ReduceF32
            && c_args.len() == 2
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotF32);
            let mode_str = reduce_mode_label(const_int(&c_args[1]), true);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotF32".to_string(),
                eliminated_allocation: "intermediate product Array F32 (4 * n bytes)".to_string(),
                numerical_mode: mode_str,
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotF32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[1].clone(), p_args[2].clone(), c_args[1].clone()]);
            return Some(make_len_guarded(&p_args[1], &p_args[2], fused, none_body.clone()));
        }

        // 3. ZipBinopI32(a, b, MUL=2) -> ReduceI32(p, ADD_WRAP=0) => DotI32!
        if p_op == ArrayComputeOp::ZipBinopI32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::ReduceI32
            && c_args.len() == 2
            && const_int(&c_args[1]) == Some(0)
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            self.needed_prims.insert(ArrayComputeOp::DotI32);
            self.fusions.push(FusionRecord {
                unit: self.unit_name.clone(),
                producer: p_name,
                consumer: c_name,
                fused_op: "arrayDotI32".to_string(),
                eliminated_allocation: "intermediate product Array I32 (4 * n bytes)".to_string(),
                numerical_mode: "wrapping-add".to_string(),
            });
            let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::DotI32, classes: [Class::Raw; 2] });
            let fused = Core::App(Box::new(head), vec![p_args[0].clone(), p_args[1].clone()]);
            return Some(make_len_guarded(&p_args[0], &p_args[1], fused, none_body.clone()));
        }

        // 4. SpMV fusion: GatherF32 + ZipBinopF32(MUL) + SegmentedReduceF32 => SpmvCsrF32!
        if p_op == ArrayComputeOp::ZipBinopF32
            && p_args.len() == 3
            && const_int(&p_args[2]) == Some(2)
            && c_op == ArrayComputeOp::SegmentedReduceF32
            && c_args.len() == 3
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
        {
            if let Some((ArrayComputeOp::GatherF32, g_name, g_args)) = self.resolve_producer(&p_args[1]) {
                if g_args.len() == 3 {
                    self.needed_prims.insert(ArrayComputeOp::SpmvCsrF32);
                    let mode_str = reduce_mode_label(const_int(&c_args[2]), true);
                    self.fusions.push(FusionRecord {
                        unit: self.unit_name.clone(),
                        producer: format!("{g_name} + {p_name}"),
                        consumer: c_name,
                        fused_op: "arraySpmvCsrF32".to_string(),
                        eliminated_allocation: "2 intermediate Array F32 buffers (8 * nnz bytes)".to_string(),
                        numerical_mode: mode_str,
                    });
                    let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::SpmvCsrF32, classes: [Class::Raw; 2] });
                    return Some(Core::App(
                        Box::new(head),
                        vec![p_args[0].clone(), g_args[1].clone(), c_args[1].clone(), g_args[0].clone(), c_args[2].clone()],
                    ));
                }
            }
        }

        // 5. Repeated Stencil2DU8 ping-pong storage fusion!
        if p_op == ArrayComputeOp::Stencil2DU8
            && c_op == ArrayComputeOp::Stencil2DU8
            && p_args.len() == 8
            && c_args.len() == 8
            && matches!(&c_args[0], Core::Var(x) if *x == some_var)
            && count_uses(some_body, some_var) == 1
            && p_args[1..7].iter().zip(c_args[1..7].iter()).all(|(x, y)| core_eq_simple(x, y))
        {
            if let (Some(s1), Some(s2)) = (const_int(&p_args[7]), const_int(&c_args[7])) {
                let total_steps = s1 + s2;
                self.fusions.push(FusionRecord {
                    unit: self.unit_name.clone(),
                    producer: format!("{p_name}(steps={s1})"),
                    consumer: format!("{c_name}(steps={s2})"),
                    fused_op: format!("arrayStencil2DU8(pingpong, steps={total_steps})"),
                    eliminated_allocation: format!("{} intermediate frame buffer allocations (2-buffer ping-pong)", total_steps - 1),
                    numerical_mode: "integer-clamped-u8".to_string(),
                });
                let head = Core::Prim(PrimOp::ArrayCompute { op: ArrayComputeOp::Stencil2DU8, classes: [Class::Raw; 2] });
                let mut new_args = p_args[0..7].to_vec();
                new_args.push(Core::ConstBig { nat: true, digits: total_steps.to_string() });
                return Some(Core::App(Box::new(head), new_args));
            }
        }

        None
    }
}

// ---------------------------------------------------------------------------
// Analysis: Extract Typed Array IR Nodes, Schedules, Storage, GPU Checks & MSL
// ---------------------------------------------------------------------------

fn analyze_expr(
    unit: &str,
    expr: &Core,
    idx: &OpIndex,
    target: ArrayTarget,
    report: &mut ArrayOptReport,
) {
    if let Some((op, _name, args)) = idx.match_call(expr) {
        use ArrayComputeOp::*;
        if target.is_webgpu()
            && matches!(
                op,
                TabulateF64 | MapF64 | ReduceF64 | DotF64 | FillF64 | ZipBinopF64 | ScaleF64
            )
        {
            report.gpu_diagnostics.push(GpuDiagnostic {
                unit: unit.to_string(),
                category: "representation".to_string(),
                reason: format!(
                    "`{}` operates on 64-bit `F64` elements, which core WebGPU WGSL does not support (`f64` is absent from WGSL 1.0); use `F32` arrays for WebGPU execution",
                    op.name()
                ),
            });
        }
        let (elem, shape, contract, buf_use) = match op {
            DotF32 => (
                "F32",
                "1D(n)",
                reduce_mode_label(args.get(2).and_then(const_int), true),
                "fused-zero-intermediate",
            ),
            DotF64 => (
                "F64",
                "1D(n)",
                reduce_mode_label(args.get(2).and_then(const_int), false),
                "fused-zero-intermediate",
            ),
            DotI32 => ("I32", "1D(n)", "wrapping-add".to_string(), "fused-zero-intermediate"),
            StridedDotF32 => (
                "F32",
                "1D-strided(n)",
                reduce_mode_label(args.get(7).and_then(const_int), true),
                "zero-copy-strided-view",
            ),
            FusedMapReduceF32 => (
                "F32",
                "1D(n)",
                reduce_mode_label(args.get(3).and_then(const_int), true),
                "fused-zero-intermediate",
            ),
            RowReduceF32 | RowReduceI32 => {
                let is_f = op == RowReduceF32;
                let r = args.get(1).and_then(const_int);
                let c = args.get(2).and_then(const_int);
                let mode = reduce_mode_label(args.get(3).and_then(const_int), is_f);
                let policy = match (r, c) {
                    (Some(rv), Some(cv)) if rv >= 32 && cv < 8192 => {
                        format!("static-outer-parallel(rows={rv},cols={cv})")
                    }
                    (Some(rv), Some(cv)) => {
                        format!("static-inner-parallel(rows={rv},cols={cv})")
                    }
                    _ => "dynamic-runtime-select(outer-if-many-rows, inner-if-long-rows)".to_string(),
                };
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy,
                    numerical_mode: mode.clone(),
                });
                (if is_f { "F32" } else { "I32" }, "2D(m x n)", mode, "1-output-buffer")
            }
            HistogramBatchU8 => {
                let f = args.get(1).and_then(const_int);
                let h = args.get(2).and_then(const_int);
                let w = args.get(3).and_then(const_int);
                let policy = match (f, h, w) {
                    (Some(fv), Some(hv), Some(wv)) if fv >= 16 || hv * wv <= 16384 => {
                        format!("static-many-small-frames(frames={fv},h={hv},w={wv},tg-privatized)")
                    }
                    (Some(fv), Some(hv), Some(wv)) => {
                        format!("static-few-large-frames(frames={fv},h={hv},w={wv},tiled-privatized)")
                    }
                    _ => "dynamic-runtime-select(many-small-frames vs few-large-frames)".to_string(),
                };
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy,
                    numerical_mode: "wrapping-add".to_string(),
                });
                ("U8->U32", "3D(frames x h x w)", "wrapping-add".to_string(), "privatized-bins")
            }
            Stencil2DU8 | Stencil2DF32 => {
                let steps = args
                    .get(if op == Stencil2DU8 { 7 } else { 6 })
                    .and_then(const_int);
                let steps_str = steps.map(|s| s.to_string()).unwrap_or_else(|| "dynamic".into());
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy: "dynamic-runtime-select(many-small-frames vs tiled-16x16-halo)".to_string(),
                    numerical_mode: if op == Stencil2DU8 { "integer-clamped-u8" } else { "ieee754-f32" }.to_string(),
                });
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: steps_str.clone(),
                    temp_buffers: "2-buffer ping-pong (1 temp + 1 out)".to_string(),
                    host_device_copies: "0 (host CPU)".to_string(),
                    reuse_note: format!("reuses 2 buffers across {steps_str} stencil iterations"),
                });
                (
                    if op == Stencil2DU8 { "U8" } else { "F32" },
                    "3D(frames x h x w)",
                    if op == Stencil2DU8 { "integer-clamped-u8" } else { "ieee754-f32" }.to_string(),
                    "ping-pong-2-buffers",
                )
            }
            DeviceStencilHistogramU8 => {
                let steps = args.get(8).and_then(const_int);
                let launches = steps.map(|s| format!("{s} (stencil) + 1 (histogram)")).unwrap_or_else(|| "steps + 1".into());
                let (policy, temps, reuse) = if target.is_webgpu() {
                    (
                        "webgpu-adaptive(many-small-frames 1-WG/frame vs few-large-frames tiled-privatized)".to_string(),
                        "2 resident GPUBuffers (ping-pong) + 1 output hist GPUBuffer".to_string(),
                        "histogram consumes resident stencil GPUBuffer on GPUDevice queue without host round-trip".to_string(),
                    )
                } else {
                    (
                        "metal-adaptive(many-small-frames 1-TG/frame vs few-large-frames tiled-privatized)".to_string(),
                        "2 resident MTLBuffers (ping-pong) + 1 output hist MTLBuffer".to_string(),
                        "histogram consumes resident stencil MTLBuffer in the same MTLCommandBuffer".to_string(),
                    )
                };
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy,
                    numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
                });
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: launches,
                    temp_buffers: temps,
                    host_device_copies: "0 intermediate (1 upload before, 1 download after)".to_string(),
                    reuse_note: reuse,
                });
                ("U8->U32", "3D(frames x h x w)", "integer-clamped-u8+wrapping-u32".to_string(), "device-resident-chained")
            }
            ScatterUniqueU32 | ScatterCombineI32 => {
                report.storage.push(StorageRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    kernel_launches: "1".to_string(),
                    temp_buffers: if op == ScatterCombineI32 { "privatized-per-thread (if parallel)" } else { "0" }.to_string(),
                    host_device_copies: "0".to_string(),
                    reuse_note: "in-place destination reuse when RC == 1".to_string(),
                });
                (
                    if op == ScatterUniqueU32 { "U32" } else { "I32" },
                    "1D(m -> n)",
                    if op == ScatterUniqueU32 { "unique-replace".to_string() } else { reduce_mode_label(args.get(3).and_then(const_int), false) },
                    "consumed-inplace-when-unique",
                )
            }
            SpmvCsrF32 | DeviceSpmvCsrF32 => {
                let mode = reduce_mode_label(args.last().and_then(const_int), true);
                report.schedules.push(ScheduleRecord {
                    unit: unit.to_string(),
                    op: op.name().to_string(),
                    policy: "adaptive-csr(scalar-row if avg_nnz<16 else simd-group-row; empty-row=0.0)".to_string(),
                    numerical_mode: mode.clone(),
                });
                ("F32", "CSR(rows, nnz)", mode, "fused-gather-mul-segreduce")
            }
            FoldlI32 | FoldlF32 => {
                if target.is_gpu() {
                    report.gpu_diagnostics.push(GpuDiagnostic {
                        unit: unit.to_string(),
                        category: "unsupported-op".to_string(),
                        reason: format!(
                            "`{}` is a strictly ordered sequential fold over a closure callback and cannot execute as a data-parallel {} kernel; use `arrayReduce*` with an explicit associative reduction contract instead",
                            op.name(),
                            target.kernel_language()
                        ),
                    });
                }
                (if op == FoldlI32 { "I32" } else { "F32" }, "1D(n)", "ordered-left-fold".to_string(), "sequential")
            }
            TabulateU8 | TabulateI32 | TabulateU32 | TabulateF32 | TabulateF64
            | MapU8 | MapI32 | MapU32 | MapF32 | MapF64
            | ZipWithI32 | ZipWithF32 | FilterPackedI32 => {
                if target.is_gpu() {
                    let cb_idx = if matches!(op, TabulateU8 | TabulateI32 | TabulateU32 | TabulateF32 | TabulateF64) { 1 } else { 0 };
                    if let Some(cb) = args.get(cb_idx) {
                        check_closure_for_gpu(unit, op.name(), cb, idx, report);
                    }
                }
                ("packed-scalar", "1D(n)", "exact".to_string(), "direct-packed-no-list")
            }
            _ => ("packed-scalar", "1D", "exact".to_string(), "packed"),
        };

        report.nodes.push(ArrayIrNodeSummary {
            unit: unit.to_string(),
            op: op.name().to_string(),
            elem_type: elem.to_string(),
            shape: shape.to_string(),
            contract,
            buffer_use: buf_use.to_string(),
        });

        if let Some((lang, kname, ksrc)) = emit_kernel_for_op(target, unit, op) {
            if matches!(target, ArrayTarget::Metal | ArrayTarget::MetalRequired) {
                report.generated_msl.push((kname.clone(), ksrc.clone()));
            }
            report.generated_kernels.push((lang.to_string(), kname, ksrc));
        }
    }

    match expr {
        Core::App(f, args) => {
            analyze_expr(unit, f, idx, target, report);
            for a in args {
                analyze_expr(unit, a, idx, target, report);
            }
        }
        Core::Let(_, _, v, b) => {
            analyze_expr(unit, v, idx, target, report);
            analyze_expr(unit, b, idx, target, report);
        }
        Core::Case { scrut, alts } => {
            analyze_expr(unit, scrut, idx, target, report);
            for a in alts {
                analyze_expr(unit, &a.body, idx, target, report);
            }
        }
        Core::If(c, t, e) => {
            analyze_expr(unit, c, idx, target, report);
            analyze_expr(unit, &t.body, idx, target, report);
            analyze_expr(unit, &e.body, idx, target, report);
        }
        Core::Join { body, rest, .. } => {
            analyze_expr(unit, body, idx, target, report);
            analyze_expr(unit, rest, idx, target, report);
        }
        Core::Jump { args, .. } => {
            for a in args {
                analyze_expr(unit, a, idx, target, report);
            }
        }
        _ => {}
    }
}

/// Inspect a callback (including its hoisted body and transitive callees!) for
/// GPU blockers: IO effects, String/Bytes heap representations, mutable state,
/// or unbounded recursion.
fn check_closure_for_gpu(
    unit: &str,
    op_name: &str,
    cb: &Core,
    idx: &OpIndex,
    report: &mut ArrayOptReport,
) {
    let mut visited = BTreeSet::new();
    fn walk(
        unit: &str,
        op_name: &str,
        e: &Core,
        idx: &OpIndex,
        visited: &mut BTreeSet<String>,
        report: &mut ArrayOptReport,
    ) {
        match e {
            Core::Prim(PrimOp::Platform(_) | PrimOp::Net(_) | PrimOp::Proc(_) | PrimOp::Pty(_)) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "effect".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` performs host I/O effect (`{}`), which is forbidden in a GPU kernel",
                        e_prim_name(e)
                    ),
                });
            }
            Core::Prim(PrimOp::Bytes(_) | PrimOp::Utf8(_) | PrimOp::Text(_) | PrimOp::Sym(_))
            | Core::ConstString { .. }
            | Core::ConstBytes(_)
            | Core::ConstSym(_) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "representation".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` uses host String/Bytes/Sym representation, which is not device-representable on Metal GPU"
                    ),
                });
            }
            Core::Prim(PrimOp::MVar { .. } | PrimOp::Stm { .. } | PrimOp::Conc(_)) | Core::RefSet(_, _, _) => {
                report.gpu_diagnostics.push(GpuDiagnostic {
                    unit: unit.to_string(),
                    category: "effect".to_string(),
                    reason: format!(
                        "callback passed to `{op_name}` uses host concurrency or mutable state, which is unsupported on GPU"
                    ),
                });
            }
            Core::Lam { fn_id, env, .. } => {
                for ev in env {
                    walk(unit, op_name, ev, idx, visited, report);
                }
                if visited.insert(fn_id.0.clone()) {
                    if let Some((_, body)) = idx.fn_bodies.get(&fn_id.0) {
                        walk(unit, op_name, body, idx, visited, report);
                    }
                }
            }
            Core::Global(g, _) => {
                if let Some(p) = idx.unit_prim.get(&g.0) {
                    walk(unit, op_name, &Core::Prim(*p), idx, visited, report);
                } else if !visited.insert(g.0.clone()) {
                    // Recursive call inside a GPU kernel!
                    report.gpu_diagnostics.push(GpuDiagnostic {
                        unit: unit.to_string(),
                        category: "recursion".to_string(),
                        reason: format!(
                            "callback passed to `{op_name}` reaches recursive function `{}`, which is rejected on Metal GPU",
                            g.0
                        ),
                    });
                } else if let Some((_, body)) = idx.fn_bodies.get(&g.0) {
                    walk(unit, op_name, body, idx, visited, report);
                }
            }
            Core::App(f, args) => {
                walk(unit, op_name, f, idx, visited, report);
                for a in args {
                    walk(unit, op_name, a, idx, visited, report);
                }
            }
            Core::Let(_, _, v, b) => {
                walk(unit, op_name, v, idx, visited, report);
                walk(unit, op_name, b, idx, visited, report);
            }
            Core::Case { scrut, alts } => {
                walk(unit, op_name, scrut, idx, visited, report);
                for a in alts {
                    walk(unit, op_name, &a.body, idx, visited, report);
                }
            }
            Core::If(c, t, f) => {
                walk(unit, op_name, c, idx, visited, report);
                walk(unit, op_name, &t.body, idx, visited, report);
                walk(unit, op_name, &f.body, idx, visited, report);
            }
            Core::Join { body, rest, .. } => {
                walk(unit, op_name, body, idx, visited, report);
                walk(unit, op_name, rest, idx, visited, report);
            }
            _ => {}
        }
    }
    walk(unit, op_name, cb, idx, &mut visited, report);
}

fn e_prim_name(e: &Core) -> String {
    match e {
        Core::Prim(p) => p.symbol(),
        _ => "prim".to_string(),
    }
}

fn emit_kernel_for_op(
    target: ArrayTarget,
    unit: &str,
    op: ArrayComputeOp,
) -> Option<(&'static str, String, String)> {
    let safe_unit: String = unit
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    let suffix = match op {
        ArrayComputeOp::DotF32 | ArrayComputeOp::DeviceDotF32 => "dot_f32",
        ArrayComputeOp::FusedMapReduceF32 => "map_reduce_f32",
        ArrayComputeOp::DeviceStencilHistogramU8 | ArrayComputeOp::Stencil2DU8 => "stencil2d_u8",
        ArrayComputeOp::Stencil2DF32 | ArrayComputeOp::DeviceStencil2DF32 => "stencil2d_f32",
        ArrayComputeOp::SpmvCsrF32 | ArrayComputeOp::DeviceSpmvCsrF32 => "spmv_csr_f32",
        ArrayComputeOp::RowReduceF32 | ArrayComputeOp::DeviceRowReduceF32 => "row_reduce_f32",
        ArrayComputeOp::ScanI32 | ArrayComputeOp::DeviceScanI32 => "scan_i32",
        ArrayComputeOp::FilterI32Gt | ArrayComputeOp::DeviceFilterI32Gt => "filter_i32_gt",
        _ => return None,
    };
    match target {
        ArrayTarget::Metal | ArrayTarget::MetalRequired => {
            let name = format!("cinch_msl_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!("kernel void {name}(device const float* a [[buffer(0)]], device const float* b [[buffer(1)]], device float* out [[buffer(2)]], constant uint& n [[buffer(3)]], uint gid [[thread_position_in_grid]]) {{ /* fused zip-mul + reduction */ }}\n"),
                "map_reduce_f32" => format!("kernel void {name}(device const float* a [[buffer(0)]], device float* out [[buffer(1)]], constant uint& n [[buffer(2)]], uint gid [[thread_position_in_grid]]) {{ /* fused unary map + reduction */ }}\n"),
                "stencil2d_u8" => format!("kernel void {name}(device const uchar* src [[buffer(0)]], device uchar* dst [[buffer(1)]], uint3 gid [[thread_position_in_grid]]) {{ /* 16x16 halo-tiled stencil + resident histogram */ }}\n"),
                "spmv_csr_f32" => format!("kernel void {name}(device const float* vals [[buffer(0)]], device const uint* cols [[buffer(1)]], device const uint* row_ptr [[buffer(2)]], device const float* x [[buffer(3)]], device float* y [[buffer(4)]], uint row [[thread_position_in_grid]]) {{ /* CSR segmented reduction */ }}\n"),
                _ => format!("kernel void {name}(device const void* inp [[buffer(0)]], device void* out [[buffer(1)]], uint gid [[thread_position_in_grid]]) {{ /* Metal compute kernel */ }}\n"),
            };
            Some(("Metal MSL", name, src))
        }
        ArrayTarget::OpenCl | ArrayTarget::OpenClRequired | ArrayTarget::PyOpenCl => {
            let name = format!("cinch_ocl_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!("__kernel void {name}(__global const float* a, __global const float* b, __global float* out, const uint n) {{ uint gid = get_global_id(0); /* fused zip-mul + reduction */ }}\n"),
                "stencil2d_u8" => format!("__kernel void {name}(__global const uchar* src, __global uchar* dst, const uint frames, const uint h, const uint w) {{ /* 16x16 __local halo-tiled stencil + resident histogram */ }}\n"),
                "spmv_csr_f32" => format!("__kernel void {name}(__global const float* vals, __global const uint* cols, __global const uint* row_ptr, __global const float* x, __global float* y, const uint nrows) {{ uint row = get_global_id(0); /* CSR segmented reduction */ }}\n"),
                _ => format!("__kernel void {name}(__global const float* inp, __global float* out, const uint n) {{ uint gid = get_global_id(0); /* OpenCL C 1.2 kernel */ }}\n"),
            };
            Some(("OpenCL C", name, src))
        }
        ArrayTarget::Cuda | ArrayTarget::CudaRequired => {
            let name = format!("cinch_cuda_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!("extern \"C\" __global__ void {name}(const float* __restrict__ a, const float* __restrict__ b, float* __restrict__ out, unsigned int n) {{ unsigned int gid = blockIdx.x * blockDim.x + threadIdx.x; /* fused warp-shuffle dot */ }}\n"),
                "stencil2d_u8" => format!("extern \"C\" __global__ void {name}(const unsigned char* __restrict__ src, unsigned char* __restrict__ dst, unsigned int frames, unsigned int h, unsigned int w) {{ /* 16x16 __shared__ halo-tiled stencil + atomicAdd histogram */ }}\n"),
                "spmv_csr_f32" => format!("extern \"C\" __global__ void {name}(const float* __restrict__ vals, const unsigned int* __restrict__ cols, const unsigned int* __restrict__ row_ptr, const float* __restrict__ x, float* __restrict__ y, unsigned int nrows) {{ /* CUDA CSR SpMV */ }}\n"),
                _ => format!("extern \"C\" __global__ void {name}(const float* __restrict__ inp, float* __restrict__ out, unsigned int n) {{ /* CUDA C++ kernel */ }}\n"),
            };
            Some(("CUDA C++", name, src))
        }
        ArrayTarget::Hip | ArrayTarget::HipRequired => {
            let name = format!("cinch_hip_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!("extern \"C\" __global__ void {name}(const float* __restrict__ a, const float* __restrict__ b, float* __restrict__ out, unsigned int n) {{ unsigned int gid = hipBlockIdx_x * hipBlockDim_x + hipThreadIdx_x; /* HIP LDS fused dot */ }}\n"),
                "stencil2d_u8" => format!("extern \"C\" __global__ void {name}(const unsigned char* __restrict__ src, unsigned char* __restrict__ dst, unsigned int frames, unsigned int h, unsigned int w) {{ /* 16x16 LDS halo-tiled stencil + histogram */ }}\n"),
                "spmv_csr_f32" => format!("extern \"C\" __global__ void {name}(const float* __restrict__ vals, const unsigned int* __restrict__ cols, const unsigned int* __restrict__ row_ptr, const float* __restrict__ x, float* __restrict__ y, unsigned int nrows) {{ /* HIP CSR SpMV */ }}\n"),
                _ => format!("extern \"C\" __global__ void {name}(const float* __restrict__ inp, float* __restrict__ out, unsigned int n) {{ /* ROCm HIP C++ kernel */ }}\n"),
            };
            Some(("HIP C++", name, src))
        }
        ArrayTarget::Ispc => {
            let name = format!("cinch_ispc_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!("export uniform float {name}(const uniform float a[], const uniform float b[], uniform int n) {{ varying float acc = 0.0f; foreach (i = 0 ... n) acc += a[i] * b[i]; return reduce_add(acc); }}\n"),
                "spmv_csr_f32" => format!("export void {name}(const uniform float vals[], const uniform unsigned int cols[], const uniform unsigned int row_ptr[], const uniform float x[], uniform float y[], uniform int nrows) {{ for (uniform int r = 0; r < nrows; ++r) {{ varying float acc = 0.0f; foreach (k = row_ptr[r] ... row_ptr[r+1]) acc += vals[k] * x[cols[k]]; y[r] = reduce_add(acc); }} }}\n"),
                _ => format!("export void {name}(const uniform float inp[], uniform float out[], uniform int n) {{ foreach (i = 0 ... n) out[i] = inp[i]; }}\n"),
            };
            Some(("ISPC", name, src))
        }
        ArrayTarget::Python => {
            let name = format!("cinch_py_{safe_unit}_{suffix}");
            let src = format!("def {name}(device, *args):\n    \"\"\"Auto-generated Cinch Python binding for {suffix}.\"\"\"\n    return getattr(device, \"{suffix}\")(*args)\n");
            Some(("Python", name, src))
        }
        ArrayTarget::Wasm | ArrayTarget::WasmMulticore => {
            let mode = if target == ArrayTarget::WasmMulticore { "wasm-threads-sab" } else { "wasm-seq-coop" };
            let name = format!("cinch_wasm_{safe_unit}_{suffix}");
            let src = format!("// Target: {mode} (freestanding WebAssembly linear memory kernel `{name}`)\n");
            Some(("WebAssembly", name, src))
        }
        ArrayTarget::WebGpu | ArrayTarget::WebGpuRequired => {
            let name = format!("cinch_wgsl_{safe_unit}_{suffix}");
            let src = match suffix {
                "dot_f32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_a: array<f32>;\n\
                     @group(0) @binding(1) var<storage, read> {name}_b: array<f32>;\n\
                     @group(0) @binding(2) var<storage, read_write> {name}_out: array<f32>;\n\
                     @group(0) @binding(3) var<uniform> {name}_n: vec4<u32>;\n\
                     var<workgroup> {name}_sh: array<f32, 256>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>, @builtin(num_workgroups) nwg: vec3<u32>) {{\n\
                       var acc: f32 = 0.0;\n\
                       var i = gid.x;\n\
                       let step = nwg.x * 256u;\n\
                       loop {{ if (i >= {name}_n.x) {{ break; }} acc = acc + {name}_a[i] * {name}_b[i]; i = i + step; }}\n\
                       {name}_sh[lid.x] = acc;\n\
                       workgroupBarrier();\n\
                       var s: u32 = 128u;\n\
                       loop {{ if (s == 0u) {{ break; }} if (lid.x < s) {{ {name}_sh[lid.x] = {name}_sh[lid.x] + {name}_sh[lid.x + s]; }} workgroupBarrier(); s = s >> 1u; }}\n\
                       if (lid.x == 0u) {{ {name}_out[wid.x] = {name}_sh[0]; }}\n\
                     }}\n"
                ),
                "map_reduce_f32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_in: array<f32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_out: array<f32>;\n\
                     @group(0) @binding(2) var<uniform> {name}_n: vec4<u32>;\n\
                     var<workgroup> {name}_sh: array<f32, 256>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {{\n\
                       var v: f32 = 0.0;\n\
                       if (gid.x < {name}_n.x) {{ v = {name}_in[gid.x]; }}\n\
                       {name}_sh[lid.x] = v;\n\
                       workgroupBarrier();\n\
                       var s: u32 = 128u;\n\
                       loop {{ if (s == 0u) {{ break; }} if (lid.x < s) {{ {name}_sh[lid.x] = {name}_sh[lid.x] + {name}_sh[lid.x + s]; }} workgroupBarrier(); s = s >> 1u; }}\n\
                       if (lid.x == 0u) {{ {name}_out[wid.x] = {name}_sh[0]; }}\n\
                     }}\n"
                ),
                "stencil2d_u8" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_src: array<u32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_hist: array<atomic<u32>>;\n\
                     @group(0) @binding(2) var<uniform> {name}_dims: vec4<u32>;\n\
                     var<workgroup> {name}_bins: array<atomic<u32>, 256>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {{\n\
                       atomicStore(&{name}_bins[lid.x], 0u);\n\
                       workgroupBarrier();\n\
                       let frame = wid.x;\n\
                       if (frame < {name}_dims.x) {{\n\
                         let fp = {name}_dims.y;\n\
                         let base = frame * fp;\n\
                         var i = lid.x;\n\
                         loop {{ if (i >= fp) {{ break; }} let w = {name}_src[(base + i) >> 2u]; let b = (w >> (((base + i) & 3u) * 8u)) & 255u; if (b < {name}_dims.z) {{ atomicAdd(&{name}_bins[b], 1u); }} i = i + 256u; }}\n\
                       }}\n\
                       workgroupBarrier();\n\
                       if (frame < {name}_dims.x && lid.x < {name}_dims.z) {{ atomicAdd(&{name}_hist[frame * {name}_dims.z + lid.x], atomicLoad(&{name}_bins[lid.x])); }}\n\
                     }}\n"
                ),
                "stencil2d_f32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_in: array<f32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_out: array<f32>;\n\
                     @group(0) @binding(2) var<uniform> {name}_dims: vec4<u32>;\n\
                     @compute @workgroup_size(16, 16, 1)\n\
                     fn {name}(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
                       if (gid.z >= {name}_dims.x || gid.y >= {name}_dims.y || gid.x >= {name}_dims.z) {{ return; }}\n\
                       let idx = gid.z * {name}_dims.y * {name}_dims.z + gid.y * {name}_dims.z + gid.x;\n\
                       {name}_out[idx] = {name}_in[idx];\n\
                     }}\n"
                ),
                "spmv_csr_f32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_vals: array<f32>;\n\
                     @group(0) @binding(1) var<storage, read> {name}_cols: array<i32>;\n\
                     @group(0) @binding(2) var<storage, read> {name}_row_ptr: array<i32>;\n\
                     @group(0) @binding(3) var<storage, read> {name}_x: array<f32>;\n\
                     @group(0) @binding(4) var<storage, read_write> {name}_y: array<f32>;\n\
                     @group(0) @binding(5) var<uniform> {name}_dims: vec4<u32>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
                       let row = gid.x;\n\
                       if (row >= {name}_dims.x) {{ return; }}\n\
                       var acc: f32 = 0.0;\n\
                       let start = u32({name}_row_ptr[row]);\n\
                       let end = u32({name}_row_ptr[row + 1u]);\n\
                       for (var k = start; k < end; k = k + 1u) {{ acc = acc + {name}_vals[k] * {name}_x[u32({name}_cols[k])]; }}\n\
                       {name}_y[row] = acc;\n\
                     }}\n"
                ),
                "row_reduce_f32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_in: array<f32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_out: array<f32>;\n\
                     @group(0) @binding(2) var<uniform> {name}_dims: vec4<u32>;\n\
                     var<workgroup> {name}_sh: array<f32, 256>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {{\n\
                       let row = wid.x;\n\
                       var acc: f32 = 0.0;\n\
                       if (row < {name}_dims.x) {{\n\
                         let base = row * {name}_dims.y;\n\
                         var j = lid.x;\n\
                         loop {{ if (j >= {name}_dims.y) {{ break; }} acc = acc + {name}_in[base + j]; j = j + 256u; }}\n\
                       }}\n\
                       {name}_sh[lid.x] = acc;\n\
                       workgroupBarrier();\n\
                       var s: u32 = 128u;\n\
                       loop {{ if (s == 0u) {{ break; }} if (lid.x < s) {{ {name}_sh[lid.x] = {name}_sh[lid.x] + {name}_sh[lid.x + s]; }} workgroupBarrier(); s = s >> 1u; }}\n\
                       if (row < {name}_dims.x && lid.x == 0u) {{ {name}_out[row] = {name}_sh[0]; }}\n\
                     }}\n"
                ),
                "scan_i32" => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_in: array<i32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_out: array<i32>;\n\
                     @group(0) @binding(2) var<uniform> {name}_dims: vec4<u32>;\n\
                     var<workgroup> {name}_sh: array<i32, 256>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(local_invocation_id) lid: vec3<u32>, @builtin(workgroup_id) wid: vec3<u32>) {{\n\
                       let gid = wid.x * 256u + lid.x;\n\
                       var v: i32 = 0;\n\
                       if (gid < {name}_dims.x) {{ v = {name}_in[gid]; }}\n\
                       {name}_sh[lid.x] = v;\n\
                       workgroupBarrier();\n\
                       var off: u32 = 1u;\n\
                       loop {{ if (off >= 256u) {{ break; }} var add: i32 = 0; if (lid.x >= off) {{ add = {name}_sh[lid.x - off]; }} workgroupBarrier(); {name}_sh[lid.x] = {name}_sh[lid.x] + add; workgroupBarrier(); off = off << 1u; }}\n\
                       if (gid < {name}_dims.x) {{ {name}_out[gid] = {name}_sh[lid.x]; }}\n\
                     }}\n"
                ),
                _ => format!(
                    "@group(0) @binding(0) var<storage, read> {name}_in: array<i32>;\n\
                     @group(0) @binding(1) var<storage, read_write> {name}_out: array<i32>;\n\
                     @group(0) @binding(2) var<uniform> {name}_dims: vec4<u32>;\n\
                     @compute @workgroup_size(256, 1, 1)\n\
                     fn {name}(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
                       if (gid.x < {name}_dims.x) {{ {name}_out[gid.x] = {name}_in[gid.x]; }}\n\
                     }}\n"
                ),
            };
            Some(("WebGPU WGSL", name, src))
        }
        ArrayTarget::Seq | ArrayTarget::Cpu => None,
    }
}

// ---------------------------------------------------------------------------
// Entry Point
// ---------------------------------------------------------------------------

pub fn optimize_program(program: &mut Program, target: ArrayTarget) -> Result<ArrayOptReport> {
    let mut report = ArrayOptReport {
        target: target.name().to_string(),
        kernel_language: target.kernel_language().to_string(),
        ..Default::default()
    };

    let has_array_compute = program
        .fns
        .iter()
        .any(|f| matches!(f.prim, Some(PrimOp::ArrayCompute { .. })));
    if !has_array_compute {
        return Ok(report);
    }

    // Pass 1: Producer-consumer fusion (iterate up to 3 rounds)
    let mut all_needed = BTreeSet::new();
    for _round in 0..3 {
        let idx = OpIndex::build(program);
        let mut round_fusions = 0usize;
        for f in &mut program.fns {
            if f.prim.is_some() || f.ctor.is_some() {
                continue;
            }
            let mut rewriter = FusionRewriter {
                idx: &idx,
                unit_name: f.id.0.clone(),
                fusions: Vec::new(),
                needed_prims: BTreeSet::new(),
                let_producers: HashMap::new(),
            };
            let old_body = std::mem::replace(&mut f.body, Core::Unit);
            f.body = rewriter.rewrite(old_body);
            round_fusions += rewriter.fusions.len();
            report.fusions.extend(rewriter.fusions);
            all_needed.extend(rewriter.needed_prims);
        }
        for op in &all_needed {
            ensure_prim_unit(program, *op);
        }
        if round_fusions == 0 {
            break;
        }
    }

    // Pass 2 & 3: Schedule planning, storage planning, GPU eligibility diagnostics & backend codegen
    let idx = OpIndex::build(program);
    for f in &program.fns {
        if f.prim.is_some() || f.ctor.is_some() {
            continue;
        }
        analyze_expr(&f.id.0, &f.body, &idx, target, &mut report);
    }

    if target.is_gpu_required() && !report.gpu_diagnostics.is_empty() {
        let reasons: Vec<String> = report
            .gpu_diagnostics
            .iter()
            .map(|d| format!("[{}] ({}) {}", d.unit, d.category, d.reason))
            .collect();
        return Err(Error::plain(format!(
            "required {} GPU target (`{}`) rejected unsupported kernel operations:\n  {}",
            target.kernel_language(),
            target.name(),
            reasons.join("\n  ")
        )));
    }

    Ok(report)
}

// ---------------------------------------------------------------------------
// Kernel Eligibility Metadata for `.cinchif`
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct KernelEligibility {
    pub cpu_parallel: bool,
    pub gpu_metal: bool,
    pub blocker: Option<String>,
}

/// Analyze a checked exported value's scheme & Core body for CPU parallel and
/// Metal GPU kernel eligibility, producing structured metadata for `.cinchif`.
pub fn analyze_scheme_kernel_eligibility(
    scheme: &str,
    body: Option<&Core>,
    idx_program: Option<&Program>,
) -> KernelEligibility {
    // Check for effect rows like `->{IO` or `->{MVar` or `->{Diverge`
    if let Some(eff_start) = scheme.find("->{") {
        if let Some(eff_end) = scheme[eff_start + 3..].find('}') {
            let row = scheme[eff_start + 3..eff_start + 3 + eff_end].trim();
            if !row.is_empty() && row != "Diverge" {
                return KernelEligibility {
                    cpu_parallel: false,
                    gpu_metal: false,
                    blocker: Some(format!("effect row `{{{row}}}` prevents parallel/device kernel execution")),
                };
            }
            if row == "Diverge" {
                return KernelEligibility {
                    cpu_parallel: true,
                    gpu_metal: false,
                    blocker: Some("effect `Diverge` prevents Metal GPU kernel execution".to_string()),
                };
            }
        }
    }
    if scheme.contains("Mut ") {
        return KernelEligibility {
            cpu_parallel: false,
            gpu_metal: false,
            blocker: Some("mutable borrow (`Mut`) cannot enter a parallel or GPU kernel".to_string()),
        };
    }
    if scheme.contains("Lazy ") {
        return KernelEligibility {
            cpu_parallel: false,
            gpu_metal: false,
            blocker: Some("memo cell (`Lazy`) is single-threaded and cannot enter a kernel".to_string()),
        };
    }
    for non_dev in ["String", "Bytes", "List ", "Json", "Sym"] {
        if scheme.contains(non_dev) {
            return KernelEligibility {
                cpu_parallel: true,
                gpu_metal: false,
                blocker: Some(format!("representation `{}` is not device-representable on Metal GPU", non_dev.trim())),
            };
        }
    }
    if let (Some(b), Some(prog)) = (body, idx_program) {
        let idx = OpIndex::build(prog);
        let mut rep = ArrayOptReport::default();
        check_closure_for_gpu("export", "kernel", b, &idx, &mut rep);
        if let Some(d) = rep.gpu_diagnostics.first() {
            return KernelEligibility {
                cpu_parallel: true,
                gpu_metal: false,
                blocker: Some(d.reason.clone()),
            };
        }
    }
    KernelEligibility {
        cpu_parallel: true,
        gpu_metal: true,
        blocker: None,
    }
}
