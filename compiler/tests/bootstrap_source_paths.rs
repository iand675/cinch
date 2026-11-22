//! Fail before expensive rung compilation when cached paths name another checkout.
mod common;

use std::path::{Path, PathBuf};

#[test]
fn embedded_manifest_path_matches_the_active_package() {
    let active = std::env::var_os("CINCH_TEST_SOURCE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap());
    common::assert_source_root(Path::new(env!("CARGO_MANIFEST_DIR")), &active);
}

#[test]
fn another_existing_directory_cannot_substitute_for_the_package() {
    let other = std::env::temp_dir().join(format!("cinch_other_source_{}", std::process::id()));
    std::fs::create_dir_all(&other).unwrap();
    let result = std::panic::catch_unwind(|| {
        common::assert_source_root(Path::new(env!("CARGO_MANIFEST_DIR")), &other);
    });
    std::fs::remove_dir(&other).unwrap();
    assert!(
        result.is_err(),
        "a cached path for another checkout was accepted"
    );
}

#[cfg(unix)]
#[test]
fn aliases_of_the_same_source_directory_are_accepted() {
    let alias = std::env::temp_dir().join(format!("cinch_source_alias_{}", std::process::id()));
    let package = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::os::unix::fs::symlink(package, &alias).unwrap();
    common::assert_source_root(package, &alias);
    std::fs::remove_file(alias).unwrap();
}
