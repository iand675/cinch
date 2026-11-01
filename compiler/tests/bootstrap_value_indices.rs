//! Typed index unification, substitution and Core witness closure.
mod common;

#[test]
fn typed_indices_preserve_carriers_scope_and_runtime_witnesses() {
    let dir = common::project(
        "value_indices",
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
                include_str!("../../bootstrap/programs/Run.ValueIndices.cinch"),
            ),
        ],
    );
    let exe =
        common::build(&dir, "Main.cinch", "value-indices").expect("build typed index engine");
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
            common::read_output(&dir, "value-indices-report.txt"),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    for size in [0, 1, 7, 31, 257] {
        run(&format!("nonce {size}"), format!(r#"{{"mode":"selftest","nonce":{size}}}"#).as_bytes(), 0);
    }
    // Complete carrier/type/index trees arrive after compilation. The
    // seed's constant evaluator cannot know or precompute these packets.
    let packets = [
        ("large natural equality", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["int",340282366920938463463374607431768211456]],"right":["index",["con","Nat","Type"],["int",340282366920938463463374607431768211456]]}"#, 0),
        ("large naturals differ", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["int",340282366920938463463374607431768211456]],"right":["index",["con","Nat","Type"],["int",340282366920938463463374607431768211457]]}"#, 4),
        ("no Nat Int coercion", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["int",3]],"right":["index",["con","Int","Type"],["int",3]]}"#, 4),
        ("undefined natural subtraction", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["sub",["int",0],["int",1]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("zero cannot hide undefined subtraction", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["mul",["int",0],["sub",["int",0],["int",1]]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("recovery type is not equality evidence", r#"{"mode":"unify","left":"error","right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("runtime natural solution", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["int",340282366920938463463374607431768211456]}"#, 0),
        ("runtime negative integer solution", r#"{"mode":"infer","carrier":["con","Int","Type"],"term":["int",-340282366920938463463374607431768211456]}"#, 0),
        ("runtime arithmetic solution", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["add",["int",3],["int",4]]}"#, 0),
        ("unknown index meta", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["meta",999]}"#, 4),
        ("unsolved index has no Core witness", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["meta",0]}"#, 4),
        ("runtime rigid escape", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["rigid",1,"inner",1]}"#, 4),
        ("zero cannot hide opaque evaluation", r#"{"mode":"admissible","carrier":["con","Nat","Type"],"term":["mul",["int",0],["apply",["global","Unknown.partial"],["int",1]]]}"#, 4),
        ("opaque multiplication cannot convert to zero", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["mul",["int",0],["global","Unknown.partial"]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("opaque subtraction cannot convert to zero", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["sub",["global","Unknown.partial"],["global","Unknown.partial"]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("symbolic addition is not conversion", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["add",["bound",1,"n"],["int",0]]],"right":["index",["con","Nat","Type"],["bound",1,"n"]]}"#, 4),
        ("symbolic zero multiplication is not conversion", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["mul",["int",0],["bound",1,"n"]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("symbolic subtraction is not conversion", r#"{"mode":"unify","left":["index",["con","Nat","Type"],["sub",["bound",1,"n"],["bound",1,"n"]]],"right":["index",["con","Nat","Type"],["int",0]]}"#, 4),
        ("inference cannot erase opaque evaluation", r#"{"mode":"infer","carrier":["con","Nat","Type"],"term":["mul",["int",0],["global","Unknown.partial"]]}"#, 4),
        ("known natural bound", r#"{"mode":"admissible","carrier":["con","Nat","Type"],"term":["sub",["int",7],["int",1]]}"#, 0),
        ("negative natural rejected", r#"{"mode":"admissible","carrier":["con","Nat","Type"],"term":["int",-1]}"#, 4),
        ("wrapping arithmetic stays opaque", r#"{"mode":"admissible","carrier":["con","U32","Type"],"term":["add",["int",1],["int",2]]}"#, 4),
        ("capture-safe typed substitution", r#"{"mode":"substitute","pairs":[[1,["bound",2,"free",["value",["con","Nat","Type"]]]]],"type":["index",["fun",["con","Nat","Type"],["con","Nat","Type"]],["lambda",2,"local",["add",["bound",1,"free"],["bound",2,"local"]]]],"result":["index",["fun",["con","Nat","Type"],["con","Nat","Type"]],["lambda",3,"fresh",["add",["bound",2,"free"],["bound",3,"fresh"]]]]}"#, 0),
        ("captured substitution rejected", r#"{"mode":"substitute","pairs":[[1,["bound",2,"free",["value",["con","Nat","Type"]]]]],"type":["index",["fun",["con","Nat","Type"],["con","Nat","Type"]],["lambda",2,"local",["add",["bound",1,"free"],["bound",2,"local"]]]],"result":["index",["fun",["con","Nat","Type"],["con","Nat","Type"]],["lambda",2,"local",["add",["bound",2,"local"],["bound",2,"local"]]]]}"#, 4),
        ("substitution carrier cannot change", r#"{"mode":"substitute","pairs":[[1,["index",["con","Int","Type"],["int",3]]]],"type":["index",["con","Nat","Type"],["bound",1,"n"]],"result":["index",["con","Nat","Type"],["int",3]]}"#, 4),
        ("defining global identities differ", r#"{"mode":"unify","left":["index",["con","Bool","Type"],["global","Left.flag"]],"right":["index",["con","Bool","Type"],["global","Right.flag"]]}"#, 4),
        ("malformed type tree", r#"{"mode":"unify","left":["index",["con","Nat","Type"]],"right":["index",["con","Nat","Type"],["int",3]]}"#, 2),
        ("malformed JSON", "{", 2),
    ];
    for (name, input, expected) in packets {
        run(name, input.as_bytes(), expected);
    }
    common::cleanup(&dir);
}
