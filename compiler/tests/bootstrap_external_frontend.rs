//! The portable harness must invoke its selected frontend without Native fallback.
mod common;
use common::external_frontend::{drive, Frontend};
use cinch::driver::Mode;
use std::path::Path;

#[cfg(unix)]
fn fixture() -> (std::path::PathBuf, Frontend) {
    use std::os::unix::fs::PermissionsExt;
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!("cinch_external_frontend_{}_{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)));
    std::fs::create_dir_all(&dir).unwrap();
    // Deliberately invalid Cinch: a Native fallback cannot make this pass.
    std::fs::write(dir.join("entry with\na newline.cinch"), "THIS IS NOT A CINCH MODULE").unwrap();
    let compiler = dir.join("selected-compiler");
    std::fs::write(&compiler, r#"#!/bin/sh
printf invoked > selected-was-invoked
if [ -f fail-compiler ]; then printf 'selected checker refused' >&2; exit 4; fi
if [ "$1" = '--check' ]; then : > "$5.diagnostics"; exit 0; fi
if [ "$1" != '--compile-only' ]; then exit 91; fi
printf 'clang-exit: 0\n' > "$7"
cat > "$6" <<'PROGRAM'
#!/bin/sh
printf ran > "$0.ran"
pwd > "$0.cwd"
exit 37
PROGRAM
chmod +x "$6"
"#).unwrap();
    std::fs::set_permissions(&compiler, std::fs::Permissions::from_mode(0o755)).unwrap();
    let interfaces = dir.join("base.json");
    std::fs::write(&interfaces, "{\"schema\":1,\"interfaces\":[]}").unwrap();
    (dir, Frontend { compiler, interfaces })
}

#[cfg(unix)]
fn invocations(dir: &Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(dir.join(".cinch-self-tests")).unwrap().map(|e| e.unwrap().path()).collect()
}

#[test]
#[cfg(unix)]
fn selected_frontend_checks_builds_runs_and_records_the_actual_sources() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    assert_eq!(drive(&frontend, Mode::Check, &dir, entry, None).unwrap(), 0);
    assert!(dir.join("selected-was-invoked").exists());
    let first = invocations(&dir).pop().unwrap();
    let receipt = std::fs::read_to_string(first.join("invocation.json")).unwrap();
    assert!(receipt.contains("selected-compiler") && receipt.contains("entry with\\na newline.cinch"));
    let bundle = std::fs::read_to_string(first.join("source.cinch")).unwrap();
    assert_eq!(bundle.lines().next().unwrap(), "-- source: entry with\\na newline.cinch");
    let output = dir.join("built-program");
    assert_eq!(drive(&frontend, Mode::Build, &dir, entry, Some(output.clone())).unwrap(), 0);
    assert!(output.is_file());
    assert!(!output.with_extension("ran").exists(), "build executed its artifact");
    assert_eq!(drive(&frontend, Mode::Run, &dir, entry, None).unwrap(), 37);
    let ran = invocations(&dir).into_iter().find(|p| p.join("program.ran").exists()).expect("selected artifact ran");
    let cwd = std::fs::read_to_string(ran.join("program.cwd")).unwrap();
    assert_eq!(Path::new(cwd.trim()).canonicalize().unwrap(), std::env::current_dir().unwrap().canonicalize().unwrap(),
        "selected artifacts must inherit the same caller working directory as the Native driver");
    assert_eq!(invocations(&dir).len(), 3, "generated bundles were collected as new source inputs");
    common::cleanup(&dir);
}

#[test]
#[cfg(unix)]
fn selected_frontend_failures_and_non_base_interfaces_never_fall_back() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    std::fs::write(dir.join("fail-compiler"), "").unwrap();
    let error = drive(&frontend, Mode::Check, &dir, entry, None).unwrap_err();
    assert!(error.contains("selected checker refused"), "{error}");
    std::fs::remove_file(dir.join("selected-was-invoked")).unwrap();
    std::fs::write(&frontend.interfaces, "{\"schema\":1,\"interfaces\":[[\"Bootstrap.Infer\",\"hash\",\"body.cinchif\"]]}").unwrap();
    let error = drive(&frontend, Mode::Check, &dir, entry, None).unwrap_err();
    assert!(error.contains("distinct checked base interfaces"), "{error}");
    assert!(!dir.join("selected-was-invoked").exists());
    common::cleanup(&dir);
}

#[test]
fn explicit_native_library_fixture_produces_checked_interfaces() {
    let dir = common::project("explicit_native_library_fixture", &[
        ("Library.cinch", "module Library where\n  value :: Int\n  value = 37\n"),
        ("Main.cinch", "module Main where\n  import Library\n  main :: Int\n  main = Library.value\n"),
    ]);
    common::check_library_fixture_native(&dir, "Main.cinch").unwrap();
    assert!(dir.join("build/Library.cinchif").is_file());
    assert!(dir.join("build/Main.cinchif").is_file());
    common::cleanup(&dir);
}

#[test]
fn native_library_fixture_cannot_supply_bootstrap_implementation_interfaces() {
    let dir = common::project("forbidden_native_compiler_fixture", &[
        ("Bootstrap.Forbidden.cinch", "module Bootstrap.Forbidden where\n  value :: Int\n  value = 37\n"),
        ("Main.cinch", "module Main where\n  import Bootstrap.Forbidden\n  main :: Int\n  main = Forbidden.value\n"),
    ]);
    let error = common::check_library_fixture_native(&dir, "Main.cinch").unwrap_err();
    assert!(error.contains("compiler implementation module Bootstrap.Forbidden"), "{error}");
    common::cleanup(&dir);
}

#[test]
#[cfg(unix)]
fn selected_frontend_bundles_imports_and_preserves_lexical_data_files() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    std::fs::write(dir.join(entry), "module Main where\n  import Library\n  main :: Int\n  main = Library.value\n").unwrap();
    // A requested provider may declare several modules in one file. Imports
    // of every included module participate, including nested directories.
    std::fs::write(dir.join("provider.cinch"), "module Library where\n  import Helper\n  value :: Int\n  value = Helper.value\nmodule Side where\n  import Nested\n").unwrap();
    std::fs::write(dir.join("Helper.cinch"), "module Helper where\n  value :: Int\n  value = 37\n").unwrap();
    std::fs::create_dir(dir.join("nested")).unwrap();
    std::fs::write(dir.join("nested/Nested.cinch"), "module Nested where\n  value :: Int\n  value = 0\n").unwrap();
    std::fs::write(dir.join("attrs.cinch"), "module M where\nmodule M where\n").unwrap();
    std::fs::write(dir.join("ticks.cinch"), "not valid program source \"\n").unwrap();
    assert_eq!(drive(&frontend, Mode::Check, &dir, entry, None).unwrap(), 0);
    let invocation = invocations(&dir).pop().unwrap();
    let bundle = std::fs::read_to_string(invocation.join("source.cinch")).unwrap();
    let receipt = std::fs::read_to_string(invocation.join("invocation.json")).unwrap();
    for path in ["provider.cinch", "Helper.cinch", "nested/Nested.cinch"] { assert!(receipt.contains(path), "{receipt}"); }
    for data in ["attrs.cinch", "ticks.cinch"] { assert!(!receipt.contains(data), "{receipt}"); }
    assert!(bundle.contains("module Side") && bundle.contains("module Nested"));
    assert_eq!(std::fs::read_to_string(dir.join("attrs.cinch")).unwrap(), "module M where\nmodule M where\n");
    common::cleanup(&dir);
}

#[test]
#[cfg(unix)]
fn duplicate_requested_providers_remain_in_the_selected_compiler_input() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    std::fs::write(dir.join(entry), "module Main where\n  import Library\n").unwrap();
    for name in ["first.cinch", "second.cinch"] { std::fs::write(dir.join(name), "module Library where\n  value :: Int\n  value = 0\n").unwrap(); }
    drive(&frontend, Mode::Check, &dir, entry, None).unwrap();
    let invocation = invocations(&dir).pop().unwrap();
    let bundle = std::fs::read_to_string(invocation.join("source.cinch")).unwrap();
    assert_eq!(bundle.matches("module Library").count(), 2, "duplicate declarations must reach the selected frontend");
    common::cleanup(&dir);
}


#[test]
#[cfg(unix)]
fn object_output_invokes_selected_frontend_with_explicit_dependency_and_companions() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    let output = dir.join("library with spaces.o");
    let missing = common::external_frontend::drive_with_objects(&frontend, Mode::EmitObject, &dir, entry, Some(output.clone()), None).unwrap_err();
    assert!(missing.contains("explicit CINCH_TEST_OBJECTS"), "{missing}");
    assert!(!dir.join("selected-was-invoked").exists());
    let dependency = dir.join("dependency with spaces.json");
    std::fs::write(&dependency, "fixture dependency configuration").unwrap();
    std::fs::write(&frontend.compiler, r#"#!/bin/sh
if [ "$1" != '--emit-object' ] || [ "$2" != '--interfaces' ] || [ "$4" != '--objects' ] || [ "$#" != 10 ]; then exit 92; fi
printf invoked > selected-was-invoked
printf 'fixture IR' > "$7"
printf 'fixture object' > "$8"
printf 'fixture metadata' > "$9"
printf 'fixture interface' > "${10}Library.cinchif"
"#).unwrap();
    assert_eq!(common::external_frontend::drive_with_objects(&frontend, Mode::EmitObject, &dir, entry, Some(output.clone()), Some(dependency.clone())).unwrap(), 0);
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "fixture object");
    assert_eq!(std::fs::read_to_string(output.with_extension("ll")).unwrap(), "fixture IR");
    assert_eq!(std::fs::read_to_string(output.with_extension("abi.json")).unwrap(), "fixture metadata");
    assert!(dir.join("build/Library.cinchif").is_file());
    let receipt = invocations(&dir).into_iter().find_map(|p| std::fs::read_to_string(p.join("invocation.json")).ok()).unwrap();
    assert!(receipt.contains("--emit-object") && receipt.contains("--objects") && receipt.contains("dependency with spaces.json"), "{receipt}");
    // A second call still has to produce all companions; an exit code alone
    // cannot turn absent artifacts into an object-emission pass.
    std::fs::remove_file(output.with_extension("abi.json")).unwrap();
    std::fs::write(&frontend.compiler, "#!/bin/sh\nexit 0\n").unwrap();
    let error = common::external_frontend::drive_with_objects(&frontend, Mode::EmitObject, &dir, entry, Some(output), Some(dependency)).unwrap_err();
    assert!(error.contains("object companions"), "{error}");
    common::cleanup(&dir);
}


#[test]
#[cfg(unix)]
fn llvm_output_invokes_the_selected_frontend_and_requires_its_output() {
    let (dir, frontend) = fixture();
    let entry = "entry with\na newline.cinch";
    std::fs::write(&frontend.compiler, r#"#!/bin/sh
if [ "$1" != '--emit-ir' ] || [ "$2" != '--interfaces' ] || [ "$#" != 5 ]; then exit 93; fi
printf invoked > selected-was-invoked
printf 'selected LLVM output' > "$5"
"#).unwrap();
    let output = dir.join("LLVM with spaces.ll");
    assert_eq!(drive(&frontend, Mode::EmitIr, &dir, entry, Some(output.clone())).unwrap(), 0);
    assert_eq!(std::fs::read_to_string(output).unwrap(), "selected LLVM output");
    assert!(dir.join("selected-was-invoked").is_file());
    let configuration = dir.join("client objects.json");
    std::fs::write(&configuration, "client object configuration").unwrap();
    std::fs::write(&frontend.compiler, r#"#!/bin/sh
if [ "$1" != '--emit-ir' ] || [ "$2" != '--interfaces' ] || [ "$4" != '--objects' ] || [ "$#" != 7 ]; then exit 94; fi
if [ "$(cat "$5")" != 'client object configuration' ]; then exit 95; fi
printf 'LLVM for the selected object' > "$7"
"#).unwrap();
    let output = dir.join("object-client.ll");
    assert_eq!(common::external_frontend::drive_with_objects(
        &frontend, Mode::EmitIr, &dir, entry, Some(output.clone()), Some(configuration),
    ).unwrap(), 0);
    assert_eq!(std::fs::read_to_string(output).unwrap(), "LLVM for the selected object");
    std::fs::write(&frontend.compiler, "#!/bin/sh\nexit 0\n").unwrap();
    let error = drive(&frontend, Mode::EmitIr, &dir, entry, None).unwrap_err();
    assert!(error.contains("without LLVM output"), "{error}");
    common::cleanup(&dir);
}
