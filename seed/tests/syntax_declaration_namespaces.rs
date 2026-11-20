//! Owned declarations carry both their binding identity and their contracts.
mod common;

use common::source_frontend::{check_sources, run_source, run_source_objects};

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}

#[track_caller]
fn refuses(source: &str, needle: &str) {
    let error = check_sources(source, "declaration-namespaces.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(needle), "{error}");
}

fn c_object(name: &str, source: &str) -> std::path::PathBuf {
    let scratch = std::env::temp_dir().join(format!("cinch-owned-foreign-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).unwrap();
    let fixture = scratch.join("fixture.c");
    let object = scratch.join("fixture.o");
    std::fs::write(&fixture, source).unwrap();
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into());
    let status = std::process::Command::new(clang).arg("-c").arg(&fixture).arg("-o").arg(&object).status().unwrap();
    assert!(status.success(), "compile native C fixture");
    object
}

#[test]
fn owned_refinement_rename_keeps_its_constructor_and_sibling_contracts() {
    let source = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    type Small = refine (value :: Nat) with value < 5;
    three :: Small;
    three = Small 3 (decide (3 < 5)) }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'three) (sym-to-name 'first)
      (Syntax.renameDeclarations (sym-to-name 'Small) (sym-to-name 'Bounded) family))
  main :: Int
  main = if unrefine first == 3 then 42 else 0
"#;
    run(source);
    refuses(&source.replace("Small 3 (decide (3 < 5))", "Small 9 (decide (9 < 5))"), "disproved");
    let wrong_witness = source.replace("three :: Small;", "three :: Nat -> Small;")
        .replace("three = Small 3 (decide (3 < 5))", "three value = Small value (decide (3 < 9))")
        .replace("unrefine first ==", "unrefine (first 3) ==");
    refuses(&wrong_witness, "index mismatch");
    let bound_witness = wrong_witness.replace(
        "three value = Small value (decide (3 < 9))",
        "three value = case decide (3 < 9) of { Yes witness -> Small value witness; No _ -> Small value (decide (3 < 9)) }",
    );
    refuses(&bound_witness, "not of");
}

#[test]
fn owned_refinement_instantiations_keep_their_independent_type_and_value_members() {
    run(r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    type Small = refine (value :: Nat) with value < 5;
    three :: Small;
    three = Small 3 (decide (3 < 5)) }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'three) (sym-to-name 'first)
      (Syntax.renameDeclarations (sym-to-name 'Small) (sym-to-name 'First) family))
    emit (Syntax.renameDeclarations (sym-to-name 'three) (sym-to-name 'second)
      (Syntax.renameDeclarations (sym-to-name 'Small) (sym-to-name 'Second) family))
  main :: Int
  main = if unrefine first == 3 && unrefine second == 3 then 42 else 0
"#);
}

#[test]
fn owned_refinement_link_replaces_the_placeholder_type_and_constructor_together() {
    run(r#"module Main where
  generate do
    let context = quoteDecls where { type Actual = refine (value :: Nat) with value < 5 }
    let template = quoteDecls where {
      type Placeholder = refine (value :: Nat) with value < 5;
      three :: Placeholder;
      three = Placeholder 3 (decide (3 < 5)) }
    emit (Syntax.appendDeclarations context
      (Syntax.linkDeclarations (sym-to-name 'Placeholder) (sym-to-name 'Actual) context template))
  main :: Int
  main = if unrefine three == 3 then 42 else 0
"#);
}

#[test]
fn refinement_group_keeps_private_definition_types_separate_from_owned_members() {
    let source = r#"module Builders where
  export family
  type Small = refine (value :: Nat) with value < 5
  stage family :: Declarations
  stage family = quoteDecls where {
    type Local = refine (value :: Nat) with value < 7;
    captured :: Small;
    captured = Small 3 (decide (3 < 5));
    owned :: Local;
    owned = Local 6 (decide (6 < 7)) }
module Main where
  import stage Builders
  type Small = refine (value :: Nat) with value < 2
  decide :: Bool
  decide = False
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'Local) (sym-to-name 'Bounded) family)
  main :: Int
  main = if unrefine captured == 3 && unrefine owned == 6 then 42 else 0
"#;
    run(source);
    refuses(&source.replace("(sym-to-name 'Local) (sym-to-name 'Bounded)", "(sym-to-name 'Small) (sym-to-name 'Stolen)"), "not an owned member");
    refuses(&source.replace("  export family\n", "  export family\n  decide :: Bool\n  decide = False\n"), "Bool");
}

#[test]
fn quoted_builtin_forms_keep_definition_site_selection_under_caller_decoys() {
    let source = r#"module Makers where
  export protect
  macro protect :: Syntax -> Code Int
  macro protect syntax = let body = Syntax.capture syntax in [| force (delay (\unit -> $(body))) |]
module Main where
  import stage Makers
  force :: Bool
  force = False
  delay :: Bool
  delay = False
  main :: Int
  main = protect!((42))
"#;
    run(source);
    refuses(&source.replace("  export protect\n", "  export protect\n  force :: Bool\n  force = False\n"), "Bool");
}

#[test]
fn foreign_group_instances_rename_cinch_bindings_and_keep_the_external_contract() {
    let source = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    foreign import ccall "cinch_syntax_scale" helper :: I64 ->{Foreign Syntax} I64;
    apply :: I64 ->{Foreign Syntax} I64;
    apply value = helper value }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'apply) (sym-to-name 'first)
      (Syntax.renameDeclarations (sym-to-name 'helper) (sym-to-name 'native-first) family))
    emit (Syntax.renameDeclarations (sym-to-name 'apply) (sym-to-name 'second)
      (Syntax.renameDeclarations (sym-to-name 'helper) (sym-to-name 'native-second) family))
  main :: Unit ->{Foreign Syntax} Int
  main unit = if first 21 == 42 && second 21 == 42 then 42 else 0
"#;
    let program = cinch::driver::check_sources(source, "owned-foreign.cinch").unwrap();
    for name in ["Main.native-first", "Main.native-second"] {
        let plan = program.foreigns.get(name).expect("renamed foreign contract");
        assert_eq!(plan.symbol, "cinch_syntax_scale");
        assert_eq!(plan.unit, name);
        assert_eq!(plan.convention, "ccall");
        assert!(!plan.export);
    }
    let object = c_object("import", "long long cinch_syntax_scale(long long value) { return value * 2; }\n");
    assert_eq!(run_source_objects(source, &[object.to_str().unwrap()]).unwrap(), 42);
    refuses(&source.replace("I64 ->{Foreign Syntax} I64", "I64 -> I64"), "must declare its effects");
}

#[test]
fn foreign_export_renames_its_cinch_definition_without_changing_the_c_entry() {
    let source = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where {
    helper :: I64 -> I64 -> I64;
    helper left right = wrapping-mul-i64 left right;
    foreign export ccall "cinch_syntax_export" helper :: I64 -> I64 -> I64 }
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'helper) (sym-to-name 'native-export) family)
"#;
    let program = cinch::driver::check_sources(source, "owned-export.cinch").unwrap();
    let plan = program.foreigns.get("Main.native-export").expect("renamed export contract");
    assert!(plan.export);
    assert_eq!(plan.unit, "Main.native-export");
    assert_eq!(plan.symbol, "cinch_syntax_export");
    let object = c_object("export", "extern long long cinch_syntax_export(long long, long long);\nint main(void) { return cinch_syntax_export(6, 7) == 42 ? 42 : 0; }\n");
    assert_eq!(run_source_objects(source, &[object.to_str().unwrap()]).unwrap(), 42);
    refuses(&source.replace(
        "foreign export ccall \"cinch_syntax_export\" helper :: I64 -> I64 -> I64",
        "foreign export ccall \"cinch_syntax_export\" helper :: I64 -> I64",
    ), "exactly the target function's type");
}

#[test]
fn primitive_group_instances_keep_sibling_bindings_and_ir_containment() {
    let source = r#"module Main where
  stage family :: Declarations
  stage family = quoteDecls where
    prim step :: U32 -> U32 where
      llvm {
        %next = add i32 %0, 1
        ret i32 %next
      }
    answer :: U32
    answer = step 41
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'answer) (sym-to-name 'first)
      (Syntax.renameDeclarations (sym-to-name 'step) (sym-to-name 'increment-first) family))
    emit (Syntax.renameDeclarations (sym-to-name 'answer) (sym-to-name 'second)
      (Syntax.renameDeclarations (sym-to-name 'step) (sym-to-name 'increment-second) family))
  main :: Int
  main = if first == 42 && second == 42 then 42 else 0
"#;
    run(source);
    refuses(&source.replace("%next = add i32 %0, 1", "%next = call i32 @sneaky(i32 %0)"), "no-escape");
}
