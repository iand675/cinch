use std::path::PathBuf;

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let grammar = manifest.join("../tooling/tree-sitter-cinch/src");
    let parser = grammar.join("parser.c");
    let scanner = grammar.join("scanner.c");

    println!("cargo:rerun-if-changed={}", parser.display());
    println!("cargo:rerun-if-changed={}", scanner.display());
    println!("cargo:rerun-if-changed={}", grammar.join("tree_sitter/parser.h").display());

    cc::Build::new()
        .include(&grammar)
        .file(parser)
        .file(scanner)
        .flag_if_supported("-std=c11")
        .compile("tree-sitter-cinch");
}
