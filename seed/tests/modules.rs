//! Multi-file modules, compiled interface metadata, and artifact manifests.
//! These tests drive the real CLI pipeline (project loading, per-module
//! interfaces, chain and coherence checks, linking with the Rust runtime).

mod common;

use cinch::driver::Mode;
use std::path::{Path, PathBuf};

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch_modtest_{}_{}",
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
    common::drive(Mode::Run, dir, entry, None)
}

fn check_entry(dir: &Path, entry: &str) -> Result<(), String> {
    common::check(dir, entry)
}

fn assert_manifest(dir: &Path, entry: &str, modules: &[&str]) {
    let build = dir.join("build");
    let manifest = std::fs::read_to_string(build.join(format!("{entry}.manifest.json")))
        .expect("checked build manifest");
    let manifest: serde_json::Value = serde_json::from_str(&manifest).expect("parse build manifest");
    assert_eq!(manifest["entry"], entry);
    assert_eq!(
        manifest["runtime_abi"].as_u64(),
        Some(u64::from(cinch::interface::RUNTIME_ABI_VERSION))
    );
    let dependencies = manifest["modules"].as_array().expect("manifest module identities");
    for module in modules {
        let interface = std::fs::read_to_string(build.join(format!("{module}.cinchif")))
            .expect("checked module interface");
        let iface = cinch::interface::Iface::from_json(&interface).expect("parse module interface");
        let hashes: Vec<_> = dependencies.iter().filter(|row| row["module"] == *module)
            .map(|row| row["iface_hash"].as_str().expect("manifest interface hash"))
            .collect();
        assert_eq!(hashes, [iface.iface_hash()], "{module}");
    }
}

#[test]
fn multi_file_module_links_and_runs() {
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Lib where\n\
             \x20 double :: Int -> Int\n\
             \x20 double x = x + x\n\
             \x20 data Wrapper where\n\
             \x20   W :: Int -> Wrapper\n\
             \x20 unwrap :: Wrapper -> Int\n\
             \x20 unwrap w = case w of\n\
             \x20   W n -> double n\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 main :: Int\n\
             \x20 main = if Lib.unwrap (Lib.W 21) == 42 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.cinch").expect("multi-file run");
    assert_eq!(code, 0);
    // Per-module interfaces and the artifact manifest are written.
    for f in ["build/Lib.cinchif", "build/Main.cinchif", "build/Main.manifest.json"] {
        assert!(dir.join(f).exists(), "missing artifact {f}");
    }
    let lib_iface = std::fs::read_to_string(dir.join("build/Lib.cinchif")).expect("iface");
    assert!(lib_iface.contains("\"module\":\"Lib\""), "{lib_iface}");
    assert!(lib_iface.contains("Lib.double"), "{lib_iface}");
    assert_manifest(&dir, "Main", &["Lib", "Main"]);
}

#[test]
fn imported_fixity_is_visible() {
    let dir = write_project(&[
        (
            "Ops.cinch",
            "module Ops where\n\
             \x20 infixl 5 <+>\n\
             \x20 (<+>) :: Int -> Int -> Int\n\
             \x20 (<+>) a b = a * 10 + b\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Ops\n\
             \x20 main :: Int\n\
             \x20 main = if 1 <+> 2 == 12 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.cinch").expect("fixity run");
    assert_eq!(code, 0);
}

#[test]
fn instance_coherence_across_modules() {
    let dir = write_project(&[
        (
            "A.cinch",
            "module A where\n\
             \x20 data T where\n\
             \x20   T :: Int -> T\n\
             \x20 implicit instance Eq_T_A :: Eq T where\n\
             \x20   (==) x y = True\n\
             \x20   (/=) x y = False\n",
        ),
        (
            "B.cinch",
            "module B where\n\
             \x20 import A\n\
             \x20 implicit instance Eq_T_B :: Eq T where\n\
             \x20   (==) x y = False\n\
             \x20   (/=) x y = True\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import A\n\
             \x20 import B\n\
             \x20 main :: Int\n\
             \x20 main = if A.T 1 == A.T 2 then 0 else 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.cinch").expect_err("coherence");
    assert!(err.contains("ambiguous instance"), "{err}");
}

#[test]
fn import_cycle_is_rejected() {
    let dir = write_project(&[
        (
            "A.cinch",
            "module A where\n\
             \x20 import B\n\
             \x20 f :: Int -> Int\n\
             \x20 f x = x\n",
        ),
        (
            "B.cinch",
            "module B where\n\
             \x20 import A\n\
             \x20 g :: Int -> Int\n\
             \x20 g x = x\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import A\n\
             \x20 main :: Int\n\
             \x20 main = A.f 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.cinch").expect_err("cycle");
    assert!(err.contains("cycle"), "{err}");
}

#[test]
fn missing_import_file_is_named() {
    let dir = write_project(&[(
        "Main.cinch",
        "module Main where\n\
         \x20 import Ghost\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    )]);
    let err = check_entry(&dir, "Main.cinch").expect_err("missing");
    assert!(err.contains("Ghost"), "{err}");
}

#[test]
fn imported_module_is_resolved_by_declaration() {
    // The staging slice supersedes the file-stem rule (decision D42):
    // `import X` resolves to a module of an already loaded file, then to
    // `X.cinch`, then to a unique `module X` beside the entry. `Lib.cinch`
    // declaring only `Wrong` provides no `Lib`, and `import Wrong` finds
    // it through the directory scan.
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Wrong where\n\
             \x20 f :: Int -> Int\n\
             \x20 f x = x\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 main :: Int\n\
             \x20 main = Lib.f 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.cinch").expect_err("no module Lib");
    assert!(err.contains("Lib"), "{err}");
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Wrong where\n\
             \x20 f :: Int -> Int\n\
             \x20 f x = x\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Wrong\n\
             \x20 main :: Int\n\
             \x20 main = Wrong.f 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.cinch").expect("directory-scan resolution");
    assert_eq!(code, 1);
}

#[test]
fn entry_module_name_is_free() {
    // The examples are `fib.cinch` declaring `module Main`; the entry is not
    // resolved by name, so its declared module may differ from its stem.
    let dir = write_project(&[(
        "smoke.cinch",
        "module Main where\n\
         \x20 main :: Int\n\
         \x20 main = 7\n",
    )]);
    let code = run_entry(&dir, "smoke.cinch").expect("free entry name");
    assert_eq!(code, 7);
}

#[test]
fn multi_module_file_links_and_runs() {
    // The staging slice supersedes the one-module-per-file rule: one
    // source file may declare several modules, and they reference each
    // other across the blocks.
    let dir = write_project(&[(
        "Main.cinch",
        "module Lib where\n\
         \x20 double :: Int -> Int\n\
         \x20 double x = x + x\n\
         \n\
         module Main where\n\
         \x20 import Lib\n\
         \x20 main :: Int\n\
         \x20 main = if Lib.double 21 == 42 then 0 else 1\n",
    )]);
    let code = run_entry(&dir, "Main.cinch").expect("multi-module file run");
    assert_eq!(code, 0);
    for f in ["build/Lib.cinchif", "build/Main.cinchif"] {
        assert!(dir.join(f).exists(), "missing artifact {f}");
    }
    assert_manifest(&dir, "Main", &["Lib", "Main"]);
}

#[test]
fn a_module_can_import_a_signature_declared_later_in_the_same_file() {
    let dir = write_project(&[("Main.cinch", "module Main where\n  import LocalSignature (Echo, echo)\n  implicit instance IntEcho :: Echo Int where\n    echo x = x + 1\n  main :: Int\n  main = if echo 18446744073709551616 == 18446744073709551617 then 213 else 1\nmodule LocalSignature where\n  export Echo, echo\n  signature Echo a where\n    echo :: a -> a\n")]);
    assert_eq!(run_entry(&dir, "Main.cinch").expect("dependency before its importer"), 213);
    let main = cinch::interface::Iface::from_json(&std::fs::read_to_string(dir.join("build/Main.cinchif")).unwrap()).unwrap();
    assert!(main.imports.iter().any(|dependency| dependency.module == "LocalSignature"));
}

#[test]
fn imported_instance_is_used_by_the_importer() {
    // The instance lives in Lib; Main resolves `==` on Lib's type through it.
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Lib where\n\
             \x20 data Token where\n\
             \x20   Tok :: I64 -> Token\n\
             \x20 implicit instance Eq_Token :: Eq Token where\n\
             \x20   (==) a b = case a of\n\
             \x20     Tok x -> case b of\n\
             \x20       Tok y -> primEqI64 x y\n\
             \x20   (/=) a b = case a of\n\
             \x20     Tok x -> case b of\n\
             \x20       Tok y -> primNeI64 x y\n\
             \x20 foreign import ccall \"cinch.prim.eqI64\" primEqI64 :: I64 -> I64 -> Bool\n\
             \x20 foreign import ccall \"cinch.prim.neI64\" primNeI64 :: I64 -> I64 -> Bool\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 main :: Int\n\
             \x20 main = if Lib.Tok 5 == Lib.Tok 5 then 0 else 1\n",
        ),
    ]);
    let code = run_entry(&dir, "Main.cinch").expect("cross-module instance");
    assert_eq!(code, 0);
}

#[test]
fn interface_round_trips_through_disk() {
    // The written .cinchif must parse back and keep its identity.
    let dir = write_project(&[(
        "Main.cinch",
        "module Main where\n\
         \x20 main :: Int\n\
         \x20 main = 0\n",
    )]);
    check_entry(&dir, "Main.cinch").expect("check");
    let text = std::fs::read_to_string(dir.join("build/Main.cinchif")).expect("iface");
    let iface = cinch::interface::Iface::from_json(&text).expect("parse written iface");
    assert_eq!(iface.module, "Main");
    assert!(iface.exports.values.iter().any(|v| v.name == "main"));
}

#[test]
fn a_failure_in_a_sibling_module_names_its_file() {
    // A `Span` carries only a line and a column (`lexer::Span`), so before the
    // checker attached a module's file to its failure, a collect or check
    // error in a module other than the entry reported a bare position: with
    // six ladder modules in one compile, `812:13: constructor POk expects 2
    // field patterns, got 1` gave no way to tell which module raised it.
    let dir = write_project(&[
        (
            "Lib.cinch",
            "module Lib where\n\
             \x20 broken :: Int -> Int\n\
             \x20 broken x = x + missing-name\n",
        ),
        (
            "Main.cinch",
            "module Main where\n\
             \x20 import Lib\n\
             \x20 main :: Int\n\
             \x20 main = Lib.broken 1\n",
        ),
    ]);
    let err = check_entry(&dir, "Main.cinch").expect_err("the sibling module is broken");
    assert!(
        err.contains("(in "),
        "the failure does not name the module it happened in: {err}"
    );
    assert!(
        err.contains("Lib.cinch"),
        "the failure does not name the failing module: {err}"
    );
    assert!(
        !err.contains("Main.cinch"),
        "the failure blames the entry instead of the failing module: {err}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// The two modules a hiding import is tested against: both export
/// `shared`, and `A` exports an operator with a fixity beside it.
const HIDING_SOURCES: [(&str, &str); 2] = [
    (
        "A.cinch",
        "module A where\n\
         \x20 export shared, other, (<+>)\n\
         \x20 infixl 5 <+>\n\
         \x20 shared :: Int\n\
         \x20 shared = 1\n\
         \x20 other :: Int\n\
         \x20 other = 10\n\
         \x20 (<+>) :: Int -> Int -> Int\n\
         \x20 (<+>) a b = a * 10 + b\n",
    ),
    (
        "B.cinch",
        "module B where\n\
         \x20 export shared\n\
         \x20 shared :: Int\n\
         \x20 shared = 2\n",
    ),
];

fn hiding_project(main: &'static str) -> PathBuf {
    write_project(&[HIDING_SOURCES[0], HIDING_SOURCES[1], ("Main.cinch", main)])
}

/// Two imports that export one name collide. `hiding` leaves one of them
/// out, so the bare name is the other module's and the rest of the hiding
/// module's surface, operators and their fixity included, still comes in.
#[test]
fn a_hiding_import_resolves_a_name_two_modules_export() {
    let dir = hiding_project(
        "module Main where\n\
         \x20 import A hiding (shared)\n\
         \x20 import B\n\
         \x20 main :: Int\n\
         \x20 main = if shared + other + (1 <+> 2) == 24 then 0 else 1\n",
    );
    let code = run_entry(&dir, "Main.cinch").expect("hiding run");
    assert_eq!(code, 0);
    let _ = std::fs::remove_dir_all(&dir);
}

/// A hidden name is out of scope under the module's qualifier as well,
/// not only bare.
#[test]
fn a_hidden_name_is_not_reachable_through_the_qualifier() {
    let dir = hiding_project(
        "module Main where\n\
         \x20 import A hiding (shared)\n\
         \x20 main :: Int\n\
         \x20 main = A.shared\n",
    );
    let err = check_entry(&dir, "Main.cinch").expect_err("the hidden name is used");
    assert!(err.contains("shared"), "the refusal does not name the hidden name: {err}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A hidden operator takes its fixity with it. `A` declares `<+>`
/// `infixl 5` and `C` declares its own `infixr 5`; with `A`'s hidden, an
/// unparenthesized chain parses by `C`'s declaration whichever import
/// comes first: `10 <+> 3 <+> 2` is `10 - (3 - 2)`.
#[test]
fn a_hidden_operator_does_not_bring_its_fixity() {
    const C: (&str, &str) = (
        "C.cinch",
        "module C where\n\
         \x20 export (<+>)\n\
         \x20 infixr 5 <+>\n\
         \x20 (<+>) :: Int -> Int -> Int\n\
         \x20 (<+>) a b = a - b\n",
    );
    for main in [
        "module Main where\n\
         \x20 import A hiding ((<+>))\n\
         \x20 import C\n\
         \x20 main :: Int\n\
         \x20 main = if 10 <+> 3 <+> 2 == 9 then 0 else 1\n",
        "module Main where\n\
         \x20 import C\n\
         \x20 import A hiding ((<+>))\n\
         \x20 main :: Int\n\
         \x20 main = if 10 <+> 3 <+> 2 == 9 then 0 else 1\n",
    ] {
        let dir = write_project(&[HIDING_SOURCES[0], C, ("Main.cinch", main)]);
        let code = run_entry(&dir, "Main.cinch").expect("hidden operator run");
        assert_eq!(code, 0);
        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// Hiding a name the module does not export would silently hide nothing,
/// so it is refused, and so is hiding beside a selection.
#[test]
fn a_hiding_import_names_only_members_and_excludes_a_selection() {
    let dir = hiding_project(
        "module Main where\n\
         \x20 import A hiding (missing)\n\
         \x20 main :: Int\n\
         \x20 main = other\n",
    );
    let err = check_entry(&dir, "Main.cinch").expect_err("no such member");
    assert!(err.contains("`A` has no member `missing` to hide"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);

    let dir = hiding_project(
        "module Main where\n\
         \x20 import A hiding (shared) (other)\n\
         \x20 main :: Int\n\
         \x20 main = other\n",
    );
    let err = check_entry(&dir, "Main.cinch").expect_err("hiding and selecting");
    assert!(err.contains("either selects members or hides them"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// The collision refusal names both ways out.
#[test]
fn an_import_collision_suggests_renaming_or_hiding() {
    let dir = hiding_project(
        "module Main where\n\
         \x20 import A\n\
         \x20 import B\n\
         \x20 main :: Int\n\
         \x20 main = other\n",
    );
    let err = check_entry(&dir, "Main.cinch").expect_err("collision");
    assert!(err.contains("the name `shared` is imported from both A and B"), "{err}");
    assert!(err.contains("import B hiding (shared)"), "{err}");
    let _ = std::fs::remove_dir_all(&dir);
}
