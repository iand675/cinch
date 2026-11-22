use cinch::total::{CheckedGraph, Definition, Expr, Op, Sort};
fn definition(name: &str, parameters: Vec<Sort>, result: Sort, body: Expr) -> Definition {
    Definition { identity: name.into(), parameters, result, body }
}
fn nat(n: &str) -> Expr { Expr::Number { sort: Sort::Nat, decimal: n.into() } }

#[test]
fn body_graph_rechecks_scope_sorts_calls_and_total_dependencies() {
    let identity = definition("id", vec![Sort::Nat], Sort::Nat, Expr::Local(0));
    let call = |target: &str, arguments: Vec<Expr>| Expr::Call { target: target.into(), arguments };
    let valid = definition("use", vec![Sort::Nat], Sort::Nat, Expr::Let {
        value: Box::new(call("id", vec![Expr::Local(0)])), body: Box::new(Expr::Primitive(Op::NatAdd, vec![Expr::Local(1), nat("1")])),
    });
    assert!(CheckedGraph::check(vec![valid.clone(), identity.clone()]).is_ok(), "forward declaration order is irrelevant; dependency cycles are not");
    let bad = [
        definition("use", vec![Sort::Nat], Sort::Nat, Expr::Local(1)),
        definition("use", vec![Sort::Nat], Sort::Bool, Expr::Local(0)),
        definition("use", vec![Sort::Nat], Sort::Nat, call("missing", vec![Expr::Local(0)])),
        definition("use", vec![Sort::Nat], Sort::Nat, call("id", vec![])),
        definition("use", vec![Sort::Bool], Sort::Nat, call("id", vec![Expr::Local(0)])),
        definition("use", vec![], Sort::Nat, nat("-1")),
        definition("use", vec![], Sort::Nat, nat("12x")),
        definition("use", vec![], Sort::I64, Expr::Number { sort: Sort::I64, decimal: "9223372036854775808".into() }),
        definition("use", vec![], Sort::Nat, Expr::If { condition: Box::new(nat("0")), yes: Box::new(nat("1")), no: Box::new(nat("2")) }),
        definition("use", vec![], Sort::Nat, Expr::If { condition: Box::new(Expr::Bool(true)), yes: Box::new(nat("1")), no: Box::new(Expr::Bool(false)) }),
        definition("use", vec![], Sort::Nat, Expr::Primitive(Op::BytesLen, vec![nat("0")])),
        definition("use", vec![], Sort::Nat, Expr::Let { value: Box::new(Expr::Local(0)), body: Box::new(nat("0")) }),
        definition("use", vec![], Sort::Nat, call("use", vec![])),
    ];
    for bad in bad { assert!(CheckedGraph::check(vec![identity.clone(), bad.clone()]).is_err(), "accepted {bad:?}"); }
    assert!(CheckedGraph::check(vec![identity.clone(), identity]).is_err());
    assert!(CheckedGraph::check(vec![definition("a", vec![], Sort::Nat, call("b", vec![])), definition("b", vec![], Sort::Nat, call("a", vec![]))]).is_err());
}

#[test]
fn native_width_has_actual_closed_method_and_primitive_bodies() {
    let source = "module Main where\n  width :: Nat -> Nat\n  width cp = if cp < 128 then 1 else if cp < 2048 then 2 else if cp < 65536 then 3 else 4\n  forward :: Nat -> Nat\n  forward value = width value\n  size :: Bytes -> Nat\n  size bytes = bytes-len bytes\n  main :: Int\n  main = 0\n";
    let program = cinch::driver::check_sources(source, "total-bodies.cinch").expect("check ordinary source bodies");
    let graph = &program.total_bodies;
    let width = graph.get("Main.width").unwrap_or_else(|| panic!("missing width; extracted {:?}", graph.definitions().map(|d| d.identity.as_str()).collect::<Vec<_>>()));
    assert_eq!(width.parameters, [Sort::Nat]); assert_eq!(width.result, Sort::Nat);
    let content = graph.content("Main.width").unwrap();
    assert!(content.contains("NatCmp") && content.contains("I64Lt"), "actual selected method body must reach its fixed primitives: {content}");
    assert!(graph.content("Main.forward").unwrap().contains(&content), "callee bodies participate in caller content");
    assert!(graph.content("Main.size").unwrap().contains("BytesLen"));
    let projected = graph.closure(["Main.width"]).expect("retain the complete selected method closure");
    assert_eq!(projected.content("Main.width"), graph.content("Main.width"));
    assert!(projected.get("Main.forward").is_none());
    assert!(projected.get("Main.size").is_none());
    assert!(projected.get("Data.String.utf8-encode-width").is_none());
    assert!(graph.closure(["Main.width", "missing"]).is_err());
    assert_eq!(CheckedGraph::from_json(&projected.to_json()).unwrap(), projected);
    let actual = graph.get("Data.String.utf8-encode-width").expect("extract the actual remaining measure function");
    assert_eq!(actual.parameters, [Sort::Nat]); assert_eq!(actual.result, Sort::Nat);
    let actual = graph.content("Data.String.utf8-encode-width").unwrap();
    assert!(actual.contains("NatCmp") && actual.contains("I64Lt") && actual.contains("65536"), "{actual}");

    let changed = source.replace("then 1 else", "then 5 else");
    let changed = cinch::driver::check_sources(&changed, "total-bodies.cinch").unwrap();
    assert_ne!(changed.total_bodies.content("Main.width"), graph.content("Main.width"));
    assert_ne!(changed.total_bodies.content("Main.forward"), graph.content("Main.forward"), "ordinary body changes invalidate transitive consumers");
    assert_eq!(changed.total_bodies.content("Main.size"), graph.content("Main.size"));
}

#[test]
fn byte_contents_unknown_foreign_functions_and_recursion_are_not_stable_bodies() {
    let source = r#"module Main where
  import Data.Bytes (bytes-get, bytes-len)
  foreign import ccall "external_identity" external :: I64 ->{Diverge} I64
  unknown :: I64 ->{Diverge} I64
  unknown value = external value
  loop :: Int -> Int
  loop value = loop value
  read-content :: using (bytes-len bytes > 0) => (bytes :: Bytes) -> Nat
  read-content bytes = bytes-get bytes 0
  small :: U8
  small = 7
  main :: Int
  main = 0
"#;
    let program = cinch::driver::check_sources(source, "total-refusals.cinch").expect("check bodies that may not become stable observations");
    for name in ["Main.unknown", "Main.loop", "Main.read-content", "Main.small"] {
        assert!(program.total_bodies.get(name).is_none(), "admitted unchecked {name}");
    }
}

#[test]
fn closed_selected_ordering_retains_its_actual_body_and_unknown_providers_stay_opaque() {
    let original = include_str!("../prelude/Data/Nat.cinch");
    let method = "(<) x y = lt-i64 (nat-cmp x y) 0";
    assert_eq!(original.matches(method).count(), 1);
    // Replace the defining module in the embedded family, preserving its
    // dependency position. Treating Data.Nat as a user module would instead
    // remove the home needed while the rest of the family is checked.
    let check_method = |body: &str| {
        let mut family = cinch::driver::parse_prelude(&[]).unwrap();
        let replacement = cinch::driver::parse_source(
            &original.replace(method, body), "prelude/Data/Nat.cinch",
            &family.base, &family.exported_fixities,
        ).unwrap();
        assert_eq!(replacement.modules.len(), 1);
        let home = family.modules.iter_mut().find(|m| m.name.text == "Data.Nat").unwrap();
        *home = replacement.modules.into_iter().next().unwrap();
        let user = cinch::driver::parse_source(r#"module Main where
  selected :: Nat -> Nat
  selected n = if n < 128 then 1 else 2
  unknown :: using (order :: Ord Nat) => Nat -> Nat
  unknown n = if n < 128 then 1 else 2
  main :: Int
  main = 0
"#, "total-selected.cinch", &family.base, &family.exported_fixities).unwrap();
        let mut context = cinch::driver::StagingCx::default();
        context.fixities = family.fixities;
        context.fixities.extend(user.fixities);
        cinch::check::check_program_multi(&family.modules, &user.modules,
            &std::collections::HashMap::new(), &context).unwrap().0
    };
    let program = check_method("(<) x y = False");
    let graph = program.total_bodies;
    let selected = graph.content("Main.selected").expect("follow the actual closed dictionary field");
    assert!(selected.contains("Ord_Nat.<") && selected.contains("Bool(false)"), "{selected}");
    assert!(!selected.contains("NatCmp"), "the ordinary Ord type/slot is no numeric authority: {selected}");
    assert!(graph.get("Main.unknown").is_none());
    let changed = check_method("(<) x y = True");
    assert_ne!(changed.total_bodies.content("Main.selected"), Some(selected));
}

#[test]
fn incremental_replacement_removes_stale_transitive_authority() {
    let call = Expr::Call { target: "callee".into(), arguments: vec![Expr::Local(0)] };
    let caller = definition("caller", vec![Sort::Nat], Sort::Nat, call);
    let original = definition("callee", vec![Sort::Nat], Sort::Nat, nat("1"));
    let changed = definition("callee", vec![Sort::Nat], Sort::Nat, nat("2"));
    let before = CheckedGraph::closed_subset(vec![caller.clone(), original.clone()]);
    let after = CheckedGraph::closed_subset(vec![caller.clone(), changed.clone()]);
    assert_ne!(before.content("caller"), after.content("caller"));
    assert!(CheckedGraph::closed_subset(vec![caller.clone()]).get("caller").is_none());
    assert!(CheckedGraph::closed_subset(vec![caller, original, changed]).get("caller").is_none(), "conflicting defining bodies cannot share authority");
}

#[test]
fn total_artifact_round_trip_rejects_unrecognized_semantics() {
    let graph = CheckedGraph::check(vec![definition("value", vec![], Sort::Nat, nat("340282366920938463463374607431768211507"))]).unwrap();
    let text = graph.to_json();
    assert_eq!(CheckedGraph::from_json(&text).unwrap(), graph);
    for malformed in [
        text.replacen("\"version\":1", "\"version\":2", 1),
        text.replacen("\"version\":1", "\"version\":1,\"version\":1", 1),
        text.replacen("\"version\":1", "\"version\":1,\"trust-me\":true", 1),
        text.replace("\"result\":\"Nat\"", "\"result\":\"Bool\""),
        text.replace("\"node\":\"number\"", "\"node\":\"foreign\""),
    ] { assert!(CheckedGraph::from_json(&malformed).is_err(), "accepted {malformed}"); }
}

#[test]
fn a_caller_retypes_changed_body_contracts_and_loses_cyclic_authority() {
    let dir = std::env::temp_dir().join(format!("cinch_total_incremental_{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let a = "module A where\n  width :: Nat -> Nat\n  width n = 1\n";
    std::fs::write(dir.join("A.cinch"), a).unwrap();
    std::fs::write(dir.join("Main.cinch"), "module Main where\n  import A\n  forward :: Nat -> Nat\n  forward n = width n\n  main :: Int\n  main = 0\n").unwrap();
    let (mut session, _) = cinch::driver::incremental::Session::open(&dir.join("Main.cinch")).unwrap();
    let before = session.total_bodies().unwrap().content("Main.forward").expect("cold caller body");
    let stable = session.rebuild().unwrap();
    assert!(stable.kept.iter().any(|id| id == "v:Main.forward"), "an unchanged caller stays cached: {:?}", stable.kept);
    std::fs::write(dir.join("A.cinch"), a.replace("= 1", "= 2")).unwrap();
    let report = session.rebuild().unwrap();
    // Exported checked bodies are proof-relevant interface contracts. The
    // unchanged caller is retyped against the changed imported contract,
    // while its own declaration is not a changed/rechecked item.
    assert!(report.retyped.iter().any(|id| id == "v:Main.forward"), "the caller must reread the changed body contract: {:?}", report);
    assert!(!report.rechecked.iter().any(|id| id == "v:Main.forward"), "the caller declaration itself is unchanged");
    let changed = session.total_bodies().unwrap().content("Main.forward").expect("warm caller body");
    assert_ne!(before, changed, "the ordinary caller must not retain an old semantic dependency");
    std::fs::write(dir.join("A.cinch"), a.replace("= 1", "= width n")).unwrap();
    session.rebuild().unwrap();
    let current = session.total_bodies().unwrap();
    assert!(current.get("A.width").is_none());
    assert!(current.get("Main.forward").is_none(), "a pure-looking but cyclic replacement removes cached dependent proof authority");
    std::fs::remove_dir_all(dir).unwrap();
}
