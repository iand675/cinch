//! Checked implicit factory selection and evidence/guard reconstruction in Cinch.
mod common;
#[test]
fn instance_factories_preserve_scope_coherence_and_actual_method_semantics() {
    let dir = common::project("instance_resolution", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Dictionary.cinch", include_str!("../../bootstrap/Bootstrap.Dictionary.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.InstanceResolution.cinch")),
    ]);
    // Supply actual checked method implementations. A string naming an
    // unregistered dictionary cannot itself justify a numeric equation.
    let family = cinch::driver::parse_prelude(&[]).unwrap();
    let provider = cinch::driver::parse_source("module ProofProvider where\n  export nat-greater, machine-greater, nat-addition\n  nat-greater :: Nat -> Nat -> Bool\n  nat-greater a b = a > b\n  machine-greater :: I64 -> I64 -> Bool\n  machine-greater a b = a > b\n  nat-addition :: Nat -> Nat -> Nat\n  nat-addition a b = a + b\n", "proof-provider.cinch", &family.base, &family.exported_fixities).unwrap();
    let mut context = cinch::driver::StagingCx::default(); context.fixities = family.fixities; context.fixities.extend(provider.fixities);
    let (_, interfaces, _) = cinch::check::check_program_multi(&family.modules, &provider.modules, &std::collections::HashMap::new(), &context).unwrap();
    let rows = interfaces.iter().map(|interface| {
        let path = dir.join(format!("{}.cinchif", interface.module)); std::fs::write(&path, interface.to_json()).unwrap();
        cinch::lsp::Json::Arr(vec![cinch::lsp::Json::Str(interface.module.clone()), cinch::lsp::Json::Str(interface.iface_hash()), cinch::lsp::Json::Str(path.display().to_string())])
    }).collect();
    std::fs::write(dir.join("interfaces.json"), cinch::lsp::Json::obj(vec![("schema", cinch::lsp::Json::Num(1.0)), ("interfaces", cinch::lsp::Json::Arr(rows))]).to_text()).unwrap();
    let exe = common::build(&dir, "Main.cinch", "instance-resolution").expect("build the Cinch checked-instance runner");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "instance/guard check at nonce {size}; failing group is exit code; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
