//! Prepared CRC-32 checksums and version-one binary checkpoints.
mod common;
use cinch::driver::Mode;

#[test]
fn prepared_crc32_checkpoint_and_stream() {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Crc32", include_str!("../prelude/Text/Crc32.cinch")),
        ("Effect.Crc32", include_str!("../prelude/Effect/Crc32.cinch")),
        ("Effect.Stream", include_str!("../prelude/Effect/Stream.cinch")),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let source = r#"module Main where
  import Text.Crc32
  import Effect.Crc32
  import Effect.Stream

  polynomial-case :: U32 -> U32 -> Bool
  polynomial-case poly expected =
    let table = crc32-table poly in
    let bytes = bytes-from-u8s [49,50,51,52,53,54,55,56,57] in
    let state = crc32-state-update (crc32-state table) bytes in
    let prefix = bytes-from-u8s [17,23] in
    let digest = crc32-state-append prefix state in
    let checkpoint = crc32-state-append-save prefix state in
    let bounds = case crc32-table-entry table 256 of None -> True; Some _ -> False in
    let entry = case crc32-table-entry table 128 of Some word -> eq-u32 word poly; None -> False in
    bounds && entry && eq-u32 (crc32-table-sum table bytes) expected &&
      eq-u32 (crc32-table-update table expected (bytes-new 0)) expected &&
      bytes-len digest == 6 && bytes-len checkpoint == 14 && bytes-len prefix == 2

  rejects-invalid :: Crc32Table -> Bytes -> Bool
  rejects-invalid table saved =
    let bad-id = case crc32-state-load table (bytes-new 0) of
          Err InvalidCrc32StateIdentifier -> True
          _ -> False in
    let bad-length = case crc32-state-load table (bytes-from-u8s [99,114,99,1]) of
          Err (InvalidCrc32StateSize 12 4) -> True
          _ -> False in
    let mismatch = case crc32-state-load (crc32-table 0x82F63B78) saved of
          Err (Crc32StateTableMismatch expected actual) -> ne-u32 expected actual
          _ -> False in
    bad-id && bad-length && mismatch

  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let table = crc32-table 0xEDB88320 in
    let start = crc32-state table in
    let first = crc32-state-update start (bytes-from-u8s [49,50,51,52]) in
    case crc32-copy 2 (reader-from-bytes (bytes-from-u8s [53,54,55,56,57])) first of
      (Ok 5, (reader, done)) -> case crc32-state-load table (crc32-state-save done) of
        Ok restored -> if eq-u32 (crc32-state-word restored) 0xCBF43926 &&
          eq-i64 (bytes-compare (crc32-state-save restored) (bytes-from-u8s [99,114,99,1,202,135,145,77,203,244,57,38])) 0 &&
          eq-u32 (crc32-state-word (crc32-state-reset done)) 0 &&
          rejects-invalid table (crc32-state-save done) &&
          polynomial-case 0x82F63B78 0xE3069283 && polynomial-case 0xEB31D82E 0x2D3DD0AE &&
          polynomial-case 0 0xFFFFFFFF &&
          eq-u32 (crc32-state-word first) 0x9BE3E0A3 then 0 else 1
        Err error -> 2
      _ -> 3
"#;
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let exe = directory.path().join("consumer");
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
        Some(exe.clone()),
        &profile,
    )
    .unwrap();
    for debug in ["0", "1"] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", debug)
            .env("CINCH_LEAK_CHECK", "1")
            .output()
            .unwrap();
        let diagnostics = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{diagnostics}");
        assert!(diagnostics.contains("leaked=0"), "{diagnostics}");
    }
}
