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
printf ran > selected-artifact-ran
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
    assert!(!dir.join("selected-artifact-ran").exists(), "build executed its artifact");
    assert_eq!(drive(&frontend, Mode::Run, &dir, entry, None).unwrap(), 37);
    assert!(dir.join("selected-artifact-ran").exists());
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
