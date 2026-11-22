mod common;

const SOURCES: &[(&str, &str)] = &[
    ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
    ("Data.Array.Packed.cinch", include_str!("../prelude/Data/Array/Packed.cinch")),
    ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
    ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
];

fn run(tag: &str, body: &str) {
    let source = format!("module Main where\n  import Data.Array\n  import Data.Array.Packed\n  import Data.Tensor\n  import Data.Columnar\n  same :: forall a. using (eq :: Eq a) => Option a -> a -> Bool\n  same o expected = case o of\n    None -> False\n    Some x -> x == expected\n{body}\n");
    let mut sources = SOURCES.to_vec();
    sources.push(("Main.cinch", &source));
    let dir = common::project(tag, &sources);
    let exe = common::build(&dir, "Main.cinch", "program").expect("build array program");
    let out = std::process::Command::new(exe).env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
    assert_eq!(out.status.code(), Some(0), "{}\nproject: {}", String::from_utf8_lossy(&out.stderr), dir.display());
    common::cleanup(&dir);
}

#[test]
fn normal_arrays_preserve_values_and_return_none_at_boundaries() {
    run("arrays_values", r#"
  values :: Array Int
  values = [10, 20, 184467440737095516160]
  main :: Int
  main = case array-set values 1 99 of
    None -> 1
    Some changed ->
      if array-length values == 3 && same (array-get values 1) 20
        && same (array-get changed 1) 99
        && same (array-get changed 2) 184467440737095516160
        && is-none (array-get values 3)
        && is-none (array-get values 9999999999999999999999999)
      then case array-slice changed 3 0 of
        None -> 2
        Some empty -> if array-length empty == 0 then 0 else 3
      else 4
"#);
}

#[test]
fn array_library_objects_stamp_matching_abi_and_owned_types() {
    let source = r#"module Library where
  import Data.Array
  import Data.Tensor
  import Data.Columnar
  export make, peek, column
  make :: Int -> Array Int
  make x = array-from-list [x]
  peek :: Tensor Int -> Array Nat -> Option Int
  peek tensor indices = tensor-get tensor indices
  column :: Array Int -> Columnar Int
  column values = columnar-column values
"#;
    let mut sources = SOURCES.to_vec();
    sources.push(("Library.cinch", source));
    let dir = common::project("arrays_object", &sources);
    let out = dir.join("library.o");
    common::drive(cinch::driver::Mode::EmitObject, &dir, "Library.cinch", Some(out.clone()))
        .expect("emit the array library object");
    assert!(out.is_file());
    let text = std::fs::read_to_string(out.with_extension("abi.json")).unwrap();
    assert!(text.contains("\"runtime_abi\":6"), "{text}");
    // Inspect the separately emitted interface through the public reader.
    let iface = cinch::interface::Iface::from_json(&std::fs::read_to_string(dir.join("build/Library.cinchif")).unwrap()).unwrap();
    assert_eq!(iface.runtime_abi, cinch::interface::RUNTIME_ABI_VERSION);
    for (name, ty) in [("make", "Array"), ("peek", "Tensor"), ("column", "Columnar")] {
        assert!(iface.exports.values.iter().any(|v| v.name == name && v.scheme.contains(ty)), "{name}: {:?}", iface.exports.values);
    }
    let ir = std::fs::read_to_string(out.with_extension("ll")).unwrap();
    for external in ["cinch_array_from_list", "cinch_tensor_get", "cinch_columnar_column"] {
        assert!(ir.contains(external), "missing {external}");
    }
    assert!(text.contains("\"result\":\"O\""), "array results must be owned objects");
    common::cleanup(&dir);
}

#[test]
fn arrays_keep_record_and_string_elements_alive() {
    run("arrays_objects", r#"
  data Item where
    Item :: String -> Int -> Item
  item-value :: Item -> Int
  item-value item = case item of
    Item name n -> if name == "kept alive" then n else 0
  make :: () -> Array Item
  make unit = array-from-list [Item "kept alive" 41, Item "kept alive" 42]
  main :: Int
  main = case array-get (array-append (make ()) (make ())) 3 of
    None -> 1
    Some item -> if item-value item == 42 then 0 else 2
"#);
}

#[test]
fn multidimensional_arrays_check_every_axis_and_shape_product() {
    run("arrays_shape", r#"
  data-words :: Array Int
  data-words = array-from-list [0, 1, 2, 3, 4, 5]
  dims :: Array Nat
  dims = array-from-list [2, 3]
  main :: Int
  main = case tensor-reshape dims data-words of
    None -> 1
    Some matrix ->
      if same (tensor-get matrix (array-from-list [1, 2])) 5
        && is-none (tensor-get matrix (array-from-list [0, 3]))
        && is-none (tensor-get matrix (array-from-list [2, 0]))
        && is-none (tensor-get matrix (array-from-list [1]))
      then case tensor-set matrix (array-from-list [1, 1]) 90 of
        None -> 2
        Some changed -> if same (tensor-get matrix (array-from-list [1, 1])) 4
          && same (tensor-get changed (array-from-list [1, 1])) 90 then 0 else 3
      else 4
"#);
}

#[test]
fn columnar_composition_checks_lengths_and_preserves_nested_row_types() {
    run("arrays_columns", r#"
  nums :: Columnar Int
  nums = columnar-column (array-from-list [7, 8])
  names :: Columnar String
  names = columnar-column (array-from-list ["seven", "eight"])
  flags :: Columnar Bool
  flags = columnar-column (array-from-list [True, False])
  main :: Int
  main = case columnar-zip nums names of
    None -> 1
    Some pair -> case columnar-zip pair flags of
      None -> 2
      Some table -> case columnar-get table 1 of
        None -> 3
        Some ((n, name), flag) -> if n == 8 && name == "eight" && not flag
          && columnar-rows table == 2 && columnar-columns table == 3 then 0 else 4
"#);
}

#[test]
fn packed_scalars_roundtrip_through_generic_operations() {
    run("arrays_packed", r#"
  small :: Array I8
  small = array-from-i8s [-128, -1, 127]
  bytes :: Array U8
  bytes = array-from-u8s [0, 128, 255]
  doubles :: Array F64
  doubles = array-from-f64s [i64-to-f64 1, i64-to-f64 (-2)]
  main :: Int
  main = if same (array-get small 0) (-128) && same (array-get small 1) (-1)
    && same (array-get bytes 2) 255
    then case array-get doubles 1 of
      None -> 1
      Some d -> if feq-f64 d (i64-to-f64 (-2)) then 0 else 2
    else 3
"#);
}


#[test]
fn array_intrinsic_signatures_and_primitive_kinds_cannot_be_forged() {
    for declaration in [
        "foreign import ccall \"cinch.runtime.arrayGet\" bad :: forall a. Array a -> Nat -> a",
        "foreign import ccall \"cinch.runtime.arrayFromList\" bad :: forall a b. List a -> Array b",
        "foreign import ccall \"cinch.runtime.arrayFromList\" bad :: List Int -> Array Int",
        "foreign import ccall \"cinch.runtime.tensorReshape\" bad :: forall a. Array Int -> Array a -> Option (Tensor a)",
        "foreign import ccall \"cinch.runtime.columnarZip\" bad :: forall a b. Columnar a -> Columnar b -> Option (Columnar (b, a))",
        "foreign import ccall \"cinch.runtime.arrayFromListU8\" bad :: List U16 -> Array U8",
        "bad :: NDArray Int\n  bad = 0",
        "foreign import ccall \"cinch.runtime.ndarrayGet\" bad :: forall a. Tensor a -> Array Nat -> Option a",
        "bad :: Array\n  bad = 0",
        "bad :: Array 3\n  bad = 0",
        "data Array a where\n    Forged :: a -> Array a",
        "opaque type Tensor a :: Type",
        "record Columnar a where\n    value :: a",
    ] {
        let source = format!("module Main where\n  {declaration}\n  main :: Int\n  main = 0\n");
        let dir = common::project("arrays_reject", &[("Main.cinch", &source)]);
        let error = common::check(&dir, "Main.cinch").expect_err(declaration);
        assert!(error.contains("intrinsic") || error.contains("kind") || error.contains("applied") || error.contains("builtin") || error.contains("not a type") || error.contains("unknown type"), "{declaration}: {error}");
        common::cleanup(&dir);
    }
}

#[test]
fn array_runtime_aliases_are_alpha_equivalent_and_first_class() {
    run("arrays_aliases", r#"
  foreign import ccall "cinch.runtime.arrayReplicate" fill :: forall element. Nat -> element -> Array element
  foreign import ccall "cinch.runtime.arrayGet" at :: forall element. Array element -> Nat -> Option element
  use :: forall a. (Array a -> Nat -> Option a) -> Array a -> Nat -> Option a
  use reader xs i = reader xs i
  main :: Int
  main = if same (use at (fill 2 (42 :: Int)) 1) 42
    && array-length (array-tabulate 5 (\n -> n + 1)) == 5 then 0 else 1
"#);
}

#[test]
fn invalid_shapes_empty_dimensions_and_column_lengths_are_typed_failures() {
    run("arrays_failures", r#"
  words :: Array Int
  words = array-replicate 6 42
  empty :: Array Int
  empty = array-from-list []
  main :: Int
  main = if is-none (tensor-reshape (array-from-list [2, 4]) words)
    && is-none (tensor-reshape (array-from-list [9223372036854775807, 9223372036854775807]) words)
    && is-none (array-set words 6 99)
    && is-none (array-slice words 5 2)
    && is-none (columnar-zip (columnar-column words) (columnar-column empty))
    then case tensor-reshape (array-from-list [99999999999999999999999999, 0]) empty of
      None -> 1
      Some zero -> case tensor-reshape (array-from-list []) (array-replicate 1 (7 :: Int)) of
        None -> 2
        Some scalar -> if same (tensor-get scalar (array-from-list [])) 7
          && is-none (tensor-get zero (array-from-list [0, 0])) then 0 else 3
    else 4
"#);
}

#[test]
fn native_staging_refuses_arrays_by_name_until_the_evaluator_supports_them() {
    let source = "module Main where\n  import Data.Array\n  stage size :: Nat\n  stage size = array-length (array-replicate 2 (1 :: Int))\n  generate do\n    emit (quoteDecls where\n      result :: Nat\n      result = $(quoteNat size))\n  main :: Int\n  main = if result == 2 then 0 else 1\n";
    let mut files = SOURCES.to_vec();
    files.push(("Main.cinch", source));
    let dir = common::project("arrays_stage_refusal", &files);
    let error = common::check(&dir, "Main.cinch").expect_err("the stage evaluator must refuse unsupported primitives");
    assert!(error.contains("arrayReplicate") && error.contains("not available at compile time"), "{error}");
    common::cleanup(&dir);
}

#[test]
fn self_hosted_intrinsic_contracts_refuse_unimplemented_array_operations() {
    let dir = common::project("arrays_self_hosted_refusal", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Main.cinch", r#"module Main where
  import Bootstrap.Builtin
  main :: Int
  main = if is-none (intrinsic-contract "cinch.runtime.arrayFromList")
    && is-none (intrinsic-contract "cinch.runtime.tensorGet")
    && is-none (intrinsic-contract "cinch.runtime.columnarZip") then 0 else 1
"#),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}


#[test]
fn column_projection_returns_the_original_typed_buffers() {
    run("arrays_project", r#"
  ids :: Array Int
  ids = array-from-list [1, 2]
  names :: Array String
  names = array-from-list ["one", "two"]
  main :: Int
  main = case columnar-zip (columnar-column ids) (columnar-column names) of
    None -> 1
    Some table -> case columnar-right table of
      None -> 2
      Some column -> case columnar-array column of
        None -> 3
        Some values -> if same (array-get values 1) "two"
          && is-none (columnar-array table)
          && is-none (columnar-left (columnar-column (array-from-list [(1 :: Int, True)]))) then 0 else 4
"#);
}

#[test]
fn packed_word_storage_resolves_width_from_the_target() {
    use cinch::{arrays::ArrayOp, core::{Class, PrimOp}, types::Scalar};
    for (from, to) in [(Scalar::Usize, Scalar::U32), (Scalar::Isize, Scalar::I32)] {
        let op = PrimOp::Array { op: ArrayOp::FromList, classes: [Class::Raw; 2], packed: Some(from) };
        assert!(matches!(op.resolve_widths(32), PrimOp::Array { packed: Some(t), .. } if t == to));
    }
}


#[test]
fn array_copy_evidence_depends_on_the_element_type() {
    let source = r#"module Main where
  bad :: forall r. Ref r (Array (Thunk {} Int)) -> Array (Thunk {} Int)
  bad a = copyOut a
  main :: Int
  main = 0
"#;
    let dir = common::project("arrays_copy", &[("Main.cinch", source)]);
    let error = common::check(&dir, "Main.cinch").expect_err("an array cannot erase affine element ownership");
    assert!(error.contains("Copy"), "{error}");
    common::cleanup(&dir);
}
