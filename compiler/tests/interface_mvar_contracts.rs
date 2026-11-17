//! Checked MVar rows retain their owner, complete effects and cancellation capabilities.
//!
//! Native source-absent checks require the separately reviewed Native MVar source increment;
//! a Rust interface roundtrip alone is not that semantic import gate.
mod common;

use cinch::interface::Iface;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

/// Retain the entire test project outside the stale-project pruner after any panic.
struct ProjectEvidence(PathBuf);

impl std::ops::Deref for ProjectEvidence {
    type Target = Path;

    /// Expose the unchanged project path to the shared compiler harness.
    fn deref(&self) -> &Path {
        &self.0
    }
}

impl Drop for ProjectEvidence {
    /// Keep failed producer inputs, generated interfaces and every importer attempt inspectable.
    fn drop(&mut self) {
        if std::thread::panicking() && self.0.exists() {
            let name = self.0.file_name().unwrap().to_string_lossy();
            let retained = self.0.with_file_name(format!("cinch_mvar_failure_{name}"));
            let path = if std::fs::rename(&self.0, &retained).is_ok() {
                retained
            } else {
                self.0.clone()
            };
            let _ = std::fs::write(path.join("retained-project.json"), serde_json::json!({
                "original_root":self.0, "retained_root":path,
                "note":"Exact manifests remain unchanged; translate this project root when replaying absolute paths.",
            }).to_string());
            eprintln!(
                "MVar contract failure evidence retained at {}",
                path.display()
            );
        }
    }
}

/// Allocate a distinct evidence directory before any shared input or report can be overwritten.
fn attempt(directory: &Path, label: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let path = directory.join("attempts").join(format!(
        "{:04}-{label}",
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&path).unwrap();
    path
}

/// Check the real public wrapper plus the exact fourteen primitive declarations in a nonembedded owner.
fn producer() -> (ProjectEvidence, Vec<Iface>) {
    let public = include_str!("../prelude/Effect/Concurrent/MVar.cinch");
    let owner = public.replace(
        "module Effect.Concurrent.MVar where",
        "module MVarOwner where",
    );
    assert_ne!(owner, public);
    let mut declarations = Vec::new();
    let mut names = Vec::new();
    let mut comment = "";
    for line in public.lines() {
        if line.trim_start().starts_with("-- |") {
            comment = line;
        }
        if line.trim_start().starts_with("foreign import ccall") {
            let tail = line
                .split_once("\" ")
                .expect("canonical intrinsic spelling")
                .1;
            names.push(
                tail.split_once(" :: ")
                    .expect("canonical intrinsic scheme")
                    .0,
            );
            declarations.push(format!("{comment}\n{line}\n"));
        }
    }
    assert_eq!(declarations.len(), 14);
    let kernel = format!("-- | Export the genuine kernel contracts for source-absent authority checks.\nmodule KernelMVar where\n  import Data.Option\n  export {}\n{}",
        names.join(", "), declarations.concat());
    let source = format!(
        "{owner}\n\n{kernel}\n\n{}",
        include_str!("fixtures/mvar-interface-contracts.cinch")
    );
    let dir = ProjectEvidence(common::project(
        "mvar_contract_producer",
        &[("Producer.cinch", &source)],
    ));
    let original = attempt(&dir, "producer");
    std::fs::write(original.join("Producer.source"), &source).unwrap();
    let outcome = common::check_interfaces(&dir, "Producer.cinch");
    let diagnostic = match &outcome {
        Ok(rows) => format!("checked {} genuine interfaces\n", rows.len()),
        Err(error) => error.clone(),
    };
    std::fs::write(dir.join("producer-check.txt"), diagnostic).unwrap();
    let interfaces = outcome.expect("check genuine MVar contracts");
    cinch::interface::verify_chain(&interfaces).unwrap();
    (dir, interfaces)
}

/// Require all fourteen genuine primitive rows and public wrappers to retain their different obligations.
#[test]
fn mvar_rows_preserve_kernel_and_public_capability_contracts() {
    let (dir, interfaces) = producer();
    let kernel = interfaces
        .iter()
        .find(|row| row.module == "KernelMVar")
        .unwrap();
    let owner = interfaces
        .iter()
        .find(|row| row.module == "MVarOwner")
        .unwrap();
    let facade = interfaces
        .iter()
        .find(|row| row.module == "MVarFacade")
        .unwrap();
    let public_type = owner
        .exports
        .types
        .iter()
        .find(|row| row.name == "MVar")
        .unwrap();
    let reexported_type = facade
        .exports
        .types
        .iter()
        .find(|row| row.name == "MVar")
        .unwrap();
    assert!(public_type.ctors.is_empty() && reexported_type.ctors.is_empty());
    let proof = public_type
        .ownership
        .as_ref()
        .expect("checked private fields survive a header-only import");
    assert_eq!(reexported_type.ownership.as_ref(), Some(proof));
    assert_eq!(proof.constructors.as_deref().unwrap(), ["MVarOwner.MkMVar"]);
    assert_eq!(proof.usage.as_ref().unwrap()[0].fields, ["(MVarCell a)"]);
    assert_eq!(owner.exports.values.len(), 14);
    for operation in cinch::mvar::MVarOp::ALL {
        let external = format!("cinch.runtime.{}", operation.name());
        let foreign = kernel
            .exports
            .foreign
            .iter()
            .find(|row| row.external == external)
            .unwrap();
        let value = kernel
            .exports
            .values
            .iter()
            .find(|row| row.name == foreign.symbol)
            .unwrap();
        let checked = value
            .resolved
            .as_ref()
            .expect("a checked kernel row must be portable");
        assert_eq!(value.arity, operation.sig().0.len() as u64);
        assert!(checked.scheme.contains("MVar"));
        assert_eq!(
            checked.scheme.contains("Diverge"),
            operation.blocking(),
            "{external}"
        );
        assert!(
            !checked.scheme.contains("Io ") && !checked.scheme.contains("Cancel "),
            "kernel cancellation does not invent a public capability"
        );
        assert!(checked
            .nominals
            .iter()
            .all(|row| !row.identity.starts_with("<builtin>.")));
        assert!(
            value.total_body.is_none(),
            "effect transport grants no executable body authority"
        );
    }
    for value in &owner.exports.values {
        let checked = value.resolved.as_ref().expect("public MVar row");
        let exported = facade
            .exports
            .values
            .iter()
            .find(|row| row.name == value.name)
            .unwrap();
        assert_eq!(exported.symbol, value.symbol);
        assert_eq!(exported.resolved, value.resolved);
        assert!(checked
            .nominals
            .iter()
            .any(|row| row.identity == "MVarOwner.MVar"));
        assert!(
            !checked.scheme.contains("Share ") && !checked.scheme.contains("Copy "),
            "local cell operations do not require shareable payloads"
        );
        if value.name.ends_with("-or-cancel") {
            assert!(
                checked.scheme.contains("Io r")
                    && checked.scheme.contains("Cancel r")
                    && checked.scheme.contains("r :: Region"),
                "{}",
                checked.scheme
            );
        }
        assert!(value.total_body.is_none());
    }
    let rows = interfaces
        .iter()
        .find(|row| row.module == "MVarRows")
        .unwrap();
    for name in ["renamed-cancel", "callback"] {
        assert!(
            rows.exports
                .values
                .iter()
                .find(|row| row.name == name)
                .unwrap()
                .resolved
                .is_some(),
            "{name}"
        );
    }
    assert!(rows
        .exports
        .values
        .iter()
        .find(|row| row.name == "open-row")
        .unwrap()
        .resolved
        .is_none());
    let user = interfaces
        .iter()
        .find(|row| row.module == "UserMVar")
        .unwrap();
    assert!(
        user.exports
            .values
            .iter()
            .find(|row| row.name == "signal")
            .unwrap()
            .resolved
            .is_none(),
        "ordinary source effects cannot acquire kernel authority"
    );
    for interface in [kernel, owner, facade, rows, user] {
        assert_eq!(Iface::from_json(&interface.to_json()).unwrap(), *interface);
    }
    common::cleanup(&dir);
}

/// Genuine source declarations cannot replace the kernel or scalar identities used by private-field export.
#[test]
fn private_usage_fields_keep_reserved_kernel_and_scalar_identity() {
    for (head, parameters, result) in [
        ("MVarCell", "a", "MVarCell a"),
        ("U64", "", "U64"),
        ("I128", "", "I128"),
        ("U128", "", "U128"),
        ("Char", "", "Char"),
    ] {
        let source = format!("-- | A source declaration cannot acquire a compiler-owned field identity.\nmodule Main where\n  -- | This mutable payload must never be classified as a kernel cell or raw scalar.\n  data {head} {parameters} where\n    -- | Refuse the declaration before any private field proof can be exported.\n    Counterfeit :: Bytes -> {result}\n  -- | No intrinsic or task executes in a source-identity refusal.\n  main :: Int\n  main = 0\n");
        let directory = ProjectEvidence(common::project(
            "usage_reserved_identity",
            &[("Main.cinch", &source)],
        ));
        let outcome = common::check(&directory, "Main.cinch");
        std::fs::write(directory.join("source-check.txt"), format!("{outcome:?}")).unwrap();
        let error = outcome.expect_err("source cannot redefine a sealed kernel or scalar head");
        assert!(error.contains("builtin"), "{head}: {error}");
        common::cleanup(&directory);
    }
}

/// Retain a complete manifest of genuine or deliberately contradictory test contracts.
fn manifest(directory: &Path, interfaces: &[Iface]) {
    let evidence = attempt(directory, "manifest");
    let folder = evidence.join("interfaces");
    std::fs::create_dir_all(&folder).unwrap();
    let mut rows = Vec::new();
    for interface in interfaces.iter().filter(|row| row.module != "Main") {
        let path = folder.join(format!("{}.cinchif", interface.module));
        std::fs::write(&path, interface.to_json()).unwrap();
        rows.push(serde_json::json!([
            interface.module,
            interface.iface_hash(),
            path
        ]));
    }
    let manifest = serde_json::json!({"schema":1,"interfaces":rows}).to_string();
    std::fs::write(evidence.join("interfaces.json"), &manifest).unwrap();
    std::fs::write(directory.join("interfaces.json"), manifest).unwrap();
}

/// Recompute only test dependency hashes so semantic contradictions reach the actual importer.
fn rehash(interfaces: &mut [Iface]) {
    for _ in 0..interfaces.len() + 1 {
        let hashes: BTreeMap<_, _> = interfaces
            .iter()
            .map(|row| (row.module.clone(), row.iface_hash()))
            .collect();
        let mut changed = false;
        for interface in interfaces.iter_mut() {
            for dependency in &mut interface.imports {
                changed |= dependency.iface_hash != hashes[&dependency.module];
                dependency.iface_hash = hashes[&dependency.module].clone();
            }
        }
        if !changed {
            break;
        }
    }
    cinch::interface::verify_chain(interfaces)
        .expect("test mutation keeps dependency hash closure");
}

/// Retain the exact compiler inputs shared by this target's independent consumer projects.
struct SharedNativeChecker {
    directory: PathBuf,
    executable: PathBuf,
    sha256: String,
}

/// Hash an executable without retaining its entire byte image in memory.
fn artifact_sha256(path: &Path) -> String {
    let mut file = std::fs::File::open(path).unwrap();
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let count = file.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    format!("{:x}", digest.finalize())
}

/// Build one actual Native importer and keep every consumer's mutable evidence separate.
fn native_checker() -> (ProjectEvidence, PathBuf) {
    static SHARED: OnceLock<SharedNativeChecker> = OnceLock::new();
    let shared = SHARED.get_or_init(|| {
    let temporary = common::project("mvar_contract_shared_compiler", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    // Move before compiling: generated manifests use the durable root from the outset.
    // The ordinary stale-project pruner must not remove a failed consumer's compiler inputs.
    let retained = temporary.with_file_name(format!("cinch_mvar_shared_{}", temporary.file_name().unwrap().to_string_lossy()));
    std::fs::rename(&temporary, &retained).unwrap();
    let directory = ProjectEvidence(retained);
    let outcome = common::build(&directory, "Main.cinch", "mvar-contract-checker");
    let diagnostic = match &outcome { Ok(path) => format!("built {}\n", path.display()), Err(error) => error.clone() };
    std::fs::write(directory.join("importer-build.txt"), diagnostic).unwrap();
    let executable = outcome.expect("build actual Native MVar importer");
    let sha256 = artifact_sha256(&executable);
    std::fs::write(directory.join("shared-compiler.json"), serde_json::json!({
        "executable": &executable, "sha256": &sha256,
        "scope": "One fresh actual compiler for this target; consumer sources and manifests remain independent.",
    }).to_string()).unwrap();
    SharedNativeChecker { directory: directory.0.clone(), executable, sha256 }
    });
    assert_eq!(
        artifact_sha256(&shared.executable),
        shared.sha256,
        "shared Native compiler input changed"
    );
    let directory = ProjectEvidence(common::project("mvar_contract_importer_cases", &[]));
    std::fs::write(
        directory.join("native-compiler.json"),
        serde_json::json!({
            "compiler_root": &shared.directory, "executable": &shared.executable,
            "sha256": &shared.sha256,
        })
        .to_string(),
    )
    .unwrap();
    (directory, shared.executable.clone())
}

/// Check one source-absent consumer and preserve exact diagnostics on a failed assertion.
fn consumer(
    directory: &Path,
    executable: &Path,
    imports: &str,
    declaration: &str,
) -> (i32, String) {
    let source = format!("-- | Consume only checked interfaces after the producer source is removed.\nmodule Main where\n  {imports}\n  -- | Exercise one exact effect, owner or capability boundary.\n  {declaration}\n  -- | Interface testing executes no synchronization operation.\n  main :: Int\n  main = 0\n");
    let evidence = attempt(directory, "source-absent");
    std::fs::write(evidence.join("consumer.source"), &source).unwrap();
    std::fs::copy(
        directory.join("interfaces.json"),
        evidence.join("interfaces.json"),
    )
    .unwrap();
    std::fs::write(directory.join("source.cinch"), source).unwrap();
    let _ = std::fs::remove_file(directory.join("imported-report.txt"));
    let outcome = std::process::Command::new(executable)
        .current_dir(directory)
        .output();
    if let Err(error) = &outcome {
        std::fs::write(evidence.join("spawn-error.txt"), error.to_string()).unwrap();
    }
    let output = outcome.expect("execute actual source-absent importer");
    let report = std::fs::read(directory.join("imported-report.txt")).unwrap_or_default();
    std::fs::write(evidence.join("imported-report.txt"), &report).unwrap();
    std::fs::write(evidence.join("stdout.txt"), &output.stdout).unwrap();
    std::fs::write(evidence.join("stderr.txt"), &output.stderr).unwrap();
    std::fs::write(
        evidence.join("status.json"),
        serde_json::json!({"code":output.status.code(), "executable":executable,"cwd":directory})
            .to_string(),
    )
    .unwrap();
    (
        output.status.code().unwrap_or(-1),
        format!(
            "{}{}",
            String::from_utf8_lossy(&report),
            String::from_utf8_lossy(&output.stderr)
        ),
    )
}

/// Admit genuine source-absent calls for every public operation, including qualified reexports.
#[test]
fn source_absent_mvar_operations_keep_complete_effects() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    manifest(&directory, &interfaces);
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    assert!(!directory.join("MVarOwner.cinch").exists());
    for (name, domain, result, effects, application) in [
        ("mvar-new", "I64", "Cells.MVar I64", "MVar", "value"),
        ("mvar-new-empty", "Unit", "Cells.MVar I64", "MVar", "value"),
        (
            "mvar-take",
            "Cells.MVar I64",
            "I64",
            "MVar, Diverge",
            "value",
        ),
        (
            "mvar-put",
            "Cells.MVar I64",
            "Unit",
            "MVar, Diverge",
            "value 7",
        ),
        (
            "mvar-read",
            "Cells.MVar I64",
            "I64",
            "MVar, Diverge",
            "value",
        ),
        (
            "mvar-swap",
            "Cells.MVar I64",
            "I64",
            "MVar, Diverge",
            "value 7",
        ),
        (
            "mvar-try-take",
            "Cells.MVar I64",
            "Option I64",
            "MVar",
            "value",
        ),
        ("mvar-try-put", "Cells.MVar I64", "Bool", "MVar", "value 7"),
        (
            "mvar-try-read",
            "Cells.MVar I64",
            "Option I64",
            "MVar",
            "value",
        ),
        ("mvar-is-empty", "Cells.MVar I64", "Bool", "MVar", "value"),
        (
            "mvar-take-or-cancel",
            "Cells.MVar I64",
            "Option I64",
            "MVar, Cancel r, Diverge",
            "value",
        ),
        (
            "mvar-put-or-cancel",
            "Cells.MVar I64",
            "Bool",
            "MVar, Cancel r, Diverge",
            "value 7",
        ),
        (
            "mvar-read-or-cancel",
            "Cells.MVar I64",
            "Option I64",
            "MVar, Cancel r, Diverge",
            "value",
        ),
        (
            "mvar-swap-or-cancel",
            "Cells.MVar I64",
            "Option I64",
            "MVar, Cancel r, Diverge",
            "value 7",
        ),
    ] {
        let prefix = if name.ends_with("-or-cancel") {
            "forall (r :: Region). using (io :: Io r) => "
        } else {
            ""
        };
        let declaration = format!("probe :: {prefix}{domain} ->{{{effects}}} {result}\n  probe value = Cells.{name} {application}");
        let (status, report) = consumer(
            &directory,
            &executable,
            "import MVarFacade as Cells",
            &declaration,
        );
        assert_eq!(status, 0, "{name}: {report}");
    }
    for declaration in [
        "bad :: Cells.MVar Nat -> Nat\n  bad value = Cells.mvar-read value",
        "bad :: forall (r :: Region) (s :: Region). using (io :: Io r) => Cells.MVar Nat ->{MVar, Cancel s, Diverge} Option Nat\n  bad value = Cells.mvar-read-or-cancel value",
        "bad :: Cells.MVar Nat ->{MVar, Cancel r, Diverge} Option Nat\n  bad value = Cells.mvar-read-or-cancel value",
        "bad :: forall (r :: Type). using (io :: Io r) => Cells.MVar Nat ->{MVar, Cancel r, Diverge} Option Nat\n  bad value = Cells.mvar-read-or-cancel value",
        "data MVar a where\n    -- | A local constructor cannot manufacture the imported nominal identity.\n    Fake :: a -> MVar a\n  -- | Same spelling is not defining-owner authority.\n  bad :: MVar Nat ->{MVar, Diverge} Nat\n  bad value = Cells.mvar-take value",
    ] {
        let (status, report) = consumer(&directory, &executable, "import MVarFacade as Cells", declaration);
        assert_eq!(status, 4, "wrong effect/role/owner/capability: {report}");
    }
    let (status, report) = consumer(
        &directory,
        &executable,
        "import UserMVar",
        "bad :: Unit ->{MVar} Unit\n  bad value = signal value",
    );
    assert_eq!(
        status, 4,
        "ordinary declaration-owned MVar is not a kernel contract: {report}"
    );
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// A canonical runtime spelling cannot override contradictory defining foreign metadata.
#[test]
fn source_absent_mvar_intrinsics_reject_contradictory_authority() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    for fault in [
        "effects",
        "abi",
        "kind",
        "operation",
        "region-role",
        "missing-blocking-effect",
    ] {
        let mut changed = interfaces.clone();
        let owner = changed
            .iter_mut()
            .find(|row| row.module == "KernelMVar")
            .unwrap();
        match fault {
            "effects" => {
                owner
                    .exports
                    .foreign
                    .iter_mut()
                    .find(|row| row.symbol == "cell-new")
                    .unwrap()
                    .effects = "IO".into()
            }
            "abi" => {
                owner
                    .exports
                    .foreign
                    .iter_mut()
                    .find(|row| row.symbol == "cell-new")
                    .unwrap()
                    .abi = "not-ccall".into()
            }
            "kind" => {
                owner
                    .exports
                    .foreign
                    .iter_mut()
                    .find(|row| row.symbol == "cell-new")
                    .unwrap()
                    .kind = "import".into()
            }
            "operation" => {
                owner
                    .exports
                    .foreign
                    .iter_mut()
                    .find(|row| row.symbol == "cell-new")
                    .unwrap()
                    .external = "cinch.runtime.mvarRead".into()
            }
            "region-role" => {
                let value = owner
                    .exports
                    .values
                    .iter_mut()
                    .find(|row| row.name == "cell-new")
                    .unwrap();
                let checked = value.resolved.as_mut().unwrap();
                assert!(checked.scheme.contains("a :: Type"));
                checked.scheme = checked.scheme.replace("a :: Type", "a :: Region");
            }
            "missing-blocking-effect" => {
                let value = owner
                    .exports
                    .values
                    .iter_mut()
                    .find(|row| row.name == "cell-take")
                    .unwrap();
                let checked = value.resolved.as_mut().unwrap();
                assert!(checked.scheme.contains("Diverge"));
                checked.scheme = checked
                    .scheme
                    .replace("Diverge, ", "")
                    .replace(", Diverge", "");
            }
            _ => unreachable!(),
        }
        rehash(&mut changed);
        manifest(&directory, &changed);
        let probe = if fault == "missing-blocking-effect" {
            "probe :: MVarCell Nat ->{MVar, Diverge} Nat\n  probe value = cell-take value"
        } else {
            "probe :: Nat ->{MVar} MVarCell Nat\n  probe value = cell-new value"
        };
        let (status, report) = consumer(&directory, &executable, "import KernelMVar", probe);
        assert!(
            status == 4 || status == 90,
            "{fault}: malformed authority was admitted: {status}: {report}"
        );
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// A source-owned operation with kernel spelling cannot justify an actual MVar primitive declaration.
#[test]
fn ordinary_user_mvar_effect_cannot_authorize_kernel_intrinsics() {
    let source = r#"-- | An ordinary effect cannot replace the kernel operation's actual checked row.
module Main where
  effect MVar where
    -- | This signal has a declaration-owned identity.
    signal :: Unit ->{MVar} Unit
  -- | The runtime intrinsic requires its own kernel effect identity.
  foreign import ccall "cinch.runtime.mvarNew" false-new :: forall a. a ->{MVar} MVarCell a
  -- | No runtime call is executed by the refusal control.
  main :: Int
  main = 0
"#;
    let directory = ProjectEvidence(common::project(
        "mvar_owned_effect_refusal",
        &[("Main.cinch", source)],
    ));
    let outcome = common::check(&directory, "Main.cinch");
    std::fs::write(directory.join("source-check.txt"), format!("{outcome:?}")).unwrap();
    let error = outcome.expect_err("ordinary MVar effect must not match kernel row");
    assert!(
        error.contains("effect") || error.contains("mismatch") || error.contains("intrinsic"),
        "{error}"
    );
    common::cleanup(&directory);
}

/// Require payload sharing evidence at task boundaries, without inventing fields for opaque imports.
#[test]
fn source_absent_mvar_handles_do_not_invent_payload_sharing() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    manifest(&directory, &interfaces);
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let mut sharing_outcomes = Vec::new();
    for (carrier, read, payload, expected) in [
        ("MVarCell", "cell-read", "Nat", 0),
        ("MVarCell", "cell-read", "Bytes", 4),
        // Genuine private-field facts must prove the public wrapper without revealing its constructor.
        ("Cells.MVar", "Cells.mvar-read", "Nat", 0),
        ("Cells.MVar", "Cells.mvar-read", "Bytes", 4),
    ] {
        let observe = if payload == "Bytes" {
            format!("bytes-len ({read} cell)")
        } else {
            format!("{read} cell")
        };
        let declaration = format!("work :: forall r. using (io :: Io r) => {carrier} {payload} ->{{Async r, Cancel r, Diverge, MVar}} Result LoadError (Nat, Nat)\n  work cell = withScope $ \\scope -> do\n    first <- spawn scope (\\() -> Ok ({observe}))\n    second <- spawn scope (\\() -> Ok ({observe}))\n    joinAllOrCancel scope first second");
        let (status, report) = consumer(
            &directory,
            &executable,
            "import KernelMVar\n  import MVarFacade as Cells",
            &declaration,
        );
        sharing_outcomes.push((format!("{carrier} {payload}"), expected, status, report));
    }
    // A single linear handle can still refer to a cell aliased outside this function.
    // Check Transfer independently: the two-task Share controls above cannot establish it.
    let mut transfer_outcomes = Vec::new();
    for (label, domain, observation, required) in [
        ("kernel Nat", "MVarCell Nat", "cell-is-empty cell", 0),
        ("kernel Bytes", "MVarCell Bytes", "cell-is-empty cell", 4),
        (
            "public Bytes",
            "Cells.MVar Bytes",
            "Cells.mvar-is-empty cell",
            4,
        ),
        (
            "public Nat",
            "Cells.MVar Nat",
            "Cells.mvar-is-empty cell",
            0,
        ),
        ("owned Bytes", "Bytes", "bytes-len cell == 0", 0),
    ] {
        let declaration = format!("work :: forall (r :: Region). using (io :: Io r) => {domain} %1 ->{{Async r, Cancel r, Diverge, MVar}} Result LoadError Bool\n  work cell = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok ({observation}))\n    joinAllOrCancel scope task");
        let (status, report) = consumer(
            &directory,
            &executable,
            "import KernelMVar\n  import MVarFacade as Cells",
            &declaration,
        );
        transfer_outcomes.push((label, required, status, report));
    }
    // Preserve both capture forms before asserting either availability or safety.
    for (label, required, status, report) in sharing_outcomes {
        assert_eq!(status, required, "shared capture {label}: {report}");
        if required == 4 {
            assert!(
                report.contains("shar") || report.contains("task") || report.contains("transfer"),
                "{report}"
            );
        }
    }
    for (label, required, status, report) in transfer_outcomes {
        assert_eq!(status, required, "single-task %1 capture {label}: {report}");
        if required == 4 {
            assert!(
                report.contains("shar") || report.contains("task") || report.contains("transfer"),
                "{report}"
            );
        }
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// Header-only imports retain structural usage facts independently of public method selection and order.
#[test]
fn source_absent_mvar_header_proofs_do_not_depend_on_method_imports() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    manifest(&directory, &interfaces);
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let mut outcomes = Vec::new();
    for owner in ["MVarOwner", "MVarFacade"] {
        for (payload, required) in [("Nat", 0), ("Bytes", 4)] {
            for usage in ["->", "%1 ->"] {
                let declarations = format!("-- | Observe a strict handle without importing any cell operation.\n  observe :: forall a. a -> Nat\n  observe value = 1\n  -- | The exact nominal header must suffice for both shared and one-shot task captures.\n  work :: forall (r :: Region). using (io :: Io r) => Cell {payload} {usage}{{Async r, Cancel r, Diverge}} Result LoadError Nat\n  work cell = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok (observe cell))\n    joinAllOrCancel scope task");
                let (status, report) = consumer(
                    &directory,
                    &executable,
                    &format!("import {owner} (MVar as Cell)"),
                    &declarations,
                );
                outcomes.push((
                    format!("{owner}/{payload}/{usage}"),
                    required,
                    status,
                    report,
                ));
            }
        }
    }
    // Private field evidence must not turn the representation constructor into a public source name.
    let (status, report) = consumer(
        &directory,
        &executable,
        "import MVarFacade as Cells",
        "forge :: MVarCell Nat -> Cells.MVar Nat\n  forge cell = Cells.MkMVar cell",
    );
    assert_ne!(
        status, 0,
        "private constructor became source-visible: {report}"
    );
    for (label, required, status, report) in outcomes {
        assert_eq!(status, required, "header-only {label}: {report}");
    }
    let (status, report) = consumer(&directory, &executable, "import HiddenUsage (Handle)",
        "-- | Check every private alternative, including the one not storing the apparent Nat payload.\n  observe :: forall a. a -> Nat\n  observe value = 1\n  work :: forall (r :: Region). using (io :: Io r) => Handle Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat\n  work value = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok (observe value))\n    joinAllOrCancel scope task");
    assert_eq!(
        status, 4,
        "a private Bytes alternative must participate in the complete proof: {report}"
    );
    let (status, report) = consumer(&directory, &executable, "import HiddenUsage (RegionHandle)",
        "-- | Reconstruct exact header roles and private borrow-field scope without creating a source constructor.\n  keep :: forall (r :: Region) a. RegionHandle r a -> RegionHandle r a\n  keep value = value");
    assert_eq!(
        status, 0,
        "explicit Region field proof must retain its owner and bound carrier roles: {report}"
    );
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// Change only the ownership syntax in every occurrence while preserving real source-produced owners.
fn change_usage_contract(value: &mut serde_json::Value, fault: &str) {
    match value {
        serde_json::Value::Object(row) => {
            if row.get("identity").and_then(|v| v.as_str()) == Some("MVarOwner.MVar")
                && row.contains_key("usage")
            {
                let usage = row.get_mut("usage").unwrap().as_array_mut().unwrap();
                assert_eq!(
                    usage.len(),
                    1,
                    "actual checked wrapper has exactly one private constructor"
                );
                match fault {
                    "callback" => usage[0]["fields"] = serde_json::json!(["(Nat -> Nat)"]),
                    "nested-forall" => usage[0]["fields"] = serde_json::json!(["forall a. a -> a"]),
                    "unknown-owner" => {
                        usage[0]["fields"] = serde_json::json!(["(Other.MVarCell a)"])
                    }
                    "wrong-result" => usage[0]["result"] = serde_json::json!("(Other.Hidden a)"),
                    "missing-result-argument" => {
                        usage[0]["result"] = serde_json::json!("MVarOwner.MVar")
                    }
                    _ => unreachable!(),
                }
            }
            for value in row.values_mut() {
                change_usage_contract(value, fault);
            }
        }
        serde_json::Value::Array(rows) => {
            for value in rows {
                change_usage_contract(value, fault);
            }
        }
        _ => {}
    }
}

/// Structurally valid metadata cannot supply task evidence through unsupported fields or a different result owner.
#[test]
fn source_absent_private_usage_proofs_reject_unsupported_carriers_and_results() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let declaration = "-- | The actual importer must validate the proof before any task can use it.\n  observe :: forall a. a -> Nat\n  observe value = 1\n  work :: forall (r :: Region). using (io :: Io r) => Cell Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat\n  work cell = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok (observe cell))\n    joinAllOrCancel scope task";
    let mut outcomes = Vec::new();
    for fault in [
        "callback",
        "nested-forall",
        "unknown-owner",
        "wrong-result",
        "missing-result-argument",
    ] {
        let mut changed: Vec<Iface> = interfaces
            .iter()
            .map(|interface| {
                let mut value: serde_json::Value =
                    serde_json::from_str(&interface.to_json()).unwrap();
                change_usage_contract(&mut value, fault);
                Iface::from_json(&value.to_string())
                    .expect("fault is structural JSON, not an envelope parse error")
            })
            .collect();
        rehash(&mut changed);
        manifest(&directory, &changed);
        let (status, report) = consumer(
            &directory,
            &executable,
            "import MVarFacade (MVar as Cell)",
            declaration,
        );
        outcomes.push((fault, status, report));
    }
    // Preserve all five independent semantic refusals before asserting them.
    for (fault, status, report) in outcomes {
        assert_eq!(
            status, 4,
            "unsupported private-field proof {fault}: {report}"
        );
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// Reexports cannot replace defining fields or alter the header roles that those fields quantify over.
#[test]
fn source_absent_private_usage_headers_reject_contradictory_defining_authority() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let mut outcomes = Vec::new();
    for fault in [
        "field-contradiction",
        "missing-constructor",
        "duplicate-constructor",
        "wrong-owner",
        "wrong-role",
    ] {
        let mut changed = interfaces.clone();
        let facade = changed
            .iter_mut()
            .find(|row| row.module == "MVarFacade")
            .unwrap();
        let proof = facade
            .exports
            .types
            .iter_mut()
            .find(|row| row.name == "MVar")
            .unwrap()
            .ownership
            .as_mut()
            .expect("genuine reexport retains the defining usage proof");
        match fault {
            "field-contradiction" => {
                proof.usage.as_mut().unwrap()[0].fields = vec!["(MVarCell Bytes)".into()]
            }
            "missing-constructor" => proof.usage.as_mut().unwrap().clear(),
            "duplicate-constructor" => {
                let row = proof.usage.as_ref().unwrap()[0].clone();
                proof.usage.as_mut().unwrap().push(row);
            }
            "wrong-owner" => proof.identity = "Other.Cell".into(),
            "wrong-role" => proof.parameters[0].kind = "Region".into(),
            _ => unreachable!(),
        }
        rehash(&mut changed);
        manifest(&directory, &changed);
        let (status, report) = consumer(&directory, &executable, "import MVarFacade (MVar as Cell)",
            "-- | Header admission must fail even without executing or importing a cell operation.\n  identity :: Cell Nat -> Cell Nat\n  identity value = value");
        outcomes.push((fault, status, report));
    }
    for (fault, status, report) in outcomes {
        assert_eq!(
            status, 90,
            "contradictory defining usage authority {fault}: {report}"
        );
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// Source-known constructors establish payload sharing without adding constraints to local operations.
#[test]
fn source_known_mvar_sharing_uses_real_private_fields() {
    let (directory, _) = producer();
    for (payload, accepted) in [("Nat", true), ("Bytes", false)] {
        let observe = if payload == "Bytes" {
            "bytes-len (mvar-read cell)"
        } else {
            "mvar-read cell"
        };
        let source = format!("-- | Source-known private fields preserve the shared handle's actual payload rule.\nmodule Use where\n  import MVarOwner\n  -- | Sharing a handle always requires its stored type to be shareable.\n  work :: forall r. using (io :: Io r) => MVar {payload} ->{{Async r, Cancel r, Diverge, MVar}} Result LoadError (Nat, Nat)\n  work cell = withScope $ \\scope -> do\n    first <- spawn scope (\\() -> Ok ({observe}))\n    second <- spawn scope (\\() -> Ok ({observe}))\n    joinAllOrCancel scope first second\n");
        let evidence = attempt(&directory, payload);
        std::fs::write(evidence.join("Use.source"), &source).unwrap();
        std::fs::write(directory.join("Use.cinch"), source).unwrap();
        let outcome = common::check(&directory, "Use.cinch");
        std::fs::write(evidence.join("source-check.txt"), format!("{outcome:?}")).unwrap();
        if accepted {
            outcome.expect("real MVarCell Nat field proves sharing");
        } else {
            let error = outcome.expect_err("MVarCell Bytes may have aliases outside the task");
            assert!(
                error.contains("shar") || error.contains("task") || error.contains("transfer"),
                "{error}"
            );
        }
    }
    common::cleanup(&directory);
}

/// Ordinary declarations can use kernel spellings without acquiring kernel effect identity.
#[test]
fn ordinary_kernel_spelling_effects_keep_their_own_handlers_and_forward_headers() {
    for name in [
        "MVar",
        "IO",
        "Mut",
        "Async",
        "Cancel",
        "Eval",
        "Foreign",
        "Global",
        "Blocking",
        "Diverge",
        "Fail",
        "Reentrant",
        "ThreadAffine",
        "Notice",
    ] {
        let source = format!(
            r#"-- | Keep ordinary operation dispatch independent from a kernel label's spelling.
module Main where
  -- | Header reservation must select the same declaration identity before its body is collected.
  before :: Unit ->{{{name}}} Int
  -- | A source effect remains legal even when a kernel label has the same spelling.
  effect {name} where
    -- | Return one value through this declaration's handler.
    signal :: Unit ->{{{name}}} Int
  -- | Check the forward signature's actual operation row.
  before unit = signal unit
  -- | Handle the ordinary effect without granting access to any kernel operation.
  main :: Int
  main = handle signal () with (handler {name} where {{ return value -> value; signal _ resume k -> k 42 }})
"#
        );
        let directory = ProjectEvidence(common::project(
            "declared_kernel_effect",
            &[("Main.cinch", &source)],
        ));
        let outcome = common::check_interfaces(&directory, "Main.cinch");
        std::fs::write(
            directory.join("source-check.txt"),
            match &outcome {
                Ok(_) => "accepted".to_string(),
                Err(error) => error.clone(),
            },
        )
        .unwrap();
        let interfaces = outcome.expect("ordinary effects with kernel spellings stay handleable");
        let main = interfaces.iter().find(|row| row.module == "Main").unwrap();
        let signal = main
            .exports
            .values
            .iter()
            .find(|row| row.name == "signal")
            .unwrap();
        let expected = if name == "Notice" {
            "Notice".to_string()
        } else {
            format!("Main.{name}")
        };
        assert!(
            signal.scheme.contains(&format!("{{{expected}}}")),
            "{name}: {}",
            signal.scheme
        );
        assert!(
            signal.resolved.is_none(),
            "ordinary declared operations do not acquire portable kernel authority"
        );
        if name == "MVar" {
            let outcome = common::build(&directory, "Main.cinch", "ordinary-mvar-handler");
            std::fs::write(directory.join("build-result.txt"), format!("{outcome:?}")).unwrap();
            let executable = outcome.expect("compile genuine ordinary MVar handler");
            for (label, heap, debug) in [("normal", "32M", "0"), ("poison", "256M", "1")] {
                let output = std::process::Command::new(&executable)
                    .current_dir(&*directory)
                    .env("CINCH_HEAP_LIMIT", heap)
                    .env("CINCH_STACK_SIZE", "8M")
                    .env("CINCH_RC_DEBUG", debug)
                    .env("CINCH_LEAK_CHECK", "1")
                    .output()
                    .unwrap();
                std::fs::write(
                    directory.join(format!("{label}-stdout.txt")),
                    &output.stdout,
                )
                .unwrap();
                std::fs::write(
                    directory.join(format!("{label}-stderr.txt")),
                    &output.stderr,
                )
                .unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(42),
                    "ordinary MVar handler {label}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
            }
        }
        common::cleanup(&directory);
    }
}

/// A custom termination label cannot discharge the canonical blocking intrinsic's divergence.
#[test]
fn ordinary_diverge_effect_cannot_authorize_blocking_mvar_intrinsics() {
    let source = r#"-- | Keep an ordinary termination-spelling effect distinct from actual divergence.
module Main where
  -- | This declaration supplies only its own operation table.
  effect Diverge where
    -- | Yield to a local handler without authorizing kernel blocking.
    signal :: Unit ->{Diverge} Unit
  -- | The real primitive needs canonical MVar and Diverge, not this declaration's row.
  foreign import ccall "cinch.runtime.mvarRead" false-read :: forall a. MVarCell a ->{MVar, Diverge} a
  -- | Never execute the invalid declaration.
  main :: Int
  main = 0
"#;
    let directory = ProjectEvidence(common::project(
        "declared_diverge_intrinsic",
        &[("Main.cinch", source)],
    ));
    let outcome = common::check(&directory, "Main.cinch");
    std::fs::write(directory.join("source-check.txt"), format!("{outcome:?}")).unwrap();
    let error = outcome.expect_err("ordinary Diverge cannot replace kernel divergence");
    assert!(
        error.contains("intrinsic") && error.contains("Diverge"),
        "{error}"
    );
    common::cleanup(&directory);
}

/// Removing an ordinary operation table must leave actual MVar mutation in the residual row.
#[test]
fn ordinary_mvar_handler_cannot_hide_a_real_kernel_operation() {
    let source = r#"-- | Declare the genuine primitive in a module with no ordinary MVar effect.
module Kernel where
  export make-cell
  -- | Only the canonical row authorizes this actual compiler intrinsic.
  foreign import ccall "cinch.runtime.mvarNew" new-cell :: forall a. a ->{MVar} MVarCell a
  -- | Allocate through a checked helper rather than a local operation table.
  make-cell :: I64 ->{MVar} MVarCell I64
  make-cell value = new-cell value

-- | An ordinary handler cannot remove a kernel effect by sharing its display name.
module Main where
  import Kernel
  -- | Keep this legal source effect separate from the kernel effect.
  effect MVar where
    -- | A handler can intercept this operation only.
    signal :: Unit ->{MVar} Int
  -- | A pure result must still refuse the unhandled kernel mutation.
  main :: Int
  main = handle (let cell = make-cell (0 :: I64) in 0) with (handler MVar where { return value -> value; signal _ resume k -> k 7 })
"#;
    let directory = ProjectEvidence(common::project(
        "declared_mvar_handler_refusal",
        &[("Main.cinch", source)],
    ));
    let outcome = common::check(&directory, "Main.cinch");
    std::fs::write(directory.join("source-check.txt"), format!("{outcome:?}")).unwrap();
    let error = outcome.expect_err("ordinary MVar handler must retain the kernel residual effect");
    assert!(
        error.contains("MVar") && (error.contains("effect") || error.contains("row")),
        "{error}"
    );
    common::cleanup(&directory);
}

/// Qualified imports preserve distinct declarations with equal kernel-spelling surface names.
#[test]
fn qualified_ordinary_mvar_handlers_keep_their_defining_owner() {
    let source = r#"-- | Define one independently handleable ordinary operation table.
module Left where
  export MVar, left-signal
  -- | This label belongs to Left, not the runtime kernel.
  effect MVar where
    -- | Return a left-owned value.
    left-signal :: Unit ->{MVar} Int

-- | Define a second table with the same surface spelling.
module Right where
  export MVar, right-signal
  -- | This label remains distinct from Left and the kernel.
  effect MVar where
    -- | Return a right-owned value.
    right-signal :: Unit ->{MVar} Int

-- | Select an actual declaration through an ordinary qualified import.
module Main where
  import Left (MVar as LeftMVar, left-signal)
  import Right (MVar as RightMVar, right-signal)
  -- | The matching handler can discharge only the left-owned operation.
  main :: Int
  main = handle Left.left-signal () with (handler Left.LeftMVar where { return value -> value; left-signal _ resume k -> k 43 })
"#;
    for (label, body, accepted) in [
        ("matching", source.to_string(), true),
        (
            "wrong-owner",
            source
                .replace("handler Left.LeftMVar", "handler Right.RightMVar")
                .replace("left-signal _ resume k", "right-signal _ resume k"),
            false,
        ),
        (
            "hidden-effect",
            source.replace("export MVar, left-signal", "export left-signal"),
            false,
        ),
    ] {
        let directory = ProjectEvidence(common::project(label, &[("Main.cinch", &body)]));
        let outcome = common::check(&directory, "Main.cinch");
        std::fs::write(directory.join("source-check.txt"), format!("{outcome:?}")).unwrap();
        if accepted {
            outcome.expect("matching qualified declaration handles its own operation");
        } else {
            let error =
                outcome.expect_err("another owner's handler cannot remove the left-owned effect");
            assert!(
                error.contains("effect")
                    || error.contains("row")
                    || label == "hidden-effect" && error.contains("no member"),
                "{error}"
            );
        }
        common::cleanup(&directory);
    }
}

/// An unresolved display cannot create a kernel MVar row, even inside a callback parameter.
#[test]
fn source_absent_unresolved_mvar_rows_do_not_invent_portable_authority() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let mut observations = Vec::new();
    for (label, spelling, declaration) in [
        (
            "legacy-explicit",
            "(Unit ->{MVar} Unit)",
            "probe :: Unit ->{MVar} Unit\n  probe value = signal value",
        ),
        (
            "latent-callback",
            "((Unit ->{MVar} Unit) -> Unit)",
            "probe :: (Unit ->{MVar} Unit) -> Unit\n  probe step = signal step",
        ),
    ] {
        let mut changed = interfaces.clone();
        let owner = changed
            .iter_mut()
            .find(|row| row.module == "UserMVar")
            .unwrap();
        let signal = owner
            .exports
            .values
            .iter_mut()
            .find(|row| row.name == "signal")
            .unwrap();
        assert!(signal.resolved.is_none());
        // The first spelling reproduces the preserved genuine pre-repair interface.
        // Coherent test hashes do not turn its absent defining contract into authority.
        signal.scheme = spelling.to_string();
        rehash(&mut changed);
        manifest(&directory, &changed);
        let (status, report) = consumer(&directory, &executable, "import UserMVar", declaration);
        observations.push((label, 4, status, report));
    }
    let mut legacy = interfaces;
    let rows = legacy
        .iter_mut()
        .find(|row| row.module == "MVarRows")
        .unwrap();
    let pure = rows
        .exports
        .values
        .iter_mut()
        .find(|row| row.name == "pure-unit")
        .unwrap();
    pure.resolved = None;
    rehash(&mut legacy);
    manifest(&directory, &legacy);
    let (status, report) = consumer(
        &directory,
        &executable,
        "import MVarRows",
        "probe :: Unit -> Unit\n  probe value = pure-unit value",
    );
    observations.push(("legacy-pure", 0, status, report));
    for (label, expected, status, report) in observations {
        assert_eq!(status, expected, "unresolved scheme {label}: {report}");
        if expected == 4 {
            assert!(
                report.contains("MVar") && report.contains("portable"),
                "{report}"
            );
        }
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}

/// Remove only optional private usage facts, preserving ordinary owner, role, constructor and effect contracts.
fn omit_optional_private_usage(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(row) => {
            if row.contains_key("identity") && row.contains_key("parameters") {
                if row.contains_key("constructors") { row.remove("usage"); }
                if row.contains_key("ctors") { row.remove("ownership"); }
            }
            for value in row.values_mut() { omit_optional_private_usage(value); }
        }
        serde_json::Value::Array(rows) => for value in rows { omit_optional_private_usage(value); },
        _ => {},
    }
}

/// Legacy interfaces cannot regain mutable-cell transfer safety when their optional private facts are unavailable.
#[test]
fn source_absent_legacy_private_fields_remain_conservative() {
    let (source, interfaces) = producer();
    let (directory, executable) = native_checker();
    std::fs::remove_file(source.join("Producer.cinch")).unwrap();
    let mut legacy: Vec<Iface> = interfaces.iter().map(|interface| {
        let mut value: serde_json::Value = serde_json::from_str(&interface.to_json()).unwrap();
        omit_optional_private_usage(&mut value);
        Iface::from_json(&value.to_string()).expect("the optional keys may be absent in a valid legacy contract")
    }).collect();
    rehash(&mut legacy);
    let mut outcomes = Vec::new();
    for (label, contracts, has_proof) in [("genuine", &interfaces, true), ("legacy", &legacy, false)] {
        manifest(&directory, contracts);
        for owner in ["MVarOwner", "MVarFacade"] {
            for payload in ["Nat", "Bytes"] {
                for usage in ["->", "%1 ->"] {
                    let required = if has_proof && payload == "Nat" { 0 } else { 4 };
                    let declarations = format!("-- | Observe a strict handle without importing methods or its private constructor.\n  observe :: forall a. a -> Nat\n  observe value = 1\n  -- | A linear handle does not imply exclusive access to the cell's stored buffer.\n  work :: forall (r :: Region). using (io :: Io r) => Cell {payload} {usage}{{Async r, Cancel r, Diverge}} Result LoadError Nat\n  work cell = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok (observe cell))\n    joinAllOrCancel scope task");
                    let (status, report) = consumer(&directory, &executable,
                        &format!("import {owner} (MVar as Cell)"), &declarations);
                    outcomes.push((format!("{label}/header/{owner}/{payload}/{usage}"), required, status, report));
                }
            }
        }
        // A result-only nominal mention must recover abstract CoreData rather than reach the unknown-type fallback.
        for payload in ["Nat", "Bytes"] {
            let required = if has_proof && payload == "Nat" { 0 } else { 4 };
            let declarations = format!("-- | Keep the public wrapper type out of the client's source scope.\n  observe :: forall a. a -> Nat\n  observe value = 1\n  -- | Derive the handle from a checked imported result and still enforce its actual stored payload.\n  work :: forall (r :: Region). using (io :: Io r) => {payload} %1 ->{{Async r, Cancel r, Diverge, MVar}} Result LoadError Nat\n  work input = do\n    cell <- allocate input\n    withScope $ \\scope -> do\n      task <- spawn scope (\\() -> Ok (observe cell))\n      joinAllOrCancel scope task");
            let (status, report) = consumer(&directory, &executable,
                "import MVarFacade (mvar-new as allocate)", &declarations);
            outcomes.push((format!("{label}/result-only/{payload}"), required, status, report));
        }
        let (status, report) = consumer(&directory, &executable, "",
            "-- | Ordinary owned Bytes transfer remains distinct from moving one alias to a shared cell.\n  work :: forall (r :: Region). using (io :: Io r) => Bytes %1 ->{Async r, Cancel r, Diverge} Result LoadError Nat\n  work value = withScope $ \\scope -> do\n    task <- spawn scope (\\() -> Ok (bytes-len value))\n    joinAllOrCancel scope task");
        outcomes.push((format!("{label}/owned-Bytes"), 0, status, report));
    }
    // Preserve every independent import and verdict before asserting either availability or refusal.
    for (label, required, status, report) in outcomes {
        assert_eq!(status, required, "optional private-field scope {label}: {report}");
    }
    common::cleanup(&directory);
    common::cleanup(&source);
}
