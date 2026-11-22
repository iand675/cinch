//! Dependent Pi conversion, substitution, scoped unification and application.
mod common;

#[test]
fn dependent_functions_preserve_binding_scope_effects_and_application() {
    let dir = common::project(
        "pi",
        &[
            (
                "Bootstrap.Ast.cinch",
                include_str!("../../bootstrap/Bootstrap.Ast.cinch"),
            ),
            (
                "Bootstrap.Report.cinch",
                include_str!("../../bootstrap/Bootstrap.Report.cinch"),
            ),
            (
                "Bootstrap.Type.cinch",
                include_str!("../../bootstrap/Bootstrap.Type.cinch"),
            ),
            (
                "Bootstrap.Core.cinch",
                include_str!("../../bootstrap/Bootstrap.Core.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.Pi.cinch"),
            ),
        ],
    );
    let exe =
        common::build(&dir, "Main.cinch", "pi").expect("build dependent Pi engine");
    let run = |name: &str, input: &[u8], expected: i32| {
        std::fs::write(dir.join("case.txt"), input).unwrap();
        let output = std::process::Command::new(&exe)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(expected),
            "case {name}; project {}\n{}\n{}",
            dir.display(),
            common::read_output(&dir, "pi-report.txt"),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    for size in [0, 1, 7, 31, 257] {
        run(&format!("nonce {size}"), format!(r#"{{"mode":"selftest","nonce":{size}}}"#).as_bytes(), 0);
    }
    // Complete carrier/type/index trees arrive after compilation. The
    // seed's constant evaluator cannot know or precompute these packets.
    let packets = [
        ("alpha binders", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",2,"n"]]]]}"#, 0),
        ("free versus bound", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",2,"n"]]]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",2,"n"]]]]}"#, 4),
        ("nested binder identity", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["pi",2,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]]],"right":["pi",3,"n",["con","Nat","Type"],["row","end"],["pi",4,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",4,"n"]]]]]}"#, 4),
        ("arbitrary precision binder id", r#"{"mode":"unify","left":["pi",340282366920938463463374607431768211456,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",340282366920938463463374607431768211456,"n"]]]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",2,"n"]]]]}"#, 0),
        ("ordinary arrow equivalence", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"right":["fun",["con","Nat","Type"],["con","Nat","Type"]]}"#, 0),
        ("ordinary arrow reverse equivalence", r#"{"mode":"unify","left":["fun",["con","Nat","Type"],["con","Nat","Type"]],"right":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 0),
        ("constant arrow cannot replace dependent one", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]],"right":["fun",["con","Nat","Type"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["int",3]]]]}"#, 4),
        ("domain mismatch", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"right":["pi",2,"n",["con","Int","Type"],["row","end"],["con","Nat","Type"]]}"#, 4),
        ("effect mismatch", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"right":["pi",2,"n",["con","Nat","Type"],["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["bound",2,"n"]]],"end"]],["con","Nat","Type"]]}"#, 4),
        ("dependent row alpha", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["bound",1,"n"]]],"end"]],["con","Nat","Type"]],"right":["pi",2,"n",["con","Nat","Type"],["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["bound",2,"n"]]],"end"]],["con","Nat","Type"]]}"#, 0),
        ("invalid effect row", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["con","Nat","Type"],["con","Nat","Type"]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 4),
        ("recovery type is no equality proof", r#"{"mode":"unify","left":"error","right":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 4),
        ("negative binder id", r#"{"mode":"unify","left":["pi",-1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 2),
        ("missing codomain", r#"{"mode":"unify","left":["pi",1,"n",["con","Nat","Type"],["row","end"]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 2),
        ("application substitutes value", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]],"argument":["index",["con","Nat","Type"],["int",3]],"result":["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["int",3]]],"effects":["row","end"]}"#, 0),
        ("application folds only resultant numerals", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["add",["bound",1,"n"],["int",1]]]]],"argument":["index",["con","Nat","Type"],["int",3]],"result":["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["int",4]]],"effects":["row","end"]}"#, 0),
        ("argument carrier mismatch", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]],"argument":["index",["con","Int","Type"],["int",3]],"result":["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["int",3]]],"effects":["row","end"]}"#, 4),
        ("undefined natural argument", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"argument":["index",["con","Nat","Type"],["sub",["int",0],["int",1]]],"result":["con","Nat","Type"],"effects":["row","end"]}"#, 4),
        ("argument is not a type", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"argument":["con","Nat","Type"],"result":["con","Nat","Type"],"effects":["row","end"]}"#, 4),
        ("effects survive application", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["bound",1,"n"]]],"end"]],["con","Nat","Type"]],"argument":["index",["con","Nat","Type"],["int",7]],"result":["con","Nat","Type"],"effects":["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["int",7]]],"end"]]}"#, 0),
        ("effects cannot disappear", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row",["arg",["app",["con","Trace",["kind-fun",["value",["con","Nat","Type"]],"Effect"]],["index",["con","Nat","Type"],["bound",1,"n"]]],"end"]],["con","Nat","Type"]],"argument":["index",["con","Nat","Type"],["int",7]],"result":["con","Nat","Type"],"effects":["row","end"]}"#, 4),
        ("malformed function cannot normalize to validity", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["add",["int",-1],["int",2]]]]],"argument":["index",["con","Nat","Type"],["int",3]],"result":["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["int",1]]],"effects":["row","end"]}"#, 4),
        ("unknown argument meta", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]],"argument":["index",["con","Nat","Type"],["meta",999]],"result":["con","Nat","Type"],"effects":["row","end"]}"#, 4),
        ("argument scope escape", r#"{"mode":"apply","function":["pi",1,"n",["con","Nat","Type"],["row","end"],["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]]],"argument":["index",["con","Nat","Type"],["rigid",999,"inner",1]],"result":["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["rigid",999,"inner",1]]],"effects":["row","end"]}"#, 4),
        ("own domain binding is invalid", r#"{"mode":"unify","left":["pi",1,"n",["app",["con","At",["kind-fun",["value",["con","Nat","Type"]],"Type"]],["index",["con","Nat","Type"],["bound",1,"n"]]],["row","end"],["con","Nat","Type"]],"right":["pi",2,"n",["con","Nat","Type"],["row","end"],["con","Nat","Type"]]}"#, 4),
        ("malformed JSON", "{", 2),
    ];
    for (name, input, expected) in packets {
        run(name, input.as_bytes(), expected);
    }
    common::cleanup(&dir);
}
