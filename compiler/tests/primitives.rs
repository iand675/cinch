//! Machine primitive conformance tests: the fixed-width scalars, the
//! checked/wrapping/saturating operation families, IEEE floating point,
//! explicit conversions and literal range checking
//! (`specifications/primitive-types-specification.md`).

use cinch::driver::{check_sources, run_source};

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
    if wrapping-add-u8 255 1 == 0 then
      if wrapping-add-i8 127 1 == -128 then
        if wrapping-sub-u8 0 1 == 255 then
          if wrapping-mul-u8 16 16 == 0 then
            if wrapping-neg-u8 1 == 255 then
              if wrapping-add-u32 4294967295 1 == 0 then
                if wrapping-add-i8 127 2 == -127 then 0 else 8
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
    case checked-add-u8 255 1 of
      None ->
        case checked-add-u8 254 1 of
          Some v ->
            if v == 255 then
              case checked-sub-u8 0 1 of
                None ->
                  case checked-mul-i8 127 2 of
                    None ->
                      case checked-neg-i8 min-bound-i8 of
                        None ->
                          case checked-sub-i8 min-bound-i8 1 of
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
    if saturating-add-u8 255 1 == 255 then
      if saturating-sub-u8 0 1 == 0 then
        if saturating-mul-i8 127 2 == 127 then
          if saturating-add-i8 127 1 == 127 then
            if saturating-sub-i8 min-bound-i8 1 == min-bound-i8 then
              if saturating-neg-i8 min-bound-i8 == 127 then
                if saturating-mul-u8 255 2 == 255 then 0 else 8
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
    case checked-div-i32 min-bound-i32 (-1) of
      None ->
        case checked-rem-i32 min-bound-i32 (-1) of
          None ->
            case checked-div-u8 7 0 of
              None ->
                case checked-rem-u8 7 0 of
                  None ->
                    case checked-div-u8 7 2 of
                      Some q ->
                        if q == 3 then
                          case checked-rem-i8 (-7) 2 of
                            Some r ->
                              if r == -1 then
                                case checked-div-i8 (-7) 2 of
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
    case checked-shl-u8 1 8 of
      None ->
        case checked-shl-i64 1 64 of
          None ->
            if wrapping-shl-u8 1 9 == 2 then
              if wrapping-shl-u8 1 8 == 1 then
                if wrapping-shr-u8 128 9 == 64 then
                  if wrapping-shr-u8 255 4 == 15 then
                    case checked-shr-i8 (-4) 1 of
                      Some s ->
                        if s == -2 then
                          case checked-shl-u8 1 7 of
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
    if rotate-left-u8 129 1 == 3 then
      if rotate-right-u8 129 1 == 192 then
        if rotate-left-u8 129 9 == 3 then
          if rotate-right-u8 129 8 == 129 then
            if rotate-left-u32 1 31 == 2147483648 then 0 else 5
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
    if bit-and-u8 240 60 == 48 then
      if bit-or-u8 240 15 == 255 then
        if bit-xor-u8 255 15 == 240 then
          if bit-not-u8 0 == 255 then
            if bit-not-i8 0 == -1 then
              if count-ones-u8 255 == 8 then
                if count-ones-u32 255 == 8 then
                  if leading-zeros-u8 1 == 7 then
                    if leading-zeros-u8 0 == 8 then
                      if trailing-zeros-u8 8 == 3 then
                        if reverse-bits-u8 1 == 128 then
                          if swap-bytes-u16 4660 == 13330 then
                            if swap-bytes-u8 7 == 7 then 0 else 14
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
    if min-bound-u8 == 0 && max-bound-u8 == 255 then
      if min-bound-i8 == -128 && max-bound-i8 == 127 then
        if max-bound-u32 == 4294967295 then
          if wrapping-add-i64 max-bound-i64 1 == min-bound-i64 then
            if min-bound-u64 == 0 then 0 else 5
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
    let nan = div-f64 0 0 in
    let inf = div-f64 1 0 in
    if feq-f64 nan nan then 1 else
    if not (fne-f64 nan nan) then 2 else
    if flt-f64 nan nan then 3 else
    if fle-f64 nan nan then 4 else
    if not (is-na-nf64 nan) then 5 else
    if not (is-infinite-f64 inf) then 6 else
    if is-infinite-f64 1 then 7 else
    if not (is-finite-f64 1) then 8 else
    if not (is-normal-f64 1) then 9 else
    if is-normal-f64 0 then 10 else
    if feq-f64 (add-f64 1 2) 3 then 0 else 11
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
    let nan = div-f64 0 0 in
    if not (sign-bit-f64 (minimum-f64 0 (neg-f64 0))) then 1 else
    if sign-bit-f64 (maximum-f64 0 (neg-f64 0)) then 2 else
    if not (is-na-nf64 (minimum-f64 nan 1)) then 3 else
    if not (is-na-nf64 (maximum-f64 1 nan)) then 4 else
    if not (feq-f64 (minimum-f64 1 2) 1) then 5 else
    if not (feq-f64 (maximum-f64 1 2) 2) then 6 else
    if not (sign-bit-f64 (neg-f64 0)) then 7 else
    if sign-bit-f64 0 then 8 else 0
"#,
        0,
    );
}

#[test]
fn total_order_orders_neg_zero_and_nan() {
    // IEEE 754-2019 totalOrder: -NaN < -inf < ... < -0 < +0 < ... < +inf
    // < +NaN. The sign of a produced NaN is unspecified, so the test
    // builds +NaN with `abs-f64` and -NaN by negating it.
    run_ok(
        "total-order",
        r#"
module Main where
  main :: Int
  main =
    let pn = abs-f64 (div-f64 0 0) in
    let nn = neg-f64 pn in
    let inf = div-f64 1 0 in
    case total-order-f64 (neg-f64 0) 0 of
      LT ->
        case total-order-f64 0 (neg-f64 0) of
          GT ->
            case total-order-f64 inf pn of
              LT ->
                case total-order-f64 pn inf of
                  GT ->
                    case total-order-f64 nn (neg-f64 inf) of
                      LT ->
                        case total-order-f64 1 1 of
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
    let nan = div-f32 0 0 in
    if feq-f32 (add-f32 1 2) 3 then
      if is-na-nf32 nan then
        if sign-bit-f32 (neg-f32 0) then
          case total-order-f32 (neg-f32 0) 0 of
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
    if char-to-u32 'a' == 97 then
      if char-to-u32 '\n' == 10 then
        if 'a' < 'b' && not ('b' < 'a') then
          case u32-to-char 65 of
            Some c ->
              if char-to-u32 c == 65 then 0 else 5
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
        "u32-to-char",
        r#"
module Main where
  main :: Int
  main =
    case u32-to-char 0xD800 of
      None ->
        case u32-to-char 0xDFFF of
          None ->
            case u32-to-char 0x110000 of
              None ->
                case u32-to-char 0x10FFFF of
                  Some c -> if char-to-u32 c == 1114111 then 0 else 5
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
    case checked-u8-to-i64 255 of
      Some v ->
        if v == 255 then
          if u8-to-i32 255 == 255 then
            case checked-i64-to-u8 256 of
              None ->
                case checked-i64-to-u8 255 of
                  Some w ->
                    if w == 255 then
                      if saturating-i64-to-u8 256 == 255 then
                        if saturating-i64-to-u8 (-1) == 0 then
                          if wrapping-i32-to-u8 256 == 0 then
                            if wrapping-i32-to-i8 128 == min-bound-i8 then
                              if u8-to-u8 7 == 7 then 0 else 10
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
  totalRound = feq-f64 (i64-to-f64 3) 3 && feq-f64 (f32-to-f64 (u8-to-f32 3)) 3

  checkedMax :: Bool
  checkedMax = case checked-f64-to-i32 (i64-to-f64 2147483647) of
    Some v -> v == 2147483647
    None -> False

  checkedBig :: Bool
  checkedBig = case checked-f64-to-i32 (i64-to-f64 2147483648) of
    None -> True
    Some _ -> False

  checkedNan :: Bool
  checkedNan = case checked-f64-to-i32 (div-f64 0 0) of
    None -> True
    Some _ -> False

  saturatingNan :: Bool
  saturatingNan = saturating-f64-to-i32 (div-f64 0 0) == 0

  saturatingBig :: Bool
  saturatingBig = saturating-f64-to-i32 (i64-to-f64 3000000000) == 2147483647

  float32Checked :: Bool
  float32Checked = case checked-f32-to-u8 (u32-to-f32 300) of
    None -> True
    Some _ -> False

  float32Saturating :: Bool
  float32Saturating = saturating-f32-to-u8 (u32-to-f32 300) == 255

  alwaysSome :: Bool
  alwaysSome = case checked-u8-to-i64 255 of
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
    if (-128 :: I8) == min-bound-i8 then
      if (255 :: U8) == max-bound-u8 then
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
      if (-2 :: I8) < (1 :: I8) && min-bound-i8 < (0 :: I8) then
        if eq-usize min-bound-usize 0 && eq-u64 max-bound-u64 (wrapping-sub-u64 0 1) then
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
    let dir = std::env::temp_dir().join(format!("cinch-prim-smoke-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create scratch dir");
    let file = dir.join("smoke.cinch");
    std::fs::write(
        &file,
        r#"
module Main where
  main :: Int
  main =
    let scaled = saturating-mul-u8 (wrapping-add-u8 200 100) 12 in
    if scaled == 255 then
      case checked-div-i32 (-7) 2 of
        Some q ->
          if q == -3 then
            if feq-f64 (mul-f64 (i64-to-f64 6) (i64-to-f64 7)) 42 then
              case u32-to-char 97 of
                Some c -> if char-to-u32 c == 97 then 23 else 3
                None -> 2
            else 1
          else 4
        None -> 5
    else 6
"#,
    )
    .expect("write smoke source");
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("run")
        .arg(&file)
        .status()
        .expect("run the cinch CLI");
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
    // The inductive `Nat` subtraction is truncated: `nat-sub` faults (exit
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
      if int-from-i64 (nat-to-i64 (nat-add 2 3)) == 5 then
        if int-to-i64 7 == 7 then
          if nat-from-i64 9 == 9 then 0 else 4
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
  main = let x = int-div 7 0 in 0
"#,
        72,
    );
    run_ok(
        "nat-rem-zero",
        r#"
module Main where
  main :: Int
  main = let x = nat-rem 7 0 in 0
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
    let b = bytes-new 4 in
    let u = bytes-set b 0 65 in
    let u2 = bytes-set b 3 66 in
    if bytes-len b == 4 then
      if bytes-get b 0 == 65 then
        if bytes-get b 3 == 66 then
          if bytes-get b 1 == 0 then 0 else 4
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
  main = let b = bytes-new 2 in let x = bytes-get b 2 in 0
"#,
        71,
    );
    run_ok(
        "bytes-set-oob",
        r#"
module Main where
  main :: Int
  main = let b = bytes-new 2 in let u = bytes-set b 2 1 in 0
"#,
        71,
    );
    run_ok(
        "bytes-set-bad-value",
        r#"
module Main where
  main :: Int
  main = let b = bytes-new 2 in let u = bytes-set b 0 300 in 0
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
  foreign import ccall "cinch.prim.addInt" f :: Int -> Int -> Int
  main :: Int
  main = 0
"#,
        "not a cinch intrinsic",
    );
    expect_err(
        "retired-eqInt",
        r#"
module Main where
  foreign import ccall "cinch.prim.eqInt" f :: Int -> Int -> Bool
  main :: Int
  main = 0
"#,
        "not a cinch intrinsic",
    );
    // The machine case is the P2 family; the mathematical case is the
    // runtime family.
    run_ok(
        "replacements-exist",
        r#"
module Main where
  main :: Int
  main =
    if eq-i64 (int-cmp 3 3) 0 then
      if eq-i64 (nat-cmp 3 4) 0 then 2 else 0
    else 1
"#,
        0,
    );
}
