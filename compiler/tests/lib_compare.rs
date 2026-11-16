//! Documentation and independent fixture coverage for explicit comparison policies.

/// Public and private declarations must have canonical Cinch documentation.
#[test]
fn comparison_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        include_str!("../prelude/Data/Compare.cinch"),
        Some("Data.Compare"),
        "Data.Compare",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert!(parsed.orphans.is_empty(), "{:?}", parsed.orphans);
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing canonical documentation", declaration.id));
        assert!(!doc.legacy, "{}", declaration.id);
        let prose = doc.markdown.to_lowercase();
        assert!(!prose.contains("golang") && !prose.contains("go's") && !prose.contains("go "));
    }
}

/// The frozen corpus covers every operand pair, including NaN payloads and zero signs.
#[test]
fn comparison_fixture_corpus_is_complete() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/compare-vectors.json")).unwrap();
    for field in ["f32", "f64"] {
        let rows = fixtures[field].as_array().unwrap();
        assert_eq!(rows.len(), 19 * 19, "{field}");
        assert!(rows
            .iter()
            .any(|row| row["compare"] == -1 && row["less"] == true));
        assert!(rows
            .iter()
            .any(|row| row["compare"] == 0 && row["less"] == false));
        assert!(rows
            .iter()
            .any(|row| row["compare"] == 1 && row["less"] == false));
    }
    assert_eq!(fixtures["integers"].as_array().unwrap().len(), 12 * 25);
    assert_eq!(fixtures["strings"].as_array().unwrap().len(), 10 * 10);
}
