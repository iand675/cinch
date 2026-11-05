//! Index applications retain their complete checked evidence telescope.
use cinch::driver::{check_sources, run_source_with, StagingCx};

fn source(constraint: &str, captured: bool) -> String {
    let body = "case (tag :: Tagged (measure payload)) of { Tag value -> value }";
    let body = if captured {
        format!("wrap!(({body}))")
    } else {
        body.into()
    };
    format!(
        r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: forall a. using ({constraint}) => a -> Nat
  measure value = let unit = ignore value in 40
  macro wrap :: Syntax -> Code Int
  macro wrap arg = Syntax.capture arg
  use :: (payload :: Int) -> Tagged (measure payload) -> Int
  use payload tag = {body}
  main :: Int
  main = use 7 (Tag 42)
"#
    )
}

#[test]
fn captured_index_applications_accept_available_dictionary_evidence() {
    for captured in [false, true] {
        assert_eq!(
            run_source_with(&source("eq :: Eq a", captured), &mut StagingCx::default()).unwrap(),
            42
        );
    }
}

#[test]
fn captured_index_applications_use_an_explicit_lexical_dictionary() {
    for captured in [false, true] {
        let program = source("eq :: Eq a", captured).replace(
            "use :: (payload :: Int)",
            "use :: using (eq :: Eq Int) => (payload :: Int)",
        );
        assert_eq!(run_source_with(&program, &mut StagingCx::default()).unwrap(), 42);
    }
}

#[test]
fn index_applications_require_their_dictionary_even_outside_macros() {
    for captured in [false, true] {
        let program = source("eq :: Eq a", captured)
            .replace(
                "  data Tagged",
                "  data Token where\n    Token :: Int -> Token\n  data Tagged",
            )
            .replace("(payload :: Int)", "(payload :: Token)")
            .replace("main = use 7", "main = use (Token 7)");
        let error = check_sources(&program, "missing-index-dictionary.cinch")
            .err()
            .expect("type-level application must check its dictionary requirements")
            .to_string();
        assert!(error.contains("Eq") && error.contains("Token"), "{error}");
    }
}

#[test]
fn captured_index_applications_discharge_true_proof_obligations() {
    for captured in [false, true] {
        assert_eq!(
            run_source_with(&source("0 == 0", captured), &mut StagingCx::default()).unwrap(),
            42
        );
    }
}

#[test]
fn index_applications_cannot_discard_false_proof_obligations() {
    for captured in [false, true] {
        let error = check_sources(&source("0 == 1", captured), "false-index-obligation.cinch")
            .err()
            .expect("an index application cannot erase its function's false obligation")
            .to_string();
        assert!(error.contains("0") && error.contains("1"), "{error}");
    }
}

#[test]
fn captured_index_applications_retain_the_selected_lexical_dictionary() {
    let program = lexical_dictionary_source();
    for provider in ["Forty", "FortyOne"] {
        let source = program.replace(
            "measureDict = Forty)",
            &format!("measureDict = {provider})"),
        );
        assert_eq!(
            run_source_with(&source, &mut StagingCx::default()).unwrap(),
            42
        );
    }
    let substituted = program.replace(
        "use payload tag = wrap!",
        "use payload tag = with (measureDict = FortyOne) in wrap!",
    );
    let error = check_sources(&substituted, "different-index-dictionary.cinch")
        .err()
        .expect("the index computed with a different dictionary is not the parameter's index")
        .to_string();
    assert!(
        error.contains("index") || error.contains("dictionary"),
        "{error}"
    );
}

fn lexical_dictionary_source() -> &'static str {
    r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  signature IndexedMeasure a where
    readMeasure :: a -> Nat
  instance Forty :: IndexedMeasure Int where
    readMeasure value = 40
  instance FortyOne :: IndexedMeasure Int where
    readMeasure value = 41
  measure :: using (measureDict :: IndexedMeasure Int) => Int -> Nat
  measure value = readMeasure value
  macro wrap :: Syntax -> Code Int
  macro wrap arg = Syntax.capture arg
  use :: using (measureDict :: IndexedMeasure Int) => (payload :: Int) -> Tagged (measure payload) -> Int
  use payload tag = wrap!((case (tag :: Tagged (measure payload)) of { Tag value -> value }))
  main :: Int
  main = with (measureDict = Forty) in use 7 (Tag 42)
"#
}

#[test]
fn captured_index_keeps_its_source_provider_under_a_generated_provider() {
    let source = lexical_dictionary_source().replace(
        "macro wrap arg = Syntax.capture arg",
        "macro wrap arg = let captured = Syntax.capture arg in [| with (measureDict = FortyOne) in $(captured) |]",
    );
    assert_eq!(run_source_with(&source, &mut StagingCx::default()).unwrap(), 42);
}

#[test]
fn copied_generic_functions_retain_their_inner_dictionary_scope() {
    let source = lexical_dictionary_source().replace(
        "  main :: Int\n  main = with (measureDict = Forty) in use 7 (Tag 42)",
        r#"  probe :: using (measureDict :: IndexedMeasure Int) => Int
  probe = if readMeasure 7 == (40 :: Nat) then 42 else 0
  main :: Int
  main = let generic item = with (measureDict = Forty) in (item, use 7 (Tag probe)) in case generic True of { (_, left) -> case generic (0 :: U32) of { (_, right) -> left + right } }"#,
    );
    assert_eq!(run_source_with(&source, &mut StagingCx::default()).unwrap(), 84);
}

#[test]
fn declaration_evidence_can_select_a_later_user_instance() {
    let program = lexical_dictionary_source()
        .replace("  instance Forty :: IndexedMeasure Int where\n    readMeasure value = 40\n", "")
        .replace("use :: using (measureDict :: IndexedMeasure Int) =>", "use ::")
        .replace("main = with (measureDict = Forty) in", "main =");
    let program = format!("{program}\n  implicit instance Forty :: IndexedMeasure Int where\n    readMeasure value = 40\n");
    assert_eq!(run_source_with(&program, &mut StagingCx::default()).unwrap(), 42);
}

#[test]
fn declaration_evidence_does_not_discard_proof_obligations() {
    for captured in [false, true] {
        let error = check_sources(&source("eq :: Eq a, 0 == 1", captured), "false-later-evidence.cinch")
            .err().expect("the strict declaration pass must check the false obligation").to_string();
        assert!(error.contains("0") && error.contains("1"), "{error}");
    }
}

#[test]
fn declaration_evidence_selects_the_later_refinement_specific_instance() {
    let program = r#"module Main where
  type Small = refine (n :: Nat) with n < 10
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  signature Count a where
    readCount :: a -> Nat
  measure :: forall a. using (count :: Count a) => a -> Nat
  measure value = readCount value
  macro wrap :: Syntax -> Code Int
  macro wrap arg = Syntax.capture arg
  use :: (payload :: Small) -> Tagged (measure payload) -> Int
  use payload tag = wrap!((case (tag :: Tagged (measure payload)) of { Tag value -> value }))
  main :: Int
  main = case decide ((7 :: Nat) < 10) of { Yes witness -> let small = Small 7 witness in if readCount small == (41 :: Nat) then use small (Tag 42) else 0; No _ -> 0 }
  implicit instance Base :: Count Nat where
    readCount value = 40
  implicit instance Specific :: Count Small where
    readCount value = 41
"#;
    // Numeric comparison refinements are carrier facts, so unconstrained
    // generic calls use Nat's instance. Semantic points retain their own.
    assert_eq!(run_source_with(program, &mut StagingCx::default()).unwrap(), 0);
    let semantic = program.replace("n < 10", "NonZero n").replace("((7 :: Nat) < 10)", "(NonZero (7 :: Nat))");
    assert_eq!(run_source_with(&semantic, &mut StagingCx::default()).unwrap(), 42);
    let base_only = semantic.replace("  implicit instance Specific :: Count Small where\n    readCount value = 41\n", "");
    assert_eq!(run_source_with(&base_only, &mut StagingCx::default()).unwrap(), 0);
}

#[test]
fn declaration_evidence_is_shared_consistently_across_dependent_headers() {
    let program = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: forall a. using (eq :: Eq a) => a -> Nat
  measure value = let unit = ignore value in 40
  makeTag :: (payload :: Int) -> Tagged (measure payload)
  makeTag payload = Tag 42
  readTag :: (payload :: Int) -> Tagged (measure payload) -> Nat
  readTag payload tag = let unit = ignore tag in 0
  use :: (payload :: Int) -> Tagged (readTag payload (makeTag payload)) -> Int
  use payload tag = case (tag :: Tagged (readTag payload (makeTag payload))) of { Tag value -> value }
  main :: Int
  main = use 7 (Tag 42)
"#;
    assert_eq!(run_source_with(program, &mut StagingCx::default()).unwrap(), 42);
}

#[test]
fn declaration_evidence_cannot_turn_recursive_instances_into_proofs() {
    for captured in [false, true] {
        let program = source("requires :: Recursive a", captured).replace(
            "  data Tagged",
            "  signature Recursive a where\n    readRecursive :: a -> Nat\n  implicit instance Loop a :: using (again :: Recursive a) => Recursive a where\n    readRecursive value = let unit = ignore value in 40\n  data Tagged",
        );
        let error = check_sources(&program, "recursive-declaration-evidence.cinch")
            .err().expect("recursive instance requirements must not become provisional evidence").to_string();
        assert!(error.contains("recursive instance") || error.contains("dictionary resolution too deep"), "{error}");
        let nonrecursive = program.replace("using (again :: Recursive a) => ", "");
        assert_eq!(run_source_with(&nonrecursive, &mut StagingCx::default()).unwrap(), 42);
    }
}

#[test]
fn declaration_evidence_can_determine_a_type_from_multiple_constraints() {
    let program = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  signature OnlyInt a where
    witness :: ()
  makeIndex :: forall a. using (eq :: Eq a, specific :: OnlyInt a) => Nat
  makeIndex = 40
  use :: Tagged makeIndex -> Int
  use tag = case (tag :: Tagged makeIndex) of { Tag value -> value }
  main :: Int
  main = use (Tag 42)
  implicit instance One :: OnlyInt Int where
    witness = ()
"#;
    assert_eq!(run_source_with(program, &mut StagingCx::default()).unwrap(), 42);
    let missing = program.replace("  implicit instance One :: OnlyInt Int where\n    witness = ()\n", "");
    let error = check_sources(&missing, "missing-specific-evidence.cinch")
        .err().expect("unresolved constraints must not choose a provisional type").to_string();
    assert!(error.contains("OnlyInt"), "{error}");
}
