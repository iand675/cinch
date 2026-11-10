//! Structural declaration builders preserve owned and captured identities.
mod common;
use common::source_frontend::{check_sources, run_source};

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}

#[track_caller]
fn refuses(source: &str, needle: &str) {
    let error = check_sources(source, "structural-builders.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(needle), "{error}");
}

const FAMILY: &str = r#"module Builders where
  export family
  macro family :: Syntax -> Declarations
  macro family payload =
    let base = quoteDecls where { data Choice where {}; choose :: Choice -> Int } in
    let choiceType = Syntax.declarationType (sym-to-name 'Choice) base in
    let fieldType = Syntax.functionType (Syntax.captureType payload) choiceType in
    let context = Syntax.addConstructor (sym-to-name 'Choice) (sym-to-name 'Accept) fieldType base in
    let clause = quoteDecls where {
      data Placeholder where { MkPlaceholder :: Int -> Placeholder };
      branch (MkPlaceholder value) = value } in
    let linked = Syntax.linkDeclarations (sym-to-name 'Placeholder) (sym-to-name 'Choice) context
          (Syntax.linkDeclarations (sym-to-name 'MkPlaceholder) (sym-to-name 'Accept) context clause) in
    let assembled = Syntax.addEquations (sym-to-name 'branch) (sym-to-name 'choose) context linked in
    let constructor = Syntax.declarationReference (sym-to-name 'Accept) assembled in
    let resultType = Syntax.declarationType (sym-to-name 'Choice) assembled in
    let maker = Syntax.specializeType (sym-to-name 'Result) resultType
          (quoteDecls where { opaque type Result :: Type; make :: Int -> Result; make value = $(constructor) value }) in
    Syntax.exportDeclaration (sym-to-name 'make)
      (Syntax.exportDeclaration (sym-to-name 'choose)
        (Syntax.exportDeclaration (sym-to-name 'Choice) (Syntax.appendDeclarations assembled maker)))
"#;

#[test]
fn native_type_constructor_clause_and_export_builders_form_a_private_family() {
    let source = format!("{FAMILY}{}", r#"module Library where
  import stage Builders
  generate do
    emit (family!((Int)))
module Main where
  import Library
  data Choice where
    Choice :: Bool -> Choice
  main :: Int
  main = choose (make 42)
"#);
    run(&source);
    refuses(&source.replace("choose (make 42)", "choose (Library.Accept 42)"), "Accept");
    refuses(&source.replace("family!((Int))", "family!((Bool))"), "type mismatch");
}

#[test]
fn constructor_extension_keeps_shared_type_anchors_and_checks_its_result() {
    let source = r#"module Main where
  macro family :: Syntax -> Declarations
  macro family input =
    let base = quoteDecls where { data Packet where {} } in
    let result = Syntax.declarationType (sym-to-name 'Packet) base in
    Syntax.addConstructor (sym-to-name 'Packet) (sym-to-name 'Wrap)
      (Syntax.functionType (Syntax.captureType input) result) base
  generate do
    emit (family!((Int)))
  main :: Int
  main = case Wrap 42 of
    Wrap answer -> answer
"#;
    run(source);
    refuses(&source.replace("(Syntax.captureType input) result", "(Syntax.captureType input) (Syntax.captureType input)"), "must return");
    refuses(&source.replace("sym-to-name 'Wrap", "sym-to-name 'wrap"), "constructor identifier");
}

#[test]
fn native_type_application_and_tuples_keep_their_scoped_trees() {
    run(r#"module Main where
  macro family :: Syntax -> Syntax -> Declarations
  macro family constructor element =
    let applied = Syntax.applyType (Syntax.captureType constructor) (Syntax.captureType element) in
    let pair = Syntax.tupleType [applied, Syntax.captureType element] in
    Syntax.specializeType (sym-to-name 'Slot) pair
      (quoteDecls where { opaque type Slot :: Type; data Box where { Box :: Slot -> Box } })
  generate do
    emit (family!((List), (Int)))
  main :: Int
  main = case Box ([20], 22) of
    Box ([left], right) -> left + right
    _ -> 0
"#);
}

#[test]
fn heterogeneous_code_tuples_keep_caller_and_definition_bindings() {
    run(r#"module Library where
  export tuple
  hidden :: Int
  hidden = 20
  macro tuple :: Syntax -> Code (Int, Bool, Int)
  macro tuple source = Syntax.tupleCode
    [Syntax.packCode [| hidden |], Syntax.packCode [| True |], Syntax.packCode (Syntax.capture source)]
module Main where
  import stage Library
  hidden :: Bool
  hidden = False
  main :: Int
  main = let hidden = 22 in case tuple!((hidden)) of
    (left, flag, right) -> if flag then left + right else 0
"#);
    run(r#"module Main where
  macro empty :: Code Unit
  macro empty = Syntax.tupleCode []
  macro one :: Syntax -> Code Int
  macro one source = Syntax.tupleCode [Syntax.packCode (Syntax.capture source)]
  main :: Int
  main = case empty!() of
    () -> one!((42))
"#);
}

#[test]
fn packing_code_preserves_carrier_and_linear_expectations() {
    refuses(r#"module Main where
  stage wrong :: Code Int
  stage wrong = [| True |]
  macro tuple :: Code (Bool, Bool)
  macro tuple = Syntax.tupleCode [Syntax.packCode wrong, Syntax.packCode [| False |]]
  main :: Int
  main = case tuple!() of
    (left, right) -> if left then 42 else 0
"#, "Int");
    refuses(r#"module Main where
  stage wrong :: Code (Nat %1 -> Nat)
  stage wrong = [| \value -> value + value |]
  macro tuple :: Code ((Nat -> Nat), Bool)
  macro tuple = Syntax.tupleCode [Syntax.packCode wrong, Syntax.packCode [| True |]]
  main :: Int
  main = case tuple!() of
    (twice, flag) -> if twice 21 == 42 then 42 else 0
"#, "%1");
}

#[test]
fn structural_metadata_cannot_escape_through_runtime_code() {
    refuses(r#"module Main where
  macro leak :: Syntax -> Code TypeSyntax
  macro leak input = [| $(Syntax.captureType input) |]
  main :: Int
  main = let leaked = leak!((Int)) in 42
"#, "produces TypeSyntax");
    refuses(r#"module Main where
  macro leak :: Syntax -> Code PackedCode
  macro leak input = [| $(Syntax.packCode (Syntax.capture input)) |]
  main :: Int
  main = let leaked = leak!((42)) in 42
"#, "produces PackedCode");
}

#[test]
fn names_cannot_acquire_free_private_members_from_a_possessed_group() {
    refuses(r#"module Library where
  export forge
  hidden :: Int
  hidden = 42
  macro forge :: Syntax -> Code Int
  macro forge input = Syntax.declarationReference (Syntax.name input)
    (quoteDecls where { answer :: Int; answer = hidden })
module Main where
  import stage Library
  main :: Int
  main = forge!((hidden))
"#, "no owned Value");
    refuses(r#"module Main where
  data Hidden where { Hidden :: Int -> Hidden }
  macro forge :: Syntax -> Declarations
  macro forge input =
    let context = quoteDecls where { identity :: Hidden -> Hidden; identity value = value } in
    Syntax.specializeType (sym-to-name 'Slot) (Syntax.declarationType (Syntax.name input) context)
      (quoteDecls where { opaque type Slot :: Type; data Box where { Box :: Slot -> Box } })
  generate do
    emit (forge!((Hidden)))
  main :: Int
  main = 42
"#, "no owned Type");
}

#[test]
fn adding_equations_retains_target_checks_and_cannot_discard_source_contracts() {
    let source = r#"module Main where
  generate do
    let context = quoteDecls where { answer :: Bool -> Int }
    let clauses = quoteDecls where { branch True = 42; branch False = 0 }
    emit (Syntax.addEquations (sym-to-name 'branch) (sym-to-name 'answer) context clauses)
  main :: Int
  main = answer True
"#;
    run(source);
    refuses(&source.replace("branch True = 42", "branch True = True"), "type mismatch");
    refuses(&source.replace("branch True = 42;", "branch :: Bool -> Int; branch True = 42;"), "cannot discard a source signature");
}

#[test]
fn descriptive_name_join_and_numeric_suffix_construct_only_valid_names() {
    let source = r#"module Main where
  macro declare :: Name -> Declarations
  macro declare protocol =
    let state = Name.append protocol (Name.suffix (sym-to-name 'P) (Stage.natStr 2)) in
    let function = Name.prefix "start-" (Name.join "-" protocol (sym-to-name 'Buyer)) in
    Syntax.renameDeclarations (sym-to-name 'answer) function
      (Syntax.renameDeclarations (sym-to-name 'State) state
        (quoteDecls where { data State where { State :: Int -> State }; answer :: State -> Int; answer (State value) = value }))
  generate do
    emit (declare!(Checkout))
  main :: Int
  main = start-Checkout-Buyer (CheckoutP2 42)
"#;
    run(source);
    refuses(&source.replace("Name.join \"-\"", "Name.join \"), injected!(\""), "one identifier");
}

#[test]
fn structural_specialization_and_signature_readers_keep_internal_and_private_type_owners() {
    let source = r#"module Library where
  export template, make
  data Payload where { Payload :: Int -> Payload }
  make :: Int -> Payload
  make value = Payload value
  stage template :: Declarations
  stage template = quoteDecls where { inspect :: Payload -> Int; inspect (Payload value) = value }
module Main where
  import stage Library
  import Library
  data Payload where { Payload :: Bool -> Payload }
  generate do
    let group = quoteDecls where {
      opaque type Slot :: Type;
      data Local where { Local :: Int -> Local };
      data Box where { Box :: Slot -> Box } }
    emit (Syntax.specializeType (sym-to-name 'Slot) (Syntax.declarationType (sym-to-name 'Local) group) group)
    let callbackType = Syntax.signatureType (sym-to-name 'inspect) Library.template
    emit (Syntax.specializeType (sym-to-name 'Slot) callbackType
      (quoteDecls where { opaque type Slot :: Type; data Callback where { Callback :: Slot -> Callback } }))
    emit Library.template
  main :: Int
  main = case Box (Local 20) of
    Box (Local left) -> case Callback inspect of
      Callback read -> left + read (make 22)
"#;
    run(source);
    refuses(&source.replace("make 22", "(Payload True :: Payload)"), "type mismatch");
    run(r#"module Main where
  generate do
    let constructor = Syntax.signatureType (sym-to-name 'value) (quoteDecls where { value :: Thunk {} })
    let element = Syntax.signatureType (sym-to-name 'value) (quoteDecls where { value :: Int })
    emit (Syntax.specializeType (sym-to-name 'Slot) (Syntax.applyType constructor element)
      (quoteDecls where { opaque type Slot :: Type; data Box where { Box :: Slot -> Box } }))
  main :: Int
  main = case Box (delay (\() -> 42)) of
    Box value -> force value
"#);
}

#[test]
fn owned_function_code_localizes_clauses_recursion_and_generic_signatures() {
    run(r#"module Main where
  macro countdown :: Code (Int -> Int)
  macro countdown = Syntax.functionCode (sym-to-name 'step)
    (quoteDecls where { step :: Int -> Int; step 0 = 42; step value = step (value - 1) })
  main :: Int
  main = countdown!() 4
"#);
    run(r#"module Main where
  macro identity :: Code (Bool -> Bool)
  macro identity = Syntax.functionCode (sym-to-name 'same)
    (quoteDecls where { same :: forall a. a -> a; same value = value })
  main :: Int
  main = if identity!() True then 42 else 0
"#);
    run(r#"module Main where
  macro factory :: Syntax -> Code (Int -> Int)
  macro factory source =
    let captured = Syntax.capture source in
    Syntax.functionCode (sym-to-name 'dispatch)
      (quoteDecls where { dispatch :: Int -> Int; dispatch value = value + $(captured) })
  main :: Int
  main = let argument0 = 22 in factory!((argument0)) 20
"#);
}

#[test]
fn owned_function_code_keeps_source_signatures_and_refuses_discarded_attributes() {
    refuses(r#"module Main where
  macro wrong :: Code (Bool -> Bool)
  macro wrong = Syntax.functionCode (sym-to-name 'same)
    (quoteDecls where { same :: forall a. a -> a; same value = True })
  main :: Int
  main = if wrong!() True then 42 else 0
"#, "type mismatch");
    refuses(r#"module Main where
  macro wrong :: Code (Int -> Bool)
  macro wrong = Syntax.functionCode (sym-to-name 'step)
    (quoteDecls where { step :: Int -> Int; step value = True })
  main :: Int
  main = if wrong!() 0 then 42 else 0
"#, "type mismatch");
    refuses(r#"module Main where
  macro wrong :: Code (Int -> Int)
  macro wrong = Syntax.functionCode (sym-to-name 'step)
    (quoteDecls where { #[no-macros] step :: Int -> Int; step value = value })
  main :: Int
  main = wrong!() 42
"#, "cannot discard declaration attributes");
}

#[test]
fn quantified_function_code_checks_recursive_bodies_before_instantiation() {
    run(r#"module Main where
  macro repeat :: Code (Bool -> Bool)
  macro repeat = Syntax.functionCode (sym-to-name 'step)
    (quoteDecls where { step :: forall a. a -> a; step value = if True then value else step value })
  main :: Int
  main = if repeat!() True then 42 else 0
"#);
    refuses(r#"module Main where
  macro wrong :: Code (Bool -> Bool)
  macro wrong = Syntax.functionCode (sym-to-name 'step)
    (quoteDecls where { step :: forall a. a -> a; step value = if True then True else step value })
  main :: Int
  main = if wrong!() True then 42 else 0
"#, "type mismatch");
    refuses(r#"module Main where
  main :: Int
  main = let outer = \value -> ((\ignored -> value) :: forall a. a -> a) True in
    if outer True then 42 else 0
"#, "escapes into a captured binding");
}

#[test]
fn quantified_function_code_retains_rows_regions_and_universal_effect_checks() {
    run(r#"module Main where
  macro invoke :: Code ((Unit -> Int) -> Int)
  macro invoke = Syntax.functionCode (sym-to-name 'call)
    (quoteDecls where { call :: forall e. (Unit ->{|e} Int) ->{|e} Int; call action = action () })
  macro keep :: forall r. Code (Io r -> Io r)
  macro keep = Syntax.functionCode (sym-to-name 'same)
    (quoteDecls where { same :: forall r. Io r -> Io r; same value = value })
  retain :: forall r. Io r -> Io r
  retain value = keep!() value
  main :: Int
  main = invoke!() (\unit -> 42)
"#);
    refuses(r#"module Main where
  macro wrong :: Code ((Unit -> Int) -> Int)
  macro wrong = Syntax.functionCode (sym-to-name 'call)
    (quoteDecls where { call :: forall e. (Unit ->{|e} Int) -> Int; call action = action () })
  main :: Int
  main = wrong!() (\unit -> 42)
"#, "effect");
}
