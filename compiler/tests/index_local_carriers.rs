//! Ordinary index annotations use checked lexical values and their carriers.
use cinch::driver::{check_sources, run_source_with, StagingCx};

const SOURCE: &str = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: Int -> Nat
  measure value = let unit = ignore value in 40
  macro wrap :: Syntax -> Code Int
  macro wrap arg = Syntax.capture arg
  use :: (payload :: Int) -> Tagged (measure payload) -> Int
  use payload tag = case (tag :: Tagged (measure payload)) of { Tag value -> value }
  main :: Int
  main = use 7 (Tag 42)
"#;

#[test]
fn ordinary_index_annotations_resolve_locals_and_check_their_carriers() {
    let program = SOURCE;
    assert_eq!(
        run_source_with(&program, &mut StagingCx::default()).unwrap(),
        42
    );
    let wrong = program
        .replace("(payload :: Int)", "(payload :: Bytes)")
        .replace(
            "main = use 7 (Tag 42)",
            "main = let buffer = bytes-new 1 in use buffer (Tag 42)",
        );
    let error = check_sources(&wrong, "wrong-ordinary-index-carrier.cinch")
        .err()
        .expect("ordinary index annotations must retain their argument's actual carrier")
        .to_string();
    assert!(error.contains("Int") && error.contains("Bytes"), "{error}");
}
