mod common;

#[test]
fn distinct_declaring_classes_with_the_same_name_select_their_own_instances() {
    let dir = common::project("class_identity", &[("Main.cinch", r#"module A where
  signature Foo a where
    foo :: a -> a
  implicit instance FooInt :: Foo Int where
    foo x = x + 1
module B where
  signature Foo a where
    foo :: a -> a
  implicit instance FooInt :: Foo Int where
    foo x = x + 40
module Main where
  import A as Alpha
  import B as Beta
  main :: Int
  main = Alpha.foo 1 + Beta.foo 1
"#)]);
    assert_eq!(common::run(&dir, "Main.cinch"), 43);
    common::cleanup(&dir);
}

#[test]
fn evidence_for_another_declaring_class_is_rejected() {
    let source = r#"module A where
  signature Foo a where
    foo :: a -> a
  accept :: using (dict :: Foo Int) => Int -> Int
  accept x = foo x
module B where
  signature Foo a where
    foo :: a -> a
  instance Dict :: Foo Int where
    foo x = x + 40
module Main where
  import A as Alpha
  import B as Beta
  main :: Int
  main = Alpha.accept @{dict = Beta.Dict} 1
"#;
    assert!(cinch::driver::check_sources(source, "wrong-class-evidence.cinch").is_err());
}
