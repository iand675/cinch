//! Buffered raw events and managed continuation proxies; full reader/writer/scanner coverage grows here.
mod common;
use cinch::driver::Mode;

/// Copy only source-distributed dependencies, preserving the compiler's embedded base family.
fn project() -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Effect.Buffered.Core",
            include_str!("../prelude/Effect/Buffered/Core.cinch"),
        ),
        (
            "Effect.Buffered.Internal.Cell",
            include_str!("../prelude/Effect/Buffered/Internal/Cell.cinch"),
        ),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    directory
}

/// Retained callbacks and independently returned handles observe one authoritative advancing continuation.
#[test]
fn buffered_core_signed_events_retained_callbacks_and_reclamation() {
    let directory = project();
    std::fs::write(
        directory.path().join("Main.cinch"),
        include_str!("fixtures/buffered-core-controls.cinch"),
    )
    .unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .unwrap();
    for poisoned in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "poisoned={poisoned}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// The private managed cell requires I/O and never exposes its array constructor to ordinary clients.
#[test]
fn buffered_continuation_cells_require_io_and_hide_storage() {
    let accepted = project();
    std::fs::write(
        accepted.path().join("Main.cinch"),
        "-- | Positive controls for effectful cell use.\nmodule Main where\n  import Effect.Buffered.Internal.Cell\n  import Data.Nat\n  -- | Allocate with the required region effect.\n  good-new :: forall r. Nat ->{IO r} IoCell r Nat\n  good-new value = cell-new value\n  -- | Read with the required region effect.\n  good-read :: forall r. IoCell r Nat ->{IO r} Nat\n  good-read cell = cell-read cell\n  -- | Write with the required region effect.\n  good-write :: forall r. IoCell r Nat ->{IO r} Unit\n  good-write cell = cell-write cell 0\n  -- | Typecheck-only sentinel.\n  main :: Int\n  main = 0\n",
    )
    .unwrap();
    common::drive_with_profile(
        Mode::Check,
        cinch::check::StageMode::Refresh,
        accepted.path(),
        "Main.cinch",
        None,
        &cinch::core::TargetProfile::native(),
    )
    .unwrap();
    for declaration in [
        "bad :: forall r. Nat -> IoCell r Nat\n  bad value = cell-new value",
        "bad :: forall r. IoCell r Nat -> Nat\n  bad cell = cell-read cell",
        "bad :: forall r. IoCell r Nat -> Unit\n  bad cell = cell-write cell 0",
        "bad :: forall r. IoCell r Nat -> Array Nat\n  bad cell = case cell of StoredCell storage -> storage",
    ] {
        let directory = project();
        let source = format!(
            "-- | Deliberate rejection of a missing I/O effect.\nmodule Main where\n  import Effect.Buffered.Internal.Cell\n  import Data.Nat\n  -- | This intentionally invalid definition omits its required I/O effect.\n  {declaration}\n  -- | Typecheck-only sentinel.\n  main :: Int\n  main = 0\n"
        );
        std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
        let result = common::drive_with_profile(
            Mode::Check,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            None,
            &cinch::core::TargetProfile::native(),
        );
        assert!(result.is_err(), "invalid cell access unexpectedly admitted");
    }
}

/// Every owned declaration and constructor uses canonical documentation independent of another language.
#[test]
fn buffered_core_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Effect.Buffered.Core",
            include_str!("../prelude/Effect/Buffered/Core.cinch"),
        ),
        (
            "Effect.Buffered.Internal.Cell",
            include_str!("../prelude/Effect/Buffered/Internal/Cell.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(parsed.diagnostics.is_empty());
        assert!(parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration.doc.as_ref().unwrap();
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}
