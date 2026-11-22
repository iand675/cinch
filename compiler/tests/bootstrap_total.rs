//! The Native producer's real body data is checked by an independently
//! compiled Cinch checker; changing input bytes does not rebuild that checker.
mod common;

#[test]
fn compiled_cinch_checks_actual_total_body_graphs_and_forged_variants() {
    let dir = common::project("total_bodies", &[
        ("Bootstrap.Total.cinch", include_str!("../../bootstrap/Bootstrap.Total.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Total.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "total-bodies").expect("compile independent total-body checker");
    let source = "module Main where\n  width :: Nat -> Nat\n  width cp = if cp < 128 then 1 else if cp < 2048 then 2 else if cp < 65536 then 3 else 4\n  main :: Int\n  main = 0\n";
    let native = cinch::driver::check_sources(source, "total-body-producer.cinch").unwrap();
    let graph = native.total_bodies.closure(["Main.width"]).unwrap().to_json();
    let run = |case: &str, data: &str, expected: i32| {
        std::fs::write(dir.join("bodies.json"), data).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "{case}; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    };
    run("actual primitive/method closure", &graph, 0);
    assert_eq!(common::read_output(&dir, "bounds.txt"), "1:4");
    let changed = cinch::driver::check_sources(&source.replace("then 1 else", "then 5 else"), "total-body-producer.cinch").unwrap();
    run("changed actual body remains a valid Nat function", &changed.total_bodies.closure(["Main.width"]).unwrap().to_json(), 0);
    assert_eq!(common::read_output(&dir, "bounds.txt"), "2:5", "bounds come from the actual changed body, not its name or original declaration");
    run("duplicate version is rejected by the JSON parser", &graph.replacen("\"version\":1", "\"version\":1,\"version\":1", 1), 2);
    for (name, data) in [
        ("unknown version", graph.replacen("\"version\":1", "\"version\":2", 1)),
        ("ignored semantic qualifier", graph.replacen("\"version\":1", "\"version\":1,\"trust-me\":true", 1)),
        ("unknown body operation", graph.replace("\"NatCmp\"", "\"AssumeNumericClassSlot\"")),
    ] { run(name, &data, 4); }
    let real_source = source.replace("module Main where\n", "module Main where\n  import Data.String (utf8-encode-width)\n")
        .replace("if cp < 128 then 1 else if cp < 2048 then 2 else if cp < 65536 then 3 else 4", "utf8-encode-width cp");
    let actual_measure = cinch::driver::check_sources(&real_source, "actual-library-measure.cinch").unwrap();
    run("actual UTF8 measure and a checked source subcall", &actual_measure.total_bodies.closure(["Main.width"]).unwrap().to_json(), 0);
    assert_eq!(common::read_output(&dir, "bounds.txt"), "1:4");
    for (name, body, expected_bounds) in [
        ("strict let retains the actual result domain", "let value = ((if cp < 128 then 5 else 7) :: Nat) in value", "5:7"),
        ("an unconstrained argument has no invented finite envelope", "cp", "unknown"),
        ("mathematical literals are not narrowed to machine integers", "340282366920938463463374607431768211507", "340282366920938463463374607431768211507:340282366920938463463374607431768211507"),
    ] {
        let input = source.replace("if cp < 128 then 1 else if cp < 2048 then 2 else if cp < 65536 then 3 else 4", body);
        let checked = cinch::driver::check_sources(&input, "total-envelope-source.cinch").unwrap();
        run(name, &checked.total_bodies.closure(["Main.width"]).unwrap().to_json(), 0);
        assert_eq!(common::read_output(&dir, "bounds.txt"), expected_bounds, "{name}");
    }
    let one = |body: &str, parameters: &str, result: &str| format!(r#"{{"version":1,"definitions":[{{"identity":"Main.width","parameters":{parameters},"result":"{result}","body":{body}}}]}}"#);
    for (name, data) in [
        ("wrong local", one(r#"{"node":"local","slot":1}"#, r#"["Nat"]"#, "Nat")),
        ("wrong result sort", one(r#"{"node":"local","slot":0}"#, r#"["Bool"]"#, "Nat")),
        ("bad decimal", one(r#"{"node":"number","sort":"Nat","decimal":"1x"}"#, r#"["Nat"]"#, "Nat")),
        ("negative Nat", one(r#"{"node":"number","sort":"Nat","decimal":"-1"}"#, r#"["Nat"]"#, "Nat")),
        ("out of range I64", one(r#"{"node":"number","sort":"I64","decimal":"9223372036854775808"}"#, r#"["Nat"]"#, "I64")),
        ("recursive pure hint", one(r#"{"node":"call","target":"Main.width","arguments":[{"node":"local","slot":0}]}"#, r#"["Nat"]"#, "Nat")),
        ("unknown pure callee", one(r#"{"node":"call","target":"missing","arguments":[{"node":"local","slot":0}]}"#, r#"["Nat"]"#, "Nat")),
        ("byte contents have no stable opcode", one(r#"{"node":"primitive","operation":"BytesGet","arguments":[]}"#, r#"["Nat"]"#, "Nat")),
        ("wrapping arithmetic is not mathematical", one(r#"{"node":"primitive","operation":"WrappingAddI64","arguments":[]}"#, r#"["Nat"]"#, "Nat")),
        ("strict let cannot reference its own RHS binder", one(r#"{"node":"let","value":{"node":"local","slot":1},"body":{"node":"local","slot":1}}"#, r#"["Nat"]"#, "Nat")),
        ("untyped condition", one(r#"{"node":"if","condition":{"node":"local","slot":0},"yes":{"node":"local","slot":0},"no":{"node":"local","slot":0}}"#, r#"["Nat"]"#, "Nat")),
    ] { run(name, &data, 4); }
    common::cleanup(&dir);
}
