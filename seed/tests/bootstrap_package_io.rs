//! Package I/O keeps the failing path and operation in diagnostics, while
//! successful mappings remain readable after the descriptor is closed.
mod common;

#[test]
fn package_io_reports_missing_and_empty_files_and_keeps_mapped_bytes_alive() {
    let source = r#"module Main where
  import Cinch.Package (read-package-source)
  import Data.Bytes (bytes-len)

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    case read-package-source "missing.cpk" of
      Ok bytes -> 1
      Err message -> if string-compare message "cannot open interface package missing.cpk: SysNotFound" /= 0 then 2
        else case read-package-source "empty.cpk" of
          Ok bytes -> 3
          Err message -> if string-compare message "interface package empty.cpk: invalid interface package length: 0" /= 0 then 4
            else case read-package-source "payload.cpk" of
              Err message -> 5
              Ok bytes -> if bytes-len bytes == 3 then 0 else 6
"#;
    let dir = common::project("package_io_diagnostics", &[
        ("Cinch.Package.cinch", include_str!("../../compiler/Cinch.Package.cinch")),
        ("Main.cinch", source),
    ]);
    std::fs::write(dir.join("empty.cpk"), b"").unwrap();
    std::fs::write(dir.join("payload.cpk"), b"abc").unwrap();
    let executable = common::build_executable(&dir, "Main.cinch", "package-io")
        .expect("compile the actual package reader");
    let output = std::process::Command::new(executable).current_dir(&dir).output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}\nproject: {}",
               String::from_utf8_lossy(&output.stderr), dir.display());
}
