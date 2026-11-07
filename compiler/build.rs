use std::{path::PathBuf, process::Command};

fn main() {
    if std::env::var_os("CARGO_FEATURE_JIT").is_some() {
        link_llvm();
    }
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let grammar = manifest.join("../tooling/tree-sitter-cinch");
    let scanner = grammar.join("src/scanner.c");
    let generated = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo OUT_DIR"))
        .join("tree-sitter-cinch");
    println!("cargo:rerun-if-changed={}", grammar.join("grammar.js").display());
    println!("cargo:rerun-if-changed={}", grammar.join("tree-sitter.json").display());
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
        .unwrap_or_else(|error| panic!("cannot run {generator:?} to generate the Cinch parser: {error}"));
    assert!(status.success(), "{generator:?} failed to generate the Cinch parser");

    cc::Build::new()
        .include(&generated)
        .file(generated.join("parser.c"))
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
    let output = Command::new(&config)
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
