//! The link plan (design/base-as-a-package.md section 4.3): semantic
//! fields become linker words, raw words need explicit consent and the
//! four product-changing words stay refused, and a machine-local override
//! is applied last and recorded. These tests build the plan only; they
//! never invoke a linker, so they run before the runtime archive exists.

use cinch::driver::{
    base_package, target_profile, DriverOptions, LinkPlan, LinkSpec, LinkTier, Package,
    ResolvedPackage, TargetFacts,
};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn resolved(name: &str, link: LinkSpec) -> (PathBuf, ResolvedPackage) {
    (
        PathBuf::from("/tmp"),
        ResolvedPackage {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            modules: Vec::new(),
            order: Vec::new(),
            link,
            prelude: cinch::driver::PreludeSpec::Default,
            matched: Vec::new(),
        },
    )
}

fn darwin() -> TargetFacts {
    TargetFacts {
        triple: "aarch64-apple-darwin".to_string(),
        os: "darwin".to_string(),
        arch: "aarch64".to_string(),
        libc: "libSystem".to_string(),
        os_version: None,
        compiler: env!("CARGO_PKG_VERSION").to_string(),
        runtime_abi: cinch::interface::RUNTIME_ABI_VERSION.to_string(),
    }
}

fn linux() -> TargetFacts {
    TargetFacts {
        triple: "x86_64-unknown-linux-gnu".to_string(),
        os: "linux".to_string(),
        arch: "x86_64".to_string(),
        libc: "glibc".to_string(),
        os_version: None,
        compiler: env!("CARGO_PKG_VERSION").to_string(),
        runtime_abi: cinch::interface::RUNTIME_ABI_VERSION.to_string(),
    }
}

fn spec(json: &str) -> LinkSpec {
    let text = format!(
        r#"{{ "name": "acme/x", "version": "0.1.0", "modules": [], "order": [], "link": {json} }}"#
    );
    // An empty modules list is legal for this test's purposes: the parser
    // only requires the field.
    Package::parse(&text).expect("the link spec parses").link
}

#[test]
fn semantic_fields_become_linker_words_in_a_stable_order() {
    let link = spec(
        r#"{ "libdirs": ["/opt/lib"], "rpaths": ["@loader_path/lib"], "libs": ["m", "z"],
             "frameworks": ["Security"], "whole_archive": ["wa"] }"#,
    );
    let plan = LinkPlan::build(
        &[resolved("acme/x", link)],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &darwin(),
    )
    .expect("a semantic-only plan builds");
    let words = plan.words();
    for want in [
        "-L/opt/lib",
        "-Wl,-rpath,@loader_path/lib",
        "-lm",
        "-lz",
        "-framework",
        "Security",
        "-Wl,-whole-archive",
        "-lwa",
        "-Wl,-no-whole-archive",
    ] {
        assert!(words.contains(&want.to_string()), "missing {want} in {words:?}");
    }
    assert_eq!(plan.entries.len(), 1);
    assert_eq!(plan.entries[0].tier, LinkTier::Semantic);
    assert_eq!(plan.entries[0].origin, "acme/x");
}

#[test]
fn the_gnu_soname_is_emitted_on_linux_only() {
    let link = spec(r#"{ "soname": "libacme.so.1" }"#);
    let plan = LinkPlan::build(
        &[resolved("acme/x", link.clone())],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &linux(),
    )
    .expect("linux takes a soname");
    assert!(
        plan.words().contains(&"-Wl,-soname,libacme.so.1".to_string()),
        "{:?}",
        plan.words()
    );
    let err = LinkPlan::build(
        &[resolved("acme/x", link)],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &darwin(),
    )
    .expect_err("ld64 has no soname");
    assert!(format!("{err}").contains("GNU linker"), "{err}");
}

#[test]
fn a_framework_is_refused_outside_darwin() {
    let link = spec(r#"{ "frameworks": ["Security"] }"#);
    let err = LinkPlan::build(
        &[resolved("acme/x", link)],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &linux(),
    )
    .expect_err("frameworks are ld64's");
    assert!(format!("{err}").contains("Darwin linker"), "{err}");
}

#[test]
fn raw_words_are_refused_without_consent_and_allowed_with_it() {
    let link = spec(r#"{ "raw": { "ldflags": ["-Wl,--as-needed"] } }"#);
    let packages = [resolved("acme/raw", link)];
    let err = LinkPlan::build(&packages, &[], &LinkSpec::default(), &BTreeMap::new(), &darwin())
        .expect_err("raw words need consent");
    let text = format!("{err}");
    assert!(text.contains("has not consented"), "{text}");
    assert!(text.contains("--allow-raw-link acme/raw"), "{text}");

    let plan = LinkPlan::build(
        &packages,
        &["acme/raw".to_string()],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &darwin(),
    )
    .expect("consented raw words build");
    assert_eq!(plan.words(), vec!["-Wl,--as-needed".to_string()]);
    assert_eq!(plan.raw_origins(), vec!["acme/raw".to_string()]);
}

#[test]
fn the_product_changing_words_stay_refused_even_with_consent() {
    for word in [
        "-Wl,--script=evil.ld",
        "-nostdlib",
        "-Xlinker",
        "@response.rsp",
        "-Wl,-e,start",
    ] {
        let link = spec(&format!(r#"{{ "raw": {{ "ldflags": ["{word}"] }} }}"#));
        let err = LinkPlan::build(
            &[resolved("acme/raw", link)],
            &["acme/raw".to_string()],
            &LinkSpec::default(),
            &BTreeMap::new(),
            &darwin(),
        )
        .expect_err("a product-changing word is refused");
        assert!(
            format!("{err}").contains("refused even with consent"),
            "{word}: {err}"
        );
    }
}

#[test]
fn a_local_override_goes_last_and_is_recorded() {
    let link = spec(r#"{ "libs": ["m"] }"#);
    let mut local = LinkSpec::default();
    local.libdirs = vec!["/opt/local/lib".to_string()];
    local.raw_ldflags = vec!["-Wl,-dead_strip".to_string()];
    let plan = LinkPlan::build(
        &[resolved("acme/x", link)],
        &[],
        &local,
        &BTreeMap::new(),
        &darwin(),
    )
    .expect("the local override builds");
    assert!(plan.local);
    let words = plan.words();
    let last = plan.entries.last().expect("the local entry");
    assert_eq!(last.tier, LinkTier::Local);
    assert_eq!(last.origin, "project-local");
    assert_eq!(
        words.last().map(String::as_str),
        Some("-Wl,-dead_strip"),
        "the local words are applied last: {words:?}"
    );
}

#[test]
fn a_pkgconfig_package_needs_its_lock_record() {
    let link = spec(r#"{ "pkgconfig": [ { "name": "gio", "min": "2.70" } ] }"#);
    let err = LinkPlan::build(
        &[resolved("acme/gio", link.clone())],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &darwin(),
    )
    .expect_err("a pkgconfig package without a lock record is refused");
    assert!(format!("{err}").contains("cinch pkg install"), "{err}");

    let mut lock: BTreeMap<String, Vec<String>> = BTreeMap::new();
    lock.insert(
        "acme/gio".to_string(),
        vec!["-L/usr/lib".to_string(), "-lgio-2.0".to_string()],
    );
    let plan = LinkPlan::build(
        &[resolved("acme/gio", link)],
        &[],
        &LinkSpec::default(),
        &lock,
        &darwin(),
    )
    .expect("the recorded words link");
    let words = plan.words();
    assert!(words.contains(&"-lgio-2.0".to_string()), "{words:?}");
}

#[test]
fn the_plan_describes_every_contribution_with_its_origin() {
    let plan = LinkPlan::build(
        &[resolved("acme/x", spec(r#"{ "libs": ["m"] }"#))],
        &[],
        &LinkSpec::default(),
        &BTreeMap::new(),
        &darwin(),
    )
    .expect("plan");
    let text = plan.describe();
    assert!(text.contains("semantic"), "{text}");
    assert!(text.contains("acme/x"), "{text}");
    assert!(text.contains("-lm"), "{text}");
}

#[test]
fn the_empty_plan_says_so() {
    let plan = LinkPlan::build(&[], &[], &LinkSpec::default(), &BTreeMap::new(), &darwin()).expect("plan");
    assert!(plan.entries.is_empty());
    assert_eq!(plan.describe(), "link plan: no package asked for anything\n");
}

#[test]
fn the_target_profile_accepts_the_host_triple_and_refuses_another() {
    let host = cinch::core::TargetProfile::native();
    let opts = DriverOptions {
        target: Some(host.triple.clone()),
        ..DriverOptions::default()
    };
    let profile = target_profile(&opts).expect("the host triple is accepted");
    assert_eq!(profile.triple, host.triple);
    assert!(!profile.os.is_empty());
    assert!(!profile.arch.is_empty());
    assert!(!profile.libc.is_empty());
    assert!(profile.os_version.is_none(), "a version is explicit, never assumed");

    let opts = DriverOptions {
        target: Some("wasm32-unknown-unknown".to_string()),
        ..DriverOptions::default()
    };
    let err = target_profile(&opts).expect_err("a foreign triple is refused");
    assert!(format!("{err}").contains("cross compilation"), "{err}");
}

#[test]
fn the_target_version_is_recorded_in_the_profile() {
    let opts = DriverOptions {
        target_version: Some("14.0".to_string()),
        ..DriverOptions::default()
    };
    let profile = target_profile(&opts).expect("a version alone is fine");
    assert_eq!(profile.os_version.as_deref(), Some("14.0"));
    assert!(profile.name.contains("14.0"), "the store key moves with it");

    let opts = DriverOptions {
        target_version: Some("   ".to_string()),
        ..DriverOptions::default()
    };
    assert!(target_profile(&opts).is_err(), "an empty version is not a version");
}

#[test]
fn the_base_manifest_carries_no_link_requirements() {
    let base = base_package();
    assert!(base.link.is_empty(), "the embedded family links nothing");
    assert!(base.raw_link.is_empty());
    assert!(base.local_link.is_empty());
}
