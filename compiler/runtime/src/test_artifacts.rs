//! Artifacts belonging to this Cargo test run, including --target-dir builds.
use std::path::PathBuf;

pub(crate) fn runtime_archives() -> Vec<PathBuf> {
    let executable = std::env::current_exe().expect("current runtime test executable");
    let directory = executable.parent().expect("runtime test artifact directory");
    let mut archives = std::fs::read_dir(directory)
        .expect("read current runtime artifact directory")
        .map(|entry| entry.expect("read runtime artifact entry").path())
        .filter(|path| {
            let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
            path.is_file() && (name == "libcinchruntime.a" || name == "libcinchruntime.rlib"
                || (name.starts_with("libcinchruntime-")
                    && (name.ends_with(".a") || name.ends_with(".rlib"))))
        })
        .collect::<Vec<_>>();
    archives.sort();
    assert!(!archives.is_empty(), "no runtime archives beside {}", executable.display());
    archives
}
