//! Real checked interface output is parsed and chained by the independent compiler.
mod common;

#[test]
fn source_interfaces_preserve_private_nominals_and_defining_fixities() {
    let dir = common::project("export_interfaces", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Export.cinch", include_str!("../../bootstrap/Bootstrap.Export.cinch")),
        ("Bootstrap.ExportType.cinch", include_str!("../../bootstrap/Bootstrap.ExportType.cinch")),
        ("Bootstrap.ExportNominal.cinch", include_str!("../../bootstrap/Bootstrap.ExportNominal.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ExportInterfaces.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "export-interfaces").expect("compile actual Cinch interface writer");
    let run = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    let error = std::fs::read_to_string(dir.join("export-error.txt")).unwrap_or_default();
    assert_eq!(run.status.code(), Some(0), "{error}\n{}\nProject: {}", String::from_utf8_lossy(&run.stderr), dir.display());
    let report = common::read_output(&dir, "exported-interfaces.jsonl");
    let interfaces: Vec<_> = report.lines().map(|line| {
        let interface = cinch::interface::Iface::from_json(line).expect("independent schema8 readback");
        assert_eq!(interface.to_json(), line, "canonical serialization");
        interface
    }).collect();
    assert_eq!(interfaces.len(), 3);
    let source = std::fs::read_to_string(dir.join("export-fixture.cinch")).unwrap();
    for (index, interface) in interfaces.iter().enumerate() {
        let begin = source.find(&format!("module {} where", interface.module)).unwrap();
        let end = interfaces.get(index + 1)
            .map(|next| source.find(&format!("module {} where", next.module)).unwrap())
            .unwrap_or(source.len());
        assert_eq!(interface.source_hash, cinch::interface::content_hash(&source[begin..end]), "complete source identity for {}", interface.module);
    }
    cinch::interface::verify_chain(&interfaces).expect("exact dependency identities");
    let a = interfaces.iter().find(|i| i.module == "A").unwrap();
    let b = interfaces.iter().find(|i| i.module == "B").unwrap();
    let main = interfaces.iter().find(|i| i.module == "Main").unwrap();
    assert_eq!(main.exports.values.iter().find(|v| v.name == "main").unwrap().symbol, "Main.main");
    assert_eq!(a.exports.fixities.len(), 1);
    assert_eq!(a.exports.fixities[0].op, "<+>");
    assert_eq!(b.exports.fixities.iter().map(|f| (f.op.as_str(), f.assoc.as_str(), f.prec)).collect::<Vec<_>>(),
        [("<#>", "left", 6), ("<+>", "left", 6)]);
    for name in ["<#>", "<+>"] {
        assert_eq!(b.exports.values.iter().find(|v| v.name == name).unwrap().symbol, "A.<+>");
    }
    for interface in [a, b] {
        for (name, identity) in [("inspect-scope", "A.inspect-scope"), ("task-source", "A.task-source")] {
            let value = interface.exports.values.iter().find(|v| v.name == name).unwrap();
            assert_eq!(value.symbol, identity);
            let resolved = value.resolved.as_ref().unwrap();
            assert!(resolved.nominals.is_empty(), "kernel Scope/Task types must not invent defining-module nominals");
            assert!(resolved.scheme.contains(if name == "inspect-scope" { "Scope" } else { "Task" }));
        }
    }
    let public = a.exports.types.iter().find(|t| t.name == "PublicPositive").expect("public defining refinement");
    assert_eq!(public.kind, "refinement");
    let template = public.refinement.as_ref().unwrap();
    assert_eq!((template.binder.as_str(), template.carrier.as_str(), template.predicate.as_str()), ("x", "I64", "0 < x"));
    let renamed = b.exports.types.iter().find(|t| t.name == "Positive").unwrap();
    assert_eq!(renamed.identity, "A.PublicPositive");
    assert_eq!(renamed.refinement, public.refinement);
    assert!(!a.exports.types.iter().any(|t| t.name == "Window" || t.name == "Positive"));
    assert!(!b.exports.types.iter().any(|t| t.name == "Window"));
    let contract = |iface: &cinch::interface::Iface, name: &str| iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone().unwrap();
    let capped = contract(a, "capped");
    assert_eq!(capped, contract(b, "capped"));
    assert!(capped.scheme.contains("cinch-result <= limit"), "{}", capped.scheme);
    let collide = contract(a, "collide");
    assert_eq!(collide, contract(b, "collide"));
    assert!(collide.scheme.contains("cinch-result-r == cinch-result"), "{}", collide.scheme);
    let bounded = contract(a, "bounded");
    assert_eq!(bounded, contract(b, "one"));
    assert_eq!(contract(a, "widen"), contract(b, "wide"));
    assert_eq!(contract(a, "positive"), contract(b, "pos"));
    assert_eq!(bounded.nominals.len(), 1);
    let window = &bounded.nominals[0];
    assert_eq!(window.identity, "A.Window");
    assert_eq!(window.kind, "refinement");
    assert!(window.constructors.is_none());
    assert_eq!(window.parameters.iter().map(|p| (p.name.as_str(), p.kind.as_str())).collect::<Vec<_>>(), [("lo", "Int"), ("hi", "Int"), ("a", "Type")]);
    let template = window.refinement.as_ref().unwrap();
    assert_eq!((template.binder.as_str(), template.carrier.as_str()), ("x", "a"));
    assert!(template.predicate.contains("lo <= x") && template.predicate.contains("x <= hi"));
    let positive = contract(b, "pos");
    assert_eq!(positive.nominals.len(), 1);
    assert_eq!(positive.nominals[0].identity, "A.Positive");
    let parcel = a.exports.types.iter().find(|t| t.name == "PositiveParcel").unwrap();
    let field_contract = parcel.ctors[0].resolved.as_ref().unwrap();
    assert!(field_contract.nominals.iter().any(|n| n.identity == "A.Positive" && n.kind == "refinement"), "constructor field lost private defining refinement");
    let defining = a.exports.signatures.iter().find(|s| s.identity == "A.Refined").unwrap();
    let reexported = b.exports.signatures.iter().find(|s| s.identity == "A.Refined").unwrap();
    let method = defining.methods.iter().find(|m| m.name == "refine").unwrap().resolved.as_ref().unwrap();
    assert!(method.nominals.iter().any(|n| n.identity == "A.Positive" && n.kind == "refinement"), "class method lost private defining refinement");
    assert_eq!(defining.methods[0].resolved, reexported.methods[0].resolved);
    let instance = a.exports.instances.iter().find(|i| i.id == "A.HasWindowBounded").unwrap();
    let instance_contract = instance.resolved.as_ref().unwrap();
    assert!(instance_contract.nominals.iter().any(|n| n.identity == "A.Window" && n.kind == "refinement"), "instance head lost its private defining refinement");
    let mut changed = b.clone();
    changed.exports.values.iter_mut().find(|v| v.name == "one").unwrap().resolved.as_mut().unwrap().nominals[0].refinement.as_mut().unwrap().predicate = "lo < x && x < hi".into();
    assert_ne!(changed.iface_hash(), b.iface_hash(), "private predicate changes invalidate consumers");
    for mutation in 0..4 {
        let mut malformed = b.clone();
        let row = &mut malformed.exports.values.iter_mut().find(|v| v.name == "one").unwrap().resolved.as_mut().unwrap().nominals[0];
        match mutation { 0 => row.refinement = None, 1 => row.constructors = Some(vec![]), 2 => row.refinement.as_mut().unwrap().binder = "lo".into(), _ => row.kind = "opaque".into() }
        assert!(cinch::interface::Iface::from_json(&malformed.to_json()).is_err(), "malformed defining template {mutation}");
    }
    assert_eq!(a.assumptions.primitives, ["cinch.runtime.textSha256"]);
    assert!(b.assumptions.primitives.is_empty());
    assert!(!a.exports.types.iter().any(|t| t.name == "Local"));
    let signal = a.exports.values.iter().find(|v| v.name == "signal").expect("public operation contract");
    assert_eq!(signal.symbol, "A.signal");
    assert_eq!(signal.arity, 1);
    assert!(signal.scheme.contains("A.Notice"), "definition-owned effect label");
    assert!(signal.resolved.is_none(), "schema8 does not pretend an operation is an ordinary function");
    assert!(signal.total_body.is_none());
    assert!(!a.exports.types.iter().any(|t| t.name == "Notice"));
    assert!(!a.exports.types.iter().any(|t| t.name == "PrivateField"));
    let public_layout = a.exports.types.iter().find(|t| t.name == "ParcelWithPrivate").unwrap();
    assert_eq!(public_layout.ctors[0].fields, ["A.PrivateField"]);
    let private = public_layout.ctors[0].resolved.as_ref().unwrap().nominals.iter().find(|n| n.identity == "A.PrivateField").unwrap();
    assert_eq!(private.constructors.as_ref().unwrap(), &["A.PrivateFieldValue"]);

    assert!(!a.exports.values.iter().any(|v| v.name == "local-op" || v.name == "private-digest"));
    let signature = a.exports.signatures.iter().find(|s| s.identity == "A.Give").unwrap();
    assert_eq!(signature.exports, ["Give"]);
    assert_eq!(signature.methods[0].name, "give");
    assert_eq!(signature.methods[0].slot, 0);
    assert_eq!(signature.methods[0].exports, ["give"]);
    let instance = a.exports.instances.iter().find(|i| i.id == "A.GiveNat").unwrap();
    assert_eq!(instance.class, "A.Give");
    assert!(instance.implicit);
    assert_eq!(instance.methods, ["A.GiveNat.give"]);
    assert!(instance.method_contracts.is_empty());
    let parcel = a.exports.types.iter().find(|t| t.name == "Parcel").unwrap();
    assert_eq!(parcel.params, ["q0"]);
    assert_eq!(parcel.ctors.iter().map(|c| (c.name.as_str(), c.tag)).collect::<Vec<_>>(), [("Empty", 0), ("Loaded", 1)]);
    assert!(parcel.ctors[0].fields.is_empty());
    assert_eq!(parcel.ctors[1].fields, ["q0"]);
    assert_eq!(parcel.ctors[1].erased, [false]);
    for interface in [a, b] {
        let token = interface.exports.types.iter().find(|t| t.name == "Token").unwrap();
        assert_eq!(token.identity, "A.Token");
        assert!(token.ctors.is_empty(), "private constructor gained a public name");
        let wrap = interface.exports.values.iter().find(|v| v.name == "wrap").unwrap();
        assert_eq!(wrap.symbol, "A.wrap");
        let nominal = wrap.resolved.as_ref().unwrap().nominals.iter().find(|n| n.identity == "A.Token").unwrap();
        assert_eq!(nominal.constructors.as_ref().unwrap(), &["A.Hidden"]);
    }
    common::cleanup(&dir);
}
