//! Collection constant folding must retain counted constructor fields.
mod common;

#[test]
fn folded_object_lists_keep_constructor_instantiations_and_reclaim_their_fields() {
    let dir = common::project("folded_object_ownership", &[("Main.cinch", r#"
module Main where
  data Item where
    Item :: String -> Int -> Item
  items :: List Item
  items = [Item "first" 184467440737095516160, Item "second" 184467440737095516161]
  main :: Int
  main = case items of
    Cons (Item name n) rest -> if name == "first" && n == 184467440737095516160 then 0 else 1
    Nil -> 2
"#)]);
    let exe = common::build(&dir, "Main.cinch", "folded-ownership").expect("build folded list program");
    let output = std::process::Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1")
        .output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    common::cleanup(&dir);
}

#[test]
fn folded_integer_lists_reclaim_small_and_bignum_tagged_heads() {
    let dir = common::project("folded_integer_ownership", &[(
        "Main.cinch", include_str!("fixtures/folded_integer_heads.cinch"),
    )]);
    let exe = common::build(&dir, "Main.cinch", "folded-integers").expect("build folded integer list");
    let output = std::process::Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1").env("CINCH_RC_DEBUG", "1")
        .output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    common::cleanup(&dir);
}
