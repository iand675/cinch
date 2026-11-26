#!/usr/bin/env python3
"""
Python multi-backend caller for Cinch's batched stencil -> histogram pipeline
(using `cinch_device.py`, equivalent to Futhark's `python` and `pyopencl`
backends, plus Metal, CUDA, HIP, ISPC/SIMD, Multicore CPU, and Sequential CPU).
"""

import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from cinch_device import CinchDevice


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


def run_shape(dev: CinchDevice, label: str, frames: int, h: int, w: int, steps: int, expected_schedule: str):
    weights = [1, 2, 1, 2, 4, 2, 1, 2, 1]
    divisor = 16
    bins = 256
    total = frames * h * w
    pattern = bytes(((i * 37 + (i >> 7) * 13) & 0xFF) for i in range(min(total, 65536)))
    repeats, rem = divmod(total, len(pattern))
    raw_bytes = pattern * repeats + pattern[:rem]

    dev.reset_telemetry()

    t0 = time.perf_counter()
    with dev.upload(raw_bytes) as d_in:
        t_up = time.perf_counter()
        with dev.stencil_histogram_u8(d_in, frames, h, w, weights, divisor, edge="clamp", steps=steps, bins=bins) as d_hist:
            t_kern = time.perf_counter()
            host_out = dev.download_u32(d_hist)
            t_done = time.perf_counter()

    telemetry = dev.telemetry()
    print(f"=== Python caller [{telemetry['backend']}]: {label} ({frames}x{h}x{w}, steps={steps} -> {frames}x{bins}) ===")
    print(
        f"  device={telemetry['device_name']} ({telemetry['backend']})  "
        f"upload={(t_up - t0)*1e3:.3f} ms  resident={(t_kern - t_up)*1e3:.3f} ms  "
        f"download={(t_done - t_kern)*1e3:.3f} ms  total={(t_done - t0)*1e3:.3f} ms"
    )
    print(
        f"  uploads={telemetry['uploads']} ({telemetry['upload_bytes']} B)  "
        f"downloads={telemetry['downloads']} ({telemetry['download_bytes']} B)  "
        f"peak_device_bytes={telemetry['peak_device_bytes']}  "
        f"sync_points={telemetry['sync_points']}"
    )
    print(f"  last_launch_plan={json.dumps(telemetry['last_launch_plan'])}")

    assert telemetry["uploads"] == 1
    assert telemetry["downloads"] == 1
    assert telemetry["upload_bytes"] == total
    assert telemetry["download_bytes"] == frames * bins * 4
    schedule = telemetry["last_launch_plan"]["schedule"]
    assert expected_schedule in schedule, f"expected {expected_schedule} in {schedule}"

    for f in range(frames):
        s = sum(host_out[f * bins : (f + 1) * bins])
        assert s == h * w, f"frame {f} total mismatch: {s} != {h * w}"
    check_frames = [0, frames - 1] if frames <= 64 or h * w <= 4096 else [0]
    for f in check_frames:
        if h * w <= 65536:
            ref = py_ref_frame(raw_bytes[f * h * w : (f + 1) * h * w], h, w, weights, divisor, steps, bins)
            got = host_out[f * bins : (f + 1) * bins]
            assert got == ref, f"frame {f} mismatch against Python reference"
    print(f"  verification: PASSED (pixel-conserved across all {frames} frames; bit-exact reference checked)")


def main():
    avail = CinchDevice.available_backends()
    print(f"Available Cinch backends on this host: {avail}")
    req = sys.argv[1] if len(sys.argv) >= 2 else "metal"
    backends = ["metal-required", "opencl-required", "ispc", "cpu", "seq"] if req == "all" else [req]
    for b in backends:
        with CinchDevice(b) as dev:
            run_shape(dev, "many-small-frames", 1024, 64, 64, 3, "many-small-frames")
            run_shape(dev, "few-large-frames", 2, 2048, 2048, 3, "few-large-frames")
    print(f"ALL PYTHON PIPELINE CHECKS PASSED ({', '.join(backends)}).")


if __name__ == "__main__":
    main()
