//! Index binding and substitution run with identities supplied after compilation.
mod common;

#[test]
fn index_substitution_preserves_scope_and_binding_identity() {
    let dir = common::project(
        "indices",
        &[
            (
                "Bootstrap.Index.cinch",
                include_str!("../../bootstrap/Bootstrap.Index.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.Indices.cinch"),
            ),
        ],
    );
    let exe =
        common::build(&dir, "Main.cinch", "indices").expect("build Cinch index substitutions");
    let run = |name: &str, input: &str, expected: i32| {
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
            common::read_output(&dir, "indices-report.txt"),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    for nonce in [0, 1, 7, 20, 257, 4096] {
        run(
            &format!("nonce {nonce}"),
            &format!(r#"{{"mode":"selftest","nonce":{nonce}}}"#),
            0,
        );
    }
    // Complete terms and scopes are decoded only after the Cinch program
    // has been compiled. Neither its inputs nor the expected outcomes are
    // available to Cinch's compile-time evaluator.
    let packets = [
        (
            "same spelling, different identity",
            r#"{"mode":"equal","left":["bound",1,"x"],"right":["bound",2,"x"]}"#,
            4,
        ),
        (
            "same identity, different spelling",
            r#"{"mode":"equal","left":["bound",1,"x"],"right":["bound",1,"renamed"]}"#,
            0,
        ),
        (
            "alpha binders",
            r#"{"mode":"equal","left":["lambda",1,"x",["bound",1,"x"]],"right":["lambda",2,"y",["bound",2,"y"]]}"#,
            0,
        ),
        (
            "free versus bound",
            r#"{"mode":"equal","left":["lambda",1,"x",["bound",2,"y"]],"right":["lambda",2,"y",["bound",2,"y"]]}"#,
            4,
        ),
        (
            "nested binder correspondence",
            r#"{"mode":"equal","left":["lambda",1,"x",["lambda",2,"y",["bound",1,"x"]]],"right":["lambda",3,"z",["lambda",3,"z",["bound",3,"z"]]]}"#,
            4,
        ),
        (
            "global identity",
            r#"{"mode":"equal","left":["global","Left.f"],"right":["global","Right.f"]}"#,
            4,
        ),
        (
            "rigid is not bound",
            r#"{"mode":"equal","left":["rigid",1,"x",0],"right":["bound",1,"x"]}"#,
            4,
        ),
        (
            "rigid level",
            r#"{"mode":"equal","left":["rigid",1,"x",0],"right":["rigid",1,"x",1]}"#,
            4,
        ),
        (
            "simultaneous swap",
            r#"{"mode":"substitute","pairs":[[1,["bound",2,"y"]],[2,["bound",1,"x"]]],"term":["apply",["bound",1,"x"],["bound",2,"y"]],"result":["apply",["bound",2,"y"],["bound",1,"x"]]}"#,
            0,
        ),
        (
            "capture avoided",
            r#"{"mode":"substitute","pairs":[[1,["bound",2,"free"]]],"term":["lambda",2,"local",["add",["bound",1,"x"],["bound",2,"local"]]],"result":["lambda",3,"new",["add",["bound",2,"free"],["bound",3,"new"]]]}"#,
            0,
        ),
        (
            "captured result rejected",
            r#"{"mode":"substitute","pairs":[[1,["bound",2,"free"]]],"term":["lambda",2,"local",["add",["bound",1,"x"],["bound",2,"local"]]],"result":["lambda",2,"local",["add",["bound",2,"free"],["bound",2,"local"]]]}"#,
            4,
        ),
        (
            "shadowed replacement masked",
            r#"{"mode":"substitute","pairs":[[1,["int",7]]],"term":["lambda",1,"x",["bound",1,"x"]],"result":["lambda",3,"z",["bound",3,"z"]]}"#,
            0,
        ),
        (
            "meta replacement capture avoided",
            r#"{"mode":"substitute-metas","pairs":[[1,["bound",2,"free"]]],"term":["lambda",2,"local",["meta",1]],"result":["lambda",3,"new",["bound",2,"free"]]}"#,
            0,
        ),
        (
            "known scope",
            r#"{"mode":"scope","bound":[1],"rigid":[2],"globals":["Left.f"],"term":["apply",["global","Left.f"],["add",["bound",1,"x"],["rigid",2,"r",0]]]}"#,
            0,
        ),
        (
            "missing rigid scope",
            r#"{"mode":"scope","bound":[1,2],"rigid":[],"globals":[],"term":["rigid",2,"r",0]}"#,
            4,
        ),
        (
            "missing global scope",
            r#"{"mode":"scope","bound":[],"rigid":[],"globals":["Left.f"],"term":["global","Right.f"]}"#,
            4,
        ),
        (
            "unresolved meta",
            r#"{"mode":"scope","bound":[1],"rigid":[1],"globals":[],"term":["meta",1]}"#,
            4,
        ),
        (
            "negative identity",
            r#"{"mode":"equal","left":["bound",-1,"x"],"right":["bound",1,"x"]}"#,
            2,
        ),
        (
            "unknown term tag",
            r#"{"mode":"equal","left":["trust-me",1],"right":["int",1]}"#,
            2,
        ),
        (
            "large integer retained",
            r#"{"mode":"equal","left":["int",340282366920938463463374607431768211456],"right":["int",340282366920938463463374607431768211456]}"#,
            0,
        ),
        (
            "large integers do not wrap",
            r#"{"mode":"equal","left":["int",340282366920938463463374607431768211456],"right":["int",340282366920938463481821351505477763072]}"#,
            4,
        ),
        (
            "large binding identities stay distinct",
            r#"{"mode":"equal","left":["bound",340282366920938463463374607431768211456,"x"],"right":["bound",340282366920938463481821351505477763072,"x"]}"#,
            4,
        ),
        (
            "large identity capture avoidance",
            r#"{"mode":"substitute","pairs":[[340282366920938463463374607431768211456,["bound",340282366920938463481821351505477763072,"free"]]],"term":["lambda",340282366920938463481821351505477763072,"local",["bound",340282366920938463463374607431768211456,"x"]],"result":["lambda",1,"fresh",["bound",340282366920938463481821351505477763072,"free"]]}"#,
            0,
        ),
    ];
    for (name, input, expected) in packets {
        run(name, input, expected);
    }
    common::cleanup(&dir);
}
