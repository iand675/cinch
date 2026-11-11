//! A type's identity is its declaring module and its name, and nothing
//! else.
//!
//! The bug these tests hold shut: a base type was identified by its bare
//! spelling while that spelling was unique across the embedded family, so
//! a second declaration of the same spelling anywhere in the family
//! silently renamed the first. `Data.Producer.Step` lost the identity
//! `Step` to a private `data Step` in an unrelated base module, and the
//! builtin `vecStreamStep`, which is typed at the producer's step form,
//! then failed to typecheck against its own prelude.

use cinch::ast;
use cinch::check::StagingCx;
use cinch::core::TargetProfile;
use cinch::driver::{parse_prelude, parse_source};
use cinch::interface::Iface;
use std::collections::HashMap;

mod common;

/// The embedded family with one module appended after its last one: a
/// source provider that declares the producer library's own type
/// spelling (`Step`) privately.
fn family_with_second_step() -> Vec<ast::Module> {
    let prelude = parse_prelude(&[]).expect("the embedded family parses");
    let extra = parse_source(
        "module Extra.Producer where\n\
         \x20 data Step where\n\
         \x20   ExtraDone :: Step\n\
         \x20 data Done where\n\
         \x20   Done :: Done\n",
        "Extra.Producer.cinch",
        &prelude.base,
        &prelude.exported_fixities,
    )
    .expect("the second declaration parses");
    let mut modules = prelude.modules;
    modules.extend(extra.modules);
    modules
}

fn parse_user(source: &str) -> Vec<ast::Module> {
    let prelude = parse_prelude(&[]).expect("the embedded family parses");
    parse_source(source, "Main.cinch", &prelude.base, &prelude.exported_fixities)
        .expect("the program parses")
        .modules
}

fn check(family: &[ast::Module], user: &[ast::Module]) -> Result<Vec<Iface>, String> {
    let hashes = user
        .iter()
        .map(|module| (module.name.text.clone(), String::new()))
        .collect::<HashMap<_, _>>();
    cinch::check::check_program_multi_full(
        family,
        user,
        &hashes,
        &StagingCx::default(),
        &[],
        &TargetProfile::native(),
    )
    .map(|(_, ifaces, _, _)| ifaces)
    .map_err(|error| error.to_string())
}

/// The producer pipeline (`prod-fold`, `prod-map`) beside the vector
/// producer, which is the builtin `vecStreamStep` typed at the step form.
const PRODUCER_PROGRAM: &str = "module Main where\n\
     \x20 step3 :: Nat -> Step Nat Nat\n\
     \x20 step3 k = case k of\n\
     \x20   0 -> Yield 3 1\n\
     \x20   1 -> Yield 5 2\n\
     \x20   _ -> Done\n\
     \x20 srcP :: Producer Nat Nat\n\
     \x20 srcP = Producer step3 0\n\
     \x20 doubleN :: Nat -> Nat\n\
     \x20 doubleN x = nat-add x x\n\
     \x20 summed :: Nat\n\
     \x20 summed = prod-fold 0 nat-add (prod-map doubleN srcP)\n\
     \x20 streamed :: forall n region. Ref region (Vec n U32) -> U32\n\
     \x20 streamed xs = scalar-sum-wrap32 xs\n\
     \x20 main :: Int\n\
     \x20 main = if summed == 16 then 0 else 1\n";

#[test]
fn a_base_type_keeps_its_identity_beside_a_second_declaration() {
    let family = family_with_second_step();
    let ifaces = check(&family, &parse_user(PRODUCER_PROGRAM)).expect("the program checks");
    let producer = ifaces
        .iter()
        .find(|iface| iface.module == "Data.Producer")
        .expect("Data.Producer");
    let steps: Vec<&str> = producer
        .exports
        .types
        .iter()
        .filter(|export| export.name == "Step")
        .map(|export| export.identity.as_str())
        .collect();
    assert_eq!(steps, ["Data.Producer.Step"]);
    let extra = ifaces
        .iter()
        .find(|iface| iface.module == "Extra.Producer")
        .expect("Extra.Producer");
    let extra_steps: Vec<&str> = extra
        .exports
        .types
        .iter()
        .filter(|export| export.name == "Step")
        .map(|export| export.identity.as_str())
        .collect();
    assert_eq!(extra_steps, ["Extra.Producer.Step"]);
}

#[test]
fn a_second_step_changes_no_interface_of_any_other_module() {
    let family = family_with_second_step();
    let user = parse_user(PRODUCER_PROGRAM);
    let with_second = check(&family, &user).expect("the program checks beside a second step");
    let family = parse_prelude(&[]).expect("the embedded family parses").modules;
    let without = check(&family, &user).expect("the program checks alone");
    let rendered = |ifaces: &[Iface]| -> Vec<(String, String)> {
        ifaces
            .iter()
            .filter(|iface| iface.module != "Extra.Producer")
            .map(|iface| (iface.module.clone(), iface.to_json()))
            .collect()
    };
    assert_eq!(rendered(&with_second), rendered(&without));
}

#[test]
fn two_declarations_with_one_bare_name_both_print_qualified() {
    let dir = common::project(
        "qualified_diagnostic",
        &[(
            "Main.cinch",
            r#"module Steps where
  data Step where
    Done :: Step
    More :: Int -> Step
module Main where
  import Steps as S
  wrong :: S.Step -> Step Nat Nat
  wrong s = s
  main :: Int
  main = 0
"#,
        )],
    );
    let message = common::check(&dir, "Main.cinch").expect_err("the mismatch is refused");
    common::cleanup(&dir);
    assert!(
        message.contains("expected Steps.Step, got Data.Producer.Step Nat Nat"),
        "two declarations share the bare name, so both print qualified: {message}"
    );
}

/// The reported bug, on disk and end to end: a project loads a module
/// that declares `Step` and `Done` privately, and its own producer
/// program must still check and run. Before the change the loaded
/// declaration renamed `Data.Producer.Step` to `Step`, and the builtin
/// `vecStreamStep` no longer matched the prelude it was typed against.
#[test]
fn a_loaded_module_declaring_step_and_done_changes_nothing() {
    let dir = common::project(
        "qualified_loaded_step",
        &[
            (
                "Extra.Producer.cinch",
                "module Extra.Producer where\n\
                 \x20 data Step where\n\
                 \x20   ExtraDone :: Step\n\
                 \x20 data Done where\n\
                 \x20   Done :: Done\n",
            ),
            (
                "Main.cinch",
                "module Main where\n\
                 \x20 import Extra.Producer as X\n\
                 \x20 step3 :: Nat -> Step Nat Nat\n\
                 \x20 step3 k = case k of\n\
                 \x20   0 -> Yield 3 1\n\
                 \x20   1 -> Yield 5 2\n\
                 \x20   _ -> Done\n\
                 \x20 srcP :: Producer Nat Nat\n\
                 \x20 srcP = Producer step3 0\n\
                 \x20 doubleN :: Nat -> Nat\n\
                 \x20 doubleN x = nat-add x x\n\
                 \x20 summed :: Nat\n\
                 \x20 summed = prod-fold 0 nat-add (prod-map doubleN srcP)\n\
                 \x20 main :: Int\n\
                 \x20 main = if summed == 16 then 0 else 1\n",
            ),
        ],
    );
    assert_eq!(common::run(&dir, "Main.cinch"), 0, "the program runs beside a loaded `Step`");
    let identity = |module: &str| -> String {
        let text = std::fs::read_to_string(dir.join(format!("build/{module}.cinchif")))
            .unwrap_or_else(|error| panic!("{module}.cinchif: {error}"));
        let iface = Iface::from_json(&text).unwrap_or_else(|error| panic!("{module}.cinchif: {error}"));
        iface
            .exports
            .types
            .iter()
            .find(|export| export.name == "Step")
            .map(|export| export.identity.clone())
            .unwrap_or_else(|| panic!("{module} exports no `Step`"))
    };
    assert_eq!(identity("Data.Producer"), "Data.Producer.Step");
    assert_eq!(identity("Extra.Producer"), "Extra.Producer.Step");
    common::cleanup(&dir);
}

#[test]
fn an_unambiguous_name_prints_bare() {
    let dir = common::project(
        "qualified_bare_diagnostic",
        &[(
            "Main.cinch",
            r#"module Main where
  data Tree where
    Leaf :: Tree
    Node :: Tree -> Int -> Tree -> Tree
  find :: Int -> Tree -> Option Tree
  find x t = t
  main :: Int
  main = 0
"#,
        )],
    );
    let message = common::check(&dir, "Main.cinch").expect_err("the mismatch is refused");
    common::cleanup(&dir);
    assert!(
        message.contains("expected Tree, got Option Tree"),
        "the message names one `Tree` and one `Option`, so both print bare: {message}"
    );
    assert!(
        !message.contains("Main.Tree") && !message.contains("Data.Core.Option"),
        "no name in this message is ambiguous: {message}"
    );
}
