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
    assert_eq!(output.status.code(), Some(42), "{}", String::from_utf8_lossy(&output.stderr));
    std::fs::remove_dir_all(&dir).ok();
}
