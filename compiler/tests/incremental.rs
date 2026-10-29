//! P29: Blaze-style incremental recompilation (design/incremental-recompilation.md).
//!
//! The pins, over real multi-module projects:
//!
//! * REAPER: kinded edges (type/contract vs body) built from the
//!   `.cinchif` metadata and contract facts; a body change marks only
//!   its declaration dirty; a signature or contract change marks the
//!   dependents' closure dirty.
//! * INCR: a warm rebuild touches only the changed units (asserted on
//!   the session's own report, not on timings).
//! * LINK: the object cache keys on declaration hashes, and deleting
//!   the cache reproduces byte-identical artifacts (the advisory rule).
//! * Latency: cold, warm, one-change and unchanged rebuilds are
//!   measured on a named project and printed (honesty discipline:
//!   report the numbers, do not assume them).

use cinch::driver::incremental::{read_cache_index, ChangeKind, EdgeKind, Session};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------- fixtures

/// The probe project: `Base` holds a type and two functions, `Lib` calls
/// into `Base` (and keeps one independent declaration), `Main` calls
/// into `Lib`. Every entry source is `module Main where` (house rule).
const BASE: &str = r#"module Base where
  data Count where
    Mk-Count :: Int -> Count
  bump-count :: Count -> Count
  bump-count c = case c of
    Mk-Count n -> Mk-Count (n + 1)
  count-value :: Count -> Int
  count-value c = case c of
    Mk-Count n -> n
"#;

/// A body-only change: `bump-count`'s equation body only (its
/// declaration signature and contract facts hold).
const BASE_BODY_EDIT: &str = r#"module Base where
  data Count where
    Mk-Count :: Int -> Count
  bump-count :: Count -> Count
  bump-count c = case c of
    Mk-Count n -> Mk-Count (n + 2)
  count-value :: Count -> Int
  count-value c = case c of
    Mk-Count n -> n
"#;

/// A contract change: `count-value` gains a verified `#[no-alloc]`
/// fact, which lands in the interface's contract projection.
const BASE_CONTRACT_EDIT: &str = r#"module Base where
  data Count where
    Mk-Count :: Int -> Count
  bump-count :: Count -> Count
  bump-count c = case c of
    Mk-Count n -> Mk-Count (n + 1)
  #[no-alloc]
  count-value :: Count -> Int
  count-value c = case c of
    Mk-Count n -> n
"#;

const LIB: &str = r#"module Lib where
  import Base
  total-count :: Count -> Int
  total-count c = count-value (bump-count c)
  standalone :: Int -> Int
  standalone x = x + 1
"#;

/// A signature change at the one declaration nothing calls.
const LIB_SIG_EDIT: &str = r#"module Lib where
  import Base
  total-count :: Count -> Int
  total-count c = count-value (bump-count c)
  standalone :: Nat -> Nat
  standalone x = x
"#;

const MAIN: &str = r#"module Main where
  import Base
  import Lib
  show-count :: Count -> Int
  show-count c = total-count c
  main :: Int
  main = if show-count (Mk-Count 41) == 42 then 0 else 1
"#;

/// The advisory project: a type, an instance with methods, and a main.
const CLASSY: &str = r#"module Classy where
  data Ticket where
    Ticket :: Int -> Ticket
  ticket-value :: Ticket -> Int
  ticket-value t = case t of
    Ticket n -> n
  bump-ticket :: Ticket -> Ticket
  bump-ticket t = case t of
    Ticket n -> Ticket (n + 1)
"#;

const CLASSY_BODY_EDIT: &str = r#"module Classy where
  data Ticket where
    Ticket :: Int -> Ticket
  ticket-value :: Ticket -> Int
  ticket-value t = case t of
    Ticket n -> n
  bump-ticket :: Ticket -> Ticket
  bump-ticket t = case t of
    Ticket n -> Ticket (n + 2)
"#;

const USE: &str = r#"module Use where
  import Classy
  implicit instance Eq-Ticket :: Eq Ticket where
    (==) x y = case x of
      Ticket a -> case y of
        Ticket b -> a == b
    (/=) x y = case x of
      Ticket a -> case y of
        Ticket b -> a /= b
"#;

const MAIN2: &str = r#"module Main where
  import Classy
  import Use
  main :: Int
  main = if ticket-value (bump-ticket (Ticket 41)) == 42 then 0 else 1
"#;

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("slincr_{}_{}", std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

fn probe_project() -> PathBuf {
    write_project(&[("Base.cinch", BASE), ("Lib.cinch", LIB), ("Main.cinch", MAIN)])
}

fn entry(dir: &Path) -> PathBuf {
    dir.join("Main.cinch")
}

fn key(dir: &Path, name: &str) -> String {
    dir.join(name).display().to_string()
}

fn open(dir: &Path) -> (Session, cinch::driver::incremental::SessionReport) {
    Session::open(&entry(dir)).unwrap_or_else(|e| panic!("open: {e}"))
}

fn set(xs: &[String]) -> BTreeSet<String> {
    xs.iter().cloned().collect()
}

// ---------------------------------------------------------- REAPER: the graph

#[test]
fn graph_edges_are_kinded_from_the_interface_metadata() {
    let dir = probe_project();
    let (mut session, _report) = open(&dir);
    let g = session.graph().expect("graph");

    // Body edges come from the checked declarations' references.
    let e = g.edges_between("v:Lib.total-count", "v:Base.count-value");
    assert!(
        e.iter().any(|e| e.kind == EdgeKind::Body),
        "total-count's body calls count-value: {e:?}"
    );
    let e = g.edges_between("v:Main.main", "v:Base.Mk-Count");
    assert!(
        e.iter().any(|e| e.kind == EdgeKind::Body),
        "main's body constructs Mk-Count: {e:?}"
    );

    // Surface (type/contract) edges come from the schemes and facts.
    let e = g.edges_between("v:Lib.total-count", "t:Base.Count");
    assert!(
        e.iter().any(|e| e.kind == EdgeKind::Surface),
        "total-count's scheme mentions Count: {e:?}"
    );
    let e = g.edges_between("v:Base.Mk-Count", "t:Base.Count");
    assert!(
        e.iter().any(|e| e.kind == EdgeKind::Surface),
        "the constructor unit hangs off its type: {e:?}"
    );

    // The `.cinchif` chain's module spine is surface-labeled.
    let e = g.edges_between("m:Lib.", "m:Base.");
    assert!(
        e.iter().any(|e| e.kind == EdgeKind::Surface),
        "Lib imports Base: {e:?}"
    );

    // Direction queries agree with the edge orientation.
    let deps: Vec<String> = g
        .dependents_of("v:Base.count-value")
        .into_iter()
        .map(|e| e.from.clone())
        .collect();
    assert!(
        deps.contains(&"v:Lib.total-count".to_string()),
        "dependents of count-value: {deps:?}"
    );
}

#[test]
fn body_change_marks_only_its_declaration_dirty() {
    let dir = probe_project();
    let (mut session, _report) = open(&dir);
    let g = session.graph().expect("graph");
    let dirty = g.dirty_closure(&[("v:Base.count-value".to_string(), ChangeKind::Body)]);
    assert_eq!(
        dirty,
        set(&["v:Base.count-value".to_string()]),
        "a body-only change rechecks its declaration alone"
    );
}

#[test]
fn surface_change_marks_the_dependents_closure_dirty() {
    let dir = probe_project();
    let (mut session, _report) = open(&dir);
    let g = session.graph().expect("graph");
    let dirty = g.dirty_closure(&[("v:Base.count-value".to_string(), ChangeKind::Surface)]);
    let expected = set(&[
        "v:Base.count-value".to_string(),
        "m:Base.".to_string(),
        "v:Lib.total-count".to_string(),
        "m:Lib.".to_string(),
        "v:Main.show-count".to_string(),
        "m:Main.".to_string(),
        "v:Main.main".to_string(),
    ]);
    assert_eq!(dirty, expected, "the dependents' closure, pinned");
    // The finesse: untouched declarations stay clean.
    for clean in ["v:Lib.standalone", "v:Base.bump-count", "t:Base.Count"] {
        assert!(!dirty.contains(clean), "{clean} must stay clean");
    }
}

// ---------------------------------------------------------- INCR: the session

#[test]
fn warm_rebuild_touches_nothing_and_reports_kept() {
    let dir = probe_project();
    let (mut session, cold) = open(&dir);
    assert!(!cold.rechecked.is_empty(), "cold rechecks everything");
    let warm = session.rebuild_edits().expect("warm rebuild");
    assert!(warm.rechecked.is_empty(), "warm: {warm:?}");
    assert!(warm.retyped.is_empty(), "warm: {warm:?}");
    assert!(warm.typed_modules.is_empty(), "warm: {warm:?}");
    assert!(
        warm.kept.len() >= 10,
        "warm keeps every user declaration: {:?}",
        warm.kept
    );
    assert!(warm.chain_ok, "warm: {warm:?}");
}

#[test]
fn body_change_rechecks_only_its_declaration() {
    let dir = probe_project();
    let (mut session, _cold) = open(&dir);
    let before = read_cache_index(session.cache_dir());
    session.set_source(&key(&dir, "Base.cinch"), BASE_BODY_EDIT);
    let report = session.rebuild_edits().expect("body rebuild");
    assert_eq!(
        set(&report.rechecked),
        set(&["v:Base.bump-count".to_string()]),
        "body change: {report:?}"
    );
    // The driver's own rebuild manifest carries the same rows (assert
    // via the report/manifest, not timing).
    let manifest = std::fs::read_to_string(dir.join("build").join("Main.rebuild.json"))
        .expect("rebuild manifest");
    assert!(
        manifest.contains("\"rechecked\":[\"v:Base.bump-count\"]"),
        "{manifest}"
    );
    assert!(manifest.contains("\"verified\":true"), "{manifest}");
    assert_eq!(
        set(&report.typed_modules),
        set(&["Base".to_string()]),
        "only the changed module is typed"
    );
    // Untouched modules are kept (never re-typed).
    assert!(report.kept.contains(&"v:Lib.total-count".to_string()));
    assert!(report.kept.contains(&"v:Main.main".to_string()));
    assert!(report.chain_ok, "{report:?}");

    // LINK: the changed unit is re-placed, the rest reused, and the
    // cache keys stay on declaration hashes (the unchanged declarations
    // keep their key across the rebuild).
    let key_of = |rows: &[_], sym: &str| -> Option<String> {
        rows.iter()
            .find(|r: &&cinch::driver::incremental::CacheRow| r.symbol == sym)
            .map(|r| r.decl_hash.clone())
    };
    let stable_before = key_of(&before, "Lib.total-count").expect("Lib.total-count in cache");
    assert!(
        report.link.placed.contains(&"Base.bump-count".to_string()),
        "the changed unit is re-placed: {:?}",
        report.link
    );
    assert!(
        !report.link.reused.is_empty(),
        "unchanged units are reused: {:?} (index rows: {})",
        report.link,
        read_cache_index(session.cache_dir()).len()
    );
    assert!(report.link.verified, "{:?}", report.link);
    let after = read_cache_index(session.cache_dir());
    let stable_after = key_of(&after, "Lib.total-count").expect("Lib.total-count in cache");
    assert_eq!(
        stable_before, stable_after,
        "an unchanged declaration keeps its cache key"
    );
    let changed_after = key_of(&after, "Base.bump-count").expect("Base.bump-count in cache");
    assert_ne!(
        key_of(&before, "Base.bump-count").expect("Base.bump-count in cache"),
        changed_after,
        "the changed declaration takes a new cache key"
    );
}

#[test]
fn contract_change_rechecks_the_dependents_closure() {
    let dir = probe_project();
    let (mut session, _cold) = open(&dir);
    session.set_source(&key(&dir, "Base.cinch"), BASE_CONTRACT_EDIT);
    let report = session.rebuild_edits().expect("contract rebuild");
    let expected = set(&[
        "v:Base.count-value".to_string(),
        "k:Base.count-value#f0".to_string(),
        "m:Base.".to_string(),
        "v:Lib.total-count".to_string(),
        "m:Lib.".to_string(),
        "v:Main.show-count".to_string(),
        "m:Main.".to_string(),
        "v:Main.main".to_string(),
    ]);
    assert_eq!(
        set(&report.rechecked),
        expected,
        "a contract change marks the dependents' closure: {report:?}"
    );
    assert!(
        report
            .changes
            .contains(&("v:Base.count-value".to_string(), ChangeKind::Surface)),
        "{:?}",
        report.changes
    );
    assert!(report.chain_ok, "{report:?}");
}

#[test]
fn signature_change_rechecks_the_dependents_closure() {
    let dir = probe_project();
    let (mut session, _cold) = open(&dir);
    session.set_source(&key(&dir, "Lib.cinch"), LIB_SIG_EDIT);
    let report = session.rebuild_edits().expect("signature rebuild");
    let expected = set(&[
        "v:Lib.standalone".to_string(),
        "m:Lib.".to_string(),
        "m:Main.".to_string(),
    ]);
    assert_eq!(
        set(&report.rechecked),
        expected,
        "a signature change marks the dependents' closure: {report:?}"
    );
    assert!(
        report
            .changes
            .contains(&("v:Lib.standalone".to_string(), ChangeKind::Surface)),
        "{:?}",
        report.changes
    );
    // The recheck re-typed the dirty modules and their context; the
    // untouched declaration of the same module is only retyped context.
    assert!(
        report.retyped.contains(&"v:Lib.total-count".to_string()),
        "{report:?}"
    );
    assert!(report.chain_ok, "{report:?}");
}

// ---------------------------------------------------------- LINK: the advisory rule

/// Every artifact byte of the build directory (the interfaces, the
/// manifest, the stage snapshot, and the linked IR). The rebuild report
/// is excluded on purpose: it is build metadata about what was touched,
/// not artifact identity.
fn artifact_bytes(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let build = dir.join("build");
    let mut out: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for e in std::fs::read_dir(&build).expect("build dir") {
        let e = e.expect("entry");
        if e.file_type().expect("type").is_dir() {
            continue;
        }
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(".rebuild.json") {
            continue;
        }
        out.insert(name, std::fs::read(e.path()).expect("read artifact"));
    }
    out
}

#[test]
fn deleting_the_cache_reproduces_byte_identical_artifacts() {
    let dir = write_project(&[
        ("Classy.cinch", CLASSY),
        ("Use.cinch", USE),
        ("Main.cinch", MAIN2),
    ]);
    // 1. Cold build.
    let (mut session, _cold) = open(&dir);
    // 2. One body change, rebuilt through the warm session (the cache
    //    re-places only the changed units).
    session.set_source(&key(&dir, "Classy.cinch"), CLASSY_BODY_EDIT);
    let report = session.rebuild_edits().expect("incremental rebuild");
    assert!(
        report.link.placed.contains(&"Classy.bump-ticket".to_string()),
        "the changed unit is re-placed: {:?}",
        report.link.placed
    );
    assert!(report.link.verified, "{:?}", report.link);
    let incremental = artifact_bytes(&dir);
    drop(session);
    // The edited source reaches disk (the cold path reads from disk).
    std::fs::write(dir.join("Classy.cinch"), CLASSY_BODY_EDIT).expect("write edit");
    // 3. Delete the cache: it must cost time, never correctness.
    std::fs::remove_dir_all(dir.join("build").join("incr")).expect("delete cache");
    let (_session2, cold2) = open(&dir);
    assert!(cold2.full_rebuild, "a deleted cache rebuilds fully: {cold2:?}");
    let rebuilt = artifact_bytes(&dir);
    let a: BTreeSet<String> = incremental.keys().cloned().collect();
    let b: BTreeSet<String> = rebuilt.keys().cloned().collect();
    assert_eq!(a, b, "the same artifact set");
    for name in &a {
        assert_eq!(
            incremental[name], rebuilt[name],
            "artifact {name} differs after cache deletion"
        );
    }
}

#[test]
fn library_projects_check_without_a_linked_artifact() {
    // A named library from the repository: `examples/source/
    // 02-vector-filter.cinch` has no `main`, so the artifact set is the
    // interfaces and the manifest (the driver's `check` mode) and the
    // link stage reports itself skipped.
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../examples/source/02-vector-filter.cinch");
    let src = std::fs::read_to_string(&example).expect("read 02-vector-filter.cinch");
    let dir = write_project(&[("VectorFilter.cinch", &src)]);
    let (mut session, cold) =
        Session::open(&dir.join("VectorFilter.cinch")).unwrap_or_else(|e| panic!("open: {e}"));
    assert!(cold.link.skipped, "{cold:?}");
    assert!(!cold.rechecked.is_empty(), "{cold:?}");
    let edit = src.replace("(i + 1)", "(i + 2)");
    assert_ne!(src, edit, "the edit is real");
    session.set_source(&key(&dir, "VectorFilter.cinch"), &edit);
    let one = session.rebuild_edits().expect("library rebuild");
    assert!(one.link.skipped, "{one:?}");
    assert_eq!(one.rechecked.len(), 1, "{one:?}");
    assert!(
        one.rechecked[0].starts_with("v:VectorFilter.filterFrom"),
        "{one:?}"
    );
    assert!(one.chain_ok, "{one:?}");
    assert!(
        dir.join("build").join("VectorFilter.manifest.json").exists(),
        "the manifest is an artifact of a library build too"
    );
    // The advisory rule holds for libraries as well: a cold rebuild
    // after cache deletion reproduces the artifacts byte for byte.
    let incremental = artifact_bytes(&dir);
    drop(session);
    std::fs::write(dir.join("VectorFilter.cinch"), &edit).expect("write edit");
    // The object cache exists only when there is a linked artifact; a
    // missing cache trivially costs time, never correctness.
    let _ = std::fs::remove_dir_all(dir.join("build").join("incr"));
    let (_session2, _cold2) = Session::open(&dir.join("VectorFilter.cinch"))
        .unwrap_or_else(|e| panic!("reopen: {e}"));
    let rebuilt = artifact_bytes(&dir);
    assert_eq!(incremental, rebuilt, "library artifacts are cache-independent");
}

// ---------------------------------------------------------- latency: measured

fn measure(label: &str, dir: &Path, edit_file: &str, edit_text: &str) {
    let t0 = std::time::Instant::now();
    let (mut session, cold) = open(dir);
    let cold_wall = t0.elapsed().as_secs_f64() * 1000.0;
    let warm = session.rebuild_edits().expect("warm");
    let warm2 = session.rebuild_edits().expect("unchanged");
    session.set_source(&key(dir, edit_file), edit_text);
    let one = session.rebuild_edits().expect("one change");

    println!("latency on {label} (ms):");
    println!(
        "  cold      wall {cold_wall:8.1}   total {:8.1}  (parse {:7.1}, check {:7.1}, optimize {:7.1}, emit {:7.1}, link {:7.1})",
        cold.timings.total_ms, cold.timings.parse_ms, cold.timings.check_ms, cold.timings.optimize_ms, cold.timings.emit_ms, cold.timings.link_ms
    );
    println!(
        "  warm      total {:8.1}  (check {:7.1})",
        warm.timings.total_ms, warm.timings.check_ms
    );
    println!(
        "  unchanged total {:8.1}  (check {:7.1})",
        warm2.timings.total_ms, warm2.timings.check_ms
    );
    println!(
        "  one-change total {:8.1}  (parse {:7.1}, check {:7.1}, optimize {:7.1}, emit {:7.1}, link {:7.1})",
        one.timings.total_ms, one.timings.parse_ms, one.timings.check_ms, one.timings.optimize_ms, one.timings.emit_ms, one.timings.link_ms
    );
    println!(
        "  one-change touched: rechecked {:?}, typed {:?}",
        one.rechecked, one.typed_modules
    );

    // The honest assertions are on the report, not the clock: the warm
    // and unchanged rebuilds run no checker at all.
    assert!(warm.typed_modules.is_empty(), "{label} warm: {warm:?}");
    assert!(warm2.typed_modules.is_empty(), "{label} unchanged: {warm2:?}");
    assert_eq!(one.rechecked.len(), 1, "{label} one-change: {one:?}");
}

#[test]
fn latency_cold_warm_one_change_unchanged() {
    // The probe project (its cold build is the prelude build too: every
    // check compiles the embedded family) ...
    let dir = probe_project();
    measure("the three-module probe", &dir, "Base.cinch", BASE_BODY_EDIT);

    // ... and a named project from the repository: the bootstrap
    // example `examples/compiler/fib.cinch` (read-only, copied out).
    let example = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../examples/compiler/fib.cinch");
    let fib = std::fs::read_to_string(&example).expect("read fib.cinch");
    let fib_edit = fib.replace(
        "fib (n - 1) + fib (n - 2)",
        "fib (n - 2) + fib (n - 1)",
    );
    assert_ne!(fib, fib_edit, "the edit is real");
    let dir = write_project(&[("Main.cinch", &fib)]);
    measure("examples/compiler/fib.cinch", &dir, "Main.cinch", &fib_edit);
}
