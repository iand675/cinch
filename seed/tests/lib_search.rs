//! Public binary-search callbacks over virtual indexed sequences.
mod common;

#[test]
fn binary_search_boundaries_duplicates_and_callback_effects() {
    let dir = common::project("index_search", &[
        ("Data.Search.cinch", include_str!("../prelude/Data/Search.cinch")),
        ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
        ("Main.cinch", include_str!("fixtures/search-consumer.cinch")),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}
