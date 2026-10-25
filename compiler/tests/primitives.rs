//! Machine primitive conformance tests: the fixed-width scalars, the
//! checked/wrapping/saturating operation families, IEEE floating point,
//! explicit conversions and literal range checking
//! (`specifications/primitive-types-specification.md`).

use safelang::driver::{check_sources, run_source};

/// Compile, run and assert the exit code.
fn run_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Assert that checking the program fails with a message containing
/// `needle`.
fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected a compilation error containing {needle:?}"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

// ------------------------------------------------------------ section 4: integer families

#[test]
fn sub_word_canonicalization() {
    // Every result is canonicalized to the type's word form: sub-word
    // values wrap at their own width.
    run_ok(
        "sub-word",
        r#"
module Main where
  main :: Int
  main =
    if wrappingAddU8 255 1 == 0 then
      if wrappingAddI8 127 1 == -128 then
        if wrappingSubU8 0 1 == 255 then
          if wrappingMulU8 16 16 == 0 then
            if wrappingNegU8 1 == 255 then
              if wrappingAddU32 4294967295 1 == 0 then
                if wrappingAddI8 127 2 == -127 then 0 else 8
              else 7
            else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn checked_overflow_returns_none() {
    run_ok(
        "checked",
        r#"
module Main where
  main :: Int
  main =
    case checkedAddU8 255 1 of
      None ->
        case checkedAddU8 254 1 of
          Some v ->
            if v == 255 then
              case checkedSubU8 0 1 of
                None ->
                  case checkedMulI8 127 2 of
                    None ->
                      case checkedNegI8 minBoundI8 of
                        None ->
                          case checkedSubI8 minBoundI8 1 of
                            None -> 0
                            Some _ -> 9
                        Some _ -> 8
                    Some _ -> 7
                Some _ -> 6
            else 5
          None -> 4
      Some _ -> 3
"#,
        0,
    );
}

#[test]
fn saturating_clamps_to_bounds() {
    run_ok(
        "saturating",
        r#"
module Main where
  main :: Int
  main =
    if saturatingAddU8 255 1 == 255 then
      if saturatingSubU8 0 1 == 0 then
        if saturatingMulI8 127 2 == 127 then
          if saturatingAddI8 127 1 == 127 then
            if saturatingSubI8 minBoundI8 1 == minBoundI8 then
              if saturatingNegI8 minBoundI8 == 127 then
                if saturatingMulU8 255 2 == 255 then 0 else 8
              else 7
            else 6
          else 5
        else 4
      else 3
    else 1
"#,
        0,
    );
}

#[test]
fn checked_div_and_rem() {
    // `None` on a zero divisor and on `minBound / -1`; quotients truncate
    // toward zero and `x = div x y * y + rem x y`.
    run_ok(
        "div-rem",
        r#"
module Main where
  main :: Int
  main =
    case checkedDivI32 minBoundI32 (-1) of
      None ->
        case checkedRemI32 minBoundI32 (-1) of
          None ->
            case checkedDivU8 7 0 of
              None ->
                case checkedRemU8 7 0 of
                  None ->
                    case checkedDivU8 7 2 of
                      Some q ->
                        if q == 3 then
                          case checkedRemI8 (-7) 2 of
                            Some r ->
                              if r == -1 then
                                case checkedDivI8 (-7) 2 of
                                  Some d -> if d == -3 then 0 else 10
                                  None -> 9
                              else 8
                            None -> 7
                        else 6
                      None -> 5
                  Some _ -> 4
              Some _ -> 3
          Some _ -> 2
      Some _ -> 1
"#,
        0,
    );
}

#[test]
fn shifts_distinguish_invalid_counts() {
    // `checked*` return `None` for counts at or above the width; the
    // `wrapping*` families reduce the count modulo the width. `shr` is
    // arithmetic on signed types and logical on unsigned ones.
    run_ok(
        "shifts",
        r#"
module Main where
  main :: Int
  main =
    case checkedShlU8 1 8 of
      None ->
        case checkedShlI64 1 64 of
          None ->
            if wrappingShlU8 1 9 == 2 then
              if wrappingShlU8 1 8 == 1 then
                if wrappingShrU8 128 9 == 64 then
                  if wrappingShrU8 255 4 == 15 then
                    case checkedShrI8 (-4) 1 of
                      Some s ->
                        if s == -2 then
                          case checkedShlU8 1 7 of
                            Some t -> if t == 128 then 0 else 10
                            None -> 9
                        else 8
                      None -> 7
                  else 6
                else 5
              else 4
            else 3
          Some _ -> 2
      Some _ -> 1
"#,
        0,
    );
}

#[test]
fn rotates_are_total() {
    run_ok(
        "rotates",
        r#"
module Main where
  main :: Int
  main =
    if rotateLeftU8 129 1 == 3 then
      if rotateRightU8 129 1 == 192 then
        if rotateLeftU8 129 9 == 3 then
          if rotateRightU8 129 8 == 129 then
            if rotateLeftU32 1 31 == 2147483648 then 0 else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn bit_operations_and_inspection() {
    run_ok(
        "bits",
        r#"
module Main where
  main :: Int
  main =
    if bitAndU8 240 60 == 48 then
      if bitOrU8 240 15 == 255 then
        if bitXorU8 255 15 == 240 then
          if bitNotU8 0 == 255 then
            if bitNotI8 0 == -1 then
              if countOnesU8 255 == 8 then
                if countOnesU32 255 == 8 then
                  if leadingZerosU8 1 == 7 then
                    if leadingZerosU8 0 == 8 then
                      if trailingZerosU8 8 == 3 then
                        if reverseBitsU8 1 == 128 then
                          if swapBytesU16 4660 == 13330 then
                            if swapBytesU8 7 == 7 then 0 else 14
                          else 13
                        else 12
                      else 11
                    else 10
                  else 9
                else 8
              else 7
            else 6
          else 5
        else 4
      else 3
    else 1
"#,
        0,
    );
}

#[test]
fn min_and_max_bounds() {
    run_ok(
        "bounds",
        r#"
module Main where
  main :: Int
  main =
    if minBoundU8 == 0 && maxBoundU8 == 255 then
      if minBoundI8 == -128 && maxBoundI8 == 127 then
        if maxBoundU32 == 4294967295 then
          if wrappingAddI64 maxBoundI64 1 == minBoundI64 then
            if minBoundU64 == 0 then 0 else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ sections 5 and 8: floats

#[test]
fn float_nan_behavior() {
    // Floats have no Eq instance: comparisons go through the quiet IEEE
    // operations, and `feq` is false on NaN.
    run_ok(
        "nan",
        r#"
module Main where
  main :: Int
  main =
    let nan = divF64 0 0 in
    let inf = divF64 1 0 in
    if feqF64 nan nan then 1 else
    if not (fneF64 nan nan) then 2 else
    if fltF64 nan nan then 3 else
    if fleF64 nan nan then 4 else
    if not (isNaNF64 nan) then 5 else
    if not (isInfiniteF64 inf) then 6 else
    if isInfiniteF64 1 then 7 else
    if not (isFiniteF64 1) then 8 else
    if not (isNormalF64 1) then 9 else
    if isNormalF64 0 then 10 else
    if feqF64 (addF64 1 2) 3 then 0 else 11
"#,
        0,
    );
}

#[test]
fn minimum_maximum_propagate_nan() {
    // IEEE 754-2019 minimum/maximum: NaN propagates and -0 sorts below +0.
    run_ok(
        "minmax",
        r#"
module Main where
  main :: Int
  main =
    let nan = divF64 0 0 in
    if not (signBitF64 (minimumF64 0 (negF64 0))) then 1 else
    if signBitF64 (maximumF64 0 (negF64 0)) then 2 else
    if not (isNaNF64 (minimumF64 nan 1)) then 3 else
    if not (isNaNF64 (maximumF64 1 nan)) then 4 else
    if not (feqF64 (minimumF64 1 2) 1) then 5 else
    if not (feqF64 (maximumF64 1 2) 2) then 6 else
    if not (signBitF64 (negF64 0)) then 7 else
    if signBitF64 0 then 8 else 0
"#,
        0,
    );
}

#[test]
fn total_order_orders_neg_zero_and_nan() {
    // IEEE 754-2019 totalOrder: -NaN < -inf < ... < -0 < +0 < ... < +inf
    // < +NaN. The sign of a produced NaN is unspecified, so the test
    // builds +NaN with `absF64` and -NaN by negating it.
    run_ok(
        "total-order",
        r#"
module Main where
  main :: Int
  main =
    let pn = absF64 (divF64 0 0) in
    let nn = negF64 pn in
    let inf = divF64 1 0 in
    case totalOrderF64 (negF64 0) 0 of
      LT ->
        case totalOrderF64 0 (negF64 0) of
          GT ->
            case totalOrderF64 inf pn of
              LT ->
                case totalOrderF64 pn inf of
                  GT ->
                    case totalOrderF64 nn (negF64 inf) of
                      LT ->
                        case totalOrderF64 1 1 of
                          EQ -> 0
                          _ -> 6
                      _ -> 5
                  _ -> 4
              _ -> 3
          _ -> 2
      _ -> 1
"#,
        0,
    );
}

#[test]
fn f32_families_match_f64_shape() {
    run_ok(
        "f32",
        r#"
module Main where
  main :: Int
  main =
    let nan = divF32 0 0 in
    if feqF32 (addF32 1 2) 3 then
      if isNaNF32 nan then
        if signBitF32 (negF32 0) then
          case totalOrderF32 (negF32 0) 0 of
            LT -> 0
            _ -> 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ section 6: Char

#[test]
fn char_values_and_ordering() {
    run_ok(
        "char",
        r#"
module Main where
  main :: Int
  main =
    if charToU32 'a' == 97 then
      if charToU32 '\n' == 10 then
        if 'a' < 'b' && not ('b' < 'a') then
          case u32ToChar 65 of
            Some c ->
              if charToU32 c == 65 then 0 else 5
            None -> 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn u32_to_char_rejects_non_scalars() {
    run_ok(
        "u32ToChar",
        r#"
module Main where
  main :: Int
  main =
    case u32ToChar 0xD800 of
      None ->
        case u32ToChar 0xDFFF of
          None ->
            case u32ToChar 0x110000 of
              None ->
                case u32ToChar 0x10FFFF of
                  Some c -> if charToU32 c == 1114111 then 0 else 5
                  None -> 4
              Some _ -> 3
          Some _ -> 2
      Some _ -> 1
"#,
        0,
    );
}

// ------------------------------------------------------------ section 7: conversions

#[test]
fn conversions_at_range_boundaries() {
    run_ok(
        "conversions",
        r#"
module Main where
  main :: Int
  main =
    case checkedU8ToI64 255 of
      Some v ->
        if v == 255 then
          if u8ToI32 255 == 255 then
            case checkedI64ToU8 256 of
              None ->
                case checkedI64ToU8 255 of
                  Some w ->
                    if w == 255 then
                      if saturatingI64ToU8 256 == 255 then
                        if saturatingI64ToU8 (-1) == 0 then
                          if wrappingI32ToU8 256 == 0 then
                            if wrappingI32ToI8 128 == minBoundI8 then
                              if u8ToU8 7 == 7 then 0 else 10
                            else 9
                          else 8
                        else 7
                      else 6
                    else 5
                  None -> 4
              Some _ -> 3
          else 2
        else 1
      None -> 11
"#,
        0,
    );
}

#[test]
fn float_conversions() {
    // Float-to-integer is never total: `checked` returns None on NaN and
    // out of range, `saturating` clamps and maps NaN to zero. The total
    // directions round per IEEE 754.
    run_ok(
        "float-conversions",
        r#"
module Main where
  totalRound :: Bool
  totalRound = feqF64 (i64ToF64 3) 3 && feqF64 (f32ToF64 (u8ToF32 3)) 3

  checkedMax :: Bool
  checkedMax = case checkedF64ToI32 (i64ToF64 2147483647) of
    Some v -> v == 2147483647
    None -> False

  checkedBig :: Bool
  checkedBig = case checkedF64ToI32 (i64ToF64 2147483648) of
    None -> True
    Some _ -> False

  checkedNan :: Bool
  checkedNan = case checkedF64ToI32 (divF64 0 0) of
    None -> True
    Some _ -> False

  saturatingNan :: Bool
  saturatingNan = saturatingF64ToI32 (divF64 0 0) == 0

  saturatingBig :: Bool
  saturatingBig = saturatingF64ToI32 (i64ToF64 3000000000) == 2147483647

  float32Checked :: Bool
  float32Checked = case checkedF32ToU8 (u32ToF32 300) of
    None -> True
    Some _ -> False

  float32Saturating :: Bool
  float32Saturating = saturatingF32ToU8 (u32ToF32 300) == 255

  alwaysSome :: Bool
  alwaysSome = case checkedU8ToI64 255 of
    Some v -> v == 255
    None -> False

  main :: Int
  main =
    if totalRound then
      if checkedMax then
        if checkedBig then
          if checkedNan then
            if saturatingNan then
              if saturatingBig then
                if float32Checked then
                  if float32Saturating then
                    if alwaysSome then 0 else 9
                  else 8
                else 7
              else 6
            else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ literals

#[test]
fn literal_range_checking() {
    expect_err(
        "lit-300-u8",
        r#"
module Main where
  main :: Int
  main = let x = (300 :: U8) in 0
"#,
        "integer literal 300 is out of range for U8",
    );
    expect_err(
        "lit-neg-u8",
        r#"
module Main where
  main :: Int
  main = let x = (-1 :: U8) in 0
"#,
        "integer literal -1 is out of range for U8",
    );
    expect_err(
        "lit-128-i8",
        r#"
module Main where
  main :: Int
  main = let x = (128 :: I8) in 0
"#,
        "integer literal 128 is out of range for I8",
    );
    expect_err(
        "lit-hex-u8",
        r#"
module Main where
  main :: Int
  main = let x = (0x100 :: U8) in 0
"#,
        "out of range for U8",
    );
    // The negative literal folds into the literal and is range-checked
    // with it: `-128 : I8` is valid, and `255 : U8` survives a wrap.
    run_ok(
        "lit-in-range",
        r#"
module Main where
  main :: Int
  main =
    if (-128 :: I8) == minBoundI8 then
      if (255 :: U8) == maxBoundU8 then
        if (0x7F :: I8) == 127 then 0 else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn unconstrained_literal_is_an_error() {
    expect_err(
        "lit-undetermined",
        r#"
module Main where
  main :: Int
  main = let x = identity 5 in 0
"#,
        "not determined by context",
    );
}

#[test]
fn literal_patterns_are_range_checked() {
    expect_err(
        "pat-lit-range",
        r#"
module Main where
  f :: U8 -> Int
  f x = case x of
    300 -> 0
    n -> 1

  main :: Int
  main = f 0
"#,
        "integer literal 300 is out of range for U8",
    );
    expect_err(
        "pat-lit-float",
        r#"
module Main where
  f :: F64 -> Int
  f x = case x of
    1 -> 0
    n -> 1

  main :: Int
  main = f 0
"#,
        "floats have no Eq instance",
    );
    run_ok(
        "pat-lit-scalar",
        r#"
module Main where
  f :: U8 -> Int
  f x = case x of
    255 -> 1
    0 -> 2
    n -> 3

  main :: Int
  main =
    if f 255 == 1 then
      if f 0 == 2 then
        if f 7 == 3 then 0 else 4
      else 3
    else 2
"#,
        0,
    );
}

#[test]
fn char_literal_rejects_non_scalars() {
    expect_err(
        "char-lit-surrogate",
        r#"
module Main where
  main :: Int
  main = let x = ('\u{D800}' :: Char) in 0
"#,
        "Unicode scalar value",
    );
}

// ------------------------------------------------------------ required rejections (section 10)

#[test]
fn rejects_arith_on_machine_integers() {
    expect_err(
        "no-add-i32",
        r#"
module Main where
  f :: I32 -> I32 -> I32
  f x y = x + y

  main :: Int
  main = 0
"#,
        "no instance for Add I32",
    );
}

#[test]
fn rejects_eq_and_ord_on_floats() {
    expect_err(
        "no-eq-f64",
        r#"
module Main where
  f :: F64 -> F64 -> Bool
  f x y = x == y

  main :: Int
  main = 0
"#,
        "no instance for Eq F64",
    );
    expect_err(
        "no-ord-f32",
        r#"
module Main where
  f :: F32 -> F32 -> Bool
  f x y = x < y

  main :: Int
  main = 0
"#,
        "no instance for Ord F32",
    );
}

#[test]
fn rejects_neg_on_non_literal_machine_numbers() {
    expect_err(
        "no-neg-u8",
        r#"
module Main where
  f :: U8 -> U8
  f x = -x

  main :: Int
  main = 0
"#,
        "no instance for Neg U8",
    );
}

#[test]
fn eq_ord_instances_for_machine_types() {
    // Eq and Ord exist for every integer type, Usize, Isize and Char
    // (section 8), with signed numeric order on the signed types and code
    // point order on Char.
    run_ok(
        "instances",
        r#"
module Main where
  main :: Int
  main =
    if (3 :: U8) == 3 && (255 :: U8) > (0 :: U8) then
      if (-2 :: I8) < (1 :: I8) && minBoundI8 < (0 :: I8) then
        if eqUsize minBoundUsize 0 && eqU64 maxBoundU64 (wrappingSubU64 0 1) then
          if ('a' :: Char) < 'b' && not (True < False) then
            if False < True && (3 :: U64) /= (4 :: U64) then 0 else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ CLI smoke

#[test]
fn cli_smoke_several_prim_families() {
    let dir = std::env::temp_dir().join(format!("safelang-prim-smoke-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    let file = dir.join("smoke.lang");
    std::fs::write(
        &file,
        r#"
module Main where
  main :: Int
  main =
    let scaled = saturatingMulU8 (wrappingAddU8 200 100) 12 in
    if scaled == 255 then
      case checkedDivI32 (-7) 2 of
        Some q ->
          if q == -3 then
            if feqF64 (mulF64 (i64ToF64 6) (i64ToF64 7)) 42 then
              case u32ToChar 97 of
                Some c -> if charToU32 c == 97 then 23 else 3
                None -> 2
            else 1
          else 4
        None -> 5
    else 6
"#,
    )
    .expect("write smoke source");
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_safelang"))
        .arg("run")
        .arg(&file)
        .status()
        .expect("run the safelang CLI");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(status.code(), Some(23), "cli smoke: expected exit 23");
}

// ------------------------------------------------------------ mathematical Nat and Int (section 1)

#[test]
fn bignum_arithmetic_through_the_language() {
    // (10^19 + 7) - 10^19 = 7: the operands are boxed bignums.
    run_ok(
        "bignum-arith",
        r#"
module Main where
  main :: Int
  main = (10000000000000000000 + 7) - 10000000000000000000
"#,
        7,
    );
}

#[test]
fn int_beyond_i64_computed_and_compared() {
    run_ok(
        "beyond-i64",
        r#"
module Main where
  twice :: Int -> Int
  twice x = x + x

  main :: Int
  main =
    if 18446744073709551617 - 1 == 18446744073709551616 then
      if twice 18446744073709551616 == 36893488147419103232 then
        if 36893488147419103232 > 36893488147419103231 then 0 else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn nat_truncating_subtraction() {
    // The inductive `Nat` subtraction is truncated: `natSub` faults (exit
    // 71, the panic path) instead of producing a negative result.
    run_ok(
        "nat-sub",
        r#"
module Main where
  main :: Int
  main =
    if (5 :: Nat) - 3 == 2 then
      if (3 :: Nat) - 3 == 0 then 0 else 2
    else 1
"#,
        0,
    );
    run_ok(
        "nat-sub-fault",
        r#"
module Main where
  main :: Int
  main = let x = (3 :: Nat) - 5 in 0
"#,
        71,
    );
}

#[test]
fn negative_nat_literal_is_rejected() {
    expect_err(
        "neg-nat",
        r#"
module Main where
  main :: Int
  main = let x = (-3 :: Nat) in 0
"#,
        "integer literal -3 is out of range for Nat",
    );
    // A negated variable at Nat has no Neg instance either.
    expect_err(
        "neg-nat-var",
        r#"
module Main where
  f :: Nat -> Nat
  f x = -x

  main :: Int
  main = 0
"#,
        "no instance for Neg Nat",
    );
}

#[test]
fn literal_ranges_at_machine_and_mathematical_types() {
    // `300 :: U8` is still rejected; `300 :: Nat` works, and so do
    // literals beyond `i64` range at `Nat` and `Int`.
    expect_err(
        "lit-u8-range",
        r#"
module Main where
  main :: Int
  main = let x = (300 :: U8) in 0
"#,
        "integer literal 300 is out of range for U8",
    );
    run_ok(
        "lit-nat-big",
        r#"
module Main where
  main :: Int
  main =
    if (300 :: Nat) == 300 then
      if (36893488147419103233 :: Nat) == 36893488147419103232 + 1 then
        if (36893488147419103232 :: Int) - 36893488147419103231 == 1 then 0 else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn literal_patterns_at_nat_and_int_fall_through() {
    run_ok(
        "nat-lit-pats",
        r#"
module Main where
  f :: Nat -> Int
  f x = case x of
    0 -> 1
    1 -> 2
    18446744073709551616 -> 3
    n -> 4

  g :: Int -> Int
  g x = case x of
    -5 -> 1
    -6 -> 2
    n -> 3

  main :: Int
  main =
    if f 0 == 1 then
      if f 1 == 2 then
        if f 18446744073709551616 == 3 then
          if f 18446744073709551617 == 4 then
            if g (-5) == 1 then
              if g (-6) == 2 then
                if g (-7) == 3 then 0 else 8
              else 7
            else 6
          else 5
        else 4
      else 3
    else 2
"#,
        0,
    );
}

#[test]
fn int_and_nat_comparison_instances() {
    run_ok(
        "cmp-instances",
        r#"
module Main where
  main :: Int
  main =
    if 100000000000000000001 == 100000000000000000001 then
      if 100000000000000000001 /= 100000000000000000002 then
        if 100000000000000000001 < 100000000000000000002 then
          if 100000000000000000002 > 100000000000000000001 then
            if (2 :: Nat) <= (2 :: Nat) then
              if (2 :: Nat) >= (3 :: Nat) then 6 else 0
            else 4
          else 3
        else 2
      else 1
    else 5
"#,
        0,
    );
}

#[test]
fn neg_and_conversions_at_int() {
    run_ok(
        "neg-conversions",
        r#"
module Main where
  main :: Int
  main =
    if negate 5 == -5 then
      if intFromI64 (natToI64 (natAdd 2 3)) == 5 then
        if intToI64 7 == 7 then
          if natFromI64 9 == 9 then 0 else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn division_by_zero_faults() {
    run_ok(
        "int-div-zero",
        r#"
module Main where
  main :: Int
  main = let x = intDiv 7 0 in 0
"#,
        72,
    );
    run_ok(
        "nat-rem-zero",
        r#"
module Main where
  main :: Int
  main = let x = natRem 7 0 in 0
"#,
        72,
    );
}

// ------------------------------------------------------------ Bytes (section 1)

#[test]
fn bytes_construction_length_get_set() {
    run_ok(
        "bytes-round-trip",
        r#"
module Main where
  main :: Int
  main =
    let b = bytesNew 4 in
    let u = bytesSet b 0 65 in
    let u2 = bytesSet b 3 66 in
    if bytesLen b == 4 then
      if bytesGet b 0 == 65 then
        if bytesGet b 3 == 66 then
          if bytesGet b 1 == 0 then 0 else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

#[test]
fn bytes_faults_are_panic_path_71() {
    // Out-of-bounds indices, byte values outside 0..=255 and negative
    // lengths all raise `Error::OutOfBounds` and exit 71 (the panic path).
    run_ok(
        "bytes-get-oob",
        r#"
module Main where
  main :: Int
  main = let b = bytesNew 2 in let x = bytesGet b 2 in 0
"#,
        71,
    );
    run_ok(
        "bytes-set-oob",
        r#"
module Main where
  main :: Int
  main = let b = bytesNew 2 in let u = bytesSet b 2 1 in 0
"#,
        71,
    );
    run_ok(
        "bytes-set-bad-value",
        r#"
module Main where
  main :: Int
  main = let b = bytesNew 2 in let u = bytesSet b 0 300 in 0
"#,
        71,
    );
}

// ------------------------------------------------------------ retired legacy family

#[test]
fn retired_int_family_no_longer_resolves() {
    expect_err(
        "retired-addInt",
        r#"
module Main where
  foreign import ccall "safelang.prim.addInt" f :: Int -> Int -> Int
  main :: Int
  main = 0
"#,
        "not a safelang intrinsic",
    );
    expect_err(
        "retired-eqInt",
        r#"
module Main where
  foreign import ccall "safelang.prim.eqInt" f :: Int -> Int -> Bool
  main :: Int
  main = 0
"#,
        "not a safelang intrinsic",
    );
    // The machine case is the P2 family; the mathematical case is the
    // runtime family.
    run_ok(
        "replacements-exist",
        r#"
module Main where
  main :: Int
  main =
    if eqI64 (intCmp 3 3) 0 then
      if eqI64 (natCmp 3 4) 0 then 2 else 0
    else 1
"#,
        0,
    );
}
