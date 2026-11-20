//! Runtime identities in effectful constructor fields must follow their family
//! arguments during construction, pattern matching, and interface imports.
mod common;

use cinch::driver::Mode;

const ACTION: &str = r#"module Action where
  export Action, pack, apply
  data Action r where
    Wrapped :: (Unit ->{IO r} Nat) -> Action r
  pack :: forall r. (Unit ->{IO r} Nat) -> Action r
  pack work = Wrapped work
  apply :: forall r. using (io :: Io r) => Action r -> Unit ->{IO r} Nat
  apply action unit = case action of
    Wrapped work -> work unit
"#;

/// A callback retains its runtime identity through a separately checked module.
#[test]
fn region_callback_constructs_destructures_and_crosses_an_interface() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("Action.cinch"), ACTION).unwrap();
    std::fs::write(
        dir.path().join("Main.cinch"),
        r#"module Main where
  import Action
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let work = pack (\() -> 42) in
    if apply work () == 42 then 0 else 1
"#,
    )
    .unwrap();
    // First produce an interface, then consume the cached contract at runtime.
    assert_eq!(
        common::drive(Mode::Check, dir.path(), "Main.cinch", None).unwrap(),
        0
    );
    assert_eq!(
        common::drive(Mode::Run, dir.path(), "Main.cinch", None).unwrap(),
        0
    );
}

/// Destructuring a callback must never authorize it under a different runtime.
#[test]
fn callback_cannot_be_relabelled_as_another_runtime() {
    let source = format!("{ACTION}\nmodule Main where\n  import Action\n  bad :: forall r s. using (io :: Io r) => Action s -> Unit ->{{IO r}} Nat\n  bad action unit = apply action unit\n  main :: Int\n  main = 0\n");
    let result = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || common::source_frontend::check_sources(&source, "different-runtime.cinch"))
        .unwrap()
        .join()
        .unwrap();
    assert!(result.is_err(), "a distinct runtime identity was erased");
}

/// Runtime identities cannot simultaneously serve as computational Nat indices.
#[test]
fn callback_region_cannot_be_a_numeric_family_index() {
    let source = "module Main where\n  data Indexed (r :: Nat) where\n    Action :: (Unit ->{IO r} Nat) -> Indexed r\n  main :: Int\n  main = 0\n";
    let result = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || common::source_frontend::check_sources(source, "indexed-runtime.cinch"))
        .unwrap()
        .join()
        .unwrap();
    assert!(
        result.is_err(),
        "a numeric index was accepted as a runtime identity"
    );
}
