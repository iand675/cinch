//! Collection literals use real FromList dictionaries and expected element types.
mod common;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}
#[test]
fn compiled_cinch_elaborates_collection_literals_and_preserves_actual_dictionaries() {
    let dir = common::project("collection_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Collections.cinch")),
    ]);

    let (_, interfaces) = cinch::driver::check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "collection-base.cinch").unwrap();
    for (module, identity) in [("Data.List", "Data.List.FromList_List"), ("Data.Bytes", "Data.Bytes.FromList_Bytes")] {
        let iface = interfaces.iter().find(|i| i.module == module).unwrap();
        assert!(iface.exports.instances.iter().find(|i| i.id == identity).unwrap().resolved.is_some(), "actual {identity} factory must carry its complete defining scheme");
    }
    let mut rows = Vec::new();
    let folder = dir.join("interfaces"); std::fs::create_dir_all(&folder).unwrap();
    for iface in interfaces.iter().filter(|i| i.module != "Main") {
        let path = folder.join(format!("{}.cinchif", iface.module));
        std::fs::write(&path, iface.to_json()).unwrap();
        rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let bag = "data Bag where\n    Bag :: Nat -> Bag\n  implicit instance BagNat :: FromList Bag Nat where\n    from-list items = Bag 0\n  ";
    let choices = "data Bag where\n    Bag :: Nat -> Bag\n  implicit instance BagNat :: FromList Bag Nat where\n    from-list items = Bag 0\n  implicit instance BagBool :: FromList Bag Bool where\n    from-list items = Bag 1\n  ";
    let cases = vec![
        ("expected Int list", "answer :: List Int\n  answer = [1, 2]".to_string(), true),
        ("expected Nat list", "answer :: List Nat\n  answer = [1, 2]".to_string(), true),
        ("negative Nat element", "bad :: List Nat\n  bad = [-1]".to_string(), false),
        ("machine byte collection", "answer :: Bytes\n  answer = [0, 255]".to_string(), true),
        ("out of range byte", "bad :: Bytes\n  bad = [256]".to_string(), false),
        ("inferred default List Int", "discard :: forall a. a -> Int\n  discard x = 0\n  answer :: Int\n  answer = discard [1, 2]".to_string(), true),
        ("polymorphic empty list", "empty :: forall a. List a\n  empty = []".to_string(), true),
        ("generalized local empty list", "answer :: (List Nat, List Bool)\n  answer = let empty = [] in (empty, empty)".to_string(), true),
        ("nested expected list", "answer :: List (List Nat)\n  answer = [[1], []]".to_string(), true),
        ("tuple element expectation", "answer :: List (Nat, Bool)\n  answer = [(1, True)]".to_string(), true),
        ("nested tuple element expectation", "answer :: List ((Nat, Bool), Nat)\n  answer = [((1, True), 2)]".to_string(), true),
        ("constructor element expectation", "data Box a where\n    Box :: a -> Box a\n  answer :: List (Box Nat)\n  answer = [Box 1]".to_string(), true),
        ("polymorphic application element", "identity :: forall a. a -> a\n  identity x = x\n  answer :: List Nat\n  answer = [identity 1]".to_string(), true),
        ("incompatible element types", "bad :: List Nat\n  bad = [1, True]".to_string(), false),
        ("custom container", format!("{bag}answer :: Bag\n  answer = [1, 2]"), true),
        ("ambiguous numeric element defaults Int", format!("{choices}bad :: Bag\n  bad = [1]"), false),
        ("custom container chooses Bool evidence", format!("{choices}answer :: Bag\n  answer = [True]"), true),
        ("application numeric ambiguity also defaults Int", format!("{choices}identity :: forall a. a -> a\n  identity x = x\n  bad :: Bag\n  bad = [identity 1]"), false),
        ("explicit Nat selects matching collection dictionary", format!("{choices}answer :: Bag\n  answer = [(1 :: Nat)]"), true),
        ("application preserves explicit element type", format!("{choices}identity :: forall a. a -> a\n  identity x = x\n  answer :: Bag\n  answer = [identity (1 :: Nat)]"), true),
        ("numeric ambiguity selects actual Int dictionary", format!("{choices}implicit instance BagInt :: FromList Bag Int where\n    from-list items = Bag 2\n  answer :: Bag\n  answer = [1]"), true),
        ("refined numeric element", "answer :: List (InRange 1 4 Nat)\n  answer = [2]".to_string(), true),
        ("refined numeric element rejected", "bad :: List (InRange 1 4 Nat)\n  bad = [7]".to_string(), false),
        ("empty custom container remains ambiguous", format!("{choices}bad :: Bag\n  bad = []"), false),
        ("lexical dictionary chooses differing element type", "make :: using (chosen :: FromList (List Nat) Bool) => List Nat\n  make = [True]".to_string(), true),
        ("lexical dictionary also determines inferred container", "make :: using (chosen :: FromList (List Nat) Bool) => List Nat\n  make = let xs = [True] in xs".to_string(), true),
        ("lexical dictionary is not bypassed", "bad :: using (chosen :: FromList (List Nat) Bool) => List Nat\n  bad = [1]".to_string(), false),
        ("list pattern", "read :: List Nat -> Nat\n  read [] = 0\n  read [x] = x\n  read (Cons first rest) = first".to_string(), true),
        ("non exhaustive list pattern", "bad :: List Nat -> Nat\n  bad [x] = x".to_string(), false),
        ("duplicate list pattern binding", "bad :: List Nat -> Nat\n  bad [x, x] = x\n  bad other = 0".to_string(), false),
        ("element effects are retained", "make :: forall r. (Unit ->{IO r} Nat) -> Unit ->{IO r} List Nat\n  make f u = [f u]".to_string(), true),
        ("element effects cannot disappear", "bad :: forall r. (Unit ->{IO r} Nat) -> Unit -> List Nat\n  bad f u = [f u]".to_string(), false),
    ];
    let mut requests = Vec::new();
    let mut oracle_errors = Vec::new();
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let actual = cinch::driver::check_sources(&source, "native-collection-oracle.cinch");
        if actual.is_ok() != accepted { oracle_errors.push(format!("{name}: {}", actual.err().map_or("accepted".into(), |e| e.message))); }
        let element = if name == "numeric ambiguity selects actual Int dictionary" { "Int" } else if accepted && name.starts_with("lexical dictionary") { "Bool" } else { "" };
        requests.push(format!("{{\"name\":{},\"source\":{},\"element\":{},\"wanted\":{}}}", quote(name), quote(&source), quote(element), if accepted { 0 } else { 4 }));
    }
    assert!(oracle_errors.is_empty(), "Native oracle differences: {oracle_errors:#?}");
    std::fs::write(dir.join("cases.json"), format!("[{}]", requests.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "collections").expect("compile independent Cinch collection elaborator");
    let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "collection-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
