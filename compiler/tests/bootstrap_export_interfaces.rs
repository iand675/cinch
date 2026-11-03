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
    assert_eq!(a.exports.fixities.len(), 1);
    assert_eq!(a.exports.fixities[0].op, "<+>");
    assert!(b.exports.fixities.is_empty());
    assert_eq!(a.assumptions.primitives, ["cinch.runtime.textSha256"]);
    assert!(b.assumptions.primitives.is_empty());
    assert!(!a.exports.types.iter().any(|t| t.name == "Local"));
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
