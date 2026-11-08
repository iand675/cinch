//! Install a registry package into a project and turn it into checked
//! interfaces and a native library object.
//!
//! The registry is the runnable v0 HTTP service in `registry/`. Reads accept
//! a bearer token for private packages; `pkg publish` uses the same token.
//! Resolution supports exact versions and common semver ranges, with conflict
//! checks when several dependencies constrain one package name. The installed
//! source tree is placed under `vendor/`, which the source loader indexes
//! recursively.

use crate::driver::{driver_opts, DriverOptions, Mode};
use crate::{Error, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};

const DEFAULT_REGISTRY: &str = "http://127.0.0.1:8787";

const USAGE: &str = "usage:
  cinch pkg add NAME@VERSION [--registry URL] [--project DIR] [--token TOKEN]
  cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
  cinch pkg verify [--project DIR]";

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

/// What one publish produced.
pub struct Published {
    pub name: String,
    pub version: String,
    pub package_id: String,
}

/// What one lock verification produced.
pub struct Verified {
    pub packages: usize,
}

enum Output {
    Installed(Installed),
    Published(Published),
    Verified(Verified),
}

/// `cinch pkg ...`.
pub fn cli(args: &[String]) -> i32 {
    match run(args) {
        Ok(Output::Installed(installed)) => {
            println!("installed {} {}", installed.name, installed.version);
            println!("  source: {}", installed.source_dir.display());
            println!("  object: {}", installed.object.display());
            println!("  abi: {}", installed.abi.display());
            println!("  interfaces: {}", installed.interfaces.display());
            0
        }
        Ok(Output::Published(published)) => {
            println!("published {} {}", published.name, published.version);
            println!("  package: {}", published.package_id);
            0
        }
        Ok(Output::Verified(verified)) => {
            println!("verified {} package(s)", verified.packages);
            0
        }
        Err(error) => {
            eprintln!("cinch pkg: {error}");
            2
        }
    }
}

fn run(args: &[String]) -> Result<Output> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(Error::plain(USAGE));
    };
    match command {
        "add" | "install" => {
            let spec = args
                .get(1)
                .ok_or_else(|| Error::plain("pkg add needs NAME@VERSION"))?;
            let flags = Flags::parse(&args[2..], false)?;
            install_with_token(
                &flags.project,
                &flags.registry,
                spec,
                flags.token.as_deref(),
            )
            .map(Output::Installed)
        }
        "publish" => {
            let flags = Flags::parse(&args[1..], true)?;
            let token = flags
                .token
                .as_deref()
                .ok_or_else(|| Error::plain("publish needs --token or CINCH_REGISTRY_TOKEN"))?;
            let visibility = flags.visibility.as_deref().unwrap_or("public");
            publish(&flags.project, &flags.registry, token, visibility).map(Output::Published)
        }
        "verify" => {
            let flags = Flags::parse(&args[1..], false)?;
            verify(&flags.project).map(Output::Verified)
        }
        _ => Err(Error::plain(USAGE)),
    }
}

struct Flags {
    registry: String,
    project: PathBuf,
    token: Option<String>,
    visibility: Option<String>,
}

impl Flags {
    fn parse(args: &[String], allow_visibility: bool) -> Result<Flags> {
        let mut flags = Flags {
            registry: std::env::var("CINCH_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.into()),
            project: std::env::current_dir()
                .map_err(|e| Error::plain(format!("current directory: {e}")))?,
            token: std::env::var("CINCH_REGISTRY_TOKEN")
                .ok()
                .filter(|t| !t.is_empty()),
            visibility: None,
        };
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--registry" => {
                    i += 1;
                    flags.registry = args
                        .get(i)
                        .cloned()
                        .ok_or_else(|| Error::plain("--registry needs a URL"))?;
                }
                "--project" => {
                    i += 1;
                    flags.project = PathBuf::from(
                        args.get(i)
                            .ok_or_else(|| Error::plain("--project needs a directory"))?,
                    );
                }
                "--token" => {
                    i += 1;
                    flags.token = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--token needs a value"))?,
                    );
                }
                "--visibility" if allow_visibility => {
                    i += 1;
                    flags.visibility = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--visibility needs public or private"))?,
                    );
                }
                other => return Err(Error::plain(format!("unknown pkg flag: {other}"))),
            }
            i += 1;
        }
        flags.registry = flags.registry.trim_end_matches('/').to_string();
        Ok(flags)
    }
}

/// Fetch and install `name@version` from `registry` into `project`.
pub fn install(project: &Path, registry: &str, spec: &str) -> Result<Installed> {
    install_with_token(project, registry, spec, None)
}

fn install_with_token(
    project: &Path,
    registry: &str,
    spec: &str,
    token: Option<&str>,
) -> Result<Installed> {
    let (name, version) = spec
        .rsplit_once('@')
        .ok_or_else(|| Error::plain("package must be written NAME@VERSION"))?;
    validate_name(name)?;
    if version.is_empty() {
        return Err(Error::plain("package version is empty"));
    }
    let registry = registry.trim_end_matches('/');
    let resolved = resolve_graph(registry, name, version, token)?;
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    install_recursive(
        project, registry, name, version, token, &resolved, &mut seen,
    )
}

fn install_recursive(
    project: &Path,
    registry: &str,
    name: &str,
    version: &str,
    token: Option<&str>,
    resolved: &BTreeMap<String, String>,
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
    let metadata = fetch_metadata(registry, name, version, token)?;
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
            validate_name(dep_name)?;
            let source = dependency
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or("registry");
            if source != "registry" {
                return Err(Error::plain(format!(
                    "dependency {dep_name} uses unsupported source {source}; only registry dependencies install today"
                )));
            }
            let requirement = Requirement::from_dependency(dependency)?;
            let dep_version = resolved.get(dep_name).cloned().ok_or_else(|| {
                Error::plain(format!(
                    "resolution did not select a version for {dep_name}"
                ))
            })?;
            if !requirement.matches(&dep_version) {
                return Err(Error::plain(format!(
                    "dependency conflict for {dep_name}: selected {dep_version}, but {name} requires {}",
                    requirement.describe()
                )));
            }
            if seen.get(dep_name) == Some(&dep_version) {
                continue;
            }
            install_recursive(
                project,
                registry,
                dep_name,
                &dep_version,
                token,
                resolved,
                seen,
            )?;
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
    let bundle: Value = serde_json::from_slice(&curl(&download_url, token)?)
        .map_err(|e| Error::plain(format!("registry bundle: {e}")))?;
    install_bundle(
        project,
        registry,
        &package_id,
        &bundle,
        metadata.get("files"),
    )
}

#[derive(Clone, PartialEq, Eq)]
enum Requirement {
    Exact(String),
    Range(String),
}

impl Requirement {
    fn from_dependency(dependency: &Value) -> Result<Requirement> {
        if let Some(version) = dependency.get("version").and_then(Value::as_str) {
            semver_key(version)?;
            return Ok(Requirement::Exact(version.to_string()));
        }
        let range = dependency
            .get("range")
            .and_then(Value::as_str)
            .unwrap_or("*");
        Ok(Requirement::Range(range.to_string()))
    }

    fn matches(&self, version: &str) -> bool {
        match self {
            Requirement::Exact(want) => want == version,
            Requirement::Range(range) => matches_range(version, range),
        }
    }

    fn describe(&self) -> String {
        match self {
            Requirement::Exact(version) => format!("={version}"),
            Requirement::Range(range) => range.clone(),
        }
    }
}

fn fetch_metadata(registry: &str, name: &str, version: &str, token: Option<&str>) -> Result<Value> {
    let encoded = name.replace('/', "%2F");
    let url = format!("{registry}/api/v1/packages/{encoded}/{version}");
    serde_json::from_slice(&curl(&url, token)?)
        .map_err(|e| Error::plain(format!("registry metadata: {e}")))
}

fn existing_install(
    project: &Path,
    registry: &str,
    name: &str,
    version: &str,
) -> Result<Installed> {
    let (namespace, package) = name
        .split_once('/')
        .ok_or_else(|| Error::plain("package name must be namespace/name"))?;
    let source_dir = project
        .join("vendor")
        .join(namespace)
        .join(package)
        .join(version);
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

fn registry_versions(registry: &str, name: &str, token: Option<&str>) -> Result<Vec<String>> {
    let encoded = name.replace('/', "%2F");
    let url = format!("{registry}/api/v1/packages/{encoded}");
    let metadata: Value = serde_json::from_slice(&curl(&url, token)?)
        .map_err(|e| Error::plain(format!("registry versions for {name}: {e}")))?;
    let versions = metadata
        .get("versions")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(format!("registry has no versions for {name}")))?;
    let mut out = Vec::new();
    for row in versions {
        let Some(version) = row.get("version").and_then(Value::as_str) else {
            continue;
        };
        if row.get("blocked").and_then(Value::as_bool).unwrap_or(false)
            || row.get("yanked").and_then(Value::as_bool).unwrap_or(false)
        {
            continue;
        }
        out.push(version.to_string());
    }
    if out.is_empty() {
        return Err(Error::plain(format!(
            "registry has no installable versions for {name}"
        )));
    }
    Ok(out)
}

fn select_version(name: &str, requirements: &[Requirement], versions: &[String]) -> Result<String> {
    let exact: Vec<&str> = requirements
        .iter()
        .filter_map(|requirement| match requirement {
            Requirement::Exact(version) => Some(version.as_str()),
            Requirement::Range(_) => None,
        })
        .collect();
    let describe = || {
        requirements
            .iter()
            .map(Requirement::describe)
            .collect::<Vec<_>>()
            .join(", ")
    };
    if let Some(first) = exact.first() {
        if exact.iter().all(|version| version == first)
            && requirements
                .iter()
                .all(|requirement| requirement.matches(first))
        {
            return Ok((*first).to_string());
        }
        return Err(Error::plain(format!(
            "no version of {name} satisfies {}",
            describe()
        )));
    }
    let mut best: Option<(Vec<u64>, String)> = None;
    for version in versions {
        if !requirements
            .iter()
            .all(|requirement| requirement.matches(version))
        {
            continue;
        }
        let key = semver_key(version)?;
        if best
            .as_ref()
            .map(|(current, _)| key > *current)
            .unwrap_or(true)
        {
            best = Some((key, version.clone()));
        }
    }
    best.map(|(_, version)| version).ok_or_else(|| {
        Error::plain(format!(
            "no version of {name} satisfies {}; available {}",
            describe(),
            versions.join(", ")
        ))
    })
}

/// Resolve one version per package name by iterating requirements to a fixed
/// point. Each round selects the highest version satisfying the constraints
/// currently reachable from the root, then recomputes the constraints from the
/// selected versions. A package that becomes unreachable is dropped.
fn resolve_graph(
    registry: &str,
    root_name: &str,
    root_version: &str,
    token: Option<&str>,
) -> Result<BTreeMap<String, String>> {
    let root_requirement = Requirement::Exact(root_version.to_string());
    let mut constraints: BTreeMap<String, Vec<Requirement>> = BTreeMap::new();
    constraints.insert(root_name.to_string(), vec![root_requirement.clone()]);
    let mut selected: BTreeMap<String, String> = BTreeMap::new();
    let mut versions_cache: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut metadata_cache: BTreeMap<(String, String), Value> = BTreeMap::new();
    for _ in 0..128 {
        let mut changed = false;
        for (package, requirements) in &constraints {
            let versions = if let Some(versions) = versions_cache.get(package) {
                versions.clone()
            } else {
                let versions = registry_versions(registry, package, token)?;
                versions_cache.insert(package.clone(), versions.clone());
                versions
            };
            let candidate = select_version(package, requirements, &versions)?;
            if selected.get(package) != Some(&candidate) {
                selected.insert(package.clone(), candidate);
                changed = true;
            }
        }
        let mut next: BTreeMap<String, Vec<Requirement>> = BTreeMap::new();
        next.insert(root_name.to_string(), vec![root_requirement.clone()]);
        let snapshot: Vec<(String, String)> = selected
            .iter()
            .map(|(package, version)| (package.clone(), version.clone()))
            .collect();
        for (package, version) in &snapshot {
            let key = (package.clone(), version.clone());
            let metadata = if let Some(metadata) = metadata_cache.get(&key) {
                metadata.clone()
            } else {
                let metadata = fetch_metadata(registry, package, version, token)?;
                metadata_cache.insert(key, metadata.clone());
                metadata
            };
            if let Some(dependencies) = metadata
                .get("manifest")
                .and_then(|manifest| manifest.get("dependencies"))
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
                    validate_name(dep_name)?;
                    let source = dependency
                        .get("source")
                        .and_then(Value::as_str)
                        .unwrap_or("registry");
                    if source != "registry" {
                        return Err(Error::plain(format!(
                            "dependency {dep_name} uses unsupported source {source}; only registry dependencies install today"
                        )));
                    }
                    next.entry(dep_name.to_string())
                        .or_default()
                        .push(Requirement::from_dependency(dependency)?);
                }
            }
        }
        selected.retain(|package, _| next.contains_key(package));
        if next != constraints {
            constraints = next;
            changed = true;
        }
        if !changed {
            return Ok(selected);
        }
    }
    Err(Error::plain(format!(
        "dependency resolution for {root_name}@{root_version} did not settle"
    )))
}

fn semver_key(version: &str) -> Result<Vec<u64>> {
    let version = version.strip_prefix('v').unwrap_or(version);
    let core = version.split(['-', '+']).next().unwrap_or(version);
    let mut parts = core.split('.');
    let mut key = Vec::new();
    for _ in 0..3 {
        let part = parts.next().unwrap_or("0");
        key.push(
            part.parse::<u64>()
                .map_err(|_| Error::plain(format!("unsupported version {version}")))?,
        );
    }
    if parts.next().is_some() {
        return Err(Error::plain(format!("unsupported version {version}")));
    }
    Ok(key)
}

/// Semver-style range matching. A range is a comma-separated intersection of
/// comparators (`^`, `~`, `=`, `>=`, `>`, `<=`, `<`, wildcards, or an exact
/// version). Spaces around commas are accepted. Pre-release ordering is out
/// of scope; the numeric core is compared.
fn matches_range(version: &str, range: &str) -> bool {
    let specs: Vec<&str> = range
        .split(',')
        .map(str::trim)
        .filter(|spec| !spec.is_empty())
        .collect();
    if specs.is_empty() {
        return true;
    }
    specs.iter().all(|spec| matches_comparator(version, spec))
}

fn matches_comparator(version: &str, spec: &str) -> bool {
    if spec == "*" {
        return true;
    }
    let Ok(got) = semver_key(version) else {
        return false;
    };
    let wildcard = |want: &str| -> Option<Vec<Option<u64>>> {
        let parts: Vec<Option<u64>> = want
            .split('.')
            .map(|part| {
                if part == "*" || part.eq_ignore_ascii_case("x") {
                    None
                } else {
                    part.parse::<u64>().ok()
                }
            })
            .collect();
        if parts.len() > 3 || parts.iter().all(Option::is_none) {
            return None;
        }
        Some(parts)
    };
    let caret = |want: &str| -> bool {
        let Ok(want) = semver_key(want) else {
            return false;
        };
        if got < want {
            return false;
        }
        if want[0] > 0 {
            got[0] == want[0]
        } else if want[1] > 0 {
            got[0] == 0 && got[1] == want[1]
        } else {
            got == want
        }
    };
    let tilde = |want: &str| -> bool {
        let Ok(want) = semver_key(want) else {
            return false;
        };
        got >= want && got[0] == want[0] && got[1] == want[1]
    };
    let compare = |want: &str| -> Option<std::cmp::Ordering> {
        semver_key(want).ok().map(|want| got.cmp(&want))
    };
    if let Some(want) = spec.strip_prefix("^") {
        return caret(want);
    }
    if let Some(want) = spec.strip_prefix("~") {
        return tilde(want);
    }
    if let Some(want) = spec.strip_prefix(">=") {
        return compare(want)
            .map(|ord| ord != std::cmp::Ordering::Less)
            .unwrap_or(false);
    }
    if let Some(want) = spec.strip_prefix("<=") {
        return compare(want)
            .map(|ord| ord != std::cmp::Ordering::Greater)
            .unwrap_or(false);
    }
    if let Some(want) = spec.strip_prefix('>') {
        return compare(want) == Some(std::cmp::Ordering::Greater);
    }
    if let Some(want) = spec.strip_prefix('<') {
        return compare(want) == Some(std::cmp::Ordering::Less);
    }
    if let Some(want) = spec.strip_prefix('=') {
        return compare(want) == Some(std::cmp::Ordering::Equal);
    }
    if let Some(parts) = wildcard(spec) {
        for (index, part) in parts.iter().enumerate() {
            if let Some(part) = part {
                if got[index] != *part {
                    return false;
                }
            } else {
                return true;
            }
        }
        return true;
    }
    compare(spec) == Some(std::cmp::Ordering::Equal)
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
        let bytes =
            base64_decode(encoded).map_err(|e| Error::plain(format!("file {path}: {e}")))?;
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
            return Err(Error::plain(format!(
                "module {module_name} path {path} is not in the bundle"
            )));
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
    let source_dir = project
        .join("vendor")
        .join(namespace)
        .join(name.split_once('/').expect("checked").1)
        .join(version);
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
    let built = driver_opts(
        Mode::EmitObject,
        &entry,
        Some(&object),
        &DriverOptions::default(),
    );
    let _ = std::fs::remove_file(&entry);
    built.map_err(|e| Error::plain(format!("build package object: {e}")))?;
    let interfaces = source_dir.join("build");
    std::fs::create_dir_all(&interfaces)
        .map_err(|e| Error::plain(format!("{}: {e}", interfaces.display())))?;
    let project_build = project.join("build");
    for module in module_names
        .iter()
        .map(String::as_str)
        .chain(std::iter::once("CinchPackageRoot"))
    {
        let from = project_build.join(format!("{module}.cinchif"));
        let to = interfaces.join(format!("{module}.cinchif"));
        if from.exists() {
            std::fs::copy(&from, &to)
                .map_err(|e| Error::plain(format!("{}: {e}", to.display())))?;
        }
    }
    if !object.exists() || !abi.exists() || !interfaces.exists() {
        return Err(Error::plain(
            "package object emission did not produce its artifacts",
        ));
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
        &object
            .strip_prefix(project)
            .unwrap_or(&object)
            .display()
            .to_string(),
        &interfaces
            .strip_prefix(project)
            .unwrap_or(&interfaces)
            .display()
            .to_string(),
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
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    };
    if !valid(namespace) || !valid(package) {
        return Err(Error::plain(
            "package names use lowercase letters, digits, `-`, and `_`",
        ));
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
            _ => {
                return Err(Error::plain(
                    "package file path may not contain `..` or a root",
                ))
            }
        }
    }
    Ok(out)
}

fn canonical(value: &Value) -> Result<String> {
    let text =
        serde_json::to_string(value).map_err(|e| Error::plain(format!("canonical JSON: {e}")))?;
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
        let bytes =
            std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
        serde_json::from_slice(&bytes)
            .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?
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

fn curl(url: &str, token: Option<&str>) -> Result<Vec<u8>> {
    let mut command = Command::new("curl");
    command.args(["-fsSL", "--max-time", "60"]);
    if let Some(token) = token {
        command
            .arg("-H")
            .arg(format!("authorization: Bearer {token}"));
    }
    let output = command
        .arg(url)
        .output()
        .map_err(|e| Error::plain(format!("run curl: {e}")))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(Error::plain(format!("GET {url}: {}", detail.trim())));
    }
    Ok(output.stdout)
}

fn curl_post(url: &str, token: &str, body: &[u8]) -> Result<Vec<u8>> {
    let mut child = Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "60",
            "-X",
            "POST",
            "-H",
            "content-type: application/json",
            "-H",
            &format!("authorization: Bearer {token}"),
            "--data-binary",
            "@-",
        ])
        .arg(url)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::plain(format!("run curl: {e}")))?;
    child
        .stdin
        .take()
        .ok_or_else(|| Error::plain("curl stdin is unavailable"))?
        .write_all(body)
        .map_err(|e| Error::plain(format!("write curl request: {e}")))?;
    let output = child
        .wait_with_output()
        .map_err(|e| Error::plain(format!("wait for curl: {e}")))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr);
        return Err(Error::plain(format!("POST {url}: {}", detail.trim())));
    }
    Ok(output.stdout)
}

fn read_manifest(project: &Path) -> Result<Value> {
    let path = project.join(".cinchpkg");
    let bytes =
        std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    serde_json::from_slice(&bytes).map_err(|e| Error::plain(format!("{}: {e}", path.display())))
}

fn manifest_files(project: &Path, manifest: &Value) -> Result<BTreeMap<String, Vec<u8>>> {
    let modules = manifest
        .get("modules")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain("manifest has no modules"))?;
    if modules.is_empty() {
        return Err(Error::plain("manifest has no modules"));
    }
    let mut files = BTreeMap::new();
    for module in modules {
        let name = module
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("module entry has no name"))?;
        let relative = module
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("module {name} has no path")))?;
        let relative = safe_relative(relative)?;
        let path = project.join(&relative);
        let bytes = std::fs::read(&path)
            .map_err(|e| Error::plain(format!("module {name}: {}: {e}", path.display())))?;
        files.insert(relative.display().to_string(), bytes);
    }
    Ok(files)
}

/// Files that belong to the package release, excluding compiler-generated
/// artifacts and the registry wrapper metadata. The registry identity covers
/// every bundle file, including files beyond the manifest's module list.
fn installed_source_files(dir: &Path) -> Result<BTreeMap<String, Vec<u8>>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) -> Result<()> {
        let entries =
            std::fs::read_dir(dir).map_err(|e| Error::plain(format!("{}: {e}", dir.display())))?;
        for entry in entries {
            let entry = entry.map_err(|e| Error::plain(format!("{}: {e}", dir.display())))?;
            let path = entry.path();
            let relative = path.strip_prefix(base).unwrap_or(&path);
            let name = relative.display().to_string();
            if relative.components().count() == 1
                && matches!(name.as_str(), ".cinchpkg" | "README.md")
            {
                continue;
            }
            let file_type = entry
                .file_type()
                .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
            if file_type.is_dir() {
                if relative.components().count() == 1 && name == "build" {
                    continue;
                }
                walk(base, &path, files)?;
            } else if file_type.is_file() {
                if matches!(
                    path.extension().and_then(|ext| ext.to_str()),
                    Some("o" | "ll")
                ) || name.ends_with(".abi.json")
                {
                    continue;
                }
                let bytes = std::fs::read(&path)
                    .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
                files.insert(name, bytes);
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    walk(dir, dir, &mut files)?;
    if files.is_empty() {
        return Err(Error::plain(format!(
            "{} has no package source files",
            dir.display()
        )));
    }
    Ok(files)
}

fn package_identity(manifest: &Value, files: &BTreeMap<String, Vec<u8>>) -> Result<String> {
    let manifest = canonical(manifest)?;
    let mut rows = Vec::new();
    for (path, bytes) in files {
        rows.push(format!("{path}\t{}", sha256_hex(bytes)));
    }
    let text = format!("package\n{manifest}\n{}", rows.join("\n"));
    Ok(format!("sha256:{}", sha256_hex(text.as_bytes())))
}

/// Publish the project's `.cinchpkg` and its module files to a registry.
fn publish(project: &Path, registry: &str, token: &str, visibility: &str) -> Result<Published> {
    if visibility != "public" && visibility != "private" {
        return Err(Error::plain("visibility must be public or private"));
    }
    let manifest = read_manifest(project)?;
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no name"))?;
    validate_name(name)?;
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no version"))?;
    semver_key(version)?;
    let files = manifest_files(project, &manifest)?;
    let package_id = package_identity(&manifest, &files)?;
    let encoded: BTreeMap<String, String> = files
        .iter()
        .map(|(path, bytes)| (path.clone(), base64_encode(bytes)))
        .collect();
    let readme = std::fs::read_to_string(project.join("README.md")).ok();
    let body = json!({
        "manifest": manifest,
        "files": encoded,
        "visibility": visibility,
        "readme": readme,
        "packageId": package_id,
    });
    let body =
        serde_json::to_vec(&body).map_err(|e| Error::plain(format!("publish request: {e}")))?;
    let url = format!("{registry}/api/v1/packages");
    let response = curl_post(&url, token, &body)?;
    let response: Value = serde_json::from_slice(&response)
        .map_err(|e| Error::plain(format!("registry publish response: {e}")))?;
    let returned = response
        .pointer("/entry/packageId")
        .and_then(Value::as_str)
        .unwrap_or(&package_id);
    if returned != package_id {
        return Err(Error::plain(format!(
            "registry stored {returned}, local identity is {package_id}"
        )));
    }
    Ok(Published {
        name: name.to_string(),
        version: version.to_string(),
        package_id,
    })
}

/// Verify every release recorded in `.cinchlock`: package identity, object
/// hash, and interface files. This is a local check; it does not contact the
/// registry and does not claim provenance.
fn verify(project: &Path) -> Result<Verified> {
    let path = project.join(".cinchlock");
    let bytes =
        std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let lock: Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let packages = lock
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(".cinchlock has no packages array"))?;
    for row in packages {
        let name = row
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("lock entry has no name"))?;
        let version = row
            .get("version")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{name} lock entry has no version")))?;
        let package_id = row
            .get("package_id")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{name} lock entry has no package_id")))?;
        let source_dir = row
            .get("source_dir")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{name} lock entry has no source_dir")))?;
        let source_dir = project.join(source_dir);
        let manifest = read_manifest(&source_dir)?;
        if manifest.get("name").and_then(Value::as_str) != Some(name)
            || manifest.get("version").and_then(Value::as_str) != Some(version)
        {
            return Err(Error::plain(format!(
                "{name}@{version}: .cinchpkg identity does not match the lock entry"
            )));
        }
        let files = installed_source_files(&source_dir)?;
        let actual_id = package_identity(&manifest, &files)?;
        if actual_id != package_id {
            return Err(Error::plain(format!(
                "{name}@{version}: package identity mismatch: lock says {package_id}, files are {actual_id}"
            )));
        }
        let object = project.join(
            row.get("object")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain(format!("{name} lock entry has no object")))?,
        );
        let abi_path = object.with_extension("abi.json");
        let object_bytes = std::fs::read(&object)
            .map_err(|e| Error::plain(format!("{}: {e}", object.display())))?;
        let abi: Value = serde_json::from_slice(
            &std::fs::read(&abi_path)
                .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?,
        )
        .map_err(|e| Error::plain(format!("{}: {e}", abi_path.display())))?;
        let expected = abi
            .get("object_content_hash")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                Error::plain(format!("{} has no object_content_hash", abi_path.display()))
            })?;
        let actual = crate::interface::content_hash_bytes(&object_bytes);
        if actual != expected {
            return Err(Error::plain(format!(
                "{}: object hash mismatch: manifest says {expected}, object is {actual}",
                object.display()
            )));
        }
        let interfaces = project.join(
            row.get("interfaces")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain(format!("{name} lock entry has no interfaces")))?,
        );
        for module in manifest
            .get("modules")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let module_name = module
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain("module entry has no name"))?;
            let iface_path = interfaces.join(format!("{module_name}.cinchif"));
            let iface = std::fs::read_to_string(&iface_path)
                .map_err(|e| Error::plain(format!("{}: {e}", iface_path.display())))?;
            let iface = crate::interface::Iface::from_json(&iface)
                .map_err(|e| Error::plain(format!("{}: {e}", iface_path.display())))?;
            if iface.module != module_name {
                return Err(Error::plain(format!(
                    "{} names module {}, expected {module_name}",
                    iface_path.display(),
                    iface.module
                )));
            }
        }
    }
    Ok(Verified {
        packages: packages.len(),
    })
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

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
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

    #[test]
    fn semver_ranges_intersect_and_reject_incompatible_versions() {
        assert!(matches_range("0.1.0", "^0.1.0"));
        assert!(matches_range("0.1.9", "^0.1.0"));
        assert!(!matches_range("0.9.4", "^0.1.0"));
        assert!(!matches_range("1.0.0", "^0.1.0"));
        assert!(matches_range("2.3.1", "^2.0.0"));
        assert!(!matches_range("3.0.0", "^2.0.0"));
        assert!(matches_range("1.2.9", "~1.2.3"));
        assert!(!matches_range("1.3.0", "~1.2.3"));
        assert!(matches_range("1.5.0", ">=1.2.0, <2.0.0"));
        assert!(!matches_range("2.0.0", ">=1.2.0, <2.0.0"));
        assert!(matches_range("1.2.7", "1.2.x"));
        assert!(!matches_range("1.3.0", "1.2.x"));
        assert!(matches_range("0.1.0", "=0.1.0"));
        assert!(!matches_range("0.1.1", "=0.1.0"));
        let range = Requirement::from_dependency(&json!({
            "range": ">=0.1.0, <0.2.0"
        }))
        .expect("range requirement");
        assert!(range.matches("0.1.5"));
        assert!(!range.matches("0.2.0"));
        let exact =
            Requirement::from_dependency(&json!({"version": "0.1.0"})).expect("exact requirement");
        assert!(exact.matches("0.1.0"));
        assert!(!exact.matches("0.1.1"));
    }

    #[test]
    fn bundle_materializes_sources_and_an_object() {
        let dir = std::env::temp_dir().join(format!("cinch-pkg-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("project");
        let source = b"module Library where\n  value :: Int\n  value = 42\n";
        let notes = b"release notes\n";
        let files = json!({
            "Library.cinch": base64_encode(source),
            "notes.txt": base64_encode(notes)
        });
        let manifest = json!({
            "schema": 2,
            "name": "acme/library",
            "version": "0.1.0",
            "modules": [{"name": "Library", "path": "Library.cinch"}],
            "order": ["Library"]
        });
        let mut decoded = BTreeMap::new();
        decoded.insert("Library.cinch".to_string(), source.to_vec());
        decoded.insert("notes.txt".to_string(), notes.to_vec());
        let package_id = format!("sha256:{}", {
            let text = format!(
                "package\n{}\nLibrary.cinch\t{}\nnotes.txt\t{}",
                canonical(&manifest).unwrap(),
                sha256_hex(source),
                sha256_hex(notes)
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
        driver_opts(
            Mode::Check,
            &dir.join("Main.cinch"),
            None,
            &DriverOptions::default(),
        )
        .expect("project imports the installed package");
        assert_eq!(verify(&dir).expect("verify lock").packages, 1);
        std::fs::write(
            installed.source_dir.join("Library.cinch"),
            b"module Library where\n",
        )
        .expect("tamper source");
        assert!(
            verify(&dir).is_err(),
            "tampered source must fail verification"
        );
        std::fs::remove_dir_all(&dir).ok();
    }
}
