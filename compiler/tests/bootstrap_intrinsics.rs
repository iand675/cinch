//! Foreign declarations supplied after the Cinch checker has been compiled.
mod common;

#[test]
fn runtime_intrinsics_check_their_contract_and_retain_canonical_symbols() {
    let dir = common::project("source_intrinsics", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Intrinsics.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "intrinsics").expect("build the Cinch intrinsic checker");
    let contracts = [
        ("cinch.runtime.textSha256", "Bytes -> Bytes", "probe input = native input"),
        ("cinch.runtime.textSha512", "Bytes -> Bytes", "probe input = native input"),
        ("cinch.runtime.textSha1", "Bytes -> Bytes", "probe input = native input"),
        ("cinch.runtime.textMd5", "Bytes -> Bytes", "probe input = native input"),
        ("cinch.runtime.textHmacSha256", "Bytes -> Bytes -> Bytes", "probe key input = native key input"),
        ("cinch.runtime.textF64ToDecimal", "F64 -> Bytes", "probe input = native input"),
        ("cinch.runtime.textF64FromDecimal", "Bytes -> F64", "probe input = native input"),
    ];
    let mut cases = Vec::new();
    for (symbol, scheme, body) in contracts {
        // These are the seed's actual runtime symbols, not source aliases.
        let cinch::core::PrimOp::Text(op) = cinch::core::PrimOp::from_symbol(symbol).unwrap() else { panic!("not a text intrinsic") };
        let external = op.extern_name();
        cases.push((symbol.to_owned(), external.to_owned(), format!("foreign import ccall \"{symbol}\" native :: {scheme}\n  probe :: {scheme}\n  {body}"), None));
    }
    let symbol = "cinch.runtime.textSha256";
    for (identity, scheme) in [
        ("cinch.prim.eqI64", "I64 -> I64 -> Bool"),
        ("cinch.prim.wrappingAddI64", "I64 -> I64 -> I64"),
        ("cinch.prim.wrappingSubI64", "I64 -> I64 -> I64"),
    ] {
        assert_eq!(cinch::core::PrimOp::from_symbol(identity).unwrap().symbol(), identity);
        cases.push((identity.into(), identity.into(), format!("foreign import ccall \"{identity}\" native :: {scheme}\n  probe :: {scheme}\n  probe = native"), None));
    }
    for (name, identity, scheme, message) in [
        ("machine wrong carrier", "cinch.prim.eqI64", "Int -> Int -> Bool", "type mismatch"),
        ("machine wrong result", "cinch.prim.wrappingAddI64", "I64 -> I64 -> Bool", "type mismatch"),
        ("machine wrong arity", "cinch.prim.wrappingSubI64", "I64 -> I64", "type mismatch"),
        ("machine polymorphic", "cinch.prim.eqI64", "forall a. a -> a -> Bool", "cannot be polymorphic"),
        ("machine unknown operation", "cinch.prim.imaginaryI64", "I64 -> I64 -> I64", "checked runtime intrinsic contract"),
    ] {
        cases.push((identity.into(), identity.into(), format!("foreign import ccall \"{identity}\" native :: {scheme}"), Some((name, message))));
    }
    for (name, source, message) in [
        ("first class", "foreign import ccall \"cinch.runtime.textSha256\" native :: Bytes -> Bytes\n  probe :: Bytes -> Bytes\n  probe = native", None),
        ("named domain", "foreign import ccall \"cinch.runtime.textSha256\" native :: (input :: Bytes) -> Bytes\n  probe :: Bytes -> Bytes\n  probe input = native input", None),
        ("wrong input", "foreign import ccall \"cinch.runtime.textSha256\" native :: Int -> Bytes", Some("type mismatch")),
        ("wrong output", "foreign import ccall \"cinch.runtime.textF64FromDecimal\" native :: Bytes -> Bytes", Some("type mismatch")),
        ("wrong arity", "foreign import ccall \"cinch.runtime.textHmacSha256\" native :: Bytes -> Bytes", Some("type mismatch")),
        ("polymorphic", "foreign import ccall \"cinch.runtime.textSha256\" native :: forall a. a -> a", Some("cannot be polymorphic")),
        ("wrong effects", "foreign import ccall \"cinch.runtime.textSha256\" native :: forall r. Bytes ->{IO r} Bytes", Some("cannot be polymorphic")),
        ("unknown intrinsic", "foreign import ccall \"cinch.runtime.textImaginary\" native :: Bytes -> Bytes", Some("checked runtime intrinsic contract")),
        ("unknown C symbol", "foreign import ccall \"untrusted_hash\" native :: Bytes -> Bytes", Some("accounted result effects")),
        ("wrong ABI", "foreign import component \"cinch.runtime.textSha256\" native :: Bytes -> Bytes", Some("requires foreign import ccall")),
        ("export", "foreign export ccall \"cinch.runtime.textSha256\" native :: Bytes -> Bytes", Some("requires foreign import ccall")),
        ("duplicate", "foreign import ccall \"cinch.runtime.textSha256\" native :: Bytes -> Bytes\n  foreign import ccall \"cinch.runtime.textMd5\" native :: Bytes -> Bytes", Some("duplicate intrinsic binding")),
        ("source replacement", "foreign import ccall \"cinch.runtime.textSha256\" native :: Bytes -> Bytes\n  native input = input", Some("cannot also have a source definition")),
    ] {
        cases.push((symbol.into(), "cinch_text_sha256".into(), source.into(), message.map(|m| (name, m))));
    }
    for (identity, external, body, refusal) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        std::fs::write(dir.join("identity.txt"), &identity).unwrap();
        std::fs::write(dir.join("external.txt"), &external).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "intrinsic-report.txt");
        assert_eq!(output.status.code(), Some(if refusal.is_some() {4} else {0}), "{identity} {refusal:?}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if let Some((name, message)) = refusal { assert!(report.contains(message), "{name}: {report}"); }
    }
    common::cleanup(&dir);
}
