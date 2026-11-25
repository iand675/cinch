"""
Reusable Python multi-backend host API for Cinch's data-parallel arrays
(equivalent to Futhark's `python` and `pyopencl` backends, plus Metal, CUDA,
HIP, ISPC/SIMD, multicore CPU, and sequential CPU).

Usage:
    from cinch_device import CinchDevice

    print(CinchDevice.available_backends())
    with CinchDevice("opencl") as dev:
        d_in = dev.upload(frame_bytes)
        d_hist = dev.stencil_histogram_u8(d_in, frames, h, w, weights, 16, steps=3)
        counts = dev.download_u32(d_hist)
        print(dev.telemetry())
"""

from __future__ import annotations

import array
import ctypes
import json
import platform
from pathlib import Path
from typing import Sequence

BACKEND_CODES = {
    "cpu": 0,
    "multicore": 0,
    "metal": 1,
    "metal-required": 2,
    "opencl-required": 3,
    "opencl": 3,
    "pyopencl": 3,
    "cuda-required": 4,
    "cuda": 4,
    "hip-required": 5,
    "hip": 5,
    "seq": 6,
    "c": 6,
    "python": 6,
    "ispc": 7,
    "simd": 7,
    "auto": 8,
    "opencl-fallback": 9,
    "cuda-fallback": 10,
    "hip-fallback": 11,
}

EDGE_CODES = {"clamp": 0, "zero": 1, "mirror": 2, "wrap": 3}
FLOAT_MODES = {"exact": 10, "exact-tree": 10, "relaxed": 11}


def _default_library_path() -> Path:
    root = Path(__file__).resolve().parents[2]
    ext = "dylib" if platform.system() == "Darwin" else "so"
    for profile in ("release", "debug"):
        p = root / "seed" / "runtime" / "target" / profile / f"libcinchruntime.{ext}"
        if p.exists():
            return p
    return root / "seed" / "runtime" / "target" / "release" / f"libcinchruntime.{ext}"


_CACHED_LIB = None


def load_runtime(lib_path: str | Path | None = None) -> ctypes.CDLL:
    global _CACHED_LIB
    if lib_path is None and _CACHED_LIB is not None:
        return _CACHED_LIB
    path = Path(lib_path) if lib_path else _default_library_path()
    if not path.exists():
        raise FileNotFoundError(
            f"Missing Cinch runtime library at {path}. "
            "Build with: cargo build --manifest-path seed/runtime/Cargo.toml --release"
        )
    lib = ctypes.CDLL(str(path))
    lib.cinch_device_open.argtypes = [ctypes.c_int64]
    lib.cinch_device_open.restype = ctypes.c_int64
    lib.cinch_device_close.argtypes = [ctypes.c_int64]
    lib.cinch_device_close.restype = ctypes.c_int64
    lib.cinch_device_reset_telemetry.argtypes = [ctypes.c_int64]
    lib.cinch_device_reset_telemetry.restype = ctypes.c_int64
    lib.cinch_device_upload_raw.argtypes = [ctypes.c_int64, ctypes.c_void_p, ctypes.c_size_t]
    lib.cinch_device_upload_raw.restype = ctypes.c_int64
    lib.cinch_device_download_raw.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_void_p, ctypes.c_size_t]
    lib.cinch_device_download_raw.restype = ctypes.c_int64
    lib.cinch_device_buffer_free.argtypes = [ctypes.c_int64]
    lib.cinch_device_buffer_free.restype = ctypes.c_int64
    lib.cinch_device_buffer_bytes.argtypes = [ctypes.c_int64]
    lib.cinch_device_buffer_bytes.restype = ctypes.c_int64
    lib.cinch_device_report_cstr.argtypes = [ctypes.c_int64, ctypes.c_char_p, ctypes.c_size_t]
    lib.cinch_device_report_cstr.restype = ctypes.c_int64
    lib.cinch_device_available_backends_cstr.argtypes = [ctypes.c_char_p, ctypes.c_size_t]
    lib.cinch_device_available_backends_cstr.restype = ctypes.c_int64
    lib.cinch_device_backend_source_cstr.argtypes = [ctypes.c_int64, ctypes.c_char_p, ctypes.c_size_t]
    lib.cinch_device_backend_source_cstr.restype = ctypes.c_int64
    lib.cinch_device_dot_f32.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.POINTER(ctypes.c_float),
    ]
    lib.cinch_device_dot_f32.restype = ctypes.c_int64
    lib.cinch_device_scan_i32.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    lib.cinch_device_scan_i32.restype = ctypes.c_int64
    lib.cinch_device_filter_i32_gt.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.POINTER(ctypes.c_int64),
    ]
    lib.cinch_device_filter_i32_gt.restype = ctypes.c_int64
    lib.cinch_device_stencil_histogram_u8.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
        ctypes.POINTER(ctypes.c_int32), ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
    ]
    lib.cinch_device_stencil_histogram_u8.restype = ctypes.c_int64
    lib.cinch_device_stencil2d_f32.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
        ctypes.POINTER(ctypes.c_float), ctypes.c_int64, ctypes.c_int64,
    ]
    lib.cinch_device_stencil2d_f32.restype = ctypes.c_int64
    lib.cinch_device_spmv_csr_f32.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
    ]
    lib.cinch_device_spmv_csr_f32.restype = ctypes.c_int64
    lib.cinch_device_row_reduce_f32.argtypes = [ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64]
    lib.cinch_device_row_reduce_f32.restype = ctypes.c_int64
    if lib_path is None:
        _CACHED_LIB = lib
    return lib


class DeviceError(RuntimeError):
    def __init__(self, code: int, message: str = ""):
        super().__init__(f"Cinch device error ({code}): {message}" if message else f"Cinch device error ({code})")
        self.code = code


class ResidentBuffer:
    def __init__(self, device: CinchDevice, handle: int, nbytes: int):
        self._device = device
        self.handle = handle
        self.nbytes = nbytes
        self._freed = False

    def free(self) -> None:
        if not self._freed and self.handle > 0:
            self._device._lib.cinch_device_buffer_free(self.handle)
            self._freed = True

    def __enter__(self) -> ResidentBuffer:
        return self

    def __exit__(self, *exc) -> None:
        self.free()

    def __del__(self) -> None:
        try:
            self.free()
        except Exception:
            pass


class CinchDevice:
    """Multi-backend device handle supporting Metal, OpenCL, CUDA, HIP, ISPC, Multicore CPU, and Seq CPU."""

    def __init__(self, backend: str = "auto", lib_path: str | Path | None = None):
        if backend not in BACKEND_CODES:
            raise ValueError(f"Unknown backend '{backend}'; choose one of {sorted(BACKEND_CODES)}")
        self._lib = load_runtime(lib_path)
        self.requested_backend = backend
        self.handle = self._lib.cinch_device_open(BACKEND_CODES[backend])
        if self.handle <= 0:
            raise DeviceError(self.handle, f"failed to open backend '{backend}'")
        self._closed = False

    @staticmethod
    def available_backends(lib_path: str | Path | None = None) -> dict[str, bool]:
        lib = load_runtime(lib_path)
        buf = ctypes.create_string_buffer(1024)
        n = lib.cinch_device_available_backends_cstr(buf, len(buf))
        if n <= 0:
            raise DeviceError(n, "available_backends query failed")
        return json.loads(buf.value.decode("utf-8"))

    @staticmethod
    def backend_source(backend: str, lib_path: str | Path | None = None) -> str:
        lib = load_runtime(lib_path)
        code = BACKEND_CODES[backend]
        needed = lib.cinch_device_backend_source_cstr(code, None, 0)
        if needed <= 0:
            raise DeviceError(needed, f"no kernel source for backend '{backend}'")
        buf = ctypes.create_string_buffer(needed + 1)
        lib.cinch_device_backend_source_cstr(code, buf, len(buf))
        return buf.value.decode("utf-8")

    def close(self) -> None:
        if not self._closed and self.handle > 0:
            self._lib.cinch_device_close(self.handle)
            self._closed = True

    def __enter__(self) -> CinchDevice:
        return self

    def __exit__(self, *exc) -> None:
        self.close()

    def reset_telemetry(self) -> None:
        rc = self._lib.cinch_device_reset_telemetry(self.handle)
        if rc != 0:
            raise DeviceError(rc)

    def telemetry(self) -> dict:
        buf = ctypes.create_string_buffer(8192)
        n = self._lib.cinch_device_report_cstr(self.handle, buf, len(buf))
        if n <= 0:
            raise DeviceError(n)
        return json.loads(buf.value.decode("utf-8"))

    def upload(self, data: bytes | bytearray | memoryview | array.array) -> ResidentBuffer:
        raw = bytes(data) if not isinstance(data, (bytes, bytearray)) else data
        c_buf = (ctypes.c_uint8 * len(raw)).from_buffer_copy(raw)
        h = self._lib.cinch_device_upload_raw(self.handle, c_buf, len(raw))
        if h <= 0:
            raise DeviceError(h, "upload failed")
        return ResidentBuffer(self, h, len(raw))

    def upload_f32(self, values: Sequence[float]) -> ResidentBuffer:
        return self.upload(array.array("f", values).tobytes())

    def upload_i32(self, values: Sequence[int]) -> ResidentBuffer:
        return self.upload(array.array("i", values).tobytes())

    def upload_u32(self, values: Sequence[int]) -> ResidentBuffer:
        return self.upload(array.array("I", values).tobytes())

    def download_bytes(self, buf: ResidentBuffer) -> bytes:
        out = (ctypes.c_uint8 * buf.nbytes)()
        rc = self._lib.cinch_device_download_raw(self.handle, buf.handle, out, buf.nbytes)
        if rc != 0:
            raise DeviceError(rc, "download failed")
        return bytes(out)

    def download_u32(self, buf: ResidentBuffer) -> list[int]:
        arr = array.array("I")
        arr.frombytes(self.download_bytes(buf))
        return list(arr)

    def download_i32(self, buf: ResidentBuffer) -> list[int]:
        arr = array.array("i")
        arr.frombytes(self.download_bytes(buf))
        return list(arr)

    def download_f32(self, buf: ResidentBuffer) -> list[float]:
        arr = array.array("f")
        arr.frombytes(self.download_bytes(buf))
        return list(arr)

    def dot_f32(self, a: ResidentBuffer, b: ResidentBuffer, n: int, mode: str = "exact") -> float:
        out = ctypes.c_float(0.0)
        rc = self._lib.cinch_device_dot_f32(self.handle, a.handle, b.handle, n, FLOAT_MODES[mode], ctypes.byref(out))
        if rc != 0:
            raise DeviceError(rc, "dot_f32 failed")
        return float(out.value)

    def scan_i32(self, inp: ResidentBuffer, n: int, inclusive: bool = False) -> ResidentBuffer:
        h = self._lib.cinch_device_scan_i32(self.handle, inp.handle, n, 1 if inclusive else 0)
        if h <= 0:
            raise DeviceError(h, "scan_i32 failed")
        return ResidentBuffer(self, h, n * 4)

    def filter_i32_gt(self, inp: ResidentBuffer, n: int, threshold: int) -> tuple[ResidentBuffer, int]:
        out_k = ctypes.c_int64(0)
        h = self._lib.cinch_device_filter_i32_gt(self.handle, inp.handle, n, threshold, ctypes.byref(out_k))
        if h <= 0:
            raise DeviceError(h, "filter_i32_gt failed")
        k = int(out_k.value)
        return ResidentBuffer(self, h, k * 4), k

    def stencil_histogram_u8(
        self,
        inp: ResidentBuffer,
        frames: int,
        height: int,
        width: int,
        weights: Sequence[int],
        divisor: int,
        edge: str = "clamp",
        steps: int = 1,
        bins: int = 256,
    ) -> ResidentBuffer:
        w9 = (ctypes.c_int32 * 9)(*weights)
        h = self._lib.cinch_device_stencil_histogram_u8(
            self.handle, inp.handle, frames, height, width, w9, divisor, EDGE_CODES[edge], steps, bins
        )
        if h <= 0:
            raise DeviceError(h, "stencil_histogram_u8 failed")
        return ResidentBuffer(self, h, frames * bins * 4)

    def stencil2d_f32(
        self,
        inp: ResidentBuffer,
        frames: int,
        height: int,
        width: int,
        weights: Sequence[float],
        edge: str = "clamp",
        steps: int = 1,
    ) -> ResidentBuffer:
        w9 = (ctypes.c_float * 9)(*weights)
        h = self._lib.cinch_device_stencil2d_f32(
            self.handle, inp.handle, frames, height, width, w9, EDGE_CODES[edge], steps
        )
        if h <= 0:
            raise DeviceError(h, "stencil2d_f32 failed")
        return ResidentBuffer(self, h, frames * height * width * 4)

    def spmv_csr_f32(
        self,
        vals: ResidentBuffer,
        cols: ResidentBuffer,
        row_ptr: ResidentBuffer,
        x: ResidentBuffer,
        nrows: int,
        nnz: int,
        mode: str = "exact",
    ) -> ResidentBuffer:
        h = self._lib.cinch_device_spmv_csr_f32(
            self.handle, vals.handle, cols.handle, row_ptr.handle, x.handle, nrows, nnz, FLOAT_MODES[mode]
        )
        if h <= 0:
            raise DeviceError(h, "spmv_csr_f32 failed")
        return ResidentBuffer(self, h, nrows * 4)

    def row_reduce_f32(self, mat: ResidentBuffer, rows: int, cols: int) -> ResidentBuffer:
        h = self._lib.cinch_device_row_reduce_f32(self.handle, mat.handle, rows, cols)
        if h <= 0:
            raise DeviceError(h, "row_reduce_f32 failed")
        return ResidentBuffer(self, h, rows * 4)
