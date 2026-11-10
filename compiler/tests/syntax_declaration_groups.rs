//! Native declaration instantiation follows opaque ownership, never text.
mod common;

use common::source_frontend::{check_source_interfaces, check_sources, run_source};

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}

#[track_caller]
fn refuses(source: &str, needle: &str) {
    let error = check_sources(source, "declaration-group.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(needle), "{error}");
}

#[test]
fn native_group_renames_record_type_constructor_and_sibling_references() {
    run(r#"module Main where
  macro family :: Name -> Name -> Declarations
  macro family tag function =
    let group = quoteDecls where
          record Packet where
            payload :: Int
          inspect :: Packet -> Int
          inspect packet = let inspect = ((\value -> value + value) :: Int -> Int) in inspect packet.payload
    in Syntax.renameDeclarations (sym-to-name 'inspect) function
      (Syntax.renameDeclarations (sym-to-name 'Packet) tag group)
  generate do
    emit (family!(Envelope, read))
  main :: Int
  main = read (Envelope {payload = 21})
"#);
}

#[test]
fn one_native_group_can_be_instantiated_twice_without_retargeting_the_first() {
    run(r#"module Main where
  stage base :: Declarations
  stage base = quoteDecls where
    record Packet where
      payload :: Int
    inspect :: Packet -> Int
    inspect packet = packet.payload
  stage family :: Name -> Name -> Declarations -> Declarations
  stage family tag function group = Syntax.renameDeclarations (sym-to-name 'inspect) function
    (Syntax.renameDeclarations (sym-to-name 'Packet) tag group)
  generate do
    let group = base
    emit (family (sym-to-name 'Envelope) (sym-to-name 'readEnvelope) group)
    emit (family (sym-to-name 'Frame) (sym-to-name 'readFrame) group)
  main :: Int
  main = readEnvelope (Envelope {payload = 20}) + readFrame (Frame {payload = 22})
"#);
}

#[test]
fn native_group_renames_data_constructor_patterns_recursion_and_annotations() {
    run(r#"module Main where
  macro family :: Name -> Name -> Name -> Declarations
  macro family tag constructor function =
    let group = quoteDecls where
          data Packet where
            Wrap :: Int -> Packet
          inspect :: Packet -> Int
          inspect packet = case packet of
            Wrap value -> if value == 0 then 42 else inspect (Wrap (value - 1) :: Packet)
    in Syntax.renameDeclarations (sym-to-name 'inspect) function
      (Syntax.renameDeclarations (sym-to-name 'Wrap) constructor
        (Syntax.renameDeclarations (sym-to-name 'Packet) tag group))
  generate do
    emit (family!(Envelope, MkEnvelope, read))
  main :: Int
  main = read (MkEnvelope 3)
"#);
}

#[test]
fn native_group_rename_preserves_private_helpers_and_caller_captures() {
    run(r#"module Library where
  export family
  helper :: Int -> Int
  helper value = value + 1
  macro family :: Name -> Syntax -> Declarations
  macro family function operand =
    let group = quoteDecls where
          answer :: Int
          answer = helper $(operand)
    in Syntax.renameDeclarations (sym-to-name 'answer) function group
module Main where
  import stage Library
  helper :: Bool
  helper = False
  answer :: Int
  answer = 41
  generate do
    emit (family!(result, answer))
  main :: Int
  main = result
"#);
}

#[test]
fn declaration_bindings_survive_a_nested_code_quotation_and_later_instantiation() {
    run(r#"module Main where
  answer :: Bool
  answer = False
  macro family :: Name -> Name -> Declarations
  macro family function quotation =
    let group = quoteDecls where {
      answer :: Int;
      answer = 42;
      quoted :: Code Int;
      quoted = [| answer |] }
    in Syntax.renameDeclarations (sym-to-name 'quoted) quotation
      (Syntax.renameDeclarations (sym-to-name 'answer) function group)
  generate do
    emit (family!(result, generated-code))
  main :: Int
  main = eval generated-code
"#);
}

#[test]
fn stage_body_macro_calls_share_nullary_and_unary_empty_argument_rules() {
    let source = r#"module Main where
  macro maker :: Code Int
  macro maker = [| 42 |]
  stage stored :: Code Int
  stage stored = maker!()
  generate do
    emit (quoteDecls where { answer :: Int; answer = $(stored) })
  main :: Int
  main = answer
"#;
    run(source);
    refuses(&source.replace("maker!()", "maker!(unexpected)"), "takes 0 token-tree");
    run(&source.replace("macro maker :: Code Int", "macro maker :: Syntax -> Code Int")
        .replace("macro maker = [|", "macro maker ignored = [|"));
    // A stage-body macro call returns Code; it does not execute that code or
    // erase its phase boundary to obtain a function returning Declarations.
    refuses(r#"module Main where
  macro maker :: Code (Unit -> Declarations)
  macro maker = [| \unit -> quoteDecls where { answer :: Int; answer = 42 } |]
  stage factory :: Unit -> Declarations
  stage factory = maker!()
  generate do
    emit (factory ())
  main :: Int
  main = 42
"#, "type mismatch");
}

#[test]
fn native_group_cannot_acquire_a_private_helper_by_selecting_its_name() {
    let source = r#"module Library where
  export family
  helper :: Int
  helper = 42
  macro family :: Name -> Declarations
  macro family function = Syntax.renameDeclarations (sym-to-name 'helper) function
    (quoteDecls where { answer :: Int; answer = helper })
module Main where
  import stage Library
  generate do
    emit (family!(result))
  main :: Int
  main = 42
"#;
    refuses(source, "not an owned member");
}

#[test]
fn native_group_exports_follow_owned_renames_without_exporting_private_constructors() {
    let source = r#"module Library where
  macro family :: Name -> Name -> Name -> Declarations
  macro family tag create inspect =
    let group = quoteDecls where {
      export Packet, make, read;
      data Packet where { Wrap :: Int -> Packet };
      make :: Int -> Packet;
      make value = Wrap value;
      read :: Packet -> Int;
      read packet = case packet of { Wrap value -> value } }
    in Syntax.renameDeclarations (sym-to-name 'read) inspect
      (Syntax.renameDeclarations (sym-to-name 'make) create
        (Syntax.renameDeclarations (sym-to-name 'Packet) tag group))
  generate do
    emit (family!(Envelope, create, inspect))
module Main where
  import Library as Wire
  main :: Int
  main = Wire.inspect (Wire.create 42)
"#;
    run(source);
    refuses(&source.replace("Wire.create 42", "Wire.Wrap 42"), "unknown constructor Wire.Wrap");
}

#[test]
fn native_group_operator_references_and_exported_fixity_follow_the_owned_member() {
    let source = r#"module Main where
  infixr 6 <#>
  (<#>) :: Int -> Int -> Int
  (<#>) left right = 0
  macro rename :: Name -> Name -> Declarations
  macro rename source desired = Syntax.renameDeclarations source desired
    (quoteDecls where {
      infixr 6 <#>;
      (<#>) :: Int -> Int -> Int;
      (<#>) left right = left - right;
      calculate :: Int;
      calculate = 50 <#> 10 <#> 2 })
  generate do
    emit (rename!(<#>, <%>))
  main :: Int
  main = calculate
"#;
    run(source);
    let interfaces = check_source_interfaces(source, "owned-fixity.cinch").unwrap();
    let main = interfaces.iter().find(|interface| interface.module == "Main").unwrap();
    assert!(main.exports.fixities.iter().any(|fixity| fixity.op == "<%>" && fixity.assoc == "right" && fixity.prec == 6));
    // The unrelated source operator and its fixity keep their own identity.
    assert!(main.exports.fixities.iter().any(|fixity| fixity.op == "<#>" && fixity.assoc == "right" && fixity.prec == 6));
}

#[test]
fn native_group_renaming_keeps_ordinary_checks_and_phase_barriers() {
    let source = r#"module Main where
  macro family :: Name -> Declarations
  macro family tag = Syntax.renameDeclarations (sym-to-name 'Packet) tag
    (quoteDecls where { record Packet where { payload :: Int } })
  generate do
    emit (family!(Envelope))
  main :: Int
  main = (Envelope {payload = True}).payload
"#;
    refuses(source, "type mismatch");
    refuses(&source.replace("Envelope))", "Int))"), "builtin type name");
    refuses(&source.replace("Envelope))", "envelope))"), "constructor identifier");
    refuses(r#"module Main where
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'Wrap) (sym-to-name 'wrap)
      (quoteDecls where { data Packet where { Wrap :: Int -> Packet } }))
  main :: Int
  main = 42
"#, "identifier kind");
    refuses(r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Int, {payload :: Bool}))
  main :: Int
  main = 42
"#, "builtin type name");
    refuses(r#"module Main where
  main :: Int
  main = Syntax.renameDeclarations (sym-to-name 'a) (sym-to-name 'b)
    (quoteDecls where { a :: Int; a = 42 })
"#, "compile-time");
}

#[test]
fn binary_format_is_a_native_group_with_definition_helpers_and_source_operands() {
    let source = format!("{}\n{}\n{}", include_str!("../../experiments/malleable-syntax/libraries/Binary.cinch"),
        include_str!("../../experiments/malleable-syntax/libraries/FormatSyntax.cinch"), r#"module Decoy where
  export decode, encode
  decode :: Bool
  decode = False
  encode :: Bool
  encode = False
module Main where
  import Data.Bytes
  import Decoy as Binary
  import stage FormatSyntax
  parsePacket :: InRange 0 255 Nat
  parsePacket = 67
  generate do
    emit (FormatSyntax.declare!(Packet, parsePacket, 1))
    emit (FormatSyntax.declare!(Frame, 68, 2))
  main :: Int
  main = let payload = bytes-from-list [10, 20, 30] in
    case write-Packet (Packet {payload = payload}) of
      None -> 0
      Some bytes -> case parse-Packet bytes of
        None -> 1
        Some packet -> case parse-Frame bytes of
          Some bad -> 2
          None -> if packet.payload == payload then 42 else 3
"#);
    run(&source);
}
