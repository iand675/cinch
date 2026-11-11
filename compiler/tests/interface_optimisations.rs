//! The interface optimisation artifact: the bodies a module's small units
//! carry for a consumer's inliner, and the consumer's admission of them.
//!
//! The producer is `crate::ifaceopt::for_module` (called by the artifact
//! writer beside every `.cinchif`); the consumer is
//! `crate::simplify::simplify_with_foreign`. The tests here fix the three
//! properties the mechanism rests on: the artifact round-trips byte for
//! byte, it is keyed by the interface identity it was made against, and a
//! foreign body replaces a call site exactly as a local body does.

use cinch::core::{Core, FnDef, Program, SimplifyReport};
use cinch::ifaceopt::{foreign_defs, for_module, load_manifest, Optimisation};
use cinch::simplify::{simplify_with_foreign, SimplifyOptions};

/// The immediate children of a node, for the census walk. The test does not
/// need the compiler's crate-private traversal; this covers the shapes the
/// fixture produces.
fn children(e: &Core) -> Vec<&Core> {
    match e {
        Core::App(head, args) => {
            let mut out = vec![&**head];
            out.extend(args.iter());
            out
        }
        Core::Let(_, _, value, body) => vec![&**value, &**body],
        Core::Proj(inner, _) | Core::DictProj(inner, ..) => vec![&**inner],
        Core::Case { scrut, alts } => {
            let mut out = vec![&**scrut];
            out.extend(alts.iter().map(|a| &*a.body));
            out
        }
        Core::If(condition, then, otherwise) => {
            vec![&**condition, &*then.body, &*otherwise.body]
        }
        Core::Dup(_, _, body) | Core::Drop(_, _, body) => vec![&**body],
        Core::InstDict(_, args, _) => args.iter().collect(),
        _ => Vec::new(),
    }
}

/// Count saturated direct calls whose callee's unit name starts with any of
/// `prefixes`: the cross-object census at the Core level, before any
/// emission.
fn calls_to(program: &Program, prefixes: &[&str]) -> usize {
    fn walk(e: &Core, prefixes: &[&str], out: &mut usize) {
        if let Core::App(head, _) = e {
            if let Core::Global(g, _) = &**head {
                if prefixes.iter().any(|p| g.0.starts_with(p)) {
                    *out += 1;
                }
            }
        }
        for child in children(e) {
            walk(child, prefixes, out);
        }
    }
    let mut n = 0;
    for f in &program.fns {
        walk(&f.body, prefixes, &mut n);
    }
    n
}

const MAIN: &str = "\
module Main where
  import Data.Bool
  f :: Bool -> Int
  f b = if not b then 1 else 0
  main :: Int
  main = f True
";

fn check(src: &str) -> (Program, Vec<cinch::interface::Iface>) {
    cinch::driver::check_sources_ifaces(src, "iface-opt.cinch").expect("check")
}

#[test]
fn artifact_round_trips_and_carries_the_class() {
    let (program, ifaces) = check(MAIN);
    let bool_iface = ifaces.iter().find(|i| i.module == "Data.Bool").expect("Data.Bool");
    let artifact = for_module(bool_iface, &program);
    assert!(
        artifact.bodies.iter().any(|b| b.unit == "Data.Bool.not"),
        "the artifact must carry the prim wrapper `not`: {:?}",
        artifact.bodies.iter().map(|b| &b.unit).collect::<Vec<_>>()
    );
    // The `prim` unit itself has an inline-IR body, never a Core one.
    assert!(!artifact.bodies.iter().any(|b| b.unit.ends_with("prim-not-bool")));
    assert_eq!(artifact.iface_hash, bool_iface.iface_hash());

    let text = artifact.to_json();
    let back = Optimisation::from_json(&text).expect("parse");
    assert_eq!(back.to_json(), text, "canonical bytes are stable");
    assert_eq!(back.content_hash(), artifact.content_hash());
    assert_eq!(back.bodies.len(), artifact.bodies.len());
    // Every body is closed over its parameters, which the reader checked.
    assert!(back.bodies.iter().all(|b| b.params.len() == b.arity));
}

#[test]
fn a_stale_artifact_is_refused() {
    let dir = std::env::temp_dir().join(format!("cinch_ifaceopt_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let (program, ifaces) = check(MAIN);
    let bool_iface = ifaces.iter().find(|i| i.module == "Data.Bool").unwrap();
    let iface_path = dir.join("Data.Bool.cinchif");
    std::fs::write(&iface_path, bool_iface.to_json()).unwrap();
    // Produce the artifact, then edit its recorded interface identity: the
    // bodies are now inlining evidence for a contract that is not this one.
    let artifact = for_module(bool_iface, &program);
    let stale = artifact.to_json().replace(
        &format!("\"iface_hash\":\"{}\"", artifact.iface_hash),
        "\"iface_hash\":\"0\"",
    );
    std::fs::write(dir.join("Data.Bool.cinchopt"), stale).unwrap();
    let manifest = dir.join("interfaces.json");
    std::fs::write(
        &manifest,
        format!(
            "{{\"schema\":1,\"interfaces\":[[\"Data.Bool\",\"{}\",\"{}\"]]}}",
            bool_iface.iface_hash(),
            iface_path.display()
        ),
    )
    .unwrap();
    let error = load_manifest(&manifest).err().expect("stale artifact refused");
    assert!(
        error.to_string().contains("produced against interface"),
        "unexpected refusal: {error}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_foreign_body_replaces_the_same_call_a_local_body_does() {
    let (mut program, ifaces) = check(MAIN);
    let bool_iface = ifaces.iter().find(|i| i.module == "Data.Bool").unwrap().clone();
    // The artifact is produced while the producer still owns the body.
    let foreign = foreign_defs(&for_module(&bool_iface, &program));
    assert!(!foreign.is_empty());
    let before = calls_to(&program, &["Data.Bool."]);
    assert!(before > 0, "the fixture must call into Data.Bool");

    // Separate compilation: the consumer has the contract and the artifact,
    // not the body. Removing the units is what the driver's
    // `--interface-optimisations` path does after checking.
    program.fns.retain(|f: &FnDef| !f.id.0.starts_with("Data.Bool."));

    let mut report = SimplifyReport::default();
    simplify_with_foreign(&mut program, &foreign, &SimplifyOptions::default(), &mut report);
    let after = calls_to(&program, &["Data.Bool."]);
    assert_eq!(after, 0, "every Data.Bool call site must be unfolded");
    assert!(
        program
            .inline
            .values()
            .any(|facts| facts.callees.iter().any(|c| c == "Data.Bool.not")),
        "some call site must be recorded as an unfolding of `not`"
    );
}

// ------------------------------------------------------------- measurement
//
// The census this mechanism exists for: the compiler's own bundle, checked
// against the base library's bodies as if the library were a separate
// object. The bundle is 46 modules plus the embedded prelude; the prelude
// units are the cross-object calls. Run with
//
//   cargo test --release --test interface_optimisations -- --ignored --nocapture
//
// and read the two line counts it prints.

fn bundle_source(root: &std::path::Path) -> String {
    use std::collections::{BTreeMap, BTreeSet};
    let mut files: BTreeMap<String, String> = BTreeMap::new();
    for entry in std::fs::read_dir(root.join("bootstrap")).expect("bootstrap") {
        let path = entry.expect("entry").path();
        if path.extension().and_then(|e| e.to_str()) == Some("cinch") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            files.insert(name, std::fs::read_to_string(&path).expect("source"));
        }
    }
    for module in ["Lex", "Scan", "Parse", "Json", "Builder"] {
        files.insert(
            format!("Text.{module}.cinch"),
            std::fs::read_to_string(root.join(format!("compiler/prelude/Text/{module}.cinch")))
                .expect("toolkit"),
        );
    }
    let mut main = std::fs::read_to_string(root.join("bootstrap/programs/Run.Pipeline.cinch"))
        .expect("entry");
    let header = "module Main where\n";
    let mut modules: Vec<String> = files
        .keys()
        .filter(|n| n.ends_with(".cinch") && *n != "Main.cinch")
        .map(|n| n.trim_end_matches(".cinch").to_string())
        .collect();
    modules.sort();
    let imports: String = modules
        .iter()
        .enumerate()
        .map(|(i, m)| format!("  import {m} as CompilerSource{i}\n"))
        .collect();
    main = main.replacen(header, &format!("{header}{imports}"), 1);
    files.insert("Main.cinch".into(), main);

    // Dependencies before dependants: the checker elaborates modules in the
    // order it is given, and an import is resolved against a module already
    // elaborated. The project loader normally supplies that order; this
    // harness derives it from the bundle's own `import` lines.
    let mut deps: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, source) in &files {
        let mut list = Vec::new();
        for line in source.lines() {
            let t = line.trim_start();
            let Some(rest) = t.strip_prefix("import ") else { continue };
            let rest = rest.strip_prefix("stage ").unwrap_or(rest);
            let Some(first) = rest.split_whitespace().next() else { continue };
            let target = format!("{first}.cinch");
            if target != *name && files.contains_key(&target) {
                list.push(target);
            }
        }
        deps.insert(name.clone(), list);
    }
    let mut order: Vec<String> = Vec::new();
    let mut placed: BTreeSet<String> = BTreeSet::new();
    fn visit(
        name: &str,
        deps: &BTreeMap<String, Vec<String>>,
        placed: &mut BTreeSet<String>,
        order: &mut Vec<String>,
    ) {
        if !placed.insert(name.to_string()) {
            return;
        }
        if let Some(list) = deps.get(name) {
            for d in list {
                visit(d, deps, placed, order);
            }
        }
        order.push(name.to_string());
    }
    for name in files.keys() {
        visit(name, &deps, &mut placed, &mut order);
    }
    order
        .iter()
        .map(|name| format!("-- source: {name}\n{}", files[name]))
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[test]
#[ignore = "measures the compiler bundle; minutes, not seconds"]
fn cross_object_census_and_unfolding() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
        .to_path_buf();
    let source = bundle_source(&root);
    let (program, ifaces) =
        cinch::driver::check_sources_ifaces(&source, "compiler-bundle.cinch").expect("check bundle");
    let local: std::collections::BTreeSet<String> = ifaces
        .iter()
        .filter(|i| {
            i.module == "Main"
                || i.module.starts_with("Bootstrap")
                || i.module.starts_with("Text.")
        })
        .map(|i| i.module.clone())
        .collect();
    let prefixes: Vec<String> = ifaces
        .iter()
        .filter(|i| !local.contains(&i.module))
        .map(|i| format!("{}.", i.module))
        .collect();
    let prefix_refs: Vec<&str> = prefixes.iter().map(String::as_str).collect();
    fn count_unit(program: &Program, unit: &str) -> usize {
        fn walk(e: &Core, unit: &str, out: &mut usize) {
            if let Core::App(head, _) = e {
                if let Core::Global(g, _) = &**head {
                    if g.0 == unit {
                        *out += 1;
                    }
                }
            }
            for c in children(e) {
                walk(c, unit, out);
            }
        }
        let mut n = 0;
        for f in &program.fns {
            walk(&f.body, unit, &mut n);
        }
        n
    }
    fn count_nodes(program: &Program) -> usize {
        fn walk(e: &Core) -> usize {
            1 + children(e).into_iter().map(walk).sum::<usize>()
        }
        program.fns.iter().map(|f| walk(&f.body)).sum()
    }
    let named = [
        "Data.Machine.Eq_I64.==",
        "Data.String.string-compare",
        "Data.Nat.Eq_Nat.==",
        "Data.Bool.not",
        "Data.Nat.Add_Nat.+",
        "Data.List.Semigroup_List@O.mk",
    ];
    // Specialization is where a dictionary projection becomes a direct call
    // to a library unit, so the census is taken there, with the simplifier
    // switched off (`max_iterations: 0`). That is also the point an
    // artifact's bodies describe: the writer runs after its own optimize.
    let profile = cinch::core::TargetProfile::native();
    let no_simplify = SimplifyOptions { max_iterations: 0, ..SimplifyOptions::default() };
    let (mut program, _) =
        cinch::specialize::optimize_with(&program, &profile, &no_simplify).expect("specialize");
    let before: Vec<usize> = named.iter().map(|u| count_unit(&program, u)).collect();

    let mut foreign: Vec<FnDef> = Vec::new();
    let mut bodies = 0usize;
    let start = std::time::Instant::now();
    let class = cinch::simplify::unfoldable_names(&program);
    let class_ms = start.elapsed();
    let start = std::time::Instant::now();
    let mut bytes = 0usize;
    for iface in ifaces.iter() {
        let artifact = cinch::ifaceopt::for_module_in(iface, &program, &class);
        if local.contains(&iface.module) {
            continue;
        }
        bodies += artifact.bodies.len();
        bytes += artifact.to_json().len();
        foreign.extend(foreign_defs(&artifact));
    }
    let write_ms = start.elapsed();
    println!("class pass: {} ms, artifact build+encode: {} ms", class_ms.as_millis(), write_ms.as_millis());
    println!("artifact units: {bodies}, bytes: {bytes}");
    println!("cross-object calls before: {}", calls_to(&program, &prefix_refs));
    println!("nodes before: {}", count_nodes(&program));

    let mut separate = program.clone();
    separate.fns.retain(|f: &FnDef| !prefix_refs.iter().any(|p| f.id.0.starts_with(p)));
    separate.instances.retain(|i| !prefix_refs.iter().any(|p| i.id.0.starts_with(p)));

    let mut without = separate.clone();
    let mut report = SimplifyReport::default();
    simplify_with_foreign(&mut without, &[], &SimplifyOptions::default(), &mut report);
    println!("cross-object calls with no artifact: {}", calls_to(&without, &prefix_refs));
    println!("nodes with no artifact: {}", count_nodes(&without));

    let mut with = separate;
    let mut report = SimplifyReport::default();
    simplify_with_foreign(&mut with, &foreign, &SimplifyOptions::default(), &mut report);
    println!("cross-object calls after: {}", calls_to(&with, &prefix_refs));
    println!("nodes after: {}", count_nodes(&with));
    println!("sites unfolded: {}", report.inlined);
    for (i, unit) in named.iter().enumerate() {
        println!(
            "{unit}: before {}, no artifact {}, after {}",
            before[i],
            count_unit(&without, unit),
            count_unit(&with, unit)
        );
    }
    // The wrappers the class exists for must vanish at every site.
    for unit in ["Data.Machine.Eq_I64.==", "Data.Nat.Eq_Nat.==", "Data.Bool.not"] {
        assert_eq!(count_unit(&with, unit), 0, "{unit} must be unfolded");
    }
}
