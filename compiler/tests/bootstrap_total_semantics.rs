//! Numeric reflection follows actual bodies rather than class identities/slots.
mod common;
#[path = "common/numeric_provider.rs"]
mod numeric_provider;

fn checked_with_nat_method(source: &str, method_body: &str) -> cinch::core::Program {
    let original = include_str!("../prelude/Data/Nat.cinch");
    let method = "(<) x y = lt-i64 (nat-cmp x y) 0";
    assert_eq!(original.matches(method).count(), 1);
    let mut family = numeric_provider::family().unwrap();
    numeric_provider::replace_method(&mut family, "Data.Nat", original, method, method_body).unwrap();
    let user = cinch::driver::parse_source(source, "numeric-bodies.cinch", &family.base, &family.exported_fixities).unwrap();
    let mut context = cinch::driver::StagingCx::default();
    context.fixities = family.fixities;
    context.fixities.extend(user.fixities);
    cinch::check::check_program_multi(&family.modules, &user.modules, &std::collections::HashMap::new(), &context).unwrap().0
}

#[test]
fn compiled_cinch_reflects_only_actual_numeric_equations() {
    let dir = common::project("numeric_body_semantics", &[
        ("Bootstrap.TotalSemantics.cinch", include_str!("../../bootstrap/Bootstrap.TotalSemantics.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.TotalSemantics.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "numeric-body-semantics").expect("compile body semantic recognizer");
    let source = r#"module Main where
  add :: Nat -> Nat -> Nat
  add a b = a + b
  multiply :: Nat -> Nat -> Nat
  multiply a b = a * b
  subtract :: Int -> Int -> Int
  subtract a b = a - b
  less :: Nat -> Nat -> Bool
  less a b = a < b
  swapped :: Nat -> Nat -> Bool
  swapped a b = b < a
  constant :: Nat -> Nat -> Bool
  constant a b = False
  altered :: Nat -> Nat -> Nat
  altered a b = a + 2 * b
  alias :: Nat -> Nat -> Nat
  alias a b = let same = a in same + b
  forward :: Nat -> Nat -> Nat
  forward a b = add a b
  main :: Int
  main = 0
"#;
    let roots = ["Main.add", "Main.multiply", "Main.subtract", "Main.less", "Main.swapped", "Main.constant", "Main.altered", "Main.alias", "Main.forward"];
    for (label, method, comparison) in [
        ("actual ordinary Nat order", "(<) x y = lt-i64 (nat-cmp x y) 0", "<"),
        ("same defining class and slot but different actual body", "(<) x y = False", "opaque"),
    ] {
        let checked = checked_with_nat_method(source, method);
        std::fs::write(dir.join("bodies.json"), checked.total_bodies.closure(roots).unwrap().to_json()).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "{label}; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert_eq!(common::read_output(&dir, "semantics.txt"), format!("Main.add:+\nMain.multiply:*\nMain.subtract:-\nMain.less:{comparison}\nMain.swapped:opaque\nMain.constant:opaque\nMain.altered:opaque\nMain.alias:+\nMain.forward:+\n"), "{label}");
    }
    common::cleanup(&dir);
}


#[test]
fn compiled_cinch_reads_closed_bodies_without_inventing_unknown_branch_results() {
    use cinch::total::{CheckedGraph, Definition, Expr, Op, Sort};
    fn def(name: &str, parameters: Vec<Sort>, result: Sort, body: Expr) -> Definition {
        Definition { identity: name.into(), parameters, result, body }
    }
    let n = |value: &str| Expr::Number { sort: Sort::Nat, decimal: value.into() };
    let call = |target: &str| Expr::Call { target: target.into(), arguments: vec![Expr::Local(0)] };
    let choice = |no| Expr::If { condition: Box::new(Expr::Local(0)), yes: Box::new(Expr::Local(1)), no: Box::new(no) };
    let mut definitions = vec![
        def("identity", vec![Sort::Nat], Sort::Nat, Expr::Local(0)),
        def("forward", vec![Sort::Nat], Sort::Nat, Expr::Let {
            value: Box::new(call("identity")),
            body: Box::new(Expr::Primitive(Op::NatAdd, vec![Expr::Local(1), n("1")])),
        }),
        def("choose", vec![Sort::Bool, Sort::Nat], Sort::Nat, choice(n("9"))),
        def("same", vec![Sort::Bool, Sort::Nat], Sort::Nat, choice(Expr::Local(1))),
        def("equal", vec![Sort::Nat, Sort::Nat], Sort::Bool, Expr::Primitive(Op::I64Eq, vec![
            Expr::Primitive(Op::NatCmp, vec![Expr::Local(0), Expr::Local(1)]),
            Expr::Number { sort: Sort::I64, decimal: "0".into() },
        ])),
        def("length", vec![Sort::Bytes], Sort::Nat, Expr::Primitive(Op::BytesLen, vec![Expr::Local(0)])),
        def("bytes", vec![Sort::Bytes], Sort::Bytes, Expr::Local(0)),
    ];
    definitions.push(def("fuel0", vec![Sort::Nat], Sort::Nat, Expr::Local(0)));
    for depth in 1..14 {
        let prior = format!("fuel{}", depth - 1);
        definitions.push(def(&format!("fuel{depth}"), vec![Sort::Nat], Sort::Nat,
            Expr::Primitive(Op::NatAdd, vec![call(&prior), call(&prior)])));
    }
    definitions.push(def("exhausted", vec![Sort::Nat], Sort::Nat, call("fuel13")));
    let graph = CheckedGraph::check(definitions.clone()).expect("independently check every graph sort and dependency");
    let dir = common::project("total_body_readings", &[
        ("Bootstrap.TotalSemantics.cinch", include_str!("../../bootstrap/Bootstrap.TotalSemantics.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.TotalBodyReadings.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "total-body-readings").expect("compile closed-body evaluator");
    let run = |data: &str, expected: i32| {
        std::fs::write(dir.join("bodies.json"), data).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir)
            .env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{}\n{}", dir.display(), format!("{}\n{}", String::from_utf8_lossy(&output.stderr), std::fs::read_to_string(dir.join("method-provider-error.txt")).unwrap_or_default()));
        assert!(String::from_utf8_lossy(&output.stderr).contains("cinch-leak: live=0 immortal=0 leaked=0"));
    };
    run(&graph.to_json(), 0);
    let report = common::read_output(&dir, "readings.txt");
    assert_eq!(report.lines().count(), 13, "{report}");
    assert!(report.lines().all(|line| line.ends_with(":passed")), "{report}");
    definitions.iter_mut().find(|d| d.identity == "identity").unwrap().body = n("7");
    run(&CheckedGraph::check(definitions).unwrap().to_json(), 0);
    assert!(common::read_output(&dir, "readings.txt").contains("identity:failed"), "changing the real definition must change its reading");
    run(&graph.to_json().replace("\"target\":\"identity\"", "\"target\":\"missing\""), 4);
    run(&graph.to_json().replace("\"operation\":\"NatAdd\"", "\"operation\":\"AssumeResult\""), 4);
    common::cleanup(&dir);
}


#[test]
fn compiled_cinch_method_facts_require_the_actual_selected_implementation() {
    let dir = common::project("method_body_authority", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.MethodBodyAuthority.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "method-body-authority").expect("compile selected-method authority checker");
    let original = include_str!("../prelude/Data/Nat.cinch");
    let method = "(<) x y = lt-i64 (nat-cmp x y) 0";
    assert_eq!(original.matches(method).count(), 1);
    for (body, wanted) in [(method, true), ("(<) x y = False", false)] {
        let mut family = numeric_provider::family().unwrap();
        numeric_provider::replace_method(&mut family, "Data.Nat", original, method, body).unwrap();
        let user = cinch::driver::parse_source("module Main where\n  main :: Int\n  main = 0\n", "method-provider.cinch", &family.base, &family.exported_fixities).unwrap();
        let mut context = cinch::driver::StagingCx::default(); context.fixities = family.fixities; context.fixities.extend(user.fixities);
        let (program, interfaces, _) = cinch::check::check_program_multi(&family.modules, &user.modules, &std::collections::HashMap::new(), &context).unwrap();
        assert!(program.total_bodies.definitions().any(|definition| definition.identity == "Data.String.utf8-encode-width"), "the unrelated definition must actually exist before checking its exclusion from the method closure");
        let folder = dir.join(if wanted { "ordinary" } else { "changed" }); std::fs::create_dir_all(&folder).unwrap();
        let rows = interfaces.iter().filter(|i| i.module != "Main").map(|iface| {
            let path = folder.join(format!("{}.cinchif", iface.module)); std::fs::write(&path, iface.to_json()).unwrap();
            cinch::lsp::Json::Arr(vec![cinch::lsp::Json::Str(iface.module.clone()), cinch::lsp::Json::Str(iface.iface_hash()), cinch::lsp::Json::Str(path.display().to_string())])
        }).collect();
        let manifest = cinch::lsp::Json::obj(vec![("schema", cinch::lsp::Json::Num(1.0)), ("interfaces", cinch::lsp::Json::Arr(rows))]);
        std::fs::write(dir.join("interfaces.json"), manifest.to_text()).unwrap();
        std::fs::write(dir.join("expect-comparison.txt"), if wanted { "actual numeric comparison" } else { "" }).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir)
            .env("CINCH_HEAP_LIMIT", "256M").env("CINCH_RC_DEBUG", "0").env("CINCH_LEAK_CHECK", "1").output().unwrap();
        assert_eq!(output.status.code(), Some(0), "actual selected body {body}; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("cinch-leak: live=0 immortal=0 leaked=0"));
    }
    common::cleanup(&dir);
}
