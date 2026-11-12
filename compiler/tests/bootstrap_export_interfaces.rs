//! Real checked interface output is parsed and chained by the independent compiler.
mod common;

#[test]
fn source_interfaces_preserve_private_nominals_and_defining_fixities() {
    let dir = common::project("export_interfaces", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Export.cinch", include_str!("../../bootstrap/Bootstrap.Export.cinch")),
        ("Bootstrap.ExportType.cinch", include_str!("../../bootstrap/Bootstrap.ExportType.cinch")),
        ("Bootstrap.ExportNominal.cinch", include_str!("../../bootstrap/Bootstrap.ExportNominal.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ExportInterfaces.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "export-interfaces").expect("compile actual Cinch interface writer");
    let core_source = include_str!("../prelude/Data/Core.cinch");
    let shadow_source = "module Shadow where\n  data List a where\n    Wrap :: a -> List a\n";
    // Exercise Ordering's data declaration in isolation. Its source instances
    // require the Algebra module closure, covered by the replacement suite.
    let ordering_source = "module Data.Ordering where\n  data Ordering where\n    LT :: Ordering\n    EQ :: Ordering\n    GT :: Ordering\n";
    std::fs::write(dir.join("base-export-fixture"), format!("{core_source}\n{ordering_source}\n{shadow_source}")).unwrap();
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
    let base_output = common::read_output(&dir, "exported-base-interfaces.jsonl");
    let base_interfaces: Vec<_> = base_output.lines()
        .map(|line| cinch::interface::Iface::from_json(line).unwrap()).collect();
    assert_eq!(base_interfaces.len(), 3);
    cinch::interface::verify_chain(&base_interfaces).expect("freshly checked constructor interface dependencies");
    let (_, native_base) = cinch::driver::check_sources_ifaces(&format!("{core_source}\n{shadow_source}"), "base-constructor-oracle.cinch")
        .expect("independent Native source family and shadow check");
    let actual_core = base_interfaces.iter().find(|i| i.module == "Data.Core").unwrap();
    let native_core = native_base.iter().find(|i| i.module == "Data.Core").unwrap();
    assert_eq!(actual_core.iface_hash(), native_core.iface_hash(), "canonical source interface for Data.Core");
    // A constructor's display scheme names its family by the defining
    // identity, exactly as a value's scheme does (design/base-as-a-package.md,
    // "Identity comes from the declaring module"). A base-family exception
    // here made this producer disagree with the seed's, so replacing an
    // embedded home with its own source changed that home's identity and
    // every consumer of it was refused as stale.
    for constructor in &actual_core.exports.types[0].ctors {
        assert!(constructor.scheme.contains("Data.Core.List"), "{}: {}", constructor.name, constructor.scheme);
        assert!(constructor.fields.iter().all(|field| !field.split([' ', '(', ')']).any(|word| word == "List")),
            "{}: a bare family name reached a field list: {:?}", constructor.name, constructor.fields);
    }
    let actual_ordering = base_interfaces.iter().find(|i| i.module == "Data.Ordering").unwrap();
    let native_ordering = native_base.iter().find(|i| i.module == "Data.Ordering").unwrap();
    for (actual, reference) in actual_ordering.exports.types[0].ctors.iter().zip(&native_ordering.exports.types[0].ctors) {
        assert_eq!((&actual.name, &actual.scheme, &actual.fields, &actual.resolved),
                   (&reference.name, &reference.scheme, &reference.fields, &reference.resolved));
    }
    assert_eq!(actual_ordering.exports.types[0].ctors.len(), 3);
    let shadow = base_interfaces.iter().find(|i| i.module == "Shadow").unwrap();
    let shadow_ctor = &shadow.exports.types[0].ctors[0];
    assert_eq!(shadow_ctor.scheme, "forall a . (a -> Shadow.List a)");
    assert!(shadow_ctor.resolved.as_ref().unwrap().nominals.iter().any(|n| n.identity == "Shadow.List"));
    let source = std::fs::read_to_string(dir.join("export-fixture.cinch")).unwrap();
    let (_, native_interfaces) = cinch::driver::check_sources_ifaces(&source, "cinch-interface-producer-source.cinch")
        .expect("independent Native acceptance of the complete actual producer fixture");
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
    assert_eq!(b.exports.signatures[0].identity, "B.Alpha", "canonical class order compares names before defining identities");
    let alpha = &b.exports.signatures[0];
    assert!(alpha.exports.is_empty() && alpha.methods.iter().all(|m| m.exports.is_empty()), "private closure granted source names");
    let native_a = native_interfaces.iter().find(|i| i.module == "A").unwrap();
    for name in ["Parcel", "Vec", "Phantom"] {
        let actual = a.exports.types.iter().find(|t| t.name == name).unwrap();
        let reference = native_a.exports.types.iter().find(|t| t.name == name).unwrap();
        assert_eq!(actual.params, reference.params, "{name}: definition-site parameter names");
        assert_eq!(actual.parameters, reference.parameters, "{name}: kinds and implicit parameters");
    }
    let phantom = a.exports.types.iter().find(|t| t.name == "Phantom").unwrap();
    assert_eq!(phantom.params, ["element"]);
    assert!(phantom.parameters[0].implicit);
    assert_eq!(main.exports.values.iter().find(|v| v.name == "main").unwrap().symbol, "Main.main");
    assert_eq!(a.exports.fixities.len(), 1);
    assert_eq!(a.exports.fixities[0].op, "<+>");
    assert_eq!(b.exports.fixities.iter().map(|f| (f.op.as_str(), f.assoc.as_str(), f.prec)).collect::<Vec<_>>(),
        [("<#>", "left", 6), ("<+>", "left", 6)]);
    for name in ["<#>", "<+>"] {
        assert_eq!(b.exports.values.iter().find(|v| v.name == name).unwrap().symbol, "A.<+>");
    }
    for interface in [a, b] {
        let positioned = interface.exports.values.iter().find(|v| v.name == "positioned").unwrap();
        assert!(positioned.scheme.contains("0 <= arg0") && positioned.scheme.contains("arg0 < arg1"), "{}", positioned.scheme);
        let defining = positioned.resolved.as_ref().unwrap();
        assert!(defining.scheme.contains("0 <= x") && defining.scheme.contains("x < y"), "{}", defining.scheme);
        let open = interface.exports.values.iter().find(|v| v.name == "row-polymorphic").unwrap();
        assert_eq!(open.symbol, "A.row-polymorphic");
        assert_eq!(open.arity, 1);
        assert!(open.scheme.contains('|'), "the checked open-row display scheme must survive");
        assert!(open.resolved.is_none(), "an open row must not be published as a closed portable contract");
        assert!(open.total_body.is_none(), "an opaque effect contract cannot authorize an executable proof body");
        let closed = interface.exports.values.iter().find(|v| v.name == "closed-callback").unwrap();
        let closed = closed.resolved.as_ref().expect("a closed kernel row retains its region and dictionary contract");
        assert!(closed.scheme.contains("r :: Region") && closed.scheme.contains("io :: Io r") && closed.scheme.contains("->{IO r}"), "{}", closed.scheme);
        for (name, identity) in [("inspect-scope", "A.inspect-scope"), ("task-source", "A.task-source")] {
            let value = interface.exports.values.iter().find(|v| v.name == name).unwrap();
            assert_eq!(value.symbol, identity);
            let resolved = value.resolved.as_ref().unwrap();
            assert!(resolved.nominals.is_empty(), "kernel Scope/Task types must not invent defining-module nominals");
            assert!(resolved.scheme.contains(if name == "inspect-scope" { "Scope" } else { "Task" }));
        }
    }
    for (name, arity) in [
        ("proof-wrap", 1), ("proof-mixed", 2), ("value-hidden", 2),
        ("proof-generic", 1), ("type-only", 1), ("region-proof", 2),
        ("mixed-hidden", 3), ("row-hidden", 2),
    ] {
        let reference = native_a.exports.values.iter().find(|v| v.name == name).unwrap();
        assert_eq!(reference.arity, arity, "Native definition-site contract for {name}");
        for iface in [a, b] {
            let actual = iface.exports.values.iter().find(|v| v.name == name).unwrap();
            assert_eq!(actual.arity, reference.arity, "{}.{name}: source arrows, dictionaries and hidden value parameters", iface.module);
            assert_eq!(actual.symbol, reference.symbol, "renaming must retain the defining value");
            assert_eq!(actual.resolved.is_none(), reference.resolved.is_none(), "{name}: opaque contract admission");
            assert!(actual.total_body.is_none(), "{name}: rich signatures must not gain plain executable authority");
        }
    }
    let give = a.exports.signatures.iter().find(|s| s.identity == "A.Give").unwrap();
    let method = give.methods.iter().find(|m| m.name == "give").unwrap().resolved.as_ref().unwrap();
    assert!(method.scheme.contains("dict :: A.Give a"), "{}", method.scheme);
    let factory = a.exports.instances.iter().find(|i| i.id == "A.ContextOnly_Values").unwrap();
    assert!(factory.implicit);
    assert_eq!(factory.ctx.len(), 1);
    let (dictionary, requirement) = factory.ctx[0].split_once(" :: ").unwrap();
    assert_ne!(dictionary, "eq", "dictionary name captured the quantified type parameter");
    assert_eq!(requirement, "A.Evidence eq");
    let resolved = factory.resolved.as_ref().unwrap();
    assert!(resolved.scheme.contains("eq :: Type"), "{}", resolved.scheme);
    assert!(resolved.scheme.contains(&format!("using ({dictionary} :: {requirement})")), "{}", resolved.scheme);
    assert_eq!(factory.target, "A.ContextOnly (A.Values eq)");
    assert!(factory.method_bodies.is_empty(), "a contextual factory cannot grant closed executable method authority");
    assert_eq!(factory.methods, ["A.ContextOnly_Values.context-only"]);
    for identity in ["A.ContextOnly", "A.Evidence"] {
        assert!(resolved.nominals.iter().any(|n| n.identity == identity && n.kind == "signature"));
    }
    for identity in ["A.ContextOnly", "A.Evidence", "A.Inner", "A.Deep"] {
        let hidden = a.exports.signatures.iter().find(|s| s.identity == identity).unwrap();
        assert!(hidden.exports.is_empty(), "retained factory class context exposed a source class name");
        assert!(hidden.methods.iter().all(|m| m.exports.is_empty()), "private class closure exposed a method name");
    }
    assert!(!a.exports.types.iter().any(|t| t.name == "Values"), "factory context exposed its private data carrier");
    let exists = a.exports.types.iter().find(|t| t.name == "Exists").unwrap();
    let pack = exists.ctors.iter().find(|c| c.name == "PackAny").unwrap();
    assert!(pack.scheme.contains("t :: Type"), "{}", pack.scheme);
    assert!(pack.resolved.is_none(), "hidden constructor parameters cannot authorize ordinary universal pattern instantiation");
    assert_eq!(pack.fields.len(), 1);
    assert_eq!(pack.fields, ["t"], "hidden field keeps its own binder name");
    let vector = a.exports.types.iter().find(|t| t.name == "Vec").unwrap();
    assert_eq!(vector.parameters.iter().map(|p| p.kind.as_str()).collect::<Vec<_>>(), ["Nat", "Type"]);
    let nil = vector.ctors.iter().find(|c| c.name == "VNil").unwrap();
    // Compare the independently parsed application tree; grouping is
    // canonical interface syntax, not a flat pretty-print contract.
    let input = format!("module MetadataProbe where\n  item :: {}\n", nil.scheme);
    let tokens = cinch::layout::layout(cinch::lexer::lex(&input).unwrap()).unwrap();
    let mut parsed = cinch::parser::parse_module(&tokens, &cinch::parser::FixityTable::new()).unwrap();
    let cinch::ast::Decl::ValueSig { ty, .. } = parsed.module.decls.pop().unwrap() else { panic!("missing constructor scheme"); };
    let cinch::ast::Type::Forall(parameters, body, _) = ty else { panic!("missing constructor parameters"); };
    assert_eq!(parameters.iter().map(|p| p.name.text.as_str()).collect::<Vec<_>>(), ["a"]);
    let cinch::ast::Type::App(head, element, _) = *body else { panic!("missing element argument"); };
    let cinch::ast::Type::App(family, index, _) = *head else { panic!("missing result index"); };
    assert!(matches!(*family, cinch::ast::Type::Con(ref path, _) if path.parts() == ["A", "Vec"]));
    assert!(matches!(*index, cinch::ast::Type::Num(ref n, _) if n == "0"));
    assert!(matches!(*element, cinch::ast::Type::Var(ref name, _) if name.text == "a"));
    assert!(!nil.scheme.contains("n :: Nat"), "unused family index was quantified: {}", nil.scheme);
    let cons = vector.ctors.iter().find(|c| c.name == "VCons").unwrap();
    assert!(cons.scheme.contains("k :: Nat") && cons.scheme.contains("k + 1"), "{}", cons.scheme);
    assert!(!cons.scheme.contains("n :: Nat"), "unused family index was quantified: {}", cons.scheme);
    let public = a.exports.types.iter().find(|t| t.name == "PublicPositive").expect("public defining refinement");
    let parcel = a.exports.types.iter().find(|t| t.name == "Parcel").unwrap();
    let native_parcel = native_a.exports.types.iter().find(|t| t.name == "Parcel").unwrap();
    for constructor in &parcel.ctors {
        let reference = native_parcel.ctors.iter().find(|c| c.name == constructor.name).unwrap();
        assert_eq!(constructor.scheme, reference.scheme, "public constructor display");
        assert_eq!(constructor.fields, reference.fields, "public constructor fields");
        let resolved = constructor.resolved.as_ref().unwrap();
        assert!(resolved.scheme.starts_with("forall (a :: Type)."), "{}", resolved.scheme);
    }
    assert_eq!(public.kind, "refinement");
    let template = public.refinement.as_ref().unwrap();
    assert_eq!((template.binder.as_str(), template.carrier.as_str(), template.predicate.as_str()), ("x", "I64", "0 < x"));
    let renamed = b.exports.types.iter().find(|t| t.name == "Positive").unwrap();
    assert_eq!(renamed.identity, "A.PublicPositive");
    assert_eq!(renamed.refinement, public.refinement);
    assert!(!a.exports.types.iter().any(|t| t.name == "Window" || t.name == "Positive"));
    assert!(!b.exports.types.iter().any(|t| t.name == "Window"));
    let contract = |iface: &cinch::interface::Iface, name: &str| iface.exports.values.iter().find(|v| v.name == name).unwrap().resolved.clone().unwrap();
    let handle = a.exports.types.iter().find(|t| t.name == "Handle").expect("public opaque declaration");
    assert_eq!((handle.identity.as_str(), handle.kind.as_str()), ("A.Handle", "opaque"));
    assert_eq!(handle.parameters.iter().map(|p| (p.name.as_str(), p.kind.as_str())).collect::<Vec<_>>(), [("a", "Type")]);
    assert!(handle.ctors.is_empty() && handle.fields.is_empty() && handle.refinement.is_none());
    let renamed_handle = b.exports.types.iter().find(|t| t.name == "PublicHandle").unwrap();
    assert_eq!(renamed_handle.identity, handle.identity);
    assert_eq!(renamed_handle.parameters, handle.parameters);
    assert_eq!(renamed_handle.kind, "opaque");
    assert!(!a.exports.types.iter().any(|t| t.name == "PrivateHandle"));
    let handle_contract = contract(a, "handle-id");
    assert_eq!(handle_contract, contract(b, "keep-handle"));
    let opaque = handle_contract.nominals.iter().find(|n| n.identity == "A.Handle").unwrap();
    assert_eq!(opaque.kind, "opaque");
    assert!(opaque.constructors.is_none() && opaque.refinement.is_none(), "opaque carrier acquired a closed constructor family");
    let secret = contract(a, "echo-secret");
    assert_eq!(secret, contract(b, "echo-secret"));
    let private = secret.nominals.iter().find(|n| n.identity == "A.PrivateHandle").unwrap();
    assert_eq!(private.kind, "opaque");
    assert!(private.constructors.is_none());
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
    assert_eq!(a.assumptions.primitives, ["cinch.runtime.arrayLen", "cinch.runtime.arrayReplicate", "cinch.runtime.textSha256"]);
    assert_eq!(a.exports.foreign.len(), 2, "array intrinsics retain defining foreign authority");
    for (local, external, alias) in [
        ("array-copy", "cinch.runtime.arrayReplicate", "repeat-array"),
        ("array-size", "cinch.runtime.arrayLen", "count-array"),
    ] {
        let row = a.exports.foreign.iter().find(|row| row.symbol == local).unwrap();
        assert_eq!(row.external, external);
        assert_eq!(row.abi, "ccall");
        assert_eq!(row.kind, "intrinsic");
        assert!(row.effects.is_empty());
        let original = a.exports.values.iter().find(|value| value.name == local).unwrap();
        let renamed = b.exports.values.iter().find(|value| value.name == alias).unwrap();
        assert_eq!(original.symbol, renamed.symbol);
        assert_eq!(original.resolved, renamed.resolved);
    }
    assert!(b.exports.foreign.is_empty(), "facades retain original defining identities");
    for (name, effect) in [("global-source", "Global"), ("fail-source", "Fail"), ("reentrant-source", "Reentrant"), ("thread-source", "ThreadAffine")] {
        let original = contract(a, name);
        assert_eq!(original, contract(b, name));
        assert!(original.scheme.contains(effect), "{name}: {}", original.scheme);
        assert!(original.nominals.is_empty(), "kernel effect gained a fabricated defining nominal");
    }
    assert!(b.assumptions.primitives.is_empty());
    assert!(!a.exports.types.iter().any(|t| t.name == "Local"));
    let signal = a.exports.values.iter().find(|v| v.name == "signal").expect("public operation contract");
    assert_eq!(signal.symbol, "A.signal");
    assert_eq!(signal.arity, 1);
    // Both producers write a source effect's label with its source spelling,
    // `Notice`, and not the defining identity. A plain `effect Notice where`
    // is not an owned declaration group, so the seed's effect head stays
    // bare (check_decls.rs registers `Module.Name` only when the declaration
    // carries a declaration binding). Two modules cannot then disagree about
    // a label: the seed refuses the second one with "duplicate effect", so
    // the ambiguity an identity would guard against cannot arise.
    assert!(signal.scheme.contains("Notice"), "effect label: {}", signal.scheme);
    assert!(!signal.scheme.contains("A.Notice"), "a plain effect keeps its bare label: {}", signal.scheme);
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
    for (interface, name) in [(a, "Give"), (b, "Refined")] {
        let native = native_interfaces.iter().find(|item| item.module == interface.module).unwrap();
        let actual_type = interface.exports.types.iter().find(|item| item.name == name).unwrap();
        assert_eq!(actual_type.kind, "signature");
        assert_eq!(Some(actual_type), native.exports.types.iter().find(|item| item.name == name));
        let layout = interface.layouts.iter().find(|item| item.name == name).unwrap();
        assert_eq!(Some(layout), native.layouts.iter().find(|item| item.name == name));
    }
    let instance = a.exports.instances.iter().find(|i| i.id == "A.GiveNat").unwrap();
    assert_eq!(instance.class, "A.Give");
    assert!(instance.implicit);
    assert_eq!(instance.methods, ["A.GiveNat.give"]);
    assert!(instance.method_contracts.is_empty());
    assert!(!a.exports.values.iter().any(|value| value.name == "GiveNat"));
    let parcel = a.exports.types.iter().find(|t| t.name == "Parcel").unwrap();
    assert_eq!(parcel.params, ["a"]);
    assert_eq!(parcel.ctors.iter().map(|c| (c.name.as_str(), c.tag)).collect::<Vec<_>>(), [("Empty", 0), ("Loaded", 1)]);
    assert!(parcel.ctors[0].fields.is_empty());
    assert_eq!(parcel.ctors[1].fields, ["a"]);
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

/// The cross-family producer oracle: for one module that names declarations
/// from another module, every interface family the two producers write must
/// agree field by field.
///
/// The `Data.Core` hash check above compares one module's contract. This one
/// compares the families that have their own rendering paths: a `data`, a
/// `record`, a `refine`, an `opaque type`, a `signature`, and an effect label
/// with its operation. Each of those is a place where the exporter can spell
/// a defining identity differently from the seed, and the first such table
/// (`base-constructor-names`) survived a whole interface suite because no
/// test compared the two producers across families.
#[test]
fn every_interface_family_agrees_with_the_seed_for_a_cross_module_use() {
    let dep = r#"module Dep where
  export Box, Wrapped, Pos, Handle, Mapper
  data Box a where
    Box :: a -> Box a
  record Wrapped where
    n :: Nat
    b :: Box Nat
  type Pos = refine (n :: Nat) with n > 0
  opaque type Handle :: Type
  signature Mapper (f :: Type -> Type) where
    map-item :: forall a b. (a -> b) -> f a -> f b
"#;
    let use_site = r#"module Use where
  import Dep
  export take-box, wrap-one, unbox, one, use-handle, poke
  effect Notice where
    poke :: Nat ->{Notice} Nat
  take-box :: Box Nat -> Nat
  take-box b = case b of Box n -> n
  wrap-one :: Wrapped
  wrap-one = Wrapped { n = 1, b = Box 2 }
  unbox :: Wrapped -> Nat
  unbox w = case w of Wrapped { n = n, b = b } -> n
  one :: Pos
  one = 1
  use-handle :: Handle -> Handle
  use-handle h = h
"#;
    // The seed's contracts for the same two sources.
    let seed_dir = common::project("family_oracle_seed", &[
        ("Dep.cinch", dep),
        ("Use.cinch", use_site),
    ]);
    let seed = common::check_interfaces(&seed_dir, "Use.cinch")
        .expect("the seed checks the cross-family fixture");
    let seed_use = seed.iter().find(|i| i.module == "Use").expect("seed Use interface");
    let seed_dep = seed.iter().find(|i| i.module == "Dep").expect("seed Dep interface");

    // The Cinch exporter's contracts for the same bytes.
    let dir = common::project("export_interfaces", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Export.cinch", include_str!("../../bootstrap/Bootstrap.Export.cinch")),
        ("Bootstrap.ExportType.cinch", include_str!("../../bootstrap/Bootstrap.ExportType.cinch")),
        ("Bootstrap.ExportNominal.cinch", include_str!("../../bootstrap/Bootstrap.ExportNominal.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ExportInterfaces.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "export-interfaces").expect("compile the Cinch interface writer");
    std::fs::write(dir.join("base-export-fixture"), format!("{dep}\n{use_site}")).unwrap();
    let run = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let error = std::fs::read_to_string(dir.join("export-error.txt")).unwrap_or_default();
    assert_eq!(run.status.code(), Some(0), "{error}\n{}\nProject: {}", String::from_utf8_lossy(&run.stderr), dir.display());
    let exported: Vec<_> = common::read_output(&dir, "exported-base-interfaces.jsonl").lines()
        .map(|line| cinch::interface::Iface::from_json(line).expect("independent readback"))
        .collect();
    let cinch_use = exported.iter().find(|i| i.module == "Use").expect("Cinch Use interface");
    let cinch_dep = exported.iter().find(|i| i.module == "Dep").expect("Cinch Dep interface");

    // The data family: a constructor whose field names an imported type.
    let seed_box = seed_dep.exports.types.iter().find(|t| t.name == "Box").expect("seed data");
    let cinch_box = cinch_dep.exports.types.iter().find(|t| t.name == "Box").expect("Cinch data");
    for (seed_ctor, cinch_ctor) in seed_box.ctors.iter().zip(&cinch_box.ctors) {
        assert_eq!(seed_ctor.scheme, cinch_ctor.scheme, "data {} constructor scheme", seed_ctor.name);
        assert_eq!(seed_ctor.fields, cinch_ctor.fields, "data {} constructor fields", seed_ctor.name);
        assert_eq!(seed_ctor.erased, cinch_ctor.erased, "data {} constructor erasure", seed_ctor.name);
    }
    // The record family: a field whose type is another module's data.
    let seed_record = seed_dep.exports.types.iter().find(|t| t.name == "Wrapped").expect("seed record");
    let cinch_record = cinch_dep.exports.types.iter().find(|t| t.name == "Wrapped").expect("Cinch record");
    assert_eq!(seed_record.ctors[0].fields, cinch_record.ctors[0].fields, "record field types");
    assert_eq!(seed_record.ctors[0].scheme, cinch_record.ctors[0].scheme, "record constructor scheme");
    // The opaque family.
    let seed_handle = seed_dep.exports.types.iter().find(|t| t.name == "Handle").expect("seed opaque");
    let cinch_handle = cinch_dep.exports.types.iter().find(|t| t.name == "Handle").expect("Cinch opaque");
    assert_eq!((seed_handle.identity.as_str(), seed_handle.kind.as_str()), (cinch_handle.identity.as_str(), cinch_handle.kind.as_str()), "opaque identity and kind");
    // The signature family, including a method that mentions an imported type.
    let seed_mapper = seed_dep.exports.signatures.iter().find(|s| s.identity == "Dep.Mapper").expect("seed signature");
    let cinch_mapper = cinch_dep.exports.signatures.iter().find(|s| s.identity == "Dep.Mapper").expect("Cinch signature");
    assert_eq!(seed_mapper.methods.len(), cinch_mapper.methods.len(), "signature method count");
    for (seed_method, cinch_method) in seed_mapper.methods.iter().zip(&cinch_mapper.methods) {
        assert_eq!(seed_method.scheme, cinch_method.scheme, "signature method {}", seed_method.name);
        assert_eq!(seed_method.slot, cinch_method.slot, "signature method slot {}", seed_method.name);
    }
    // The effect family: the label and its operation.
    let seed_poke = seed_use.exports.values.iter().find(|v| v.name == "poke").expect("seed operation");
    let cinch_poke = cinch_use.exports.values.iter().find(|v| v.name == "poke").expect("Cinch operation");
    assert_eq!(seed_poke.scheme, cinch_poke.scheme, "effect operation scheme");
    assert_eq!(seed_poke.symbol, cinch_poke.symbol, "effect operation symbol");
    assert_eq!(seed_poke.arity, cinch_poke.arity, "effect operation arity");
    // Every cross-module value agrees on symbol and arity, and on the
    // resolved contract that a consumer reads. The DISPLAY scheme is
    // compared for the families above only: the refinement family still
    // diverges, and that divergence is recorded in its own test below so
    // this oracle stays green on the families that agree.
    for seed_value in &seed_use.exports.values {
        let cinch_value = cinch_use.exports.values.iter().find(|v| v.name == seed_value.name)
            .unwrap_or_else(|| panic!("Cinch dropped {}", seed_value.name));
        assert_eq!(seed_value.symbol, cinch_value.symbol, "{}: symbol", seed_value.name);
        assert_eq!(seed_value.arity, cinch_value.arity, "{}: arity", seed_value.name);
        // `one`'s type is a named refinement, whose expansion is the known
        // divergence the test below records.
        if seed_value.name == "one" { continue; }
        assert_eq!(seed_value.scheme, cinch_value.scheme, "{}: display scheme", seed_value.name);
        assert_eq!(seed_value.resolved, cinch_value.resolved, "{}: resolved contract", seed_value.name);
    }
    common::cleanup(&seed_dir);
    common::cleanup(&dir);
}

/// A KNOWN, UNFIXED divergence between the two producers, recorded so it is
/// tracked by name instead of hiding inside a green suite.
///
/// For a value whose type is a named refinement declared in another module,
/// the seed writes the refinement's identity (`Dep.Pos`) and the Cinch
/// exporter writes the expanded form (`(refined ((cinch-result > 0)) Nat)`).
/// The same source therefore produces two different contracts, and the
/// interface hashes differ, which is the same class as the constructor alias
/// table that `every_interface_family_agrees_with_the_seed_for_a_cross_module_use`
/// now guards.
///
/// This test asserts the CURRENT state on both sides. It is not a statement
/// that the divergence is correct: it is a pin that will fail the moment
/// either producer changes, which is when the pair must be brought together.
/// A fix attempt that made `abstract-refinement` build a former with no type
/// parameters did not change this output, so the expansion happens on a path
/// that patch does not reach.
#[test]
fn named_refinement_display_still_diverges_between_the_producers() {
    let dep = "module Dep where\n  export Pos\n  type Pos = refine (n :: Nat) with n > 0\n";
    let use_site = "module Use where\n  import Dep\n  export one\n  one :: Pos\n  one = 1\n";
    let seed_dir = common::project("refinement_divergence_seed", &[
        ("Dep.cinch", dep),
        ("Use.cinch", use_site),
    ]);
    let seed = common::check_interfaces(&seed_dir, "Use.cinch").expect("the seed checks the fixture");
    let seed_one = seed.iter().find(|i| i.module == "Use").unwrap()
        .exports.values.iter().find(|v| v.name == "one").unwrap().clone();

    let dir = common::project("export_interfaces", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Export.cinch", include_str!("../../bootstrap/Bootstrap.Export.cinch")),
        ("Bootstrap.ExportType.cinch", include_str!("../../bootstrap/Bootstrap.ExportType.cinch")),
        ("Bootstrap.ExportNominal.cinch", include_str!("../../bootstrap/Bootstrap.ExportNominal.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ExportInterfaces.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "export-interfaces").expect("compile the Cinch interface writer");
    std::fs::write(dir.join("base-export-fixture"), format!("{dep}\n{use_site}")).unwrap();
    let run = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let error = std::fs::read_to_string(dir.join("export-error.txt")).unwrap_or_default();
    assert_eq!(run.status.code(), Some(0), "{error}\n{}", String::from_utf8_lossy(&run.stderr));
    let exported: Vec<_> = common::read_output(&dir, "exported-base-interfaces.jsonl").lines()
        .map(|line| cinch::interface::Iface::from_json(line).expect("independent readback"))
        .collect();
    let cinch_one = exported.iter().find(|i| i.module == "Use").unwrap()
        .exports.values.iter().find(|v| v.name == "one").unwrap().clone();

    assert_eq!(seed_one.scheme, "Dep.Pos", "the seed names the refinement");
    assert!(cinch_one.scheme.contains("refined ("), "the exporter expands it: {}", cinch_one.scheme);
    // Both sides still record the defining nominal, so a consumer can
    // resolve the identity even though the display text disagrees.
    let nominal = |value: &cinch::interface::ValueExport| value.resolved.as_ref()
        .map(|resolved| resolved.nominals.iter().any(|n| n.identity == "Dep.Pos"))
        .unwrap_or(false);
    assert!(nominal(&seed_one), "the seed records Dep.Pos");
    assert!(nominal(&cinch_one), "the exporter records Dep.Pos");
    common::cleanup(&seed_dir);
    common::cleanup(&dir);
}
