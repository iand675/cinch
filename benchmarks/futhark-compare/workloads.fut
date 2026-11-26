-- Futhark benchmark suite matching Cinch's data-parallel array contracts.
-- Compiled with Futhark 0.25.37 (c, multicore, and opencl/metal where supported).

-- 1. Dot product (F32 associative sum & I32 wrapping sum)
entry dot_f32 [n] (xs: [n]f32) (ys: [n]f32) : f32 =
  reduce (+) 0.0f32 (map2 (*) xs ys)

entry dot_i32 [n] (xs: [n]i32) (ys: [n]i32) : i32 =
  reduce (+) 0i32 (map2 (*) xs ys)

-- 2. Inclusive prefix scan (I32 wrapping add) and stable compaction (filter > thr)
entry scan_i32_incl [n] (xs: [n]i32) : [n]i32 =
  scan (+) 0i32 xs

entry filter_i32_gt [n] (xs: [n]i32) (thr: i32) : []i32 =
  filter (> thr) xs

-- 3. 256-bin histogram over U8 (both uniform and skewed/hot-bin inputs)
entry histogram_u8 [n] (xs: [n]u8) : [256]u32 =
  let idxs = map i64.u8 xs
  let ones = replicate n 1u32
  in reduce_by_index (replicate 256 0u32) (+) 0u32 idxs ones

-- 4a. Multi-step 2D 3x3 F32 stencil with EdgeClamp boundary policy
def clamp_i64 (x: i64) (hi: i64) : i64 =
  if x < 0 then 0 else if x >= hi then hi - 1 else x

def stencil2d_f32_step [h][w] (img: [h][w]f32) (ws: [9]f32) : [h][w]f32 =
  tabulate_2d h w (\y x ->
    let y0 = clamp_i64 (y - 1) h
    let y1 = y
    let y2 = clamp_i64 (y + 1) h
    let x0 = clamp_i64 (x - 1) w
    let x1 = x
    let x2 = clamp_i64 (x + 1) w
    in img[y0, x0] * ws[0] + img[y0, x1] * ws[1] + img[y0, x2] * ws[2]
     + img[y1, x0] * ws[3] + img[y1, x1] * ws[4] + img[y1, x2] * ws[5]
     + img[y2, x0] * ws[6] + img[y2, x1] * ws[7] + img[y2, x2] * ws[8])

entry stencil2d_f32 [h][w] (img: [h][w]f32) (ws: [9]f32) (steps: i64) : [h][w]f32 =
  loop cur = img for _i < steps do
    stencil2d_f32_step cur ws

-- 4b. Target pipeline: batched frames x h x w U8 -> 3x3 EdgeClamp stencil -> per-frame 256-bin histogram
def stencil2d_u8_step [h][w] (img: [h][w]u8) (ws: [9]i32) (divisor: i32) : [h][w]u8 =
  tabulate_2d h w (\y x ->
    let y0 = clamp_i64 (y - 1) h
    let y1 = y
    let y2 = clamp_i64 (y + 1) h
    let x0 = clamp_i64 (x - 1) w
    let x1 = x
    let x2 = clamp_i64 (x + 1) w
    let acc =
      i32.u8 img[y0, x0] * ws[0] + i32.u8 img[y0, x1] * ws[1] + i32.u8 img[y0, x2] * ws[2]
      + i32.u8 img[y1, x0] * ws[3] + i32.u8 img[y1, x1] * ws[4] + i32.u8 img[y1, x2] * ws[5]
      + i32.u8 img[y2, x0] * ws[6] + i32.u8 img[y2, x1] * ws[7] + i32.u8 img[y2, x2] * ws[8]
    let q = acc / divisor
    let c = if q < 0 then 0 else if q > 255 then 255 else q
    in u8.i32 c)

entry stencil_histogram_batch_u8 [f][h][w]
    (imgs: [f][h][w]u8) (ws: [9]i32) (divisor: i32) (steps: i64) : [f][256]u32 =
  let blurred =
    loop cur = imgs for _s < steps do
      map (\frame -> stencil2d_u8_step frame ws divisor) cur
  in map (\frame ->
    let flat = flatten frame
    let idxs = map i64.u8 flat
    let ones = replicate (h * w) 1u32
    in reduce_by_index (replicate 256 0u32) (+) 0u32 idxs ones) blurred

-- 5. CSR Sparse Matrix-Vector Multiplication with empty rows (segmented reduction)
entry spmv_csr_f32 [nnz][rp1][nx]
    (vals: [nnz]f32) (cols: [nnz]i32) (row_ptr: [rp1]u32) (x: [nx]f32) : []f32 =
  let num_rows = rp1 - 1
  in tabulate num_rows (\r ->
    let start = i64.u32 row_ptr[r]
    let end_i = i64.u32 row_ptr[r + 1]
    in loop acc = 0.0f32 for j in start..<end_i do
         let c = i64.i32 cols[j]
         in acc + vals[j] * x[c])
