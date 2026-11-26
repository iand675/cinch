//! End-to-end integration tests for Cinch forward-mode (JVP) and reverse-mode
//! (VJP) automatic differentiation (`Data.AD`), covering scalars, transcendentals,
//! multi-variable tuples & records, control flow & loops, higher-order AD
//! (second derivatives and forward-over-reverse Hessian-vector products),
//! parallel array AD (including gather/scatter-add and 2D stencil adjoints),
//! and rank-polymorphic N-dimensional array AD (`Data.Array.ND`).

mod common;

use std::process::Command;

const SOURCES: &[(&str, &str)] = &[
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Array.Packed.cinch", include_str!("../prelude/Data/Array/Packed.cinch")),
    ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
    ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
    ("Data.Array.Compute.cinch", include_str!("../prelude/Data/Array/Compute.cinch")),
    ("Data.Array.ND.cinch", include_str!("../prelude/Data/Array/ND.cinch")),
    ("Data.AD.cinch", include_str!("../prelude/Data/AD.cinch")),
];

fn run_cinch(tag: &str, main_src: &str) {
    let mut sources = SOURCES.to_vec();
    sources.push(("Main.cinch", main_src));
    let dir = common::project(tag, &sources);
    let exe = common::build(&dir, "Main.cinch", "program").expect("build program");
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(0),
        "tag={tag}\nstdout:\n{stdout}\nstderr:\n{stderr}\ndir: {}",
        dir.display()
    );
    common::cleanup(&dir);
}

#[test]
fn scalar_jvp_and_vjp_arithmetic_and_transcendentals() {
    run_cinch(
        "ad_scalar_transcendentals",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Machine
  import Data.Bool
  import Data.AD

  approx-f64 :: F64 -> F64 -> F64 -> Bool
  approx-f64 a b tol = flt-f64 (abs-f64 (sub-f64 a b)) tol

  approx-f32 :: F32 -> F32 -> F32 -> Bool
  approx-f32 a b tol = flt-f32 (abs-f32 (sub-f32 a b)) tol

  -- f(x) = sin(x^2) * exp(-0.5 * x) + log(x + 2) / sqrt(x + 1) + tanh(x)
  f64 :: F64 -> F64
  f64 x =
    let t1 = mul-f64 (sin-f64 (mul-f64 x x)) (exp-f64 (mul-f64 (neg-f64 0.5) x))
        t2 = div-f64 (log-f64 (add-f64 x 2.0)) (sqrt-f64 (add-f64 x 1.0))
        t3 = tanh-f64 x
    in add-f64 (add-f64 t1 t2) t3

  -- Central finite difference for ground truth verification
  fd64 :: F64 -> F64
  fd64 x =
    let h = 0.000001
    in div-f64 (sub-f64 (f64 (add-f64 x h)) (f64 (sub-f64 x h))) (mul-f64 2.0 h)

  f32 :: F32 -> F32
  f32 x = add-f32 (mul-f32 (cos-f32 x) (exp-f32 x)) (div-f32 (sqrt-f32 x) (add-f32 x 1.0))

  fd32 :: F32 -> F32
  fd32 x =
    let h = 0.001
    in div-f32 (sub-f32 (f32 (add-f32 x h)) (f32 (sub-f32 x h))) (mul-f32 2.0 h)

  main :: Int
  main =
    let x = 1.3
        dy-fd = fd64 x
        x32 = 2.5
        d32-fd = fd32 x32
    in case jvp2 f64 x 1.0 of
      (y-j, dy-j) -> case vjp2 f64 x 1.0 of
        (y-v, dy-v) -> case jvp2 f32 x32 1.0 of
          (y32-j, d32-j) -> case vjp2 f32 x32 1.0 of
            (y32-v, d32-v) ->
              let ok-val = approx-f64 y-j (f64 x) 0.000000000001 && approx-f64 y-v (f64 x) 0.000000000001
                  ok-jv = approx-f64 dy-j dy-v 0.000000000001
                  ok-fd = approx-f64 dy-j dy-fd 0.000001
                  ok-f32 = approx-f32 y32-j y32-v 0.00001 && approx-f32 d32-j d32-v 0.00001 && approx-f32 d32-j d32-fd 0.005
              in if ok-val then (if ok-jv then (if ok-fd then (if ok-f32 then 0 else 40) else 30) else 20) else 10
"#,
    );
}

#[test]
fn multivar_tuples_rosenbrock_records_and_adjoint_identity() {
    run_cinch(
        "ad_multivar_rosenbrock",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Machine
  import Data.Bool
  import Data.AD

  approx :: F64 -> F64 -> F64 -> Bool
  approx a b tol = flt-f64 (abs-f64 (sub-f64 a b)) tol

  -- Rosenbrock: f(x, y) = (1 - x)^2 + 100 * (y - x^2)^2
  rosenbrock :: (F64, F64) -> F64
  rosenbrock p = case p of
    (x, y) ->
      let a = sub-f64 1.0 x
          b = sub-f64 y (mul-f64 x x)
      in add-f64 (mul-f64 a a) (mul-f64 100.0 (mul-f64 b b))

  -- Vector-to-vector map g(x, y) = (x^2 * y + sin(x), exp(y) - x / (y + 2))
  vec-fn :: (F64, F64) -> (F64, F64)
  vec-fn p = case p of
    (x, y) ->
      let o1 = add-f64 (mul-f64 (mul-f64 x x) y) (sin-f64 x)
          o2 = sub-f64 (exp-f64 y) (div-f64 x (add-f64 y 2.0))
      in (o1, o2)

  -- User-defined single-constructor record differentiated by AD
  data Vec2 where
    MkVec2 :: F64 -> F64 -> Vec2

  vec2-energy :: Vec2 -> F64
  vec2-energy v = case v of
    MkVec2 x y -> add-f64 (mul-f64 3.0 (mul-f64 x x)) (mul-f64 x (mul-f64 y y))

  main :: Int
  main =
    let pt = (1.5, 2.0)
        v = (0.7, neg-f64 1.3)
        u = (2.1, 0.4)
        grec = grad-f64 vec2-energy (MkVec2 2.0 3.0)
    in case value-and-grad-f64 rosenbrock pt of
      (val, grad-ros) -> case grad-ros of
        (gx, gy) -> case jvp vec-fn pt v of
          (jv1, jv2) -> case vjp vec-fn pt u of
            (jtu1, jtu2) ->
              let ok-ros = approx val 6.5 0.000000001
                        && approx gx 151.0 0.000000001
                        && approx gy (neg-f64 50.0) 0.000000001
                  lhs = add-f64 (mul-f64 jv1 2.1) (mul-f64 jv2 0.4)
                  rhs = add-f64 (mul-f64 0.7 jtu1) (mul-f64 (neg-f64 1.3) jtu2)
                  ok-adj = approx lhs rhs 0.0000000001
                  ok-rec = case grec of
                    MkVec2 rx ry -> approx rx 21.0 0.000000001 && approx ry 12.0 0.000000001
              in if ok-ros && ok-adj && ok-rec then 0 else 1
"#,
    );
}

#[test]
fn control_flow_and_recursive_loops() {
    run_cinch(
        "ad_control_flow_and_loops",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Machine
  import Data.Nat
  import Data.Bool
  import Relation.Ord
  import Algebra.Arithmetic
  import Data.AD

  approx :: F64 -> F64 -> F64 -> Bool
  approx a b tol = flt-f64 (abs-f64 (sub-f64 a b)) tol

  leaky-relu :: F64 -> F64
  leaky-relu x =
    if fge-f64 x 0.0
      then mul-f64 x x
      else mul-f64 0.1 x

  -- Recursive power: pow x n = x^n
  pow-f64 :: F64 -> Nat -> F64
  pow-f64 x n =
    if n == 0
      then 1.0
      else mul-f64 x (pow-f64 x (n - 1))

  -- Tail-recursive Horner loop: 1 + 2x + 3x^2 + 4x^3 (acc starts at 4.0)
  horner-loop :: F64 -> Nat -> F64 -> F64
  horner-loop x i acc =
    if i == 0
      then acc
      else
        let k = i - 1
            coeff = if k == 0 then 1.0 else if k == 1 then 2.0 else if k == 2 then 3.0 else 4.0
        in horner-loop x k (add-f64 coeff (mul-f64 x acc))

  poly :: F64 -> F64
  poly x = horner-loop x 3 4.0

  main :: Int
  main =
    let d-pos-j = deriv-f64 leaky-relu 3.0
        d-pos-v = grad-f64 leaky-relu 3.0
        d-neg-j = deriv-f64 leaky-relu (neg-f64 2.0)
        d-neg-v = grad-f64 leaky-relu (neg-f64 2.0)
        ok-if = approx d-pos-j 6.0 0.000000001
             && approx d-pos-v 6.0 0.000000001
             && approx d-neg-j 0.1 0.000000001
             && approx d-neg-v 0.1 0.000000001
        dp-j = deriv-f64 (\x -> pow-f64 x 5) 2.0
        dp-v = grad-f64 (\x -> pow-f64 x 5) 2.0
        ok-pow-j = approx dp-j 80.0 0.000000001
        ok-pow-v = approx dp-v 80.0 0.000000001
        dh-j = deriv-f64 poly 2.0
        dh-v = grad-f64 poly 2.0
        ok-horner-j = approx dh-j 62.0 0.000000001
        ok-horner-v = approx dh-v 62.0 0.000000001
    in if ok-if then (if ok-pow-j then (if ok-pow-v then (if ok-horner-j then (if ok-horner-v then 0 else 32) else 31) else 22) else 21) else 10
"#,
    );
}

#[test]
fn higher_order_second_derivative_and_forward_over_reverse_hvp() {
    run_cinch(
        "ad_higher_order_hvp",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Machine
  import Data.Bool
  import Data.AD

  approx :: F64 -> F64 -> F64 -> Bool
  approx a b tol = flt-f64 (abs-f64 (sub-f64 a b)) tol

  -- f(x) = sin(x^2)
  -- f'(x) = 2 x cos(x^2)
  -- f''(x) = 2 cos(x^2) - 4 x^2 sin(x^2)
  f-sin-sq :: F64 -> F64
  f-sin-sq x = sin-f64 (mul-f64 x x)

  rosenbrock :: (F64, F64) -> F64
  rosenbrock p = case p of
    (x, y) ->
      let a = sub-f64 1.0 x
          b = sub-f64 y (mul-f64 x x)
      in add-f64 (mul-f64 a a) (mul-f64 100.0 (mul-f64 b b))

  main :: Int
  main =
    let x = 1.2
        d2 = deriv-f64 (\t -> deriv-f64 f-sin-sq t) x
        x2 = mul-f64 x x
        exact-d2 = sub-f64 (mul-f64 2.0 (cos-f64 x2)) (mul-f64 (mul-f64 4.0 x2) (sin-f64 x2))
        ok-d2 = approx d2 exact-d2 0.000000001
    in case hvp-f64 rosenbrock (1.5, 2.0) (0.7, neg-f64 0.3) of
      (hv0, hv1) ->
        let ok-hvp = approx hv0 1511.4 0.0000001 && approx hv1 (neg-f64 480.0) 0.0000001
        in if ok-d2 then (if ok-hvp then 0 else 20) else 10
"#,
    );
}

#[test]
fn parallel_array_jvp_and_vjp() {
    run_cinch(
        "ad_parallel_arrays",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine
  import Data.Nat
  import Data.Bool
  import Data.AD

  approx :: F32 -> F32 -> F32 -> Bool
  approx a b tol = flt-f32 (abs-f32 (sub-f32 a b)) tol

  get-f32 :: Array F32 -> Nat -> F32
  get-f32 a i = case array-get a i of
    Some v -> v
    None -> 0.0

  -- Nonlinear scalar loss over a packed F32 array:
  -- L(x) = sum(map (\t -> t^3 + sin(t)) x) + dot(x * x, w)
  array-loss :: Array F32 -> Array F32 -> F32
  array-loss w x =
    let mapped = array-map-f32 (\t -> add-f32 (mul-f32 t (mul-f32 t t)) (sin-f32 t)) x
        s1 = array-reduce-f32 mapped 10
        sq = array-map-unary-f32 x 3 0.0
        s2 = array-dot-f32 sq w 10
    in add-f32 s1 s2

  -- Exact analytical gradient: dL/dx_i = 3*x_i^2 + cos(x_i) + 2*x_i*w_i
  exact-grad-at :: F32 -> F32 -> F32
  exact-grad-at xi wi =
    add-f32 (add-f32 (mul-f32 3.0 (mul-f32 xi xi)) (cos-f32 xi)) (mul-f32 2.0 (mul-f32 xi wi))

  main :: Int
  main =
    let x = array-from-f32s (Cons 0.5 (Cons 1.0 (Cons 1.5 (Cons (neg-f32 0.75) Nil))))
        w = array-from-f32s (Cons 2.0 (Cons (neg-f32 1.0) (Cons 0.5 (Cons 3.0 Nil))))
        v = array-from-f32s (Cons 0.25 (Cons (neg-f32 0.5) (Cons 1.0 (Cons 0.75 Nil))))
    in case value-and-grad-f32 (\a -> array-loss w a) x of
      (loss-v, gx) -> case jvp2 (\a -> array-loss w a) x v of
        (loss-j, dloss-j) ->
          let dloss-v = array-dot-f32 gx v 10
              ok-loss = approx loss-v loss-j 0.0001
              ok-adj = approx dloss-j dloss-v 0.0005
              ok-g0 = approx (get-f32 gx 0) (exact-grad-at 0.5 2.0) 0.0005
              ok-g1 = approx (get-f32 gx 1) (exact-grad-at 1.0 (neg-f32 1.0)) 0.0005
              ok-g2 = approx (get-f32 gx 2) (exact-grad-at 1.5 0.5) 0.0005
              ok-g3 = approx (get-f32 gx 3) (exact-grad-at (neg-f32 0.75) 3.0) 0.0005
          in if ok-loss && ok-adj && ok-g0 && ok-g1 && ok-g2 && ok-g3 then 0 else 1
"#,
    );
}

#[test]
fn parallel_array_row_reduce_gather_and_stencil2d_adjoint() {
    run_cinch(
        "ad_row_gather_stencil",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Machine
  import Data.Nat
  import Data.Bool
  import Data.AD

  approx :: F32 -> F32 -> F32 -> Bool
  approx a b tol = flt-f32 (abs-f32 (sub-f32 a b)) tol

  get-f32 :: Array F32 -> Nat -> F32
  get-f32 a i = case array-get a i of
    Some v -> v
    None -> 0.0

  unwrap-arr :: Array F32 -> Option (Array F32) -> Array F32
  unwrap-arr fb o = case o of
    Some a -> a
    None -> fb

  main :: Int
  main =
    let -- 1. Row reduction VJP: 2x3 matrix -> row sums -> dot with [3.0, 5.0]
        mat = array-from-f32s (Cons 1.0 (Cons 2.0 (Cons 3.0 (Cons 4.0 (Cons 5.0 (Cons 6.0 Nil))))))
        rw = array-from-f32s (Cons 3.0 (Cons 5.0 Nil))
        gmat = grad-f32 (\m -> array-dot-f32 (unwrap-arr rw (array-row-reduce-f32 m 2 3 10)) rw 10) mat
        ok-rr = approx (get-f32 gmat 0) 3.0 0.0001
             && approx (get-f32 gmat 2) 3.0 0.0001
             && approx (get-f32 gmat 3) 5.0 0.0001
             && approx (get-f32 gmat 5) 5.0 0.0001

        -- 2. Gather VJP with repeated indices [1, 0, 1, 2] and weights [2, 3, 5, 7]
        src = array-from-f32s (Cons 10.0 (Cons 20.0 (Cons 30.0 Nil)))
        idx = array-from-i32s (Cons 1 (Cons 0 (Cons 1 (Cons 2 Nil))))
        gw = array-from-f32s (Cons 2.0 (Cons 3.0 (Cons 5.0 (Cons 7.0 Nil))))
        gsrc = grad-f32 (\s -> array-dot-f32 (array-gather-f32 s idx 0) gw 10) src
        ok-gath = approx (get-f32 gsrc 0) 3.0 0.0001
               && approx (get-f32 gsrc 1) 7.0 0.0001
               && approx (get-f32 gsrc 2) 7.0 0.0001

        -- 3. 2D Stencil VJP vs JVP adjoint identity (2 steps, asymmetric 3x3 weights, EdgeClamp)
        img = array-from-f32s (Cons 0.1 (Cons 0.2 (Cons 0.3 (Cons 0.4 (Cons 0.5 (Cons 0.6 (Cons 0.7 (Cons 0.8 (Cons 0.9 (Cons 1.0 (Cons 1.1 (Cons 1.2 (Cons 1.3 (Cons 1.4 (Cons 1.5 (Cons 1.6 Nil))))))))))))))))
        dimg = array-from-f32s (Cons 0.25 (Cons 0.30 (Cons 0.35 (Cons 0.40 (Cons 0.45 (Cons 0.50 (Cons 0.55 (Cons 0.60 (Cons 0.65 (Cons 0.70 (Cons 0.75 (Cons 0.80 (Cons 0.85 (Cons 0.90 (Cons 0.95 (Cons 1.00 Nil))))))))))))))))
        outw = array-from-f32s (Cons 1.00 (Cons 0.96 (Cons 0.92 (Cons 0.88 (Cons 0.84 (Cons 0.80 (Cons 0.76 (Cons 0.72 (Cons 0.68 (Cons 0.64 (Cons 0.60 (Cons 0.56 (Cons 0.52 (Cons 0.48 (Cons 0.44 (Cons 0.40 Nil))))))))))))))))
        wts = array-from-f32s (Cons 0.05 (Cons 0.1 (Cons 0.07 (Cons 0.12 (Cons 0.4 (Cons 0.08 (Cons 0.06 (Cons 0.09 (Cons 0.03 Nil)))))))))
        st-loss = \x -> array-dot-f32 (unwrap-arr x (array-stencil2d-f32 x 1 4 4 wts 0 2)) outw 10
        gimg = grad-f32 st-loss img
        j-val = jvp st-loss img dimg
        v-val = array-dot-f32 gimg dimg 10
        ok-st = approx j-val v-val 0.0005
    in if ok-rr then (if ok-gath then (if ok-st then 0 else 30) else 20) else 10
"#,
    );
}

#[test]
fn rank_polymorphic_nd_array_ad() {
    run_cinch(
        "ad_nd_arrays",
        r#"module Main where
  import Data.Core
  import Data.Int
  import Data.Array
  import Data.Array.Packed
  import Data.Array.Compute
  import Data.Array.ND
  import Data.Machine
  import Data.Nat
  import Data.Bool
  import Data.AD

  approx :: F32 -> F32 -> F32 -> Bool
  approx a b tol = flt-f32 (abs-f32 (sub-f32 a b)) tol

  get-f32 :: Array F32 -> Nat -> F32
  get-f32 a i = case array-get a i of
    Some v -> v
    None -> 0.0

  -- Rank-polymorphic loss on a 2D view (Shape2 2 3):
  -- square every element, reduce along the last axis (2x3 -> 2), then dot with weights!
  nd-loss :: NDView (Shape1 2) F32 -> NDView (Shape2 2 3) F32 -> F32
  nd-loss w v =
    let sq = nd-map-f32 (\x -> mul-f32 x x) v
        red = nd-reduce-last-f32 sq FloatExactTree
    in nd-dot-f32 red w FloatExactTree

  main :: Int
  main =
    let s23 = SWDim 2 (SWDim 3 SWZ)
        s2 = SWDim 2 SWZ
        raw-x = array-from-f32s (Cons 1.0 (Cons 2.0 (Cons 3.0 (Cons 4.0 (Cons 5.0 (Cons 6.0 Nil))))))
        raw-w = array-from-f32s (Cons 2.0 (Cons 3.0 Nil))
        raw-dx = array-from-f32s (Cons 0.5 (Cons 1.0 (Cons (neg-f32 0.5) (Cons 0.25 (Cons 0.75 (Cons (neg-f32 1.0) Nil))))))
    in case nd-from-array s23 raw-x of
      None -> 2
      Some vx -> case nd-from-array s2 raw-w of
        None -> 3
        Some vw -> case nd-from-array s23 raw-dx of
          None -> 4
          Some vdx ->
            case value-and-grad-f32 (\m -> nd-loss vw m) vx of
              (loss-v, gvx) -> case jvp2 (\m -> nd-loss vw m) vx vdx of
                (loss-j, dloss-j) ->
                  let ga = nd-data gvx
                      dloss-v = array-dot-f32 ga raw-dx 10
                      ok-val = approx loss-v 259.0 0.001 && approx loss-j 259.0 0.001
                      ok-adj = approx dloss-j dloss-v 0.005
                      ok-grad = approx (get-f32 ga 0) 4.0 0.001
                             && approx (get-f32 ga 1) 8.0 0.001
                             && approx (get-f32 ga 2) 12.0 0.001
                             && approx (get-f32 ga 3) 24.0 0.001
                             && approx (get-f32 ga 4) 30.0 0.001
                             && approx (get-f32 ga 5) 36.0 0.001
                  in if ok-val then (if ok-adj then (if ok-grad then 0 else 30) else 20) else 10
"#,
    );
}
