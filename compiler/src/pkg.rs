//! Install a package into a project and turn it into checked interfaces and a
//! native library object.
//!
//! Sources are registry releases over the v0 HTTP service in `registry/`,
//! local `path` directories, and `git` checkouts (`<path>`, `file://`, and
//! https). Registry reads accept a bearer token for private packages; `pkg
//! publish` uses the same token. Resolution supports exact versions and common
//! semver ranges, with conflict checks when several dependencies constrain one
//! package name.
//!
//! Path and git identity is the content root over the canonical manifest and
//! the sorted source hashes, never the directory or the revision. Every
//! installed release is placed under `vendor/`, which the source loader indexes
//! recursively.

use crate::driver::{driver_opts, package_execution_path, read_package_execution, DriverOptions, Mode};
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
  cinch pkg add PATH_DIR | --path DIR [--project DIR]
  cinch pkg add URL --git-rev REV | --git URL --git-rev REV [--project DIR]
  cinch pkg install [--project DIR] [--registry URL] [--token TOKEN] [--locked]
  cinch pkg publish [--registry URL] [--project DIR] [--token TOKEN] [--visibility public|private]
  cinch pkg verify [--slsa] [--project DIR]
  cinch pkg info NAME [--project DIR] [--target TRIPLE] [--target-version VERSION]";

/// What one install produced.
#[derive(Debug)]
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
    pub provenance_levels: Vec<String>,
}

enum Output {
    Installed(Installed),
    InstalledList(Vec<Installed>),
    Published(Published),
    Verified(Verified),
    Info(String),
}

/// `cinch pkg ...`.
pub fn cli(args: &[String]) -> i32 {
    match run(args) {
        Ok(Output::Installed(installed)) => {
            report(&installed);
            0
        }
        Ok(Output::InstalledList(installed)) => {
            for package in &installed {
                report(package);
            }
            0
        }
        Ok(Output::Info(text)) => {
            print!("{text}");
            0
        }
        Ok(Output::Published(published)) => {
            println!("published {} {}", published.name, published.version);
            println!("  package: {}", published.package_id);
            0
        }
        Ok(Output::Verified(verified)) => {
            println!("verified {} package(s)", verified.packages);
            for level in &verified.provenance_levels {
                println!("  provenance: {level}");
            }
            0
        }
        Err(error) => {
            eprintln!("cinch pkg: {error}");
            2
        }
    }
}

fn report(installed: &Installed) {
    println!("installed {} {}", installed.name, installed.version);
    if !installed.package_id.is_empty() {
        println!("  identity: {}", installed.package_id);
    }
    println!("  source: {}", installed.source_dir.display());
    println!("  object: {}", installed.object.display());
    println!("  execution interface: {}", installed.abi.display());
    println!("  interfaces: {}", installed.interfaces.display());
}

/// `cinch pkg info NAME`: what one installed package resolves to for this
/// target, before anything is compiled (design/base-as-a-package.md
/// sections 4.2 and 4.3). It answers the questions a build would
/// otherwise answer silently: which modules the target selects, which of
/// them are internal, which prelude the package injects, what the link
/// plan will add, and whether raw words will need consent.
fn info(project: &Path, name: &str, facts: &crate::driver::TargetFacts) -> Result<String> {
    let (namespace, package) = name
        .split_once('/')
        .ok_or_else(|| Error::plain(format!("{name}: a package name is namespace/name")))?;
    let lock = read_lock(project)?;
    let row = lock
        .get(name)
        .ok_or_else(|| Error::plain(format!("{name} is not installed in this project")))?;
    let version = row
        .get("version")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| Error::plain(format!("{name}: the lock entry has no version")))?;
    let source_dir = project.join("vendor").join(namespace).join(package).join(version);
    let manifest_path = source_dir.join(".cinchpkg");
    let text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
    let declared = crate::driver::Package::parse(&text)?;
    let resolved = declared.resolve(facts)?;

    let mut out = String::new();
    out.push_str(&format!("{} {}\n", resolved.name, resolved.version));
    out.push_str(&format!("target: {}\n", facts.triple));
    if let Some(platform) = &facts.os_version {
        out.push_str(&format!("target version: {platform}\n"));
    }
    out.push_str(&format!("schema: {}\n", declared.schema));
    match &resolved.prelude {
        crate::driver::PreludeSpec::Default => out.push_str("prelude: default\n"),
        crate::driver::PreludeSpec::None => out.push_str("prelude: none\n"),
        crate::driver::PreludeSpec::Module(m) => out.push_str(&format!("prelude: {m}\n")),
    }
    out.push_str("modules:\n");
    for m in &resolved.modules {
        let exposure = if m.exposed { "" } else { " (internal)" };
        let facade = if m.facade { " (facade)" } else { "" };
        out.push_str(&format!("  {}{}{} from {}\n", m.name, exposure, facade, m.path));
    }
    if declared.select.is_empty() {
        out.push_str("select: no conditions\n");
    } else {
        out.push_str("select:\n");
        for (i, entry) in declared.select.iter().enumerate() {
            let state = if resolved.matched.contains(&i) {
                "applies"
            } else {
                "does not apply"
            };
            let mut keys: Vec<String> = Vec::new();
            if !entry.when.os.is_empty() {
                keys.push(format!("os={}", entry.when.os.join("|")));
            }
            if !entry.when.arch.is_empty() {
                keys.push(format!("arch={}", entry.when.arch.join("|")));
            }
            if !entry.when.libc.is_empty() {
                keys.push(format!("libc={}", entry.when.libc.join("|")));
            }
            if let Some(v) = &entry.when.os_version {
                keys.push(format!("os_version {v}"));
            }
            if !entry.when.target.is_empty() {
                keys.push(format!("target={}", entry.when.target.join("|")));
            }
            if keys.is_empty() {
                keys.push("always".to_string());
            }
            out.push_str(&format!(
                "  {i}: {} ({}, {} module(s)){}\n",
                keys.join(", "),
                state,
                entry.modules.len(),
                if entry.link.has_raw() { ", raw link words" } else { "" }
            ));
        }
    }
    let words = resolved.link.words_for(crate::driver::LinkTier::Semantic, &facts.os);
    if words.is_empty() && resolved.link.pkgconfig.is_empty() {
        out.push_str("link: nothing\n");
    } else {
        out.push_str(&format!("link: {}\n", words.join(" ")));
        for pc in &resolved.link.pkgconfig {
            let min = pc.min.clone().unwrap_or_else(|| "any".to_string());
            out.push_str(&format!("  pkgconfig {} (min {min})\n", pc.name));
        }
        if !resolved.link.pkgconfig.is_empty() {
            match row.get("native").and_then(|n| n.get("words")) {
                Some(serde_json::Value::Array(words)) => {
                    let recorded: Vec<String> = words
                        .iter()
                        .filter_map(|w| w.as_str().map(str::to_string))
                        .collect();
                    out.push_str(&format!("  recorded at install: {}\n", recorded.join(" ")));
                }
                _ => out.push_str("  recorded at install: nothing yet, run `cinch pkg install`\n"),
            }
        }
    }
    if resolved.link.has_raw() {
        out.push_str(&format!(
            "raw link words: {} (the consuming project must consent)\n",
            resolved.link.raw_ldflags.join(" ")
        ));
    }
    Ok(out)
}

fn run(args: &[String]) -> Result<Output> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(Error::plain(USAGE));
    };
    match command {
        "add" | "install" => {
            let (flags, positional) = Flags::parse(&args[1..], false)?;
            let request = source_request(command, &flags, positional)?;
            match request {
                SourceRequest::Registry(spec) => {
                    let installed = install_with_token(
                        &flags.project,
                        &flags.registry,
                        &spec,
                        flags.token.as_deref(),
                    )?;
                    Ok(Output::Installed(installed))
                }
                SourceRequest::Local(source) => {
                    let mut context = InstallContext::new(
                        &flags.project,
                        &flags.registry,
                        flags.token.as_deref(),
                        flags.locked,
                    );
                    let installed = context.install_sources(vec![source.resolve()?])?;
                    Ok(Output::InstalledList(installed))
                }
                SourceRequest::Project => {
                    let mut context = InstallContext::new(
                        &flags.project,
                        &flags.registry,
                        flags.token.as_deref(),
                        flags.locked,
                    );
                    let installed = context.install_project_manifest()?;
                    Ok(Output::InstalledList(installed))
                }
            }
        }
        "publish" => {
            let (flags, _) = Flags::parse(&args[1..], true)?;
            let token = flags
                .token
                .as_deref()
                .ok_or_else(|| Error::plain("publish needs --token or CINCH_REGISTRY_TOKEN"))?;
            let visibility = flags.visibility.as_deref().unwrap_or("public");
            publish(&flags.project, &flags.registry, token, visibility).map(Output::Published)
        }
        "info" => {
            let (flags, positional) = Flags::parse(&args[1..], false)?;
            let name = positional
                .ok_or_else(|| Error::plain("cinch pkg info needs a package name"))?;
            let facts = flags.facts()?;
            info(&flags.project, &name, &facts).map(Output::Info)
        }
        "verify" => {
            let (flags, _) = Flags::parse(&args[1..], false)?;
            if flags.slsa {
                verify_slsa(&flags.project).map(Output::Verified)
            } else {
                verify(&flags.project).map(Output::Verified)
            }
        }
        _ => Err(Error::plain(USAGE)),
    }
}

/// A package source named on the command line.
enum PackageSource {
    Path { path: String },
    Git { url: String, rev: String },
}

impl PackageSource {
    fn resolve(&self) -> Result<LocalSource> {
        match self {
            PackageSource::Path { path } => {
                let dir = std::fs::canonicalize(path)
                    .map_err(|e| Error::plain(format!("path package {path}: {e}")))?;
                if !dir.join(".cinchpkg").is_file() {
                    return Err(Error::plain(format!(
                        "path package {path} has no .cinchpkg manifest"
                    )));
                }
                Ok(LocalSource {
                    kind: LocalKind::Path,
                    original: path.clone(),
                    rev: None,
                    dir,
                    marker: None,
                })
            }
            PackageSource::Git { url, rev } => {
                let checkout = git_checkout(url, rev)?;
                Ok(LocalSource {
                    kind: LocalKind::Git,
                    original: url.clone(),
                    rev: Some(rev.clone()),
                    dir: checkout.dir,
                    marker: Some(checkout.marker),
                })
            }
        }
    }
}

/// What `pkg add` and `pkg install` were asked to install.
enum SourceRequest {
    Registry(String),
    Local(PackageSource),
    Project,
}

fn source_request(
    command: &str,
    flags: &Flags,
    positional: Option<String>,
) -> Result<SourceRequest> {
    if flags.path.is_some() && flags.git.is_some() {
        return Err(Error::plain("--path and --git are mutually exclusive"));
    }
    if flags.git_rev.is_some() && flags.path.is_some() {
        return Err(Error::plain(
            "--git-rev belongs to a git source, not a path",
        ));
    }
    if let Some(path) = &flags.path {
        if positional.is_some() {
            return Err(Error::plain(
                "give the package directory positionally or with --path, not both",
            ));
        }
        return Ok(SourceRequest::Local(PackageSource::Path {
            path: path.clone(),
        }));
    }
    if let Some(url) = &flags.git {
        let rev = flags
            .git_rev
            .clone()
            .ok_or_else(|| Error::plain("--git needs --git-rev REV"))?;
        if positional.is_some() {
            return Err(Error::plain(
                "give the git URL positionally or with --git, not both",
            ));
        }
        return Ok(SourceRequest::Local(PackageSource::Git {
            url: url.clone(),
            rev,
        }));
    }
    let Some(spec) = positional else {
        if command == "install" {
            return Ok(SourceRequest::Project);
        }
        return Err(Error::plain(
            "pkg add needs NAME@VERSION, a package directory, or a git URL with --git-rev REV",
        ));
    };
    if let Some(rev) = &flags.git_rev {
        return Ok(SourceRequest::Local(PackageSource::Git {
            url: spec,
            rev: rev.clone(),
        }));
    }
    if spec.contains('@') {
        return Ok(SourceRequest::Registry(spec));
    }
    if Path::new(&spec).is_dir() {
        return Ok(SourceRequest::Local(PackageSource::Path { path: spec }));
    }
    if spec.contains("://") || spec.ends_with(".git") {
        return Err(Error::plain(format!(
            "git package {spec} needs --git-rev REV"
        )));
    }
    Err(Error::plain(
        "pkg add needs NAME@VERSION, a package directory, or a git URL with --git-rev REV",
    ))
}

struct Flags {
    registry: String,
    project: PathBuf,
    token: Option<String>,
    visibility: Option<String>,
    path: Option<String>,
    git: Option<String>,
    git_rev: Option<String>,
    locked: bool,
    slsa: bool,
    /// `--target <triple>` and `--target-version <version>`: the facts a
    /// package's `select` conditions resolve against (design section
    /// 4.1). A build uses the same two flags.
    target: Option<String>,
    target_version: Option<String>,
}

impl Flags {
    /// The target facts `select` conditions resolve against, from the
    /// same two flags a build uses. An unusable triple or an empty
    /// version is refused here rather than inside resolution.
    fn facts(&self) -> Result<crate::driver::TargetFacts> {
        let profile = crate::driver::target_profile(&crate::driver::DriverOptions {
            target: self.target.clone(),
            target_version: self.target_version.clone(),
            ..crate::driver::DriverOptions::default()
        })?;
        Ok(crate::driver::TargetFacts::of(&profile))
    }

    fn parse(args: &[String], allow_visibility: bool) -> Result<(Flags, Option<String>)> {
        let mut flags = Flags {
            registry: std::env::var("CINCH_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY.into()),
            project: std::env::current_dir()
                .map_err(|e| Error::plain(format!("current directory: {e}")))?,
            token: std::env::var("CINCH_REGISTRY_TOKEN")
                .ok()
                .filter(|t| !t.is_empty()),
            visibility: None,
            path: None,
            git: None,
            git_rev: None,
            locked: false,
            slsa: false,
            target: None,
            target_version: None,
        };
        let mut positional: Option<String> = None;
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
                "--path" => {
                    i += 1;
                    flags.path = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--path needs a directory"))?,
                    );
                }
                "--git" => {
                    i += 1;
                    flags.git = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--git needs a URL"))?,
                    );
                }
                "--git-rev" => {
                    i += 1;
                    flags.git_rev = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--git-rev needs a revision"))?,
                    );
                }
                "--locked" => flags.locked = true,
                "--target" => {
                    i += 1;
                    flags.target = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--target needs a triple"))?,
                    );
                }
                "--target-version" => {
                    i += 1;
                    flags.target_version = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--target-version needs a version"))?,
                    );
                }
                "--slsa" => flags.slsa = true,
                "--visibility" if allow_visibility => {
                    i += 1;
                    flags.visibility = Some(
                        args.get(i)
                            .cloned()
                            .ok_or_else(|| Error::plain("--visibility needs public or private"))?,
                    );
                }
                other if other.starts_with('-') => {
                    return Err(Error::plain(format!("unknown pkg flag: {other}")))
                }
                other => {
                    if positional.is_some() {
                        return Err(Error::plain(format!(
                            "unexpected extra package argument: {other}"
                        )));
                    }
                    positional = Some(other.to_string());
                }
            }
            i += 1;
        }
        flags.registry = flags.registry.trim_end_matches('/').to_string();
        Ok((flags, positional))
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
    let mut context = InstallContext::new(project, registry, token, false);
    context.install_registry(name, version, &resolved)
}

/// A local package source: a directory or a git checkout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LocalKind {
    Path,
    Git,
}

impl LocalKind {
    fn as_str(self) -> &'static str {
        match self {
            LocalKind::Path => "path",
            LocalKind::Git => "git",
        }
    }
}

/// Where a local package is read from. `original` keeps the authored spelling
/// (a relative path or a git URL) so the lock records what the manifest says,
/// not the absolute directory this process happened to resolve.
#[derive(Clone, Debug)]
struct LocalSource {
    kind: LocalKind,
    original: String,
    rev: Option<String>,
    dir: PathBuf,
    marker: Option<PathBuf>,
}

impl LocalSource {
    fn label(&self) -> String {
        match self.kind {
            LocalKind::Path => format!("path {}", self.original),
            LocalKind::Git => format!(
                "git {} at {}",
                self.original,
                self.rev.as_deref().unwrap_or("?")
            ),
        }
    }

    fn lock_source(&self) -> Value {
        match self.kind {
            LocalKind::Path => json!({"kind": "path", "path": self.original}),
            LocalKind::Git => json!({
                "kind": "git",
                "url": self.original,
                "rev": self.rev.clone().unwrap_or_default()
            }),
        }
    }
}

/// One local package read and identified before any materialization.
struct LocalPackage {
    name: String,
    version: String,
    identity: String,
    manifest: Value,
    files: BTreeMap<String, Vec<u8>>,
    source: LocalSource,
}

struct LocalPlan {
    packages: Vec<LocalPackage>,
    seeds: BTreeMap<String, Vec<Requirement>>,
}

/// Install state for one command: the project, the registry, and the names
/// already claimed by this closure.
struct InstallContext<'a> {
    project: &'a Path,
    registry: &'a str,
    token: Option<&'a str>,
    locked: bool,
    /// Module name to the package that declares it.
    module_owner: BTreeMap<String, String>,
    /// Registry name to the version selected in this closure.
    seen: BTreeMap<String, String>,
}

impl<'a> InstallContext<'a> {
    fn new(
        project: &'a Path,
        registry: &'a str,
        token: Option<&'a str>,
        locked: bool,
    ) -> InstallContext<'a> {
        InstallContext {
            project,
            registry: registry.trim_end_matches('/'),
            token,
            locked,
            module_owner: BTreeMap::new(),
            seen: BTreeMap::new(),
        }
    }

    fn install_sources(&mut self, roots: Vec<LocalSource>) -> Result<Vec<Installed>> {
        if let Some(root) = roots.first() {
            if same_directory(&root.dir, self.project) {
                return Err(Error::plain(format!(
                    "{} names the project directory; a project is not its own dependency",
                    root.label()
                )));
            }
        }
        let plan = plan_packages(&roots, None, &mut self.module_owner)?;
        self.install_plan(plan)
    }

    fn install_project_manifest(&mut self) -> Result<Vec<Installed>> {
        let manifest = read_manifest(self.project).map_err(|e| {
            Error::plain(format!(
                "{} has no project manifest: {}",
                self.project.display(),
                e.message
            ))
        })?;
        let plan = plan_packages(&[], Some((&manifest, self.project)), &mut self.module_owner)?;
        self.install_plan(plan)
    }

    fn install_plan(&mut self, plan: LocalPlan) -> Result<Vec<Installed>> {
        let resolved = if plan.seeds.is_empty() {
            BTreeMap::new()
        } else {
            resolve_constraints(self.registry, &plan.seeds, self.token)?
        };
        for package in &plan.packages {
            if resolved.contains_key(&package.name) {
                return Err(Error::plain(format!(
                    "one version per package name: {} is a local {} dependency and also a registry release in this closure",
                    package.name,
                    package.source.kind.as_str()
                )));
            }
        }
        if self.locked {
            self.check_lock(&plan, &resolved)?;
        }
        let mut installed = Vec::new();
        for (name, version) in &resolved {
            installed.push(self.install_registry(name, version, &resolved)?);
        }
        for package in &plan.packages {
            installed.push(self.materialize_local(package)?);
        }
        Ok(installed)
    }

    /// Install one registry release and its registry dependencies.
    fn install_registry(
        &mut self,
        name: &str,
        version: &str,
        resolved: &BTreeMap<String, String>,
    ) -> Result<Installed> {
        if let Some(previous) = self.seen.get(name) {
            if previous == version {
                return existing_install(self.project, name, version);
            }
            return Err(Error::plain(format!(
                "one version per package name: {name} resolves to {previous} and {version}"
            )));
        }
        self.seen.insert(name.to_string(), version.to_string());
        let metadata = fetch_metadata(self.registry, name, version, self.token)?;
        if let Some(manifest) = metadata.get("manifest") {
            self.claim_modules(name, manifest)?;
            if let Some(dependencies) = manifest.get("dependencies").and_then(Value::as_array) {
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
                        return Err(local_source_from_registry_error(name, dep_name, source));
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
                    if self.seen.get(dep_name) == Some(&dep_version) {
                        continue;
                    }
                    self.install_registry(dep_name, &dep_version, resolved)?;
                }
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
        let download_url = format!("{}/api/v1/download/{hex}", self.registry);
        let bundle: Value = serde_json::from_slice(&curl(&download_url, self.token)?)
            .map_err(|e| Error::plain(format!("registry bundle: {e}")))?;
        install_bundle(
            self.project,
            self.registry,
            &package_id,
            &bundle,
            metadata.get("files"),
            metadata.get("attestations"),
        )
    }

    fn claim_modules(&mut self, package: &str, manifest: &Value) -> Result<()> {
        for module in manifest_modules(manifest, package)? {
            if let Some(owner) = self.module_owner.get(&module) {
                if owner != package {
                    return Err(Error::plain(format!(
                        "module {module} is declared by both {owner} and {package}; a build selects one package per module name"
                    )));
                }
            }
            self.module_owner.insert(module, package.to_string());
        }
        Ok(())
    }

    fn materialize_local(&mut self, package: &LocalPackage) -> Result<Installed> {
        let source = package.source.lock_source();
        if let Some(installed) =
            existing_materialization(self.project, &package.name, &package.identity)
        {
            update_lock(
                self.project,
                &package.name,
                &package.version,
                &package.identity,
                &source,
                &installed.source_dir,
                &installed.object,
                &installed.interfaces,
            )?;
            record_provenance(
                self.project,
                &package.name,
                &package.version,
                &package.identity,
                &installed.source_dir,
            )?;
            return Ok(installed);
        }
        let readme = std::fs::read_to_string(package.source.dir.join("README.md")).ok();
        materialize(
            self.project,
            &package.manifest,
            &package.files,
            &package.identity,
            source,
            readme.as_deref(),
        )
    }

    /// `--locked` refuses to rewrite a lock. A path dependency whose bytes
    /// changed is the dirty case the design names.
    fn check_lock(&self, plan: &LocalPlan, resolved: &BTreeMap<String, String>) -> Result<()> {
        let lock = read_lock(self.project)?;
        for package in &plan.packages {
            match lock.get(&package.name) {
                Some(row)
                    if row.get("package_id").and_then(Value::as_str)
                        == Some(package.identity.as_str()) => {}
                Some(row) => {
                    let previous = row
                        .get("package_id")
                        .and_then(Value::as_str)
                        .unwrap_or("<missing>");
                    return Err(Error::plain(format!(
                        "--locked: {} dependency is dirty: .cinchlock records {previous} for {} {}, but {} now has content root {}; run `cinch pkg install` without --locked to update the lock",
                        package.source.kind.as_str(),
                        package.name,
                        package.version,
                        package.source.label(),
                        package.identity
                    )));
                }
                None => {
                    return Err(Error::plain(format!(
                        "--locked: {} is not in .cinchlock; run `cinch pkg install` without --locked first",
                        package.name
                    )))
                }
            }
        }
        for (name, version) in resolved {
            match lock.get(name) {
                Some(row)
                    if row.get("version").and_then(Value::as_str) == Some(version.as_str()) => {}
                Some(row) => {
                    let previous = row
                        .get("version")
                        .and_then(Value::as_str)
                        .unwrap_or("<missing>");
                    return Err(Error::plain(format!(
                        "--locked: registry dependency {name} would change from {previous} to {version}; run `cinch pkg install` without --locked to update the lock"
                    )));
                }
                None => {
                    return Err(Error::plain(format!(
                        "--locked: registry dependency {name}@{version} is not in .cinchlock; run `cinch pkg install` without --locked first"
                    )))
                }
            }
        }
        Ok(())
    }
}

fn local_source_from_registry_error(package: &str, dependency: &str, source: &str) -> Error {
    Error::plain(format!(
        "registry package {package} declares a {source} dependency on {dependency}. A registry release cannot carry a usable local source; add {dependency} to the project manifest as a {source} dependency (or install it with `cinch pkg add --path` or `cinch pkg add --git URL --git-rev REV`) and run `cinch pkg install` from the project root"
    ))
}

fn same_directory(left: &Path, right: &Path) -> bool {
    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}

fn read_lock(project: &Path) -> Result<BTreeMap<String, Value>> {
    let path = project.join(".cinchlock");
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let bytes =
        std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let lock: Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let packages = lock
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(".cinchlock has no packages array"))?;
    let mut entries = BTreeMap::new();
    for row in packages {
        let name = row
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("lock entry has no name"))?;
        entries.insert(name.to_string(), row.clone());
    }
    Ok(entries)
}

/// The artifacts of a previous install when the lock already records this
/// content root, so an unchanged path or git dependency is not rebuilt.
fn existing_materialization(project: &Path, name: &str, identity: &str) -> Option<Installed> {
    let lock = read_lock(project).ok()?;
    let row = lock.get(name)?;
    if row.get("package_id").and_then(Value::as_str) != Some(identity) {
        return None;
    }
    let version = row.get("version").and_then(Value::as_str)?;
    let (namespace, package) = name.split_once('/')?;
    let source_dir = project
        .join("vendor")
        .join(namespace)
        .join(package)
        .join(version);
    let object = source_dir.join("package.o");
    let abi = package_execution_path(&object);
    let interfaces = source_dir.join("build");
    if !object.is_file() || !interfaces.is_dir() || read_package_execution(&object).is_err() {
        return None;
    }
    Some(Installed {
        name: name.to_string(),
        version: version.to_string(),
        package_id: identity.to_string(),
        source_dir,
        object,
        abi,
        interfaces,
    })
}

fn manifest_modules(manifest: &Value, owner: &str) -> Result<Vec<String>> {
    let modules = manifest
        .get("modules")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(format!("{owner}: manifest has no modules")))?;
    if modules.is_empty() {
        return Err(Error::plain(format!("{owner}: manifest has no modules")));
    }
    let mut names = Vec::new();
    for module in modules {
        let name = module
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{owner}: a module entry has no name")))?;
        names.push(name.to_string());
    }
    Ok(names)
}

/// One dependency entry of a manifest.
enum DependencySource {
    Registry(Requirement),
    Path(String),
    Git { url: String, rev: String },
}

struct Dependency {
    name: String,
    source: DependencySource,
    /// The version or range the depending manifest suggests. It never
    /// participates in identity.
    hint: Option<Requirement>,
}

fn parse_dependencies(manifest: &Value, owner: &str) -> Result<Vec<Dependency>> {
    let mut out = Vec::new();
    let Some(rows) = manifest.get("dependencies") else {
        return Ok(out);
    };
    let rows = rows
        .as_array()
        .ok_or_else(|| Error::plain(format!("{owner}: dependencies must be an array")))?;
    for row in rows {
        let name = row
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{owner}: a dependency has no name")))?
            .to_string();
        if name == "cinch/base" || name == "cinch-base" {
            continue;
        }
        validate_name(&name).map_err(|e| {
            Error::plain(format!("{owner} declares dependency {name}: {}", e.message))
        })?;
        let kind = row
            .get("source")
            .and_then(Value::as_str)
            .unwrap_or("registry");
        let source = match kind {
            "registry" => DependencySource::Registry(Requirement::from_dependency(row)?),
            "path" => {
                let path = row.get("path").and_then(Value::as_str).ok_or_else(|| {
                    Error::plain(format!(
                        "{owner} dependency {name}: a path source needs a path"
                    ))
                })?;
                if path.is_empty() {
                    return Err(Error::plain(format!(
                        "{owner} dependency {name}: the path is empty"
                    )));
                }
                DependencySource::Path(path.to_string())
            }
            "git" => {
                let url = row.get("url").and_then(Value::as_str).ok_or_else(|| {
                    Error::plain(format!(
                        "{owner} dependency {name}: a git source needs a url"
                    ))
                })?;
                let rev = row.get("rev").and_then(Value::as_str).ok_or_else(|| {
                    Error::plain(format!(
                        "{owner} dependency {name}: a git source needs a rev"
                    ))
                })?;
                if url.is_empty() || rev.is_empty() {
                    return Err(Error::plain(format!(
                        "{owner} dependency {name}: a git source needs a url and a rev"
                    )));
                }
                DependencySource::Git {
                    url: url.to_string(),
                    rev: rev.to_string(),
                }
            }
            other => {
                return Err(Error::plain(format!(
                "{owner} dependency {name}: unsupported source {other}; use registry, path, or git"
            )))
            }
        };
        let hint = if matches!(source, DependencySource::Registry(_)) {
            None
        } else if row.get("version").is_some() || row.get("range").is_some() {
            Some(Requirement::from_dependency(row)?)
        } else {
            None
        };
        out.push(Dependency { name, source, hint });
    }
    Ok(out)
}

fn resolve_dependency_dir(base: &Path, path: &str) -> PathBuf {
    let joined = if Path::new(path).is_absolute() {
        PathBuf::from(path)
    } else {
        base.join(path)
    };
    std::fs::canonicalize(&joined).unwrap_or(joined)
}

struct LocalPlanner<'a> {
    packages: Vec<LocalPackage>,
    by_name: BTreeMap<String, usize>,
    stack: Vec<String>,
    seeds: BTreeMap<String, Vec<Requirement>>,
    module_owner: &'a mut BTreeMap<String, String>,
}

fn plan_packages(
    roots: &[LocalSource],
    manifest_root: Option<(&Value, &Path)>,
    module_owner: &mut BTreeMap<String, String>,
) -> Result<LocalPlan> {
    let mut planner = LocalPlanner {
        packages: Vec::new(),
        by_name: BTreeMap::new(),
        stack: Vec::new(),
        seeds: BTreeMap::new(),
        module_owner,
    };
    for root in roots {
        planner.visit(root)?;
    }
    if let Some((manifest, dir)) = manifest_root {
        planner.visit_dependencies(manifest, dir, "the project manifest")?;
    }
    Ok(LocalPlan {
        packages: planner.packages,
        seeds: planner.seeds,
    })
}

impl LocalPlanner<'_> {
    /// Read one local package, verify its identity, and plan its dependencies.
    /// Returns the index of the package in the plan (an already planned
    /// identical package keeps its earlier index).
    fn visit(&mut self, source: &LocalSource) -> Result<usize> {
        let manifest = read_manifest(&source.dir)
            .map_err(|e| Error::plain(format!("{}: {}", source.label(), e.message)))?;
        let name = manifest
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{}: .cinchpkg has no name", source.label())))?
            .to_string();
        validate_name(&name)
            .map_err(|e| Error::plain(format!("{}: {}", source.label(), e.message)))?;
        let version = manifest
            .get("version")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{}: .cinchpkg has no version", source.label())))?
            .to_string();
        semver_key(&version)
            .map_err(|e| Error::plain(format!("{}: {}", source.label(), e.message)))?;
        let files = manifest_files(&source.dir, &manifest)
            .map_err(|e| Error::plain(format!("{}: {}", source.label(), e.message)))?;
        let identity = package_identity(&manifest, &files)?;
        if let Some(marker) = &source.marker {
            match std::fs::read_to_string(marker) {
                Ok(previous) if previous.trim() == identity => {}
                Ok(previous) => {
                    return Err(Error::plain(format!(
                        "{}: the cached checkout {} has content root {}, but the root recorded when it was fetched is {}; remove the checkout and retry",
                        source.label(),
                        source.dir.display(),
                        identity,
                        previous.trim()
                    )))
                }
                Err(_) => {
                    if let Some(parent) = marker.parent() {
                        std::fs::create_dir_all(parent).map_err(|e| {
                            Error::plain(format!("{}: {e}", parent.display()))
                        })?;
                    }
                    std::fs::write(marker, &identity)
                        .map_err(|e| Error::plain(format!("{}: {e}", marker.display())))?;
                }
            }
        }
        if let Some(index) = self.by_name.get(&name).copied() {
            if self.packages[index].identity == identity {
                return Ok(index);
            }
            return Err(Error::plain(format!(
                "one version per package name: {name} resolves to two different contents ({} and {}); keep one release of a package name in a closure",
                self.packages[index].source.label(),
                source.label()
            )));
        }
        if let Some(position) = self.stack.iter().position(|planned| planned == &name) {
            let mut chain: Vec<String> = self.stack[position..].to_vec();
            chain.push(name.clone());
            return Err(Error::plain(format!(
                "dependency cycle: {}",
                chain.join(" -> ")
            )));
        }
        for module in manifest_modules(&manifest, &source.label())? {
            if let Some(owner) = self.module_owner.get(&module) {
                if owner != &name {
                    return Err(Error::plain(format!(
                        "module {module} is declared by both {owner} and {name}; a build selects one package per module name"
                    )));
                }
            }
            self.module_owner.insert(module, name.clone());
        }
        self.stack.push(name.clone());
        let walked = self.visit_dependencies(&manifest, &source.dir, &source.label());
        self.stack.pop();
        walked?;
        let index = self.packages.len();
        self.by_name.insert(name.clone(), index);
        self.packages.push(LocalPackage {
            name,
            version,
            identity,
            manifest,
            files,
            source: source.clone(),
        });
        Ok(index)
    }

    fn visit_dependencies(&mut self, manifest: &Value, base: &Path, owner: &str) -> Result<()> {
        for dependency in parse_dependencies(manifest, owner)? {
            match &dependency.source {
                DependencySource::Registry(requirement) => {
                    self.seeds
                        .entry(dependency.name.clone())
                        .or_default()
                        .push(requirement.clone());
                }
                DependencySource::Path(path) => {
                    let dir = resolve_dependency_dir(base, path);
                    if !dir.is_dir() {
                        return Err(Error::plain(format!(
                            "{owner} declares path dependency {} at {path}, but {} is not a directory",
                            dependency.name,
                            dir.display()
                        )));
                    }
                    let source = LocalSource {
                        kind: LocalKind::Path,
                        original: path.clone(),
                        rev: None,
                        dir,
                        marker: None,
                    };
                    let index = self.visit(&source)?;
                    self.check_dependency(owner, &dependency, index)?;
                }
                DependencySource::Git { url, rev } => {
                    let checkout = git_checkout(url, rev)?;
                    let source = LocalSource {
                        kind: LocalKind::Git,
                        original: url.clone(),
                        rev: Some(rev.clone()),
                        dir: checkout.dir,
                        marker: Some(checkout.marker),
                    };
                    let index = self.visit(&source)?;
                    self.check_dependency(owner, &dependency, index)?;
                }
            }
        }
        Ok(())
    }

    fn check_dependency(&self, owner: &str, dependency: &Dependency, index: usize) -> Result<()> {
        let package = &self.packages[index];
        if package.name != dependency.name {
            return Err(Error::plain(format!(
                "{owner} declares a dependency on {}, but {} provides package {}",
                dependency.name,
                package.source.label(),
                package.name
            )));
        }
        if let Some(hint) = &dependency.hint {
            if !hint.matches(&package.version) {
                return Err(Error::plain(format!(
                    "{owner} asks for {} {}, but {} provides version {}; the version is a selection hint and the content root is the identity",
                    dependency.name,
                    hint.describe(),
                    package.source.label(),
                    package.version
                )));
            }
        }
        Ok(())
    }
}

const GIT_CACHE_DIR: &str = "git-packages";

#[cfg(test)]
thread_local! {
    /// Tests point the checkout cache at a temporary directory so a run does
    /// not accumulate clones under the user's store.
    static GIT_CACHE_OVERRIDE: std::cell::RefCell<Option<PathBuf>> =
        std::cell::RefCell::new(None);
}

fn git_cache_root() -> PathBuf {
    #[cfg(test)]
    if let Some(root) = GIT_CACHE_OVERRIDE.with(|cell| cell.borrow().clone()) {
        return root;
    }
    crate::store::store_root()
}

struct GitCheckout {
    dir: PathBuf,
    marker: PathBuf,
}

/// Clone `url` at `rev` into a content-addressed cache under the store and
/// return the checkout. The revision is a fetch hint; the caller identifies
/// the package by its content root.
fn git_checkout(url: &str, rev: &str) -> Result<GitCheckout> {
    let root = git_cache_root().join(GIT_CACHE_DIR);
    std::fs::create_dir_all(&root).map_err(|e| Error::plain(format!("{}: {e}", root.display())))?;
    let full_key = sha256_hex(format!("{url}\n{rev}").as_bytes());
    let key = &full_key[..32];
    let dir = root.join(key);
    let marker = root.join(format!("{key}.identity"));
    if dir.join(".git").is_dir() {
        let status = git_stdout(&dir, &["status", "--porcelain"])?;
        if !status.trim().is_empty() {
            return Err(Error::plain(format!(
                "git dependency {url} at {rev}: cached checkout {} has local changes; remove that directory and retry",
                dir.display()
            )));
        }
        return Ok(GitCheckout { dir, marker });
    }
    if dir.exists() {
        std::fs::remove_dir_all(&dir)
            .map_err(|e| Error::plain(format!("{}: {e}", dir.display())))?;
    }
    let tmp = root.join(format!("{key}.tmp-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    let clone = Command::new("git")
        .args(["clone", "--quiet", "--no-checkout", url])
        .arg(&tmp)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .map_err(|e| Error::plain(format!("run git clone for {url}: {e}")))?;
    if !clone.status.success() {
        let _ = std::fs::remove_dir_all(&tmp);
        return Err(Error::plain(format!(
            "git clone {url}: {}",
            String::from_utf8_lossy(&clone.stderr).trim()
        )));
    }
    if let Err(error) = checkout_rev(&tmp, rev).and_then(|()| {
        let head = git_stdout(&tmp, &["rev-parse", "HEAD"])?;
        if head.trim().is_empty() {
            return Err(Error::plain("the checkout has no HEAD"));
        }
        Ok(())
    }) {
        let _ = std::fs::remove_dir_all(&tmp);
        return Err(Error::plain(format!(
            "git dependency {url} at {rev}: {error}"
        )));
    }
    std::fs::rename(&tmp, &dir).map_err(|e| {
        let _ = std::fs::remove_dir_all(&tmp);
        Error::plain(format!("{}: {e}", dir.display()))
    })?;
    Ok(GitCheckout { dir, marker })
}

fn checkout_rev(dir: &Path, rev: &str) -> Result<()> {
    let first = git_output(dir, &["checkout", "--quiet", "--detach", "--force", rev])?;
    if first.status.success() {
        return Ok(());
    }
    let fetch = git_output(dir, &["fetch", "--quiet", "origin", rev])?;
    if !fetch.status.success() {
        return Err(Error::plain(format!(
            "revision {rev} is not in the checkout: {}",
            String::from_utf8_lossy(&first.stderr).trim()
        )));
    }
    let second = git_output(dir, &["checkout", "--quiet", "--detach", "--force", rev])?;
    if !second.status.success() {
        return Err(Error::plain(format!(
            "revision {rev}: {}",
            String::from_utf8_lossy(&second.stderr).trim()
        )));
    }
    Ok(())
}

fn git_output(dir: &Path, args: &[&str]) -> Result<std::process::Output> {
    Command::new("git")
        .args(["-c", "advice.detachedHead=false", "-C"])
        .arg(dir)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .map_err(|e| Error::plain(format!("run git {}: {e}", args.join(" "))))
}

fn git_stdout(dir: &Path, args: &[&str]) -> Result<String> {
    let output = git_output(dir, args)?;
    if !output.status.success() {
        return Err(Error::plain(format!(
            "git {} in {}: {}",
            args.join(" "),
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
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

fn existing_install(project: &Path, name: &str, version: &str) -> Result<Installed> {
    let (namespace, package) = name
        .split_once('/')
        .ok_or_else(|| Error::plain("package name must be namespace/name"))?;
    let source_dir = project
        .join("vendor")
        .join(namespace)
        .join(package)
        .join(version);
    let object = source_dir.join("package.o");
    let abi = package_execution_path(&object);
    let interfaces = source_dir.join("build");
    if !object.is_file() || !interfaces.is_dir() || read_package_execution(&object).is_err() {
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
    let mut seeds: BTreeMap<String, Vec<Requirement>> = BTreeMap::new();
    seeds.insert(
        root_name.to_string(),
        vec![Requirement::Exact(root_version.to_string())],
    );
    resolve_constraints(registry, &seeds, token).map_err(|e| {
        Error::plain(format!(
            "dependency resolution for {root_name}@{root_version}: {}",
            e.message
        ))
    })
}

/// Resolve one version per registry package name from the constraint seeds a
/// closure contributes, iterating reachable requirements to a fixed point.
fn resolve_constraints(
    registry: &str,
    seeds: &BTreeMap<String, Vec<Requirement>>,
    token: Option<&str>,
) -> Result<BTreeMap<String, String>> {
    let mut constraints: BTreeMap<String, Vec<Requirement>> = seeds.clone();
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
        let mut next: BTreeMap<String, Vec<Requirement>> = seeds.clone();
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
                        return Err(local_source_from_registry_error(package, dep_name, source));
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
    Err(Error::plain("dependency resolution did not settle"))
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
/// The package objects vendored under `project` (`vendor/<namespace>/
/// <name>/<version>/package.o`) other than the one at `except`, in path
/// order.
fn vendored_objects(project: &Path, except: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let level = |dir: &Path| -> Vec<PathBuf> {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        entries.sort();
        entries
    };
    for namespace in level(&project.join("vendor")) {
        for package in level(&namespace) {
            for version in level(&package) {
                let object = version.join("package.o");
                if version != except && object.is_file() && read_package_execution(&object).is_ok() {
                    out.push(object);
                }
            }
        }
    }
    out
}

pub fn install_bundle(
    project: &Path,
    registry: &str,
    package_id: &str,
    bundle: &Value,
    metadata_files: Option<&Value>,
    metadata_attestations: Option<&Value>,
) -> Result<Installed> {
    let manifest = bundle
        .get("manifest")
        .ok_or_else(|| Error::plain("bundle has no manifest"))?;
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no name"))?;
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
    let readme = bundle.get("readme").and_then(Value::as_str);
    let installed = materialize(
        project,
        manifest,
        &decoded,
        package_id,
        json!({"kind": "registry", "registry": registry}),
        readme,
    )?;
    if let Some(attestations) = metadata_attestations.and_then(Value::as_array) {
        for attestation in attestations {
            if attestation.get("predicateType").and_then(Value::as_str)
                != Some(crate::provenance::PREDICATE_TYPE)
            {
                continue;
            }
            if let Some(envelope) = attestation.get("envelope") {
                let path = installed.source_dir.join("provenance.jsonl");
                let digest = crate::provenance::write_envelope(&path, envelope)?;
                update_lock_provenance(
                    project,
                    installed.name.as_str(),
                    &relative_text(project, &path),
                    &digest,
                )?;
            }
        }
    }
    Ok(installed)
}

/// Write one package's source files and manifest under
/// `vendor/<namespace>/<name>/<version>/`, check it into interfaces, emit its
/// native object and ABI manifest, and record the lock entry.
fn materialize(
    project: &Path,
    manifest: &Value,
    files: &BTreeMap<String, Vec<u8>>,
    package_id: &str,
    source: Value,
    readme: Option<&str>,
) -> Result<Installed> {
    let name = manifest
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no name"))?;
    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("manifest has no version"))?;
    validate_name(name)?;
    let module_names = manifest_modules(manifest, name)?;
    for module in manifest
        .get("modules")
        .and_then(Value::as_array)
        .expect("manifest_modules checked modules")
    {
        let path = module
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("module entry has no path"))?;
        if !files.contains_key(path) {
            return Err(Error::plain(format!(
                "module {} path {path} is not in the package sources",
                module
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("<unnamed>")
            )));
        }
    }

    let (namespace, package) = name
        .split_once('/')
        .ok_or_else(|| Error::plain("package name must be namespace/name"))?;
    let source_dir = project
        .join("vendor")
        .join(namespace)
        .join(package)
        .join(version);
    if source_dir.exists() {
        std::fs::remove_dir_all(&source_dir)
            .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    }
    std::fs::create_dir_all(&source_dir)
        .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    for (path, bytes) in files {
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
    if let Some(readme) = readme {
        std::fs::write(source_dir.join("README.md"), readme)
            .map_err(|e| Error::plain(format!("{}: {e}", source_dir.display())))?;
    }

    // A synthetic library root imports every package module. It lives at the
    // project root so dependency modules vendored beside the project resolve
    // through the same recursive source index the project uses. `emit-object`
    // checks the package and writes its native object and execution interface.
    let entry = project.join(".cinch-package-root.cinch");
    let mut source_text = String::from("module CinchPackageRoot where\n");
    for module in &module_names {
        source_text.push_str(&format!("  import {module}\n"));
    }
    std::fs::write(&entry, source_text)
        .map_err(|e| Error::plain(format!("{}: {e}", entry.display())))?;
    let object = source_dir.join("package.o");
    let abi = package_execution_path(&object);
    // A package that needs system libraries has its `pkgconfig` entries
    // resolved here, before the object is built, because the build's link
    // plan refuses a package whose words are not recorded anywhere
    // (design/base-as-a-package.md section 4.4). The record lands in the
    // lock afterwards.
    let mut options = DriverOptions {
        emit_modules: module_names.clone(),
        // The dependencies already vendored here: this object calls their
        // units with the lent conventions they publish.
        lent_objects: vendored_objects(project, &source_dir),
        ..DriverOptions::default()
    };
    if let Some(native) = native_record(&source_dir)? {
        if let Some(words) = native.get("words").and_then(|w| w.as_array()) {
            let words: Vec<String> = words
                .iter()
                .filter_map(|w| w.as_str().map(str::to_string))
                .collect();
            options.native_words.insert(name.to_string(), words);
        }
    }
    let built = driver_opts(Mode::EmitObject, &entry, Some(&object), &options);
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
        // The optimisation artifact is the contract's optional companion
        // (`crate::ifaceopt`); an installed package ships it so a consumer
        // can inline the library's small units without its sources.
        let opt_from = project_build.join(format!("{module}.cinchopt"));
        let opt_to = interfaces.join(format!("{module}.cinchopt"));
        if opt_from.exists() {
            std::fs::copy(&opt_from, &opt_to)
                .map_err(|e| Error::plain(format!("{}: {e}", opt_to.display())))?;
        }
    }
    if !object.exists() || !abi.exists() || !interfaces.exists() {
        return Err(Error::plain(
            "package object emission did not produce its artifacts",
        ));
    }

    update_lock(
        project,
        name,
        version,
        package_id,
        &source,
        &source_dir,
        &object,
        &interfaces,
    )?;
    record_provenance(project, name, version, package_id, &source_dir)?;

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

/// Resolve a package's native requirements once, at install time, and
/// return what the lock has to record so a later build never consults the
/// ambient system (design/base-as-a-package.md section 4.4): the target
/// triple, each `pkgconfig` entry's resolved version and words, and the
/// semantic link words that do not depend on the host.
fn native_record(source_dir: &Path) -> Result<Option<serde_json::Value>> {
    let manifest_path = source_dir.join(".cinchpkg");
    if !manifest_path.is_file() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| Error::plain(format!("{}: {e}", manifest_path.display())))?;
    let package = crate::driver::Package::parse(&text)?;
    let facts = crate::driver::TargetFacts::of(&crate::core::TargetProfile::native());
    let resolved = package.resolve(&facts)?;
    let mut semantic = resolved.link.clone();
    // The pkgconfig entries must not survive into the recorded semantic
    // words: their words come from the resolution below, which is the
    // whole point of recording it.
    semantic.pkgconfig.clear();
    let mut pkgconfig: Vec<serde_json::Value> = Vec::new();
    let mut resolved_words: Vec<String> =
        semantic.words_for(crate::driver::LinkTier::Semantic, &facts.os);
    for entry in &resolved.link.pkgconfig {
        let version = pkg_config(&["--modversion", &entry.name], source_dir)?;
        if let Some(min) = &entry.min {
            if !crate::driver::version_matches(version.trim(), &format!(">={min}")) {
                return Err(Error::plain(format!(
                    "{}: pkg-config reports {} {}, but the manifest needs at least {min}",
                    resolved.name, entry.name, version.trim()
                )));
            }
        }
        let libs = pkg_config(&["--libs", &entry.name], source_dir)?;
        let cflags = pkg_config(&["--cflags", &entry.name], source_dir)?;
        for word in libs.split_whitespace() {
            if !resolved_words.iter().any(|w| w == word) {
                resolved_words.push(word.to_string());
            }
        }
        pkgconfig.push(serde_json::json!({
            "name": entry.name,
            "version": version.trim(),
            "libs": libs.trim(),
            "cflags": cflags.trim(),
        }));
    }
    if resolved_words.is_empty() && pkgconfig.is_empty() {
        return Ok(None);
    }
    Ok(Some(serde_json::json!({
        "target": facts.triple,
        "words": resolved_words,
        "pkgconfig": pkgconfig,
    })))
}

/// One `pkg-config` invocation, with stderr folded into the error so a
/// missing module reads as the tool's own message.
fn pkg_config(args: &[&str], dir: &Path) -> Result<String> {
    let output = std::process::Command::new("pkg-config")
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| Error::plain(format!("pkg-config {}: {e}", args.join(" "))))?;
    if !output.status.success() {
        return Err(Error::plain(format!(
            "pkg-config {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn relative_text(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn update_lock(
    project: &Path,
    name: &str,
    version: &str,
    package_id: &str,
    source: &Value,
    source_dir: &Path,
    object: &Path,
    interfaces: &Path,
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
    let mut row = json!({
        "name": name,
        "version": version,
        "package_id": package_id,
        "source": source,
        "source_dir": relative_text(project, source_dir),
        "object": relative_text(project, object),
        "interfaces": relative_text(project, interfaces)
    });
    if let Some(native) = native_record(&source_dir)? {
        row.as_object_mut()
            .expect("a lock row is an object")
            .insert("native".into(), native);
    }
    packages.push(row);
    std::fs::write(&path, canonical(&lock)?)
        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    Ok(())
}

fn record_provenance(
    project: &Path,
    name: &str,
    version: &str,
    package_id: &str,
    source_dir: &Path,
) -> Result<()> {
    let path = source_dir.join("provenance.jsonl");
    let digest = crate::provenance::write_jsonl(&path, package_id, name, version)?;
    update_lock_provenance(project, name, &relative_text(project, &path), &digest)
}

fn update_lock_provenance(project: &Path, name: &str, path: &str, digest: &str) -> Result<()> {
    let lock_path = project.join(".cinchlock");
    let bytes = std::fs::read(&lock_path)
        .map_err(|e| Error::plain(format!("{}: {e}", lock_path.display())))?;
    let mut lock: Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("{}: {e}", lock_path.display())))?;
    let packages = lock
        .get_mut("packages")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| Error::plain(".cinchlock has no packages array"))?;
    let row = packages
        .iter_mut()
        .find(|row| row.get("name").and_then(Value::as_str) == Some(name))
        .ok_or_else(|| Error::plain(format!("{name} is missing from .cinchlock")))?;
    let object = row
        .as_object_mut()
        .ok_or_else(|| Error::plain("lock entry must be an object"))?;
    object.insert("provenance".into(), json!({"path": path, "digest": digest}));
    std::fs::write(&lock_path, canonical(&lock)?)
        .map_err(|e| Error::plain(format!("{}: {e}", lock_path.display())))?;
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
                && matches!(
                    name.as_str(),
                    ".cinchpkg" | "README.md" | "provenance.jsonl"
                )
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
    let provenance = crate::provenance::envelope(&package_id, name, version)?;
    let body = json!({
        "manifest": manifest,
        "files": encoded,
        "visibility": visibility,
        "readme": readme,
        "packageId": package_id,
        "attestations": [{
            "predicateType": crate::provenance::PREDICATE_TYPE,
            "envelope": provenance,
        }],
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
        let abi_path = package_execution_path(&object);
        let object_bytes = std::fs::read(&object)
            .map_err(|e| Error::plain(format!("{}: {e}", object.display())))?;
        let abi = read_package_execution(&object)?;
        if abi.get("modules") != Some(&json!(manifest_modules(&manifest, name)?)) {
            return Err(Error::plain(format!(
                "{}: package execution modules differ from .cinchpkg",
                abi_path.display()
            )));
        }
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
        provenance_levels: Vec::new(),
    })
}

/// Verify the local L1 provenance files recorded by install. A registry can
/// raise an attestation to L2 by accepting a trusted builder signature, but
/// this client does not pretend to hold those roots of trust.
fn verify_slsa(project: &Path) -> Result<Verified> {
    let path = project.join(".cinchlock");
    let bytes =
        std::fs::read(&path).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let lock: Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let packages = lock
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain(".cinchlock has no packages array"))?;
    let mut levels = Vec::new();
    for row in packages {
        let name = row
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain("lock entry has no name"))?;
        let package_id = row
            .get("package_id")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::plain(format!("{name} lock entry has no package_id")))?;
        let source_dir = project.join(
            row.get("source_dir")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain(format!("{name} lock entry has no source_dir")))?,
        );
        let provenance = row
            .get("provenance")
            .ok_or_else(|| Error::plain(format!("{name} has no recorded provenance")))?;
        let provenance_path = project.join(
            provenance
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| Error::plain(format!("{name} provenance has no path")))?,
        );
        let line = std::fs::read_to_string(&provenance_path)
            .map_err(|e| Error::plain(format!("{}: {e}", provenance_path.display())))?;
        let envelope: Value = serde_json::from_str(line.trim())
            .map_err(|e| Error::plain(format!("{}: {e}", provenance_path.display())))?;
        let check = crate::provenance::verify(&envelope, package_id)?;
        let digest = provenance
            .get("digest")
            .and_then(Value::as_str)
            .unwrap_or("<missing>");
        let bytes = std::fs::read(&provenance_path)
            .map_err(|e| Error::plain(format!("{}: {e}", provenance_path.display())))?;
        let mut sha = Sha256::new();
        sha.update(&bytes);
        let actual = format!(
            "sha256:{}",
            sha.finalize()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        );
        if actual != digest {
            return Err(Error::plain(format!(
                "{}: provenance digest mismatch: lock says {digest}, file is {actual}",
                provenance_path.display()
            )));
        }
        if source_dir.join("provenance.jsonl") != provenance_path {
            return Err(Error::plain(format!(
                "{name}: recorded provenance is not beside the installed source"
            )));
        }
        levels.push(format!(
            "{}@{} {}",
            name,
            row.get("version")
                .and_then(Value::as_str)
                .unwrap_or("<unknown>"),
            check.level
        ));
    }
    Ok(Verified {
        packages: packages.len(),
        provenance_levels: levels,
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
        let constraints = vec![
            Requirement::Range(">=0.1.0".to_string()),
            Requirement::Range("<0.2.0".to_string()),
        ];
        let versions = vec!["0.1.0".to_string(), "0.2.0".to_string()];
        assert_eq!(
            select_version("alice/base", &constraints, &versions).expect("intersection"),
            "0.1.0"
        );
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
        let installed = install_bundle(
            &dir,
            "http://registry.local",
            &package_id,
            &bundle,
            None,
            None,
        )
        .expect("install bundle");
        assert!(installed.source_dir.join("Library.cinch").exists());
        assert!(installed.object.exists());
        assert!(installed.interfaces.join("Library.cinchif").exists());
        assert_eq!(installed.abi, installed.interfaces.join("package.cinchif"));
        assert!(!installed.object.with_extension("abi.json").exists());
        let abi: Value =
            serde_json::from_slice(&std::fs::read(&installed.abi).expect("package ABI"))
                .expect("package ABI JSON");
        let units = abi["units"].as_array().expect("ABI units");
        assert!(
            units
                .iter()
                .all(|unit| unit["symbol"]
                    .as_str()
                    .is_some_and(|symbol| symbol == "Library"
                        || symbol.starts_with("Library.")
                        || symbol.starts_with("cinch.")
                        || symbol.starts_with("<builtin>"))),
            "package object must not bundle dependency modules, got {units:?}"
        );
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
        let old_sidecar = installed.object.with_extension("abi.json");
        std::fs::rename(&installed.abi, &old_sidecar).expect("move execution interface aside");
        let error = verify(&dir).err().expect("verification requires package.cinchif");
        assert!(error.to_string().contains("package.cinchif"), "{error}");
        std::fs::rename(&old_sidecar, &installed.abi).expect("restore execution interface");
        let slsa = verify_slsa(&dir).expect("verify slsa");
        assert_eq!(slsa.packages, 1);
        assert!(slsa.provenance_levels[0].contains("SLSA_BUILD_LEVEL_1"));
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

    fn temp_dir(prefix: &str) -> tempfile::TempDir {
        tempfile::Builder::new()
            .prefix(prefix)
            .tempdir()
            .expect("temporary directory")
    }

    /// Write `.cinchpkg` plus the package's module sources.
    fn write_package(dir: &Path, manifest: Value, sources: &[(&str, &str)]) {
        std::fs::create_dir_all(dir).expect("package directory");
        std::fs::write(
            dir.join(".cinchpkg"),
            canonical(&manifest).expect("canonical manifest"),
        )
        .expect("package manifest");
        for (path, source) in sources {
            let target = dir.join(path);
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent).expect("source directory");
            }
            std::fs::write(target, source).expect("package source");
        }
    }

    fn package_manifest(name: &str, version: &str, module: &str, dependencies: Value) -> Value {
        let mut manifest = json!({
            "schema": 2,
            "name": name,
            "version": version,
            "modules": [{"name": module, "path": format!("{module}.cinch")}],
            "order": [module]
        });
        if !dependencies.as_array().map(Vec::is_empty).unwrap_or(true) {
            manifest["dependencies"] = dependencies;
        }
        manifest
    }

    fn write_project(project: &Path, name: &str, dependencies: Value) {
        std::fs::create_dir_all(project).expect("project directory");
        let manifest = package_manifest(name, "0.1.0", "App", dependencies);
        std::fs::write(
            project.join(".cinchpkg"),
            canonical(&manifest).expect("project manifest"),
        )
        .expect("project manifest");
    }

    fn install_project(project: &Path, registry: &str, locked: bool) -> Result<Vec<Installed>> {
        InstallContext::new(project, registry, None, locked).install_project_manifest()
    }

    #[test]
    fn path_dependencies_install_sources_and_lock_entries() {
        let root = temp_dir("cinch-pkg-path");
        let project = root.path().join("app");
        let bytes = root.path().join("libs/bytes");
        write_package(
            &bytes,
            package_manifest("acme/bytes", "0.1.0", "Bytes", json!([])),
            &[(
                "Bytes.cinch",
                "module Bytes where\n  value :: Int\n  value = 7\n",
            )],
        );
        let text = root.path().join("libs/text");
        write_package(
            &text,
            package_manifest(
                "acme/text",
                "0.2.0",
                "Text",
                json!([{"name": "acme/bytes", "source": "path", "path": "../bytes", "version": "0.1.0"}]),
            ),
            &[(
                "Text.cinch",
                "module Text where\n  import Bytes\n  doubled :: Int\n  doubled = value + value\n",
            )],
        );
        write_project(
            &project,
            "acme/app",
            json!([{"name": "acme/text", "source": "path", "path": "../libs/text"}]),
        );

        let installed =
            install_project(&project, "http://127.0.0.1:9", false).expect("install path closure");
        assert_eq!(installed.len(), 2);
        assert_eq!(installed[0].name, "acme/bytes");
        assert_eq!(installed[1].name, "acme/text");
        assert!(project
            .join("vendor/acme/bytes/0.1.0/Bytes.cinch")
            .is_file());
        assert!(project.join("vendor/acme/text/0.2.0/Text.cinch").is_file());

        let lock = read_lock(&project).expect("lock");
        let bytes_row = lock.get("acme/bytes").expect("bytes lock entry");
        assert_eq!(bytes_row["source"]["kind"], "path");
        assert_eq!(bytes_row["source"]["path"], "../bytes");
        assert_eq!(
            bytes_row["package_id"],
            package_identity(
                &read_manifest(&bytes).expect("bytes manifest"),
                &manifest_files(&bytes, &read_manifest(&bytes).expect("manifest"))
                    .expect("bytes files")
            )
            .expect("identity")
        );
        let text_row = lock.get("acme/text").expect("text lock entry");
        assert_eq!(text_row["source"]["path"], "../libs/text");

        std::fs::write(
            project.join("Main.cinch"),
            "module Main where\n  import Text\n  main :: Int\n  main = doubled\n",
        )
        .expect("main source");
        driver_opts(
            Mode::Check,
            &project.join("Main.cinch"),
            None,
            &DriverOptions::default(),
        )
        .expect("project imports installed path packages");
        assert_eq!(verify(&project).expect("verify path closure").packages, 2);
    }

    #[test]
    fn path_dependency_cycles_are_named() {
        let root = temp_dir("cinch-pkg-cycle");
        let project = root.path().join("app");
        let first = root.path().join("a");
        let second = root.path().join("b");
        write_package(
            &first,
            package_manifest(
                "acme/a",
                "0.1.0",
                "A",
                json!([{"name": "acme/b", "source": "path", "path": "../b"}]),
            ),
            &[(
                "A.cinch",
                "module A where\n  import B\n  value :: Int\n  value = other\n",
            )],
        );
        write_package(
            &second,
            package_manifest(
                "acme/b",
                "0.1.0",
                "B",
                json!([{"name": "acme/a", "source": "path", "path": "../a"}]),
            ),
            &[("B.cinch", "module B where\n  other :: Int\n  other = 1\n")],
        );
        write_project(
            &project,
            "acme/app",
            json!([{"name": "acme/a", "source": "path", "path": "../a"}]),
        );
        let error = install_project(&project, "http://127.0.0.1:9", false)
            .expect_err("a path cycle must be rejected");
        assert!(
            error.message.contains("dependency cycle")
                && error.message.contains("acme/a -> acme/b -> acme/a"),
            "{}",
            error.message
        );
    }

    #[test]
    fn one_package_name_cannot_resolve_to_two_contents() {
        let root = temp_dir("cinch-pkg-conflict");
        let project = root.path().join("app");
        let first = root.path().join("one");
        let second = root.path().join("two");
        write_package(
            &first,
            package_manifest("acme/dup", "0.1.0", "First", json!([])),
            &[(
                "First.cinch",
                "module First where\n  value :: Int\n  value = 1\n",
            )],
        );
        write_package(
            &second,
            package_manifest("acme/dup", "0.1.0", "Second", json!([])),
            &[(
                "Second.cinch",
                "module Second where\n  value :: Int\n  value = 2\n",
            )],
        );
        write_project(
            &project,
            "acme/app",
            json!([
                {"name": "acme/dup", "source": "path", "path": "../one"},
                {"name": "acme/dup", "source": "path", "path": "../two"}
            ]),
        );
        let error = install_project(&project, "http://127.0.0.1:9", false)
            .expect_err("two contents under one name must be rejected");
        assert!(
            error.message.contains("one version per package name"),
            "{}",
            error.message
        );
    }

    #[test]
    fn missing_and_dirty_path_dependencies_are_named() {
        let root = temp_dir("cinch-pkg-dirty");
        let project = root.path().join("app");
        write_project(
            &project,
            "acme/app",
            json!([{"name": "acme/gone", "source": "path", "path": "../gone"}]),
        );
        let error = install_project(&project, "http://127.0.0.1:9", false)
            .expect_err("a missing path dependency must be rejected");
        assert!(
            error.message.contains("is not a directory"),
            "{}",
            error.message
        );

        let library = root.path().join("libs/lib");
        write_package(
            &library,
            package_manifest("acme/lib", "0.1.0", "Lib", json!([])),
            &[(
                "Lib.cinch",
                "module Lib where\n  value :: Int\n  value = 1\n",
            )],
        );
        write_project(
            &project,
            "acme/app",
            json!([{"name": "acme/lib", "source": "path", "path": "../libs/lib"}]),
        );
        install_project(&project, "http://127.0.0.1:9", false).expect("clean install");
        std::fs::write(
            library.join("Lib.cinch"),
            "module Lib where\n  value :: Int\n  value = 2\n",
        )
        .expect("edit path source");

        let error = install_project(&project, "http://127.0.0.1:9", true)
            .expect_err("a dirty path dependency must be rejected under --locked");
        assert!(error.message.contains("dirty"), "{}", error.message);

        install_project(&project, "http://127.0.0.1:9", false).expect("refresh the lock");
        assert_eq!(verify(&project).expect("verify refreshed lock").packages, 1);
    }

    fn run_git(dir: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(["-c", "advice.detachedHead=false"])
            .args(args)
            .current_dir(dir)
            .env("GIT_TERMINAL_PROMPT", "0")
            .output()
            .expect("run git");
        assert!(
            output.status.success(),
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).to_string()
    }

    #[test]
    fn git_dependency_installs_from_a_local_repository() {
        let root = temp_dir("cinch-pkg-git");
        let cache = root.path().join("cache");
        GIT_CACHE_OVERRIDE.with(|cell| *cell.borrow_mut() = Some(cache));
        let repo = root.path().join("remote");
        write_package(
            &repo,
            package_manifest("acme/gitlib", "0.3.0", "GitLib", json!([])),
            &[(
                "GitLib.cinch",
                "module GitLib where\n  value :: Int\n  value = 9\n",
            )],
        );
        run_git(&repo, &["init", "--quiet"]);
        run_git(&repo, &["config", "user.email", "test@example.com"]);
        run_git(&repo, &["config", "user.name", "Cinch Test"]);
        run_git(&repo, &["add", "."]);
        run_git(&repo, &["commit", "--quiet", "-m", "release"]);
        let rev = run_git(&repo, &["rev-parse", "HEAD"]).trim().to_string();

        let project = root.path().join("app");
        let url = repo.to_str().expect("repository path").to_string();
        write_project(
            &project,
            "acme/app",
            json!([{"name": "acme/gitlib", "source": "git", "url": url, "rev": rev}]),
        );
        let installed =
            install_project(&project, "http://127.0.0.1:9", false).expect("install git package");
        assert_eq!(installed.len(), 1);
        assert!(installed[0].source_dir.join("GitLib.cinch").is_file());
        let lock = read_lock(&project).expect("lock");
        let row = lock.get("acme/gitlib").expect("git lock entry");
        assert_eq!(row["source"]["kind"], "git");
        assert_eq!(row["source"]["rev"], rev);
        assert_eq!(
            row["package_id"],
            package_identity(
                &read_manifest(&repo).expect("repo manifest"),
                &manifest_files(&repo, &read_manifest(&repo).expect("manifest"))
                    .expect("repo files")
            )
            .expect("identity")
        );

        // A second install reuses the checkout cache and leaves the lock alone.
        install_project(&project, "http://127.0.0.1:9", true).expect("cached git install");
        assert_eq!(verify(&project).expect("verify git closure").packages, 1);
        GIT_CACHE_OVERRIDE.with(|cell| *cell.borrow_mut() = None);
    }

    struct RegistryFixture {
        manifest: Value,
        files: BTreeMap<String, Vec<u8>>,
    }

    fn registry_fixture(manifest: Value, sources: &[(&str, &str)]) -> RegistryFixture {
        let files = sources
            .iter()
            .map(|(path, source)| (path.to_string(), source.as_bytes().to_vec()))
            .collect();
        RegistryFixture { manifest, files }
    }

    fn registry_routes(fixtures: &[RegistryFixture]) -> BTreeMap<String, String> {
        let mut routes = BTreeMap::new();
        for fixture in fixtures {
            let manifest = &fixture.manifest;
            let name = manifest["name"].as_str().expect("fixture name").to_string();
            let version = manifest["version"]
                .as_str()
                .expect("fixture version")
                .to_string();
            let package_id = package_identity(manifest, &fixture.files).expect("fixture identity");
            let hex = package_id.trim_start_matches("sha256:");
            let encoded: BTreeMap<String, String> = fixture
                .files
                .iter()
                .map(|(path, bytes)| (path.clone(), base64_encode(bytes)))
                .collect();
            let bundle = json!({
                "schema": 1,
                "manifest": manifest,
                "files": encoded,
                "visibility": "public"
            });
            let encoded_name = name.replace('/', "%2F");
            routes.insert(
                format!("/api/v1/packages/{encoded_name}"),
                json!({"versions": [{"version": version}]}).to_string(),
            );
            routes.insert(
                format!("/api/v1/packages/{encoded_name}/{version}"),
                json!({"packageId": package_id, "manifest": manifest}).to_string(),
            );
            routes.insert(format!("/api/v1/download/{hex}"), bundle.to_string());
        }
        routes
    }

    fn start_registry(routes: BTreeMap<String, String>) -> String {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind registry");
        let address = listener.local_addr().expect("registry address");
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let mut buffer = [0u8; 8192];
                let read = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..read]);
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .unwrap_or("/")
                    .to_string();
                let (status, body) = match routes.get(&path) {
                    Some(body) => ("200 OK", body.clone()),
                    None => ("404 Not Found", "{}".to_string()),
                };
                let response = format!(
                    "HTTP/1.1 {status}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = stream.write_all(response.as_bytes());
            }
        });
        format!("http://{address}")
    }

    #[test]
    fn registry_and_path_dependencies_share_one_closure() {
        let registry = start_registry(registry_routes(&[registry_fixture(
            package_manifest("reg/util", "1.2.0", "Util", json!([])),
            &[(
                "Util.cinch",
                "module Util where\n  base :: Int\n  base = 3\n",
            )],
        )]));
        let root = temp_dir("cinch-pkg-mixed");
        let project = root.path().join("app");
        let local = root.path().join("libs/local");
        write_package(
            &local,
            package_manifest(
                "acme/local",
                "0.1.0",
                "Local",
                json!([{"name": "reg/util", "source": "registry", "range": "^1.0.0"}]),
            ),
            &[(
                "Local.cinch",
                "module Local where\n  import Util\n  total :: Int\n  total = base + 1\n",
            )],
        );
        write_project(
            &project,
            "acme/app",
            json!([
                {"name": "acme/local", "source": "path", "path": "../libs/local"},
                {"name": "reg/util", "source": "registry", "range": "^1.0.0"}
            ]),
        );

        let installed = install_project(&project, &registry, false).expect("mixed closure");
        assert_eq!(installed.len(), 2);
        let lock = read_lock(&project).expect("lock");
        assert_eq!(
            lock.get("reg/util").expect("registry entry")["version"],
            "1.2.0"
        );
        assert_eq!(
            lock.get("acme/local").expect("local entry")["source"]["kind"],
            "path"
        );

        std::fs::write(
            project.join("Main.cinch"),
            "module Main where\n  import Local\n  main :: Int\n  main = total\n",
        )
        .expect("main source");
        driver_opts(
            Mode::Check,
            &project.join("Main.cinch"),
            None,
            &DriverOptions::default(),
        )
        .expect("project imports the mixed closure");
        assert_eq!(verify(&project).expect("verify mixed closure").packages, 2);
    }

    #[test]
    fn registry_releases_cannot_disclose_path_dependencies() {
        let registry = start_registry(registry_routes(&[registry_fixture(
            package_manifest(
                "reg/bad",
                "0.1.0",
                "Bad",
                json!([{"name": "acme/other", "source": "path", "path": "../other"}]),
            ),
            &[(
                "Bad.cinch",
                "module Bad where\n  value :: Int\n  value = 1\n",
            )],
        )]));
        let root = temp_dir("cinch-pkg-registry-path");
        let project = root.path().join("app");
        write_project(
            &project,
            "acme/app",
            json!([{"name": "reg/bad", "source": "registry", "range": "^0.1.0"}]),
        );
        let error = install_project(&project, &registry, false)
            .expect_err("a registry path disclosure must be rejected");
        assert!(
            error
                .message
                .contains("declares a path dependency on acme/other")
                && error.message.contains("project manifest"),
            "{}",
            error.message
        );
    }
    #[test]
    fn native_build_consumes_package_objects_and_dependencies() {
        let dir = std::env::temp_dir().join(format!("cinch-pkg-native-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("project");
        let install = |name: &str, module: &str, source: &[u8]| {
            let manifest = json!({
                "schema": 2,
                "name": name,
                "version": "0.1.0",
                "modules": [{"name": module, "path": format!("{module}.cinch")}],
                "order": [module],
            });
            let mut files = BTreeMap::new();
            files.insert(format!("{module}.cinch"), source.to_vec());
            let id = package_identity(&manifest, &files).expect("identity");
            let bundle = json!({
                "schema": 1,
                "manifest": manifest,
                "files": {format!("{module}.cinch"): base64_encode(source)},
                "visibility": "public",
            });
            install_bundle(&dir, "http://registry.local", &id, &bundle, None, None)
                .expect("install package")
        };
        let base = install(
            "acme/base",
            "Base",
            b"module Base where\n  value :: Int\n  value = 7\n",
        );
        let top = install(
            "acme/top",
            "Top",
            b"module Top where\n  import Base\n  answer :: Int\n  answer = value + 1\n",
        );
        std::fs::write(
            dir.join("Main.cinch"),
            "module Main where\n  import Top\n  main :: Int\n  main = answer\n",
        )
        .expect("entry");
        let options = DriverOptions {
            package_objects: vec![top.object.clone(), base.object.clone()],
            ..DriverOptions::default()
        };
        let status = driver_opts(Mode::Run, &dir.join("Main.cinch"), None, &options)
            .expect("native build with package objects");
        assert_eq!(status, 8);
        let execution_bytes = std::fs::read(&top.abi).expect("package execution interface");
        let mut execution: Value =
            serde_json::from_slice(&execution_bytes).expect("package execution JSON");
        execution["runtime_abi"] = json!(crate::interface::RUNTIME_ABI_VERSION - 1);
        std::fs::write(
            &top.abi,
            canonical(&execution).expect("encode old runtime ABI"),
        )
        .expect("replace package execution interface");
        let error = driver_opts(Mode::Run, &dir.join("Main.cinch"), None, &options)
            .expect_err("native linker must reject an old package runtime ABI");
        assert!(
            error.to_string().contains("package object runtime ABI"),
            "{error}"
        );
        std::fs::write(&top.abi, execution_bytes).expect("restore package execution interface");
        let old_sidecar = top.object.with_extension("abi.json");
        std::fs::rename(&top.abi, &old_sidecar).expect("move execution interface aside");
        let error = driver_opts(Mode::Run, &dir.join("Main.cinch"), None, &options)
            .expect_err("native package linking requires package.cinchif");
        assert!(error.to_string().contains("package.cinchif"), "{error}");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Borrow modes cross the object boundary. `Base.count` only reads
    /// its chain (a type of Base's own: a package object does not carry the
    /// prelude's units, so a prelude list could not cross the link), so its object publishes it lent (a worker that borrows,
    /// behind an owned public symbol). `Top`, built against Base's object,
    /// and `Main`, linked against both, call the worker and keep the list;
    /// `Main` also reaches `Base.total` through a closure, which enters by
    /// the owned symbol. Every path must balance its counts.
    #[test]
    fn borrow_modes_cross_package_objects() {
        let dir = std::env::temp_dir().join(format!("cinch-pkg-lent-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("project");
        let install = |name: &str, module: &str, source: &[u8]| {
            let manifest = json!({
                "schema": 2,
                "name": name,
                "version": "0.1.0",
                "modules": [{"name": module, "path": format!("{module}.cinch")}],
                "order": [module],
            });
            let mut files = BTreeMap::new();
            files.insert(format!("{module}.cinch"), source.to_vec());
            let id = package_identity(&manifest, &files).expect("identity");
            let bundle = json!({
                "schema": 1,
                "manifest": manifest,
                "files": {format!("{module}.cinch"): base64_encode(source)},
                "visibility": "public",
            });
            install_bundle(&dir, "http://registry.local", &id, &bundle, None, None)
                .expect("install package")
        };
        let base = install(
            "acme/base",
            "Base",
            b"module Base where\n  data Chain where\n    End :: Chain\n    Link :: Int -> Chain -> Chain\n  count :: Chain -> Int -> Int\n  count xs acc = case xs of\n    End -> acc\n    Link x rest -> count rest (acc + 1)\n  total :: Chain -> Int\n  total xs = count xs 0 + count xs 0\n",
        );
        let top = install(
            "acme/top",
            "Top",
            b"module Top where\n  import Base\n  answer :: Chain -> Int\n  answer xs = total xs + count xs 1\n",
        );
        std::fs::write(
            dir.join("Main.cinch"),
            "module Main where\n  import Base\n  import Top\n  apply :: (Chain -> Int) -> Chain -> Int\n  apply f xs = f xs\n  main :: Int\n  main =\n    let xs = Link 1 (Link 2 (Link 3 End)) in\n    if answer xs + total xs + apply total xs == 22 then 0 else 1\n",
        )
        .expect("entry");
        let execution = read_package_execution(&base.object).expect("base execution interface");
        let unit = |symbol: &str| {
            execution["units"]
                .as_array()
                .and_then(|units| units.iter().find(|u| u["symbol"] == symbol))
                .cloned()
                .unwrap_or(Value::Null)
        };
        assert_eq!(unit("Base.count")["lent"], json!([true, false]), "{}", unit("Base.count"));
        assert_eq!(unit("Base.count")["lent_entry"], json!("Base.count#tail"));
        assert_eq!(unit("Base.total")["lent"], json!([true]), "{}", unit("Base.total"));
        let options = DriverOptions {
            package_objects: vec![top.object.clone(), base.object.clone()],
            ..DriverOptions::default()
        };
        // Top was compiled against Base's published modes: it calls a lent
        // worker (an undefined symbol of its object; `total` itself may be
        // inlined, which leaves the call to `count`'s), and it records the
        // conventions it assumed for its final link to check.
        let top_bytes = std::fs::read(&top.object).expect("top object");
        assert!(
            top_bytes.windows(b"Base.count#tail".len()).any(|w| w == b"Base.count#tail"),
            "Top must call Base's lent worker"
        );
        let top_execution = read_package_execution(&top.object).expect("top execution interface");
        let imports = top_execution["lent_imports"].as_array().cloned().unwrap_or_default();
        assert!(
            imports.iter().any(|r| r["symbol"] == "Base.count" && r["lent"] == json!([true, false])),
            "{imports:?}"
        );
        let exe = dir.join("main-lent");
        driver_opts(Mode::Build, &dir.join("Main.cinch"), Some(&exe), &options)
            .expect("native build with package objects");
        for debug in [false, true] {
            let mut cmd = Command::new(&exe);
            cmd.env("CINCH_LEAK_CHECK", "1");
            if debug {
                cmd.env("CINCH_RC_DEBUG", "1");
            }
            let out = cmd.output().expect("run");
            let err = String::from_utf8_lossy(&out.stderr);
            assert_eq!(out.status.code(), Some(0), "debug {debug}: {err}");
            assert!(err.contains("leaked=0"), "debug {debug}: {err}");
        }
        // A Base that publishes a different convention than the one Top was
        // compiled against is refused at the link, not linked into an
        // imbalance.
        let path = package_execution_path(&base.object);
        let mut published: Value = serde_json::from_slice(&std::fs::read(&path).expect("base ABI")).expect("JSON");
        for unit in published["units"].as_array_mut().expect("units") {
            if unit["symbol"] == "Base.count" {
                unit["lent"] = json!([false, false]);
            }
        }
        std::fs::write(&path, canonical(&published).expect("encode")).expect("rewrite base ABI");
        let error = driver_opts(Mode::Build, &dir.join("Main.cinch"), Some(&exe), &options)
            .expect_err("a changed lent convention must be refused");
        assert!(error.to_string().contains("lent convention for Base.count"), "{error}");
        std::fs::remove_dir_all(&dir).ok();
    }
}
