//! Fixity and compiled interfaces: an operator's precedence and
//! associativity are part of the module's interface, so an importer
//! parses the operator without redeclaring it.
//!
//! Three claims, each a program that must compile and run:
//!   1. A single-source program (the `check_sources` path) that imports
//!      `Algebra.Semigroup` and writes `<>` infix needs no local fixity
//!      declaration: the fixity travels with the import.
//!   2. A project module that imports a sibling declaring `infixr 6 <+>`
//!      likewise parses `<+>` infix without declaring it.
//!   3. The sibling's fixity is recorded in its `.cinchif` interface, so a
//!      consumer checked against the interface alone still parses the
//!      operator.

use cinch::driver::{check_sources, driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Claim 1: the single-source path carries the imported fixity.
#[test]
fn imported_fixity_parses_in_a_single_source_program() {
    let src = "module Main where\n\
               \x20 import Algebra.Semigroup\n\
               \n\
               \x20 main :: Int\n\
               \x20 main =\n\
               \x20   let xs = (Cons 1 Nil) <> (Cons 2 Nil)\n\
               \x20   in case xs of\n\
               \x20     Nil -> 1\n\
               \x20     Cons _ _ -> 0\n";
    match check_sources(src, "<fixity-single-source>") {
        Ok(_) => {}
        Err(e) => panic!("importing Algebra.Semigroup must supply `<>`'s fixity: {e}"),
    }
}

const PLUS_SIBLING: &str = "module Sibling where\n\
                            \n\
                            \x20 infixr 6 <+>\n\
                            \n\
                            \x20 (<+>) :: Int -> Int -> Int\n\
                            \x20 (<+>) a b = b\n\
                            \n\
                            \x20 add-and-flag :: Int -> Int -> Int\n\
                            \x20 add-and-flag a b = a <+> b\n";

/// Claims 2 and 3: a project consumer parses the imported operator, and
/// the interface records the fixity that let it.
#[test]
fn project_consumer_parses_imported_operator_and_iface_records_it() {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_fixity_iface_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Sibling.cinch"), PLUS_SIBLING).expect("write sibling");
    std::fs::write(
        dir.join("Main.cinch"),
        "module Main where\n\
         \x20 import Sibling\n\
         \n\
         \x20 main :: Int\n\
         \x20 main = 0 <+> 1\n",
    )
    .expect("write main");

    let entry = dir.join("Main.cinch");
    match driver(Mode::Run, &entry, None) {
        Ok(code) => assert_eq!(code, 1, "the imported operator must run and add"),
        Err(e) => panic!("a consumer must parse the imported operator without redeclaring it: {e}"),
    }

    let iface = dir.join("build").join("Sibling.cinchif");
    let text = std::fs::read_to_string(&iface)
        .unwrap_or_else(|e| panic!("Sibling interface {}: {e}", iface.display()));
    assert!(
        text.contains("<+>"),
        "the interface must record the operator's fixity, got:\n{text}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
