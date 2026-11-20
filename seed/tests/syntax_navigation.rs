use cinch::analysis::Analysis;

fn position(source: &str, needle: &str) -> (u32, u32) {
    let offset = source.find(needle).expect("source occurrence");
    let before = &source[..offset];
    (
        before.bytes().filter(|b| *b == b'\n').count() as u32 + 1,
        before.rsplit('\n').next().unwrap().chars().count() as u32 + 1,
    )
}

#[test]
fn captured_global_navigation_survives_a_generated_same_spelled_binder() {
    let source = r#"module Main where
  greeting :: Int
  greeting = 42
  macro shadow :: Syntax -> Name -> Code Int
  macro shadow source name = let binding = Syntax.newBinding name in Syntax.letIn binding [| False |] (Syntax.capture source)
  main :: Int
  main = shadow!(greeting, greeting)
"#;
    let mut analysis = Analysis::new(std::env::temp_dir());
    analysis.set_document("Main.cinch", source);
    let report = analysis.update();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    let (line, col) = position(source, "greeting, greeting");
    let hover = analysis
        .hover("Main.cinch", line, col)
        .expect("captured source reference");
    assert!(hover.contains("Int") && !hover.contains("Bool"), "{hover}");
    assert_eq!(
        analysis.definition("Main.cinch", line, col),
        Some(("Main.cinch".into(), 2, 3))
    );
}

#[test]
fn definition_site_capture_navigates_to_its_owner_after_insertion_elsewhere() {
    let library = r#"module Library where
  export outer
  hidden :: Int
  hidden = 42
  macro capture :: forall a. Syntax -> Code a
  macro capture source = Syntax.capture source
  macro outer :: Syntax -> Code Int
  macro outer source = capture!(hidden)
"#;
    let main = r#"module Main where
  import stage Library
  hidden :: Bool
  hidden = False
  main :: Int
  main = outer!(0)
"#;
    let mut analysis = Analysis::new(std::env::temp_dir());
    analysis.set_document("Library.cinch", library);
    analysis.set_document("Main.cinch", main);
    let report = analysis.update();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    let (line, col) = position(library, "hidden)");
    let hover = analysis
        .hover("Library.cinch", line, col)
        .expect("definition-site reference");
    assert!(hover.contains("Int") && !hover.contains("Bool"), "{hover}");
    assert_eq!(
        analysis.definition("Library.cinch", line, col),
        Some(("Library.cinch".into(), 3, 3))
    );
}

#[test]
fn generated_api_uses_checked_types_and_navigates_to_its_emission_site() {
    let library = r#"module Library where
  export answer
  macro declare :: Unit -> Declarations
  macro declare unit = quoteDecls where { answer :: Int; answer = 42 }
  generate do
    emit (declare!(()))
"#;
    let main = "module Main where\n  import Library\n  main :: Int\n  main = answer\n";
    let mut analysis = Analysis::new(std::env::temp_dir());
    analysis.set_document("Library.cinch", library);
    analysis.set_document("Main.cinch", main);
    let report = analysis.update();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    let (line, col) = position(main, "answer");
    assert!(analysis.hover("Main.cinch", line, col).expect("generated API hover").contains("Int"));
    assert_eq!(analysis.definition("Main.cinch", line, col), Some(("Library.cinch".into(), 5, 3)));
    // All generated definitions share the emission destination. None is a
    // physical source token replacing the invocation at that destination.
    assert_eq!(analysis.hover("Library.cinch", 5, 3), None);

    let renamed = library.replace("answer", "replacement");
    let changed_main = main.replace("answer", "replacement");
    analysis.set_document("Library.cinch", &renamed);
    analysis.set_document("Main.cinch", &changed_main);
    let report = analysis.update();
    assert!(report.diagnostics.is_empty(), "{:?}", report.diagnostics);
    let (line, col) = position(&changed_main, "replacement");
    assert!(analysis.hover("Main.cinch", line, col).expect("updated generated API").contains("Int"));
    assert_eq!(analysis.definition("Main.cinch", line, col), Some(("Library.cinch".into(), 5, 3)));
    assert!(analysis.completions_matching("Main.cinch", "answer").is_empty());
}
