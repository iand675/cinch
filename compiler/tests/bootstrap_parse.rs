
//! Rung 1 of the bootstrap ladder: `Bootstrap.Parse` reads the language's
//! own surface grammar into the shared contract tree.
//!
//! The Cinch program this test runs is
//! `bootstrap/programs/Run.Parse.cinch` (shipped as the entry, with
//! `__DIR__` substituted for the project directory), beside
//! `bootstrap/Bootstrap.Parse.cinch` and the toolkit. It parses the two
//! hand-checked examples and the whole prelude corpus and writes
//! `report.txt`; the test reads that back. The evidence pinned here:
//!
//!   * `examples/compiler/fib.cinch` and `classes.cinch` parse with ZERO
//!     unmodeled nodes, and their declaration names, constructor names
//!     and fixity facts are the hand-listed ones;
//!   * every corpus module (the 55 of
//!     `compiler/prelude/cinch-base.cinchpkg` plus `examples/source/*.cinch`)
//!     is fed through the parser in its OWN process (one module's
//!     allocation per process, while the runtime frees nothing), and its
//!     unmodeled count matches the measured table;
//!   * the agreement relation against the compiler's own parser: for each
//!     corpus module the compiler parses, every declaration name the
//!     compiler finds is also found by `Bootstrap.Parse`, unless the
//!     module has an unmodeled node to account for the miss. A missing
//!     name with no unmodeled node is the silent drop this catches.
//!
//! What is still unmodeled (named, so the boundary is honest): the
//! refinement `type R = refine (x :: T) with p` declarations (including
//! `InRange`'s `Nat`/`Int` index parameters), the class `law` members of
//! the class homes, the few `do`/`with` terms, the anonymous `refined P
//! T` result types and the predicate `using` contexts of the range-carrying
//! signatures (`nat-from-i64`, the widening conversions and integer bounds
//! in `Data.Machine`, the clock readings, the bytes primitives), and any
//! form only `examples/source` (the surface proposals, "not compiled")
//! uses. `EXPECTED_UNMODELED` is the measured residual per module.

mod common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

const AST_SRC: &str = include_str!("../../bootstrap/Bootstrap.Ast.cinch");
const REPORT_SRC: &str = include_str!("../../bootstrap/Bootstrap.Report.cinch");
const PARSE_SRC: &str = include_str!("../../bootstrap/Bootstrap.Parse.cinch");
const RUN: &str = include_str!("../../bootstrap/programs/Run.Parse.cinch");
const FIB: &str = include_str!("../../examples/compiler/fib.cinch");
const CLASSES: &str = include_str!("../../examples/compiler/classes.cinch");

/// Coverage is a number: each corpus module's unmodeled-node count, measured
/// module by module. A regression fails here, and so does an improvement, on
/// purpose: the table is the record of what the parser covers, and the
/// failure message prints the whole measured table to replace this one.
const EXPECTED_UNMODELED: &[(&str, usize)] = &[
    ("Data.Core", 0),
    ("Data.Ordering", 0),
    ("Relation.Eq", 0),
    ("Relation.Ord", 0),
    ("Algebra.Arithmetic", 2),
    ("Algebra.Bits", 0),
    ("Function.Combinators", 0),
    ("Data.Nat", 7),
    ("Data.Int", 2),
    ("Data.Option", 3),
    ("Data.Result", 1),
    ("Data.List", 7),
    ("Data.Enum", 0),
    ("Data.Char", 2),
    ("Data.Bool", 0),
    ("Data.Machine", 52),
    ("Data.Bytes", 10),
    ("Data.Map", 0),
    ("Data.String", 8),
    ("Data.Json", 0),
    ("Data.Producer", 0),
    ("Data.Target", 0),
    ("Data.Path", 0),
    ("Effect.Sequencing", 0),
    ("Effect.Concurrent", 0),
    ("Effect.Clock", 6),
    ("Effect.Fs", 0),
    ("Effect.Process", 3),
    ("Effect.Io", 0),
    ("Data.FromList", 0),
    ("Data.FromMap", 0),
    ("Text.Show", 1),
    ("Data.Sym", 0),
    ("Prelude", 0),
    ("Algebra.Semigroup", 8),
    ("Algebra.Group", 0),
    ("Algebra.Semiring", 0),
    ("Algebra.Monoid", 4),
    ("Category.Functor", 3),
    ("Category.Applicative", 2),
    ("Category.Monad", 3),
    ("Category.Alternative", 0),
    ("Category.Foldable", 0),
    ("Category.Traversable", 0),
    ("Data.Identity", 0),
    ("Data.Const", 2),
    ("Data.Compose", 0),
    ("Data.NonEmpty", 2),
    ("Data.Sorted", 1),
    ("Data.NonZero", 0),
    ("Data.Ascii", 2),
    ("Data.Set", 0),
    ("Data.Hash", 0),
    ("Text.Codec", 0),
    ("Data.HashMap", 0),
    ("01-bounded-index", 0),
    ("02-vector-filter", 2),
    ("03-map-fusion", 3),
    ("04-productive-stream", 5),
    ("05-schema-generator", 3),
    ("06-counter-upgrade", 6),
    ("07-rejected-programs", 3),
    ("08-platform-simd", 1),
    ("09-foreign-codec", 2),
    ("10-wasm-component", 2),
    ("11-structured-tasks", 1),
    ("12-runtime-effects", 2),
];

/// A corpus module: its declared name, the path the test writes it under,
/// and the source.
fn corpus() -> Vec<(String, String, String)> {
    let mut out: Vec<(String, String, String)> = Vec::new();
    for (path, src) in cinch::driver::prelude_sources() {
        let name = module_name(src).unwrap_or_else(|| path.to_string());
        out.push((name, path.to_string(), src.to_string()));
    }
    let exdir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../examples/source");
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&exdir)
        .unwrap_or_else(|e| panic!("read {}: {e}", exdir.display()))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "cinch").unwrap_or(false))
        .collect();
    entries.sort();
    for p in entries {
        let src =
            std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()));
        let key = p.file_stem().unwrap().to_string_lossy().into_owned();
        // A relative path: the copy lands under the project's `corpus/`,
        // never back in the repository (joining an absolute path replaces
        // the directory it is joined onto).
        out.push((key.clone(), format!("examples/source/{key}"), src));
    }
    out
}

/// The `module X` header's name, from the source, without a full parse.
fn module_name(src: &str) -> Option<String> {
    cinch::driver::scan_file(src, "<scan>")
        .ok()
        .and_then(|s| s.modules.into_iter().next().map(|(m, _)| m))
}

/// One module's parse result from the report.
#[derive(Default)]
struct ModuleFacts {
    unmodeled: usize,
    failed: bool,
    decls: BTreeSet<String>,
    ctors: BTreeSet<String>,
    fixities: BTreeSet<(String, usize, String)>,
}

fn parse_report(text: &str) -> (BTreeMap<String, ModuleFacts>, Vec<String>) {
    let mut mods: BTreeMap<String, ModuleFacts> = BTreeMap::new();
    let mut order: Vec<String> = Vec::new();
    let touch = |mods: &mut BTreeMap<String, ModuleFacts>, name: &str| {
        mods.entry(name.to_string()).or_default();
    };
    for line in text.lines() {
        let f: Vec<&str> = line.split_whitespace().collect();
        match f.as_slice() {
            ["D", module, name] => {
                touch(&mut mods, module);
                mods.get_mut(*module).unwrap().decls.insert(name.to_string());
            }
            ["C", module, ctor] => {
                touch(&mut mods, module);
                mods.get_mut(*module).unwrap().ctors.insert(ctor.to_string());
            }
            ["X", module, assoc, prec, op] => {
                touch(&mut mods, module);
                mods.get_mut(*module)
                    .unwrap()
                    .fixities
                    .insert((assoc.to_string(), prec.parse().unwrap(), op.to_string()));
            }
            [module, "unmodeled", n] => {
                touch(&mut mods, module);
                mods.get_mut(*module).unwrap().unmodeled = n.parse().unwrap();
            }
            [module, "failed", ..] => {
                touch(&mut mods, module);
                mods.get_mut(*module).unwrap().failed = true;
            }
            _ => {}
        }
        if let Some(first) = f.first() {
            if !order.contains(&first.to_string()) {
                order.push(first.to_string());
            }
        }
    }
    (mods, order)
}

/// A declaration's name, as the compiler's own parser sees it.
fn decl_name(d: &cinch::ast::Decl) -> Option<String> {
    use cinch::ast::Decl::*;
    match d {
        Signature(s) => Some(s.name.text.clone()),
        Instance(i) => Some(i.name.text.clone()),
        Data(x) => Some(x.name.text.clone()),
        Record(x) => Some(x.name.text.clone()),
        Codata(x) => Some(x.name.text.clone()),
        ValueSig { name, .. } => Some(name.text.clone()),
        Equation { eq, .. } => Some(eq.name.text.clone()),
        Foreign(x) => Some(x.name.text.clone()),
        Refine(x) => Some(x.name.text.clone()),
        OpaqueType(x) => Some(x.name.text.clone()),
        Effect(x) => Some(x.name.text.clone()),
        Attributed { inner, .. } => decl_name(inner),
        _ => None,
    }
}

/// Every top-level declaration name the compiler's parser finds in a
/// corpus module, or `None` when the compiler itself refuses the source.
fn compiler_names(
    base: &cinch::parser::FixityTable,
    tables: &std::collections::HashMap<String, cinch::parser::FixityTable>,
    src: &str,
    path: &str,
) -> Option<BTreeSet<String>> {
    let parsed = cinch::driver::parse_source(src, path, base, tables).ok()?;
    let mut out = BTreeSet::new();
    for m in &parsed.modules {
        for d in &m.decls {
            if let Some(n) = decl_name(d) {
                out.insert(n);
            }
        }
    }
    Some(out)
}

/// The built parse program, in a project holding only its own sources: the
/// examples and the corpus are written after the build, because every
/// `.cinch` beside the entry joins the project (and corpus copies named like
/// prelude modules would shadow the embedded family).
fn built_program(tag: &str) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &[]);
    let lit = format!("{:?}", dir.to_string_lossy());
    std::fs::write(dir.join("Bootstrap.Ast.cinch"), AST_SRC).expect("write ast");
    std::fs::write(dir.join("Bootstrap.Report.cinch"), REPORT_SRC).expect("write report");
    std::fs::write(dir.join("Bootstrap.Parse.cinch"), PARSE_SRC).expect("write parse module");
    std::fs::write(dir.join("Main.cinch"), RUN.replace("__DIR__", &lit)).expect("write run");
    let exe = common::build(&dir, "Main.cinch", "parse")
        .unwrap_or_else(|e| panic!("building the parse program failed: {e}"));
    std::fs::write(dir.join("fib.cinch"), FIB).expect("write fib");
    std::fs::write(dir.join("classes.cinch"), CLASSES).expect("write classes");
    (dir, exe)
}

/// One run of the program over one corpus module (or none), in its own
/// process: its exit code, its stderr (the `cinch-alloc` line among it) and
/// the report it wrote.
fn run_one(dir: &std::path::Path, exe: &std::path::Path, module: Option<(&str, &str, &str)>) -> (Option<i32>, String, String) {
    let mut manifest = String::new();
    if let Some((name, generated, src)) = module {
        let file = dir.join("corpus").join(format!("{generated}.cinch"));
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, src).unwrap_or_else(|e| panic!("write {}: {e}", file.display()));
        manifest.push_str(name);
        manifest.push('\t');
        manifest.push_str(&file.to_string_lossy());
        manifest.push('\n');
    }
    std::fs::write(dir.join("corpus.txt"), manifest).expect("write manifest");
    let _ = std::fs::remove_file(dir.join("report.txt"));
    let out = std::process::Command::new(exe)
        .current_dir(dir)
        .env("CINCH_ALLOC_STATS", "1")
        .output()
        .expect("run the parse program");
    let report = std::fs::read_to_string(dir.join("report.txt")).unwrap_or_default();
    (out.status.code(), String::from_utf8_lossy(&out.stderr).into_owned(), report)
}

#[test]
fn the_parser_reads_the_examples_and_agrees_on_the_corpus() {
    let corpus = corpus();
    let (dir, exe) = built_program("parse");

    // 1. the examples, alone: zero unmodeled nodes and the hand-listed facts.
    let (code, stderr, report) = run_one(&dir, &exe, None);
    assert_eq!(code, Some(0), "the examples run must exit 0\nstderr: {stderr}\nreport:\n{report}");
    let (mods, _) = parse_report(&report);
    let fib = mods.get("FIB").expect("FIB section");
    let classes = mods.get("CLASSES").expect("CLASSES section");
    assert_eq!(fib.unmodeled, 0, "fib.cinch must parse with zero unmodeled nodes\n{report}");
    assert_eq!(classes.unmodeled, 0, "classes.cinch must parse with zero unmodeled nodes\n{report}");
    for name in ["fib", "main"] {
        assert!(fib.decls.contains(name), "fib.cinch declaration `{name}` missing");
    }
    for name in ["Box", "|+|", "Wrap", "Box_Wrap", "twice", "main"] {
        assert!(classes.decls.contains(name), "classes.cinch declaration `{name}` missing");
    }
    assert!(classes.ctors.contains("W"), "classes.cinch constructor `W` missing");
    assert!(
        classes.fixities.contains(&("infixl".to_string(), 5, "|+|".to_string())),
        "classes.cinch fixity `infixl 5 |+|` missing: {:?}",
        classes.fixities
    );

    // 2. every corpus module in its own process: blocked ones stop at the
    // heap cap, every other one parses, and its unmodeled count is measured.
    let prelude = cinch::driver::parse_prelude(&[]).expect("the embedded prelude parses");
    let mut measured: Vec<(String, usize)> = Vec::new();
    let mut problems: Vec<String> = Vec::new();
    for (name, generated, src) in &corpus {
        let (code, stderr, report) = run_one(&dir, &exe, Some((name, generated, src)));
        let alloc = stderr.lines().find(|l| l.starts_with("cinch-alloc:")).unwrap_or("").to_string();
        let (mods, _) = parse_report(&report);
        let Some(m) = mods.get(name.as_str()) else {
            problems.push(format!("{name}: no report line (exit {code:?}, {alloc})\n{stderr}"));
            continue;
        };
        if m.failed {
            problems.push(format!("{name}: failed to parse (exit {code:?})"));
            continue;
        }
        measured.push((name.clone(), m.unmodeled));
        // 3. agreement with the compiler's own parser as a subset relation:
        // a name the compiler finds and the ladder misses must be accounted
        // for by an unmodeled node, or it is a silent drop.
        if let Some(want) = compiler_names(&prelude.base, &prelude.fixities, src, name) {
            let missing: Vec<&String> = want.difference(&m.decls).collect();
            if missing.len() > m.unmodeled {
                problems.push(format!(
                    "{name}: {} declaration names the compiler finds are absent from Bootstrap.Parse with only {} unmodeled nodes to account for them: {missing:?}",
                    missing.len(),
                    m.unmodeled
                ));
            }
        }
    }
    common::cleanup(&dir);

    let expected: BTreeMap<&str, usize> = EXPECTED_UNMODELED.iter().copied().collect();
    let got: BTreeMap<&str, usize> = measured.iter().map(|(n, c)| (n.as_str(), *c)).collect();
    if expected != got {
        let table: String = measured
            .iter()
            .map(|(n, c)| format!("    ({n:?}, {c}),\n"))
            .collect();
        problems.push(format!(
            "the measured unmodeled counts differ from EXPECTED_UNMODELED; the measured table:\nconst EXPECTED_UNMODELED: &[(&str, usize)] = &[\n{table}];"
        ));
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
