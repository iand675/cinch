//! Install a registry package into a project and turn it into checked
//! interfaces and a native library object.
//!
//! The registry is the runnable v0 HTTP service in `registry/`. This client
//! deliberately accepts public packages only for now; authentication,
//! resolution ranges, and provenance verification remain follow-up work.
//! The installed source tree is placed under `vendor/`, which the source
//! loader indexes recursively.

use crate::driver::{driver_opts, DriverOptions, Mode};
use crate::{Error, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const DEFAULT_REGISTRY: &str = "http://127.0.0.1:8787";

/// What one install produced.
pub struct Installed {
    pub name: String,
    pub version: String,
    pub package_id: String,
    pub source_dir: PathBuf,
    pub object: PathBuf,
    pub abi: PathBuf,
    pub interfaces: PathBuf,
}

/// `cinch pkg ...`.
pub fn cli(args: &[String]) -> i32 {
    match run(args) {
        Ok(installed) => {
            println!("installed {} {}", installed.name, installed.version);
            println!("  source: {}", installed.source_dir.display());
            println!("  object: {}", installed.object.display());
            println!("  abi: {}", installed.abi.display());
            println!("  interfaces: {}", installed.interfaces.display());
            0
        }
        Err(error) => {
            eprintln!("cinch pkg: {error}");
            2
        }
    }
}

fn run(args: &[String]) -> Result<Installed> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(Error::plain(
            "usage: cinch pkg add NAME@VERSION [--registry URL] [--project DIR]",
        ));
    };
    match command {
        "add" | "install" => {
            let spec = args.get(1).ok_or_else(|| Error::plain("pkg add needs NAME@VERSION"))?;
            let mut registry = std::env::var("CINCH_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.into());
            let mut project = std::env::current_dir()
                .map_err(|e| Error::plain(format!("current directory: {e}")))?;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--registry" => {
                        i += 1;
                        registry = args.get(i).cloned().ok_or_else(|| Error::plain("--registry needs a URL"))?;
                    }
                    "--project" => {
                        i += 1;
                        project = PathBuf::from(args.get(i).ok_or_else(|| Error::plain("--project needs a directory"))?);
                    }
                    other => return Err(Error::plain(format!("unknown pkg flag: {other}"))),
                }
                i += 1;
            }
            install(&project, &registry, spec)
        }
        _ => Err(Error::plain(
            "usage: cinch pkg add NAME@VERSION [--registry URL] [--project DIR]",
        )),
    }
}

/// Fetch and install `name@version` from `registry` into `project`.
pub fn install(project: &Path, registry: &str, spec: &str) -> Result<Installed> {
    let (name, version) = spec
        .rsplit_once('@')
        .ok_or_else(|| Error::plain("package must be written NAME@VERSION"))?;
    validate_name(name)?;
    if version.is_empty() {
        return Err(Error::plain("package version is empty"));
    }
    let registry = registry.trim_end_matches('/');
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    install_recursive(project, registry, name, version, &mut seen)
}

fn install_recursive(
    project: &Path,
    registry: &str,
    name: &str,
    version: &str,
    seen: &mut BTreeMap<String, String>,
) -> Result<Installed> {
    if let Some(previous) = seen.get(name) {
        if previous == version {
            return existing_install(project, registry, name, version);
        }
        return Err(Error::plain(format!(
            "dependency cycle or version conflict for {name}: {previous} and {version}"
        )));
    }
    seen.insert(name.to_string(), version.to_string());
    let metadata = fetch_metadata(registry, name, version)?;
    if let Some(dependencies) = metadata
        .get("manifest")
        .and_then(|m| m.get("dependencies"))
        .and_then(Value::as_array)
    {
        for dependency in dependencies {
            let dep_name = dependency
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain("a dependency has no name"))?;
            if dep_name == "cinch/base" || dep_name == "cinch-base" {
                continue;
            }
            let source = dependency.get("source").and_then(Value::as_str).unwrap_or("registry");
            if source != "registry" {
                return Err(Error::plain(format!(
                    "dependency {dep_name} uses unsupported source {source}; only registry dependencies install today"
                )));
            }
            let dep_version = match dependency.get("version").and_then(Value::as_str) {
                Some(version) => version.to_string(),
                None => match dependency.get("range").and_then(Value::as_str) {
                    Some(range) => resolve_range(registry, dep_name, range)?,
                    None => resolve_range(registry, dep_name, "*")?,
                },
            };
            if seen.get(dep_name) == Some(&dep_version) {
                continue;
            }
            install_recursive(project, registry, dep_name, &dep_version, seen)?;
        }
    }
    let package_id = metadata
        .get("packageId")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("registry metadata has no packageId"))?
        .to_string();
    let hex = package_id
        .strip_prefix("sha256:")
        .ok_or_else(|| Error::plain("packageId is not a sha256 digest"))?;
    if hex.len() != 64 {
        return Err(Error::plain("packageId is not a 32-byte SHA-256 digest"));
    }
    let download_url = format!("{registry}/api/v1/download/{hex}");
    let bundle: Value = serde_json::from_slice(&curl(&download_url)?)
        .map_err(|e| Error::plain(format!("registry bundle: {e}")))?;
    install_bundle(project, registry, &package_id, &bundle, metadata.get("files"))
}

fn fetch_metadata(registry: &str, name: &str, version: &str) -> Result<Value> {
    let encoded = name.replace('/', "%2F");
    let url = format!("{registry}/api/v1/packages/{encoded}/{version}");
    serde_json::from_slice(&curl(&url)?).map_err(|e| Error::plain(format!("registry metadata: {e}")))
}

fn existing_install(project: &Path, registry: &str, name: &str, version: &str) -> Result<Installed> {
    let (namespace, package) = name
        .split_once('/')
        .ok_or_else(|| Error::plain("package name must be namespace/name"))?;
    let source_dir = project.join("vendor").join(namespace).join(package).join(version);
    let object = source_dir.join("package.o");
    let abi = source_dir.join("package.abi.json");
    let interfaces = source_dir.join("build");
    if !object.exists() || !abi.exists() || !interfaces.exists() {
        return Err(Error::plain(format!(
            "{name}@{version} is already in the dependency resolution but its artifacts are missing; remove vendor/{namespace}/{package} and retry"
        )));
    }
    Ok(Installed {
        name: name.to_string(),
        version: version.to_string(),
        package_id: String::new(),
        source_dir,
        object,
        abi,
        interfaces,
    })
}

fn resolve_range(registry: &str, name: &str, range: &str) -> Result<String> {
    let encoded = name.replace('/', "%2F");
    let url = format!("{registry}/api/v1/packages/{encoded}");
    let metadata: Value = serde_json::from_slice(&curl(&url)?)
        .map_err(|e| Error::plain(format!("registry versions for {name}: {e}")))?;
    let versions = metadata
        .get("versions")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(format!("registry has no versions for {name}")))?;
    let mut best: Option<(Vec<u64>, String)> = None;
    for row in versions {
        let Some(version) = row.get("version").and_then(Value::as_str) else { continue };
        if row.get("blocked").and_then(Value::as_bool).unwrap_or(false) {
            continue;
        }
        if !matches_range(version, range) {
            continue;
        }
        let key = semver_key(version)?;
        if best.as_ref().map(|(current, _)| key > *current).unwrap_or(true) {
            best = Some((key, version.to_string()));
        }
    }
    best.map(|(_, version)| version)
        .ok_or_else(|| Error::plain(format!("no version of {name} matches {range}")))
}

fn semver_key(version: &str) -> Result<Vec<u64>> {
    let core = version.split(['-', '+']).next().unwrap_or(version);
    let mut parts = core.split('.');
    let mut key = Vec::new();
    for _ in 0..3 {
        let part = parts.next().unwrap_or("0");
        key.push(part.parse::<u64>().map_err(|_| Error::plain(format!("unsupported version {version}")))?);
    }
    Ok(key)
}

fn matches_range(version: &str, range: &str) -> bool {
    if range == "*" || range.is_empty() {
        return true;
    }
    if let Some(exact) = range.strip_prefix('=') {
        return version == exact;
    }
    if let Some(caret) = range.strip_prefix('^') {
        let Ok(want) = semver_key(caret) else { return false };
        let Ok(got) = semver_key(version) else { return false };
        return got[0] == want[0] && got >= want;
    }
    version == range
}

/// Install a bundle already fetched from a registry. The bundle shape is
/// `{ schema, manifest, files: { path: base64 }, visibility, readme }`.
pub fn install_bundle(
    project: &Path,
    registry: &str,
    package_id: &str,
    bundle: &Value,
    metadata_files: Option<&Value>,
) -> Result<Installed> {
    let manifest = bundle
        .get("manifest")
        .ok_or_else(|| Error::plain("bundle has no manifest"))?;
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no name"))?;
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no version"))?;
    validate_name(name)?;
    let files = bundle
        .get("files")
        .and_then(Value::as_object)
        .ok_or_else(|| Error::plain("bundle has no files object"))?;
    let mut decoded: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for (path, encoded) in files {
        let encoded = encoded
            .as_str()
            .ok_or_else(|| Error::plain(format!("file {path} is not a base64 string")))?;
        let bytes = base64_decode(encoded)
            .map_err(|e| Error::plain(format!("file {path}: {e}")))?;
        decoded.insert(path.clone(), bytes);
    }
    verify_identity(package_id, manifest, &decoded, metadata_files)?;

    let modules = manifest
        .get("modules")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain("manifest has no modules"))?;
    let mut module_names: Vec<String> = Vec::new();
    for module in modules {
        let module_name = module
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("module entry has no name"))?;
        let path = module
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("module entry has no path"))?;
        if !decoded.contains_key(path) {
            return Err(Error::plain(format!("module {module_name} path {path} is not in the bundle")));
        }
        module_names.push(module_name.to_string());
    }
    if module_names.is_empty() {
        return Err(Error::plain("package has no modules"));
    }

    let namespace = name
        .split_once('/')
        .map(|(namespace, _)| namespace)
        .ok_or_else(|| Error::plain("package name must be namespace/name"))?;
    let source_dir = project.join("vendor").join(namespace).join(name.split_once('/').expect("checked").1).join(version);
    std::fs::create_dir_all(&source_dir)
        .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    for (path, bytes) in &decoded {
        let relative = safe_relative(path)?;
        let target = source_dir.join(relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::plain(format!("{}: {e}", parent.display())))?;
        }
        std::fs::write(&target, bytes)
            .map_err(|e| Error::plain(format!("{}: {e}", target.display())))?;
    }
    std::fs::write(source_dir.join(".cinchpkg"), canonical(manifest)?)
        .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    if let Some(readme) = bundle.get("readme").and_then(Value::as_str) {
        std::fs::write(source_dir.join("README.md"), readme)
            .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    }

    // A synthetic library root imports every package module. It lives at the
    // project root so dependency modules vendored beside the project resolve
    // through the same recursive source index the project uses. `emit-object`
    // checks the package and writes its native object plus ABI manifest.
    let entry = project.join(".cinch-package-root.cinch");
    let mut source = String::from("module CinchPackageRoot where\n");
    for module in &module_names {
        source.push_str(&format!("  import {module}\n"));
    }
    std::fs::write(&entry, source)
        .map_err(|e| Error::plain(format!("{}: {e}", entry.display())))?;
    let object = source_dir.join("package.o");
    let abi = source_dir.join("package.abi.json");
    let built = driver_opts(Mode::EmitObject, &entry, Some(&object), &DriverOptions::default());
    let _ = std::fs::remove_file(&entry);
    built.map_err(|e| Error::plain(format!("build package object: {e}")))?;
    let interfaces = source_dir.join("build");
    std::fs::create_dir_all(&interfaces)
        .map_err(|e| Error::plain(format!("{}: {e}", interfaces.display())))?;
    let project_build = project.join("build");
    for module in module_names.iter().map(String::as_str).chain(std::iter::once("CinchPackageRoot")) {
        let from = project_build.join(format!("{module}.cinchif"));
        let to = interfaces.join(format!("{module}.cinchif"));
        if from.exists() {
            std::fs::copy(&from, &to)
                .map_err(|e| Error::plain(format!("{}: {e}", to.display())))?;
        }
    }
    if !object.exists() || !abi.exists() || !interfaces.exists() {
        return Err(Error::plain("package object emission did not produce its artifacts"));
    }

    let relative_source = source_dir
        .strip_prefix(project)
        .unwrap_or(&source_dir)
        .display()
        .to_string();
    update_lock(
        project,
        name,
        version,
        package_id,
        registry,
        &relative_source,
        &object.strip_prefix(project).unwrap_or(&object).display().to_string(),
        &interfaces.strip_prefix(project).unwrap_or(&interfaces).display().to_string(),
    )?;

    Ok(Installed {
        name: name.to_string(),
        version: version.to_string(),
        package_id: package_id.to_string(),
        source_dir,
        object,
        abi,
        interfaces,
    })
}

fn validate_name(name: &str) -> Result<()> {
    let Some((namespace, package)) = name.split_once('/') else {
        return Err(Error::plain("package name must be namespace/name"));
    };
    if namespace.is_empty() || package.is_empty() || package.contains('/') {
        return Err(Error::plain("package name must contain exactly one `/`"));
    }
    let valid = |s: &str| {
        !s.is_empty()
            && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    };
    if !valid(namespace) || !valid(package) {
        return Err(Error::plain("package names use lowercase letters, digits, `-`, and `_`"));
    }
    Ok(())
}

fn safe_relative(path: &str) -> Result<PathBuf> {
    let path = Path::new(path);
    if path.is_absolute() || path.as_os_str().is_empty() {
        return Err(Error::plain("package file path must be relative"));
    }
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => out.push(part),
            _ => return Err(Error::plain("package file path may not contain `..` or a root")),
        }
    }
    Ok(out)
}

fn canonical(value: &Value) -> Result<String> {
    let text = serde_json::to_string(value)
        .map_err(|e| Error::plain(format!("canonical JSON: {e}")))?;
    let parsed = crate::interface::parse_json(&text)
        .map_err(|e| Error::plain(format!("canonical JSON: {e}")))?;
    Ok(parsed.to_canonical_string())
}

fn verify_identity(
    package_id: &str,
    manifest: &Value,
    files: &BTreeMap<String, Vec<u8>>,
    metadata_files: Option<&Value>,
) -> Result<()> {
    if let Some(metadata_files) = metadata_files.and_then(Value::as_object) {
        for (path, expected) in metadata_files {
            let expected = expected.as_str().unwrap_or_default();
            let actual = sha256_hex(files.get(path).map(Vec::as_slice).unwrap_or_default());
            if expected != actual {
                return Err(Error::plain(format!("file hash mismatch for {path}")));
            }
        }
    }
    let manifest = canonical(manifest)?;
    let mut rows = Vec::new();
    for (path, bytes) in files {
        rows.push(format!("{path}\t{}", sha256_hex(bytes)));
    }
    let text = format!("package\n{manifest}\n{}", rows.join("\n"));
    let actual = format!("sha256:{}", sha256_hex(text.as_bytes()));
    if actual != package_id {
        return Err(Error::plain(format!(
            "package identity mismatch: registry says {package_id}, bundle is {actual}"
        )));
    }
    Ok(())
}

fn update_lock(
    project: &Path,
    name: &str,
    version: &str,
    package_id: &str,
    registry: &str,
    source: &str,
    object: &str,
    interfaces: &str,
) -> Result<()> {
    let path = project.join(".cinchlock");
    let mut lock: Value = if path.exists() {
        let bytes = std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        serde_json::from_slice(&bytes).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?
    } else {
        json!({"schema": 1, "packages": []})
    };
    let packages = lock
        .get_mut("packages")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| Error::plain(".cinchlock has no packages array"))?;
    packages.retain(|row| row.get("name").and_then(Value::as_str) != Some(name));
    packages.push(json!({
        "name": name,
        "version": version,
        "package_id": package_id,
        "source": {"kind": "registry", "registry": registry},
        "source_dir": source,
        "object": object,
        "interfaces": interfaces
    }));
    std::fs::write(&path, canonical(&lock)?)
        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    Ok(())
}

fn curl(url: &str) -> Result<Vec<u8>> {
    let output = Command::new("curl")
        .args(["-fsSL", "--max-time", "60"])
        .arg(url)
        .output()
        .map_err(|e| Error::plain(format!("run curl: {e}")))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(Error::plain(format!("GET {url}: {}", detail.trim())));
    }
    Ok(output.stdout)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(bytes);
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn base64_decode(text: &str) -> std::result::Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for byte in text.bytes() {
        if byte == b'=' || byte == b'\n' || byte == b'\r' {
            continue;
        }
        let value = match byte {
            b'A'..=b'Z' => (byte - b'A') as u32,
            b'a'..=b'z' => (byte - b'a' + 26) as u32,
            b'0'..=b'9' => (byte - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => return Err("invalid base64".into()),
        };
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base64_encode(bytes: &[u8]) -> String {
        const TABLE: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
            let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
            out.push(TABLE[((n >> 18) & 63) as usize] as char);
            out.push(TABLE[((n >> 12) & 63) as usize] as char);
            out.push(if chunk.len() > 1 { TABLE[((n >> 6) & 63) as usize] as char } else { '=' });
            out.push(if chunk.len() > 2 { TABLE[(n & 63) as usize] as char } else { '=' });
        }
        out
    }

    #[test]
    fn caret_ranges_select_compatible_majors() {
        assert!(matches_range("0.1.0", "^0.1.0"));
        assert!(matches_range("0.9.4", "^0.1.0"));
        assert!(!matches_range("1.0.0", "^0.1.0"));
        assert!(matches_range("2.3.1", "^2.0.0"));
        assert!(!matches_range("3.0.0", "^2.0.0"));
        assert!(matches_range("0.1.0", "=0.1.0"));
        assert!(!matches_range("0.1.1", "=0.1.0"));
    }

    #[test]
    fn bundle_materializes_sources_and_an_object() {
        let dir = std::env::temp_dir().join(format!("cinch-pkg-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("project");
        let source = b"module Library where\n  value :: Int\n  value = 42\n";
        let files = json!({"Library.cinch": base64_encode(source)});
        let manifest = json!({
            "schema": 2,
            "name": "acme/library",
            "version": "0.1.0",
            "modules": [{"name": "Library", "path": "Library.cinch"}],
            "order": ["Library"]
        });
        let mut decoded = BTreeMap::new();
        decoded.insert("Library.cinch".to_string(), source.to_vec());
        let package_id = format!("sha256:{}", {
            let text = format!(
                "package\n{}\nLibrary.cinch\t{}",
                canonical(&manifest).unwrap(),
                sha256_hex(source)
            );
            sha256_hex(text.as_bytes())
        });
        let bundle = json!({
            "schema": 1,
            "manifest": manifest,
            "files": files,
            "visibility": "public"
        });
        let installed = install_bundle(&dir, "http://registry.local", &package_id, &bundle, None)
            .expect("install bundle");
        assert!(installed.source_dir.join("Library.cinch").exists());
        assert!(installed.object.exists());
        assert!(installed.interfaces.join("Library.cinchif").exists());
        let lock = std::fs::read_to_string(dir.join(".cinchlock")).expect("lock");
        assert!(lock.contains("acme/library"));
        std::fs::write(
            dir.join("Main.cinch"),
            "module Main where\n  import Library\n  main :: Int\n  main = value + 1\n",
        )
        .expect("project source");
        driver_opts(Mode::Check, &dir.join("Main.cinch"), None, &DriverOptions::default())
            .expect("project imports the installed package");
        std::fs::remove_dir_all(&dir).ok();
    }
}
