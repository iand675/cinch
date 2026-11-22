#![cfg(feature = "jit")]

use std::process::Command;

#[test]
fn cinch_jit_runs_a_checked_program() {
    let dir = std::env::temp_dir().join(format!("cinch-jit-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("temp directory");
    let main = dir.join("Main.cinch");
    std::fs::write(
        &main,
        "module Main where\n  data Pair where\n    Pair :: Int -> Int -> Pair\n  main :: Int\n  main = case Pair 20 22 of\n    Pair x y -> x + y\n",
    )
    .expect("Main.cinch");
    let output = Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("jit")
        .arg(&main)
        .output()
        .expect("run cinch jit");
    assert_eq!(
        output.status.code(),
        Some(42),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[test]
fn native_jit_runs_and_recovers_after_an_edit() {
    use std::fs;
    use std::io::{BufRead, BufReader, Write};
    use std::process::Stdio;

    let Some(stage) = std::env::var_os("CINCH_TEST_NATIVE_STAGE") else {
        eprintln!("skipping staged native JIT test: CINCH_TEST_NATIVE_STAGE is unset");
        return;
    };
    let dir = tempfile::tempdir().unwrap();
    let source = dir.path().join("Main.cinch");
    let mut command = Command::new(env!("CARGO_BIN_EXE_cinch"));
    command.args(["jit", "--native-stage"]).arg(stage);
    if let Some(runtime) = std::env::var_os("CINCH_TEST_NATIVE_RUNTIME") {
        command.arg("--runtime").arg(runtime);
    }

    let program = |status| {
        format!(
            r#"module Main where
  import Data.List
  import Data.Result
  import Effect.Process
  check-word :: String -> Int
  check-word word = if word == "two words" then {status} else 98
  main :: Unit ->{{IO r}} Int
  main _ = with (io = newIO ()) in case process-args () of
    Ok (Cons first (Cons second Nil)) ->
      if second == "--literal" then
        case map check-word (Cons first Nil) of
          Cons result Nil -> result
          other -> 97
      else 98
    other -> 99
"#
        )
    };
    fs::write(&source, program(17)).unwrap();
    let output = Command::new(command.get_program())
        .args(command.get_args())
        .arg(&source)
        .args(["--", "two words", "--literal"])
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(17),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mut session = command
        .arg("--serve")
        .arg(&source)
        .args(["--", "two words", "--literal"])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut output = BufReader::new(session.stderr.take().unwrap());
    let mut input = session.stdin.take().unwrap();
    let mut expect = |prefix: &str| {
        let mut line = String::new();
        let mut preceding = String::new();
        loop {
            assert_ne!(output.read_line(&mut line).unwrap(), 0, "{preceding}");
            if line.starts_with(prefix) {
                return (line, preceding);
            }
            preceding.push_str(&line);
            line.clear();
        }
    };
    assert_eq!(expect("cinch jit: ready").0, "cinch jit: ready\n");
    for _ in 0..2 {
        input.write_all(b"\n").unwrap();
        assert_eq!(expect("cinch jit: status ").0, "cinch jit: status 17\n");
    }
    fs::write(
        &source,
        "module Main where\n  main :: Int\n  main = missing-name\n",
    )
    .unwrap();
    input.write_all(b"\n").unwrap();
    let (failure, diagnostic) = expect("cinch jit: status ");
    assert_eq!(failure, "cinch jit: status 4\n", "{diagnostic}");
    assert!(diagnostic.contains("missing-name"), "{diagnostic}");
    fs::write(&source, program(29)).unwrap();
    input.write_all(b"\n").unwrap();
    assert_eq!(expect("cinch jit: status ").0, "cinch jit: status 29\n");
    drop(input);
    assert!(session.wait().unwrap().success());
}
