//! Persistent array edits preserve input values through overlap and failed bounds.
mod common;

#[test]
fn persistent_range_edits_and_compaction() {
    let dir = common::project("array_edit", &[
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Array.Edit.cinch", include_str!("../prelude/Data/Array/Edit.cinch")),
        ("Main.cinch", include_str!("fixtures/array-edit-consumer.cinch")),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}
