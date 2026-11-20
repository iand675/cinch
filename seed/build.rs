use std::{path::PathBuf, process::Command};

fn main() {
    if std::env::var_os("CARGO_FEATURE_JIT").is_some() {
        link_llvm();
    }
    build_identity();
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

/// `CINCH_BUILD_ID`: a hash of every source this compiler is built from
/// (its Rust sources, the embedded prelude, and the block-layout constants
/// it shares with the runtime). Two builds with one `CARGO_PKG_VERSION` can
/// render the same input differently, so the package store keys on this
/// too (`crate::store::package_key_for`): an entry under a key is then a
/// function of the compiler that wrote it, and a compile can keep an intact
/// entry without re-rendering it to compare.
fn build_identity() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let roots = [manifest.join("src"), manifest.join("prelude"), manifest.join("runtime/src/block_kinds.rs")];
    let mut files: Vec<PathBuf> = Vec::new();
    for root in &roots {
        println!("cargo:rerun-if-changed={}", root.display());
        collect(root, &mut files);
    }
    files.sort();
    // FNV-1a 64: stable across toolchains, unlike `DefaultHasher`.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |bytes: &[u8]| {
        for b in bytes {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    };
    for file in &files {
        eat(file.strip_prefix(&manifest).unwrap_or(file).to_string_lossy().as_bytes());
        eat(&[0]);
        eat(&std::fs::read(file).unwrap_or_default());
        eat(&[0]);
    }
    println!("cargo:rustc-env=CINCH_BUILD_ID={h:016x}");
}

fn collect(path: &std::path::Path, out: &mut Vec<PathBuf>) {
    if path.is_file() {
        out.push(path.to_path_buf());
    } else if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            collect(&entry.path(), out);
        }
    }
}
