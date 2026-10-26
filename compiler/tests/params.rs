//! Module parameters and signatures as module arguments (modules
//! specification section 1; the A5 row of the P16 table): parameter
//! blocks with signature-typed and value-typed parameters, instantiation
//! with dictionary-passing semantics, parameter identity in `.sliface`,
//! and intact instance coherence. These tests drive the real CLI pipeline.

use safelang::driver::{driver, Mode};
use std::path::{Path, PathBuf};

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "safelang_paramtest_{}_{}",
        std::process::id(),
        files.as_ptr() as usize
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

fn run_entry(dir: &Path, entry: &str) -> Result<i32, String> {
    driver(Mode::Run, &dir.join(entry), None).map_err(|e| e.to_string())
}

fn check_entry(dir: &Path, entry: &str) -> Result<(), String> {
    driver(Mode::Check, &dir.join(entry), None)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// The shared vocabulary: a user-declared `signature` and two explicit
/// dictionaries of it (non-implicit: selected by name at instantiation,
/// never by default resolution).
const API: &str = "module Api where\n\
    \x20 signature Stepper where\n\
    \x20   step :: Int -> Int\n\
    \x20 instance StepUp :: Stepper where\n\
    \x20   step x = x + 1\n\
    \x20 instance StepDown :: Stepper where\n\
    \x20   step x = x - 1\n";

const TALLY: &str = "module Tally (S :: Stepper) where\n\
    \x20 import Api\n\
    \x20 bump :: Int -> Int\n\
    \x20 bump x = step x\n";

#[test]
fn two_instantiations_observe_different_dictionaries() {
    // `Tally` takes its stepping dictionary as a module argument; the two
    // units observe different dictionaries and compute differently.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Tally (StepUp) as Up\n\
             \x20 import Tally (StepDown) as Down\n\
             \x20 main :: Int\n\
             \x20 main = if Up.bump 1 == 2 && Down.bump 1 == 0 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.lang").expect("two instantiations run");
    assert_eq!(code, 0);
}

#[test]
fn signature_passed_as_a_parameter_sequences_through_it() {
    // The example-12 shape: a `signature` (the sequencing provider class)
    // is the parameter's type, its dictionary is the module argument, and
    // the module's `do` notation composes through it. The two runs show
    // both the Ok flow and the Err short-circuit of `ResultSequencing`.
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 data ConfigError where\n\
             \x20   Bad :: ConfigError\n\
             \x20 fetch :: Int -> Result ConfigError Int\n\
             \x20 fetch n = if n > 0 then Ok n else Err Bad\n\
             \x20 parse :: Int -> Result ConfigError Int\n\
             \x20 parse n = Ok (n + 1)\n\
             \x20 fromOk :: Result ConfigError Int -> Int\n\
             \x20 fromOk r = case r of\n\
             \x20   Ok v -> v + 100\n\
             \x20   Err _ -> 0\n",
        ),
        (
            "ImportConfig.lang",
            "module ImportConfig (Seq :: ResultSequencing ConfigError) where\n\
             \x20 import Lib\n\
             \x20 load :: Int -> Result ConfigError Int\n\
             \x20 load n = do\n\
             \x20   v <- fetch n\n\
             \x20   w <- parse v\n\
             \x20   pure w\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 import ImportConfig (SeqResult) as IC\n\
             \x20 main :: Int\n\
             \x20 main = if fromOk (IC.load 2) == 103 && fromOk (IC.load 0) == 0 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.lang").expect("sequencing parameter run");
    assert_eq!(code, 0);
}

#[test]
fn value_parameter_is_an_ordinary_binding() {
    // A non-signature parameter is an ordinary binding of the module
    // scope, supplied as a static value or literal at instantiation.
    let dir = write_project(&[
        (
            "Lib.lang",
            "module Lib where\n\
             \x20 base :: Int\n\
             \x20 base = 41\n",
        ),
        (
            "AddK.lang",
            "module AddK (k :: Int) where\n\
             \x20 bump :: Int -> Int\n\
             \x20 bump x = x + k\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 import AddK (base) as ByName\n\
             \x20 import AddK (1) as ByLit\n\
             \x20 main :: Int\n\
             \x20 main = if ByName.bump 1 == 42 && ByLit.bump 1 == 2 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.lang").expect("value parameter run");
    assert_eq!(code, 0);
}

#[test]
fn wrong_typed_instantiation_is_rejected() {
    // A dictionary of the wrong signature.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Tally (SeqDirect) as X\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("wrong signature");
    assert!(err.contains("does not match module parameter type"), "{err}");

    // A literal where the parameter is a dictionary.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Tally (42) as X\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("literal dictionary");
    assert!(err.contains("is a dictionary"), "{err}");

    // An instance named where a value is wanted.
    let dir = write_project(&[
        ("Api.lang", API),
        (
            "AddK.lang",
            "module AddK (k :: Int) where\n\
             \x20 bump :: Int -> Int\n\
             \x20 bump x = x + k\n",
        ),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import AddK (StepUp) as X\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("dictionary value");
    assert!(err.contains("is a dictionary, but module parameter"), "{err}");

    // The wrong number of arguments.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Tally (StepUp, StepDown) as X\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("arity");
    assert!(err.contains("takes 1 module arguments, got 2"), "{err}");

    // An unknown dictionary name.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Tally (NoSuchInstance) as X\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("unknown instance");
    assert!(err.contains("unknown instance NoSuchInstance"), "{err}");
}

#[test]
fn parameter_identity_is_recorded_and_instantiation_is_representation() {
    // With instantiations: `Tally.sliface` records the parameter
    // signatures (contract projection), `Main.sliface` records the units
    // with distinct computed identities (representation).
    let with_units = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Tally (StepUp) as Up\n\
             \x20 import Tally (StepDown) as Down\n\
             \x20 main :: Int\n\
             \x20 main = if Up.bump 1 == 2 && Down.bump 1 == 0 then 0 else 1\n",
        ),
    ]);
    check_entry(&with_units, "Main.lang").expect("check with units");
    let tally_text =
        std::fs::read_to_string(with_units.join("build/Tally.sliface")).expect("Tally iface");
    let tally = safelang::interface::Iface::from_json(&tally_text).expect("parse Tally iface");
    assert_eq!(tally.params, vec!["S :: Stepper".to_string()], "{tally_text}");
    let main_text =
        std::fs::read_to_string(with_units.join("build/Main.sliface")).expect("Main iface");
    let main = safelang::interface::Iface::from_json(&main_text).expect("parse Main iface");
    assert_eq!(main.units.len(), 2, "{main_text}");
    let names: Vec<&str> = main.units.iter().map(|u| u.name.as_str()).collect();
    assert!(names.contains(&"Up") && names.contains(&"Down"), "{names:?}");
    for u in &main.units {
        assert_eq!(u.target, "Tally");
    }
    assert_ne!(
        main.units[0].unit_id, main.units[1].unit_id,
        "two instantiations at different dictionaries are distinct units"
    );

    // Without instantiations: the parameterized module's contract hash is
    // unchanged (instantiation is representation, not contract).
    let without_units = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Tally\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    check_entry(&without_units, "Main.lang").expect("check without units");
    let tally2_text =
        std::fs::read_to_string(without_units.join("build/Tally.sliface")).expect("Tally iface 2");
    let tally2 =
        safelang::interface::Iface::from_json(&tally2_text).expect("parse Tally iface 2");
    assert_eq!(
        tally.iface_hash(),
        tally2.iface_hash(),
        "instantiation must not change the parameterized module's contract hash"
    );

    // The instantiator's own contract is likewise unchanged by which
    // units it declares (the units are representation).
    let one_unit = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Tally (StepUp) as Up\n\
             \x20 main :: Int\n\
             \x20 main = 0\n",
        ),
    ]);
    check_entry(&one_unit, "Main.lang").expect("check one unit");
    let main1_text =
        std::fs::read_to_string(one_unit.join("build/Main.sliface")).expect("Main iface 1");
    let main1 = safelang::interface::Iface::from_json(&main1_text).expect("parse Main iface 1");
    assert_eq!(main.units.len(), 2);
    assert_eq!(main1.units.len(), 1);
    assert_eq!(
        main.iface_hash(),
        main1.iface_hash(),
        "instantiation is representation: the instantiator's contract hash is unchanged"
    );
}

#[test]
fn direct_use_of_a_parameterized_module_is_rejected() {
    // A parameterized module's functions are reached through an
    // instantiation unit, never directly.
    let dir = write_project(&[
        ("Api.lang", API),
        ("Tally.lang", TALLY),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Tally\n\
             \x20 main :: Int\n\
             \x20 main = Tally.bump 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("direct use");
    assert!(err.contains("reach it through an instantiation"), "{err}");
}

#[test]
fn instances_from_two_instantiations_do_not_merge() {
    // A parameterized module declaring an implicit instance publishes one
    // copy per instantiation; two copies of one class are ambiguous at
    // default resolution and the program is rejected (coherence intact,
    // nothing silently merges them).
    let api = "module Api where\n\
        \x20 signature Tag where\n\
        \x20   tag :: Int -> Int\n\
        \x20 instance TagA :: Tag where\n\
        \x20   tag x = 1\n\
        \x20 instance TagB :: Tag where\n\
        \x20   tag x = 2\n";
    let pub_mod = "module Pub (S :: Tag) where\n\
        \x20 import Api\n\
        \x20 implicit instance LocalTag :: Tag where\n\
        \x20   tag x = 3\n";
    let dir = write_project(&[
        ("Api.lang", api),
        ("Pub.lang", pub_mod),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Pub (TagA) as A\n\
             \x20 import Pub (TagB) as B\n\
             \x20 main :: Int\n\
             \x20 main = if tag 0 == 3 then 0 else 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.lang").expect_err("two published instances");
    assert!(err.contains("ambiguous instance"), "{err}");

    // One instantiation publishes one instance: coherent, and the use
    // site resolves through it.
    let dir = write_project(&[
        ("Api.lang", api),
        ("Pub.lang", pub_mod),
        (
            "Main.lang",
            "module Main where\n\
             \x20 import Api\n\
             \x20 import Pub (TagA) as A\n\
             \x20 main :: Int\n\
             \x20 main = if tag 0 == 3 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.lang").expect("one published instance");
    assert_eq!(code, 0);
}
