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
    assert!(text.contains(&format!("\"runtime_abi\":{}", cinch::interface::RUNTIME_ABI_VERSION)), "{text}");
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
        ("Bootstrap.RuntimeAbi.cinch", "module Bootstrap.RuntimeAbi where\n  export runtime-abi, runtime-abi-int\n  runtime-abi :: Nat\n  runtime-abi = 5\n  runtime-abi-int :: Int\n  runtime-abi-int = 5\n"),
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


#[test]
fn cinch_array_runtime_matches_checked_kinds_bounds_formats_and_ownership() {
    let dir = common::project("array_runtime_bootstrap", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.RuntimeIr.cinch", include_str!("../../bootstrap/Bootstrap.RuntimeIr.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RuntimeIr.cinch")),
    ]);
    // Explicit Native fixture producer for the defining inductive contracts.
    // Every source program below is checked and lowered by the Cinch emitter.
    array_test_interfaces(&dir, include_str!("../prelude/Data/Core.cinch"));
    let compiler = common::build(&dir, "Main.cinch", "array-emitter").expect("build actual Cinch array emitter");
    let runtime = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("runtime/target/debug/libcinchruntime.a");
    assert!(runtime.is_file(), "matching runtime archive must be built by the Native driver");
    let mut declarations = String::from("module Main where\n  import Data.Core\n  missing :: forall a. Option a -> Bool\n  missing option = case option of None -> True; Some _ -> False\n  int-result :: Option Int -> Int\n  int-result option = case option of None -> -1; Some n -> n\n");
    for module in [include_str!("../prelude/Data/Array.cinch"), include_str!("../prelude/Data/Tensor.cinch"), include_str!("../prelude/Data/Columnar.cinch"), include_str!("../prelude/Data/Array/Packed.cinch")] {
        for line in module.lines().filter(|line| line.trim_start().starts_with("foreign import ccall")) {
            // Keep the source module's independent canonical scheme, changing only its local alias.
            let mut fields = line.trim().splitn(6, ' ');
            let prefix = fields.by_ref().take(4).collect::<Vec<_>>().join(" ");
            let alias = fields.next().unwrap();
            let signature = fields.next().unwrap();
            declarations.push_str(&format!("  {prefix} {} {signature}\n", alias.replace('-', "")));
        }
    }
    let cases: Vec<(&str, &str, i32)> = vec![
        ("from-list", "  main :: Int\n  main = case arrayget (arrayfromlist (Cons 18446744073709551616 (Cons 42 Nil))) 1 of Some x -> x; None -> 1\n", 42),
        ("replicate-boundaries", "  main :: Int\n  main = let xs = arrayreplicate 3 (42 :: Int) in if arraylength xs == 3 && missing (arrayget xs 99999999999999999999999999) && missing (arrayget xs 3) then int-result (arrayget xs 2) else 4\n", 42),
        ("persistent-set", "  main :: Int\n  main = let xs = arrayreplicate 2 (20 :: Int) in case arrayset xs 1 42 of None -> 1; Some changed -> case (arrayget xs 1, arrayget changed 1) of (Some old, Some new) -> if old == 20 then new else 2; _ -> 3\n", 42),
        ("slice-empty-end", "  main :: Int\n  main = case arrayslice (arrayreplicate 2 (7 :: Int)) 2 0 of Some xs -> if arraylength xs == 0 then 42 else 1; None -> 2\n", 42),
        ("slice-invalid", "  main :: Int\n  main = case arrayslice (arrayreplicate 2 (7 :: Int)) 1 2 of None -> 42; Some xs -> 1\n", 42),
        ("append", "  main :: Int\n  main = case arrayget (arrayappend (arrayreplicate 2 (7 :: Int)) (arrayreplicate 1 42)) 2 of Some x -> x; None -> 1\n", 42),
        ("array-first-class", "  apply :: forall a. (Array a -> Nat -> Option a) -> Array a -> Nat -> Option a\n  apply read xs n = read xs n\n  main :: Int\n  main = case apply arrayget (arrayreplicate 1 (42 :: Int)) 0 of Some n -> n; None -> 1\n", 42),
        ("owned-record", "  data Box where\n    Box :: Int -> Box\n  make :: Unit -> Array Box\n  make unit = arrayfromlist (Cons (Box 18446744073709551616) (Cons (Box 42) Nil))\n  main :: Int\n  main = case arrayget (arrayappend (make ()) (make ())) 3 of Some (Box n) -> n; None -> 1\n", 42),
        ("tensor-all-accessors", "  main :: Int\n  main = case tensorreshape (arrayfromlist (Cons 2 (Cons 3 Nil))) (arrayreplicate 6 (42 :: Int)) of None -> 1; Some t -> if arraylength (tensorshape t) == 2 && arraylength (tensordata t) == 6 then int-result (tensorget t (arrayfromlist (Cons 1 (Cons 2 Nil)))) else 3\n", 42),
        ("tensor-persistent-set", "  main :: Int\n  main = case tensorreshape (arrayfromlist (Cons 2 Nil)) (arrayreplicate 2 (20 :: Int)) of None -> 1; Some t -> case tensorset t (arrayfromlist (Cons 1 Nil)) 42 of None -> 2; Some u -> case (tensorget t (arrayfromlist (Cons 1 Nil)), tensorget u (arrayfromlist (Cons 1 Nil))) of (Some old, Some new) -> if old == 20 then new else 3; _ -> 4\n", 42),
        ("tensor-axis-refusal", "  main :: Int\n  main = case tensorreshape (arrayfromlist (Cons 2 (Cons 3 Nil))) (arrayreplicate 6 (42 :: Int)) of None -> 1; Some t -> if missing (tensorget t (arrayfromlist (Cons 0 (Cons 3 Nil)))) && missing (tensorget t (arrayfromlist (Cons 1 Nil))) then 42 else 2\n", 42),
        ("tensor-shape-failure", "  main :: Int\n  main = case tensorreshape (arrayfromlist (Cons 2 (Cons 4 Nil))) (arrayreplicate 6 (42 :: Int)) of None -> 42; Some t -> 1\n", 42),
        ("columnar-mixed-map", "  row-result :: Option (Bool, Int) -> Int\n  row-result value = case value of None -> 3; Some (flag, n) -> if flag && n == 18446744073709551616 then 42 else 2\n  main :: Int\n  main = case columnarzip (columnarcolumn (arrayreplicate 2 True)) (columnarcolumn (arrayreplicate 2 (18446744073709551616 :: Int))) of None -> 1; Some t -> if columnarrows t == 2 && columnarcolumns t == 2 then row-result (columnarget t 1) else 4\n", 42),
        ("columnar-projections", "  column-value :: Option (Columnar Int) -> Int\n  column-value col = case col of\n    None -> -1\n    Some c -> case columnararray c of\n      None -> -2\n      Some a -> int-result (arrayget a 0)\n  main :: Int\n  main = case columnarzip (columnarcolumn (arrayreplicate 2 (7 :: Int))) (columnarcolumn (arrayreplicate 2 (42 :: Int))) of\n    None -> 1\n    Some t -> if missing (columnararray t) && column-value (columnarleft t) == 7 then column-value (columnarright t) else 2\n", 42),
        ("columnar-shape-refusal", "  main :: Int\n  main = case columnarzip (columnarcolumn (arrayreplicate 2 (7 :: Int))) (columnarcolumn (arrayreplicate 1 (42 :: Int))) of None -> 42; Some t -> 1\n", 42),
        ("columnar-tuple-leaf", "  main :: Int\n  main = let leaf = columnarcolumn (arrayreplicate 1 (True, 42 :: Int)) in if missing (columnarleft leaf) && missing (columnarright leaf) then 42 else 2\n", 42),
        ("generic-empty", "  main :: Int\n  main = case arrayget (arrayfromlist (Nil :: List Int)) 0 of None -> 42; Some x -> 1\n", 42),
        ("set-invalid", "  main :: Int\n  main = case arrayset (arrayreplicate 1 (42 :: Int)) 999999999999999999999 9 of None -> 42; Some x -> 1\n", 42),
    ];
    let mut sources = cases.into_iter().map(|(n, b, c)| (n.to_string(), format!("{declarations}{b}"), c)).collect::<Vec<_>>();
    for (kind, value) in [("I8", "-128"), ("I16", "-32768"), ("I32", "-2147483648"), ("I64", "-9223372036854775808"), ("Isize", "-9223372036854775808"), ("U8", "255"), ("U16", "65535"), ("U32", "4294967295"), ("U64", "18446744073709551615"), ("Usize", "18446744073709551615")] {
        sources.push((format!("packed-{kind}"), format!("{declarations}  main :: Int\n  main = case arrayget (arrayfrom{}s (Cons ({value} :: {kind}) Nil)) 0 of Some n -> if n == {value} then 42 else 1; None -> 2\n", kind.to_lowercase()), 42));
    }
    sources.push(("packed-char".into(), format!("{declarations}  main :: Int\n  main = case arrayget (arrayfromchars (Cons 'z' Nil)) 0 of Some 'z' -> 42; _ -> 2\n"), 42));
    // Empty packed float buffers exercise their canonical contracts and layouts.
    // Nonempty float bit-pattern conformance remains required separately.
    for kind in ["F32", "F64"] {
        sources.push((format!("packed-empty-{kind}"), format!("{declarations}  main :: Int\n  main = if arraylength (arrayfrom{}s Nil) == 0 then 42 else 1\n", kind.to_lowercase()), 42));
    }
    std::fs::write(dir.join("batch.count"), vec![b'x'; sources.len()]).unwrap();
    for (index, (_, source, _)) in sources.iter().enumerate() { std::fs::write(dir.join(format!("input-{index}.source")), source).unwrap(); }
    let output = std::process::Command::new(&compiler).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "batch driver: {}", String::from_utf8_lossy(&output.stderr));
    for (index, (name, _, expected)) in sources.iter().enumerate() {
        let status = common::read_output(&dir, &format!("output-{index}.status"));
        let diagnostics = common::read_output(&dir, &format!("output-{index}.diagnostics"));
        assert_eq!(status, "0", "{name}: {diagnostics}");
        let executable = dir.join(format!("case-{index}"));
        let linked = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into())).arg(dir.join(format!("output-{index}.ll"))).arg(&runtime).args(["-lpthread", "-ldl", "-lm", "-o"]).arg(&executable).output().unwrap();
        assert!(linked.status.success(), "{name}: {}", String::from_utf8_lossy(&linked.stderr));
        let output = std::process::Command::new(executable).env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1").env("CINCH_HEAP_LIMIT", "32M").output().unwrap();
        assert_eq!(output.status.code(), Some(*expected), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).split_whitespace().any(|word| word == "leaked=0"), "{name}: {}", String::from_utf8_lossy(&output.stderr));
        println!("Cinch array {name}: exit {expected}; {}", String::from_utf8_lossy(&output.stderr).trim());
    }
    for (name, declaration) in [
        ("erased-option", "foreign import ccall \"cinch.runtime.arrayGet\" bad :: forall a. Array a -> Nat -> a"),
        ("changed-element", "foreign import ccall \"cinch.runtime.arrayFromList\" bad :: forall a b. List a -> Array b"),
        ("monomorphic-forgery", "foreign import ccall \"cinch.runtime.arrayFromList\" bad :: List Int -> Array Int"),
        ("wrong-shape", "foreign import ccall \"cinch.runtime.tensorReshape\" bad :: forall a. Array Int -> Array a -> Option (Tensor a)"),
        ("reversed-row", "foreign import ccall \"cinch.runtime.columnarZip\" bad :: forall a b. Columnar a -> Columnar b -> Option (Columnar (b, a))"),
        ("wrong-packed", "foreign import ccall \"cinch.runtime.arrayFromListU8\" bad :: List U16 -> Array U8"),
        ("unsaturated-kind", "bad :: Array\n  bad = 0"),
        ("wrong-kind", "bad :: Array 3\n  bad = 0"),
        ("usize-overflow", "bad :: Usize\n  bad = 18446744073709551616"),
        ("isize-overflow", "bad :: Isize\n  bad = 9223372036854775808"),
        ("forge-data", "data Array a where\n    Forged :: a -> Array a"),
        ("forge-opaque", "opaque type Tensor a :: Type"),
    ] {
        std::fs::remove_file(dir.join("batch.count")).unwrap_or(());
        std::fs::write(dir.join("input.source"), format!("module Main where\n  import Data.Core\n  {declaration}\n  main :: Int\n  main = 0\n")).unwrap();
        let output = std::process::Command::new(&compiler).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(4), "{name}: {}", common::read_output(&dir, "output.diagnostics"));
        assert!(common::read_output(&dir, "output.ll").is_empty(), "{name}: refused source retained an old artifact");
    }
    // A valid nominal name alone cannot grant a physical runtime layout.
    // Rebuild checked interfaces with changed constructors and never execute them.
    let original_core = include_str!("../prelude/Data/Core.cinch");
    for (name, core_source, body) in [
        ("changed-list-fields", original_core.replace("Cons :: a -> List a -> List a", "Cons :: List a -> a -> List a"),
            "  main :: Int\n  main = case arrayget (arrayfromlist (Cons Nil (42 :: Int))) 0 of None -> 1; Some _ -> 42\n"),
        ("changed-option-payload", original_core.replace("Some :: a -> Option a", "Some :: List a -> Option a"),
            "  main :: Int\n  main = case arrayget (arrayreplicate 1 (42 :: Int)) 0 of None -> 1; Some _ -> 42\n"),
        ("changed-option-tags", original_core.replace("None :: Option a\n    Some :: a -> Option a", "Some :: a -> Option a\n    None :: Option a"),
            "  main :: Int\n  main = case arrayget (arrayreplicate 1 (42 :: Int)) 0 of None -> 1; Some _ -> 42\n"),
    ] {
        assert_ne!(core_source, original_core, "{name}: mutation did not apply");
        array_test_interfaces(&dir, &core_source);
        let abi_declarations = declarations.lines().filter(|line| line.starts_with("module ") || line.trim_start().starts_with("import ") || line.trim_start().starts_with("foreign import ")).collect::<Vec<_>>().join("\n");
        std::fs::write(dir.join("input.source"), format!("{abi_declarations}\n{body}")).unwrap();
        let output = std::process::Command::new(&compiler).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(5), "{name}: {}", common::read_output(&dir, "output.diagnostics"));
        assert!(common::read_output(&dir, "output.ll").is_empty(), "{name}: mismatched physical ABI emitted an artifact");
        assert!(common::read_output(&dir, "output.diagnostics").contains("array"), "{name}: refusal must identify the array ABI boundary");
        println!("Cinch array {name}: refused checked nominal with mismatched runtime layout; no artifact or execution");
    }
    common::cleanup(&dir);
}

fn array_test_quote(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '\"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"),
            c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('\"'); out
}

fn array_test_interfaces(dir: &std::path::Path, core_source: &str) {
    let fixities = Default::default();
    let tables = std::collections::HashMap::new();
    let prelude_source = "module Prelude where\n";
    let main_source = "module Main where\n  main :: Int\n  main = 0\n";
    let prelude = cinch::driver::parse_source(prelude_source, "Prelude.cinch", &fixities, &tables).unwrap();
    let core = cinch::driver::parse_source(core_source, "Data.Core.cinch", &fixities, &tables).unwrap();
    let entry = cinch::driver::parse_source(main_source, "Main.cinch", &fixities, &tables).unwrap();
    let hashes = [("Prelude".into(), cinch::interface::content_hash(prelude_source)),
        ("Data.Core".into(), cinch::interface::content_hash(core_source)),
        ("Main".into(), cinch::interface::content_hash(main_source))].into_iter().collect();
    let (_, interfaces, _, _) = cinch::check::check_program_multi_full(
        &prelude.modules, &[core.modules, entry.modules].concat(), &hashes,
        &cinch::check::StagingCx::default(), &[], &cinch::core::TargetProfile::native(),
    ).expect("produce genuine defining Core contracts");
    let mut rows = Vec::new();
    for interface in interfaces.into_iter().filter(|i| i.module != "Main") {
        let path = dir.join(format!("{}.cinchif", interface.module));
        std::fs::write(&path, interface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", array_test_quote(&interface.module), array_test_quote(&interface.iface_hash()), array_test_quote(&path.display().to_string())));
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
}
