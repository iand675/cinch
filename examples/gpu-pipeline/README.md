# Cinch Metal GPU Stencil + Histogram Pipeline (Cinch, C, and Python)

This example implements the target numerical pipeline:

- Input: batch of grayscale images with shape `frames × height × width` (`U8`)
- Stage 1: multi-step 3×3 2D convolution stencil with explicit `EdgeClamp` boundary policy and 2-buffer ping-pong storage reuse
- Stage 2: per-frame 256-bin histogram consuming the device-resident stencil output directly (zero host/device round-trip)
- Output: `frames × 256` (`U32`) histogram counts downloaded once

## Prerequisites

Build the Cinch runtime release library (produces `libcinchruntime.a` and `libcinchruntime.dylib`):

```sh
cargo build --manifest-path seed/runtime/Cargo.toml --release
```

## Run from C

```sh
clang -O2 -Iexamples/gpu-pipeline examples/gpu-pipeline/run_pipeline.c \
  seed/runtime/target/release/libcinchruntime.dylib \
  -Wl,-rpath,@executable_path/../../seed/runtime/target/release \
  -o examples/gpu-pipeline/run_pipeline_c
./examples/gpu-pipeline/run_pipeline_c
```

## Run from Python (`ctypes`)

```sh
python3 examples/gpu-pipeline/run_pipeline.py
```

## What to observe

Both callers run two contrasting shapes on the same Apple Metal GPU context:

1. `many-small-frames` (`1024 × 64 × 64`, 3 stencil steps): selects `many-small-frames` histogram schedule (1 threadgroup per frame with shared threadgroup `atomic_uint[256]`).
2. `few-large-frames` (`2 × 2048 × 2048`, 3 stencil steps): selects `few-large-frames` histogram schedule (multi-threadgroup per frame with threadgroup-privatized bins merged into global bins).

In both shapes:
- `uploads == 1` (raw image batch uploaded once)
- `downloads == 1` (only the `frames × 256` histogram counts downloaded)
- `sync_points` explicitly lists the single GPU command-buffer wait boundary
- Every bin matches the scalar CPU reference bit-for-bit.
