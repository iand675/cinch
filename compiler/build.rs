use std::path::PathBuf;

fn main() {
    if std::env::var_os("CARGO_FEATURE_JIT").is_some() {
        link_llvm();
    }
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

/// Resolve the LLVM C API used by the optional embedded ORC backend.
/// `CINCH_LLVM_CONFIG` selects a versioned `llvm-config`; the default
/// follows PATH. The JIT links the shared LLVM library, not a second
/// copy of the toolchain.
fn link_llvm() {
    println!("cargo:rerun-if-env-changed=CINCH_LLVM_CONFIG");
    let config = std::env::var("CINCH_LLVM_CONFIG").unwrap_or_else(|_| "llvm-config".into());
    let output = std::process::Command::new(&config)
        .args(["--libs", "--ldflags"])
        .output()
        .unwrap_or_else(|e| panic!("jit feature: failed to run {config}: {e}"));
    if !output.status.success() {
        panic!("jit feature: {config} --libs --ldflags failed");
    }
    let text = String::from_utf8_lossy(&output.stdout);
    for token in text.split_whitespace() {
        if let Some(lib) = token.strip_prefix("-l") {
            println!("cargo:rustc-link-lib=dylib={lib}");
        } else if let Some(path) = token.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        }
    }
}
