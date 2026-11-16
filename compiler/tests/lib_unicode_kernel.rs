//! Independent scalar tables, custom rules and canonical documentation for the Unicode kernel.
mod common;

use cinch::driver::Mode;

/// Keep the actual kernel, generated tables and checked machine-word helpers beside the consumer.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Data.Unicode.Kernel", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode.Tables", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build, cinch::check::StageMode::Refresh, directory.path(),
        "Main.cinch", Some(executable.clone()), &profile,
    ).unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output().unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}: {}\n{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Supplementary case mappings, fold cycles and scalar-invalid numeric inputs stay distinct.
#[test]
fn unicode_kernel_scalar_and_custom_case_contracts() {
    run(r#"-- | Unicode kernel scalar and custom-case integration.
module Main where
  import Data.Unicode.Kernel
  import Data.Char
  import Data.List
  import Data.Machine
  import Data.Option
  import Data.Result
  -- | Validate a private custom rule and its exact observation.
  custom :: Unit -> Bool
  custom unit = case unicode-special-case [MkUnicodeCaseRange 65 65 0 0 0] of
    Err problem -> False
    Ok rules -> unicode-special-to-lower rules 'A' == 'A'
      && unicode-special-to-lower rules 'B' == 'b'
      && unicode-special-affects-ascii rules
      && unicode-special-case-count rules == 1
  -- | Invalid scalar mappings must be refused before owned table construction.
  refusal :: Unit -> Bool
  refusal unit = case unicode-special-case [MkUnicodeCaseRange 55296 55296 0 0 0] of
    Err problem -> True
    Ok rules -> False
  -- | Cover exact fold cycles, supplementary planes, invalid codes and preset overrides.
  main :: Int
  main = if unicode-to-upper '\u{10428}' == '\u{10400}'
    && unicode-to-lower '\u{1E900}' == '\u{1E922}'
    && unicode-is-letter '\u{16EA0}' && unicode-is-space '\u{202F}'
    && unicode-equal-fold 'K' '\u{212A}' && unicode-equal-fold '\u{3A3}' '\u{3C2}'
    && not (unicode-equal-fold 'I' '\u{131}')
    && not (unicode-is-scalar-code 55296) && not (unicode-is-scalar-code (-1))
    && unicode-to-lower-code (-1) == (-1) && unicode-simple-fold-code 1114112 == 1114112
    && unicode-to-code 3 65 == 65533 && unicode-equal-fold-code (-1) (-1)
    && unicode-special-to-lower unicode-turkish-case 'I' == '\u{131}'
    && unicode-special-to-upper unicode-azeri-case 'i' == '\u{130}'
    && unicode-case-range-count == 336 && custom () && refusal () then 0 else 1
"#);
}

/// The source generator documents every numeric selector, constructor and implementation helper.
#[test]
fn unicode_kernel_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Data.Unicode.Kernel", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode.Tables", include_str!("../prelude/Data/Unicode/Tables.cinch")),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(parsed.diagnostics.is_empty(), "{name}: {:?}", parsed.diagnostics);
        assert!(parsed.orphans.is_empty(), "{name}: {:?}", parsed.orphans);
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration.doc.as_ref().unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}
