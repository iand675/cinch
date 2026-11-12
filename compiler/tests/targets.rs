//! Target selection: the triples `--target` accepts, the profile each one
//! builds from, the module header the emitted IR carries, the
//! architecture-gated `asm` bodies a target selects for the syscall layer,
//! what a cross `build` says when its runtime archive is absent or belongs
//! to another architecture, and what `run` refuses.
//!
//! The point of every assertion here is that layout and body selection read
//! the TARGET, not the build host: the same test binary runs on one host and
//! asserts two operating systems and two architectures.

mod common;

use cinch::core::TargetProfile;
use cinch::driver::{
    base_package, target_profile, DriverOptions, Mode, SUPPORTED_TARGETS,
};
use std::path::PathBuf;

/// The `aarch64-unknown-linux-gnu` datalayout, spelled as clang prints it.
const AARCH64_LINUX_LAYOUT: &str = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128";

/// The `x86_64-unknown-linux-gnu` datalayout.
const X86_64_LINUX_LAYOUT: &str =
    "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128";

/// The `aarch64-apple-darwin` datalayout.
const AARCH64_DARWIN_LAYOUT: &str =
    "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128";

/// A two-line module: enough for `check`, `emit-ir`, and `build`.
const TRIVIAL: &str = "module Main where\n  main :: Int\n  main = 0\n";

/// A module that reaches the Linux syscall layer, so the emitted IR carries
/// the `asm` bodies the target selects. `sys-o-rdonly` names the module
/// without performing I/O, which `main :: Int` cannot do.
/// A module that reaches the Linux syscall layer's `openat` body, so the
/// emitted IR carries the `asm` bodies the target selects. The program is
/// never run: the body is what this asserts on. It goes through
/// `sys-open-read`, because a whole-program build emits only the units an
/// entry point reaches and the constant `sys-o-rdonly` alone folds away.
const SYS: &str = "\
module Main where
  import Prim.Sys.Linux

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case sys-open-read \"\" of
      _ -> 0
";

/// A module that runs a transaction, so the emitted IR carries the atomic
/// operations and the two architecture-gated bodies of `Effect.Stm` (the
/// spin hint and the cycle counter).
const STM: &str = "\
module Main where
  import Effect.Stm

  main :: Unit ->{STM, Diverge} Int
  main u =
    let t = tvar-new-io (41 :: Nat) in
    case atomically (tvar-modify t (\\x -> x + 1)) of
      _ -> if tvar-read-io t == 42 then 0 else 1
";

/// The profile `--target <triple>` builds.
fn profile(triple: &str) -> TargetProfile {
    target_profile(&DriverOptions {
        target: Some(triple.to_string()),
        ..DriverOptions::default()
    })
    .unwrap_or_else(|e| panic!("{triple}: {e}"))
}

/// Write one source to a fresh temporary project and return its directory.
fn write_project(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch-targets-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    dir
}

/// Emit one source's IR under one target.
fn emit(name: &str, src: &str, target: &TargetProfile) -> String {
    let program = cinch::driver::check_sources_profile(src, name, target)
        .unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    cinch::codegen::emit_for(target, &program)
        .unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

/// Assemble emitted IR to an object for `triple` with clang, the
/// assembler a build uses. This is where an inline `asm` body written in
/// the wrong dialect, or with an instruction the architecture lacks, is
/// found: emission only copies the text.
fn assemble(name: &str, ir: &str, triple: &str) {
    let dir = write_project(name, "");
    let ll = dir.join("out.ll");
    std::fs::write(&ll, ir).expect("write IR");
    let out = std::process::Command::new("clang")
        .arg("-c")
        .arg(format!("--target={triple}"))
        .arg(&ll)
        .arg("-o")
        .arg(dir.join("out.o"))
        .output()
        .expect("run clang");
    assert!(
        out.status.success(),
        "{name}: clang refused the {triple} module: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A minimal static archive holding one object that begins with `object`,
/// so the archive check can be exercised without a cross toolchain.
fn archive_with(object: &[u8]) -> Vec<u8> {
    let mut out = b"!<arch>\n".to_vec();
    let mut header = [b' '; 60];
    header[0..6].copy_from_slice(b"obj.o/");
    // size is the decimal field at 48..58, then the "`\n" terminator.
    let size = format!("{}", object.len());
    header[48..48 + size.len()].copy_from_slice(size.as_bytes());
    header[58..60].copy_from_slice(b"`\n");
    out.extend_from_slice(&header);
    out.extend_from_slice(object);
    out
}

#[test]
fn every_supported_triple_builds_a_profile_from_the_triple_alone() {
    for triple in SUPPORTED_TARGETS {
        let p = profile(triple);
        assert_eq!(&p.triple, triple, "the profile carries the requested triple");
        assert_eq!(p.name, *triple, "the store key names the target, not the host");
        // wasm32 is the one 32-bit target in the table; every other
        // triple is 64-bit. The word comes from the triple, never from
        // the host, which is what this test is about.
        let words = if triple.starts_with("wasm32") { 32 } else { 64 };
        assert_eq!(p.word_size, words, "{triple} carries the word width its triple names");
        assert_eq!(p.endian, "little", "{triple} is little-endian");
        assert!(!p.os.is_empty() && !p.arch.is_empty() && !p.libc.is_empty());
    }
    // The components come from the triple: one host, two operating systems
    // and two architectures, and neither set is this host's own answer.
    let linux = profile("x86_64-unknown-linux-gnu");
    assert_eq!((linux.os.as_str(), linux.arch.as_str(), linux.libc.as_str()), ("linux", "x86_64", "glibc"));
    let darwin = profile("aarch64-apple-darwin");
    assert_eq!((darwin.os.as_str(), darwin.arch.as_str(), darwin.libc.as_str()), ("darwin", "aarch64", "libSystem"));
    assert_eq!(profile("x86_64-apple-darwin").arch, "x86_64");
    assert_eq!(profile("aarch64-unknown-linux-gnu").arch, "aarch64");
}

#[test]
fn an_unsupported_triple_is_refused_by_naming_the_supported_list() {
    let err = target_profile(&DriverOptions {
        target: Some("riscv64gc-unknown-linux-gnu".to_string()),
        ..DriverOptions::default()
    })
    .expect_err("an unsupported triple is refused");
    let msg = err.to_string();
    assert!(msg.contains("riscv64gc-unknown-linux-gnu"), "{msg}");
    for triple in SUPPORTED_TARGETS {
        assert!(msg.contains(triple), "{msg} does not name {triple}");
    }
    // A list, not a guess: one comma-separated run of triples.
    assert!(
        msg.contains(&SUPPORTED_TARGETS.join(", ")),
        "{msg} does not read as the list"
    );
    // The message keeps the phrase `package_link`'s refusal test pins.
    assert!(msg.contains("cross compilation"), "{msg}");
}

#[test]
fn the_host_triple_is_accepted_and_keeps_the_host_profile() {
    let host = TargetProfile::native();
    if !SUPPORTED_TARGETS.contains(&host.triple.as_str()) {
        // A host outside the table is refused like any other unknown
        // triple; the table grows by adding the host's own spelling.
        assert!(target_profile(&DriverOptions {
            target: Some(host.triple.clone()),
            ..DriverOptions::default()
        })
        .is_err());
        return;
    }
    let p = profile(&host.triple);
    assert_eq!(p.triple, host.triple);
    assert_eq!(p.os, host.os);
    assert_eq!(p.arch, host.arch);
    assert_eq!(p.libc, host.libc);
    assert_eq!(p.word_size, host.word_size);
    assert_eq!(p.endian, host.endian);
}

#[test]
fn the_emitted_module_names_the_target_and_its_datalayout() {
    let arm = emit("sys-arm", SYS, &profile("aarch64-unknown-linux-gnu"));
    assert!(
        arm.contains("target triple = \"aarch64-unknown-linux-gnu\""),
        "the module names the target triple: {arm}"
    );
    assert!(
        arm.contains(&format!("target datalayout = \"{AARCH64_LINUX_LAYOUT}\"")),
        "the aarch64 ELF datalayout is the module's: {arm}"
    );
    assert!(arm.contains("mov x8, #56"), "the aarch64 `openat` body: {arm}");
    assert!(arm.contains("svc #0"), "the aarch64 trap instruction: {arm}");
    assert!(!arm.contains("mov rax"), "no x86_64 body survived: {arm}");

    let x86 = emit("sys-x86", SYS, &profile("x86_64-unknown-linux-gnu"));
    assert!(
        x86.contains("target triple = \"x86_64-unknown-linux-gnu\""),
        "the module names the target triple: {x86}"
    );
    assert!(
        x86.contains(&format!("target datalayout = \"{X86_64_LINUX_LAYOUT}\"")),
        "the x86_64 ELF datalayout is the module's: {x86}"
    );
    assert!(x86.contains("mov rax, 257"), "the x86_64 `openat` body: {x86}");
    assert!(x86.contains("syscall"), "the x86_64 trap instruction: {x86}");
    assert!(!x86.contains("mov x8, #56"), "no aarch64 body survived: {x86}");
    assert!(!x86.contains("svc #0"), "no aarch64 body survived: {x86}");

    assert_ne!(arm, x86, "the two targets do not emit the same module");
}

#[test]
fn the_target_gated_bodies_assemble_for_each_architecture() {
    for triple in ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "aarch64-apple-darwin"] {
        let target = profile(triple);
        assemble(&format!("asm-sys-{triple}"), &emit("asm-sys", SYS, &target), triple);
        let stm = emit("asm-stm", STM, &target);
        let (hint, counter) = if triple.starts_with("x86_64") {
            ("\"pause\"", "\"rdtsc\"")
        } else {
            ("\"isb\"", "cntvct_el0")
        };
        assert!(stm.contains(hint), "{triple}: the spin hint is the architecture's own");
        assert!(stm.contains(counter), "{triple}: the cycle counter is the architecture's own");
        assemble(&format!("asm-stm-{triple}"), &stm, triple);
    }
}

#[test]
fn a_darwin_target_emits_a_macho_datalayout() {
    let ir = emit("trivial-arm-darwin", TRIVIAL, &profile("aarch64-apple-darwin"));
    assert!(ir.contains("target triple = \"aarch64-apple-darwin\""), "{ir}");
    assert!(
        ir.contains(&format!("target datalayout = \"{AARCH64_DARWIN_LAYOUT}\"")),
        "the aarch64 Mach-O datalayout is the module's: {ir}"
    );
    // `m:o` is Mach-O and `m:e` is ELF: the distinction the layout carries.
    assert!(AARCH64_DARWIN_LAYOUT.contains("-m:o-"), "Mach-O spelling");
    assert!(AARCH64_LINUX_LAYOUT.contains("-m:e-"), "ELF spelling");
}

#[test]
fn a_cross_build_without_a_runtime_archive_names_what_is_missing() {
    let dir = write_project("cross-no-archive", TRIVIAL);
    let out = dir.join("prog");
    let opts = DriverOptions {
        target: Some("aarch64-unknown-linux-gnu".to_string()),
        ..DriverOptions::default()
    };
    let err = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(out.clone()), &opts)
        .expect_err("a cross build without the target's runtime archive fails");
    let msg = err.to_string();
    assert!(msg.contains("aarch64-unknown-linux-gnu"), "{msg}");
    assert!(msg.contains("--runtime-archive"), "{msg} does not name the flag");
    assert!(
        msg.contains("none was named"),
        "{msg} does not say what is missing"
    );
    assert!(
        !msg.contains("is not implemented"),
        "{msg} is the old blanket refusal"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_runtime_archive_that_is_absent_is_named_by_path() {
    let dir = write_project("cross-absent-archive", TRIVIAL);
    let missing = dir.join("nope").join("libcinchruntime.a");
    let opts = DriverOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        runtime_archive: Some(missing.clone()),
        ..DriverOptions::default()
    };
    let err = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(dir.join("prog")), &opts)
        .expect_err("an absent archive fails the build");
    let msg = err.to_string();
    assert!(msg.contains(&missing.display().to_string()), "{msg}");
    assert!(msg.contains("x86_64-unknown-linux-gnu"), "{msg}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_runtime_archive_of_the_wrong_architecture_is_named_as_such() {
    let dir = write_project("cross-wrong-archive", TRIVIAL);
    // A well-formed archive whose one member is an x86_64 Mach-O object,
    // offered for an aarch64 Linux target.
    let mut object = vec![0u8; 32];
    object[0..4].copy_from_slice(b"\xcf\xfa\xed\xfe");
    object[4..8].copy_from_slice(&0x0100_0007u32.to_le_bytes());
    let archive = dir.join("libcinchruntime.a");
    std::fs::write(&archive, archive_with(&object)).expect("write a synthetic archive");
    let opts = DriverOptions {
        target: Some("aarch64-unknown-linux-gnu".to_string()),
        runtime_archive: Some(archive.clone()),
        ..DriverOptions::default()
    };
    let err = common::drive_opts(Mode::Build, &dir, "Main.cinch", Some(dir.join("prog")), &opts)
        .expect_err("the wrong architecture is refused before the link");
    let msg = err.to_string();
    assert!(msg.contains("x86_64"), "{msg} does not name what the archive holds");
    assert!(msg.contains("not aarch64"), "{msg} does not name the target's arch");
    assert!(msg.contains("aarch64-unknown-linux-gnu"), "{msg}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn run_refuses_a_foreign_target_and_says_why() {
    let dir = write_project("cross-run", TRIVIAL);
    let opts = DriverOptions {
        target: Some("x86_64-unknown-linux-gnu".to_string()),
        ..DriverOptions::default()
    };
    let err = common::drive_opts(Mode::Run, &dir, "Main.cinch", None, &opts)
        .expect_err("a foreign target cannot run here");
    let msg = err.to_string();
    assert!(msg.contains("x86_64-unknown-linux-gnu"), "{msg}");
    let host = TargetProfile::native();
    assert!(msg.contains(&host.triple), "{msg} does not name this host");
    assert!(msg.contains("does not run here"), "{msg}");
    assert!(msg.contains("cinch build"), "{msg} does not say what to do instead");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_artifact_manifest_records_the_target_of_a_cross_check() {
    let dir = write_project("cross-manifest", TRIVIAL);
    let opts = DriverOptions {
        target: Some("aarch64-unknown-linux-gnu".to_string()),
        ..DriverOptions::default()
    };
    common::drive_opts(Mode::Check, &dir, "Main.cinch", None, &opts).expect("the cross check writes its artifacts");
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("the artifact manifest");
    assert!(
        manifest.contains("\"triple\":\"aarch64-unknown-linux-gnu\""),
        "{manifest}"
    );
    assert!(manifest.contains("\"os\":\"linux\""), "{manifest}");
    assert!(manifest.contains("\"arch\":\"aarch64\""), "{manifest}");
    assert!(manifest.contains("\"libc\":\"glibc\""), "{manifest}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_package_store_key_moves_with_the_target() {
    let pkg = base_package();
    let sources = pkg
        .ordered_sources()
        .expect("the base manifest names its embedded sources");
    let key = |triple: &str| {
        cinch::store::package_key_for(pkg, &sources, &profile(triple), &[])
    };
    let linux_arm = key("aarch64-unknown-linux-gnu");
    let linux_x86 = key("x86_64-unknown-linux-gnu");
    let darwin_arm = key("aarch64-apple-darwin");
    assert_ne!(linux_arm, linux_x86, "the architecture is part of the key");
    assert_ne!(linux_arm, darwin_arm, "the operating system is part of the key");
    assert_ne!(
        linux_arm,
        cinch::store::package_key_for(pkg, &sources, &TargetProfile::native(), &[]),
        "a cross target is not the host's entry"
    );
}
