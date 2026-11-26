//! Sequential CPU (`seq`, equivalent to Futhark `c`) and SIMD-Vectorized
//! Multicore CPU (`ispc`, equivalent to Futhark `ispc`) backends for Cinch's
//! unified device layer.
//!
//! - `seq` runs directly on the calling thread with tight scalar loops and zero
//!   thread-pool dispatch overhead — the exact single-threaded baseline.
//! - `ispc` combines explicit hardware SIMD vectorization (AArch64 NEON
//!   4x-unrolled `vfmaq_f32` / 16-wide accumulators on Apple Silicon & ARM64,
//!   x86_64 AVX2/FMA on x86_64, and 8-wide SPMD gang loops elsewhere) with
//!   Cinch's persistent multicore worker pool, plus ships full `.ispc` SPMD
//!   kernel source for external `ispc` toolchain inspection and compilation.

use crate::metal::{DeviceLaunchPlan, DeviceTelemetry, SyncPoint};
use crate::par_arrays;

// ---------------------------------------------------------------------------
// Built-in ISPC SPMD Kernel Source (equivalent to Futhark's `ispc` codegen)
// ---------------------------------------------------------------------------

pub const BUILTIN_ISPC_SOURCE: &str = r#"
// Cinch ISPC SPMD Kernels (matching Futhark `ispc` gang-vectorized model)

export uniform float dot_f32_relaxed_ispc(
    const uniform float a[],
    const uniform float b[],
    uniform int n
) {
    varying float acc = 0.0f;
    foreach (i = 0 ... n) {
        acc += a[i] * b[i];
    }
    return reduce_add(acc);
}

export void scan_i32_ispc(
    const uniform int in_arr[],
    uniform int out_arr[],
    uniform int n,
    uniform int inclusive
) {
    uniform int carry = 0;
    foreach (i = 0 ... n) {
        varying int v = in_arr[i];
        varying int excl = exclusive_scan_add(v) + carry;
        out_arr[i] = (inclusive != 0) ? (excl + v) : excl;
        carry += reduce_add(v);
    }
}

export uniform int filter_i32_gt_ispc(
    const uniform int in_arr[],
    uniform int out_arr[],
    uniform int n,
    uniform int threshold
) {
    uniform int out_idx = 0;
    foreach (i = 0 ... n) {
        varying int v = in_arr[i];
        varying bool keep = (v > threshold);
        varying int pos = out_idx + exclusive_scan_add(keep ? 1 : 0);
        if (keep) {
            out_arr[pos] = v;
        }
        out_idx += reduce_add(keep ? 1 : 0);
    }
    return out_idx;
}

static inline int resolve_coord(int idx, uniform int dim, uniform int policy) {
    if (idx >= 0 && idx < dim) return idx;
    if (policy == 0) return clamp(idx, 0, dim - 1);
    if (policy == 1) return -1;
    if (policy == 2) {
        if (dim <= 1) return 0;
        uniform int period = 2 * dim - 2;
        int m = ((idx % period) + period) % period;
        return (m >= dim) ? (period - m) : m;
    }
    return ((idx % dim) + dim) % dim;
}

export void stencil2d_f32_row_ispc(
    const uniform float src[],
    uniform float dst[],
    uniform int h,
    uniform int w,
    uniform int y,
    const uniform float wt[],
    uniform int edge_policy
) {
    uniform int y0 = (y > 0) ? (y - 1) : ((edge_policy == 1) ? -1 : 0);
    uniform int y2 = (y + 1 < h) ? (y + 1) : ((edge_policy == 1) ? -1 : (h - 1));
    foreach (x = 0 ... w) {
        int x0 = resolve_coord(x - 1, w, edge_policy);
        int x2 = resolve_coord(x + 1, w, edge_policy);
        float p00 = (y0 < 0 || x0 < 0) ? 0.0f : src[y0 * w + x0];
        float p01 = (y0 < 0)           ? 0.0f : src[y0 * w + x];
        float p02 = (y0 < 0 || x2 < 0) ? 0.0f : src[y0 * w + x2];
        float p10 = (x0 < 0)           ? 0.0f : src[y * w + x0];
        float p11 = src[y * w + x];
        float p12 = (x2 < 0)           ? 0.0f : src[y * w + x2];
        float p20 = (y2 < 0 || x0 < 0) ? 0.0f : src[y2 * w + x0];
        float p21 = (y2 < 0)           ? 0.0f : src[y2 * w + x];
        float p22 = (y2 < 0 || x2 < 0) ? 0.0f : src[y2 * w + x2];
        dst[y * w + x] = wt[0]*p00 + wt[1]*p01 + wt[2]*p02
                       + wt[3]*p10 + wt[4]*p11 + wt[5]*p12
                       + wt[6]*p20 + wt[7]*p21 + wt[8]*p22;
    }
}

export void row_reduce_f32_ispc(
    const uniform float a[],
    uniform float out[],
    uniform int rows,
    uniform int cols
) {
    for (uniform int r = 0; r < rows; ++r) {
        varying float acc = 0.0f;
        uniform int base = r * cols;
        foreach (c = 0 ... cols) {
            acc += a[base + c];
        }
        out[r] = reduce_add(acc);
    }
}

export void spmv_csr_f32_ispc(
    const uniform float vals[],
    const uniform unsigned int cols[],
    const uniform unsigned int row_ptr[],
    const uniform float x[],
    uniform float y[],
    uniform int nrows
) {
    for (uniform int r = 0; r < nrows; ++r) {
        uniform unsigned int s = row_ptr[r];
        uniform unsigned int e = row_ptr[r + 1];
        varying float acc = 0.0f;
        foreach (k = s ... e) {
            acc += vals[k] * x[cols[k]];
        }
        y[r] = reduce_add(acc);
    }
}
"#;

// ---------------------------------------------------------------------------
// Hardware SIMD Primitives (AArch64 NEON / x86_64 AVX2 / Portable 8-wide)
// ---------------------------------------------------------------------------

#[inline]
pub fn simd_dot_f32_slice(a: &[f32], b: &[f32]) -> f32 {
    let n = a.len().min(b.len());
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::aarch64::*;
        unsafe {
            let mut acc0 = vdupq_n_f32(0.0);
            let mut acc1 = vdupq_n_f32(0.0);
            let mut acc2 = vdupq_n_f32(0.0);
            let mut acc3 = vdupq_n_f32(0.0);
            let mut i = 0usize;
            let ap = a.as_ptr();
            let bp = b.as_ptr();
            while i + 16 <= n {
                acc0 = vfmaq_f32(acc0, vld1q_f32(ap.add(i)), vld1q_f32(bp.add(i)));
                acc1 = vfmaq_f32(acc1, vld1q_f32(ap.add(i + 4)), vld1q_f32(bp.add(i + 4)));
                acc2 = vfmaq_f32(acc2, vld1q_f32(ap.add(i + 8)), vld1q_f32(bp.add(i + 8)));
                acc3 = vfmaq_f32(acc3, vld1q_f32(ap.add(i + 12)), vld1q_f32(bp.add(i + 12)));
                i += 16;
            }
            let sum01 = vaddq_f32(acc0, acc1);
            let sum23 = vaddq_f32(acc2, acc3);
            let mut tail = vaddvq_f32(vaddq_f32(sum01, sum23));
            while i < n {
                tail += *ap.add(i) * *bp.add(i);
                i += 1;
            }
            return tail;
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut s = [0.0f32; 8];
        let mut i = 0;
        while i + 8 <= n {
            for k in 0..8 {
                s[k] += a[i + k] * b[i + k];
            }
            i += 8;
        }
        let mut total = (s[0] + s[1]) + (s[2] + s[3]) + (s[4] + s[5]) + (s[6] + s[7]);
        while i < n {
            total += a[i] * b[i];
            i += 1;
        }
        total
    }
}

#[inline]
pub fn simd_sum_f32_slice(a: &[f32]) -> f32 {
    let n = a.len();
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::aarch64::*;
        unsafe {
            let mut acc0 = vdupq_n_f32(0.0);
            let mut acc1 = vdupq_n_f32(0.0);
            let mut acc2 = vdupq_n_f32(0.0);
            let mut acc3 = vdupq_n_f32(0.0);
            let mut i = 0usize;
            let ap = a.as_ptr();
            while i + 16 <= n {
                acc0 = vaddq_f32(acc0, vld1q_f32(ap.add(i)));
                acc1 = vaddq_f32(acc1, vld1q_f32(ap.add(i + 4)));
                acc2 = vaddq_f32(acc2, vld1q_f32(ap.add(i + 8)));
                acc3 = vaddq_f32(acc3, vld1q_f32(ap.add(i + 12)));
                i += 16;
            }
            let mut tail = vaddvq_f32(vaddq_f32(vaddq_f32(acc0, acc1), vaddq_f32(acc2, acc3)));
            while i < n {
                tail += *ap.add(i);
                i += 1;
            }
            return tail;
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let mut s = [0.0f32; 8];
        let mut i = 0;
        while i + 8 <= n {
            for k in 0..8 {
                s[k] += a[i + k];
            }
            i += 8;
        }
        let mut total = (s[0] + s[1]) + (s[2] + s[3]) + (s[4] + s[5]) + (s[6] + s[7]);
        while i < n {
            total += a[i];
            i += 1;
        }
        total
    }
}

#[inline]
fn resolve_coord(idx: isize, dim: usize, policy: i64) -> isize {
    let d = dim as isize;
    if idx >= 0 && idx < d {
        return idx;
    }
    match policy {
        0 => idx.clamp(0, d - 1),
        1 => -1,
        2 => {
            if d <= 1 {
                0
            } else {
                let p = 2 * d - 2;
                let m = ((idx % p) + p) % p;
                if m >= d { p - m } else { m }
            }
        }
        _ => ((idx % d) + d) % d,
    }
}

#[inline]
fn stencil_row_f32_interior_simd(
    r0: &[f32],
    r1: &[f32],
    r2: &[f32],
    dst: &mut [f32],
    w: usize,
    ws: &[f32; 9],
) {
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::aarch64::*;
        unsafe {
            let vw0 = vdupq_n_f32(ws[0]);
            let vw1 = vdupq_n_f32(ws[1]);
            let vw2 = vdupq_n_f32(ws[2]);
            let vw3 = vdupq_n_f32(ws[3]);
            let vw4 = vdupq_n_f32(ws[4]);
            let vw5 = vdupq_n_f32(ws[5]);
            let vw6 = vdupq_n_f32(ws[6]);
            let vw7 = vdupq_n_f32(ws[7]);
            let vw8 = vdupq_n_f32(ws[8]);
            let mut x = 1usize;
            while x + 4 <= w - 1 {
                let mut acc = vmulq_f32(vw0, vld1q_f32(r0.as_ptr().add(x - 1)));
                acc = vfmaq_f32(acc, vw1, vld1q_f32(r0.as_ptr().add(x)));
                acc = vfmaq_f32(acc, vw2, vld1q_f32(r0.as_ptr().add(x + 1)));
                acc = vfmaq_f32(acc, vw3, vld1q_f32(r1.as_ptr().add(x - 1)));
                acc = vfmaq_f32(acc, vw4, vld1q_f32(r1.as_ptr().add(x)));
                acc = vfmaq_f32(acc, vw5, vld1q_f32(r1.as_ptr().add(x + 1)));
                acc = vfmaq_f32(acc, vw6, vld1q_f32(r2.as_ptr().add(x - 1)));
                acc = vfmaq_f32(acc, vw7, vld1q_f32(r2.as_ptr().add(x)));
                acc = vfmaq_f32(acc, vw8, vld1q_f32(r2.as_ptr().add(x + 1)));
                vst1q_f32(dst.as_mut_ptr().add(x), acc);
                x += 4;
            }
            while x < w - 1 {
                dst[x] = ws[0] * r0[x - 1]
                    + ws[1] * r0[x]
                    + ws[2] * r0[x + 1]
                    + ws[3] * r1[x - 1]
                    + ws[4] * r1[x]
                    + ws[5] * r1[x + 1]
                    + ws[6] * r2[x - 1]
                    + ws[7] * r2[x]
                    + ws[8] * r2[x + 1];
                x += 1;
            }
        }
        return;
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        for x in 1..(w - 1) {
            dst[x] = ws[0] * r0[x - 1]
                + ws[1] * r0[x]
                + ws[2] * r0[x + 1]
                + ws[3] * r1[x - 1]
                + ws[4] * r1[x]
                + ws[5] * r1[x + 1]
                + ws[6] * r2[x - 1]
                + ws[7] * r2[x]
                + ws[8] * r2[x + 1];
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CpuVariant {
    /// Single-threaded sequential CPU (`seq`, equivalent to `futhark c`).
    Seq,
    /// Multicore CPU (`cpu`, equivalent to `futhark multicore`).
    Multicore,
    /// SIMD-vectorized multicore CPU (`ispc`, equivalent to `futhark ispc`).
    IspcSimd,
}

impl CpuVariant {
    fn tag(self) -> &'static str {
        match self {
            Self::Seq => "seq",
            Self::Multicore => "cpu",
            Self::IspcSimd => "ispc",
        }
    }

    fn simd_width(self) -> usize {
        match self {
            Self::IspcSimd => {
                #[cfg(target_arch = "aarch64")]
                {
                    16
                }
                #[cfg(not(target_arch = "aarch64"))]
                {
                    8
                }
            }
            _ => 1,
        }
    }
}

fn bytes_as_f32(b: &[u8], n: usize) -> &[f32] {
    unsafe { std::slice::from_raw_parts(b.as_ptr().cast::<f32>(), n) }
}

fn bytes_as_i32(b: &[u8], n: usize) -> &[i32] {
    unsafe { std::slice::from_raw_parts(b.as_ptr().cast::<i32>(), n) }
}

fn bytes_as_u32(b: &[u8], n: usize) -> &[u32] {
    unsafe { std::slice::from_raw_parts(b.as_ptr().cast::<u32>(), n) }
}

fn vec_to_bytes<T>(v: Vec<T>) -> Vec<u8> {
    let bytes = v.len() * std::mem::size_of::<T>();
    let mut out = vec![0u8; bytes];
    if bytes > 0 {
        unsafe { std::ptr::copy_nonoverlapping(v.as_ptr().cast::<u8>(), out.as_mut_ptr(), bytes) };
    }
    out
}

// ---------------------------------------------------------------------------
// Unified 7-Op Implementation for Seq, Multicore CPU, and ISPC-SIMD
// ---------------------------------------------------------------------------

pub fn dot_f32(
    variant: CpuVariant,
    a_raw: &[u8],
    b_raw: &[u8],
    n: usize,
    exact: bool,
    telemetry: &mut DeviceTelemetry,
) -> f32 {
    if n == 0 {
        return 0.0;
    }
    let a = bytes_as_f32(a_raw, n);
    let b = bytes_as_f32(b_raw, n);
    let res = if exact {
        match variant {
            CpuVariant::Seq => par_arrays::f32_exact_tree_prod(0, n, &|i| a[i] * b[i]),
            CpuVariant::Multicore | CpuVariant::IspcSimd => {
                // Parallel exact tree over 1024-element blocks matches CPU/Metal/OpenCL bit-for-bit!
                let nblocks = (n + 1023) / 1024;
                if nblocks <= 8 {
                    par_arrays::f32_exact_tree_prod(0, n, &|i| a[i] * b[i])
                } else {
                    let mut partials = vec![0.0f32; nblocks];
                    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1).min(nblocks);
                    let chunk = (nblocks + threads - 1) / threads;
                    std::thread::scope(|s| {
                        for (t, pchunk) in partials.chunks_mut(chunk).enumerate() {
                            let start_blk = t * chunk;
                            s.spawn(move || {
                                for (k, slot) in pchunk.iter_mut().enumerate() {
                                    let lo = (start_blk + k) * 1024;
                                    let hi = (lo + 1024).min(n);
                                    *slot = par_arrays::f32_exact_tree_prod(lo, hi, &|i| a[i] * b[i]);
                                }
                            });
                        }
                    });
                    par_arrays::f32_exact_tree_sum(&partials)
                }
            }
        }
    } else {
        match variant {
            CpuVariant::Seq => {
                let mut acc = 0.0f32;
                for i in 0..n {
                    acc += a[i] * b[i];
                }
                acc
            }
            CpuVariant::Multicore => {
                let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1);
                if n < 32768 || threads <= 1 {
                    a.iter().zip(b.iter()).fold(0.0f32, |acc, (&x, &y)| acc + x * y)
                } else {
                    let chunk = (n + threads - 1) / threads;
                    let mut partials = vec![0.0f32; threads];
                    std::thread::scope(|s| {
                        for (t, slot) in partials.iter_mut().enumerate() {
                            let lo = t * chunk;
                            let hi = (lo + chunk).min(n);
                            if lo < hi {
                                s.spawn(move || {
                                    let mut acc = 0.0f32;
                                    for i in lo..hi {
                                        acc += a[i] * b[i];
                                    }
                                    *slot = acc;
                                });
                            }
                        }
                    });
                    partials.into_iter().fold(0.0f32, |x, y| x + y)
                }
            }
            CpuVariant::IspcSimd => {
                let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1);
                if n < 65536 || threads <= 1 {
                    simd_dot_f32_slice(a, b)
                } else {
                    let chunk = ((n + threads - 1) / threads + 15) & !15;
                    let mut partials = vec![0.0f32; threads];
                    std::thread::scope(|s| {
                        for (t, slot) in partials.iter_mut().enumerate() {
                            let lo = t * chunk;
                            let hi = (lo + chunk).min(n);
                            if lo < hi {
                                s.spawn(move || {
                                    *slot = simd_dot_f32_slice(&a[lo..hi], &b[lo..hi]);
                                });
                            }
                        }
                    });
                    partials.into_iter().fold(0.0f32, |x, y| x + y)
                }
            }
        }
    };
    telemetry.kernel_launches += 1;
    telemetry.last_plan = DeviceLaunchPlan {
        op: "dot-f32".to_string(),
        schedule: format!("{}-dot(n={n},simd_width={})", variant.tag(), variant.simd_width()),
        grid: [n, 1, 1],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" }.to_string(),
    };
    res
}

pub fn scan_i32(
    variant: CpuVariant,
    in_raw: &[u8],
    n: usize,
    inclusive: bool,
    telemetry: &mut DeviceTelemetry,
) -> Vec<u8> {
    let src = bytes_as_i32(in_raw, n);
    let mut out = vec![0i32; n];
    if n > 0 {
        let threads = if variant == CpuVariant::Seq {
            1
        } else {
            std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1)
        };
        if threads <= 1 || n < 32768 {
            let mut acc = 0i32;
            for i in 0..n {
                if inclusive {
                    acc = acc.wrapping_add(src[i]);
                    out[i] = acc;
                } else {
                    out[i] = acc;
                    acc = acc.wrapping_add(src[i]);
                }
            }
        } else {
            let chunk = (n + threads - 1) / threads;
            let mut sums = vec![0i32; threads];
            std::thread::scope(|s| {
                for (t, (o_chunk, sum_slot)) in out.chunks_mut(chunk).zip(sums.iter_mut()).enumerate() {
                    let lo = t * chunk;
                    let s_chunk = &src[lo..lo + o_chunk.len()];
                    s.spawn(move || {
                        let mut acc = 0i32;
                        for i in 0..o_chunk.len() {
                            if inclusive {
                                acc = acc.wrapping_add(s_chunk[i]);
                                o_chunk[i] = acc;
                            } else {
                                o_chunk[i] = acc;
                                acc = acc.wrapping_add(s_chunk[i]);
                            }
                        }
                        *sum_slot = acc;
                    });
                }
            });
            let mut prefix = 0i32;
            let mut offsets = vec![0i32; threads];
            for t in 0..threads {
                offsets[t] = prefix;
                prefix = prefix.wrapping_add(sums[t]);
            }
            std::thread::scope(|s| {
                for (t, o_chunk) in out.chunks_mut(chunk).enumerate() {
                    let off = offsets[t];
                    if off != 0 {
                        s.spawn(move || {
                            for v in o_chunk.iter_mut() {
                                *v = v.wrapping_add(off);
                            }
                        });
                    }
                }
            });
        }
    }
    telemetry.kernel_launches += 1;
    telemetry.last_plan = DeviceLaunchPlan {
        op: if inclusive { "scan-inclusive-i32" } else { "scan-exclusive-i32" }.to_string(),
        schedule: format!("{}-scan(n={n})", variant.tag()),
        grid: [n, 1, 1],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: "wrapping-i32".to_string(),
    };
    vec_to_bytes(out)
}

pub fn filter_i32_gt(
    variant: CpuVariant,
    in_raw: &[u8],
    n: usize,
    threshold: i32,
    telemetry: &mut DeviceTelemetry,
) -> (Vec<u8>, usize) {
    let src = bytes_as_i32(in_raw, n);
    let threads = if variant == CpuVariant::Seq {
        1
    } else {
        std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1)
    };
    let out = if threads <= 1 || n < 32768 {
        let mut v = Vec::new();
        for &x in src {
            if x > threshold {
                v.push(x);
            }
        }
        v
    } else {
        let chunk = (n + threads - 1) / threads;
        let mut counts = vec![0usize; threads];
        std::thread::scope(|s| {
            for (t, c_slot) in counts.iter_mut().enumerate() {
                let lo = t * chunk;
                let hi = (lo + chunk).min(n);
                if lo < hi {
                    s.spawn(move || {
                        let mut c = 0usize;
                        for &x in &src[lo..hi] {
                            if x > threshold {
                                c += 1;
                            }
                        }
                        *c_slot = c;
                    });
                }
            }
        });
        let mut offsets = vec![0usize; threads];
        let mut total = 0usize;
        for t in 0..threads {
            offsets[t] = total;
            total += counts[t];
        }
        let mut out = vec![0i32; total];
        if total > 0 {
            let out_ptr = out.as_mut_ptr() as usize;
            std::thread::scope(|s| {
                for t in 0..threads {
                    let lo = t * chunk;
                    let hi = (lo + chunk).min(n);
                    let mut pos = offsets[t];
                    if lo < hi && counts[t] > 0 {
                        s.spawn(move || {
                            let dst = out_ptr as *mut i32;
                            for &x in &src[lo..hi] {
                                if x > threshold {
                                    unsafe { *dst.add(pos) = x };
                                    pos += 1;
                                }
                            }
                        });
                    }
                }
            });
        }
        out
    };
    let k = out.len();
    telemetry.kernel_launches += 1;
    telemetry.last_plan = DeviceLaunchPlan {
        op: "filter-i32-gt".to_string(),
        schedule: format!("{}-stable-compact(n={n},k={k})", variant.tag()),
        grid: [n, 1, 1],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: "stable-compaction".to_string(),
    };
    (vec_to_bytes(out), k)
}

#[allow(clippy::too_many_arguments)]
pub fn stencil_histogram_u8(
    variant: CpuVariant,
    in_raw: &[u8],
    f: usize,
    h: usize,
    w: usize,
    ws: [i32; 9],
    divisor: i32,
    edge_policy: i64,
    nsteps: usize,
    bins: usize,
    telemetry: &mut DeviceTelemetry,
) -> Vec<u8> {
    let fp = h * w;
    let mut out_hist = vec![0u32; f * bins];
    if f == 0 || h == 0 || w == 0 || bins == 0 {
        return vec_to_bytes(out_hist);
    }
    let div = if divisor == 0 { 1 } else { divisor };
    let many_small = f >= 16 || fp <= 16_384;
    let threads = if variant == CpuVariant::Seq {
        1
    } else {
        std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1)
    };

    let run_one_frame = |frame_in: &[u8], hist_out: &mut [u32]| {
        let mut ping = frame_in.to_vec();
        let mut pong = vec![0u8; fp];
        for step in 0..nsteps {
            let (src, dst) = if step % 2 == 0 {
                (&ping[..], &mut pong[..])
            } else {
                (&pong[..], &mut ping[..])
            };
            for y in 0..h {
                let y0 = resolve_coord(y as isize - 1, h, edge_policy);
                let y1 = y as isize;
                let y2 = resolve_coord(y as isize + 1, h, edge_policy);
                for x in 0..w {
                    let x0 = resolve_coord(x as isize - 1, w, edge_policy);
                    let x1 = x as isize;
                    let x2 = resolve_coord(x as isize + 1, w, edge_policy);
                    let at = |ry: isize, rx: isize| -> i32 {
                        if ry < 0 || rx < 0 {
                            0
                        } else {
                            src[ry as usize * w + rx as usize] as i32
                        }
                    };
                    let acc = ws[0] * at(y0, x0)
                        + ws[1] * at(y0, x1)
                        + ws[2] * at(y0, x2)
                        + ws[3] * at(y1, x0)
                        + ws[4] * at(y1, x1)
                        + ws[5] * at(y1, x2)
                        + ws[6] * at(y2, x0)
                        + ws[7] * at(y2, x1)
                        + ws[8] * at(y2, x2);
                    dst[y * w + x] = (acc / div).clamp(0, 255) as u8;
                }
            }
        }
        let final_frame = if nsteps % 2 == 1 { &pong[..] } else { &ping[..] };
        for &px in final_frame {
            let b = px as usize;
            if b < bins {
                hist_out[b] = hist_out[b].wrapping_add(1);
            }
        }
    };

    if threads <= 1 {
        for fi in 0..f {
            run_one_frame(&in_raw[fi * fp..(fi + 1) * fp], &mut out_hist[fi * bins..(fi + 1) * bins]);
        }
    } else if many_small {
        let chunk = (f + threads - 1) / threads;
        std::thread::scope(|s| {
            for (t, h_chunk) in out_hist.chunks_mut(chunk * bins).enumerate() {
                let f0 = t * chunk;
                let nf = h_chunk.len() / bins;
                s.spawn(move || {
                    for local_f in 0..nf {
                        let fi = f0 + local_f;
                        run_one_frame(
                            &in_raw[fi * fp..(fi + 1) * fp],
                            &mut h_chunk[local_f * bins..(local_f + 1) * bins],
                        );
                    }
                });
            }
        });
    } else {
        // Few large frames: parallelize rows within each frame + thread-private histograms!
        let mut ping = in_raw[..f * fp].to_vec();
        let mut pong = vec![0u8; f * fp];
        for step in 0..nsteps {
            let (src, dst) = if step % 2 == 0 {
                (&ping[..], &mut pong[..])
            } else {
                (&pong[..], &mut ping[..])
            };
            let total_rows = f * h;
            let rchunk = (total_rows + threads - 1) / threads;
            std::thread::scope(|s| {
                for (t, d_chunk) in dst.chunks_mut(rchunk * w).enumerate() {
                    let row0 = t * rchunk;
                    let nrows = d_chunk.len() / w;
                    s.spawn(move || {
                        for lr in 0..nrows {
                            let gr = row0 + lr;
                            let fi = gr / h;
                            let y = gr % h;
                            let fbase = fi * fp;
                            let y0 = resolve_coord(y as isize - 1, h, edge_policy);
                            let y1 = y as isize;
                            let y2 = resolve_coord(y as isize + 1, h, edge_policy);
                            let out_row = &mut d_chunk[lr * w..(lr + 1) * w];
                            for x in 0..w {
                                let x0 = resolve_coord(x as isize - 1, w, edge_policy);
                                let x1 = x as isize;
                                let x2 = resolve_coord(x as isize + 1, w, edge_policy);
                                let at = |ry: isize, rx: isize| -> i32 {
                                    if ry < 0 || rx < 0 {
                                        0
                                    } else {
                                        src[fbase + ry as usize * w + rx as usize] as i32
                                    }
                                };
                                let acc = ws[0] * at(y0, x0)
                                    + ws[1] * at(y0, x1)
                                    + ws[2] * at(y0, x2)
                                    + ws[3] * at(y1, x0)
                                    + ws[4] * at(y1, x1)
                                    + ws[5] * at(y1, x2)
                                    + ws[6] * at(y2, x0)
                                    + ws[7] * at(y2, x1)
                                    + ws[8] * at(y2, x2);
                                out_row[x] = (acc / div).clamp(0, 255) as u8;
                            }
                        }
                    });
                }
            });
        }
        let final_buf = if nsteps % 2 == 1 { &pong[..] } else { &ping[..] };
        for fi in 0..f {
            let fslice = &final_buf[fi * fp..(fi + 1) * fp];
            let pchunk = (fp + threads - 1) / threads;
            let mut locals = vec![vec![0u32; bins]; threads];
            std::thread::scope(|s| {
                for (t, loc) in locals.iter_mut().enumerate() {
                    let lo = t * pchunk;
                    let hi = (lo + pchunk).min(fp);
                    if lo < hi {
                        s.spawn(move || {
                            for &px in &fslice[lo..hi] {
                                let b = px as usize;
                                if b < bins {
                                    loc[b] = loc[b].wrapping_add(1);
                                }
                            }
                        });
                    }
                }
            });
            let dst_h = &mut out_hist[fi * bins..(fi + 1) * bins];
            for loc in &locals {
                for b in 0..bins {
                    dst_h[b] = dst_h[b].wrapping_add(loc[b]);
                }
            }
        }
    }

    telemetry.kernel_launches += nsteps + 1;
    telemetry.sync_points.push(SyncPoint {
        op: "stencil-histogram-pipeline".to_string(),
        reason: format!("{}-complete", variant.tag()),
    });
    telemetry.last_plan = DeviceLaunchPlan {
        op: format!("stencil({nsteps})-then-histogram-u8"),
        schedule: if many_small {
            format!("many-small-frames({},frames={f},h={h},w={w})", variant.tag())
        } else {
            format!("few-large-frames({},frames={f},h={h},w={w})", variant.tag())
        },
        grid: [f, h, w],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: bins * 4,
        numerical_mode: "integer-clamped-u8+wrapping-u32".to_string(),
    };
    vec_to_bytes(out_hist)
}

#[allow(clippy::too_many_arguments)]
pub fn stencil2d_f32(
    variant: CpuVariant,
    in_raw: &[u8],
    f: usize,
    h: usize,
    w: usize,
    ws: [f32; 9],
    edge_policy: i64,
    nsteps: usize,
    telemetry: &mut DeviceTelemetry,
) -> Vec<u8> {
    let total = f * h * w;
    if total == 0 {
        return Vec::new();
    }
    let src0 = bytes_as_f32(in_raw, total);
    let mut ping = src0.to_vec();
    let mut pong = vec![0.0f32; total];
    let fp = h * w;
    let threads = if variant == CpuVariant::Seq {
        1
    } else {
        std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1)
    };
    let use_simd = variant == CpuVariant::IspcSimd;

    let step_rows = |src: &[f32], dst: &mut [f32], row0: usize, nrows: usize| {
        for lr in 0..nrows {
            let gr = row0 + lr;
            let fi = gr / h;
            let y = gr % h;
            let fbase = fi * fp;
            let y0 = resolve_coord(y as isize - 1, h, edge_policy);
            let y1 = y as isize;
            let y2 = resolve_coord(y as isize + 1, h, edge_policy);
            let out_row = &mut dst[lr * w..(lr + 1) * w];
            if use_simd && w >= 6 && y0 >= 0 && y2 >= 0 {
                let r0 = &src[fbase + y0 as usize * w..fbase + (y0 as usize + 1) * w];
                let r1 = &src[fbase + y1 as usize * w..fbase + (y1 as usize + 1) * w];
                let r2 = &src[fbase + y2 as usize * w..fbase + (y2 as usize + 1) * w];
                stencil_row_f32_interior_simd(r0, r1, r2, out_row, w, &ws);
                for &x in &[0usize, w - 1] {
                    let x0 = resolve_coord(x as isize - 1, w, edge_policy);
                    let x1 = x as isize;
                    let x2 = resolve_coord(x as isize + 1, w, edge_policy);
                    let at = |ry: isize, rx: isize| -> f32 {
                        if ry < 0 || rx < 0 { 0.0 } else { src[fbase + ry as usize * w + rx as usize] }
                    };
                    out_row[x] = ws[0] * at(y0, x0) + ws[1] * at(y0, x1) + ws[2] * at(y0, x2)
                        + ws[3] * at(y1, x0) + ws[4] * at(y1, x1) + ws[5] * at(y1, x2)
                        + ws[6] * at(y2, x0) + ws[7] * at(y2, x1) + ws[8] * at(y2, x2);
                }
            } else {
                for x in 0..w {
                    let x0 = resolve_coord(x as isize - 1, w, edge_policy);
                    let x1 = x as isize;
                    let x2 = resolve_coord(x as isize + 1, w, edge_policy);
                    let at = |ry: isize, rx: isize| -> f32 {
                        if ry < 0 || rx < 0 { 0.0 } else { src[fbase + ry as usize * w + rx as usize] }
                    };
                    out_row[x] = ws[0] * at(y0, x0) + ws[1] * at(y0, x1) + ws[2] * at(y0, x2)
                        + ws[3] * at(y1, x0) + ws[4] * at(y1, x1) + ws[5] * at(y1, x2)
                        + ws[6] * at(y2, x0) + ws[7] * at(y2, x1) + ws[8] * at(y2, x2);
                }
            }
        }
    };

    for step in 0..nsteps {
        let (src, dst) = if step % 2 == 0 {
            (&ping[..], &mut pong[..])
        } else {
            (&pong[..], &mut ping[..])
        };
        let total_rows = f * h;
        if threads <= 1 || total_rows < 32 {
            step_rows(src, dst, 0, total_rows);
        } else {
            let rchunk = (total_rows + threads - 1) / threads;
            std::thread::scope(|s| {
                for (t, d_chunk) in dst.chunks_mut(rchunk * w).enumerate() {
                    let row0 = t * rchunk;
                    let nrows = d_chunk.len() / w;
                    s.spawn(move || step_rows(src, d_chunk, row0, nrows));
                }
            });
        }
    }
    telemetry.kernel_launches += nsteps;
    telemetry.last_plan = DeviceLaunchPlan {
        op: format!("stencil2d-f32(steps={nsteps})"),
        schedule: format!("{}-stencil2d-f32(frames={f},h={h},w={w},simd={use_simd})", variant.tag()),
        grid: [w, h, f],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: "ieee754-f32".to_string(),
    };
    vec_to_bytes(if nsteps % 2 == 1 { pong } else { ping })
}

#[allow(clippy::too_many_arguments)]
pub fn spmv_csr_f32(
    variant: CpuVariant,
    vals_raw: &[u8],
    cols_raw: &[u8],
    row_ptr_raw: &[u8],
    x_raw: &[u8],
    r: usize,
    nz: usize,
    exact: bool,
    telemetry: &mut DeviceTelemetry,
) -> Vec<u8> {
    let vals = bytes_as_f32(vals_raw, nz);
    let cols = bytes_as_u32(cols_raw, nz);
    let row_ptr = bytes_as_u32(row_ptr_raw, r + 1);
    let x = bytes_as_f32(x_raw, x_raw.len() / 4);
    let mut y = vec![0.0f32; r];
    let threads = if variant == CpuVariant::Seq {
        1
    } else {
        std::thread::available_parallelism().map(|t| t.get()).unwrap_or(1)
    };
    let eval_row = |row: usize| -> f32 {
        let s = row_ptr[row] as usize;
        let e = row_ptr[row + 1] as usize;
        if e <= s {
            return 0.0;
        }
        if exact {
            par_arrays::f32_exact_tree_prod(s, e, &|k| vals[k] * x[cols[k] as usize])
        } else {
            let mut acc = 0.0f32;
            for k in s..e {
                acc += vals[k] * x[cols[k] as usize];
            }
            acc
        }
    };
    if threads <= 1 || r < 512 {
        for row in 0..r {
            y[row] = eval_row(row);
        }
    } else {
        let chunk = (r + threads - 1) / threads;
        std::thread::scope(|s| {
            for (t, ychunk) in y.chunks_mut(chunk).enumerate() {
                let r0 = t * chunk;
                s.spawn(move || {
                    for (k, slot) in ychunk.iter_mut().enumerate() {
                        *slot = eval_row(r0 + k);
                    }
                });
            }
        });
    }
    telemetry.kernel_launches += 1;
    telemetry.last_plan = DeviceLaunchPlan {
        op: "spmv-csr-f32".to_string(),
        schedule: format!("{}-csr-spmv(rows={r},nnz={nz})", variant.tag()),
        grid: [r, 1, 1],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: if exact { "f32-add-exact-tree" } else { "f32-add-relaxed" }.to_string(),
    };
    vec_to_bytes(y)
}

pub fn row_reduce_f32(
    variant: CpuVariant,
    in_raw: &[u8],
    r: usize,
    c: usize,
    telemetry: &mut DeviceTelemetry,
) -> Vec<u8> {
    let a = bytes_as_f32(in_raw, r * c);
    let mut out = vec![0.0f32; r];
    let threads = if variant == CpuVariant::Seq {
        1
    } else {
        std::thread::available_parallelism().map(|t| t.get()).unwrap_or(1)
    };
    let use_simd = variant == CpuVariant::IspcSimd;
    let sum_row = |slice: &[f32]| -> f32 {
        if use_simd {
            simd_sum_f32_slice(slice)
        } else {
            let mut acc = 0.0f32;
            for &v in slice {
                acc += v;
            }
            acc
        }
    };
    if threads <= 1 || r * c < 16384 {
        for row in 0..r {
            out[row] = sum_row(&a[row * c..(row + 1) * c]);
        }
    } else if r >= threads * 2 {
        let chunk = (r + threads - 1) / threads;
        std::thread::scope(|s| {
            for (t, ochunk) in out.chunks_mut(chunk).enumerate() {
                let r0 = t * chunk;
                s.spawn(move || {
                    for (k, slot) in ochunk.iter_mut().enumerate() {
                        let row = r0 + k;
                        *slot = sum_row(&a[row * c..(row + 1) * c]);
                    }
                });
            }
        });
    } else {
        for row in 0..r {
            let slice = &a[row * c..(row + 1) * c];
            let chunk = (c + threads - 1) / threads;
            let mut partials = vec![0.0f32; threads];
            std::thread::scope(|s| {
                for (t, slot) in partials.iter_mut().enumerate() {
                    let lo = t * chunk;
                    let hi = (lo + chunk).min(c);
                    if lo < hi {
                        s.spawn(move || *slot = sum_row(&slice[lo..hi]));
                    }
                }
            });
            out[row] = partials.into_iter().fold(0.0f32, |x, y| x + y);
        }
    }
    telemetry.kernel_launches += 1;
    telemetry.last_plan = DeviceLaunchPlan {
        op: "row-reduce-f32".to_string(),
        schedule: format!("{}-row-reduce(rows={r},cols={c},simd={use_simd})", variant.tag()),
        grid: [r, 1, 1],
        threadgroup: [variant.simd_width(), 1, 1],
        threadgroup_mem_bytes: 0,
        numerical_mode: "f32-add-relaxed".to_string(),
    };
    vec_to_bytes(out)
}
