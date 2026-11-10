//! Core output (`design/core-output.md`): the golden corpus regenerates byte
//! for byte, the frontend stages stay target independent, the verifier
//! rejects what it must, `diff` reports the documented path, and a build's
//! manifest records the dump it wrote.
//!
//! The golden files are produced by the same composition the compiler uses
//! ([`cinch::coretext::dump`] with the checked interfaces, the entry modules
//! and specialization's rewrite applications). Set `CINCH_UPDATE_CORE=1` to
//! rewrite them after an intended format change; the default run only reads.

mod common;

use cinch::core::TargetProfile;
use cinch::coretext::{self, Stage};
use cinch::driver::{check_sources_ifaces, check_sources_profile_ifaces};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn core_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/core")
}

/// The stage each golden file is dumped at: the stage that makes its feature
/// explicit. A feature the checked program already carries is dumped at
/// `checked`, so the file also shows the frontend's own answers.
fn golden_stage(name: &str) -> Stage {
    match name {
        "dictionaries" | "evidence" | "indices" => Stage::Checked,
        _ => Stage::Lowered,
    }
}

/// Check one source and print it the way the compiler does: the same
/// composition [`cinch::coretext::dump`] gives the build, with the golden's
/// entry module and specialization's rewrite applications.
fn dump_source(src: &str, file: &str, stage: Stage, profile: &TargetProfile) -> String {
    let (program, ifaces) = check_sources_profile_ifaces(src, file, profile)
        .unwrap_or_else(|e| panic!("{file}: check failed: {e}"));
    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
    match stage {
        Stage::Checked => {
            coretext::dump(&program, stage, profile, &ifaces, &modules, &BTreeMap::new(), 0)
        }
        Stage::Owned => {
            let owned = cinch::rc::insert(program, profile).expect("reference counting");
            coretext::dump(&owned, stage, profile, &ifaces, &modules, &BTreeMap::new(), 0)
        }
        Stage::Lowered => {
            let (specialized, report) =
                cinch::specialize::optimize(&program, profile).expect("specialization");
            let lowered = cinch::rc::insert_closed(specialized, profile).expect("reference counting");
            let mut rewrites: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for site in &report.sites {
                if let Some(rule) = site.kind.strip_prefix("rewrite:") {
                    rewrites.entry(site.fn_id.clone()).or_default().push(rule.to_string());
                }
            }
            // The simplifier's tick count is part of the lowered header, so
            // the golden pins it as well as the bodies it rewrote.
            coretext::dump_with_ticks(
                &lowered,
                stage,
                profile,
                &ifaces,
                &modules,
                &rewrites,
                0,
                Some(report.simplify.ticks),
            )
        }
    }
}

#[test]
fn golden_files_regenerate_byte_for_byte() {
    let profile = TargetProfile::native();
    let update = std::env::var_os("CINCH_UPDATE_CORE").is_some();
    let dir = core_dir();
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("tests/core")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map(|ext| ext == "cinch").unwrap_or(false) {
                Some(path.file_stem()?.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();
    names.sort();
    assert!(
        names.len() >= 7,
        "the corpus covers dictionaries, evidence, indices, ownership, rewrites, prims and quotes"
    );
    for name in names {
        let src_path = dir.join(format!("{name}.cinch"));
        let src = std::fs::read_to_string(&src_path).expect("golden source");
        let stage = golden_stage(&name);
        let text = std::thread::Builder::new()
            .stack_size(128 * 1024 * 1024)
            .spawn({
                let src = src.clone();
                let name = name.clone();
                let profile = profile.clone();
                move || dump_source(&src, &format!("{name}.cinch"), stage, &profile)
            })
            .expect("spawn")
            .join()
            .expect("golden thread");
        // The printer is deterministic: the same program prints the same
        // bytes, so a golden file is a fixed point rather than a sample.
        let again = dump_source(&src, &format!("{name}.cinch"), stage, &profile);
        assert_eq!(text, again, "{name}: printing twice changed the text");
        // Every golden file verifies: the reader checks the printed grammar.
        if let Err(errors) = coretext::verify(&text) {
            let rows: Vec<String> = errors.iter().map(|error| error.to_string()).collect();
            panic!("{name}: the dump does not verify:\n{}", rows.join("\n"));
        }
        let out_path = dir.join(format!("{name}.core"));
        if update {
            std::fs::write(&out_path, &text).expect("write golden");
        } else {
            let expected = std::fs::read_to_string(&out_path).unwrap_or_else(|e| {
                panic!(
                    "{name}: cannot read {}: {e}; run with CINCH_UPDATE_CORE=1",
                    out_path.display()
                )
            });
            assert_eq!(
                text,
                expected,
                "{name}: the dump moved; run with CINCH_UPDATE_CORE=1 to accept it"
            );
        }
    }
}

#[test]
fn checked_and_owned_are_target_independent() {
    let src = r#"
module Main where
  import Data.List

  pick :: forall a. using (oe :: Ord a) => a -> a -> a
  pick x y = if x < y then y else x

  drain :: List Int -> Int
  drain xs = case xs of
                Nil -> 0
                Cons h t -> h + drain t

  main :: Int
  main = drain (Cons (pick 1 2) Nil)
"#;
    let triples = [
        "aarch64-apple-darwin",
        "x86_64-apple-darwin",
        "x86_64-unknown-linux-gnu",
    ];
    let mut checked: Vec<String> = Vec::new();
    let mut owned: Vec<String> = Vec::new();
    let mut lowered: Vec<String> = Vec::new();
    for triple in triples {
        let profile = profile_of(triple);
        let text = std::thread::Builder::new()
            .stack_size(128 * 1024 * 1024)
            .spawn({
                let src = src.to_string();
                let profile = profile.clone();
                move || {
                    let (program, ifaces) =
                        check_sources_profile_ifaces(&src, "targets.cinch", &profile)
                            .unwrap_or_else(|e| panic!("{triple}: check failed: {e}"));
                    let modules: BTreeSet<String> = ["Main".to_string()].into_iter().collect();
                    let checked = coretext::dump(
                        &program,
                        Stage::Checked,
                        &profile,
                        &ifaces,
                        &modules,
                        &BTreeMap::new(),
                        0,
                    );
                    let counts = cinch::rc::insert(program.clone(), &profile).expect("rc");
                    let owned = coretext::dump(
                        &counts,
                        Stage::Owned,
                        &profile,
                        &ifaces,
                        &modules,
                        &BTreeMap::new(),
                        0,
                    );
                    let (specialized, _) =
                        cinch::specialize::optimize(&program, &profile).expect("specialize");
                    let lowered = cinch::rc::insert_closed(specialized, &profile).expect("rc");
                    let lowered = coretext::dump(
                        &lowered,
                        Stage::Lowered,
                        &profile,
                        &ifaces,
                        &modules,
                        &BTreeMap::new(),
                        0,
                    );
                    (checked, owned, lowered)
                }
            })
            .expect("spawn")
            .join()
            .expect("target thread");
        checked.push(text.0);
        owned.push(text.1);
        lowered.push(text.2);
    }
    for (index, triple) in triples.iter().enumerate().skip(1) {
        assert_eq!(
            checked[0], checked[index],
            "checked Core differs for {triple}: a target fact leaked into the frontend"
        );
        assert_eq!(
            owned[0], owned[index],
            "owned Core differs for {triple}: a target fact leaked into the frontend"
        );
    }
    assert!(
        !checked[0].contains("target "),
        "the checked stage prints no target line"
    );
    assert!(
        !owned[0].contains("target "),
        "the owned stage prints no target line"
    );
    // The lowered stage names its target, so at least one triple must differ.
    assert!(
        lowered.iter().any(|text| text != &lowered[0]),
        "the lowered stage prints the target, so the three triples cannot all agree"
    );
}

fn profile_of(triple: &str) -> TargetProfile {
    match triple {
        "aarch64-apple-darwin" => {
            let mut profile = TargetProfile::native();
            profile.name = triple.to_string();
            profile.triple = triple.to_string();
            profile.os = "darwin".to_string();
            profile.arch = "aarch64".to_string();
            profile.libc = "libSystem".to_string();
            profile
        }
        "x86_64-apple-darwin" => {
            let mut profile = TargetProfile::native();
            profile.name = triple.to_string();
            profile.triple = triple.to_string();
            profile.os = "darwin".to_string();
            profile.arch = "x86_64".to_string();
            profile.libc = "libSystem".to_string();
            profile
        }
        _ => {
            let mut profile = TargetProfile::native();
            profile.name = triple.to_string();
            profile.triple = triple.to_string();
            profile.os = "linux".to_string();
            profile.arch = "x86_64".to_string();
            profile.libc = "glibc".to_string();
            profile
        }
    }
}

/// A minimal readable file the verifier tests edit in place.
const BASE: &str = "\
core 1
module Main
source sha256:0000000000000000000000000000000000000000000000000000000000000000
unit Main.small :: Nat -> Nat
  @params t0 :: Nat @owned
  @stage(checked)
  body (local t0)
end
";

fn with_unit(extra: &str) -> String {
    format!("{BASE}{extra}")
}

#[test]
fn verify_accepts_the_hand_written_file() {
    let report = coretext::verify(BASE).expect("the base file verifies");
    assert_eq!(report.units, 1);
    assert_eq!(report.stage.as_deref(), Some("checked"));
    assert_eq!(report.expressions, 1);
    assert_eq!(report.annotations.get("stage"), Some(&1));
}

#[test]
fn verify_rejects_an_out_of_scope_local() {
    let text = BASE.replace("body (local t0)", "body (local t7)");
    let errors = coretext::verify(&text).expect_err("an unbound local is refused");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].unit, "Main.small");
    assert_eq!(errors[0].path, "body");
    assert!(errors[0].message.contains("t7"), "{}", errors[0].message);
}

#[test]
fn verify_rejects_a_duplicated_unit_name() {
    let text = with_unit(
        "\
unit Main.small :: Nat -> Nat
  @stage(checked)
  body (int 0)
end
",
    );
    let errors = coretext::verify(&text).expect_err("a repeated unit is refused");
    assert!(
        errors.iter().any(|error| error.unit == "Main.small"
            && error.message.contains("not unique")),
        "{errors:?}"
    );
}

#[test]
fn verify_rejects_an_unknown_annotation() {
    let text = BASE.replace("@stage(checked)", "@inlined(yes)");
    let errors = coretext::verify(&text).expect_err("an unknown annotation is refused");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].path, "@");
    assert!(errors[0].message.contains("@inlined"), "{}", errors[0].message);
}

#[test]
fn verify_rejects_an_annotation_out_of_order() {
    let text = BASE.replace(
        "  @stage(checked)\n",
        "  @stage(checked)\n  @borrowed(1)\n",
    );
    let errors = coretext::verify(&text).expect_err("order is part of the grammar");
    assert!(
        errors.iter().any(|error| error.message.contains("out of order")),
        "{errors:?}"
    );
}

#[test]
fn verify_rejects_a_missing_dictionary_slot() {
    let text = with_unit(
        "\
unit Main.EchoInt :: Main.Echo
  @instance(tparams 0, subdicts 0)
  @stage(checked)
  static {0 = Main.EchoInt.echo}
end
unit Main.EchoInt.echo :: Int -> Int
  @params t0 :: Int @owned
  @stage(checked)
  body (local t0)
end
unit Main.use :: Int -> Int
  @params t0 :: Int @owned
  @stage(checked)
  body (app (dictproj (instdict Main.EchoInt) 3 Main.Echo) (local t0))
end
",
    );
    let errors = coretext::verify(&text).expect_err("slot 3 does not exist");
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].unit, "Main.use");
    assert_eq!(errors[0].path, "body:callee");
    assert!(errors[0].message.contains("slot 3"), "{}", errors[0].message);
}

#[test]
fn verify_rejects_a_dictionary_class_mismatch() {
    let text = with_unit(
        "\
unit Main.EchoInt :: Main.Echo
  @instance(tparams 0, subdicts 0)
  @stage(checked)
  static {0 = Main.EchoInt.echo}
end
unit Main.EchoInt.echo :: Int -> Int
  @stage(checked)
  body (int 0)
end
unit Main.use :: Int -> Int
  @stage(checked)
  body (dictproj (instdict Main.EchoInt) 0 Main.Other)
end
",
    );
    let errors = coretext::verify(&text).expect_err("the class must be the instance's");
    assert!(
        errors.iter().any(|error| error.message.contains("Main.Other")),
        "{errors:?}"
    );
}

#[test]
fn verify_rejects_an_unresolved_global() {
    let text = BASE.replace("body (local t0)", "body (global Main.missing)");
    let errors = coretext::verify(&text).expect_err("an undefined unit is refused");
    assert_eq!(errors[0].path, "body");
    assert!(errors[0].message.contains("Main.missing"), "{}", errors[0].message);
}

#[test]
fn verify_rejects_a_bad_header_and_a_bad_span() {
    let errors = coretext::verify("core 2\nmodule Main\n").expect_err("wrong version");
    assert_eq!(errors[0].path, "header");
    let text = BASE.replace(
        "  @stage(checked)",
        "  @span Main.cinch:0:1\n  @stage(checked)",
    );
    let errors = coretext::verify(&text).expect_err("line 0 is not a source position");
    assert!(
        errors.iter().any(|error| error.path == "@span" && error.message.contains("line")),
        "{errors:?}"
    );
}

#[test]
fn diff_reports_the_documented_path() {
    // The documented shape: `Main.map:body:alt 2:field 1: left (int 1) right
    // (int 2)`. The second alternative's first field differs.
    let left = BASE.replace(
        "  body (local t0)",
        "  body (case (local t0)\n    ((con 0) (int 1))\n    ((int 1) (int 0)))",
    );
    let right = left.replace("((int 1) (int 0))", "((int 2) (int 0))");
    let divergence = coretext::diff(&left, &right).expect("the files differ");
    assert_eq!(
        divergence,
        "Main.small:body:alt 2:field 1: left (int 1) right (int 2)"
    );
    assert_eq!(coretext::diff(&left, &left), None, "identical files do not diverge");
}

#[test]
fn diff_reports_a_unit_present_on_one_side() {
    let right = with_unit(
        "\
unit Main.extra :: Nat -> Nat
  @stage(checked)
  body (int 1)
end
",
    );
    let divergence = coretext::diff(BASE, &right).expect("the files differ");
    assert_eq!(divergence, "Main.extra: only in the right file");
    assert_eq!(
        coretext::diff(&right, BASE).as_deref(),
        Some("Main.extra: only in the left file")
    );
}

#[test]
fn diff_reports_a_differing_annotation() {
    let left = BASE.replace("@stage(checked)", "@stage(checked)\n  @borrowed(1)");
    let right = BASE.replace("@stage(checked)", "@stage(checked)\n  @borrowed(2)");
    let divergence = coretext::diff(&left, &right).expect("the annotations differ");
    assert_eq!(divergence, "Main.small:@borrowed: left (1) right (2)");
}

// ------------------------------------------------------- the build's dump

#[test]
fn dump_core_lowered_matches_the_manifest_digest() {
    use sha2::{Digest, Sha256};
    let dir = std::env::temp_dir().join(format!("cinch_core_dump_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let src = r#"module Main where
  main :: Int
  main = 41 + 1
"#;
    let entry = dir.join("Main.cinch");
    std::fs::write(&entry, src).expect("write source");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("emit-ir")
        .arg("--dump-core=lowered")
        .arg("-o")
        .arg(dir.join("main.ll"))
        .arg(&entry)
        .output()
        .expect("run cinch");
    assert!(
        output.status.success(),
        "emit-ir failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let core_path = dir.join("build/Main.core");
    let text = std::fs::read_to_string(&core_path).expect("build/Main.core");
    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("build/Main.manifest.json");
    let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
    assert!(
        manifest.contains(&format!("\"digest\":\"{digest}\"")),
        "the manifest does not record the dump's digest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"stage\":\"lowered\""),
        "the manifest does not record the dump's stage"
    );
    assert!(text.contains("target aarch64") || text.contains("target x86_64"), "{text}");
    // The dump the build wrote is the dump the reader accepts.
    coretext::verify(&text).expect("the build's dump verifies");
    // The stage the mode reaches is printed: `check` cannot reach `lowered`.
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("check")
        .arg("--dump-core=lowered")
        .arg(&entry)
        .output()
        .expect("run cinch check");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let checked = std::fs::read_to_string(dir.join("build/Main.checked.core"))
        .expect("build/Main.checked.core");
    assert!(checked.contains("@stage(checked)"), "{checked}");
    assert!(!checked.contains("target "), "the checked dump prints no target line");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn core_emit_writes_the_golden_file() {
    let dir = std::env::temp_dir().join(format!("cinch_core_emit_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let name = "ownership";
    // The compile writes its artifacts beside the entry file, so the entry is
    // a copy in the scratch directory: the corpus directory stays clean.
    let src = dir.join(format!("{name}.cinch"));
    std::fs::copy(core_dir().join(format!("{name}.cinch")), &src).expect("copy source");
    let out = dir.join(format!("{name}.core"));
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("core")
        .arg("emit")
        .arg(&src)
        .arg("-o")
        .arg(&out)
        .output()
        .expect("run cinch core emit");
    assert!(
        output.status.success(),
        "core emit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let emitted = std::fs::read_to_string(&out).expect("emitted dump");
    let golden = std::fs::read_to_string(core_dir().join(format!("{name}.core"))).expect("golden");
    assert_eq!(emitted, golden, "core emit and the test harness disagree");
    // `verify` and `diff` are the reader's command line surface.
    let verify = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("core")
        .arg("verify")
        .arg(&out)
        .output()
        .expect("run cinch core verify");
    assert!(verify.status.success(), "{}", String::from_utf8_lossy(&verify.stderr));
    let diff = std::process::Command::new(env!("CARGO_BIN_EXE_cinch"))
        .arg("core")
        .arg("diff")
        .arg(&out)
        .arg(&out)
        .output()
        .expect("run cinch core diff");
    assert_eq!(diff.status.code(), Some(0));
    assert!(diff.stdout.is_empty(), "identical files print nothing");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn checked_sources_helper_is_reachable() {
    // `check_sources_ifaces` is the single-module entry the corpus harness
    // does not use; keep it exercised so the two entry points cannot drift.
    let (_, ifaces) = check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "one.cinch")
        .expect("check");
    assert!(ifaces.iter().any(|iface| iface.module == "Main"));
}
