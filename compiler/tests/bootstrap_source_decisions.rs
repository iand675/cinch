//! Defining constructor contracts retain the source proof slot and zero payload.
mod common;

#[test]
fn source_decisions_preserve_predicates_and_check_authority() {
    let dir = common::project("source_decisions", &[
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceDecisions.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-decisions").expect("build source constructor checker");
    let cases = [
        ("ambiguous conjunction branch", "choose :: (n :: Int) -> Dec (n > 0 && True)\n  choose n = decide (n > 0 && True)", 0),
        ("ambiguous disjunction branch", "choose :: (n :: Int) -> Dec (n > 0 || False)\n  choose n = decide (n > 0 || False)", 0),
        ("total Nat subtraction", "choose :: (n :: Nat) -> Dec (n >= 1 && n - n == 0)\n  choose n = decide (n >= 1 && n - n == 0)", 0),
        ("disjunction retains total Nat subtraction", "choose :: (n :: Nat) -> Dec (n < 1 || n - n == 0)\n  choose n = decide (n < 1 || n - n == 0)", 0),
        ("Nat subtraction strict let", "choose :: (n :: Nat) -> Dec (n >= 1 && n - n == 0)\n  choose n = let alias = n in decide (alias >= 1 && alias - alias == 0)", 0),
        ("wrong short circuit branch does not order subtraction", "choose :: Nat -> Int\n  choose n = case decide (n < 1 && n - 1 > 0) of Yes _ -> 1; No _ -> 0", 4),
        ("no proof escape from disjunction", "choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = case decide (n > 0 || n == 0) of Yes h -> Yes h; No h -> No h", 4),
        ("nonzero Int", "choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = decide (NonZero n)", 0),
        ("numeric comparison", "choose :: (n :: Int) -> Dec (n > 0)\n  choose n = decide (n > 0)", 0),
        ("nonzero Nat", "choose :: (n :: Nat) -> Dec (NonZero n)\n  choose n = decide (NonZero n)", 0),
        ("comparison machine byte", "choose :: (n :: U8) -> Dec (n < 255)\n  choose n = decide (n < 255)", 0),
        ("strict let alias", "choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = let alias = n in decide (NonZero alias)", 0),
        ("positive evidence remains proposition", "choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = case decide (NonZero n) of Yes h -> Yes h; No h -> No h", 0),
        ("constant truth", "choose :: Dec True\n  choose = decide True", 0),
        ("constant falsehood", "choose :: Dec False\n  choose = decide False", 0),
        ("logical negation", "choose :: (n :: Int) -> Dec (Not (n > 0))\n  choose n = decide (Not (n > 0))", 0),
        ("conjunction", "choose :: (n :: Int) -> Dec (n > 0 && n < 20)\n  choose n = decide (n > 0 && n < 20)", 0),
        ("disjunction", "choose :: (n :: Int) -> Dec (n < 0 || n > 20)\n  choose n = decide (n < 0 || n > 20)", 0),
        ("mathematical arithmetic", "choose :: (n :: Int) -> Dec (n + 1 > 0)\n  choose n = decide (n + 1 > 0)", 0),
        ("negative literal", "choose :: (n :: Int) -> Dec (n > -5)\n  choose n = decide (n > -5)", 0),
        ("large integer", "choose :: (n :: Int) -> Dec (n > 340282366920938463463374607431768211507)\n  choose n = decide (n > 340282366920938463463374607431768211507)", 0),
        ("negative evidence cannot prove positive", "choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = case decide (NonZero n) of Yes h -> Yes h; No h -> Yes h", 4),
        ("changed value cannot retain proof", "choose :: (n :: Int) -> (m :: Int) -> Dec (NonZero m)\n  choose n m = decide (NonZero n)", 4),
        ("changed comparison cannot retain proof", "choose :: (n :: Int) -> Dec (n < 0)\n  choose n = decide (n > 0)", 4),
        ("runtime Bool is not predicate syntax", "choose :: Bool -> Dec True\n  choose condition = decide condition", 4),
        ("opaque Prop needs procedure", "choose :: forall (p :: Prop). Dec p\n  choose = decide p", 3),
        ("proof reference is not procedure", "choose :: forall (p :: Prop). (h :: p) -> Dec p\n  choose h = decide h", 4),
        ("first class decide is rejected", "choose = decide", 4),
        ("arbitrary predicate function", "predicate :: Int -> Bool\n  predicate n = True\n  choose :: Int -> Int\n  choose n = case decide (predicate n) of Yes _ -> 1; No _ -> 0", 4),
        ("ordinary global numeric function", "identity :: Int -> Int\n  identity n = n\n  choose :: Int -> Int\n  choose n = case decide (identity n > 0) of Yes _ -> 1; No _ -> 0", 4),
        ("machine arithmetic cannot pretend mathematical", "choose :: U8 -> Int\n  choose n = case decide (n + 1 > 0) of Yes _ -> 1; No _ -> 0", 4),
        ("Nat subtraction requires ordering", "choose :: Nat -> Int\n  choose n = case decide (n - 1 > 0) of Yes _ -> 1; No _ -> 0", 4),
        ("proof cannot be runtime result", "choose :: Int -> Int\n  choose n = case decide (NonZero n) of Yes h -> h; No h -> 0", 4),
    ];
    for (name, body, expected) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        std::fs::write(dir.join("mutation.txt"), "").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-decisions-report.txt");
        assert_eq!(output.status.code(), Some(expected), "case {name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    for (body, diagnostic) in [
        ("choose :: U8 -> Int\n  choose n = case decide (n + 1 > 0) of Yes _ -> 1; No _ -> 0", "decision arithmetic needs checked mathematical semantics"),
        ("choose :: Nat -> Int\n  choose n = case decide (n - 1 > 0) of Yes _ -> 1; No _ -> 0", "decision Nat subtraction needs checked ordering evidence"),
        ("identity :: Int -> Int\n  identity n = n\n  choose :: Int -> Int\n  choose n = case decide (identity n > 0) of Yes _ -> 1; No _ -> 0", "decision calls need checked pure total body authority"),
    ] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        std::fs::write(dir.join("mutation.txt"), "").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-decisions-report.txt");
        assert_eq!(output.status.code(), Some(4), "expected {diagnostic}: {report}");
        assert!(report.contains(diagnostic), "expected {diagnostic}: {report}");
    }
    std::fs::write(dir.join("source.cinch"), "module Main where\n  choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = decide (NonZero n)\n  main :: Int\n  main = 0\n").unwrap();
    for field in ["name", "operation", "runtime_abi", "first_class", "predicate_kind", "scheme"] {
        std::fs::write(dir.join("mutation.txt"), field).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-decisions-report.txt");
        assert_eq!(output.status.code(), Some(4), "mutated {field}; project {}\n{report}", dir.display());
        assert!(report.contains("kernel predicate contract"), "mutated {field}: {report}");
    }
    std::fs::write(dir.join("source.cinch"), "module Main where\n  choose :: (n :: Int) -> Dec (NonZero n)\n  choose n = case decide (NonZero n) of Yes h -> Yes h; No h -> No h\n  main :: Int\n  main = 0\n").unwrap();
    for field in ["syntax-type", "constructor-type"] {
        std::fs::write(dir.join("mutation.txt"), field).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-decisions-report.txt");
        assert_eq!(output.status.code(), Some(4), "cached {field}; project {}\n{report}", dir.display());
        assert!(report.contains("cached kernel type disagrees"), "cached {field}: {report}");
    }
    for body in ["decide :: Int -> Int\n  decide n = n\n  choose :: Int -> Int\n  choose n = decide n", "choose :: (Int -> Int) -> Int\n  choose decide = decide 0"] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        std::fs::write(dir.join("mutation.txt"), "ordinary").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-decisions-report.txt");
        assert_eq!(output.status.code(), Some(0), "ordinary shadow; project {}\n{report}", dir.display());
    }
    common::cleanup(&dir);
}
