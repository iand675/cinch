//! Native declaration objects preserve the scope of captured field types.
mod common;

use cinch::driver::{Mode, StageMode};
use common::drive_with;
use common::source_frontend::{check_source_interfaces, check_sources, run_source};

fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}

fn refuses(source: &str, needles: &[&str]) {
    let error = check_sources(source, "native-declaration.cinch")
        .err().expect("invalid native declarations must be refused").to_string();
    for needle in needles { assert!(error.contains(needle), "{error}"); }
}

#[test]
fn native_record_declaration_constructs_checked_field_trees() {
    run(r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Envelope, {payload :: Int; label :: String; callback :: Int -> Int}))
  main :: Int
  main = let row = Envelope {payload = 21, label = "native", callback = \value -> value + value} in
    if row.label == "native" then row.callback row.payload else 0
"#);
}

#[test]
fn ordinary_headers_can_name_generated_record_types() {
    run(r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Envelope, {payload :: Int}))
  make :: Int -> Envelope
  make value = Envelope {payload = value}
  inspect :: Envelope -> Int
  inspect row = row.payload
  main :: Int
  main = inspect (make 42)
"#);
}

#[test]
fn ordinary_headers_before_generators_can_name_generated_types() {
    run(r#"module Main where
  inspect :: Envelope -> Int
  inspect row = row.payload
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  main :: Int
  main = inspect (Envelope {payload = 42})
"#);
}

#[test]
fn generated_types_keep_export_and_import_ownership() {
    let source = r#"module Library where
  export Envelope, make, inspect
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  make :: Int -> Envelope
  make value = Envelope {payload = value}
  inspect :: Envelope -> Int
  inspect row = row.payload
module Main where
  import Library as Wire
  record Envelope where
    payload :: Bool
  main :: Int
  main = Wire.inspect (Wire.make 42)
"#;
    run(source);
    let interfaces = check_source_interfaces(source, "generated-export.cinch").unwrap();
    let library = interfaces.iter().find(|interface| interface.module == "Library").unwrap();
    let caller = interfaces.iter().find(|interface| interface.module == "Main").unwrap();
    let wire_type = library.exports.types.iter().find(|ty| ty.name == "Envelope").unwrap();
    let caller_type = caller.exports.types.iter().find(|ty| ty.name == "Envelope").unwrap();
    assert_ne!(wire_type.identity, caller_type.identity);
    // Portable value contracts for record fields are still unsupported.
    // The native-import milestone must supply that representation; the
    // declaration boundary here proves the actual exported type identity.
    refuses(&source.replace("Wire.make 42", "(Envelope {payload = True} :: Envelope)"), &["type mismatch"]);
}

#[test]
fn generated_data_headers_keep_portable_definition_site_owners() {
    let source = r#"module Library where
  export Envelope, make, inspect
  generate do
    emit (quoteDecls where
      data Envelope where
        Wrap :: Int -> Envelope)
  make :: Int -> Envelope
  make value = Wrap value
  inspect :: Envelope -> Int
  inspect packet = case packet of Wrap value -> value
module Main where
  import Library as Wire
  data Envelope where
    Wrap :: Bool -> Envelope
  main :: Int
  main = Wire.inspect (Wire.make 42)
"#;
    run(source);
    let interfaces = check_source_interfaces(source, "generated-data-export.cinch").unwrap();
    let library = interfaces.iter().find(|interface| interface.module == "Library").unwrap();
    let wire_type = library.exports.types.iter().find(|ty| ty.name == "Envelope").unwrap();
    let make = library.exports.values.iter().find(|value| value.name == "make").unwrap();
    assert!(make.resolved.as_ref().expect("data contracts retain checked nominal ownership").nominals.iter()
        .any(|nominal| nominal.identity == wire_type.identity));
}

#[test]
fn generated_nominal_names_and_constructors_reach_the_driver_manifest() {
    let directory = std::env::temp_dir().join(format!("cinch-generated-nominal-report-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let path = directory.join("Main.cinch");
    std::fs::write(&path, r#"module Main where
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int
      data Choice where
        Off :: Choice
        On :: Int -> Choice)
  main :: Int
  main = 42
"#).unwrap();
    drive_with(Mode::Check, StageMode::Refresh, &directory, "Main.cinch", None).unwrap();
    let manifest = std::fs::read_to_string(directory.join("build/Main.manifest.json")).unwrap();
    let compact: String = manifest.chars().filter(|character| !character.is_whitespace()).collect();
    assert!(compact.contains(r#""generated":{"Main":["Choice","Envelope","Off","On"]}"#), "{manifest}");
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn generated_types_support_selected_renamed_imports_and_reexports() {
    run(r#"module Library where
  export Envelope, make, inspect
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  make :: Int -> Envelope
  make value = Envelope {payload = value}
  inspect :: Envelope -> Int
  inspect row = row.payload
module Public where
  import Library (Envelope as Payload, make as produce, inspect as read)
  export Payload, produce, read
module Main where
  import Public as Wire
  inspect :: Wire.Payload -> Int
  inspect packet = Wire.read packet
  main :: Int
  main = inspect (Wire.produce 42)
"#);
}

#[test]
fn generated_header_deferral_does_not_admit_missing_names_or_bad_bodies() {
    let source = r#"module Main where
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  make :: Int -> Envelope
  make value = Envelope {payload = value}
  main :: Int
  main = (make 42).payload
"#;
    refuses(&source.replace("Int -> Envelope", "Int -> Absent"), &["unknown type Absent"]);
    refuses(&source.replace("payload = value", "payload = True"), &["type mismatch"]);
    refuses(&source.replace("  make value = Envelope {payload = value}\n", ""), &["missing defining equation for make"]);
    refuses(&source.replace("  main :: Int", "  make :: Int -> Envelope\n  main :: Int"), &["duplicate signature for make"]);
}

#[test]
fn generated_header_deferral_keeps_private_names_and_validates_final_exports() {
    let source = r#"module Library where
  export make
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  make :: Int -> Envelope
  make value = Envelope {payload = value}
module Main where
  import Library as Wire
  inspect :: Wire.Envelope -> Int
  inspect row = row.payload
  main :: Int
  main = inspect (Wire.make 42)
"#;
    refuses(source, &["unknown type Wire.Envelope"]);
    let public = source.replace("export make", "export make, Envelope");
    run(&public);
    refuses(&public.replace("export make, Envelope", "export make, Envelope, Absent"), &["not a name in scope to export"]);
    let selection = public.replace("import Library as Wire", "import Library (Absent as missing)")
        .replace("  inspect :: Wire.Envelope -> Int\n  inspect row = row.payload\n", "")
        .replace("main = inspect (Wire.make 42)", "main = 42");
    refuses(&selection, &["has no member `Absent` to import"]);
}

#[test]
fn later_generators_can_compile_helpers_with_generated_nominal_headers() {
    run(r#"module Main where
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int)
  inspect :: Envelope -> Int
  inspect row = row.payload
  generate do
    emit (quoteDecls where
      answer :: Int
      answer = $(quoteInt (inspect (Envelope {payload = 42}))))
  main :: Int
  main = answer
"#);
}

#[test]
fn generated_nominal_headers_participate_in_staging_cycle_detection() {
    refuses(r#"module Main where
  inspect :: Envelope -> Int
  inspect row = row.payload
  generate do
    emit (quoteDecls where
      record Envelope where
        payload :: Int
      answer :: Int
      answer = $(quoteInt (inspect (Envelope {payload = 42}))))
  main :: Int
  main = answer
"#, &["cycle"]);
}

fn definition_source() -> &'static str {
    r#"module Library where
  export declare, makeToken, inspectToken
  data Token where
    Token :: Int -> Token
  makeToken :: Int -> Token
  makeToken value = Token value
  inspectToken :: Token -> Int
  inspectToken token = case token of Token value -> value
  macro fields :: Syntax -> Syntax
  macro fields source = source
  macro declare :: Name -> Declarations
  macro declare name = Syntax.recordDeclaration name (fields!({payload :: Token; callback :: Token -> Int}))
module Main where
  import Library
  import stage Library
  data Token where
    Token :: Bool -> Token
  generate do
    emit (declare!(Envelope))
  main :: Int
  main = let row = Envelope {payload = makeToken 42, callback = inspectToken} in row.callback row.payload
"#
}

#[test]
fn native_record_fields_keep_private_definition_types_under_same_spelled_caller_types() {
    let source = definition_source();
    run(source);
    refuses(&source.replace("payload = makeToken 42", "payload = (Token True :: Token)"), &["type mismatch", "Token"]);
}

#[test]
fn native_record_fields_cannot_acquire_private_types_from_insertion() {
    let source = definition_source().replace("  generate do\n    emit (declare!(Envelope))", r#"  macro plain :: Name -> Syntax -> Declarations
  macro plain name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Envelope))
    emit (plain!(Leak, {payload :: Library.Token}))"#);
    refuses(&source, &["unknown type Library.Token"]);
}

#[test]
fn unbound_native_field_types_stay_unbound_when_delivered() {
    let source = definition_source()
        .replace("fields!({payload :: Token;", "fields!({payload :: Absent;")
        .replace("module Main where", "module Main where\n  data Absent where\n    Absent :: Int -> Absent");
    refuses(&source, &["unknown type Absent"]);
}

#[test]
fn native_record_fields_still_check_duplicates_and_requested_names() {
    let source = r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Envelope, {payload :: Int; payload :: Bool}))
  main :: Int
  main = 42
"#;
    refuses(source, &["duplicate record field payload"]);
    refuses(&source.replace("Envelope,", "envelope,"), &["record's name must be a constructor identifier"]);
}

#[test]
fn native_record_erased_fields_keep_proof_obligations() {
    let source = r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Envelope, {payload :: Int; erased evidence :: Eq 42 42}))
  main :: Int
  main = let row = Envelope {payload = 42, evidence = refl} in row.payload
"#;
    run(source);
    refuses(&source.replace("Eq 42 42", "Eq 42 43"), &["obligation disproved"]);
}

#[test]
fn native_record_fields_bind_the_record_itself() {
    run(r#"module Main where
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Node, {next :: Option Node}))
  main :: Int
  main = let row = Node {next = None} in case row.next of
    None -> 42
    Some next -> 0
"#);
}

#[test]
fn native_record_fields_bind_only_preceding_telescope_values() {
    let source = r#"module Main where
  import Data.Bytes
  macro declare :: Name -> Syntax -> Declarations
  macro declare name fields = Syntax.recordDeclaration name fields
  generate do
    emit (declare!(Packet, {bytes :: Bytes; erased bound :: bytes-len bytes == 3}))
  main :: Int
  main = let buffer = bytes-new 3 in
    case decide (bytes-len buffer == 3) of
      Yes evidence -> let row = Packet {bytes = buffer, bound = evidence} in
        if bytes-len row.bytes == 3 then 42 else 0
      No _ -> 0
"#;
    run(source);
    refuses(&source.replace("bytes :: Bytes; erased bound :: bytes-len bytes == 3", "erased bound :: bytes-len bytes == 3; bytes :: Bytes"), &["unknown index bytes"]);
}

#[test]
fn declaration_quotations_bind_their_own_data_and_record_constructors() {
    run(r#"module Main where
  generate do
    emit (quoteDecls where
      data Box where
        MkBox :: Int -> Box
      record Envelope where
        payload :: Box
      answer :: Int
      answer = case (Envelope {payload = MkBox 42}).payload of MkBox value -> value)
  main :: Int
  main = answer
"#);
}

#[test]
fn declaration_quotation_index_helpers_keep_definition_site_authority() {
    let source = r#"module Library where
  export declare, Tagged, Tag, known
  measure :: Int -> Nat
  measure value = 1
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  known :: Tagged (measure 7)
  known = Tag 42
  macro declare :: Syntax -> Declarations
  macro declare ignored = quoteDecls where
    record Envelope where
      payload :: Tagged (measure 7)
module Main where
  import Library
  import stage Library
  measure :: Int -> Nat
  measure value = 2
  generate do
    emit (declare!(0))
  main :: Int
  main = let row = Envelope {payload = known} in case row.payload of Tag value -> value
"#;
    run(source);
    let unbound = source.replace("payload :: Tagged (measure 7)", "payload :: Tagged (absent 7)")
        .replace("  measure value = 2", "  measure value = 2\n  absent :: Int -> Nat\n  absent value = 1")
        .replace("main = let row = Envelope {payload = known} in case row.payload of Tag value -> value", "main = 42");
    refuses(&unbound, &["absent", "does not resolve where this template was defined"]);
}

#[test]
fn declaration_quotation_field_telescopes_keep_local_binders() {
    let source = r#"module Main where
  import Data.Bytes
  generate do
    emit (quoteDecls where
      record Packet where
        bytes :: Bytes
        erased bound :: bytes-len bytes == 3)
  main :: Int
  main = let buffer = bytes-new 3 in case decide (bytes-len buffer == 3) of
    Yes evidence -> let row = Packet {bytes = buffer, bound = evidence} in
      if bytes-len row.bytes == 3 then 42 else 0
    No _ -> 0
"#;
    run(source);
    refuses(&source.replace("bytes :: Bytes\n        erased bound :: bytes-len bytes == 3", "erased bound :: bytes-len bytes == 3\n        bytes :: Bytes"), &["bytes", "does not resolve where this template was defined"]);
}

#[test]
fn native_declaration_construction_cannot_escape_into_runtime() {
    refuses(r#"module Main where
  main :: Int
  main = let construct = Syntax.recordDeclaration in 42
"#, &["compile-time only"]);
}

#[test]
fn declaration_quotation_rows_and_regions_follow_their_renamed_binders() {
    run(r#"module Main where
  generate do
    emit (quoteDecls where
      invoke :: forall e. (Unit ->{|e} Int) ->{|e} Int
      invoke action = action ())
  main :: Int
  main = invoke (\unit -> 42)
"#);
    let source = r#"module Main where
  generate do
    emit (quoteDecls where
      invoke :: forall r e. Ref r Int -> (Unit ->{Read r | e} Int) ->{Read r | e} Int
      invoke value action = action ())
  main :: Int
  main = 42
"#;
    check_sources(source, "quoted-region-row.cinch").unwrap();
}

#[test]
fn declaration_quotation_effect_parameters_scope_their_operation_types() {
    run(r#"module Main where
  generate do
    emit (quoteDecls where
      effect Sample r where
        sample :: Unit ->{Sample r} Int)
  main :: Int
  main = 42
"#);
}

#[test]
fn declaration_quotation_builtin_propositions_keep_their_native_reading() {
    let source = r#"module Main where
  generate do
    emit (quoteDecls where
      proof equal :: Eq (2 + 3) 5
      proof equal = refl
      consume :: Eq (2 + 3) 5 -> Int
      consume evidence = 42)
  main :: Int
  main = consume equal
"#;
    run(source);
    refuses(&source.replace("Eq (2 + 3) 5", "Eq (2 + 3) 6"), &["obligation disproved"]);
}

#[test]
fn declaration_quotation_capitalized_constructor_parameters_are_bound() {
    run(r#"module Main where
  generate do
    emit (quoteDecls where
      data Wrap F a where
        Wrap :: F a -> Wrap F a
      extract :: forall f a. Wrap f a -> f a
      extract wrapper = case wrapper of Wrap value -> value)
  main :: Int
  main = case (extract (Wrap (Some (21 :: Int))) :: Option Int) of
    Some value -> case (extract (Wrap [value, value]) :: List Int) of
      [left, right] -> left + right
      _ -> 0
    None -> 0
"#);
}

#[test]
fn declaration_generation_keeps_stage_and_macro_phase_barriers_under_attributes() {
    for attribute in ["", "#[inline] "] {
        refuses(&format!(r#"module Main where
  generate do
    emit (quoteDecls where
      {attribute}stage hidden :: Int
      stage hidden = 42)
  main :: Int
  main = 42
"#), &["a generator cannot emit `generate`, `stage` or `macro`"]);
    }
    refuses(r#"module Main where
  generate do
    emit (quoteDecls where
      macro hidden :: Syntax -> Code Int
      macro hidden source = [| 42 |])
  main :: Int
  main = 42
"#, &["a generator cannot emit `generate`, `stage` or `macro`"]);
}
