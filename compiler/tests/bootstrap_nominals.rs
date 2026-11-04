mod common;
use cinch::interface::{Iface, IfaceRef, ValueExport, ResolvedScheme, NominalContract, TypeExport, CtorExport, ParameterExport};

fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}

#[test]
fn imported_nominals_are_checked_with_defining_identities_in_cinch() {
    let source = common::project("nominal_source", &[
        ("A.cinch", "module A where\n  export Token, Mk, keep, secret-api, Box, Boxed, Partial, Open, Witness, OtherWitness, witnessed, context-api\n  data Token where\n    Mk :: Token\n  data HiddenToken where\n    Hide :: Int -> HiddenToken\n  data Box a where\n    Boxed :: a -> Box a\n  data Partial where\n    Open :: Partial\n    Closed :: Partial\n  signature Witness a where\n    first-method :: a -> a\n    witnessed :: a -> Bool\n  signature OtherWitness a where\n    other-witnessed :: a -> Bool\n  context-api :: forall a. using (witness :: Witness a) => a -> a\n  context-api x = x\n  keep :: Token -> Token\n  keep x = x\n  secret-api :: HiddenToken -> HiddenToken\n  secret-api x = x\n"),
        ("B.cinch", "module B where\n  data Token where\n    Mk :: Token\n  data String where\n    UserText :: String\n  keep-text :: String -> String\n  keep-text x = x\n"),
        ("Facade.cinch", "module Facade where\n  import A (Token as Label, Mk as Make, keep as use, secret-api as opaque-use, Box as Container, Boxed as Pack, Partial as Visible, Open)\n  export Label, Make, use, opaque-use, Container, Pack, Visible, Open\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  import B as Other\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&source, "Main.cinch").expect("emit nominal contracts");
    let dir = common::project("nominal_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Nominal.cinch")),
    ]);
    let mut rows = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" { rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string()))); }
        }
    }
    rows.sort();
    // A checked qualified scheme must agree with the dictionary-bearing
    // ABI, rather than passing because its source arrows happen to match.
    let a = Iface::from_json(&std::fs::read_to_string(source.join("build/A.cinchif")).unwrap()).unwrap();
    let mut wrong_arity = Iface::new("WrongArity", "");
    wrong_arity.imports.push(IfaceRef { module: "A".into(), iface_hash: a.iface_hash() });
    let mut call = a.exports.values.iter().find(|v| v.name == "context-api").unwrap().clone();
    assert_eq!(call.arity, 2, "one dictionary and one source value argument");
    call.name = "call".into();
    call.symbol = "WrongArity.call".into();
    call.arity = 1;
    wrong_arity.exports.values.push(call);
    let path = dir.join("WrongArity.cinchif");
    std::fs::write(&path, wrong_arity.to_json()).unwrap();
    rows.push(format!("[{},{},{}]", quote("WrongArity"), quote(&wrong_arity.iface_hash()), quote(&path.display().to_string())));
    for (module, scheme, resolved) in [
        ("DisplayOnly", "String", None),
        ("Unbound", "Int", Some(ResolvedScheme { measures: vec![], scheme: "Missing.Token".into(), nominals: vec![] })),
    ] {
        let mut iface = Iface::new(module, "");
        iface.exports.values.push(ValueExport { total_body: None, name: "value".into(), symbol: format!("{module}.value"), scheme: scheme.into(), resolved, arity: 0 });
        let path = dir.join(format!("{module}.cinchif"));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    // Complete carrier annotations must be qualified in preceding-binder
    // scope. These synthetic contracts exercise the consumer independently
    // of the seed's producer; importing them does not execute a body.
    for (module, scheme, nominal) in [
        ("OpenRow", "forall (e :: Row). (Unit ->{|e} Unit) ->{|e} Unit", false),
        ("WrongRowKind", "forall (e :: Type). (Unit ->{|e} Unit) ->{|e} Unit", false),
        ("UnboundRow", "(Unit ->{|missing} Unit) ->{|missing} Unit", false),
        ("CarrierGeneric", "forall (a :: Type) (x :: a). (Int -> Int)", false),
        ("CarrierFunction", "forall (a :: Type) (f :: a -> a). (Int -> Int)", false),
        ("CarrierNominal", "forall (x :: A.Token). (Int -> Int)", true),
        ("CarrierForward", "forall (x :: later) (later :: Type). (Int -> Int)", false),
        ("CarrierSelf", "forall (x :: x). (Int -> Int)", false),
        ("CarrierMissing", "forall (x :: Missing.Token). (Int -> Int)", false),
    ] {
        let mut iface = Iface::new(module, "");
        let nominals = if nominal {
            iface.imports.push(IfaceRef { module: "A".into(), iface_hash: a.iface_hash() });
            vec![NominalContract { refinement: None, identity: "A.Token".into(), kind: "data".into(), parameters: vec![], constructors: Some(vec!["A.Mk".into()]) }]
        } else { vec![] };
        iface.exports.values.push(ValueExport { total_body: None,
            name: "value".into(), symbol: format!("{module}.value"), scheme: scheme.into(), arity: 1,
            resolved: Some(ResolvedScheme { measures: vec![], scheme: scheme.into(), nominals }),
        });
        let path = dir.join(format!("{module}.cinchif"));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    // Neither an existential witness nor an indexed result may enter the
    // regular-constructor path as an ordinary fresh universal variable.
    for (module, parameters, body) in [
        ("UniversalCtor", vec![], "(a -> UniversalCtor.Box)"),
        ("IndexedCtor", vec![ParameterExport { name: "a".into(), kind: "Type".into(), implicit: false }],
            "(Int -> (IndexedCtor.Box Int))"),
    ] {
        let mut iface = Iface::new(module, "");
        let identity = format!("{module}.Box");
        let symbol = format!("{module}.Pack");
        iface.exports.types.push(TypeExport {
            name: "Box".into(), identity: identity.clone(),
            params: parameters.iter().map(|p| p.name.clone()).collect(), parameters: parameters.clone(),
            kind: "data".into(), fields: vec![], refinement: None,
            ctors: vec![CtorExport {
                name: "Pack".into(), symbol: symbol.clone(), tag: 0,
                scheme: format!("forall (a :: Type). {body}"), fields: vec!["a".into()], erased: vec![false],
                resolved: Some(ResolvedScheme { measures: vec![], scheme: format!("forall (a :: Type). {body}"), nominals: vec![NominalContract { refinement: None,
                    identity, kind: "data".into(), parameters, constructors: Some(vec![symbol]),
                }] }),
            }],
        });
        let path = dir.join(format!("{module}.cinchif"));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "nominals").expect("build Cinch nominal consumer");
    let output = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "nominal-report.txt");
    assert_eq!(output.status.code(), Some(0), "nominal case failed; project {}\n{}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr), report);
    common::cleanup(&source); common::cleanup(&dir);
}
