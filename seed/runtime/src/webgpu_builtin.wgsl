// ===========================================================================
// Cinch Data-Parallel Array Runtime — Built-in WebGPU (WGSL 1.0) Kernels
// ===========================================================================

// ---------------------------------------------------------------------------
// 1. Fused Dot Product (f32 relaxed workgroup reduction & exact-tree 1024)
// ---------------------------------------------------------------------------

struct DotParams {
    n: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
};

@group(0) @binding(0) var<storage, read> dot_a: array<f32>;
@group(0) @binding(1) var<storage, read> dot_b: array<f32>;
@group(0) @binding(2) var<storage, read_write> dot_partials: array<f32>;
@group(0) @binding(3) var<uniform> dot_params: DotParams;

var<workgroup> dot_sh_256: array<f32, 256>;
var<workgroup> dot_sh_1024: array<f32, 1024>;

@compute @workgroup_size(256, 1, 1)
fn dot_f32_relaxed(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
    @builtin(num_workgroups) num_wg: vec3<u32>,
) {
    let tid = lid.x;
    let n = dot_params.n;
    let gsize = num_wg.x * 256u;
    var acc: f32 = 0.0;
    var i = gid.x;
    loop {
        if (i >= n) { break; }
        acc = acc + dot_a[i] * dot_b[i];
        i = i + gsize;
    }
    dot_sh_256[tid] = acc;
    workgroupBarrier();

    var stride: u32 = 128u;
    loop {
        if (stride == 0u) { break; }
        if (tid < stride) {
            dot_sh_256[tid] = dot_sh_256[tid] + dot_sh_256[tid + stride];
        }
        workgroupBarrier();
        stride = stride >> 1u;
    }
    if (tid == 0u) {
        dot_partials[wid.x] = dot_sh_256[0];
    }
}

@compute @workgroup_size(256, 1, 1)
fn dot_f32_exact_block(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let tid = lid.x;
    let n = dot_params.n;
    let base = wid.x * 1024u;
    for (var k: u32 = 0u; k < 4u; k = k + 1u) {
        let idx = tid + k * 256u;
        let g = base + idx;
        if (g < n) {
            dot_sh_1024[idx] = dot_a[g] * dot_b[g];
        } else {
            dot_sh_1024[idx] = 0.0;
        }
    }
    workgroupBarrier();

    var len: u32 = 0u;
    if (base < n) {
        len = min(n - base, 1024u);
    }

    if (len == 1024u) {
        var stride: u32 = 1u;
        loop {
            if (stride >= 1024u) { break; }
            for (var k: u32 = 0u; k < 4u; k = k + 1u) {
                let idx = tid + k * 256u;
                if ((idx & ((stride << 1u) - 1u)) == 0u) {
                    dot_sh_1024[idx] = dot_sh_1024[idx] + dot_sh_1024[idx + stride];
                }
            }
            workgroupBarrier();
            stride = stride << 1u;
        }
        if (tid == 0u) {
            dot_partials[wid.x] = dot_sh_1024[0];
        }
    } else if (tid == 0u) {
        var vals: array<f32, 12>;
        var sizes: array<u32, 12>;
        var sp: u32 = 0u;
        for (var i: u32 = 0u; i < len; i = i + 1u) {
            var cur = dot_sh_1024[i];
            var sz: u32 = 1u;
            loop {
                if (sp == 0u || sizes[sp - 1u] != sz) { break; }
                cur = vals[sp - 1u] + cur;
                sz = sz << 1u;
                sp = sp - 1u;
            }
            vals[sp] = cur;
            sizes[sp] = sz;
            sp = sp + 1u;
        }
        var acc: f32 = 0.0;
        if (sp > 0u) {
            acc = vals[sp - 1u];
            if (sp >= 2u) {
                var j: i32 = i32(sp) - 2;
                loop {
                    if (j < 0) { break; }
                    acc = vals[u32(j)] + acc;
                    j = j - 1;
                }
            }
        }
        dot_partials[wid.x] = acc;
    }
}

// ---------------------------------------------------------------------------
// 2. Prefix Scan (i32 wrapping, inclusive & exclusive)
// ---------------------------------------------------------------------------

struct ScanParams {
    n: u32,
    inclusive: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> scan_in: array<i32>;
@group(0) @binding(1) var<storage, read_write> scan_out: array<i32>;
@group(0) @binding(2) var<storage, read_write> scan_block_sums: array<i32>;
@group(0) @binding(3) var<uniform> scan_params: ScanParams;

var<workgroup> scan_sh: array<i32, 256>;

@compute @workgroup_size(256, 1, 1)
fn scan_i32_block(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let tid = lid.x;
    let gid = wid.x * 256u + tid;
    var val: i32 = 0;
    if (gid < scan_params.n) {
        val = scan_in[gid];
    }
    scan_sh[tid] = val;
    workgroupBarrier();

    var offset: u32 = 1u;
    loop {
        if (offset >= 256u) { break; }
        var add: i32 = 0;
        if (tid >= offset) {
            add = scan_sh[tid - offset];
        }
        workgroupBarrier();
        scan_sh[tid] = scan_sh[tid] + add;
        workgroupBarrier();
        offset = offset << 1u;
    }

    if (tid == 255u) {
        scan_block_sums[wid.x] = scan_sh[255];
    }
    if (gid < scan_params.n) {
        if (scan_params.inclusive != 0u) {
            scan_out[gid] = scan_sh[tid];
        } else {
            scan_out[gid] = scan_sh[tid] - val;
        }
    }
}

@compute @workgroup_size(256, 1, 1)
fn scan_i32_add_offsets(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let gid = wid.x * 256u + lid.x;
    if (gid < scan_params.n) {
        scan_out[gid] = scan_out[gid] + scan_block_sums[wid.x];
    }
}

// ---------------------------------------------------------------------------
// 3. Stable Compaction (filter i32 > threshold)
// ---------------------------------------------------------------------------

struct FilterParams {
    n: u32,
    threshold: i32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> filt_in: array<i32>;
@group(0) @binding(1) var<storage, read_write> filt_flags: array<i32>;
@group(0) @binding(2) var<storage, read> filt_pos: array<i32>;
@group(0) @binding(3) var<storage, read_write> filt_out: array<i32>;
@group(0) @binding(4) var<uniform> filt_params: FilterParams;

@compute @workgroup_size(256, 1, 1)
fn filter_i32_gt_flags(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < filt_params.n) {
        if (filt_in[i] > filt_params.threshold) {
            filt_flags[i] = 1;
        } else {
            filt_flags[i] = 0;
        }
    }
}

@compute @workgroup_size(256, 1, 1)
fn filter_i32_gt_scatter(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < filt_params.n && filt_flags[i] != 0) {
        let dst = u32(filt_pos[i]);
        filt_out[dst] = filt_in[i];
    }
}

// ---------------------------------------------------------------------------
// 4. Batched 2D Stencil (u8 packed in u32) + Privatized Histogram (u32)
// ---------------------------------------------------------------------------

struct StencilU8Params {
    frames: u32,
    h: u32,
    w: u32,
    divisor: i32,
    edge_policy: u32,
    w0: i32, w1: i32, w2: i32,
    w3: i32, w4: i32, w5: i32,
    w6: i32, w7: i32, w8: i32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> st8_in: array<u32>;
@group(0) @binding(1) var<storage, read_write> st8_out: array<atomic<u32>>;
@group(0) @binding(2) var<uniform> st8_p: StencilU8Params;

fn load_packed_u8(byte_idx: u32) -> i32 {
    let word = st8_in[byte_idx >> 2u];
    let shift = (byte_idx & 3u) * 8u;
    return i32((word >> shift) & 255u);
}

fn edge_coord(c: i32, lim: i32, policy: u32) -> i32 {
    if (c >= 0 && c < lim) {
        return c;
    }
    if (policy == 0u) {
        return clamp(c, 0, lim - 1);
    } else if (policy == 1u) {
        return -1;
    } else if (policy == 2u) {
        let r = c % lim;
        return select(r + lim, r, r >= 0);
    } else {
        var rc = c;
        if (rc < 0) { rc = -rc; }
        if (rc >= lim) { rc = 2 * lim - 2 - rc; }
        return clamp(rc, 0, lim - 1);
    }
}

@compute @workgroup_size(16, 16, 1)
fn stencil2d_u8_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    let frame = gid.z;
    if (frame >= st8_p.frames || y >= st8_p.h || x >= st8_p.w) {
        return;
    }
    let h_i = i32(st8_p.h);
    let w_i = i32(st8_p.w);
    let frame_base = frame * st8_p.h * st8_p.w;
    var weights: array<i32, 9> = array<i32, 9>(
        st8_p.w0, st8_p.w1, st8_p.w2,
        st8_p.w3, st8_p.w4, st8_p.w5,
        st8_p.w6, st8_p.w7, st8_p.w8,
    );
    var acc: i32 = 0;
    var wi: u32 = 0u;
    for (var dy: i32 = -1; dy <= 1; dy = dy + 1) {
        let ny = edge_coord(i32(y) + dy, h_i, st8_p.edge_policy);
        for (var dx: i32 = -1; dx <= 1; dx = dx + 1) {
            let nx = edge_coord(i32(x) + dx, w_i, st8_p.edge_policy);
            var px: i32 = 0;
            if (ny >= 0 && nx >= 0) {
                let idx = frame_base + u32(ny) * st8_p.w + u32(nx);
                px = load_packed_u8(idx);
            }
            acc = acc + weights[wi] * px;
            wi = wi + 1u;
        }
    }
    let d = select(1, st8_p.divisor, st8_p.divisor != 0);
    let q = clamp(acc / d, 0, 255);
    let out_byte = frame_base + y * st8_p.w + x;
    let word_idx = out_byte >> 2u;
    let shift = (out_byte & 3u) * 8u;
    atomicOr(&st8_out[word_idx], u32(q) << shift);
}

struct HistParams {
    frames: u32,
    frame_pixels: u32,
    num_bins: u32,
    tiles_per_frame: u32,
};

@group(0) @binding(0) var<storage, read> hist_pixels: array<u32>;
@group(0) @binding(1) var<storage, read_write> hist_out: array<atomic<u32>>;
@group(0) @binding(2) var<uniform> hist_p: HistParams;

var<workgroup> local_bins: array<atomic<u32>, 256>;

fn load_hist_u8(byte_idx: u32) -> u32 {
    let word = hist_pixels[byte_idx >> 2u];
    return (word >> ((byte_idx & 3u) * 8u)) & 255u;
}

@compute @workgroup_size(256, 1, 1)
fn hist_u8_many_small_frames(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let frame = wid.x;
    let tid = lid.x;
    atomicStore(&local_bins[tid], 0u);
    workgroupBarrier();

    if (frame < hist_p.frames) {
        let base = frame * hist_p.frame_pixels;
        var i = tid;
        loop {
            if (i >= hist_p.frame_pixels) { break; }
            let b = load_hist_u8(base + i);
            if (b < hist_p.num_bins) {
                atomicAdd(&local_bins[b], 1u);
            }
            i = i + 256u;
        }
    }
    workgroupBarrier();

    if (frame < hist_p.frames && tid < hist_p.num_bins) {
        atomicStore(&hist_out[frame * hist_p.num_bins + tid], atomicLoad(&local_bins[tid]));
    }
}

@compute @workgroup_size(256, 1, 1)
fn hist_u8_few_large_frames(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let tiles = max(hist_p.tiles_per_frame, 1u);
    let frame = wid.x / tiles;
    let tile = wid.x % tiles;
    let tid = lid.x;
    atomicStore(&local_bins[tid], 0u);
    workgroupBarrier();

    if (frame < hist_p.frames) {
        let base = frame * hist_p.frame_pixels;
        let stride = tiles * 256u;
        var i = tile * 256u + tid;
        loop {
            if (i >= hist_p.frame_pixels) { break; }
            let b = load_hist_u8(base + i);
            if (b < hist_p.num_bins) {
                atomicAdd(&local_bins[b], 1u);
            }
            i = i + stride;
        }
    }
    workgroupBarrier();

    if (frame < hist_p.frames && tid < hist_p.num_bins) {
        let c = atomicLoad(&local_bins[tid]);
        if (c != 0u) {
            atomicAdd(&hist_out[frame * hist_p.num_bins + tid], c);
        }
    }
}

// ---------------------------------------------------------------------------
// 5. Batched 2D Float32 Stencil
// ---------------------------------------------------------------------------

struct StencilF32Params {
    frames: u32,
    h: u32,
    w: u32,
    edge_policy: u32,
    w0: f32, w1: f32, w2: f32, w3: f32,
    w4: f32, w5: f32, w6: f32, w7: f32,
    w8: f32,
    _pad0: u32, _pad1: u32, _pad2: u32,
};

@group(0) @binding(0) var<storage, read> stf_in: array<f32>;
@group(0) @binding(1) var<storage, read_write> stf_out: array<f32>;
@group(0) @binding(2) var<uniform> stf_p: StencilF32Params;

@compute @workgroup_size(16, 16, 1)
fn stencil2d_f32_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;
    let frame = gid.z;
    if (frame >= stf_p.frames || y >= stf_p.h || x >= stf_p.w) {
        return;
    }
    let h_i = i32(stf_p.h);
    let w_i = i32(stf_p.w);
    let frame_base = frame * stf_p.h * stf_p.w;
    var weights: array<f32, 9> = array<f32, 9>(
        stf_p.w0, stf_p.w1, stf_p.w2,
        stf_p.w3, stf_p.w4, stf_p.w5,
        stf_p.w6, stf_p.w7, stf_p.w8,
    );
    var acc: f32 = 0.0;
    var wi: u32 = 0u;
    for (var dy: i32 = -1; dy <= 1; dy = dy + 1) {
        let ny = edge_coord(i32(y) + dy, h_i, stf_p.edge_policy);
        for (var dx: i32 = -1; dx <= 1; dx = dx + 1) {
            let nx = edge_coord(i32(x) + dx, w_i, stf_p.edge_policy);
            var px: f32 = 0.0;
            if (ny >= 0 && nx >= 0) {
                px = stf_in[frame_base + u32(ny) * stf_p.w + u32(nx)];
            }
            acc = acc + weights[wi] * px;
            wi = wi + 1u;
        }
    }
    stf_out[frame_base + y * stf_p.w + x] = acc;
}

// ---------------------------------------------------------------------------
// 6. Sparse Matrix-Vector Multiply (CSR f32, exact-tree & scalar)
// ---------------------------------------------------------------------------

struct SpmvParams {
    rows: u32,
    exact: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> spmv_vals: array<f32>;
@group(0) @binding(1) var<storage, read> spmv_cols: array<i32>;
@group(0) @binding(2) var<storage, read> spmv_row_ptr: array<i32>;
@group(0) @binding(3) var<storage, read> spmv_x: array<f32>;
@group(0) @binding(4) var<storage, read_write> spmv_y: array<f32>;
@group(0) @binding(5) var<uniform> spmv_p: SpmvParams;

@compute @workgroup_size(256, 1, 1)
fn spmv_csr_f32_scalar(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row = gid.x;
    if (row >= spmv_p.rows) {
        return;
    }
    let start = u32(spmv_row_ptr[row]);
    let end = u32(spmv_row_ptr[row + 1u]);
    if (spmv_p.exact != 0u) {
        var vals: array<f32, 32>;
        var sizes: array<u32, 32>;
        var sp: u32 = 0u;
        for (var k = start; k < end; k = k + 1u) {
            var cur = spmv_vals[k] * spmv_x[u32(spmv_cols[k])];
            var sz: u32 = 1u;
            loop {
                if (sp == 0u || sizes[sp - 1u] != sz) { break; }
                cur = vals[sp - 1u] + cur;
                sz = sz << 1u;
                sp = sp - 1u;
            }
            vals[sp] = cur;
            sizes[sp] = sz;
            sp = sp + 1u;
        }
        var acc: f32 = 0.0;
        if (sp > 0u) {
            acc = vals[sp - 1u];
            if (sp >= 2u) {
                var j: i32 = i32(sp) - 2;
                loop {
                    if (j < 0) { break; }
                    acc = vals[u32(j)] + acc;
                    j = j - 1;
                }
            }
        }
        spmv_y[row] = acc;
    } else {
        var acc: f32 = 0.0;
        for (var k = start; k < end; k = k + 1u) {
            acc = acc + spmv_vals[k] * spmv_x[u32(spmv_cols[k])];
        }
        spmv_y[row] = acc;
    }
}

// ---------------------------------------------------------------------------
// 7. Nested-Parallel Row Reduction (f32: outer rows vs inner workgroup)
// ---------------------------------------------------------------------------

struct RowRedParams {
    rows: u32,
    cols: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read> rr_in: array<f32>;
@group(0) @binding(1) var<storage, read_write> rr_out: array<f32>;
@group(0) @binding(2) var<uniform> rr_p: RowRedParams;

var<workgroup> rr_sh: array<f32, 256>;

@compute @workgroup_size(256, 1, 1)
fn row_reduce_f32_outer(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row = gid.x;
    if (row >= rr_p.rows) {
        return;
    }
    let base = row * rr_p.cols;
    var acc: f32 = 0.0;
    for (var j: u32 = 0u; j < rr_p.cols; j = j + 1u) {
        acc = acc + rr_in[base + j];
    }
    rr_out[row] = acc;
}

@compute @workgroup_size(256, 1, 1)
fn row_reduce_f32_inner(
    @builtin(local_invocation_id) lid: vec3<u32>,
    @builtin(workgroup_id) wid: vec3<u32>,
) {
    let row = wid.x;
    let tid = lid.x;
    var acc: f32 = 0.0;
    if (row < rr_p.rows) {
        let base = row * rr_p.cols;
        var j = tid;
        loop {
            if (j >= rr_p.cols) { break; }
            acc = acc + rr_in[base + j];
            j = j + 256u;
        }
    }
    rr_sh[tid] = acc;
    workgroupBarrier();

    var stride: u32 = 128u;
    loop {
        if (stride == 0u) { break; }
        if (tid < stride) {
            rr_sh[tid] = rr_sh[tid] + rr_sh[tid + stride];
        }
        workgroupBarrier();
        stride = stride >> 1u;
    }
    if (row < rr_p.rows && tid == 0u) {
        rr_out[row] = rr_sh[0];
    }
}
