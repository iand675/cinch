//! Conditional surface tests (modules specification section 11): the two
//! positions a `when` clause can take beyond a `prim` body, and the
//! facade those positions build.
//!
//! What is pinned here:
//!
//!   imports              a false import contributes nothing (its names are
//!                        unknown), and the same source with the
//!                        complementary condition resolves them;
//!   declarations         a false declaration is absent from the checked
//!                        interface, and a true one is present;
//!   disjointness         two definitions of one name are accepted only
//!                        when no profile can satisfy both conditions, and
//!                        an overlap is refused with both conditions and
//!                        the overlapping atom named;
//!   the interface        it records the selected import only, so a
//!                        Darwin-built interface names no Linux module;
//!   the facade           `Prim.Sys`'s surface is exactly the selected
//!                        backend's surface, and `Effect.Fs` reaches the
//!                        platform layer through it with one import line.
//!
//! Every probe compiles through the same driver the CLI uses, at an
//! explicit target profile, so the answer never depends on the machine
//! running the tests.

use cinch::driver::{
    check_sources_profile, check_sources_profile_ifaces, target_profile, DriverOptions,
};
use cinch::interface::Iface;
use std::collections::BTreeSet;

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

const DARWIN: &str = "aarch64-apple-darwin";
const LINUX: &str = "x86_64-unknown-linux-gnu";

fn profile(triple: &str) -> cinch::core::TargetProfile {
    target_profile(&DriverOptions {
        target: Some(triple.to_string()),
        ..Default::default()
    })
    .expect("the test's triple is a supported target")
}

fn check_at(name: &str, src: &str, triple: &str) -> Result<(), String> {
    check_sources_profile(src, "<test>", &profile(triple))
        .map(|_| ())
        .map_err(|e| format!("{name}: {e}"))
}

fn expect_err(name: &str, src: &str, triple: &str, needles: &[&str]) {
    match check_sources_profile(src, "<test>", &profile(triple)) {
        Ok(_) => panic!("{name}: expected an error, but the program checked for {triple}"),
        Err(e) => {
            let msg = e.to_string();
            for needle in needles {
                assert!(
                    msg.contains(needle),
                    "{name}: error {msg:?} does not name {needle:?}"
                );
            }
        }
    }
}

fn iface_of<'a>(ifaces: &'a [Iface], module: &str) -> &'a Iface {
    ifaces
        .iter()
        .find(|i| i.module == module)
        .unwrap_or_else(|| panic!("{module} publishes an interface"))
}

fn iface_imports(iface: &Iface) -> BTreeSet<String> {
    iface.imports.iter().map(|r| r.module.clone()).collect()
}

fn iface_values(iface: &Iface) -> BTreeSet<String> {
    iface.exports.values.iter().map(|v| v.name.clone()).collect()
}

/// The `Main` a name-resolution probe wraps around one import line and
/// one use of a platform entry point, at the type the entry point has when
/// the import holds.
fn probe(import: &str, use_body: &str) -> String {
    format!(
        "module Main where\n\n  {import}\n\n  probe :: forall r. using (io :: Io r) => String ->{{IO r}} Result SysError I64\n  probe path = {use_body}\n\n  main :: Unit ->{{IO r}} Int\n  main u = 0\n"
    )
}

/// The same shape without the platform vocabulary in the signature, so
/// the probe's first failure is the use of the entry point itself.
fn bare_probe(import: &str, use_body: &str) -> String {
    format!(
        "module Main where\n\n  {import}\n\n  probe :: forall r. using (io :: Io r) => String ->{{IO r}} I64\n  probe path = {use_body}\n\n  main :: Unit ->{{IO r}} Int\n  main u = 0\n"
    )
}

// ----------------------------------------------------------------
// Imports
// ----------------------------------------------------------------

#[test]
fn a_false_conditional_import_leaves_its_names_unknown() {
    // `Prim.Sys.Linux` is not imported on a Darwin target, so the entry
    // point it would have supplied is not in scope.
    expect_err(
        "false import",
        &bare_probe("import Prim.Sys.Linux when not (os darwin)", "sys-open-read path"),
        DARWIN,
        &["sys-open-read"],
    );
}

#[test]
fn the_complementary_condition_resolves_the_name() {
    // The same source, one clause different: on a Darwin target the
    // import holds and the name resolves.
    check_at(
        "true import",
        &probe("import Prim.Sys.Linux when os darwin", "sys-open-read path"),
        DARWIN,
    )
    .expect("the selected import supplies the name");
    // And on the target the first spelling names, it is the other way
    // round: `not (os darwin)` holds on Linux.
    check_at(
        "true negated import",
        &probe("import Prim.Sys.Linux when not (os darwin)", "sys-open-read path"),
        LINUX,
    )
    .expect("the negated condition holds on Linux");
}

#[test]
fn two_clauses_on_one_import_are_a_conjunction() {
    // `os darwin` and `arch aarch64` are both true of this target.
    check_at(
        "conjunction holds",
        &probe(
            "import Prim.Sys.Darwin when os darwin when arch aarch64",
            "sys-open-read path",
        ),
        DARWIN,
    )
    .expect("both clauses hold");
    // The same import on Linux fails the first clause.
    expect_err(
        "conjunction fails",
        &bare_probe(
            "import Prim.Sys.Darwin when os darwin when arch aarch64",
            "sys-open-read path",
        ),
        LINUX,
        &["sys-open-read"],
    );
}

#[test]
fn a_false_conditional_import_is_not_a_module_dependency() {
    // The module is not loaded for this target at all, so even a program
    // that only names it conditionally checks while the file it would
    // have needed is absent: the driver never asks for it.
    let src = "module Main where\n\n  import Data.List when os linux\n\n  main :: Unit ->{IO r} Int\n  main u = 0\n";
    check_at("unselected import", src, DARWIN).expect("a false import is not a dependency");
}

// ----------------------------------------------------------------
// Declarations
// ----------------------------------------------------------------

#[test]
fn a_false_conditional_declaration_is_absent_from_the_interface() {
    let src = r#"
module Main where

  when os linux platform-word :: I64
  when os linux platform-word = 1

  when os darwin darwin-word :: I64
  when os darwin darwin-word = 2

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(DARWIN))
        .expect("the darwin view checks");
    let main = iface_of(&ifaces, "Main");
    let names = iface_values(main);
    assert!(
        names.contains("darwin-word"),
        "the Darwin declaration is present: {names:?}"
    );
    assert!(
        !names.contains("platform-word"),
        "the Linux declaration is absent on Darwin: {names:?}"
    );

    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(LINUX))
        .expect("the linux view checks");
    let names = iface_values(iface_of(&ifaces, "Main"));
    assert!(
        names.contains("platform-word"),
        "the Linux declaration is present: {names:?}"
    );
    assert!(
        !names.contains("darwin-word"),
        "the Darwin declaration is absent on Linux: {names:?}"
    );
}

#[test]
fn a_false_conditional_declaration_is_absent_from_code_generation() {
    // A false declaration is not a name at all, so a use of it does not
    // resolve: the body never reaches code generation.
    let src = r#"
module Main where

  when os linux platform-word :: I64
  when os linux platform-word = 1

  probe :: Unit -> I64
  probe u = platform-word

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    expect_err("absent declaration", src, DARWIN, &["platform-word"]);
    check_at("present declaration", src, LINUX).expect("the Linux view defines it");
}

#[test]
fn two_definitions_with_disjoint_conditions_select_one() {
    let src = r#"
module Main where

  when os darwin word :: I64
  when os darwin word = 1

  when os linux word :: I64
  when os linux word = 2

  export word

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(DARWIN))
        .expect("the darwin view checks");
    let names = iface_values(iface_of(&ifaces, "Main"));
    assert!(names.contains("word"), "exactly one definition applies: {names:?}");

    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(LINUX))
        .expect("the linux view checks");
    let names = iface_values(iface_of(&ifaces, "Main"));
    assert!(names.contains("word"), "exactly one definition applies: {names:?}");
}

#[test]
fn two_definitions_whose_conditions_overlap_are_refused() {
    // `os darwin` and `arch aarch64` can hold together, so this target
    // would have two definitions of `word`. The error names both
    // conditions and the atom they share.
    let src = r#"
module Main where

  when os darwin word :: I64
  when os darwin word = 1

  when os darwin when arch aarch64 word :: I64
  when os darwin when arch aarch64 word = 2

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    expect_err(
        "overlapping conditions",
        src,
        DARWIN,
        &[
            "two definitions of `word`",
            "`os darwin`",
            "`os darwin and arch aarch64`",
            "the atom `os darwin` holds under both",
        ],
    );
}

#[test]
fn a_multi_clause_definition_is_one_definition() {
    // The clauses of one name share a condition, so they are one
    // definition: comparing clause against clause refused every ordinary
    // multi-clause function, which is the regression this pins.
    let src = r#"
module Main where

  word :: Bool -> I64
  word True = 1
  word False = 2
  word flag = 3

  main :: Unit ->{IO r} Int
  main u = if eq-i64 (word False) 2 then 0 else 1
"#;
    check_at("multi-clause definition", src, DARWIN).expect("the clauses are one definition");
}

#[test]
fn a_conditional_definition_may_not_overlap_a_plain_one() {
    // The plain definition holds on every target, so no profile leaves
    // room for the conditional one.
    let src = r#"
module Main where

  word :: I64
  word = 1

  when os darwin word :: I64
  when os darwin word = 2

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    expect_err(
        "plain against conditional",
        src,
        DARWIN,
        &["two definitions of `word`", "unconditional"],
    );
}

// ----------------------------------------------------------------
// The interface
// ----------------------------------------------------------------

#[test]
fn the_interface_records_only_the_selected_import() {
    let src = r#"
module Main where

  import Prim.Sys.Darwin when os darwin
  import Prim.Sys.Linux when os linux

  main :: Unit ->{IO r} Int
  main u = 0
"#;
    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(DARWIN))
        .expect("the darwin view checks");
    let imports = iface_imports(iface_of(&ifaces, "Main"));
    assert!(
        imports.contains("Prim.Sys.Darwin"),
        "the selected import is recorded: {imports:?}"
    );
    assert!(
        !imports.contains("Prim.Sys.Linux"),
        "the unselected import is absent from a Darwin interface: {imports:?}"
    );

    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(LINUX))
        .expect("the linux view checks");
    let imports = iface_imports(iface_of(&ifaces, "Main"));
    assert!(
        imports.contains("Prim.Sys.Linux"),
        "the selected import is recorded: {imports:?}"
    );
    assert!(
        !imports.contains("Prim.Sys.Darwin"),
        "the unselected import is absent from a Linux interface: {imports:?}"
    );
}

// ----------------------------------------------------------------
// The facade
// ----------------------------------------------------------------

#[test]
fn the_facade_surface_is_the_selected_backends_surface() {
    let src = "module Main where\n\n  main :: Unit ->{IO r} Int\n  main u = 0\n";
    let (_program, darwin) = check_sources_profile_ifaces(src, "<test>", &profile(DARWIN))
        .expect("the darwin view checks");
    let (_program, linux) = check_sources_profile_ifaces(src, "<test>", &profile(LINUX))
        .expect("the linux view checks");

    let facade_darwin = iface_values(iface_of(&darwin, "Prim.Sys"));
    let backend_darwin = iface_values(iface_of(&darwin, "Prim.Sys.Darwin"));
    assert_eq!(
        facade_darwin, backend_darwin,
        "on Darwin the facade is the Darwin backend, name for name"
    );

    let facade_linux = iface_values(iface_of(&linux, "Prim.Sys"));
    let backend_linux = iface_values(iface_of(&linux, "Prim.Sys.Linux"));
    assert_eq!(
        facade_linux, backend_linux,
        "on Linux the facade is the Linux backend, name for name"
    );

    // The selection is real: the two backends' surfaces differ (Darwin
    // answers `sys-errno`, which reads the calling thread's error slot,
    // and Linux has no slot to read), so a facade that ignored the target
    // could not be equal to both.
    assert_ne!(
        facade_darwin, facade_linux,
        "the facade's surface follows the target"
    );
    assert!(facade_darwin.contains("sys-errno"), "{facade_darwin:?}");
    assert!(!facade_linux.contains("sys-errno"), "{facade_linux:?}");
}

#[test]
fn a_target_with_neither_backend_reaches_the_portable_floor() {
    // A Windows profile has no syscall backend, so the facade selects the
    // portable module: the same names, every entry point answering the
    // named error, and no dependency on Darwin or Linux.
    let src = "module Main where\n\n  main :: Unit ->{IO r} Int\n  main u = 0\n";
    // A profile, not a `--target` triple: Windows is not one of the four
    // cross targets, and the facade's floor is a profile question.
    let mut windows = cinch::core::TargetProfile::native();
    windows.name = "x86_64-pc-windows-msvc".to_string();
    windows.triple = "x86_64-pc-windows-msvc".to_string();
    windows.os = "windows".to_string();
    windows.arch = "x86_64".to_string();
    windows.libc = "msvcrt".to_string();
    let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &windows)
        .expect("the prelude family checks for a backend-less target");
    let imports = iface_imports(iface_of(&ifaces, "Prim.Sys"));
    assert!(
        imports.contains("Prim.Sys.Portable"),
        "the facade selects the portable floor: {imports:?}"
    );
    for backend in ["Prim.Sys.Darwin", "Prim.Sys.Linux"] {
        assert!(
            !imports.contains(backend),
            "no real backend is selected: {imports:?}"
        );
    }
    let facade = iface_values(iface_of(&ifaces, "Prim.Sys"));
    let portable = iface_values(iface_of(&ifaces, "Prim.Sys.Portable"));
    assert_eq!(facade, portable, "the facade is the portable floor, name for name");
    assert!(facade.contains("sys-open-read"), "{facade:?}");
    // `Effect.Fs` resolves its platform names through the facade on every
    // target, which is why the floor exists.
    let fs_imports = iface_imports(iface_of(&ifaces, "Effect.Fs"));
    assert!(fs_imports.contains("Prim.Sys"), "{fs_imports:?}");
}

#[test]
fn the_facade_interface_names_one_backend() {
    let src = "module Main where\n\n  main :: Unit ->{IO r} Int\n  main u = 0\n";
    let (_program, darwin) = check_sources_profile_ifaces(src, "<test>", &profile(DARWIN))
        .expect("the darwin view checks");
    let imports = iface_imports(iface_of(&darwin, "Prim.Sys"));
    assert!(
        imports.contains("Prim.Sys.Darwin"),
        "the facade imports the selected backend: {imports:?}"
    );
    assert!(
        !imports.contains("Prim.Sys.Linux"),
        "the facade imports no other backend: {imports:?}"
    );

    let (_program, linux) = check_sources_profile_ifaces(src, "<test>", &profile(LINUX))
        .expect("the linux view checks");
    let imports = iface_imports(iface_of(&linux, "Prim.Sys"));
    assert!(
        imports.contains("Prim.Sys.Linux"),
        "the facade imports the selected backend: {imports:?}"
    );
    assert!(
        !imports.contains("Prim.Sys.Darwin"),
        "the facade imports no other backend: {imports:?}"
    );
}

#[test]
fn the_filesystem_home_reaches_the_platform_layer_through_the_facade() {
    // `Effect.Fs` imports `Prim.Sys` and nothing platform specific: its
    // interface names the facade, and the facade names one backend.
    let src = "module Main where\n\n  main :: Unit ->{IO r} Int\n  main u = 0\n";
    for triple in [DARWIN, LINUX] {
        let (_program, ifaces) = check_sources_profile_ifaces(src, "<test>", &profile(triple))
            .unwrap_or_else(|e| panic!("the {triple} view checks: {e}"));
        let imports = iface_imports(iface_of(&ifaces, "Effect.Fs"));
        assert!(
            imports.contains("Prim.Sys"),
            "{triple}: the filesystem home imports the facade: {imports:?}"
        );
        for platform in ["Prim.Sys.Darwin", "Prim.Sys.Linux"] {
            assert!(
                !imports.contains(platform),
                "{triple}: the filesystem home names no backend directly: {imports:?}"
            );
        }
    }
}
