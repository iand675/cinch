//! Composition links owned declarations; captured syntax keeps source scope.
mod common;

use common::source_frontend::{check_sources, run_source};

#[track_caller]
fn run(source: &str) {
    assert_eq!(run_source(source).unwrap(), 42);
}
#[track_caller]
fn refuses(source: &str, needle: &str) {
    let error = check_sources(source, "declaration-composition.cinch").err().expect("must refuse").to_string();
    assert!(error.contains(needle), "{error}");
}

#[test]
fn composed_protocol_states_and_transitions_use_actual_owned_members() {
    let source = format!("{}\n{}", include_str!("../../experiments/malleable-syntax/libraries/ProtocolSyntax.cinch"), r#"module Main where
  import stage ProtocolSyntax
  generate do
    emit (ProtocolSyntax.declare!(([Offered, Pending, Complete]), ([(authorize, Offered, Pending), (confirm, Pending, Complete)])))
  main :: Int
  main = case confirm (authorize (Offered 0)) of
    Complete count -> if count == 2 then 42 else 0
"#);
    run(&source);
    refuses(&source.replace("confirm (authorize (Offered 0))", "confirm (Offered 0)"), "type mismatch");
    let absent = source.replace("(confirm, Pending, Complete)", "(confirm, Pending, Missing)")
        .replace("module Main where", "module Main where\n  data Missing where\n    Missing :: Nat -> Missing");
    refuses(&absent, "no owned");
}

#[test]
fn syntax_sequence_readers_preserve_lexical_captures_and_nested_delimiters() {
    let source = r#"module Library where
  export first
  value :: Bool
  value = False
  macro first :: Syntax -> Code Int
  macro first input = case Syntax.items input of
    [operand, ignored] -> Syntax.capture operand
    _ -> Syntax.error "expected two captured operands"
module Main where
  import stage Library
  main :: Int
  main = let value = 42 in first!(((value), (("comma, in text", [1, 2]))))
"#;
    run(source);
    refuses(&source.replace("((value), ((\"comma, in text\", [1, 2])))", "[]"), "expected two captured operands");
}

#[test]
fn syntax_name_is_descriptive_and_does_not_acquire_a_private_definition() {
    refuses(r#"module Library where
  export construct
  helper :: Int
  helper = 42
  macro construct :: Syntax -> Declarations
  macro construct input = Syntax.renameDeclarations (Syntax.name input) (sym-to-name 'result)
    (quoteDecls where { answer :: Int; answer = helper })
module Main where
  import stage Library
  generate do
    emit (construct!((helper)))
  main :: Int
  main = 42
"#, "not an owned member");
}

#[test]
fn linking_a_type_refuses_dangling_constructor_references_instead_of_rebinding_them() {
    refuses(r#"module Main where
  generate do
    let context = quoteDecls where { data Destination where { Destination :: Int -> Destination } }
    let template = quoteDecls where {
      data Placeholder where { MkPlaceholder :: Int -> Placeholder };
      make :: Int -> Placeholder;
      make value = MkPlaceholder value }
    emit (Syntax.appendDeclarations context
      (Syntax.linkDeclarations (sym-to-name 'Placeholder) (sym-to-name 'Destination) context template))
  main :: Int
  main = 42
"#, "unbound owned member");
}

#[test]
fn linking_refuses_ambiguous_context_members() {
    refuses(r#"module Main where
  generate do
    let context = (Syntax.appendDeclarations
      (quoteDecls where { data Destination where { Destination :: Int -> Destination } })
      (quoteDecls where { data Destination where { Destination :: Bool -> Destination } }))
    let template = quoteDecls where { data Placeholder where { Placeholder :: Int -> Placeholder } }
    emit (Syntax.linkDeclarations (sym-to-name 'Placeholder) (sym-to-name 'Destination) context template)
  main :: Int
  main = 42
"#, "ambiguous owned declaration member");
}

#[test]
fn native_syntax_readers_and_errors_keep_the_compile_time_boundary() {
    refuses(r#"module Main where
  main :: Int
  main = Syntax.error "runtime escape"
"#, "compile-time only");
    refuses(r#"module Main where
  macro escape :: Syntax -> Code (List Syntax)
  macro escape input = [| $(Syntax.items input) |]
  main :: Int
  main = let leaked = escape!(([a, b])) in 42
"#, "produces List Syntax");
}

#[test]
fn machine_payload_types_and_transition_handlers_keep_caller_bindings() {
    let source = format!("{}\n{}", include_str!("../../experiments/malleable-syntax/libraries/MachineSyntax.cinch"), r#"module Main where
  import stage MachineSyntax
  data Payload where
    Payload :: Int -> Payload
  transform :: Payload -> (Int, Bool)
  transform value = case value of
    Payload amount -> (amount + 1, True)
  generate do
    emit (MachineSyntax.declare!(([(Editing, Payload), (Reviewing, (Int, Bool)), (Complete, Int)]), ([(review, Editing, Reviewing, transform), (submit, Reviewing, Complete, ((\pair -> case pair of { (amount, flag) -> if flag then amount + 1 else 0 }) :: (Int, Bool) -> Int))])))
  main :: Int
  main = case submit (review (Editing (Payload 40))) of
    Complete answer -> answer
"#);
    run(&source);
    refuses(&source.replace("submit (review (Editing (Payload 40)))", "submit (Editing (Payload 40))"), "type mismatch");
    refuses(&source.replace("Reviewing, transform)", "Reviewing, (\\value -> True))"), "type mismatch");
}

#[test]
fn specializing_owned_types_preserves_private_source_types_across_insertions() {
    let source = r#"module Library where
  export declare
  data Payload where
    Payload :: Bool -> Payload
  macro declare :: Syntax -> Declarations
  macro declare payload = Syntax.specializeDeclarationType (sym-to-name 'Slot) payload
    (quoteDecls where { opaque type Slot :: Type; data Wrapped where { Wrapped :: Slot -> Wrapped } })
module Client where
  export generated, make, read
  import stage Library
  data Payload where
    Payload :: Int -> Payload
  stage generated :: Declarations
  stage generated = Library.declare!((Payload))
  make :: Int -> Payload
  make value = Payload value
  read :: Payload -> Int
  read value = case value of
    Payload amount -> amount
module Main where
  import stage Client
  import Client
  data Payload where
    Payload :: Bool -> Payload
  generate do
    emit Client.generated
  main :: Int
  main = case Wrapped (make 42) of
    Wrapped value -> read value
"#;
    run(source);
    refuses(&source.replace("Wrapped (make 42)", "Wrapped (Payload True :: Payload)"), "type mismatch");
    refuses(&source.replace("(sym-to-name 'Slot) payload", "(sym-to-name 'Payload) payload"), "not an owned opaque type slot");
}

#[test]
fn opaque_type_quotation_renames_owned_uses_without_capturing_source_names() {
    run(r#"module Main where
  generate do
    emit (Syntax.renameDeclarations (sym-to-name 'Hidden) (sym-to-name 'Token)
      (quoteDecls where { opaque type Hidden :: Type; identity :: Hidden -> Hidden; identity value = value }))
  main :: Int
  main = 42
"#);
}

#[test]
fn combined_groups_can_be_reused_and_renamed_without_retargeting_other_instances() {
    run(r#"module Main where
  stage family :: Declarations
  stage family =
    let context = quoteDecls where { data Destination where { Destination :: Int -> Destination } } in
    let template = quoteDecls where {
      data Placeholder where { Placeholder :: Int -> Placeholder };
      unwrap :: Placeholder -> Int;
      unwrap value = case value of { Placeholder amount -> amount } } in
    Syntax.appendDeclarations context
      (Syntax.linkDeclarations (sym-to-name 'Placeholder) (sym-to-name 'Destination) context template)
  generate do
    let base = family
    emit (Syntax.renameDeclarations (sym-to-name 'unwrap) (sym-to-name 'readFirst)
      (Syntax.renameDeclarations (sym-to-name 'Destination) (sym-to-name 'First) base))
    emit (Syntax.renameDeclarations (sym-to-name 'unwrap) (sym-to-name 'readSecond)
      (Syntax.renameDeclarations (sym-to-name 'Destination) (sym-to-name 'Second) base))
  main :: Int
  main = readFirst (First 20) + readSecond (Second 22)
"#);
}

#[test]
fn captured_type_constructors_specialize_higher_kinded_slots_under_forall() {
    let source = r#"module Main where
  macro declare :: Syntax -> Declarations
  macro declare constructor = Syntax.specializeDeclarationType (sym-to-name 'Slot) constructor
    (quoteDecls where {
      opaque type Slot :: Type -> Type;
      identity :: forall a. Slot a -> Slot a;
      identity value = value })
  generate do
    emit (declare!((List)))
  main :: Int
  main = case identity [42] of
    [answer] -> answer
    _ -> 0
"#;
    run(source);
    refuses(&source.replace("declare!((List))", "declare!((Bool))"), "unknown type Bool");
}

#[test]
fn linking_within_one_possessed_group_follows_its_instantiated_anchors() {
    run(r#"module Main where
  generate do
    let group = quoteDecls where {
      opaque type Slot :: Type;
      data Payload where { Payload :: Int -> Payload };
      data Wrapped where { Wrapped :: Slot -> Wrapped } }
    emit (Syntax.linkDeclarations (sym-to-name 'Slot) (sym-to-name 'Payload) group group)
  main :: Int
  main = case Wrapped (Payload 42) of
    Wrapped (Payload answer) -> answer
"#);
}
