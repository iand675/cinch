//! Declaration-owned effects retain their defining identity through handlers and module visibility.
mod common;

use std::path::{Path, PathBuf};

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
