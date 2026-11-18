use std::{path::PathBuf, process::Command};

fn main() {
    let grammar = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let scanner = grammar.join("src/scanner.c");
    let generated = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo OUT_DIR"))
        .join("tree-sitter-cinch");
    println!(
        "cargo:rerun-if-changed={}",
        grammar.join("grammar.js").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        grammar.join("tree-sitter.json").display()
    );
    println!("cargo:rerun-if-changed={}", scanner.display());
    println!("cargo:rerun-if-env-changed=TREE_SITTER");
    println!("cargo:rerun-if-env-changed=TREE_SITTER_JS_RUNTIME");

    let generator = std::env::var_os("TREE_SITTER").unwrap_or_else(|| "tree-sitter".into());
    let status = Command::new(&generator)
        .current_dir(&grammar)
        .arg("generate")
        .arg("--output")
        .arg(&generated)
        .status()
        .unwrap_or_else(|error| {
            panic!("cannot run {generator:?} to generate the Cinch parser: {error}")
        });
    assert!(
        status.success(),
        "{generator:?} failed to generate the Cinch parser"
    );

    cc::Build::new()
        .include(&generated)
        .file(generated.join("parser.c"))
        .file(scanner)
        .flag_if_supported("-std=c11")
        .compile("tree-sitter-cinch");
}
