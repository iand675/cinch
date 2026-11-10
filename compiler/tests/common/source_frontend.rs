//! Shared source checks that use the selected frontend when configured.
use super::{cleanup, external_frontend, project_directory};
use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{self, Mode};
use cinch::interface::Iface;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn source_project<T>(
    name: &str,
    source: &str,
    run: impl FnOnce(&Path, &str) -> Result<T, String>,
) -> Result<T, String> {
    let dir = project_directory("source");
    let entry = if name.ends_with(".cinch") && Path::new(name).file_name().is_some_and(|file| file == name) {
        name
    } else {
        "Main.cinch"
    };
    std::fs::write(dir.join(entry), source).map_err(|error| error.to_string())?;
    let result = run(&dir, entry);
    if result.is_ok() {
        cleanup(&dir);
    }
    result
}

pub fn check_sources(source: &str, name: &str) -> Result<(), String> {
    match external_frontend::configured()? {
        Some(frontend) => source_project(name, source, |dir, entry| {
            external_frontend::drive(&frontend, Mode::Check, dir, entry, None).map(|_| ())
        }),
        None => driver::check_sources(source, name).map(|_| ()).map_err(|error| error.to_string()),
    }
}

pub fn check_source_interfaces(source: &str, name: &str) -> Result<Vec<Iface>, String> {
    if external_frontend::configured()?.is_some() {
        source_project(name, source, super::check_interfaces)
    } else {
        driver::check_sources_ifaces(source, name)
            .map(|(_, interfaces)| interfaces)
            .map_err(|error| error.to_string())
    }
}

pub fn run_source(source: &str) -> Result<i32, String> {
    match external_frontend::configured()? {
        Some(frontend) => source_project("Main.cinch", source, |dir, entry| {
            external_frontend::drive(&frontend, Mode::Run, dir, entry, None)
        }),
        None => driver::run_source(source).map_err(|error| error.to_string()),
    }
}

/// [`check_sources`] under an explicit target profile.
pub fn check_sources_profile(source: &str, name: &str, profile: &TargetProfile) -> Result<(), String> {
    match external_frontend::configured()? {
        Some(frontend) => {
            let request = external_frontend::profile_request(Mode::Check, StageMode::Refresh, profile)?;
            source_project(name, source, |dir, entry| {
                external_frontend::drive_profile(&frontend, Mode::Check, dir, entry, None, &request).map(|_| ())
            })
        }
        None => driver::check_sources_profile(source, name, profile).map(|_| ()).map_err(|error| error.to_string()),
    }
}

/// [`check_source_interfaces`] under an explicit target profile.
pub fn check_source_interfaces_profile(source: &str, name: &str, profile: &TargetProfile) -> Result<Vec<Iface>, String> {
    if external_frontend::configured()?.is_some() {
        source_project(name, source, |dir, entry| super::check_interfaces_profile(dir, entry, profile))
    } else {
        driver::check_sources_profile_ifaces(source, name, profile)
            .map(|(_, interfaces)| interfaces)
            .map_err(|error| error.to_string())
    }
}

/// [`run_source`] under an explicit target profile.
pub fn run_source_profile(source: &str, profile: &TargetProfile) -> Result<i32, String> {
    match external_frontend::configured()? {
        Some(frontend) => {
            let request = external_frontend::profile_request(Mode::Run, StageMode::Refresh, profile)?;
            source_project("Main.cinch", source, |dir, entry| {
                external_frontend::drive_profile(&frontend, Mode::Run, dir, entry, None, &request)
            })
        }
        None => driver::run_source_profile(source, profile).map_err(|error| error.to_string()),
    }
}

pub fn run_source_io(source: &str, input: &[u8]) -> Result<(i32, String), String> {
    match external_frontend::configured()? {
        Some(frontend) => run_linked_source(&frontend, source, input, &[]),
        None => driver::run_source_io(source, input).map_err(|error| error.to_string()),
    }
}

pub fn run_source_objects(source: &str, objects: &[&str]) -> Result<i32, String> {
    match external_frontend::configured()? {
        Some(frontend) => run_linked_source(&frontend, source, &[], objects).map(|(code, _)| code),
        None => driver::run_source_objects(source, objects).map_err(|error| error.to_string()),
    }
}

fn required_path(name: &str) -> Result<PathBuf, String> {
    std::env::var_os(name).filter(|value| !value.is_empty()).map(PathBuf::from)
        .ok_or_else(|| format!("{name} is required for the selected Cinch frontend"))
}

fn run_linked_source(
    frontend: &external_frontend::Frontend,
    source: &str,
    input: &[u8],
    objects: &[&str],
) -> Result<(i32, String), String> {
    source_project("Main.cinch", source, |dir, entry| {
        let ir = dir.join("program.ll");
        external_frontend::drive(frontend, Mode::EmitIr, dir, entry, Some(ir.clone()))?;
        let program = dir.join("program");
        let mut linker = Command::new(std::env::var_os("CINCH_CLANG").unwrap_or_else(|| "clang".into()));
        linker.arg(&ir).arg(required_path("CINCH_TEST_LIBRARY_OBJECT")?).args(objects)
            .arg(required_path("CINCH_TEST_RUNTIME_ARCHIVE")?).arg("-o").arg(&program);
        if cfg!(target_os = "linux") {
            linker.args(["-ldl", "-lpthread", "-lm"]);
        }
        let linked = linker.output().map_err(|error| error.to_string())?;
        if !linked.status.success() {
            return Err(format!("clang failed for {}: {}", dir.display(), String::from_utf8_lossy(&linked.stderr)));
        }
        let mut child = Command::new(&program)
            .env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1")
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
            .spawn().map_err(|error| error.to_string())?;
        let written = child.stdin.take().expect("piped stdin").write_all(input);
        let output = child.wait_with_output().map_err(|error| error.to_string())?;
        written.map_err(|error| error.to_string())?;
        let code = output.status.code().ok_or_else(|| format!("{} terminated without an exit code", program.display()))?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.matches("cinch-leak: live=0 immortal=0 leaked=0").count() != 1 {
            return Err(format!("runtime leak report for {}: {stderr}", program.display()));
        }
        Ok((code, String::from_utf8(output.stdout).map_err(|error| error.to_string())?))
    })
}
