//! Interface types are printed by Cinch and consumed as source by Native.
mod common;
#[test]
fn checked_schemes_have_portable_definition_owned_syntax() {
    let dir = common::project("export_types", &[
        ("Bootstrap.ExportType.cinch", include_str!("../../bootstrap/Bootstrap.ExportType.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Nominal.cinch", include_str!("../../bootstrap/Bootstrap.Nominal.cinch")),
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ExportType.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "export-types").expect("compile actual Cinch type exporter");
    let run = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    assert_eq!(run.status.code(), Some(0), "{}", String::from_utf8_lossy(&run.stderr));
    let report = common::read_output(&dir, "export-types.txt");
    for line in report.lines() {
        let (name, scheme) = line.split_once('\t').expect("named type export");
        let body = match name {
            "source-quantifier" => { assert!(scheme.starts_with("forall (a :: Type)."), "{scheme}"); "probe x = x" }
            "source-application" => { assert!(scheme.contains("A.Vec 0 a"), "{scheme}"); "probe x = x" }
            "nested-source-application" => { assert!(scheme.contains("A.Wrap (A.Vec 0 a)"), "{scheme}"); "probe x = x" }
            "argument-positions" => { assert!(scheme.contains("0 <= arg0") && scheme.contains("arg0 < arg1"), "{scheme}"); "probe x y = x" }
            "display-binder-collision" => { assert!(scheme.contains("arg0-r <= arg0"), "{scheme}"); "probe x = x" }
            "result-collision" => { assert!(scheme.contains("cinch-result-r == cinch-result"), "{scheme}"); "probe x = x" }
            "argument-name" => { assert!(scheme.contains("cinch-result <= limit"), "{scheme}"); "probe x = x" }
            "nat" | "polymorphic" | "dependent" | "refinement" | "nominal" => "probe x = x",
            "literal-proof" | "generic-proof" => "probe h = Yes h",
            "proof-effects" => "probe h = 0",
            "region" => "probe u = 0",
            "scope" => "probe x = x",
            "task" => "probe u = probe u",
            "source-identity" => { assert_eq!(scheme, "A.Token"); continue; }
            "escaped-identity" => { assert_eq!(scheme, "Bootstrap.Infer.Bootstrap.Infer.add-error"); continue; }
            "spelling" => { assert_eq!(scheme, "i.Bootstrap$2eInfer$2eBootstrap$2eInfer$2eadd$2derror"); continue; }
            "escaped-spelling" => { assert_eq!(scheme, "i.Data$2eSym$2e$24"); continue; }
            "scalar-spelling" => { assert_eq!(scheme, "i.Data$2eSym$2e$e2$86$92"); continue; }
            // `!` and `?` are not plain identity characters, so the
            // spelling is the lowercase hex of each byte: `$21` and `$3f`.
            "bang-spelling" => { assert_eq!(scheme, "Main.print!"); continue; }
            "question-spelling" => { assert_eq!(scheme, "i.Main$2eempty$3f"); continue; }
            "alpha-stable" => { assert_eq!(scheme, "yes"); continue; }
            "unsolved" | "escaped" | "malformed-identity" | "independent-row-tails" | "proof-domain-not-proposition" | "proof-row-not-effects" | "early-proof-precondition" => {
                assert!(scheme.starts_with("ERROR:"), "{name}: {scheme}"); continue;
            }
            _ => panic!("unexpected exporter result {line}"),
        };
        assert!(!scheme.starts_with("ERROR:"), "{name}: {scheme}");
        if matches!(name, "region" | "scope" | "task" | "proof-effects") {
            // Region kind annotations are part of resolved interface syntax.
            // Compare with Native's own checked interface production rather
            // than interpreting that internal spelling as a source kind.
            let (_, interfaces) = cinch::driver::check_sources_ifaces(
                match name {
                    "region" => "module Main where\n  probe :: forall {q0}. using (d0 :: Io q0) => Unit ->{IO q0} Int\n  probe u = 0\n  main :: Int\n  main = 0\n",
                    "proof-effects" => "module Main where\n  probe :: forall {q0}. using (d0 :: Io q0) => False ->{IO q0} Int\n  probe h = 0\n  main :: Int\n  main = 0\n",
                    "scope" => "module Main where\n  probe :: forall {q0} {q1}. Scope q0 q1 -> Scope q0 q1\n  probe x = x\n  main :: Int\n  main = 0\n",
                    "task" => "module Main where\n  probe :: forall {q0} q1 q2. Unit -> Task q0 q1 q2\n  probe u = probe u\n  main :: Int\n  main = 0\n",
                    _ => unreachable!(),
                },
                "native-region-interface-oracle.cinch",
            ).unwrap();
            let main = interfaces.iter().find(|i| i.module == "Main").unwrap();
            let reference = main.exports.values.iter().find(|v| v.name == "probe").unwrap()
                .resolved.as_ref().unwrap();
            let compact = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();
            assert_eq!(compact(scheme), compact(&reference.scheme), "region interface contract");
            continue;
        }
        let source = format!("module A where\n  data Token where\n    Token :: Token\n  data Vec (n :: Nat) a where\n    VNil :: Vec 0 a\n  data Wrap a where\n    Wrapped :: a -> Wrap a\nmodule Main where\n  import A\n  import Data.Core\n  probe :: {scheme}\n  {body}\n  main :: Int\n  main = 0\n");
        let checked = cinch::driver::check_sources(&source, "cinch-exported-type.cinch");
        assert!(checked.is_ok(), "Native readback {name}: {scheme}\n{checked:?}\nProject: {}", dir.display());
    }
    // Every row `Run.ExportType.cinch` writes, including the identity
    // escapes and the sigil identifiers.
    assert_eq!(report.lines().count(), 33, "{report}");
    common::cleanup(&dir);
}
