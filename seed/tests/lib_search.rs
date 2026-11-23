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

#[test]
fn array_search_uses_element_order_and_short_circuits_callbacks() {
    let dir = common::project("array_search", &[
        ("Data.Search.cinch", include_str!("../prelude/Data/Search.cinch")),
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Array.Search.cinch", include_str!("../prelude/Data/Array/Search.cinch")),
        ("Data.Compare.cinch", include_str!("../prelude/Data/Compare.cinch")),
        ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
        ("Main.cinch", include_str!("fixtures/array-search-consumer.cinch")),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}
