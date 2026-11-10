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
//!                        platform layer through it with one import line;
//!   the host and target  a build carries both profiles: a module gated on
//!                        the host's operating system is refused when
//!                        compile-time code reaches it (naming both
//!                        profiles and both contexts) and when ordinary
//!                        code does (naming the target's condition), and
//!                        one build selects a different module for each
//!                        profile while the interface records the target's
//!                        selection only.
//!
//! Every probe compiles through the same driver the CLI uses, at an
//! explicit target profile, so the answer never depends on the machine
//! running the tests. The host and target probes gate their fixtures on
//! the profile of whatever machine runs them, so they assert the same
//! split on a Darwin or a Linux host.

mod common;

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

// ----------------------------------------------------------------
// The host and the target
// ----------------------------------------------------------------
//
// A build carries two profiles (design/base-as-a-package.md, "Host and
// target are different questions"): the host, where compile-time code
// runs, and the target, which the artifact is emitted for. Module
// availability is therefore a per-profile fact, and the fixtures below
// gate a module on the HOST's operating system, so the same project
// resolves it for the host and not for a cross target on any machine
// running the suite.

/// One scratch project: `files` are `(relative path, text)`, written
/// under a fresh directory named after `tag`.
fn project(tag: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-host-target-{}-{tag}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch project");
    for (name, text) in files {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent directory");
        }
        std::fs::write(&path, text).expect("write fixture");
    }
    dir
}

/// Check `dir/entry` for `triple` (the host's own target when `None`).
/// The driver writes the manifest and interfaces beside the entry, so a
/// caller can read what the build recorded.
fn check_project(
    dir: &std::path::Path,
    entry: &str,
    triple: Option<&str>,
) -> Result<(), String> {
    if triple.is_none() {
        return common::check(dir, entry);
    }
    let opts = DriverOptions {
        target: triple.map(str::to_string),
        ..DriverOptions::default()
    };
    cinch::driver::driver_opts(cinch::driver::Mode::Check, &dir.join(entry), None, &opts)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

/// The profile of the machine running this test: the build host.
fn host() -> cinch::core::TargetProfile {
    cinch::core::TargetProfile::native()
}

/// The supported triple for the operating system the host is NOT, so the
/// two profiles differ and the fixture's condition separates them.
fn other_os_triple() -> String {
    let other = if host().os == "darwin" { "linux" } else { "darwin" };
    cinch::driver::SUPPORTED_TARGETS
        .iter()
        .find(|t| profile(t).os == other)
        .map(|t| (*t).to_string())
        .unwrap_or_else(|| panic!("a supported triple for {other}"))
}

/// The `apple-darwin` triple whose architecture is not the host's, so a
/// module gated on `arch <that>` is available for the target and not for
/// the host. Both Apple triples are supported, so this always exists.
fn other_arch_triple() -> String {
    let other = if host().arch == "aarch64" { "x86_64" } else { "aarch64" };
    cinch::driver::SUPPORTED_TARGETS
        .iter()
        .find(|t| {
            let p = profile(t);
            p.os == "darwin" && p.arch == other
        })
        .map(|t| (*t).to_string())
        .unwrap_or_else(|| panic!("a Darwin triple for {other}"))
}

/// A project whose manifest gates one module on the host's operating
/// system, plus the entry source that imports it.
fn host_gated_project(tag: &str, main: &str) -> std::path::PathBuf {
    let os = host().os;
    let manifest = format!(
        r#"{{
  "schema": 2,
  "name": "acme/app",
  "version": "0.1.0",
  "modules": [ {{ "name": "Main", "path": "Main.cinch" }} ],
  "order": ["Main"],
  "select": [
    {{ "when": {{ "os": ["{os}"] }},
      "modules": [ {{ "name": "Host.Only", "path": "host/Host.cinch" }} ] }}
  ]
}}"#
    );
    project(
        tag,
        &[
            (".cinchpkg", manifest.as_str()),
            (
                "host/Host.cinch",
                "module Host.Only where\n  export host-flavour\n\n  host-flavour :: Int\n  host-flavour = 3\n",
            ),
            ("Main.cinch", main),
        ],
    )
}

#[test]
fn compile_time_code_that_reaches_a_host_only_module_is_refused_naming_both_profiles() {
    // The staged definition runs during the build, on the host, and the
    // module it imports resolves for the host and not for the target: the
    // refusal has to name both profiles and both contexts, because the
    // fix differs (move the code out of the build-time path, or widen the
    // package's conditions).
    let main = "module Main where\n  import Host.Only\n\n  stage flavour :: Int\n  stage flavour = host-flavour\n\n  main :: Int\n  main = 0\n";
    let dir = host_gated_project("compile-time", main);
    let triple = other_os_triple();
    let err = check_project(&dir, "Main.cinch", Some(&triple))
        .expect_err("a compile-time reach of a host-only module is refused");
    let target = profile(&triple);
    let host = host();
    for needle in [
        "Host.Only",
        "compile-time code",
        host.triple.as_str(),
        host.os.as_str(),
        target.triple.as_str(),
        target.os.as_str(),
        "acme/app",
        "move the code",
        "widen",
    ] {
        assert!(err.contains(needle), "the refusal names {needle:?}: {err}");
    }
    // Both profiles, so a reader sees which way round the split is.
    assert!(
        err.contains(&format!("host {}", host.triple))
            && err.contains(&format!("target {}", target.triple)),
        "the refusal names the host and the target: {err}"
    );
}

#[test]
fn emitted_code_that_reaches_a_host_only_module_is_refused_as_unavailable_for_the_target() {
    // The same import from ordinary code is the target's question only:
    // no stage, so the refusal states that the module is unavailable for
    // the target and names the condition that would select it.
    let main = "module Main where\n  import Host.Only\n\n  main :: Int\n  main = host-flavour\n";
    let dir = host_gated_project("emitted", main);
    let triple = other_os_triple();
    let err = check_project(&dir, "Main.cinch", Some(&triple))
        .expect_err("ordinary code cannot import a module the target does not resolve");
    let target = profile(&triple);
    for needle in [
        "Host.Only",
        target.triple.as_str(),
        target.os.as_str(),
        "not available for target",
        "when os",
    ] {
        assert!(err.contains(needle), "the refusal names {needle:?}: {err}");
    }
    assert!(
        !err.contains("compile-time code"),
        "ordinary code is not a compile-time context: {err}"
    );
}

#[test]
fn the_same_staged_source_builds_for_the_hosts_own_target() {
    // Nothing about the split changes the host-equals-target path: the
    // module resolves for both profiles, so the same source checks.
    let main = "module Main where\n  import Host.Only\n\n  stage flavour :: Int\n  stage flavour = host-flavour\n\n  main :: Int\n  main = 0\n";
    let dir = host_gated_project("native", main);
    let host_triple = host().triple;
    check_project(&dir, "Main.cinch", Some(&host_triple))
        .unwrap_or_else(|e| panic!("the host's own target builds: {e}"));
    // The manifest records the host next to the target, and says the two
    // are the same machine's profile.
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("the artifact manifest");
    assert!(manifest.contains("\"cross\":false"), "{manifest}");
    assert!(
        manifest.contains(&format!("\"triple\":\"{host_triple}\"")),
        "the host is recorded: {manifest}"
    );
    assert!(
        manifest.contains(
            "{\"host\":\"available\",\"module\":\"Host.Only\",\"target\":\"available\"}"
        ),
        "the module resolves for both profiles: {manifest}"
    );
}

#[test]
fn compile_time_code_that_reaches_a_target_only_module_names_the_host_context() {
    // The mirror direction: a module the target resolves and the host does
    // not. Compile-time code runs on the host, so it cannot reach it, and
    // the refusal says so rather than reporting an unknown name.
    let other_arch = if host().arch == "aarch64" { "x86_64" } else { "aarch64" };
    let manifest = format!(
        r#"{{
  "schema": 2,
  "name": "acme/app",
  "version": "0.1.0",
  "modules": [ {{ "name": "Main", "path": "Main.cinch" }} ],
  "order": ["Main"],
  "select": [
    {{ "when": {{ "arch": ["{other_arch}"] }},
      "modules": [ {{ "name": "Target.Only", "path": "platform/Target.cinch" }} ] }}
  ]
}}"#
    );
    let main = "module Main where\n  import Target.Only\n\n  stage flavour :: Int\n  stage flavour = target-flavour\n\n  main :: Int\n  main = 0\n";
    let dir = project(
        "target-only",
        &[
            (".cinchpkg", manifest.as_str()),
            (
                "platform/Target.cinch",
                "module Target.Only where\n  export target-flavour\n\n  target-flavour :: Int\n  target-flavour = 4\n",
            ),
            ("Main.cinch", main),
        ],
    );
    let triple = other_arch_triple();
    let err = check_project(&dir, "Main.cinch", Some(&triple))
        .expect_err("compile-time code cannot reach a target-only module");
    let target = profile(&triple);
    for needle in [
        "Target.Only",
        "compile-time code",
        "resolves for the target",
        "not for the host",
        host().triple.as_str(),
        target.triple.as_str(),
    ] {
        assert!(err.contains(needle), "the refusal names {needle:?}: {err}");
    }
}

#[test]
fn one_build_selects_different_modules_for_the_host_and_the_target() {
    // One build, two resolutions: the manifest's `select` plus an
    // in-source `when` choose a different implementation for each profile,
    // and the artifact records the split. The interface records the
    // target's selection only, because interfaces describe the artifact.
    let os = host().os;
    let other = if os == "darwin" { "linux" } else { "darwin" };
    let manifest = format!(
        r#"{{
  "schema": 2,
  "name": "acme/frames",
  "version": "0.1.0",
  "modules": [ {{ "name": "Main", "path": "Main.cinch" }} ],
  "order": [],
  "select": [
    {{ "when": {{ "os": ["{os}"] }},
      "modules": [ {{ "name": "Frame.Host", "path": "host/Frame.cinch" }} ] }},
    {{ "when": {{ "os": ["{other}"] }},
      "modules": [ {{ "name": "Frame.Cross", "path": "cross/Frame.cinch" }} ] }}
  ]
}}"#
    );
    let main = format!(
        "module Main where\n  import Frame.Host when os {os}\n  import Frame.Cross when os {other}\n\n  when os {os} main :: Int\n  when os {os} main = frame-name\n\n  when os {other} main :: Int\n  when os {other} main = frame-name\n"
    );
    let dir = project(
        "split",
        &[
            (".cinchpkg", manifest.as_str()),
            (
                "host/Frame.cinch",
                "module Frame.Host where\n  export frame-name\n\n  frame-name :: Int\n  frame-name = 1\n",
            ),
            (
                "cross/Frame.cinch",
                "module Frame.Cross where\n  export frame-name\n\n  frame-name :: Int\n  frame-name = 2\n",
            ),
            ("Main.cinch", main.as_str()),
        ],
    );
    let triple = other_os_triple();
    check_project(&dir, "Main.cinch", Some(&triple))
        .unwrap_or_else(|e| panic!("both profiles resolve in one build: {e}"));
    let manifest_text = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("the artifact manifest");
    assert!(manifest_text.contains("\"cross\":true"), "{manifest_text}");
    assert!(
        manifest_text
            .contains("{\"host\":\"available\",\"module\":\"Frame.Host\",\"target\":\"unavailable\"}"),
        "the host selects its implementation and the target does not: {manifest_text}"
    );
    assert!(
        manifest_text
            .contains("{\"host\":\"unavailable\",\"module\":\"Frame.Cross\",\"target\":\"available\"}"),
        "the target selects the other implementation: {manifest_text}"
    );
    // The interface records the target's selection only: the host's
    // module is not a dependency of the artifact.
    let text = std::fs::read_to_string(dir.join("build/Main.cinchif"))
        .expect("the module's interface");
    let iface = Iface::from_json(&text).expect("the interface parses");
    let imports = iface_imports(&iface);
    assert!(
        imports.contains("Frame.Cross"),
        "the target's selection is recorded: {imports:?}"
    );
    assert!(
        !imports.contains("Frame.Host"),
        "the host's selection is not: {imports:?}"
    );
}

#[test]
fn the_condition_context_is_the_one_place_a_position_picks_its_profile() {
    use cinch::core::{BuildContext, BuildProfiles};
    let target = profile(LINUX);
    let build = BuildProfiles::of_target(&target);
    assert_eq!(
        cinch::core::condition_context(&build, BuildContext::Emitted).triple,
        LINUX,
        "emitted code and its modules resolve against the target"
    );
    assert_eq!(
        cinch::core::condition_context(&build, BuildContext::CompileTime).triple,
        cinch::core::TargetProfile::native().triple,
        "compile-time code resolves against this machine, whatever the target is"
    );
    assert_eq!(
        build.cross(),
        build.host.triple != build.target.triple,
        "a build is a cross build exactly when the two profiles differ"
    );

    // Without `--target` the machine's own profile answers both ways, which
    // is the coincidence the split exists to make explicit.
    let native = BuildProfiles::native();
    assert_eq!(native.host.triple, native.target.triple);
    assert!(!native.cross());
}
