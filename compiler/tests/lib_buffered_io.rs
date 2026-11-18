//! Buffered raw events, independent operation traces and ownership controls.
//! Arbitrary cross-retained callback cycles remain an explicit runtime review gate.
mod common;
use cinch::driver::Mode;

/// Exact source-distributed closure; embedded base-family definitions are never shadowed.
const SOURCES: [(&str, &str); 14] = [
    (
        "Effect.Buffered",
        include_str!("../prelude/Effect/Buffered.cinch"),
    ),
    (
        "Effect.Buffered.Core",
        include_str!("../prelude/Effect/Buffered/Core.cinch"),
    ),
    (
        "Effect.Buffered.Internal.Cell",
        include_str!("../prelude/Effect/Buffered/Internal/Cell.cinch"),
    ),
    (
        "Effect.Buffered.Internal.Scan",
        include_str!("../prelude/Effect/Buffered/Internal/Scan.cinch"),
    ),
    (
        "Effect.Buffered.Reader",
        include_str!("../prelude/Effect/Buffered/Reader.cinch"),
    ),
    (
        "Effect.Buffered.Writer",
        include_str!("../prelude/Effect/Buffered/Writer.cinch"),
    ),
    (
        "Effect.Scanner",
        include_str!("../prelude/Effect/Scanner.cinch"),
    ),
    (
        "Effect.Scanner.Split",
        include_str!("../prelude/Effect/Scanner/Split.cinch"),
    ),
    (
        "Effect.Stream",
        include_str!("../prelude/Effect/Stream.cinch"),
    ),
    (
        "Data.Bytes.Build",
        include_str!("../prelude/Data/Bytes/Build.cinch"),
    ),
    (
        "Data.Bytes.Utf8",
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
    ),
    (
        "Data.Bytes.Word",
        include_str!("../prelude/Data/Bytes/Word.cinch"),
    ),
    (
        "Data.Unicode.Kernel",
        include_str!("../prelude/Data/Unicode/Kernel.cinch"),
    ),
    (
        "Data.Unicode.Tables",
        include_str!("../prelude/Data/Unicode/Tables.cinch"),
    ),
];

/// Copy only source-distributed dependencies, preserving the compiler's embedded base family.
fn project() -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in SOURCES {
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
fn buffered_family_declarations_have_canonical_documentation() {
    for (name, source) in SOURCES.into_iter().filter(|(name, _)| {
        name.starts_with("Effect.Buffered") || name.starts_with("Effect.Scanner")
    }) {
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

/// Execute an independent maintained consumer with the same fixed ownership and memory profiles as the core gate.
fn run_family_fixture(source: &str) {
    let directory = project();
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
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
        assert!(String::from_utf8_lossy(&output.stderr).contains("live=0 immortal=0 leaked=0"));
    }
}

/// Independent reader histories preserve bytes, decoded widths, unread permissions and actual transport request sizes.
#[test]
fn buffered_reader_independent_operation_histories() {
    run_family_fixture(include_str!("fixtures/buffered-reader-traces.cinch"));
}

/// Independent writer histories preserve accepted prefixes, sticky failures, reservations and exact sink offers.
#[test]
fn buffered_writer_independent_operation_histories() {
    run_family_fixture(include_str!("fixtures/buffered-writer-traces.cinch"));
}

/// Scanner histories preserve absence versus empty tokens, final states and malformed split progress.
#[test]
fn buffered_scanner_independent_operation_histories() {
    run_family_fixture(include_str!("fixtures/buffered-scanner-traces.cinch"));
}

/// Every built-in splitter processes independent malformed, Unicode and delimiter boundary observations.
#[test]
fn buffered_splitters_independent_byte_and_unicode_observations() {
    run_family_fixture(include_str!("fixtures/buffered-split-traces.cinch"));
}

/// Public layered pipelines retain fresh output owners and distinguish one-layer flush from explicit deep finish.
#[test]
fn buffered_layered_pipeline_ownership_and_flush_routes() {
    run_family_fixture(include_str!("fixtures/buffered-layer-controls.cinch"));
}
