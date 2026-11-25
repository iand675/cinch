//! Integration tests for Cinch type families, data families, GADT branch
//! refinement, and rank-polymorphic N-dimensional arrays (`Data.Array.ND`).

mod common;

use std::process::Command;

const SOURCES: &[(&str, &str)] = &[
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Array.Packed.cinch", include_str!("../prelude/Data/Array/Packed.cinch")),
    ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
    ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
    ("Data.Array.Compute.cinch", include_str!("../prelude/Data/Array/Compute.cinch")),
    ("Data.Array.ND.cinch", include_str!("../prelude/Data/Array/ND.cinch")),
];

fn run_cinch(tag: &str, extra_modules: &[(&str, &str)], main_src: &str) {
    let mut sources = SOURCES.to_vec();
    sources.extend_from_slice(extra_modules);
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

fn expect_build_error(tag: &str, extra_modules: &[(&str, &str)], main_src: &str, expected_substrings: &[&str]) {
    let mut sources = SOURCES.to_vec();
    sources.extend_from_slice(extra_modules);
    sources.push(("Main.cinch", main_src));
    let dir = common::project(tag, &sources);
    let res = common::build(&dir, "Main.cinch", "program");
    common::cleanup(&dir);
    let err = match res {
        Ok(exe) => panic!("tag={tag}: expected compile error, but succeeded producing {}", exe.display()),
        Err(e) => e,
    };
    for sub in expected_substrings {
        assert!(
            err.to_lowercase().contains(&sub.to_lowercase()),
            "tag={tag}: expected error containing `{sub}`, got:\n{err}"
        );
    }
}

#[test]
fn closed_and_open_type_families_reduce_types_and_nats() {
    run_cinch(
        "tf_closed_open",
        &[(
            "Ext.cinch",
            r#"module Ext where
  import Data.Array.ND
  import Data.Core
  import Data.Nat

  -- Extend open type family `AccumType` from another module!
  type instance AccumType Int = Int

  -- Extend open data family across module boundaries!
  data family Boxed a
  data instance Boxed Int where
    BoxInt :: Int -> Boxed Int

  export Boxed, BoxInt
"#,
        )],
        r#"module Main where
  import Data.Array.ND
  import Data.Machine
  import Ext

  -- Extend open data family `Ext.Boxed` in Main!
  data instance Boxed Bool where
    BoxBool :: Bool -> Boxed Bool

  -- Closed type family with ordered overlapping equations.
  type family IsInt a :: Nat where
    IsInt Int = 1
    IsInt _ = 0

  -- Verify `Type`-returning open type family across modules.
  id-accum-f32 :: AccumType F32 -> F32
  id-accum-f32 x = x

  id-accum-u8 :: AccumType U8 -> U32
  id-accum-u8 x = x

  id-accum-int :: AccumType Int -> Int
  id-accum-int x = x

  -- Verify `Nat`-returning closed type family + `using` proof discharge!
  check-rank :: using (Rank (Shape3 2 3 4) == 3, Size (Shape3 2 3 4) == 24, LastDim (Shape3 2 3 4) == 4, IsInt Int == 1, IsInt Bool == 0) => Int -> Int
  check-rank x = x

  -- Verify cross-module open data family pattern matching + refutation!
  unbox-int :: Boxed Int -> Int
  unbox-int b = case b of
    BoxInt n -> n

  unbox-bool :: Boxed Bool -> Bool
  unbox-bool b = case b of
    BoxBool flag -> flag

  main :: Int
  main =
    let a = id-accum-int 42
        b = id-accum-u8 7
        c = id-accum-f32 1.5
        d = unbox-int (BoxInt 10)
        e = unbox-bool (BoxBool True)
    in if a == 42 && u32-to-i64 b == 7 && saturating-f32-to-i64 c == 1 && d == 10 && e
       then check-rank 0
       else 1
"#,
    );
}

#[test]
fn type_and_data_family_errors_are_diagnosed() {
    // 1. Adding `type instance` to a closed type family is rejected.
    expect_build_error(
        "tf_err_closed_inst",
        &[],
        r#"module Main where
  import Data.Array.ND
  type instance InitShape Int = Int
  main :: Int
  main = 0
"#,
        &["closed type family"],
    );

    // 2. Overlapping `type instance` for an open type family is rejected.
    expect_build_error(
        "tf_err_overlap_type",
        &[],
        r#"module Main where
  import Data.Array.ND
  type instance AccumType F32 = F64
  main :: Int
  main = 0
"#,
        &["overlapping"],
    );

    // 3. Overlapping `data instance` for an open data family is rejected.
    expect_build_error(
        "tf_err_overlap_data",
        &[],
        r#"module Main where
  import Data.Array.ND
  data instance ShapeWitness Z where
    SWZ2 :: ShapeWitness Z
  main :: Int
  main = 0
"#,
        &["overlapping"],
    );

    // 4. Impossible arm on a data family is rejected (SWZ cannot build ShapeWitness (Dim 3 Z)).
    expect_build_error(
        "df_err_impossible_arm",
        &[],
        r#"module Main where
  import Data.Array.ND
  bad :: ShapeWitness (Shape1 3) -> Nat
  bad w = case w of
    SWZ -> 0
    SWDim d _ -> d
  main :: Int
  main = 0
"#,
        &["impossible arm"],
    );

    // 5. Using a Nat-returning type family in a Type position is rejected.
    expect_build_error(
        "tf_err_nat_in_type_pos",
        &[],
        r#"module Main where
  import Data.Array.ND
  bad :: Rank (Shape1 3) -> Int
  bad x = 0
  main :: Int
  main = 0
"#,
        &["index value"],
    );

    // 6. Non-terminating type family terminates safely via fuel bound (does not hang).
    expect_build_error(
        "tf_err_nonterm_fuel",
        &[],
        r#"module Main where
  type family Loop a where
    Loop a = Loop a
  bad :: Loop Int -> Int
  bad x = x
  main :: Int
  main = 0
"#,
        &["mismatch"],
    );
}

#[test]
fn nd_array_static_shape_and_reshape_checks_reject_mismatches() {
    // 1. Statically invalid reshape: 2x3x4 (size 24) -> 5x5 (size 25) rejected at compile time!
    expect_build_error(
        "nd_err_bad_reshape",
        &[],
        r#"module Main where
  import Data.Array.ND
  bad :: NDView (Shape3 2 3 4) F32 -> Option (NDView (Shape2 5 5) F32)
  bad v = nd-reshape v (SWDim 5 (SWDim 5 SWZ))
  main :: Int
  main = 0
"#,
        &["24", "25"],
    );

    // 2. Statically mismatched N-D shapes in dot product: 2x3 vs 3x2 rejected at compile time!
    expect_build_error(
        "nd_err_bad_dot_shape",
        &[],
        r#"module Main where
  import Data.Array.Compute
  import Data.Array.ND
  bad :: NDView (Shape2 2 3) F32 -> NDView (Shape2 3 2) F32 -> F32
  bad a b = nd-dot-f32 a b FloatExactTree
  main :: Int
  main = 0
"#,
        &["mismatch"],
    );
}

#[test]
fn nd_array_rank_polymorphic_0d_1d_2d_3d_4d_end_to_end() {
    run_cinch(
        "nd_e2e_01234d",
        &[],
        r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Tensor
  import Data.Array.Compute
  import Data.Array.ND
  import Data.Machine

  -- One single function that works over ANY rank N (0D, 1D, 2D, 3D, 4D, ...)!
  sum-of-squares :: forall sh. NDView sh F32 -> F32
  sum-of-squares v = nd-dot-f32 v v FloatExactTree

  main :: Int
  main =
    -- Build a 3D array (2 x 3 x 4 = 24 elements): values 1.0, 2.0, ..., 24.0.
    let w3 = SWDim 2 (SWDim 3 (SWDim 4 SWZ))
        v3 = nd-tabulate-f32 w3 (\k -> if k + 1 <= 1000000 then i64-to-f32 (nat-to-i64 (k + 1)) else 0.0)
    in if nd-rank v3 /= 3 || nd-size v3 /= 24 then 1
       else
         -- 1. Zero-copy statically shape-checked reshapes across 3D -> 2D -> 4D -> 1D!
         --    Size (Shape3 2 3 4) == 24 == Size (Shape2 6 4) == Size (Shape4 2 2 3 2) == Size (Shape1 24).
         let w2 = SWDim 6 (SWDim 4 SWZ)
             w4 = SWDim 2 (SWDim 2 (SWDim 3 (SWDim 2 SWZ)))
             w1 = SWDim 24 SWZ
         in case nd-reshape v3 w2 of
           None -> 2
           Some v2 -> case nd-reshape v2 w4 of
             None -> 3
             Some v4 -> case nd-reshape v4 w1 of
               None -> 4
               Some v1 ->
                 -- 2. Rank-polymorphic `sum-of-squares` agrees identically across 1D, 2D, 3D, 4D!
                 --    Sum_{k=1..24} k^2 = 24*25*49/6 = 4900.
                 let s1 = saturating-f32-to-i64 (sum-of-squares v1)
                     s2 = saturating-f32-to-i64 (sum-of-squares v2)
                     s3 = saturating-f32-to-i64 (sum-of-squares v3)
                     s4 = saturating-f32-to-i64 (sum-of-squares v4)
                 in if s1 /= 4900 || s2 /= 4900 || s3 /= 4900 || s4 /= 4900 then 5
                    else
                      -- 3. Rank-polymorphic `nd-reduce-last-f32` reduces innermost axis at EVERY rank!
                      --    4D (2x2x3x2) -> 3D (2x2x3) -> 2D (2x2) -> 1D (2) -> 0D (Z)!
                      let r3 = nd-reduce-last-f32 v4 FloatExactTree
                          r2 = nd-reduce-last-f32 r3 FloatExactTree
                          r1 = nd-reduce-last-f32 r2 FloatExactTree
                          r0 = nd-reduce-last-f32 r1 FloatExactTree
                      in if nd-rank r3 /= 3 || nd-rank r2 /= 2 || nd-rank r1 /= 1 || nd-rank r0 /= 0 then 6
                         else
                           -- 0D scalar `r0 :: NDView Z F32` indexed with `CoordZ`!
                           -- Sum_{k=1..24} k = 300.
                           case nd-get r0 CoordZ of
                             None -> 7
                             Some total ->
                               if saturating-f32-to-i64 total /= 300 then 8
                               else
                                 -- 4. Zero-copy N-D transpose (`nd-transpose`: 2x3x4 -> 3x2x4, `nd-transpose-all`: 2x3x4 -> 4x3x2)
                                 --    and bounds-checked `NDCoord` indexing!
                                 --    In v3 (2x3x4), element (1, 2, 3) is at linear index 1*12 + 2*4 + 3 = 23 -> value 24.0.
                                 --    In tr2 = nd-transpose v3 (3x2x4), element (2, 1, 3) must be 24.0!
                                 --    In trAll = nd-transpose-all v3 (4x3x2), element (3, 2, 1) must be 24.0!
                                 let tr2 = nd-transpose v3
                                     tr-all = nd-transpose-all v3
                                 in if nd-is-contiguous tr2 || nd-is-contiguous tr-all then 9
                                    else case decide (2 < 3) of
                                      No _ -> 10
                                      Yes b23 -> case decide (1 < 2) of
                                        No _ -> 11
                                        Yes b12 -> case decide (3 < 4) of
                                          No _ -> 12
                                          Yes b34 ->
                                            let c-tr2 = CoordCons (Fin 2 b23) (CoordCons (Fin 1 b12) (CoordCons (Fin 3 b34) CoordZ))
                                                c-all = CoordCons (Fin 3 b34) (CoordCons (Fin 2 b23) (CoordCons (Fin 1 b12) CoordZ))
                                            in case nd-get tr2 c-tr2 of
                                              None -> 13
                                              Some x1 -> case nd-get tr-all c-all of
                                                None -> 14
                                                Some x2 ->
                                                  if saturating-f32-to-i64 x1 /= 24 || saturating-f32-to-i64 x2 /= 24 then 15
                                                  else
                                                    -- 5. Zero-copy `nd-slice-outer` drops rank 3 -> 2 (slice frame 1 of 2x3x4 -> 3x4)
                                                    --    Element (2, 3) of slice 1 must be 24.0!
                                                    let sl = nd-slice-outer v3 (Fin 1 b12)
                                                        c-sl = CoordCons (Fin 2 b23) (CoordCons (Fin 3 b34) CoordZ)
                                                    in case nd-get sl c-sl of
                                                      None -> 16
                                                      Some x3 ->
                                                        if nd-rank sl == 2 && saturating-f32-to-i64 x3 == 24 then 0 else 17
"#,
    );
}

#[test]
fn nd_array_tensor_boundary_stencil_and_histogram_across_ranks() {
    run_cinch(
        "nd_tensor_stencil_hist",
        &[],
        r#"module Main where
  import Data.Array
  import Data.Array.Packed
  import Data.Tensor
  import Data.Array.Compute
  import Data.Array.ND
  import Data.Machine

  main :: Int
  main =
    -- Build a 4D Tensor (2 cameras x 2 frames x 3 height x 3 width = 36 pixels)
    -- and validate dynamically into `NDView (Shape4 2 2 3 3) U8` via `nd-from-tensor`.
    let raw = array-tabulate-u8 36 (\k -> if k + 10 <= 1000000 then wrapping-i64-to-u8 (nat-to-i64 (k + 10)) else 0)
        dims = array-from-list [2, 2, 3, 3]
        w4 = SWDim 2 (SWDim 2 (SWDim 3 (SWDim 3 SWZ)))
        batch2 = SWDim 2 (SWDim 2 SWZ)
        id-weights = array-from-i32s [0, 0, 0, 0, 1, 0, 0, 0, 0]
    in case tensor-reshape dims raw of
      None -> 1
      Some t ->
        -- Wrong rank witness (3D) must fail runtime validation cleanly!
        let bad-w3 = SWDim 4 (SWDim 3 (SWDim 3 SWZ))
        in if is-some (nd-from-tensor bad-w3 t) then 2
           else case nd-from-tensor w4 t of
             None -> 3
             Some v4 ->
               -- Apply rank-polymorphic 2D stencil over 4D batch (`AppendShape (Shape2 2 2) (Shape2 3 3)`)!
               case nd-stencil2d-batch-u8 batch2 v4 id-weights 1 EdgeClamp 2 of
                 None -> 4
                 Some blurred ->
                   -- Apply rank-polymorphic histogram over the innermost axis (`ReplaceLast (Shape4 2 2 3 3) 64` -> `2 x 2 x 3 x 64`)!
                   case nd-histogram-last-u8 blurred 64 of
                     None -> 5
                     Some hist4 ->
                       -- Rank stays 4, size becomes 2*2*3*64 = 768!
                       -- Row (0, 0, 0) had pixels [10, 11, 12], so bin 10 has count 1!
                       if nd-rank hist4 /= 4 || nd-size hist4 /= 768 then 6
                       else case decide (0 < 2) of
                         No _ -> 7
                         Yes b02 -> case decide (0 < 3) of
                           No _ -> 8
                           Yes b03 -> case decide (10 < 64) of
                             No _ -> 9
                             Yes b10 ->
                               let c = CoordCons (Fin 0 b02) (CoordCons (Fin 0 b02) (CoordCons (Fin 0 b03) (CoordCons (Fin 10 b10) CoordZ)))
                               in case nd-get hist4 c of
                                 None -> 10
                                 Some cnt -> if u32-to-i64 cnt == 1 then 0 else 11
"#,
    );
}
