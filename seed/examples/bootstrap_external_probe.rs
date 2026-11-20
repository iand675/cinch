//! Exercise the portable harness with an actual compiled Cinch frontend.
#[path = "../tests/common/mod.rs"]
mod common;
use cinch::driver::Mode;
use std::path::PathBuf;

fn main() {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    assert_eq!(args.len(), 3, "compiler, checked base manifest, fresh project directory");
    let compiler = PathBuf::from(&args[0]).canonicalize().unwrap();
    let interfaces = PathBuf::from(&args[1]).canonicalize().unwrap();
    let project = PathBuf::from(&args[2]);
    std::fs::create_dir(&project).expect("fresh external frontend probe project");
    std::env::set_var("CINCH_TEST_COMPILER", &compiler);
    std::env::set_var("CINCH_TEST_INTERFACES", &interfaces);
    std::fs::write(project.join("A with\na newline.cinch"), "module A where\n  answer :: Int\n  answer = 37\n").unwrap();
    std::fs::write(project.join("Main.cinch"), "module Main where\n  import A\n  main :: Int\n  main = answer\n").unwrap();
    assert_eq!(common::drive(Mode::Check, &project, "Main.cinch", None).unwrap(), 0);
    let built = common::build(&project, "Main.cinch", "separately-built").unwrap();
    assert!(built.is_file());
    assert_eq!(std::process::Command::new(&built).status().unwrap().code(), Some(37));
    assert_eq!(common::run(&project, "Main.cinch"), 37);
    std::fs::write(project.join("Main.cinch"), "module Main where\n  main :: Int\n  main = missing\n").unwrap();
    let error = common::check(&project, "Main.cinch").unwrap_err();
    assert!(error.contains("missing"), "{error}");
    let count = std::fs::read_dir(project.join(".cinch-self-tests")).unwrap().count();
    assert_eq!(count, 4, "each actual external compilation must retain its invocation");
    std::fs::write(project.join("probe-result.txt"), "check: 0\nbuild: 0\nseparate-artifact: 37\nrun: 37\ninvalid-check: refused\nexternal-invocations: 4\n").unwrap();
}
