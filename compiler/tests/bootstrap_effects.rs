//! Compile the Cinch checker once, then change its source input between runs.
mod common;

#[test]
fn source_effects_and_lexical_io_capabilities_are_checked_in_cinch() {
    let dir = common::project("source_effects", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Effects.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-effects").expect("build Cinch source effect checker");
    let cases = [
        ("propagate", "pass :: forall r. (Unit ->{IO r} Unit) -> Unit ->{IO r} Unit\n  pass f x = f x", None),
        ("capture", "capture :: forall r. (Unit ->{IO r} Unit) -> (Unit ->{IO r} Unit)\n  capture f = \\x -> f x", None),
        ("given", "write :: forall r. using (io :: Io r) => Bytes ->{IO r} Unit\n  write bytes = ioWrite bytes", None),
        ("with", "write :: forall r. Io r -> Bytes ->{IO r} Unit\n  write cap bytes = with (io = cap) in ioWrite bytes", None),
        ("fresh", "write :: forall r. Bytes ->{IO r} Unit\n  write bytes = with (io = newIO ()) in ioWrite bytes", None),
        ("implicit-row-region", "write :: Bytes ->{IO r} Unit\n  write bytes = with (io = newIO ()) in ioWrite bytes", None),
        ("pure-body", "bad :: forall r. (Unit ->{IO r} Unit) -> Unit -> Unit\n  bad f x = f x", Some("performed effects exceed")),
        ("pure-callback", "bad :: forall r. (Unit ->{IO r} Unit) -> (Unit -> Unit)\n  bad f = \\x -> f x", Some("performed effects exceed")),
        ("missing-provider", "bad :: forall r. Bytes ->{IO r} Unit\n  bad bytes = ioWrite bytes", Some("dictionary instance resolution")),
        ("wrong-region", "bad :: forall r s. using (io :: Io r) => Bytes ->{IO s} Unit\n  bad bytes = ioWrite bytes", Some("dictionary instance resolution")),
        ("with-does-not-mask", "bad :: forall r. Io r -> Bytes -> Unit\n  bad cap bytes = with (io = cap) in ioWrite bytes", Some("performed effects exceed")),
        ("scope-does-not-leak", "bad :: forall r. Io r -> Bytes ->{IO r} Unit\n  bad cap bytes = let ignored = with (io = cap) in () in ioWrite bytes", Some("dictionary instance resolution")),
        ("fresh-closure-escapes", "bad :: Unit -> Unit\n  bad x = let escaped = with (io = newIO ()) in \\bytes -> ioWrite bytes in ()", Some("fresh capability region escapes")),
        ("wrong-region-kind", "bad :: forall (r :: Type). Unit ->{IO r} Unit\n  bad x = x", Some("must have kind Region")),
        ("capability-is-not-a-region", "bad :: forall (r :: Region). r -> r\n  bad x = x", Some("must have kind Type")),
    ];
    for (name, body, expected) in cases {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  {body}\n")).unwrap();
        let out = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "effects-report.txt");
        assert_eq!(out.status.code(), Some(if expected.is_some() {4} else {0}), "case {name}; project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&out.stderr));
        if let Some(message) = expected {
            assert!(report.contains(message), "case {name} expected {message:?}; project {}\n{report}", dir.display());
        }
    }
    common::cleanup(&dir);
}
