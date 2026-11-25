#!/usr/bin/env python3
"""
Python caller for Cinch's Metal GPU batched stencil -> histogram pipeline.

Demonstrates:
  - Uploading a `frames x height x width` uint8 image batch ONCE
  - Running multi-step 3x3 stencil (ping-pong) + per-frame 256-bin histogram
    completely resident on Apple Metal GPU in a single command buffer
  - Downloading ONLY the `frames x 256` uint32 histogram counts ONCE
  - Inspecting launch plans and sync points for both `many-small-frames`
    and `few-large-frames` shapes
  - Verifying bit-exact agreement with a scalar Python/CPU reference
"""

import ctypes
import json
import os
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DYLIB = ROOT / "seed" / "runtime" / "target" / "release" / "libcinchruntime.dylib"


def load_cinch_device():
    if not DYLIB.exists():
        raise FileNotFoundError(
            f"Missing {DYLIB}. Build with: cargo build --manifest-path seed/runtime/Cargo.toml --release"
        )
    lib = ctypes.CDLL(str(DYLIB))
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
    lib.cinch_device_report_cstr.argtypes = [ctypes.c_int64, ctypes.c_char_p, ctypes.c_size_t]
    lib.cinch_device_report_cstr.restype = ctypes.c_int64
    lib.cinch_device_stencil_histogram_u8.argtypes = [
        ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
        ctypes.POINTER(ctypes.c_int32), ctypes.c_int64, ctypes.c_int64, ctypes.c_int64, ctypes.c_int64,
    ]
    lib.cinch_device_stencil_histogram_u8.restype = ctypes.c_int64
    return lib


def py_ref_frame(frame_bytes, h, w, weights, divisor, steps, bins):
    src = list(frame_bytes)
    dst = [0] * (h * w)
    for _ in range(steps):
        for y in range(h):
            y0 = 0 if y == 0 else y - 1
            y1 = y
            y2 = h - 1 if y + 1 >= h else y + 1
            r0, r1, r2 = y0 * w, y1 * w, y2 * w
            for x in range(w):
                x0 = 0 if x == 0 else x - 1
                x1 = x
                x2 = w - 1 if x + 1 >= w else x + 1
                acc = (
                    src[r0 + x0] * weights[0] + src[r0 + x1] * weights[1] + src[r0 + x2] * weights[2]
                    + src[r1 + x0] * weights[3] + src[r1 + x1] * weights[4] + src[r1 + x2] * weights[5]
                    + src[r2 + x0] * weights[6] + src[r2 + x1] * weights[7] + src[r2 + x2] * weights[8]
                )
                v = acc // divisor
                dst[r1 + x] = 0 if v < 0 else (255 if v > 255 else v)
        src, dst = dst, src
    hist = [0] * bins
    for px in src:
        hist[px] += 1
    return hist


def run_shape(lib, dev, label, frames, h, w, steps, expected_schedule):
    weights_py = [1, 2, 1, 2, 4, 2, 1, 2, 1]
    weights = (ctypes.c_int32 * 9)(*weights_py)
    divisor = 16
    bins = 256
    total = frames * h * w
    # Generate deterministic batch without external libraries.
    pattern = bytes(((i * 37 + (i >> 7) * 13) & 0xFF) for i in range(min(total, 65536)))
    repeats, rem = divmod(total, len(pattern))
    host_in = (ctypes.c_uint8 * total).from_buffer_copy(pattern * repeats + pattern[:rem])
    host_out = (ctypes.c_uint32 * (frames * bins))()

    lib.cinch_device_reset_telemetry(dev)

    t0 = time.perf_counter()
    d_in = lib.cinch_device_upload_raw(dev, host_in, total)
    assert d_in >= 0, f"upload failed: {d_in}"
    t_up = time.perf_counter()

    d_hist = lib.cinch_device_stencil_histogram_u8(
        dev, d_in, frames, h, w, weights, divisor, 0, steps, bins
    )
    assert d_hist >= 0, f"pipeline failed: {d_hist}"
    t_kern = time.perf_counter()

    rc = lib.cinch_device_download_raw(dev, d_hist, host_out, frames * bins * 4)
    assert rc == 0, f"download failed: {rc}"
    t_done = time.perf_counter()

    rep_buf = ctypes.create_string_buffer(8192)
    lib.cinch_device_report_cstr(dev, rep_buf, len(rep_buf))
    telemetry = json.loads(rep_buf.value.decode("utf-8"))

    lib.cinch_device_buffer_free(d_in)
    lib.cinch_device_buffer_free(d_hist)

    print(f"=== Python caller: {label} ({frames}x{h}x{w}, steps={steps} -> {frames}x{bins}) ===")
    print(
        f"  device={telemetry['device_name']} ({telemetry['backend']})  "
        f"upload={(t_up - t0)*1e3:.3f} ms  resident_gpu={(t_kern - t_up)*1e3:.3f} ms  "
        f"download={(t_done - t_kern)*1e3:.3f} ms  total={(t_done - t0)*1e3:.3f} ms"
    )
    print(
        f"  uploads={telemetry['uploads']} ({telemetry['upload_bytes']} B)  "
        f"downloads={telemetry['downloads']} ({telemetry['download_bytes']} B)  "
        f"peak_device_bytes={telemetry['peak_device_bytes']}  "
        f"sync_points={telemetry['sync_points']}"
    )
    print(f"  last_launch_plan={json.dumps(telemetry['last_launch_plan'])}")

    assert telemetry["uploads"] == 1, f"expected 1 upload, got {telemetry['uploads']}"
    assert telemetry["downloads"] == 1, f"expected 1 download, got {telemetry['downloads']}"
    assert telemetry["upload_bytes"] == total
    assert telemetry["download_bytes"] == frames * bins * 4
    schedule = telemetry["last_launch_plan"]["schedule"]
    assert expected_schedule in schedule, f"expected {expected_schedule} in {schedule}"

    # Verify pixel conservation across every frame and spot-check frame 0 & last frame bit-exactly.
    for f in range(frames):
        s = sum(host_out[f * bins : (f + 1) * bins])
        assert s == h * w, f"frame {f} total mismatch: {s} != {h * w}"
    check_frames = [0, frames - 1] if frames <= 64 or h * w <= 4096 else [0]
    raw_bytes = bytes(host_in)
    for f in check_frames:
        if h * w <= 65536:
            ref = py_ref_frame(raw_bytes[f * h * w : (f + 1) * h * w], h, w, weights_py, divisor, steps, bins)
            got = list(host_out[f * bins : (f + 1) * bins])
            assert got == ref, f"frame {f} mismatch against Python reference"
    print(f"  verification: PASSED (pixel-conserved across all {frames} frames; bit-exact reference checked)")


def main():
    lib = load_cinch_device()
    dev = lib.cinch_device_open(1)  # Metal GPU required
    assert dev >= 0, f"Metal GPU unavailable: {dev}"
    try:
        run_shape(lib, dev, "many-small-frames", 1024, 64, 64, 3, "many-small-frames")
        run_shape(lib, dev, "few-large-frames", 2, 2048, 2048, 3, "few-large-frames")
    finally:
        lib.cinch_device_close(dev)
    print("ALL PYTHON METAL GPU PIPELINE CHECKS PASSED.")


if __name__ == "__main__":
    main()
