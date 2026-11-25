// Cinch WebGPU Host Bridge for WebAssembly (ES Module)
// Provides the `cinch_webgpu` Wasm import module backed by `navigator.gpu`.
// Supports synchronous GPU-resident uploads/dispatches and JSPI readbacks
// (`WebAssembly.Suspending`) as well as direct async host invocation.

export const CINCH_WGSL_SOURCE = `__CINCH_WGSL_PLACEHOLDER__`;

const BUF_STORAGE_RW =
  typeof GPUBufferUsage !== "undefined"
    ? GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST
    : 0x80 | 0x04 | 0x08;
const BUF_UNIFORM =
  typeof GPUBufferUsage !== "undefined"
    ? GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST
    : 0x40 | 0x08;
const BUF_MAP_READ =
  typeof GPUBufferUsage !== "undefined"
    ? GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
    : 0x01 | 0x08;
const MAP_READ_MODE =
  typeof GPUMapMode !== "undefined" ? GPUMapMode.READ : 0x0001;

function align4(n) {
  const m = Math.max(4, n | 0);
  return (m + 3) & ~3;
}

function f32ExactTreeSum(arr) {
  const vals = [];
  const sizes = [];
  for (let i = 0; i < arr.length; i++) {
    let cur = Math.fround(arr[i]);
    let sz = 1;
    while (sizes.length > 0 && sizes[sizes.length - 1] === sz) {
      cur = Math.fround(vals.pop() + cur);
      sizes.pop();
      sz <<= 1;
    }
    vals.push(cur);
    sizes.push(sz);
  }
  if (vals.length === 0) return 0.0;
  let acc = vals[vals.length - 1];
  for (let j = vals.length - 2; j >= 0; j--) {
    acc = Math.fround(vals[j] + acc);
  }
  return acc;
}

export class CinchWebGpuHost {
  constructor(device, adapterInfo, wgslSource = CINCH_WGSL_SOURCE) {
    this.device = device;
    this.adapterInfo = adapterInfo ?? { description: "WebGPU" };
    this.wgslSource = wgslSource;
    this.shaderModule = device.createShaderModule({ code: wgslSource });
    this.pipelines = new Map();
    this.buffers = new Map(); // handle -> { gpuBuf, bytes, size }
    this.nextHandle = 1;
    this.ctxOpen = false;
    this.telemetry = {
      kernelLaunches: 0,
      hostToDeviceBytes: 0,
      deviceToHostBytes: 0,
    };
  }

  static async create(options = {}) {
    const gpu = options.gpu ?? globalThis.navigator?.gpu;
    if (!gpu) {
      throw new Error("WebGPU is unavailable: navigator.gpu is not defined");
    }
    const adapter = await gpu.requestAdapter({
      powerPreference: options.powerPreference ?? "high-performance",
    });
    if (!adapter) {
      throw new Error("WebGPU adapter request returned null");
    }
    const device = await adapter.requestDevice();
    return new CinchWebGpuHost(device, adapter.info, options.wgslSource);
  }

  pipeline(entryPoint) {
    let p = this.pipelines.get(entryPoint);
    if (!p) {
      p = this.device.createComputePipeline({
        layout: "auto",
        compute: { module: this.shaderModule, entryPoint },
      });
      this.pipelines.set(entryPoint, p);
    }
    return p;
  }

  allocBuffer(bytes) {
    const size = align4(bytes);
    const gpuBuf = this.device.createBuffer({ size, usage: BUF_STORAGE_RW });
    const id = this.nextHandle++;
    this.buffers.set(id, { gpuBuf, bytes: bytes | 0, size });
    return id;
  }

  freeBuffer(id) {
    const entry = this.buffers.get(id);
    if (entry) {
      entry.gpuBuf.destroy();
      this.buffers.delete(id);
    }
  }

  uploadBytes(srcUint8) {
    const id = this.allocBuffer(srcUint8.byteLength);
    const entry = this.buffers.get(id);
    if (srcUint8.byteLength > 0) {
      const padded = new Uint8Array(entry.size);
      padded.set(srcUint8);
      this.device.queue.writeBuffer(entry.gpuBuf, 0, padded);
      this.telemetry.hostToDeviceBytes += srcUint8.byteLength;
    }
    return id;
  }

  async downloadBytes(id, byteLen) {
    const entry = this.buffers.get(id);
    if (!entry) throw new Error(`Invalid WebGPU buffer handle ${id}`);
    const len = byteLen ?? entry.bytes;
    if (len === 0) return new Uint8Array(0);
    const size = align4(len);
    const staging = this.device.createBuffer({ size, usage: BUF_MAP_READ });
    const enc = this.device.createCommandEncoder();
    enc.copyBufferToBuffer(entry.gpuBuf, 0, staging, 0, size);
    this.device.queue.submit([enc.finish()]);
    await staging.mapAsync(MAP_READ_MODE);
    const view = new Uint8Array(staging.getMappedRange(), 0, len);
    const copy = new Uint8Array(len);
    copy.set(view);
    staging.unmap();
    staging.destroy();
    this.telemetry.deviceToHostBytes += len;
    return copy;
  }

  makeUniform(typedArray) {
    const bytes = align4(typedArray.byteLength);
    const padded = new Uint8Array(Math.max(16, (bytes + 15) & ~15));
    padded.set(
      new Uint8Array(
        typedArray.buffer,
        typedArray.byteOffset,
        typedArray.byteLength,
      ),
    );
    const buf = this.device.createBuffer({
      size: padded.byteLength,
      usage: BUF_UNIFORM,
    });
    this.device.queue.writeBuffer(buf, 0, padded);
    return buf;
  }

  dispatch(entryPoint, bindings, workgroups, clearBufs = []) {
    const pipe = this.pipeline(entryPoint);
    const bg = this.device.createBindGroup({
      layout: pipe.getBindGroupLayout(0),
      entries: bindings.map((buffer, binding) => ({
        binding,
        resource: { buffer },
      })),
    });
    const enc = this.device.createCommandEncoder();
    for (const cb of clearBufs) {
      enc.clearBuffer(cb);
    }
    const pass = enc.beginComputePass();
    pass.setPipeline(pipe);
    pass.setBindGroup(0, bg);
    pass.dispatchWorkgroups(workgroups[0], workgroups[1] ?? 1, workgroups[2] ?? 1);
    pass.end();
    this.device.queue.submit([enc.finish()]);
    this.telemetry.kernelLaunches += 1;
  }

  async dotF32(aId, bId, n, exact) {
    if (n === 0) return 0.0;
    const a = this.buffers.get(aId)?.gpuBuf;
    const b = this.buffers.get(bId)?.gpuBuf;
    const groups = exact
      ? Math.ceil(n / 1024)
      : Math.min(1024, Math.max(1, Math.ceil(n / 256)));
    const partialId = this.allocBuffer(groups * 4);
    const ubuf = this.makeUniform(new Uint32Array([n, 0, 0, 0]));
    this.dispatch(
      exact ? "dot_f32_exact_block" : "dot_f32_relaxed",
      [a, b, this.buffers.get(partialId).gpuBuf, ubuf],
      [groups, 1, 1],
    );
    const raw = await this.downloadBytes(partialId, groups * 4);
    this.freeBuffer(partialId);
    ubuf.destroy();
    const partials = new Float32Array(raw.buffer, raw.byteOffset, groups);
    if (exact) return f32ExactTreeSum(partials);
    let acc = 0.0;
    for (let i = 0; i < partials.length; i++) acc = Math.fround(acc + partials[i]);
    return acc;
  }

  async scanI32(inId, n, inclusive) {
    const outId = this.allocBuffer(n * 4);
    if (n === 0) return outId;
    const nblocks = Math.ceil(n / 256);
    const bsId = this.allocBuffer(nblocks * 4);
    const ubuf = this.makeUniform(new Uint32Array([n, inclusive ? 1 : 0, 0, 0]));
    this.dispatch(
      "scan_i32_block",
      [
        this.buffers.get(inId).gpuBuf,
        this.buffers.get(outId).gpuBuf,
        this.buffers.get(bsId).gpuBuf,
        ubuf,
      ],
      [nblocks, 1, 1],
    );
    if (nblocks > 1) {
      const raw = await this.downloadBytes(bsId, nblocks * 4);
      const sums = new Int32Array(raw.buffer, raw.byteOffset, nblocks);
      let acc = 0;
      for (let i = 0; i < nblocks; i++) {
        const x = sums[i];
        sums[i] = acc;
        acc = (acc + x) | 0;
      }
      const offsId = this.uploadBytes(
        new Uint8Array(sums.buffer, sums.byteOffset, sums.byteLength),
      );
      this.dispatch(
        "scan_i32_add_offsets",
        [
          this.buffers.get(inId).gpuBuf,
          this.buffers.get(outId).gpuBuf,
          this.buffers.get(offsId).gpuBuf,
          ubuf,
        ],
        [nblocks, 1, 1],
      );
      this.freeBuffer(offsId);
    }
    this.freeBuffer(bsId);
    ubuf.destroy();
    return outId;
  }

  async filterI32Gt(inId, n, threshold) {
    if (n === 0) return { bufId: this.allocBuffer(0), len: 0 };
    const flagsId = this.allocBuffer(n * 4);
    const dummyPosId = this.allocBuffer(4);
    const dummyOutId = this.allocBuffer(4);
    const ubuf = this.makeUniform(new Int32Array([n, threshold, 0, 0]));
    const nblocks = Math.ceil(n / 256);
    this.dispatch(
      "filter_i32_gt_flags",
      [
        this.buffers.get(inId).gpuBuf,
        this.buffers.get(flagsId).gpuBuf,
        this.buffers.get(dummyPosId).gpuBuf,
        this.buffers.get(dummyOutId).gpuBuf,
        ubuf,
      ],
      [nblocks, 1, 1],
    );
    this.freeBuffer(dummyPosId);
    this.freeBuffer(dummyOutId);
    const posId = await this.scanI32(flagsId, n, false);
    const fRaw = await this.downloadBytes(flagsId, n * 4);
    const pRaw = await this.downloadBytes(posId, n * 4);
    const fArr = new Int32Array(fRaw.buffer, fRaw.byteOffset, n);
    const pArr = new Int32Array(pRaw.buffer, pRaw.byteOffset, n);
    const k = Math.max(0, (pArr[n - 1] + fArr[n - 1]) | 0);
    const outId = this.allocBuffer(k * 4);
    if (k > 0) {
      this.dispatch(
        "filter_i32_gt_scatter",
        [
          this.buffers.get(inId).gpuBuf,
          this.buffers.get(flagsId).gpuBuf,
          this.buffers.get(posId).gpuBuf,
          this.buffers.get(outId).gpuBuf,
          ubuf,
        ],
        [nblocks, 1, 1],
      );
    }
    this.freeBuffer(flagsId);
    this.freeBuffer(posId);
    ubuf.destroy();
    return { bufId: outId, len: k };
  }

  stencilHistogramU8(inId, f, h, w, ws, divisor, edgePolicy, nsteps, bins) {
    const outHistId = this.allocBuffer(f * bins * 4);
    if (f === 0 || h === 0 || w === 0 || bins === 0) return outHistId;
    const total = f * h * w;
    const pingId = nsteps >= 1 ? this.allocBuffer(total) : 0;
    const pongId = nsteps >= 2 ? this.allocBuffer(total) : 0;
    let curId = inId;
    if (nsteps > 0) {
      const u = new Int32Array([
        f, h, w, divisor, edgePolicy,
        ws[0], ws[1], ws[2], ws[3], ws[4], ws[5], ws[6], ws[7], ws[8], 0, 0,
      ]);
      const ubuf = this.makeUniform(u);
      const gx = Math.ceil(w / 16);
      const gy = Math.ceil(h / 16);
      for (let step = 0; step < nsteps; step++) {
        const dstId = step % 2 === 0 ? pingId : pongId;
        const dstBuf = this.buffers.get(dstId).gpuBuf;
        this.dispatch(
          "stencil2d_u8_step",
          [this.buffers.get(curId).gpuBuf, dstBuf, ubuf],
          [gx, gy, f],
          [dstBuf],
        );
        curId = dstId;
      }
      ubuf.destroy();
    }
    const framePixels = h * w;
    const manySmall = f >= 16 || framePixels <= 16384;
    const tilesPerFrame = manySmall
      ? 1
      : Math.min(256, Math.max(1, Math.floor(framePixels / (256 * 32))));
    const hub = this.makeUniform(
      new Uint32Array([f, framePixels, bins, tilesPerFrame]),
    );
    const outBuf = this.buffers.get(outHistId).gpuBuf;
    this.dispatch(
      manySmall ? "hist_u8_many_small_frames" : "hist_u8_few_large_frames",
      [this.buffers.get(curId).gpuBuf, outBuf, hub],
      [f * tilesPerFrame, 1, 1],
      manySmall ? [] : [outBuf],
    );
    hub.destroy();
    if (pingId) this.freeBuffer(pingId);
    if (pongId) this.freeBuffer(pongId);
    return outHistId;
  }

  stencil2dF32(inId, f, h, w, ws, edgePolicy, nsteps) {
    const total = f * h * w;
    const pingId = this.allocBuffer(total * 4);
    const pongId = nsteps >= 2 ? this.allocBuffer(total * 4) : 0;
    const raw = new ArrayBuffer(64);
    const u32 = new Uint32Array(raw);
    const f32 = new Float32Array(raw);
    u32[0] = f; u32[1] = h; u32[2] = w; u32[3] = edgePolicy;
    for (let i = 0; i < 9; i++) f32[4 + i] = ws[i];
    const ubuf = this.makeUniform(new Uint8Array(raw));
    const gx = Math.ceil(w / 16);
    const gy = Math.ceil(h / 16);
    let curId = inId;
    let finalPing = true;
    for (let step = 0; step < nsteps; step++) {
      const dstId = step % 2 === 0 ? pingId : pongId;
      finalPing = step % 2 === 0;
      this.dispatch(
        "stencil2d_f32_step",
        [this.buffers.get(curId).gpuBuf, this.buffers.get(dstId).gpuBuf, ubuf],
        [gx, gy, f],
      );
      curId = dstId;
    }
    ubuf.destroy();
    if (finalPing) {
      if (pongId) this.freeBuffer(pongId);
      return pingId;
    }
    this.freeBuffer(pingId);
    return pongId;
  }

  spmvCsrF32(valsId, colsId, rowPtrId, xId, rows, exact) {
    const outId = this.allocBuffer(rows * 4);
    if (rows === 0) return outId;
    const ubuf = this.makeUniform(new Uint32Array([rows, exact ? 1 : 0, 0, 0]));
    this.dispatch(
      "spmv_csr_f32_scalar",
      [
        this.buffers.get(valsId).gpuBuf,
        this.buffers.get(colsId).gpuBuf,
        this.buffers.get(rowPtrId).gpuBuf,
        this.buffers.get(xId).gpuBuf,
        this.buffers.get(outId).gpuBuf,
        ubuf,
      ],
      [Math.ceil(rows / 256), 1, 1],
    );
    ubuf.destroy();
    return outId;
  }

  rowReduceF32(inId, rows, cols) {
    const outId = this.allocBuffer(rows * 4);
    if (rows === 0) return outId;
    const outer = rows >= 128 && cols < 2048;
    const ubuf = this.makeUniform(new Uint32Array([rows, cols, 0, 0]));
    this.dispatch(
      outer ? "row_reduce_f32_outer" : "row_reduce_f32_inner",
      [this.buffers.get(inId).gpuBuf, this.buffers.get(outId).gpuBuf, ubuf],
      [outer ? Math.ceil(rows / 256) : rows, 1, 1],
    );
    ubuf.destroy();
    return outId;
  }
}

export function createCinchWebGpuImports(host, getMemory, options = {}) {
  const wrapAsync = (fn) =>
    options.jspi !== false && typeof WebAssembly?.Suspending === "function"
      ? new WebAssembly.Suspending(fn)
      : fn;

  return {
    cinch_webgpu: {
      cinch_webgpu_available: () => (host ? 1 : 0),
      cinch_webgpu_open: () => {
        if (!host) return -1;
        host.ctxOpen = true;
        return 1;
      },
      cinch_webgpu_close: () => {
        if (host) host.ctxOpen = false;
      },
      cinch_webgpu_alloc: (_ctx, bytes) => host.allocBuffer(bytes >>> 0),
      cinch_webgpu_upload: (_ctx, ptr, bytes) => {
        const mem = new Uint8Array(getMemory().buffer, ptr >>> 0, bytes >>> 0);
        return host.uploadBytes(mem);
      },
      cinch_webgpu_download: wrapAsync(async (_ctx, bufId, dstPtr, bytes) => {
        const data = await host.downloadBytes(bufId, bytes >>> 0);
        new Uint8Array(getMemory().buffer, dstPtr >>> 0, data.byteLength).set(data);
        return 0;
      }),
      cinch_webgpu_free: (_ctx, bufId) => host.freeBuffer(bufId),
      cinch_webgpu_dot_f32: wrapAsync(async (_ctx, a, b, n, exact, outPtr) => {
        const val = await host.dotF32(a, b, n >>> 0, exact !== 0);
        new DataView(getMemory().buffer).setFloat32(outPtr >>> 0, val, true);
        return 0;
      }),
      cinch_webgpu_scan_i32: wrapAsync(async (_ctx, inId, n, inclusive) =>
        host.scanI32(inId, n >>> 0, inclusive !== 0),
      ),
      cinch_webgpu_filter_i32_gt: wrapAsync(
        async (_ctx, inId, n, threshold, outLenPtr) => {
          const { bufId, len } = await host.filterI32Gt(
            inId,
            n >>> 0,
            threshold | 0,
          );
          new DataView(getMemory().buffer).setUint32(outLenPtr >>> 0, len >>> 0, true);
          return bufId;
        },
      ),
      cinch_webgpu_stencil_hist_u8: (
        _ctx,
        inId,
        f,
        h,
        w,
        wsPtr,
        divisor,
        edge,
        nsteps,
        bins,
      ) => {
        const ws = new Int32Array(getMemory().buffer, wsPtr >>> 0, 9);
        return host.stencilHistogramU8(
          inId,
          f >>> 0,
          h >>> 0,
          w >>> 0,
          ws,
          divisor | 0,
          edge >>> 0,
          nsteps >>> 0,
          bins >>> 0,
        );
      },
      cinch_webgpu_stencil2d_f32: (
        _ctx,
        inId,
        f,
        h,
        w,
        wsPtr,
        edge,
        nsteps,
      ) => {
        const ws = new Float32Array(getMemory().buffer, wsPtr >>> 0, 9);
        return host.stencil2dF32(
          inId,
          f >>> 0,
          h >>> 0,
          w >>> 0,
          ws,
          edge >>> 0,
          nsteps >>> 0,
        );
      },
      cinch_webgpu_spmv_csr_f32: (_ctx, vals, cols, rowPtr, x, r, _nz, exact) =>
        host.spmvCsrF32(vals, cols, rowPtr, x, r >>> 0, exact !== 0),
      cinch_webgpu_row_reduce_f32: (_ctx, inId, r, c) =>
        host.rowReduceF32(inId, r >>> 0, c >>> 0),
    },
  };
}

export async function loadCinchWebGpuWasm(
  wasmSource,
  extraImports = {},
  options = {},
) {
  const host = await CinchWebGpuHost.create(options);
  let instanceMemory = null;
  const webgpuImports = createCinchWebGpuImports(
    host,
    () => instanceMemory,
    options,
  );
  const imports = { ...extraImports, ...webgpuImports };
  const bytes =
    wasmSource instanceof ArrayBuffer || ArrayBuffer.isView(wasmSource)
      ? wasmSource
      : await (await fetch(wasmSource)).arrayBuffer();
  const { instance, module } = await WebAssembly.instantiate(bytes, imports);
  instanceMemory = instance.exports.memory;
  return { instance, module, host };
}
